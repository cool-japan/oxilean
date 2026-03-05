//! # ParseErrorCollector - Trait Implementations
//!
//! This module contains trait implementations for `ParseErrorCollector`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseErrorCollector;
use std::fmt;

impl fmt::Display for ParseErrorCollector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseErrorCollector({} errors)", self.errors().len())
    }
}
