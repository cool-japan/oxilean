//! # SearchStats - Trait Implementations
//!
//! This module contains trait implementations for `SearchStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SearchStats;

impl std::fmt::Display for SearchStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "nodes={}, backtracks={}, max_depth={}, candidates={}, \
             applies(ok={}, fail={}), goals_closed={}",
            self.nodes_explored,
            self.backtracks,
            self.depth_reached,
            self.candidates_tried,
            self.successful_applies,
            self.failed_applies,
            self.goals_closed,
        )
    }
}
