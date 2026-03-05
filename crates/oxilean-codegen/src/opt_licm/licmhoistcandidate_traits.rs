//! # LicmHoistCandidate - Trait Implementations
//!
//! This module contains trait implementations for `LicmHoistCandidate`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LicmHoistCandidate;
use std::fmt;

impl std::fmt::Display for LicmHoistCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hoist#{}(loop={}, cost={}, pure={})",
            self.inst_id, self.loop_id, self.cost, self.is_side_effect_free
        )
    }
}
