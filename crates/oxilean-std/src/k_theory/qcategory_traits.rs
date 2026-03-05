//! # QCategory - Trait Implementations
//!
//! This module contains trait implementations for `QCategory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::QCategory;
use std::fmt;

impl std::fmt::Display for QCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "QCat[{}]: {} objects, {} monos, {} epis",
            self.name,
            self.objects.len(),
            self.mono.len(),
            self.epi.len()
        )
    }
}
