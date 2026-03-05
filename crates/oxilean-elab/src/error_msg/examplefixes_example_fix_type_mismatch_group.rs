//! # ExampleFixes - example_fix_type_mismatch_group Methods
//!
//! This module contains method implementations for `ExampleFixes`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::examplefixes_type::ExampleFixes;
use super::functions::*;

impl ExampleFixes {
    /// Example: Fix type mismatch
    pub fn example_fix_type_mismatch() -> &'static str {
        "BEFORE:\n\
         def f : Nat := \"hello\"\n\
         \n\
         AFTER:\n\
         def f : String := \"hello\"\n\
         \n\
         OR:\n\
         def f : Nat := 42"
    }
}
