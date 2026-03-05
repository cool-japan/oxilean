//! # MetaVarSubst - Trait Implementations
//!
//! This module contains trait implementations for `MetaVarSubst`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaVarSubst;
use std::fmt;

impl Default for MetaVarSubst {
    fn default() -> Self {
        MetaVarSubst::new()
    }
}
