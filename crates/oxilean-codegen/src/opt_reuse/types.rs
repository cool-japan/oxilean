//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::opt_licm::LicmHoistCandidate;
use std::collections::{HashMap, HashSet};

use std::collections::VecDeque;

/// Reuse analysis pass summary
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReusePassSummary {
    pub pass_name: String,
    pub functions_analyzed: usize,
    pub reuses_applied: usize,
    pub stack_allocations: usize,
    pub bytes_saved: u64,
    pub duration_us: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ORLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl ORLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        ORLivenessInfo {
            live_in: vec![std::collections::HashSet::new(); block_count],
            live_out: vec![std::collections::HashSet::new(); block_count],
            defs: vec![std::collections::HashSet::new(); block_count],
            uses: vec![std::collections::HashSet::new(); block_count],
        }
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, block: usize, var: u32) {
        if block < self.defs.len() {
            self.defs[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, block: usize, var: u32) {
        if block < self.uses.len() {
            self.uses[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn is_live_in(&self, block: usize, var: u32) -> bool {
        self.live_in
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn is_live_out(&self, block: usize, var: u32) -> bool {
        self.live_out
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
}
/// Reuse analysis interference graph
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseInterferenceGraph {
    pub num_nodes: usize,
    pub edges: std::collections::HashSet<(u32, u32)>,
}
#[allow(dead_code)]
impl ReuseInterferenceGraph {
    pub fn new(n: usize) -> Self {
        Self {
            num_nodes: n,
            edges: std::collections::HashSet::new(),
        }
    }
    pub fn add_edge(&mut self, a: u32, b: u32) {
        let key = if a < b { (a, b) } else { (b, a) };
        self.edges.insert(key);
    }
    pub fn interfere(&self, a: u32, b: u32) -> bool {
        let key = if a < b { (a, b) } else { (b, a) };
        self.edges.contains(&key)
    }
    pub fn degree(&self, v: u32) -> usize {
        self.edges
            .iter()
            .filter(|(a, b)| *a == v || *b == v)
            .count()
    }
}
/// Allocation site descriptor
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AllocSite {
    pub id: u32,
    pub size_class: ReuseMemSizeClass,
    pub func: String,
    pub is_recursive: bool,
}
/// Reuse analysis region (for inter-procedural analysis)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReuseRegion {
    pub region_id: u32,
    pub func_name: String,
    pub allocs: Vec<AllocSite>,
    pub decisions: Vec<(u32, ReuseDecision)>,
}
/// Information about which RC operations can be eliminated
#[derive(Debug, Clone, PartialEq)]
pub struct RcElimInfo {
    /// The variable whose RC operation can be eliminated
    pub var: LcnfVarId,
    /// The kind of RC operation
    pub kind: RcElimKind,
    /// Why this elimination is valid
    pub reason: RcElimReason,
}
#[allow(dead_code)]
pub struct ORConstantFoldingHelper;
impl ORConstantFoldingHelper {
    #[allow(dead_code)]
    pub fn fold_add_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn fold_sub_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn fold_mul_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn fold_div_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_add_f64(a: f64, b: f64) -> f64 {
        a + b
    }
    #[allow(dead_code)]
    pub fn fold_mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }
    #[allow(dead_code)]
    pub fn fold_neg_i64(a: i64) -> Option<i64> {
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn fold_not_bool(a: bool) -> bool {
        !a
    }
    #[allow(dead_code)]
    pub fn fold_and_bool(a: bool, b: bool) -> bool {
        a && b
    }
    #[allow(dead_code)]
    pub fn fold_or_bool(a: bool, b: bool) -> bool {
        a || b
    }
    #[allow(dead_code)]
    pub fn fold_shl_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shl(b)
    }
    #[allow(dead_code)]
    pub fn fold_shr_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shr(b)
    }
    #[allow(dead_code)]
    pub fn fold_rem_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            Some(a % b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_bitand_i64(a: i64, b: i64) -> i64 {
        a & b
    }
    #[allow(dead_code)]
    pub fn fold_bitor_i64(a: i64, b: i64) -> i64 {
        a | b
    }
    #[allow(dead_code)]
    pub fn fold_bitxor_i64(a: i64, b: i64) -> i64 {
        a ^ b
    }
    #[allow(dead_code)]
    pub fn fold_bitnot_i64(a: i64) -> i64 {
        !a
    }
}
/// Reuse analysis allocation log
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseAllocLog {
    pub records: Vec<ReuseAllocRecord>,
}
#[allow(dead_code)]
impl ReuseAllocLog {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, r: ReuseAllocRecord) {
        self.records.push(r);
    }
    pub fn heap_count(&self) -> usize {
        self.records
            .iter()
            .filter(|r| r.kind == ReuseAllocKind::Heap)
            .count()
    }
    pub fn stack_count(&self) -> usize {
        self.records
            .iter()
            .filter(|r| r.kind == ReuseAllocKind::Stack)
            .count()
    }
    pub fn reuse_count(&self) -> usize {
        self.records
            .iter()
            .filter(|r| r.reused_from.is_some())
            .count()
    }
    pub fn total_bytes(&self) -> u64 {
        self.records.iter().map(|r| r.size).sum()
    }
}
/// Reuse analysis diagnostic
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReuseDiagLevel {
    Info,
    Warning,
    Error,
}
/// Ownership status of a variable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ownership {
    /// Unique owner (refcount = 1)
    Unique,
    /// Shared (refcount > 1 or unknown)
    Shared,
    /// Borrowed (temporary reference)
    Borrowed,
    /// Unknown ownership
    Unknown,
}
impl Ownership {
    pub(super) fn merge(self, other: Ownership) -> Ownership {
        match (self, other) {
            (Ownership::Unique, Ownership::Unique) => Ownership::Unique,
            (Ownership::Borrowed, Ownership::Borrowed) => Ownership::Borrowed,
            (Ownership::Shared, _) | (_, Ownership::Shared) => Ownership::Shared,
            (Ownership::Unknown, _) | (_, Ownership::Unknown) => Ownership::Unknown,
            _ => Ownership::Shared,
        }
    }
}
/// Simple lifetime analysis for borrow correctness
///
/// Ensures that borrowed values outlive their borrows.
pub struct LifetimeAnalyzer {
    /// Stack of active lifetimes
    pub(super) active: Vec<LifetimeScope>,
}
impl LifetimeAnalyzer {
    pub(super) fn new() -> Self {
        LifetimeAnalyzer { active: Vec::new() }
    }
    pub(super) fn push_scope(&mut self) {
        self.active.push(LifetimeScope {
            defined: HashSet::new(),
            borrowed: HashSet::new(),
        });
    }
    pub(super) fn pop_scope(&mut self) -> Option<LifetimeScope> {
        self.active.pop()
    }
    pub(super) fn define_var(&mut self, var: LcnfVarId) {
        if let Some(scope) = self.active.last_mut() {
            scope.defined.insert(var);
        }
    }
    pub(super) fn borrow_var(&mut self, var: LcnfVarId) {
        if let Some(scope) = self.active.last_mut() {
            scope.borrowed.insert(var);
        }
    }
    /// Check if a borrow is safe (the borrowed value outlives the borrow)
    pub(super) fn is_borrow_safe(&self, borrowed_var: LcnfVarId) -> bool {
        let mut found_definition = false;
        for scope in &self.active {
            if scope.defined.contains(&borrowed_var) {
                found_definition = true;
            }
        }
        found_definition
    }
    /// Analyze lifetimes in an expression
    pub(super) fn analyze(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let { id, body, .. } => {
                self.define_var(*id);
                self.analyze(body);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.push_scope();
                    for field in &alt.params {
                        self.define_var(field.id);
                    }
                    self.analyze(&alt.body);
                    self.pop_scope();
                }
                if let Some(def) = default {
                    self.push_scope();
                    self.analyze(def);
                    self.pop_scope();
                }
            }
            _ => {}
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ORPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl ORPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ORPassPhase::Analysis => "analysis",
            ORPassPhase::Transformation => "transformation",
            ORPassPhase::Verification => "verification",
            ORPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, ORPassPhase::Transformation | ORPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReuseDiag {
    pub level: ReuseDiagLevel,
    pub message: String,
    pub var: Option<u32>,
}
/// Reuse allocation kind
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReuseAllocKind {
    Heap,
    Stack,
    Scratch,
    Static,
    Inline,
}
/// Free pool tracker (for reuse optimization)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseFreePool {
    pub pool: std::collections::HashMap<ReuseMemSizeClass, Vec<u32>>,
}
#[allow(dead_code)]
impl ReuseFreePool {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, var: u32, class: ReuseMemSizeClass) {
        self.pool.entry(class).or_default().push(var);
    }
    pub fn pop(&mut self, class: &ReuseMemSizeClass) -> Option<u32> {
        self.pool.get_mut(class)?.pop()
    }
    pub fn total_free(&self) -> usize {
        self.pool.values().map(|v| v.len()).sum()
    }
}
/// Reuse analysis statistics (extended)
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ReuseStatsExt {
    pub allocs_analyzed: usize,
    pub reuses_applied: usize,
    pub stack_allocations: usize,
    pub rc_bumps: usize,
    pub inlines: usize,
    pub scratch_uses: usize,
    pub bytes_saved: u64,
    pub allocs_eliminated: usize,
}
/// A potential reset-reuse opportunity
#[derive(Debug, Clone, PartialEq)]
pub struct ReuseOpportunity {
    /// The variable being deallocated (potential reset source)
    pub dealloc_var: LcnfVarId,
    /// The allocation site (potential reuse target)
    pub alloc_var: LcnfVarId,
    /// Constructor name for the deallocation
    pub dealloc_ctor: String,
    /// Constructor tag for the deallocation
    pub dealloc_tag: u32,
    /// Constructor name for the allocation
    pub alloc_ctor: String,
    /// Constructor tag for the allocation
    pub alloc_tag: u32,
    /// Whether the layouts are compatible
    pub layout_compatible: bool,
    /// Estimated savings from this reuse (in allocation cost units)
    pub estimated_savings: usize,
}
/// Main reuse analysis and optimization struct
pub struct ReuseAnalyzer {
    pub(super) config: ReuseConfig,
    /// Borrow information for each variable
    pub(super) borrow_info: HashMap<LcnfVarId, BorrowInfo>,
    /// Detected reuse opportunities
    pub(super) reuse_opportunities: Vec<ReuseOpportunity>,
    /// RC operations that can be eliminated
    pub(super) rc_eliminations: Vec<RcElimInfo>,
    /// Statistics
    pub(super) stats: ReuseStats,
    /// Layout information cache
    pub(super) layout_cache: HashMap<String, LayoutInfo>,
    /// Use count for each variable
    pub(super) use_counts: HashMap<LcnfVarId, usize>,
}
impl ReuseAnalyzer {
    /// Create a new reuse analyzer
    pub fn new(config: ReuseConfig) -> Self {
        ReuseAnalyzer {
            config,
            borrow_info: HashMap::new(),
            reuse_opportunities: Vec::new(),
            rc_eliminations: Vec::new(),
            stats: ReuseStats::default(),
            layout_cache: HashMap::new(),
            use_counts: HashMap::new(),
        }
    }
    /// Get the optimization statistics
    pub fn stats(&self) -> &ReuseStats {
        &self.stats
    }
    /// Get detected reuse opportunities
    pub fn reuse_opportunities(&self) -> &[ReuseOpportunity] {
        &self.reuse_opportunities
    }
    /// Get RC eliminations
    pub fn rc_eliminations(&self) -> &[RcElimInfo] {
        &self.rc_eliminations
    }
    /// Get borrow info for a variable
    pub fn get_borrow_info(&self, var: &LcnfVarId) -> Option<&BorrowInfo> {
        self.borrow_info.get(var)
    }
    /// Analyze a single declaration
    pub(super) fn analyze_decl(&mut self, decl: &LcnfFunDecl) {
        self.use_counts.clear();
        self.compute_use_counts(&decl.body);
        for param in &decl.params {
            self.stats.vars_analyzed += 1;
            let ownership = if self.use_counts.get(&param.id).copied().unwrap_or(0) <= 1 {
                Ownership::Unique
            } else {
                Ownership::Shared
            };
            let info = BorrowInfo::with_ownership(param.id, ownership);
            self.borrow_info.insert(param.id, info);
        }
        self.analyze_ownership(&decl.body, 0);
        if self.config.enable_borrow {
            self.infer_borrows(decl);
        }
        if self.config.enable_reset_reuse {
            self.find_reuse_opportunities(&decl.body);
        }
        if self.config.enable_rc_elim {
            self.find_rc_eliminations(&decl.body);
        }
    }
    /// Compute use counts for all variables in an expression
    pub(super) fn compute_use_counts(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                self.count_value_uses(value);
                self.compute_use_counts(body);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                *self.use_counts.entry(*scrutinee).or_insert(0) += 1;
                for alt in alts {
                    self.compute_use_counts(&alt.body);
                }
                if let Some(def) = default {
                    self.compute_use_counts(def);
                }
            }
            LcnfExpr::Return(arg) => {
                if let LcnfArg::Var(v) = arg {
                    *self.use_counts.entry(*v).or_insert(0) += 1;
                }
            }
            LcnfExpr::TailCall(func, args) => {
                if let LcnfArg::Var(v) = func {
                    *self.use_counts.entry(*v).or_insert(0) += 1;
                }
                for a in args {
                    if let LcnfArg::Var(v) = a {
                        *self.use_counts.entry(*v).or_insert(0) += 1;
                    }
                }
            }
            LcnfExpr::Unreachable => {}
        }
    }
    /// Count uses in a let-value
    pub(super) fn count_value_uses(&mut self, value: &LcnfLetValue) {
        match value {
            LcnfLetValue::App(func, args) => {
                if let LcnfArg::Var(v) = func {
                    *self.use_counts.entry(*v).or_insert(0) += 1;
                }
                for a in args {
                    if let LcnfArg::Var(v) = a {
                        *self.use_counts.entry(*v).or_insert(0) += 1;
                    }
                }
            }
            LcnfLetValue::Proj(_, _, v) => {
                *self.use_counts.entry(*v).or_insert(0) += 1;
            }
            LcnfLetValue::Ctor(_, _, args) => {
                for a in args {
                    if let LcnfArg::Var(v) = a {
                        *self.use_counts.entry(*v).or_insert(0) += 1;
                    }
                }
            }
            LcnfLetValue::FVar(v) => {
                *self.use_counts.entry(*v).or_insert(0) += 1;
            }
            LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
            LcnfLetValue::Reset(v) => {
                *self.use_counts.entry(*v).or_insert(0) += 1;
            }
            LcnfLetValue::Reuse(slot, _, _, args) => {
                *self.use_counts.entry(*slot).or_insert(0) += 1;
                for a in args {
                    if let LcnfArg::Var(v) = a {
                        *self.use_counts.entry(*v).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    /// Analyze ownership status through an expression
    pub(super) fn analyze_ownership(&mut self, expr: &LcnfExpr, depth: usize) {
        if depth > self.config.analysis_depth {
            return;
        }
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                self.stats.vars_analyzed += 1;
                let ownership = self.infer_value_ownership(value);
                let mut info = BorrowInfo::with_ownership(*id, ownership);
                match value {
                    LcnfLetValue::FVar(src) | LcnfLetValue::Proj(_, _, src) => {
                        info.derived_from.push(*src);
                    }
                    _ => {}
                }
                info.escapes = self.does_escape(*id, body);
                if ownership == Ownership::Unique {
                    self.stats.unique_ownership += 1;
                }
                self.borrow_info.insert(*id, info);
                self.analyze_ownership(body, depth + 1);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    for field in &alt.params {
                        self.stats.vars_analyzed += 1;
                        let info = BorrowInfo::new(field.id);
                        self.borrow_info.insert(field.id, info);
                    }
                    self.analyze_ownership(&alt.body, depth + 1);
                }
                if let Some(def) = default {
                    self.analyze_ownership(def, depth + 1);
                }
            }
            _ => {}
        }
    }
    /// Infer ownership status from a let-value
    pub(super) fn infer_value_ownership(&self, value: &LcnfLetValue) -> Ownership {
        match value {
            LcnfLetValue::Ctor(_, _, _) => Ownership::Unique,
            LcnfLetValue::Lit(_) => Ownership::Unique,
            LcnfLetValue::Erased => Ownership::Unique,
            LcnfLetValue::FVar(src) => self
                .borrow_info
                .get(src)
                .map(|info| info.ownership)
                .unwrap_or(Ownership::Unknown),
            LcnfLetValue::Proj(_, _, src) => {
                let parent_ownership = self
                    .borrow_info
                    .get(src)
                    .map(|info| info.ownership)
                    .unwrap_or(Ownership::Unknown);
                match parent_ownership {
                    Ownership::Unique => Ownership::Borrowed,
                    _ => Ownership::Shared,
                }
            }
            LcnfLetValue::App(_, _) => Ownership::Unknown,
            LcnfLetValue::Reset(_) => Ownership::Unique,
            LcnfLetValue::Reuse(_, _, _, _) => Ownership::Unique,
        }
    }
    /// Check if a variable escapes the current scope
    pub(super) fn does_escape(&self, var: LcnfVarId, expr: &LcnfExpr) -> bool {
        match expr {
            LcnfExpr::Return(LcnfArg::Var(v)) => *v == var,
            LcnfExpr::TailCall(_, args) => args
                .iter()
                .any(|a| matches!(a, LcnfArg::Var(v) if * v == var)),
            LcnfExpr::Let { value, body, .. } => {
                let in_value = match value {
                    LcnfLetValue::Ctor(_, _, args) => args
                        .iter()
                        .any(|a| matches!(a, LcnfArg::Var(v) if * v == var)),
                    LcnfLetValue::App(_, args) => args
                        .iter()
                        .any(|a| matches!(a, LcnfArg::Var(v) if * v == var)),
                    _ => false,
                };
                in_value || self.does_escape(var, body)
            }
            LcnfExpr::Case { alts, default, .. } => {
                alts.iter().any(|a| self.does_escape(var, &a.body))
                    || default.as_ref().is_some_and(|d| self.does_escape(var, d))
            }
            _ => false,
        }
    }
    /// Infer borrow annotations for function parameters
    pub(super) fn infer_borrows(&mut self, decl: &LcnfFunDecl) {
        for param in &decl.params {
            let use_count = self.use_counts.get(&param.id).copied().unwrap_or(0);
            let escapes = self.does_escape(param.id, &decl.body);
            if !escapes && use_count <= 2 {
                if let Some(info) = self.borrow_info.get_mut(&param.id) {
                    info.can_borrow = true;
                    info.ownership = Ownership::Borrowed;
                    self.stats.borrows_inferred += 1;
                }
            }
        }
    }
    /// Find reset-reuse opportunities
    pub(super) fn find_reuse_opportunities(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Case {
                scrutinee, alts, ..
            } => {
                let scrutinee_unique = self
                    .borrow_info
                    .get(scrutinee)
                    .is_some_and(|info| info.ownership == Ownership::Unique);
                if scrutinee_unique {
                    for alt in alts {
                        self.find_ctor_in_body(
                            &alt.body,
                            *scrutinee,
                            &alt.ctor_name,
                            alt.ctor_tag,
                            alt.params.len(),
                        );
                    }
                }
                for alt in alts {
                    self.find_reuse_opportunities(&alt.body);
                }
            }
            LcnfExpr::Let { body, .. } => {
                self.find_reuse_opportunities(body);
            }
            _ => {}
        }
    }
    /// Find constructor allocations in a case body that could reuse the scrutinee
    pub(super) fn find_ctor_in_body(
        &mut self,
        expr: &LcnfExpr,
        scrutinee: LcnfVarId,
        dealloc_ctor: &str,
        dealloc_tag: u32,
        dealloc_fields: usize,
    ) {
        match expr {
            LcnfExpr::Let {
                id,
                value: LcnfLetValue::Ctor(alloc_ctor, alloc_tag, args),
                body,
                ..
            } => {
                let dealloc_layout = LayoutInfo::new(dealloc_ctor, dealloc_tag, dealloc_fields, 0);
                let alloc_layout = LayoutInfo::new(alloc_ctor, *alloc_tag, args.len(), 0);
                if alloc_layout.fits_in(&dealloc_layout) {
                    self.reuse_opportunities.push(ReuseOpportunity {
                        dealloc_var: scrutinee,
                        alloc_var: *id,
                        dealloc_ctor: dealloc_ctor.to_string(),
                        dealloc_tag,
                        alloc_ctor: alloc_ctor.clone(),
                        alloc_tag: *alloc_tag,
                        layout_compatible: dealloc_layout.is_compatible_with(&alloc_layout),
                        estimated_savings: alloc_layout.total_words * 8,
                    });
                    self.stats.reuse_pairs += 1;
                }
                self.find_ctor_in_body(body, scrutinee, dealloc_ctor, dealloc_tag, dealloc_fields);
            }
            LcnfExpr::Let { body, .. } => {
                self.find_ctor_in_body(body, scrutinee, dealloc_ctor, dealloc_tag, dealloc_fields);
            }
            _ => {}
        }
    }
    /// Find RC operations that can be eliminated
    pub(super) fn find_rc_eliminations(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                if let Some(info) = self.borrow_info.get(id) {
                    if info.ownership == Ownership::Unique {
                        let use_count = self.use_counts.get(id).copied().unwrap_or(0);
                        if use_count <= 1 {
                            self.rc_eliminations.push(RcElimInfo {
                                var: *id,
                                kind: RcElimKind::SkipDec,
                                reason: RcElimReason::UniqueOwnership,
                            });
                            self.stats.rc_ops_eliminated += 1;
                        }
                    } else if info.can_borrow {
                        self.rc_eliminations.push(RcElimInfo {
                            var: *id,
                            kind: RcElimKind::SkipInc,
                            reason: RcElimReason::Borrowed,
                        });
                        self.stats.rc_ops_eliminated += 1;
                    }
                }
                self.find_cancel_pairs(value, *id);
                self.find_rc_eliminations(body);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.find_rc_eliminations(&alt.body);
                }
                if let Some(def) = default {
                    self.find_rc_eliminations(def);
                }
            }
            _ => {}
        }
    }
    /// Find RC inc/dec pairs that cancel each other
    pub(super) fn find_cancel_pairs(&mut self, value: &LcnfLetValue, id: LcnfVarId) {
        if let LcnfLetValue::App(_, args) = value {
            for arg in args {
                if let LcnfArg::Var(x) = arg {
                    let use_count = self.use_counts.get(x).copied().unwrap_or(0);
                    if use_count <= 1 {
                        if !self
                            .rc_eliminations
                            .iter()
                            .any(|e| e.var == *x && e.kind == RcElimKind::CancelPair)
                        {
                            self.rc_eliminations.push(RcElimInfo {
                                var: *x,
                                kind: RcElimKind::CancelPair,
                                reason: RcElimReason::CancelledPair,
                            });
                            self.stats.rc_ops_eliminated += 1;
                        }
                    }
                }
            }
        }
        let _ = id;
    }
    /// Apply reset-reuse transformations to an expression
    pub(super) fn apply_reuse(&self, expr: &mut LcnfExpr) {
        let reuse_map: HashMap<LcnfVarId, &ReuseOpportunity> = self
            .reuse_opportunities
            .iter()
            .map(|opp| (opp.alloc_var, opp))
            .collect();
        self.apply_reuse_inner(expr, &reuse_map);
    }
    /// Recursively apply reuse transformations.
    ///
    /// When `id` is in `reuse_map` and the bound value is a `Ctor`, we replace:
    /// ```text
    ///   let id = Ctor(args...)
    /// ```
    /// with:
    /// ```text
    ///   let slot = reset(dealloc_var)
    ///   let id   = reuse(slot, Ctor, args...)
    /// ```
    pub(super) fn apply_reuse_inner(
        &self,
        expr: &mut LcnfExpr,
        reuse_map: &HashMap<LcnfVarId, &ReuseOpportunity>,
    ) {
        let can_reuse = if let LcnfExpr::Let { id, value, .. } = &*expr {
            matches!(value, LcnfLetValue::Ctor(..))
                && reuse_map.get(id).is_some_and(|o| o.layout_compatible)
        } else {
            false
        };
        let (dealloc_var, cur_id) = if can_reuse {
            if let LcnfExpr::Let { id, .. } = &*expr {
                let opp = reuse_map[id];
                (opp.dealloc_var, *id)
            } else {
                unreachable!()
            }
        } else {
            (LcnfVarId(0), LcnfVarId(0))
        };
        if can_reuse {
            let slot_id = LcnfVarId(cur_id.0 + 1_000_000);
            let old_expr = std::mem::replace(expr, LcnfExpr::Unreachable);
            if let LcnfExpr::Let {
                id: old_id,
                name,
                ty,
                value: LcnfLetValue::Ctor(ctor_name, ctor_tag, args),
                body,
            } = old_expr
            {
                let reuse_let = LcnfExpr::Let {
                    id: old_id,
                    name: name.clone(),
                    ty,
                    value: LcnfLetValue::Reuse(slot_id, ctor_name, ctor_tag, args),
                    body,
                };
                let reset_let = LcnfExpr::Let {
                    id: slot_id,
                    name: format!("{}_slot", name),
                    ty: LcnfType::Object,
                    value: LcnfLetValue::Reset(dealloc_var),
                    body: Box::new(reuse_let),
                };
                *expr = reset_let;
                if let LcnfExpr::Let {
                    body: slot_body, ..
                } = expr
                {
                    if let LcnfExpr::Let {
                        body: reuse_body, ..
                    } = slot_body.as_mut()
                    {
                        self.apply_reuse_inner(reuse_body, reuse_map);
                    }
                }
                return;
            }
        }
        match expr {
            LcnfExpr::Let { body, .. } => {
                self.apply_reuse_inner(body, reuse_map);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.apply_reuse_inner(&mut alt.body, reuse_map);
                }
                if let Some(def) = default {
                    self.apply_reuse_inner(def, reuse_map);
                }
            }
            _ => {}
        }
    }
    /// Apply borrow annotations to a declaration
    pub(super) fn apply_borrows(&self, decl: &mut LcnfFunDecl) {
        for param in &mut decl.params {
            if let Some(info) = self.borrow_info.get(&param.id) {
                if info.can_borrow && !param.erased {
                    param.borrowed = true;
                }
            }
        }
    }
}
/// Statistics for reuse optimization
#[derive(Debug, Clone, Default)]
pub struct ReuseStats {
    /// Number of reset-reuse pairs found
    pub reuse_pairs: usize,
    /// Number of borrows inferred
    pub borrows_inferred: usize,
    /// Number of RC operations eliminated
    pub rc_ops_eliminated: usize,
    /// Number of in-place updates enabled
    pub in_place_updates: usize,
    /// Number of unique ownership inferences
    pub unique_ownership: usize,
    /// Number of variables analyzed
    pub vars_analyzed: usize,
}
/// Reuse analysis interprocedural summary
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseIPASummary {
    pub regions: Vec<ReuseRegion>,
    pub global_reuse_count: usize,
    pub global_stack_count: usize,
    pub total_bytes_saved: u64,
}
#[allow(dead_code)]
impl ReuseIPASummary {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_region(&mut self, r: ReuseRegion) {
        self.global_reuse_count += r
            .decisions
            .iter()
            .filter(|(_, d)| matches!(d, ReuseDecision::Reuse(_)))
            .count();
        self.global_stack_count += r
            .decisions
            .iter()
            .filter(|(_, d)| *d == ReuseDecision::StackAlloc)
            .count();
        self.regions.push(r);
    }
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ORDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl ORDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ORDepGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_dep(&mut self, dep: u32, dependent: u32) {
        self.add_node(dep);
        self.add_node(dependent);
        self.edges.push((dep, dependent));
    }
    #[allow(dead_code)]
    pub fn dependents_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(d, _)| *d == node)
            .map(|(_, dep)| *dep)
            .collect()
    }
    #[allow(dead_code)]
    pub fn dependencies_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(_, dep)| *dep == node)
            .map(|(d, _)| *d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Vec<u32> {
        let mut in_degree: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for &n in &self.nodes {
            in_degree.insert(n, 0);
        }
        for (_, dep) in &self.edges {
            *in_degree.entry(*dep).or_insert(0) += 1;
        }
        let mut queue: std::collections::VecDeque<u32> = self
            .nodes
            .iter()
            .filter(|&&n| in_degree[&n] == 0)
            .copied()
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            for dep in self.dependents_of(node) {
                let cnt = in_degree.entry(dep).or_insert(0);
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    queue.push_back(dep);
                }
            }
        }
        result
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topological_sort().len() < self.nodes.len()
    }
}
/// Borrow analysis results for a variable
#[derive(Debug, Clone, PartialEq)]
pub struct BorrowInfo {
    /// The variable being analyzed
    pub var: LcnfVarId,
    /// Whether this variable can be borrowed (vs owned)
    pub can_borrow: bool,
    /// The ownership status
    pub ownership: Ownership,
    /// Where this variable is last used
    pub last_use: Option<usize>,
    /// Whether this variable escapes the current scope
    pub escapes: bool,
    /// Which parameters this variable is derived from
    pub derived_from: Vec<LcnfVarId>,
}
impl BorrowInfo {
    pub(super) fn new(var: LcnfVarId) -> Self {
        BorrowInfo {
            var,
            can_borrow: false,
            ownership: Ownership::Unknown,
            last_use: None,
            escapes: false,
            derived_from: Vec::new(),
        }
    }
    pub(super) fn with_ownership(var: LcnfVarId, ownership: Ownership) -> Self {
        BorrowInfo {
            var,
            can_borrow: matches!(ownership, Ownership::Borrowed),
            ownership,
            last_use: None,
            escapes: false,
            derived_from: Vec::new(),
        }
    }
}
/// Reuse analysis emit stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ReuseExtEmitStats {
    pub bytes_written: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
}
/// Allocation size class
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReuseMemSizeClass {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Dynamic,
}
/// Reuse analysis global config
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ReuseGlobalConfig {
    pub max_scratch_size: u64,
    pub enable_ipa: bool,
    pub enable_coloring: bool,
    pub enable_linear_scan: bool,
    pub max_regions: usize,
}
#[allow(dead_code)]
pub struct ReuseAllocSiteInfo {
    pub site: AllocSite,
    pub live_range: ReuseLiveRange,
    pub decision: ReuseDecision,
}
/// Reuse analysis feature flags
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ReuseFeatureFlags {
    pub reuse_constructor_allocated: bool,
    pub reuse_rc_singleton: bool,
    pub reuse_trivial: bool,
    pub inline_small_closures: bool,
}
/// An in-place update opportunity
#[derive(Debug, Clone)]
pub struct InPlaceUpdate {
    /// The source variable (being updated)
    pub(super) source: LcnfVarId,
    /// The result variable (the updated value)
    pub(super) result: LcnfVarId,
    /// Which fields are changed (index -> new value)
    pub(super) changed_fields: Vec<(usize, LcnfArg)>,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseDiagSink {
    pub diags: Vec<ReuseDiag>,
}
#[allow(dead_code)]
impl ReuseDiagSink {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, level: ReuseDiagLevel, msg: &str, var: Option<u32>) {
        self.diags.push(ReuseDiag {
            level,
            message: msg.to_string(),
            var,
        });
    }
    pub fn has_errors(&self) -> bool {
        self.diags.iter().any(|d| d.level == ReuseDiagLevel::Error)
    }
}
/// Configuration for reuse optimization
#[derive(Debug, Clone)]
pub struct ReuseConfig {
    /// Enable reset-reuse optimization
    pub enable_reset_reuse: bool,
    /// Enable borrow inference
    pub enable_borrow: bool,
    /// Enable reference counting elimination
    pub enable_rc_elim: bool,
    /// Enable in-place update optimization
    pub enable_in_place: bool,
    /// Maximum depth for ownership analysis
    pub analysis_depth: usize,
    /// Whether to track ownership through function calls
    pub interprocedural: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ORPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl ORPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_run(&mut self, changes: u64, time_ms: u64, iterations: u32) {
        self.total_runs += 1;
        self.successful_runs += 1;
        self.total_changes += changes;
        self.time_ms += time_ms;
        self.iterations_used = iterations;
    }
    #[allow(dead_code)]
    pub fn average_changes_per_run(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.total_changes as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.successful_runs as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Runs: {}/{}, Changes: {}, Time: {}ms",
            self.successful_runs, self.total_runs, self.total_changes, self.time_ms
        )
    }
}
/// Reuse analysis pass config (extended)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReuseConfigExt {
    pub enable_reuse: bool,
    pub enable_stack_alloc: bool,
    pub enable_inline: bool,
    pub enable_scratch_buffer: bool,
    pub max_reuse_distance: usize,
    pub max_live_ranges: usize,
    pub scratch_buffer_size: u64,
}
/// Kind of RC elimination
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RcElimKind {
    /// Remove an increment (the value is uniquely owned)
    SkipInc,
    /// Remove a decrement (the value is still live)
    SkipDec,
    /// Combine inc+dec into nothing
    CancelPair,
}
/// Reuse analysis source buffer
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseExtSourceBuffer {
    pub content: String,
}
#[allow(dead_code)]
impl ReuseExtSourceBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write(&mut self, s: &str) {
        self.content.push_str(s);
    }
    pub fn writeln(&mut self, s: &str) {
        self.content.push_str(s);
        self.content.push('\n');
    }
    pub fn finish(self) -> String {
        self.content
    }
}
/// Reuse analysis builder
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReusePassBuilder {
    pub config: ReuseConfigExt,
    pub decisions: ReuseDecisionMap,
    pub stats: ReuseStatsExt,
    pub diags: ReuseDiagSink,
}
#[allow(dead_code)]
impl ReusePassBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(mut self, cfg: ReuseConfigExt) -> Self {
        self.config = cfg;
        self
    }
    pub fn report(&self) -> String {
        format!("{}", self.stats)
    }
}
/// Reuse analysis linear scan allocator (for scratch buffer assignment)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseLinearScanAlloc {
    pub slots: Vec<(u64, bool)>,
    pub assignment: std::collections::HashMap<u32, usize>,
}
#[allow(dead_code)]
impl ReuseLinearScanAlloc {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn alloc(&mut self, var: u32, size: u64) -> usize {
        for (i, (slot_sz, in_use)) in self.slots.iter_mut().enumerate() {
            if !*in_use && *slot_sz >= size {
                *in_use = true;
                self.assignment.insert(var, i);
                return i;
            }
        }
        let i = self.slots.len();
        self.slots.push((size, true));
        self.assignment.insert(var, i);
        i
    }
    pub fn free(&mut self, var: u32) {
        if let Some(i) = self.assignment.remove(&var) {
            if let Some((_, in_use)) = self.slots.get_mut(i) {
                *in_use = false;
            }
        }
    }
    pub fn slots_used(&self) -> usize {
        self.slots.iter().filter(|(_, u)| *u).count()
    }
    pub fn total_slots(&self) -> usize {
        self.slots.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ORCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Reuse allocation record
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReuseAllocRecord {
    pub var: u32,
    pub kind: ReuseAllocKind,
    pub size: u64,
    pub reused_from: Option<u32>,
}
/// Reuse analysis profiler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseExtProfiler {
    pub timings: Vec<(String, u64)>,
}
#[allow(dead_code)]
impl ReuseExtProfiler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, pass: &str, us: u64) {
        self.timings.push((pass.to_string(), us));
    }
    pub fn total_us(&self) -> u64 {
        self.timings.iter().map(|(_, t)| *t).sum()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ORWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl ORWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ORWorklist {
            items: std::collections::VecDeque::new(),
            in_worklist: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: u32) -> bool {
        if self.in_worklist.insert(item) {
            self.items.push_back(item);
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        let item = self.items.pop_front()?;
        self.in_worklist.remove(&item);
        Some(item)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, item: u32) -> bool {
        self.in_worklist.contains(&item)
    }
}
/// Reuse analysis coloring (for register-like reuse assignment)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseColoring {
    pub color: std::collections::HashMap<u32, u32>,
    pub num_colors: u32,
}
#[allow(dead_code)]
impl ReuseColoring {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn assign(&mut self, var: u32, color: u32) {
        self.color.insert(var, color);
        self.num_colors = self.num_colors.max(color + 1);
    }
    pub fn get(&self, var: u32) -> Option<u32> {
        self.color.get(&var).copied()
    }
    pub fn colors_used(&self) -> u32 {
        self.num_colors
    }
    pub fn vars_with_color(&self, c: u32) -> Vec<u32> {
        self.color
            .iter()
            .filter(|(_, &col)| col == c)
            .map(|(&v, _)| v)
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ORPassConfig {
    pub phase: ORPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl ORPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: ORPassPhase) -> Self {
        ORPassConfig {
            phase,
            enabled: true,
            max_iterations: 10,
            debug_output: false,
            pass_name: name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[allow(dead_code)]
    pub fn max_iter(mut self, n: u32) -> Self {
        self.max_iterations = n;
        self
    }
}
/// Live range for reuse analysis
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReuseLiveRange {
    pub var: u32,
    pub def_point: usize,
    pub last_use: usize,
    pub size_class: ReuseMemSizeClass,
}
impl ReuseLiveRange {
    #[allow(dead_code)]
    pub fn overlaps(&self, other: &ReuseLiveRange) -> bool {
        !(self.last_use < other.def_point || other.last_use < self.def_point)
    }
    #[allow(dead_code)]
    pub fn can_reuse(&self, other: &ReuseLiveRange) -> bool {
        !self.overlaps(other) && self.size_class == other.size_class
    }
}
/// Reuse analysis code stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ReuseCodeStats {
    pub allocs_analyzed: usize,
    pub reuses: usize,
    pub stack_allocs: usize,
    pub inlines: usize,
    pub scratch_uses: usize,
    pub bytes_saved: u64,
}
/// Reason why an RC elimination is valid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RcElimReason {
    /// Variable has unique ownership
    UniqueOwnership,
    /// Variable is borrowed (no ownership transfer)
    Borrowed,
    /// Inc and dec on same variable cancel out
    CancelledPair,
    /// Value is immediately consumed
    ImmediateConsume,
    /// Value is never used after this point
    DeadAfterPoint,
}
/// Memory layout information for a constructor
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutInfo {
    /// Constructor name
    pub(super) ctor_name: String,
    /// Constructor tag
    pub(super) ctor_tag: u32,
    /// Number of object (pointer) fields
    pub(super) obj_fields: usize,
    /// Number of scalar fields
    pub(super) scalar_fields: usize,
    /// Total size in words (8 bytes each)
    pub(super) total_words: usize,
}
impl LayoutInfo {
    pub(super) fn new(
        ctor_name: &str,
        ctor_tag: u32,
        obj_fields: usize,
        scalar_fields: usize,
    ) -> Self {
        LayoutInfo {
            ctor_name: ctor_name.to_string(),
            ctor_tag,
            obj_fields,
            scalar_fields,
            total_words: 1 + obj_fields + scalar_fields,
        }
    }
    /// Check if another layout is compatible for reuse
    pub(super) fn is_compatible_with(&self, other: &LayoutInfo) -> bool {
        self.total_words == other.total_words
    }
    /// Check if another layout is a subset (smaller or equal)
    pub(super) fn fits_in(&self, other: &LayoutInfo) -> bool {
        self.total_words <= other.total_words
    }
}
/// A lifetime scope
#[derive(Debug, Clone)]
pub(crate) struct LifetimeScope {
    /// Variables defined in this scope
    pub(super) defined: HashSet<LcnfVarId>,
    /// Variables borrowed in this scope
    pub(super) borrowed: HashSet<LcnfVarId>,
}
/// Reuse analysis builder v2
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReusePassBuilderV2 {
    pub config: ReuseGlobalConfig,
    pub ipa_summary: ReuseIPASummary,
    pub coloring: ReuseColoring,
    pub liveness: ReuseLiveness,
    pub stats: ReuseCodeStats,
}
#[allow(dead_code)]
impl ReusePassBuilderV2 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(mut self, cfg: ReuseGlobalConfig) -> Self {
        self.config = cfg;
        self
    }
    pub fn report(&self) -> String {
        format!("{}", self.stats)
    }
    pub fn ipa_report(&self) -> String {
        format!("{}", self.ipa_summary)
    }
}
#[allow(dead_code)]
pub struct ORPassRegistry {
    pub(super) configs: Vec<ORPassConfig>,
    pub(super) stats: std::collections::HashMap<String, ORPassStats>,
}
impl ORPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ORPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: ORPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), ORPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ORPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&ORPassStats> {
        self.stats.get(name)
    }
    #[allow(dead_code)]
    pub fn total_passes(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn enabled_count(&self) -> usize {
        self.enabled_passes().len()
    }
    #[allow(dead_code)]
    pub fn update_stats(&mut self, name: &str, changes: u64, time_ms: u64, iter: u32) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, time_ms, iter);
        }
    }
}
/// Reuse analysis decision map
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseDecisionMap {
    pub decisions: std::collections::HashMap<u32, ReuseDecision>,
    pub reuse_count: usize,
    pub stack_alloc_count: usize,
}
#[allow(dead_code)]
impl ReuseDecisionMap {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set(&mut self, var: u32, decision: ReuseDecision) {
        match &decision {
            ReuseDecision::Reuse(_) => self.reuse_count += 1,
            ReuseDecision::StackAlloc => self.stack_alloc_count += 1,
            _ => {}
        }
        self.decisions.insert(var, decision);
    }
    pub fn get(&self, var: u32) -> Option<&ReuseDecision> {
        self.decisions.get(&var)
    }
    pub fn total_decisions(&self) -> usize {
        self.decisions.len()
    }
    pub fn reuse_rate(&self) -> f64 {
        if self.decisions.is_empty() {
            0.0
        } else {
            self.reuse_count as f64 / self.decisions.len() as f64
        }
    }
}
/// Reuse analysis flow-sensitive liveness
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseLiveness {
    pub live_in: std::collections::HashMap<u32, std::collections::HashSet<u32>>,
    pub live_out: std::collections::HashMap<u32, std::collections::HashSet<u32>>,
}
#[allow(dead_code)]
impl ReuseLiveness {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_live_at(&self, block: u32, var: u32) -> bool {
        self.live_in
            .get(&block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    pub fn add_live_in(&mut self, block: u32, var: u32) {
        self.live_in.entry(block).or_default().insert(var);
    }
    pub fn add_live_out(&mut self, block: u32, var: u32) {
        self.live_out.entry(block).or_default().insert(var);
    }
}
/// Reuse analysis id generator
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseExtIdGen {
    pub(super) counter: u32,
}
#[allow(dead_code)]
impl ReuseExtIdGen {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn next(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        id
    }
}
/// Reuse candidate heap (priority queue by benefit)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ReuseCandidateHeap {
    pub items: Vec<(i64, LicmHoistCandidate)>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ORAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, ORCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl ORAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        ORAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&ORCacheEntry> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        if self.entries.len() >= self.max_size {
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.clone(),
            ORCacheEntry {
                key,
                data,
                timestamp: 0,
                valid: true,
            },
        );
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.valid = false;
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ORDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl ORDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        ORDominatorTree {
            idom: vec![None; size],
            dom_children: vec![Vec::new(); size],
            dom_depth: vec![0; size],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, idom: u32) {
        self.idom[node] = Some(idom);
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let mut cur = b;
        loop {
            match self.idom[cur] {
                Some(parent) if parent as usize == a => return true,
                Some(parent) if parent as usize == cur => return false,
                Some(parent) => cur = parent as usize,
                None => return false,
            }
        }
    }
    #[allow(dead_code)]
    pub fn depth(&self, node: usize) -> u32 {
        self.dom_depth.get(node).copied().unwrap_or(0)
    }
}
/// Reuse decision
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReuseDecision {
    Reuse(u32),
    NewAlloc,
    RcBump(u32),
    StackAlloc,
    Inline,
    ScratchBuffer(u32),
}
