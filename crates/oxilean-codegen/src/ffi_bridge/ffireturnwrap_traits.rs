//! # FfiReturnWrap - Trait Implementations
//!
//! This module contains trait implementations for `FfiReturnWrap`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiReturnWrap;
use std::fmt;

impl std::fmt::Display for FfiReturnWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiReturnWrap::Direct(t) => write!(f, "direct({})", t),
            FfiReturnWrap::ErrorCode(t, e) => write!(f, "errcode({} -> {})", t, e),
            FfiReturnWrap::OutParam(p) => write!(f, "out({})", p),
            FfiReturnWrap::Bool(e) => write!(f, "bool({})", e),
        }
    }
}
