//! # PatternCompiler - analyze_usefulness_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::types::PatternRow;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Analyze pattern usefulness using basic algorithm.
    pub fn analyze_usefulness(&self, rows: &[PatternRow], new_pattern: &[Pattern]) -> bool {
        rows.is_empty() || !self.subsumes_all(rows, new_pattern)
    }
}
