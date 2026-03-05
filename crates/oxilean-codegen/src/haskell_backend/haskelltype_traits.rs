//! # HaskellType - Trait Implementations
//!
//! This module contains trait implementations for `HaskellType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::HaskellType;
use std::fmt;

impl fmt::Display for HaskellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HaskellType::Int => write!(f, "Int"),
            HaskellType::Integer => write!(f, "Integer"),
            HaskellType::Double => write!(f, "Double"),
            HaskellType::Float => write!(f, "Float"),
            HaskellType::Bool => write!(f, "Bool"),
            HaskellType::Char => write!(f, "Char"),
            HaskellType::HsString => write!(f, "String"),
            HaskellType::Unit => write!(f, "()"),
            HaskellType::IO(inner) => write!(f, "IO {}", paren_type(inner)),
            HaskellType::List(inner) => write!(f, "[{}]", inner),
            HaskellType::Maybe(inner) => write!(f, "Maybe {}", paren_type(inner)),
            HaskellType::Either(a, b) => {
                write!(f, "Either {} {}", paren_type(a), paren_type(b))
            }
            HaskellType::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            HaskellType::Fun(a, b) => write!(f, "{} -> {}", paren_fun_type(a), b),
            HaskellType::Custom(name) => write!(f, "{}", name),
            HaskellType::Polymorphic(var) => write!(f, "{}", var),
            HaskellType::Constraint(cls, args) => {
                write!(f, "{}", cls)?;
                for a in args {
                    write!(f, " {}", paren_type(a))?;
                }
                Ok(())
            }
        }
    }
}
