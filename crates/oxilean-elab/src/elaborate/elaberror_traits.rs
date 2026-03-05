//! # ElabError - Trait Implementations
//!
//! This module contains trait implementations for `ElabError`.
//!
//! ## Implemented Traits
//!
//! - `From`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabError;
use oxilean_kernel::*;
use std::fmt;

impl From<oxilean_kernel::KernelError> for ElabError {
    fn from(e: oxilean_kernel::KernelError) -> Self {
        ElabError::Kernel(e)
    }
}

impl std::fmt::Display for ElabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElabError::Kernel(e) => write!(f, "kernel error: {:?}", e),
            ElabError::NameNotFound(n) => write!(f, "name not found: {}", n),
            ElabError::TypeError(s) => write!(f, "type error: {}", s),
            ElabError::Ambiguous(s) => write!(f, "ambiguous: {}", s),
            ElabError::ImplicitArgFailed(s) => write!(f, "implicit arg failed: {}", s),
            ElabError::OverloadAmbiguity(s) => write!(f, "overload ambiguity: {}", s),
            ElabError::CoercionFailed(s) => write!(f, "coercion failed: {}", s),
            ElabError::TacticFailed(s) => write!(f, "tactic failed: {}", s),
            ElabError::Other(s) => write!(f, "{}", s),
        }
    }
}
