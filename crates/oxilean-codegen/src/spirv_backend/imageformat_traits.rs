//! # ImageFormat - Trait Implementations
//!
//! This module contains trait implementations for `ImageFormat`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ImageFormat;
use std::fmt;

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageFormat::Unknown => write!(f, "Unknown"),
            ImageFormat::Rgba32f => write!(f, "Rgba32f"),
            ImageFormat::Rgba16f => write!(f, "Rgba16f"),
            ImageFormat::R32f => write!(f, "R32f"),
            ImageFormat::Rgba8 => write!(f, "Rgba8"),
            ImageFormat::R32i => write!(f, "R32i"),
            ImageFormat::R32ui => write!(f, "R32ui"),
        }
    }
}
