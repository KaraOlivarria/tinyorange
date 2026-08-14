//! The v1 math subset (format.md §5), parsed into a semantic tree.
//! Renderers emit engine-native output (Typst math, MathML) from this
//! tree — TeX source is never passed through to any engine, and unknown
//! commands are errors: the subset grows by RFC, not by leniency.

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MathNode {
    /// A run of digits: `19`
    Num(String),
    /// A single-letter identifier, rendered italic: `x`
    Ident(char),
    /// An operator or punctuation character: `+ - = ( ) , .`
    Op(char),
    /// `\frac{a}{b}` / `\dfrac{a}{b}`
    Frac(Vec<MathNode>, Vec<MathNode>),
    /// `base^{script}`
    Sup(Box<MathNode>, Vec<MathNode>),
    /// `base_{script}`
    Sub(Box<MathNode>, Vec<MathNode>),
    /// `\cdot`
    Cdot,
    /// `\quad`
    Quad,
    /// `\qquad`
    QQuad,
}

pub fn parse_math(src: &str) -> Result<Vec<MathNode>, String> {
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    let seq = parse_seq(&chars, &mut i, false)?;
    if i < chars.len() {
        return Err(format!("unexpected `{}` in math", chars[i]));
    }
    Ok(seq)
}

fn parse_seq(chars: &[char], i: &mut usize, in_group: bool) -> Result<Vec<MathNode>, String> {
    let mut out: Vec<MathNode> = Vec::new();
    while *i < chars.len() {
        let c = chars[*i];
        match c {
            '}' if in_group => break,
            '}' => return Err("unmatched `}` in math".into()),
            w if w.is_whitespace() => {
                *i += 1;
            }
            '\\' => {
                *i += 1;
                let mut name = String::new();
                while *i < chars.len() && chars[*i].is_ascii_alphabetic() {
                    name.push(chars[*i]);
                    *i += 1;
                }
                match name.as_str() {
                    "cdot" => out.push(MathNode::Cdot),
                    "quad" => out.push(MathNode::Quad),
                    "qquad" => out.push(MathNode::QQuad),
                    "frac" | "dfrac" => {
                        let a = group(chars, i)?;
                        let b = group(chars, i)?;
                        out.push(MathNode::Frac(a, b));
                    }
                    "left" | "right" => {} // delimiters pass through as themselves
                    "" => return Err("stray `\\` in math".into()),
                    other => {
                        return Err(format!(
                            "unsupported TeX command `\\{other}` (v1 math subset, format.md §5)"
                        ))
                    }
                }
            }
            '^' | '_' => {
                *i += 1;
                let script = script_arg(chars, i, c)?;
                let base = out
                    .pop()
                    .ok_or_else(|| format!("`{c}` has nothing to attach to"))?;
                out.push(match c {
                    '^' => MathNode::Sup(Box::new(base), script),
                    _ => MathNode::Sub(Box::new(base), script),
                });
            }
            '{' => {
                let g = group(chars, i)?;
                out.extend(g);
            }
            d if d.is_ascii_digit() => {
                let mut num = String::new();
                while *i < chars.len() && chars[*i].is_ascii_digit() {
                    num.push(chars[*i]);
                    *i += 1;
                }
                out.push(MathNode::Num(num));
            }
            a if a.is_ascii_alphabetic() => {
                out.push(MathNode::Ident(a));
                *i += 1;
            }
            other => {
                out.push(MathNode::Op(other));
                *i += 1;
            }
        }
    }
    if in_group && (*i >= chars.len() || chars[*i] != '}') {
        return Err("unclosed `{` in math".into());
    }
    Ok(out)
}

fn group(chars: &[char], i: &mut usize) -> Result<Vec<MathNode>, String> {
    while chars.get(*i).is_some_and(|c| c.is_whitespace()) {
        *i += 1;
    }
    if chars.get(*i) != Some(&'{') {
        return Err("expected `{` after command".into());
    }
    *i += 1;
    let inner = parse_seq(chars, i, true)?;
    if chars.get(*i) != Some(&'}') {
        return Err("unclosed `{` in math".into());
    }
    *i += 1;
    Ok(inner)
}

/// A script argument: a `{group}` or a single token (TeX semantics —
/// `x^12` is x¹ followed by 2).
fn script_arg(chars: &[char], i: &mut usize, which: char) -> Result<Vec<MathNode>, String> {
    while chars.get(*i).is_some_and(|c| c.is_whitespace()) {
        *i += 1;
    }
    match chars.get(*i) {
        Some('{') => group(chars, i),
        Some(&d) if d.is_ascii_digit() => {
            *i += 1;
            Ok(vec![MathNode::Num(d.to_string())])
        }
        Some(&a) if a.is_ascii_alphabetic() => {
            *i += 1;
            Ok(vec![MathNode::Ident(a)])
        }
        _ => Err(format!("`{which}` needs a script (a token or {{group}})")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MathNode::*;

    #[test]
    fn basic_sequence() {
        assert_eq!(
            parse_math("3x + 4 = 19").unwrap(),
            vec![
                Num("3".into()),
                Ident('x'),
                Op('+'),
                Num("4".into()),
                Op('='),
                Num("19".into()),
            ]
        );
    }

    #[test]
    fn frac_scripts_cdot() {
        assert_eq!(
            parse_math(r"\dfrac{x}{4}").unwrap(),
            vec![Frac(vec![Ident('x')], vec![Num("4".into())])]
        );
        assert_eq!(
            parse_math("x^2").unwrap(),
            vec![Sup(Box::new(Ident('x')), vec![Num("2".into())])]
        );
        assert_eq!(
            parse_math("a^{n+1}").unwrap(),
            vec![Sup(
                Box::new(Ident('a')),
                vec![Ident('n'), Op('+'), Num("1".into())]
            )]
        );
        assert_eq!(
            parse_math(r"3 \cdot 6").unwrap(),
            vec![Num("3".into()), Cdot, Num("6".into())]
        );
    }

    #[test]
    fn tex_script_tokenization() {
        // x^12 is x¹ then 2, per TeX
        assert_eq!(
            parse_math("x^12").unwrap(),
            vec![
                Sup(Box::new(Ident('x')), vec![Num("1".into())]),
                Num("2".into())
            ]
        );
    }

    #[test]
    fn errors() {
        assert!(parse_math(r"\mathbb{R}").unwrap_err().contains("mathbb"));
        assert!(parse_math("^2").unwrap_err().contains("attach"));
        assert!(parse_math(r"\frac{1}").unwrap_err().contains("expected `{`"));
    }
}
