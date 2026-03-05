//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Expr;
use std::time::{Duration, Instant};

use std::collections::HashMap;

/// A versioned record that stores a history of values.
#[allow(dead_code)]
pub struct VersionedRecord<T: Clone> {
    history: Vec<T>,
}
#[allow(dead_code)]
impl<T: Clone> VersionedRecord<T> {
    /// Creates a new record with an initial value.
    pub fn new(initial: T) -> Self {
        Self {
            history: vec![initial],
        }
    }
    /// Updates the record with a new version.
    pub fn update(&mut self, val: T) {
        self.history.push(val);
    }
    /// Returns the current (latest) value.
    pub fn current(&self) -> &T {
        self.history
            .last()
            .expect("VersionedRecord history is always non-empty after construction")
    }
    /// Returns the value at version `n` (0-indexed), or `None`.
    pub fn at_version(&self, n: usize) -> Option<&T> {
        self.history.get(n)
    }
    /// Returns the version number of the current value.
    pub fn version(&self) -> usize {
        self.history.len() - 1
    }
    /// Returns `true` if more than one version exists.
    pub fn has_history(&self) -> bool {
        self.history.len() > 1
    }
}
/// A pair of `StatSummary` values tracking before/after a transformation.
#[allow(dead_code)]
pub struct TransformStat {
    before: StatSummary,
    after: StatSummary,
}
#[allow(dead_code)]
impl TransformStat {
    /// Creates a new transform stat recorder.
    pub fn new() -> Self {
        Self {
            before: StatSummary::new(),
            after: StatSummary::new(),
        }
    }
    /// Records a before value.
    pub fn record_before(&mut self, v: f64) {
        self.before.record(v);
    }
    /// Records an after value.
    pub fn record_after(&mut self, v: f64) {
        self.after.record(v);
    }
    /// Returns the mean reduction ratio (after/before).
    pub fn mean_ratio(&self) -> Option<f64> {
        let b = self.before.mean()?;
        let a = self.after.mean()?;
        if b.abs() < f64::EPSILON {
            return None;
        }
        Some(a / b)
    }
}
/// Tracer for collecting debugging information during type checking.
pub struct Tracer {
    level: TraceLevel,
    events: Vec<TraceEvent>,
    max_events: usize,
    reduction_steps: Vec<ReductionStep>,
    depth: u32,
    timing_enabled: bool,
    start_time: Option<Instant>,
    suppressed_categories: Vec<TraceCategory>,
}
impl Tracer {
    /// Create a new tracer with the given level.
    pub fn new(level: TraceLevel) -> Self {
        Self {
            level,
            events: Vec::new(),
            max_events: 1000,
            reduction_steps: Vec::new(),
            depth: 0,
            timing_enabled: false,
            start_time: None,
            suppressed_categories: Vec::new(),
        }
    }
    /// Create a silent tracer.
    pub fn silent() -> Self {
        Self::new(TraceLevel::Off)
    }
    /// Create a fully verbose tracer.
    pub fn verbose() -> Self {
        Self::new(TraceLevel::Trace)
    }
    /// Set the trace level.
    pub fn set_level(&mut self, level: TraceLevel) {
        self.level = level;
    }
    /// Get the trace level.
    pub fn level(&self) -> TraceLevel {
        self.level
    }
    /// Set the max events buffer size.
    pub fn set_max_events(&mut self, max: usize) {
        self.max_events = max;
    }
    /// Enable or disable timing.
    pub fn set_timing(&mut self, enabled: bool) {
        self.timing_enabled = enabled;
        self.start_time = if enabled { Some(Instant::now()) } else { None };
    }
    /// Suppress a category.
    pub fn suppress(&mut self, cat: TraceCategory) {
        if !self.suppressed_categories.contains(&cat) {
            self.suppressed_categories.push(cat);
        }
    }
    /// Allow a category.
    pub fn allow(&mut self, cat: &TraceCategory) {
        self.suppressed_categories.retain(|c| c != cat);
    }
    /// Log a trace event.
    pub fn log(&mut self, mut event: TraceEvent) {
        if event.level > self.level {
            return;
        }
        if let Some(cat) = &event.category {
            if self.suppressed_categories.contains(cat) {
                return;
            }
        }
        event.depth = self.depth;
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push(event);
    }
    /// Log a string at the given level.
    pub fn log_msg(&mut self, level: TraceLevel, msg: impl Into<String>) {
        self.log(TraceEvent::new(level, msg.into()));
    }
    /// Log an error.
    pub fn error(&mut self, msg: impl Into<String>) {
        self.log_msg(TraceLevel::Error, msg);
    }
    /// Log a warning.
    pub fn warn(&mut self, msg: impl Into<String>) {
        self.log_msg(TraceLevel::Warn, msg);
    }
    /// Log an info message.
    pub fn info(&mut self, msg: impl Into<String>) {
        self.log_msg(TraceLevel::Info, msg);
    }
    /// Log a debug message.
    pub fn debug(&mut self, msg: impl Into<String>) {
        self.log_msg(TraceLevel::Debug, msg);
    }
    /// Log a trace message.
    pub fn trace_msg(&mut self, msg: impl Into<String>) {
        self.log_msg(TraceLevel::Trace, msg);
    }
    /// Log a trace-level message in the Infer category.
    pub fn trace_infer(&mut self, msg: impl Into<String>) {
        self.log(
            TraceEvent::new(TraceLevel::Trace, msg.into()).with_category(TraceCategory::Infer),
        );
    }
    /// Log a trace-level message in the Reduce category.
    pub fn trace_reduce(&mut self, msg: impl Into<String>) {
        self.log(
            TraceEvent::new(TraceLevel::Trace, msg.into()).with_category(TraceCategory::Reduce),
        );
    }
    /// Record a reduction step.
    pub fn record_reduction(&mut self, rule: ReductionRule, before: Expr, after: Expr) {
        if self.level >= TraceLevel::Trace {
            self.reduction_steps.push(ReductionStep {
                rule,
                before,
                after,
            });
        }
    }
    /// Get reduction steps.
    pub fn reduction_steps(&self) -> &[ReductionStep] {
        &self.reduction_steps
    }
    /// Clear reduction steps.
    pub fn clear_reductions(&mut self) {
        self.reduction_steps.clear();
    }
    /// Get all logged events.
    pub fn events(&self) -> &[TraceEvent] {
        &self.events
    }
    /// Get events at a specific level.
    pub fn events_at_level(&self, level: TraceLevel) -> Vec<&TraceEvent> {
        self.events.iter().filter(|e| e.level <= level).collect()
    }
    /// Get events in a specific category.
    pub fn events_in_category(&self, cat: &TraceCategory) -> Vec<&TraceEvent> {
        self.events
            .iter()
            .filter(|e| e.category.as_ref() == Some(cat))
            .collect()
    }
    /// Clear all events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
    /// Count events.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
    /// Push depth.
    pub fn push(&mut self) {
        self.depth += 1;
    }
    /// Pop depth.
    pub fn pop(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
    /// Get current depth.
    pub fn current_depth(&self) -> u32 {
        self.depth
    }
    /// Get elapsed time since tracing started.
    pub fn elapsed(&self) -> Option<Duration> {
        self.start_time.map(|t| t.elapsed())
    }
    /// Render all events as a multi-line string.
    pub fn render(&self) -> String {
        self.events
            .iter()
            .map(|e| e.format())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// Extension trait for `Tracer` providing convenience methods.
impl Tracer {
    /// Log and return a message at Debug level.
    pub fn debug_return<T>(&mut self, value: T, msg: impl Into<String>) -> T {
        self.debug(msg);
        value
    }
    /// Log an expression at Debug level.
    pub fn debug_expr(&mut self, label: &str, expr: &Expr) {
        self.debug(format!("{}: {:?}", label, expr));
    }
    /// Compute statistics for this tracer.
    pub fn stats(&self) -> TracerStats {
        TracerStats::compute(self)
    }
    /// Return the most recent event, if any.
    pub fn last_event(&self) -> Option<&TraceEvent> {
        self.events().last()
    }
    /// Return the most recent error event, if any.
    pub fn last_error(&self) -> Option<&TraceEvent> {
        self.events()
            .iter()
            .rev()
            .find(|e| e.level == TraceLevel::Error)
    }
    /// Check whether the tracer has any events.
    pub fn is_empty(&self) -> bool {
        self.events().is_empty()
    }
    /// Log with a specific category.
    pub fn log_with_category(
        &mut self,
        level: TraceLevel,
        category: TraceCategory,
        msg: impl Into<String>,
    ) {
        self.log(TraceEvent::new(level, msg.into()).with_category(category));
    }
    /// Log an infer event at Debug level.
    pub fn log_infer(&mut self, msg: impl Into<String>) {
        self.log_with_category(TraceLevel::Debug, TraceCategory::Infer, msg);
    }
    /// Log a simp event at Trace level.
    pub fn log_simp(&mut self, msg: impl Into<String>) {
        self.log_with_category(TraceLevel::Trace, TraceCategory::Simp, msg);
    }
    /// Log a tactic event at Info level.
    pub fn log_tactic(&mut self, msg: impl Into<String>) {
        self.log_with_category(TraceLevel::Info, TraceCategory::Tactic, msg);
    }
    /// Log an elaboration event at Debug level.
    pub fn log_elab(&mut self, msg: impl Into<String>) {
        self.log_with_category(TraceLevel::Debug, TraceCategory::Elab, msg);
    }
    /// Return all events in a given category as formatted strings.
    pub fn category_log(&self, cat: &TraceCategory) -> Vec<String> {
        self.events_in_category(cat)
            .iter()
            .map(|e| e.format())
            .collect()
    }
    /// Clear only events in a given category.
    pub fn clear_category(&mut self, cat: &TraceCategory) {
        self.info(format!("[cleared category {}]", cat));
    }
}
/// Represents a rewrite rule `lhs → rhs`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RewriteRule {
    /// The name of the rule.
    pub name: String,
    /// A string representation of the LHS pattern.
    pub lhs: String,
    /// A string representation of the RHS.
    pub rhs: String,
    /// Whether this is a conditional rule (has side conditions).
    pub conditional: bool,
}
#[allow(dead_code)]
impl RewriteRule {
    /// Creates an unconditional rewrite rule.
    pub fn unconditional(
        name: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
            conditional: false,
        }
    }
    /// Creates a conditional rewrite rule.
    pub fn conditional(
        name: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
            conditional: true,
        }
    }
    /// Returns a textual representation.
    pub fn display(&self) -> String {
        format!("{}: {} → {}", self.name, self.lhs, self.rhs)
    }
}
/// A simple stack-based calculator for arithmetic expressions.
#[allow(dead_code)]
pub struct StackCalc {
    stack: Vec<i64>,
}
#[allow(dead_code)]
impl StackCalc {
    /// Creates a new empty calculator.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    /// Pushes an integer literal.
    pub fn push(&mut self, n: i64) {
        self.stack.push(n);
    }
    /// Adds the top two values.  Panics if fewer than two values.
    pub fn add(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for add");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for add");
        self.stack.push(a + b);
    }
    /// Subtracts top from second.
    pub fn sub(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for sub");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for sub");
        self.stack.push(a - b);
    }
    /// Multiplies the top two values.
    pub fn mul(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for mul");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for mul");
        self.stack.push(a * b);
    }
    /// Peeks the top value.
    pub fn peek(&self) -> Option<i64> {
        self.stack.last().copied()
    }
    /// Returns the stack depth.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}
