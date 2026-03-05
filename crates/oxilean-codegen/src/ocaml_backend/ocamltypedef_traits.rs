//! # OcamlTypeDef - Trait Implementations
//!
//! This module contains trait implementations for `OcamlTypeDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::{OcamlTypeDecl, OcamlTypeDef};
use std::fmt;

impl fmt::Display for OcamlTypeDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = if self.type_params.is_empty() {
            std::string::String::new()
        } else if self.type_params.len() == 1 {
            format!("'{} ", self.type_params[0])
        } else {
            let ps: Vec<_> = self.type_params.iter().map(|p| format!("'{}", p)).collect();
            format!("({}) ", ps.join(", "))
        };
        match &self.decl {
            OcamlTypeDecl::Alias(ty) => {
                write!(f, "type {}{} = {}", params, self.name, ty)
            }
            OcamlTypeDecl::Record(fields) => {
                writeln!(f, "type {}{} = {{", params, self.name)?;
                for field in fields {
                    if field.mutable {
                        writeln!(f, "  mutable {}: {};", field.name, field.ty)?;
                    } else {
                        writeln!(f, "  {}: {};", field.name, field.ty)?;
                    }
                }
                write!(f, "}}")
            }
            OcamlTypeDecl::Variant(variants) => {
                write!(f, "type {}{} =", params, self.name)?;
                for (ctor, args) in variants {
                    if args.is_empty() {
                        write!(f, "\n  | {}", ctor)?;
                    } else {
                        let arg_str: Vec<_> = args.iter().map(|t| t.to_string()).collect();
                        write!(f, "\n  | {} of {}", ctor, arg_str.join(" * "))?;
                    }
                }
                Ok(())
            }
            OcamlTypeDecl::Abstract => write!(f, "type {}{}", params, self.name),
        }
    }
}
