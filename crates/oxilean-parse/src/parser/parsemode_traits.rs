//! # ParseMode - Trait Implementations
//!
//! This module contains trait implementations for `ParseMode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseMode;

impl std::fmt::Display for ParseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParseMode(tactics={}, recover={}, lenient={})",
            self.allow_tactics, self.recover_on_error, self.lenient
        )
    }
}
