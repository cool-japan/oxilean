//! # StringLayout - Trait Implementations
//!
//! This module contains trait implementations for `StringLayout`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::StringLayout;
use std::fmt;

impl fmt::Display for StringLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StringLayout {{ sso={}, max_len={}, data_offset={} }}",
            self.is_sso, self.sso_max_len, self.data_offset
        )
    }
}
