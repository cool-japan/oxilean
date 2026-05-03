//! Functions for the indexed environment layer.
//!
//! Implements construction, insertion, lookup, and statistics operations for
//! all index types declared in [`super::types`].

use std::collections::HashMap;

use super::types::{EnvIndex, IndexStats, LookupResult, ModuleIndex, NameIndex, TypeIndex};

// ---------------------------------------------------------------------------
// NameIndex
// ---------------------------------------------------------------------------

impl NameIndex {
    /// Create a new, empty `NameIndex`.
    pub fn new() -> Self {
        NameIndex {
            name_to_id: HashMap::new(),
            id_to_name: Vec::new(),
        }
    }

    /// Insert `name` and return its assigned id.
    ///
    /// If `name` was already inserted the existing id is returned unchanged;
    /// no duplicate entry is created.
    pub fn insert(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }
        let id = self.id_to_name.len() as u32;
        self.id_to_name.push(name.to_owned());
        self.name_to_id.insert(name.to_owned(), id);
        id
    }

    /// Look up the id of `name`, returning `None` if it has not been inserted.
    pub fn lookup_id(&self, name: &str) -> Option<u32> {
        self.name_to_id.get(name).copied()
    }

    /// Look up the name string for `id`, returning `None` if out of range.
    pub fn lookup_name(&self, id: u32) -> Option<&str> {
        self.id_to_name.get(id as usize).map(String::as_str)
    }

    /// Return the number of indexed names.
    pub fn len(&self) -> usize {
        self.id_to_name.len()
    }

    /// Return `true` if no names have been inserted.
    pub fn is_empty(&self) -> bool {
        self.id_to_name.is_empty()
    }
}

impl Default for NameIndex {
    fn default() -> Self {
        NameIndex::new()
    }
}

// ---------------------------------------------------------------------------
// ModuleIndex
// ---------------------------------------------------------------------------

impl ModuleIndex {
    /// Create a new, empty `ModuleIndex`.
    pub fn new() -> Self {
        ModuleIndex {
            modules: HashMap::new(),
        }
    }

    /// Record that declaration `id` belongs to the namespace of `name`.
    ///
    /// The namespace is derived from `name` via [`namespace_of`].
    pub fn add(&mut self, name: &str, id: u32) {
        let ns = namespace_of(name).to_owned();
        self.modules.entry(ns).or_default().push(id);
    }

    /// Return all declaration ids that belong to namespace `prefix`.
    ///
    /// Returns an empty `Vec` if no declarations with that prefix exist.
    pub fn lookup_namespace(&self, prefix: &str) -> Vec<u32> {
        self.modules.get(prefix).cloned().unwrap_or_default()
    }

    /// Return all namespace prefixes that have at least one declaration.
    pub fn all_namespaces(&self) -> Vec<String> {
        let mut ns: Vec<String> = self.modules.keys().cloned().collect();
        ns.sort();
        ns
    }
}

impl Default for ModuleIndex {
    fn default() -> Self {
        ModuleIndex::new()
    }
}

// ---------------------------------------------------------------------------
// TypeIndex
// ---------------------------------------------------------------------------

impl TypeIndex {
    /// Create a new, empty `TypeIndex`.
    pub fn new() -> Self {
        TypeIndex {
            by_sort: HashMap::new(),
            by_arity: HashMap::new(),
        }
    }

    /// Record that declaration `id` has outermost sort level `sort_level`.
    pub fn add_by_sort(&mut self, sort_level: u8, id: u32) {
        self.by_sort.entry(sort_level).or_default().push(id);
    }

    /// Record that declaration `id` has Pi arity `arity`.
    pub fn add_by_arity(&mut self, arity: u32, id: u32) {
        self.by_arity.entry(arity).or_default().push(id);
    }

    /// Return all declaration ids with outermost sort level `sort_level`.
    pub fn lookup_by_sort(&self, sort_level: u8) -> Vec<u32> {
        self.by_sort.get(&sort_level).cloned().unwrap_or_default()
    }

    /// Return all declaration ids with Pi arity `arity`.
    pub fn lookup_by_arity(&self, arity: u32) -> Vec<u32> {
        self.by_arity.get(&arity).cloned().unwrap_or_default()
    }
}

impl Default for TypeIndex {
    fn default() -> Self {
        TypeIndex::new()
    }
}

// ---------------------------------------------------------------------------
// EnvIndex
// ---------------------------------------------------------------------------

impl EnvIndex {
    /// Create a new, empty `EnvIndex`.
    pub fn new() -> Self {
        EnvIndex {
            name_idx: NameIndex::new(),
            type_idx: TypeIndex::new(),
            module_idx: ModuleIndex::new(),
            size: 0,
        }
    }

