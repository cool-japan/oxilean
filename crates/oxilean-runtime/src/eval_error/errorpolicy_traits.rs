//! # ErrorPolicy - Trait Implementations
//!
//! This module contains trait implementations for `ErrorPolicy`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ErrorPolicy;

impl Default for ErrorPolicy {
    fn default() -> Self {
        ErrorPolicy::strict()
    }
}
