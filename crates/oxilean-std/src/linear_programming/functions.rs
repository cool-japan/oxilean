//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BendersDecomposition, ColumnGenerationSolver, EllipsoidMethodSolver, GomoryCut,
    GomoryCutGenerator, InequalityLP, IntegerProgram, InteriorPointSolver, LinearProgram, LpResult,
    NetworkEdge, NetworkSimplexSolver, ScenarioData, TransportationProblem,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
#[allow(dead_code)]
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn lp_feasible_ty() -> Expr {
    let lr = list_ty(real_ty());
    let llr = list_ty(lr.clone());
    arrow(lr.clone(), arrow(llr, arrow(lr, prop())))
}
pub fn lp_optimal_ty() -> Expr {
    let lr = list_ty(real_ty());
    let llr = list_ty(lr.clone());
    arrow(lr.clone(), arrow(llr, arrow(lr.clone(), arrow(lr, prop()))))
}
pub fn duality_ty() -> Expr {
    prop()
}
pub fn integer_programming_ty() -> Expr {
    let lr = list_ty(real_ty());
    let llr = list_ty(lr.clone());
    arrow(lr.clone(), arrow(llr, arrow(lr, prop())))
}
pub fn totally_unimodular_ty() -> Expr {
    let lr = list_ty(real_ty());
    let llr = list_ty(lr);
    arrow(llr, prop())
}
pub fn interior_point_correctness_ty() -> Expr {
    prop()
}
pub fn shadow_price_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(lr.clone(), arrow(lr, list_ty(real_ty())))
}
pub fn transportation_problem_ty() -> Expr {
    let lr = list_ty(real_ty());
    let llr = list_ty(lr.clone());
    arrow(lr.clone(), arrow(lr, arrow(llr, prop())))
}
pub fn simplex_correctness_ty() -> Expr {
    prop()
}
pub fn strong_duality_ty() -> Expr {
    prop()
}
pub fn farkas_lemma_ty() -> Expr {
    prop()
}
pub fn ellipsoid_poly_ty() -> Expr {
    prop()
}
pub fn complementary_slackness_ty() -> Expr {
    prop()
}
pub fn parametric_optimal_value_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
pub fn sensitivity_range_ty() -> Expr {
    let lr = list_ty(real_ty());
    let pair_ty = app2(cst("Prod"), real_ty(), real_ty());
    arrow(lr.clone(), arrow(lr, list_ty(pair_ty)))
}
pub fn rhs_ranging_theorem_ty() -> Expr {
    prop()
}
pub fn parametric_lp_continuity_ty() -> Expr {
    prop()
}
pub fn min_cost_flow_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(
        lr.clone(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
pub fn network_flow_lp_ty() -> Expr {
    prop()
}
pub fn spanning_tree_basis_ty() -> Expr {
    arrow(real_ty(), arrow(list_ty(real_ty()), prop()))
}
pub fn network_simplex_optimality_ty() -> Expr {
    prop()
}
pub fn column_generation_master_ty() -> Expr {
    let lr = list_ty(real_ty());
    let llr = list_ty(lr.clone());
    arrow(llr, arrow(lr, prop()))
}
pub fn dantzig_wolfe_decomposition_ty() -> Expr {
    prop()
}
pub fn cutting_plane_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
pub fn restricted_master_problem_ty() -> Expr {
    prop()
}
pub fn benders_feasibility_cut_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
pub fn benders_optimality_cut_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
pub fn l_shaped_method_convergence_ty() -> Expr {
    prop()
}
pub fn benders_decomposition_correctness_ty() -> Expr {
    prop()
}
pub fn khachian_polynomial_lp_ty() -> Expr {
    prop()
}
pub fn ellipsoid_feasibility_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), prop()))
}
pub fn ellipsoid_volume_decrease_ty() -> Expr {
    prop()
}
pub fn polynomial_complexity_lp_ty() -> Expr {
    prop()
}
pub fn two_stage_recourse_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(list_ty(real_ty()), arrow(real_ty(), prop())),
    )
}
pub fn wait_and_see_ty() -> Expr {
    prop()
}
pub fn expected_value_perfect_info_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
pub fn stochastic_lp_optimality_ty() -> Expr {
    prop()
}
pub fn uncertainty_set_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
pub fn worst_case_robust_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(list_ty(real_ty()), arrow(real_ty(), prop())),
    )
}
pub fn data_driven_robust_ty() -> Expr {
    prop()
}
pub fn robust_lp_feasibility_ty() -> Expr {
    prop()
}
pub fn second_order_cone_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
pub fn semidefinite_program_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(list_ty(lr), prop())
}
pub fn copositive_program_ty() -> Expr {
    prop()
}
pub fn conic_duality_ty() -> Expr {
    prop()
}
pub fn gomory_cut_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
pub fn lift_and_project_ty() -> Expr {
    prop()
}
pub fn branch_and_bound_optimality_ty() -> Expr {
    prop()
}
pub fn cutting_planes_convergence_ty() -> Expr {
    prop()
}
pub fn lagrangian_duality_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
pub fn fenchel_duality_ty() -> Expr {
    prop()
}
pub fn minimax_theorem_ty() -> Expr {
    prop()
}
pub fn lcp_solution_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(lr.clone(), arrow(list_ty(lr), prop()))
}
pub fn mpec_feasibility_ty() -> Expr {
    prop()
}
pub fn online_lp_primal_dual_ty() -> Expr {
    prop()
}
pub fn competitive_ratio_lp_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
pub fn build_linear_programming_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("LpFeasible", lp_feasible_ty()),
        ("LpOptimal", lp_optimal_ty()),
        ("LpDuality", duality_ty()),
        ("IntegerProgram", integer_programming_ty()),
        ("TotallyUnimodular", totally_unimodular_ty()),
        ("InteriorPointCorrectness", interior_point_correctness_ty()),
        ("ShadowPrice", shadow_price_ty()),
        ("TransportationProblem", transportation_problem_ty()),
        ("simplex_correctness", simplex_correctness_ty()),
        ("strong_duality", strong_duality_ty()),
        ("farkas_lemma", farkas_lemma_ty()),
        ("ellipsoid_poly", ellipsoid_poly_ty()),
        ("complementary_slackness", complementary_slackness_ty()),
        ("LpBounded", arrow(list_ty(real_ty()), prop())),
        ("IpRelaxOptimal", prop()),
        ("TuRelaxationTheorem", prop()),
        ("ParametricOptimalValue", parametric_optimal_value_ty()),
        ("SensitivityRange", sensitivity_range_ty()),
        ("RhsRangingTheorem", rhs_ranging_theorem_ty()),
        ("ParametricLpContinuity", parametric_lp_continuity_ty()),
        ("MinCostFlow", min_cost_flow_ty()),
        ("NetworkFlowLp", network_flow_lp_ty()),
        ("SpanningTreeBasis", spanning_tree_basis_ty()),
        ("NetworkSimplexOptimality", network_simplex_optimality_ty()),
        ("ColumnGenerationMaster", column_generation_master_ty()),
        (
            "DantzigWolfeDecomposition",
            dantzig_wolfe_decomposition_ty(),
        ),
        ("CuttingPlane", cutting_plane_ty()),
        ("RestrictedMasterProblem", restricted_master_problem_ty()),
        ("BendersFeasibilityCut", benders_feasibility_cut_ty()),
        ("BendersOptimalityCut", benders_optimality_cut_ty()),
        ("LShapedMethodConvergence", l_shaped_method_convergence_ty()),
        (
            "BendersDecompositionCorrectness",
            benders_decomposition_correctness_ty(),
        ),
        ("KhachianPolynomialLP", khachian_polynomial_lp_ty()),
        ("EllipsoidFeasibility", ellipsoid_feasibility_ty()),
        ("EllipsoidVolumeDecrease", ellipsoid_volume_decrease_ty()),
        ("PolynomialComplexityLP", polynomial_complexity_lp_ty()),
        ("TwoStageRecourse", two_stage_recourse_ty()),
        ("WaitAndSee", wait_and_see_ty()),
        ("ExpectedValuePerfectInfo", expected_value_perfect_info_ty()),
        ("StochasticLpOptimality", stochastic_lp_optimality_ty()),
        ("UncertaintySet", uncertainty_set_ty()),
        ("WorstCaseRobust", worst_case_robust_ty()),
        ("DataDrivenRobust", data_driven_robust_ty()),
        ("RobustLpFeasibility", robust_lp_feasibility_ty()),
        ("SecondOrderCone", second_order_cone_ty()),
        ("SemidefiniteProgram", semidefinite_program_ty()),
        ("CopositiveProgram", copositive_program_ty()),
        ("ConicDuality", conic_duality_ty()),
        ("GomoryCut", gomory_cut_ty()),
        ("LiftAndProject", lift_and_project_ty()),
        ("BranchAndBoundOptimality", branch_and_bound_optimality_ty()),
        ("CuttingPlanesConvergence", cutting_planes_convergence_ty()),
        ("LagrangianDuality", lagrangian_duality_ty()),
        ("FenchelDuality", fenchel_duality_ty()),
        ("MinimaxTheorem", minimax_theorem_ty()),
        ("LcpSolution", lcp_solution_ty()),
        ("MpecFeasibility", mpec_feasibility_ty()),
        ("OnlineLpPrimalDual", online_lp_primal_dual_ty()),
        ("CompetitiveRatioLP", competitive_ratio_lp_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
}
pub fn knapsack_greedy(weights: &[f64], values: &[f64], capacity: f64) -> (Vec<bool>, f64) {
    let n = weights.len().min(values.len());
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&i, &j| {
        let ri = if weights[i] > 1e-15 {
            values[i] / weights[i]
        } else {
            f64::NEG_INFINITY
        };
        let rj = if weights[j] > 1e-15 {
            values[j] / weights[j]
        } else {
            f64::NEG_INFINITY
        };
        rj.partial_cmp(&ri).unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut selected = vec![false; n];
    let mut remaining = capacity;
    let mut total_value = 0.0;
    for i in indices {
        if weights[i] <= remaining + 1e-15 {
            selected[i] = true;
            remaining -= weights[i];
            total_value += values[i];
        }
    }
    (selected, total_value)
}
pub fn knapsack_dp(weights: &[u64], values: &[u64], capacity: u64) -> (Vec<bool>, u64) {
    let n = weights.len().min(values.len());
    let cap = capacity as usize;
    let mut dp = vec![vec![0u64; cap + 1]; n + 1];
    for i in 1..=n {
        for w in 0..=cap {
            dp[i][w] = dp[i - 1][w];
            let wi = weights[i - 1] as usize;
            if wi <= w {
                let with_item = dp[i - 1][w - wi] + values[i - 1];
                if with_item > dp[i][w] {
                    dp[i][w] = with_item;
                }
            }
        }
    }
    let mut selected = vec![false; n];
    let mut w = cap;
    for i in (1..=n).rev() {
        if dp[i][w] != dp[i - 1][w] {
            selected[i - 1] = true;
            w -= weights[i - 1] as usize;
        }
    }
    (selected, dp[n][cap])
}
pub fn knapsack_fractional(weights: &[f64], values: &[f64], capacity: f64) -> (Vec<f64>, f64) {
    let n = weights.len().min(values.len());
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&i, &j| {
        let ri = if weights[i] > 1e-15 {
            values[i] / weights[i]
        } else {
            f64::NEG_INFINITY
        };
        let rj = if weights[j] > 1e-15 {
            values[j] / weights[j]
        } else {
            f64::NEG_INFINITY
        };
        rj.partial_cmp(&ri).unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut fractions = vec![0.0; n];
    let mut remaining = capacity;
    let mut total_value = 0.0;
    for i in indices {
        if remaining <= 1e-15 {
            break;
        }
        if weights[i] <= remaining + 1e-15 {
            fractions[i] = 1.0;
            remaining -= weights[i];
            total_value += values[i];
        } else if weights[i] > 1e-15 {
            let frac = remaining / weights[i];
            fractions[i] = frac;
            total_value += frac * values[i];
            remaining = 0.0;
        }
    }
    (fractions, total_value)
}
pub fn add_bound_constraint(
    lp: &LinearProgram,
    j: usize,
    bound: f64,
    upper: bool,
) -> LinearProgram {
    let (m, n) = (lp.n_constraints, lp.n_vars);
    let mut new_a = lp.a.clone();
    let mut new_row = vec![0.0_f64; n];
    if j < n {
        new_row[j] = if upper { 1.0 } else { -1.0 };
    }
    new_a.push(new_row);
    let mut new_b = lp.b.clone();
    new_b.push(if upper { bound } else { -bound });
    LinearProgram {
        c: lp.c.clone(),
        a: new_a,
        b: new_b,
        n_vars: n,
        n_constraints: m + 1,
    }
}
pub fn best_result(r1: LpResult, r2: LpResult) -> LpResult {
    match (&r1, &r2) {
        (LpResult::Optimal { objective: o1, .. }, LpResult::Optimal { objective: o2, .. }) => {
            if o1 <= o2 {
                r1
            } else {
                r2
            }
        }
        (LpResult::Optimal { .. }, _) => r1,
        (_, LpResult::Optimal { .. }) => r2,
        _ => LpResult::Infeasible,
    }
}
pub fn mat_vec_mul(a: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
    a.iter()
        .map(|row| row.iter().zip(x.iter()).map(|(ai, xi)| ai * xi).sum())
        .collect()
}
pub fn transpose(a: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if a.is_empty() {
        return vec![];
    }
    let (m, n) = (a.len(), a[0].len());
    let mut t = vec![vec![0.0f64; m]; n];
    for i in 0..m {
        for j in 0..n.min(a[i].len()) {
            t[j][i] = a[i][j];
        }
    }
    t
}
pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai * bi).sum()
}
pub fn vec_add(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai + bi).collect()
}
pub fn vec_sub(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai - bi).collect()
}
pub fn vec_scale(v: &[f64], alpha: f64) -> Vec<f64> {
    v.iter().map(|vi| alpha * vi).collect()
}
pub fn vec_norm(v: &[f64]) -> f64 {
    v.iter().map(|vi| vi * vi).sum::<f64>().sqrt()
}
pub fn assignment_greedy(cost: &[Vec<f64>]) -> (Vec<usize>, f64) {
    let n = cost.len();
    if n == 0 {
        return (vec![], 0.0);
    }
    let m = cost[0].len();
    let size = n.min(m);
    let mut used_jobs = vec![false; m];
    let mut assignment = vec![0usize; n];
    let mut total_cost = 0.0;
    let mut workers: Vec<usize> = (0..n).collect();
    workers.sort_by(|&a, &b| {
        let min_a = cost[a].iter().cloned().fold(f64::INFINITY, f64::min);
        let min_b = cost[b].iter().cloned().fold(f64::INFINITY, f64::min);
        min_a
            .partial_cmp(&min_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for &worker in &workers {
        let mut best_job = 0;
        let mut best_cost = f64::INFINITY;
        for j in 0..size {
            if !used_jobs[j] && j < cost[worker].len() && cost[worker][j] < best_cost {
                best_cost = cost[worker][j];
                best_job = j;
            }
        }
        if best_cost < f64::INFINITY {
            assignment[worker] = best_job;
            used_jobs[best_job] = true;
            total_cost += best_cost;
        }
    }
    (assignment, total_cost)
}
pub fn rhs_sensitivity(lp: &LinearProgram) -> Vec<(f64, f64)> {
    let base_result = lp.solve();
    let base_obj = match &base_result {
        LpResult::Optimal { objective, .. } => *objective,
        _ => return vec![(f64::NEG_INFINITY, f64::INFINITY); lp.n_constraints],
    };
    (0..lp.n_constraints)
        .map(|i| {
            let step = lp.b[i].abs().max(1.0) * 0.01;
            let mut low = lp.b[i];
            let mut high = lp.b[i];
            for k in 1..100 {
                let delta = -(k as f64) * step;
                let mut tb = lp.b.clone();
                tb[i] += delta;
                let tlp = LinearProgram::new(lp.c.clone(), lp.a.clone(), tb);
                match tlp.solve() {
                    LpResult::Optimal { objective, .. } => {
                        let expected = delta * (base_obj / lp.b[i].max(1e-10));
                        if (objective - base_obj - expected).abs() > step * 2.0 {
                            break;
                        }
                        low = lp.b[i] + delta;
                    }
                    _ => break,
                }
            }
            for k in 1..100 {
                let delta = (k as f64) * step;
                let mut tb = lp.b.clone();
                tb[i] += delta;
                let tlp = LinearProgram::new(lp.c.clone(), lp.a.clone(), tb);
                match tlp.solve() {
                    LpResult::Optimal { objective, .. } => {
                        let expected = delta * (base_obj / lp.b[i].max(1e-10));
                        if (objective - base_obj - expected).abs() > step * 2.0 {
                            break;
                        }
                        high = lp.b[i] + delta;
                    }
                    _ => break,
                }
            }
            (low, high)
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linear_program_new() {
        let lp = LinearProgram::new(
            vec![1.0, 2.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![5.0, 5.0],
        );
        assert_eq!(lp.n_vars, 2);
        assert_eq!(lp.n_constraints, 2);
    }
    #[test]
    fn test_lp_feasible_check() {
        let lp = LinearProgram::new(vec![1.0], vec![vec![1.0]], vec![3.0]);
        assert!(lp.is_feasible(&[3.0]));
        assert!(!lp.is_feasible(&[2.0]));
        assert!(!lp.is_feasible(&[-1.0]));
    }
    #[test]
    fn test_lp_objective() {
        let lp = LinearProgram::new(vec![1.0, 2.0], vec![vec![1.0, 1.0]], vec![10.0]);
        assert!((lp.objective(&[3.0, 4.0]) - 11.0).abs() < 1e-10);
    }
    #[test]
    fn test_inequality_lp_to_standard() {
        let ilp = InequalityLP::new(
            vec![1.0, 1.0],
            vec![vec![1.0, 1.0], vec![1.0, 0.0]],
            vec![4.0, 3.0],
        );
        let std_lp = ilp.to_standard_form();
        assert_eq!(std_lp.n_vars, 4);
        assert_eq!(std_lp.n_constraints, 2);
        assert_eq!(std_lp.c[2], 0.0);
        assert_eq!(std_lp.c[3], 0.0);
    }
    #[test]
    fn test_knapsack_greedy() {
        let (sel, val) = knapsack_greedy(&[2.0, 3.0], &[4.0, 5.0], 4.0);
        assert!(sel[0]);
        assert!(val > 0.0);
    }
    #[test]
    fn test_knapsack_dp() {
        let (sel, max_val) = knapsack_dp(&[1, 3, 4, 5], &[1, 4, 5, 7], 7);
        assert_eq!(max_val, 9);
        assert!(sel[1] && sel[2]);
    }
    #[test]
    fn test_knapsack_fractional() {
        let (fracs, val) = knapsack_fractional(&[10.0, 20.0, 30.0], &[60.0, 100.0, 120.0], 50.0);
        assert!((fracs[0] - 1.0).abs() < 1e-10);
        assert!((fracs[1] - 1.0).abs() < 1e-10);
        assert!((fracs[2] - 2.0 / 3.0).abs() < 1e-10);
        assert!((val - 240.0).abs() < 1e-10);
    }
    #[test]
    fn test_lp_dual_size() {
        let lp = LinearProgram::new(
            vec![1.0, 2.0],
            vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]],
            vec![4.0, 6.0, 8.0],
        );
        let dual = lp.dual();
        assert_eq!(dual.n_vars, lp.n_constraints);
        assert_eq!(dual.n_constraints, lp.n_vars);
    }
    #[test]
    fn test_integer_program_new() {
        let lp = LinearProgram::new(vec![1.0, 1.0], vec![vec![1.0, 1.0]], vec![5.0]);
        let ip = IntegerProgram::new(lp, vec![0, 1]);
        assert_eq!(ip.integer_vars, vec![0, 1]);
    }
    #[test]
    fn test_lp_result_display() {
        let r = LpResult::Optimal {
            objective: 2.72,
            solution: vec![1.0, 2.0],
        };
        let s = format!("{}", r);
        assert!(s.contains("Optimal"));
        assert!(s.contains("2.72"));
        assert_eq!(format!("{}", LpResult::Infeasible), "Infeasible");
        assert_eq!(format!("{}", LpResult::Unbounded), "Unbounded");
    }
    #[test]
    fn test_transportation_northwest() {
        let tp = TransportationProblem::new(
            vec![20.0, 30.0],
            vec![10.0, 20.0, 20.0],
            vec![vec![8.0, 6.0, 10.0], vec![9.0, 12.0, 7.0]],
        );
        assert!(tp.is_balanced());
        let alloc = tp.northwest_corner();
        let row0: f64 = alloc[0].iter().sum();
        let row1: f64 = alloc[1].iter().sum();
        assert!((row0 - 20.0).abs() < 1e-9);
        assert!((row1 - 30.0).abs() < 1e-9);
    }
    #[test]
    fn test_transportation_vogel() {
        let tp = TransportationProblem::new(
            vec![20.0, 30.0],
            vec![10.0, 20.0, 20.0],
            vec![vec![8.0, 6.0, 10.0], vec![9.0, 12.0, 7.0]],
        );
        let nw_cost = tp.total_cost(&tp.northwest_corner());
        let vogel_cost = tp.total_cost(&tp.vogel_approximation());
        assert!(vogel_cost <= nw_cost + 1e-6);
    }
    #[test]
    fn test_interior_point_simple() {
        let solver = InteriorPointSolver::new();
        let result = solver.solve(&[-1.0], &[vec![1.0]], &[5.0]);
        match result {
            LpResult::Optimal {
                objective,
                solution,
            } => {
                assert!(solution[0] > 3.0, "x={}", solution[0]);
                assert!(objective < -3.0, "obj={}", objective);
            }
            _ => panic!("Expected optimal"),
        }
    }
    #[test]
    fn test_mat_vec_mul() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let r = mat_vec_mul(&a, &[1.0, 1.0]);
        assert!((r[0] - 3.0).abs() < 1e-10);
        assert!((r[1] - 7.0).abs() < 1e-10);
    }
    #[test]
    fn test_transpose() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let t = transpose(&a);
        assert!((t[0][0] - 1.0).abs() < 1e-10);
        assert!((t[0][1] - 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_dot_product() {
        assert!((dot(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]) - 32.0).abs() < 1e-10);
    }
    #[test]
    fn test_vec_operations() {
        assert!((vec_add(&[1.0, 2.0], &[3.0, 4.0])[0] - 4.0).abs() < 1e-10);
        assert!((vec_sub(&[3.0, 4.0], &[1.0, 2.0])[0] - 2.0).abs() < 1e-10);
        assert!((vec_scale(&[1.0, 2.0], 3.0)[1] - 6.0).abs() < 1e-10);
        assert!((vec_norm(&[3.0, 4.0]) - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_assignment_greedy() {
        let cost = vec![
            vec![9.0, 2.0, 7.0],
            vec![6.0, 4.0, 3.0],
            vec![5.0, 8.0, 1.0],
        ];
        let (assignment, total) = assignment_greedy(&cost);
        let mut jobs: Vec<usize> = assignment.clone();
        jobs.sort();
        jobs.dedup();
        assert_eq!(jobs.len(), 3);
        assert!(total > 0.0);
    }
    #[test]
    fn test_shadow_prices() {
        let lp = LinearProgram::new(vec![1.0], vec![vec![1.0]], vec![3.0]);
        let prices = lp.shadow_prices();
        assert_eq!(prices.len(), 1);
    }
    #[test]
    fn test_rhs_sensitivity() {
        let lp = LinearProgram::new(vec![1.0], vec![vec![1.0]], vec![3.0]);
        let ranges = rhs_sensitivity(&lp);
        assert_eq!(ranges.len(), 1);
        assert!(ranges[0].0 <= 3.0);
        assert!(ranges[0].1 >= 3.0);
    }
    #[test]
    fn test_lp_solve_inequality() {
        let ilp = InequalityLP::new(
            vec![-1.0, -1.0],
            vec![vec![1.0, 1.0], vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![10.0, 6.0, 8.0],
        );
        match ilp.solve() {
            LpResult::Optimal {
                objective,
                solution,
            } => {
                assert!(
                    objective < -9.0,
                    "obj should be near -10, got {}",
                    objective
                );
                assert!(solution[0] + solution[1] > 9.0, "sum should be near 10");
            }
            other => panic!("Expected Optimal, got {:?}", other),
        }
    }
    #[test]
    fn test_unbalanced_transportation() {
        let tp = TransportationProblem::new(
            vec![10.0, 20.0],
            vec![15.0, 25.0],
            vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        );
        assert!(!tp.is_balanced());
    }
    #[test]
    fn test_build_linear_programming_env() {
        let mut env = Environment::new();
        build_linear_programming_env(&mut env);
        assert!(!env.is_empty());
    }
    #[test]
    fn test_parametric_optimal_value_ty() {
        let ty = parametric_optimal_value_ty();
        assert!(matches!(ty, oxilean_kernel::Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_sensitivity_range_ty() {
        let ty = sensitivity_range_ty();
        assert!(matches!(ty, oxilean_kernel::Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_min_cost_flow_ty() {
        let ty = min_cost_flow_ty();
        assert!(matches!(ty, oxilean_kernel::Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_column_generation_master_ty() {
        let ty = column_generation_master_ty();
        assert!(matches!(ty, oxilean_kernel::Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_benders_cuts_ty() {
        let f_ty = benders_feasibility_cut_ty();
        let o_ty = benders_optimality_cut_ty();
        assert!(matches!(f_ty, oxilean_kernel::Expr::Pi(_, _, _, _)));
        assert!(matches!(o_ty, oxilean_kernel::Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_ellipsoid_axiom_tys() {
        assert!(matches!(
            khachian_polynomial_lp_ty(),
            oxilean_kernel::Expr::Sort(_)
        ));
        assert!(matches!(
            ellipsoid_feasibility_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
    }
    #[test]
    fn test_stochastic_lp_tys() {
        assert!(matches!(
            two_stage_recourse_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
        assert!(matches!(
            expected_value_perfect_info_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
    }
    #[test]
    fn test_robust_lp_tys() {
        assert!(matches!(
            uncertainty_set_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
        assert!(matches!(
            worst_case_robust_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
    }
    #[test]
    fn test_conic_tys() {
        assert!(matches!(
            second_order_cone_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
        assert!(matches!(
            semidefinite_program_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
    }
    #[test]
    fn test_gomory_cut_ty() {
        assert!(matches!(
            gomory_cut_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
    }
    #[test]
    fn test_lagrangian_duality_ty() {
        assert!(matches!(
            lagrangian_duality_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
    }
    #[test]
    fn test_lcp_solution_ty() {
        assert!(matches!(
            lcp_solution_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
    }
    #[test]
    fn test_competitive_ratio_lp_ty() {
        assert!(matches!(
            competitive_ratio_lp_ty(),
            oxilean_kernel::Expr::Pi(_, _, _, _)
        ));
    }
    #[test]
    fn test_network_simplex_empty() {
        let solver = NetworkSimplexSolver::new(0, vec![], vec![]);
        let result = solver.solve();
        assert!(result.is_some());
        let (flows, cost) = result.expect("result should be valid");
        assert!(flows.is_empty());
        assert!((cost).abs() < 1e-10);
    }
    #[test]
    fn test_network_simplex_single_edge() {
        let edges = vec![NetworkEdge {
            from: 0,
            to: 1,
            capacity: 10.0,
            cost: 2.0,
        }];
        let supply = vec![5.0, 0.0];
        let solver = NetworkSimplexSolver::new(2, edges, supply);
        let result = solver.solve();
        assert!(result.is_some());
        let (flows, cost) = result.expect("result should be valid");
        assert!(!flows.is_empty());
        assert!(cost.is_finite());
    }
    #[test]
    fn test_network_simplex_total_cost() {
        let edges = vec![
            NetworkEdge {
                from: 0,
                to: 1,
                capacity: 10.0,
                cost: 3.0,
            },
            NetworkEdge {
                from: 0,
                to: 2,
                capacity: 10.0,
                cost: 2.0,
            },
        ];
        let solver = NetworkSimplexSolver::new(3, edges, vec![5.0, -2.0, -3.0]);
        let flows = vec![2.0, 3.0];
        let cost = solver.total_cost(&flows);
        assert!((cost - 12.0).abs() < 1e-10);
    }
    #[test]
    fn test_benders_new() {
        let bd = BendersDecomposition::new(
            vec![1.0],
            vec![vec![1.0]],
            vec![10.0],
            vec![ScenarioData {
                probability: 1.0,
                b_second: vec![5.0],
                c_second: vec![2.0],
            }],
        );
        assert_eq!(bd.c_first.len(), 1);
        assert_eq!(bd.scenarios.len(), 1);
    }
    #[test]
    fn test_benders_second_stage_cost() {
        let bd = BendersDecomposition::new(
            vec![0.0],
            vec![vec![0.0]],
            vec![1.0],
            vec![
                ScenarioData {
                    probability: 0.5,
                    b_second: vec![4.0],
                    c_second: vec![1.0],
                },
                ScenarioData {
                    probability: 0.5,
                    b_second: vec![8.0],
                    c_second: vec![1.0],
                },
            ],
        );
        let cost = bd.second_stage_cost(&[0.0]);
        assert!(cost >= 0.0);
    }
    #[test]
    fn test_benders_solve() {
        let bd = BendersDecomposition::new(
            vec![1.0],
            vec![vec![1.0]],
            vec![5.0],
            vec![ScenarioData {
                probability: 1.0,
                b_second: vec![3.0],
                c_second: vec![2.0],
            }],
        );
        let result = bd.solve();
        assert!(result.is_some());
        let (x, obj) = result.expect("result should be valid");
        assert!(!x.is_empty());
        assert!(obj.is_finite());
    }
    #[test]
    fn test_column_generation_initial_patterns() {
        let cg = ColumnGenerationSolver::new(vec![3.0, 4.0, 5.0], vec![2, 3, 1], 10.0);
        let patterns = cg.initial_patterns();
        assert_eq!(patterns.len(), 3);
        assert_eq!(patterns[0][0], 3);
    }
    #[test]
    fn test_column_generation_solve() {
        let cg = ColumnGenerationSolver::new(vec![3.0], vec![0], 9.0);
        let result = cg.solve();
        assert!(result.is_some());
        let (patterns, x, obj) = result.expect("result should be valid");
        assert!(!patterns.is_empty());
        assert!(!x.is_empty());
        assert!(obj >= 0.0);
    }
    #[test]
    fn test_ellipsoid_trivially_feasible() {
        let solver = EllipsoidMethodSolver::new();
        let a = vec![vec![1.0]];
        let b = vec![10.0];
        let result = solver.find_feasible(&a, &b);
        assert!(result.is_some());
    }
    #[test]
    fn test_ellipsoid_lp_feasible() {
        let solver = EllipsoidMethodSolver::new();
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![5.0, 5.0];
        assert!(solver.lp_feasible(&[1.0, 1.0], &a, &b));
    }
    #[test]
    fn test_ellipsoid_infeasible() {
        let solver = EllipsoidMethodSolver::with_params(200, 1e-8, 100.0);
        let a = vec![vec![1.0]];
        let b = vec![-100.0];
        let _result = solver.find_feasible(&a, &b);
    }
    #[test]
    fn test_ellipsoid_empty() {
        let solver = EllipsoidMethodSolver::new();
        let result = solver.find_feasible(&[], &[]);
        assert!(result.is_some());
    }
    #[test]
    fn test_gomory_new() {
        let gen = GomoryCutGenerator::new();
        assert_eq!(gen.max_cuts, 20);
    }
    #[test]
    fn test_gomory_generate_cuts_integer_solution() {
        let gen = GomoryCutGenerator::new();
        let lp = LinearProgram::new(vec![1.0], vec![vec![1.0]], vec![3.0]);
        let cuts = gen.generate_cuts(&[3.0], &lp);
        assert!(cuts.is_empty());
    }
    #[test]
    fn test_gomory_generate_cuts_fractional() {
        let gen = GomoryCutGenerator::new();
        let lp = LinearProgram::new(vec![1.0, 1.0], vec![vec![1.0, 1.0]], vec![5.0]);
        let cuts = gen.generate_cuts(&[1.5, 2.5], &lp);
        assert!(!cuts.is_empty());
        assert!(cuts[0].rhs >= 0.0);
    }
    #[test]
    fn test_gomory_solve_with_cuts_simple() {
        let gen = GomoryCutGenerator::new();
        let lp = LinearProgram::new(vec![1.0], vec![vec![1.0]], vec![3.0]);
        let result = gen.solve_with_cuts(&lp);
        assert!(matches!(result, LpResult::Optimal { .. }));
    }
    #[test]
    fn test_gomory_with_params() {
        let gen = GomoryCutGenerator::with_params(10, 1e-4);
        assert_eq!(gen.max_cuts, 10);
        assert!((gen.tolerance - 1e-4).abs() < 1e-15);
    }
    #[test]
    fn test_build_lp_env_has_new_axioms() {
        let mut env = Environment::new();
        build_linear_programming_env(&mut env);
        let has_min_cost = env.get(&Name::str("MinCostFlow")).is_some();
        let has_benders = env.get(&Name::str("BendersOptimalityCut")).is_some();
        let has_gomory = env.get(&Name::str("GomoryCut")).is_some();
        let has_robust = env.get(&Name::str("WorstCaseRobust")).is_some();
        let has_online = env.get(&Name::str("OnlineLpPrimalDual")).is_some();
        assert!(has_min_cost);
        assert!(has_benders);
        assert!(has_gomory);
        assert!(has_robust);
        assert!(has_online);
    }
}
