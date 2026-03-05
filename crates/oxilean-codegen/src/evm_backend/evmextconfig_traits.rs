//! # EvmExtConfig - Trait Implementations
//!
//! This module contains trait implementations for `EvmExtConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmExtConfig;

impl Default for EvmExtConfig {
    fn default() -> Self {
        Self {
            evm_version: "paris".to_string(),
            optimize: true,
            optimize_runs: 200,
            emit_ir: false,
            emit_asm: false,
            via_ir: false,
            revert_strings: false,
            use_yul: false,
        }
    }
}
