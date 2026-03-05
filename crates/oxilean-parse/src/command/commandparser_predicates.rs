//! # CommandParser - predicates Methods
//!
//! This module contains method implementations for `CommandParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::TokenKind;

use super::commandparser_type::CommandParser;

impl CommandParser {
    /// Check if a token is a command keyword.
    pub fn is_command_keyword(token: &TokenKind) -> bool {
        matches!(
            token,
            TokenKind::Axiom
                | TokenKind::Definition
                | TokenKind::Theorem
                | TokenKind::Lemma
                | TokenKind::Inductive
                | TokenKind::Structure
                | TokenKind::Class
                | TokenKind::Instance
                | TokenKind::Opaque
                | TokenKind::Constant
                | TokenKind::Constants
                | TokenKind::Namespace
                | TokenKind::Section
                | TokenKind::Variable
                | TokenKind::Variables
                | TokenKind::Parameter
                | TokenKind::Parameters
                | TokenKind::End
                | TokenKind::Import
                | TokenKind::Export
                | TokenKind::Open
                | TokenKind::Attribute
                | TokenKind::Hash
        ) || matches!(
            token, TokenKind::Ident(s) if matches!(s.as_str(), "set_option" |
            "universe" | "universes" | "notation" | "prefix" | "infix" | "infixl" |
            "infixr" | "postfix" | "derive" | "deriving" | "syntax" | "precedence")
        )
    }
}
