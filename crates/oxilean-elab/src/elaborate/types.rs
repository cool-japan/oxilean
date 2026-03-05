//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

/// Elaboration error.
#[derive(Clone, Debug)]
pub enum ElabError {
    /// Kernel error
    Kernel(oxilean_kernel::KernelError),
    /// Name not found
    NameNotFound(String),
    /// Type error
    TypeError(String),
    /// Ambiguous
    Ambiguous(String),
    /// Implicit argument resolution failed
    ImplicitArgFailed(String),
    /// Overload resolution is ambiguous
    OverloadAmbiguity(String),
    /// Coercion insertion failed
    CoercionFailed(String),
    /// Tactic evaluation failed
    TacticFailed(String),
    /// Other error
    Other(String),
}
/// Tracks metrics for a single elaboration run over a file.
#[derive(Clone, Debug, Default)]
pub struct ElabRunMetrics {
    /// Total declarations elaborated.
    pub decls_elaborated: u64,
    /// Declarations that failed elaboration.
    pub decls_failed: u64,
    /// Total expressions elaborated.
    pub exprs_elaborated: u64,
    /// Total implicit arguments inserted.
    pub implicits_inserted: u64,
    /// Total metavariables created.
    pub metavars_created: u64,
    /// Total metavariables solved.
    pub metavars_solved: u64,
}
impl ElabRunMetrics {
    /// Create zeroed metrics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a declaration elaboration result.
    pub fn record_decl(&mut self, ok: bool) {
        self.decls_elaborated += 1;
        if !ok {
            self.decls_failed += 1;
        }
    }
    /// Success rate for declarations.
    pub fn decl_success_rate(&self) -> f64 {
        if self.decls_elaborated == 0 {
            1.0
        } else {
            (self.decls_elaborated - self.decls_failed) as f64 / self.decls_elaborated as f64
        }
    }
    /// Metavar solve rate.
    pub fn metavar_solve_rate(&self) -> f64 {
        if self.metavars_created == 0 {
            1.0
        } else {
            self.metavars_solved as f64 / self.metavars_created as f64
        }
    }
    /// Merge another metrics object into this one.
    pub fn merge(&mut self, other: &ElabRunMetrics) {
        self.decls_elaborated += other.decls_elaborated;
        self.decls_failed += other.decls_failed;
        self.exprs_elaborated += other.exprs_elaborated;
        self.implicits_inserted += other.implicits_inserted;
        self.metavars_created += other.metavars_created;
        self.metavars_solved += other.metavars_solved;
    }
    /// One-line summary.
    pub fn summary(&self) -> String {
        format!(
            "decls={} failed={} exprs={} implicits={} metavars={}/{} ok={:.1}%",
            self.decls_elaborated,
            self.decls_failed,
            self.exprs_elaborated,
            self.implicits_inserted,
            self.metavars_solved,
            self.metavars_created,
            self.decl_success_rate() * 100.0,
        )
    }
}
