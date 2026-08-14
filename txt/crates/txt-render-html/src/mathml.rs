//! MathML Core emission for the semantic math tree (txt-core `math`).
//! MathML renders natively in all modern browsers and is what screen
//! readers consume — math accessibility is not an afterthought here.

use txt_core::math::{parse_math, MathNode};

pub fn tex_to_mathml(src: &str, display: bool) -> Result<String, String> {
    let nodes = parse_math(src)?;
    let display_attr = if display { " display=\"block\"" } else { "" };
    Ok(format!(
        "<math{display_attr}><mrow>{}</mrow></math>",
        emit_seq(&nodes)
    ))
}

fn emit_seq(nodes: &[MathNode]) -> String {
    nodes.iter().map(emit).collect()
}

fn emit(node: &MathNode) -> String {
    match node {
        MathNode::Num(n) => format!("<mn>{n}</mn>"),
        MathNode::Ident(c) => format!("<mi>{c}</mi>"),
        MathNode::Op(c) => format!("<mo>{}</mo>", esc(*c)),
        MathNode::Frac(a, b) => format!(
            "<mfrac><mrow>{}</mrow><mrow>{}</mrow></mfrac>",
            emit_seq(a),
            emit_seq(b)
        ),
        MathNode::Sup(base, script) => format!(
            "<msup>{}<mrow>{}</mrow></msup>",
            emit(base),
            emit_seq(script)
        ),
        MathNode::Sub(base, script) => format!(
            "<msub>{}<mrow>{}</mrow></msub>",
            emit(base),
            emit_seq(script)
        ),
        MathNode::Cdot => "<mo>⋅</mo>".into(),
        MathNode::Quad => "<mspace width=\"1em\"></mspace>".into(),
        MathNode::QQuad => "<mspace width=\"2em\"></mspace>".into(),
    }
}

fn esc(c: char) -> String {
    match c {
        '<' => "&lt;".into(),
        '>' => "&gt;".into(),
        '&' => "&amp;".into(),
        '-' => "−".into(), // U+2212 minus, not hyphen
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::tex_to_mathml;

    #[test]
    fn inline_and_display() {
        assert_eq!(
            tex_to_mathml("3x", false).unwrap(),
            "<math><mrow><mn>3</mn><mi>x</mi></mrow></math>"
        );
        assert!(tex_to_mathml("3x", true).unwrap().starts_with("<math display=\"block\">"));
    }

    #[test]
    fn frac_and_scripts() {
        let m = tex_to_mathml(r"\dfrac{x}{4}", false).unwrap();
        assert!(m.contains("<mfrac><mrow><mi>x</mi></mrow><mrow><mn>4</mn></mrow></mfrac>"), "{m}");
        let s = tex_to_mathml("x^2", false).unwrap();
        assert!(s.contains("<msup><mi>x</mi><mrow><mn>2</mn></mrow></msup>"), "{s}");
    }

    #[test]
    fn minus_becomes_true_minus() {
        assert!(tex_to_mathml("5 - 2", false).unwrap().contains("<mo>−</mo>"));
    }
}
