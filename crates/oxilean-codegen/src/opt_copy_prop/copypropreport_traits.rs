//! # CopyPropReport - Trait Implementations
//!
//! This module contains trait implementations for `CopyPropReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CopyPropReport;
use std::fmt;

impl fmt::Display for CopyPropReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CopyPropReport {{ copies_eliminated={}, chains_followed={} }}",
            self.copies_eliminated, self.chains_followed
        )
    }
}
