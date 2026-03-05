//! # ReuseAllocSiteInfo - Trait Implementations
//!
//! This module contains trait implementations for `ReuseAllocSiteInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseAllocSiteInfo;
use std::fmt;

impl std::fmt::Display for ReuseAllocSiteInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.site, self.decision)
    }
}
