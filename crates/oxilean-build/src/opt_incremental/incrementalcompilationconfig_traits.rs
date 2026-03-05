//! # IncrementalCompilationConfig - Trait Implementations
//!
//! This module contains trait implementations for `IncrementalCompilationConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IncrementalCompilationConfig;

impl Default for IncrementalCompilationConfig {
    fn default() -> Self {
        Self {
            use_interface_hashes: true,
            transitive_propagation: true,
            persist_fingerprints: true,
            max_parallel_rebuilds: 4,
            verbose_logging: false,
        }
    }
}
