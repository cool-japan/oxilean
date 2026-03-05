//! # ArenaStats - Trait Implementations
//!
//! This module contains trait implementations for `ArenaStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Arena, ArenaStats};

impl std::fmt::Display for ArenaStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Arena {{ len: {}, capacity: {}, utilisation: {:.1}% }}",
            self.len,
            self.capacity,
            self.utilisation() * 100.0
        )
    }
}
