//! # FfiSizeHint - Trait Implementations
//!
//! This module contains trait implementations for `FfiSizeHint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiSizeHint;
use std::fmt;

impl std::fmt::Display for FfiSizeHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiSizeHint::Fixed(n) => write!(f, "size={}", n),
            FfiSizeHint::Param(p) => write!(f, "size=param({})", p),
            FfiSizeHint::Dynamic => write!(f, "size=dynamic"),
        }
    }
}
