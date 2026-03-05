//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Declaration, Expr, Name, ReducibilityHint};
use oxilean_parse::AttributeKind;
use std::collections::{HashMap, HashSet};

/// Relation of an argument in a recursive call relative to the caller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgRelation {
    /// Argument is syntactically equal to the corresponding parameter
    Equal,
    /// Argument is structurally smaller
    Smaller,
    /// Unknown relation
    Unknown,
}
/// Call graph tracking recursive calls in a mutual block.
#[derive(Clone, Debug, Default)]
pub struct CallGraph {
    /// Map from caller name to list of recursive calls it makes
    calls: HashMap<Name, Vec<RecursiveCall>>,
    /// All function names in the mutual block
    names: Vec<Name>,
}
impl CallGraph {
    /// Build a call graph from a mutual block.
    ///
    /// This performs a conservative syntactic analysis of function bodies
    /// to detect recursive calls to functions in the mutual block.
    #[allow(dead_code)]
    pub fn build_from_block(block: &MutualBlock) -> Self {
        let mut calls: HashMap<Name, Vec<RecursiveCall>> = HashMap::new();
        let block_names: HashSet<Name> = block.names.iter().cloned().collect();
        for name in &block.names {
            let mut func_calls = Vec::new();
            if let Some(body) = block.get_body(name) {
                Self::collect_calls(name, body, &block_names, &mut func_calls);
            }
            calls.insert(name.clone(), func_calls);
        }
        Self {
            calls,
            names: block.names.clone(),
        }
    }
    /// Recursively collect calls from an expression.
    /// Peel a curried application into (head, [arg0, arg1, ...]).
    fn peel_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
        let mut args = Vec::new();
        let mut cur = expr;
        while let Expr::App(f, a) = cur {
            args.push(a.as_ref());
            cur = f.as_ref();
        }
        args.reverse();
        (cur, args)
    }
    fn collect_calls(
        caller: &Name,
        expr: &Expr,
        block_names: &HashSet<Name>,
        out: &mut Vec<RecursiveCall>,
    ) {
        match expr {
            Expr::App(func, arg) => {
                let (head, all_args) = Self::peel_app(expr);
                if let Some(callee_name) = Self::get_const_head(head) {
                    if block_names.contains(&callee_name) {
                        let relations: Vec<ArgRelation> =
                            all_args.iter().map(|a| Self::classify_arg(a)).collect();
                        out.push(RecursiveCall {
                            caller: caller.clone(),
                            callee: callee_name,
                            args: relations,
                        });
                        for a in &all_args {
                            Self::collect_calls(caller, a, block_names, out);
                        }
                        return;
                    }
                }
                Self::collect_calls(caller, func, block_names, out);
                Self::collect_calls(caller, arg, block_names, out);
            }
            Expr::Lam(_, _, ty, body) => {
                Self::collect_calls(caller, ty, block_names, out);
                Self::collect_calls(caller, body, block_names, out);
            }
            Expr::Pi(_, _, ty, body) => {
                Self::collect_calls(caller, ty, block_names, out);
                Self::collect_calls(caller, body, block_names, out);
            }
            Expr::Let(_, ty, val, body) => {
                Self::collect_calls(caller, ty, block_names, out);
                Self::collect_calls(caller, val, block_names, out);
                Self::collect_calls(caller, body, block_names, out);
            }
            Expr::Proj(_, _, base) => {
                Self::collect_calls(caller, base, block_names, out);
            }
            _ => {}
        }
    }
    /// Extract the constant name from the head of a (possibly nested) application.
    fn get_const_head(expr: &Expr) -> Option<Name> {
        match expr {
            Expr::Const(name, _) => Some(name.clone()),
            Expr::App(func, _) => Self::get_const_head(func),
            _ => None,
        }
    }
    /// Classify an argument expression as Equal, Smaller, or Unknown.
    fn classify_arg(expr: &Expr) -> ArgRelation {
        match expr {
            Expr::BVar(_) => ArgRelation::Equal,
            Expr::Proj(_, _, base) => {
                if matches!(base.as_ref(), Expr::BVar(_)) {
                    ArgRelation::Smaller
                } else {
                    ArgRelation::Unknown
                }
            }
            Expr::App(func, _) => {
                if matches!(func.as_ref(), Expr::BVar(_)) {
                    ArgRelation::Smaller
                } else {
                    ArgRelation::Unknown
                }
            }
            _ => ArgRelation::Unknown,
        }
    }
    /// Check if a specific argument position is structurally decreasing
    /// for a given function.
    #[allow(dead_code)]
    pub fn is_structurally_decreasing(&self, name: &Name, arg_idx: usize) -> bool {
        if let Some(func_calls) = self.calls.get(name) {
            if func_calls.is_empty() {
                return true;
            }
            let mut has_smaller = false;
            for call in func_calls {
                if call.callee == *name {
                    match call.args.get(arg_idx) {
                        Some(ArgRelation::Smaller) => has_smaller = true,
                        Some(ArgRelation::Equal) => {}
                        _ => return false,
                    }
                }
            }
            has_smaller || func_calls.iter().all(|c| c.callee != *name)
        } else {
            false
        }
    }
    /// Find the first argument index that is structurally decreasing.
    #[allow(dead_code)]
    pub fn find_decreasing_arg(&self, name: &Name) -> Option<usize> {
        let max_args = self
            .calls
            .get(name)
            .map(|calls| {
                calls
                    .iter()
                    .filter(|c| &c.callee == name)
                    .map(|c| c.args.len())
                    .max()
                    .unwrap_or(0)
            })
            .unwrap_or(0);
        let n = if max_args == 0 { 1 } else { max_args };
        (0..n).find(|&idx| self.is_structurally_decreasing(name, idx))
    }
    /// Check if the block is mutually recursive (cross-function calls exist).
    #[allow(dead_code)]
    pub fn is_mutually_recursive(&self) -> bool {
        for (caller, func_calls) in &self.calls {
            for call in func_calls {
                if &call.callee != caller {
                    return true;
                }
            }
        }
        false
    }
    /// Check if any function in the block is self-recursive.
    #[allow(dead_code)]
    pub fn is_self_recursive(&self, name: &Name) -> bool {
        self.calls
            .get(name)
            .map(|cs| cs.iter().any(|c| c.callee == *name))
            .unwrap_or(false)
    }
    /// Check if any function in the block is recursive at all.
    #[allow(dead_code)]
    pub fn is_recursive(&self) -> bool {
        for func_calls in self.calls.values() {
            if !func_calls.is_empty() {
                return true;
            }
        }
        false
    }
    /// Get all calls made by a specific function.
    #[allow(dead_code)]
    pub fn get_calls(&self, name: &Name) -> &[RecursiveCall] {
        self.calls.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Compute strongly connected components using Tarjan's algorithm.
    #[allow(dead_code)]
    pub fn strongly_connected_components(&self) -> Vec<Vec<Name>> {
        let n = self.names.len();
        if n == 0 {
            return Vec::new();
        }
        let name_to_idx: HashMap<Name, usize> = self
            .names
            .iter()
            .enumerate()
            .map(|(i, name)| (name.clone(), i))
            .collect();
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (caller, func_calls) in &self.calls {
            if let Some(&ci) = name_to_idx.get(caller) {
                for call in func_calls {
                    if let Some(&cj) = name_to_idx.get(&call.callee) {
                        if !adj[ci].contains(&cj) {
                            adj[ci].push(cj);
                        }
                    }
                }
            }
        }
        let mut index_counter: usize = 0;
        let mut stack: Vec<usize> = Vec::new();
        let mut on_stack = vec![false; n];
        let mut indices: Vec<Option<usize>> = vec![None; n];
        let mut lowlinks = vec![0usize; n];
        let mut result: Vec<Vec<Name>> = Vec::new();
        for v in 0..n {
            if indices[v].is_none() {
                Self::tarjan_visit(
                    v,
                    &adj,
                    &mut index_counter,
                    &mut stack,
                    &mut on_stack,
                    &mut indices,
                    &mut lowlinks,
                    &mut result,
                    &self.names,
                );
            }
        }
        result
    }
    /// Tarjan DFS visit helper.
    #[allow(clippy::too_many_arguments)]
    fn tarjan_visit(
        v: usize,
        adj: &[Vec<usize>],
        index_counter: &mut usize,
        stack: &mut Vec<usize>,
        on_stack: &mut Vec<bool>,
        indices: &mut Vec<Option<usize>>,
        lowlinks: &mut Vec<usize>,
        result: &mut Vec<Vec<Name>>,
        names: &[Name],
    ) {
        indices[v] = Some(*index_counter);
        lowlinks[v] = *index_counter;
        *index_counter += 1;
        stack.push(v);
        on_stack[v] = true;
        for &w in &adj[v] {
            if indices[w].is_none() {
                Self::tarjan_visit(
                    w,
                    adj,
                    index_counter,
                    stack,
                    on_stack,
                    indices,
                    lowlinks,
                    result,
                    names,
                );
                lowlinks[v] = lowlinks[v].min(lowlinks[w]);
            } else if on_stack[w] {
                lowlinks[v] =
                    lowlinks[v].min(indices[w].expect("w is on stack so indices[w] is set"));
            }
        }
        if lowlinks[v] == indices[v].expect("v was just assigned an index above") {
            let mut component = Vec::new();
            loop {
                let w = stack
                    .pop()
                    .expect("stack is non-empty: v is always on it when we reach the SCC root");
                on_stack[w] = false;
                component.push(names[w].clone());
                if w == v {
                    break;
                }
            }
            result.push(component);
        }
    }
}
/// Encoder for well-founded recursive definitions.
///
/// Used when structural recursion cannot be established. Requires
/// a well-founded relation and a measure function.
#[derive(Clone, Debug)]
pub struct WellFoundedRecursion {
    /// The mutual block being processed
    pub block: MutualBlock,
    /// Optional measure function name
    pub measure: Option<Name>,
    /// Optional well-founded relation expression
    pub rel: Option<Expr>,
    /// For each function, which argument indices decrease
    pub decreasing_args: HashMap<Name, Vec<usize>>,
}
impl WellFoundedRecursion {
    /// Create a new well-founded recursion encoder.
    #[allow(dead_code)]
    pub fn new(block: MutualBlock) -> Self {
        Self {
            block,
            measure: None,
            rel: None,
            decreasing_args: HashMap::new(),
        }
    }
    /// Set the measure function.
    #[allow(dead_code)]
    pub fn set_measure(&mut self, name: Name) {
        self.measure = Some(name);
    }
    /// Set the well-founded relation.
    #[allow(dead_code)]
    pub fn set_relation(&mut self, rel: Expr) {
        self.rel = Some(rel);
    }
    /// Detect which arguments are decreasing under the given measure.
    ///
    /// Uses structural analysis via `CallGraph` to find which argument positions
    /// are structurally decreasing. Falls back to argument 0 if none can be
    /// determined (e.g. for well-founded recursion with an explicit measure).
    #[allow(dead_code)]
    pub fn detect_decreasing_args(&mut self) -> Result<(), MutualElabError> {
        let call_graph = CallGraph::build_from_block(&self.block);
        for name in &self.block.names {
            if call_graph.is_self_recursive(name) || call_graph.is_mutually_recursive() {
                let dec_idx = call_graph.find_decreasing_arg(name).unwrap_or(0);
                self.decreasing_args
                    .entry(name.clone())
                    .or_default()
                    .push(dec_idx);
            }
        }
        Ok(())
    }
    /// Encode the definitions using well-founded recursion.
    ///
    /// Transforms the mutual block into a form that uses `WellFounded.fix`
    /// (or equivalent) to justify termination.
    #[allow(dead_code)]
    pub fn encode_as_wf_recursion(&self) -> Result<MutualBlock, MutualElabError> {
        if self.measure.is_none() && self.rel.is_none() {
            return Err(MutualElabError::TerminationFailure(
                "well-founded recursion requires a measure or relation".to_string(),
            ));
        }
        let mut result = self.block.clone();
        let wf_rel: Expr = match (&self.measure, &self.rel) {
            (Some(m), _) => Expr::App(
                Box::new(Expr::Const(Name::str("Measure"), vec![])),
                Box::new(Expr::Const(m.clone(), vec![])),
            ),
            (None, Some(r)) => r.clone(),
            (None, None) => unreachable!("checked above"),
        };
        let wf_proof = Expr::App(
            Box::new(Expr::Const(Name::str("WellFounded.wf"), vec![])),
            Box::new(wf_rel.clone()),
        );
        let call_graph = CallGraph::build_from_block(&self.block);
        for name in &self.block.names {
            if !call_graph.is_self_recursive(name) {
                continue;
            }
            if let Some(body) = self.block.get_body(name) {
                let dec_idx = self
                    .decreasing_args
                    .get(name)
                    .and_then(|v| v.first())
                    .copied()
                    .unwrap_or(0);
                let rec_ty = self
                    .block
                    .types
                    .get(name)
                    .cloned()
                    .unwrap_or(Expr::Const(Name::str("_"), vec![]));
                let step = Expr::Lam(
                    oxilean_kernel::BinderInfo::Default,
                    name.clone(),
                    Box::new(rec_ty),
                    Box::new(body.clone()),
                );
                let init_arg = Expr::BVar(dec_idx as u32);
                let wrapped = Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("WellFounded.fix"), vec![])),
                            Box::new(wf_proof.clone()),
                        )),
                        Box::new(step),
                    )),
                    Box::new(init_arg),
                );
                result.bodies.insert(name.clone(), wrapped);
            }
            result
                .attrs
                .entry(name.clone())
                .or_default()
                .push(AttributeKind::Custom("_wf_rec".to_string()));
        }
        Ok(result)
    }
    /// Generate a termination proof obligation.
    ///
    /// Returns an expression representing the proof obligation
    /// that all recursive calls decrease under the measure.
    #[allow(dead_code)]
    pub fn generate_termination_proof(&self) -> Result<Expr, MutualElabError> {
        if self.measure.is_some() || self.rel.is_some() {
            Ok(Expr::Const(Name::str("sorry"), vec![]))
        } else {
            Err(MutualElabError::TerminationFailure(
                "no measure or relation provided".to_string(),
            ))
        }
    }
}
/// A dependency graph over declaration names for cycle detection.
#[derive(Clone, Debug, Default)]
pub struct DeclDependencyGraph {
    /// Ordered list of declaration names.
    names: Vec<Name>,
    /// Adjacency list: `edges[i]` contains indices j such that decl[i] calls decl[j].
    edges: Vec<Vec<usize>>,
}
impl DeclDependencyGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a declaration node.  Returns its index.
    pub fn add_node(&mut self, name: Name) -> usize {
        let idx = self.names.len();
        self.names.push(name);
        self.edges.push(Vec::new());
        idx
    }
    /// Add a directed dependency edge `from -> to`.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if !self.edges[from].contains(&to) {
            self.edges[from].push(to);
        }
    }
    /// Return the index of a declaration by name.
    pub fn index_of(&self, name: &Name) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }
    /// Compute all SCCs.
    pub fn sccs(&self) -> Vec<Vec<Name>> {
        let raw = tarjan_scc(self.names.len(), &self.edges);
        raw.into_iter()
            .map(|scc| scc.iter().map(|&i| self.names[i].clone()).collect())
            .collect()
    }
    /// Return `true` if any SCC has more than one node (true cycle).
    pub fn has_cycle(&self) -> bool {
        self.sccs().iter().any(|scc| scc.len() > 1)
    }
    /// Return all cyclic SCCs (those with more than one node).
    pub fn cyclic_sccs(&self) -> Vec<Vec<Name>> {
        self.sccs().into_iter().filter(|s| s.len() > 1).collect()
    }
    /// Topological order of nodes (only valid when `has_cycle()` is false).
    pub fn topological_order(&self) -> Vec<Name> {
        let sccs = self.sccs();
        sccs.into_iter().flatten().collect()
    }
    /// Number of nodes in the graph.
    pub fn num_nodes(&self) -> usize {
        self.names.len()
    }
}
/// High-level cycle detector for mutual definition groups.
///
/// Given a set of definitions and their direct call dependencies, this
/// struct computes which definitions form genuine mutual recursion cycles
/// and which are merely co-defined but not mutually recursive.
#[derive(Debug, Default)]
pub struct MutualDefCycleDetector {
    graph: DeclDependencyGraph,
}
impl MutualDefCycleDetector {
    /// Create an empty detector.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a declaration name.  Returns its graph index.
    pub fn register(&mut self, name: Name) -> usize {
        self.graph.add_node(name)
    }
    /// Declare that `caller` directly uses `callee`.
    ///
    /// Both names must have been registered first.
    pub fn add_dependency(&mut self, caller: &Name, callee: &Name) -> bool {
        match (self.graph.index_of(caller), self.graph.index_of(callee)) {
            (Some(from), Some(to)) => {
                self.graph.add_edge(from, to);
                true
            }
            _ => false,
        }
    }
    /// Check whether there are any non-trivial mutual recursion cycles.
    pub fn has_mutual_recursion(&self) -> bool {
        self.graph.has_cycle()
    }
    /// Return all groups of mutually recursive declarations.
    pub fn mutual_groups(&self) -> Vec<Vec<Name>> {
        self.graph.cyclic_sccs()
    }
    /// Return the topological order for the declarations (deepest dependencies
    /// first), only meaningful if no cycles exist.
    pub fn elaboration_order(&self) -> Vec<Name> {
        self.graph.topological_order()
    }
    /// Number of registered declarations.
    pub fn num_decls(&self) -> usize {
        self.graph.num_nodes()
    }
}
/// Stages of the mutual-definition elaboration pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MutualElabStage {
    /// Initial collection of signatures.
    SigCollection,
    /// Dependency analysis and SCC computation.
    DependencyAnalysis,
    /// Body elaboration.
    BodyElab,
    /// Termination checking.
    TerminationCheck,
    /// Post-processing (wf-encode, add to env).
    PostProcess,
    /// Complete.
    Done,
}
/// A partially-known signature for a mutually recursive function.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PartialSig {
    /// Function name.
    pub name: Name,
    /// Declared type (if any).
    pub declared_type: Option<Expr>,
    /// Inferred type (filled in during elaboration).
    pub inferred_type: Option<Expr>,
    /// Whether the signature is fully resolved.
    pub resolved: bool,
}
#[allow(dead_code)]
impl PartialSig {
    /// Create a new partial signature with only a name.
    pub fn new(name: Name) -> Self {
        Self {
            name,
            declared_type: None,
            inferred_type: None,
            resolved: false,
        }
    }
    /// Mark the signature as resolved.
    pub fn resolve(&mut self, ty: Expr) {
        self.inferred_type = Some(ty);
        self.resolved = true;
    }
    /// Return the best available type (inferred > declared > None).
    pub fn best_type(&self) -> Option<&Expr> {
        self.inferred_type.as_ref().or(self.declared_type.as_ref())
    }
}
/// A budget for mutual elaboration (limits recursion/unfolding).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MutualElabBudget {
    /// Maximum allowed SCC size.
    pub max_scc_size: usize,
    /// Maximum depth for termination checking.
    pub max_termination_depth: usize,
    /// Maximum number of structural recursion arguments to check.
    pub max_structural_args: usize,
    /// Maximum number of refinement iterations.
    pub max_refinements: usize,
}
#[allow(dead_code)]
impl MutualElabBudget {
    /// Create a budget with default limits.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a liberal budget (for debugging).
    pub fn liberal() -> Self {
        Self {
            max_scc_size: 256,
            max_termination_depth: 1024,
            max_structural_args: 64,
            max_refinements: 32,
        }
    }
    /// Create a strict budget (for fast pre-checks).
    pub fn strict() -> Self {
        Self {
            max_scc_size: 8,
            max_termination_depth: 32,
            max_structural_args: 4,
            max_refinements: 2,
        }
    }
    /// Check if an SCC of size `n` is within budget.
    pub fn allows_scc_size(&self, n: usize) -> bool {
        n <= self.max_scc_size
    }
    /// Check if a termination depth `d` is within budget.
    pub fn allows_termination_depth(&self, d: usize) -> bool {
        d <= self.max_termination_depth
    }
}
/// A single recursive call found in a function body.
#[derive(Clone, Debug)]
pub struct RecursiveCall {
    /// Name of the calling function
    pub caller: Name,
    /// Name of the called function
    pub callee: Name,
    /// Relation of each argument to the corresponding parameter
    pub args: Vec<ArgRelation>,
}
/// Per-node metadata for Tarjan's algorithm.
#[derive(Debug, Clone, Default)]
pub struct TarjanNode {
    pub index: usize,
    pub lowlink: usize,
    pub on_stack: bool,
    pub discovered: bool,
}
/// Collection of partial signatures for a mutually recursive group.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct MutualSigCollection {
    sigs: Vec<PartialSig>,
}
#[allow(dead_code)]
impl MutualSigCollection {
    /// Create an empty collection.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a partial signature.
    pub fn add(&mut self, sig: PartialSig) {
        self.sigs.push(sig);
    }
    /// Return the number of signatures.
    pub fn len(&self) -> usize {
        self.sigs.len()
    }
    /// Return true if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.sigs.is_empty()
    }
    /// Return the number of resolved signatures.
    pub fn num_resolved(&self) -> usize {
        self.sigs.iter().filter(|s| s.resolved).count()
    }
    /// Return true if all signatures are resolved.
    pub fn all_resolved(&self) -> bool {
        self.sigs.iter().all(|s| s.resolved)
    }
    /// Look up a signature by name.
    pub fn get(&self, name: &Name) -> Option<&PartialSig> {
        self.sigs.iter().find(|s| &s.name == name)
    }
    /// Mutably look up a signature by name.
    pub fn get_mut(&mut self, name: &Name) -> Option<&mut PartialSig> {
        self.sigs.iter_mut().find(|s| &s.name == name)
    }
    /// Iterate over all signatures.
    pub fn iter(&self) -> impl Iterator<Item = &PartialSig> {
        self.sigs.iter()
    }
}
/// Mutual recursion checker and elaborator.
pub struct MutualChecker {
    /// Current mutual block being checked
    current_block: Option<MutualBlock>,
}
impl MutualChecker {
    /// Create a new mutual checker.
    pub fn new() -> Self {
        Self {
            current_block: None,
        }
    }
    /// Start a new mutual block.
    pub fn start_block(&mut self) {
        self.current_block = Some(MutualBlock::new());
    }
    /// Add a definition to the current block.
    pub fn add_def(&mut self, name: Name, ty: Expr, body: Expr) -> Result<(), String> {
        if let Some(block) = &mut self.current_block {
            block.add(name, ty, body);
            Ok(())
        } else {
            Err("No mutual block started".to_string())
        }
    }
    /// Finish the current mutual block.
    pub fn finish_block(&mut self) -> Result<MutualBlock, String> {
        self.current_block
            .take()
            .ok_or_else(|| "No mutual block to finish".to_string())
    }
    /// Get the current block (if any).
    pub fn current_block(&self) -> Option<&MutualBlock> {
        self.current_block.as_ref()
    }
    /// Check well-formedness of a mutual block.
    ///
    /// Validates:
    /// - All types are present
    /// - All bodies are present
    /// - No duplicate names
    #[allow(dead_code)]
    pub fn check_well_formedness(block: &MutualBlock) -> Result<(), MutualElabError> {
        block.validate()?;
        let block_names: HashSet<Name> = block.names.iter().cloned().collect();
        for name in &block.names {
            if let Some(ty) = block.get_type(name) {
                Self::check_no_external_forward_refs(ty, &block_names)?;
            }
        }
        Ok(())
    }
    /// Check that an expression does not reference undefined names
    /// outside the mutual block.
    fn check_no_external_forward_refs(
        expr: &Expr,
        block_names: &HashSet<Name>,
    ) -> Result<(), MutualElabError> {
        match expr {
            Expr::Const(name, _) => {
                let _ = block_names.contains(name);
                Ok(())
            }
            Expr::App(f, a) => {
                Self::check_no_external_forward_refs(f, block_names)?;
                Self::check_no_external_forward_refs(a, block_names)?;
                Ok(())
            }
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                Self::check_no_external_forward_refs(ty, block_names)?;
                Self::check_no_external_forward_refs(body, block_names)?;
                Ok(())
            }
            Expr::Let(_, ty, val, body) => {
                Self::check_no_external_forward_refs(ty, block_names)?;
                Self::check_no_external_forward_refs(val, block_names)?;
                Self::check_no_external_forward_refs(body, block_names)?;
                Ok(())
            }
            Expr::Proj(_, _, base) => {
                Self::check_no_external_forward_refs(base, block_names)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
    /// Check termination of a mutual block.
    ///
    /// Determines if the definitions are:
    /// - Non-recursive
    /// - Structurally recursive
    /// - Requiring well-founded recursion
    #[allow(dead_code)]
    pub fn check_termination(block: &MutualBlock) -> Result<TerminationKind, MutualElabError> {
        let call_graph = CallGraph::build_from_block(block);
        if !call_graph.is_recursive() {
            return Ok(TerminationKind::NonRecursive);
        }
        let mut structural_args = HashMap::new();
        let mut all_structural = true;
        for name in &block.names {
            if call_graph.is_self_recursive(name) || call_graph.is_mutually_recursive() {
                match call_graph.find_decreasing_arg(name) {
                    Some(idx) => {
                        structural_args.insert(name.clone(), idx);
                    }
                    None => {
                        all_structural = false;
                        break;
                    }
                }
            }
        }
        if all_structural && !structural_args.is_empty() {
            return Ok(TerminationKind::Structural(structural_args));
        }
        Ok(TerminationKind::WellFounded)
    }
    /// Elaborate a set of mutual definitions.
    ///
    /// This is the main entry point for mutual definition elaboration:
    /// 1. Forward-declare all names
    /// 2. Elaborate types
    /// 3. Elaborate bodies in extended context
    /// 4. Check termination
    #[allow(dead_code)]
    pub fn elaborate_mutual_defs(
        names: &[Name],
        types: &[Expr],
        bodies: &[Expr],
    ) -> Result<MutualBlock, MutualElabError> {
        if names.len() != types.len() || names.len() != bodies.len() {
            return Err(MutualElabError::Other(
                "mismatched lengths for names, types, and bodies".to_string(),
            ));
        }
        if names.is_empty() {
            return Err(MutualElabError::Other("empty mutual block".to_string()));
        }
        let mut block = MutualBlock::new();
        for i in 0..names.len() {
            block.add(names[i].clone(), types[i].clone(), bodies[i].clone());
        }
        block.validate()?;
        Ok(block)
    }
    /// Encode recursion in a mutual block based on the termination kind.
    #[allow(dead_code)]
    pub fn encode_recursion(
        block: MutualBlock,
        kind: &TerminationKind,
    ) -> Result<MutualBlock, MutualElabError> {
        match kind {
            TerminationKind::NonRecursive => Ok(block),
            TerminationKind::Structural(_args) => {
                let mut sr = StructuralRecursion::new(block);
                sr.detect_structural_recursion()?;
                sr.encode_as_recursor_application()
            }
            TerminationKind::WellFounded => {
                let mut wfr = WellFoundedRecursion::new(block);
                wfr.detect_decreasing_args()?;
                if wfr.measure.is_none() && wfr.rel.is_none() {
                    wfr.set_measure(Name::str("Nat.lt"));
                }
                wfr.encode_as_wf_recursion()
            }
        }
    }
    /// Split a mutual block into individual declarations.
    #[allow(dead_code)]
    pub fn split_mutual_block(block: &MutualBlock) -> Vec<Declaration> {
        let mut decls = Vec::new();
        for name in &block.names {
            if let (Some(ty), Some(val)) = (block.get_type(name), block.get_body(name)) {
                decls.push(Declaration::Definition {
                    name: name.clone(),
                    univ_params: block.univ_params.clone(),
                    ty: ty.clone(),
                    val: val.clone(),
                    hint: ReducibilityHint::Regular(100),
                });
            }
        }
        decls
    }
}
/// Error during mutual definition elaboration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MutualElabError {
    /// Type mismatch between declared and inferred type
    TypeMismatch(String),
    /// Invalid recursion pattern
    InvalidRecursion(String),
    /// A definition in the mutual block is missing
    MissingDefinition(String),
    /// Types form a cycle (not allowed without inductive)
    CyclicType(String),
    /// Failed to prove termination
    TerminationFailure(String),
    /// Other error
    Other(String),
}
/// Describes a well-founded ordering on terms used for termination proofs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WellFoundedOrder {
    /// Lexicographic ordering on a tuple of arguments.
    Lexicographic(Vec<usize>),
    /// Measure function applied to a single argument.
    Measure(usize),
    /// Structural recursion on a specific argument position.
    Structural(usize),
    /// Multiset ordering.
    Multiset(Vec<usize>),
    /// Unknown / unresolved ordering.
    Unknown,
}
/// Tracks the progress of the mutual-elaboration pipeline.
#[derive(Debug, Clone)]
pub struct MutualElabProgress {
    /// Names being elaborated.
    pub names: Vec<Name>,
    /// Current stage.
    pub stage: MutualElabStage,
    /// Stages that have been completed.
    pub completed: Vec<MutualElabStage>,
    /// Any error encountered.
    pub error: Option<MutualElabError>,
}
impl MutualElabProgress {
    /// Create progress for the given names starting at `SigCollection`.
    pub fn new(names: Vec<Name>) -> Self {
        Self {
            names,
            stage: MutualElabStage::SigCollection,
            completed: Vec::new(),
            error: None,
        }
    }
    /// Advance to the next stage.
    pub fn advance(&mut self) {
        let next = match self.stage {
            MutualElabStage::SigCollection => MutualElabStage::DependencyAnalysis,
            MutualElabStage::DependencyAnalysis => MutualElabStage::BodyElab,
            MutualElabStage::BodyElab => MutualElabStage::TerminationCheck,
            MutualElabStage::TerminationCheck => MutualElabStage::PostProcess,
            MutualElabStage::PostProcess => MutualElabStage::Done,
            MutualElabStage::Done => MutualElabStage::Done,
        };
        self.completed.push(self.stage);
        self.stage = next;
    }
    /// Mark the elaboration as failed with the given error.
    pub fn fail(&mut self, err: MutualElabError) {
        self.error = Some(err);
        self.stage = MutualElabStage::Done;
    }
    /// Return `true` if elaboration has completed (either successfully or with error).
    pub fn is_done(&self) -> bool {
        self.stage == MutualElabStage::Done
    }
    /// Return `true` if elaboration succeeded (done, no error).
    pub fn is_success(&self) -> bool {
        self.is_done() && self.error.is_none()
    }
}
/// A summary of the mutual recursion analysis for a block of definitions.
#[derive(Clone, Debug)]
pub struct MutualRecursionSummary {
    /// Names in this block.
    pub names: Vec<Name>,
    /// Whether the block contains genuine mutual recursion.
    pub is_mutually_recursive: bool,
    /// Detected mutual groups.
    pub mutual_groups: Vec<Vec<Name>>,
    /// Inferred termination measure, if any.
    pub termination_measure: Option<TerminationMeasure>,
    /// Diagnostics accumulated during analysis.
    pub diagnostics: Vec<String>,
}
impl MutualRecursionSummary {
    /// Create a summary from a cycle detector and a termination measure.
    pub fn from_detector(
        detector: &MutualDefCycleDetector,
        measure: Option<TerminationMeasure>,
    ) -> Self {
        let groups = detector.mutual_groups();
        let is_mutual = !groups.is_empty();
        Self {
            names: (0..detector.num_decls())
                .filter_map(|i| detector.graph.names.get(i).cloned())
                .collect(),
            is_mutually_recursive: is_mutual,
            mutual_groups: groups,
            termination_measure: measure,
            diagnostics: Vec::new(),
        }
    }
    /// Add a diagnostic message.
    pub fn add_diagnostic(&mut self, msg: impl Into<String>) {
        self.diagnostics.push(msg.into());
    }
    /// Return `true` if the analysis produced any diagnostics.
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }
}
/// A heuristic termination measure inference result.
#[derive(Clone, Debug)]
pub struct TerminationMeasure {
    /// The inferred well-founded ordering.
    pub order: WellFoundedOrder,
    /// Confidence score in [0, 1].
    pub confidence: f64,
    /// Human-readable justification.
    pub justification: String,
}
impl TerminationMeasure {
    /// Create a measure with full confidence.
    pub fn certain(order: WellFoundedOrder, justification: impl Into<String>) -> Self {
        Self {
            order,
            confidence: 1.0,
            justification: justification.into(),
        }
    }
    /// Create a measure with partial confidence.
    pub fn heuristic(
        order: WellFoundedOrder,
        confidence: f64,
        justification: impl Into<String>,
    ) -> Self {
        Self {
            order,
            confidence,
            justification: justification.into(),
        }
    }
    /// Return `true` if this measure is judged reliable.
    pub fn is_reliable(&self) -> bool {
        self.confidence >= 0.8
    }
}
/// How a recursive definition terminates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminationKind {
    /// Structurally recursive on the given argument index per function
    Structural(HashMap<Name, usize>),
    /// Well-founded recursion (requires a measure/relation)
    WellFounded,
    /// Not recursive at all
    NonRecursive,
}
/// A mutually recursive block of definitions.
#[derive(Debug, Clone)]
pub struct MutualBlock {
    /// Names of all definitions in this mutual block (in declaration order)
    pub names: Vec<Name>,
    /// Types of all definitions
    pub types: HashMap<Name, Expr>,
    /// Bodies of all definitions
    pub bodies: HashMap<Name, Expr>,
    /// Universe parameters shared by all definitions
    pub univ_params: Vec<Name>,
    /// Attributes per definition
    pub attrs: HashMap<Name, Vec<AttributeKind>>,
    /// Whether each definition is noncomputable
    pub is_noncomputable: HashMap<Name, bool>,
}
impl MutualBlock {
    /// Create a new mutual block.
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            types: HashMap::new(),
            bodies: HashMap::new(),
            univ_params: Vec::new(),
            attrs: HashMap::new(),
            is_noncomputable: HashMap::new(),
        }
    }
    /// Add a definition to the mutual block.
    pub fn add(&mut self, name: Name, ty: Expr, body: Expr) {
        self.names.push(name.clone());
        self.types.insert(name.clone(), ty);
        self.bodies.insert(name, body);
    }
    /// Add a definition with attributes and noncomputable flag.
    #[allow(dead_code)]
    pub fn add_with_attrs(
        &mut self,
        name: Name,
        ty: Expr,
        body: Expr,
        attrs: Vec<AttributeKind>,
        noncomputable: bool,
    ) {
        self.names.push(name.clone());
        self.types.insert(name.clone(), ty);
        self.bodies.insert(name.clone(), body);
        self.attrs.insert(name.clone(), attrs);
        self.is_noncomputable.insert(name, noncomputable);
    }
    /// Get the type of a definition.
    pub fn get_type(&self, name: &Name) -> Option<&Expr> {
        self.types.get(name)
    }
    /// Get the body of a definition.
    pub fn get_body(&self, name: &Name) -> Option<&Expr> {
        self.bodies.get(name)
    }
    /// Get the number of definitions in this block.
    pub fn size(&self) -> usize {
        self.names.len()
    }
    /// Check if a name is in this mutual block.
    pub fn contains(&self, name: &Name) -> bool {
        self.names.contains(name)
    }
    /// Get names in declaration order.
    #[allow(dead_code)]
    pub fn names_in_order(&self) -> &[Name] {
        &self.names
    }
    /// Get all (name, body) pairs.
    #[allow(dead_code)]
    pub fn get_all_bodies(&self) -> Vec<(&Name, &Expr)> {
        self.names
            .iter()
            .filter_map(|name| self.bodies.get(name).map(|body| (name, body)))
            .collect()
    }
    /// Validate the mutual block: every name must have both a type and a body.
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), MutualElabError> {
        if self.names.is_empty() {
            return Err(MutualElabError::Other("empty mutual block".to_string()));
        }
        let mut seen = HashSet::new();
        for name in &self.names {
            if !seen.insert(name.clone()) {
                return Err(MutualElabError::Other(format!(
                    "duplicate name in mutual block: {:?}",
                    name
                )));
            }
        }
        for name in &self.names {
            if !self.types.contains_key(name) {
                return Err(MutualElabError::MissingDefinition(format!(
                    "no type for '{:?}'",
                    name
                )));
            }
        }
        for name in &self.names {
            if !self.bodies.contains_key(name) {
                return Err(MutualElabError::MissingDefinition(format!(
                    "no body for '{:?}'",
                    name
                )));
            }
        }
        Ok(())
    }
    /// Set universe parameters for the entire block.
    #[allow(dead_code)]
    pub fn set_univ_params(&mut self, params: Vec<Name>) {
        self.univ_params = params;
    }
    /// Set attributes for a specific definition.
    #[allow(dead_code)]
    pub fn set_attrs(&mut self, name: &Name, attrs: Vec<AttributeKind>) {
        self.attrs.insert(name.clone(), attrs);
    }
    /// Mark a definition as noncomputable.
    #[allow(dead_code)]
    pub fn set_noncomputable(&mut self, name: &Name, noncomputable: bool) {
        self.is_noncomputable.insert(name.clone(), noncomputable);
    }
    /// Check if a definition is noncomputable.
    #[allow(dead_code)]
    pub fn is_def_noncomputable(&self, name: &Name) -> bool {
        self.is_noncomputable.get(name).copied().unwrap_or(false)
    }
    /// Get the attributes for a definition.
    #[allow(dead_code)]
    pub fn get_attrs(&self, name: &Name) -> &[AttributeKind] {
        self.attrs.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
}
/// Encoder for structurally recursive definitions.
///
/// Transforms structurally recursive definitions into applications
/// of the recursor (eliminator) for the decreasing argument's type.
#[derive(Clone, Debug)]
pub struct StructuralRecursion {
    /// The mutual block being processed
    pub block: MutualBlock,
    /// For each function, which argument indices are recursive
    pub recursive_args: HashMap<Name, Vec<usize>>,
}
impl StructuralRecursion {
    /// Create a new structural recursion encoder.
    #[allow(dead_code)]
    pub fn new(block: MutualBlock) -> Self {
        Self {
            block,
            recursive_args: HashMap::new(),
        }
    }
    /// Detect which arguments are structurally decreasing.
    #[allow(dead_code)]
    pub fn detect_structural_recursion(&mut self) -> Result<(), MutualElabError> {
        let call_graph = CallGraph::build_from_block(&self.block);
        for name in &self.block.names {
            if call_graph.is_self_recursive(name) {
                match call_graph.find_decreasing_arg(name) {
                    Some(idx) => {
                        self.recursive_args
                            .entry(name.clone())
                            .or_default()
                            .push(idx);
                    }
                    None => {
                        return Err(MutualElabError::TerminationFailure(format!(
                            "could not find structurally decreasing argument for '{:?}'",
                            name
                        )));
                    }
                }
            }
        }
        Ok(())
    }
    /// Encode the structural recursion as recursor applications.
    ///
    /// In a full implementation, this would replace each recursive function
    /// with an application of the appropriate recursor/eliminator.
    #[allow(dead_code)]
    pub fn encode_as_recursor_application(&self) -> Result<MutualBlock, MutualElabError> {
        let mut result = self.block.clone();
        let call_graph = CallGraph::build_from_block(&self.block);
        for name in &self.block.names {
            if call_graph.is_self_recursive(name) && !self.recursive_args.contains_key(name) {
                return Err(MutualElabError::TerminationFailure(format!(
                    "no structural recursion info for '{:?}'",
                    name
                )));
            }
        }
        for (name, args) in &self.recursive_args {
            let attr_name = format!(
                "_rec_arg_{}",
                args.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("_")
            );
            result
                .attrs
                .entry(name.clone())
                .or_default()
                .push(AttributeKind::Custom(attr_name));
        }
        Ok(result)
    }
    /// Get the detected recursive arguments.
    #[allow(dead_code)]
    pub fn get_recursive_args(&self) -> &HashMap<Name, Vec<usize>> {
        &self.recursive_args
    }
}
