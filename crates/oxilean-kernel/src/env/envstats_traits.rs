//! # EnvStats - Trait Implementations
//!
//! This module contains trait implementations for `EnvStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EnvStats;

impl std::fmt::Display for EnvStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "total={}, axioms={}, defs={}, thms={}, inductives={}, ctors={}, recs={}",
            self.total,
            self.axioms,
            self.definitions,
            self.theorems,
            self.inductives,
            self.constructors,
            self.recursors,
        )
    }
}
