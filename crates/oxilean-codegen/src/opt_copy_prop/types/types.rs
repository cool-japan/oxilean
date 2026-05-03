//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;
use crate::lcnf::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use super::functions::FnMap;
use super::types_2::{
    AliasingCopyFilter, ConditionalCopyProp, ConstantFolder, CopyChainCollapser, CopyPropReport,
    CopyPropStats, InlineConfig, InterferenceGraph, MoveSemanticsCopyProp, PipelineResult,
    SubstMap, UsedVars, ValueNumberingCopyProp,
};

/// Identifies an optimization pass in the pipeline.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassKind {
    CopyProp,
    DeadBinding,
    ConstantFold,
    Inlining,
}
/// Report from the constant folding pass.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ConstantFoldReport {
    pub folds_performed: usize,
}
/// Orchestrates the complete copy propagation pipeline.
///
/// Runs the following passes in sequence:
/// 1. Forward copy propagation (existing `CopyProp`)
/// 2. Value numbering copy propagation (`ValueNumberingCopyProp`)
/// 3. Dead copy elimination (`DeadCopyEliminator`)
/// 4. Conditional copy propagation (`ConditionalCopyProp`)
/// 5. Move semantics copy propagation (`MoveSemanticsCopyProp`)
/// 6. Copy chain collapsing (`CopyChainCollapser`)
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CopyPropPipeline {
    /// Configuration for the forward copy propagation pass.
    pub config: CopyPropConfig,
    /// Collected reports across all analyzed functions.
    pub reports: Vec<CopyPropPipelineReport>,
    /// Global interference graph (accumulated across all functions).
    pub interference_graph: InterferenceGraph,
    /// Global alias filter.
    pub alias_filter: AliasingCopyFilter,
    /// Global statistics across all functions.
    pub global_stats: CopyPropStats,
}
#[allow(dead_code)]
impl CopyPropPipeline {
    /// Creates a new copy propagation pipeline with the given configuration.
    pub fn new(config: CopyPropConfig) -> Self {
        CopyPropPipeline {
            config,
            reports: Vec::new(),
            interference_graph: InterferenceGraph::new(),
            alias_filter: AliasingCopyFilter::new(),
            global_stats: CopyPropStats::new(),
        }
    }
    /// Creates a pipeline with default configuration.
    pub fn default_pipeline() -> Self {
        CopyPropPipeline::new(CopyPropConfig::default())
    }
    /// Runs the full pipeline on a function declaration.
    ///
    /// Returns a report summarizing the optimizations applied.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) -> CopyPropPipelineReport {
        let mut report = CopyPropPipelineReport::new(decl.name.clone());
        let mut fwd = CopyProp::new(self.config.clone());
        fwd.run(decl);
        let fwd_report = fwd.report();
        report.stats.copies_eliminated += fwd_report.copies_eliminated;
        let mut vn = ValueNumberingCopyProp::new();
        vn.run(decl);
        report.stats.vn_eliminated += vn.eliminated;
        let mut dce = DeadCopyEliminator::new();
        dce.run(decl);
        report.stats.dead_bindings_removed += dce.removed;
        let mut cond = ConditionalCopyProp::new();
        cond.run(decl);
        report.stats.phi_collapses += cond.phi_collapses;
        let mut mv = MoveSemanticsCopyProp::new();
        mv.run(decl);
        report.stats.moves_performed += mv.moves_performed;
        let mut chain = CopyChainCollapser::new();
        chain.run(decl);
        report.stats.chain_collapses += chain.chains_collapsed;
        report.stats.max_chain_depth = chain.max_chain_depth;
        report.any_change = report.stats.total_optimizations() > 0;
        self.global_stats.copies_eliminated += report.stats.copies_eliminated;
        self.global_stats.vn_eliminated += report.stats.vn_eliminated;
        self.global_stats.dead_bindings_removed += report.stats.dead_bindings_removed;
        self.global_stats.phi_collapses += report.stats.phi_collapses;
        self.global_stats.moves_performed += report.stats.moves_performed;
        self.global_stats.chain_collapses += report.stats.chain_collapses;
        if chain.max_chain_depth > self.global_stats.max_chain_depth {
            self.global_stats.max_chain_depth = chain.max_chain_depth;
        }
        self.reports.push(report.clone());
        report
    }
    /// Returns the global statistics across all functions.
    pub fn global_report(&self) -> String {
        format!(
            "CopyPropPipeline: {} functions analyzed\n{}",
            self.reports.len(),
            self.global_stats.report()
        )
    }
    /// Returns the number of functions analyzed.
    pub fn num_analyzed(&self) -> usize {
        self.reports.len()
    }
    /// Returns all pipeline reports.
    pub fn all_reports(&self) -> &[CopyPropPipelineReport] {
        &self.reports
    }
    /// Returns the interference graph.
    pub fn interference_graph(&self) -> &InterferenceGraph {
        &self.interference_graph
    }
}
/// An optimization pipeline that runs passes in order.
#[allow(dead_code)]
pub struct OptPipeline {
    pub copy_prop: CopyProp,
    pub dead_binding: DeadBindingElim,
    pub constant_fold: ConstantFolder,
    pub enabled: Vec<PassKind>,
}
#[allow(dead_code)]
impl OptPipeline {
    pub fn new() -> Self {
        OptPipeline {
            copy_prop: CopyProp::default_pass(),
            dead_binding: DeadBindingElim::default_pass(),
            constant_fold: ConstantFolder::default_pass(),
            enabled: vec![
                PassKind::CopyProp,
                PassKind::DeadBinding,
                PassKind::ConstantFold,
            ],
        }
    }
    pub fn with_passes(passes: Vec<PassKind>) -> Self {
        let mut p = Self::new();
        p.enabled = passes;
        p
    }
    /// Run the full pipeline on a single function declaration.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) -> PipelineResult {
        if self.enabled.contains(&PassKind::CopyProp) {
            self.copy_prop.run(decl);
        }
        if self.enabled.contains(&PassKind::ConstantFold) {
            self.constant_fold.run(decl);
        }
        if self.enabled.contains(&PassKind::DeadBinding) {
            self.dead_binding.run(decl);
        }
        PipelineResult {
            copy_prop: self.copy_prop.report().clone(),
            dead_binding: self.dead_binding.report().clone(),
            constant_fold: self.constant_fold.report().clone(),
        }
    }
    /// Run the full pipeline on a module (a slice of declarations).
    pub fn run_module(&mut self, decls: &mut [LcnfFunDecl]) -> Vec<PipelineResult> {
        decls.iter_mut().map(|d| self.run(d)).collect()
    }
}
/// Eliminates bindings that are copies (`let x = y`) where `x` is never used.
///
/// After copy propagation, bindings like `let x = y` where all uses of `x`
/// were replaced with `y` become dead and should be removed.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct DeadCopyEliminator {
    /// Number of dead copy bindings removed.
    pub removed: usize,
}
#[allow(dead_code)]
impl DeadCopyEliminator {
    /// Creates a new dead copy eliminator.
    pub fn new() -> Self {
        DeadCopyEliminator::default()
    }
    /// Runs dead copy elimination on a function declaration.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        let used = self.collect_used(&decl.body);
        decl.body = self.elim_expr(
            std::mem::replace(&mut decl.body, LcnfExpr::Return(LcnfArg::Erased)),
            &used,
        );
    }
    pub(super) fn collect_used(&self, expr: &LcnfExpr) -> std::collections::HashSet<LcnfVarId> {
        let mut used: std::collections::HashSet<LcnfVarId> = std::collections::HashSet::new();
        self.collect_used_expr(expr, &mut used);
        used
    }
    pub(super) fn collect_used_expr(
        &self,
        expr: &LcnfExpr,
        used: &mut std::collections::HashSet<LcnfVarId>,
    ) {
        match expr {
            LcnfExpr::Return(arg) => self.collect_used_arg(arg, used),
            LcnfExpr::Let { value, body, .. } => {
                self.collect_used_let_value(value, used);
                self.collect_used_expr(body, used);
            }
            LcnfExpr::TailCall(fun, args) => {
                self.collect_used_arg(fun, used);
                for a in args {
                    self.collect_used_arg(a, used);
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                used.insert(*scrutinee);
                for alt in alts {
                    self.collect_used_expr(&alt.body, used);
                }
                if let Some(d) = default {
                    self.collect_used_expr(d, used);
                }
            }
            _ => {}
        }
    }
    pub(super) fn collect_used_arg(
        &self,
        arg: &LcnfArg,
        used: &mut std::collections::HashSet<LcnfVarId>,
    ) {
        if let LcnfArg::Var(id) = arg {
            used.insert(*id);
        }
    }
    pub(super) fn collect_used_let_value(
        &self,
        value: &LcnfLetValue,
        used: &mut std::collections::HashSet<LcnfVarId>,
    ) {
        match value {
            LcnfLetValue::FVar(a) => {
                used.insert(*a);
            }
            LcnfLetValue::App(fun, args) => {
                self.collect_used_arg(fun, used);
                for a in args {
                    self.collect_used_arg(a, used);
                }
            }
            _ => {}
        }
    }
    pub(super) fn elim_expr(
        &mut self,
        expr: LcnfExpr,
        used: &std::collections::HashSet<LcnfVarId>,
    ) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let is_copy = matches!(&value, LcnfLetValue::FVar(_));
                if is_copy && !used.contains(&id) {
                    self.removed += 1;
                    self.elim_expr(*body, used)
                } else {
                    let new_body = self.elim_expr(*body, used);
                    LcnfExpr::Let {
                        id,
                        name,
                        ty,
                        value,
                        body: Box::new(new_body),
                    }
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                let new_alts = alts
                    .into_iter()
                    .map(|alt| crate::lcnf::LcnfAlt {
                        ctor_name: alt.ctor_name,
                        ctor_tag: alt.ctor_tag,
                        params: alt.params,
                        body: self.elim_expr(alt.body, used),
                    })
                    .collect();
                let new_default = default.map(|d| Box::new(self.elim_expr(*d, used)));
                LcnfExpr::Case {
                    scrutinee,
                    scrutinee_ty: LcnfType::Erased,
                    alts: new_alts,
                    default: new_default,
                }
            }
            other => other,
        }
    }
    /// Returns a report of dead copy elimination.
    pub fn report(&self) -> String {
        format!("DeadCopyEliminator: {} dead bindings removed", self.removed)
    }
}
/// An edge in the interference graph between two variables.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct InterferenceEdge {
    /// First variable.
    pub u: LcnfVarId,
    /// Second variable.
    pub v: LcnfVarId,
}
#[allow(dead_code)]
impl InterferenceEdge {
    /// Creates a new interference edge (normalised so u ≤ v).
    pub fn new(a: LcnfVarId, b: LcnfVarId) -> Self {
        if a.0 <= b.0 {
            InterferenceEdge { u: a, v: b }
        } else {
            InterferenceEdge { u: b, v: a }
        }
    }
}
/// Configuration for dead-binding elimination.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeadBindingConfig {
    /// If true, bindings with observable side-effects are also removed
    /// (e.g. `let _ = panic!("...") in ...`).  Defaults to `false`.
    pub remove_effectful: bool,
    /// Maximum number of passes to run (default: 8).
    pub max_passes: usize,
}
/// A worklist-based dataflow solver for copy propagation.
///
/// Maintains a worklist of variables that need re-analysis. When the copy
/// source of a variable changes (e.g., due to propagation), all variables
/// that depend on it are added back to the worklist.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct WorklistSolver {
    /// Current substitution map.
    pub subst: std::collections::HashMap<LcnfVarId, LcnfArg>,
    /// Reverse dependency: var → set of vars that depend on var.
    pub dependents: std::collections::HashMap<LcnfVarId, std::collections::HashSet<LcnfVarId>>,
    /// The worklist.
    pub(super) worklist: std::collections::VecDeque<LcnfVarId>,
    /// Number of iterations performed.
    pub iterations: usize,
}
#[allow(dead_code)]
impl WorklistSolver {
    /// Creates a new worklist solver.
    pub fn new() -> Self {
        WorklistSolver::default()
    }
    /// Adds a copy fact: `id` is a copy of `src`.
    pub fn add_copy(&mut self, id: LcnfVarId, src: LcnfArg) {
        if let LcnfArg::Var(src_id) = &src {
            self.dependents.entry(*src_id).or_default().insert(id);
        }
        self.subst.insert(id, src);
        self.worklist.push_back(id);
    }
    /// Runs the worklist until it is empty.
    ///
    /// At each step, dequeues a variable, follows its copy chain to find the
    /// root, and updates the substitution. If the root changed, re-enqueues
    /// dependents.
    pub fn solve(&mut self) {
        while let Some(id) = self.worklist.pop_front() {
            self.iterations += 1;
            let new_root = self.follow_chain(id);
            let old = self.subst.get(&id).cloned();
            if old.as_ref() != Some(&new_root) {
                self.subst.insert(id, new_root);
                if let Some(deps) = self.dependents.get(&id).cloned() {
                    for dep in deps {
                        self.worklist.push_back(dep);
                    }
                }
            }
        }
    }
    pub(super) fn follow_chain(&self, id: LcnfVarId) -> LcnfArg {
        let mut current = LcnfArg::Var(id);
        let mut visited: std::collections::HashSet<LcnfVarId> = std::collections::HashSet::new();
        loop {
            match current {
                LcnfArg::Var(v) => {
                    if visited.contains(&v) {
                        break LcnfArg::Var(v);
                    }
                    visited.insert(v);
                    match self.subst.get(&v) {
                        Some(next) => current = next.clone(),
                        None => break LcnfArg::Var(v),
                    }
                }
                other => break other,
            }
        }
    }
    /// Looks up the root of `id`'s copy chain.
    pub fn lookup(&self, id: LcnfVarId) -> LcnfArg {
        self.subst.get(&id).cloned().unwrap_or(LcnfArg::Var(id))
    }
    /// Returns a summary report.
    pub fn report(&self) -> String {
        format!(
            "WorklistSolver: {} substitutions, {} iterations",
            self.subst.len(),
            self.iterations
        )
    }
}
/// Configuration for the copy propagation pass.
#[derive(Debug, Clone)]
pub struct CopyPropConfig {
    /// Maximum transitive chain depth to follow (e.g. `a=b, b=c, c=d`
    /// with `max_chain_depth=2` would resolve `a` to `c`, stopping before `d`).
    /// Use `usize::MAX` for unlimited depth.
    pub max_chain_depth: usize,
    /// If `true`, literal bindings (`let x = 42`) are also inlined at use
    /// sites.  If `false`, only variable aliases (`let x = y`) are propagated.
    pub fold_literals: bool,
}
/// Report from a single pipeline run.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct CopyPropPipelineReport {
    /// Function name.
    pub fn_name: String,
    /// Statistics from the run.
    pub stats: CopyPropStats,
    /// Whether any optimization was applied.
    pub any_change: bool,
}
#[allow(dead_code)]
impl CopyPropPipelineReport {
    /// Creates an empty report.
    pub fn new(fn_name: String) -> Self {
        CopyPropPipelineReport {
            fn_name,
            stats: CopyPropStats::new(),
            any_change: false,
        }
    }
    /// Returns a human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "CopyPropPipelineReport[{}]: {} total opts, changed={}",
            self.fn_name,
            self.stats.total_optimizations(),
            self.any_change
        )
    }
}
/// Dead-binding elimination pass.
///
/// This pass removes `let x = rhs` bindings where `x` is never
/// referenced in the continuation.  It works by first collecting a set of
/// all used variable IDs, then doing a second scan that drops any binding
/// whose `id` is not in that set.
#[allow(dead_code)]
pub struct DeadBindingElim {
    pub(super) config: DeadBindingConfig,
    pub(super) report: DeadBindingReport,
}
#[allow(dead_code)]
impl DeadBindingElim {
    pub fn new(config: DeadBindingConfig) -> Self {
        DeadBindingElim {
            config,
            report: DeadBindingReport::default(),
        }
    }
    pub fn default_pass() -> Self {
        Self::new(DeadBindingConfig::default())
    }
    pub fn report(&self) -> &DeadBindingReport {
        &self.report
    }
    /// Run the pass (potentially multiple times) until stable.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        for _ in 0..self.config.max_passes {
            let mut used = UsedVars::default();
            collect_used(&decl.body, &mut used);
            let old_count = self.report.bindings_removed;
            let old_body = std::mem::replace(&mut decl.body, LcnfExpr::Unreachable);
            let (new_body, changed) = self.elim(old_body, &used);
            decl.body = new_body;
            self.report.passes_run += 1;
            if !changed {
                break;
            }
            self.report.bindings_removed += self.report.bindings_removed - old_count;
            let _ = self.report.bindings_removed;
        }
    }
    pub(super) fn elim(&mut self, expr: LcnfExpr, used: &UsedVars) -> (LcnfExpr, bool) {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let (body2, mut changed) = self.elim(*body, used);
                if !used.vars.contains(&id) {
                    self.report.bindings_removed += 1;
                    changed = true;
                    (body2, changed)
                } else {
                    (
                        LcnfExpr::Let {
                            id,
                            name,
                            ty,
                            value,
                            body: Box::new(body2),
                        },
                        changed,
                    )
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                let mut changed = false;
                let alts2 = alts
                    .into_iter()
                    .map(|alt| {
                        let (body2, c) = self.elim(alt.body, used);
                        changed |= c;
                        LcnfAlt {
                            ctor_name: alt.ctor_name,
                            ctor_tag: alt.ctor_tag,
                            params: alt.params,
                            body: body2,
                        }
                    })
                    .collect();
                let default2 = default.map(|d| {
                    let (b, c) = self.elim(*d, used);
                    changed |= c;
                    Box::new(b)
                });
                (
                    LcnfExpr::Case {
                        scrutinee,
                        scrutinee_ty,
                        alts: alts2,
                        default: default2,
                    },
                    changed,
                )
            }
            other => (other, false),
        }
    }
}
/// Configuration for the constant folder.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConstantFoldConfig {
    /// Fold natural number arithmetic (`Nat.add`, `Nat.mul`, etc.).
    pub fold_nat_arith: bool,
    /// Fold boolean operations (`Bool.and`, `Bool.or`, `Bool.not`).
    pub fold_bool_ops: bool,
    /// Maximum result size for Nat literals (prevents giant numbers).
    pub max_nat_value: u64,
}
/// Inlining optimization pass.
///
/// Walks each function's LCNF body and replaces calls to small, non-recursive
/// callees with the callee's body (with parameters substituted for arguments).
/// The pass uses a fixed-point loop via [`InliningPass::run_all`] to handle
/// chains of inlinable functions.
pub struct InliningPass {
    pub config: InlineConfig,
    pub report: InlineReport,
}
impl InliningPass {
    pub fn new(config: InlineConfig) -> Self {
        InliningPass {
            config,
            report: InlineReport::default(),
        }
    }
    pub fn default_pass() -> Self {
        Self::new(InlineConfig::default())
    }
    pub fn report(&self) -> &InlineReport {
        &self.report
    }
    /// Check whether a declaration is a candidate for inlining.
    pub fn is_inline_candidate(&self, decl: &LcnfFunDecl) -> bool {
        if decl.is_recursive && !self.config.inline_recursive {
            return false;
        }
        decl.inline_cost <= self.config.threshold as usize
    }
    /// Run the inlining pass on a single function, given a map of all known
    /// function declarations.  Increments `report.functions_considered` and
    /// `report.inlines_performed` for each call site that is inlined.
    pub fn run_with_context(&mut self, decl: &mut LcnfFunDecl, fn_map: &FnMap) {
        self.report.functions_considered += 1;
        let caller_max_id = super::super::functions::max_var_id_in_expr(&decl.body);
        let mut id_counter: u64 = 0;
        let new_body = super::super::functions::inline_expr_walk(
            decl.body.clone(),
            fn_map,
            &self.config,
            caller_max_id,
            &mut id_counter,
            &mut self.report.inlines_performed,
        );
        decl.body = new_body;
    }
    /// Run the inlining pass over a full module (slice of declarations),
    /// iterating until a fixed-point is reached (no new inlines) or the
    /// maximum pass count (8) is exhausted.
    pub fn run_all(&mut self, decls: &mut Vec<LcnfFunDecl>) {
        for _pass in 0..8 {
            let fn_map: FnMap = decls.iter().map(|d| (d.name.clone(), d.clone())).collect();
            let before = self.report.inlines_performed;
            for decl in decls.iter_mut() {
                self.run_with_context(decl, &fn_map);
            }
            if self.report.inlines_performed == before {
                break;
            }
        }
    }
}
/// Report produced by dead-binding elimination.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DeadBindingReport {
    /// Number of dead let-bindings removed across all passes.
    pub bindings_removed: usize,
    /// Number of passes that actually changed something.
    pub passes_run: usize,
}
/// Copy propagation pass for LCNF.
pub struct CopyProp {
    pub(super) config: CopyPropConfig,
    pub(super) report: CopyPropReport,
}
impl CopyProp {
    /// Create a new pass with the given configuration.
    pub fn new(config: CopyPropConfig) -> Self {
        CopyProp {
            config,
            report: CopyPropReport::default(),
        }
    }
    /// Create a pass with default configuration.
    pub fn default_pass() -> Self {
        Self::new(CopyPropConfig::default())
    }
    /// Return the accumulated report after running the pass.
    pub fn report(&self) -> &CopyPropReport {
        &self.report
    }
    /// Run the pass on a single function declaration in place.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        let mut subst = SubstMap::new();
        let old_body = std::mem::replace(&mut decl.body, LcnfExpr::Unreachable);
        let new_body = self.prop_expr(old_body, &mut subst);
        decl.body = new_body;
    }
    /// Recursively propagate copies through an expression.
    pub(super) fn prop_expr(&mut self, expr: LcnfExpr, subst: &mut SubstMap) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let is_chain = if let LcnfLetValue::FVar(src) = &value {
                    subst.inner.contains_key(src)
                } else {
                    false
                };
                let value2 = self.prop_value(value, subst);
                if let Some(copy_arg) = self.extract_copy(&value2) {
                    self.report.copies_eliminated += 1;
                    if is_chain {
                        self.report.chains_followed += 1;
                    }
                    subst.insert(id, copy_arg);
                    self.prop_expr(*body, subst)
                } else {
                    let body2 = self.prop_expr(*body, subst);
                    LcnfExpr::Let {
                        id,
                        name,
                        ty,
                        value: value2,
                        body: Box::new(body2),
                    }
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                let scrutinee2 = {
                    let (arg, hops) = subst.lookup(scrutinee, self.config.max_chain_depth);
                    if hops > 0 {
                        self.report.copies_eliminated += 1;
                        if hops > 1 {
                            self.report.chains_followed += hops - 1;
                        }
                    }
                    match arg {
                        LcnfArg::Var(v) => v,
                        _ => scrutinee,
                    }
                };
                let alts2 = alts
                    .into_iter()
                    .map(|alt| {
                        let mut branch_subst = SubstMap {
                            inner: subst.inner.clone(),
                        };
                        LcnfAlt {
                            ctor_name: alt.ctor_name,
                            ctor_tag: alt.ctor_tag,
                            params: alt.params,
                            body: self.prop_expr(alt.body, &mut branch_subst),
                        }
                    })
                    .collect();
                let default2 = default.map(|d| {
                    let mut branch_subst = SubstMap {
                        inner: subst.inner.clone(),
                    };
                    Box::new(self.prop_expr(*d, &mut branch_subst))
                });
                LcnfExpr::Case {
                    scrutinee: scrutinee2,
                    scrutinee_ty,
                    alts: alts2,
                    default: default2,
                }
            }
            LcnfExpr::Return(arg) => {
                let (arg2, _hops) = subst.apply_arg(arg, self.config.max_chain_depth);
                LcnfExpr::Return(arg2)
            }
            LcnfExpr::TailCall(func, args) => {
                let (func2, h0) = subst.apply_arg(func, self.config.max_chain_depth);
                if h0 > 0 {
                    self.report.copies_eliminated += 1;
                    if h0 > 1 {
                        self.report.chains_followed += h0 - 1;
                    }
                }
                let args2 = args
                    .into_iter()
                    .map(|a| {
                        let (a2, hops) = subst.apply_arg(a, self.config.max_chain_depth);
                        if hops > 0 {
                            self.report.copies_eliminated += 1;
                            if hops > 1 {
                                self.report.chains_followed += hops - 1;
                            }
                        }
                        a2
                    })
                    .collect();
                LcnfExpr::TailCall(func2, args2)
            }
            LcnfExpr::Unreachable => LcnfExpr::Unreachable,
        }
    }
    /// Apply substitution to a let-value.
    pub(super) fn prop_value(&mut self, value: LcnfLetValue, subst: &SubstMap) -> LcnfLetValue {
        match value {
            LcnfLetValue::App(func, args) => {
                let (func2, h0) = subst.apply_arg(func, self.config.max_chain_depth);
                if h0 > 0 {
                    self.report.copies_eliminated += 1;
                    if h0 > 1 {
                        self.report.chains_followed += h0 - 1;
                    }
                }
                let args2 = args
                    .into_iter()
                    .map(|a| {
                        let (a2, hops) = subst.apply_arg(a, self.config.max_chain_depth);
                        if hops > 0 {
                            self.report.copies_eliminated += 1;
                            if hops > 1 {
                                self.report.chains_followed += hops - 1;
                            }
                        }
                        a2
                    })
                    .collect();
                LcnfLetValue::App(func2, args2)
            }
            LcnfLetValue::Proj(name, idx, var) => {
                let (arg, hops) = subst.apply_arg(LcnfArg::Var(var), self.config.max_chain_depth);
                if hops > 0 {
                    self.report.copies_eliminated += 1;
                    if hops > 1 {
                        self.report.chains_followed += hops - 1;
                    }
                }
                let var2 = match arg {
                    LcnfArg::Var(v) => v,
                    _ => var,
                };
                LcnfLetValue::Proj(name, idx, var2)
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let args2 = args
                    .into_iter()
                    .map(|a| {
                        let (a2, hops) = subst.apply_arg(a, self.config.max_chain_depth);
                        if hops > 0 {
                            self.report.copies_eliminated += 1;
                            if hops > 1 {
                                self.report.chains_followed += hops - 1;
                            }
                        }
                        a2
                    })
                    .collect();
                LcnfLetValue::Ctor(name, tag, args2)
            }
            LcnfLetValue::FVar(var) => {
                let (arg, _hops) = subst.apply_arg(LcnfArg::Var(var), self.config.max_chain_depth);
                match arg {
                    LcnfArg::Var(v) => LcnfLetValue::FVar(v),
                    LcnfArg::Lit(l) => LcnfLetValue::Lit(l),
                    _ => LcnfLetValue::FVar(var),
                }
            }
            LcnfLetValue::Reset(var) => {
                let (arg, hops) = subst.apply_arg(LcnfArg::Var(var), self.config.max_chain_depth);
                if hops > 0 {
                    self.report.copies_eliminated += 1;
                    if hops > 1 {
                        self.report.chains_followed += hops - 1;
                    }
                }
                let var2 = match arg {
                    LcnfArg::Var(v) => v,
                    _ => var,
                };
                LcnfLetValue::Reset(var2)
            }
            LcnfLetValue::Reuse(slot, name, tag, args) => {
                let (sarg, hops) = subst.apply_arg(LcnfArg::Var(slot), self.config.max_chain_depth);
                if hops > 0 {
                    self.report.copies_eliminated += 1;
                    if hops > 1 {
                        self.report.chains_followed += hops - 1;
                    }
                }
                let slot2 = match sarg {
                    LcnfArg::Var(v) => v,
                    _ => slot,
                };
                let args2 = args
                    .into_iter()
                    .map(|a| {
                        let (a2, hs) = subst.apply_arg(a, self.config.max_chain_depth);
                        if hs > 0 {
                            self.report.copies_eliminated += 1;
                            if hs > 1 {
                                self.report.chains_followed += hs - 1;
                            }
                        }
                        a2
                    })
                    .collect();
                LcnfLetValue::Reuse(slot2, name, tag, args2)
            }
            other => other,
        }
    }
    /// If `value` qualifies as a copy (FVar alias or, when `fold_literals` is
    /// set, a literal), return the `LcnfArg` to substitute for uses of the
    /// bound variable.  Otherwise return `None`.
    pub(super) fn extract_copy(&self, value: &LcnfLetValue) -> Option<LcnfArg> {
        match value {
            LcnfLetValue::FVar(v) => Some(LcnfArg::Var(*v)),
            LcnfLetValue::Lit(l) if self.config.fold_literals => Some(LcnfArg::Lit(l.clone())),
            LcnfLetValue::Erased => Some(LcnfArg::Erased),
            _ => None,
        }
    }
}
/// A hint for the register allocator to coalesce two variables.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RegisterCoalescingHint {
    /// The source variable (copy source).
    pub src: LcnfVarId,
    /// The destination variable (copy destination, to be eliminated).
    pub dst: LcnfVarId,
    /// Whether this coalescing is guaranteed safe (no interference).
    pub is_safe: bool,
    /// Estimated benefit (e.g., reduction in register pressure).
    pub benefit: u32,
}
#[allow(dead_code)]
impl RegisterCoalescingHint {
    /// Creates a new register coalescing hint.
    pub fn new(src: LcnfVarId, dst: LcnfVarId, is_safe: bool, benefit: u32) -> Self {
        RegisterCoalescingHint {
            src,
            dst,
            is_safe,
            benefit,
        }
    }
    /// Returns a human-readable description of the hint.
    pub fn describe(&self) -> String {
        format!(
            "Coalesce v{} ← v{} [{}] benefit={}",
            self.dst.0,
            self.src.0,
            if self.is_safe { "safe" } else { "speculative" },
            self.benefit
        )
    }
}
/// Report from the inlining pass.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct InlineReport {
    pub inlines_performed: usize,
    pub functions_considered: usize,
}
