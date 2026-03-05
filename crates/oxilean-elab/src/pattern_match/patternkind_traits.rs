//! # PatternKind - Trait Implementations
//!
//! This module contains trait implementations for `PatternKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PatternKind;
use oxilean_kernel::*;
use std::fmt;

impl std::fmt::Display for PatternKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternKind::Wild => write!(f, "Wild"),
            PatternKind::Variable => write!(f, "Variable"),
            PatternKind::Constructor => write!(f, "Constructor"),
            PatternKind::Literal => write!(f, "Literal"),
            PatternKind::Or => write!(f, "Or"),
            PatternKind::As => write!(f, "As"),
            PatternKind::Inaccessible => write!(f, "Inaccessible"),
        }
    }
}
