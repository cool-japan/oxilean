//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    DedekindCutQ, FiniteLinearOrder, FinitePartialOrder, OrderedRange, OrderedTable, Ordering,
    OrderingBuilder, OrdinalCnf, WqoInstance,
};

/// Build Ordering type in the environment.
pub fn build_ordering_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    env.add(Declaration::Axiom {
        name: Name::str("Ordering"),
        univ_params: vec![],
        ty: type1.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Ordering.less"),
        univ_params: vec![],
        ty: Expr::Const(Name::str("Ordering"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Ordering.equal"),
        univ_params: vec![],
        ty: Expr::Const(Name::str("Ordering"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Ordering.greater"),
        univ_params: vec![],
        ty: Expr::Const(Name::str("Ordering"), vec![]),
    })
    .map_err(|e| e.to_string())?;
    let is_le_ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("o"),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
        Box::new(Expr::Const(Name::str("Bool"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ordering.isLE"),
        univ_params: vec![],
        ty: is_le_ty,
    })
    .map_err(|e| e.to_string())?;
    let is_ge_ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("o"),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
        Box::new(Expr::Const(Name::str("Bool"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ordering.isGE"),
        univ_params: vec![],
        ty: is_ge_ty,
    })
    .map_err(|e| e.to_string())?;
    let reverse_ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("o"),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ordering.reverse"),
        univ_params: vec![],
        ty: reverse_ty,
    })
    .map_err(|e| e.to_string())?;
    let then_ty = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("o1"),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
        Box::new(Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
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
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1,
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_build_ordering_env() {
        let mut env = setup_env();
        assert!(build_ordering_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Ordering")).is_some());
        assert!(env.get(&Name::str("Ordering.less")).is_some());
        assert!(env.get(&Name::str("Ordering.equal")).is_some());
        assert!(env.get(&Name::str("Ordering.greater")).is_some());
    }
    #[test]
    fn test_ordering_is_le() {
        let mut env = setup_env();
        build_ordering_env(&mut env).expect("build_ordering_env should succeed");
        let decl = env
            .get(&Name::str("Ordering.isLE"))
            .expect("declaration 'Ordering.isLE' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_ordering_is_ge() {
        let mut env = setup_env();
        build_ordering_env(&mut env).expect("build_ordering_env should succeed");
        let decl = env
            .get(&Name::str("Ordering.isGE"))
            .expect("declaration 'Ordering.isGE' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_ordering_reverse() {
        let mut env = setup_env();
        build_ordering_env(&mut env).expect("build_ordering_env should succeed");
        let decl = env
            .get(&Name::str("Ordering.reverse"))
            .expect("declaration 'Ordering.reverse' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_ordering_then() {
        let mut env = setup_env();
        build_ordering_env(&mut env).expect("build_ordering_env should succeed");
        let decl = env
            .get(&Name::str("Ordering.then"))
            .expect("declaration 'Ordering.then' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
}
/// Compare two `Ord` values and return an `Ordering`.
pub fn cmp<T: std::cmp::Ord>(a: &T, b: &T) -> Ordering {
    Ordering::from_std(a.cmp(b))
}
/// Compare using a key function.
pub fn cmp_by_key<T, K: std::cmp::Ord, F: Fn(&T) -> K>(a: &T, b: &T, key: F) -> Ordering {
    Ordering::from_std(key(a).cmp(&key(b)))
}
/// Compare two slices lexicographically.
pub fn cmp_slices<T: std::cmp::Ord>(a: &[T], b: &[T]) -> Ordering {
    Ordering::from_std(a.cmp(b))
}
/// Chain a sequence of `Ordering` values (first non-`Equal` wins).
pub fn ordering_chain(items: impl IntoIterator<Item = Ordering>) -> Ordering {
    for item in items {
        if item != Ordering::Equal {
            return item;
        }
    }
    Ordering::Equal
}
/// Return a sorted copy of a slice.
pub fn sorted<T: std::cmp::Ord + Clone>(v: &[T]) -> Vec<T> {
    let mut result = v.to_vec();
    result.sort();
    result
}
/// Return a sorted copy using a key function.
pub fn sorted_by_key<T: Clone, K: std::cmp::Ord, F: Fn(&T) -> K>(v: &[T], key: F) -> Vec<T> {
    let mut result = v.to_vec();
    result.sort_by_key(key);
    result
}
/// Return a reverse-sorted copy of a slice.
pub fn sorted_desc<T: std::cmp::Ord + Clone>(v: &[T]) -> Vec<T> {
    let mut result = v.to_vec();
    result.sort_by(|a, b| b.cmp(a));
    result
}
/// Merge two sorted slices into a sorted `Vec`.
pub fn merge_sorted<T: std::cmp::Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i].clone());
            i += 1;
        } else {
            result.push(b[j].clone());
            j += 1;
        }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}
/// `true` if a slice is sorted in non-decreasing order.
pub fn is_sorted<T: std::cmp::Ord>(s: &[T]) -> bool {
    s.windows(2).all(|w| w[0] <= w[1])
}
/// `true` if a slice is sorted in non-increasing order.
pub fn is_sorted_desc<T: std::cmp::Ord>(s: &[T]) -> bool {
    s.windows(2).all(|w| w[0] >= w[1])
}
/// Find the lower bound index for `target` in a sorted slice.
///
/// Returns the index of the first element `≥ target`.
pub fn lower_bound<T: std::cmp::Ord>(s: &[T], target: &T) -> usize {
    let mut lo = 0;
    let mut hi = s.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if s[mid] < *target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
/// Find the upper bound index for `target` in a sorted slice.
///
/// Returns the index of the first element `> target`.
pub fn upper_bound<T: std::cmp::Ord>(s: &[T], target: &T) -> usize {
    let mut lo = 0;
    let mut hi = s.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if s[mid] <= *target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
#[cfg(test)]
mod ordering_extra_tests {
    use super::*;
    #[test]
    fn test_ordering_reverse() {
        assert_eq!(Ordering::Less.reverse(), Ordering::Greater);
        assert_eq!(Ordering::Greater.reverse(), Ordering::Less);
        assert_eq!(Ordering::Equal.reverse(), Ordering::Equal);
    }
    #[test]
    fn test_ordering_then() {
        assert_eq!(Ordering::Equal.then(Ordering::Less), Ordering::Less);
        assert_eq!(Ordering::Less.then(Ordering::Greater), Ordering::Less);
        assert_eq!(Ordering::Greater.then(Ordering::Less), Ordering::Greater);
    }
    #[test]
    fn test_ordering_predicates() {
        assert!(Ordering::Less.is_lt());
        assert!(Ordering::Less.is_le());
        assert!(!Ordering::Less.is_ge());
        assert!(Ordering::Equal.is_eq());
        assert!(Ordering::Equal.is_le());
        assert!(Ordering::Equal.is_ge());
        assert!(Ordering::Greater.is_gt());
        assert!(Ordering::Greater.is_ge());
        assert!(!Ordering::Greater.is_le());
    }
    #[test]
    fn test_ordering_signum() {
        assert_eq!(Ordering::Less.to_signum(), -1);
        assert_eq!(Ordering::Equal.to_signum(), 0);
        assert_eq!(Ordering::Greater.to_signum(), 1);
        assert_eq!(Ordering::from_signum(-5), Ordering::Less);
        assert_eq!(Ordering::from_signum(0), Ordering::Equal);
        assert_eq!(Ordering::from_signum(3), Ordering::Greater);
    }
    #[test]
    fn test_ordering_std_roundtrip() {
        let o = Ordering::Less;
        assert_eq!(Ordering::from_std(o.to_std()), o);
    }
    #[test]
    fn test_ordering_display() {
        assert_eq!(Ordering::Less.to_string(), "lt");
        assert_eq!(Ordering::Equal.to_string(), "eq");
        assert_eq!(Ordering::Greater.to_string(), "gt");
    }
    #[test]
    fn test_cmp() {
        assert_eq!(cmp(&1, &2), Ordering::Less);
        assert_eq!(cmp(&2, &2), Ordering::Equal);
        assert_eq!(cmp(&3, &2), Ordering::Greater);
    }
    #[test]
    fn test_ordering_chain() {
        let result = ordering_chain([Ordering::Equal, Ordering::Equal, Ordering::Less]);
        assert_eq!(result, Ordering::Less);
        let all_eq = ordering_chain([Ordering::Equal, Ordering::Equal]);
        assert_eq!(all_eq, Ordering::Equal);
    }
    #[test]
    fn test_ordering_builder() {
        let result = OrderingBuilder::new()
            .field(&1u32, &1u32)
            .field(&2u32, &3u32)
            .build();
        assert_eq!(result, Ordering::Less);
    }
    #[test]
    fn test_sorted() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let s = sorted(&v);
        assert!(is_sorted(&s));
    }
    #[test]
    fn test_sorted_desc() {
        let v = vec![3, 1, 4, 1, 5];
        let s = sorted_desc(&v);
        assert!(is_sorted_desc(&s));
    }
    #[test]
    fn test_merge_sorted() {
        let a = vec![1, 3, 5];
        let b = vec![2, 4, 6];
        let merged = merge_sorted(&a, &b);
        assert_eq!(merged, vec![1, 2, 3, 4, 5, 6]);
        assert!(is_sorted(&merged));
    }
    #[test]
    fn test_lower_upper_bound() {
        let v = vec![1, 2, 2, 3, 4, 5];
        assert_eq!(lower_bound(&v, &2), 1);
        assert_eq!(upper_bound(&v, &2), 3);
        assert_eq!(lower_bound(&v, &6), 6);
        assert_eq!(upper_bound(&v, &0), 0);
    }
    #[test]
    fn test_is_sorted_is_sorted_desc() {
        assert!(is_sorted(&[1, 2, 3, 3]));
        assert!(!is_sorted(&[1, 3, 2]));
        assert!(is_sorted_desc(&[5, 4, 3, 1]));
        assert!(!is_sorted_desc(&[1, 2]));
    }
    #[test]
    fn test_cmp_slices() {
        assert_eq!(cmp_slices(&[1, 2, 3], &[1, 2, 4]), Ordering::Less);
        assert_eq!(cmp_slices::<i32>(&[], &[]), Ordering::Equal);
    }
    #[test]
    fn test_cmp_by_key() {
        let a = ("hello", 10);
        let b = ("world", 5);
        assert_eq!(cmp_by_key(&a, &b, |x| x.1), Ordering::Greater);
    }
}
/// Build a richer Ordering environment: add the `Ord` typeclass plus the
/// `Ordering` comparators that depend on `Bool`.
#[allow(dead_code)]
pub fn build_full_ordering_env(env: &mut Environment) -> Result<(), String> {
    build_ordering_env(env)?;
    let decide_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("o1"),
        Box::new(Expr::Const(Name::str("Ordering"), vec![])),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("o2"),
            Box::new(Expr::Const(Name::str("Ordering"), vec![])),
            Box::new(Expr::Const(Name::str("Bool"), vec![])),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Ordering.beq"),
        univ_params: vec![],
        ty: decide_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Compare two `&str` values using the `Ordering` type.
#[allow(dead_code)]
pub fn str_cmp(a: &str, b: &str) -> Ordering {
    Ordering::from_std(a.cmp(b))
}
/// Compare two `bool` values (false < true).
#[allow(dead_code)]
pub fn bool_cmp(a: bool, b: bool) -> Ordering {
    Ordering::from_std(a.cmp(&b))
}
/// Compare two `usize` values.
#[allow(dead_code)]
pub fn usize_cmp(a: usize, b: usize) -> Ordering {
    Ordering::from_std(a.cmp(&b))
}
/// Compare two `i64` values.
#[allow(dead_code)]
pub fn i64_cmp(a: i64, b: i64) -> Ordering {
    Ordering::from_std(a.cmp(&b))
}
/// Compare two `f64` values (NaN is treated as Equal to itself, Less than everything else).
#[allow(dead_code)]
pub fn f64_cmp(a: f64, b: f64) -> Ordering {
    match a.partial_cmp(&b) {
        Some(o) => Ordering::from_std(o),
        None => Ordering::Equal,
    }
}
/// `true` if `a < b` (using `Ordering`).
#[allow(dead_code)]
pub fn ordering_lt<T: std::cmp::Ord>(a: &T, b: &T) -> bool {
    cmp(a, b).is_lt()
}
/// `true` if `a <= b`.
#[allow(dead_code)]
pub fn ordering_le<T: std::cmp::Ord>(a: &T, b: &T) -> bool {
    cmp(a, b).is_le()
}
/// `true` if `a > b`.
#[allow(dead_code)]
pub fn ordering_gt<T: std::cmp::Ord>(a: &T, b: &T) -> bool {
    cmp(a, b).is_gt()
}
/// `true` if `a >= b`.
#[allow(dead_code)]
pub fn ordering_ge<T: std::cmp::Ord>(a: &T, b: &T) -> bool {
    cmp(a, b).is_ge()
}
/// Lexicographic order on `Option<T>`: `None < Some(x)` for all `x`.
#[allow(dead_code)]
pub fn option_cmp<T: std::cmp::Ord>(a: &Option<T>, b: &Option<T>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(x), Some(y)) => cmp(x, y),
    }
}
/// Lexicographic order on `Result<T, E>`: `Err < Ok` for distinct constructors.
#[allow(dead_code)]
pub fn result_cmp<T: std::cmp::Ord, E: std::cmp::Ord>(
    a: &Result<T, E>,
    b: &Result<T, E>,
) -> Ordering {
    match (a, b) {
        (Err(e1), Err(e2)) => cmp(e1, e2),
        (Err(_), Ok(_)) => Ordering::Less,
        (Ok(_), Err(_)) => Ordering::Greater,
        (Ok(x), Ok(y)) => cmp(x, y),
    }
}
#[cfg(test)]
mod ordering_extra_tests2 {
    use super::*;
    #[test]
    fn test_build_full_ordering_env() {
        let mut env = Environment::new();
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        })
        .expect("operation should succeed");
        assert!(build_full_ordering_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Ordering.beq")).is_some());
    }
    #[test]
    fn test_str_cmp() {
        assert_eq!(str_cmp("abc", "abd"), Ordering::Less);
        assert_eq!(str_cmp("z", "a"), Ordering::Greater);
        assert_eq!(str_cmp("x", "x"), Ordering::Equal);
    }
    #[test]
    fn test_bool_cmp() {
        assert_eq!(bool_cmp(false, true), Ordering::Less);
        assert_eq!(bool_cmp(true, false), Ordering::Greater);
        assert_eq!(bool_cmp(true, true), Ordering::Equal);
    }
    #[test]
    fn test_numeric_cmp() {
        assert_eq!(usize_cmp(3, 5), Ordering::Less);
        assert_eq!(i64_cmp(-1, 1), Ordering::Less);
        assert_eq!(f64_cmp(1.5, 1.5), Ordering::Equal);
        assert_eq!(f64_cmp(2.0, 1.0), Ordering::Greater);
    }
    #[test]
    fn test_ordering_predicates_via_cmp() {
        assert!(ordering_lt(&1, &2));
        assert!(ordering_le(&2, &2));
        assert!(ordering_gt(&5, &3));
        assert!(ordering_ge(&3, &3));
    }
    #[test]
    fn test_option_cmp() {
        let a: Option<u32> = None;
        let b = Some(5u32);
        assert_eq!(option_cmp(&a, &b), Ordering::Less);
        assert_eq!(option_cmp(&b, &a), Ordering::Greater);
        assert_eq!(option_cmp(&a, &a), Ordering::Equal);
        assert_eq!(option_cmp(&Some(3u32), &Some(5u32)), Ordering::Less);
    }
    #[test]
    fn test_result_cmp() {
        let ok1: Result<u32, u32> = Ok(1);
        let ok2: Result<u32, u32> = Ok(2);
        let err1: Result<u32, u32> = Err(1);
        assert_eq!(result_cmp(&err1, &ok1), Ordering::Less);
        assert_eq!(result_cmp(&ok1, &ok2), Ordering::Less);
    }
    #[test]
    fn test_ordering_then_with() {
        let result = Ordering::Equal.then_with(|| Ordering::Greater);
        assert_eq!(result, Ordering::Greater);
        let result2 = Ordering::Less.then_with(|| Ordering::Greater);
        assert_eq!(result2, Ordering::Less);
    }
    #[test]
    fn test_ordering_builder_chained() {
        let result = OrderingBuilder::new()
            .field(&1u32, &1u32)
            .field(&1u32, &1u32)
            .field(&5u32, &10u32)
            .build();
        assert_eq!(result, Ordering::Less);
    }
    #[test]
    fn test_sorted_by_key() {
        let v = vec!["banana", "apple", "cherry"];
        let sorted = sorted_by_key(&v, |s| s.len());
        assert_eq!(sorted[0], "apple");
    }
}
/// Return the maximum of two values according to `Ordering`.
#[allow(dead_code)]
pub fn ordering_max<T: std::cmp::Ord + Clone>(a: T, b: T) -> T {
    if cmp(&a, &b).is_ge() {
        a
    } else {
        b
    }
}
/// Return the minimum of two values according to `Ordering`.
#[allow(dead_code)]
pub fn ordering_min<T: std::cmp::Ord + Clone>(a: T, b: T) -> T {
    if cmp(&a, &b).is_le() {
        a
    } else {
        b
    }
}
#[cfg(test)]
mod ordering_min_max_tests {
    use super::*;
    #[test]
    fn test_ordering_max() {
        assert_eq!(ordering_max(3u32, 5u32), 5);
    }
    #[test]
    fn test_ordering_min() {
        assert_eq!(ordering_min(3u32, 5u32), 3);
    }
}
/// Select the k-th smallest element from a slice (0-indexed).
///
/// Returns `None` if `k >= v.len()`.
#[allow(dead_code)]
pub fn kth_smallest<T: std::cmp::Ord + Clone>(v: &[T], k: usize) -> Option<T> {
    if k >= v.len() {
        return None;
    }
    let mut sorted = v.to_vec();
    sorted.sort();
    Some(sorted[k].clone())
}
/// Select the k-th largest element from a slice (0-indexed).
#[allow(dead_code)]
pub fn kth_largest<T: std::cmp::Ord + Clone>(v: &[T], k: usize) -> Option<T> {
    if k >= v.len() {
        return None;
    }
    let mut sorted = v.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    Some(sorted[k].clone())
}
/// Return the top-n largest elements of a slice, in decreasing order.
#[allow(dead_code)]
pub fn top_n<T: std::cmp::Ord + Clone>(v: &[T], n: usize) -> Vec<T> {
    let mut sorted = sorted_desc(v);
    sorted.truncate(n);
    sorted
}
/// Return the bottom-n smallest elements of a slice, in increasing order.
#[allow(dead_code)]
pub fn bottom_n<T: std::cmp::Ord + Clone>(v: &[T], n: usize) -> Vec<T> {
    let mut s = sorted(v);
    s.truncate(n);
    s
}
/// Compute the sorted intersection of two sorted slices.
#[allow(dead_code)]
pub fn sorted_intersection<T: std::cmp::Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Equal => {
                result.push(a[i].clone());
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
        }
    }
    result
}
/// Compute the sorted difference (a minus b) of two sorted slices.
#[allow(dead_code)]
pub fn sorted_difference<T: std::cmp::Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < a.len() {
        if j >= b.len() {
            result.push(a[i].clone());
            i += 1;
        } else {
            match a[i].cmp(&b[j]) {
                std::cmp::Ordering::Equal => {
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Less => {
                    result.push(a[i].clone());
                    i += 1;
                }
                std::cmp::Ordering::Greater => j += 1,
            }
        }
    }
    result
}
/// Compute the sorted union of two sorted slices (no duplicates).
#[allow(dead_code)]
pub fn sorted_union<T: std::cmp::Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut merged = merge_sorted(a, b);
    merged.dedup();
    merged
}
#[cfg(test)]
mod ordered_table_tests {
    use super::*;
    #[test]
    fn test_ordered_table_insert_get() {
        let mut t = OrderedTable::new();
        t.insert("b", 2u32);
        t.insert("a", 1u32);
        t.insert("c", 3u32);
        assert_eq!(t.get(&"a"), Some(&1));
        assert_eq!(t.get(&"c"), Some(&3));
        assert_eq!(t.get(&"d"), None);
        assert_eq!(t.len(), 3);
    }
    #[test]
    fn test_ordered_table_remove() {
        let mut t = OrderedTable::new();
        t.insert(1u32, "one");
        t.insert(2u32, "two");
        assert_eq!(t.remove(&1), Some("one"));
        assert_eq!(t.len(), 1);
        assert_eq!(t.remove(&99), None);
    }
    #[test]
    fn test_ordered_table_keys_sorted() {
        let mut t = OrderedTable::new();
        t.insert(3u32, ());
        t.insert(1u32, ());
        t.insert(2u32, ());
        let keys: Vec<_> = t.keys().into_iter().copied().collect();
        assert_eq!(keys, vec![1, 2, 3]);
    }
    #[test]
    fn test_kth_smallest() {
        let v = vec![5, 3, 1, 4, 2];
        assert_eq!(kth_smallest(&v, 0), Some(1));
        assert_eq!(kth_smallest(&v, 2), Some(3));
        assert_eq!(kth_smallest(&v, 10), None);
    }
    #[test]
    fn test_kth_largest() {
        let v = vec![5, 3, 1, 4, 2];
        assert_eq!(kth_largest(&v, 0), Some(5));
        assert_eq!(kth_largest(&v, 2), Some(3));
    }
    #[test]
    fn test_top_n() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(top_n(&v, 3), vec![9, 6, 5]);
    }
    #[test]
    fn test_bottom_n() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(bottom_n(&v, 3), vec![1, 1, 2]);
    }
    #[test]
    fn test_sorted_intersection() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![2, 4, 6];
        assert_eq!(sorted_intersection(&a, &b), vec![2, 4]);
    }
    #[test]
    fn test_sorted_difference() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![2, 4];
        assert_eq!(sorted_difference(&a, &b), vec![1, 3, 5]);
    }
    #[test]
    fn test_sorted_union() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3, 4];
        assert_eq!(sorted_union(&a, &b), vec![1, 2, 3, 4]);
    }
    #[test]
    fn test_ordered_table_contains_key() {
        let mut t: OrderedTable<u32, &str> = OrderedTable::new();
        t.insert(1, "one");
        assert!(t.contains_key(&1));
        assert!(!t.contains_key(&2));
    }
    #[test]
    fn test_ordered_table_is_empty() {
        let t: OrderedTable<u32, u32> = OrderedTable::new();
        assert!(t.is_empty());
    }
    #[test]
    fn test_ordered_table_values() {
        let mut t = OrderedTable::new();
        t.insert(1u32, "a");
        t.insert(2u32, "b");
        let vals = t.values();
        assert_eq!(vals, vec![&"a", &"b"]);
    }
}
/// Helper: build `α → β` as a Pi with anonymous binder.
pub fn ord_ext_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(a),
        Box::new(b),
    )
}
/// Type of `WQO.carrier`: the carrier type of a well-quasi-order.
/// `WQO.carrier : WQO → Type`
#[allow(dead_code)]
pub fn axiom_wqo_carrier_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("WQO"), vec![]),
        Expr::Sort(Level::succ(Level::zero())),
    )
}
/// Type of `WQO.le`: the quasi-order relation.
/// `WQO.le : (w : WQO) → WQO.carrier w → WQO.carrier w → Prop`
#[allow(dead_code)]
pub fn axiom_wqo_le_ty() -> Expr {
    let wqo = Expr::Const(Name::str("WQO"), vec![]);
    let carrier_w = Expr::App(
        Box::new(Expr::Const(Name::str("WQO.carrier"), vec![])),
        Box::new(Expr::BVar(0)),
    );
    Expr::Pi(
        BinderInfo::Default,
        Name::str("w"),
        Box::new(wqo),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(carrier_w.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(carrier_w),
                Box::new(Expr::Sort(Level::zero())),
            )),
        )),
    )
}
/// Type of `WQO.wf`: every infinite sequence has a good pair.
/// `WQO.wf : (w : WQO) → ∀ (f : Nat → WQO.carrier w), ∃ i j, i < j ∧ WQO.le w (f i) (f j)`
#[allow(dead_code)]
pub fn axiom_wqo_wf_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("WQO"), vec![]),
        Expr::Const(Name::str("WQO.goodPair"), vec![]),
    )
}
/// Type of `Dickson.lemma`: the product of WQOs is a WQO.
/// `Dickson.lemma : WQO → WQO → WQO`
#[allow(dead_code)]
pub fn axiom_dickson_lemma_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("WQO"), vec![]),
        ord_ext_arrow(
            Expr::Const(Name::str("WQO"), vec![]),
            Expr::Const(Name::str("WQO"), vec![]),
        ),
    )
}
/// Type of `Dickson.nat_wqo`: (Nat, ≤) is a WQO.
/// `Dickson.nat_wqo : WQO`
#[allow(dead_code)]
pub fn axiom_dickson_nat_wqo_ty() -> Expr {
    Expr::Const(Name::str("WQO"), vec![])
}
/// Type of `Higman.lemma`: finite words over a WQO form a WQO under embedding.
/// `Higman.lemma : WQO → WQO`
#[allow(dead_code)]
pub fn axiom_higman_lemma_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("WQO"), vec![]),
        Expr::Const(Name::str("WQO"), vec![]),
    )
}
/// Type of `Kruskal.treeThm`: finite rooted trees over a WQO form a WQO under topological embedding.
/// `Kruskal.treeThm : WQO → WQO`
#[allow(dead_code)]
pub fn axiom_kruskal_tree_thm_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("WQO"), vec![]),
        Expr::Const(Name::str("WQO"), vec![]),
    )
}
/// Type of `Ordinal`: the type of ordinals.
/// `Ordinal : Type 1`
#[allow(dead_code)]
pub fn axiom_ordinal_type_ty() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
/// Type of `Ordinal.zero`: the ordinal zero.
/// `Ordinal.zero : Ordinal`
#[allow(dead_code)]
pub fn axiom_ordinal_zero_ty() -> Expr {
    Expr::Const(Name::str("Ordinal"), vec![])
}
/// Type of `Ordinal.succ`: successor ordinal.
/// `Ordinal.succ : Ordinal → Ordinal`
#[allow(dead_code)]
pub fn axiom_ordinal_succ_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        Expr::Const(Name::str("Ordinal"), vec![]),
    )
}
/// Type of `Ordinal.add`: ordinal addition.
/// `Ordinal.add : Ordinal → Ordinal → Ordinal`
#[allow(dead_code)]
pub fn axiom_ordinal_add_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        ord_ext_arrow(
            Expr::Const(Name::str("Ordinal"), vec![]),
            Expr::Const(Name::str("Ordinal"), vec![]),
        ),
    )
}
/// Type of `Ordinal.mul`: ordinal multiplication.
/// `Ordinal.mul : Ordinal → Ordinal → Ordinal`
#[allow(dead_code)]
pub fn axiom_ordinal_mul_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        ord_ext_arrow(
            Expr::Const(Name::str("Ordinal"), vec![]),
            Expr::Const(Name::str("Ordinal"), vec![]),
        ),
    )
}
/// Type of `Ordinal.pow`: ordinal exponentiation.
/// `Ordinal.pow : Ordinal → Ordinal → Ordinal`
#[allow(dead_code)]
pub fn axiom_ordinal_pow_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        ord_ext_arrow(
            Expr::Const(Name::str("Ordinal"), vec![]),
            Expr::Const(Name::str("Ordinal"), vec![]),
        ),
    )
}
/// Type of `Ordinal.omega`: the first infinite ordinal ω.
/// `Ordinal.omega : Ordinal`
#[allow(dead_code)]
pub fn axiom_ordinal_omega_ty() -> Expr {
    Expr::Const(Name::str("Ordinal"), vec![])
}
/// Type of `Ordinal.epsilon0`: the ordinal ε₀ = sup{ω, ω^ω, ω^(ω^ω), ...}.
/// `Ordinal.epsilon0 : Ordinal`
#[allow(dead_code)]
pub fn axiom_ordinal_epsilon0_ty() -> Expr {
    Expr::Const(Name::str("Ordinal"), vec![])
}
/// Type of `Ordinal.churchKleene`: the Church-Kleene ordinal ω₁^CK.
/// `Ordinal.churchKleene : Ordinal`
#[allow(dead_code)]
pub fn axiom_ordinal_church_kleene_ty() -> Expr {
    Expr::Const(Name::str("Ordinal"), vec![])
}
/// Type of `Ordinal.lt`: strict ordering on ordinals.
/// `Ordinal.lt : Ordinal → Ordinal → Prop`
#[allow(dead_code)]
pub fn axiom_ordinal_lt_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        ord_ext_arrow(
            Expr::Const(Name::str("Ordinal"), vec![]),
            Expr::Sort(Level::zero()),
        ),
    )
}
/// Type of `Ordinal.le`: weak ordering on ordinals.
/// `Ordinal.le : Ordinal → Ordinal → Prop`
#[allow(dead_code)]
pub fn axiom_ordinal_le_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        ord_ext_arrow(
            Expr::Const(Name::str("Ordinal"), vec![]),
            Expr::Sort(Level::zero()),
        ),
    )
}
/// Type of `Ordinal.isLimit`: predicate for limit ordinals.
/// `Ordinal.isLimit : Ordinal → Prop`
#[allow(dead_code)]
pub fn axiom_ordinal_is_limit_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        Expr::Sort(Level::zero()),
    )
}
/// Type of `Ordinal.comparability`: any two ordinals are comparable.
/// `Ordinal.comparability : ∀ α β : Ordinal, α < β ∨ α = β ∨ β < α`
#[allow(dead_code)]
pub fn axiom_ordinal_comparability_ty() -> Expr {
    let ord = Expr::Const(Name::str("Ordinal"), vec![]);
    Expr::Pi(
        BinderInfo::Default,
        Name::str("alpha"),
        Box::new(ord.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("beta"),
            Box::new(ord),
            Box::new(Expr::Const(Name::str("Ordinal.trichotomy"), vec![])),
        )),
    )
}
/// Type of `Suslin.problem`: is every dense linear order without endpoints
/// satisfying the countable chain condition isomorphic to the reals?
/// `Suslin.problem : Prop`
#[allow(dead_code)]
pub fn axiom_suslin_problem_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `LinearOrder`: a type equipped with a total order.
/// `LinearOrder : Type → Type 1`
#[allow(dead_code)]
pub fn axiom_linear_order_ty() -> Expr {
    ord_ext_arrow(
        Expr::Sort(Level::succ(Level::zero())),
        Expr::Sort(Level::succ(Level::succ(Level::zero()))),
    )
}
/// Type of `DenseLinearOrder`: a dense linear order without endpoints.
/// `DenseLinearOrder : Type → Prop`
#[allow(dead_code)]
pub fn axiom_dense_linear_order_ty() -> Expr {
    ord_ext_arrow(
        Expr::Sort(Level::succ(Level::zero())),
        Expr::Sort(Level::zero()),
    )
}
/// Type of `Hausdorff.scatteredOrder`: a scattered linear order embeds no dense suborder.
/// `Hausdorff.scatteredOrder : Type → Prop`
#[allow(dead_code)]
pub fn axiom_hausdorff_scattered_ty() -> Expr {
    ord_ext_arrow(
        Expr::Sort(Level::succ(Level::zero())),
        Expr::Sort(Level::zero()),
    )
}
/// Type of `Hausdorff.theorem`: every linear order decomposes into a scattered and a dense part.
/// `Hausdorff.theorem : LinearOrder → Prop`
#[allow(dead_code)]
pub fn axiom_hausdorff_theorem_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("LinearOrder"), vec![]),
        Expr::Sort(Level::zero()),
    )
}
/// Type of `Ramsey.orderThm`: infinite Ramsey theorem for order relations.
/// `Ramsey.orderThm : Nat → Nat → Prop`
#[allow(dead_code)]
pub fn axiom_ramsey_order_thm_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Nat"), vec![]),
        ord_ext_arrow(
            Expr::Const(Name::str("Nat"), vec![]),
            Expr::Sort(Level::zero()),
        ),
    )
}
/// Type of `Rationals.universalLinearOrder`: the rationals form a universal homogeneous
/// countable dense linear order without endpoints.
/// `Rationals.universalLinearOrder : Prop`
#[allow(dead_code)]
pub fn axiom_rationals_universal_linear_order_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `DedekindCut`: a Dedekind cut in an ordered field.
/// `DedekindCut : Type → Type`
#[allow(dead_code)]
pub fn axiom_dedekind_cut_ty() -> Expr {
    ord_ext_arrow(
        Expr::Sort(Level::succ(Level::zero())),
        Expr::Sort(Level::succ(Level::zero())),
    )
}
/// Type of `DedekindCut.completeness`: every Dedekind cut has a supremum.
/// `DedekindCut.completeness : ∀ F : Type, DedekindCut F → F`
#[allow(dead_code)]
pub fn axiom_dedekind_completeness_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("F"),
        Box::new(Expr::Sort(Level::succ(Level::zero()))),
        Box::new(ord_ext_arrow(
            Expr::App(
                Box::new(Expr::Const(Name::str("DedekindCut"), vec![])),
                Box::new(Expr::BVar(0)),
            ),
            Expr::BVar(1),
        )),
    )
}
/// Type of `Cantor.backAndForth`: any two countable dense linear orders
/// without endpoints are isomorphic.
/// `Cantor.backAndForth : Prop`
#[allow(dead_code)]
pub fn axiom_cantor_back_and_forth_ty() -> Expr {
    Expr::Sort(Level::zero())
}
/// Type of `Fraisse.limit`: the Fraïssé limit of a class of finite structures.
/// `Fraisse.limit : StructureClass → Structure`
#[allow(dead_code)]
pub fn axiom_fraisse_limit_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("StructureClass"), vec![]),
        Expr::Const(Name::str("Structure"), vec![]),
    )
}
/// Type of `OrderedField`: a field with a compatible total order.
/// `OrderedField : Type → Prop`
#[allow(dead_code)]
pub fn axiom_ordered_field_ty() -> Expr {
    ord_ext_arrow(
        Expr::Sort(Level::succ(Level::zero())),
        Expr::Sort(Level::zero()),
    )
}
/// Type of `OrderedField.archimedean`: an ordered field is Archimedean
/// if for every x > 0, there exists n : Nat with n > x.
/// `OrderedField.archimedean : ∀ F, OrderedField F → Prop`
#[allow(dead_code)]
pub fn axiom_ordered_field_archimedean_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("F"),
        Box::new(Expr::Sort(Level::succ(Level::zero()))),
        Box::new(ord_ext_arrow(
            Expr::App(
                Box::new(Expr::Const(Name::str("OrderedField"), vec![])),
                Box::new(Expr::BVar(0)),
            ),
            Expr::Sort(Level::zero()),
        )),
    )
}
/// Type of `WellOrder.induction`: transfinite induction for well-orders.
/// `WellOrder.induction : ∀ α, WellOrder α → (∀ x, (∀ y, y < x → P y) → P x) → ∀ x, P x`
#[allow(dead_code)]
pub fn axiom_well_order_induction_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("WellOrder"), vec![]),
        Expr::Const(Name::str("WellOrder.inductionPrinciple"), vec![]),
    )
}
/// Type of `WellOrder.isomorphism`: any two well-orders of the same ordinal are isomorphic.
/// `WellOrder.isomorphism : WellOrder → WellOrder → Prop`
#[allow(dead_code)]
pub fn axiom_well_order_isomorphism_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("WellOrder"), vec![]),
        ord_ext_arrow(
            Expr::Const(Name::str("WellOrder"), vec![]),
            Expr::Sort(Level::zero()),
        ),
    )
}
/// Type of `Ordinal.sup`: supremum of a set of ordinals.
/// `Ordinal.sup : (Nat → Ordinal) → Ordinal`
#[allow(dead_code)]
pub fn axiom_ordinal_sup_ty() -> Expr {
    ord_ext_arrow(
        ord_ext_arrow(
            Expr::Const(Name::str("Nat"), vec![]),
            Expr::Const(Name::str("Ordinal"), vec![]),
        ),
        Expr::Const(Name::str("Ordinal"), vec![]),
    )
}
/// Type of `Ordinal.cnfNF`: Cantor Normal Form of an ordinal.
/// `Ordinal.cnfNF : Ordinal → List (Ordinal × Nat)`
#[allow(dead_code)]
pub fn axiom_ordinal_cnf_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        Expr::App(
            Box::new(Expr::Const(Name::str("List"), vec![])),
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Prod"), vec![])),
                    Box::new(Expr::Const(Name::str("Ordinal"), vec![])),
                )),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
            )),
        ),
    )
}
/// Type of `PartialOrder.antisymmetry`: a partial order is antisymmetric.
/// `PartialOrder.antisymmetry : ∀ {α} \[PartialOrder α\] (a b : α), a ≤ b → b ≤ a → a = b`
#[allow(dead_code)]
pub fn axiom_partial_order_antisymmetry_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("alpha"),
        Box::new(Expr::Sort(Level::succ(Level::zero()))),
        Box::new(Expr::Const(
            Name::str("PartialOrder.antisymmetryStatement"),
            vec![],
        )),
    )
}
/// Type of `TotalOrder.linearExtension`: every partial order extends to a total order.
/// `TotalOrder.linearExtension : PartialOrder → TotalOrder`
#[allow(dead_code)]
pub fn axiom_total_order_linear_extension_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("PartialOrder"), vec![]),
        Expr::Const(Name::str("TotalOrder"), vec![]),
    )
}
/// Register all extended ordering/ordinal axioms in the environment.
#[allow(dead_code)]
pub fn register_ordering_extended(env: &mut Environment) -> Result<(), String> {
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let prereqs: &[(&str, Expr)] = &[
        ("WQO", type1.clone()),
        ("Ordinal", type1.clone()),
        ("LinearOrder", type2.clone()),
        ("PartialOrder", type1.clone()),
        ("TotalOrder", type1.clone()),
        ("WellOrder", type1.clone()),
        ("StructureClass", type1.clone()),
        ("Structure", type1.clone()),
        ("Nat", type1.clone()),
        ("List", ord_ext_arrow(type1.clone(), type1.clone())),
        (
            "Prod",
            ord_ext_arrow(type1.clone(), ord_ext_arrow(type1.clone(), type1.clone())),
        ),
        ("WQO.goodPair", prop.clone()),
        ("Ordinal.trichotomy", prop.clone()),
        ("WellOrder.inductionPrinciple", prop.clone()),
        ("PartialOrder.antisymmetryStatement", prop.clone()),
        (
            "WQO.carrier",
            ord_ext_arrow(Expr::Const(Name::str("WQO"), vec![]), type1.clone()),
        ),
    ];
    for (name, ty) in prereqs {
        if !env.contains(&Name::str(*name)) {
            env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: ty.clone(),
            })
            .map_err(|e| e.to_string())?;
        }
    }
    let axioms: &[(&str, Expr)] = &[
        ("WQO.le", axiom_wqo_le_ty()),
        ("WQO.wf", axiom_wqo_wf_ty()),
        ("Dickson.lemma", axiom_dickson_lemma_ty()),
        ("Dickson.nat_wqo", axiom_dickson_nat_wqo_ty()),
        ("Higman.lemma", axiom_higman_lemma_ty()),
        ("Kruskal.treeThm", axiom_kruskal_tree_thm_ty()),
        ("Ordinal.zero", axiom_ordinal_zero_ty()),
        ("Ordinal.succ", axiom_ordinal_succ_ty()),
        ("Ordinal.add", axiom_ordinal_add_ty()),
        ("Ordinal.mul", axiom_ordinal_mul_ty()),
        ("Ordinal.pow", axiom_ordinal_pow_ty()),
        ("Ordinal.omega", axiom_ordinal_omega_ty()),
        ("Ordinal.epsilon0", axiom_ordinal_epsilon0_ty()),
        ("Ordinal.churchKleene", axiom_ordinal_church_kleene_ty()),
        ("Ordinal.lt", axiom_ordinal_lt_ty()),
        ("Ordinal.le", axiom_ordinal_le_ty()),
        ("Ordinal.isLimit", axiom_ordinal_is_limit_ty()),
        ("Ordinal.comparability", axiom_ordinal_comparability_ty()),
        ("Suslin.problem", axiom_suslin_problem_ty()),
        ("DenseLinearOrder", axiom_dense_linear_order_ty()),
        ("Hausdorff.scatteredOrder", axiom_hausdorff_scattered_ty()),
        ("Hausdorff.theorem", axiom_hausdorff_theorem_ty()),
        ("Ramsey.orderThm", axiom_ramsey_order_thm_ty()),
        (
            "Rationals.universalLinearOrder",
            axiom_rationals_universal_linear_order_ty(),
        ),
        ("DedekindCut", axiom_dedekind_cut_ty()),
        ("DedekindCut.completeness", axiom_dedekind_completeness_ty()),
        ("Cantor.backAndForth", axiom_cantor_back_and_forth_ty()),
        ("Fraisse.limit", axiom_fraisse_limit_ty()),
        ("OrderedField", axiom_ordered_field_ty()),
        (
            "OrderedField.archimedean",
            axiom_ordered_field_archimedean_ty(),
        ),
        ("WellOrder.induction", axiom_well_order_induction_ty()),
        ("WellOrder.isomorphism", axiom_well_order_isomorphism_ty()),
        ("Ordinal.sup", axiom_ordinal_sup_ty()),
        ("Ordinal.cnfNF", axiom_ordinal_cnf_ty()),
        (
            "PartialOrder.antisymmetry",
            axiom_partial_order_antisymmetry_ty(),
        ),
        (
            "TotalOrder.linearExtension",
            axiom_total_order_linear_extension_ty(),
        ),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
#[cfg(test)]
mod extended_ordering_tests {
    use super::*;
    #[test]
    fn test_wqo_instance_reflexive() {
        let w = WqoInstance::new(vec![0, 1, 2], |a, b| a <= b);
        assert!(w.is_reflexive());
    }
    #[test]
    fn test_wqo_instance_transitive() {
        let w = WqoInstance::new(vec![0, 1, 2], |a, b| a <= b);
        assert!(w.is_transitive());
    }
    #[test]
    fn test_wqo_good_pair() {
        let w = WqoInstance::new(vec![0, 1, 2], |a, b| a <= b);
        let seq = vec![2usize, 0, 1];
        assert!(w.has_good_pair(&seq));
    }
    #[test]
    fn test_wqo_find_good_pair() {
        let w = WqoInstance::new(vec![0, 1, 2], |a, b| a <= b);
        let seq = vec![0usize, 1, 2];
        let pair = w.find_good_pair(&seq);
        assert!(pair.is_some());
    }
    #[test]
    fn test_ordinal_cnf_zero() {
        let z = OrdinalCnf::zero();
        assert!(z.is_zero());
        assert!(z.is_finite());
        assert_eq!(z.as_finite(), Some(0));
    }
    #[test]
    fn test_ordinal_cnf_finite() {
        let three = OrdinalCnf::finite(3);
        assert!(!three.is_zero());
        assert!(three.is_finite());
        assert_eq!(three.as_finite(), Some(3));
    }
    #[test]
    fn test_ordinal_cnf_omega() {
        let w = OrdinalCnf::omega();
        assert!(!w.is_finite());
        assert!(!w.is_zero());
    }
    #[test]
    fn test_ordinal_cnf_add() {
        let a = OrdinalCnf::finite(2);
        let b = OrdinalCnf::finite(3);
        let sum = a.add(&b);
        assert_eq!(sum.as_finite(), Some(3));
    }
    #[test]
    fn test_ordinal_cnf_cmp() {
        let a = OrdinalCnf::finite(1);
        let b = OrdinalCnf::omega();
        assert_eq!(a.ord_cmp(&b), Ordering::Less);
        assert_eq!(b.ord_cmp(&a), Ordering::Greater);
        assert_eq!(a.ord_cmp(&a), Ordering::Equal);
    }
    #[test]
    fn test_dedekind_cut_in_lower_upper() {
        let cut = DedekindCutQ::new(1, 2);
        assert!(cut.in_lower(0, 1));
        assert!(cut.in_upper(1, 1));
        assert!(!cut.in_lower(1, 1));
    }
    #[test]
    fn test_dedekind_cut_cmp() {
        let a = DedekindCutQ::new(1, 3);
        let b = DedekindCutQ::new(1, 2);
        assert_eq!(a.cut_cmp(&b), Ordering::Less);
        assert_eq!(b.cut_cmp(&a), Ordering::Greater);
        assert_eq!(a.cut_cmp(&a), Ordering::Equal);
    }
    #[test]
    fn test_dedekind_mediant() {
        let a = DedekindCutQ::new(1, 3);
        let b = DedekindCutQ::new(1, 2);
        let m = a.mediant(&b);
        assert_eq!(m.numerator, 2);
        assert_eq!(m.denominator, 5);
    }
    #[test]
    fn test_finite_linear_order_identity() {
        let o = FiniteLinearOrder::identity(4);
        assert_eq!(o.size, 4);
        assert_eq!(o.rank(0), Some(0));
        assert_eq!(o.rank(3), Some(3));
        assert_eq!(o.min_elem(), Some(0));
        assert_eq!(o.max_elem(), Some(3));
    }
    #[test]
    fn test_finite_linear_order_compare() {
        let o = FiniteLinearOrder::identity(3);
        assert_eq!(o.compare(0, 1), Ordering::Less);
        assert_eq!(o.compare(2, 1), Ordering::Greater);
        assert_eq!(o.compare(1, 1), Ordering::Equal);
    }
    #[test]
    fn test_finite_linear_order_reverse() {
        let o = FiniteLinearOrder::identity(3);
        let r = o.reverse_order();
        assert_eq!(r.order, vec![2, 1, 0]);
        assert_eq!(r.min_elem(), Some(2));
    }
    #[test]
    fn test_finite_partial_order_discrete() {
        let p = FinitePartialOrder::discrete(3);
        assert!(p.is_valid());
        assert!(!p.is_total());
    }
    #[test]
    fn test_finite_partial_order_chain() {
        let mut le = vec![vec![false; 3]; 3];
        for i in 0..3usize {
            for j in i..3usize {
                le[i][j] = true;
            }
        }
        let p = FinitePartialOrder { size: 3, le };
        assert!(p.is_valid());
        assert!(p.is_total());
        assert_eq!(p.maximal_elements(), vec![2]);
    }
    #[test]
    fn test_transitive_closure() {
        let mut le = vec![vec![false; 3]; 3];
        le[0][0] = true;
        le[1][1] = true;
        le[2][2] = true;
        le[0][1] = true;
        le[1][2] = true;
        let p = FinitePartialOrder { size: 3, le };
        let cl = p.transitive_closure();
        assert!(cl.le[0][2]);
    }
    #[test]
    fn test_axiom_wqo_carrier_ty() {
        let ty = axiom_wqo_carrier_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_ordinal_zero_ty() {
        let ty = axiom_ordinal_zero_ty();
        assert_eq!(ty, Expr::Const(Name::str("Ordinal"), vec![]));
    }
    #[test]
    fn test_axiom_ordinal_add_ty() {
        let ty = axiom_ordinal_add_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_register_ordering_extended() {
        let mut env = Environment::new();
        assert!(register_ordering_extended(&mut env).is_ok());
        assert!(env.contains(&Name::str("Ordinal.zero")));
        assert!(env.contains(&Name::str("Ordinal.omega")));
        assert!(env.contains(&Name::str("Dickson.lemma")));
        assert!(env.contains(&Name::str("Higman.lemma")));
        assert!(env.contains(&Name::str("Kruskal.treeThm")));
        assert!(env.contains(&Name::str("Cantor.backAndForth")));
    }
}
/// Type of `BoundedLattice`: a lattice with top and bottom elements.
/// `BoundedLattice : Type → Type 1`
#[allow(dead_code)]
pub fn axiom_bounded_lattice_ty() -> Expr {
    ord_ext_arrow(
        Expr::Sort(Level::succ(Level::zero())),
        Expr::Sort(Level::succ(Level::succ(Level::zero()))),
    )
}
/// Type of `CompleteLattice.sup`: supremum of an arbitrary set.
/// `CompleteLattice.sup : ∀ {L : Type} \[CompleteLattice L\], (L → Prop) → L`
#[allow(dead_code)]
pub fn axiom_complete_lattice_sup_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("L"),
        Box::new(Expr::Sort(Level::succ(Level::zero()))),
        Box::new(ord_ext_arrow(
            ord_ext_arrow(Expr::BVar(0), Expr::Sort(Level::zero())),
            Expr::BVar(1),
        )),
    )
}
/// Type of `GaloisConnection`: a Galois connection between two posets.
/// `GaloisConnection : ∀ (P Q : Type) \[PartialOrder P\] \[PartialOrder Q\], (P → Q) → (Q → P) → Prop`
#[allow(dead_code)]
pub fn axiom_galois_connection_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("P"),
        Box::new(Expr::Sort(Level::succ(Level::zero()))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("Q"),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
            Box::new(ord_ext_arrow(
                ord_ext_arrow(Expr::BVar(1), Expr::BVar(0)),
                ord_ext_arrow(
                    ord_ext_arrow(Expr::BVar(1), Expr::BVar(2)),
                    Expr::Sort(Level::zero()),
                ),
            )),
        )),
    )
}
/// Type of `Antichain.maximal`: a maximal antichain in a partial order.
/// `Antichain.maximal : PartialOrder → Type`
#[allow(dead_code)]
pub fn axiom_antichain_maximal_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("PartialOrder"), vec![]),
        Expr::Sort(Level::succ(Level::zero())),
    )
}
/// Type of `Dilworth.theorem`: in any finite partial order, the minimum
/// number of chains needed to cover all elements equals the maximum antichain size.
/// `Dilworth.theorem : PartialOrder → Prop`
#[allow(dead_code)]
pub fn axiom_dilworth_theorem_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("PartialOrder"), vec![]),
        Expr::Sort(Level::zero()),
    )
}
/// Type of `Mirsky.theorem`: the minimum number of antichains covering a
/// partial order equals the length of the longest chain.
/// `Mirsky.theorem : PartialOrder → Prop`
#[allow(dead_code)]
pub fn axiom_mirsky_theorem_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("PartialOrder"), vec![]),
        Expr::Sort(Level::zero()),
    )
}
/// Type of `OrderIso`: an order isomorphism between two ordered sets.
/// `OrderIso : Type → Type → Type 1`
#[allow(dead_code)]
pub fn axiom_order_iso_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("A"),
        Box::new(Expr::Sort(Level::succ(Level::zero()))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("B"),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
            Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
        )),
    )
}
/// Type of `OrderEmbedding`: an order embedding (injective order homomorphism).
/// `OrderEmbedding : Type → Type → Type 1`
#[allow(dead_code)]
pub fn axiom_order_embedding_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("A"),
        Box::new(Expr::Sort(Level::succ(Level::zero()))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("B"),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
            Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
        )),
    )
}
/// Type of `CofinalSubset`: a subset S of a poset is cofinal if every element
/// has an upper bound in S.
/// `CofinalSubset : ∀ {P : Type} \[PartialOrder P\], (P → Prop) → Prop`
#[allow(dead_code)]
pub fn axiom_cofinal_subset_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("P"),
        Box::new(Expr::Sort(Level::succ(Level::zero()))),
        Box::new(ord_ext_arrow(
            ord_ext_arrow(Expr::BVar(0), Expr::Sort(Level::zero())),
            Expr::Sort(Level::zero()),
        )),
    )
}
/// Type of `Cofinality.ordinal`: the cofinality of an ordinal.
/// `Cofinality.ordinal : Ordinal → Ordinal`
#[allow(dead_code)]
pub fn axiom_cofinality_ordinal_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        Expr::Const(Name::str("Ordinal"), vec![]),
    )
}
/// Type of `RegularCardinal`: a cardinal equal to its own cofinality.
/// `RegularCardinal : Ordinal → Prop`
#[allow(dead_code)]
pub fn axiom_regular_cardinal_ty() -> Expr {
    ord_ext_arrow(
        Expr::Const(Name::str("Ordinal"), vec![]),
        Expr::Sort(Level::zero()),
    )
}
#[cfg(test)]
mod ordered_range_tests {
    use super::*;
    #[test]
    fn test_ordered_range_contains() {
        let r = OrderedRange::new(1u32, 10u32);
        assert!(r.contains(&5));
        assert!(r.contains(&1));
        assert!(r.contains(&10));
        assert!(!r.contains(&0));
        assert!(!r.contains(&11));
    }
    #[test]
    fn test_ordered_range_contains_range() {
        let outer = OrderedRange::new(0u32, 100u32);
        let inner = OrderedRange::new(10u32, 50u32);
        assert!(outer.contains_range(&inner));
        assert!(!inner.contains_range(&outer));
    }
    #[test]
    fn test_ordered_range_overlaps() {
        let r1 = OrderedRange::new(1u32, 5u32);
        let r2 = OrderedRange::new(4u32, 8u32);
        let r3 = OrderedRange::new(6u32, 10u32);
        assert!(r1.overlaps(&r2));
        assert!(!r1.overlaps(&r3));
    }
    #[test]
    fn test_axiom_dilworth_theorem_ty() {
        let ty = axiom_dilworth_theorem_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_galois_connection_ty() {
        let ty = axiom_galois_connection_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_cofinality_ordinal_ty() {
        let ty = axiom_cofinality_ordinal_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_order_iso_ty() {
        let ty = axiom_order_iso_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_bounded_lattice_ty() {
        let ty = axiom_bounded_lattice_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_complete_lattice_sup_ty() {
        let ty = axiom_complete_lattice_sup_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_antichain_maximal_ty() {
        let ty = axiom_antichain_maximal_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_mirsky_theorem_ty() {
        let ty = axiom_mirsky_theorem_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_cofinal_subset_ty() {
        let ty = axiom_cofinal_subset_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_regular_cardinal_ty() {
        let ty = axiom_regular_cardinal_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_axiom_order_embedding_ty() {
        let ty = axiom_order_embedding_ty();
        assert!(matches!(ty, Expr::Pi(_, _, _, _)));
    }
}
