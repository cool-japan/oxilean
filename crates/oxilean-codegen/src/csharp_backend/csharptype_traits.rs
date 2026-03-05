//! # CSharpType - Trait Implementations
//!
//! This module contains trait implementations for `CSharpType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CSharpType;
use std::fmt;

impl fmt::Display for CSharpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CSharpType::Int => write!(f, "int"),
            CSharpType::Long => write!(f, "long"),
            CSharpType::Double => write!(f, "double"),
            CSharpType::Float => write!(f, "float"),
            CSharpType::Bool => write!(f, "bool"),
            CSharpType::String => write!(f, "string"),
            CSharpType::Void => write!(f, "void"),
            CSharpType::Object => write!(f, "object"),
            CSharpType::List(inner) => write!(f, "List<{}>", inner),
            CSharpType::Dict(k, v) => write!(f, "Dictionary<{}, {}>", k, v),
            CSharpType::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            CSharpType::Custom(name) => write!(f, "{}", name),
            CSharpType::Nullable(inner) => write!(f, "{}?", inner),
            CSharpType::Task(inner) => {
                if **inner == CSharpType::Void {
                    write!(f, "Task")
                } else {
                    write!(f, "Task<{}>", inner)
                }
            }
            CSharpType::IEnumerable(inner) => write!(f, "IEnumerable<{}>", inner),
            CSharpType::Func(params, ret) => {
                write!(f, "Func<")?;
                for p in params.iter() {
                    write!(f, "{}, ", p)?;
                }
                write!(f, "{}>", ret)
            }
            CSharpType::Action(params) => {
                if params.is_empty() {
                    write!(f, "Action")
                } else {
                    write!(f, "Action<")?;
                    for (i, p) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", p)?;
                    }
                    write!(f, ">")
                }
            }
        }
    }
}
