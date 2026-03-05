//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

#[derive(Debug, Clone)]
pub struct TransportationProblem {
    pub supply: Vec<f64>,
    pub demand: Vec<f64>,
    pub cost: Vec<Vec<f64>>,
}
impl TransportationProblem {
    pub fn new(supply: Vec<f64>, demand: Vec<f64>, cost: Vec<Vec<f64>>) -> Self {
        TransportationProblem {
            supply,
            demand,
            cost,
        }
    }
    pub fn is_balanced(&self) -> bool {
        let ts: f64 = self.supply.iter().sum();
        let td: f64 = self.demand.iter().sum();
        (ts - td).abs() < 1e-9
    }
    pub fn northwest_corner(&self) -> Vec<Vec<f64>> {
        let m = self.supply.len();
        let n = self.demand.len();
        let mut alloc = vec![vec![0.0f64; n]; m];
        let mut supply = self.supply.clone();
        let mut demand = self.demand.clone();
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            let qty = supply[i].min(demand[j]);
            alloc[i][j] = qty;
            supply[i] -= qty;
            demand[j] -= qty;
            if supply[i] < 1e-10 {
                i += 1;
            }
            if demand[j] < 1e-10 {
                j += 1;
            }
        }
        alloc
    }
    pub fn total_cost(&self, alloc: &[Vec<f64>]) -> f64 {
        let mut cost = 0.0;
        for (i, row) in alloc.iter().enumerate() {
            for (j, &qty) in row.iter().enumerate() {
                if i < self.cost.len() && j < self.cost[i].len() {
                    cost += qty * self.cost[i][j];
                }
            }
        }
        cost
    }
    pub fn vogel_approximation(&self) -> Vec<Vec<f64>> {
        let m = self.supply.len();
        let n = self.demand.len();
        let mut alloc = vec![vec![0.0f64; n]; m];
        let mut supply = self.supply.clone();
        let mut demand = self.demand.clone();
        let mut row_done = vec![false; m];
        let mut col_done = vec![false; n];
        for _ in 0..(m + n) {
            let mut best_penalty = -1.0f64;
            let mut best_is_row = true;
            let mut best_idx = 0;
            for i in 0..m {
                if row_done[i] || supply[i] < 1e-10 {
                    continue;
                }
                let mut costs: Vec<f64> = (0..n)
                    .filter(|&j| !col_done[j] && demand[j] > 1e-10)
                    .map(|j| self.cost[i][j])
                    .collect();
                costs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let penalty = if costs.len() >= 2 {
                    costs[1] - costs[0]
                } else if costs.len() == 1 {
                    costs[0]
                } else {
                    continue;
                };
                if penalty > best_penalty {
                    best_penalty = penalty;
                    best_is_row = true;
                    best_idx = i;
                }
            }
            for j in 0..n {
                if col_done[j] || demand[j] < 1e-10 {
                    continue;
                }
                let mut costs: Vec<f64> = (0..m)
                    .filter(|&i| !row_done[i] && supply[i] > 1e-10)
                    .map(|i| self.cost[i][j])
                    .collect();
                costs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let penalty = if costs.len() >= 2 {
                    costs[1] - costs[0]
                } else if costs.len() == 1 {
                    costs[0]
                } else {
                    continue;
                };
                if penalty > best_penalty {
                    best_penalty = penalty;
                    best_is_row = false;
                    best_idx = j;
                }
            }
            if best_penalty < 0.0 {
                break;
            }
            if best_is_row {
                let i = best_idx;
                let j = (0..n)
                    .filter(|&j| !col_done[j] && demand[j] > 1e-10)
                    .min_by(|&a, &b| {
                        self.cost[i][a]
                            .partial_cmp(&self.cost[i][b])
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap_or(0);
                let qty = supply[i].min(demand[j]);
                alloc[i][j] = qty;
                supply[i] -= qty;
                demand[j] -= qty;
                if supply[i] < 1e-10 {
                    row_done[i] = true;
                }
                if demand[j] < 1e-10 {
                    col_done[j] = true;
                }
            } else {
                let j = best_idx;
                let i = (0..m)
                    .filter(|&i| !row_done[i] && supply[i] > 1e-10)
                    .min_by(|&a, &b| {
                        self.cost[a][j]
                            .partial_cmp(&self.cost[b][j])
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap_or(0);
                let qty = supply[i].min(demand[j]);
                alloc[i][j] = qty;
                supply[i] -= qty;
                demand[j] -= qty;
                if supply[i] < 1e-10 {
                    row_done[i] = true;
                }
                if demand[j] < 1e-10 {
                    col_done[j] = true;
                }
            }
        }
        alloc
    }
}
#[derive(Debug, Clone)]
pub struct LinearProgram {
    pub c: Vec<f64>,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<f64>,
    pub n_vars: usize,
    pub n_constraints: usize,
}
impl LinearProgram {
    pub fn new(c: Vec<f64>, a: Vec<Vec<f64>>, b: Vec<f64>) -> Self {
        let n_vars = c.len();
        let n_constraints = b.len();
        LinearProgram {
            c,
            a,
            b,
            n_vars,
            n_constraints,
        }
    }
    pub fn solve(&self) -> LpResult {
        if self.n_vars == 0 || self.n_constraints == 0 {
            return LpResult::Optimal {
                objective: 0.0,
                solution: vec![0.0; self.n_vars],
            };
        }
        let m = self.n_constraints;
        let n = self.n_vars;
        for &bi in &self.b {
            if bi < -1e-10 {
                return LpResult::Infeasible;
            }
        }
        let mut basis: Vec<usize> = (n..n + m).collect();
        let total = n + m;
        let mut tableau: Vec<Vec<f64>> = (0..m)
            .map(|i| {
                let mut row = vec![0.0_f64; total + 1];
                for j in 0..n {
                    row[j] = if j < self.a[i].len() {
                        self.a[i][j]
                    } else {
                        0.0
                    };
                }
                row[n + i] = 1.0;
                row[total] = self.b[i];
                row
            })
            .collect();
        let mut cost: Vec<f64> = self.c.clone();
        cost.extend(vec![0.0; m]);
        for _ in 0..4 * (n + m) {
            let (enter, rc) = cost
                .iter()
                .enumerate()
                .filter(|(j, _)| !basis.contains(j))
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(j, &v)| (j, v))
                .unwrap_or((0, 0.0));
            if rc >= -1e-10 {
                break;
            }
            let mut min_ratio = f64::INFINITY;
            let mut leave_row = None;
            for (i, row) in tableau.iter().enumerate() {
                let a_ij = row[enter];
                if a_ij > 1e-10 {
                    let ratio = row[total] / a_ij;
                    if ratio < min_ratio - 1e-14 {
                        min_ratio = ratio;
                        leave_row = Some(i);
                    }
                }
            }
            let lr = match leave_row {
                Some(r) => r,
                None => return LpResult::Unbounded,
            };
            let pivot = tableau[lr][enter];
            for v in tableau[lr].iter_mut() {
                *v /= pivot;
            }
            for i in 0..m {
                if i != lr {
                    let factor = tableau[i][enter];
                    let pivot_row: Vec<f64> = tableau[lr].clone();
                    for j in 0..=total {
                        tableau[i][j] -= factor * pivot_row[j];
                    }
                }
            }
            let factor = cost[enter];
            for j in 0..=total {
                let delta = factor * tableau[lr][j];
                if j < total {
                    cost[j] -= delta;
                }
            }
            cost[enter] = 0.0;
            basis[lr] = enter;
        }
        let mut x = vec![0.0_f64; n];
        for (i, &b_var) in basis.iter().enumerate() {
            if b_var < n {
                x[b_var] = tableau[i][total];
            }
        }
        LpResult::Optimal {
            objective: self.objective(&x),
            solution: x,
        }
    }
    pub fn is_feasible(&self, x: &[f64]) -> bool {
        if x.len() != self.n_vars {
            return false;
        }
        if x.iter().any(|&v| v < -1e-9) {
            return false;
        }
        for (i, row) in self.a.iter().enumerate() {
            let lhs: f64 = row.iter().zip(x.iter()).map(|(a, xv)| a * xv).sum();
            let rhs = if i < self.b.len() { self.b[i] } else { 0.0 };
            if (lhs - rhs).abs() > 1e-9 {
                return false;
            }
        }
        true
    }
    pub fn objective(&self, x: &[f64]) -> f64 {
        self.c.iter().zip(x.iter()).map(|(ci, xi)| ci * xi).sum()
    }
    pub fn dual(&self) -> LinearProgram {
        let m = self.n_constraints;
        let n = self.n_vars;
        let dual_c: Vec<f64> = self.b.iter().map(|&bi| -bi).collect();
        let dual_b: Vec<f64> = self.c.clone();
        let dual_a: Vec<Vec<f64>> = (0..n)
            .map(|j| {
                (0..m)
                    .map(|i| {
                        if i < self.a.len() && j < self.a[i].len() {
                            self.a[i][j]
                        } else {
                            0.0
                        }
                    })
                    .collect()
            })
            .collect();
        LinearProgram::new(dual_c, dual_a, dual_b)
    }
    pub fn shadow_prices(&self) -> Vec<f64> {
        match self.solve() {
            LpResult::Optimal { solution, .. } => {
                let base_obj = self.objective(&solution);
                (0..self.n_constraints)
                    .map(|i| {
                        let mut pb = self.b.clone();
                        pb[i] += 1e-5;
                        let plp = LinearProgram::new(self.c.clone(), self.a.clone(), pb);
                        match plp.solve() {
                            LpResult::Optimal { objective, .. } => (objective - base_obj) / 1e-5,
                            _ => 0.0,
                        }
                    })
                    .collect()
            }
            _ => vec![0.0; self.n_constraints],
        }
    }
    pub fn reduced_costs(&self) -> Vec<f64> {
        let dual = self.dual();
        match dual.solve() {
            LpResult::Optimal { solution: y, .. } => {
                let mut rc = self.c.clone();
                for j in 0..self.n_vars {
                    let mut dc = 0.0;
                    for (i, yi) in y.iter().enumerate() {
                        if i < self.a.len() && j < self.a[i].len() {
                            dc += yi * self.a[i][j];
                        }
                    }
                    rc[j] -= dc;
                }
                rc
            }
            _ => self.c.clone(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct NetworkEdge {
    pub from: usize,
    pub to: usize,
    pub capacity: f64,
    pub cost: f64,
}
#[derive(Debug, Clone)]
pub struct InequalityLP {
    pub c: Vec<f64>,
    pub a: Vec<Vec<f64>>,
    pub b: Vec<f64>,
}
impl InequalityLP {
    pub fn new(c: Vec<f64>, a: Vec<Vec<f64>>, b: Vec<f64>) -> Self {
        InequalityLP { c, a, b }
    }
    pub fn to_standard_form(&self) -> LinearProgram {
        let m = self.b.len();
        let n = self.c.len();
        let mut std_c = self.c.clone();
        std_c.extend(vec![0.0; m]);
        let std_a: Vec<Vec<f64>> = (0..m)
            .map(|i| {
                let mut row: Vec<f64> = if i < self.a.len() {
                    let mut r = self.a[i].clone();
                    r.resize(n, 0.0);
                    r
                } else {
                    vec![0.0; n]
                };
                for k in 0..m {
                    row.push(if k == i { 1.0 } else { 0.0 });
                }
                row
            })
            .collect();
        LinearProgram::new(std_c, std_a, self.b.clone())
    }
    pub fn solve(&self) -> LpResult {
        self.to_standard_form().solve()
    }
}
pub struct IntegerProgram {
    pub lp: LinearProgram,
    pub integer_vars: Vec<usize>,
}
impl IntegerProgram {
    pub fn new(lp: LinearProgram, integer_vars: Vec<usize>) -> Self {
        IntegerProgram { lp, integer_vars }
    }
    pub fn solve(&self) -> LpResult {
        self.branch_and_bound(self.lp.clone(), 0)
    }
    fn branch_and_bound(&self, lp: LinearProgram, depth: usize) -> LpResult {
        if depth > 8 {
            return lp.solve();
        }
        match lp.solve() {
            LpResult::Infeasible => LpResult::Infeasible,
            LpResult::Unbounded => LpResult::Unbounded,
            LpResult::Optimal {
                objective,
                solution,
            } => {
                let frac_var = self.integer_vars.iter().find(|&&j| {
                    j < solution.len() && (solution[j] - solution[j].round()).abs() > 1e-6
                });
                match frac_var {
                    None => LpResult::Optimal {
                        objective,
                        solution,
                    },
                    Some(&j) => {
                        let val = solution[j];
                        let r1 = self.branch_and_bound(
                            add_bound_constraint(&lp, j, val.floor(), true),
                            depth + 1,
                        );
                        let r2 = self.branch_and_bound(
                            add_bound_constraint(&lp, j, val.ceil(), false),
                            depth + 1,
                        );
                        best_result(r1, r2)
                    }
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct BendersDecomposition {
    pub c_first: Vec<f64>,
    pub a_first: Vec<Vec<f64>>,
    pub b_first: Vec<f64>,
    pub scenarios: Vec<ScenarioData>,
    pub max_iter: u32,
    pub tolerance: f64,
}
impl BendersDecomposition {
    pub fn new(
        c_first: Vec<f64>,
        a_first: Vec<Vec<f64>>,
        b_first: Vec<f64>,
        scenarios: Vec<ScenarioData>,
    ) -> Self {
        BendersDecomposition {
            c_first,
            a_first,
            b_first,
            scenarios,
            max_iter: 50,
            tolerance: 1e-6,
        }
    }
    /// Solve the master problem (first stage only, ignoring second stage).
    pub fn solve_master(&self, cuts: &[(Vec<f64>, f64)]) -> LpResult {
        let n = self.c_first.len();
        let mut c = self.c_first.clone();
        c.push(1.0);
        let mut rows = self.a_first.clone();
        let mut rhs = self.b_first.clone();
        for (pi, q) in cuts {
            let mut row: Vec<f64> = pi.iter().map(|v| -v).collect();
            row.resize(n, 0.0);
            row.push(-1.0);
            rows.push(row);
            rhs.push(-q);
        }
        let ilp = InequalityLP::new(c, rows, rhs);
        ilp.solve()
    }
    /// Evaluate expected second-stage cost given first-stage solution x.
    pub fn second_stage_cost(&self, x: &[f64]) -> f64 {
        self.scenarios
            .iter()
            .map(|s| {
                let n2 = s.c_second.len();
                if n2 == 0 {
                    return 0.0;
                }
                let b_eff: Vec<f64> = s
                    .b_second
                    .iter()
                    .enumerate()
                    .map(|(i, bi)| {
                        let ax: f64 = if i < self.a_first.len() {
                            self.a_first[i]
                                .iter()
                                .zip(x.iter())
                                .map(|(a, xi)| a * xi)
                                .sum()
                        } else {
                            0.0
                        };
                        bi - ax
                    })
                    .collect();
                let a_eye: Vec<Vec<f64>> = (0..n2)
                    .map(|k| {
                        let mut row = vec![0.0; n2];
                        if k < row.len() {
                            row[k] = 1.0;
                        }
                        row
                    })
                    .collect();
                let lp = LinearProgram::new(s.c_second.clone(), a_eye, b_eff);
                match lp.solve() {
                    LpResult::Optimal { objective, .. } => s.probability * objective,
                    _ => 0.0,
                }
            })
            .sum()
    }
    /// Run L-shaped method iterations.
    pub fn solve(&self) -> Option<(Vec<f64>, f64)> {
        let mut cuts: Vec<(Vec<f64>, f64)> = Vec::new();
        let mut best_x = vec![0.0; self.c_first.len()];
        let mut best_obj = f64::INFINITY;
        for _iter in 0..self.max_iter {
            match self.solve_master(&cuts) {
                LpResult::Optimal { solution, .. } => {
                    let x: Vec<f64> = solution[..self.c_first.len()].to_vec();
                    let first_cost: f64 = self
                        .c_first
                        .iter()
                        .zip(x.iter())
                        .map(|(c, xi)| c * xi)
                        .sum();
                    let second_cost = self.second_stage_cost(&x);
                    let total = first_cost + second_cost;
                    if total < best_obj - self.tolerance {
                        best_obj = total;
                        best_x = x.clone();
                    }
                    let pi: Vec<f64> = self.c_first.clone();
                    let q = second_cost;
                    cuts.push((pi, q));
                    if cuts.len() as u32 >= self.max_iter {
                        break;
                    }
                }
                _ => break,
            }
        }
        if best_obj < f64::INFINITY {
            Some((best_x, best_obj))
        } else {
            None
        }
    }
}
#[derive(Debug, Clone)]
pub struct NetworkSimplexSolver {
    pub nodes: usize,
    pub edges: Vec<NetworkEdge>,
    pub supply: Vec<f64>,
}
impl NetworkSimplexSolver {
    pub fn new(nodes: usize, edges: Vec<NetworkEdge>, supply: Vec<f64>) -> Self {
        NetworkSimplexSolver {
            nodes,
            edges,
            supply,
        }
    }
    /// Solve min-cost flow by converting to a standard-form LP and solving via simplex.
    ///
    /// Variables: f[0..e] (edge flows) and s[0..n_supply_nodes] (surplus slacks).
    /// For each source node with supply > 0:
    ///   outflow - inflow + slack_neg = supply   (slack_neg >= 0, penalised in cost)
    ///   outflow - inflow - slack_pos = supply   becomes: flow_sum <= supply (capacity style)
    /// We use a penalty-relaxation: equality `flow_sum = supply` encoded as
    ///   flow_sum <= supply  (conserved)  AND  flow_sum >= supply via penalty on deficit.
    /// For simplicity we solve the relaxed problem: only upper bounds on flow (capacity)
    /// and a supply upper-bound per source node, minimising cost.
    pub fn solve(&self) -> Option<(Vec<f64>, f64)> {
        let e = self.edges.len();
        let n = self.nodes;
        if e == 0 || n == 0 {
            return Some((vec![], 0.0));
        }
        let c: Vec<f64> = self.edges.iter().map(|ed| ed.cost).collect();
        let mut rows: Vec<Vec<f64>> = Vec::new();
        let mut rhs: Vec<f64> = Vec::new();
        for node in 0..n {
            let sup = if node < self.supply.len() {
                self.supply[node]
            } else {
                0.0
            };
            if sup < 1e-12 {
                continue;
            }
            let mut row = vec![0.0; e];
            for (k, ed) in self.edges.iter().enumerate() {
                if ed.from == node {
                    row[k] = 1.0;
                }
            }
            rows.push(row);
            rhs.push(sup);
        }
        for (k, ed) in self.edges.iter().enumerate() {
            if ed.capacity < f64::INFINITY {
                let mut row = vec![0.0; e];
                row[k] = 1.0;
                rows.push(row);
                rhs.push(ed.capacity);
            }
        }
        let ilp = InequalityLP::new(c, rows, rhs);
        match ilp.solve() {
            LpResult::Optimal {
                objective,
                solution,
            } => Some((solution, objective)),
            _ => None,
        }
    }
    pub fn total_cost(&self, flows: &[f64]) -> f64 {
        self.edges
            .iter()
            .zip(flows.iter())
            .map(|(ed, &f)| ed.cost * f)
            .sum()
    }
}
pub struct InteriorPointSolver {
    pub mu: f64,
    pub t_init: f64,
    pub max_outer: u32,
    pub max_inner: u32,
    pub tolerance: f64,
}
impl InteriorPointSolver {
    pub fn new() -> Self {
        InteriorPointSolver {
            mu: 10.0,
            t_init: 1.0,
            max_outer: 50,
            max_inner: 100,
            tolerance: 1e-8,
        }
    }
    pub fn with_params(mu: f64, t_init: f64, max_outer: u32, max_inner: u32) -> Self {
        InteriorPointSolver {
            mu,
            t_init,
            max_outer,
            max_inner,
            tolerance: 1e-8,
        }
    }
    pub fn solve(&self, c: &[f64], a: &[Vec<f64>], b: &[f64]) -> LpResult {
        let n = c.len();
        let m = b.len();
        if n == 0 {
            return LpResult::Optimal {
                objective: 0.0,
                solution: vec![],
            };
        }
        let mut x = vec![0.1_f64; n];
        if !self.is_strictly_feasible(&x, a, b) {
            x = self.find_initial_point(n, a, b);
            if !self.is_strictly_feasible(&x, a, b) {
                return LpResult::Infeasible;
            }
        }
        let mut t = self.t_init;
        let num_ineq = m + n;
        for _ in 0..self.max_outer {
            x = self.centering_step(&x, c, a, b, t);
            if (num_ineq as f64 / t) < self.tolerance {
                break;
            }
            t *= self.mu;
        }
        let obj: f64 = c.iter().zip(x.iter()).map(|(ci, xi)| ci * xi).sum();
        LpResult::Optimal {
            objective: obj,
            solution: x,
        }
    }
    fn is_strictly_feasible(&self, x: &[f64], a: &[Vec<f64>], b: &[f64]) -> bool {
        if x.iter().any(|&v| v <= 0.0) {
            return false;
        }
        for (i, row) in a.iter().enumerate() {
            let lhs: f64 = row.iter().zip(x.iter()).map(|(ai, xi)| ai * xi).sum();
            if lhs >= b[i] - 1e-10 {
                return false;
            }
        }
        true
    }
    fn find_initial_point(&self, n: usize, a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
        let mut scale = 0.1;
        for _ in 0..20 {
            let x = vec![scale; n];
            if self.is_strictly_feasible(&x, a, b) {
                return x;
            }
            scale *= 0.5;
        }
        vec![scale; n]
    }
    fn centering_step(&self, x0: &[f64], c: &[f64], a: &[Vec<f64>], b: &[f64], t: f64) -> Vec<f64> {
        let n = x0.len();
        let mut x = x0.to_vec();
        let step_size = 0.01;
        for _ in 0..self.max_inner {
            let mut grad = vec![0.0f64; n];
            for j in 0..n {
                grad[j] = t * c[j];
            }
            for j in 0..n {
                if x[j] > 1e-15 {
                    grad[j] -= 1.0 / x[j];
                }
            }
            for (i, row) in a.iter().enumerate() {
                let slack: f64 = b[i]
                    - row
                        .iter()
                        .zip(x.iter())
                        .map(|(ai, xi)| ai * xi)
                        .sum::<f64>();
                if slack > 1e-15 {
                    for j in 0..n.min(row.len()) {
                        grad[j] += row[j] / slack;
                    }
                }
            }
            let grad_norm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            if grad_norm < self.tolerance {
                break;
            }
            let mut alpha = step_size;
            for _ in 0..20 {
                let x_new: Vec<f64> = x
                    .iter()
                    .zip(grad.iter())
                    .map(|(&xi, &gi)| xi - alpha * gi)
                    .collect();
                if self.is_strictly_feasible(&x_new, a, b) {
                    x = x_new;
                    break;
                }
                alpha *= 0.5;
            }
        }
        x
    }
}
#[derive(Debug, Clone)]
pub enum LpResult {
    Optimal { objective: f64, solution: Vec<f64> },
    Infeasible,
    Unbounded,
}
#[derive(Debug, Clone)]
pub struct ColumnGenerationSolver {
    pub item_lengths: Vec<f64>,
    pub demands: Vec<u64>,
    pub stock_length: f64,
    pub max_iter: u32,
}
impl ColumnGenerationSolver {
    pub fn new(item_lengths: Vec<f64>, demands: Vec<u64>, stock_length: f64) -> Self {
        ColumnGenerationSolver {
            item_lengths,
            demands,
            stock_length,
            max_iter: 100,
        }
    }
    /// Generate initial patterns (one item per pattern).
    pub fn initial_patterns(&self) -> Vec<Vec<u64>> {
        self.item_lengths
            .iter()
            .map(|&len| {
                let count = (self.stock_length / len).floor() as u64;
                let mut pattern = vec![0u64; self.item_lengths.len()];
                if let Some(pos) = self
                    .item_lengths
                    .iter()
                    .position(|&l| (l - len).abs() < 1e-12)
                {
                    pattern[pos] = count;
                }
                pattern
            })
            .collect()
    }
    /// Solve the restricted master LP (minimize number of stocks used).
    fn solve_master(&self, patterns: &[Vec<u64>]) -> Option<Vec<f64>> {
        let n_items = self.item_lengths.len();
        let n_patterns = patterns.len();
        if n_patterns == 0 {
            return None;
        }
        let c = vec![1.0; n_patterns];
        let rows: Vec<Vec<f64>> = (0..n_items)
            .map(|i| {
                patterns
                    .iter()
                    .map(|p| -(if i < p.len() { p[i] as f64 } else { 0.0 }))
                    .collect()
            })
            .collect();
        let rhs: Vec<f64> = self.demands.iter().map(|&d| -(d as f64)).collect();
        let ilp = InequalityLP::new(c, rows, rhs);
        match ilp.solve() {
            LpResult::Optimal { solution, .. } => Some(solution),
            _ => None,
        }
    }
    /// Pricing subproblem: find pattern with most negative reduced cost.
    fn pricing_subproblem(&self, duals: &[f64]) -> Option<Vec<u64>> {
        let n = self.item_lengths.len();
        let scale = 100.0;
        let cap = (self.stock_length * scale) as u64;
        let weights: Vec<u64> = self
            .item_lengths
            .iter()
            .map(|&l| (l * scale).round() as u64)
            .collect();
        let values: Vec<u64> = duals
            .iter()
            .map(|&d| (d.max(0.0) * scale * scale).round() as u64)
            .collect();
        let (selected, _) = knapsack_dp(&weights, &values, cap);
        let mut pattern = vec![0u64; n];
        for (i, s) in selected.iter().enumerate() {
            if *s {
                pattern[i] += 1;
            }
        }
        let rc: f64 = 1.0
            - duals
                .iter()
                .zip(pattern.iter())
                .map(|(&d, &p)| d * p as f64)
                .sum::<f64>();
        if rc < -1e-6 {
            Some(pattern)
        } else {
            None
        }
    }
    /// Run column generation iterations.
    pub fn solve(&self) -> Option<(Vec<Vec<u64>>, Vec<f64>, f64)> {
        let mut patterns = self.initial_patterns();
        for _ in 0..self.max_iter {
            let x = self.solve_master(&patterns)?;
            let n_items = self.item_lengths.len();
            let n_pat = patterns.len();
            let duals: Vec<f64> = (0..n_items)
                .map(|i| {
                    let contrib: f64 = patterns
                        .iter()
                        .zip(x.iter())
                        .map(|(p, &xj)| {
                            let pij = if i < p.len() { p[i] as f64 } else { 0.0 };
                            pij * xj
                        })
                        .sum();
                    let demand = if i < self.demands.len() {
                        self.demands[i] as f64
                    } else {
                        0.0
                    };
                    if demand > 1e-10 {
                        contrib / demand
                    } else {
                        0.0
                    }
                })
                .collect();
            match self.pricing_subproblem(&duals) {
                Some(new_pat) => patterns.push(new_pat),
                None => {
                    let obj: f64 = x.iter().sum();
                    let _ = n_pat;
                    return Some((patterns, x, obj));
                }
            }
        }
        let x = self.solve_master(&patterns)?;
        let obj: f64 = x.iter().sum();
        Some((patterns, x, obj))
    }
}
#[derive(Debug, Clone)]
pub struct EllipsoidMethodSolver {
    pub max_iter: u32,
    pub tolerance: f64,
    pub initial_radius: f64,
}
impl EllipsoidMethodSolver {
    pub fn new() -> Self {
        EllipsoidMethodSolver {
            max_iter: 500,
            tolerance: 1e-8,
            initial_radius: 1e6,
        }
    }
    pub fn with_params(max_iter: u32, tolerance: f64, initial_radius: f64) -> Self {
        EllipsoidMethodSolver {
            max_iter,
            tolerance,
            initial_radius,
        }
    }
    /// Check feasibility of Ax <= b via ellipsoid iterations.
    /// Returns a feasible point if found.
    pub fn find_feasible(&self, a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
        let n = if a.is_empty() {
            return Some(vec![]);
        } else {
            a[0].len()
        };
        if n == 0 {
            return Some(vec![]);
        }
        let mut center = vec![0.0; n];
        let mut p_diag = vec![self.initial_radius * self.initial_radius; n];
        for _ in 0..self.max_iter {
            let (viol_idx, viol_val) = a
                .iter()
                .zip(b.iter())
                .enumerate()
                .map(|(i, (row, &bi))| {
                    let ax: f64 = row
                        .iter()
                        .zip(center.iter())
                        .map(|(aij, xj)| aij * xj)
                        .sum();
                    (i, ax - bi)
                })
                .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or((0, f64::NEG_INFINITY));
            if viol_val <= self.tolerance {
                return Some(center);
            }
            let g: Vec<f64> = if viol_idx < a.len() {
                a[viol_idx].clone()
            } else {
                vec![0.0; n]
            };
            let pg: Vec<f64> = g
                .iter()
                .zip(p_diag.iter())
                .map(|(gi, pi)| gi * pi)
                .collect();
            let gpg: f64 = g.iter().zip(pg.iter()).map(|(gi, pgi)| gi * pgi).sum();
            if gpg < 1e-30 {
                break;
            }
            let sqrt_gpg = gpg.sqrt();
            let step = 1.0 / ((n + 1) as f64);
            for j in 0..n {
                center[j] -= step * pg[j] / sqrt_gpg;
            }
            let factor = (n * n) as f64 / ((n * n - 1).max(1) as f64);
            let cut = 2.0 / ((n + 1) as f64);
            for j in 0..n {
                let pgj = pg[j];
                p_diag[j] = factor * (p_diag[j] - cut * pgj * pgj / gpg);
                if p_diag[j] < 1e-30 {
                    p_diag[j] = 1e-30;
                }
            }
        }
        None
    }
    /// Check if a linear program is feasible (ignoring optimality).
    pub fn lp_feasible(&self, c: &[f64], a: &[Vec<f64>], b: &[f64]) -> bool {
        let _ = c;
        self.find_feasible(a, b).is_some()
    }
}
#[derive(Debug, Clone)]
pub struct GomoryCutGenerator {
    pub max_cuts: u32,
    pub tolerance: f64,
}
impl GomoryCutGenerator {
    pub fn new() -> Self {
        GomoryCutGenerator {
            max_cuts: 20,
            tolerance: 1e-6,
        }
    }
    pub fn with_params(max_cuts: u32, tolerance: f64) -> Self {
        GomoryCutGenerator {
            max_cuts,
            tolerance,
        }
    }
    /// Generate Gomory cuts from a fractional LP solution.
    /// For each fractional variable x_j = f + integer_part,
    /// we generate the cut: x_j >= ceil(x_j) expressed as a simple bound.
    pub fn generate_cuts(&self, solution: &[f64], _lp: &LinearProgram) -> Vec<GomoryCut> {
        let mut cuts = Vec::new();
        for (j, &xj) in solution.iter().enumerate() {
            if cuts.len() as u32 >= self.max_cuts {
                break;
            }
            let frac = xj - xj.floor();
            if frac > self.tolerance && frac < 1.0 - self.tolerance {
                let mut coeffs = vec![0.0; solution.len()];
                if j < coeffs.len() {
                    coeffs[j] = 1.0;
                }
                cuts.push(GomoryCut {
                    coefficients: coeffs,
                    rhs: xj.floor(),
                });
            }
        }
        cuts
    }
    /// Apply Gomory cuts to a LinearProgram and resolve.
    pub fn solve_with_cuts(&self, lp: &LinearProgram) -> LpResult {
        let mut current_lp = lp.clone();
        for _ in 0..self.max_cuts {
            match current_lp.solve() {
                LpResult::Optimal {
                    objective,
                    solution,
                } => {
                    let cuts = self.generate_cuts(&solution, &current_lp);
                    if cuts.is_empty() {
                        return LpResult::Optimal {
                            objective,
                            solution,
                        };
                    }
                    for cut in &cuts {
                        let m = current_lp.n_constraints;
                        let n = current_lp.n_vars;
                        let mut new_a = current_lp.a.clone();
                        let mut row = cut.coefficients.clone();
                        row.resize(n, 0.0);
                        new_a.push(row);
                        let mut new_b = current_lp.b.clone();
                        new_b.push(cut.rhs);
                        current_lp = LinearProgram {
                            c: current_lp.c.clone(),
                            a: new_a,
                            b: new_b,
                            n_vars: n,
                            n_constraints: m + 1,
                        };
                    }
                }
                other => return other,
            }
        }
        current_lp.solve()
    }
}
#[derive(Debug, Clone)]
pub struct GomoryCut {
    pub coefficients: Vec<f64>,
    pub rhs: f64,
}
#[derive(Debug, Clone)]
pub struct ScenarioData {
    pub probability: f64,
    pub b_second: Vec<f64>,
    pub c_second: Vec<f64>,
}
