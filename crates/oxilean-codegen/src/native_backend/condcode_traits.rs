//! # CondCode - Trait Implementations
//!
//! This module contains trait implementations for `CondCode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CondCode;
use std::fmt;

impl fmt::Display for CondCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CondCode::Eq => write!(f, "eq"),
            CondCode::Ne => write!(f, "ne"),
            CondCode::Lt => write!(f, "lt"),
            CondCode::Le => write!(f, "le"),
            CondCode::Gt => write!(f, "gt"),
            CondCode::Ge => write!(f, "ge"),
            CondCode::Ult => write!(f, "ult"),
            CondCode::Ule => write!(f, "ule"),
            CondCode::Ugt => write!(f, "ugt"),
            CondCode::Uge => write!(f, "uge"),
        }
    }
}
