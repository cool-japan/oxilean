//! # CanonicalOptions - Trait Implementations
//!
//! This module contains trait implementations for `CanonicalOptions`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{CanonicalOptions, StringEncoding};

impl Default for CanonicalOptions {
    fn default() -> Self {
        Self {
            memory: None,
            realloc: None,
            post_return: None,
            string_encoding: StringEncoding::Utf8,
        }
    }
}
