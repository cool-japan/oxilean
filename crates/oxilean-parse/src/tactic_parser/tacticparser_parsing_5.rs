//! # TacticParser - parsing Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{ParseError, TokenKind};

use super::types::{SimpArgs, TacticExpr};

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    /// Parse `simp`, `simp only [lem1, lem2]`, `simp [*]`, or `simp [lem1] {config}`
    pub(super) fn parse_simp(&mut self) -> Result<TacticExpr, ParseError> {
        self.advance();
        let only = if self.check_keyword("only") {
            self.advance();
            true
        } else {
            false
        };
        let lemmas = self.parse_simp_lemma_list()?;
        let config = self.parse_simp_config()?;
        Ok(TacticExpr::Simp(SimpArgs {
            only,
            lemmas,
            config,
        }))
    }
    pub(super) fn check_keyword(&self, kw: &str) -> bool {
        if let Some(token) = self.current() {
            matches!(& token.kind, TokenKind::Ident(s) if s == kw)
        } else {
            false
        }
    }
}
