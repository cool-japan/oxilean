//! # QuoteBuilder - Trait Implementations
//!
//! This module contains trait implementations for `QuoteBuilder`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::QuoteBuilder;
use std::fmt;

impl Default for QuoteBuilder {
    fn default() -> Self {
        Self::new()
    }
}
