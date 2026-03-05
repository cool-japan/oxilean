//! # JoinPointConfig - Trait Implementations
//!
//! This module contains trait implementations for `JoinPointConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JoinPointConfig;

impl Default for JoinPointConfig {
    fn default() -> Self {
        JoinPointConfig {
            max_join_size: 10,
            inline_small_joins: true,
            detect_tail_calls: true,
            enable_contification: true,
            float_join_points: true,
            eliminate_dead_joins: true,
            max_iterations: 4,
        }
    }
}
