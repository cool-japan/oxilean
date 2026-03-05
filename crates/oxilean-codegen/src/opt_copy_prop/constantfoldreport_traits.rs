//! # ConstantFoldReport - Trait Implementations
//!
//! This module contains trait implementations for `ConstantFoldReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ConstantFoldReport;
use std::fmt;

impl fmt::Display for ConstantFoldReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConstantFoldReport {{ folds={} }}", self.folds_performed)
    }
}
