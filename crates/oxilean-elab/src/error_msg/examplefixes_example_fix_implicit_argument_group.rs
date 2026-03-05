//! # ExampleFixes - example_fix_implicit_argument_group Methods
//!
//! This module contains method implementations for `ExampleFixes`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::examplefixes_type::ExampleFixes;
use super::functions::*;

impl ExampleFixes {
    /// Example: Fix implicit argument
    pub fn example_fix_implicit_argument() -> &'static str {
        "BEFORE:\n\
         def const (α : Type) (β : Type) (x : α) : β → α := fun _ => x\n\
         -- Usage fails due to ambiguity\n\
         \n\
         AFTER:\n\
         def const {α : Type} {β : Type} (x : α) : β → α := fun _ => x"
    }
}
