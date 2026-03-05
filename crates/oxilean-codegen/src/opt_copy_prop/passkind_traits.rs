//! # PassKind - Trait Implementations
//!
//! This module contains trait implementations for `PassKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{CopyProp, PassKind};
use std::fmt;

impl fmt::Display for PassKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PassKind::CopyProp => write!(f, "CopyProp"),
            PassKind::DeadBinding => write!(f, "DeadBinding"),
            PassKind::ConstantFold => write!(f, "ConstantFold"),
            PassKind::Inlining => write!(f, "Inlining"),
        }
    }
}
