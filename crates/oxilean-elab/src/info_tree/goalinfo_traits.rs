//! # GoalInfo - Trait Implementations
//!
//! This module contains trait implementations for `GoalInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GoalInfo;
use std::fmt;

impl fmt::Display for GoalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "case {} ", name)?;
        }
        for (h_name, h_ty) in &self.hypotheses {
            write!(f, "{} : {:?}, ", h_name, h_ty)?;
        }
        write!(f, "|- {:?}", self.target)
    }
}
