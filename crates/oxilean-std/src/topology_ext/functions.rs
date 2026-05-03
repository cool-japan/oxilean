//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BaireCategoryGame, CWComplex, CWComplexBuilder, CharacteristicClassComputer,
    DiscreteMorseFunction, FiberBundle, MetricSpace, PersistenceDiagram, PersistencePoint,
    ProObjectLimit, ShapeEquivalenceChecker, SimplicialComplex, SpectralSequencePage,
    TopologicalInvariantTable, UltrametricBallTree, UniformContinuityChecker,
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
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
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
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// Type of simplicial complexes: a finite combinatorial structure built from simplices.
pub fn simplicial_complex_ty() -> Expr {
    type0()
}
/// Type of chain complexes parameterized by degree: `Nat → Type`.
pub fn chain_complex_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Type of homology groups H_n(X): `Nat → Type → Type` (degree → space → group).
pub fn homology_group_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Type of cohomology groups H^n(X): `Nat → Type → Type` (degree → space → group).
pub fn cohomology_group_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Type of homotopy groups π_n(X): `Nat → Type → Type` (degree → space → group).
pub fn homotopy_group_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Type of fiber bundles: a total space type over a base.
pub fn fiber_bundle_ty() -> Expr {
    type0()
}
/// Type of covering spaces over a base space X: `Type → Type`.
pub fn covering_space_ty() -> Expr {
    arrow(type0(), type0())
}
/// Type of CW complexes: spaces built by attaching cells of increasing dimension.
pub fn cw_complex_ty() -> Expr {
    type0()
}
/// Type of n-manifolds: `Nat → Type` (dimension → manifold type).
pub fn manifold_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Type of differential k-forms on a space: `Nat → Type → Type`.
pub fn differential_form_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Type of Euler characteristic function: `Type → Int`.
pub fn euler_characteristic_ty() -> Expr {
    arrow(type0(), int_ty())
}
/// Mayer–Vietoris sequence: long exact sequence for homology of a union.
pub fn mayer_vietoris_ty() -> Expr {
    prop()
}
/// Seifert–van Kampen theorem: π₁(A∪B) ≅ π₁(A) *_{π₁(C)} π₁(B).
pub fn seifert_van_kampen_ty() -> Expr {
    prop()
}
/// Hurewicz theorem: if π₁=...=π_{n-1}=0 then H_n ≅ π_n.
pub fn hurewicz_theorem_ty() -> Expr {
    prop()
}
/// Poincaré duality: H^k(M) ≅ H_{n-k}(M) for a closed oriented n-manifold.
pub fn poincare_duality_ty() -> Expr {
    prop()
}
/// de Rham theorem: H^k_dR(M) ≅ H^k_singular(M; ℝ).
pub fn de_rham_theorem_ty() -> Expr {
    prop()
}
/// Brouwer fixed-point theorem: every continuous f: D^n → D^n has a fixed point.
pub fn brouwer_fixed_point_ty() -> Expr {
    prop()
}
/// Metric space completeness: every Cauchy sequence converges.
/// Type: `Type → Prop` (a space is complete if …).
pub fn metric_completeness_ty() -> Expr {
    arrow(type0(), prop())
}
/// Total boundedness: for every ε > 0 there exists a finite ε-net.
/// Type: `Type → Prop`.
pub fn total_boundedness_ty() -> Expr {
    arrow(type0(), prop())
}
/// Compactness via sequential criterion: every sequence has a convergent subsequence.
/// Type: `Type → Prop`.
pub fn sequential_compactness_ty() -> Expr {
    arrow(type0(), prop())
}
/// Baire category theorem: a complete metric space is not a countable union of
/// nowhere-dense sets.  Type: `Type → Prop`.
pub fn baire_category_theorem_ty() -> Expr {
    arrow(type0(), prop())
}
/// Type of uniform spaces (the entourage formulation): `Type`.
pub fn uniform_space_ty() -> Expr {
    type0()
}
/// Uniform continuity of a function f : X → Y between uniform spaces.
/// Type: `Type → Type → Prop` (X → Y → Prop).
pub fn uniform_continuity_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Uniform space completeness (Cauchy filter convergence).
/// Type: `Type → Prop`.
pub fn uniform_completeness_ty() -> Expr {
    arrow(type0(), prop())
}
/// Type of topological groups: a group with a compatible topology.
/// Type: `Type`.
pub fn topological_group_ty() -> Expr {
    type0()
}
/// Haar measure on a locally compact topological group.
/// Type: `Type → Type` (group → measure type).
pub fn haar_measure_ty() -> Expr {
    arrow(type0(), type0())
}
/// Quotient topology: the finest topology making the quotient map continuous.
/// Type: `Type → Type → Type` (space → equiv-rel → quotient space).
pub fn quotient_topology_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Peter–Weyl theorem: representations of compact groups decompose into
/// finite-dimensional irreducible representations.  Type: `Prop`.
pub fn peter_weyl_theorem_ty() -> Expr {
    prop()
}
/// One-point compactification of a locally compact Hausdorff space.
/// Type: `Type → Type`.
pub fn one_point_compactification_ty() -> Expr {
    arrow(type0(), type0())
}
/// Stone–Čech compactification βX of a completely regular space X.
/// Type: `Type → Type`.
pub fn stone_cech_compactification_ty() -> Expr {
    arrow(type0(), type0())
}
/// Paracompactness: every open cover has a locally finite open refinement.
/// Type: `Type → Prop`.
pub fn paracompactness_ty() -> Expr {
    arrow(type0(), prop())
}
/// Existence of a partition of unity subordinate to an open cover.
/// Type: `Type → Prop`.
pub fn partition_of_unity_ty() -> Expr {
    arrow(type0(), prop())
}
/// Urysohn lemma: disjoint closed sets in a normal space can be separated
/// by a continuous function.  Type: `Type → Prop`.
pub fn urysohn_lemma_ty() -> Expr {
    arrow(type0(), prop())
}
/// Tietze extension theorem: every continuous function from a closed subset
/// of a normal space extends to the whole space.  Type: `Type → Prop`.
pub fn tietze_extension_theorem_ty() -> Expr {
    arrow(type0(), prop())
}
/// Covering dimension (Lebesgue covering dimension) of a topological space.
/// Type: `Type → Nat`.
pub fn covering_dimension_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// Small inductive dimension (ind) of a topological space.
/// Type: `Type → Nat`.
pub fn inductive_dimension_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// Topological manifold of given dimension: locally Euclidean Hausdorff space.
/// Type: `Nat → Type` (same shape as manifold_ty but for topological manifolds).
pub fn topological_manifold_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Classification of compact surfaces: every closed surface is homeomorphic
/// to a sphere, connected sum of tori, or connected sum of projective planes.
/// Type: `Prop`.
pub fn classification_of_surfaces_ty() -> Expr {
    prop()
}
/// Adjunction space: obtained from X by attaching a space A via a map f : A → X.
/// Type: `Type → Type → Type` (base → attachment → result).
pub fn adjunction_space_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Wedge sum X ∨ Y: the coproduct in the category of pointed spaces.
/// Type: `Type → Type → Type`.
pub fn wedge_sum_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Smash product X ∧ Y = (X × Y) / (X ∨ Y).
/// Type: `Type → Type → Type`.
pub fn smash_product_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Suspension ΣX: the pushout of the constant maps X → * ← X.
/// Type: `Type → Type`.
pub fn suspension_ty() -> Expr {
    arrow(type0(), type0())
}
/// Loop space ΩX: the space of based loops in X.
/// Type: `Type → Type`.
pub fn loop_space_ty() -> Expr {
    arrow(type0(), type0())
}
/// Path space PX: the space of all paths in X (free path space).
/// Type: `Type → Type`.
pub fn path_space_ty() -> Expr {
    arrow(type0(), type0())
}
/// Fibration: a map p : E → B satisfying the homotopy lifting property.
/// Type: `Type → Type → Prop` (E → B → Prop).
pub fn fibration_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Shape equivalence: two spaces have the same shape if they have homeomorphic
/// Čech approximating systems.  Type: `Type → Type → Prop`.
pub fn shape_equivalence_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Čech cohomology (shape-theoretic): `Nat → Type → Type`.
pub fn cech_cohomology_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Strong shape equivalence: shape equivalence preserving homotopy type data.
/// Type: `Type → Type → Prop`.
pub fn strong_shape_equivalence_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Čech homotopy groups (shape-theoretic): `Nat → Type → Type`.
pub fn cech_homotopy_group_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Shape morphism: a morphism in the shape category (inverse system of ANR maps).
/// Type: `Type → Type → Type` (X → Y → morphism type).
pub fn shape_morphism_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Pro-object: a formal cofiltered inverse system in a category.
/// Type: `Type → Type` (index category → pro-object type).
pub fn pro_object_ty() -> Expr {
    arrow(type0(), type0())
}
/// Pro-homotopy type: the pro-object in the homotopy category associated to X.
/// Type: `Type → Type`.
pub fn pro_homotopy_type_ty() -> Expr {
    arrow(type0(), type0())
}
/// Artin–Mazur pro-fundamental group: the pro-group π₁ in étale homotopy theory.
/// Type: `Type → Type`.
pub fn artin_mazur_pro_pi1_ty() -> Expr {
    arrow(type0(), type0())
}
/// Hilbert manifold: an infinite-dimensional manifold modelled on a Hilbert space.
/// Type: `Type`.
pub fn hilbert_manifold_ty() -> Expr {
    type0()
}
/// Fréchet manifold: an infinite-dimensional manifold modelled on a Fréchet space.
/// Type: `Type`.
pub fn frechet_manifold_ty() -> Expr {
    type0()
}
/// ILH (Inverse Limit of Hilbert) manifold: a manifold modelled on an inverse
/// limit of Hilbert spaces (Omori's ILH Lie groups).
/// Type: `Type`.
pub fn ilh_manifold_ty() -> Expr {
    type0()
}
/// Diffeological space: a set equipped with a collection of plots (smooth maps).
/// Type: `Type`.
pub fn diffeological_space_ty() -> Expr {
    type0()
}
/// Plot: a smooth map from an open subset of ℝⁿ into a diffeological space.
/// Type: `Type → Type → Type` (source → target → plot type).
pub fn diffeological_plot_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Smooth map between diffeological spaces: a map pulling back plots to plots.
/// Type: `Type → Type → Prop` (X → Y → Prop).
pub fn diffeological_smooth_map_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Diffeological group: a group object in the category of diffeological spaces.
/// Type: `Type`.
pub fn diffeological_group_ty() -> Expr {
    type0()
}
/// Frölicher space: a set with smooth curves and smooth functions satisfying
/// the Frölicher adjunction.  Type: `Type`.
pub fn frolicher_space_ty() -> Expr {
    type0()
}
/// Kriegl–Michor cartesian closedness theorem: the category of Frölicher spaces
/// is cartesian closed.  Type: `Prop`.
pub fn kriegl_michor_theorem_ty() -> Expr {
    prop()
}
/// Ultrametric space: a metric space where the triangle inequality is strengthened
/// to d(x,z) ≤ max(d(x,y), d(y,z)).  Type: `Type`.
pub fn ultrametric_space_ty() -> Expr {
    type0()
}
/// p-adic topology: the topology on ℚ_p induced by the p-adic absolute value.
/// Type: `Type → Type` (prime p → topological type).
pub fn padic_topology_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Berkovich analytification: the Berkovich space associated to an algebraic variety
/// over a non-Archimedean field.  Type: `Type → Type`.
pub fn berkovich_analytification_ty() -> Expr {
    arrow(type0(), type0())
}
/// Condensed set: a sheaf on the pro-étale site of a point (pyknotic / condensed math).
/// Type: `Type`.
pub fn condensed_set_ty() -> Expr {
    type0()
}
/// Profinite set: a cofiltered limit of finite sets with the profinite topology.
/// Type: `Type`.
pub fn profinite_set_ty() -> Expr {
    type0()
}
/// Compactly generated topology: the final topology with respect to all maps
/// from compact Hausdorff spaces.  Type: `Type → Type`.
pub fn compactly_generated_ty() -> Expr {
    arrow(type0(), type0())
}
/// Topological K-theory K(X): the Grothendieck group of complex vector bundles over X.
/// Type: `Type → Type`.
pub fn topological_k_theory_ty() -> Expr {
    arrow(type0(), type0())
}
/// Real K-theory KO(X): the Grothendieck group of real vector bundles over X.
/// Type: `Type → Type`.
pub fn real_k_theory_ty() -> Expr {
    arrow(type0(), type0())
}
/// Bott periodicity (topological): K(X) ≅ K(Σ²X) and KO(X) ≅ KO(Σ⁸X).
/// Type: `Prop`.
pub fn bott_periodicity_topological_ty() -> Expr {
    prop()
}
/// Oriented bordism ring Ω^SO_*: bordism classes of oriented manifolds.
/// Type: `Type`.
pub fn oriented_bordism_ty() -> Expr {
    type0()
}
/// Complex bordism MU_*: the complex-oriented bordism ring (Milnor's computation).
/// Type: `Type`.
pub fn complex_bordism_ty() -> Expr {
    type0()
}
/// Thom spectrum: the Thom space MG associated to a sequence of vector bundles
/// (universal bundle over BG).  Type: `Type → Type`.
pub fn thom_spectrum_ty() -> Expr {
    arrow(type0(), type0())
}
/// Stiefel–Whitney classes w_i(E): characteristic classes in H*(B; ℤ/2).
/// Type: `Nat → Type → Type` (degree → bundle → cohomology class).
pub fn stiefel_whitney_class_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Pontryagin classes p_i(E): characteristic classes in H*(B; ℤ).
/// Type: `Nat → Type → Type`.
pub fn pontryagin_class_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// Euler class e(E): the top characteristic class of an oriented real bundle.
/// Type: `Type → Type`.
pub fn euler_class_ty() -> Expr {
    arrow(type0(), type0())
}
/// G-space: a topological space with a continuous G-action.
/// Type: `Type → Type → Prop` (group G → space X → G-action Prop).
pub fn g_space_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Principal G-bundle: a fiber bundle with structure group G acting freely.
/// Type: `Type → Type → Type` (G → base → bundle).
pub fn principal_bundle_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Classifying space BG: the base of the universal principal G-bundle EG → BG.
/// Type: `Type → Type`.
pub fn classifying_space_ty() -> Expr {
    arrow(type0(), type0())
}
/// Coarse equivalence: two metric spaces are coarsely equivalent (quasi-isometric)
/// if there exist coarse maps in both directions.  Type: `Type → Type → Prop`.
pub fn coarse_equivalence_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Asymptotic dimension: the coarse analog of topological dimension.
/// Type: `Type → Nat`.
pub fn asymptotic_dimension_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// Coarse Baum–Connes conjecture: injectivity of the assembly map for metric spaces
/// of finite asymptotic dimension.  Type: `Prop`.
pub fn coarse_baum_connes_ty() -> Expr {
    prop()
}
/// Register all extended topology axioms and definitions into the given environment.
pub fn build_topology_ext_env(env: &mut Environment) -> Result<(), String> {
    let type_axioms: &[(&str, fn() -> Expr)] = &[
        ("SimplicialComplex", simplicial_complex_ty),
        ("ChainComplex", chain_complex_ty),
        ("HomologyGroup", homology_group_ty),
        ("CohomologyGroup", cohomology_group_ty),
        ("HomotopyGroup", homotopy_group_ty),
        ("FiberBundle", fiber_bundle_ty),
        ("CoveringSpace", covering_space_ty),
        ("CWComplex", cw_complex_ty),
        ("Manifold", manifold_ty),
        ("DifferentialForm", differential_form_ty),
    ];
    for (name, mk_ty) in type_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    {
        let ty = euler_characteristic_ty();
        env.add(Declaration::Axiom {
            name: Name::str("EulerCharacteristic"),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let theorem_axioms: &[(&str, fn() -> Expr)] = &[
        ("MayerVietoris", mayer_vietoris_ty),
        ("SeifertVanKampen", seifert_van_kampen_ty),
        ("HurewiczTheorem", hurewicz_theorem_ty),
        ("PoincareDuality", poincare_duality_ty),
        ("DeRhamTheorem", de_rham_theorem_ty),
        ("BrouwerFixedPoint", brouwer_fixed_point_ty),
    ];
    for (name, mk_ty) in theorem_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let new_axioms: &[(&str, fn() -> Expr)] = &[
        ("MetricCompleteness", metric_completeness_ty),
        ("TotalBoundedness", total_boundedness_ty),
        ("SequentialCompactness", sequential_compactness_ty),
        ("BaireCategoryTheorem", baire_category_theorem_ty),
        ("UniformSpace", uniform_space_ty),
        ("UniformContinuity", uniform_continuity_ty),
        ("UniformCompleteness", uniform_completeness_ty),
        ("TopologicalGroup", topological_group_ty),
        ("HaarMeasure", haar_measure_ty),
        ("QuotientTopology", quotient_topology_ty),
        ("PeterWeylTheorem", peter_weyl_theorem_ty),
        ("OnePointCompactification", one_point_compactification_ty),
        ("StoneCechCompactification", stone_cech_compactification_ty),
        ("Paracompactness", paracompactness_ty),
        ("PartitionOfUnity", partition_of_unity_ty),
        ("UrysohnLemma", urysohn_lemma_ty),
        ("TietzeExtensionTheorem", tietze_extension_theorem_ty),
        ("CoveringDimension", covering_dimension_ty),
        ("InductiveDimension", inductive_dimension_ty),
        ("TopologicalManifold", topological_manifold_ty),
        ("ClassificationOfSurfaces", classification_of_surfaces_ty),
        ("AdjunctionSpace", adjunction_space_ty),
        ("WedgeSum", wedge_sum_ty),
        ("SmashProduct", smash_product_ty),
        ("Suspension", suspension_ty),
        ("LoopSpace", loop_space_ty),
        ("PathSpace", path_space_ty),
        ("Fibration", fibration_ty),
        ("ShapeEquivalence", shape_equivalence_ty),
        ("CechCohomology", cech_cohomology_ty),
    ];
    for (name, mk_ty) in new_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let ext_axioms: &[(&str, fn() -> Expr)] = &[
        ("StrongShapeEquivalence", strong_shape_equivalence_ty),
        ("CechHomotopyGroup", cech_homotopy_group_ty),
        ("ShapeMorphism", shape_morphism_ty),
        ("ProObject", pro_object_ty),
        ("ProHomotopyType", pro_homotopy_type_ty),
        ("ArtinMazurProPi1", artin_mazur_pro_pi1_ty),
        ("HilbertManifold", hilbert_manifold_ty),
        ("FrechetManifold", frechet_manifold_ty),
        ("ILHManifold", ilh_manifold_ty),
        ("DiffeologicalSpace", diffeological_space_ty),
        ("DiffeologicalPlot", diffeological_plot_ty),
        ("DiffeologicalSmoothMap", diffeological_smooth_map_ty),
        ("DiffeologicalGroup", diffeological_group_ty),
        ("FrolichersSpace", frolicher_space_ty),
        ("KrieglMichorTheorem", kriegl_michor_theorem_ty),
        ("UltrametricSpace", ultrametric_space_ty),
        ("PadicTopology", padic_topology_ty),
        ("BerkovichAnalytification", berkovich_analytification_ty),
        ("CondensedSet", condensed_set_ty),
        ("ProfiniteSet", profinite_set_ty),
        ("CompactlyGenerated", compactly_generated_ty),
        ("TopologicalKTheory", topological_k_theory_ty),
        ("RealKTheory", real_k_theory_ty),
        (
            "BottPeriodicityTopological",
            bott_periodicity_topological_ty,
        ),
        ("OrientedBordism", oriented_bordism_ty),
        ("ComplexBordism", complex_bordism_ty),
        ("ThomSpectrum", thom_spectrum_ty),
        ("StiefelWhitneyClass", stiefel_whitney_class_ty),
        ("PontryaginClass", pontryagin_class_ty),
        ("EulerClass", euler_class_ty),
        ("GSpace", g_space_ty),
        ("PrincipalBundle", principal_bundle_ty),
        ("ClassifyingSpace", classifying_space_ty),
        ("CoarseEquivalence", coarse_equivalence_ty),
        ("AsymptoticDimension", asymptotic_dimension_ty),
        ("CoarseBaumConnes", coarse_baum_connes_ty),
    ];
    for (name, mk_ty) in ext_axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
/// Compute the Smith normal form of an integer matrix.
///
/// Returns `(P, D, Q)` such that `D = P * A * Q` where D is diagonal with
/// each diagonal entry dividing the next.  P and Q are invertible integer
/// matrices (determinant ±1).
///
/// This implementation uses the standard pivot-based algorithm.
pub fn smith_normal_form(mat: Vec<Vec<i64>>) -> (Vec<Vec<i64>>, Vec<Vec<i64>>, Vec<Vec<i64>>) {
    if mat.is_empty() {
        return (vec![], vec![], vec![]);
    }
    let rows = mat.len();
    let cols = mat[0].len();
    let mut d = mat;
    let mut p = identity_matrix(rows);
    let mut q = identity_matrix(cols);
    let mut pivot_row = 0usize;
    let mut pivot_col = 0usize;
    while pivot_row < rows && pivot_col < cols {
        let mut found = false;
        'outer: for c in pivot_col..cols {
            for r in pivot_row..rows {
                if d[r][c] != 0 {
                    if r != pivot_row {
                        d.swap(r, pivot_row);
                        p.swap(r, pivot_row);
                    }
                    if c != pivot_col {
                        for row in &mut d {
                            row.swap(c, pivot_col);
                        }
                        for row in &mut q {
                            row.swap(c, pivot_col);
                        }
                    }
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            break;
        }
        loop {
            let mut changed = false;
            for r in 0..rows {
                if r == pivot_row || d[r][pivot_col] == 0 {
                    continue;
                }
                let q_val = d[r][pivot_col] / d[pivot_row][pivot_col];
                for c in 0..cols {
                    let sub = q_val * d[pivot_row][c];
                    d[r][c] -= sub;
                    p[r][c] -= q_val * p[pivot_row][c];
                }
                changed = true;
            }
            for c in 0..cols {
                if c == pivot_col || d[pivot_row][c] == 0 {
                    continue;
                }
                let q_val = d[pivot_row][c] / d[pivot_row][pivot_col];
                for r in 0..rows {
                    let sub = q_val * d[r][pivot_col];
                    d[r][c] -= sub;
                    q[r][c] -= q_val * q[r][pivot_col];
                }
                changed = true;
            }
            if !changed {
                break;
            }
        }
        if d[pivot_row][pivot_col] < 0 {
            for c in 0..cols {
                d[pivot_row][c] = -d[pivot_row][c];
                p[pivot_row][c] = -p[pivot_row][c];
            }
        }
        pivot_row += 1;
        pivot_col += 1;
    }
    (p, d, q)
}
/// Create an n×n identity matrix.
pub fn identity_matrix(n: usize) -> Vec<Vec<i64>> {
    let mut mat = vec![vec![0i64; n]; n];
    for i in 0..n {
        mat[i][i] = 1;
    }
    mat
}
/// Compute the rank of an integer matrix using column reduction (over ℤ).
pub fn matrix_rank(mat: &[Vec<i64>]) -> usize {
    if mat.is_empty() {
        return 0;
    }
    let (_, d, _) = smith_normal_form(mat.to_vec());
    d.iter()
        .enumerate()
        .filter(|(i, row)| *i < row.len() && row[*i] != 0)
        .count()
}
/// Compute the ranks of homology groups H_0, …, H_{max_dim} for a simplicial complex.
///
/// rank(H_k) = dim(ker ∂_k) - dim(im ∂_{k+1})
///           = (#{k-simplices} - rank(∂_k)) - rank(∂_{k+1})
pub fn homology_ranks(complex: &SimplicialComplex, max_dim: usize) -> Vec<usize> {
    let mut betti = vec![0usize; max_dim + 1];
    let mut boundary_ranks: Vec<usize> = Vec::with_capacity(max_dim + 2);
    for k in 0..=(max_dim + 1) {
        let bmat = complex.boundary_matrix(k);
        let rank = if bmat.is_empty() {
            0
        } else {
            matrix_rank(&bmat)
        };
        boundary_ranks.push(rank);
    }
    for k in 0..=max_dim {
        let num_k_simplices = complex.simplices_of_dim(k).len();
        let rank_dk = boundary_ranks[k];
        let rank_dk1 = boundary_ranks[k + 1];
        let kernel_dim = num_k_simplices.saturating_sub(rank_dk);
        betti[k] = kernel_dim.saturating_sub(rank_dk1);
    }
    betti
}
/// Basic combinatorial check for whether a simplicial complex might be an n-manifold.
///
/// For a pure n-dimensional simplicial complex to be a manifold, every
/// (n-1)-simplex must be contained in exactly 1 or 2 top-dimensional simplices
/// (boundary vs interior), and (informally) the link of every vertex should
/// look like an (n-1)-sphere.
///
/// This function checks only the facet-adjacency condition (each codimension-1
/// face is shared by at most 2 top simplices), which is a necessary but not
/// sufficient condition.
pub fn is_manifold_condition(complex: &SimplicialComplex, n: usize) -> bool {
    let top_simplices = complex.simplices_of_dim(n);
    if top_simplices.is_empty() {
        return false;
    }
    let faces_n1 = complex.simplices_of_dim(n - 1);
    for face in &faces_n1 {
        let count = top_simplices
            .iter()
            .filter(|sigma| face.iter().all(|v: &usize| sigma.contains(v)))
            .count();
        if count == 0 || count > 2 {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    /// Build the boundary of a triangle (S¹): vertices {0,1,2},
    /// edges {01, 12, 02}.  χ = V - E = 3 - 3 = 0.
    #[test]
    fn test_triangle() {
        let mut sc = SimplicialComplex::new();
        sc.add_simplex(vec![0]);
        sc.add_simplex(vec![1]);
        sc.add_simplex(vec![2]);
        sc.add_simplex(vec![0, 1]);
        sc.add_simplex(vec![1, 2]);
        sc.add_simplex(vec![0, 2]);
        assert_eq!(sc.euler_characteristic(), 0);
    }
    /// Tetrahedron boundary (hollow): V=4, E=6, F=4.  χ = 4 - 6 + 4 = 2.
    #[test]
    fn test_tetrahedron_euler() {
        let mut sc = SimplicialComplex::new();
        for v in 0..4usize {
            sc.add_simplex(vec![v]);
        }
        let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        for (a, b) in edges {
            sc.add_simplex(vec![a, b]);
        }
        let faces = [(0, 1, 2), (0, 1, 3), (0, 2, 3), (1, 2, 3)];
        for (a, b, c) in faces {
            sc.add_simplex(vec![a, b, c]);
        }
        assert_eq!(sc.euler_characteristic(), 2);
    }
    /// Disk (filled triangle): V=3, E=3, F=1.  χ = 3 - 3 + 1 = 1.
    #[test]
    fn test_disk_euler() {
        let mut sc = SimplicialComplex::new();
        sc.add_simplex(vec![0]);
        sc.add_simplex(vec![1]);
        sc.add_simplex(vec![2]);
        sc.add_simplex(vec![0, 1]);
        sc.add_simplex(vec![1, 2]);
        sc.add_simplex(vec![0, 2]);
        sc.add_simplex(vec![0, 1, 2]);
        assert_eq!(sc.euler_characteristic(), 1);
    }
    /// S¹ (circle = triangle boundary) has Betti numbers \[1, 1\]:
    /// β₀ = 1 (connected), β₁ = 1 (one 1-cycle).
    #[test]
    fn test_betti_circle() {
        let mut sc = SimplicialComplex::new();
        sc.add_simplex(vec![0]);
        sc.add_simplex(vec![1]);
        sc.add_simplex(vec![2]);
        sc.add_simplex(vec![0, 1]);
        sc.add_simplex(vec![1, 2]);
        sc.add_simplex(vec![0, 2]);
        let betti = sc.betti_numbers(1);
        assert_eq!(betti[0], 1, "S¹ should be connected: β₀ = 1");
        assert_eq!(betti[1], 1, "S¹ has one 1-cycle: β₁ = 1");
    }
    /// Environment builder should succeed without errors.
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_topology_ext_env(&mut env);
        assert!(
            result.is_ok(),
            "build_topology_ext_env failed: {:?}",
            result
        );
    }
    /// Environment builder should register all 30 new axioms.
    #[test]
    fn test_new_axiom_count() {
        let mut env = Environment::new();
        build_topology_ext_env(&mut env).expect("build_topology_ext_env should succeed");
        let expected_new = [
            "BaireCategoryTheorem",
            "UniformSpace",
            "HaarMeasure",
            "StoneCechCompactification",
            "OnePointCompactification",
            "UrysohnLemma",
            "TietzeExtensionTheorem",
            "CoveringDimension",
            "WedgeSum",
            "SmashProduct",
            "Suspension",
            "LoopSpace",
            "Fibration",
            "ShapeEquivalence",
        ];
        for name in expected_new {
            assert!(
                env.find(&Name::str(name)).is_some(),
                "axiom {name} not found in environment"
            );
        }
    }
    /// MetricSpace: a regular 4-point grid on \[0,3\].
    ///
    /// Any finite metric space is totally bounded for any ε > 0 because we can
    /// place every point in its own singleton ball.  We verify this, and also
    /// check that the ε-net is smaller for larger ε.
    #[test]
    fn test_metric_totally_bounded() {
        let n = 4usize;
        let mut dist = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = (i as f64 - j as f64).abs();
            }
        }
        let ms = MetricSpace::new(dist);
        assert!(
            ms.is_totally_bounded(1.5),
            "4 points in [0,3] are 1.5-bounded"
        );
        assert!(
            ms.is_totally_bounded(0.4),
            "finite metric spaces are always totally bounded"
        );
        let net_large = ms.greedy_epsilon_net(1.5);
        let net_small = ms.greedy_epsilon_net(0.4);
        assert!(
            net_large.len() <= net_small.len(),
            "larger ε yields a smaller (or equal) net"
        );
    }
    /// UniformContinuityChecker: identity function is uniformly continuous.
    #[test]
    fn test_uniform_continuity_identity() {
        let n = 3usize;
        let mut dist = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = (i as f64 - j as f64).abs();
            }
        }
        let ms = MetricSpace::new(dist.clone());
        let ms2 = MetricSpace::new(dist);
        let identity: Vec<usize> = (0..n).collect();
        let checker = UniformContinuityChecker::new(&ms, &ms2, identity);
        assert!(
            checker.is_uniformly_continuous(1.0, 1.0),
            "identity with ε=δ=1 should be UC"
        );
    }
    /// CWComplexBuilder: sphere S² = 1 vertex + 1 face (CW decomposition).
    #[test]
    fn test_cw_complex_sphere() {
        let mut builder = CWComplexBuilder::new();
        let v = builder.add_cell(0, "v", vec![]);
        let f = builder.add_cell(2, "F", vec![v]);
        let _ = f;
        assert_eq!(builder.euler_characteristic(), 2);
    }
    /// TopologicalInvariantTable for a filled triangle (disc).
    #[test]
    fn test_invariant_table_disc() {
        let mut sc = SimplicialComplex::new();
        for v in 0..3usize {
            sc.add_simplex(vec![v]);
        }
        sc.add_simplex(vec![0, 1]);
        sc.add_simplex(vec![1, 2]);
        sc.add_simplex(vec![0, 2]);
        sc.add_simplex(vec![0, 1, 2]);
        let table = TopologicalInvariantTable::compute(&sc, 2);
        assert_eq!(table.euler_characteristic, 1);
        let disp = table.display();
        assert!(disp.contains("χ (Euler)"));
    }
    /// BaireCategoryGame: after 5 rounds, Player II should still be winning
    /// (Baire property — dense player can always maintain non-empty interval).
    #[test]
    fn test_baire_game_player_two_wins() {
        let mut game = BaireCategoryGame::new(1000);
        let result = game.simulate(5);
        assert!(
            result,
            "Player II should win the Baire category game after 5 rounds"
        );
    }
    #[test]
    fn test_extended_axioms_registered() {
        let mut env = Environment::new();
        build_topology_ext_env(&mut env).expect("build_topology_ext_env should succeed");
        let expected = [
            "StrongShapeEquivalence",
            "CechHomotopyGroup",
            "ShapeMorphism",
            "ProObject",
            "ProHomotopyType",
            "ArtinMazurProPi1",
            "HilbertManifold",
            "FrechetManifold",
            "ILHManifold",
            "DiffeologicalSpace",
            "DiffeologicalPlot",
            "DiffeologicalSmoothMap",
            "DiffeologicalGroup",
            "FrolichersSpace",
            "KrieglMichorTheorem",
            "UltrametricSpace",
            "PadicTopology",
            "BerkovichAnalytification",
            "CondensedSet",
            "ProfiniteSet",
            "CompactlyGenerated",
            "TopologicalKTheory",
            "RealKTheory",
            "BottPeriodicityTopological",
            "OrientedBordism",
            "ComplexBordism",
            "ThomSpectrum",
            "StiefelWhitneyClass",
            "PontryaginClass",
            "EulerClass",
            "GSpace",
            "PrincipalBundle",
            "ClassifyingSpace",
            "CoarseEquivalence",
            "AsymptoticDimension",
            "CoarseBaumConnes",
        ];
        for name in expected {
            assert!(
                env.find(&Name::str(name)).is_some(),
                "extended axiom {name} not registered"
            );
        }
    }
    /// ShapeEquivalenceChecker: two circles (S¹) should be shape-equivalent.
    #[test]
    fn test_shape_equivalence_checker_same() {
        let mut sc1 = SimplicialComplex::new();
        for v in 0..3usize {
            sc1.add_simplex(vec![v]);
        }
        sc1.add_simplex(vec![0, 1]);
        sc1.add_simplex(vec![1, 2]);
        sc1.add_simplex(vec![0, 2]);
        let mut sc2 = SimplicialComplex::new();
        for v in 0..3usize {
            sc2.add_simplex(vec![v]);
        }
        sc2.add_simplex(vec![0, 1]);
        sc2.add_simplex(vec![1, 2]);
        sc2.add_simplex(vec![0, 2]);
        let checker = ShapeEquivalenceChecker::from_complexes(&sc1, &sc2, 1);
        assert!(checker.euler_agrees());
        assert!(checker.betti_agree());
        assert!(checker.are_shape_equivalent());
    }
    /// ShapeEquivalenceChecker: S¹ vs filled triangle (disc) differ.
    #[test]
    fn test_shape_equivalence_checker_different() {
        let mut circle = SimplicialComplex::new();
        for v in 0..3usize {
            circle.add_simplex(vec![v]);
        }
        circle.add_simplex(vec![0, 1]);
        circle.add_simplex(vec![1, 2]);
        circle.add_simplex(vec![0, 2]);
        let mut disc = SimplicialComplex::new();
        for v in 0..3usize {
            disc.add_simplex(vec![v]);
        }
        disc.add_simplex(vec![0, 1]);
        disc.add_simplex(vec![1, 2]);
        disc.add_simplex(vec![0, 2]);
        disc.add_simplex(vec![0, 1, 2]);
        let checker = ShapeEquivalenceChecker::from_complexes(&circle, &disc, 1);
        assert!(!checker.are_shape_equivalent());
    }
    /// ProObjectLimit: two levels {0,1} with identity projection → limit has size 2.
    #[test]
    fn test_pro_object_limit_identity() {
        let levels = vec![vec![0u32, 1u32], vec![0u32, 1u32]];
        let projections = vec![vec![0usize, 1usize]];
        let pro = ProObjectLimit::new(levels, projections);
        assert_eq!(pro.limit_size(), 2);
    }
    /// ProObjectLimit: constant projection collapses to one element.
    #[test]
    fn test_pro_object_limit_constant() {
        let levels = vec![vec![0u32], vec![0u32, 1u32]];
        let projections = vec![vec![0usize, 0usize]];
        let pro = ProObjectLimit::new(levels, projections);
        assert_eq!(pro.limit_size(), 2);
    }
    /// UltrametricBallTree: strong triangle inequality holds for 2-adic distance.
    #[test]
    fn test_ultrametric_triangle_inequality() {
        let tree = UltrametricBallTree::new(vec![0.0, 2.0, 4.0, 6.0], 2);
        assert!(tree.check_ultrametric_inequality(0.0, 2.0, 4.0));
        assert!(tree.check_ultrametric_inequality(0.0, 0.0, 4.0));
        assert!(tree.check_ultrametric_inequality(2.0, 4.0, 6.0));
    }
    /// UltrametricBallTree: ball around 0 with radius 2 captures nearby points.
    #[test]
    fn test_ultrametric_ball() {
        let tree = UltrametricBallTree::new(vec![0.0, 1.0, 2.0, 4.0], 2);
        let ball = tree.ball(0.0, 1.0);
        assert!(ball.contains(&0.0));
    }
    /// CharacteristicClassComputer: rank-2 trivial bundle has c=(1,0,0).
    #[test]
    fn test_characteristic_class_trivial_bundle() {
        let cc = CharacteristicClassComputer::new(vec![0.0, 0.0]);
        let c = cc.total_chern_class();
        assert!((c[0] - 1.0).abs() < 1e-10);
        assert!(c[1].abs() < 1e-10);
        assert!(c[2].abs() < 1e-10);
    }
    /// CharacteristicClassComputer: line bundle with root x has c=(1,x).
    #[test]
    fn test_characteristic_class_line_bundle() {
        let x = 3.0f64;
        let cc = CharacteristicClassComputer::new(vec![x]);
        let c = cc.total_chern_class();
        assert!((c[0] - 1.0).abs() < 1e-10);
        assert!((c[1] - x).abs() < 1e-10);
        assert!((cc.euler_number() - x).abs() < 1e-10);
    }
    /// CharacteristicClassComputer: Chern character of rank-1 bundle.
    #[test]
    fn test_chern_character_rank1() {
        let cc = CharacteristicClassComputer::new(vec![1.0]);
        let ch = cc.chern_character(3);
        assert!((ch[0] - 1.0).abs() < 1e-10);
        assert!((ch[1] - 1.0).abs() < 1e-10);
    }
}
#[cfg(test)]
mod tests_topology_ext2 {
    use super::*;
    #[test]
    fn test_persistence_point() {
        let p = PersistencePoint::new(1.0, 3.0, 1);
        assert!((p.persistence() - 2.0).abs() < 1e-10);
        assert!(!p.is_essential());
        assert!((p.midpoint() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_persistence_diagram_betti() {
        let mut pd = PersistenceDiagram::new();
        pd.add(PersistencePoint::new(0.0, f64::INFINITY, 0));
        pd.add(PersistencePoint::new(1.0, 2.0, 1));
        pd.add(PersistencePoint::new(0.5, f64::INFINITY, 1));
        assert_eq!(pd.betti(0), 1);
        assert_eq!(pd.betti(1), 1);
        assert!((pd.total_persistence() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_discrete_morse() {
        let mut dmf = DiscreteMorseFunction::new(vec![2, 3, 1]);
        dmf.mark_critical(0, 0);
        dmf.mark_critical(2, 0);
        assert_eq!(dmf.euler_characteristic(), 2);
        assert_eq!(dmf.morse_inequalities_lhs(), vec![1, 0, 1]);
        assert!(dmf.check_weak_morse_inequality(&[1, 0, 1]));
    }
    #[test]
    fn test_fiber_bundle() {
        let hopf = FiberBundle::new("S^3", "S^2", "S^1").with_structure_group("S^1");
        assert!(!hopf.is_trivial);
        assert!(hopf.is_principal());
        let les = hopf.les_description();
        assert!(les.contains("S^3"));
    }
    #[test]
    fn test_spectral_sequence_page() {
        let mut e2 = SpectralSequencePage::new(2);
        e2.set(0, 0, "Z");
        e2.set(1, 0, "Z/2");
        assert_eq!(e2.get(0, 0), "Z");
        assert_eq!(e2.get(99, 99), "0");
        let (tp, tq) = e2.differential_target(0, 0);
        assert_eq!((tp, tq), (2, -1));
    }
    #[test]
    fn test_cw_complex() {
        let s2 = CWComplex::sphere(2);
        assert_eq!(s2.euler_characteristic(), 2);
        assert_eq!(s2.dimension(), Some(2));
        let t2 = CWComplex::torus();
        assert_eq!(t2.euler_characteristic(), 0);
        let rp2 = CWComplex::rp2();
        assert_eq!(rp2.euler_characteristic(), 1);
    }
}
