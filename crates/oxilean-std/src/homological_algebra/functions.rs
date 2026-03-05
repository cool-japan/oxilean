//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::HashMap;

use super::types::{
    ChainComplex, ChainGroup, ChowGroupData, DGCategoryData, ExtGroup, MixedHodgeStructureData,
    SpectralSequence, SpectralSequencePage, TriangulatedCategoryData,
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
/// `ChainComplex : (n : ℤ) → ModuleObj n → Type` — a graded family of modules
/// with boundary maps d_n : C_n → C_{n-1} satisfying d ∘ d = 0.
pub fn chain_complex_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        int_ty(),
        arrow(app(cst("ModuleObj"), bvar(0)), type0()),
    )
}
/// `Homology : ChainComplex → ℤ → Module` — the n-th homology group
/// H_n(C) = ker(d_n) / im(d_{n+1}).
pub fn homology_group_ty() -> Expr {
    arrow(cst("ChainComplex"), arrow(int_ty(), cst("Module")))
}
/// `ExactSequence : Module → Module → Module → Prop` — exactness of 0 → A → B → C → 0.
///
/// The sequence is exact iff im(f) = ker(g) at every position.
pub fn exact_sequence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("Module"),
        pi(
            BinderInfo::Default,
            "B",
            cst("Module"),
            pi(
                BinderInfo::Default,
                "C",
                cst("Module"),
                arrow(
                    arrow(bvar(2), bvar(1)),
                    arrow(arrow(bvar(2), bvar(1)), prop()),
                ),
            ),
        ),
    )
}
/// `Ext : Nat → Module → Module → Module` — the n-th Ext group Ext^n(M, N),
/// the right derived functor of Hom(M, –).
pub fn derived_functor_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("Module"), arrow(cst("Module"), cst("Module"))),
    )
}
/// `Tor : Nat → Module → Module → Module` — the n-th Tor group Tor_n(M, N),
/// the left derived functor of M ⊗ –.
pub fn tor_functor_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("Module"), arrow(cst("Module"), cst("Module"))),
    )
}
/// Snake Lemma: given a commutative diagram with exact rows
///
/// ```text
///   0 → A → B → C → 0
///       ↓f  ↓g  ↓h
///   0 → A'→ B'→ C'→ 0
/// ```
///
/// there is a connecting homomorphism δ : ker(h) → coker(f) making the
/// six-term sequence exact.
pub fn snake_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("Module"),
        pi(
            BinderInfo::Default,
            "B",
            cst("Module"),
            pi(
                BinderInfo::Default,
                "C",
                cst("Module"),
                arrow(app3(cst("ExactRow"), bvar(2), bvar(1), bvar(0)), prop()),
            ),
        ),
    )
}
/// Five Lemma: in a commutative diagram with exact rows
///
/// ```text
///  A₁ → A₂ → A₃ → A₄ → A₅
///  ↓f₁  ↓f₂  ↓f₃  ↓f₄  ↓f₅
///  B₁ → B₂ → B₃ → B₄ → B₅
/// ```
///
/// if f₁, f₂, f₄, f₅ are isomorphisms then f₃ is also an isomorphism.
pub fn five_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("FiveTermSeq"),
        pi(
            BinderInfo::Default,
            "B",
            cst("FiveTermSeq"),
            arrow(
                app2(cst("CornerIsomorphisms"), bvar(1), bvar(0)),
                app2(cst("MiddleIsomorphism"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// Populate an `Environment` with homological-algebra axioms.
pub fn build_homological_algebra_env(env: &mut Environment) {
    let base_types: &[(&str, fn() -> Expr)] = &[
        ("Module", || type1()),
        ("ModuleObj", || arrow(int_ty(), type1())),
        ("ChainComplex", || type1()),
        ("ExactRow", || {
            arrow(
                cst("Module"),
                arrow(cst("Module"), arrow(cst("Module"), prop())),
            )
        }),
        ("FiveTermSeq", || type1()),
    ];
    for (name, mk_ty) in base_types {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let type_axioms: &[(&str, fn() -> Expr)] = &[
        ("Homology", homology_group_ty),
        ("ExactSequence", exact_sequence_ty),
        ("Ext", derived_functor_ty),
        ("Tor", tor_functor_ty),
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
    let theorem_axioms: &[(&str, fn() -> Expr)] = &[
        ("snake_lemma", snake_lemma_ty),
        ("five_lemma", five_lemma_ty),
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
}
/// Compute the rank of the image of an integer matrix via row reduction over ℤ.
///
/// The matrix has `cols` columns.  Row-reduction counts the number of nonzero
/// pivot rows, which equals the image rank.
pub fn image_rank(matrix: &[Vec<i64>], cols: usize) -> usize {
    if matrix.is_empty() || cols == 0 {
        return 0;
    }
    let rows = matrix.len();
    let mut m: Vec<Vec<i64>> = (0..rows)
        .map(|r| (0..cols).map(|c| matrix[r][c]).collect())
        .collect();
    let mut pivot_col = 0usize;
    let mut pivot_row = 0usize;
    while pivot_row < rows && pivot_col < cols {
        let found = (pivot_row..rows).find(|&r| m[r][pivot_col] != 0);
        match found {
            None => {
                pivot_col += 1;
            }
            Some(r) => {
                m.swap(pivot_row, r);
                let piv = m[pivot_row][pivot_col];
                for r2 in (pivot_row + 1)..rows {
                    let factor = m[r2][pivot_col];
                    if factor != 0 {
                        for c in 0..cols {
                            m[r2][c] = m[r2][c] * piv - m[pivot_row][c] * factor;
                        }
                    }
                }
                pivot_row += 1;
                pivot_col += 1;
            }
        }
    }
    pivot_row
}
/// Compute the rank of the kernel of an integer matrix.
///
/// For a linear map d : ℤ^cols → ℤ^rows, ker rank = cols - image rank.
pub fn kernel_rank(matrix: &[Vec<i64>], cols: usize) -> usize {
    let img = image_rank(matrix, cols);
    cols.saturating_sub(img)
}
/// `TriangulatedCategory : Type` — a category equipped with an autoequivalence
/// Σ (suspension) and a class of distinguished triangles X → Y → Z → ΣX.
pub fn triangulated_category_ty() -> Expr {
    type1()
}
/// `DistinguishedTriangle : TriangulatedCategory → Obj → Obj → Obj → Prop`
/// — asserts that X →f Y →g Z →h ΣX is a distinguished triangle.
pub fn distinguished_triangle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("TriangulatedCategory"),
        pi(
            BinderInfo::Default,
            "X",
            app(cst("CatObj"), bvar(0)),
            pi(
                BinderInfo::Default,
                "Y",
                app(cst("CatObj"), bvar(1)),
                pi(
                    BinderInfo::Default,
                    "Z",
                    app(cst("CatObj"), bvar(2)),
                    prop(),
                ),
            ),
        ),
    )
}
/// `OctahedralAxiom : TriangulatedCategory → Prop` — the octahedral axiom
/// states that for composable morphisms f : X → Y, g : Y → Z, the cones
/// fit into a distinguished triangle Cone(f) → Cone(g∘f) → Cone(g) → ΣCone(f).
pub fn octahedral_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("TriangulatedCategory"),
        prop(),
    )
}
/// `TStructureAxiom : TriangulatedCategory → Type` — a t-structure on a
/// triangulated category consists of full subcategories (D^≤0, D^≥0) satisfying
/// orthogonality, stability under shifts, and a truncation axiom.
pub fn t_structure_ty() -> Expr {
    arrow(cst("TriangulatedCategory"), type1())
}
/// `HeartOfTStructure : TStructureAxiom → Type` — the heart of a t-structure,
/// i.e. the abelian category D^≤0 ∩ D^≥0.
pub fn heart_of_t_structure_ty() -> Expr {
    arrow(cst("TStructureAxiom"), type1())
}
/// `TruncationFunctor : TStructureAxiom → Int → DerivedCat → DerivedCat`
/// — the truncation τ^≤n and τ^≥n functors associated to a t-structure.
pub fn truncation_functor_ty() -> Expr {
    arrow(
        cst("TStructureAxiom"),
        arrow(int_ty(), arrow(cst("DerivedCat"), cst("DerivedCat"))),
    )
}
/// `DGCategory : Type` — a category enriched over cochain complexes of k-modules.
pub fn dg_category_ty() -> Expr {
    type1()
}
/// `MaurerCartanElement : DGCategory → DGObj → Prop` — a Maurer-Cartan element
/// a ∈ Hom(X,X) of degree 1 satisfying d(a) + a·a = 0 in the DG-endomorphism
/// algebra.
pub fn maurer_cartan_element_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        cst("DGCategory"),
        pi(BinderInfo::Default, "X", app(cst("DGObj"), bvar(0)), prop()),
    )
}
/// `TwistedComplex : DGCategory → Type` — a twisted complex over a DG-category,
/// i.e. a sequence of objects with a degree-1 upper-triangular twisted differential.
pub fn twisted_complex_ty() -> Expr {
    arrow(cst("DGCategory"), type1())
}
/// `AInfinityCategory : Type` — an A∞-category with higher composition maps
/// μ^n : A⊗^n → A of degree 2-n satisfying the A∞-associativity equations.
pub fn a_infinity_category_ty() -> Expr {
    type1()
}
/// `AInfinityFunctor : AInfinityCategory → AInfinityCategory → Type` — a functor
/// between A∞-categories with components f^n satisfying the A∞-functor equations.
pub fn a_infinity_functor_ty() -> Expr {
    arrow(
        cst("AInfinityCategory"),
        arrow(cst("AInfinityCategory"), type1()),
    )
}
/// `YonedaEmbeddingDG : DGCategory → AInfinityCategory → Prop` — the DG Yoneda
/// embedding h : C → Mod(C) into the category of DG C-modules is a quasi-fully
/// faithful A∞-functor.
pub fn yoneda_embedding_dg_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        cst("DGCategory"),
        pi(BinderInfo::Default, "M", cst("AInfinityCategory"), prop()),
    )
}
/// `InfinityCategory : Type` — an (∞,1)-category modeled as a quasi-category
/// (a simplicial set satisfying inner horn filling conditions).
pub fn infinity_category_ty() -> Expr {
    type1()
}
/// `StableInfinityCategory : InfinityCategory → Prop` — a stable ∞-category
/// is pointed with finite limits and colimits where pushouts ↔ pullbacks.
pub fn stable_infinity_category_ty() -> Expr {
    arrow(cst("InfinityCategory"), prop())
}
/// `InfinityTopos : Type` — an ∞-topos in Lurie's sense: a presentable ∞-category
/// satisfying descent and object classifier axioms.
pub fn infinity_topos_ty() -> Expr {
    type1()
}
/// `PresentableInfinityCategory : InfinityCategory → Prop` — a presentable
/// ∞-category is accessible and cocomplete.
pub fn presentable_infinity_category_ty() -> Expr {
    arrow(cst("InfinityCategory"), prop())
}
/// `HomotopyCoherentDiagram : InfinityCategory → InfinityCategory → Type`
/// — a functor between ∞-categories (homotopy coherent diagram).
pub fn homotopy_coherent_diagram_ty() -> Expr {
    arrow(
        cst("InfinityCategory"),
        arrow(cst("InfinityCategory"), type1()),
    )
}
/// `HomotopyLimit : InfinityCategory → InfinityCategory → HomotopyCoherentDiagram → CatObj → Prop`
/// — the homotopy limit holim F of a diagram F : I → C.
pub fn homotopy_limit_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "I",
        cst("InfinityCategory"),
        pi(
            BinderInfo::Default,
            "C",
            cst("InfinityCategory"),
            pi(
                BinderInfo::Default,
                "F",
                app2(cst("HomotopyCoherentDiagram"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "X",
                    app(cst("CatObj"), bvar(1)),
                    prop(),
                ),
            ),
        ),
    )
}
/// `HomotopyColimit : InfinityCategory → InfinityCategory → HomotopyCoherentDiagram → CatObj → Prop`
/// — the homotopy colimit hocolim F, the ∞-categorical pushout/colimit.
pub fn homotopy_colimit_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "I",
        cst("InfinityCategory"),
        pi(
            BinderInfo::Default,
            "C",
            cst("InfinityCategory"),
            pi(
                BinderInfo::Default,
                "F",
                app2(cst("HomotopyCoherentDiagram"), bvar(1), bvar(0)),
                pi(
                    BinderInfo::Default,
                    "X",
                    app(cst("CatObj"), bvar(1)),
                    prop(),
                ),
            ),
        ),
    )
}
/// `DerivedHom : DerivedCat → DerivedCatObj → DerivedCatObj → ChainCx`
/// — the derived Hom complex RHom(X, Y) in a derived category.
pub fn derived_hom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        cst("DerivedCat"),
        pi(
            BinderInfo::Default,
            "X",
            app(cst("DerivedCatObj"), bvar(0)),
            pi(
                BinderInfo::Default,
                "Y",
                app(cst("DerivedCatObj"), bvar(1)),
                cst("ChainCx"),
            ),
        ),
    )
}
/// `DerivedTensor : DerivedCat → DerivedCatObj → DerivedCatObj → DerivedCatObj`
/// — the left derived tensor product X ⊗^L Y in a derived category of modules.
pub fn derived_tensor_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        cst("DerivedCat"),
        pi(
            BinderInfo::Default,
            "X",
            app(cst("DerivedCatObj"), bvar(0)),
            pi(
                BinderInfo::Default,
                "Y",
                app(cst("DerivedCatObj"), bvar(1)),
                app(cst("DerivedCatObj"), bvar(2)),
            ),
        ),
    )
}
/// `DerivedPushforward : SchemeMorphism → DerivedCatObj → DerivedCatObj`
/// — the right derived pushforward Rf_* along a morphism of schemes.
pub fn derived_pushforward_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("SchemeMorphism"),
        arrow(
            app(cst("DerivedCatObj"), cst("DerivedCat")),
            app(cst("DerivedCatObj"), cst("DerivedCat")),
        ),
    )
}
/// `DerivedPullback : SchemeMorphism → DerivedCatObj → DerivedCatObj`
/// — the derived pullback f^* (exact for flat morphisms, left derived in general).
pub fn derived_pullback_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("SchemeMorphism"),
        arrow(
            app(cst("DerivedCatObj"), cst("DerivedCat")),
            app(cst("DerivedCatObj"), cst("DerivedCat")),
        ),
    )
}
/// `PerverseSheaf : Scheme → Type` — a perverse sheaf on X, i.e. an object of
/// the derived category satisfying the middle-perversity support conditions.
pub fn perverse_sheaf_ty() -> Expr {
    arrow(cst("Scheme"), type1())
}
/// `MiddlePerversity : Scheme → Nat → Nat` — the middle perversity function
/// p̄(k) = floor((k-1)/2) used in the definition of perverse sheaves.
pub fn middle_perversity_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), nat_ty()))
}
/// `IntersectionCohomology : Scheme → PerverseSheaf → Nat → Module` — the
/// intersection cohomology groups IH^n(X; IC_X) with the IC sheaf.
pub fn intersection_cohomology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "IC",
            app(cst("PerverseSheaf"), bvar(0)),
            arrow(nat_ty(), cst("Module")),
        ),
    )
}
/// `DecompositionTheorem : SchemeMorphism → PerverseSheaf → Prop`
/// — Beilinson-Bernstein-Deligne-Gabber: Rf_* IC_X decomposes as a direct sum
/// of shifted IC sheaves on the target.
pub fn decomposition_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("SchemeMorphism"),
        pi(BinderInfo::Default, "IC", cst("PerverseSheaf"), prop()),
    )
}
/// `MixedHodgeStructure : Type` — a ℚ-vector space V with a Hodge filtration F
/// and a weight filtration W, compatible via Deligne's axioms.
pub fn mixed_hodge_structure_ty() -> Expr {
    type1()
}
/// `HodgeFiltration : MixedHodgeStructure → Nat → Module` — the Hodge
/// filtration F^p on a mixed Hodge structure.
pub fn hodge_filtration_ty() -> Expr {
    arrow(cst("MixedHodgeStructure"), arrow(nat_ty(), cst("Module")))
}
/// `WeightFiltration : MixedHodgeStructure → Int → Module` — the weight
/// filtration W_n on a mixed Hodge structure.
pub fn weight_filtration_ty() -> Expr {
    arrow(cst("MixedHodgeStructure"), arrow(int_ty(), cst("Module")))
}
/// `PolarizableMHS : MixedHodgeStructure → Prop` — a mixed Hodge structure is
/// polarizable if each graded piece Gr^W_n carries a polarization.
pub fn polarizable_mhs_ty() -> Expr {
    arrow(cst("MixedHodgeStructure"), prop())
}
/// `HodgeDecomposition : MixedHodgeStructure → Prop` — the Hodge decomposition
/// V_ℂ = ⊕_{p+q=n} H^{p,q} for pure weight-n Hodge structures.
pub fn hodge_decomposition_ty() -> Expr {
    arrow(cst("MixedHodgeStructure"), prop())
}
/// `MotivicCohomology : Scheme → Int → Int → Module` — the motivic cohomology
/// groups H^{n,m}(X, ℤ) = CH^m(X, 2m-n) (Bloch's higher Chow groups).
pub fn motivic_cohomology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(int_ty(), arrow(int_ty(), cst("Module"))),
    )
}
/// `AlgebraicKTheory : Scheme → Nat → Module` — the algebraic K-group K_n(X),
/// connected to motivic cohomology via the Atiyah-Hirzebruch spectral sequence.
pub fn algebraic_k_theory_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), cst("Module")))
}
/// `MilnorKTheory : Field → Nat → Module` — the Milnor K-theory group K^M_n(F),
/// generated by symbols {a_1, …, a_n} with a_i ∈ F*.
pub fn milnor_k_theory_ty() -> Expr {
    arrow(cst("Field"), arrow(nat_ty(), cst("Module")))
}
/// `BlochKatoConjecture : Field → Nat → Prop` — the norm-residue isomorphism
/// (Voevodsky's theorem): K^M_n(F)/ℓ ≅ H^n_et(F, μ_ℓ^⊗n).
pub fn bloch_kato_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("Field"),
        arrow(nat_ty(), prop()),
    )
}
/// `Hypercovering : Site → Scheme → Type` — a hypercovering U_• → X in a site
/// is a simplicial scheme satisfying hypercover conditions at each level.
pub fn hypercovering_ty() -> Expr {
    arrow(cst("Site"), arrow(cst("Scheme"), type1()))
}
/// `CohomologicalDescent : Site → Sheaf → Prop` — a sheaf F satisfies
/// cohomological descent if for every hypercovering U_• → X,
/// F(X) ≅ holim F(U_•).
pub fn cohomological_descent_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        cst("Site"),
        pi(BinderInfo::Default, "F", cst("Sheaf"), prop()),
    )
}
/// `DualizingComplex : Scheme → DerivedCatObj` — the dualizing complex ω_X^•
/// in the derived category, generalizing the dualizing sheaf of a Cohen-Macaulay scheme.
pub fn dualizing_complex_ty() -> Expr {
    arrow(cst("Scheme"), app(cst("DerivedCatObj"), cst("DerivedCat")))
}
/// `VerdierDuality : Scheme → Prop` — the Verdier duality functor
/// D_X = RHom(–, ω_X^•) is a contravariant involution on D^b_c(X).
pub fn verdier_duality_ty() -> Expr {
    pi(BinderInfo::Default, "X", cst("Scheme"), prop())
}
/// `SixFunctors : SchemeMorphism → Prop` — Grothendieck's six-functor formalism:
/// (f*, f_*, f!, f^!, ⊗^L, RHom) satisfying the full suite of adjunctions and
/// base change formulas.
pub fn six_functors_ty() -> Expr {
    arrow(cst("SchemeMorphism"), prop())
}
/// `LocalCohomology : Scheme → ClosedSubscheme → Nat → Sheaf → Module`
/// — the local cohomology groups H^n_Z(X, F) supported on a closed subscheme Z ⊆ X.
pub fn local_cohomology_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Z",
            cst("ClosedSubscheme"),
            pi(
                BinderInfo::Default,
                "F",
                cst("Sheaf"),
                arrow(nat_ty(), cst("Module")),
            ),
        ),
    )
}
/// `LocalDuality : Ring → Module → Nat → Prop` — Grothendieck's local duality:
/// H^n_m(M) ≅ Hom(Ext^{d-n}(M,R), E(k)) where E(k) is the injective hull of k.
pub fn local_duality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "R",
        cst("Ring"),
        pi(
            BinderInfo::Default,
            "M",
            cst("Module"),
            arrow(nat_ty(), prop()),
        ),
    )
}
/// `DeRhamCohomology : SmoothScheme → Nat → Module` — the algebraic de Rham
/// cohomology H^n_dR(X/k) = H^n(X, Ω^•_{X/k}).
pub fn de_rham_cohomology_ty() -> Expr {
    arrow(cst("SmoothScheme"), arrow(nat_ty(), cst("Module")))
}
/// `DeRhamComparison : SmoothScheme → Nat → Prop` — the de Rham comparison
/// isomorphism H^n_dR(X/ℂ) ≅ H^n_Betti(X^an, ℂ) for smooth complex varieties.
pub fn de_rham_comparison_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("SmoothScheme"),
        arrow(nat_ty(), prop()),
    )
}
/// `GaussManinConnection : FamilyOfSchemes → DeRhamBundle → Prop` — the
/// Gauss-Manin connection ∇ on the relative de Rham cohomology bundle of a family.
pub fn gauss_manin_connection_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("FamilyOfSchemes"),
        pi(BinderInfo::Default, "E", cst("DeRhamBundle"), prop()),
    )
}
/// `CrystallineCohomology : SmoothScheme → Nat → Module` — the crystalline
/// cohomology H^n_cris(X/W(k)) for X/k smooth over a perfect field of char p.
pub fn crystalline_cohomology_ty() -> Expr {
    arrow(cst("SmoothScheme"), arrow(nat_ty(), cst("Module")))
}
/// `FCrystal : SmoothScheme → Type` — an F-crystal on X, i.e. a locally free
/// crystal of O_{X/W}-modules equipped with a Frobenius-linear endomorphism φ.
pub fn f_crystal_ty() -> Expr {
    arrow(cst("SmoothScheme"), type1())
}
/// `DieudonneModule : FormalGroup → Type` — the Dieudonné module M(G) of a
/// formal group G, a finitely generated W(k)-module with F and V operators.
pub fn dieudonne_module_ty() -> Expr {
    arrow(cst("FormalGroup"), type1())
}
/// `EtaleCohomology : Scheme → Nat → Module` — the ℓ-adic étale cohomology
/// H^n_et(X, ℤ_ℓ) for ℓ ≠ char(k).
pub fn etale_cohomology_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), cst("Module")))
}
/// `WeilConjectures : Scheme → Prop` — the Weil conjectures for a smooth
/// projective variety X/𝔽_q: rationality, functional equation, Riemann hypothesis.
pub fn weil_conjectures_ty() -> Expr {
    arrow(cst("Scheme"), prop())
}
/// `PurityTheorem : Scheme → ClosedSubscheme → Nat → Prop` — the purity theorem:
/// for a smooth closed subscheme Z of codimension c in X,
/// H^n_Z(X, ℤ_ℓ) ≅ H^{n-2c}_et(Z, ℤ_ℓ)(-c).
pub fn purity_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Z",
            cst("ClosedSubscheme"),
            arrow(nat_ty(), prop()),
        ),
    )
}
/// `ProperBaseChange : SchemeMorphism → Sheaf → Prop` — the proper base change
/// theorem: for proper f, the formation of Rf_* commutes with arbitrary base change.
pub fn proper_base_change_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        cst("SchemeMorphism"),
        arrow(cst("Sheaf"), prop()),
    )
}
/// `ChowGroup : Scheme → Nat → Module` — the Chow group CH^p(X) of algebraic
/// cycles of codimension p modulo rational equivalence.
pub fn chow_group_ty() -> Expr {
    arrow(cst("Scheme"), arrow(nat_ty(), cst("Module")))
}
/// `BlochsFormula : SmoothScheme → Nat → Prop` — Bloch's formula:
/// CH^p(X) ≅ H^p(X, K_p) where K_p is the Zariski sheaf of Milnor K-theory.
pub fn blochs_formula_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("SmoothScheme"),
        arrow(nat_ty(), prop()),
    )
}
/// `HigherChowGroup : Scheme → Nat → Nat → Module` — Bloch's higher Chow groups
/// CH^p(X, n) = motivic cohomology H^{2p-n}(X, ℤ(p)).
pub fn higher_chow_group_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(nat_ty(), arrow(nat_ty(), cst("Module"))),
    )
}
/// `CycleClassMap : Scheme → Nat → Prop` — the cycle class map
/// cl : CH^p(X) → H^{2p}_et(X, ℤ_ℓ(p)) to étale cohomology.
pub fn cycle_class_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        arrow(nat_ty(), prop()),
    )
}
/// `HodgeConjecture : SmoothProjectiveScheme → Nat → Prop` — the Hodge
/// conjecture: every rational (p,p)-class in H^{2p}_Betti(X, ℚ) is algebraic.
pub fn hodge_conjecture_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("SmoothProjectiveScheme"),
        arrow(nat_ty(), prop()),
    )
}
/// `GrothendieckStandardConjectures : Scheme → Prop` — Grothendieck's standard
/// conjectures (Lefschetz, Künneth, homological = numerical equivalence).
pub fn grothendieck_standard_conjectures_ty() -> Expr {
    arrow(cst("Scheme"), prop())
}
/// Register all new advanced homological algebra axioms into an environment.
pub fn register_advanced_homological_axioms(env: &mut Environment) {
    let new_base_types: &[(&str, fn() -> Expr)] = &[
        ("TriangulatedCategory", triangulated_category_ty),
        ("DerivedCat", || type1()),
        ("TStructureAxiom", || {
            arrow(cst("TriangulatedCategory"), type1())
        }),
        ("DGCategory", dg_category_ty),
        ("DGObj", || arrow(cst("DGCategory"), type1())),
        ("AInfinityCategory", a_infinity_category_ty),
        ("InfinityCategory", infinity_category_ty),
        ("CatObj", || arrow(cst("InfinityCategory"), type1())),
        ("DerivedCatObj", || arrow(cst("DerivedCat"), type1())),
        ("ChainCx", || type1()),
        ("SchemeMorphism", || type1()),
        ("Scheme", || type1()),
        ("SmoothScheme", || type1()),
        ("SmoothProjectiveScheme", || type1()),
        ("Site", || type1()),
        ("Sheaf", || type1()),
        ("Ring", || type1()),
        ("Field", || type1()),
        ("FormalGroup", || type1()),
        ("ClosedSubscheme", || type1()),
        ("FamilyOfSchemes", || type1()),
        ("DeRhamBundle", || type1()),
        ("PerverseSheaf", || arrow(cst("Scheme"), type1())),
        ("MixedHodgeStructure", mixed_hodge_structure_ty),
    ];
    for (name, mk_ty) in new_base_types {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let new_axioms: &[(&str, fn() -> Expr)] = &[
        ("DistinguishedTriangle", distinguished_triangle_ty),
        ("OctahedralAxiom", octahedral_axiom_ty),
        ("HeartOfTStructure", heart_of_t_structure_ty),
        ("TruncationFunctor", truncation_functor_ty),
        ("MaurerCartanElement", maurer_cartan_element_ty),
        ("TwistedComplex", twisted_complex_ty),
        ("AInfinityFunctor", a_infinity_functor_ty),
        ("YonedaEmbeddingDG", yoneda_embedding_dg_ty),
        ("StableInfinityCategory", stable_infinity_category_ty),
        ("InfinityTopos", infinity_topos_ty),
        (
            "PresentableInfinityCategory",
            presentable_infinity_category_ty,
        ),
        ("HomotopyCoherentDiagram", homotopy_coherent_diagram_ty),
        ("HomotopyLimit", homotopy_limit_ty),
        ("HomotopyColimit", homotopy_colimit_ty),
        ("DerivedHom", derived_hom_ty),
        ("DerivedTensor", derived_tensor_ty),
        ("DerivedPushforward", derived_pushforward_ty),
        ("DerivedPullback", derived_pullback_ty),
        ("MiddlePerversity", middle_perversity_ty),
        ("IntersectionCohomology", intersection_cohomology_ty),
        ("DecompositionTheorem", decomposition_theorem_ty),
        ("HodgeFiltration", hodge_filtration_ty),
        ("WeightFiltration", weight_filtration_ty),
        ("PolarizableMHS", polarizable_mhs_ty),
        ("HodgeDecomposition", hodge_decomposition_ty),
        ("MotivicCohomology", motivic_cohomology_ty),
        ("AlgebraicKTheory", algebraic_k_theory_ty),
        ("MilnorKTheory", milnor_k_theory_ty),
        ("BlochKatoConjecture", bloch_kato_conjecture_ty),
        ("Hypercovering", hypercovering_ty),
        ("CohomologicalDescent", cohomological_descent_ty),
        ("DualizingComplex", dualizing_complex_ty),
        ("VerdierDuality", verdier_duality_ty),
        ("SixFunctors", six_functors_ty),
        ("LocalCohomology", local_cohomology_ty),
        ("LocalDuality", local_duality_ty),
        ("DeRhamCohomology", de_rham_cohomology_ty),
        ("DeRhamComparison", de_rham_comparison_ty),
        ("GaussManinConnection", gauss_manin_connection_ty),
        ("CrystallineCohomology", crystalline_cohomology_ty),
        ("FCrystal", f_crystal_ty),
        ("DieudonneModule", dieudonne_module_ty),
        ("EtaleCohomology", etale_cohomology_ty),
        ("WeilConjectures", weil_conjectures_ty),
        ("PurityTheorem", purity_theorem_ty),
        ("ProperBaseChange", proper_base_change_ty),
        ("ChowGroup", chow_group_ty),
        ("BlochsFormula", blochs_formula_ty),
        ("HigherChowGroup", higher_chow_group_ty),
        ("CycleClassMap", cycle_class_map_ty),
        ("HodgeConjecture", hodge_conjecture_ty),
        (
            "GrothendieckStandardConjectures",
            grothendieck_standard_conjectures_ty,
        ),
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
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chain_group_display() {
        let g = ChainGroup::new(3, "C_2");
        assert!(g.to_string().contains("C_2"));
        assert!(g.to_string().contains("3"));
    }
    #[test]
    fn test_chain_complex_trivial_betti() {
        let mut c = ChainComplex::new();
        c.add_group(1, "C_0");
        let betti = c.compute_betti_numbers();
        assert_eq!(betti, vec![1]);
    }
    #[test]
    fn test_chain_complex_euler_characteristic_circle() {
        let mut c = ChainComplex::new();
        c.add_group(3, "C_0");
        c.add_group(3, "C_1");
        c.add_boundary(vec![vec![-1, 0, -1], vec![1, -1, 0], vec![0, 1, 1]]);
        let chi = c.euler_characteristic();
        assert_eq!(chi, 0);
    }
    #[test]
    fn test_chain_complex_is_exact_at_acyclic() {
        let mut c = ChainComplex::new();
        c.add_group(1, "C_0");
        c.add_group(1, "C_1");
        c.add_boundary(vec![vec![1]]);
        assert!(c.is_exact_at(0));
    }
    #[test]
    fn test_ext_group_is_zero() {
        let e0 = ExtGroup::new("Z", "Z", 0, 1);
        let e1 = ExtGroup::new("Z", "Z", 1, 0);
        assert!(!e0.is_zero());
        assert!(e1.is_zero());
    }
    #[test]
    fn test_ext_group_display() {
        let e = ExtGroup::new("Z/2Z", "Z", 1, 1);
        let s = e.to_string();
        assert!(s.contains("Ext^1"));
        assert!(s.contains("Z/2Z"));
        assert!(s.contains("rank=1"));
    }
    #[test]
    fn test_spectral_sequence_e_term() {
        let mut ss = SpectralSequence::new();
        let mut page0: HashMap<(i32, i32), usize> = HashMap::new();
        page0.insert((0, 0), 1);
        page0.insert((1, 0), 2);
        ss.add_page(page0);
        assert_eq!(ss.e_term(0, 0, 0), Some(1));
        assert_eq!(ss.e_term(0, 1, 0), Some(2));
        assert_eq!(ss.e_term(0, 2, 0), None);
    }
    #[test]
    fn test_build_homological_algebra_env() {
        let mut env = Environment::new();
        build_homological_algebra_env(&mut env);
        assert!(env.get(&Name::str("Homology")).is_some());
        assert!(env.get(&Name::str("Ext")).is_some());
        assert!(env.get(&Name::str("Tor")).is_some());
        assert!(env.get(&Name::str("snake_lemma")).is_some());
        assert!(env.get(&Name::str("five_lemma")).is_some());
    }
    #[test]
    fn test_register_advanced_axioms_env() {
        let mut env = Environment::new();
        register_advanced_homological_axioms(&mut env);
        assert!(env.get(&Name::str("TriangulatedCategory")).is_some());
        assert!(env.get(&Name::str("OctahedralAxiom")).is_some());
        assert!(env.get(&Name::str("DGCategory")).is_some());
        assert!(env.get(&Name::str("AInfinityCategory")).is_some());
        assert!(env.get(&Name::str("InfinityCategory")).is_some());
        assert!(env.get(&Name::str("HomotopyLimit")).is_some());
        assert!(env.get(&Name::str("HomotopyColimit")).is_some());
        assert!(env.get(&Name::str("VerdierDuality")).is_some());
        assert!(env.get(&Name::str("WeilConjectures")).is_some());
        assert!(env.get(&Name::str("ChowGroup")).is_some());
        assert!(env.get(&Name::str("HodgeConjecture")).is_some());
    }
    #[test]
    fn test_triangulated_category_data_basic() {
        let mut t = TriangulatedCategoryData::new();
        let x = t.add_object("X");
        let y = t.add_object("Y");
        let z = t.add_object("Z");
        t.add_triangle(x, y, z);
        assert!(t.is_distinguished(x, y, z));
        assert!(!t.is_distinguished(y, x, z));
        assert_eq!(t.triangle_count(), 1);
    }
    #[test]
    fn test_triangulated_octahedral_check() {
        let mut t = TriangulatedCategoryData::new();
        let cone_xy = t.add_object("Cone(f)");
        let cone_xz = t.add_object("Cone(gf)");
        let cone_yz = t.add_object("Cone(g)");
        t.add_triangle(cone_xy, cone_xz, cone_yz);
        assert!(t.check_octahedral(cone_xy, cone_xz, cone_yz));
        assert!(!t.check_octahedral(cone_yz, cone_xy, cone_xz));
    }
    #[test]
    fn test_dg_category_ext_degree() {
        let mut dg = DGCategoryData::new();
        let x = dg.add_object("X");
        let y = dg.add_object("Y");
        let mut hom = ChainComplex::new();
        hom.add_group(1, "Hom0");
        hom.add_group(1, "Hom1");
        hom.add_boundary(vec![vec![1]]);
        dg.set_hom(x, y, hom);
        assert_eq!(dg.ext_degree(x, y, 0), Some(0));
        assert!(dg.are_quasi_isomorphic(x, y));
    }
    #[test]
    fn test_mixed_hodge_structure_hodge_numbers() {
        let mut mhs = MixedHodgeStructureData::new(2);
        mhs.set_hodge_number(2, 0, 1);
        mhs.set_hodge_number(1, 1, 20);
        mhs.set_hodge_number(0, 2, 1);
        assert_eq!(mhs.hodge_number(2, 0), 1);
        assert_eq!(mhs.hodge_number(0, 2), 1);
        assert_eq!(mhs.hodge_number(1, 1), 20);
        assert!(mhs.satisfies_hodge_symmetry());
        assert!(mhs.is_pure());
        assert_eq!(mhs.total_dimension(), 22);
    }
    #[test]
    fn test_mixed_hodge_structure_not_pure() {
        let mut mhs = MixedHodgeStructureData::new(2);
        mhs.set_hodge_number(1, 0, 3);
        assert!(!mhs.is_pure());
    }
    #[test]
    fn test_chow_group_degree_and_zero() {
        let mut ch = ChowGroupData::new("P1", 1);
        assert!(ch.is_zero());
        ch.add_cycle("pt", 3);
        ch.add_cycle("pt", -3);
        assert!(ch.is_zero());
        ch.add_cycle("line", 2);
        assert_eq!(ch.degree(), 2);
    }
    #[test]
    fn test_chow_group_intersection_number() {
        let mut ch1 = ChowGroupData::new("P2", 2);
        ch1.add_cycle("p", 3);
        let mut ch2 = ChowGroupData::new("P2", 2);
        ch2.add_cycle("q", 2);
        assert_eq!(ch1.intersection_number(&ch2), 6);
    }
    #[test]
    fn test_spectral_sequence_page_next() {
        let mut page = SpectralSequencePage::new(2);
        page.set_group(0, 2, 3);
        page.set_group(2, 1, 2);
        page.add_differential(2, 1, 1);
        let next = page.compute_next_page();
        assert_eq!(next.get(&(2, 1)).copied().unwrap_or(0), 1);
        assert_eq!(next.get(&(0, 2)).copied().unwrap_or(0), 2);
    }
    #[test]
    fn test_spectral_sequence_page_degenerate() {
        let mut page = SpectralSequencePage::new(3);
        page.set_group(0, 0, 5);
        assert!(page.is_degenerate());
        page.add_differential(0, 0, 1);
        assert!(!page.is_degenerate());
    }
}
/// `BoundedDerivedCategory : AbelianCat → Type` — the bounded derived category
/// D^b(A) of an abelian category A, consisting of bounded chain complexes
/// up to quasi-isomorphism.
pub fn ha_ext_bounded_derived_category_ty() -> Expr {
    arrow(cst("AbelianCat"), type1())
}
/// `DerivedEquivalence : AbelianCat → AbelianCat → Prop` — two abelian
/// categories are derived-equivalent if their bounded derived categories
/// are equivalent as triangulated categories.
pub fn ha_ext_derived_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("AbelianCat"),
        pi(BinderInfo::Default, "B", cst("AbelianCat"), prop()),
    )
}
/// `ShiftFunctor : TriangulatedCategory → Int → TriangulatedCategory`
/// — the translation functor T = [1] (suspension) on a triangulated category,
/// with T^n = [n] for n ∈ ℤ.
pub fn ha_ext_shift_functor_ty() -> Expr {
    arrow(
        cst("TriangulatedCategory"),
        arrow(int_ty(), cst("TriangulatedCategory")),
    )
}
/// `ExactFunctor : TriangulatedCategory → TriangulatedCategory → Type`
/// — a functor between triangulated categories that preserves distinguished
/// triangles and commutes with the shift functor.
pub fn ha_ext_exact_functor_ty() -> Expr {
    arrow(
        cst("TriangulatedCategory"),
        arrow(cst("TriangulatedCategory"), type1()),
    )
}
/// `RotationAxiom : TriangulatedCategory → Prop` — the rotation axiom:
/// if X → Y → Z → ΣX is distinguished, so is Y → Z → ΣX → ΣY.
pub fn ha_ext_rotation_axiom_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        cst("TriangulatedCategory"),
        prop(),
    )
}
/// `StabilityCondition : TriangulatedCategory → Type` — a Bridgeland stability
/// condition on D^b(X) consists of a heart A ⊆ D^b(X) and a central charge
/// Z : K(A) → ℂ satisfying the Harder-Narasimhan property.
pub fn ha_ext_stability_condition_ty() -> Expr {
    arrow(cst("TriangulatedCategory"), type1())
}
/// `CentralCharge : StabilityCondition → Module → Complex` — the central charge
/// Z : K(A) → ℂ mapping the Grothendieck group to the complex numbers.
pub fn ha_ext_central_charge_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "σ",
        cst("StabilityCondition"),
        arrow(cst("Module"), cst("Complex")),
    )
}
/// `HarderNarasimhanFiltration : StabilityCondition → Module → Type`
/// — the Harder-Narasimhan filtration of an object E with respect to a
/// stability condition σ: a filtration 0 = E_0 ⊆ … ⊆ E_n = E whose
/// graded pieces are σ-semistable.
pub fn ha_ext_harder_narasimhan_filtration_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "σ",
        cst("StabilityCondition"),
        arrow(cst("Module"), type1()),
    )
}
/// `SemistableObject : StabilityCondition → Module → Prop` — an object E
/// is σ-semistable if every destabilising subobject has larger or equal phase.
pub fn ha_ext_semistable_object_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "σ",
        cst("StabilityCondition"),
        arrow(cst("Module"), prop()),
    )
}
/// `WallCrossing : TriangulatedCategory → Prop` — as the stability condition
/// crosses a wall in Stab(D), the set of semistable objects changes and
/// moduli spaces undergo a birational transformation.
pub fn ha_ext_wall_crossing_ty() -> Expr {
    arrow(cst("TriangulatedCategory"), prop())
}
/// `FourierMukaiKernel : Scheme → Scheme → DerivedCatObj` — an object P ∈ D^b(X×Y)
/// serving as the kernel of a Fourier-Mukai transform Φ_P : D^b(X) → D^b(Y).
pub fn ha_ext_fourier_mukai_kernel_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("Scheme"),
            app(cst("DerivedCatObj"), cst("DerivedCat")),
        ),
    )
}
/// `FourierMukaiTransform : Scheme → Scheme → DerivedCatObj → ExactFunctor`
/// — Φ_P(−) = Rπ_{Y,*}(P ⊗^L Lπ_X^*(−)) for a kernel P ∈ D^b(X×Y).
pub fn ha_ext_fourier_mukai_transform_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("Scheme"),
            pi(
                BinderInfo::Default,
                "P",
                app2(cst("FourierMukaiKernel"), bvar(1), bvar(0)),
                cst("ExactFunctor"),
            ),
        ),
    )
}
/// `BondalOrlovReconstruction : Scheme → Prop` — the Bondal-Orlov reconstruction
/// theorem: if X has ample (anti-)canonical bundle, then D^b(X) ≅ D^b(Y) implies X ≅ Y.
pub fn ha_ext_bondal_orlov_reconstruction_ty() -> Expr {
    pi(BinderInfo::Default, "X", cst("Scheme"), prop())
}
/// `MukaiLattice : K3Surface → Module` — the Mukai lattice H̃(X, ℤ) of a K3
/// surface X, the total cohomology with the Mukai pairing.
pub fn ha_ext_mukai_lattice_ty() -> Expr {
    arrow(cst("K3Surface"), cst("Module"))
}
/// `FourierMukaiIsEquivalence : Scheme → Scheme → DerivedCatObj → Prop`
/// — Φ_P is an equivalence iff the kernel satisfies the point-object conditions.
pub fn ha_ext_fourier_mukai_is_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("Scheme"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("Scheme"),
            pi(
                BinderInfo::Default,
                "P",
                app2(cst("FourierMukaiKernel"), bvar(1), bvar(0)),
                prop(),
            ),
        ),
    )
}
/// `SerreFunctor : TriangulatedCategory → Type` — a Serre functor S on a
/// k-linear triangulated category D: a self-equivalence with natural isomorphisms
/// Hom(X, Y) ≅ Hom(Y, S(X))^∨.
pub fn ha_ext_serre_functor_ty() -> Expr {
    arrow(cst("TriangulatedCategory"), type1())
}
/// `SerreIsomorphism : TriangulatedCategory → Module → Module → Prop`
/// — Hom_D(X, Y) ≅ Hom_D(Y, S(X))^* (the Serre duality isomorphism).
pub fn ha_ext_serre_isomorphism_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        cst("TriangulatedCategory"),
        pi(
            BinderInfo::Default,
            "X",
            cst("Module"),
            pi(BinderInfo::Default, "Y", cst("Module"), prop()),
        ),
    )
}
/// `CalabiyauCategory : TriangulatedCategory → Nat → Prop` — a triangulated
/// category D is n-Calabi-Yau if the Serre functor S ≅ [n] (shift by n).
pub fn ha_ext_calabi_yau_category_ty() -> Expr {
    arrow(cst("TriangulatedCategory"), arrow(nat_ty(), prop()))
}
/// `CalabiyauDimension : TriangulatedCategory → Int` — the Calabi-Yau dimension
/// of a triangulated category: the unique n such that S ≅ [n].
pub fn ha_ext_calabi_yau_dimension_ty() -> Expr {
    arrow(cst("TriangulatedCategory"), int_ty())
}
/// `SphericalObject : TriangulatedCategory → Module → Prop` — an object E in
/// a triangulated category is spherical if Ext^*(E, E) ≅ H^*(S^n; k) for some n.
pub fn ha_ext_spherical_object_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        cst("TriangulatedCategory"),
        arrow(cst("Module"), prop()),
    )
}
/// `SphericalTwist : TriangulatedCategory → Module → ExactFunctor` — the
/// spherical twist T_E associated to a spherical object E: the autoequivalence
/// given by the exact triangle E ⊗ RHom(E,−) → id → T_E.
pub fn ha_ext_spherical_twist_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        cst("TriangulatedCategory"),
        arrow(cst("Module"), cst("ExactFunctor")),
    )
}
/// `ModelCategory : Type` — a model category is a category with three
/// distinguished classes of morphisms (weak equivalences, fibrations, cofibrations)
/// satisfying Quillen's five axioms (MC1–MC5).
pub fn ha_ext_model_category_ty() -> Expr {
    type1()
}
/// `WeakEquivalence : ModelCategory → Morphism → Prop` — a morphism that becomes
/// an isomorphism in the homotopy category Ho(M).
pub fn ha_ext_weak_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("ModelCategory"),
        arrow(cst("Morphism"), prop()),
    )
}
/// `Fibration : ModelCategory → Morphism → Prop` — a fibration in a model category:
/// the right class in the (cofibration, acyclic fibration) factorisation.
pub fn ha_ext_fibration_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("ModelCategory"),
        arrow(cst("Morphism"), prop()),
    )
}
/// `Cofibration : ModelCategory → Morphism → Prop` — a cofibration in a model
/// category: the left class in the (cofibration, acyclic fibration) factorisation.
pub fn ha_ext_cofibration_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("ModelCategory"),
        arrow(cst("Morphism"), prop()),
    )
}
/// `CofibrantObject : ModelCategory → Obj → Prop` — an object X is cofibrant
/// if the map from the initial object ∅ → X is a cofibration.
pub fn ha_ext_cofibrant_object_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("ModelCategory"),
        arrow(cst("Obj"), prop()),
    )
}
/// `FibrantObject : ModelCategory → Obj → Prop` — an object X is fibrant if
/// the map X → * to the terminal object is a fibration.
pub fn ha_ext_fibrant_object_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("ModelCategory"),
        arrow(cst("Obj"), prop()),
    )
}
/// `CofibrantReplacement : ModelCategory → Obj → Obj → Prop` — a cofibrant
/// replacement Q(X) ↠ X is a cofibrant object with a weak equivalence to X.
pub fn ha_ext_cofibrant_replacement_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("ModelCategory"),
        pi(
            BinderInfo::Default,
            "X",
            cst("Obj"),
            arrow(cst("Obj"), prop()),
        ),
    )
}
/// `FibrantReplacement : ModelCategory → Obj → Obj → Prop` — a fibrant
/// replacement X → R(X) is a fibrant object with a weak equivalence from X.
pub fn ha_ext_fibrant_replacement_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("ModelCategory"),
        pi(
            BinderInfo::Default,
            "X",
            cst("Obj"),
            arrow(cst("Obj"), prop()),
        ),
    )
}
/// `HomotopyCategory : ModelCategory → Type` — the homotopy category Ho(M)
/// obtained by inverting all weak equivalences in M.
pub fn ha_ext_homotopy_category_ty() -> Expr {
    arrow(cst("ModelCategory"), type1())
}
/// `QuillenAdjunction : ModelCategory → ModelCategory → Type` — a Quillen
/// adjunction (L ⊣ R) where L preserves cofibrations and R preserves fibrations.
pub fn ha_ext_quillen_adjunction_ty() -> Expr {
    arrow(cst("ModelCategory"), arrow(cst("ModelCategory"), type1()))
}
/// `QuillenEquivalence : ModelCategory → ModelCategory → Prop` — a Quillen
/// adjunction that induces an equivalence of homotopy categories.
pub fn ha_ext_quillen_equivalence_ty() -> Expr {
    arrow(cst("ModelCategory"), arrow(cst("ModelCategory"), prop()))
}
/// `BousfieldLocalization : ModelCategory → Class → ModelCategory`
/// — the left Bousfield localization L_C(M) of a model category M with respect
/// to a class C of morphisms: new model structure with the same cofibrations
/// but more weak equivalences.
pub fn ha_ext_bousfield_localization_ty() -> Expr {
    arrow(
        cst("ModelCategory"),
        arrow(cst("Class"), cst("ModelCategory")),
    )
}
/// `LocalObject : BousfieldLocalization → Obj → Prop` — an object X is C-local
/// if Map(f, X) is a weak equivalence for every f ∈ C.
pub fn ha_ext_local_object_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("BousfieldLocalization"),
        arrow(cst("Obj"), prop()),
    )
}
/// `LocalEquivalence : ModelCategory → Class → Morphism → Prop` — a morphism
/// f is a C-local equivalence if Map(f, X) is a weak eq. for every C-local X.
pub fn ha_ext_local_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("ModelCategory"),
        pi(
            BinderInfo::Default,
            "C",
            cst("Class"),
            arrow(cst("Morphism"), prop()),
        ),
    )
}
/// `SmashingLocalization : ModelCategory → Prop` — a Bousfield localization is
/// smashing if the localization functor L commutes with filtered hocolimits.
pub fn ha_ext_smashing_localization_ty() -> Expr {
    arrow(cst("ModelCategory"), prop())
}
/// `TiltingObject : AbelianCat → Module → Prop` — a tilting object T ∈ A
/// generates D^b(A) and has vanishing self-Ext^n for n > 0.
pub fn ha_ext_tilting_object_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("AbelianCat"),
        arrow(cst("Module"), prop()),
    )
}
/// `TiltingEquivalence : AbelianCat → AbelianCat → Module → Prop` — the
/// Beilinson-Bernstein-Tilting equivalence: if T is a tilting A-module,
/// then D^b(A) ≅ D^b(End_A(T)-mod).
pub fn ha_ext_tilting_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("AbelianCat"),
        pi(
            BinderInfo::Default,
            "B",
            cst("AbelianCat"),
            arrow(cst("Module"), prop()),
        ),
    )
}
/// `MutationOfTilting : AbelianCat → Module → Nat → Module` — left/right
/// mutation of a tilting object at an indecomposable summand.
pub fn ha_ext_mutation_of_tilting_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        cst("AbelianCat"),
        arrow(cst("Module"), arrow(nat_ty(), cst("Module"))),
    )
}
/// `SemiorthogonalDecomposition : TriangulatedCategory → Type` — a
/// semi-orthogonal decomposition D = ⟨A_1, …, A_n⟩ where each A_i is
/// an admissible subcategory and Hom(A_j, A_i) = 0 for j > i.
pub fn ha_ext_semiorthogonal_decomposition_ty() -> Expr {
    arrow(cst("TriangulatedCategory"), type1())
}
/// `ExceptionalCollection : TriangulatedCategory → Type` — a sequence of
/// exceptional objects E_1, …, E_n with Hom^*(E_i, E_j) = 0 for i > j
/// and Hom^*(E_i, E_i) = k.
pub fn ha_ext_exceptional_collection_ty() -> Expr {
    arrow(cst("TriangulatedCategory"), type1())
}
/// Register all extended homological algebra axioms (Section 7) into an environment.
pub fn register_homological_algebra_extended(env: &mut Environment) -> Result<(), String> {
    let base_types: &[(&str, fn() -> Expr)] = &[
        ("AbelianCat", || type1()),
        ("K3Surface", || type1()),
        ("Complex", || type1()),
        ("Morphism", || type1()),
        ("Obj", || type1()),
        ("Class", || type1()),
        ("ModelCategory", || type1()),
        ("BousfieldLocalization", || {
            arrow(
                cst("ModelCategory"),
                arrow(cst("Class"), cst("ModelCategory")),
            )
        }),
        ("ExactFunctor", || {
            arrow(
                cst("TriangulatedCategory"),
                arrow(cst("TriangulatedCategory"), type1()),
            )
        }),
        ("StabilityCondition", ha_ext_stability_condition_ty),
        ("FourierMukaiKernel", ha_ext_fourier_mukai_kernel_ty),
    ];
    for (name, mk_ty) in base_types {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("BoundedDerivedCategory", ha_ext_bounded_derived_category_ty),
        ("DerivedEquivalence", ha_ext_derived_equivalence_ty),
        ("ShiftFunctor", ha_ext_shift_functor_ty),
        ("RotationAxiom", ha_ext_rotation_axiom_ty),
        ("CentralCharge", ha_ext_central_charge_ty),
        (
            "HarderNarasimhanFiltration",
            ha_ext_harder_narasimhan_filtration_ty,
        ),
        ("SemistableObject", ha_ext_semistable_object_ty),
        ("WallCrossing", ha_ext_wall_crossing_ty),
        ("FourierMukaiTransform", ha_ext_fourier_mukai_transform_ty),
        (
            "BondalOrlovReconstruction",
            ha_ext_bondal_orlov_reconstruction_ty,
        ),
        ("MukaiLattice", ha_ext_mukai_lattice_ty),
        (
            "FourierMukaiIsEquivalence",
            ha_ext_fourier_mukai_is_equivalence_ty,
        ),
        ("SerreFunctor", ha_ext_serre_functor_ty),
        ("SerreIsomorphism", ha_ext_serre_isomorphism_ty),
        ("CalabiyauCategory", ha_ext_calabi_yau_category_ty),
        ("CalabiyauDimension", ha_ext_calabi_yau_dimension_ty),
        ("SphericalObject", ha_ext_spherical_object_ty),
        ("SphericalTwist", ha_ext_spherical_twist_ty),
        ("WeakEquivalence", ha_ext_weak_equivalence_ty),
        ("Fibration", ha_ext_fibration_ty),
        ("Cofibration", ha_ext_cofibration_ty),
        ("CofibrantObject", ha_ext_cofibrant_object_ty),
        ("FibrantObject", ha_ext_fibrant_object_ty),
        ("CofibrantReplacement", ha_ext_cofibrant_replacement_ty),
        ("FibrantReplacement", ha_ext_fibrant_replacement_ty),
        ("HomotopyCategory", ha_ext_homotopy_category_ty),
        ("QuillenAdjunction", ha_ext_quillen_adjunction_ty),
        ("QuillenEquivalence", ha_ext_quillen_equivalence_ty),
        ("LocalObject", ha_ext_local_object_ty),
        ("LocalEquivalence", ha_ext_local_equivalence_ty),
        ("SmashingLocalization", ha_ext_smashing_localization_ty),
        ("TiltingObject", ha_ext_tilting_object_ty),
        ("TiltingEquivalence", ha_ext_tilting_equivalence_ty),
        ("MutationOfTilting", ha_ext_mutation_of_tilting_ty),
        (
            "SemiorthogonalDecomposition",
            ha_ext_semiorthogonal_decomposition_ty,
        ),
        ("ExceptionalCollection", ha_ext_exceptional_collection_ty),
    ];
    for (name, mk_ty) in axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .map_err(|e| format!("Failed to add '{}': {:?}", name, e))?;
    }
    Ok(())
}
