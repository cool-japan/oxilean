//! # SimplicialObject - Trait Implementations
//!
//! This module contains trait implementations for `SimplicialObject`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SimplicialObject;
use std::fmt;

impl<T: fmt::Debug> fmt::Display for SimplicialObject<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SimplicialObject[0..{}]({:?})",
            self.objects.len(),
            self.face_map_desc
        )
    }
}
