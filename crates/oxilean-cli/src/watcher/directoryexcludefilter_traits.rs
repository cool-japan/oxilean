//! # DirectoryExcludeFilter - Trait Implementations
//!
//! This module contains trait implementations for `DirectoryExcludeFilter`.
//!
//! ## Implemented Traits
//!
//! - `WatchEventFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::path::{Path, PathBuf};

use super::functions::WatchEventFilter;
use super::types::{DirectoryExcludeFilter, WatchEventKind};

impl WatchEventFilter for DirectoryExcludeFilter {
    fn accepts(&self, path: &Path, _kind: WatchEventKind) -> bool {
        for component in path.components() {
            let name = component.as_os_str().to_string_lossy();
            if self.excluded.iter().any(|e| e == name.as_ref()) {
                return false;
            }
        }
        true
    }
}
