//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// DPLL-based propositional SAT solver.
/// Variables are 1..=n. Clauses are lists of literals (positive = var, negative = -var).
pub struct DpllSolver {
    n_vars: usize,
    clauses: Vec<Vec<i32>>,
}
impl DpllSolver {
    /// Create a new DPLL solver with `n_vars` variables.
    pub fn new(n_vars: usize) -> Self {
        Self {
            n_vars,
            clauses: vec![],
        }
    }
    /// Add a clause (disjunction of literals).
    pub fn add_clause(&mut self, clause: Vec<i32>) {
        self.clauses.push(clause);
    }
    /// Solve. Returns `Some(assignment)` if SAT (assignment[i] = value of variable i+1).
    pub fn solve(&self) -> Option<Vec<bool>> {
        let mut assignment = vec![None::<bool>; self.n_vars + 1];
        if self.dpll(&mut assignment) {
            Some(
                assignment
                    .into_iter()
                    .skip(1)
                    .map(|v| v.unwrap_or(false))
                    .collect(),
            )
        } else {
            None
        }
    }
    fn dpll(&self, assignment: &mut Vec<Option<bool>>) -> bool {
        let mut changed = true;
        while changed {
            changed = false;
            for clause in &self.clauses {
                let (unset, sat, unit_lit) = self.clause_status(clause, assignment);
                if sat {
                    continue;
                }
                if unset == 0 {
                    return false;
                }
                if unset == 1 {
                    let lit = unit_lit.expect(
                        "unit_lit is Some: unset == 1 means exactly one unset literal was tracked",
                    );
                    let var = lit.unsigned_abs() as usize;
                    let val = lit > 0;
                    if assignment[var] == Some(!val) {
                        return false;
                    }
                    assignment[var] = Some(val);
                    changed = true;
                }
            }
        }
        if self
            .clauses
            .iter()
            .all(|c| self.clause_status(c, assignment).1)
        {
            return true;
        }
        let var = match (1..=self.n_vars).find(|&v| assignment[v].is_none()) {
            Some(v) => v,
            None => return false,
        };
        assignment[var] = Some(true);
        if self.dpll(assignment) {
            return true;
        }
        assignment[var] = Some(false);
        if self.dpll(assignment) {
            return true;
        }
        assignment[var] = None;
        false
    }
    /// Returns (unset_count, is_satisfied, last_unset_literal)
    fn clause_status(
        &self,
        clause: &[i32],
        assignment: &[Option<bool>],
    ) -> (usize, bool, Option<i32>) {
        let mut unset = 0;
        let mut last_unset = None;
        for &lit in clause {
            let var = lit.unsigned_abs() as usize;
            match assignment[var] {
                None => {
                    unset += 1;
                    last_unset = Some(lit);
                }
                Some(val) => {
                    if (lit > 0) == val {
                        return (0, true, None);
                    }
                }
            }
        }
        (unset, false, last_unset)
    }
}
/// Evaluate a Boolean circuit on a given input assignment.
#[allow(dead_code)]
pub struct CircuitEvaluator {
    pub gates: Vec<CircuitGate>,
}
impl CircuitEvaluator {
    /// Create a new circuit evaluator.
    pub fn new() -> Self {
        Self { gates: vec![] }
    }
    /// Add a gate and return its index.
    pub fn add_gate(&mut self, kind: GateKind, left: Option<usize>, right: Option<usize>) -> usize {
        let idx = self.gates.len();
        self.gates.push(CircuitGate { kind, left, right });
        idx
    }
    /// Evaluate the circuit (top gate = last gate index) on `inputs`.
    pub fn evaluate(&self, inputs: &[bool]) -> bool {
        let output_gate = self.gates.len().saturating_sub(1);
        self.eval_gate(output_gate, inputs)
    }
    fn eval_gate(&self, idx: usize, inputs: &[bool]) -> bool {
        let gate = &self.gates[idx];
        match gate.kind {
            GateKind::Input(i) => inputs.get(i).copied().unwrap_or(false),
            GateKind::Const(b) => b,
            GateKind::And => {
                let l = gate.left.map(|i| self.eval_gate(i, inputs)).unwrap_or(true);
                let r = gate
                    .right
                    .map(|i| self.eval_gate(i, inputs))
                    .unwrap_or(true);
                l && r
            }
            GateKind::Or => {
                let l = gate
                    .left
                    .map(|i| self.eval_gate(i, inputs))
                    .unwrap_or(false);
                let r = gate
                    .right
                    .map(|i| self.eval_gate(i, inputs))
                    .unwrap_or(false);
                l || r
            }
            GateKind::Not => {
                let l = gate
                    .left
                    .map(|i| self.eval_gate(i, inputs))
                    .unwrap_or(false);
                !l
            }
        }
    }
}
/// Simple Sudoku solver using backtracking (9×9).
pub struct SudokuSolver {
    /// 81-element grid, 0 = empty
    pub grid: [u8; 81],
}
impl SudokuSolver {
    /// Create a new solver from a 81-element grid.
    pub fn new(grid: [u8; 81]) -> Self {
        Self { grid }
    }
    /// Solve the Sudoku. Returns true if a solution is found.
    pub fn solve(&mut self) -> bool {
        for pos in 0..81 {
            if self.grid[pos] == 0 {
                let row = pos / 9;
                let col = pos % 9;
                for digit in 1u8..=9 {
                    if self.is_valid(row, col, digit) {
                        self.grid[pos] = digit;
                        if self.solve() {
                            return true;
                        }
                        self.grid[pos] = 0;
                    }
                }
                return false;
            }
        }
        true
    }
    fn is_valid(&self, row: usize, col: usize, digit: u8) -> bool {
        for c in 0..9 {
            if self.grid[row * 9 + c] == digit {
                return false;
            }
        }
        for r in 0..9 {
            if self.grid[r * 9 + col] == digit {
                return false;
            }
        }
        let br = (row / 3) * 3;
        let bc = (col / 3) * 3;
        for r in br..br + 3 {
            for c in bc..bc + 3 {
                if self.grid[r * 9 + c] == digit {
                    return false;
                }
            }
        }
        true
    }
}
/// Evaluate a Boolean circuit represented as a DAG.
/// Gates: 0 = AND, 1 = OR, 2 = NOT, 3 = INPUT (index stored in `left`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GateKind {
    And,
    Or,
    Not,
    Input(usize),
    Const(bool),
}
/// Compute rank-based lower bounds for communication complexity.
/// Uses Gaussian elimination over GF(2) to find rank of the communication matrix.
#[allow(dead_code)]
pub struct CommunicationMatrixAnalyzer {
    /// rows × cols matrix over GF(2)
    pub matrix: Vec<Vec<u8>>,
}
impl CommunicationMatrixAnalyzer {
    /// Create from a matrix (entries should be 0 or 1).
    pub fn new(matrix: Vec<Vec<u8>>) -> Self {
        Self { matrix }
    }
    /// Compute the GF(2) rank of the matrix.
    pub fn rank_gf2(&self) -> usize {
        let rows = self.matrix.len();
        if rows == 0 {
            return 0;
        }
        let cols = self.matrix[0].len();
        let mut mat: Vec<Vec<u8>> = self.matrix.iter().map(|r| r.clone()).collect();
        let mut rank = 0;
        let mut pivot_row = 0;
        for col in 0..cols {
            let mut found = None;
            for row in pivot_row..rows {
                if mat[row][col] == 1 {
                    found = Some(row);
                    break;
                }
            }
            if let Some(r) = found {
                mat.swap(pivot_row, r);
                for row in 0..rows {
                    if row != pivot_row && mat[row][col] == 1 {
                        let pivot = mat[pivot_row].clone();
                        let target = &mut mat[row];
                        for j in 0..cols {
                            target[j] ^= pivot[j];
                        }
                    }
                }
                rank += 1;
                pivot_row += 1;
            }
        }
        rank
    }
    /// Log2 rank lower bound on deterministic communication complexity.
    /// Returns floor(log2(r)) where r is the GF(2) rank.
    pub fn log_rank_lower_bound(&self) -> usize {
        let r = self.rank_gf2();
        if r == 0 {
            return 0;
        }
        (usize::BITS - 1 - r.leading_zeros()) as usize
    }
}
#[allow(dead_code)]
pub struct CircuitGate {
    pub kind: GateKind,
    pub left: Option<usize>,
    pub right: Option<usize>,
}
/// Check whether an algorithm runs within FPT time bound f(k) * n^c.
/// Accepts observed runtime, parameter k, input size n, computable f, and constant c.
#[allow(dead_code)]
pub struct ParameterizedAlgorithmChecker {
    /// Name of the parameter
    pub param_name: String,
}
impl ParameterizedAlgorithmChecker {
    /// Create a new checker.
    pub fn new(param_name: &str) -> Self {
        Self {
            param_name: param_name.to_string(),
        }
    }
    /// Return true if `observed` ≤ f(k) * n^c (i.e., the bound holds).
    pub fn check(&self, observed: u64, k: u64, n: u64, f: impl Fn(u64) -> u64, c: u32) -> bool {
        let fk = f(k);
        let nc = n.saturating_pow(c);
        observed <= fk.saturating_mul(nc)
    }
    /// Check the standard 2^k * n bound (common FPT patterns).
    pub fn check_2k_n(&self, observed: u64, k: u64, n: u64) -> bool {
        let fk = 1u64.checked_shl(k as u32).unwrap_or(u64::MAX);
        observed <= fk.saturating_mul(n)
    }
}
/// Compute sensitivity and block sensitivity of a Boolean function.
/// The function is given as a truth table (index = bit string, value = output).
#[allow(dead_code)]
pub struct SensitivityChecker {
    /// Truth table: table[x] = f(x). Length must be 2^n.
    pub table: Vec<bool>,
    /// Number of input bits n.
    pub n: usize,
}
impl SensitivityChecker {
    /// Create from a truth table. Length must be 2^n.
    pub fn new(table: Vec<bool>) -> Self {
        let n = if table.is_empty() {
            0
        } else {
            usize::BITS as usize - table.len().leading_zeros() as usize - 1
        };
        Self { table, n }
    }
    /// Sensitivity of f at input x: number of coordinates i where flipping bit i changes f(x).
    pub fn sensitivity_at(&self, x: usize) -> usize {
        let fx = self.table.get(x).copied().unwrap_or(false);
        (0..self.n)
            .filter(|&i| {
                let xp = x ^ (1 << i);
                self.table.get(xp).copied().unwrap_or(false) != fx
            })
            .count()
    }
    /// Maximum sensitivity over all inputs.
    pub fn max_sensitivity(&self) -> usize {
        (0..self.table.len())
            .map(|x| self.sensitivity_at(x))
            .max()
            .unwrap_or(0)
    }
    /// Block sensitivity at input x: max number of disjoint sensitive blocks.
    pub fn block_sensitivity_at(&self, x: usize) -> usize {
        let fx = self.table.get(x).copied().unwrap_or(false);
        let mut used = vec![false; self.n];
        let mut count = 0;
        for size in 1..=self.n {
            for mask in 0..(1usize << self.n) {
                if mask.count_ones() as usize != size {
                    continue;
                }
                let all_unused = (0..self.n).all(|i| !used[i] || (mask >> i) & 1 == 0);
                if !all_unused {
                    continue;
                }
                let xp = x ^ mask;
                if self.table.get(xp).copied().unwrap_or(false) != fx {
                    for i in 0..self.n {
                        if (mask >> i) & 1 == 1 {
                            used[i] = true;
                        }
                    }
                    count += 1;
                    break;
                }
            }
        }
        count
    }
    /// Maximum block sensitivity over all inputs.
    pub fn max_block_sensitivity(&self) -> usize {
        (0..self.table.len())
            .map(|x| self.block_sensitivity_at(x))
            .max()
            .unwrap_or(0)
    }
    /// Check Huang's sensitivity theorem: s(f)^2 >= bs(f).
    pub fn check_huang_theorem(&self) -> bool {
        let s = self.max_sensitivity();
        let bs = self.max_block_sensitivity();
        s * s >= bs
    }
}
/// Evaluate a 2-CNF formula (given as list of clauses over bool variables).
/// Returns `true` if satisfiable (2-SAT, solved via Kosaraju SCC).
pub struct TwoSatSolver {
    n: usize,
    /// Implication graph: adj[2*i] = positive literal i, adj[2*i+1] = negative literal i
    adj: Vec<Vec<usize>>,
    /// Reverse implication graph
    radj: Vec<Vec<usize>>,
}
impl TwoSatSolver {
    /// Create a new 2-SAT solver for `n` variables.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![vec![]; 2 * n],
            radj: vec![vec![]; 2 * n],
        }
    }
    /// Add clause (a OR b). `a` and `b` are literals: 2*var for positive, 2*var+1 for negative.
    pub fn add_clause(&mut self, a: usize, b: usize) {
        let na = a ^ 1;
        let nb = b ^ 1;
        self.adj[na].push(b);
        self.adj[nb].push(a);
        self.radj[b].push(na);
        self.radj[a].push(nb);
    }
    /// Solve. Returns `Some(assignment)` if satisfiable, `None` otherwise.
    pub fn solve(&self) -> Option<Vec<bool>> {
        let n2 = 2 * self.n;
        let mut order: Vec<usize> = Vec::with_capacity(n2);
        let mut visited = vec![false; n2];
        for i in 0..n2 {
            if !visited[i] {
                self.dfs1(i, &mut visited, &mut order);
            }
        }
        let mut comp = vec![usize::MAX; n2];
        let mut c = 0;
        for &v in order.iter().rev() {
            if comp[v] == usize::MAX {
                self.dfs2(v, c, &mut comp);
                c += 1;
            }
        }
        let mut assignment = vec![false; self.n];
        for i in 0..self.n {
            if comp[2 * i] == comp[2 * i + 1] {
                return None;
            }
            assignment[i] = comp[2 * i] > comp[2 * i + 1];
        }
        Some(assignment)
    }
    fn dfs1(&self, v: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[v] = true;
        for &u in &self.adj[v] {
            if !visited[u] {
                self.dfs1(u, visited, order);
            }
        }
        order.push(v);
    }
    fn dfs2(&self, v: usize, c: usize, comp: &mut Vec<usize>) {
        comp[v] = c;
        for &u in &self.radj[v] {
            if comp[u] == usize::MAX {
                self.dfs2(u, c, comp);
            }
        }
    }
}
/// Simple resolution-based propositional prover.
/// Works on clauses over integer literals (positive = var, negative = negated var).
/// Attempts to derive the empty clause (refutation).
#[allow(dead_code)]
pub struct ResolutionProverSmall {
    clauses: Vec<Vec<i32>>,
}
impl ResolutionProverSmall {
    /// Create a new prover.
    pub fn new() -> Self {
        Self { clauses: vec![] }
    }
    /// Add a clause.
    pub fn add_clause(&mut self, clause: Vec<i32>) {
        let mut c = clause;
        c.sort_unstable();
        c.dedup();
        self.clauses.push(c);
    }
    /// Attempt to find a resolution refutation (saturation, bounded by `max_steps`).
    /// Returns true if the empty clause is derived.
    pub fn refute(&self, max_steps: usize) -> bool {
        use std::collections::HashSet;
        let normalize = |c: &Vec<i32>| -> Vec<i32> {
            let mut v = c.clone();
            v.sort_unstable();
            v.dedup();
            v
        };
        let mut known: HashSet<Vec<i32>> = HashSet::new();
        let mut all_clauses: Vec<Vec<i32>> = vec![];
        for c in &self.clauses {
            let n = normalize(c);
            if n.is_empty() {
                return true;
            }
            if known.insert(n.clone()) {
                all_clauses.push(n);
            }
        }
        let mut steps = 0;
        let mut new_start = 0;
        loop {
            if steps >= max_steps {
                break;
            }
            let end = all_clauses.len();
            if new_start >= end {
                break;
            }
            let mut added = vec![];
            for i in new_start..end {
                for j in 0..end {
                    if i == j {
                        continue;
                    }
                    if let Some(resolved) = Self::resolve(&all_clauses[i], &all_clauses[j]) {
                        let n = normalize(&resolved);
                        if n.is_empty() {
                            return true;
                        }
                        if known.insert(n.clone()) {
                            added.push(n);
                        }
                    }
                    steps += 1;
                    if steps >= max_steps {
                        break;
                    }
                }
                if steps >= max_steps {
                    break;
                }
            }
            if added.is_empty() {
                break;
            }
            new_start = end;
            all_clauses.extend(added);
        }
        false
    }
    /// Resolve two clauses on a single complementary literal pair.
    fn resolve(c1: &[i32], c2: &[i32]) -> Option<Vec<i32>> {
        for &lit in c1 {
            if c2.contains(&-lit) {
                let mut result: Vec<i32> = c1
                    .iter()
                    .filter(|&&l| l != lit)
                    .copied()
                    .chain(c2.iter().filter(|&&l| l != -lit).copied())
                    .collect();
                result.sort_unstable();
                result.dedup();
                for &l in &result {
                    if result.contains(&-l) {
                        return None;
                    }
                }
                return Some(result);
            }
        }
        None
    }
}
