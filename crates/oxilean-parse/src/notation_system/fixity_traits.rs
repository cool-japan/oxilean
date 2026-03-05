//! # Fixity - Trait Implementations
//!
//! This module contains trait implementations for `Fixity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Fixity;

impl std::fmt::Display for Fixity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fixity::Prefix => write!(f, "prefix"),
            Fixity::Infixl => write!(f, "infixl"),
            Fixity::Infixr => write!(f, "infixr"),
            Fixity::Postfix => write!(f, "postfix"),
        }
    }
}
