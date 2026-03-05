//! # MetaVarContext - fresh_opaque_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;

use super::types::{MetaVar, MetaVarKind};

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Create a fresh synthetic-opaque metavariable (captures current scope).
    pub fn fresh_opaque(&mut self, ty: Expr) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let scope = self.current_scope.clone();
        self.metas.insert(
            id,
            MetaVar::with_kind(id, ty, MetaVarKind::SyntheticOpaque)
                .with_depth(self.depth)
                .with_scope(scope),
        );
        id
    }
}
