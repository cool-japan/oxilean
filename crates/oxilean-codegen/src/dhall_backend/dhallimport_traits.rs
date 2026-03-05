//! # DhallImport - Trait Implementations
//!
//! This module contains trait implementations for `DhallImport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DhallImport;
use std::fmt;

impl fmt::Display for DhallImport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DhallImport::Local(path) => write!(f, "{}", path),
            DhallImport::Remote(url) => write!(f, "{}", url),
            DhallImport::Env(var) => write!(f, "env:{}", var),
            DhallImport::Missing => write!(f, "missing"),
            DhallImport::Hashed(imp, hash) => write!(f, "{} sha256:{}", imp, hash),
            DhallImport::Fallback(a, b) => write!(f, "{} ? {}", a, b),
        }
    }
}
