//! # LowercaseCommandFilter - Trait Implementations
//!
//! This module contains trait implementations for `LowercaseCommandFilter`.
//!
//! ## Implemented Traits
//!
//! - `InputFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::InputFilter;
use super::types::LowercaseCommandFilter;

impl InputFilter for LowercaseCommandFilter {
    fn filter(&self, input: &str) -> String {
        let trimmed = input.trim_start();
        if let Some(rest) = trimmed.strip_prefix(':') {
            let (cmd, tail) = rest
                .find(char::is_whitespace)
                .map(|i| (&rest[..i], &rest[i..]))
                .unwrap_or((rest, ""));
            format!(":{}{}", cmd.to_lowercase(), tail)
        } else {
            input.to_string()
        }
    }
}
