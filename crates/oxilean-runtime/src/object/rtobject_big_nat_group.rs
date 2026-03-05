//! # RtObject - big_nat_group Methods
//!
//! This module contains method implementations for `RtObject`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::TAG_HEAP;
use super::rtobject_type::RtObject;
use super::types::{BigNatData, HeapObject, ObjectHeader, ObjectStore, TypeTag};

impl RtObject {
    /// Create a big natural number (heap-allocated).
    pub(super) fn big_nat(n: u64) -> Self {
        let heap = HeapObject::BigNat(BigNatData {
            header: ObjectHeader::new(TypeTag::BigNat, 2),
            digits: vec![n],
        });
        RtObject::from_heap(heap)
    }
    /// Create a heap-allocated object reference.
    pub(super) fn from_heap(obj: HeapObject) -> Self {
        let id = ObjectStore::global_store(|store| store.allocate(obj));
        RtObject {
            bits: ((TAG_HEAP as u64) << 56) | (id as u64),
        }
    }
}
