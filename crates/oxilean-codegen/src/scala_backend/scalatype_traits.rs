//! # ScalaType - Trait Implementations
//!
//! This module contains trait implementations for `ScalaType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaType;
use std::fmt;

impl fmt::Display for ScalaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalaType::Int => write!(f, "Int"),
            ScalaType::Long => write!(f, "Long"),
            ScalaType::Double => write!(f, "Double"),
            ScalaType::Float => write!(f, "Float"),
            ScalaType::Boolean => write!(f, "Boolean"),
            ScalaType::Char => write!(f, "Char"),
            ScalaType::ScalaString => write!(f, "String"),
            ScalaType::Unit => write!(f, "Unit"),
            ScalaType::Null => write!(f, "Null"),
            ScalaType::Nothing => write!(f, "Nothing"),
            ScalaType::Any => write!(f, "Any"),
            ScalaType::AnyRef => write!(f, "AnyRef"),
            ScalaType::AnyVal => write!(f, "AnyVal"),
            ScalaType::List(inner) => write!(f, "List[{}]", inner),
            ScalaType::Option(inner) => write!(f, "Option[{}]", inner),
            ScalaType::Either(a, b) => write!(f, "Either[{}, {}]", a, b),
            ScalaType::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            ScalaType::Function(params, ret) => {
                if params.len() == 1 {
                    write!(f, "{} => {}", params[0], ret)
                } else {
                    write!(f, "(")?;
                    for (i, p) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", p)?;
                    }
                    write!(f, ") => {}", ret)
                }
            }
            ScalaType::Custom(name) => write!(f, "{}", name),
            ScalaType::Generic(name, args) => {
                write!(f, "{}[", name)?;
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
