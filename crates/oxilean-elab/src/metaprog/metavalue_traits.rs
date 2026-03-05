//! # MetaValue - Trait Implementations
//!
//! This module contains trait implementations for `MetaValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaValue;
use std::fmt;

impl std::fmt::Display for MetaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaValue::Quoted(q) => write!(f, "'{}", q.to_debug_string()),
            MetaValue::Bool(b) => write!(f, "{}", b),
            MetaValue::String(s) => write!(f, "\"{}\"", s),
            MetaValue::Int(n) => write!(f, "{}", n),
            MetaValue::List(vs) => {
                let parts: Vec<String> = vs.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", parts.join(", "))
            }
            MetaValue::Unit => write!(f, "()"),
            MetaValue::Error(e) => write!(f, "Error({})", e),
        }
    }
}
