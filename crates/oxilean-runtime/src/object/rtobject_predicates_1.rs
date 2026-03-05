//! # RtObject - predicates Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::rtobject_type::RtObject;

impl RtObject {
    /// Check if this is a heap-allocated object.
    pub fn is_heap(&self) -> bool {
        !self.is_inline()
    }
}
