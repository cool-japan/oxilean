//! # ExtensionFilter - Trait Implementations
//!
//! This module contains trait implementations for `ExtensionFilter`.
//!
//! ## Implemented Traits
//!
//! - `WatchEventFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::path::{Path, PathBuf};

use super::functions::WatchEventFilter;
use super::types::{ExtensionFilter, WatchEventKind};

impl WatchEventFilter for ExtensionFilter {
    fn accepts(&self, path: &Path, _kind: WatchEventKind) -> bool {
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            self.extensions.iter().any(|e| e == ext_str.as_ref())
        } else {
            false
        }
    }
}
