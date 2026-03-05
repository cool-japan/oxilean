//! # TsType - Trait Implementations
//!
//! This module contains trait implementations for `TsType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsType;
use std::fmt;

impl fmt::Display for TsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsType::Number => write!(f, "number"),
            TsType::String => write!(f, "string"),
            TsType::Boolean => write!(f, "boolean"),
            TsType::Void => write!(f, "void"),
            TsType::Never => write!(f, "never"),
            TsType::Unknown => write!(f, "unknown"),
            TsType::Any => write!(f, "any"),
            TsType::Null => write!(f, "null"),
            TsType::Undefined => write!(f, "undefined"),
            TsType::Tuple(elems) => {
                write!(f, "[")?;
                for (i, t) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, "]")
            }
            TsType::Array(inner) => write!(f, "{}[]", inner),
            TsType::Object(fields) => {
                write!(f, "{{ ")?;
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, " }}")
            }
            TsType::Union(variants) => {
                for (i, v) in variants.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", v)?;
                }
                Ok(())
            }
            TsType::Intersection(parts) => {
                for (i, p) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(f, " & ")?;
                    }
                    write!(f, "{}", p)?;
                }
                Ok(())
            }
            TsType::Function { params, ret } => {
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "p{}: {}", i, p)?;
                }
                write!(f, ") => {}", ret)
            }
            TsType::Custom(name) => write!(f, "{}", name),
            TsType::Generic(name, args) => {
                write!(f, "{}<", name)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ">")
            }
            TsType::ReadOnly(inner) => write!(f, "readonly {}", inner),
            TsType::Readonly => write!(f, "Readonly"),
        }
    }
}
