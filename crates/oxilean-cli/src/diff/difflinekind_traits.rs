//! # DiffLineKind - Trait Implementations
//!
//! This module contains trait implementations for `DiffLineKind`.
//!
//! ## Implemented Traits
//!
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ChangeKind, DiffLineKind};
use std::fmt;

impl From<ChangeKind> for DiffLineKind {
    fn from(k: ChangeKind) -> Self {
        match k {
            ChangeKind::Added => DiffLineKind::Added,
            ChangeKind::Removed => DiffLineKind::Removed,
            ChangeKind::Unchanged => DiffLineKind::Context,
        }
    }
}
