//! Proof quality lint module.
//!
//! Provides rules that analyse the structure, style and completeness of
//! OxiLean proof terms, including detection of `sorry` placeholders, deeply
//! nested proofs, repeated tactics, and unused hypotheses.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
