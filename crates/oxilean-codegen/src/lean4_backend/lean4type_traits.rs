//! # Lean4Type - Trait Implementations
//!
//! This module contains trait implementations for `Lean4Type`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::types::Lean4Type;
use std::fmt;

impl fmt::Display for Lean4Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lean4Type::Nat => write!(f, "Nat"),
            Lean4Type::Int => write!(f, "Int"),
            Lean4Type::Float => write!(f, "Float"),
            Lean4Type::Bool => write!(f, "Bool"),
            Lean4Type::String => write!(f, "String"),
            Lean4Type::Unit => write!(f, "Unit"),
            Lean4Type::Prop => write!(f, "Prop"),
            Lean4Type::Type(0) => write!(f, "Type"),
            Lean4Type::Type(n) => write!(f, "Type {}", n),
            Lean4Type::List(inner) => write!(f, "List {}", paren_type(inner)),
            Lean4Type::Option(inner) => write!(f, "Option {}", paren_type(inner)),
            Lean4Type::Prod(a, b) => {
                write!(f, "{} × {}", paren_complex_type(a), paren_complex_type(b))
            }
            Lean4Type::Sum(a, b) => {
                write!(f, "{} ⊕ {}", paren_complex_type(a), paren_complex_type(b))
            }
            Lean4Type::Fun(a, b) => write!(f, "{} → {}", paren_fun_type(a), b),
            Lean4Type::Custom(name) => write!(f, "{}", name),
            Lean4Type::ForAll(var, domain, body) => {
                write!(f, "∀ ({} : {}), {}", var, domain, body)
            }
            Lean4Type::App(func, arg) => write!(f, "{} {}", func, paren_type(arg)),
            Lean4Type::IO(inner) => write!(f, "IO {}", paren_type(inner)),
            Lean4Type::Array(inner) => write!(f, "Array {}", paren_type(inner)),
            Lean4Type::Fin(n) => write!(f, "Fin {}", paren_type(n)),
            Lean4Type::Char => write!(f, "Char"),
        }
    }
}
