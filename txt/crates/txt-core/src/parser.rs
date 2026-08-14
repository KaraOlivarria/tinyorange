//! Line-oriented parser for the `.tx` format (v0): TOML frontmatter between
//! `+++` fences, then a body of text runs and nested `:::` directive blocks.
//!
//! Known v0 limitation: `:::` markers inside fenced code blocks are not yet
//! ignored (tracked in txt/README.md).

use crate::document::{Block, Document, Frontmatter, Node, Text};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ParseError {
    /// 1-based line number.
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

fn err(line: usize, message: impl Into<String>) -> ParseError {
    ParseError { line, message: message.into() }
}

pub fn parse(src: &str) -> Result<Document, ParseError> {
    let lines: Vec<&str> = src.lines().collect();
    let (frontmatter, body_start) = parse_frontmatter(&lines)?;
    let nodes = parse_body(&lines, body_start)?;
    Ok(Document { frontmatter, nodes })
}

fn take_string(
    table: &mut toml::value::Table,
    key: &str,
    line: usize,
) -> Result<String, ParseError> {
    match table.remove(key) {
        Some(toml::Value::String(s)) => Ok(s),
        Some(_) => Err(err(line, format!("frontmatter field `{key}` must be a string"))),
        None => Err(err(line, format!("frontmatter is missing required field `{key}`"))),
    }
}

fn parse_frontmatter(lines: &[&str]) -> Result<(Frontmatter, usize), ParseError> {
    let mut open = 0;
    while open < lines.len() && lines[open].trim().is_empty() {
        open += 1;
    }
    if open >= lines.len() || lines[open].trim() != "+++" {
        return Err(err(open + 1, "file must begin with `+++` TOML frontmatter"));
    }
    let close = lines
        .iter()
        .enumerate()
        .skip(open + 1)
        .find(|(_, l)| l.trim() == "+++")
        .map(|(i, _)| i)
        .ok_or_else(|| err(open + 1, "unterminated frontmatter: missing closing `+++`"))?;

    let toml_src = lines[open + 1..close].join("\n");
    let value: toml::Value = toml::from_str(&toml_src)
        .map_err(|e| err(open + 2, format!("invalid TOML in frontmatter: {e}")))?;
    let mut table = match value {
        toml::Value::Table(t) => t,
        _ => return Err(err(open + 2, "frontmatter must be a TOML table")),
    };

    let fm_line = open + 2;
    let id = take_string(&mut table, "id", fm_line)?;
    let kind = take_string(&mut table, "kind", fm_line)?;
    let title = take_string(&mut table, "title", fm_line)?;
    let standards = match table.remove("standards") {
        None => Vec::new(),
        Some(toml::Value::Array(a)) => a
            .into_iter()
            .map(|v| match v {
                toml::Value::String(s) => Ok(s),
                _ => Err(err(fm_line, "`standards` must be an array of strings")),
            })
            .collect::<Result<Vec<_>, _>>()?,
        Some(_) => return Err(err(fm_line, "`standards` must be an array of strings")),
    };

    let frontmatter = Frontmatter {
        id,
        kind,
        title,
        standards,
        extra: table.into_iter().collect(),
    };
    Ok((frontmatter, close + 1))
}

struct BodyParser {
    stack: Vec<Block>,
    root: Vec<Node>,
    buf: Vec<(usize, String)>,
}

impl BodyParser {
    fn attach(&mut self, node: Node) {
        match self.stack.last_mut() {
            Some(top) => top.children.push(node),
            None => self.root.push(node),
        }
    }

    fn flush_text(&mut self) {
        while self.buf.first().is_some_and(|(_, s)| s.trim().is_empty()) {
            self.buf.remove(0);
        }
        while self.buf.last().is_some_and(|(_, s)| s.trim().is_empty()) {
            self.buf.pop();
        }
        if self.buf.is_empty() {
            return;
        }
        let line = self.buf[0].0;
        let content = self
            .buf
            .iter()
            .map(|(_, s)| s.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        self.buf.clear();
        self.attach(Node::Text(Text { line, content }));
    }
}

fn parse_body(lines: &[&str], start: usize) -> Result<Vec<Node>, ParseError> {
    let mut p = BodyParser { stack: Vec::new(), root: Vec::new(), buf: Vec::new() };
    for (idx, raw) in lines.iter().enumerate().skip(start) {
        let ln = idx + 1;
        let t = raw.trim();
        if t == ":::" {
            p.flush_text();
            match p.stack.pop() {
                Some(done) => p.attach(Node::Block(done)),
                None => return Err(err(ln, "closing `:::` with no open block")),
            }
        } else if let Some(rest) = t.strip_prefix(":::") {
            if !rest.starts_with(char::is_whitespace) {
                return Err(err(ln, format!("malformed block marker `{t}`; expected `::: <kind>`")));
            }
            p.flush_text();
            let block = parse_open(rest.trim(), ln)?;
            p.stack.push(block);
        } else {
            p.buf.push((ln, raw.trim_end().to_string()));
        }
    }
    p.flush_text();
    if let Some(open) = p.stack.last() {
        return Err(err(open.line, format!("unclosed `::: {}` block", open.kind)));
    }
    Ok(p.root)
}

fn parse_open(rest: &str, line: usize) -> Result<Block, ParseError> {
    let (kind_part, attr_part) = match rest.find('{') {
        Some(pos) => {
            let attrs = rest[pos..].trim();
            if !attrs.ends_with('}') {
                return Err(err(line, "unterminated `{...}` attribute list"));
            }
            (rest[..pos].trim(), Some(&attrs[1..attrs.len() - 1]))
        }
        None => (rest, None),
    };
    if kind_part.is_empty()
        || !kind_part.chars().all(|c| c.is_ascii_lowercase() || c == '-')
    {
        return Err(err(line, format!("invalid block kind `{kind_part}`")));
    }
    let (id, attrs) = match attr_part {
        Some(a) => parse_attrs(a, line)?,
        None => (None, BTreeMap::new()),
    };
    Ok(Block { kind: kind_part.to_string(), id, attrs, line, children: Vec::new() })
}

/// Parses `#id key=value key="quoted value"` (already stripped of braces).
fn parse_attrs(
    inner: &str,
    line: usize,
) -> Result<(Option<String>, BTreeMap<String, String>), ParseError> {
    let mut tokens: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for c in inner.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                cur.push(c);
            }
            c if c.is_whitespace() && !in_quotes => {
                if !cur.is_empty() {
                    tokens.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if in_quotes {
        return Err(err(line, "unterminated quote in attribute list"));
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }

    let mut id = None;
    let mut attrs = BTreeMap::new();
    for tok in tokens {
        if let Some(rest) = tok.strip_prefix('#') {
            if id.replace(rest.to_string()).is_some() {
                return Err(err(line, "more than one `#id` in attribute list"));
            }
        } else if let Some((k, v)) = tok.split_once('=') {
            attrs.insert(k.to_string(), v.trim_matches('"').to_string());
        } else {
            attrs.insert(tok, "true".to_string());
        }
    }
    Ok((id, attrs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Node;

    const MINIMAL: &str = r#"+++
id = "t"
kind = "section"
title = "Test"
+++

Intro text.

::: example {#ex-one difficulty=2}
Solve $x = 1$.

::: solution
Trivially, $x = 1$.
:::
:::
"#;

    #[test]
    fn parses_nested_blocks() {
        let doc = parse(MINIMAL).unwrap();
        assert_eq!(doc.frontmatter.id, "t");
        assert_eq!(doc.nodes.len(), 2); // intro text + example block
        let blocks = doc.blocks();
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].kind, "example");
        assert_eq!(blocks[0].id.as_deref(), Some("ex-one"));
        assert_eq!(blocks[0].attrs.get("difficulty").map(String::as_str), Some("2"));
        assert_eq!(blocks[1].kind, "solution");
        assert!(blocks[1].id.is_none());
    }

    #[test]
    fn quoted_attrs_keep_spaces() {
        let (id, attrs) = parse_attrs(r#"#d term="linear equation""#, 1).unwrap();
        assert_eq!(id.as_deref(), Some("d"));
        assert_eq!(attrs.get("term").map(String::as_str), Some("linear equation"));
    }

    #[test]
    fn unmatched_close_is_an_error() {
        let src = "+++\nid = \"t\"\nkind = \"section\"\ntitle = \"T\"\n+++\n:::\n";
        let e = parse(src).unwrap_err();
        assert_eq!(e.line, 6);
    }

    #[test]
    fn unclosed_block_is_an_error() {
        let src = "+++\nid = \"t\"\nkind = \"section\"\ntitle = \"T\"\n+++\n::: note {#n}\nbody\n";
        let e = parse(src).unwrap_err();
        assert_eq!(e.line, 6);
        assert!(e.message.contains("unclosed"));
    }

    #[test]
    fn missing_frontmatter_field_is_an_error() {
        let src = "+++\nid = \"t\"\nkind = \"section\"\n+++\nbody\n";
        let e = parse(src).unwrap_err();
        assert!(e.message.contains("title"));
    }

    #[test]
    fn text_runs_keep_line_numbers() {
        let doc = parse(MINIMAL).unwrap();
        match &doc.nodes[0] {
            Node::Text(t) => {
                assert_eq!(t.line, 7);
                assert_eq!(t.content, "Intro text.");
            }
            other => panic!("expected text, got {other:?}"),
        }
    }
}
