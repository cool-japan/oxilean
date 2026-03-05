//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::cell::OnceCell;

use super::functions::*;
use std::collections::HashMap;

/// Applicative functor over thunks.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ThunkApp<A: Clone> {
    pub val: A,
}
#[allow(dead_code)]
impl<A: Clone> ThunkApp<A> {
    pub fn pure(a: A) -> Self {
        ThunkApp { val: a }
    }
    pub fn map<B: Clone, F: Fn(A) -> B>(self, f: F) -> ThunkApp<B> {
        ThunkApp { val: f(self.val) }
    }
    pub fn ap<B: Clone, F>(self, tf: ThunkApp<F>) -> ThunkApp<B>
    where
        F: Fn(A) -> B + Clone,
    {
        ThunkApp {
            val: (tf.val)(self.val),
        }
    }
}
/// A memoized function.
///
/// Wraps a function `f : A → B` such that each call is cached by key.
pub struct Memo<A: std::hash::Hash + Eq + Clone, B: Clone> {
    f: Box<dyn Fn(&A) -> B>,
    cache: std::collections::HashMap<A, B>,
}
impl<A: std::hash::Hash + Eq + Clone, B: Clone> Memo<A, B> {
    /// Create a new memoized function.
    pub fn new<F: Fn(&A) -> B + 'static>(f: F) -> Self {
        Self {
            f: Box::new(f),
            cache: std::collections::HashMap::new(),
        }
    }
    /// Call the memoized function, caching the result.
    pub fn call(&mut self, arg: &A) -> &B {
        if !self.cache.contains_key(arg) {
            let result = (self.f)(arg);
            self.cache.insert(arg.clone(), result);
        }
        &self.cache[arg]
    }
    /// Clear the cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    /// Number of cached entries.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}
