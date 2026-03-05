//! # CommandParser - parsing Methods
//!
//! This module contains method implementations for `CommandParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{ParseError, Span, Token, TokenKind};

use super::types::Command;

use super::commandparser_type::CommandParser;

impl CommandParser {
    /// Get the current token (if any).
    pub(super) fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    /// Check if the current token matches a given kind (without consuming).
    pub(super) fn check(&self, kind: &TokenKind) -> bool {
        self.current().is_some_and(|t| &t.kind == kind)
    }
    /// Check if the current token is an identifier with a specific name.
    pub(super) fn check_ident(&self, name: &str) -> bool {
        self.current()
            .is_some_and(|t| matches!(& t.kind, TokenKind::Ident(s) if s == name))
    }
    /// Return the span of the current token, or a synthetic eof span.
    pub(super) fn current_span(&self) -> Span {
        self.current()
            .map(|t| t.span.clone())
            .unwrap_or_else(|| self.eof_span())
    }
    /// Parse `syntax <name> [<prec>] := <pattern>`.
    pub(super) fn parse_syntax(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let name = self.parse_ident()?;
        let prec = if let Some(token) = self.current() {
            if let TokenKind::Nat(n) = &token.kind {
                let p = *n as u32;
                self.advance();
                Some(p)
            } else {
                None
            }
        } else {
            None
        };
        self.expect(&TokenKind::Assign)?;
        let pattern = self.collect_rest_as_string();
        let span = start_span.merge(&self.current_span());
        Ok(Command::Syntax {
            name,
            prec,
            pattern,
            span,
        })
    }
    /// Helper: check if current is 'keyword'
    pub(super) fn check_keyword(&self, keyword: &str) -> bool {
        self.current()
            .is_some_and(|t| matches!(& t.kind, TokenKind::Ident(s) if s == keyword))
    }
    /// Helper: safely check if at end of token stream
    pub(super) fn at_end(&self) -> bool {
        self.current().is_none() || self.check(&TokenKind::Eof)
    }
}
