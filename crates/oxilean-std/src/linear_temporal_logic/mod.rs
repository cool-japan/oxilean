//! Linear Temporal Logic (LTL) — model checking over Kripke structures.
//!
//! Provides:
//! - Full LTL formula type with all classical operators (X, U, R, G, F, W).
//! - Lasso-shaped infinite trace semantics.
//! - Negation Normal Form (NNF) conversion.
//! - Simple LTL parser and pretty-printer.
//! - Explicit-state model checking over finite Kripke structures.
//! - A catalogue of known LTL equivalences.

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
