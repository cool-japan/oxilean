//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Representation of a Galois connection between two posets.
///
/// A Galois connection is a pair of monotone maps `(l : A → B, u : B → A)`
/// satisfying: `l(a) ≤ b ↔ a ≤ u(b)`.
#[allow(dead_code)]
pub struct GaloisConnection<A, B> {
    /// Left adjoint (lower adjoint).
    pub lower: Box<dyn Fn(A) -> B>,
    /// Right adjoint (upper adjoint).
    pub upper: Box<dyn Fn(B) -> A>,
}
#[allow(dead_code)]
impl<A: Clone, B: Clone> GaloisConnection<A, B> {
    /// Create a new Galois connection from the adjoint pair.
    pub fn new(lower: impl Fn(A) -> B + 'static, upper: impl Fn(B) -> A + 'static) -> Self {
        Self {
            lower: Box::new(lower),
            upper: Box::new(upper),
        }
    }
    /// Apply the lower adjoint.
    pub fn apply_lower(&self, a: A) -> B {
        (self.lower)(a)
    }
    /// Apply the upper adjoint.
    pub fn apply_upper(&self, b: B) -> A {
        (self.upper)(b)
    }
    /// Verify the Galois connection condition on concrete elements given
    /// order predicates for both sides.
    pub fn verify(
        &self,
        a: A,
        b: B,
        le_a: impl Fn(&A, &A) -> bool,
        le_b: impl Fn(&B, &B) -> bool,
    ) -> bool {
        let la = (self.lower)(a.clone());
        let ub = (self.upper)(b.clone());
        le_b(&la, &b) == le_a(&a, &ub)
    }
}
/// Representation of the Knaster-Tarski fixed-point construction on a
/// complete lattice encoded via a supremum function.
#[allow(dead_code)]
pub struct KnasterTarskiFixpoint<T> {
    /// The monotone endofunction `f`.
    pub func: Box<dyn Fn(T) -> T>,
    /// Supremum over an arbitrary predicate (models `sSup`).
    pub ssup: Box<dyn Fn(Box<dyn Fn(&T) -> bool>) -> T>,
}
#[allow(dead_code)]
impl<T: Clone + PartialEq> KnasterTarskiFixpoint<T> {
    /// Construct a new Knaster-Tarski calculator.
    pub fn new(
        func: impl Fn(T) -> T + 'static,
        ssup: impl Fn(Box<dyn Fn(&T) -> bool>) -> T + 'static,
    ) -> Self {
        Self {
            func: Box::new(func),
            ssup: Box::new(ssup),
        }
    }
    /// Compute the least fixed point by iterating from `bot`.
    pub fn lfp_iterate(&self, bot: T, max_iter: usize) -> Option<T> {
        let mut x = bot;
        for _ in 0..max_iter {
            let fx = (self.func)(x.clone());
            if fx == x {
                return Some(x);
            }
            x = fx;
        }
        None
    }
    /// Check whether a given element is a fixed point of `f`.
    pub fn is_fixed_point(&self, x: T) -> bool {
        let fx = (self.func)(x.clone());
        fx == x
    }
    /// Apply `f` n times starting from `x`.
    pub fn iterate_n(&self, x: T, n: usize) -> T {
        let mut current = x;
        for _ in 0..n {
            current = (self.func)(current);
        }
        current
    }
}
/// Checker for partial-order properties on a finite set encoded as a
/// comparison closure.
#[allow(dead_code)]
pub struct PartialOrderChecker<T> {
    /// Elements of the poset.
    pub elements: Vec<T>,
    /// The underlying partial order relation: returns `true` iff `a ≤ b`.
    pub le: Box<dyn Fn(&T, &T) -> bool>,
}
#[allow(dead_code)]
impl<T: Clone + PartialEq> PartialOrderChecker<T> {
    /// Create a new checker.
    pub fn new(elements: Vec<T>, le: impl Fn(&T, &T) -> bool + 'static) -> Self {
        Self {
            elements,
            le: Box::new(le),
        }
    }
    /// Verify reflexivity: `∀ a, a ≤ a`.
    pub fn check_reflexive(&self) -> bool {
        self.elements.iter().all(|a| (self.le)(a, a))
    }
    /// Verify transitivity: `∀ a b c, a ≤ b ∧ b ≤ c → a ≤ c`.
    pub fn check_transitive(&self) -> bool {
        for a in &self.elements {
            for b in &self.elements {
                for c in &self.elements {
                    if (self.le)(a, b) && (self.le)(b, c) && !(self.le)(a, c) {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Verify antisymmetry: `∀ a b, a ≤ b ∧ b ≤ a → a = b`.
    pub fn check_antisymmetric(&self) -> bool {
        for a in &self.elements {
            for b in &self.elements {
                if (self.le)(a, b) && (self.le)(b, a) && a != b {
                    return false;
                }
            }
        }
        true
    }
    /// Check all three partial-order axioms at once.
    pub fn is_partial_order(&self) -> bool {
        self.check_reflexive() && self.check_transitive() && self.check_antisymmetric()
    }
    /// Find all maximal elements (no strictly larger element exists).
    pub fn maximal_elements(&self) -> Vec<T> {
        self.elements
            .iter()
            .filter(|a| {
                !self
                    .elements
                    .iter()
                    .any(|b| (self.le)(a, b) && !(self.le)(b, a))
            })
            .cloned()
            .collect()
    }
    /// Find all minimal elements.
    pub fn minimal_elements(&self) -> Vec<T> {
        self.elements
            .iter()
            .filter(|a| {
                !self
                    .elements
                    .iter()
                    .any(|b| (self.le)(b, a) && !(self.le)(a, b))
            })
            .cloned()
            .collect()
    }
}
/// Binary lattice operations on elements that implement `Clone + PartialEq`.
#[allow(dead_code)]
pub struct LatticeOps<T> {
    /// Join (supremum) of two elements.
    pub join: Box<dyn Fn(T, T) -> T>,
    /// Meet (infimum) of two elements.
    pub meet: Box<dyn Fn(T, T) -> T>,
}
#[allow(dead_code)]
impl<T: Clone + PartialEq> LatticeOps<T> {
    /// Create from explicit join/meet functions.
    pub fn new(join: impl Fn(T, T) -> T + 'static, meet: impl Fn(T, T) -> T + 'static) -> Self {
        Self {
            join: Box::new(join),
            meet: Box::new(meet),
        }
    }
    /// Compute the join of a non-empty slice.
    pub fn fold_join(&self, elems: &[T]) -> Option<T> {
        let mut iter = elems.iter().cloned();
        let first = iter.next()?;
        Some(iter.fold(first, |acc, x| (self.join)(acc, x)))
    }
    /// Compute the meet of a non-empty slice.
    pub fn fold_meet(&self, elems: &[T]) -> Option<T> {
        let mut iter = elems.iter().cloned();
        let first = iter.next()?;
        Some(iter.fold(first, |acc, x| (self.meet)(acc, x)))
    }
    /// Check absorption: `join a (meet a b) = a`.
    pub fn check_absorption_join_meet(&self, a: T, b: T) -> bool {
        let lhs = (self.join)(a.clone(), (self.meet)(a.clone(), b));
        lhs == a
    }
    /// Check absorption: `meet a (join a b) = a`.
    pub fn check_absorption_meet_join(&self, a: T, b: T) -> bool {
        let lhs = (self.meet)(a.clone(), (self.join)(a.clone(), b));
        lhs == a
    }
}
/// A closure operator on a poset: an extensive, monotone, idempotent
/// endofunction.
#[allow(dead_code)]
pub struct ClosureOperator<T> {
    /// The closure map.
    pub close: Box<dyn Fn(T) -> T>,
    /// The underlying order used to verify properties.
    pub le: Box<dyn Fn(&T, &T) -> bool>,
}
#[allow(dead_code)]
impl<T: Clone + PartialEq> ClosureOperator<T> {
    /// Construct a closure operator.
    pub fn new(close: impl Fn(T) -> T + 'static, le: impl Fn(&T, &T) -> bool + 'static) -> Self {
        Self {
            close: Box::new(close),
            le: Box::new(le),
        }
    }
    /// Verify extensiveness: `a ≤ close(a)`.
    pub fn check_extensive(&self, a: T) -> bool {
        let ca = (self.close)(a.clone());
        (self.le)(&a, &ca)
    }
    /// Verify monotonicity: `a ≤ b → close(a) ≤ close(b)`.
    pub fn check_monotone(&self, a: T, b: T) -> bool {
        if (self.le)(&a, &b) {
            let ca = (self.close)(a);
            let cb = (self.close)(b);
            (self.le)(&ca, &cb)
        } else {
            true
        }
    }
    /// Verify idempotency: `close(close(a)) = close(a)`.
    pub fn check_idempotent(&self, a: T) -> bool {
        let ca = (self.close)(a);
        let cca = (self.close)(ca.clone());
        ca == cca
    }
    /// Apply closure until a fixed point is reached.
    /// Returns `None` after `max_iter` iterations without stabilisation.
    pub fn iterate_to_fixed_point(&self, a: T, max_iter: usize) -> Option<T> {
        let mut current = a;
        for _ in 0..max_iter {
            let next = (self.close)(current.clone());
            if next == current {
                return Some(current);
            }
            current = next;
        }
        None
    }
}
