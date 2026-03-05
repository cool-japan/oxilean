//! # CompletionKind - Trait Implementations
//!
//! This module contains trait implementations for `CompletionKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CompletionKind;
use std::fmt;

impl fmt::Display for CompletionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompletionKind::Function => write!(f, "function"),
            CompletionKind::Type => write!(f, "type"),
            CompletionKind::Tactic => write!(f, "tactic"),
            CompletionKind::Variable => write!(f, "variable"),
            CompletionKind::Namespace => write!(f, "namespace"),
            CompletionKind::Keyword => write!(f, "keyword"),
            CompletionKind::Field => write!(f, "field"),
            CompletionKind::Constructor => write!(f, "constructor"),
            CompletionKind::Theorem => write!(f, "theorem"),
            CompletionKind::Attribute => write!(f, "attribute"),
            CompletionKind::Option => write!(f, "option"),
        }
    }
}
