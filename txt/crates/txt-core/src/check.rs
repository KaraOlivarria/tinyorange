//! Build-time checks (`txt check`): the rules that make a build a
//! guarantee. The block vocabulary itself comes from the embedded schema
//! (`txt/schema/blocks.toml`); accessibility rules are errors, not
//! warnings, by design (spec §1.5).

use crate::document::{Block, Document, Node};
use crate::schema::{BlockSchema, IdRequirement};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// 1-based line number.
    pub line: usize,
    pub rule: &'static str,
    pub severity: Severity,
    pub message: String,
}

impl Diagnostic {
    pub fn error(line: usize, rule: &'static str, message: impl Into<String>) -> Self {
        Self { line, rule, severity: Severity::Error, message: message.into() }
    }
}

/// Rules that need only a single file, validated against the v1 schema.
pub fn check_document(doc: &Document) -> Vec<Diagnostic> {
    let schema = BlockSchema::v1();
    let mut out = Vec::new();
    check_nodes(&doc.nodes, None, schema, &mut out);
    out
}

fn check_nodes(
    nodes: &[Node],
    parent: Option<&Block>,
    schema: &BlockSchema,
    out: &mut Vec<Diagnostic>,
) {
    for node in nodes {
        let Node::Block(b) = node else { continue };

        match schema.kind(&b.kind) {
            None => {
                out.push(Diagnostic::error(
                    b.line,
                    "unknown-block-kind",
                    format!("`{}` is not in the block vocabulary (schema/blocks.toml)", b.kind),
                ));
            }
            Some(spec) => {
                if spec.id == IdRequirement::Required && b.id.is_none() {
                    out.push(Diagnostic::error(
                        b.line,
                        "missing-block-id",
                        format!("`{}` blocks require a stable `{{#id}}`", b.kind),
                    ));
                }
                if let Some(allowed_parents) = &spec.parents {
                    let ok = parent.is_some_and(|p| allowed_parents.contains(&p.kind));
                    if !ok {
                        out.push(Diagnostic::error(
                            b.line,
                            "misplaced-block",
                            format!(
                                "`{}` may only appear inside {}",
                                b.kind,
                                allowed_parents.join("/")
                            ),
                        ));
                    }
                }
                for attr in b.attrs.keys() {
                    if !schema.attr_allowed(spec, attr) {
                        out.push(Diagnostic::error(
                            b.line,
                            "unknown-attr",
                            format!("`{}` blocks do not take attribute `{attr}`", b.kind),
                        ));
                    }
                }
                for required in &spec.requires_attrs {
                    if !b.attrs.contains_key(required) {
                        out.push(Diagnostic::error(
                            b.line,
                            "missing-attr",
                            format!("`{}` blocks require a `{required}` attribute", b.kind),
                        ));
                    }
                }
                if !spec.fields.is_empty() {
                    let fields = directive_fields(b, &spec.fields);
                    for required in &spec.requires_fields {
                        match fields.get(required) {
                            Some(v) if !v.trim().is_empty() => {}
                            _ => {
                                let extra = if b.kind == "figure" && required == "alt" {
                                    " — builds without alt text are refused"
                                } else {
                                    ""
                                };
                                out.push(Diagnostic::error(
                                    b.line,
                                    "missing-field",
                                    format!(
                                        "`{}` blocks require a non-empty `{required}:` field{extra}",
                                        b.kind
                                    ),
                                ));
                            }
                        }
                    }
                }
            }
        }

        if let Some(audience) = b.attrs.get("audience") {
            if audience != "teacher" {
                out.push(Diagnostic::error(
                    b.line,
                    "invalid-audience",
                    format!("audience `{audience}` is not recognized (v1 supports `teacher`)"),
                ));
            }
        }

        check_nodes(&b.children, Some(b), schema, out);
    }
}

/// Whole-book rules: id uniqueness, `@ref` resolution, figure sources on
/// disk. `files` pairs each document with its path.
pub fn check_book(files: &[(PathBuf, Document)]) -> Vec<(PathBuf, Diagnostic)> {
    let mut out: Vec<(PathBuf, Diagnostic)> = Vec::new();

    for (path, doc) in files {
        for d in check_document(doc) {
            out.push((path.clone(), d));
        }
    }

    // Collect every declared id (frontmatter + blocks), flagging duplicates.
    let mut ids: BTreeMap<String, (PathBuf, usize)> = BTreeMap::new();
    for (path, doc) in files {
        let mut declare = |id: &str, line: usize, out: &mut Vec<(PathBuf, Diagnostic)>| {
            if let Some((first_path, first_line)) = ids.get(id) {
                out.push((
                    path.clone(),
                    Diagnostic::error(
                        line,
                        "duplicate-id",
                        format!(
                            "id `{id}` already declared at {}:{first_line}",
                            first_path.display()
                        ),
                    ),
                ));
            } else {
                ids.insert(id.to_string(), (path.clone(), line));
            }
        };
        declare(&doc.frontmatter.id, 1, &mut out);
        for b in doc.blocks() {
            if let Some(id) = &b.id {
                declare(id, b.line, &mut out);
            }
        }
    }

    // Every `@ref` must resolve somewhere in the book.
    for (path, doc) in files {
        for t in doc.texts() {
            for (offset, line) in t.content.lines().enumerate() {
                for r in scan_refs(line) {
                    if !ids.contains_key(&r) {
                        out.push((
                            path.clone(),
                            Diagnostic::error(
                                t.line + offset,
                                "dangling-ref",
                                format!("reference `@{r}` does not resolve to any id in this book"),
                            ),
                        ));
                    }
                }
            }
        }
    }

    // Figure sources must exist relative to the declaring file.
    for (path, doc) in files {
        let base = path.parent().unwrap_or(Path::new("."));
        for b in doc.blocks() {
            if b.kind == "figure" {
                if let Some(src) = b.attrs.get("src") {
                    if !base.join(src).exists() {
                        out.push((
                            path.clone(),
                            Diagnostic::error(
                                b.line,
                                "figure-src-not-found",
                                format!(
                                    "figure source `{src}` not found relative to {}",
                                    base.display()
                                ),
                            ),
                        ));
                    }
                }
            }
        }
    }

    out
}

