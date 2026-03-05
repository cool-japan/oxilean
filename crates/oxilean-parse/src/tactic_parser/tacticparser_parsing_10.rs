//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ParseError;

use super::types::TacticExpr;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse `clear h1 h2 ...`
    pub(super) fn parse_clear(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let names = self.parse_ident_list()?;
        Ok(TacticExpr::Clear(names))
    }
}
