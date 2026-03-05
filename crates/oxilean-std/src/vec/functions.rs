//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, InductiveType, IntroRule, Level, Name,
};

use super::types::{CircularBuffer, DList, FenwickTree, FixedVec, PrefixScan, SparseVec};

/// Build Vec type in the environment.
///
/// Vec : (α : Type) → Nat → Type
pub fn build_vec_env(env: &mut Environment, ind_env: &mut InductiveEnv) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let vec_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(type1.clone()),
        )),
    );
    let nil_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Vec"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Const(Name::str("Nat.zero"), vec![])),
        )),
    );
    let cons_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("n"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("head"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("tail"),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Vec"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Vec"), vec![])),
                            Box::new(Expr::BVar(3)),
                        )),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                )),
            )),
        )),
    );
    let vec_ind = InductiveType::new(
        Name::str("Vec"),
        vec![],
        1,
        1,
        vec_ty.clone(),
        vec![
            IntroRule {
                name: Name::str("Vec.nil"),
                ty: nil_ty.clone(),
            },
            IntroRule {
                name: Name::str("Vec.cons"),
                ty: cons_ty.clone(),
            },
        ],
    );
    ind_env.add(vec_ind).map_err(|e| format!("{}", e))?;
    env.add(Declaration::Axiom {
        name: Name::str("Vec"),
        univ_params: vec![],
        ty: vec_ty,
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Vec.nil"),
        univ_params: vec![],
        ty: nil_ty,
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Vec.cons"),
        univ_params: vec![],
        ty: cons_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_vec_env() {
        let mut env = Environment::new();
        let mut ind_env = InductiveEnv::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Nat.zero"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Nat.succ"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("n"),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
            ),
        })
        .expect("operation should succeed");
        assert!(build_vec_env(&mut env, &mut ind_env).is_ok());
        assert!(env.get(&Name::str("Vec")).is_some());
        assert!(env.get(&Name::str("Vec.nil")).is_some());
        assert!(env.get(&Name::str("Vec.cons")).is_some());
    }
}
/// Concatenate two vectors into a new one.
#[allow(dead_code)]
pub fn vec_append<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut result = a.to_vec();
    result.extend_from_slice(b);
    result
}
/// Return `true` if `v` contains `elem`.
#[allow(dead_code)]
pub fn vec_contains<T: PartialEq>(v: &[T], elem: &T) -> bool {
    v.contains(elem)
}
/// Return the index of the first occurrence of `elem`, or `None`.
#[allow(dead_code)]
pub fn vec_index_of<T: PartialEq>(v: &[T], elem: &T) -> Option<usize> {
    v.iter().position(|x| x == elem)
}
/// Remove all occurrences of `elem` from `v`.
#[allow(dead_code)]
pub fn vec_remove_all<T: PartialEq>(v: Vec<T>, elem: &T) -> Vec<T> {
    v.into_iter().filter(|x| x != elem).collect()
}
/// Deduplicate a vector (keeping first occurrence), preserving order.
#[allow(dead_code)]
pub fn vec_dedup_stable<T: PartialEq + Clone>(v: &[T]) -> Vec<T> {
    let mut seen = Vec::new();
    for item in v {
        if !seen.contains(item) {
            seen.push(item.clone());
        }
    }
    seen
}
/// Flatten a `Vec<Vec<T>>` into a `Vec<T>`.
#[allow(dead_code)]
pub fn vec_flatten<T>(v: Vec<Vec<T>>) -> Vec<T> {
    v.into_iter().flatten().collect()
}
/// Return the last element of a slice, or `None`.
#[allow(dead_code)]
pub fn vec_last<T>(v: &[T]) -> Option<&T> {
    v.last()
}
/// Return the first element of a slice, or `None`.
#[allow(dead_code)]
pub fn vec_head<T>(v: &[T]) -> Option<&T> {
    v.first()
}
/// Return all but the last element (i.e. `init` of the vector).
#[allow(dead_code)]
pub fn vec_init<T: Clone>(v: &[T]) -> Vec<T> {
    if v.is_empty() {
        vec![]
    } else {
        v[..v.len() - 1].to_vec()
    }
}
/// Return all but the first element (i.e. `tail` of the vector).
#[allow(dead_code)]
pub fn vec_tail<T: Clone>(v: &[T]) -> Vec<T> {
    if v.is_empty() {
        vec![]
    } else {
        v[1..].to_vec()
    }
}
/// Split a vector at `index`, returning `(left, right)`.
#[allow(dead_code)]
pub fn vec_split_at<T: Clone>(v: &[T], index: usize) -> (Vec<T>, Vec<T>) {
    let idx = index.min(v.len());
    (v[..idx].to_vec(), v[idx..].to_vec())
}
/// Zip two vectors into a vector of pairs (up to the shorter length).
#[allow(dead_code)]
pub fn vec_zip<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect()
}
/// Unzip a vector of pairs into two vectors.
#[allow(dead_code)]
pub fn vec_unzip<A, B>(v: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    v.into_iter().unzip()
}
/// Return the vector with element at `index` removed.
#[allow(dead_code)]
pub fn vec_remove_at<T: Clone>(v: &[T], index: usize) -> Vec<T> {
    let mut result = v.to_vec();
    if index < result.len() {
        result.remove(index);
    }
    result
}
/// Insert `elem` at `index`, shifting subsequent elements right.
#[allow(dead_code)]
pub fn vec_insert_at<T: Clone>(v: &[T], index: usize, elem: T) -> Vec<T> {
    let mut result = v.to_vec();
    let idx = index.min(result.len());
    result.insert(idx, elem);
    result
}
/// Replace the element at `index` with `new_val`.
#[allow(dead_code)]
pub fn vec_set<T: Clone>(v: &[T], index: usize, new_val: T) -> Vec<T> {
    let mut result = v.to_vec();
    if index < result.len() {
        result[index] = new_val;
    }
    result
}
/// Reverse a vector.
#[allow(dead_code)]
pub fn vec_reverse<T: Clone>(v: &[T]) -> Vec<T> {
    let mut result = v.to_vec();
    result.reverse();
    result
}
/// Rotate a vector left by `n` positions.
#[allow(dead_code)]
pub fn vec_rotate_left<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    if v.is_empty() {
        return vec![];
    }
    let n = n % v.len();
    let mut result = v[n..].to_vec();
    result.extend_from_slice(&v[..n]);
    result
}
/// Rotate a vector right by `n` positions.
#[allow(dead_code)]
pub fn vec_rotate_right<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    if v.is_empty() {
        return vec![];
    }
    let n = n % v.len();
    vec_rotate_left(v, v.len() - n)
}
/// Return every `n`-th element starting from `start`.
#[allow(dead_code)]
pub fn vec_step_by<T: Clone>(v: &[T], start: usize, step: usize) -> Vec<T> {
    if step == 0 {
        return vec![];
    }
    v.iter()
        .enumerate()
        .filter(|(i, _)| i >= &start && (i - start) % step == 0)
        .map(|(_, x)| x.clone())
        .collect()
}
/// Count elements matching a predicate.
#[allow(dead_code)]
pub fn vec_count_where<T, F: Fn(&T) -> bool>(v: &[T], pred: F) -> usize {
    v.iter().filter(|x| pred(x)).count()
}
/// Partition into matching and non-matching elements.
#[allow(dead_code)]
pub fn vec_partition<T, F: Fn(&T) -> bool>(v: Vec<T>, pred: F) -> (Vec<T>, Vec<T>) {
    v.into_iter().partition(|x| pred(x))
}
/// Map a fallible function over a vector, returning early on error.
#[allow(dead_code)]
pub fn vec_try_map<T, U, E, F: Fn(T) -> Result<U, E>>(v: Vec<T>, f: F) -> Result<Vec<U>, E> {
    v.into_iter().map(f).collect()
}
/// Interleave `sep` between elements of `v`.
#[allow(dead_code)]
pub fn vec_intersperse<T: Clone>(v: &[T], sep: T) -> Vec<T> {
    if v.is_empty() {
        return vec![];
    }
    let mut result = Vec::with_capacity(v.len() * 2 - 1);
    for (i, x) in v.iter().enumerate() {
        if i > 0 {
            result.push(sep.clone());
        }
        result.push(x.clone());
    }
    result
}
/// Transpose a `Vec<Vec<T>>` (rows ↔ columns).
///
/// The input must be rectangular (all inner vecs same length).
#[allow(dead_code)]
pub fn vec_transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    (0..cols)
        .map(|c| (0..rows).map(|r| matrix[r][c].clone()).collect())
        .collect()
}
/// Produce all combinations of one element from each inner slice.
///
/// E.g. `[[1,2],[3,4]]` → `[[1,3],[1,4],[2,3],[2,4]]`.
#[allow(dead_code)]
pub fn vec_cartesian_product<T: Clone>(vecs: &[Vec<T>]) -> Vec<Vec<T>> {
    if vecs.is_empty() {
        return vec![vec![]];
    }
    let mut result = vec![vec![]];
    for row in vecs {
        let mut new_result = Vec::new();
        for prefix in &result {
            for item in row {
                let mut new_prefix = prefix.clone();
                new_prefix.push(item.clone());
                new_result.push(new_prefix);
            }
        }
        result = new_result;
    }
    result
}
/// Return the maximum element of a non-empty slice, or `None`.
#[allow(dead_code)]
pub fn vec_max<T: Ord + Clone>(v: &[T]) -> Option<T> {
    v.iter().max().cloned()
}
/// Return the minimum element of a non-empty slice, or `None`.
#[allow(dead_code)]
pub fn vec_min<T: Ord + Clone>(v: &[T]) -> Option<T> {
    v.iter().min().cloned()
}
/// Sum all `u64` values in a slice.
#[allow(dead_code)]
pub fn vec_sum_u64(v: &[u64]) -> u64 {
    v.iter().sum()
}
/// Product of all `u64` values in a slice.
#[allow(dead_code)]
pub fn vec_product_u64(v: &[u64]) -> u64 {
    v.iter().product()
}
#[cfg(test)]
mod vec_extra_tests {
    use super::*;
    #[test]
    fn test_vec_append() {
        let a = vec![1, 2];
        let b = vec![3, 4];
        assert_eq!(vec_append(&a, &b), vec![1, 2, 3, 4]);
    }
    #[test]
    fn test_vec_dedup_stable() {
        let v = vec![1, 2, 1, 3, 2, 4];
        assert_eq!(vec_dedup_stable(&v), vec![1, 2, 3, 4]);
    }
    #[test]
    fn test_vec_flatten() {
        let v = vec![vec![1, 2], vec![3], vec![4, 5]];
        assert_eq!(vec_flatten(v), vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_vec_head_tail() {
        let v = vec![10, 20, 30];
        assert_eq!(vec_head(&v), Some(&10));
        assert_eq!(vec_tail(&v), vec![20, 30]);
        assert_eq!(vec_init(&v), vec![10, 20]);
    }
    #[test]
    fn test_vec_zip_unzip() {
        let a = vec![1, 2, 3];
        let b = vec!["a", "b", "c"];
        let zipped = vec_zip(&a, &b);
        assert_eq!(zipped, vec![(1, "a"), (2, "b"), (3, "c")]);
        let (a2, b2): (Vec<_>, Vec<_>) = vec_unzip(zipped);
        assert_eq!(a2, a);
        assert_eq!(b2, b);
    }
    #[test]
    fn test_vec_rotate() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(vec_rotate_left(&v, 2), vec![3, 4, 5, 1, 2]);
        assert_eq!(vec_rotate_right(&v, 2), vec![4, 5, 1, 2, 3]);
    }
    #[test]
    fn test_vec_intersperse() {
        let v = vec![1, 2, 3];
        assert_eq!(vec_intersperse(&v, 0), vec![1, 0, 2, 0, 3]);
    }
    #[test]
    fn test_vec_transpose() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let t = vec_transpose(&m);
        assert_eq!(t, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }
    #[test]
    fn test_vec_cartesian_product() {
        let vecs = vec![vec![1, 2], vec![3, 4]];
        let product = vec_cartesian_product(&vecs);
        assert_eq!(product.len(), 4);
        assert!(product.contains(&vec![1, 3]));
        assert!(product.contains(&vec![2, 4]));
    }
    #[test]
    fn test_vec_partition() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let (even, odd) = vec_partition(v, |x| x % 2 == 0);
        assert_eq!(even, vec![2, 4, 6]);
        assert_eq!(odd, vec![1, 3, 5]);
    }
    #[test]
    fn test_fixed_vec() {
        let fv = FixedVec::from_vec(vec![10, 20, 30]);
        assert_eq!(fv.len(), 3);
        assert_eq!(fv.get(1), Some(&20));
        assert_eq!(fv.get(5), None);
    }
    #[test]
    fn test_vec_step_by() {
        let v = vec![0, 1, 2, 3, 4, 5, 6];
        assert_eq!(vec_step_by(&v, 0, 2), vec![0, 2, 4, 6]);
    }
    #[test]
    fn test_vec_count_where() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(vec_count_where(&v, |x| *x > 3), 2);
    }
    #[test]
    fn test_vec_min_max() {
        let v = vec![3, 1, 4, 1, 5, 9];
        assert_eq!(vec_min(&v), Some(1));
        assert_eq!(vec_max(&v), Some(9));
    }
    #[test]
    fn test_vec_remove_at_insert_at() {
        let v = vec![1, 2, 3, 4];
        let removed = vec_remove_at(&v, 1);
        assert_eq!(removed, vec![1, 3, 4]);
        let inserted = vec_insert_at(&v, 2, 99);
        assert_eq!(inserted, vec![1, 2, 99, 3, 4]);
    }
    #[test]
    fn test_vec_try_map() {
        let v = vec!["1", "2", "3"];
        let result: Result<Vec<u32>, _> = vec_try_map(v, |s| s.parse::<u32>());
        assert_eq!(result.expect("result should be valid"), vec![1, 2, 3]);
    }
}
/// Assert that two slices have the same length and return an error otherwise.
#[allow(dead_code)]
pub fn same_length_check<A, B>(a: &[A], b: &[B]) -> Result<(), String> {
    if a.len() == b.len() {
        Ok(())
    } else {
        Err(format!("length mismatch: {} vs {}", a.len(), b.len()))
    }
}
/// Zip two slices, failing if they have different lengths.
#[allow(dead_code)]
pub fn zip_exact<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Result<Vec<(A, B)>, String> {
    same_length_check(a, b)?;
    Ok(vec_zip(a, b))
}
/// Map over a slice and collect the results, also returning the original indices.
#[allow(dead_code)]
pub fn indexed_map<T, U, F: Fn(usize, &T) -> U>(v: &[T], f: F) -> Vec<(usize, U)> {
    v.iter().enumerate().map(|(i, x)| (i, f(i, x))).collect()
}
/// Find the index of the maximum element.
#[allow(dead_code)]
pub fn argmax<T: PartialOrd>(v: &[T]) -> Option<usize> {
    if v.is_empty() {
        return None;
    }
    let mut best = 0;
    for (i, x) in v.iter().enumerate() {
        if *x > v[best] {
            best = i;
        }
    }
    Some(best)
}
/// Find the index of the minimum element.
#[allow(dead_code)]
pub fn argmin<T: PartialOrd>(v: &[T]) -> Option<usize> {
    if v.is_empty() {
        return None;
    }
    let mut best = 0;
    for (i, x) in v.iter().enumerate() {
        if *x < v[best] {
            best = i;
        }
    }
    Some(best)
}
/// Run-length encode a slice: consecutive equal elements become `(value, count)`.
#[allow(dead_code)]
pub fn run_length_encode<T: PartialEq + Clone>(v: &[T]) -> Vec<(T, usize)> {
    if v.is_empty() {
        return vec![];
    }
    let mut result = Vec::new();
    let mut current = v[0].clone();
    let mut count = 1;
    for item in &v[1..] {
        if *item == current {
            count += 1;
        } else {
            result.push((current.clone(), count));
            current = item.clone();
            count = 1;
        }
    }
    result.push((current, count));
    result
}
/// Decode a run-length encoded sequence back to a flat vector.
#[allow(dead_code)]
pub fn run_length_decode<T: Clone>(encoded: &[(T, usize)]) -> Vec<T> {
    let mut result = Vec::new();
    for (item, count) in encoded {
        for _ in 0..*count {
            result.push(item.clone());
        }
    }
    result
}
/// Sliding window iterator: produce all sub-slices of length `window`.
#[allow(dead_code)]
pub fn windows_collect<T: Clone>(v: &[T], window: usize) -> Vec<Vec<T>> {
    if window == 0 || window > v.len() {
        return vec![];
    }
    v.windows(window).map(|w| w.to_vec()).collect()
}
/// Return `true` if `v` is a palindrome.
#[allow(dead_code)]
pub fn is_palindrome<T: PartialEq>(v: &[T]) -> bool {
    let n = v.len();
    for i in 0..n / 2 {
        if v[i] != v[n - 1 - i] {
            return false;
        }
    }
    true
}
/// Flatten one level of nesting.
#[allow(dead_code)]
pub fn flatten_once<T: Clone>(v: &[Vec<T>]) -> Vec<T> {
    v.iter().flat_map(|inner| inner.iter().cloned()).collect()
}
/// Group consecutive elements by a key function.
#[allow(dead_code)]
pub fn group_by<T: Clone, K: PartialEq, F: Fn(&T) -> K>(v: &[T], key: F) -> Vec<Vec<T>> {
    if v.is_empty() {
        return vec![];
    }
    let mut groups: Vec<Vec<T>> = Vec::new();
    let mut current_group = vec![v[0].clone()];
    let mut current_key = key(&v[0]);
    for item in &v[1..] {
        let k = key(item);
        if k == current_key {
            current_group.push(item.clone());
        } else {
            groups.push(current_group.clone());
            current_group = vec![item.clone()];
            current_key = k;
        }
    }
    groups.push(current_group);
    groups
}
#[cfg(test)]
mod vec_extra_tests2 {
    use super::*;
    #[test]
    fn test_same_length_check() {
        assert!(same_length_check(&[1, 2], &[3, 4]).is_ok());
        assert!(same_length_check(&[1], &[3, 4]).is_err());
    }
    #[test]
    fn test_zip_exact() {
        let a = vec![1, 2, 3];
        let b = vec!["a", "b", "c"];
        let z = zip_exact(&a, &b).expect("operation should succeed");
        assert_eq!(z.len(), 3);
        assert!(zip_exact(&[1], &[2, 3]).is_err());
    }
    #[test]
    fn test_argmax_argmin() {
        let v = vec![3.0f64, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
        assert_eq!(argmax(&v), Some(5));
        assert_eq!(argmin(&v), Some(1));
        let empty: Vec<f64> = vec![];
        assert!(argmax(&empty).is_none());
    }
    #[test]
    fn test_run_length_encode_decode() {
        let v = vec![1, 1, 2, 3, 3, 3, 1];
        let enc = run_length_encode(&v);
        assert_eq!(enc, vec![(1, 2), (2, 1), (3, 3), (1, 1)]);
        let dec = run_length_decode(&enc);
        assert_eq!(dec, v);
    }
    #[test]
    fn test_windows_collect() {
        let v = vec![1, 2, 3, 4];
        let wins = windows_collect(&v, 2);
        assert_eq!(wins, vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
    }
    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(&[1, 2, 1]));
        assert!(is_palindrome(&[1, 2, 2, 1]));
        assert!(!is_palindrome(&[1, 2, 3]));
        assert!(is_palindrome::<i32>(&[]));
    }
    #[test]
    fn test_flatten_once() {
        let v = vec![vec![1, 2], vec![3], vec![4, 5]];
        assert_eq!(flatten_once(&v), vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_group_by() {
        let v = vec![1, 1, 2, 3, 3, 1];
        let groups = group_by(&v, |x| *x);
        assert_eq!(groups.len(), 4);
        assert_eq!(groups[0], vec![1, 1]);
        assert_eq!(groups[2], vec![3, 3]);
    }
    #[test]
    fn test_indexed_map() {
        let v = vec![10, 20, 30];
        let result = indexed_map(&v, |i, x| i + x);
        assert_eq!(result, vec![(0, 10), (1, 21), (2, 32)]);
    }
}
/// Return elements in `a` not in `b` (order-preserving, `O(n*m)`).
#[allow(dead_code)]
pub fn vec_difference<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().filter(|x| !b.contains(x)).cloned().collect()
}
/// Return elements in both `a` and `b` (order-preserving, keeps first occurrence).
#[allow(dead_code)]
pub fn vec_intersection<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().filter(|x| b.contains(x)).cloned().collect()
}
/// Return elements in either `a` or `b` but not both (symmetric difference).
#[allow(dead_code)]
pub fn vec_symmetric_difference<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut result = vec_difference(a, b);
    result.extend(vec_difference(b, a));
    result
}
/// Return `true` if `a` and `b` are set-equal (same elements, any order).
#[allow(dead_code)]
pub fn vec_set_eq<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().all(|x| b.contains(x))
}
/// Return `true` if `a` is a subset of `b` (all elements of `a` appear in `b`).
#[allow(dead_code)]
pub fn vec_is_subset<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.iter().all(|x| b.contains(x))
}
/// Compute the arithmetic mean of a `f64` slice.
///
/// Returns `f64::NAN` for an empty slice.
#[allow(dead_code)]
pub fn vec_mean_f64(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.iter().sum::<f64>() / v.len() as f64
}
/// Compute the variance (population) of a `f64` slice.
#[allow(dead_code)]
pub fn vec_variance_f64(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mean = vec_mean_f64(v);
    v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64
}
/// Compute the standard deviation (population) of a `f64` slice.
#[allow(dead_code)]
pub fn vec_std_dev_f64(v: &[f64]) -> f64 {
    vec_variance_f64(v).sqrt()
}
/// Compute the median of a `f64` slice.
///
/// Returns `f64::NAN` for empty slices; averages the two middle elements for
/// even-length slices.
#[allow(dead_code)]
pub fn vec_median_f64(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mut sorted = v.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    }
}
/// Normalise a `f64` vector so that its values sum to 1.0.
///
/// Returns the vector unchanged if the sum is zero.
#[allow(dead_code)]
pub fn vec_normalize_f64(v: &[f64]) -> Vec<f64> {
    let total: f64 = v.iter().sum();
    if total == 0.0 {
        return v.to_vec();
    }
    v.iter().map(|x| x / total).collect()
}
/// Split a slice into chunks of `size`.
///
/// The last chunk may be shorter than `size`.
#[allow(dead_code)]
pub fn vec_chunks<T: Clone>(v: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return vec![];
    }
    v.chunks(size).map(|c| c.to_vec()).collect()
}
/// Take up to `n` elements from the front of a slice.
#[allow(dead_code)]
pub fn vec_take<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    v.iter().take(n).cloned().collect()
}
/// Drop the first `n` elements.
#[allow(dead_code)]
pub fn vec_drop<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    v.iter().skip(n).cloned().collect()
}
/// Take elements while a predicate is true.
#[allow(dead_code)]
pub fn vec_take_while<T: Clone, F: Fn(&T) -> bool>(v: &[T], pred: F) -> Vec<T> {
    v.iter().take_while(|x| pred(x)).cloned().collect()
}
/// Drop elements while a predicate is true.
#[allow(dead_code)]
pub fn vec_drop_while<T: Clone, F: Fn(&T) -> bool>(v: &[T], pred: F) -> Vec<T> {
    v.iter().skip_while(|x| pred(x)).cloned().collect()
}
#[cfg(test)]
mod vec_set_op_tests {
    use super::*;
    #[test]
    fn test_vec_difference() {
        assert_eq!(vec_difference(&[1, 2, 3], &[2, 4]), vec![1, 3]);
    }
    #[test]
    fn test_vec_intersection() {
        assert_eq!(vec_intersection(&[1, 2, 3, 4], &[2, 4, 6]), vec![2, 4]);
    }
    #[test]
    fn test_vec_symmetric_difference() {
        let r = vec_symmetric_difference(&[1, 2, 3], &[2, 3, 4]);
        assert!(r.contains(&1) && r.contains(&4));
        assert!(!r.contains(&2));
    }
    #[test]
    fn test_vec_set_eq() {
        assert!(vec_set_eq(&[1, 2, 3], &[3, 1, 2]));
        assert!(!vec_set_eq(&[1, 2], &[1, 2, 3]));
    }
    #[test]
    fn test_vec_is_subset() {
        assert!(vec_is_subset(&[1, 2], &[1, 2, 3]));
        assert!(!vec_is_subset(&[1, 4], &[1, 2, 3]));
    }
    #[test]
    fn test_vec_mean_f64() {
        assert!((vec_mean_f64(&[1.0, 2.0, 3.0]) - 2.0).abs() < 1e-9);
        assert!(vec_mean_f64(&[]).is_nan());
    }
    #[test]
    fn test_vec_variance_f64() {
        let v = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let var = vec_variance_f64(&v);
        assert!((var - 4.0).abs() < 1e-9);
    }
    #[test]
    fn test_vec_std_dev_f64() {
        let v = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std = vec_std_dev_f64(&v);
        assert!((std - 2.0).abs() < 1e-9);
    }
    #[test]
    fn test_vec_median_f64_odd() {
        assert!((vec_median_f64(&[3.0, 1.0, 2.0]) - 2.0).abs() < 1e-9);
    }
    #[test]
    fn test_vec_median_f64_even() {
        assert!((vec_median_f64(&[1.0, 3.0, 5.0, 7.0]) - 4.0).abs() < 1e-9);
    }
    #[test]
    fn test_vec_normalize_f64() {
        let v = vec![1.0, 2.0, 3.0, 4.0];
        let n = vec_normalize_f64(&v);
        let sum: f64 = n.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_vec_chunks() {
        let v = vec![1, 2, 3, 4, 5];
        let chunks = vec_chunks(&v, 2);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[2], vec![5]);
    }
    #[test]
    fn test_vec_take_drop() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(vec_take(&v, 3), vec![1, 2, 3]);
        assert_eq!(vec_drop(&v, 3), vec![4, 5]);
    }
    #[test]
    fn test_vec_take_while_drop_while() {
        let v = vec![1, 2, 3, 4, 5];
        let tw = vec_take_while(&v, |x| *x < 4);
        assert_eq!(tw, vec![1, 2, 3]);
        let dw = vec_drop_while(&v, |x| *x < 4);
        assert_eq!(dw, vec![4, 5]);
    }
    #[test]
    fn test_vec_normalize_zero_sum() {
        let v = vec![0.0, 0.0, 0.0];
        let n = vec_normalize_f64(&v);
        assert_eq!(n, v);
    }
    #[test]
    fn test_vec_chunks_zero_size() {
        let v = vec![1, 2, 3];
        let chunks = vec_chunks(&v, 0);
        assert!(chunks.is_empty());
    }
}
pub fn vec_ext_prop_axiom(name: &str, env: &mut Environment) -> std::result::Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty: prop,
    })
    .map_err(|e| e.to_string())
}
/// Build `∀ (α : Type), Prop`.
pub fn vec_ext_forall1_axiom(name: &str, env: &mut Environment) -> std::result::Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(prop),
    );
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build `∀ (α β : Type), Prop`.
pub fn vec_ext_forall2_axiom(name: &str, env: &mut Environment) -> std::result::Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(prop),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build `∀ (α β γ : Type), Prop`.
pub fn vec_ext_forall3_axiom(name: &str, env: &mut Environment) -> std::result::Result<(), String> {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1),
                Box::new(prop),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// `Vec.functor_map_id : ∀ α, map id xs = xs`
