//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ControlSystem, Functional, IsoperimetricProblem, SobolevSpace, Symmetry,
    WassersteinGradientFlow,
};

/// Statement of the fundamental lemma of variational calculus (du Bois-Reymond lemma).
pub fn fundamental_lemma_of_variational_calculus() -> &'static str {
    "If f : [a,b] -> R is continuous and integral_{a}^{b} f(x) eta(x) dx = 0 \
     for all smooth eta with eta(a) = eta(b) = 0, then f = 0 on [a,b]."
}
/// Statement of Hamilton's principle (principle of stationary action).
pub fn hamiltons_principle_statement() -> &'static str {
    "The actual trajectory of a mechanical system between times t0 and t1 is the one \
     for which the action S[q] = integral_{t0}^{t1} L(q, q-dot, t) dt is stationary, \
     i.e., delta S = 0 for all variations vanishing at the endpoints."
}
/// Statement of Noether's theorem.
pub fn noether_theorem_statement() -> &'static str {
    "Every differentiable symmetry of the action of a physical system with conservative forces \
     corresponds to a conservation law. Specifically, if the action is invariant under a \
     one-parameter family of transformations, there exists a corresponding conserved current \
     whose charge is constant along solutions of the equations of motion."
}
/// Canonical symmetry–conservation-law correspondences.
///
/// Returns pairs of `(symmetry, conserved_quantity)`.
pub fn conservation_laws() -> Vec<(&'static str, &'static str)> {
    vec![
        ("time translation invariance", "conservation of energy"),
        (
            "spatial translation invariance",
            "conservation of linear momentum",
        ),
        ("rotational invariance", "conservation of angular momentum"),
        (
            "Lorentz boost invariance",
            "conservation of centre-of-mass motion",
        ),
        ("U(1) gauge invariance", "conservation of electric charge"),
        ("SU(3) colour symmetry", "conservation of colour charge"),
    ]
}
/// Statement of the direct method in calculus of variations.
pub fn direct_method_in_calculus_of_variations() -> &'static str {
    "If a functional F : X -> R union {+inf} on a reflexive Banach space X is \
     coercive (F(u) -> +inf as ||u|| -> inf) and weakly lower semicontinuous, \
     then F attains its infimum on X. The infimum is achieved by taking any \
     minimizing sequence, extracting a weakly convergent subsequence, and using \
     weak lower semicontinuity."
}
/// Statement of Pontryagin's maximum principle.
pub fn pontryagin_maximum_principle() -> &'static str {
    "Let (x*, u*) be an optimal state-control pair for the problem \
     min_u integral_{t0}^{t1} L(x,u,t) dt subject to dx/dt = f(x,u,t). \
     Then there exists an adjoint variable p(t) such that: \
     (1) dp/dt = -∂H/∂x along (x*, u*, p); \
     (2) u*(t) minimizes H(x*(t), u, p(t), t) over all admissible u at each t; \
     (3) H(x*(t), u*(t), p(t), t) is constant in t when L, f are time-independent."
}
/// Compactness theorem for Gamma-convergence.
pub fn gamma_convergence_compactness_theorem() -> &'static str {
    "Every sequence of functionals F_n : X -> R union {+inf} on a separable metric space X \
     has a Gamma-convergent subsequence (Gamma-compactness). Moreover, if F_n Gamma-converges \
     to F and x_n are quasi-minimizers of F_n with x_n -> x, then x minimizes F and \
     F_n(x_n) -> F(x) (fundamental theorem of Gamma-convergence)."
}
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
pub fn pi_ty(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi_ty(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    pi_ty(bi, name, dom, body)
}
pub fn int_ty() -> Expr {
    cst("Int")
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
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
/// `EulerLagrangeOp : (Real → Real → Real → Real) → (Real → Real) → Prop`
/// EulerLagrangeOp L q ↦ d/dt(∂L/∂q̇) − ∂L/∂q = 0.
pub fn euler_lagrange_op_ty() -> Expr {
    arrow(
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `SecondVariationPositive : Functional → Extremal → Prop`
/// The second variation δ²F[q₀; ·] is a positive quadratic form.
pub fn second_variation_positive_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("Extremal"), prop()))
}
/// `JacobiCondition : Extremal → Bool`
/// Returns true if there are no conjugate points on the open interval (a, b).
pub fn jacobi_condition_ty() -> Expr {
    arrow(cst("Extremal"), bool_ty())
}
/// `ConjugatePoint : Extremal → Real → Prop`
/// t̄ ∈ (a, b) is conjugate to a if the Jacobi equation has a non-trivial zero.
pub fn conjugate_point_ty() -> Expr {
    arrow(cst("Extremal"), arrow(real_ty(), prop()))
}
/// `LegendreCondition : Lagrangian → Extremal → Prop`
/// Necessary condition: L_{ẋẋ} ≥ 0 along the extremal.
pub fn legendre_condition_ty() -> Expr {
    arrow(cst("Lagrangian"), arrow(cst("Extremal"), prop()))
}
/// `StrongLegendreCondition : Lagrangian → Extremal → Prop`
/// Sufficient condition: L_{ẋẋ} > 0 (strict) along the extremal.
pub fn strong_legendre_condition_ty() -> Expr {
    arrow(cst("Lagrangian"), arrow(cst("Extremal"), prop()))
}
/// `WeierstrassCondition : Lagrangian → Extremal → Real → Prop`
/// Weierstrass excess function E(x, y, p, q) ≥ 0 for strong local minima.
pub fn weierstrass_condition_ty() -> Expr {
    arrow(
        cst("Lagrangian"),
        arrow(cst("Extremal"), arrow(real_ty(), prop())),
    )
}
/// `WeakLocalMinimum : Functional → Extremal → Prop`
/// q₀ is a weak local minimum of F: F(q₀) ≤ F(q) for q close in C¹.
pub fn weak_local_minimum_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("Extremal"), prop()))
}
/// `StrongLocalMinimum : Functional → Extremal → Prop`
/// q₀ is a strong local minimum: F(q₀) ≤ F(q) for q close in C⁰.
pub fn strong_local_minimum_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("Extremal"), prop()))
}
/// `IsoperimetricConstraint : (Real → Real) → Real → Prop`
/// ∫_a^b g(x, y, y') dx = C (integral side condition).
pub fn isoperimetric_constraint_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `LagrangeMultiplierInfiniteDim : Functional → Functional → Real → Prop`
/// F'[u] = λ G'[u] at a constrained critical point.
pub fn lagrange_multiplier_inf_dim_ty() -> Expr {
    arrow(
        cst("Functional"),
        arrow(cst("Functional"), arrow(real_ty(), prop())),
    )
}
/// `IsoperimetricSolution : IsoperimetricProblem → (Real → Real) → Prop`
/// The solution curve y* satisfies the augmented E-L equation with multiplier λ.
pub fn isoperimetric_solution_ty() -> Expr {
    arrow(
        cst("IsoperimetricProblem"),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `Costate : ControlSystem → (Real → Real) → Prop`
/// The adjoint variable p satisfying dp/dt = −∂H/∂x.
pub fn costate_ty() -> Expr {
    arrow(
        cst("ControlSystem"),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `PontryaginHamiltonian : ControlSystem → Real → Real → Real → Real`
/// H(x, u, p, t) = L(x, u, t) + p · f(x, u, t).
pub fn pontryagin_hamiltonian_ty() -> Expr {
    arrow(
        cst("ControlSystem"),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
    )
}
/// `OptimalControl : ControlSystem → (Real → Real) → Prop`
/// u*(t) minimises the Pontryagin Hamiltonian pointwise.
pub fn optimal_control_ty() -> Expr {
    arrow(
        cst("ControlSystem"),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `TransversalityCondition : ControlSystem → Real → Prop`
/// Boundary condition on the costate at free endpoints.
pub fn transversality_condition_ty() -> Expr {
    arrow(cst("ControlSystem"), arrow(real_ty(), prop()))
}
/// `HamiltonJacobiEquation : (Real → Real → Real) → Prop`
/// −∂V/∂t + H(x, ∂V/∂x, t) = 0 (PDE for the value function).
pub fn hamilton_jacobi_equation_ty() -> Expr {
    arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop())
}
/// `HJBValueFunction : ControlSystem → (Real → Real → Real) → Prop`
/// V(x, t) satisfies the HJB equation.
pub fn hjb_value_function_ty() -> Expr {
    arrow(
        cst("ControlSystem"),
        arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop()),
    )
}
/// `VerificationTheorem : ControlSystem → Prop`
/// A smooth HJB solution is the optimal value function.
pub fn verification_theorem_ty() -> Expr {
    arrow(cst("ControlSystem"), prop())
}
/// `CharacteristicMethod : (Real → Real → Real) → Prop`
/// Hamilton-Jacobi solved via method of characteristics: Hamiltonian ODE system.
pub fn characteristic_method_ty() -> Expr {
    arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop())
}
/// `GeodesicVariationalProblem : Metric → (Real → Real) → Prop`
/// Geodesic as a curve minimising ∫ √(g_{ij} ẋⁱ ẋʲ) dt.
pub fn geodesic_variational_problem_ty() -> Expr {
    arrow(cst("Metric"), arrow(arrow(real_ty(), real_ty()), prop()))
}
/// `GeodesicCompleteness : Metric → Prop`
/// Hopf-Rinow: a complete Riemannian manifold has geodesics between any two points.
pub fn geodesic_completeness_ty() -> Expr {
    arrow(cst("Metric"), prop())
}
/// `PlateauProblem : Curve → (Real → Real → Real) → Prop`
/// Existence of a minimal surface (zero mean curvature) spanning a given Jordan curve.
pub fn plateau_problem_ty() -> Expr {
    arrow(
        cst("Curve"),
        arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop()),
    )
}
/// `PlateauSolutionDouglas : Curve → Prop`
/// Douglas-Rado theorem: every rectifiable Jordan curve bounds a minimal disk.
pub fn plateau_solution_douglas_ty() -> Expr {
    arrow(cst("Curve"), prop())
}
/// `BernsteinTheorem : (Real → Real → Real) → Prop`
/// Bernstein: an entire minimal graph over R² must be a plane.
pub fn bernstein_theorem_ty() -> Expr {
    arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop())
}
/// `WillimoreFunctional : Surface → Real`
/// W(Σ) = ∫_Σ H² dA where H is mean curvature.
pub fn willmore_functional_ty() -> Expr {
    arrow(cst("Surface"), real_ty())
}
/// `WillmoreInequality : Surface → Prop`
/// W(Σ) ≥ 4π, with equality iff Σ is a round sphere.
pub fn willmore_inequality_ty() -> Expr {
    arrow(cst("Surface"), prop())
}
/// `MeanCurvatureFlow : Surface → Real → Surface`
/// Normal velocity = mean curvature H; short-time existence guaranteed.
pub fn mean_curvature_flow_ty() -> Expr {
    arrow(cst("Surface"), arrow(real_ty(), cst("Surface")))
}
/// `QuasiconvexEnvelope : (Real → Real) → (Real → Real)`
/// QW(F) = inf { ∫ W(F + ∇φ) : φ ∈ W^{1,∞}_0 }.
pub fn quasiconvex_envelope_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), real_ty()))
}
/// `RelaxedFunctional : Functional → Functional`
/// sc⁻F = lower-semicontinuous relaxation of F in a weak topology.
pub fn relaxed_functional_ty() -> Expr {
    arrow(cst("Functional"), cst("Functional"))
}
/// `GammaConvergence : (Nat → Functional) → Functional → Prop`
/// F_n Γ-converges to F: lsc condition + recovery sequences.
pub fn gamma_convergence_ty() -> Expr {
    arrow(
        arrow(nat_ty(), cst("Functional")),
        arrow(cst("Functional"), prop()),
    )
}
/// `GammaLimitUnique : (Nat → Functional) → Prop`
/// The Γ-limit is unique when it exists.
pub fn gamma_limit_unique_ty() -> Expr {
    arrow(arrow(nat_ty(), cst("Functional")), prop())
}
/// `FundamentalTheoremGammaConvergence : (Nat → Functional) → Functional → Prop`
/// If F_n → F in Γ and x_n quasi-minimise F_n, then every cluster point minimises F.
pub fn fundamental_theorem_gamma_convergence_ty() -> Expr {
    arrow(
        arrow(nat_ty(), cst("Functional")),
        arrow(cst("Functional"), prop()),
    )
}
/// `EpsilonTransitionLayer : Real → Functional`
/// Modica-Mortola functional F_ε approximating the perimeter functional.
pub fn epsilon_transition_layer_ty() -> Expr {
    arrow(real_ty(), cst("Functional"))
}
/// `WassersteinDistance : (Real → Real) → (Real → Real) → Real`
/// W_p(μ, ν) = (inf_π ∫ |x−y|^p dπ)^{1/p}.
pub fn wasserstein_distance_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), real_ty()),
    )
}
/// `OptimalTransportMap : (Real → Real) → (Real → Real) → (Real → Real) → Prop`
/// T# μ = ν and T minimises ∫ c(x, T(x)) dμ.
pub fn optimal_transport_map_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
    )
}
/// `BrenierTheorem : (Real → Real) → (Real → Real) → Prop`
/// Brenier: the optimal L² transport map is the gradient of a convex function.
pub fn brenier_theorem_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `MongeAmpereEquation : (Real → Real) → (Real → Real) → Prop`
/// det(D²u) = f/g ∘ ∇u (Monge-Ampère PDE for optimal transport).
pub fn monge_ampere_equation_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), prop()),
    )
}
/// `KantorovichDuality : (Real → Real) → (Real → Real) → Real → Prop`
/// W_p^p(μ,ν) = sup { ∫ φ dμ + ∫ ψ dν : φ(x)+ψ(y) ≤ c(x,y) }.
pub fn kantorovich_duality_ty() -> Expr {
    arrow(
        arrow(real_ty(), real_ty()),
        arrow(arrow(real_ty(), real_ty()), arrow(real_ty(), prop())),
    )
}
/// `WassersteinGradientFlow : Functional → (Real → (Real → Real)) → Prop`
/// ∂_t ρ = ∇ · (ρ ∇(δF/δρ)) — gradient flow of F in (P(R^n), W_2).
pub fn wasserstein_gradient_flow_ty() -> Expr {
    arrow(
        cst("Functional"),
        arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop()),
    )
}
/// `JKOScheme : Functional → Real → (Nat → (Real → Real)) → Prop`
/// Jordan-Kinderlehrer-Otto minimising movement scheme:
/// ρ^{n+1} = argmin { F(ρ) + W_2²(ρ^n, ρ)/(2τ) }.
pub fn jko_scheme_ty() -> Expr {
    arrow(
        cst("Functional"),
        arrow(
            real_ty(),
            arrow(arrow(nat_ty(), arrow(real_ty(), real_ty())), prop()),
        ),
    )
}
/// `ContinuityEquation : (Real → (Real → Real)) → Prop`
/// ∂_t ρ + ∇ · (ρ v) = 0 — conservation of mass.
pub fn continuity_equation_ty() -> Expr {
    arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop())
}
/// Populate an `Environment` with all variational-calculus kernel axioms.
pub fn build_variational_calculus_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("EulerLagrangeOp", euler_lagrange_op_ty()),
        ("SecondVariationPositive", second_variation_positive_ty()),
        ("JacobiCondition", jacobi_condition_ty()),
        ("ConjugatePoint", conjugate_point_ty()),
        ("LegendreCondition", legendre_condition_ty()),
        ("StrongLegendreCondition", strong_legendre_condition_ty()),
        ("WeierstrassCondition", weierstrass_condition_ty()),
        ("WeakLocalMinimum", weak_local_minimum_ty()),
        ("StrongLocalMinimum", strong_local_minimum_ty()),
        ("IsoperimetricConstraint", isoperimetric_constraint_ty()),
        ("LagrangeMultiplierInfDim", lagrange_multiplier_inf_dim_ty()),
        ("IsoperimetricSolution", isoperimetric_solution_ty()),
        ("Costate", costate_ty()),
        ("PontryaginHamiltonian", pontryagin_hamiltonian_ty()),
        ("OptimalControl", optimal_control_ty()),
        ("TransversalityCondition", transversality_condition_ty()),
        ("HamiltonJacobiEquation", hamilton_jacobi_equation_ty()),
        ("HJBValueFunction", hjb_value_function_ty()),
        ("VerificationTheorem", verification_theorem_ty()),
        ("CharacteristicMethod", characteristic_method_ty()),
        (
            "GeodesicVariationalProblem",
            geodesic_variational_problem_ty(),
        ),
        ("GeodesicCompleteness", geodesic_completeness_ty()),
        ("PlateauProblem", plateau_problem_ty()),
        ("PlateauSolutionDouglas", plateau_solution_douglas_ty()),
        ("BernsteinTheorem", bernstein_theorem_ty()),
        ("WillimoreFunctional", willmore_functional_ty()),
        ("WillmoreInequality", willmore_inequality_ty()),
        ("MeanCurvatureFlow", mean_curvature_flow_ty()),
        ("QuasiconvexEnvelope", quasiconvex_envelope_ty()),
        ("RelaxedFunctional", relaxed_functional_ty()),
        ("GammaConvergence", gamma_convergence_ty()),
        ("GammaLimitUnique", gamma_limit_unique_ty()),
        (
            "FundamentalTheoremGammaConvergence",
            fundamental_theorem_gamma_convergence_ty(),
        ),
        ("EpsilonTransitionLayer", epsilon_transition_layer_ty()),
        ("WassersteinDistance", wasserstein_distance_ty()),
        ("OptimalTransportMap", optimal_transport_map_ty()),
        ("BrenierTheorem", brenier_theorem_ty()),
        ("MongeAmpereEquation", monge_ampere_equation_ty()),
        ("KantorovichDuality", kantorovich_duality_ty()),
        ("WassersteinGradientFlow", wasserstein_gradient_flow_ty()),
        ("JKOScheme", jko_scheme_ty()),
        ("ContinuityEquation", continuity_equation_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add '{}': {:?}", name, e))?;
    }
    Ok(())
}
/// Weierstrass excess function characterisation.
pub fn weierstrass_excess_statement() -> &'static str {
    "An extremal q* provides a strong local minimum of J[q] = ∫ L(q, q', t) dt \
     if and only if the Weierstrass excess function \
     E(t, q, p, p*) = L(t,q,p) - L(t,q,p*) - (p-p*) L_{p*}(t,q,p*) ≥ 0 \
     for all admissible (t, q) and all slopes p."
}
/// Pontryagin maximum principle (full statement).
pub fn pontryagin_maximum_principle_full() -> &'static str {
    "Let (x*, u*) be an optimal pair for min_u ∫ L(x,u,t) dt s.t. ẋ = f(x,u,t). \
     Then there exists an adjoint p(t) and H(x,u,p,t) = L + p·f such that: \
     (1) ṗ = -∂H/∂x,  (2) H(x*(t), u*(t), p(t), t) = min_u H(x*(t), u, p(t), t), \
     (3) H is constant if L, f do not depend on t,  (4) transversality conditions hold."
}
/// Brenier's theorem statement.
pub fn brenier_theorem_statement() -> &'static str {
    "Let μ and ν be probability measures on R^n, with μ absolutely continuous. \
     Then the optimal L²-transport map T: R^n → R^n (minimising ∫|x-Tx|² dμ) \
     exists uniquely, is the μ-a.e. gradient of a convex function φ: T = ∇φ, \
     and φ satisfies the Monge-Ampère equation det(D²φ) = dμ/dν(∇φ)."
}
/// JKO (Jordan-Kinderlehrer-Otto) theorem.
pub fn jko_theorem_statement() -> &'static str {
    "Let F: P_2(R^n) → R be a λ-geodesically convex functional on the space of \
     probability measures with finite second moment, equipped with the W_2 metric. \
     Then the JKO minimising movement scheme ρ^{k+1} = argmin_ρ { F(ρ) + W_2²(ρ^k,ρ)/(2τ) } \
     converges as τ → 0 to the gradient flow ∂_t ρ = div(ρ ∇(δF/δρ))."
}
/// `EulerLagrangePDE : (Real^n → Real) → Prop`
/// Multi-variable E-L equation: Σ_i ∂/∂x_i (∂L/∂(∂u/∂x_i)) − ∂L/∂u = 0.
pub fn vc_ext_euler_lagrange_pde_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), prop())
}
/// `DirichletPrinciple : SobolevSpace → Functional → Prop`
/// The minimiser of the Dirichlet energy ∫ |∇u|² dx is the harmonic function
/// satisfying Δu = 0 with the given boundary data.
pub fn vc_ext_dirichlet_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "W",
        cst("SobolevSpace"),
        arrow(cst("Functional"), prop()),
    )
}
/// `DirichletEnergy : (Real → Real) → Real`
/// E[u] = (1/2) ∫_Ω |∇u|² dx.
pub fn vc_ext_dirichlet_energy_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), real_ty())
}
/// `HarmonicMap : Manifold → Manifold → (Real → Real) → Prop`
/// u : M → N is harmonic if it is a critical point of the Dirichlet energy.
pub fn vc_ext_harmonic_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("Manifold"),
        pi(
            BinderInfo::Default,
            "N",
            cst("Manifold"),
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
    )
}
/// `BiharmonicMap : Manifold → Manifold → (Real → Real) → Prop`
/// u is biharmonic: Δ²u = 0 (critical point of ∫ |Δu|² dx).
pub fn vc_ext_biharmonic_map_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("Manifold"),
        pi(
            BinderInfo::Default,
            "N",
            cst("Manifold"),
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
    )
}
/// `NoetherCurrentPDE : Lagrangian → Symmetry → (Real → Real) → Prop`
/// In field theory: J^μ = (∂L/∂(∂_μφ)) δφ is conserved (∂_μ J^μ = 0)
/// for every continuous symmetry of L.
pub fn vc_ext_noether_current_pde_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("Lagrangian"),
        pi(
            BinderInfo::Default,
            "sym",
            cst("Symmetry"),
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
    )
}
/// `ConservationLawPDE : (Real → Real) → Prop`
/// ∂_t ρ + ∇ · J = 0 — a conservation law in PDE form.
pub fn vc_ext_conservation_law_pde_ty() -> Expr {
    arrow(arrow(real_ty(), real_ty()), prop())
}
/// `EnergyMomentumTensor : Lagrangian → (Real → Real → Real) → Prop`
/// T^{μν} = (∂L/∂(∂_μφ)) ∂^ν φ − g^{μν} L is the canonical energy-momentum tensor.
pub fn vc_ext_energy_momentum_tensor_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        cst("Lagrangian"),
        arrow(arrow(real_ty(), arrow(real_ty(), real_ty())), prop()),
    )
}
/// `YangMillsFunctional : Connection → Real`
/// YM(A) = (1/2) ∫ |F_A|² dx where F_A = dA + A ∧ A is the curvature.
pub fn vc_ext_yang_mills_functional_ty() -> Expr {
    arrow(cst("Connection"), real_ty())
}
/// `YangMillsEquation : Connection → Prop`
/// The Yang-Mills equation: D_A * F_A = 0 (critical point of YM functional).
pub fn vc_ext_yang_mills_equation_ty() -> Expr {
    arrow(cst("Connection"), prop())
}
/// `AntiSelfDualConnection : Connection → Prop`
/// ASD condition: *F_A = −F_A (absolute minimisers of Yang-Mills on 4-manifolds).
pub fn vc_ext_anti_self_dual_connection_ty() -> Expr {
    arrow(cst("Connection"), prop())
}
/// `DonaldsonInvariant : Manifold → Int → Nat → Int`
/// Donaldson invariants D_k(M, L) counting ASD instantons on a 4-manifold.
pub fn vc_ext_donaldson_invariant_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("Manifold"),
        arrow(int_ty(), arrow(nat_ty(), int_ty())),
    )
}
/// `PalaisSmaleCondition : Functional → SobolevSpace → Prop`
/// (PS): every sequence u_n with |F(u_n)| bounded and F'(u_n) → 0
///       has a convergent subsequence.
pub fn vc_ext_palais_smale_condition_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("SobolevSpace"), prop()))
}
/// `MountainPassTheorem : Functional → SobolevSpace → Prop`
/// Ambrosetti-Rabinowitz: if F satisfies (PS) and the mountain-pass geometry,
/// there exists a critical point at the minimax level.
pub fn vc_ext_mountain_pass_theorem_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("SobolevSpace"), prop()))
}
/// `MountainPassLevel : Functional → Real`
/// c = inf_{γ ∈ Γ} max_{t ∈ [0,1]} F(γ(t)) — the mountain-pass critical value.
pub fn vc_ext_mountain_pass_level_ty() -> Expr {
    arrow(cst("Functional"), real_ty())
}
/// `SaddlePointTheorem : Functional → SobolevSpace → Prop`
/// Rabinowitz saddle-point theorem for functionals with a linking geometry.
pub fn vc_ext_saddle_point_theorem_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("SobolevSpace"), prop()))
}
/// `LinkingGeometry : Functional → SobolevSpace → Prop`
/// The functional has a linking geometry: two subsets A, B with A ∩ B = ∅
/// and inf_B F > max_A F (enables saddle-point or mountain-pass arguments).
pub fn vc_ext_linking_geometry_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("SobolevSpace"), prop()))
}
/// `LyusternikSchnirelmannCategory : Manifold → Nat`
/// cat(M) = the minimum number of contractible open sets covering M.
pub fn vc_ext_lyusternik_schnirelmann_category_ty() -> Expr {
    arrow(cst("Manifold"), nat_ty())
}
/// `LjusternikSchnirelmannTheorem : Functional → Manifold → Prop`
/// A smooth functional on a manifold M has at least cat(M) critical points.
pub fn vc_ext_ljusternik_schnirelmann_theorem_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("Manifold"), prop()))
}
/// `CupLengthLowerBound : Manifold → Nat → Prop`
/// cat(M) ≥ cup-length of H*(M; k) + 1.
pub fn vc_ext_cup_length_lower_bound_ty() -> Expr {
    arrow(cst("Manifold"), arrow(nat_ty(), prop()))
}
/// `MorseIndex : Functional → Extremal → Nat`
/// The Morse index of a critical point: the number of negative eigenvalues
/// of the Hessian (second variation operator).
pub fn vc_ext_morse_index_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("Extremal"), nat_ty()))
}
/// `MorseInequalityWeak : Functional → Manifold → Nat → Prop`
/// Weak Morse inequality: C_k ≥ β_k where C_k = # critical points of index k
/// and β_k = k-th Betti number of M.
pub fn vc_ext_morse_inequality_weak_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("Functional"),
        pi(
            BinderInfo::Default,
            "M",
            cst("Manifold"),
            arrow(nat_ty(), prop()),
        ),
    )
}
/// `MorseInequalityStrong : Functional → Manifold → Nat → Prop`
/// Strong Morse inequality: Σ_{k≤n} (-1)^{n-k} C_k ≥ Σ_{k≤n} (-1)^{n-k} β_k.
pub fn vc_ext_morse_inequality_strong_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("Functional"),
        pi(
            BinderInfo::Default,
            "M",
            cst("Manifold"),
            arrow(nat_ty(), prop()),
        ),
    )
}
/// `MorseComplex : Functional → Manifold → Type`
/// The Morse complex (CM_*, ∂) generated by critical points, with differential
/// counting gradient flow lines between adjacent index critical points.
pub fn vc_ext_morse_complex_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("Manifold"), type0()))
}
/// `FloerComplex : Functional → Manifold → Type`
/// Floer's infinite-dimensional Morse complex for the action functional on
/// the loop space (used in symplectic Floer theory).
pub fn vc_ext_floer_complex_ty() -> Expr {
    arrow(cst("Functional"), arrow(cst("Manifold"), type0()))
}
/// `GradientFlowEquation : Functional → Manifold → (Real → Real) → Prop`
/// du/dt = -grad F(u) — the gradient flow ODE/PDE for the functional F.
pub fn vc_ext_gradient_flow_equation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("Functional"),
        pi(
            BinderInfo::Default,
            "M",
            cst("Manifold"),
            arrow(arrow(real_ty(), real_ty()), prop()),
        ),
    )
}
/// `EkelandVariationalPrinciple : Functional → SobolevSpace → Real → Prop`
/// For every ε > 0 and approximate minimiser u₀ with F(u₀) ≤ inf F + ε,
/// there exists u_ε with |u_ε − u₀| ≤ ε^(1/2) and F'(u_ε) is small.
pub fn vc_ext_ekeland_variational_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("Functional"),
        pi(
            BinderInfo::Default,
            "X",
            cst("SobolevSpace"),
            arrow(real_ty(), prop()),
        ),
    )
}
/// `ApproximateMinimiser : Functional → SobolevSpace → Real → Real → Prop`
/// u is an ε-approximate minimiser if F(u) ≤ inf F + ε.
pub fn vc_ext_approximate_minimiser_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("Functional"),
        pi(
            BinderInfo::Default,
            "X",
            cst("SobolevSpace"),
            arrow(real_ty(), arrow(real_ty(), prop())),
        ),
    )
}
/// `IsoperimetricInequality : Real → Real → Prop`
/// The classical isoperimetric inequality: 4π A ≤ L² for a planar domain
/// with area A and perimeter L, with equality iff the domain is a disk.
pub fn vc_ext_isoperimetric_inequality_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `ConstrainedEulerLagrange : Functional → Functional → Real → Prop`
/// Constrained E-L equation: F'[u] = λ G'[u] at a constrained critical point.
pub fn vc_ext_constrained_euler_lagrange_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("Functional"),
        pi(
            BinderInfo::Default,
            "G",
            cst("Functional"),
            arrow(real_ty(), prop()),
        ),
    )
}
/// `DualityGap : Functional → Functional → Real → Prop`
/// The duality gap F(u) − G*(0) ≥ 0 in convex duality, vanishing at the optimum.
pub fn vc_ext_duality_gap_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        cst("Functional"),
        pi(
            BinderInfo::Default,
            "G",
            cst("Functional"),
            arrow(real_ty(), prop()),
        ),
    )
}
/// `GeodesicFlow : Manifold → Real → (Real → Real) → Prop`
/// The geodesic flow φ_t on the unit tangent bundle T¹M.
pub fn vc_ext_geodesic_flow_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("Manifold"),
        arrow(real_ty(), arrow(arrow(real_ty(), real_ty()), prop())),
    )
}
/// `CutLocus : Manifold → Real → Real → Prop`
/// The cut locus of a point p ∈ M: locus where geodesics from p cease to be minimising.
pub fn vc_ext_cut_locus_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("Manifold"),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `IndexFormBilinear : Manifold → (Real → Real) → (Real → Real) → Real`
/// Index form I(V, W) = ∫_0^1 [⟨∇_γ V, ∇_γ W⟩ − R(V, γ', γ', W)] dt.
pub fn vc_ext_index_form_bilinear_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "M",
        cst("Manifold"),
        arrow(
            arrow(real_ty(), real_ty()),
            arrow(arrow(real_ty(), real_ty()), real_ty()),
        ),
    )
}
/// `StableMinimalSurface : Surface → Prop`
/// A minimal surface Σ is stable if the second variation of area is ≥ 0
/// for all compactly supported normal variations.
pub fn vc_ext_stable_minimal_surface_ty() -> Expr {
    arrow(cst("Surface"), prop())
}
/// `SchoenYauPositiveMassTheorem : Manifold → Prop`
/// Schoen-Yau: the ADM mass of an asymptotically flat Riemannian manifold
/// with non-negative scalar curvature is non-negative, zero iff flat.
pub fn vc_ext_schoen_yau_positive_mass_ty() -> Expr {
    arrow(cst("Manifold"), prop())
}
/// Register all extended variational calculus axioms (Section 18) into an environment.
pub fn register_variational_calculus_extended(env: &mut Environment) -> Result<(), String> {
    let base_types: &[(&str, fn() -> Expr)] = &[
        ("Manifold", || type0()),
        ("Connection", || type0()),
        ("Surface", || type0()),
        ("Lagrangian", || type0()),
        ("Symmetry", || type0()),
        ("Extremal", || type0()),
        ("SobolevSpace", || type0()),
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
        ("EulerLagrangePDE", vc_ext_euler_lagrange_pde_ty),
        ("DirichletPrinciple", vc_ext_dirichlet_principle_ty),
        ("DirichletEnergy", vc_ext_dirichlet_energy_ty),
        ("HarmonicMap", vc_ext_harmonic_map_ty),
        ("BiharmonicMap", vc_ext_biharmonic_map_ty),
        ("NoetherCurrentPDE", vc_ext_noether_current_pde_ty),
        ("ConservationLawPDE", vc_ext_conservation_law_pde_ty),
        ("EnergyMomentumTensor", vc_ext_energy_momentum_tensor_ty),
        ("YangMillsFunctional", vc_ext_yang_mills_functional_ty),
        ("YangMillsEquation", vc_ext_yang_mills_equation_ty),
        (
            "AntiSelfDualConnection",
            vc_ext_anti_self_dual_connection_ty,
        ),
        ("DonaldsonInvariant", vc_ext_donaldson_invariant_ty),
        ("PalaisSmaleCondition", vc_ext_palais_smale_condition_ty),
        ("MountainPassTheorem", vc_ext_mountain_pass_theorem_ty),
        ("MountainPassLevel", vc_ext_mountain_pass_level_ty),
        ("SaddlePointTheorem", vc_ext_saddle_point_theorem_ty),
        ("LinkingGeometry", vc_ext_linking_geometry_ty),
        (
            "LyusternikSchnirelmannCategory",
            vc_ext_lyusternik_schnirelmann_category_ty,
        ),
        (
            "LjusternikSchnirelmannTheorem",
            vc_ext_ljusternik_schnirelmann_theorem_ty,
        ),
        ("CupLengthLowerBound", vc_ext_cup_length_lower_bound_ty),
        ("MorseIndex", vc_ext_morse_index_ty),
        ("MorseInequalityWeak", vc_ext_morse_inequality_weak_ty),
        ("MorseInequalityStrong", vc_ext_morse_inequality_strong_ty),
        ("MorseComplex", vc_ext_morse_complex_ty),
        ("FloerComplex", vc_ext_floer_complex_ty),
        ("GradientFlowEquation", vc_ext_gradient_flow_equation_ty),
        (
            "EkelandVariationalPrinciple",
            vc_ext_ekeland_variational_principle_ty,
        ),
        ("ApproximateMinimiser", vc_ext_approximate_minimiser_ty),
        (
            "IsoperimetricInequality",
            vc_ext_isoperimetric_inequality_ty,
        ),
        (
            "ConstrainedEulerLagrange",
            vc_ext_constrained_euler_lagrange_ty,
        ),
        ("DualityGap", vc_ext_duality_gap_ty),
        ("GeodesicFlow", vc_ext_geodesic_flow_ty),
        ("CutLocus", vc_ext_cut_locus_ty),
        ("IndexFormBilinear", vc_ext_index_form_bilinear_ty),
        ("StableMinimalSurface", vc_ext_stable_minimal_surface_ty),
        (
            "SchoenYauPositiveMassTheorem",
            vc_ext_schoen_yau_positive_mass_ty,
        ),
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
/// Standard Noether correspondences for field theory.
pub fn vc_ext_standard_noether_correspondences() -> Vec<(&'static str, &'static str)> {
    vec![
        ("time translation", "energy E = ∫ T^{00} d³x"),
        ("space translation in x_i", "momentum P_i = ∫ T^{0i} d³x"),
        ("rotation in x_i-x_j plane", "angular momentum L_{ij}"),
        ("U(1) global phase", "electric charge Q = ∫ J^0 d³x"),
        ("scale invariance (conformal)", "dilatation current D^μ"),
        ("special conformal", "conformal current K^μ_ν"),
    ]
}
