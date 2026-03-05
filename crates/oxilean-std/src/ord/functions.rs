//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{OrdResult, Permutation, SortedMap, SortedSet};

/// Build Ord type class in the environment.
pub fn build_ord_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let ordering_ty = type1.clone();
    env.add(Declaration::Axiom {
        name: Name::str("Ordering"),
        univ_params: vec![],
        ty: ordering_ty,
    })
    .map_err(|e| e.to_string())?;
    for variant in &["Ordering.lt", "Ordering.eq", "Ordering.gt"] {
        env.add(Declaration::Axiom {
            name: Name::str(*variant),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Ordering"), vec![]),
        })
        .map_err(|e| e.to_string())?;
    }
    let ord_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(type2.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ord"),
        univ_params: vec![],
        ty: ord_ty,
    })
    .map_err(|e| e.to_string())?;
    let compare_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Ord"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::Const(Name::str("Ordering"), vec![])),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ord.compare"),
        univ_params: vec![],
        ty: compare_ty,
    })
    .map_err(|e| e.to_string())?;
    add_ordering_predicate(env, "Ordering.isLT")?;
    add_ordering_predicate(env, "Ordering.isEQ")?;
    add_ordering_predicate(env, "Ordering.isGT")?;
    add_ordering_predicate(env, "Ordering.isLE")?;
    add_ordering_predicate(env, "Ordering.isGE")?;
    let swap_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("o"),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ordering.swap"),
        univ_params: vec![],
        ty: swap_ty,
    })
    .map_err(|e| e.to_string())?;
    let then_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("o1"),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("o2"),
            Box::new(Expr::Const(Name::str("Ordering"), vec![])),
            Box::new(Expr::Const(Name::str("Ordering"), vec![])),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ordering.then"),
        univ_params: vec![],
        ty: then_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Helper: add a predicate `name : Ordering → Bool` to the environment.
