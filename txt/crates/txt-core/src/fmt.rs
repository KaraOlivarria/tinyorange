//! `txt fmt`: the canonical formatter. One canonical form (attribute
//! order, quoting, blank-line spacing) so diffs show meaning, not style —
//! and auto-generation of missing required ids so authors aren't burdened
//! (format.md §3). Formatting must round-trip: parse(fmt(x)) preserves
//! the document, and fmt is idempotent.

use crate::document::{Block, Document, Node};
use crate::parser::{parse, ParseError};
use crate::schema::{BlockSchema, IdRequirement};

/// Parses, assigns missing ids, and reformats. The result is canonical.
pub fn format_source(src: &str) -> Result<String, ParseError> {
    let mut doc = parse(src)?;
    assign_missing_ids(&mut doc);
    Ok(format_document(&doc))
}

pub fn format_document(doc: &Document) -> String {
    let mut out = String::new();
    out.push_str("+++\n");
    push_kv(&mut out, "id", &doc.frontmatter.id);
    push_kv(&mut out, "kind", &doc.frontmatter.kind);
    push_kv(&mut out, "title", &doc.frontmatter.title);
    if !doc.frontmatter.standards.is_empty() {
        let arr = toml::Value::Array(
            doc.frontmatter
                .standards
                .iter()
                .map(|s| toml::Value::String(s.clone()))
                .collect(),
        );
        out.push_str(&format!("standards = {arr}\n"));
    }
    for (k, v) in &doc.frontmatter.extra {
        out.push_str(&format!("{k} = {v}\n"));
    }
    out.push_str("+++\n");
    if !doc.nodes.is_empty() {
        out.push('\n');
        emit_nodes(&doc.nodes, &mut out);
    }
    out
}

fn push_kv(out: &mut String, key: &str, value: &str) {
    out.push_str(&format!("{key} = {}\n", toml::Value::String(value.to_string())));
}

fn emit_nodes(nodes: &[Node], out: &mut String) {
    for (i, node) in nodes.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        match node {
            Node::Text(t) => {
                out.push_str(&t.content);
                out.push('\n');
            }
            Node::Block(b) => {
                out.push_str(&open_marker(b));
                out.push('\n');
                emit_nodes(&b.children, out);
                out.push_str(":::\n");
            }
        }
    }
}

/// Canonical opening marker: `::: kind {#id a=1 b="two"}` — id first,
/// attributes alphabetical, integers bare, everything else quoted,
/// `"true"` as a bare flag.
fn open_marker(b: &Block) -> String {
    let mut attrs = Vec::new();
    if let Some(id) = &b.id {
        attrs.push(format!("#{id}"));
    }
    for (k, v) in &b.attrs {
        if v == "true" {
            attrs.push(k.clone());
        } else if !v.is_empty() && v.chars().all(|c| c.is_ascii_digit()) {
            attrs.push(format!("{k}={v}"));
        } else {
            attrs.push(format!("{k}=\"{v}\""));
        }
    }
    if attrs.is_empty() {
        format!("::: {}", b.kind)
    } else {
        format!("::: {} {{{}}}", b.kind, attrs.join(" "))
    }
}

/// Gives every id-required block that lacks an id a stable generated one:
/// `{kind}-{slug}-{hash}` (format.md §3). Returns how many were assigned.
pub fn assign_missing_ids(doc: &mut Document) -> usize {
    let schema = BlockSchema::v1();
    let mut count = 0;
    walk_assign(&mut doc.nodes, schema, &mut count);
    count
}

fn walk_assign(nodes: &mut [Node], schema: &BlockSchema, count: &mut usize) {
    for node in nodes {
        if let Node::Block(b) = node {
            if b.id.is_none()
                && schema
                    .kind(&b.kind)
                    .is_some_and(|s| s.id == IdRequirement::Required)
            {
                b.id = Some(generate_id(b));
                *count += 1;
            }
            walk_assign(&mut b.children, schema, count);
        }
    }
}

fn generate_id(b: &Block) -> String {
    let text = collect_text(b);
    let slug: String = text
        .split_whitespace()
        .take(3)
        .collect::<Vec<_>>()
        .join("-")
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c == '-' {
                Some('-')
            } else {
                None
            }
        })
        .collect();
    let slug = slug.trim_matches('-');
    let hash = fnv1a(&format!("{}\u{1f}{}", b.kind, text)) & 0xff_ffff;
    if slug.is_empty() {
        format!("{}-{hash:06x}", b.kind)
    } else {
        format!("{}-{slug}-{hash:06x}", b.kind)
    }
}

fn collect_text(b: &Block) -> String {
    let mut out = String::new();
    fn walk(nodes: &[Node], out: &mut String) {
        for n in nodes {
            match n {
                Node::Text(t) => {
                    out.push_str(&t.content);
                    out.push('\n');
                }
                Node::Block(b) => walk(&b.children, out),
            }
        }
    }
    walk(&b.children, &mut out);
    out
}

fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in s.bytes() {
        h ^= u64::from(byte);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSY: &str = "+++\ntitle = \"T\"\nkind = \"section\"\nid = \"t\"\n+++\n\n\n::: exercise   {standards=\"X.1\" difficulty=2 #e1}\nSolve.\n\n\n::: answer {audience=\"teacher\"}\nDone.\n:::\n:::\n";

    #[test]
    fn canonicalizes_and_is_idempotent() {
        let once = format_source(MESSY).unwrap();
        assert!(once.contains("::: exercise {#e1 difficulty=2 standards=\"X.1\"}"), "{once}");
        assert!(once.starts_with("+++\nid = \"t\"\nkind = \"section\"\ntitle = \"T\"\n+++\n"), "{once}");
        let twice = format_source(&once).unwrap();
        assert_eq!(once, twice);
    }

    #[test]
    fn generates_missing_ids_stably() {
        let src = "+++\nid = \"t\"\nkind = \"section\"\ntitle = \"T\"\n+++\n::: definition\nA **prime** has exactly two divisors.\n:::\n";
        let a = format_source(src).unwrap();
        let b = format_source(src).unwrap();
        assert_eq!(a, b);
        assert!(a.contains("::: definition {#definition-a-prime-has-"), "{a}");
        // formatting again keeps the generated id
        assert_eq!(format_source(&a).unwrap(), a);
    }

    #[test]
    fn flags_and_quoting() {
        let src = "+++\nid = \"t\"\nkind = \"section\"\ntitle = \"T\"\n+++\n::: note {#n draft term=\"two words\"}\nhi\n:::\n";
        let out = format_source(src).unwrap();
        assert!(out.contains("::: note {#n draft term=\"two words\"}"), "{out}");
    }
}
