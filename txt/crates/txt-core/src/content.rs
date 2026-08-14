//! Structuring of text runs (format.md §5, grammar.ebnf): headings,
//! paragraphs, lists, and display math, plus the inline grammar —
//! strong/emphasis, code, math, cross-references, citations, glossary
//! spans. Unmatched delimiters degrade to literal text rather than
//! erroring; `txt check` rules stay the authority on what is *invalid*.

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum BodyElement {
    Heading { level: u8, content: Vec<Inline> },
    Paragraph(Vec<Inline>),
    BulletList(Vec<Vec<Inline>>),
    OrderedList(Vec<Vec<Inline>>),
    DisplayMath(String),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Inline {
    Str(String),
    Strong(Vec<Inline>),
    Emph(Vec<Inline>),
    Code(String),
    Math(String),
    Ref(String),
    Citation(String),
    Gloss(String),
}

/// Structures one text run (the content of a `Text` node) into body
/// elements. HTML comments are dropped.
pub fn structure(text: &str) -> Vec<BodyElement> {
    let mut out = Vec::new();
    let mut para: Vec<String> = Vec::new();
    let mut bullets: Vec<String> = Vec::new();
    let mut ordered: Vec<String> = Vec::new();
    let mut in_comment = false;

    fn flush_para(para: &mut Vec<String>, out: &mut Vec<BodyElement>) {
        if !para.is_empty() {
            let joined = para.join(" ");
            para.clear();
            out.push(BodyElement::Paragraph(parse_inlines(&joined)));
        }
    }
    fn flush_lists(
        bullets: &mut Vec<String>,
        ordered: &mut Vec<String>,
        out: &mut Vec<BodyElement>,
    ) {
        if !bullets.is_empty() {
            let items = bullets.drain(..).map(|i| parse_inlines(&i)).collect();
            out.push(BodyElement::BulletList(items));
        }
        if !ordered.is_empty() {
            let items = ordered.drain(..).map(|i| parse_inlines(&i)).collect();
            out.push(BodyElement::OrderedList(items));
        }
    }

    for raw in text.lines() {
        let line = raw.trim_end();
        let trimmed = line.trim_start();

        if in_comment {
            if trimmed.contains("-->") {
                in_comment = false;
            }
            continue;
        }
        if trimmed.starts_with("<!--") {
            if !trimmed.contains("-->") {
                in_comment = true;
            }
            continue;
        }

        if trimmed.is_empty() {
            flush_para(&mut para, &mut out);
            flush_lists(&mut bullets, &mut ordered, &mut out);
        } else if trimmed.starts_with('#') {
            flush_para(&mut para, &mut out);
            flush_lists(&mut bullets, &mut ordered, &mut out);
            let level = trimmed.chars().take_while(|&c| c == '#').count().min(6) as u8;
            let rest = trimmed.trim_start_matches('#').trim_start();
            out.push(BodyElement::Heading { level, content: parse_inlines(rest) });
        } else if trimmed.starts_with("$$") && trimmed.ends_with("$$") && trimmed.len() > 4 {
            flush_para(&mut para, &mut out);
            flush_lists(&mut bullets, &mut ordered, &mut out);
            out.push(BodyElement::DisplayMath(
                trimmed[2..trimmed.len() - 2].trim().to_string(),
            ));
        } else if let Some(item) = trimmed.strip_prefix("- ") {
            flush_para(&mut para, &mut out);
            bullets.push(item.trim_start().to_string());
        } else if let Some(item) = ordered_item(trimmed) {
            flush_para(&mut para, &mut out);
            ordered.push(item.to_string());
        } else if line.starts_with(char::is_whitespace)
            && (!bullets.is_empty() || !ordered.is_empty())
        {
            // continuation of the previous list item
            if let Some(last) = bullets.last_mut().or(ordered.last_mut()) {
                last.push(' ');
                last.push_str(trimmed);
            }
        } else {
            flush_lists(&mut bullets, &mut ordered, &mut out);
            para.push(trimmed.to_string());
        }
    }
    flush_para(&mut para, &mut out);
    flush_lists(&mut bullets, &mut ordered, &mut out);
    out
}

fn ordered_item(line: &str) -> Option<&str> {
    let digits = line.chars().take_while(|c| c.is_ascii_digit()).count();
    if digits == 0 {
        return None;
    }
    line[digits..].strip_prefix(". ").map(str::trim_start)
}

/// Parses the inline grammar. Never fails: anything unrecognized is
/// literal text.
pub fn parse_inlines(src: &str) -> Vec<Inline> {
    let chars: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut i = 0;

    fn flush(buf: &mut String, out: &mut Vec<Inline>) {
        if !buf.is_empty() {
            out.push(Inline::Str(std::mem::take(buf)));
        }
    }
    // finds the next occurrence of a delimiter sequence
    fn find(chars: &[char], from: usize, delim: &[char]) -> Option<usize> {
        (from..=chars.len().saturating_sub(delim.len()))
            .find(|&j| chars[j..j + delim.len()] == *delim)
    }

    while i < chars.len() {
        let c = chars[i];
        match c {
            '\\' if chars.get(i + 1).is_some_and(|n| {
                matches!(n, '\\' | '$' | '*' | '[' | ']' | '`' | '@' | '#' | '_')
            }) =>
            {
                buf.push(chars[i + 1]);
                i += 2;
            }
            '$' => {
                if let Some(end) = find(&chars, i + 1, &['$']) {
                    flush(&mut buf, &mut out);
                    out.push(Inline::Math(chars[i + 1..end].iter().collect()));
                    i = end + 1;
                } else {
                    buf.push(c);
                    i += 1;
                }
            }
            '`' => {
                if let Some(end) = find(&chars, i + 1, &['`']) {
                    flush(&mut buf, &mut out);
                    out.push(Inline::Code(chars[i + 1..end].iter().collect()));
                    i = end + 1;
                } else {
                    buf.push(c);
                    i += 1;
                }
            }
            '*' => {
                let strong = i + 1 < chars.len() && chars[i + 1] == '*';
                let (delim, start): (&[char], usize) =
                    if strong { (&['*', '*'], i + 2) } else { (&['*'], i + 1) };
                if let Some(end) = find(&chars, start, delim) {
                    flush(&mut buf, &mut out);
                    let inner: String = chars[start..end].iter().collect();
                    let inner = parse_inlines(&inner);
                    out.push(if strong { Inline::Strong(inner) } else { Inline::Emph(inner) });
                    i = end + delim.len();
                } else {
                    buf.push(c);
                    i += 1;
                }
            }
            '[' => {
                // citation [@key] or glossary span [text]{.gloss}
                if let Some(close) = find(&chars, i + 1, &[']']) {
                    let inner: String = chars[i + 1..close].iter().collect();
                    let after: String =
                        chars[close + 1..(close + 9).min(chars.len())].iter().collect();
                    if let Some(key) = inner.strip_prefix('@') {
                        flush(&mut buf, &mut out);
                        out.push(Inline::Citation(key.to_string()));
                        i = close + 1;
                        continue;
                    }
                    if after.starts_with("{.gloss}") {
                        flush(&mut buf, &mut out);
                        out.push(Inline::Gloss(inner));
                        i = close + 1 + "{.gloss}".len();
                        continue;
                    }
                }
                buf.push(c);
                i += 1;
            }
            '@' => {
                let prev_ok = i == 0
                    || !(chars[i - 1].is_alphanumeric() || chars[i - 1] == '[');
                let mut j = i + 1;
                let mut name = String::new();
                while j < chars.len()
                    && (chars[j].is_ascii_alphanumeric()
                        || chars[j] == '-'
                        || chars[j] == '_')
                {
                    name.push(chars[j]);
                    j += 1;
                }
                let starts_ok =
                    name.chars().next().is_some_and(|c| c.is_ascii_alphabetic());
                if prev_ok && starts_ok {
                    flush(&mut buf, &mut out);
                    out.push(Inline::Ref(name));
                    i = j;
                } else {
                    buf.push(c);
                    i += 1;
                }
            }
            _ => {
                buf.push(c);
                i += 1;
            }
        }
    }
    flush(&mut buf, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use Inline::*;

    #[test]
    fn inline_mix() {
        let got = parse_inlines("Solve $3x + 4 = 19$ using **inverse operations** (see @fig-balance).");
        assert_eq!(
            got,
            vec![
                Str("Solve ".into()),
                Math("3x + 4 = 19".into()),
                Str(" using ".into()),
                Strong(vec![Str("inverse operations".into())]),
                Str(" (see ".into()),
                Ref("fig-balance".into()),
                Str(").".into()),
            ]
        );
    }

    #[test]
    fn gloss_citation_and_email() {
        let got = parse_inlines("[equation]{.gloss} per [@smith2024], mail a@b.com");
        assert_eq!(got[0], Gloss("equation".into()));
        assert!(got.iter().any(|i| *i == Citation("smith2024".into())));
        assert!(!got.iter().any(|i| matches!(i, Ref(_))));
    }

    #[test]
    fn emphasis_vs_strong() {
        let got = parse_inlines("*solving* means **finding** values");
        assert_eq!(got[0], Emph(vec![Str("solving".into())]));
        assert_eq!(got[2], Strong(vec![Str("finding".into())]));
    }

    #[test]
    fn unmatched_delimiters_stay_literal() {
        let got = parse_inlines("cost is $6 for tickets");
        assert_eq!(got, vec![Str("cost is $6 for tickets".into())]);
    }

    #[test]
    fn escapes_yield_literals() {
        let got = parse_inlines(r"tickets cost \$18, one costs \$6 — solve $3t = 18$");
        assert_eq!(
            got,
            vec![
                Str("tickets cost $18, one costs $6 — solve ".into()),
                Math("3t = 18".into()),
            ]
        );
        assert_eq!(parse_inlines(r"a \* b \@here"), vec![Str("a * b @here".into())]);
    }

    #[test]
    fn structure_headings_lists_math() {
        let els = structure("## The balance rule\n\nKeep it balanced.\n\n$$3x = 15$$\n\n- first item\n- second\n  continues\n\n1. one\n2. two\n");
        assert_eq!(els.len(), 5);
        assert!(matches!(&els[0], BodyElement::Heading { level: 2, .. }));
        assert!(matches!(&els[1], BodyElement::Paragraph(_)));
        assert_eq!(els[2], BodyElement::DisplayMath("3x = 15".into()));
        match &els[3] {
            BodyElement::BulletList(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[1], vec![Str("second continues".into())]);
            }
            other => panic!("expected bullet list, got {other:?}"),
        }
        assert!(matches!(&els[4], BodyElement::OrderedList(items) if items.len() == 2));
    }

    #[test]
    fn comments_are_dropped() {
        let els = structure("<!-- a note\n   spanning lines -->\nReal text.");
        assert_eq!(els, vec![BodyElement::Paragraph(vec![Str("Real text.".into())])]);
    }
}
