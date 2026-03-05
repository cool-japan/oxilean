//! # InductionScheme - Trait Implementations
//!
//! This module contains trait implementations for `InductionScheme`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InductionScheme;
use std::fmt;

impl fmt::Display for InductionScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InductionScheme({}, major={}, params={}, indices={}, ctors=[{}])",
            self.recursor,
            self.major_idx,
            self.num_params,
            self.num_indices,
            self.minor_premises
                .iter()
                .map(|mp| format!("{}", mp.ctor_name))
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}
