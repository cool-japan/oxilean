//! # CostModel - Trait Implementations
//!
//! This module contains trait implementations for `CostModel`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CostModel;

impl Default for CostModel {
    fn default() -> Self {
        CostModel {
            let_cost: 1,
            app_cost: 3,
            case_cost: 2,
            return_cost: 0,
            branch_penalty: 1,
        }
    }
}
