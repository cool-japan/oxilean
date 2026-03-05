//! # Config - Trait Implementations
//!
//! This module contains trait implementations for `Config`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Config;
use std::fmt;

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
