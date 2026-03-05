//! # MemFlags - Trait Implementations
//!
//! This module contains trait implementations for `MemFlags`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemFlags;
use std::fmt;

impl fmt::Display for MemFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemFlags::None => write!(f, "mem_flags::mem_none"),
            MemFlags::Device => write!(f, "mem_flags::mem_device"),
            MemFlags::Threadgroup => write!(f, "mem_flags::mem_threadgroup"),
            MemFlags::Texture => write!(f, "mem_flags::mem_texture"),
        }
    }
}
