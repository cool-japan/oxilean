//! Formal Epistemology — belief revision, epistemic logic, Kripke semantics.
//!
//! Implements:
//! - AGM belief revision (expansion, contraction, revision)
//! - Kripke semantics for epistemic logic
//! - Common knowledge and distributed knowledge
//! - Muddy children puzzle
//! - Bayesian update, KL divergence, Shannon entropy

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
