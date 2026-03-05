//! # AliasResultExt - Trait Implementations
//!
//! This module contains trait implementations for `AliasResultExt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AliasResultExt;
use std::fmt;

impl std::fmt::Display for AliasResultExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AliasResultExt::NoAlias => write!(f, "NoAlias"),
            AliasResultExt::MayAlias => write!(f, "MayAlias"),
            AliasResultExt::PartialAlias => write!(f, "PartialAlias"),
            AliasResultExt::MustAlias => write!(f, "MustAlias"),
        }
    }
}
