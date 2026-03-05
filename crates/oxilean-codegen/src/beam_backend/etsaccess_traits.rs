//! # EtsAccess - Trait Implementations
//!
//! This module contains trait implementations for `EtsAccess`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::EtsAccess;
use std::fmt;

impl std::fmt::Display for EtsAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EtsAccess::Private => write!(f, "private"),
            EtsAccess::Protected => write!(f, "protected"),
            EtsAccess::Public => write!(f, "public"),
        }
    }
}
