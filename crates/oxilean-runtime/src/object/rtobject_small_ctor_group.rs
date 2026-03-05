//! # RtObject - small_ctor_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TAG_CTOR;
use super::rtobject_type::RtObject;

impl RtObject {
    /// Create a small constructor tag.
    ///
    /// For inductives with no fields and a small number of constructors,
    /// we can encode the constructor index inline.
    pub fn small_ctor(index: u32) -> Self {
        RtObject {
            bits: ((TAG_CTOR as u64) << 56) | (index as u64),
        }
    }
}
