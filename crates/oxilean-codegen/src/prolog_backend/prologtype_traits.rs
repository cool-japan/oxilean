//! # PrologType - Trait Implementations
//!
//! This module contains trait implementations for `PrologType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::fmt_dcg_seq;
use super::types::PrologType;
use std::fmt;

impl fmt::Display for PrologType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrologType::Integer => write!(f, "integer"),
            PrologType::Float => write!(f, "float"),
            PrologType::Atom => write!(f, "atom"),
            PrologType::PrologString => write!(f, "string"),
            PrologType::List(inner) => write!(f, "list({})", inner),
            PrologType::Compound => write!(f, "compound"),
            PrologType::Callable => write!(f, "callable"),
            PrologType::Term => write!(f, "term"),
            PrologType::Boolean => write!(f, "boolean"),
            PrologType::Var => write!(f, "var"),
            PrologType::Nonvar => write!(f, "nonvar"),
            PrologType::Number => write!(f, "number"),
            PrologType::Atomic => write!(f, "atomic"),
            PrologType::PositiveInteger => write!(f, "positive_integer"),
            PrologType::NonNeg => write!(f, "nonneg"),
            PrologType::Custom(s) => write!(f, "{}", s),
        }
    }
}
