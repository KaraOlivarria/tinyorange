//! The block vocabulary schema, embedded from `txt/schema/blocks.toml` —
//! the normative definition of which blocks exist and how they may be
//! used (format.md §3). `txt check` enforces it; renderers consume it.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

const SCHEMA_V1: &str = include_str!("../../../schema/blocks.toml");

/// Attributes allowed on every block kind.
pub const UNIVERSAL_ATTRS: &[&str] = &["audience"];

#[derive(Debug, Deserialize)]
pub struct BlockSchema {
    pub version: u32,
    pub kinds: BTreeMap<String, KindSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KindSpec {
    pub id: IdRequirement,
    #[serde(default)]
    pub attrs: Vec<String>,
    #[serde(default)]
    pub requires_attrs: Vec<String>,
    /// If set, the block may appear only inside these kinds.
    #[serde(default)]
    pub parents: Option<Vec<String>>,
    /// `key: value` field lines recognized in the block body.
    #[serde(default)]
    pub fields: Vec<String>,
    #[serde(default)]
    pub requires_fields: Vec<String>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IdRequirement {
    Required,
    Optional,
}

impl BlockSchema {
    /// The embedded v1 schema.
    pub fn v1() -> &'static BlockSchema {
        static SCHEMA: OnceLock<BlockSchema> = OnceLock::new();
        SCHEMA.get_or_init(|| {
            toml::from_str(SCHEMA_V1).expect("embedded blocks.toml must be valid")
        })
    }

    pub fn kind(&self, name: &str) -> Option<&KindSpec> {
        self.kinds.get(name)
    }

    pub fn attr_allowed(&self, kind: &KindSpec, attr: &str) -> bool {
        UNIVERSAL_ATTRS.contains(&attr) || kind.attrs.iter().any(|a| a == attr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_schema_loads() {
        let s = BlockSchema::v1();
        assert_eq!(s.version, 1);
        assert!(s.kind("definition").is_some());
        assert_eq!(s.kind("figure").unwrap().requires_fields, vec!["alt"]);
        assert_eq!(
            s.kind("solution").unwrap().parents.as_deref(),
            Some(&["example".to_string(), "exercise".into(), "activity".into()][..])
        );
    }
}
