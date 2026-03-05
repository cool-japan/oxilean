//! # WGSLAddressSpace - Trait Implementations
//!
//! This module contains trait implementations for `WGSLAddressSpace`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WGSLAddressSpace;
use std::fmt;

impl fmt::Display for WGSLAddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WGSLAddressSpace::Function => write!(f, "function"),
            WGSLAddressSpace::Private => write!(f, "private"),
            WGSLAddressSpace::Workgroup => write!(f, "workgroup"),
            WGSLAddressSpace::Uniform => write!(f, "uniform"),
            WGSLAddressSpace::Storage => write!(f, "storage"),
            WGSLAddressSpace::Handle => write!(f, "handle"),
        }
    }
}
