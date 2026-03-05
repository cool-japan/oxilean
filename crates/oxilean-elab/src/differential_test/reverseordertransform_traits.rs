//! # ReverseOrderTransform - Trait Implementations
//!
//! This module contains trait implementations for `ReverseOrderTransform`.
//!
//! ## Implemented Traits
//!
//! - `SuiteTransform`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::SuiteTransform;
use super::types::{DiffTestSuite, ReverseOrderTransform};
use std::fmt;

impl SuiteTransform for ReverseOrderTransform {
    fn transform(&self, mut suite: DiffTestSuite) -> DiffTestSuite {
        suite.cases.reverse();
        suite
    }
    fn name(&self) -> &str {
        "reverse_order"
    }
}
