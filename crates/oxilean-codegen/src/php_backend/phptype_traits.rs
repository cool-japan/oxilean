//! # PHPType - Trait Implementations
//!
//! This module contains trait implementations for `PHPType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_param;
use super::types::PHPType;
use std::fmt;

impl fmt::Display for PHPType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PHPType::Int => write!(f, "int"),
            PHPType::Float => write!(f, "float"),
            PHPType::String => write!(f, "string"),
            PHPType::Bool => write!(f, "bool"),
            PHPType::Array => write!(f, "array"),
            PHPType::Null => write!(f, "null"),
            PHPType::Mixed => write!(f, "mixed"),
            PHPType::Callable => write!(f, "callable"),
            PHPType::Void => write!(f, "void"),
            PHPType::Never => write!(f, "never"),
            PHPType::Object => write!(f, "object"),
            PHPType::Iterable => write!(f, "iterable"),
            PHPType::Nullable(inner) => write!(f, "?{}", inner),
            PHPType::Union(parts) => {
                let s: Vec<std::string::String> = parts.iter().map(|t| format!("{}", t)).collect();
                write!(f, "{}", s.join("|"))
            }
            PHPType::Intersection(parts) => {
                let s: Vec<std::string::String> = parts.iter().map(|t| format!("{}", t)).collect();
                write!(f, "{}", s.join("&"))
            }
            PHPType::Named(name) => write!(f, "{}", name),
            PHPType::Self_ => write!(f, "self"),
            PHPType::Static => write!(f, "static"),
            PHPType::Parent => write!(f, "parent"),
        }
    }
}
