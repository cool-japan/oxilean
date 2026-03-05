//! # ObjectTag - Trait Implementations
//!
//! This module contains trait implementations for `ObjectTag`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::ObjectTag;
use std::fmt;

impl fmt::Display for ObjectTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectTag::Scalar => write!(f, "scalar"),
            ObjectTag::Closure => write!(f, "closure"),
            ObjectTag::Array => write!(f, "array"),
            ObjectTag::Struct => write!(f, "struct"),
            ObjectTag::External => write!(f, "external"),
            ObjectTag::String => write!(f, "string"),
            ObjectTag::BigNat => write!(f, "bignat"),
            ObjectTag::Thunk => write!(f, "thunk"),
        }
    }
}
