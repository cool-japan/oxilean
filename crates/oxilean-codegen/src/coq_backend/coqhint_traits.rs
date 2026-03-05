//! # CoqHint - Trait Implementations
//!
//! This module contains trait implementations for `CoqHint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqHint;
use std::fmt;

impl std::fmt::Display for CoqHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoqHint::Resolve(ls, db) => {
                let db_str = db
                    .as_deref()
                    .map(|d| format!(" : {}", d))
                    .unwrap_or_default();
                write!(f, "Hint Resolve {}{}.  ", ls.join(" "), db_str)
            }
            CoqHint::Rewrite(ls, db) => {
                let db_str = db
                    .as_deref()
                    .map(|d| format!(" : {}", d))
                    .unwrap_or_default();
                write!(f, "Hint Rewrite {}{}.  ", ls.join(" "), db_str)
            }
            CoqHint::Unfold(ls, db) => {
                let db_str = db
                    .as_deref()
                    .map(|d| format!(" : {}", d))
                    .unwrap_or_default();
                write!(f, "Hint Unfold {}{}.  ", ls.join(" "), db_str)
            }
            CoqHint::Immediate(ls, db) => {
                let db_str = db
                    .as_deref()
                    .map(|d| format!(" : {}", d))
                    .unwrap_or_default();
                write!(f, "Hint Immediate {}{}.  ", ls.join(" "), db_str)
            }
            CoqHint::Constructors(ls, db) => {
                let db_str = db
                    .as_deref()
                    .map(|d| format!(" : {}", d))
                    .unwrap_or_default();
                write!(f, "Hint Constructors {}{}.  ", ls.join(" "), db_str)
            }
            CoqHint::Extern(n, pat, tac) => {
                let pat_str = pat.as_deref().unwrap_or("_");
                write!(f, "Hint Extern {} ({}) => {}.  ", n, pat_str, tac)
            }
        }
    }
}
