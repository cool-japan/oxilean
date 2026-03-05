//! # ClusterConfig - Trait Implementations
//!
//! This module contains trait implementations for `ClusterConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ClusterConfig;

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            name: "default-cluster".to_string(),
            max_workers: 64,
            dynamic_scaling: false,
            scale_threshold: 0.8,
        }
    }
}
