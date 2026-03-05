//! # CommandParser - reset_group Methods
//!
//! This module contains method implementations for `CommandParser`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::commandparser_type::CommandParser;

impl CommandParser {
    /// Reset the parser.
    pub fn reset(&mut self) {
        self.pos = 0;
        self.tokens.clear();
    }
}
