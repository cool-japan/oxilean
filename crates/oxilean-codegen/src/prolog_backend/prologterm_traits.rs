//! # PrologTerm - Trait Implementations
//!
//! This module contains trait implementations for `PrologTerm`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::fmt_dcg_seq;
use super::types::PrologTerm;
use std::fmt;

impl fmt::Display for PrologTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrologTerm::Atom(s) => write!(f, "{}", Self::fmt_atom(s)),
            PrologTerm::Integer(n) => write!(f, "{}", n),
            PrologTerm::Float(x) => {
                let s = format!("{}", x);
                if s.contains('.') {
                    write!(f, "{}", s)
                } else {
                    write!(f, "{}.0", s)
                }
            }
            PrologTerm::Variable(v) => write!(f, "{}", v),
            PrologTerm::Anon => write!(f, "_"),
            PrologTerm::Cut => write!(f, "!"),
            PrologTerm::Nil => write!(f, "[]"),
            PrologTerm::Compound(functor, args) => {
                write!(f, "{}", Self::fmt_atom(functor))?;
                write!(f, "(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if arg.needs_parens_as_arg() {
                        write!(f, "({})", arg)?;
                    } else {
                        write!(f, "{}", arg)?;
                    }
                }
                write!(f, ")")
            }
            PrologTerm::List(elems, tail) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                if let Some(t) = tail {
                    if !elems.is_empty() {
                        write!(f, "|")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, "]")
            }
            PrologTerm::Op(op, lhs, rhs) => {
                let lhs_s = if lhs.needs_parens_as_arg() {
                    format!("({})", lhs)
                } else {
                    format!("{}", lhs)
                };
                let rhs_s = if rhs.needs_parens_as_arg() {
                    format!("({})", rhs)
                } else {
                    format!("{}", rhs)
                };
                write!(f, "{} {} {}", lhs_s, op, rhs_s)
            }
            PrologTerm::PrefixOp(op, arg) => write!(f, "{}({})", op, arg),
            PrologTerm::DcgPhrase(rule, list) => write!(f, "phrase({}, {})", rule, list),
        }
    }
}
