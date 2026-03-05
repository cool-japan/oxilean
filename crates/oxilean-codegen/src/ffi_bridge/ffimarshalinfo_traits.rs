//! # FfiMarshalInfo - Trait Implementations
//!
//! This module contains trait implementations for `FfiMarshalInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiMarshalInfo;
use std::fmt;

impl fmt::Display for FfiMarshalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Marshal({}", self.native_type)?;
        if !self.is_trivial {
            write!(f, ", to={}, from={}", self.to_native, self.from_native)?;
        }
        write!(f, ")")
    }
}
