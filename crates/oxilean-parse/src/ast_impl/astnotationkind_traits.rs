//! # AstNotationKind - Trait Implementations
//!
//! This module contains trait implementations for `AstNotationKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AstNotationKind;
use std::fmt;

impl fmt::Display for AstNotationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNotationKind::Prefix => write!(f, "prefix"),
            AstNotationKind::Postfix => write!(f, "postfix"),
            AstNotationKind::Infixl => write!(f, "infixl"),
            AstNotationKind::Infixr => write!(f, "infixr"),
            AstNotationKind::Notation => write!(f, "notation"),
        }
    }
}
