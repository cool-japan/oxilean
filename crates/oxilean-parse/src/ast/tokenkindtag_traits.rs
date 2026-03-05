//! # TokenKindTag - Trait Implementations
//!
//! This module contains trait implementations for `TokenKindTag`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TokenKindTag;
use std::fmt;

impl fmt::Display for TokenKindTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKindTag::Ident => write!(f, "ident"),
            TokenKindTag::Num => write!(f, "num"),
            TokenKindTag::Str => write!(f, "str"),
            TokenKindTag::Op => write!(f, "op"),
            TokenKindTag::Delim => write!(f, "delim"),
            TokenKindTag::Eof => write!(f, "eof"),
        }
    }
}
