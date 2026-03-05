//! # ScalaDecl - Trait Implementations
//!
//! This module contains trait implementations for `ScalaDecl`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaDecl;
use std::fmt;

impl fmt::Display for ScalaDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalaDecl::CaseClass(c) => write!(f, "{}", c),
            ScalaDecl::Trait(t) => write!(f, "{}", t),
            ScalaDecl::Enum(e) => write!(f, "{}", e),
            ScalaDecl::Object(o) => write!(f, "{}", o),
            ScalaDecl::Class(c) => write!(f, "{}", c),
            ScalaDecl::Method(m) => write!(f, "{}", m),
            ScalaDecl::Val(name, ty, expr) => {
                write!(f, "val {}: {} = {}", name, ty, expr)
            }
            ScalaDecl::TypeAlias(name, params, ty) => {
                write!(f, "type {}", name)?;
                if !params.is_empty() {
                    write!(f, "[")?;
                    for (i, p) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", p)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, " = {}", ty)
            }
            ScalaDecl::OpaqueType(name, params, ty) => {
                write!(f, "opaque type {}", name)?;
                if !params.is_empty() {
                    write!(f, "[")?;
                    for (i, p) in params.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", p)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, " = {}", ty)
            }
            ScalaDecl::Extension(ty, methods) => {
                write!(f, "extension (x: {})", ty)?;
                write!(f, " {{")?;
                for m in methods {
                    write!(f, "\n  {}", m)?;
                }
                write!(f, "\n}}")
            }
            ScalaDecl::Given(name, ty, methods) => {
                write!(f, "given {}: {} with {{", name, ty)?;
                for m in methods {
                    write!(f, "\n  {}", m)?;
                }
                write!(f, "\n}}")
            }
            ScalaDecl::Comment(c) => write!(f, "// {}", c),
            ScalaDecl::RawLine(l) => write!(f, "{}", l),
        }
    }
}
