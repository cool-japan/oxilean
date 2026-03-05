//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::fmt;

/// Continuation monad: ContM r a = (a -> r) -> r
#[allow(dead_code)]
pub struct ContM<R, A> {
    pub(super) run_cont: Box<dyn FnOnce(Box<dyn FnOnce(A) -> R>) -> R>,
}
/// Codensity monad: CPS-transformed monad for efficiency.
/// Codensity m a = forall r. (a -> m r) -> m r
#[allow(dead_code)]
pub struct CodensityM<M, A> {
    run_codensity: Box<dyn FnOnce(Box<dyn FnOnce(A) -> M>) -> M>,
}
/// A simple `Maybe` monad (equivalent to `Option`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Maybe<A>(pub Option<A>);
impl<A> Maybe<A> {
    /// Wrap a value as `Just a`.
    pub fn just(a: A) -> Self {
        Maybe(Some(a))
    }
    /// The `Nothing` constructor.
    pub fn nothing() -> Self {
        Maybe(None)
    }
    /// Return true if this is `Nothing`.
    pub fn is_nothing(&self) -> bool {
        self.0.is_none()
    }
    /// Return true if this is `Just _`.
    pub fn is_just(&self) -> bool {
        self.0.is_some()
    }
    /// Monadic bind.
    pub fn bind<B>(self, f: impl FnOnce(A) -> Maybe<B>) -> Maybe<B> {
        match self.0 {
            Some(a) => f(a),
            None => Maybe(None),
        }
    }
    /// Functor map.
    pub fn fmap<B>(self, f: impl FnOnce(A) -> B) -> Maybe<B> {
        Maybe(self.0.map(f))
    }
    /// Return the inner `Option`.
    pub fn into_option(self) -> Option<A> {
        self.0
    }
    /// Alternative: use `other` if `self` is `Nothing`.
    pub fn or_else(self, other: Maybe<A>) -> Maybe<A> {
        if self.is_just() {
            self
        } else {
            other
        }
    }
}
/// A simple identity monad: wraps a value with no effects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity<A> {
    /// The wrapped value.
    pub value: A,
}
impl<A> Identity<A> {
    /// Wrap a value.
    pub fn pure(a: A) -> Self {
        Identity { value: a }
    }
    /// Bind: apply a function to the contained value.
    pub fn bind<B>(self, f: impl FnOnce(A) -> Identity<B>) -> Identity<B> {
        f(self.value)
    }
    /// Map: apply a function to the contained value.
    pub fn map<B>(self, f: impl FnOnce(A) -> B) -> Identity<B> {
        Identity {
            value: f(self.value),
        }
    }
    /// Extract the value.
    pub fn run(self) -> A {
        self.value
    }
}
/// Free monad: Free f a = Pure a | Free (f (Free f a))
/// Represents computations as data for interpretation.
#[allow(dead_code)]
pub enum FreeM<A> {
    /// Pure value in the free monad.
    Pure(A),
    /// Suspended computation wrapped in an effect layer.
    Free(Box<FreeM<A>>),
}
/// A `Reader` monad: computation depending on a shared environment `R`.
pub struct Reader<R, A> {
    run_fn: Box<dyn FnOnce(&R) -> A>,
}
impl<R: 'static, A: 'static> Reader<R, A> {
    /// Create a `Reader` from a function.
    pub fn new(f: impl FnOnce(&R) -> A + 'static) -> Self {
        Reader {
            run_fn: Box::new(f),
        }
    }
    /// Lift a pure value.
    pub fn pure(a: A) -> Self
    where
        A: Clone,
    {
        Reader::new(move |_| a.clone())
    }
    /// Monadic bind.
    pub fn bind<B: 'static>(self, f: impl FnOnce(A) -> Reader<R, B> + 'static) -> Reader<R, B> {
        Reader::new(move |r| {
            let a = (self.run_fn)(r);
            (f(a).run_fn)(r)
        })
    }
    /// Map over the result.
    pub fn map<B: 'static>(self, f: impl FnOnce(A) -> B + 'static) -> Reader<R, B> {
        Reader::new(move |r| f((self.run_fn)(r)))
    }
    /// Run the computation with an environment.
    pub fn run(self, env: &R) -> A {
        (self.run_fn)(env)
    }
    /// Ask for the full environment.
    pub fn ask() -> Reader<R, R>
    where
        R: Clone,
    {
        Reader::new(|r: &R| r.clone())
    }
    /// Ask for a projection of the environment.
    pub fn asks<B: 'static>(f: impl FnOnce(&R) -> B + 'static) -> Reader<R, B> {
        Reader::new(f)
    }
}
/// A `Writer` monad: accumulates a log alongside a value.
#[derive(Debug, Clone)]
pub struct Writer<W, A> {
    /// The produced value.
    pub value: A,
    /// The accumulated log.
    pub log: W,
}
impl<W: Default + Extend<W::Item>, A> Writer<W, A>
where
    W: IntoIterator + Clone,
{
    /// Lift a pure value with an empty log.
    pub fn pure(a: A) -> Self
    where
        W: Default,
    {
        Writer {
            value: a,
            log: W::default(),
        }
    }
}
impl<A> Writer<Vec<String>, A> {
    /// Create a writer with a value and initial log entries.
    pub fn new(value: A, log: Vec<String>) -> Self {
        Writer { value, log }
    }
    /// Monadic bind: combine logs.
    pub fn bind<B>(self, f: impl FnOnce(A) -> Writer<Vec<String>, B>) -> Writer<Vec<String>, B> {
        let Writer {
            value: a,
            log: log1,
        } = self;
        let mut log1 = log1;
        let Writer {
            value: b,
            log: log2,
        } = f(a);
        log1.extend(log2);
        Writer {
            value: b,
            log: log1,
        }
    }
    /// Map over the value.
    pub fn fmap<B>(self, f: impl FnOnce(A) -> B) -> Writer<Vec<String>, B> {
        Writer {
            value: f(self.value),
            log: self.log,
        }
    }
    /// Emit a single log message.
    pub fn tell(msg: String) -> Writer<Vec<String>, ()> {
        Writer {
            value: (),
            log: vec![msg],
        }
    }
    /// Return only the log.
    pub fn into_log(self) -> Vec<String> {
        self.log
    }
    /// Return only the value.
    pub fn into_value(self) -> A {
        self.value
    }
}
/// Indexed state monad: IxState i j a = i -> (a, j)
/// Tracks index transition from i to j while producing a.
#[allow(dead_code)]
pub struct IxState<I, J, A> {
    pub(super) run_ix_state: Box<dyn FnOnce(I) -> (A, J)>,
}
/// A simple `Either` monad with error type `E`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Either<E, A> {
    inner: Result<A, E>,
}
impl<E, A> Either<E, A> {
    /// Construct a `Right a` (success).
    pub fn right(a: A) -> Self {
        Either { inner: Ok(a) }
    }
    /// Construct a `Left e` (error).
    pub fn left(e: E) -> Self {
        Either { inner: Err(e) }
    }
    /// Return true if `Right`.
    pub fn is_right(&self) -> bool {
        self.inner.is_ok()
    }
    /// Return true if `Left`.
    pub fn is_left(&self) -> bool {
        self.inner.is_err()
    }
    /// Monadic bind on the `Right` side.
    pub fn bind<B>(self, f: impl FnOnce(A) -> Either<E, B>) -> Either<E, B> {
        match self.inner {
            Ok(a) => f(a),
            Err(e) => Either::left(e),
        }
    }
    /// Map over the `Right` side.
    pub fn fmap<B>(self, f: impl FnOnce(A) -> B) -> Either<E, B> {
        Either {
            inner: self.inner.map(f),
        }
    }
    /// Map over the `Left` side.
    pub fn map_left<F2>(self, f: impl FnOnce(E) -> F2) -> Either<F2, A> {
        Either {
            inner: self.inner.map_err(f),
        }
    }
    /// Convert to `Result`.
    pub fn into_result(self) -> Result<A, E> {
        self.inner
    }
    /// Extract the right value or panic.
    pub fn unwrap_right(self) -> A
    where
        E: std::fmt::Debug,
    {
        self.inner
            .expect("Either::unwrap_right called on Left variant")
    }
}
/// A `State` monad: transforms state `S` into `(A, S)`.
pub struct State<S, A> {
    run_fn: Box<dyn FnOnce(S) -> (A, S)>,
}
impl<S: 'static, A: 'static> State<S, A> {
    /// Create a `State` from a state transformation function.
    pub fn new(f: impl FnOnce(S) -> (A, S) + 'static) -> Self {
        State {
            run_fn: Box::new(f),
        }
    }
    /// Lift a pure value into the `State` monad.
    pub fn pure(a: A) -> Self
    where
        A: Clone,
    {
        State::new(move |s| (a.clone(), s))
    }
    /// Monadic bind.
    pub fn bind<B: 'static>(self, f: impl FnOnce(A) -> State<S, B> + 'static) -> State<S, B> {
        State::new(move |s| {
            let (a, s2) = (self.run_fn)(s);
            (f(a).run_fn)(s2)
        })
    }
    /// Map over the result value.
    pub fn map<B: 'static>(self, f: impl FnOnce(A) -> B + 'static) -> State<S, B> {
        State::new(move |s| {
            let (a, s2) = (self.run_fn)(s);
            (f(a), s2)
        })
    }
    /// Run the state computation with an initial state.
    pub fn run(self, initial: S) -> (A, S) {
        (self.run_fn)(initial)
    }
    /// Run and return only the value.
    pub fn eval(self, initial: S) -> A {
        self.run(initial).0
    }
    /// Run and return only the final state.
    pub fn exec(self, initial: S) -> S {
        self.run(initial).1
    }
    /// Read the current state.
    pub fn get() -> State<S, S>
    where
        S: Clone,
    {
        State::new(|s: S| {
            let s2 = s.clone();
            (s2, s)
        })
    }
    /// Set a new state.
    pub fn put(new_s: S) -> State<S, ()>
    where
        S: 'static,
    {
        State::new(move |_| ((), new_s))
    }
    /// Modify the state with a function.
    pub fn modify(f: impl FnOnce(S) -> S + 'static) -> State<S, ()> {
        State::new(move |s| ((), f(s)))
    }
}
/// Arrow abstraction generalizing functions with effects.
/// arr :: (a -> b) -> f a b
#[allow(dead_code)]
pub struct ArrowF<A, B> {
    pub(super) run_arrow: Box<dyn FnOnce(A) -> B>,
}
