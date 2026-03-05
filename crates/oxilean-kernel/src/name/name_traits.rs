//! # Name - Trait Implementations
//!
//! This module contains trait implementations for `Name`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `PartialOrd`
//! - `Ord`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Name;

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Name::Anonymous => write!(f, "_"),
            Name::Str(parent, s) => {
                if parent.is_anonymous() {
                    write!(f, "{s}")
                } else {
                    write!(f, "{parent}.{s}")
                }
            }
            Name::Num(parent, n) => {
                if parent.is_anonymous() {
                    write!(f, "{n}")
                } else {
                    write!(f, "{parent}.{n}")
                }
            }
        }
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}
