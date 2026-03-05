//! # CoqExtConfig - Trait Implementations
//!
//! This module contains trait implementations for `CoqExtConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqExtConfig;

impl Default for CoqExtConfig {
    fn default() -> Self {
        Self {
            emit_comments: true,
            use_program: false,
            use_equations: false,
            universe_polymorphism: false,
            default_db: "core".to_string(),
            emit_extracted: false,
        }
    }
}
