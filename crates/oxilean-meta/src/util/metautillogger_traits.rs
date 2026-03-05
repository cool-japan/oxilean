//! # MetaUtilLogger - Trait Implementations
//!
//! This module contains trait implementations for `MetaUtilLogger`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaUtilLogger;

impl Default for MetaUtilLogger {
    fn default() -> Self {
        Self::new(1000)
    }
}
