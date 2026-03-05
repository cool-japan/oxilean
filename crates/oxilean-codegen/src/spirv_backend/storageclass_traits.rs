//! # StorageClass - Trait Implementations
//!
//! This module contains trait implementations for `StorageClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StorageClass;
use std::fmt;

impl fmt::Display for StorageClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageClass::Uniform => write!(f, "Uniform"),
            StorageClass::StorageBuffer => write!(f, "StorageBuffer"),
            StorageClass::PushConstant => write!(f, "PushConstant"),
            StorageClass::Input => write!(f, "Input"),
            StorageClass::Output => write!(f, "Output"),
            StorageClass::Function => write!(f, "Function"),
            StorageClass::Private => write!(f, "Private"),
            StorageClass::Workgroup => write!(f, "Workgroup"),
            StorageClass::CrossWorkgroup => write!(f, "CrossWorkgroup"),
            StorageClass::Image => write!(f, "Image"),
            StorageClass::Generic => write!(f, "Generic"),
        }
    }
}
