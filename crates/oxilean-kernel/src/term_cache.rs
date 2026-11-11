//! Caches keyed by terms, and the equality that verifies their keys.
//!
//! The reducer, the def-eq checker and the type checker each remember what
//! they computed for a term. A term is a DAG, and the caches are what makes
//! the kernel's work proportional to the DAG: a subterm reached along many
//! paths is reduced, compared or typed once, and found the next time.
//!
//! Two things made the caches miss exactly the terms that needed them:
//!
//! * they kept a term only if it was small AS A TREE (4,096 nodes), and a
//!   term of a few hundred shared nodes can be a tree of millions — so the
//!   heavily shared terms were recomputed on every visit;
//! * their keys were compared with `==`, which follows every edge: verifying
//!   a hit on a shared term cost its size as a tree.
//!
//! [`same_term`] is structural equality that visits a pair of shared
//! subterms once, and [`TermCache`] / [`PairCache`] are tables keyed by the
//! 64-bit structural hash every term stores, with the stored key verified by
//! `same_term`. Neither has a size limit on the terms it handles.
//!
//! # Soundness
//!
//! A lookup returns a value only for a key that [`same_term`] found equal to
//! the query — exactly what `==` decides, so a table of this module answers
//! as a `HashMap<Expr, V>` would, except that it may forget: two different
//! terms with the same 64-bit hash share a slot, and the later insertion
//! replaces the earlier. Forgetting costs a recomputation; it cannot return a
//! value computed for another term.

use std::collections::{HashMap, HashSet};

use crate::node_memo::WordBuildHasher;
use crate::structural_hash::mix;
use crate::{Expr, Node};

/// A pair of shared subterms is remembered as equal only when the subterms
/// are at least this big (as trees): below it, comparing again is cheaper
/// than a table lookup.
const PAIR_MEMO_MIN_COST: u32 = 64;

/// Structural equality of two terms — exactly what `==` decides — visiting
/// each pair of shared subterms once.
///
/// `==` compares two equal terms that are separate allocations by walking
/// both, every edge of them: two equal terms of a few hundred shared nodes
/// each are compared as the trees they stand for. Here a pair of subterms
/// that both have several owners is remembered once it has been found equal
/// (by address — sound for the duration of the call, during which both terms
/// are borrowed and none of their subterms can be freed), and the next path
/// to the same pair returns at once.
///
/// As in `==`, one allocation is equal to itself and two subterms whose
/// stored hashes differ are different; only what is left is compared.
#[must_use]
pub(crate) fn same_term(a: &Expr, b: &Expr) -> bool {
    let mut walk = SameTerm {
        equal_pairs: HashSet::default(),
    };
    walk.exprs(a, b)
}

/// One [`same_term`] call.
struct SameTerm {
    /// Pairs of shared subterms already found equal, by address. Allocated
    /// on the first insertion.
    equal_pairs: HashSet<(*const Expr, *const Expr), WordBuildHasher>,
}

impl SameTerm {
    fn nodes(&mut self, a: &Node, b: &Node) -> bool {
        if Node::ptr_eq(a, b) {
            return true;
        }
        if a.structural_hash() != b.structural_hash() {
            return false;
        }
        let remember = a.rebuild_cost() >= PAIR_MEMO_MIN_COST && a.is_shared() && b.is_shared();
        if remember
            && !self.equal_pairs.is_empty()
            && self.equal_pairs.contains(&(a.as_ptr(), b.as_ptr()))
        {
            return true;
        }
        let equal = self.exprs(a, b);
        if equal && remember {
            self.equal_pairs.insert((a.as_ptr(), b.as_ptr()));
        }
        equal
    }

