//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ParseError;

use super::types::TacticExpr;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse `show T`.
    pub(super) fn parse_show(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let ty = self.expect_ident()?;
        Ok(TacticExpr::Show(ty))
    }
}
