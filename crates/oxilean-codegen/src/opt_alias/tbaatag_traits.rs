//! # TbaaTag - Trait Implementations
//!
//! This module contains trait implementations for `TbaaTag`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TbaaTag;
use std::fmt;

impl std::fmt::Display for TbaaTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tbaa!({} at +{} as {}{})",
            self.base_type.name,
            self.offset,
            self.access_type.name,
            if self.is_const { " [const]" } else { "" }
        )
    }
}
