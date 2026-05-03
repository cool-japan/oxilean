//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ADMMSolver, ConvexSubdifferential, EkelandPrinciple, FunctionSequence, MetricRegularityChecker,
    MordukhovichSubdiffApprox, MountainPassConfig, ProxFnType, ProxRegularSet, ProximalOperator,
    ProximalPointSolver,
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
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
pub fn vec_ty() -> Expr {
    list_ty(real_ty())
}
pub fn seq_ty(elem: Expr) -> Expr {
    fn_ty(nat_ty(), elem)
}
/// `RegularSubdifferential : (List Real -> Real) -> List Real -> (List Real -> Prop)`
/// F-subdifferential (Fréchet): v ∈ D̂f(x) iff liminf_{y→x} (f(y)-f(x)-⟨v,y-x⟩)/‖y-x‖ ≥ 0.
pub fn regular_subdifferential_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `LimitingSubdifferential : (List Real -> Real) -> List Real -> (List Real -> Prop)`
/// Mordukhovich (basic) subdifferential: cluster points of regular subgradients along sequences.
pub fn limiting_subdifferential_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `ClarkeSubdifferential : (List Real -> Real) -> List Real -> (List Real -> Prop)`
/// ∂^C f(x) = conv(∂_L f(x)): convex hull of limiting subdifferential.
pub fn clarke_subdifferential_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `ClarkeGeneralizedGradient : (List Real -> Real) -> List Real -> List Real`
/// A selection from Clarke subdifferential (not unique in general).
pub fn clarke_generalized_gradient_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), fn_ty(vec_ty(), vec_ty()))
}
/// `MordukhovichCriterion : (List Real -> Prop) -> List Real -> Prop`
/// Mordukhovich normal cone regularity criterion for closed sets.
pub fn mordukhovich_criterion_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), arrow(vec_ty(), prop()))
}
/// `LimitingNormalCone : (List Real -> Prop) -> List Real -> (List Real -> Prop)`
/// N_L(C; x) = Limsup_{y→x, y∈C} N̂(C; y).
pub fn limiting_normal_cone_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), prop()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `SubdiffSumRule : Prop`
/// Limiting subdiff of sum: ∂(f+g)(x) ⊆ ∂f(x) + ∂g(x) under SNEC condition.
pub fn subdiff_sum_rule_ty() -> Expr {
    prop()
}
/// `SubdiffChainRule : Prop`
/// Chain rule for limiting subdifferential of composed mappings.
pub fn subdiff_chain_rule_ty() -> Expr {
    prop()
}
/// `FuzzySum : Prop`
/// Fuzzy (approximate) sum rule for regular subdifferentials.
pub fn fuzzy_sum_rule_ty() -> Expr {
    prop()
}
/// `IsProxRegularSet : (List Real -> Prop) -> Real -> Prop`
/// C is r-prox-regular: the projection onto C is single-valued on a tube of radius r.
pub fn is_prox_regular_set_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), arrow(real_ty(), prop()))
}
/// `IsProxRegularFunction : (List Real -> Real) -> Real -> Prop`
/// f is r-prox-regular: its epigraph is r-prox-regular as a set.
pub fn is_prox_regular_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `ProxRegularityProjection : (List Real -> Prop) -> Real -> Prop`
/// For r-prox-regular C, proj_C is single-valued on the open r-tube around C.
pub fn prox_regularity_projection_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), arrow(real_ty(), prop()))
}
/// `SubsmoothFunction : (List Real -> Real) -> Prop`
/// A subsmooth function: regular subdifferential equals limiting subdifferential everywhere.
pub fn subsmooth_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `UniformlyProxRegular : (List Real -> Prop) -> Real -> Prop`
/// Uniformly r-prox-regular: same r works at every boundary point.
pub fn uniformly_prox_regular_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), arrow(real_ty(), prop()))
}
/// `EpiConverges : (Nat -> List Real -> Real) -> (List Real -> Real) -> Prop`
/// f_n epi-converges to f: epi(f_n) → epi(f) in the set convergence sense.
pub fn epi_converges_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    let seq_fn = seq_ty(rn_r.clone());
    arrow(seq_fn, arrow(rn_r, prop()))
}
/// `EpiLimInf : (Nat -> List Real -> Real) -> List Real -> Real`
/// (e-liminf f_n)(x) = liminf_{y_n → x} liminf_n f_n(y_n).
pub fn epi_liminf_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(seq_ty(rn_r.clone()), rn_r)
}
/// `EpiLimSup : (Nat -> List Real -> Real) -> List Real -> Real`
/// (e-limsup f_n)(x) = liminf_{y_n → x} limsup_n f_n(y_n).
pub fn epi_limsup_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(seq_ty(rn_r.clone()), rn_r)
}
/// `MoscoConverges : (Nat -> List Real -> Real) -> (List Real -> Real) -> Prop`
/// Mosco convergence: combines strong epi and weak epi convergence conditions.
pub fn mosco_converges_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(seq_ty(rn_r.clone()), arrow(rn_r, prop()))
}
/// `EpiConvergenceImpliesMinConvergence : Prop`
/// If f_n epi-converges to f then inf f_n → inf f and argmin_n converges.
pub fn epi_conv_implies_min_ty() -> Expr {
    prop()
}
/// `MoscoImpliesEpi : Prop`
/// Mosco convergence implies epi-convergence (in reflexive Banach spaces).
pub fn mosco_implies_epi_ty() -> Expr {
    prop()
}
/// `GammaLimInf : (Nat -> List Real -> Real) -> List Real -> Real`
/// Γ-liminf_n f_n(x) = sup_{U∋x} liminf_n inf_{y∈U} f_n(y).
pub fn gamma_liminf_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(seq_ty(rn_r.clone()), rn_r)
}
/// `GammaLimSup : (Nat -> List Real -> Real) -> List Real -> Real`
/// Γ-limsup_n f_n(x) = sup_{U∋x} lim_n inf_{y∈U} f_n(y).
pub fn gamma_limsup_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(seq_ty(rn_r.clone()), rn_r)
}
/// `GammaConverges : (Nat -> List Real -> Real) -> (List Real -> Real) -> Prop`
/// f_n Γ-converges to f iff Γ-liminf = Γ-limsup = f.
pub fn gamma_converges_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(seq_ty(rn_r.clone()), arrow(rn_r, prop()))
}
/// `GammaCompactness : Prop`
/// Every sequence of equi-coercive functions has a Γ-convergent subsequence.
pub fn gamma_compactness_ty() -> Expr {
    prop()
}
/// `GammaConvergenceMinimisers : Prop`
/// Γ-convergence ensures convergence of minimisers and minimum values.
pub fn gamma_conv_minimisers_ty() -> Expr {
    prop()
}
/// `GammaConvergenceStability : Prop`
/// Γ-convergence is stable under addition of continuous perturbations.
pub fn gamma_conv_stability_ty() -> Expr {
    prop()
}
/// `EkelandVariationalPrinciple : (List Real -> Real) -> Prop`
/// For every ε > 0 and near-minimiser x₀ of lsc proper f bounded below,
/// ∃ xε: f(xε) ≤ f(x₀) and f(xε) ≤ f(x) + ε‖x-xε‖ for all x.
pub fn ekeland_variational_principle_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `BorweinPreissPrinciple : (List Real -> Real) -> Prop`
/// Smooth variational principle: perturbation by a smooth gauge function.
pub fn borwein_preiss_principle_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `DensityOfMinimisers : (List Real -> Real) -> Prop`
/// The set of strict minimisers is dense (consequence of variational principles).
pub fn density_of_minimisers_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `StarShapedMinimisation : Prop`
/// Variational principle for star-shaped constraint sets.
pub fn star_shaped_minimisation_ty() -> Expr {
    prop()
}
/// `TrustRegionPrinciple : Prop`
/// Trust-region model for approximate variational principles.
pub fn trust_region_principle_ty() -> Expr {
    prop()
}
/// `PalaisSmaleCondition : (List Real -> Real) -> Prop`
/// (PS): every sequence with bounded values and gradient → 0 has a convergent subsequence.
pub fn palais_smale_condition_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `Cerami condition : (List Real -> Real) -> Prop`
/// Cerami variant of (PS): (1 + ‖x_n‖)‖∇f(x_n)‖ → 0.
pub fn cerami_condition_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `MountainPassTheorem : (List Real -> Real) -> Prop`
/// If f satisfies (PS) and has a mountain-pass geometry, ∃ critical point.
pub fn mountain_pass_theorem_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `SaddlePointTheorem : (List Real -> Real) -> Prop`
/// Generalisation of mountain pass to saddle geometries.
pub fn saddle_point_theorem_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `LinkingTheorem : (List Real -> Real) -> Prop`
/// Abstract linking theorem yielding critical points.
pub fn linking_theorem_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `MorseIndex : (List Real -> Real) -> List Real -> Nat`
/// Index of a nondegenerate critical point (dimension of negative eigenspace of Hessian).
pub fn morse_index_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), fn_ty(vec_ty(), nat_ty()))
}
/// `MorseInequalities : (List Real -> Real) -> Prop`
/// Morse inequalities relating critical points and Betti numbers.
pub fn morse_inequalities_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `DeformationLemma : (List Real -> Real) -> Prop`
/// Deformation retract of sub-level sets avoiding critical levels.
pub fn deformation_lemma_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// Build an [`Environment`] containing variational analysis axioms and theorems.
pub fn build_variational_analysis_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("RegularSubdifferential", regular_subdifferential_ty()),
        ("LimitingSubdifferential", limiting_subdifferential_ty()),
        ("ClarkeSubdifferential", clarke_subdifferential_ty()),
        (
            "ClarkeGeneralizedGradient",
            clarke_generalized_gradient_ty(),
        ),
        ("MordukhovichCriterion", mordukhovich_criterion_ty()),
        ("LimitingNormalCone", limiting_normal_cone_ty()),
        ("SubdiffSumRule", subdiff_sum_rule_ty()),
        ("SubdiffChainRule", subdiff_chain_rule_ty()),
        ("FuzzySumRule", fuzzy_sum_rule_ty()),
        ("IsProxRegularSet", is_prox_regular_set_ty()),
        ("IsProxRegularFunction", is_prox_regular_function_ty()),
        ("ProxRegularityProjection", prox_regularity_projection_ty()),
        ("SubmootFunction", subsmooth_function_ty()),
        ("UniformlyProxRegular", uniformly_prox_regular_ty()),
        ("EpiConverges", epi_converges_ty()),
        ("EpiLimInf", epi_liminf_ty()),
        ("EpiLimSup", epi_limsup_ty()),
        ("MoscoConverges", mosco_converges_ty()),
        ("EpiConvImpliesMin", epi_conv_implies_min_ty()),
        ("MoscoImpliesEpi", mosco_implies_epi_ty()),
        ("GammaLimInf", gamma_liminf_ty()),
        ("GammaLimSup", gamma_limsup_ty()),
        ("GammaConverges", gamma_converges_ty()),
        ("GammaCompactness", gamma_compactness_ty()),
        ("GammaConvergenceMinimisers", gamma_conv_minimisers_ty()),
        ("GammaConvergenceStability", gamma_conv_stability_ty()),
        (
            "EkelandVariationalPrinciple",
            ekeland_variational_principle_ty(),
        ),
        ("BorweinPreissPrinciple", borwein_preiss_principle_ty()),
        ("DensityOfMinimisers", density_of_minimisers_ty()),
        ("StarShapedMinimisation", star_shaped_minimisation_ty()),
        ("TrustRegionPrinciple", trust_region_principle_ty()),
        ("PalaisSmaleCondition", palais_smale_condition_ty()),
        ("CeramiCondition", cerami_condition_ty()),
        ("MountainPassTheorem", mountain_pass_theorem_ty()),
        ("SaddlePointTheorem", saddle_point_theorem_ty()),
        ("LinkingTheorem", linking_theorem_ty()),
        ("MorseIndex", morse_index_ty()),
        ("MorseInequalities", morse_inequalities_ty()),
        ("DeformationLemma", deformation_lemma_ty()),
    ];
    for &(name, ref ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// Compute a Clarke-type subgradient via smoothing (convolution with Gaussian).
/// For locally Lipschitz f, approximate Clarke gradient by averaging finite differences.
pub fn clarke_gradient_approx(f: impl Fn(&[f64]) -> f64, x: &[f64], epsilon: f64) -> Vec<f64> {
    let n = x.len();
    let mut grad = vec![0.0; n];
    let h = epsilon;
    for i in 0..n {
        let mut xp = x.to_vec();
        xp[i] += h;
        let mut xm = x.to_vec();
        xm[i] -= h;
        grad[i] = (f(&xp) - f(&xm)) / (2.0 * h);
    }
    grad
}
/// Check whether a vector is a regular (Fréchet) subgradient:
/// liminf_{y→x} (f(y) - f(x) - ⟨v, y-x⟩) / ‖y-x‖ ≥ -tol
/// by sampling random directions.
pub fn check_regular_subgradient(
    f: impl Fn(&[f64]) -> f64,
    x: &[f64],
    v: &[f64],
    h: f64,
    tol: f64,
) -> bool {
    let n = x.len();
    let fx = f(x);
    for i in 0..n {
        for sign in &[-1.0_f64, 1.0_f64] {
            let mut d = vec![0.0; n];
            d[i] = *sign;
            let y: Vec<f64> = x.iter().zip(d.iter()).map(|(xi, di)| xi + h * di).collect();
            let fy = f(&y);
            let inner: f64 = v.iter().zip(d.iter()).map(|(vi, di)| vi * di * h).sum();
            let ratio = (fy - fx - inner) / h;
            if ratio < -tol {
                return false;
            }
        }
    }
    true
}
/// Approximate Ekeland minimiser: finds x_ε near x₀ such that
/// f(x_ε) ≤ f(x₀) and f(x_ε) ≤ f(x) + eps * ‖x - x_ε‖ for all tested x.
pub fn ekeland_approximate_minimiser(
    f: impl Fn(&[f64]) -> f64,
    x0: &[f64],
    eps: f64,
    max_iter: usize,
) -> Vec<f64> {
    let mut x = x0.to_vec();
    let step = eps * 0.1;
    let n = x.len();
    let mut fx = f(&x);
    for _ in 0..max_iter {
        let mut improved = false;
        for i in 0..n {
            for &sign in &[-1.0_f64, 1.0_f64] {
                let mut y = x.clone();
                y[i] += sign * step;
                let fy = f(&y);
                let dist: f64 = x
                    .iter()
                    .zip(y.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f64>()
                    .sqrt();
                if fy + eps * dist < fx - 1e-12 {
                    fx = fy;
                    x = y;
                    improved = true;
                }
            }
        }
        if !improved {
            break;
        }
    }
    x
}
/// Check whether a finite sequence is a Palais-Smale sequence:
/// |f(x_n)| bounded AND ‖∇f(x_n)‖ → 0.
pub fn is_palais_smale_sequence(
    f: impl Fn(&[f64]) -> f64,
    grad_f: impl Fn(&[f64]) -> Vec<f64>,
    sequence: &[Vec<f64>],
    bound: f64,
) -> bool {
    if sequence.is_empty() {
        return true;
    }
    for x in sequence {
        if f(x).abs() > bound {
            return false;
        }
    }
    let last = sequence
        .last()
        .expect("sequence is non-empty: checked by early return");
    let g = grad_f(last);
    let norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
    norm < 0.1 * bound.max(1.0)
}
/// Compute penalty approximation of indicator function of \[a, b\]:
/// f_n(x) = n * (max(0, x - b) + max(0, a - x)).
pub fn penalty_indicator_1d(x: f64, a: f64, b: f64, n: f64) -> f64 {
    n * (0.0_f64.max(x - b) + 0.0_f64.max(a - x))
}
/// Check whether a sequence of penalty functions Gamma-converges to the indicator of \[a,b\].
/// Only verifies the liminf inequality at a test point.
pub fn check_gamma_convergence_indicator(x: f64, a: f64, b: f64, ns: &[f64]) -> bool {
    let inside = x >= a && x <= b;
    if inside {
        ns.iter()
            .all(|&n| (penalty_indicator_1d(x, a, b, n) - 0.0).abs() < 1e-12)
    } else {
        let vals: Vec<f64> = ns
            .iter()
            .map(|&n| penalty_indicator_1d(x, a, b, n))
            .collect();
        vals.windows(2).all(|w| w[1] >= w[0] - 1e-12)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_env_keys() {
        let env = build_variational_analysis_env();
        assert!(env.get(&Name::str("RegularSubdifferential")).is_some());
        assert!(env.get(&Name::str("LimitingSubdifferential")).is_some());
        assert!(env.get(&Name::str("ClarkeSubdifferential")).is_some());
        assert!(env.get(&Name::str("EkelandVariationalPrinciple")).is_some());
        assert!(env.get(&Name::str("MountainPassTheorem")).is_some());
        assert!(env.get(&Name::str("PalaisSmaleCondition")).is_some());
        assert!(env.get(&Name::str("GammaConverges")).is_some());
        assert!(env.get(&Name::str("MoscoConverges")).is_some());
    }
    #[test]
    fn test_regular_subgradient_smooth() {
        let f = |x: &[f64]| x[0] * x[0];
        let x = vec![1.0];
        let v = vec![2.0];
        assert!(check_regular_subgradient(f, &x, &v, 1e-4, 1e-5));
    }
    #[test]
    fn test_clarke_gradient_smooth_quadratic() {
        let f = |x: &[f64]| 0.5 * x[0] * x[0];
        let x = vec![2.0];
        let g = clarke_gradient_approx(f, &x, 1e-6);
        assert!((g[0] - 2.0).abs() < 1e-4, "Clarke grad = {}", g[0]);
    }
    #[test]
    fn test_epi_liminf_constant_sequence() {
        let fns: Vec<Box<dyn Fn(&[f64]) -> f64 + Send + Sync>> = (0..5)
            .map(|_| Box::new(|x: &[f64]| x[0] * x[0]) as Box<dyn Fn(&[f64]) -> f64 + Send + Sync>)
            .collect();
        let seq = FunctionSequence::new(fns);
        let x = vec![0.0];
        let val = seq.epi_liminf(&x, 0.5, 10);
        assert!(val >= -1e-9, "epi-liminf at 0 = {val}");
        assert!(val < 0.26, "epi-liminf at 0 too large: {val}");
    }
    #[test]
    fn test_mountain_pass_geometry() {
        let f = |x: &[f64]| x[0].powi(4) - 2.0 * x[0].powi(2);
        let mut cfg = MountainPassConfig::new(vec![-1.0], vec![1.0], 100);
        let pass_level = cfg.estimate_pass_level(&f);
        assert!((pass_level - 0.0).abs() < 1e-9, "pass level = {pass_level}");
        assert!(cfg.has_mountain_pass_geometry(&f));
    }
    #[test]
    fn test_ekeland_approximate_minimiser() {
        let f = |x: &[f64]| x[0] * x[0];
        let x_eps = ekeland_approximate_minimiser(f, &[3.0], 1.0, 1000);
        assert!(x_eps[0].abs() < 1.0, "Ekeland minimiser at {}", x_eps[0]);
    }
    #[test]
    fn test_palais_smale_sequence() {
        let f = |x: &[f64]| x[0] * x[0];
        let grad_f = |x: &[f64]| vec![2.0 * x[0]];
        let seq: Vec<Vec<f64>> = vec![vec![0.1], vec![0.01], vec![0.001]];
        assert!(is_palais_smale_sequence(f, grad_f, &seq, 10.0));
    }
    #[test]
    fn test_prox_regular_set() {
        let pts: Vec<Vec<f64>> = (0..20)
            .map(|k| {
                let theta = k as f64 * std::f64::consts::TAU / 20.0;
                vec![theta.cos(), theta.sin()]
            })
            .collect();
        let prs = ProxRegularSet::new(pts, 0.5);
        let x = vec![0.8, 0.0];
        assert!(
            prs.has_unique_projection(&x),
            "near point should have unique projection"
        );
    }
    #[test]
    fn test_gamma_convergence_indicator() {
        let ns = vec![1.0, 2.0, 5.0, 10.0, 50.0, 100.0];
        assert!(check_gamma_convergence_indicator(0.5, 0.0, 1.0, &ns));
        assert!(check_gamma_convergence_indicator(2.0, 0.0, 1.0, &ns));
    }
}
/// Build an `Environment` with variational analysis kernel axioms.
pub fn build_env() -> oxilean_kernel::Environment {
    build_variational_analysis_env()
}
/// `TopologicalEpiConvergence : (Nat -> List Real -> Real) -> (List Real -> Real) -> Prop`
/// Epi-convergence in a general topological space setting.
pub fn topological_epi_convergence_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(seq_ty(rn_r.clone()), arrow(rn_r, prop()))
}
/// `MoscoConvergenceReflexiveBanach : Prop`
/// In reflexive Banach spaces, Mosco convergence is equivalent to weak epi + strong epi convergence.
pub fn mosco_convergence_reflexive_banach_ty() -> Expr {
    prop()
}
/// `EpiConvergenceStability : Prop`
/// Epi-convergence is stable under continuous perturbations of the functionals.
pub fn epi_convergence_stability_ty() -> Expr {
    prop()
}
/// `MetricRegularity : (List Real -> List Real -> Prop) -> Real -> Prop`
/// F is metrically regular at (x,y) with constant κ: d(x, F⁻¹(y)) ≤ κ d(y, F(x)).
pub fn metric_regularity_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), fn_ty(vec_ty(), prop())),
        arrow(real_ty(), prop()),
    )
}
/// `LinearRegularity : (List (List Real -> Prop)) -> Prop`
/// A collection of sets is linearly regular: d(x, ∩Cᵢ) ≤ κ max d(x, Cᵢ).
pub fn linear_regularity_ty() -> Expr {
    arrow(list_ty(fn_ty(vec_ty(), prop())), prop())
}
/// `LipschitzStability : (List Real -> List Real) -> Real -> Prop`
/// A map F is Lipschitz-stable (calm) with constant κ near a point.
pub fn lipschitz_stability_ty() -> Expr {
    arrow(fn_ty(vec_ty(), vec_ty()), arrow(real_ty(), prop()))
}
/// `RobinsonStability : (List Real -> List Real -> Prop) -> Prop`
/// Robinson's stability theorem: MFCQ implies Lipschitz stability of solution maps.
pub fn robinson_stability_ty() -> Expr {
    arrow(fn_ty(vec_ty(), fn_ty(vec_ty(), prop())), prop())
}
/// `AubinPropertyCriteria : (List Real -> List Real -> Prop) -> Prop`
/// Aubin (pseudo-Lipschitz) property: set-valued map has Lipschitz-like continuity.
pub fn aubin_property_criteria_ty() -> Expr {
    arrow(fn_ty(vec_ty(), fn_ty(vec_ty(), prop())), prop())
}
/// `ClarkeSubdiffSumRule : Prop`
/// Clarke's sum rule: ∂^C(f+g)(x) ⊆ ∂^C f(x) + ∂^C g(x).
pub fn clarke_subdiff_sum_rule_ty() -> Expr {
    prop()
}
/// `ClarkeSubdiffChainRule : Prop`
/// Clarke's chain rule for composite locally Lipschitz functions.
pub fn clarke_subdiff_chain_rule_ty() -> Expr {
    prop()
}
/// `MordukhovichSubdiffSumRule : Prop`
/// Mordukhovich sum rule with SNC (sequential normal compactness) condition.
pub fn mordukhovich_subdiff_sum_rule_ty() -> Expr {
    prop()
}
/// `MordukhovichSubdiffChainRule : Prop`
/// Mordukhovich chain rule for compositions of Lipschitz mappings.
pub fn mordukhovich_subdiff_chain_rule_ty() -> Expr {
    prop()
}
/// `ClarkeStationaryCondition : (List Real -> Real) -> List Real -> Prop`
/// 0 ∈ ∂^C f(x): Clarke stationarity as a necessary optimality condition.
pub fn clarke_stationary_condition_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(vec_ty(), prop()))
}
/// `EkelandWithErrorBound : (List Real -> Real) -> Real -> Prop`
/// Ekeland's principle with explicit error bound for approximate minimisers.
pub fn ekeland_with_error_bound_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `BorweinPreissSmooth : (List Real -> Real) -> Prop`
/// Borwein-Preiss: for any ε > 0, there exists a smooth perturbation realising a minimiser.
pub fn borwein_preiss_smooth_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `FloweringPrinciple : Prop`
/// Stegall's "flowering" variational principle in Asplund spaces.
pub fn flowering_principle_ty() -> Expr {
    prop()
}
/// `Deville_Godefroy_Zizler : Prop`
/// Deville-Godefroy-Zizler variational principle: smooth bump function perturbation.
pub fn deville_godefroy_zizler_ty() -> Expr {
    prop()
}
/// `ProximalNormalCone : (List Real -> Prop) -> List Real -> (List Real -> Prop)`
/// N̂^P(C;x): proximal normals via squared distance to C.
pub fn proximal_normal_cone_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), prop()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `FrechetNormalCone : (List Real -> Prop) -> List Real -> (List Real -> Prop)`
/// N̂(C;x): Fréchet (regular) normal cone — subdifferential of indicator function.
pub fn frechet_normal_cone_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), prop()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `NormalConeInclusion : Prop`
/// Proximal ⊆ Fréchet ⊆ Limiting normal cone (fundamental inclusions).
pub fn normal_cone_inclusion_ty() -> Expr {
    prop()
}
/// `NormalConeCalculus : (List Real -> Prop) -> (List Real -> Prop) -> Prop`
/// Normal cone calculus: N(C₁ ∩ C₂; x) ⊆ N(C₁; x) + N(C₂; x) under regularity.
pub fn normal_cone_calculus_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), prop()),
        arrow(fn_ty(vec_ty(), prop()), prop()),
    )
}
/// `MordukhovichCoderivative : (List Real -> List Real) -> List Real -> (List Real -> List Real -> Prop)`
/// D*F(x)(v): coderivative of a set-valued map F at (x, y).
pub fn mordukhovich_coderivative_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), vec_ty()),
        arrow(vec_ty(), fn_ty(vec_ty(), fn_ty(vec_ty(), prop()))),
    )
}
/// `StrictDifferentiability : (List Real -> List Real) -> List Real -> Prop`
/// F is strictly differentiable at x: limit of (F(y)-F(z))/(y-z) as y,z→x is uniform.
pub fn strict_differentiability_ty() -> Expr {
    arrow(fn_ty(vec_ty(), vec_ty()), arrow(vec_ty(), prop()))
}
/// `ClarkeGeneralisedJacobian : (List Real -> List Real) -> List Real -> Type`
/// ∂^C F(x) = conv{lim ∇F(xₙ) | xₙ → x, xₙ ∉ Ω_F}: Clarke generalised Jacobian.
pub fn clarke_generalised_jacobian_ty() -> Expr {
    arrow(fn_ty(vec_ty(), vec_ty()), arrow(vec_ty(), type0()))
}
/// `HausdorffContinuity : (List Real -> List Real -> Prop) -> Prop`
/// A set-valued map is Hausdorff continuous if d_H(F(x), F(y)) → 0 as y → x.
pub fn hausdorff_continuity_ty() -> Expr {
    arrow(fn_ty(vec_ty(), fn_ty(vec_ty(), prop())), prop())
}
/// `BergeMaximumTheorem : (List Real -> Real) -> (List Real -> List Real -> Prop) -> Prop`
/// Berge's maximum theorem: if f is continuous and C is continuous compact-valued,
/// then V(x) = max_{y∈C(x)} f(x,y) is continuous and the argmax correspondence is USC.
pub fn berge_maximum_theorem_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(fn_ty(vec_ty(), fn_ty(vec_ty(), prop())), prop()),
    )
}
/// `MichaelSelectionTheorem : (List Real -> List Real -> Prop) -> Prop`
/// Michael's selection theorem: an LSC map with convex closed values has a continuous selection.
pub fn michael_selection_theorem_ty() -> Expr {
    arrow(fn_ty(vec_ty(), fn_ty(vec_ty(), prop())), prop())
}
/// `KuratowskiRyllNardzewski : (List Real -> List Real -> Prop) -> Prop`
/// Kuratowski-Ryll-Nardzewski measurable selection theorem.
pub fn kuratowski_ryll_nardzewski_ty() -> Expr {
    arrow(fn_ty(vec_ty(), fn_ty(vec_ty(), prop())), prop())
}
/// `ProximalPointAlgorithm : (List Real -> Real) -> Prop`
/// Proximal point algorithm: x_{k+1} = prox_{λf}(xₖ) converges to a minimiser.
pub fn proximal_point_algorithm_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `BundleMethod : (List Real -> Real) -> Prop`
/// Bundle method: collects subgradient information in a "bundle" to build cutting planes.
pub fn bundle_method_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `ProxRegularMinimisation : (List Real -> Real) -> Real -> Prop`
/// Prox-regularity ensures the proximal point algorithm converges at linear rate.
pub fn prox_regular_minimisation_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `DouglasRachfordSplitting : (List Real -> Real) -> (List Real -> Real) -> Prop`
/// Douglas-Rachford splitting for minimising f + g via proximal operators.
pub fn douglas_rachford_splitting_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), arrow(rn_r, prop()))
}
/// `UpperDiniDerivative : (List Real -> Real) -> List Real -> List Real -> Real`
/// D⁺f(x;v) = limsup_{t↓0} (f(x+tv) - f(x))/t.
pub fn upper_dini_derivative_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), arrow(vec_ty(), real_ty())),
    )
}
/// `LowerDiniDerivative : (List Real -> Real) -> List Real -> List Real -> Real`
/// D₋f(x;v) = liminf_{t↓0} (f(x+tv) - f(x))/t.
pub fn lower_dini_derivative_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), arrow(vec_ty(), real_ty())),
    )
}
/// `Quasidifferential : (List Real -> Real) -> List Real -> Type`
/// Quasidifferential ∂f(x) = (Df(x), -Df(x)): pair of convex compact sets characterising df.
pub fn quasidifferential_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(vec_ty(), type0()))
}
/// `ClarkeGeneralisedDirectionalDerivative : (List Real -> Real) -> List Real -> List Real -> Real`
/// f°(x;v) = limsup_{y→x, t↓0} (f(y+tv) - f(y))/t.
pub fn clarke_directional_derivative_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), arrow(vec_ty(), real_ty())),
    )
}
/// `GradientFlow : (List Real -> Real) -> Prop`
/// Gradient flow: ẋ ∈ -∂f(x), the steepest descent curve in the sense of maximal slope.
pub fn gradient_flow_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `BregmanDynamics : (List Real -> Real) -> (List Real -> Real) -> Prop`
/// Bregman proximal dynamics: xₖ₊₁ = argmin {f(x) + Dφ(x, xₖ)/λ}.
pub fn bregman_dynamics_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), arrow(rn_r, prop()))
}
/// `SaddlePointDynamics : (List Real -> List Real -> Real) -> Prop`
/// Primal-dual saddle point dynamics: (ẋ, ẏ) = (-∂_x L(x,y), ∂_y L(x,y)).
pub fn saddle_point_dynamics_ty() -> Expr {
    arrow(fn_ty(vec_ty(), fn_ty(vec_ty(), real_ty())), prop())
}
/// `MirrorDescentAlgorithm : (List Real -> Real) -> (List Real -> Real) -> Prop`
/// Mirror descent: generalises gradient descent to curved geometries via Bregman divergence.
pub fn mirror_descent_algorithm_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), arrow(rn_r, prop()))
}
/// `IsQuasiconvex : (List Real -> Real) -> Prop`
/// f is quasiconvex: level sets {x | f(x) ≤ α} are convex for all α.
pub fn is_quasiconvex_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `IsPseudoconvex : (List Real -> Real) -> Prop`
/// f is pseudoconvex: ⟨∇f(x), y-x⟩ ≥ 0 ⟹ f(y) ≥ f(x).
pub fn is_pseudoconvex_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `QuasiconvexLevelSets : (List Real -> Real) -> Real -> Prop`
/// Level set {x | f(x) ≤ α} is convex for each α ∈ ℝ when f is quasiconvex.
pub fn quasiconvex_level_sets_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `StrictQuasiconvexity : (List Real -> Real) -> Prop`
/// Strict quasiconvexity: f(λx + (1-λ)y) < max(f(x), f(y)) for x≠y, λ∈(0,1).
pub fn strict_quasiconvexity_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `MFCQ : (List (List Real -> Real)) -> (List (List Real -> Real)) -> List Real -> Prop`
/// Mangasarian-Fromovitz CQ: active constraint gradients + inequality multipliers positively independent.
pub fn mfcq_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    let constraint_list = list_ty(rn_r.clone());
    arrow(
        constraint_list.clone(),
        arrow(constraint_list, arrow(vec_ty(), prop())),
    )
}
/// `LICQ : (List (List Real -> Real)) -> List Real -> Prop`
/// Linear Independence CQ: gradients of active constraints are linearly independent.
pub fn licq_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(list_ty(rn_r), arrow(vec_ty(), prop()))
}
/// `SecondOrderSufficientConditions : (List Real -> Real) -> List Real -> Prop`
/// SOSC: positive definiteness of Lagrangian Hessian on the critical cone.
pub fn second_order_sufficient_conditions_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(vec_ty(), prop()))
}
/// `SecondOrderNecessaryConditions : (List Real -> Real) -> List Real -> Prop`
/// SONC: Hessian of Lagrangian is PSD on the critical cone at a local minimiser.
pub fn second_order_necessary_conditions_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(vec_ty(), prop()))
}
/// `KKTConditions : (List Real -> Real) -> (List (List Real -> Real)) -> List Real -> Prop`
/// Karush-Kuhn-Tucker necessary optimality conditions.
pub fn kkt_conditions_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), arrow(list_ty(rn_r), arrow(vec_ty(), prop())))
}
/// Build the extended variational analysis environment (adds §7 axioms to existing §1-§6).
pub fn build_variational_analysis_env_extended() -> Environment {
    let mut env = build_variational_analysis_env();
    let axioms: &[(&str, Expr)] = &[
        (
            "TopologicalEpiConvergence",
            topological_epi_convergence_ty(),
        ),
        (
            "MoscoConvergenceReflexiveBanach",
            mosco_convergence_reflexive_banach_ty(),
        ),
        ("EpiConvergenceStability", epi_convergence_stability_ty()),
        ("MetricRegularity", metric_regularity_ty()),
        ("LinearRegularity", linear_regularity_ty()),
        ("LipschitzStability", lipschitz_stability_ty()),
        ("RobinsonStability", robinson_stability_ty()),
        ("AubinPropertyCriteria", aubin_property_criteria_ty()),
        ("ClarkeSubdiffSumRule", clarke_subdiff_sum_rule_ty()),
        ("ClarkeSubdiffChainRule", clarke_subdiff_chain_rule_ty()),
        (
            "MordukhovichSubdiffSumRule",
            mordukhovich_subdiff_sum_rule_ty(),
        ),
        (
            "MordukhovichSubdiffChainRule",
            mordukhovich_subdiff_chain_rule_ty(),
        ),
        (
            "ClarkeStationaryCondition",
            clarke_stationary_condition_ty(),
        ),
        ("EkelandWithErrorBound", ekeland_with_error_bound_ty()),
        ("BorweinPreissSmooth", borwein_preiss_smooth_ty()),
        ("FloweringPrinciple", flowering_principle_ty()),
        ("DevilleGodefroyZizler", deville_godefroy_zizler_ty()),
        ("ProximalNormalCone", proximal_normal_cone_ty()),
        ("FrechetNormalCone", frechet_normal_cone_ty()),
        ("NormalConeInclusion", normal_cone_inclusion_ty()),
        ("NormalConeCalculus", normal_cone_calculus_ty()),
        ("MordukhovichCoderivative", mordukhovich_coderivative_ty()),
        ("StrictDifferentiability", strict_differentiability_ty()),
        (
            "ClarkeGeneralisedJacobian",
            clarke_generalised_jacobian_ty(),
        ),
        ("HausdorffContinuity", hausdorff_continuity_ty()),
        ("BergeMaximumTheorem", berge_maximum_theorem_ty()),
        ("MichaelSelectionTheorem", michael_selection_theorem_ty()),
        ("KuratowskiRyllNardzewski", kuratowski_ryll_nardzewski_ty()),
        ("ProximalPointAlgorithm", proximal_point_algorithm_ty()),
        ("BundleMethod", bundle_method_ty()),
        ("ProxRegularMinimisation", prox_regular_minimisation_ty()),
        ("DouglasRachfordSplitting", douglas_rachford_splitting_ty()),
        ("UpperDiniDerivative", upper_dini_derivative_ty()),
        ("LowerDiniDerivative", lower_dini_derivative_ty()),
        ("Quasidifferential", quasidifferential_ty()),
        (
            "ClarkeDirectionalDerivative",
            clarke_directional_derivative_ty(),
        ),
        ("GradientFlow", gradient_flow_ty()),
        ("BregmanDynamics", bregman_dynamics_ty()),
        ("SaddlePointDynamics", saddle_point_dynamics_ty()),
        ("MirrorDescentAlgorithm", mirror_descent_algorithm_ty()),
        ("IsQuasiconvex", is_quasiconvex_ty()),
        ("IsPseudoconvex", is_pseudoconvex_ty()),
        ("QuasiconvexLevelSets", quasiconvex_level_sets_ty()),
        ("StrictQuasiconvexity", strict_quasiconvexity_ty()),
        ("MFCQ", mfcq_ty()),
        ("LICQ", licq_ty()),
        (
            "SecondOrderSufficientConditions",
            second_order_sufficient_conditions_ty(),
        ),
        (
            "SecondOrderNecessaryConditions",
            second_order_necessary_conditions_ty(),
        ),
        ("KKTConditions", kkt_conditions_ty()),
    ];
    for &(name, ref ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_extended_env_axioms_registered() {
        let env = build_variational_analysis_env_extended();
        assert!(env.get(&Name::str("MetricRegularity")).is_some());
        assert!(env.get(&Name::str("LinearRegularity")).is_some());
        assert!(env.get(&Name::str("MichaelSelectionTheorem")).is_some());
        assert!(env.get(&Name::str("BergeMaximumTheorem")).is_some());
        assert!(env.get(&Name::str("ProximalPointAlgorithm")).is_some());
        assert!(env.get(&Name::str("DouglasRachfordSplitting")).is_some());
        assert!(env.get(&Name::str("IsQuasiconvex")).is_some());
        assert!(env.get(&Name::str("KKTConditions")).is_some());
        assert!(env.get(&Name::str("MFCQ")).is_some());
        assert!(env.get(&Name::str("LICQ")).is_some());
        assert!(env.get(&Name::str("GradientFlow")).is_some());
        assert!(env.get(&Name::str("BregmanDynamics")).is_some());
    }
    #[test]
    fn test_ekeland_principle_smooth_quadratic() {
        let f = |x: &[f64]| x[0] * x[0];
        let ep = EkelandPrinciple::new(0.5, 500);
        let x_eps = ep.find_minimiser(f, &[2.0]);
        assert!(x_eps[0].abs() < 2.0, "Ekeland minimiser: {}", x_eps[0]);
        let samples: Vec<Vec<f64>> = (0..5).map(|i| vec![i as f64 * 0.5]).collect();
        assert!(ep.verify_ekeland_condition(f, &x_eps, &samples));
    }
    #[test]
    fn test_mordukhovich_subdiff_stationary() {
        let f = |x: &[f64]| x[0] * x[0];
        let approx = MordukhovichSubdiffApprox::new(1e-5, 10, 1e-3);
        assert!(approx.is_stationary(f, &[0.0]));
        assert!(!approx.is_stationary(f, &[1.0]));
    }
    #[test]
    fn test_mordukhovich_subdiff_frechet_quadratic() {
        let f = |x: &[f64]| 0.5 * x[0] * x[0];
        let approx = MordukhovichSubdiffApprox::new(1e-5, 5, 0.01);
        let g = approx.frechet_subgradient(f, &[3.0]);
        assert!(
            (g[0] - 3.0).abs() < 1e-3,
            "gradient at 3 should be ≈3, got {}",
            g[0]
        );
    }
    #[test]
    fn test_proximal_point_solver_convergence() {
        let f = |x: &[f64]| x[0] * x[0];
        let solver = ProximalPointSolver::new(0.5, 50, 1e-4);
        let iterates = solver.solve(f, &[3.0]);
        assert!(!iterates.is_empty());
        let last = iterates.last().expect("last should succeed");
        assert!(
            last[0].abs() < 3.0,
            "PPA should move towards 0, got {}",
            last[0]
        );
    }
    #[test]
    fn test_proximal_point_solver_converged() {
        let f = |x: &[f64]| x[0] * x[0];
        let solver = ProximalPointSolver::new(1.0, 200, 1e-3);
        let iterates = solver.solve(f, &[1.0]);
        assert!(
            solver.has_converged(&iterates),
            "PPA should converge for convex quadratic"
        );
    }
    #[test]
    fn test_metric_regularity_checker_quasiconvex() {
        let f = |x: &[f64]| x[0] * x[0];
        let result = MetricRegularityChecker::check_quasiconvex(f, &[-1.0], &[1.0], 20);
        assert!(result, "x^2 should be quasiconvex");
    }
    #[test]
    fn test_metric_regularity_checker_not_quasiconvex() {
        let f = |x: &[f64]| -(x[0] * x[0]);
        let result = MetricRegularityChecker::check_quasiconvex(f, &[-1.0], &[1.0], 20);
        assert!(!result, "-(x^2) should NOT be quasiconvex");
    }
    #[test]
    fn test_metric_regularity_checker_cq() {
        let constraints: Vec<Box<dyn Fn(&[f64]) -> f64>> = vec![Box::new(|x: &[f64]| x[0])];
        let x = vec![0.0];
        let result =
            MetricRegularityChecker::check_constraint_qualification(&constraints, &x, 1e-5);
        assert!(result, "Single nonzero-gradient constraint satisfies CQ");
    }
    #[test]
    fn test_ekeland_near_minimality_gap() {
        let f = |x: &[f64]| (x[0] - 2.0).powi(2);
        let ep = EkelandPrinciple::new(0.1, 1000);
        let x_eps = ep.find_minimiser(f, &[5.0]);
        let gap = ep.near_minimality_gap(f, &x_eps, &[5.0]);
        assert!(gap >= 0.0, "gap should be non-negative: {gap}");
    }
    #[test]
    fn test_build_variational_env_original_still_present() {
        let env = build_variational_analysis_env_extended();
        assert!(env.get(&Name::str("RegularSubdifferential")).is_some());
        assert!(env.get(&Name::str("EkelandVariationalPrinciple")).is_some());
        assert!(env.get(&Name::str("MountainPassTheorem")).is_some());
        assert!(env.get(&Name::str("GammaConverges")).is_some());
    }
}
#[cfg(test)]
mod tests_variational_extended {
    use super::*;
    #[test]
    fn test_prox_l1_soft_threshold() {
        let prox = ProximalOperator::new(0.5, ProxFnType::L1Norm);
        assert!((prox.apply_scalar(1.5) - 1.0).abs() < 1e-10);
        assert!((prox.apply_scalar(0.3) - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_prox_l2_shrinkage() {
        let prox = ProximalOperator::new(1.0, ProxFnType::L2NormSquared);
        assert!((prox.apply_scalar(3.0) - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_prox_nonneg() {
        let prox = ProximalOperator::new(1.0, ProxFnType::NonNegativeOrtHant);
        assert!((prox.apply_scalar(-3.0) - 0.0).abs() < 1e-10);
        assert!((prox.apply_scalar(2.0) - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_moreau_decomposition() {
        let prox = ProximalOperator::new(1.0, ProxFnType::L1Norm);
        let v = 2.5;
        let (p, d) = prox.moreau_decomposition(v);
        assert!((p + d - v).abs() < 1e-10);
    }
    #[test]
    fn test_admm_dual_update() {
        let admm = ADMMSolver::new(1.0, 100, 1e-6);
        let y_new = admm.dual_update(0.5, 0.1);
        assert!((y_new - 0.6).abs() < 1e-10);
    }
    #[test]
    fn test_admm_stopping() {
        let admm = ADMMSolver::new(1.0, 100, 1e-6);
        assert!(admm.stopping_criteria(1e-7, 1e-7));
        assert!(!admm.stopping_criteria(1e-5, 1e-7));
    }
    #[test]
    fn test_gradient_descent_rate() {
        let f = ConvexSubdifferential::new("quadratic")
            .with_differentiability()
            .with_strong_convexity(1.0)
            .with_lipschitz(10.0);
        let rate = f
            .gradient_descent_rate()
            .expect("gradient_descent_rate should succeed");
        assert!((rate - 0.9).abs() < 1e-10);
    }
}
