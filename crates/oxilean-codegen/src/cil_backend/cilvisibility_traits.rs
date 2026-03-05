//! # CilVisibility - Trait Implementations
//!
//! This module contains trait implementations for `CilVisibility`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilVisibility;
use std::fmt;

impl fmt::Display for CilVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CilVisibility::Private => write!(f, "private"),
            CilVisibility::Assembly => write!(f, "assembly"),
            CilVisibility::Family => write!(f, "family"),
            CilVisibility::Public => write!(f, "public"),
        }
    }
}