/// Extracts recognized `key: value` fields (with indented continuation
/// lines) from a block's direct text content. Only keys in `allowed` are
/// treated as fields; everything else stays ordinary text, so prose
/// containing colons is never misread.
pub fn directive_fields(block: &Block, allowed: &[String]) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    let mut current: Option<String> = None;
    for node in &block.children {
        let Node::Text(t) = node else { continue };
        for line in t.content.lines() {
            let indented = line.starts_with(char::is_whitespace);
            if !indented {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    if allowed.iter().any(|a| a == key) {
                        fields.insert(key.to_string(), value.trim().to_string());
                        current = Some(key.to_string());
                        continue;
                    }
                }
                current = None;
            } else if let Some(key) = &current {
                let entry = fields.get_mut(key).expect("current key always present");
                entry.push(' ');
                entry.push_str(line.trim());
            }
        }
    }
    fields
}

/// Finds `@ref-name` cross-references in a line of text. Skips citation
/// syntax (`[@key]` resolves against the bibliography, not block ids) and
/// email-like text (`a@b` — the `@` must not follow an alphanumeric).
pub fn scan_refs(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut refs = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '@' {
            i += 1;
            continue;
        }
        let prev = if i == 0 { None } else { Some(chars[i - 1]) };
        let mut j = i + 1;
        let mut name = String::new();
        while j < chars.len()
            && (chars[j].is_ascii_alphanumeric() || chars[j] == '-' || chars[j] == '_')
        {
            name.push(chars[j]);
            j += 1;
        }
        let starts_ok = name.chars().next().is_some_and(|c| c.is_ascii_alphabetic());
        let prev_ok = !prev.is_some_and(|c| c.is_alphanumeric() || c == '[' || c == '\\');
        if starts_ok && prev_ok {
            refs.push(name);
        }
        i = j.max(i + 1);
    }
    refs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn doc(body: &str) -> Document {
        let src = format!("+++\nid = \"t\"\nkind = \"section\"\ntitle = \"T\"\n+++\n{body}");
        parse(&src).unwrap()
    }

    #[test]
    fn figure_without_alt_is_an_error() {
        let d = doc("::: figure {#f src=\"x.svg\"}\ncaption: A caption only.\n:::\n");
        let diags = check_document(&d);
        assert!(diags.iter().any(|d| d.rule == "missing-field"), "{diags:?}");
    }

    #[test]
    fn figure_with_alt_passes() {
        let d = doc("::: figure {#f src=\"x.svg\"}\nalt: A description\n  spanning two lines.\ncaption: Cap.\n:::\n");
        let diags = check_document(&d);
        assert!(!diags.iter().any(|d| d.rule == "missing-field"), "{diags:?}");
    }

    #[test]
    fn figure_without_src_is_an_error() {
        let d = doc("::: figure {#f}\nalt: Description.\n:::\n");
        let diags = check_document(&d);
        assert!(diags.iter().any(|d| d.rule == "missing-attr"), "{diags:?}");
    }

    #[test]
    fn unknown_kind_missing_id_and_unknown_attr_flagged() {
        let d = doc("::: mystery\nhello\n:::\n\n::: definition\nno id here\n:::\n\n::: note {#n color=\"red\"}\nhi\n:::\n");
        let rules: Vec<_> = check_document(&d).iter().map(|d| d.rule).collect();
        assert!(rules.contains(&"unknown-block-kind"));
        assert!(rules.contains(&"missing-block-id"));
        assert!(rules.contains(&"unknown-attr"));
    }

    #[test]
    fn misplaced_solution_flagged() {
        let d = doc("::: solution\nfloating free\n:::\n");
        let diags = check_document(&d);
        assert!(diags.iter().any(|d| d.rule == "misplaced-block"), "{diags:?}");
    }

    #[test]
    fn nested_solution_passes() {
        let d = doc("::: example {#e}\nSolve.\n\n::: solution\nDone.\n:::\n:::\n");
        let diags = check_document(&d);
        assert!(!diags.iter().any(|d| d.rule == "misplaced-block"), "{diags:?}");
    }

    #[test]
    fn invalid_audience_flagged() {
        let d = doc("::: exercise {#e}\nQ?\n\n::: answer {audience=\"grader\"}\nx = 1\n:::\n:::\n");
        let diags = check_document(&d);
        assert!(diags.iter().any(|d| d.rule == "invalid-audience"));
    }

    #[test]
    fn prose_colons_are_not_fields() {
        let d = doc("::: figure {#f src=\"x.svg\"}\nalt: Real alt text.\nCheck: this line is prose, not a field.\n:::\n");
        let diags = check_document(&d);
        assert!(diags.is_empty(), "{diags:?}");
    }

    #[test]
    fn ref_scanning_skips_citations_and_emails() {
        assert_eq!(scan_refs("see @fig-one and (@ex-two)."), vec!["fig-one", "ex-two"]);
        assert!(scan_refs("cite [@smith2024] and mail a@b.com").is_empty());
    }

    #[test]
    fn dangling_ref_detected_across_book() {
        let d = doc("See @nowhere for details.\n");
        let problems = check_book(&[(std::path::PathBuf::from("t.tx"), d)]);
        assert!(problems.iter().any(|(_, d)| d.rule == "dangling-ref"));
    }
}
