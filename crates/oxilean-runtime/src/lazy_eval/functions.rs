//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::cell::RefCell;
use std::sync::{Arc, Mutex, OnceLock};

use super::types::{
    BatchEval, BlackHoleDetector, CycleSafeMemo, DeferredValue, FutureChain, LazyAccumulator,
    LazyCell, LazyFilter, LazyList, LazyMap, LazyMap2, LazyRange, LazySieve, LazyString, LazyTree,
    MemoCache, MemoFn, MemoFn2, MemoTable, Once, SharedThunk, StreamThunk, TakeIter, Thunk,
    ThunkCache, ThunkVec, TrackedCell,
};

/// The lazy tail function type used inside [`TakeIter`].
pub(super) type TailFn<T> = Arc<dyn Fn() -> LazyList<T> + Send + Sync>;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_thunk_new_and_force() {
        let _calls = RefCell::new(0u32);
        let thunk = Thunk::new(|| 42u64);
        let v = *thunk.force();
        assert_eq!(v, 42);
    }
    #[test]
    fn test_thunk_memoized() {
        use std::sync::atomic::{AtomicU32, Ordering};
        let counter = Arc::new(AtomicU32::new(0));
        let c2 = counter.clone();
        let thunk = Thunk::new(move || {
            c2.fetch_add(1, Ordering::SeqCst);
            42u64
        });
        let _ = thunk.force();
        let _ = thunk.force();
        let _ = thunk.force();
        assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "fn called more than once"
        );
    }
    #[test]
    fn test_thunk_pure() {
        let thunk = Thunk::<u64>::pure(99);
        assert!(thunk.is_evaluated());
        assert_eq!(*thunk.force(), 99);
    }
    #[test]
    fn test_thunk_is_evaluated() {
        let thunk = Thunk::new(|| 1u64);
        assert!(!thunk.is_evaluated());
        let _ = thunk.force();
        assert!(thunk.is_evaluated());
    }
    #[test]
    fn test_shared_thunk_force() {
        let thunk = SharedThunk::new(|| 42u64);
        assert_eq!(thunk.force(), 42);
        assert_eq!(thunk.force(), 42);
    }
    #[test]
    fn test_shared_thunk_pure() {
        let thunk = SharedThunk::pure(7u64);
        assert!(thunk.is_evaluated());
        assert_eq!(thunk.force(), 7);
    }
    #[test]
    fn test_shared_thunk_share() {
        let t1 = SharedThunk::new(|| 100u64);
        let t2 = t1.share();
        assert_eq!(t1.force(), 100);
        assert!(t2.is_evaluated());
    }
    #[test]
    fn test_thunk_cache_insert_and_force() {
        let mut cache = ThunkCache::new();
        cache.insert("answer", || 42u64);
        let v: u64 = cache
            .force("answer")
            .expect("test operation should succeed");
        assert_eq!(v, 42);
    }
    #[test]
    fn test_thunk_cache_insert_pure() {
        let mut cache = ThunkCache::new();
        cache.insert_pure("pi_approx", 3u64);
        assert!(cache.contains("pi_approx"));
        let v: u64 = cache
            .force("pi_approx")
            .expect("test operation should succeed");
        assert_eq!(v, 3);
    }
    #[test]
    fn test_thunk_cache_len() {
        let mut cache = ThunkCache::new();
        assert!(cache.is_empty());
        cache.insert("a", || 1u64);
        cache.insert("b", || 2u64);
        assert_eq!(cache.len(), 2);
    }
    #[test]
    fn test_thunk_cache_missing_key() {
        let cache = ThunkCache::new();
        let v: Option<u64> = cache.force("missing");
        assert!(v.is_none());
    }
    #[test]
    fn test_lazy_list_take() {
        let nats = LazyList::from_fn(0u64, |n| n + 1);
        let first5: Vec<u64> = nats.take(5).collect();
        assert_eq!(first5, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_lazy_list_empty() {
        let empty: LazyList<u64> = LazyList::empty();
        assert!(empty.is_empty());
        let v: Vec<u64> = empty.take(10).collect();
        assert!(v.is_empty());
    }
    #[test]
    fn test_lazy_list_cons() {
        let list = LazyList::cons(1u64, || LazyList::cons(2, LazyList::empty));
        let v: Vec<u64> = list.take(5).collect();
        assert_eq!(v, vec![1, 2]);
    }
    #[test]
    fn test_memo_fn_basic() {
        let mut f = MemoFn::new(|n: u64| n * n);
        assert_eq!(f.call(5), 25);
        assert_eq!(f.call(5), 25);
        assert_eq!(f.cache_size(), 1);
    }
    #[test]
    fn test_memo_fn_multiple_keys() {
        let mut f = MemoFn::new(|n: u64| n + 100);
        let _ = f.call(1);
        let _ = f.call(2);
        let _ = f.call(3);
        assert_eq!(f.cache_size(), 3);
    }
    #[test]
    fn test_memo_fn_clear_cache() {
        let mut f = MemoFn::new(|n: u64| n);
        let _ = f.call(1);
        f.clear_cache();
        assert_eq!(f.cache_size(), 0);
    }
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_lazy_cell_uninit() {
        let cell = LazyCell::<u64>::new();
        assert!(!cell.is_initialized());
        assert!(cell.get().is_none());
    }
    #[test]
    fn test_lazy_cell_init() {
        let cell = LazyCell::<u64>::new();
        cell.init(42).expect("test operation should succeed");
        assert!(cell.is_initialized());
        assert_eq!(*cell.get().expect("key should exist in map"), 42);
    }
    #[test]
    fn test_lazy_cell_double_init() {
        let cell = LazyCell::<u64>::new();
        cell.init(1).expect("test operation should succeed");
        let err = cell.init(2);
        assert!(err.is_err());
        assert_eq!(*cell.get().expect("key should exist in map"), 1);
    }
    #[test]
    fn test_lazy_cell_get_or_init() {
        let cell = LazyCell::<u64>::new();
        let v = cell.get_or_init(|| 99);
        assert_eq!(*v, 99);
        let v2 = cell.get_or_init(|| 100);
        assert_eq!(*v2, 99);
    }
    #[test]
    fn test_stream_thunk_naturals() {
        let mut stream = StreamThunk::new(0u64, |s| (*s, *s + 1));
        let first5 = stream.take_n(5);
        assert_eq!(first5, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_stream_thunk_fibonacci() {
        let mut fib = StreamThunk::new((0u64, 1u64), |&(a, b)| (a, (b, a + b)));
        let fibs = fib.take_n(8);
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }
    #[test]
    fn test_stream_thunk_state() {
        let stream = StreamThunk::new(42u64, |s| (*s, *s));
        assert_eq!(*stream.current_state(), 42);
    }
    #[test]
    fn test_deferred_value_ready() {
        let v: DeferredValue<u64> = DeferredValue::Ready(42);
        assert!(v.is_ready());
        assert_eq!(*v.get().expect("key should exist in map"), 42);
    }
    #[test]
    fn test_deferred_value_deferred() {
        let v: DeferredValue<u64> = DeferredValue::Deferred {
            name: "x".to_string(),
        };
        assert!(v.is_deferred());
        assert!(v.get().is_none());
    }
    #[test]
    fn test_deferred_value_failed() {
        let v: DeferredValue<u64> = DeferredValue::Failed {
            error: "oops".to_string(),
        };
        assert!(v.is_failed());
    }
    #[test]
    fn test_deferred_value_map() {
        let v: DeferredValue<u64> = DeferredValue::Ready(10);
        let v2 = v.map(|x| x * 2);
        assert_eq!(*v2.get().expect("key should exist in map"), 20);
    }
    #[test]
    fn test_deferred_value_map_non_ready() {
        let v: DeferredValue<u64> = DeferredValue::Deferred {
            name: "y".to_string(),
        };
        let v2 = v.map(|x| x + 1);
        assert!(v2.is_deferred());
    }
    #[test]
    fn test_future_chain_no_steps() {
        let chain = FutureChain::new(|| 42u64);
        assert_eq!(chain.step_count(), 0);
        assert_eq!(chain.force(), 42);
    }
    #[test]
    fn test_future_chain_multiple_steps() {
        let chain = FutureChain::new(|| 0u64)
            .then(|x| x + 10)
            .then(|x| x * 2)
            .then(|x| x + 1);
        assert_eq!(chain.step_count(), 3);
        assert_eq!(chain.force(), 21);
    }
    #[test]
    fn test_lazy_map_insert_lazy() {
        let mut m = LazyMap::<String, u64>::new();
        m.insert_lazy("answer".to_string(), || 42);
        assert!(m.contains(&"answer".to_string()));
        assert_eq!(m.pending_count(), 1);
        assert_eq!(m.computed_count(), 0);
    }
    #[test]
    fn test_lazy_map_get_forces() {
        let mut m = LazyMap::<String, u64>::new();
        m.insert_lazy("x".to_string(), || 100);
        let val = m.get(&"x".to_string());
        assert_eq!(*val.expect("test operation should succeed"), 100);
        assert_eq!(m.pending_count(), 0);
        assert_eq!(m.computed_count(), 1);
    }
    #[test]
    fn test_lazy_map_insert_ready() {
        let mut m = LazyMap::<String, u64>::new();
        m.insert_ready("y".to_string(), 50);
        let val = m.get(&"y".to_string());
        assert_eq!(*val.expect("test operation should succeed"), 50);
    }
    #[test]
    fn test_lazy_map_len() {
        let mut m = LazyMap::<String, u64>::new();
        m.insert_lazy("a".to_string(), || 1);
        m.insert_ready("b".to_string(), 2);
        assert_eq!(m.len(), 2);
    }
    #[test]
    fn test_black_hole_detector_no_cycle() {
        let mut d = BlackHoleDetector::new();
        d.enter("thunkA").expect("test operation should succeed");
        assert_eq!(d.depth(), 1);
        d.exit("thunkA");
        assert_eq!(d.depth(), 0);
    }
    #[test]
    fn test_black_hole_detector_cycle() {
        let mut d = BlackHoleDetector::new();
        d.enter("thunkB").expect("test operation should succeed");
        let result = d.enter("thunkB");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("black hole"));
    }
    #[test]
    fn test_black_hole_detector_is_in_progress() {
        let mut d = BlackHoleDetector::new();
        d.enter("thunkC").expect("test operation should succeed");
        assert!(d.is_in_progress("thunkC"));
        assert!(!d.is_in_progress("thunkD"));
    }
    #[test]
    fn test_memo_table_insert_get() {
        let mut table = MemoTable::new();
        table.insert("key", 42u64);
        assert_eq!(
            *table
                .get::<u64>("key")
                .expect("test operation should succeed"),
            42
        );
    }
    #[test]
    fn test_memo_table_wrong_type() {
        let mut table = MemoTable::new();
        table.insert("key", 42u64);
        let as_bool: Option<&bool> = table.get::<bool>("key");
        assert!(as_bool.is_none());
    }
    #[test]
    fn test_memo_table_missing() {
        let table = MemoTable::new();
        let v: Option<&u64> = table.get("missing");
        assert!(v.is_none());
    }
    #[test]
    fn test_memo_table_remove() {
        let mut table = MemoTable::new();
        table.insert("x", 1u64);
        assert!(table.remove("x"));
        assert!(!table.contains("x"));
        assert!(!table.remove("x"));
    }
    #[test]
    fn test_memo_table_clear() {
        let mut table = MemoTable::new();
        table.insert("a", 1u64);
        table.insert("b", 2u64);
        table.clear();
        assert!(table.is_empty());
    }
    #[test]
    fn test_memo_table_debug() {
        let mut table = MemoTable::new();
        table.insert("a", 1u64);
        let s = format!("{:?}", table);
        assert!(s.contains("1 entries"));
    }
    #[test]
    fn test_lazy_cell_debug_uninitialized() {
        let cell = LazyCell::<u64>::new();
        let s = format!("{:?}", cell);
        assert!(s.contains("Uninitialized"));
    }
    #[test]
    fn test_lazy_cell_debug_initialized() {
        let cell = LazyCell::<u64>::new();
        cell.init(7).expect("test operation should succeed");
        let s = format!("{:?}", cell);
        assert!(s.contains("Initialized"));
    }
}
/// Compute a value lazily only if `cond` is true.
#[allow(dead_code)]
pub fn lazy_if<T, F: FnOnce() -> T>(cond: bool, f: F) -> Option<T> {
    if cond {
        Some(f())
    } else {
        None
    }
}
/// Compute a value lazily, returning `default` if the result would be `None`.
#[allow(dead_code)]
pub fn lazy_or_default<T: Default, F: FnOnce() -> Option<T>>(f: F) -> T {
    f().unwrap_or_default()
}
#[cfg(test)]
mod tests_extended2 {
    use super::*;
    #[test]
    fn test_batch_eval_basic() {
        let mut batch = BatchEval::<u64>::new();
        batch.add("a", || 1);
        batch.add("b", || 2);
        batch.add("c", || 3);
        assert_eq!(batch.pending(), 3);
        batch.run_all();
        assert_eq!(batch.pending(), 0);
        assert_eq!(batch.completed(), 3);
    }
    #[test]
    fn test_batch_eval_result() {
        let mut batch = BatchEval::<u64>::new();
        batch.add("ans", || 42);
        batch.run_all();
        assert_eq!(
            *batch.result("ans").expect("test operation should succeed"),
            42
        );
    }
    #[test]
    fn test_batch_eval_missing_result() {
        let batch = BatchEval::<u64>::new();
        assert!(batch.result("missing").is_none());
    }
    #[test]
    fn test_batch_eval_all_results() {
        let mut batch = BatchEval::<u64>::new();
        batch.add("x", || 10);
        batch.add("y", || 20);
        batch.run_all();
        assert_eq!(batch.all_results().len(), 2);
    }
    #[test]
    fn test_lazy_if_true() {
        let v = lazy_if(true, || 42u64);
        assert_eq!(v, Some(42));
    }
    #[test]
    fn test_lazy_if_false() {
        let v = lazy_if(false, || 42u64);
        assert!(v.is_none());
    }
    #[test]
    fn test_lazy_or_default_some() {
        let v = lazy_or_default(|| Some(7u64));
        assert_eq!(v, 7);
    }
    #[test]
    fn test_lazy_or_default_none() {
        let v = lazy_or_default::<u64, _>(|| None);
        assert_eq!(v, 0);
    }
    #[test]
    fn test_memo_fn2_basic() {
        let mut f = MemoFn2::new(|a: u64, b: u64| a + b);
        assert_eq!(f.call(3, 4), 7);
        assert_eq!(f.call(3, 4), 7);
        assert_eq!(f.cache_size(), 1);
    }
    #[test]
    fn test_memo_fn2_multiple_keys() {
        let mut f = MemoFn2::new(|a: u64, b: u64| a * b);
        f.call(2, 3);
        f.call(4, 5);
        assert_eq!(f.cache_size(), 2);
    }
    #[test]
    fn test_memo_fn2_clear_cache() {
        let mut f = MemoFn2::new(|a: u64, b: u64| a + b);
        f.call(1, 1);
        f.clear_cache();
        assert_eq!(f.cache_size(), 0);
    }
    #[test]
    fn test_thunk_vec_push_ready() {
        let mut v = ThunkVec::<u64>::new();
        v.push_ready(42);
        assert_eq!(v.len(), 1);
        assert_eq!(v.get(0), Some(42));
    }
    #[test]
    fn test_thunk_vec_empty() {
        let v = ThunkVec::<u64>::new();
        assert!(v.is_empty());
        assert!(v.get(0).is_none());
    }
    #[test]
    fn test_thunk_vec_force_all() {
        let mut v = ThunkVec::<u64>::new();
        v.push_ready(1);
        v.push_ready(2);
        v.push_ready(3);
        let all = v.force_all();
        assert_eq!(all, vec![1, 2, 3]);
    }
    #[test]
    fn test_cycle_safe_memo_basic() {
        let mut memo = CycleSafeMemo::<u64>::new();
        let v = memo.get_or_compute("x", |_| 99);
        assert_eq!(v, 99);
        let v2 = memo.get_or_compute("x", |_| 0);
        assert_eq!(v2, 99);
    }
    #[test]
    fn test_cycle_safe_memo_cycle() {
        let mut memo = CycleSafeMemo::<u64>::new();
        let v = memo.get_or_compute("a", |m| {
            let inner = m.get_or_compute("a", |_| 1000);
            inner + 1
        });
        assert_eq!(v, 1);
    }
    #[test]
    fn test_cycle_safe_memo_cache_size() {
        let mut memo = CycleSafeMemo::<u64>::new();
        memo.get_or_compute("a", |_| 1);
        memo.get_or_compute("b", |_| 2);
        assert_eq!(memo.cache_size(), 2);
    }
    #[test]
    fn test_black_hole_detector_multiple_entries() {
        let mut d = BlackHoleDetector::new();
        d.enter("a").expect("test operation should succeed");
        d.enter("b").expect("test operation should succeed");
        d.enter("c").expect("test operation should succeed");
        assert_eq!(d.depth(), 3);
        d.exit("b");
        assert_eq!(d.depth(), 2);
        assert!(!d.is_in_progress("b"));
    }
    #[test]
    fn test_lazy_map_missing_key() {
        let mut m = LazyMap::<String, u64>::new();
        assert!(m.get(&"missing".to_string()).is_none());
    }
    #[test]
    fn test_lazy_map_overwrite() {
        let mut m = LazyMap::<String, u64>::new();
        m.insert_ready("k".to_string(), 1);
        m.insert_ready("k".to_string(), 2);
        assert_eq!(
            *m.get(&"k".to_string()).expect("key should exist in map"),
            2
        );
    }
    #[test]
    fn test_future_chain_identity() {
        let chain = FutureChain::new(|| 42u64).then(|x| x);
        assert_eq!(chain.force(), 42);
    }
    #[test]
    fn test_future_chain_string() {
        let chain = FutureChain::new(|| "hello".to_string()).then(|s| format!("{} world", s));
        assert_eq!(chain.force(), "hello world");
    }
}
#[cfg(test)]
mod tests_lazy_extra {
    use super::*;
    #[test]
    fn test_lazy_tree_leaf() {
        let leaf = LazyTree::leaf(42u64);
        assert!(leaf.is_leaf());
        assert!(leaf.children().is_empty());
        assert_eq!(leaf.value, 42);
    }
    #[test]
    fn test_lazy_tree_node_with_children() {
        let tree = LazyTree::node(1u64, || vec![LazyTree::leaf(2), LazyTree::leaf(3)]);
        assert!(!tree.is_leaf());
        let children = tree.children();
        assert_eq!(children.len(), 2);
    }
    #[test]
    fn test_lazy_tree_dfs() {
        let tree = LazyTree::node(1u64, || {
            vec![
                LazyTree::node(2, || vec![LazyTree::leaf(4), LazyTree::leaf(5)]),
                LazyTree::leaf(3),
            ]
        });
        let dfs = tree.dfs();
        assert_eq!(dfs, vec![1, 2, 4, 5, 3]);
    }
    #[test]
    fn test_lazy_sieve_first_prime() {
        let mut sieve = LazySieve::new();
        assert_eq!(sieve.next_prime(), 2);
    }
    #[test]
    fn test_lazy_sieve_first_10_primes() {
        let mut sieve = LazySieve::new();
        let primes = sieve.take_primes(10);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
    #[test]
    fn test_lazy_sieve_default() {
        let mut s = LazySieve::default();
        assert_eq!(s.next_prime(), 2);
    }
    #[test]
    fn test_once_get_or_init() {
        let once = Once::<u64>::new();
        let v = once.get_or_init(|| 42);
        assert_eq!(*v, 42);
        let v2 = once.get_or_init(|| 100);
        assert_eq!(*v2, 42);
    }
    #[test]
    fn test_once_is_initialized() {
        let once = Once::<u64>::new();
        assert!(!once.is_initialized());
        once.get_or_init(|| 1);
        assert!(once.is_initialized());
    }
    #[test]
    fn test_once_debug_uninitialized() {
        let once = Once::<u64>::new();
        assert!(format!("{:?}", once).contains("Uninitialized"));
    }
    #[test]
    fn test_once_debug_initialized() {
        let once = Once::<u64>::new();
        once.get_or_init(|| 7);
        assert!(format!("{:?}", once).contains("Initialized"));
    }
    #[test]
    fn test_lazy_map2_get() {
        let data = vec![1u64, 2, 3, 4];
        let lm = LazyMap2::new(&data, |x| x * 2);
        assert_eq!(lm.get(0), Some(2));
        assert_eq!(lm.get(3), Some(8));
        assert_eq!(lm.get(4), None);
    }
    #[test]
    fn test_lazy_map2_collect_all() {
        let data = vec![1u64, 2, 3];
        let lm = LazyMap2::new(&data, |x| x + 10);
        assert_eq!(lm.collect_all(), vec![11, 12, 13]);
    }
    #[test]
    fn test_lazy_map2_iterator() {
        let data = vec![1u64, 2, 3];
        let lm = LazyMap2::new(&data, |x| x * x);
        let collected: Vec<_> = lm.collect();
        assert_eq!(collected, vec![1, 4, 9]);
    }
    #[test]
    fn test_lazy_filter_collect_all() {
        let data = vec![1u64, 2, 3, 4, 5, 6];
        let lf = LazyFilter::new(&data, |x| x % 2 == 0);
        assert_eq!(lf.collect_all(), vec![2, 4, 6]);
    }
    #[test]
    fn test_lazy_filter_iterator() {
        let data = vec![1u64, 2, 3, 4, 5];
        let lf = LazyFilter::new(&data, |x| *x > 3);
        let v: Vec<_> = lf.collect();
        assert_eq!(v, vec![4, 5]);
    }
    #[test]
    fn test_lazy_filter_empty_result() {
        let data = vec![1u64, 3, 5];
        let lf = LazyFilter::new(&data, |x| x % 2 == 0);
        assert!(lf.collect_all().is_empty());
    }
    #[test]
    fn test_memo_fn_string_key() {
        let mut f = MemoFn::new(|s: String| s.len());
        assert_eq!(f.call("hello".to_string()), 5);
        assert_eq!(f.call("hello".to_string()), 5);
        assert_eq!(f.cache_size(), 1);
    }
    #[test]
    fn test_thunk_chained() {
        let t1 = Thunk::new(|| 10u64);
        let v1 = *t1.force();
        let t2 = Thunk::new(move || v1 + 5);
        assert_eq!(*t2.force(), 15);
    }
    #[test]
    fn test_lazy_list_head_empty() {
        let empty: LazyList<u64> = LazyList::empty();
        assert!(empty.head().is_none());
    }
    #[test]
    fn test_lazy_list_take_zero() {
        let nats = LazyList::from_fn(0u64, |n| n + 1);
        let v: Vec<u64> = nats.take(0).collect();
        assert!(v.is_empty());
    }
    #[test]
    fn test_lazy_list_squares() {
        let squares = LazyList::from_fn(0u64, |n| n + 1);
        let first5: Vec<u64> = squares.take(5).collect();
        assert_eq!(first5, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_batch_eval_empty() {
        let mut batch = BatchEval::<u64>::new();
        batch.run_all();
        assert_eq!(batch.completed(), 0);
    }
}
#[cfg(test)]
mod tests_lazy_final {
    use super::*;
    #[test]
    fn test_lazy_string_empty() {
        let s = LazyString::new().build();
        assert_eq!(s, "");
    }
    #[test]
    fn test_lazy_string_concat() {
        let s = LazyString::new()
            .push("hello")
            .push(", ")
            .push("world")
            .build();
        assert_eq!(s, "hello, world");
    }
    #[test]
    fn test_lazy_string_part_count() {
        let ls = LazyString::new().push("a").push("b").push("c");
        assert_eq!(ls.part_count(), 3);
    }
    #[test]
    fn test_lazy_accumulator_add_flush() {
        let mut acc = LazyAccumulator::<u64>::new();
        acc.add(|| 1);
        acc.add(|| 2);
        acc.add(|| 3);
        assert_eq!(acc.pending_count(), 3);
        acc.flush();
        assert_eq!(acc.pending_count(), 0);
        assert_eq!(acc.collected_count(), 3);
    }
    #[test]
    fn test_lazy_accumulator_collect() {
        let mut acc = LazyAccumulator::<u64>::new();
        acc.add(|| 10);
        acc.add(|| 20);
        let v = acc.collect().to_vec();
        assert_eq!(v.len(), 2);
    }
    #[test]
    fn test_lazy_range_basic() {
        let r = LazyRange::up_to(5);
        let v: Vec<i64> = r.collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_lazy_range_with_step() {
        let r = LazyRange::new(0, 10, 2);
        let v: Vec<i64> = r.collect();
        assert_eq!(v, vec![0, 2, 4, 6, 8]);
    }
    #[test]
    fn test_lazy_range_descending() {
        let r = LazyRange::new(5, 0, -1);
        let v: Vec<i64> = r.collect();
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }
    #[test]
    fn test_lazy_range_collect_all() {
        let r = LazyRange::new(10, 13, 1);
        assert_eq!(r.collect_all(), vec![10, 11, 12]);
    }
    #[test]
    fn test_stream_thunk_state_changes() {
        let mut s = StreamThunk::new(0u64, |n| (*n, n + 1));
        s.next();
        s.next();
        assert_eq!(*s.current_state(), 2);
    }
    #[test]
    fn test_lazy_cell_default() {
        let cell = LazyCell::<u64>::default();
        assert!(!cell.is_initialized());
    }
    #[test]
    fn test_memo_fn_single_call() {
        let mut f = MemoFn::new(|n: u64| n + 1);
        assert_eq!(f.call(41), 42);
        assert_eq!(f.cache_size(), 1);
    }
    #[test]
    fn test_deferred_value_failed_map() {
        let v: DeferredValue<u64> = DeferredValue::Failed {
            error: "err".to_string(),
        };
        let v2 = v.map(|x| x + 1);
        assert!(v2.is_failed());
    }
}
/// Apply a cumulative fold lazily, producing each intermediate result.
#[allow(dead_code)]
pub fn lazy_scan<T, B, F>(data: &[T], init: B, f: F) -> Vec<B>
where
    B: Clone,
    F: Fn(B, &T) -> B,
{
    let mut acc = init;
    let mut result = vec![acc.clone()];
    for item in data {
        acc = f(acc, item);
        result.push(acc.clone());
    }
    result
}
/// Lazily compute prefix sums over a slice of u64.
#[allow(dead_code)]
pub fn prefix_sums(data: &[u64]) -> Vec<u64> {
    lazy_scan(data, 0u64, |acc, x| acc + x)
}
/// Lazily compute prefix products.
#[allow(dead_code)]
pub fn prefix_products(data: &[u64]) -> Vec<u64> {
    lazy_scan(data, 1u64, |acc, x| acc * x)
}
#[cfg(test)]
mod tests_scan {
    use super::*;
    #[test]
    fn test_prefix_sums() {
        let v = prefix_sums(&[1, 2, 3, 4]);
        assert_eq!(v, vec![0, 1, 3, 6, 10]);
    }
    #[test]
    fn test_prefix_products() {
        let v = prefix_products(&[1, 2, 3, 4]);
        assert_eq!(v, vec![1, 1, 2, 6, 24]);
    }
    #[test]
    fn test_lazy_scan_max() {
        let v = lazy_scan(&[3u64, 1, 4, 1, 5, 9], 0u64, |acc, x| acc.max(*x));
        assert_eq!(v, vec![0, 3, 3, 4, 4, 5, 9]);
    }
    #[test]
    fn test_prefix_sums_empty() {
        let v = prefix_sums(&[]);
        assert_eq!(v, vec![0]);
    }
    #[test]
    fn test_lazy_accumulator_empty() {
        let mut acc = LazyAccumulator::<u64>::new();
        let result = acc.collect().to_vec();
        assert!(result.is_empty());
    }
    #[test]
    fn test_lazy_string_default() {
        let s = LazyString::default().build();
        assert!(s.is_empty());
    }
}
#[cfg(test)]
mod extra_lazy_tests {
    use super::*;
    #[test]
    fn test_memo_cache_hit_miss() {
        let mut cache: MemoCache<i32> = MemoCache::new();
        assert_eq!(cache.get(1), None);
        cache.insert(1, 42);
        assert_eq!(cache.get(1), Some(&42));
        assert_eq!(cache.hits, 1);
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_tracked_cell() {
        let mut cell: TrackedCell<i32> = TrackedCell::new();
        assert!(!cell.is_initialized());
        cell.set(99);
        assert!(cell.is_initialized());
        cell.get();
        assert_eq!(cell.force_count(), 1);
        cell.set(0);
        assert_eq!(cell.get(), Some(&99));
    }
}
