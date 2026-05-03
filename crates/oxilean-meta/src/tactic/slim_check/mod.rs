//! The `slim_check` tactic: random counterexample search.
//!
//! Mirrors Lean 4's `slim_check`.  On a positive result (no counterexample
//! found) the goal is left open, as if by `sorry`.  On a negative result
//! (counterexample found) an error is returned with the witness.

#![allow(dead_code)]
#![allow(missing_docs)]

pub mod functions;
pub mod types;

pub use functions::{
    extract_forall_vars, gen_bool, gen_int, gen_nat, lcg_rand, tac_slim_check,
    tac_slim_check_with_config, try_find_counterexample, ForallVar,
};
pub use types::{Counterexample, SlimCheckConfig, SlimCheckOutcome, SlimCheckResult};
