//! # AdamsOperationApplier - Trait Implementations
//!
//! This module contains trait implementations for `AdamsOperationApplier`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AdamsOperationApplier;
use std::fmt;

impl std::fmt::Display for AdamsOperationApplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ψ^{} with Chern classes {:?}",
            self.degree, self.chern_classes
        )
    }
}
