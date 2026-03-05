//! # ExtLemma - Trait Implementations
//!
//! This module contains trait implementations for `ExtLemma`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExtLemma;
use std::fmt;

impl fmt::Display for ExtLemma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ExtLemma({}, priority={}, params={})",
            self.name, self.priority, self.num_params
        )
    }
}
