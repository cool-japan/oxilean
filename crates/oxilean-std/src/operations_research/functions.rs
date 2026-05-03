//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BanditEnvironment, BellmanFord, Dijkstra, DynamicProgramming, FloydWarshall, FordFulkerson,
    HungarianSolver, InventoryModel, JobScheduler, KnapsackDP, LagrangianRelaxationSolver,
    MdpSolver, MultiArmedBanditUcb, NetworkFlowGraph, NewsvendorModel, PrimMst, QueueingSystem,
    ReliabilitySystem, SimplexSolver, TwoStageStochasticLP,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// `NetworkFlow : Nat -> List (Nat × Nat × Nat) -> Nat -> Nat -> Nat -> Prop`
/// Max-flow / min-cut predicate for a directed graph with `n` nodes.
pub fn network_flow_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            list_ty(nat_ty()),
            arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop()))),
        ),
    )
}
/// `Scheduling : List (Nat × Nat) -> List Nat -> Prop`
/// Job scheduling: list of (processing_time, deadline) pairs → feasible schedule.
pub fn scheduling_ty() -> Expr {
    let pair = list_ty(nat_ty());
    arrow(list_ty(pair), arrow(list_ty(nat_ty()), prop()))
}
/// `Inventory : Real -> Real -> Real -> Real -> Prop`
/// EOQ inventory model: demand D, order cost S, holding cost H, lead time L.
pub fn inventory_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `QueueingSystem : Real -> Real -> Nat -> Prop`
/// M/M/c queue with arrival rate λ, service rate μ, and c servers.
pub fn queuing_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `MaxFlowMinCut : Prop`
/// The max-flow min-cut theorem: max flow value = min cut capacity.
pub fn max_flow_min_cut_ty() -> Expr {
    prop()
}
/// `FordFulkerson : Prop`
/// The Ford-Fulkerson theorem: max flow = min cut.
pub fn ford_fulkerson_ty() -> Expr {
    prop()
}
/// `BellmanOptimality : Prop`
/// Bellman's principle of optimality for dynamic programming.
pub fn bellman_optimality_ty() -> Expr {
    prop()
}
/// `LittleLaw : Prop`
/// Little's law: L = λW (mean queue length = arrival rate × mean sojourn time).
pub fn little_law_ty() -> Expr {
    prop()
}
/// `EoqFormula : Prop`
/// The EOQ formula: Q* = sqrt(2DS / H).
pub fn eoq_formula_ty() -> Expr {
    prop()
}
/// `LpFeasible : Nat -> Nat -> List Real -> List (List Real) -> List Real -> Prop`
/// LP feasibility: given n variables, m constraints, objective c, matrix A, rhs b.
pub fn lp_feasible_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            nat_ty(),
            arrow(
                list_ty(real_ty()),
                arrow(
                    list_ty(list_ty(real_ty())),
                    arrow(list_ty(real_ty()), prop()),
                ),
            ),
        ),
    )
}
/// `SimplexOptimal : Prop`
/// The simplex method terminates at a basic feasible solution that is optimal
/// for the LP if one exists.
pub fn simplex_optimal_ty() -> Expr {
    prop()
}
/// `LpDuality : Prop`
/// Strong duality: for a primal LP and its dual, if both are feasible then
/// optimal primal value = optimal dual value.
pub fn lp_duality_ty() -> Expr {
    prop()
}
/// `RevisedSimplex : Nat -> Nat -> List Real -> Prop`
/// Revised simplex method for LP with n variables and m constraints.
pub fn revised_simplex_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(list_ty(real_ty()), prop())))
}
/// `ComplementarySlackness : Prop`
/// Complementary slackness conditions for LP optimality.
pub fn complementary_slackness_ty() -> Expr {
    prop()
}
/// `IntegerProgramming : Nat -> Nat -> List Real -> Prop`
/// ILP feasibility / optimality for a mixed-integer program.
pub fn integer_programming_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(list_ty(real_ty()), prop())))
}
/// `BranchAndBound : Prop`
/// Branch-and-bound algorithm terminates with an optimal integer solution.
pub fn branch_and_bound_ty() -> Expr {
    prop()
}
/// `CuttingPlane : Prop`
/// Gomory cutting-plane algorithm: cuts off fractional LP relaxation solutions.
pub fn cutting_plane_ty() -> Expr {
    prop()
}
/// `BranchAndCut : Prop`
/// Branch-and-cut combines B&B with cutting planes for MIP.
pub fn branch_and_cut_ty() -> Expr {
    prop()
}
/// `SetCover : Nat -> List (List Nat) -> List Nat -> Prop`
/// Set cover: universe of n elements, collection of subsets, selected cover.
pub fn set_cover_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(list_ty(list_ty(nat_ty())), arrow(list_ty(nat_ty()), prop())),
    )
}
/// `Knapsack : Nat -> List Nat -> List Nat -> Nat -> Prop`
/// 0/1 knapsack: capacity, weights, values, optimal value.
pub fn knapsack_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            list_ty(nat_ty()),
            arrow(list_ty(nat_ty()), arrow(nat_ty(), prop())),
        ),
    )
}
/// `GraphColoring : Nat -> List (Nat × Nat) -> Nat -> Prop`
/// Graph coloring: n vertices, edge list, chromatic number k.
pub fn graph_coloring_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(nat_ty()), arrow(nat_ty(), prop())))
}
/// `ChromaticNumber : Nat -> Nat -> Prop`
/// Chromatic number χ(G) = k for graph with n vertices.
pub fn chromatic_number_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MinimumSpanningTree : Nat -> List (Nat × Nat × Nat) -> Nat -> Prop`
/// MST: graph with n nodes and weighted edges, optimal total weight.
pub fn minimum_spanning_tree_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(nat_ty()), arrow(nat_ty(), prop())))
}
/// `ShortestPath : Nat -> List (Nat × Nat × Nat) -> Nat -> Nat -> List Nat -> Prop`
/// Shortest path from source to sink in a weighted graph.
pub fn shortest_path_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            list_ty(nat_ty()),
            arrow(nat_ty(), arrow(nat_ty(), arrow(list_ty(nat_ty()), prop()))),
        ),
    )
}
/// `DijkstraCorrectness : Prop`
/// Dijkstra's algorithm computes correct shortest paths on non-negative weights.
pub fn dijkstra_correctness_ty() -> Expr {
    prop()
}
/// `BellmanFordCorrectness : Prop`
/// Bellman-Ford detects negative cycles and computes SSSP otherwise.
pub fn bellman_ford_correctness_ty() -> Expr {
    prop()
}
/// `FloydWarshallCorrectness : Prop`
/// Floyd-Warshall computes all-pairs shortest paths in O(n³).
pub fn floyd_warshall_correctness_ty() -> Expr {
    prop()
}
/// `TspTour : Nat -> List (Nat × Nat × Nat) -> List Nat -> Nat -> Prop`
/// TSP: n cities, distance matrix, Hamiltonian tour, tour length.
pub fn tsp_tour_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            list_ty(nat_ty()),
            arrow(list_ty(nat_ty()), arrow(nat_ty(), prop())),
        ),
    )
}
/// `TspLowerBound : Prop`
/// TSP lower bound via minimum spanning tree (Christofides-style bound).
pub fn tsp_lower_bound_ty() -> Expr {
    prop()
}
/// `VehicleRouting : Nat -> Nat -> List (Nat × Nat × Nat) -> Prop`
/// VRP: n customers, k vehicles, capacity-constrained routes.
pub fn vehicle_routing_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(list_ty(nat_ty()), prop())))
}
/// `MakespanMinimization : List (List Nat) -> Nat -> Prop`
/// Parallel machine scheduling: jobs on machines, minimum makespan.
pub fn makespan_minimization_ty() -> Expr {
    arrow(list_ty(list_ty(nat_ty())), arrow(nat_ty(), prop()))
}
/// `FlowShopScheduling : Nat -> Nat -> List (List Nat) -> Nat -> Prop`
/// Flow shop: n jobs, m machines, processing times, optimal makespan.
pub fn flow_shop_scheduling_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            nat_ty(),
            arrow(list_ty(list_ty(nat_ty())), arrow(nat_ty(), prop())),
        ),
    )
}
/// `AssignmentProblem : Nat -> List (List Nat) -> List (Nat × Nat) -> Prop`
/// Assignment problem: n agents/tasks, cost matrix, optimal assignment.
pub fn assignment_problem_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(list_ty(list_ty(nat_ty())), arrow(list_ty(nat_ty()), prop())),
    )
}
/// `HungarianAlgorithm : Prop`
/// Hungarian algorithm solves the assignment problem in O(n³).
pub fn hungarian_algorithm_ty() -> Expr {
    prop()
}
/// `QuadraticProgram : Nat -> List Real -> List Real -> Prop`
/// QP: minimize (1/2) xᵀQx + cᵀx subject to linear constraints.
pub fn quadratic_program_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), prop())),
    )
}
/// `TwoStageStochastic : Prop`
/// Two-stage stochastic programming: here-and-now decisions + recourse.
pub fn two_stage_stochastic_ty() -> Expr {
    prop()
}
/// `ScenarioProgramming : Nat -> List Real -> Prop`
/// Scenario-based stochastic program with S scenarios and their probabilities.
pub fn scenario_programming_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(real_ty()), prop()))
}
/// `RobustOptimization : Prop`
/// Min-max robust formulation: minimize worst-case objective over uncertainty set.
pub fn robust_optimization_ty() -> Expr {
    prop()
}
/// `BellmanEquation : (Nat -> Real) -> (Nat -> Real) -> Prop`
/// Bellman equation: V(s) = max_a { r(s,a) + γ V(s') }.
pub fn bellman_equation_ty() -> Expr {
    arrow(
        fn_ty(nat_ty(), real_ty()),
        arrow(fn_ty(nat_ty(), real_ty()), prop()),
    )
}
/// `MG1Queue : Real -> Real -> Real -> Prop`
/// M/G/1 queue: arrival rate λ, mean service time E\[S\], variance Var\[S\].
pub fn mg1_queue_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `PollaczekKhinchine : Prop`
/// Pollaczek-Khinchine mean value formula for M/G/1 queue.
pub fn pollaczek_khinchine_ty() -> Expr {
    prop()
}
/// `NewsvendorModel : Real -> Real -> Real -> Real -> Prop`
/// Newsvendor: demand distribution, unit cost, selling price, salvage value.
pub fn newsvendor_model_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `SeriesSystem : List Real -> Real -> Prop`
/// Series system reliability: component reliabilities, system reliability.
pub fn series_system_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
/// `ParallelSystem : List Real -> Real -> Prop`
/// Parallel system reliability: component reliabilities, system reliability.
pub fn parallel_system_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
/// `EventDrivenSimulation : Nat -> Real -> Prop`
/// Discrete-event simulation: n events, simulation time horizon.
pub fn event_driven_simulation_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `MonteCarloEstimator : Nat -> Real -> Real -> Prop`
/// Monte Carlo estimation: sample size N, estimate, confidence interval width.
pub fn monte_carlo_estimator_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// Build an [`Environment`] containing operations research axioms and theorems.
pub fn build_operations_research_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("NetworkFlow", network_flow_ty()),
        ("Scheduling", scheduling_ty()),
        ("Inventory", inventory_ty()),
        ("QueueingSystem", queuing_ty()),
        ("MaxFlowMinCut", max_flow_min_cut_ty()),
        ("ford_fulkerson", ford_fulkerson_ty()),
        ("bellman_optimality", bellman_optimality_ty()),
        ("little_law", little_law_ty()),
        ("eoq_formula", eoq_formula_ty()),
        ("EdfSchedule", prop()),
        ("SjfSchedule", prop()),
        ("DpOptimal", prop()),
        ("LpFeasible", lp_feasible_ty()),
        ("simplex_optimal", simplex_optimal_ty()),
        ("lp_duality", lp_duality_ty()),
        ("RevisedSimplex", revised_simplex_ty()),
        ("complementary_slackness", complementary_slackness_ty()),
        ("IntegerProgramming", integer_programming_ty()),
        ("branch_and_bound", branch_and_bound_ty()),
        ("cutting_plane", cutting_plane_ty()),
        ("branch_and_cut", branch_and_cut_ty()),
        ("SetCover", set_cover_ty()),
        ("Knapsack", knapsack_ty()),
        ("GraphColoring", graph_coloring_ty()),
        ("ChromaticNumber", chromatic_number_ty()),
        ("MinimumSpanningTree", minimum_spanning_tree_ty()),
        ("ShortestPath", shortest_path_ty()),
        ("dijkstra_correctness", dijkstra_correctness_ty()),
        ("bellman_ford_correctness", bellman_ford_correctness_ty()),
        (
            "floyd_warshall_correctness",
            floyd_warshall_correctness_ty(),
        ),
        ("TspTour", tsp_tour_ty()),
        ("tsp_lower_bound", tsp_lower_bound_ty()),
        ("VehicleRouting", vehicle_routing_ty()),
        ("MakespanMinimization", makespan_minimization_ty()),
        ("FlowShopScheduling", flow_shop_scheduling_ty()),
        ("AssignmentProblem", assignment_problem_ty()),
        ("hungarian_algorithm", hungarian_algorithm_ty()),
        ("QuadraticProgram", quadratic_program_ty()),
        ("two_stage_stochastic", two_stage_stochastic_ty()),
        ("ScenarioProgramming", scenario_programming_ty()),
        ("robust_optimization", robust_optimization_ty()),
        ("BellmanEquation", bellman_equation_ty()),
        ("MG1Queue", mg1_queue_ty()),
        ("pollaczek_khinchine", pollaczek_khinchine_ty()),
        ("NewsvendorModel", newsvendor_model_ty()),
        ("SeriesSystem", series_system_ty()),
        ("ParallelSystem", parallel_system_ty()),
        ("EventDrivenSimulation", event_driven_simulation_ty()),
        ("MonteCarloEstimator", monte_carlo_estimator_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// Compute binomial coefficient C(n, k).
pub(super) fn binomial_coeff(n: usize, k: usize) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result = 1_u64;
    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }
    result
}
/// Branch-and-bound completeness: the algorithm finds the optimal integer solution
/// in finite time for bounded feasible MIPs
/// Type: Prop
pub fn branch_and_bound_completeness_ty() -> Expr {
    prop()
}
/// LP relaxation bound: optimal LP relaxation ≥ optimal IP value (minimization)
/// Type: Prop
pub fn lp_relaxation_bound_ty() -> Expr {
    prop()
}
/// Gomory cut: valid cutting plane derived from LP tableau for ILP
/// Type: Nat → List Real → Real → Prop  (pivot row, coefficients, rhs → Gomory cut)
pub fn gomory_cut_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(list_ty(real_ty()), arrow(real_ty(), prop())),
    )
}
/// Mixed-integer Gomory cut: Gomory cut for mixed-integer programs
/// Type: Prop
pub fn mixed_integer_gomory_cut_ty() -> Expr {
    prop()
}
/// Lift-and-project cuts: stronger cuts for binary programs
/// Type: Nat → Prop  (variable index → cut for that variable)
pub fn lift_and_project_cut_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Split cut: general cutting plane from a split disjunction
/// Type: List Real → Real → Prop  (coefficients, rhs → split cut)
pub fn split_cut_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
/// Dantzig-Wolfe decomposition: reformulation by convex combinations of extreme points
/// Type: Prop
pub fn dantzig_wolfe_decomposition_ty() -> Expr {
    prop()
}
/// Dantzig-Wolfe restricted master problem: LP with subset of columns
/// Type: Nat → List Real → Prop  (column count, current columns → RMP)
pub fn dantzig_wolfe_master_problem_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(real_ty()), prop()))
}
/// Benders decomposition: partition variables into master and subproblem
/// Type: Prop
pub fn benders_decomposition_ty() -> Expr {
    prop()
}
/// Benders feasibility cut: cut added when subproblem is infeasible
/// Type: List Real → Real → Prop  (dual ray coefficients, rhs → feasibility cut)
pub fn benders_feasibility_cut_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
/// Benders optimality cut: cut from subproblem optimal dual
/// Type: List Real → Real → Prop  (dual solution, rhs → optimality cut)
pub fn benders_optimality_cut_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(real_ty(), prop()))
}
/// Column generation: pricing problem selects columns with negative reduced cost
/// Type: (List Real → Real) → Prop  (pricing oracle → column generation valid)
pub fn column_generation_ty() -> Expr {
    arrow(fn_ty(list_ty(real_ty()), real_ty()), prop())
}
/// Lagrangian relaxation: relax complicating constraints with multipliers
/// Type: List Real → (List Real → Real) → Real
///       (Lagrange multipliers, objective → Lagrangian lower bound)
pub fn lagrangian_relaxation_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(fn_ty(list_ty(real_ty()), real_ty()), real_ty()),
    )
}
/// Subgradient method: update Lagrange multipliers using subgradient
/// Type: List Real → List Real → Real → List Real
///       (current multipliers, subgradient, step size → updated multipliers)
pub fn subgradient_update_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(list_ty(real_ty()), arrow(real_ty(), list_ty(real_ty()))),
    )
}
/// Lagrangian dual: max over multipliers λ ≥ 0 of Lagrangian lower bound
/// Type: (List Real → Real) → Real  (Lagrangian function → dual bound)
pub fn lagrangian_dual_ty() -> Expr {
    arrow(fn_ty(list_ty(real_ty()), real_ty()), real_ty())
}
/// Semidefinite program: minimize c·x subject to Σ x_i A_i ⪰ B, A_i symmetric
/// Type: Nat → List Real → Prop  (matrix size, objective coefficients → SDP)
pub fn semidefinite_program_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(real_ty()), prop()))
}
/// Positive semidefinite constraint: X ⪰ 0 (all eigenvalues ≥ 0)
/// Type: Type → Prop  (matrix type → PSD constraint)
pub fn psd_constraint_ty() -> Expr {
    arrow(type0(), prop())
}
/// SDP duality: strong duality for semidefinite programs under strict feasibility
/// Type: Prop
pub fn sdp_duality_ty() -> Expr {
    prop()
}
/// Second-order cone program: minimize cᵀx subject to ||A_i x + b_i||₂ ≤ cᵀx + d
/// Type: Nat → Nat → Prop  (dimension, number of cone constraints → SOCP)
pub fn second_order_cone_program_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// SOCP duality: strong duality for second-order cone programs
/// Type: Prop
pub fn socp_duality_ty() -> Expr {
    prop()
}
/// Cone programming general: minimize cᵀx subject to Ax = b, x ∈ K
/// Type: Type → Prop  (cone K → cone program over K)
pub fn cone_program_ty() -> Expr {
    arrow(type0(), prop())
}
/// Minimax regret: minimize worst-case regret over uncertainty set
/// Type: Type → Prop  (uncertainty set → minimax regret problem)
pub fn minimax_regret_ty() -> Expr {
    arrow(type0(), prop())
}
/// Box uncertainty set: each parameter varies in interval \[a_i - Δ_i, a_i + Δ_i\]
/// Type: List Real → List Real → Type  (center, radius → box uncertainty set)
pub fn box_uncertainty_set_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), type0()))
}
/// Ellipsoidal uncertainty set: {u : (u-u₀)ᵀ Σ⁻¹ (u-u₀) ≤ Ω²}
/// Type: Nat → Real → Type  (dimension, Omega → ellipsoid)
pub fn ellipsoidal_uncertainty_set_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), type0()))
}
/// Robust counterpart: worst-case formulation for linear program with uncertain data
/// Type: Type → Prop  (uncertainty set → robust counterpart is tractable)
pub fn robust_counterpart_ty() -> Expr {
    arrow(type0(), prop())
}
/// Distributionally robust optimization: minimize worst-case expected cost over ambiguity set
/// Type: Type → Prop  (ambiguity set of distributions → DRO problem)
pub fn distributionally_robust_optimization_ty() -> Expr {
    arrow(type0(), prop())
}
/// Chance constraint: P(g(x, ξ) ≤ 0) ≥ 1 - ε for random ξ
/// Type: Real → Prop  (reliability level 1-ε → chance constraint)
pub fn chance_constraint_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// CVaR (Conditional Value at Risk): expected shortfall above α-quantile
/// Type: Real → (Real → Real) → Real  (α, loss distribution → CVaR)
pub fn cvar_ty() -> Expr {
    arrow(real_ty(), arrow(fn_ty(real_ty(), real_ty()), real_ty()))
}
/// Sample average approximation (SAA): replace expectation with sample mean
/// Type: Nat → Prop  (number of scenarios → SAA problem)
pub fn sample_average_approximation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Progressive hedging: decomposition for stochastic programs
/// Type: Nat → Real → Prop  (number of scenarios, penalty ρ → PH valid)
pub fn progressive_hedging_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// MDP: (S, A, P, R, γ) — states, actions, transitions, rewards, discount
/// Type: Nat → Nat → Real → Prop  (|S|, |A|, discount γ → MDP)
pub fn markov_decision_process_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop())))
}
/// Bellman optimality equation: V*(s) = max_a Σ P(s'|s,a)\[R(s,a,s') + γV*(s')\]
/// Type: (Nat → Real) → Prop  (value function V → satisfies Bellman optimality)
pub fn bellman_optimality_equation_ty() -> Expr {
    arrow(fn_ty(nat_ty(), real_ty()), prop())
}
/// Value iteration convergence: V_k → V* as k → ∞ for γ < 1
/// Type: Real → Prop  (discount factor γ → value iteration converges)
pub fn value_iteration_convergence_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// Policy iteration convergence: policy iteration terminates in finite steps
/// Type: Nat → Prop  (|S| → policy iteration terminates in at most |S|! steps)
pub fn policy_iteration_convergence_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Optimal policy: deterministic policy achieving V*
/// Type: Nat → Nat → Prop  (|S|, |A| → optimal deterministic policy exists)
pub fn optimal_policy_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// Q-function (action-value): Q*(s,a) = R(s,a) + γ Σ P(s'|s,a) V*(s')
/// Type: Nat → Nat → Real  (state s, action a → Q-value)
pub fn q_function_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// Contraction mapping in MDP: Bellman operator is a γ-contraction in sup norm
/// Type: Real → Prop  (γ → Bellman operator is contraction)
pub fn bellman_contraction_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// POMDP: partially observable MDP with observation model
/// Type: Nat → Nat → Nat → Real → Prop  (|S|, |A|, |O|, γ → POMDP)
pub fn pomdp_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// Belief state: probability distribution over hidden states
/// Type: Nat → Type  (|S| → belief simplex type)
pub fn belief_state_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// POMDP value function: piecewise-linear convex function of belief state
/// Type: Nat → Prop  (|S| → PWLC value function exists)
pub fn pomdp_value_function_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Multi-armed bandit: k arms with unknown reward distributions
/// Type: Nat → Prop  (k arms → MAB problem)
pub fn multi_armed_bandit_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// UCB1 algorithm: Upper Confidence Bound policy for MAB
/// Type: Nat → Real → Nat → Real  (k arms, total plays, arm plays → UCB index)
pub fn ucb1_index_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(nat_ty(), real_ty())))
}
/// UCB regret bound: E\[regret\] = O(√(kT log T)) for UCB1
/// Type: Nat → Nat → Real  (k, T → regret bound)
pub fn ucb_regret_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// Thompson sampling: Bayesian approach to MAB using posterior sampling
/// Type: Nat → Prop  (k arms → Thompson sampling valid)
pub fn thompson_sampling_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Explore-exploit tradeoff: fundamental tension in online learning
/// Type: Prop
pub fn explore_exploit_tradeoff_ty() -> Expr {
    prop()
}
/// Register all extended operations research axioms into the environment.
pub fn register_operations_research_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        (
            "BranchAndBoundCompleteness",
            branch_and_bound_completeness_ty(),
        ),
        ("LpRelaxationBound", lp_relaxation_bound_ty()),
        ("GomoryCut", gomory_cut_ty()),
        ("MixedIntegerGomoryCut", mixed_integer_gomory_cut_ty()),
        ("LiftAndProjectCut", lift_and_project_cut_ty()),
        ("SplitCut", split_cut_ty()),
        (
            "DantzigWolfeDecomposition",
            dantzig_wolfe_decomposition_ty(),
        ),
        (
            "DantzigWolfeMasterProblem",
            dantzig_wolfe_master_problem_ty(),
        ),
        ("BendersDecomposition", benders_decomposition_ty()),
        ("BendersFeasibilityCut", benders_feasibility_cut_ty()),
        ("BendersOptimalityCut", benders_optimality_cut_ty()),
        ("ColumnGeneration", column_generation_ty()),
        ("LagrangianRelaxation", lagrangian_relaxation_ty()),
        ("SubgradientUpdate", subgradient_update_ty()),
        ("LagrangianDual", lagrangian_dual_ty()),
        ("SemidefiniteProgram", semidefinite_program_ty()),
        ("PSDConstraint", psd_constraint_ty()),
        ("SDPDuality", sdp_duality_ty()),
        ("SecondOrderConeProgram", second_order_cone_program_ty()),
        ("SOCPDuality", socp_duality_ty()),
        ("ConeProgram", cone_program_ty()),
        ("MinimaxRegret", minimax_regret_ty()),
        ("BoxUncertaintySet", box_uncertainty_set_ty()),
        (
            "EllipsoidalUncertaintySet",
            ellipsoidal_uncertainty_set_ty(),
        ),
        ("RobustCounterpart", robust_counterpart_ty()),
        (
            "DistributionallyRobustOptimization",
            distributionally_robust_optimization_ty(),
        ),
        ("ChanceConstraint", chance_constraint_ty()),
        ("CVaR", cvar_ty()),
        (
            "SampleAverageApproximation",
            sample_average_approximation_ty(),
        ),
        ("ProgressiveHedging", progressive_hedging_ty()),
        ("MarkovDecisionProcess", markov_decision_process_ty()),
        (
            "BellmanOptimalityEquation",
            bellman_optimality_equation_ty(),
        ),
        (
            "ValueIterationConvergence",
            value_iteration_convergence_ty(),
        ),
        (
            "PolicyIterationConvergence",
            policy_iteration_convergence_ty(),
        ),
        ("OptimalPolicy", optimal_policy_ty()),
        ("QFunction", q_function_ty()),
        ("BellmanContraction", bellman_contraction_ty()),
        ("POMDP", pomdp_ty()),
        ("BeliefState", belief_state_ty()),
        ("POMDPValueFunction", pomdp_value_function_ty()),
        ("MultiArmedBandit", multi_armed_bandit_ty()),
        ("UCB1Index", ucb1_index_ty()),
        ("UCBRegretBound", ucb_regret_bound_ty()),
        ("ThompsonSampling", thompson_sampling_ty()),
        ("ExploreExploitTradeoff", explore_exploit_tradeoff_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add {name}: {e:?}"))?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queueing_utilization() {
        let qs = QueueingSystem::new(3.0, 5.0, 1);
        assert!((qs.utilization() - 0.6).abs() < 1e-9);
    }
    #[test]
    fn test_queueing_mean_queue_length() {
        let qs = QueueingSystem::new(1.0, 2.0, 1);
        let l = qs
            .mean_queue_length_m_m_1()
            .expect("mean_queue_length_m_m_1 should succeed");
        assert!((l - 1.0).abs() < 1e-9, "L={l}");
    }
    #[test]
    fn test_network_flow_simple() {
        let mut g = NetworkFlowGraph::new(4);
        g.add_edge(0, 1, 10);
        g.add_edge(0, 2, 10);
        g.add_edge(1, 3, 10);
        g.add_edge(2, 3, 10);
        assert_eq!(g.max_flow_bfs(0, 3), 20);
    }
    #[test]
    fn test_job_scheduler_edf() {
        let mut sched = JobScheduler::new();
        sched.add_job("A", 2, 5);
        sched.add_job("B", 3, 3);
        sched.add_job("C", 1, 7);
        let order = sched.earliest_deadline_first();
        assert_eq!(order, vec!["B", "A", "C"]);
    }
    #[test]
    fn test_dp_knapsack() {
        let weights = [2, 3, 4, 5];
        let values = [3, 4, 5, 6];
        assert_eq!(DynamicProgramming::knapsack(5, &weights, &values), 7);
    }
    #[test]
    fn test_dp_lcs() {
        let s1 = b"ABCBDAB";
        let s2 = b"BDCABA";
        assert_eq!(DynamicProgramming::longest_common_subseq(s1, s2), 4);
    }
    #[test]
    fn test_dp_coin_change() {
        assert_eq!(
            DynamicProgramming::coin_change(&[1, 5, 10, 25], 41),
            Some(4)
        );
        assert_eq!(DynamicProgramming::coin_change(&[2], 3), None);
    }
    #[test]
    fn test_inventory_eoq() {
        let inv = InventoryModel::new(1000.0, 50.0, 5.0, 1.0);
        let q = inv.eoq();
        assert!((q - 200.0_f64.sqrt() * 10.0).abs() < 1e-3, "eoq={q}");
        let tc = inv.total_cost(q);
        assert!(tc > 0.0);
    }
    #[test]
    fn test_simplex_solver_basic() {
        let obj = vec![-1.0, -1.0];
        let a = vec![vec![1.0, 1.0], vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![4.0, 3.0, 3.0];
        let solver = SimplexSolver::new(obj, a, b);
        let val = solver.solve().expect("solve should succeed");
        assert!((val - (-4.0)).abs() < 1e-6, "simplex val={val}");
    }
    #[test]
    fn test_ford_fulkerson_simple() {
        let mut ff = FordFulkerson::new(4);
        ff.add_edge(0, 1, 10);
        ff.add_edge(0, 2, 10);
        ff.add_edge(1, 3, 10);
        ff.add_edge(2, 3, 10);
        assert_eq!(ff.max_flow(0, 3), 20);
    }
    #[test]
    fn test_hungarian_solver_simple() {
        let cost = vec![vec![9, 2, 7], vec![3, 6, 1], vec![5, 8, 4]];
        let (min_cost, assignment) = HungarianSolver::new(cost).solve();
        assert_eq!(
            min_cost, 8,
            "Hungarian cost={min_cost}, assignment={assignment:?}"
        );
    }
    #[test]
    fn test_bellman_ford_basic() {
        let mut bf = BellmanFord::new(5);
        bf.add_edge(0, 1, 6);
        bf.add_edge(0, 2, 7);
        bf.add_edge(1, 2, 8);
        bf.add_edge(1, 3, 5);
        bf.add_edge(1, 4, -4);
        bf.add_edge(2, 3, -3);
        bf.add_edge(2, 4, 9);
        bf.add_edge(3, 1, -2);
        bf.add_edge(4, 0, 2);
        bf.add_edge(4, 3, 7);
        let dist = bf.shortest_paths(0).expect("shortest_paths should succeed");
        assert_eq!(dist[0], 0);
        assert_eq!(dist[1], 2);
        assert_eq!(dist[2], 7);
    }
    #[test]
    fn test_knapsack_dp_with_selection() {
        let solver = KnapsackDP::new(5, vec![2, 3, 4, 5], vec![3, 4, 5, 6]);
        let (val, selected) = solver.solve();
        assert_eq!(val, 7, "knapsack value={val}");
        assert!(
            selected.contains(&0) && selected.contains(&1),
            "selected={selected:?}"
        );
    }
    #[test]
    fn test_dijkstra_basic() {
        let mut g = Dijkstra::new(5);
        g.add_edge(0, 1, 10);
        g.add_edge(0, 3, 5);
        g.add_edge(1, 2, 1);
        g.add_edge(3, 1, 3);
        g.add_edge(3, 2, 9);
        g.add_edge(2, 4, 4);
        g.add_edge(3, 4, 2);
        let dist = g.shortest_paths(0);
        assert_eq!(dist[0], 0);
        assert_eq!(dist[3], 5);
        assert_eq!(dist[1], 8);
        assert_eq!(dist[4], 7);
    }
    #[test]
    fn test_floyd_warshall_basic() {
        let mut fw = FloydWarshall::new(4);
        fw.add_edge(0, 1, 3);
        fw.add_edge(0, 3, 7);
        fw.add_edge(1, 0, 8);
        fw.add_edge(1, 2, 2);
        fw.add_edge(2, 0, 5);
        fw.add_edge(2, 3, 1);
        fw.add_edge(3, 0, 2);
        let d = fw.run().expect("run should succeed");
        assert_eq!(d[0][2], 5);
        assert_eq!(d[3][1], 5);
    }
    #[test]
    fn test_prim_mst() {
        let mut prim = PrimMst::new(4);
        prim.add_edge(0, 1, 1);
        prim.add_edge(0, 2, 4);
        prim.add_edge(1, 2, 2);
        prim.add_edge(1, 3, 5);
        prim.add_edge(2, 3, 3);
        let (total, _edges) = prim.run();
        assert_eq!(total, 6, "MST weight={total}");
    }
    #[test]
    fn test_newsvendor_optimal_quantity() {
        let nv = NewsvendorModel::new(0.0, 100.0, 5.0, 10.0, 2.0);
        let q = nv.optimal_quantity();
        assert!((q - 62.5).abs() < 1e-6, "Q*={q}");
    }
    #[test]
    fn test_reliability_series() {
        let sys = ReliabilitySystem::new(vec![0.9, 0.8, 0.95]);
        let r = sys.series_reliability();
        assert!((r - 0.9 * 0.8 * 0.95).abs() < 1e-9, "series R={r}");
    }
    #[test]
    fn test_reliability_parallel() {
        let sys = ReliabilitySystem::new(vec![0.9, 0.9]);
        let r = sys.parallel_reliability();
        assert!((r - 0.99).abs() < 1e-9, "parallel R={r}");
    }
    #[test]
    fn test_build_env_has_axioms() {
        let env = build_operations_research_env();
        assert!(env.get(&Name::str("ford_fulkerson")).is_some());
        assert!(env.get(&Name::str("simplex_optimal")).is_some());
        assert!(env.get(&Name::str("branch_and_bound")).is_some());
        assert!(env.get(&Name::str("dijkstra_correctness")).is_some());
        assert!(env.get(&Name::str("hungarian_algorithm")).is_some());
        assert!(env.get(&Name::str("robust_optimization")).is_some());
        assert!(env.get(&Name::str("pollaczek_khinchine")).is_some());
        assert!(env.get(&Name::str("MonteCarloEstimator")).is_some());
    }
    #[test]
    fn test_extended_env_has_axioms() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        register_operations_research_extended(&mut env).expect("Environment::new should succeed");
        let names = [
            "BranchAndBoundCompleteness",
            "GomoryCut",
            "DantzigWolfeDecomposition",
            "BendersDecomposition",
            "ColumnGeneration",
            "LagrangianRelaxation",
            "SemidefiniteProgram",
            "SecondOrderConeProgram",
            "MinimaxRegret",
            "ChanceConstraint",
            "MarkovDecisionProcess",
            "BellmanOptimalityEquation",
            "ValueIterationConvergence",
            "PolicyIterationConvergence",
            "POMDP",
            "MultiArmedBandit",
            "UCB1Index",
            "ThompsonSampling",
        ];
        for name in &names {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "Extended axiom '{name}' not found"
            );
        }
    }
    #[test]
    fn test_mdp_value_iteration_simple() {
        let n_states = 2;
        let n_actions = 2;
        let discount = 0.9_f64;
        let rewards = vec![vec![-1.0, 0.0], vec![0.0, 1.0]];
        let transitions = vec![
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![vec![0.0, 1.0], vec![0.0, 1.0]],
        ];
        let mdp = MdpSolver::new(n_states, n_actions, discount, rewards, transitions);
        let (v, _iters) = mdp.value_iteration(1e-6, 1000);
        assert!(
            v[1] > v[0],
            "V(good state) should exceed V(bad state): V={v:?}"
        );
    }
    #[test]
    fn test_mdp_policy_extraction() {
        let n_states = 2;
        let n_actions = 2;
        let discount = 0.9_f64;
        let rewards = vec![vec![0.0, 0.5], vec![0.0, 1.0]];
        let transitions = vec![
            vec![vec![1.0, 0.0], vec![0.0, 1.0]],
            vec![vec![0.0, 1.0], vec![0.0, 1.0]],
        ];
        let mdp = MdpSolver::new(n_states, n_actions, discount, rewards, transitions);
        let (v, _) = mdp.value_iteration(1e-6, 1000);
        let policy = mdp.extract_policy(&v);
        assert_eq!(policy[1], 1, "At state 1, should choose action 1");
    }
    #[test]
    fn test_ucb_bandit_selects_best() {
        let means = vec![0.2, 0.5, 0.8];
        let mut env = BanditEnvironment::new(means.clone());
        assert_eq!(env.optimal_arm(), 2, "Optimal arm should be 2");
        let regret = env.run_ucb1(200);
        assert!(regret >= 0.0, "Regret should be non-negative, got {regret}");
        assert!(regret.is_finite(), "Regret should be finite, got {regret}");
    }
    #[test]
    fn test_ucb1_index_infinite_for_unplayed() {
        let bandit = MultiArmedBanditUcb::new(3);
        assert_eq!(
            bandit.ucb_index(0),
            f64::INFINITY,
            "Unplayed arm should have infinite UCB"
        );
        assert_eq!(bandit.select_arm(), 0, "Should select first unplayed arm");
    }
    #[test]
    fn test_lagrangian_solver_polyak_step() {
        let mut solver = LagrangianRelaxationSolver::new(2, 1.0);
        let subgradient = [0.5, -0.3];
        let step = solver.polyak_step(10.0, 8.0, &subgradient);
        assert!(step > 0.0, "Polyak step should be positive, got {step}");
        solver.subgradient_update(&subgradient, step);
        assert!(
            solver.multipliers[0] > 0.0,
            "Multiplier 0 should increase: {}",
            solver.multipliers[0]
        );
        assert!(
            solver.multipliers[1] >= 0.0,
            "Multiplier 1 should be non-negative: {}",
            solver.multipliers[1]
        );
    }
    #[test]
    fn test_two_stage_stochastic_cost() {
        let model = TwoStageStochasticLP::new(
            vec![2.0],
            vec![3.0],
            vec![0.4, 0.6],
            vec![vec![5.0], vec![7.0]],
        );
        let x = vec![1.0];
        let y = vec![vec![2.0], vec![3.0]];
        let expected_recourse = 0.4 * 6.0 + 0.6 * 9.0;
        let recourse = model.expected_recourse_cost(&y);
        assert!(
            (recourse - expected_recourse).abs() < 1e-9,
            "Expected recourse={expected_recourse}, got {recourse}"
        );
        let total = model.total_cost(&x, &y);
        assert!(
            (total - (2.0 + expected_recourse)).abs() < 1e-9,
            "Total cost={total}"
        );
        assert!(model.is_stage1_feasible(&x), "x=[1.0] should be feasible");
    }
}
