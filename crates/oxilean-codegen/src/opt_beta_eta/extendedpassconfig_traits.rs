//! # ExtendedPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `ExtendedPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExtendedPassConfig;

impl Default for ExtendedPassConfig {
    fn default() -> Self {
        ExtendedPassConfig {
            do_let_float: true,
            do_case_of_case: true,
            do_case_of_known_ctor: true,
            do_dead_let: true,
            max_case_of_case: 8,
            max_let_float_depth: 64,
        }
    }
}
