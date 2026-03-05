//! # ChapelIntent - Trait Implementations
//!
//! This module contains trait implementations for `ChapelIntent`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ChapelIntent;
use std::fmt;

impl fmt::Display for ChapelIntent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChapelIntent::In => write!(f, "in"),
            ChapelIntent::Out => write!(f, "out"),
            ChapelIntent::InOut => write!(f, "inout"),
            ChapelIntent::Ref => write!(f, "ref"),
            ChapelIntent::Const => write!(f, "const"),
            ChapelIntent::ConstRef => write!(f, "const ref"),
            ChapelIntent::Param => write!(f, "param"),
            ChapelIntent::Type => write!(f, "type"),
        }
    }
}
