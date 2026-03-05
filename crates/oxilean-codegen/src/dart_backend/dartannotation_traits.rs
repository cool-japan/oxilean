//! # DartAnnotation - Trait Implementations
//!
//! This module contains trait implementations for `DartAnnotation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_args, fmt_typed_params};
use super::types::DartAnnotation;
use std::fmt;

impl fmt::Display for DartAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DartAnnotation::Override => write!(f, "@override"),
            DartAnnotation::Deprecated => write!(f, "@deprecated"),
            DartAnnotation::VisibleForTesting => write!(f, "@visibleForTesting"),
            DartAnnotation::Immutable => write!(f, "@immutable"),
            DartAnnotation::Sealed => write!(f, "@sealed"),
            DartAnnotation::Custom(name, args) => {
                if args.is_empty() {
                    write!(f, "@{}", name)
                } else {
                    write!(f, "@{}({})", name, args.join(", "))
                }
            }
        }
    }
}
