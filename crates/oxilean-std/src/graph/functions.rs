//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashSet, VecDeque};

use super::types::{
    DiGraph, ExpanderChecker, GraphonSampler, SzemerédiRegularityLemma, TreewidthHeuristic,
    TuttePolynomialEval, UndirectedGraph,
};

#[allow(dead_code)]
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
#[allow(dead_code)]
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
#[allow(dead_code)]
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
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// SimpleGraph V = { adj : V → V → Prop // sym, irrefl }
pub fn simple_graph_ty() -> Expr {
    pi(BinderInfo::Default, "V", type0(), type0())
}
/// SimpleGraph.adj : ∀ {V}, SimpleGraph V → V → V → Prop
pub fn simple_graph_adj_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("SimpleGraph"), bvar(0)),
            arrow(bvar(0), arrow(bvar(1), prop())),
        ),
    )
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// SimpleGraph.degree : ∀ {V} [Fintype V] [DecidableEq V], SimpleGraph V → V → Nat
pub fn simple_graph_degree_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), arrow(bvar(0), nat_ty())),
    )
}
/// Digraph V = { adj : V → V → Bool }
pub fn digraph_ty() -> Expr {
    pi(BinderInfo::Default, "V", type0(), type0())
}
/// digraph_adj : ∀ {V}, Digraph V → V → V → Bool
pub fn digraph_adj_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("Digraph"), bvar(0)),
            arrow(bvar(0), arrow(bvar(1), bool_ty())),
        ),
    )
}
/// Walk G u v = path from u to v in G (list of adjacent vertices)
pub fn walk_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("SimpleGraph"), bvar(0)),
            arrow(bvar(0), arrow(bvar(1), type0())),
        ),
    )
}
/// SimpleGraph.Connected : ∀ {V}, SimpleGraph V → Prop
pub fn simple_graph_connected_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
    )
}
/// IsTree G ↔ Connected G ∧ acyclic G
pub fn is_tree_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
    )
}
/// chromatic_number : ∀ {V} [Fintype V], SimpleGraph V → Nat
pub fn chromatic_number_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), nat_ty()),
    )
}
/// IsEulerian : ∀ {V}, SimpleGraph V → Prop
/// A graph has an Eulerian circuit iff every vertex has even degree and graph is connected
pub fn is_eulerian_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
    )
}
/// IsHamiltonian : ∀ {V}, SimpleGraph V → Prop
/// A graph has a Hamiltonian cycle visiting each vertex exactly once
pub fn is_hamiltonian_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
    )
}
/// IsBipartite : ∀ {V}, SimpleGraph V → Prop
pub fn is_bipartite_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
    )
}
/// IsPlanar : ∀ {V}, SimpleGraph V → Prop
pub fn is_planar_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
    )
}
/// Matching G = Set of independent edges
pub fn matching_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), type0()),
    )
}
/// IsPerfectMatching : ∀ {V} [Fintype V], SimpleGraph V → Matching → Prop
pub fn is_perfect_matching_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("SimpleGraph"), bvar(0)),
            arrow(app2(cst("Matching"), bvar(0), bvar(0)), prop()),
        ),
    )
}
/// Four Color Theorem: χ(G) ≤ 4 for any planar graph G
pub fn four_color_theorem_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            arrow(
                app(cst("IsPlanar"), bvar(0)),
                app2(
                    cst("Nat.le"),
                    app(cst("chromatic_number"), bvar(1)),
                    Expr::Lit(oxilean_kernel::Literal::Nat(4)),
                ),
            ),
        ),
    )
}
/// Euler's formula: V - E + F = 2 for connected planar graphs
pub fn euler_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "v",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "e",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "f",
                nat_ty(),
                arrow(
                    app3(cst("IsConnectedPlanar"), bvar(2), bvar(1), bvar(0)),
                    app2(
                        app(cst("Eq"), nat_ty()),
                        app2(
                            cst("Nat.add"),
                            app2(cst("Nat.sub"), bvar(2), bvar(1)),
                            bvar(0),
                        ),
                        Expr::Lit(oxilean_kernel::Literal::Nat(2)),
                    ),
                ),
            ),
        ),
    )
}
/// Eulerian circuit theorem: G has Eulerian circuit ↔ G is connected ∧ all degrees even
pub fn eulerian_circuit_thm_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            app2(
                cst("Iff"),
                app(cst("IsEulerian"), bvar(0)),
                app2(
                    cst("And"),
                    app(cst("SimpleGraph.Connected"), bvar(1)),
                    app(cst("AllDegreesEven"), bvar(1)),
                ),
            ),
        ),
    )
}
/// Hall's theorem: bipartite graph G=(A∪B, E) has perfect matching ↔
/// for all S ⊆ A, |N(S)| ≥ |S|
pub fn halls_theorem_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            arrow(
                app(cst("IsBipartite"), bvar(0)),
                app2(
                    cst("Iff"),
                    app(cst("HasPerfectMatching"), bvar(1)),
                    app(cst("HallCondition"), bvar(1)),
                ),
            ),
        ),
    )
}
/// Kuratowski's theorem: G is planar ↔ G has no K₅ or K₃₃ subdivision
pub fn kuratowski_theorem_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            app2(
                cst("Iff"),
                app(cst("IsPlanar"), bvar(0)),
                app(cst("NoK5OrK33Subdivision"), bvar(0)),
            ),
        ),
    )
}
pub fn real_ty() -> Expr {
    cst("Real")
}
/// GraphMinor : ∀ {V W}, SimpleGraph V → SimpleGraph W → Prop
/// H is a minor of G if H can be obtained from a subgraph of G by contracting edges.
#[allow(dead_code)]
pub fn graph_minor_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        impl_pi(
            "W",
            type0(),
            arrow(
                app(cst("SimpleGraph"), bvar(1)),
                arrow(app(cst("SimpleGraph"), bvar(1)), prop()),
            ),
        ),
    )
}
/// RobertsonSeymourWQO : ∀ {V} (seq : Nat → SimpleGraph V), ∃ i j, i < j ∧ Minor (seq i) (seq j)
/// The Robertson-Seymour theorem: graphs are well-quasi-ordered under the minor relation.
#[allow(dead_code)]
pub fn robertson_seymour_wqo_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "seq",
            arrow(nat_ty(), app(cst("SimpleGraph"), bvar(0))),
            app(cst("WellQuasiOrdered"), app(cst("GraphMinor"), bvar(0))),
        ),
    )
}
/// ExcludedMinorCharacterization : ∀ {V}, SimpleGraph V → Type
/// A graph class is characterized by a finite set of forbidden minors.
#[allow(dead_code)]
pub fn excluded_minor_characterization_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
    )
}
/// VertexExpansion : ∀ {V}, SimpleGraph V → Real → Prop
/// h-vertex expander: for all S with |S| ≤ |V|/2, |N(S)| ≥ h·|S|.
#[allow(dead_code)]
pub fn vertex_expansion_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), arrow(real_ty(), prop())),
    )
}
/// EdgeExpansion : ∀ {V}, SimpleGraph V → Real → Prop
/// Cheeger constant h(G) = min over S of |∂S|/min(|S|,|V\S|).
#[allow(dead_code)]
pub fn edge_expansion_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), arrow(real_ty(), prop())),
    )
}
/// CheegerConstant : ∀ {V} [Fintype V], SimpleGraph V → Real
/// The Cheeger constant (edge expansion ratio) of a graph.
#[allow(dead_code)]
pub fn cheeger_constant_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), real_ty()),
    )
}
/// CheegerInequality : ∀ {V} [Fintype V] (G : SimpleGraph V),
///   λ₂/2 ≤ h(G) ≤ √(2·d·λ₂)
/// The Cheeger inequality connecting spectral gap λ₂ to edge expansion h(G).
#[allow(dead_code)]
pub fn cheeger_inequality_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            app2(
                cst("CheegerBounds"),
                app(cst("SpectralGap"), bvar(0)),
                app(cst("CheegerConstant"), bvar(0)),
            ),
        ),
    )
}
/// SpectralGap : ∀ {V} [Fintype V], SimpleGraph V → Real
/// The spectral gap λ₂ = second smallest eigenvalue of the normalized Laplacian.
#[allow(dead_code)]
pub fn spectral_gap_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), real_ty()),
    )
}
/// ErdosRenyiGraph : Nat → Real → SimpleGraph
/// The Erdős-Rényi G(n,p) random graph model.
#[allow(dead_code)]
pub fn erdos_renyi_graph_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(real_ty(), app(cst("SimpleGraph"), nat_ty())),
    )
}
/// ConnectivityThreshold : Nat → Real
/// The threshold probability p_c = ln(n)/n for G(n,p) connectivity.
#[allow(dead_code)]
pub fn connectivity_threshold_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// PhaseTransition : ∀ (n : Nat) (p : Real), p > 1/n → GiantComponentExists (ErdosRenyi n p)
/// Phase transition at p = 1/n: a giant component emerges.
#[allow(dead_code)]
pub fn phase_transition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "p",
            real_ty(),
            arrow(
                app2(cst("Real.gt"), bvar(0), app(cst("Real.inv"), bvar(1))),
                app(
                    cst("GiantComponentExists"),
                    app2(cst("ErdosRenyiGraph"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Graphon : Type
/// A graphon is a symmetric measurable function W : [0,1]² → [0,1].
#[allow(dead_code)]
pub fn graphon_ty() -> Expr {
    type0()
}
/// GraphonCutDistance : Graphon → Graphon → Real
/// The cut distance between two graphons.
#[allow(dead_code)]
pub fn graphon_cut_distance_ty() -> Expr {
    arrow(cst("Graphon"), arrow(cst("Graphon"), real_ty()))
}
/// SzemerediRegularity : ∀ {V} [Fintype V] (G : SimpleGraph V) (ε : Real),
///   ∃ partition, ε-regular and |partition| ≤ M(ε)
/// Szemerédi regularity lemma: every graph has an ε-regular partition.
#[allow(dead_code)]
pub fn szemeredi_regularity_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            pi(
                BinderInfo::Default,
                "eps",
                real_ty(),
                app2(
                    cst("Exists"),
                    cst("Partition"),
                    app2(cst("IsEpsRegular"), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// IsPerfectGraph : ∀ {V}, SimpleGraph V → Prop
/// A graph is perfect if χ(H) = ω(H) for every induced subgraph H.
#[allow(dead_code)]
pub fn is_perfect_graph_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
    )
}
/// StrongPerfectGraphThm : ∀ {V} (G : SimpleGraph V),
///   IsPerfect G ↔ (NoOddHole G ∧ NoOddAntihole G)
/// The strong perfect graph theorem (Chudnovsky-Robertson-Seymour-Thomas 2006).
#[allow(dead_code)]
pub fn strong_perfect_graph_thm_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            app2(
                cst("Iff"),
                app(cst("IsPerfectGraph"), bvar(0)),
                app2(
                    cst("And"),
                    app(cst("NoOddHole"), bvar(0)),
                    app(cst("NoOddAntihole"), bvar(0)),
                ),
            ),
        ),
    )
}
/// CliqueCoverNumber : ∀ {V} [Fintype V], SimpleGraph V → Nat
/// The minimum number of cliques needed to cover all vertices.
#[allow(dead_code)]
pub fn clique_cover_number_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), nat_ty()),
    )
}
/// VertexConnectivity : ∀ {V} [Fintype V], SimpleGraph V → Nat
/// κ(G) = minimum number of vertices whose removal disconnects G.
#[allow(dead_code)]
pub fn vertex_connectivity_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), nat_ty()),
    )
}
/// EdgeConnectivity : ∀ {V} [Fintype V], SimpleGraph V → Nat
/// λ(G) = minimum number of edges whose removal disconnects G.
#[allow(dead_code)]
pub fn edge_connectivity_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), nat_ty()),
    )
}
/// MengersThm : ∀ {V} [Fintype V] (G : SimpleGraph V) (s t : V),
///   MaxDisjointPaths G s t = MinVertexCut G s t
/// Menger's theorem: max number of vertex-disjoint s-t paths = min vertex cut size.
#[allow(dead_code)]
pub fn mengers_theorem_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            pi(
                BinderInfo::Default,
                "s",
                bvar(1),
                pi(
                    BinderInfo::Default,
                    "t",
                    bvar(2),
                    app2(
                        app(cst("Eq"), nat_ty()),
                        app3(cst("MaxDisjointPaths"), bvar(2), bvar(1), bvar(0)),
                        app3(cst("MinVertexCut"), bvar(2), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// MaxFlowMinCut : ∀ (n : Nat) (s t : Nat) (cap : Nat → Nat → Real),
///   MaxFlow n s t cap = MinCutCapacity n s t cap
/// Max-flow min-cut theorem for real-valued capacities.
#[allow(dead_code)]
pub fn max_flow_min_cut_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "s",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "t",
                nat_ty(),
                pi(
                    BinderInfo::Default,
                    "cap",
                    arrow(nat_ty(), arrow(nat_ty(), real_ty())),
                    app2(
                        app(cst("Eq"), real_ty()),
                        app3(app(cst("MaxFlowVal"), bvar(3)), bvar(2), bvar(1), bvar(0)),
                        app3(
                            app(cst("MinCutCapacity"), bvar(3)),
                            bvar(2),
                            bvar(1),
                            bvar(0),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Genus : ∀ {V} [Fintype V], SimpleGraph V → Nat
/// The genus of a graph: minimum genus of orientable surface for embedding.
#[allow(dead_code)]
pub fn genus_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), nat_ty()),
    )
}
/// CrossingNumber : ∀ {V} [Fintype V], SimpleGraph V → Nat
/// The crossing number cr(G): minimum crossings in any planar drawing.
#[allow(dead_code)]
pub fn crossing_number_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), nat_ty()),
    )
}
/// WagnerThm : ∀ {V} (G : SimpleGraph V),
///   IsPlanar G ↔ NoK5Minor G ∧ NoK33Minor G
/// Wagner's theorem: G is planar iff G has no K₅ or K₃,₃ minor.
#[allow(dead_code)]
pub fn wagner_theorem_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            app2(
                cst("Iff"),
                app(cst("IsPlanar"), bvar(0)),
                app2(
                    cst("And"),
                    app(cst("NoK5Minor"), bvar(0)),
                    app(cst("NoK33Minor"), bvar(0)),
                ),
            ),
        ),
    )
}
/// Hypergraph : Type → Type
/// A hypergraph on vertex type V: a collection of hyperedges (subsets of V).
#[allow(dead_code)]
pub fn hypergraph_ty() -> Expr {
    pi(BinderInfo::Default, "V", type0(), type0())
}
/// HypergraphColoring : ∀ {V}, Hypergraph V → Nat → Prop
/// A proper k-coloring of a hypergraph: each hyperedge has at least 2 colors.
#[allow(dead_code)]
pub fn hypergraph_coloring_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("Hypergraph"), bvar(0)), arrow(nat_ty(), prop())),
    )
}
/// SunflowerLemma : ∀ (p k : Nat) (F : Finset (Finset Nat)),
///   |F| > (p-1)^k · k! → ∃ sunflower of size p in F
/// Erdős-Ko-Rado sunflower lemma for set families.
#[allow(dead_code)]
pub fn sunflower_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "p",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "F",
                app(cst("Finset"), app(cst("Finset"), nat_ty())),
                arrow(
                    app2(
                        cst("SunflowerSizeCondition"),
                        bvar(2),
                        app(cst("Finset.card"), bvar(0)),
                    ),
                    app2(cst("HasSunflower"), bvar(0), bvar(2)),
                ),
            ),
        ),
    )
}
/// HypergraphTuran : ∀ (r k n : Nat), TuranDensity r k n
/// Turán-type density results for hypergraphs.
#[allow(dead_code)]
pub fn hypergraph_turan_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "r",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "k",
            nat_ty(),
            pi(
                BinderInfo::Default,
                "n",
                nat_ty(),
                app3(cst("TuranDensity"), bvar(2), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// GraphHomomorphism : ∀ {V W}, SimpleGraph V → SimpleGraph W → Type
/// A graph homomorphism f : V → W preserving adjacency.
#[allow(dead_code)]
pub fn graph_homomorphism_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        impl_pi(
            "W",
            type0(),
            arrow(
                app(cst("SimpleGraph"), bvar(1)),
                arrow(app(cst("SimpleGraph"), bvar(1)), type0()),
            ),
        ),
    )
}
/// LovaszThetaFunction : ∀ {V} [Fintype V], SimpleGraph V → Real
/// The Lovász theta function ϑ(G), sandwiched between ω(G) and χ(G̅).
#[allow(dead_code)]
pub fn lovasz_theta_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), real_ty()),
    )
}
/// FractionalChromaticNumber : ∀ {V} [Fintype V], SimpleGraph V → Real
/// χ_f(G) = inf { k/d : G has a (k:d)-coloring }.
#[allow(dead_code)]
pub fn fractional_chromatic_number_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), real_ty()),
    )
}
/// HomomorphismDuality : ∀ {V W} (H : SimpleGraph V) (G : SimpleGraph W),
///   ¬ (Hom H G) ↔ ∃ (D : Digraph), DualObstruction D G
/// Homomorphism duality: absence of H-homomorphism characterized by dual.
#[allow(dead_code)]
pub fn homomorphism_duality_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        impl_pi(
            "W",
            type0(),
            pi(
                BinderInfo::Default,
                "H",
                app(cst("SimpleGraph"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "G",
                    app(cst("SimpleGraph"), bvar(1)),
                    app2(
                        cst("Iff"),
                        app(cst("Not"), app2(cst("GraphHom"), bvar(1), bvar(0))),
                        app2(cst("HasDualObstruction"), cst("Digraph"), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// TreeDecomposition : ∀ {V}, SimpleGraph V → Type
/// A tree decomposition of G: a tree T with bags B(t) ⊆ V covering edges.
#[allow(dead_code)]
pub fn tree_decomposition_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), type0()),
    )
}
/// Treewidth : ∀ {V} [Fintype V], SimpleGraph V → Nat
/// tw(G) = min over tree decompositions of (max bag size - 1).
#[allow(dead_code)]
pub fn treewidth_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), nat_ty()),
    )
}
/// Pathwidth : ∀ {V} [Fintype V], SimpleGraph V → Nat
/// pw(G) = min over path decompositions of (max bag size - 1).
#[allow(dead_code)]
pub fn pathwidth_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), nat_ty()),
    )
}
/// TreewidthDuality : ∀ {V} [Fintype V] (G : SimpleGraph V) (k : Nat),
///   Treewidth G ≤ k ↔ ¬ HasBranchDecompositionObstruction G k
/// Treewidth duality via brambles/haven obstructions.
#[allow(dead_code)]
pub fn treewidth_duality_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            pi(
                BinderInfo::Default,
                "k",
                nat_ty(),
                app2(
                    cst("Iff"),
                    app2(cst("Nat.le"), app(cst("Treewidth"), bvar(1)), bvar(0)),
                    app(
                        cst("Not"),
                        app2(cst("HasBranchDecompositionObstruction"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// AdjacencyMatrix : ∀ {V} [Fintype V], SimpleGraph V → Matrix V V Real
/// The adjacency matrix of a graph.
#[allow(dead_code)]
pub fn adjacency_matrix_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("SimpleGraph"), bvar(0)),
            app3(cst("Matrix"), bvar(0), bvar(0), real_ty()),
        ),
    )
}
/// GraphLaplacian : ∀ {V} [Fintype V], SimpleGraph V → Matrix V V Real
/// L = D - A where D is degree matrix, A is adjacency matrix.
#[allow(dead_code)]
pub fn graph_laplacian_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("SimpleGraph"), bvar(0)),
            app3(cst("Matrix"), bvar(0), bvar(0), real_ty()),
        ),
    )
}
/// EigenvalueInterlacing : ∀ {V} [Fintype V] (G H : SimpleGraph V),
///   IsInducedSubgraph H G → InterlacesEigenvalues (Laplacian G) (Laplacian H)
/// Cauchy's eigenvalue interlacing theorem for graph Laplacians.
#[allow(dead_code)]
pub fn eigenvalue_interlacing_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            pi(
                BinderInfo::Default,
                "H",
                app(cst("SimpleGraph"), bvar(1)),
                arrow(
                    app2(cst("IsInducedSubgraph"), bvar(0), bvar(1)),
                    app2(
                        cst("InterlacesEigenvalues"),
                        app(cst("GraphLaplacian"), bvar(1)),
                        app(cst("GraphLaplacian"), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// IharaZetaFunction : ∀ {V} [Fintype V], SimpleGraph V → (Complex → Complex)
/// The Ihara zeta function Z_G(u) = ∏_p (1 - u^{|p|})^{-1} over prime cycles p.
#[allow(dead_code)]
pub fn ihara_zeta_function_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("SimpleGraph"), bvar(0)),
            arrow(cst("Complex"), cst("Complex")),
        ),
    )
}
/// ChromaticPolynomial : ∀ {V} [Fintype V], SimpleGraph V → (Nat → Int)
/// P(G, k) = number of proper k-colorings of G.
#[allow(dead_code)]
pub fn chromatic_polynomial_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("SimpleGraph"), bvar(0)),
            arrow(nat_ty(), cst("Int")),
        ),
    )
}
/// TuttePolynomial : ∀ {V} [Fintype V], SimpleGraph V → (Real → Real → Real)
/// T(G; x, y) = universal graph polynomial encoding many graph invariants.
#[allow(dead_code)]
pub fn tutte_polynomial_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("SimpleGraph"), bvar(0)),
            arrow(real_ty(), arrow(real_ty(), real_ty())),
        ),
    )
}
/// MatchingPolynomial : ∀ {V} [Fintype V], SimpleGraph V → (Real → Real)
/// μ(G, x) = ∑_k (-1)^k m_k(G) x^{n-2k} where m_k = number of k-matchings.
#[allow(dead_code)]
pub fn matching_polynomial_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(
            app(cst("SimpleGraph"), bvar(0)),
            arrow(real_ty(), real_ty()),
        ),
    )
}
/// DeletionContractionTutte : ∀ {V} [Fintype V] (G : SimpleGraph V) (e : Edge V),
///   TuttePolynomial G = TuttePolynomial (Delete G e) + TuttePolynomial (Contract G e)
/// Deletion-contraction recurrence for the Tutte polynomial.
#[allow(dead_code)]
pub fn deletion_contraction_tutte_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            pi(
                BinderInfo::Default,
                "e",
                app(cst("Edge"), bvar(1)),
                app2(
                    app(cst("Eq"), arrow(real_ty(), arrow(real_ty(), real_ty()))),
                    app(cst("TuttePolynomial"), bvar(1)),
                    app2(
                        cst("TutteSum"),
                        app(
                            cst("TuttePolynomial"),
                            app2(cst("Delete"), bvar(1), bvar(0)),
                        ),
                        app(
                            cst("TuttePolynomial"),
                            app2(cst("Contract"), bvar(1), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// CayleyGraph : ∀ (G : Group) (S : Finset G), SimpleGraph G
/// The Cayley graph Cay(G, S) with vertex set G and edges from S-generating set.
#[allow(dead_code)]
pub fn cayley_graph_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "S",
            app(cst("Finset"), bvar(0)),
            app(cst("SimpleGraph"), bvar(1)),
        ),
    )
}
/// StronglyRegularGraph : ∀ {V} [Fintype V] (G : SimpleGraph V) (k λ μ : Nat), Prop
/// srg(n, k, λ, μ): k-regular, adjacent pairs have λ common neighbors,
/// non-adjacent pairs have μ common neighbors.
#[allow(dead_code)]
pub fn strongly_regular_graph_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        pi(
            BinderInfo::Default,
            "G",
            app(cst("SimpleGraph"), bvar(0)),
            pi(
                BinderInfo::Default,
                "k",
                nat_ty(),
                pi(
                    BinderInfo::Default,
                    "lam",
                    nat_ty(),
                    pi(BinderInfo::Default, "mu", nat_ty(), prop()),
                ),
            ),
        ),
    )
}
/// DynamicGraph : Type → Type
/// A dynamic graph supporting incremental/decremental edge updates.
#[allow(dead_code)]
pub fn dynamic_graph_ty() -> Expr {
    pi(BinderInfo::Default, "V", type0(), type0())
}
/// FullyDynamicConnectivity : ∀ {V} [Fintype V], DynamicGraph V → Prop
/// A data structure supporting edge insertions, deletions, and connectivity queries.
#[allow(dead_code)]
pub fn fully_dynamic_connectivity_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("DynamicGraph"), bvar(0)), prop()),
    )
}
/// IncrementalConnectivity : ∀ {V} [Fintype V], DynamicGraph V → Prop
/// An online connectivity structure supporting only edge insertions.
#[allow(dead_code)]
pub fn incremental_connectivity_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("DynamicGraph"), bvar(0)), prop()),
    )
}
/// SemiStreamingSketch : ∀ {V} [Fintype V], SimpleGraph V → Type
/// A semi-streaming sketch using O(n · polylog n) space for n = |V|.
#[allow(dead_code)]
pub fn semi_streaming_sketch_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SimpleGraph"), bvar(0)), type0()),
    )
}
/// SketchedConnectivity : ∀ {V} [Fintype V], SemiStreamingSketch → Prop
/// Determine connectivity from a semi-streaming sketch.
#[allow(dead_code)]
pub fn sketched_connectivity_ty() -> Expr {
    impl_pi(
        "V",
        type0(),
        arrow(app(cst("SemiStreamingSketch"), bvar(0)), prop()),
    )
}
/// Build the graph theory environment, registering all axioms and theorems.
#[allow(dead_code)]
pub fn build_graph_theory_env(env: &mut Environment) -> Result<(), String> {
    let _ = env.add(Declaration::Axiom {
        name: Name::str("SimpleGraph"),
        univ_params: vec![],
        ty: simple_graph_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("SimpleGraph.adj"),
        univ_params: vec![],
        ty: simple_graph_adj_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("SimpleGraph.degree"),
        univ_params: vec![],
        ty: simple_graph_degree_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Digraph"),
        univ_params: vec![],
        ty: digraph_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Digraph.adj"),
        univ_params: vec![],
        ty: digraph_adj_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Walk"),
        univ_params: vec![],
        ty: walk_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("SimpleGraph.Connected"),
        univ_params: vec![],
        ty: simple_graph_connected_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsTree"),
        univ_params: vec![],
        ty: is_tree_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("chromatic_number"),
        univ_params: vec![],
        ty: chromatic_number_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsEulerian"),
        univ_params: vec![],
        ty: is_eulerian_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsHamiltonian"),
        univ_params: vec![],
        ty: is_hamiltonian_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsBipartite"),
        univ_params: vec![],
        ty: is_bipartite_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsPlanar"),
        univ_params: vec![],
        ty: is_planar_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("four_color_theorem"),
        univ_params: vec![],
        ty: four_color_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("eulerian_circuit_theorem"),
        univ_params: vec![],
        ty: eulerian_circuit_thm_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("halls_theorem"),
        univ_params: vec![],
        ty: halls_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("kuratowski_theorem"),
        univ_params: vec![],
        ty: kuratowski_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("GraphMinor"),
        univ_params: vec![],
        ty: graph_minor_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("robertson_seymour_wqo"),
        univ_params: vec![],
        ty: robertson_seymour_wqo_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("VertexExpansion"),
        univ_params: vec![],
        ty: vertex_expansion_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("EdgeExpansion"),
        univ_params: vec![],
        ty: edge_expansion_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CheegerConstant"),
        univ_params: vec![],
        ty: cheeger_constant_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("SpectralGap"),
        univ_params: vec![],
        ty: spectral_gap_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Graphon"),
        univ_params: vec![],
        ty: graphon_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("GraphonCutDistance"),
        univ_params: vec![],
        ty: graphon_cut_distance_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IsPerfectGraph"),
        univ_params: vec![],
        ty: is_perfect_graph_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("strong_perfect_graph_theorem"),
        univ_params: vec![],
        ty: strong_perfect_graph_thm_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("VertexConnectivity"),
        univ_params: vec![],
        ty: vertex_connectivity_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("EdgeConnectivity"),
        univ_params: vec![],
        ty: edge_connectivity_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Genus"),
        univ_params: vec![],
        ty: genus_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CrossingNumber"),
        univ_params: vec![],
        ty: crossing_number_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Hypergraph"),
        univ_params: vec![],
        ty: hypergraph_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("HypergraphColoring"),
        univ_params: vec![],
        ty: hypergraph_coloring_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("GraphHomomorphism"),
        univ_params: vec![],
        ty: graph_homomorphism_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("LovaszTheta"),
        univ_params: vec![],
        ty: lovasz_theta_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FractionalChromaticNumber"),
        univ_params: vec![],
        ty: fractional_chromatic_number_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("TreeDecomposition"),
        univ_params: vec![],
        ty: tree_decomposition_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Treewidth"),
        univ_params: vec![],
        ty: treewidth_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Pathwidth"),
        univ_params: vec![],
        ty: pathwidth_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("AdjacencyMatrix"),
        univ_params: vec![],
        ty: adjacency_matrix_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("GraphLaplacian"),
        univ_params: vec![],
        ty: graph_laplacian_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("IharaZetaFunction"),
        univ_params: vec![],
        ty: ihara_zeta_function_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ChromaticPolynomial"),
        univ_params: vec![],
        ty: chromatic_polynomial_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("TuttePolynomial"),
        univ_params: vec![],
        ty: tutte_polynomial_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("MatchingPolynomial"),
        univ_params: vec![],
        ty: matching_polynomial_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CayleyGraph"),
        univ_params: vec![],
        ty: cayley_graph_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("StronglyRegularGraph"),
        univ_params: vec![],
        ty: strongly_regular_graph_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("DynamicGraph"),
        univ_params: vec![],
        ty: dynamic_graph_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("FullyDynamicConnectivity"),
        univ_params: vec![],
        ty: fully_dynamic_connectivity_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("SemiStreamingSketch"),
        univ_params: vec![],
        ty: semi_streaming_sketch_ty(),
    });
    Ok(())
}
/// Minimum spanning tree via Kruskal's algorithm (weighted undirected graph).
pub fn kruskal_mst(n: usize, mut edges: Vec<(usize, usize, i64)>) -> Vec<(usize, usize, i64)> {
    edges.sort_by_key(|&(_, _, w)| w);
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0usize; n];
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) -> bool {
        let rx = find(parent, x);
        let ry = find(parent, y);
        if rx == ry {
            return false;
        }
        if rank[rx] < rank[ry] {
            parent[rx] = ry;
        } else if rank[rx] > rank[ry] {
            parent[ry] = rx;
        } else {
            parent[ry] = rx;
            rank[rx] += 1;
        }
        true
    }
    let mut mst = Vec::new();
    for (u, v, w) in edges {
        if union(&mut parent, &mut rank, u, v) {
            mst.push((u, v, w));
            if mst.len() == n - 1 {
                break;
            }
        }
    }
    mst
}
/// Max-flow via Ford-Fulkerson with BFS (Edmonds-Karp).
pub fn max_flow(n: usize, source: usize, sink: usize, capacity: &[Vec<i64>]) -> i64 {
    let mut flow_matrix = vec![vec![0i64; n]; n];
    let mut total_flow = 0i64;
    loop {
        let mut parent = vec![usize::MAX; n];
        parent[source] = source;
        let mut queue = VecDeque::new();
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            if u == sink {
                break;
            }
            for v in 0..n {
                let residual = capacity[u][v] - flow_matrix[u][v];
                if parent[v] == usize::MAX && residual > 0 {
                    parent[v] = u;
                    queue.push_back(v);
                }
            }
        }
        if parent[sink] == usize::MAX {
            break;
        }
        let mut bottleneck = i64::MAX;
        let mut v = sink;
        while v != source {
            let u = parent[v];
            bottleneck = bottleneck.min(capacity[u][v] - flow_matrix[u][v]);
            v = u;
        }
        v = sink;
        while v != source {
            let u = parent[v];
            flow_matrix[u][v] += bottleneck;
            flow_matrix[v][u] -= bottleneck;
            v = u;
        }
        total_flow += bottleneck;
    }
    total_flow
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs_connected() {
        let mut g = UndirectedGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(g.is_connected());
        let dist = g.bfs(0);
        assert_eq!(dist, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_bfs_disconnected() {
        let mut g = UndirectedGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(2, 3);
        assert!(!g.is_connected());
        assert_eq!(g.num_components(), 2);
    }
    #[test]
    fn test_bipartite() {
        let k33 = UndirectedGraph::complete_bipartite(3, 3);
        assert!(k33.is_bipartite());
        let k3 = UndirectedGraph::complete(3);
        assert!(!k3.is_bipartite());
    }
    #[test]
    fn test_eulerian_even_cycle() {
        let c4 = UndirectedGraph::cycle(4);
        assert!(c4.all_degrees_even());
        assert!(c4.has_eulerian_circuit());
    }
    #[test]
    fn test_eulerian_odd_cycle_fails() {
        let p4 = UndirectedGraph::path(4);
        assert!(!p4.all_degrees_even());
        assert!(!p4.has_eulerian_circuit());
    }
    #[test]
    fn test_greedy_coloring_path() {
        let p4 = UndirectedGraph::path(4);
        let (k, coloring) = p4.greedy_coloring();
        assert!(k <= 2);
        assert!(p4.is_proper_coloring(&coloring));
    }
    #[test]
    fn test_greedy_coloring_complete() {
        let k4 = UndirectedGraph::complete(4);
        let (k, coloring) = k4.greedy_coloring();
        assert_eq!(k, 4);
        assert!(k4.is_proper_coloring(&coloring));
    }
    #[test]
    fn test_spanning_tree() {
        let k4 = UndirectedGraph::complete(4);
        let tree = k4.spanning_tree();
        assert_eq!(tree.len(), 3);
    }
    #[test]
    fn test_digraph_bfs() {
        let mut g = DiGraph::new(5);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 3, 1);
        g.add_edge(3, 4, 1);
        let dist = g.bfs(0);
        assert_eq!(dist, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_topo_sort_dag() {
        let mut g = DiGraph::new(4);
        g.add_edge(0, 1, 0);
        g.add_edge(0, 2, 0);
        g.add_edge(1, 3, 0);
        g.add_edge(2, 3, 0);
        let order = g.topo_sort().expect("topo_sort should succeed");
        assert_eq!(order.len(), 4);
    }
    #[test]
    fn test_topo_sort_cycle() {
        let mut g = DiGraph::new(3);
        g.add_edge(0, 1, 0);
        g.add_edge(1, 2, 0);
        g.add_edge(2, 0, 0);
        assert!(g.topo_sort().is_none());
    }
    #[test]
    fn test_scc() {
        let mut g = DiGraph::new(4);
        g.add_edge(0, 1, 0);
        g.add_edge(1, 0, 0);
        g.add_edge(2, 3, 0);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 3);
    }
    #[test]
    fn test_dijkstra() {
        let mut g = DiGraph::new(4);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 2);
        g.add_edge(1, 3, 5);
        g.add_edge(2, 3, 1);
        let (dist, _) = g.dijkstra(0);
        assert_eq!(dist[3], 4);
    }
    #[test]
    fn test_bellman_ford() {
        let mut g = DiGraph::new(3);
        g.add_edge(0, 1, -1);
        g.add_edge(1, 2, 3);
        let dist = g.bellman_ford(0).expect("bellman_ford should succeed");
        assert_eq!(dist[2], 2);
    }
    #[test]
    fn test_floyd_warshall() {
        let mut g = DiGraph::new(3);
        g.add_edge(0, 1, 5);
        g.add_edge(1, 2, 3);
        g.add_edge(0, 2, 10);
        let d = g.floyd_warshall();
        assert_eq!(d[0][2], 8);
    }
    #[test]
    fn test_kruskal_mst() {
        let edges = vec![(0, 1, 1), (1, 2, 2), (0, 2, 10), (2, 3, 3)];
        let mst = kruskal_mst(4, edges);
        assert_eq!(mst.len(), 3);
        let total: i64 = mst.iter().map(|&(_, _, w)| w).sum();
        assert_eq!(total, 6);
    }
    #[test]
    fn test_max_flow() {
        let n = 4;
        let mut cap = vec![vec![0i64; n]; n];
        cap[0][1] = 3;
        cap[0][2] = 2;
        cap[1][3] = 2;
        cap[2][3] = 3;
        cap[1][2] = 1;
        let flow = max_flow(n, 0, 3, &cap);
        assert_eq!(flow, 5);
    }
    #[test]
    fn test_build_graph_theory_env() {
        let mut env = Environment::new();
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type0(),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("Nat.le"),
            univ_params: vec![],
            ty: arrow(nat_ty(), arrow(nat_ty(), prop())),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("AllDegreesEven"),
            univ_params: vec![],
            ty: impl_pi(
                "V",
                type0(),
                arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("HasPerfectMatching"),
            univ_params: vec![],
            ty: impl_pi(
                "V",
                type0(),
                arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("HallCondition"),
            univ_params: vec![],
            ty: impl_pi(
                "V",
                type0(),
                arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("NoK5OrK33Subdivision"),
            univ_params: vec![],
            ty: impl_pi(
                "V",
                type0(),
                arrow(app(cst("SimpleGraph"), bvar(0)), prop()),
            ),
        });
        let _ = env.add(Declaration::Axiom {
            name: Name::str("IsConnectedPlanar"),
            univ_params: vec![],
            ty: arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop()))),
        });
        let result = build_graph_theory_env(&mut env);
        assert!(result.is_ok());
    }
    #[test]
    fn test_expander_checker_complete_graph() {
        let k4 = UndirectedGraph::complete(4);
        let checker = ExpanderChecker::new(k4);
        let h = checker.approximate_cheeger();
        assert!(h >= 1.0, "K_4 should have good expansion: h = {}", h);
        assert!(checker.is_expander(1.0));
    }
    #[test]
    fn test_expander_checker_path_graph() {
        let p4 = UndirectedGraph::path(4);
        let checker = ExpanderChecker::new(p4);
        let h = checker.approximate_cheeger();
        assert!(
            h < 2.0,
            "Path graph should have limited expansion: h = {}",
            h
        );
    }
    #[test]
    fn test_graphon_sampler_constant() {
        let sampler = GraphonSampler::new(5, Box::new(|_x, _y| 0.8));
        let g = sampler.sample_deterministic();
        assert_eq!(g.edge_count(), 10);
    }
    #[test]
    fn test_graphon_sampler_zero() {
        let sampler = GraphonSampler::new(5, Box::new(|_x, _y| 0.3));
        let g = sampler.sample_deterministic();
        assert_eq!(g.edge_count(), 0);
    }
    #[test]
    fn test_graphon_sampler_threshold() {
        let n = 6usize;
        let sampler = GraphonSampler::new(
            n,
            Box::new(move |x, y| {
                let half = 0.5f64;
                if (x < half) != (y < half) {
                    0.9
                } else {
                    0.1
                }
            }),
        );
        let g = sampler.sample_at_threshold(0.5);
        assert!(g.edge_count() > 0);
        assert!(g.is_bipartite());
    }
    #[test]
    fn test_treewidth_heuristic_tree() {
        let p5 = UndirectedGraph::path(5);
        let mut h = TreewidthHeuristic::new(&p5);
        let (_order, tw) = h.run();
        assert_eq!(tw, 1, "Path graph has treewidth 1");
    }
    #[test]
    fn test_treewidth_heuristic_complete() {
        let k4 = UndirectedGraph::complete(4);
        let mut h = TreewidthHeuristic::new(&k4);
        let (_order, tw) = h.run();
        assert!(tw >= 3, "K_4 treewidth should be at least 3, got {}", tw);
    }
    #[test]
    fn test_tutte_polynomial_tree() {
        let p3 = UndirectedGraph::path(3);
        let eval = TuttePolynomialEval::from_graph(&p3);
        let val = eval.evaluate(2.0, 2.0);
        assert!((val - 4.0).abs() < 1e-9, "T(path3; 2,2) = {}", val);
    }
    #[test]
    fn test_tutte_polynomial_single_edge() {
        let mut g = UndirectedGraph::new(2);
        g.add_edge(0, 1);
        let eval = TuttePolynomialEval::from_graph(&g);
        let val = eval.evaluate(2.0, 2.0);
        assert!(
            (val - 2.0).abs() < 1e-9,
            "T(K2; 2,2) = {} (expected 2.0)",
            val
        );
    }
    #[test]
    fn test_szemeredi_regularity_complete() {
        let k5 = UndirectedGraph::complete(5);
        let rl = SzemerédiRegularityLemma::new(k5, 0.5);
        let (parts, regular_pairs) = rl.run(2);
        assert_eq!(parts.len(), 2);
        let _ = regular_pairs;
    }
    #[test]
    fn test_szemeredi_density_bipartite() {
        let k33 = UndirectedGraph::complete_bipartite(3, 3);
        let rl = SzemerédiRegularityLemma::new(k33, 0.5);
        let part_a: Vec<usize> = (0..3).collect();
        let part_b: Vec<usize> = (3..6).collect();
        let d = rl.density(&part_a, &part_b);
        assert!((d - 1.0).abs() < 1e-9, "K_3,3 density = {}", d);
        assert!(rl.is_regular_pair(&part_a, &part_b));
    }
    #[test]
    fn test_new_axiom_types_build() {
        let _ = graph_minor_ty();
        let _ = vertex_expansion_ty();
        let _ = edge_expansion_ty();
        let _ = cheeger_constant_ty();
        let _ = spectral_gap_ty();
        let _ = graphon_ty();
        let _ = graphon_cut_distance_ty();
        let _ = is_perfect_graph_ty();
        let _ = strong_perfect_graph_thm_ty();
        let _ = vertex_connectivity_ty();
        let _ = edge_connectivity_ty();
        let _ = genus_ty();
        let _ = crossing_number_ty();
        let _ = hypergraph_ty();
        let _ = hypergraph_coloring_ty();
        let _ = graph_homomorphism_ty();
        let _ = lovasz_theta_ty();
        let _ = fractional_chromatic_number_ty();
        let _ = tree_decomposition_ty();
        let _ = treewidth_ty();
        let _ = pathwidth_ty();
        let _ = adjacency_matrix_ty();
        let _ = graph_laplacian_ty();
        let _ = ihara_zeta_function_ty();
        let _ = chromatic_polynomial_ty();
        let _ = tutte_polynomial_ty();
        let _ = matching_polynomial_ty();
        let _ = cayley_graph_ty();
        let _ = strongly_regular_graph_ty();
        let _ = dynamic_graph_ty();
        let _ = fully_dynamic_connectivity_ty();
        let _ = semi_streaming_sketch_ty();
    }
}
