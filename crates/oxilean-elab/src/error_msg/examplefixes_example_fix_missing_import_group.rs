//! # ExampleFixes - example_fix_missing_import_group Methods
//!
//! This module contains method implementations for `ExampleFixes`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::examplefixes_type::ExampleFixes;
use super::functions::*;

impl ExampleFixes {
    /// Example: Fix missing import
    pub fn example_fix_missing_import() -> &'static str {
        "BEFORE:\n\
         def f := List.map (· + 1) [1,2,3]\n\
         -- Error: List not found\n\
         \n\
         AFTER:\n\
         import Data.List.Basic\n\
         \n\
         def f := List.map (· + 1) [1,2,3]"
    }
}
