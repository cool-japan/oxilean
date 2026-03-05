//! # KeywordFilter - Trait Implementations
//!
//! This module contains trait implementations for `KeywordFilter`.
//!
//! ## Implemented Traits
//!
//! - `DiffFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::DiffFilter;
use super::types::KeywordFilter;
use std::fmt;

#[allow(dead_code)]
impl DiffFilter for KeywordFilter {
    fn filter_lines<'a>(&self, lines: Vec<&'a str>) -> Vec<&'a str> {
        lines
            .into_iter()
            .filter(|l| l.contains(&self.keyword))
            .collect()
    }
}
