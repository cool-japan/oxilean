//! # DeduplicateTransform - Trait Implementations
//!
//! This module contains trait implementations for `DeduplicateTransform`.
//!
//! ## Implemented Traits
//!
//! - `SuiteTransform`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::SuiteTransform;
use super::types::{DeduplicateTransform, DiffTestSuite};
use std::fmt;

impl SuiteTransform for DeduplicateTransform {
    fn transform(&self, suite: DiffTestSuite) -> DiffTestSuite {
        let mut seen = std::collections::HashSet::new();
        let cases: Vec<_> = suite
            .cases
            .into_iter()
            .filter(|c| seen.insert(c.name.clone()))
            .collect();
        DiffTestSuite {
            cases,
            name: suite.name,
        }
    }
    fn name(&self) -> &str {
        "deduplicate"
    }
}
