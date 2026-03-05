//! # RtObject - from_heap_with_tag_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::rtobject_type::RtObject;
use super::types::{HeapObject, ObjectStore};

impl RtObject {
    /// Create a tagged reference to a heap object.
    pub(super) fn from_heap_with_tag(obj: HeapObject, tag: u8) -> Self {
        let id = ObjectStore::global_store(|store| store.allocate(obj));
        RtObject {
            bits: ((tag as u64) << 56) | (id as u64),
        }
    }
}
