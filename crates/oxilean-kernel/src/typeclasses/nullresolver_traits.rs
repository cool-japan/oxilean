//! # NullResolver - Trait Implementations
//!
//! This module contains trait implementations for `NullResolver`.
//!
//! ## Implemented Traits
//!
//! - `InstanceResolver`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Name};

use super::functions::InstanceResolver;
use super::types::{Instance, NullResolver};

impl InstanceResolver for NullResolver {
    fn resolve(&self, _class: &Name, _ty: &Expr) -> Option<Instance> {
        None
    }
    fn name(&self) -> &'static str {
        "null"
    }
}
