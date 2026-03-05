//! # NerveFunctor - Trait Implementations
//!
//! This module contains trait implementations for `NerveFunctor`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NerveFunctor;
use std::fmt;

impl fmt::Display for NerveFunctor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N({}): Cat → sSet", self.category_name)
    }
}
