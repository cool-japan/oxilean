//! # StackValue - Trait Implementations
//!
//! This module contains trait implementations for `StackValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StackValue;
use std::fmt;

impl std::fmt::Display for StackValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::Nat(n) => write!(f, "{}", n),
            StackValue::Int(i) => write!(f, "{}", i),
            StackValue::Bool(b) => write!(f, "{}", b),
            StackValue::Str(s) => write!(f, "{:?}", s),
            StackValue::Closure { .. } => write!(f, "<closure>"),
            StackValue::Nil => write!(f, "nil"),
        }
    }
}
