//! # IdentityTransformExt - Trait Implementations
//!
//! This module contains trait implementations for `IdentityTransformExt`.
//!
//! ## Implemented Traits
//!
//! - `TreeTransformExt`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[allow(unused_imports)]
use crate::ast::{SimpleNodeKindExt, TreeNodeExt};

use super::functions::TreeTransformExt;
use super::types::IdentityTransformExt;

impl TreeTransformExt for IdentityTransformExt {
    fn transform(&mut self, node: TreeNodeExt) -> TreeNodeExt {
        let new_children = node
            .children
            .into_iter()
            .map(|c| self.transform(c))
            .collect();
        TreeNodeExt {
            kind: node.kind,
            label: node.label,
            children: new_children,
        }
    }
}
