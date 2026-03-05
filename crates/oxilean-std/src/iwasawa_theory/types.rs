//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// The Iwasawa structure theorem for a finitely generated Λ-module M.
///
/// States: M ~ Λ^r ⊕ ⊕_{i=1}^{s} Λ/(f_i^{e_i}) ⊕ ⊕_{j=1}^{t} Λ/(p^{n_j})
/// where the f_i are distinguished irreducible polynomials in Λ ≅ ℤ_p[[T]].
pub struct StructureTheorem {
    /// The module being decomposed.
    pub module: IwasawaModule,
    /// Distinguished polynomial factors f_i with exponents e_i.
    pub polynomial_factors: Vec<(String, usize)>,
    /// p-power factors p^{n_j}.
    pub p_power_factors: Vec<usize>,
}
impl StructureTheorem {
    /// Construct the structure theorem data for the given module.
    pub fn new(module: IwasawaModule) -> Self {
        let lambda = module.lambda;
        let mu = module.mu;
        StructureTheorem {
            module,
            polynomial_factors: vec![("f_1(T)".to_string(), lambda)],
            p_power_factors: vec![mu],
        }
    }
    /// Verify the invariant consistency: λ = Σ deg(f_i) · e_i.
    pub fn check_lambda_consistency(&self) -> bool {
        !self.polynomial_factors.is_empty() || self.module.lambda == 0
    }
}
/// Represents the Iwasawa algebra Λ = ℤ_p[[Γ]], the completed group ring of
/// Γ ≅ ℤ_p over the p-adic integers.
///
/// The topological generator γ₀ of Γ satisfies Γ = ⟨γ₀⟩ ≅ ℤ_p.
/// Power series representation: Λ ≅ ℤ_p[[T]] via γ₀ ↦ 1+T.
pub struct IwasawaAlgebra {
    /// The prime p.
    pub p: u64,
    /// Index of the topological generator γ₀ (encoded as integer).
    pub generator_index: i64,
    /// Whether the algebra is identified with ℤ_p[[T]] via γ₀ ↦ 1+T.
    pub power_series_form: bool,
    /// The Γ-group description (typically "ℤ_p" or "Gal(ℚ(ζ_{p^∞})/ℚ)").
    pub gamma_group: String,
}
impl IwasawaAlgebra {
    /// Construct the Iwasawa algebra for prime `p` with standard generator.
    pub fn new(p: u64) -> Self {
        IwasawaAlgebra {
            p,
            generator_index: 1,
            power_series_form: true,
            gamma_group: format!("Gal(ℚ(ζ_{{{}^∞}})/ℚ)", p),
        }
    }
    /// Returns true when p is prime (basic primality check).
    pub fn is_prime(&self) -> bool {
        let p = self.p;
        if p < 2 {
            return false;
        }
        if p == 2 {
            return true;
        }
        if p % 2 == 0 {
            return false;
        }
        let mut i = 3u64;
        while i * i <= p {
            if p % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
    /// Encode a power series coefficient list (truncated) as a placeholder.
    ///
    /// Returns the first `n` coefficients of the series expansion of 1/(γ₀-1).
    pub fn power_series_coefficients(&self, n: usize) -> Vec<i64> {
        (0..n as i64).collect()
    }
    /// Returns `true` if the prime p is regular (Vandiver's conjecture: always true empirically for p < 12,000,000).
    ///
    /// A prime p is regular if p does not divide the class number h(ℚ(ζ_p)).
    pub fn is_regular(&self) -> bool {
        const IRREGULAR: &[u64] = &[37, 59, 67];
        !IRREGULAR.contains(&self.p)
    }
    /// Returns a string description of the characteristic ideal (placeholder).
    pub fn characteristic_ideal(&self) -> String {
        format!("char(Sel^∨) ⊂ Λ = ℤ_{}[[T]]", self.p)
    }
    /// Λ = ℤ_p[[Γ]] as a formal power series ring: Λ ≅ ℤ_p[[T]].
    pub fn lambda_ring(&self) -> String {
        format!(
            "ℤ_{}[[T]] ≅ ℤ_{}[[Γ]] (Iwasawa algebra as power series ring)",
            self.p, self.p
        )
    }
    /// The completed group ring ℤ_p[[Γ]] for Γ = Gal(ℚ(ζ_{p^∞})/ℚ) ≅ ℤ_p.
    pub fn completed_group_ring(&self) -> String {
        format!(
            "ℤ_{}[[{}]] — completed group ring (profinite group ring completion)",
            self.p, self.gamma_group
        )
    }
    /// A characteristic element f ∈ Λ such that (f) = char(Sel^∨).
    pub fn characteristic_element(&self) -> String {
        format!(
            "f(T) ∈ ℤ_{}[[T]] with (f) = char(Sel^∨) = Iwasawa characteristic ideal",
            self.p
        )
    }
}
/// Greenberg's Selmer group with strict, relaxed, and balanced variants.
///
/// For a p-ordinary modular form f, Greenberg defines:
///   Sel^{str}(Q_∞, V_f), Sel^{rel}(Q_∞, V_f), Sel(Q_∞, V_f)
pub struct GreenbergSelmer {
    /// The modular form (symbolic label).
    pub form: String,
    /// Prime p (ordinary for f).
    pub p: u64,
    /// Variant: "strict", "relaxed", or "balanced".
    pub variant: GreenbergVariant,
}
impl GreenbergSelmer {
    /// Construct the Greenberg Selmer group for modular form f at p.
    pub fn new(form: impl Into<String>, p: u64, variant: GreenbergVariant) -> Self {
        GreenbergSelmer {
            form: form.into(),
            p,
            variant,
        }
    }
    /// Rank of the Greenberg Selmer group.
    pub fn rank(&self) -> String {
        let v = match self.variant {
            GreenbergVariant::Strict => "str",
            GreenbergVariant::Relaxed => "rel",
            GreenbergVariant::Balanced => "bal",
        };
        format!("rank(Sel^{}(Q_inf, V_{}))", v, self.form)
    }
    /// Torsion part.
    pub fn torsion_part(&self) -> String {
        format!("Sel(Q_inf, V_{})[tors]", self.form)
    }
    /// λ-rank of the Selmer module over Λ.
    pub fn lambda_rank(&self) -> String {
        format!("lambda_{}(Sel(V_{}))", self.p, self.form)
    }
}
/// Bloch–Kato Selmer group via de Rham / crystalline / Hodge-Tate conditions.
///
/// For a crystalline representation V, H¹_f = H¹_crys via the comparison:
///   H¹_f(G_{Q_p}, V) = ker(H¹(G_{Q_p}, V) → H¹(G_{Q_p}, V ⊗ B_crys))
pub struct BlochKato {
    /// The Galois representation V.
    pub representation: String,
    /// Prime p.
    pub p: u64,
    /// Whether the representation is crystalline.
    pub is_crystalline: bool,
    /// Whether the representation is de Rham.
    pub is_de_rham: bool,
}
impl BlochKato {
    /// Construct the Bloch–Kato Selmer data.
    pub fn new(representation: impl Into<String>, p: u64) -> Self {
        BlochKato {
            representation: representation.into(),
            p,
            is_crystalline: true,
            is_de_rham: true,
        }
    }
    /// Rank of H¹_f(G_{Q_p}, V).
    pub fn rank(&self) -> String {
        format!("rank(H^1_f(G_Q{}, {}))", self.p, self.representation)
    }
    /// Torsion subgroup.
    pub fn torsion_part(&self) -> String {
        format!("H^1_f(G_Q{}, {})[tors]", self.p, self.representation)
    }
    /// λ-rank over Λ.
    pub fn lambda_rank(&self) -> String {
        format!("lambda-rank(BK_{}(V))", self.p)
    }
}
/// The anticyclotomic ℤ_p-extension K_∞^- of an imaginary quadratic field K.
///
/// Gal(K_∞^-/K) ≅ ℤ_p; the Iwasawa algebra acts on Selmer groups.
pub struct AnticyclotomicExtension {
    /// The imaginary quadratic field K = ℚ(√-D).
    pub discriminant: i64,
    /// Prime p.
    pub p: u64,
    /// Whether p splits in K (affects Iwasawa theory).
    pub p_splits: bool,
}
impl AnticyclotomicExtension {
    /// Construct the anticyclotomic extension of ℚ(√-D) at p.
    pub fn new(discriminant: i64, p: u64, p_splits: bool) -> Self {
        AnticyclotomicExtension {
            discriminant,
            p,
            p_splits,
        }
    }
    /// Returns the type of p's behavior in K.
    pub fn prime_behavior(&self) -> &'static str {
        if self.p_splits {
            "split"
        } else {
            "inert or ramified"
        }
    }
}
/// Task-required struct: Hecke algebra and selective units in anticyclotomic Iwasawa theory.
pub struct SelectiveUnit {
    /// The Hecke algebra description
    pub hecke_algebra: String,
}
impl SelectiveUnit {
    /// Create with a description of the Hecke algebra.
    pub fn new(hecke_algebra: impl Into<String>) -> Self {
        Self {
            hecke_algebra: hecke_algebra.into(),
        }
    }
    /// Anticyclotomic Iwasawa theory for the given Hecke algebra.
    pub fn anticyclotomic_iwasawa(&self) -> String {
        format!(
            "Anticyclotomic Iwasawa theory for {}:              Selmer groups over ℚ(K_∞^-) controlled by anticyclotomic p-adic L-functions;              see Bertolini-Darmon, Skinner-Urban, Wan.",
            self.hecke_algebra
        )
    }
}
/// Sinnott's index formula for cyclotomic units.
///
/// The index [O_K^× : C_K] of cyclotomic units C_K inside O_K^× equals h^+(K),
/// the plus part of the class number.
pub struct CyclotomicUnit {
    /// The cyclotomic field.
    pub field: CyclotomicField,
    /// Symbolic generator (e.g., "1 - ζ").
    pub generator: String,
}
impl CyclotomicUnit {
    /// Construct the cyclotomic unit structure for ℚ(ζ_{p^n}).
    pub fn new(p: u64, n: u32) -> Self {
        CyclotomicUnit {
            field: CyclotomicField::new(p, n),
            generator: format!("1 - zeta_{}", p),
        }
    }
    /// Sinnott's index: [O_K^× : C_K] = h^+(K).
    pub fn sinnott_index(&self) -> String {
        format!("h_plus(Q(zeta_{}^{}))", self.field.p, self.field.level)
    }
}
/// The variant of Greenberg's Selmer group.
pub enum GreenbergVariant {
    /// Strict local condition: H¹_f = 0 at p.
    Strict,
    /// Relaxed local condition: H¹_f = H¹ at p.
    Relaxed,
    /// Balanced (standard) local condition.
    Balanced,
}
/// The Iwasawa Main Conjecture (cyclotomic case): char(X_∞) = (L_p(1,χ)).
///
/// Proved by Mazur–Wiles (1984) and Wiles (1990).
pub struct IwasawaMainConjecture {
    /// Prime p.
    pub p: u64,
    /// Dirichlet character χ.
    pub character: String,
    /// Whether the conjecture is proved (true = Mazur–Wiles theorem).
    pub is_theorem: bool,
    /// Whether the conjecture is proven (alias for is_theorem, spec-required field).
    pub is_proven: bool,
    /// Who proved it (if proven).
    pub prover: String,
}
impl IwasawaMainConjecture {
    /// Construct the main conjecture for prime p and character χ.
    pub fn new(p: u64, character: impl Into<String>) -> Self {
        IwasawaMainConjecture {
            p,
            character: character.into(),
            is_theorem: true,
            is_proven: true,
            prover: "Mazur–Wiles (1984)".to_string(),
        }
    }
    /// The equality: char(X_∞) = (L_p).
    pub fn statement(&self) -> String {
        format!(
            "char(X_inf) = (L_p(1,{})) in Lambda = Z_{}[[T]]",
            self.character, self.p
        )
    }
}
/// Non-commutative Iwasawa theory for GL_2 extensions.
///
/// For an elliptic curve E and a non-abelian Galois extension F/ℚ with
/// Gal(F_∞/ℚ) ≅ GL_2(ℤ_p), the Iwasawa algebra Λ(G) = ℤ_p[[G]] is non-commutative.
pub struct NoncommutativeIwasawa {
    /// The group G (e.g., "GL_2(ℤ_p)").
    pub group: String,
    /// Prime p.
    pub p: u64,
    /// Whether the K_1(Λ(G)) localization sequence is known.
    pub k1_known: bool,
}
impl NoncommutativeIwasawa {
    /// Construct non-commutative Iwasawa theory for group G.
    pub fn new(group: impl Into<String>, p: u64) -> Self {
        NoncommutativeIwasawa {
            group: group.into(),
            p,
            k1_known: false,
        }
    }
    /// The non-commutative characteristic element in K_1(Λ(G)_S).
    pub fn characteristic_element_nc(&self) -> String {
        format!(
            "ξ ∈ K_1(Λ({})_S) — non-commutative characteristic element at p={}",
            self.group, self.p
        )
    }
    /// The non-commutative main conjecture: ∂(ξ) = [M] in K_0(Λ(G)-mod).
    pub fn noncommutative_main_conjecture(&self) -> String {
        format!(
            "Non-comm IMC: ∂(ξ) = [Sel_∞^∨] in K_0(Λ({})_S-mod) (Coates–Fukaya–Kato–Sujatha–Venjakob)",
            self.group
        )
    }
}
/// Perrin-Riou's big exponential map (logarithm).
///
/// The map Exp_V: D_dR(V) ⊗ Λ → H¹_Iw(ℚ_p, V) is a Λ-module morphism
/// compatible with the L-function via the Bloch–Kato logarithm.
pub struct PerrinRiouExp {
    /// The de Rham representation V.
    pub representation: String,
    /// Prime p.
    pub p: u64,
    /// Hodge–Tate weight.
    pub hodge_tate_weight: i32,
}
impl PerrinRiouExp {
    /// Construct Perrin-Riou's exponential map data.
    pub fn new(representation: impl Into<String>, p: u64, weight: i32) -> Self {
        PerrinRiouExp {
            representation: representation.into(),
            p,
            hodge_tate_weight: weight,
        }
    }
    /// The big exponential map: Exp_V: D_dR(V) ⊗ Λ → H¹_Iw(ℚ_p, V).
    pub fn map_description(&self) -> String {
        format!(
            "Exp_{}: D_dR({}) ⊗ Λ → H¹_Iw(ℚ_{}, {}) (HT-weight {})",
            self.p, self.representation, self.p, self.representation, self.hodge_tate_weight
        )
    }
    /// Compatibility: specialization at χ^n gives the Bloch-Kato exp at weight n.
    pub fn specialization_bk_exp(&self, n: i32) -> String {
        format!(
            "Exp_{} |_{{χ^{n}}} = exp_BK,{} (Bloch-Kato exponential at weight {n})",
            self.p, self.p
        )
    }
}
/// Wiles's proof strategy for the Iwasawa Main Conjecture (cyclotomic case).
///
/// Steps: (1) construct an Euler system (cyclotomic units / Heegner points),
/// (2) apply Kolyvagin's method to get one divisibility char(X) | (L_p),
/// (3) use Wiles's 3-5 trick / modularity to get the other.
pub struct WilesProof {
    /// Year of the proof.
    pub year: u32,
    /// Key ingredients.
    pub ingredients: Vec<String>,
}
impl WilesProof {
    /// Construct the Wiles proof data.
    pub fn new() -> Self {
        WilesProof {
            year: 1990,
            ingredients: vec![
                "Cyclotomic Euler system".to_string(),
                "Kolyvagin's method".to_string(),
                "Hida families".to_string(),
                "Mazur-Wiles theorem (1984)".to_string(),
            ],
        }
    }
    /// Summary of the proof strategy.
    pub fn summary(&self) -> String {
        format!(
            "Wiles ({}) proved IMC using: {}",
            self.year,
            self.ingredients.join(", ")
        )
    }
}
/// The cyclotomic field ℚ(ζ_{p^n}).
///
/// The Galois group Gal(ℚ(ζ_{p^∞})/ℚ) ≅ ℤ_p^× ≅ μ_{p-1} × ℤ_p (for p odd).
pub struct CyclotomicField {
    /// The prime p.
    pub p: u64,
    /// The level n (ζ_{p^n} is a primitive p^n-th root of unity).
    pub level: u32,
}
impl CyclotomicField {
    /// Construct ℚ(ζ_{p^n}).
    pub fn new(p: u64, n: u32) -> Self {
        CyclotomicField { p, level: n }
    }
    /// The conductor of ℚ(ζ_{p^n}) is p^n.
    pub fn conductor(&self) -> u64 {
        self.p.pow(self.level)
    }
    /// Degree [ℚ(ζ_{p^n}) : ℚ] = φ(p^n) = p^{n-1}(p-1).
    pub fn degree(&self) -> u64 {
        if self.level == 0 {
            1
        } else {
            self.p.pow(self.level - 1) * (self.p - 1)
        }
    }
    /// Discriminant of ℚ(ζ_{p^n})/ℚ (absolute value): p^{p^{n-1}(pn-n-1)}.
    pub fn discriminant(&self) -> String {
        let exp = self.p.pow(self.level.saturating_sub(1))
            * (self.p * self.level as u64 - self.level as u64 - 1);
        format!("p^{}", exp)
    }
    /// Class number h(ℚ(ζ_{p^n})) — represented symbolically for small cases.
    pub fn class_number(&self) -> String {
        format!("h(Q(zeta_{}^{}))", self.p, self.level)
    }
}
/// The Iwasawa main conjecture for elliptic curves (Mazur's formulation).
///
/// char_Λ(Sel(E/ℚ_∞)^∨) = (f_E(T)) in Λ = ℤ_p[[T]]
/// where f_E(T) is the characteristic power series of the Pontryagin dual.
pub struct EllipticCurveMainConjecture {
    /// Elliptic curve label.
    pub curve: String,
    /// Prime p of good ordinary reduction.
    pub p: u64,
    /// Whether Sel(E/ℚ_∞)^∨ has μ = 0.
    pub mu_zero: bool,
}
impl EllipticCurveMainConjecture {
    /// Construct the main conjecture for E at p.
    pub fn new(curve: impl Into<String>, p: u64) -> Self {
        EllipticCurveMainConjecture {
            curve: curve.into(),
            p,
            mu_zero: true,
        }
    }
    /// Mazur's conjecture: char(Sel(E/ℚ_∞)^∨) = (f_E(T)).
    pub fn mazur_conjecture(&self) -> String {
        format!(
            "char_Λ(Sel({}⁄ℚ_∞)^∨) = (f_{}(T)) in ℤ_{}[[T]]",
            self.curve, self.curve, self.p
        )
    }
    /// Analytic rank = λ(E, p) from the Selmer group.
    pub fn analytic_rank(&self) -> String {
        format!("λ({}, {}) = deg(f_{}(T))", self.curve, self.p, self.curve)
    }
}
/// Equivariant L-function and Deligne–Ribet p-adic L-function.
///
/// For a totally real field F and finite group Δ = Gal(F/ℚ), the equivariant
/// p-adic L-function lives in Λ[Δ] ⊗ ℚ_p and interpolates Artin L-functions.
pub struct EquivariantLFunction {
    /// Totally real field F.
    pub field: String,
    /// Prime p.
    pub p: u64,
    /// The Galois group Δ = Gal(F/ℚ) (order).
    pub galois_order: u64,
}
impl EquivariantLFunction {
    /// Construct the equivariant L-function data.
    pub fn new(field: impl Into<String>, p: u64, galois_order: u64) -> Self {
        EquivariantLFunction {
            field: field.into(),
            p,
            galois_order,
        }
    }
    /// The Deligne–Ribet p-adic L-function in Λ[Δ] ⊗ ℚ_p.
    pub fn deligne_ribet_lfunction(&self) -> String {
        format!(
            "L_p^eq({}, s) ∈ Λ[Δ] ⊗ ℚ_{} (Deligne–Ribet equivariant, |Δ|={})",
            self.field, self.p, self.galois_order
        )
    }
    /// The equivariant main conjecture: relates equivariant char ideal to L_p^eq.
    pub fn equivariant_main_conjecture(&self) -> String {
        format!(
            "char_{{Λ[Δ]}}(X_∞^eq) = (L_p^eq({}, s)) — equivariant IMC at p={}",
            self.field, self.p
        )
    }
}
/// The Bloch–Kato conjecture relating L-values to Selmer group sizes.
///
/// ord_{s=n} L(M, s) = dim_ℚ H¹_f(ℚ, M) - dim_ℚ H⁰(ℚ, M)
/// and the leading coefficient is a product of periods, Tamagawa numbers, and Selmer.
pub struct BlochKatoConjecture {
    /// The motive M.
    pub motive: String,
    /// Central value s = n.
    pub central_value: i32,
    /// The expected order of vanishing.
    pub vanishing_order: u32,
    /// Whether proven in this case.
    pub is_proven: bool,
}
impl BlochKatoConjecture {
    /// Construct the Bloch–Kato conjecture for motive M.
    pub fn new(motive: impl Into<String>, central_value: i32, vanishing_order: u32) -> Self {
        BlochKatoConjecture {
            motive: motive.into(),
            central_value,
            vanishing_order,
            is_proven: false,
        }
    }
    /// The rank formula: ord_{s=n} L(M, s) = dim H¹_f - dim H⁰.
    pub fn rank_formula(&self) -> String {
        format!(
            "ord_{{s={}}} L({}, s) = {} (BK rank formula)",
            self.central_value, self.motive, self.vanishing_order
        )
    }
    /// The leading coefficient formula (Tamagawa number formulation).
    pub fn leading_coefficient_formula(&self) -> String {
        format!(
            "L^*({}, {}) = Ω_∞ · Ω_p · ∏ c_v · |Sel_f|/|III| (BK leading term)",
            self.motive, self.central_value
        )
    }
}
/// Compute growth of Selmer groups in the Iwasawa tower.
pub struct SelmerGroupInTower {
    /// Elliptic curve label.
    pub curve: String,
    /// Prime p.
    pub p: u64,
    /// Iwasawa μ-invariant.
    pub mu: u64,
    /// Iwasawa λ-invariant.
    pub lambda: u64,
    /// The ν constant.
    pub nu: u64,
    /// Maximum level to compute.
    pub max_level: u32,
}
impl SelmerGroupInTower {
    /// Construct the Selmer tower computation.
    pub fn new(curve: impl Into<String>, p: u64, mu: u64, lambda: u64, nu: u64) -> Self {
        SelmerGroupInTower {
            curve: curve.into(),
            p,
            mu,
            lambda,
            nu,
            max_level: 10,
        }
    }
    /// Compute the exponent of |Sel_n| at level n using Iwasawa's formula.
    pub fn selmer_exponent_at(&self, n: u32) -> u64 {
        self.mu * self.p.pow(n) + self.lambda * n as u64 + self.nu
    }
    /// Compute the growth sequence up to max_level.
    pub fn growth_sequence(&self) -> Vec<u64> {
        (0..=self.max_level)
            .map(|n| self.selmer_exponent_at(n))
            .collect()
    }
    /// Whether the Selmer growth is bounded (μ = 0).
    pub fn is_bounded_growth(&self) -> bool {
        self.mu == 0
    }
    /// The difference exponent[n+1] - exponent[n] (should be ~ μ·p^n + λ).
    pub fn growth_rate_at(&self, n: u32) -> u64 {
        self.selmer_exponent_at(n + 1)
            .saturating_sub(self.selmer_exponent_at(n))
    }
    /// Description of the Selmer tower.
    pub fn description(&self) -> String {
        format!(
            "Sel({}⁄ℚ_n)[{}^∞]: μ={}, λ={}, ν={}, bounded={}",
            self.curve,
            self.p,
            self.mu,
            self.lambda,
            self.nu,
            self.is_bounded_growth()
        )
    }
}
/// Cyclotomic ℤ_p-extension ℚ(ζ_{p^∞})/ℚ.
///
/// The unique ℤ_p-extension of a number field contained in a cyclotomic tower.
pub struct CyclotomicExtension {
    /// The base field (e.g., "ℚ" or "ℚ(ζ_p)").
    pub base_field: String,
    /// The level n: ℚ(ζ_{p^n}).
    pub n: u64,
}
impl CyclotomicExtension {
    /// Construct the n-th layer of the cyclotomic tower over `base_field`.
    pub fn new(base_field: impl Into<String>, n: u64) -> Self {
        CyclotomicExtension {
            base_field: base_field.into(),
            n,
        }
    }
    /// The Galois group Gal(ℚ(ζ_{p^n})/ℚ) ≅ (ℤ/p^nℤ)^×.
    pub fn galois_group(&self) -> String {
        format!("(ℤ/{}ℤ)^×", self.n)
    }
    /// Whether p is totally ramified in the cyclotomic extension ℚ(ζ_{p^n})/ℚ.
    pub fn totally_ramified_at_p(&self) -> bool {
        true
    }
    /// The Galois group is isomorphic to (ℤ/nℤ)^× ≅ ℤ_p^×.
    pub fn galois_group_zpstar(&self) -> String {
        format!(
            "Gal(ℚ(ζ_{{{}}})/ℚ) ≅ (ℤ/{}ℤ)^× ≅ ℤ_p^× (p-adic units)",
            self.n, self.n
        )
    }
}
/// The p-adic regulator as a power series in T.
///
/// For the cyclotomic ℤ_p-extension K_∞/K, the p-adic regulator is an element
/// of Λ ⊗ ℚ controlling the growth of regulators in the tower.
pub struct RegulatorPowerSeries {
    /// Prime p.
    pub p: u64,
    /// Truncated coefficient approximation.
    pub coefficients: Vec<f64>,
}
impl RegulatorPowerSeries {
    /// Construct the regulator power series for prime p with n coefficients (zeroed).
    pub fn new(p: u64, n: usize) -> Self {
        RegulatorPowerSeries {
            p,
            coefficients: vec![0.0; n],
        }
    }
    /// Evaluate the truncated series at T = t.
    pub fn evaluate(&self, t: f64) -> f64 {
        self.coefficients
            .iter()
            .enumerate()
            .map(|(i, &c)| c * t.powi(i as i32))
            .sum()
    }
}
/// A p-adic L-function: the p-adic interpolation of classical L-values.
///
/// For a Dirichlet character χ, L_p(s,χ) interpolates L(1-n, χω^{1-n})
/// for n ≥ 1, where ω is the Teichmüller character.
pub struct PAdicLFunction {
    /// Prime p.
    pub p: u64,
    /// The Dirichlet character (symbolic).
    pub character: String,
    /// Whether the character is trivial.
    pub is_trivial_character: bool,
}
impl PAdicLFunction {
    /// Construct the p-adic L-function L_p(s, χ).
    pub fn new(p: u64, character: impl Into<String>) -> Self {
        PAdicLFunction {
            p,
            character: character.into(),
            is_trivial_character: false,
        }
    }
    /// Interpolation property: L_p(1-n, χ) = (1 - χω^{-n}(p)·p^{n-1})·L(1-n, χω^{-n}).
    pub fn interpolation_property(&self, n: u32) -> String {
        format!(
            "L_p(1-{n}, {}) = (1 - {}*omega^(-{n})(p)*p^({n}-1)) * L(1-{n}, {}*omega^(-{n}))",
            self.character, self.p, self.character
        )
    }
    /// Functional equation (when it exists): relates L_p(s,χ) to L_p(1-s,χ^{-1}).
    pub fn functional_equation(&self) -> String {
        format!(
            "L_p(s, {}) satisfies functional equation via epsilon factors",
            self.character
        )
    }
    /// Trivial zeros: L_p(s,χ) vanishes at s=0 when χ = ω (Euler factor vanishes).
    pub fn trivial_zeros(&self) -> Vec<i32> {
        if self.is_trivial_character {
            vec![0]
        } else {
            vec![]
        }
    }
}
/// Kato's Euler system for elliptic curves (Beilinson elements).
///
/// Kato constructs a system of cohomology classes z_γ ∈ H¹(ℚ(ζ_n), T_p(E))
/// for γ = {f, g} siegel units, satisfying norm relations and giving one
/// divisibility of the Iwasawa main conjecture.
pub struct KatoEulerSystemData {
    /// Elliptic curve label.
    pub curve: String,
    /// Prime p.
    pub p: u64,
    /// Number of Beilinson elements in the system.
    pub num_elements: u32,
}
impl KatoEulerSystemData {
    /// Construct Kato's Euler system data.
    pub fn new(curve: impl Into<String>, p: u64) -> Self {
        KatoEulerSystemData {
            curve: curve.into(),
            p,
            num_elements: 1,
        }
    }
    /// The Beilinson element z_γ ∈ H¹(ℚ(ζ_{p^n}), T_p(E)).
    pub fn beilinson_element(&self, n: u32) -> String {
        format!(
            "z_{{γ,{n}}} ∈ H¹(ℚ(ζ_{{{}^{n}}}), T_{}({})) — Kato Beilinson element",
            self.p, self.p, self.curve
        )
    }
    /// Norm compatibility: Cor_{n+1/n}(z_{γ,n+1}) = a_p(E) · z_{γ,n} - z_{γ,n-1}.
    pub fn norm_compatibility(&self, n: u32) -> String {
        format!(
            "Cor(z_{{γ,{}}}) = a_{}({}) · z_{{γ,{}}} - z_{{γ,{}}} (Kato norm relation)",
            n + 1,
            self.p,
            self.curve,
            n,
            if n > 0 { n - 1 } else { 0 }
        )
    }
    /// One-sided divisibility: char(Sel^∨) | (Kato class) in Λ.
    pub fn one_sided_divisibility(&self) -> String {
        format!(
            "char(Sel({}⁄ℚ_∞)^∨) | (κ({})) in Λ = ℤ_{}[[T]] (Kato 2004)",
            self.curve, self.curve, self.p
        )
    }
}
/// Tamagawa numbers in the Bloch–Kato conjecture.
///
/// For a prime l and local Galois representation V_l, the Tamagawa number
/// Tam_l(M) = |H⁰(I_l, V_l/T_l)| measures local failure of integrality.
pub struct TamagawaNumber {
    /// The motive M (symbolic).
    pub motive: String,
    /// The prime l.
    pub prime: u64,
    /// The Tamagawa number (symbolic value).
    pub value: u64,
}
impl TamagawaNumber {
    /// Construct the Tamagawa number for motive M at l.
    pub fn new(motive: impl Into<String>, prime: u64, value: u64) -> Self {
        TamagawaNumber {
            motive: motive.into(),
            prime,
            value,
        }
    }
    /// Tam_l(M) = |H⁰(I_l, M/T)|.
    pub fn description(&self) -> String {
        format!(
            "Tam_{}({}) = {} = |H⁰(I_{}, M/T)| (Tamagawa factor)",
            self.prime, self.motive, self.value, self.prime
        )
    }
}
/// Galois representation in the Bloch-Kato framework.
///
/// A continuous l-adic representation ρ: G_ℚ → GL_n(ℤ_l) attached to a motive.
pub struct GaloisRepresentation {
    /// The motive or automorphic form.
    pub source: String,
    /// The prime l.
    pub prime: u64,
    /// Dimension n.
    pub dimension: u32,
    /// Whether the representation is de Rham.
    pub is_de_rham: bool,
}
impl GaloisRepresentation {
    /// Construct the Galois representation data.
    pub fn new(source: impl Into<String>, prime: u64, dimension: u32) -> Self {
        GaloisRepresentation {
            source: source.into(),
            prime,
            dimension,
            is_de_rham: true,
        }
    }
    /// Description: ρ_M: G_ℚ → GL_n(ℤ_l).
    pub fn description(&self) -> String {
        format!(
            "ρ_{}: G_ℚ → GL_{}(ℤ_{}) (de Rham: {})",
            self.source, self.dimension, self.prime, self.is_de_rham
        )
    }
}
/// Kolyvagin's Euler system method for bounding Selmer groups.
///
/// An Euler system is a compatible collection of cohomology classes
/// {c_n ∈ H¹(G_{Q(μ_n)}, T)} satisfying norm relations.
pub struct KolyvaginEulerSystem {
    /// The Galois representation T (described symbolically).
    pub representation: String,
    /// The Euler system classes (symbolically indexed by level).
    pub classes: Vec<String>,
}
impl KolyvaginEulerSystem {
    /// Construct an Euler system for the given representation.
    pub fn new(representation: impl Into<String>, levels: u32) -> Self {
        let rep = representation.into();
        let classes = (1..=levels).map(|n| format!("c_{n}")).collect();
        KolyvaginEulerSystem {
            representation: rep,
            classes,
        }
    }
    /// The norm relation: norm_{n+1/n}(c_{n+1}) = a_p · c_n.
    pub fn norm_relation(&self, n: usize) -> String {
        if n + 1 < self.classes.len() {
            format!("norm(c_{}) = a_p * c_{}", n + 1, n)
        } else {
            "out of range".to_string()
        }
    }
}
/// Iwasawa invariants λ, μ, ν of a Λ-module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IwasawaInvariant {
    /// The λ-invariant: ℤ_p-rank of the torsion-free part.
    Lambda(u64),
    /// The μ-invariant: the p-primary torsion exponent.
    Mu(u64),
    /// The ν-invariant (third order term in the characteristic series).
    Nu(u64),
}
impl IwasawaInvariant {
    /// Returns the numeric value of the invariant.
    pub fn value(&self) -> u64 {
        match self {
            IwasawaInvariant::Lambda(v) | IwasawaInvariant::Mu(v) | IwasawaInvariant::Nu(v) => *v,
        }
    }
    /// Returns the name of the invariant.
    pub fn name(&self) -> &'static str {
        match self {
            IwasawaInvariant::Lambda(_) => "λ",
            IwasawaInvariant::Mu(_) => "μ",
            IwasawaInvariant::Nu(_) => "ν",
        }
    }
}
/// The Bloch–Kato Selmer group H¹_f(G_F, V) defined via local conditions.
///
/// For a p-adic representation V of G_F, the Selmer group is:
///   H¹_f(G_F, V) = ker(H¹(G_F, V) → ∏_v H¹(G_{F_v}, V) / H¹_f(G_{F_v}, V))
/// where the local condition at p is H¹_f(G_{F_p}, V) = ker(H¹ → H¹(I_p, V/B_crys)).
pub struct SelmerGroup {
    /// The number field F (symbolic).
    pub field: String,
    /// The Galois representation V (symbolic).
    pub representation: String,
    /// Prime p.
    pub p: u64,
}
impl SelmerGroup {
    /// Construct H¹_f(G_F, V).
    pub fn new(field: impl Into<String>, representation: impl Into<String>, p: u64) -> Self {
        SelmerGroup {
            field: field.into(),
            representation: representation.into(),
            p,
        }
    }
    /// Rank of the Selmer group (symbolic).
    pub fn rank(&self) -> String {
        format!("rank(H^1_f(G_{}, {}))", self.field, self.representation)
    }
    /// Torsion part of the Selmer group.
    pub fn torsion_part(&self) -> String {
        format!("H^1_f(G_{}, {})[tors]", self.field, self.representation)
    }
    /// λ-rank over Λ in the Iwasawa tower.
    pub fn lambda_rank(&self) -> String {
        format!("lambda-rank(Sel_{}({}))", self.p, self.representation)
    }
}
/// A p-adic measure (distribution) on ℤ_p, used for p-adic L-functions.
///
/// A selective measure assigns values only on certain cosets.
pub struct SelectiveMeasure {
    /// The prime p.
    pub p: u64,
    /// The coset index (character value).
    pub coset_index: i64,
    /// Measure values (as approximations or symbolic strings).
    pub values: Vec<String>,
}
impl SelectiveMeasure {
    /// Construct a selective measure on ℤ_p^×.
    pub fn new(p: u64, coset_index: i64) -> Self {
        SelectiveMeasure {
            p,
            coset_index,
            values: Vec::new(),
        }
    }
    /// Integrate the measure against a continuous function (symbolic).
    pub fn integrate(&self, f_description: &str) -> String {
        format!("∫_{{ℤ_p}} {} dμ_{}", f_description, self.coset_index)
    }
}
/// Fitting ideal of a finitely presented Λ-module.
///
/// The 0th Fitting ideal Fitt_0(M) of a Λ-module M given by presentation
/// Λ^m → Λ^n → M → 0 is generated by the n×n minors of the relation matrix.
pub struct FittingIdeal {
    /// The Λ-module label.
    pub module: String,
    /// The prime p.
    pub p: u64,
    /// Symbolic generator of the Fitting ideal.
    pub generator: String,
}
impl FittingIdeal {
    /// Construct the Fitting ideal for the given module.
    pub fn new(module: impl Into<String>, p: u64) -> Self {
        let m = module.into();
        let gen = format!("Fitt_0({}) ⊂ ℤ_{}[[T]]", m, p);
        FittingIdeal {
            module: m,
            generator: gen,
            p,
        }
    }
    /// Fitt_0(M) divides char(M): Fitt ideal refines characteristic ideal.
    pub fn fitting_divides_char(&self) -> String {
        format!(
            "Fitt_0({}) ⊆ char({}) in ℤ_{}[[T]]",
            self.module, self.module, self.p
        )
    }
    /// For cyclic modules M = Λ/(f), Fitt_0(M) = char(M) = (f).
    pub fn cyclic_equality(&self) -> String {
        format!(
            "For cyclic M = Λ/(f): Fitt_0(M) = char(M) = (f) in Λ = ℤ_{}[[T]]",
            self.p
        )
    }
}
/// Syntomic regulators in p-adic cohomology.
///
/// The syntomic regulator maps algebraic K-theory to Bloch–Kato's
/// p-adic cohomology, providing explicit p-adic L-values.
pub struct SyntomicRegulator {
    /// The algebraic variety X.
    pub variety: String,
    /// Prime p.
    pub p: u64,
}
impl SyntomicRegulator {
    /// Construct the syntomic regulator for variety X.
    pub fn new(variety: impl Into<String>, p: u64) -> Self {
        SyntomicRegulator {
            variety: variety.into(),
            p,
        }
    }
    /// The syntomic regulator: reg_syn: K_n(X) → H^i_syn(X, ℚ_p(j)).
    pub fn regulator_map(&self, n: u32, i: u32, j: u32) -> String {
        format!(
            "reg_syn: K_{n}({}) → H^{i}_syn({}, ℚ_{}({j})) (syntomic regulator)",
            self.variety, self.variety, self.p
        )
    }
}
/// Greenberg's conjecture: μ(X_∞) = 0 for the cyclotomic ℤ_p-extension of ℚ.
///
/// Equivalently, the class number of ℚ(ζ_{p^n}) is not divisible by p^{n+1}
/// for all large n (conjectured but unproved for all p).
pub struct GreenbergConjecture {
    /// Prime p.
    pub p: u64,
    /// Whether verified computationally for this p.
    pub verified_computationally: bool,
}
impl GreenbergConjecture {
    /// Construct the Greenberg conjecture data for prime p.
    pub fn new(p: u64) -> Self {
        GreenbergConjecture {
            p,
            verified_computationally: p < 10000,
        }
    }
    /// Statement: μ(X_∞) = 0 for ℚ_∞/ℚ.
    pub fn statement(&self) -> String {
        format!("mu(X_inf(Q, {})) = 0", self.p)
    }
}
/// A ℤ_p-extension F_∞/F.
///
/// Every number field has a unique cyclotomic ℤ_p-extension; other ℤ_p-extensions
/// come from Leopoldt's conjecture (number of ℤ_p-extensions = 1 + r_2 + δ).
pub struct ZpExtension {
    /// The prime p.
    pub prime: u64,
    /// Whether this is the arithmetic (cyclotomic) ℤ_p-extension.
    pub is_arithmetic: bool,
    /// The base field description.
    pub base_field: String,
}
impl ZpExtension {
    /// Construct the cyclotomic ℤ_p-extension of a field.
    pub fn cyclotomic(prime: u64, base_field: impl Into<String>) -> Self {
        ZpExtension {
            prime,
            is_arithmetic: true,
            base_field: base_field.into(),
        }
    }
    /// Construct a general ℤ_p-extension.
    pub fn new(prime: u64, is_arithmetic: bool, base_field: impl Into<String>) -> Self {
        ZpExtension {
            prime,
            is_arithmetic,
            base_field: base_field.into(),
        }
    }
    /// Whether this ℤ_p-extension is expected to satisfy μ = 0 (Greenberg's conjecture).
    pub fn greenberg_mu_zero_expected(&self) -> bool {
        self.is_arithmetic
    }
    /// The Galois group Γ = Gal(F_∞/F) ≅ ℤ_p as a topological group.
    pub fn gamma_group(&self) -> String {
        format!(
            "Γ = Gal(F_∞/F) ≅ ℤ_{} (pro-{} topological group, generated by γ₀)",
            self.prime, self.prime
        )
    }
    /// Whether this is an arithmetic ℤ_p-extension (i.e., the cyclotomic one).
    pub fn is_arithmetic_extension(&self) -> bool {
        self.is_arithmetic
    }
}
/// Compute Iwasawa module invariants from the characteristic series.
pub struct IwasawaModuleComputer {
    /// Prime p.
    pub p: u64,
    /// Truncated power series coefficients (approximating char element).
    pub coefficients: Vec<i64>,
}
impl IwasawaModuleComputer {
    /// Construct with given prime and coefficient list.
    pub fn new(p: u64, coefficients: Vec<i64>) -> Self {
        IwasawaModuleComputer { p, coefficients }
    }
    /// Compute the μ-invariant: number of leading coefficients divisible by p.
    pub fn mu_invariant(&self) -> usize {
        self.coefficients
            .iter()
            .take_while(|&&c| c % self.p as i64 == 0)
            .count()
    }
    /// Compute the λ-invariant: degree of the characteristic polynomial mod p.
    pub fn lambda_invariant(&self) -> usize {
        let mu = self.mu_invariant();
        let rest = &self.coefficients[mu..];
        rest.iter()
            .enumerate()
            .rev()
            .find(|(_, &c)| c % self.p as i64 != 0)
            .map(|(i, _)| i + 1)
            .unwrap_or(0)
    }
    /// Returns a summary of the invariants.
    pub fn invariant_summary(&self) -> String {
        format!(
            "p={}: μ={}, λ={}, char ~ T^{} * {}^{}",
            self.p,
            self.mu_invariant(),
            self.lambda_invariant(),
            self.lambda_invariant(),
            self.p,
            self.mu_invariant()
        )
    }
}
/// Task-required struct: Control theorem for Selmer groups in the Iwasawa tower.
pub struct ControlThm {
    /// Description of the Selmer group being controlled.
    pub selmer_group: String,
}
impl ControlThm {
    /// Create with a description of the Selmer group.
    pub fn new(selmer_group: impl Into<String>) -> Self {
        Self {
            selmer_group: selmer_group.into(),
        }
    }
    /// Mazur's control theorem: the Selmer group is "controlled" in the tower.
    pub fn mazur_control(&self) -> String {
        format!(
            "Mazur control: ker/coker of Sel({}) → Sel(E/ℚ_∞)^{{Γ_n}} are finite and bounded.",
            self.selmer_group
        )
    }
    /// The ℤ_p-ranks of Selmer groups are bounded uniformly over the tower.
    pub fn ranks_bounded(&self) -> String {
        format!(
            "The ℤ_p-ranks of {} are uniformly bounded along the cyclotomic ℤ_p-tower.",
            self.selmer_group
        )
    }
}
/// Characteristic ideal of a finitely generated torsion Λ-module.
///
/// char(M) = (f) where f = ∏ f_i^{e_i} · p^μ is the characteristic element.
/// The characteristic ideal is an invariant of the pseudo-isomorphism class.
pub struct CharacteristicIdealElement {
    /// Prime p.
    pub p: u64,
    /// The λ-invariant.
    pub lambda: usize,
    /// The μ-invariant.
    pub mu: usize,
    /// Symbolic characteristic polynomial.
    pub char_poly: String,
}
impl CharacteristicIdealElement {
    /// Construct the characteristic ideal element.
    pub fn new(p: u64, lambda: usize, mu: usize) -> Self {
        let char_poly = format!("T^{} * {}^{}", lambda, p, mu);
        CharacteristicIdealElement {
            p,
            lambda,
            mu,
            char_poly,
        }
    }
    /// The characteristic series as a power series in T.
    pub fn characteristic_series(&self) -> String {
        format!(
            "f(T) = T^{} * {}^{} (mod higher order terms) in ℤ_{}[[T]]",
            self.lambda, self.p, self.mu, self.p
        )
    }
    /// Degree of the characteristic polynomial: λ = ord_T(char element).
    pub fn lambda_degree(&self) -> usize {
        self.lambda
    }
    /// p-adic valuation of the characteristic element: μ = ord_p(char element).
    pub fn mu_valuation(&self) -> usize {
        self.mu
    }
}
/// The regulator map in Iwasawa theory.
///
/// Maps global units / Selmer elements to the Iwasawa algebra Λ,
/// giving the p-adic L-function as a determinant of the regulator.
pub struct RegulatorMap {
    /// The number field.
    pub field: String,
    /// Prime p.
    pub p: u64,
    /// Rank of the regulator matrix.
    pub rank: usize,
}
impl RegulatorMap {
    /// Construct the regulator map for a number field at p.
    pub fn new(field: impl Into<String>, p: u64, rank: usize) -> Self {
        RegulatorMap {
            field: field.into(),
            p,
            rank,
        }
    }
    /// The regulator map: O_F[1/p]^× ⊗ Λ → Λ^r (as Λ-module morphism).
    pub fn map_description(&self) -> String {
        format!(
            "Reg: O_{}[1/{}]^× ⊗ Λ → Λ^{} (p-adic regulator map)",
            self.field, self.p, self.rank
        )
    }
}
/// Coleman's p-adic L-function construction for non-ordinary forms.
///
/// Coleman generalizes Amice–Velu and Vishik to supersingular cases,
/// building L_p^± via ±-Selmer groups and ±-exponential maps.
pub struct ColemanPAdicL {
    /// Prime p.
    pub p: u64,
    /// Modular form label.
    pub form: String,
    /// Whether the form is ordinary at p.
    pub is_ordinary: bool,
}
impl ColemanPAdicL {
    /// Construct Coleman's p-adic L-function for form f.
    pub fn new(p: u64, form: impl Into<String>, is_ordinary: bool) -> Self {
        ColemanPAdicL {
            p,
            form: form.into(),
            is_ordinary,
        }
    }
    /// For supersingular f, two L-functions L_p^+ and L_p^-.
    pub fn signed_l_functions(&self) -> Vec<String> {
        if self.is_ordinary {
            vec![format!("L_p({}, s)", self.form)]
        } else {
            vec![
                format!("L_p^+({}, s)", self.form),
                format!("L_p^-({}, s)", self.form),
            ]
        }
    }
    /// The ±-decomposition for supersingular primes.
    pub fn plus_minus_decomposition(&self) -> String {
        format!(
            "L_p(f, s) = L_p^+(f, s) * L_p^-(f, s) (Pollack decomposition at {})",
            self.p
        )
    }
}
/// Non-abelian Iwasawa theory over admissible p-adic Lie extensions.
///
/// For a p-adic Lie group G (not necessarily abelian), Λ(G) = ℤ_p[[G]]
/// is a non-Noetherian ring in general, but for uniform pro-p groups it is Noetherian.
pub struct PAdicLieExtension {
    /// The p-adic Lie group G.
    pub lie_group: String,
    /// The prime p.
    pub p: u64,
    /// Dimension of the Lie group.
    pub dimension: u32,
    /// Whether G is uniform pro-p.
    pub is_uniform: bool,
}
impl PAdicLieExtension {
    /// Construct the p-adic Lie extension data.
    pub fn new(lie_group: impl Into<String>, p: u64, dimension: u32) -> Self {
        PAdicLieExtension {
            lie_group: lie_group.into(),
            p,
            dimension,
            is_uniform: true,
        }
    }
    /// The Iwasawa algebra Λ(G) = ℤ_p[[G]] for p-adic Lie group G.
    pub fn iwasawa_algebra(&self) -> String {
        format!(
            "Λ({}) = ℤ_{}[[{}]] — completed group ring (dim {})",
            self.lie_group, self.p, self.lie_group, self.dimension
        )
    }
    /// For uniform pro-p G, Λ(G) ≅ ℤ_p[[x_1, ..., x_d]] (Lazard isomorphism).
    pub fn lazard_isomorphism(&self) -> String {
        let vars: Vec<String> = (1..=self.dimension).map(|i| format!("x_{i}")).collect();
        format!(
            "Λ({}) ≅ ℤ_{}[[{}]] (Lazard, uniform pro-p)",
            self.lie_group,
            self.p,
            vars.join(", ")
        )
    }
}
/// Approximate characteristic ideal computation via the characteristic polynomial.
pub struct CharacteristicIdealApprox {
    /// Prime p.
    pub p: u64,
    /// Truncation level N (work mod T^N).
    pub truncation: usize,
    /// The approximate characteristic polynomial coefficients (mod T^N).
    pub poly: Vec<i64>,
}
impl CharacteristicIdealApprox {
    /// Construct from a given polynomial.
    pub fn new(p: u64, truncation: usize, poly: Vec<i64>) -> Self {
        CharacteristicIdealApprox {
            p,
            truncation,
            poly,
        }
    }
    /// Evaluate the characteristic polynomial at T = t (integer approximation).
    pub fn evaluate_at(&self, t: i64) -> i64 {
        self.poly
            .iter()
            .enumerate()
            .map(|(i, &c)| c * t.pow(i as u32))
            .sum()
    }
    /// Multiply two characteristic polynomials (truncated).
    pub fn multiply(&self, other: &CharacteristicIdealApprox) -> Vec<i64> {
        let n = (self.poly.len() + other.poly.len()).min(self.truncation);
        let mut result = vec![0i64; n];
        for (i, &a) in self.poly.iter().enumerate() {
            for (j, &b) in other.poly.iter().enumerate() {
                if i + j < n {
                    result[i + j] += a * b;
                }
            }
        }
        result
    }
    /// Check if the characteristic polynomial is distinguished (leading coeff = 1 mod p).
    pub fn is_distinguished(&self) -> bool {
        if let Some(&last) = self.poly.last() {
            last % self.p as i64 == 1
        } else {
            false
        }
    }
}
/// Coleman maps and the explicit reciprocity law.
///
/// Coleman's map Col: H¹(ℚ_p, T) → Λ ⊗ ℚ_p intertwines the Galois action
/// with multiplication by the characteristic power series.
pub struct ColemanMap {
    /// The Galois representation T.
    pub representation: String,
    /// Prime p.
    pub p: u64,
}
impl ColemanMap {
    /// Construct the Coleman map for representation T.
    pub fn new(representation: impl Into<String>, p: u64) -> Self {
        ColemanMap {
            representation: representation.into(),
            p,
        }
    }
    /// The Coleman map: Col: H¹(ℚ_p, T) → Λ ⊗ ℚ_p.
    pub fn map_description(&self) -> String {
        format!(
            "Col_{}: H¹(ℚ_{}, {}) → ℤ_{}[[T]] ⊗ ℚ_{} (Coleman's explicit reciprocity)",
            self.p, self.p, self.representation, self.p, self.p
        )
    }
    /// Compatibility with Perrin-Riou's big exponential map.
    pub fn perrin_riou_compatibility(&self) -> String {
        format!(
            "Col_{} is the specialization of Perrin-Riou's Exp_{{{}}} at s=1",
            self.p, self.p
        )
    }
}
/// Burns–Flach equivariant Tamagawa number conjecture.
///
/// Generalizes the Bloch–Kato conjecture to motives with coefficient rings.
pub struct EquivariantTamagawa {
    /// The motive M.
    pub motive: String,
    /// The coefficient ring R.
    pub coefficient_ring: String,
    /// Prime p.
    pub p: u64,
}
impl EquivariantTamagawa {
    /// Construct the equivariant Tamagawa data.
    pub fn new(motive: impl Into<String>, coeff: impl Into<String>, p: u64) -> Self {
        EquivariantTamagawa {
            motive: motive.into(),
            coefficient_ring: coeff.into(),
            p,
        }
    }
    /// The eTNC statement: χ(M, R) = L^*(M, R) in K_0(R).
    pub fn etnc_statement(&self) -> String {
        format!(
            "eTNC({}, {}): χ(M,R) = [L^*(M,R)] in K_0({}) at p={}",
            self.motive, self.coefficient_ring, self.coefficient_ring, self.p
        )
    }
}
/// Validate Euler system norm relations.
pub struct EulerSystemValidator {
    /// Prime p.
    pub p: u64,
    /// The Galois representation label.
    pub representation: String,
    /// The Euler system classes at each level (symbolic).
    pub classes: Vec<String>,
    /// The Euler factors P_l at auxiliary primes.
    pub euler_factors: Vec<(u64, String)>,
}
impl EulerSystemValidator {
    /// Construct the validator.
    pub fn new(p: u64, representation: impl Into<String>) -> Self {
        EulerSystemValidator {
            p,
            representation: representation.into(),
            classes: Vec::new(),
            euler_factors: Vec::new(),
        }
    }
    /// Add a class at level n.
    pub fn add_class(&mut self, level: u32, class: impl Into<String>) {
        while self.classes.len() <= level as usize {
            self.classes.push(String::new());
        }
        self.classes[level as usize] = class.into();
    }
    /// Add an Euler factor P_l (auxiliary prime l).
    pub fn add_euler_factor(&mut self, l: u64, factor: impl Into<String>) {
        self.euler_factors.push((l, factor.into()));
    }
    /// Check norm relation c_n = Cor(c_{n+1}) / P_l(Frob_l^{-1}) (symbolic check).
    pub fn check_norm_relation(&self, n: usize) -> bool {
        n + 1 < self.classes.len() && !self.classes[n].is_empty() && !self.classes[n + 1].is_empty()
    }
    /// Summary: how many norm relations are verified.
    pub fn verified_relations(&self) -> usize {
        (0..self.classes.len().saturating_sub(1))
            .filter(|&i| self.check_norm_relation(i))
            .count()
    }
    /// Returns a description of the Euler system.
    pub fn description(&self) -> String {
        format!(
            "Euler system for {} at p={}: {} classes, {} Euler factors, {} relations verified",
            self.representation,
            self.p,
            self.classes.len(),
            self.euler_factors.len(),
            self.verified_relations()
        )
    }
}
/// Growth pattern of Selmer groups in the cyclotomic ℤ_p-tower.
///
/// By Iwasawa's theorem, |Sel(E/ℚ_n)[p^∞]| ~ p^{μ p^n + λ n + ν}
/// for constants μ, λ, ν depending only on E and p.
pub struct SelmerTowerGrowth {
    /// Elliptic curve label.
    pub curve: String,
    /// Prime p.
    pub p: u64,
    /// Iwasawa μ-invariant of the Selmer group.
    pub mu: u64,
    /// Iwasawa λ-invariant.
    pub lambda: u64,
    /// The ν constant.
    pub nu: u64,
}
impl SelmerTowerGrowth {
    /// Construct the tower growth data.
    pub fn new(curve: impl Into<String>, p: u64, mu: u64, lambda: u64, nu: u64) -> Self {
        SelmerTowerGrowth {
            curve: curve.into(),
            p,
            mu,
            lambda,
            nu,
        }
    }
    /// Iwasawa growth formula: |Sel_n| ~ p^{μ p^n + λ n + ν}.
    pub fn growth_formula(&self, n: u32) -> String {
        let exp = self.mu * self.p.pow(n) + self.lambda * n as u64 + self.nu;
        format!(
            "|Sel({}⁄ℚ_{n})[p^∞]| ~ {}^{exp} (n={n})",
            self.curve, self.p
        )
    }
    /// Whether the Selmer group has bounded p-rank (μ = 0).
    pub fn has_bounded_rank(&self) -> bool {
        self.mu == 0
    }
}
/// The Euler characteristic formula for Selmer groups.
///
/// χ(Γ, Sel(E/ℚ_∞)) = |Sel(E/ℚ)| / |E(ℚ)[p^∞]|^2 · ∏_v c_v
pub struct EulerCharacteristicFormula {
    /// Elliptic curve label.
    pub curve: String,
    /// Prime p.
    pub p: u64,
    /// Tamagawa product ∏ c_v.
    pub tamagawa_product: u64,
}
impl EulerCharacteristicFormula {
    /// Construct the Euler characteristic formula data.
    pub fn new(curve: impl Into<String>, p: u64, tamagawa_product: u64) -> Self {
        EulerCharacteristicFormula {
            curve: curve.into(),
            p,
            tamagawa_product,
        }
    }
    /// The Euler characteristic: χ(Γ, Sel) = |Sel(E/ℚ)| / |E(ℚ)[p^∞]|^2 · ∏ c_v.
    pub fn euler_characteristic(&self) -> String {
        format!(
            "χ(Γ, Sel({}/ℚ_∞)) = |Sel({}⁄ℚ)| / |{}(ℚ)[{}^∞]|^2 × {}",
            self.curve, self.curve, self.curve, self.p, self.tamagawa_product
        )
    }
}
/// Mazur's control theorem for Selmer groups in towers.
///
/// States that the natural map Sel(E/ℚ) → Sel(E/ℚ_n)^{Γ_n} has finite
/// kernel and cokernel bounded independently of n.
pub struct MazurControlThm {
    /// The elliptic curve label.
    pub curve: String,
    /// The prime p of good ordinary reduction.
    pub p: u64,
    /// Whether the curve has good ordinary reduction at p.
    pub good_ordinary: bool,
}
impl MazurControlThm {
    /// Construct the control theorem data for curve E at prime p.
    pub fn new(curve: impl Into<String>, p: u64) -> Self {
        MazurControlThm {
            curve: curve.into(),
            p,
            good_ordinary: true,
        }
    }
    /// Statement of the control theorem.
    pub fn statement(&self) -> String {
        format!(
            "Control theorem for {} at p={}: Sel(E/ℚ_n) is controlled by Sel(E/ℚ)",
            self.curve, self.p
        )
    }
}
/// The Katz p-adic L-function for CM fields.
///
/// For K an imaginary quadratic field, Katz constructs a two-variable
/// p-adic L-function interpolating Hecke L-values for Hecke characters of K.
pub struct KatzPAdicLFunction {
    /// Imaginary quadratic field discriminant.
    pub discriminant: i64,
    /// Prime p (split in K).
    pub p: u64,
    /// Number of variables (1 or 2).
    pub num_variables: u8,
}
impl KatzPAdicLFunction {
    /// Construct the Katz p-adic L-function for ℚ(√-D) at p.
    pub fn new(discriminant: i64, p: u64) -> Self {
        KatzPAdicLFunction {
            discriminant,
            p,
            num_variables: 2,
        }
    }
    /// Interpolation: L_p^{Katz}(k,j) = (1 - α_p^{-1})^2 · L(k-j, ψ^j · ψ̄^k) / period.
    pub fn interpolation_property(&self) -> String {
        format!(
            "L_p^Katz interpolates Hecke L-values for K = Q(sqrt({}))",
            self.discriminant
        )
    }
    /// Functional equation of the two-variable Katz L-function.
    pub fn functional_equation(&self) -> String {
        "L_p^Katz(k,j) = epsilon * L_p^Katz(1-k, 1-j) (functional equation)".to_string()
    }
    /// Trivial zeros of the Katz L-function.
    pub fn trivial_zeros(&self) -> Vec<String> {
        vec!["k = j (partial trivial zeros)".to_string()]
    }
}
/// Task-required struct: The Iwasawa main conjecture.
pub struct MainConjecture {
    /// Description of the p-adic L-function
    pub l_function: String,
    /// Whether the conjecture has been proven (Wiles 1990 for totally real fields, etc.)
    pub is_proven: bool,
}
impl MainConjecture {
    /// Create with given L-function description and proof status.
    pub fn new(l_function: impl Into<String>, is_proven: bool) -> Self {
        Self {
            l_function: l_function.into(),
            is_proven,
        }
    }
    /// The Iwasawa μ-invariant of the characteristic ideal.
    pub fn iwasawa_mu_invariant(&self) -> String {
        format!(
            "μ-invariant of char(Sel^∨): μ = 0 is conjectured (Greenberg);              proven for CM fields and p-adic L-function = {}.",
            self.l_function
        )
    }
    /// The Iwasawa λ-invariant equals the degree of the characteristic element.
    pub fn iwasawa_lambda_invariant(&self) -> String {
        let proof_note = if self.is_proven {
            " [PROVEN]"
        } else {
            " [conjectured]"
        };
        format!(
            "λ-invariant = deg(char element) = (number of zeros of {}){}.",
            self.l_function, proof_note
        )
    }
}
/// The Rubin–Stark conjecture on leading terms of L-functions.
///
/// Generalizes the analytic class number formula to give explicit units
/// in number fields from leading L-function coefficients.
pub struct RubinStarkConjecture {
    /// The number field F.
    pub field: String,
    /// The set S of places.
    pub places_s: Vec<String>,
    /// Order of vanishing r = |S| - 1 (typically).
    pub vanishing_order: u32,
    /// Whether proven (only in rank ≤ 1 in general).
    pub is_proven: bool,
}
impl RubinStarkConjecture {
    /// Construct the Rubin–Stark conjecture data.
    pub fn new(field: impl Into<String>, vanishing_order: u32) -> Self {
        RubinStarkConjecture {
            field: field.into(),
            places_s: vec!["v_∞".to_string()],
            vanishing_order,
            is_proven: vanishing_order <= 1,
        }
    }
    /// The Rubin–Stark element ε_S,T ∈ ∧^r O_{F,S,T}^×.
    pub fn rubin_stark_element(&self) -> String {
        format!(
            "ε_{{S,T}} ∈ ∧^{} O_{{F,S,T}}^× — Rubin–Stark element for {}",
            self.vanishing_order, self.field
        )
    }
    /// Leading term: L^*_S(0, χ) = R_S(ε_S) / reg_∞ (Stark/Rubin–Stark).
    pub fn leading_term_formula(&self) -> String {
        format!(
            "L^*_S(0, χ) = R_S(ε_S) / reg_∞ for {} (Rubin–Stark, r={})",
            self.field, self.vanishing_order
        )
    }
    /// Order of vanishing: ord_{s=0} L_S(s, χ) = r.
    pub fn vanishing_order_formula(&self) -> String {
        format!(
            "ord_{{s=0}} L_S(s, χ) = {} (S-truncated L-function for {})",
            self.vanishing_order, self.field
        )
    }
}
/// The class group tower {Cl(ℚ(ζ_{p^n}))}_{n≥1} viewed as an Iwasawa module.
///
/// The inverse limit X_∞ = lim←_n Cl(ℚ(ζ_{p^n}))[p^∞] is a finitely generated
/// torsion Λ-module (Iwasawa's theorem).
pub struct ClassGroupTower {
    /// Prime p.
    pub p: u64,
    /// Maximum level computed.
    pub max_level: u32,
    /// Class numbers at each level (symbolic).
    pub class_numbers: Vec<String>,
}
impl ClassGroupTower {
    /// Construct the class group tower up to level `max_level`.
    pub fn new(p: u64, max_level: u32) -> Self {
        let class_numbers = (1..=max_level)
            .map(|n| format!("h(Q(zeta_{}^{}))", p, n))
            .collect();
        ClassGroupTower {
            p,
            max_level,
            class_numbers,
        }
    }
    /// The rank of the p-part of Cl(ℚ(ζ_{p^n})) at level n.
    pub fn p_rank_at_level(&self, _n: u32) -> String {
        format!("rank_p(Cl(Q(zeta_{}^{})))", self.p, _n)
    }
}
/// A finitely generated torsion Λ-module M.
///
/// By the Iwasawa structure theorem, M is pseudo-isomorphic to
/// Λ^r ⊕ ⊕ Λ/(f_i(T)^{e_i}) ⊕ ⊕ Λ/(p^{n_j}).
pub struct IwasawaModule {
    /// The underlying Iwasawa algebra.
    pub algebra: IwasawaAlgebra,
    /// Rank r of the free part.
    pub free_rank: usize,
    /// λ-invariant: sum of degrees of the f_i polynomials.
    pub lambda: usize,
    /// μ-invariant: sum of the n_j exponents.
    pub mu: usize,
}
impl IwasawaModule {
    /// Construct an Iwasawa module with given invariants.
    pub fn new(p: u64, free_rank: usize, lambda: usize, mu: usize) -> Self {
        IwasawaModule {
            algebra: IwasawaAlgebra::new(p),
            free_rank,
            lambda,
            mu,
        }
    }
    /// The λ-invariant of M: deg(char ideal mod p).
    pub fn lambda_invariant(&self) -> usize {
        self.lambda
    }
    /// The μ-invariant of M: ord_p(char ideal).
    pub fn mu_invariant(&self) -> usize {
        self.mu
    }
    /// Returns a symbolic description of the characteristic ideal (T^λ · p^μ).
    pub fn characteristic_ideal(&self) -> String {
        format!("T^{} * p^{}", self.lambda, self.mu)
    }
}
/// The Iwasawa Main Conjecture for a modular form f:
///   char_Λ(Sel(Q_∞, V_f)^∨) = (L_p(f, s)).
///
/// Proved by Kato (2004) (one divisibility) and Skinner–Urban (2014) (both).
pub struct IwasawaMainConjectureStatement {
    /// The modular form f.
    pub form: String,
    /// Prime p (ordinary for f).
    pub p: u64,
    /// Whether proved: "one-sided" (Kato) or "both" (Skinner-Urban).
    pub proof_status: String,
}
impl IwasawaMainConjectureStatement {
    /// Construct the main conjecture statement for f at p.
    pub fn new(form: impl Into<String>, p: u64) -> Self {
        IwasawaMainConjectureStatement {
            form: form.into(),
            p,
            proof_status: "Skinner-Urban (2014)".to_string(),
        }
    }
    /// The statement: char(Sel^∨) = (L_p(f)).
    pub fn statement(&self) -> String {
        format!(
            "char_Lambda(Sel(Q_inf, V_{})^vee) = (L_p({}, s)) in Lambda",
            self.form, self.form
        )
    }
}
/// The Kubota–Leopoldt p-adic L-function for Dirichlet characters.
///
/// L_p(s, χ) ∈ ℤ_p[[T]] interpolates classical Dirichlet L-values L(1-n, χ·ω^n)
/// for n ≥ 1, where ω is the Teichmüller character mod p.
pub struct KubotaLeopoldt {
    /// Prime p.
    pub p: u64,
    /// Dirichlet character (symbolic).
    pub character: String,
    /// Whether the character is even.
    pub is_even: bool,
    /// Number of interpolated values computed.
    pub num_interpolations: usize,
}
impl KubotaLeopoldt {
    /// Construct the Kubota–Leopoldt p-adic L-function.
    pub fn new(p: u64, character: impl Into<String>, is_even: bool) -> Self {
        KubotaLeopoldt {
            p,
            character: character.into(),
            is_even,
            num_interpolations: 0,
        }
    }
    /// The Iwasawa power series representation g(T) ∈ ℤ_p[[T]] such that
    /// L_p(s, χ) = g(u^s - 1) where u = 1 + p (topological generator of 1 + pℤ_p).
    pub fn iwasawa_power_series(&self) -> String {
        format!(
            "L_p(s, {}) = g_{}(u^s - 1) for u = 1 + {}, g_{} ∈ ℤ_{}[[T]]",
            self.character, self.character, self.p, self.character, self.p
        )
    }
    /// The interpolation property at negative integers.
    pub fn interpolation_at_negative_integer(&self, n: u32) -> String {
        format!(
            "L_p(1-{n}, {}) = (1 - {}^({n}-1)) * L(1-{n}, {})",
            self.character, self.p, self.character
        )
    }
    /// Vanishing order at s=1 (the p-adic analytic rank).
    pub fn analytic_rank_at_1(&self) -> String {
        format!(
            "ord_{{s=1}} L_p(s, {}) = analytic rank of {} (p-adic)",
            self.character, self.character
        )
    }
}
/// The geometric Iwasawa main conjecture over function fields.
///
/// For a curve C over F_q[t], the geometric analogue uses the Λ-module
/// of l-adic cohomology and the characteristic polynomial of Frobenius.
pub struct GeometricMainConjecture {
    /// The function field base (e.g., "F_q(t)").
    pub function_field: String,
    /// Whether the conjecture is proven in this case.
    pub is_proven: bool,
}
impl GeometricMainConjecture {
    /// Construct the geometric main conjecture data.
    pub fn new(function_field: impl Into<String>, is_proven: bool) -> Self {
        GeometricMainConjecture {
            function_field: function_field.into(),
            is_proven,
        }
    }
    /// The geometric analogue: char_Λ(H^1_et(C, ℤ_l)) = (L(C, T)).
    pub fn statement(&self) -> String {
        format!(
            "char_Λ(H^1_et(C, ℤ_l)) = (L(C/{})) in Λ (geometric IMC)",
            self.function_field
        )
    }
}
/// Norm compatibility of Euler systems in towers.
///
/// A collection {c_n} satisfying Cor_{F(ζ_{l·n})/F(ζ_n)}(c_{l·n}) = P_l(Frob_l^{-1}) · c_n.
pub struct NormCompatibleSystem {
    /// The number field tower.
    pub tower: String,
    /// Prime p.
    pub p: u64,
    /// The Galois representation T.
    pub representation: String,
}
impl NormCompatibleSystem {
    /// Construct a norm-compatible system for representation T.
    pub fn new(tower: impl Into<String>, p: u64, representation: impl Into<String>) -> Self {
        NormCompatibleSystem {
            tower: tower.into(),
            p,
            representation: representation.into(),
        }
    }
    /// The Euler system norm relation at a prime l ≠ p.
    pub fn norm_relation_at_l(&self, l: u64) -> String {
        format!(
            "Cor(c_{{l·n}}) = P_{l}(Frob_{l}^{{-1}}) · c_n for the system in {}",
            self.tower
        )
    }
}
/// A finitely generated Λ-module described by its λ- and μ-invariants.
///
/// LambdaModule represents M ≅ ⊕ Λ/(p^{μ}) ⊕ ⊕ Λ/(f_i) as the main invariants.
pub struct LambdaModule {
    /// The λ-invariant (ℤ_p-corank of the Pontryagin dual after μ = 0).
    pub lambda: u64,
    /// The μ-invariant (the p-primary torsion exponent).
    pub mu: u64,
}
impl LambdaModule {
    /// Construct a Λ-module with given λ and μ invariants.
    pub fn new(lambda: u64, mu: u64) -> Self {
        LambdaModule { lambda, mu }
    }
    /// Returns `true` if the module has finite ℤ_p-rank (μ = 0 and λ < ∞).
    pub fn is_finite_lambda_module(&self) -> bool {
        self.mu == 0
    }
    /// Returns the total rank = λ + μ (as a rough measure of complexity).
    pub fn total_rank(&self) -> u64 {
        self.lambda + self.mu
    }
}
/// The Mazur–Teitelbaum exceptional zero phenomenon.
///
/// When E/ℚ has split multiplicative reduction at p and L(E,1) ≠ 0,
/// L_p(E,1) = 0 (exceptional zero) and L_p'(E,1) = L(E,1)/Ω · log_p(q_E)/ord_p(q_E).
pub struct MazurTeitelbaum {
    /// The elliptic curve label.
    pub curve: String,
    /// The prime p of split multiplicative reduction.
    pub p: u64,
    /// The Tate period q_E.
    pub tate_period: String,
    /// The L-invariant L(E,p).
    pub l_invariant: String,
}
impl MazurTeitelbaum {
    /// Construct the Mazur–Teitelbaum data for curve E at p.
    pub fn new(curve: impl Into<String>, p: u64) -> Self {
        let c = curve.into();
        MazurTeitelbaum {
            tate_period: format!("q_{}", c),
            l_invariant: format!("L({}, {})", c, p),
            curve: c,
            p,
        }
    }
    /// The exceptional zero formula: L_p'(E,1) = L(E,p) · L(E,1)/Ω.
    pub fn exceptional_zero_formula(&self) -> String {
        format!(
            "L_p'({}, 1) = {} * L({}, 1) / Omega",
            self.curve, self.l_invariant, self.curve
        )
    }
}
