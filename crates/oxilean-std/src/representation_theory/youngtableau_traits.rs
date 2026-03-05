//! # YoungTableau - Trait Implementations
//!
//! This module contains trait implementations for `YoungTableau`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::YoungTableau;
use std::fmt;

impl Default for YoungTableau {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for YoungTableau {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            let row_str: Vec<String> = row.iter().map(|x| x.to_string()).collect();
            writeln!(f, "[ {} ]", row_str.join(" "))?;
        }
        Ok(())
    }
}
