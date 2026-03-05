//! # CommandParser - parsing Methods
//!
//! This module contains method implementations for `CommandParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{ParseError, TokenKind};

use super::commandparser_type::CommandParser;

impl CommandParser {
    /// Parse a dotted path like `Foo.Bar.Baz` and return as separate segments.
    pub(super) fn parse_dotted_path(&mut self) -> Result<Vec<String>, ParseError> {
        let first = self.parse_ident()?;
        let mut parts = vec![first];
        while self.consume(&TokenKind::Dot) {
            parts.push(self.parse_ident()?);
        }
        Ok(parts)
    }
}
