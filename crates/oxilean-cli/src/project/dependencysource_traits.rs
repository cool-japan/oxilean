//! # DependencySource - Trait Implementations
//!
//! This module contains trait implementations for `DependencySource`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::path::{Path, PathBuf};

use super::types::DependencySource;

impl fmt::Display for DependencySource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependencySource::Path(p) => write!(f, "path \"{}\"", p.display()),
            DependencySource::Git { url, rev } => {
                write!(f, "git \"{}\"", url)?;
                if let Some(r) = rev {
                    write!(f, " rev \"{}\"", r)?;
                }
                Ok(())
            }
            DependencySource::Registry { registry } => {
                write!(f, "registry \"{}\"", registry)
            }
        }
    }
}
