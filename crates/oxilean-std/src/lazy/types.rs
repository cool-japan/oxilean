//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::cell::OnceCell;
use std::sync::{Arc, OnceLock};

use super::functions::*;
use std::collections::VecDeque;

/// A lazy list (stream) node — a potentially infinite structure.
pub enum LazyList<A> {
    /// The empty stream.
    Nil,
    /// A head element and a deferred tail.
    Cons(A, Box<dyn FnOnce() -> LazyList<A>>),
}
impl<A: Clone> LazyList<A> {
    /// Create a `Nil` stream.
    pub fn nil() -> Self {
        LazyList::Nil
    }
    /// Create a `Cons` node.
    pub fn cons(head: A, tail: impl FnOnce() -> LazyList<A> + 'static) -> Self {
        LazyList::Cons(head, Box::new(tail))
    }
    /// Force the stream and collect the first `n` elements.
    pub fn take(self, n: usize) -> Vec<A> {
        let mut result = Vec::with_capacity(n);
        let mut current = self;
        while result.len() < n {
            match current {
                LazyList::Nil => break,
                LazyList::Cons(h, t) => {
                    result.push(h);
                    current = t();
                }
            }
        }
        result
    }
    /// Check if the stream is empty.
    pub fn is_nil(&self) -> bool {
        matches!(self, LazyList::Nil)
    }
}
/// A guarded recursive definition builder.
///
/// Implements Löb's principle: if we can build `A` assuming we already have
/// a `▶A` (a "later" version), then we can produce `A` unconditionally.
#[allow(dead_code)]
pub struct GuardedFix<A> {
    cell: OnceCell<A>,
}
#[allow(dead_code)]
impl<A: Clone + 'static> GuardedFix<A> {
    /// Create a guarded fixpoint by providing a step function.
    ///
    /// The step function receives a thunk for the recursive call.
    pub fn evaluate(step: impl Fn(Box<dyn Fn() -> A>) -> A) -> A {
        let placeholder: std::cell::RefCell<Option<A>> = std::cell::RefCell::new(None);
        let result = step(Box::new(move || {
            placeholder
                .borrow()
                .clone()
                .expect("guarded: value not yet available")
        }));
        result
    }
    /// Check if the cell has been forced.
    pub fn is_computed(&self) -> bool {
        self.cell.get().is_some()
    }
}
/// A thread-safe memoized value. The computation runs exactly once across threads.
pub struct Memo<A> {
    pub(super) lock: OnceLock<A>,
    init: Arc<dyn Fn() -> A + Send + Sync>,
}
impl<A: Send + Sync + 'static> Memo<A> {
    /// Create a new `Memo` with a given initialization function.
    pub fn new(f: impl Fn() -> A + Send + Sync + 'static) -> Self {
        Memo {
            lock: OnceLock::new(),
            init: Arc::new(f),
        }
    }
    /// Get the memoized value, computing it if necessary.
    pub fn get(&self) -> &A {
        self.lock.get_or_init(|| (self.init)())
    }
    /// Return true if already computed.
    pub fn is_initialized(&self) -> bool {
        self.lock.get().is_some()
    }
}
/// A batched lazy evaluator that processes multiple `Deferred<A>` values.
///
/// Useful when you have a list of deferred computations and want to force
/// them all at once in a controlled manner.
pub struct LazyBatch<A> {
    items: Vec<Deferred<A>>,
}
impl<A: 'static> LazyBatch<A> {
    /// Create an empty batch.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Add a deferred item to the batch.
    pub fn push(mut self, item: Deferred<A>) -> Self {
        self.items.push(item);
        self
    }
    /// Force all items and collect results.
    pub fn force_all(self) -> Vec<A> {
        self.items.into_iter().map(|d| d.force()).collect()
    }
    /// Return the number of items in the batch.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Check if the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
