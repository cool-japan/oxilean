//! # BeamType - Trait Implementations
//!
//! This module contains trait implementations for `BeamType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::BeamType;
use std::fmt;

impl fmt::Display for BeamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeamType::Integer => write!(f, "integer()"),
            BeamType::Float => write!(f, "float()"),
            BeamType::Atom => write!(f, "atom()"),
            BeamType::Pid => write!(f, "pid()"),
            BeamType::Port => write!(f, "port()"),
            BeamType::Reference => write!(f, "reference()"),
            BeamType::Binary => write!(f, "binary()"),
            BeamType::List(inner) => write!(f, "list({})", inner),
            BeamType::Tuple(elems) => {
                write!(f, "{{")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "}}")
            }
            BeamType::Map(k, v) => write!(f, "#{{{}  => {}}}", k, v),
            BeamType::Fun(params, ret) => {
                write!(f, "fun((")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {})", ret)
            }
            BeamType::Any => write!(f, "any()"),
            BeamType::None => write!(f, "none()"),
            BeamType::Union(types) => {
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", t)?;
                }
                Ok(())
            }
            BeamType::Named(name) => write!(f, "{}()", name),
        }
    }
}
