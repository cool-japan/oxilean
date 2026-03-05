//! # TacticParser - check_methods Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::TokenKind;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    pub(super) fn check_op(&self, op: &str) -> bool {
        if let Some(token) = self.current() {
            match &token.kind {
                TokenKind::LParen if op == "(" => true,
                TokenKind::RParen if op == ")" => true,
                TokenKind::LBrace if op == "{" => true,
                TokenKind::RBrace if op == "}" => true,
                TokenKind::LBracket if op == "[" => true,
                TokenKind::RBracket if op == "]" => true,
                TokenKind::Comma if op == "," => true,
                TokenKind::Semicolon if op == ";" => true,
                TokenKind::Colon if op == ":" => true,
                TokenKind::Assign if op == ":=" => true,
                TokenKind::Arrow if op == "=>" => true,
                TokenKind::Bar if op == "|" => true,
                TokenKind::Underscore if op == "_" => true,
                TokenKind::Star if op == "*" => true,
                TokenKind::Minus if op == "-" => true,
                TokenKind::Ident(s) if s == op => true,
                _ => false,
            }
        } else {
            false
        }
    }
    pub(super) fn consume_op(&mut self, op: &str) -> bool {
        if self.check_op(op) {
            self.advance();
            true
        } else {
            false
        }
    }
}
