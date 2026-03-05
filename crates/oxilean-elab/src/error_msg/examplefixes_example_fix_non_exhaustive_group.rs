//! # ExampleFixes - example_fix_non_exhaustive_group Methods
//!
//! This module contains method implementations for `ExampleFixes`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::examplefixes_type::ExampleFixes;
use super::functions::*;

impl ExampleFixes {
    /// Example: Fix non-exhaustive pattern
    pub fn example_fix_non_exhaustive() -> &'static str {
        "BEFORE:\n\
         def is_zero : Nat → Bool\n\
           | 0 => true\n\
         -- Error: Missing case for succ\n\
         \n\
         AFTER:\n\
         def is_zero : Nat → Bool\n\
           | 0 => true\n\
           | _ => false"
    }
}
