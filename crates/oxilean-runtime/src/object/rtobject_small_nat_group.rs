//! # RtObject - small_nat_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{MAX_SMALL_NAT, TAG_SMALL_NAT};
use super::rtobject_type::RtObject;

impl RtObject {
    /// Create a small natural number (inline).
    ///
    /// Returns `None` if the value exceeds 56 bits.
    pub fn small_nat(n: u64) -> Option<Self> {
        if n > MAX_SMALL_NAT {
            return None;
        }
        Some(RtObject {
            bits: ((TAG_SMALL_NAT as u64) << 56) | n,
        })
    }
    /// Create a natural number, using small representation if possible.
    pub fn nat(n: u64) -> Self {
        RtObject::small_nat(n).unwrap_or_else(|| RtObject::big_nat(n))
    }
}
