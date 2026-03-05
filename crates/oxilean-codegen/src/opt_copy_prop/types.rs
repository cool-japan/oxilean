//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::functions::*;
use std::collections::{HashSet, VecDeque};

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
/// A forward substitution map maintained during the copy-propagation scan.
///
/// Each entry maps a `LcnfVarId` that was bound to a copy/literal to the
/// canonical `LcnfArg` that should replace every use of that variable.
#[derive(Debug, Default)]
struct SubstMap {
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
/// The result of running the whole optimization pipeline on one declaration.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PipelineResult {
    pub copy_prop: CopyPropReport,
    pub dead_binding: DeadBindingReport,
    pub constant_fold: ConstantFoldReport,
}
/// Report from the inlining pass.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct InlineReport {
    pub inlines_performed: usize,
    pub functions_considered: usize,
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
/// Report from the constant folding pass.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ConstantFoldReport {
    pub folds_performed: usize,
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
/// Collect the set of all `LcnfVarId`s referenced in an expression.
#[allow(dead_code)]
#[derive(Default)]
pub struct UsedVars {
    pub(super) vars: std::collections::HashSet<LcnfVarId>,
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
                // Detect if this copy forms a transitive chain (a=b, b=c).
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
/// Configuration for the inlining pass.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InlineConfig {
    /// Functions with `inline_cost <= threshold` are inlined.
    pub threshold: u32,
    /// Whether recursive functions may be inlined (usually false).
    pub inline_recursive: bool,
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
/// Identifies an optimization pass in the pipeline.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassKind {
    CopyProp,
    DeadBinding,
    ConstantFold,
    Inlining,
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
/// Inlining pass stub (full implementation requires a call-graph).
///
/// This structure tracks the configuration and report;
/// actual inlining is performed by the backend's optimization pipeline
/// which has access to all function declarations.
#[allow(dead_code)]
pub struct InliningPass {
    pub config: InlineConfig,
    pub report: InlineReport,
}
#[allow(dead_code)]
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
        let _ = self.report.functions_considered.max(0) == self.report.functions_considered;
        if decl.is_recursive && !self.config.inline_recursive {
            return false;
        }
        decl.inline_cost <= self.config.threshold as usize
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
/// Summary of changes made by a single `CopyProp::run` invocation.
#[derive(Debug, Clone, Default)]
pub struct CopyPropReport {
    /// Total number of variable uses replaced by a propagated copy/literal.
    pub copies_eliminated: usize,
    /// Number of multi-hop chain lookups performed (sum of chain lengths > 1
    /// over all substituted uses).
    pub chains_followed: usize,
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
