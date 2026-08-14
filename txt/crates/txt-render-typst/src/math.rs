//! TeX-subset → Typst math translation. The subset is defined by
//! format.md §5 — `\frac`, `\dfrac`, `\cdot`, `\quad`, `\qquad`, and
//! `^`/`_` scripts. Unknown commands are an error by design: the set
//! grows by RFC, never by renderer leniency.

pub fn tex_to_typst(src: &str) -> Result<String, String> {
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    let out = convert(&chars, &mut i, false)?;
    Ok(out.split_whitespace().collect::<Vec<_>>().join(" "))
}

fn convert(chars: &[char], i: &mut usize, in_group: bool) -> Result<String, String> {
    let mut out = String::new();
    while *i < chars.len() {
        let c = chars[*i];
        match c {
            '}' if in_group => return Ok(out),
            '}' => return Err("unmatched `}`".into()),
            '\\' => {
                *i += 1;
                let name: String = {
                    let mut n = String::new();
                    while *i < chars.len() && chars[*i].is_ascii_alphabetic() {
                        n.push(chars[*i]);
                        *i += 1;
                    }
                    n
                };
                match name.as_str() {
                    "cdot" => out.push_str(" dot "),
                    "quad" => out.push_str(" quad "),
                    "qquad" => out.push_str(" quad quad "),
                    "frac" | "dfrac" => {
                        let a = brace_group(chars, i)?;
                        let b = brace_group(chars, i)?;
                        out.push_str(&format!(" ({a})/({b}) "));
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
                let script = if chars.get(*i) == Some(&'{') {
                    format!("({})", brace_group(chars, i)?)
                } else if let Some(&sc) = chars.get(*i) {
                    *i += 1;
                    sc.to_string()
                } else {
                    return Err(format!("`{c}` at end of math"));
                };
                out.push(c);
                out.push_str(&script);
                out.push(' ');
            }
            '{' => {
                let inner = brace_group(chars, i)?;
                out.push_str(&inner);
            }
            d if d.is_ascii_digit() => {
                while *i < chars.len() && chars[*i].is_ascii_digit() {
                    out.push(chars[*i]);
                    *i += 1;
                }
                // `3x` must become `3 x` — juxtaposed letters would read
                // as a multi-letter identifier in Typst.
                if chars.get(*i).is_some_and(|c| c.is_ascii_alphabetic()) {
                    out.push(' ');
                }
            }
            a if a.is_ascii_alphabetic() => {
                out.push(a);
                *i += 1;
                if chars.get(*i).is_some_and(|c| c.is_ascii_alphanumeric()) {
                    out.push(' ');
                }
            }
            other => {
                out.push(other);
                *i += 1;
            }
        }
    }
    if in_group {
        Err("unclosed `{` in math".into())
    } else {
        Ok(out)
    }
}

fn brace_group(chars: &[char], i: &mut usize) -> Result<String, String> {
    while chars.get(*i) == Some(&' ') {
        *i += 1;
    }
    if chars.get(*i) != Some(&'{') {
        return Err("expected `{` after command".into());
    }
    *i += 1;
    let inner = convert(chars, i, true)?;
    if chars.get(*i) != Some(&'}') {
        return Err("unclosed `{` in math".into());
    }
    *i += 1;
    Ok(inner.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::tex_to_typst;

    #[test]
    fn juxtaposition_gets_spaces() {
        assert_eq!(tex_to_typst("3x + 4 = 19").unwrap(), "3 x + 4 = 19");
        assert_eq!(tex_to_typst("5x - 2 = 13").unwrap(), "5 x - 2 = 13");
        assert_eq!(tex_to_typst("2(x + 3) = 2x + 4").unwrap(), "2(x + 3) = 2 x + 4");
    }

    #[test]
    fn commands_translate() {
        assert_eq!(tex_to_typst(r"3 \cdot 6 = 18").unwrap(), "3 dot 6 = 18");
        assert_eq!(tex_to_typst(r"\dfrac{x}{4} + 7 = 12").unwrap(), "(x)/(4) + 7 = 12");
        assert_eq!(tex_to_typst(r"5x = 20 \qquad x = 4").unwrap(), "5 x = 20 quad quad x = 4");
    }

    #[test]
    fn scripts() {
        assert_eq!(tex_to_typst("x^2 = 9").unwrap(), "x^2 = 9");
        assert_eq!(tex_to_typst("a^{n+1}").unwrap(), "a^(n+1)");
    }

    #[test]
    fn unknown_command_is_an_error() {
        let e = tex_to_typst(r"\mathbb{R}").unwrap_err();
        assert!(e.contains("mathbb"));
    }
}
