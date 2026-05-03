//! Extension types: SimpLemmaFilter, SimpTrace, SimpNormalForm, SimpConfigExt,
//! SimpLemmaCache, SimpScheduler, SimpBudget.

#![allow(dead_code)]
#![allow(missing_docs)]

use oxilean_kernel::Name;

use super::types::SimpConfig;

// ============================================================
// SimpLemmaFilter: predicate-based lemma filtering
// ============================================================

/// A filter for selecting which simp lemmas to apply.
#[derive(Clone, Debug)]
pub struct SimpLemmaFilter {
    /// If Some, only apply lemmas whose names start with this prefix.
    pub name_prefix: Option<String>,
    /// If Some, only apply lemmas with priority >= this value.
    pub min_priority: Option<u32>,
    /// If true, exclude conditional lemmas.
    pub exclude_conditional: bool,
}

impl SimpLemmaFilter {
    /// Create a filter that passes all lemmas.
    pub fn all() -> Self {
        Self {
            name_prefix: None,
            min_priority: None,
            exclude_conditional: false,
        }
    }

    /// Create a filter for lemmas with a given name prefix.
    pub fn with_prefix(prefix: &str) -> Self {
        Self {
            name_prefix: Some(prefix.to_string()),
            ..Self::all()
        }
    }

    /// Test whether a lemma name passes the filter.
    pub fn passes(&self, name: &Name) -> bool {
        if let Some(prefix) = &self.name_prefix {
            if !name.to_string().starts_with(prefix.to_string().as_str()) {
                return false;
            }
        }
        true
    }
}

// ============================================================
// SimpTrace: records fired lemmas in order
// ============================================================

/// Records the sequence of simp lemmas that fired during a simp run.
#[derive(Clone, Debug, Default)]
pub struct SimpTrace {
    /// Lemma names in the order they fired.
    pub fired: Vec<Name>,
    /// Whether tracing is enabled.
    pub enabled: bool,
}

impl SimpTrace {
    /// Create an enabled trace.
    pub fn enabled() -> Self {
        Self {
            fired: Vec::new(),
            enabled: true,
        }
    }

    /// Create a disabled trace.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a lemma firing.
    pub fn record(&mut self, name: Name) {
        if self.enabled {
            self.fired.push(name);
        }
    }

    /// Number of lemma firings recorded.
    pub fn len(&self) -> usize {
        self.fired.len()
    }

    /// Whether no lemmas were recorded.
    pub fn is_empty(&self) -> bool {
        self.fired.is_empty()
    }

    /// Clear the trace.
    pub fn clear(&mut self) {
        self.fired.clear();
    }
}

impl std::fmt::Display for SimpTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpTrace({} firings)", self.fired.len())
    }
}

// ============================================================
// SimpNormalForm: result of normalizing with simp
// ============================================================

/// The result of computing the simp normal form of an expression.
#[derive(Clone, Debug)]
pub struct SimpNormalForm {
    /// The normalized expression.
    pub expr: oxilean_kernel::Expr,
    /// Whether the expression was changed.
    pub changed: bool,
    /// Lemmas used.
    pub lemmas: Vec<Name>,
}

impl SimpNormalForm {
    /// Create an unchanged normal form.
    pub fn unchanged(expr: oxilean_kernel::Expr) -> Self {
        Self {
            expr,
            changed: false,
            lemmas: Vec::new(),
        }
    }

    /// Create a changed normal form.
    pub fn changed(expr: oxilean_kernel::Expr, lemmas: Vec<Name>) -> Self {
        Self {
            expr,
            changed: true,
            lemmas,
        }
    }
}

// ============================================================
// SimpConfigExt: builder API trait for SimpConfig
// ============================================================

/// Additional builder methods for `SimpConfig`.
pub trait SimpConfigExt {
    /// Enable all reductions.
    fn all_reductions(self) -> SimpConfig;
    /// Disable all reductions (lemma-only mode).
    fn lemma_only(self) -> SimpConfig;
    /// Set a custom max_steps.
    fn with_steps(self, n: u32) -> SimpConfig;
}

impl SimpConfigExt for SimpConfig {
    fn all_reductions(mut self) -> SimpConfig {
        self.beta = true;
        self.eta = true;
        self.iota = true;
        self.zeta = true;
        self
    }

    fn lemma_only(mut self) -> SimpConfig {
        self.beta = false;
        self.eta = false;
        self.iota = false;
        self.zeta = false;
        self
    }

    fn with_steps(mut self, n: u32) -> SimpConfig {
        self.max_steps = n;
        self
    }
}

// ============================================================
// SimpLemmaCache: memoized lemma lookup
// ============================================================

/// A simple cache mapping name strings to lemma counts.
#[derive(Clone, Debug, Default)]
pub struct SimpLemmaCache {
    /// lookup_count: how many times each lemma was looked up
    pub lookups: std::collections::HashMap<String, u64>,
}

