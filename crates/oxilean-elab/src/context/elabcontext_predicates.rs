//! # ElabContext - predicates Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::types::LocalEntry;

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Look up a local variable by name.
    pub fn lookup_local(&self, name: &Name) -> Option<&LocalEntry> {
        self.locals.iter().rev().find(|e| &e.name == name)
    }
    /// Check whether a name is locally bound.
    pub fn has_local(&self, name: &Name) -> bool {
        self.lookup_local(name).is_some()
    }
}
