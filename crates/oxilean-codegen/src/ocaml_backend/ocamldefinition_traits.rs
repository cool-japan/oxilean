//! # OcamlDefinition - Trait Implementations
//!
//! This module contains trait implementations for `OcamlDefinition`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::OcamlDefinition;
use std::fmt;

impl fmt::Display for OcamlDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OcamlDefinition::TypeDef(td) => write!(f, "{}", td),
            OcamlDefinition::Let(lb) => write!(f, "{}", lb),
            OcamlDefinition::Signature(sig) => write!(f, "{}", sig),
            OcamlDefinition::Exception(name, ty) => {
                if let Some(t) = ty {
                    write!(f, "exception {} of {}", name, t)
                } else {
                    write!(f, "exception {}", name)
                }
            }
            OcamlDefinition::Open(module) => write!(f, "open {}", module),
            OcamlDefinition::SubModule(m) => write!(f, "{}", m),
            OcamlDefinition::Comment(text) => write!(f, "(* {} *)", text),
        }
    }
}