/// A lazy tree structure where children are computed on demand.
pub enum ThunkTree<T> {
    /// A leaf node.
    Leaf(T),
    /// A lazy node whose children are thunked.
    Node(Box<dyn Fn() -> Vec<ThunkTree<T>>>),
}
impl<T: Clone> ThunkTree<T> {
    /// Create a leaf.
    pub fn leaf(value: T) -> Self {
        Self::Leaf(value)
    }
    /// Create a lazy node.
    pub fn node<F: Fn() -> Vec<ThunkTree<T>> + 'static>(f: F) -> Self {
        Self::Node(Box::new(f))
    }
    /// Collect all leaves by forcing the tree recursively.
    pub fn leaves(&self) -> Vec<T>
    where
        T: Clone,
    {
        match self {
            ThunkTree::Leaf(v) => vec![v.clone()],
            ThunkTree::Node(f) => f().iter().flat_map(|c| c.leaves()).collect(),
        }
    }
    /// Check if this is a leaf.
    pub fn is_leaf(&self) -> bool {
        matches!(self, ThunkTree::Leaf(_))
    }
}
/// A lazily-built list (stream-like).
///
/// Each element is produced by a function, allowing potentially infinite streams
/// to be represented finitely and evaluated on demand.
pub struct LazyList<T> {
    produced: Vec<T>,
    next: Option<Box<dyn Fn(usize) -> Option<T>>>,
}
impl<T: Clone> LazyList<T> {
    /// Create a list backed by a generating function.
    pub fn from_fn<F: Fn(usize) -> Option<T> + 'static>(f: F) -> Self {
        Self {
            produced: Vec::new(),
            next: Some(Box::new(f)),
        }
    }
    /// Get the element at index `i`.
    pub fn get(&mut self, i: usize) -> Option<&T> {
        while self.produced.len() <= i {
            let idx = self.produced.len();
            if let Some(f) = &self.next {
                match f(idx) {
                    Some(v) => self.produced.push(v),
                    None => {
                        self.next = None;
                        break;
                    }
                }
            } else {
                break;
            }
        }
        self.produced.get(i)
    }
    /// Take the first `n` elements, forcing them.
    pub fn take(&mut self, n: usize) -> Vec<T> {
        (0..n).filter_map(|i| self.get(i).cloned()).collect()
    }
    /// Number of elements produced so far.
    pub fn produced_count(&self) -> usize {
        self.produced.len()
    }
}
/// A deferred value that may fail to compute.
pub struct TryThunk<T, E> {
    init: Option<Box<dyn FnOnce() -> Result<T, E>>>,
    pub(super) value: Option<Result<T, E>>,
}
impl<T, E> TryThunk<T, E> {
    /// Create a new try-thunk.
    pub fn new<F: FnOnce() -> Result<T, E> + 'static>(f: F) -> Self {
        Self {
            init: Some(Box::new(f)),
            value: None,
        }
    }
    /// Force the thunk. Returns `&Result<T, E>`.
    pub fn force(&mut self) -> &Result<T, E> {
        if self.value.is_none() {
            if let Some(f) = self.init.take() {
                self.value = Some(f());
            }
        }
        self.value
            .as_ref()
            .expect("TryThunk: value missing after force")
    }
    /// Check if already forced.
    pub fn is_forced(&self) -> bool {
        self.value.is_some()
    }
}
/// A lazy stream (coinductive list) implemented as a linked structure of thunks.
///
/// Models the cofree comonad / productive corecursion pattern.
#[allow(dead_code)]
pub enum LazyStream<T> {
    /// Empty stream.
    Nil,
    /// A head value paired with a lazily-computed tail.
    Cons(T, Box<dyn FnOnce() -> LazyStream<T>>),
}
#[allow(dead_code)]
impl<T: Clone> LazyStream<T> {
    /// Construct an empty stream.
    pub fn nil() -> Self {
        LazyStream::Nil
    }
    /// Construct a cons cell.
    pub fn cons<F: FnOnce() -> LazyStream<T> + 'static>(head: T, tail: F) -> Self {
        LazyStream::Cons(head, Box::new(tail))
    }
    /// Force the stream and collect up to `n` elements.
    pub fn take(self, n: usize) -> Vec<T> {
        let mut result = Vec::new();
        let mut current = self;
        for _ in 0..n {
            match current {
                LazyStream::Nil => break,
                LazyStream::Cons(h, tl) => {
                    result.push(h);
                    current = tl();
                }
            }
        }
        result
    }
    /// Check if the stream is empty (without forcing the tail).
    pub fn is_nil(&self) -> bool {
        matches!(self, LazyStream::Nil)
    }
}
/// A sequence of thunks evaluated lazily.
///
/// Each element is computed on demand and cached independently.
pub struct ThunkSeq<T> {
    items: Vec<Box<dyn Fn() -> T>>,
    cache: Vec<Option<T>>,
}
impl<T: Clone> ThunkSeq<T> {
    /// Create an empty sequence.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            cache: Vec::new(),
        }
    }
    /// Push a lazy element.
    pub fn push<F: Fn() -> T + 'static>(&mut self, f: F) {
        self.items.push(Box::new(f));
        self.cache.push(None);
    }
    /// Get the element at index `i`, forcing it if needed.
    pub fn get(&mut self, i: usize) -> Option<&T> {
        if i >= self.items.len() {
            return None;
        }
        if self.cache[i].is_none() {
            self.cache[i] = Some((self.items[i])());
        }
        self.cache[i].as_ref()
    }
    /// Number of elements.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Count forced elements.
    pub fn forced_count(&self) -> usize {
        self.cache.iter().filter(|c| c.is_some()).count()
    }
}
/// A thunk backed by `OnceCell` for safe lazy initialization.
pub struct OnceCellThunk<T> {
    pub(super) cell: OnceCell<T>,
    init: Option<Box<dyn FnOnce() -> T>>,
}
impl<T> OnceCellThunk<T> {
    /// Create a new `OnceCellThunk` with the given initializer.
    pub fn new<F: FnOnce() -> T + 'static>(f: F) -> Self {
        Self {
            cell: OnceCell::new(),
            init: Some(Box::new(f)),
        }
    }
    /// Force the thunk, returning a reference to the value.
    pub fn force(&mut self) -> &T {
        if self.cell.get().is_none() {
            if let Some(f) = self.init.take() {
                let _ = self.cell.set(f());
            }
        }
        self.cell.get().expect("OnceCellThunk: init was None")
    }
    /// Check whether the thunk has been forced.
    pub fn is_forced(&self) -> bool {
        self.cell.get().is_some()
    }
}
/// Cofree comonad: infinite stream with comonadic structure.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CofreeStream<A> {
    pub elements: Vec<A>,
    pub period: usize,
}
#[allow(dead_code)]
impl<A: Clone> CofreeStream<A> {
    pub fn from_periodic(seed: Vec<A>) -> Self {
        let len = seed.len().max(1);
        CofreeStream {
            elements: seed,
            period: len,
        }
    }
    pub fn extract(&self) -> &A {
        &self.elements[0]
    }
    pub fn tail(&self) -> Self {
        if self.elements.len() <= 1 {
            return self.clone();
        }
        let mut new_elems = self.elements[1..].to_vec();
        new_elems.push(self.elements[0].clone());
        CofreeStream {
            elements: new_elems,
            period: self.period,
        }
    }
    pub fn nth(&self, n: usize) -> &A {
        &self.elements[n % self.elements.len()]
    }
    pub fn map<B, F: Fn(&A) -> B>(&self, f: F) -> CofreeStream<B> {
        CofreeStream {
            elements: self.elements.iter().map(f).collect(),
            period: self.period,
        }
    }
}
/// Demand-driven computation graph node.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ComputeNode<T: Clone> {
    pub name: String,
    pub value: Option<T>,
    pub deps: Vec<String>,
    pub is_dirty: bool,
}
#[allow(dead_code)]
impl<T: Clone> ComputeNode<T> {
    pub fn new(name: &str) -> Self {
        ComputeNode {
            name: name.to_string(),
            value: None,
            deps: Vec::new(),
            is_dirty: true,
        }
    }
    pub fn add_dep(&mut self, dep: &str) {
        self.deps.push(dep.to_string());
    }
    pub fn set_value(&mut self, val: T) {
        self.value = Some(val);
        self.is_dirty = false;
    }
    pub fn invalidate(&mut self) {
        self.value = None;
        self.is_dirty = true;
    }
    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }
}
/// A simple game-semantics arena: tracks questions and answers for a thunk-like computation.
///
/// In game semantics, evaluation of `Thunk α` corresponds to a two-player game
/// where Opponent asks "what is the value?" and Proponent answers with `α`.
/// This struct tracks that dialogue.
#[allow(dead_code)]
pub struct GameArena {
    /// Moves played so far (question index, answer value).
    moves: Vec<(usize, String)>,
    /// Counter for generating fresh question indices.
    question_counter: usize,
}
#[allow(dead_code)]
impl GameArena {
    /// Create an empty arena.
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            question_counter: 0,
        }
    }
    /// Opponent asks a question; returns the question index.
    pub fn ask(&mut self) -> usize {
        let q = self.question_counter;
        self.question_counter += 1;
        q
    }
    /// Proponent answers question `q` with value `v`.
    pub fn answer(&mut self, q: usize, v: impl Into<String>) {
        self.moves.push((q, v.into()));
    }
    /// Find the answer to question `q`, if any.
    pub fn get_answer(&self, q: usize) -> Option<&str> {
        self.moves
            .iter()
            .find(|(qi, _)| *qi == q)
            .map(|(_, a)| a.as_str())
    }
    /// Number of moves played.
    pub fn move_count(&self) -> usize {
        self.moves.len()
    }
    /// Check if the game is "complete" (every question has an answer).
    pub fn is_complete(&self) -> bool {
        let answered: std::collections::HashSet<usize> =
            self.moves.iter().map(|(q, _)| *q).collect();
        (0..self.question_counter).all(|q| answered.contains(&q))
    }
}
/// A future-like value that becomes available after a delay (modelled as count).
#[derive(Debug, Clone)]
pub struct Delayed<T> {
    value: T,
    delay_steps: usize,
    elapsed: usize,
}
impl<T: Clone> Delayed<T> {
    /// Create a delayed value that becomes available after `steps` evaluations.
    pub fn new(value: T, steps: usize) -> Self {
        Self {
            value,
            delay_steps: steps,
            elapsed: 0,
        }
    }
    /// Advance one step. Returns the value when ready.
    pub fn step(&mut self) -> Option<T> {
        self.elapsed += 1;
        if self.elapsed >= self.delay_steps {
            Some(self.value.clone())
        } else {
            None
        }
    }
    /// Check if ready.
    pub fn is_ready(&self) -> bool {
        self.elapsed >= self.delay_steps
    }
    /// Remaining steps.
    pub fn remaining(&self) -> usize {
        self.delay_steps.saturating_sub(self.elapsed)
    }
}
/// Blackhole detection for cyclic thunk evaluation.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ThunkState<T> {
    Unevaluated,
    Evaluating,
    Evaluated(T),
    Error(String),
}
#[allow(dead_code)]
impl<T: Clone> ThunkState<T> {
    pub fn is_value(&self) -> bool {
        matches!(self, ThunkState::Evaluated(_))
    }
    pub fn is_blackhole(&self) -> bool {
        matches!(self, ThunkState::Evaluating)
    }
    pub fn get_value(&self) -> Option<&T> {
        match self {
            ThunkState::Evaluated(v) => Some(v),
            _ => None,
        }
    }
    pub fn enter(&mut self) -> bool {
        match self {
            ThunkState::Unevaluated => {
                *self = ThunkState::Evaluating;
                true
            }
            ThunkState::Evaluating => false,
            _ => false,
        }
    }
    pub fn fill(&mut self, val: T) {
        *self = ThunkState::Evaluated(val);
    }
    pub fn fail(&mut self, msg: &str) {
        *self = ThunkState::Error(msg.to_string());
    }
}
/// A Kleisli arrow in the Thunk monad: `A → Thunk B` represented as a Rust closure.
///
/// Provides composition via cokleisli / kleisli combinators.
#[allow(dead_code)]
pub struct ThunkKleisli<A, B> {
    arrow: Box<dyn Fn(A) -> Option<B>>,
}
#[allow(dead_code)]
impl<A: 'static, B: Clone + 'static> ThunkKleisli<A, B> {
    /// Create a Kleisli arrow from a function.
    pub fn new<F: Fn(A) -> Option<B> + 'static>(f: F) -> Self {
        Self { arrow: Box::new(f) }
    }
    /// Apply the arrow.
    pub fn apply(&self, a: A) -> Option<B> {
        (self.arrow)(a)
    }
    /// Compose this arrow with another: `self >=> other`.
    pub fn compose<C: Clone + 'static>(self, other: ThunkKleisli<B, C>) -> ThunkKleisli<A, C> {
        ThunkKleisli::new(move |a| {
            let b = (self.arrow)(a)?;
            other.apply(b)
        })
    }
}
/// Suspension monad: delayed computation with abort capability.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Suspension<A> {
    Done(A),
    Suspended(String),
    Aborted(String),
}
#[allow(dead_code)]
impl<A: Clone> Suspension<A> {
    pub fn pure(a: A) -> Self {
        Suspension::Done(a)
    }
    pub fn suspend(desc: &str) -> Self {
        Suspension::Suspended(desc.to_string())
    }
    pub fn abort(msg: &str) -> Self {
        Suspension::Aborted(msg.to_string())
    }
    pub fn is_done(&self) -> bool {
        matches!(self, Suspension::Done(_))
    }
    pub fn map<B: Clone, F: Fn(A) -> B>(self, f: F) -> Suspension<B> {
        match self {
            Suspension::Done(a) => Suspension::Done(f(a)),
            Suspension::Suspended(d) => Suspension::Suspended(d),
            Suspension::Aborted(e) => Suspension::Aborted(e),
        }
    }
    pub fn and_then<B: Clone, F: Fn(A) -> Suspension<B>>(self, f: F) -> Suspension<B> {
        match self {
            Suspension::Done(a) => f(a),
            Suspension::Suspended(d) => Suspension::Suspended(d),
            Suspension::Aborted(e) => Suspension::Aborted(e),
        }
    }
}
/// A comonad-style context: wraps a focused value with an environment.
///
/// Models `(env, focus)` pairs where `focus` is the current value and
/// `env` captures additional context (e.g., for zipper traversal).
#[allow(dead_code)]
pub struct ComonadCtx<E, A> {
    pub(super) env: E,
    pub(super) focus: A,
}
#[allow(dead_code)]
impl<E: Clone, A: Clone> ComonadCtx<E, A> {
    /// Create a new comonad context.
    pub fn new(env: E, focus: A) -> Self {
        Self { env, focus }
    }
    /// Extract the focused value (comonad `extract`).
    pub fn extract(&self) -> A {
        self.focus.clone()
    }
    /// Duplicate: produce a context focused on itself.
    pub fn duplicate(&self) -> ComonadCtx<E, ComonadCtx<E, A>> {
        ComonadCtx {
            env: self.env.clone(),
            focus: self.clone(),
        }
    }
    /// Extend: apply a function to the whole context, producing a new context.
    pub fn extend<B, F: Fn(&ComonadCtx<E, A>) -> B>(&self, f: F) -> ComonadCtx<E, B> {
        ComonadCtx {
            env: self.env.clone(),
            focus: f(self),
        }
    }
    /// Map over the focused value.
    pub fn map<B, F: FnOnce(A) -> B>(self, f: F) -> ComonadCtx<E, B> {
        ComonadCtx {
            env: self.env,
            focus: f(self.focus),
        }
    }
    /// Get a reference to the environment.
    pub fn env(&self) -> &E {
        &self.env
    }
}
/// A pool of deferred computations indexed by `usize`.
#[derive(Default)]
pub struct DeferredPool<T> {
    pending: std::collections::HashMap<usize, Box<dyn FnOnce() -> T>>,
    resolved: std::collections::HashMap<usize, T>,
    next_id: usize,
}
impl<T> DeferredPool<T> {
    /// Create an empty pool.
    pub fn new() -> Self {
        Self {
            pending: std::collections::HashMap::new(),
            resolved: std::collections::HashMap::new(),
            next_id: 0,
        }
    }
    /// Submit a computation, returning its handle.
    pub fn submit<F: FnOnce() -> T + 'static>(&mut self, f: F) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.pending.insert(id, Box::new(f));
        id
    }
    /// Force a specific deferred computation.
    pub fn force(&mut self, id: usize) -> Option<&T> {
        if !self.resolved.contains_key(&id) {
            if let Some(f) = self.pending.remove(&id) {
                self.resolved.insert(id, f());
            }
        }
        self.resolved.get(&id)
    }
    /// Force all pending computations.
    pub fn force_all(&mut self) {
        let ids: Vec<usize> = self.pending.keys().copied().collect();
        for id in ids {
            self.force(id);
        }
    }
    /// Number of pending computations.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Number of resolved computations.
    pub fn resolved_count(&self) -> usize {
        self.resolved.len()
    }
}
/// A fixed-point computation using memoized thunks.
///
/// Implements `fix f = f (fix f)` lazily: each call to `compute` evaluates
/// one step, caching the result.  Useful for lazy recursive definitions.
#[allow(clippy::type_complexity)]
pub struct FixThunk<T: Clone> {
    memo: std::collections::HashMap<usize, T>,
    step: Box<dyn Fn(usize, &dyn Fn(usize) -> T) -> T>,
}
impl<T: Clone> FixThunk<T> {
    /// Create a new fixed-point thunk.
    ///
    /// `f` receives the current index and a reference to `compute` so it can
    /// look up previously computed values recursively.
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(usize, &dyn Fn(usize) -> T) -> T + 'static,
    {
        Self {
            memo: std::collections::HashMap::new(),
            step: Box::new(f),
        }
    }
    /// Compute the value at index `n`, caching and reusing previous results.
    pub fn compute(&mut self, n: usize) -> T {
        if let Some(v) = self.memo.get(&n) {
            return v.clone();
        }
        let memo_ref: *mut std::collections::HashMap<usize, T> = &mut self.memo;
        let lookup = |k: usize| -> T {
            unsafe { (*memo_ref).get(&k).cloned() }.expect("missing cached value")
        };
        let v = (self.step)(n, &lookup);
        self.memo.insert(n, v.clone());
        v
    }
}
/// A memoized thunk that caches its result after first evaluation.
#[allow(dead_code)]
pub struct MemoThunk<T: Clone> {
    cached: Option<T>,
}
#[allow(dead_code)]
impl<T: Clone> MemoThunk<T> {
    pub fn new() -> Self {
        MemoThunk { cached: None }
    }
    pub fn force_with<F: FnOnce() -> T>(&mut self, f: F) -> T {
        if let Some(ref val) = self.cached {
            val.clone()
        } else {
            let val = f();
            self.cached = Some(val.clone());
            val
        }
    }
    pub fn is_forced(&self) -> bool {
        self.cached.is_some()
    }
    pub fn reset(&mut self) {
        self.cached = None;
    }
}
/// Lazy linked-list (stream) using closures for infinite sequences.
#[allow(dead_code)]
pub struct LazyLinkedStream<T> {
    pub head: T,
    pub tail_fn: Box<dyn Fn() -> Option<Box<LazyLinkedStream<T>>>>,
}
#[allow(dead_code)]
impl<T: Clone + 'static> LazyLinkedStream<T> {
    pub fn singleton(val: T) -> Self {
        LazyLinkedStream {
            head: val,
            tail_fn: Box::new(|| None),
        }
    }
    pub fn take_n(&self, n: usize) -> Vec<T> {
        let mut result = vec![self.head.clone()];
        let mut current = (self.tail_fn)();
        while result.len() < n {
            match current {
                None => break,
                Some(node) => {
                    result.push(node.head.clone());
                    current = (node.tail_fn)();
                }
            }
        }
        result
    }
}
/// A value that is either immediately available or deferred.
#[derive(Debug, Clone)]
pub enum Deferred<T> {
    /// Available immediately.
    Now(T),
    /// Will be computed later (opaque handle).
    Later(usize),
}
impl<T: Clone> Deferred<T> {
    /// Create an immediate value.
    pub fn now(v: T) -> Self {
        Self::Now(v)
    }
    /// Create a deferred handle.
    pub fn later(id: usize) -> Self {
        Self::Later(id)
    }
    /// Check if immediately available.
    pub fn is_ready(&self) -> bool {
        matches!(self, Deferred::Now(_))
    }
    /// Unwrap the immediate value, or call `f` with the handle.
    pub fn resolve_or<F: FnOnce(usize) -> T>(self, f: F) -> T {
        match self {
            Deferred::Now(v) => v,
            Deferred::Later(id) => f(id),
        }
    }
    /// Map over an immediate value.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Deferred<U> {
        match self {
            Deferred::Now(v) => Deferred::Now(f(v)),
            Deferred::Later(id) => Deferred::Later(id),
        }
    }
}
/// An interaction tree node modelling one step of a potentially-infinite computation.
///
/// Based on the `ITree` structure used in verified concurrency frameworks.
#[allow(dead_code)]
pub enum ITreeNode<E, R> {
    /// Pure return value.
    Ret(R),
    /// Silent step (τ): delays computation by one tick.
    Tau(Box<dyn FnOnce() -> ITreeNode<E, R>>),
    /// Visible event `e` followed by a continuation `k`.
    Vis(E, Box<dyn Fn(usize) -> ITreeNode<E, R>>),
}
#[allow(dead_code)]
impl<E: Clone, R: Clone> ITreeNode<E, R> {
    /// Construct a pure return.
    pub fn ret(r: R) -> Self {
        ITreeNode::Ret(r)
    }
    /// Construct a tau step.
    pub fn tau<F: FnOnce() -> ITreeNode<E, R> + 'static>(f: F) -> Self {
        ITreeNode::Tau(Box::new(f))
    }
    /// Construct a visible event step.
    pub fn vis<K: Fn(usize) -> ITreeNode<E, R> + 'static>(e: E, k: K) -> Self {
        ITreeNode::Vis(e, Box::new(k))
    }
    /// Spin-up at most `fuel` tau steps and return the result if reached.
    pub fn run(self, fuel: usize) -> Option<R> {
        let mut current = self;
        for _ in 0..fuel {
            match current {
                ITreeNode::Ret(r) => return Some(r),
                ITreeNode::Tau(f) => {
                    current = f();
                }
                ITreeNode::Vis(_, _) => return None,
            }
        }
        None
    }
}
