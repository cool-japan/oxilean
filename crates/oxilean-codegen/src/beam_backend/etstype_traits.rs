//! # EtsType - Trait Implementations
//!
//! This module contains trait implementations for `EtsType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::EtsType;
use std::fmt;

impl std::fmt::Display for EtsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EtsType::Set => write!(f, "set"),
            EtsType::OrderedSet => write!(f, "ordered_set"),
            EtsType::Bag => write!(f, "bag"),
            EtsType::DuplicateBag => write!(f, "duplicate_bag"),
        }
    }
}
