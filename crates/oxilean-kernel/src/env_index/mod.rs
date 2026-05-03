//! Indexed environment for fast declaration lookup.
//!
//! Maintains bidirectional Name ↔ compact u32 id mapping ([`NameIndex`]) and
//! multi-dimensional secondary indices over declaration attributes
//! ([`TypeIndex`] for sort level / Pi arity; [`ModuleIndex`] for namespace
//! prefix).  The composite [`EnvIndex`] provides a single entry-point for all
//! indexed operations.
//!
//! # Design
//!
//! - **O(1) name→id and id→name** via dual `HashMap`/`Vec` in [`NameIndex`].
//! - **Namespace grouping** via [`ModuleIndex`] using the dot-prefix of each
//!   qualified name (`"Nat.add"` → namespace `"Nat"`).
//! - **Type attribute indexing** via [`TypeIndex`] keyed by sort level (`u8`)
//!   or Pi arity (`u32`).
//! - **Append-only**: ids are assigned monotonically and never recycled; the
//!   index never shrinks.
//!
//! # Example
//!
//! ```
//! use oxilean_kernel::env_index::{EnvIndex, is_in_namespace};
//!
//! let mut idx = EnvIndex::new();
//! let id = idx.insert("Nat.add");
//! let result = idx.lookup("Nat.add").unwrap();
//! assert_eq!(result.id, id);
//! assert_eq!(result.name, "Nat.add");
//!
//! assert!(is_in_namespace("Nat.add", "Nat"));
//! assert!(!is_in_namespace("List.map", "Nat"));
//! ```

pub mod functions;
pub mod types;

pub use functions::{is_in_namespace, namespace_of};
pub use types::{EnvIndex, IndexStats, LookupResult, ModuleIndex, NameIndex, TypeIndex};
