//! # DecisionTree - Trait Implementations
//!
//! This module contains trait implementations for `DecisionTree`.
//!
//! ## Implemented Traits
//!
//! - `PartialEq`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DecisionTree;

impl PartialEq for DecisionTree {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                DecisionTree::Leaf {
                    arm_idx: a1,
                    bindings: b1,
                },
                DecisionTree::Leaf {
                    arm_idx: a2,
                    bindings: b2,
                },
            ) => a1 == a2 && b1.len() == b2.len(),
            (DecisionTree::Failure, DecisionTree::Failure) => true,
            (
                DecisionTree::Switch {
                    column: c1,
                    branches: b1,
                    ..
                },
                DecisionTree::Switch {
                    column: c2,
                    branches: b2,
                    ..
                },
            ) => c1 == c2 && b1.len() == b2.len(),
            _ => false,
        }
    }
}
