//! Tests of the types copied byte for byte into several kernel modules:
//! `VersionedRecord` (every copy against a verbatim copy of its earlier
//! `Vec`-backed form), `StackCalc` (every copy, including a short stack),
//! and `SimpleLruCache`, `Slot` and `MemoSlot` (every copy; the cache
//! against a verbatim copy of its earlier `put`).

/// The record as it was before it held its current value separately.
struct HistoryRecord<T: Clone> {
    history: Vec<T>,
}
impl<T: Clone> HistoryRecord<T> {
    fn new(initial: T) -> Self {
        Self {
            history: vec![initial],
        }
    }
    fn update(&mut self, val: T) {
        self.history.push(val);
    }
    fn current(&self) -> &T {
        self.history
            .last()
            .expect("VersionedRecord history is always non-empty after construction")
    }
    fn at_version(&self, n: usize) -> Option<&T> {
        self.history.get(n)
    }
    fn version(&self) -> usize {
        self.history.len() - 1
    }
    fn has_history(&self) -> bool {
        self.history.len() > 1
    }
}

/// A small deterministic generator (64-bit LCG, high bits) for the sequences.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }
}

/// A log of the ids of dropped [`Logged`] values, one log per test.
#[derive(Default)]
struct DropLog(std::rc::Rc<std::cell::RefCell<Vec<u32>>>);
impl DropLog {
    fn value(&self, id: u32) -> Logged {
        Logged {
            id,
            log: std::rc::Rc::clone(&self.0),
        }
    }
    fn dropped(&self) -> Vec<u32> {
        self.0.borrow().clone()
    }
}
/// A value that writes its id into its log when it is dropped.
#[derive(Clone)]
struct Logged {
    id: u32,
    log: std::rc::Rc<std::cell::RefCell<Vec<u32>>>,
}
impl Drop for Logged {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.id);
    }
}

/// Every accessor of `new` agrees with the history-backed form after each
/// update, on 200 seeded sequences of up to 12 updates, probing versions past
/// the end as well; and `update` drops no value, while a dropped record drops
/// its values in the order the history-backed form's `Vec` drops them, oldest
/// first.
macro_rules! versioned_record_agrees {
    ($($name:ident => $m:ident),* $(,)?) => {$(
        mod $name {
            #[test]
            fn drops_values_oldest_first() {
                use crate::$m::VersionedRecord;
                for updates in 0..4u32 {
                    let new_log = super::DropLog::default();
                    let old_log = super::DropLog::default();
                    let mut new = VersionedRecord::new(new_log.value(0));
                    let mut old = super::HistoryRecord::new(old_log.value(0));
                    for id in 1..=updates {
                        new.update(new_log.value(id));
                        old.update(old_log.value(id));
                    }
                    assert!(new_log.dropped().is_empty());
                    drop(new);
                    drop(old);
                    assert_eq!(new_log.dropped(), old_log.dropped());
                    assert_eq!(new_log.dropped(), (0..=updates).collect::<Vec<_>>());
                }
            }
        }

        #[test]
        fn $name() {
            use crate::$m::VersionedRecord;
            let mut rng = Lcg(0x5eed);
            for _ in 0..200 {
                let first = rng.next();
                let mut new = VersionedRecord::new(first);
                let mut old = HistoryRecord::new(first);
                let updates = rng.next() % 13;
                for _ in 0..=updates {
                    assert_eq!(new.current(), old.current());
                    assert_eq!(new.version(), old.version());
                    assert_eq!(new.has_history(), old.has_history());
                    for n in 0..old.version() + 3 {
                        assert_eq!(new.at_version(n), old.at_version(n));
                    }
                    let value = rng.next();
                    new.update(value);
                    old.update(value);
                }
            }
        }
    )*};
}