pub fn add_ordering_predicate(env: &mut Environment, name: &str) -> Result<(), String> {
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("o"),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
        Box::new(Expr::Const(Name::str("Bool"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Compare two values that implement `Ord` and return an `OrdResult`.
pub fn compare<T: Ord>(a: &T, b: &T) -> OrdResult {
    OrdResult::from_std(a.cmp(b))
}
/// Compare by a key function.
pub fn compare_by_key<T, K: Ord, F: Fn(&T) -> K>(a: &T, b: &T, key: F) -> OrdResult {
    OrdResult::from_std(key(a).cmp(&key(b)))
}
/// Lexicographic comparison of two slices.
pub fn compare_slices<T: Ord>(a: &[T], b: &[T]) -> OrdResult {
    OrdResult::from_std(a.cmp(b))
}
/// Return the minimum of two values.
pub fn ord_min<T: Ord>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}
/// Return the maximum of two values.
pub fn ord_max<T: Ord>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}
/// Clamp a value within `[lo, hi]`.
pub fn ord_clamp<T: Ord>(val: T, lo: T, hi: T) -> T {
    if val < lo {
        lo
    } else if val > hi {
        hi
    } else {
        val
    }
}
/// Stable sort a `Vec` using a comparison closure returning `OrdResult`.
pub fn sort_by<T, F>(v: &mut [T], mut cmp: F)
where
    F: FnMut(&T, &T) -> OrdResult,
{
    v.sort_by(|a, b| cmp(a, b).to_std());
}
/// Return `true` if a slice is sorted in non-decreasing order.
pub fn is_sorted<T: Ord>(s: &[T]) -> bool {
    s.windows(2).all(|w| w[0] <= w[1])
}
/// Return `true` if a slice is sorted in non-increasing order.
pub fn is_sorted_desc<T: Ord>(s: &[T]) -> bool {
    s.windows(2).all(|w| w[0] >= w[1])
}
/// Binary search returning an `OrdResult`-based position description.
///
/// Returns `Ok(index)` if found, `Err(index)` for the insertion point.
pub fn ord_binary_search<T: Ord>(s: &[T], target: &T) -> Result<usize, usize> {
    s.binary_search(target)
}
/// Chain multiple comparisons together with `then`.
///
/// Evaluates comparisons left-to-right, stopping as soon as one is not `Equal`.
pub fn compare_chain(comparisons: &[OrdResult]) -> OrdResult {
    comparisons
        .iter()
        .copied()
        .fold(OrdResult::Equal, OrdResult::then)
}
/// Reverse a comparison function (swap its output).
pub fn reverse_cmp<T, F>(a: &T, b: &T, cmp: F) -> OrdResult
where
    F: Fn(&T, &T) -> OrdResult,
{
    cmp(a, b).swap()
}
/// `true` if `a < b`.
pub fn lt<T: Ord>(a: &T, b: &T) -> bool {
    a < b
}
/// `true` if `a <= b`.
pub fn le<T: Ord>(a: &T, b: &T) -> bool {
    a <= b
}
/// `true` if `a > b`.
pub fn gt<T: Ord>(a: &T, b: &T) -> bool {
    a > b
}
/// `true` if `a >= b`.
pub fn ge<T: Ord>(a: &T, b: &T) -> bool {
    a >= b
}
/// `true` if `a == b`.
pub fn eq<T: Ord>(a: &T, b: &T) -> bool {
    a == b
}
/// `true` if `a != b`.
pub fn ne<T: Ord>(a: &T, b: &T) -> bool {
    a != b
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_ord_env() {
        let mut env = Environment::new();
        assert!(build_ord_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Ordering")).is_some());
        assert!(env.get(&Name::str("Ord")).is_some());
        assert!(env.get(&Name::str("Ord.compare")).is_some());
    }
    #[test]
    fn test_ord_result_swap() {
        assert_eq!(OrdResult::Less.swap(), OrdResult::Greater);
        assert_eq!(OrdResult::Greater.swap(), OrdResult::Less);
        assert_eq!(OrdResult::Equal.swap(), OrdResult::Equal);
    }
    #[test]
    fn test_ord_result_then() {
        assert_eq!(OrdResult::Equal.then(OrdResult::Less), OrdResult::Less);
        assert_eq!(OrdResult::Less.then(OrdResult::Greater), OrdResult::Less);
    }
    #[test]
    fn test_ord_result_predicates() {
        let lt = OrdResult::Less;
        let eq = OrdResult::Equal;
        let gt = OrdResult::Greater;
        assert!(lt.is_lt() && lt.is_le() && !lt.is_eq() && !lt.is_gt() && !lt.is_ge());
        assert!(eq.is_eq() && eq.is_le() && eq.is_ge() && !eq.is_lt() && !eq.is_gt());
        assert!(gt.is_gt() && gt.is_ge() && !gt.is_lt() && !gt.is_eq() && !gt.is_le());
    }
    #[test]
    fn test_compare() {
        assert_eq!(compare(&1, &2), OrdResult::Less);
        assert_eq!(compare(&2, &2), OrdResult::Equal);
        assert_eq!(compare(&3, &2), OrdResult::Greater);
    }
    #[test]
    fn test_compare_chain() {
        let chain = compare_chain(&[OrdResult::Equal, OrdResult::Equal, OrdResult::Less]);
        assert_eq!(chain, OrdResult::Less);
        let all_eq = compare_chain(&[OrdResult::Equal, OrdResult::Equal]);
        assert_eq!(all_eq, OrdResult::Equal);
    }
    #[test]
    fn test_sort_by() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
        sort_by(&mut v, |a, b| compare(a, b));
        assert!(is_sorted(&v));
    }
    #[test]
    fn test_is_sorted() {
        assert!(is_sorted(&[1, 2, 3, 4]));
        assert!(!is_sorted(&[1, 3, 2]));
        assert!(is_sorted_desc(&[4, 3, 2, 1]));
        assert!(!is_sorted_desc(&[1, 2, 3]));
    }
    #[test]
    fn test_ord_min_max_clamp() {
        assert_eq!(ord_min(3, 5), 3);
        assert_eq!(ord_max(3, 5), 5);
        assert_eq!(ord_clamp(10, 0, 5), 5);
        assert_eq!(ord_clamp(-1, 0, 5), 0);
        assert_eq!(ord_clamp(3, 0, 5), 3);
    }
    #[test]
    fn test_compare_by_key() {
        let a = ("b", 1);
        let b = ("a", 2);
        let res = compare_by_key(&a, &b, |x| x.0);
        assert_eq!(res, OrdResult::Greater);
    }
    #[test]
    fn test_compare_slices() {
        let a = &[1, 2, 3][..];
        let b = &[1, 2, 4][..];
        assert_eq!(compare_slices(a, b), OrdResult::Less);
    }
    #[test]
    fn test_signum() {
        assert_eq!(OrdResult::Less.to_signum(), -1);
        assert_eq!(OrdResult::Equal.to_signum(), 0);
        assert_eq!(OrdResult::Greater.to_signum(), 1);
    }
    #[test]
    fn test_display() {
        assert_eq!(OrdResult::Less.to_string(), "lt");
        assert_eq!(OrdResult::Equal.to_string(), "eq");
        assert_eq!(OrdResult::Greater.to_string(), "gt");
    }
    #[test]
    fn test_named_predicates() {
        assert!(lt(&1, &2));
        assert!(le(&2, &2));
        assert!(gt(&3, &2));
        assert!(ge(&2, &2));
        assert!(eq(&5, &5));
        assert!(ne(&5, &6));
    }
    #[test]
    fn test_reverse_cmp() {
        let res = reverse_cmp(&1i32, &2i32, |a, b| compare(a, b));
        assert_eq!(res, OrdResult::Greater);
    }
    #[test]
    fn test_binary_search() {
        let v = vec![1, 3, 5, 7, 9];
        assert!(ord_binary_search(&v, &5).is_ok());
        assert!(ord_binary_search(&v, &4).is_err());
    }
    #[test]
    fn test_to_from_std() {
        let o = OrdResult::Less;
        assert_eq!(OrdResult::from_std(o.to_std()), o);
    }
}
/// Compare by multiple keys in priority order.
///
/// Takes a list of `(a_key, b_key)` pairs and returns the first non-equal
/// comparison, or `Equal` if all keys are equal.
pub fn multi_key_compare<K: Ord>(key_pairs: &[(K, K)]) -> OrdResult {
    for (a, b) in key_pairs {
        let r = compare(a, b);
        if r != OrdResult::Equal {
            return r;
        }
    }
    OrdResult::Equal
}
/// Return the median element of a slice (or `None` if empty).
///
/// Uses the "lower median" for even-length slices.
pub fn median<T: Ord + Clone>(v: &[T]) -> Option<T> {
    if v.is_empty() {
        return None;
    }
    let mut sorted = v.to_vec();
    sorted.sort();
    Some(sorted[(sorted.len() - 1) / 2].clone())
}
/// Return `true` if `v` is a non-strict subset of `u` (all elements of `v` are in `u`).
pub fn is_subset<T: Ord>(v: &[T], u: &[T]) -> bool {
    v.iter().all(|item| u.binary_search(item).is_ok())
}
#[cfg(test)]
mod ord_extra_tests {
    use super::*;
    #[test]
    fn test_sorted_map_insert_get() {
        let mut m: SortedMap<u32, &str> = SortedMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&1), Some(&"one"));
        assert_eq!(m.get(&3), Some(&"three"));
        assert!(m.get(&5).is_none());
    }
    #[test]
    fn test_sorted_map_keys_ordered() {
        let mut m: SortedMap<u32, u32> = SortedMap::new();
        m.insert(5, 50);
        m.insert(1, 10);
        m.insert(3, 30);
        let keys: Vec<_> = m.keys().copied().collect();
        assert_eq!(keys, vec![1, 3, 5]);
    }
    #[test]
    fn test_sorted_map_remove() {
        let mut m: SortedMap<u32, u32> = SortedMap::new();
        m.insert(1, 100);
        assert_eq!(m.remove(&1), Some(100));
        assert!(m.get(&1).is_none());
    }
    #[test]
    fn test_sorted_set_insert_contains() {
        let mut s: SortedSet<u32> = SortedSet::new();
        s.insert(5);
        s.insert(3);
        s.insert(7);
        assert!(s.contains(&3));
        assert!(s.contains(&5));
        assert!(!s.contains(&4));
    }
    #[test]
    fn test_sorted_set_union_intersection() {
        let mut a: SortedSet<u32> = SortedSet::new();
        a.insert(1);
        a.insert(2);
        a.insert(3);
        let mut b: SortedSet<u32> = SortedSet::new();
        b.insert(2);
        b.insert(3);
        b.insert(4);
        let u = a.union(&b);
        assert_eq!(u.len(), 4);
        let i = a.intersection(&b);
        assert_eq!(i.len(), 2);
        assert!(i.contains(&2));
        assert!(i.contains(&3));
    }
    #[test]
    fn test_sorted_set_difference() {
        let mut a: SortedSet<u32> = SortedSet::new();
        a.insert(1);
        a.insert(2);
        a.insert(3);
        let mut b: SortedSet<u32> = SortedSet::new();
        b.insert(2);
        let diff = a.difference(&b);
        assert_eq!(diff.len(), 2);
        assert!(diff.contains(&1));
        assert!(diff.contains(&3));
    }
    #[test]
    fn test_multi_key_compare() {
        let result = multi_key_compare(&[(1u32, 1u32), (2u32, 3u32)]);
        assert_eq!(result, OrdResult::Less);
        let all_eq = multi_key_compare(&[(1u32, 1u32), (2u32, 2u32)]);
        assert_eq!(all_eq, OrdResult::Equal);
    }
    #[test]
    fn test_median() {
        assert_eq!(median(&[3u32, 1, 4, 1, 5]), Some(3));
        assert_eq!(median::<u32>(&[]), None);
        assert_eq!(median(&[2u32, 4]), Some(2));
    }
    #[test]
    fn test_is_subset() {
        let u = vec![1u32, 2, 3, 4, 5];
        let v = vec![2u32, 4];
        assert!(is_subset(&v, &u));
        let w = vec![2u32, 6];
        assert!(!is_subset(&w, &u));
    }
}
/// Compare two `Option<T>` values: `None < Some(x)` for all `x`.
#[allow(dead_code)]
pub fn compare_option<T: Ord>(a: &Option<T>, b: &Option<T>) -> OrdResult {
    match (a, b) {
        (None, None) => OrdResult::Equal,
        (None, Some(_)) => OrdResult::Less,
        (Some(_), None) => OrdResult::Greater,
        (Some(x), Some(y)) => compare(x, y),
    }
}
/// Compare two `bool` values (`false < true`).
#[allow(dead_code)]
pub fn compare_bool(a: bool, b: bool) -> OrdResult {
    OrdResult::from_std(a.cmp(&b))
}
#[cfg(test)]
mod ord_final_tests {
    use super::*;
    #[test]
    fn test_compare_option() {
        assert_eq!(compare_option::<u32>(&None, &None), OrdResult::Equal);
        assert_eq!(compare_option::<u32>(&None, &Some(1)), OrdResult::Less);
        assert_eq!(compare_option::<u32>(&Some(1), &None), OrdResult::Greater);
    }
    #[test]
    fn test_compare_bool() {
        assert_eq!(compare_bool(false, true), OrdResult::Less);
        assert_eq!(compare_bool(true, true), OrdResult::Equal);
    }
}
/// Simple topological sort for dependency graphs.
///
/// Nodes are identified by `usize` indices. Returns a sorted list of node
/// indices, or an error if the graph has a cycle.
pub fn topological_sort(n: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, String> {
    let mut in_degree = vec![0usize; n];
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(from, to) in edges {
        adj[from].push(to);
        in_degree[to] += 1;
    }
    let mut queue: std::collections::VecDeque<usize> =
        (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut result = Vec::new();
    while let Some(node) = queue.pop_front() {
        result.push(node);
        for &next in &adj[node] {
            in_degree[next] -= 1;
            if in_degree[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    if result.len() == n {
        Ok(result)
    } else {
        Err("cycle detected in dependency graph".to_string())
    }
}
/// Assign dense ranks to a slice of comparable values.
///
/// Returns a `Vec<usize>` where the `i`-th entry is the rank of `v[i]`
/// (0 = smallest). Equal values receive the same rank.
pub fn dense_rank<T: Ord>(v: &[T]) -> Vec<usize> {
    let mut indexed: Vec<(usize, &T)> = v.iter().enumerate().collect();
    indexed.sort_by_key(|(_, val)| *val);
    let mut rank = vec![0usize; v.len()];
    let mut current_rank = 0usize;
    for i in 0..indexed.len() {
        if i > 0 && indexed[i].1 != indexed[i - 1].1 {
            current_rank += 1;
        }
        rank[indexed[i].0] = current_rank;
    }
    rank
}
/// Assign competition ranks (1224 ranking: equal items get same rank,
/// next rank skips).
pub fn competition_rank<T: Ord>(v: &[T]) -> Vec<usize> {
    let n = v.len();
    if n == 0 {
        return Vec::new();
    }
    let mut ranks = vec![1usize; n];
    for i in 0..n {
        for j in 0..n {
            if i != j && v[j] < v[i] {
                ranks[i] += 1;
            }
        }
    }
    ranks
}
/// Extension trait for ordered types providing utility methods.
pub trait OrdExt: Ord + Sized {
    /// Return the value clamped to `[lo, hi]`.
    fn clamped(self, lo: Self, hi: Self) -> Self {
        ord_clamp(self, lo, hi)
    }
    /// Return the `OrdResult` of comparing `self` with `other`.
    fn ord_cmp(&self, other: &Self) -> OrdResult {
        compare(self, other)
    }
    /// Return `true` if `self` is strictly between `lo` and `hi`.
    fn strictly_between(&self, lo: &Self, hi: &Self) -> bool {
        self > lo && self < hi
    }
    /// Return `true` if `self` is in the closed interval `[lo, hi]`.
    fn in_range(&self, lo: &Self, hi: &Self) -> bool {
        self >= lo && self <= hi
    }
}
impl<T: Ord> OrdExt for T {}
/// Build `Ord.min : {α : Type} → [Ord α] → α → α → α`.
pub fn build_ord_min(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Ord"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::BVar(3)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ord.min"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build `Ord.max : {α : Type} → [Ord α] → α → α → α`.
pub fn build_ord_max(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Ord"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("b"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::BVar(3)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ord.max"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
#[cfg(test)]
mod ord_advanced_tests {
    use super::*;
    #[test]
    fn test_topological_sort_dag() {
        let result = topological_sort(3, &[(0, 1), (1, 2)]).expect("operation should succeed");
        assert!(result.iter().position(|&x| x == 0) < result.iter().position(|&x| x == 1));
        assert!(result.iter().position(|&x| x == 1) < result.iter().position(|&x| x == 2));
    }
    #[test]
    fn test_topological_sort_cycle() {
        assert!(topological_sort(2, &[(0, 1), (1, 0)]).is_err());
    }
    #[test]
    fn test_topological_sort_empty() {
        let result = topological_sort(0, &[]).expect("operation should succeed");
        assert!(result.is_empty());
    }
    #[test]
    fn test_dense_rank_basic() {
        let v = vec![3u32, 1, 4, 1, 5, 9, 2, 6];
        let ranks = dense_rank(&v);
        assert_eq!(ranks[1], 0);
        assert_eq!(ranks[3], 0);
    }
    #[test]
    fn test_dense_rank_all_equal() {
        let v = vec![5u32, 5, 5];
        let ranks = dense_rank(&v);
        assert!(ranks.iter().all(|&r| r == 0));
    }
    #[test]
    fn test_competition_rank() {
        let v = vec![1u32, 2, 2, 3];
        let ranks = competition_rank(&v);
        assert_eq!(ranks[0], 1);
        assert_eq!(ranks[1], 2);
        assert_eq!(ranks[2], 2);
        assert_eq!(ranks[3], 4);
    }
    #[test]
    fn test_ord_ext_clamped() {
        assert_eq!(5u32.clamped(1, 10), 5);
        assert_eq!(0u32.clamped(1, 10), 1);
        assert_eq!(15u32.clamped(1, 10), 10);
    }
    #[test]
    fn test_ord_ext_strictly_between() {
        assert!(5u32.strictly_between(&1, &10));
        assert!(!1u32.strictly_between(&1, &10));
        assert!(!10u32.strictly_between(&1, &10));
    }
    #[test]
    fn test_ord_ext_in_range() {
        assert!(5u32.in_range(&1, &10));
        assert!(1u32.in_range(&1, &10));
        assert!(10u32.in_range(&1, &10));
        assert!(!0u32.in_range(&1, &10));
    }
    #[test]
    fn test_permutation_identity() {
        let p = Permutation::identity(3);
        assert!(p.is_identity());
    }
    #[test]
    fn test_permutation_from_sort_order() {
        let v = vec![3u32, 1, 2];
        let p = Permutation::from_sort_order(&v);
        let sorted = p.apply(&v);
        assert_eq!(sorted, vec![1, 2, 3]);
    }
    #[test]
    fn test_permutation_inverse() {
        let v = vec![3u32, 1, 2];
        let p = Permutation::from_sort_order(&v);
        let inv = p.inverse();
        let composed = p.compose(&inv);
        assert!(composed.is_identity());
    }
    #[test]
    fn test_permutation_compose() {
        let p = Permutation {
            perm: vec![1, 0, 2],
        };
        let q = Permutation {
            perm: vec![0, 1, 2],
        };
        let pq = p.compose(&q);
        assert_eq!(pq.perm, vec![1, 0, 2]);
    }
    #[test]
    fn test_build_ord_min_max() {
        let mut env = Environment::new();
        build_ord_env(&mut env).expect("build_ord_env should succeed");
        assert!(build_ord_min(&mut env).is_ok());
        assert!(build_ord_max(&mut env).is_ok());
    }
    #[test]
    fn test_sorted_map_overwrite() {
        let mut m: SortedMap<u32, u32> = SortedMap::new();
        m.insert(1, 100);
        m.insert(1, 200);
        assert_eq!(m.get(&1), Some(&200));
        assert_eq!(m.len(), 1);
    }
    #[test]
    fn test_sorted_set_remove() {
        let mut s: SortedSet<u32> = SortedSet::new();
        s.insert(3);
        s.insert(5);
        assert!(s.remove(&3));
        assert!(!s.contains(&3));
        assert!(!s.remove(&3));
    }
}
pub fn ord_e_type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn ord_e_type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn ord_e_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn ord_e_arrow(dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(dom),
        Box::new(cod),
    )
}
pub fn ord_e_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn ord_e_ipi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn ord_e_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn ord_e_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    ord_e_app(ord_e_app(f, a), b)
}
pub fn ord_e_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    ord_e_app(ord_e_app2(f, a, b), c)
}
pub fn ord_e_nat() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
pub fn ord_e_bool() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
pub fn ord_e_prop_app2(name: &str, a: Expr, b: Expr) -> Expr {
    ord_e_app2(Expr::Const(Name::str(name), vec![]), a, b)
}
pub fn ord_e_eq(ty: Expr, a: Expr, b: Expr) -> Expr {
    ord_e_app3(Expr::Const(Name::str("Eq"), vec![]), ty, a, b)
}
pub fn ord_e_and(a: Expr, b: Expr) -> Expr {
    ord_e_prop_app2("And", a, b)
}
pub fn ord_e_iff(a: Expr, b: Expr) -> Expr {
    ord_e_prop_app2("Iff", a, b)
}
pub fn ord_e_ordering() -> Expr {
    Expr::Const(Name::str("Ordering"), vec![])
}
pub fn ord_e_ord_inst(alpha: Expr) -> Expr {
    ord_e_app(Expr::Const(Name::str("Ord"), vec![]), alpha)
}
pub fn ord_e_add_axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// `OrdCat : Type 2` — the category of preordered sets.
pub fn axiom_ord_cat_ty() -> Expr {
    ord_e_type2()
}
/// `OrdCat.obj : OrdCat → Type` — extract the underlying type.
pub fn axiom_ord_cat_obj_ty() -> Expr {
    ord_e_arrow(Expr::Const(Name::str("OrdCat"), vec![]), ord_e_type1())
}
/// `MonotoneMap : {α β : Type} → [Ord α] → [Ord β] → Type` — monotone maps.
pub fn axiom_monotone_map_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        ord_e_ipi(
            "β",
            ord_e_type1(),
            Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("_"),
                Box::new(ord_e_ord_inst(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("_"),
                    Box::new(ord_e_ord_inst(Expr::BVar(1))),
                    Box::new(ord_e_type1()),
                )),
            ),
        ),
    )
}
/// `MonotoneMap.mk : (f : α → β) → (∀ a b, a ≤ b → f a ≤ f b) → MonotoneMap`
pub fn axiom_monotone_map_mk_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        ord_e_ipi(
            "β",
            ord_e_type1(),
            ord_e_pi(
                "f",
                ord_e_arrow(Expr::BVar(1), Expr::BVar(0)),
                ord_e_pi(
                    "mono",
                    Expr::Const(Name::str("MonotoneMap.mono_proof_ty"), vec![]),
                    Expr::Const(Name::str("MonotoneMap"), vec![]),
                ),
            ),
        ),
    )
}
/// `MonotoneMapsFormCat : Prop` — order-preserving maps form a category.
pub fn axiom_monotone_maps_form_cat_ty() -> Expr {
    ord_e_prop()
}
/// `GaloisConnection : {α β : Type} → [Ord α] → [Ord β] → (α → β) → (β → α) → Prop`
pub fn axiom_galois_connection_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        ord_e_ipi(
            "β",
            ord_e_type1(),
            Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("_"),
                Box::new(ord_e_ord_inst(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("_"),
                    Box::new(ord_e_ord_inst(Expr::BVar(1))),
                    Box::new(ord_e_pi(
                        "l",
                        ord_e_arrow(Expr::BVar(3), Expr::BVar(2)),
                        ord_e_pi("r", ord_e_arrow(Expr::BVar(3), Expr::BVar(4)), ord_e_prop()),
                    )),
                )),
            ),
        ),
    )
}
/// `GaloisConnection.adjoint_iff : Prop` — adjoint functors as Galois connections.
pub fn axiom_galois_adjoint_iff_ty() -> Expr {
    ord_e_prop()
}
/// `OrdEnrichedCat : Type 2` — Ord-enriched categories.
pub fn axiom_ord_enriched_cat_ty() -> Expr {
    ord_e_type2()
}
/// `FixpointExists : {α : Type} → [Ord α] → (α → α) → Prop` — Knaster-Tarski fixpoint.
pub fn axiom_fixpoint_exists_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "f",
                ord_e_arrow(Expr::BVar(1), Expr::BVar(1)),
                ord_e_prop(),
            )),
        ),
    )
}
/// `KnasterTarski : {α : Type} → [CompleteLattice α] → (α → α) → α` — least fixpoint.
pub fn axiom_knaster_tarski_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_app(
                Expr::Const(Name::str("CompleteLattice"), vec![]),
                Expr::BVar(0),
            )),
            Box::new(ord_e_pi(
                "f",
                ord_e_arrow(Expr::BVar(1), Expr::BVar(1)),
                Expr::BVar(2),
            )),
        ),
    )
}
/// `KnasterTarski.least_fixpoint : Prop` — KT produces the least fixpoint.
pub fn axiom_knaster_tarski_least_ty() -> Expr {
    ord_e_prop()
}
/// `ScottContinuous : {α β : Type} → [Ord α] → [Ord β] → (α → β) → Prop`.
pub fn axiom_scott_continuous_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        ord_e_ipi(
            "β",
            ord_e_type1(),
            Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("_"),
                Box::new(ord_e_ord_inst(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("_"),
                    Box::new(ord_e_ord_inst(Expr::BVar(1))),
                    Box::new(ord_e_pi(
                        "f",
                        ord_e_arrow(Expr::BVar(3), Expr::BVar(2)),
                        ord_e_prop(),
                    )),
                )),
            ),
        ),
    )
}
/// `DCPO : Type 2` — directed-complete partial orders.
pub fn axiom_dcpo_ty() -> Expr {
    ord_e_type2()
}
/// `DCPO.lfp : {D : DCPO} → (D.carrier → D.carrier) → D.carrier` — least fixpoint in DCPO.
pub fn axiom_dcpo_lfp_ty() -> Expr {
    ord_e_pi(
        "D",
        Expr::Const(Name::str("DCPO"), vec![]),
        ord_e_pi(
            "f",
            ord_e_arrow(
                ord_e_app(
                    Expr::Const(Name::str("DCPO.carrier"), vec![]),
                    Expr::BVar(0),
                ),
                ord_e_app(
                    Expr::Const(Name::str("DCPO.carrier"), vec![]),
                    Expr::BVar(0),
                ),
            ),
            ord_e_app(
                Expr::Const(Name::str("DCPO.carrier"), vec![]),
                Expr::BVar(1),
            ),
        ),
    )
}
/// `OmegaCPO : Type 2` — omega-complete partial orders.
pub fn axiom_omega_cpo_ty() -> Expr {
    ord_e_type2()
}
/// `OmegaCPO.chain_limit : {D : OmegaCPO} → (Nat → D.carrier) → D.carrier`.
pub fn axiom_omega_cpo_chain_limit_ty() -> Expr {
    ord_e_pi(
        "D",
        Expr::Const(Name::str("OmegaCPO"), vec![]),
        ord_e_pi(
            "chain",
            ord_e_arrow(
                ord_e_nat(),
                ord_e_app(
                    Expr::Const(Name::str("OmegaCPO.carrier"), vec![]),
                    Expr::BVar(0),
                ),
            ),
            ord_e_app(
                Expr::Const(Name::str("OmegaCPO.carrier"), vec![]),
                Expr::BVar(1),
            ),
        ),
    )
}
/// `LazyEvalOrd : {α : Type} → [Ord α] → Thunk α → Thunk α → Ordering` — lazy comparison.
pub fn axiom_lazy_eval_ord_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "a",
                ord_e_app(Expr::Const(Name::str("Thunk"), vec![]), Expr::BVar(1)),
                ord_e_pi(
                    "b",
                    ord_e_app(Expr::Const(Name::str("Thunk"), vec![]), Expr::BVar(2)),
                    ord_e_ordering(),
                ),
            )),
        ),
    )
}
/// `SortingNetworkCorrect : (n : Nat) → (net : SortingNetwork n) → Prop`.
pub fn axiom_sorting_network_correct_ty() -> Expr {
    ord_e_pi(
        "n",
        ord_e_nat(),
        ord_e_pi(
            "net",
            ord_e_app(
                Expr::Const(Name::str("SortingNetwork"), vec![]),
                Expr::BVar(0),
            ),
            ord_e_prop(),
        ),
    )
}
/// `ComparisonSortLowerBound : Prop` — Omega(n log n) lower bound for comparison sorts.
pub fn axiom_comparison_sort_lower_bound_ty() -> Expr {
    ord_e_prop()
}
/// `BTreeInvariant : {α : Type} → [Ord α] → (t : BTree α) → Prop`.
pub fn axiom_btree_invariant_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "t",
                ord_e_app(Expr::Const(Name::str("BTree"), vec![]), Expr::BVar(1)),
                ord_e_prop(),
            )),
        ),
    )
}
/// `RedBlackBalance : {α : Type} → [Ord α] → (t : RBTree α) → Prop`.
pub fn axiom_red_black_balance_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "t",
                ord_e_app(Expr::Const(Name::str("RBTree"), vec![]), Expr::BVar(1)),
                ord_e_prop(),
            )),
        ),
    )
}
/// `WellFounded.lt : {α : Type} → [Ord α] → WellFounded (· < ·)` — well-foundedness.
pub fn axiom_well_founded_lt_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_app(
                Expr::Const(Name::str("WellFounded"), vec![]),
                Expr::Const(Name::str("LT.lt"), vec![]),
            )),
        ),
    )
}
/// `Antisymm : {α : Type} → [Ord α] → ∀ a b, a ≤ b → b ≤ a → a = b`.
pub fn axiom_antisymm_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "a",
                Expr::BVar(1),
                ord_e_pi(
                    "b",
                    Expr::BVar(2),
                    ord_e_pi(
                        "h1",
                        ord_e_prop_app2("LE.le", Expr::BVar(1), Expr::BVar(0)),
                        ord_e_pi(
                            "h2",
                            ord_e_prop_app2("LE.le", Expr::BVar(1), Expr::BVar(2)),
                            ord_e_eq(Expr::BVar(4), Expr::BVar(3), Expr::BVar(2)),
                        ),
                    ),
                ),
            )),
        ),
    )
}
/// `Transitivity : {α : Type} → [Ord α] → ∀ a b c, a ≤ b → b ≤ c → a ≤ c`.
pub fn axiom_transitivity_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "a",
                Expr::BVar(1),
                ord_e_pi(
                    "b",
                    Expr::BVar(2),
                    ord_e_pi(
                        "c",
                        Expr::BVar(3),
                        ord_e_pi(
                            "hab",
                            ord_e_prop_app2("LE.le", Expr::BVar(2), Expr::BVar(1)),
                            ord_e_pi(
                                "hbc",
                                ord_e_prop_app2("LE.le", Expr::BVar(2), Expr::BVar(1)),
                                ord_e_prop_app2("LE.le", Expr::BVar(4), Expr::BVar(2)),
                            ),
                        ),
                    ),
                ),
            )),
        ),
    )
}
/// `Totality : {α : Type} → [Ord α] → ∀ a b, a ≤ b ∨ b ≤ a`.
pub fn axiom_totality_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "a",
                Expr::BVar(1),
                ord_e_pi(
                    "b",
                    Expr::BVar(2),
                    ord_e_app2(
                        Expr::Const(Name::str("Or"), vec![]),
                        ord_e_prop_app2("LE.le", Expr::BVar(1), Expr::BVar(0)),
                        ord_e_prop_app2("LE.le", Expr::BVar(0), Expr::BVar(1)),
                    ),
                ),
            )),
        ),
    )
}
/// `Reflexivity : {α : Type} → [Ord α] → ∀ a, a ≤ a`.
pub fn axiom_reflexivity_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "a",
                Expr::BVar(1),
                ord_e_prop_app2("LE.le", Expr::BVar(0), Expr::BVar(0)),
            )),
        ),
    )
}
/// `Monotone : {α β : Type} → [Ord α] → [Ord β] → (α → β) → Prop`.
pub fn axiom_monotone_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        ord_e_ipi(
            "β",
            ord_e_type1(),
            Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("_"),
                Box::new(ord_e_ord_inst(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("_"),
                    Box::new(ord_e_ord_inst(Expr::BVar(1))),
                    Box::new(ord_e_pi(
                        "f",
                        ord_e_arrow(Expr::BVar(3), Expr::BVar(2)),
                        ord_e_prop(),
                    )),
                )),
            ),
        ),
    )
}
/// `StrictMono : {α β : Type} → [Ord α] → [Ord β] → (α → β) → Prop`.
pub fn axiom_strict_mono_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        ord_e_ipi(
            "β",
            ord_e_type1(),
            Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("_"),
                Box::new(ord_e_ord_inst(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("_"),
                    Box::new(ord_e_ord_inst(Expr::BVar(1))),
                    Box::new(ord_e_pi(
                        "f",
                        ord_e_arrow(Expr::BVar(3), Expr::BVar(2)),
                        ord_e_prop(),
                    )),
                )),
            ),
        ),
    )
}
/// `CompleteLattice : Type → Type 1`.
pub fn axiom_complete_lattice_ty() -> Expr {
    ord_e_arrow(ord_e_type1(), ord_e_type2())
}
/// `CompleteLattice.sSup : {α : Type} → [CompleteLattice α] → Set α → α`.
pub fn axiom_complete_lattice_ssup_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_app(
                Expr::Const(Name::str("CompleteLattice"), vec![]),
                Expr::BVar(0),
            )),
            Box::new(ord_e_pi(
                "S",
                ord_e_app(Expr::Const(Name::str("Set"), vec![]), Expr::BVar(1)),
                Expr::BVar(2),
            )),
        ),
    )
}
/// `CompleteLattice.sInf : {α : Type} → [CompleteLattice α] → Set α → α`.
pub fn axiom_complete_lattice_sinf_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_app(
                Expr::Const(Name::str("CompleteLattice"), vec![]),
                Expr::BVar(0),
            )),
            Box::new(ord_e_pi(
                "S",
                ord_e_app(Expr::Const(Name::str("Set"), vec![]), Expr::BVar(1)),
                Expr::BVar(2),
            )),
        ),
    )
}
/// `UpperBound : {α : Type} → [Ord α] → Set α → α → Prop`.
pub fn axiom_upper_bound_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "S",
                ord_e_app(Expr::Const(Name::str("Set"), vec![]), Expr::BVar(1)),
                ord_e_pi("x", Expr::BVar(2), ord_e_prop()),
            )),
        ),
    )
}
/// `LowerBound : {α : Type} → [Ord α] → Set α → α → Prop`.
pub fn axiom_lower_bound_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "S",
                ord_e_app(Expr::Const(Name::str("Set"), vec![]), Expr::BVar(1)),
                ord_e_pi("x", Expr::BVar(2), ord_e_prop()),
            )),
        ),
    )
}
/// `IsLUB : {α : Type} → [Ord α] → Set α → α → Prop` — least upper bound predicate.
pub fn axiom_is_lub_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "S",
                ord_e_app(Expr::Const(Name::str("Set"), vec![]), Expr::BVar(1)),
                ord_e_pi("x", Expr::BVar(2), ord_e_prop()),
            )),
        ),
    )
}
/// `IsGLB : {α : Type} → [Ord α] → Set α → α → Prop` — greatest lower bound predicate.
pub fn axiom_is_glb_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "S",
                ord_e_app(Expr::Const(Name::str("Set"), vec![]), Expr::BVar(1)),
                ord_e_pi("x", Expr::BVar(2), ord_e_prop()),
            )),
        ),
    )
}
/// `OrderIso : {α β : Type} → [Ord α] → [Ord β] → Type` — order isomorphisms.
pub fn axiom_order_iso_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        ord_e_ipi(
            "β",
            ord_e_type1(),
            Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("_"),
                Box::new(ord_e_ord_inst(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("_"),
                    Box::new(ord_e_ord_inst(Expr::BVar(1))),
                    Box::new(ord_e_type1()),
                )),
            ),
        ),
    )
}
/// `OrderEmbedding : {α β : Type} → [Ord α] → [Ord β] → Type` — order embeddings.
pub fn axiom_order_embedding_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        ord_e_ipi(
            "β",
            ord_e_type1(),
            Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("_"),
                Box::new(ord_e_ord_inst(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("_"),
                    Box::new(ord_e_ord_inst(Expr::BVar(1))),
                    Box::new(ord_e_type1()),
                )),
            ),
        ),
    )
}
/// `Antichain : {α : Type} → [Ord α] → Set α → Prop` — an antichain in a partial order.
pub fn axiom_antichain_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "S",
                ord_e_app(Expr::Const(Name::str("Set"), vec![]), Expr::BVar(1)),
                ord_e_prop(),
            )),
        ),
    )
}
/// `DilworthTheorem : Prop` — Dilworth's theorem relating chains and antichains.
pub fn axiom_dilworth_theorem_ty() -> Expr {
    ord_e_prop()
}
/// `MirskysTheorem : Prop` — Mirsky's theorem dual to Dilworth's.
pub fn axiom_mirskys_theorem_ty() -> Expr {
    ord_e_prop()
}
/// `OrdCompare.compare_eq_iff : {α : Type} → [Ord α] → ∀ a b, compare a b = Ordering.eq ↔ a = b`.
pub fn axiom_compare_eq_iff_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "a",
                Expr::BVar(1),
                ord_e_pi(
                    "b",
                    Expr::BVar(2),
                    ord_e_iff(
                        ord_e_eq(
                            ord_e_ordering(),
                            ord_e_app2(
                                Expr::Const(Name::str("Ord.compare"), vec![]),
                                Expr::BVar(1),
                                Expr::BVar(0),
                            ),
                            Expr::Const(Name::str("Ordering.eq"), vec![]),
                        ),
                        ord_e_eq(Expr::BVar(3), Expr::BVar(1), Expr::BVar(0)),
                    ),
                ),
            )),
        ),
    )
}
/// `OrdCompare.compare_swap : {α : Type} → [Ord α] → ∀ a b, compare a b = (compare b a).swap`.
pub fn axiom_compare_swap_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_pi(
                "a",
                Expr::BVar(1),
                ord_e_pi(
                    "b",
                    Expr::BVar(2),
                    ord_e_eq(
                        ord_e_ordering(),
                        ord_e_app2(
                            Expr::Const(Name::str("Ord.compare"), vec![]),
                            Expr::BVar(1),
                            Expr::BVar(0),
                        ),
                        ord_e_app(
                            Expr::Const(Name::str("Ordering.swap"), vec![]),
                            ord_e_app2(
                                Expr::Const(Name::str("Ord.compare"), vec![]),
                                Expr::BVar(0),
                                Expr::BVar(1),
                            ),
                        ),
                    ),
                ),
            )),
        ),
    )
}
/// `BoolOrd : Ord Bool` — the canonical ordering on `Bool` (false < true).
pub fn axiom_bool_ord_ty() -> Expr {
    ord_e_ord_inst(ord_e_bool())
}
/// `NatOrd : Ord Nat` — the canonical ordering on `Nat`.
pub fn axiom_nat_ord_ty() -> Expr {
    ord_e_ord_inst(ord_e_nat())
}
/// `ProdOrd : {α β : Type} → [Ord α] → [Ord β] → Ord (α × β)` — lexicographic product order.
pub fn axiom_prod_ord_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        ord_e_ipi(
            "β",
            ord_e_type1(),
            Expr::Pi(
                BinderInfo::InstImplicit,
                Name::str("_"),
                Box::new(ord_e_ord_inst(Expr::BVar(1))),
                Box::new(Expr::Pi(
                    BinderInfo::InstImplicit,
                    Name::str("_"),
                    Box::new(ord_e_ord_inst(Expr::BVar(1))),
                    Box::new(ord_e_ord_inst(ord_e_app2(
                        Expr::Const(Name::str("Prod"), vec![]),
                        Expr::BVar(3),
                        Expr::BVar(2),
                    ))),
                )),
            ),
        ),
    )
}
/// `ListOrd : {α : Type} → [Ord α] → Ord (List α)` — lexicographic list order.
pub fn axiom_list_ord_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_ord_inst(ord_e_app(
                Expr::Const(Name::str("List"), vec![]),
                Expr::BVar(1),
            ))),
        ),
    )
}
/// `OptionOrd : {α : Type} → [Ord α] → Ord (Option α)` — option order (None is least).
pub fn axiom_option_ord_ty() -> Expr {
    ord_e_ipi(
        "α",
        ord_e_type1(),
        Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(ord_e_ord_inst(Expr::BVar(0))),
            Box::new(ord_e_ord_inst(ord_e_app(
                Expr::Const(Name::str("Option"), vec![]),
                Expr::BVar(1),
            ))),
        ),
    )
}
/// `HeytingAlgebra : Type → Type 1` — Heyting algebras (intuitionistic lattices).
pub fn axiom_heyting_algebra_ty() -> Expr {
    ord_e_arrow(ord_e_type1(), ord_e_type2())
}
/// `BooleanAlgebra : Type → Type 1` — Boolean algebras.
pub fn axiom_boolean_algebra_ty() -> Expr {
    ord_e_arrow(ord_e_type1(), ord_e_type2())
}
/// Register all extended Ord axioms into the environment.
pub fn register_ord_extended(env: &mut Environment) -> Result<(), String> {
    ord_e_add_axiom(env, "OrdCat", axiom_ord_cat_ty())?;
    ord_e_add_axiom(env, "OrdCat.obj", axiom_ord_cat_obj_ty())?;
    ord_e_add_axiom(
        env,
        "MonotoneMapsFormCat",
        axiom_monotone_maps_form_cat_ty(),
    )?;
    ord_e_add_axiom(
        env,
        "GaloisConnection.adjoint_iff",
        axiom_galois_adjoint_iff_ty(),
    )?;
    ord_e_add_axiom(env, "OrdEnrichedCat", axiom_ord_enriched_cat_ty())?;
    ord_e_add_axiom(
        env,
        "KnasterTarski.least_fixpoint",
        axiom_knaster_tarski_least_ty(),
    )?;
    ord_e_add_axiom(env, "DCPO", axiom_dcpo_ty())?;
    ord_e_add_axiom(env, "OmegaCPO", axiom_omega_cpo_ty())?;
    ord_e_add_axiom(
        env,
        "ComparisonSortLowerBound",
        axiom_comparison_sort_lower_bound_ty(),
    )?;
    ord_e_add_axiom(env, "WellFounded.lt", axiom_well_founded_lt_ty())?;
    ord_e_add_axiom(env, "Antisymm", axiom_antisymm_ty())?;
    ord_e_add_axiom(env, "Transitivity", axiom_transitivity_ty())?;
    ord_e_add_axiom(env, "Totality", axiom_totality_ty())?;
    ord_e_add_axiom(env, "Reflexivity", axiom_reflexivity_ty())?;
    ord_e_add_axiom(env, "Monotone", axiom_monotone_ty())?;
    ord_e_add_axiom(env, "StrictMono", axiom_strict_mono_ty())?;
    ord_e_add_axiom(env, "CompleteLattice", axiom_complete_lattice_ty())?;
    ord_e_add_axiom(env, "UpperBound", axiom_upper_bound_ty())?;
    ord_e_add_axiom(env, "LowerBound", axiom_lower_bound_ty())?;
    ord_e_add_axiom(env, "IsLUB", axiom_is_lub_ty())?;
    ord_e_add_axiom(env, "IsGLB", axiom_is_glb_ty())?;
    ord_e_add_axiom(env, "OrderIso", axiom_order_iso_ty())?;
    ord_e_add_axiom(env, "OrderEmbedding", axiom_order_embedding_ty())?;
    ord_e_add_axiom(env, "Antichain", axiom_antichain_ty())?;
    ord_e_add_axiom(env, "DilworthTheorem", axiom_dilworth_theorem_ty())?;
    ord_e_add_axiom(env, "MirskysTheorem", axiom_mirskys_theorem_ty())?;
    ord_e_add_axiom(env, "OrdCompare.compare_eq_iff", axiom_compare_eq_iff_ty())?;
    ord_e_add_axiom(env, "OrdCompare.compare_swap", axiom_compare_swap_ty())?;
    ord_e_add_axiom(env, "BoolOrd", axiom_bool_ord_ty())?;
    ord_e_add_axiom(env, "NatOrd", axiom_nat_ord_ty())?;
    ord_e_add_axiom(env, "ProdOrd", axiom_prod_ord_ty())?;
    ord_e_add_axiom(env, "ListOrd", axiom_list_ord_ty())?;
    ord_e_add_axiom(env, "OptionOrd", axiom_option_ord_ty())?;
    ord_e_add_axiom(env, "HeytingAlgebra", axiom_heyting_algebra_ty())?;
    ord_e_add_axiom(env, "BooleanAlgebra", axiom_boolean_algebra_ty())?;
    Ok(())
}
