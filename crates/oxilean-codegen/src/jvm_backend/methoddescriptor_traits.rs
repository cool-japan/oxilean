//! # MethodDescriptor - Trait Implementations
//!
//! This module contains trait implementations for `MethodDescriptor`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::MethodDescriptor;
use std::fmt;

impl std::fmt::Display for MethodDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: String = self.params.iter().map(|t| t.descriptor()).collect();
        write!(f, "({}){}", params, self.return_type.descriptor())
    }
}
