//! The `convert` tactic: close a goal with an expression that may not match
//! the goal type exactly, generating new subgoals for each mismatch.
//!
//! Mirrors Lean 4's `convert` tactic.  When the provided expression matches
//! the goal exactly it acts like `exact`; otherwise each structural mismatch
//! becomes a new subgoal of the form `expected = provided`.

#![allow(dead_code)]
#![allow(missing_docs)]

pub mod functions;
pub mod types;

pub use functions::{find_mismatches, tac_convert, tac_convert_with_config};
pub use types::{ConvertConfig, ConvertResult};