impl SimpLemmaCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a lemma lookup.
    pub fn record_lookup(&mut self, name: &oxilean_kernel::Name) {
        *self.lookups.entry(name.to_string()).or_insert(0) += 1;
    }

    /// Total number of lookups.
    pub fn total_lookups(&self) -> u64 {
        self.lookups.values().sum()
    }

    /// Most-looked-up lemma name.
    pub fn hottest_lemma(&self) -> Option<&str> {
        self.lookups
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(k, _)| k.as_str())
    }
}

impl std::fmt::Display for SimpLemmaCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SimpLemmaCache({} distinct lemmas, {} total)",
            self.lookups.len(),
            self.total_lookups()
        )
    }
}

// ============================================================
// SimpScheduler: manages the order in which lemmas are tried
// ============================================================

/// Determines the order in which simp lemmas are tried.
#[derive(Clone, Debug, Default)]
pub struct SimpScheduler {
    /// Lemma names ordered by priority (highest first).
    pub(super) ordered: Vec<(u32, oxilean_kernel::Name)>,
}

impl SimpScheduler {
    /// Create an empty scheduler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a lemma with its priority.
    pub fn register(&mut self, name: oxilean_kernel::Name, priority: u32) {
        // Insert maintaining sorted order (descending priority)
        let pos = self.ordered.partition_point(|(p, _)| *p >= priority);
        self.ordered.insert(pos, (priority, name));
    }

    /// Deregister a lemma.
    pub fn deregister(&mut self, name: &oxilean_kernel::Name) {
        self.ordered.retain(|(_, n)| n != name);
    }

    /// Iterate lemma names in priority order.
    pub fn iter_by_priority(&self) -> impl Iterator<Item = &oxilean_kernel::Name> {
        self.ordered.iter().map(|(_, n)| n)
    }

    /// Number of registered lemmas.
    pub fn len(&self) -> usize {
        self.ordered.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.ordered.is_empty()
    }

    /// Get the highest-priority lemma name.
    pub fn top(&self) -> Option<&oxilean_kernel::Name> {
        self.ordered.first().map(|(_, n)| n)
    }
}

impl std::fmt::Display for SimpScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpScheduler({} lemmas)", self.len())
    }
}

// ============================================================
// SimpBudget: tracks remaining simp work
// ============================================================

/// Tracks work budget for a simp invocation.
#[derive(Clone, Debug)]
pub struct SimpBudget {
    /// Total step budget.
    pub(super) total: u32,
    /// Remaining steps.
    pub(super) remaining: u32,
    /// Whether the budget was exhausted.
    pub(super) exhausted: bool,
}

impl SimpBudget {
    /// Create a budget with `total` steps.
    pub fn new(total: u32) -> Self {
        Self {
            total,
            remaining: total,
            exhausted: false,
        }
    }

    /// Consume `n` steps. Returns `false` if budget is exhausted.
    pub fn consume(&mut self, n: u32) -> bool {
        if self.remaining < n {
            self.remaining = 0;
            self.exhausted = true;
            false
        } else {
            self.remaining -= n;
            true
        }
    }

    /// Check if any budget remains.
    pub fn has_budget(&self) -> bool {
        self.remaining > 0
    }

    /// Whether budget was exhausted.
    pub fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    /// Remaining steps.
    pub fn remaining(&self) -> u32 {
        self.remaining
    }

    /// Total steps.
    pub fn total(&self) -> u32 {
        self.total
    }

    /// Used steps.
    pub fn used(&self) -> u32 {
        self.total - self.remaining
    }