/// A memoized function: wraps a computation and caches the result for each call.
/// Useful when the same lazy value is referenced multiple times.
pub struct MemoFn<A> {
    cell: OnceCell<A>,
    func: Box<dyn Fn() -> A>,
}
impl<A> MemoFn<A> {
    /// Create a new `MemoFn`.
    pub fn new(f: impl Fn() -> A + 'static) -> Self {
        MemoFn {
            cell: OnceCell::new(),
            func: Box::new(f),
        }
    }
    /// Get the memoized result, computing it on first call.
    pub fn get(&self) -> &A {
        self.cell.get_or_init(|| (self.func)())
    }
    /// Return true if already computed.
    pub fn is_cached(&self) -> bool {
        self.cell.get().is_some()
    }
}
/// A deferred value that is computed from another lazy value.
pub struct Deferred<A> {
    inner: Box<dyn FnOnce() -> A>,
}
impl<A: 'static> Deferred<A> {
    /// Defer a computation.
    pub fn new(f: impl FnOnce() -> A + 'static) -> Self {
        Deferred { inner: Box::new(f) }
    }
    /// Lift a value into `Deferred`.
    pub fn pure(a: A) -> Self {
        Deferred::new(move || a)
    }
    /// Force the deferred value.
    pub fn force(self) -> A {
        (self.inner)()
    }
    /// Map over the deferred value (still lazy).
    pub fn map<B: 'static>(self, f: impl FnOnce(A) -> B + 'static) -> Deferred<B> {
        Deferred::new(move || f(self.force()))
    }
    /// Bind: flat-map over a deferred value.
    pub fn bind<B: 'static>(self, f: impl FnOnce(A) -> Deferred<B> + 'static) -> Deferred<B> {
        Deferred::new(move || f(self.force()).force())
    }
    /// Zip two deferred values into a pair (still lazy).
    pub fn zip<B: 'static>(self, other: Deferred<B>) -> Deferred<(A, B)> {
        Deferred::new(move || (self.force(), other.force()))
    }
}
/// A lazy state machine that transitions lazily.
///
/// Each transition is deferred until the state is actually needed.
#[allow(dead_code)]
pub struct LazyStateMachineRs<S, I> {
    /// Current state (lazily wrapped).
    state: Deferred<S>,
    /// Transition function.
    transition: Arc<dyn Fn(S, I) -> S + Send + Sync>,
}
#[allow(dead_code)]
impl<S: Clone + 'static, I: 'static> LazyStateMachineRs<S, I> {
    /// Create a new lazy state machine.
    pub fn new(initial: S, transition: impl Fn(S, I) -> S + Send + Sync + 'static) -> Self {
        Self {
            state: Deferred::pure(initial),
            transition: Arc::new(transition),
        }
    }
    /// Perform a lazy transition with the given input.
    pub fn step(self, input: I) -> Self {
        let t = self.transition.clone();
        let t2 = t.clone();
        let new_state = self.state.map(move |s| t(s, input));
        Self {
            state: new_state,
            transition: t2,
        }
    }
    /// Force the current state.
    pub fn current_state(self) -> S {
        self.state.force()
    }
}
/// A circular buffer implementing a sliding window over lazy streams.
///
/// Used for stream algorithms requiring recent history access.
#[allow(dead_code)]
pub struct LazyWindowRs<T> {
    buf: std::collections::VecDeque<Deferred<T>>,
    capacity: usize,
}
#[allow(dead_code)]
impl<T: 'static> LazyWindowRs<T> {
    /// Create a new lazy window with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: std::collections::VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    /// Push a deferred value into the window.
    pub fn push(&mut self, val: Deferred<T>) {
        if self.buf.len() == self.capacity {
            self.buf.pop_front();
        }
        self.buf.push_back(val);
    }
    /// Force all values in the window.
    pub fn force_all(self) -> Vec<T> {
        self.buf.into_iter().map(|d| d.force()).collect()
    }
    /// Return the current number of elements in the window.
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    /// Return true if the window is empty.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    /// Return the window capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
/// A lazy natural number (conatural number) represented in Rust.
///
/// Can be either finite (a concrete `u64`) or infinite.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoNatRs {
    /// A finite conatural number.
    Finite(u64),
    /// The infinite conatural number (ω).
    Infinity,
}
#[allow(dead_code)]
impl CoNatRs {
    /// The zero conatural number.
    pub fn zero() -> Self {
        CoNatRs::Finite(0)
    }
    /// The successor of a conatural number.
    pub fn succ(self) -> Self {
        match self {
            CoNatRs::Finite(n) => CoNatRs::Finite(n + 1),
            CoNatRs::Infinity => CoNatRs::Infinity,
        }
    }
    /// Check if zero.
    pub fn is_zero(&self) -> bool {
        matches!(self, CoNatRs::Finite(0))
    }
    /// Check if infinite.
    pub fn is_infinity(&self) -> bool {
        matches!(self, CoNatRs::Infinity)
    }
    /// Convert to Option<u64>.
    pub fn to_finite(self) -> Option<u64> {
        match self {
            CoNatRs::Finite(n) => Some(n),
            CoNatRs::Infinity => None,
        }
    }
    /// Add two conatural numbers.
    pub fn add(self, other: CoNatRs) -> CoNatRs {
        match (self, other) {
            (CoNatRs::Infinity, _) | (_, CoNatRs::Infinity) => CoNatRs::Infinity,
            (CoNatRs::Finite(a), CoNatRs::Finite(b)) => CoNatRs::Finite(a + b),
        }
    }
    /// Minimum of two conatural numbers.
    pub fn min(self, other: CoNatRs) -> CoNatRs {
        match (self, other) {
            (CoNatRs::Finite(a), CoNatRs::Finite(b)) => CoNatRs::Finite(a.min(b)),
            (CoNatRs::Infinity, x) | (x, CoNatRs::Infinity) => x,
        }
    }
}
/// A `LazyOption<A>` wraps a potentially-absent lazy value.
pub enum LazyOption<A> {
    /// No value.
    None,
    /// A deferred value.
    Some(Deferred<A>),
}
impl<A: 'static> LazyOption<A> {
    /// Return the `None` variant.
    pub fn none() -> Self {
        LazyOption::None
    }
    /// Wrap a deferred computation.
    pub fn some(f: impl FnOnce() -> A + 'static) -> Self {
        LazyOption::Some(Deferred::new(f))
    }
    /// Lift an eager value.
    pub fn pure(a: A) -> Self {
        LazyOption::Some(Deferred::pure(a))
    }
    /// Return true if this is `None`.
    pub fn is_none(&self) -> bool {
        matches!(self, LazyOption::None)
    }
    /// Force into `Option<A>`.
    pub fn force(self) -> Option<A> {
        match self {
            LazyOption::None => None,
            LazyOption::Some(d) => Some(d.force()),
        }
    }
    /// Map over the lazy option.
    pub fn map<B: 'static>(self, f: impl FnOnce(A) -> B + 'static) -> LazyOption<B> {
        match self {
            LazyOption::None => LazyOption::None,
            LazyOption::Some(d) => LazyOption::Some(d.map(f)),
        }
    }
}
/// A delay monad value in Rust: either immediately available or delayed.
///
/// Models the partiality / delay monad for potentially non-terminating computations.
#[allow(dead_code)]
pub enum DelayRs<A> {
    /// The value is available now.
    Now(A),
    /// The computation needs one more step.
    Later(Box<dyn FnOnce() -> DelayRs<A>>),
}
#[allow(dead_code)]
impl<A: 'static> DelayRs<A> {
    /// Inject a value immediately.
    pub fn now(a: A) -> Self {
        DelayRs::Now(a)
    }
    /// Delay by one step.
    pub fn later(f: impl FnOnce() -> DelayRs<A> + 'static) -> Self {
        DelayRs::Later(Box::new(f))
    }
    /// Force the computation, running at most `fuel` steps.
    pub fn run(self, fuel: usize) -> Option<A> {
        let mut current = self;
        let mut remaining = fuel;
        loop {
            match current {
                DelayRs::Now(a) => return Some(a),
                DelayRs::Later(f) => {
                    if remaining == 0 {
                        return None;
                    }
                    remaining -= 1;
                    current = f();
                }
            }
        }
    }
    /// Map over a delayed value.
    pub fn map<B: 'static>(self, f: impl FnOnce(A) -> B + 'static) -> DelayRs<B> {
        match self {
            DelayRs::Now(a) => DelayRs::Now(f(a)),
            DelayRs::Later(g) => DelayRs::later(move || g().map(f)),
        }
    }
    /// Bind over a delayed value.
    pub fn bind<B: 'static>(self, f: impl FnOnce(A) -> DelayRs<B> + 'static) -> DelayRs<B> {
        match self {
            DelayRs::Now(a) => f(a),
            DelayRs::Later(g) => DelayRs::later(move || g().bind(f)),
        }
    }
}
/// A pair of deferred computations.
pub struct LazyPair<A, B> {
    first: Deferred<A>,
    second: Deferred<B>,
}
impl<A: 'static, B: 'static> LazyPair<A, B> {
    /// Create a new lazy pair.
    pub fn new(a: Deferred<A>, b: Deferred<B>) -> Self {
        Self {
            first: a,
            second: b,
        }
    }
    /// Force both elements.
    pub fn force(self) -> (A, B) {
        (self.first.force(), self.second.force())
    }
    /// Map over the first element.
    pub fn map_first<C: 'static>(self, f: impl FnOnce(A) -> C + 'static) -> LazyPair<C, B> {
        LazyPair {
            first: self.first.map(f),
            second: self.second,
        }
    }
    /// Map over the second element.
    pub fn map_second<C: 'static>(self, f: impl FnOnce(B) -> C + 'static) -> LazyPair<A, C> {
        LazyPair {
            first: self.first,
            second: self.second.map(f),
        }
    }
}
/// A `Thunk<A>` is a lazily-evaluated value: the computation runs at most once.
pub struct Thunk<A> {
    pub(super) cell: OnceCell<A>,
    init: Option<Box<dyn FnOnce() -> A>>,
}
impl<A> Thunk<A> {
    /// Create a new thunk wrapping a deferred computation.
    pub fn new(f: impl FnOnce() -> A + 'static) -> Self {
        Thunk {
            cell: OnceCell::new(),
            init: Some(Box::new(f)),
        }
    }
    /// Create an already-evaluated thunk.
    pub fn evaluated(a: A) -> Self {
        let cell = OnceCell::new();
        let _ = cell.set(a);
        Thunk { cell, init: None }
    }
    /// Force the thunk: run the computation if not yet evaluated.
    pub fn force(&mut self) -> &A {
        if self.cell.get().is_none() {
            if let Some(f) = self.init.take() {
                let _ = self.cell.set(f());
            }
        }
        self.cell.get().expect("thunk init failed")
    }
    /// Return true if the thunk has already been forced.
    pub fn is_evaluated(&self) -> bool {
        self.cell.get().is_some()
    }
}
