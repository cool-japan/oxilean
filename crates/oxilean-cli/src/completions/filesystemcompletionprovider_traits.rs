//! # FileSystemCompletionProvider - Trait Implementations
//!
//! This module contains trait implementations for `FileSystemCompletionProvider`.
//!
//! ## Implemented Traits
//!
//! - `DynamicCompletionProvider`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::DynamicCompletionProvider;
use super::types::{CompletionCandidate, FileSystemCompletionProvider};
use std::fmt;

impl DynamicCompletionProvider for FileSystemCompletionProvider {
    fn handles_argument(&self) -> &str {
        &self.argument_name
    }
    fn candidates(&self, partial: &str) -> Vec<CompletionCandidate> {
        let dir = if partial.is_empty() {
            ".".to_string()
        } else if let Some(parent) = std::path::Path::new(partial).parent() {
            if parent == std::path::Path::new("") {
                ".".to_string()
            } else {
                parent.to_string_lossy().to_string()
            }
        } else {
            ".".to_string()
        };
        let Ok(entries) = std::fs::read_dir(&dir) else {
            return vec![];
        };
        let mut candidates = vec![];
        for entry in entries.flatten() {
            let p = entry.path();
            let name = p.to_string_lossy().to_string();
            if !name.starts_with(partial) {
                continue;
            }
            if !self.filter_extensions.is_empty() {
                if let Some(ext) = p.extension() {
                    let ext_str = ext.to_string_lossy();
                    if !self.filter_extensions.iter().any(|e| e == ext_str.as_ref()) {
                        continue;
                    }
                } else if !p.is_dir() {
                    continue;
                }
            }
            let desc = if p.is_dir() { "directory" } else { "file" };
            candidates.push(CompletionCandidate::new(name, desc));
        }
        candidates
    }
}
