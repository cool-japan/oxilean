//! # PrintOptionsExt - Trait Implementations
//!
//! This module contains trait implementations for `PrintOptionsExt`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PrintOptionsExt;
use std::fmt;

impl Default for PrintOptionsExt {
    fn default() -> Self {
        Self {
            width: 80,
            compact: false,
            indent: "  ".to_owned(),
        }
    }
}
