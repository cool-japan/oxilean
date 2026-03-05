//! # MlirAttr - Trait Implementations
//!
//! This module contains trait implementations for `MlirAttr`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{AffineMap, MlirAttr};
use std::fmt;

impl fmt::Display for MlirAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MlirAttr::Integer(n, ty) => write!(f, "{} : {}", n, ty),
            MlirAttr::Float(v) => write!(f, "{:.6e}", v),
            MlirAttr::Str(s) => {
                write!(f, "\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
            }
            MlirAttr::Type(ty) => write!(f, "{}", ty),
            MlirAttr::Array(elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            MlirAttr::Dict(pairs) => {
                write!(f, "{{")?;
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} = {}", k, v)?;
                }
                write!(f, "}}")
            }
            MlirAttr::AffineMap(s) => write!(f, "affine_map<{}>", s),
            MlirAttr::Unit => write!(f, "unit"),
            MlirAttr::Bool(b) => write!(f, "{}", b),
            MlirAttr::Symbol(name) => write!(f, "@{}", name),
            MlirAttr::Dense(elems, ty) => {
                write!(f, "dense<[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, "]> : {}", ty)
            }
        }
    }
}
