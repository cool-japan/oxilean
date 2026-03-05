//! # BuildPhaseTimer - Trait Implementations
//!
//! This module contains trait implementations for `BuildPhaseTimer`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildPhaseTimer;

impl std::fmt::Display for BuildPhaseTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "parse={}ms typecheck={}ms codegen={}ms link={}ms",
            self.parse_ms, self.typecheck_ms, self.codegen_ms, self.link_ms
        )
    }
}
