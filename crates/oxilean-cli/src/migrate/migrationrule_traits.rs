//! # MigrationRule - Trait Implementations
//!
//! This module contains trait implementations for `MigrationRule`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MigrationRule;
use std::fmt;

impl fmt::Debug for MigrationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MigrationRule")
            .field("name", &self.name)
            .field("priority", &self.priority)
            .finish()
    }
}
