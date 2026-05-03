//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;
use crate::lcnf::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use super::types::{
    ConstantFoldConfig, ConstantFoldReport, CopyProp, DeadBindingReport, InterferenceEdge,
};

/// Combines global value numbering (GVN) with copy propagation.
///
/// Assigns value numbers to expressions; when two bindings have the same
/// value number (i.e., compute the same value), they are aliases of each
/// other and can be copy-propagated freely.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct ValueNumberingCopyProp {
    /// Map from canonical expression key → representative variable ID.
    pub value_table: std::collections::HashMap<String, LcnfVarId>,
    /// Substitution map built by value numbering.
    pub subst: std::collections::HashMap<LcnfVarId, LcnfArg>,
    /// Number of bindings eliminated by value numbering.
    pub eliminated: usize,
}
#[allow(dead_code)]
impl ValueNumberingCopyProp {
    /// Creates a new value numbering copy propagation pass.
    pub fn new() -> Self {
        ValueNumberingCopyProp::default()
    }
    /// Computes a canonical string key for a `LcnfLetValue`.
    ///
    /// Two values that compute the same result should produce the same key.
    pub fn value_key(value: &LcnfLetValue) -> Option<String> {
        match value {
            LcnfLetValue::App(fun, args) => {
                let fun_str = Self::arg_key(fun);
                let args_str: Vec<String> = args.iter().map(Self::arg_key).collect();
                Some(format!("app({},{})", fun_str, args_str.join(",")))
            }
            LcnfLetValue::FVar(arg) => Some(format!("fvar(v{})", arg.0)),
            LcnfLetValue::Lit(lit) => Some(format!("lit({:?})", lit)),
            _ => None,
        }
    }
    pub(super) fn arg_key(arg: &LcnfArg) -> String {
        match arg {
            LcnfArg::Var(id) => format!("v{}", id.0),
            LcnfArg::Lit(lit) => format!("l{:?}", lit),
            LcnfArg::Erased => "erased".to_string(),
            LcnfArg::Type(ty) => format!("type({:?})", ty),
        }
    }
    /// Runs value numbering copy propagation on a function declaration.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        decl.body = self.process_expr(std::mem::replace(
            &mut decl.body,
            LcnfExpr::Return(LcnfArg::Erased),
        ));
    }
    pub(super) fn process_expr(&mut self, expr: LcnfExpr) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let substed_value = self.subst_let_value(value);
                if let Some(key) = Self::value_key(&substed_value) {
                    if let Some(&existing_id) = self.value_table.get(&key) {
                        self.subst.insert(id, LcnfArg::Var(existing_id));
                        self.eliminated += 1;
                        return self.process_expr(*body);
                    } else {
                        self.value_table.insert(key, id);
                    }
                }
                let new_body = self.process_expr(*body);
                LcnfExpr::Let {
                    id,
                    name,
                    ty,
                    value: substed_value,
                    body: Box::new(new_body),
                }
            }
            LcnfExpr::Return(arg) => LcnfExpr::Return(self.subst_arg(arg)),
            LcnfExpr::TailCall(fun, args) => {
                let new_fun = self.subst_arg(fun);
                let new_args = args.into_iter().map(|a| self.subst_arg(a)).collect();
                LcnfExpr::TailCall(new_fun, new_args)
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                let new_scrutinee = match self.subst.get(&scrutinee) {
                    Some(LcnfArg::Var(vid)) => *vid,
                    _ => scrutinee,
                };
                let saved_table = self.value_table.clone();
                let new_alts: Vec<crate::lcnf::LcnfAlt> = alts
                    .into_iter()
                    .map(|alt| {
                        self.value_table = saved_table.clone();
                        let new_body = self.process_expr(alt.body);
                        crate::lcnf::LcnfAlt {
                            ctor_name: alt.ctor_name,
                            ctor_tag: alt.ctor_tag,
                            params: alt.params,
                            body: new_body,
                        }
                    })
                    .collect();
                self.value_table = saved_table;
                let new_default = default.map(|d| Box::new(self.process_expr(*d)));
                LcnfExpr::Case {
                    scrutinee: new_scrutinee,
                    scrutinee_ty: LcnfType::Erased,
                    alts: new_alts,
                    default: new_default,
                }
            }
            other => other,
        }
    }
    pub(super) fn subst_arg(&self, arg: LcnfArg) -> LcnfArg {
        match arg {
            LcnfArg::Var(id) => self.subst.get(&id).cloned().unwrap_or(LcnfArg::Var(id)),
            other => other,
        }
    }
    pub(super) fn subst_let_value(&self, value: LcnfLetValue) -> LcnfLetValue {
        match value {
            LcnfLetValue::FVar(a) => match self.subst.get(&a) {
                Some(LcnfArg::Var(vid)) => LcnfLetValue::FVar(*vid),
                Some(other) => LcnfLetValue::App(other.clone(), vec![]),
                None => LcnfLetValue::FVar(a),
            },
            LcnfLetValue::App(fun, args) => {
                let new_fun = self.subst_arg(fun);
                let new_args = args.into_iter().map(|a| self.subst_arg(a)).collect();
                LcnfLetValue::App(new_fun, new_args)
            }
            other => other,
        }
    }
    /// Returns a report of value numbering copy propagation.
    pub fn report(&self) -> String {
        format!(
            "ValueNumberingCopyProp: {} bindings eliminated, {} value numbers",
            self.eliminated,
            self.value_table.len()
        )
    }
}
/// A forward substitution map maintained during the copy-propagation scan.
///
/// Each entry maps a `LcnfVarId` that was bound to a copy/literal to the
/// canonical `LcnfArg` that should replace every use of that variable.
#[derive(Debug, Default)]
pub(crate) struct SubstMap {
    pub(super) inner: HashMap<LcnfVarId, LcnfArg>,
}
impl SubstMap {
    pub(super) fn new() -> Self {
        Self::default()
    }
    /// Insert a direct (non-transitive) mapping.
    pub(super) fn insert(&mut self, from: LcnfVarId, to: LcnfArg) {
        self.inner.insert(from, to);
    }
    /// Look up `id` following transitive chains up to `max_depth` hops.
    /// Returns `(resolved_arg, hops)`.  `hops == 0` means no substitution.
    pub(super) fn lookup(&self, id: LcnfVarId, max_depth: usize) -> (LcnfArg, usize) {
        let mut current = LcnfArg::Var(id);
        let mut hops = 0usize;
        loop {
            if hops >= max_depth {
                break;
            }
            let next_id = match &current {
                LcnfArg::Var(v) => *v,
                _ => break,
            };
            match self.inner.get(&next_id) {
                Some(mapped) => {
                    current = mapped.clone();
                    hops += 1;
                }
                None => break,
            }
        }
        (current, hops)
    }
    /// Apply the substitution to a single `LcnfArg`.
    pub(super) fn apply_arg(&self, arg: LcnfArg, max_depth: usize) -> (LcnfArg, usize) {
        match &arg {
            LcnfArg::Var(id) => self.lookup(*id, max_depth),
            _ => (arg, 0),
        }
    }
}
/// Aggregate statistics for the entire copy propagation pipeline.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct CopyPropStats {
    /// Total copies eliminated by forward propagation.
    pub copies_eliminated: usize,
    /// Total copies eliminated by value numbering.
    pub vn_eliminated: usize,
    /// Total dead copy bindings removed.
    pub dead_bindings_removed: usize,
    /// Total phi collapses (conditional copy prop).
    pub phi_collapses: usize,
    /// Total move operations performed.
    pub moves_performed: usize,
    /// Total coalescing candidates identified.
    pub coalescing_candidates: usize,
    /// Total chain collapses.
    pub chain_collapses: usize,
    /// Maximum chain depth encountered.
    pub max_chain_depth: usize,
}
#[allow(dead_code)]
impl CopyPropStats {
    /// Creates zeroed statistics.
    pub fn new() -> Self {
        CopyPropStats::default()
    }
    /// Returns the total number of optimizations performed.
    pub fn total_optimizations(&self) -> usize {
        self.copies_eliminated
            + self.vn_eliminated
            + self.dead_bindings_removed
            + self.phi_collapses
            + self.moves_performed
    }
    /// Returns a human-readable report.
    pub fn report(&self) -> String {
        format!(
            concat!(
                "=== Copy Propagation Statistics ===\n",
                "  Copies eliminated (forward)  : {}\n",
                "  Copies eliminated (VN)        : {}\n",
                "  Dead bindings removed         : {}\n",
                "  Phi collapses                 : {}\n",
                "  Moves performed               : {}\n",
                "  Coalescing candidates         : {}\n",
                "  Chain collapses               : {}\n",
                "  Max chain depth               : {}\n",
                "  Total optimizations           : {}\n",
            ),
            self.copies_eliminated,
            self.vn_eliminated,
            self.dead_bindings_removed,
            self.phi_collapses,
            self.moves_performed,
            self.coalescing_candidates,
            self.chain_collapses,
            self.max_chain_depth,
            self.total_optimizations()
        )
    }
}
/// Lightweight constant-folding pass over LCNF.
///
/// Folds expressions of the form:
///   `let r = Nat.add(lit_a, lit_b) in ...`  →  `let r = (a+b) in ...`
#[allow(dead_code)]
pub struct ConstantFolder {
    pub(super) config: ConstantFoldConfig,
    pub(super) report: ConstantFoldReport,
}
#[allow(dead_code)]
impl ConstantFolder {
    pub fn new(config: ConstantFoldConfig) -> Self {
        ConstantFolder {
            config,
            report: ConstantFoldReport::default(),
        }
    }
    pub fn default_pass() -> Self {
        Self::new(ConstantFoldConfig::default())
    }
    pub fn report(&self) -> &ConstantFoldReport {
        &self.report
    }
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        let old_body = std::mem::replace(&mut decl.body, LcnfExpr::Unreachable);
        decl.body = self.fold_expr(old_body);
    }
    pub(super) fn fold_expr(&mut self, expr: LcnfExpr) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let value2 = self.fold_value(value);
                let body2 = self.fold_expr(*body);
                LcnfExpr::Let {
                    id,
                    name,
                    ty,
                    value: value2,
                    body: Box::new(body2),
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                let alts2 = alts
                    .into_iter()
                    .map(|alt| LcnfAlt {
                        ctor_name: alt.ctor_name,
                        ctor_tag: alt.ctor_tag,
                        params: alt.params,
                        body: self.fold_expr(alt.body),
                    })
                    .collect();
                let default2 = default.map(|d| Box::new(self.fold_expr(*d)));
                LcnfExpr::Case {
                    scrutinee,
                    scrutinee_ty,
                    alts: alts2,
                    default: default2,
                }
            }
            other => other,
        }
    }
    pub(super) fn fold_value(&mut self, value: LcnfLetValue) -> LcnfLetValue {
        if let LcnfLetValue::App(ref callee_id, ref args) = value {
            if args.len() == 2 {
                if let (LcnfArg::Lit(LcnfLit::Nat(a)), LcnfArg::Lit(LcnfLit::Nat(b))) =
                    (&args[0], &args[1])
                {
                    let _callee = callee_id;
                    if self.config.fold_nat_arith {
                        let sum = a.saturating_add(*b);
                        if sum <= self.config.max_nat_value {
                            self.report.folds_performed += 1;
                            return LcnfLetValue::Lit(LcnfLit::Nat(sum));
                        }
                    }
                }
            }
        }
        value
    }
}
/// Summary of changes made by a single `CopyProp::run` invocation.
#[derive(Debug, Clone, Default)]
pub struct CopyPropReport {
    /// Total number of variable uses replaced by a propagated copy/literal.
    pub copies_eliminated: usize,
    /// Number of multi-hop chain lookups performed (sum of chain lengths > 1
    /// over all substituted uses).
    pub chains_followed: usize,
}
/// Collect the set of all `LcnfVarId`s referenced in an expression.
#[allow(dead_code)]
#[derive(Default)]
pub struct UsedVars {
    pub(crate) vars: std::collections::HashSet<LcnfVarId>,
}
/// An interference graph for register allocation.
///
/// Two variables interfere if they are simultaneously live at any program
/// point. Interfering variables cannot be assigned to the same register.
///
/// Copy-related variables (connected by copy edges) that do NOT interfere
/// are candidates for register coalescing (assigning them the same register).
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct InterferenceGraph {
    /// Set of interference edges.
    pub interference: std::collections::HashSet<InterferenceEdge>,
    /// Set of copy (preference) edges.
    pub copy_edges: std::collections::HashSet<InterferenceEdge>,
    /// All variables in the graph.
    pub vars: std::collections::HashSet<LcnfVarId>,
}
#[allow(dead_code)]
impl InterferenceGraph {
    /// Creates an empty interference graph.
    pub fn new() -> Self {
        InterferenceGraph::default()
    }
    /// Adds an interference edge between `a` and `b`.
    pub fn add_interference(&mut self, a: LcnfVarId, b: LcnfVarId) {
        if a != b {
            self.vars.insert(a);
            self.vars.insert(b);
            self.interference.insert(InterferenceEdge::new(a, b));
        }
    }
    /// Adds a copy (preference) edge between `a` and `b`.
    pub fn add_copy_edge(&mut self, a: LcnfVarId, b: LcnfVarId) {
        if a != b {
            self.vars.insert(a);
            self.vars.insert(b);
            self.copy_edges.insert(InterferenceEdge::new(a, b));
        }
    }
    /// Returns `true` if `a` and `b` interfere.
    pub fn interfere(&self, a: LcnfVarId, b: LcnfVarId) -> bool {
        self.interference.contains(&InterferenceEdge::new(a, b))
    }
    /// Returns `true` if `a` and `b` have a copy preference edge.
    pub fn have_copy_edge(&self, a: LcnfVarId, b: LcnfVarId) -> bool {
        self.copy_edges.contains(&InterferenceEdge::new(a, b))
    }
    /// Returns all copy edges that do NOT interfere (coalescing candidates).
    pub fn coalescing_candidates(&self) -> Vec<&InterferenceEdge> {
        self.copy_edges
            .iter()
            .filter(|e| !self.interference.contains(e))
            .collect()
    }
    /// Returns the degree of variable `v` in the interference graph.
    pub fn interference_degree(&self, v: LcnfVarId) -> usize {
        self.interference
            .iter()
            .filter(|e| e.u == v || e.v == v)
            .count()
    }
    /// Returns the number of variables in the graph.
    pub fn num_vars(&self) -> usize {
        self.vars.len()
    }
    /// Returns the number of interference edges.
    pub fn num_interference_edges(&self) -> usize {
        self.interference.len()
    }
    /// Returns the number of copy preference edges.
    pub fn num_copy_edges(&self) -> usize {
        self.copy_edges.len()
    }
    /// Returns a summary report.
    pub fn report(&self) -> String {
        format!(
            "InterferenceGraph: {} vars, {} interference edges, {} copy edges, {} coalesc. candidates",
            self.num_vars(), self.num_interference_edges(), self.num_copy_edges(), self
            .coalescing_candidates().len()
        )
    }
}
/// Extends copy propagation with move semantics.
///
/// When a variable `x` is a copy of `y`, and `x` is used exactly once,
/// we can "move" `y` to the use site (instead of copying). This is safe
/// when `y` has no other uses after the move point.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct MoveSemanticsCopyProp {
    /// Number of moves performed.
    pub moves_performed: usize,
    /// Number of copies remaining (not moved).
    pub copies_remaining: usize,
    /// Map from variable to its use count.
    pub(super) use_counts: std::collections::HashMap<LcnfVarId, usize>,
}
#[allow(dead_code)]
impl MoveSemanticsCopyProp {
    /// Creates a new move-semantics copy propagation pass.
    pub fn new() -> Self {
        MoveSemanticsCopyProp::default()
    }
    /// Counts the number of uses of each variable in the expression.
    pub fn count_uses(&mut self, expr: &LcnfExpr) {
        self.use_counts.clear();
        self.count_uses_expr(expr);
    }
    pub(super) fn count_uses_expr(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Return(arg) => self.count_uses_arg(arg),
            LcnfExpr::Let { value, body, .. } => {
                self.count_uses_let_value(value);
                self.count_uses_expr(body);
            }
            LcnfExpr::TailCall(fun, args) => {
                self.count_uses_arg(fun);
                for a in args {
                    self.count_uses_arg(a);
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                *self.use_counts.entry(*scrutinee).or_insert(0) += 1;
                for alt in alts {
                    self.count_uses_expr(&alt.body);
                }
                if let Some(d) = default {
                    self.count_uses_expr(d);
                }
            }
            _ => {}
        }
    }
    pub(super) fn count_uses_arg(&mut self, arg: &LcnfArg) {
        if let LcnfArg::Var(id) = arg {
            *self.use_counts.entry(*id).or_insert(0) += 1;
        }
    }
    pub(super) fn count_uses_let_value(&mut self, value: &LcnfLetValue) {
        match value {
            LcnfLetValue::FVar(a) => {
                *self.use_counts.entry(*a).or_insert(0) += 1;
            }
            LcnfLetValue::App(fun, args) => {
                self.count_uses_arg(fun);
                for a in args {
                    self.count_uses_arg(a);
                }
            }
            _ => {}
        }
    }
    /// Returns the use count of variable `id`.
    pub fn use_count(&self, id: LcnfVarId) -> usize {
        self.use_counts.get(&id).copied().unwrap_or(0)
    }
    /// Returns `true` if variable `id` is used exactly once.
    pub fn is_single_use(&self, id: LcnfVarId) -> bool {
        self.use_count(id) == 1
    }
    /// Returns `true` if variable `id` is unused.
    pub fn is_unused(&self, id: LcnfVarId) -> bool {
        self.use_count(id) == 0
    }
    /// Runs move-semantics copy propagation on a function declaration.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        self.count_uses(&decl.body);
        let (new_body, _) = self.process_expr(
            std::mem::replace(&mut decl.body, LcnfExpr::Return(LcnfArg::Erased)),
            &std::collections::HashMap::new(),
        );
        decl.body = new_body;
    }
    pub(super) fn process_expr(
        &mut self,
        expr: LcnfExpr,
        subst: &std::collections::HashMap<LcnfVarId, LcnfArg>,
    ) -> (LcnfExpr, bool) {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                if let LcnfLetValue::FVar(ref src) = value {
                    if self.is_single_use(id) {
                        self.moves_performed += 1;
                        let mut new_subst = subst.clone();
                        new_subst.insert(id, LcnfArg::Var(*src));
                        let (new_body, _) = self.process_expr(*body, &new_subst);
                        return (new_body, true);
                    } else {
                        self.copies_remaining += 1;
                    }
                }
                let new_value = self.subst_let_value(value, subst);
                let (new_body, changed) = self.process_expr(*body, subst);
                (
                    LcnfExpr::Let {
                        id,
                        name,
                        ty,
                        value: new_value,
                        body: Box::new(new_body),
                    },
                    changed,
                )
            }
            LcnfExpr::Return(arg) => {
                let new_arg = self.subst_arg(arg, subst);
                (LcnfExpr::Return(new_arg), false)
            }
            LcnfExpr::TailCall(fun, args) => {
                let new_fun = self.subst_arg(fun, subst);
                let new_args = args.into_iter().map(|a| self.subst_arg(a, subst)).collect();
                (LcnfExpr::TailCall(new_fun, new_args), false)
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                let new_scrutinee = match subst.get(&scrutinee) {
                    Some(LcnfArg::Var(vid)) => *vid,
                    _ => scrutinee,
                };
                let new_alts = alts
                    .into_iter()
                    .map(|alt| {
                        let (new_body, _) = self.process_expr(alt.body, subst);
                        crate::lcnf::LcnfAlt {
                            ctor_name: alt.ctor_name,
                            ctor_tag: alt.ctor_tag,
                            params: alt.params,
                            body: new_body,
                        }
                    })
                    .collect();
                let new_default = default.map(|d| {
                    let (nb, _) = self.process_expr(*d, subst);
                    Box::new(nb)
                });
                (
                    LcnfExpr::Case {
                        scrutinee: new_scrutinee,
                        scrutinee_ty: LcnfType::Erased,
                        alts: new_alts,
                        default: new_default,
                    },
                    false,
                )
            }
            other => (other, false),
        }
    }
    pub(super) fn subst_arg(
        &self,
        arg: LcnfArg,
        subst: &std::collections::HashMap<LcnfVarId, LcnfArg>,
    ) -> LcnfArg {
        match arg {
            LcnfArg::Var(id) => subst.get(&id).cloned().unwrap_or(LcnfArg::Var(id)),
            other => other,
        }
    }
    pub(super) fn subst_let_value(
        &self,
        value: LcnfLetValue,
        subst: &std::collections::HashMap<LcnfVarId, LcnfArg>,
    ) -> LcnfLetValue {
        match value {
            LcnfLetValue::FVar(a) => match subst.get(&a) {
                Some(LcnfArg::Var(vid)) => LcnfLetValue::FVar(*vid),
                Some(other) => LcnfLetValue::App(other.clone(), vec![]),
                None => LcnfLetValue::FVar(a),
            },
            LcnfLetValue::App(fun, args) => {
                let new_fun = self.subst_arg(fun, subst);
                let new_args = args.into_iter().map(|a| self.subst_arg(a, subst)).collect();
                LcnfLetValue::App(new_fun, new_args)
            }
            other => other,
        }
    }
    /// Returns a report of move semantics copy propagation.
    pub fn report(&self) -> String {
        format!(
            "MoveSemanticsCopyProp: {} moves performed, {} copies remaining",
            self.moves_performed, self.copies_remaining
        )
    }
}
/// Collapses copy chains using a union-find (disjoint set) structure.
///
/// For a chain `a = b; b = c; c = d`, union-find discovers the root (`d`)
/// in nearly O(1) time with path compression, avoiding O(n) traversal.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CopyChainCollapser {
    /// Parent map for union-find. `parent[x] = x` for roots.
    pub(super) parent: std::collections::HashMap<LcnfVarId, LcnfArg>,
    /// Rank for union by rank.
    pub(super) rank: std::collections::HashMap<LcnfVarId, usize>,
    /// Number of chains collapsed.
    pub chains_collapsed: usize,
    /// Maximum chain depth encountered.
    pub max_chain_depth: usize,
}
#[allow(dead_code)]
impl CopyChainCollapser {
    /// Creates a new empty copy chain collapser.
    pub fn new() -> Self {
        CopyChainCollapser::default()
    }
    /// Registers that variable `id` is a copy of `src`.
    pub fn register_copy(&mut self, id: LcnfVarId, src: LcnfArg) {
        self.parent.insert(id, src);
        self.rank.entry(id).or_insert(0);
    }
    /// Finds the ultimate source of `id`, following copy chains.
    ///
    /// Uses path compression: re-links all nodes along the path directly to
    /// the root, flattening future lookups to O(α(n)).
    pub fn find_root(&mut self, id: LcnfVarId) -> LcnfArg {
        self.find_root_impl(id, 0)
    }
    pub(super) fn find_root_impl(&mut self, id: LcnfVarId, depth: usize) -> LcnfArg {
        if depth > self.max_chain_depth {
            self.max_chain_depth = depth;
        }
        let src = match self.parent.get(&id) {
            Some(s) => s.clone(),
            None => return LcnfArg::Var(id),
        };
        match src {
            LcnfArg::Var(next_id) if next_id != id => {
                let root = self.find_root_impl(next_id, depth + 1);
                self.parent.insert(id, root.clone());
                root
            }
            other => other,
        }
    }
    /// Applies chain collapsing to a function declaration.
    ///
    /// After scanning for copy pairs (`let x = y`), rewrites the body to use
    /// the root of each chain.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        self.collect_copies(&decl.body);
        decl.body = self.rewrite_expr(std::mem::replace(
            &mut decl.body,
            LcnfExpr::Return(LcnfArg::Erased),
        ));
    }
    pub(super) fn collect_copies(&mut self, expr: &LcnfExpr) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                if let LcnfLetValue::FVar(src) = value {
                    self.register_copy(*id, LcnfArg::Var(*src));
                    self.chains_collapsed += 1;
                }
                self.collect_copies(body);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.collect_copies(&alt.body);
                }
                if let Some(d) = default {
                    self.collect_copies(d);
                }
            }
            _ => {}
        }
    }
    pub(super) fn rewrite_arg(&mut self, arg: LcnfArg) -> LcnfArg {
        match arg {
            LcnfArg::Var(id) => self.find_root(id),
            other => other,
        }
    }
    pub(super) fn rewrite_expr(&mut self, expr: LcnfExpr) -> LcnfExpr {
        match expr {
            LcnfExpr::Return(arg) => LcnfExpr::Return(self.rewrite_arg(arg)),
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let new_value = match value {
                    LcnfLetValue::FVar(a) => {
                        let rewritten = self.rewrite_arg(LcnfArg::Var(a));
                        match rewritten {
                            LcnfArg::Var(vid) => LcnfLetValue::FVar(vid),
                            other_arg => LcnfLetValue::App(other_arg, vec![]),
                        }
                    }
                    LcnfLetValue::App(f, args) => {
                        let new_f = self.rewrite_arg(f);
                        let new_args = args.into_iter().map(|a| self.rewrite_arg(a)).collect();
                        LcnfLetValue::App(new_f, new_args)
                    }
                    other => other,
                };
                let new_body = self.rewrite_expr(*body);
                LcnfExpr::Let {
                    id,
                    name,
                    ty,
                    value: new_value,
                    body: Box::new(new_body),
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
                        body: self.rewrite_expr(alt.body),
                    })
                    .collect();
                let new_default = default.map(|d| Box::new(self.rewrite_expr(*d)));
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
    /// Returns a report of chain collapsing results.
    pub fn report(&self) -> String {
        format!(
            "CopyChainCollapser: {} chains collapsed, max depth {}",
            self.chains_collapsed, self.max_chain_depth
        )
    }
}
/// Configuration for the inlining pass.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InlineConfig {
    /// Functions with `inline_cost <= threshold` are inlined.
    pub threshold: u32,
    /// Whether recursive functions may be inlined (usually false).
    pub inline_recursive: bool,
}
/// The result of running the whole optimization pipeline on one declaration.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PipelineResult {
    pub copy_prop: CopyPropReport,
    pub dead_binding: DeadBindingReport,
    pub constant_fold: ConstantFoldReport,
}
/// Conditional copy propagation handles the case where a phi-node (or OxiLean
/// case expression) copies the same value in all arms.
///
/// If `match x { A => y, B => y, C => y }` produces `z`, then `z` is a copy
/// of `y` and can be propagated everywhere `z` is used.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ConditionalCopyProp {
    /// Number of case expressions simplified.
    pub cases_simplified: usize,
    /// Number of phi-equivalent nodes collapsed.
    pub phi_collapses: usize,
}
#[allow(dead_code)]
impl ConditionalCopyProp {
    /// Creates a new conditional copy propagation pass.
    pub fn new() -> Self {
        ConditionalCopyProp::default()
    }
    /// Checks if all arms of a case expression return the same `LcnfArg`.
    ///
    /// If all arms of `alts` produce the same value (same variable or literal),
    /// returns `Some(that_value)`. Otherwise returns `None`.
    pub fn uniform_value(alts: &[crate::lcnf::LcnfAlt]) -> Option<LcnfArg> {
        if alts.is_empty() {
            return None;
        }
        let values: Vec<Option<LcnfArg>> = alts.iter().map(Self::arm_return_value).collect();
        let first = values.first()?.as_ref()?;
        if values.iter().all(|v| v.as_ref() == Some(first)) {
            Some(first.clone())
        } else {
            None
        }
    }
    /// Extracts the return value from a case arm, if it is a simple `Return`.
    pub(super) fn arm_return_value(alt: &crate::lcnf::LcnfAlt) -> Option<LcnfArg> {
        match &alt.body {
            LcnfExpr::Return(arg) => Some(arg.clone()),
            _ => None,
        }
    }
    /// Runs the conditional copy propagation pass on a function declaration.
    ///
    /// Walks the expression tree looking for case expressions where all arms
    /// return the same value.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        decl.body = self.propagate_expr(std::mem::replace(
            &mut decl.body,
            LcnfExpr::Return(LcnfArg::Erased),
        ));
    }
    pub(super) fn propagate_expr(&mut self, expr: LcnfExpr) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let new_body = self.propagate_expr(*body);
                LcnfExpr::Let {
                    id,
                    name,
                    ty,
                    value,
                    body: Box::new(new_body),
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                if let Some(uniform) = Self::uniform_value(&alts) {
                    self.phi_collapses += 1;
                    self.cases_simplified += 1;
                    return LcnfExpr::Return(uniform);
                }
                let new_alts: Vec<crate::lcnf::LcnfAlt> = alts
                    .into_iter()
                    .map(|alt| crate::lcnf::LcnfAlt {
                        ctor_name: alt.ctor_name,
                        ctor_tag: alt.ctor_tag,
                        params: alt.params,
                        body: self.propagate_expr(alt.body),
                    })
                    .collect();
                let new_default = default.map(|d| Box::new(self.propagate_expr(*d)));
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
    /// Returns a report string.
    pub fn report(&self) -> String {
        format!(
            "ConditionalCopyProp: {} cases simplified, {} phi collapses",
            self.cases_simplified, self.phi_collapses
        )
    }
}
/// Filters copy propagation for variables that may be aliased.
///
/// If two variables `x` and `y` may alias the same memory location,
/// substituting `x` with `y` could be unsound. This filter tracks
/// aliasing information to prevent incorrect propagation.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct AliasingCopyFilter {
    /// Set of variable IDs that are known to be aliased (may-alias set).
    pub aliased_vars: std::collections::HashSet<LcnfVarId>,
    /// Number of copies skipped due to aliasing.
    pub copies_skipped: usize,
    /// Number of copies allowed (not aliased).
    pub copies_allowed: usize,
}
#[allow(dead_code)]
impl AliasingCopyFilter {
    /// Creates a new aliasing copy filter.
    pub fn new() -> Self {
        AliasingCopyFilter::default()
    }
    /// Marks variable `id` as potentially aliased.
    pub fn mark_aliased(&mut self, id: LcnfVarId) {
        self.aliased_vars.insert(id);
    }
    /// Returns `true` if variable `id` may be aliased.
    pub fn is_aliased(&self, id: LcnfVarId) -> bool {
        self.aliased_vars.contains(&id)
    }
    /// Checks whether propagating `src` (replacing uses of `dst`) is safe.
    ///
    /// Returns `false` if either variable is aliased.
    pub fn is_safe_to_propagate(&mut self, src: LcnfVarId, dst: LcnfVarId) -> bool {
        if self.is_aliased(src) || self.is_aliased(dst) {
            self.copies_skipped += 1;
            false
        } else {
            self.copies_allowed += 1;
            true
        }
    }
    /// Returns a report.
    pub fn report(&self) -> String {
        format!(
            "AliasingCopyFilter: {} skipped, {} allowed, {} aliased vars",
            self.copies_skipped,
            self.copies_allowed,
            self.aliased_vars.len()
        )
    }
}
