//! Symbol index for cross-reference resolution in multi-file doc generation.
//!
//! Builds a flat map from each symbol name to the HTML page and anchor where
//! it is documented, enabling the cross-reference resolver to emit correct
//! relative links between module pages.

use crate::walker::ModuleGroup;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// The resolved location of a symbol within the generated HTML output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolLocation {
    /// Relative path of the HTML page (e.g. `"Nat.html"` or `"root.html"`).
    pub page: String,
    /// HTML anchor `id` within that page (e.g. `"fn-Nat_add"`).
    pub anchor: String,
}

/// Maps every symbol name to the page + anchor where it is documented.
///
/// Built in a single pass over all [`ModuleGroup`]s so that the cross-reference
/// resolver can do O(1) lookups per `[Name]` reference.
#[derive(Debug, Default)]
pub struct SymbolIndex {
    inner: HashMap<String, SymbolLocation>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

impl SymbolIndex {
    /// Build a [`SymbolIndex`] from a slice of [`ModuleGroup`]s.
    ///
    /// The page name for each group is derived by replacing `"::"` with `"_"`
    /// and appending `".html"` (e.g. module `"Nat::Ops"` → `"Nat_Ops.html"`).
    /// The anchor for each item is derived from its name (same replacement).
    pub fn build(groups: &[ModuleGroup]) -> Self {
        let mut inner = HashMap::with_capacity(groups.iter().map(|g| g.items.len()).sum());
        for group in groups {
            let page = module_to_page(&group.module_name);
            for item in &group.items {
                let anchor = name_to_anchor(&item.name);
                inner.insert(
                    item.name.clone(),
                    SymbolLocation {
                        page: page.clone(),
                        anchor,
                    },
                );
            }
        }
        Self { inner }
    }

    /// Look up where `name` is documented.
    ///
    /// Returns `None` if the symbol is not in the index.
    pub fn lookup(&self, name: &str) -> Option<&SymbolLocation> {
        self.inner.get(name)
    }

