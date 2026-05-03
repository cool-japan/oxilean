//! Functions for the reduction statistics module.
//!
//! These helpers operate on `ReductionStats` and provide instrumented
//! wrappers around the WHNF reducer for counting reductions.

use crate::{Environment, Expr, Reducer};

use super::types::{DepthGuard, ReductionKind, ReductionSession, ReductionStats};

// ── Instrumented reduce helpers ───────────────────────────────────────────────

/// Perform a single WHNF step on `expr` and record statistics.
///
/// The caller supplies a mutable `Reducer` and `ReductionStats`.
/// This wrapper:
/// 1. Pushes/pops the depth counter.
/// 2. Records a beta step if the outermost form is `App(Lam, _)`.
/// 3. Records a zeta step if the outermost form is `Let(...)`.
/// 4. Records a delta step if the outermost form is `Const(name, _)` that
///    has a definition in `env`.
///
/// Returns the reduced expression.
pub fn whnf_tracked(
    expr: &Expr,
    env: &Environment,
    reducer: &mut Reducer,
    stats: &mut ReductionStats,
) -> Expr {
    // Push depth before borrowing stats for specific counters
    stats.push_depth();

    // Classify what kind of top-level step will happen
    match expr {
        Expr::App(f, _) => {
            if let Expr::Lam(_, _, _, _) = f.as_ref() {
                stats.increment_beta();
            }
        }
        Expr::Let(_, _, _, _) => stats.increment_zeta(),
        Expr::Const(name, _) if env.find(name).and_then(|ci| ci.value()).is_some() => {
            stats.increment_delta();
        }
        _ => {}
    }

    let result = reducer.whnf_env(expr, env);
    stats.pop_depth();
    result
}

/// Run a full WHNF reduction under stats tracking and return both the
/// reduced expression and a `ReductionStats` delta for this call.
pub fn whnf_with_stats(
    expr: &Expr,
    env: &Environment,
    reducer: &mut Reducer,
    stats: &mut ReductionStats,
) -> (Expr, ReductionStats) {
    let session = ReductionSession::begin(stats);
    let result = whnf_tracked(expr, env, reducer, stats);
    let delta = session.finish(stats);
    (result, delta)
}

/// Apply `f` while tracking depth in `stats`.  Useful for instrumenting
/// recursive calls that don't go through `whnf_tracked`.
pub fn with_depth_tracking<T, F>(stats: &mut ReductionStats, f: F) -> T
where
    F: FnOnce() -> T,
{
    stats.push_depth();
    let result = f();
    stats.pop_depth();
    result
}

/// Record a reduction of the given kind, returning the current total.
pub fn record_reduction(stats: &mut ReductionStats, kind: ReductionKind) -> u64 {
    stats.increment(kind);
    stats.total()
}

/// Format a `ReductionStats` as a human-readable table.
pub fn format_stats(stats: &ReductionStats) -> String {
    format!(
        "┌─────────────────────────────┐\n\
         │      Reduction Statistics   │\n\
         ├──────────────┬──────────────┤\n\
         │ β (beta)     │ {:>12} │\n\
         │ δ (delta)    │ {:>12} │\n\
         │ ζ (zeta)     │ {:>12} │\n\
         │ ι (iota)     │ {:>12} │\n\
         │ η (eta)      │ {:>12} │\n\
         │ level        │ {:>12} │\n\
         ├──────────────┼──────────────┤\n\
         │ Total steps  │ {:>12} │\n\
         │ Max depth    │ {:>12} │\n\
         └──────────────┴──────────────┘",
        stats.beta_count,
        stats.delta_count,
        stats.zeta_count,
        stats.iota_count,
        stats.eta_count,
        stats.level_count,
        stats.total_steps,
        stats.max_depth,
    )
}

