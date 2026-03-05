//! # RtObject - from_raw_bits_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::rtobject_type::RtObject;

impl RtObject {
    /// Create from raw bits (used for deserialization).
    pub fn from_raw_bits(bits: u64) -> Self {
        RtObject { bits }
    }
}
