//! # ReuseConfig - Trait Implementations
//!
//! This module contains trait implementations for `ReuseConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseConfig;

impl Default for ReuseConfig {
    fn default() -> Self {
        ReuseConfig {
            enable_reset_reuse: true,
            enable_borrow: true,
            enable_rc_elim: true,
            enable_in_place: true,
            analysis_depth: 10,
            interprocedural: false,
        }
    }
}
