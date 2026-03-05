//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A Mealy machine over Rust values: given state S, reads input I, emits output O.
///
/// Each step function takes the current state and an input and returns
/// the next state plus an output.
#[allow(dead_code)]
pub struct MealyMachineRs<S, I, O> {
    /// The transition function: (state, input) → (next_state, output).
    transition: Box<dyn Fn(S, I) -> (S, O)>,
    /// The current state of the machine.
    state: S,
}
#[allow(dead_code)]
impl<S: Clone + 'static, I: 'static, O: 'static> MealyMachineRs<S, I, O> {
    /// Create a new Mealy machine with an initial state and transition function.
    pub fn new(state: S, transition: impl Fn(S, I) -> (S, O) + 'static) -> Self {
        Self {
            transition: Box::new(transition),
            state,
        }
    }
    /// Step the machine with one input, returning the output and advancing state.
    pub fn step(&mut self, input: I) -> O {
        let old_state = self.state.clone();
        let (new_state, output) = (self.transition)(old_state, input);
        self.state = new_state;
        output
    }
    /// Run the machine over a vector of inputs, collecting outputs.
    pub fn run_vec(&mut self, inputs: Vec<I>) -> Vec<O> {
        inputs.into_iter().map(|i| self.step(i)).collect()
    }
    /// Get the current state.
    pub fn current_state(&self) -> &S {
        &self.state
    }
    /// Reset the machine to a new state.
    pub fn reset(&mut self, state: S) {
        self.state = state;
    }
}
/// A Moore machine over Rust values: output depends only on state.
///
/// Each transition takes a state and input and returns the next state.
/// The output function maps states to outputs.
#[allow(dead_code)]
pub struct MooreMachineRs<S, I, O> {
    /// Transition function: (state, input) → next_state.
    transition: Box<dyn Fn(&S, I) -> S>,
    /// Output function: state → output.
    output: Box<dyn Fn(&S) -> O>,
    /// The current state.
    state: S,
}
#[allow(dead_code)]
impl<S: 'static, I: 'static, O: 'static> MooreMachineRs<S, I, O> {
    /// Create a new Moore machine.
    pub fn new(
        state: S,
        transition: impl Fn(&S, I) -> S + 'static,
        output: impl Fn(&S) -> O + 'static,
    ) -> Self {
        Self {
            transition: Box::new(transition),
            output: Box::new(output),
            state,
        }
    }
    /// Read the current output (depends only on state).
    pub fn read_output(&self) -> O {
        (self.output)(&self.state)
    }
    /// Advance the machine with one input.
    pub fn step(&mut self, input: I) {
        self.state = (self.transition)(&self.state, input);
    }
    /// Run the machine over a vector of inputs, collecting one output per step.
    pub fn run_vec(&mut self, inputs: Vec<I>) -> Vec<O> {
        let mut results = Vec::with_capacity(inputs.len());
        for inp in inputs {
            results.push(self.read_output());
            self.step(inp);
        }
        results
    }
    /// Get the current state.
    pub fn state(&self) -> &S {
        &self.state
    }
}
/// A stream window that maintains a sliding window over a stream.
///
/// Useful for streaming algorithms that need fixed-size recent history.
#[allow(dead_code)]
pub struct StreamWindow<T> {
    /// The circular buffer.
    buf: Vec<Option<T>>,
    /// Window size.
    size: usize,
    /// Write position.
    pos: usize,
    /// Number of elements inserted.
    count: usize,
}
#[allow(dead_code)]
impl<T: Clone> StreamWindow<T> {
    /// Create a new stream window of the given size.
    pub fn new(size: usize) -> Self {
        Self {
            buf: vec![None; size],
            size,
            pos: 0,
            count: 0,
        }
    }
    /// Push a new element into the window.
    pub fn push(&mut self, val: T) {
        self.buf[self.pos] = Some(val);
        self.pos = (self.pos + 1) % self.size;
        self.count += 1;
    }
    /// Return the current window contents in order (oldest to newest).
    pub fn window(&self) -> Vec<T> {
        let len = self.count.min(self.size);
        let mut result = Vec::with_capacity(len);
        let start = if self.count >= self.size { self.pos } else { 0 };
        for i in 0..len {
            let idx = (start + i) % self.size;
            if let Some(ref v) = self.buf[idx] {
                result.push(v.clone());
            }
        }
        result
    }
    /// Return the number of elements currently in the window.
    pub fn len(&self) -> usize {
        self.count.min(self.size)
    }
    /// Return true if the window is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Return the window size.
    pub fn window_size(&self) -> usize {
        self.size
    }
}
/// Count stream-related declarations by category.
#[derive(Debug, Clone, Default)]
pub struct StreamDeclStats {
    /// Number of core type/constructor declarations.
    pub core: usize,
    /// Number of combinator declarations.
    pub combinators: usize,
    /// Number of monad/applicative declarations.
    pub monad: usize,
    /// Number of theorem declarations.
    pub theorems: usize,
}
impl StreamDeclStats {
    /// Compute stats for the given environment.
    pub fn compute(env: &Environment) -> Self {
        let core_names = ["Stream", "Stream.cons", "Stream.head", "Stream.tail"];
        let combinator_names = [
            "Stream.map",
            "Stream.take",
            "Stream.zip",
            "Stream.iterate",
            "Stream.drop",
            "Stream.nth",
            "Stream.const",
            "Stream.filter",
        ];
        let monad_names = ["Stream.pure", "Stream.scan", "Stream.interleave"];
        let theorem_names = ["Stream.head_cons", "Stream.tail_cons"];
        let count = |names: &[&str]| {
            names
                .iter()
                .filter(|&&n| env.get(&Name::str(n)).is_some())
                .count()
        };
        Self {
            core: count(&core_names),
            combinators: count(&combinator_names),
            monad: count(&monad_names),
            theorems: count(&theorem_names),
        }
    }
    /// Total number of stream declarations.
    pub fn total(&self) -> usize {
        self.core + self.combinators + self.monad + self.theorems
    }
}
/// A reactive stream combinator that merges two streams by priority.
///
/// When both streams have elements, the left stream takes priority.
#[allow(dead_code)]
pub struct PriorityMerge<T> {
    left: LazyStream<T>,
    right: LazyStream<T>,
}
#[allow(dead_code)]
impl<T: 'static> PriorityMerge<T> {
    /// Create a new priority merge of two streams.
    pub fn new(left: LazyStream<T>, right: LazyStream<T>) -> Self {
        Self { left, right }
    }
    /// Get the next element, preferring the left stream.
    pub fn next(&mut self) -> Option<T> {
        self.left.next().or_else(|| self.right.next())
    }
    /// Collect `n` elements from the merged stream.
    pub fn take_n(&mut self, n: usize) -> Vec<T> {
        (0..n).filter_map(|_| self.next()).collect()
    }
}
/// A count-min sketch for streaming frequency estimation.
///
/// Provides approximate frequency counts for stream elements.
#[allow(dead_code)]
pub struct CountMinSketchRs {
    /// 2D table of counts.
    table: Vec<Vec<u64>>,
    /// Number of rows (hash functions).
    d: usize,
    /// Number of columns (width).
    w: usize,
}
#[allow(dead_code)]
impl CountMinSketchRs {
    /// Create a new count-min sketch with `d` rows and `w` columns.
    pub fn new(d: usize, w: usize) -> Self {
        Self {
            table: vec![vec![0u64; w]; d],
            d,
            w,
        }
    }
    /// Update the sketch with one occurrence of an item.
    pub fn update(&mut self, item: u64) {
        for row in 0..self.d {
            let col = self.strm_ext_hash(item, row as u64) % self.w;
            self.table[row][col] += 1;
        }
    }
    /// Estimate the frequency of an item.
    pub fn estimate(&self, item: u64) -> u64 {
        (0..self.d)
            .map(|row| {
                let col = self.strm_ext_hash(item, row as u64) % self.w;
                self.table[row][col]
            })
            .min()
            .unwrap_or(0)
    }
    fn strm_ext_hash(&self, item: u64, seed: u64) -> usize {
        let h = item
            .wrapping_mul(2654435761)
            .wrapping_add(seed.wrapping_mul(40503));
        h as usize
    }
    /// Return total number of updates recorded (sum of row 0).
    pub fn total_updates(&self) -> u64 {
        if self.d > 0 {
            self.table[0].iter().sum()
        } else {
            0
        }
    }
}
/// A purely Rust-level lazy stream (for algorithm testing).
///
/// Unlike the OxiLean expression-level Stream, this is a Rust iterator wrapper.
pub struct LazyStream<T> {
    /// Underlying item source (boxed closure for laziness).
    gen: Box<dyn FnMut() -> Option<T>>,
}
impl<T> LazyStream<T> {
    /// Create a stream from a generator function.
    pub fn from_fn(gen: impl FnMut() -> Option<T> + 'static) -> Self {
        Self { gen: Box::new(gen) }
    }
    /// Create an infinite stream that always yields the same value.
    pub fn constant(val: T) -> Self
    where
        T: Clone + 'static,
    {
        Self::from_fn(move || Some(val.clone()))
    }
    /// Create a stream that iterates `init, f(init), f(f(init)), ...`.
    pub fn iterate(mut init: T, mut f: impl FnMut(T) -> T + 'static) -> Self
    where
        T: Clone + 'static,
    {
        Self::from_fn(move || {
            let curr = init.clone();
            init = f(init.clone());
            Some(curr)
        })
    }
    /// Advance the stream and get the next element.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<T> {
        (self.gen)()
    }
    /// Take the first `n` elements.
    pub fn take(mut self, n: usize) -> Vec<T> {
        (0..n).filter_map(|_| self.next()).collect()
    }
}
/// A simple Bloom filter over stream elements.
///
/// Uses k independent hash functions to track set membership approximately.
#[allow(dead_code)]
pub struct BloomFilterRs {
    /// Bit array.
    bits: Vec<bool>,
    /// Number of hash functions.
    k: usize,
    /// Size of the bit array.
    m: usize,
}
#[allow(dead_code)]
impl BloomFilterRs {
    /// Create a new Bloom filter with `m` bits and `k` hash functions.
    pub fn new(m: usize, k: usize) -> Self {
        Self {
            bits: vec![false; m],
            k,
            m,
        }
    }
    /// Insert an element into the filter.
    pub fn insert(&mut self, item: u64) {
        for i in 0..self.k {
            let idx = self.strm_ext_hash(item, i as u64) % self.m;
            self.bits[idx] = true;
        }
    }
    /// Query whether an element might be in the set.
    pub fn query(&self, item: u64) -> bool {
        (0..self.k).all(|i| {
            let idx = self.strm_ext_hash(item, i as u64) % self.m;
            self.bits[idx]
        })
    }
    /// Simple internal hash combining item and seed.
    fn strm_ext_hash(&self, item: u64, seed: u64) -> usize {
        let h = item
            .wrapping_mul(6364136223846793005)
            .wrapping_add(seed.wrapping_mul(1442695040888963407));
        h as usize
    }
    /// Return the number of set bits.
    pub fn count_set(&self) -> usize {
        self.bits.iter().filter(|&&b| b).count()
    }
    /// Return the capacity (m).
    pub fn capacity(&self) -> usize {
        self.m
    }
}
