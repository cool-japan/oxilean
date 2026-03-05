//! # DependenceKind - Trait Implementations
//!
//! This module contains trait implementations for `DependenceKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DependenceKind;
use std::fmt;

impl fmt::Display for DependenceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependenceKind::True => write!(f, "RAW"),
            DependenceKind::Anti => write!(f, "WAR"),
            DependenceKind::Output => write!(f, "WAW"),
            DependenceKind::Input => write!(f, "RAR"),
        }
    }
}
