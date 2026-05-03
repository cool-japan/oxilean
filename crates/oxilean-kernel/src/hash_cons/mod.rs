//! Structural sharing (hash-consing) for `Expr` values.
//!
//! A plain `Arena<Expr>` lets duplicate nodes coexist:
//!
//! ```text
//! arena.alloc(Expr::BVar(0))  → Idx 0
//! arena.alloc(Expr::BVar(0))  → Idx 1   // duplicate!
//! ```
//!
//! `HashConsArena` wraps `Arena<Expr>` with a deduplication cache so that
//! structurally equal expressions always share the same `Idx<Expr>`.
//!
//! # Usage
//!
//! ```
//! use oxilean_kernel::hash_cons::HashConsArena;
//! use oxilean_kernel::{Level, Name, BinderInfo};
//!
//! let mut hc = HashConsArena::new();
//! let s1 = hc.mk_sort(Level::Zero);
//! let s2 = hc.mk_sort(Level::Zero);
//! assert_eq!(s1, s2, "identical sorts share one Idx");
//! let stats = hc.stats();
//! assert_eq!(stats.hits, 1);
//! ```

pub mod arena;
pub mod key;
pub mod stats;

pub use arena::HashConsArena;
pub use key::ExprKey;
pub use stats::HashConsStats;
