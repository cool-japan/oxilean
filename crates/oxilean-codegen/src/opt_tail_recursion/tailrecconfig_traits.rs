//! # TailRecConfig - Trait Implementations
//!
//! This module contains trait implementations for `TailRecConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TailRecConfig;

impl Default for TailRecConfig {
    fn default() -> Self {
        TailRecConfig {
            transform_linear: true,
            introduce_accum: true,
        }
    }
}
