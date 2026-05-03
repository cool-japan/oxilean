//! Per-invocation context types: SimpStats, SimpContext, SimpReport, SimpLemmaDatabase.

#![allow(dead_code)]
#![allow(missing_docs)]

use oxilean_kernel::{Expr, Name};

use super::types::{default_simp_lemmas, SimpConfig, SimpLemma, SimpResult, SimpTheorems};

// ============================================================
// SimpStats: aggregate statistics for a simp run
// ============================================================

/// Statistics collected during a single simp invocation.
#[derive(Clone, Debug, Default)]
pub struct SimpStats {
    /// Number of lemmas tried.
    pub lemmas_tried: u64,
    /// Number of successful rewrites.
    pub rewrites_applied: u64,
    /// Number of beta-reduction steps.
    pub beta_steps: u64,
    /// Number of eta-reduction steps.
    pub eta_steps: u64,
    /// Number of iota-reduction steps.
    pub iota_steps: u64,
    /// Number of zeta-reduction steps.
    pub zeta_steps: u64,
    /// Number of congruence closure applications.
    pub congr_steps: u64,
    /// Number of side goals generated and discharged.
    pub side_goals_discharged: u64,
    /// Number of side goals that failed to discharge.
    pub side_goals_failed: u64,
    /// Total subexpressions visited.
    pub exprs_visited: u64,
    /// Whether the budget was exhausted.
    pub budget_exhausted: bool,
}

impl SimpStats {
    /// Create zero-initialized stats.
    pub fn new() -> Self {
        Self::default()
    }

    /// Total reduction steps (not counting congruence/lemmas).
    pub fn total_reduction_steps(&self) -> u64 {
        self.beta_steps + self.eta_steps + self.iota_steps + self.zeta_steps
    }

    /// Whether any progress was made.
    pub fn any_progress(&self) -> bool {
        self.rewrites_applied > 0 || self.total_reduction_steps() > 0
    }

    /// Whether all side goals were discharged.
    pub fn all_side_goals_discharged(&self) -> bool {
        self.side_goals_failed == 0
    }

    /// Add another stats record into this one.
    pub fn merge(&mut self, other: &SimpStats) {
        self.lemmas_tried += other.lemmas_tried;
        self.rewrites_applied += other.rewrites_applied;
        self.beta_steps += other.beta_steps;
        self.eta_steps += other.eta_steps;
        self.iota_steps += other.iota_steps;
        self.zeta_steps += other.zeta_steps;
        self.congr_steps += other.congr_steps;
        self.side_goals_discharged += other.side_goals_discharged;
        self.side_goals_failed += other.side_goals_failed;
        self.exprs_visited += other.exprs_visited;
        self.budget_exhausted |= other.budget_exhausted;
    }
}

impl std::fmt::Display for SimpStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SimpStats {{ rewrites: {}, lemmas_tried: {}, beta: {}, iota: {} }}",
            self.rewrites_applied, self.lemmas_tried, self.beta_steps, self.iota_steps
        )
    }
}

// ============================================================
// SimpContext: per-invocation mutable state
// ============================================================

/// Mutable state for a single simp invocation.
pub struct SimpContext<'a> {
    /// Configuration controlling the simp run.
    pub config: &'a SimpConfig,
    /// The active simp lemma database.
    pub theorems: &'a SimpTheorems,
    /// Accumulated statistics.
    pub stats: SimpStats,
    /// Remaining step budget.
    pub budget: u32,
    /// Additional locally-scoped lemmas.
    pub local_lemmas: Vec<SimpLemma>,
    /// Names of lemmas to exclude.
    pub excluded: Vec<Name>,
}

impl<'a> SimpContext<'a> {
    /// Create a new simp context.
    pub fn new(config: &'a SimpConfig, theorems: &'a SimpTheorems) -> Self {
        Self {
            budget: config.max_steps,
            config,
            theorems,
            stats: SimpStats::new(),
            local_lemmas: Vec::new(),
            excluded: Vec::new(),
        }
    }

    /// Add a local lemma for this invocation.
    pub fn add_local_lemma(&mut self, lemma: SimpLemma) {
        self.local_lemmas.push(lemma);
    }

    /// Exclude a lemma by name.
    pub fn exclude(&mut self, name: Name) {
        self.excluded.push(name);
    }

    /// Check whether the given lemma is excluded.
    pub fn is_excluded(&self, name: &Name) -> bool {
        self.excluded.contains(name)
    }

    /// Consume one step from the budget.
    ///
    /// Returns false when the budget is exhausted.
    pub fn consume_budget(&mut self) -> bool {
        if self.budget == 0 {
            self.stats.budget_exhausted = true;
            return false;
        }
        self.budget -= 1;
        true
    }

    /// Whether the budget is still active.
    pub fn has_budget(&self) -> bool {
        self.budget > 0
    }
}

// ============================================================
// SimpReport: post-run summary
// ============================================================

