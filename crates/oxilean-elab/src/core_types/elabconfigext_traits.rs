//! # ElabConfigExt - Trait Implementations
//!
//! This module contains trait implementations for `ElabConfigExt`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ElabConfigExt, UniverseCheckMode};

impl Default for ElabConfigExt {
    fn default() -> Self {
        Self {
            max_metavars: 10_000,
            max_depth: 500,
            warn_sorry: true,
            check_unused_hyps: false,
            allow_sorry: true,
            resolve_coercions: true,
            bidir_checking: true,
            universe_checking: UniverseCheckMode::Partial,
        }
    }
}
