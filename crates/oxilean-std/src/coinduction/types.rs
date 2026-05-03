//! Coinductive proofs and greatest fixed points — types.

/// A lazy, eventually-periodic stream.
///
/// The stream is defined by a finite `prefix` followed by an infinitely
/// repeating `cycle`.  If `cycle` is empty, the stream is exactly `prefix`
/// (i.e. finite for practical purposes but the API treats it as an
/// infinite stream repeating the last element when both slices are exhausted,
/// which is handled in the accessor functions).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LazyStream<T: Clone> {
    /// Finite prefix elements (emitted first, each exactly once).
    pub(super) prefix: Vec<T>,
    /// Repeating cycle (emitted round-robin after the prefix runs out).
    /// Must be non-empty unless the stream is intended to be empty.
    pub(super) cycle: Vec<T>,
}

/// A set of state pairs representing a candidate bisimulation relation.
///
/// A `BisimulationRelation<S>` is *valid* if for every pair `(s, t)` in the
/// relation, every transition from `s` has a matching transition from `t`
/// (and vice-versa) such that the resulting states are again in the relation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BisimulationRelation<S: Clone + Eq> {
    pub(super) pairs: Vec<(S, S)>,
}

/// A completed coinductive proof certificate.
///
/// Contains the bisimulation relation that was found to be valid, plus a
/// trace of the progress steps made during the proof search.
#[derive(Clone, Debug)]
pub struct CoinductiveProof<S: Clone + Eq> {
    pub relation: BisimulationRelation<S>,
    pub progress: Vec<String>,
}

/// A coalgebra map S → O × S represented as an explicit transition table.
///
/// Each entry `(src, obs, dst)` means: when in state `src`, emit observation
/// `obs` and move to state `dst`.
#[derive(Clone, Debug)]
pub struct CoalgebraMap<S: Clone + Eq, O: Clone + Eq> {
    pub(super) transitions: Vec<(S, O, S)>,
}

/// A Greibach Normal Form grammar used for coinductive context-free grammars.
///
/// Each rule `(lhs, terminal, rhs)` expands non-terminal `lhs` by consuming
/// terminal `terminal` and pushing `rhs` non-terminals onto the stack.
#[derive(Clone, Debug)]
pub struct GreibachNormalForm {
    pub(super) rules: Vec<(String, char, Vec<String>)>,
}

/// `Codata<F>` represents the greatest fixed point of the functor `F`.
///
/// Because Rust does not support infinite types directly, we model it as an
/// explicit thunk: call `unfold()` to expose one layer of the codata on demand.
pub struct Codata<F> {
    /// The unfolding thunk: call it to expose one layer of the codata.
    pub(super) unfold: Box<dyn Fn() -> F>,
}

/// A finite approximation of an infinite stream (one unrolled cons cell).
#[derive(Clone, Debug)]
pub struct StreamNode<T: Clone> {
    pub head: T,
    pub tail: Box<StreamApprox<T>>,
}

/// A finite approximation of a stream (either `Nil` or a `Cons`).
#[derive(Clone, Debug)]
pub enum StreamApprox<T: Clone> {
    /// Stream exhausted at this depth.
    Nil,
    Cons(Box<StreamNode<T>>),
}
