//! # IdrisType - Trait Implementations
//!
//! This module contains trait implementations for `IdrisType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IdrisType;
use std::fmt;

impl fmt::Display for IdrisType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdrisType::Type => write!(f, "Type"),
            IdrisType::Integer => write!(f, "Integer"),
            IdrisType::Nat => write!(f, "Nat"),
            IdrisType::Bool => write!(f, "Bool"),
            IdrisType::String => write!(f, "String"),
            IdrisType::Char => write!(f, "Char"),
            IdrisType::Double => write!(f, "Double"),
            IdrisType::Unit => write!(f, "()"),
            IdrisType::Var(v) => write!(f, "{}", v),
            IdrisType::List(a) => write!(f, "List {}", a.fmt_parens()),
            IdrisType::Vect(n, a) => write!(f, "Vect {} {}", n, a.fmt_parens()),
            IdrisType::Pair(a, b) => write!(f, "({}, {})", a, b),
            IdrisType::IO(a) => write!(f, "IO {}", a.fmt_parens()),
            IdrisType::Maybe(a) => write!(f, "Maybe {}", a.fmt_parens()),
            IdrisType::Either(a, b) => {
                write!(f, "Either {} {}", a.fmt_parens(), b.fmt_parens())
            }
            IdrisType::Function(a, b) => write!(f, "{} -> {}", a.fmt_parens(), b),
            IdrisType::Linear(a, b) => write!(f, "(1 _ : {}) -> {}", a, b),
            IdrisType::Erased(a, b) => write!(f, "(0 _ : {}) -> {}", a, b),
            IdrisType::Pi(x, a, b) => write!(f, "({} : {}) -> {}", x, a, b),
            IdrisType::Data(name, args) => {
                if args.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}", name)?;
                    for a in args {
                        write!(f, " {}", a.fmt_parens())?;
                    }
                    Ok(())
                }
            }
            IdrisType::Interface(name, args) => {
                if args.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}", name)?;
                    for a in args {
                        write!(f, " {}", a.fmt_parens())?;
                    }
                    Ok(())
                }
            }
        }
    }
}
