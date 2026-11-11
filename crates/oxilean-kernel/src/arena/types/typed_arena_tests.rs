//! `TypedArena` against a verbatim copy of its earlier `Vec`-only form.

use super::TypedArena;
use std::cell::RefCell;
use std::rc::Rc;

/// The arena as it was, every item in one `Vec`.
struct VecArena<T> {
    items: Vec<T>,
}
impl<T> VecArena<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    fn alloc(&mut self, val: T) -> &T {
        self.items.push(val);
        self.items.last().expect("items list must be non-empty")
    }
    fn len(&self) -> usize {
        self.items.len()
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    fn clear(&mut self) {
        self.items.clear();
    }
}

#[test]
fn alloc_len_is_empty_and_clear_agree_with_the_vec_form() {
    let mut new = TypedArena::new();
    let mut old = VecArena::new();
    assert_eq!((new.len(), new.is_empty()), (old.len(), old.is_empty()));
    for round in 0..6u32 {
        for i in 0..round * 3 {
            let value = format!("{round}-{i}");
            assert_eq!(new.alloc(value.clone()), old.alloc(value));
            assert_eq!(new.len(), old.len());
            assert_eq!(new.is_empty(), old.is_empty());
        }
        new.clear();
        old.clear();
        assert_eq!((new.len(), new.is_empty()), (old.len(), old.is_empty()));
    }
}

/// Records its id in a shared log when dropped.
struct Logged(u32, Rc<RefCell<Vec<u32>>>);
impl Drop for Logged {
    fn drop(&mut self) {
        self.1.borrow_mut().push(self.0);
    }
}

#[test]
fn items_are_dropped_once_in_allocation_order() {
    for count in 0..5u32 {
        let new_log = Rc::new(RefCell::new(Vec::new()));
        let old_log = Rc::new(RefCell::new(Vec::new()));
        let mut new = TypedArena::new();
        let mut old = VecArena::new();
        for id in 0..count {
            new.alloc(Logged(id, Rc::clone(&new_log)));
            old.alloc(Logged(id, Rc::clone(&old_log)));
        }
        new.clear();
        old.clear();
        assert_eq!(*new_log.borrow(), *old_log.borrow());
        for id in 0..count {
            new.alloc(Logged(id, Rc::clone(&new_log)));
            old.alloc(Logged(id, Rc::clone(&old_log)));
        }
        drop(new);
        drop(old);
        assert_eq!(*new_log.borrow(), *old_log.borrow());
        assert_eq!(new_log.borrow().len(), 2 * count as usize);
    }
}

/// Logs its id when dropped, and then panics if it was built to.
struct Fragile(u32, bool, Rc<RefCell<Vec<u32>>>);
impl Drop for Fragile {
    fn drop(&mut self) {
        self.2.borrow_mut().push(self.0);
        if self.1 && !std::thread::panicking() {
            panic!("an item drop that panics, on purpose");
        }
    }
}

#[test]
fn clear_empties_the_arena_even_when_an_item_drop_panics() {
    use std::panic::{catch_unwind, AssertUnwindSafe};
    for panicking in 0..3u32 {
        let new_log = Rc::new(RefCell::new(Vec::new()));
        let old_log = Rc::new(RefCell::new(Vec::new()));
        let mut new = TypedArena::new();
        let mut old = VecArena::new();
        for id in 0..3 {
            new.alloc(Fragile(id, id == panicking, Rc::clone(&new_log)));
            old.alloc(Fragile(id, id == panicking, Rc::clone(&old_log)));
        }
        assert!(catch_unwind(AssertUnwindSafe(|| new.clear())).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| old.clear())).is_err());
        assert_eq!((new.len(), new.is_empty()), (0, true));
        assert_eq!((new.len(), new.is_empty()), (old.len(), old.is_empty()));
        assert_eq!(*new_log.borrow(), vec![0, 1, 2]);
        assert_eq!(*new_log.borrow(), *old_log.borrow());
        drop(new);
        drop(old);
        assert_eq!(*new_log.borrow(), vec![0, 1, 2]);
    }
}
