//! # SearchFailure - Trait Implementations
//!
//! This module contains trait implementations for `SearchFailure`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SearchFailure;

impl std::fmt::Display for SearchFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchFailure::DepthExceeded => write!(f, "depth limit exceeded"),
            SearchFailure::NoGoals => write!(f, "no goals"),
            SearchFailure::NoCandidates => write!(f, "no candidate lemmas"),
            SearchFailure::AllCandidatesExhausted => {
                write!(f, "all candidates exhausted")
            }
            SearchFailure::BacktrackLimitReached => write!(f, "backtrack limit reached"),
        }
    }
}
