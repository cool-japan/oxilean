//! # TableOracle - Trait Implementations
//!
//! This module contains trait implementations for `TableOracle`.
//!
//! ## Implemented Traits
//!
//! - `Oracle`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::Oracle;
use super::types::TableOracle;

impl Oracle for TableOracle {
    fn query(&self, input: &[String]) -> Option<String> {
        self.table.get(input).cloned()
    }
}
