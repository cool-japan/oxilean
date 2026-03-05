//! # PatternStats - Trait Implementations
//!
//! This module contains trait implementations for `PatternStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PatternStats;

impl std::fmt::Display for PatternStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PatternStats {{ total: {}, wildcards: {}, constructors: {}, literals: {}, or: {} }}",
            self.total_patterns, self.wildcards, self.constructors, self.literals, self.or_patterns
        )
    }
}
