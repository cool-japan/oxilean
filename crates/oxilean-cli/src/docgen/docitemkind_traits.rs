//! # DocItemKind - Trait Implementations
//!
//! This module contains trait implementations for `DocItemKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DocItemKind;
use std::fmt;

impl fmt::Display for DocItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocItemKind::Module => write!(f, "module"),
            DocItemKind::Definition => write!(f, "definition"),
            DocItemKind::Theorem => write!(f, "theorem"),
            DocItemKind::Inductive => write!(f, "inductive"),
            DocItemKind::Structure => write!(f, "structure"),
            DocItemKind::Class => write!(f, "class"),
            DocItemKind::Instance => write!(f, "instance"),
            DocItemKind::Tactic => write!(f, "tactic"),
            DocItemKind::Axiom => write!(f, "axiom"),
        }
    }
}
