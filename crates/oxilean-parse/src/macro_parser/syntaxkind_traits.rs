//! # SyntaxKind - Trait Implementations
//!
//! This module contains trait implementations for `SyntaxKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SyntaxKind;
use std::fmt;

impl fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxKind::Term => write!(f, "term"),
            SyntaxKind::Command => write!(f, "command"),
            SyntaxKind::Tactic => write!(f, "tactic"),
            SyntaxKind::Level => write!(f, "level"),
            SyntaxKind::Attr => write!(f, "attr"),
        }
    }
}
