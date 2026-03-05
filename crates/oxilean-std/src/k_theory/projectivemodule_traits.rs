//! # ProjectiveModule - Trait Implementations
//!
//! This module contains trait implementations for `ProjectiveModule`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProjectiveModule;
use std::fmt;

impl std::fmt::Display for ProjectiveModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let free_str = if self.is_free { "free" } else { "projective" };
        write!(
            f,
            "{}^{} over {} ({})",
            self.ring, self.rank, self.ring, free_str
        )
    }
}
