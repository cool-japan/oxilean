//! # CtfeValue - Trait Implementations
//!
//! This module contains trait implementations for `CtfeValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeValue;
use std::fmt;

impl fmt::Display for CtfeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CtfeValue::Int(n) => write!(f, "{}", n),
            CtfeValue::Float(x) => write!(f, "{}", x),
            CtfeValue::Bool(b) => write!(f, "{}", b),
            CtfeValue::String(s) => write!(f, "\"{}\"", s),
            CtfeValue::List(xs) => {
                write!(f, "[")?;
                for (i, v) in xs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            CtfeValue::Tuple(xs) => {
                write!(f, "(")?;
                for (i, v) in xs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
            CtfeValue::Constructor(name, fields) => {
                write!(f, "{}", name)?;
                if !fields.is_empty() {
                    write!(f, "(")?;
                    for (i, v) in fields.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", v)?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            CtfeValue::Undef => write!(f, "undef"),
        }
    }
}
