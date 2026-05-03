//! Enhanced `decide` tactic.
//!
//! Extends the baseline `decide` tactic with support for:
//! - Natural-number arithmetic evaluation (`NatArith`),
//! - Boolean expression evaluation (`BoolEval`),
//! - DPLL-based propositional SAT solving (`PropFormulaDpll`),
//! - Finite-enumeration checking (`FiniteEnum`).

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
