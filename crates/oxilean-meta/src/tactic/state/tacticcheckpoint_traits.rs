//! # TacticCheckpoint - Trait Implementations
//!
//! This module contains trait implementations for `TacticCheckpoint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacticCheckpoint;

impl std::fmt::Display for TacticCheckpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Checkpoint[{}]({} goals)", self.name, self.num_goals())
    }
}