versioned_record_agrees! {
    versioned_record_abstract => r#abstract,
    versioned_record_alpha => alpha,
    versioned_record_axiom => axiom,
    versioned_record_beta => beta,
    versioned_record_builtin => builtin,
    versioned_record_cache => cache,
    versioned_record_check => check,
    versioned_record_congruence => congruence,
    versioned_record_context => context,
    versioned_record_conversion => conversion,
    versioned_record_core_types => core_types,
    versioned_record_def_eq => def_eq,
    versioned_record_env => env,
    versioned_record_error => error,
    versioned_record_eta => eta,
    versioned_record_export => export,
    versioned_record_expr => expr,
    versioned_record_expr_cache => expr_cache,
    versioned_record_expr_util => expr_util,
    versioned_record_ffi => ffi,
    versioned_record_inductive => inductive,
    versioned_record_infer => infer,
    versioned_record_instantiate => instantiate,
    versioned_record_level => level,
    versioned_record_match_compile => match_compile,
    versioned_record_normalize => normalize,
    versioned_record_proof => proof,
    versioned_record_quotient => quotient,
    versioned_record_reduce => reduce,
    versioned_record_reduction => reduction,
    versioned_record_simp => simp,
    versioned_record_subst => subst,
    versioned_record_substitution => substitution,
    versioned_record_termination => termination,
    versioned_record_trace => trace,
    versioned_record_typeclasses => typeclasses,
    versioned_record_universe => universe,
    versioned_record_whnf => whnf,
}

/// `add`, `sub` and `mul` return the value that replaces the top two, in the
/// order the earlier version computed it (`second op top`), and on a stack of
/// fewer than two values return `None` and leave the stack as it was.
macro_rules! stack_calc_ops {
    ($($name:ident => $m:ident),* $(,)?) => {$(
        #[test]
        fn $name() {
            use crate::$m::StackCalc;
            let mut calc = StackCalc::new();
            assert_eq!(calc.add(), None);
            assert_eq!(calc.sub(), None);
            assert_eq!(calc.mul(), None);
            assert_eq!(calc.depth(), 0);
            calc.push(9);
            assert_eq!(calc.add(), None);
            assert_eq!(calc.sub(), None);
            assert_eq!(calc.mul(), None);
            assert_eq!((calc.depth(), calc.peek()), (1, Some(9)));
            calc.push(4);
            assert_eq!(calc.sub(), Some(5));
            assert_eq!((calc.depth(), calc.peek()), (1, Some(5)));
            calc.push(-3);
            assert_eq!(calc.mul(), Some(-15));
            calc.push(20);
            assert_eq!(calc.add(), Some(5));
            assert_eq!((calc.depth(), calc.peek()), (1, Some(5)));
            calc.push(1);
            calc.push(2);
            calc.push(3);
            assert_eq!(calc.sub(), Some(-1));
            assert_eq!(calc.depth(), 3);
            assert_eq!(calc.mul(), Some(-1));
            assert_eq!(calc.add(), Some(4));
            assert_eq!((calc.depth(), calc.peek()), (1, Some(4)));
        }
    )*};
}

stack_calc_ops! {
    stack_calc_abstract => r#abstract,
    stack_calc_alpha => alpha,
    stack_calc_axiom => axiom,
    stack_calc_beta => beta,
    stack_calc_builtin => builtin,
    stack_calc_cache => cache,
    stack_calc_check => check,
    stack_calc_congruence => congruence,
    stack_calc_context => context,
    stack_calc_conversion => conversion,
    stack_calc_core_types => core_types,
    stack_calc_def_eq => def_eq,
    stack_calc_env => env,
    stack_calc_error => error,
    stack_calc_expr => expr,
    stack_calc_expr_cache => expr_cache,
    stack_calc_expr_util => expr_util,
    stack_calc_ffi => ffi,
    stack_calc_infer => infer,
    stack_calc_instantiate => instantiate,
    stack_calc_level => level,
    stack_calc_match_compile => match_compile,
    stack_calc_normalize => normalize,
    stack_calc_proof => proof,
    stack_calc_reduce => reduce,
    stack_calc_reduction => reduction,
    stack_calc_simp => simp,
    stack_calc_subst => subst,
    stack_calc_substitution => substitution,
    stack_calc_termination => termination,
    stack_calc_trace => trace,
    stack_calc_typeclasses => typeclasses,
    stack_calc_universe => universe,
    stack_calc_whnf => whnf,
}

