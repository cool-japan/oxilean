//! # MemcpyKind - Trait Implementations
//!
//! This module contains trait implementations for `MemcpyKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemcpyKind;
use std::fmt;

impl fmt::Display for MemcpyKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemcpyKind::HostToDevice => write!(f, "cudaMemcpyHostToDevice"),
            MemcpyKind::DeviceToHost => write!(f, "cudaMemcpyDeviceToHost"),
            MemcpyKind::DeviceToDevice => write!(f, "cudaMemcpyDeviceToDevice"),
            MemcpyKind::HostToHost => write!(f, "cudaMemcpyHostToHost"),
        }
    }
}
