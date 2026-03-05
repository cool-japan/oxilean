//! # LICMPass - Trait Implementations
//!
//! This module contains trait implementations for `LICMPass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{LICMConfig, LICMPass};

impl Default for LICMPass {
    fn default() -> Self {
        Self::new(LICMConfig::default())
    }
}
