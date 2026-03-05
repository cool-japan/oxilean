//! # ConvTarget - Trait Implementations
//!
//! This module contains trait implementations for `ConvTarget`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ConvTarget;
use std::fmt;

impl fmt::Display for ConvTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvTarget::Lhs => write!(f, "lhs"),
            ConvTarget::Rhs => write!(f, "rhs"),
            ConvTarget::Arg(n) => write!(f, "arg {}", n),
            ConvTarget::Fun => write!(f, "fun"),
            ConvTarget::Pattern(_) => write!(f, "pattern"),
            ConvTarget::Enter(dirs) => {
                write!(f, "enter [")?;
                for (i, d) in dirs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", d)?;
                }
                write!(f, "]")
            }
        }
    }
}
