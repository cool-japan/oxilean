//! # WidgetEventListener - Trait Implementations
//!
//! This module contains trait implementations for `WidgetEventListener`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WidgetEventListener;
use std::fmt;

impl std::fmt::Debug for WidgetEventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetEventListener")
            .field("name", &self.name)
            .field("kinds", &self.kinds)
            .finish()
    }
}
