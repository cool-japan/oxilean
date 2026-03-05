//! # ExampleFixes - example_fix_universe_level_group Methods
//!
//! This module contains method implementations for `ExampleFixes`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::examplefixes_type::ExampleFixes;
use super::functions::*;

impl ExampleFixes {
    /// Example: Fix universe level
    pub fn example_fix_universe_level() -> &'static str {
        "BEFORE:\n\
         def id (α : Type) : α → α := fun x => x\n\
         -- Error: α is in Type 0 but needs flexibility\n\
         \n\
         AFTER:\n\
         def id {α : Type u} : α → α := fun x => x"
    }
}
