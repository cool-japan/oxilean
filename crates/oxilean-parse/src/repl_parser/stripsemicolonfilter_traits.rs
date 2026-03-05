//! # StripSemicolonFilter - Trait Implementations
//!
//! This module contains trait implementations for `StripSemicolonFilter`.
//!
//! ## Implemented Traits
//!
//! - `InputFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::InputFilter;
use super::types::StripSemicolonFilter;

impl InputFilter for StripSemicolonFilter {
    fn filter(&self, input: &str) -> String {
        input.trim_end_matches(';').to_string()
    }
}
