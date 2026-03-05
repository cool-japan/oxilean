//! # MetaDbgLogger - Trait Implementations
//!
//! This module contains trait implementations for `MetaDbgLogger`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaDbgLogger;

impl Default for MetaDbgLogger {
    fn default() -> Self {
        Self::new(1000)
    }
}
