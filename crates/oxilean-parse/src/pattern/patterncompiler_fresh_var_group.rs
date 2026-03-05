//! # PatternCompiler - fresh_var_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Name;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Generate a fresh variable name.
    pub fn fresh_var(&mut self) -> Name {
        let var = Name::str(format!("_x{}", self.next_var));
        self.next_var += 1;
        var
    }
}
