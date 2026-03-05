//! # ColorMode - Trait Implementations
//!
//! This module contains trait implementations for `ColorMode`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ColorMode;
use std::fmt;

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Auto
    }
}
