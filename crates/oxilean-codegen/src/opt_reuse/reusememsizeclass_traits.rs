//! # ReuseMemSizeClass - Trait Implementations
//!
//! This module contains trait implementations for `ReuseMemSizeClass`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseMemSizeClass;
use std::fmt;

impl std::fmt::Display for ReuseMemSizeClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReuseMemSizeClass::Tiny => write!(f, "tiny"),
            ReuseMemSizeClass::Small => write!(f, "small"),
            ReuseMemSizeClass::Medium => write!(f, "medium"),
            ReuseMemSizeClass::Large => write!(f, "large"),
            ReuseMemSizeClass::Huge => write!(f, "huge"),
            ReuseMemSizeClass::Dynamic => write!(f, "dynamic"),
        }
    }
}
