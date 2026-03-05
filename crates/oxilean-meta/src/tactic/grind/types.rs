//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Name};
use std::collections::{HashMap, HashSet, VecDeque};

/// A complete grind proof.
#[derive(Clone, Debug)]
pub struct GrindProof {
    /// The proof steps.
    pub steps: Vec<ProofStep>,
    /// The final proof term.
    pub term: Expr,
}
impl GrindProof {
    /// Create a new grind proof.
    fn new(steps: Vec<ProofStep>, term: Expr) -> Self {
        GrindProof { steps, term }
    }
}
/// The kind of relation in a Nat arithmetic constraint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NatRelKind {
    /// `lhs ≤ rhs`
    Le,
    /// `lhs < rhs`
    Lt,
    /// `lhs = rhs`
    Eq,
    /// `lhs ≥ rhs`
    Ge,
    /// `lhs > rhs`
    Gt,
}
/// Manages case splitting for the grind tactic.
///
/// When the congruence closure saturates without solving the goal,
/// the case splitter tries splitting on disjunctions (`P \/ Q` hypotheses)
/// or boolean terms. It supports backtracking on split failure.
pub struct CaseSplitter {
    /// Registered disjunctions.
    pub(super) disjunctions: Vec<DisjunctionCase>,
    /// Boolean terms that could be split on.
    pub(super) bool_terms: Vec<Expr>,
    /// Stack of CC snapshots for backtracking.
    pub(super) snapshots: Vec<CaseSplitSnapshot>,
    /// Index of next disjunction to try.
    pub(super) next_disj: usize,
    /// Total splits performed.
    pub(super) total_splits: usize,
}
impl CaseSplitter {
    /// Create a new case splitter.
    pub fn new() -> Self {
        CaseSplitter {
            disjunctions: Vec::new(),
            bool_terms: Vec::new(),
            snapshots: Vec::new(),
            next_disj: 0,
            total_splits: 0,
        }
    }
    /// Register a disjunction for potential splitting.
    pub fn add_disjunction(&mut self, hyp_name: Name, left: Expr, right: Expr) {
        self.disjunctions.push(DisjunctionCase {
            hyp_name,
            left,
            right,
            split: false,
        });
    }
    /// Register a boolean term for potential splitting.
    pub fn add_bool_term(&mut self, term: Expr) {
        self.bool_terms.push(term);
    }
    /// Try to perform a case split, returning true if one was made.
    pub fn try_split(&mut self, cc: &mut CongruenceClosure) -> bool {
        while self.next_disj < self.disjunctions.len() {
            let idx = self.next_disj;
            self.next_disj += 1;
            if self.disjunctions[idx].split {
                continue;
            }
            let snapshot = CaseSplitSnapshot {
                disj_idx: idx,
                branch: 0,
                cc_num_nodes: cc.num_nodes(),
                cc_merge_log_len: cc.merge_log.len(),
            };
            self.snapshots.push(snapshot);
            self.disjunctions[idx].split = true;
            let left = self.disjunctions[idx].left.clone();
            let left_node = cc.add_term(&left);
            let true_expr = Expr::Const(Name::str("True"), vec![]);
            if let Some(true_node) = cc.lookup_expr(&true_expr) {
                cc.merge_with_reason(left_node, true_node, MergeReason::Assertion);
            }
            self.total_splits += 1;
            return true;
        }
        if let Some(snapshot) = self.snapshots.pop() {
            if snapshot.branch == 0 {
                let idx = snapshot.disj_idx;
                let right = self.disjunctions[idx].right.clone();
                let right_node = cc.add_term(&right);
                let true_expr = Expr::Const(Name::str("True"), vec![]);
                if let Some(true_node) = cc.lookup_expr(&true_expr) {
                    cc.merge_with_reason(right_node, true_node, MergeReason::Assertion);
                }
                self.snapshots.push(CaseSplitSnapshot {
                    disj_idx: idx,
                    branch: 1,
                    cc_num_nodes: snapshot.cc_num_nodes,
                    cc_merge_log_len: snapshot.cc_merge_log_len,
                });
                self.total_splits += 1;
                return true;
            }
        }
        for term in &self.bool_terms {
            let node = cc.add_term(term);
            let true_expr = Expr::Const(Name::str("True"), vec![]);
            let false_expr = Expr::Const(Name::str("False"), vec![]);
            let is_already_true = cc
                .lookup_expr(&true_expr)
                .is_some_and(|t| cc.are_equal(node, t));
            let is_already_false = cc
                .lookup_expr(&false_expr)
                .is_some_and(|f| cc.are_equal(node, f));
            if !is_already_true && !is_already_false {
                if let Some(true_node) = cc.lookup_expr(&true_expr) {
                    cc.merge_with_reason(node, true_node, MergeReason::Assertion);
                }
                self.total_splits += 1;
                return true;
            }
        }
        false
    }
    /// Number of available disjunctions.
    pub fn num_disjunctions(&self) -> usize {
        self.disjunctions.len()
    }
    /// Number of splits performed so far.
    pub fn num_splits(&self) -> usize {
        self.total_splits
    }
    /// Number of backtrack snapshots.
    pub fn num_snapshots(&self) -> usize {
        self.snapshots.len()
    }
}
/// Snapshot for backtracking after a failed case split.
#[derive(Clone, Debug)]
struct CaseSplitSnapshot {
    /// Which disjunction was split.
    pub(super) disj_idx: usize,
    /// Which branch we took (0 = left, 1 = right).
    pub(super) branch: u8,
    /// Number of nodes in CC at snapshot time.
    pub(super) cc_num_nodes: usize,
    /// Number of merge log entries at snapshot time.
    pub(super) cc_merge_log_len: usize,
}
/// Compiler that creates EPatterns from universally quantified expressions.
pub struct EMatchCompiler {
    /// Next pattern variable id.
    pub(super) next_var: u32,
    /// Mapping from bound variable index to pattern variable index.
    pub(super) bvar_to_pvar: HashMap<u32, u32>,
}
impl EMatchCompiler {
    /// Create a new compiler.
    pub fn new() -> Self {
        EMatchCompiler {
            next_var: 0,
            bvar_to_pvar: HashMap::new(),
        }
    }
    /// Compile a universally quantified expression into a pattern.
    ///
    /// Given `forall x1 ... xn, body`, strips the forall binders and
    /// compiles `body` into an EPattern where each `xi` becomes a pattern variable.
    pub fn compile(&mut self, expr: &Expr) -> EPattern {
        self.next_var = 0;
        self.bvar_to_pvar.clear();
        let (num_binders, body) = strip_forall(expr);
        for i in 0..num_binders {
            let pvar = self.fresh_var();
            self.bvar_to_pvar.insert(i as u32, pvar);
        }
        let root = self.compile_node(&body, 0);
        EPattern {
            root,
            num_vars: self.next_var,
            origin: expr.clone(),
            description: format!("pattern from {:?}", expr),
        }
    }
    /// Compile a single expression node into a pattern node.
    fn compile_node(&mut self, expr: &Expr, _depth: u32) -> EPatternNode {
        match expr {
            Expr::BVar(idx) => {
                if let Some(&pvar) = self.bvar_to_pvar.get(idx) {
                    EPatternNode::Var(pvar)
                } else {
                    EPatternNode::Exact(expr.clone())
                }
            }
            Expr::App(_, _) => {
                let (head, args) = flatten_app(expr);
                let head_name = expr_head_name(&head);
                let compiled_args: Vec<EPatternNode> = args
                    .iter()
                    .map(|a| self.compile_node(a, _depth + 1))
                    .collect();
                EPatternNode::App {
                    func: head_name,
                    args: compiled_args,
                }
            }
            Expr::Const(name, _) => EPatternNode::App {
                func: name.clone(),
                args: Vec::new(),
            },
            Expr::FVar(_) | Expr::Lit(_) | Expr::Sort(_) => EPatternNode::Exact(expr.clone()),
            _ => EPatternNode::Wildcard,
        }
    }
    /// Allocate a fresh pattern variable.
    fn fresh_var(&mut self) -> u32 {
        let v = self.next_var;
        self.next_var += 1;
        v
    }
}
/// The main grind state that drives the saturation loop.
pub struct GrindState {
    /// The congruence closure engine.
    pub(super) cc: CongruenceClosure,
    /// Compiled E-match patterns from hypotheses.
    pub(super) patterns: Vec<(Name, EPattern)>,
    /// Already-generated instances (to avoid duplicates).
    pub(super) generated_instances: HashSet<Vec<Option<ENodeId>>>,
    /// Configuration.
    pub(super) config: GrindConfig,
    /// Statistics.
    pub(super) stats: GrindStats,
    /// The goal expression.
    pub(super) goal_expr: Option<Expr>,
    /// Goal node in the E-graph.
    pub(super) goal_node: Option<ENodeId>,
    /// Hypothesis names and expressions.
    pub(super) hypotheses: Vec<(Name, Expr)>,
    /// Case splitter state.
    pub(super) case_splitter: CaseSplitter,
    /// Term index for fast lookup.
    pub(super) term_index: TermIndex,
    /// Whether the goal has been proved.
    pub(super) proved: bool,
    /// The proof term if found.
    pub(super) proof_term: Option<Expr>,
    /// Current fuel remaining.
    pub(super) fuel: usize,
}
impl GrindState {
    /// Create a new grind state with the given configuration.
    pub fn new(config: GrindConfig) -> Self {
        let fuel = config.fuel;
        GrindState {
            cc: CongruenceClosure::with_capacity(256),
            patterns: Vec::new(),
            generated_instances: HashSet::new(),
            config,
            stats: GrindStats::default(),
            goal_expr: None,
            goal_node: None,
            hypotheses: Vec::new(),
            case_splitter: CaseSplitter::new(),
            term_index: TermIndex::new(),
            proved: false,
            proof_term: None,
            fuel,
        }
    }
    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(GrindConfig::default())
    }
    /// Add the goal to the E-graph.
    pub fn set_goal(&mut self, goal: Expr) {
        let node_id = self.cc.add_term(&goal);
        self.goal_expr = Some(goal);
        self.goal_node = Some(node_id);
    }
    /// Add a hypothesis to the E-graph and compile patterns.
    pub fn add_hypothesis(&mut self, name: Name, ty: Expr) {
        self.hypotheses.push((name.clone(), ty.clone()));
        self.stats.hyps_processed += 1;
        if let Some((lhs, rhs)) = decompose_eq(&ty) {
            let lhs_node = self.cc.add_term(&lhs);
            let rhs_node = self.cc.add_term(&rhs);
            self.cc.merge_with_reason(
                lhs_node,
                rhs_node,
                MergeReason::Hypothesis(name.clone(), ty.clone()),
            );
            return;
        }
        let _node = self.cc.add_term(&ty);
        if is_forall(&ty) {
            let mut compiler = EMatchCompiler::new();
            let pattern = compiler.compile(&ty);
            if pattern.num_vars > 0 {
                self.patterns.push((name.clone(), pattern));
            }
        }
        if let Some((left, right)) = decompose_or(&ty) {
            self.case_splitter.add_disjunction(name, left, right);
        }
    }
    /// Run the main saturation loop.
    pub fn run(&mut self) -> GrindResult {
        self.add_logic_constants();
        self.term_index = TermIndex::build_from_cc(&self.cc);
        for round in 0..self.config.max_rounds {
            self.stats.rounds = round + 1;
            if self.fuel == 0 {
                return GrindResult::ResourceLimit("fuel exhausted".to_string());
            }
            self.fuel -= 1;
            if self.cc.num_nodes() > self.config.max_nodes {
                return GrindResult::ResourceLimit(format!(
                    "max nodes ({}) exceeded",
                    self.config.max_nodes
                ));
            }
            if self.check_goal_proved() {
                let proof = self.reconstruct_proof();
                return GrindResult::Proved(proof);
            }
            if self.cc.is_inconsistent() {
                let proof = self.build_inconsistency_proof();
                return GrindResult::Proved(proof);
            }
            let new_instances = self.run_ematching_round();
            self.stats.instances += new_instances as u64;
            if new_instances == 0 {
                if self.config.split_cases
                    && self.stats.splits < self.config.max_splits
                    && self.try_case_split()
                {
                    self.stats.splits += 1;
                    continue;
                }
                return GrindResult::Saturated;
            }
            self.stats.merges = self.cc.total_merges();
            self.update_max_eclass_stat();
        }
        GrindResult::ResourceLimit("max rounds exceeded".to_string())
    }
    /// Add True/False constants.
    fn add_logic_constants(&mut self) {
        let true_expr = Expr::Const(Name::str("True"), vec![]);
        let false_expr = Expr::Const(Name::str("False"), vec![]);
        self.cc.add_term(&true_expr);
        self.cc.add_term(&false_expr);
    }
    /// Check if the goal is trivially proved.
    fn check_goal_proved(&self) -> bool {
        if let Some(goal_node) = self.goal_node {
            if self.cc.is_true(goal_node) {
                return true;
            }
            if let Some(goal_expr) = &self.goal_expr {
                if let Some((lhs, rhs)) = decompose_eq(goal_expr) {
                    if let (Some(lhs_node), Some(rhs_node)) =
                        (self.cc.lookup_expr(&lhs), self.cc.lookup_expr(&rhs))
                    {
                        return self.cc.are_equal(lhs_node, rhs_node);
                    }
                }
            }
        }
        false
    }
    /// Run one round of E-matching.
    fn run_ematching_round(&mut self) -> usize {
        let pattern_refs: Vec<EPattern> = self.patterns.iter().map(|(_, p)| p.clone()).collect();
        let pattern_names: Vec<Name> = self.patterns.iter().map(|(n, _)| n.clone()).collect();
        let remaining = self
            .config
            .max_instances
            .saturating_sub(self.stats.instances as usize);
        let matches = run_ematching(&self.cc, &pattern_refs, remaining);
        self.stats.ematches += matches.len() as u64;
        let mut new_count = 0;
        for (pat_idx, subst) in &matches {
            if self.generated_instances.contains(&subst.bindings) {
                continue;
            }
            self.generated_instances.insert(subst.bindings.clone());
            if *pat_idx < pattern_names.len() {
                let hyp_name = &pattern_names[*pat_idx];
                let pattern = &pattern_refs[*pat_idx];
                let (_, body) = strip_forall(&pattern.origin);
                let instantiated = apply_subst_to_expr(&body, subst, &self.cc);
                if let Some((lhs, rhs)) = decompose_eq(&instantiated) {
                    let lhs_node = self.cc.add_term(&lhs);
                    let rhs_node = self.cc.add_term(&rhs);
                    let expr_bindings = subst.to_expr_bindings(&self.cc);
                    self.cc.merge_with_reason(
                        lhs_node,
                        rhs_node,
                        MergeReason::EMatchInstance {
                            hyp_name: hyp_name.clone(),
                            subst: expr_bindings,
                        },
                    );
                    new_count += 1;
                } else {
                    let node = self.cc.add_term(&instantiated);
                    if is_prop_like(&instantiated) {
                        if let Some(true_expr) =
                            self.cc.lookup_expr(&Expr::Const(Name::str("True"), vec![]))
                        {
                            self.cc.merge_with_reason(
                                node,
                                true_expr,
                                MergeReason::EMatchInstance {
                                    hyp_name: hyp_name.clone(),
                                    subst: subst.to_expr_bindings(&self.cc),
                                },
                            );
                        }
                    }
                    new_count += 1;
                }
            }
        }
        new_count
    }
    /// Try to perform a case split.
    fn try_case_split(&mut self) -> bool {
        self.case_splitter.try_split(&mut self.cc)
    }
    /// Reconstruct a proof term from the E-graph.
    fn reconstruct_proof(&self) -> Expr {
        if !self.config.reconstruct_proofs {
            return Expr::Const(Name::str("grind_proof"), vec![]);
        }
        if let Some(goal_expr) = &self.goal_expr {
            if let Some((lhs, rhs)) = decompose_eq(goal_expr) {
                if let (Some(lhs_node), Some(rhs_node)) =
                    (self.cc.lookup_expr(&lhs), self.cc.lookup_expr(&rhs))
                {
                    let steps = self.cc.explain_equality(lhs_node, rhs_node);
                    return build_proof(&steps);
                }
            }
        }
        Expr::Const(Name::str("grind_proof"), vec![])
    }
    /// Build a proof from inconsistency (True = False).
    fn build_inconsistency_proof(&self) -> Expr {
        Expr::App(
            Box::new(Expr::Const(Name::str("absurd"), vec![])),
            Box::new(Expr::Const(Name::str("grind_false_proof"), vec![])),
        )
    }
    /// Update the max eclass size statistic.
    fn update_max_eclass_stat(&mut self) {
        let max = self
            .cc
            .all_classes()
            .iter()
            .filter_map(|&cid| self.cc.get_class(cid))
            .map(|c| c.size())
            .max()
            .unwrap_or(0);
        if max > self.stats.max_eclass {
            self.stats.max_eclass = max;
        }
        self.stats.total_nodes = self.cc.num_nodes();
    }
    /// Get a reference to the statistics.
    pub fn stats(&self) -> &GrindStats {
        &self.stats
    }
    /// Get a reference to the congruence closure.
    pub fn cc(&self) -> &CongruenceClosure {
        &self.cc
    }
    /// Check if proved.
    pub fn is_proved(&self) -> bool {
        self.proved
    }
}
/// An equivalence class in the E-graph.
#[derive(Clone, Debug)]
pub struct EClass {
    /// The canonical id for this class.
    pub id: EClassId,
    /// All E-nodes that belong to this class.
    pub nodes: Vec<ENodeId>,
    /// Parent E-nodes that reference a node in this class (for upward merging).
    pub parents: Vec<ENodeId>,
    /// Representative expression for this class.
    pub repr_expr: Option<Expr>,
    /// Whether this class contains True.
    pub is_true: bool,
    /// Whether this class contains False.
    pub is_false: bool,
}
impl EClass {
    /// Create a new equivalence class.
    pub(crate) fn new(id: EClassId) -> Self {
        EClass {
            id,
            nodes: Vec::new(),
            parents: Vec::new(),
            repr_expr: None,
            is_true: false,
            is_false: false,
        }
    }
    /// Add a node to this class.
    pub(crate) fn add_node(&mut self, node_id: ENodeId) {
        self.nodes.push(node_id);
    }
    /// Add a parent reference.
    pub(crate) fn add_parent(&mut self, parent_id: ENodeId) {
        self.parents.push(parent_id);
    }
    /// Number of nodes.
    pub(crate) fn size(&self) -> usize {
        self.nodes.len()
    }
}
/// Reason why two nodes were merged (for proof reconstruction).
#[derive(Clone, Debug)]
pub enum MergeReason {
    /// Merged because of a hypothesis: `h : a = b`.
    Hypothesis(Name, Expr),
    /// Merged by congruence: `f(a1..an) = f(b1..bn)` because `ai = bi`.
    Congruence(ENodeId, ENodeId),
    /// Merged by an E-matching instantiation.
    EMatchInstance {
        /// The quantified hypothesis that was instantiated.
        hyp_name: Name,
        /// The substitution used.
        subst: Vec<(Name, Expr)>,
    },
    /// Merged because both sides reduce to the same value.
    Reduction,
    /// Merged by user assertion.
    Assertion,
    /// Merged by reflexivity.
    Reflexivity,
}
/// A substitution produced by E-matching.
#[derive(Clone, Debug)]
pub struct Substitution {
    /// Bindings: pattern variable index -> ENodeId.
    pub bindings: Vec<Option<ENodeId>>,
    /// The matched class for the whole pattern.
    pub matched_class: EClassId,
}
impl Substitution {
    /// Create a new empty substitution.
    pub(super) fn new(num_vars: u32) -> Self {
        Substitution {
            bindings: vec![None; num_vars as usize],
            matched_class: EClassId(0),
        }
    }
    /// Bind a pattern variable.
    pub(super) fn bind(&mut self, var: u32, node: ENodeId) -> bool {
        let idx = var as usize;
        if idx >= self.bindings.len() {
            return false;
        }
        if let Some(existing) = self.bindings[idx] {
            return existing == node;
        }
        self.bindings[idx] = Some(node);
        true
    }
    /// Get the binding for a variable.
    pub(super) fn get(&self, var: u32) -> Option<ENodeId> {
        self.bindings.get(var as usize).copied().flatten()
    }
    /// Check if all variables are bound.
    pub(crate) fn is_complete(&self) -> bool {
        self.bindings.iter().all(|b| b.is_some())
    }
    /// Convert bindings to (Name, Expr) pairs using the CC.
    fn to_expr_bindings(&self, cc: &CongruenceClosure) -> Vec<(Name, Expr)> {
        self.bindings
            .iter()
            .enumerate()
            .filter_map(|(i, b)| {
                b.and_then(|nid| {
                    cc.get_node(nid).and_then(|node| {
                        node.origin_expr
                            .clone()
                            .map(|expr| (Name::str(format!("x_{}", i)), expr))
                    })
                })
            })
            .collect()
    }
}
/// Congruence Closure data structure.
///
/// Maintains an E-graph with a union-find for equivalence classes,
/// supports adding terms, merging, querying equality, and generating
/// explanations for equalities.
pub struct CongruenceClosure {
    /// The union-find structure for equivalence classes.
    pub(super) uf: UnionFind,
    /// All E-nodes.
    pub(super) nodes: Vec<ENode>,
    /// All equivalence classes indexed by their canonical id.
    pub(super) classes: HashMap<EClassId, EClass>,
    /// Signature table for congruence detection: (func, [canonical_class_of_arg]) -> ENodeId.
    pub(super) sig_table: HashMap<(Name, Vec<EClassId>), ENodeId>,
    /// Mapping from kernel Expr to ENodeId for deduplication.
    pub(super) expr_to_node: HashMap<Expr, ENodeId>,
    /// Merge log for proof reconstruction.
    pub(super) merge_log: Vec<(EClassId, EClassId, MergeReason)>,
    /// Pending merges worklist.
    pub(super) pending: VecDeque<PendingMerge>,
    /// Next class id.
    pub(super) next_class_id: u32,
    /// Total number of merges performed.
    pub(super) total_merges: u64,
    /// Whether the E-graph is inconsistent (True = False).
    pub(super) inconsistent: bool,
    /// The EClassId representing True, if it exists.
    pub(super) true_class: Option<EClassId>,
    /// The EClassId representing False, if it exists.
    pub(super) false_class: Option<EClassId>,
}
impl CongruenceClosure {
    /// Create a new empty congruence closure.
    pub fn new() -> Self {
        CongruenceClosure {
            uf: UnionFind::new(),
            nodes: Vec::new(),
            classes: HashMap::new(),
            sig_table: HashMap::new(),
            expr_to_node: HashMap::new(),
            merge_log: Vec::new(),
            pending: VecDeque::new(),
            next_class_id: 0,
            total_merges: 0,
            inconsistent: false,
            true_class: None,
            false_class: None,
        }
    }
    /// Create with pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        CongruenceClosure {
            uf: UnionFind::with_capacity(cap),
            nodes: Vec::with_capacity(cap),
            classes: HashMap::with_capacity(cap),
            sig_table: HashMap::with_capacity(cap),
            expr_to_node: HashMap::with_capacity(cap),
            merge_log: Vec::new(),
            pending: VecDeque::new(),
            next_class_id: 0,
            total_merges: 0,
            inconsistent: false,
            true_class: None,
            false_class: None,
        }
    }
    /// Number of E-nodes.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    /// Number of equivalence classes.
    pub fn num_classes(&self) -> usize {
        self.uf.num_sets() as usize
    }
    /// Is the E-graph inconsistent (proved False)?
    pub fn is_inconsistent(&self) -> bool {
        self.inconsistent
    }
    /// Total merges.
    pub fn total_merges(&self) -> u64 {
        self.total_merges
    }
    /// Allocate a fresh EClassId.
    fn fresh_class_id(&mut self) -> EClassId {
        let id = EClassId(self.next_class_id);
        self.next_class_id += 1;
        id
    }
    /// Get the canonical EClassId for a given class.
    pub fn find(&mut self, id: EClassId) -> EClassId {
        let canonical = self.uf.find(id.0);
        EClassId(canonical)
    }
    /// Get canonical class id without mutation.
    pub fn find_immut(&self, id: EClassId) -> EClassId {
        EClassId(self.uf.find_immut(id.0))
    }
    /// Get the ENode for a given node id.
    pub fn get_node(&self, id: ENodeId) -> Option<&ENode> {
        self.nodes.get(id.0 as usize)
    }
    /// Get the EClass for a given class id (must be canonical).
    pub fn get_class(&self, id: EClassId) -> Option<&EClass> {
        let canonical = self.find_immut(id);
        self.classes.get(&canonical)
    }
    /// Build a signature key for congruence lookup.
    fn make_sig(&self, func: &Name, args: &[ENodeId]) -> (Name, Vec<EClassId>) {
        let canonical_args: Vec<EClassId> = args
            .iter()
            .map(|a| {
                let node = &self.nodes[a.0 as usize];
                self.find_immut(node.eclass)
            })
            .collect();
        (func.clone(), canonical_args)
    }
    /// Add a term (kernel Expr) to the E-graph, returning its ENodeId.
    ///
    /// If the term already exists, returns the existing node id.
    /// Recursively adds sub-terms.
    pub fn add_term(&mut self, expr: &Expr) -> ENodeId {
        if let Some(&node_id) = self.expr_to_node.get(expr) {
            return node_id;
        }
        match expr {
            Expr::Const(name, _) => self.add_leaf(name.clone(), expr.clone()),
            Expr::FVar(fid) => {
                let name = Name::str(format!("fvar_{}", fid.0));
                self.add_leaf(name, expr.clone())
            }
            Expr::BVar(idx) => {
                let name = Name::str(format!("bvar_{}", idx));
                self.add_leaf(name, expr.clone())
            }
            Expr::Lit(lit) => {
                let name = Name::str(format!("lit_{}", lit));
                self.add_leaf(name, expr.clone())
            }
            Expr::Sort(level) => {
                let name = Name::str(format!("sort_{:?}", level));
                self.add_leaf(name, expr.clone())
            }
            Expr::App(_f, _a) => {
                let (head, args_exprs) = flatten_app(expr);
                let head_name = expr_head_name(&head);
                let arg_ids: Vec<ENodeId> =
                    args_exprs.iter().map(|arg| self.add_term(arg)).collect();
                let sig = self.make_sig_from_classes(&head_name, &arg_ids);
                if let Some(&existing) = self.sig_table.get(&sig) {
                    self.expr_to_node.insert(expr.clone(), existing);
                    return existing;
                }
                self.add_node_internal(head_name, arg_ids, expr.clone())
            }
            Expr::Lam(..) | Expr::Pi(..) | Expr::Let(..) => {
                let name = Name::str(format!("opaque_{}", self.nodes.len()));
                self.add_leaf(name, expr.clone())
            }
            Expr::Proj(proj_name, idx, base) => {
                let base_id = self.add_term(base);
                let func_name = Name::str(format!("proj_{}_{}", proj_name, idx));
                let sig = self.make_sig_from_classes(&func_name, &[base_id]);
                if let Some(&existing) = self.sig_table.get(&sig) {
                    self.expr_to_node.insert(expr.clone(), existing);
                    return existing;
                }
                self.add_node_internal(func_name, vec![base_id], expr.clone())
            }
        }
    }
    /// Build a signature key using node ids directly (looking up their classes).
    fn make_sig_from_classes(&self, func: &Name, arg_ids: &[ENodeId]) -> (Name, Vec<EClassId>) {
        let canonical_args: Vec<EClassId> = arg_ids
            .iter()
            .map(|&nid| {
                let node = &self.nodes[nid.0 as usize];
                self.find_immut(node.eclass)
            })
            .collect();
        (func.clone(), canonical_args)
    }
    /// Add a leaf node (no arguments).
    fn add_leaf(&mut self, func: Name, expr: Expr) -> ENodeId {
        if let Some(&existing) = self.expr_to_node.get(&expr) {
            return existing;
        }
        self.add_node_internal(func, Vec::new(), expr)
    }
    /// Internal: create a new E-node, its class, update tables.
    fn add_node_internal(&mut self, func: Name, args: Vec<ENodeId>, expr: Expr) -> ENodeId {
        let node_id = ENodeId(self.nodes.len() as u32);
        let class_id = self.fresh_class_id();
        let _ = self.uf.make_set();
        let mut eclass = EClass::new(class_id);
        eclass.add_node(node_id);
        eclass.repr_expr = Some(expr.clone());
        if is_true_expr(&expr) {
            eclass.is_true = true;
            self.true_class = Some(class_id);
        }
        if is_false_expr(&expr) {
            eclass.is_false = true;
            self.false_class = Some(class_id);
        }
        let node = ENode::with_origin(func.clone(), args.clone(), class_id, expr.clone());
        self.nodes.push(node);
        let sig = self.make_sig_from_classes(&func, &args);
        self.sig_table.insert(sig, node_id);
        for &arg_id in &args {
            let arg_class = self.find_immut(self.nodes[arg_id.0 as usize].eclass);
            if let Some(c) = self.classes.get_mut(&arg_class) {
                c.add_parent(node_id);
            }
        }
        self.classes.insert(class_id, eclass);
        self.expr_to_node.insert(expr, node_id);
        node_id
    }
    /// Check if two E-nodes are in the same equivalence class.
    pub fn are_equal(&self, a: ENodeId, b: ENodeId) -> bool {
        if a == b {
            return true;
        }
        let ca = self.nodes[a.0 as usize].eclass;
        let cb = self.nodes[b.0 as usize].eclass;
        self.uf.are_connected(ca.0, cb.0)
    }
    /// Check if two classes are equal.
    pub fn classes_equal(&self, a: EClassId, b: EClassId) -> bool {
        self.uf.are_connected(a.0, b.0)
    }
    /// Merge two equivalence classes, propagating congruence.
    pub fn merge(&mut self, a: ENodeId, b: ENodeId) {
        self.merge_with_reason(a, b, MergeReason::Assertion);
    }
    /// Merge two E-nodes with a given reason.
    pub fn merge_with_reason(&mut self, a: ENodeId, b: ENodeId, reason: MergeReason) {
        let ca = self.nodes[a.0 as usize].eclass;
        let cb = self.nodes[b.0 as usize].eclass;
        self.merge_classes(ca, cb, reason);
    }
    /// Merge two classes by id.
    fn merge_classes(&mut self, a: EClassId, b: EClassId, reason: MergeReason) {
        let ca = self.find(a);
        let cb = self.find(b);
        if ca == cb {
            return;
        }
        self.pending.push_back(PendingMerge {
            a: ca,
            b: cb,
            reason,
        });
        self.propagate();
    }
    /// Process all pending merges and propagate congruences.
    fn propagate(&mut self) {
        while let Some(pm) = self.pending.pop_front() {
            let ca = self.find(pm.a);
            let cb = self.find(pm.b);
            if ca == cb {
                continue;
            }
            self.merge_log.push((ca, cb, pm.reason));
            self.total_merges += 1;
            let (winner, loser) = {
                let sa = self.classes.get(&ca).map(|c| c.size()).unwrap_or(0);
                let sb = self.classes.get(&cb).map(|c| c.size()).unwrap_or(0);
                if sa >= sb {
                    (ca, cb)
                } else {
                    (cb, ca)
                }
            };
            self.uf.union(winner.0, loser.0);
            let new_root = EClassId(self.uf.find(winner.0));
            let loser_parents: Vec<ENodeId> = self
                .classes
                .get(&loser)
                .map(|c| c.parents.clone())
                .unwrap_or_default();
            let winner_parents: Vec<ENodeId> = self
                .classes
                .get(&winner)
                .map(|c| c.parents.clone())
                .unwrap_or_default();
            if let Some(loser_class) = self.classes.remove(&loser) {
                let winner_class = self
                    .classes
                    .entry(new_root)
                    .or_insert_with(|| EClass::new(new_root));
                for n in &loser_class.nodes {
                    winner_class.add_node(*n);
                }
                for p in &loser_class.parents {
                    winner_class.add_parent(*p);
                }
                if loser_class.is_true {
                    winner_class.is_true = true;
                }
                if loser_class.is_false {
                    winner_class.is_false = true;
                }
                if winner_class.is_true && winner_class.is_false {
                    self.inconsistent = true;
                }
            }
            if new_root != winner {
                if let Some(class_data) = self.classes.remove(&winner) {
                    self.classes.insert(new_root, class_data);
                }
            }
            self.check_congruences(&loser_parents, &winner_parents);
        }
    }
    /// Check for new congruences after a merge.
    fn check_congruences(&mut self, loser_parents: &[ENodeId], _winner_parents: &[ENodeId]) {
        let mut new_sigs: Vec<(ENodeId, (Name, Vec<EClassId>))> = Vec::new();
        for &parent_id in loser_parents {
            if parent_id.0 as usize >= self.nodes.len() {
                continue;
            }
            let node = &self.nodes[parent_id.0 as usize];
            let new_sig = self.make_sig_from_classes(&node.func, &node.args);
            new_sigs.push((parent_id, new_sig));
        }
        for (parent_id, sig) in new_sigs {
            if let Some(&existing_id) = self.sig_table.get(&sig) {
                if existing_id != parent_id && !self.are_equal(existing_id, parent_id) {
                    let ca = self.nodes[existing_id.0 as usize].eclass;
                    let cb = self.nodes[parent_id.0 as usize].eclass;
                    let fca = self.find(ca);
                    let fcb = self.find(cb);
                    if fca != fcb {
                        self.pending.push_back(PendingMerge {
                            a: fca,
                            b: fcb,
                            reason: MergeReason::Congruence(existing_id, parent_id),
                        });
                    }
                }
            } else {
                self.sig_table.insert(sig, parent_id);
            }
        }
    }
    /// Explain why two nodes are equal, returning a list of equality steps.
    pub fn explain_equality(&self, a: ENodeId, b: ENodeId) -> Vec<EqualityStep> {
        if !self.are_equal(a, b) {
            return Vec::new();
        }
        if a == b {
            let expr = self.nodes[a.0 as usize]
                .origin_expr
                .clone()
                .unwrap_or(Expr::BVar(0));
            return vec![EqualityStep {
                lhs: expr.clone(),
                rhs: expr,
                reason: MergeReason::Reflexivity,
            }];
        }
        let target_a = self.nodes[a.0 as usize].eclass;
        let target_b = self.nodes[b.0 as usize].eclass;
        self.find_explanation_path(target_a, target_b)
    }
    /// Find an explanation path in the merge log using BFS.
    fn find_explanation_path(&self, a: EClassId, b: EClassId) -> Vec<EqualityStep> {
        let mut adj: HashMap<EClassId, Vec<(EClassId, usize)>> = HashMap::new();
        for (i, (c1, c2, _)) in self.merge_log.iter().enumerate() {
            adj.entry(*c1).or_default().push((*c2, i));
            adj.entry(*c2).or_default().push((*c1, i));
        }
        let mut visited: HashSet<EClassId> = HashSet::new();
        let mut queue: VecDeque<(EClassId, Vec<usize>)> = VecDeque::new();
        visited.insert(a);
        queue.push_back((a, Vec::new()));
        while let Some((current, path)) = queue.pop_front() {
            if self.uf.are_connected(current.0, b.0) && !path.is_empty() {
                return path
                    .iter()
                    .map(|&idx| {
                        let (c1, c2, reason) = &self.merge_log[idx];
                        let lhs = self
                            .classes
                            .get(c1)
                            .and_then(|c| c.repr_expr.clone())
                            .unwrap_or(Expr::BVar(0));
                        let rhs = self
                            .classes
                            .get(c2)
                            .and_then(|c| c.repr_expr.clone())
                            .unwrap_or(Expr::BVar(0));
                        EqualityStep {
                            lhs,
                            rhs,
                            reason: reason.clone(),
                        }
                    })
                    .collect();
            }
            if let Some(neighbors) = adj.get(&current) {
                for &(next, merge_idx) in neighbors {
                    if visited.insert(next) {
                        let mut new_path = path.clone();
                        new_path.push(merge_idx);
                        queue.push_back((next, new_path));
                    }
                }
            }
        }
        Vec::new()
    }
    /// Get the ENodeId for an expression if it exists.
    pub fn lookup_expr(&self, expr: &Expr) -> Option<ENodeId> {
        self.expr_to_node.get(expr).copied()
    }
    /// Get all node ids in a given class.
    pub fn class_nodes(&self, class_id: EClassId) -> Vec<ENodeId> {
        let canonical = self.find_immut(class_id);
        self.classes
            .get(&canonical)
            .map(|c| c.nodes.clone())
            .unwrap_or_default()
    }
    /// Get all classes.
    pub fn all_classes(&self) -> Vec<EClassId> {
        self.classes.keys().copied().collect()
    }
    /// Iterate over all nodes.
    pub fn all_nodes(&self) -> &[ENode] {
        &self.nodes
    }
    /// Check if an expression's class is equivalent to True.
    pub fn is_true(&self, node_id: ENodeId) -> bool {
        if let Some(true_class) = self.true_class {
            let node_class = self.nodes[node_id.0 as usize].eclass;
            self.uf.are_connected(node_class.0, true_class.0)
        } else {
            false
        }
    }
    /// Check if an expression's class is equivalent to False.
    pub fn is_false(&self, node_id: ENodeId) -> bool {
        if let Some(false_class) = self.false_class {
            let node_class = self.nodes[node_id.0 as usize].eclass;
            self.uf.are_connected(node_class.0, false_class.0)
        } else {
            false
        }
    }
}
/// Term index for fast lookup by function symbol.
pub struct TermIndex {
    /// Function symbol -> list of E-node ids with that head.
    pub(super) func_index: HashMap<Name, Vec<ENodeId>>,
    /// Arity index: (func, arity) -> list of nodes.
    pub(super) arity_index: HashMap<(Name, usize), Vec<ENodeId>>,
    /// Total indexed nodes.
    pub(super) total_entries: usize,
}
impl TermIndex {
    /// Create a new empty term index.
    pub fn new() -> Self {
        TermIndex {
            func_index: HashMap::new(),
            arity_index: HashMap::new(),
            total_entries: 0,
        }
    }
    /// Build the index from a CongruenceClosure.
    pub fn build_from_cc(cc: &CongruenceClosure) -> Self {
        let mut index = TermIndex::new();
        for (i, node) in cc.all_nodes().iter().enumerate() {
            let nid = ENodeId(i as u32);
            index
                .func_index
                .entry(node.func.clone())
                .or_default()
                .push(nid);
            index
                .arity_index
                .entry((node.func.clone(), node.arity()))
                .or_default()
                .push(nid);
            index.total_entries += 1;
        }
        index
    }
    /// Add a single node.
    pub fn add_node(&mut self, node_id: ENodeId, node: &ENode) {
        self.func_index
            .entry(node.func.clone())
            .or_default()
            .push(node_id);
        self.arity_index
            .entry((node.func.clone(), node.arity()))
            .or_default()
            .push(node_id);
        self.total_entries += 1;
    }
    /// Lookup nodes by function symbol.
    pub fn lookup_func(&self, func: &Name) -> &[ENodeId] {
        self.func_index
            .get(func)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Lookup nodes by function symbol and arity.
    pub fn lookup_func_arity(&self, func: &Name, arity: usize) -> &[ENodeId] {
        self.arity_index
            .get(&(func.clone(), arity))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Number of indexed entries.
    pub fn len(&self) -> usize {
        self.total_entries
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.total_entries == 0
    }
    /// All indexed function symbols.
    pub fn all_funcs(&self) -> Vec<&Name> {
        self.func_index.keys().collect()
    }
}
/// A compiled E-matching pattern.
#[derive(Clone, Debug)]
pub struct EPattern {
    /// The root pattern node.
    pub root: EPatternNode,
    /// Number of pattern variables.
    pub num_vars: u32,
    /// The original expression this pattern was compiled from.
    pub origin: Expr,
    /// Human-readable description.
    pub description: String,
}
/// Signature table for hash-consing of function applications.
///
/// Maps `(func, [canonical_class_of_arg1, ...])` to the canonical ENodeId.
pub struct SignatureTable {
    /// The table mapping signatures to node ids.
    pub(super) table: HashMap<(Name, Vec<EClassId>), ENodeId>,
    /// Number of entries.
    pub(super) num_entries: usize,
    /// Number of lookups.
    pub(super) num_lookups: u64,
    /// Number of hits.
    pub(super) num_hits: u64,
}
impl SignatureTable {
    /// Create a new empty table.
    pub fn new() -> Self {
        SignatureTable {
            table: HashMap::new(),
            num_entries: 0,
            num_lookups: 0,
            num_hits: 0,
        }
    }
    /// Insert a new entry.
    pub fn insert(&mut self, func: Name, args: Vec<EClassId>, node: ENodeId) {
        self.table.insert((func, args), node);
        self.num_entries += 1;
    }
    /// Lookup.
    pub fn lookup(&mut self, func: &Name, args: &[EClassId]) -> Option<ENodeId> {
        self.num_lookups += 1;
        let key = (func.clone(), args.to_vec());
        let result = self.table.get(&key).copied();
        if result.is_some() {
            self.num_hits += 1;
        }
        result
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.num_entries
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.num_entries == 0
    }
    /// Hit rate.
    pub fn hit_rate(&self) -> f64 {
        if self.num_lookups == 0 {
            0.0
        } else {
            self.num_hits as f64 / self.num_lookups as f64
        }
    }
    /// Rebuild the table after merges using the CC.
    pub fn rebuild(&mut self, cc: &CongruenceClosure) {
        self.table.clear();
        self.num_entries = 0;
        for (i, node) in cc.all_nodes().iter().enumerate() {
            let canonical_args: Vec<EClassId> = node
                .args
                .iter()
                .map(|&a| {
                    let n = &cc.all_nodes()[a.0 as usize];
                    cc.find_immut(n.eclass)
                })
                .collect();
            let key = (node.func.clone(), canonical_args);
            self.table.entry(key).or_insert(ENodeId(i as u32));
            self.num_entries += 1;
        }
    }
}
/// An E-match instance: a pattern together with its bindings.
#[derive(Clone, Debug)]
pub struct EMatch {
    /// The pattern that was matched.
    pub pattern: EPattern,
    /// The substitution that was found.
    pub subst: Substitution,
}
/// Multi-pattern: conjunction of patterns that must all match simultaneously.
#[derive(Clone, Debug)]
pub struct MultiPattern {
    /// Component patterns.
    pub patterns: Vec<EPattern>,
    /// Origin hypothesis name.
    pub hyp_name: Name,
}
/// Union-Find data structure with path compression and union by rank.
#[derive(Clone, Debug)]
pub struct UnionFind {
    /// Parent pointers. `parent[i]` is the parent of class i.
    pub(super) parent: Vec<u32>,
    /// Rank for union by rank.
    pub(super) rank: Vec<u32>,
    /// Number of distinct sets.
    pub(super) num_sets: u32,
}
impl UnionFind {
    /// Create a new empty union-find.
    pub(crate) fn new() -> Self {
        UnionFind {
            parent: Vec::new(),
            rank: Vec::new(),
            num_sets: 0,
        }
    }
    /// Create a new union-find with capacity.
    pub(crate) fn with_capacity(cap: usize) -> Self {
        UnionFind {
            parent: Vec::with_capacity(cap),
            rank: Vec::with_capacity(cap),
            num_sets: 0,
        }
    }
    /// Add a new element, returning its id.
    pub(crate) fn make_set(&mut self) -> u32 {
        let id = self.parent.len() as u32;
        self.parent.push(id);
        self.rank.push(0);
        self.num_sets += 1;
        id
    }
    /// Find the representative of the set containing `x`, with path compression.
    pub(crate) fn find(&mut self, mut x: u32) -> u32 {
        let mut root = x;
        while self.parent[root as usize] != root {
            root = self.parent[root as usize];
        }
        while self.parent[x as usize] != root {
            let next = self.parent[x as usize];
            self.parent[x as usize] = root;
            x = next;
        }
        root
    }
    /// Find without mutation (for read-only queries).
    fn find_immut(&self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            x = self.parent[x as usize];
        }
        x
    }
    /// Union two sets. Returns true if they were different sets.
    pub(crate) fn union(&mut self, a: u32, b: u32) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        match self.rank[ra as usize].cmp(&self.rank[rb as usize]) {
            std::cmp::Ordering::Less => {
                self.parent[ra as usize] = rb;
            }
            std::cmp::Ordering::Greater => {
                self.parent[rb as usize] = ra;
            }
            std::cmp::Ordering::Equal => {
                self.parent[rb as usize] = ra;
                self.rank[ra as usize] += 1;
            }
        }
        self.num_sets -= 1;
        true
    }
    /// Check if two elements are in the same set.
    pub(crate) fn are_connected(&self, a: u32, b: u32) -> bool {
        self.find_immut(a) == self.find_immut(b)
    }
    /// Number of distinct sets.
    pub(crate) fn num_sets(&self) -> u32 {
        self.num_sets
    }
    /// Total number of elements.
    pub(crate) fn len(&self) -> usize {
        self.parent.len()
    }
    /// Check if empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}
/// Configuration for the grind tactic.
#[derive(Clone, Debug)]
pub struct GrindConfig {
    /// Maximum number of saturation rounds.
    pub max_rounds: usize,
    /// Maximum number of E-match instances to generate.
    pub max_instances: usize,
    /// Maximum size of any equivalence class before giving up.
    pub max_eclass_size: usize,
    /// Whether to attempt case splitting on disjunctions.
    pub split_cases: bool,
    /// Whether to use simp before/during grind.
    pub use_simp: bool,
    /// Maximum number of case splits allowed.
    pub max_splits: usize,
    /// Maximum total E-nodes in the E-graph.
    pub max_nodes: usize,
    /// Whether to collect statistics.
    pub collect_stats: bool,
    /// Timeout in rounds (0 = no limit beyond max_rounds).
    pub fuel: usize,
    /// Whether to attempt proof reconstruction.
    pub reconstruct_proofs: bool,
}
impl GrindConfig {
    /// Create a config with increased limits for harder problems.
    pub fn aggressive() -> Self {
        GrindConfig {
            max_rounds: 500,
            max_instances: 5000,
            max_eclass_size: 2000,
            split_cases: true,
            use_simp: true,
            max_splits: 50,
            max_nodes: 50000,
            collect_stats: true,
            fuel: 1000,
            reconstruct_proofs: true,
        }
    }
    /// Builder: set max rounds.
    pub fn with_max_rounds(mut self, n: usize) -> Self {
        self.max_rounds = n;
        self
    }
    /// Builder: set max instances.
    pub fn with_max_instances(mut self, n: usize) -> Self {
        self.max_instances = n;
        self
    }
    /// Builder: set max eclass size.
    pub fn with_max_eclass_size(mut self, n: usize) -> Self {
        self.max_eclass_size = n;
        self
    }
    /// Builder: toggle case splitting.
    pub fn with_split_cases(mut self, v: bool) -> Self {
        self.split_cases = v;
        self
    }
    /// Builder: toggle simp usage.
    pub fn with_simp(mut self, v: bool) -> Self {
        self.use_simp = v;
        self
    }
    /// Builder: set max splits.
    pub fn with_max_splits(mut self, n: usize) -> Self {
        self.max_splits = n;
        self
    }
    /// Builder: set fuel.
    pub fn with_fuel(mut self, n: usize) -> Self {
        self.fuel = n;
        self
    }
    /// Builder: toggle proof reconstruction.
    pub fn with_proofs(mut self, v: bool) -> Self {
        self.reconstruct_proofs = v;
        self
    }
    /// Builder: toggle stats collection.
    pub fn with_stats(mut self, v: bool) -> Self {
        self.collect_stats = v;
        self
    }
}
/// A disjunction registered for potential case splitting.
#[derive(Clone, Debug)]
struct DisjunctionCase {
    /// Hypothesis name.
    pub(super) hyp_name: Name,
    /// Left branch.
    pub(super) left: Expr,
    /// Right branch.
    pub(super) right: Expr,
    /// Whether this has been split already.
    pub(super) split: bool,
}
/// A pending merge in the worklist.
#[derive(Clone, Debug)]
struct PendingMerge {
    pub(super) a: EClassId,
    pub(super) b: EClassId,
    pub(super) reason: MergeReason,
}
/// A compiled E-matching pattern node.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EPatternNode {
    /// A pattern variable to be bound during matching.
    Var(u32),
    /// A function application pattern: func(args...).
    App {
        /// Head function symbol.
        func: Name,
        /// Argument patterns.
        args: Vec<EPatternNode>,
    },
    /// A wildcard that matches anything.
    Wildcard,
    /// A constant/literal that must match exactly.
    Exact(Expr),
}
/// Unique identifier for an E-node in the E-graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ENodeId(pub u32);
/// Unique identifier for an equivalence class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EClassId(pub u32);
/// Statistics collected during grind execution.
#[derive(Clone, Debug, Default)]
pub struct GrindStats {
    /// Number of saturation rounds performed.
    pub rounds: usize,
    /// Total merges in the congruence closure.
    pub merges: u64,
    /// Total E-match attempts.
    pub ematches: u64,
    /// Total instances generated.
    pub instances: u64,
    /// Number of case splits performed.
    pub splits: usize,
    /// Maximum equivalence class size observed.
    pub max_eclass: usize,
    /// Total nodes in E-graph at completion.
    pub total_nodes: usize,
    /// Number of congruence closures triggered.
    pub congruences: u64,
    /// Number of hypotheses processed.
    pub hyps_processed: usize,
}
/// An E-node represents a function application `func(args...)` in the E-graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ENode {
    /// The function symbol (head) of this node.
    pub func: Name,
    /// Arguments as ENodeIds.
    pub args: Vec<ENodeId>,
    /// The equivalence class this node belongs to.
    pub eclass: EClassId,
    /// The original kernel expression (for proof reconstruction).
    pub origin_expr: Option<Expr>,
}
impl ENode {
    /// Create a new E-node.
    pub(crate) fn new(func: Name, args: Vec<ENodeId>, eclass: EClassId) -> Self {
        ENode {
            func,
            args,
            eclass,
            origin_expr: None,
        }
    }
    /// Create a new E-node with an origin expression.
    pub(crate) fn with_origin(
        func: Name,
        args: Vec<ENodeId>,
        eclass: EClassId,
        expr: Expr,
    ) -> Self {
        ENode {
            func,
            args,
            eclass,
            origin_expr: Some(expr),
        }
    }
    /// Arity of this node.
    pub(crate) fn arity(&self) -> usize {
        self.args.len()
    }
    /// Check if this is a leaf node (constant / variable).
    pub(crate) fn is_leaf(&self) -> bool {
        self.args.is_empty()
    }
}
/// A simple linear arithmetic constraint for Nat goals discovered during grind.
#[derive(Clone, Debug, PartialEq)]
pub struct NatConstraint {
    /// Left-hand side expression (as a flat term).
    pub lhs: Expr,
    /// Right-hand side expression.
    pub rhs: Expr,
    /// The relation kind.
    pub rel: NatRelKind,
}
/// Result of a grind execution.
#[derive(Clone, Debug)]
pub enum GrindResult {
    /// Successfully proved the goal; contains the proof term.
    Proved(Expr),
    /// Reached saturation without proving the goal.
    Saturated,
    /// Hit a resource limit.
    ResourceLimit(String),
}
impl GrindResult {
    /// Check if the goal was proved.
    pub fn is_proved(&self) -> bool {
        matches!(self, GrindResult::Proved(_))
    }
    /// Get the proof term if proved.
    pub fn proof_term(&self) -> Option<&Expr> {
        if let GrindResult::Proved(p) = self {
            Some(p)
        } else {
            None
        }
    }
}
/// A single step in an equality explanation.
#[derive(Clone, Debug)]
pub struct EqualityStep {
    /// The left-hand side expression.
    pub lhs: Expr,
    /// The right-hand side expression.
    pub rhs: Expr,
    /// The reason for this equality.
    pub reason: MergeReason,
}
/// A step in a grind proof.
#[derive(Clone, Debug)]
pub enum ProofStep {
    /// Reflexivity: a = a.
    Refl(Expr),
    /// Symmetry: if a = b then b = a.
    Symm(Box<ProofStep>),
    /// Transitivity: if a = b and b = c then a = c.
    Trans(Box<ProofStep>, Box<ProofStep>),
    /// Congruence: if f = g and a = b then f(a) = g(b).
    Congr {
        /// Proof that functions are equal.
        func_proof: Box<ProofStep>,
        /// Proof that arguments are equal.
        arg_proof: Box<ProofStep>,
    },
    /// From a hypothesis.
    Hypothesis(Name, Expr),
    /// From an E-matching instance.
    Instance {
        /// Hypothesis name.
        hyp_name: Name,
        /// Substitution applied.
        subst: Vec<(Name, Expr)>,
    },
    /// Proof of absurdity (for inconsistency).
    Absurd,
}
