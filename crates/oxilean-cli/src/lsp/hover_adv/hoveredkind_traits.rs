//! # HoveredKind - Trait Implementations
//!
//! This module contains trait implementations for `HoveredKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HoveredKind;
use std::fmt;

impl std::fmt::Display for HoveredKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoveredKind::Declaration => write!(f, "declaration"),
            HoveredKind::Tactic => write!(f, "tactic"),
            HoveredKind::Keyword => write!(f, "keyword"),
            HoveredKind::Literal => write!(f, "literal"),
            HoveredKind::TypeClass => write!(f, "typeclass"),
            HoveredKind::Instance => write!(f, "instance"),
            HoveredKind::Unknown => write!(f, "unknown"),
        }
    }
}
