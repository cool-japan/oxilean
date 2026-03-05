//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ParseError;

use super::types::TacticExpr;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse `generalize h : expr`.
    #[allow(dead_code)]
    pub(super) fn parse_generalize(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let name = self.expect_ident()?;
        self.expect_op(":")?;
        let expr = self.expect_ident()?;
        Ok(TacticExpr::Generalize(name, expr))
    }
}
