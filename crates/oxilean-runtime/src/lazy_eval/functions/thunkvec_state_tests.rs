//! Tests for `ThunkVec::get` on element types of every kind, on a panicking
//! thunk and on a re-entrant `get` of the element being forced.

use super::super::types::ThunkVec;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[test]
fn forces_elements_whose_type_has_no_all_zero_value() {
    let mut v: ThunkVec<String> = ThunkVec::new();
    v.push_lazy(|| String::from("forced"));
    v.push_ready(String::from("ready"));
    v.push_lazy(|| "x".repeat(3));
    assert_eq!(v.get(0).as_deref(), Some("forced"));
    assert_eq!(v.get(0).as_deref(), Some("forced"));
    assert_eq!(v.get(1).as_deref(), Some("ready"));
    assert_eq!(v.force_all(), vec!["forced", "ready", "xxx"]);
    let mut boxed: ThunkVec<Box<u32>> = ThunkVec::new();
    boxed.push_lazy(|| Box::new(7));
    assert_eq!(boxed.get(0), Some(Box::new(7)));
    assert_eq!(boxed.get(1), None);
}

#[test]
fn element_whose_thunk_panicked_has_no_value() {
    let mut v: ThunkVec<u64> = ThunkVec::new();
    v.push_lazy(|| panic!("thunk fails"));
    v.push_lazy(|| 5);
    let first = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| v.get(0)));
    assert!(first.is_err());
    assert_eq!(v.get(0), None);
    assert_eq!(v.get(1), Some(5));
}

#[test]
fn reentrant_get_of_the_element_being_forced_has_no_value() {
    let shared: Rc<RefCell<ThunkVec<u64>>> = Rc::new(RefCell::new(ThunkVec::new()));
    let inner_seen: Rc<RefCell<Option<Option<u64>>>> = Rc::new(RefCell::new(None));
    let weak: Weak<RefCell<ThunkVec<u64>>> = Rc::downgrade(&shared);
    let record = Rc::clone(&inner_seen);
    shared.borrow_mut().push_lazy(move || {
        let seen = weak.upgrade().and_then(|v| v.borrow().get(0));
        *record.borrow_mut() = Some(seen);
        41
    });
    let outer = shared.borrow().get(0);
    assert_eq!(outer, Some(41));
    assert_eq!(*inner_seen.borrow(), Some(None));
    assert_eq!(shared.borrow().get(0), Some(41));
}

#[test]
fn force_all_skips_an_element_without_a_value() {
    let mut v: ThunkVec<u64> = ThunkVec::new();
    v.push_lazy(|| 1);
    v.push_lazy(|| panic!("thunk fails"));
    v.push_lazy(|| 3);
    let first = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| v.get(1)));
    assert!(first.is_err());
    assert_eq!(v.force_all(), vec![1, 3]);
    assert_eq!(v.len(), 3);
}
