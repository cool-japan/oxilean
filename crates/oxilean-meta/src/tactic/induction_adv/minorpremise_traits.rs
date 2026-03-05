//! # MinorPremise - Trait Implementations
//!
//! This module contains trait implementations for `MinorPremise`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MinorPremise;
use std::fmt;

impl fmt::Display for MinorPremise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MinorPremise({}, fields={}, recursive={})",
            self.ctor_name, self.num_fields, self.num_recursive_args,
        )
    }
}
