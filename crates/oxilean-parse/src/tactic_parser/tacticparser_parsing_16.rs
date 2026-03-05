//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ParseError;

use super::types::TacticExpr;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse `ac_rfl`.
    #[allow(dead_code)]
    pub(super) fn parse_ac_rfl(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        Ok(TacticExpr::AcRfl)
    }
}
