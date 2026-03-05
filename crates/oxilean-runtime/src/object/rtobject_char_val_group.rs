//! # RtObject - char_val_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TAG_CHAR;
use super::rtobject_type::RtObject;

impl RtObject {
    /// Create a character value.
    pub fn char_val(c: char) -> Self {
        let payload = c as u64;
        RtObject {
            bits: ((TAG_CHAR as u64) << 56) | payload,
        }
    }
}
