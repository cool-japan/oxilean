//! # NatRelKind - Trait Implementations
//!
//! This module contains trait implementations for `NatRelKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NatRelKind;

impl std::fmt::Display for NatRelKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NatRelKind::Le => write!(f, "≤"),
            NatRelKind::Lt => write!(f, "<"),
            NatRelKind::Eq => write!(f, "="),
            NatRelKind::Ge => write!(f, "≥"),
            NatRelKind::Gt => write!(f, ">"),
        }
    }
}
