//! # NixType - Trait Implementations
//!
//! This module contains trait implementations for `NixType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NixType;
use std::fmt;

impl fmt::Display for NixType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NixType::Int => write!(f, "int"),
            NixType::Float => write!(f, "float"),
            NixType::Bool => write!(f, "bool"),
            NixType::String => write!(f, "string"),
            NixType::Path => write!(f, "path"),
            NixType::NullType => write!(f, "null"),
            NixType::List(t) => write!(f, "[ {} ]", t),
            NixType::AttrSet(fields) => {
                write!(f, "{{ ")?;
                for (i, (name, ty)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{} : {}", name, ty)?;
                }
                write!(f, " }}")
            }
            NixType::Function(a, b) => write!(f, "{} -> {}", a, b),
            NixType::Derivation => write!(f, "derivation"),
        }
    }
}
