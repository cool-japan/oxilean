//! # NeronModel - Trait Implementations
//!
//! This module contains trait implementations for `NeronModel`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{NeronModel, NeronReductionType};
use std::fmt;

impl std::fmt::Display for NeronModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let red = match self.reduction_type {
            NeronReductionType::Good => "good",
            NeronReductionType::SemiStable => "semi-stable",
            NeronReductionType::PurelyToric => "purely toric",
            NeronReductionType::Additive => "additive",
        };
        write!(
            f,
            "Néron model of {}/{} over {} ({})",
            self.variety, self.fraction_field, self.dvr, red
        )
    }
}
