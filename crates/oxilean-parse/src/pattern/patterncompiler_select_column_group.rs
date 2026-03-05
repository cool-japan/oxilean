//! # PatternCompiler - select_column_group Methods
//!
//! This module contains method implementations for `PatternCompiler`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Pattern;

use super::types::PatternRow;

use super::patterncompiler_type::PatternCompiler;

impl PatternCompiler {
    /// Select the best column to split on.
    ///
    /// Uses a simple heuristic: pick the column with the most constructor
    /// patterns. This tends to produce smaller decision trees.
    #[allow(dead_code)]
    pub fn select_column(&self, rows: &[PatternRow], num_cols: usize) -> usize {
        let mut best_col = 0;
        let mut best_score: usize = 0;
        for col in 0..num_cols {
            let mut score: usize = 0;
            for row in rows {
                if col < row.patterns.len() {
                    match &row.patterns[col] {
                        Pattern::Ctor(_, _) | Pattern::Lit(_) | Pattern::Or(_, _) => {
                            score += 1;
                        }
                        Pattern::Wild | Pattern::Var(_) => {}
                    }
                }
            }
            if score > best_score {
                best_score = score;
                best_col = col;
            }
        }
        best_col
    }
}
