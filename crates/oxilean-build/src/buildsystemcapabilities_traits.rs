//! # BuildSystemCapabilities - Trait Implementations
//!
//! This module contains trait implementations for `BuildSystemCapabilities`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildSystemCapabilities;
use std::fmt;

impl std::fmt::Display for BuildSystemCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Capabilities[incremental={} distributed={} remote_cache={} parallel={} max_jobs={}]",
            self.incremental, self.distributed, self.remote_cache, self.parallel, self.max_jobs,
        )
    }
}
