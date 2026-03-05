//! # EvalQuota - Trait Implementations
//!
//! This module contains trait implementations for `EvalQuota`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvalQuota;

impl Default for EvalQuota {
    fn default() -> Self {
        Self::unlimited()
    }
}
