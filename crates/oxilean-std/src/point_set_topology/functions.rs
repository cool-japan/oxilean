//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BaireApplication, ConnectedComponents, ContinuousMap, CoveringSpaceData, Filter,
    FiniteTopology, HomotopyEquivalence, MetricSpace, MetricSpace2, Net, ProductTopology,
    QuotientTopology, QuotientTopology2, SeparationAxiom, SubspaceTopology, TopologicalDimension,
    TopologicalSpace,
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
pub fn real_ty() -> Expr {
    cst("Real")
}
/// Topological space type: (X, τ) where τ is a collection of open sets.
/// TopologicalSpace : Type → Type
pub fn topological_space_ty() -> Expr {
    arrow(type0(), type0())
}
/// Continuous map type: f : X → Y that is continuous.
/// ContinuousMap : (X Y : Type) → TopologicalSpace X → TopologicalSpace Y → Type
pub fn continuous_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(
                app(cst("TopologicalSpace"), cst("X")),
                arrow(app(cst("TopologicalSpace"), cst("Y")), type0()),
            ),
        ),
    )
}
/// Homeomorphism type: bicontinuous bijection between topological spaces.
/// Homeomorphism : (X Y : Type) → TopologicalSpace X → TopologicalSpace Y → Type
pub fn homeomorphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(
                app(cst("TopologicalSpace"), cst("X")),
                arrow(app(cst("TopologicalSpace"), cst("Y")), type0()),
            ),
        ),
    )
}
/// Compact space type: every open cover has a finite subcover.
/// CompactSpace : (X : Type) → TopologicalSpace X → Prop
pub fn compact_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// Connected space type: cannot be partitioned into two disjoint open sets.
/// ConnectedSpace : (X : Type) → TopologicalSpace X → Prop
pub fn connected_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// Metric space type: (X, d) where d is a metric.
/// MetricSpace : Type → Type
pub fn metric_space_ty() -> Expr {
    arrow(type0(), type0())
}
/// Hausdorff (T2) space type: distinct points have disjoint neighbourhoods.
/// HausdorffSpace : (X : Type) → TopologicalSpace X → Prop
pub fn hausdorff_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// OpenSet : (X : Type) → TopologicalSpace X → Set X → Prop
pub fn open_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("TopologicalSpace"), cst("X")),
            arrow(app(cst("Set"), cst("X")), prop()),
        ),
    )
}
/// ClosedSet : (X : Type) → TopologicalSpace X → Set X → Prop
pub fn closed_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("TopologicalSpace"), cst("X")),
            arrow(app(cst("Set"), cst("X")), prop()),
        ),
    )
}
/// Closure of a set: smallest closed superset.
/// Closure : (X : Type) → TopologicalSpace X → Set X → Set X
pub fn closure_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("TopologicalSpace"), cst("X")),
            arrow(app(cst("Set"), cst("X")), app(cst("Set"), cst("X"))),
        ),
    )
}
/// Interior of a set: largest open subset.
/// Interior : (X : Type) → TopologicalSpace X → Set X → Set X
pub fn interior_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("TopologicalSpace"), cst("X")),
            arrow(app(cst("Set"), cst("X")), app(cst("Set"), cst("X"))),
        ),
    )
}
/// Boundary of a set: closure minus interior.
/// Boundary : (X : Type) → TopologicalSpace X → Set X → Set X
pub fn boundary_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("TopologicalSpace"), cst("X")),
            arrow(app(cst("Set"), cst("X")), app(cst("Set"), cst("X"))),
        ),
    )
}
/// SubspaceTopology : (X : Type) → TopologicalSpace X → Set X → TopologicalSpace (Subtype X S)
pub fn subspace_topology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "τ",
            app(cst("TopologicalSpace"), cst("X")),
            pi(
                BinderInfo::Default,
                "S",
                app(cst("Set"), cst("X")),
                app(
                    cst("TopologicalSpace"),
                    app2(cst("Subtype"), cst("X"), cst("S")),
                ),
            ),
        ),
    )
}
/// ProductTopology2 : (X Y : Type) → TopologicalSpace X → TopologicalSpace Y
///                  → TopologicalSpace (Prod X Y)
pub fn product_topology2_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(
                app(cst("TopologicalSpace"), cst("X")),
                arrow(
                    app(cst("TopologicalSpace"), cst("Y")),
                    app(
                        cst("TopologicalSpace"),
                        app2(cst("Prod"), cst("X"), cst("Y")),
                    ),
                ),
            ),
        ),
    )
}
/// QuotientTopology : (X : Type) → TopologicalSpace X → (X → X → Prop)
///                  → TopologicalSpace (Quotient X R)
pub fn quotient_topology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "τ",
            app(cst("TopologicalSpace"), cst("X")),
            pi(
                BinderInfo::Default,
                "R",
                arrow(cst("X"), arrow(cst("X"), prop())),
                app(
                    cst("TopologicalSpace"),
                    app2(cst("Quotient"), cst("X"), cst("R")),
                ),
            ),
        ),
    )
}
/// T0Space (Kolmogorov): for any two distinct points, some open set
/// contains one but not the other.
/// T0Space : (X : Type) → TopologicalSpace X → Prop
pub fn t0_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// T1Space: every singleton {x} is a closed set.
/// T1Space : (X : Type) → TopologicalSpace X → Prop
pub fn t1_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// T3Space (regular Hausdorff): T1 + any point and disjoint closed set
/// can be separated by open sets.
/// T3Space : (X : Type) → TopologicalSpace X → Prop
pub fn t3_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// T3_5Space (Tychonoff / completely regular): T1 + any point and disjoint
/// closed set can be separated by a continuous function.
/// T3_5Space : (X : Type) → TopologicalSpace X → Prop
pub fn t3_5_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// T4Space (normal): T1 + any two disjoint closed sets can be separated
/// by open sets.
/// T4Space : (X : Type) → TopologicalSpace X → Prop
pub fn t4_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// FirstCountable : (X : Type) → TopologicalSpace X → Prop
/// Every point has a countable neighbourhood basis.
pub fn first_countable_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// SecondCountable : (X : Type) → TopologicalSpace X → Prop
/// The whole topology has a countable basis.
pub fn second_countable_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// SeparableSpace : (X : Type) → TopologicalSpace X → Prop
/// Contains a countable dense subset.
pub fn separable_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// LindelofSpace : (X : Type) → TopologicalSpace X → Prop
/// Every open cover has a countable subcover.
pub fn lindelof_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// SequentiallyCompact : (X : Type) → TopologicalSpace X → Prop
/// Every sequence has a convergent subsequence.
pub fn sequentially_compact_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// TotallyBounded : (X : Type) → MetricSpace X → Prop
/// For every ε>0 there is a finite ε-net.
pub fn totally_bounded_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("MetricSpace"), cst("X")), prop()),
    )
}
/// HeineBorel : (X : Type) → MetricSpace X → Prop
/// Compact iff closed and totally bounded.
pub fn heine_borel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("MetricSpace"), cst("X")),
            arrow(
                app(cst("ClosedBounded"), cst("X")),
                app2(
                    cst("CompactSpace"),
                    cst("X"),
                    app(cst("MetricTopology"), cst("X")),
                ),
            ),
        ),
    )
}
/// PathConnected : (X : Type) → TopologicalSpace X → Prop
/// Any two points can be joined by a continuous path.
pub fn path_connected_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// LocallyConnected : (X : Type) → TopologicalSpace X → Prop
/// Every point has a neighbourhood basis of connected open sets.
pub fn locally_connected_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// LocallyPathConnected : (X : Type) → TopologicalSpace X → Prop
pub fn locally_path_connected_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// Filter : (X : Type) → Type
/// A filter on X is a collection of subsets satisfying the filter axioms.
pub fn filter_ty() -> Expr {
    arrow(type0(), type0())
}
/// NetConverges : (X : Type) → TopologicalSpace X → (I → X) → X → Prop
/// A net (directed function) converges to a point.
pub fn net_converges_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "I",
            type0(),
            arrow(
                app(cst("TopologicalSpace"), cst("X")),
                arrow(arrow(cst("I"), cst("X")), arrow(cst("X"), prop())),
            ),
        ),
    )
}
/// FilterConverges : (X : Type) → TopologicalSpace X → Filter X → X → Prop
pub fn filter_converges_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("TopologicalSpace"), cst("X")),
            arrow(app(cst("Filter"), cst("X")), arrow(cst("X"), prop())),
        ),
    )
}
/// Ultrafilter : (X : Type) → Filter X → Prop
/// An ultrafilter is a maximal filter.
pub fn ultrafilter_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("Filter"), cst("X")), prop()),
    )
}
/// CompactnessViaUltrafilters : (X : Type) → TopologicalSpace X →
///   (∀ F : Ultrafilter X, ∃ x, FilterConverges X τ F x) → CompactSpace X τ
pub fn compactness_via_ultrafilters_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "τ",
            app(cst("TopologicalSpace"), cst("X")),
            arrow(
                app2(cst("AllUltrafiltersConverge"), cst("X"), cst("τ")),
                app2(cst("CompactSpace"), cst("X"), cst("τ")),
            ),
        ),
    )
}
/// AlexandroffOnePoint : (X : Type) → TopologicalSpace X →
///   TopologicalSpace (Option X)
/// The one-point compactification adds a point at infinity.
pub fn alexandroff_one_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("TopologicalSpace"), cst("X")),
            app(cst("TopologicalSpace"), app(cst("Option"), cst("X"))),
        ),
    )
}
/// StoneCechCompactification : (X : Type) → T3_5Space X →
///   ∃ (βX : Type), CompactHausdorff βX ∧ DenseEmbedding X βX
pub fn stone_cech_compactification_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("T3_5Space"), cst("X")),
            app(cst("HasStoneCechCompactification"), cst("X")),
        ),
    )
}
/// ProperMap : (X Y : Type) → TopologicalSpace X → TopologicalSpace Y →
///   (X → Y) → Prop
/// f is proper if preimages of compact sets are compact.
pub fn proper_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(
                app(cst("TopologicalSpace"), cst("X")),
                arrow(
                    app(cst("TopologicalSpace"), cst("Y")),
                    arrow(arrow(cst("X"), cst("Y")), prop()),
                ),
            ),
        ),
    )
}
/// QuotientMap : (X Y : Type) → TopologicalSpace X → TopologicalSpace Y →
///   (X → Y) → Prop
/// f is a quotient map if τ_Y = {V | f⁻¹(V) ∈ τ_X}.
pub fn quotient_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(
                app(cst("TopologicalSpace"), cst("X")),
                arrow(
                    app(cst("TopologicalSpace"), cst("Y")),
                    arrow(arrow(cst("X"), cst("Y")), prop()),
                ),
            ),
        ),
    )
}
/// MetrizableSpace : (X : Type) → TopologicalSpace X → Prop
/// X is metrizable if its topology is induced by some metric.
pub fn metrizable_space_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
    )
}
/// UrysohnMetrization : (X : Type) → TopologicalSpace X →
///   SecondCountable X → T3Space X → MetrizableSpace X
pub fn urysohn_metrization_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "τ",
            app(cst("TopologicalSpace"), cst("X")),
            arrow(
                app2(cst("SecondCountable"), cst("X"), cst("τ")),
                arrow(
                    app2(cst("T3Space"), cst("X"), cst("τ")),
                    app2(cst("MetrizableSpace"), cst("X"), cst("τ")),
                ),
            ),
        ),
    )
}
/// TopologicalDimension : (X : Type) → TopologicalSpace X → Nat
/// The (Lebesgue covering) dimension of X.
pub fn topological_dimension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("TopologicalSpace"), cst("X")), cst("Nat")),
    )
}
/// HausdorffDimension : (X : Type) → MetricSpace X → Real
/// The Hausdorff fractal dimension.
pub fn hausdorff_dimension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("MetricSpace"), cst("X")), real_ty()),
    )
}
/// BoxCountingDimension : (X : Type) → MetricSpace X → Real
/// The Minkowski–Bouligand (box-counting) dimension.
pub fn box_counting_dimension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(app(cst("MetricSpace"), cst("X")), real_ty()),
    )
}
/// Tychonoff theorem: product of compact spaces is compact.
pub fn tychonoff_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "I",
        type0(),
        pi(
            BinderInfo::Default,
            "X",
            arrow(cst("I"), type0()),
            arrow(
                app2(cst("CompactFamily"), cst("I"), cst("X")),
                app2(cst("CompactProduct"), cst("I"), cst("X")),
            ),
        ),
    )
}
/// Heine-Cantor theorem.
pub fn heine_cantor_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "Y",
            type0(),
            arrow(
                app2(cst("ContinuousMap"), cst("X"), cst("Y")),
                app2(cst("UniformlyContinuous"), cst("X"), cst("Y")),
            ),
        ),
    )
}
/// Intermediate value theorem.
pub fn intermediate_value_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(real_ty(), real_ty()),
        arrow(
            app2(cst("Continuous"), cst("f"), real_ty()),
            arrow(
                app(cst("Connected"), real_ty()),
                app(cst("HasIntermediateValue"), cst("f")),
            ),
        ),
    )
}
/// Baire category theorem.
pub fn baire_category_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("CompleteMetricSpace"), cst("X")),
            app(cst("BaireSpace"), cst("X")),
        ),
    )
}
/// Urysohn's lemma.
pub fn urysohn_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("NormalSpace"), cst("X")),
            arrow(
                app2(cst("DisjointClosed"), cst("A"), cst("B")),
                app(cst("UrysohnSeparation"), cst("X")),
            ),
        ),
    )
}
/// Tietze Extension Theorem: on a normal space, any continuous function
/// from a closed subspace to ℝ extends to all of X.
/// TietzeExtension : NormalSpace X → ClosedSet X A →
///   ContinuousMap A ℝ → ∃ f : ContinuousMap X ℝ, f|A = g
pub fn tietze_extension_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app(cst("NormalSpace"), cst("X")),
            arrow(
                app2(cst("ClosedSubspace"), cst("X"), cst("A")),
                arrow(
                    app2(cst("ContinuousMap"), cst("A"), real_ty()),
                    app2(cst("HasExtension"), cst("X"), cst("A")),
                ),
            ),
        ),
    )
}
/// Alexander Subbase Theorem: a space is compact iff every subbase cover
/// has a finite subcover.
/// AlexanderSubbase : SubbaseCoverFinite X τ → CompactSpace X τ
pub fn alexander_subbase_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        pi(
            BinderInfo::Default,
            "τ",
            app(cst("TopologicalSpace"), cst("X")),
            arrow(
                app2(cst("SubbaseCoverFinite"), cst("X"), cst("τ")),
                app2(cst("CompactSpace"), cst("X"), cst("τ")),
            ),
        ),
    )
}
/// Cantor Intersection Theorem: in a compact space, a nested sequence of
/// non-empty closed sets has non-empty intersection.
/// CantorIntersection : CompactSpace X → NestedClosedSeq X → NonEmptyIntersection X
pub fn cantor_intersection_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        type0(),
        arrow(
            app2(cst("CompactSpace"), cst("X"), app(cst("TopOf"), cst("X"))),
            arrow(
                app(cst("NestedClosedSeq"), cst("X")),
                app(cst("NonEmptyIntersection"), cst("X")),
            ),
        ),
    )
}
/// Populate an `Environment` with point-set topology axioms and theorems.
pub fn build_point_set_topology_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("TopologicalSpace", topological_space_ty()),
        ("ContinuousMap", continuous_map_ty()),
        ("Homeomorphism", homeomorphism_ty()),
        ("CompactSpace", compact_space_ty()),
        ("ConnectedSpace", connected_space_ty()),
        ("MetricSpace", metric_space_ty()),
        ("HausdorffSpace", hausdorff_space_ty()),
        ("TychonoffTheorem", tychonoff_theorem_ty()),
        ("HeineCantorTheorem", heine_cantor_ty()),
        ("IntermediateValue", intermediate_value_ty()),
        ("BaireCategoryTheorem", baire_category_ty()),
        ("UrysohnLemma", urysohn_lemma_ty()),
        ("NormalSpace", arrow(type0(), prop())),
        ("CompleteMetricSpace", arrow(type0(), prop())),
        ("BaireSpace", arrow(type0(), prop())),
        (
            "Continuous",
            arrow(arrow(real_ty(), real_ty()), arrow(type0(), prop())),
        ),
        ("Connected", arrow(type0(), prop())),
        (
            "HasIntermediateValue",
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
        (
            "UniformlyContinuous",
            arrow(type0(), arrow(type0(), prop())),
        ),
        (
            "CompactFamily",
            arrow(type0(), arrow(arrow(type0(), type0()), prop())),
        ),
        (
            "CompactProduct",
            arrow(type0(), arrow(arrow(type0(), type0()), prop())),
        ),
        ("ProductTopology", arrow(type0(), type0())),
        ("DisjointClosed", arrow(type0(), arrow(type0(), prop()))),
        ("UrysohnSeparation", arrow(type0(), prop())),
        ("Set", arrow(type0(), type0())),
        ("OpenSet", open_set_ty()),
        ("ClosedSet", closed_set_ty()),
        ("Closure", closure_ty()),
        ("Interior", interior_ty()),
        ("Boundary", boundary_ty()),
        (
            "Subtype",
            arrow(type0(), arrow(arrow(type0(), prop()), type0())),
        ),
        ("Prod", arrow(type0(), arrow(type0(), type0()))),
        (
            "Quotient",
            arrow(
                type0(),
                arrow(arrow(type0(), arrow(type0(), prop())), type0()),
            ),
        ),
        ("SubspaceTopology", subspace_topology_ty()),
        ("ProductTopology2", product_topology2_ty()),
        ("QuotientTopology", quotient_topology_ty()),
        ("T0Space", t0_space_ty()),
        ("T1Space", t1_space_ty()),
        ("T3Space", t3_space_ty()),
        ("T3_5Space", t3_5_space_ty()),
        ("T4Space", t4_space_ty()),
        ("FirstCountable", first_countable_ty()),
        ("SecondCountable", second_countable_ty()),
        ("SeparableSpace", separable_space_ty()),
        ("LindelofSpace", lindelof_space_ty()),
        ("SequentiallyCompact", sequentially_compact_ty()),
        ("TotallyBounded", totally_bounded_ty()),
        ("ClosedBounded", arrow(type0(), prop())),
        (
            "MetricTopology",
            arrow(type0(), app(cst("TopologicalSpace"), cst("X"))),
        ),
        ("HeineBorel", heine_borel_ty()),
        ("PathConnected", path_connected_ty()),
        ("LocallyConnected", locally_connected_ty()),
        ("LocallyPathConnected", locally_path_connected_ty()),
        ("Filter", filter_ty()),
        ("NetConverges", net_converges_ty()),
        ("FilterConverges", filter_converges_ty()),
        ("Ultrafilter", ultrafilter_ty()),
        (
            "AllUltrafiltersConverge",
            arrow(
                type0(),
                arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
            ),
        ),
        (
            "CompactnessViaUltrafilters",
            compactness_via_ultrafilters_ty(),
        ),
        ("Option", arrow(type0(), type0())),
        ("AlexandroffOnePoint", alexandroff_one_point_ty()),
        ("HasStoneCechCompactification", arrow(type0(), prop())),
        (
            "StoneCechCompactification",
            stone_cech_compactification_ty(),
        ),
        ("ProperMap", proper_map_ty()),
        ("QuotientMap", quotient_map_ty()),
        ("MetrizableSpace", metrizable_space_ty()),
        ("UrysohnMetrization", urysohn_metrization_ty()),
        ("TopologicalDimension", topological_dimension_ty()),
        ("HausdorffDimension", hausdorff_dimension_ty()),
        ("BoxCountingDimension", box_counting_dimension_ty()),
        ("ClosedSubspace", arrow(type0(), arrow(type0(), prop()))),
        ("HasExtension", arrow(type0(), arrow(type0(), prop()))),
        ("TietzeExtension", tietze_extension_ty()),
        (
            "SubbaseCoverFinite",
            arrow(
                type0(),
                arrow(app(cst("TopologicalSpace"), cst("X")), prop()),
            ),
        ),
        ("AlexanderSubbase", alexander_subbase_ty()),
        ("NestedClosedSeq", arrow(type0(), prop())),
        ("NonEmptyIntersection", arrow(type0(), prop())),
        (
            "TopOf",
            arrow(type0(), app(cst("TopologicalSpace"), cst("X"))),
        ),
        ("CantorIntersection", cantor_intersection_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_topological_space_new_has_two_open_sets() {
        let ts = TopologicalSpace::new(3);
        assert_eq!(ts.open_sets.len(), 2);
    }
    #[test]
    fn test_topological_space_is_open() {
        let ts = TopologicalSpace::new(2);
        assert!(ts.is_open(&[false, false]));
        assert!(ts.is_open(&[true, true]));
        assert!(!ts.is_open(&[true, false]));
    }
    #[test]
    fn test_topological_space_is_closed() {
        let ts = TopologicalSpace::new(2);
        assert!(ts.is_closed(&[true, true]));
        assert!(ts.is_closed(&[false, false]));
    }
    #[test]
    fn test_topological_space_closure_and_interior() {
        let mut ts = TopologicalSpace::new(3);
        ts.add_open_set(vec![true, false, false]);
        let cl = ts.closure(&[false, true, false]);
        assert!(cl[1]);
        let int = ts.interior(&[true, true, false]);
        assert!(int[0]);
        assert!(!int[1]);
    }
    #[test]
    fn test_topological_space_boundary() {
        let mut ts = TopologicalSpace::new(3);
        ts.add_open_set(vec![true, false, false]);
        let bd = ts.boundary(&[true, false, false]);
        assert!(!bd[0]);
        assert!(bd[1]);
        assert!(bd[2]);
    }
    #[test]
    fn test_topological_space_t0_t1() {
        let mut ts = TopologicalSpace::new(2);
        ts.add_open_set(vec![true, false]);
        assert!(ts.is_t0());
        assert!(!ts.is_t1());
    }
    #[test]
    fn test_topological_space_hausdorff_indiscrete() {
        let ts = TopologicalSpace::new(3);
        assert!(!ts.is_hausdorff());
    }
    #[test]
    fn test_topological_space_connected_components_indiscrete() {
        let ts = TopologicalSpace::new(4);
        assert_eq!(ts.num_connected_components(), 1);
    }
    #[test]
    fn test_topological_space_connected_components_discrete() {
        let ft = FiniteTopology::discrete(3);
        assert_eq!(ft.space().num_connected_components(), 3);
    }
    #[test]
    fn test_finite_topology_discrete_lattice_size() {
        let ft = FiniteTopology::discrete(3);
        assert_eq!(ft.lattice_size(), 8);
    }
    #[test]
    fn test_finite_topology_join_meet() {
        let ft = FiniteTopology::discrete(3);
        let a = vec![true, false, false];
        let b = vec![false, true, false];
        let j = ft.join(&a, &b);
        assert_eq!(j, vec![true, true, false]);
        let m = ft.meet(&a, &b);
        assert_eq!(m, vec![false, false, false]);
    }
    #[test]
    fn test_finite_topology_dense() {
        let ft = FiniteTopology::indiscrete(3);
        assert!(ft.is_dense(&[true, true, true]));
        assert!(!ft.is_dense(&[false, false, false]));
    }
    #[test]
    fn test_subspace_topology() {
        let mut ts = TopologicalSpace::new(3);
        ts.add_open_set(vec![true, false, false]);
        let sub = SubspaceTopology::new(&ts, vec![false, true, true]);
        assert_eq!(sub.size, 2);
        assert!(sub.is_open(&[false, false]));
        assert!(sub.is_open(&[true, true]));
    }
    #[test]
    fn test_product_topology_basic() {
        let ts2 = TopologicalSpace::new(2);
        let pt = ProductTopology::new(&ts2, &ts2);
        assert_eq!(pt.total_points(), 4);
        assert!(pt.is_open(&[false, false, false, false]));
        assert!(pt.is_open(&[true, true, true, true]));
    }
    #[test]
    fn test_quotient_topology_two_classes() {
        let mut ts = TopologicalSpace::new(4);
        ts.add_open_set(vec![true, true, false, false]);
        let qt = QuotientTopology::new(&ts, vec![0, 0, 1, 1]);
        assert_eq!(qt.num_classes, 2);
        assert!(qt.is_open(&[true, false]));
        assert!(!qt.is_open(&[false, true]));
    }
    #[test]
    fn test_connected_components_struct() {
        let ft = FiniteTopology::discrete(3);
        let cc = ConnectedComponents::compute(ft.space());
        assert_eq!(cc.num_components(), 3);
        for p in 0..3 {
            let s = cc.component_set(cc.labels[p]);
            assert!(s[p]);
        }
    }
    #[test]
    fn test_metric_space_basics() {
        let mut ms = MetricSpace::new(3);
        ms.set_dist(0, 1, 1.0);
        ms.set_dist(1, 2, 2.0);
        ms.set_dist(0, 2, 3.0);
        assert!((ms.distance(0, 1) - 1.0).abs() < 1e-12);
        assert!((ms.diameter() - 3.0).abs() < 1e-12);
        assert!(ms.is_compact_finite());
    }
    #[test]
    fn test_metric_space_ball() {
        let mut ms = MetricSpace::new(4);
        ms.set_dist(0, 1, 1.0);
        ms.set_dist(0, 2, 2.0);
        ms.set_dist(0, 3, 5.0);
        let ball = ms.ball(0, 2.0);
        assert!(ball.contains(&0));
        assert!(ball.contains(&1));
        assert!(ball.contains(&2));
        assert!(!ball.contains(&3));
    }
    #[test]
    fn test_metric_space_mst() {
        let mut ms = MetricSpace::new(3);
        ms.set_dist(0, 1, 1.0);
        ms.set_dist(1, 2, 2.0);
        ms.set_dist(0, 2, 4.0);
        let mst = ms.mst_length();
        assert!((mst - 3.0).abs() < 1e-12);
    }
    #[test]
    fn test_metric_space_to_topology() {
        let mut ms = MetricSpace::new(3);
        ms.set_dist(0, 1, 1.0);
        ms.set_dist(0, 2, 2.0);
        ms.set_dist(1, 2, 1.0);
        let ts = ms.to_topology();
        assert!(ts.is_open(&[false, false, false]));
        assert!(ts.is_open(&[true, true, true]));
    }
    #[test]
    fn test_continuous_map_bijective_and_compose() {
        let f = ContinuousMap::new(3, 3, vec![1, 2, 0]);
        assert!(f.is_bijective());
        let g = ContinuousMap::new(3, 3, vec![2, 0, 1]);
        let gf = f.compose(&g).expect("compose should succeed");
        assert_eq!(gf.mapping, vec![0, 1, 2]);
    }
    #[test]
    fn test_continuous_map_is_continuous() {
        let ts = TopologicalSpace::new(3);
        let id = ContinuousMap::new(3, 3, vec![0, 1, 2]);
        assert!(id.is_continuous(&ts, &ts));
    }
    #[test]
    fn test_build_point_set_topology_env() {
        let env = build_point_set_topology_env();
        let _ = env;
    }
}
/// Standard examples of topological spaces.
#[allow(dead_code)]
pub fn standard_topological_spaces() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("R^n", "locally compact", "separable"),
        ("S^n (sphere)", "compact", "separable"),
        ("T^n (torus)", "compact", "separable"),
        ("RP^n (real projective)", "compact", "separable"),
        ("CP^n (complex projective)", "compact", "separable"),
        ("Cantor set", "compact", "separable"),
        ("Hilbert cube", "compact", "separable"),
        ("Baire space N^N", "locally compact", "separable"),
        ("Sorgenfrey line", "not compact", "second countable"),
        ("Long line", "locally compact", "not separable"),
    ]
}
/// Urysohn's metrization theorem.
#[allow(dead_code)]
pub fn urysohn_metrization_theorem() -> &'static str {
    "Every regular second-countable T1 space is metrizable (Urysohn)."
}
/// Tychonoff's theorem.
#[allow(dead_code)]
pub fn tychonoff_theorem() -> &'static str {
    "Arbitrary product of compact spaces is compact (Tychonoff, using Axiom of Choice)."
}
#[cfg(test)]
mod pst_ext_tests {
    use super::*;
    #[test]
    fn test_metric_space() {
        let r3 = MetricSpace2::euclidean(3);
        assert!(r3.is_complete);
        assert!(r3.baire_category_theorem_applies());
        assert!(r3.is_polish_space());
    }
    #[test]
    fn test_separation_axioms() {
        assert!(SeparationAxiom::T4.urysohn_lemma_applies());
        assert!(!SeparationAxiom::T2.urysohn_lemma_applies());
        assert!(SeparationAxiom::T4 > SeparationAxiom::T2);
    }
    #[test]
    fn test_covering_space() {
        let univ = CoveringSpaceData::universal_cover("T^2");
        assert!(univ.is_universal());
        assert!(!univ.deck_transformations_description().is_empty());
    }
    #[test]
    fn test_quotient_topology() {
        let s1 = QuotientTopology2::circle_from_interval();
        assert_eq!(s1.quotient_name, "S^1");
    }
    #[test]
    fn test_homotopy_equivalence() {
        let h = HomotopyEquivalence::contractible("R^n");
        assert!(h.same_homology_groups());
    }
    #[test]
    fn test_standard_spaces_nonempty() {
        let spaces = standard_topological_spaces();
        assert!(!spaces.is_empty());
    }
    #[test]
    fn test_tychonoff_theorem() {
        let thm = tychonoff_theorem();
        assert!(thm.contains("compact"));
    }
}
#[cfg(test)]
mod pst_net_filter_tests {
    use super::*;
    #[test]
    fn test_net() {
        let mut n = Net::new("N", "R");
        n.set_limit("0");
        assert!(n.is_convergent);
        assert!(!n.kelley_theorem_description().is_empty());
    }
    #[test]
    fn test_filter() {
        let uf = Filter::free_ultrafilter("N");
        assert!(uf.is_ultrafilter);
        assert!(uf.converges_in_compact_space());
    }
    #[test]
    fn test_baire_application() {
        let ba = BaireApplication::new("R", vec!["Q (rationals)", "Cantor set"], "irrationals");
        assert!(ba.nowhere_dense_union_is_meager());
    }
}
#[cfg(test)]
mod dimension_tests {
    use super::*;
    #[test]
    fn test_dimension_rn() {
        let d = TopologicalDimension::for_rn(3);
        assert_eq!(d.lebesgue_covering_dim, Some(3));
        assert!(d.all_dimensions_agree());
    }
    #[test]
    fn test_cantor_set_dim() {
        let cs = TopologicalDimension::cantor_set();
        assert_eq!(cs.lebesgue_covering_dim, Some(0));
        let hd = cs.hausdorff_dim.expect("hausdorff_dim should be valid");
        assert!((hd - 0.6309).abs() < 0.001);
    }
}
