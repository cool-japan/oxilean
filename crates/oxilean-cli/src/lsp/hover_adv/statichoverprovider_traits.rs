//! # StaticHoverProvider - Trait Implementations
//!
//! This module contains trait implementations for `StaticHoverProvider`.
//!
//! ## Implemented Traits
//!
//! - `HoverInfoProvider`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::HoverInfoProvider;
use super::types::{HoverInfo, StaticHoverProvider};
use std::fmt;

impl HoverInfoProvider for StaticHoverProvider {
    fn hover_for(&self, name: &str) -> Option<HoverInfo> {
        self.data.get(name).cloned()
    }
}
