//! # MutType - Trait Implementations
//!
//! This module contains trait implementations for `MutType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MutType;
use std::fmt;

impl std::fmt::Display for MutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.deref() {
            MutType::Free(i) => write!(f, "?α{}", i),
            MutType::Bound(_) => write!(f, "<bound>"),
            MutType::Base(s) => write!(f, "{}", s),
            MutType::Arrow(a, b) => write!(f, "({} → {})", a, b),
            MutType::App(c, args) => {
                if args.is_empty() {
                    write!(f, "{}", c)
                } else {
                    write!(
                        f,
                        "{}<{}>",
                        c,
                        args.iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
        }
    }
}