/// Compute a percentage breakdown of reduction kinds.
///
/// Returns a `Vec<(ReductionKind, f64)>` sorted by frequency descending.
/// Returns an empty vec if `stats.total()` is zero.
pub fn reduction_breakdown(stats: &ReductionStats) -> Vec<(ReductionKind, f64)> {
    let total = stats.total();
    if total == 0 {
        return Vec::new();
    }
    let mut entries = vec![
        (ReductionKind::Beta, stats.beta_count),
        (ReductionKind::Delta, stats.delta_count),
        (ReductionKind::Zeta, stats.zeta_count),
        (ReductionKind::Iota, stats.iota_count),
        (ReductionKind::Eta, stats.eta_count),
        (ReductionKind::Level, stats.level_count),
    ];
    entries.sort_by_key(|b| std::cmp::Reverse(b.1));
    entries
        .into_iter()
        .map(|(kind, count)| (kind, count as f64 / total as f64 * 100.0))
        .collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reduction_stats::types::{ReductionKind, ReductionStats};

    // ── Basic counting ──────────────────────────────────────────────────────

    #[test]
    fn test_increment_beta() {
        let mut s = ReductionStats::new();
        s.increment_beta();
        assert_eq!(s.beta_count, 1);
        assert_eq!(s.total(), 1);
    }

    #[test]
    fn test_increment_delta() {
        let mut s = ReductionStats::new();
        s.increment_delta();
        assert_eq!(s.delta_count, 1);
        assert_eq!(s.total(), 1);
    }

    #[test]
    fn test_increment_zeta() {
        let mut s = ReductionStats::new();
        s.increment_zeta();
        assert_eq!(s.zeta_count, 1);
        assert_eq!(s.total(), 1);
    }

    #[test]
    fn test_increment_iota() {
        let mut s = ReductionStats::new();
        s.increment_iota();
        assert_eq!(s.iota_count, 1);
        assert_eq!(s.total(), 1);
    }

    #[test]
    fn test_increment_eta() {
        let mut s = ReductionStats::new();
        s.increment_eta();
        assert_eq!(s.eta_count, 1);
        assert_eq!(s.total(), 1);
    }

    #[test]
    fn test_increment_level() {
        let mut s = ReductionStats::new();
        s.increment_level();
        assert_eq!(s.level_count, 1);
        assert_eq!(s.total(), 1);
    }

    #[test]
    fn test_increment_by_kind() {
        let mut s = ReductionStats::new();
        s.increment(ReductionKind::Beta);
        s.increment(ReductionKind::Delta);
        s.increment(ReductionKind::Zeta);
        s.increment(ReductionKind::Iota);
        s.increment(ReductionKind::Eta);
        s.increment(ReductionKind::Level);
        assert_eq!(s.total(), 6);
        assert_eq!(s.count_for(ReductionKind::Beta), 1);
        assert_eq!(s.count_for(ReductionKind::Delta), 1);
        assert_eq!(s.count_for(ReductionKind::Zeta), 1);
        assert_eq!(s.count_for(ReductionKind::Iota), 1);
        assert_eq!(s.count_for(ReductionKind::Eta), 1);
        assert_eq!(s.count_for(ReductionKind::Level), 1);
    }

    #[test]
    fn test_total_is_sum() {
        let mut s = ReductionStats::new();
        for _ in 0..5 {
            s.increment_beta();
        }
        for _ in 0..3 {
            s.increment_delta();
        }
        assert_eq!(s.total(), 8);
        assert_eq!(s.beta_count + s.delta_count, s.total_steps);
    }

    // ── Depth tracking ──────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_depth() {
        let mut s = ReductionStats::new();
        assert_eq!(s.current_depth(), 0);
        s.push_depth();
        assert_eq!(s.current_depth(), 1);
        s.push_depth();
        assert_eq!(s.current_depth(), 2);
        assert_eq!(s.max_depth, 2);
        s.pop_depth();
        assert_eq!(s.current_depth(), 1);
        s.pop_depth();
        assert_eq!(s.current_depth(), 0);
        assert_eq!(s.max_depth, 2); // max_depth doesn't decrease
    }

    #[test]
    fn test_pop_depth_saturates() {
        let mut s = ReductionStats::new();
        s.pop_depth(); // should not underflow
        assert_eq!(s.current_depth(), 0);
    }

    #[test]
    fn test_depth_guard() {
        let mut s = ReductionStats::new();
        // DepthGuard borrows `s` exclusively — test its effect after it drops
        {
            let _g = DepthGuard::new(&mut s);
            // Can't read s.current_depth() here due to exclusive borrow —
            // the guard is verified by observing max_depth after it drops.
        }
        // After drop: depth is back to 0; max_depth records the high-water mark
        assert_eq!(s.current_depth(), 0);
        assert_eq!(s.max_depth, 1, "DepthGuard should have pushed to depth 1");
    }

    #[test]
    fn test_nested_depth_guards() {
        let mut s = ReductionStats::new();
        // Test depth guard effect by using nested scopes and checking after drop
        {
            // Push manually to verify intermediate states
            s.push_depth(); // depth 1
            assert_eq!(s.current_depth(), 1);
            {
                s.push_depth(); // depth 2
                assert_eq!(s.current_depth(), 2);
                s.pop_depth(); // depth 1
            }
            assert_eq!(s.current_depth(), 1);
            s.pop_depth(); // depth 0
        }
        assert_eq!(s.current_depth(), 0);
        assert_eq!(s.max_depth, 2);
    }

    // ── Merge ───────────────────────────────────────────────────────────────

    #[test]
    fn test_merge() {
        let mut a = ReductionStats::new();
        a.increment_beta();
        a.increment_beta();
        a.push_depth();
        a.push_depth();

        let mut b = ReductionStats::new();
        b.increment_delta();
        b.push_depth();
        b.push_depth();
        b.push_depth();

        a.merge(&b);
        assert_eq!(a.beta_count, 2);
        assert_eq!(a.delta_count, 1);
        assert_eq!(a.total(), 3);
        assert_eq!(a.max_depth, 3); // max of a.max_depth=2 and b.max_depth=3
    }

    // ── Reset ───────────────────────────────────────────────────────────────

    #[test]
    fn test_reset() {
        let mut s = ReductionStats::new();
        s.increment_beta();
        s.increment_delta();
        s.push_depth();
        s.reset();
        assert_eq!(s.total(), 0);
        assert_eq!(s.max_depth, 0);
        assert_eq!(s.current_depth(), 0);
    }

    // ── Snapshot / delta ────────────────────────────────────────────────────

    #[test]
    fn test_snapshot_and_delta() {
        let mut s = ReductionStats::new();
        s.increment_beta();
        let snap = s.snapshot();
        s.increment_beta();
        s.increment_delta();
        let delta = s.delta_from(&snap);
        assert_eq!(delta.beta_count, 1);
        assert_eq!(delta.delta_count, 1);
        assert_eq!(delta.total(), 2);
    }

    #[test]
    fn test_session() {
        let mut s = ReductionStats::new();
        let session = ReductionSession::begin(&s);
        s.increment_beta();
        s.increment_iota();
        let delta = session.finish(&s);
        assert_eq!(delta.total(), 2);
        assert_eq!(delta.beta_count, 1);
        assert_eq!(delta.iota_count, 1);
    }

    // ── record_reduction ────────────────────────────────────────────────────

    #[test]
    fn test_record_reduction() {
        let mut s = ReductionStats::new();
        let total = record_reduction(&mut s, ReductionKind::Beta);
        assert_eq!(total, 1);
        let total2 = record_reduction(&mut s, ReductionKind::Iota);
        assert_eq!(total2, 2);
    }

    // ── breakdown ───────────────────────────────────────────────────────────

    #[test]
    fn test_breakdown_empty() {
        let s = ReductionStats::new();
        assert!(reduction_breakdown(&s).is_empty());
    }

    #[test]
    fn test_breakdown_percentages_sum_to_100() {
        let mut s = ReductionStats::new();
        for _ in 0..30 {
            s.increment_beta();
        }
        for _ in 0..70 {
            s.increment_delta();
        }
        let breakdown = reduction_breakdown(&s);
        let sum: f64 = breakdown.iter().map(|(_, pct)| pct).sum();
        assert!((sum - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_breakdown_sorted_descending() {
        let mut s = ReductionStats::new();
        s.increment_iota();
        s.increment_iota();
        s.increment_iota();
        s.increment_beta();
        let breakdown = reduction_breakdown(&s);
        assert_eq!(breakdown[0].0, ReductionKind::Iota);
        assert_eq!(breakdown[1].0, ReductionKind::Beta);
    }

    // ── is_empty ────────────────────────────────────────────────────────────

    #[test]
    fn test_is_empty() {
        let s = ReductionStats::new();
        assert!(s.is_empty());
        let mut s2 = ReductionStats::new();
        s2.increment_beta();
        assert!(!s2.is_empty());
    }

    // ── Display ─────────────────────────────────────────────────────────────

    #[test]
    fn test_display_contains_totals() {
        let mut s = ReductionStats::new();
        s.increment_beta();
        s.increment_delta();
        let text = format!("{}", s);
        assert!(text.contains("total"));
        assert!(text.contains("2") || text.contains("beta") || text.contains("delta"));
    }

    #[test]
    fn test_format_stats_contains_sections() {
        let s = ReductionStats::new();
        let table = format_stats(&s);
        assert!(table.contains("beta"));
        assert!(table.contains("delta"));
        assert!(table.contains("Total"));
        assert!(table.contains("Max depth"));
    }
}
