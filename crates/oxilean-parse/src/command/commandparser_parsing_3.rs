//! # CommandParser - parsing Methods
//!
//! This module contains method implementations for `CommandParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{ParseError, TokenKind};

use super::commandparser_type::CommandParser;

impl CommandParser {
    /// Parse a dotted name like `Foo.Bar.Baz` and return it as a single string.
    pub(super) fn parse_dotted_name(&mut self) -> Result<String, ParseError> {
        let first = self.parse_ident()?;
        let mut result = first;
        while self.consume(&TokenKind::Dot) {
            let next = self.parse_ident()?;
            result.push('.');
            result.push_str(&next);
        }
        Ok(result)
    }
}