    /// Insert a declaration `name` into all sub-indices and return its id.
    ///
    /// The sort level is recorded as `0` and the arity as `0` for the base
    /// call-site (callers that have type information should use the individual
    /// sub-index methods directly after calling `insert`).
    ///
    /// If the name already exists, its existing id is returned and `size` is
    /// not incremented again.
    pub fn insert(&mut self, name: &str) -> u32 {
        let already = self.name_idx.lookup_id(name).is_some();
        let id = self.name_idx.insert(name);
        if !already {
            self.module_idx.add(name, id);
            self.size += 1;
        }
        id
    }

    /// Look up `name` and return a [`LookupResult`] on success.
    pub fn lookup<'a>(&'a self, name: &str) -> Option<LookupResult<'a>> {
        let id = self.name_idx.lookup_id(name)?;
        let stored_name = self.name_idx.lookup_name(id)?;
        Some(LookupResult {
            id,
            name: stored_name,
        })
    }

    /// Return all declaration ids whose namespace prefix equals `prefix`.
    pub fn by_namespace(&self, prefix: &str) -> Vec<u32> {
        self.module_idx.lookup_namespace(prefix)
    }

    /// Return an [`IndexStats`] snapshot for this index.
    ///
    /// `by_kind` is left empty; callers that track declaration kinds may
    /// populate it externally.
    pub fn stats(&self) -> IndexStats {
        IndexStats {
            total: self.size,
            by_kind: HashMap::new(),
            namespaces: self.module_idx.modules.len(),
        }
    }
}

