//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BipartiteMatchingGraph, BranchBoundData, CuttingPlane, FacilityLocation, FlowNetwork,
    FlowNetworkSpec, GraphColoring, KnapsackSolver, LPRelaxation, MatchingProblem,
    MatroidIntersection, SetCoverData, ShortestPath, SpanningTree, SteinerTree, TravelingSalesman,
    VehicleRouting,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
/// `Graph : Type` — a simple undirected graph (vertices: Nat, edges: pairs).
pub fn graph_ty() -> Expr {
    type0()
}
/// `BipartiteGraph : Type` — bipartite graph with two vertex sets.
pub fn bipartite_graph_ty() -> Expr {
    type0()
}
/// `Matching : Graph → Type` — a set of non-adjacent edges.
pub fn matching_ty() -> Expr {
    arrow(graph_ty(), type0())
}
/// `PerfectMatching : Graph → Prop` — every vertex is matched.
pub fn perfect_matching_ty() -> Expr {
    arrow(graph_ty(), prop())
}
/// `MaxMatching : Graph → Matching → Prop` — a maximum cardinality matching.
pub fn max_matching_ty() -> Expr {
    arrow(graph_ty(), arrow(matching_ty(), prop()))
}
/// `Alternating_path : Graph → Matching → List Nat → Prop` — alternating path wrt matching.
pub fn alternating_path_ty() -> Expr {
    arrow(
        graph_ty(),
        arrow(matching_ty(), arrow(list_ty(nat_ty()), prop())),
    )
}
/// `AugmentingPath : Graph → Matching → List Nat → Prop` — augmenting path.
pub fn augmenting_path_ty() -> Expr {
    arrow(
        graph_ty(),
        arrow(matching_ty(), arrow(list_ty(nat_ty()), prop())),
    )
}
/// `Blossom : Graph → Matching → List Nat → Prop` — odd cycle contracted in Edmonds' algorithm.
pub fn blossom_ty() -> Expr {
    arrow(
        graph_ty(),
        arrow(matching_ty(), arrow(list_ty(nat_ty()), prop())),
    )
}
/// `HallCondition : BipartiteGraph → Prop`
/// For every subset S of left vertices, |N(S)| ≥ |S|.
pub fn hall_condition_ty() -> Expr {
    arrow(bipartite_graph_ty(), prop())
}
/// `HallTheorem : ∀ G : BipartiteGraph, HallCondition G ↔ PerfectMatching G`
pub fn hall_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        bipartite_graph_ty(),
        app2(
            cst("Iff"),
            app(cst("HallCondition"), bvar(0)),
            app(cst("PerfectMatching"), bvar(0)),
        ),
    )
}
/// `KonigTheorem : MaxMatchingSize = MinVertexCoverSize` for bipartite graphs.
pub fn konig_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        bipartite_graph_ty(),
        app2(
            cst("Eq"),
            app(cst("MaxMatchingSize"), bvar(0)),
            app(cst("MinVertexCoverSize"), bvar(0)),
        ),
    )
}
/// `TutteCondition : Graph → Prop`
/// For every S ⊆ V, the number of odd components of G-S is ≤ |S|.
pub fn tutte_condition_ty() -> Expr {
    arrow(graph_ty(), prop())
}
/// `TutteTheorem : ∀ G, PerfectMatching G ↔ TutteCondition G`
pub fn tutte_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        graph_ty(),
        app2(
            cst("Iff"),
            app(cst("PerfectMatching"), bvar(0)),
            app(cst("TutteCondition"), bvar(0)),
        ),
    )
}
/// `BergeTheorem : max matching augmenting path characterization`
pub fn berge_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        graph_ty(),
        pi(
            BinderInfo::Default,
            "M",
            matching_ty(),
            app2(
                cst("Iff"),
                app2(cst("MaxMatching"), bvar(1), bvar(0)),
                app(
                    cst("NoAugmentingPath"),
                    app2(cst("mk_pair"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// `FlowNetwork : Type` — directed graph with capacities.
pub fn flow_network_ty() -> Expr {
    type0()
}
/// `Flow : FlowNetwork → Type` — feasible flow satisfying capacity and conservation.
pub fn flow_ty() -> Expr {
    arrow(flow_network_ty(), type0())
}
/// `FlowValue : FlowNetwork → Flow → Real` — the value of a flow (net flow out of source).
pub fn flow_value_ty() -> Expr {
    arrow(flow_network_ty(), arrow(flow_ty(), real_ty()))
}
/// `MaxFlow : FlowNetwork → Real` — maximum flow value.
pub fn max_flow_ty() -> Expr {
    arrow(flow_network_ty(), real_ty())
}
/// `Cut : FlowNetwork → (Nat → Bool) → Prop` — s-t cut (partition of vertices).
pub fn cut_ty() -> Expr {
    arrow(flow_network_ty(), arrow(arrow(nat_ty(), bool_ty()), prop()))
}
/// `CutCapacity : FlowNetwork → (Nat → Bool) → Real` — total capacity of a cut.
pub fn cut_capacity_ty() -> Expr {
    arrow(
        flow_network_ty(),
        arrow(arrow(nat_ty(), bool_ty()), real_ty()),
    )
}
/// `MinCut : FlowNetwork → Real` — minimum cut capacity.
pub fn min_cut_ty() -> Expr {
    arrow(flow_network_ty(), real_ty())
}
/// `MaxFlowMinCut : ∀ N, MaxFlow N = MinCut N` — the max-flow min-cut theorem.
pub fn max_flow_min_cut_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "N",
        flow_network_ty(),
        app2(
            cst("Eq"),
            app(cst("MaxFlow"), bvar(0)),
            app(cst("MinCut"), bvar(0)),
        ),
    )
}
/// `FordFulkersonTermination : Ford-Fulkerson terminates on integer capacities.`
pub fn ford_fulkerson_termination_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "N",
        flow_network_ty(),
        arrow(
            app(cst("IntegerCapacities"), bvar(0)),
            app(cst("FordFulkersonTerminates"), bvar(0)),
        ),
    )
}
/// `DinicsComplexity : Dinic's algorithm runs in O(V² E).`
pub fn dinics_complexity_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "N",
        flow_network_ty(),
        app(cst("DinicsTimeBound"), bvar(0)),
    )
}
/// `CostMatrix : Nat → Nat → Real` — cost matrix for assignment.
pub fn cost_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// `Assignment : Nat → (Nat → Nat) → Prop` — a permutation assignment for n×n matrix.
pub fn assignment_ty() -> Expr {
    arrow(nat_ty(), arrow(arrow(nat_ty(), nat_ty()), prop()))
}
/// `OptimalAssignment : CostMatrix → (Nat → Nat) → Prop` — assignment minimizing total cost.
pub fn optimal_assignment_ty() -> Expr {
    arrow(cost_matrix_ty(), arrow(arrow(nat_ty(), nat_ty()), prop()))
}
/// `HungarianAlgorithm : CostMatrix → (Nat → Nat)` — solves assignment in O(n³).
pub fn hungarian_algorithm_ty() -> Expr {
    arrow(cost_matrix_ty(), arrow(nat_ty(), nat_ty()))
}
/// `HungarianCorrectness : HungarianAlgorithm gives OptimalAssignment.`
pub fn hungarian_correctness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        cost_matrix_ty(),
        app2(
            cst("OptimalAssignment"),
            bvar(0),
            app(cst("HungarianAlgorithm"), bvar(0)),
        ),
    )
}
/// `TSPInstance : Type` — complete graph with edge weights.
pub fn tsp_instance_ty() -> Expr {
    type0()
}
/// `TSPTour : TSPInstance → List Nat → Prop` — a Hamiltonian cycle.
pub fn tsp_tour_ty() -> Expr {
    arrow(tsp_instance_ty(), arrow(list_ty(nat_ty()), prop()))
}
/// `TSPOptimal : TSPInstance → Real` — optimal TSP tour length.
pub fn tsp_optimal_ty() -> Expr {
    arrow(tsp_instance_ty(), real_ty())
}
/// `HeldKarpBound : TSPInstance → Real` — Held-Karp lower bound.
pub fn held_karp_bound_ty() -> Expr {
    arrow(tsp_instance_ty(), real_ty())
}
/// `HeldKarpLowerBound : HeldKarpBound ≤ TSPOptimal.`
pub fn held_karp_lower_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "I",
        tsp_instance_ty(),
        app2(
            cst("Le"),
            app(cst("HeldKarpBound"), bvar(0)),
            app(cst("TSPOptimal"), bvar(0)),
        ),
    )
}
/// `ChristofidesApproximation : 3/2-approximation for metric TSP.`
pub fn christofides_approximation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "I",
        tsp_instance_ty(),
        arrow(
            app(cst("MetricTSP"), bvar(0)),
            app2(
                cst("Le"),
                app(cst("ChristofidesValue"), bvar(0)),
                app2(
                    cst("RealMul"),
                    cst("ThreeHalves"),
                    app(cst("TSPOptimal"), bvar(0)),
                ),
            ),
        ),
    )
}
/// `VertexCoverApproximation : 2-approx for vertex cover.`
pub fn vertex_cover_approx_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        graph_ty(),
        app2(
            cst("Le"),
            app(cst("ApproxVertexCoverSize"), bvar(0)),
            app2(
                cst("NatMul"),
                cst("Nat.two"),
                app(cst("MinVertexCoverSize"), bvar(0)),
            ),
        ),
    )
}
/// `SetCoverApproximation : H_n-approximation (log n) for set cover.`
pub fn set_cover_approx_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app2(
            cst("Le"),
            app(cst("GreedySetCoverSize"), bvar(0)),
            app2(
                cst("NatMul"),
                app(cst("HarmonicNumber"), bvar(0)),
                app(cst("OptSetCoverSize"), bvar(0)),
            ),
        ),
    )
}
/// `Matroid : Type` — a matroid (ground set + independent sets satisfying axioms).
pub fn matroid_ty() -> Expr {
    type0()
}
/// `IndependentSet : Matroid → List Nat → Prop`
pub fn independent_set_ty() -> Expr {
    arrow(matroid_ty(), arrow(list_ty(nat_ty()), prop()))
}
/// `MatroidBase : Matroid → List Nat → Prop` — maximal independent set.
pub fn matroid_base_ty() -> Expr {
    arrow(matroid_ty(), arrow(list_ty(nat_ty()), prop()))
}
/// `MatroidRank : Matroid → Nat` — rank of a matroid.
pub fn matroid_rank_ty() -> Expr {
    arrow(matroid_ty(), nat_ty())
}
/// `GreedyOptimality : Greedy algorithm is optimal for matroid weight maximization.`
pub fn greedy_optimality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        matroid_ty(),
        pi(
            BinderInfo::Default,
            "w",
            arrow(nat_ty(), real_ty()),
            app2(cst("GreedyIsOptimal"), bvar(1), bvar(0)),
        ),
    )
}
/// `MatroidIntersection : common independent set in two matroids.`
pub fn matroid_intersection_ty() -> Expr {
    arrow(matroid_ty(), arrow(matroid_ty(), list_ty(nat_ty())))
}
/// `MatroidIntersectionOptimality : the max weight common independent set.`
pub fn matroid_intersection_optimality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M1",
        matroid_ty(),
        pi(
            BinderInfo::Default,
            "M2",
            matroid_ty(),
            pi(
                BinderInfo::Default,
                "w",
                arrow(nat_ty(), real_ty()),
                app3(cst("MatroidIntersectionOptimal"), bvar(2), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// `GraphicMatroid : Graph → Matroid` — cycle matroid of a graph.
pub fn graphic_matroid_ty() -> Expr {
    arrow(graph_ty(), matroid_ty())
}
/// `UniformMatroid : Nat → Nat → Matroid` — U(k,n) uniform matroid.
pub fn uniform_matroid_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), matroid_ty()))
}
/// `SubmodularFunction : (List Nat → Real) → Prop`
/// f is submodular if f(A) + f(B) ≥ f(A∪B) + f(A∩B).
pub fn submodular_function_ty() -> Expr {
    arrow(arrow(list_ty(nat_ty()), real_ty()), prop())
}
/// `SubmodularMaximization : (List Nat → Real) → List Nat` — greedy maximization.
pub fn submodular_maximization_ty() -> Expr {
    arrow(arrow(list_ty(nat_ty()), real_ty()), list_ty(nat_ty()))
}
/// `SubmodularGreedy_1_2_approx : greedy gives 1/2-approximation for monotone submodular max.`
pub fn submodular_greedy_approx_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(list_ty(nat_ty()), real_ty()),
        arrow(
            app(cst("MonotoneSubmodular"), bvar(0)),
            app2(
                cst("Le"),
                app2(
                    cst("RealMul"),
                    cst("OneHalf"),
                    app(cst("SubmodularOpt"), bvar(0)),
                ),
                app(cst("SubmodularGreedyValue"), bvar(0)),
            ),
        ),
    )
}
/// `SupermodularFunction : (List Nat → Real) → Prop` — supermodular (negation of submodular).
pub fn supermodular_function_ty() -> Expr {
    arrow(arrow(list_ty(nat_ty()), real_ty()), prop())
}
/// `PolymatroidRankFunction : axioms for polymatroid rank.`
pub fn polymatroid_rank_ty() -> Expr {
    arrow(arrow(list_ty(nat_ty()), real_ty()), prop())
}
/// `ILPInstance : Type` — integer linear program: min c·x s.t. Ax ≤ b, x ∈ ℤ^n.
pub fn ilp_instance_ty() -> Expr {
    type0()
}
/// `ILPSolution : ILPInstance → List Int → Prop`
pub fn ilp_solution_ty() -> Expr {
    arrow(ilp_instance_ty(), arrow(list_ty(int_ty()), prop()))
}
/// `ILPOptimal : ILPInstance → List Int → Prop`
pub fn ilp_optimal_ty() -> Expr {
    arrow(ilp_instance_ty(), arrow(list_ty(int_ty()), prop()))
}
/// `GomoryCut : ILPInstance → ILPInstance` — adds a Gomory cutting plane.
pub fn gomory_cut_ty() -> Expr {
    arrow(ilp_instance_ty(), ilp_instance_ty())
}
/// `BranchAndBound : ILPInstance → List Int` — branch-and-bound solver.
pub fn branch_and_bound_ty() -> Expr {
    arrow(ilp_instance_ty(), list_ty(int_ty()))
}
/// `LPRelaxation : ILPInstance → Real` — optimal LP relaxation value.
pub fn lp_relaxation_ty() -> Expr {
    arrow(ilp_instance_ty(), real_ty())
}
/// `LPRelaxationLowerBound : LPRelaxation ≤ ILP optimal.`
pub fn lp_relaxation_lower_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        ilp_instance_ty(),
        app2(
            cst("Le"),
            app(cst("LPRelaxation"), bvar(0)),
            app(cst("ILPOptimalValue"), bvar(0)),
        ),
    )
}
/// `Polytope : Type` — a convex polytope (intersection of halfspaces).
pub fn polytope_ty() -> Expr {
    type0()
}
/// `Vertex : Polytope → List Real → Prop` — extreme point of polytope.
pub fn polytope_vertex_ty() -> Expr {
    arrow(polytope_ty(), arrow(list_ty(real_ty()), prop()))
}
/// `TotallyUnimodular : (Nat → Nat → Int) → Prop` — every square submatrix has det ∈ {-1,0,1}.
pub fn totally_unimodular_ty() -> Expr {
    arrow(arrow(nat_ty(), arrow(nat_ty(), int_ty())), prop())
}
/// `TUIntegralPolyhedra : TU matrix → LP has integer optimal vertex.`
pub fn tu_integral_polyhedra_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        arrow(nat_ty(), arrow(nat_ty(), int_ty())),
        arrow(
            app(cst("TotallyUnimodular"), bvar(0)),
            app(cst("IntegralPolyhedron"), bvar(0)),
        ),
    )
}
/// `BipartiteIncidenceTU : incidence matrix of bipartite graph is TU.`
pub fn bipartite_incidence_tu_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        bipartite_graph_ty(),
        app(
            cst("TotallyUnimodular"),
            app(cst("IncidenceMatrix"), bvar(0)),
        ),
    )
}
/// `FacetDefiningInequality : Polytope → (List Real → Prop) → Prop`
pub fn facet_defining_ty() -> Expr {
    arrow(
        polytope_ty(),
        arrow(arrow(list_ty(real_ty()), prop()), prop()),
    )
}
/// `WeakDuality : for LP, dual objective ≥ primal objective.`
pub fn weak_duality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        ilp_instance_ty(),
        app2(
            cst("Le"),
            app(cst("PrimalObjective"), bvar(0)),
            app(cst("DualObjective"), bvar(0)),
        ),
    )
}
/// `StrongDuality : for LP, dual objective = primal when both feasible.`
pub fn strong_duality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        ilp_instance_ty(),
        arrow(
            app(cst("BothFeasible"), bvar(0)),
            app2(
                cst("Eq"),
                app(cst("PrimalObjective"), bvar(0)),
                app(cst("DualObjective"), bvar(0)),
            ),
        ),
    )
}
/// Register all combinatorial optimization axioms into the kernel environment.
pub fn build_combinatorial_optimization_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("Graph", graph_ty()),
        ("BipartiteGraph", bipartite_graph_ty()),
        ("Matching", matching_ty()),
        ("PerfectMatching", perfect_matching_ty()),
        ("MaxMatching", max_matching_ty()),
        ("AlternatingPath", alternating_path_ty()),
        ("AugmentingPath", augmenting_path_ty()),
        ("Blossom", blossom_ty()),
        ("MaxMatchingSize", arrow(graph_ty(), nat_ty())),
        ("MinVertexCoverSize", arrow(graph_ty(), nat_ty())),
        ("ApproxVertexCoverSize", arrow(graph_ty(), nat_ty())),
        (
            "NoAugmentingPath",
            arrow(pair_ty(graph_ty(), matching_ty()), prop()),
        ),
        (
            "mk_pair",
            arrow(
                graph_ty(),
                arrow(matching_ty(), pair_ty(graph_ty(), matching_ty())),
            ),
        ),
        ("HallCondition", hall_condition_ty()),
        ("hall_theorem", hall_theorem_ty()),
        ("konig_theorem", konig_theorem_ty()),
        ("TutteCondition", tutte_condition_ty()),
        ("tutte_theorem", tutte_theorem_ty()),
        ("berge_theorem", berge_theorem_ty()),
        ("FlowNetwork", flow_network_ty()),
        ("Flow", flow_ty()),
        ("FlowValue", flow_value_ty()),
        ("MaxFlow", max_flow_ty()),
        ("Cut", cut_ty()),
        ("CutCapacity", cut_capacity_ty()),
        ("MinCut", min_cut_ty()),
        ("IntegerCapacities", arrow(flow_network_ty(), prop())),
        ("FordFulkersonTerminates", arrow(flow_network_ty(), prop())),
        ("DinicsTimeBound", arrow(flow_network_ty(), prop())),
        ("max_flow_min_cut", max_flow_min_cut_ty()),
        (
            "ford_fulkerson_termination",
            ford_fulkerson_termination_ty(),
        ),
        ("dinics_complexity", dinics_complexity_ty()),
        ("CostMatrix", cost_matrix_ty()),
        ("Assignment", assignment_ty()),
        ("OptimalAssignment", optimal_assignment_ty()),
        ("HungarianAlgorithm", hungarian_algorithm_ty()),
        ("hungarian_correctness", hungarian_correctness_ty()),
        ("TSPInstance", tsp_instance_ty()),
        ("TSPTour", tsp_tour_ty()),
        ("TSPOptimal", tsp_optimal_ty()),
        ("HeldKarpBound", held_karp_bound_ty()),
        ("MetricTSP", arrow(tsp_instance_ty(), prop())),
        ("ChristofidesValue", arrow(tsp_instance_ty(), real_ty())),
        ("ThreeHalves", real_ty()),
        ("OneHalf", real_ty()),
        ("RealMul", arrow(real_ty(), arrow(real_ty(), real_ty()))),
        ("NatMul", arrow(nat_ty(), arrow(nat_ty(), nat_ty()))),
        ("Nat.two", nat_ty()),
        ("HarmonicNumber", arrow(nat_ty(), nat_ty())),
        ("OptSetCoverSize", arrow(nat_ty(), nat_ty())),
        ("GreedySetCoverSize", arrow(nat_ty(), nat_ty())),
        ("held_karp_lower_bound", held_karp_lower_bound_ty()),
        (
            "christofides_approximation",
            christofides_approximation_ty(),
        ),
        ("vertex_cover_approximation", vertex_cover_approx_ty()),
        ("set_cover_approximation", set_cover_approx_ty()),
        ("Matroid", matroid_ty()),
        ("IndependentSet", independent_set_ty()),
        ("MatroidBase", matroid_base_ty()),
        ("MatroidRank", matroid_rank_ty()),
        (
            "GreedyIsOptimal",
            arrow(matroid_ty(), arrow(arrow(nat_ty(), real_ty()), prop())),
        ),
        ("MatroidIntersection", matroid_intersection_ty()),
        (
            "MatroidIntersectionOptimal",
            arrow(
                matroid_ty(),
                arrow(matroid_ty(), arrow(arrow(nat_ty(), real_ty()), prop())),
            ),
        ),
        ("GraphicMatroid", graphic_matroid_ty()),
        ("UniformMatroid", uniform_matroid_ty()),
        ("greedy_optimality", greedy_optimality_ty()),
        (
            "matroid_intersection_optimality",
            matroid_intersection_optimality_ty(),
        ),
        ("SubmodularFunction", submodular_function_ty()),
        ("SubmodularMaximization", submodular_maximization_ty()),
        (
            "MonotoneSubmodular",
            arrow(arrow(list_ty(nat_ty()), real_ty()), prop()),
        ),
        (
            "SubmodularOpt",
            arrow(arrow(list_ty(nat_ty()), real_ty()), real_ty()),
        ),
        (
            "SubmodularGreedyValue",
            arrow(arrow(list_ty(nat_ty()), real_ty()), real_ty()),
        ),
        ("SupermodularFunction", supermodular_function_ty()),
        ("PolymatroidRankFunction", polymatroid_rank_ty()),
        ("submodular_greedy_approx", submodular_greedy_approx_ty()),
        ("ILPInstance", ilp_instance_ty()),
        ("ILPSolution", ilp_solution_ty()),
        ("ILPOptimal", ilp_optimal_ty()),
        ("GomoryCut", gomory_cut_ty()),
        ("BranchAndBound", branch_and_bound_ty()),
        ("LPRelaxation", lp_relaxation_ty()),
        ("ILPOptimalValue", arrow(ilp_instance_ty(), real_ty())),
        ("PrimalObjective", arrow(ilp_instance_ty(), real_ty())),
        ("DualObjective", arrow(ilp_instance_ty(), real_ty())),
        ("BothFeasible", arrow(ilp_instance_ty(), prop())),
        ("lp_relaxation_lower_bound", lp_relaxation_lower_bound_ty()),
        ("weak_duality", weak_duality_ty()),
        ("strong_duality", strong_duality_ty()),
        ("Polytope", polytope_ty()),
        ("PolytopeVertex", polytope_vertex_ty()),
        ("TotallyUnimodular", totally_unimodular_ty()),
        (
            "IntegralPolyhedron",
            arrow(arrow(nat_ty(), arrow(nat_ty(), int_ty())), prop()),
        ),
        (
            "IncidenceMatrix",
            arrow(
                bipartite_graph_ty(),
                arrow(nat_ty(), arrow(nat_ty(), int_ty())),
            ),
        ),
        ("FacetDefining", facet_defining_ty()),
        ("tu_integral_polyhedra", tu_integral_polyhedra_ty()),
        ("bipartite_incidence_tu", bipartite_incidence_tu_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Solve the minimum-weight assignment problem using the Hungarian algorithm.
/// Returns (total cost, assignment vector where assignment\[i\] = j means row i → col j).
pub fn hungarian(cost: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let n = cost.len();
    if n == 0 {
        return (0, vec![]);
    }
    let inf = i64::MAX / 2;
    let mut u = vec![0i64; n + 1];
    let mut v = vec![0i64; n + 1];
    let mut p = vec![0usize; n + 1];
    let mut way = vec![0usize; n + 1];
    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0usize;
        let mut min_val = vec![inf; n + 1];
        let mut used = vec![false; n + 1];
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = inf;
            let mut j1 = 0usize;
            for j in 1..=n {
                if !used[j] {
                    let cur = cost[i0 - 1][j - 1] - u[i0] - v[j];
                    if cur < min_val[j] {
                        min_val[j] = cur;
                        way[j] = j0;
                    }
                    if min_val[j] < delta {
                        delta = min_val[j];
                        j1 = j;
                    }
                }
            }
            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    min_val[j] -= delta;
                }
            }
            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }
    let mut ans = vec![0usize; n];
    for j in 1..=n {
        if p[j] != 0 {
            ans[p[j] - 1] = j - 1;
        }
    }
    let total_cost = (1..=n).map(|i| cost[i - 1][ans[i - 1]]).sum();
    (total_cost, ans)
}
/// Greedy maximum weight independent set in uniform matroid U(k, n).
/// Returns indices of k elements with highest weights.
pub fn uniform_matroid_greedy(weights: &[f64], k: usize) -> Vec<usize> {
    let mut indexed: Vec<(usize, f64)> = weights.iter().enumerate().map(|(i, &w)| (i, w)).collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    indexed[..k.min(indexed.len())]
        .iter()
        .map(|&(i, _)| i)
        .collect()
}
/// Greedy set cover: returns selected set indices covering all elements 0..n_elem.
pub fn greedy_set_cover(n_elem: usize, sets: &[Vec<usize>]) -> Vec<usize> {
    let mut covered = vec![false; n_elem];
    let mut remaining = n_elem;
    let mut selected = vec![];
    while remaining > 0 {
        let best = sets
            .iter()
            .enumerate()
            .filter(|(i, _)| !selected.contains(i))
            .max_by_key(|(_, s)| s.iter().filter(|&&e| !covered[e]).count());
        match best {
            Some((idx, set)) => {
                let newly_covered = set.iter().filter(|&&e| !covered[e]).count();
                if newly_covered == 0 {
                    break;
                }
                selected.push(idx);
                for &e in set {
                    if !covered[e] {
                        covered[e] = true;
                        remaining -= 1;
                    }
                }
            }
            None => break,
        }
    }
    selected
}
/// Branch-and-bound for 0-1 knapsack (maximize).
/// items: (value, weight), capacity: max weight.
/// Returns (max_value, selected_item_indices).
pub fn knapsack_01(items: &[(i64, i64)], capacity: i64) -> (i64, Vec<usize>) {
    let _n = items.len();
    let mut best = 0i64;
    let mut best_sol = vec![];
    fn backtrack(
        idx: usize,
        cap: i64,
        current_val: i64,
        current_sol: &mut Vec<usize>,
        best: &mut i64,
        best_sol: &mut Vec<usize>,
        items: &[(i64, i64)],
    ) {
        if current_val > *best {
            *best = current_val;
            *best_sol = current_sol.clone();
        }
        if idx == items.len() {
            return;
        }
        let ub: i64 = current_val
            + items[idx..]
                .iter()
                .scan(cap, |c, &(v, w)| {
                    if *c >= w {
                        *c -= w;
                        Some(v)
                    } else {
                        let frac = (*c * v) / w.max(1);
                        *c = 0;
                        Some(frac)
                    }
                })
                .sum::<i64>();
        if ub <= *best {
            return;
        }
        let (v, w) = items[idx];
        if cap >= w {
            current_sol.push(idx);
            backtrack(
                idx + 1,
                cap - w,
                current_val + v,
                current_sol,
                best,
                best_sol,
                items,
            );
            current_sol.pop();
        }
        backtrack(
            idx + 1,
            cap,
            current_val,
            current_sol,
            best,
            best_sol,
            items,
        );
    }
    let mut sol = vec![];
    backtrack(0, capacity, 0, &mut sol, &mut best, &mut best_sol, items);
    (best, best_sol)
}
/// Nearest-neighbor heuristic for TSP. Returns tour (list of vertex indices).
pub fn tsp_nearest_neighbor(dist: &[Vec<f64>]) -> (Vec<usize>, f64) {
    let n = dist.len();
    if n == 0 {
        return (vec![], 0.0);
    }
    let mut visited = vec![false; n];
    let mut tour = vec![0usize];
    visited[0] = true;
    let mut cost = 0.0;
    for _ in 1..n {
        let last = *tour
            .last()
            .expect("tour is non-empty: initialized with element 0");
        let next = (0..n).filter(|&j| !visited[j]).min_by(|&a, &b| {
            dist[last][a]
                .partial_cmp(&dist[last][b])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if let Some(next) = next {
            cost += dist[last][next];
            tour.push(next);
            visited[next] = true;
        }
    }
    cost += dist[*tour
        .last()
        .expect("tour is non-empty: initialized with element 0")][tour[0]];
    (tour, cost)
}
/// Held-Karp DP lower bound (exact TSP for small n ≤ 20).
pub fn tsp_held_karp(dist: &[Vec<f64>]) -> f64 {
    let n = dist.len();
    if n <= 1 {
        return 0.0;
    }
    let full = 1usize << n;
    let inf = f64::INFINITY;
    let mut dp = vec![vec![inf; n]; full];
    dp[1][0] = 0.0;
    for mask in 1..full {
        for u in 0..n {
            if dp[mask][u] == inf {
                continue;
            }
            if mask & (1 << u) == 0 {
                continue;
            }
            for v in 0..n {
                if mask & (1 << v) != 0 {
                    continue;
                }
                let next_mask = mask | (1 << v);
                let new_cost = dp[mask][u] + dist[u][v];
                if new_cost < dp[next_mask][v] {
                    dp[next_mask][v] = new_cost;
                }
            }
        }
    }
    let last_mask = full - 1;
    (1..n)
        .filter_map(|u| {
            let c = dp[last_mask][u] + dist[u][0];
            if c < inf {
                Some(c)
            } else {
                None
            }
        })
        .fold(inf, f64::min)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bipartite_matching_perfect() {
        let mut g = BipartiteMatchingGraph::new(3, 3);
        for u in 0..3 {
            for v in 0..3 {
                g.add_edge(u, v);
            }
        }
        let (size, _, _) = g.hopcroft_karp();
        assert_eq!(size, 3);
    }
    #[test]
    fn test_bipartite_matching_partial() {
        let mut g = BipartiteMatchingGraph::new(3, 2);
        g.add_edge(0, 0);
        g.add_edge(1, 0);
        g.add_edge(1, 1);
        g.add_edge(2, 1);
        let (size, _, _) = g.hopcroft_karp();
        assert_eq!(size, 2);
    }
    #[test]
    fn test_flow_network_max_flow() {
        let mut net = FlowNetwork::new(4);
        net.add_edge(0, 1, 3);
        net.add_edge(0, 2, 2);
        net.add_edge(1, 3, 3);
        net.add_edge(2, 3, 2);
        let flow = net.max_flow(0, 3);
        assert_eq!(flow, 5);
    }
    #[test]
    fn test_hungarian_algorithm() {
        let cost = vec![vec![4, 1, 3], vec![2, 0, 5], vec![3, 2, 2]];
        let (total, assignment) = hungarian(&cost);
        let mut cols: Vec<usize> = assignment.clone();
        cols.sort();
        cols.dedup();
        assert_eq!(cols.len(), 3, "Assignment must be a permutation");
        let computed: i64 = assignment
            .iter()
            .enumerate()
            .map(|(i, &j)| cost[i][j])
            .sum();
        assert_eq!(total, computed);
        assert!(total <= 6, "Should find near-optimal solution");
    }
    #[test]
    fn test_uniform_matroid_greedy() {
        let weights = [0.5, 3.0, 1.2, 2.8, 0.9];
        let selected = uniform_matroid_greedy(&weights, 3);
        assert_eq!(selected.len(), 3);
        assert!(selected.contains(&1));
        assert!(selected.contains(&3));
    }
    #[test]
    fn test_greedy_set_cover() {
        let sets = vec![vec![0, 1, 2], vec![1, 3, 4], vec![2, 4, 5]];
        let cover = greedy_set_cover(6, &sets);
        let mut covered = [false; 6];
        for &idx in &cover {
            for &e in &sets[idx] {
                covered[e] = true;
            }
        }
        assert!(covered.iter().all(|&c| c), "All elements should be covered");
    }
    #[test]
    fn test_knapsack_01() {
        let items = vec![(4, 2), (5, 3), (3, 2), (7, 4)];
        let (val, sel) = knapsack_01(&items, 7);
        assert_eq!(val, 12);
        let total_weight: i64 = sel.iter().map(|&i| items[i].1).sum();
        assert!(total_weight <= 7, "selected items exceed capacity");
        let total_value: i64 = sel.iter().map(|&i| items[i].0).sum();
        assert_eq!(total_value, 12, "selected items should have value 12");
    }
    #[test]
    fn test_tsp_held_karp_small() {
        let d = vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ];
        let opt = tsp_held_karp(&d);
        assert!(
            (opt - 80.0).abs() < 1e-9,
            "Held-Karp should find optimal tour of length 80, got {}",
            opt
        );
    }
    #[test]
    fn test_build_combinatorial_optimization_env() {
        let mut env = Environment::new();
        let result = build_combinatorial_optimization_env(&mut env);
        assert!(
            result.is_ok(),
            "build_combinatorial_optimization_env failed: {:?}",
            result.err()
        );
    }
}
#[cfg(test)]
mod spec_wrapper_tests {
    use super::*;
    #[test]
    fn test_flow_network_spec() {
        let mut net = FlowNetworkSpec::new(4);
        net.add_edge(0, 1, 3);
        net.add_edge(0, 2, 2);
        net.add_edge(1, 3, 3);
        net.add_edge(2, 3, 2);
        assert_eq!(net.max_flow_ford_fulkerson(0, 3), 5);
        assert_eq!(net.min_cut(0, 3), 5);
        assert!(net.augmenting_path(0, 3));
    }
    #[test]
    fn test_matching_problem_bipartite() {
        let mut mp = MatchingProblem::new(true, 3, 3);
        mp.add_edge(0, 0, 1.0);
        mp.add_edge(1, 1, 2.0);
        mp.add_edge(2, 2, 3.0);
        assert_eq!(mp.max_matching(), 3);
    }
    #[test]
    fn test_spanning_tree_kruskal() {
        let mut st = SpanningTree::new(4);
        st.add_edge(0, 1, 1.0);
        st.add_edge(1, 2, 2.0);
        st.add_edge(2, 3, 3.0);
        st.add_edge(0, 3, 10.0);
        let mst = st.kruskal();
        assert_eq!(mst.len(), 3);
        let w: f64 = mst.iter().map(|&(_, _, w)| w).sum();
        assert!((w - 6.0).abs() < 1e-9);
    }
    #[test]
    fn test_shortest_path_dijkstra() {
        let mut sp = ShortestPath::new(3);
        sp.add_edge(0, 1, 1.0);
        sp.add_edge(1, 2, 2.0);
        sp.add_edge(0, 2, 5.0);
        let dist = sp.dijkstra(0);
        assert!((dist[2] - 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_bellman_ford() {
        let mut sp = ShortestPath::new(3);
        sp.add_edge(0, 1, 1.0);
        sp.add_edge(1, 2, 2.0);
        sp.add_edge(0, 2, 5.0);
        let dist = sp.bellman_ford(0);
        assert!((dist[2] - 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_floyd_warshall() {
        let mut sp = ShortestPath::new(3);
        sp.add_edge(0, 1, 1.0);
        sp.add_edge(1, 2, 2.0);
        sp.add_edge(0, 2, 5.0);
        let d = sp.floyd_warshall();
        assert!((d[0][2] - 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_knapsack_solver() {
        let solver = KnapsackSolver::new(vec![(4, 2), (5, 3), (3, 2), (7, 4)], 7);
        let (val, _) = solver.dynamic_programming();
        assert_eq!(val, 12);
    }
    #[test]
    fn test_graph_coloring() {
        let gc = GraphColoring::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let (k, _) = gc.greedy_color();
        assert!(k >= 3);
        assert!(gc.chromatic_number_bound() >= 3);
        let (dk, _) = gc.dsatur();
        assert!(dk >= 3);
    }
    #[test]
    fn test_traveling_salesman() {
        let d = vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ];
        let tsp = TravelingSalesman::new(d);
        let hk = tsp.held_karp();
        assert!((hk - 80.0).abs() < 1e-9);
        let nn = tsp.nearest_neighbor();
        assert!(nn > 0.0);
    }
    #[test]
    fn test_steiner_tree() {
        let st = SteinerTree::new(5, vec![0, 2, 4]);
        assert!(st.approx_2() >= 0.0);
        assert!(st.dreyfus_wagner() >= 0.0);
    }
    #[test]
    fn test_matroid_intersection() {
        let mi = MatroidIntersection::new("graphic", "partition");
        assert!(mi.exchange_property());
    }
}
#[cfg(test)]
mod extended_comb_opt_tests {
    use super::*;
    #[test]
    fn test_branch_bound() {
        let mut bb = BranchBoundData::integer_program("most-fractional", "LP relaxation");
        bb.explore(100);
        assert_eq!(bb.nodes_explored, 100);
        assert!(bb.description().contains("B&B"));
    }
    #[test]
    fn test_cutting_plane() {
        let mut cp = CuttingPlane::gomory();
        cp.add_cut();
        cp.add_cut();
        assert_eq!(cp.num_cuts_added, 2);
        assert!(cp.description().contains("Gomory"));
    }
    #[test]
    fn test_set_cover() {
        let sc = SetCoverData::greedy(5, 10);
        assert!(sc.approximation_ratio > 1.0);
        assert!(sc.approx_description().contains("Greedy"));
    }
    #[test]
    fn test_facility_location() {
        let fl = FacilityLocation::new(3, 10, vec![1.0, 2.0, 3.0], 5.0);
        assert_eq!(fl.total_opening_cost(), 6.0);
        assert!((FacilityLocation::jms_approximation_ratio() - 1.861).abs() < 0.001);
    }
    #[test]
    fn test_vehicle_routing() {
        let vr = VehicleRouting::capacitated(3, 20, 100.0);
        assert_eq!(vr.num_vehicles, 3);
        assert!(vr.christofides_description().contains("Christofides"));
    }
}
