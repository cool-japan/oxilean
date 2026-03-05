//! # GVNPipeline - Trait Implementations
//!
//! This module contains trait implementations for `GVNPipeline`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{GVNPipeline, GVNStatistics};

impl Default for GVNPipeline {
    fn default() -> Self {
        GVNPipeline {
            do_base_gvn: true,
            do_load_elim: true,
            do_pre: false,
            do_ccp: false,
            do_fixpoint: false,
            max_fixpoint_iter: 5,
            stats: GVNStatistics::new(),
        }
    }
}
