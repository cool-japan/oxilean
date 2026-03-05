//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Environment, Expr, Name};
use std::f64::consts::PI;

use super::types::{
    AdditiveSet, ArithmeticProgression, DirichletCharacter, DirichletLFunction, DirichletSeries,
    EllipticCurveLFunction, ExponentialSumBound, GaussSum, GaussSumComputer, GoldbachConjecture,
    GrandRiemannHypothesis, KloostermanSum, LFunction, LFunctionZeroChecker, ModularForm,
    ModularWeight, PrimeCountingFunction, PrimeGap, RiemannHypothesis, SieveEstimator,
    WaringCircleMethod, ZetaFunction,
};

/// Returns the formal statement of the Riemann Hypothesis.
pub fn riemann_hypothesis_statement() -> &'static str {
    "All non-trivial zeros of the Riemann zeta function ζ(s) \
     have real part equal to 1/2."
}
/// Computes ζ(2n) = π^{2n} · |B_{2n}| / (2 · (2n)!) for small positive n.
///
/// Uses the known formula involving Bernoulli numbers. Only accurate for n ≤ 6.
pub fn riemann_zeta_at_even_positive(n: u32) -> f64 {
    let bernoulli_abs: [f64; 7] = [
        1.0,
        1.0 / 6.0,
        1.0 / 30.0,
        1.0 / 42.0,
        1.0 / 30.0,
        5.0 / 66.0,
        691.0 / 2730.0,
    ];
    if n == 0 || n as usize >= bernoulli_abs.len() {
        return f64::NAN;
    }
    let two_n = 2 * n;
    let pi_pow = PI.powi(two_n as i32);
    let b = bernoulli_abs[n as usize];
    let fact: f64 = (1..=(two_n as u64)).map(|k| k as f64).product();
    pi_pow * b / (2.0 * fact) * (2.0_f64.powi(two_n as i32 - 1))
}
/// Returns the formal statement of the Prime Number Theorem.
pub fn prime_number_theorem_statement() -> &'static str {
    "π(x) ~ x / ln(x) as x → ∞, i.e., lim_{x→∞} π(x) · ln(x) / x = 1."
}
/// Computes the von Mangoldt function Λ(n).
///
/// Λ(n) = ln(p) if n = p^k for some prime p and k ≥ 1, and 0 otherwise.
pub fn von_mangoldt_function(n: u64) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    let p = smallest_prime_factor(n);
    let mut m = n;
    while m % p == 0 {
        m /= p;
    }
    if m == 1 {
        (p as f64).ln()
    } else {
        0.0
    }
}
/// Computes Chebyshev's ψ(x) = Σ_{p^k ≤ x} ln(p) (sum of von Mangoldt values).
pub fn chebyshev_psi(x: f64) -> f64 {
    if x < 2.0 {
        return 0.0;
    }
    let limit = x as u64;
    (2..=limit).map(von_mangoldt_function).sum()
}
/// Computes Euler's totient function φ(n) = #{k ≤ n : gcd(k, n) = 1}.
pub fn euler_totient(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = n;
    let mut m = n;
    let mut p = 2u64;
    while p * p <= m {
        if m % p == 0 {
            while m % p == 0 {
                m /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if m > 1 {
        result -= result / m;
    }
    result
}
/// Computes the Mobius function μ(n).
///
/// μ(n) = 1 if n = 1, (-1)^k if n is a product of k distinct primes, 0 if n has a squared factor.
pub fn mobius_function(n: u64) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut m = n;
    let mut factor_count = 0i32;
    let mut p = 2u64;
    while p * p <= m {
        if m % p == 0 {
            factor_count += 1;
            m /= p;
            if m % p == 0 {
                return 0;
            }
        }
        p += 1;
    }
    if m > 1 {
        factor_count += 1;
    }
    if factor_count % 2 == 0 {
        1
    } else {
        -1
    }
}
/// Computes the Liouville function λ(n) = (-1)^{Ω(n)},
/// where Ω(n) is the number of prime factors of n counted with multiplicity.
pub fn liouville_function(n: u64) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut m = n;
    let mut omega = 0u32;
    let mut p = 2u64;
    while p * p <= m {
        while m % p == 0 {
            omega += 1;
            m /= p;
        }
        p += 1;
    }
    if m > 1 {
        omega += 1;
    }
    if omega % 2 == 0 {
        1
    } else {
        -1
    }
}
/// Computes the divisor sum σ_k(n) = Σ_{d | n} d^k.
pub fn divisor_sigma(n: u64, k: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    (1..=n).filter(|&d| n % d == 0).map(|d| d.pow(k)).sum()
}
/// Returns the statement of the Mobius inversion theorem.
pub fn mobius_inversion_theorem() -> &'static str {
    "If g(n) = Σ_{d|n} f(d), then f(n) = Σ_{d|n} μ(d) g(n/d). \
     Equivalently, f = μ * g in the Dirichlet convolution ring."
}
/// Returns a statement of the Selberg sieve method.
pub fn selberg_sieve_statement() -> &'static str {
    "Selberg's sieve provides upper bounds for the number of integers in a set \
     avoiding a given set of residue classes. It gives a sharp upper bound \
     π(x) - π(√x) + 1 ≤ 2 x / ln(x)."
}
/// Returns a statement of Chen's theorem on Goldbach-like representations.
pub fn chen_theorem_statement() -> &'static str {
    "Chen's theorem (1973): Every sufficiently large even integer N can be \
     written as N = p + P₂, where p is prime and P₂ is either prime or a \
     semiprime (product of exactly two primes)."
}
/// Compute gcd(a, b) using the Euclidean algorithm.
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
/// Return the smallest prime factor of n ≥ 2.
pub fn smallest_prime_factor(n: u64) -> u64 {
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            return p;
        }
        p += 1;
    }
    n
}
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
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// `ZetaFunctionTy : Real → Real` — the Riemann zeta function.
pub fn zeta_function_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `RiemannHypothesisTy : Prop` — all non-trivial zeros on the critical line.
pub fn riemann_hypothesis_ty() -> Expr {
    prop()
}
/// `DirichletSeriesTy : Nat → Real → Real` — a Dirichlet series.
pub fn dirichlet_series_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// `DirichletCharacterTy : Nat → Type` — a Dirichlet character of given modulus.
pub fn dirichlet_character_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `DirichletLFunctionTy : Nat → Real → Real` — L(s, χ).
pub fn dirichlet_l_function_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// `PrimeCountingFunctionTy : Real → Nat` — π(x).
pub fn prime_counting_function_ty() -> Expr {
    arrow(real_ty(), nat_ty())
}
/// `VonMangoldtTy : Nat → Real` — Λ(n).
pub fn von_mangoldt_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `ChebyshevPsiTy : Real → Real` — ψ(x).
pub fn chebyshev_psi_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// `EulerTotientTy : Nat → Nat` — φ(n).
pub fn euler_totient_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `MobiusFunctionTy : Nat → Int` — μ(n).
pub fn mobius_function_ty() -> Expr {
    arrow(nat_ty(), cst("Int"))
}
/// `LiouvilleFunctionTy : Nat → Int` — λ(n).
pub fn liouville_function_ty() -> Expr {
    arrow(nat_ty(), cst("Int"))
}
/// `DivisorSigmaTy : Nat → Nat → Nat` — σ_k(n).
pub fn divisor_sigma_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `GaussSumTy : Nat → Real` — τ(χ).
pub fn gauss_sum_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// `KloostermanSumTy : Nat → Nat → Nat → Real` — S(a, b; m).
pub fn kloosterman_sum_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), real_ty())))
}
/// `LFunctionTy : Nat → Real → Real` — a general L-function with conductor.
pub fn l_function_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// `GrandRiemannHypothesisTy : Prop`.
pub fn grand_riemann_hypothesis_ty() -> Expr {
    prop()
}
/// `BirchSwinnertonDyerTy : Prop`.
pub fn birch_swinnerton_dyer_ty() -> Expr {
    prop()
}
/// `PrimeGapTy : Nat → Nat → Nat` — gap between consecutive primes.
pub fn prime_gap_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `BrunConstantTy : Real` — Brun's constant B₂.
pub fn brun_constant_ty() -> Expr {
    real_ty()
}
/// `MobiusInversionTy : Prop` — Mobius inversion formula.
pub fn mobius_inversion_ty() -> Expr {
    prop()
}
/// `ChebotarevDensityAnTy : Prop` — the analytic Chebotarev density theorem.
pub fn chebotarev_density_an_ty() -> Expr {
    prop()
}
/// `PrimeNumberTheoremTy : Prop`.
pub fn prime_number_theorem_ty() -> Expr {
    prop()
}
/// `ChenTheoremTy : Prop`.
pub fn chen_theorem_ty() -> Expr {
    prop()
}
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn pi_impl(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
/// WeylInequalityTy : ∀ (f : Nat → Real) (N : Nat), ExpSum f N ≤ WeylBound f N
pub fn weyl_inequality_ty() -> Expr {
    pi_impl(
        "f",
        arrow(nat_ty(), real_ty()),
        pi_impl("n", nat_ty(), prop()),
    )
}
/// VanDerCorputBProcessTy : Prop — B-process for exponential sums
pub fn van_der_corput_b_process_ty() -> Expr {
    prop()
}
/// ExponentPairsTy : Real → Real → Prop — (k, l) exponent pair property
pub fn exponent_pairs_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// ExpSumBoundTy : Nat → Real → Real — sum bound as function of N and parameter
pub fn exp_sum_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// MajorArcTy : Nat → Real → Real → Prop — major arc around rational a/q
pub fn major_arc_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// MinorArcTy : Real → Prop — minor arc contribution bound
pub fn minor_arc_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// WaringProblemBoundTy : Nat → Nat → Prop — Waring: every n is sum of g(k) k-th powers
pub fn waring_problem_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// GoldbachCircleMethodTy : Prop — Goldbach via Hardy-Littlewood circle method
pub fn goldbach_circle_method_ty() -> Expr {
    prop()
}
/// VinogradovThreePrimesTy : Prop — every large odd integer is sum of 3 primes
pub fn vinogradov_three_primes_ty() -> Expr {
    prop()
}
/// LargeSieveInequalityTy : Prop — Σ |Σ a_n e(nα)|^2 ≤ (N + Q^2) Σ |a_n|^2
pub fn large_sieve_inequality_ty() -> Expr {
    prop()
}
/// BombieriVinogradovTy : Prop — primes equidistributed in AP on average
pub fn bombieri_vinogradov_ty() -> Expr {
    prop()
}
/// BarbanDavenportHalberstamTy : Prop — variance of primes in AP
pub fn barban_davenport_halberstam_ty() -> Expr {
    prop()
}
/// SelbergUpperBoundTy : Nat → Real — Selberg upper bound sieve
pub fn selberg_upper_bound_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// LFunctionalEquationTy : Nat → Prop — L(s,χ) satisfies functional equation
pub fn l_functional_equation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// ConvexityBoundTy : Real → Real — convexity bound |L(1/2+it)| ≤ C q^{1/4}
pub fn convexity_bound_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// LindelofHypothesisTy : Prop — |ζ(1/2+it)| = O(t^ε) for all ε > 0
pub fn lindelof_hypothesis_ty() -> Expr {
    prop()
}
/// SubconvexityBoundTy : Nat → Real → Real — breaking convexity bound
pub fn subconvexity_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), real_ty()))
}
/// DeLaValleePoussinRegionTy : Prop — zero-free region σ > 1 - c/log(t)
pub fn de_la_vallee_poussin_region_ty() -> Expr {
    prop()
}
/// SiegelZeroTy : Real → Prop — existence of exceptional real zero near s=1
pub fn siegel_zero_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// EffectiveChebotarevTy : Prop — effective version of Chebotarev density
pub fn effective_chebotarev_ty() -> Expr {
    prop()
}
/// ZeroFreeBoundTy : Real → Real → Prop — ζ(s) ≠ 0 for σ > f(t)
pub fn zero_free_bound_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// CramerConjectureTy : Prop — p_{n+1} - p_n = O((log p_n)^2)
pub fn cramer_conjecture_ty() -> Expr {
    prop()
}
/// GreenTaoTheoremTy : Prop — primes contain arbitrarily long arithmetic progressions
pub fn green_tao_theorem_ty() -> Expr {
    prop()
}
/// BoundedGapsTy : Prop — lim inf (p_{n+1} - p_n) < ∞
pub fn bounded_gaps_ty() -> Expr {
    prop()
}
/// PrimeGapDistributionTy : Nat → Real — distribution of gaps of size g
pub fn prime_gap_distribution_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// ChebyshevThetaTy : Real → Real — θ(x) = Σ_{p≤x} log p
pub fn chebyshev_theta_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// ExplicitVonMangoldtTy : Real → Real — explicit formula for ψ(x)
pub fn explicit_von_mangoldt_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// PrimesInAPTy : Nat → Nat → Real — π(x; q, a) primes ≡ a (mod q)
pub fn primes_in_ap_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// DirichletTheoremAPTy : Prop — infinitely many primes in any AP with gcd(a,q)=1
pub fn dirichlet_theorem_ap_ty() -> Expr {
    prop()
}
/// CharacterSumTy : Nat → Nat → Real — |Σ χ(n)| ≤ √q log q
pub fn character_sum_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// GaussSumPrimitiveTy : Nat → Real — |τ(χ)| = √q for primitive χ mod q
pub fn gauss_sum_primitive_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// JacobiSumTy : Nat → Nat → Real — J(χ,ψ) = τ(χ)τ(ψ)/τ(χψ)
pub fn jacobi_sum_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// RamanujanSumTy : Nat → Nat → Real — c_q(n) = Σ_{gcd(a,q)=1} e^{2πian/q}
pub fn ramanujan_sum_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// SumProductTheoremTy : Prop — |A+A| + |A·A| ≥ |A|^{1+ε} (Erdős-Szemerédi)
pub fn sum_product_theorem_ty() -> Expr {
    prop()
}
/// SzemerediTrotterTy : Prop — incidence bound for points and lines
pub fn szemeredi_trotter_ty() -> Expr {
    prop()
}
/// AdditiveEnergyTy : Nat → Nat — E(A) = #{a+b=c+d : a,b,c,d ∈ A}
pub fn additive_energy_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// FreimanTheoremTy : Prop — small doubling implies GAP structure
pub fn freiman_theorem_ty() -> Expr {
    prop()
}
/// EulerProductTy : Real → Real — ζ(s) = ∏_p (1 - p^{-s})^{-1} for Re(s) > 1
pub fn euler_product_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// ZeroDensityEstimateTy : Real → Real — N(σ, T) ≤ C T^{A(1-σ)} log^B T
pub fn zero_density_estimate_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// MomentsOfZetaTy : Nat → Real — ∫_0^T |ζ(1/2+it)|^{2k} dt
pub fn moments_of_zeta_ty() -> Expr {
    arrow(nat_ty(), real_ty())
}
/// CriticalLineResTy : Prop — non-trivial zeros have Re(s) = 1/2 (RH)
pub fn critical_line_res_ty() -> Expr {
    prop()
}
/// PrimitiveCharacterTy : Nat → Prop — χ is a primitive character mod q
pub fn primitive_character_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// OrthogonalityRelationTy : Prop — Σ_{χ mod q} χ(m) \overline{χ}(n) = φ(q)·1_{m≡n}
pub fn orthogonality_relation_ty() -> Expr {
    prop()
}
/// LOneChiNonzeroTy : Nat → Prop — L(1, χ) ≠ 0 for non-principal χ
pub fn l_one_chi_nonzero_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// ConductorDiscriminantTy : Nat → Nat — conductor of the character
pub fn conductor_discriminant_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// ModularFormTy : Nat → Nat → Type — modular form of weight k level N
pub fn modular_form_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// RamanujanPeterssonTy : Prop — |a_p| ≤ 2 p^{(k-1)/2} for Hecke eigenvalues
pub fn ramanujan_petersson_ty() -> Expr {
    prop()
}
/// RankinSelbergTy : Prop — L(s, f × g) is entire (for f ≠ g)
pub fn rankin_selberg_ty() -> Expr {
    prop()
}
/// HeckeEigenvalueTy : Nat → Nat → Real — a_n(f) for Hecke eigenform f
pub fn hecke_eigenvalue_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), real_ty()))
}
/// Populate `env` with all analytic number theory axioms and theorems.
pub fn register_analytic_number_theory(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("ZetaFunction", zeta_function_ty()),
        ("RiemannHypothesis", riemann_hypothesis_ty()),
        ("DirichletSeries", dirichlet_series_ty()),
        ("DirichletCharacter", dirichlet_character_ty()),
        ("DirichletLFunction", dirichlet_l_function_ty()),
        ("PrimeCountingFunction", prime_counting_function_ty()),
        ("VonMangoldt", von_mangoldt_ty()),
        ("ChebyshevPsi", chebyshev_psi_ty()),
        ("EulerTotient", euler_totient_ty()),
        ("MobiusFunction", mobius_function_ty()),
        ("LiouvilleFunction", liouville_function_ty()),
        ("DivisorSigma", divisor_sigma_ty()),
        ("GaussSum", gauss_sum_ty()),
        ("KloostermanSum", kloosterman_sum_ty()),
        ("LFunction", l_function_ty()),
        ("GrandRiemannHypothesis", grand_riemann_hypothesis_ty()),
        ("BirchSwinnertonDyer", birch_swinnerton_dyer_ty()),
        ("PrimeGap", prime_gap_ty()),
        ("BrunConstant", brun_constant_ty()),
        ("MobiusInversion", mobius_inversion_ty()),
        ("ChebotarevDensityAn", chebotarev_density_an_ty()),
        ("PrimeNumberTheorem", prime_number_theorem_ty()),
        ("ChenTheorem", chen_theorem_ty()),
        ("WeylInequality", weyl_inequality_ty()),
        ("VanDerCorputBProcess", van_der_corput_b_process_ty()),
        ("ExponentPairs", exponent_pairs_ty()),
        ("ExpSumBound", exp_sum_bound_ty()),
        ("MajorArc", major_arc_ty()),
        ("MinorArc", minor_arc_ty()),
        ("WaringProblemBound", waring_problem_bound_ty()),
        ("GoldbachCircleMethod", goldbach_circle_method_ty()),
        ("VinogradovThreePrimes", vinogradov_three_primes_ty()),
        ("LargeSieveInequality", large_sieve_inequality_ty()),
        ("BombieriVinogradov", bombieri_vinogradov_ty()),
        (
            "BarbanDavenportHalberstam",
            barban_davenport_halberstam_ty(),
        ),
        ("SelbergUpperBound", selberg_upper_bound_ty()),
        ("LFunctionalEquation", l_functional_equation_ty()),
        ("ConvexityBound", convexity_bound_ty()),
        ("LindelofHypothesis", lindelof_hypothesis_ty()),
        ("SubconvexityBound", subconvexity_bound_ty()),
        ("DeLaValleePoussinRegion", de_la_vallee_poussin_region_ty()),
        ("SiegelZero", siegel_zero_ty()),
        ("EffectiveChebotarev", effective_chebotarev_ty()),
        ("ZeroFreeBound", zero_free_bound_ty()),
        ("CramerConjecture", cramer_conjecture_ty()),
        ("GreenTaoTheorem", green_tao_theorem_ty()),
        ("BoundedGaps", bounded_gaps_ty()),
        ("PrimeGapDistribution", prime_gap_distribution_ty()),
        ("ChebyshevTheta", chebyshev_theta_ty()),
        ("ExplicitVonMangoldt", explicit_von_mangoldt_ty()),
        ("PrimesInAP", primes_in_ap_ty()),
        ("DirichletTheoremAP", dirichlet_theorem_ap_ty()),
        ("CharacterSum", character_sum_ty()),
        ("GaussSumPrimitive", gauss_sum_primitive_ty()),
        ("JacobiSum", jacobi_sum_ty()),
        ("RamanujanSum", ramanujan_sum_ty()),
        ("SumProductTheorem", sum_product_theorem_ty()),
        ("SzemerediTrotter", szemeredi_trotter_ty()),
        ("AdditiveEnergy", additive_energy_ty()),
        ("FreimanTheorem", freiman_theorem_ty()),
        ("EulerProduct", euler_product_ty()),
        ("ZeroDensityEstimate", zero_density_estimate_ty()),
        ("MomentsOfZeta", moments_of_zeta_ty()),
        ("CriticalLineRes", critical_line_res_ty()),
        ("PrimitiveCharacter", primitive_character_ty()),
        ("OrthogonalityRelation", orthogonality_relation_ty()),
        ("LOneChiNonzero", l_one_chi_nonzero_ty()),
        ("ConductorDiscriminant", conductor_discriminant_ty()),
        ("ModularForm", modular_form_ty()),
        ("RamanujanPetersson", ramanujan_petersson_ty()),
        ("RankinSelberg", rankin_selberg_ty()),
        ("HeckeEigenvalue", hecke_eigenvalue_ty()),
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
/// Construct the default GoldbachConjecture (unproven).
pub fn goldbach_conjecture() -> GoldbachConjecture {
    GoldbachConjecture::new(
        false,
        "Chen's theorem (1973): every sufficiently large even integer is p + q \
         where p is prime and q has at most two prime factors."
            .to_string(),
    )
}
/// Build an `Environment` populated with analytic number theory axioms.
pub fn build_env() -> oxilean_kernel::Environment {
    use oxilean_kernel::Environment;
    let mut env = Environment::new();
    register_analytic_number_theory(&mut env);
    env
}
/// Compute the modular inverse of `a` modulo `m` using extended Euclidean algorithm.
/// Returns `None` if gcd(a, m) ≠ 1.
pub fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }
    let (mut old_r, mut r) = (a as i64, m as i64);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let q = old_r / r;
        let tmp = r;
        r = old_r - q * r;
        old_r = tmp;
        let tmp = s;
        s = old_s - q * s;
        old_s = tmp;
    }
    if old_r != 1 {
        None
    } else {
        Some(((old_s % m as i64 + m as i64) % m as i64) as u64)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_register_analytic_number_theory() {
        let env = build_env();
        assert!(env.get(&Name::str("ZetaFunction")).is_some());
        assert!(env.get(&Name::str("RiemannHypothesis")).is_some());
        assert!(env.get(&Name::str("PrimeNumberTheorem")).is_some());
    }
    #[test]
    fn test_extended_axioms_registered() {
        let env = build_env();
        assert!(env.get(&Name::str("WeylInequality")).is_some());
        assert!(env.get(&Name::str("ExponentPairs")).is_some());
        assert!(env.get(&Name::str("VinogradovThreePrimes")).is_some());
        assert!(env.get(&Name::str("WaringProblemBound")).is_some());
        assert!(env.get(&Name::str("BombieriVinogradov")).is_some());
        assert!(env.get(&Name::str("LargeSieveInequality")).is_some());
        assert!(env.get(&Name::str("LindelofHypothesis")).is_some());
        assert!(env.get(&Name::str("SubconvexityBound")).is_some());
        assert!(env.get(&Name::str("DeLaValleePoussinRegion")).is_some());
        assert!(env.get(&Name::str("SiegelZero")).is_some());
        assert!(env.get(&Name::str("GreenTaoTheorem")).is_some());
        assert!(env.get(&Name::str("BoundedGaps")).is_some());
        assert!(env.get(&Name::str("DirichletTheoremAP")).is_some());
        assert!(env.get(&Name::str("ExplicitVonMangoldt")).is_some());
        assert!(env.get(&Name::str("JacobiSum")).is_some());
        assert!(env.get(&Name::str("RamanujanSum")).is_some());
        assert!(env.get(&Name::str("SumProductTheorem")).is_some());
        assert!(env.get(&Name::str("FreimanTheorem")).is_some());
        assert!(env.get(&Name::str("ZeroDensityEstimate")).is_some());
        assert!(env.get(&Name::str("MomentsOfZeta")).is_some());
        assert!(env.get(&Name::str("OrthogonalityRelation")).is_some());
        assert!(env.get(&Name::str("LOneChiNonzero")).is_some());
        assert!(env.get(&Name::str("RamanujanPetersson")).is_some());
        assert!(env.get(&Name::str("RankinSelberg")).is_some());
    }
    #[test]
    fn test_gauss_sum_computer_principal() {
        let g = GaussSumComputer::new(5);
        let (re, im) = g.principal_gauss_sum();
        assert!(re.abs() < 2.0);
        assert!(im.abs() < 1e-10);
    }
    #[test]
    fn test_gauss_sum_magnitude_squared() {
        let g = GaussSumComputer::new(7);
        assert_eq!(g.magnitude_squared_primitive(), 7.0);
    }
    #[test]
    fn test_gauss_sum_weil_bound() {
        let g = GaussSumComputer::new(7);
        assert!((g.weil_bound() - 2.0 * 7f64.sqrt()).abs() < 1e-10);
    }
    #[test]
    fn test_ramanujan_sum_at_1() {
        let g = GaussSumComputer::new(5);
        let c = g.ramanujan_sum(1);
        assert!((c - (-1.0)).abs() < 1e-8);
    }
    #[test]
    fn test_ramanujan_sum_at_0() {
        let g = GaussSumComputer::new(5);
        let c = g.ramanujan_sum(0);
        assert!((c - 4.0).abs() < 1e-8);
    }
    #[test]
    fn test_kloosterman_weil_bound() {
        let g = GaussSumComputer::new(11);
        let (re, im) = g.kloosterman_sum(1, 1);
        let magnitude = (re * re + im * im).sqrt();
        assert!(magnitude <= g.weil_bound() + 1e-8);
    }
    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 7), Some(5));
        assert_eq!(mod_inverse(2, 6), None);
    }
    #[test]
    fn test_l_function_zero_checker_riemann() {
        let checker = LFunctionZeroChecker::riemann_zeta();
        assert!(checker.all_zeros_on_critical_line());
        assert!(checker.in_critical_strip(0.5));
        assert!(!checker.in_critical_strip(1.5));
    }
    #[test]
    fn test_l_function_zero_free_bound() {
        let checker = LFunctionZeroChecker::riemann_zeta();
        let bound = checker.zero_free_bound(1000.0);
        assert!(bound > 0.9 && bound < 1.0);
    }
    #[test]
    fn test_l_function_zero_count_estimate() {
        let checker = LFunctionZeroChecker::riemann_zeta();
        let count = checker.zero_count_estimate(100.0);
        assert!(count > 0.0);
    }
    #[test]
    fn test_l_function_siegel_zero() {
        let checker = LFunctionZeroChecker::riemann_zeta();
        assert!(!checker.siegel_zero_possible());
        let checker2 = LFunctionZeroChecker::dirichlet(2000);
        assert!(checker2.siegel_zero_possible());
    }
    #[test]
    fn test_sieve_estimator_large_sieve() {
        let s = SieveEstimator::new(1_000_000, 1000);
        let factor = s.large_sieve_factor();
        assert_eq!(factor, 1_000_000.0 + 1_000_000.0);
    }
    #[test]
    fn test_sieve_estimator_selberg() {
        let s = SieveEstimator::new(1_000_000, 100);
        let bound = s.selberg_upper_bound();
        assert!(bound > 200_000.0 && bound < 250_000.0);
    }
    #[test]
    fn test_sieve_estimator_bombieri_vinogradov() {
        let s = SieveEstimator::new(10_000, 100);
        let q = s.bombieri_vinogradov_q();
        assert!(q > 0.0);
    }
    #[test]
    fn test_sieve_estimator_brun_titchmarsh() {
        let s = SieveEstimator::new(1000, 10);
        let bound = s.brun_titchmarsh_bound(1);
        assert!(bound > 0.0);
    }
    #[test]
    fn test_sieve_prime_count_estimate() {
        let s = SieveEstimator::new(1000, 30);
        let count = s.sieve_prime_count_estimate();
        assert!(count > 50 && count < 200);
    }
    #[test]
    fn test_exp_sum_trivial_bound() {
        let e = ExponentialSumBound::new(1000, 0.01, 0.001);
        assert_eq!(e.trivial_bound(), 1000.0);
    }
    #[test]
    fn test_exp_sum_weyl_bound() {
        let e = ExponentialSumBound::new(1000, 0.01, 0.001);
        let w2 = e.weyl_bound(2);
        assert!((w2 - 1000f64.powf(0.5)).abs() < 1e-8);
    }
    #[test]
    fn test_exp_sum_van_der_corput_a() {
        let e = ExponentialSumBound::new(1000, 0.01, 0.001);
        let bound = e.van_der_corput_a();
        assert!(bound > 0.0 && bound <= 1000.0);
    }
    #[test]
    fn test_exp_sum_van_der_corput_b() {
        let e = ExponentialSumBound::new(1000, 0.01, 0.001);
        let bound = e.van_der_corput_b();
        assert!(bound > 0.0 && bound <= 1000.0);
    }
    #[test]
    fn test_exp_sum_exponent_pair() {
        let e = ExponentialSumBound::new(1000, 0.01, 0.001);
        let bound = e.exponent_pair_bound(1.0 / 6.0, 2.0 / 3.0);
        assert!(bound > 0.0 && bound <= 1000.0);
    }
    #[test]
    fn test_exp_sum_best_bound() {
        let e = ExponentialSumBound::new(1000, 0.01, 0.001);
        let best = e.best_bound(3);
        assert!(best > 0.0 && best <= 1000.0);
        assert!(best <= e.trivial_bound());
    }
}
/// Compute gcd for u64 values.
#[allow(dead_code)]
pub fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// Approximate Euler's totient φ(q) as f64.
#[allow(dead_code)]
pub fn euler_totient_f64(q: u64) -> f64 {
    if q == 0 {
        return 0.0;
    }
    let mut result = q as f64;
    let mut n = q;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result *= 1.0 - 1.0 / (p as f64);
        }
        p += 1;
    }
    if n > 1 {
        result *= 1.0 - 1.0 / (n as f64);
    }
    result
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_modular_form_dimension() {
        // level=1 gives dim=0 since (N-1)=0, so use level=11
        let mf = ModularForm::new(11, ModularWeight::Integer(12), true, true);
        let dim = mf.cusp_form_dimension_approx();
        assert!(dim > 0.0);
    }
    #[test]
    fn test_modular_form_ramanujan_bound() {
        let mf = ModularForm::new(1, ModularWeight::Integer(12), true, true);
        let bound = mf.ramanujan_bound(2);
        assert!(bound > 0.0);
    }
    #[test]
    fn test_circle_method_partition() {
        let p_approx = WaringCircleMethod::partition_asymptotic(100);
        // Hardy-Ramanujan: p(100) ≈ 1.9e8
        assert!(p_approx > 1.0e8);
    }
    #[test]
    fn test_circle_method_waring() {
        let cm = WaringCircleMethod::new(1000, 9, 3);
        assert_eq!(cm.waring_g(), Some(9));
    }
    #[test]
    fn test_arithmetic_progression_validity() {
        let ap = ArithmeticProgression::new(1, 4);
        assert!(ap.is_valid());
        let ap2 = ArithmeticProgression::new(2, 4);
        assert!(!ap2.is_valid());
    }
    #[test]
    fn test_siegel_walfisz_main_term() {
        let ap = ArithmeticProgression::new(1, 4);
        let mt = ap.siegel_walfisz_main_term(1_000_000.0);
        assert!(mt > 0.0);
    }
    #[test]
    fn test_elliptic_curve_discriminant() {
        let ec = EllipticCurveLFunction::new(-1, 0, 0, 0);
        let d = ec.discriminant();
        assert_eq!(d, 64);
        assert!(ec.is_non_singular());
    }
    #[test]
    fn test_bsd_weak() {
        let ec = EllipticCurveLFunction::new(-1, 0, 1, 1);
        assert!(ec.bsd_weak_holds());
    }
    #[test]
    fn test_additive_set_sumset() {
        let a = AdditiveSet::new(vec![0, 1, 2, 3]);
        let s = a.sumset();
        assert_eq!(s.size(), 7);
    }
    #[test]
    fn test_additive_energy() {
        let a = AdditiveSet::new(vec![0, 1, 2]);
        let e = a.additive_energy();
        assert!(e >= 9);
    }
    #[test]
    fn test_doubling_constant() {
        let a = AdditiveSet::new(vec![0, 1, 2, 3, 4]);
        let dc = a.doubling_constant();
        assert!((dc - 1.8).abs() < 0.01);
    }
}
