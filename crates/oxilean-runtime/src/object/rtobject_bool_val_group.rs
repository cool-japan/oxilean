//! # RtObject - bool_val_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TAG_BOOL;
use super::rtobject_type::RtObject;

impl RtObject {
    /// Create a boolean value.
    pub fn bool_val(b: bool) -> Self {
        let payload = if b { 1u64 } else { 0u64 };
        RtObject {
            bits: ((TAG_BOOL as u64) << 56) | payload,
        }
    }
}
