//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Name;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::functions::BoxInto;
use super::rtobject_type::RtObject;

/// Data for a big integer.
#[derive(Clone, Debug)]
pub struct BigIntData {
    /// Object header.
    pub header: ObjectHeader,
    /// Sign: true = negative.
    pub negative: bool,
    /// Magnitude as limbs.
    pub digits: Vec<u64>,
}
/// Data for a string object.
#[derive(Clone, Debug)]
pub struct StringData {
    /// Object header.
    pub header: ObjectHeader,
    /// The string value.
    pub value: String,
    /// Cached hash.
    pub cached_hash: Option<u64>,
}
/// Data for a big natural number.
#[derive(Clone, Debug)]
pub struct BigNatData {
    /// Object header.
    pub header: ObjectHeader,
    /// Limbs (base-2^64 digits, least significant first).
    pub digits: Vec<u64>,
}
/// Thunk state.
#[derive(Clone, Debug)]
pub enum ThunkState {
    /// Thunk has not been evaluated.
    Unevaluated {
        /// Closure to evaluate.
        closure: RtObject,
    },
    /// Thunk is currently being evaluated (cycle detection).
    Evaluating,
    /// Thunk has been evaluated and the result is cached.
    Evaluated {
        /// The cached result.
        value: RtObject,
    },
    /// Thunk evaluation resulted in an exception.
    Excepted {
        /// The exception value.
        exception: RtObject,
    },
}
/// Data for a boxed float.
#[derive(Clone, Debug)]
pub struct BoxedFloatData {
    /// Object header.
    pub header: ObjectHeader,
    /// The float value.
    pub value: f64,
}
/// Data for a mutual recursive closure group.
#[derive(Clone, Debug)]
pub struct MutRecData {
    /// Object header.
    pub header: ObjectHeader,
    /// The closures in the mutual recursive group.
    pub closures: Vec<RtObject>,
    /// Which closure in the group this reference points to.
    pub index: u32,
}
/// Access fields of constructor objects.
pub struct FieldAccess;
impl FieldAccess {
    /// Get a field of a constructor object by index.
    pub fn get_field(obj: &RtObject, field_index: usize) -> Option<RtObject> {
        obj.with_heap(|heap| {
            if let HeapObject::Constructor(data) = heap {
                data.object_fields.get(field_index).cloned()
            } else {
                None
            }
        })
        .flatten()
    }
    /// Set a field of a constructor object by index (requires unique ownership).
    pub fn set_field(obj: &RtObject, field_index: usize, value: RtObject) -> bool {
        obj.with_heap_mut(|heap| {
            if let HeapObject::Constructor(data) = heap {
                if field_index < data.object_fields.len() {
                    data.object_fields[field_index] = value;
                    return true;
                }
            }
            false
        })
        .unwrap_or(false)
    }
    /// Get the constructor index of an object.
    pub fn get_ctor_index(obj: &RtObject) -> Option<u32> {
        if let Some(idx) = obj.as_small_ctor() {
            return Some(idx);
        }
        obj.with_heap(|heap| {
            if let HeapObject::Constructor(data) = heap {
                Some(data.ctor_index)
            } else {
                None
            }
        })
        .flatten()
    }
    /// Get the number of fields of a constructor object.
    pub fn num_fields(obj: &RtObject) -> Option<usize> {
        if obj.is_small_ctor() {
            return Some(0);
        }
        obj.with_heap(|heap| {
            if let HeapObject::Constructor(data) = heap {
                Some(data.object_fields.len())
            } else {
                None
            }
        })
        .flatten()
    }
    /// Get a scalar field of a constructor object.
    pub fn get_scalar_field(obj: &RtObject, field_index: usize) -> Option<u64> {
        obj.with_heap(|heap| {
            if let HeapObject::Constructor(data) = heap {
                data.scalar_fields.get(field_index).copied()
            } else {
                None
            }
        })
        .flatten()
    }
    /// Project a field from a Prod (pair) type.
    pub fn proj_fst(obj: &RtObject) -> Option<RtObject> {
        Self::get_field(obj, 0)
    }
    /// Project the second field from a Prod (pair) type.
    pub fn proj_snd(obj: &RtObject) -> Option<RtObject> {
        Self::get_field(obj, 1)
    }
}
/// A heap-allocated runtime object.
#[derive(Clone, Debug)]
pub enum HeapObject {
    /// A closure with captured environment.
    Closure(ClosureData),
    /// A constructor with fields.
    Constructor(ConstructorData),
    /// An array of runtime objects.
    Array(ArrayData),
    /// A string value.
    StringObj(StringData),
    /// A big natural number.
    BigNat(BigNatData),
    /// A big integer.
    BigInt(BigIntData),
    /// A thunk (lazy value).
    Thunk(ThunkData),
    /// An IO action.
    IoAction(IoActionData),
    /// A task (concurrent computation).
    Task(TaskData),
    /// An external/opaque value.
    External(ExternalData),
    /// A partial application.
    Pap(PapData),
    /// A mutual recursive closure group.
    MutRec(MutRecData),
    /// A boxed float.
    BoxedFloat(BoxedFloatData),
    /// A byte array.
    ByteArray(ByteArrayData),
}
impl HeapObject {
    /// Get the type tag for this heap object.
    pub fn type_tag(&self) -> TypeTag {
        match self {
            HeapObject::Closure(_) => TypeTag::Closure,
            HeapObject::Constructor(_) => TypeTag::Constructor,
            HeapObject::Array(_) => TypeTag::Array,
            HeapObject::StringObj(_) => TypeTag::StringObj,
            HeapObject::BigNat(_) => TypeTag::BigNat,
            HeapObject::BigInt(_) => TypeTag::BigInt,
            HeapObject::Thunk(_) => TypeTag::Thunk,
            HeapObject::IoAction(_) => TypeTag::IoAction,
            HeapObject::Task(_) => TypeTag::Task,
            HeapObject::External(_) => TypeTag::External,
            HeapObject::Pap(_) => TypeTag::Pap,
            HeapObject::MutRec(_) => TypeTag::MutRec,
            HeapObject::BoxedFloat(_) => TypeTag::BoxedFloat,
            HeapObject::ByteArray(_) => TypeTag::ByteArray,
        }
    }
    /// Get the object header.
    pub fn header(&self) -> &ObjectHeader {
        match self {
            HeapObject::Closure(d) => &d.header,
            HeapObject::Constructor(d) => &d.header,
            HeapObject::Array(d) => &d.header,
            HeapObject::StringObj(d) => &d.header,
            HeapObject::BigNat(d) => &d.header,
            HeapObject::BigInt(d) => &d.header,
            HeapObject::Thunk(d) => &d.header,
            HeapObject::IoAction(d) => &d.header,
            HeapObject::Task(d) => &d.header,
            HeapObject::External(d) => &d.header,
            HeapObject::Pap(d) => &d.header,
            HeapObject::MutRec(d) => &d.header,
            HeapObject::BoxedFloat(d) => &d.header,
            HeapObject::ByteArray(d) => &d.header,
        }
    }
    /// Get the mutable object header.
    pub fn header_mut(&mut self) -> &mut ObjectHeader {
        match self {
            HeapObject::Closure(d) => &mut d.header,
            HeapObject::Constructor(d) => &mut d.header,
            HeapObject::Array(d) => &mut d.header,
            HeapObject::StringObj(d) => &mut d.header,
            HeapObject::BigNat(d) => &mut d.header,
            HeapObject::BigInt(d) => &mut d.header,
            HeapObject::Thunk(d) => &mut d.header,
            HeapObject::IoAction(d) => &mut d.header,
            HeapObject::Task(d) => &mut d.header,
            HeapObject::External(d) => &mut d.header,
            HeapObject::Pap(d) => &mut d.header,
            HeapObject::MutRec(d) => &mut d.header,
            HeapObject::BoxedFloat(d) => &mut d.header,
            HeapObject::ByteArray(d) => &mut d.header,
        }
    }
}
/// IO action kinds.
#[derive(Clone, Debug)]
pub enum IoActionKind {
    /// Pure value: `pure a`.
    Pure(RtObject),
    /// Bind: `x >>= f`.
    Bind {
        /// First action.
        action: Box<IoActionKind>,
        /// Continuation.
        continuation: RtObject,
    },
    /// Print a string.
    PrintLn(String),
    /// Read a line from stdin.
    ReadLn,
    /// Read a file.
    ReadFile(String),
    /// Write a file.
    WriteFile {
        /// File path.
        path: String,
        /// Contents to write.
        contents: String,
    },
    /// Get the current time.
    GetTime,
    /// Exit with a code.
    Exit(i32),
    /// Throw an exception.
    Throw(RtObject),
    /// Catch an exception.
    Catch {
        /// Action that might throw.
        action: Box<IoActionKind>,
        /// Handler for exceptions.
        handler: RtObject,
    },
    /// Create a mutable reference.
    NewRef(RtObject),
    /// Read a mutable reference.
    ReadRef(u64),
    /// Write a mutable reference.
    WriteRef(u64, RtObject),
    /// Spawn a new task.
    Spawn(RtObject),
    /// Wait for a task to complete.
    Wait(u64),
}
/// Operations on array objects.
pub struct ArrayOps;
impl ArrayOps {
    /// Get the length of an array.
    pub fn len(obj: &RtObject) -> Option<usize> {
        obj.with_heap(|heap| {
            if let HeapObject::Array(data) = heap {
                Some(data.elements.len())
            } else {
                None
            }
        })
        .flatten()
    }
    /// Check if an array is empty.
    pub fn is_empty(obj: &RtObject) -> Option<bool> {
        Self::len(obj).map(|l| l == 0)
    }
    /// Get an element by index.
    pub fn get(obj: &RtObject, index: usize) -> Option<RtObject> {
        obj.with_heap(|heap| {
            if let HeapObject::Array(data) = heap {
                data.elements.get(index).cloned()
            } else {
                None
            }
        })
        .flatten()
    }
    /// Set an element by index (in-place if unique, otherwise copy).
    pub fn set(obj: &RtObject, index: usize, value: RtObject) -> Option<RtObject> {
        let mutated = obj.with_heap_mut(|heap| {
            if let HeapObject::Array(data) = heap {
                if data.header.is_unique() && index < data.elements.len() {
                    data.elements[index] = value.clone();
                    return true;
                }
            }
            false
        });
        if mutated == Some(true) {
            return Some(obj.clone());
        }
        obj.with_heap(|heap| {
            if let HeapObject::Array(data) = heap {
                if index < data.elements.len() {
                    let mut new_elements = data.elements.clone();
                    new_elements[index] = value;
                    Some(RtObject::array(new_elements))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
    }
    /// Push an element onto the end of an array.
    pub fn push(obj: &RtObject, value: RtObject) -> Option<RtObject> {
        let pushed = obj.with_heap_mut(|heap| {
            if let HeapObject::Array(data) = heap {
                if data.header.is_unique() {
                    data.elements.push(value.clone());
                    return true;
                }
            }
            false
        });
        if pushed == Some(true) {
            return Some(obj.clone());
        }
        obj.with_heap(|heap| {
            if let HeapObject::Array(data) = heap {
                let mut new_elements = data.elements.clone();
                new_elements.push(value);
                Some(RtObject::array(new_elements))
            } else {
                None
            }
        })
        .flatten()
    }
    /// Pop the last element from an array.
    pub fn pop(obj: &RtObject) -> Option<(RtObject, RtObject)> {
        obj.with_heap(|heap| {
            if let HeapObject::Array(data) = heap {
                if data.elements.is_empty() {
                    return None;
                }
                let mut new_elements = data.elements.clone();
                let last = new_elements
                    .pop()
                    .expect("elements is non-empty as verified by the is_empty check above");
                Some((RtObject::array(new_elements), last))
            } else {
                None
            }
        })
        .flatten()
    }
    /// Create an array of the given size filled with a default value.
    pub fn mk_array(size: usize, default: RtObject) -> RtObject {
        let elements = vec![default; size];
        RtObject::array(elements)
    }
    /// Concatenate two arrays.
    pub fn concat(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let elems_a = a
            .with_heap(|heap| {
                if let HeapObject::Array(data) = heap {
                    Some(data.elements.clone())
                } else {
                    None
                }
            })
            .flatten()?;
        let elems_b = b
            .with_heap(|heap| {
                if let HeapObject::Array(data) = heap {
                    Some(data.elements.clone())
                } else {
                    None
                }
            })
            .flatten()?;
        let mut combined = elems_a;
        combined.extend(elems_b);
        Some(RtObject::array(combined))
    }
    /// Slice an array.
    pub fn slice(obj: &RtObject, start: usize, end: usize) -> Option<RtObject> {
        obj.with_heap(|heap| {
            if let HeapObject::Array(data) = heap {
                let end = end.min(data.elements.len());
                if start > end {
                    return Some(RtObject::array(Vec::new()));
                }
                Some(RtObject::array(data.elements[start..end].to_vec()))
            } else {
                None
            }
        })
        .flatten()
    }
    /// Reverse an array.
    pub fn reverse(obj: &RtObject) -> Option<RtObject> {
        obj.with_heap(|heap| {
            if let HeapObject::Array(data) = heap {
                let mut rev = data.elements.clone();
                rev.reverse();
                Some(RtObject::array(rev))
            } else {
                None
            }
        })
        .flatten()
    }
}
/// Operations on string objects.
pub struct StringOps;
impl StringOps {
    /// Get the length of a string in bytes.
    pub fn byte_len(obj: &RtObject) -> Option<usize> {
        obj.with_heap(|heap| {
            if let HeapObject::StringObj(data) = heap {
                Some(data.value.len())
            } else {
                None
            }
        })
        .flatten()
    }
    /// Get the length of a string in characters.
    pub fn char_len(obj: &RtObject) -> Option<usize> {
        obj.with_heap(|heap| {
            if let HeapObject::StringObj(data) = heap {
                Some(data.value.chars().count())
            } else {
                None
            }
        })
        .flatten()
    }
    /// Get the string value.
    pub fn as_str(obj: &RtObject) -> Option<String> {
        obj.with_heap(|heap| {
            if let HeapObject::StringObj(data) = heap {
                Some(data.value.clone())
            } else {
                None
            }
        })
        .flatten()
    }
    /// Concatenate two strings.
    pub fn concat(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let sa = Self::as_str(a)?;
        let sb = Self::as_str(b)?;
        Some(RtObject::string(format!("{}{}", sa, sb)))
    }
    /// Get a character at an index.
    pub fn char_at(obj: &RtObject, index: usize) -> Option<RtObject> {
        obj.with_heap(|heap| {
            if let HeapObject::StringObj(data) = heap {
                data.value.chars().nth(index).map(RtObject::char_val)
            } else {
                None
            }
        })
        .flatten()
    }
    /// Take a substring.
    pub fn substring(obj: &RtObject, start: usize, len: usize) -> Option<RtObject> {
        obj.with_heap(|heap| {
            if let HeapObject::StringObj(data) = heap {
                let s: String = data.value.chars().skip(start).take(len).collect();
                Some(RtObject::string(s))
            } else {
                None
            }
        })
        .flatten()
    }
    /// Convert a string to a list of characters.
    pub fn to_char_list(obj: &RtObject) -> Option<Vec<RtObject>> {
        obj.with_heap(|heap| {
            if let HeapObject::StringObj(data) = heap {
                Some(data.value.chars().map(RtObject::char_val).collect())
            } else {
                None
            }
        })
        .flatten()
    }
    /// Convert a natural number to its string representation.
    pub fn nat_to_string(n: &RtObject) -> Option<RtObject> {
        let v = n.as_small_nat()?;
        Some(RtObject::string(format!("{}", v)))
    }
    /// Append a character to a string.
    pub fn push_char(s: &RtObject, c: &RtObject) -> Option<RtObject> {
        let sv = Self::as_str(s)?;
        let cv = c.as_char()?;
        let mut result = sv;
        result.push(cv);
        Some(RtObject::string(result))
    }
}
/// Object comparison utilities.
#[allow(dead_code)]
pub struct RtObjectCmp;
#[allow(dead_code)]
impl RtObjectCmp {
    /// Returns true if two RtObjects are numerically equal (both Int or both Float).
    pub fn numeric_eq(a: &RtObject, b: &RtObject) -> bool {
        match (a.as_small_int(), b.as_small_int()) {
            (Some(x), Some(y)) => return x == y,
            _ => {}
        }
        match (a.as_float_bits(), b.as_float_bits()) {
            (Some(x), Some(y)) => return x == y,
            _ => {}
        }
        false
    }
    /// Returns true if `a` is strictly less than `b` (integers only).
    pub fn int_lt(a: &RtObject, b: &RtObject) -> Option<bool> {
        match (a.as_small_int(), b.as_small_int()) {
            (Some(x), Some(y)) => Some(x < y),
            _ => None,
        }
    }
}
/// Runtime type information for a type.
#[derive(Clone, Debug)]
pub struct TypeInfo {
    /// Fully qualified name of the type.
    pub name: Name,
    /// Number of type parameters.
    pub num_params: u32,
    /// Whether this is a proposition (proof-irrelevant).
    pub is_prop: bool,
    /// Constructor descriptors.
    pub constructors: Vec<CtorInfo>,
    /// Whether this type has a special optimized representation.
    pub special_repr: Option<SpecialRepr>,
}
/// Registry of all known runtime types.
pub struct TypeRegistry {
    /// Map from type name to type info.
    types: HashMap<String, TypeInfo>,
}
impl TypeRegistry {
    /// Create a new empty type registry.
    pub fn new() -> Self {
        TypeRegistry {
            types: HashMap::new(),
        }
    }
    /// Register a type.
    pub fn register(&mut self, info: TypeInfo) {
        let key = format!("{}", info.name);
        self.types.insert(key, info);
    }
    /// Look up type information by name.
    pub fn lookup(&self, name: &str) -> Option<&TypeInfo> {
        self.types.get(name)
    }
    /// Get all registered types.
    pub fn all_types(&self) -> impl Iterator<Item = &TypeInfo> {
        self.types.values()
    }
    /// Number of registered types.
    pub fn len(&self) -> usize {
        self.types.len()
    }
    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
    /// Register built-in types (Nat, Bool, Unit, etc.).
    pub fn register_builtins(&mut self) {
        self.register(TypeInfo {
            name: Name::str("Nat"),
            num_params: 0,
            is_prop: false,
            constructors: vec![
                CtorInfo {
                    name: Name::str("Nat").append_str("zero"),
                    index: 0,
                    num_scalar_fields: 0,
                    num_object_fields: 0,
                    field_names: Vec::new(),
                },
                CtorInfo {
                    name: Name::str("Nat").append_str("succ"),
                    index: 1,
                    num_scalar_fields: 0,
                    num_object_fields: 1,
                    field_names: vec!["n".to_string()],
                },
            ],
            special_repr: Some(SpecialRepr::InlineNat),
        });
        self.register(TypeInfo {
            name: Name::str("Bool"),
            num_params: 0,
            is_prop: false,
            constructors: vec![
                CtorInfo {
                    name: Name::str("Bool").append_str("false"),
                    index: 0,
                    num_scalar_fields: 0,
                    num_object_fields: 0,
                    field_names: Vec::new(),
                },
                CtorInfo {
                    name: Name::str("Bool").append_str("true"),
                    index: 1,
                    num_scalar_fields: 0,
                    num_object_fields: 0,
                    field_names: Vec::new(),
                },
            ],
            special_repr: Some(SpecialRepr::InlineBool),
        });
        self.register(TypeInfo {
            name: Name::str("Unit"),
            num_params: 0,
            is_prop: false,
            constructors: vec![CtorInfo {
                name: Name::str("Unit").append_str("unit"),
                index: 0,
                num_scalar_fields: 0,
                num_object_fields: 0,
                field_names: Vec::new(),
            }],
            special_repr: Some(SpecialRepr::InlineUnit),
        });
        self.register(TypeInfo {
            name: Name::str("Option"),
            num_params: 1,
            is_prop: false,
            constructors: vec![
                CtorInfo {
                    name: Name::str("Option").append_str("none"),
                    index: 0,
                    num_scalar_fields: 0,
                    num_object_fields: 0,
                    field_names: Vec::new(),
                },
                CtorInfo {
                    name: Name::str("Option").append_str("some"),
                    index: 1,
                    num_scalar_fields: 0,
                    num_object_fields: 1,
                    field_names: vec!["val".to_string()],
                },
            ],
            special_repr: None,
        });
        self.register(TypeInfo {
            name: Name::str("List"),
            num_params: 1,
            is_prop: false,
            constructors: vec![
                CtorInfo {
                    name: Name::str("List").append_str("nil"),
                    index: 0,
                    num_scalar_fields: 0,
                    num_object_fields: 0,
                    field_names: Vec::new(),
                },
                CtorInfo {
                    name: Name::str("List").append_str("cons"),
                    index: 1,
                    num_scalar_fields: 0,
                    num_object_fields: 2,
                    field_names: vec!["head".to_string(), "tail".to_string()],
                },
            ],
            special_repr: None,
        });
        self.register(TypeInfo {
            name: Name::str("Prod"),
            num_params: 2,
            is_prop: false,
            constructors: vec![CtorInfo {
                name: Name::str("Prod").append_str("mk"),
                index: 0,
                num_scalar_fields: 0,
                num_object_fields: 2,
                field_names: vec!["fst".to_string(), "snd".to_string()],
            }],
            special_repr: None,
        });
    }
}
/// Flags that can be set on a heap-allocated object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectFlags(u8);
impl ObjectFlags {
    /// No flags set.
    pub fn empty() -> Self {
        ObjectFlags(0)
    }
    /// Object has been moved (for compacting GC, if used).
    pub fn moved() -> Self {
        ObjectFlags(0x01)
    }
    /// Object is pinned and must not be relocated.
    pub fn pinned() -> Self {
        ObjectFlags(0x02)
    }
    /// Object is shared across threads (uses atomic RC).
    pub fn shared() -> Self {
        ObjectFlags(0x04)
    }
    /// Object is finalized (destructor has been called).
    pub fn finalized() -> Self {
        ObjectFlags(0x08)
    }
    /// Object is immutable.
    pub fn immutable() -> Self {
        ObjectFlags(0x10)
    }
    /// Check if a specific flag is set.
    pub fn has(&self, flag: ObjectFlags) -> bool {
        (self.0 & flag.0) != 0
    }
    /// Set a flag.
    pub fn set(&mut self, flag: ObjectFlags) {
        self.0 |= flag.0;
    }
    /// Clear a flag.
    pub fn clear(&mut self, flag: ObjectFlags) {
        self.0 &= !flag.0;
    }
    /// Get raw bits.
    pub fn bits(&self) -> u8 {
        self.0
    }
}
/// A trivial object pool for reuse.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RtObjectPool {
    free: Vec<RtObject>,
}
#[allow(dead_code)]
impl RtObjectPool {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn acquire_unit(&mut self) -> RtObject {
        self.free.pop().unwrap_or_else(RtObject::unit)
    }
    pub fn release(&mut self, obj: RtObject) {
        if self.free.len() < 64 {
            self.free.push(obj);
        }
    }
    pub fn free_count(&self) -> usize {
        self.free.len()
    }
}
/// Data for a byte array.
#[derive(Clone, Debug)]
pub struct ByteArrayData {
    /// Object header.
    pub header: ObjectHeader,
    /// The byte data.
    pub bytes: Vec<u8>,
}
/// Constructor information for runtime dispatch.
#[derive(Clone, Debug)]
pub struct CtorInfo {
    /// Constructor name.
    pub name: Name,
    /// Constructor index.
    pub index: u32,
    /// Number of scalar (unboxed) fields.
    pub num_scalar_fields: u16,
    /// Number of object (boxed) fields.
    pub num_object_fields: u16,
    /// Field names (if named constructor).
    pub field_names: Vec<String>,
}
/// Runtime type tag for heap-allocated objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TypeTag {
    /// A closure object.
    Closure = 0,
    /// A constructor with fields.
    Constructor = 1,
    /// An array.
    Array = 2,
    /// A string.
    StringObj = 3,
    /// A big natural number (arbitrary precision).
    BigNat = 4,
    /// A big integer (arbitrary precision).
    BigInt = 5,
    /// A thunk (lazy value).
    Thunk = 6,
    /// An IO action.
    IoAction = 7,
    /// A task (concurrent computation).
    Task = 8,
    /// An external/opaque object.
    External = 9,
    /// A partial application (PAP).
    Pap = 10,
    /// A mutual recursive closure group.
    MutRec = 11,
    /// An environment captured by a closure.
    ClosureEnv = 12,
    /// A boxed float (full precision).
    BoxedFloat = 13,
    /// A byte array.
    ByteArray = 14,
    /// A module object.
    Module = 15,
}
impl TypeTag {
    /// Convert from raw u8.
    pub fn from_u8(v: u8) -> Option<TypeTag> {
        match v {
            0 => Some(TypeTag::Closure),
            1 => Some(TypeTag::Constructor),
            2 => Some(TypeTag::Array),
            3 => Some(TypeTag::StringObj),
            4 => Some(TypeTag::BigNat),
            5 => Some(TypeTag::BigInt),
            6 => Some(TypeTag::Thunk),
            7 => Some(TypeTag::IoAction),
            8 => Some(TypeTag::Task),
            9 => Some(TypeTag::External),
            10 => Some(TypeTag::Pap),
            11 => Some(TypeTag::MutRec),
            12 => Some(TypeTag::ClosureEnv),
            13 => Some(TypeTag::BoxedFloat),
            14 => Some(TypeTag::ByteArray),
            15 => Some(TypeTag::Module),
            _ => None,
        }
    }
    /// Convert to raw u8.
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
/// Header placed at the beginning of every heap-allocated object.
///
/// Layout:
/// - `rc_count` (u32): reference count
/// - `type_tag` (TypeTag): which kind of heap object
/// - `flags` (ObjectFlags): status flags
/// - `size` (u16): total object size in 8-byte words (max 512 KB)
#[derive(Clone, Debug)]
pub struct ObjectHeader {
    /// Reference count.
    pub rc_count: u32,
    /// Type tag for the heap object.
    pub type_tag: TypeTag,
    /// Object flags.
    pub flags: ObjectFlags,
    /// Size of the object in 8-byte words.
    pub size_words: u16,
}
impl ObjectHeader {
    /// Create a new object header.
    pub fn new(type_tag: TypeTag, size_words: u16) -> Self {
        ObjectHeader {
            rc_count: 1,
            type_tag,
            flags: ObjectFlags::empty(),
            size_words,
        }
    }
    /// Increment the reference count.
    pub fn inc_ref(&mut self) {
        self.rc_count = self.rc_count.saturating_add(1);
    }
    /// Decrement the reference count. Returns true if the count reaches zero.
    pub fn dec_ref(&mut self) -> bool {
        if self.rc_count == 0 {
            return true;
        }
        self.rc_count -= 1;
        self.rc_count == 0
    }
    /// Check if the object has a single owner.
    pub fn is_unique(&self) -> bool {
        self.rc_count == 1
    }
    /// Check if the object is shared.
    pub fn is_shared(&self) -> bool {
        self.rc_count > 1 || self.flags.has(ObjectFlags::shared())
    }
    /// Total size in bytes.
    pub fn size_bytes(&self) -> usize {
        self.size_words as usize * 8
    }
    /// Encode header into a u64 for compact storage.
    pub fn encode(&self) -> u64 {
        let rc = self.rc_count as u64;
        let tag = self.type_tag.as_u8() as u64;
        let flags = self.flags.bits() as u64;
        let size = self.size_words as u64;
        (rc << 32) | (tag << 24) | (flags << 16) | size
    }
    /// Decode header from a u64.
    pub fn decode(bits: u64) -> Option<Self> {
        let rc = (bits >> 32) as u32;
        let tag_byte = ((bits >> 24) & 0xFF) as u8;
        let flags_byte = ((bits >> 16) & 0xFF) as u8;
        let size = (bits & 0xFFFF) as u16;
        let type_tag = TypeTag::from_u8(tag_byte)?;
        Some(ObjectHeader {
            rc_count: rc,
            type_tag,
            flags: ObjectFlags(flags_byte),
            size_words: size,
        })
    }
}
/// Data for a partial application (PAP).
#[derive(Clone, Debug)]
pub struct PapData {
    /// Object header.
    pub header: ObjectHeader,
    /// The closure being partially applied.
    pub closure: RtObject,
    /// Total arity of the closure.
    pub arity: u16,
    /// Arguments applied so far.
    pub args: Vec<RtObject>,
}
/// Data for a task.
#[derive(Clone, Debug)]
pub struct TaskData {
    /// Object header.
    pub header: ObjectHeader,
    /// Task state.
    pub state: TaskState,
    /// Task ID.
    pub task_id: u64,
}
/// Operations on thunk objects.
pub struct ThunkOps;
impl ThunkOps {
    /// Check if a thunk has been evaluated.
    pub fn is_evaluated(obj: &RtObject) -> Option<bool> {
        obj.with_heap(|heap| {
            if let HeapObject::Thunk(data) = heap {
                Some(matches!(data.state, ThunkState::Evaluated { .. }))
            } else {
                None
            }
        })
        .flatten()
    }
    /// Get the cached value of an evaluated thunk.
    pub fn get_value(obj: &RtObject) -> Option<RtObject> {
        obj.with_heap(|heap| {
            if let HeapObject::Thunk(data) = heap {
                if let ThunkState::Evaluated { ref value } = data.state {
                    Some(value.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
    }
    /// Set the value of a thunk (mark as evaluated).
    pub fn set_value(obj: &RtObject, value: RtObject) -> bool {
        obj.with_heap_mut(|heap| {
            if let HeapObject::Thunk(data) = heap {
                data.state = ThunkState::Evaluated { value };
                true
            } else {
                false
            }
        })
        .unwrap_or(false)
    }
    /// Mark a thunk as currently evaluating (for cycle detection).
    pub fn mark_evaluating(obj: &RtObject) -> bool {
        obj.with_heap_mut(|heap| {
            if let HeapObject::Thunk(data) = heap {
                data.state = ThunkState::Evaluating;
                true
            } else {
                false
            }
        })
        .unwrap_or(false)
    }
    /// Check if a thunk is in the evaluating state (cycle).
    pub fn is_evaluating(obj: &RtObject) -> Option<bool> {
        obj.with_heap(|heap| {
            if let HeapObject::Thunk(data) = heap {
                Some(matches!(data.state, ThunkState::Evaluating))
            } else {
                None
            }
        })
        .flatten()
    }
}
/// Allocation statistics.
#[derive(Clone, Debug, Default)]
pub struct AllocationStats {
    /// Total number of allocations.
    pub total_allocations: u64,
    /// Total number of deallocations.
    pub total_deallocations: u64,
    /// Current number of live objects.
    pub live_objects: u64,
    /// Peak number of live objects.
    pub peak_objects: u64,
    /// Total bytes allocated.
    pub total_bytes_allocated: u64,
}
/// Data for a constructor object.
#[derive(Clone, Debug)]
pub struct ConstructorData {
    /// Object header.
    pub header: ObjectHeader,
    /// Constructor index within the inductive type.
    pub ctor_index: u32,
    /// Number of fields.
    pub num_fields: u16,
    /// Scalar fields (unboxed small values).
    pub scalar_fields: Vec<u64>,
    /// Object fields (boxed values).
    pub object_fields: Vec<RtObject>,
    /// The name of the constructor (optional).
    pub name: Option<Name>,
}
/// Data for a thunk (lazy value).
#[derive(Clone, Debug)]
pub struct ThunkData {
    /// Object header.
    pub header: ObjectHeader,
    /// Current thunk state.
    pub state: ThunkState,
}
/// Data for a closure object.
#[derive(Clone, Debug)]
pub struct ClosureData {
    /// Object header.
    pub header: ObjectHeader,
    /// Function pointer index into the code table.
    pub fn_index: u32,
    /// Arity of the function (total number of parameters).
    pub arity: u16,
    /// Number of captured environment variables.
    pub env_size: u16,
    /// Captured environment values.
    pub env: Vec<RtObject>,
}
/// Special representation hints for common types.
#[derive(Clone, Debug)]
pub enum SpecialRepr {
    /// This type is represented as a small nat inline.
    InlineNat,
    /// This type is represented as a bool inline.
    InlineBool,
    /// This type is represented as a unit inline.
    InlineUnit,
    /// This type is represented as a char inline.
    InlineChar,
    /// This type uses enum-style representation (no fields).
    EnumTag {
        /// Number of constructors.
        num_ctors: u32,
    },
    /// This type uses a packed struct representation.
    PackedStruct {
        /// Total size in bytes.
        size_bytes: u32,
    },
    /// This type is represented as a boxed array.
    BoxedArray,
    /// This type is represented as a string.
    BoxedString,
}
/// Thread-local storage for heap objects.
///
/// Uses a `Vec<Option<HeapObject>>` with a free-list for O(1) alloc/free.
/// A production implementation could use mmap'd regions for large heaps,
/// but the current design is correct and suitable for the interpreter.
pub struct ObjectStore {
    /// All allocated objects, indexed by their ID.
    objects: Vec<Option<HeapObject>>,
    /// Free list for reuse.
    free_list: Vec<usize>,
    /// Statistics.
    stats: AllocationStats,
}
impl ObjectStore {
    /// Create a new empty object store.
    pub fn new() -> Self {
        ObjectStore {
            objects: Vec::new(),
            free_list: Vec::new(),
            stats: AllocationStats::default(),
        }
    }
    /// Create a new object store with pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        ObjectStore {
            objects: Vec::with_capacity(cap),
            free_list: Vec::new(),
            stats: AllocationStats::default(),
        }
    }
    /// Allocate a new heap object and return its ID.
    pub fn allocate(&mut self, obj: HeapObject) -> usize {
        self.stats.total_allocations += 1;
        self.stats.live_objects += 1;
        if self.stats.live_objects > self.stats.peak_objects {
            self.stats.peak_objects = self.stats.live_objects;
        }
        self.stats.total_bytes_allocated += obj.header().size_bytes() as u64;
        if let Some(id) = self.free_list.pop() {
            self.objects[id] = Some(obj);
            id
        } else {
            let id = self.objects.len();
            self.objects.push(Some(obj));
            id
        }
    }
    /// Deallocate a heap object by ID.
    pub fn deallocate(&mut self, id: usize) -> Option<HeapObject> {
        if id >= self.objects.len() {
            return None;
        }
        let obj = self.objects[id].take();
        if obj.is_some() {
            self.free_list.push(id);
            self.stats.total_deallocations += 1;
            self.stats.live_objects = self.stats.live_objects.saturating_sub(1);
        }
        obj
    }
    /// Get a reference to a heap object by ID.
    pub fn get(&self, id: usize) -> Option<&HeapObject> {
        self.objects.get(id).and_then(|o| o.as_ref())
    }
    /// Get a mutable reference to a heap object by ID.
    pub fn get_mut(&mut self, id: usize) -> Option<&mut HeapObject> {
        self.objects.get_mut(id).and_then(|o| o.as_mut())
    }
    /// Get allocation statistics.
    pub fn stats(&self) -> &AllocationStats {
        &self.stats
    }
    /// Number of live objects.
    pub fn live_count(&self) -> usize {
        self.stats.live_objects as usize
    }
    /// Total capacity (including free slots).
    pub fn capacity(&self) -> usize {
        self.objects.capacity()
    }
    /// Access the global store via a thread-local.
    pub(super) fn global_store<R>(f: impl FnOnce(&mut ObjectStore) -> R) -> R {
        thread_local! {
            static STORE : std::cell::RefCell < ObjectStore > =
            std::cell::RefCell::new(ObjectStore::new());
        }
        STORE.with(|store| f(&mut store.borrow_mut()))
    }
}
/// Data for an array object.
#[derive(Clone, Debug)]
pub struct ArrayData {
    /// Object header.
    pub header: ObjectHeader,
    /// Array elements.
    pub elements: Vec<RtObject>,
    /// Capacity (for pre-allocated arrays).
    pub capacity: usize,
}
/// Data for an IO action.
#[derive(Clone, Debug)]
pub struct IoActionData {
    /// Object header.
    pub header: ObjectHeader,
    /// The IO action kind.
    pub kind: IoActionKind,
}
/// Task state.
#[derive(Clone, Debug)]
pub enum TaskState {
    /// Task is pending (not yet started or in progress).
    Pending,
    /// Task is running.
    Running,
    /// Task has completed with a result.
    Completed(RtObject),
    /// Task has failed with an error.
    Failed(RtObject),
    /// Task has been cancelled.
    Cancelled,
}
/// Data for an external/opaque value.
#[derive(Clone, Debug)]
pub struct ExternalData {
    /// Object header.
    pub header: ObjectHeader,
    /// Type name for the external value.
    pub type_name: String,
    /// Opaque payload (serialized or boxed).
    pub payload: Vec<u8>,
}
/// A table mapping names to runtime objects.
///
/// Used for the global constant table, module exports, etc.
pub struct ObjectTable {
    /// Map from name to object.
    entries: HashMap<String, RtObject>,
    /// Insertion order (for deterministic iteration).
    order: Vec<String>,
}
impl ObjectTable {
    /// Create a new empty object table.
    pub fn new() -> Self {
        ObjectTable {
            entries: HashMap::new(),
            order: Vec::new(),
        }
    }
    /// Insert an entry.
    pub fn insert(&mut self, name: String, obj: RtObject) {
        if !self.entries.contains_key(&name) {
            self.order.push(name.clone());
        }
        self.entries.insert(name, obj);
    }
    /// Look up an entry by name.
    pub fn get(&self, name: &str) -> Option<&RtObject> {
        self.entries.get(name)
    }
    /// Check if a name exists.
    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Iterate entries in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &RtObject)> {
        self.order
            .iter()
            .filter_map(move |name| self.entries.get(name).map(|obj| (name.as_str(), obj)))
    }
    /// Remove an entry.
    pub fn remove(&mut self, name: &str) -> Option<RtObject> {
        if let Some(obj) = self.entries.remove(name) {
            self.order.retain(|n| n != name);
            Some(obj)
        } else {
            None
        }
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
    }
}
/// Arithmetic operations on runtime objects.
pub struct RtArith;
impl RtArith {
    /// Add two natural numbers.
    pub fn nat_add(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_nat()?;
        let bv = b.as_small_nat()?;
        Some(RtObject::nat(av.wrapping_add(bv)))
    }
    /// Subtract two natural numbers (saturating).
    pub fn nat_sub(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_nat()?;
        let bv = b.as_small_nat()?;
        Some(RtObject::nat(av.saturating_sub(bv)))
    }
    /// Multiply two natural numbers.
    pub fn nat_mul(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_nat()?;
        let bv = b.as_small_nat()?;
        Some(RtObject::nat(av.wrapping_mul(bv)))
    }
    /// Divide two natural numbers (integer division, rounds toward zero).
    pub fn nat_div(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_nat()?;
        let bv = b.as_small_nat()?;
        if bv == 0 {
            return Some(RtObject::nat(0));
        }
        Some(RtObject::nat(av / bv))
    }
    /// Modulo of two natural numbers.
    pub fn nat_mod(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_nat()?;
        let bv = b.as_small_nat()?;
        if bv == 0 {
            return Some(RtObject::nat(av));
        }
        Some(RtObject::nat(av % bv))
    }
    /// Compare two natural numbers (less than or equal).
    pub fn nat_le(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_nat()?;
        let bv = b.as_small_nat()?;
        Some(RtObject::bool_val(av <= bv))
    }
    /// Compare two natural numbers (less than).
    pub fn nat_lt(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_nat()?;
        let bv = b.as_small_nat()?;
        Some(RtObject::bool_val(av < bv))
    }
    /// Compare two natural numbers for equality.
    pub fn nat_eq(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_nat()?;
        let bv = b.as_small_nat()?;
        Some(RtObject::bool_val(av == bv))
    }
    /// Boolean AND.
    pub fn bool_and(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_bool()?;
        let bv = b.as_bool()?;
        Some(RtObject::bool_val(av && bv))
    }
    /// Boolean OR.
    pub fn bool_or(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_bool()?;
        let bv = b.as_bool()?;
        Some(RtObject::bool_val(av || bv))
    }
    /// Boolean NOT.
    pub fn bool_not(a: &RtObject) -> Option<RtObject> {
        let av = a.as_bool()?;
        Some(RtObject::bool_val(!av))
    }
    /// Integer addition.
    pub fn int_add(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_int()?;
        let bv = b.as_small_int()?;
        Some(av.wrapping_add(bv).box_into())
    }
    /// Integer subtraction.
    pub fn int_sub(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_int()?;
        let bv = b.as_small_int()?;
        Some(av.wrapping_sub(bv).box_into())
    }
    /// Integer multiplication.
    pub fn int_mul(a: &RtObject, b: &RtObject) -> Option<RtObject> {
        let av = a.as_small_int()?;
        let bv = b.as_small_int()?;
        Some(av.wrapping_mul(bv).box_into())
    }
    /// Integer negation.
    pub fn int_neg(a: &RtObject) -> Option<RtObject> {
        let av = a.as_small_int()?;
        Some(av.wrapping_neg().box_into())
    }
}
