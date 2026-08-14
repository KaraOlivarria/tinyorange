//! Typst emission for the semantic math tree (txt-core `math` module).

use txt_core::math::{parse_math, MathNode};

pub fn tex_to_typst(src: &str) -> Result<String, String> {
    Ok(emit_seq(&parse_math(src)?))
}

fn emit_seq(nodes: &[MathNode]) -> String {
    nodes.iter().map(emit).collect::<Vec<_>>().join(" ")
}

fn emit(node: &MathNode) -> String {
    match node {
        MathNode::Num(n) => n.clone(),
        MathNode::Ident(c) => c.to_string(),
        MathNode::Op(c) => c.to_string(),
        MathNode::Frac(a, b) => format!("({})/({})", emit_seq(a), emit_seq(b)),
        MathNode::Sup(base, script) => format!("{}^({})", emit(base), emit_seq(script)),
        MathNode::Sub(base, script) => format!("{}_({})", emit(base), emit_seq(script)),
        MathNode::Cdot => "dot".into(),
        MathNode::Quad => "quad".into(),
        MathNode::QQuad => "quad quad".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::tex_to_typst;

    #[test]
    fn juxtaposition_gets_spaces() {
        assert_eq!(tex_to_typst("3x + 4 = 19").unwrap(), "3 x + 4 = 19");
        assert_eq!(tex_to_typst("5x - 2 = 13").unwrap(), "5 x - 2 = 13");
        assert_eq!(tex_to_typst("2(x + 3) = 2x + 4").unwrap(), "2 ( x + 3 ) = 2 x + 4");
    }

    #[test]
    fn commands_translate() {
        assert_eq!(tex_to_typst(r"3 \cdot 6 = 18").unwrap(), "3 dot 6 = 18");
        assert_eq!(tex_to_typst(r"\dfrac{x}{4} + 7 = 12").unwrap(), "(x)/(4) + 7 = 12");
        assert_eq!(
            tex_to_typst(r"5x = 20 \qquad x = 4").unwrap(),
            "5 x = 20 quad quad x = 4"
        );
    }

    #[test]
    fn scripts() {
        assert_eq!(tex_to_typst("x^2 = 9").unwrap(), "x^(2) = 9");
        assert_eq!(tex_to_typst("a^{n+1}").unwrap(), "a^(n + 1)");
    }

    #[test]
    fn unknown_command_is_an_error() {
        let e = tex_to_typst(r"\mathbb{R}").unwrap_err();
        assert!(e.contains("mathbb"));
    }
}
