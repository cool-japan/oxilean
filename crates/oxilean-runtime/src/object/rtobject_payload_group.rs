//! # RtObject - payload_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::PAYLOAD_MASK;

use super::rtobject_type::RtObject;

impl RtObject {
    /// Get the 56-bit payload.
    pub fn payload(&self) -> u64 {
        self.bits & PAYLOAD_MASK
    }
}
