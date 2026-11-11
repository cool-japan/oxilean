//! `SimpleLruCache::put` (every copy) when a call unwinds out of the key's
//! `Clone`, the key's `Hash`, or the evicted value's `Drop`, and the caller
//! catches the panic: the cache still holds at most `capacity` entries, and
//! every key put afterwards is found.

use std::cell::Cell;
use std::hash::{Hash, Hasher};
use std::panic::{catch_unwind, AssertUnwindSafe};

thread_local! {
    /// When set, the next `Key::clone` on this thread panics.
    static CLONE_PANICS: Cell<bool> = const { Cell::new(false) };
    /// When `Some(n)`, `n` more `Key::hash` calls on this thread succeed and
    /// the one after them panics.
    static HASH_PANICS_AFTER: Cell<Option<u32>> = const { Cell::new(None) };
}

/// A key whose `Clone` and `Hash` panic when the flags above say so.
#[derive(Debug, PartialEq, Eq)]
struct Key(u64);
impl Clone for Key {
    fn clone(&self) -> Self {
        if CLONE_PANICS.with(|flag| flag.replace(false)) {
            panic!("a key clone that panics, on purpose");
        }
        Key(self.0)
    }
}
impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let panics = HASH_PANICS_AFTER.with(|left| match left.get() {
            Some(0) => {
                left.set(None);
                true
            }
            Some(n) => {
                left.set(Some(n - 1));
                false
            }
            None => false,
        });
        if panics {
            panic!("a key hash that panics, on purpose");
        }
        self.0.hash(state);
    }
}

/// A value whose `Drop` panics when it was built with `panics_on_drop`.
#[derive(Clone, Debug)]
struct Val {
    id: u64,
    panics_on_drop: bool,
}
impl Drop for Val {
    fn drop(&mut self) {
        if self.panics_on_drop && !std::thread::panicking() {
            panic!("a value drop that panics, on purpose");
        }
    }
}

fn val(id: u64) -> Val {
    Val {
        id,
        panics_on_drop: false,
    }
}

macro_rules! lru_put_unwinding {
    ($($m:ident => $clone:ident, $hash:ident, $drop:ident;)*) => {$(
        /// The key's `Clone` panics in a `put` that would evict.
        #[test]
        fn $clone() {
            for capacity in [1u64, 3] {
                let mut cache = crate::$m::SimpleLruCache::<Key, Val>::new(capacity as usize);
                for i in 0..capacity {
                    cache.put(Key(i), val(i));
                }
                CLONE_PANICS.with(|flag| flag.set(true));
                let unwound = catch_unwind(AssertUnwindSafe(|| cache.put(Key(100), val(100))));
                CLONE_PANICS.with(|flag| flag.set(false));
                assert!(unwound.is_err());
                assert_eq!(cache.len(), capacity as usize);
                for i in 200..210 {
                    cache.put(Key(i), val(i));
                    assert!(cache.len() <= capacity as usize);
                }
                assert_eq!(cache.len(), capacity as usize);
                for i in 210 - capacity..210 {
                    assert_eq!(cache.get(&Key(i)).map(|v| v.id), Some(i));
                }
            }
        }

        /// The key's `Hash` panics where a `put` that adds a slot enters the
        /// key into the map.
        #[test]
        fn $hash() {
            for capacity in [1u64, 3] {
                let mut cache = crate::$m::SimpleLruCache::<Key, Val>::new(capacity as usize);
                for i in 0..capacity - 1 {
                    cache.put(Key(i), val(i));
                }
                // The lookup at the top of `put` hashes the key unless the map
                // is empty; the next hash is the one that enters it.
                HASH_PANICS_AFTER.with(|left| left.set(Some(u32::from(capacity > 1))));
                let unwound = catch_unwind(AssertUnwindSafe(|| cache.put(Key(100), val(100))));
                HASH_PANICS_AFTER.with(|left| left.set(None));
                assert!(unwound.is_err());
                assert!(cache.len() <= capacity as usize);
                for i in 200..210 {
                    cache.put(Key(i), val(i));
                    assert!(cache.len() <= capacity as usize);
                }
                assert_eq!(cache.len(), capacity as usize);
                for i in 210 - capacity..210 {
                    assert_eq!(cache.get(&Key(i)).map(|v| v.id), Some(i));
                }
            }
        }

        /// The evicted value's `Drop` panics: the cache keeps its capacity,
        /// and the new entry is already in place when that drop runs.
        #[test]
        fn $drop() {
            for capacity in [1u64, 3] {
                for check_new_entry_first in [false, true] {
                    let mut cache =
                        crate::$m::SimpleLruCache::<Key, Val>::new(capacity as usize);
                    cache.put(
                        Key(0),
                        Val {
                            id: 0,
                            panics_on_drop: true,
                        },
                    );
                    for i in 1..capacity {
                        cache.put(Key(i), val(i));
                    }
                    let unwound =
                        catch_unwind(AssertUnwindSafe(|| cache.put(Key(100), val(100))));
                    assert!(unwound.is_err());
                    assert_eq!(cache.len(), capacity as usize);
                    if check_new_entry_first {
                        assert_eq!(cache.get(&Key(100)).map(|v| v.id), Some(100));
                        assert!(cache.get(&Key(0)).is_none());
                    }
                    for i in 200..210 {
                        cache.put(Key(i), val(i));
                        assert!(cache.len() <= capacity as usize);
                    }
                    assert_eq!(cache.len(), capacity as usize);
                    for i in 210 - capacity..210 {
                        assert_eq!(cache.get(&Key(i)).map(|v| v.id), Some(i));
                    }
                }
            }
        }
    )*};
}

lru_put_unwinding! {
    arena => lru_clone_panic_arena, lru_hash_panic_arena, lru_drop_panic_arena;
    declaration => lru_clone_panic_declaration, lru_hash_panic_declaration, lru_drop_panic_declaration;
    equiv_manager => lru_clone_panic_equiv_manager, lru_hash_panic_equiv_manager, lru_drop_panic_equiv_manager;
    name => lru_clone_panic_name, lru_hash_panic_name, lru_drop_panic_name;
    prettyprint => lru_clone_panic_prettyprint, lru_hash_panic_prettyprint, lru_drop_panic_prettyprint;
}
