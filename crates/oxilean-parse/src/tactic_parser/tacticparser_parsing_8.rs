//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ParseError;

use super::types::TacticExpr;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse a `calc` block.
    pub(super) fn parse_calc(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let steps = self.parse_calc_steps()?;
        Ok(TacticExpr::Calc(steps))
    }
}
