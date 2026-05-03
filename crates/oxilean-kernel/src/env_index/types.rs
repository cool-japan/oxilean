//! Types for the indexed environment layer.
//!
//! Provides bidirectional Name ↔ compact u32 id mapping and multi-dimensional
//! indices over declaration attributes (sort level, Pi arity, namespace prefix)
//! for sub-linear declaration lookup in large kernel environments.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// NameIndex
// ---------------------------------------------------------------------------

/// Bidirectional mapping between declaration names and compact `u32` identifiers.
///
/// Assigning a dense integer id to each name enables O(1) lookup by id and
/// compact storage in downstream indices.
#[derive(Clone, Debug)]
pub struct NameIndex {
    /// Maps the string form of a name to its assigned id.
    pub(super) name_to_id: HashMap<String, u32>,
    /// Maps an id back to the string form of the name.
    pub(super) id_to_name: Vec<String>,
}

// ---------------------------------------------------------------------------
// TypeIndex
// ---------------------------------------------------------------------------

/// Index over declaration type attributes: sort level and Pi-type arity.
///
/// Enables efficient retrieval of all declarations with a given sort level or
/// a given number of explicit arguments.
#[derive(Clone, Debug)]
pub struct TypeIndex {
    /// Maps a sort level (universe level byte) to the ids of declarations
    /// whose outermost type lives at that sort.
    pub(super) by_sort: HashMap<u8, Vec<u32>>,
    /// Maps a Pi arity (number of binders before the return type) to the ids
    /// of declarations with that arity.
    pub(super) by_arity: HashMap<u32, Vec<u32>>,
}

// ---------------------------------------------------------------------------
// ModuleIndex
// ---------------------------------------------------------------------------

/// Groups declarations by their namespace prefix.
///
/// The namespace is the dot-separated prefix of a name: the namespace of
/// `"Nat.add"` is `"Nat"`, and top-level names (no dot) live in the `""`
/// (empty) namespace.
#[derive(Clone, Debug)]
pub struct ModuleIndex {
    /// Maps a namespace prefix to the ids of all declarations in that namespace.
    pub(super) modules: HashMap<String, Vec<u32>>,
}

// ---------------------------------------------------------------------------
// EnvIndex
// ---------------------------------------------------------------------------

/// Composite index combining name, type, and module indices.
///
/// Provides a single entry point for all indexed lookups over a kernel
/// `Environment`.  The index is append-only: names are inserted as
/// declarations are added, and ids are never recycled.
#[derive(Clone, Debug)]
pub struct EnvIndex {
    /// Bidirectional name ↔ id mapping.
    pub name_idx: NameIndex,
    /// Index over sort level and Pi arity.
    pub type_idx: TypeIndex,
    /// Index grouping declarations by namespace.
    pub module_idx: ModuleIndex,
    /// Total number of indexed declarations (equals `name_idx.len()`).
    pub size: usize,
}

// ---------------------------------------------------------------------------
// IndexStats
// ---------------------------------------------------------------------------

/// A snapshot of index statistics.
#[derive(Clone, Debug)]
pub struct IndexStats {
    /// Total number of indexed declarations.
    pub total: usize,
    /// Declaration counts grouped by a logical kind tag (currently unused by
    /// the index itself; callers populate this from external data).
    pub by_kind: HashMap<String, usize>,
    /// Number of distinct namespace prefixes.
    pub namespaces: usize,
}

impl std::fmt::Display for IndexStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IndexStats {{ total: {}, namespaces: {} }}",
            self.total, self.namespaces
        )
    }
}

// ---------------------------------------------------------------------------
// LookupResult
// ---------------------------------------------------------------------------

/// The result of a successful indexed lookup.
///
/// Borrows the name string from the owning `NameIndex` to avoid allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LookupResult<'a> {
    /// The compact identifier assigned at insertion time.
    pub id: u32,
    /// The string form of the matched name.
    pub name: &'a str,
}
