//! # NativeValue - Trait Implementations
//!
//! This module contains trait implementations for `NativeValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::NativeValue;
use std::fmt;

impl fmt::Display for NativeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NativeValue::Reg(r) => write!(f, "{}", r),
            NativeValue::Imm(n) => write!(f, "#{}", n),
            NativeValue::FRef(name) => write!(f, "@{}", name),
            NativeValue::StackSlot(slot) => write!(f, "ss{}", slot),
            NativeValue::UImm(n) => write!(f, "#{}u", n),
            NativeValue::StrRef(s) => write!(f, "{:?}", s),
        }
    }
}
