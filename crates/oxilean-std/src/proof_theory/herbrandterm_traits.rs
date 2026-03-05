//! # HerbrandTerm - Trait Implementations
//!
//! This module contains trait implementations for `HerbrandTerm`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HerbrandTerm;
use std::fmt;

impl std::fmt::Display for HerbrandTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HerbrandTerm::Const(c) => write!(f, "{}", c),
            HerbrandTerm::Fun(name, args) => {
                let arg_strs: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", name, arg_strs.join(", "))
            }
        }
    }
}
