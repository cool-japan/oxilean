//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Token;

/// Parser for tactic expressions.
pub struct TacticParser<'a> {
    pub(super) tokens: &'a [Token],
    pub(super) pos: usize,
}
