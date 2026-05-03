//! Persistent (immutable) data structures.
//!
//! Functional, fully-persistent data structures with structural sharing:
//! - `PersistentVec<T>` — RRB-tree persistent vector
//! - `PersistentMap<K, V>` — Hash Array Mapped Trie (HAMT) map
//! - `PersistentSet<T>` — set backed by `PersistentMap`
//! - `PersistentQueue<T>` — banker's queue (amortised O(1))
//! - `PersistentStack<T>` — linked-list stack (O(1) push/pop)

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
