//! # RtObject - float_bits_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{PAYLOAD_MASK, TAG_FLOAT_BITS};
use super::rtobject_type::RtObject;

impl RtObject {
    /// Create a float from reduced-precision bits.
    pub fn float_bits(bits: u64) -> Self {
        RtObject {
            bits: ((TAG_FLOAT_BITS as u64) << 56) | (bits & PAYLOAD_MASK),
        }
    }
}
