//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Environment, Expr, Level, Name};

use super::types::{
    AtkinLehnerInvolution, AutomorphicRepresentation, DirichletCharacter, EisensteinSeries,
    HeckeLFunction, HeckeOperator, HeckeOperatorDataMatrix, Mat2x2, ModularCurve, ModularCurveType,
    ModularForm, ModularFormCusp, ModularSymbol, MoonshineDatum, NewformDecomposition,
    PeterssonInnerProduct, QExpansion, RamanujanTau, RamanujanTauFunction,
    RankinSelbergConvolution, ShimuraVariety, SiegelModularForm,
};

/// Type alias for backward compatibility.
pub type HeckeOperatorMatrix = HeckeOperatorDataMatrix;

pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn complex_ty() -> Expr {
    cst("Complex")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
pub fn arrow3(a: Expr, b: Expr, c: Expr) -> Expr {
    arrow(a, arrow(b, c))
}
pub fn arrow4(a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    arrow(a, arrow3(b, c, d))
}
pub fn arrow5(a: Expr, b: Expr, c: Expr, d: Expr, e: Expr) -> Expr {
    arrow(a, arrow4(b, c, d, e))
}
/// Compute σ_{k-1}(n) = ∑_{d|n} d^{k-1}, the divisor power sum.
pub fn sigma_k_minus_1(n: u64, k: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = 0u64;
    let exp = k.saturating_sub(1);
    for d in 1..=n {
        if n % d == 0 {
            result = result.saturating_add(d.saturating_pow(exp));
        }
    }
    result
}
/// Fourier coefficient of E_k at n: a_0 = 1 (for normalized Eisenstein series G_k/2ζ(k)),
/// a_n = σ_{k-1}(n) for n ≥ 1.
pub fn eisenstein_fourier_coeff(n: u64, k: u32) -> u64 {
    if n == 0 {
        1
    } else {
        sigma_k_minus_1(n, k)
    }
}
/// The Ramanujan tau function τ(n) for small n, computed via the identity
/// Δ = q ∏_{n≥1}(1-q^n)^24.  We compute coefficients up to N_MAX by convolution.
/// This is an integer-exact computation for small n.
pub fn ramanujan_tau_up_to(n_max: usize) -> Vec<i64> {
    let mut coeffs = vec![0i64; n_max + 1];
    let size = n_max + 1;
    let mut prod = vec![0i64; size];
    prod[0] = 1;
    for m in 1..size {
        for _ in 0..24 {
            for j in (m..size).rev() {
                prod[j] -= prod[j - m];
            }
        }
    }
    for n in 1..=n_max {
        if n >= 1 {
            coeffs[n] = *prod.get(n - 1).unwrap_or(&0);
        }
    }
    coeffs[0] = 0;
    coeffs
}
/// Check Ramanujan's congruence τ(n) ≡ σ_{11}(n) (mod 691) for n ≤ N.
pub fn check_ramanujan_congruence(n_max: usize) -> bool {
    let taus = ramanujan_tau_up_to(n_max);
    for n in 1..=n_max {
        let tau_n = taus[n].rem_euclid(691) as u64;
        let sigma11_n = sigma_k_minus_1(n as u64, 12) % 691;
        if tau_n != sigma11_n {
            return false;
        }
    }
    true
}
/// Theta series coefficient: r_2(n) = #{(a,b) ∈ ℤ²: a²+b²=n}.
pub fn r2(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut count = 0u64;
    let bound = (n as f64).sqrt() as i64 + 1;
    for a in -bound..=bound {
        let b2 = n as i64 - a * a;
        if b2 < 0 {
            continue;
        }
        let b = (b2 as f64).sqrt() as i64;
        if b * b == b2 {
            count += 1;
            if b > 0 {
                count += 1;
            }
        }
    }
    count
}
/// Hecke operator T_p action on a modular form with Fourier coefficients a_n.
/// If f = ∑ a_n q^n has weight k, then T_p(f) = ∑ b_n q^n where
/// b_n = a_{pn} + p^{k-1} a_{n/p} (with a_{n/p}=0 if p∤n).
pub fn hecke_tp_coefficients(a: &[i64], p: u64, k: u32) -> Vec<i64> {
    let n_max = a.len();
    let mut b = vec![0i64; n_max];
    let pk1 = (p as i64).pow(k.saturating_sub(1));
    for n in 0..n_max {
        let pn = (n as u64).saturating_mul(p) as usize;
        b[n] = if pn < n_max { a[pn] } else { 0 };
        if n as u64 % p == 0 {
            let np = (n as u64 / p) as usize;
            b[n] += pk1 * a[np];
        }
    }
    b
}
/// `ModularGroup : Type` — SL₂(ℤ) as a group.
pub fn modular_group_ty() -> Expr {
    type0()
}
/// `PSL2Z : Type` — PSL₂(ℤ) = SL₂(ℤ)/{±I}.
pub fn psl2z_ty() -> Expr {
    type0()
}
/// `GeneratorS : ModularGroup` — S = [\[0,-1\],\[1,0\]].
pub fn generator_s_ty() -> Expr {
    cst("ModularGroup")
}
/// `GeneratorT : ModularGroup` — T = [\[1,1\],\[0,1\]].
pub fn generator_t_ty() -> Expr {
    cst("ModularGroup")
}
/// `FundamentalDomain : Type` — standard fundamental domain F for SL₂(ℤ).
pub fn fundamental_domain_ty() -> Expr {
    type0()
}
/// `CongruenceSubgroup : Nat → Type` — Γ(N) = ker(SL₂(ℤ) → SL₂(ℤ/Nℤ)).
pub fn congruence_subgroup_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `Gamma0 : Nat → Type` — Γ₀(N) = {[\[a,b\],\[c,d\]] : N|c}.
pub fn gamma0_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `Gamma1 : Nat → Type` — Γ₁(N) = {[\[a,b\],\[c,d\]] : N|c, a≡d≡1 mod N}.
pub fn gamma1_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SubgroupIndex : Nat → Nat` — index \[SL₂(ℤ) : Γ₀(N)\].
pub fn subgroup_index_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `SubgroupLevel : CongruenceSubgroup → Nat` — level of a congruence subgroup.
pub fn subgroup_level_ty() -> Expr {
    arrow(cst("CongruenceSubgroup"), nat_ty())
}
/// `ModularForm : Nat → CongruenceSubgroup → Type`
/// — space M_k(Γ) of weight-k modular forms for Γ.
pub fn modular_form_ty() -> Expr {
    arrow3(nat_ty(), cst("CongruenceSubgroup"), type0())
}
/// `CuspForm : Nat → CongruenceSubgroup → Type`
/// — space S_k(Γ) of weight-k cusp forms for Γ.
pub fn cusp_form_ty() -> Expr {
    arrow3(nat_ty(), cst("CongruenceSubgroup"), type0())
}
/// `EisensteinSeries : Nat → Type`
/// — Eisenstein series E_k (for k ≥ 4, even).
pub fn eisenstein_series_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GFunction : Nat → Type`
/// — non-normalised Eisenstein series G_k = ∑_{(c,d)≠(0,0)} (cτ+d)^{-k}.
pub fn g_function_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FourierCoefficient : ModularForm → Nat → Complex`
/// — n-th Fourier coefficient a_n(f).
pub fn fourier_coefficient_ty() -> Expr {
    arrow3(cst("ModularForm"), nat_ty(), complex_ty())
}
/// `ModularFormWeight : ModularForm → Nat` — weight k of a modular form.
pub fn modular_form_weight_ty() -> Expr {
    arrow(cst("ModularForm"), nat_ty())
}
/// `HolomorphicAtCusps : ModularForm → Prop` — f is holomorphic at all cusps.
pub fn holomorphic_at_cusps_ty() -> Expr {
    arrow(cst("ModularForm"), prop())
}
/// `VanishesAtCusps : ModularForm → Prop` — f is a cusp form (a_0 = 0 at all cusps).
pub fn vanishes_at_cusps_ty() -> Expr {
    arrow(cst("ModularForm"), prop())
}
/// `PeterssonInnerProduct : CuspForm → CuspForm → Complex`
/// — ⟨f, g⟩ = ∫_Γ\H f(τ)·ḡ(τ)·y^k dxdy/y².
pub fn petersson_inner_product_ty() -> Expr {
    arrow3(cst("CuspForm"), cst("CuspForm"), complex_ty())
}
/// `PeterssonNorm : CuspForm → Real` — ‖f‖² = ⟨f,f⟩.
pub fn petersson_norm_ty() -> Expr {
    arrow(cst("CuspForm"), real_ty())
}
/// `HeckeOperator : Nat → Nat → CongruenceSubgroup → Type`
/// — T_n acting on M_k(Γ).
pub fn hecke_operator_ty() -> Expr {
    arrow4(nat_ty(), nat_ty(), cst("CongruenceSubgroup"), type0())
}
/// `HeckeAlgebra : Nat → CongruenceSubgroup → Type`
/// — commutative algebra generated by all T_n.
pub fn hecke_algebra_ty() -> Expr {
    arrow3(nat_ty(), cst("CongruenceSubgroup"), type0())
}
/// `DiamondOperator : Nat → Nat → ModularForm → ModularForm`
/// — ⟨d⟩ acting on M_k(Γ₁(N)).
pub fn diamond_operator_ty() -> Expr {
    arrow4(nat_ty(), nat_ty(), cst("ModularForm"), cst("ModularForm"))
}
/// `HeckeEigenform : ModularForm → Prop`
/// — f is a simultaneous eigenform for all Hecke operators.
pub fn hecke_eigenform_ty() -> Expr {
    arrow(cst("ModularForm"), prop())
}
/// `HeckeEigenvalue : ModularForm → Nat → Complex`
/// — eigenvalue λ_n(f) of T_n on f.
pub fn hecke_eigenvalue_ty() -> Expr {
    arrow3(cst("ModularForm"), nat_ty(), complex_ty())
}
/// `HeckeMultiplicativity : Nat → CongruenceSubgroup → Prop`
/// — T_mn = T_m ∘ T_n when gcd(m,n) = 1.
pub fn hecke_multiplicativity_ty() -> Expr {
    arrow3(nat_ty(), cst("CongruenceSubgroup"), prop())
}
/// `LFunction_MF : ModularForm → Complex → Complex`
/// — L(f,s) = ∑ a_n(f)·n^{-s}.
pub fn l_function_mf_ty() -> Expr {
    arrow3(cst("ModularForm"), complex_ty(), complex_ty())
}
/// `CompletedLFunction : ModularForm → Complex → Complex`
/// — Λ(f,s) = (2π)^{-s} Γ(s) L(f,s) (completed L-function).
pub fn completed_l_function_ty() -> Expr {
    arrow3(cst("ModularForm"), complex_ty(), complex_ty())
}
/// `FunctionalEquation_MF : ModularForm → Prop`
/// — Λ(f,s) = ε·Λ(f̃, k-s) for some root number ε ∈ {±1}.
pub fn functional_equation_mf_ty() -> Expr {
    arrow(cst("ModularForm"), prop())
}
/// `RootNumber : ModularForm → Int` — global root number ε(f) ∈ {±1}.
pub fn root_number_ty() -> Expr {
    arrow(cst("ModularForm"), int_ty())
}
/// `EulerProduct_MF : ModularForm → Prop`
/// — L(f,s) = ∏_p (1 - a_p p^{-s} + p^{k-1-2s})^{-1} for f a newform.
pub fn euler_product_mf_ty() -> Expr {
    arrow(cst("ModularForm"), prop())
}
/// `RamanujanTau : Nat → Int` — τ(n) = n-th Fourier coefficient of Δ = q∏(1-q^n)^24.
pub fn ramanujan_tau_ty() -> Expr {
    arrow(nat_ty(), int_ty())
}
/// `DeltaForm : Type` — the discriminant modular form Δ ∈ S_{12}(SL₂(ℤ)).
pub fn delta_form_ty() -> Expr {
    type0()
}
/// `RamanujanConjecture : Nat → Prop`
/// — |τ(p)| ≤ 2·p^{11/2} for all primes p (proved by Deligne as Weil conjecture).
pub fn ramanujan_conjecture_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `RamanujanCongruence : Nat → Prop`
/// — τ(n) ≡ σ_{11}(n) (mod 691).
pub fn ramanujan_congruence_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ThetaSeries : Type → Type`
/// — theta series θ_L(τ) = ∑_{x∈L} q^{Q(x)} for a lattice L.
pub fn theta_series_ty() -> Expr {
    arrow(type0(), type0())
}
/// `ThetaSeriesLattice : Nat → Type`
/// — theta series attached to a rank-n integral lattice.
pub fn theta_series_lattice_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `JacobiThetaFunction : Complex → Complex → Complex`
/// — ϑ(z, τ) = ∑_{n∈ℤ} exp(πi n² τ + 2πi n z).
pub fn jacobi_theta_function_ty() -> Expr {
    arrow3(complex_ty(), complex_ty(), complex_ty())
}
/// `ModularSymbol : Nat → CongruenceSubgroup → Type`
/// — {α, β} ∈ H_1(X(Γ), cusps, ℤ), the Manin-Eichler-Shimura symbol.
pub fn modular_symbol_ty() -> Expr {
    arrow3(nat_ty(), cst("CongruenceSubgroup"), type0())
}
/// `ManinSymbol : Nat → Type` — Manin symbol (c:d) ∈ P¹(ℤ/Nℤ).
pub fn manin_symbol_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EichlerShimuraPairing : ModularForm → ModularSymbol → Complex`
/// — integration pairing ∫_{α}^{β} f(τ)(2πi τ)^j dτ.
pub fn eichler_shimura_pairing_ty() -> Expr {
    arrow3(cst("ModularForm"), cst("ModularSymbol"), complex_ty())
}
/// `EichlerShimuraRelation : Nat → Prop`
/// — T_p on cusp forms corresponds to Frobenius at p on Jac(X(Γ)).
pub fn eichler_shimura_relation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ModularCurve : CongruenceSubgroup → Type`
/// — modular curve Y(Γ) = Γ\H (or its compactification X(Γ)).
pub fn modular_curve_ty() -> Expr {
    arrow(cst("CongruenceSubgroup"), type0())
}
/// `Cusp : CongruenceSubgroup → Type` — cusps of the modular curve X(Γ).
pub fn cusp_ty() -> Expr {
    arrow(cst("CongruenceSubgroup"), type0())
}
/// `JacobianOfModularCurve : CongruenceSubgroup → Type`
/// — Jac(X(Γ)) as an abelian variety.
pub fn jacobian_of_modular_curve_ty() -> Expr {
    arrow(cst("CongruenceSubgroup"), type0())
}
/// `Newform : Nat → Nat → Type`
/// — primitive newform of weight k and level N.
pub fn newform_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), type0())
}
/// `OldformSpace : Nat → Nat → Type`
/// — space of oldforms of weight k, level N.
pub fn oldform_space_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), type0())
}
/// `NewformDecomposition : Nat → CongruenceSubgroup → Prop`
/// — S_k(Γ₀(N)) = S_k^{new}(N) ⊕ S_k^{old}(N).
pub fn newform_decomposition_ty() -> Expr {
    arrow3(nat_ty(), cst("CongruenceSubgroup"), prop())
}
/// `StrongMultiplicityOne : Nat → Nat → Prop`
/// — two newforms with the same a_p for almost all p are identical.
pub fn strong_multiplicity_one_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), prop())
}
/// `AtkinLehnerInvolution : Nat → ModularForm → ModularForm`
/// — w_N involution on S_k(Γ₀(N)).
pub fn atkin_lehner_involution_ty() -> Expr {
    arrow3(nat_ty(), cst("ModularForm"), cst("ModularForm"))
}
/// `AtkinLehnerEigenvalue : Nat → Newform → Int`
/// — eigenvalue of w_N on a newform: ±1.
pub fn atkin_lehner_eigenvalue_ty() -> Expr {
    arrow3(nat_ty(), cst("Newform"), int_ty())
}
/// `GrossZagierFormula : Nat → Prop`
/// — Gross-Zagier: L'(E,1) = (height of Heegner point) · (period).
pub fn gross_zagier_formula_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `TateTwist_MF : ModularForm → Nat → ModularForm`
/// — Tate twist f ⊗ χ for a Dirichlet character χ mod M.
pub fn tate_twist_mf_ty() -> Expr {
    arrow3(cst("ModularForm"), nat_ty(), cst("ModularForm"))
}
/// `GaloisConjugate_MF : Newform → Type`
/// — Galois conjugate forms: the orbit of f under Gal(Q(a_n)/Q).
pub fn galois_conjugate_mf_ty() -> Expr {
    arrow(cst("Newform"), type0())
}
/// `ModularFormDimension : Nat → Nat → Nat`
/// — dim M_k(Γ₀(N)) via Riemann-Roch / Gauss-Bonnet.
pub fn modular_form_dimension_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), nat_ty())
}
/// `CuspFormDimension : Nat → Nat → Nat`
/// — dim S_k(Γ₀(N)).
pub fn cusp_form_dimension_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), nat_ty())
}
/// `PoincareSeries : Nat → Nat → Complex → Complex`
/// — Poincaré series P_{k,m}(τ) = ∑_{γ∈Γ_∞\Γ} (cτ+d)^{-k} e^{2πimγτ}.
pub fn poincare_series_ty() -> Expr {
    arrow4(nat_ty(), nat_ty(), complex_ty(), complex_ty())
}
/// `SiegelModularForm : Nat → Nat → Type`
/// — Siegel modular form of weight k and degree g.
pub fn siegel_modular_form_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), type0())
}
/// `HilbertModularForm : Nat → Type`
/// — Hilbert modular form over a totally real field of degree n.
pub fn hilbert_modular_form_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ModularFormLift : Newform → Nat → Prop`
/// — base change / automorphic lift of a newform to GL_2 over a number field.
pub fn modular_form_lift_ty() -> Expr {
    arrow3(cst("Newform"), nat_ty(), prop())
}
/// `SatoTateConjecture : Nat → Nat → Prop`
/// — equidistribution of a_p/(2√p) w.r.t. Sato-Tate measure (proved by Taylor et al.).
pub fn sato_tate_conjecture_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), prop())
}
/// `LanglandsCorrespondence_GL2 : Newform → Type`
/// — automorphic representation of GL_2(ℚ\𝔸_ℚ) attached to a newform.
pub fn langlands_correspondence_gl2_ty() -> Expr {
    arrow(cst("Newform"), type0())
}
/// `ModularFormConductor : Newform → Nat` — arithmetic conductor N(f).
pub fn modular_form_conductor_ty() -> Expr {
    arrow(cst("Newform"), nat_ty())
}
/// `NebentypusCharacter : Newform → Type`
/// — nebentypus character χ: (ℤ/Nℤ)× → ℂ× of a newform.
pub fn nebentypus_character_ty() -> Expr {
    arrow(cst("Newform"), type0())
}
/// `HeckeTnMatrix : Nat → Nat → Type`
/// — explicit Hecke matrix double coset \[Γ diag(1,n) Γ\] acting on q-expansions.
pub fn hecke_tn_matrix_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), type0())
}
/// `HeckeCommutative : Nat → CongruenceSubgroup → Prop`
/// — the Hecke algebra is commutative: T_m T_n = T_n T_m.
pub fn hecke_commutative_ty() -> Expr {
    arrow3(nat_ty(), cst("CongruenceSubgroup"), prop())
}
/// `HeckeNormalOperator : ModularForm → Prop`
/// — T_n is normal w.r.t. Petersson inner product: T_n* = T_n.
pub fn hecke_normal_operator_ty() -> Expr {
    arrow(cst("ModularForm"), prop())
}
/// `NewformEigenSystem : Nat → Nat → Type`
/// — eigenpacket (system of Hecke eigenvalues) for a newform of weight k, level N.
pub fn newform_eigen_system_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), type0())
}
/// `MaassForm : Nat → Type`
/// — real-analytic eigenfunction of the Laplace-Beltrami operator Δ_k on Γ\H.
pub fn maass_form_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `LaplaceBeltramiEigenvalue : MaassForm → Real`
/// — spectral eigenvalue λ = s(1-s) (where s ∈ ℂ) of Δ acting on a Maass form.
pub fn laplace_beltrami_eigenvalue_ty() -> Expr {
    arrow(cst("MaassForm"), real_ty())
}
/// `MaassLFunction : MaassForm → Complex → Complex`
/// — L-function L(f, s) = ∑ a_n n^{-s} for a Maass cusp form f.
pub fn maass_l_function_ty() -> Expr {
    arrow3(cst("MaassForm"), complex_ty(), complex_ty())
}
/// `SelvergEigenvalueConjecture : Prop`
/// — Selberg's eigenvalue conjecture: all Maass cusp forms for Γ₀(N) have
/// λ ≥ 1/4 (equivalently s = 1/2 + it, t real).
pub fn selberg_eigenvalue_conjecture_ty() -> Expr {
    prop()
}
/// `WeylLaw : Nat → Prop`
/// — Weyl's law: #{λ_j ≤ T} ~ (Area(Γ\H) / 4π) T as T → ∞ (Selberg).
pub fn weyl_law_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `HalfIntegerWeightForm : Nat → Nat → Type`
/// — modular form of weight k/2 (k odd) for Γ₀(4N) with nebentypus.
pub fn half_integer_weight_form_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), type0())
}
/// `ShimuraCorrespondence : Nat → Prop`
/// — Shimura lifting: half-integer weight forms ↔ integer weight forms
/// (Shimura 1973; explicit via Hecke correspondences).
pub fn shimura_correspondence_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `WaldspurgerFormula : Nat → Prop`
/// — Waldspurger's theorem: central L-values L(f, 1/2, χ) encoded in
/// Fourier coefficients of half-integer weight forms.
pub fn waldspurger_formula_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `KokbeVariance : Nat → Nat → Real`
/// — variance of a half-integer weight Fourier coefficient a(n):
/// linked to L(f, 1/2) via Kohnen-Zagier / Waldspurger.
pub fn kohnen_variance_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), real_ty())
}
/// `GaloisRepresentation_MF : Newform → Nat → Type`
/// — l-adic Galois representation ρ_{f,l}: Gal(Q̄/Q) → GL₂(Q_l)
/// attached to a newform f (Eichler-Shimura / Deligne).
pub fn galois_representation_mf_ty() -> Expr {
    arrow3(cst("Newform"), nat_ty(), type0())
}
/// `EichlerShimuraConstruction : Newform → Prop`
/// — the construction of ρ_{f,l} via the l-adic cohomology of the
/// modular curve X₀(N) (Eichler-Shimura relation).
pub fn eichler_shimura_construction_ty() -> Expr {
    arrow(cst("Newform"), prop())
}
/// `LAdic_Representation : Newform → Nat → Prop`
/// — the l-adic representation is unramified at all p ∤ lN
/// with Frobenius characteristic polynomial X² - a_p X + p^{k-1}.
pub fn l_adic_representation_ty() -> Expr {
    arrow3(cst("Newform"), nat_ty(), prop())
}
/// `DeligneSemiSimplicity : Newform → Prop`
/// — Deligne's theorem: ρ_{f,l} is semi-simple and pure of weight k-1
/// (used in the proof of Weil conjectures for curves).
pub fn deligne_semisimplicity_ty() -> Expr {
    arrow(cst("Newform"), prop())
}
/// `OverconvergentModularForm : Nat → Real → Type`
/// — p-adic overconvergent modular form of weight k and overconvergence radius r.
pub fn overconvergent_modular_form_ty() -> Expr {
    arrow3(nat_ty(), real_ty(), type0())
}
/// `ColemanFamily : Nat → Type`
/// — Coleman family of overconvergent eigenforms varying p-adically in weight.
pub fn coleman_family_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PAdicLFunction_MF : Newform → Nat → Complex → Complex`
/// — p-adic L-function L_p(f, s) interpolating critical values L(f, j) (j = 1…k-1).
pub fn p_adic_l_function_mf_ty() -> Expr {
    arrow4(cst("Newform"), nat_ty(), complex_ty(), complex_ty())
}
/// `HidaFamily : Nat → Nat → Type`
/// — Hida family: a p-adic analytic family of ordinary newforms of varying weight.
pub fn hida_family_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), type0())
}
/// `EigenvarietyCurve : Nat → Type`
/// — the eigencurve (Coleman-Mazur): a rigid analytic curve parametrising
/// overconvergent eigenforms of all weights.
pub fn eigencurve_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `X0N : Nat → Type` — compactified modular curve X₀(N) over ℚ.
pub fn x0n_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `X1N : Nat → Type` — compactified modular curve X₁(N) over ℚ.
pub fn x1n_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CuspResolution : Nat → Type`
/// — the resolution of cusps of X₀(N): local coordinates at each cusp.
pub fn cusp_resolution_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ModularUnit : Nat → Type`
/// — a modular unit u on X₀(N): a rational function with zeros/poles only at cusps.
pub fn modular_unit_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SiegelUnit : Nat → Complex → Complex`
/// — Siegel unit g_a(τ) on the upper half-plane: special modular function.
pub fn siegel_unit_ty() -> Expr {
    arrow3(nat_ty(), complex_ty(), complex_ty())
}
/// `CMPoint : Nat → Type`
/// — a CM point τ ∈ H: quadratic imaginary τ, fixed by an order in an imaginary
/// quadratic field K.
pub fn cm_point_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ShimuraReciprocity : Nat → Prop`
/// — Shimura reciprocity law: the action of Gal(K^ab/K) on CM values of
/// modular functions via the Artin map.
pub fn shimura_reciprocity_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CMTheory : Nat → Prop`
/// — CM theory: explicit class field theory for imaginary quadratic fields
/// via j-function values j(τ) for CM points τ.
pub fn cm_theory_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `HeegnerPoint : Nat → Type`
/// — a Heegner point on X₀(N): a CM point in the modular curve X₀(N).
pub fn heegner_point_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GrossZagierHeegner : Nat → Prop`
/// — Gross-Zagier formula: L'(E, 1) is proportional to the Néron-Tate height
/// of the Heegner point in J₀(N).
pub fn gross_zagier_heegner_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SatoTateMeasure : Nat → Type`
/// — the Sato-Tate measure μ_ST on \[-2,2\]: (2/π)√(1 - x²/4) dx.
pub fn sato_tate_measure_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SatoTateEquidistribution : Newform → Prop`
/// — Sato-Tate equidistribution: a_p/(2p^{(k-1)/2}) is equidistributed
/// w.r.t. the Sato-Tate measure (Taylor-Barnet-Lamb-Geraghty-Harris 2011).
pub fn sato_tate_equidistribution_ty() -> Expr {
    arrow(cst("Newform"), prop())
}
/// `SatoTateLFunction : Newform → Nat → Complex`
/// — symmetric power L-function Sym^n L(f, s) used in the proof of Sato-Tate.
pub fn sato_tate_l_function_ty() -> Expr {
    arrow3(cst("Newform"), nat_ty(), complex_ty())
}
/// `ModularityLifting : Nat → Prop`
/// — R = T theorem: the universal deformation ring R of a residual Galois
/// representation equals the Hecke algebra T (Taylor-Wiles 1995).
pub fn modularity_lifting_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ResidualRepresentation : Nat → Type`
/// — a mod-l Galois representation ρ̄: Gal(Q̄/Q) → GL₂(F_l).
pub fn residual_representation_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `DeformationRing : Nat → Type`
/// — the universal deformation ring R (pro-representable hull) of ρ̄.
pub fn deformation_ring_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `TaylorWilesMethod : Nat → Prop`
/// — the Taylor-Wiles patching method: proves R→T is an isomorphism
/// by patching over auxiliary primes Q.
pub fn taylor_wiles_method_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `FermatLastTheoremViaModularity : Prop`
/// — Fermat's Last Theorem follows from Wiles' R=T (modularity lifting) theorem.
pub fn fermat_last_theorem_via_modularity_ty() -> Expr {
    prop()
}
/// `SiegelThetaSeries : Nat → Nat → Type`
/// — Siegel theta series θ_L of genus g attached to a positive-definite
/// even integral lattice L of rank n.
pub fn siegel_theta_series_ty() -> Expr {
    arrow3(nat_ty(), nat_ty(), type0())
}
/// `JacobiFormulaFourSquares : Nat → Nat`
/// — Jacobi's four-square theorem: r_4(n) = 8 ∑_{d|n, 4∤d} d.
pub fn jacobi_four_squares_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// Build the modular forms kernel environment.
pub fn build_modular_forms_env() -> Environment {
    let mut env = Environment::new();
    register_modular_forms(&mut env);
    env
}
/// Register all modular form axioms into an existing environment.
pub fn register_modular_forms(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("ModularGroup", modular_group_ty()),
        ("PSL2Z", psl2z_ty()),
        ("GeneratorS", generator_s_ty()),
        ("GeneratorT", generator_t_ty()),
        ("FundamentalDomain", fundamental_domain_ty()),
        ("CongruenceSubgroup", congruence_subgroup_ty()),
        ("Gamma0", gamma0_ty()),
        ("Gamma1", gamma1_ty()),
        ("SubgroupIndex", subgroup_index_ty()),
        ("SubgroupLevel", subgroup_level_ty()),
        ("ModularForm", modular_form_ty()),
        ("CuspForm", cusp_form_ty()),
        ("EisensteinSeries", eisenstein_series_ty()),
        ("GFunction", g_function_ty()),
        ("FourierCoefficient", fourier_coefficient_ty()),
        ("ModularFormWeight", modular_form_weight_ty()),
        ("HolomorphicAtCusps", holomorphic_at_cusps_ty()),
        ("VanishesAtCusps", vanishes_at_cusps_ty()),
        ("PeterssonInnerProduct", petersson_inner_product_ty()),
        ("PeterssonNorm", petersson_norm_ty()),
        ("HeckeOperator", hecke_operator_ty()),
        ("HeckeAlgebra", hecke_algebra_ty()),
        ("DiamondOperator", diamond_operator_ty()),
        ("HeckeEigenform", hecke_eigenform_ty()),
        ("HeckeEigenvalue", hecke_eigenvalue_ty()),
        ("HeckeMultiplicativity", hecke_multiplicativity_ty()),
        ("LFunction_MF", l_function_mf_ty()),
        ("CompletedLFunction", completed_l_function_ty()),
        ("FunctionalEquation_MF", functional_equation_mf_ty()),
        ("RootNumber", root_number_ty()),
        ("EulerProduct_MF", euler_product_mf_ty()),
        ("RamanujanTau", ramanujan_tau_ty()),
        ("DeltaForm", delta_form_ty()),
        ("RamanujanConjecture", ramanujan_conjecture_ty()),
        ("RamanujanCongruence", ramanujan_congruence_ty()),
        ("ThetaSeries", theta_series_ty()),
        ("ThetaSeriesLattice", theta_series_lattice_ty()),
        ("JacobiThetaFunction", jacobi_theta_function_ty()),
        ("ModularSymbol", modular_symbol_ty()),
        ("ManinSymbol", manin_symbol_ty()),
        ("EichlerShimuraPairing", eichler_shimura_pairing_ty()),
        ("EichlerShimuraRelation", eichler_shimura_relation_ty()),
        ("ModularCurve", modular_curve_ty()),
        ("Cusp", cusp_ty()),
        ("JacobianOfModularCurve", jacobian_of_modular_curve_ty()),
        ("Newform", newform_ty()),
        ("OldformSpace", oldform_space_ty()),
        ("NewformDecomposition", newform_decomposition_ty()),
        ("StrongMultiplicityOne", strong_multiplicity_one_ty()),
        ("AtkinLehnerInvolution", atkin_lehner_involution_ty()),
        ("AtkinLehnerEigenvalue", atkin_lehner_eigenvalue_ty()),
        ("GrossZagierFormula", gross_zagier_formula_ty()),
        ("TateTwist_MF", tate_twist_mf_ty()),
        ("GaloisConjugate_MF", galois_conjugate_mf_ty()),
        ("ModularFormDimension", modular_form_dimension_ty()),
        ("CuspFormDimension", cusp_form_dimension_ty()),
        ("PoincareSeries", poincare_series_ty()),
        ("SiegelModularForm", siegel_modular_form_ty()),
        ("HilbertModularForm", hilbert_modular_form_ty()),
        ("ModularFormLift", modular_form_lift_ty()),
        ("SatoTateConjecture", sato_tate_conjecture_ty()),
        (
            "LanglandsCorrespondence_GL2",
            langlands_correspondence_gl2_ty(),
        ),
        ("ModularFormConductor", modular_form_conductor_ty()),
        ("NebentypusCharacter", nebentypus_character_ty()),
        ("HeckeTnMatrix", hecke_tn_matrix_ty()),
        ("HeckeCommutative", hecke_commutative_ty()),
        ("HeckeNormalOperator", hecke_normal_operator_ty()),
        ("NewformEigenSystem", newform_eigen_system_ty()),
        ("MaassForm", maass_form_ty()),
        (
            "LaplaceBeltramiEigenvalue",
            laplace_beltrami_eigenvalue_ty(),
        ),
        ("MaassLFunction", maass_l_function_ty()),
        (
            "SelvergEigenvalueConjecture",
            selberg_eigenvalue_conjecture_ty(),
        ),
        ("WeylLaw", weyl_law_ty()),
        ("HalfIntegerWeightForm", half_integer_weight_form_ty()),
        ("ShimuraCorrespondence", shimura_correspondence_ty()),
        ("WaldspurgerFormula", waldspurger_formula_ty()),
        ("KohnenVariance", kohnen_variance_ty()),
        ("GaloisRepresentation_MF", galois_representation_mf_ty()),
        (
            "EichlerShimuraConstruction",
            eichler_shimura_construction_ty(),
        ),
        ("LAdic_Representation", l_adic_representation_ty()),
        ("DeligneSemiSimplicity", deligne_semisimplicity_ty()),
        (
            "OverconvergentModularForm",
            overconvergent_modular_form_ty(),
        ),
        ("ColemanFamily", coleman_family_ty()),
        ("PAdicLFunction_MF", p_adic_l_function_mf_ty()),
        ("HidaFamily", hida_family_ty()),
        ("EigenvarietyCurve", eigencurve_ty()),
        ("X0N", x0n_ty()),
        ("X1N", x1n_ty()),
        ("CuspResolution", cusp_resolution_ty()),
        ("ModularUnit", modular_unit_ty()),
        ("SiegelUnit", siegel_unit_ty()),
        ("CMPoint", cm_point_ty()),
        ("ShimuraReciprocity", shimura_reciprocity_ty()),
        ("CMTheory", cm_theory_ty()),
        ("HeegnerPoint", heegner_point_ty()),
        ("GrossZagierHeegner", gross_zagier_heegner_ty()),
        ("SatoTateMeasure", sato_tate_measure_ty()),
        ("SatoTateEquidistribution", sato_tate_equidistribution_ty()),
        ("SatoTateLFunction", sato_tate_l_function_ty()),
        ("ModularityLifting", modularity_lifting_ty()),
        ("ResidualRepresentation", residual_representation_ty()),
        ("DeformationRing", deformation_ring_ty()),
        ("TaylorWilesMethod", taylor_wiles_method_ty()),
        (
            "FermatLastTheoremViaModularity",
            fermat_last_theorem_via_modularity_ty(),
        ),
        ("SiegelThetaSeries", siegel_theta_series_ty()),
        ("JacobiFourSquares", jacobi_four_squares_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_modular_forms_env() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("ModularGroup")).is_some());
        assert!(env.get(&Name::str("PSL2Z")).is_some());
        assert!(env.get(&Name::str("FundamentalDomain")).is_some());
    }
    #[test]
    fn test_congruence_subgroups() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("CongruenceSubgroup")).is_some());
        assert!(env.get(&Name::str("Gamma0")).is_some());
        assert!(env.get(&Name::str("Gamma1")).is_some());
        assert!(env.get(&Name::str("SubgroupIndex")).is_some());
    }
    #[test]
    fn test_forms_and_operators() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("ModularForm")).is_some());
        assert!(env.get(&Name::str("CuspForm")).is_some());
        assert!(env.get(&Name::str("EisensteinSeries")).is_some());
        assert!(env.get(&Name::str("HeckeOperator")).is_some());
        assert!(env.get(&Name::str("HeckeAlgebra")).is_some());
        assert!(env.get(&Name::str("DiamondOperator")).is_some());
        assert!(env.get(&Name::str("HeckeEigenform")).is_some());
    }
    #[test]
    fn test_l_functions() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("LFunction_MF")).is_some());
        assert!(env.get(&Name::str("CompletedLFunction")).is_some());
        assert!(env.get(&Name::str("FunctionalEquation_MF")).is_some());
        assert!(env.get(&Name::str("RootNumber")).is_some());
        assert!(env.get(&Name::str("EulerProduct_MF")).is_some());
    }
    #[test]
    fn test_ramanujan_and_theta() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("RamanujanTau")).is_some());
        assert!(env.get(&Name::str("DeltaForm")).is_some());
        assert!(env.get(&Name::str("RamanujanConjecture")).is_some());
        assert!(env.get(&Name::str("RamanujanCongruence")).is_some());
        assert!(env.get(&Name::str("ThetaSeries")).is_some());
        assert!(env.get(&Name::str("JacobiThetaFunction")).is_some());
    }
    #[test]
    fn test_newforms_and_atkin_lehner() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("Newform")).is_some());
        assert!(env.get(&Name::str("NewformDecomposition")).is_some());
        assert!(env.get(&Name::str("StrongMultiplicityOne")).is_some());
        assert!(env.get(&Name::str("AtkinLehnerInvolution")).is_some());
        assert!(env.get(&Name::str("AtkinLehnerEigenvalue")).is_some());
    }
    #[test]
    fn test_eichler_shimura_and_curves() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("ModularSymbol")).is_some());
        assert!(env.get(&Name::str("EichlerShimuraPairing")).is_some());
        assert!(env.get(&Name::str("EichlerShimuraRelation")).is_some());
        assert!(env.get(&Name::str("ModularCurve")).is_some());
        assert!(env.get(&Name::str("JacobianOfModularCurve")).is_some());
    }
    #[test]
    fn test_sl2z_generators_rust() {
        let s = Mat2x2::generator_s();
        let t = Mat2x2::generator_t();
        assert!(s.is_sl2z());
        assert!(t.is_sl2z());
        let s2 = s.mul(&s);
        assert_eq!(
            s2,
            Mat2x2 {
                a: -1,
                b: 0,
                c: 0,
                d: -1
            }
        );
        assert!(t.mul(&t).is_sl2z());
    }
    #[test]
    fn test_eisenstein_and_ramanujan_tau_rust() {
        assert_eq!(sigma_k_minus_1(6, 2), 12);
        assert_eq!(sigma_k_minus_1(6, 1), 4);
        let taus = ramanujan_tau_up_to(5);
        assert_eq!(taus[1], 1, "τ(1) should be 1");
        assert_eq!(taus[2], -24, "τ(2) should be -24");
        assert_eq!(r2(1), 4);
        assert_eq!(r2(5), 8);
    }
    #[test]
    fn test_hecke_operator_rust() {
        let coeffs: Vec<i64> = (0..10).map(|n| sigma_k_minus_1(n, 4) as i64).collect();
        let b = hecke_tp_coefficients(&coeffs, 2, 4);
        assert_eq!(b.len(), coeffs.len());
    }
    #[test]
    fn test_extended_hecke_axioms() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("HeckeTnMatrix")).is_some());
        assert!(env.get(&Name::str("HeckeCommutative")).is_some());
        assert!(env.get(&Name::str("HeckeNormalOperator")).is_some());
        assert!(env.get(&Name::str("NewformEigenSystem")).is_some());
    }
    #[test]
    fn test_maass_forms_axioms() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("MaassForm")).is_some());
        assert!(env.get(&Name::str("LaplaceBeltramiEigenvalue")).is_some());
        assert!(env.get(&Name::str("MaassLFunction")).is_some());
        assert!(env.get(&Name::str("SelvergEigenvalueConjecture")).is_some());
        assert!(env.get(&Name::str("WeylLaw")).is_some());
    }
    #[test]
    fn test_half_integer_weight_axioms() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("HalfIntegerWeightForm")).is_some());
        assert!(env.get(&Name::str("ShimuraCorrespondence")).is_some());
        assert!(env.get(&Name::str("WaldspurgerFormula")).is_some());
        assert!(env.get(&Name::str("KohnenVariance")).is_some());
    }
    #[test]
    fn test_galois_representation_axioms() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("GaloisRepresentation_MF")).is_some());
        assert!(env.get(&Name::str("EichlerShimuraConstruction")).is_some());
        assert!(env.get(&Name::str("LAdic_Representation")).is_some());
        assert!(env.get(&Name::str("DeligneSemiSimplicity")).is_some());
    }
    #[test]
    fn test_padic_overconvergent_axioms() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("OverconvergentModularForm")).is_some());
        assert!(env.get(&Name::str("ColemanFamily")).is_some());
        assert!(env.get(&Name::str("PAdicLFunction_MF")).is_some());
        assert!(env.get(&Name::str("HidaFamily")).is_some());
        assert!(env.get(&Name::str("EigenvarietyCurve")).is_some());
    }
    #[test]
    fn test_modular_curves_units_axioms() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("X0N")).is_some());
        assert!(env.get(&Name::str("X1N")).is_some());
        assert!(env.get(&Name::str("CuspResolution")).is_some());
        assert!(env.get(&Name::str("ModularUnit")).is_some());
        assert!(env.get(&Name::str("SiegelUnit")).is_some());
    }
    #[test]
    fn test_cm_and_heegner_axioms() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("CMPoint")).is_some());
        assert!(env.get(&Name::str("ShimuraReciprocity")).is_some());
        assert!(env.get(&Name::str("CMTheory")).is_some());
        assert!(env.get(&Name::str("HeegnerPoint")).is_some());
        assert!(env.get(&Name::str("GrossZagierHeegner")).is_some());
    }
    #[test]
    fn test_sato_tate_axioms() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("SatoTateMeasure")).is_some());
        assert!(env.get(&Name::str("SatoTateEquidistribution")).is_some());
        assert!(env.get(&Name::str("SatoTateLFunction")).is_some());
    }
    #[test]
    fn test_modularity_lifting_axioms() {
        let env = build_modular_forms_env();
        assert!(env.get(&Name::str("ModularityLifting")).is_some());
        assert!(env.get(&Name::str("ResidualRepresentation")).is_some());
        assert!(env.get(&Name::str("DeformationRing")).is_some());
        assert!(env.get(&Name::str("TaylorWilesMethod")).is_some());
        assert!(env
            .get(&Name::str("FermatLastTheoremViaModularity"))
            .is_some());
        assert!(env.get(&Name::str("SiegelThetaSeries")).is_some());
        assert!(env.get(&Name::str("JacobiFourSquares")).is_some());
    }
    #[test]
    fn test_hecke_operator_matrix_rust() {
        let m = HeckeOperatorMatrix::eigenvalue_matrix(6, 12);
        assert!(m.is_diagonal());
        let expected = sigma_k_minus_1(6, 12) as i64;
        assert_eq!(m.trace(), expected);
    }
    #[test]
    fn test_q_expansion_delta() {
        let qe = QExpansion::delta(6);
        assert!((qe.coeffs[1] - 1.0).abs() < 1e-9);
        assert!((qe.coeffs[2] - (-24.0)).abs() < 1e-9);
        assert_eq!(qe.valuation(), Some(1));
    }
    #[test]
    fn test_q_expansion_eisenstein() {
        let qe = QExpansion::eisenstein(4, 5);
        assert!((qe.coeffs[0] - 1.0).abs() < 1e-9);
        assert!((qe.coeffs[1] - 1.0).abs() < 1e-9);
        assert!((qe.coeffs[2] - 9.0).abs() < 1e-9);
    }
    #[test]
    fn test_ramanujan_tau_function_rust() {
        let rtf = RamanujanTauFunction::new(20);
        assert_eq!(rtf.tau(1), 1);
        assert_eq!(rtf.tau(2), -24);
        assert!(rtf.check_multiplicativity(2, 3));
        assert!(rtf.verify_congruence_691());
    }
    #[test]
    fn test_modular_form_cusp_rust() {
        let inf = ModularFormCusp::infinity(11);
        let zero = ModularFormCusp::zero(11);
        assert!(inf.is_infinity());
        assert!(!zero.is_infinity());
        assert_eq!(inf.width(), 1);
        assert_eq!(ModularFormCusp::cusp_count(11), 2);
        assert_eq!(ModularFormCusp::cusp_count(1), 1);
    }
}
pub fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// Build an environment containing the required named axioms for the spec structs.
pub fn build_env() -> Environment {
    build_modular_forms_env()
}
/// Famous modular forms table.
#[allow(dead_code)]
pub fn famous_modular_forms() -> Vec<(&'static str, u32, u64, &'static str)> {
    vec![
        (
            "Delta",
            12,
            1,
            "Ramanujan's Delta function, generates S_12(SL2Z)",
        ),
        (
            "E4",
            4,
            1,
            "Eisenstein series, E4 = 1 + 240 sum tau3(n) q^n",
        ),
        (
            "E6",
            6,
            1,
            "Eisenstein series, E6 = 1 - 504 sum tau5(n) q^n",
        ),
        (
            "E2*",
            2,
            1,
            "Quasi-modular; weight 2 Eisenstein (non-holomorphic)",
        ),
        (
            "j-function",
            0,
            1,
            "j = E4^3/Delta, modular function (weight 0)",
        ),
        (
            "eta",
            0,
            1,
            "Dedekind eta = q^(1/24) prod (1-q^n), weight 1/2",
        ),
        (
            "theta series",
            0,
            4,
            "theta(z) = sum q^(n^2), weight 1/2 automorphic",
        ),
        (
            "CM newform f_37",
            2,
            37,
            "Associated to elliptic curve y^2=x^3-x",
        ),
        (
            "Ramanujan tau-function",
            12,
            1,
            "tau(n): tau(p) = a_p for Delta",
        ),
        (
            "Mock theta f(q)",
            0,
            1,
            "Ramanujan's third-order mock theta function",
        ),
    ]
}
/// Monstrous moonshine conjecture (proved by Borcherds).
#[allow(dead_code)]
pub fn monstrous_moonshine_data() -> Vec<MoonshineDatum> {
    vec![
        MoonshineDatum::new("1A", "J(q) = q^-1 + 196884q + ...", true),
        MoonshineDatum::new("2A", "T_{2A}(q)", true),
        MoonshineDatum::new("3A", "T_{3A}(q)", true),
        MoonshineDatum::new("5A", "T_{5A}(q)", true),
    ]
}
#[cfg(test)]
mod modular_forms_ext_tests {
    use super::*;
    #[test]
    fn test_dirichlet_character() {
        let chi = DirichletCharacter::legendre_symbol(5);
        assert_eq!(chi.order, 2);
        assert!(chi.is_primitive);
    }
    #[test]
    fn test_hecke_l_function() {
        let l = HeckeLFunction::new("11a1", 2, 11);
        assert!(!l.euler_product_description().is_empty());
    }
    #[test]
    fn test_shimura_variety() {
        let x0 = ShimuraVariety::modular_curve(37);
        assert_eq!(x0.dimension, 1);
        assert!(x0.has_canonical_model());
    }
    #[test]
    fn test_modular_curve_genus() {
        let x0_11 = ModularCurveType::X0(11);
        assert!(x0_11.genus() <= 2);
    }
    #[test]
    fn test_famous_forms_nonempty() {
        let forms = famous_modular_forms();
        assert!(!forms.is_empty());
    }
    #[test]
    fn test_moonshine_data() {
        let md = monstrous_moonshine_data();
        assert!(!md.is_empty());
        assert!(md[0].hauptmodul);
    }
}
/// Modular forms dimension formula (approximate).
#[allow(dead_code)]
pub fn dimension_s_k(k: u32, level: u64) -> u64 {
    if k < 2 || k % 2 != 0 {
        return 0;
    }
    let mu = level;
    if k == 2 {
        if level <= 10 {
            0
        } else {
            mu / 12
        }
    } else {
        ((k - 1) as u64) * mu / 12
    }
}
#[cfg(test)]
mod modular_forms_extra_tests {
    use super::*;
    #[test]
    fn test_petersson_ip() {
        let ip = PeterssonInnerProduct::new(2, 11);
        assert!(ip.hecke_operators_self_adjoint());
        assert!(ip.newforms_orthogonal());
    }
    #[test]
    fn test_automorphic_rep() {
        let pi = AutomorphicRepresentation::classical_newform(37, 2);
        assert!(pi.is_cuspidal);
        assert!(pi.is_tempered);
    }
    #[test]
    fn test_dimension_formula() {
        let d = dimension_s_k(12, 1);
        assert_eq!(d, 0);
        let d2 = dimension_s_k(2, 37);
        assert!(d2 <= 4);
    }
    #[test]
    fn test_rankin_selberg() {
        let rs = RankinSelbergConvolution::new("f", "f");
        assert!(rs.nonvanishing_at_s1());
        assert!(rs.analytic_continuation_entire());
    }
}