/// A hierarchical span for structured tracing.
pub struct TraceSpan<'a> {
    pub(crate) tracer: &'a mut Tracer,
    pub(crate) name: &'static str,
}
impl<'a> TraceSpan<'a> {
    /// Open a new span, pushing depth.
    pub fn open(tracer: &'a mut Tracer, name: &'static str) -> Self {
        tracer.info(format!(">>> {}", name));
        tracer.push();
        Self { tracer, name }
    }
    /// Log within the span.
    pub fn log(&mut self, msg: impl Into<String>) {
        self.tracer.info(msg);
    }
    /// Close the span, popping depth.
    pub fn close(self) {}
}
/// A mutable reference stack for tracking the current "focus" in a tree traversal.
#[allow(dead_code)]
pub struct FocusStack<T> {
    items: Vec<T>,
}
#[allow(dead_code)]
impl<T> FocusStack<T> {
    /// Creates an empty focus stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Focuses on `item`.
    pub fn focus(&mut self, item: T) {
        self.items.push(item);
    }
    /// Blurs (pops) the current focus.
    pub fn blur(&mut self) -> Option<T> {
        self.items.pop()
    }
    /// Returns the current focus, or `None`.
    pub fn current(&self) -> Option<&T> {
        self.items.last()
    }
    /// Returns the focus depth.
    pub fn depth(&self) -> usize {
        self.items.len()
    }
    /// Returns `true` if there is no current focus.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
/// A non-empty list (at least one element guaranteed).
#[allow(dead_code)]
pub struct NonEmptyVec<T> {
    head: T,
    tail: Vec<T>,
}
#[allow(dead_code)]
impl<T> NonEmptyVec<T> {
    /// Creates a non-empty vec with a single element.
    pub fn singleton(val: T) -> Self {
        Self {
            head: val,
            tail: Vec::new(),
        }
    }
    /// Pushes an element.
    pub fn push(&mut self, val: T) {
        self.tail.push(val);
    }
    /// Returns a reference to the first element.
    pub fn first(&self) -> &T {
        &self.head
    }
    /// Returns a reference to the last element.
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
    }
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns all elements as a Vec.
    pub fn to_vec(&self) -> Vec<&T> {
        let mut v = vec![&self.head];
        v.extend(self.tail.iter());
        v
    }
}
/// A flat list of substitution pairs `(from, to)`.
#[allow(dead_code)]
pub struct FlatSubstitution {
    pairs: Vec<(String, String)>,
}
#[allow(dead_code)]
impl FlatSubstitution {
    /// Creates an empty substitution.
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }
    /// Adds a pair.
    pub fn add(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.pairs.push((from.into(), to.into()));
    }
    /// Applies all substitutions to `s` (leftmost-first order).
    pub fn apply(&self, s: &str) -> String {
        let mut result = s.to_string();
        for (from, to) in &self.pairs {
            result = result.replace(from.as_str(), to.as_str());
        }
        result
    }
    /// Returns the number of pairs.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}
