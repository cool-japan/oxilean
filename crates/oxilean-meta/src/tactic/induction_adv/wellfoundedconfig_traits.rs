//! # WellFoundedConfig - Trait Implementations
//!
//! This module contains trait implementations for `WellFoundedConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WellFoundedConfig;

impl Default for WellFoundedConfig {
    fn default() -> Self {
        Self {
            relation: None,
            wf_proof_name: None,
            measure: None,
            auto_measure: true,
            ih_names: Vec::new(),
            max_depth: 64,
            use_sizeof: false,
        }
    }
}
