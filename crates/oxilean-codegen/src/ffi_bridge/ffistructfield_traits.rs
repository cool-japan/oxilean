//! # FfiStructField - Trait Implementations
//!
//! This module contains trait implementations for `FfiStructField`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiStructField;
use std::fmt;

impl std::fmt::Display for FfiStructField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(bw) = self.bit_width {
            write!(f, "{} {} : {}", self.field_type, self.name, bw)
        } else {
            write!(f, "{} {}", self.field_type, self.name)
        }
    }
}
