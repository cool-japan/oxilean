//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ActionAngleVariables, BifurcationDiagram, BoxCounting, CantorSet, ChaoticSystem,
    DiophantineCondition, EntropyOfMap, FeigenbaumData, FeigenbaumLogisticMap, Fractal,
    FractalDimensionEstimator, HausdorffDimension, HenonMap, HopfBifurcation,
    IteratedFunctionSystem, IteratedFunctionSystemExt, KAMTorus, KochCurve, KolmogorovThm,
    LogisticMap, LorenzAttractorSimulator, LorenzSystem, LyapunovExponentEstimator,
    LyapunovStabilityData, MandelbrotSet, MixingProperty, PeriodDoublingCascade,
    PerturbedHamiltonian, PitchforkBifurcation, SaddleNodeBifurcation, SensitiveDependence,
    ShiftSpace, StabilityType, TopologicalTransitivity,
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
/// Type for a map R^n → R^n (discrete-time dynamical system).
pub fn map_ty(dim: usize) -> Expr {
    let rn = list_ty(real_ty());
    let _ = dim;
    arrow(rn.clone(), rn)
}
/// Type for a flow R^n × R → R^n (continuous-time dynamical system).
pub fn flow_ty() -> Expr {
    let rn = list_ty(real_ty());
    arrow(rn.clone(), arrow(real_ty(), rn))
}
/// ChaoticSystem: name × dim × params × equations × Lyapunov spectrum → Prop
pub fn chaotic_system_ty() -> Expr {
    prop()
}
/// LorenzSystem: (σ, ρ, β) → Prop
pub fn lorenz_system_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "sigma",
        real_ty(),
        pi(
            BinderInfo::Default,
            "rho",
            real_ty(),
            pi(BinderInfo::Default, "beta", real_ty(), prop()),
        ),
    )
}
/// HenonMap: (a, b) → Prop
pub fn henon_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "a",
        real_ty(),
        pi(BinderInfo::Default, "b", real_ty(), prop()),
    )
}
/// LogisticMap: r → Prop
pub fn logistic_map_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// MandelbrotSet: Prop (subset of C defined by z → z² + c iteration)
pub fn mandelbrot_set_ty() -> Expr {
    prop()
}
/// BifurcationDiagram: parameter space × phase space → Prop
pub fn bifurcation_diagram_ty() -> Expr {
    prop()
}
/// SaddleNodeBifurcation: parameter value at which two equilibria collide
pub fn saddle_node_bifurcation_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// PitchforkBifurcation: symmetric bifurcation (Bool = is supercritical)
pub fn pitchfork_bifurcation_ty() -> Expr {
    arrow(real_ty(), arrow(bool_ty(), prop()))
}
/// HopfBifurcation: parameter value at which equilibrium → limit cycle
pub fn hopf_bifurcation_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// PeriodDoublingCascade: Prop (Feigenbaum universal constants)
pub fn period_doubling_cascade_ty() -> Expr {
    prop()
}
/// Fractal: self-similar set with fractal dimension > topological dimension
pub fn fractal_ty() -> Expr {
    prop()
}
/// HausdorffDimension: d_H(F) = inf{d : H^d(F) = 0}
pub fn hausdorff_dimension_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// BoxCounting: N(ε) ~ ε^{-d_B}
pub fn box_counting_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// IteratedFunctionSystem: list of contractions
pub fn iterated_function_system_ty() -> Expr {
    arrow(list_ty(map_ty(0)), prop())
}
/// KochCurve: dimension ln4/ln3 ≈ 1.261...
pub fn koch_curve_ty() -> Expr {
    prop()
}
/// CantorSet: dimension ln2/ln3 ≈ 0.631...
pub fn cantor_set_ty() -> Expr {
    prop()
}
/// KAMTorus: quasi-periodic invariant torus
pub fn kam_torus_ty() -> Expr {
    prop()
}
/// ActionAngleVariables: (J, θ) for integrable system
pub fn action_angle_variables_ty() -> Expr {
    prop()
}
/// PerturbedHamiltonian: H = H_0(J) + εH_1(J,θ)
pub fn perturbed_hamiltonian_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// Diophantine condition: |k·ω| ≥ γ/|k|^n for all k≠0
pub fn diophantine_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(real_ty(), arrow(nat_ty(), prop())),
    )
}
/// KolmogorovThm: persistence of KAM tori under perturbation
pub fn kolmogorov_thm_ty() -> Expr {
    prop()
}
/// MixingProperty: μ(A ∩ T^{-n}B) → μ(A)μ(B) as n→∞
pub fn mixing_property_ty() -> Expr {
    prop()
}
/// SensitiveDependence: ∃δ: ∀x,ε, ∃y with d(x,y)<ε and d(f^n x, f^n y)>δ
pub fn sensitive_dependence_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// TopologicalTransitivity: dense orbit exists
pub fn topological_transitivity_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// DevaneyChaoticMap: three conditions (dense periodic orbits + transitivity + sensitivity)
pub fn devaney_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// EntropyOfMap: topological entropy h(f) = sup_ε lim (1/n) log N(ε,n)
pub fn entropy_of_map_ty() -> Expr {
    arrow(map_ty(0), real_ty())
}
/// Register all chaos theory axioms in the kernel environment.
pub fn build_chaos_theory_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("ChaoticSystem", chaotic_system_ty()),
        ("LorenzSystem", lorenz_system_ty()),
        ("HenonMap", henon_map_ty()),
        ("LogisticMap", logistic_map_ty()),
        ("MandelbrotSet", mandelbrot_set_ty()),
        ("LyapunovExponent", arrow(map_ty(0), real_ty())),
        ("HasStrangeAttractor", arrow(map_ty(0), prop())),
        ("IsChaotic", arrow(map_ty(0), prop())),
        ("PeriodDoublingRatio", arrow(map_ty(0), real_ty())),
        ("BifurcationDiagram", bifurcation_diagram_ty()),
        ("SaddleNodeBifurcation", saddle_node_bifurcation_ty()),
        ("PitchforkBifurcation", pitchfork_bifurcation_ty()),
        ("HopfBifurcation", hopf_bifurcation_ty()),
        ("PeriodDoublingCascade", period_doubling_cascade_ty()),
        ("FeigenbaumDelta", real_ty()),
        ("FeigenbaumAlpha", real_ty()),
        ("BifurcationValue", arrow(map_ty(0), real_ty())),
        ("NormalForm", arrow(map_ty(0), map_ty(0))),
        ("Fractal", fractal_ty()),
        ("HausdorffDimension", hausdorff_dimension_ty()),
        ("BoxCountingDimension", box_counting_ty()),
        ("IteratedFunctionSystem", iterated_function_system_ty()),
        ("KochCurve", koch_curve_ty()),
        ("CantorSet", cantor_set_ty()),
        ("IsSelfSimilar", arrow(prop(), prop())),
        ("TopologicalDimension", arrow(prop(), nat_ty())),
        ("KAMTorus", kam_torus_ty()),
        ("ActionAngleVariables", action_angle_variables_ty()),
        ("PerturbedHamiltonian", perturbed_hamiltonian_ty()),
        ("DiophantineCondition", diophantine_ty()),
        ("KolmogorovThm", kolmogorov_thm_ty()),
        ("KAMThm", prop()),
        ("ArnoldThm", prop()),
        ("MixingProperty", mixing_property_ty()),
        ("SensitiveDependence", sensitive_dependence_ty()),
        ("TopologicalTransitivity", topological_transitivity_ty()),
        ("DevaneyChaoticMap", devaney_ty()),
        ("EntropyOfMap", entropy_of_map_ty()),
        ("ErgodicMeasure", prop()),
        ("BirkhoffErgodicThm", prop()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Feigenbaum universal constant δ ≈ 4.669201609...
pub const FEIGENBAUM_DELTA: f64 = 4.669_201_609_102_991;
/// Feigenbaum universal constant α ≈ -2.502907875...
pub const FEIGENBAUM_ALPHA: f64 = -2.502_907_875_095_892;
/// Lyapunov exponent spectrum type: map → list of real numbers (one per dimension).
pub fn lyapunov_spectrum_ty() -> Expr {
    arrow(map_ty(0), list_ty(real_ty()))
}
/// SRB (Sinai-Ruelle-Bowen) measure: physical invariant measure for chaotic attractor.
pub fn srb_measure_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Stretching and folding mechanism: necessary geometric condition for chaos.
pub fn stretching_folding_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Lyapunov dimension (Kaplan-Yorke formula): d_L = j + (λ_1 + ... + λ_j) / |λ_{j+1}|.
pub fn lyapunov_dimension_ty() -> Expr {
    arrow(list_ty(real_ty()), real_ty())
}
/// Information dimension: related to Renyi entropy; dim_1 via measure-weighted formula.
pub fn information_dimension_ty() -> Expr {
    arrow(map_ty(0), real_ty())
}
/// Correlation dimension (Grassberger-Procaccia): C(r) ~ r^{d_c}.
pub fn correlation_dimension_ty() -> Expr {
    arrow(map_ty(0), real_ty())
}
/// Smale horseshoe map: archetype of uniform hyperbolicity and symbolic dynamics.
pub fn smale_horseshoe_ty() -> Expr {
    prop()
}
/// Symbolic dynamics: shift map on sequence space as topological conjugate.
pub fn symbolic_dynamics_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Topological conjugacy: h ∘ f = g ∘ h for homeomorphism h.
pub fn topological_conjugacy_ty() -> Expr {
    arrow(map_ty(0), arrow(map_ty(0), prop()))
}
/// Sub-shift of finite type: symbolic dynamics defined by a transition matrix.
pub fn subshift_finite_type_ty() -> Expr {
    arrow(list_ty(list_ty(nat_ty())), prop())
}
/// Zeta function of a dynamical system: Z(t) = exp(Σ |Fix(f^n)| t^n / n).
pub fn dynamical_zeta_fn_ty() -> Expr {
    arrow(map_ty(0), arrow(real_ty(), real_ty()))
}
/// Lorenz attractor: specific strange attractor with butterfly shape.
pub fn lorenz_attractor_ty() -> Expr {
    prop()
}
/// Lorenz flow: continuous-time flow generated by Lorenz equations.
pub fn lorenz_flow_ty() -> Expr {
    lorenz_system_ty()
}
/// Geometric Lorenz model: hyperbolic model approximating true Lorenz attractor.
pub fn geometric_lorenz_model_ty() -> Expr {
    prop()
}
/// Lorenz template: branched surface encoding topology of Lorenz attractor.
pub fn lorenz_template_ty() -> Expr {
    prop()
}
/// Homoclinic bifurcation: orbit connecting equilibrium to itself.
pub fn homoclinic_bifurcation_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Heteroclinic bifurcation: orbit connecting two distinct equilibria.
pub fn heteroclinic_bifurcation_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Global bifurcation: bifurcation involving large-scale changes in phase portrait.
pub fn global_bifurcation_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Center manifold: invariant manifold at bifurcation point.
pub fn center_manifold_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Renormalization group: self-similar structure at period-doubling accumulation.
pub fn renormalization_group_ty() -> Expr {
    prop()
}
/// Arnold diffusion: slow drift through resonant zones in higher-dimensional systems.
pub fn arnold_diffusion_ty() -> Expr {
    prop()
}
/// Cantorus: fractal remnant of destroyed KAM torus (cantori theory).
pub fn cantorus_ty() -> Expr {
    prop()
}
/// Twist map: area-preserving map with monotone twist condition.
pub fn twist_map_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Poincaré-Birkhoff theorem: twist maps have periodic fixed points.
pub fn poincare_birkhoff_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Birkhoff ergodic theorem: time average = space average almost everywhere.
pub fn birkhoff_ergodic_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        arrow(real_ty(), real_ty()),
        pi(BinderInfo::Default, "mu", prop(), prop()),
    )
}
/// Ergodic decomposition: every invariant measure decomposes into ergodic measures.
pub fn ergodic_decomposition_ty() -> Expr {
    arrow(prop(), prop())
}
/// Poincaré recurrence theorem: almost every point returns to any neighborhood.
pub fn poincare_recurrence_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Measure-theoretic entropy (Kolmogorov-Sinai entropy): h_{KS}(f) = sup_P h(f, P).
pub fn ks_entropy_ty() -> Expr {
    arrow(map_ty(0), real_ty())
}
/// Pecora-Carroll synchronization: drive-response scheme for chaos synchronization.
pub fn pecora_carroll_ty() -> Expr {
    arrow(map_ty(0), arrow(map_ty(0), prop()))
}
/// Adaptive synchronization: parameters adapt to achieve synchronization.
pub fn adaptive_synchronization_ty() -> Expr {
    arrow(map_ty(0), arrow(map_ty(0), prop()))
}
/// Generalized synchronization: functional relationship between drive/response.
pub fn generalized_synchronization_ty() -> Expr {
    arrow(map_ty(0), arrow(map_ty(0), prop()))
}
/// Phase synchronization: phases lock while amplitudes remain chaotic.
pub fn phase_synchronization_ty() -> Expr {
    arrow(map_ty(0), arrow(map_ty(0), prop()))
}
/// Gutzwiller trace formula: semiclassical approximation via periodic orbits.
pub fn gutzwiller_trace_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Energy level statistics: GOE/GUE/GSE statistics for chaotic quantum systems.
pub fn energy_level_statistics_ty() -> Expr {
    prop()
}
/// Quantum ergodicity: eigenfunctions equidistribute on the classical energy surface.
pub fn quantum_ergodicity_ty() -> Expr {
    prop()
}
/// Berry-Tabor conjecture: integrable systems have Poisson level statistics.
pub fn berry_tabor_ty() -> Expr {
    prop()
}
/// Bohigas-Giannoni-Schmit conjecture: chaotic systems have RMT statistics.
pub fn bgs_conjecture_ty() -> Expr {
    prop()
}
/// Self-affine set: set invariant under affine (not necessarily similar) maps.
pub fn self_affine_ty() -> Expr {
    arrow(list_ty(map_ty(0)), prop())
}
/// Multifractal spectrum: f(α) spectrum for varying local Hölder exponents.
pub fn multifractal_spectrum_ty() -> Expr {
    arrow(real_ty(), real_ty())
}
/// Julia set: boundary of Fatou domain for complex polynomial iteration.
pub fn julia_set_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// Sierpinski gasket: fractal with dimension ln3/ln2 ≈ 1.585.
pub fn sierpinski_gasket_ty() -> Expr {
    prop()
}
/// Menger sponge: 3D fractal with dimension ln20/ln3 ≈ 2.727.
pub fn menger_sponge_ty() -> Expr {
    prop()
}
/// Auslander-Yorke chaos: sensitivity in transitive systems.
pub fn auslander_yorke_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Li-Yorke chaos: period 3 implies chaos (Li-Yorke theorem).
pub fn li_yorke_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Sharkovskii ordering: period-3 implies all periods.
pub fn sharkovskii_ty() -> Expr {
    arrow(map_ty(0), arrow(nat_ty(), prop()))
}
/// Topological horseshoe: combinatorial criterion implying positive entropy.
pub fn topological_horseshoe_ty() -> Expr {
    arrow(map_ty(0), prop())
}
/// Register the extended set of chaos theory axioms (new §6–§15 axioms).
pub fn build_chaos_theory_env_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("LyapunovSpectrum", lyapunov_spectrum_ty()),
        ("SRBMeasure", srb_measure_ty()),
        ("StretchingFolding", stretching_folding_ty()),
        ("LyapunovDimension", lyapunov_dimension_ty()),
        ("InformationDimension", information_dimension_ty()),
        ("CorrelationDimension", correlation_dimension_ty()),
        ("SmaleHorseshoe", smale_horseshoe_ty()),
        ("SymbolicDynamics", symbolic_dynamics_ty()),
        ("TopologicalConjugacy", topological_conjugacy_ty()),
        ("SubshiftFiniteType", subshift_finite_type_ty()),
        ("DynamicalZetaFn", dynamical_zeta_fn_ty()),
        ("LorenzAttractor", lorenz_attractor_ty()),
        ("LorenzFlow", lorenz_flow_ty()),
        ("GeometricLorenzModel", geometric_lorenz_model_ty()),
        ("LorenzTemplate", lorenz_template_ty()),
        ("HomoclinicBifurcation", homoclinic_bifurcation_ty()),
        ("HeteroclinicBifurcation", heteroclinic_bifurcation_ty()),
        ("GlobalBifurcation", global_bifurcation_ty()),
        ("CenterManifold", center_manifold_ty()),
        ("RenormalizationGroup", renormalization_group_ty()),
        ("ArnoldDiffusion", arnold_diffusion_ty()),
        ("Cantorus", cantorus_ty()),
        ("TwistMap", twist_map_ty()),
        ("PoincareBirkhoff", poincare_birkhoff_ty()),
        ("BirkhoffErgodicThm2", birkhoff_ergodic_ty()),
        ("ErgodicDecomposition", ergodic_decomposition_ty()),
        ("PoincareRecurrence", poincare_recurrence_ty()),
        ("KSEntropy", ks_entropy_ty()),
        ("PecoraCarroll", pecora_carroll_ty()),
        ("AdaptiveSynchronization", adaptive_synchronization_ty()),
        (
            "GeneralizedSynchronization",
            generalized_synchronization_ty(),
        ),
        ("PhaseSynchronization", phase_synchronization_ty()),
        ("GutzwillerTrace", gutzwiller_trace_ty()),
        ("EnergyLevelStatistics", energy_level_statistics_ty()),
        ("QuantumErgodicity", quantum_ergodicity_ty()),
        ("BerryTabor", berry_tabor_ty()),
        ("BGSConjecture", bgs_conjecture_ty()),
        ("SelfAffineSet", self_affine_ty()),
        ("MultifractalSpectrum", multifractal_spectrum_ty()),
        ("JuliaSet", julia_set_ty()),
        ("SierpinskiGasket", sierpinski_gasket_ty()),
        ("MengerSponge", menger_sponge_ty()),
        ("AuslanderYorke", auslander_yorke_ty()),
        ("LiYorke", li_yorke_ty()),
        ("Sharkovskii", sharkovskii_ty()),
        ("TopologicalHorseshoe", topological_horseshoe_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lyapunov_estimator_logistic_chaotic() {
        let est = LyapunovExponentEstimator::new();
        let le = est.estimate_logistic(4.0);
        assert!(le > 0.5, "Expected positive LE for r=4, got {le}");
    }
    #[test]
    fn test_lyapunov_estimator_logistic_stable() {
        let est = LyapunovExponentEstimator::new();
        let le = est.estimate_logistic(2.0);
        assert!(le < 0.0, "Expected negative LE for r=2, got {le}");
    }
    #[test]
    fn test_lorenz_simulator_rk4() {
        let sim = LorenzAttractorSimulator::classic(0.01);
        let traj = sim.simulate(1.0, 0.0, 0.0, 100);
        assert_eq!(traj.len(), 101);
        let (x0, y0, z0) = traj[0];
        let (xn, yn, zn) = traj[100];
        let dist = ((xn - x0).powi(2) + (yn - y0).powi(2) + (zn - z0).powi(2)).sqrt();
        assert!(dist > 0.01, "Trajectory should move: dist={dist}");
    }
    #[test]
    fn test_lorenz_simulator_le() {
        let sim = LorenzAttractorSimulator::classic(0.01);
        let le = sim.lyapunov_exponent(5000);
        assert!(le > 0.0, "Classic Lorenz should have positive LE, got {le}");
    }
    #[test]
    fn test_feigenbaum_logistic_map() {
        let fg = FeigenbaumLogisticMap::new();
        let deltas = fg.estimated_delta();
        assert!(!deltas.is_empty());
        let last = *deltas.last().expect("last should succeed");
        assert!(
            (last - FEIGENBAUM_DELTA).abs() < 1.0,
            "Estimated δ={last} far from Feigenbaum {FEIGENBAUM_DELTA}"
        );
        assert!(fg.is_converging_to_feigenbaum());
    }
    #[test]
    fn test_feigenbaum_period_detection() {
        let fg = FeigenbaumLogisticMap::new();
        let p = fg.detect_period(3.2, 0.5);
        assert_eq!(p, 2, "r=3.2 should give period 2, got {p}");
        let p1 = fg.detect_period(2.5, 0.5);
        assert_eq!(p1, 1, "r=2.5 should give period 1, got {p1}");
    }
    #[test]
    fn test_fractal_dimension_henon() {
        let est = FractalDimensionEstimator::from_henon_attractor(5000);
        let dim = est.estimate_dimension();
        assert!(dim.is_some(), "Should get a dimension estimate");
        let d = dim.expect("dim should be valid");
        assert!(
            d > 1.0 && d < 2.0,
            "Hénon attractor dimension should be in (1,2), got {d}"
        );
    }
    #[test]
    fn test_fractal_dimension_is_fractal() {
        let est = FractalDimensionEstimator::from_henon_attractor(3000);
        assert!(
            est.is_fractal(),
            "Hénon attractor should be detected as fractal"
        );
    }
    #[test]
    fn test_build_chaos_theory_env() {
        let mut env = Environment::new();
        build_chaos_theory_env(&mut env).expect("build_chaos_theory_env should succeed");
        build_chaos_theory_env_extended(&mut env)
            .expect("build_chaos_theory_env_extended should succeed");
        assert!(env.get(&Name::str("SRBMeasure")).is_some());
        assert!(env.get(&Name::str("SmaleHorseshoe")).is_some());
        assert!(env.get(&Name::str("LorenzAttractor")).is_some());
        assert!(env.get(&Name::str("HomoclinicBifurcation")).is_some());
        assert!(env.get(&Name::str("ArnoldDiffusion")).is_some());
        assert!(env.get(&Name::str("PecoraCarroll")).is_some());
        assert!(env.get(&Name::str("GutzwillerTrace")).is_some());
        assert!(env.get(&Name::str("MultifractalSpectrum")).is_some());
        assert!(env.get(&Name::str("LiYorke")).is_some());
        assert!(env.get(&Name::str("KSEntropy")).is_some());
    }
}
#[cfg(test)]
mod tests_chaos_ext {
    use super::*;
    #[test]
    fn test_lyapunov_stability_asymp_stable() {
        let ls = LyapunovStabilityData::new("origin", vec![-1.0, -2.0]);
        assert_eq!(ls.stability, StabilityType::AsymptoticallyStable);
        assert!(ls.is_stable());
        assert!(ls.hartman_grobman().contains("linearization"));
    }
    #[test]
    fn test_lyapunov_stability_unstable() {
        let ls = LyapunovStabilityData::new("saddle", vec![-1.0, 0.5]);
        assert_eq!(ls.stability, StabilityType::Unstable);
        assert!(!ls.is_stable());
    }
    #[test]
    fn test_cantor_set_hausdorff_dim() {
        let cantor = IteratedFunctionSystemExt::cantor_set();
        let expected = 2.0f64.ln() / 3.0f64.ln();
        assert!(
            (cantor.hausdorff_dim - expected).abs() < 0.001,
            "Cantor dim = {:.4}, expected {:.4}",
            cantor.hausdorff_dim,
            expected
        );
    }
    #[test]
    fn test_sierpinski_hausdorff_dim() {
        let sier = IteratedFunctionSystemExt::sierpinski();
        let expected = 3.0f64.ln() / 2.0f64.ln();
        assert!(
            (sier.hausdorff_dim - expected).abs() < 0.001,
            "Sierpinski dim = {:.4}, expected {:.4}",
            sier.hausdorff_dim,
            expected
        );
    }
    #[test]
    fn test_feigenbaum() {
        let mut f = FeigenbaumData::new();
        assert!((f.delta - 4.6692).abs() < 0.001);
        f.add_bifurcation(3.0);
        f.add_bifurcation(3.449);
        f.add_bifurcation(3.5441);
        let ratio = f.check_scaling().expect("check_scaling should succeed");
        assert!(ratio > 2.0 && ratio < 10.0, "Ratio = {ratio}");
        assert!(f.universality_description().contains("Feigenbaum"));
    }
}
#[cfg(test)]
mod tests_chaos_ext2 {
    use super::*;
    #[test]
    fn test_shift_space_full() {
        let fs = ShiftSpace::full_shift(3);
        assert_eq!(fs.alphabet_size, 3);
        let ent = fs.topological_entropy_approx();
        assert!((ent - 3.0f64.ln()).abs() < 1e-10);
        assert!(fs.is_topologically_mixing_approx());
    }
    #[test]
    fn test_golden_mean_shift() {
        let mat = vec![vec![true, true], vec![true, false]];
        let gm = ShiftSpace::from_transition_matrix(mat);
        let ent = gm.topological_entropy_approx();
        assert!((ent - 2.0f64.ln()).abs() < 1e-10);
        assert!(ShiftSpace::golden_mean_description().contains("golden mean"));
    }
}
