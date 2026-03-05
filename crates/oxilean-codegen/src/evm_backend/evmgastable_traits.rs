//! # EvmGasTable - Trait Implementations
//!
//! This module contains trait implementations for `EvmGasTable`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmGasTable;

impl Default for EvmGasTable {
    fn default() -> Self {
        Self {
            stop: 0,
            add: 3,
            mul: 5,
            sub: 3,
            div: 5,
            sdiv: 5,
            mload: 3,
            mstore: 3,
            sload: 2100,
            sstore_set: 20000,
            sstore_clear: 5000,
            call: 100,
            create: 32000,
            sha3: 30,
            sha3_word: 6,
            log: 375,
            log_topic: 375,
            log_byte: 8,
        }
    }
}
