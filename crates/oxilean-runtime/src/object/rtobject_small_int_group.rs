//! # RtObject - small_int_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{MAX_SMALL_INT, MIN_SMALL_INT, PAYLOAD_MASK, TAG_INT};
use super::rtobject_type::RtObject;

impl RtObject {
    /// Create a small signed integer.
    pub fn small_int(n: i64) -> Option<Self> {
        if !(MIN_SMALL_INT..=MAX_SMALL_INT).contains(&n) {
            return None;
        }
        let payload = if n >= 0 {
            n as u64
        } else {
            (n as u64) & PAYLOAD_MASK
        };
        Some(RtObject {
            bits: ((TAG_INT as u64) << 56) | payload,
        })
    }
}
