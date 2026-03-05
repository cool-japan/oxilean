//! # CseReport - Trait Implementations
//!
//! This module contains trait implementations for `CseReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CseReport;
use std::fmt;

impl fmt::Display for CseReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CseReport {{ found={}, eliminated={}, hoisted={} }}",
            self.expressions_found, self.expressions_eliminated, self.lets_hoisted
        )
    }
}
