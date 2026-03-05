//! # TokenCategory - Trait Implementations
//!
//! This module contains trait implementations for `TokenCategory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TokenCategory;

impl std::fmt::Display for TokenCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenCategory::Ident => write!(f, "ident"),
            TokenCategory::Keyword => write!(f, "keyword"),
            TokenCategory::Literal => write!(f, "literal"),
            TokenCategory::Operator => write!(f, "operator"),
            TokenCategory::Delimiter => write!(f, "delimiter"),
            TokenCategory::Trivia => write!(f, "trivia"),
            TokenCategory::Eof => write!(f, "eof"),
        }
    }
}
