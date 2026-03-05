//! # RustVisibility - Trait Implementations
//!
//! This module contains trait implementations for `RustVisibility`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::RustVisibility;
use std::fmt;

impl fmt::Display for RustVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustVisibility::Private => Ok(()),
            RustVisibility::Pub => write!(f, "pub "),
            RustVisibility::PubCrate => write!(f, "pub(crate) "),
            RustVisibility::PubSuper => write!(f, "pub(super) "),
        }
    }
}
