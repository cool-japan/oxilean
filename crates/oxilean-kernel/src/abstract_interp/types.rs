//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::AbstractDomain;

/// A reduced product of interval and parity domains.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub struct IntervalParityProduct {
    pub interval: Interval,
    pub parity: ParityDomain,
}
#[allow(dead_code)]
impl IntervalParityProduct {
    /// Create from components.
    pub fn new(interval: Interval, parity: ParityDomain) -> Self {
        IntervalParityProduct { interval, parity }
    }
    /// Create from a concrete value.
    pub fn from_value(v: i64) -> Self {
        IntervalParityProduct {
            interval: Interval::new(v, v),
            parity: ParityDomain::from_value(v),
        }
    }
    /// Return the top element.
    pub fn top() -> Self {
        IntervalParityProduct {
            interval: Interval::top(),
            parity: ParityDomain::Top,
        }
    }
    /// Return the bottom element.
    pub fn bottom() -> Self {
        IntervalParityProduct {
            interval: Interval::bottom(),
            parity: ParityDomain::Bottom,
        }
    }
    /// Join two products.
    pub fn join(&self, other: &IntervalParityProduct) -> IntervalParityProduct {
        IntervalParityProduct {
            interval: self.interval.join(&other.interval),
            parity: self.parity.join(&other.parity),
        }
    }
    /// Add two products.
    pub fn add(&self, other: &IntervalParityProduct) -> IntervalParityProduct {
        IntervalParityProduct {
            interval: self.interval.add(&other.interval),
            parity: self.parity.add(&other.parity),
        }
    }
    /// Return whether this is bottom.
    pub fn is_bottom(&self) -> bool {
        self.interval.is_bottom() || self.parity.is_bottom()
    }
}
/// An alarm generated when abstract analysis detects a potential issue.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct AbstractAlarm {
    pub label: String,
    pub message: String,
    pub severity: AlarmSeverity,
}
#[allow(dead_code)]
impl AbstractAlarm {
    /// Create an alarm.
    pub fn new(label: &str, message: &str, severity: AlarmSeverity) -> Self {
        AbstractAlarm {
            label: label.to_string(),
            message: message.to_string(),
            severity,
        }
    }
    /// Return whether this is an error-level alarm.
    pub fn is_error(&self) -> bool {
        self.severity == AlarmSeverity::Error
    }
}
/// An analysis pass configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct AnalysisConfig {
    pub max_iterations: u32,
    pub use_widening: bool,
    pub use_narrowing: bool,
    pub collect_alarms: bool,
    pub verbose: bool,
}
#[allow(dead_code)]
impl AnalysisConfig {
    /// Default configuration.
    pub fn default_config() -> Self {
        AnalysisConfig {
            max_iterations: 100,
            use_widening: true,
            use_narrowing: true,
            collect_alarms: true,
            verbose: false,
        }
    }
    /// Quick (fast) configuration for CI.
    pub fn fast() -> Self {
        AnalysisConfig {
            max_iterations: 10,
            use_widening: true,
            use_narrowing: false,
            collect_alarms: false,
            verbose: false,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum AlarmSeverity {
    Info,
    Warning,
    Error,
}
/// Abstract comparison result (may hold, definitely holds, definitely fails).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum AbstractCmp {
    DefinitelyTrue,
    DefinitelyFalse,
    Unknown,
}
#[allow(dead_code)]
impl AbstractCmp {
    /// Abstract less-than comparison of two intervals.
    pub fn lt(a: &Interval, b: &Interval) -> Self {
        if a.is_bottom() || b.is_bottom() {
            return AbstractCmp::Unknown;
        }
        if a.hi < b.lo {
            AbstractCmp::DefinitelyTrue
        } else if a.lo >= b.hi {
            AbstractCmp::DefinitelyFalse
        } else {
            AbstractCmp::Unknown
        }
    }
    /// Abstract equality comparison.
    pub fn eq(a: &Interval, b: &Interval) -> Self {
        if a.is_bottom() || b.is_bottom() {
            return AbstractCmp::Unknown;
        }
        let meet = a.meet(b);
        if meet.is_bottom() {
            AbstractCmp::DefinitelyFalse
        } else if a.lo == a.hi && b.lo == b.hi && a.lo == b.lo {
            AbstractCmp::DefinitelyTrue
        } else {
            AbstractCmp::Unknown
        }
    }
    /// Return whether the comparison is definitely true.
    pub fn is_definitely_true(&self) -> bool {
        *self == AbstractCmp::DefinitelyTrue
    }
    /// Return whether the comparison is definitely false.
    pub fn is_definitely_false(&self) -> bool {
        *self == AbstractCmp::DefinitelyFalse
    }
}
/// A collection of abstract analysis results for all variables.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AnalysisResults {
    entries: Vec<(String, IntervalParityProduct)>,
}
#[allow(dead_code)]
impl AnalysisResults {
    /// Create empty results.
    pub fn new() -> Self {
        AnalysisResults {
            entries: Vec::new(),
        }
    }
    /// Record results for a variable.
    pub fn set(&mut self, var: &str, result: IntervalParityProduct) {
        if let Some(e) = self.entries.iter_mut().find(|(v, _)| v == var) {
            e.1 = result;
        } else {
            self.entries.push((var.to_string(), result));
        }
    }
    /// Look up results for a variable.
    pub fn get(&self, var: &str) -> Option<IntervalParityProduct> {
        self.entries.iter().find(|(v, _)| v == var).map(|(_, r)| *r)
    }
    /// Return all variables and their results.
    pub fn all(&self) -> &[(String, IntervalParityProduct)] {
        &self.entries
    }
    /// Return the number of analyzed variables.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether any results are present.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Return variables where the interval is proven to be non-negative.
    pub fn proven_non_negative(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(_, r)| r.interval.lo >= 0 && !r.interval.is_bottom())
            .map(|(v, _)| v.as_str())
            .collect()
    }
    /// Return variables proven to be even.
    pub fn proven_even(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(_, r)| r.parity == ParityDomain::Even)
            .map(|(v, _)| v.as_str())
            .collect()
    }
}
/// Abstract domain for expression depth analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DepthDomain {
    /// Maximum allowed depth.
    pub max_depth: usize,
    /// Current depth reached.
    pub current_depth: usize,
}
impl DepthDomain {
    /// Create a new depth domain with the given maximum.
    pub fn new(max: usize) -> Self {
        DepthDomain {
            max_depth: max,
            current_depth: 0,
        }
    }
    /// True if the current depth is within bounds.
    pub fn is_bounded(&self) -> bool {
        self.current_depth <= self.max_depth
    }
    /// Increase depth by one, saturating at max_depth + 1.
    pub fn increase(&self) -> DepthDomain {
        DepthDomain {
            max_depth: self.max_depth,
            current_depth: self.current_depth.saturating_add(1),
        }
    }
    /// Join (take the maximum current depth) with another DepthDomain.
    pub fn join(&self, other: &DepthDomain) -> DepthDomain {
        DepthDomain {
            max_depth: self.max_depth.max(other.max_depth),
            current_depth: self.current_depth.max(other.current_depth),
        }
    }
}
/// An abstract value in the interval domain [lo, hi].
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Interval {
    pub lo: i64,
    pub hi: i64,
}
#[allow(dead_code)]
impl Interval {
    /// Create an interval [lo, hi].
    pub fn new(lo: i64, hi: i64) -> Self {
        Interval {
            lo: lo.min(hi),
            hi: lo.max(hi),
        }
    }
    /// Return the top interval (-∞, +∞) represented as (i64::MIN, i64::MAX).
    pub fn top() -> Self {
        Interval {
            lo: i64::MIN,
            hi: i64::MAX,
        }
    }
    /// Return the bottom interval (empty).
    pub fn bottom() -> Self {
        Interval {
            lo: i64::MAX,
            hi: i64::MIN,
        }
    }
    /// Return whether this is the bottom (empty) interval.
    pub fn is_bottom(&self) -> bool {
        self.lo > self.hi
    }
    /// Return whether this is the top interval.
    pub fn is_top(&self) -> bool {
        self.lo == i64::MIN && self.hi == i64::MAX
    }
    /// Return the join (widening step) of two intervals.
    pub fn join(&self, other: &Interval) -> Interval {
        if self.is_bottom() {
            return *other;
        }
        if other.is_bottom() {
            return *self;
        }
        Interval::new(self.lo.min(other.lo), self.hi.max(other.hi))
    }
    /// Return the meet (intersection) of two intervals.
    pub fn meet(&self, other: &Interval) -> Interval {
        let lo = self.lo.max(other.lo);
        let hi = self.hi.min(other.hi);
        if lo > hi {
            Interval::bottom()
        } else {
            Interval { lo, hi }
        }
    }
    /// Return whether this interval contains a given value.
    pub fn contains(&self, v: i64) -> bool {
        !self.is_bottom() && self.lo <= v && v <= self.hi
    }
    /// Return the width (hi - lo + 1), or 0 if bottom.
    pub fn width(&self) -> u64 {
        if self.is_bottom() {
            0
        } else {
            (self.hi - self.lo + 1) as u64
        }
    }
    /// Add two intervals.
    pub fn add(&self, other: &Interval) -> Interval {
        if self.is_bottom() || other.is_bottom() {
            return Interval::bottom();
        }
        Interval::new(
            self.lo.saturating_add(other.lo),
            self.hi.saturating_add(other.hi),
        )
    }
    /// Negate an interval.
    pub fn negate(&self) -> Interval {
        if self.is_bottom() {
            return Interval::bottom();
        }
        Interval::new(-self.hi, -self.lo)
    }
    /// Subtract two intervals.
    pub fn sub(&self, other: &Interval) -> Interval {
        self.add(&other.negate())
    }
}
/// Abstract interpreter for kernel expressions.
pub struct AbstractInterpreter {
    max_depth: usize,
}
impl AbstractInterpreter {
    /// Create a new abstract interpreter with the given maximum depth.
    pub fn new(max_depth: usize) -> Self {
        AbstractInterpreter { max_depth }
    }
    /// Analyze the nesting depth of a parenthesised expression string.
    pub fn analyze_depth(&self, expr_str: &str) -> DepthDomain {
        let mut domain = DepthDomain::new(self.max_depth);
        let mut max_seen: usize = 0;
        let mut current: usize = 0;
        for ch in expr_str.chars() {
            match ch {
                '(' | '[' | '{' => {
                    current = current.saturating_add(1);
                    if current > max_seen {
                        max_seen = current;
                    }
                }
                ')' | ']' | '}' => {
                    current = current.saturating_sub(1);
                }
                _ => {}
            }
        }
        domain.current_depth = max_seen;
        domain
    }
    /// Analyze the numeric sign of a simple expression string.
    ///
    /// Recognises leading `-` for negative and plain digits for positive.
    pub fn analyze_sign(&self, expr_str: &str) -> SignDomain {
        let trimmed = expr_str.trim();
        if trimmed.is_empty() {
            return SignDomain::Bottom;
        }
        if let Ok(n) = trimmed.parse::<i64>() {
            return match n.cmp(&0) {
                std::cmp::Ordering::Less => SignDomain::Neg,
                std::cmp::Ordering::Equal => SignDomain::Zero,
                std::cmp::Ordering::Greater => SignDomain::Pos,
            };
        }
        if trimmed.starts_with('-') {
            SignDomain::Neg
        } else if trimmed.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            SignDomain::Pos
        } else {
            SignDomain::Top
        }
    }
    /// Analyze the approximate size of an expression string.
    pub fn analyze_size(&self, expr_str: &str) -> SizeDomain {
        let count = expr_str
            .split(|c: char| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']'))
            .filter(|s| !s.is_empty())
            .count();
        SizeDomain::from_count(count)
    }
    /// Compute a fixed point of `f` starting from `init`.
    ///
    /// Iterates until the value stabilises or a maximum number of steps
    /// (1 000) is reached, at which point the last value is returned.
    pub fn fixed_point<S: Clone + PartialEq, F: Fn(&S) -> S>(&self, init: S, f: F) -> S {
        const MAX_ITERS: usize = 1_000;
        let mut current = init;
        for _ in 0..MAX_ITERS {
            let next = f(&current);
            if next == current {
                return current;
            }
            current = next;
        }
        current
    }
}
/// Abstract reachability analysis: which program points are reachable?
#[allow(dead_code)]
pub struct ReachabilityAnalysis {
    reachable: Vec<String>,
    unreachable: Vec<String>,
}
#[allow(dead_code)]
impl ReachabilityAnalysis {
    /// Create an empty analysis.
    pub fn new() -> Self {
        ReachabilityAnalysis {
            reachable: Vec::new(),
            unreachable: Vec::new(),
        }
    }
    /// Mark a point as reachable.
    pub fn mark_reachable(&mut self, label: &str) {
        if !self.reachable.contains(&label.to_string()) {
            self.reachable.push(label.to_string());
        }
        self.unreachable.retain(|s| s != label);
    }
    /// Mark a point as unreachable.
    pub fn mark_unreachable(&mut self, label: &str) {
        if !self.unreachable.contains(&label.to_string()) {
            self.unreachable.push(label.to_string());
        }
        self.reachable.retain(|s| s != label);
    }
    /// Return whether a point is definitely reachable.
    pub fn is_reachable(&self, label: &str) -> bool {
        self.reachable.contains(&label.to_string())
    }
    /// Return whether a point is definitely unreachable.
    pub fn is_unreachable(&self, label: &str) -> bool {
        self.unreachable.contains(&label.to_string())
    }
    /// Return the count of reachable points.
    pub fn reachable_count(&self) -> usize {
        self.reachable.len()
    }
    /// Return the count of unreachable points.
    pub fn unreachable_count(&self) -> usize {
        self.unreachable.len()
    }
}
/// Combined abstract state for expression analysis.
#[derive(Debug, Clone)]
pub struct AbstractState {
    /// Sign information.
    pub sign: SignDomain,
    /// Depth information.
    pub depth: DepthDomain,
    /// Size information.
    pub size: SizeDomain,
}
impl AbstractState {
    /// Create an initial abstract state.
    pub fn new() -> Self {
        AbstractState {
            sign: SignDomain::Bottom,
            depth: DepthDomain::new(1024),
            size: SizeDomain::Zero,
        }
    }
    /// Join two abstract states component-wise.
    pub fn join(&self, other: &AbstractState) -> AbstractState {
        AbstractState {
            sign: self.sign.join(&other.sign),
            depth: self.depth.join(&other.depth),
            size: SizeDomain::max(self.size, other.size),
        }
    }
}
/// A database of function summaries.
#[allow(dead_code)]
pub struct SummaryDatabase {
    entries: Vec<FunctionSummary>,
}
#[allow(dead_code)]
impl SummaryDatabase {
    /// Create an empty database.
    pub fn new() -> Self {
        SummaryDatabase {
            entries: Vec::new(),
        }
    }
    /// Add a summary.
    pub fn add(&mut self, summary: FunctionSummary) {
        self.entries.push(summary);
    }
    /// Look up a summary by function name.
    pub fn find(&self, name: &str) -> Option<&FunctionSummary> {
        self.entries.iter().find(|s| s.function_name == name)
    }
    /// Return all functions with proven termination.
    pub fn proven_terminating(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|s| s.terminates())
            .map(|s| s.function_name.as_str())
            .collect()
    }
    /// Return the number of summaries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether the database is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// An abstract reachability domain for basic blocks.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum BlockReachability {
    Unreachable,
    Reachable,
    Unknown,
}
#[allow(dead_code)]
impl BlockReachability {
    /// Join two reachability values.
    pub fn join(&self, other: &BlockReachability) -> BlockReachability {
        use BlockReachability::*;
        match (self, other) {
            (Unreachable, x) | (x, Unreachable) => *x,
            (Reachable, _) | (_, Reachable) => Reachable,
            _ => Unknown,
        }
    }
    /// Return whether this block may be reachable.
    pub fn may_be_reachable(&self) -> bool {
        !matches!(self, BlockReachability::Unreachable)
    }
}
/// Collects alarms during abstract analysis.
#[allow(dead_code)]
pub struct AlarmCollector {
    alarms: Vec<AbstractAlarm>,
}
#[allow(dead_code)]
impl AlarmCollector {
    /// Create an empty collector.
    pub fn new() -> Self {
        AlarmCollector { alarms: Vec::new() }
    }
    /// Add an alarm.
    pub fn add(&mut self, alarm: AbstractAlarm) {
        self.alarms.push(alarm);
    }
    /// Return all alarms.
    pub fn alarms(&self) -> &[AbstractAlarm] {
        &self.alarms
    }
    /// Return all error-level alarms.
    pub fn errors(&self) -> Vec<&AbstractAlarm> {
        self.alarms.iter().filter(|a| a.is_error()).collect()
    }
    /// Return whether there are any errors.
    pub fn has_errors(&self) -> bool {
        self.alarms.iter().any(|a| a.is_error())
    }
    /// Return the total alarm count.
    pub fn len(&self) -> usize {
        self.alarms.len()
    }
    /// Return whether no alarms were collected.
    pub fn is_empty(&self) -> bool {
        self.alarms.is_empty()
    }
    /// Clear all alarms.
    pub fn clear(&mut self) {
        self.alarms.clear();
    }
    /// Return the count of alarms at each severity.
    pub fn count_by_severity(&self) -> (usize, usize, usize) {
        let info = self
            .alarms
            .iter()
            .filter(|a| a.severity == AlarmSeverity::Info)
            .count();
        let warn = self
            .alarms
            .iter()
            .filter(|a| a.severity == AlarmSeverity::Warning)
            .count();
        let err = self
            .alarms
            .iter()
            .filter(|a| a.severity == AlarmSeverity::Error)
            .count();
        (info, warn, err)
    }
}
/// A powerset domain over a finite set of tags.
#[allow(dead_code)]
pub struct PowersetDomain {
    elements: u64,
    universe_size: u8,
}
#[allow(dead_code)]
impl PowersetDomain {
    /// Create an empty set (bottom).
    pub fn bottom(universe_size: u8) -> Self {
        PowersetDomain {
            elements: 0,
            universe_size,
        }
    }
    /// Create the full set (top).
    pub fn top(universe_size: u8) -> Self {
        let mask = if universe_size >= 64 {
            u64::MAX
        } else {
            (1u64 << universe_size) - 1
        };
        PowersetDomain {
            elements: mask,
            universe_size,
        }
    }
    /// Add an element by index.
    pub fn add(&mut self, idx: u8) {
        if idx < self.universe_size {
            self.elements |= 1 << idx;
        }
    }
    /// Remove an element by index.
    pub fn remove(&mut self, idx: u8) {
        self.elements &= !(1 << idx);
    }
    /// Return whether an element is present.
    pub fn contains(&self, idx: u8) -> bool {
        (self.elements >> idx) & 1 != 0
    }
    /// Return the join (union).
    pub fn join(&self, other: &PowersetDomain) -> PowersetDomain {
        PowersetDomain {
            elements: self.elements | other.elements,
            universe_size: self.universe_size,
        }
    }
    /// Return the meet (intersection).
    pub fn meet(&self, other: &PowersetDomain) -> PowersetDomain {
        PowersetDomain {
            elements: self.elements & other.elements,
            universe_size: self.universe_size,
        }
    }
    /// Return the number of elements.
    pub fn count(&self) -> u32 {
        self.elements.count_ones()
    }
    /// Return whether the set is empty (bottom).
    pub fn is_bottom(&self) -> bool {
        self.elements == 0
    }
    /// Return whether the set is full (top).
    pub fn is_top(&self) -> bool {
        let mask = if self.universe_size >= 64 {
            u64::MAX
        } else {
            (1u64 << self.universe_size) - 1
        };
        self.elements == mask
    }
}
/// A trace of abstract values at program points.
#[allow(dead_code)]
pub struct AbstractTrace {
    points: Vec<(String, Interval)>,
}
#[allow(dead_code)]
impl AbstractTrace {
    /// Create an empty trace.
    pub fn new() -> Self {
        AbstractTrace { points: Vec::new() }
    }
    /// Record an abstract value at a program point.
    pub fn record(&mut self, label: &str, iv: Interval) {
        self.points.push((label.to_string(), iv));
    }
    /// Return the interval at the given label (first match).
    pub fn at(&self, label: &str) -> Option<Interval> {
        self.points
            .iter()
            .find(|(l, _)| l == label)
            .map(|(_, iv)| *iv)
    }
    /// Return all trace points.
    pub fn all(&self) -> &[(String, Interval)] {
        &self.points
    }
    /// Return the number of trace points.
    pub fn len(&self) -> usize {
        self.points.len()
    }
    /// Return whether the trace is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
    /// Format the trace as a readable string.
    pub fn format(&self) -> String {
        self.points
            .iter()
            .map(|(l, iv)| format!("{}: [{}, {}]", l, iv.lo, iv.hi))
            .collect::<Vec<_>>()
            .join("; ")
    }
}
/// Abstract cost model: estimated number of reduction steps.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub struct CostBound {
    pub lower: u64,
    pub upper: Option<u64>,
}
#[allow(dead_code)]
impl CostBound {
    /// Create a tight bound.
    pub fn exact(n: u64) -> Self {
        CostBound {
            lower: n,
            upper: Some(n),
        }
    }
    /// Create an open-ended bound.
    pub fn at_least(n: u64) -> Self {
        CostBound {
            lower: n,
            upper: None,
        }
    }
    /// Create a range bound.
    pub fn range(lo: u64, hi: u64) -> Self {
        CostBound {
            lower: lo,
            upper: Some(hi),
        }
    }
    /// Return whether the cost is bounded.
    pub fn is_bounded(&self) -> bool {
        self.upper.is_some()
    }
    /// Return the cost width (hi - lo), or None if unbounded.
    pub fn width(&self) -> Option<u64> {
        self.upper.map(|h| h - self.lower)
    }
    /// Add two cost bounds.
    pub fn add(&self, other: &CostBound) -> CostBound {
        CostBound {
            lower: self.lower.saturating_add(other.lower),
            upper: match (self.upper, other.upper) {
                (Some(a), Some(b)) => Some(a.saturating_add(b)),
                _ => None,
            },
        }
    }
}
/// The congruence abstract domain: values congruent to r mod m.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct CongruenceDomain {
    pub modulus: u64,
    pub residue: u64,
}
#[allow(dead_code)]
impl CongruenceDomain {
    /// Create a singleton value (mod=1, res=0 is "any").
    pub fn singleton(v: u64) -> Self {
        CongruenceDomain {
            modulus: 0,
            residue: v,
        }
    }
    /// Create "all values congruent to r mod m".
    pub fn congruent(modulus: u64, residue: u64) -> Self {
        let r = if modulus == 0 { 0 } else { residue % modulus };
        CongruenceDomain {
            modulus,
            residue: r,
        }
    }
    /// Create the top element (any value).
    pub fn top() -> Self {
        CongruenceDomain {
            modulus: 1,
            residue: 0,
        }
    }
    /// Create the bottom element (no values).
    pub fn bottom() -> Self {
        CongruenceDomain {
            modulus: 0,
            residue: u64::MAX,
        }
    }
    /// Return whether this is top (everything).
    pub fn is_top(&self) -> bool {
        self.modulus == 1
    }
    /// Return whether this is bottom (nothing).
    pub fn is_bottom(&self) -> bool {
        self.modulus == 0 && self.residue == u64::MAX
    }
    /// Return whether a value satisfies this congruence.
    pub fn satisfies(&self, v: u64) -> bool {
        if self.is_bottom() {
            return false;
        }
        if self.is_top() {
            return true;
        }
        if self.modulus == 0 {
            return v == self.residue;
        }
        v % self.modulus == self.residue
    }
    /// Join (GCD-based).
    pub fn join(&self, other: &CongruenceDomain) -> CongruenceDomain {
        if self.is_bottom() {
            return *other;
        }
        if other.is_bottom() {
            return *self;
        }
        if self.is_top() || other.is_top() {
            return CongruenceDomain::top();
        }
        if self.modulus == other.modulus && self.residue == other.residue {
            return *self;
        }
        CongruenceDomain::top()
    }
}
/// The three-valued logic domain (maybe, true, false).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TrileanDomain {
    Bottom,
    False,
    True,
    Top,
}
#[allow(dead_code)]
impl TrileanDomain {
    /// Create from a concrete bool.
    pub fn from_bool(b: bool) -> Self {
        if b {
            TrileanDomain::True
        } else {
            TrileanDomain::False
        }
    }
    /// Logical AND of two trileans.
    pub fn and(&self, other: &TrileanDomain) -> TrileanDomain {
        use TrileanDomain::*;
        match (self, other) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (False, _) | (_, False) => False,
            (True, True) => True,
            _ => Top,
        }
    }
    /// Logical OR of two trileans.
    pub fn or(&self, other: &TrileanDomain) -> TrileanDomain {
        use TrileanDomain::*;
        match (self, other) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (True, _) | (_, True) => True,
            (False, False) => False,
            _ => Top,
        }
    }
    /// Logical NOT.
    pub fn not(&self) -> TrileanDomain {
        use TrileanDomain::*;
        match self {
            Bottom => Bottom,
            False => True,
            True => False,
            Top => Top,
        }
    }
    /// Join two trileans.
    pub fn join(&self, other: &TrileanDomain) -> TrileanDomain {
        use TrileanDomain::*;
        match (self, other) {
            (Bottom, x) | (x, Bottom) => *x,
            (True, True) => True,
            (False, False) => False,
            _ => Top,
        }
    }
    /// Return whether this is definitely true.
    pub fn is_definitely_true(&self) -> bool {
        *self == TrileanDomain::True
    }
    /// Return whether this might be true.
    pub fn may_be_true(&self) -> bool {
        matches!(self, TrileanDomain::True | TrileanDomain::Top)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum TransferEffect {
    /// Assign a variable to an interval.
    Assign { var: String, interval: Interval },
    /// Constrain a variable to a non-bottom interval.
    Constrain { var: String, constraint: Interval },
    /// Invalidate (set to top) a variable.
    Invalidate { var: String },
}
/// Abstract sign domain for numeric expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignDomain {
    /// No information (least element).
    Bottom,
    /// Strictly negative.
    Neg,
    /// Exactly zero.
    Zero,
    /// Strictly positive.
    Pos,
    /// Non-zero (either negative or positive).
    Nonzero,
    /// Non-negative (zero or positive).
    NonNeg,
    /// Non-positive (zero or negative).
    NonPos,
    /// All values (greatest element).
    Top,
}
#[allow(clippy::should_implement_trait)]
impl SignDomain {
    /// Negate a sign value.
    pub fn negate(&self) -> SignDomain {
        use SignDomain::*;
        match self {
            Bottom => Bottom,
            Neg => Pos,
            Zero => Zero,
            Pos => Neg,
            Nonzero => Nonzero,
            NonNeg => NonPos,
            NonPos => NonNeg,
            Top => Top,
        }
    }
    /// Abstract addition of two sign values.
    pub fn add(s1: SignDomain, s2: SignDomain) -> SignDomain {
        use SignDomain::*;
        match (s1, s2) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (Zero, x) | (x, Zero) => x,
            (Pos, Pos) => Pos,
            (Neg, Neg) => Neg,
            (Pos, NonNeg) | (NonNeg, Pos) => Pos,
            (Neg, NonPos) | (NonPos, Neg) => Neg,
            (NonNeg, NonNeg) => NonNeg,
            (NonPos, NonPos) => NonPos,
            _ => Top,
        }
    }
    /// Abstract multiplication of two sign values.
    pub fn mul(s1: SignDomain, s2: SignDomain) -> SignDomain {
        use SignDomain::*;
        match (s1, s2) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (Zero, _) | (_, Zero) => Zero,
            (Top, _) | (_, Top) => Top,
            (Pos, Pos) | (Neg, Neg) => Pos,
            (Pos, Neg) | (Neg, Pos) => Neg,
            (NonNeg, NonNeg) => NonNeg,
            (NonPos, NonPos) => NonNeg,
            (NonNeg, NonPos) | (NonPos, NonNeg) => NonPos,
            (Pos, NonNeg) | (NonNeg, Pos) => NonNeg,
            (Neg, NonPos) | (NonPos, Neg) => NonNeg,
            (Pos, NonPos) | (NonPos, Pos) => NonPos,
            (Neg, NonNeg) | (NonNeg, Neg) => NonPos,
            _ => Top,
        }
    }
}
/// A stub abstract interpreter that runs for a fixed number of steps.
#[allow(dead_code)]
pub struct SimpleAbstractInterpreter {
    config: AnalysisConfig,
    results: AnalysisResults,
    alarms: AlarmCollector,
}
#[allow(dead_code)]
impl SimpleAbstractInterpreter {
    /// Create a new interpreter with the given config.
    pub fn new(config: AnalysisConfig) -> Self {
        SimpleAbstractInterpreter {
            config,
            results: AnalysisResults::new(),
            alarms: AlarmCollector::new(),
        }
    }
    /// Set an initial abstract value for a variable.
    pub fn init_var(&mut self, var: &str, value: IntervalParityProduct) {
        self.results.set(var, value);
    }
    /// Return a reference to current results.
    pub fn results(&self) -> &AnalysisResults {
        &self.results
    }
    /// Return a reference to collected alarms.
    pub fn alarms(&self) -> &AlarmCollector {
        &self.alarms
    }
    /// Return a summary (stub: always converges in 1 iteration).
    pub fn run_stub(&mut self) -> InterpretationSummary {
        InterpretationSummary::new(1, true, self.alarms.len())
    }
}
/// A fixpoint iteration engine for abstract interpretation.
#[allow(dead_code)]
pub struct FixpointEngine {
    max_iterations: u32,
    iterations_done: u32,
    widening_threshold: u32,
}
#[allow(dead_code)]
impl FixpointEngine {
    /// Create an engine with a given maximum iteration count.
    pub fn new(max_iterations: u32) -> Self {
        FixpointEngine {
            max_iterations,
            iterations_done: 0,
            widening_threshold: max_iterations / 2,
        }
    }
    /// Return whether fixpoint was reached (next == current).
    pub fn is_fixpoint(current: &IntervalEnv, next: &IntervalEnv) -> bool {
        current.is_at_least_as_wide(next) && next.is_at_least_as_wide(current)
    }
    /// Return whether we should apply widening (past the widening threshold).
    pub fn should_widen(&self) -> bool {
        self.iterations_done >= self.widening_threshold
    }
    /// Record one iteration step.
    pub fn step(&mut self) {
        self.iterations_done += 1;
    }
    /// Return whether we've exceeded the iteration limit.
    pub fn is_exhausted(&self) -> bool {
        self.iterations_done >= self.max_iterations
    }
    /// Return the iteration count so far.
    pub fn iterations(&self) -> u32 {
        self.iterations_done
    }
    /// Reset to initial state.
    pub fn reset(&mut self) {
        self.iterations_done = 0;
    }
}
/// The parity abstract domain: even, odd, or top/bottom.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ParityDomain {
    Bottom,
    Even,
    Odd,
    Top,
}
#[allow(dead_code)]
impl ParityDomain {
    /// Create from a concrete integer.
    pub fn from_value(v: i64) -> Self {
        if v % 2 == 0 {
            ParityDomain::Even
        } else {
            ParityDomain::Odd
        }
    }
    /// Join two parity values.
    pub fn join(&self, other: &ParityDomain) -> ParityDomain {
        use ParityDomain::*;
        match (self, other) {
            (Bottom, x) | (x, Bottom) => *x,
            (Even, Even) => Even,
            (Odd, Odd) => Odd,
            _ => Top,
        }
    }
    /// Meet two parity values.
    pub fn meet(&self, other: &ParityDomain) -> ParityDomain {
        use ParityDomain::*;
        match (self, other) {
            (Top, x) | (x, Top) => *x,
            (Even, Even) => Even,
            (Odd, Odd) => Odd,
            _ => Bottom,
        }
    }
    /// Add two parity values.
    pub fn add(&self, other: &ParityDomain) -> ParityDomain {
        use ParityDomain::*;
        match (self, other) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (Top, _) | (_, Top) => Top,
            (Even, x) | (x, Even) => *x,
            (Odd, Odd) => Even,
        }
    }
    /// Multiply two parity values.
    pub fn mul(&self, other: &ParityDomain) -> ParityDomain {
        use ParityDomain::*;
        match (self, other) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (Even, _) | (_, Even) => Even,
            (Odd, Odd) => Odd,
            _ => Top,
        }
    }
    /// Return whether this is bottom.
    pub fn is_bottom(&self) -> bool {
        *self == ParityDomain::Bottom
    }
    /// Return whether this is top.
    pub fn is_top(&self) -> bool {
        *self == ParityDomain::Top
    }
}
/// A node in the call graph (function definition).
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct CallGraphNode {
    pub name: String,
    pub is_recursive: bool,
    pub callees: Vec<String>,
}
#[allow(dead_code)]
impl CallGraphNode {
    /// Create a new call graph node.
    pub fn new(name: impl Into<String>) -> Self {
        CallGraphNode {
            name: name.into(),
            is_recursive: false,
            callees: Vec::new(),
        }
    }
    /// Add a callee.
    pub fn add_callee(&mut self, name: &str) {
        if name == self.name.as_str() {
            self.is_recursive = true;
        }
        if !self.callees.iter().any(|s| s == name) {
            self.callees.push(name.to_string());
        }
    }
    /// Return whether this node calls `name`.
    pub fn calls(&self, name: &str) -> bool {
        self.callees.iter().any(|s| s == name)
    }
}
/// A product abstract domain combining two domains.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub struct ProductDomain<A: Copy, B: Copy> {
    pub first: A,
    pub second: B,
}
#[allow(dead_code)]
impl<A: Copy, B: Copy> ProductDomain<A, B> {
    /// Create a product domain.
    pub fn new(first: A, second: B) -> Self {
        ProductDomain { first, second }
    }
}
/// A simple chaotic iteration fixpoint solver.
#[allow(dead_code)]
pub struct ChaoticIterator {
    max_steps: u32,
    current_step: u32,
    converged: bool,
}
#[allow(dead_code)]
impl ChaoticIterator {
    /// Create an iterator with a step limit.
    pub fn new(max_steps: u32) -> Self {
        ChaoticIterator {
            max_steps,
            current_step: 0,
            converged: false,
        }
    }
    /// Mark as converged.
    pub fn mark_converged(&mut self) {
        self.converged = true;
    }
    /// Advance one step. Returns false if limit exceeded.
    pub fn advance(&mut self) -> bool {
        if self.current_step >= self.max_steps {
            return false;
        }
        self.current_step += 1;
        true
    }
    /// Return whether the iterator has converged.
    pub fn is_converged(&self) -> bool {
        self.converged
    }
    /// Return whether the step limit was exceeded without convergence.
    pub fn is_limit_exceeded(&self) -> bool {
        !self.converged && self.current_step >= self.max_steps
    }
    /// Return the number of steps taken.
    pub fn steps(&self) -> u32 {
        self.current_step
    }
    /// Reset to initial state.
    pub fn reset(&mut self) {
        self.current_step = 0;
        self.converged = false;
    }
}
/// A call graph for program analysis.
#[allow(dead_code)]
pub struct CallGraph {
    nodes: Vec<CallGraphNode>,
}
#[allow(dead_code)]
impl CallGraph {
    /// Create an empty call graph.
    pub fn new() -> Self {
        CallGraph { nodes: Vec::new() }
    }
    /// Add a node.
    pub fn add_node(&mut self, node: CallGraphNode) {
        self.nodes.push(node);
    }
    /// Look up a node by name.
    pub fn find(&self, name: &str) -> Option<&CallGraphNode> {
        self.nodes.iter().find(|n| n.name == name)
    }
    /// Return all recursive functions.
    pub fn recursive_fns(&self) -> Vec<&str> {
        self.nodes
            .iter()
            .filter(|n| n.is_recursive)
            .map(|n| n.name.as_str())
            .collect()
    }
    /// Return callers of a given function.
    pub fn callers_of(&self, name: &str) -> Vec<&str> {
        self.nodes
            .iter()
            .filter(|n| n.calls(name))
            .map(|n| n.name.as_str())
            .collect()
    }
    /// Return the number of nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    /// Return whether the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    /// Return all function names.
    pub fn function_names(&self) -> Vec<&str> {
        self.nodes.iter().map(|n| n.name.as_str()).collect()
    }
}
/// Abstract termination evidence: witness for why a function terminates.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub enum TerminationEvidence {
    /// Structural recursion on argument `idx`.
    Structural { arg_index: u32 },
    /// Lexicographic tuple ordering.
    Lexicographic { measures: Vec<String> },
    /// Unknown; may not terminate.
    Unknown,
}
#[allow(dead_code)]
impl TerminationEvidence {
    /// Return whether termination is proven.
    pub fn is_proven(&self) -> bool {
        !matches!(self, TerminationEvidence::Unknown)
    }
    /// Format a description.
    pub fn describe(&self) -> String {
        match self {
            TerminationEvidence::Structural { arg_index } => {
                format!("structural on arg #{}", arg_index)
            }
            TerminationEvidence::Lexicographic { measures } => {
                format!("lexicographic on [{}]", measures.join(", "))
            }
            TerminationEvidence::Unknown => "unknown".to_string(),
        }
    }
}
/// An abstract environment mapping variable names to interval values.
#[allow(dead_code)]
pub struct IntervalEnv {
    pub(super) bindings: Vec<(String, Interval)>,
}
#[allow(dead_code)]
impl IntervalEnv {
    /// Create an empty environment.
    pub fn new() -> Self {
        IntervalEnv {
            bindings: Vec::new(),
        }
    }
    /// Bind a variable to an interval.
    pub fn set(&mut self, name: &str, iv: Interval) {
        if let Some(b) = self.bindings.iter_mut().find(|(n, _)| n == name) {
            b.1 = iv;
        } else {
            self.bindings.push((name.to_string(), iv));
        }
    }
    /// Look up a variable's interval (returns Top if not found).
    pub fn get(&self, name: &str) -> Interval {
        self.bindings
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, iv)| *iv)
            .unwrap_or(Interval::top())
    }
    /// Join two environments point-wise.
    pub fn join(&self, other: &IntervalEnv) -> IntervalEnv {
        let mut result = IntervalEnv::new();
        for (name, iv) in &self.bindings {
            let iv2 = other.get(name);
            result.set(name, iv.join(&iv2));
        }
        for (name, iv) in &other.bindings {
            if self.bindings.iter().all(|(n, _)| n != name) {
                result.set(name, *iv);
            }
        }
        result
    }
    /// Return whether this environment is "wider" than or equal to `other`.
    pub fn is_at_least_as_wide(&self, other: &IntervalEnv) -> bool {
        other.bindings.iter().all(|(name, iv2)| {
            let iv1 = self.get(name);
            iv1.lo <= iv2.lo && iv1.hi >= iv2.hi
        })
    }
    /// Return the number of variables bound.
    pub fn len(&self) -> usize {
        self.bindings.len()
    }
    /// Return whether the environment is empty.
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
    /// Return all variable names.
    pub fn names(&self) -> Vec<&str> {
        self.bindings.iter().map(|(n, _)| n.as_str()).collect()
    }
}
/// Abstract domain for expression size analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeDomain {
    /// No information.
    Bottom,
    /// Exactly zero nodes.
    Zero,
    /// A small, known number of nodes.
    Small(usize),
    /// Large but finite (unknown exact count).
    Large,
    /// Unknown or unbounded size.
    Top,
}
#[allow(clippy::should_implement_trait)]
impl SizeDomain {
    /// Construct from a concrete node count.
    pub fn from_count(n: usize) -> Self {
        const SMALL_THRESHOLD: usize = 100;
        if n == 0 {
            SizeDomain::Zero
        } else if n <= SMALL_THRESHOLD {
            SizeDomain::Small(n)
        } else {
            SizeDomain::Large
        }
    }
    /// Abstract addition of two sizes.
    pub fn add(a: SizeDomain, b: SizeDomain) -> SizeDomain {
        use SizeDomain::*;
        match (a, b) {
            (Bottom, _) | (_, Bottom) => Bottom,
            (Top, _) | (_, Top) => Top,
            (Zero, x) | (x, Zero) => x,
            (Small(m), Small(n)) => SizeDomain::from_count(m + n),
            (Large, _) | (_, Large) => Large,
        }
    }
    /// Abstract maximum of two sizes.
    pub fn max(a: SizeDomain, b: SizeDomain) -> SizeDomain {
        use SizeDomain::*;
        match (a, b) {
            (Bottom, x) | (x, Bottom) => x,
            (Top, _) | (_, Top) => Top,
            (Zero, x) | (x, Zero) => x,
            (Small(m), Small(n)) => Small(m.max(n)),
            (Large, _) | (_, Large) => Large,
        }
    }
}
/// A summary of an abstract interpretation run.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct InterpretationSummary {
    pub iterations: u32,
    pub converged: bool,
    pub alarm_count: usize,
    pub proven_safe: bool,
}
#[allow(dead_code)]
impl InterpretationSummary {
    /// Create a new summary.
    pub fn new(iterations: u32, converged: bool, alarm_count: usize) -> Self {
        InterpretationSummary {
            iterations,
            converged,
            alarm_count,
            proven_safe: converged && alarm_count == 0,
        }
    }
    /// Format a brief description.
    pub fn describe(&self) -> String {
        format!(
            "iters={} converged={} alarms={} safe={}",
            self.iterations, self.converged, self.alarm_count, self.proven_safe
        )
    }
}
/// A transfer function maps an abstract state at a point to the next point.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TransferFunction {
    pub name: String,
    pub effects: Vec<TransferEffect>,
}
#[allow(dead_code)]
impl TransferFunction {
    /// Create an empty transfer function.
    pub fn new(name: impl Into<String>) -> Self {
        TransferFunction {
            name: name.into(),
            effects: Vec::new(),
        }
    }
    /// Add an effect.
    pub fn add_effect(&mut self, effect: TransferEffect) {
        self.effects.push(effect);
    }
    /// Apply the transfer function to an environment.
    pub fn apply(&self, env: &IntervalEnv) -> IntervalEnv {
        let result_bindings = env.bindings.clone();
        let mut result = IntervalEnv {
            bindings: result_bindings,
        };
        for effect in &self.effects {
            match effect {
                TransferEffect::Assign { var, interval } => {
                    result.set(var, *interval);
                }
                TransferEffect::Constrain { var, constraint } => {
                    let current = result.get(var);
                    let narrowed = current.meet(constraint);
                    result.set(var, narrowed);
                }
                TransferEffect::Invalidate { var } => {
                    result.set(var, Interval::top());
                }
            }
        }
        result
    }
    /// Return the number of effects.
    pub fn effect_count(&self) -> usize {
        self.effects.len()
    }
}
/// A function summary: pre/post conditions as interval environments.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FunctionSummary {
    pub function_name: String,
    pub precondition: IntervalEnv,
    pub postcondition: IntervalEnv,
    pub termination: TerminationEvidence,
    pub cost: CostBound,
}
#[allow(dead_code)]
impl FunctionSummary {
    /// Create a new summary.
    pub fn new(name: impl Into<String>) -> Self {
        FunctionSummary {
            function_name: name.into(),
            precondition: IntervalEnv::new(),
            postcondition: IntervalEnv::new(),
            termination: TerminationEvidence::Unknown,
            cost: CostBound::at_least(0),
        }
    }
    /// Return whether the function is proven to terminate.
    pub fn terminates(&self) -> bool {
        self.termination.is_proven()
    }
    /// Return a description of the summary.
    pub fn describe(&self) -> String {
        format!(
            "fn {}: terminates={} cost=[{}, {:?}]",
            self.function_name,
            self.terminates(),
            self.cost.lower,
            self.cost.upper
        )
    }
}
/// The nullness abstract domain: null, non-null, or top.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum NullnessDomain {
    Bottom,
    Null,
    NonNull,
    Top,
}
#[allow(dead_code)]
impl NullnessDomain {
    /// Create from a bool (true = non-null, false = null).
    pub fn from_bool(non_null: bool) -> Self {
        if non_null {
            NullnessDomain::NonNull
        } else {
            NullnessDomain::Null
        }
    }
    /// Join two values.
    pub fn join(&self, other: &NullnessDomain) -> NullnessDomain {
        use NullnessDomain::*;
        match (self, other) {
            (Bottom, x) | (x, Bottom) => *x,
            (Null, Null) => Null,
            (NonNull, NonNull) => NonNull,
            _ => Top,
        }
    }
    /// Return whether this might be null.
    pub fn may_be_null(&self) -> bool {
        matches!(self, NullnessDomain::Null | NullnessDomain::Top)
    }
    /// Return whether this is definitely non-null.
    pub fn is_definitely_non_null(&self) -> bool {
        *self == NullnessDomain::NonNull
    }
}
