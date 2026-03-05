//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ParseError;

use super::types::TacticExpr;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse `apply <ident>`
    pub(super) fn parse_apply(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let name = self.expect_ident()?;
        Ok(TacticExpr::Apply(name))
    }
    pub(super) fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }
}
