//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::trampoline;
use std::collections::{HashMap, HashSet};

/// A labeled trampoline step: tag identifies which "function" to run next.
#[allow(dead_code)]
pub enum MultiStep<State, Output> {
    /// Run the "even" branch.
    Even(State),
    /// Run the "odd" branch.
    Odd(State),
    /// Done with output.
    Done(Output),
}
/// Detects potential infinite loops by tracking state hashes.
#[allow(dead_code)]
pub struct LoopDetector {
    /// Set of observed state fingerprints.
    seen: std::collections::HashSet<u64>,
    /// Number of states checked.
    pub checked: u64,
}
impl LoopDetector {
    /// Create a new detector.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            seen: std::collections::HashSet::new(),
            checked: 0,
        }
    }
    /// Record a state with the given fingerprint.
    /// Returns `true` if this state was seen before (loop detected).
    #[allow(dead_code)]
    pub fn check(&mut self, fingerprint: u64) -> bool {
        self.checked += 1;
        !self.seen.insert(fingerprint)
    }
    /// Clear history.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.seen.clear();
        self.checked = 0;
    }
    /// Number of unique states seen.
    #[allow(dead_code)]
    pub fn unique_states(&self) -> usize {
        self.seen.len()
    }
}
/// Aggregated TCO metrics across many functions.
#[allow(dead_code)]
#[derive(Default)]
pub struct TrampolineMetricsRegistry {
    /// Map from function name to metrics.
    pub functions: std::collections::HashMap<String, FunctionTcoMetrics>,
}
impl TrampolineMetricsRegistry {
    /// Create an empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a TCO call for `function` at `depth`.
    #[allow(dead_code)]
    pub fn record(&mut self, function: &str, depth: u64) {
        self.functions
            .entry(function.to_string())
            .or_insert_with(|| FunctionTcoMetrics::new(function))
            .record(depth);
    }
    /// Return the top `n` functions by total steps.
    #[allow(dead_code)]
    pub fn top_by_steps(&self, n: usize) -> Vec<&FunctionTcoMetrics> {
        let mut sorted: Vec<&FunctionTcoMetrics> = self.functions.values().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.total_steps));
        sorted.truncate(n);
        sorted
    }
    /// Total calls across all functions.
    #[allow(dead_code)]
    pub fn total_calls(&self) -> u64 {
        self.functions.values().map(|m| m.call_count).sum()
    }
}
/// A trampoline driver with a configurable step limit.
#[allow(dead_code)]
pub struct BoundedTrampoline {
    /// The maximum number of steps allowed.
    pub step_limit: u64,
}
impl BoundedTrampoline {
    /// Create a bounded trampoline with the given step limit.
    #[allow(dead_code)]
    pub fn new(step_limit: u64) -> Self {
        Self { step_limit }
    }
    /// Run `step` to completion, returning `Ok(value)` or an error if the limit
    /// was exceeded.
    #[allow(dead_code)]
    pub fn run<T>(&self, mut step: TailCall<T>) -> Result<T, String> {
        let mut count = 0u64;
        loop {
            match step {
                TailCall::Done(v) => return Ok(v),
                TailCall::Call(f) => {
                    count += 1;
                    if count > self.step_limit {
                        return Err(format!(
                            "trampoline step limit {} exceeded at step {}",
                            self.step_limit, count
                        ));
                    }
                    step = f();
                }
            }
        }
    }
}
/// A scheduler that drives a `TailCall<T>` in bounded batches.
#[allow(dead_code)]
pub struct TailCallScheduler<T> {
    /// Current pending step (None if already finished or not started).
    pending: Option<TailCall<T>>,
    /// Configuration for this scheduler.
    pub config: TailCallSchedulerConfig,
    /// Total steps taken so far.
    pub total_steps: u64,
}
impl<T> TailCallScheduler<T> {
    /// Create a new scheduler with the given step and the default config.
    #[allow(dead_code)]
    pub fn new(step: TailCall<T>) -> Self {
        Self {
            pending: Some(step),
            config: TailCallSchedulerConfig::default(),
            total_steps: 0,
        }
    }
    /// Create a scheduler with a custom config.
    #[allow(dead_code)]
    pub fn with_config(step: TailCall<T>, config: TailCallSchedulerConfig) -> Self {
        Self {
            pending: Some(step),
            config,
            total_steps: 0,
        }
    }
    /// Run up to `max_steps_per_batch` steps.
    #[allow(dead_code)]
    pub fn tick(&mut self) -> SchedulerTickResult<T> {
        let batch = self.config.max_steps_per_batch;
        for _ in 0..batch {
            match self.pending.take() {
                None => return SchedulerTickResult::Pending,
                Some(TailCall::Done(v)) => return SchedulerTickResult::Finished(v),
                Some(TailCall::Call(f)) => {
                    self.total_steps += 1;
                    if self.total_steps > self.config.step_limit {
                        return SchedulerTickResult::StepLimitExceeded;
                    }
                    self.pending = Some(f());
                }
            }
        }
        SchedulerTickResult::Pending
    }
    /// Run to completion (ignoring the batch limit).
    #[allow(dead_code)]
    pub fn run_to_completion(mut self) -> Result<T, String> {
        loop {
            match self.tick() {
                SchedulerTickResult::Finished(v) => return Ok(v),
                SchedulerTickResult::StepLimitExceeded => {
                    return Err(format!("step limit {} exceeded", self.config.step_limit));
                }
                SchedulerTickResult::Pending => {}
            }
        }
    }
}
/// Configuration for loop unrolling.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct UnrollConfig {
    /// Factor by which to unroll the loop (e.g., 4 = 4 copies of the body).
    pub factor: usize,
    /// Maximum loop count to consider for full unrolling.
    pub full_unroll_limit: usize,
}
/// Result of loop unrolling analysis.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct UnrollResult {
    /// Whether the loop was fully unrolled.
    pub fully_unrolled: bool,
    /// Factor used.
    pub factor: usize,
    /// Estimated savings (iterations avoided).
    pub iterations_saved: usize,
}
impl UnrollResult {
    /// Compute unroll result for a loop with `n` iterations.
    #[allow(dead_code)]
    pub fn compute(n: usize, cfg: &UnrollConfig) -> Self {
        if n <= cfg.full_unroll_limit {
            UnrollResult {
                fully_unrolled: true,
                factor: n,
                iterations_saved: n,
            }
        } else {
            let factor = cfg.factor.min(n);
            let remainder = n % factor;
            let main_iters = n / factor;
            let saved = n - main_iters - remainder;
            UnrollResult {
                fully_unrolled: false,
                factor,
                iterations_saved: saved,
            }
        }
    }
}
/// A simple evaluation context that maps names to values.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct EvaluationContext {
    /// Variable bindings.
    pub bindings: std::collections::HashMap<String, u64>,
    /// Stack depth.
    pub depth: u32,
}
impl EvaluationContext {
    /// Create an empty context.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Bind a variable.
    #[allow(dead_code)]
    pub fn bind(&mut self, name: &str, value: u64) {
        self.bindings.insert(name.to_string(), value);
    }
    /// Look up a variable.
    #[allow(dead_code)]
    pub fn lookup(&self, name: &str) -> Option<u64> {
        self.bindings.get(name).copied()
    }
    /// Return a child context (incremented depth).
    #[allow(dead_code)]
    pub fn child(&self) -> Self {
        Self {
            bindings: self.bindings.clone(),
            depth: self.depth + 1,
        }
    }
    /// Number of bindings.
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.bindings.len()
    }
}
/// A simple peephole optimizer that applies rules to an opcode list.
#[allow(dead_code)]
pub struct PeepholeOptimizer {
    rules: Vec<PeepholeRule>,
    /// Number of rewrites applied in the last `optimize` call.
    pub rewrites: usize,
}
impl PeepholeOptimizer {
    /// Create an optimizer with no rules.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            rewrites: 0,
        }
    }
    /// Add a rule.
    #[allow(dead_code)]
    pub fn add_rule(&mut self, rule: PeepholeRule) {
        self.rules.push(rule);
    }
    /// Apply all rules to `opcodes`, returning the optimized sequence.
    #[allow(dead_code)]
    pub fn optimize<'a>(&mut self, opcodes: &[&'a str]) -> Vec<&'a str> {
        let mut result: Vec<&'a str> = opcodes.to_vec();
        let mut changed = true;
        self.rewrites = 0;
        while changed {
            changed = false;
            'outer: for rule in &self.rules {
                let plen = rule.pattern.len();
                if plen == 0 {
                    continue;
                }
                let mut i = 0;
                while i + plen <= result.len() {
                    if result[i..i + plen] == rule.pattern[..] {
                        let mut new_result = result[..i].to_vec();
                        new_result.extend_from_slice(&rule.replacement);
                        new_result.extend_from_slice(&result[i + plen..]);
                        result = new_result;
                        self.rewrites += 1;
                        changed = true;
                        continue 'outer;
                    }
                    i += 1;
                }
            }
        }
        result
    }
}
/// A simple heuristic for identifying tail-position `Call` instructions
/// in a flat bytecode sequence.
///
/// The rule is: a `Call` instruction is in tail position if it is
/// immediately followed by a `Return` instruction (with no intervening
/// instructions that produce stack values consumed after the return).
pub struct TailCallDetector {
    /// Positions (0-based) of instructions that are in tail position.
    pub tail_positions: Vec<usize>,
}
impl TailCallDetector {
    /// Create a new detector (no positions identified yet).
    pub fn new() -> Self {
        TailCallDetector {
            tail_positions: Vec::new(),
        }
    }
    /// Analyse a sequence of opcode names (strings) and populate
    /// [`tail_positions`](Self::tail_positions).
    ///
    /// The opcode names are strings like `"Call"`, `"Return"`, `"Halt"`,
    /// matching what [`crate::bytecode_interp::Opcode`] would produce via
    /// `Debug`. This keeps the detector decoupled from the specific
    /// `Opcode` enum variant structure.
    pub fn analyse(&mut self, opcode_names: &[&str]) {
        self.tail_positions.clear();
        for (i, name) in opcode_names.iter().enumerate() {
            if *name == "Call" {
                let next = opcode_names.get(i + 1).copied().unwrap_or("");
                if next == "Return" || next == "Halt" {
                    self.tail_positions.push(i);
                }
            }
        }
    }
    /// Returns `true` if position `idx` was identified as a tail call.
    pub fn is_tail(&self, idx: usize) -> bool {
        self.tail_positions.contains(&idx)
    }
    /// Number of tail-call positions found.
    pub fn count(&self) -> usize {
        self.tail_positions.len()
    }
}
/// Aggregate statistics for a batch of TCO-enabled computations.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct TcoStatistics {
    /// Total number of computations run.
    pub total_runs: u64,
    /// Total trampoline steps across all runs.
    pub total_steps: u64,
    /// Maximum depth seen in any single run.
    pub global_max_depth: u64,
    /// Number of runs that hit the step limit.
    pub step_limit_hits: u64,
}
impl TcoStatistics {
    /// Create zeroed statistics.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record the result of a single run.
    #[allow(dead_code)]
    pub fn record_run(&mut self, counter: &TailCallCounter, hit_limit: bool) {
        self.total_runs += 1;
        self.total_steps += counter.optimized;
        if counter.max_depth > self.global_max_depth {
            self.global_max_depth = counter.max_depth;
        }
        if hit_limit {
            self.step_limit_hits += 1;
        }
    }
    /// Average steps per run.
    #[allow(dead_code)]
    pub fn avg_steps(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.total_steps as f64 / self.total_runs as f64
        }
    }
    /// Format as a summary string.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "TcoStats: runs={}, total_steps={}, max_depth={}, limit_hits={}, avg_steps={:.2}",
            self.total_runs,
            self.total_steps,
            self.global_max_depth,
            self.step_limit_hits,
            self.avg_steps()
        )
    }
}
/// A simulated stack frame for explicit-stack recursion.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct StackFrame {
    /// The function name at this frame.
    pub function: String,
    /// Local variable bindings as `(name, value)` pairs.
    pub locals: Vec<(String, u64)>,
    /// The return address (index into a bytecode chunk).
    pub return_address: usize,
}
impl StackFrame {
    /// Create a new stack frame.
    #[allow(dead_code)]
    pub fn new(function: &str, return_address: usize) -> Self {
        Self {
            function: function.to_string(),
            locals: Vec::new(),
            return_address,
        }
    }
    /// Add a local variable binding.
    #[allow(dead_code)]
    pub fn bind(&mut self, name: &str, value: u64) {
        self.locals.push((name.to_string(), value));
    }
    /// Look up a local variable by name.
    #[allow(dead_code)]
    pub fn lookup(&self, name: &str) -> Option<u64> {
        self.locals
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, v)| *v)
    }
}
/// Threshold configuration for inlining decisions.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InliningThreshold {
    /// Maximum size (in opcodes) of a function that may be inlined.
    pub max_size: usize,
    /// Maximum call depth after which inlining is suppressed.
    pub max_depth: usize,
    /// Minimum call count before inlining is triggered.
    pub min_call_count: u64,
}
/// A builder that converts a two-argument accumulator-style recursion
/// into a trampolined computation.
///
/// ```
/// # use oxilean_runtime::tco::{RecursiveStep, trampoline};
/// // factorial via recursive step
/// let result = RecursiveStep::run(10u64, 1u64, |n, acc| {
///     if n == 0 { None } else { Some((n - 1, n * acc)) }
/// });
/// assert_eq!(result, 3628800);
/// ```
pub struct RecursiveStep;
impl RecursiveStep {
    /// Run the accumulator loop.
    ///
    /// - `initial`: the initial input value.
    /// - `acc`: the initial accumulator.
    /// - `step`: given `(input, acc)`, returns `Some((next_input, next_acc))`
    ///   to continue, or `None` to stop and return `acc`.
    pub fn run<I, A, F>(initial: I, acc: A, step: F) -> A
    where
        I: Clone + 'static,
        A: Clone + 'static,
        F: Fn(I, A) -> Option<(I, A)> + Clone + 'static,
    {
        fn go<
            I: Clone + 'static,
            A: Clone + 'static,
            F: Fn(I, A) -> Option<(I, A)> + Clone + 'static,
        >(
            i: I,
            a: A,
            f: F,
        ) -> TailCall<A> {
            match f(i, a.clone()) {
                None => TailCall::Done(a),
                Some((next_i, next_a)) => {
                    let f2 = f.clone();
                    TailCall::Call(Box::new(move || go(next_i, next_a, f2)))
                }
            }
        }
        trampoline(go(initial, acc, step))
    }
}
/// Kind of binary operation.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinopKind {
    Add,
    Sub,
    Mul,
    Div,
}
impl BinopKind {
    /// Evaluate this binary operation.
    #[allow(dead_code)]
    pub fn eval(self, lhs: u64, rhs: u64) -> Option<u64> {
        match self {
            BinopKind::Add => Some(lhs.wrapping_add(rhs)),
            BinopKind::Sub => Some(lhs.wrapping_sub(rhs)),
            BinopKind::Mul => Some(lhs.wrapping_mul(rhs)),
            BinopKind::Div => lhs.checked_div(rhs),
        }
    }
}
/// The result of a step in a trampoline loop.
///
/// A function that supports TCO returns `TailCall<T>` instead of `T`:
/// - [`TailCall::Done`] means the final value is ready.
/// - [`TailCall::Call`] means there is another step to execute.
pub enum TailCall<T> {
    /// Computation is finished; `T` is the final value.
    Done(T),
    /// Another step is needed; the boxed closure produces the next
    /// `TailCall<T>` without growing the call stack.
    Call(Box<dyn FnOnce() -> TailCall<T>>),
}
/// Result of a single interpreter step (for a TCO-aware interpreter loop).
pub enum StepResult<State, Output> {
    /// Continue with an updated state.
    Continue(State),
    /// Computation finished with this output.
    Finished(Output),
    /// An error occurred.
    Error(String),
}
/// High-level optimizer that applies TCO analysis to bytecode chunks.
#[allow(dead_code)]
pub struct TailCallOptimizer {
    detector: TailCallDetector,
    /// Statistics accumulator.
    pub stats: TailCallCounter,
}
impl TailCallOptimizer {
    /// Create a new optimizer.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            detector: TailCallDetector::new(),
            stats: TailCallCounter::new(),
        }
    }
    /// Analyse a chunk of opcodes and return the report.
    #[allow(dead_code)]
    pub fn analyse_chunk(&mut self, opcodes: &[&str]) -> TailCallAnalysisReport {
        self.detector.analyse(opcodes);
        let report = TailCallAnalysisReport::build(&self.detector, opcodes);
        self.stats.optimized += report.tail_positions.len() as u64;
        report
    }
    /// Returns `true` if the given index is a tail call.
    #[allow(dead_code)]
    pub fn is_tail_call(&self, idx: usize) -> bool {
        self.detector.is_tail(idx)
    }
}
/// Analysis result for a bytecode chunk, pairing detection results with stats.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TailCallAnalysisReport {
    /// Indices of tail-position call instructions.
    pub tail_positions: Vec<usize>,
    /// Total number of call instructions found.
    pub total_calls: usize,
    /// Fraction of calls that are in tail position.
    pub tail_ratio: f64,
    /// Human-readable summary.
    pub summary: String,
}
impl TailCallAnalysisReport {
    /// Build a report from a detector result and the original opcodes.
    #[allow(dead_code)]
    pub fn build(detector: &TailCallDetector, opcodes: &[&str]) -> Self {
        let total_calls = opcodes.iter().filter(|&&op| op == "Call").count();
        let tail_count = detector.count();
        let tail_ratio = if total_calls == 0 {
            0.0
        } else {
            tail_count as f64 / total_calls as f64
        };
        let summary = format!(
            "{}/{} calls ({:.0}%) are in tail position",
            tail_count,
            total_calls,
            tail_ratio * 100.0
        );
        Self {
            tail_positions: detector.tail_positions.clone(),
            total_calls,
            tail_ratio,
            summary,
        }
    }
}
/// Extended detector that classifies tail positions more finely.
#[allow(dead_code)]
pub struct ExtendedTailCallDetector {
    /// Current function being analyzed.
    pub current_function: String,
    /// Positions and their classifications.
    pub classified: Vec<(usize, TailPositionKind)>,
}
impl ExtendedTailCallDetector {
    /// Create a new detector for `function_name`.
    #[allow(dead_code)]
    pub fn new(function_name: &str) -> Self {
        Self {
            current_function: function_name.to_string(),
            classified: Vec::new(),
        }
    }
    /// Analyse a list of `(opcode_name, callee_name)` pairs.
    #[allow(dead_code)]
    pub fn analyse_with_callees(&mut self, ops: &[(&str, Option<&str>)]) {
        self.classified.clear();
        for (i, (op, callee)) in ops.iter().enumerate() {
            if *op != "Call" {
                continue;
            }
            let next = ops.get(i + 1).map(|(o, _)| *o).unwrap_or("");
            if next != "Return" && next != "Halt" {
                self.classified.push((i, TailPositionKind::NonTail));
                continue;
            }
            let kind = match *callee {
                Some(name) if name == self.current_function => TailPositionKind::SelfTailCall,
                Some(_) => TailPositionKind::MutualTailCall,
                None => TailPositionKind::ExternalTailCall,
            };
            self.classified.push((i, kind));
        }
    }
    /// Count how many positions are of the given kind.
    #[allow(dead_code)]
    pub fn count_kind(&self, kind: TailPositionKind) -> usize {
        self.classified.iter().filter(|(_, k)| *k == kind).count()
    }
}
/// More fine-grained classification of a tail position.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TailPositionKind {
    /// Direct self-tail-call (trivially TCO-able).
    SelfTailCall,
    /// Tail call to a sibling function (mutually recursive TCO).
    MutualTailCall,
    /// Tail call to an unknown/external function.
    ExternalTailCall,
    /// Not a tail call.
    NonTail,
}
/// A thunk: either an unevaluated computation or a memoized value.
#[allow(dead_code)]
pub enum Thunk<T> {
    /// Not yet evaluated.
    Deferred(Box<dyn FnOnce() -> T>),
    /// Already evaluated; value is cached.
    Evaluated(T),
}
impl<T: Clone> Thunk<T> {
    /// Create a deferred thunk.
    #[allow(dead_code)]
    pub fn defer(f: impl FnOnce() -> T + 'static) -> Self {
        Thunk::Deferred(Box::new(f))
    }
    /// Force evaluation and memoize the result.
    #[allow(dead_code)]
    pub fn force(&mut self) -> &T {
        match self {
            Thunk::Evaluated(ref v) => v,
            Thunk::Deferred(_) => {
                let Thunk::Deferred(f) =
                    std::mem::replace(self, Thunk::Evaluated(unsafe { std::mem::zeroed() }))
                else {
                    unreachable!()
                };
                let val = f();
                *self = Thunk::Evaluated(val);
                match self {
                    Thunk::Evaluated(ref v) => v,
                    _ => unreachable!(),
                }
            }
        }
    }
}
/// A simple symbolic rewrite rule.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RewriteRule {
    /// Pattern string (LHS).
    pub lhs: String,
    /// Replacement string (RHS).
    pub rhs: String,
    /// Whether this rule fires unconditionally.
    pub unconditional: bool,
}
impl RewriteRule {
    /// Create a new unconditional rewrite rule.
    #[allow(dead_code)]
    pub fn new(lhs: &str, rhs: &str) -> Self {
        Self {
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            unconditional: true,
        }
    }
    /// Create a conditional rewrite rule.
    #[allow(dead_code)]
    pub fn conditional(lhs: &str, rhs: &str) -> Self {
        Self {
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            unconditional: false,
        }
    }
}
/// Per-function trampoline metrics.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct FunctionTcoMetrics {
    /// Name of the function.
    pub name: String,
    /// Number of times this function was called via trampoline.
    pub call_count: u64,
    /// Maximum recursion depth eliminated for this function.
    pub max_depth_eliminated: u64,
    /// Total steps this function contributed.
    pub total_steps: u64,
}
impl FunctionTcoMetrics {
    /// Create metrics for a named function.
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
    /// Record a call at the given depth.
    #[allow(dead_code)]
    pub fn record(&mut self, depth: u64) {
        self.call_count += 1;
        self.total_steps += depth;
        if depth > self.max_depth_eliminated {
            self.max_depth_eliminated = depth;
        }
    }
    /// Average depth per call.
    #[allow(dead_code)]
    pub fn avg_depth(&self) -> f64 {
        if self.call_count == 0 {
            0.0
        } else {
            self.total_steps as f64 / self.call_count as f64
        }
    }
}
/// Represents a chain of tail calls that can potentially be fused.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TailCallChain {
    /// Ordered list of function names in the chain.
    pub functions: Vec<String>,
    /// Whether the chain can be safely fused into a single jump.
    pub can_fuse: bool,
}
impl TailCallChain {
    /// Create a new chain.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            can_fuse: true,
        }
    }
    /// Add a function to the chain.
    #[allow(dead_code)]
    pub fn push(&mut self, name: &str) {
        self.functions.push(name.to_string());
    }
    /// Mark as non-fusable.
    #[allow(dead_code)]
    pub fn mark_non_fusable(&mut self) {
        self.can_fuse = false;
    }
    /// Length of the chain.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.functions.len()
    }
    /// Whether the chain is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }
}
/// A full TCO optimization pass over a module's functions.
#[allow(dead_code)]
pub struct TailCallOptimizationPass {
    /// Optimizer instance.
    pub optimizer: TailCallOptimizer,
    /// Functions processed.
    pub processed: Vec<String>,
    /// Functions skipped (already TCO-safe or inlined).
    pub skipped: Vec<String>,
}
impl TailCallOptimizationPass {
    /// Create a new pass.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            optimizer: TailCallOptimizer::new(),
            processed: Vec::new(),
            skipped: Vec::new(),
        }
    }
    /// Process a function with the given bytecode opcode names.
    /// Returns the analysis report.
    #[allow(dead_code)]
    pub fn process_function(
        &mut self,
        name: &str,
        opcodes: &[&str],
        skip: bool,
    ) -> Option<TailCallAnalysisReport> {
        if skip {
            self.skipped.push(name.to_string());
            return None;
        }
        let report = self.optimizer.analyse_chunk(opcodes);
        self.processed.push(name.to_string());
        Some(report)
    }
    /// Summary of processing.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "TCO pass: {} processed, {} skipped, {} tail calls found",
            self.processed.len(),
            self.skipped.len(),
            self.optimizer.stats.optimized
        )
    }
}
/// Tracks how many tail calls were optimized during a run.
#[derive(Clone, Debug, Default)]
pub struct TailCallCounter {
    /// Total tail calls optimized (turned into loop iterations).
    pub optimized: u64,
    /// Maximum trampoline depth seen in a single evaluation.
    pub max_depth: u64,
}
impl TailCallCounter {
    /// Create a zeroed counter.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one optimized tail call at the given depth.
    pub fn record(&mut self, depth: u64) {
        self.optimized += 1;
        if depth > self.max_depth {
            self.max_depth = depth;
        }
    }
}
/// Decision on whether to inline a call.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InliningDecision {
    /// Inline the callee.
    Inline,
    /// Do not inline (call it normally).
    DoNotInline,
    /// Force inline regardless of heuristics (e.g., `#[inline(always)]`).
    ForceInline,
}
/// Controls the behaviour of a batched tail-call scheduler.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TailCallSchedulerConfig {
    /// Maximum number of trampoline steps per batch before yielding.
    pub max_steps_per_batch: usize,
    /// Maximum total steps allowed before returning an error.
    pub step_limit: u64,
}
/// Result of a tail-call micro-benchmark.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TailCallBenchmarkResult {
    /// Name of the benchmark.
    pub name: String,
    /// Number of iterations run.
    pub iterations: u64,
    /// Total wall-clock nanoseconds (stub: always 0 in tests).
    pub total_ns: u64,
    /// Final computed value.
    pub value: u64,
}
impl TailCallBenchmarkResult {
    /// Compute throughput in iterations per second (0 if total_ns == 0).
    #[allow(dead_code)]
    pub fn throughput(&self) -> f64 {
        if self.total_ns == 0 {
            0.0
        } else {
            self.iterations as f64 / (self.total_ns as f64 / 1_000_000_000.0)
        }
    }
    /// Format as a human-readable report line.
    #[allow(dead_code)]
    pub fn report(&self) -> String {
        format!(
            "Benchmark[{}]: {} iters, {} ns, value={}",
            self.name, self.iterations, self.total_ns, self.value
        )
    }
}
/// A peephole optimization rule: if the opcode pattern matches, replace it.
#[allow(dead_code)]
pub struct PeepholeRule {
    /// The opcode sequence to match (by name).
    pub pattern: Vec<&'static str>,
    /// The replacement opcode sequence.
    pub replacement: Vec<&'static str>,
    /// Human-readable description of the optimization.
    pub description: &'static str,
}
impl PeepholeRule {
    /// Create a new peephole rule.
    #[allow(dead_code)]
    pub fn new(
        pattern: Vec<&'static str>,
        replacement: Vec<&'static str>,
        description: &'static str,
    ) -> Self {
        Self {
            pattern,
            replacement,
            description,
        }
    }
}
/// A proof certificate that a function is TCO-safe.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TailCallProof {
    /// Name of the function proven TCO-safe.
    pub function_name: String,
    /// Which argument decreases on each call.
    pub decreasing_argument: String,
    /// Well-founded ordering used.
    pub ordering: String,
    /// Informal justification.
    pub justification: String,
}
impl TailCallProof {
    /// Create a proof certificate.
    #[allow(dead_code)]
    pub fn new(
        function_name: &str,
        decreasing_argument: &str,
        ordering: &str,
        justification: &str,
    ) -> Self {
        Self {
            function_name: function_name.to_string(),
            decreasing_argument: decreasing_argument.to_string(),
            ordering: ordering.to_string(),
            justification: justification.to_string(),
        }
    }
    /// Format the proof as a human-readable string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!(
            "TCO Proof for `{}`:\n  Decreasing: {}\n  Ordering: {}\n  Justification: {}",
            self.function_name, self.decreasing_argument, self.ordering, self.justification
        )
    }
}
/// Analysis result for a single bytecode instruction position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TailPosition {
    /// The instruction is in tail position (its result leaves the function).
    Tail,
    /// The instruction is not in tail position.
    NonTail,
}
/// An explicit call stack used when TCO cannot be applied.
#[allow(dead_code)]
pub struct ExplicitCallStack {
    frames: Vec<StackFrame>,
    /// Maximum depth ever reached.
    pub max_depth: usize,
}
impl ExplicitCallStack {
    /// Create an empty call stack.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            max_depth: 0,
        }
    }
    /// Push a new frame.
    #[allow(dead_code)]
    pub fn push(&mut self, frame: StackFrame) {
        self.frames.push(frame);
        if self.frames.len() > self.max_depth {
            self.max_depth = self.frames.len();
        }
    }
    /// Pop the top frame.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }
    /// Return the current depth.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Peek at the top frame without removing it.
    #[allow(dead_code)]
    pub fn top(&self) -> Option<&StackFrame> {
        self.frames.last()
    }
    /// Peek at the top frame mutably.
    #[allow(dead_code)]
    pub fn top_mut(&mut self) -> Option<&mut StackFrame> {
        self.frames.last_mut()
    }
    /// Return a backtrace as a list of function names.
    #[allow(dead_code)]
    pub fn backtrace(&self) -> Vec<&str> {
        self.frames.iter().map(|f| f.function.as_str()).collect()
    }
    /// Return a formatted backtrace string.
    #[allow(dead_code)]
    pub fn format_backtrace(&self) -> String {
        let mut out = String::from("Backtrace (most recent last):\n");
        for (i, frame) in self.frames.iter().enumerate() {
            out.push_str(&format!(
                "  {:3}: {} @ {}\n",
                i, frame.function, frame.return_address
            ));
        }
        out
    }
}
/// Result of a scheduler tick.
#[allow(dead_code)]
pub enum SchedulerTickResult<T> {
    /// The computation is not yet done.
    Pending,
    /// The computation finished with value `T`.
    Finished(T),
    /// The step limit was exceeded.
    StepLimitExceeded,
}
/// Represents a step of a mutually-recursive computation.
///
/// Two functions `even` and `odd` can be represented using a single
/// `MutualTailCall` enum, with `A` and `B` tagging which "side" we are on.
#[allow(dead_code)]
pub enum MutualTailCall<A, B, R> {
    /// Continue with the A-branch.
    GoA(A, Box<dyn FnOnce(A) -> MutualTailCall<A, B, R>>),
    /// Continue with the B-branch.
    GoB(B, Box<dyn FnOnce(B) -> MutualTailCall<A, B, R>>),
    /// Final result.
    Done(R),
}
/// An explicit continuation frame.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ContinuationFrame {
    /// Apply a binary operation to the top of the value stack and this operand.
    ApplyBinop { op: BinopKind, operand: u64 },
    /// Store the result in the named variable.
    StoreResult { var: String },
    /// Print the result.
    PrintResult,
}
/// The result of partially evaluating an expression.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum PartialValue {
    /// Known concrete value.
    Known(u64),
    /// Unknown symbolic value.
    Unknown(String),
    /// Bottom (error / undefined).
    Bottom,
}
impl PartialValue {
    /// Return true if the value is concrete.
    #[allow(dead_code)]
    pub fn is_known(&self) -> bool {
        matches!(self, PartialValue::Known(_))
    }
    /// Add two partial values.
    #[allow(dead_code)]
    pub fn add(&self, other: &PartialValue) -> PartialValue {
        match (self, other) {
            (PartialValue::Known(a), PartialValue::Known(b)) => {
                PartialValue::Known(a.wrapping_add(*b))
            }
            (PartialValue::Bottom, _) | (_, PartialValue::Bottom) => PartialValue::Bottom,
            _ => PartialValue::Unknown(format!("{:?} + {:?}", self, other)),
        }
    }
    /// Multiply two partial values.
    #[allow(dead_code)]
    pub fn mul(&self, other: &PartialValue) -> PartialValue {
        match (self, other) {
            (PartialValue::Known(a), PartialValue::Known(b)) => {
                PartialValue::Known(a.wrapping_mul(*b))
            }
            (PartialValue::Known(0), _) | (_, PartialValue::Known(0)) => PartialValue::Known(0),
            (PartialValue::Bottom, _) | (_, PartialValue::Bottom) => PartialValue::Bottom,
            _ => PartialValue::Unknown(format!("{:?} * {:?}", self, other)),
        }
    }
}
/// A state in an explicit finite state machine.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StateMachineState {
    /// Numeric state identifier.
    pub id: u32,
    /// Accumulated output collected during transitions.
    pub output: Vec<String>,
}
impl StateMachineState {
    /// Create a state with the given id and empty output.
    #[allow(dead_code)]
    pub fn new(id: u32) -> Self {
        Self {
            id,
            output: Vec::new(),
        }
    }
    /// Append a string to the accumulated output.
    #[allow(dead_code)]
    pub fn emit(mut self, s: &str) -> Self {
        self.output.push(s.to_string());
        self
    }
}
/// A continuation-based evaluator with an explicit continuation stack.
#[allow(dead_code)]
pub struct ContinuationEvaluator {
    /// Value stack.
    pub values: Vec<u64>,
    /// Continuation stack.
    pub continuations: Vec<ContinuationFrame>,
    /// Named variable bindings.
    pub env: std::collections::HashMap<String, u64>,
}
impl ContinuationEvaluator {
    /// Create a new evaluator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            continuations: Vec::new(),
            env: std::collections::HashMap::new(),
        }
    }
    /// Push a value onto the value stack.
    #[allow(dead_code)]
    pub fn push_value(&mut self, v: u64) {
        self.values.push(v);
    }
    /// Pop a value from the value stack.
    #[allow(dead_code)]
    pub fn pop_value(&mut self) -> Option<u64> {
        self.values.pop()
    }
    /// Push a continuation frame.
    #[allow(dead_code)]
    pub fn push_cont(&mut self, frame: ContinuationFrame) {
        self.continuations.push(frame);
    }
    /// Step: pop one continuation frame and apply it to the top value.
    #[allow(dead_code)]
    pub fn step(&mut self) -> bool {
        let frame = match self.continuations.pop() {
            Some(f) => f,
            None => return false,
        };
        match frame {
            ContinuationFrame::ApplyBinop { op, operand } => {
                if let Some(lhs) = self.values.pop() {
                    if let Some(result) = op.eval(lhs, operand) {
                        self.values.push(result);
                    }
                }
            }
            ContinuationFrame::StoreResult { var } => {
                if let Some(v) = self.values.last().copied() {
                    self.env.insert(var, v);
                }
            }
            ContinuationFrame::PrintResult => {}
        }
        true
    }
    /// Run to completion.
    #[allow(dead_code)]
    pub fn run(&mut self) {
        while self.step() {}
    }
}
