//! # CfgSnapshot - Trait Implementations
//!
//! This module contains trait implementations for `CfgSnapshot`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CfgSnapshot;

impl std::fmt::Display for CfgSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "arch={} os={} debug={} 64bit={}",
            self.arch, self.os, self.debug, self.is_64bit
        )
    }
}
