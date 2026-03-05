//! # TypeAnnotationMap - Trait Implementations
//!
//! This module contains trait implementations for `TypeAnnotationMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TypeAnnotationMap;
use std::fmt;

impl Default for TypeAnnotationMap {
    fn default() -> Self {
        TypeAnnotationMap::new()
    }
}
