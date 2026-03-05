//! # NativeEmitConfig - Trait Implementations
//!
//! This module contains trait implementations for `NativeEmitConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::NativeEmitConfig;

impl Default for NativeEmitConfig {
    fn default() -> Self {
        NativeEmitConfig {
            opt_level: 1,
            debug_info: false,
            target_arch: "x86_64".to_string(),
            num_gp_regs: 16,
            emit_comments: true,
        }
    }
}
