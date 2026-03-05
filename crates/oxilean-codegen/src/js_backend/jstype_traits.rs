//! # JsType - Trait Implementations
//!
//! This module contains trait implementations for `JsType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JsType;
use std::fmt;

impl fmt::Display for JsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsType::Undefined => write!(f, "undefined"),
            JsType::Null => write!(f, "null"),
            JsType::Boolean => write!(f, "boolean"),
            JsType::Number => write!(f, "number"),
            JsType::BigInt => write!(f, "bigint"),
            JsType::String => write!(f, "string"),
            JsType::Object => write!(f, "object"),
            JsType::Array => write!(f, "Array"),
            JsType::Function => write!(f, "function"),
            JsType::Unknown => write!(f, "unknown"),
        }
    }
}
