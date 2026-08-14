//! Book-level types and semantics shared by every renderer backend.

use crate::document::Document;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Where an `@ref` points: the human label ("Example 3") and the file it
/// lives in. Computed once, book-wide, so every backend numbers
/// identically.
#[derive(Debug, Clone)]
pub struct RefTarget {
    /// Frontmatter id of the file containing the target.
    pub file_id: String,
    /// Human-readable label: "Figure 1", "Example 3", or a file's title.
    pub label: String,
    /// True for block targets (anchorable), false for file-level ids.
    pub is_block: bool,
}

/// Numbers referenceable blocks per kind in book order and maps every id
/// (frontmatter + blocks) to its [`RefTarget`].
pub fn ref_targets(files: &[(PathBuf, Document)]) -> BTreeMap<String, RefTarget> {
    let mut targets = BTreeMap::new();
    let mut counters: BTreeMap<&'static str, usize> = BTreeMap::new();
    for (_, doc) in files {
        let file_id = doc.frontmatter.id.clone();
        targets.insert(
            file_id.clone(),
            RefTarget {
                file_id: file_id.clone(),
                label: doc.frontmatter.title.clone(),
                is_block: false,
            },
        );
        for b in doc.blocks() {
            let Some(id) = &b.id else { continue };
            let label = match b.kind.as_str() {
                "figure" => Some("Figure"),
                "example" => Some("Example"),
                "exercise" => Some("Exercise"),
                "definition" => Some("Definition"),
                "theorem" => Some("Theorem"),
                "table" => Some("Table"),
                _ => None,
            };
            let label = match label {
                Some(kind) => {
                    let n = counters.entry(kind).or_insert(0);
                    *n += 1;
                    format!("{kind} {n}")
                }
                None => format!("“{id}”"),
            };
            targets.insert(
                id.clone(),
                RefTarget { file_id: file_id.clone(), label, is_block: true },
            );
        }
    }
    targets
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Audience {
    Student,
    Teacher,
}

impl Audience {
    pub fn as_str(self) -> &'static str {
        match self {
            Audience::Student => "student",
            Audience::Teacher => "teacher",
        }
    }
}

#[derive(Debug, Clone)]
pub struct BookMeta {
    pub title: String,
    pub subtitle: Option<String>,
    pub edition: String,
    /// BCP 47 tag from book.toml (`en-US`); renderers may use the primary
    /// subtag for hyphenation.
    pub language: String,
    /// Optional repository URL — powers "report an issue" links.
    pub repository: Option<String>,
}

/// A file a build directory must contain (figure images, etc.).
#[derive(Debug, Clone)]
pub struct Asset {
    pub source: std::path::PathBuf,
    /// Destination path relative to the build directory.
    pub dest: String,
}
