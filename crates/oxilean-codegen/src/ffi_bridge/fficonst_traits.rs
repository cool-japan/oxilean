//! # FfiConst - Trait Implementations
//!
//! This module contains trait implementations for `FfiConst`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiConst;
use std::fmt;

impl std::fmt::Display for FfiConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "static const {} {} = {};",
            self.const_type, self.name, self.value
        )
    }
}
