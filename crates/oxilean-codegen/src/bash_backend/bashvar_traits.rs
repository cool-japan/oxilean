//! # BashVar - Trait Implementations
//!
//! This module contains trait implementations for `BashVar`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BashVar;
use std::fmt;

impl fmt::Display for BashVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${{{}}}", self.name())
    }
}