/// The cache as it was, with the eviction branch's earlier `expect`.
struct OldLru<K: std::hash::Hash + Eq + Clone, V: Clone> {
    capacity: usize,
    map: std::collections::HashMap<K, usize>,
    keys: Vec<K>,
    vals: Vec<V>,
    order: Vec<usize>,
}
impl<K: std::hash::Hash + Eq + Clone, V: Clone> OldLru<K, V> {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            map: std::collections::HashMap::new(),
            keys: Vec::new(),
            vals: Vec::new(),
            order: Vec::new(),
        }
    }
    fn put(&mut self, key: K, val: V) {
        if let Some(&idx) = self.map.get(&key) {
            self.vals[idx] = val;
            self.order.retain(|&x| x != idx);
            self.order.insert(0, idx);
            return;
        }
        if self.keys.len() >= self.capacity {
            let evict_idx = *self
                .order
                .last()
                .expect("order list must be non-empty before eviction");
            self.map.remove(&self.keys[evict_idx]);
            self.order.pop();
            self.keys[evict_idx] = key.clone();
            self.vals[evict_idx] = val;
            self.map.insert(key, evict_idx);
            self.order.insert(0, evict_idx);
        } else {
            let idx = self.keys.len();
            self.keys.push(key.clone());
            self.vals.push(val);
            self.map.insert(key, idx);
            self.order.insert(0, idx);
        }
    }
    fn get(&mut self, key: &K) -> Option<&V> {
        let idx = *self.map.get(key)?;
        self.order.retain(|&x| x != idx);
        self.order.insert(0, idx);
        Some(&self.vals[idx])
    }
    fn len(&self) -> usize {
        self.keys.len()
    }
}

/// Every copy answers `get`, `len` and `is_empty` as the earlier cache does,
/// on 300 seeded sequences of 60 `put` / `get` operations over 8 keys at
/// capacities 1 to 5; `Slot` and `MemoSlot` call their initialiser once and
/// hand back the stored value.
macro_rules! copied_cache_types {
    ($($name:ident => $m:ident),* $(,)?) => {$(
        #[test]
        fn $name() {
            use crate::$m as m;
            let mut rng = Lcg(0xcac4e);
            for round in 0..300u64 {
                let capacity = (round % 5 + 1) as usize;
                let mut new = m::SimpleLruCache::<u64, u64>::new(capacity);
                let mut old = OldLru::<u64, u64>::new(capacity);
                for _ in 0..60 {
                    let key = rng.next() % 8;
                    if rng.next() % 3 == 0 {
                        assert_eq!(new.get(&key), old.get(&key));
                    } else {
                        let val = rng.next();
                        new.put(key, val);
                        old.put(key, val);
                    }
                    assert_eq!(new.len(), old.len());
                    assert_eq!(new.is_empty(), old.len() == 0);
                }
                for key in 0..8 {
                    assert_eq!(new.get(&key), old.get(&key));
                }
            }

            let mut calls = 0;
            let mut slot = m::Slot::empty();
            assert_eq!(*slot.get_or_fill_with(|| { calls += 1; 7 }), 7);
            assert_eq!(*slot.get_or_fill_with(|| { calls += 1; 8 }), 7);
            assert_eq!((calls, slot.get()), (1, Some(&7)));

            let mut memo = m::MemoSlot::new();
            assert!(!memo.is_cached());
            assert_eq!(*memo.get_or_compute(|| { calls += 1; 11 }), 11);
            assert_eq!(*memo.get_or_compute(|| { calls += 1; 12 }), 11);
            assert_eq!(calls, 2);
            memo.invalidate();
            assert_eq!(*memo.get_or_compute(|| { calls += 1; 13 }), 13);
            assert_eq!(calls, 3);
        }
    )*};
}

copied_cache_types! {
    copied_cache_types_arena => arena,
    copied_cache_types_declaration => declaration,
    copied_cache_types_equiv_manager => equiv_manager,
    copied_cache_types_name => name,
    copied_cache_types_prettyprint => prettyprint,
}
