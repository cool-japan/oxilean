//! # DistributedBuildConfig - Trait Implementations
//!
//! This module contains trait implementations for `DistributedBuildConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{DistributedBuildConfig, FaultTolerance, JobSchedulerKind};

impl Default for DistributedBuildConfig {
    fn default() -> Self {
        Self {
            scheduler: JobSchedulerKind::LeastLoaded,
            ft: FaultTolerance::default(),
            use_remote_cache: true,
            max_global_jobs: 32,
            verbose: false,
        }
    }
}
