//! # NotationKind - Trait Implementations
//!
//! This module contains trait implementations for `NotationKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NotationKind;

impl std::fmt::Display for NotationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotationKind::Prefix => write!(f, "prefix"),
            NotationKind::Postfix => write!(f, "postfix"),
            NotationKind::Infixl => write!(f, "infixl"),
            NotationKind::Infixr => write!(f, "infixr"),
            NotationKind::Notation => write!(f, "notation"),
        }
    }
}
