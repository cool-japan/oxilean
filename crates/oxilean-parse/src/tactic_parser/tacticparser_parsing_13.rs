//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{ParseError, Token, TokenKind};

use super::types::TacticExpr;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse `by_contra` or `by_contra h`
    pub(super) fn parse_by_contra(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let name = if let Some(token) = self.current() {
            if let TokenKind::Ident(s) = &token.kind {
                if !self.is_tactic_terminator(s) {
                    let n = s.clone();
                    self.advance();
                    Some(n)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        Ok(TacticExpr::ByContra(name))
    }
    /// Check if an identifier is a tactic sequence terminator (`;`, `}`, etc.).
    pub(super) fn is_tactic_terminator(&self, _name: &str) -> bool {
        false
    }
    pub(super) fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
}
