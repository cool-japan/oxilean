//! # Region - Trait Implementations
//!
//! This module contains trait implementations for `Region`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Region;
use std::fmt;

impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Region")
            .field("id", &self.id)
            .field("parent_id", &self.parent_id)
            .field("active", &self.active)
            .field("bytes_used", &self.arena.bytes_used())
            .field("children", &self.children.len())
            .finish()
    }
}
