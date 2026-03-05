//! # FiniteMatrix - Trait Implementations
//!
//! This module contains trait implementations for `FiniteMatrix`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FiniteMatrix;
use std::fmt;

impl std::fmt::Display for FiniteMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix({}x{})", self.n, self.n)?;
        for i in 0..self.n {
            write!(f, "\n  [")?;
            for j in 0..self.n {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.4}", self.get(i, j))?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}
