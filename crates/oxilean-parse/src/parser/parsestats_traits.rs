//! # ParseStats - Trait Implementations
//!
//! This module contains trait implementations for `ParseStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseStats;
use std::fmt;

impl fmt::Display for ParseStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ParseStats {{ files: {}, decls: {}, errors: {} }}",
            self.files_parsed, self.decls_parsed, self.errors_total
        )
    }
}
