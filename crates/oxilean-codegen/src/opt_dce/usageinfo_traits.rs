//! # UsageInfo - Trait Implementations
//!
//! This module contains trait implementations for `UsageInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::UsageInfo;
use std::fmt;

impl fmt::Display for UsageInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UsageInfo {{ uses={}, escaping={}, loop={} }}",
            self.use_count, self.is_escaping, self.is_in_loop,
        )
    }
}
