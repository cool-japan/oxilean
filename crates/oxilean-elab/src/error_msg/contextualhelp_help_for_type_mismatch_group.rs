//! # ContextualHelp - help_for_type_mismatch_group Methods
//!
//! This module contains method implementations for `ContextualHelp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::contextualhelp_type::ContextualHelp;
use super::functions::*;

impl ContextualHelp {
    /// Help for type mismatch errors
    pub fn help_for_type_mismatch() -> &'static str {
        "Type mismatch occurs when the inferred type doesn't match the expected type.\n\
         Common causes:\n\
         1. Missing type annotation\n\
         2. Wrong function application\n\
         3. Incorrect argument type\n\
         4. Implicit argument mismatch\n\
         \n\
         Solutions:\n\
         - Add explicit type annotations: `(x : Nat) => ...`\n\
         - Check function signature: Use `:type` to inspect\n\
         - Use type coercion if applicable"
    }
}
