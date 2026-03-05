//! # BeamVal - Trait Implementations
//!
//! This module contains trait implementations for `BeamVal`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::BeamVal;
use std::fmt;

impl fmt::Display for BeamVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeamVal::Reg(r) => write!(f, "{}", r),
            BeamVal::Int(n) => write!(f, "{}", n),
            BeamVal::Float(v) => write!(f, "{:.6}", v),
            BeamVal::Atom(a) => write!(f, "'{}'", a),
            BeamVal::Nil => write!(f, "[]"),
            BeamVal::Literal(idx) => write!(f, "literal({})", idx),
        }
    }
}
