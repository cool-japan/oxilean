//! # LlvmAttr - Trait Implementations
//!
//! This module contains trait implementations for `LlvmAttr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LlvmAttr;
use std::fmt;

impl fmt::Display for LlvmAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlvmAttr::NoUnwind => write!(f, "nounwind"),
            LlvmAttr::ReadOnly => write!(f, "readonly"),
            LlvmAttr::WriteOnly => write!(f, "writeonly"),
            LlvmAttr::NoReturn => write!(f, "noreturn"),
            LlvmAttr::NoAlias => write!(f, "noalias"),
            LlvmAttr::Align(n) => write!(f, "align {}", n),
            LlvmAttr::Dereferenceable(n) => write!(f, "dereferenceable({})", n),
            LlvmAttr::InlineHint => write!(f, "inlinehint"),
            LlvmAttr::AlwaysInline => write!(f, "alwaysinline"),
            LlvmAttr::NoInline => write!(f, "noinline"),
            LlvmAttr::Cold => write!(f, "cold"),
            LlvmAttr::OptSize => write!(f, "optsize"),
            LlvmAttr::UwTable => write!(f, "uwtable"),
            LlvmAttr::StackProtect => write!(f, "sspstrong"),
        }
    }
}
