//! # FnPtr - Trait Implementations
//!
//! This module contains trait implementations for `FnPtr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FnPtr;
use std::fmt;

impl fmt::Display for FnPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.module_id == 0 {
            write!(f, "fn#{}", self.index)
        } else {
            write!(f, "fn#{}:{}", self.module_id, self.index)
        }
    }
}
