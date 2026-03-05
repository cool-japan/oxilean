//! # AesopConfig - Trait Implementations
//!
//! This module contains trait implementations for `AesopConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{AesopConfig, TransparencyMode};

impl Default for AesopConfig {
    fn default() -> Self {
        Self {
            max_rules: 1024,
            max_depth: 30,
            max_iters: 5000,
            max_rule_apps: 50_000,
            norm_simp: true,
            use_default_rules: true,
            transparency: TransparencyMode::Default,
            depth_penalty: 1.2,
            enable_cache: true,
            timeout_ms: 5000,
            warn_on_failure: false,
            collect_stats: false,
        }
    }
}
