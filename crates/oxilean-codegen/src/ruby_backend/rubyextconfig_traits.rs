//! # RubyExtConfig - Trait Implementations
//!
//! This module contains trait implementations for `RubyExtConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::RubyExtConfig;

impl Default for RubyExtConfig {
    fn default() -> Self {
        Self {
            ruby_version: "3.3".to_string(),
            use_sorbet: false,
            use_rbs: false,
            frozen_string_literals: true,
            encoding: "UTF-8".to_string(),
            indent_size: 2,
            use_keyword_args: true,
        }
    }
}
