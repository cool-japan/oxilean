//! # FfiLifetime - Trait Implementations
//!
//! This module contains trait implementations for `FfiLifetime`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiLifetime;
use std::fmt;

impl std::fmt::Display for FfiLifetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiLifetime::Static => write!(f, "static"),
            FfiLifetime::ScopedToCall => write!(f, "scoped_to_call"),
            FfiLifetime::CallerManaged => write!(f, "caller_managed"),
            FfiLifetime::LibraryManaged => write!(f, "library_managed"),
            FfiLifetime::RefCounted => write!(f, "ref_counted"),
        }
    }
}
