//! # FutharkKernelLaunch - Trait Implementations
//!
//! This module contains trait implementations for `FutharkKernelLaunch`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkKernelLaunch;
use std::fmt;

impl std::fmt::Display for FutharkKernelLaunch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let global: Vec<String> = self.global_size.iter().map(|x| x.to_string()).collect();
        let local: Vec<String> = self.local_size.iter().map(|x| x.to_string()).collect();
        write!(
            f,
            "launch {}([{}], [{}], shm={})",
            self.kernel_name,
            global.join(", "),
            local.join(", "),
            self.shared_mem_bytes,
        )
    }
}
