//! # FutharkAttr - Trait Implementations
//!
//! This module contains trait implementations for `FutharkAttr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkAttr;
use std::fmt;

impl fmt::Display for FutharkAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FutharkAttr::Inline => write!(f, "#[inline]"),
            FutharkAttr::NoInline => write!(f, "#[noinline]"),
            FutharkAttr::NoMap => write!(f, "#[nomap]"),
            FutharkAttr::Sequential => write!(f, "#[sequential]"),
            FutharkAttr::Custom(s) => write!(f, "#[{s}]"),
        }
    }
}
