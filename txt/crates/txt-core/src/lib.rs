//! txt-core: parser, document model, and build-time checks for the `.tx`
//! open-textbook format.
//!
//! The format is specified in the Texture founding repo under
//! `docs/txt/format.md`; this crate is its reference implementation.

pub mod check;
pub mod document;
pub mod parser;

pub use check::{check_book, check_document, Diagnostic, Severity};
pub use document::{Block, Document, Frontmatter, Node, Text};
pub use parser::{parse, ParseError};
