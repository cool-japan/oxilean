//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::HashMap;

use super::types::{
    BarResolution, BirthDeathPair, CechCocycle, ChainComplex, ChainMap, Convergence,
    CyclicHomologyData, DifferentialMap, ExtFunctor, ExtGroup, ExtGroupComputation, FlatResolution,
    GroupCohomologyBar, HochschildComplex, HomologyGroup, InjectiveResolution, KunnethData,
    LocalCohomology, LongExactSequence, LyndonHochschildSerre, PersistenceBarcode,
    PersistenceInterval, PersistentHomologyComputer, PerverseSheafData, ProjectiveResolution,
    SimplexBoundaryMatrix, SpectralSequence, SpectralSequenceData, SpectralSequencePage,
    SpectralSequencePageManager, TorExtData, TorFunctor, TorGroup, TruncationFunctor,
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
/// `ChainComplex : Type` — a chain complex C_• with boundary maps d∘d = 0.
pub fn chain_complex_ty() -> Expr {
    type1()
}
/// `ChainMap : ChainComplex → ChainComplex → Type` — a chain map f: C_• → D_•.
/// Commutes with boundary: d_D ∘ f_n = f_{n-1} ∘ d_C.
pub fn chain_map_ty() -> Expr {
    arrow(cst("ChainComplex"), arrow(cst("ChainComplex"), type0()))
}
/// `HomologyGroup : ChainComplex → ℤ → Module` — H_n(C) = ker(d_n)/im(d_{n+1}).
pub fn homology_group_ty() -> Expr {
    arrow(cst("ChainComplex"), arrow(int_ty(), cst("Module")))
}
/// `ProjectiveResolution : Module → Type` — a projective resolution
/// … → P_2 → P_1 → P_0 → M → 0.
pub fn projective_resolution_ty() -> Expr {
    arrow(cst("Module"), type0())
}
/// `InjectiveResolution : Module → Type` — an injective resolution
/// 0 → M → I_0 → I_1 → …
pub fn injective_resolution_ty() -> Expr {
    arrow(cst("Module"), type0())
}
/// `FlatResolution : Module → Type` — a flat resolution of M.
pub fn flat_resolution_ty() -> Expr {
    arrow(cst("Module"), type0())
}
/// `TorFunctor : ℕ → Module → Module → Module` — Tor_n^R(M, N).
pub fn tor_functor_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("Module"), arrow(cst("Module"), cst("Module"))),
    )
}
/// `ExtFunctor : ℕ → Module → Module → Module` — Ext^n_R(M, N).
pub fn ext_functor_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("Module"), arrow(cst("Module"), cst("Module"))),
    )
}
/// `LocalCohomology : Ideal → ℕ → Module → Module` — H^n_I(M).
pub fn local_cohomology_ty() -> Expr {
    arrow(
        cst("Ideal"),
        arrow(nat_ty(), arrow(cst("Module"), cst("Module"))),
    )
}
/// `SpectralSequence : Type` — a spectral sequence (E_r, d_r)_{r ≥ 0}
/// converging to H^{p+q} of some filtration.
pub fn spectral_sequence_ty() -> Expr {
    type1()
}
/// `LyndonHochschildSerre : GroupExt → Module → SpectralSequence` —
/// the LHS spectral sequence for a group extension 1 → N → G → Q → 1.
pub fn lhs_spectral_sequence_ty() -> Expr {
    arrow(
        cst("GroupExt"),
        arrow(cst("Module"), cst("SpectralSequence")),
    )
}
/// `BarResolution : Group → Ring → ProjectiveResolution` — the standard bar resolution.
pub fn bar_resolution_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("Ring"), cst("ProjectiveResolution")),
    )
}
/// `IsExact : ChainComplex → Prop` — the chain complex is exact (all homology vanishes).
pub fn is_exact_ty() -> Expr {
    arrow(cst("ChainComplex"), prop())
}
/// Populate an OxiLean kernel `Environment` with homological computation axioms.
pub fn build_homological_computations_env(env: &mut Environment) {
    let base_types: &[(&str, fn() -> Expr)] = &[
        ("Module", || type1()),
        ("Ring", || type1()),
        ("Group", || type1()),
        ("Ideal", || type0()),
        ("GroupExt", || type0()),
        ("ProjectiveResolution", || type0()),
        ("InjectiveResolution", || type0()),
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
        ("ChainComplex", chain_complex_ty),
        ("ChainMap", chain_map_ty),
        ("HomologyGroup", homology_group_ty),
        ("ProjectiveResolutionOf", projective_resolution_ty),
        ("InjectiveResolutionOf", injective_resolution_ty),
        ("FlatResolutionOf", flat_resolution_ty),
        ("Tor", tor_functor_ty),
        ("Ext", ext_functor_ty),
        ("LocalCohomology", local_cohomology_ty),
        ("SpectralSequence", spectral_sequence_ty),
        ("LyndonHochschildSerre", lhs_spectral_sequence_ty),
        ("BarResolution", bar_resolution_ty),
        ("IsExact", is_exact_ty),
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
}
/// Compute the image rank of a matrix (over ℤ, by row reduction).
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
/// Compute the kernel rank of a matrix.
pub fn kernel_rank(matrix: &[Vec<i64>], cols: usize) -> usize {
    cols.saturating_sub(image_rank(matrix, cols))
}
/// `VietorisRipsFiltration : MetricSpace → ℝ → SimplicialComplex` —
/// the Vietoris-Rips complex at scale ε.
pub fn vietoris_rips_filtration_ty() -> Expr {
    arrow(
        cst("MetricSpace"),
        arrow(cst("Real"), cst("SimplicialComplex")),
    )
}
/// `PersistenceBarcode : PersistentHomology → ℕ → Type` —
/// the barcode diagram in degree n: a multiset of (birth, death) pairs.
pub fn persistence_barcode_ty() -> Expr {
    arrow(cst("PersistentHomology"), arrow(nat_ty(), type0()))
}
/// `PersistenceStability : PersistentHomology → PersistentHomology → ℝ → Prop` —
/// bottleneck distance stability: d_B(dgm(f), dgm(g)) ≤ ‖f − g‖_∞.
pub fn persistence_stability_ty() -> Expr {
    arrow(
        cst("PersistentHomology"),
        arrow(cst("PersistentHomology"), arrow(cst("Real"), prop())),
    )
}
/// `PersistentHomology : SimplicialComplex → ℕ → Type` —
/// the persistent homology in degree n of a filtered complex.
pub fn persistent_homology_ty() -> Expr {
    arrow(cst("SimplicialComplex"), arrow(nat_ty(), type0()))
}
/// `CubicalComplex : Type` — a cubical complex built from elementary cubes in ℝⁿ.
pub fn cubical_complex_ty() -> Expr {
    type1()
}
/// `CubicalChainComplex : CubicalComplex → ChainComplex` —
/// the cubical chain complex associated to a cubical complex.
pub fn cubical_chain_complex_ty() -> Expr {
    arrow(cst("CubicalComplex"), cst("ChainComplex"))
}
/// `CubicalHomology : CubicalComplex → ℕ → Module` —
/// the n-th cubical homology group H_n^cub(X).
pub fn cubical_homology_ty() -> Expr {
    arrow(cst("CubicalComplex"), arrow(nat_ty(), cst("Module")))
}
/// `CechCohomology : TopologicalSpace → Sheaf → ℕ → Module` —
/// the Čech cohomology Ȟⁿ(X; F) of a sheaf F over a space X.
pub fn cech_cohomology_ty() -> Expr {
    arrow(
        cst("TopologicalSpace"),
        arrow(cst("Sheaf"), arrow(nat_ty(), cst("Module"))),
    )
}
/// `LerayAcyclicity : Cover → Sheaf → Prop` —
/// the Leray acyclicity condition: all higher Čech cohomology over the cover vanishes.
pub fn leray_acyclicity_ty() -> Expr {
    arrow(cst("Cover"), arrow(cst("Sheaf"), prop()))
}
/// `SheafCohomology : TopologicalSpace → Sheaf → ℕ → Module` —
/// the sheaf cohomology Hⁿ(X; F) via derived functors of global sections.
pub fn sheaf_cohomology_ty() -> Expr {
    arrow(
        cst("TopologicalSpace"),
        arrow(cst("Sheaf"), arrow(nat_ty(), cst("Module"))),
    )
}
/// `GroupCohomology : Group → Module → ℕ → Module` — Hⁿ(G; M).
pub fn group_cohomology_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("Module"), arrow(nat_ty(), cst("Module"))),
    )
}
/// `GroupHomology : Group → Module → ℕ → Module` — H_n(G; M).
pub fn group_homology_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("Module"), arrow(nat_ty(), cst("Module"))),
    )
}
/// `LyndonHochschildSerreSeq : GroupExt → Module → SpectralSequence` —
/// E_2^{p,q} = Hᵖ(Q, Hᵍ(N, M)) ⟹ Hᵖ⁺ᵍ(G, M).
pub fn lhs_seq_ty() -> Expr {
    arrow(
        cst("GroupExt"),
        arrow(cst("Module"), cst("SpectralSequence")),
    )
}
/// `HochschildCohomology : Algebra → Bimodule → ℕ → Module` —
/// the Hochschild cohomology HHⁿ(A; M).
pub fn hochschild_cohomology_ty() -> Expr {
    arrow(
        cst("Algebra"),
        arrow(cst("Bimodule"), arrow(nat_ty(), cst("Module"))),
    )
}
/// `HKRTheorem : SmoothAlgebra → HochschildCohomology → DifferentialForms → Prop` —
/// the Hochschild-Kostant-Rosenberg theorem: HH*(A) ≅ Ω*(A) for smooth A.
pub fn hkr_theorem_ty() -> Expr {
    arrow(
        cst("SmoothAlgebra"),
        arrow(
            cst("HochschildCohomology"),
            arrow(cst("DifferentialForms"), prop()),
        ),
    )
}
/// `DeformationComplex : Algebra → ChainComplex` —
/// the deformation complex controlling deformations of an algebra (Hochschild complex).
pub fn deformation_complex_ty() -> Expr {
    arrow(cst("Algebra"), cst("ChainComplex"))
}
/// `ProjectiveResolutionOf : Module → Ring → Type` —
/// a projective resolution of M as an R-module.
pub fn projective_resolution_of_ty() -> Expr {
    arrow(cst("Module"), arrow(cst("Ring"), type0()))
}
/// `InjectiveResolutionOf : Module → Ring → Type` —
/// an injective resolution of M as an R-module.
pub fn injective_resolution_of_ty() -> Expr {
    arrow(cst("Module"), arrow(cst("Ring"), type0()))
}
/// `DerivedFunctor : (Module → Module) → ℕ → Module → Module` —
/// the n-th derived functor R^n F of a left-exact functor F.
pub fn derived_functor_ty() -> Expr {
    arrow(
        arrow(cst("Module"), cst("Module")),
        arrow(nat_ty(), arrow(cst("Module"), cst("Module"))),
    )
}
/// `AdamsSpectralSequence : Spectrum → Spectrum → SpectralSequence` —
/// E_2^{s,t} = Ext^{s,t}_{A}(H*(Y), H*(X)) ⟹ [X, Y]_{t-s}^∧.
pub fn adams_spectral_sequence_ty() -> Expr {
    arrow(
        cst("Spectrum"),
        arrow(cst("Spectrum"), cst("SpectralSequence")),
    )
}
/// `EilenbergMooreSpectralSequence : Fibration → SpectralSequence` —
/// for a fibration F → E → B, E_2 = Tor_{H*(B)}(H*(E), H*(F)).
pub fn eilenberg_moore_ss_ty() -> Expr {
    arrow(cst("Fibration"), cst("SpectralSequence"))
}
/// `AtiyahHirzebruchSpectralSequence : TopologicalSpace → CohomologyTheory → SpectralSequence` —
/// E_2^{p,q} = H^p(X; h^q(pt)) ⟹ h^{p+q}(X).
pub fn atiyah_hirzebruch_ss_ty() -> Expr {
    arrow(
        cst("TopologicalSpace"),
        arrow(cst("CohomologyTheory"), cst("SpectralSequence")),
    )
}
/// `Perversity : ℕ → ℕ` — a perversity function p̄: ℕ → ℕ with p̄(0)=p̄(1)=0.
pub fn perversity_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `IntersectionHomology : PseudomanifoldX → Perversity → ℕ → Module` —
/// the intersection homology IH^p̄_n(X).
pub fn intersection_homology_ty() -> Expr {
    arrow(
        cst("PseudomanifoldX"),
        arrow(cst("Perversity"), arrow(nat_ty(), cst("Module"))),
    )
}
/// `IHvsCohomology : PseudomanifoldX → Prop` —
/// for a manifold, IH*(X) ≅ H*(X).
pub fn ih_vs_cohomology_ty() -> Expr {
    arrow(cst("PseudomanifoldX"), prop())
}
/// `IntersectionForm : Manifold4 → BilinearForm` —
/// the intersection form Q: H_2(M) × H_2(M) → ℤ on a 4-manifold.
pub fn intersection_form_ty() -> Expr {
    arrow(cst("Manifold4"), cst("BilinearForm"))
}
/// `PoincaredualityIso : ClosedManifold → ℕ → ℕ → Prop` —
/// Poincaré duality: H^k(M) ≅ H_{n-k}(M) for a closed n-manifold.
pub fn poincare_duality_iso_ty() -> Expr {
    arrow(
        cst("ClosedManifold"),
        arrow(nat_ty(), arrow(nat_ty(), prop())),
    )
}
/// `LinkingForm : Manifold → BilinearForm` — the linking form on torsion homology.
pub fn linking_form_ty() -> Expr {
    arrow(cst("Manifold"), cst("BilinearForm"))
}
/// `SignatureInvariant : Manifold4 → Int` — the signature σ(M) = b₊ − b₋.
pub fn signature_invariant_ty() -> Expr {
    arrow(cst("Manifold4"), int_ty())
}
/// `BoundaryMatrix : SimplicialComplex → ℕ → Matrix` —
/// the n-th boundary matrix ∂_n of a simplicial complex.
pub fn boundary_matrix_ty() -> Expr {
    arrow(cst("SimplicialComplex"), arrow(nat_ty(), cst("Matrix")))
}
/// `SmithNormalForm : Matrix → SmithDecomposition` —
/// the Smith normal form of an integer matrix.
pub fn smith_normal_form_ty() -> Expr {
    arrow(cst("Matrix"), cst("SmithDecomposition"))
}
/// `SimplicialHomologyAlg : SimplicialComplex → ℕ → Module` —
/// algorithmic computation of H_n(K) via SNF reduction.
pub fn simplicial_homology_alg_ty() -> Expr {
    arrow(cst("SimplicialComplex"), arrow(nat_ty(), cst("Module")))
}
/// `CupProductPersistence : PersistentCohomology → ℕ → ℕ → PersistencePairing` —
/// persistent cup product structure in degrees p and q.
pub fn cup_product_persistence_ty() -> Expr {
    arrow(
        cst("PersistentCohomology"),
        arrow(nat_ty(), arrow(nat_ty(), cst("PersistencePairing"))),
    )
}
/// `SteenrodOperation : ℕ → PersistentCohomology → PersistentCohomology` —
/// the Steenrod square Sq^i on persistent cohomology.
pub fn steenrod_operation_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("PersistentCohomology"), cst("PersistentCohomology")),
    )
}
/// `MagnitudeHomology : MetricSpace → ℕ → Module` —
/// the magnitude homology MH_n(X) of a metric space (Hepworth-Willerton).
pub fn magnitude_homology_ty() -> Expr {
    arrow(cst("MetricSpace"), arrow(nat_ty(), cst("Module")))
}
/// `MagnitudeFunction : MetricSpace → ℝ → ℝ` —
/// the magnitude function |X|_t (Leinster).
pub fn magnitude_function_ty() -> Expr {
    arrow(cst("MetricSpace"), arrow(cst("Real"), cst("Real")))
}
/// `ConfigurationSpace : TopologicalSpace → ℕ → TopologicalSpace` —
/// the ordered configuration space Conf_n(X).
pub fn configuration_space_ty() -> Expr {
    arrow(
        cst("TopologicalSpace"),
        arrow(nat_ty(), cst("TopologicalSpace")),
    )
}
/// `FadellNeuwirthFibration : TopologicalSpace → ℕ → ℕ → Fibration` —
/// the Fadell-Neuwirth fibration Conf_{n+k}(X) → Conf_n(X).
pub fn fadell_neuwirth_fibration_ty() -> Expr {
    arrow(
        cst("TopologicalSpace"),
        arrow(nat_ty(), arrow(nat_ty(), cst("Fibration"))),
    )
}
/// `BorelConstruction : Group → TopologicalSpace → TopologicalSpace` —
/// the Borel construction EG ×_G X.
pub fn borel_construction_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("TopologicalSpace"), cst("TopologicalSpace")),
    )
}
/// `EquivariantCohomology : Group → TopologicalSpace → ℕ → Module` —
/// the equivariant cohomology H^n_G(X).
pub fn equivariant_cohomology_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("TopologicalSpace"), arrow(nat_ty(), cst("Module"))),
    )
}
/// `LocalizationTheorem : Group → TopologicalSpace → Prop` —
/// the localization theorem: H*_G(X) ≅ H*_G(X^G) after inverting |G|.
pub fn localization_theorem_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("TopologicalSpace"), prop()))
}
/// `MotivicCohomology : Scheme → ℕ → ℕ → Module` —
/// the motivic cohomology H^{n,p}(X, ℤ).
pub fn motivic_cohomology_ty() -> Expr {
    arrow(
        cst("Scheme"),
        arrow(nat_ty(), arrow(nat_ty(), cst("Module"))),
    )
}
/// `CycleClassMap : MotivicCohomology → SingularCohomology → Prop` —
/// the cycle class map cl: CH^p(X) → H^{2p}(X, ℤ).
pub fn cycle_class_map_ty() -> Expr {
    arrow(
        cst("MotivicCohomology"),
        arrow(cst("SingularCohomology"), prop()),
    )
}
/// `MilnorBlochKatoConjecture : Field → ℕ → Prop` —
/// the Milnor-Bloch-Kato conjecture (Voevodsky's theorem):
/// K^M_n(F)/l ≅ H^n_et(F, μ_l^⊗n).
pub fn milnor_bloch_kato_ty() -> Expr {
    arrow(cst("Field"), arrow(nat_ty(), prop()))
}
/// Populate the environment with the extended set of homological computation axioms.
pub fn build_homological_computations_env_extended(env: &mut Environment) {
    let extra_base: &[(&str, fn() -> Expr)] = &[
        ("MetricSpace", || type1()),
        ("SimplicialComplex", || type1()),
        ("PersistentHomology", || type1()),
        ("TopologicalSpace", || type1()),
        ("Sheaf", || type0()),
        ("Cover", || type0()),
        ("Algebra", || type1()),
        ("Bimodule", || type0()),
        ("SmoothAlgebra", || type1()),
        ("HochschildCohomology", || type0()),
        ("DifferentialForms", || type0()),
        ("Spectrum", || type1()),
        ("Fibration", || type0()),
        ("CohomologyTheory", || type1()),
        ("PseudomanifoldX", || type1()),
        ("Manifold4", || type1()),
        ("Manifold", || type1()),
        ("ClosedManifold", || type1()),
        ("BilinearForm", || type0()),
        ("Matrix", || type0()),
        ("SmithDecomposition", || type0()),
        ("PersistentCohomology", || type0()),
        ("PersistencePairing", || type0()),
        ("Real", || type0()),
        ("ConfigurationSpace", || type1()),
        ("Scheme", || type1()),
        ("Field", || type1()),
        ("MotivicCohomology", || type0()),
        ("SingularCohomology", || type0()),
        ("CubicalComplex", || type1()),
    ];
    for (name, mk_ty) in extra_base {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    let new_axioms: &[(&str, fn() -> Expr)] = &[
        ("VietorisRipsFiltration", vietoris_rips_filtration_ty),
        ("PersistenceBarcode", persistence_barcode_ty),
        ("PersistenceStability", persistence_stability_ty),
        ("PersistentHomologyOf", persistent_homology_ty),
        ("CubicalComplex", cubical_complex_ty),
        ("CubicalChainComplex", cubical_chain_complex_ty),
        ("CubicalHomology", cubical_homology_ty),
        ("CechCohomology", cech_cohomology_ty),
        ("LerayAcyclicity", leray_acyclicity_ty),
        ("SheafCohomology", sheaf_cohomology_ty),
        ("GroupCohomology", group_cohomology_ty),
        ("GroupHomology", group_homology_ty),
        ("LyndonHochschildSerreSeq", lhs_seq_ty),
        ("HochschildCohomology", hochschild_cohomology_ty),
        ("HKRTheorem", hkr_theorem_ty),
        ("DeformationComplex", deformation_complex_ty),
        ("ProjectiveResolutionOfRing", projective_resolution_of_ty),
        ("InjectiveResolutionOfRing", injective_resolution_of_ty),
        ("DerivedFunctor", derived_functor_ty),
        ("AdamsSpectralSequence", adams_spectral_sequence_ty),
        ("EilenbergMooreSpectralSequence", eilenberg_moore_ss_ty),
        ("AtiyahHirzebruchSpectralSequence", atiyah_hirzebruch_ss_ty),
        ("Perversity", perversity_ty),
        ("IntersectionHomology", intersection_homology_ty),
        ("IHvsCohomology", ih_vs_cohomology_ty),
        ("IntersectionForm", intersection_form_ty),
        ("PoincaredualityIso", poincare_duality_iso_ty),
        ("LinkingForm", linking_form_ty),
        ("SignatureInvariant", signature_invariant_ty),
        ("BoundaryMatrix", boundary_matrix_ty),
        ("SmithNormalForm", smith_normal_form_ty),
        ("SimplicialHomologyAlg", simplicial_homology_alg_ty),
        ("CupProductPersistence", cup_product_persistence_ty),
        ("SteenrodOperation", steenrod_operation_ty),
        ("MagnitudeHomology", magnitude_homology_ty),
        ("MagnitudeFunction", magnitude_function_ty),
        ("ConfigurationSpaceOf", configuration_space_ty),
        ("FadellNeuwirthFibration", fadell_neuwirth_fibration_ty),
        ("BorelConstruction", borel_construction_ty),
        ("EquivariantCohomology", equivariant_cohomology_ty),
        ("LocalizationTheorem", localization_theorem_ty),
        ("MotivicCohomologyOf", motivic_cohomology_ty),
        ("CycleClassMap", cycle_class_map_ty),
        ("MilnorBlochKato", milnor_bloch_kato_ty),
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
    fn test_chain_complex_betti_circle() {
        let mut c = ChainComplex::new();
        c.add_group(1, "C_0");
        c.add_group(1, "C_1");
        c.add_boundary(vec![vec![0]]);
        let betti = c.betti_numbers();
        assert_eq!(betti[0], 1);
        assert_eq!(betti[1], 1);
        assert_eq!(c.euler_characteristic(), 0);
    }
    #[test]
    fn test_chain_complex_sphere_s2() {
        let mut c = ChainComplex::new();
        c.add_group(1, "C_0");
        c.add_group(0, "C_1");
        c.add_group(1, "C_2");
        c.add_boundary(vec![]);
        c.add_boundary(vec![]);
        let chi = c.euler_characteristic();
        assert_eq!(chi, 2);
    }
    #[test]
    fn test_chain_complex_is_exact_acyclic() {
        let mut c = ChainComplex::new();
        c.add_group(1, "C_0");
        c.add_group(1, "C_1");
        c.add_boundary(vec![vec![1]]);
        assert!(c.is_exact_at(0));
        assert!(c.is_exact_at(1));
    }
    #[test]
    fn test_homology_group_display() {
        let h = HomologyGroup::new(1, 2, vec![3]);
        let s = format!("{}", h);
        assert!(s.contains("H_1"));
        assert!(s.contains("Z^2"));
    }
    #[test]
    fn test_homology_trivial() {
        let h = HomologyGroup::new(0, 0, vec![]);
        assert!(h.is_trivial());
    }
    #[test]
    fn test_projective_resolution_betti() {
        let mut res = ProjectiveResolution::new("M");
        res.add_step(1, vec![]);
        res.add_step(2, vec![vec![1, 0], vec![0, 1]]);
        res.add_step(1, vec![vec![-1], vec![1]]);
        assert_eq!(res.betti_numbers(), vec![1, 2, 1]);
        assert_eq!(res.projective_dimension(), Some(2));
    }
    #[test]
    fn test_injective_resolution_dim() {
        let mut res = InjectiveResolution::new("N");
        res.add_step(1, vec![]);
        res.add_step(2, vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(res.injective_dimension(), Some(1));
    }
    #[test]
    fn test_bar_resolution_ranks() {
        let bar = BarResolution::new("Z/2", 2, 4);
        assert_eq!(bar.rank_at(0), 1);
        assert_eq!(bar.rank_at(1), 2);
        assert_eq!(bar.rank_at(2), 4);
        assert_eq!(bar.rank_at(3), 8);
    }
    #[test]
    fn test_tor_functor() {
        let mut res = ProjectiveResolution::new("M");
        res.add_step(1, vec![]);
        res.add_step(1, vec![vec![2]]);
        let tor = TorFunctor::compute("M", "Z/2", &res);
        assert_eq!(tor.tor_at(0).map(|t| t.rank), Some(1));
        assert_eq!(tor.projective_dimension(), Some(1));
    }
    #[test]
    fn test_ext_functor() {
        let mut res = ProjectiveResolution::new("M");
        res.add_step(1, vec![]);
        res.add_step(1, vec![vec![3]]);
        let ext = ExtFunctor::compute("M", "Z", &res);
        let e0 = ext.ext_at(0).expect("ext_at should succeed");
        assert_eq!(e0.degree, 0);
        assert!(!e0.is_zero());
    }
    #[test]
    fn test_ext_group_display() {
        let e = ExtGroup::new(2, 3);
        let s = format!("{}", e);
        assert!(s.contains("Ext^2"));
        assert!(s.contains("rank=3"));
    }
    #[test]
    fn test_tor_group_display() {
        let t = TorGroup::new(1, 2);
        let s = format!("{}", t);
        assert!(s.contains("Tor_1"));
        assert!(s.contains("rank=2"));
    }
    #[test]
    fn test_spectral_sequence_e_term() {
        let mut ss = SpectralSequence::new();
        let mut page0: HashMap<(i32, i32), usize> = HashMap::new();
        page0.insert((0, 0), 1);
        page0.insert((1, 1), 2);
        ss.add_page(page0);
        assert_eq!(ss.e_term(0, 0, 0), Some(1));
        assert_eq!(ss.e_term(0, 1, 1), Some(2));
        assert_eq!(ss.e_term(0, 2, 2), Some(0));
    }
    #[test]
    fn test_spectral_sequence_page_total_rank() {
        let mut page = SpectralSequencePage::new(2);
        page.set(0, 0, 1);
        page.set(1, 0, 3);
        page.set(0, 1, 2);
        assert_eq!(page.total_rank(), 6);
    }
    #[test]
    fn test_differential_map_target() {
        let d = DifferentialMap::new(2, 1, 3, 1);
        assert_eq!(d.target, (3, 2));
    }
    #[test]
    fn test_convergence_cohomology_rank() {
        let mut e_inf = SpectralSequencePage::new(0);
        e_inf.set(0, 2, 1);
        e_inf.set(1, 1, 2);
        e_inf.set(2, 0, 1);
        let conv = Convergence::new(e_inf);
        assert_eq!(conv.cohomology_rank(2), 4);
    }
    #[test]
    fn test_lhs_spectral_sequence_abutment() {
        let mut e2: HashMap<(i32, i32), usize> = HashMap::new();
        e2.insert((0, 0), 1);
        e2.insert((1, 0), 1);
        e2.insert((0, 1), 2);
        let lhs = LyndonHochschildSerre::new("N", "Q", "k", e2);
        assert_eq!(lhs.abutment_rank(1), 3);
    }
    #[test]
    fn test_local_cohomology_cohom_dim() {
        let mut lc = LocalCohomology::new("m", "M");
        lc.add_group(0, 0);
        lc.add_group(1, 0);
        lc.add_group(2, 1);
        assert_eq!(lc.cohomological_dimension(), Some(2));
    }
    #[test]
    fn test_flat_resolution() {
        let fr = FlatResolution::new("M", vec![1, 1, 0]);
        assert_eq!(fr.flat_dim, Some(1));
    }
    #[test]
    fn test_build_homological_computations_env() {
        let mut env = Environment::new();
        build_homological_computations_env(&mut env);
        assert!(env.get(&Name::str("ChainComplex")).is_some());
        assert!(env.get(&Name::str("HomologyGroup")).is_some());
        assert!(env.get(&Name::str("Tor")).is_some());
        assert!(env.get(&Name::str("Ext")).is_some());
        assert!(env.get(&Name::str("SpectralSequence")).is_some());
        assert!(env.get(&Name::str("BarResolution")).is_some());
    }
    #[test]
    fn test_chain_map_induced_rank() {
        let mut src = ChainComplex::new();
        src.add_group(2, "C_0");
        let mut tgt = ChainComplex::new();
        tgt.add_group(2, "D_0");
        let cm = ChainMap::new(src, tgt, vec![vec![vec![1, 0], vec![0, 1]]]);
        assert_eq!(cm.induced_homology_rank(0), 2);
    }
    #[test]
    fn test_birth_death_pair_persistence() {
        let p = BirthDeathPair::new(0.5, 1.5, 1);
        assert!((p.persistence() - 1.0).abs() < 1e-10);
        assert!(!p.is_essential());
    }
    #[test]
    fn test_birth_death_pair_essential() {
        let p = BirthDeathPair::new(0.0, f64::INFINITY, 0);
        assert!(p.is_essential());
    }
    #[test]
    fn test_persistence_barcode_betti() {
        let mut bc = PersistenceBarcode::new();
        bc.add(0.0, 1.0, 0);
        bc.add(0.0, f64::INFINITY, 0);
        bc.add(0.5, 1.5, 1);
        assert_eq!(bc.betti_number(0), 2);
        assert_eq!(bc.betti_number(1), 1);
    }
    #[test]
    fn test_persistence_barcode_bottleneck_distance() {
        let mut bc1 = PersistenceBarcode::new();
        bc1.add(0.0, 1.0, 0);
        bc1.add(0.0, 2.0, 0);
        let mut bc2 = PersistenceBarcode::new();
        bc2.add(0.0, 1.2, 0);
        bc2.add(0.0, 1.8, 0);
        let d = bc1.bottleneck_distance(&bc2, 0);
        assert!(d >= 0.0);
    }
    #[test]
    fn test_persistent_homology_computer_add_and_compute() {
        let mut ph = PersistentHomologyComputer::new();
        ph.add_simplex(0.0, 0);
        ph.add_simplex(0.0, 0);
        ph.add_simplex(1.0, 1);
        let bc = ph.compute_barcode();
        assert!(bc.betti_number(0) >= 1);
    }
    #[test]
    fn test_simplex_boundary_matrix_set_get() {
        let mut m = SimplexBoundaryMatrix::new(3, 3);
        m.set(0, 1, 5);
        m.set(2, 0, -3);
        assert_eq!(m.get(0, 1), 5);
        assert_eq!(m.get(2, 0), -3);
        assert_eq!(m.get(1, 1), 0);
    }
    #[test]
    fn test_simplex_boundary_matrix_rank() {
        let mut m = SimplexBoundaryMatrix::new(2, 2);
        m.set(0, 0, 1);
        m.set(1, 1, 2);
        assert_eq!(m.rank(), 2);
    }
    #[test]
    fn test_simplex_boundary_matrix_torsion() {
        let mut m = SimplexBoundaryMatrix::new(2, 2);
        m.set(0, 0, 1);
        m.set(1, 1, 6);
        let torsion = m.torsion_coefficients();
        assert!(torsion.contains(&6));
    }
    #[test]
    fn test_simplex_boundary_matrix_to_dense() {
        let mut m = SimplexBoundaryMatrix::new(2, 2);
        m.set(0, 0, 3);
        m.set(1, 1, -1);
        let dense = m.to_dense();
        assert_eq!(dense[0][0], 3);
        assert_eq!(dense[1][1], -1);
        assert_eq!(dense[0][1], 0);
    }
    #[test]
    fn test_spectral_sequence_page_manager_add_and_get() {
        let mut mgr = SpectralSequencePageManager::new();
        let mut entries: HashMap<(i32, i32), usize> = HashMap::new();
        entries.insert((0, 0), 1);
        entries.insert((1, 1), 3);
        mgr.add_page(entries);
        let page = mgr.page(2).expect("page should succeed");
        assert_eq!(page.get(0, 0), 1);
        assert_eq!(page.get(1, 1), 3);
    }
    #[test]
    fn test_spectral_sequence_page_manager_advance() {
        let mut mgr = SpectralSequencePageManager::new();
        let mut entries: HashMap<(i32, i32), usize> = HashMap::new();
        entries.insert((0, 0), 2);
        entries.insert((2, -1), 1);
        mgr.add_page(entries);
        mgr.add_differential(0, 0, 1);
        mgr.advance();
        assert!(mgr.page(3).is_some());
    }
    #[test]
    fn test_spectral_sequence_page_manager_collapsed() {
        let mut mgr = SpectralSequencePageManager::new();
        let mut entries: HashMap<(i32, i32), usize> = HashMap::new();
        entries.insert((0, 0), 1);
        mgr.add_page(entries);
        assert!(mgr.has_collapsed());
    }
    #[test]
    fn test_group_cohomology_bar_cochain_rank() {
        let gcb = GroupCohomologyBar::new("Z/3", 3, "Z");
        assert_eq!(gcb.cochain_rank(2, 1), 9);
        assert_eq!(gcb.cochain_rank(0, 1), 1);
    }
    #[test]
    fn test_group_cohomology_bar_compute() {
        let mut gcb = GroupCohomologyBar::new("Z/2", 2, "Z");
        gcb.compute_cohomology(1, 3);
        assert_eq!(gcb.cohomology_at(0), 1);
        assert_eq!(gcb.cohomology_at(1), 0);
        assert_eq!(gcb.cohomology_at(2), 0);
    }
    #[test]
    fn test_group_cohomology_bar_euler() {
        let mut gcb = GroupCohomologyBar::new("Z/2", 2, "Z");
        gcb.compute_cohomology(1, 2);
        let chi = gcb.euler_characteristic();
        assert_eq!(chi, 1);
    }
    #[test]
    fn test_build_homological_computations_env_extended() {
        let mut env = Environment::new();
        build_homological_computations_env(&mut env);
        build_homological_computations_env_extended(&mut env);
        assert!(env.get(&Name::str("VietorisRipsFiltration")).is_some());
        assert!(env.get(&Name::str("CubicalHomology")).is_some());
        assert!(env.get(&Name::str("CechCohomology")).is_some());
        assert!(env.get(&Name::str("HochschildCohomology")).is_some());
        assert!(env.get(&Name::str("AdamsSpectralSequence")).is_some());
        assert!(env.get(&Name::str("IntersectionHomology")).is_some());
        assert!(env.get(&Name::str("IntersectionForm")).is_some());
        assert!(env.get(&Name::str("MagnitudeHomology")).is_some());
        assert!(env.get(&Name::str("EquivariantCohomology")).is_some());
        assert!(env.get(&Name::str("MilnorBlochKato")).is_some());
    }
    #[test]
    fn test_new_axiom_types_registered() {
        let mut env = Environment::new();
        build_homological_computations_env_extended(&mut env);
        assert!(env.get(&Name::str("PersistenceStability")).is_some());
        assert!(env.get(&Name::str("LerayAcyclicity")).is_some());
        assert!(env.get(&Name::str("GroupCohomology")).is_some());
        assert!(env.get(&Name::str("SteenrodOperation")).is_some());
        assert!(env.get(&Name::str("FadellNeuwirthFibration")).is_some());
        assert!(env.get(&Name::str("CycleClassMap")).is_some());
        assert!(env.get(&Name::str("SmithNormalForm")).is_some());
        assert!(env.get(&Name::str("PoincaredualityIso")).is_some());
        assert!(env.get(&Name::str("LocalizationTheorem")).is_some());
        assert!(env.get(&Name::str("SignatureInvariant")).is_some());
    }
}
#[cfg(test)]
mod extended_homological_tests {
    use super::*;
    #[test]
    fn test_spectral_sequence() {
        let ss = SpectralSequenceData::serre("B", "E", "F");
        assert_eq!(ss.page, 2);
        assert!(ss.e2_page_description().contains("E_2"));
    }
    #[test]
    fn test_tor_ext() {
        let tor = TorExtData::tor("Z", "Z/2", "Z/3", 1);
        assert!(tor.is_balanced());
        assert!(tor.value.contains("Tor"));
        let ext = TorExtData::ext("Z", "Z/2", "Z", 1);
        assert!(!ext.is_balanced());
    }
    #[test]
    fn test_long_exact_sequence() {
        let les = LongExactSequence::from_short("A", "B", "C");
        assert_eq!(les.length(), 4);
        assert!(les.connecting_homomorphism.contains("delta"));
    }
    #[test]
    fn test_kunneth() {
        let k = KunnethData::over_field("S^2", "S^3");
        assert!(k.kunneth_description().contains("≅"));
    }
    #[test]
    fn test_persistence_interval() {
        let i = PersistenceInterval::new(1, 0.5, 2.0);
        assert!((i.persistence() - 1.5).abs() < 1e-9);
        assert!(!i.is_essential());
        assert!(i.contains(1.0));
        assert!(!i.contains(3.0));
    }
}
#[cfg(test)]
mod tests_homol_comp_ext {
    use super::*;
    #[test]
    fn test_cech_cocycle() {
        let cech = CechCocycle::new(3, 1);
        assert!(cech.is_cocycle);
        let leray = cech.leray_theorem();
        assert!(leray.contains("Leray"));
        let mv = cech.mayer_vietoris_for_two_opens();
        assert!(mv.contains("Mayer-Vietoris"));
    }
    #[test]
    fn test_ext_group() {
        let ext = ExtGroupComputation::new("Z/2", "Z");
        let e0 = ext.compute_ext_0();
        assert!(e0.contains("Ext^0"));
        let e1 = ext.compute_ext_1();
        assert!(e1.contains("Ext^1"));
        let horse = ext.horseshoe_lemma();
        assert!(horse.contains("Horseshoe"));
    }
    #[test]
    fn test_hochschild_complex() {
        let hh = HochschildComplex {
            algebra: "A".to_string(),
            module: "A".to_string(),
            dimension: 2,
            is_free_algebra: true,
        };
        let hkr = hh.hochschild_kostant_rosenberg();
        assert!(hkr.contains("HKR"));
        let cc = hh.cyclic_homology_connection();
        assert!(cc.contains("HC"));
        let lqt = hh.loday_quillen_tsygan();
        assert!(lqt.contains("Loday"));
        assert!(hh.degeneration_at_e2());
    }
    #[test]
    fn test_cyclic_homology() {
        let ch = CyclicHomologyData::for_polynomial_ring(3);
        let connes = ch.connes_differential();
        assert!(connes.contains("Connes"));
        let chern = ch.primary_characteristic_class();
        assert!(chern.contains("Chern character"));
    }
    #[test]
    fn test_truncation_functor() {
        let tf = TruncationFunctor::new("D(Sh(X))", 0, true);
        let desc = tf.truncation_description();
        assert!(desc.contains("kills H^i"));
        let comp = tf.composed_truncation(3);
        assert!(comp.contains("degrees"));
    }
    #[test]
    fn test_perverse_sheaf() {
        let ps = PerverseSheafData::new(vec!["X_0".to_string(), "X_1".to_string()], vec![0, -1]);
        let ic = ps.intersection_cohomology_description();
        assert!(ic.contains("IC sheaf"));
        let bbdg = ps.bbdg_decomposition();
        assert!(bbdg.contains("BBDG"));
        let vd = ps.verdier_duality();
        assert!(vd.contains("Verdier"));
        let supp = ps.support_condition();
        assert!(supp.contains("strata"));
    }
}
