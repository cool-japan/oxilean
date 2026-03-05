//! # LimitTransform - Trait Implementations
//!
//! This module contains trait implementations for `LimitTransform`.
//!
//! ## Implemented Traits
//!
//! - `SuiteTransform`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::SuiteTransform;
use super::types::{DiffTestSuite, LimitTransform};
use std::fmt;

impl SuiteTransform for LimitTransform {
    fn transform(&self, mut suite: DiffTestSuite) -> DiffTestSuite {
        suite.cases.truncate(self.max);
        suite
    }
    fn name(&self) -> &str {
        "limit"
    }
}
