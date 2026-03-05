//! # RegistrySummary - Trait Implementations
//!
//! This module contains trait implementations for `RegistrySummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RegistrySummary;
use std::fmt;

impl fmt::Display for RegistrySummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ExtLemmaRegistry: {} lemmas, {} heads",
            self.total_lemmas, self.num_heads
        )
    }
}
