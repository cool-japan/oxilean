//! # OcamlType - Trait Implementations
//!
//! This module contains trait implementations for `OcamlType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::OcamlType;
use std::fmt;

impl fmt::Display for OcamlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OcamlType::Int => write!(f, "int"),
            OcamlType::Float => write!(f, "float"),
            OcamlType::Bool => write!(f, "bool"),
            OcamlType::Char => write!(f, "char"),
            OcamlType::String => write!(f, "string"),
            OcamlType::Unit => write!(f, "unit"),
            OcamlType::Never => write!(f, "'never"),
            OcamlType::List(inner) => write!(f, "{} list", inner),
            OcamlType::Array(inner) => write!(f, "{} array", inner),
            OcamlType::Tuple(elems) => {
                for (i, t) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    match t {
                        OcamlType::Fun(_, _) => write!(f, "({})", t)?,
                        _ => write!(f, "{}", t)?,
                    }
                }
                Ok(())
            }
            OcamlType::Option(inner) => write!(f, "{} option", inner),
            OcamlType::Result(ok, err) => write!(f, "({}, {}) result", ok, err),
            OcamlType::Fun(param, ret) => match param.as_ref() {
                OcamlType::Fun(_, _) => write!(f, "({}) -> {}", param, ret),
                _ => write!(f, "{} -> {}", param, ret),
            },
            OcamlType::Custom(name) => write!(f, "{}", name),
            OcamlType::Polymorphic(name) => write!(f, "'{}", name),
            OcamlType::Module(path) => write!(f, "{}", path),
        }
    }
}
