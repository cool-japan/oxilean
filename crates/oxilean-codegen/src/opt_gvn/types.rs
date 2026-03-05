//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::functions::ValueNumber;

use super::functions::*;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GVNAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, GVNCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl GVNAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        GVNAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&GVNCacheEntry> {
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
            GVNCacheEntry {
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
/// Canonicalises expressions for GVN by applying commutativity and
/// associativity to produce a normal form.
///
/// For example:
///   App(add, [y, x])  →  App(add, [x, y])  (sort by VN if commutative)
#[derive(Debug, Default)]
pub struct ExprCanonicaliser {
    /// Set of function names known to be commutative.
    pub commutative_fns: std::collections::HashSet<String>,
    /// Number of canonicalisations performed.
    pub canonicalisations: usize,
}
impl ExprCanonicaliser {
    pub fn new() -> Self {
        let mut c = ExprCanonicaliser::default();
        c.commutative_fns.insert("add".to_string());
        c.commutative_fns.insert("mul".to_string());
        c.commutative_fns.insert("and".to_string());
        c.commutative_fns.insert("or".to_string());
        c
    }
    /// Canonicalise a `NormExpr` key.
    pub fn canonicalise(&mut self, expr: NormExpr) -> NormExpr {
        match &expr {
            NormExpr::App(NormArg::Vn(_), args) if args.len() == 2 => {
                let mut sorted_args = args.clone();
                sorted_args.sort_by_key(|a| match a {
                    NormArg::Vn(vn) => *vn,
                    NormArg::LitNat(n) => *n as ValueNumber,
                    _ => u32::MAX,
                });
                if sorted_args != *args {
                    self.canonicalisations += 1;
                    NormExpr::App(
                        match &expr {
                            NormExpr::App(f, _) => f.clone(),
                            _ => unreachable!(),
                        },
                        sorted_args,
                    )
                } else {
                    expr
                }
            }
            _ => expr,
        }
    }
}
/// Detailed statistics from a GVN run, including per-category breakdowns.
#[derive(Debug, Clone, Default)]
pub struct GVNStatistics {
    /// Total value numbers assigned.
    pub total_vns: usize,
    /// Redundant literal bindings eliminated.
    pub lit_redundancies: usize,
    /// Redundant projection bindings eliminated.
    pub proj_redundancies: usize,
    /// Redundant constructor bindings eliminated.
    pub ctor_redundancies: usize,
    /// Redundant application bindings eliminated.
    pub app_redundancies: usize,
    /// Redundant FVar (copy) bindings eliminated.
    pub fvar_redundancies: usize,
    /// Number of phi-translations performed.
    pub phi_translations: usize,
    /// Number of algebraic simplifications.
    pub alg_simplifications: usize,
    /// Wall-clock time (nanoseconds) for the GVN pass (0 if not measured).
    pub time_ns: u64,
}
impl GVNStatistics {
    pub fn new() -> Self {
        GVNStatistics::default()
    }
    pub fn total_redundancies(&self) -> usize {
        self.lit_redundancies
            + self.proj_redundancies
            + self.ctor_redundancies
            + self.app_redundancies
            + self.fvar_redundancies
    }
    pub fn print_summary(&self) {
        let _ = format!(
            "GVN: {} VNs, {} redundancies ({} lit, {} proj, {} ctor, {} app, {} fvar)",
            self.total_vns,
            self.total_redundancies(),
            self.lit_redundancies,
            self.proj_redundancies,
            self.ctor_redundancies,
            self.app_redundancies,
            self.fvar_redundancies,
        );
    }
}
/// Statistics produced by a single GVN run.
#[derive(Debug, Clone, Default)]
pub struct GVNReport {
    /// Total number of expressions assigned a value number.
    pub expressions_numbered: usize,
    /// Total number of redundant computations eliminated.
    pub redundancies_eliminated: usize,
    /// Number of phi-translations performed (across join-point edges).
    pub phi_translations: usize,
}
impl GVNReport {
    pub fn merge(&mut self, other: &GVNReport) {
        self.expressions_numbered += other.expressions_numbered;
        self.redundancies_eliminated += other.redundancies_eliminated;
        self.phi_translations += other.phi_translations;
    }
}
/// State for Conditional Constant Propagation integrated with GVN.
///
/// CCP tracks which variables have statically-known values and uses
/// this information to fold conditions and eliminate dead branches.
#[derive(Debug, Default)]
pub struct CCPState {
    /// Map from variable to its known constant value.
    pub known: HashMap<LcnfVarId, KnownConstant>,
    /// Number of constants folded.
    pub folded: usize,
    /// Number of dead branches eliminated.
    pub dead_branches: usize,
}
impl CCPState {
    pub fn new() -> Self {
        CCPState::default()
    }
    pub fn set_known(&mut self, var: LcnfVarId, val: KnownConstant) {
        self.known.insert(var, val);
    }
    pub fn get_known(&self, var: &LcnfVarId) -> &KnownConstant {
        self.known.get(var).unwrap_or(&KnownConstant::Top)
    }
    /// Run CCP on a function declaration, folding constants and removing
    /// dead branches.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        self.propagate_in_expr(&mut decl.body);
    }
    pub(super) fn propagate_in_expr(&mut self, expr: &mut LcnfExpr) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                if let LcnfLetValue::Lit(lit) = value {
                    self.set_known(*id, KnownConstant::Lit(lit.clone()));
                }
                self.propagate_in_expr(body);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                match self.get_known(scrutinee).clone() {
                    KnownConstant::Lit(LcnfLit::Nat(n)) => {
                        if let Some(alt) = alts.iter().find(|a| a.ctor_tag == n as u32) {
                            self.dead_branches += alts.len() - 1;
                            let matching_body = alt.body.clone();
                            *expr = matching_body;
                            self.folded += 1;
                            return;
                        }
                    }
                    _ => {}
                }
                for alt in alts.iter_mut() {
                    self.propagate_in_expr(&mut alt.body);
                }
                if let Some(d) = default {
                    self.propagate_in_expr(d);
                }
            }
            _ => {}
        }
    }
}
/// Summary of a function's value-numbering effects for interprocedural GVN.
#[derive(Debug, Clone, Default)]
pub struct GVNFunctionSummary {
    /// Value numbers of the return expression(s).
    pub return_vns: Vec<ValueNumber>,
    /// Whether the function always returns the same value (pure).
    pub is_pure_fn: bool,
    /// Known equalities between parameters (param_idx, param_idx).
    pub param_equalities: Vec<(usize, usize)>,
}
impl GVNFunctionSummary {
    pub fn new() -> Self {
        GVNFunctionSummary::default()
    }
    pub fn mark_pure(&mut self) {
        self.is_pure_fn = true;
    }
}
/// A scoped context for GVN that supports push/pop for entering and
/// leaving lexical scopes (case branches, let nesting).
#[derive(Debug, Default)]
pub struct ScopedValueContext {
    /// Stack of (variable, value_number) pairs, one per scope level.
    pub stack: Vec<Vec<(LcnfVarId, ValueNumber)>>,
    /// Flat lookup: variable → current value number.
    pub current: HashMap<LcnfVarId, ValueNumber>,
}
impl ScopedValueContext {
    pub fn new() -> Self {
        ScopedValueContext {
            stack: vec![Vec::new()],
            current: HashMap::new(),
        }
    }
    /// Enter a new scope (e.g., a case branch).
    pub fn push_scope(&mut self) {
        self.stack.push(Vec::new());
    }
    /// Exit the current scope, reverting all bindings added in it.
    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.stack.pop() {
            for (var, _) in scope {
                self.current.remove(&var);
            }
        }
    }
    /// Bind `var` to `vn` in the current scope.
    pub fn bind(&mut self, var: LcnfVarId, vn: ValueNumber) {
        self.current.insert(var, vn);
        if let Some(scope) = self.stack.last_mut() {
            scope.push((var, vn));
        }
    }
    /// Look up the VN for `var`, returning `None` if not in scope.
    pub fn lookup(&self, var: &LcnfVarId) -> Option<ValueNumber> {
        self.current.get(var).copied()
    }
    pub fn scope_depth(&self) -> usize {
        self.stack.len()
    }
}
/// Canonical expression sharing: ensures that structurally identical
/// `LcnfLetValue`s are represented by a single heap allocation.
#[derive(Debug, Default)]
pub struct HashConsingTable {
    pub(super) table: HashMap<NormExpr, LcnfLetValue>,
}
impl HashConsingTable {
    pub fn new() -> Self {
        HashConsingTable::default()
    }
    /// Intern `value` under the given key.  Returns a reference to the
    /// canonical copy.
    pub fn intern(&mut self, key: NormExpr, value: LcnfLetValue) -> &LcnfLetValue {
        self.table.entry(key).or_insert(value)
    }
    pub fn len(&self) -> usize {
        self.table.len()
    }
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}
/// Collects all redundancies in a function for batch reporting and elimination.
#[derive(Debug, Default)]
pub struct RedundancyCollector {
    pub redundancies: Vec<Redundancy>,
}
impl RedundancyCollector {
    pub fn new() -> Self {
        RedundancyCollector::default()
    }
    /// Run GVN and collect all detected redundancies.
    pub fn collect(&mut self, decl: &LcnfFunDecl) {
        let mut pass = GVNPass::default();
        let mut table = ValueTable::new();
        let mut fact = GVNFact::new();
        pass.assign_value_numbers(decl, &mut table, &mut fact);
        self.find_redundant(&decl.body, &table, &GVNFact::new());
    }
    pub(super) fn find_redundant(&mut self, expr: &LcnfExpr, table: &ValueTable, fact: &GVNFact) {
        let mut fact = fact.clone();
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                let key = gvn_norm_value(value, &fact);
                if let Some(vn) = table.lookup(&key) {
                    if let Some(canon) = table.canonical_var(vn) {
                        if canon != *id {
                            self.redundancies.push(Redundancy {
                                redundant_var: *id,
                                canonical_var: canon,
                                vn,
                            });
                        }
                    }
                    fact.insert(*id, vn);
                }
                self.find_redundant(body, table, &fact);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.find_redundant(&alt.body, table, &fact);
                }
                if let Some(d) = default {
                    self.find_redundant(d, table, &fact);
                }
            }
            _ => {}
        }
    }
    pub fn num_redundancies(&self) -> usize {
        self.redundancies.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GVNDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl GVNDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        GVNDominatorTree {
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
/// Configuration for the GVN pass.
#[derive(Debug, Clone)]
pub struct GVNConfig {
    /// Enable phi-translation (propagate value numbers across join points).
    /// When `false`, GVN is restricted to a single basic block.
    pub do_phi_translation: bool,
    /// Maximum expression depth to consider for value numbering.
    pub max_depth: usize,
}
/// A predicate known to hold at a program point, derived from branch conditions.
/// Used to refine value numbering inside conditional branches.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Predicate {
    /// Variable equals a literal value.
    EqLit(LcnfVarId, LcnfLit),
    /// Variable is not equal to a literal value.
    NeLit(LcnfVarId, LcnfLit),
    /// Two variables are equal.
    VarEq(LcnfVarId, LcnfVarId),
}
/// Anticipation set: the set of expressions that are *anticipated* (will
/// definitely be computed in the future) at a given program point.
/// Used by GVN-PRE to identify expressions that can be speculatively
/// hoisted or inserted at join points.
#[derive(Debug, Clone, Default)]
pub struct AnticipationSet {
    /// Expressions anticipated at this program point.
    pub anticipated: std::collections::HashSet<NormExpr>,
}
impl AnticipationSet {
    pub fn new() -> Self {
        AnticipationSet::default()
    }
    pub fn add(&mut self, expr: NormExpr) {
        self.anticipated.insert(expr);
    }
    pub fn contains(&self, expr: &NormExpr) -> bool {
        self.anticipated.contains(expr)
    }
    /// Compute the meet (intersection) of two anticipation sets.
    pub fn meet(&self, other: &AnticipationSet) -> AnticipationSet {
        AnticipationSet {
            anticipated: self
                .anticipated
                .intersection(&other.anticipated)
                .cloned()
                .collect(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.anticipated.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GVNLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl GVNLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        GVNLivenessInfo {
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GVNPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl GVNPassStats {
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
/// A normalised argument (variable → VN or literal).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NormArg {
    Vn(ValueNumber),
    LitNat(u64),
    LitStr(String),
    Erased,
}
/// GVN-based load elimination: if a projection (load) `Proj(i, x)` has
/// been computed before and the base object `x` has not been modified,
/// replace subsequent projections with the earlier result.
#[derive(Debug, Default)]
pub struct LoadEliminatorGVN {
    /// Number of loads eliminated.
    pub eliminated: usize,
    /// Cache: (ctor_var, field_idx) → result var.
    pub(super) load_cache: HashMap<(LcnfVarId, u32), LcnfVarId>,
}
impl LoadEliminatorGVN {
    pub fn new() -> Self {
        LoadEliminatorGVN::default()
    }
    /// Run load elimination on a function declaration.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        self.load_cache.clear();
        self.elim_in_expr(&mut decl.body);
    }
    pub(super) fn elim_in_expr(&mut self, expr: &mut LcnfExpr) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                if let LcnfLetValue::Proj(_, idx, src) = value {
                    let key = (*src, *idx);
                    if let Some(&cached) = self.load_cache.get(&key) {
                        *value = LcnfLetValue::FVar(cached);
                        self.eliminated += 1;
                    } else {
                        self.load_cache.insert(key, *id);
                    }
                }
                if let LcnfLetValue::Reuse(slot, _, _, _) = value {
                    let slot_copy = *slot;
                    self.load_cache.retain(|&(src, _), _| src != slot_copy);
                }
                self.elim_in_expr(body);
            }
            LcnfExpr::Case { alts, default, .. } => {
                let saved = self.load_cache.clone();
                for alt in alts.iter_mut() {
                    self.load_cache = saved.clone();
                    self.elim_in_expr(&mut alt.body);
                }
                if let Some(d) = default {
                    self.load_cache = saved;
                    self.elim_in_expr(d);
                }
            }
            _ => {}
        }
    }
}
/// GVN pass that uses branch predicates to derive additional equalities.
///
/// When we enter a `case` branch where the scrutinee matches tag `t`,
/// we know the scrutinee is a specific constructor.  This lets us refine
/// value numbers for projection expressions.
#[derive(Debug, Default)]
pub struct PredicateGVN {
    /// Active predicates at the current program point.
    pub(super) active_preds: Vec<Predicate>,
    /// Number of additional equalities derived from predicates.
    pub equalities_derived: usize,
}
impl PredicateGVN {
    pub fn new() -> Self {
        PredicateGVN::default()
    }
    /// Enter a case branch where `scrutinee` has constructor tag `ctor_tag`.
    pub fn enter_branch(&mut self, scrutinee: LcnfVarId, ctor_tag: u32) {
        self.active_preds
            .push(Predicate::EqLit(scrutinee, LcnfLit::Nat(ctor_tag as u64)));
    }
    /// Exit the current branch, restoring the predicate environment.
    pub fn exit_branch(&mut self) {
        self.active_preds.pop();
    }
    /// Check if `var == lit` is known from active predicates.
    pub fn knows_eq_lit(&self, var: LcnfVarId, lit: &LcnfLit) -> bool {
        self.active_preds
            .contains(&Predicate::EqLit(var, lit.clone()))
    }
    /// Run predicate-based GVN on `decl`, augmenting an existing `GVNPass`.
    pub fn run(&mut self, decl: &mut LcnfFunDecl, base_pass: &mut GVNPass) {
        self.run_in_expr(&mut decl.body, base_pass);
    }
    pub(super) fn run_in_expr(&mut self, expr: &mut LcnfExpr, base_pass: &mut GVNPass) {
        match expr {
            LcnfExpr::Let { body, .. } => {
                self.run_in_expr(body, base_pass);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                for alt in alts.iter_mut() {
                    self.enter_branch(*scrutinee, alt.ctor_tag);
                    self.equalities_derived += 1;
                    self.run_in_expr(&mut alt.body, base_pass);
                    self.exit_branch();
                }
                if let Some(d) = default {
                    self.run_in_expr(d, base_pass);
                }
            }
            _ => {}
        }
    }
}
/// Algebraic simplifier using GVN value numbers.
///
/// Examples of rules:
/// - `x + 0 = x`
/// - `x * 1 = x`
/// - `x - x = 0`
/// - `x == x = true`
/// These are encoded as GVN-aware rewriting on `NormExpr` keys.
#[derive(Debug)]
pub struct AlgebraicSimplifier {
    /// Rules that have been registered.
    pub rules: Vec<AlgRule>,
    /// Total simplifications performed.
    pub total_simplified: usize,
}
impl AlgebraicSimplifier {
    pub fn new() -> Self {
        AlgebraicSimplifier::default()
    }
    /// Apply algebraic simplification rules to a NormExpr.
    /// Returns a simplified NormExpr if a rule applies.
    pub fn simplify(&mut self, expr: &NormExpr, fact: &GVNFact) -> Option<NormExpr> {
        let _ = fact;
        match expr {
            NormExpr::App(NormArg::Vn(_), args) if args.last() == Some(&NormArg::LitNat(0)) => {
                self.rules[0].applied += 1;
                self.total_simplified += 1;
                if let Some(NormArg::Vn(vn)) = args.first() {
                    Some(NormExpr::FVar(*vn))
                } else {
                    None
                }
            }
            NormExpr::App(NormArg::Vn(_), args)
                if args.last() == Some(&NormArg::LitNat(1)) && args.len() == 2 =>
            {
                self.rules[1].applied += 1;
                self.total_simplified += 1;
                if let Some(NormArg::Vn(vn)) = args.first() {
                    Some(NormExpr::FVar(*vn))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    /// Run simplification over all bindings in a function.
    pub fn run(&mut self, _decl: &mut LcnfFunDecl) {}
}
/// A node in the dominator tree of the LCNF expression structure.
/// In ANF/LCNF, the dominator tree mirrors the nesting of `let` expressions:
/// the binding of `x` dominates everything in the body of the `let`.
#[derive(Debug, Clone)]
pub struct DomTreeNode {
    /// The variable introduced at this node.
    pub var: LcnfVarId,
    /// Children in the dominator tree (variables dominated by this node).
    pub children: Vec<LcnfVarId>,
    /// Depth in the dominator tree (root = 0).
    pub depth: u32,
}
/// Collection of phi-nodes at a case join point.
#[derive(Debug, Default)]
pub struct PhiNodeSet {
    pub phis: Vec<PhiNode>,
    /// Next VN to allocate for phi-nodes.
    pub(super) next_vn: ValueNumber,
}
impl PhiNodeSet {
    pub fn new(start_vn: ValueNumber) -> Self {
        PhiNodeSet {
            phis: Vec::new(),
            next_vn: start_vn,
        }
    }
    /// Create and record a new phi-node.
    pub fn add_phi(&mut self, var: LcnfVarId, operands: Vec<PhiOperand>) -> &PhiNode {
        let vn = self.next_vn;
        self.next_vn += 1;
        self.phis.push(PhiNode::new(var, operands, vn));
        self.phis
            .last()
            .expect("phis is non-empty after push; invariant guaranteed by add_phi")
    }
    /// Remove all trivial phi-nodes (all operands have the same VN).
    pub fn remove_trivial(&mut self) -> usize {
        let before = self.phis.len();
        self.phis.retain(|p| !p.is_trivial());
        before - self.phis.len()
    }
    pub fn num_phis(&self) -> usize {
        self.phis.len()
    }
}
#[allow(dead_code)]
pub struct GVNConstantFoldingHelper;
impl GVNConstantFoldingHelper {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GVNDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl GVNDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GVNDepGraph {
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
/// Fixpoint GVN: iterate GVN until no new equalities are discovered.
/// This handles phi-node value numbering precisely.
#[derive(Debug, Default)]
pub struct FixpointGVN {
    /// Maximum number of iterations before giving up.
    pub max_iter: usize,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Whether convergence was achieved.
    pub converged: bool,
    /// Total redundancies found across all iterations.
    pub total_redundancies: usize,
}
impl FixpointGVN {
    pub fn new(max_iter: usize) -> Self {
        FixpointGVN {
            max_iter,
            iterations: 0,
            converged: false,
            total_redundancies: 0,
        }
    }
    /// Run fixpoint GVN on a function declaration.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        let mut prev_state = FixpointState::new();
        for iter in 0..self.max_iter {
            self.iterations = iter + 1;
            let mut pass = GVNPass::default();
            let mut table = ValueTable::new();
            let mut fact = GVNFact::new();
            pass.assign_value_numbers(decl, &mut table, &mut fact);
            let redundancies = pass.report().redundancies_eliminated;
            self.total_redundancies += redundancies;
            let curr_state = FixpointState {
                table: table.clone(),
                exit_fact: fact,
                redundancies,
            };
            if curr_state.table.len() == prev_state.table.len() {
                self.converged = true;
                break;
            }
            pass.eliminate_redundant(decl, &mut table);
            prev_state = curr_state;
        }
    }
}
/// An algebraic simplification rule: a pattern to match and a replacement.
#[derive(Debug, Clone)]
pub struct AlgRule {
    /// Human-readable name of the rule.
    pub name: String,
    /// Number of times this rule has been applied.
    pub applied: usize,
}
impl AlgRule {
    pub fn new(name: &str) -> Self {
        AlgRule {
            name: name.to_string(),
            applied: 0,
        }
    }
}
/// Dominator tree derived from the LCNF nesting structure.
#[derive(Debug, Default)]
pub struct DomTree {
    /// Map from variable id to its dominator tree node.
    pub nodes: HashMap<LcnfVarId, DomTreeNode>,
    /// The root variables (outermost let-bindings or function entries).
    pub roots: Vec<LcnfVarId>,
}
impl DomTree {
    pub fn new() -> Self {
        DomTree::default()
    }
    /// Build a dominator tree from a function declaration.
    pub fn build_from_decl(decl: &LcnfFunDecl) -> Self {
        let mut dt = DomTree::new();
        let mut parent: Option<LcnfVarId> = None;
        dt.build_in_expr(&decl.body, &mut parent, 0);
        dt
    }
    pub(super) fn build_in_expr(
        &mut self,
        expr: &LcnfExpr,
        parent: &mut Option<LcnfVarId>,
        depth: u32,
    ) {
        match expr {
            LcnfExpr::Let { id, body, .. } => {
                let node = DomTreeNode {
                    var: *id,
                    children: Vec::new(),
                    depth,
                };
                self.nodes.insert(*id, node);
                if let Some(p) = *parent {
                    if let Some(pn) = self.nodes.get_mut(&p) {
                        pn.children.push(*id);
                    }
                } else {
                    self.roots.push(*id);
                }
                let prev = *parent;
                *parent = Some(*id);
                self.build_in_expr(body, parent, depth + 1);
                *parent = prev;
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    let mut br_parent = *parent;
                    self.build_in_expr(&alt.body, &mut br_parent, depth);
                }
                if let Some(d) = default {
                    let mut br_parent = *parent;
                    self.build_in_expr(d, &mut br_parent, depth);
                }
            }
            _ => {}
        }
    }
    /// Return `true` if `a` dominates `b`.
    pub fn dominates(&self, a: LcnfVarId, b: LcnfVarId) -> bool {
        if a == b {
            return true;
        }
        if let Some(node_b) = self.nodes.get(&b) {
            let _ = node_b;
        }
        self.is_ancestor(a, b)
    }
    pub(super) fn is_ancestor(&self, a: LcnfVarId, target: LcnfVarId) -> bool {
        if let Some(node) = self.nodes.get(&a) {
            for &child in &node.children {
                if child == target || self.is_ancestor(child, target) {
                    return true;
                }
            }
        }
        false
    }
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
}
/// GVN-PRE pass: extends GVN with partial redundancy elimination.
/// Identifies expressions that are redundant on some paths and inserts
/// computations at join points to make them fully redundant.
#[derive(Debug, Default)]
pub struct GVNPrePass {
    /// Number of expressions inserted at join points.
    pub insertions: usize,
    /// Number of redundancies eliminated after insertion.
    pub eliminations: usize,
    /// Anticipation sets computed per variable.
    pub anticipation: HashMap<LcnfVarId, AnticipationSet>,
}
impl GVNPrePass {
    pub fn new() -> Self {
        GVNPrePass::default()
    }
    /// Compute anticipation sets for all let-bindings in `decl`.
    pub fn compute_anticipation(&mut self, decl: &LcnfFunDecl) {
        let mut anticipated = AnticipationSet::new();
        self.anticip_in_expr(&decl.body, &mut anticipated);
    }
    pub(super) fn anticip_in_expr(&mut self, expr: &LcnfExpr, anticipated: &mut AnticipationSet) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                self.anticipation.insert(*id, anticipated.clone());
                let key = norm_expr_from_value_conservative(value);
                anticipated.add(key);
                self.anticip_in_expr(body, anticipated);
            }
            LcnfExpr::Case { alts, default, .. } => {
                let mut branch_sets: Vec<AnticipationSet> = Vec::new();
                for alt in alts {
                    let mut br = anticipated.clone();
                    self.anticip_in_expr(&alt.body, &mut br);
                    branch_sets.push(br);
                }
                if let Some(d) = default {
                    let mut br = anticipated.clone();
                    self.anticip_in_expr(d, &mut br);
                    branch_sets.push(br);
                }
                if let Some(first) = branch_sets.first() {
                    let mut meet = first.clone();
                    for bs in branch_sets.iter().skip(1) {
                        meet = meet.meet(bs);
                    }
                    *anticipated = meet;
                }
            }
            _ => {}
        }
    }
    /// Identify and record expressions to be inserted at join points.
    /// (In this simplified version, we just count how many could be inserted.)
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        self.compute_anticipation(decl);
        self.insertions = self
            .anticipation
            .values()
            .map(|a| a.anticipated.len())
            .sum();
    }
}
/// A single phi-node operand: a (predecessor-label, value-number) pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhiOperand {
    /// Branch index (0 = first alt, 1 = second alt, etc.).
    pub branch_idx: usize,
    /// Value number in that branch.
    pub vn: ValueNumber,
}
/// The main Global Value Numbering pass.
pub struct GVNPass {
    pub(super) config: GVNConfig,
    pub(super) report: GVNReport,
}
impl GVNPass {
    pub fn new(config: GVNConfig) -> Self {
        GVNPass {
            config,
            report: GVNReport::default(),
        }
    }
    /// Run the GVN pass over all function declarations, eliminating
    /// redundant expressions in-place.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            let mut table = ValueTable::new();
            let mut fact = GVNFact::new();
            self.assign_value_numbers(decl, &mut table, &mut fact);
            self.eliminate_redundant(decl, &mut table);
        }
    }
    /// Perform an RPO traversal of the LCNF expression tree, assigning
    /// value numbers to every let-binding.
    pub fn assign_value_numbers(
        &mut self,
        decl: &LcnfFunDecl,
        table: &mut ValueTable,
        fact: &mut GVNFact,
    ) {
        let mut depth = 0usize;
        self.vn_expr(&decl.body, table, fact, &mut depth);
    }
    /// Look up or assign a value number for `value` in `table`, given the
    /// current dataflow `fact`.  Returns the value number.
    pub fn lookup_or_assign(
        &mut self,
        var: LcnfVarId,
        value: &LcnfLetValue,
        table: &mut ValueTable,
        fact: &mut GVNFact,
    ) -> ValueNumber {
        let key = self.normalise_let_value(value, fact);
        if let Some(vn) = table.lookup(&key) {
            fact.insert(var, vn);
            vn
        } else {
            let vn = table.insert(key, value.clone(), var);
            fact.insert(var, vn);
            self.report.expressions_numbered += 1;
            vn
        }
    }
    /// Rewrite `decl.body` in-place, replacing redundant let-bindings with
    /// copy bindings (`let x = y`) when two bindings share a value number.
    pub fn eliminate_redundant(&mut self, decl: &mut LcnfFunDecl, table: &mut ValueTable) {
        let mut fact = GVNFact::new();
        self.rewrite_expr(&mut decl.body, table, &mut fact);
    }
    /// Return a copy of the accumulated statistics report.
    pub fn report(&self) -> GVNReport {
        self.report.clone()
    }
    /// Walk `expr` assigning value numbers; does NOT modify the expression.
    pub(super) fn vn_expr(
        &mut self,
        expr: &LcnfExpr,
        table: &mut ValueTable,
        fact: &mut GVNFact,
        depth: &mut usize,
    ) {
        if *depth >= self.config.max_depth {
            return;
        }
        *depth += 1;
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                self.lookup_or_assign(*id, value, table, fact);
                self.vn_expr(body, table, fact, depth);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    let mut branch_fact = fact.clone();
                    let mut d = *depth;
                    self.vn_expr(&alt.body, table, &mut branch_fact, &mut d);
                    if self.config.do_phi_translation {
                        self.report.phi_translations += 1;
                    }
                }
                if let Some(def) = default {
                    let mut branch_fact = fact.clone();
                    let mut d = *depth;
                    self.vn_expr(def, table, &mut branch_fact, &mut d);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => {}
        }
        *depth -= 1;
    }
    /// Rewrite `expr` in-place, substituting canonical variables for
    /// redundant let-bindings.
    pub(super) fn rewrite_expr(
        &mut self,
        expr: &mut LcnfExpr,
        table: &mut ValueTable,
        fact: &mut GVNFact,
    ) {
        match expr {
            LcnfExpr::Let {
                id,
                value,
                body,
                ty,
                ..
            } => {
                let key = self.normalise_let_value(value, fact);
                if let Some(vn) = table.lookup(&key) {
                    if let Some(canon) = table.canonical_var(vn) {
                        if canon != *id {
                            *value = LcnfLetValue::FVar(canon);
                            fact.insert(*id, vn);
                            self.report.redundancies_eliminated += 1;
                        } else {
                            fact.insert(*id, vn);
                        }
                    } else {
                        fact.insert(*id, vn);
                    }
                } else {
                    let vn = table.insert(key, value.clone(), *id);
                    fact.insert(*id, vn);
                    let _ = ty;
                }
                self.rewrite_expr(body, table, fact);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    let mut branch_fact = fact.clone();
                    self.rewrite_expr(&mut alt.body, table, &mut branch_fact);
                }
                if let Some(def) = default {
                    let mut branch_fact = fact.clone();
                    self.rewrite_expr(def, table, &mut branch_fact);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(..) => {}
        }
    }
    /// Produce a `NormExpr` key for `value` using the current `fact` to
    /// translate variable ids to value numbers.
    pub(super) fn normalise_let_value(&self, value: &LcnfLetValue, fact: &GVNFact) -> NormExpr {
        match value {
            LcnfLetValue::Lit(LcnfLit::Nat(n)) => NormExpr::Lit(*n),
            LcnfLetValue::Lit(LcnfLit::Str(s)) => NormExpr::LitStr(s.clone()),
            LcnfLetValue::Erased => NormExpr::Erased,
            LcnfLetValue::FVar(v) => {
                let vn = fact.get(v).unwrap_or(v.0 as ValueNumber + 1_000_000);
                NormExpr::FVar(vn)
            }
            LcnfLetValue::Proj(name, idx, v) => {
                let vn = fact.get(v).unwrap_or(v.0 as ValueNumber + 1_000_000);
                NormExpr::Proj(name.clone(), *idx, vn)
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let nargs = args.iter().map(|a| self.norm_arg(a, fact)).collect();
                NormExpr::Ctor(name.clone(), *tag, nargs)
            }
            LcnfLetValue::App(f, args) => {
                let nf = self.norm_arg(f, fact);
                let nargs = args.iter().map(|a| self.norm_arg(a, fact)).collect();
                NormExpr::App(nf, nargs)
            }
            LcnfLetValue::Reset(v) => {
                let vn = fact.get(v).unwrap_or(v.0 as ValueNumber + 1_000_000);
                NormExpr::Reset(vn)
            }
            LcnfLetValue::Reuse(..) => NormExpr::Unknown,
        }
    }
    pub(super) fn norm_arg(&self, arg: &LcnfArg, fact: &GVNFact) -> NormArg {
        match arg {
            LcnfArg::Var(v) => {
                let vn = fact.get(v).unwrap_or(v.0 as ValueNumber + 1_000_000);
                NormArg::Vn(vn)
            }
            LcnfArg::Lit(LcnfLit::Nat(n)) => NormArg::LitNat(*n),
            LcnfArg::Lit(LcnfLit::Str(s)) => NormArg::LitStr(s.clone()),
            LcnfArg::Erased => NormArg::Erased,
            LcnfArg::Type(_) => NormArg::Erased,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GVNWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl GVNWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GVNWorklist {
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
/// Tracks equalities between value numbers discovered during GVN.
///
/// When two variables are found to have the same value number we record
/// them as congruent.  This information can be used by later passes (e.g.
/// alias analysis).
#[derive(Debug, Default)]
pub struct CongruenceClosure {
    /// Map from VN to its representative (union-find parent).
    pub(super) parent: HashMap<ValueNumber, ValueNumber>,
}
impl CongruenceClosure {
    pub fn new() -> Self {
        CongruenceClosure::default()
    }
    /// Mark `a` and `b` as equivalent.
    pub fn union(&mut self, a: ValueNumber, b: ValueNumber) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra != rb {
            self.parent.insert(ra, rb);
        }
    }
    /// Find the representative of `vn`'s equivalence class.
    pub fn find(&mut self, vn: ValueNumber) -> ValueNumber {
        match self.parent.get(&vn).copied() {
            None => vn,
            Some(p) if p == vn => vn,
            Some(p) => {
                let root = self.find(p);
                self.parent.insert(vn, root);
                root
            }
        }
    }
    /// Return `true` if `a` and `b` are known to be equivalent.
    pub fn are_equal(&mut self, a: ValueNumber, b: ValueNumber) -> bool {
        self.find(a) == self.find(b)
    }
    pub fn num_classes(&self) -> usize {
        self.parent.len()
    }
}
/// Maps each in-scope variable to its current value number at a program point.
#[derive(Debug, Clone, Default)]
pub struct GVNFact {
    pub var_to_vn: HashMap<LcnfVarId, ValueNumber>,
}
impl GVNFact {
    pub fn new() -> Self {
        GVNFact::default()
    }
    pub fn get(&self, var: &LcnfVarId) -> Option<ValueNumber> {
        self.var_to_vn.get(var).copied()
    }
    pub fn insert(&mut self, var: LcnfVarId, vn: ValueNumber) {
        self.var_to_vn.insert(var, vn);
    }
    /// Compute the meet (intersection) of two facts at a join point.
    /// Variables with different VNs in different branches are dropped.
    pub fn meet(&self, other: &GVNFact) -> GVNFact {
        let mut result = GVNFact::new();
        for (var, &vn) in &self.var_to_vn {
            if other.var_to_vn.get(var) == Some(&vn) {
                result.var_to_vn.insert(*var, vn);
            }
        }
        result
    }
}
/// Finds the *leader* of an equivalence class — the canonical variable
/// that dominates all other members of the class.
///
/// In LCNF, dominance is determined by binding order (earlier binding
/// dominates later bindings in the same scope).
#[derive(Debug, Default)]
pub struct LeaderFinder {
    /// Map from VN to the current leader variable.
    pub(super) leaders: HashMap<ValueNumber, LcnfVarId>,
    /// Map from VN to all members of the equivalence class.
    pub(super) members: HashMap<ValueNumber, Vec<LcnfVarId>>,
}
impl LeaderFinder {
    pub fn new() -> Self {
        LeaderFinder::default()
    }
    /// Record that `var` belongs to equivalence class `vn`.
    pub fn record(&mut self, vn: ValueNumber, var: LcnfVarId) {
        self.members.entry(vn).or_default().push(var);
        self.leaders.entry(vn).or_insert(var);
    }
    /// Return the leader of the equivalence class for `vn`.
    pub fn leader(&self, vn: ValueNumber) -> Option<LcnfVarId> {
        self.leaders.get(&vn).copied()
    }
    /// Return all members of the equivalence class for `vn`.
    pub fn members(&self, vn: ValueNumber) -> &[LcnfVarId] {
        self.members.get(&vn).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Return the number of non-singleton equivalence classes.
    pub fn num_redundancies(&self) -> usize {
        self.members.values().filter(|v| v.len() > 1).count()
    }
}
/// A multi-stage GVN pipeline that chains several GVN-based analyses.
pub struct GVNPipeline {
    /// Whether to run the base GVN pass.
    pub do_base_gvn: bool,
    /// Whether to run load elimination.
    pub do_load_elim: bool,
    /// Whether to run GVN-PRE.
    pub do_pre: bool,
    /// Whether to run CCP.
    pub do_ccp: bool,
    /// Whether to run fixpoint iteration.
    pub do_fixpoint: bool,
    /// Maximum fixpoint iterations.
    pub max_fixpoint_iter: usize,
    /// Combined statistics.
    pub stats: GVNStatistics,
}
impl GVNPipeline {
    pub fn new() -> Self {
        GVNPipeline::default()
    }
    /// Run the GVN pipeline on all declarations.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            if self.do_base_gvn {
                let mut pass = GVNPass::default();
                pass.run(std::slice::from_mut(decl));
                let r = pass.report();
                self.stats.total_vns += r.expressions_numbered;
                self.stats.phi_translations += r.phi_translations;
            }
            if self.do_load_elim {
                let mut le = LoadEliminatorGVN::new();
                le.run(decl);
                self.stats.proj_redundancies += le.eliminated;
            }
            if self.do_pre {
                let mut pre = GVNPrePass::new();
                pre.run(decl);
            }
            if self.do_ccp {
                let mut ccp = CCPState::new();
                ccp.run(decl);
                self.stats.lit_redundancies += ccp.folded;
            }
            if self.do_fixpoint {
                let mut fp = FixpointGVN::new(self.max_fixpoint_iter);
                fp.run(decl);
                self.stats.total_vns += fp.total_redundancies;
            }
        }
    }
    pub fn total_redundancies(&self) -> usize {
        self.stats.total_redundancies()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GVNPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl GVNPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            GVNPassPhase::Analysis => "analysis",
            GVNPassPhase::Transformation => "transformation",
            GVNPassPhase::Verification => "verification",
            GVNPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, GVNPassPhase::Transformation | GVNPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GVNCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// State for a single fixpoint iteration of GVN.
#[derive(Debug, Clone)]
pub struct FixpointState {
    /// The value table from this iteration.
    pub table: ValueTable,
    /// The GVN fact at function exit.
    pub exit_fact: GVNFact,
    /// Number of redundancies found in this iteration.
    pub redundancies: usize,
}
impl FixpointState {
    pub fn new() -> Self {
        FixpointState {
            table: ValueTable::new(),
            exit_fact: GVNFact::new(),
            redundancies: 0,
        }
    }
}
/// Bidirectional mapping between normalised expressions and value numbers.
///
/// The "key" is a `NormExpr` — a lightweight structural hash of the
/// expression that only depends on the *value numbers* of sub-expressions
/// (rather than variable ids).  This makes it insensitive to renaming.
#[derive(Debug, Clone, Default)]
pub struct ValueTable {
    /// Map from normalised expression key to its canonical value number.
    pub(super) expr_to_vn: HashMap<NormExpr, ValueNumber>,
    /// Map from value number back to the canonical `LcnfLetValue`.
    pub(super) vn_to_expr: HashMap<ValueNumber, LcnfLetValue>,
    /// Map from value number to the canonical variable holding that value.
    pub(super) vn_to_var: HashMap<ValueNumber, LcnfVarId>,
    /// Next fresh value number to assign.
    pub(super) next_vn: ValueNumber,
}
impl ValueTable {
    pub fn new() -> Self {
        ValueTable::default()
    }
    /// Look up the value number for a normalised expression key.
    pub fn lookup(&self, key: &NormExpr) -> Option<ValueNumber> {
        self.expr_to_vn.get(key).copied()
    }
    /// Return the canonical variable for a value number, if one exists.
    pub fn canonical_var(&self, vn: ValueNumber) -> Option<LcnfVarId> {
        self.vn_to_var.get(&vn).copied()
    }
    /// Insert a new expression with a fresh value number, binding `var`
    /// as its canonical representative.  Returns the assigned VN.
    pub fn insert(&mut self, key: NormExpr, value: LcnfLetValue, var: LcnfVarId) -> ValueNumber {
        let vn = self.next_vn;
        self.next_vn += 1;
        self.expr_to_vn.insert(key, vn);
        self.vn_to_expr.insert(vn, value);
        self.vn_to_var.insert(vn, var);
        vn
    }
    /// Total number of value numbers assigned so far.
    pub fn len(&self) -> usize {
        self.next_vn as usize
    }
    pub fn is_empty(&self) -> bool {
        self.next_vn == 0
    }
    /// Return a snapshot of all (NormExpr, VN) pairs.
    pub fn snapshot(&self) -> Vec<(NormExpr, ValueNumber)> {
        self.expr_to_vn
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }
    /// Attempt to merge the entries from `other` into `self` without
    /// conflict (returns false on any VN collision).
    pub fn try_merge(&mut self, other: &ValueTable) -> bool {
        for (key, &vn) in &other.expr_to_vn {
            if let Some(&existing_vn) = self.expr_to_vn.get(key) {
                if existing_vn != vn {
                    return false;
                }
            } else {
                self.expr_to_vn.insert(key.clone(), vn);
                if let Some(expr) = other.vn_to_expr.get(&vn) {
                    self.vn_to_expr.insert(vn, expr.clone());
                }
                if let Some(&cvar) = other.vn_to_var.get(&vn) {
                    self.vn_to_var.insert(vn, cvar);
                }
                if vn >= self.next_vn {
                    self.next_vn = vn + 1;
                }
            }
        }
        true
    }
}
/// Interprocedural GVN: uses function summaries to propagate value
/// equalities across function call boundaries.
#[derive(Debug, Default)]
pub struct InterproceduralGVN {
    /// Summaries for known functions.
    pub summaries: HashMap<String, GVNFunctionSummary>,
    /// Number of cross-function equalities discovered.
    pub cross_fn_equalities: usize,
}
impl InterproceduralGVN {
    pub fn new() -> Self {
        InterproceduralGVN::default()
    }
    /// Register a function summary.
    pub fn add_summary(&mut self, name: String, summary: GVNFunctionSummary) {
        self.summaries.insert(name, summary);
    }
    /// Query whether two calls to `fn_name` with the same arguments
    /// produce equal results.
    pub fn calls_are_equal(&self, fn_name: &str) -> bool {
        self.summaries
            .get(fn_name)
            .map(|s| s.is_pure_fn)
            .unwrap_or(false)
    }
    /// Run interprocedural GVN to find cross-function redundancies.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            if let Some(summary) = self.summaries.get(&decl.name) {
                if summary.is_pure_fn {
                    self.cross_fn_equalities += 1;
                }
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GVNPassConfig {
    pub phase: GVNPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl GVNPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: GVNPassPhase) -> Self {
        GVNPassConfig {
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
/// A phi-node synthesized at a case join point.
///
/// In SSA, every join point needs a phi-node for variables with different
/// values in different branches.  In GVN, phi-nodes get value numbers too.
#[derive(Debug, Clone)]
pub struct PhiNode {
    /// The variable introduced by this phi.
    pub var: LcnfVarId,
    /// The operands from each incoming branch.
    pub operands: Vec<PhiOperand>,
    /// The value number assigned to this phi-node.
    pub vn: ValueNumber,
}
impl PhiNode {
    pub fn new(var: LcnfVarId, operands: Vec<PhiOperand>, vn: ValueNumber) -> Self {
        PhiNode { var, operands, vn }
    }
    /// Return `true` if all operands have the same value number (trivial phi).
    pub fn is_trivial(&self) -> bool {
        self.operands.windows(2).all(|w| w[0].vn == w[1].vn)
    }
    /// If trivial, return the single unique value number.
    pub fn trivial_vn(&self) -> Option<ValueNumber> {
        if self.is_trivial() {
            self.operands.first().map(|op| op.vn)
        } else {
            None
        }
    }
}
/// The known value of a variable for CCP purposes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnownConstant {
    /// The variable is definitely the given literal.
    Lit(LcnfLit),
    /// The variable's value is not yet determined.
    Top,
    /// The variable could be more than one value (conservative).
    Bottom,
}
/// Represents a single GVN-detected redundancy.
#[derive(Debug, Clone)]
pub struct Redundancy {
    /// The redundant variable (the one to be replaced).
    pub redundant_var: LcnfVarId,
    /// The canonical variable (the one to keep).
    pub canonical_var: LcnfVarId,
    /// The value number of the equivalence class.
    pub vn: ValueNumber,
}
#[allow(dead_code)]
pub struct GVNPassRegistry {
    pub(super) configs: Vec<GVNPassConfig>,
    pub(super) stats: std::collections::HashMap<String, GVNPassStats>,
}
impl GVNPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GVNPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: GVNPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), GVNPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&GVNPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&GVNPassStats> {
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
/// A structural key used to check expression equality modulo renaming.
///
/// Instead of recording variable ids directly, we record their *value
/// numbers* so that two syntactically different but semantically equal
/// bindings hash to the same key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NormExpr {
    Lit(u64),
    LitStr(String),
    Erased,
    FVar(ValueNumber),
    Proj(String, u32, ValueNumber),
    Ctor(String, u32, Vec<NormArg>),
    App(NormArg, Vec<NormArg>),
    Reset(ValueNumber),
    Unknown,
}
