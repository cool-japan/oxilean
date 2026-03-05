//! # Info - Trait Implementations
//!
//! This module contains trait implementations for `Info`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Info;
use std::fmt;

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}..{}] {}",
            self.stx_range.0, self.stx_range.1, self.data
        )
    }
}
