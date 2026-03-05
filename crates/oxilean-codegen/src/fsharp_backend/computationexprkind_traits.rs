//! # ComputationExprKind - Trait Implementations
//!
//! This module contains trait implementations for `ComputationExprKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ComputationExprKind;
use std::fmt;

impl fmt::Display for ComputationExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputationExprKind::Async => write!(f, "async"),
            ComputationExprKind::Seq => write!(f, "seq"),
            ComputationExprKind::Result => write!(f, "result"),
            ComputationExprKind::OptionCe => write!(f, "option"),
            ComputationExprKind::Custom(name) => write!(f, "{}", name),
        }
    }
}
