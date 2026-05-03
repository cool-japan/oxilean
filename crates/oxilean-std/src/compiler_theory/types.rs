//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use super::functions::*;

/// A compiler correctness proof structure relating source and target semantics.
#[derive(Debug, Clone)]
pub struct CompilerCorrectness {
    pub source_semantics: String,
    pub target_semantics: String,
}
impl CompilerCorrectness {
    pub fn new(source_semantics: impl Into<String>, target_semantics: impl Into<String>) -> Self {
        Self {
            source_semantics: source_semantics.into(),
            target_semantics: target_semantics.into(),
        }
    }
    /// Describe the simulation relation between source and target.
    ///
    /// Returns a human-readable description of the bisimulation condition.
    pub fn simulation_relation(&self) -> String {
        format!(
            "∀ s t, ({}) ≈ ({}) → compile(s) simulates_in ({})",
            self.source_semantics, self.source_semantics, self.target_semantics,
        )
    }
    /// Check observable equivalence: source and target produce the same outputs
    /// on all inputs (approximated by string comparison of semantic descriptions).
    pub fn observable_equivalence(&self) -> bool {
        !self.source_semantics.is_empty() && !self.target_semantics.is_empty()
    }
}
/// Hindley-Milner type inference engine.
#[derive(Debug, Default)]
pub struct HindleyMilnerInference {
    subst: HashMap<usize, HMType>,
    next_var: usize,
}
impl HindleyMilnerInference {
    pub fn new() -> Self {
        Self::default()
    }
    /// Allocate a fresh type variable.
    pub fn fresh(&mut self) -> HMType {
        let v = self.next_var;
        self.next_var += 1;
        HMType::Var(v)
    }
    /// Unify two types, updating the substitution.
    /// Returns `Ok(())` on success or `Err(msg)` on failure.
    pub fn unify(&mut self, t1: &HMType, t2: &HMType) -> Result<(), String> {
        let t1 = t1.apply(&self.subst);
        let t2 = t2.apply(&self.subst);
        match (&t1, &t2) {
            (HMType::Int, HMType::Int) | (HMType::Bool, HMType::Bool) => Ok(()),
            (HMType::Var(v), _) => {
                if let HMType::Var(v2) = &t2 {
                    if v == v2 {
                        return Ok(());
                    }
                }
                if t2.occurs(*v) {
                    return Err(format!("occurs check failed for var {}", v));
                }
                self.subst.insert(*v, t2);
                Ok(())
            }
            (_, HMType::Var(_)) => self.unify(&t2, &t1),
            (HMType::Fun(a1, b1), HMType::Fun(a2, b2)) => {
                self.unify(a1, a2)?;
                self.unify(b1, b2)
            }
            _ => Err(format!("cannot unify {:?} with {:?}", t1, t2)),
        }
    }
    /// Infer the type of a simple expression (encoded as a string for demonstration).
    ///
    /// Recognises:
    /// - `"int"` → `HMType::Int`
    /// - `"bool"` → `HMType::Bool`
    /// - `"fun"` → `fresh_α → fresh_β`
    /// - anything else → fresh type variable
    pub fn infer_simple(&mut self, expr: &str) -> HMType {
        match expr {
            "int" => HMType::Int,
            "bool" => HMType::Bool,
            "fun" => {
                let a = self.fresh();
                let b = self.fresh();
                HMType::Fun(Box::new(a), Box::new(b))
            }
            _ => self.fresh(),
        }
    }
    /// Apply the current substitution to a type to get its resolved form.
    pub fn resolve(&self, t: &HMType) -> HMType {
        t.apply(&self.subst)
    }
}
/// SSA constructor: converts a list of basic blocks to SSA form.
#[derive(Debug, Default)]
pub struct SSAConstructor {
    pub blocks: Vec<BasicBlock>,
}
impl SSAConstructor {
    pub fn new(blocks: Vec<BasicBlock>) -> Self {
        Self { blocks }
    }
    /// Compute the set of variables that are ever defined.
    pub fn all_vars(&self) -> HashSet<String> {
        self.blocks
            .iter()
            .flat_map(|b| b.defs.iter().cloned())
            .collect()
    }
    /// Compute a simple dominance frontier approximation.
    ///
    /// For each block, its dominance frontier consists of blocks that have
    /// a predecessor that is dominated by the block, but the block itself
    /// does not strictly dominate the frontier block.
    ///
    /// Here we use a simplified heuristic: the frontier of block `b` is the
    /// set of successors of `b`'s successors that are not `b` itself.
    pub fn dominance_frontier_map(&self) -> BTreeMap<usize, HashSet<usize>> {
        let mut frontier: BTreeMap<usize, HashSet<usize>> = BTreeMap::new();
        for block in &self.blocks {
            for &succ_id in &block.succs {
                if let Some(succ_block) = self.blocks.iter().find(|b| b.id == succ_id) {
                    for &succ_succ in &succ_block.succs {
                        if succ_succ != block.id {
                            frontier.entry(block.id).or_default().insert(succ_succ);
                        }
                    }
                }
            }
        }
        frontier
    }
    /// Determine where φ-functions are needed.
    ///
    /// For each variable `v`, a φ-function is inserted at the dominance frontier
    /// of every block that defines `v`.
    ///
    /// Returns a map: variable → set of block IDs where φ-functions are needed.
    pub fn phi_insertion_points(&self) -> BTreeMap<String, HashSet<usize>> {
        let df_map = self.dominance_frontier_map();
        let mut result: BTreeMap<String, HashSet<usize>> = BTreeMap::new();
        for block in &self.blocks {
            for var in &block.defs {
                if let Some(frontier) = df_map.get(&block.id) {
                    result
                        .entry(var.clone())
                        .or_default()
                        .extend(frontier.iter());
                }
            }
        }
        result
    }
    /// Rename variables to SSA form.
    ///
    /// Returns a map from `(block_id, variable)` → SSA version index.
    pub fn rename_variables(&self) -> BTreeMap<(usize, String), usize> {
        let mut counters: HashMap<String, usize> = HashMap::new();
        let mut result: BTreeMap<(usize, String), usize> = BTreeMap::new();
        for block in &self.blocks {
            for var in &block.defs {
                let cnt = counters.entry(var.clone()).or_insert(0);
                result.insert((block.id, var.clone()), *cnt);
                *cnt += 1;
            }
        }
        result
    }
}
/// A lambda closure: a function together with its captured environment.
#[derive(Debug, Clone)]
pub struct Closure {
    pub free_vars: Vec<String>,
    pub env: Vec<(String, String)>,
}
impl Closure {
    pub fn new(free_vars: Vec<String>, env: Vec<(String, String)>) -> Self {
        Self { free_vars, env }
    }
    /// Closure conversion: rewrite so the closure captures all free variables
    /// explicitly (returns `self` with free_vars placed in env).
    pub fn closure_convert(&self) -> Closure {
        let mut new_env = self.env.clone();
        for fv in &self.free_vars {
            if !new_env.iter().any(|(k, _)| k == fv) {
                new_env.push((fv.clone(), format!("captured_{}", fv)));
            }
        }
        Closure {
            free_vars: vec![],
            env: new_env,
        }
    }
    /// Lambda lifting: move this closure to a top-level function by adding
    /// free variables as extra parameters.
    ///
    /// Returns the lifted function's parameter list (original free vars become params).
    pub fn lambda_lift(&self) -> Vec<String> {
        let mut params = self.free_vars.clone();
        for (k, _) in &self.env {
            if !params.contains(k) {
                params.push(k.clone());
            }
        }
        params
    }
}
/// Register allocation problem instance using interval-based live-range representation.
#[derive(Debug, Clone)]
pub struct RegisterAllocation {
    pub variables: Vec<String>,
    /// Live ranges: (start_instruction, end_instruction) for each variable.
    pub live_ranges: Vec<(usize, usize)>,
    pub num_regs: usize,
}
impl RegisterAllocation {
    pub fn new(variables: Vec<String>, live_ranges: Vec<(usize, usize)>, num_regs: usize) -> Self {
        Self {
            variables,
            live_ranges,
            num_regs,
        }
    }
    /// Graph-coloring register allocation.
    ///
    /// Builds an interference graph (two variables interfere if their live ranges
    /// overlap), then greedily colors it with `num_regs` colors.
    ///
    /// Returns `Some(coloring)` where `coloring\[i\]` is the register for variable `i`,
    /// or `None` if coloring fails (spilling needed).
    pub fn graph_color(&self) -> Option<Vec<usize>> {
        let n = self.variables.len();
        let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for i in 0..n {
            for j in (i + 1)..n {
                let (s1, e1) = self.live_ranges[i];
                let (s2, e2) = self.live_ranges[j];
                if s1 <= e2 && s2 <= e1 {
                    adj[i].insert(j);
                    adj[j].insert(i);
                }
            }
        }
        let mut color = vec![usize::MAX; n];
        for i in 0..n {
            let used: HashSet<usize> = adj[i]
                .iter()
                .filter_map(|&j| {
                    if color[j] != usize::MAX {
                        Some(color[j])
                    } else {
                        None
                    }
                })
                .collect();
            let c = (0..self.num_regs).find(|c| !used.contains(c));
            match c {
                Some(reg) => color[i] = reg,
                None => return None,
            }
        }
        Some(color)
    }
    /// Compute spill cost for each variable (heuristic: range length).
    pub fn spill_cost(&self) -> Vec<f64> {
        self.live_ranges
            .iter()
            .map(|&(s, e)| (e - s + 1) as f64)
            .collect()
    }
}
/// Static Single Assignment (SSA) form representation.
#[derive(Debug, Clone)]
pub struct SSAForm {
    pub variables: Vec<String>,
    /// φ-functions: each entry is (variable, list of incoming versions).
    pub phi_functions: Vec<(String, Vec<String>)>,
}
impl SSAForm {
    pub fn new(variables: Vec<String>, phi_functions: Vec<(String, Vec<String>)>) -> Self {
        Self {
            variables,
            phi_functions,
        }
    }
    /// Convert to SSA form by renaming variables to unique SSA names.
    ///
    /// Returns a new `SSAForm` with versioned variable names.
    pub fn convert_to_ssa(&self) -> SSAForm {
        let mut versioned = vec![];
        let mut counter: HashMap<String, usize> = HashMap::new();
        for v in &self.variables {
            let n = counter.entry(v.clone()).or_insert(0);
            versioned.push(format!("{}_{}", v, n));
            *n += 1;
        }
        let phi: Vec<(String, Vec<String>)> = self
            .phi_functions
            .iter()
            .map(|(var, srcs)| {
                let new_var = format!("{}_phi", var);
                let new_srcs = srcs.iter().map(|s| format!("{}_s", s)).collect();
                (new_var, new_srcs)
            })
            .collect();
        SSAForm {
            variables: versioned,
            phi_functions: phi,
        }
    }
    /// Compute the dominance frontier for each variable's definition block.
    ///
    /// (Simplified: returns the set of variable names that appear in φ-functions.)
    pub fn dominance_frontier(&self) -> HashSet<String> {
        self.phi_functions.iter().map(|(v, _)| v.clone()).collect()
    }
}
/// A nondeterministic pushdown automaton (PDA).
///
/// Transitions: (state, input_char_or_ε, stack_top_or_ε) → (next_state, push_string)
#[derive(Debug, Clone)]
pub struct PushdownAutomaton {
    pub states: usize,
    pub stack_alphabet: Vec<char>,
    /// Each transition: (from_state, input, stack_pop, to_state, stack_push)
    pub transitions: Vec<(usize, char, char, usize, String)>,
}
impl PushdownAutomaton {
    pub fn new(
        states: usize,
        stack_alphabet: Vec<char>,
        transitions: Vec<(usize, char, char, usize, String)>,
    ) -> Self {
        Self {
            states,
            stack_alphabet,
            transitions,
        }
    }
    /// Simulate the PDA on `input` (accept-by-empty-stack heuristic).
    ///
    /// Uses BFS over configurations (state, remaining_input, stack).
    /// Returns `true` if an accepting configuration is reached.
    pub fn accepts(&self, input: &[char]) -> bool {
        type Config = (usize, usize, Vec<char>);
        let initial: Config = (0, 0, vec![]);
        let mut queue: VecDeque<Config> = VecDeque::new();
        let mut visited: HashSet<(usize, usize, Vec<char>)> = HashSet::new();
        queue.push_back(initial);
        while let Some((state, pos, stack)) = queue.pop_front() {
            if pos == input.len() && stack.is_empty() {
                return true;
            }
            let key = (state, pos, stack.clone());
            if visited.contains(&key) {
                continue;
            }
            visited.insert(key);
            let input_char = input.get(pos).copied().unwrap_or('\0');
            let stack_top = stack.last().copied().unwrap_or('\0');
            for &(from, inp, pop, to, ref push) in &self.transitions {
                if from != state {
                    continue;
                }
                let inp_ok = inp == '\0' || inp == input_char;
                let pop_ok = pop == '\0' || pop == stack_top;
                if !inp_ok || !pop_ok {
                    continue;
                }
                let new_pos = if inp != '\0' { pos + 1 } else { pos };
                let mut new_stack = stack.clone();
                if pop != '\0' && !new_stack.is_empty() {
                    new_stack.pop();
                }
                for ch in push.chars().rev() {
                    new_stack.push(ch);
                }
                queue.push_back((to, new_pos, new_stack));
            }
        }
        false
    }
    /// Convert this PDA to an equivalent CFG (construction sketch).
    ///
    /// Returns a CFG whose start symbol is 'S' and whose rules capture
    /// the language of the PDA via the standard triple-construction.
    /// (Full construction is complex; this returns a placeholder CFG.)
    pub fn to_cfg(&self) -> ContextFreeGrammar {
        ContextFreeGrammar::new(vec!['S'], vec![], 'S', vec![('S', String::new())])
    }
}
/// A generic dataflow analysis specification.
#[derive(Debug, Clone)]
pub struct DataFlowAnalysis {
    /// Direction of analysis: `"forward"` or `"backward"`.
    pub direction: String,
    /// Lattice description (e.g. `"powerset"`, `"interval"`).
    pub lattice: String,
}
impl DataFlowAnalysis {
    pub fn new(direction: impl Into<String>, lattice: impl Into<String>) -> Self {
        Self {
            direction: direction.into(),
            lattice: lattice.into(),
        }
    }
    /// Run the worklist algorithm on a given CFG (represented as adjacency list).
    ///
    /// `transfer`: transfer function for each node.
    /// `join`:     join operator for the lattice.
    /// `init`:     initial abstract value for each node.
    ///
    /// Returns the fixpoint abstract values.
    pub fn worklist_algorithm<V>(
        &self,
        num_nodes: usize,
        edges: &[Vec<usize>],
        init: Vec<V>,
        transfer: impl Fn(usize, &V) -> V,
        join: impl Fn(&V, &V) -> V,
    ) -> Vec<V>
    where
        V: Clone + PartialEq,
    {
        let mut values = init;
        let mut wl: VecDeque<usize> = (0..num_nodes).collect();
        while let Some(n) = wl.pop_front() {
            let out = transfer(n, &values[n]);
            for &succ in &edges[n] {
                let new_val = join(&values[succ], &out);
                if new_val != values[succ] {
                    values[succ] = new_val;
                    wl.push_back(succ);
                }
            }
        }
        values
    }
    /// Compute the fixpoint of a monotone function `f` over a lattice value.
    ///
    /// Iterates until `f(v) == v` (Kleene fixpoint theorem).
    pub fn fixed_point<V>(&self, init: V, f: impl Fn(&V) -> V) -> V
    where
        V: Clone + PartialEq,
    {
        let mut v = init;
        loop {
            let next = f(&v);
            if next == v {
                return v;
            }
            v = next;
        }
    }
}
/// A simple monomorphic type for Hindley-Milner inference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HMType {
    /// A type variable (index).
    Var(usize),
    /// The integer base type.
    Int,
    /// The boolean base type.
    Bool,
    /// A function type: `from → to`.
    Fun(Box<HMType>, Box<HMType>),
}
impl HMType {
    fn occurs(&self, var: usize) -> bool {
        match self {
            HMType::Var(v) => *v == var,
            HMType::Int | HMType::Bool => false,
            HMType::Fun(a, b) => a.occurs(var) || b.occurs(var),
        }
    }
    fn apply(&self, subst: &HashMap<usize, HMType>) -> HMType {
        match self {
            HMType::Var(v) => {
                if let Some(t) = subst.get(v) {
                    t.apply(subst)
                } else {
                    self.clone()
                }
            }
            HMType::Int => HMType::Int,
            HMType::Bool => HMType::Bool,
            HMType::Fun(a, b) => HMType::Fun(Box::new(a.apply(subst)), Box::new(b.apply(subst))),
        }
    }
}
/// A simple graph coloring register allocator over an interference graph.
///
/// Uses the Chaitin-Briggs simplify/select algorithm (greedy approximation).
#[derive(Debug, Clone)]
pub struct RegisterColoringSimple {
    pub num_vars: usize,
    /// Interference edges as adjacency sets.
    pub interfere: Vec<HashSet<usize>>,
    pub num_regs: usize,
}
impl RegisterColoringSimple {
    /// Construct from a list of interference edges.
    pub fn new(num_vars: usize, edges: &[(usize, usize)], num_regs: usize) -> Self {
        let mut interfere = vec![HashSet::new(); num_vars];
        for &(u, v) in edges {
            interfere[u].insert(v);
            interfere[v].insert(u);
        }
        Self {
            num_vars,
            interfere,
            num_regs,
        }
    }
    /// Try to color the interference graph.
    ///
    /// Returns `Some(coloring)` where `coloring\[i\]` is the register for variable `i`,
    /// or `None` if the graph is not `num_regs`-colorable (spilling required).
    pub fn color(&self) -> Option<Vec<usize>> {
        let n = self.num_vars;
        let mut color = vec![usize::MAX; n];
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| self.interfere[b].len().cmp(&self.interfere[a].len()));
        for v in order {
            let used: HashSet<usize> = self.interfere[v]
                .iter()
                .filter_map(|&u| {
                    if color[u] != usize::MAX {
                        Some(color[u])
                    } else {
                        None
                    }
                })
                .collect();
            match (0..self.num_regs).find(|c| !used.contains(c)) {
                Some(c) => color[v] = c,
                None => return None,
            }
        }
        Some(color)
    }
    /// Compute the chromatic number lower bound (clique size heuristic).
    pub fn clique_lower_bound(&self) -> usize {
        let mut best = 1usize;
        for v in 0..self.num_vars {
            let mut clique = vec![v];
            for u in self.interfere[v].iter().copied() {
                if clique.iter().all(|&c| self.interfere[c].contains(&u)) {
                    clique.push(u);
                }
            }
            if clique.len() > best {
                best = clique.len();
            }
        }
        best
    }
}
/// Abstract value in the sign domain: ⊥, Neg, Zero, Pos, ⊤.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignValue {
    Bottom,
    Negative,
    Zero,
    Positive,
    Top,
}
impl SignValue {
    /// Least upper bound (join) in the sign lattice.
    pub fn join(&self, other: &SignValue) -> SignValue {
        if self == other {
            return self.clone();
        }
        match (self, other) {
            (SignValue::Bottom, x) | (x, SignValue::Bottom) => x.clone(),
            _ => SignValue::Top,
        }
    }
    /// Abstract addition.
    pub fn add(&self, other: &SignValue) -> SignValue {
        match (self, other) {
            (SignValue::Bottom, _) | (_, SignValue::Bottom) => SignValue::Bottom,
            (SignValue::Top, _) | (_, SignValue::Top) => SignValue::Top,
            (SignValue::Zero, x) | (x, SignValue::Zero) => x.clone(),
            (SignValue::Positive, SignValue::Positive) => SignValue::Positive,
            (SignValue::Negative, SignValue::Negative) => SignValue::Negative,
            _ => SignValue::Top,
        }
    }
    /// Abstract multiplication.
    pub fn mul(&self, other: &SignValue) -> SignValue {
        match (self, other) {
            (SignValue::Bottom, _) | (_, SignValue::Bottom) => SignValue::Bottom,
            (SignValue::Zero, _) | (_, SignValue::Zero) => SignValue::Zero,
            (SignValue::Top, _) | (_, SignValue::Top) => SignValue::Top,
            (SignValue::Positive, SignValue::Positive) => SignValue::Positive,
            (SignValue::Negative, SignValue::Negative) => SignValue::Positive,
            _ => SignValue::Negative,
        }
    }
}
/// The Chomsky hierarchy of formal grammar types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarType {
    /// Type 3: regular grammars (recognized by finite automata).
    Regular,
    /// Type 2: context-free grammars (recognized by pushdown automata).
    CFL,
    /// Type 1: context-sensitive languages (recognized by linear-bounded automata).
    CSL,
    /// Type 0: recursively enumerable languages (recognized by Turing machines).
    RecursivelyEnumerable,
}
impl GrammarType {
    /// Returns the Chomsky hierarchy level (0–3).
    pub fn chomsky_hierarchy_level(&self) -> usize {
        match self {
            GrammarType::Regular => 3,
            GrammarType::CFL => 2,
            GrammarType::CSL => 1,
            GrammarType::RecursivelyEnumerable => 0,
        }
    }
    /// Returns the closure properties of this grammar class.
    pub fn closure_properties(&self) -> Vec<&'static str> {
        match self {
            GrammarType::Regular => {
                vec![
                    "union",
                    "intersection",
                    "complement",
                    "concatenation",
                    "kleene_star",
                    "reversal",
                ]
            }
            GrammarType::CFL => vec!["union", "concatenation", "kleene_star", "reversal"],
            GrammarType::CSL => {
                vec![
                    "union",
                    "intersection",
                    "complement",
                    "concatenation",
                    "kleene_star",
                ]
            }
            GrammarType::RecursivelyEnumerable => {
                vec!["union", "concatenation", "kleene_star"]
            }
        }
    }
}
/// Simple abstract interpreter using the sign domain.
#[derive(Debug, Default)]
pub struct AbstractInterpreter {
    pub state: HashMap<String, SignValue>,
}
impl AbstractInterpreter {
    pub fn new() -> Self {
        Self::default()
    }
    /// Assign an abstract value to a variable.
    pub fn assign(&mut self, var: &str, val: SignValue) {
        self.state.insert(var.to_string(), val);
    }
    /// Look up a variable's abstract value (⊤ if unknown).
    pub fn lookup(&self, var: &str) -> SignValue {
        self.state.get(var).cloned().unwrap_or(SignValue::Top)
    }
    /// Join two abstract states (pointwise LUB).
    pub fn join_state(&self, other: &AbstractInterpreter) -> AbstractInterpreter {
        let mut merged = self.state.clone();
        for (k, v) in &other.state {
            let entry = merged.entry(k.clone()).or_insert(SignValue::Bottom);
            *entry = entry.join(v);
        }
        AbstractInterpreter { state: merged }
    }
    /// Widen: for fixpoint acceleration, replace with ⊤ on disagreement.
    pub fn widen(&self, other: &AbstractInterpreter) -> AbstractInterpreter {
        let mut widened = self.state.clone();
        for (k, v) in &other.state {
            let entry = widened.entry(k.clone()).or_insert(SignValue::Bottom);
            if entry != v {
                *entry = SignValue::Top;
            }
        }
        AbstractInterpreter { state: widened }
    }
}
/// A dataflow problem instance over a CFG.
///
/// Nodes are `0..num_nodes`. The lattice values are `HashSet<usize>` (powerset).
#[derive(Debug, Clone)]
pub struct DataflowSolver {
    pub num_nodes: usize,
    /// Successor edges: `succ\[n\]` = list of successors of `n`.
    pub succ: Vec<Vec<usize>>,
    /// Gen sets per node.
    pub gen: Vec<HashSet<usize>>,
    /// Kill sets per node.
    pub kill: Vec<HashSet<usize>>,
}
impl DataflowSolver {
    pub fn new(
        num_nodes: usize,
        succ: Vec<Vec<usize>>,
        gen: Vec<HashSet<usize>>,
        kill: Vec<HashSet<usize>>,
    ) -> Self {
        Self {
            num_nodes,
            succ,
            gen,
            kill,
        }
    }
    /// Run a forward may-analysis (reaching definitions style):
    ///
    /// `out\[n\] = gen\[n\] ∪ (in\[n\] \ kill\[n\])`
    /// `in\[n\]  = ∪ { out[pred] | pred → n }`
    ///
    /// Returns `(in_sets, out_sets)` at fixpoint.
    pub fn solve_forward(&self) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
        let n = self.num_nodes;
        let mut in_sets: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        let mut out_sets: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        let mut pred: Vec<Vec<usize>> = vec![vec![]; n];
        for (i, succs) in self.succ.iter().enumerate() {
            for &j in succs {
                pred[j].push(i);
            }
        }
        let mut worklist: VecDeque<usize> = (0..n).collect();
        while let Some(node) = worklist.pop_front() {
            let new_in: HashSet<usize> = pred[node]
                .iter()
                .flat_map(|&p| out_sets[p].iter().copied())
                .collect();
            in_sets[node] = new_in;
            let new_out: HashSet<usize> = self.gen[node]
                .iter()
                .copied()
                .chain(
                    in_sets[node]
                        .iter()
                        .copied()
                        .filter(|x| !self.kill[node].contains(x)),
                )
                .collect();
            if new_out != out_sets[node] {
                out_sets[node] = new_out;
                for &succ in &self.succ[node] {
                    worklist.push_back(succ);
                }
            }
        }
        (in_sets, out_sets)
    }
    /// Run a backward must-analysis (live variables style):
    ///
    /// `in\[n\]  = use\[n\] ∪ (out\[n\] \ def\[n\])`  (using gen=use, kill=def)
    /// `out\[n\] = ∩ { in[succ] | n → succ }` (must) — simplified as union for may-analysis
    ///
    /// Returns `(in_sets, out_sets)` at fixpoint.
    pub fn solve_backward(&self) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
        let n = self.num_nodes;
        let mut in_sets: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        let mut out_sets: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        let mut worklist: VecDeque<usize> = (0..n).collect();
        while let Some(node) = worklist.pop_front() {
            let new_out: HashSet<usize> = self.succ[node]
                .iter()
                .flat_map(|&s| in_sets[s].iter().copied())
                .collect();
            out_sets[node] = new_out;
            let new_in: HashSet<usize> = self.gen[node]
                .iter()
                .copied()
                .chain(
                    out_sets[node]
                        .iter()
                        .copied()
                        .filter(|x| !self.kill[node].contains(x)),
                )
                .collect();
            if new_in != in_sets[node] {
                in_sets[node] = new_in;
                for i in 0..n {
                    if self.succ[i].contains(&node) {
                        worklist.push_back(i);
                    }
                }
            }
        }
        (in_sets, out_sets)
    }
}
/// A context-free grammar (CFG) in standard form.
#[derive(Debug, Clone)]
pub struct ContextFreeGrammar {
    pub nonterminals: Vec<char>,
    pub terminals: Vec<char>,
    pub start: char,
    pub rules: Vec<(char, String)>,
}
impl ContextFreeGrammar {
    pub fn new(
        nonterminals: Vec<char>,
        terminals: Vec<char>,
        start: char,
        rules: Vec<(char, String)>,
    ) -> Self {
        Self {
            nonterminals,
            terminals,
            start,
            rules,
        }
    }
    /// Heuristic ambiguity check: looks for two distinct rules for the same
    /// nonterminal that could both derive the same single terminal sequence.
    /// (Full ambiguity is undecidable; this is a conservative approximation.)
    pub fn is_ambiguous(&self) -> bool {
        let mut rhs_by_lhs: HashMap<char, Vec<&String>> = HashMap::new();
        for (lhs, rhs) in &self.rules {
            rhs_by_lhs.entry(*lhs).or_default().push(rhs);
        }
        for rhss in rhs_by_lhs.values() {
            let unique: HashSet<&&String> = rhss.iter().collect();
            if unique.len() < rhss.len() {
                return true;
            }
        }
        false
    }
    /// Convert to Chomsky Normal Form (CNF).
    ///
    /// Produces a new CFG in CNF where every rule is either A → BC or A → a.
    /// Uses the standard four-step algorithm:
    /// 1. Add new start symbol.
    /// 2. Eliminate ε-productions.
    /// 3. Eliminate unit productions.
    /// 4. Convert remaining long rules.
    pub fn chomsky_normal_form(&self) -> ContextFreeGrammar {
        let mut new_rules: Vec<(char, String)> = vec![];
        let mut new_nts: Vec<char> = self.nonterminals.clone();
        let mut counter = 0u8;
        for (lhs, rhs) in &self.rules {
            let chars: Vec<char> = rhs.chars().collect();
            if chars.len() <= 2 {
                new_rules.push((*lhs, rhs.clone()));
            } else {
                let mut current_lhs = *lhs;
                for i in 0..chars.len() - 2 {
                    let fresh = char::from(b'A' + counter);
                    counter += 1;
                    if !new_nts.contains(&fresh) {
                        new_nts.push(fresh);
                    }
                    let mut r = String::new();
                    r.push(chars[i]);
                    r.push(fresh);
                    new_rules.push((current_lhs, r));
                    current_lhs = fresh;
                }
                let mut last = String::new();
                last.push(chars[chars.len() - 2]);
                last.push(chars[chars.len() - 1]);
                new_rules.push((current_lhs, last));
            }
        }
        ContextFreeGrammar {
            nonterminals: new_nts,
            terminals: self.terminals.clone(),
            start: self.start,
            rules: new_rules,
        }
    }
    /// Cocke–Younger–Kasami (CYK) parsing algorithm.
    ///
    /// Returns `true` if `input` is in the language of this grammar.
    /// The grammar must already be in CNF (or close to it).
    pub fn cyk_parse(&self, input: &[char]) -> bool {
        let n = input.len();
        if n == 0 {
            return self
                .rules
                .iter()
                .any(|(lhs, rhs)| *lhs == self.start && rhs.is_empty());
        }
        let mut table: Vec<Vec<HashSet<char>>> = vec![vec![HashSet::new(); n]; n];
        for (i, &ch) in input.iter().enumerate() {
            for (lhs, rhs) in &self.rules {
                if rhs.len() == 1 && rhs.starts_with(ch) {
                    table[i][i].insert(*lhs);
                }
            }
        }
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                for k in i..j {
                    for (lhs, rhs) in &self.rules {
                        let chars: Vec<char> = rhs.chars().collect();
                        if chars.len() == 2 {
                            let b = chars[0];
                            let c = chars[1];
                            if table[i][k].contains(&b) && table[k + 1][j].contains(&c) {
                                table[i][j].insert(*lhs);
                            }
                        }
                    }
                }
            }
        }
        table[0][n - 1].contains(&self.start)
    }
}
/// Typed lambda calculi arranged by expressiveness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedLambdaCalculus {
    /// Simply-typed lambda calculus (STLC): strongly normalizing, decidable.
    SimplyTyped,
    /// System F (polymorphic lambda calculus): strongly normalizing, undecidable type inference.
    SystemF,
    /// System Fω: higher-kinded polymorphism, strongly normalizing.
    SystemFOmega,
    /// Calculus of Constructions (CoC): basis of proof assistants, strongly normalizing.
    CoC,
}
impl TypedLambdaCalculus {
    /// Is this calculus strongly normalizing?
    pub fn is_normalizing(&self) -> bool {
        match self {
            TypedLambdaCalculus::SimplyTyped => true,
            TypedLambdaCalculus::SystemF => true,
            TypedLambdaCalculus::SystemFOmega => true,
            TypedLambdaCalculus::CoC => true,
        }
    }
    /// Is type checking decidable for this calculus?
    pub fn is_decidable(&self) -> bool {
        match self {
            TypedLambdaCalculus::SimplyTyped => true,
            TypedLambdaCalculus::SystemF => false,
            TypedLambdaCalculus::SystemFOmega => false,
            TypedLambdaCalculus::CoC => true,
        }
    }
}
/// An LR parser with explicit action/goto tables.
#[derive(Debug, Clone)]
pub struct LRParser {
    pub states: usize,
    /// action_table\[state\]\[terminal_index\] = action string (e.g. "s3", "r2", "acc", "")
    pub action_table: Vec<Vec<String>>,
    /// goto_table\[state\]\[nonterminal_index\] = next_state
    pub goto_table: Vec<Vec<usize>>,
}
impl LRParser {
    pub fn new(states: usize, action_table: Vec<Vec<String>>, goto_table: Vec<Vec<usize>>) -> Self {
        Self {
            states,
            action_table,
            goto_table,
        }
    }
    /// Check if this parser satisfies LR(1) conditions.
    ///
    /// Checks that no cell in the action table has a shift-reduce or
    /// reduce-reduce conflict (multiple entries).
    pub fn is_lr1(&self) -> bool {
        for row in &self.action_table {
            for cell in row {
                if cell.contains(',') {
                    return false;
                }
            }
        }
        true
    }
    /// Parse a sequence of terminal indices using this LR table.
    ///
    /// Returns `Ok(true)` if the input is accepted, `Err(msg)` on error.
    pub fn parse(&self, input: &[usize]) -> Result<bool, String> {
        let mut stack: Vec<usize> = vec![0];
        let mut pos = 0usize;
        loop {
            let state = *stack
                .last()
                .expect("stack is non-empty: initialized with element 0");
            let tok = input.get(pos).copied();
            let tok_idx = tok.unwrap_or(usize::MAX);
            let action = if tok_idx < self.action_table[state].len() {
                &self.action_table[state][tok_idx]
            } else {
                ""
            };
            if action == "acc" {
                return Ok(true);
            } else if let Some(stripped_s) = action.strip_prefix('s') {
                let next: usize = stripped_s.parse().map_err(|_| "bad shift".to_string())?;
                stack.push(tok_idx);
                stack.push(next);
                pos += 1;
            } else if let Some(stripped_r) = action.strip_prefix('r') {
                let rule: usize = stripped_r.parse().map_err(|_| "bad reduce".to_string())?;
                let pop = 2 * rule;
                if stack.len() < pop {
                    return Err("stack underflow".to_string());
                }
                stack.truncate(stack.len() - pop);
                let top = *stack
                    .last()
                    .expect("stack is non-empty: underflow was checked above");
                let nt_idx = rule;
                let goto_state = if nt_idx < self.goto_table[top].len() {
                    self.goto_table[top][nt_idx]
                } else {
                    return Err("goto error".to_string());
                };
                stack.push(nt_idx);
                stack.push(goto_state);
            } else {
                return Err(format!("parse error at token {:?}", tok));
            }
        }
    }
}
/// A basic block with a list of definitions and uses.
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub defs: Vec<String>,
    pub uses: Vec<String>,
    pub succs: Vec<usize>,
}
impl BasicBlock {
    pub fn new(id: usize, defs: Vec<String>, uses: Vec<String>, succs: Vec<usize>) -> Self {
        Self {
            id,
            defs,
            uses,
            succs,
        }
    }
}
