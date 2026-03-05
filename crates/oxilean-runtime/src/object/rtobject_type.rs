//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Name;

use super::functions::{TAG_ARRAY, TAG_EXTERNAL, TAG_IO_ACTION, TAG_STRING, TAG_TASK, TAG_THUNK};
use super::types::{
    ArrayData, BoxedFloatData, ByteArrayData, ConstructorData, ExternalData, HeapObject,
    IoActionData, IoActionKind, ObjectHeader, ObjectStore, StringData, TaskData, TaskState,
    ThunkData, ThunkState, TypeTag,
};

/// The core runtime value representation.
///
/// `RtObject` uses a tagged encoding to store small values inline
/// (in the u64 itself) and reference heap-allocated objects for
/// larger values. This avoids allocation for common cases like
/// small naturals, booleans, and unit.
#[derive(Clone)]
pub struct RtObject {
    /// The raw tagged bits.
    pub(super) bits: u64,
}
impl RtObject {
    /// Access the heap object for this reference (if heap-allocated).
    pub fn with_heap<R>(&self, f: impl FnOnce(&HeapObject) -> R) -> Option<R> {
        if self.is_inline() {
            return None;
        }
        let id = self.payload() as usize;
        ObjectStore::global_store(|store| store.get(id).map(f))
    }
    /// Access the heap object mutably.
    pub fn with_heap_mut<R>(&self, f: impl FnOnce(&mut HeapObject) -> R) -> Option<R> {
        if self.is_inline() {
            return None;
        }
        let id = self.payload() as usize;
        ObjectStore::global_store(|store| store.get_mut(id).map(f))
    }
    /// Create a string object.
    pub fn string(s: String) -> Self {
        let heap = HeapObject::StringObj(StringData {
            header: ObjectHeader::new(TypeTag::StringObj, ((s.len() + 7) / 8 + 1) as u16),
            value: s,
            cached_hash: None,
        });
        RtObject::from_heap_with_tag(heap, TAG_STRING)
    }
    /// Create an array object.
    pub fn array(elements: Vec<RtObject>) -> Self {
        let cap = elements.capacity();
        let heap = HeapObject::Array(ArrayData {
            header: ObjectHeader::new(TypeTag::Array, (elements.len() + 1) as u16),
            elements,
            capacity: cap,
        });
        RtObject::from_heap_with_tag(heap, TAG_ARRAY)
    }
    /// Create a constructor object with fields.
    pub fn constructor(ctor_index: u32, fields: Vec<RtObject>) -> Self {
        let num_fields = fields.len() as u16;
        let heap = HeapObject::Constructor(ConstructorData {
            header: ObjectHeader::new(TypeTag::Constructor, (fields.len() + 2) as u16),
            ctor_index,
            num_fields,
            scalar_fields: Vec::new(),
            object_fields: fields,
            name: None,
        });
        RtObject::from_heap(heap)
    }
    /// Create a named constructor object.
    pub fn named_constructor(name: Name, ctor_index: u32, fields: Vec<RtObject>) -> Self {
        let num_fields = fields.len() as u16;
        let heap = HeapObject::Constructor(ConstructorData {
            header: ObjectHeader::new(TypeTag::Constructor, (fields.len() + 2) as u16),
            ctor_index,
            num_fields,
            scalar_fields: Vec::new(),
            object_fields: fields,
            name: Some(name),
        });
        RtObject::from_heap(heap)
    }
    /// Create a thunk (lazy value).
    pub fn thunk(closure: RtObject) -> Self {
        let heap = HeapObject::Thunk(ThunkData {
            header: ObjectHeader::new(TypeTag::Thunk, 2),
            state: ThunkState::Unevaluated { closure },
        });
        RtObject::from_heap_with_tag(heap, TAG_THUNK)
    }
    /// Create an IO action.
    pub fn io_pure(value: RtObject) -> Self {
        let heap = HeapObject::IoAction(IoActionData {
            header: ObjectHeader::new(TypeTag::IoAction, 2),
            kind: IoActionKind::Pure(value),
        });
        RtObject::from_heap_with_tag(heap, TAG_IO_ACTION)
    }
    /// Create a task object.
    pub fn task(task_id: u64) -> Self {
        let heap = HeapObject::Task(TaskData {
            header: ObjectHeader::new(TypeTag::Task, 2),
            state: TaskState::Pending,
            task_id,
        });
        RtObject::from_heap_with_tag(heap, TAG_TASK)
    }
    /// Create an external object.
    pub fn external(type_name: String, payload: Vec<u8>) -> Self {
        let heap = HeapObject::External(ExternalData {
            header: ObjectHeader::new(TypeTag::External, 2),
            type_name,
            payload,
        });
        RtObject::from_heap_with_tag(heap, TAG_EXTERNAL)
    }
    /// Create a boxed float.
    pub fn boxed_float(value: f64) -> Self {
        let heap = HeapObject::BoxedFloat(BoxedFloatData {
            header: ObjectHeader::new(TypeTag::BoxedFloat, 2),
            value,
        });
        RtObject::from_heap(heap)
    }
    /// Create a byte array.
    pub fn byte_array(bytes: Vec<u8>) -> Self {
        let heap = HeapObject::ByteArray(ByteArrayData {
            header: ObjectHeader::new(TypeTag::ByteArray, ((bytes.len() + 7) / 8 + 1) as u16),
            bytes,
        });
        RtObject::from_heap(heap)
    }
}
