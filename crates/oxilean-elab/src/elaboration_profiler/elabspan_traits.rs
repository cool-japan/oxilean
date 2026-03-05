//! # ElabSpan - Trait Implementations
//!
//! This module contains trait implementations for `ElabSpan`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabSpan;
use std::fmt;

impl std::fmt::Display for ElabSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.file {
            Some(file) => write!(f, "{}:[{}..{}]", file, self.start, self.end),
            None => write!(f, "[{}..{}]", self.start, self.end),
        }
    }
}
