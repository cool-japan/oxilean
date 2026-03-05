//! # BeamReg - Trait Implementations
//!
//! This module contains trait implementations for `BeamReg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::BeamReg;
use std::fmt;

impl fmt::Display for BeamReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeamReg::X(n) => write!(f, "x({})", n),
            BeamReg::Y(n) => write!(f, "y({})", n),
            BeamReg::FR(n) => write!(f, "fr({})", n),
        }
    }
}
