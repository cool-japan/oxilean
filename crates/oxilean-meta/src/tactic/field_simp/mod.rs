//! The `field_simp` tactic: clear denominators and normalize fractions.
//!
//! Mirrors Lean 4's `field_simp` which eliminates division from equality
//! goals by multiplying both sides by the relevant denominators, then
//! simplifying.

#![allow(dead_code)]
#![allow(missing_docs)]

pub mod functions;
pub mod types;

pub use functions::{
    clear_denominator, exprs_structurally_equal, field_simp_expr, find_division_patterns,
    normalize_fractions, tac_field_simp, tac_field_simp_with_config,
};
pub use types::{DivisionPattern, FieldSimpConfig, FieldSimpResult};
