//! # OpenItem - Trait Implementations
//!
//! This module contains trait implementations for `OpenItem`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OpenItem;

impl std::fmt::Display for OpenItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenItem::All => write!(f, "*"),
            OpenItem::Only(names) => write!(f, "only [{}]", names.join(", ")),
            OpenItem::Hiding(names) => write!(f, "hiding [{}]", names.join(", ")),
            OpenItem::Renaming(old, new) => write!(f, "{} -> {}", old, new),
        }
    }
}
