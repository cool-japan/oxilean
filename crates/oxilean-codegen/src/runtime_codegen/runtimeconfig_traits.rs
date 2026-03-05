//! # RuntimeConfig - Trait Implementations
//!
//! This module contains trait implementations for `RuntimeConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::{AllocStrategy, ClosureRepr, RcStrategy, RuntimeConfig};
use std::fmt;

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfig {
            rc_strategy: RcStrategy::Standard,
            alloc_strategy: AllocStrategy::LeanRuntime,
            closure_repr: ClosureRepr::Standard,
            debug_checks: false,
            thread_safe: false,
        }
    }
}

impl fmt::Display for RuntimeConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RuntimeConfig {{ rc={:?}, alloc={:?}, closure={:?}, debug={}, thread_safe={} }}",
            self.rc_strategy,
            self.alloc_strategy,
            self.closure_repr,
            self.debug_checks,
            self.thread_safe,
        )
    }
}
