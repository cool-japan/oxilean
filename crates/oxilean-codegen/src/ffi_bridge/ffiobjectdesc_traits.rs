//! # FfiObjectDesc - Trait Implementations
//!
//! This module contains trait implementations for `FfiObjectDesc`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiObjectDesc;
use std::fmt;

impl std::fmt::Display for FfiObjectDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FfiObject({} -> {}, lifetime={})",
            self.c_type, self.rust_type, self.lifetime,
        )
    }
}
