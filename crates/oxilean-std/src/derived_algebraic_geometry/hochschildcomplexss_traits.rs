//! # HochschildComplexSS - Trait Implementations
//!
//! This module contains trait implementations for `HochschildComplexSS`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HochschildComplexSS;
use std::fmt;

impl fmt::Display for HochschildComplexSS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HS-SS(1→{}→{}→{}→1; coeff={})",
            self.normal_subgroup, self.group, self.quotient, self.coefficients
        )
    }
}
