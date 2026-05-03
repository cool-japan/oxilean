//! Persistent (immutable) data structures — types.
//!
//! Functional, fully-persistent data structures with structural sharing.

use std::sync::Arc;

// ── PersistentVec: RRB-tree ───────────────────────────────────────────────────

/// Branching factor for RRB-tree nodes (32 children per internal node).
pub(super) const BRANCHING: usize = 32;
/// Log₂ of BRANCHING — bit-shift per tree level.
pub(super) const BRANCHING_BITS: usize = 5;

/// An RRB-tree persistent vector.
///
/// Provides O(log n) get, set, and push operations.  All operations
/// return a new `PersistentVec` and leave the original unchanged
/// (structural sharing via `Arc`).
#[derive(Clone, Debug)]
pub struct PersistentVec<T: Clone> {
    pub(super) root: Option<Arc<VecNode<T>>>,
    pub(super) len: usize,
    /// Bit-shift that selects the correct child at the root level.
    pub(super) shift: usize,
}

/// Internal node of the RRB-tree.
#[derive(Clone, Debug)]
pub enum VecNode<T: Clone> {
    /// An internal node holding up to BRANCHING child arcs.
    Internal {
        children: Vec<Arc<VecNode<T>>>,
        /// Total number of elements in this sub-tree.
        size: usize,
    },
    /// A leaf node holding up to BRANCHING values.
    Leaf { values: Vec<T> },
}

// ── PersistentMap: HAMT ───────────────────────────────────────────────────────

/// Hash Array Mapped Trie (HAMT) persistent map.
///
/// Provides O(log₃₂ n) insert, lookup, and remove with structural sharing.
#[derive(Clone, Debug)]
pub struct PersistentMap<K: Clone, V: Clone> {
    pub(super) root: Option<Arc<MapNode<K, V>>>,
    pub(super) len: usize,
}

/// Node in the HAMT.
#[derive(Clone, Debug)]
pub enum MapNode<K: Clone, V: Clone> {
    /// An empty sub-trie.
    Empty,
    /// A single key-value pair stored at the leaf level.
    Leaf {
        key: K,
        value: V,
        /// Pre-computed hash of `key`.
        hash: u64,
    },
    /// An internal node with a sparse children array indexed by a bitmap.
    Inner {
        /// 32-bit bitmap: bit i is set iff the i-th child is present.
        bitmap: u32,
        children: Vec<Arc<MapNode<K, V>>>,
    },
}

// ── PersistentSet ─────────────────────────────────────────────────────────────

/// A persistent set backed by a `PersistentMap<T, ()>`.
///
/// All operations are O(log n) with structural sharing.
#[derive(Clone, Debug)]
pub struct PersistentSet<T: Clone> {
    pub(super) map: PersistentMap<T, ()>,
}

// ── PersistentQueue: banker's queue ──────────────────────────────────────────

/// A purely-functional amortised O(1) queue (banker's queue).
///
/// Invariant: `back` is only non-empty when `front` is non-empty.
/// When `front` becomes empty, `back` is reversed into `front`.
#[derive(Clone, Debug)]
pub struct PersistentQueue<T: Clone> {
    /// Elements available at the front, in order.
    pub(super) front: Vec<T>,
    /// Elements waiting at the back, in *reverse* order.
    pub(super) back: Vec<T>,
}

// ── PersistentStack ───────────────────────────────────────────────────────────

/// A persistent linked-list stack.
///
/// Push and pop are O(1) with structural sharing.
#[derive(Clone, Debug)]
pub struct PersistentStack<T: Clone> {
    pub(super) head: Option<Arc<StackNode<T>>>,
    pub(super) len: usize,
}

/// A single node in the persistent stack.
#[derive(Clone, Debug)]
pub struct StackNode<T: Clone> {
    pub value: T,
    pub tail: Option<Arc<StackNode<T>>>,
}
