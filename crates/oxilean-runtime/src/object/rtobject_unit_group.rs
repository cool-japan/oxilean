//! # RtObject - unit_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TAG_UNIT;
use super::rtobject_type::RtObject;

impl RtObject {
    /// Create a unit value.
    pub fn unit() -> Self {
        RtObject {
            bits: (TAG_UNIT as u64) << 56,
        }
    }
}