/// A report produced after a simp run.
#[derive(Clone, Debug)]
pub struct SimpReport {
    /// Whether the goal was fully proved.
    pub proved: bool,
    /// Whether the expression changed.
    pub simplified: bool,
    /// The resulting expression (after simplification).
    pub result: Option<Expr>,
    /// Statistics from the run.
    pub stats: SimpStats,
    /// Names of lemmas that fired.
    pub lemmas_used: Vec<Name>,
}

impl SimpReport {
    /// Create a report for an unchanged expression.
    pub fn unchanged(expr: Expr) -> Self {
        Self {
            proved: false,
            simplified: false,
            result: Some(expr),
            stats: SimpStats::new(),
            lemmas_used: Vec::new(),
        }
    }

    /// Create a report for a simplified expression.
    pub fn simplified(result: Expr, stats: SimpStats) -> Self {
        Self {
            proved: false,
            simplified: true,
            result: Some(result),
            stats,
            lemmas_used: Vec::new(),
        }
    }

    /// Create a report for a proved goal.
    pub fn proved(stats: SimpStats) -> Self {
        Self {
            proved: true,
            simplified: true,
            result: None,
            stats,
            lemmas_used: Vec::new(),
        }
    }

    /// Add a lemma to the "used" list.
    pub fn record_lemma(&mut self, name: Name) {
        if !self.lemmas_used.contains(&name) {
            self.lemmas_used.push(name);
        }
    }
}

impl std::fmt::Display for SimpReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.proved {
            write!(f, "SimpReport {{ proved, {} }}", self.stats)
        } else if self.simplified {
            write!(f, "SimpReport {{ simplified, {} }}", self.stats)
        } else {
            write!(f, "SimpReport {{ unchanged, {} }}", self.stats)
        }
    }
}

// ============================================================
// SimpLemmaDatabase: a named, versioned collection
// ============================================================

/// A named, versioned database of simp lemmas.
#[derive(Clone, Debug)]
pub struct SimpLemmaDatabase {
    /// Database label (e.g., "default", "ring").
    pub label: String,
    /// Version counter, incremented on each mutation.
    pub version: u64,
    /// Underlying theorem storage.
    pub theorems: SimpTheorems,
}

