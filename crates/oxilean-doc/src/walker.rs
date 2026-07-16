//! Module grouping for multi-file documentation generation.
//!
//! Groups a flat list of [`DocItem`]s by their `module` field,
//! producing one [`ModuleGroup`] per distinct module name.

use crate::extractor::DocItem;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A collection of [`DocItem`]s that share the same module name.
#[derive(Debug, Clone)]
pub struct ModuleGroup {
    /// The module/namespace name (e.g. `"Nat"`, `"root"`).
    pub module_name: String,
    /// All declarations belonging to this module.
    pub items: Vec<DocItem>,
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Group a slice of [`DocItem`]s by their `module` field.
///
/// The returned vector is sorted alphabetically by module name for
/// deterministic output.  Items that share the same `module` are kept
/// in their original relative order.
///
/// # Examples
///
/// ```ignore
/// let groups = group_by_module(&items);
/// for g in &groups {
///     println!("Module {} has {} item(s)", g.module_name, g.items.len());
/// }
/// ```
pub fn group_by_module(items: &[DocItem]) -> Vec<ModuleGroup> {
    // Preserve insertion order across module names while grouping.
    // We use an index list instead of a HashMap so the result is
    // deterministic without extra sorting overhead on the value side.
    let mut module_order: Vec<String> = Vec::new();
    let mut module_items: std::collections::HashMap<String, Vec<DocItem>> =
        std::collections::HashMap::new();

    for item in items {
        let key = item.module.clone();
        if !module_items.contains_key(&key) {
            module_order.push(key.clone());
        }
        module_items.entry(key).or_default().push(item.clone());
    }

    // Sort module names for reproducible, hosting-portable output.
    module_order.sort();

    module_order
        .into_iter()
        .filter_map(|name| {
            module_items.remove(&name).map(|items| ModuleGroup {
                module_name: name,
                items,
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractor::{DeclKind, DocItem};

    fn item(name: &str, module: &str) -> DocItem {
        DocItem {
            name: name.to_string(),
            module: module.to_string(),
            kind: DeclKind::Def,
            signature: format!("def {name} := 0"),
            doc_comment: None,
            deprecated: false,
        }
    }

    #[test]
    fn test_group_by_module_basic() {
        let items = vec![
            item("Nat.add", "Nat"),
            item("Nat.zero", "Nat"),
            item("List.map", "List"),
        ];
        let groups = group_by_module(&items);
        assert_eq!(groups.len(), 2, "should have 2 module groups");

        // Sorted alphabetically: List < Nat
        assert_eq!(groups[0].module_name, "List");
        assert_eq!(groups[0].items.len(), 1);
        assert_eq!(groups[1].module_name, "Nat");
        assert_eq!(groups[1].items.len(), 2);
    }

    #[test]
    fn test_group_by_module_empty() {
        let groups = group_by_module(&[]);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_group_by_module_single_module() {
        let items = vec![item("foo", "root"), item("bar", "root")];
        let groups = group_by_module(&items);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].module_name, "root");
        assert_eq!(groups[0].items.len(), 2);
    }

    #[test]
    fn test_group_by_module_sorted_names() {
        let items = vec![
            item("z_decl", "ZMod"),
            item("a_decl", "Array"),
            item("b_decl", "Bool"),
        ];
        let groups = group_by_module(&items);
        let names: Vec<&str> = groups.iter().map(|g| g.module_name.as_str()).collect();
        assert_eq!(names, vec!["Array", "Bool", "ZMod"]);
    }
}
