//! # DiffResult - Trait Implementations
//!
//! This module contains trait implementations for `DiffResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_diff;
use super::types::{DiffConfig, DiffResult};
use std::fmt;

impl fmt::Display for DiffResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let config = DiffConfig::new().with_color(false);
        write!(f, "{}", format_diff(self, &config))
    }
}
