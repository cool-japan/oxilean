//! # MetalAddressSpace - Trait Implementations
//!
//! This module contains trait implementations for `MetalAddressSpace`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetalAddressSpace;
use std::fmt;

impl fmt::Display for MetalAddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetalAddressSpace::Device => write!(f, "device"),
            MetalAddressSpace::Constant => write!(f, "constant"),
            MetalAddressSpace::Threadgroup => write!(f, "threadgroup"),
            MetalAddressSpace::ThreadgroupImageblock => {
                write!(f, "threadgroup_imageblock")
            }
            MetalAddressSpace::RayData => write!(f, "ray_data"),
            MetalAddressSpace::ObjectData => write!(f, "object_data"),
            MetalAddressSpace::Thread => write!(f, "thread"),
        }
    }
}