    /// The same case analysis as the derived `PartialEq` of [`Expr`]: every
    /// field takes part, binder names and binder annotations included.
    fn exprs(&mut self, a: &Expr, b: &Expr) -> bool {
        match (a, b) {
            (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
            (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
            (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
            (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
            (Expr::App(f1, a1), Expr::App(f2, a2)) => self.nodes(f1, f2) && self.nodes(a1, a2),
            (Expr::Lam(bi1, n1, ty1, b1), Expr::Lam(bi2, n2, ty2, b2))
            | (Expr::Pi(bi1, n1, ty1, b1), Expr::Pi(bi2, n2, ty2, b2)) => {
                bi1 == bi2 && n1 == n2 && self.nodes(ty1, ty2) && self.nodes(b1, b2)
            }
            (Expr::Let(n1, ty1, v1, b1), Expr::Let(n2, ty2, v2, b2)) => {
                n1 == n2 && self.nodes(ty1, ty2) && self.nodes(v1, v2) && self.nodes(b1, b2)
            }
            (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
            (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
                n1 == n2 && i1 == i2 && self.nodes(e1, e2)
            }
            _ => false,
        }
    }
}

/// What was computed for a term, by term. See the module documentation.
pub(crate) struct TermCache<V> {
    /// By structural hash: the term the value was computed for, and the
    /// value.
    entries: HashMap<u64, (Expr, V), WordBuildHasher>,
}

impl<V> TermCache<V> {
    /// An empty cache.
    pub(crate) fn new() -> Self {
        Self {
            entries: HashMap::default(),
        }
    }

    /// The value stored for `term`, if any.
    pub(crate) fn get(&self, term: &Expr) -> Option<&V> {
        if self.entries.is_empty() {
            return None;
        }
        let (key, value) = self.entries.get(&term.structural_hash())?;
        same_term(key, term).then_some(value)
    }

    /// Store `value` for `term`, replacing whatever shares its hash.
    pub(crate) fn insert(&mut self, term: &Expr, value: V) {
        self.entries
            .insert(term.structural_hash(), (term.clone(), value));
    }

    /// Forget everything.
    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }

    /// How many terms have a value stored (for tests).
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
}

/// What was computed for an ordered pair of terms, by pair. See the module
/// documentation.
pub(crate) struct PairCache<V> {
    /// By the combined structural hash of the pair: the two terms the value
    /// was computed for, and the value.
    entries: HashMap<u64, (Expr, Expr, V), WordBuildHasher>,
}

impl<V> PairCache<V> {
    /// An empty cache.
    pub(crate) fn new() -> Self {
        Self {
            entries: HashMap::default(),
        }
    }

    /// The slot of the ordered pair `(a, b)`. `mix` is not commutative, so
    /// `(a, b)` and `(b, a)` have different slots.
    fn slot(a: &Expr, b: &Expr) -> u64 {
        mix(a.structural_hash(), b.structural_hash())
    }

    /// The value stored for the ordered pair `(a, b)`, if any.
    pub(crate) fn get(&self, a: &Expr, b: &Expr) -> Option<&V> {
        if self.entries.is_empty() {
            return None;
        }
        let (key_a, key_b, value) = self.entries.get(&Self::slot(a, b))?;
        (same_term(key_a, a) && same_term(key_b, b)).then_some(value)
    }

    /// Store `value` for the ordered pair `(a, b)`, replacing whatever
    /// shares its slot.
    pub(crate) fn insert(&mut self, a: &Expr, b: &Expr, value: V) {
        self.entries
            .insert(Self::slot(a, b), (a.clone(), b.clone(), value));
    }

    /// Forget everything.
    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }

    /// How many pairs have a value stored.
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, FVarId, Level, Literal, Name};

    fn lcg(state: &mut u64) -> u64 {
        *state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        *state >> 33
    }

    /// A random term that reuses subterms it has already built: a DAG.
    /// `pool` holds the shared subterms; each new node picks children from
    /// the pool or makes fresh leaves.
    fn random_dag(state: &mut u64, steps: usize) -> Expr {
        fn leaf(state: &mut u64) -> Expr {
            match lcg(state) % 6 {
                0 => Expr::BVar((lcg(state) % 3) as u32),
                1 => Expr::Sort(Level::zero()),
                2 => Expr::FVar(FVarId(lcg(state) % 3)),
                3 => Expr::Lit(Literal::nat(lcg(state) % 3)),
                4 => Expr::Const(Name::str("d"), vec![Level::param(Name::str("u"))]),
                _ => Expr::Const(Name::str("c"), Vec::new()),
            }
        }
        let mut pool: Vec<Node> = (0..4).map(|_| Node::new(leaf(state))).collect();
        for _ in 0..steps {
            let pick = |state: &mut u64, pool: &Vec<Node>| -> Node {
                if lcg(state).is_multiple_of(4) {
                    Node::new(leaf(state))
                } else {
                    pool[(lcg(state) as usize) % pool.len()].clone()
                }
            };
            let name = Name::str(if lcg(state).is_multiple_of(2) {
                "x"
            } else {
                "y"
            });
            let bi = if lcg(state).is_multiple_of(2) {
                BinderInfo::Default
            } else {
                BinderInfo::Implicit
            };
            let node = match lcg(state) % 6 {
                0 | 1 => Expr::App(pick(state, &pool), pick(state, &pool)),
                2 => Expr::Lam(bi, name, pick(state, &pool), pick(state, &pool)),
                3 => Expr::Pi(bi, name, pick(state, &pool), pick(state, &pool)),
                4 => Expr::Let(
                    name,
                    pick(state, &pool),
                    pick(state, &pool),
                    pick(state, &pool),
                ),
                _ => Expr::Proj(Name::str("S"), (lcg(state) % 2) as u32, pick(state, &pool)),
            };
            pool.push(Node::new(node));
        }
        let last = pool.len() - 1;
        (*pool[last]).clone()
    }

    /// A copy of `e` in which no allocation is shared with `e` — or with
    /// itself: the tree `e` stands for.
    fn unshared_copy(e: &Expr) -> Expr {
        let copy = |node: &Node| Node::new(unshared_copy(node));
        match e {
            Expr::App(f, a) => Expr::App(copy(f), copy(a)),
            Expr::Lam(bi, n, ty, b) => Expr::Lam(*bi, n.clone(), copy(ty), copy(b)),
            Expr::Pi(bi, n, ty, b) => Expr::Pi(*bi, n.clone(), copy(ty), copy(b)),
            Expr::Let(n, ty, v, b) => Expr::Let(n.clone(), copy(ty), copy(v), copy(b)),
            Expr::Proj(n, i, s) => Expr::Proj(n.clone(), *i, copy(s)),
            leaf => leaf.clone(),
        }
    }

    #[test]
    fn same_term_decides_what_equality_decides() {
        let mut state = 0x5eed_0001_u64;
        let terms: Vec<Expr> = (0..120).map(|_| random_dag(&mut state, 14)).collect();
        let mut equal_pairs = 0;
        for (i, a) in terms.iter().enumerate() {
            // Against itself, against a copy sharing nothing, and against
            // every other term.
            assert!(same_term(a, a), "term {i} against itself");
            let copy = unshared_copy(a);
            assert!(same_term(a, &copy), "term {i} against its copy");
            assert!(same_term(&copy, a), "copy of term {i} against it");
            for (j, b) in terms.iter().enumerate() {
                let expected = a == b;
                assert_eq!(same_term(a, b), expected, "terms {i} and {j}");
                if expected && i != j {
                    equal_pairs += 1;
                }
            }
        }
        // The generator is small enough to produce equal terms more than once.
        assert!(equal_pairs > 0, "no two generated terms were equal");
    }

    #[test]
    fn same_term_tells_binder_names_and_annotations_apart() {
        let body = Node::new(Expr::BVar(0));
        let ty = Node::new(Expr::Sort(Level::zero()));
        let lam = |bi, name: &str| Expr::Lam(bi, Name::str(name), ty.clone(), body.clone());
        assert!(same_term(
            &lam(BinderInfo::Default, "x"),
            &lam(BinderInfo::Default, "x")
        ));
        assert!(!same_term(
            &lam(BinderInfo::Default, "x"),
            &lam(BinderInfo::Default, "y")
        ));
        assert!(!same_term(
            &lam(BinderInfo::Default, "x"),
            &lam(BinderInfo::Implicit, "x")
        ));
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            ty.clone(),
            body.clone(),
        );
        assert!(!same_term(&lam(BinderInfo::Default, "x"), &pi));
    }

    /// `xₖ₊₁ = xₖ xₖ`, both children one allocation: `height` allocations
    /// standing for a tree of `2^height` leaves.
    fn tower(leaf: &str, height: u32) -> Expr {
        let mut term = Expr::Const(Name::str(leaf), Vec::new());
        for _ in 0..height {
            let shared = Node::new(term);
            term = Expr::App(shared.clone(), shared);
        }
        term
    }

    #[test]
    fn two_separate_towers_are_compared_without_walking_their_trees() {
        // Each tower is 34 allocations and stands for 2^34 leaves; the two
        // share nothing. `==` walks the trees — a minute or more for these.
        // Here each level's pair is compared once: microseconds. (The bound
        // is five orders of magnitude above what it takes and far below what
        // a walk of the trees takes, so a regression fails instead of
        // stalling the test.)
        let start = crate::wall_clock::Instant::now();
        let a = tower("c", 34);
        let b = tower("c", 34);
        assert!(same_term(&a, &b));
        let c = tower("d", 34);
        assert!(!same_term(&a, &c));
        // Different heights: the hashes differ at the root.
        assert!(!same_term(&a, &tower("c", 33)));
        assert!(
            start.elapsed().as_secs() < 10,
            "comparing two 34-level towers took {:?}",
            start.elapsed()
        );
    }

    #[test]
    fn a_cache_finds_an_equal_term_in_another_allocation() {
        let mut cache: TermCache<u32> = TermCache::new();
        assert!(cache.get(&tower("c", 50)).is_none());
        cache.insert(&tower("c", 50), 1);
        cache.insert(&tower("d", 50), 2);
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&tower("c", 50)), Some(&1));
        assert_eq!(cache.get(&tower("d", 50)), Some(&2));
        assert_eq!(cache.get(&tower("c", 49)), None);
        cache.insert(&tower("c", 50), 3);
        assert_eq!(cache.len(), 2, "the same term keeps its slot");
        assert_eq!(cache.get(&tower("c", 50)), Some(&3));
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.get(&tower("c", 50)), None);
    }

    #[test]
    fn a_cache_answers_like_a_map_keyed_by_the_terms() {
        let mut state = 0x5eed_0002_u64;
        let terms: Vec<Expr> = (0..200).map(|_| random_dag(&mut state, 10)).collect();
        let mut cache: TermCache<usize> = TermCache::new();
        let mut reference: HashMap<Expr, usize> = HashMap::new();
        for (i, term) in terms.iter().enumerate() {
            assert_eq!(cache.get(term), reference.get(term), "before inserting {i}");
            cache.insert(term, i);
            reference.insert(term.clone(), i);
        }
        for (i, term) in terms.iter().enumerate() {
            // No two of these terms collide on the 64-bit hash, so nothing
            // was forgotten.
            assert_eq!(cache.get(term), reference.get(term), "term {i}");
            assert_eq!(cache.get(&unshared_copy(term)), reference.get(term));
        }
        assert_eq!(cache.len(), reference.len());
    }

    #[test]
    fn a_pair_cache_is_keyed_by_the_ordered_pair() {
        let a = tower("c", 40);
        let b = tower("d", 40);
        let mut cache: PairCache<bool> = PairCache::new();
        assert!(cache.get(&a, &b).is_none());
        cache.insert(&a, &b, true);
        assert_eq!(cache.get(&a, &b), Some(&true));
        assert_eq!(cache.get(&tower("c", 40), &tower("d", 40)), Some(&true));
        assert_eq!(cache.get(&b, &a), None, "the other order is another key");
        assert_eq!(cache.get(&a, &a), None);
        cache.insert(&b, &a, false);
        assert_eq!(cache.get(&b, &a), Some(&false));
        assert_eq!(cache.get(&a, &b), Some(&true));
        assert_eq!(cache.len(), 2);
        cache.clear();
        assert!(cache.get(&a, &b).is_none());
    }
}
