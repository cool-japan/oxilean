//! # VersionConstraint - Trait Implementations
//!
//! This module contains trait implementations for `VersionConstraint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VersionConstraint;
use std::fmt;

impl fmt::Display for VersionConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(v) => write!(f, "= {}", v),
            Self::GreaterEqual(v) => write!(f, ">= {}", v),
            Self::Greater(v) => write!(f, "> {}", v),
            Self::LessEqual(v) => write!(f, "<= {}", v),
            Self::Less(v) => write!(f, "< {}", v),
            Self::Caret(v) => write!(f, "^{}", v),
            Self::Tilde(v) => write!(f, "~{}", v),
            Self::Wildcard { major, minor } => match minor {
                Some(m) => write!(f, "{}.{}.*", major, m),
                None => write!(f, "{}.*", major),
            },
            Self::And(cs) => {
                for (i, c) in cs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", c)?;
                }
                Ok(())
            }
            Self::Or(cs) => {
                for (i, c) in cs.iter().enumerate() {
                    if i > 0 {
                        write!(f, " || ")?;
                    }
                    write!(f, "{}", c)?;
                }
                Ok(())
            }
            Self::Any => write!(f, "*"),
        }
    }
}
