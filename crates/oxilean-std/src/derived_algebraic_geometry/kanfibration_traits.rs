//! # KanFibration - Trait Implementations
//!
//! This module contains trait implementations for `KanFibration`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::KanFibration;
use std::fmt;

impl fmt::Display for KanFibration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KanFib: {} →→ {} (Kan={})",
            self.total_space, self.base_space, self.is_kan
        )
    }
}
