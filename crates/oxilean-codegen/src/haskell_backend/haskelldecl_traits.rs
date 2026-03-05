//! # HaskellDecl - Trait Implementations
//!
//! This module contains trait implementations for `HaskellDecl`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::HaskellDecl;
use std::fmt;

impl fmt::Display for HaskellDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HaskellDecl::Data(d) => write!(f, "{}", d),
            HaskellDecl::Newtype(n) => write!(f, "{}", n),
            HaskellDecl::TypeClass(c) => write!(f, "{}", c),
            HaskellDecl::Instance(i) => write!(f, "{}", i),
            HaskellDecl::Function(func) => write!(f, "{}", func),
            HaskellDecl::TypeSynonym(name, params, ty) => {
                write!(f, "type {}", name)?;
                for p in params {
                    write!(f, " {}", p)?;
                }
                write!(f, " = {}", ty)
            }
            HaskellDecl::Comment(c) => write!(f, "-- {}", c),
            HaskellDecl::RawLine(l) => write!(f, "{}", l),
        }
    }
}