impl SimpLemmaDatabase {
    /// Create a new empty database with a label.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            version: 0,
            theorems: SimpTheorems::new(),
        }
    }

    /// Create the default simp database.
    pub fn default_db() -> Self {
        Self {
            label: "default".to_string(),
            version: 1,
            theorems: default_simp_lemmas(),
        }
    }

    /// Add a lemma, bumping the version.
    pub fn add(&mut self, lemma: SimpLemma) {
        self.theorems.add_lemma(lemma);
        self.version += 1;
    }

    /// Remove a lemma by name, bumping the version.
    pub fn remove(&mut self, name: &Name) {
        self.theorems.remove_lemma(name);
        self.version += 1;
    }

    /// Number of lemmas in the database.
    pub fn len(&self) -> usize {
        self.theorems.num_lemmas()
    }

    /// Whether the database is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Expr, Level, Name};

    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }

    fn prop_expr() -> Expr {
        Expr::Sort(Level::zero())
    }

    #[test]
    fn test_simp_stats_default() {
        let s = SimpStats::new();
        assert_eq!(s.rewrites_applied, 0);
        assert!(!s.any_progress());
        assert!(s.all_side_goals_discharged());
    }

    #[test]
    fn test_simp_stats_merge() {
        let mut s1 = SimpStats {
            rewrites_applied: 3,
            beta_steps: 2,
            ..SimpStats::default()
        };
        let s2 = SimpStats {
            rewrites_applied: 1,
            iota_steps: 5,
            ..SimpStats::default()
        };
        s1.merge(&s2);
        assert_eq!(s1.rewrites_applied, 4);
        assert_eq!(s1.iota_steps, 5);
        assert_eq!(s1.beta_steps, 2);
    }

    #[test]
    fn test_simp_stats_any_progress() {
        let s = SimpStats {
            rewrites_applied: 1,
            ..SimpStats::default()
        };
        assert!(s.any_progress());
    }

    #[test]
    fn test_simp_stats_all_side_goals_discharged_false() {
        let s = SimpStats {
            side_goals_failed: 1,
            ..SimpStats::default()
        };
        assert!(!s.all_side_goals_discharged());
    }

    #[test]
    fn test_simp_stats_total_reduction_steps() {
        let s = SimpStats {
            beta_steps: 1,
            eta_steps: 2,
            iota_steps: 3,
            zeta_steps: 4,
            ..SimpStats::default()
        };
        assert_eq!(s.total_reduction_steps(), 10);
    }

    #[test]
    fn test_simp_stats_display() {
        let s = SimpStats {
            rewrites_applied: 5,
            lemmas_tried: 10,
            ..SimpStats::default()
        };
        let display = format!("{}", s);
        assert!(display.contains("rewrites: 5"));
    }

    #[test]
    fn test_simp_context_consume_budget() {
        let config = SimpConfig::default();
        let theorems = SimpTheorems::new();
        let mut ctx = SimpContext::new(&config, &theorems);
        assert!(ctx.has_budget());
        for _ in 0..100 {
            ctx.consume_budget();
        }
        assert_eq!(ctx.budget, config.max_steps - 100);
    }

    #[test]
    fn test_simp_context_exclude() {
        let config = SimpConfig::default();
        let theorems = SimpTheorems::new();
        let mut ctx = SimpContext::new(&config, &theorems);
        let name = Name::str("bad_lemma");
        ctx.exclude(name.clone());
        assert!(ctx.is_excluded(&name));
        assert!(!ctx.is_excluded(&Name::str("good_lemma")));
    }

    #[test]
    fn test_simp_report_unchanged() {
        let r = SimpReport::unchanged(nat_expr());
        assert!(!r.proved);
        assert!(!r.simplified);
        assert!(r.result.is_some());
    }

    #[test]
    fn test_simp_report_simplified() {
        let r = SimpReport::simplified(nat_expr(), SimpStats::new());
        assert!(!r.proved);
        assert!(r.simplified);
    }

    #[test]
    fn test_simp_report_proved() {
        let r = SimpReport::proved(SimpStats::new());
        assert!(r.proved);
        assert!(r.simplified);
        assert!(r.result.is_none());
    }

    #[test]
    fn test_simp_report_record_lemma() {
        let mut r = SimpReport::unchanged(nat_expr());
        r.record_lemma(Name::str("add_comm"));
        r.record_lemma(Name::str("add_comm")); // duplicate
        assert_eq!(r.lemmas_used.len(), 1);
    }

    #[test]
    fn test_simp_report_display() {
        let r = SimpReport::unchanged(nat_expr());
        let s = format!("{}", r);
        assert!(s.contains("unchanged"));
    }

    #[test]
    fn test_simp_lemma_database_new() {
        let db = SimpLemmaDatabase::new("test");
        assert_eq!(db.label, "test");
        assert_eq!(db.version, 0);
        assert!(db.is_empty());
    }

    #[test]
    fn test_simp_lemma_database_default_db() {
        let db = SimpLemmaDatabase::default_db();
        assert_eq!(db.label, "default");
        assert!(db.version > 0);
    }

    #[test]
    fn test_simp_lemma_database_remove_bumps_version() {
        let mut db = SimpLemmaDatabase::new("test");
        let v0 = db.version;
        db.remove(&Name::str("nonexistent"));
        assert_eq!(db.version, v0 + 1);
    }

    #[test]
    fn test_simp_config_default() {
        let cfg = SimpConfig::default();
        assert!(cfg.beta);
        assert!(cfg.use_default_lemmas);
        assert!(!cfg.simp_hyps);
    }

    #[test]
    fn test_simp_config_only() {
        let cfg = SimpConfig::only();
        assert!(!cfg.use_default_lemmas);
    }

    #[test]
    fn test_simp_context_budget_exhausted() {
        let config = SimpConfig {
            max_steps: 2,
            ..SimpConfig::default()
        };
        let theorems = SimpTheorems::new();
        let mut ctx = SimpContext::new(&config, &theorems);
        assert!(ctx.consume_budget());
        assert!(ctx.consume_budget());
        assert!(!ctx.consume_budget());
        assert!(ctx.stats.budget_exhausted);
    }

    #[test]
    fn test_simp_result_is_simplified() {
        let r = SimpResult::Simplified {
            new_expr: nat_expr(),
            proof: Some(prop_expr()),
        };
        assert!(r.is_simplified());
        assert!(!r.is_proved());
    }

    #[test]
    fn test_simp_result_unchanged() {
        let r = SimpResult::Unchanged;
        assert!(!r.is_simplified());
    }

    #[test]
    fn test_simp_context_add_local_lemma() {
        let cfg = SimpConfig::default();
        let theorems = SimpTheorems::new();
        let mut ctx = SimpContext::new(&cfg, &theorems);
        let lemma = SimpLemma {
            name: Name::str("my_lemma"),
            lhs: Expr::Sort(Level::zero()),
            rhs: Expr::Sort(Level::zero()),
            proof: Expr::Sort(Level::zero()),
            is_conditional: false,
            is_forward: true,
            priority: 1000,
        };
        ctx.add_local_lemma(lemma);
        assert_eq!(ctx.local_lemmas.len(), 1);
    }

    #[test]
    fn test_simp_lemma_database_add_bumps_version() {
        let mut db = SimpLemmaDatabase::new("test");
        let v0 = db.version;
        let lemma = SimpLemma {
            name: Name::str("test_lemma"),
            lhs: Expr::Sort(Level::zero()),
            rhs: Expr::Sort(Level::zero()),
            proof: Expr::Sort(Level::zero()),
            is_conditional: false,
            is_forward: true,
            priority: 1000,
        };
        db.add(lemma);
        assert_eq!(db.version, v0 + 1);
        assert_eq!(db.len(), 1);
    }

    #[test]
    fn test_simp_stats_budget_exhausted_merge() {
        let mut s1 = SimpStats::default();
        let s2 = SimpStats {
            budget_exhausted: true,
            ..SimpStats::default()
        };
        s1.merge(&s2);
        assert!(s1.budget_exhausted);
    }
}
