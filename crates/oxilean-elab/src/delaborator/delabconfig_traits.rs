//! # DelabConfig - Trait Implementations
//!
//! This module contains trait implementations for `DelabConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use std::fmt;

use super::types::DelabConfig;

impl Default for DelabConfig {
    fn default() -> Self {
        Self {
            show_implicit: false,
            show_universes: false,
            use_notation: true,
            use_abbreviations: true,
            hide_proofs: false,
            max_depth: 100,
            use_unicode: true,
            show_binder_info: true,
            omit_redundant_types: true,
            name_overrides: HashMap::new(),
        }
    }
}
