//! # FfiParamAttr - Trait Implementations
//!
//! This module contains trait implementations for `FfiParamAttr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiParamAttr;
use std::fmt;

impl std::fmt::Display for FfiParamAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiParamAttr::In => write!(f, "_In_"),
            FfiParamAttr::Out => write!(f, "_Out_"),
            FfiParamAttr::InOut => write!(f, "_Inout_"),
            FfiParamAttr::Const => write!(f, "const"),
            FfiParamAttr::Volatile => write!(f, "volatile"),
            FfiParamAttr::Restrict => write!(f, "restrict"),
            FfiParamAttr::NullTerminated => write!(f, "_NullTerminated_"),
            FfiParamAttr::Nonnull => write!(f, "nonnull"),
            FfiParamAttr::Nullable => write!(f, "nullable"),
            FfiParamAttr::Retain => write!(f, "retain"),
            FfiParamAttr::Escaping => write!(f, "@escaping"),
        }
    }
}
