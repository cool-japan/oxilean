//! # DhallType - Trait Implementations
//!
//! This module contains trait implementations for `DhallType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DhallType;
use std::fmt;

impl fmt::Display for DhallType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DhallType::Bool => write!(f, "Bool"),
            DhallType::Natural => write!(f, "Natural"),
            DhallType::Integer => write!(f, "Integer"),
            DhallType::Double => write!(f, "Double"),
            DhallType::Text => write!(f, "Text"),
            DhallType::List(t) => write!(f, "List {}", t),
            DhallType::Optional(t) => write!(f, "Optional {}", t),
            DhallType::Record(fields) => {
                write!(f, "{{ ")?;
                for (i, (name, ty)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} : {}", name, ty)?;
                }
                write!(f, " }}")
            }
            DhallType::Union(variants) => {
                write!(f, "< ")?;
                for (i, (name, ty)) in variants.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    match ty {
                        None => write!(f, "{}", name)?,
                        Some(t) => write!(f, "{} : {}", name, t)?,
                    }
                }
                write!(f, " >")
            }
            DhallType::Function(a, b) => write!(f, "{} -> {}", a, b),
            DhallType::Forall(x, a, b) => write!(f, "forall ({} : {}) -> {}", x, a, b),
            DhallType::Type => write!(f, "Type"),
            DhallType::Kind => write!(f, "Kind"),
            DhallType::Sort => write!(f, "Sort"),
            DhallType::Named(n) => write!(f, "{}", n),
        }
    }
}
