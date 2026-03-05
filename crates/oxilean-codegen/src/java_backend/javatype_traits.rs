//! # JavaType - Trait Implementations
//!
//! This module contains trait implementations for `JavaType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use super::types::JavaType;
use std::fmt;

impl fmt::Display for JavaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JavaType::Int => write!(f, "int"),
            JavaType::Long => write!(f, "long"),
            JavaType::Double => write!(f, "double"),
            JavaType::Float => write!(f, "float"),
            JavaType::Boolean => write!(f, "boolean"),
            JavaType::Char => write!(f, "char"),
            JavaType::Byte => write!(f, "byte"),
            JavaType::Short => write!(f, "short"),
            JavaType::Void => write!(f, "void"),
            JavaType::String => write!(f, "String"),
            JavaType::Object => write!(f, "Object"),
            JavaType::Array(inner) => write!(f, "{}[]", inner),
            JavaType::List(inner) => write!(f, "List<{}>", boxed_to_ref(inner)),
            JavaType::Map(k, v) => {
                write!(f, "Map<{}, {}>", boxed_to_ref(k), boxed_to_ref(v))
            }
            JavaType::Optional(inner) => write!(f, "Optional<{}>", boxed_to_ref(inner)),
            JavaType::Custom(name) => write!(f, "{}", name),
            JavaType::Generic(base, args) => {
                write!(f, "{}<", base)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ">")
            }
        }
    }
}
