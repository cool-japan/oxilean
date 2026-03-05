//! # RtObject - raw_bits_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::rtobject_type::RtObject;

impl RtObject {
    /// Get the raw bits.
    pub fn raw_bits(&self) -> u64 {
        self.bits
    }
}
