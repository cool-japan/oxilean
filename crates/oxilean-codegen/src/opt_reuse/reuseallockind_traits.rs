//! # ReuseAllocKind - Trait Implementations
//!
//! This module contains trait implementations for `ReuseAllocKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseAllocKind;
use std::fmt;

impl std::fmt::Display for ReuseAllocKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReuseAllocKind::Heap => write!(f, "heap"),
            ReuseAllocKind::Stack => write!(f, "stack"),
            ReuseAllocKind::Scratch => write!(f, "scratch"),
            ReuseAllocKind::Static => write!(f, "static"),
            ReuseAllocKind::Inline => write!(f, "inline"),
        }
    }
}
