//! # HeapPredicate - Trait Implementations
//!
//! This module contains trait implementations for `HeapPredicate`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HeapPredicate;
use std::fmt;

impl std::fmt::Display for HeapPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeapPredicate::Emp => write!(f, "emp"),
            HeapPredicate::PointsTo(x, v) => write!(f, "{x} ↦ {v}"),
            HeapPredicate::Sep(p, q) => write!(f, "({p} * {q})"),
            HeapPredicate::Or(p, q) => write!(f, "({p} ∨ {q})"),
            HeapPredicate::Exists(x, body) => write!(f, "∃ {x}, {body}"),
        }
    }
}
