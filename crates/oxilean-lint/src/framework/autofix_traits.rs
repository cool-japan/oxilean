//! # AutoFix - Trait Implementations
//!
//! This module contains trait implementations for `AutoFix`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::types::AutoFix;

impl fmt::Display for AutoFix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} edit(s))", self.message, self.edits.len())
    }
}
