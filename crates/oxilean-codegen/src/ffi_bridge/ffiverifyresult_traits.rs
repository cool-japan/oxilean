//! # FfiVerifyResult - Trait Implementations
//!
//! This module contains trait implementations for `FfiVerifyResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiVerifyResult;
use std::fmt;

impl std::fmt::Display for FfiVerifyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FfiVerify {{ ok={}, errors={}, warnings={} }}",
            self.ok,
            self.errors.len(),
            self.warnings.len()
        )
    }
}
