//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::hash::Hash;

use super::oxihashset_type::OxiHashSet;
use super::types::{Bijection, MultiSet, UnionFind};

/// A set of `String` values optimized for name lookups.
///
/// Used internally for tracking declared names, reserved identifiers, etc.
pub type NameSet = OxiHashSet<String>;
impl NameSet {
    /// Insert a `&str` directly.
    pub fn insert_str(&mut self, s: &str) -> bool {
        self.insert(s.to_string())
    }
    /// Check whether the set contains the given `&str`.
    pub fn contains_str(&self, s: &str) -> bool {
        self.contains(&s.to_string())
    }
    /// Remove by `&str`.
    pub fn remove_str(&mut self, s: &str) -> bool {
        self.remove(&s.to_string())
    }
}
/// A set of `u64` values, useful for tracking metavariable IDs.
pub type U64Set = OxiHashSet<u64>;
impl U64Set {
    /// Return the minimum element if non-empty.
    pub fn min_elem(&self) -> Option<u64> {
        self.inner.iter().copied().min()
    }
    /// Return the maximum element if non-empty.
    pub fn max_elem(&self) -> Option<u64> {
        self.inner.iter().copied().max()
    }
    /// Return the sum of all elements.
    pub fn sum(&self) -> u64 {
        self.inner.iter().copied().sum()
    }
    /// Check whether the range `lo..=hi` is completely covered.
    pub fn covers_range(&self, lo: u64, hi: u64) -> bool {
        (lo..=hi).all(|i| self.inner.contains(&i))
    }
}
/// A set of `usize` values.
pub type UsizeSet = OxiHashSet<usize>;
impl UsizeSet {
    /// Check whether the indices `0..n` are all present.
    pub fn covers_indices(&self, n: usize) -> bool {
        (0..n).all(|i| self.contains(&i))
    }
    /// Return elements as a sorted vector.
    pub fn sorted(&self) -> Vec<usize> {
        let mut v = self.to_vec();
        v.sort_unstable();
        v
    }
}
/// Compute the union of a collection of sets.
pub fn big_union<T: Eq + Hash + Clone, I: IntoIterator<Item = OxiHashSet<T>>>(
    sets: I,
) -> OxiHashSet<T> {
    sets.into_iter()
        .fold(OxiHashSet::new(), |acc, s| acc.union(&s))
}
/// Compute the intersection of a collection of sets (empty input → empty set).
pub fn big_intersection<T: Eq + Hash + Clone, I: IntoIterator<Item = OxiHashSet<T>>>(
    sets: I,
) -> OxiHashSet<T> {
    let mut iter = sets.into_iter();
    match iter.next() {
        None => OxiHashSet::new(),
        Some(first) => iter.fold(first, |acc, s| acc.intersection(&s)),
    }
}
/// Return `true` if all sets in the collection are pairwise disjoint.
pub fn pairwise_disjoint<T: Eq + Hash + Clone>(sets: &[OxiHashSet<T>]) -> bool {
    for i in 0..sets.len() {
        for j in (i + 1)..sets.len() {
            if !sets[i].is_disjoint(&sets[j]) {
                return false;
            }
        }
    }
    true
}
/// Compute the cartesian product of two sets as a set of pairs.
pub fn cartesian_product<A, B>(a: &OxiHashSet<A>, b: &OxiHashSet<B>) -> OxiHashSet<(A, B)>
where
    A: Eq + Hash + Clone,
    B: Eq + Hash + Clone,
{
    let mut result = OxiHashSet::new();
    for x in a.iter() {
        for y in b.iter() {
            result.insert((x.clone(), y.clone()));
        }
    }
    result
}
/// Build a set from a range `lo..hi`.
pub fn range_set(lo: u64, hi: u64) -> U64Set {
    U64Set::from_iter(lo..hi)
}
/// Check whether two sets have exactly the same elements (alias for `==`).
pub fn set_equal<T: Eq + Hash + Clone>(a: &OxiHashSet<T>, b: &OxiHashSet<T>) -> bool {
    a == b
}
/// Return the number of elements present in exactly one of the two sets.
pub fn symmetric_diff_size<T: Eq + Hash + Clone>(a: &OxiHashSet<T>, b: &OxiHashSet<T>) -> usize {
    a.symmetric_difference(b).len()
}
/// Convert a `Vec<T>` to a `OxiHashSet<T>`, discarding duplicates.
pub fn deduplicate<T: Eq + Hash + Clone>(v: Vec<T>) -> OxiHashSet<T> {
    OxiHashSet::from_iter(v)
}
/// Return elements that appear in `a` but not in any set in `others`.
pub fn difference_all<'a, T: Eq + Hash + Clone + 'a>(
    a: &OxiHashSet<T>,
    others: impl Iterator<Item = &'a OxiHashSet<T>>,
) -> OxiHashSet<T> {
    others.fold(a.clone(), |acc, s| acc.difference(s))
}
#[cfg(test)]
mod tests {
    use super::*;
    fn mk_set(v: Vec<i32>) -> OxiHashSet<i32> {
        OxiHashSet::from_iter(v)
    }
    #[test]
    fn test_insert_contains_len() {
        let mut s = OxiHashSet::new();
        assert!(s.insert(1));
        assert!(!s.insert(1));
        assert!(s.contains(&1));
        assert_eq!(s.len(), 1);
    }
    #[test]
    fn test_remove() {
        let mut s = mk_set(vec![1, 2, 3]);
        assert!(s.remove(&2));
        assert!(!s.contains(&2));
        assert_eq!(s.len(), 2);
    }
    #[test]
    fn test_union() {
        let a = mk_set(vec![1, 2]);
        let b = mk_set(vec![2, 3]);
        let u = a.union(&b);
        assert_eq!(u.len(), 3);
        assert!(u.contains(&1) && u.contains(&2) && u.contains(&3));
    }
    #[test]
    fn test_intersection() {
        let a = mk_set(vec![1, 2, 3]);
        let b = mk_set(vec![2, 3, 4]);
        let i = a.intersection(&b);
        assert_eq!(i.len(), 2);
        assert!(i.contains(&2) && i.contains(&3));
    }
    #[test]
    fn test_difference() {
        let a = mk_set(vec![1, 2, 3]);
        let b = mk_set(vec![2]);
        let d = a.difference(&b);
        assert!(d.contains(&1) && d.contains(&3));
        assert!(!d.contains(&2));
    }
    #[test]
    fn test_is_subset_superset() {
        let a = mk_set(vec![1, 2]);
        let b = mk_set(vec![1, 2, 3]);
        assert!(a.is_subset(&b));
        assert!(b.is_superset(&a));
        assert!(!b.is_subset(&a));
    }
    #[test]
    fn test_is_disjoint() {
        let a = mk_set(vec![1, 2]);
        let b = mk_set(vec![3, 4]);
        assert!(a.is_disjoint(&b));
        let c = mk_set(vec![2, 5]);
        assert!(!a.is_disjoint(&c));
    }
    #[test]
    fn test_map() {
        let a = mk_set(vec![1, 2, 3]);
        let doubled: OxiHashSet<i32> = a.map(|x| x * 2);
        assert!(doubled.contains(&2) && doubled.contains(&4) && doubled.contains(&6));
    }
    #[test]
    fn test_retain_clone() {
        let a = mk_set(vec![1, 2, 3, 4]);
        let evens = a.retain_clone(|x| x % 2 == 0);
        assert_eq!(evens.len(), 2);
        assert!(evens.contains(&2) && evens.contains(&4));
    }
    #[test]
    fn test_fold() {
        let a = mk_set(vec![1, 2, 3, 4]);
        let sum = a.fold(0, |acc, x| acc + x);
        assert_eq!(sum, 10);
    }
    #[test]
    fn test_any_all() {
        let a = mk_set(vec![2, 4, 6]);
        assert!(a.all(|x| x % 2 == 0));
        assert!(a.any(|x| *x == 4));
        assert!(!a.any(|x| *x == 3));
    }
    #[test]
    fn test_partition() {
        let a = mk_set(vec![1, 2, 3, 4, 5]);
        let (evens, odds) = a.partition(|x| x % 2 == 0);
        assert_eq!(evens.len(), 2);
        assert_eq!(odds.len(), 3);
    }
    #[test]
    fn test_singleton() {
        let s = OxiHashSet::singleton(42i32);
        assert_eq!(s.len(), 1);
        assert!(s.contains(&42));
    }
    #[test]
    fn test_name_set() {
        let mut ns = NameSet::new();
        ns.insert_str("foo");
        ns.insert_str("bar");
        assert!(ns.contains_str("foo"));
        ns.remove_str("foo");
        assert!(!ns.contains_str("foo"));
    }
    #[test]
    fn test_u64_set_min_max() {
        let s = U64Set::from_iter(vec![10u64, 3, 7]);
        assert_eq!(s.min_elem(), Some(3));
        assert_eq!(s.max_elem(), Some(10));
        assert_eq!(s.sum(), 20);
    }
    #[test]
    fn test_big_union() {
        let sets = vec![mk_set(vec![1, 2]), mk_set(vec![3]), mk_set(vec![2, 4])];
        let u = big_union(sets);
        assert_eq!(u.len(), 4);
    }
    #[test]
    fn test_big_intersection() {
        let sets = vec![
            mk_set(vec![1, 2, 3]),
            mk_set(vec![2, 3, 4]),
            mk_set(vec![3, 5]),
        ];
        let i = big_intersection(sets);
        assert_eq!(i.len(), 1);
        assert!(i.contains(&3));
    }
    #[test]
    fn test_pairwise_disjoint() {
        let sets = vec![mk_set(vec![1]), mk_set(vec![2]), mk_set(vec![3])];
        assert!(pairwise_disjoint(&sets));
        let overlapping = vec![mk_set(vec![1, 2]), mk_set(vec![2, 3])];
        assert!(!pairwise_disjoint(&overlapping));
    }
    #[test]
    fn test_cartesian_product() {
        let a = mk_set(vec![1, 2]);
        let b = mk_set(vec![10, 20]);
        let prod = cartesian_product(&a, &b);
        assert_eq!(prod.len(), 4);
    }
    #[test]
    fn test_range_set() {
        let r = range_set(0, 5);
        assert_eq!(r.len(), 5);
        assert!(r.covers_range(0, 4));
    }
    #[test]
    fn test_jaccard() {
        let a = mk_set(vec![1, 2, 3]);
        let b = mk_set(vec![2, 3, 4]);
        let j = a.jaccard(&b);
        assert!((j - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_symmetric_difference() {
        let a = mk_set(vec![1, 2, 3]);
        let b = mk_set(vec![3, 4, 5]);
        let sd = a.symmetric_difference(&b);
        assert_eq!(sd.len(), 4);
        assert!(sd.contains(&1) && sd.contains(&2) && sd.contains(&4) && sd.contains(&5));
        assert!(!sd.contains(&3));
    }
    #[test]
    fn test_flat_map() {
        let a = mk_set(vec![1, 2]);
        let result: OxiHashSet<i32> = a.flat_map(|x| OxiHashSet::from_iter(vec![*x, x * 10]));
        assert!(result.contains(&1) && result.contains(&10));
        assert!(result.contains(&2) && result.contains(&20));
    }
    #[test]
    fn test_power_set() {
        let s = mk_set(vec![1, 2]);
        let ps = s.power_set();
        assert_eq!(ps.len(), 4);
    }
    #[test]
    fn test_usize_set_sorted() {
        let s = UsizeSet::from_iter(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let sorted = s.sorted();
        let expected: Vec<usize> = (1..=9)
            .filter(|&x| [1, 2, 3, 4, 5, 6, 9].contains(&x))
            .collect();
        assert_eq!(sorted, expected);
    }
    #[test]
    fn test_extend() {
        let mut s = mk_set(vec![1]);
        s.extend(vec![2, 3, 4]);
        assert_eq!(s.len(), 4);
    }
    #[test]
    fn test_clear() {
        let mut s = mk_set(vec![1, 2, 3]);
        s.clear();
        assert!(s.is_empty());
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_multiset_insert_count() {
        let mut ms = MultiSet::new();
        ms.insert(1i32);
        ms.insert(1i32);
        ms.insert(2i32);
        assert_eq!(ms.count(&1), 2);
        assert_eq!(ms.count(&2), 1);
        assert_eq!(ms.total(), 3);
        assert_eq!(ms.distinct_count(), 2);
    }
    #[test]
    fn test_multiset_remove_one() {
        let mut ms = MultiSet::from(vec![1i32, 1, 2]);
        ms.remove_one(&1);
        assert_eq!(ms.count(&1), 1);
        ms.remove_one(&1);
        assert_eq!(ms.count(&1), 0);
        assert!(!ms.contains(&1));
    }
    #[test]
    fn test_multiset_to_set() {
        let ms = MultiSet::from(vec![1i32, 1, 2, 3]);
        let s = ms.to_set();
        assert_eq!(s.len(), 3);
    }
    #[test]
    fn test_bijection_insert_forward_backward() {
        let mut bij: Bijection<i32, &str> = Bijection::new();
        assert!(bij.insert(1, "one"));
        assert!(bij.insert(2, "two"));
        assert_eq!(bij.forward(&1), Some(&"one"));
        assert_eq!(bij.backward(&"one"), Some(&1));
        assert_eq!(bij.len(), 2);
    }
    #[test]
    fn test_bijection_no_duplicate() {
        let mut bij: Bijection<i32, i32> = Bijection::new();
        bij.insert(1, 10);
        assert!(!bij.insert(1, 20));
        assert!(!bij.insert(2, 10));
    }
    #[test]
    fn test_union_find_basic() {
        let mut uf = UnionFind::new(5);
        assert!(!uf.same(0, 1));
        uf.union(0, 1);
        assert!(uf.same(0, 1));
        uf.union(1, 2);
        assert!(uf.same(0, 2));
        assert!(!uf.same(0, 3));
    }
    #[test]
    fn test_union_find_component_count() {
        let mut uf = UnionFind::new(6);
        assert_eq!(uf.component_count(), 6);
        uf.union(0, 1);
        uf.union(2, 3);
        assert_eq!(uf.component_count(), 4);
    }
    #[test]
    fn test_union_find_component_of() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(1, 2);
        let comp = uf.component_of(0);
        assert_eq!(comp.len(), 3);
        assert!(comp.contains(&0) && comp.contains(&1) && comp.contains(&2));
    }
    #[test]
    fn test_sorted_vec() {
        let s = OxiHashSet::from_iter(vec![3i32, 1, 4, 1, 5, 9, 2, 6]);
        let sorted = s.sorted_vec();
        let expected = vec![1, 2, 3, 4, 5, 6, 9];
        assert_eq!(sorted, expected);
    }
    #[test]
    fn test_min_max_ord() {
        let s = OxiHashSet::from_iter(vec![3i32, 1, 4, 2]);
        assert_eq!(s.min(), Some(&1));
        assert_eq!(s.max(), Some(&4));
    }
    #[test]
    fn test_difference_all() {
        let a = OxiHashSet::from_iter(vec![1i32, 2, 3, 4, 5]);
        let others: Vec<OxiHashSet<i32>> = vec![
            OxiHashSet::from_iter(vec![1]),
            OxiHashSet::from_iter(vec![3]),
        ];
        let result = difference_all(&a, others.iter());
        assert_eq!(result.len(), 3);
        assert!(result.contains(&2) && result.contains(&4) && result.contains(&5));
    }
}
/// Build the standard HashSet environment declarations.
///
/// Registers the `HashSet` type constructor and its core operations as axioms
/// in the OxiLean kernel environment.
pub fn build_hashset_env(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let mut add = |name: &str, ty: Expr| -> Result<(), String> {
        match env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        }) {
            Ok(()) | Err(_) => Ok(()),
        }
    };
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let nat_ty = || -> Expr { cst("Nat") };
    let bool_ty = || -> Expr { cst("Bool") };
    let list_of = |ty: Expr| -> Expr { app(cst("List"), ty) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let option_of = |ty: Expr| -> Expr { app(cst("Option"), ty) };
    add("HashSet", arr(type1(), type1()))?;
    let empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(hashset_of(Expr::BVar(0))),
    );
    add("HashSet.empty", empty_ty)?;
    let insert_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
        )),
    );
    add("HashSet.insert", insert_ty)?;
    let contains_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(Expr::BVar(1), bool_ty()),
        )),
    );
    add("HashSet.contains", contains_ty)?;
    let erase_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(Expr::BVar(1), hashset_of(Expr::BVar(2))),
        )),
    );
    add("HashSet.erase", erase_ty)?;
    let size_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(hashset_of(Expr::BVar(0)), nat_ty())),
    );
    add("HashSet.size", size_ty)?;
    let is_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(hashset_of(Expr::BVar(0)), bool_ty())),
    );
    add("HashSet.isEmpty", is_empty_ty)?;
    let to_list_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(hashset_of(Expr::BVar(0)), list_of(Expr::BVar(1)))),
    );
    add("HashSet.toList", to_list_ty)?;
    let of_list_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(list_of(Expr::BVar(0)), hashset_of(Expr::BVar(1)))),
    );
    add("HashSet.ofList", of_list_ty)?;
    let union_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
        )),
    );
    add("HashSet.union", union_ty)?;
    let inter_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
        )),
    );
    add("HashSet.inter", inter_ty)?;
    let sdiff_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
        )),
    );
    add("HashSet.sdiff", sdiff_ty)?;
    let subset_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(hashset_of(Expr::BVar(1)), bool_ty()),
        )),
    );
    add("HashSet.subset", subset_ty)?;
    let filter_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), bool_ty()),
            arr(hashset_of(Expr::BVar(1)), hashset_of(Expr::BVar(2))),
        )),
    );
    add("HashSet.filter", filter_ty)?;
    let fold_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("β"),
            Box::new(type1()),
            Box::new(arr(
                arr(Expr::BVar(0), arr(Expr::BVar(1), Expr::BVar(2))),
                arr(Expr::BVar(3), arr(hashset_of(Expr::BVar(4)), Expr::BVar(5))),
            )),
        )),
    );
    add("HashSet.fold", fold_ty)?;
    let any_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), bool_ty()),
            arr(hashset_of(Expr::BVar(1)), bool_ty()),
        )),
    );
    add("HashSet.any", any_ty)?;
    let all_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), bool_ty()),
            arr(hashset_of(Expr::BVar(1)), bool_ty()),
        )),
    );
    add("HashSet.all", all_ty)?;
    let find_first_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            arr(Expr::BVar(0), bool_ty()),
            arr(hashset_of(Expr::BVar(1)), option_of(Expr::BVar(2))),
        )),
    );
    add("HashSet.findFirst?", find_first_ty)?;
    Ok(())
}
/// Compute the symmetric difference of two sets.
pub fn symmetric_difference<T: Eq + Hash + Clone>(
    a: &OxiHashSet<T>,
    b: &OxiHashSet<T>,
) -> OxiHashSet<T> {
    let mut result = OxiHashSet::new();
    for x in a.iter() {
        if !b.contains(x) {
            result.insert(x.clone());
        }
    }
    for x in b.iter() {
        if !a.contains(x) {
            result.insert(x.clone());
        }
    }
    result
}
/// Check if two sets are disjoint (no elements in common).
pub fn are_disjoint<T: Eq + Hash + Clone>(a: &OxiHashSet<T>, b: &OxiHashSet<T>) -> bool {
    a.iter().all(|x| !b.contains(x))
}
/// Partition a set into elements satisfying and not satisfying a predicate.
pub fn partition<T, P>(set: &OxiHashSet<T>, pred: P) -> (OxiHashSet<T>, OxiHashSet<T>)
where
    T: Eq + Hash + Clone,
    P: Fn(&T) -> bool,
{
    let mut yes = OxiHashSet::new();
    let mut no = OxiHashSet::new();
    for x in set.iter() {
        if pred(x) {
            yes.insert(x.clone());
        } else {
            no.insert(x.clone());
        }
    }
    (yes, no)
}
/// Build a set from an exclusive range `[start, end)`.
pub fn range_set_i64(start: i64, end: i64) -> OxiHashSet<i64> {
    OxiHashSet::from_iter(start..end)
}
/// Check whether the set forms a contiguous range `[min, max]`.
pub fn is_contiguous_range(set: &OxiHashSet<i64>) -> bool {
    if set.is_empty() {
        return true;
    }
    let mut v: Vec<i64> = set.iter().cloned().collect();
    v.sort();
    v.windows(2).all(|w| w[1] == w[0] + 1)
}
#[cfg(test)]
mod hashset_extra_tests {
    use super::*;
    #[test]
    fn test_symmetric_difference() {
        let a = OxiHashSet::from_iter(vec![1i32, 2, 3]);
        let b = OxiHashSet::from_iter(vec![2i32, 3, 4]);
        let sym = symmetric_difference(&a, &b);
        assert_eq!(sym.len(), 2);
        assert!(sym.contains(&1));
        assert!(sym.contains(&4));
        assert!(!sym.contains(&2));
    }
    #[test]
    fn test_are_disjoint_true() {
        let a = OxiHashSet::from_iter(vec![1i32, 2]);
        let b = OxiHashSet::from_iter(vec![3i32, 4]);
        assert!(are_disjoint(&a, &b));
    }
    #[test]
    fn test_are_disjoint_false() {
        let a = OxiHashSet::from_iter(vec![1i32, 2, 3]);
        let b = OxiHashSet::from_iter(vec![3i32, 4, 5]);
        assert!(!are_disjoint(&a, &b));
    }
    #[test]
    fn test_partition() {
        let s = OxiHashSet::from_iter(vec![1i32, 2, 3, 4, 5]);
        let (evens, odds) = partition(&s, |x| x % 2 == 0);
        assert_eq!(evens.len(), 2);
        assert_eq!(odds.len(), 3);
        assert!(evens.contains(&2) && evens.contains(&4));
        assert!(odds.contains(&1) && odds.contains(&3) && odds.contains(&5));
    }
    #[test]
    fn test_range_set_i64() {
        let s = range_set_i64(1, 6);
        assert_eq!(s.len(), 5);
        for i in 1..6 {
            assert!(s.contains(&i));
        }
    }
    #[test]
    fn test_is_contiguous_range_true() {
        let s = range_set_i64(3, 8);
        assert!(is_contiguous_range(&s));
    }
    #[test]
    fn test_is_contiguous_range_false() {
        let s = OxiHashSet::from_iter(vec![1i64, 2, 4, 5]);
        assert!(!is_contiguous_range(&s));
    }
    #[test]
    fn test_is_contiguous_range_empty() {
        let s: OxiHashSet<i64> = OxiHashSet::new();
        assert!(is_contiguous_range(&s));
    }
    #[test]
    fn test_to_sorted_vec() {
        let s = OxiHashSet::from_iter(vec![5i32, 3, 1, 4, 2]);
        let v = s.to_sorted_vec();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
pub fn hs_ext_finite_set_type(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let is_finite_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(hashset_of(Expr::BVar(0)), cst("Prop"))),
    );
    add("HashSet.isFinite", is_finite_ty)?;
    let as_finite_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(hashset_of(Expr::BVar(0)), type1())),
    );
    add("HashSet.asFiniteSubset", as_finite_ty)?;
    Ok(())
}
pub fn hs_ext_membership_axioms(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let bool_ty = || -> Expr { cst("Bool") };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let mem_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(Expr::BVar(0), arr(hashset_of(Expr::BVar(1)), prop()))),
    );
    add("HashSet.mem", mem_ty)?;
    let mem_iff_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                Expr::BVar(1),
                app(
                    app(
                        cst("Iff"),
                        app(app(cst("HashSet.mem"), Expr::BVar(2)), Expr::BVar(1)),
                    ),
                    app(
                        app(
                            cst("Eq"),
                            app(app(cst("HashSet.contains"), Expr::BVar(2)), Expr::BVar(1)),
                        ),
                        bool_ty(),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.mem_iff_contains", mem_iff_ty)?;
    let not_mem_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                app(app(cst("HashSet.mem"), Expr::BVar(1)), cst("HashSet.empty")),
                cst("False"),
            ),
        )),
    );
    add("HashSet.not_mem_empty", not_mem_empty_ty)?;
    Ok(())
}
pub fn hs_ext_insert_delete_axioms(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let _prop = || -> Expr { Expr::Sort(Level::zero()) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let mem_insert_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                Expr::BVar(1),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Iff"),
                            app(
                                app(cst("HashSet.mem"), Expr::BVar(0)),
                                app(app(cst("HashSet.insert"), Expr::BVar(1)), Expr::BVar(2)),
                            ),
                        ),
                        app(
                            app(cst("Or"), app(app(cst("Eq"), Expr::BVar(0)), Expr::BVar(1))),
                            app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.mem_insert", mem_insert_ty)?;
    let mem_erase_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                Expr::BVar(1),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Iff"),
                            app(
                                app(cst("HashSet.mem"), Expr::BVar(0)),
                                app(app(cst("HashSet.erase"), Expr::BVar(1)), Expr::BVar(2)),
                            ),
                        ),
                        app(
                            app(
                                cst("And"),
                                arr(
                                    app(app(cst("Eq"), Expr::BVar(0)), Expr::BVar(1)),
                                    cst("False"),
                                ),
                            ),
                            app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.mem_erase", mem_erase_ty)?;
    let insert_insert_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(
                            app(cst("HashSet.insert"), Expr::BVar(0)),
                            app(app(cst("HashSet.insert"), Expr::BVar(0)), Expr::BVar(1)),
                        ),
                    ),
                    app(app(cst("HashSet.insert"), Expr::BVar(0)), Expr::BVar(1)),
                ),
            ),
        )),
    );
    add("HashSet.insert_insert_idempotent", insert_insert_ty)?;
    let erase_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            app(
                app(
                    cst("Eq"),
                    app(
                        app(cst("HashSet.erase"), Expr::BVar(0)),
                        cst("HashSet.empty"),
                    ),
                ),
                cst("HashSet.empty"),
            ),
        )),
    );
    add("HashSet.erase_empty", erase_empty_ty)?;
    let insert_erase_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(1)),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                app(cst("HashSet.insert"), Expr::BVar(0)),
                                app(app(cst("HashSet.erase"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                        ),
                        Expr::BVar(1),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.insert_erase_mem", insert_erase_ty)?;
    let size_insert_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    arr(
                        app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(1)),
                        cst("False"),
                    ),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                cst("HashSet.size"),
                                app(app(cst("HashSet.insert"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                        ),
                        app(
                            app(cst("Nat.succ"), app(cst("HashSet.size"), Expr::BVar(1))),
                            cst("Nat.zero"),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.size_insert_new", size_insert_ty)?;
    let size_erase_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(1)),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                app(
                                    cst("Nat.add"),
                                    app(
                                        cst("HashSet.size"),
                                        app(
                                            app(cst("HashSet.erase"), Expr::BVar(0)),
                                            Expr::BVar(1),
                                        ),
                                    ),
                                ),
                                cst("1"),
                            ),
                        ),
                        app(cst("HashSet.size"), Expr::BVar(1)),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.size_erase_mem", size_erase_ty)?;
    Ok(())
}
pub fn hs_ext_union_laws(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let union_comm_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                    app(app(cst("HashSet.union"), Expr::BVar(1)), Expr::BVar(0)),
                ),
            ),
        )),
    );
    add("HashSet.union_comm", union_comm_ty)?;
    let union_assoc_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                app(
                                    cst("HashSet.union"),
                                    app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                                ),
                                Expr::BVar(2),
                            ),
                        ),
                        app(
                            app(cst("HashSet.union"), Expr::BVar(0)),
                            app(app(cst("HashSet.union"), Expr::BVar(1)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.union_assoc", union_assoc_ty)?;
    let union_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(
                        app(cst("HashSet.union"), Expr::BVar(0)),
                        cst("HashSet.empty"),
                    ),
                ),
                Expr::BVar(0),
            ),
        )),
    );
    add("HashSet.union_empty", union_empty_ty)?;
    let empty_union_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(
                        app(cst("HashSet.union"), cst("HashSet.empty")),
                        Expr::BVar(0),
                    ),
                ),
                Expr::BVar(0),
            ),
        )),
    );
    add("HashSet.empty_union", empty_union_ty)?;
    let union_self_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(0)),
                ),
                Expr::BVar(0),
            ),
        )),
    );
    add("HashSet.union_self", union_self_ty)?;
    let mem_union_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Iff"),
                            app(
                                app(cst("HashSet.mem"), Expr::BVar(0)),
                                app(app(cst("HashSet.union"), Expr::BVar(1)), Expr::BVar(2)),
                            ),
                        ),
                        app(
                            app(
                                cst("Or"),
                                app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                            app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.mem_union", mem_union_ty)?;
    Ok(())
}
pub fn hs_ext_intersection_laws(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let inter_comm_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Eq"),
                        app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                    app(app(cst("HashSet.inter"), Expr::BVar(1)), Expr::BVar(0)),
                ),
            ),
        )),
    );
    add("HashSet.inter_comm", inter_comm_ty)?;
    let inter_assoc_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                app(
                                    cst("HashSet.inter"),
                                    app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                                ),
                                Expr::BVar(2),
                            ),
                        ),
                        app(
                            app(cst("HashSet.inter"), Expr::BVar(0)),
                            app(app(cst("HashSet.inter"), Expr::BVar(1)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.inter_assoc", inter_assoc_ty)?;
    let inter_self_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(0)),
                ),
                Expr::BVar(0),
            ),
        )),
    );
    add("HashSet.inter_self", inter_self_ty)?;
    let inter_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(
                        app(cst("HashSet.inter"), Expr::BVar(0)),
                        cst("HashSet.empty"),
                    ),
                ),
                cst("HashSet.empty"),
            ),
        )),
    );
    add("HashSet.inter_empty", inter_empty_ty)?;
    let mem_inter_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Iff"),
                            app(
                                app(cst("HashSet.mem"), Expr::BVar(0)),
                                app(app(cst("HashSet.inter"), Expr::BVar(1)), Expr::BVar(2)),
                            ),
                        ),
                        app(
                            app(
                                cst("And"),
                                app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                            app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.mem_inter", mem_inter_ty)?;
    let union_distrib_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                app(cst("HashSet.union"), Expr::BVar(0)),
                                app(app(cst("HashSet.inter"), Expr::BVar(1)), Expr::BVar(2)),
                            ),
                        ),
                        app(
                            app(
                                cst("HashSet.inter"),
                                app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                            app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.union_distrib_inter", union_distrib_ty)?;
    Ok(())
}
pub fn hs_ext_difference_laws(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let prop = || -> Expr { Expr::Sort(Level::zero()) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let mem_sdiff_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            Expr::BVar(0),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    app(
                        app(
                            cst("Iff"),
                            app(
                                app(cst("HashSet.mem"), Expr::BVar(0)),
                                app(app(cst("HashSet.sdiff"), Expr::BVar(1)), Expr::BVar(2)),
                            ),
                        ),
                        app(
                            app(
                                cst("And"),
                                app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(1)),
                            ),
                            arr(
                                app(app(cst("HashSet.mem"), Expr::BVar(0)), Expr::BVar(2)),
                                cst("False"),
                            ),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.mem_sdiff", mem_sdiff_ty)?;
    let sdiff_empty_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(
                        app(cst("HashSet.sdiff"), Expr::BVar(0)),
                        cst("HashSet.empty"),
                    ),
                ),
                Expr::BVar(0),
            ),
        )),
    );
    add("HashSet.sdiff_empty", sdiff_empty_ty)?;
    let empty_sdiff_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(
                        app(cst("HashSet.sdiff"), cst("HashSet.empty")),
                        Expr::BVar(0),
                    ),
                ),
                cst("HashSet.empty"),
            ),
        )),
    );
    add("HashSet.empty_sdiff", empty_sdiff_ty)?;
    let sdiff_self_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(app(cst("HashSet.sdiff"), Expr::BVar(0)), Expr::BVar(0)),
                ),
                cst("HashSet.empty"),
            ),
        )),
    );
    add("HashSet.sdiff_self", sdiff_self_ty)?;
    let _ = prop();
    let sdiff_union_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    app(app(cst("HashSet.subset"), Expr::BVar(0)), Expr::BVar(1)),
                    app(
                        app(
                            cst("Eq"),
                            app(
                                app(
                                    cst("HashSet.union"),
                                    app(app(cst("HashSet.sdiff"), Expr::BVar(1)), Expr::BVar(0)),
                                ),
                                Expr::BVar(0),
                            ),
                        ),
                        Expr::BVar(1),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.sdiff_union_of_subset", sdiff_union_ty)?;
    Ok(())
}
pub fn hs_ext_subset_partial_order(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let subset_refl_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(app(cst("HashSet.subset"), Expr::BVar(0)), Expr::BVar(0)),
        )),
    );
    add("HashSet.subset_refl", subset_refl_ty)?;
    let subset_trans_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    hashset_of(Expr::BVar(2)),
                    arr(
                        app(app(cst("HashSet.subset"), Expr::BVar(0)), Expr::BVar(1)),
                        arr(
                            app(app(cst("HashSet.subset"), Expr::BVar(1)), Expr::BVar(2)),
                            app(app(cst("HashSet.subset"), Expr::BVar(0)), Expr::BVar(2)),
                        ),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.subset_trans", subset_trans_ty)?;
    let subset_antisymm_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                arr(
                    app(app(cst("HashSet.subset"), Expr::BVar(0)), Expr::BVar(1)),
                    arr(
                        app(app(cst("HashSet.subset"), Expr::BVar(1)), Expr::BVar(0)),
                        app(app(cst("Eq"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                ),
            ),
        )),
    );
    add("HashSet.subset_antisymm", subset_antisymm_ty)?;
    let empty_subset_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(cst("HashSet.subset"), cst("HashSet.empty")),
                Expr::BVar(0),
            ),
        )),
    );
    add("HashSet.empty_subset", empty_subset_ty)?;
    let subset_union_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(cst("HashSet.subset"), Expr::BVar(0)),
                    app(app(cst("HashSet.union"), Expr::BVar(0)), Expr::BVar(1)),
                ),
            ),
        )),
    );
    add("HashSet.subset_self_union_left", subset_union_ty)?;
    let inter_subset_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("HashSet.subset"),
                        app(app(cst("HashSet.inter"), Expr::BVar(0)), Expr::BVar(1)),
                    ),
                    Expr::BVar(0),
                ),
            ),
        )),
    );
    add("HashSet.inter_subset_left", inter_subset_ty)?;
    Ok(())
}
pub fn hs_ext_power_set_axioms(
    add: &mut impl FnMut(&str, oxilean_kernel::Expr) -> Result<(), String>,
) -> Result<(), String> {
    use super::functions::*;
    use oxilean_kernel::{BinderInfo as Bi, Expr, Level, Name};
    let cst = |s: &str| -> Expr { Expr::Const(Name::str(s), vec![]) };
    let app = |f: Expr, a: Expr| -> Expr { Expr::App(Box::new(f), Box::new(a)) };
    let arr = |a: Expr, b: Expr| -> Expr {
        Expr::Pi(Bi::Default, Name::Anonymous, Box::new(a), Box::new(b))
    };
    let type1 = || -> Expr { Expr::Sort(Level::succ(Level::zero())) };
    let nat_ty = || -> Expr { cst("Nat") };
    let hashset_of = |ty: Expr| -> Expr { app(cst("HashSet"), ty) };
    let powerset_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            hashset_of(hashset_of(Expr::BVar(1))),
        )),
    );
    add("HashSet.powerset", powerset_ty)?;
    let mem_powerset_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            arr(
                hashset_of(Expr::BVar(1)),
                app(
                    app(
                        cst("Iff"),
                        app(
                            app(cst("HashSet.mem"), Expr::BVar(0)),
                            app(cst("HashSet.powerset"), Expr::BVar(1)),
                        ),
                    ),
                    app(app(cst("HashSet.subset"), Expr::BVar(0)), Expr::BVar(1)),
                ),
            ),
        )),
    );
    add("HashSet.mem_powerset", mem_powerset_ty)?;
    let card_powerset_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(
                    cst("Eq"),
                    app(
                        cst("HashSet.size"),
                        app(cst("HashSet.powerset"), Expr::BVar(0)),
                    ),
                ),
                app(
                    app(cst("Nat.pow"), nat_ty()),
                    app(cst("HashSet.size"), Expr::BVar(0)),
                ),
            ),
        )),
    );
    add("HashSet.card_powerset", card_powerset_ty)?;
    let cantor_ty = Expr::Pi(
        Bi::Implicit,
        Name::str("α"),
        Box::new(type1()),
        Box::new(arr(
            hashset_of(Expr::BVar(0)),
            app(
                app(cst("Nat.lt"), app(cst("HashSet.size"), Expr::BVar(0))),
                app(
                    cst("HashSet.size"),
                    app(cst("HashSet.powerset"), Expr::BVar(0)),
                ),
            ),
        )),
    );
    add("HashSet.cantor_strict_lt", cantor_ty)?;
    Ok(())
}
