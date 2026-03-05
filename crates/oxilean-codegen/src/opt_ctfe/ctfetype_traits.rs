//! # CtfeType - Trait Implementations
//!
//! This module contains trait implementations for `CtfeType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeType;
use std::fmt;

impl std::fmt::Display for CtfeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CtfeType::Unit => write!(f, "Unit"),
            CtfeType::Bool => write!(f, "Bool"),
            CtfeType::Int => write!(f, "Int"),
            CtfeType::Uint => write!(f, "Uint"),
            CtfeType::Float => write!(f, "Float"),
            CtfeType::Str => write!(f, "Str"),
            CtfeType::Tuple(ts) => {
                let ss: Vec<String> = ts.iter().map(|t| t.to_string()).collect();
                write!(f, "({})", ss.join(", "))
            }
            CtfeType::List(t) => write!(f, "List[{}]", t),
            CtfeType::Named(n) => write!(f, "{}", n),
            CtfeType::Unknown => write!(f, "?"),
        }
    }
}
