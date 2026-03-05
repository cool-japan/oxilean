//! # AutoLemmaScorer - Trait Implementations
//!
//! This module contains trait implementations for `AutoLemmaScorer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AutoLemmaScorer;
use std::fmt;

impl Default for AutoLemmaScorer {
    fn default() -> Self {
        AutoLemmaScorer::new()
    }
}
