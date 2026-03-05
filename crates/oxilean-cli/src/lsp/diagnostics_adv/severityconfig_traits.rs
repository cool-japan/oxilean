//! # SeverityConfig - Trait Implementations
//!
//! This module contains trait implementations for `SeverityConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use std::fmt;

use super::types::SeverityConfig;

impl Default for SeverityConfig {
    fn default() -> Self {
        Self {
            overrides: HashMap::new(),
            suppressed: HashSet::new(),
            warnings_as_errors: false,
            suppress_hints: false,
            max_errors_per_file: 100,
            max_warnings_per_file: 200,
        }
    }
}
