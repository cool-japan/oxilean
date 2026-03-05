//! # DeclKind - Trait Implementations
//!
//! This module contains trait implementations for `DeclKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::types::DeclKind;

impl fmt::Display for DeclKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeclKind::Definition => write!(f, "definition"),
            DeclKind::Theorem => write!(f, "theorem"),
            DeclKind::Inductive => write!(f, "inductive"),
            DeclKind::Structure => write!(f, "structure"),
            DeclKind::Class => write!(f, "class"),
            DeclKind::Instance => write!(f, "instance"),
            DeclKind::Axiom => write!(f, "axiom"),
            DeclKind::Variable => write!(f, "variable"),
            DeclKind::Namespace => write!(f, "namespace"),
        }
    }
}
