//! # OcamlSigItem - Trait Implementations
//!
//! This module contains trait implementations for `OcamlSigItem`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::OcamlSigItem;
use std::fmt;

impl fmt::Display for OcamlSigItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OcamlSigItem::Val(name, ty) => write!(f, "val {} : {}", name, ty),
            OcamlSigItem::Type(typedef) => write!(f, "{}", typedef),
            OcamlSigItem::Module(name, sig) => write!(f, "module {} : {}", name, sig),
            OcamlSigItem::Exception(name, ty) => {
                if let Some(t) = ty {
                    write!(f, "exception {} of {}", name, t)
                } else {
                    write!(f, "exception {}", name)
                }
            }
        }
    }
}
