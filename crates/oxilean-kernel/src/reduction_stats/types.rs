//! Types for reduction statistics tracking.
//!
//! `ReductionStats` is a zero-cost (when unused) statistics accumulator for
//! the WHNF reducer.  Each kind of reduction is counted separately so that
//! profiling can identify hot-spots in the type-checking pipeline.

use std::fmt;

// ── Reduction kinds ───────────────────────────────────────────────────────────

/// The different reduction kinds tracked by `ReductionStats`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ReductionKind {
    /// β-reduction: `(λ x, t) a  →  t[a/x]`.
    Beta,
    /// δ-reduction: unfolding a definition.
    Delta,
    /// ζ-reduction: `let x := v in t  →  t[v/x]`.
    Zeta,
    /// ι-reduction: recursor / match reduction (iota).
    Iota,
    /// η-reduction / expansion.
    Eta,
    /// Universe level simplification.
    Level,
}

impl fmt::Display for ReductionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReductionKind::Beta => write!(f, "beta"),
            ReductionKind::Delta => write!(f, "delta"),
            ReductionKind::Zeta => write!(f, "zeta"),
            ReductionKind::Iota => write!(f, "iota"),
            ReductionKind::Eta => write!(f, "eta"),
            ReductionKind::Level => write!(f, "level"),
        }
    }
}

// ── ReductionStats ────────────────────────────────────────────────────────────

/// Accumulated statistics for a WHNF reduction session.
///
/// Counts are kept per reduction kind as well as totals.  The `max_depth`
/// field records the deepest reduction stack seen.
///
/// ```
/// use oxilean_kernel::reduction_stats::ReductionStats;
///
/// let mut stats = ReductionStats::new();
/// stats.increment_beta();
/// stats.increment_delta();
/// assert_eq!(stats.total(), 2);
/// ```
#[derive(Clone, Debug, Default)]
pub struct ReductionStats {
    /// Number of β-reductions performed.
    pub beta_count: u64,
    /// Number of δ-reductions (definition unfoldings) performed.
    pub delta_count: u64,
    /// Number of ζ-reductions (let-binding unfoldings) performed.
    pub zeta_count: u64,
    /// Number of ι-reductions (recursor/match evaluations) performed.
    pub iota_count: u64,
    /// Number of η-reductions performed.
    pub eta_count: u64,
    /// Number of universe-level simplifications performed.
    pub level_count: u64,
    /// Total reduction steps (sum of all individual counters).
    pub total_steps: u64,
    /// Maximum reduction depth encountered in a single WHNF call.
    pub max_depth: u64,
    /// Current depth (used during active reduction; reset between calls).
    current_depth: u64,
}

impl ReductionStats {
    /// Create a fresh zeroed statistics object.
    pub fn new() -> Self {
        Self::default()
    }

    // ── Increment helpers ─────────────────────────────────────────────────

    /// Record one β-reduction step.
    #[inline]
    pub fn increment_beta(&mut self) {
        self.beta_count += 1;
        self.total_steps += 1;
    }

    /// Record one δ-reduction step.
    #[inline]
    pub fn increment_delta(&mut self) {
        self.delta_count += 1;
        self.total_steps += 1;
    }

    /// Record one ζ-reduction step.
    #[inline]
    pub fn increment_zeta(&mut self) {
        self.zeta_count += 1;
        self.total_steps += 1;
    }

    /// Record one ι-reduction step.
    #[inline]
    pub fn increment_iota(&mut self) {
        self.iota_count += 1;
        self.total_steps += 1;
    }

    /// Record one η-reduction step.
    #[inline]
    pub fn increment_eta(&mut self) {
        self.eta_count += 1;
        self.total_steps += 1;
    }

    /// Record one universe-level simplification step.
    #[inline]
    pub fn increment_level(&mut self) {
        self.level_count += 1;
        self.total_steps += 1;
    }

    /// Increment the counter for the given `ReductionKind`.
    #[inline]
    pub fn increment(&mut self, kind: ReductionKind) {
        match kind {
            ReductionKind::Beta => self.increment_beta(),
            ReductionKind::Delta => self.increment_delta(),
            ReductionKind::Zeta => self.increment_zeta(),
            ReductionKind::Iota => self.increment_iota(),
            ReductionKind::Eta => self.increment_eta(),
            ReductionKind::Level => self.increment_level(),
        }
    }

    // ── Depth tracking ────────────────────────────────────────────────────

    /// Signal that the reducer is entering a deeper level.
    #[inline]
    pub fn push_depth(&mut self) {
        self.current_depth += 1;
        if self.current_depth > self.max_depth {
            self.max_depth = self.current_depth;
        }
    }

    /// Signal that the reducer is leaving a level.
    #[inline]
    pub fn pop_depth(&mut self) {
        self.current_depth = self.current_depth.saturating_sub(1);
    }

