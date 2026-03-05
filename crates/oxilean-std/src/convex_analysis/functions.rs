//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ADMMData, AdmmSolver, AlternatingProjectionSolver, ConvexCone, ConvexConjugate, ConvexFunction,
    ConvexProblemClass, ConvexProgram, Epigraph, ExtragradientMethod, FenchelConjugate,
    FenchelConjugateEvaluator, FenchelDualityPair, FunctionClass, Hyperplane, LagrangianDuality,
    MirrorDescent, ProxConfig, ProximalOperator, ProximalOperatorNew, ProximalPointAlgorithm,
    RecessionCone, SeparatingHyperplaneFinder, StepSchedule, Subdifferential, SubgradientMethod,
    SublevelSet, VariationalInequality,
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
pub fn mat_ty() -> Expr {
    list_ty(list_ty(real_ty()))
}
pub fn extended_real_ty() -> Expr {
    cst("ExtendedReal")
}
/// `IsConvexSet : (List Real -> Prop) -> Prop`
/// A set C ⊆ ℝ^n given as characteristic predicate is convex.
pub fn is_convex_set_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), prop())
}
/// `IsConvexFunction : (List Real -> Real) -> Prop`
/// f is convex: f(λx + (1-λ)y) ≤ λf(x) + (1-λ)f(y) for all λ ∈ [0,1].
pub fn is_convex_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `IsStrictlyConvexFunction : (List Real -> Real) -> Prop`
/// f is strictly convex: strict inequality for λ ∈ (0,1) and x ≠ y.
pub fn is_strictly_convex_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `IsStronglyConvexFunction : (List Real -> Real) -> Real -> Prop`
/// f is strongly convex with modulus m: f(λx+(1-λ)y) ≤ λf(x)+(1-λ)f(y) - (m/2)λ(1-λ)‖x-y‖².
pub fn is_strongly_convex_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `Epigraph : (List Real -> Real) -> (List Real -> Prop)`
/// epi(f) = {(x, t) | f(x) ≤ t}, encoded as a predicate on ℝ^{n+1}.
pub fn epigraph_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), fn_ty(vec_ty(), prop()))
}
/// `LevelSet : (List Real -> Real) -> Real -> (List Real -> Prop)`
/// lev_α(f) = {x | f(x) ≤ α}.
pub fn level_set_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(real_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `ClosureFunction : (List Real -> Real) -> (List Real -> Real)`
/// cl(f): lower semi-continuous closure of f.
pub fn closure_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), fn_ty(vec_ty(), real_ty()))
}
/// `IsLowerSemicontinuous : (List Real -> Real) -> Prop`
/// f is lsc iff its epigraph is closed.
pub fn is_lsc_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `IsProperConvex : (List Real -> Real) -> Prop`
/// f is proper: never −∞ and not identically +∞.
pub fn is_proper_convex_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `SupportingHyperplane : (List Real -> Prop) -> List Real -> Prop`
/// At boundary point x₀ of convex set C, ∃ nonzero c with c·x ≤ c·x₀ for all x ∈ C.
pub fn supporting_hyperplane_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), arrow(vec_ty(), prop()))
}
/// `SeparatingHyperplane : (List Real -> Prop) -> (List Real -> Prop) -> Prop`
/// Two disjoint convex sets can be separated by a hyperplane.
pub fn separating_hyperplane_ty() -> Expr {
    let set_ty = fn_ty(vec_ty(), prop());
    arrow(set_ty.clone(), arrow(set_ty, prop()))
}
/// `StrictSeparation : (List Real -> Prop) -> (List Real -> Prop) -> Prop`
/// Compact convex C and closed convex D, disjoint, can be strictly separated.
pub fn strict_separation_ty() -> Expr {
    let set_ty = fn_ty(vec_ty(), prop());
    arrow(set_ty.clone(), arrow(set_ty, prop()))
}
/// `SupportFunction : (List Real -> Prop) -> List Real -> Real`
/// σ_C(y) = sup_{x ∈ C} ⟨y, x⟩.
pub fn support_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), fn_ty(vec_ty(), real_ty()))
}
/// `GaugeFunction : (List Real -> Prop) -> List Real -> Real`
/// γ_C(x) = inf{t ≥ 0 | x ∈ t·C}.
pub fn gauge_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), fn_ty(vec_ty(), real_ty()))
}
/// `FenchelConjugate : (List Real -> Real) -> (List Real -> Real)`
/// f*(y) = sup_x {⟨y,x⟩ − f(x)}.
pub fn fenchel_conjugate_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), rn_r)
}
/// `Biconjugate : (List Real -> Real) -> (List Real -> Real)`
/// f** = cl(conv(f)): the lsc convex hull of f.
pub fn biconjugate_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), rn_r)
}
/// `FenchelYoungInequality : (List Real -> Real) -> Prop`
/// For any f and its conjugate f*: ⟨x, y⟩ ≤ f(x) + f*(y).
pub fn fenchel_young_inequality_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `FenchelYoungEquality : (List Real -> Real) -> Prop`
/// ⟨x, y⟩ = f(x) + f*(y) iff y ∈ ∂f(x) (subgradient condition).
pub fn fenchel_young_equality_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `LegendreFenchelTransform : (List Real -> Real) -> (List Real -> Real)`
/// Alias for FenchelConjugate for convex lsc functions.
pub fn legendre_fenchel_transform_ty() -> Expr {
    fenchel_conjugate_ty()
}
/// `ConjugateOfSum : Prop`
/// (f + g)*(y) = (f* □ g*)(y) where □ is inf-convolution (under qualification).
pub fn conjugate_of_sum_ty() -> Expr {
    prop()
}
/// `MorozovIdentity : (List Real -> Real) -> Prop`
/// Moreau decomposition: prox_f(x) + prox_{f*}(x) = x.
pub fn moreau_identity_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `Subgradient : (List Real -> Real) -> List Real -> List Real -> Prop`
/// g ∈ ∂f(x) iff f(y) ≥ f(x) + ⟨g, y - x⟩ for all y.
pub fn subgradient_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), arrow(vec_ty(), prop())),
    )
}
/// `Subdifferential : (List Real -> Real) -> List Real -> (List Real -> Prop)`
/// ∂f(x) = {g | g is a subgradient of f at x}.
pub fn subdifferential_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `NormalCone : (List Real -> Prop) -> List Real -> (List Real -> Prop)`
/// N_C(x) = {v | ⟨v, y-x⟩ ≤ 0 for all y ∈ C}.
pub fn normal_cone_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), prop()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `TangentCone : (List Real -> Prop) -> List Real -> (List Real -> Prop)`
/// T_C(x) = cl({d | ∃ t_k ↘ 0, x + t_k d ∈ C}).
pub fn tangent_cone_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), prop()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `OptimalityConditionConvex : (List Real -> Real) -> (List Real -> Prop) -> Prop`
/// x* minimizes f over C iff 0 ∈ ∂f(x*) + N_C(x*).
pub fn optimality_condition_convex_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(fn_ty(vec_ty(), prop()), prop()),
    )
}
/// `SubdiffOfSum : Prop`
/// Under constraint qualification: ∂(f+g)(x) ⊇ ∂f(x) + ∂g(x) (Moreau-Rockafellar).
pub fn subdiff_of_sum_ty() -> Expr {
    prop()
}
/// `SubdiffOfComposition : Prop`
/// Chain rule for subdifferentials of composed convex functions.
pub fn subdiff_of_composition_ty() -> Expr {
    prop()
}
/// `RecessionCone : (List Real -> Prop) -> (List Real -> Prop)`
/// rec(C) = {d | x + td ∈ C for all t ≥ 0, for some x ∈ C}.
pub fn recession_cone_ty() -> Expr {
    let set_ty = fn_ty(vec_ty(), prop());
    arrow(set_ty.clone(), set_ty)
}
/// `RecessionFunction : (List Real -> Real) -> (List Real -> Real)`
/// rec(f)(d) = lim_{t→∞} f(x + td)/t (horizon function of f).
pub fn recession_function_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), rn_r)
}
/// `IsCoercive : (List Real -> Real) -> Prop`
/// f is coercive: f(x) → +∞ as ‖x‖ → ∞.
pub fn is_coercive_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `IsLevelBounded : (List Real -> Real) -> Prop`
/// All level sets {x | f(x) ≤ α} are bounded.
pub fn is_level_bounded_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `ExistenceOfMinimum : (List Real -> Real) -> (List Real -> Prop) -> Prop`
/// Weierstrass theorem: lsc proper coercive f on closed set has a minimizer.
pub fn existence_of_minimum_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(fn_ty(vec_ty(), prop()), prop()),
    )
}
/// `ProximalOperator : (List Real -> Real) -> Real -> List Real -> List Real`
/// prox_{λf}(v) = argmin_x {f(x) + (1/2λ)‖x-v‖²}.
pub fn proximal_operator_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(real_ty(), fn_ty(vec_ty(), vec_ty())),
    )
}
/// `MoreauEnvelope : (List Real -> Real) -> Real -> List Real -> Real`
/// e_{λf}(x) = inf_y {f(y) + (1/2λ)‖x-y‖²}.
pub fn moreau_envelope_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(real_ty(), fn_ty(vec_ty(), real_ty())),
    )
}
/// `InfConvolution : (List Real -> Real) -> (List Real -> Real) -> List Real -> Real`
/// (f □ g)(x) = inf_{y} {f(y) + g(x - y)}.
pub fn inf_convolution_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), arrow(rn_r, fn_ty(vec_ty(), real_ty())))
}
/// `ProxFirmlyNonexpansive : (List Real -> Real) -> Prop`
/// prox_{λf} is firmly nonexpansive: ‖prox x - prox y‖² ≤ ⟨prox x - prox y, x - y⟩.
pub fn prox_firmly_nonexpansive_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `MoreauEnvelopeSmooth : (List Real -> Real) -> Prop`
/// e_{λf} is 1/λ-smooth and ∇e_{λf}(x) = (1/λ)(x - prox_{λf}(x)).
pub fn moreau_envelope_smooth_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `MoreauEnvelopeConvergence : (List Real -> Real) -> Prop`
/// e_{λf}(x) ↗ f(x) as λ ↘ 0 (pointwise approximation).
pub fn moreau_envelope_convergence_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `BregmanDivergence : (List Real -> Real) -> List Real -> List Real -> Real`
/// D_f(x, y) = f(x) - f(y) - ⟨∇f(y), x - y⟩ for differentiable f.
pub fn bregman_divergence_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        fn_ty(vec_ty(), fn_ty(vec_ty(), real_ty())),
    )
}
/// `BregmanProjection : (List Real -> Real) -> (List Real -> Prop) -> List Real -> List Real`
/// argmin_{y ∈ C} D_f(y, x) — Bregman projection of x onto C.
pub fn bregman_projection_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(fn_ty(vec_ty(), prop()), fn_ty(vec_ty(), vec_ty())),
    )
}
/// `BregmanThreePointIdentity : (List Real -> Real) -> Prop`
/// D_f(x, z) = D_f(x, y) + D_f(y, z) + ⟨∇f(y) - ∇f(z), x - y⟩.
pub fn bregman_three_point_identity_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `MirrorDescentStep : (List Real -> Real) -> (List Real -> Real) -> Real -> List Real -> List Real`
/// Mirror descent update: x_{k+1} = argmin_{y} {⟨g_k, y⟩ + (1/η) D_f(y, x_k)}.
pub fn mirror_descent_step_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(
        rn_r.clone(),
        arrow(rn_r, arrow(real_ty(), fn_ty(vec_ty(), vec_ty()))),
    )
}
/// `ExtremePoint : (List Real -> Prop) -> List Real -> Prop`
/// x is an extreme point of C: x ∉ (y, z) strictly for any y, z ∈ C with y ≠ z.
pub fn extreme_point_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), arrow(vec_ty(), prop()))
}
/// `CaratheodoryTheorem : (List Real -> Prop) -> Nat -> Prop`
/// Every point in conv(C) ⊆ ℝ^n can be written as a convex combination of ≤ n+1 points of C.
pub fn caratheodory_theorem_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), arrow(nat_ty(), prop()))
}
/// `KreinMilmanTheorem : (List Real -> Prop) -> Prop`
/// Every compact convex set is the closed convex hull of its extreme points.
pub fn krein_milman_theorem_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), prop())
}
/// `ConvexHull : (List Real -> Prop) -> (List Real -> Prop)`
/// conv(C) = smallest convex set containing C.
pub fn convex_hull_ty() -> Expr {
    let set_ty = fn_ty(vec_ty(), prop());
    arrow(set_ty.clone(), set_ty)
}
/// `ClosedConvexHull : (List Real -> Prop) -> (List Real -> Prop)`
/// cl(conv(C)).
pub fn closed_convex_hull_ty() -> Expr {
    let set_ty = fn_ty(vec_ty(), prop());
    arrow(set_ty.clone(), set_ty)
}
/// `FarkasLemma : Prop`
/// Ax = b, x ≥ 0 has a solution iff for all y with A^Ty ≥ 0 we have b^Ty ≥ 0.
pub fn farkas_lemma_ty() -> Expr {
    prop()
}
/// `FarkasLemmaMatrix : (List (List Real)) -> List Real -> Prop`
/// Farkas lemma parameterised by matrix A and vector b.
pub fn farkas_lemma_matrix_ty() -> Expr {
    arrow(mat_ty(), arrow(vec_ty(), prop()))
}
/// `GordonAlternative : (List (List Real)) -> Prop`
/// Gordon's theorem: either ∃x, Ax < 0 or ∃y ≥ 0, y ≠ 0, A^Ty = 0, but not both.
pub fn gordon_alternative_ty() -> Expr {
    arrow(mat_ty(), prop())
}
/// `TuckerAlternative : (List (List Real)) -> Prop`
/// Tucker's theorem of the alternative for linear inequalities.
pub fn tucker_alternative_ty() -> Expr {
    arrow(mat_ty(), prop())
}
/// `StiemkePosanAlternative : Prop`
/// Stiemke/Positivstellensatz alternative for polynomial systems.
pub fn stiemke_alternative_ty() -> Expr {
    prop()
}
/// `IsLipschitzGradient : (List Real -> Real) -> Real -> Prop`
/// ‖∇f(x) - ∇f(y)‖ ≤ L‖x-y‖ for all x, y.
pub fn is_lipschitz_gradient_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `StrongConvexityQuadraticBound : (List Real -> Real) -> Real -> Prop`
/// f(y) ≥ f(x) + ⟨∇f(x), y-x⟩ + (m/2)‖y-x‖² for all x, y.
pub fn strong_convexity_quadratic_bound_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `IsSmooth : (List Real -> Real) -> Real -> Prop`
/// f is L-smooth: has L-Lipschitz gradient.
pub fn is_smooth_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `DescentLemma : (List Real -> Real) -> Real -> Prop`
/// f(y) ≤ f(x) + ⟨∇f(x), y-x⟩ + (L/2)‖y-x‖² (upper quadratic bound for L-smooth f).
pub fn descent_lemma_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `ConvergenceRateGradient : (List Real -> Real) -> Real -> Prop`
/// Gradient descent on L-smooth convex f converges at rate O(1/k).
pub fn convergence_rate_gradient_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), arrow(real_ty(), prop()))
}
/// `LinearConvergenceStrongConvex : (List Real -> Real) -> Real -> Real -> Prop`
/// Gradient descent on m-strongly convex L-smooth f converges at linear rate (1 - m/L)^k.
pub fn linear_convergence_strong_convex_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `IsBarrierFunction : (List Real -> Prop) -> (List Real -> Real) -> Prop`
/// B is a barrier for C: B(x) → +∞ as x approaches the boundary of C.
pub fn is_barrier_function_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), prop()),
        arrow(fn_ty(vec_ty(), real_ty()), prop()),
    )
}
/// `IsSelfConcordant : (List Real -> Real) -> Prop`
/// f is self-concordant: |∇³f(x)[d,d,d]| ≤ 2(∇²f(x)[d,d])^(3/2) for all d.
pub fn is_self_concordant_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `LogBarrier : (List Real -> Prop) -> (List Real -> Real)`
/// B(x) = -∑ log(constraints(x)) — standard log barrier for a polytope.
pub fn log_barrier_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), fn_ty(vec_ty(), real_ty()))
}
/// `CentralPath : (List Real -> Real) -> (List Real -> Prop) -> Real -> (List Real -> Prop)`
/// x*(t) = argmin_x { t * f(x) + B(x) } — central path parameterised by t.
pub fn central_path_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(
            fn_ty(vec_ty(), prop()),
            arrow(real_ty(), fn_ty(vec_ty(), prop())),
        ),
    )
}
/// `InteriorPointConvergence : (List Real -> Real) -> (List Real -> Prop) -> Prop`
/// Central path converges to a solution as t → ∞.
pub fn interior_point_convergence_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(fn_ty(vec_ty(), prop()), prop()),
    )
}
/// `IsSOCConstraint : List Real -> Real -> Prop`
/// ‖Ax + b‖ ≤ c^T x + d — second-order cone (Lorentz cone) constraint.
pub fn is_soc_constraint_ty() -> Expr {
    arrow(vec_ty(), arrow(real_ty(), prop()))
}
/// `LorentzCone : Nat -> (List Real -> Prop)`
/// K_n = {(x, t) ∈ ℝ^{n+1} | ‖x‖ ≤ t}.
pub fn lorentz_cone_ty() -> Expr {
    arrow(nat_ty(), fn_ty(vec_ty(), prop()))
}
/// `PositiveSemidefiniteCone : Nat -> (List (List Real) -> Prop)`
/// PSD cone: S_n^+ = {X ∈ S_n | X ⪰ 0}.
pub fn psd_cone_ty() -> Expr {
    arrow(nat_ty(), fn_ty(mat_ty(), prop()))
}
/// `SOCPDuality : Prop`
/// Strong duality for second-order cone programming under Slater's condition.
pub fn socp_duality_ty() -> Expr {
    prop()
}
/// `SDPDuality : Prop`
/// Strong duality for semidefinite programming under Slater's condition.
pub fn sdp_duality_ty() -> Expr {
    prop()
}
/// `LagrangianFunction : (List Real -> Real) -> (List (List Real -> Real)) -> List Real -> List Real -> Real`
/// L(x, λ) = f(x) + ∑_i λ_i g_i(x) — Lagrangian of the primal problem.
pub fn lagrangian_function_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(
        rn_r.clone(),
        arrow(list_ty(rn_r), fn_ty(vec_ty(), fn_ty(vec_ty(), real_ty()))),
    )
}
/// `DualFunction : (List Real -> Real) -> (List (List Real -> Real)) -> List Real -> Real`
/// q(λ) = inf_x L(x, λ) — Lagrange dual function (always concave).
pub fn dual_function_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(
        rn_r.clone(),
        arrow(list_ty(rn_r), fn_ty(vec_ty(), real_ty())),
    )
}
/// `WeakDuality : Prop`
/// Weak duality: d* = sup_λ q(λ) ≤ p* = inf_x f(x).
pub fn weak_duality_ty() -> Expr {
    prop()
}
/// `StrongDuality : (List Real -> Real) -> (List (List Real -> Real)) -> Prop`
/// Strong duality: d* = p* (under constraint qualification like Slater's).
pub fn strong_duality_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), arrow(list_ty(rn_r), prop()))
}
/// `SlaterCondition : (List (List Real -> Real)) -> (List Real -> Prop) -> Prop`
/// ∃ x ∈ int(D) with g_i(x) < 0 for all i (strict feasibility).
pub fn slater_condition_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(list_ty(rn_r), arrow(fn_ty(vec_ty(), prop()), prop()))
}
/// `KKTConditions : (List Real -> Real) -> (List (List Real -> Real)) -> List Real -> List Real -> Prop`
/// KKT conditions: stationarity, primal feasibility, dual feasibility, complementary slackness.
pub fn kkt_conditions_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(
        rn_r.clone(),
        arrow(list_ty(rn_r), arrow(vec_ty(), arrow(vec_ty(), prop()))),
    )
}
/// `KKTSufficiency : (List Real -> Real) -> (List (List Real -> Real)) -> Prop`
/// For convex problems, KKT conditions are sufficient for global optimality.
pub fn kkt_sufficiency_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), arrow(list_ty(rn_r), prop()))
}
/// `DualityGap : (List Real -> Real) -> (List (List Real -> Real)) -> Real`
/// Gap = p* - d* ≥ 0.
pub fn duality_gap_ty() -> Expr {
    let rn_r = fn_ty(vec_ty(), real_ty());
    arrow(rn_r.clone(), arrow(list_ty(rn_r), real_ty()))
}
/// `FenchelRockafellarDuality : Prop`
/// inf_x { f(Ax) + g(x) } = - inf_y { f*(y) + g*(-A^T y) } under regularity.
pub fn fenchel_rockafellar_duality_ty() -> Expr {
    prop()
}
/// `ClarkeSubdifferential : (List Real -> Real) -> List Real -> (List Real -> Prop)`
/// ∂^C f(x) = conv { lim ∇f(x_k) | x_k → x, x_k ∉ null set }.
pub fn clarke_subdifferential_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), fn_ty(vec_ty(), prop())),
    )
}
/// `ClarkeGeneralisedGradient : (List Real -> Real) -> List Real -> List Real -> Prop`
/// v ∈ ∂^C f(x): the Clarke generalised directional derivative condition.
pub fn clarke_generalised_gradient_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), real_ty()),
        arrow(vec_ty(), arrow(vec_ty(), prop())),
    )
}
/// `IsRegularFunction : (List Real -> Real) -> Prop`
/// f is Clarke-regular: directional derivative equals Clarke directional derivative.
pub fn is_regular_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `NonSmoothChainRule : Prop`
/// Clarke chain rule: ∂^C(f ∘ g)(x) ⊆ ∂^C f(g(x)) ∘ ∂^C g(x) (in appropriate sense).
pub fn nonsmooth_chain_rule_ty() -> Expr {
    prop()
}
/// `IsMonotoneOperator : (List Real -> List Real -> Prop) -> Prop`
/// T is monotone: ⟨Tx - Ty, x - y⟩ ≥ 0 for all x, y.
pub fn is_monotone_operator_ty() -> Expr {
    let set_rel = fn_ty(vec_ty(), fn_ty(vec_ty(), prop()));
    arrow(set_rel, prop())
}
/// `IsMaximalMonotone : (List Real -> List Real -> Prop) -> Prop`
/// T is maximal monotone: no proper monotone extension exists.
pub fn is_maximal_monotone_ty() -> Expr {
    let set_rel = fn_ty(vec_ty(), fn_ty(vec_ty(), prop()));
    arrow(set_rel, prop())
}
/// `Resolvent : (List Real -> List Real -> Prop) -> Real -> (List Real -> List Real)`
/// J_{λT} = (I + λT)^{-1} — resolvent operator of T with parameter λ.
pub fn resolvent_ty() -> Expr {
    let set_rel = fn_ty(vec_ty(), fn_ty(vec_ty(), prop()));
    arrow(set_rel, arrow(real_ty(), fn_ty(vec_ty(), vec_ty())))
}
/// `YosidaApproximation : (List Real -> List Real -> Prop) -> Real -> (List Real -> List Real)`
/// T_λ = (1/λ)(I - J_{λT}) — Yosida approximation (Lipschitz, monotone).
pub fn yosida_approximation_ty() -> Expr {
    let set_rel = fn_ty(vec_ty(), fn_ty(vec_ty(), prop()));
    arrow(set_rel, arrow(real_ty(), fn_ty(vec_ty(), vec_ty())))
}
/// `BrouwerFixedPoint : (List Real -> List Real) -> (List Real -> Prop) -> Prop`
/// Every continuous function f : C → C on a compact convex set has a fixed point.
pub fn brouwer_fixed_point_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), vec_ty()),
        arrow(fn_ty(vec_ty(), prop()), prop()),
    )
}
/// `SchauderFixedPoint : (List Real -> List Real) -> (List Real -> Prop) -> Prop`
/// Infinite-dimensional generalisation: compact continuous map on convex compact set.
pub fn schauder_fixed_point_ty() -> Expr {
    arrow(
        fn_ty(vec_ty(), vec_ty()),
        arrow(fn_ty(vec_ty(), prop()), prop()),
    )
}
/// `MinimaxTheorem : (List Real -> List Real -> Real) -> (List Real -> Prop) -> (List Real -> Prop) -> Prop`
/// Rockafellar minimax: min_x max_y f(x,y) = max_y min_x f(x,y) under convex-concave.
pub fn minimax_theorem_ty() -> Expr {
    let bilinear = fn_ty(vec_ty(), fn_ty(vec_ty(), real_ty()));
    let set_ty = fn_ty(vec_ty(), prop());
    arrow(bilinear, arrow(set_ty.clone(), arrow(set_ty, prop())))
}
/// `OperatorSplitting : Prop`
/// Douglas-Rachford / ADMM splitting convergence for sum of maximal monotone operators.
pub fn operator_splitting_ty() -> Expr {
    prop()
}
/// `ProjectionOperator : (List Real -> Prop) -> List Real -> List Real`
/// proj_C(x) = argmin_{y ∈ C} ‖y - x‖.
pub fn projection_operator_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), fn_ty(vec_ty(), vec_ty()))
}
/// `IsNonexpansive : (List Real -> List Real) -> Prop`
/// ‖Tx - Ty‖ ≤ ‖x - y‖ for all x, y.
pub fn is_nonexpansive_ty() -> Expr {
    arrow(fn_ty(vec_ty(), vec_ty()), prop())
}
/// `IsFirmlyNonexpansive : (List Real -> List Real) -> Prop`
/// ‖Tx - Ty‖² ≤ ⟨Tx - Ty, x - y⟩ for all x, y.
pub fn is_firmly_nonexpansive_ty() -> Expr {
    arrow(fn_ty(vec_ty(), vec_ty()), prop())
}
/// `AlternatingProjectionConvergence : (List Real -> Prop) -> (List Real -> Prop) -> Prop`
/// von Neumann / Bregman: alternating projections converge to a point in C ∩ D.
pub fn alternating_projection_convergence_ty() -> Expr {
    let set_ty = fn_ty(vec_ty(), prop());
    arrow(set_ty.clone(), arrow(set_ty, prop()))
}
/// Build an [`Environment`] containing convex analysis axioms and theorems.
pub fn build_convex_analysis_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("IsConvexSet", is_convex_set_ty()),
        ("IsConvexFunction", is_convex_function_ty()),
        ("IsStrictlyConvexFunction", is_strictly_convex_function_ty()),
        ("IsStronglyConvexFunction", is_strongly_convex_function_ty()),
        ("Epigraph", epigraph_ty()),
        ("LevelSet", level_set_ty()),
        ("ClosureFunction", closure_function_ty()),
        ("IsLowerSemicontinuous", is_lsc_ty()),
        ("IsProperConvex", is_proper_convex_ty()),
        ("SupportingHyperplane", supporting_hyperplane_ty()),
        ("SeparatingHyperplane", separating_hyperplane_ty()),
        ("StrictSeparation", strict_separation_ty()),
        ("SupportFunction", support_function_ty()),
        ("GaugeFunction", gauge_function_ty()),
        ("FenchelConjugate", fenchel_conjugate_ty()),
        ("Biconjugate", biconjugate_ty()),
        ("FenchelYoungInequality", fenchel_young_inequality_ty()),
        ("FenchelYoungEquality", fenchel_young_equality_ty()),
        ("LegendreFenchelTransform", legendre_fenchel_transform_ty()),
        ("ConjugateOfSum", conjugate_of_sum_ty()),
        ("MoreauIdentity", moreau_identity_ty()),
        ("Subgradient", subgradient_ty()),
        ("Subdifferential", subdifferential_ty()),
        ("NormalCone", normal_cone_ty()),
        ("TangentCone", tangent_cone_ty()),
        (
            "OptimalityConditionConvex",
            optimality_condition_convex_ty(),
        ),
        ("SubdiffOfSum", subdiff_of_sum_ty()),
        ("SubdiffOfComposition", subdiff_of_composition_ty()),
        ("RecessionCone", recession_cone_ty()),
        ("RecessionFunction", recession_function_ty()),
        ("IsCoercive", is_coercive_ty()),
        ("IsLevelBounded", is_level_bounded_ty()),
        ("ExistenceOfMinimum", existence_of_minimum_ty()),
        ("ProximalOperator", proximal_operator_ty()),
        ("MoreauEnvelope", moreau_envelope_ty()),
        ("InfConvolution", inf_convolution_ty()),
        ("ProxFirmlyNonexpansive", prox_firmly_nonexpansive_ty()),
        ("MoreauEnvelopeSmooth", moreau_envelope_smooth_ty()),
        (
            "MoreauEnvelopeConvergence",
            moreau_envelope_convergence_ty(),
        ),
        ("BregmanDivergence", bregman_divergence_ty()),
        ("BregmanProjection", bregman_projection_ty()),
        (
            "BregmanThreePointIdentity",
            bregman_three_point_identity_ty(),
        ),
        ("MirrorDescentStep", mirror_descent_step_ty()),
        ("ExtremePoint", extreme_point_ty()),
        ("CaratheodoryTheorem", caratheodory_theorem_ty()),
        ("KreinMilmanTheorem", krein_milman_theorem_ty()),
        ("ConvexHull", convex_hull_ty()),
        ("ClosedConvexHull", closed_convex_hull_ty()),
        ("FarkasLemma", farkas_lemma_ty()),
        ("FarkasLemmaMatrix", farkas_lemma_matrix_ty()),
        ("GordonAlternative", gordon_alternative_ty()),
        ("TuckerAlternative", tucker_alternative_ty()),
        ("StiemkeAlternative", stiemke_alternative_ty()),
        ("IsLipschitzGradient", is_lipschitz_gradient_ty()),
        (
            "StrongConvexityQuadraticBound",
            strong_convexity_quadratic_bound_ty(),
        ),
        ("IsSmooth", is_smooth_ty()),
        ("DescentLemma", descent_lemma_ty()),
        ("ConvergenceRateGradient", convergence_rate_gradient_ty()),
        (
            "LinearConvergenceStrongConvex",
            linear_convergence_strong_convex_ty(),
        ),
        ("IsBarrierFunction", is_barrier_function_ty()),
        ("IsSelfConcordant", is_self_concordant_ty()),
        ("LogBarrier", log_barrier_ty()),
        ("CentralPath", central_path_ty()),
        ("InteriorPointConvergence", interior_point_convergence_ty()),
        ("IsSOCConstraint", is_soc_constraint_ty()),
        ("LorentzCone", lorentz_cone_ty()),
        ("PositiveSemidefiniteCone", psd_cone_ty()),
        ("SOCPDuality", socp_duality_ty()),
        ("SDPDuality", sdp_duality_ty()),
        ("LagrangianFunction", lagrangian_function_ty()),
        ("DualFunction", dual_function_ty()),
        ("WeakDuality", weak_duality_ty()),
        ("StrongDuality", strong_duality_ty()),
        ("SlaterCondition", slater_condition_ty()),
        ("KKTConditions", kkt_conditions_ty()),
        ("KKTSufficiency", kkt_sufficiency_ty()),
        ("DualityGap", duality_gap_ty()),
        (
            "FenchelRockafellarDuality",
            fenchel_rockafellar_duality_ty(),
        ),
        ("ClarkeSubdifferential", clarke_subdifferential_ty()),
        (
            "ClarkeGeneralisedGradient",
            clarke_generalised_gradient_ty(),
        ),
        ("IsRegularFunction", is_regular_function_ty()),
        ("NonSmoothChainRule", nonsmooth_chain_rule_ty()),
        ("IsMonotoneOperator", is_monotone_operator_ty()),
        ("IsMaximalMonotone", is_maximal_monotone_ty()),
        ("Resolvent", resolvent_ty()),
        ("YosidaApproximation", yosida_approximation_ty()),
        ("BrouwerFixedPoint", brouwer_fixed_point_ty()),
        ("SchauderFixedPoint", schauder_fixed_point_ty()),
        ("MinimaxTheorem", minimax_theorem_ty()),
        ("OperatorSplitting", operator_splitting_ty()),
        ("ProjectionOperator", projection_operator_ty()),
        ("IsNonexpansive", is_nonexpansive_ty()),
        ("IsFirmlyNonexpansive", is_firmly_nonexpansive_ty()),
        (
            "AlternatingProjectionConvergence",
            alternating_projection_convergence_ty(),
        ),
    ];
    for (name, ty) in axioms {
        let decl = Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        };
        let _ = env.add(decl);
    }
    env
}
/// Computes a subgradient of a convex function at a given point via finite differences.
pub fn compute_subgradient(f: &ConvexFunction, x: &[f64]) -> Vec<f64> {
    f.gradient(x)
}
/// Checks the subgradient inequality: f(y) ≥ f(x) + ⟨g, y-x⟩.
pub fn check_subgradient_inequality(f: &ConvexFunction, x: &[f64], g: &[f64], y: &[f64]) -> bool {
    let fx = f.eval(x);
    let fy = f.eval(y);
    let inner: f64 = g
        .iter()
        .zip(y.iter().zip(x.iter()))
        .map(|(gi, (yi, xi))| gi * (yi - xi))
        .sum();
    fy >= fx + inner - 1e-9
}
/// Checks v ∈ N_C(x): ⟨v, y - x⟩ ≤ ε for all sampled y in C.
pub fn check_normal_cone_membership(v: &[f64], x: &[f64], set_points: &[Vec<f64>]) -> bool {
    for y in set_points {
        let dot: f64 = v
            .iter()
            .zip(y.iter().zip(x.iter()))
            .map(|(vi, (yi, xi))| vi * (yi - xi))
            .sum();
        if dot > 1e-9 {
            return false;
        }
    }
    true
}
/// Compute prox_{λf}(v) = argmin_x { f(x) + 1/(2λ) ‖x-v‖² } using gradient descent.
pub fn proximal_operator(f: &ConvexFunction, v: &[f64], cfg: &ProxConfig) -> Vec<f64> {
    let n = v.len();
    let mut x = v.to_vec();
    let inv_lambda = 1.0 / cfg.lambda;
    for _ in 0..cfg.max_iter {
        let grad_f = f.gradient(&x);
        let mut new_x = vec![0.0; n];
        let mut delta_sq = 0.0;
        for i in 0..n {
            let g = grad_f[i] + inv_lambda * (x[i] - v[i]);
            let dx = cfg.step_size * g;
            new_x[i] = x[i] - dx;
            delta_sq += dx * dx;
        }
        x = new_x;
        if delta_sq.sqrt() < cfg.tol {
            break;
        }
    }
    x
}
/// Compute Moreau envelope e_{λf}(x) = inf_y { f(y) + 1/(2λ)‖x-y‖² }.
pub fn moreau_envelope(f: &ConvexFunction, x: &[f64], cfg: &ProxConfig) -> f64 {
    let p = proximal_operator(f, x, cfg);
    let dist_sq: f64 = x
        .iter()
        .zip(p.iter())
        .map(|(xi, pi)| (xi - pi).powi(2))
        .sum();
    f.eval(&p) + dist_sq / (2.0 * cfg.lambda)
}
/// Compute inf-convolution (f □ g)(x) = inf_y { f(y) + g(x-y) } by grid search in 1D.
pub fn inf_convolution_1d(
    f: fn(f64) -> f64,
    g: fn(f64) -> f64,
    x: f64,
    steps: usize,
    radius: f64,
) -> f64 {
    let step = 2.0 * radius / steps as f64;
    let mut best = f64::INFINITY;
    for k in 0..=steps {
        let y = -radius + k as f64 * step;
        let val = f(y) + g(x - y);
        if val < best {
            best = val;
        }
    }
    best
}
/// Bregman divergence D_f(x, y) = f(x) - f(y) - ⟨∇f(y), x-y⟩.
pub fn bregman_divergence(f: &ConvexFunction, x: &[f64], y: &[f64]) -> f64 {
    let fy = f.eval(y);
    let fx = f.eval(x);
    let grad_y = f.gradient(y);
    let inner: f64 = grad_y
        .iter()
        .zip(x.iter().zip(y.iter()))
        .map(|(g, (xi, yi))| g * (xi - yi))
        .sum();
    fx - fy - inner
}
/// Check three-point identity (approximately):
/// D_f(x, z) ≈ D_f(x, y) + D_f(y, z) + ⟨∇f(y) - ∇f(z), x - y⟩.
pub fn check_three_point_identity(f: &ConvexFunction, x: &[f64], y: &[f64], z: &[f64]) -> bool {
    let lhs = bregman_divergence(f, x, z);
    let d_xy = bregman_divergence(f, x, y);
    let d_yz = bregman_divergence(f, y, z);
    let grad_y = f.gradient(y);
    let grad_z = f.gradient(z);
    let inner: f64 = grad_y
        .iter()
        .zip(grad_z.iter())
        .zip(x.iter().zip(y.iter()))
        .map(|((gy, gz), (xi, yi))| (gy - gz) * (xi - yi))
        .sum();
    let rhs = d_xy + d_yz + inner;
    (lhs - rhs).abs() < 1e-5
}
/// Compute a separating hyperplane between two finite point sets using the midpoint normal.
pub fn compute_separating_hyperplane(
    a_points: &[Vec<f64>],
    b_points: &[Vec<f64>],
) -> Option<Hyperplane> {
    if a_points.is_empty() || b_points.is_empty() {
        return None;
    }
    let n = a_points[0].len();
    let ca: Vec<f64> = (0..n)
        .map(|i| a_points.iter().map(|p| p[i]).sum::<f64>() / a_points.len() as f64)
        .collect();
    let cb: Vec<f64> = (0..n)
        .map(|i| b_points.iter().map(|p| p[i]).sum::<f64>() / b_points.len() as f64)
        .collect();
    let normal: Vec<f64> = ca.iter().zip(cb.iter()).map(|(a, b)| a - b).collect();
    let norm = normal.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm < 1e-12 {
        return None;
    }
    let unit_normal: Vec<f64> = normal.iter().map(|x| x / norm).collect();
    let mid: Vec<f64> = ca
        .iter()
        .zip(cb.iter())
        .map(|(a, b)| (a + b) / 2.0)
        .collect();
    let offset: f64 = unit_normal.iter().zip(mid.iter()).map(|(a, b)| a * b).sum();
    Some(Hyperplane::new(unit_normal, offset))
}
/// Compute σ_C(y) = max_{x ∈ C} ⟨y, x⟩ over a finite set of points.
pub fn support_function(c_points: &[Vec<f64>], y: &[f64]) -> f64 {
    c_points
        .iter()
        .map(|x| x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum::<f64>())
        .fold(f64::NEG_INFINITY, f64::max)
}
/// Compute gauge function γ_C(x) = inf { t ≥ 0 | x ∈ t·C } via binary search.
pub fn gauge_function(c_points: &[Vec<f64>], x: &[f64]) -> f64 {
    let norm_x: f64 = x.iter().map(|xi| xi * xi).sum::<f64>().sqrt();
    if norm_x < 1e-15 {
        return 0.0;
    }
    let mut lo = 0.0_f64;
    let mut hi = norm_x * 100.0 + 1.0;
    for _ in 0..60 {
        let mid = (lo + hi) / 2.0;
        let scaled: Vec<f64> = x.iter().map(|xi| xi / mid.max(1e-15)).collect();
        let s = support_function(c_points, &scaled);
        let s_x = support_function(c_points, x);
        if s_x <= mid * s + 1e-9 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    hi
}
#[cfg(test)]
mod tests {
    use super::*;
    fn sq_func(x: &[f64]) -> f64 {
        x.iter().map(|xi| xi * xi).sum::<f64>() * 0.5
    }
    fn sq_grad(x: &[f64]) -> Vec<f64> {
        x.to_vec()
    }
    #[test]
    fn test_build_env_keys() {
        let env = build_convex_analysis_env();
        assert!(env.get(&Name::str("IsConvexFunction")).is_some());
        assert!(env.get(&Name::str("FenchelConjugate")).is_some());
        assert!(env.get(&Name::str("Subdifferential")).is_some());
        assert!(env.get(&Name::str("NormalCone")).is_some());
        assert!(env.get(&Name::str("ProximalOperator")).is_some());
        assert!(env.get(&Name::str("MoreauEnvelope")).is_some());
        assert!(env.get(&Name::str("BregmanDivergence")).is_some());
        assert!(env.get(&Name::str("InfConvolution")).is_some());
    }
    #[test]
    fn test_epigraph_membership() {
        let f = ConvexFunction::new("sq", sq_func, Some(sq_grad));
        assert!(f.in_epigraph(&[1.0, 1.0], 1.5));
        assert!(!f.in_epigraph(&[1.0, 1.0], 0.5));
    }
    #[test]
    fn test_level_set_membership() {
        let f = ConvexFunction::new("sq", sq_func, Some(sq_grad));
        assert!(f.in_level_set(&[0.5, 0.5], 1.0));
        assert!(!f.in_level_set(&[2.0, 0.0], 1.0));
    }
    #[test]
    fn test_subgradient_inequality() {
        let f = ConvexFunction::new("sq", sq_func, Some(sq_grad));
        let x = vec![1.0];
        let g = f.gradient(&x);
        let y = vec![3.0];
        assert!(check_subgradient_inequality(&f, &x, &g, &y));
        let y2 = vec![-2.0];
        assert!(check_subgradient_inequality(&f, &x, &g, &y2));
    }
    #[test]
    fn test_proximal_operator_quadratic() {
        let f = ConvexFunction::new("sq", sq_func, Some(sq_grad));
        let cfg = ProxConfig::new(1.0);
        let v = vec![2.0];
        let p = proximal_operator(&f, &v, &cfg);
        assert!((p[0] - 1.0).abs() < 1e-3, "prox = {}", p[0]);
    }
    #[test]
    fn test_moreau_envelope_quadratic() {
        let f = ConvexFunction::new("sq", sq_func, Some(sq_grad));
        let cfg = ProxConfig::new(1.0);
        let x = vec![2.0];
        let env_val = moreau_envelope(&f, &x, &cfg);
        let expected = 0.5 * 4.0 / 2.0;
        assert!((env_val - expected).abs() < 1e-3, "env = {}", env_val);
    }
    #[test]
    fn test_bregman_divergence_quadratic() {
        let f = ConvexFunction::new("sq", sq_func, Some(sq_grad));
        let x = vec![3.0, 0.0];
        let y = vec![1.0, 0.0];
        let d = bregman_divergence(&f, &x, &y);
        assert!((d - 2.0).abs() < 1e-6, "D_f = {d}");
    }
    #[test]
    fn test_bregman_three_point_identity() {
        let f = ConvexFunction::new("sq", sq_func, Some(sq_grad));
        let x = vec![3.0];
        let y = vec![1.0];
        let z = vec![-1.0];
        assert!(check_three_point_identity(&f, &x, &y, &z));
    }
    #[test]
    fn test_separating_hyperplane() {
        let a = vec![vec![0.0, 0.0], vec![0.5, 0.0]];
        let b = vec![vec![2.0, 0.0], vec![2.5, 0.0]];
        let hp = compute_separating_hyperplane(&a, &b).expect("operation should succeed");
        assert!(hp.separates(&a, &b), "hyperplane should separate A from B");
    }
    #[test]
    fn test_support_function_unit_square() {
        let c = vec![
            vec![0.0, 0.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
            vec![0.0, 1.0],
        ];
        let s1 = support_function(&c, &[1.0, 0.0]);
        assert!((s1 - 1.0).abs() < 1e-12, "σ(1,0) = 1");
        let s2 = support_function(&c, &[1.0, 1.0]);
        assert!((s2 - 2.0).abs() < 1e-12, "σ(1,1) = 2");
    }
    #[test]
    fn test_inf_convolution_1d() {
        let f = |x: f64| 0.5 * x * x;
        let g = |x: f64| 0.5 * x * x;
        let x = 4.0;
        let val = inf_convolution_1d(f, g, x, 1000, 10.0);
        let expected = 0.25 * x * x;
        assert!(
            (val - expected).abs() < 0.05,
            "inf-conv = {val}, expected {expected}"
        );
    }
    #[test]
    fn test_build_env_new_axioms() {
        let env = build_convex_analysis_env();
        assert!(env.get(&Name::str("ExtremePoint")).is_some());
        assert!(env.get(&Name::str("CaratheodoryTheorem")).is_some());
        assert!(env.get(&Name::str("KreinMilmanTheorem")).is_some());
        assert!(env.get(&Name::str("ConvexHull")).is_some());
        assert!(env.get(&Name::str("FarkasLemma")).is_some());
        assert!(env.get(&Name::str("GordonAlternative")).is_some());
        assert!(env.get(&Name::str("IsLipschitzGradient")).is_some());
        assert!(env.get(&Name::str("IsSmooth")).is_some());
        assert!(env.get(&Name::str("DescentLemma")).is_some());
        assert!(env.get(&Name::str("IsBarrierFunction")).is_some());
        assert!(env.get(&Name::str("IsSelfConcordant")).is_some());
        assert!(env.get(&Name::str("CentralPath")).is_some());
        assert!(env.get(&Name::str("LorentzCone")).is_some());
        assert!(env.get(&Name::str("PositiveSemidefiniteCone")).is_some());
        assert!(env.get(&Name::str("SOCPDuality")).is_some());
        assert!(env.get(&Name::str("KKTConditions")).is_some());
        assert!(env.get(&Name::str("SlaterCondition")).is_some());
        assert!(env.get(&Name::str("StrongDuality")).is_some());
        assert!(env.get(&Name::str("FenchelRockafellarDuality")).is_some());
        assert!(env.get(&Name::str("ClarkeSubdifferential")).is_some());
        assert!(env.get(&Name::str("IsRegularFunction")).is_some());
        assert!(env.get(&Name::str("IsMonotoneOperator")).is_some());
        assert!(env.get(&Name::str("Resolvent")).is_some());
        assert!(env.get(&Name::str("BrouwerFixedPoint")).is_some());
        assert!(env.get(&Name::str("MinimaxTheorem")).is_some());
        assert!(env.get(&Name::str("ProjectionOperator")).is_some());
        assert!(env
            .get(&Name::str("AlternatingProjectionConvergence"))
            .is_some());
    }
    #[test]
    fn test_subgradient_method_quadratic() {
        let f = ConvexFunction::new("sq", sq_func, Some(sq_grad));
        let method = SubgradientMethod::new(StepSchedule::DiminishingSqrt(1.0), 500);
        let (best, _history) = method.run(&f, &[5.0]);
        assert!(
            best[0].abs() < 0.5,
            "should converge near 0, got {}",
            best[0]
        );
    }
    #[test]
    fn test_proximal_point_algorithm_quadratic() {
        let f = ConvexFunction::new("sq", sq_func, Some(sq_grad));
        let ppa = ProximalPointAlgorithm::constant(1.0, 50, 1e-6);
        let iterates = ppa.run(&f, &[4.0]);
        let last = iterates.last().expect("last should succeed");
        assert!(
            last[0].abs() < 0.1,
            "PPA should converge near 0, got {}",
            last[0]
        );
    }
    #[test]
    fn test_fenchel_conjugate_squared_norm() {
        let ev = FenchelConjugateEvaluator::new(FunctionClass::SquaredNorm);
        let y = vec![3.0, 4.0];
        let conj = ev.eval(&y);
        assert!((conj - 12.5).abs() < 1e-10, "f*(3,4) = 12.5, got {conj}");
    }
    #[test]
    fn test_fenchel_young_inequality() {
        let ev = FenchelConjugateEvaluator::new(FunctionClass::SquaredNorm);
        let x = vec![1.0, 2.0];
        let y = vec![3.0, 0.5];
        let fx = 0.5 * (1.0_f64 + 4.0_f64);
        assert!(ev.check_fenchel_young(&x, &y, fx));
    }
    #[test]
    fn test_fenchel_conjugate_negative_entropy() {
        let ev = FenchelConjugateEvaluator::new(FunctionClass::NegativeEntropy);
        let y = vec![1.0];
        let val = ev.eval(&y);
        assert!((val - 1.0).abs() < 1e-10, "f*(1) = exp(0) = 1, got {val}");
    }
    #[test]
    fn test_fenchel_conjugate_box_indicator() {
        let ev = FenchelConjugateEvaluator::new(FunctionClass::BoxIndicator { lo: -1.0, hi: 1.0 });
        let y = vec![2.0, -3.0];
        let val = ev.eval(&y);
        assert!((val - 5.0).abs() < 1e-10, "f*(2,-3) = 5, got {val}");
    }
    #[test]
    fn test_separating_hyperplane_finder() {
        let a = vec![vec![0.0, 0.0], vec![0.0, 1.0]];
        let b = vec![vec![3.0, 0.0], vec![3.0, 1.0]];
        let finder = SeparatingHyperplaneFinder::new(0.01, 200, 1e-4);
        let hp = finder.find(&a, &b);
        assert!(hp.is_some(), "should find a separating hyperplane");
        let hp = hp.expect("hp should be valid");
        assert!(hp.separates(&a, &b), "hyperplane should separate A and B");
    }
    #[test]
    fn test_alternating_projections_intersection() {
        let proj_a = |x: &[f64]| vec![x[0].max(1.0)];
        let proj_b = |x: &[f64]| vec![x[0].min(2.0)];
        let solver = AlternatingProjectionSolver::new(100, 1e-8);
        let iters = solver.run(proj_a, proj_b, &[5.0]);
        let last = iters.last().expect("last should succeed");
        assert!(
            (last[0] - 2.0).abs() < 1e-6,
            "should converge to 2, got {}",
            last[0]
        );
    }
}
#[cfg(test)]
mod convex_ext_tests {
    use super::*;
    #[test]
    fn test_convex_cone() {
        let soc = ConvexCone::second_order_cone(3);
        assert!(soc.is_pointed);
        assert!(soc.is_closed);
        assert!(!soc.dual_cone_description().is_empty());
    }
    #[test]
    fn test_fenchel_conjugate() {
        let fc = FenchelConjugate::new("||x||^2/2");
        assert!(!fc.definition().is_empty());
        assert!(!fc.fenchel_inequality().is_empty());
    }
    #[test]
    fn test_proximal_operator() {
        let prox = ProximalOperator::new("||x||_1", 0.1);
        assert!(!prox.definition().is_empty());
        assert!(!prox.for_indicator_function().is_empty());
    }
    #[test]
    fn test_admm() {
        let admm = AdmmSolver::new(1.0, 1000, 1e-4);
        assert!(!admm.convergence_description().is_empty());
    }
    #[test]
    fn test_convex_program() {
        let lp = ConvexProgram::new("diet", 100, 50, ConvexProblemClass::Lp);
        assert!(lp.is_lp());
        assert!(lp.strong_duality_holds());
    }
}
#[cfg(test)]
mod duality_ext_tests {
    use super::*;
    #[test]
    fn test_lagrangian_duality() {
        let ld = LagrangianDuality::new("min f(x)", "max g(lambda)", 0.0);
        assert!(ld.strong_duality());
        assert_eq!(ld.kkt_conditions().len(), 4);
    }
    #[test]
    fn test_mirror_descent() {
        let md = MirrorDescent::with_entropy(0.01, 1000);
        let rate = md.convergence_rate(1.0, 1.0);
        assert!(rate > 0.0 && rate < 1.0);
    }
}
#[cfg(test)]
mod epi_level_tests {
    use super::*;
    #[test]
    fn test_epigraph() {
        let epi = Epigraph::new("||x||^2");
        assert!(epi.f_convex_iff_epi_convex());
        assert!(!epi.definition().is_empty());
    }
    #[test]
    fn test_sublevel_set() {
        let sl = SublevelSet::new("exp(x)", 2.0);
        assert!(sl.is_convex_if_f_quasiconvex());
    }
    #[test]
    fn test_recession_cone() {
        let rc = RecessionCone::new("R^n+");
        assert!(rc.compact_iff_trivial_recession());
    }
}
#[cfg(test)]
mod tests_convex_analysis_ext {
    use super::*;
    #[test]
    fn test_convex_conjugate() {
        let quad = ConvexConjugate::quadratic_conjugate();
        assert!(quad.biconjugate_equals_f());
        let fm = quad.fenchel_moreau_theorem();
        assert!(fm.contains("Fenchel-Moreau"));
        let yf = quad.young_fenchel_inequality();
        assert!(yf.contains("Young-Fenchel"));
        let ind = ConvexConjugate::indicator_conjugate("C");
        assert!(ind.conjugate_name.contains("h_"));
    }
    #[test]
    fn test_fenchel_duality() {
        let strong = FenchelDualityPair::new("min f", "max -f*", 0.0);
        assert!(strong.strong_duality_holds);
        assert!(strong.slater_condition_met());
        let desc = strong.duality_gap_description();
        assert!(desc.contains("Strong duality"));
    }
    #[test]
    fn test_proximal_l1() {
        let prox = ProximalOperatorNew::l1_norm(0.5);
        assert!(prox.has_closed_form);
        let formula = prox.proximal_point_formula();
        assert!(formula.contains("prox_"));
        let moreau = prox.moreau_decomposition();
        assert!(moreau.contains("Moreau"));
    }
    #[test]
    fn test_admm() {
        let mut admm = ADMMData::new(1.0);
        assert!(!admm.has_converged(1e-6));
        admm.update_residuals(1e-8, 1e-8);
        assert!(admm.has_converged(1e-6));
        let desc = admm.admm_update_description();
        assert!(desc.contains("ADMM"));
        let conv = admm.convergence_guarantee();
        assert!(conv.contains("ADMM"));
    }
    #[test]
    fn test_variational_inequality() {
        let vi = VariationalInequality::new("F", "C", true);
        let form = vi.vi_formulation();
        assert!(form.contains("F(x*)"));
        let stamp = vi.stampacchia_existence();
        assert!(stamp.contains("Stampacchia"));
        let svi = VariationalInequality::strongly_monotone("G", "K", 0.5);
        assert!(svi.unique_solution_exists());
    }
    #[test]
    fn test_extragradient() {
        let mut eg = ExtragradientMethod::new(0.01, "F");
        assert!(eg.convergence_condition(50.0));
        assert!(!eg.convergence_condition(200.0));
        eg.do_step(0.5);
        assert_eq!(eg.iterations, 1);
        let desc = eg.korpelevich_step_description();
        assert!(desc.contains("Korpelevich"));
    }
}
