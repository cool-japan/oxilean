//! # TraceCategory - Trait Implementations
//!
//! This module contains trait implementations for `TraceCategory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TraceCategory;
use std::fmt;

impl fmt::Display for TraceCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceCategory::Infer => write!(f, "infer"),
            TraceCategory::DefEq => write!(f, "def_eq"),
            TraceCategory::Reduce => write!(f, "reduce"),
            TraceCategory::Unify => write!(f, "unify"),
            TraceCategory::Tactic => write!(f, "tactic"),
            TraceCategory::Elab => write!(f, "elab"),
            TraceCategory::Typeclass => write!(f, "typeclass"),
            TraceCategory::Simp => write!(f, "simp"),
            TraceCategory::Custom(s) => write!(f, "{s}"),
        }
    }
}
