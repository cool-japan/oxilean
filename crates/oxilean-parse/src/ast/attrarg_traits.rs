//! # AttrArg - Trait Implementations
//!
//! This module contains trait implementations for `AttrArg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AttrArg;
use std::fmt;

impl fmt::Display for AttrArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttrArg::Ident(s) => write!(f, "{}", s),
            AttrArg::Num(n) => write!(f, "{}", n),
            AttrArg::Str(s) => write!(f, "\"{}\"", s),
            AttrArg::List(args) => {
                write!(f, "[")?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, "]")
            }
        }
    }
}
