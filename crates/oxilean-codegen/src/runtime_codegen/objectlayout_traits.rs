//! # ObjectLayout - Trait Implementations
//!
//! This module contains trait implementations for `ObjectLayout`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::ObjectLayout;
use std::fmt;

impl fmt::Display for ObjectLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ObjectLayout {{ tag={}, size={}, align={}, obj_fields={}, scalar={} }}",
            self.tag, self.total_size, self.alignment, self.num_obj_fields, self.scalar_size,
        )
    }
}
