//! # CraneliftPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `CraneliftPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CraneliftPassConfig;

impl Default for CraneliftPassConfig {
    fn default() -> Self {
        CraneliftPassConfig {
            const_folding: true,
            dce: true,
            inst_combine: true,
            branch_opt: true,
            licm: false,
            reg_coalescing: true,
            load_elim: true,
            tail_call_opt: true,
            inline_depth: 3,
            debug_info: false,
        }
    }
}
