//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, InductiveType, IntroRule, Level, Name,
};

use super::functions::*;

/// Statistics about a collection of `Coproduct<A, B>` values.
#[derive(Clone, Debug, Default)]
pub struct SumStats {
    /// Number of left values.
    pub left_count: usize,
    /// Number of right values.
    pub right_count: usize,
}
impl SumStats {
    /// Collect statistics from a slice.
    pub fn from_slice<A, B>(xs: &[Coproduct<A, B>]) -> Self {
        let left_count = xs.iter().filter(|c| c.is_left()).count();
        let right_count = xs.len() - left_count;
        Self {
            left_count,
            right_count,
        }
    }
    /// Total count.
    pub fn total(&self) -> usize {
        self.left_count + self.right_count
    }
    /// Fraction of left values (0.0 to 1.0).
    pub fn left_fraction(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            self.left_count as f64 / self.total() as f64
        }
    }
    /// Whether all values are left.
    pub fn all_left(&self) -> bool {
        self.right_count == 0 && self.total() > 0
    }
    /// Whether all values are right.
    pub fn all_right(&self) -> bool {
        self.left_count == 0 && self.total() > 0
    }
}
/// A chain of operations on a coproduct that short-circuits on the left.
pub struct SumChain<A, B> {
    inner: Coproduct<A, B>,
}
impl<A, B> SumChain<A, B> {
    /// Wrap a coproduct.
    pub fn new(c: Coproduct<A, B>) -> Self {
        Self { inner: c }
    }
    /// Map the right side, short-circuiting on the left.
    pub fn map<C>(self, f: impl FnOnce(B) -> C) -> SumChain<A, C> {
        SumChain {
            inner: self.inner.map_right(f),
        }
    }
    /// FlatMap the right side, short-circuiting on the left.
    pub fn flat_map<C>(self, f: impl FnOnce(B) -> Coproduct<A, C>) -> SumChain<A, C> {
        SumChain {
            inner: match self.inner {
                Coproduct::Inl(a) => Coproduct::Inl(a),
                Coproduct::Inr(b) => f(b),
            },
        }
    }
    /// Unwrap the chain.
    pub fn into_inner(self) -> Coproduct<A, B> {
        self.inner
    }
    /// Whether the chain is in the success (right) state.
    pub fn is_ok(&self) -> bool {
        self.inner.is_right()
    }
    /// Whether the chain is in the error (left) state.
    pub fn is_err(&self) -> bool {
        self.inner.is_left()
    }
}
/// Witness of the bifunctor structure on Sum: maps `Sum A B → Sum C D`.
#[allow(dead_code)]
pub struct SumBifunctor<A, B, C, D> {
    /// Phantom for source left type.
    _a: std::marker::PhantomData<A>,
    /// Phantom for source right type.
    _b: std::marker::PhantomData<B>,
    /// Phantom for target left type.
    _c: std::marker::PhantomData<C>,
    /// Phantom for target right type.
    _d: std::marker::PhantomData<D>,
    /// Name tag for this bifunctor instance.
    pub name: String,
}
impl<A, B, C, D> SumBifunctor<A, B, C, D> {
    /// Create a new SumBifunctor.
    pub fn new(name: impl Into<String>) -> Self {
        SumBifunctor {
            _a: std::marker::PhantomData,
            _b: std::marker::PhantomData,
            _c: std::marker::PhantomData,
            _d: std::marker::PhantomData,
            name: name.into(),
        }
    }
    /// Apply the bifunctor.
    pub fn apply(
        &self,
        s: Coproduct<A, B>,
        f: impl FnOnce(A) -> C,
        g: impl FnOnce(B) -> D,
    ) -> Coproduct<C, D> {
        s.bimap(f, g)
    }
}
/// A general coproduct (sum) of two types, equivalent to `Result<A, B>` but
/// without the error/success semantics. Both sides are equally valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Coproduct<A, B> {
    /// The left injection.
    Inl(A),
    /// The right injection.
    Inr(B),
}
impl<A, B> Coproduct<A, B> {
    /// Construct `Inl a`.
    pub fn inl(a: A) -> Self {
        Coproduct::Inl(a)
    }
    /// Construct `Inr b`.
    pub fn inr(b: B) -> Self {
        Coproduct::Inr(b)
    }
    /// Return true if `Inl`.
    pub fn is_left(&self) -> bool {
        matches!(self, Coproduct::Inl(_))
    }
    /// Return true if `Inr`.
    pub fn is_right(&self) -> bool {
        matches!(self, Coproduct::Inr(_))
    }
    /// Eliminate the coproduct by providing a function for each side.
    pub fn elim<C>(self, f: impl FnOnce(A) -> C, g: impl FnOnce(B) -> C) -> C {
        match self {
            Coproduct::Inl(a) => f(a),
            Coproduct::Inr(b) => g(b),
        }
    }
    /// Swap the two sides.
    pub fn swap(self) -> Coproduct<B, A> {
        match self {
            Coproduct::Inl(a) => Coproduct::Inr(a),
            Coproduct::Inr(b) => Coproduct::Inl(b),
        }
    }
    /// Bifunctor map: apply `f` to `Inl` and `g` to `Inr`.
    pub fn bimap<C, D>(self, f: impl FnOnce(A) -> C, g: impl FnOnce(B) -> D) -> Coproduct<C, D> {
        match self {
            Coproduct::Inl(a) => Coproduct::Inl(f(a)),
            Coproduct::Inr(b) => Coproduct::Inr(g(b)),
        }
    }
    /// Map only the left side.
    pub fn map_left<C>(self, f: impl FnOnce(A) -> C) -> Coproduct<C, B> {
        match self {
            Coproduct::Inl(a) => Coproduct::Inl(f(a)),
            Coproduct::Inr(b) => Coproduct::Inr(b),
        }
    }
    /// Map only the right side.
    pub fn map_right<D>(self, g: impl FnOnce(B) -> D) -> Coproduct<A, D> {
        match self {
            Coproduct::Inl(a) => Coproduct::Inl(a),
            Coproduct::Inr(b) => Coproduct::Inr(g(b)),
        }
    }
    /// Extract the left value or return a default.
    pub fn left_or(self, default: A) -> A {
        match self {
            Coproduct::Inl(a) => a,
            Coproduct::Inr(_) => default,
        }
    }
    /// Extract the right value or return a default.
    pub fn right_or(self, default: B) -> B {
        match self {
            Coproduct::Inl(_) => default,
            Coproduct::Inr(b) => b,
        }
    }
    /// Try to extract the left value.
    pub fn into_left(self) -> Option<A> {
        match self {
            Coproduct::Inl(a) => Some(a),
            Coproduct::Inr(_) => None,
        }
    }
    /// Try to extract the right value.
    pub fn into_right(self) -> Option<B> {
        match self {
            Coproduct::Inl(_) => None,
            Coproduct::Inr(b) => Some(b),
        }
    }
    /// Convert to `Result<B, A>` (left = error, right = ok — standard `Result` convention).
    pub fn into_result(self) -> Result<B, A> {
        match self {
            Coproduct::Inl(a) => Err(a),
            Coproduct::Inr(b) => Ok(b),
        }
    }
    /// Convert from `Result<B, A>`.
    pub fn from_result(r: Result<B, A>) -> Self {
        match r {
            Ok(b) => Coproduct::Inr(b),
            Err(a) => Coproduct::Inl(a),
        }
    }
}
impl<A: Clone, B: Clone> Coproduct<A, B> {
    /// Coalesce: if both sides have the same type, return the inner value.
    pub fn merge(self) -> A
    where
        A: Clone,
        B: Into<A>,
    {
        match self {
            Coproduct::Inl(a) => a,
            Coproduct::Inr(b) => b.into(),
        }
    }
}
/// A disjoint union of four types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sum4<A, B, C, D> {
    /// First injection.
    In1(A),
    /// Second injection.
    In2(B),
    /// Third injection.
    In3(C),
    /// Fourth injection.
    In4(D),
}
impl<A, B, C, D> Sum4<A, B, C, D> {
    /// Eliminate by providing one function per variant.
    #[allow(clippy::too_many_arguments)]
    pub fn elim<E>(
        self,
        f1: impl FnOnce(A) -> E,
        f2: impl FnOnce(B) -> E,
        f3: impl FnOnce(C) -> E,
        f4: impl FnOnce(D) -> E,
    ) -> E {
        match self {
            Sum4::In1(a) => f1(a),
            Sum4::In2(b) => f2(b),
            Sum4::In3(c) => f3(c),
            Sum4::In4(d) => f4(d),
        }
    }
    /// Tag of the active variant (1-indexed).
    pub fn tag(&self) -> u8 {
        match self {
            Sum4::In1(_) => 1,
            Sum4::In2(_) => 2,
            Sum4::In3(_) => 3,
            Sum4::In4(_) => 4,
        }
    }
    /// Whether this is the first variant.
    pub fn is_first(&self) -> bool {
        matches!(self, Sum4::In1(_))
    }
    /// Whether this is the second variant.
    pub fn is_second(&self) -> bool {
        matches!(self, Sum4::In2(_))
    }
    /// Whether this is the third variant.
    pub fn is_third(&self) -> bool {
        matches!(self, Sum4::In3(_))
    }
    /// Whether this is the fourth variant.
    pub fn is_fourth(&self) -> bool {
        matches!(self, Sum4::In4(_))
    }
}
/// A mapping from tag labels to descriptions.
#[derive(Clone, Debug, Default)]
pub struct InjectionMap {
    labels: Vec<(u8, String)>,
}
impl InjectionMap {
    /// Create an empty map.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a label for a tag.
    pub fn register(&mut self, tag: u8, label: impl Into<String>) {
        self.labels.push((tag, label.into()));
    }
    /// Look up a label for a tag.
    pub fn get(&self, tag: u8) -> Option<&str> {
        self.labels
            .iter()
            .find(|(t, _)| *t == tag)
            .map(|(_, l)| l.as_str())
    }
    /// Number of registered labels.
    pub fn len(&self) -> usize {
        self.labels.len()
    }
    /// Whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}
/// A tagged union with an explicit tag for inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tagged<T, A> {
    /// The discriminant / tag.
    pub tag: T,
    /// The payload.
    pub value: A,
}
impl<T, A> Tagged<T, A> {
    /// Create a new `Tagged` value.
    pub fn new(tag: T, value: A) -> Self {
        Tagged { tag, value }
    }
    /// Map the value while preserving the tag.
    pub fn map<B>(self, f: impl FnOnce(A) -> B) -> Tagged<T, B> {
        Tagged {
            tag: self.tag,
            value: f(self.value),
        }
    }
    /// Map the tag while preserving the value.
    pub fn map_tag<U>(self, f: impl FnOnce(T) -> U) -> Tagged<U, A> {
        Tagged {
            tag: f(self.tag),
            value: self.value,
        }
    }
}
/// A tagged-union type using string discriminants.
#[allow(dead_code)]
pub struct TaggedUnionSm {
    /// The discriminant tag.
    pub tag: String,
    /// The serialized payload.
    pub payload: String,
}
impl TaggedUnionSm {
    /// Create a new tagged union value.
    pub fn new(tag: impl Into<String>, payload: impl Into<String>) -> Self {
        TaggedUnionSm {
            tag: tag.into(),
            payload: payload.into(),
        }
    }
    /// Render as a display string.
    pub fn render(&self) -> String {
        format!("{}({})", self.tag, self.payload)
    }
    /// Check the tag.
    pub fn has_tag(&self, t: &str) -> bool {
        self.tag == t
    }
}
/// A symmetric product (non-dependent pair) for use with Sum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<A, B> {
    /// First component.
    pub fst: A,
    /// Second component.
    pub snd: B,
}
impl<A, B> Pair<A, B> {
    /// Construct a `Pair`.
    pub fn new(fst: A, snd: B) -> Self {
        Pair { fst, snd }
    }
    /// Swap the pair.
    pub fn swap(self) -> Pair<B, A> {
        Pair {
            fst: self.snd,
            snd: self.fst,
        }
    }
    /// Map the first component.
    pub fn map_fst<C>(self, f: impl FnOnce(A) -> C) -> Pair<C, B> {
        Pair {
            fst: f(self.fst),
            snd: self.snd,
        }
    }
    /// Map the second component.
    pub fn map_snd<D>(self, g: impl FnOnce(B) -> D) -> Pair<A, D> {
        Pair {
            fst: self.fst,
            snd: g(self.snd),
        }
    }
    /// Convert to a tuple.
    pub fn into_tuple(self) -> (A, B) {
        (self.fst, self.snd)
    }
    /// Convert from a tuple.
    pub fn from_tuple(t: (A, B)) -> Self {
        Pair { fst: t.0, snd: t.1 }
    }
}
/// Partition result for Either (Sum) type, holding separated lefts and rights.
#[allow(dead_code)]
pub struct EitherPartitionSm<L, R> {
    /// All left values.
    pub lefts: Vec<L>,
    /// All right values.
    pub rights: Vec<R>,
}
impl<L, R> EitherPartitionSm<L, R> {
    /// Create from a vector of Coproducts.
    pub fn from_vec(xs: Vec<Coproduct<L, R>>) -> Self {
        let (ls, rs) = partition(xs);
        EitherPartitionSm {
            lefts: ls,
            rights: rs,
        }
    }
    /// Total number of elements.
    pub fn total(&self) -> usize {
        self.lefts.len() + self.rights.len()
    }
    /// True if there are no left values.
    pub fn all_right(&self) -> bool {
        self.lefts.is_empty()
    }
    /// True if there are no right values.
    pub fn all_left(&self) -> bool {
        self.rights.is_empty()
    }
    /// Left count.
    pub fn left_count(&self) -> usize {
        self.lefts.len()
    }
    /// Right count.
    pub fn right_count(&self) -> usize {
        self.rights.len()
    }
}
/// A traversal of a Sum type: applies an effectful function over the right side.
#[allow(dead_code)]
pub struct SumTraversal<A, B> {
    /// Phantom for left type.
    _a: std::marker::PhantomData<A>,
    /// Phantom for right type.
    _b: std::marker::PhantomData<B>,
    /// Description of the traversal.
    pub description: String,
}
impl<A, B> SumTraversal<A, B> {
    /// Create a new SumTraversal.
    pub fn new(description: impl Into<String>) -> Self {
        SumTraversal {
            _a: std::marker::PhantomData,
            _b: std::marker::PhantomData,
            description: description.into(),
        }
    }
    /// Traverse: apply `f` to the right side, preserving left.
    pub fn traverse_option<C>(
        &self,
        s: Coproduct<A, B>,
        f: impl FnOnce(B) -> Option<C>,
    ) -> Option<Coproduct<A, C>> {
        match s {
            Coproduct::Inl(a) => Some(Coproduct::Inl(a)),
            Coproduct::Inr(b) => f(b).map(Coproduct::Inr),
        }
    }
}
/// Witness of the universal property of Sum: given `f: A → Z` and `g: B → Z`,
/// there is a unique mediating map `Sum A B → Z`.
#[allow(dead_code)]
pub struct SumUniversal<A, B, Z> {
    /// The left branch.
    pub left_branch: std::marker::PhantomData<fn(A) -> Z>,
    /// The right branch.
    pub right_branch: std::marker::PhantomData<fn(B) -> Z>,
    /// Description of the universal property.
    pub description: String,
}
impl<A, B, Z> SumUniversal<A, B, Z> {
    /// Create a new SumUniversal witness.
    pub fn new(description: impl Into<String>) -> Self {
        SumUniversal {
            left_branch: std::marker::PhantomData,
            right_branch: std::marker::PhantomData,
            description: description.into(),
        }
    }
    /// Apply the universal mediator using concrete functions.
    pub fn mediate(&self, s: Coproduct<A, B>, f: impl FnOnce(A) -> Z, g: impl FnOnce(B) -> Z) -> Z {
        s.elim(f, g)
    }
}
/// A coproduct of three types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sum3<A, B, C> {
    /// First injection.
    In1(A),
    /// Second injection.
    In2(B),
    /// Third injection.
    In3(C),
}
impl<A, B, C> Sum3<A, B, C> {
    /// Eliminate by providing one function per variant.
    pub fn elim<D>(
        self,
        f1: impl FnOnce(A) -> D,
        f2: impl FnOnce(B) -> D,
        f3: impl FnOnce(C) -> D,
    ) -> D {
        match self {
            Sum3::In1(a) => f1(a),
            Sum3::In2(b) => f2(b),
            Sum3::In3(c) => f3(c),
        }
    }
    /// Return true if this is the first variant.
    pub fn is_first(&self) -> bool {
        matches!(self, Sum3::In1(_))
    }
    /// Return true if this is the second variant.
    pub fn is_second(&self) -> bool {
        matches!(self, Sum3::In2(_))
    }
    /// Return true if this is the third variant.
    pub fn is_third(&self) -> bool {
        matches!(self, Sum3::In3(_))
    }
}
