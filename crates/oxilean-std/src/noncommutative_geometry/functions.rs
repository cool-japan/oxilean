//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BaumConnesStatus, CStarAlgebra, CompactQuantumGroup, DiracOperator, DiracOperatorData,
    FactorType, HopfAlgebra, NoncommutativeTorusData, SemicircleDistribution, SpectralTriple,
    VonNeumannAlgebra,
};

/// Statement of the Gelfand representation theorem.
pub fn gelfand_theorem_statement() -> &'static str {
    "Gelfand Representation Theorem: Every commutative C*-algebra A is isometrically \
     *-isomorphic to the algebra C(Sp(A)) of continuous functions on its spectrum \
     (a compact Hausdorff space). In particular, commutative unital C*-algebras \
     are in bijection with compact Hausdorff spaces."
}
/// Statement of Connes' reconstruction theorem.
pub fn connes_reconstruction_theorem() -> &'static str {
    "Connes Reconstruction Theorem: A commutative spectral triple (C^∞(M), L²(M,S), D) \
     satisfying the seven axioms of noncommutative geometry (regularity, finiteness, \
     reality, first-order, orientability, Poincaré duality) reconstructs a unique \
     compact oriented Riemannian spin manifold M up to isometry."
}
/// Statement of the six-term exact sequence in K-theory.
pub fn six_term_exact_sequence() -> &'static str {
    "Six-Term Exact Sequence: For a short exact sequence 0 → I → A → A/I → 0 \
     of C*-algebras there is a natural six-term exact sequence \
     K₀(I) → K₀(A) → K₀(A/I) \
                  ↑                ↓ (index/exponential maps) \
     K₁(A/I) ← K₁(A) ← K₁(I)."
}
/// Statement of the Connes–Chern character theorem.
pub fn connes_chern_character_statement() -> &'static str {
    "Connes–Chern Character: There is a natural ring homomorphism \
     ch* : K*(A) → HP*(A) from the K-theory of a Banach algebra A \
     to its periodic cyclic cohomology. For a smooth manifold M, \
     ch* : K*(C^∞(M)) → HP*(C^∞(M)) ≅ H^*_dR(M) recovers the \
     classical Chern character to de Rham cohomology."
}
/// The Moyal product formula for noncommutative Euclidean space.
pub fn moyal_product_formula() -> &'static str {
    "Moyal Product: On R^{2n} with Poisson bivector θ^{ij}, the Moyal star-product is \
     (f ★_θ g)(x) = exp(iℏ/2 · θ^{ij} ∂_i ⊗ ∂_j)(f ⊗ g)|_{x=y}. \
     This deforms the commutative algebra C^∞(R^{2n}) into a noncommutative algebra \
     isomorphic (as a Fréchet space) to the Weyl algebra."
}
/// Statement of the Morita equivalence theorem.
pub fn morita_equivalence_theorem() -> &'static str {
    "Morita Equivalence Theorem (Rieffel): Two C*-algebras A and B are Morita \
     equivalent (i.e. have equivalent categories of Hilbert C*-modules) if and \
     only if there exists an A–B imprimitivity bimodule. This is further equivalent \
     to stable isomorphism: A ⊗ K ≅ B ⊗ K, where K = K(ℓ²) is the compact operators."
}
/// Statement of the McKean–Singer formula.
pub fn mckean_singer_formula() -> &'static str {
    "McKean–Singer Formula: For a Dirac operator D on a compact even-dimensional \
     Riemannian spin manifold M with Z/2-grading γ, \
     index(D_+) = Tr(γ · e^{-tD²}) for all t > 0. \
     In particular, the supertrace Str(e^{-tD²}) is independent of t \
     and equals the Atiyah–Singer topological index ∫_M Â(M)."
}
/// Returns a list of core axioms for noncommutative geometry in OxiLean format.
///
/// These axioms state properties of C*-algebras, spectral triples, and related
/// structures that form the axiomatic basis of Connes' noncommutative geometry.
pub fn build_env() -> Vec<(&'static str, &'static str)> {
    vec![
        ("cstar_c_star_identity",
        "forall (A : CStarAlgebra) (a : A), norm (mul (star a) a) = mul (norm a) (norm a)",),
        ("cstar_gelfand_naimark",
        "forall (A : CStarAlgebra), is_commutative A -> exists (X : CompactHausdorff), iso A (C X)",),
        ("spectral_triple_bounded_commutator",
        "forall (T : SpectralTriple) (a : algebra T), is_bounded (commutator (dirac T) a)",),
        ("dirac_self_adjoint", "forall (D : DiracOperator), is_self_adjoint D",),
        ("dirac_compact_resolvent",
        "forall (D : DiracOperator), is_compact (resolvent D)",), ("k_theory_six_term",
        "forall (I A B : CStarAlgebra), short_exact I A B -> six_term_exact (K0 I) (K0 A) (K0 B) (K1 I) (K1 A) (K1 B)",),
        ("cyclic_cohomology_periodic",
        "forall (A : Algebra) (n : Nat), iso (HC n A) (HC (add n 2) A)",),
        ("haar_state_unique",
        "forall (G : CompactQuantumGroup), exists_unique (h : State G), is_haar_state h",),
        ("connes_distance_metric",
        "forall (T : SpectralTriple) (phi psi : State (algebra T)), 0 <= connes_dist T phi psi",),
        ("morita_preserves_k_theory",
        "forall (A B : CStarAlgebra), morita_equivalent A B -> iso (K0 A) (K0 B) /\\ iso (K1 A) (K1 B)",),
        ("spectral_triple_regularity",
        "forall (T : SpectralTriple) (a : algebra T), forall (k : Nat), is_bounded (iterated_commutator (dirac T) a k)",),
        ("spectral_triple_finiteness",
        "forall (T : SpectralTriple), is_finite_projective (smooth_domain (dirac T))",),
        ("spectral_triple_reality",
        "forall (T : SpectralTriple), exists (J : RealStructure T), satisfies_reality_axiom T J",),
        ("spectral_triple_first_order",
        "forall (T : SpectralTriple) (a b : algebra T), commutator (dirac T) (right_action b) = 0",),
        ("spectral_triple_orientability",
        "forall (T : SpectralTriple), exists (chi : HochschildCycle T), represents_volume_form chi",),
        ("spectral_triple_poincare_duality",
        "forall (T : SpectralTriple), is_invertible (cap_product_pairing T)",),
        ("connes_distance_formula",
        "forall (T : SpectralTriple) (phi psi : State (algebra T)), connes_dist T phi psi = Sup (fun a => abs (sub (phi a) (psi a))) (fun a => norm_commutator_le (dirac T) a 1)",),
        ("connes_distance_recovers_geodesic",
        "forall (M : RiemannianManifold) (x y : M), connes_dist (canonical_triple M) (eval_at x) (eval_at y) = geodesic_dist M x y",),
        ("nc_torus_commutation_relation",
        "forall (theta : Real) (U V : Generator (A_theta theta)), mul V U = mul (scalar (exp (mul (scalar (mul 2 pi)) (mul i theta)))) (mul U V)",),
        ("nc_torus_simple_irrational",
        "forall (theta : Real), is_irrational theta -> is_simple_cstar_algebra (A_theta theta)",),
        ("nc_torus_k_theory",
        "forall (theta : Real), iso (K0 (A_theta theta)) (Prod Int Int) /\\ iso (K1 (A_theta theta)) (Prod Int Int)",),
        ("moyal_product_associative",
        "forall (theta : Real) (f g h : SchwarzFunction), star_product theta (star_product theta f g) h = star_product theta f (star_product theta g h)",),
        ("moyal_product_deforms_pointwise",
        "forall (f g : SchwarzFunction), Lim (fun theta => star_product theta f g) 0 = mul f g",),
        ("wigner_function_real_valued",
        "forall (rho : DensityMatrix), is_real_valued (wigner_transform rho)",),
        ("wigner_function_normalized",
        "forall (rho : DensityMatrix), integral (wigner_transform rho) = 1",),
        ("hopf_antipode_axiom",
        "forall (H : HopfAlgebra) (a : H), mul (antipode H a) a = counit H a /\\ mul a (antipode H a) = counit H a",),
        ("hopf_coproduct_coassociative",
        "forall (H : HopfAlgebra) (a : H), Eq (comp (tensor_id (coproduct H)) (coproduct H) a) (comp (id_tensor (coproduct H)) (coproduct H) a)",),
        ("woronowicz_compact_qg_haar",
        "forall (G : WoronowiczQuantumGroup), exists_unique (h : State (cstar G)), is_invariant_state h",),
        ("quantum_group_peter_weyl",
        "forall (G : CompactQuantumGroup), is_dense (linear_span (matrix_coefficients G)) (cstar G)",),
        ("hochschild_b_nilpotent",
        "forall (A : Algebra) (n : Nat) (phi : CyclicCochain A n), b (b phi) = zero",),
        ("connes_b_operator",
        "forall (A : Algebra) (n : Nat) (phi : CyclicCochain A n), b (B phi) + B (b phi) = zero",),
        ("sbi_sequence",
        "forall (A : Algebra) (n : Nat), long_exact (HC n A) (S_op) (HC (add n 2) A) (B_op) (HH (add n 1) A) (I_map) (HC n A)",),
        ("connes_chern_char_in_ncg",
        "forall (T : SpectralTriple) (e : Projection (algebra T)), connes_chern_char T e = index_pairing T e",),
        ("spectral_triple_index",
        "forall (T : SpectralTriple) (e : Projection (algebra T)), index (fredholm_module T) e = connes_chern_char T e",),
        ("local_index_formula",
        "forall (T : RegularSpectralTriple) (a0 : algebra T), index_pairing T a0 = Sum (fun k => mul (local_coeff k T) (zeta_residue T a0 k))",),
        ("spectral_action_bosonic",
        "forall (T : SpectralTriple) (Lambda : Real) (f : Real -> Real), spectral_action T Lambda f = Tr (f (div (dirac T) Lambda))",),
        ("spectral_action_standard_model",
        "forall (T : StandardModelTriple), spectral_action_expansion T = add (einstein_hilbert_action T) (yang_mills_higgs_action T)",),
        ("bott_periodicity",
        "forall (A : CStarAlgebra), iso (K0 A) (K0 (double_suspension A))",),
        ("kasparov_kk_theory",
        "forall (A B : CStarAlgebra) (x : KK A B) (y : KK B A), Eq (kasparov_product x y) (id_class A) -> is_kk_equivalence x",),
        ("derivation_leibniz",
        "forall (A : Algebra) (delta : Derivation A) (a b : A), delta (mul a b) = add (mul (delta a) b) (mul a (delta b))",),
        ("inner_derivation",
        "forall (A : Algebra) (x : A), is_inner_derivation (fun a => sub (mul x a) (mul a x))",),
        ("hochschild_smooth_hkr",
        "forall (M : SmoothManifold), iso (HH_n (smooth_functions M)) (differential_forms M n)",),
        ("connes_reconstruction",
        "forall (T : CommutativeSpectralTriple), satisfies_seven_axioms T -> exists_unique (M : RiemannianSpinManifold), iso_spectral_triple T (canonical_triple M)",),
        ("nc_measure_space_semifinite_trace",
        "forall (M : VonNeumannAlgebra), is_semifinite M -> exists (tau : NormalTrace M), is_semifinite_trace tau",),
    ]
}
#[allow(dead_code)]
pub(super) fn gcd_i64(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd_i64(b, a % b)
    }
}
/// Known Baum-Connes results.
#[allow(dead_code)]
pub fn known_baum_connes_results() -> Vec<BaumConnesStatus> {
    vec![
        BaumConnesStatus::known("amenable groups", true, true),
        BaumConnesStatus::known("free groups", true, true),
        BaumConnesStatus::known("hyperbolic groups (Connes-Kasparov)", true, false),
        BaumConnesStatus::known("CAT(0) cubical groups", true, true),
        BaumConnesStatus::unknown("property T groups (general)"),
        BaumConnesStatus::unknown("Thompson's group F"),
    ]
}
#[allow(dead_code)]
pub(super) fn catalan(n: usize) -> u64 {
    let mut result = 1u64;
    for i in 0..n {
        result = result * (2 * n - i) as u64 / (i + 1) as u64;
    }
    result / (n + 1) as u64
}
/// Summary of key NCG theorems for documentation.
#[allow(dead_code)]
pub fn ncg_theorem_summary() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "Connes Reconstruction",
            "Commutative spectral triples <-> Riemannian spin manifolds",
        ),
        (
            "Connes Distance Formula",
            "Geodesic dist = sup{|f(p)-f(q)| : ||[D,f]||<=1}",
        ),
        (
            "Connes-Moscovici Index",
            "Local index formula for spectral triples",
        ),
        (
            "Baum-Connes (amenable)",
            "K_*(C*_r G) = K^top_*(BG) for amenable G",
        ),
        (
            "Tomita-Takesaki",
            "Modular automorphism group exists for any cyclic separating vector",
        ),
        (
            "Woronowicz",
            "Compact quantum groups have unique Haar state",
        ),
        (
            "Kasparov KK",
            "KK(A,B) bivariant K-theory; Kasparov product associative",
        ),
        (
            "Effros-Ruan",
            "Operator space duality extends Banach space duality",
        ),
        (
            "Haagerup",
            "R_omega factor is unique hyperfinite II_1 factor",
        ),
        (
            "Connes III_lambda",
            "Outer automorphism group action classifies type III_lambda factors",
        ),
    ]
}
#[cfg(test)]
mod ncg_ext_tests {
    use super::*;
    #[test]
    fn test_nc_torus() {
        let t = NoncommutativeTorusData::new(0.0);
        assert!(t.is_rational());
        assert_eq!(t.k0_group_rank(), 2);
    }
    #[test]
    fn test_factor_types() {
        let f = FactorType::TypeII1;
        assert!(f.has_finite_trace());
        assert!(f.is_hyperfinite());
        let f3 = FactorType::TypeIII(0);
        assert!(!f3.has_finite_trace());
    }
    #[test]
    fn test_dirac_operator() {
        let d = DiracOperatorData::new("S^4", 4);
        assert_eq!(d.spinor_bundle_rank, 4);
        assert!(d.is_self_adjoint());
    }
    #[test]
    fn test_semicircle_density() {
        let s = SemicircleDistribution::new(2.0);
        let mid = s.density_at(0.0);
        assert!(mid > 0.0);
        let out = s.density_at(3.0);
        assert_eq!(out, 0.0);
    }
    #[test]
    fn test_ncg_summary_nonempty() {
        let summary = ncg_theorem_summary();
        assert!(!summary.is_empty());
    }
    #[test]
    fn test_baum_connes_known() {
        let results = known_baum_connes_results();
        assert!(!results.is_empty());
        let amenable = &results[0];
        assert!(amenable.implies_novikov_conjecture());
    }
}
