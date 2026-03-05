//! # MatlabType - Trait Implementations
//!
//! This module contains trait implementations for `MatlabType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MatlabType;
use std::fmt;

impl fmt::Display for MatlabType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatlabType::Double => write!(f, "double"),
            MatlabType::Single => write!(f, "single"),
            MatlabType::Int8 => write!(f, "int8"),
            MatlabType::Int16 => write!(f, "int16"),
            MatlabType::Int32 => write!(f, "int32"),
            MatlabType::Int64 => write!(f, "int64"),
            MatlabType::Uint8 => write!(f, "uint8"),
            MatlabType::Uint16 => write!(f, "uint16"),
            MatlabType::Uint32 => write!(f, "uint32"),
            MatlabType::Uint64 => write!(f, "uint64"),
            MatlabType::Logical => write!(f, "logical"),
            MatlabType::Char => write!(f, "char"),
            MatlabType::StringArray => write!(f, "string"),
            MatlabType::Cell => write!(f, "cell"),
            MatlabType::StructType(name) => write!(f, "struct({})", name),
            MatlabType::FunctionHandle => write!(f, "function_handle"),
            MatlabType::Sparse => write!(f, "sparse"),
            MatlabType::Array(inner, dims) => {
                let dims_str: Vec<String> = dims
                    .iter()
                    .map(|d| d.map(|n| n.to_string()).unwrap_or_else(|| ":".to_string()))
                    .collect();
                write!(f, "{}[{}]", inner, dims_str.join("x"))
            }
            MatlabType::Class(name) => write!(f, "{}", name),
            MatlabType::Any => write!(f, "any"),
        }
    }
}
