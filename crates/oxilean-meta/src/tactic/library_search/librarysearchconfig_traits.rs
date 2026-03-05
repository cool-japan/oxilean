//! # LibrarySearchConfig - Trait Implementations
//!
//! This module contains trait implementations for `LibrarySearchConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LibrarySearchConfig;

impl Default for LibrarySearchConfig {
    fn default() -> Self {
        Self {
            max_candidates: 256,
            max_depth: 4,
            timeout_ms: 5000,
            include_local: true,
            suggest_only: false,
            max_results: 10,
            allow_subgoals: false,
            max_remaining_goals: 3,
            max_synth_args: 8,
            use_discr_tree: true,
            min_score: 0.0,
            specificity_weight: 3.0,
            remaining_goals_weight: 2.0,
            edit_distance_weight: 1.0,
        }
    }
}
