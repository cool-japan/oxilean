//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AbelSum, AbelSumComputer, AbelSummableSequence, CesaroSum, CesaroSumComputer,
    CesaroSummability, KaramataSlowVariation, PrimeSieve, RegularlyVaryingFn, SummabilityMethod,
    TauberianBoundChecker, TauberianCondition, TauberianTheorem,
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
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn add_axiom(
    env: &mut Environment,
    name: &str,
    univ_params: Vec<Name>,
    ty: Expr,
) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params,
        ty,
    })
    .map_err(|e| format!("add_axiom({name}): {e:?}"))
}
/// `HardyLittlewoodTauberianThm : Prop`
///
/// Hardy-Littlewood Tauberian theorem (1914): if ∑ aₙ xⁿ → s as x → 1⁻
/// and n aₙ ≥ -M for all n (one-sided Tauberian condition), then ∑ aₙ = s.
pub fn hardy_littlewood_tauberian_ty() -> Expr {
    prop()
}
/// `HardyLittlewoodTwoSidedThmAxiom : Prop`
///
/// Two-sided Hardy-Littlewood: if the generating function ∑ aₙ xⁿ has Abel
/// limit s and |n aₙ| ≤ M for all n, then the series converges to s.
pub fn hardy_littlewood_two_sided_ty() -> Expr {
    prop()
}
/// `TauberianCondition : Type → Prop`
///
/// A Tauberian condition on a sequence (aₙ): an extra hypothesis that,
/// combined with Abel summability, implies ordinary convergence.
pub fn tauberian_condition_ty() -> Expr {
    arrow(type0(), prop())
}
/// `AbelSummability : Type → Real`
///
/// Abel summability: the series ∑ aₙ is Abel summable to s if
/// lim_{x→1⁻} ∑ aₙ xⁿ = s.
pub fn abel_summability_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `OneSidedTauberianCond : Type → Prop`
///
/// The one-sided Tauberian condition: n aₙ ≥ -M for some M ≥ 0 and all n ≥ 1.
pub fn one_sided_tauberian_cond_ty() -> Expr {
    arrow(type0(), prop())
}
/// `TauberOriginalThm : Prop`
///
/// Tauber's original theorem (1897): if ∑ aₙ xⁿ → s as x → 1 and n aₙ → 0,
/// then ∑ aₙ = s. The precursor to all Tauberian theory.
pub fn tauber_original_thm_ty() -> Expr {
    prop()
}
/// `IkeharaTauberianThm : Prop`
///
/// Ikehara's Tauberian theorem (1931): if F(s) = ∑ aₙ n^{-s} (with aₙ ≥ 0)
/// has an analytic continuation to Re(s) ≥ 1 except for a simple pole at s=1
/// with residue A, then ∑_{n≤x} aₙ ~ Ax as x → ∞.
pub fn ikehara_tauberian_thm_ty() -> Expr {
    prop()
}
/// `IkeharaNoPoleCondition : Prop`
///
/// The Ikehara no-pole condition: the Dirichlet series F(s) - A/(s-1) extends
/// analytically to an open neighborhood of {Re(s) ≥ 1}.
pub fn ikehara_no_pole_condition_ty() -> Expr {
    prop()
}
/// `DirichletSeriesCoeff : Nat → Real`
///
/// The coefficient sequence aₙ of a Dirichlet series F(s) = ∑ aₙ n^{-s}.
pub fn dirichlet_series_coeff_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `DirichletSeriesFunction : Real → Real`
///
/// A Dirichlet series as a function of the real part of s (simplified).
pub fn dirichlet_series_function_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `IkeharaWienerThm : Prop`
///
/// The Ikehara-Wiener theorem: combines Ikehara's Tauberian theorem with
/// Wiener's lemma to give a clean proof of the prime number theorem.
pub fn ikehara_wiener_thm_ty() -> Expr {
    prop()
}
/// `PartialSumAsymptotics : Nat → Real`
///
/// Partial sum asymptotics: A(x) = ∑_{n≤x} aₙ describes the summatory function.
pub fn partial_sum_asymptotics_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `KaramataTauberianThm : Prop`
///
/// Karamata's Tauberian theorem (1931): for a power series f(x) = ∑ aₙ xⁿ
/// with aₙ ≥ 0, if (1-x) f(x) → s as x → 1⁻ and aₙ is non-negative,
/// then the Cesàro mean ∑_{k≤n} aₖ / n → s.
pub fn karamata_tauberian_thm_ty() -> Expr {
    prop()
}
/// `KaramataAbelianThm : Prop`
///
/// Karamata's Abelian theorem: if L is slowly varying and
/// ∑_{n≤x} aₙ ~ x^ρ L(x) / Γ(ρ+1), then the power series ∑ aₙ xⁿ
/// satisfies (1-x)^{-ρ} ∑ aₙ xⁿ → L(1/(1-x)) as x → 1⁻.
pub fn karamata_abelian_thm_ty() -> Expr {
    prop()
}
/// `KaramataRepresentationThm : Prop`
///
/// Karamata's representation theorem: every slowly varying function L can be
/// written as L(x) = c(x) exp(∫₁ˣ ε(t)/t dt) for measurable c → c > 0 and ε → 0.
pub fn karamata_representation_thm_ty() -> Expr {
    prop()
}
/// `KaramataIntegrationThm : Prop`
///
/// Karamata's integration theorem: if L is slowly varying and ρ > -1,
/// then ∫₀ˣ t^ρ L(t) dt ~ x^{ρ+1} L(x) / (ρ+1) as x → ∞.
pub fn karamata_integration_thm_ty() -> Expr {
    prop()
}
/// `WienerTauberianThm : Prop`
///
/// Wiener's Tauberian theorem (1932): if K ∈ L¹(ℝ) has non-vanishing
/// Fourier transform K̂(ξ) ≠ 0 for all ξ, and f * K(x) → A · ∫K as x → ∞,
/// then f * g(x) → A · ∫g as x → ∞ for any g ∈ L¹.
pub fn wiener_tauberian_thm_ty() -> Expr {
    prop()
}
/// `WienerLemma : Prop`
///
/// Wiener's lemma on inverses in L¹: if f ∈ L¹(ℝ) and f̂(ξ) ≠ 0 for all ξ,
/// then 1/f̂ is also the Fourier transform of an L¹ function.
pub fn wiener_lemma_ty() -> Expr {
    prop()
}
/// `SpanOfTranslates : Type → Prop`
///
/// The span of translates of f is dense in L¹ if and only if f̂ has no real zeros
/// (Wiener's characterization of L¹-cyclic elements).
pub fn span_of_translates_ty() -> Expr {
    arrow(type0(), prop())
}
/// `WienerAlgebra : Type`
///
/// The Wiener algebra A(ℝ) = {f ∈ L¹ : f̂ ∈ L¹}: a Banach algebra under
/// pointwise multiplication, with norm ‖f‖_A = ‖f̂‖_{L¹}.
pub fn wiener_algebra_ty() -> Expr {
    type0()
}
/// `BochnerThm : Prop`
///
/// Bochner's theorem: a continuous function φ on ℝ is positive definite if and
/// only if it is the Fourier transform of a finite positive Borel measure.
pub fn bochner_thm_ty() -> Expr {
    prop()
}
/// `TauberianTheoremForL1 : Prop`
///
/// Tauberian theorem for L¹ kernels: extends Wiener's theorem to weighted L¹ spaces.
pub fn tauberian_thm_l1_ty() -> Expr {
    prop()
}
/// `SlowlyVaryingFunction : Type → Prop`
///
/// A measurable function L: (0,∞) → (0,∞) is slowly varying (at infinity) if
/// L(tx)/L(x) → 1 as x → ∞ for every fixed t > 0.
pub fn slowly_varying_function_ty() -> Expr {
    arrow(type0(), prop())
}
/// `RegularlyVaryingFunction : Type → Real → Prop`
///
/// A function f is regularly varying with index ρ ∈ ℝ if f(tx)/f(x) → t^ρ
/// as x → ∞ for every t > 0. Written f ∈ RV_ρ.
pub fn regularly_varying_function_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), prop()))
}
/// `PotterBound : Prop`
///
/// Potter's bound: for L slowly varying and any ε > 0, there exists X₀ such
/// that for x, y ≥ X₀: (x/y)^{-ε} ≤ L(x)/L(y) ≤ (x/y)^ε · (1+ε).
pub fn potter_bound_ty() -> Expr {
    prop()
}
/// `UniformConvergenceThm : Prop`
///
/// Uniform convergence theorem for regular variation: if f ∈ RV_ρ,
/// then f(tx)/f(x) → t^ρ uniformly on compact subsets of (0,∞).
pub fn uniform_convergence_thm_ty() -> Expr {
    prop()
}
/// `ClosureProperties : Prop`
///
/// Closure properties of RV_ρ: sums, products, and compositions of regularly
/// varying functions are regularly varying (with appropriate indices).
pub fn closure_properties_ty() -> Expr {
    prop()
}
/// `KaramataIndex : Type → Real`
///
/// The Karamata (variation) index ρ of a regularly varying function f:
/// ρ = lim_{x→∞} log f(x) / log x.
pub fn karamata_index_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `TauberianForRV : Prop`
///
/// Tauberian theorem for regularly varying functions: if ∑ aₙ is Abel summable
/// to s, aₙ ≥ 0, and A(n) = ∑_{k≤n} aₖ ∈ RV_ρ, then the series converges.
pub fn tauberian_for_rv_ty() -> Expr {
    prop()
}
/// `AbelianTheoremForSeries : Prop`
///
/// The Abelian theorem for power series: if ∑_{n=0}^∞ aₙ = s (convergent),
/// then ∑ aₙ xⁿ → s as x → 1⁻ (the generating function has limit s at 1).
pub fn abelian_thm_series_ty() -> Expr {
    prop()
}
/// `AbelianTheoremForLaplace : Prop`
///
/// The Abelian theorem for Laplace transforms: if F(t) → A as t → ∞ (in a
/// suitable sense), then the Laplace transform f(s) ~ A/s as s → 0⁺.
pub fn abelian_thm_laplace_ty() -> Expr {
    prop()
}
/// `AbelianTheoremForDirichlet : Prop`
///
/// Abelian theorem for Dirichlet series: convergence of ∑ aₙ n^{-s₀} implies
/// holomorphic continuation and limit of F(s) as Re(s) → Re(s₀)⁺.
pub fn abelian_thm_dirichlet_ty() -> Expr {
    prop()
}
/// `CesaroSummability : Type → Real`
///
/// Cesàro summability: the series ∑ aₙ is Cesàro summable (C,1) to s if
/// the arithmetic means (a₀ + a₁ + ... + aₙ)/(n+1) → s.
pub fn cesaro_summability_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `HardyAbelThm : Prop`
///
/// Hardy's theorem: Abel summability implies Cesàro summability (C, 1),
/// but the converse requires a Tauberian condition.
pub fn hardy_abel_thm_ty() -> Expr {
    prop()
}
/// `RiessMeanAxiom : Type → Real`
///
/// Riesz means R^δ of a series: a generalization of Cesàro means with
/// weight (1 - n/N)^δ.
pub fn riesz_mean_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `LittlewoodTauberianThm : Prop`
///
/// Littlewood's Tauberian theorem (1911): if ∑ aₙ xⁿ → s and aₙ = O(1/n),
/// then ∑ aₙ = s. A strengthening of Tauber's original result.
pub fn littlewood_tauberian_thm_ty() -> Expr {
    prop()
}
/// `LaplaceTransformFn : Type → Real`
///
/// The Laplace transform: (Lf)(s) = ∫₀^∞ e^{-st} f(t) dt.
pub fn laplace_transform_fn_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `TauberianForLaplace : Prop`
///
/// Tauberian theorem for Laplace transforms: if (Lf)(s) ~ C s^{-α} L(1/s)
/// as s → 0⁺ (with L slowly varying) and f is non-negative, then
/// F(t) = ∫₀ᵗ f(u) du ~ C t^α L(t) / Γ(α+1).
pub fn tauberian_for_laplace_ty() -> Expr {
    prop()
}
/// `StieltjesTauberianThm : Prop`
///
/// Tauberian theorem for Stieltjes transforms: relates the behavior of
/// f̃(s) = ∫ dμ(t)/(s+t) near s = 0 to the asymptotics of μ([0, x]).
pub fn stieltjes_tauberian_thm_ty() -> Expr {
    prop()
}
/// `HardyLittlewoodMaximalFn : Type → Real`
///
/// The Hardy-Littlewood maximal function Mf(x) = sup_{r>0} (1/2r) ∫_{x-r}^{x+r} |f|.
pub fn hardy_littlewood_maximal_fn_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `LaplaceAsymptotics : Prop`
///
/// Laplace asymptotics: if f(t) ~ L(t) t^{ρ-1} as t → ∞ (L slowly varying),
/// then (Lf)(s) ~ L(1/s) Γ(ρ) s^{-ρ} as s → 0⁺.
pub fn laplace_asymptotics_ty() -> Expr {
    prop()
}
/// `MonotoneTauberianCond : Type → Prop`
///
/// Monotone Tauberian condition: f is ultimately monotone (non-increasing or
/// non-decreasing), which often serves as a Tauberian condition.
pub fn monotone_tauberian_cond_ty() -> Expr {
    arrow(type0(), prop())
}
/// `PostWidderInversionAxiom : Prop`
///
/// Post-Widder inversion formula: recovers f(t) from its Laplace transform
/// via f(t) = lim_{n→∞} (-1)^n (n/t)^{n+1} f^{(n)}(n/t) / n!.
pub fn post_widder_inversion_ty() -> Expr {
    prop()
}
/// `PrimeNumberTheoremTaub : Prop`
///
/// The prime number theorem via Tauberian methods: π(x) ~ x / log(x),
/// proved by applying Ikehara's theorem to the Riemann zeta function.
pub fn prime_number_theorem_taub_ty() -> Expr {
    prop()
}
/// `ZetaFunctionNoPoleOnLine : Prop`
///
/// The non-vanishing of ζ(s) on Re(s) = 1: ζ(1 + it) ≠ 0 for all t ∈ ℝ,
/// a crucial input to the Ikehara-Wiener proof of PNT.
pub fn zeta_no_pole_on_line_ty() -> Expr {
    prop()
}
/// `ZetaSimplePoleAtOne : Prop`
///
/// The Riemann zeta function ζ(s) has a simple pole at s = 1 with residue 1:
/// ζ(s) - 1/(s-1) extends holomorphically to a neighborhood of s = 1.
pub fn zeta_simple_pole_at_one_ty() -> Expr {
    prop()
}
/// `ChebyshevPsiAsymptotics : Prop`
///
/// Chebyshev's ψ-function asymptotics: ψ(x) = ∑_{p^k ≤ x} log p ~ x,
/// equivalent form of the prime number theorem.
pub fn chebyshev_psi_asymptotics_ty() -> Expr {
    prop()
}
/// `MertensTheoremTaub : Prop`
///
/// Mertens' theorem: ∑_{p ≤ x} 1/p = log log x + M + o(1), where M is
/// the Meissel-Mertens constant, proved via elementary Tauberian arguments.
pub fn mertens_theorem_taub_ty() -> Expr {
    prop()
}
/// `PrimeGapsTaub : Prop`
///
/// Prime gaps from Tauberian theory: the average prime gap near x is ~ log x,
/// a consequence of PNT.
pub fn prime_gaps_taub_ty() -> Expr {
    prop()
}
/// `DirichletPNTTaub : Prop`
///
/// Dirichlet's theorem on primes in arithmetic progressions via Tauberian methods:
/// π(x; q, a) ~ x / (φ(q) log x) for gcd(a, q) = 1.
pub fn dirichlet_pnt_taub_ty() -> Expr {
    prop()
}
/// `BombierVinogradovTaub : Prop`
///
/// The Bombieri-Vinogradov theorem: on average over moduli q ≤ x^{1/2-ε},
/// the error terms in π(x; q, a) are small (quasi-GRH on average).
pub fn bombieri_vinogradov_taub_ty() -> Expr {
    prop()
}
/// Populate `env` with all Tauberian theory axioms.
pub fn build_tauberian_theory_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(
        env,
        "HardyLittlewoodTauberianThm",
        vec![],
        hardy_littlewood_tauberian_ty(),
    )?;
    add_axiom(
        env,
        "HardyLittlewoodTwoSidedThm",
        vec![],
        hardy_littlewood_two_sided_ty(),
    )?;
    add_axiom(env, "TauberianCondition", vec![], tauberian_condition_ty())?;
    add_axiom(env, "AbelSummability", vec![], abel_summability_ty())?;
    add_axiom(
        env,
        "OneSidedTauberianCond",
        vec![],
        one_sided_tauberian_cond_ty(),
    )?;
    add_axiom(env, "TauberOriginalThm", vec![], tauber_original_thm_ty())?;
    add_axiom(
        env,
        "IkeharaTauberianThm",
        vec![],
        ikehara_tauberian_thm_ty(),
    )?;
    add_axiom(
        env,
        "IkeharaNoPoleCondition",
        vec![],
        ikehara_no_pole_condition_ty(),
    )?;
    add_axiom(
        env,
        "DirichletSeriesCoeff",
        vec![],
        dirichlet_series_coeff_ty(),
    )?;
    add_axiom(
        env,
        "DirichletSeriesFunction",
        vec![],
        dirichlet_series_function_ty(),
    )?;
    add_axiom(env, "IkeharaWienerThm", vec![], ikehara_wiener_thm_ty())?;
    add_axiom(
        env,
        "PartialSumAsymptotics",
        vec![],
        partial_sum_asymptotics_ty(),
    )?;
    add_axiom(
        env,
        "KaramataTauberianThm",
        vec![],
        karamata_tauberian_thm_ty(),
    )?;
    add_axiom(env, "KaramataAbelianThm", vec![], karamata_abelian_thm_ty())?;
    add_axiom(
        env,
        "KaramataRepresentationThm",
        vec![],
        karamata_representation_thm_ty(),
    )?;
    add_axiom(
        env,
        "KaramataIntegrationThm",
        vec![],
        karamata_integration_thm_ty(),
    )?;
    add_axiom(env, "WienerTauberianThm", vec![], wiener_tauberian_thm_ty())?;
    add_axiom(env, "WienerLemma", vec![], wiener_lemma_ty())?;
    add_axiom(env, "SpanOfTranslates", vec![], span_of_translates_ty())?;
    add_axiom(env, "WienerAlgebra", vec![], wiener_algebra_ty())?;
    add_axiom(env, "BochnerThm", vec![], bochner_thm_ty())?;
    add_axiom(env, "TauberianThmL1", vec![], tauberian_thm_l1_ty())?;
    add_axiom(
        env,
        "SlowlyVaryingFunction",
        vec![],
        slowly_varying_function_ty(),
    )?;
    add_axiom(
        env,
        "RegularlyVaryingFunction",
        vec![],
        regularly_varying_function_ty(),
    )?;
    add_axiom(env, "PotterBound", vec![], potter_bound_ty())?;
    add_axiom(
        env,
        "UniformConvergenceThm",
        vec![],
        uniform_convergence_thm_ty(),
    )?;
    add_axiom(env, "ClosureProperties", vec![], closure_properties_ty())?;
    add_axiom(env, "KaramataIndex", vec![], karamata_index_ty())?;
    add_axiom(env, "TauberianForRV", vec![], tauberian_for_rv_ty())?;
    add_axiom(env, "AbelianThmSeries", vec![], abelian_thm_series_ty())?;
    add_axiom(env, "AbelianThmLaplace", vec![], abelian_thm_laplace_ty())?;
    add_axiom(
        env,
        "AbelianThmDirichlet",
        vec![],
        abelian_thm_dirichlet_ty(),
    )?;
    add_axiom(env, "CesaroSummability", vec![], cesaro_summability_ty())?;
    add_axiom(env, "HardyAbelThm", vec![], hardy_abel_thm_ty())?;
    add_axiom(env, "RiessMean", vec![], riesz_mean_ty())?;
    add_axiom(
        env,
        "LittlewoodTauberianThm",
        vec![],
        littlewood_tauberian_thm_ty(),
    )?;
    add_axiom(env, "LaplaceTransformFn", vec![], laplace_transform_fn_ty())?;
    add_axiom(
        env,
        "TauberianForLaplace",
        vec![],
        tauberian_for_laplace_ty(),
    )?;
    add_axiom(
        env,
        "StieltjesTauberianThm",
        vec![],
        stieltjes_tauberian_thm_ty(),
    )?;
    add_axiom(
        env,
        "HardyLittlewoodMaximalFn",
        vec![],
        hardy_littlewood_maximal_fn_ty(),
    )?;
    add_axiom(env, "LaplaceAsymptotics", vec![], laplace_asymptotics_ty())?;
    add_axiom(
        env,
        "MonotoneTauberianCond",
        vec![],
        monotone_tauberian_cond_ty(),
    )?;
    add_axiom(
        env,
        "PostWidderInversion",
        vec![],
        post_widder_inversion_ty(),
    )?;
    add_axiom(
        env,
        "PrimeNumberTheoremTaub",
        vec![],
        prime_number_theorem_taub_ty(),
    )?;
    add_axiom(env, "ZetaNoPoleonLine", vec![], zeta_no_pole_on_line_ty())?;
    add_axiom(
        env,
        "ZetaSimplePoleAtOne",
        vec![],
        zeta_simple_pole_at_one_ty(),
    )?;
    add_axiom(
        env,
        "ChebyshevPsiAsymptotics",
        vec![],
        chebyshev_psi_asymptotics_ty(),
    )?;
    add_axiom(env, "MertensTheoremTaub", vec![], mertens_theorem_taub_ty())?;
    add_axiom(env, "PrimeGapsTaub", vec![], prime_gaps_taub_ty())?;
    add_axiom(env, "DirichletPNTTaub", vec![], dirichlet_pnt_taub_ty())?;
    add_axiom(
        env,
        "BombierVinogradovTaub",
        vec![],
        bombieri_vinogradov_taub_ty(),
    )?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn test_env() -> Environment {
        let mut env = Environment::new();
        build_tauberian_theory_env(&mut env).expect("build_tauberian_theory_env failed");
        env
    }
    #[test]
    fn test_hardy_littlewood_and_ikehara_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("HardyLittlewoodTauberianThm")).is_some());
        assert!(env.get(&Name::str("HardyLittlewoodTwoSidedThm")).is_some());
        assert!(env.get(&Name::str("TauberOriginalThm")).is_some());
        assert!(env.get(&Name::str("IkeharaTauberianThm")).is_some());
        assert!(env.get(&Name::str("IkeharaWienerThm")).is_some());
    }
    #[test]
    fn test_karamata_and_wiener_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("KaramataTauberianThm")).is_some());
        assert!(env.get(&Name::str("KaramataAbelianThm")).is_some());
        assert!(env.get(&Name::str("KaramataRepresentationThm")).is_some());
        assert!(env.get(&Name::str("WienerTauberianThm")).is_some());
        assert!(env.get(&Name::str("WienerLemma")).is_some());
        assert!(env.get(&Name::str("WienerAlgebra")).is_some());
    }
    #[test]
    fn test_regular_variation_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("SlowlyVaryingFunction")).is_some());
        assert!(env.get(&Name::str("RegularlyVaryingFunction")).is_some());
        assert!(env.get(&Name::str("PotterBound")).is_some());
        assert!(env.get(&Name::str("UniformConvergenceThm")).is_some());
        assert!(env.get(&Name::str("KaramataIndex")).is_some());
    }
    #[test]
    fn test_abelian_and_laplace_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("AbelianThmSeries")).is_some());
        assert!(env.get(&Name::str("AbelianThmLaplace")).is_some());
        assert!(env.get(&Name::str("CesaroSummability")).is_some());
        assert!(env.get(&Name::str("LittlewoodTauberianThm")).is_some());
        assert!(env.get(&Name::str("LaplaceTransformFn")).is_some());
        assert!(env.get(&Name::str("TauberianForLaplace")).is_some());
        assert!(env.get(&Name::str("PostWidderInversion")).is_some());
    }
    #[test]
    fn test_pnt_applications_registered() {
        let env = test_env();
        assert!(env.get(&Name::str("PrimeNumberTheoremTaub")).is_some());
        assert!(env.get(&Name::str("ZetaNoPoleonLine")).is_some());
        assert!(env.get(&Name::str("ZetaSimplePoleAtOne")).is_some());
        assert!(env.get(&Name::str("ChebyshevPsiAsymptotics")).is_some());
        assert!(env.get(&Name::str("MertensTheoremTaub")).is_some());
        assert!(env.get(&Name::str("DirichletPNTTaub")).is_some());
    }
    #[test]
    fn test_abel_summable_sequence() {
        let terms: Vec<f64> = (0..20).map(|n| 0.5_f64.powi(n)).collect();
        let seq = AbelSummableSequence::new(terms, 2.0);
        let gf = seq.generating_function(0.9);
        assert!((gf - 1.0 / (1.0 - 0.5 * 0.9)).abs() < 0.01);
        assert!(seq.ordinary_sum_matches_abel(0.01));
        assert!(seq.tauberian_constant().is_some());
    }
    #[test]
    fn test_regularly_varying_function() {
        let rv = RegularlyVaryingFn::new(2.0, 1.0, 1.0);
        let ratio = rv.check_rv_property(3.0, 100.0);
        assert!((ratio - 9.0).abs() < 1e-6);
        assert_eq!(rv.karamata_index(), 2.0);
        assert!(!rv.is_slowly_varying());
        let sv = RegularlyVaryingFn::new(0.0, 1.0, 1.0);
        assert!(sv.is_slowly_varying());
    }
    #[test]
    fn test_prime_sieve_and_pnt() {
        let sieve = PrimeSieve::new(1000);
        assert_eq!(sieve.prime_counting(100), 25);
        assert_eq!(sieve.prime_counting(1000), 168);
        let ratio = sieve.pnt_ratio(1000);
        assert!(ratio > 1.0 && ratio < 1.3);
        let psi = sieve.chebyshev_psi(10);
        // ψ(10) = 3·ln2 + 2·ln3 + ln5 + ln7 ≈ 7.83
        assert!(psi > 7.0 && psi < 9.0);
    }
}
/// Build an `Environment` with the minimal Tauberian theory kernel axioms.
pub fn build_env() -> oxilean_kernel::Environment {
    let mut env = oxilean_kernel::Environment::new();
    let _ = build_tauberian_theory_env(&mut env);
    env
}
/// `BorelSummability : Type → Real`
///
/// Borel summability: a series ∑ aₙ is Borel summable to s if
/// e^{-x} ∑_{n≥0} (∑_{k=0}^n aₖ) xⁿ / n! → s as x → ∞.
pub fn borel_summability_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `RieszSummabilityMethod : Type → Real → Real`
///
/// Riesz summability (R, λ, κ): generalized weighted means using exponents λₙ
/// and order κ. Generalizes both Cesàro and ordinary convergence.
pub fn riesz_summability_method_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), real_ty()))
}
/// `HausdorffMean : Type → Real`
///
/// Hausdorff means: a matrix summability method defined by a sequence (μₙ)
/// via [Δ^m μ]_n; includes Cesàro and Hölder means as special cases.
pub fn hausdorff_mean_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `MercerianThm : Prop`
///
/// Mercerian theorem (converse Tauberian for Cesàro): if a series is Cesàro
/// summable (C,k) to s and satisfies the Tauberian condition aₙ ≥ -C/n,
/// then it is also Cesàro summable (C, k-1) and eventually ordinarily convergent.
pub fn mercerian_thm_ty() -> Expr {
    prop()
}
/// `HausdorffMercerianThm : Prop`
///
/// Hausdorff's Mercerian theorem: for a Hausdorff matrix H with associated
/// measure μ on [0,1], H is regular iff ∫ t^n dμ(t) → 0 as n → ∞.
pub fn hausdorff_mercerian_thm_ty() -> Expr {
    prop()
}
/// `SummabilityHierarchy : Prop`
///
/// The summability hierarchy: ordinary convergence ⊂ (C,1) ⊂ (C,2) ⊂ ... ⊂ Abel
/// ⊂ Borel, with each inclusion strict.
pub fn summability_hierarchy_ty() -> Expr {
    prop()
}
/// `StrongTauberianCond : Type → Prop`
///
/// Strong Tauberian condition: the difference condition aₙ - aₙ₋₁ = O(1/n),
/// stronger than boundedness alone.
pub fn strong_tauberian_cond_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SlowOscillationCond : Type → Prop`
///
/// Slow oscillation condition (Schmidt 1925): a sequence (sₙ) satisfies
/// the slow oscillation condition if for every ε > 0 there exist N, λ > 1
/// such that |sₙ - sₘ| < ε whenever n, m ≥ N with n/m ≤ λ.
pub fn slow_oscillation_cond_ty() -> Expr {
    arrow(type0(), prop())
}
/// `BoundednessTauberianCond : Type → Prop`
///
/// Boundedness Tauberian condition: the sequence of partial sums (Sₙ) is
/// bounded from below (or above), sufficient for some summability methods.
pub fn boundedness_tauberian_cond_ty() -> Expr {
    arrow(type0(), prop())
}
/// `NewmanTauberianThm : Prop`
///
/// Newman's Tauberian theorem (1980): if f is bounded and holomorphic on
/// Re(s) > 0, and extends continuously to Re(s) = 0, then ∫₀^∞ f(it) e^{xt} dt
/// converges as x → ∞. Gives an elementary proof of PNT.
pub fn newman_tauberian_thm_ty() -> Expr {
    prop()
}
/// `ZagierNewmanProof : Prop`
///
/// Zagier's simplification of Newman's Tauberian theorem: a streamlined version
/// that gives PNT in fewer than a page of mathematical text.
pub fn zagier_newman_proof_ty() -> Expr {
    prop()
}
/// `SelbergDelangeMethod : Prop`
///
/// Selberg-Delange method: for Dirichlet series F(s) = ζ(s)^z G(s) with z ∈ ℂ
/// and G analytic at s = 1, the partial sums satisfy
/// ∑_{n≤x} a_n ~ C x (log x)^{z-1} as x → ∞.
pub fn selberg_delange_method_ty() -> Expr {
    prop()
}
/// `TauberianRemainderThm : Prop`
///
/// Quantitative Tauberian theorem: if (Sₙ) is (C,1) summable and
/// |Sₙ - σₙ| = O(f(n)), then |Sₙ - s| = O(g(n)) for explicit g.
/// Provides error bounds in the Tauberian theorem.
pub fn tauberian_remainder_thm_ty() -> Expr {
    prop()
}
/// `EffectiveTauberianThm : Prop`
///
/// Effective (quantitative) form of Wiener's Tauberian theorem with
/// explicit error terms, useful in analytic number theory.
pub fn effective_tauberian_thm_ty() -> Expr {
    prop()
}
/// `ComplexTauberianCondition : Type → Prop`
///
/// Complex Tauberian condition: the generating function or Laplace transform
/// satisfies a boundary condition on the line Re(s) = σ₀.
pub fn complex_tauberian_condition_ty() -> Expr {
    arrow(type0(), prop())
}
/// `DirichletSeriesAbscissa : Type → Real`
///
/// The abscissa of convergence σ₀ of a Dirichlet series:
/// the infimum of σ such that ∑ aₙ n^{-s} converges for Re(s) > σ.
pub fn dirichlet_series_abscissa_ty() -> Expr {
    arrow(type0(), real_ty())
}
/// `HardyRamanujanThm : Prop`
///
/// Hardy-Ramanujan theorem (1917): for almost all integers n, the number of
/// prime factors ω(n) satisfies |ω(n) - log log n| < (log log n)^{1/2+ε}.
pub fn hardy_ramanujan_thm_ty() -> Expr {
    prop()
}
/// `PartitionAsymptotics : Prop`
///
/// Hardy-Ramanujan asymptotic formula for the partition function:
/// p(n) ~ (1/(4n√3)) · exp(π √(2n/3)) as n → ∞.
/// The first major application of the circle method (Hardy-Ramanujan, 1918).
pub fn partition_asymptotics_ty() -> Expr {
    prop()
}
/// `RademacherSeriesThm : Prop`
///
/// Rademacher's exact formula (1937): the partition function p(n) equals an
/// exact convergent series involving Bessel functions and Kloosterman sums.
pub fn rademacher_series_thm_ty() -> Expr {
    prop()
}
/// `CircleMethodApplication : Prop`
///
/// The Hardy-Littlewood circle method: a technique for extracting coefficients
/// of generating functions via integral over a circle, with major and minor arcs.
pub fn circle_method_application_ty() -> Expr {
    prop()
}
/// `WarigsTheoremPartitions : Prop`
///
/// Waring's problem via circle method: every sufficiently large positive integer
/// is a sum of at most g(k) k-th powers, with g(k) determined explicitly.
pub fn warings_theorem_partitions_ty() -> Expr {
    prop()
}
/// `PartitionFunctionGrowth : Nat → Real`
///
/// The partition function p(n) as a function of n, with its exponential growth rate.
pub fn partition_function_growth_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `DeHaanClass : Type → Prop`
///
/// De Haan's class Π of slowly varying functions with auxiliary function:
/// f ∈ Π if (f(tx) - f(x)) / g(x) → log t for some auxiliary g (slowly varying).
pub fn de_haan_class_ty() -> Expr {
    arrow(type0(), prop())
}
/// `GammaVariation : Type → Prop`
///
/// Γ-variation (de Haan 1970): f ∈ Γ if (f(x + tg(x)) - f(x)) → t
/// for some self-neglecting auxiliary function g.
pub fn gamma_variation_ty() -> Expr {
    arrow(type0(), prop())
}
/// `PiVariation : Type → Prop`
///
/// Π-variation (de Haan): f ∈ Π if (f(tx) - f(x))/g(x) → log t
/// uniformly on compact t-sets, where g is the auxiliary slowly varying function.
pub fn pi_variation_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SecondOrderRegularVariation : Type → Real → Real → Prop`
///
/// Second-order regular variation: f ∈ 2RV(ρ, τ) if
/// (f(tx)/f(x) - t^ρ) / A(x) → t^ρ (t^τ - 1)/τ
/// for some auxiliary function A with A(x) → 0.
pub fn second_order_regular_variation_ty() -> Expr {
    arrow(type0(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `BeurlingSlowlyVaryingFn : Type → Prop`
///
/// Beurling slowly varying function: f is Beurling SVF if for every increasing
/// sequence xₙ → ∞ with xₙ₊₁/xₙ → 1 and f(xₙ) → L ∈ (0,∞), also f(txₙ) → L.
pub fn beurling_slowly_varying_fn_ty() -> Expr {
    arrow(type0(), prop())
}
/// `RepresentationTheoremExtended : Prop`
///
/// Extended representation theorem (de Haan): every function in de Haan's class Π
/// admits a representation via a specific integral involving a slowly varying function.
pub fn representation_thm_extended_ty() -> Expr {
    prop()
}
/// `TauberianForDeHaan : Prop`
///
/// Tauberian theorem for de Haan class: if f ∈ Π and the transform F satisfies
/// a suitable summability condition, then f is eventually monotone.
pub fn tauberian_for_de_haan_ty() -> Expr {
    prop()
}
/// `MellinTauberianThm : Prop`
///
/// Mellin-Tauberian theorem: if the Mellin transform M[f](s) = ∫₀^∞ f(t) t^{s-1} dt
/// has abscissa of convergence σ₀ and extends analytically to Re(s) ≥ σ₀ except for
/// a pole at σ₀, then f(x) ~ C x^{σ₀} L(x) with L slowly varying.
pub fn mellin_tauberian_thm_ty() -> Expr {
    prop()
}
/// `StiltjesTauberianExtended : Prop`
///
/// Extended Stieltjes-Tauberian theorem: the Stieltjes transform
/// S[μ](x) = ∫ dμ(t)/(x + t) satisfies S[μ](x) ~ C x^{-α} L(x) as x → ∞
/// iff μ([0,t]) ~ C t^α L(t) / (Γ(α)Γ(1-α)) for α ∈ (0,1), L slowly varying.
pub fn stieltjes_tauberian_extended_ty() -> Expr {
    prop()
}
/// Populate `env` with the §9–§12 extensions.
pub fn build_tauberian_theory_ext_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "BorelSummability", vec![], borel_summability_ty())?;
    add_axiom(
        env,
        "RieszSummabilityMethod",
        vec![],
        riesz_summability_method_ty(),
    )?;
    add_axiom(env, "HausdorffMean", vec![], hausdorff_mean_ty())?;
    add_axiom(env, "MercerianThm", vec![], mercerian_thm_ty())?;
    add_axiom(
        env,
        "HausdorffMercerianThm",
        vec![],
        hausdorff_mercerian_thm_ty(),
    )?;
    add_axiom(
        env,
        "SummabilityHierarchy",
        vec![],
        summability_hierarchy_ty(),
    )?;
    add_axiom(
        env,
        "StrongTauberianCond",
        vec![],
        strong_tauberian_cond_ty(),
    )?;
    add_axiom(
        env,
        "SlowOscillationCond",
        vec![],
        slow_oscillation_cond_ty(),
    )?;
    add_axiom(
        env,
        "BoundednessTauberianCond",
        vec![],
        boundedness_tauberian_cond_ty(),
    )?;
    add_axiom(env, "NewmanTauberianThm", vec![], newman_tauberian_thm_ty())?;
    add_axiom(env, "ZagierNewmanProof", vec![], zagier_newman_proof_ty())?;
    add_axiom(
        env,
        "SelbergDelangeMethod",
        vec![],
        selberg_delange_method_ty(),
    )?;
    add_axiom(
        env,
        "TauberianRemainderThm",
        vec![],
        tauberian_remainder_thm_ty(),
    )?;
    add_axiom(
        env,
        "EffectiveTauberianThm",
        vec![],
        effective_tauberian_thm_ty(),
    )?;
    add_axiom(
        env,
        "ComplexTauberianCondition",
        vec![],
        complex_tauberian_condition_ty(),
    )?;
    add_axiom(
        env,
        "DirichletSeriesAbscissa",
        vec![],
        dirichlet_series_abscissa_ty(),
    )?;
    add_axiom(env, "HardyRamanujanThm", vec![], hardy_ramanujan_thm_ty())?;
    add_axiom(
        env,
        "PartitionAsymptotics",
        vec![],
        partition_asymptotics_ty(),
    )?;
    add_axiom(
        env,
        "RademacherSeriesThm",
        vec![],
        rademacher_series_thm_ty(),
    )?;
    add_axiom(
        env,
        "CircleMethodApplication",
        vec![],
        circle_method_application_ty(),
    )?;
    add_axiom(
        env,
        "WaringsTheoremPartitions",
        vec![],
        warings_theorem_partitions_ty(),
    )?;
    add_axiom(
        env,
        "PartitionFunctionGrowth",
        vec![],
        partition_function_growth_ty(),
    )?;
    add_axiom(env, "DeHaanClass", vec![], de_haan_class_ty())?;
    add_axiom(env, "GammaVariation", vec![], gamma_variation_ty())?;
    add_axiom(env, "PiVariation", vec![], pi_variation_ty())?;
    add_axiom(
        env,
        "SecondOrderRegularVariation",
        vec![],
        second_order_regular_variation_ty(),
    )?;
    add_axiom(
        env,
        "BeurlingSlowlyVaryingFn",
        vec![],
        beurling_slowly_varying_fn_ty(),
    )?;
    add_axiom(
        env,
        "RepresentationThmExtended",
        vec![],
        representation_thm_extended_ty(),
    )?;
    add_axiom(
        env,
        "TauberianForDeHaan",
        vec![],
        tauberian_for_de_haan_ty(),
    )?;
    add_axiom(env, "MellinTauberianThm", vec![], mellin_tauberian_thm_ty())?;
    add_axiom(
        env,
        "StieltjesTauberianExtended",
        vec![],
        stieltjes_tauberian_extended_ty(),
    )?;
    Ok(())
}
#[cfg(test)]
mod tests_ext {
    use super::*;
    fn test_ext_env() -> Environment {
        let mut env = Environment::new();
        build_tauberian_theory_env(&mut env).expect("base env failed");
        build_tauberian_theory_ext_env(&mut env).expect("ext env failed");
        env
    }
    #[test]
    fn test_summability_methods_registered() {
        let env = test_ext_env();
        assert!(env.get(&Name::str("BorelSummability")).is_some());
        assert!(env.get(&Name::str("HausdorffMean")).is_some());
        assert!(env.get(&Name::str("MercerianThm")).is_some());
        assert!(env.get(&Name::str("SummabilityHierarchy")).is_some());
        assert!(env.get(&Name::str("SlowOscillationCond")).is_some());
    }
    #[test]
    fn test_complex_tauberian_registered() {
        let env = test_ext_env();
        assert!(env.get(&Name::str("NewmanTauberianThm")).is_some());
        assert!(env.get(&Name::str("ZagierNewmanProof")).is_some());
        assert!(env.get(&Name::str("SelbergDelangeMethod")).is_some());
        assert!(env.get(&Name::str("TauberianRemainderThm")).is_some());
        assert!(env.get(&Name::str("DirichletSeriesAbscissa")).is_some());
    }
    #[test]
    fn test_hardy_ramanujan_registered() {
        let env = test_ext_env();
        assert!(env.get(&Name::str("HardyRamanujanThm")).is_some());
        assert!(env.get(&Name::str("PartitionAsymptotics")).is_some());
        assert!(env.get(&Name::str("RademacherSeriesThm")).is_some());
        assert!(env.get(&Name::str("CircleMethodApplication")).is_some());
    }
    #[test]
    fn test_de_haan_registered() {
        let env = test_ext_env();
        assert!(env.get(&Name::str("DeHaanClass")).is_some());
        assert!(env.get(&Name::str("GammaVariation")).is_some());
        assert!(env.get(&Name::str("PiVariation")).is_some());
        assert!(env.get(&Name::str("SecondOrderRegularVariation")).is_some());
        assert!(env.get(&Name::str("BeurlingSlowlyVaryingFn")).is_some());
        assert!(env.get(&Name::str("MellinTauberianThm")).is_some());
    }
    #[test]
    fn test_karamata_slow_variation() {
        let xs: Vec<f64> = (1..=1000).map(|i| i as f64 * 10.0).collect();
        let ls: Vec<f64> = xs.iter().map(|&x| x.ln()).collect();
        let ksv = KaramataSlowVariation::new(xs, ls);
        let ratio = ksv.slow_variation_ratio(2.0, 5000.0);
        assert!(ratio.is_some());
        let r = ratio.expect("ratio should be valid");
        assert!(r > 0.9 && r < 1.2, "ratio = {r}");
        let idx = ksv.estimate_index();
        assert!(idx.abs() < 0.2, "index for log should be near 0, got {idx}");
    }
    #[test]
    fn test_cesaro_sum_computer() {
        let terms: Vec<f64> = (0..100)
            .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
            .collect();
        let comp = CesaroSumComputer::new(terms);
        let c1 = comp.cesaro_c1();
        let last = c1.last().expect("last should succeed");
        assert!((last - 0.5).abs() < 0.05, "C1 mean = {last}");
        assert!(comp.appears_cesaro_summable(1, 0.1));
        let c2_sum = comp.cesaro_sum(2).expect("cesaro_sum should succeed");
        assert!((c2_sum - 0.5).abs() < 0.1, "C2 sum = {c2_sum}");
    }
    #[test]
    fn test_abel_sum_computer() {
        let terms: Vec<f64> = (0..5000)
            .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
            .collect();
        let comp = AbelSumComputer::new(terms);
        let gf = comp.generating_function(0.99);
        assert!((gf - 0.5).abs() < 0.05, "f(0.99) = {gf}");
        let abel = comp.estimate_abel_sum(0.001);
        assert!((abel - 0.5).abs() < 0.05, "Abel sum estimate = {abel}");
        let m = comp.one_sided_tauberian_constant();
        assert!(m >= 0.0);
    }
    #[test]
    fn test_tauberian_bound_checker() {
        let terms: Vec<f64> = (1..=100).map(|n| 1.0 / (n as f64 * n as f64)).collect();
        let target = std::f64::consts::PI * std::f64::consts::PI / 6.0;
        let checker = TauberianBoundChecker::new(terms.clone(), target);
        let m = checker.one_sided_condition();
        assert!(m.is_some() && m.expect("m should be valid") < 1e-10);
        assert!(checker.littlewood_condition(3, 0.2));
        let rem = checker.tauberian_remainder();
        // Max |S_n - π²/6| occurs at S_1 = 1.0, giving ≈ 0.645
        assert!(rem < 1.0, "remainder = {rem}");
        let bound = checker.boundedness_condition();
        assert!(bound < target + 0.1, "bound = {bound}");
        // Cesaro means of 1/n² converge slowly; use larger epsilon
        let idx = checker.cesaro_convergence_index(0.1);
        assert!(idx.is_some());
    }
}
#[cfg(test)]
mod tests_tauberian_extra {
    use super::*;
    #[test]
    fn test_cesaro_sum() {
        let mut cs = CesaroSum::new(1);
        for k in 0..100 {
            let term = if k % 2 == 0 { 1.0 } else { -1.0 };
            cs.add_term(term);
        }
        assert!(cs.converges_to(0.5, 0.01));
    }
    #[test]
    fn test_abel_sum() {
        let mut abel = AbelSum::new();
        for k in 0..20 {
            abel.add_coeff(if k % 2 == 0 { 1.0 } else { -1.0 });
        }
        let val = abel.evaluate_at(0.9);
        assert!(val > 0.4 && val < 0.6, "Abel sum near 0.5, got {val}");
    }
    #[test]
    fn test_summability_hierarchy() {
        assert!(SummabilityMethod::Cesaro1.stronger_than_ordinary());
        assert!(SummabilityMethod::AbelSumm.stronger_than_ordinary());
        assert!(!SummabilityMethod::Ordinary.stronger_than_ordinary());
        assert!(SummabilityMethod::Cesaro1.is_regular());
    }
    #[test]
    fn test_tauberian_theorem() {
        let hl = TauberianTheorem::hardylittlewood();
        assert_eq!(hl.summability_from, SummabilityMethod::AbelSumm);
        assert_eq!(hl.summability_to, SummabilityMethod::Ordinary);
        assert!(hl.is_valid_direction());
    }
}
