//! # RenameTransformExt - Trait Implementations
//!
//! This module contains trait implementations for `RenameTransformExt`.
//!
//! ## Implemented Traits
//!
//! - `TreeTransformExt`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[allow(unused_imports)]
use crate::ast::{SimpleNodeKindExt, TreeNodeExt};

use super::functions::TreeTransformExt;
use super::types::RenameTransformExt;

impl TreeTransformExt for RenameTransformExt {
    fn transform(&mut self, node: TreeNodeExt) -> TreeNodeExt {
        if node.kind == SimpleNodeKindExt::Leaf && node.label == self.from {
            self.count += 1;
            return TreeNodeExt::leaf(&self.to.clone());
        }
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
