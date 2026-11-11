//! Tests pinning `ArrayOps::pop` on an empty array and on a scalar.

use super::super::rtobject_type::RtObject;
use super::super::types::ArrayOps;

#[test]
fn pop_of_an_empty_array_or_a_scalar_is_none() {
    let empty = RtObject::array(Vec::new());
    assert_eq!(ArrayOps::len(&empty), Some(0));
    assert!(ArrayOps::pop(&empty).is_none());
    assert!(ArrayOps::pop(&RtObject::nat(4)).is_none());
    assert!(ArrayOps::pop(&RtObject::nat(0)).is_none());
}
