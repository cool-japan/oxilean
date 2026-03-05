//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ParseError;

use super::types::TacticExpr;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse `rewrite [lem1, <-lem2, lem3]` or `rw [...]`
    pub(super) fn parse_rewrite(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let rules = self.parse_rewrite_args()?;
        Ok(TacticExpr::Rewrite(rules))
    }
}
