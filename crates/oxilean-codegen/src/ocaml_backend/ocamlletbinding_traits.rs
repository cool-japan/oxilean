//! # OcamlLetBinding - Trait Implementations
//!
//! This module contains trait implementations for `OcamlLetBinding`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::OcamlLetBinding;
use std::fmt;

impl fmt::Display for OcamlLetBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_rec {
            write!(f, "let rec {}", self.name)?;
        } else {
            write!(f, "let {}", self.name)?;
        }
        for (param, ty) in &self.params {
            if let Some(t) = ty {
                write!(f, " ({} : {})", param, t)?;
            } else {
                write!(f, " {}", param)?;
            }
        }
        if let Some(ret_ty) = &self.type_annotation {
            write!(f, " : {}", ret_ty)?;
        }
        let body_str = format_ocaml_expr(&self.body, 0);
        write!(f, " =\n  {}", body_str)
    }
}
