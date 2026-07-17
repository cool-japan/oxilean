//! # Level - Trait Implementations
//!
//! This module contains trait implementations for `Level`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Level, LevelView};
use std::fmt;

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.view() {
            LevelView::Zero => write!(f, "0"),
            LevelView::Succ(l) => {
                if let Some(n) = self.to_nat() {
                    write!(f, "{}", n)
                } else {
                    write!(f, "({} + 1)", l)
                }
            }
            LevelView::Max(l1, l2) => write!(f, "max({}, {})", l1, l2),
            LevelView::IMax(l1, l2) => write!(f, "imax({}, {})", l1, l2),
            LevelView::Param(n) => write!(f, "{}", n),
            LevelView::MVar(id) => write!(f, "?u_{}", id.0),
        }
    }
}
