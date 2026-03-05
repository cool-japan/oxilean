//! # SourceMapEntry - Trait Implementations
//!
//! This module contains trait implementations for `SourceMapEntry`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::SourceMapEntry;
use std::fmt;

impl fmt::Display for SourceMapEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} -> {}:{}",
            self.gen_line, self.gen_col, self.source_fn, self.source_line
        )
    }
}
