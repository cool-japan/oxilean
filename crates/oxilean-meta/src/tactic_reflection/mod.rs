//! Tactic reflection — inspect and construct tactic expressions at meta-level.
//!
//! This module provides a reflective representation of tactics (`TacticRepr`)
//! and a simulated proof state (`ReflectionCtx`), enabling meta-programs to
//! inspect, construct, optimise, and simulate tactic proof scripts without
//! running the full elaborator.

pub mod functions;
pub mod types;

pub use functions::{
    apply_tactic, combine_tactics, goal_count, is_complete, optimize_script, script_from_string,
    sequence,
};
pub use types::{GoalRepr, ReflectionCtx, RewriteDir, TacticRepr, TacticScript};
