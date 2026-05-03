//! # WorkspaceInfo - Trait Implementations
//!
//! This module contains trait implementations for `WorkspaceInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WorkspaceInfo;
use std::fmt;

impl std::fmt::Display for WorkspaceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Workspace({}, {} members)",
            self.name,
            self.members.len()
        )
    }
}
