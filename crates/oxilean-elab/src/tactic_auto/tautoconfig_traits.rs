//! # TautoConfig - Trait Implementations
//!
//! This module contains trait implementations for `TautoConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TautoConfig;
use std::fmt;

impl Default for TautoConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            use_disj_syllogism: true,
            use_hypo_syllogism: true,
            use_modus_ponens: true,
        }
    }
}
