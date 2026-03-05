//! # WorkspaceConfig - Trait Implementations
//!
//! This module contains trait implementations for `WorkspaceConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WorkspaceConfig;
use std::fmt;

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            root_uri: None,
            root_path: None,
            workspace_folders: Vec::new(),
            max_diagnostics_per_file: 200,
            check_on_save: true,
            semantic_tokens_enabled: true,
            inlay_hints_enabled: true,
        }
    }
}
