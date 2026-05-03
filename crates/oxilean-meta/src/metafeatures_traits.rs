//! # MetaFeatures - Trait Implementations
//!
//! This module contains trait implementations for `MetaFeatures`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaFeatures;

impl Default for MetaFeatures {
    fn default() -> Self {
        Self {
            discr_tree: true,
            whnf_cache: true,
            proof_recording: false,
            instance_synth: true,
            congr_lemmas: true,
        }
    }
}
