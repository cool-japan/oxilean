//! # SynthConfig - Trait Implementations
//!
//! This module contains trait implementations for `SynthConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{DiamondResolutionStrategy, SynthConfig};
use std::fmt;

impl Default for SynthConfig {
    fn default() -> Self {
        SynthConfig {
            max_depth: 32,
            max_instances: 1024,
            allow_defaults: true,
            diamond_strategy: DiamondResolutionStrategy::PreferLowestPriority,
        }
    }
}