    /// Current reduction depth.
    #[inline]
    pub fn current_depth(&self) -> u64 {
        self.current_depth
    }

    // ── Aggregates ────────────────────────────────────────────────────────

    /// Total number of reduction steps across all kinds.
    #[inline]
    pub fn total(&self) -> u64 {
        self.total_steps
    }

    /// Count for a specific kind.
    pub fn count_for(&self, kind: ReductionKind) -> u64 {
        match kind {
            ReductionKind::Beta => self.beta_count,
            ReductionKind::Delta => self.delta_count,
            ReductionKind::Zeta => self.zeta_count,
            ReductionKind::Iota => self.iota_count,
            ReductionKind::Eta => self.eta_count,
            ReductionKind::Level => self.level_count,
        }
    }

    /// Returns `true` if no reductions have been recorded.
    pub fn is_empty(&self) -> bool {
        self.total_steps == 0
    }

    // ── Merge / reset ─────────────────────────────────────────────────────

    /// Merge another `ReductionStats` into `self` (add all counts).
    pub fn merge(&mut self, other: &ReductionStats) {
        self.beta_count += other.beta_count;
        self.delta_count += other.delta_count;
        self.zeta_count += other.zeta_count;
        self.iota_count += other.iota_count;
        self.eta_count += other.eta_count;
        self.level_count += other.level_count;
        self.total_steps += other.total_steps;
        if other.max_depth > self.max_depth {
            self.max_depth = other.max_depth;
        }
    }

    /// Reset all counters to zero.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    // ── Snapshot ──────────────────────────────────────────────────────────

    /// Take a snapshot (clone) of the current statistics.
    pub fn snapshot(&self) -> ReductionStats {
        self.clone()
    }

    /// Compute the delta between `self` (later) and a prior `snapshot`.
    pub fn delta_from(&self, snapshot: &ReductionStats) -> ReductionStats {
        ReductionStats {
            beta_count: self.beta_count.saturating_sub(snapshot.beta_count),
            delta_count: self.delta_count.saturating_sub(snapshot.delta_count),
            zeta_count: self.zeta_count.saturating_sub(snapshot.zeta_count),
            iota_count: self.iota_count.saturating_sub(snapshot.iota_count),
            eta_count: self.eta_count.saturating_sub(snapshot.eta_count),
            level_count: self.level_count.saturating_sub(snapshot.level_count),
            total_steps: self.total_steps.saturating_sub(snapshot.total_steps),
            max_depth: self.max_depth,
            current_depth: 0,
        }
    }
}

impl fmt::Display for ReductionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ReductionStats {{ total: {}, beta: {}, delta: {}, zeta: {}, iota: {}, eta: {}, level: {}, max_depth: {} }}",
            self.total_steps,
            self.beta_count,
            self.delta_count,
            self.zeta_count,
            self.iota_count,
            self.eta_count,
            self.level_count,
            self.max_depth,
        )
    }
}

// ── ReductionSession ──────────────────────────────────────────────────────────

/// A RAII guard that tracks a single reduction "session".
///
/// On creation it records a snapshot of the global stats; on drop (or when
/// [`ReductionSession::finish`] is called) it returns the delta.
///
/// ```ignore
/// let session = ReductionSession::begin(&mut stats);
/// // ... run the reducer ...
/// let delta = session.finish(&stats);
/// println!("Steps in this call: {}", delta.total());
/// ```
pub struct ReductionSession {
    snapshot: ReductionStats,
}

impl ReductionSession {
    /// Begin a session by snapshotting the current stats.
    pub fn begin(stats: &ReductionStats) -> Self {
        Self {
            snapshot: stats.snapshot(),
        }
    }

    /// Finish the session and return the delta since `begin`.
    pub fn finish(self, current: &ReductionStats) -> ReductionStats {
        current.delta_from(&self.snapshot)
    }
}

// ── DepthGuard ────────────────────────────────────────────────────────────────

/// A RAII guard for depth tracking.
///
/// Calls `push_depth` on construction and `pop_depth` on drop.
/// Use this to instrument recursive WHNF calls.
///
/// ```ignore
/// fn whnf_recursive(expr: &Expr, stats: &mut ReductionStats) -> Expr {
///     let _guard = DepthGuard::new(stats);
///     // ... reduction logic ...
/// }
/// ```
pub struct DepthGuard<'a> {
    stats: &'a mut ReductionStats,
}

impl<'a> DepthGuard<'a> {
    /// Push the depth counter and return a guard that pops on drop.
    pub fn new(stats: &'a mut ReductionStats) -> Self {
        stats.push_depth();
        Self { stats }
    }
}

impl Drop for DepthGuard<'_> {
    fn drop(&mut self) {
        self.stats.pop_depth();
    }
}
