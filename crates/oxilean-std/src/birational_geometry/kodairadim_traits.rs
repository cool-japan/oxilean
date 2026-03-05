//! # KodairaDim - Trait Implementations
//!
//! This module contains trait implementations for `KodairaDim`.
//!
//! ## Implemented Traits
//!
//! - `PartialOrd`
//! - `Ord`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::KodairaDim;
use std::fmt;

impl PartialOrd for KodairaDim {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KodairaDim {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (KodairaDim::NegInfinity, KodairaDim::NegInfinity) => std::cmp::Ordering::Equal,
            (KodairaDim::NegInfinity, _) => std::cmp::Ordering::Less,
            (_, KodairaDim::NegInfinity) => std::cmp::Ordering::Greater,
            (KodairaDim::Finite(a), KodairaDim::Finite(b)) => a.cmp(b),
        }
    }
}

impl std::fmt::Display for KodairaDim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KodairaDim::NegInfinity => write!(f, "-∞"),
            KodairaDim::Finite(k) => write!(f, "{}", k),
        }
    }
}
