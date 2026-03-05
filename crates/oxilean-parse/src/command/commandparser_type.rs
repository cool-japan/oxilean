//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Token;

/// Command parser.
pub struct CommandParser {
    /// Current position
    pub(super) pos: usize,
    /// Token stream being parsed
    pub(super) tokens: Vec<Token>,
}
