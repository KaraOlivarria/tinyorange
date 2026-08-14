//! Golden test: the demonstration book in examples/ must always parse and
//! pass `txt check` cleanly. If a format change breaks this, either the
//! change needs a migration or the demo needs updating — never ignore it.

use std::path::PathBuf;
use txt_core::{check_book, parse};

fn demo_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../examples/algebra-1-demo")
}

#[test]
fn demo_book_parses_and_passes_checks() {
    let dir = demo_dir();
    let files = [
        "SUMMARY.tx",
        "chapters/01-linear-equations/chapter.tx",
        "chapters/01-linear-equations/exercises.tx",
    ];

    let mut parsed = Vec::new();
    for f in files {
        let path = dir.join(f);
        let src = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
        let doc = parse(&src).unwrap_or_else(|e| panic!("{f}: {e}"));
        parsed.push((path, doc));
    }

    let chapter = &parsed[1].1;
    assert_eq!(chapter.frontmatter.id, "ch-linear-eq");
    assert_eq!(chapter.frontmatter.standards, vec!["CCSS.MATH.8.EE.C.7"]);
    let kinds = |k: &str| chapter.blocks().iter().filter(|b| b.kind == k).count();
    assert_eq!(kinds("objective"), 2);
    assert_eq!(kinds("definition"), 3);
    assert_eq!(kinds("example"), 4);
    assert_eq!(kinds("solution"), 4);
    assert_eq!(kinds("figure"), 1);

    let exercises = &parsed[2].1;
    assert_eq!(
        exercises.blocks().iter().filter(|b| b.kind == "exercise").count(),
        7
    );
    // every answer is teacher-only
    for b in exercises.blocks() {
        if b.kind == "answer" {
            assert_eq!(b.attrs.get("audience").map(String::as_str), Some("teacher"));
        }
    }

    let problems = check_book(&parsed);
    let rendered: Vec<String> = problems
        .iter()
        .map(|(p, d)| format!("{}:{}: [{}] {}", p.display(), d.line, d.rule, d.message))
        .collect();
    assert!(problems.is_empty(), "expected clean check, got:\n{}", rendered.join("\n"));
}
