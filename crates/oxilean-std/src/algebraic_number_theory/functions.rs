//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Environment, Expr, Name};

use super::types::{
    AlgebraicInteger, ClassFieldTowerChecker, ClassGroupSim, DirichletCharacter,
    GaloisCohomologyH1, IdealFactor, IwasawaInvariantsComputer, NormResidueMap, NumberField,
    PrimeFactorization, SelmerGroupBound,
};

pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(oxilean_kernel::Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(oxilean_kernel::Level::succ(oxilean_kernel::Level::zero()))
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// `AlgebraicNumberField : Type` — an algebraic number field K/Q.
pub fn algebraic_number_field_ty() -> Expr {
    type0()
}
/// `RingOfIntegers : AlgebraicNumberField → Type` — the ring of integers O_K.
pub fn ring_of_integers_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `ClassGroup : AlgebraicNumberField → Type` — the ideal class group Cl(K).
pub fn class_group_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `ClassNumber : AlgebraicNumberField → Nat` — the class number h(K) = |Cl(K)|.
pub fn class_number_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), nat_ty())
}
/// `MinkowskiBound : AlgebraicNumberField → Real` — bound on ideal norms.
pub fn minkowski_bound_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), cst("Real"))
}
/// `UnitGroup : AlgebraicNumberField → Type` — the unit group O_K^×.
pub fn unit_group_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `DirichletUnitTheorem : AlgebraicNumberField → Prop` — rank of unit group theorem.
pub fn dirichlet_unit_theorem_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `FundamentalUnit : AlgebraicNumberField → Type` — generator of unit group.
pub fn fundamental_unit_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `Norm_NF : AlgebraicNumberField → Type → Real` — norm map N_{K/Q}.
pub fn norm_nf_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), cst("Real")))
}
/// `Trace_NF : AlgebraicNumberField → Type → Real` — trace map Tr_{K/Q}.
pub fn trace_nf_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), cst("Real")))
}
/// `Discriminant_NF : AlgebraicNumberField → Int` — discriminant disc(K/Q).
pub fn discriminant_nf_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), cst("Int"))
}
/// `DifferentIdeal : AlgebraicNumberField → Type` — different ideal D_{K/Q}.
pub fn different_ideal_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `RamifiedPrime : Nat → AlgebraicNumberField → Prop` — p ramified in K.
pub fn ramified_prime_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("AlgebraicNumberField"), prop()))
}
/// `SplitPrime : Nat → AlgebraicNumberField → Prop` — p split in K.
pub fn split_prime_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("AlgebraicNumberField"), prop()))
}
/// `InertPrime : Nat → AlgebraicNumberField → Prop` — p inert in K.
pub fn inert_prime_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("AlgebraicNumberField"), prop()))
}
/// `FrobeniusElement : Nat → AlgebraicNumberField → Type` — Frobenius element Frob_p ∈ Gal(K/Q).
pub fn frobenius_element_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("AlgebraicNumberField"), type0()))
}
/// `ChebotarevDensity : AlgebraicNumberField → Prop` — Chebotarev density theorem.
pub fn chebotarev_density_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `KroneckerWeber : Prop` — Kronecker-Weber theorem (abelian extensions of Q are cyclotomic).
pub fn kronecker_weber_ty() -> Expr {
    prop()
}
/// `Artin_L_function : AlgebraicNumberField → Type → Type` — Artin L-functions.
pub fn artin_l_function_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `BrauerGroup : AlgebraicNumberField → Type` — Brauer group Br(K).
pub fn brauer_group_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `Adeles : AlgebraicNumberField → Type` — adele ring A_K.
pub fn adeles_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `Ideles : AlgebraicNumberField → Type` — idele group I_K.
pub fn ideles_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `CfkNumber : AlgebraicNumberField → Type` — Hecke characters (Größencharaktere).
pub fn cfk_number_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `LSeriesFunction : AlgebraicNumberField → Type → Type` — L-series of algebraic objects.
pub fn l_series_function_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `EichlerShimura : AlgebraicNumberField → Prop` — Eichler-Shimura relation.
pub fn eichler_shimura_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `AlgebraicInteger : Type` — an algebraic integer (root of monic polynomial with integer coefficients).
pub fn algebraic_integer_ty() -> Expr {
    type0()
}
/// `MinimalPolynomial : AlgebraicInteger → Type` — the minimal polynomial of an algebraic integer.
pub fn minimal_polynomial_ty() -> Expr {
    arrow(cst("AlgebraicInteger"), type0())
}
/// `FieldDegree : AlgebraicNumberField → Nat` — the degree \[K : Q\] of a number field.
pub fn field_degree_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), nat_ty())
}
/// `DedekindDomain : Type` — a Dedekind domain (Noetherian, integrally closed, Krull dimension 1).
pub fn dedekind_domain_ty() -> Expr {
    type0()
}
/// `FractionalIdeal : DedekindDomain → Type` — a fractional ideal of a Dedekind domain.
pub fn fractional_ideal_ty() -> Expr {
    arrow(cst("DedekindDomain"), type0())
}
/// `IdealFactorization : DedekindDomain → Prop` — unique factorization of ideals into prime ideals.
pub fn ideal_factorization_ty() -> Expr {
    arrow(cst("DedekindDomain"), prop())
}
/// `PrimeIdealDecomposition : Nat → AlgebraicNumberField → Type`
/// — the factorization of the prime ideal (p) in O_K as a product of prime ideals.
pub fn prime_ideal_decomposition_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("AlgebraicNumberField"), type0()))
}
/// `RamificationIndex : Nat → AlgebraicNumberField → Nat`
/// — the ramification index e(P|p) for a prime P of O_K above p.
pub fn ramification_index_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("AlgebraicNumberField"), nat_ty()))
}
/// `InertialDegree : Nat → AlgebraicNumberField → Nat`
/// — the inertial degree f(P|p) = \[O_K/P : F_p\].
pub fn inertial_degree_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("AlgebraicNumberField"), nat_ty()))
}
/// `DiscriminantRamification : AlgebraicNumberField → Prop`
/// — the discriminant-ramification theorem: p | disc(K) iff p is ramified in K.
pub fn discriminant_ramification_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `RayClassGroup : AlgebraicNumberField → Type → Type`
/// — the ray class group Cl_m(K) modulo a modulus m.
pub fn ray_class_group_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `RayClassField : AlgebraicNumberField → Type → Type`
/// — the ray class field K_m corresponding to a modulus m (class field theory).
pub fn ray_class_field_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `LocalField : Type` — a local field (complete field with discrete valuation, finite residue field).
pub fn local_field_ty() -> Expr {
    type0()
}
/// `GlobalField : Type` — a global field (number field or function field over a finite field).
pub fn global_field_ty() -> Expr {
    type0()
}
/// `Completion : AlgebraicNumberField → Nat → Type`
/// — the p-adic completion K_p (local field at a prime p).
pub fn completion_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `ProductFormula : AlgebraicNumberField → Prop`
/// — the product formula: for all x ≠ 0 in K, ∏_v |x|_v = 1.
pub fn product_formula_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `HasseMinkowski : Prop`
/// — the Hasse-Minkowski theorem (local-global principle for quadratic forms).
pub fn hasse_minkowski_ty() -> Expr {
    prop()
}
/// `QuadraticForm : AlgebraicNumberField → Nat → Type`
/// — a quadratic form of rank n over a number field K.
pub fn quadratic_form_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `HeckeCharacter : AlgebraicNumberField → Type`
/// — a Hecke Grössencharakter (generalized Dirichlet character).
pub fn hecke_character_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `HeckeL_function : AlgebraicNumberField → Type → Type`
/// — Hecke L-function L(s, χ) for a Hecke character χ.
pub fn hecke_l_function_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `TateCohomology : AlgebraicNumberField → Type → Type`
/// — Tate cohomology groups Ĥ^n(G, M) used in class field theory.
pub fn tate_cohomology_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `TateThesis : AlgebraicNumberField → Prop`
/// — Tate's thesis: functional equation and analytic continuation of Hecke L-functions via adeles.
pub fn tate_thesis_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `LocalGlobalPrinciple : AlgebraicNumberField → Prop`
/// — general local-global (Hasse) principle for algebraic varieties over number fields.
pub fn local_global_principle_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `DirichletCharacter : Nat → Type`
/// — a Dirichlet character mod N (homomorphism (Z/NZ)^× → C^×).
pub fn dirichlet_character_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `DirichletL_function : Nat → Type → Type`
/// — Dirichlet L-function L(s, χ) for a character χ mod N.
pub fn dirichlet_l_function_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `AnalyticClassNumberFormula : AlgebraicNumberField → Prop`
/// — the analytic class number formula relating residues of Dedekind zeta to class number and regulator.
pub fn analytic_class_number_formula_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `DedekindZeta : AlgebraicNumberField → Type`
/// — the Dedekind zeta function ζ_K(s) of a number field.
pub fn dedekind_zeta_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `Regulator : AlgebraicNumberField → cst("Real")`
/// — the regulator R_K of a number field (covolume of the unit lattice).
pub fn regulator_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), cst("Real"))
}
/// `IdeleClassGroup : AlgebraicNumberField → Type`
/// — the idele class group C_K = I_K / K^× (key object in class field theory).
pub fn idele_class_group_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `GlobalReciprocityLaw : AlgebraicNumberField → Prop`
/// — global Artin reciprocity: the Artin map on the idele class group induces the abelianization of the absolute Galois group.
pub fn global_reciprocity_law_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `NormResidueSymbol : AlgebraicNumberField → Nat → Prop`
/// — the norm residue (Hilbert) symbol and its properties over a number field.
pub fn norm_residue_symbol_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), prop()))
}
/// `IwasawaGammaExtension : AlgebraicNumberField → Type`
/// — the Γ-extension K_∞/K (Z_p-extension in Iwasawa theory).
pub fn iwasawa_gamma_extension_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `IwasawaMuInvariant : AlgebraicNumberField → Nat → Nat`
/// — the Iwasawa μ-invariant of the cyclotomic Z_p-extension.
pub fn iwasawa_mu_invariant_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), nat_ty()))
}
/// `IwasawaLambdaInvariant : AlgebraicNumberField → Nat → Nat`
/// — the Iwasawa λ-invariant of the cyclotomic Z_p-extension.
pub fn iwasawa_lambda_invariant_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), nat_ty()))
}
/// `KubotaLeopoldt_pAdicL : Nat → Type → Type`
/// — the Kubota-Leopoldt p-adic L-function interpolating Dirichlet L-values.
pub fn kubota_leopoldt_padic_l_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// `IwasawaModule : AlgebraicNumberField → Nat → Type`
/// — an Iwasawa module (finitely generated Λ-module) for the cyclotomic Z_p-extension.
pub fn iwasawa_module_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `EllipticCurveNF : AlgebraicNumberField → Type`
/// — an elliptic curve E defined over a number field K.
pub fn elliptic_curve_nf_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `AnalyticRank : AlgebraicNumberField → Type → Nat`
/// — the analytic rank of an elliptic curve E/K (order of vanishing of L(E,s) at s=1).
pub fn analytic_rank_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), nat_ty()))
}
/// `AlgebraicRank : AlgebraicNumberField → Type → Nat`
/// — the algebraic rank of E/K, i.e., rank of the Mordell-Weil group E(K).
pub fn algebraic_rank_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), nat_ty()))
}
/// `BSDConjecture : AlgebraicNumberField → Type → Prop`
/// — the Birch and Swinnerton-Dyer conjecture: analytic rank = algebraic rank.
pub fn bsd_conjecture_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), prop()))
}
/// `SelmerGroup : AlgebraicNumberField → Type → Nat → Type`
/// — the n-Selmer group Sel^n(E/K) ⊆ H^1(G_K, E\[n\]).
pub fn selmer_group_ty() -> Expr {
    arrow(
        cst("AlgebraicNumberField"),
        arrow(type0(), arrow(nat_ty(), type0())),
    )
}
/// `CuspForm : Nat → Nat → Type`
/// — a cusp form of weight k and level N (holomorphic modular form vanishing at cusps).
pub fn cusp_form_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `HeckeOperator : Nat → Nat → Type → Type`
/// — the Hecke operator T_n acting on modular forms of weight k and level N.
pub fn hecke_operator_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(type0(), type0())))
}
/// `HeckeL_FunctionGL2 : Nat → Nat → Type`
/// — the Hecke L-function L(s, f) for a Hecke eigenform f of weight k, level N.
pub fn hecke_l_function_gl2_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `RamanujanConjectureGL2 : Nat → Nat → Prop`
/// — the Ramanujan conjecture for GL(2): Hecke eigenvalues satisfy |a_p| ≤ 2p^{(k-1)/2}.
pub fn ramanujan_conjecture_gl2_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `pAdicRepresentation : AlgebraicNumberField → Nat → Type`
/// — a p-adic Galois representation ρ : Gal(K̄/K) → GL_n(Q_p).
pub fn padic_representation_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `CrystallineRepresentation : AlgebraicNumberField → Nat → Prop`
/// — a crystalline p-adic representation (Fontaine's Bcrys-admissible).
pub fn crystalline_representation_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), prop()))
}
/// `DeRhamRepresentation : AlgebraicNumberField → Nat → Prop`
/// — a de Rham p-adic representation (BdR-admissible).
pub fn de_rham_representation_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), prop()))
}
/// `BarsottiTateRepresentation : AlgebraicNumberField → Nat → Prop`
/// — a Barsotti-Tate (flat) p-adic representation.
pub fn barsotti_tate_representation_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), prop()))
}
/// `GaloisCohomologyH1 : AlgebraicNumberField → Type → Type`
/// — H^1(G_K, M): the first Galois cohomology group with coefficients in a G_K-module M.
pub fn galois_cohomology_h1_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `TateLocalDuality : AlgebraicNumberField → Nat → Prop`
/// — Tate local duality: H^r(G_{K_v}, M) × H^{2-r}(G_{K_v}, M^*(1)) → Q/Z is perfect.
pub fn tate_local_duality_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), prop()))
}
/// `EulerCharacteristicFormula : AlgebraicNumberField → Type → Prop`
/// — the Euler characteristic formula for Galois cohomology over local fields.
pub fn euler_characteristic_formula_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), prop()))
}
/// `CharacteristicIdeal : AlgebraicNumberField → Nat → Type`
/// — the characteristic ideal char(X) of an Iwasawa module X over Λ = Z_p[\[T\]].
pub fn characteristic_ideal_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `FittingIdeal : AlgebraicNumberField → Type → Type`
/// — the Fitting ideal Fitt(M) of a finitely presented module M.
pub fn fitting_ideal_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `IwasawaMainConjecture : AlgebraicNumberField → Nat → Prop`
/// — the Iwasawa main conjecture: char(X_∞) = (ξ_p) as ideals in Λ.
pub fn iwasawa_main_conjecture_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), prop()))
}
/// `GolodShafarevichInequality : AlgebraicNumberField → Prop`
/// — the Golod-Shafarevich inequality: d(G)^2 > 4·r(G) implies the p-tower is infinite.
pub fn golod_shafarevich_inequality_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `InfiniteClassFieldTower : AlgebraicNumberField → Prop`
/// — the p-class field tower K ⊂ K^1 ⊂ K^2 ⊂ … is infinite.
pub fn infinite_class_field_tower_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `ArtinSymbol : AlgebraicNumberField → Nat → Type`
/// — the Artin symbol (K/Q, p) ∈ Gal(K/Q) for an unramified prime p.
pub fn artin_symbol_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `LocalGlobalCompatibility : AlgebraicNumberField → Prop`
/// — local-global compatibility of the Langlands correspondence.
pub fn local_global_compatibility_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `HeegnerPoint : AlgebraicNumberField → Type → Type`
/// — a Heegner point on an elliptic curve E over K (from CM points on modular curves).
pub fn heegner_point_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `DarmonPoint : AlgebraicNumberField → Type → Type`
/// — a Darmon (Stark-Heegner) point: p-adic analogue of Heegner points over real quadratic fields.
pub fn darmon_point_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `BlochKatoExponential : AlgebraicNumberField → Nat → Type`
/// — the Bloch-Kato exponential map exp : D_dR(V) → H^1(G_K, V).
pub fn bloch_kato_exponential_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `BlochKatoLogarithm : AlgebraicNumberField → Nat → Type`
/// — the Bloch-Kato logarithm map log : H^1_f(G_K, V) → D_dR(V).
pub fn bloch_kato_logarithm_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `BlochKatoSelmer : AlgebraicNumberField → Type → Type`
/// — the Bloch-Kato Selmer group H^1_f(G_K, V) ⊆ H^1(G_K, V).
pub fn bloch_kato_selmer_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `SUnitGroup : AlgebraicNumberField → Type → Type`
/// — the S-unit group O_{K,S}^× for a set of places S.
pub fn s_unit_group_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `SUnitTheorem : AlgebraicNumberField → Type → Prop`
/// — the S-unit theorem: O_{K,S}^× is a finitely generated abelian group of rank |S| + r1 + r2 - 1.
pub fn s_unit_theorem_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), prop()))
}
/// `ArithmeticIntersection : AlgebraicNumberField → Type`
/// — arithmetic intersection pairing on an arithmetic surface (Arakelov theory).
pub fn arithmetic_intersection_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `ArakelovRiemannRoch : AlgebraicNumberField → Prop`
/// — the Arakelov-Riemann-Roch theorem for arithmetic surfaces.
pub fn arakelov_riemann_roch_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// Populate `env` with all algebraic number theory axioms and theorems.
pub fn register_algebraic_number_theory(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("AlgebraicNumberField", algebraic_number_field_ty()),
        ("RingOfIntegers", ring_of_integers_ty()),
        ("ClassGroup", class_group_ty()),
        ("ClassNumber", class_number_ty()),
        ("MinkowskiBound", minkowski_bound_ty()),
        ("UnitGroup", unit_group_ty()),
        ("DirichletUnitTheorem", dirichlet_unit_theorem_ty()),
        ("FundamentalUnit", fundamental_unit_ty()),
        ("Norm_NF", norm_nf_ty()),
        ("Trace_NF", trace_nf_ty()),
        ("Discriminant_NF", discriminant_nf_ty()),
        ("DifferentIdeal", different_ideal_ty()),
        ("RamifiedPrime", ramified_prime_ty()),
        ("SplitPrime", split_prime_ty()),
        ("InertPrime", inert_prime_ty()),
        ("FrobeniusElement", frobenius_element_ty()),
        ("ChebotarevDensity", chebotarev_density_ty()),
        ("KroneckerWeber", kronecker_weber_ty()),
        ("Artin_L_function", artin_l_function_ty()),
        ("BrauerGroup", brauer_group_ty()),
        ("Adeles", adeles_ty()),
        ("Ideles", ideles_ty()),
        ("CfkNumber", cfk_number_ty()),
        ("LSeriesFunction", l_series_function_ty()),
        ("EichlerShimura", eichler_shimura_ty()),
        ("AlgebraicInteger", algebraic_integer_ty()),
        ("MinimalPolynomial", minimal_polynomial_ty()),
        ("FieldDegree", field_degree_ty()),
        ("DedekindDomain", dedekind_domain_ty()),
        ("FractionalIdeal", fractional_ideal_ty()),
        ("IdealFactorization", ideal_factorization_ty()),
        ("PrimeIdealDecomposition", prime_ideal_decomposition_ty()),
        ("RamificationIndex", ramification_index_ty()),
        ("InertialDegree", inertial_degree_ty()),
        ("DiscriminantRamification", discriminant_ramification_ty()),
        ("RayClassGroup", ray_class_group_ty()),
        ("RayClassField", ray_class_field_ty()),
        ("LocalField", local_field_ty()),
        ("GlobalField", global_field_ty()),
        ("Completion", completion_ty()),
        ("ProductFormula", product_formula_ty()),
        ("HasseMinkowski", hasse_minkowski_ty()),
        ("QuadraticForm", quadratic_form_ty()),
        ("HeckeCharacter", hecke_character_ty()),
        ("HeckeL_function", hecke_l_function_ty()),
        ("TateCohomology", tate_cohomology_ty()),
        ("TateThesis", tate_thesis_ty()),
        ("LocalGlobalPrinciple", local_global_principle_ty()),
        ("DirichletCharacter", dirichlet_character_ty()),
        ("DirichletL_function", dirichlet_l_function_ty()),
        (
            "AnalyticClassNumberFormula",
            analytic_class_number_formula_ty(),
        ),
        ("DedekindZeta", dedekind_zeta_ty()),
        ("Regulator", regulator_ty()),
        ("IdeleClassGroup", idele_class_group_ty()),
        ("GlobalReciprocityLaw", global_reciprocity_law_ty()),
        ("NormResidueSymbol", norm_residue_symbol_ty()),
        ("IwasawaGammaExtension", iwasawa_gamma_extension_ty()),
        ("IwasawaMuInvariant", iwasawa_mu_invariant_ty()),
        ("IwasawaLambdaInvariant", iwasawa_lambda_invariant_ty()),
        ("KubotaLeopoldt_pAdicL", kubota_leopoldt_padic_l_ty()),
        ("IwasawaModule", iwasawa_module_ty()),
        ("EllipticCurveNF", elliptic_curve_nf_ty()),
        ("AnalyticRank", analytic_rank_ty()),
        ("AlgebraicRank", algebraic_rank_ty()),
        ("BSDConjecture", bsd_conjecture_ty()),
        ("SelmerGroup", selmer_group_ty()),
        ("CuspForm", cusp_form_ty()),
        ("HeckeOperator", hecke_operator_ty()),
        ("HeckeL_FunctionGL2", hecke_l_function_gl2_ty()),
        ("RamanujanConjectureGL2", ramanujan_conjecture_gl2_ty()),
        ("pAdicRepresentation", padic_representation_ty()),
        ("CrystallineRepresentation", crystalline_representation_ty()),
        ("DeRhamRepresentation", de_rham_representation_ty()),
        (
            "BarsottiTateRepresentation",
            barsotti_tate_representation_ty(),
        ),
        ("GaloisCohomologyH1", galois_cohomology_h1_ty()),
        ("TateLocalDuality", tate_local_duality_ty()),
        (
            "EulerCharacteristicFormula",
            euler_characteristic_formula_ty(),
        ),
        ("CharacteristicIdeal", characteristic_ideal_ty()),
        ("FittingIdeal", fitting_ideal_ty()),
        ("IwasawaMainConjecture", iwasawa_main_conjecture_ty()),
        (
            "GolodShafarevichInequality",
            golod_shafarevich_inequality_ty(),
        ),
        ("InfiniteClassFieldTower", infinite_class_field_tower_ty()),
        ("ArtinSymbol", artin_symbol_ty()),
        ("LocalGlobalCompatibility", local_global_compatibility_ty()),
        ("HeegnerPoint", heegner_point_ty()),
        ("DarmonPoint", darmon_point_ty()),
        ("BlochKatoExponential", bloch_kato_exponential_ty()),
        ("BlochKatoLogarithm", bloch_kato_logarithm_ty()),
        ("BlochKatoSelmer", bloch_kato_selmer_ty()),
        ("SUnitGroup", s_unit_group_ty()),
        ("SUnitTheorem", s_unit_theorem_ty()),
        ("ArithmeticIntersection", arithmetic_intersection_ty()),
        ("ArakelovRiemannRoch", arakelov_riemann_roch_ty()),
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
/// Compute the greatest common divisor of two non-negative integers.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// Compute the least common multiple of two non-negative integers.
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}
/// Fast modular exponentiation: compute base^exp mod modulus.
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp /= 2;
        base = base * base % modulus;
    }
    result
}
/// `ArtinReciprocityMap : AlgebraicNumberField → Type`
/// — the Artin reciprocity map rec_K : I_K → Gal(K^ab/K) (global class field theory).
pub fn ant_ext_artin_reciprocity_map_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), type0())
}
/// `ClassFieldMainTheorem : AlgebraicNumberField → Prop`
/// — the main theorem of class field theory: abelian extensions of K
/// are in bijection with open subgroups of the idele class group C_K.
pub fn ant_ext_class_field_main_theorem_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `AbelianExtension : AlgebraicNumberField → Type → Type`
/// — an abelian Galois extension L/K classified by class field theory.
pub fn ant_ext_abelian_extension_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `NormGroupCorrespondence : AlgebraicNumberField → Type → Prop`
/// — the norm group N_{L/K}(I_L) ↔ open subgroup correspondence.
pub fn ant_ext_norm_group_correspondence_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), prop()))
}
/// `ArtinConductor : AlgebraicNumberField → Type → Nat`
/// — the Artin conductor f(ρ, K) of a Galois representation ρ.
pub fn ant_ext_artin_conductor_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), nat_ty()))
}
/// `FunctionalEquationHeckeL : AlgebraicNumberField → Prop`
/// — the functional equation Λ(s, χ) = ε(s, χ) Λ(1-s, χ̄) for Hecke L-functions.
pub fn ant_ext_functional_equation_hecke_l_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `EpsilonFactor : AlgebraicNumberField → Type → Type`
/// — the epsilon (root number) factor ε(s, ρ, ψ) in the functional equation.
pub fn ant_ext_epsilon_factor_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `LocalEpsilonFactor : AlgebraicNumberField → Nat → Type → Type`
/// — the local epsilon factor ε_v(s, ρ_v, ψ_v) at a place v.
pub fn ant_ext_local_epsilon_factor_ty() -> Expr {
    arrow(
        cst("AlgebraicNumberField"),
        arrow(nat_ty(), arrow(type0(), type0())),
    )
}
/// `LocalLanglandsGL1 : AlgebraicNumberField → Prop`
/// — local Langlands for GL(1): local class field theory (local reciprocity law).
pub fn ant_ext_local_langlands_gl1_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `LocalArtinMap : AlgebraicNumberField → Nat → Type`
/// — the local Artin map art_{K_v} : K_v^× → W_{K_v}^ab for a place v.
pub fn ant_ext_local_artin_map_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `WeilGroup : AlgebraicNumberField → Nat → Type`
/// — the Weil group W_{K_v} of a local field K_v.
pub fn ant_ext_weil_group_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `AutomorphicRepresentation : AlgebraicNumberField → Type → Type`
/// — an automorphic representation π of GL_n over the adeles A_K.
pub fn ant_ext_automorphic_representation_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `GlobalLanglandsCorrespondence : AlgebraicNumberField → Prop`
/// — the global Langlands correspondence (heuristic): automorphic representations
/// of GL_n(A_K) correspond to n-dimensional Galois representations.
pub fn ant_ext_global_langlands_correspondence_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `LanglandsDualGroup : Type → Type`
/// — the Langlands dual group (L-group) ĜL associated to a reductive group G.
pub fn ant_ext_langlands_dual_group_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ModularGaloisRepresentation : Nat → Nat → Type`
/// — the Galois representation ρ_f attached to a modular eigenform f of weight k, level N.
pub fn ant_ext_modular_galois_representation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ShimuraTaniyamaConjecture : AlgebraicNumberField → Type → Prop`
/// — the Shimura-Taniyama-Weil conjecture: every elliptic curve over Q is modular.
pub fn ant_ext_shimura_taniyama_conjecture_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), prop()))
}
/// `FermatsLastTheorem : Prop`
/// — Fermat's Last Theorem: there are no positive integer solutions to x^n + y^n = z^n (n ≥ 3).
pub fn ant_ext_fermats_last_theorem_ty() -> Expr {
    prop()
}
/// `ModularityLiftingTheorem : AlgebraicNumberField → Prop`
/// — Wiles' modularity lifting theorem (Taylor-Wiles): residually modular
/// deformations lift to modular Galois representations.
pub fn ant_ext_modularity_lifting_theorem_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `FaltingsTheorem : AlgebraicNumberField → Prop`
/// — Faltings' theorem (Mordell conjecture): a smooth projective curve of genus ≥ 2
/// over a number field has only finitely many rational points.
pub fn ant_ext_faltings_theorem_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `MordellWeilGroup : AlgebraicNumberField → Type → Type`
/// — the Mordell-Weil group E(K) of rational points on an elliptic curve E over K.
pub fn ant_ext_mordell_weil_group_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `NaiveMordellWeilRank : AlgebraicNumberField → Type → Nat`
/// — the rank of the Mordell-Weil group (the free part of E(K)).
pub fn ant_ext_mordell_weil_rank_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), nat_ty()))
}
/// `pAdicLFunction : AlgebraicNumberField → Nat → Type`
/// — a p-adic L-function L_p(s, χ) interpolating classical L-values.
pub fn ant_ext_padic_l_function_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `pAdicRegulator : AlgebraicNumberField → Nat → cst("Real")`
/// — the p-adic regulator R_p(K) appearing in the p-adic BSD formula.
pub fn ant_ext_padic_regulator_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), cst("Real")))
}
/// `pAdicHeightPairing : AlgebraicNumberField → Nat → Type`
/// — the p-adic height pairing on the Mordell-Weil group (p-adic BSD).
pub fn ant_ext_padic_height_pairing_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), type0()))
}
/// `BSDLeadingCoefficient : AlgebraicNumberField → Type → cst("Real")`
/// — the leading coefficient in the Taylor expansion of L(E, s) at s = 1 (BSD).
pub fn ant_ext_bsd_leading_coefficient_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), cst("Real")))
}
/// `TamagawaNumber : AlgebraicNumberField → Type → Nat → Nat`
/// — the Tamagawa number c_v of E/Q_v (local factor in the BSD leading coefficient).
pub fn ant_ext_tamagawa_number_ty() -> Expr {
    arrow(
        cst("AlgebraicNumberField"),
        arrow(type0(), arrow(nat_ty(), nat_ty())),
    )
}
/// `ShaTateWeilGroup : AlgebraicNumberField → Type → Type`
/// — the Shafarevich-Tate group Ш(E/K) = ker(H^1(G_K, E) → ∏_v H^1(G_{K_v}, E)).
pub fn ant_ext_sha_tate_weil_group_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `TateDualityPairing : AlgebraicNumberField → Type → Prop`
/// — Tate's global duality (Cassels-Tate pairing): Ш(E/K) is finite and self-dual.
pub fn ant_ext_tate_duality_pairing_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), prop()))
}
/// `CasselsEulerCharacteristic : AlgebraicNumberField → Prop`
/// — the Cassels Euler characteristic formula for Selmer groups over number fields.
pub fn ant_ext_cassels_euler_characteristic_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `WeilPairing : AlgebraicNumberField → Type → Nat → Type`
/// — the Weil pairing e_n : E\[n\] × E\[n\] → μ_n on n-torsion points.
pub fn ant_ext_weil_pairing_ty() -> Expr {
    arrow(
        cst("AlgebraicNumberField"),
        arrow(type0(), arrow(nat_ty(), type0())),
    )
}
/// `StarkRegulator : AlgebraicNumberField → Type → cst("Real")`
/// — the Stark regulator appearing in Stark's conjectures on L-function leading terms.
pub fn ant_ext_stark_regulator_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), cst("Real")))
}
/// `StarkConjecture : AlgebraicNumberField → Prop`
/// — Stark's conjecture: the leading coefficient of an Artin L-function at s=0
/// is related to a specific unit (Stark unit) in an abelian extension.
pub fn ant_ext_stark_conjecture_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), prop())
}
/// `StarkUnit : AlgebraicNumberField → Type → Type`
/// — a Stark unit ε ∈ L^× predicted by the Stark conjecture for an abelian extension L/K.
pub fn ant_ext_stark_unit_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `ExplicitReciprocityLaw : AlgebraicNumberField → Nat → Prop`
/// — an explicit reciprocity law (Perrin-Riou, Kato) relating p-adic L-functions
/// to Selmer groups via Euler systems.
pub fn ant_ext_explicit_reciprocity_law_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(nat_ty(), prop()))
}
/// `EulerSystem : AlgebraicNumberField → Type → Type`
/// — an Euler system: a compatible collection of cohomology classes for a Galois representation.
pub fn ant_ext_euler_system_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// `KolyvaginSystem : AlgebraicNumberField → Type → Type`
/// — a Kolyvagin system derived from an Euler system via Kolyvagin's derivative operator.
pub fn ant_ext_kolyvagin_system_ty() -> Expr {
    arrow(cst("AlgebraicNumberField"), arrow(type0(), type0()))
}
/// Register all extended algebraic number theory axioms into `env`.
pub fn register_algebraic_number_theory_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("ArtinReciprocityMap", ant_ext_artin_reciprocity_map_ty()),
        (
            "ClassFieldMainTheorem",
            ant_ext_class_field_main_theorem_ty(),
        ),
        ("AbelianExtension", ant_ext_abelian_extension_ty()),
        (
            "NormGroupCorrespondence",
            ant_ext_norm_group_correspondence_ty(),
        ),
        ("ArtinConductor", ant_ext_artin_conductor_ty()),
        (
            "FunctionalEquationHeckeL",
            ant_ext_functional_equation_hecke_l_ty(),
        ),
        ("EpsilonFactor", ant_ext_epsilon_factor_ty()),
        ("LocalEpsilonFactor", ant_ext_local_epsilon_factor_ty()),
        ("LocalLanglandsGL1", ant_ext_local_langlands_gl1_ty()),
        ("LocalArtinMap", ant_ext_local_artin_map_ty()),
        ("WeilGroup", ant_ext_weil_group_ty()),
        (
            "AutomorphicRepresentation",
            ant_ext_automorphic_representation_ty(),
        ),
        (
            "GlobalLanglandsCorrespondence",
            ant_ext_global_langlands_correspondence_ty(),
        ),
        ("LanglandsDualGroup", ant_ext_langlands_dual_group_ty()),
        (
            "ModularGaloisRepresentation",
            ant_ext_modular_galois_representation_ty(),
        ),
        (
            "ShimuraTaniyamaConjecture",
            ant_ext_shimura_taniyama_conjecture_ty(),
        ),
        ("FermatsLastTheorem", ant_ext_fermats_last_theorem_ty()),
        (
            "ModularityLiftingTheorem",
            ant_ext_modularity_lifting_theorem_ty(),
        ),
        ("FaltingsTheorem", ant_ext_faltings_theorem_ty()),
        ("MordellWeilGroup", ant_ext_mordell_weil_group_ty()),
        ("NaiveMordellWeilRank", ant_ext_mordell_weil_rank_ty()),
        ("pAdicLFunction", ant_ext_padic_l_function_ty()),
        ("pAdicRegulator", ant_ext_padic_regulator_ty()),
        ("pAdicHeightPairing", ant_ext_padic_height_pairing_ty()),
        (
            "BSDLeadingCoefficient",
            ant_ext_bsd_leading_coefficient_ty(),
        ),
        ("TamagawaNumber", ant_ext_tamagawa_number_ty()),
        ("ShaTateWeilGroup", ant_ext_sha_tate_weil_group_ty()),
        ("TateDualityPairing", ant_ext_tate_duality_pairing_ty()),
        (
            "CasselsEulerCharacteristic",
            ant_ext_cassels_euler_characteristic_ty(),
        ),
        ("WeilPairing", ant_ext_weil_pairing_ty()),
        ("StarkRegulator", ant_ext_stark_regulator_ty()),
        ("StarkConjecture", ant_ext_stark_conjecture_ty()),
        ("StarkUnit", ant_ext_stark_unit_ty()),
        (
            "ExplicitReciprocityLaw",
            ant_ext_explicit_reciprocity_law_ty(),
        ),
        ("EulerSystem", ant_ext_euler_system_ty()),
        ("KolyvaginSystem", ant_ext_kolyvagin_system_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add {}: {:?}", name, e))?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_algebraic_number_theory_registration() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("AlgebraicNumberField")).is_some());
        assert!(env.get(&Name::str("RingOfIntegers")).is_some());
    }
    #[test]
    fn test_class_group_and_number() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("ClassGroup")).is_some());
        assert!(env.get(&Name::str("ClassNumber")).is_some());
    }
    #[test]
    fn test_unit_theory() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("UnitGroup")).is_some());
        assert!(env.get(&Name::str("DirichletUnitTheorem")).is_some());
        assert!(env.get(&Name::str("FundamentalUnit")).is_some());
    }
    #[test]
    fn test_norm_trace_discriminant() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("Norm_NF")).is_some());
        assert!(env.get(&Name::str("Trace_NF")).is_some());
        assert!(env.get(&Name::str("Discriminant_NF")).is_some());
        assert!(env.get(&Name::str("DifferentIdeal")).is_some());
    }
    #[test]
    fn test_prime_splitting() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("RamifiedPrime")).is_some());
        assert!(env.get(&Name::str("SplitPrime")).is_some());
        assert!(env.get(&Name::str("InertPrime")).is_some());
        assert!(env.get(&Name::str("FrobeniusElement")).is_some());
    }
    #[test]
    fn test_density_theorems() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("ChebotarevDensity")).is_some());
        assert!(env.get(&Name::str("KroneckerWeber")).is_some());
    }
    #[test]
    fn test_l_functions_and_adeles() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("Artin_L_function")).is_some());
        assert!(env.get(&Name::str("BrauerGroup")).is_some());
        assert!(env.get(&Name::str("Adeles")).is_some());
        assert!(env.get(&Name::str("Ideles")).is_some());
        assert!(env.get(&Name::str("LSeriesFunction")).is_some());
    }
    #[test]
    fn test_eichler_shimura_and_hecke() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("CfkNumber")).is_some());
        assert!(env.get(&Name::str("EichlerShimura")).is_some());
    }
    #[test]
    fn test_new_axioms_algebraic_integer() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("AlgebraicInteger")).is_some());
        assert!(env.get(&Name::str("MinimalPolynomial")).is_some());
        assert!(env.get(&Name::str("FieldDegree")).is_some());
    }
    #[test]
    fn test_new_axioms_dedekind() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("DedekindDomain")).is_some());
        assert!(env.get(&Name::str("FractionalIdeal")).is_some());
        assert!(env.get(&Name::str("IdealFactorization")).is_some());
    }
    #[test]
    fn test_new_axioms_ramification() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("PrimeIdealDecomposition")).is_some());
        assert!(env.get(&Name::str("RamificationIndex")).is_some());
        assert!(env.get(&Name::str("InertialDegree")).is_some());
        assert!(env.get(&Name::str("DiscriminantRamification")).is_some());
    }
    #[test]
    fn test_new_axioms_class_field_theory() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("RayClassGroup")).is_some());
        assert!(env.get(&Name::str("RayClassField")).is_some());
        assert!(env.get(&Name::str("IdeleClassGroup")).is_some());
        assert!(env.get(&Name::str("GlobalReciprocityLaw")).is_some());
    }
    #[test]
    fn test_new_axioms_local_global() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("LocalField")).is_some());
        assert!(env.get(&Name::str("GlobalField")).is_some());
        assert!(env.get(&Name::str("Completion")).is_some());
        assert!(env.get(&Name::str("ProductFormula")).is_some());
        assert!(env.get(&Name::str("HasseMinkowski")).is_some());
        assert!(env.get(&Name::str("LocalGlobalPrinciple")).is_some());
    }
    #[test]
    fn test_new_axioms_l_functions() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("HeckeCharacter")).is_some());
        assert!(env.get(&Name::str("HeckeL_function")).is_some());
        assert!(env.get(&Name::str("TateCohomology")).is_some());
        assert!(env.get(&Name::str("TateThesis")).is_some());
        assert!(env.get(&Name::str("DirichletCharacter")).is_some());
        assert!(env.get(&Name::str("DirichletL_function")).is_some());
        assert!(env.get(&Name::str("DedekindZeta")).is_some());
    }
    #[test]
    fn test_new_axioms_analytic() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("AnalyticClassNumberFormula")).is_some());
        assert!(env.get(&Name::str("Regulator")).is_some());
        assert!(env.get(&Name::str("NormResidueSymbol")).is_some());
        assert!(env.get(&Name::str("QuadraticForm")).is_some());
    }
    #[test]
    fn test_number_field_basic() {
        let q = NumberField::rationals();
        assert_eq!(q.degree, 1);
        assert_eq!(q.discriminant, 1);
        assert_eq!(q.unit_rank(), 0);
        let gauss = NumberField::gaussian();
        assert_eq!(gauss.degree, 2);
        assert_eq!(gauss.discriminant, -4);
        let (r1, r2) = gauss.signature();
        assert_eq!(r1, 0);
        assert_eq!(r2, 1);
        assert_eq!(gauss.unit_rank(), 0);
    }
    #[test]
    fn test_number_field_minkowski() {
        let gauss = NumberField::gaussian();
        let mb = gauss.minkowski_bound();
        assert!(mb > 0.0);
    }
    #[test]
    fn test_algebraic_integer_eval() {
        let sqrt2 = AlgebraicInteger::new(vec![-2, 0]);
        assert_eq!(sqrt2.degree(), 2);
        assert_eq!(sqrt2.eval_at(1), -1);
        assert_eq!(sqrt2.eval_at(2), 2);
    }
    #[test]
    fn test_algebraic_integer_norm_trace() {
        let phi = AlgebraicInteger::new(vec![-1, -1]);
        assert_eq!(phi.norm(), -1);
        assert_eq!(phi.trace(), 1);
    }
    #[test]
    fn test_ideal_factor_norm() {
        let f = IdealFactor::new(7, 1, 2);
        assert_eq!(f.norm(), 49);
        assert!(!f.is_ramified());
        assert!(!f.is_split());
        assert!(f.is_inert(2));
    }
    #[test]
    fn test_prime_factorization_identity() {
        let factors = vec![IdealFactor::new(5, 2, 1)];
        let factorization = PrimeFactorization::new(5, factors);
        assert!(factorization.check_identity(2));
        assert!(factorization.is_totally_ramified(2));
        assert!(!factorization.is_totally_split());
    }
    #[test]
    fn test_class_group_sim() {
        let trivial = ClassGroupSim::trivial();
        assert_eq!(trivial.class_number(), 1);
        assert!(trivial.is_trivial());
        let cyclic3 = ClassGroupSim::cyclic(3);
        assert_eq!(cyclic3.class_number(), 3);
        assert_eq!(cyclic3.rank(), 1);
        assert!(cyclic3.contains_element(&[0]));
        assert!(cyclic3.contains_element(&[2]));
        assert!(!cyclic3.contains_element(&[3]));
        assert_eq!(cyclic3.element_order(&[1]), 3);
        assert_eq!(cyclic3.element_order(&[0]), 1);
    }
    #[test]
    fn test_dirichlet_character_principal() {
        let chi = DirichletCharacter::principal(5);
        assert_eq!(chi.modulus, 5);
        assert!(chi.is_principal());
        assert_eq!(chi.eval(1), 1);
        assert_eq!(chi.eval(2), 1);
        assert_eq!(chi.eval(5), 0);
        assert_eq!(chi.phi(), 4);
    }
    #[test]
    fn test_gcd_lcm() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(0, 7), 7);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(0, 5), 0);
    }
    #[test]
    fn test_total_axiom_count() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        let new_names = [
            "AlgebraicInteger",
            "MinimalPolynomial",
            "FieldDegree",
            "DedekindDomain",
            "FractionalIdeal",
            "IdealFactorization",
            "PrimeIdealDecomposition",
            "RamificationIndex",
            "InertialDegree",
            "DiscriminantRamification",
            "RayClassGroup",
            "RayClassField",
            "LocalField",
            "GlobalField",
            "Completion",
            "ProductFormula",
            "HasseMinkowski",
            "QuadraticForm",
            "HeckeCharacter",
            "HeckeL_function",
            "TateCohomology",
            "TateThesis",
            "LocalGlobalPrinciple",
            "DirichletCharacter",
            "DirichletL_function",
            "AnalyticClassNumberFormula",
            "DedekindZeta",
            "Regulator",
            "IdeleClassGroup",
            "GlobalReciprocityLaw",
            "NormResidueSymbol",
        ];
        for name in new_names {
            assert!(
                env.get(&Name::str(name)).is_some(),
                "Missing axiom: {}",
                name
            );
        }
    }
    #[test]
    fn test_advanced_axioms_iwasawa() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("IwasawaGammaExtension")).is_some());
        assert!(env.get(&Name::str("IwasawaMuInvariant")).is_some());
        assert!(env.get(&Name::str("IwasawaLambdaInvariant")).is_some());
        assert!(env.get(&Name::str("KubotaLeopoldt_pAdicL")).is_some());
        assert!(env.get(&Name::str("IwasawaModule")).is_some());
        assert!(env.get(&Name::str("IwasawaMainConjecture")).is_some());
    }
    #[test]
    fn test_advanced_axioms_elliptic_bsd() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("EllipticCurveNF")).is_some());
        assert!(env.get(&Name::str("AnalyticRank")).is_some());
        assert!(env.get(&Name::str("AlgebraicRank")).is_some());
        assert!(env.get(&Name::str("BSDConjecture")).is_some());
        assert!(env.get(&Name::str("SelmerGroup")).is_some());
    }
    #[test]
    fn test_advanced_axioms_automorphic() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("CuspForm")).is_some());
        assert!(env.get(&Name::str("HeckeOperator")).is_some());
        assert!(env.get(&Name::str("HeckeL_FunctionGL2")).is_some());
        assert!(env.get(&Name::str("RamanujanConjectureGL2")).is_some());
    }
    #[test]
    fn test_advanced_axioms_padic_rep() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("pAdicRepresentation")).is_some());
        assert!(env.get(&Name::str("CrystallineRepresentation")).is_some());
        assert!(env.get(&Name::str("DeRhamRepresentation")).is_some());
        assert!(env.get(&Name::str("BarsottiTateRepresentation")).is_some());
    }
    #[test]
    fn test_advanced_axioms_galois_cohomology() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("GaloisCohomologyH1")).is_some());
        assert!(env.get(&Name::str("TateLocalDuality")).is_some());
        assert!(env.get(&Name::str("EulerCharacteristicFormula")).is_some());
        assert!(env.get(&Name::str("CharacteristicIdeal")).is_some());
        assert!(env.get(&Name::str("FittingIdeal")).is_some());
    }
    #[test]
    fn test_advanced_axioms_class_field_tower() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("GolodShafarevichInequality")).is_some());
        assert!(env.get(&Name::str("InfiniteClassFieldTower")).is_some());
        assert!(env.get(&Name::str("ArtinSymbol")).is_some());
        assert!(env.get(&Name::str("LocalGlobalCompatibility")).is_some());
    }
    #[test]
    fn test_advanced_axioms_stark_heegner_bloch_kato() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("HeegnerPoint")).is_some());
        assert!(env.get(&Name::str("DarmonPoint")).is_some());
        assert!(env.get(&Name::str("BlochKatoExponential")).is_some());
        assert!(env.get(&Name::str("BlochKatoLogarithm")).is_some());
        assert!(env.get(&Name::str("BlochKatoSelmer")).is_some());
    }
    #[test]
    fn test_advanced_axioms_units_arakelov() {
        let mut env = Environment::new();
        register_algebraic_number_theory(&mut env);
        assert!(env.get(&Name::str("SUnitGroup")).is_some());
        assert!(env.get(&Name::str("SUnitTheorem")).is_some());
        assert!(env.get(&Name::str("ArithmeticIntersection")).is_some());
        assert!(env.get(&Name::str("ArakelovRiemannRoch")).is_some());
    }
    #[test]
    fn test_iwasawa_invariants_regular_prime() {
        let comp = IwasawaInvariantsComputer::new(5, 1);
        assert_eq!(comp.mu_invariant(), 0);
        assert_eq!(comp.lambda_estimate(), 0);
        assert!(!comp.is_irregular_prime());
    }
    #[test]
    fn test_iwasawa_invariants_irregular_prime() {
        let comp = IwasawaInvariantsComputer::new(37, 1);
        assert_eq!(comp.mu_invariant(), 0);
        assert_eq!(comp.lambda_estimate(), 1);
        assert!(comp.is_irregular_prime());
    }
    #[test]
    fn test_galois_cohomology_h1_cyclic() {
        let h1 = GaloisCohomologyH1::new(6, 4);
        assert_eq!(h1.h1_order(), 2);
        assert_eq!(h1.h0_order(), 4);
        assert_eq!(h1.h2_order(), 2);
        assert!(h1.euler_characteristic_trivial());
    }
    #[test]
    fn test_galois_cohomology_h1_trivial() {
        let h1 = GaloisCohomologyH1::new(5, 7);
        assert_eq!(h1.h1_order(), 1);
        assert!(h1.euler_characteristic_trivial());
    }
    #[test]
    fn test_norm_residue_legendre() {
        let nrm = NormResidueMap::new(7);
        assert_eq!(nrm.legendre(1), 1);
        assert_eq!(nrm.legendre(2), 1);
        assert_eq!(nrm.legendre(4), 1);
        assert_eq!(nrm.legendre(3), -1);
        assert_eq!(nrm.legendre(5), -1);
        assert_eq!(nrm.legendre(6), -1);
        assert_eq!(nrm.legendre(7), 0);
        assert_eq!(nrm.count_qr(), 3);
    }
    #[test]
    fn test_norm_residue_prime5() {
        let nrm = NormResidueMap::new(5);
        assert!(nrm.is_qr(1));
        assert!(nrm.is_qr(4));
        assert!(!nrm.is_qr(2));
        assert!(!nrm.is_qr(3));
        assert_eq!(nrm.count_qr(), 2);
    }
    #[test]
    fn test_selmer_group_bound_no_bad_reduction() {
        let sel = SelmerGroupBound::new(-1, 1, vec![2, 3, 5]);
        let disc = sel.discriminant();
        assert_ne!(disc, 0);
        assert!(sel.rank_upper_bound() >= 1);
    }
    #[test]
    fn test_selmer_group_bound_bad_reduction() {
        let sel = SelmerGroupBound::new(0, 125, vec![2, 3, 5, 7]);
        let disc = sel.discriminant();
        assert!(disc % 5 == 0);
        assert!(sel.selmer_rank_bound() > 1);
    }
    #[test]
    fn test_class_field_tower_infinite() {
        let checker = ClassFieldTowerChecker::new(3, 2, 2);
        assert!(checker.is_infinite_tower());
        assert!(checker.defect() > 0);
        assert_eq!(checker.tower_depth_estimate(), None);
    }
    #[test]
    fn test_class_field_tower_finite() {
        let checker = ClassFieldTowerChecker::new(1, 1, 2);
        assert!(!checker.is_infinite_tower());
        assert!(checker.defect() <= 0);
        assert!(checker.tower_depth_estimate().is_some());
    }
    #[test]
    fn test_class_field_tower_golod_shafarevich_boundary() {
        let checker = ClassFieldTowerChecker::new(2, 1, 2);
        assert!(!checker.is_infinite_tower());
        let checker2 = ClassFieldTowerChecker::new(2, 0, 2);
        assert!(checker2.is_infinite_tower());
    }
    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 4, 7), 4);
        assert_eq!(mod_pow(0, 5, 13), 0);
        assert_eq!(mod_pow(7, 0, 13), 1);
        assert_eq!(mod_pow(5, 1, 1), 0);
    }
    #[test]
    fn test_relations_lower_bound() {
        assert_eq!(ClassFieldTowerChecker::relations_lower_bound(3), 3);
        assert_eq!(ClassFieldTowerChecker::relations_lower_bound(0), 0);
    }
}
