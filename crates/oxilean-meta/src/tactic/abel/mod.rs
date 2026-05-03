//! The `abel` tactic: decide equalities in abelian groups.
//!
//! Normalizes both sides of a goal `a = b` as elements of a free abelian
//! group and closes the goal with `rfl` when the normal forms coincide.
//! Handles commutativity, associativity, negation, subtraction, and
//! scalar multiplication by integer literals.

#![allow(dead_code)]
#![allow(missing_docs)]

pub mod functions;
pub mod types;

pub use functions::{
    abel_forms_equal, abel_to_expr, expr_to_abel, normalize_abel_term, tac_abel,
    tac_abel_with_config,
};
pub use types::{AbelConfig, AbelNormalForm, AbelTerm};
