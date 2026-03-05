//! # OperatorSpaceData - Trait Implementations
//!
//! This module contains trait implementations for `OperatorSpaceData`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OperatorSpaceData;
use std::fmt;

impl std::fmt::Display for OperatorSpaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OpSpace[{}](col_H={}, self_dual={}, cb_norm={:.3})",
            self.name, self.is_column_hilbert_space, self.is_self_dual, self.cb_norm_estimate
        )
    }
}
