//! # MacroToken - Trait Implementations
//!
//! This module contains trait implementations for `MacroToken`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MacroToken;
use std::fmt;

impl fmt::Display for MacroToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MacroToken::Literal(tk) => write!(f, "{}", tk),
            MacroToken::Var(name) => write!(f, "${}", name),
            MacroToken::Repeat(toks) => {
                write!(f, "$(")?;
                for t in toks {
                    write!(f, "{}", t)?;
                }
                write!(f, ")*")
            }
            MacroToken::Optional(toks) => {
                write!(f, "$(")?;
                for t in toks {
                    write!(f, "{}", t)?;
                }
                write!(f, ")?")
            }
            MacroToken::Quote(toks) => {
                write!(f, "`(")?;
                for t in toks {
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            MacroToken::Antiquote(name) => write!(f, "${}", name),
            MacroToken::SpliceArray(name) => write!(f, "$[{}]*", name),
        }
    }
}
