//! # FixpointState - Trait Implementations
//!
//! This module contains trait implementations for `FixpointState`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FixpointState;

impl Default for FixpointState {
    fn default() -> Self {
        FixpointState::new()
    }
}
