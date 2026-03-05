//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Vec functor wrapper.
pub struct VecFunctor<A>(pub Vec<A>);
/// A `Writer` monad: a computation that produces a log alongside a value.
///
/// Wraps a pair `(A, W)` where `W` is a monoid.
pub struct Writer<A, W> {
    value: A,
    log: W,
}
impl<A, W: Default + Clone> Writer<A, W> {
    /// Create a writer with the given value and log.
    pub fn new(value: A, log: W) -> Self {
        Self { value, log }
    }
    /// Return a writer that writes nothing.
    pub fn pure(value: A) -> Self {
        Self {
            value,
            log: W::default(),
        }
    }
    /// Extract the value.
    pub fn get_value(self) -> A {
        self.value
    }
    /// Extract the log.
    pub fn get_log(&self) -> &W {
        &self.log
    }
    /// Run the writer, producing `(value, log)`.
    pub fn run(self) -> (A, W) {
        (self.value, self.log)
    }
}
/// Day convolution of two functors F and G over A.
#[allow(dead_code)]
pub struct DayConvolution<F, G, A> {
    /// Left functor value.
    pub left: F,
    /// Right functor value.
    pub right: G,
    /// Combined value type marker.
    pub _phantom: std::marker::PhantomData<A>,
}
/// Representable functor extension: tabulate/index.
#[allow(dead_code)]
pub struct RepresentableFunctorExt<A> {
    /// Tabulated values indexed by usize.
    pub table: Vec<A>,
}
impl<A: Clone> RepresentableFunctorExt<A> {
    /// Create from a tabulating function.
    pub fn tabulate(n: usize, f: impl Fn(usize) -> A) -> Self {
        Self {
            table: (0..n).map(f).collect(),
        }
    }
    /// Index into the representable functor.
    pub fn index(&self, i: usize) -> Option<&A> {
        self.table.get(i)
    }
}
/// Result functor wrapper.
pub struct ResultFunctor<A, E>(pub Result<A, E>);
/// A `Reader` monad: a computation that depends on an environment `E`.
///
/// Wraps a function `E -> A`.
pub struct Reader<E, A> {
    run: Box<dyn Fn(E) -> A>,
}
impl<E: Clone + 'static, A: 'static> Reader<E, A> {
    /// Create a reader from a function.
    pub fn new(f: impl Fn(E) -> A + 'static) -> Self {
        Self { run: Box::new(f) }
    }
    /// Run the reader with the given environment.
    pub fn run_reader(self, env: E) -> A {
        (self.run)(env)
    }
    /// Map a function over the result.
    pub fn map<B: 'static>(self, f: impl Fn(A) -> B + 'static) -> Reader<E, B> {
        let run = self.run;
        Reader::new(move |env: E| f(run(env)))
    }
    /// Create a reader that ignores its environment and returns `a`.
    pub fn pure(a: A) -> Self
    where
        A: Clone,
    {
        Reader::new(move |_| a.clone())
    }
}
/// Contravariant functor wrapper (extended).
#[allow(dead_code)]
pub struct ContravariantFunctor<A> {
    /// Internal predicate function.
    pub predicate: Box<dyn Fn(A) -> bool>,
}
/// Predicate as contravariant functor.
pub struct Pred<A>(pub Box<dyn Fn(A) -> bool>);
impl<A> Pred<A> {
    /// Create from closure.
    pub fn new(f: impl Fn(A) -> bool + 'static) -> Self {
        Pred(Box::new(f))
    }
    /// Test the predicate.
    pub fn test(&self, a: A) -> bool {
        (self.0)(a)
    }
}
impl<A: 'static> Pred<A> {
    /// Contramap: apply f before the predicate.
    pub fn contramap<B, F: Fn(B) -> A + 'static>(self, f: F) -> Pred<B> {
        Pred(Box::new(move |b| (self.0)(f(b))))
    }
}
/// Profunctor composition (A -> B -> C).
#[allow(dead_code)]
pub struct ProfunctorCompose<A, B, C> {
    /// Left leg: A -> B.
    pub left: Box<dyn Fn(A) -> B>,
    /// Right leg: B -> C.
    pub right: Box<dyn Fn(B) -> C>,
}
/// Either type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Either<A, B> {
    /// Left variant.
    Left(A),
    /// Right variant.
    Right(B),
}
impl<A, B> Either<A, B> {
    /// Is Left?
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    /// Is Right?
    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
    /// Unwrap left.
    pub fn unwrap_left(self) -> A {
        match self {
            Either::Left(a) => a,
            _ => panic!("Right"),
        }
    }
    /// Unwrap right.
    pub fn unwrap_right(self) -> B {
        match self {
            Either::Right(b) => b,
            _ => panic!("Left"),
        }
    }
    /// Map left side.
    pub fn map_left<C, F: FnOnce(A) -> C>(self, f: F) -> Either<C, B> {
        match self {
            Either::Left(a) => Either::Left(f(a)),
            Either::Right(b) => Either::Right(b),
        }
    }
    /// Map right side.
    pub fn map_right<D, G: FnOnce(B) -> D>(self, g: G) -> Either<A, D> {
        match self {
            Either::Left(a) => Either::Left(a),
            Either::Right(b) => Either::Right(g(b)),
        }
    }
    /// Map both sides.
    pub fn bimap<C, D, F: FnOnce(A) -> C, G: FnOnce(B) -> D>(self, f: F, g: G) -> Either<C, D> {
        match self {
            Either::Left(a) => Either::Left(f(a)),
            Either::Right(b) => Either::Right(g(b)),
        }
    }
    /// Convert to Option<A>.
    pub fn left(self) -> Option<A> {
        match self {
            Either::Left(a) => Some(a),
            _ => None,
        }
    }
    /// Convert to Option<B>.
    pub fn right(self) -> Option<B> {
        match self {
            Either::Right(b) => Some(b),
            _ => None,
        }
    }
}
/// Option functor wrapper.
pub struct OptionFunctor<A>(pub Option<A>);
/// Composition of two functors F and G over A.
#[allow(dead_code)]
pub struct FunctorCompose<F, G, A> {
    /// Outer functor.
    pub outer: F,
    /// Inner functor.
    pub inner: G,
    /// Value phantom.
    pub _phantom: std::marker::PhantomData<A>,
}
