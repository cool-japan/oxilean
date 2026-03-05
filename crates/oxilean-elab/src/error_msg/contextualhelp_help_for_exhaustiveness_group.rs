//! # ContextualHelp - help_for_exhaustiveness_group Methods
//!
//! This module contains method implementations for `ContextualHelp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::contextualhelp_type::ContextualHelp;
use super::functions::*;

impl ContextualHelp {
    /// Help for exhaustiveness checking
    pub fn help_for_exhaustiveness() -> &'static str {
        "Pattern matching must cover all possible constructors.\n\
         \n\
         When you see 'non-exhaustive patterns':\n\
         1. Add missing cases\n\
         2. Use `_` for catch-all case\n\
         3. Use `match` instead of pattern matching if conditional\n\
         \n\
         Tip: Lean will suggest missing patterns in the error message"
    }
}
