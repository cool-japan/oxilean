//! # ChangeKind - Trait Implementations
//!
//! This module contains trait implementations for `ChangeKind`.
//!
//! ## Implemented Traits
//!
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ChangeKind, DiffLineKind};
use std::fmt;

impl From<DiffLineKind> for ChangeKind {
    fn from(k: DiffLineKind) -> Self {
        match k {
            DiffLineKind::Added => ChangeKind::Added,
            DiffLineKind::Removed => ChangeKind::Removed,
            DiffLineKind::Context => ChangeKind::Unchanged,
        }
    }
}
