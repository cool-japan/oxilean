//! # TacCasesLogger - Trait Implementations
//!
//! This module contains trait implementations for `TacCasesLogger`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacCasesLogger;

impl Default for TacCasesLogger {
    fn default() -> Self {
        Self::new(1000)
    }
}
