//! # MetaContext - accessors Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Name};

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get all local hypotheses as (name, type) pairs.
    pub fn get_local_hyps(&self) -> Vec<(Name, Expr)> {
        self.local_decls
            .iter()
            .map(|d| (d.user_name.clone(), d.ty.clone()))
            .collect()
    }
}
