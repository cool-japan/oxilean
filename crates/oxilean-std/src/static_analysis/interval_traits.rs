//! # Interval - Trait Implementations
//!
//! This module contains trait implementations for `Interval`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Interval;
use std::fmt;

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Bottom => write!(f, "⊥"),
            Interval::Range(lo, hi) => {
                let l = if *lo == i64::MIN {
                    "-∞".to_string()
                } else {
                    lo.to_string()
                };
                let r = if *hi == i64::MAX {
                    "+∞".to_string()
                } else {
                    hi.to_string()
                };
                write!(f, "[{}, {}]", l, r)
            }
        }
    }
}
