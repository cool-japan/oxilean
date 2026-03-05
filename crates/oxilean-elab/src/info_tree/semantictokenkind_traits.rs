//! # SemanticTokenKind - Trait Implementations
//!
//! This module contains trait implementations for `SemanticTokenKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SemanticTokenKind;
use std::fmt;

impl fmt::Display for SemanticTokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SemanticTokenKind::Keyword => "keyword",
            SemanticTokenKind::Type => "type",
            SemanticTokenKind::Function => "function",
            SemanticTokenKind::Variable => "variable",
            SemanticTokenKind::Constructor => "constructor",
            SemanticTokenKind::Tactic => "tactic",
            SemanticTokenKind::Number => "number",
            SemanticTokenKind::String => "string",
            SemanticTokenKind::Operator => "operator",
            SemanticTokenKind::Namespace => "namespace",
            SemanticTokenKind::Comment => "comment",
        };
        f.write_str(s)
    }
}
