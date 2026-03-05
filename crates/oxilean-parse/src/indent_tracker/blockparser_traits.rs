//! # BlockParser - Trait Implementations
//!
//! This module contains trait implementations for `BlockParser`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BlockParser;

impl Default for BlockParser {
    fn default() -> Self {
        Self::new(4)
    }
}
