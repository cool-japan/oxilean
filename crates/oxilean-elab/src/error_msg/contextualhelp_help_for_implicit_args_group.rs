//! # ContextualHelp - help_for_implicit_args_group Methods
//!
//! This module contains method implementations for `ContextualHelp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::contextualhelp_type::ContextualHelp;
use super::functions::*;

impl ContextualHelp {
    /// Help for implicit arguments
    pub fn help_for_implicit_args() -> &'static str {
        "Implicit arguments are inferred automatically but can cause ambiguity.\n\
         \n\
         When inference fails:\n\
         1. Make arguments explicit: Use `{_}` in function calls\n\
         2. Add type annotations to help inference\n\
         3. Use `@` to provide implicit arguments explicitly\n\
         \n\
         Example: `func @α @β x` passes implicit arguments explicitly"
    }
}
