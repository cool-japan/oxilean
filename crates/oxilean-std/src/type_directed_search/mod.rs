//! Hoogle-like type-directed function search.
//!
//! Given a type signature query, ranks functions in a `SearchDB` by how well
//! they match: exact, up-to-renaming, specialization, generalization, or partial.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
