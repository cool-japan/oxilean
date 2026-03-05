//! # AstNodeKind - Trait Implementations
//!
//! This module contains trait implementations for `AstNodeKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AstNodeKind;

impl std::fmt::Display for AstNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AstNodeKind::Var => "var",
            AstNodeKind::Lam => "lam",
            AstNodeKind::Pi => "pi",
            AstNodeKind::App => "app",
            AstNodeKind::Let => "let",
            AstNodeKind::NatLit => "nat-lit",
            AstNodeKind::StrLit => "str-lit",
            AstNodeKind::Sort => "sort",
            AstNodeKind::Hole => "hole",
            AstNodeKind::DefDecl => "def",
            AstNodeKind::TheoremDecl => "theorem",
            AstNodeKind::AxiomDecl => "axiom",
        };
        write!(f, "{}", s)
    }
}
