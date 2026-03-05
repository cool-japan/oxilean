//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ParseError;

use super::types::TacticExpr;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse `let x := e`.
    pub(super) fn parse_let(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let name = self.expect_ident()?;
        self.expect_op(":=")?;
        let value = self.expect_ident()?;
        Ok(TacticExpr::Let(name, value))
    }
}
