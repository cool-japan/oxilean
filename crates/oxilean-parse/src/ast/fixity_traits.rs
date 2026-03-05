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
use std::fmt;

impl fmt::Display for Fixity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fixity::InfixLeft => write!(f, "infixl"),
            Fixity::InfixRight => write!(f, "infixr"),
            Fixity::Infix => write!(f, "infix"),
            Fixity::Prefix => write!(f, "prefix"),
            Fixity::Postfix => write!(f, "postfix"),
        }
    }
}
