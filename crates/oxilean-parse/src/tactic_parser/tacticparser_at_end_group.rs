//! # TacticParser - at_end_group Methods
//!
//! This module contains method implementations for `TacticParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::TokenKind;

use super::tacticparser_type::TacticParser;

impl<'a> TacticParser<'a> {
    pub(super) fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
            || self
                .current()
                .map_or(true, |t| matches!(t.kind, TokenKind::Eof))
    }
}
