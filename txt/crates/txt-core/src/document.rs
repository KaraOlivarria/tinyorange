//! The `.tx` document model: what a file *is* after parsing.

use serde::Serialize;
use std::collections::BTreeMap;

/// Parsed TOML frontmatter. `id`, `kind`, and `title` are required by the
/// format; unrecognized keys are preserved in `extra`.
#[derive(Debug, Clone, Serialize)]
pub struct Frontmatter {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub standards: Vec<String>,
    pub extra: BTreeMap<String, toml::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Document {
    pub frontmatter: Frontmatter,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Node {
    Block(Block),
    Text(Text),
}

/// A run of body lines between block markers.
#[derive(Debug, Clone, Serialize)]
pub struct Text {
    /// 1-based line number of the first line of the run.
    pub line: usize,
    pub content: String,
}

/// A `::: kind {#id key="value"}` directive block.
#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub kind: String,
    pub id: Option<String>,
    pub attrs: BTreeMap<String, String>,
    /// 1-based line number of the opening marker.
    pub line: usize,
    pub children: Vec<Node>,
}

impl Document {
    /// Every block in the document, depth-first.
    pub fn blocks(&self) -> Vec<&Block> {
        fn walk<'a>(nodes: &'a [Node], out: &mut Vec<&'a Block>) {
            for n in nodes {
                if let Node::Block(b) = n {
                    out.push(b);
                    walk(&b.children, out);
                }
            }
        }
        let mut out = Vec::new();
        walk(&self.nodes, &mut out);
        out
    }

    /// Every text run in the document, including inside blocks.
    pub fn texts(&self) -> Vec<&Text> {
        fn walk<'a>(nodes: &'a [Node], out: &mut Vec<&'a Text>) {
            for n in nodes {
                match n {
                    Node::Text(t) => out.push(t),
                    Node::Block(b) => walk(&b.children, out),
                }
            }
        }
        let mut out = Vec::new();
        walk(&self.nodes, &mut out);
        out
    }
}