    /// Serialise the index to a JSON array suitable for client-side search.
    ///
    /// Each entry has the shape:
    /// ```json
    /// {"name":"Foo","kind":"decl","page":"math.html","anchor":"sym-Foo","signature":""}
    /// ```
    ///
    /// The output is sorted lexicographically by entry text for deterministic builds.
    pub fn to_search_json(&self) -> String {
        let mut entries: Vec<String> = self
            .inner
            .iter()
            .map(|(name, loc)| {
                let escaped_name = escape_json_string(name);
                let escaped_page = escape_json_string(&loc.page);
                let escaped_anchor = escape_json_string(&loc.anchor);
                format!(
                    r#"{{"name":"{escaped_name}","kind":"decl","page":"{escaped_page}","anchor":"{escaped_anchor}","signature":""}}"#
                )
            })
            .collect();
        // Deterministic output regardless of HashMap iteration order.
        entries.sort();
        format!("[{}]", entries.join(","))
    }
}

// ---------------------------------------------------------------------------
// Private JSON helpers
// ---------------------------------------------------------------------------

/// Escape a string value for embedding in a JSON double-quoted string.
///
/// Only the characters that have special meaning inside JSON strings need
/// escaping: `"` → `\"`, `\` → `\\`, and the control characters `\n`, `\r`,
/// `\t` and the remaining C0 range.
fn escape_json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Helpers (also `pub(crate)` so the multi-file generator can reuse them)
// ---------------------------------------------------------------------------

/// Convert a module name like `"Nat::Ops"` to an HTML file name `"Nat_Ops.html"`.
pub(crate) fn module_to_page(module_name: &str) -> String {
    format!("{}.html", module_name.replace("::", "_"))
}

/// Convert a symbol name like `"Nat.add"` to an HTML anchor id `"sym-Nat_add"`.
pub(crate) fn name_to_anchor(name: &str) -> String {
    let sanitized = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();
    format!("sym-{sanitized}")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractor::{DeclKind, DocItem};
    use crate::walker::ModuleGroup;

    fn make_group(module: &str, names: &[&str]) -> ModuleGroup {
        ModuleGroup {
            module_name: module.to_string(),
            items: names
                .iter()
                .map(|&n| DocItem {
                    name: n.to_string(),
                    module: module.to_string(),
                    kind: DeclKind::Def,
                    signature: format!("def {n} := 0"),
                    doc_comment: None,
                    deprecated: false,
                })
                .collect(),
        }
    }

    #[test]
    fn test_build_and_lookup() {
        let groups = vec![make_group("Nat", &["Nat.add", "Nat.zero"])];
        let index = SymbolIndex::build(&groups);

        assert!(
            index.lookup("Nat.add").is_some(),
            "Nat.add should be indexed"
        );
        assert!(
            index.lookup("Nat.zero").is_some(),
            "Nat.zero should be indexed"
        );

        let loc = index.lookup("Nat.add").expect("Nat.add should be indexed");
        assert_eq!(loc.page, "Nat.html");
        assert!(loc.anchor.contains("Nat"), "anchor should encode the name");
    }

    #[test]
    fn test_lookup_missing_returns_none() {
        let groups: Vec<ModuleGroup> = vec![];
        let index = SymbolIndex::build(&groups);
        assert!(index.lookup("missing").is_none());
    }

    #[test]
    fn test_module_to_page_double_colon() {
        assert_eq!(module_to_page("Nat::Ops"), "Nat_Ops.html");
        assert_eq!(module_to_page("root"), "root.html");
    }

    #[test]
    fn test_name_to_anchor_dots() {
        let anchor = name_to_anchor("Nat.add");
        assert!(anchor.starts_with("sym-"), "anchor has prefix");
        assert!(!anchor.contains('.'), "dots replaced by underscores");
    }

    #[test]
    fn test_multi_module_index() {
        let groups = vec![
            make_group("Nat", &["Nat.add"]),
            make_group("List", &["List.map"]),
        ];
        let index = SymbolIndex::build(&groups);

        let nat_loc = index.lookup("Nat.add").expect("Nat.add");
        assert_eq!(nat_loc.page, "Nat.html");

        let list_loc = index.lookup("List.map").expect("List.map");
        assert_eq!(list_loc.page, "List.html");
    }

    #[test]
    fn test_is_empty() {
        let index = SymbolIndex::build(&[]);
        assert!(index.lookup("anything").is_none());
    }

    #[test]
    fn test_to_search_json_empty_index() {
        let index = SymbolIndex::build(&[]);
        let json = index.to_search_json();
        assert_eq!(json, "[]", "empty index should produce empty JSON array");
    }

    #[test]
    fn test_to_search_json_valid_array() {
        let groups = vec![make_group("Nat", &["Nat.add", "Nat.zero"])];
        let index = SymbolIndex::build(&groups);
        let json = index.to_search_json();
        assert!(json.starts_with('['), "JSON must start with [");
        assert!(json.ends_with(']'), "JSON must end with ]");
    }

    #[test]
    fn test_to_search_json_contains_name() {
        let groups = vec![make_group("Nat", &["Nat.add"])];
        let index = SymbolIndex::build(&groups);
        let json = index.to_search_json();
        assert!(
            json.contains("Nat.add"),
            "JSON should contain the symbol name"
        );
    }

    #[test]
    fn test_to_search_json_contains_page() {
        let groups = vec![make_group("Nat", &["Nat.add"])];
        let index = SymbolIndex::build(&groups);
        let json = index.to_search_json();
        assert!(
            json.contains("Nat.html"),
            "JSON should contain the page name"
        );
    }

    #[test]
    fn test_to_search_json_deterministic() {
        // Two calls on the same index must produce identical output.
        let groups = vec![
            make_group("Nat", &["Nat.add", "Nat.zero"]),
            make_group("List", &["List.map"]),
        ];
        let index = SymbolIndex::build(&groups);
        let json1 = index.to_search_json();
        let json2 = index.to_search_json();
        assert_eq!(json1, json2, "to_search_json must be deterministic");
    }

    #[test]
    fn test_to_search_json_escapes_quotes() {
        // Symbol names containing `"` must be escaped in the JSON output.
        let groups = vec![make_group(r#"Mod"Test"#, &[r#"sym"quote"#])];
        let index = SymbolIndex::build(&groups);
        let json = index.to_search_json();
        // The raw `"` must not appear unescaped between the property value quotes.
        // Valid JSON will have `\"` in the output.
        assert!(
            json.contains(r#"\""#),
            "quotes inside values must be JSON-escaped, got: {json}"
        );
    }
}
