//! # OcamlPattern - Trait Implementations
//!
//! This module contains trait implementations for `OcamlPattern`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::OcamlPattern;
use std::fmt;

impl fmt::Display for OcamlPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OcamlPattern::Wildcard => write!(f, "_"),
            OcamlPattern::Var(name) => write!(f, "{}", name),
            OcamlPattern::Const(lit) => write!(f, "{}", lit),
            OcamlPattern::Tuple(pats) => {
                write!(f, "(")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
            OcamlPattern::Cons(head, tail) => write!(f, "{} :: {}", head, tail),
            OcamlPattern::List(pats) => {
                write!(f, "[")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, "]")
            }
            OcamlPattern::Ctor(name, args) => {
                write!(f, "{}", name)?;
                if !args.is_empty() {
                    if args.len() == 1 {
                        write!(f, " {}", args[0])?;
                    } else {
                        write!(f, " (")?;
                        for (i, a) in args.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{}", a)?;
                        }
                        write!(f, ")")?;
                    }
                }
                Ok(())
            }
            OcamlPattern::Record(fields) => {
                write!(f, "{{ ")?;
                for (i, (name, pat)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{} = {}", name, pat)?;
                }
                write!(f, " }}")
            }
            OcamlPattern::Or(p1, p2) => write!(f, "{} | {}", p1, p2),
            OcamlPattern::As(pat, name) => write!(f, "({} as {})", pat, name),
        }
    }
}
