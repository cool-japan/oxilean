//! # ContextualHelp - help_for_universe_level_group Methods
//!
//! This module contains method implementations for `ContextualHelp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::contextualhelp_type::ContextualHelp;
use super::functions::*;

impl ContextualHelp {
    /// Help for universe level errors
    pub fn help_for_universe_level() -> &'static str {
        "Universe level constraints ensure type safety and predicativity.\n\
         When you get 'universe level too small':\n\
         1. A type is used in a universe that's too low\n\
         2. Polymorphism may help\n\
         \n\
         Solutions:\n\
         - Use `@u` to make definitions universe polymorphic\n\
         - Increase the universe level in declarations\n\
         - Check if you're using impredicative definitions"
    }
}
