//! # CommandParser - parsing Methods
//!
//! This module contains method implementations for `CommandParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{ParseError, TokenKind};

use super::types::Command;

use super::commandparser_type::CommandParser;

impl CommandParser {
    /// Advance the position by one token.
    pub(super) fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }
    /// If the current token matches `kind`, consume it and return true.
    pub(super) fn consume(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    /// Consume an identifier with a specific name.
    #[allow(dead_code)]
    pub(super) fn consume_ident(&mut self, name: &str) -> bool {
        if self.check_ident(name) {
            self.advance();
            true
        } else {
            false
        }
    }
    /// Parse `import <module>`.
    pub(super) fn parse_import(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let module = self.parse_dotted_name()?;
        let span = start_span.merge(&self.current_span());
        Ok(Command::Import { module, span })
    }
    /// Parse `namespace <name>`.
    pub(super) fn parse_namespace(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let name = self.parse_ident()?;
        let span = start_span.merge(&self.current_span());
        Ok(Command::Namespace { name, span })
    }
    /// Parse `end`.
    pub(super) fn parse_end(&mut self) -> Result<Command, ParseError> {
        let span = self.current_span();
        self.advance();
        Ok(Command::End { span })
    }
    /// Parse `section <name>`.
    pub(super) fn parse_section(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let name = self.parse_ident()?;
        let span = start_span.merge(&self.current_span());
        Ok(Command::Section { name, span })
    }
    /// Parse `set_option <name> <value>`.
    pub(super) fn parse_set_option(&mut self) -> Result<Command, ParseError> {
        let start_span = self.current_span();
        self.advance();
        let name = self.parse_dotted_name()?;
        let value = self.collect_rest_as_string();
        let span = start_span.merge(&self.current_span());
        Ok(Command::SetOption { name, value, span })
    }
}
