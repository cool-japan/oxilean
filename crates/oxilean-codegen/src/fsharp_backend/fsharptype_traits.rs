//! # FSharpType - Trait Implementations
//!
//! This module contains trait implementations for `FSharpType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::types::FSharpType;
use std::fmt;

impl fmt::Display for FSharpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FSharpType::Int => write!(f, "int"),
            FSharpType::Int64 => write!(f, "int64"),
            FSharpType::Float => write!(f, "float"),
            FSharpType::Float32 => write!(f, "float32"),
            FSharpType::Bool => write!(f, "bool"),
            FSharpType::FsString => write!(f, "string"),
            FSharpType::Char => write!(f, "char"),
            FSharpType::Unit => write!(f, "unit"),
            FSharpType::Byte => write!(f, "byte"),
            FSharpType::List(inner) => write!(f, "{} list", paren_type(inner)),
            FSharpType::Array(inner) => write!(f, "{} array", paren_type(inner)),
            FSharpType::Option(inner) => write!(f, "{} option", paren_type(inner)),
            FSharpType::Result(ok, err) => write!(f, "Result<{}, {}>", ok, err),
            FSharpType::Tuple(elems) => {
                let parts: Vec<String> = elems.iter().map(|e| format!("{}", e)).collect();
                write!(f, "{}", parts.join(" * "))
            }
            FSharpType::Fun(from, to) => write!(f, "{} -> {}", paren_fun_type(from), to),
            FSharpType::Custom(name) => write!(f, "{}", name),
            FSharpType::Generic(name, args) => {
                let arg_str: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                write!(f, "{}<{}>", name, arg_str.join(", "))
            }
            FSharpType::TypeVar(v) => write!(f, "'{}", v),
        }
    }
}
