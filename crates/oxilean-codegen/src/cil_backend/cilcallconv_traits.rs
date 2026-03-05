//! # CilCallConv - Trait Implementations
//!
//! This module contains trait implementations for `CilCallConv`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilCallConv;
use std::fmt;

impl fmt::Display for CilCallConv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CilCallConv::Default => Ok(()),
            CilCallConv::Instance => write!(f, "instance"),
            CilCallConv::Generic(n) => write!(f, "instance generic({})", n),
        }
    }
}