    /// Fraction used (0.0–1.0).
    pub fn fraction_used(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            self.used() as f32 / self.total as f32
        }
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Level, Name};

    #[test]
    fn test_simp_lemma_filter_all_passes() {
        let f = SimpLemmaFilter::all();
        let n = Name::str("add_comm");
        assert!(f.passes(&n));
    }

    #[test]
    fn test_simp_lemma_filter_prefix() {
        let f = SimpLemmaFilter::with_prefix("add_");
        assert!(f.passes(&Name::str("add_comm")));
        assert!(!f.passes(&Name::str("mul_comm")));
    }

    #[test]
    fn test_simp_trace_enabled_records() {
        let mut t = SimpTrace::enabled();
        t.record(Name::str("add_zero"));
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn test_simp_trace_disabled_does_not_record() {
        let mut t = SimpTrace::new();
        t.record(Name::str("add_zero"));
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn test_simp_trace_clear() {
        let mut t = SimpTrace::enabled();
        t.record(Name::str("x"));
        t.clear();
        assert!(t.is_empty());
    }

    #[test]
    fn test_simp_trace_display() {
        let t = SimpTrace::enabled();
        let s = format!("{}", t);
        assert!(s.contains("SimpTrace"));
    }

    #[test]
    fn test_simp_normal_form_unchanged() {
        use oxilean_kernel::Expr;
        let e = Expr::Sort(Level::zero());
        let nf = SimpNormalForm::unchanged(e);
        assert!(!nf.changed);
        assert!(nf.lemmas.is_empty());
    }

    #[test]
    fn test_simp_normal_form_changed() {
        use oxilean_kernel::Expr;
        let e = Expr::Sort(Level::zero());
        let nf = SimpNormalForm::changed(e, vec![Name::str("add_comm")]);
        assert!(nf.changed);
        assert_eq!(nf.lemmas.len(), 1);
    }

    #[test]
    fn test_simp_config_all_reductions() {
        let cfg = SimpConfig::default().all_reductions();
        assert!(cfg.beta);
        assert!(cfg.iota);
    }

    #[test]
    fn test_simp_config_lemma_only() {
        let cfg = SimpConfig::default().lemma_only();
        assert!(!cfg.beta);
        assert!(!cfg.iota);
    }

    #[test]
    fn test_simp_config_with_steps() {
        let cfg = SimpConfig::default().with_steps(999);
        assert_eq!(cfg.max_steps, 999);
    }

    #[test]
    fn test_simp_lemma_cache_record_lookup() {
        let mut c = SimpLemmaCache::new();
        c.record_lookup(&Name::str("add_comm"));
        c.record_lookup(&Name::str("add_comm"));
        assert_eq!(
            *c.lookups
                .get("add_comm")
                .expect("element at 'add_comm' should exist"),
            2
        );
    }

    #[test]
    fn test_simp_lemma_cache_total() {
        let mut c = SimpLemmaCache::new();
        c.record_lookup(&Name::str("a"));
        c.record_lookup(&Name::str("b"));
        assert_eq!(c.total_lookups(), 2);
    }

    #[test]
    fn test_simp_lemma_cache_hottest() {
        let mut c = SimpLemmaCache::new();
        c.record_lookup(&Name::str("rare"));
        c.record_lookup(&Name::str("hot"));
        c.record_lookup(&Name::str("hot"));
        assert_eq!(c.hottest_lemma(), Some("hot"));
    }

    #[test]
    fn test_simp_lemma_cache_display() {
        let c = SimpLemmaCache::new();
        let s = format!("{}", c);
        assert!(s.contains("SimpLemmaCache"));
    }

    #[test]
    fn test_simp_scheduler_register_order() {
        let mut s = SimpScheduler::new();
        s.register(Name::str("low"), 10);
        s.register(Name::str("high"), 1000);
        s.register(Name::str("mid"), 500);
        let names: Vec<_> = s.iter_by_priority().collect();
        assert_eq!(names[0], &Name::str("high"));
        assert_eq!(names[2], &Name::str("low"));
    }

    #[test]
    fn test_simp_scheduler_deregister() {
        let mut s = SimpScheduler::new();
        s.register(Name::str("a"), 100);
        s.deregister(&Name::str("a"));
        assert!(s.is_empty());
    }

    #[test]
    fn test_simp_scheduler_top() {
        let mut s = SimpScheduler::new();
        s.register(Name::str("x"), 50);
        s.register(Name::str("y"), 200);
        assert_eq!(s.top(), Some(&Name::str("y")));
    }

    #[test]
    fn test_simp_scheduler_display() {
        let s = SimpScheduler::new();
        let d = format!("{}", s);
        assert!(d.contains("SimpScheduler"));
    }

    #[test]
    fn test_simp_budget_consume() {
        let mut b = SimpBudget::new(10);
        assert!(b.consume(5));
        assert_eq!(b.remaining(), 5);
        assert!(!b.is_exhausted());
    }

    #[test]
    fn test_simp_budget_exhausted() {
        let mut b = SimpBudget::new(3);
        assert!(!b.consume(5));
        assert!(b.is_exhausted());
        assert_eq!(b.remaining(), 0);
    }

    #[test]
    fn test_simp_budget_fraction_used() {
        let mut b = SimpBudget::new(100);
        b.consume(25);
        assert!((b.fraction_used() - 0.25).abs() < 1e-5);
    }

    #[test]
    fn test_simp_budget_used() {
        let mut b = SimpBudget::new(20);
        b.consume(7);
        assert_eq!(b.used(), 7);
    }

    #[test]
    fn test_simp_budget_zero_total() {
        let b = SimpBudget::new(0);
        assert_eq!(b.fraction_used(), 0.0);
    }

    #[test]
    fn test_simp_scheduler_len() {
        let mut s = SimpScheduler::new();
        assert_eq!(s.len(), 0);
        s.register(Name::str("a"), 1);
        s.register(Name::str("b"), 2);
        assert_eq!(s.len(), 2);
    }
}
