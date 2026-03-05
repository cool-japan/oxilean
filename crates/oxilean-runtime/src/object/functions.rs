//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::rtobject_type::RtObject;
use super::types::{
    ArrayOps, BigIntData, FieldAccess, HeapObject, ObjectFlags, ObjectHeader, ObjectTable, RtArith,
    RtObjectCmp, RtObjectPool, StringOps, TypeRegistry, TypeTag,
};

/// Tag for heap-allocated objects.
pub(super) const TAG_HEAP: u8 = 0x00;
/// Tag for small natural numbers.
pub(super) const TAG_SMALL_NAT: u8 = 0x01;
/// Tag for boolean values.
pub(super) const TAG_BOOL: u8 = 0x02;
/// Tag for the unit value.
pub(super) const TAG_UNIT: u8 = 0x03;
/// Tag for character values.
pub(super) const TAG_CHAR: u8 = 0x04;
/// Tag for small constructor indices.
pub(super) const TAG_CTOR: u8 = 0x05;
/// Tag for signed integers.
pub(super) const TAG_INT: u8 = 0x06;
/// Tag for float bit patterns (reduced precision).
pub(super) const TAG_FLOAT_BITS: u8 = 0x07;
/// Tag for closure references.
pub(super) const TAG_CLOSURE: u8 = 0x08;
/// Tag for array references.
pub(super) const TAG_ARRAY: u8 = 0x09;
/// Tag for string references.
pub(super) const TAG_STRING: u8 = 0x0A;
/// Tag for thunk references.
pub(super) const TAG_THUNK: u8 = 0x0B;
/// Tag for IO action references.
pub(super) const TAG_IO_ACTION: u8 = 0x0C;
/// Tag for task references.
pub(super) const TAG_TASK: u8 = 0x0D;
/// Tag for external/opaque references.
pub(super) const TAG_EXTERNAL: u8 = 0x0E;
/// Reserved tag.
pub(super) const _TAG_RESERVED: u8 = 0x0F;
/// Mask for extracting the 56-bit payload from a tagged u64.
pub(super) const PAYLOAD_MASK: u64 = 0x00FF_FFFF_FFFF_FFFF;
/// Maximum value for a small natural number.
pub(super) const MAX_SMALL_NAT: u64 = PAYLOAD_MASK;
/// Maximum value for an inline signed integer (55-bit magnitude + sign).
pub(super) const MAX_SMALL_INT: i64 = (1_i64 << 55) - 1;
/// Minimum value for an inline signed integer.
pub(super) const MIN_SMALL_INT: i64 = -(1_i64 << 55);
/// Type-safe boxing of Rust primitives into `RtObject`.
pub trait BoxInto {
    /// Box this value into an `RtObject`.
    fn box_into(self) -> RtObject;
}
/// Type-safe unboxing of `RtObject` into Rust primitives.
pub trait UnboxFrom: Sized {
    /// Try to unbox an `RtObject` into this type.
    fn unbox_from(obj: &RtObject) -> Option<Self>;
}
impl BoxInto for bool {
    fn box_into(self) -> RtObject {
        RtObject::bool_val(self)
    }
}
impl UnboxFrom for bool {
    fn unbox_from(obj: &RtObject) -> Option<Self> {
        obj.as_bool()
    }
}
impl BoxInto for u64 {
    fn box_into(self) -> RtObject {
        RtObject::nat(self)
    }
}
impl UnboxFrom for u64 {
    fn unbox_from(obj: &RtObject) -> Option<Self> {
        obj.as_small_nat()
    }
}
impl BoxInto for i64 {
    fn box_into(self) -> RtObject {
        RtObject::small_int(self).unwrap_or_else(|| {
            let negative = self < 0;
            let magnitude = if negative {
                (self as i128).unsigned_abs() as u64
            } else {
                self as u64
            };
            let heap = HeapObject::BigInt(BigIntData {
                header: ObjectHeader::new(TypeTag::BigInt, 2),
                negative,
                digits: vec![magnitude],
            });
            RtObject::from_heap(heap)
        })
    }
}
impl UnboxFrom for i64 {
    fn unbox_from(obj: &RtObject) -> Option<Self> {
        obj.as_small_int()
    }
}
impl BoxInto for char {
    fn box_into(self) -> RtObject {
        RtObject::char_val(self)
    }
}
impl UnboxFrom for char {
    fn unbox_from(obj: &RtObject) -> Option<Self> {
        obj.as_char()
    }
}
impl BoxInto for f64 {
    fn box_into(self) -> RtObject {
        RtObject::boxed_float(self)
    }
}
impl BoxInto for String {
    fn box_into(self) -> RtObject {
        RtObject::string(self)
    }
}
impl BoxInto for &str {
    fn box_into(self) -> RtObject {
        RtObject::string(self.to_string())
    }
}
impl BoxInto for () {
    fn box_into(self) -> RtObject {
        RtObject::unit()
    }
}
impl UnboxFrom for () {
    fn unbox_from(obj: &RtObject) -> Option<Self> {
        if obj.is_unit() {
            Some(())
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_small_nat() {
        let n = RtObject::nat(42);
        assert!(n.is_nat());
        assert_eq!(n.as_small_nat(), Some(42));
    }
    #[test]
    fn test_bool() {
        let t = RtObject::bool_val(true);
        let f = RtObject::bool_val(false);
        assert_eq!(t.as_bool(), Some(true));
        assert_eq!(f.as_bool(), Some(false));
    }
    #[test]
    fn test_unit() {
        let u = RtObject::unit();
        assert!(u.is_unit());
    }
    #[test]
    fn test_char() {
        let c = RtObject::char_val('A');
        assert_eq!(c.as_char(), Some('A'));
    }
    #[test]
    fn test_small_int() {
        let positive = RtObject::small_int(42).expect("test operation should succeed");
        assert_eq!(positive.as_small_int(), Some(42));
        let negative = RtObject::small_int(-7).expect("test operation should succeed");
        assert_eq!(negative.as_small_int(), Some(-7));
    }
    #[test]
    fn test_object_header_encode_decode() {
        let header = ObjectHeader::new(TypeTag::Closure, 5);
        let encoded = header.encode();
        let decoded = ObjectHeader::decode(encoded).expect("test operation should succeed");
        assert_eq!(decoded.type_tag, TypeTag::Closure);
        assert_eq!(decoded.size_words, 5);
        assert_eq!(decoded.rc_count, 1);
    }
    #[test]
    fn test_object_flags() {
        let mut flags = ObjectFlags::empty();
        assert!(!flags.has(ObjectFlags::pinned()));
        flags.set(ObjectFlags::pinned());
        assert!(flags.has(ObjectFlags::pinned()));
        flags.clear(ObjectFlags::pinned());
        assert!(!flags.has(ObjectFlags::pinned()));
    }
    #[test]
    fn test_nat_arithmetic() {
        let a = RtObject::nat(10);
        let b = RtObject::nat(3);
        assert_eq!(
            RtArith::nat_add(&a, &b)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(13)
        );
        assert_eq!(
            RtArith::nat_sub(&a, &b)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(7)
        );
        assert_eq!(
            RtArith::nat_mul(&a, &b)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(30)
        );
        assert_eq!(
            RtArith::nat_div(&a, &b)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(3)
        );
        assert_eq!(
            RtArith::nat_mod(&a, &b)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(1)
        );
    }
    #[test]
    fn test_object_table() {
        let mut table = ObjectTable::new();
        table.insert("x".to_string(), RtObject::nat(42));
        table.insert("y".to_string(), RtObject::bool_val(true));
        assert_eq!(table.len(), 2);
        assert_eq!(
            table
                .get("x")
                .expect("key should exist in map")
                .as_small_nat(),
            Some(42)
        );
        assert!(table.contains("y"));
    }
    #[test]
    fn test_type_registry_builtins() {
        let mut registry = TypeRegistry::new();
        registry.register_builtins();
        assert!(registry.lookup("Nat").is_some());
        assert!(registry.lookup("Bool").is_some());
        assert!(registry.lookup("Unit").is_some());
        assert!(registry.lookup("List").is_some());
    }
    #[test]
    fn test_boxing() {
        let b: RtObject = true.box_into();
        assert_eq!(bool::unbox_from(&b), Some(true));
        let n: RtObject = 42u64.box_into();
        assert_eq!(u64::unbox_from(&n), Some(42));
        let u: RtObject = ().box_into();
        assert_eq!(<()>::unbox_from(&u), Some(()));
    }
    #[test]
    fn test_string_object() {
        let s = RtObject::string("hello".to_string());
        assert!(s.is_string_ref());
        assert_eq!(StringOps::as_str(&s), Some("hello".to_string()));
        assert_eq!(StringOps::byte_len(&s), Some(5));
    }
    #[test]
    fn test_array_object() {
        let arr = RtObject::array(vec![RtObject::nat(1), RtObject::nat(2), RtObject::nat(3)]);
        assert_eq!(ArrayOps::len(&arr), Some(3));
        assert_eq!(
            ArrayOps::get(&arr, 1)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(2)
        );
    }
    #[test]
    fn test_constructor_fields() {
        let pair = RtObject::constructor(0, vec![RtObject::nat(1), RtObject::nat(2)]);
        assert_eq!(FieldAccess::get_ctor_index(&pair), Some(0));
        assert_eq!(FieldAccess::num_fields(&pair), Some(2));
        assert_eq!(
            FieldAccess::proj_fst(&pair)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(1)
        );
        assert_eq!(
            FieldAccess::proj_snd(&pair)
                .expect("type conversion should succeed")
                .as_small_nat(),
            Some(2)
        );
    }
    #[test]
    fn test_bool_ops() {
        let t = RtObject::bool_val(true);
        let f = RtObject::bool_val(false);
        assert_eq!(
            RtArith::bool_and(&t, &f)
                .expect("type conversion should succeed")
                .as_bool(),
            Some(false)
        );
        assert_eq!(
            RtArith::bool_or(&t, &f)
                .expect("type conversion should succeed")
                .as_bool(),
            Some(true)
        );
        assert_eq!(
            RtArith::bool_not(&t)
                .expect("type conversion should succeed")
                .as_bool(),
            Some(false)
        );
    }
}
#[cfg(test)]
mod extra_object_tests {
    use super::*;
    #[test]
    fn test_numeric_eq_int() {
        let a = RtObject::small_int(10).expect("test operation should succeed");
        let b = RtObject::small_int(10).expect("test operation should succeed");
        let c = RtObject::small_int(11).expect("test operation should succeed");
        assert!(RtObjectCmp::numeric_eq(&a, &b));
        assert!(!RtObjectCmp::numeric_eq(&a, &c));
    }
    #[test]
    fn test_int_lt() {
        let a = RtObject::small_int(3).expect("test operation should succeed");
        let b = RtObject::small_int(5).expect("test operation should succeed");
        assert_eq!(RtObjectCmp::int_lt(&a, &b), Some(true));
        assert_eq!(RtObjectCmp::int_lt(&b, &a), Some(false));
    }
    #[test]
    fn test_object_pool() {
        let mut pool = RtObjectPool::new();
        let obj = pool.acquire_unit();
        pool.release(obj);
        assert_eq!(pool.free_count(), 1);
        let _ = pool.acquire_unit();
        assert_eq!(pool.free_count(), 0);
    }
}
