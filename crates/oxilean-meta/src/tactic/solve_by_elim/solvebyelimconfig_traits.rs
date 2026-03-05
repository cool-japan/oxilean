//! # SolveByElimConfig - Trait Implementations
//!
//! This module contains trait implementations for `SolveByElimConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SolveByElimConfig;

impl Default for SolveByElimConfig {
    fn default() -> Self {
        Self {
            max_depth: 6,
            max_backtrack: 32,
            use_hyps: true,
            use_exfalso: true,
            all_goals: false,
            pre_apply: None,
            backtrack_all: false,
        }
    }
}