/// A filter for trace events.
#[derive(Clone, Debug)]
pub struct TraceFilter {
    /// Minimum level to include.
    pub min_level: TraceLevel,
    /// If set, only include events in these categories.
    pub categories: Option<Vec<TraceCategory>>,
    /// If set, exclude events matching this text.
    pub exclude_text: Option<String>,
}
impl TraceFilter {
    /// Create a filter that accepts everything at the given level.
    pub fn at_level(level: TraceLevel) -> Self {
        Self {
            min_level: level,
            categories: None,
            exclude_text: None,
        }
    }
    /// Restrict to specific categories.
    pub fn with_categories(mut self, cats: Vec<TraceCategory>) -> Self {
        self.categories = Some(cats);
        self
    }
    /// Exclude events containing specific text.
    pub fn excluding(mut self, text: impl Into<String>) -> Self {
        self.exclude_text = Some(text.into());
        self
    }
    /// Check if a trace event passes this filter.
    ///
    /// An event is accepted when its level is at most `min_level` in verbosity
    /// (i.e., at least as important / severe as `min_level`). More verbose
    /// events (Debug, Trace) are filtered out when `min_level` is lower
    /// (e.g., Info).
    pub fn accepts(&self, event: &TraceEvent) -> bool {
        if event.level > self.min_level {
            return false;
        }
        if let Some(ref cats) = self.categories {
            if let Some(ref ec) = event.category {
                if !cats.contains(ec) {
                    return false;
                }
            } else {
                return false;
            }
        }
        if let Some(ref excl) = self.exclude_text {
            if event.message.contains(excl.as_str()) {
                return false;
            }
        }
        true
    }
}
/// A fixed-size sliding window that computes a running sum.
#[allow(dead_code)]
pub struct SlidingSum {
    window: Vec<f64>,
    capacity: usize,
    pos: usize,
    sum: f64,
    count: usize,
}
#[allow(dead_code)]
impl SlidingSum {
    /// Creates a sliding sum with the given window size.
    pub fn new(capacity: usize) -> Self {
        Self {
            window: vec![0.0; capacity],
            capacity,
            pos: 0,
            sum: 0.0,
            count: 0,
        }
    }
    /// Adds a value to the window, removing the oldest if full.
    pub fn push(&mut self, val: f64) {
        let oldest = self.window[self.pos];
        self.sum -= oldest;
        self.sum += val;
        self.window[self.pos] = val;
        self.pos = (self.pos + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }
    /// Returns the current window sum.
    pub fn sum(&self) -> f64 {
        self.sum
    }
    /// Returns the window mean, or `None` if empty.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
    /// Returns the current window size (number of valid elements).
    pub fn count(&self) -> usize {
        self.count
    }
}
/// A reduction step recorded during tracing.
#[derive(Debug, Clone)]
pub struct ReductionStep {
    /// Reduction rule applied.
    pub rule: ReductionRule,
    /// Expression before reduction.
    pub before: Expr,
    /// Expression after reduction.
    pub after: Expr,
}
/// A window iterator that yields overlapping windows of size `n`.
#[allow(dead_code)]
pub struct WindowIterator<'a, T> {
    pub(super) data: &'a [T],
    pub(super) pos: usize,
    pub(super) window: usize,
}
#[allow(dead_code)]
impl<'a, T> WindowIterator<'a, T> {
    /// Creates a new window iterator.
    pub fn new(data: &'a [T], window: usize) -> Self {
        Self {
            data,
            pos: 0,
            window,
        }
    }
}
/// A ring buffer for trace events that overwrites the oldest entries.
pub struct RingTracer {
    buffer: Vec<TraceEvent>,
    capacity: usize,
    head: usize,
    count: usize,
    level: TraceLevel,
}
impl RingTracer {
    /// Create a ring tracer with the given capacity and level.
    pub fn new(capacity: usize, level: TraceLevel) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
            head: 0,
            count: 0,
            level,
        }
    }
    /// Log a trace event (overwrites oldest if full).
    pub fn log(&mut self, event: TraceEvent) {
        if event.level > self.level {
            return;
        }
        if self.buffer.len() < self.capacity {
            self.buffer.push(event);
        } else {
            self.buffer[self.head] = event;
            self.head = (self.head + 1) % self.capacity;
        }
        self.count += 1;
    }
    /// Log a message at the given level.
    pub fn log_msg(&mut self, level: TraceLevel, msg: impl Into<String>) {
        self.log(TraceEvent::new(level, msg.into()));
    }
    /// Return the number of events currently stored.
    pub fn stored_count(&self) -> usize {
        self.buffer.len()
    }
    /// Return the total number of events logged (including overwritten ones).
    pub fn total_count(&self) -> usize {
        self.count
    }
    /// Clear all stored events.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.head = 0;
        self.count = 0;
    }
    /// Drain events in order (oldest first).
    pub fn drain_ordered(&self) -> Vec<&TraceEvent> {
        let n = self.buffer.len();
        if n < self.capacity || self.count <= self.capacity {
            self.buffer.iter().collect()
        } else {
            let start = self.head;
            let mut result = Vec::with_capacity(n);
            for i in 0..n {
                result.push(&self.buffer[(start + i) % n]);
            }
            result
        }
    }
}
/// A label set for a graph node.
#[allow(dead_code)]
pub struct LabelSet {
    labels: Vec<String>,
}
#[allow(dead_code)]
impl LabelSet {
    /// Creates a new empty label set.
    pub fn new() -> Self {
        Self { labels: Vec::new() }
    }
    /// Adds a label (deduplicates).
    pub fn add(&mut self, label: impl Into<String>) {
        let s = label.into();
        if !self.labels.contains(&s) {
            self.labels.push(s);
        }
    }
    /// Returns `true` if `label` is present.
    pub fn has(&self, label: &str) -> bool {
        self.labels.iter().any(|l| l == label)
    }
    /// Returns the count of labels.
    pub fn count(&self) -> usize {
        self.labels.len()
    }
    /// Returns all labels.
    pub fn all(&self) -> &[String] {
        &self.labels
    }
}
/// A set of rewrite rules.
#[allow(dead_code)]
pub struct RewriteRuleSet {
    rules: Vec<RewriteRule>,
}
#[allow(dead_code)]
impl RewriteRuleSet {
    /// Creates an empty rule set.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    /// Adds a rule.
    pub fn add(&mut self, rule: RewriteRule) {
        self.rules.push(rule);
    }
    /// Returns the number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Returns `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Returns all conditional rules.
    pub fn conditional_rules(&self) -> Vec<&RewriteRule> {
        self.rules.iter().filter(|r| r.conditional).collect()
    }
    /// Returns all unconditional rules.
    pub fn unconditional_rules(&self) -> Vec<&RewriteRule> {
        self.rules.iter().filter(|r| !r.conditional).collect()
    }
    /// Looks up a rule by name.
    pub fn get(&self, name: &str) -> Option<&RewriteRule> {
        self.rules.iter().find(|r| r.name == name)
    }
}
/// A counter that can measure elapsed time between snapshots.
#[allow(dead_code)]
pub struct Stopwatch {
    start: std::time::Instant,
    splits: Vec<f64>,
}
#[allow(dead_code)]
impl Stopwatch {
    /// Creates and starts a new stopwatch.
    pub fn start() -> Self {
        Self {
            start: std::time::Instant::now(),
            splits: Vec::new(),
        }
    }
    /// Records a split time (elapsed since start).
    pub fn split(&mut self) {
        self.splits.push(self.elapsed_ms());
    }
    /// Returns total elapsed milliseconds since start.
    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }
    /// Returns all recorded split times.
    pub fn splits(&self) -> &[f64] {
        &self.splits
    }
    /// Returns the number of splits.
    pub fn num_splits(&self) -> usize {
        self.splits.len()
    }
}
/// Category of trace event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceCategory {
    /// Kernel type inference.
    Infer,
    /// Definitional equality checking.
    DefEq,
    /// Reduction / normalization.
    Reduce,
    /// Unification.
    Unify,
    /// Tactic execution.
    Tactic,
    /// Elaboration.
    Elab,
    /// Typeclass resolution.
    Typeclass,
    /// Simp tactic.
    Simp,
    /// User-defined category.
    Custom(String),
}
/// A sparse vector: stores only non-default elements.
#[allow(dead_code)]
pub struct SparseVec<T: Default + Clone + PartialEq> {
    entries: std::collections::HashMap<usize, T>,
    default_: T,
    logical_len: usize,
}
#[allow(dead_code)]
impl<T: Default + Clone + PartialEq> SparseVec<T> {
    /// Creates a new sparse vector with logical length `len`.
    pub fn new(len: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            default_: T::default(),
            logical_len: len,
        }
    }
    /// Sets element at `idx`.
    pub fn set(&mut self, idx: usize, val: T) {
        if val == self.default_ {
            self.entries.remove(&idx);
        } else {
            self.entries.insert(idx, val);
        }
    }
    /// Gets element at `idx`.
    pub fn get(&self, idx: usize) -> &T {
        self.entries.get(&idx).unwrap_or(&self.default_)
    }
    /// Returns the logical length.
    pub fn len(&self) -> usize {
        self.logical_len
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the number of non-default elements.
    pub fn nnz(&self) -> usize {
        self.entries.len()
    }
}
/// A reusable scratch buffer for path computations.
#[allow(dead_code)]
pub struct PathBuf {
    components: Vec<String>,
}
#[allow(dead_code)]
impl PathBuf {
    /// Creates a new empty path buffer.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    /// Pushes a component.
    pub fn push(&mut self, comp: impl Into<String>) {
        self.components.push(comp.into());
    }
    /// Pops the last component.
    pub fn pop(&mut self) {
        self.components.pop();
    }
    /// Returns the current path as a `/`-separated string.
    pub fn as_str(&self) -> String {
        self.components.join("/")
    }
    /// Returns the depth of the path.
    pub fn depth(&self) -> usize {
        self.components.len()
    }
    /// Clears the path.
    pub fn clear(&mut self) {
        self.components.clear();
    }
}
/// A dependency closure builder (transitive closure via BFS).
#[allow(dead_code)]
pub struct TransitiveClosure {
    adj: Vec<Vec<usize>>,
    n: usize,
}
#[allow(dead_code)]
impl TransitiveClosure {
    /// Creates a transitive closure builder for `n` nodes.
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
            n,
        }
    }
    /// Adds a direct edge.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n {
            self.adj[from].push(to);
        }
    }
    /// Computes all nodes reachable from `start` (including `start`).
    pub fn reachable_from(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            if node >= self.n || visited[node] {
                continue;
            }
            visited[node] = true;
            for &next in &self.adj[node] {
                queue.push_back(next);
            }
        }
        (0..self.n).filter(|&i| visited[i]).collect()
    }
    /// Returns `true` if `from` can transitively reach `to`.
    pub fn can_reach(&self, from: usize, to: usize) -> bool {
        self.reachable_from(from).contains(&to)
    }
}
/// A simple decision tree node for rule dispatching.
#[allow(dead_code)]
#[allow(missing_docs)]
pub enum DecisionNode {
    /// A leaf with an action string.
    Leaf(String),
    /// An interior node: check `key` equals `val` → `yes_branch`, else `no_branch`.
    Branch {
        key: String,
        val: String,
        yes_branch: Box<DecisionNode>,
        no_branch: Box<DecisionNode>,
    },
}
#[allow(dead_code)]
impl DecisionNode {
    /// Evaluates the decision tree with the given context.
    pub fn evaluate(&self, ctx: &std::collections::HashMap<String, String>) -> &str {
        match self {
            DecisionNode::Leaf(action) => action.as_str(),
            DecisionNode::Branch {
                key,
                val,
                yes_branch,
                no_branch,
            } => {
                let actual = ctx.get(key).map(|s| s.as_str()).unwrap_or("");
                if actual == val.as_str() {
                    yes_branch.evaluate(ctx)
                } else {
                    no_branch.evaluate(ctx)
                }
            }
        }
    }
    /// Returns the depth of the decision tree.
    pub fn depth(&self) -> usize {
        match self {
            DecisionNode::Leaf(_) => 0,
            DecisionNode::Branch {
                yes_branch,
                no_branch,
                ..
            } => 1 + yes_branch.depth().max(no_branch.depth()),
        }
    }
}
/// A tracer that also forwards events to a callback.
pub struct ForwardingTracer<F: FnMut(&TraceEvent)> {
    inner: Tracer,
    callback: F,
}
impl<F: FnMut(&TraceEvent)> ForwardingTracer<F> {
    /// Create a new forwarding tracer.
    pub fn new(level: TraceLevel, callback: F) -> Self {
        Self {
            inner: Tracer::new(level),
            callback,
        }
    }
    /// Log an event (forwarding to callback as well).
    pub fn log(&mut self, event: TraceEvent) {
        (self.callback)(&event);
        self.inner.log(event);
    }
    /// Get events.
    pub fn events(&self) -> &[TraceEvent] {
        self.inner.events()
    }
}
/// A hierarchical configuration tree.
#[allow(dead_code)]
pub struct ConfigNode {
    key: String,
    value: Option<String>,
    children: Vec<ConfigNode>,
}
#[allow(dead_code)]
impl ConfigNode {
    /// Creates a leaf config node with a value.
    pub fn leaf(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: Some(value.into()),
            children: Vec::new(),
        }
    }
    /// Creates a section node with children.
    pub fn section(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
            children: Vec::new(),
        }
    }
    /// Adds a child node.
    pub fn add_child(&mut self, child: ConfigNode) {
        self.children.push(child);
    }
    /// Returns the key.
    pub fn key(&self) -> &str {
        &self.key
    }
    /// Returns the value, or `None` for section nodes.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
    /// Returns the number of children.
    pub fn num_children(&self) -> usize {
        self.children.len()
    }
    /// Looks up a dot-separated path.
    pub fn lookup(&self, path: &str) -> Option<&str> {
        let mut parts = path.splitn(2, '.');
        let head = parts.next()?;
        let tail = parts.next();
        if head != self.key {
            return None;
        }
        match tail {
            None => self.value.as_deref(),
            Some(rest) => self.children.iter().find_map(|c| c.lookup_relative(rest)),
        }
    }
    fn lookup_relative(&self, path: &str) -> Option<&str> {
        let mut parts = path.splitn(2, '.');
        let head = parts.next()?;
        let tail = parts.next();
        if head != self.key {
            return None;
        }
        match tail {
            None => self.value.as_deref(),
            Some(rest) => self.children.iter().find_map(|c| c.lookup_relative(rest)),
        }
    }
}
/// A type-erased function pointer with arity tracking.
#[allow(dead_code)]
pub struct RawFnPtr {
    /// The raw function pointer (stored as usize for type erasure).
    ptr: usize,
    arity: usize,
    name: String,
}
#[allow(dead_code)]
impl RawFnPtr {
    /// Creates a new raw function pointer descriptor.
    pub fn new(ptr: usize, arity: usize, name: impl Into<String>) -> Self {
        Self {
            ptr,
            arity,
            name: name.into(),
        }
    }
    /// Returns the arity.
    pub fn arity(&self) -> usize {
        self.arity
    }
    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the raw pointer value.
    pub fn raw(&self) -> usize {
        self.ptr
    }
}
/// A pool of reusable string buffers.
#[allow(dead_code)]
pub struct StringPool {
    free: Vec<String>,
}
#[allow(dead_code)]
impl StringPool {
    /// Creates a new empty string pool.
    pub fn new() -> Self {
        Self { free: Vec::new() }
    }
    /// Takes a string from the pool (may be empty).
    pub fn take(&mut self) -> String {
        self.free.pop().unwrap_or_default()
    }
    /// Returns a string to the pool.
    pub fn give(&mut self, mut s: String) {
        s.clear();
        self.free.push(s);
    }
    /// Returns the number of free strings in the pool.
    pub fn free_count(&self) -> usize {
        self.free.len()
    }
}
/// Kind of reduction rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReductionRule {
    /// Beta reduction.
    Beta,
    /// Delta reduction (definition unfolding).
    Delta,
    /// Iota reduction (recursion / match).
    Iota,
    /// Zeta reduction (let unfolding).
    Zeta,
    /// Eta reduction.
    Eta,
    /// Quotient reduction.
    Quot,
}
/// A single trace event during type checking or elaboration.
#[derive(Debug, Clone)]
pub struct TraceEvent {
    /// Severity / verbosity level.
    pub level: TraceLevel,
    /// Human-readable message.
    pub message: String,
    /// Expression associated with this event.
    pub expr: Option<Expr>,
    /// Location string.
    pub location: String,
    /// Category.
    pub category: Option<TraceCategory>,
    /// Depth in the type-checking stack.
    pub depth: u32,
}
impl TraceEvent {
    /// Create a minimal trace event.
    pub fn new(level: TraceLevel, message: String) -> Self {
        Self {
            level,
            message,
            expr: None,
            location: String::new(),
            category: None,
            depth: 0,
        }
    }
    /// Attach an expression.
    pub fn with_expr(mut self, expr: Expr) -> Self {
        self.expr = Some(expr);
        self
    }
    /// Set the location.
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = location.into();
        self
    }
    /// Set the category.
    pub fn with_category(mut self, category: TraceCategory) -> Self {
        self.category = Some(category);
        self
    }
    /// Set the depth.
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }
    /// Format as a log line.
    pub fn format(&self) -> String {
        let cat = self
            .category
            .as_ref()
            .map(|c| format!("[{c}] "))
            .unwrap_or_default();
        let loc = if self.location.is_empty() {
            String::new()
        } else {
            format!(" ({})", self.location)
        };
        let indent = "  ".repeat(self.depth as usize);
        format!("{}{} {}{}{}", indent, self.level, cat, self.message, loc)
    }
}
/// A write-once cell.
#[allow(dead_code)]
pub struct WriteOnce<T> {
    value: std::cell::Cell<Option<T>>,
}
#[allow(dead_code)]
impl<T: Copy> WriteOnce<T> {
    /// Creates an empty write-once cell.
    pub fn new() -> Self {
        Self {
            value: std::cell::Cell::new(None),
        }
    }
    /// Writes a value.  Returns `false` if already written.
    pub fn write(&self, val: T) -> bool {
        if self.value.get().is_some() {
            return false;
        }
        self.value.set(Some(val));
        true
    }
    /// Returns the value if written.
    pub fn read(&self) -> Option<T> {
        self.value.get()
    }
    /// Returns `true` if the value has been written.
    pub fn is_written(&self) -> bool {
        self.value.get().is_some()
    }
}
/// A simple directed acyclic graph.
#[allow(dead_code)]
pub struct SimpleDag {
    /// `edges[i]` is the list of direct successors of node `i`.
    edges: Vec<Vec<usize>>,
}
#[allow(dead_code)]
impl SimpleDag {
    /// Creates a DAG with `n` nodes and no edges.
    pub fn new(n: usize) -> Self {
        Self {
            edges: vec![Vec::new(); n],
        }
    }
    /// Adds an edge from `from` to `to`.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.edges.len() {
            self.edges[from].push(to);
        }
    }
    /// Returns the successors of `node`.
    pub fn successors(&self, node: usize) -> &[usize] {
        self.edges.get(node).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Returns `true` if `from` can reach `to` via DFS.
    pub fn can_reach(&self, from: usize, to: usize) -> bool {
        let mut visited = vec![false; self.edges.len()];
        self.dfs(from, to, &mut visited)
    }
    fn dfs(&self, cur: usize, target: usize, visited: &mut Vec<bool>) -> bool {
        if cur == target {
            return true;
        }
        if cur >= visited.len() || visited[cur] {
            return false;
        }
        visited[cur] = true;
        for &next in self.successors(cur) {
            if self.dfs(next, target, visited) {
                return true;
            }
        }
        false
    }
    /// Returns the topological order of nodes, or `None` if a cycle is detected.
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.edges.len();
        let mut in_degree = vec![0usize; n];
        for succs in &self.edges {
            for &s in succs {
                if s < n {
                    in_degree[s] += 1;
                }
            }
        }
        let mut queue: std::collections::VecDeque<usize> =
            (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &s in self.successors(node) {
                if s < n {
                    in_degree[s] -= 1;
                    if in_degree[s] == 0 {
                        queue.push_back(s);
                    }
                }
            }
        }
        if order.len() == n {
            Some(order)
        } else {
            None
        }
    }
    /// Returns the number of nodes.
    pub fn num_nodes(&self) -> usize {
        self.edges.len()
    }
}
/// A generic counter that tracks min/max/sum for statistical summaries.
#[allow(dead_code)]
pub struct StatSummary {
    count: u64,
    sum: f64,
    min: f64,
    max: f64,
}
#[allow(dead_code)]
impl StatSummary {
    /// Creates an empty summary.
    pub fn new() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    /// Records a sample.
    pub fn record(&mut self, val: f64) {
        self.count += 1;
        self.sum += val;
        if val < self.min {
            self.min = val;
        }
        if val > self.max {
            self.max = val;
        }
    }
    /// Returns the mean, or `None` if no samples.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
    /// Returns the minimum, or `None` if no samples.
    pub fn min(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.min)
        }
    }
    /// Returns the maximum, or `None` if no samples.
    pub fn max(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.max)
        }
    }
    /// Returns the count of recorded samples.
    pub fn count(&self) -> u64 {
        self.count
    }
}
/// Trace level for filtering output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceLevel {
    /// No tracing.
    Off,
    /// Fatal errors only.
    Error,
    /// Warnings and errors.
    Warn,
    /// Informational messages.
    Info,
    /// Verbose debugging.
    Debug,
    /// Everything including internal details.
    Trace,
}
impl TraceLevel {
    /// Parse from a string.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "off" => Some(TraceLevel::Off),
            "error" => Some(TraceLevel::Error),
            "warn" | "warning" => Some(TraceLevel::Warn),
            "info" => Some(TraceLevel::Info),
            "debug" => Some(TraceLevel::Debug),
            "trace" => Some(TraceLevel::Trace),
            _ => None,
        }
    }
    /// Check whether at least as verbose as other.
    pub fn is_at_least(&self, other: TraceLevel) -> bool {
        *self >= other
    }
}
/// Aggregate statistics about a Tracer's events.
#[derive(Debug, Default, Clone)]
pub struct TracerStats {
    /// Total events logged.
    pub total: usize,
    /// Events at Error level.
    pub errors: usize,
    /// Events at Warn level.
    pub warnings: usize,
    /// Events at Info level.
    pub infos: usize,
    /// Events at Debug level.
    pub debugs: usize,
    /// Events at Trace level.
    pub traces: usize,
}
impl TracerStats {
    /// Compute stats for a tracer.
    pub fn compute(tracer: &Tracer) -> Self {
        let mut stats = Self {
            total: tracer.event_count(),
            ..Default::default()
        };
        for event in tracer.events() {
            match event.level {
                TraceLevel::Error => stats.errors += 1,
                TraceLevel::Warn => stats.warnings += 1,
                TraceLevel::Info => stats.infos += 1,
                TraceLevel::Debug => stats.debugs += 1,
                TraceLevel::Trace => stats.traces += 1,
                TraceLevel::Off => {}
            }
        }
        stats
    }
    /// Check whether any errors were logged.
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }
    /// Check whether any warnings were logged.
    pub fn has_warnings(&self) -> bool {
        self.warnings > 0
    }
}
/// A simple key-value store backed by a sorted Vec for small maps.
#[allow(dead_code)]
pub struct SmallMap<K: Ord + Clone, V: Clone> {
    entries: Vec<(K, V)>,
}
#[allow(dead_code)]
impl<K: Ord + Clone, V: Clone> SmallMap<K, V> {
    /// Creates a new empty small map.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Inserts or replaces the value for `key`.
    pub fn insert(&mut self, key: K, val: V) {
        match self.entries.binary_search_by_key(&&key, |(k, _)| k) {
            Ok(i) => self.entries[i].1 = val,
            Err(i) => self.entries.insert(i, (key, val)),
        }
    }
    /// Returns the value for `key`, or `None`.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries
            .binary_search_by_key(&key, |(k, _)| k)
            .ok()
            .map(|i| &self.entries[i].1)
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Returns all keys.
    pub fn keys(&self) -> Vec<&K> {
        self.entries.iter().map(|(k, _)| k).collect()
    }
    /// Returns all values.
    pub fn values(&self) -> Vec<&V> {
        self.entries.iter().map(|(_, v)| v).collect()
    }
}
/// A tagged union for representing a simple two-case discriminated union.
#[allow(dead_code)]
pub enum Either2<A, B> {
    /// The first alternative.
    First(A),
    /// The second alternative.
    Second(B),
}
#[allow(dead_code)]
impl<A, B> Either2<A, B> {
    /// Returns `true` if this is the first alternative.
    pub fn is_first(&self) -> bool {
        matches!(self, Either2::First(_))
    }
    /// Returns `true` if this is the second alternative.
    pub fn is_second(&self) -> bool {
        matches!(self, Either2::Second(_))
    }
    /// Returns the first value if present.
    pub fn first(self) -> Option<A> {
        match self {
            Either2::First(a) => Some(a),
            _ => None,
        }
    }
    /// Returns the second value if present.
    pub fn second(self) -> Option<B> {
        match self {
            Either2::Second(b) => Some(b),
            _ => None,
        }
    }
    /// Maps over the first alternative.
    pub fn map_first<C, F: FnOnce(A) -> C>(self, f: F) -> Either2<C, B> {
        match self {
            Either2::First(a) => Either2::First(f(a)),
            Either2::Second(b) => Either2::Second(b),
        }
    }
}
/// A token bucket rate limiter.
#[allow(dead_code)]
pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    refill_per_ms: u64,
    last_refill: std::time::Instant,
}
#[allow(dead_code)]
impl TokenBucket {
    /// Creates a new token bucket.
    pub fn new(capacity: u64, refill_per_ms: u64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_per_ms,
            last_refill: std::time::Instant::now(),
        }
    }
    /// Attempts to consume `n` tokens.  Returns `true` on success.
    pub fn try_consume(&mut self, n: u64) -> bool {
        self.refill();
        if self.tokens >= n {
            self.tokens -= n;
            true
        } else {
            false
        }
    }
    fn refill(&mut self) {
        let now = std::time::Instant::now();
        let elapsed_ms = now.duration_since(self.last_refill).as_millis() as u64;
        if elapsed_ms > 0 {
            let new_tokens = elapsed_ms * self.refill_per_ms;
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill = now;
        }
    }
    /// Returns the number of currently available tokens.
    pub fn available(&self) -> u64 {
        self.tokens
    }
    /// Returns the bucket capacity.
    pub fn capacity(&self) -> u64 {
        self.capacity
    }
}
/// A simple log sink that writes to a `Vec<String>`.
#[derive(Default, Debug, Clone)]
pub struct StringSink {
    pub(crate) lines: Vec<String>,
}
impl StringSink {
    /// Create an empty sink.
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
    /// Record a formatted event.
    pub fn record(&mut self, event: &TraceEvent) {
        self.lines.push(event.format());
    }
    /// Get all recorded lines.
    pub fn lines(&self) -> &[String] {
        &self.lines
    }
    /// Clear all lines.
    pub fn clear(&mut self) {
        self.lines.clear();
    }
    /// Number of recorded lines.
    pub fn len(&self) -> usize {
        self.lines.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    /// Render to a single string with newlines.
    pub fn render(&self) -> String {
        self.lines.join("\n")
    }
}