impl Default for EnvIndex {
    fn default() -> Self {
        EnvIndex::new()
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Extract the namespace prefix from a dot-qualified name.
///
/// `"Nat.add"` → `"Nat"`, `"List.map"` → `"List"`, `"Nat"` → `""`.
///
/// The returned slice borrows from the input; no allocation occurs.
pub fn namespace_of(name: &str) -> &str {
    match name.rfind('.') {
        Some(pos) => &name[..pos],
        None => "",
    }
}

/// Return `true` if `name` belongs to namespace `ns`.
///
/// A name belongs to namespace `ns` when its namespace prefix (as computed by
/// [`namespace_of`]) is exactly `ns`.
///
/// ```
/// use oxilean_kernel::env_index::{is_in_namespace, namespace_of};
///
/// assert!(is_in_namespace("Nat.add", "Nat"));
/// assert!(!is_in_namespace("Nat.add", "List"));
/// assert!(is_in_namespace("succ", ""));
/// ```
pub fn is_in_namespace(name: &str, ns: &str) -> bool {
    namespace_of(name) == ns
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- namespace_of ---

    #[test]
    fn test_namespace_of_dotted() {
        assert_eq!(namespace_of("Nat.add"), "Nat");
    }

    #[test]
    fn test_namespace_of_nested() {
        assert_eq!(namespace_of("List.Lex.lt"), "List.Lex");
    }

    #[test]
    fn test_namespace_of_top_level() {
        assert_eq!(namespace_of("succ"), "");
    }

    #[test]
    fn test_namespace_of_empty() {
        assert_eq!(namespace_of(""), "");
    }

    // --- is_in_namespace ---

    #[test]
    fn test_is_in_namespace_true() {
        assert!(is_in_namespace("Nat.add", "Nat"));
    }

    #[test]
    fn test_is_in_namespace_false() {
        assert!(!is_in_namespace("Nat.add", "List"));
    }

    #[test]
    fn test_is_in_namespace_top_level() {
        assert!(is_in_namespace("succ", ""));
    }

    #[test]
    fn test_is_in_namespace_prefix_not_subset() {
        // "Na" is not the namespace of "Nat.add"
        assert!(!is_in_namespace("Nat.add", "Na"));
    }

    // --- NameIndex ---

    #[test]
    fn test_name_index_new_empty() {
        let idx = NameIndex::new();
        assert!(idx.is_empty());
        assert_eq!(idx.len(), 0);
    }

    #[test]
    fn test_name_index_insert_and_lookup_id() {
        let mut idx = NameIndex::new();
        let id = idx.insert("Nat.add");
        assert_eq!(idx.lookup_id("Nat.add"), Some(id));
    }

    #[test]
    fn test_name_index_insert_idempotent() {
        let mut idx = NameIndex::new();
        let id1 = idx.insert("Nat.add");
        let id2 = idx.insert("Nat.add");
        assert_eq!(id1, id2);
        assert_eq!(idx.len(), 1);
    }

    #[test]
    fn test_name_index_lookup_name() {
        let mut idx = NameIndex::new();
        let id = idx.insert("List.map");
        assert_eq!(idx.lookup_name(id), Some("List.map"));
    }

    #[test]
    fn test_name_index_lookup_id_missing() {
        let idx = NameIndex::new();
        assert_eq!(idx.lookup_id("unknown"), None);
    }

    #[test]
    fn test_name_index_lookup_name_out_of_range() {
        let idx = NameIndex::new();
        assert_eq!(idx.lookup_name(99), None);
    }

    #[test]
    fn test_name_index_multiple_entries() {
        let mut idx = NameIndex::new();
        let ids: Vec<u32> = ["a", "b", "c"].iter().map(|n| idx.insert(n)).collect();
        assert_eq!(ids, vec![0, 1, 2]);
        assert_eq!(idx.len(), 3);
    }

    // --- ModuleIndex ---

    #[test]
    fn test_module_index_new_empty() {
        let idx = ModuleIndex::new();
        assert!(idx.all_namespaces().is_empty());
    }

    #[test]
    fn test_module_index_add_and_lookup() {
        let mut idx = ModuleIndex::new();
        idx.add("Nat.add", 0);
        idx.add("Nat.sub", 1);
        let mut ids = idx.lookup_namespace("Nat");
        ids.sort();
        assert_eq!(ids, vec![0, 1]);
    }

    #[test]
    fn test_module_index_lookup_missing() {
        let idx = ModuleIndex::new();
        assert!(idx.lookup_namespace("List").is_empty());
    }

    #[test]
    fn test_module_index_all_namespaces_sorted() {
        let mut idx = ModuleIndex::new();
        idx.add("Nat.add", 0);
        idx.add("List.map", 1);
        idx.add("succ", 2);
        let ns = idx.all_namespaces();
        let mut expected = vec!["", "List", "Nat"];
        expected.sort();
        assert_eq!(ns, expected);
    }

    // --- TypeIndex ---

    #[test]
    fn test_type_index_by_sort() {
        let mut idx = TypeIndex::new();
        idx.add_by_sort(0, 10);
        idx.add_by_sort(0, 20);
        idx.add_by_sort(1, 30);
        let mut s0 = idx.lookup_by_sort(0);
        s0.sort();
        assert_eq!(s0, vec![10, 20]);
        assert_eq!(idx.lookup_by_sort(1), vec![30]);
    }

    #[test]
    fn test_type_index_by_arity() {
        let mut idx = TypeIndex::new();
        idx.add_by_arity(2, 5);
        idx.add_by_arity(2, 6);
        idx.add_by_arity(0, 7);
        let mut a2 = idx.lookup_by_arity(2);
        a2.sort();
        assert_eq!(a2, vec![5, 6]);
    }

    #[test]
    fn test_type_index_missing_sort() {
        let idx = TypeIndex::new();
        assert!(idx.lookup_by_sort(5).is_empty());
    }

    #[test]
    fn test_type_index_missing_arity() {
        let idx = TypeIndex::new();
        assert!(idx.lookup_by_arity(3).is_empty());
    }

    // --- EnvIndex ---

    #[test]
    fn test_env_index_new_empty() {
        let idx = EnvIndex::new();
        assert_eq!(idx.size, 0);
    }

    #[test]
    fn test_env_index_insert_and_lookup() {
        let mut idx = EnvIndex::new();
        let id = idx.insert("Nat.add");
        let result = idx.lookup("Nat.add").expect("should find inserted name");
        assert_eq!(result.id, id);
        assert_eq!(result.name, "Nat.add");
    }

    #[test]
    fn test_env_index_insert_idempotent() {
        let mut idx = EnvIndex::new();
        idx.insert("Nat.add");
        idx.insert("Nat.add");
        assert_eq!(idx.size, 1);
    }

    #[test]
    fn test_env_index_lookup_missing() {
        let idx = EnvIndex::new();
        assert!(idx.lookup("ghost").is_none());
    }

    #[test]
    fn test_env_index_by_namespace() {
        let mut idx = EnvIndex::new();
        idx.insert("Nat.add");
        idx.insert("Nat.sub");
        idx.insert("List.map");
        let mut nat = idx.by_namespace("Nat");
        nat.sort();
        assert_eq!(nat.len(), 2);
        assert!(idx.by_namespace("List").len() == 1);
    }

    #[test]
    fn test_env_index_stats() {
        let mut idx = EnvIndex::new();
        idx.insert("Nat.add");
        idx.insert("List.map");
        idx.insert("succ");
        let s = idx.stats();
        assert_eq!(s.total, 3);
        assert_eq!(s.namespaces, 3); // "Nat", "List", ""
    }

    #[test]
    fn test_env_index_stats_display() {
        let idx = EnvIndex::new();
        let s = idx.stats();
        let text = format!("{}", s);
        assert!(text.contains("IndexStats"));
    }

    #[test]
    fn test_env_index_type_idx_sort() {
        let mut idx = EnvIndex::new();
        let id = idx.insert("Prop");
        idx.type_idx.add_by_sort(0, id);
        let results = idx.type_idx.lookup_by_sort(0);
        assert_eq!(results, vec![id]);
    }

    #[test]
    fn test_env_index_type_idx_arity() {
        let mut idx = EnvIndex::new();
        let id = idx.insert("Nat.add");
        idx.type_idx.add_by_arity(2, id);
        let results = idx.type_idx.lookup_by_arity(2);
        assert_eq!(results, vec![id]);
    }
}
