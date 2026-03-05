//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// A monad transformer wrapper representing `M (Either E A)`.
///
/// `EitherTMonad` is a thin Rust-level encoding of the EitherT transformer,
/// holding the inner monadic value as an `OxiEither<A, E>` wrapped in a `Vec`
/// (as a stand-in for a generic monad `M`).
#[allow(dead_code)]
pub struct EitherTMonad<M, E, A> {
    /// The inner value, represented as `M (Either E A)`.
    /// We use `Vec` as a concrete stand-in for the monad `M`.
    pub inner: Vec<OxiEither<A, E>>,
    /// Phantom to hold the monad type parameter.
    _phantom: std::marker::PhantomData<M>,
}
#[allow(dead_code)]
impl<M, E: Clone, A: Clone> EitherTMonad<M, E, A> {
    /// Wrap a single `OxiEither` value into the transformer.
    pub fn new(value: OxiEither<A, E>) -> Self {
        Self {
            inner: vec![value],
            _phantom: std::marker::PhantomData,
        }
    }
    /// Lift a pure value into the transformer (right-inject).
    pub fn pure(a: A) -> Self {
        Self::new(OxiEither::Left(a))
    }
    /// Fail with an error (left-inject).
    pub fn throw(e: E) -> Self {
        Self::new(OxiEither::Right(e))
    }
    /// Run the transformer, extracting the inner value.
    pub fn run(self) -> Option<OxiEither<A, E>> {
        self.inner.into_iter().next()
    }
    /// Bind over the transformer.
    pub fn bind<B: Clone, F>(self, f: F) -> EitherTMonad<M, E, B>
    where
        F: FnOnce(A) -> EitherTMonad<M, E, B>,
    {
        match self.run() {
            Some(OxiEither::Left(a)) => f(a),
            Some(OxiEither::Right(e)) => EitherTMonad::<M, E, B>::throw(e),
            None => EitherTMonad {
                inner: vec![],
                _phantom: std::marker::PhantomData,
            },
        }
    }
    /// Map over the success value.
    pub fn map<B: Clone, F: FnOnce(A) -> B>(self, f: F) -> EitherTMonad<M, E, B> {
        match self.run() {
            Some(OxiEither::Left(a)) => EitherTMonad::<M, E, B>::new(OxiEither::Left(f(a))),
            Some(OxiEither::Right(e)) => EitherTMonad::<M, E, B>::new(OxiEither::Right(e)),
            None => EitherTMonad {
                inner: vec![],
                _phantom: std::marker::PhantomData,
            },
        }
    }
}
/// A Kleisli arrow `A → Either E B` in the Either monad's Kleisli category.
#[allow(dead_code)]
pub struct EitherKleisli<E, A, B> {
    /// The underlying function from `A` to `OxiEither<B, E>`.
    pub run: Box<dyn Fn(A) -> OxiEither<B, E>>,
}
#[allow(dead_code)]
impl<E: 'static, A: 'static, B: 'static> EitherKleisli<E, A, B> {
    /// Construct a new Kleisli arrow from a function.
    pub fn new<F: Fn(A) -> OxiEither<B, E> + 'static>(f: F) -> Self {
        Self { run: Box::new(f) }
    }
    /// Apply the Kleisli arrow to an input.
    pub fn apply(&self, a: A) -> OxiEither<B, E> {
        (self.run)(a)
    }
    /// Compose two Kleisli arrows: (f >=> g)
    pub fn compose<C: 'static>(self, g: EitherKleisli<E, B, C>) -> EitherKleisli<E, A, C>
    where
        E: Clone + 'static,
    {
        EitherKleisli::new(move |a: A| match (self.run)(a) {
            OxiEither::Left(b) => (g.run)(b),
            OxiEither::Right(e) => OxiEither::Right(e),
        })
    }
    /// Lift a plain function into the Kleisli category (i.e., return ∘ f).
    pub fn lift_fn<F: Fn(A) -> B + 'static>(f: F) -> Self {
        Self::new(move |a| OxiEither::Left(f(a)))
    }
}
/// The select combinator for `Either`, implementing `Selective` behaviour.
///
/// `select` runs a computation that may short-circuit or choose between two
/// branches. Given `OxiEither<A, E>` (lhs) and `OxiEither<Box<dyn Fn(A) -> B>, E>` (rhs),
/// it applies the function when lhs is `Left(a)`, otherwise short-circuits.
#[allow(dead_code)]
pub struct SelectCombinator<E, A, B> {
    /// The left-or-value argument.
    pub lhs: OxiEither<A, E>,
    /// The function-or-error argument.
    pub rhs: OxiEither<Box<dyn Fn(A) -> B>, E>,
}
#[allow(dead_code)]
impl<E: Clone, A, B> SelectCombinator<E, A, B> {
    /// Construct a new select combinator.
    pub fn new(lhs: OxiEither<A, E>, rhs: OxiEither<Box<dyn Fn(A) -> B>, E>) -> Self {
        Self { lhs, rhs }
    }
    /// Run the select combinator, returning `OxiEither<B, E>`.
    pub fn select(self) -> OxiEither<B, E> {
        match self.lhs {
            OxiEither::Left(a) => match self.rhs {
                OxiEither::Left(f) => OxiEither::Left(f(a)),
                OxiEither::Right(e) => OxiEither::Right(e),
            },
            OxiEither::Right(e) => OxiEither::Right(e),
        }
    }
    /// Witness of: select (Right e) _ = Right e
    pub fn select_right_law(e: E) -> OxiEither<B, E> {
        OxiEither::Right(e)
    }
}
/// An iterator that yields the left value of an `OxiEither` once.
pub struct EitherLeftIter<A: Clone, B> {
    pub(super) inner: Option<OxiEither<A, B>>,
}
impl<A: Clone, B> EitherLeftIter<A, B> {
    /// Create from an either value.
    pub fn new(e: OxiEither<A, B>) -> Self {
        Self { inner: Some(e) }
    }
}
/// A Rust-level sum type representing one of two values.
/// Used by iterator utilities and functional combinators in this module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OxiEither<A, B> {
    /// The left variant (analogous to `Either.inl`).
    Left(A),
    /// The right variant (analogous to `Either.inr`).
    Right(B),
}
impl<A, B> OxiEither<A, B> {
    /// Check if this is a `Left` value.
    pub fn is_left(&self) -> bool {
        matches!(self, OxiEither::Left(_))
    }
    /// Check if this is a `Right` value.
    pub fn is_right(&self) -> bool {
        matches!(self, OxiEither::Right(_))
    }
    /// Map over the `Left` value.
    pub fn map_left<C, F: FnOnce(A) -> C>(self, f: F) -> OxiEither<C, B> {
        match self {
            OxiEither::Left(a) => OxiEither::Left(f(a)),
            OxiEither::Right(b) => OxiEither::Right(b),
        }
    }
    /// Map over the `Right` value.
    pub fn map_right<C, F: FnOnce(B) -> C>(self, f: F) -> OxiEither<A, C> {
        match self {
            OxiEither::Left(a) => OxiEither::Left(a),
            OxiEither::Right(b) => OxiEither::Right(f(b)),
        }
    }
    /// Fold into a single value using two functions.
    pub fn fold<C, FL: FnOnce(A) -> C, FR: FnOnce(B) -> C>(self, fl: FL, fr: FR) -> C {
        match self {
            OxiEither::Left(a) => fl(a),
            OxiEither::Right(b) => fr(b),
        }
    }
    /// Swap left and right.
    pub fn swap(self) -> OxiEither<B, A> {
        match self {
            OxiEither::Left(a) => OxiEither::Right(a),
            OxiEither::Right(b) => OxiEither::Left(b),
        }
    }
    /// Extract left value or use a default.
    pub fn left_or(self, default: A) -> A {
        match self {
            OxiEither::Left(a) => a,
            OxiEither::Right(_) => default,
        }
    }
    /// Extract right value or use a default.
    pub fn right_or(self, default: B) -> B {
        match self {
            OxiEither::Left(_) => default,
            OxiEither::Right(b) => b,
        }
    }
    /// Convert to Option of left.
    pub fn left(self) -> Option<A> {
        match self {
            OxiEither::Left(a) => Some(a),
            OxiEither::Right(_) => None,
        }
    }
    /// Convert to Option of right.
    pub fn right(self) -> Option<B> {
        match self {
            OxiEither::Left(_) => None,
            OxiEither::Right(b) => Some(b),
        }
    }
    /// Borrow the left value as an `Option<&A>`.
    pub fn as_left(&self) -> Option<&A> {
        match self {
            OxiEither::Left(a) => Some(a),
            OxiEither::Right(_) => None,
        }
    }
    /// Borrow the right value as an `Option<&B>`.
    pub fn as_right(&self) -> Option<&B> {
        match self {
            OxiEither::Left(_) => None,
            OxiEither::Right(b) => Some(b),
        }
    }
}
impl<T> OxiEither<T, T> {
    /// Merge both sides using a function.
    pub fn merge<F: FnOnce(T) -> T>(self, f: F) -> T {
        match self {
            OxiEither::Left(a) | OxiEither::Right(a) => f(a),
        }
    }
    /// Extract the value regardless of which side it's on.
    pub fn into_inner(self) -> T {
        match self {
            OxiEither::Left(a) | OxiEither::Right(a) => a,
        }
    }
}
/// An iterator that yields the right value of an `OxiEither` once.
pub struct EitherRightIter<A, B> {
    pub(super) inner: Option<OxiEither<A, B>>,
}
impl<A, B: Clone> EitherRightIter<A, B> {
    /// Create from an either value.
    pub fn new(e: OxiEither<A, B>) -> Self {
        Self { inner: Some(e) }
    }
}
/// A partition result holding separated Left and Right values from a collection.
#[allow(dead_code)]
pub struct EitherPartition<L, R> {
    /// All Left values collected from the input.
    pub lefts: Vec<L>,
    /// All Right values collected from the input.
    pub rights: Vec<R>,
}
#[allow(dead_code)]
impl<L, R> EitherPartition<L, R> {
    /// Partition an iterator of `OxiEither` values into lefts and rights.
    pub fn from_iter<I: IntoIterator<Item = OxiEither<L, R>>>(iter: I) -> Self {
        let mut lefts = Vec::new();
        let mut rights = Vec::new();
        for item in iter {
            match item {
                OxiEither::Left(l) => lefts.push(l),
                OxiEither::Right(r) => rights.push(r),
            }
        }
        Self { lefts, rights }
    }
    /// Returns the total count of collected items.
    pub fn total(&self) -> usize {
        self.lefts.len() + self.rights.len()
    }
    /// Returns true if there are no Left values.
    pub fn no_lefts(&self) -> bool {
        self.lefts.is_empty()
    }
    /// Returns true if there are no Right values.
    pub fn no_rights(&self) -> bool {
        self.rights.is_empty()
    }
    /// Returns the ratio of lefts to total (0.0 if empty).
    pub fn left_ratio(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            self.lefts.len() as f64 / total as f64
        }
    }
}
/// Iterator over the `Right` values in a collection.
pub struct RightIter<A, B, I: Iterator<Item = OxiEither<A, B>>> {
    pub(super) inner: I,
}
/// Iterator over the `Left` values in a collection.
pub struct LeftIter<A, B, I: Iterator<Item = OxiEither<A, B>>> {
    pub(super) inner: I,
}
/// A triple sum type (three possible values).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TripleSum<A, B, C> {
    /// First variant.
    First(A),
    /// Second variant.
    Second(B),
    /// Third variant.
    Third(C),
}
impl<A, B, C> TripleSum<A, B, C> {
    /// Convert to `OxiEither<A, OxiEither<B, C>>`.
    pub fn to_nested(self) -> OxiEither<A, OxiEither<B, C>> {
        match self {
            TripleSum::First(a) => OxiEither::Left(a),
            TripleSum::Second(b) => OxiEither::Right(OxiEither::Left(b)),
            TripleSum::Third(c) => OxiEither::Right(OxiEither::Right(c)),
        }
    }
    /// Check which variant is present.
    pub fn is_first(&self) -> bool {
        matches!(self, TripleSum::First(_))
    }
    /// Check which variant is present.
    pub fn is_second(&self) -> bool {
        matches!(self, TripleSum::Second(_))
    }
    /// Check which variant is present.
    pub fn is_third(&self) -> bool {
        matches!(self, TripleSum::Third(_))
    }
}
/// A traversal context for applying an Either-returning function across a collection.
#[allow(dead_code)]
pub struct EitherTraversal<A, B> {
    /// The accumulated successful (Left in OxiEither) results so far.
    pub successes: Vec<B>,
    /// The first error encountered, if any.
    pub error: Option<A>,
}
#[allow(dead_code)]
impl<A, B> EitherTraversal<A, B> {
    /// Create a new, empty traversal context.
    pub fn new() -> Self {
        Self {
            successes: Vec::new(),
            error: None,
        }
    }
    /// Step the traversal with a new Either value.
    /// Once an error is set, subsequent values are ignored.
    pub fn step(&mut self, value: OxiEither<B, A>) {
        if self.error.is_some() {
            return;
        }
        match value {
            OxiEither::Left(b) => self.successes.push(b),
            OxiEither::Right(a) => self.error = Some(a),
        }
    }
    /// Finalise the traversal, consuming self.
    /// Returns `Left(successes)` if no error occurred, or `Right(error)`.
    pub fn finish(self) -> OxiEither<Vec<B>, A> {
        match self.error {
            None => OxiEither::Left(self.successes),
            Some(e) => OxiEither::Right(e),
        }
    }
    /// Returns true if traversal ended with an error.
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    /// Returns the count of accumulated successes.
    pub fn success_count(&self) -> usize {
        self.successes.len()
    }
}
/// An `EitherIter` yields one element: the single value in an `OxiEither`.
#[derive(Clone, Debug)]
pub struct EitherIter<A, B> {
    inner: OxiEither<A, B>,
    done: bool,
}
impl<A: Clone, B: Clone> EitherIter<A, B> {
    /// Create an iterator from an `OxiEither`.
    pub fn new(e: OxiEither<A, B>) -> Self {
        Self {
            inner: e,
            done: false,
        }
    }
}
