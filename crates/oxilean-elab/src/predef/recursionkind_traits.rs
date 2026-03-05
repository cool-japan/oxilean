//! # RecursionKind - Trait Implementations
//!
//! This module contains trait implementations for `RecursionKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RecursionKind;
use std::fmt;

impl fmt::Display for RecursionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecursionKind::Structural(name) => {
                write!(f, "structural recursion on '{}'", name)
            }
            RecursionKind::WellFounded { rel, measure } => {
                write!(f, "well-founded recursion (rel: {:?}", rel)?;
                if let Some(m) = measure {
                    write!(f, ", measure: {:?}", m)?;
                }
                write!(f, ")")
            }
            RecursionKind::Mutual(names) => {
                write!(f, "mutual recursion: [")?;
                for (i, name) in names.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", name)?;
                }
                write!(f, "]")
            }
            RecursionKind::NonRecursive => write!(f, "non-recursive"),
        }
    }
}