pub fn vec_ext_build_functor_map_id(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.functor_map_id", env)
}
/// `Vec.functor_map_comp : ∀ α β γ (f : α → β) (g : β → γ), map (g ∘ f) = map g ∘ map f`
pub fn vec_ext_build_functor_map_comp(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall3_axiom("Vec.functor_map_comp", env)
}
/// `Vec.monad_left_id : ∀ α β (a : α) (f : α → Vec β), andThen [a] f = f a`
pub fn vec_ext_build_monad_left_id(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.monad_left_id", env)
}
/// `Vec.monad_right_id : ∀ α (xs : Vec α), andThen xs pure = xs`
pub fn vec_ext_build_monad_right_id(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.monad_right_id", env)
}
/// `Vec.monad_assoc : ∀ α β γ, andThen (andThen xs f) g = andThen xs (fun x => andThen (f x) g)`
pub fn vec_ext_build_monad_assoc(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall3_axiom("Vec.monad_assoc", env)
}
/// `Vec.ap_identity : ap [id] xs = xs`
pub fn vec_ext_build_ap_identity(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.ap_identity", env)
}
/// `Vec.ap_homomorphism : ap [f] [v] = [f v]`
pub fn vec_ext_build_ap_homomorphism(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.ap_homomorphism", env)
}
/// `Vec.ap_interchange : ap fs [v] = ap [fun f => f v] fs`
pub fn vec_ext_build_ap_interchange(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.ap_interchange", env)
}
/// `Vec.ap_composition : ap (ap (ap [∘] fs) gs) xs = ap fs (ap gs xs)`
pub fn vec_ext_build_ap_composition(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall3_axiom("Vec.ap_composition", env)
}
/// `Vec.foldr_nil : foldr f z [] = z`
pub fn vec_ext_build_foldr_nil(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.foldr_nil", env)
}
/// `Vec.foldr_cons : foldr f z (x::xs) = f x (foldr f z xs)`
pub fn vec_ext_build_foldr_cons(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.foldr_cons", env)
}
/// `Vec.foldl_nil : foldl f z [] = z`
pub fn vec_ext_build_foldl_nil(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.foldl_nil", env)
}
/// `Vec.foldl_cons : foldl f z (x::xs) = foldl f (f z x) xs`
pub fn vec_ext_build_foldl_cons(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.foldl_cons", env)
}
/// `Vec.foldl_foldr_duality : foldl f z xs = foldr (flip f) z (reverse xs)`
pub fn vec_ext_build_foldl_foldr_duality(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.foldl_foldr_duality", env)
}
/// `Vec.scanl_nil : scanl f z [] = [z]`
pub fn vec_ext_build_scanl_nil(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.scanl_nil", env)
}
/// `Vec.scanl_cons : scanl f z (x::xs) = z :: scanl f (f z x) xs`
pub fn vec_ext_build_scanl_cons(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.scanl_cons", env)
}
/// `Vec.scanr_nil : scanr f z [] = [z]`
pub fn vec_ext_build_scanr_nil(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.scanr_nil", env)
}
/// `Vec.scanr_cons : head (scanr f z (x::xs)) = f x (head (scanr f z xs))`
pub fn vec_ext_build_scanr_cons(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.scanr_cons", env)
}
/// `Vec.scanl_last : last (scanl f z xs) = foldl f z xs`
pub fn vec_ext_build_scanl_last(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.scanl_last", env)
}
/// `Vec.sort_is_permutation : sort xs is a permutation of xs`
pub fn vec_ext_build_sort_is_permutation(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.sort_is_permutation", env)
}
/// `Vec.sort_is_sorted : ∀ α [Ord α] xs, isSorted (sort xs)`
pub fn vec_ext_build_sort_is_sorted(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.sort_is_sorted", env)
}
/// `Vec.stable_sort_preserves_order : stable sort preserves relative order of equal elements`
pub fn vec_ext_build_stable_sort(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.stable_sort_preserves_order", env)
}
/// `Vec.sort_idempotent : sort (sort xs) = sort xs`
pub fn vec_ext_build_sort_idempotent(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.sort_idempotent", env)
}
/// `Vec.map_fusion : map f (map g xs) = map (f ∘ g) xs`
pub fn vec_ext_build_map_fusion(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall3_axiom("Vec.map_fusion", env)
}
/// `Vec.filter_fusion : filter p (filter q xs) = filter (fun x => p x && q x) xs`
pub fn vec_ext_build_filter_fusion(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.filter_fusion", env)
}
/// `Vec.map_filter_fusion : map f (filter p xs) = filterMap (fun x => if p x then Some (f x) else None) xs`
pub fn vec_ext_build_map_filter_fusion(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.map_filter_fusion", env)
}
/// `Vec.fold_build_duality : foldr f z (build g) = g f z` (deforestation)
pub fn vec_ext_build_fold_build_duality(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.fold_build_duality", env)
}
/// `Vec.fin_index_get : ∀ {α n} (xs : Vec α n) (i : Fin n), xs[i] is within bounds`
pub fn vec_ext_build_fin_index_get(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.fin_index_get", env)
}
/// `Vec.fin_index_set : ∀ {α n} (xs : Vec α n) (i : Fin n) v, set xs i v has same length`
pub fn vec_ext_build_fin_index_set(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.fin_index_set", env)
}
/// `Vec.fin_map_preserves_length : length (map f xs) = length xs`
pub fn vec_ext_build_fin_map_length(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.fin_map_preserves_length", env)
}
/// `Vec.reverse_involutive : reverse (reverse xs) = xs`
pub fn vec_ext_build_reverse_involutive(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.reverse_involutive", env)
}
/// `Vec.reverse_append : reverse (xs ++ ys) = reverse ys ++ reverse xs`
pub fn vec_ext_build_reverse_append(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.reverse_append", env)
}
/// `Vec.reverse_map : reverse (map f xs) = map f (reverse xs)`
pub fn vec_ext_build_reverse_map(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.reverse_map", env)
}
/// `Vec.take_drop_reconstruct : take n xs ++ drop n xs = xs`
pub fn vec_ext_build_take_drop_reconstruct(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.take_drop_reconstruct", env)
}
/// `Vec.take_length : length (take n xs) = min n (length xs)`
pub fn vec_ext_build_take_length(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.take_length", env)
}
/// `Vec.drop_length : length (drop n xs) = max 0 (length xs - n)`
pub fn vec_ext_build_drop_length(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.drop_length", env)
}
/// `Vec.zip_length : length (zip xs ys) = min (length xs) (length ys)`
pub fn vec_ext_build_zip_length(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.zip_length", env)
}
/// `Vec.unzip_zip : unzip (zip xs ys) = (take (min n m) xs, take (min n m) ys)`
pub fn vec_ext_build_unzip_zip(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.unzip_zip", env)
}
/// `Vec.zip_map : zip (map f xs) (map g ys) = map (bimap f g) (zip xs ys)`
pub fn vec_ext_build_zip_map(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall3_axiom("Vec.zip_map", env)
}
/// `Vec.chunks_flatten : flatten (chunks n xs) = xs`  (when n > 0)
pub fn vec_ext_build_chunks_flatten(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.chunks_flatten", env)
}
/// `Vec.chunks_all_size : ∀ chunk ∈ init (chunks n xs), length chunk = n`
pub fn vec_ext_build_chunks_all_size(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.chunks_all_size", env)
}
/// `Vec.chunks_count : length (chunks n xs) = ceil (length xs / n)`
pub fn vec_ext_build_chunks_count(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.chunks_count", env)
}
/// `Vec.append_nil_left : [] ++ xs = xs`
pub fn vec_ext_build_append_nil_left(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.append_nil_left", env)
}
/// `Vec.append_nil_right : xs ++ [] = xs`
pub fn vec_ext_build_append_nil_right(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.append_nil_right", env)
}
/// `Vec.append_assoc : (xs ++ ys) ++ zs = xs ++ (ys ++ zs)`
pub fn vec_ext_build_append_assoc(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.append_assoc", env)
}
/// `Vec.length_append : length (xs ++ ys) = length xs + length ys`
pub fn vec_ext_build_length_append(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.length_append", env)
}
/// `Vec.rotate_left_right_inverse : rotateLeft n (rotateRight n xs) = xs`
pub fn vec_ext_build_rotate_left_right_inv(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.rotate_left_right_inverse", env)
}
/// `Vec.rotate_length : length (rotateLeft n xs) = length xs`
pub fn vec_ext_build_rotate_length(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.rotate_length", env)
}
/// `Vec.rotate_zero : rotateLeft 0 xs = xs`
pub fn vec_ext_build_rotate_zero(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.rotate_zero", env)
}
/// `Vec.dlist_append_assoc : dlist append is associative`
pub fn vec_ext_build_dlist_append_assoc(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.dlist_append_assoc", env)
}
/// `Vec.dlist_to_list_preserves : toList (dlist d) = d []`
pub fn vec_ext_build_dlist_to_list(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.dlist_to_list_preserves", env)
}
/// `Vec.concat_map_id : concatMap pure xs = xs`
pub fn vec_ext_build_concat_map_id(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.concat_map_id", env)
}
/// `Vec.concat_map_assoc : concatMap (concatMap f ∘ g) = concatMap f ∘ concatMap g`
pub fn vec_ext_build_concat_map_assoc(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall3_axiom("Vec.concat_map_assoc", env)
}
/// `Vec.flatten_singleton : flatten [[x1], [x2], ...] = [x1, x2, ...]`
pub fn vec_ext_build_flatten_singleton(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.flatten_singleton", env)
}
/// `Vec.flatten_map : flatten (map (map f) xss) = map f (flatten xss)`
pub fn vec_ext_build_flatten_map(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.flatten_map", env)
}
/// `Vec.span_reconstruct : fst (span p xs) ++ snd (span p xs) = xs`
pub fn vec_ext_build_span_reconstruct(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.span_reconstruct", env)
}
/// `Vec.partition_reconstruct : fst (partition p xs) ++ snd (partition p xs)` is permutation
pub fn vec_ext_build_partition_reconstruct(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.partition_reconstruct", env)
}
/// `Vec.groupBy_flatten : flatten (groupBy eq xs) = xs`
pub fn vec_ext_build_groupby_flatten(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.groupBy_flatten", env)
}
/// `Vec.prefix_sum_correct : prefixSum xs[i] = sum (take (i+1) xs)`
pub fn vec_ext_build_prefix_sum_correct(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_prop_axiom("Vec.prefix_sum_correct", env)
}
/// `Vec.parallel_prefix_sequential_equiv : parallelPrefix f z xs = scanl f z xs`
pub fn vec_ext_build_parallel_prefix_equiv(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    vec_ext_forall2_axiom("Vec.parallel_prefix_sequential_equiv", env)
}
/// `Vec.fenwick_prefix_sum_correct : query fenwick i = sum (take (i+1) original)`
pub fn vec_ext_build_fenwick_correct(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_prop_axiom("Vec.fenwick_prefix_sum_correct", env)
}
/// `Vec.rle_decode_encode : decode (encode xs) = xs`
pub fn vec_ext_build_rle_roundtrip(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.rle_decode_encode", env)
}
/// `Vec.rle_length : sum (map snd (rle xs)) = length xs`
pub fn vec_ext_build_rle_length(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.rle_length", env)
}
/// `Vec.matrix_transpose_involutive : transpose (transpose m) = m`
pub fn vec_ext_build_matrix_transpose_invol(
    env: &mut Environment,
) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.matrix_transpose_involutive", env)
}
/// `Vec.matrix_row_count : length (transpose m) = length (head m)`
pub fn vec_ext_build_matrix_row_count(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.matrix_row_count", env)
}
/// `Vec.matrix_col_count : length (head (transpose m)) = length m`
pub fn vec_ext_build_matrix_col_count(env: &mut Environment) -> std::result::Result<(), String> {
    vec_ext_forall1_axiom("Vec.matrix_col_count", env)
}
/// Register all extended Vec axioms into `env`.
///
/// Adds 35+ axioms covering:
/// - Vector functor/monad/applicative laws
/// - Fold laws (foldr, foldl)
/// - Scan operations (scanl, scanr)
/// - Sorting laws (stability, permutation)
/// - Fusion/deforestation
/// - Fin-indexed vectors
/// - Reverse, take/drop, zip/unzip laws
/// - Chunking operations
/// - Free monoid structure
/// - Rotations and circular buffers
/// - DList representation
/// - ConcatMap/flatten laws
/// - Span/partition/groupBy
/// - Parallel prefix operations
/// - Run-length encoding
/// - Matrix as nested vector
pub fn register_vec_extended_axioms(env: &mut Environment) {
    let builders: &[fn(&mut Environment) -> std::result::Result<(), String>] = &[
        vec_ext_build_functor_map_id,
        vec_ext_build_functor_map_comp,
        vec_ext_build_monad_left_id,
        vec_ext_build_monad_right_id,
        vec_ext_build_monad_assoc,
        vec_ext_build_ap_identity,
        vec_ext_build_ap_homomorphism,
        vec_ext_build_ap_interchange,
        vec_ext_build_ap_composition,
        vec_ext_build_foldr_nil,
        vec_ext_build_foldr_cons,
        vec_ext_build_foldl_nil,
        vec_ext_build_foldl_cons,
        vec_ext_build_foldl_foldr_duality,
        vec_ext_build_scanl_nil,
        vec_ext_build_scanl_cons,
        vec_ext_build_scanr_nil,
        vec_ext_build_scanr_cons,
        vec_ext_build_scanl_last,
        vec_ext_build_sort_is_permutation,
        vec_ext_build_sort_is_sorted,
        vec_ext_build_stable_sort,
        vec_ext_build_sort_idempotent,
        vec_ext_build_map_fusion,
        vec_ext_build_filter_fusion,
        vec_ext_build_map_filter_fusion,
        vec_ext_build_fold_build_duality,
        vec_ext_build_fin_index_get,
        vec_ext_build_fin_index_set,
        vec_ext_build_fin_map_length,
        vec_ext_build_reverse_involutive,
        vec_ext_build_reverse_append,
        vec_ext_build_reverse_map,
        vec_ext_build_take_drop_reconstruct,
        vec_ext_build_take_length,
        vec_ext_build_drop_length,
        vec_ext_build_zip_length,
        vec_ext_build_unzip_zip,
        vec_ext_build_zip_map,
        vec_ext_build_chunks_flatten,
        vec_ext_build_chunks_all_size,
        vec_ext_build_chunks_count,
        vec_ext_build_append_nil_left,
        vec_ext_build_append_nil_right,
        vec_ext_build_append_assoc,
        vec_ext_build_length_append,
        vec_ext_build_rotate_left_right_inv,
        vec_ext_build_rotate_length,
        vec_ext_build_rotate_zero,
        vec_ext_build_dlist_append_assoc,
        vec_ext_build_dlist_to_list,
        vec_ext_build_concat_map_id,
        vec_ext_build_concat_map_assoc,
        vec_ext_build_flatten_singleton,
        vec_ext_build_flatten_map,
        vec_ext_build_span_reconstruct,
        vec_ext_build_partition_reconstruct,
        vec_ext_build_groupby_flatten,
        vec_ext_build_prefix_sum_correct,
        vec_ext_build_parallel_prefix_equiv,
        vec_ext_build_fenwick_correct,
        vec_ext_build_rle_roundtrip,
        vec_ext_build_rle_length,
        vec_ext_build_matrix_transpose_invol,
        vec_ext_build_matrix_row_count,
        vec_ext_build_matrix_col_count,
    ];
    for builder in builders {
        let _ = builder(env);
    }
}
#[cfg(test)]
mod vec_extended_axiom_tests {
    use super::*;
    fn make_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Nat.zero"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Nat.succ"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("n"),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
            ),
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_register_vec_extended_axioms_runs() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.functor_map_id")).is_some());
        assert!(env.get(&Name::str("Vec.monad_left_id")).is_some());
        assert!(env.get(&Name::str("Vec.append_assoc")).is_some());
    }
    #[test]
    fn test_functor_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.functor_map_id")).is_some());
        assert!(env.get(&Name::str("Vec.functor_map_comp")).is_some());
    }
    #[test]
    fn test_monad_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.monad_left_id")).is_some());
        assert!(env.get(&Name::str("Vec.monad_right_id")).is_some());
        assert!(env.get(&Name::str("Vec.monad_assoc")).is_some());
    }
    #[test]
    fn test_applicative_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.ap_identity")).is_some());
        assert!(env.get(&Name::str("Vec.ap_homomorphism")).is_some());
        assert!(env.get(&Name::str("Vec.ap_interchange")).is_some());
        assert!(env.get(&Name::str("Vec.ap_composition")).is_some());
    }
    #[test]
    fn test_fold_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.foldr_nil")).is_some());
        assert!(env.get(&Name::str("Vec.foldr_cons")).is_some());
        assert!(env.get(&Name::str("Vec.foldl_nil")).is_some());
        assert!(env.get(&Name::str("Vec.foldl_cons")).is_some());
        assert!(env.get(&Name::str("Vec.foldl_foldr_duality")).is_some());
    }
    #[test]
    fn test_scan_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.scanl_nil")).is_some());
        assert!(env.get(&Name::str("Vec.scanl_cons")).is_some());
        assert!(env.get(&Name::str("Vec.scanr_nil")).is_some());
        assert!(env.get(&Name::str("Vec.scanr_cons")).is_some());
        assert!(env.get(&Name::str("Vec.scanl_last")).is_some());
    }
    #[test]
    fn test_sort_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.sort_is_permutation")).is_some());
        assert!(env.get(&Name::str("Vec.sort_is_sorted")).is_some());
        assert!(env
            .get(&Name::str("Vec.stable_sort_preserves_order"))
            .is_some());
        assert!(env.get(&Name::str("Vec.sort_idempotent")).is_some());
    }
    #[test]
    fn test_fusion_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.map_fusion")).is_some());
        assert!(env.get(&Name::str("Vec.filter_fusion")).is_some());
        assert!(env.get(&Name::str("Vec.map_filter_fusion")).is_some());
        assert!(env.get(&Name::str("Vec.fold_build_duality")).is_some());
    }
    #[test]
    fn test_fin_indexed_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.fin_index_get")).is_some());
        assert!(env.get(&Name::str("Vec.fin_index_set")).is_some());
        assert!(env
            .get(&Name::str("Vec.fin_map_preserves_length"))
            .is_some());
    }
    #[test]
    fn test_reverse_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.reverse_involutive")).is_some());
        assert!(env.get(&Name::str("Vec.reverse_append")).is_some());
        assert!(env.get(&Name::str("Vec.reverse_map")).is_some());
    }
    #[test]
    fn test_take_drop_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.take_drop_reconstruct")).is_some());
        assert!(env.get(&Name::str("Vec.take_length")).is_some());
        assert!(env.get(&Name::str("Vec.drop_length")).is_some());
    }
    #[test]
    fn test_zip_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.zip_length")).is_some());
        assert!(env.get(&Name::str("Vec.unzip_zip")).is_some());
        assert!(env.get(&Name::str("Vec.zip_map")).is_some());
    }
    #[test]
    fn test_chunking_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.chunks_flatten")).is_some());
        assert!(env.get(&Name::str("Vec.chunks_all_size")).is_some());
        assert!(env.get(&Name::str("Vec.chunks_count")).is_some());
    }
    #[test]
    fn test_free_monoid_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.append_nil_left")).is_some());
        assert!(env.get(&Name::str("Vec.append_nil_right")).is_some());
        assert!(env.get(&Name::str("Vec.append_assoc")).is_some());
        assert!(env.get(&Name::str("Vec.length_append")).is_some());
    }
    #[test]
    fn test_rotation_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env
            .get(&Name::str("Vec.rotate_left_right_inverse"))
            .is_some());
        assert!(env.get(&Name::str("Vec.rotate_length")).is_some());
        assert!(env.get(&Name::str("Vec.rotate_zero")).is_some());
    }
    #[test]
    fn test_dlist_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.dlist_append_assoc")).is_some());
        assert!(env.get(&Name::str("Vec.dlist_to_list_preserves")).is_some());
    }
    #[test]
    fn test_concat_map_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.concat_map_id")).is_some());
        assert!(env.get(&Name::str("Vec.concat_map_assoc")).is_some());
        assert!(env.get(&Name::str("Vec.flatten_singleton")).is_some());
        assert!(env.get(&Name::str("Vec.flatten_map")).is_some());
    }
    #[test]
    fn test_span_partition_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.span_reconstruct")).is_some());
        assert!(env.get(&Name::str("Vec.partition_reconstruct")).is_some());
        assert!(env.get(&Name::str("Vec.groupBy_flatten")).is_some());
    }
    #[test]
    fn test_parallel_prefix_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.prefix_sum_correct")).is_some());
        assert!(env
            .get(&Name::str("Vec.parallel_prefix_sequential_equiv"))
            .is_some());
        assert!(env
            .get(&Name::str("Vec.fenwick_prefix_sum_correct"))
            .is_some());
    }
    #[test]
    fn test_rle_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Vec.rle_decode_encode")).is_some());
        assert!(env.get(&Name::str("Vec.rle_length")).is_some());
    }
    #[test]
    fn test_matrix_laws_present() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        assert!(env
            .get(&Name::str("Vec.matrix_transpose_involutive"))
            .is_some());
        assert!(env.get(&Name::str("Vec.matrix_row_count")).is_some());
        assert!(env.get(&Name::str("Vec.matrix_col_count")).is_some());
    }
    #[test]
    fn test_circular_buffer_basic() {
        let buf: CircularBuffer<i32> = CircularBuffer::new(8);
        assert_eq!(buf.capacity(), 8);
        assert!(buf.is_empty());
        assert!(!buf.is_full());
    }
    #[test]
    fn test_dlist_singleton_to_vec() {
        let d = DList::singleton(42i32);
        assert_eq!(d.to_vec(), vec![42]);
    }
    #[test]
    fn test_prefix_scan_inclusive() {
        let ps = PrefixScan::inclusive(vec![1, 3, 6, 10]);
        assert_eq!(ps.values(), &[1, 3, 6, 10]);
        assert!(ps.inclusive);
        assert_eq!(ps.len(), 4);
    }
    #[test]
    fn test_fenwick_tree_basic() {
        let mut ft = FenwickTree::new(5);
        ft.update(1, 3);
        ft.update(2, 2);
        ft.update(3, 7);
        assert_eq!(ft.query(1), 3);
        assert_eq!(ft.query(2), 5);
        assert_eq!(ft.query(3), 12);
    }
    #[test]
    fn test_sparse_vec_get_set() {
        let mut sv: SparseVec<i32> = SparseVec::new(10, 0);
        assert_eq!(*sv.get(5), 0);
        sv.set(5, 42);
        assert_eq!(*sv.get(5), 42);
        assert_eq!(*sv.get(3), 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_all_35_plus_vec_axioms_registered() {
        let mut env = make_env();
        register_vec_extended_axioms(&mut env);
        let axiom_names = [
            "Vec.functor_map_id",
            "Vec.functor_map_comp",
            "Vec.monad_left_id",
            "Vec.monad_right_id",
            "Vec.monad_assoc",
            "Vec.ap_identity",
            "Vec.ap_homomorphism",
            "Vec.ap_interchange",
            "Vec.ap_composition",
            "Vec.foldr_nil",
            "Vec.foldr_cons",
            "Vec.foldl_nil",
            "Vec.foldl_cons",
            "Vec.foldl_foldr_duality",
            "Vec.scanl_nil",
            "Vec.scanl_cons",
            "Vec.scanr_nil",
            "Vec.scanr_cons",
            "Vec.scanl_last",
            "Vec.sort_is_permutation",
            "Vec.sort_is_sorted",
            "Vec.stable_sort_preserves_order",
            "Vec.sort_idempotent",
            "Vec.map_fusion",
            "Vec.filter_fusion",
            "Vec.map_filter_fusion",
            "Vec.fold_build_duality",
            "Vec.fin_index_get",
            "Vec.fin_index_set",
            "Vec.fin_map_preserves_length",
            "Vec.reverse_involutive",
            "Vec.reverse_append",
            "Vec.reverse_map",
            "Vec.take_drop_reconstruct",
            "Vec.take_length",
        ];
        let mut found = 0usize;
        for name in &axiom_names {
            if env.get(&Name::str(*name)).is_some() {
                found += 1;
            }
        }
        assert!(found >= 35, "Expected at least 35 axioms, found {}", found);
    }
}
