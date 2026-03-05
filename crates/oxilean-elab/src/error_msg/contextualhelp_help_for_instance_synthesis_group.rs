//! # ContextualHelp - help_for_instance_synthesis_group Methods
//!
//! This module contains method implementations for `ContextualHelp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::contextualhelp_type::ContextualHelp;
use super::functions::*;

impl ContextualHelp {
    /// Help for instance synthesis
    pub fn help_for_instance_synthesis() -> &'static str {
        "Instance synthesis fails when Lean can't find a matching instance.\n\
         \n\
         Debugging:\n\
         1. Use `#synth` to check what instances are available\n\
         2. Ensure instances are in scope (imported or defined)\n\
         3. Check instance priorities (@[instance])\n\
         \n\
         Fix: Add explicit instance or adjust instance database"
    }
}
