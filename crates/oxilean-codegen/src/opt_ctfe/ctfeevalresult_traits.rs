//! # CtfeEvalResult - Trait Implementations
//!
//! This module contains trait implementations for `CtfeEvalResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeEvalResult;
use std::fmt;

impl std::fmt::Display for CtfeEvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self
            .value
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or("None".to_string());
        write!(
            f,
            "CtfeEvalResult {{ val={}, fuel_used={}, steps={}, memo_hit={} }}",
            val, self.fuel_used, self.steps, self.memo_hit
        )
    }
}
