//! # VectorWidth - Trait Implementations
//!
//! This module contains trait implementations for `VectorWidth`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VectorWidth;
use std::fmt;

impl fmt::Display for VectorWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VectorWidth::W64 => write!(f, "64-bit"),
            VectorWidth::W128 => write!(f, "128-bit (SSE)"),
            VectorWidth::W256 => write!(f, "256-bit (AVX)"),
            VectorWidth::W512 => write!(f, "512-bit (AVX-512)"),
        }
    }
}
