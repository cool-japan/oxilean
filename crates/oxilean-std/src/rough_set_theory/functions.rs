//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet};

use super::types::{
    AttributeReduct, BayesianRoughSet, CoveringSystem, DecisionSystemReducer, DecisionTable,
    DecisionTableExt, DiscernibilityMatrix, DiscernibilityMatrixExt, DominanceRoughSet, Granule,
    GranuleStructure, InformationSystem, MultiGranulationRoughSet, NeighborhoodSystem,
    RoughFuzzyMembership, RoughTruth, VPRSApproximation, VariablePrecisionApprox, VPRS,
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
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// Compute the discernibility matrix for a decision table.
/// disc[i][j] = set of attributes that distinguish objects i and j
/// w.r.t. the decision attribute.
pub fn discernibility_matrix(dt: &DecisionTable) -> Vec<Vec<HashSet<usize>>> {
    let n = dt.info.n_objects;
    let cond = dt.condition_attrs();
    let mut matrix = vec![vec![HashSet::new(); n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if dt.info.get(i, dt.decision_attr) != dt.info.get(j, dt.decision_attr) {
                let disc: HashSet<usize> = cond
                    .iter()
                    .copied()
                    .filter(|&a| dt.info.get(i, a) != dt.info.get(j, a))
                    .collect();
                matrix[i][j] = disc.clone();
                matrix[j][i] = disc;
            }
        }
    }
    matrix
}
/// Attribute significance: how much removing attribute a reduces the dependency degree.
pub fn attribute_significance(dt: &DecisionTable, attr: usize) -> f64 {
    let full_dep = dt.dependency_degree();
    let cond: Vec<usize> = dt
        .condition_attrs()
        .into_iter()
        .filter(|&a| a != attr)
        .collect();
    let reduced_dep = dt.info.quality_of_approximation(&cond, &[dt.decision_attr]);
    full_dep - reduced_dep
}
/// Approximation space type: ApproxSpace U = (U → U → Prop) → Prop.
pub fn approx_space_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(arrow(bvar(0), arrow(bvar(1), prop())), prop()),
    )
}
/// Rough set type: a pair (Lower, Upper) of subsets of U.
pub fn rough_set_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Set"), bvar(0)),
            arrow(app(cst("Set"), bvar(1)), prop()),
        ),
    )
}
/// Indiscernibility relation type: Indiscern B x y — objects x, y agree on all B-attributes.
pub fn indiscern_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B",
            app(cst("Set"), cst("Attr")),
            arrow(bvar(1), arrow(bvar(2), prop())),
        ),
    )
}
/// Lower approximation type: Lower B X = { x | [x]_B ⊆ X }.
pub fn lower_approx_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B",
            app(cst("Set"), cst("Attr")),
            arrow(app(cst("Set"), bvar(1)), app(cst("Set"), bvar(2))),
        ),
    )
}
/// Upper approximation type: Upper B X = { x | [x]_B ∩ X ≠ ∅ }.
pub fn upper_approx_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B",
            app(cst("Set"), cst("Attr")),
            arrow(app(cst("Set"), bvar(1)), app(cst("Set"), bvar(2))),
        ),
    )
}
/// Reduct type: Reduct B C D — B is a reduct of C relative to D.
pub fn reduct_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Set"), cst("Attr")),
            arrow(
                app(cst("Set"), cst("Attr")),
                arrow(app(cst("Set"), cst("Attr")), prop()),
            ),
        ),
    )
}
/// Core type: Core C D — the intersection of all reducts.
pub fn core_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Set"), cst("Attr")),
            arrow(app(cst("Set"), cst("Attr")), app(cst("Set"), cst("Attr"))),
        ),
    )
}
/// Dependency degree type: Dependency C D → Real.
pub fn dependency_ty() -> Expr {
    arrow(
        app(cst("Set"), cst("Attr")),
        arrow(app(cst("Set"), cst("Attr")), cst("Real")),
    )
}
/// Dominance relation type for DRSA.
pub fn dominance_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "C",
            app(cst("Set"), cst("Attr")),
            arrow(bvar(1), arrow(bvar(2), prop())),
        ),
    )
}
/// Neighborhood system type.
pub fn neighborhood_system_ty() -> Expr {
    impl_pi("U", type0(), arrow(bvar(0), app(cst("Set"), bvar(1))))
}
/// Variable precision rough set type.
pub fn vprs_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            cst("Real"),
            arrow(
                app(cst("Set"), cst("Attr")),
                arrow(app(cst("Set"), bvar(2)), app(cst("Set"), bvar(3))),
            ),
        ),
    )
}
/// Fundamental theorem of rough sets: Lower(X) ⊆ X ⊆ Upper(X).
pub fn sandwich_theorem_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B",
            app(cst("Set"), cst("Attr")),
            impl_pi(
                "X",
                app(cst("Set"), bvar(1)),
                app2(cst("Subset"), app2(cst("Lower"), bvar(2), bvar(0)), bvar(0)),
            ),
        ),
    )
}
/// The lower approximation is definable (union of equivalence classes).
pub fn lower_definable_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B",
            app(cst("Set"), cst("Attr")),
            impl_pi(
                "X",
                app(cst("Set"), bvar(1)),
                app2(
                    cst("IsDefinable"),
                    app2(cst("Lower"), bvar(2), bvar(0)),
                    bvar(2),
                ),
            ),
        ),
    )
}
/// Reduct existence theorem: every approximation space has at least one reduct.
pub fn reduct_existence_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "C",
            app(cst("Set"), cst("Attr")),
            impl_pi(
                "D",
                app(cst("Set"), cst("Attr")),
                app(cst("Exists"), app2(cst("Reduct"), bvar(1), bvar(0))),
            ),
        ),
    )
}
/// Core equals intersection of all reducts.
pub fn core_characterization_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "C",
            app(cst("Set"), cst("Attr")),
            impl_pi(
                "D",
                app(cst("Set"), cst("Attr")),
                app2(
                    cst("SetEq"),
                    app2(cst("Core"), bvar(1), bvar(0)),
                    app2(cst("IntersectionOfReducts"), bvar(2), bvar(1)),
                ),
            ),
        ),
    )
}
/// DRSA: lower approximation of upward union is a union of dominance classes.
pub fn drsa_lower_union_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "C",
            app(cst("Set"), cst("Attr")),
            arrow(
                app2(cst("DRSASpace"), bvar(1), bvar(0)),
                app(cst("IsDominanceUnion"), app(cst("DRSALower"), bvar(2))),
            ),
        ),
    )
}
/// Accuracy monotonicity: adding attributes can only increase accuracy.
pub fn accuracy_monotonicity_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B1",
            app(cst("Set"), cst("Attr")),
            impl_pi(
                "B2",
                app(cst("Set"), cst("Attr")),
                arrow(
                    app2(cst("Subset"), bvar(1), bvar(0)),
                    arrow(
                        app(cst("Set"), bvar(2)),
                        arrow(
                            app(cst("Set"), bvar(3)),
                            app2(
                                cst("Le"),
                                app2(cst("Accuracy"), bvar(4), cst("X")),
                                app2(cst("Accuracy"), bvar(3), cst("X")),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Granule decomposition: every set can be expressed in terms of granules.
pub fn granule_decomposition_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "G",
            app(cst("GranuleStructure"), bvar(0)),
            impl_pi(
                "X",
                app(cst("Set"), bvar(1)),
                app2(
                    cst("GranuleApprox"),
                    app2(cst("GranuleLower"), bvar(2), bvar(0)),
                    app2(cst("GranuleUpper"), bvar(2), bvar(0)),
                ),
            ),
        ),
    )
}
/// VPRS inclusion: VPRS_0 lower = standard lower, VPRS approaches union as l → 0.5.
pub fn vprs_monotonicity_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "l1",
            cst("Real"),
            impl_pi(
                "l2",
                cst("Real"),
                arrow(
                    app2(cst("Le"), bvar(1), bvar(0)),
                    arrow(
                        app(cst("Set"), bvar(2)),
                        app2(
                            cst("Subset"),
                            app3(cst("VPRSLower"), bvar(4), bvar(3), cst("X")),
                            app3(cst("VPRSLower"), bvar(4), bvar(2), cst("X")),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Decision system type: maps condition attrs × decision attrs → reduct set.
pub fn decision_system_ty() -> Expr {
    arrow(
        app(cst("Set"), cst("Attr")),
        arrow(
            app(cst("Set"), cst("Attr")),
            app(cst("Set"), app(cst("Set"), cst("Attr"))),
        ),
    )
}
/// Discernibility matrix type: DiscernMatrix U n → (n × n → Set Attr).
pub fn discernibility_matrix_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            cst("Nat"),
            arrow(
                arrow(cst("Nat"), arrow(cst("Nat"), app(cst("Set"), cst("Attr")))),
                prop(),
            ),
        ),
    )
}
/// Fuzzy lower approximation type: FuzzyLower ~ (U → Real) → (U → Real).
pub fn fuzzy_lower_approx_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(arrow(bvar(0), cst("Real")), arrow(bvar(1), cst("Real"))),
    )
}
/// Fuzzy upper approximation type: FuzzyUpper ~ (U → Real) → (U → Real).
pub fn fuzzy_upper_approx_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(arrow(bvar(0), cst("Real")), arrow(bvar(1), cst("Real"))),
    )
}
/// Covering-based lower approximation type.
pub fn covering_lower_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Covering"), bvar(0)),
            arrow(app(cst("Set"), bvar(1)), app(cst("Set"), bvar(2))),
        ),
    )
}
/// Covering-based upper approximation type.
pub fn covering_upper_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Covering"), bvar(0)),
            arrow(app(cst("Set"), bvar(1)), app(cst("Set"), bvar(2))),
        ),
    )
}
/// Bayesian rough set type with decision thresholds.
pub fn bayesian_rough_ty() -> Expr {
    arrow(
        cst("Real"),
        arrow(
            cst("Real"),
            arrow(
                app(cst("Set"), cst("Attr")),
                arrow(
                    app(cst("Set"), bvar(2)),
                    app3(
                        cst("Prod"),
                        app(cst("Set"), bvar(3)),
                        app(cst("Set"), bvar(4)),
                        app(cst("Set"), bvar(5)),
                    ),
                ),
            ),
        ),
    )
}
/// Probabilistic rough set type: conditional probability based.
pub fn probabilistic_rough_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Set"), cst("Attr")),
            arrow(app(cst("Set"), bvar(1)), arrow(bvar(2), cst("Real"))),
        ),
    )
}
/// Multi-granulation lower approximation (optimistic).
pub fn multi_granulation_lower_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("List"), app(cst("Set"), cst("Attr"))),
            arrow(app(cst("Set"), bvar(1)), app(cst("Set"), bvar(2))),
        ),
    )
}
/// Multi-granulation upper approximation (pessimistic).
pub fn multi_granulation_upper_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("List"), app(cst("Set"), cst("Attr"))),
            arrow(app(cst("Set"), bvar(1)), app(cst("Set"), bvar(2))),
        ),
    )
}
/// Partial inclusion (rough mereology): PartialInc A B r — A is partially included in B with degree r.
pub fn partial_inclusion_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Set"), bvar(0)),
            arrow(app(cst("Set"), bvar(1)), arrow(cst("Real"), prop())),
        ),
    )
}
/// Similarity rough set type: uses a similarity relation R.
pub fn similarity_rough_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            arrow(bvar(0), arrow(bvar(1), cst("Real"))),
            arrow(app(cst("Set"), bvar(2)), app(cst("Set"), bvar(3))),
        ),
    )
}
/// Matroidal structure induced by rough sets.
pub fn matroid_rough_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Set"), cst("Attr")),
            app(cst("Matroid"), app(cst("Set"), cst("Attr"))),
        ),
    )
}
/// Topology induced by rough sets: opens are definable sets.
pub fn rough_topology_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(app(cst("Set"), cst("Attr")), app(cst("Topology"), bvar(1))),
    )
}
/// Rough closure operator.
pub fn rough_closure_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Set"), cst("Attr")),
            arrow(app(cst("Set"), bvar(1)), app(cst("Set"), bvar(2))),
        ),
    )
}
/// S5 rough logic modality: □φ (certain) and ◇φ (possible).
pub fn rough_modality_ty() -> Expr {
    arrow(prop(), prop())
}
/// Rough truth assignment type.
pub fn rough_truth_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            app(cst("Set"), bvar(0)),
            arrow(
                arrow(app(cst("Set"), bvar(1)), app(cst("Set"), bvar(2))),
                app(cst("RoughTruthVal"), bvar(3)),
            ),
        ),
    )
}
/// Information granule type: a labeled subset.
pub fn information_granule_ty() -> Expr {
    impl_pi("U", type0(), arrow(cst("String"), app(cst("Set"), bvar(1))))
}
/// Covering lower ⊆ standard lower when covering is a partition.
pub fn covering_reduces_to_standard_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "C",
            app(cst("Covering"), bvar(0)),
            arrow(
                app(cst("IsPartition"), bvar(0)),
                impl_pi(
                    "X",
                    app(cst("Set"), bvar(2)),
                    app2(
                        cst("SetEq"),
                        app2(cst("CoveringLower"), bvar(3), bvar(0)),
                        app2(cst("Lower"), bvar(3), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// Bayesian positive region ⊆ standard lower approximation.
pub fn bayesian_positive_subset_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "alpha",
            cst("Real"),
            impl_pi(
                "B",
                app(cst("Set"), cst("Attr")),
                impl_pi(
                    "X",
                    app(cst("Set"), bvar(2)),
                    app2(
                        cst("Subset"),
                        app3(cst("BayesPos"), bvar(3), bvar(2), bvar(0)),
                        app2(cst("Lower"), bvar(3), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// Multi-granulation pessimistic lower ⊆ optimistic lower.
pub fn multi_gran_pessimistic_le_optimistic_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "Gs",
            app(cst("List"), app(cst("Set"), cst("Attr"))),
            impl_pi(
                "X",
                app(cst("Set"), bvar(1)),
                app2(
                    cst("Subset"),
                    app2(cst("PessimisticLower"), bvar(2), bvar(0)),
                    app2(cst("OptimisticLower"), bvar(2), bvar(0)),
                ),
            ),
        ),
    )
}
/// Fuzzy lower ≤ membership ≤ fuzzy upper (pointwise).
pub fn fuzzy_sandwich_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "mu",
            arrow(bvar(0), cst("Real")),
            impl_pi(
                "x",
                bvar(1),
                app2(
                    cst("Le"),
                    app(app(cst("FuzzyLower"), bvar(2)), bvar(0)),
                    app(bvar(2), bvar(0)),
                ),
            ),
        ),
    )
}
/// Partial inclusion reflexivity: A ⊆_1 A.
pub fn partial_inclusion_refl_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "A",
            app(cst("Set"), bvar(0)),
            app3(cst("PartialInc"), bvar(1), bvar(0), cst("one")),
        ),
    )
}
/// Matroid exchange property induced by rough sets.
pub fn matroid_exchange_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B",
            app(cst("Set"), cst("Attr")),
            app(cst("SatisfiesExchange"), app(cst("RoughMatroid"), bvar(0))),
        ),
    )
}
/// Rough topology satisfies Kuratowski axioms.
pub fn rough_topology_kuratowski_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B",
            app(cst("Set"), cst("Attr")),
            app(
                cst("IsTopology"),
                app2(cst("RoughTopology"), bvar(1), bvar(0)),
            ),
        ),
    )
}
/// Rough S5 modality: □φ → φ (T axiom).
pub fn rough_s5_t_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        prop(),
        arrow(app(cst("RoughNecessary"), bvar(0)), bvar(0)),
    )
}
/// Rough S5 modality: □φ → □□φ (4 axiom).
pub fn rough_s5_4_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        prop(),
        arrow(
            app(cst("RoughNecessary"), bvar(0)),
            app(cst("RoughNecessary"), app(cst("RoughNecessary"), bvar(0))),
        ),
    )
}
/// Rough S5 modality: ◇φ → □◇φ (5 axiom).
pub fn rough_s5_5_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        prop(),
        arrow(
            app(cst("RoughPossible"), bvar(0)),
            app(cst("RoughNecessary"), app(cst("RoughPossible"), bvar(0))),
        ),
    )
}
/// Discernibility-based reduct characterization.
pub fn reduct_discernibility_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B",
            app(cst("Set"), cst("Attr")),
            impl_pi(
                "D",
                app(cst("Set"), cst("Attr")),
                app2(
                    cst("Iff"),
                    app2(cst("IsReduct"), bvar(1), bvar(0)),
                    app2(
                        cst("IsHittingSet"),
                        bvar(2),
                        app2(cst("DiscMatrix"), bvar(1), bvar(0)),
                    ),
                ),
            ),
        ),
    )
}
/// Dependency monotonicity: B ⊆ C → γ(B,D) ≤ γ(C,D).
pub fn dependency_monotonicity_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        impl_pi(
            "B",
            app(cst("Set"), cst("Attr")),
            impl_pi(
                "C",
                app(cst("Set"), cst("Attr")),
                impl_pi(
                    "D",
                    app(cst("Set"), cst("Attr")),
                    arrow(
                        app2(cst("Subset"), bvar(2), bvar(1)),
                        app2(
                            cst("Le"),
                            app2(cst("Dependency"), bvar(3), bvar(0)),
                            app2(cst("Dependency"), bvar(2), bvar(0)),
                        ),
                    ),
                ),
            ),
        ),
    )
}
/// Neighborhood-based topology: N(x) defines a topology iff N is a neighborhood filter.
pub fn neighborhood_topology_ty() -> Expr {
    impl_pi(
        "U",
        type0(),
        arrow(
            arrow(bvar(0), app(cst("Set"), bvar(1))),
            arrow(
                app(cst("IsNeighborhoodFilter"), bvar(1)),
                app(
                    cst("IsTopology"),
                    app(cst("TopologyFromNeighborhood"), bvar(2)),
                ),
            ),
        ),
    )
}
/// Build the rough set theory kernel environment.
pub fn build_rough_set_env() -> Environment {
    let mut env = Environment::new();
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ApproxSpace"),
        univ_params: vec![],
        ty: approx_space_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("RoughSet"),
        univ_params: vec![],
        ty: rough_set_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Indiscernibility"),
        univ_params: vec![],
        ty: indiscern_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("LowerApproximation"),
        univ_params: vec![],
        ty: lower_approx_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("UpperApproximation"),
        univ_params: vec![],
        ty: upper_approx_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Reduct"),
        univ_params: vec![],
        ty: reduct_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("Core"),
        univ_params: vec![],
        ty: core_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("DependencyDegree"),
        univ_params: vec![],
        ty: dependency_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("DominanceRelation"),
        univ_params: vec![],
        ty: dominance_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("NeighborhoodSystem"),
        univ_params: vec![],
        ty: neighborhood_system_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("VPRSApprox"),
        univ_params: vec![],
        ty: vprs_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("SandwichTheorem"),
        univ_params: vec![],
        ty: sandwich_theorem_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("LowerDefinable"),
        univ_params: vec![],
        ty: lower_definable_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("ReductsExist"),
        univ_params: vec![],
        ty: reduct_existence_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("CoreCharacterization"),
        univ_params: vec![],
        ty: core_characterization_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("DRSALowerUnion"),
        univ_params: vec![],
        ty: drsa_lower_union_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("AccuracyMonotonicity"),
        univ_params: vec![],
        ty: accuracy_monotonicity_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("GranuleDecomposition"),
        univ_params: vec![],
        ty: granule_decomposition_ty(),
    });
    let _ = env.add(Declaration::Axiom {
        name: Name::str("VPRSMonotonicity"),
        univ_params: vec![],
        ty: vprs_monotonicity_ty(),
    });
    let extended: &[(&str, fn() -> Expr)] = &[
        ("DecisionSystem", decision_system_ty),
        ("DiscernibilityMatrix", discernibility_matrix_ty),
        ("FuzzyLowerApprox", fuzzy_lower_approx_ty),
        ("FuzzyUpperApprox", fuzzy_upper_approx_ty),
        ("CoveringLower", covering_lower_ty),
        ("CoveringUpper", covering_upper_ty),
        ("BayesianRough", bayesian_rough_ty),
        ("ProbabilisticRough", probabilistic_rough_ty),
        ("MultiGranLower", multi_granulation_lower_ty),
        ("MultiGranUpper", multi_granulation_upper_ty),
        ("PartialInclusion", partial_inclusion_ty),
        ("SimilarityRough", similarity_rough_ty),
        ("MatroidRough", matroid_rough_ty),
        ("RoughTopology", rough_topology_ty),
        ("RoughClosure", rough_closure_ty),
        ("RoughNecessary", rough_modality_ty),
        ("RoughPossible", rough_modality_ty),
        ("RoughTruth", rough_truth_ty),
        ("InformationGranule", information_granule_ty),
        ("CoveringReducesToStandard", covering_reduces_to_standard_ty),
        ("BayesianPositiveSubset", bayesian_positive_subset_ty),
        (
            "MultiGranPessLeOpt",
            multi_gran_pessimistic_le_optimistic_ty,
        ),
        ("FuzzySandwich", fuzzy_sandwich_ty),
        ("PartialInclusionRefl", partial_inclusion_refl_ty),
        ("MatroidExchange", matroid_exchange_ty),
        ("RoughTopologyKuratowski", rough_topology_kuratowski_ty),
        ("RoughS5TAxiom", rough_s5_t_axiom_ty),
        ("RoughS5FourAxiom", rough_s5_4_axiom_ty),
        ("RoughS5FiveAxiom", rough_s5_5_axiom_ty),
        ("ReductDiscernibility", reduct_discernibility_ty),
        ("DependencyMonotonicity", dependency_monotonicity_ty),
        ("NeighborhoodTopology", neighborhood_topology_ty),
    ];
    for (name, ty_fn) in extended {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        });
    }
    env
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_simple_info() -> InformationSystem {
        let mut info = InformationSystem::new(6, 4);
        let data = [
            [1u32, 2, 0, 1],
            [1, 1, 1, 1],
            [0, 2, 1, 0],
            [1, 2, 1, 1],
            [0, 1, 0, 0],
            [0, 2, 0, 0],
        ];
        for (i, row) in data.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                info.set(i, j, val);
            }
        }
        info
    }
    fn make_decision_table() -> DecisionTable {
        let info = make_simple_info();
        DecisionTable::new(info, 3)
    }
    #[test]
    fn test_indiscernibility_classes() {
        let info = make_simple_info();
        let classes = info.indiscernibility_classes(&[0, 1]);
        let total: usize = classes.iter().map(|c| c.len()).sum();
        assert_eq!(total, info.n_objects);
    }
    #[test]
    fn test_lower_upper_approximation() {
        let info = make_simple_info();
        let target: HashSet<usize> = [0, 1, 3].iter().copied().collect();
        let lower = info.lower_approximation(&[0, 1, 2], &target);
        let upper = info.upper_approximation(&[0, 1, 2], &target);
        assert!(lower.is_subset(&target));
        assert!(target.is_subset(&upper));
        assert!(lower.is_subset(&upper));
    }
    #[test]
    fn test_dependency_degree() {
        let dt = make_decision_table();
        let dep = dt.dependency_degree();
        assert!(dep >= 0.0 && dep <= 1.0);
    }
    #[test]
    fn test_reducts_and_core() {
        let mut info = InformationSystem::new(4, 3);
        let data = [[1u32, 0, 1], [1, 1, 1], [0, 0, 0], [0, 1, 0]];
        for (i, row) in data.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                info.set(i, j, val);
            }
        }
        let dt = DecisionTable::new(info, 2);
        let dep = dt.dependency_degree();
        let dep_a = dt.info.quality_of_approximation(&[0], &[2]);
        let dep_b = dt.info.quality_of_approximation(&[1], &[2]);
        assert!(dep_a >= 0.0 && dep_b >= 0.0);
        assert!(dep >= dep_a.max(dep_b) - 1e-9);
    }
    #[test]
    fn test_vprs_approximation() {
        let info = make_simple_info();
        let target: HashSet<usize> = [0, 1, 3].iter().copied().collect();
        let vprs = VPRSApproximation::new(0.2);
        let lower = vprs.lower_approximation(&info, &[0, 1, 2], &target);
        let standard_lower = info.lower_approximation(&[0, 1, 2], &target);
        assert!(standard_lower.is_subset(&lower));
    }
    #[test]
    fn test_neighborhood_system() {
        let mut ns = NeighborhoodSystem::new(4);
        for x in 0..4 {
            ns.add_neighbor(x, x);
        }
        ns.add_neighbor(0, 1);
        ns.add_neighbor(1, 0);
        ns.add_neighbor(2, 3);
        ns.add_neighbor(3, 2);
        assert!(ns.is_reflexive());
        assert!(ns.is_symmetric());
        let target: HashSet<usize> = [0, 1].iter().copied().collect();
        let lower = ns.lower_approximation(&target);
        assert!(lower.contains(&0));
        assert!(lower.contains(&1));
        assert!(!lower.contains(&2));
    }
    #[test]
    fn test_dominance_rough_set() {
        let mut info = InformationSystem::new(4, 2);
        let data = [[3u32, 1], [2, 2], [1, 1], [2, 3]];
        for (i, row) in data.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                info.set(i, j, val);
            }
        }
        let drs = DominanceRoughSet::new(info, vec![0], vec![1]);
        assert!(drs.dominates(0, 1));
        assert!(!drs.dominates(1, 0));
        let dc = drs.dominance_class_up(0);
        assert!(dc.contains(&0));
    }
    #[test]
    fn test_granule_structure() {
        let mut gs = GranuleStructure::new(6);
        let g1 = Granule::new("G1", [0, 1, 2].iter().copied().collect());
        let g2 = Granule::new("G2", [3, 4, 5].iter().copied().collect());
        gs.add_granule(g1);
        gs.add_granule(g2);
        assert!(gs.is_partition());
        let target: HashSet<usize> = [1, 2, 3].iter().copied().collect();
        let lower = gs.lower_approximation(&target);
        let upper = gs.upper_approximation(&target);
        assert!(lower.is_empty());
        assert_eq!(upper.len(), 6);
    }
    #[test]
    fn test_build_rough_set_env() {
        let env = build_rough_set_env();
        assert!(env.get(&Name::str("ApproxSpace")).is_some());
        assert!(env.get(&Name::str("RoughSet")).is_some());
        assert!(env.get(&Name::str("LowerApproximation")).is_some());
        assert!(env.get(&Name::str("UpperApproximation")).is_some());
        assert!(env.get(&Name::str("Reduct")).is_some());
        assert!(env.get(&Name::str("Core")).is_some());
        assert!(env.get(&Name::str("SandwichTheorem")).is_some());
        assert!(env.get(&Name::str("CoreCharacterization")).is_some());
    }
    #[test]
    fn test_extended_env_axioms() {
        let env = build_rough_set_env();
        for name in &[
            "DecisionSystem",
            "DiscernibilityMatrix",
            "FuzzyLowerApprox",
            "FuzzyUpperApprox",
            "CoveringLower",
            "CoveringUpper",
            "BayesianRough",
            "ProbabilisticRough",
            "MultiGranLower",
            "MultiGranUpper",
            "PartialInclusion",
            "SimilarityRough",
            "MatroidRough",
            "RoughTopology",
            "RoughClosure",
            "RoughNecessary",
            "RoughPossible",
            "RoughTruth",
            "InformationGranule",
            "CoveringReducesToStandard",
            "BayesianPositiveSubset",
            "MultiGranPessLeOpt",
            "FuzzySandwich",
            "PartialInclusionRefl",
            "MatroidExchange",
            "RoughTopologyKuratowski",
            "RoughS5TAxiom",
            "RoughS5FourAxiom",
            "RoughS5FiveAxiom",
            "ReductDiscernibility",
            "DependencyMonotonicity",
            "NeighborhoodTopology",
        ] {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "missing axiom: {name}"
            );
        }
    }
    #[test]
    fn test_decision_system_reducer() {
        let mut info = InformationSystem::new(4, 3);
        let data = [[1u32, 0, 1], [1, 1, 1], [0, 0, 0], [0, 1, 0]];
        for (i, row) in data.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                info.set(i, j, val);
            }
        }
        let dt = DecisionTable::new(info, 2);
        let reducer = DecisionSystemReducer::new(dt);
        let ranked = reducer.ranked_attributes();
        assert!(!ranked.is_empty());
        assert!(ranked.iter().all(|(_, s)| *s >= 0.0));
    }
    #[test]
    fn test_discernibility_matrix_struct() {
        let info = make_simple_info();
        let dt = DecisionTable::new(info, 3);
        let dm = DiscernibilityMatrix::build(&dt);
        assert_eq!(dm.n_objects, 6);
        for i in 0..6 {
            assert!(dm.get(i, i).is_empty());
        }
        let clauses = dm.discernibility_function();
        assert!(!clauses.is_empty());
    }
    #[test]
    fn test_variable_precision_approx_struct() {
        let info = make_simple_info();
        let target: HashSet<usize> = [0, 1, 3].iter().copied().collect();
        let vprs = VariablePrecisionApprox::new(info.clone(), 0.2);
        let lower = vprs.u_lower(&[0, 1, 2], &target);
        let standard_lower = info.lower_approximation(&[0, 1, 2], &target);
        assert!(standard_lower.is_subset(&lower));
        let inc = VariablePrecisionApprox::inclusion_measure(&target, &target);
        assert!((inc - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_rough_fuzzy_membership() {
        let membership = vec![1.0, 0.5, 0.0, 0.8];
        let n = 4;
        let similarity: Vec<Vec<f64>> = (0..n)
            .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.3 }).collect())
            .collect();
        let rfm = RoughFuzzyMembership::new(membership, similarity);
        let upper_0 = rfm.fuzzy_upper(0);
        let lower_0 = rfm.fuzzy_lower(0);
        assert!(upper_0 >= lower_0);
        assert!((RoughFuzzyMembership::t_norm_lukasiewicz(0.8, 0.8) - 0.6).abs() < 1e-9);
        assert!((RoughFuzzyMembership::t_conorm_lukasiewicz(0.3, 0.4) - 0.7).abs() < 1e-9);
    }
    #[test]
    fn test_bayesian_rough_set() {
        let info = make_simple_info();
        let target: HashSet<usize> = [0, 1, 3].iter().copied().collect();
        let brs = BayesianRoughSet::new(0.7, 0.3);
        let (pos, bnd, neg) = brs.classify(&info, &[0, 1, 2], &target);
        let total = pos.len() + bnd.len() + neg.len();
        assert!(total <= info.n_objects);
        assert!(pos.intersection(&neg).next().is_none());
    }
    #[test]
    fn test_covering_system() {
        let mut cs = CoveringSystem::new(4);
        cs.add_cover([0, 1, 2].iter().copied().collect());
        cs.add_cover([1, 2, 3].iter().copied().collect());
        assert!(cs.is_valid_covering());
        let target: HashSet<usize> = [1, 2].iter().copied().collect();
        let upper = cs.upper_approximation(&target);
        assert_eq!(upper.len(), 4);
    }
    #[test]
    fn test_multi_granulation_rough_set() {
        let info = make_simple_info();
        let target: HashSet<usize> = [0, 1, 3].iter().copied().collect();
        let mg = MultiGranulationRoughSet::new(info.clone(), vec![vec![0], vec![1], vec![2]]);
        let opt_lower = mg.optimistic_lower(&target);
        let pess_lower = mg.pessimistic_lower(&target);
        let opt_upper = mg.optimistic_upper(&target);
        let pess_upper = mg.pessimistic_upper(&target);
        assert!(pess_lower.is_subset(&opt_lower));
        assert!(opt_upper.is_subset(&pess_upper));
    }
    #[test]
    fn test_rough_truth_enum() {
        let lower: HashSet<usize> = [0, 1].iter().copied().collect();
        let upper: HashSet<usize> = [0, 1, 2].iter().copied().collect();
        assert_eq!(
            RoughTruth::from_membership(0, &lower, &upper),
            RoughTruth::CertainlyTrue
        );
        assert_eq!(
            RoughTruth::from_membership(2, &lower, &upper),
            RoughTruth::Uncertain
        );
        assert_eq!(
            RoughTruth::from_membership(3, &lower, &upper),
            RoughTruth::CertainlyFalse
        );
        assert_eq!(RoughTruth::CertainlyTrue.not(), RoughTruth::CertainlyFalse);
        assert_eq!(RoughTruth::CertainlyFalse.not(), RoughTruth::CertainlyTrue);
    }
}
#[cfg(test)]
mod tests_rst_extra {
    use super::*;
    fn sample_table() -> DecisionTableExt {
        let mut dt = DecisionTableExt::new(4, vec!["a", "b", "c"], "d");
        dt.set_row(0, vec![1, 0, 2], 1);
        dt.set_row(1, vec![1, 0, 2], 1);
        dt.set_row(2, vec![2, 1, 0], 2);
        dt.set_row(3, vec![2, 1, 1], 2);
        dt
    }
    #[test]
    fn test_equivalence_classes() {
        let dt = sample_table();
        let classes = dt.equivalence_classes(&[0, 1, 2]);
        let has_01_class = classes.iter().any(|c| c.contains(&0) && c.contains(&1));
        assert!(has_01_class, "0 and 1 should be indiscernible");
    }
    #[test]
    fn test_positive_region() {
        let dt = sample_table();
        let all_attrs: Vec<usize> = (0..3).collect();
        let pos = dt.positive_region(&all_attrs);
        assert_eq!(pos.len(), 4);
    }
    #[test]
    fn test_accuracy() {
        let dt = sample_table();
        let concept = vec![0, 1];
        let acc = dt.accuracy(&concept, &[0, 1, 2]);
        assert!(
            (acc - 1.0).abs() < 1e-9,
            "pure class should have accuracy 1"
        );
    }
    #[test]
    fn test_vprs() {
        let vprs = VPRS::new(0.1);
        assert!(vprs.relative_positive_region(10, 10));
        assert!(!vprs.relative_positive_region(5, 10));
        assert!(vprs.relative_positive_region(9, 10));
    }
    #[test]
    fn test_reduct() {
        let r = AttributeReduct::new(vec![0, 2], 5);
        assert!((r.reduction_ratio() - 0.6).abs() < 1e-9);
        assert!(r.is_minimal());
    }
    #[test]
    fn test_discernibility_matrix() {
        let dt = sample_table();
        let dm = DiscernibilityMatrixExt::from_table(&dt);
        assert!(dm.is_consistent());
        let d02 = dm.get(0, 2);
        assert!(!d02.is_empty(), "Objects 0 and 2 should be discernible");
    }
}
