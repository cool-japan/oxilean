//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BundleMethodSolver, CuttingPlaneSolver, FISTASolver, GeometricProgramSolver, GradientDescent,
    L1NormFunction, MirrorDescentSolver, OnlineLearner, ProjectedGradient, ProximalGradientSolver,
    QuadraticFunction, RipVerifier, SDPRelaxation, SinkhornSolver, ADMM,
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
/// `ConvexSet : (List Real -> Prop) -> Prop`
/// Predicate asserting a set (represented as a characteristic function) is convex.
pub fn convex_set_ty() -> Expr {
    arrow(fn_ty(list_ty(real_ty()), prop()), prop())
}
/// `ConvexFunction : (List Real -> Real) -> Prop`
/// Predicate asserting f: ℝ^n → ℝ is a convex function.
pub fn convex_function_ty() -> Expr {
    arrow(fn_ty(list_ty(real_ty()), real_ty()), prop())
}
/// `KktConditions : (List Real -> Real) -> List (List Real -> Real) -> List Real -> Prop`
/// KKT optimality conditions for a constrained convex program.
pub fn kkt_conditions_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let list_rn_to_r = list_ty(rn_to_r.clone());
    arrow(
        rn_to_r,
        arrow(list_rn_to_r, arrow(list_ty(real_ty()), prop())),
    )
}
/// `Lagrangian : (List Real -> Real) -> List (List Real -> Real) -> List Real -> List Real -> Real`
/// Lagrangian function L(x, λ) = f(x) + Σ λ_i g_i(x).
pub fn lagrangian_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let list_rn_to_r = list_ty(rn_to_r.clone());
    arrow(
        rn_to_r,
        arrow(
            list_rn_to_r,
            arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
        ),
    )
}
/// `StrongDuality : Prop`
/// Slater's condition implies strong duality: primal optimal = dual optimal.
pub fn strong_duality_ty() -> Expr {
    prop()
}
/// `ProjectionTheorem : Prop`
/// Every nonempty closed convex set has a unique nearest point (projection).
pub fn projection_theorem_ty() -> Expr {
    prop()
}
/// `SupportingHyperplane : Prop`
/// At every boundary point of a convex set there exists a supporting hyperplane.
pub fn supporting_hyperplane_ty() -> Expr {
    prop()
}
/// `JensenInequality : (List Real -> Real) -> Prop`
/// For convex f: f(E[X]) ≤ E[f(X)].
pub fn jensen_inequality_ty() -> Expr {
    arrow(fn_ty(list_ty(real_ty()), real_ty()), prop())
}
/// `SlaterCondition : Prop`
/// Strong duality holds for convex programs satisfying Slater's condition.
pub fn slater_condition_ty() -> Expr {
    prop()
}
/// `FenchelConjugate : (List Real -> Real) -> List Real -> Real`
/// The Fenchel conjugate f*(y) = sup_x { <y,x> - f(x) }.
pub fn fenchel_conjugate_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(rn_to_r, fn_ty(list_ty(real_ty()), real_ty()))
}
/// `FenchelRockafellarDuality : Prop`
/// inf f(x) + g(Ax) = -inf f*(-A^T y) + g*(y) under qualification.
pub fn fenchel_rockafellar_duality_ty() -> Expr {
    prop()
}
/// `ConjugateSubgradient : (List Real -> Real) -> List Real -> List Real -> Prop`
/// y ∈ ∂f*(x*) iff x* ∈ ∂f(y) (Fenchel–Young equality condition).
pub fn conjugate_subgradient_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let lr = list_ty(real_ty());
    arrow(rn_to_r, arrow(lr.clone(), arrow(lr, prop())))
}
/// `LagrangianDualFunction : (List Real -> Real) -> List (List Real -> Real) -> List Real -> Real`
/// Dual function q(λ) = inf_x L(x, λ).
pub fn lagrangian_dual_function_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let list_rn_to_r = list_ty(rn_to_r.clone());
    arrow(
        rn_to_r,
        arrow(list_rn_to_r, fn_ty(list_ty(real_ty()), real_ty())),
    )
}
/// `LinearIndependenceConstraintQualification : Prop`
/// LICQ: gradients of active constraints are linearly independent at x*.
pub fn licq_ty() -> Expr {
    prop()
}
/// `MangasarianFromovitzCQ : Prop`
/// MFCQ constraint qualification weaker than LICQ.
pub fn mfcq_ty() -> Expr {
    prop()
}
/// `ComplementarySlackness : List Real -> List Real -> Prop`
/// λ_i g_i(x) = 0 for all i (complementary slackness conditions).
pub fn complementary_slackness_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(lr.clone(), arrow(lr, prop()))
}
/// `KktSufficiency : Prop`
/// Under convexity, KKT conditions are sufficient for global optimality.
pub fn kkt_sufficiency_ty() -> Expr {
    prop()
}
/// `BarrierFunction : (List Real -> Prop) -> (List Real -> Real) -> Prop`
/// A barrier function for a convex set: diverges at boundary.
pub fn barrier_function_ty() -> Expr {
    let rn_to_prop = fn_ty(list_ty(real_ty()), prop());
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(rn_to_prop, arrow(rn_to_r, prop()))
}
/// `PathFollowingMethod : Prop`
/// Central path: x*(t) minimizes t·f(x) + φ(x) for barrier φ.
pub fn path_following_method_ty() -> Expr {
    prop()
}
/// `PredictorCorrectorMethod : Prop`
/// Mehrotra predictor-corrector interior point algorithm converges in O(√n·L) steps.
pub fn predictor_corrector_method_ty() -> Expr {
    prop()
}
/// `CentralPathConvergence : Real -> Prop`
/// The central path converges to the primal-dual optimal as t → ∞.
pub fn central_path_convergence_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `PositiveSemidefinite : List (List Real) -> Prop`
/// Predicate that a matrix (as list of rows) is positive semidefinite.
pub fn positive_semidefinite_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    arrow(mat, prop())
}
/// `SdpFeasibility : List (List (List Real)) -> List Real -> Prop`
/// SDP feasibility: ∑ x_i A_i ⪰ 0 for given data matrices {A_i}.
pub fn sdp_feasibility_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    let mats = list_ty(mat);
    let lr = list_ty(real_ty());
    arrow(mats, arrow(lr, prop()))
}
/// `SdpDuality : Prop`
/// SDP strong duality under Slater's condition (Alaoglu–Fan theorem).
pub fn sdp_duality_ty() -> Expr {
    prop()
}
/// `SdpOptimality : List (List (List Real)) -> List Real -> List Real -> Real -> Prop`
/// SDP optimality certificate: primal X and dual y satisfy complementarity.
pub fn sdp_optimality_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    let mats = list_ty(mat);
    let lr = list_ty(real_ty());
    arrow(mats, arrow(lr.clone(), arrow(lr, arrow(real_ty(), prop()))))
}
/// `SecondOrderCone : List Real -> Real -> Prop`
/// (x, t) ∈ SOC iff ‖x‖ ≤ t.
pub fn second_order_cone_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(lr, arrow(real_ty(), prop()))
}
/// `SocpFeasibility : Prop`
/// SOCP feasibility: minimize c^T x subject to second-order cone constraints.
pub fn socp_feasibility_ty() -> Expr {
    prop()
}
/// `Monomial : List Real -> List Real -> Real -> Real`
/// Monomial c · ∏ x_i^{a_i} in geometric programming.
pub fn monomial_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(lr.clone(), arrow(lr, arrow(real_ty(), real_ty())))
}
/// `Posynomial : List (List Real -> Real) -> List Real -> Real`
/// Posynomial: sum of monomials evaluated at x.
pub fn posynomial_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let lr = list_ty(real_ty());
    arrow(list_ty(rn_to_r), arrow(lr, real_ty()))
}
/// `GeometricProgramDuality : Prop`
/// GP can be converted to convex form via log transformation.
pub fn geometric_program_duality_ty() -> Expr {
    prop()
}
/// `SmoothGradientConvergence : Real -> Real -> Nat -> Prop`
/// GD with step 1/L: f(x_k) - f* ≤ L‖x_0-x*‖²/(2k) for L-smooth f.
pub fn smooth_gradient_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `StronglyConvexConvergence : Real -> Real -> Nat -> Prop`
/// GD converges linearly for μ-strongly convex L-smooth functions.
pub fn strongly_convex_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `ProximalOperator : (List Real -> Real) -> Real -> List Real -> List Real`
/// prox_{t·f}(v) = argmin_x { f(x) + 1/(2t) ‖x - v‖² }.
pub fn proximal_operator_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let lr = list_ty(real_ty());
    arrow(rn_to_r, arrow(real_ty(), fn_ty(lr.clone(), lr)))
}
/// `IstaConvergence : Real -> Real -> Nat -> Prop`
/// ISTA (proximal gradient): f(x_k) - f* ≤ ‖x_0-x*‖²/(2tk).
pub fn ista_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `FistaConvergence : Real -> Nat -> Prop`
/// FISTA (accelerated proximal gradient): O(1/k²) convergence rate.
pub fn fista_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `ProximalGradientDescent : (List Real -> Real) -> (List Real -> Real) -> Prop`
/// Proximal gradient: minimize f(x) + g(x), f smooth, g proxable.
pub fn proximal_gradient_descent_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(rn_to_r.clone(), arrow(rn_to_r, prop()))
}
/// `DouglasRachfordSplitting : Prop`
/// DR splitting converges for sum of two maximal monotone operators.
pub fn douglas_rachford_splitting_ty() -> Expr {
    prop()
}
/// `ChambollePockAlgorithm : Real -> Real -> Nat -> Prop`
/// Chambolle-Pock primal-dual algorithm with step sizes τ, σ satisfying τσ‖K‖² < 1.
pub fn chambolle_pock_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `AugmentedLagrangian : (List Real -> Real) -> List (List Real -> Real) -> Real -> List Real -> List Real -> Real`
/// L_ρ(x, λ) = f(x) + λ^T h(x) + (ρ/2)‖h(x)‖².
pub fn augmented_lagrangian_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let list_rn_to_r = list_ty(rn_to_r.clone());
    arrow(
        rn_to_r,
        arrow(
            list_rn_to_r,
            arrow(
                real_ty(),
                arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
            ),
        ),
    )
}
/// `AdmmConvergence : Real -> Prop`
/// ADMM converges to optimal for convex problems with penalty ρ > 0.
pub fn admm_convergence_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `SupportingHyperplaneCut : (List Real -> Real) -> List Real -> List Real -> Real -> Prop`
/// At x_k: f(x) ≥ f(x_k) + g_k^T (x - x_k) for subgradient g_k.
pub fn supporting_hyperplane_cut_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let lr = list_ty(real_ty());
    arrow(
        rn_to_r,
        arrow(lr.clone(), arrow(lr, arrow(real_ty(), prop()))),
    )
}
/// `KelleyMethod : Prop`
/// Kelley's cutting-plane method converges for convex Lipschitz functions.
pub fn kelley_method_ty() -> Expr {
    prop()
}
/// `BundleMethodConvergence : Real -> Prop`
/// Bundle method for nonsmooth optimization converges with tolerance ε.
pub fn bundle_method_convergence_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `EllipsoidMethodComplexity : Real -> Real -> Nat -> Prop`
/// Ellipsoid method finds ε-optimal solution in O(n² log(R/ε)) iterations.
pub fn ellipsoid_method_complexity_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `CenterOfGravityMethod : Nat -> Prop`
/// Center-of-gravity method reduces feasible set volume by (1-1/n) per step.
pub fn center_of_gravity_method_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SubgradientInequality : (List Real -> Real) -> List Real -> List Real -> Prop`
/// g ∈ ∂f(x): f(y) ≥ f(x) + g^T (y - x) for all y.
pub fn subgradient_inequality_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let lr = list_ty(real_ty());
    arrow(rn_to_r, arrow(lr.clone(), arrow(lr, prop())))
}
/// `SubgradientMethodConvergence : Real -> Real -> Nat -> Prop`
/// Subgradient method with step t_k = R/(G√k): best f - f* ≤ RG/√k.
pub fn subgradient_method_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `PolyakStepsize : Real -> Real -> Prop`
/// Polyak step size t_k = (f(x_k) - f*)/(‖g_k‖²) gives optimal convergence.
pub fn polyak_stepsize_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `StochasticGradientDescent : Real -> Nat -> Prop`
/// SGD with step η: E[f(x_k)] - f* = O(σ/(μ√k)) for strongly convex f.
pub fn sgd_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `SvrgConvergence : Real -> Real -> Nat -> Prop`
/// SVRG: variance-reduced SGD with geometric convergence rate (1 - μη/2)^k.
pub fn svrg_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `SarahConvergence : Real -> Real -> Nat -> Prop`
/// SARAH: stochastic recursive gradient with near-optimal convergence.
pub fn sarah_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `SpiderConvergence : Real -> Real -> Nat -> Prop`
/// SPIDER: stochastic path-integrated differential estimator convergence.
pub fn spider_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `DcpAtomConvex : (List Real -> Real) -> Prop`
/// An atom is declared convex in the DCP ruleset.
pub fn dcp_atom_convex_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(rn_to_r, prop())
}
/// `DcpCompositionRule : Prop`
/// DCP composition rules: convex nondecreasing ∘ convex = convex, etc.
pub fn dcp_composition_rule_ty() -> Expr {
    prop()
}
/// `DcpVerification : (List Real -> Real) -> Prop`
/// An expression satisfies DCP rules and is thus provably convex.
pub fn dcp_verification_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(rn_to_r, prop())
}
/// `SelfConcordantBarrier : (List Real -> Real) -> Real -> Prop`
/// A function phi is nu-self-concordant: |D^3 phi(x)[h,h,h]| <= 2(D^2 phi(x)[h,h])^{3/2}.
pub fn self_concordant_barrier_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(rn_to_r, arrow(real_ty(), prop()))
}
/// `SelfConcordantComplexity : Real -> Nat -> Prop`
/// IPM with nu-self-concordant barrier terminates in O(sqrt(nu) * log(1/eps)) Newton steps.
pub fn self_concordant_complexity_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `LogarithmicBarrier : Nat -> (List Real -> Real) -> Prop`
/// The logarithmic barrier -sum log(-g_i(x)) is self-concordant with parameter m.
pub fn logarithmic_barrier_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(nat_ty(), arrow(rn_to_r, prop()))
}
/// `NewtonDecrement : (List Real -> Real) -> List Real -> Real`
/// lambda(f,x)^2 = grad f(x)^T [Hess f(x)]^{-1} grad f(x), the Newton decrement squared.
pub fn newton_decrement_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let lr = list_ty(real_ty());
    arrow(rn_to_r, arrow(lr, real_ty()))
}
/// `SdpSlaterCondition : Prop`
/// SDP Slater: there exists strictly feasible X > 0 satisfying primal constraints.
pub fn sdp_slater_condition_ty() -> Expr {
    prop()
}
/// `SdpComplementarity : List (List Real) -> List (List Real) -> Prop`
/// SDP complementarity: X >= 0, S >= 0, and XS = 0 at optimality.
pub fn sdp_complementarity_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    arrow(mat.clone(), arrow(mat, prop()))
}
/// `SdpDualityGap : List (List Real) -> List Real -> Real`
/// Duality gap = tr(C X) - b^T y for primal X, dual y.
pub fn sdp_duality_gap_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    let lr = list_ty(real_ty());
    arrow(mat, arrow(lr, real_ty()))
}
/// `LorentzCone : Nat -> Prop`
/// The Lorentz (ice cream) cone L^n = {(x,t) : ||x|| <= t} in R^{n+1}.
pub fn lorentz_cone_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SocpDuality : Prop`
/// SOCP strong duality: primal optimal = dual optimal under Slater's condition.
pub fn socp_duality_ty() -> Expr {
    prop()
}
/// `RotatedLorentzCone : List Real -> Real -> Real -> Prop`
/// Rotated Lorentz cone: (x,y,z) with ||x||^2 <= 2yz, y,z >= 0.
pub fn rotated_lorentz_cone_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(lr, arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `AdmmLinearConvergence : Real -> Real -> Nat -> Prop`
/// ADMM with penalty rho converges linearly: ||x^k - x*|| <= C * r^k.
pub fn admm_linear_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `AdmmPrimalResidual : List Real -> List Real -> Real -> Prop`
/// Primal residual ||Ax + Bz - c|| <= eps_pri at iteration k.
pub fn admm_primal_residual_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(lr.clone(), arrow(lr, arrow(real_ty(), prop())))
}
/// `AdmmDualResidual : List Real -> Real -> Prop`
/// Dual residual rho * ||B^T(z^k - z^{k-1})|| <= eps_dual.
pub fn admm_dual_residual_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(lr, arrow(real_ty(), prop()))
}
/// `ProximalPointAlgorithm : (List Real -> Real) -> Real -> Nat -> Prop`
/// PPA: x^{k+1} = prox_{t f}(x^k) converges to argmin f.
pub fn proximal_point_algorithm_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(rn_to_r, arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `ResolventOperator : (List Real -> List Real -> Prop) -> Real -> List Real -> List Real`
/// Resolvent J_{t A}(x) = (I + tA)^{-1}(x) of a monotone operator A.
pub fn resolvent_operator_ty() -> Expr {
    let rn_rn_prop = fn_ty(list_ty(real_ty()), fn_ty(list_ty(real_ty()), prop()));
    let lr = list_ty(real_ty());
    arrow(rn_rn_prop, arrow(real_ty(), fn_ty(lr.clone(), lr)))
}
/// `FirmlyNonexpansive : (List Real -> List Real) -> Prop`
/// T is firmly nonexpansive: ||Tx - Ty||^2 <= <Tx - Ty, x - y>.
pub fn firmly_nonexpansive_ty() -> Expr {
    let lr = list_ty(real_ty());
    let rn_to_rn = fn_ty(lr.clone(), lr);
    arrow(rn_to_rn, prop())
}
/// `BregmanDivergence : (List Real -> Real) -> List Real -> List Real -> Real`
/// D_h(x,y) = h(x) - h(y) - <grad h(y), x-y> for strictly convex h.
pub fn bregman_divergence_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    let lr = list_ty(real_ty());
    arrow(rn_to_r, arrow(lr.clone(), arrow(lr, real_ty())))
}
/// `MirrorDescentConvergence : Real -> Real -> Nat -> Prop`
/// Mirror descent: sum_{t=1}^{T} f(x_t) - f(x*) <= D_h(x*,x_1)/eta + eta*sum||grad f||^2_*.
pub fn mirror_descent_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `NegativeEntropyFunction : List Real -> Real`
/// Negative entropy: h(x) = sum x_i log(x_i), used as Bregman generating function.
pub fn negative_entropy_function_ty() -> Expr {
    let lr = list_ty(real_ty());
    fn_ty(lr, real_ty())
}
/// `ExponentialWeightsAlgorithm : Real -> Nat -> Prop`
/// EWA/Hedge: mirror descent with negative entropy achieves O(sqrt(T log n)) regret.
pub fn exponential_weights_algorithm_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `SagaConvergence : Real -> Real -> Nat -> Prop`
/// SAGA: variance-reduced SGD with O(1/k) convergence for non-strongly-convex.
pub fn saga_convergence_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `AdamConvergence : Real -> Real -> Real -> Nat -> Prop`
/// Adam optimizer: adaptive moment estimation with O(1/sqrt(T)) regret bound.
pub fn adam_convergence_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop()))),
    )
}
/// `MomentumSgd : Real -> Real -> Nat -> Prop`
/// SGD with momentum (heavy ball): beta-momentum improves convergence constant.
pub fn momentum_sgd_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `MaximalMonotoneOperator : (List Real -> List (List Real)) -> Prop`
/// A set-valued operator A: R^n -> 2^{R^n} is maximal monotone.
pub fn maximal_monotone_operator_ty() -> Expr {
    let lr = list_ty(real_ty());
    let set_valued = fn_ty(lr.clone(), list_ty(lr));
    arrow(set_valued, prop())
}
/// `MonotoneInclusionProblem : (List Real -> List (List Real)) -> List Real -> Prop`
/// Find x* such that 0 in A(x*) for maximal monotone operator A.
pub fn monotone_inclusion_problem_ty() -> Expr {
    let lr = list_ty(real_ty());
    let set_valued = fn_ty(lr.clone(), list_ty(lr.clone()));
    arrow(set_valued, arrow(lr, prop()))
}
/// `SplittingConvergence : Prop`
/// Operator splitting methods (DR, FBS, etc.) converge for sum of monotone operators.
pub fn splitting_convergence_ty() -> Expr {
    prop()
}
/// `KantorovichProblem : (List Real -> List Real -> Real) -> List Real -> List Real -> Real`
/// Kantorovich: min_{gamma in Pi(mu,nu)} integral c(x,y) d_gamma(x,y) over transport plans.
pub fn kantorovich_problem_ty() -> Expr {
    let lr = list_ty(real_ty());
    let cost = fn_ty(lr.clone(), fn_ty(lr.clone(), real_ty()));
    arrow(cost, arrow(lr.clone(), arrow(lr, real_ty())))
}
/// `KantorovichDuality : Prop`
/// min_{gamma} integral c d_gamma = max_{phi,psi: phi(x)+psi(y)<=c(x,y)} integral phi d_mu + integral psi d_nu.
pub fn kantorovich_duality_ty() -> Expr {
    prop()
}
/// `WassersteinDistance : Real -> List Real -> List Real -> Real`
/// W_p(mu,nu) = (min_{gamma in Pi(mu,nu)} integral ||x-y||^p d_gamma)^{1/p}.
pub fn wasserstein_distance_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(real_ty(), arrow(lr.clone(), arrow(lr, real_ty())))
}
/// `SinkhornAlgorithm : Real -> Nat -> Prop`
/// Sinkhorn: entropic regularization gives O(n^2/eps^2) algorithm for OT.
pub fn sinkhorn_algorithm_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `RestrictedIsometryProperty : Nat -> Real -> List (List Real) -> Prop`
/// RIP-s: (1-delta)||x||^2 <= ||Ax||^2 <= (1+delta)||x||^2 for all s-sparse x.
pub fn restricted_isometry_property_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    arrow(nat_ty(), arrow(real_ty(), arrow(mat, prop())))
}
/// `BasisPursuitRecovery : List (List Real) -> List Real -> Nat -> Prop`
/// If A satisfies RIP-{2s}, basis pursuit recovers s-sparse x from Ax = b.
pub fn basis_pursuit_recovery_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    let lr = list_ty(real_ty());
    arrow(mat, arrow(lr, arrow(nat_ty(), prop())))
}
/// `LassoSparsity : Real -> Nat -> Prop`
/// LASSO with appropriate lambda recovers s-sparse signal with O(s log p / n) samples.
pub fn lasso_sparsity_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `NuclearNorm : List (List Real) -> Real`
/// ||X||_* = sum sigma_i(X), the nuclear norm (sum of singular values).
pub fn nuclear_norm_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    fn_ty(mat, real_ty())
}
/// `MatrixCompletionTheorem : Nat -> Nat -> Nat -> Real -> Prop`
/// If rank-r matrix M satisfies incoherence, nuclear norm minimization recovers M
/// from O(r n log n) entries.
pub fn matrix_completion_theorem_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(real_ty(), prop()))),
    )
}
/// `RobustPca : List (List Real) -> List (List Real) -> List (List Real) -> Prop`
/// Robust PCA: decompose M = L + S, L low-rank, S sparse, via PCP convex program.
pub fn robust_pca_ty() -> Expr {
    let mat = list_ty(list_ty(real_ty()));
    arrow(mat.clone(), arrow(mat.clone(), arrow(mat, prop())))
}
/// `ChanceConstraint : (List Real -> Prop) -> Real -> Prop`
/// P(g(x) <= 0) >= 1 - eps: probabilistic constraint with confidence 1-eps.
pub fn chance_constraint_ty() -> Expr {
    let rn_prop = fn_ty(list_ty(real_ty()), prop());
    arrow(rn_prop, arrow(real_ty(), prop()))
}
/// `DistributionallyRobustObjective : (List Real -> Real) -> Real -> Prop`
/// DRO: min_x max_{P in U} E_P[f(x,xi)] over ambiguity set U.
pub fn distributionally_robust_objective_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(rn_to_r, arrow(real_ty(), prop()))
}
/// `WassersteinAmbiguitySet : List Real -> Real -> Prop`
/// Wasserstein ball of radius rho around empirical distribution P_N.
pub fn wasserstein_ambiguity_set_ty() -> Expr {
    let lr = list_ty(real_ty());
    arrow(lr, arrow(real_ty(), prop()))
}
/// `CvarConstraint : (List Real -> Real) -> Real -> Real -> Prop`
/// Conditional value-at-risk CVaR_alpha(f(x,xi)) <= t is a convex constraint in x, t.
pub fn cvar_constraint_ty() -> Expr {
    let rn_to_r = fn_ty(list_ty(real_ty()), real_ty());
    arrow(rn_to_r, arrow(real_ty(), arrow(real_ty(), prop())))
}
/// `OnlineConvexOptimization : Nat -> Real -> Prop`
/// OCO regret bound: sum_{t=1}^T f_t(x_t) - min_x sum f_t(x) <= R_T.
pub fn online_convex_optimization_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `FtrlRegretBound : Real -> Nat -> Prop`
/// FTRL achieves O(sqrt(T)) regret for convex losses with regularizer.
pub fn ftrl_regret_bound_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// `AdaptiveRegretBound : Real -> Real -> Nat -> Prop`
/// Adaptive online algorithms achieve tighter regret via local norms.
pub fn adaptive_regret_bound_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// `OnlineGradientDescentRegret : Real -> Nat -> Prop`
/// OGD regret: O(G*R*sqrt(T)) where G is gradient bound, R is domain diameter.
pub fn online_gradient_descent_regret_ty() -> Expr {
    arrow(real_ty(), arrow(nat_ty(), prop()))
}
/// Build an [`Environment`] containing convex optimization axioms and theorems.
pub fn build_convex_optimization_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("ConvexSet", convex_set_ty()),
        ("ConvexFunction", convex_function_ty()),
        ("KktConditions", kkt_conditions_ty()),
        ("Lagrangian", lagrangian_ty()),
        ("StrongDuality", strong_duality_ty()),
        ("projection_theorem", projection_theorem_ty()),
        ("supporting_hyperplane", supporting_hyperplane_ty()),
        ("jensen_inequality", jensen_inequality_ty()),
        ("slater_condition", slater_condition_ty()),
        ("ConvexProgram", prop()),
        ("DualProgram", prop()),
        ("OptimalityGap", arrow(real_ty(), prop())),
        ("FenchelConjugate", fenchel_conjugate_ty()),
        (
            "FenchelRockafellarDuality",
            fenchel_rockafellar_duality_ty(),
        ),
        ("ConjugateSubgradient", conjugate_subgradient_ty()),
        ("LagrangianDualFunction", lagrangian_dual_function_ty()),
        ("LinearIndependenceCQ", licq_ty()),
        ("MangasarianFromovitzCQ", mfcq_ty()),
        ("ComplementarySlackness", complementary_slackness_ty()),
        ("KktSufficiency", kkt_sufficiency_ty()),
        ("BarrierFunction", barrier_function_ty()),
        ("PathFollowingMethod", path_following_method_ty()),
        ("PredictorCorrectorMethod", predictor_corrector_method_ty()),
        ("CentralPathConvergence", central_path_convergence_ty()),
        ("PositiveSemidefinite", positive_semidefinite_ty()),
        ("SdpFeasibility", sdp_feasibility_ty()),
        ("SdpDuality", sdp_duality_ty()),
        ("SdpOptimality", sdp_optimality_ty()),
        ("SecondOrderCone", second_order_cone_ty()),
        ("SocpFeasibility", socp_feasibility_ty()),
        ("Monomial", monomial_ty()),
        ("Posynomial", posynomial_ty()),
        ("GeometricProgramDuality", geometric_program_duality_ty()),
        (
            "SmoothGradientConvergence",
            smooth_gradient_convergence_ty(),
        ),
        (
            "StronglyConvexConvergence",
            strongly_convex_convergence_ty(),
        ),
        ("ProximalOperator", proximal_operator_ty()),
        ("IstaConvergence", ista_convergence_ty()),
        ("FistaConvergence", fista_convergence_ty()),
        ("ProximalGradientDescent", proximal_gradient_descent_ty()),
        ("DouglasRachfordSplitting", douglas_rachford_splitting_ty()),
        ("ChambollePockAlgorithm", chambolle_pock_ty()),
        ("AugmentedLagrangian", augmented_lagrangian_ty()),
        ("AdmmConvergence", admm_convergence_ty()),
        ("SupportingHyperplaneCut", supporting_hyperplane_cut_ty()),
        ("KelleyMethod", kelley_method_ty()),
        ("BundleMethodConvergence", bundle_method_convergence_ty()),
        (
            "EllipsoidMethodComplexity",
            ellipsoid_method_complexity_ty(),
        ),
        ("CenterOfGravityMethod", center_of_gravity_method_ty()),
        ("SubgradientInequality", subgradient_inequality_ty()),
        (
            "SubgradientMethodConvergence",
            subgradient_method_convergence_ty(),
        ),
        ("PolyakStepsize", polyak_stepsize_ty()),
        ("SgdConvergence", sgd_convergence_ty()),
        ("SvrgConvergence", svrg_convergence_ty()),
        ("SarahConvergence", sarah_convergence_ty()),
        ("SpiderConvergence", spider_convergence_ty()),
        ("DcpAtomConvex", dcp_atom_convex_ty()),
        ("DcpCompositionRule", dcp_composition_rule_ty()),
        ("DcpVerification", dcp_verification_ty()),
        ("SelfConcordantBarrier", self_concordant_barrier_ty()),
        ("SelfConcordantComplexity", self_concordant_complexity_ty()),
        ("LogarithmicBarrier", logarithmic_barrier_ty()),
        ("NewtonDecrement", newton_decrement_ty()),
        ("SdpSlaterCondition", sdp_slater_condition_ty()),
        ("SdpComplementarity", sdp_complementarity_ty()),
        ("SdpDualityGap", sdp_duality_gap_ty()),
        ("LorentzCone", lorentz_cone_ty()),
        ("SocpDuality", socp_duality_ty()),
        ("RotatedLorentzCone", rotated_lorentz_cone_ty()),
        ("AdmmLinearConvergence", admm_linear_convergence_ty()),
        ("AdmmPrimalResidual", admm_primal_residual_ty()),
        ("AdmmDualResidual", admm_dual_residual_ty()),
        ("ProximalPointAlgorithm", proximal_point_algorithm_ty()),
        ("ResolventOperator", resolvent_operator_ty()),
        ("FirmlyNonexpansive", firmly_nonexpansive_ty()),
        ("BregmanDivergence", bregman_divergence_ty()),
        ("MirrorDescentConvergence", mirror_descent_convergence_ty()),
        ("NegativeEntropyFunction", negative_entropy_function_ty()),
        (
            "ExponentialWeightsAlgorithm",
            exponential_weights_algorithm_ty(),
        ),
        ("SagaConvergence", saga_convergence_ty()),
        ("AdamConvergence", adam_convergence_ty()),
        ("MomentumSgd", momentum_sgd_ty()),
        ("MaximalMonotoneOperator", maximal_monotone_operator_ty()),
        ("MonotoneInclusionProblem", monotone_inclusion_problem_ty()),
        ("SplittingConvergence", splitting_convergence_ty()),
        ("KantorovichProblem", kantorovich_problem_ty()),
        ("KantorovichDuality", kantorovich_duality_ty()),
        ("WassersteinDistance", wasserstein_distance_ty()),
        ("SinkhornAlgorithm", sinkhorn_algorithm_ty()),
        (
            "RestrictedIsometryProperty",
            restricted_isometry_property_ty(),
        ),
        ("BasisPursuitRecovery", basis_pursuit_recovery_ty()),
        ("LassoSparsity", lasso_sparsity_ty()),
        ("NuclearNorm", nuclear_norm_ty()),
        ("MatrixCompletionTheorem", matrix_completion_theorem_ty()),
        ("RobustPca", robust_pca_ty()),
        ("ChanceConstraint", chance_constraint_ty()),
        (
            "DistributionallyRobustObjective",
            distributionally_robust_objective_ty(),
        ),
        ("WassersteinAmbiguitySet", wasserstein_ambiguity_set_ty()),
        ("CvarConstraint", cvar_constraint_ty()),
        ("OnlineConvexOptimization", online_convex_optimization_ty()),
        ("FtrlRegretBound", ftrl_regret_bound_ty()),
        ("AdaptiveRegretBound", adaptive_regret_bound_ty()),
        (
            "OnlineGradientDescentRegret",
            online_gradient_descent_regret_ty(),
        ),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// Trait for convex functions over ℝ^n.
pub trait ConvexFunction {
    /// Evaluate f(x).
    fn eval(&self, x: &[f64]) -> f64;
    /// Compute the gradient ∇f(x).
    fn gradient(&self, x: &[f64]) -> Vec<f64>;
    /// Return `true` if f is strongly convex.
    fn is_strongly_convex(&self) -> bool;
}
/// Trait for functions with a computable proximal operator.
pub trait ProxableFunction: ConvexFunction {
    /// prox_{t·self}(v) = argmin_x { self(x) + 1/(2t) ‖x-v‖² }.
    fn prox(&self, v: &[f64], t: f64) -> Vec<f64>;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quadratic_eval_origin() {
        let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let f = QuadraticFunction::new(q, vec![0.0, 0.0], 0.0);
        assert!((f.eval(&[0.0, 0.0])).abs() < 1e-12);
    }
    #[test]
    fn test_quadratic_eval_nonzero() {
        let q = vec![vec![2.0]];
        let f = QuadraticFunction::new(q, vec![0.0], 0.0);
        assert!((f.eval(&[2.0]) - 4.0).abs() < 1e-9);
    }
    #[test]
    fn test_quadratic_gradient() {
        let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let f = QuadraticFunction::new(q, vec![0.0, 0.0], 0.0);
        let grad = f.gradient(&[3.0, -1.0]);
        assert!((grad[0] - 3.0).abs() < 1e-9);
        assert!((grad[1] + 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_strongly_convex() {
        let q = vec![vec![2.0, 0.0], vec![0.0, 3.0]];
        let f = QuadraticFunction::new(q, vec![0.0, 0.0], 0.0);
        assert!(f.is_strongly_convex());
    }
    #[test]
    fn test_gradient_descent_minimizes_quadratic() {
        let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let f = QuadraticFunction::new(q, vec![0.0, 0.0], 0.0);
        let gd = GradientDescent::new(0.1, 500, 1e-6);
        let (x, fval, _iters) = gd.minimize(&f, &[5.0, -3.0]);
        assert!(fval < 1e-6, "fval={fval}");
        assert!(x[0].abs() < 1e-3);
        assert!(x[1].abs() < 1e-3);
    }
    #[test]
    fn test_projected_gradient_box_constraint() {
        let q = vec![vec![1.0]];
        let f = QuadraticFunction::new(q, vec![0.0], 0.0);
        let pg = ProjectedGradient::new(0.1, 300, 1e-6, vec![1.0], vec![5.0]);
        let (x, fval) = pg.minimize(&f, &[4.0]);
        assert!((x[0] - 1.0).abs() < 1e-3, "x={}", x[0]);
        assert!((fval - 0.5).abs() < 1e-3, "fval={fval}");
    }
    #[test]
    fn test_admm_solve_lasso_stub() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![1.0, 2.0];
        let admm = ADMM::new(1.0);
        let x = admm.solve_lasso(&a, &b, 0.1);
        assert_eq!(x.len(), 2);
        assert_eq!(x, vec![0.0, 0.0]);
    }
    #[test]
    fn test_build_convex_optimization_env() {
        let env = build_convex_optimization_env();
        assert!(env.get(&Name::str("ConvexSet")).is_some());
        assert!(env.get(&Name::str("ConvexFunction")).is_some());
        assert!(env.get(&Name::str("KktConditions")).is_some());
        assert!(env.get(&Name::str("projection_theorem")).is_some());
        assert!(env.get(&Name::str("jensen_inequality")).is_some());
        assert!(env.get(&Name::str("FenchelConjugate")).is_some());
        assert!(env.get(&Name::str("FenchelRockafellarDuality")).is_some());
        assert!(env.get(&Name::str("PositiveSemidefinite")).is_some());
        assert!(env.get(&Name::str("SdpDuality")).is_some());
        assert!(env.get(&Name::str("FistaConvergence")).is_some());
        assert!(env.get(&Name::str("ProximalOperator")).is_some());
        assert!(env.get(&Name::str("DouglasRachfordSplitting")).is_some());
        assert!(env.get(&Name::str("ChambollePockAlgorithm")).is_some());
        assert!(env.get(&Name::str("EllipsoidMethodComplexity")).is_some());
        assert!(env.get(&Name::str("SvrgConvergence")).is_some());
        assert!(env.get(&Name::str("SpiderConvergence")).is_some());
        assert!(env.get(&Name::str("DcpVerification")).is_some());
    }
    #[test]
    fn test_l1_norm_prox_soft_threshold() {
        let g = L1NormFunction::new(1.0);
        let result = g.prox(&[3.0, -0.5], 1.0);
        assert!((result[0] - 2.0).abs() < 1e-12, "result[0]={}", result[0]);
        assert!(result[1].abs() < 1e-12, "result[1]={}", result[1]);
    }
    #[test]
    fn test_fista_minimizes_smooth_quadratic() {
        let smooth = QuadraticFunction::new(vec![vec![1.0]], vec![0.0], 0.0);
        let reg = L1NormFunction::new(0.0);
        let solver = FISTASolver::new(1.0, 200, 1e-6);
        let (x, _iters) = solver.minimize(&smooth, &reg, &[5.0]);
        assert!(x[0].abs() < 1e-3, "x[0]={}", x[0]);
    }
    #[test]
    fn test_sdp_relaxation_psd_check() {
        let id = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        assert!(SDPRelaxation::is_psd(&id));
        let neg = vec![vec![-1.0, 0.0], vec![0.0, 1.0]];
        assert!(!SDPRelaxation::is_psd(&neg));
    }
    #[test]
    fn test_sdp_relaxation_solve_returns_bound() {
        let q = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
        let c = vec![0.0, 0.0];
        let a = vec![vec![1.0, 0.0]];
        let b = vec![1.0];
        let sdp = SDPRelaxation::new(q, c, a, b);
        let bound = sdp.solve();
        assert!((bound - 0.0).abs() < 1e-12);
    }
    #[test]
    fn test_geometric_program_log_sum_exp() {
        let monomials = vec![(0.0_f64, vec![1.0_f64])];
        let lse = GeometricProgramSolver::log_sum_exp_posynomial(&monomials, &[1.0]);
        assert!((lse - 1.0).abs() < 1e-9, "lse={lse}");
    }
    #[test]
    fn test_cutting_plane_minimizes_quadratic() {
        let f = QuadraticFunction::new(vec![vec![2.0]], vec![0.0], 0.0);
        let solver = CuttingPlaneSolver::new(100, 1e-4, 2.0);
        let (x, fval, _iters) = solver.minimize(&f, &[3.0]);
        assert!(fval < 1.0, "fval={fval}");
        let _ = x;
    }
    #[test]
    fn test_bundle_method_minimizes_quadratic() {
        let f = QuadraticFunction::new(vec![vec![2.0]], vec![0.0], 0.0);
        let solver = BundleMethodSolver::new(1.0, 0.5, 20, 200, 1e-5);
        let (x, fval, _iters) = solver.minimize(&f, &[4.0]);
        assert!(fval < 1.0, "fval={fval}");
        let _ = x;
    }
    #[test]
    fn test_new_axioms_in_env() {
        let env = build_convex_optimization_env();
        assert!(env.get(&Name::str("SelfConcordantBarrier")).is_some());
        assert!(env.get(&Name::str("SelfConcordantComplexity")).is_some());
        assert!(env.get(&Name::str("LogarithmicBarrier")).is_some());
        assert!(env.get(&Name::str("NewtonDecrement")).is_some());
        assert!(env.get(&Name::str("SdpSlaterCondition")).is_some());
        assert!(env.get(&Name::str("SdpComplementarity")).is_some());
        assert!(env.get(&Name::str("SdpDualityGap")).is_some());
        assert!(env.get(&Name::str("LorentzCone")).is_some());
        assert!(env.get(&Name::str("SocpDuality")).is_some());
        assert!(env.get(&Name::str("RotatedLorentzCone")).is_some());
        assert!(env.get(&Name::str("AdmmLinearConvergence")).is_some());
        assert!(env.get(&Name::str("AdmmPrimalResidual")).is_some());
        assert!(env.get(&Name::str("AdmmDualResidual")).is_some());
        assert!(env.get(&Name::str("ProximalPointAlgorithm")).is_some());
        assert!(env.get(&Name::str("ResolventOperator")).is_some());
        assert!(env.get(&Name::str("FirmlyNonexpansive")).is_some());
        assert!(env.get(&Name::str("BregmanDivergence")).is_some());
        assert!(env.get(&Name::str("MirrorDescentConvergence")).is_some());
        assert!(env.get(&Name::str("NegativeEntropyFunction")).is_some());
        assert!(env.get(&Name::str("ExponentialWeightsAlgorithm")).is_some());
        assert!(env.get(&Name::str("SagaConvergence")).is_some());
        assert!(env.get(&Name::str("AdamConvergence")).is_some());
        assert!(env.get(&Name::str("MomentumSgd")).is_some());
        assert!(env.get(&Name::str("MaximalMonotoneOperator")).is_some());
        assert!(env.get(&Name::str("MonotoneInclusionProblem")).is_some());
        assert!(env.get(&Name::str("SplittingConvergence")).is_some());
        assert!(env.get(&Name::str("KantorovichProblem")).is_some());
        assert!(env.get(&Name::str("KantorovichDuality")).is_some());
        assert!(env.get(&Name::str("WassersteinDistance")).is_some());
        assert!(env.get(&Name::str("SinkhornAlgorithm")).is_some());
        assert!(env.get(&Name::str("RestrictedIsometryProperty")).is_some());
        assert!(env.get(&Name::str("BasisPursuitRecovery")).is_some());
        assert!(env.get(&Name::str("LassoSparsity")).is_some());
        assert!(env.get(&Name::str("NuclearNorm")).is_some());
        assert!(env.get(&Name::str("MatrixCompletionTheorem")).is_some());
        assert!(env.get(&Name::str("RobustPca")).is_some());
        assert!(env.get(&Name::str("ChanceConstraint")).is_some());
        assert!(env
            .get(&Name::str("DistributionallyRobustObjective"))
            .is_some());
        assert!(env.get(&Name::str("WassersteinAmbiguitySet")).is_some());
        assert!(env.get(&Name::str("CvarConstraint")).is_some());
        assert!(env.get(&Name::str("OnlineConvexOptimization")).is_some());
        assert!(env.get(&Name::str("FtrlRegretBound")).is_some());
        assert!(env.get(&Name::str("AdaptiveRegretBound")).is_some());
        assert!(env.get(&Name::str("OnlineGradientDescentRegret")).is_some());
    }
    #[test]
    fn test_mirror_descent_project_simplex() {
        let v = vec![0.5, 0.5];
        let p = MirrorDescentSolver::project_simplex(&v);
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert!(p.iter().all(|&x| x >= 0.0));
    }
    #[test]
    fn test_mirror_descent_simplex_sum_to_one() {
        let v = vec![3.0, -1.0, 2.0];
        let p = MirrorDescentSolver::project_simplex(&v);
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert!(p.iter().all(|&x| x >= 0.0));
    }
    #[test]
    fn test_mirror_descent_bregman_kl_zero() {
        let x = vec![0.25, 0.25, 0.5];
        let kl = MirrorDescentSolver::bregman_kl(&x, &x);
        assert!(kl.abs() < 1e-10, "kl={kl}");
    }
    #[test]
    fn test_mirror_descent_minimizes_quadratic() {
        let f = QuadraticFunction::new(vec![vec![1.0]], vec![0.0], 0.0);
        let solver = MirrorDescentSolver::new(0.05, 500, 1e-6, false);
        let (x, fval, _iters) = solver.minimize(&f, &[3.0]);
        assert!(fval < 0.1, "fval={fval}");
        let _ = x;
    }
    #[test]
    fn test_proximal_gradient_ista_minimizes() {
        let smooth = QuadraticFunction::new(vec![vec![2.0]], vec![0.0], 0.0);
        let reg = L1NormFunction::new(0.0);
        let solver = ProximalGradientSolver::new(2.0, 300, 1e-7, false);
        let (x, iters) = solver.minimize(&smooth, &reg, &[5.0]);
        assert!(x[0].abs() < 0.01, "x[0]={}", x[0]);
        let _ = iters;
    }
    #[test]
    fn test_proximal_gradient_fista_accelerated() {
        let smooth = QuadraticFunction::new(vec![vec![2.0]], vec![0.0], 0.0);
        let reg = L1NormFunction::new(0.0);
        let solver_fista = ProximalGradientSolver::new(2.0, 200, 1e-7, true);
        let solver_ista = ProximalGradientSolver::new(2.0, 200, 1e-7, false);
        let (x_fista, iters_fista) = solver_fista.minimize(&smooth, &reg, &[5.0]);
        let (x_ista, iters_ista) = solver_ista.minimize(&smooth, &reg, &[5.0]);
        assert!(
            iters_fista <= iters_ista + 10,
            "fista={iters_fista}, ista={iters_ista}"
        );
        assert!(x_fista[0].abs() < 0.01, "x_fista={}", x_fista[0]);
        let _ = x_ista;
    }
    #[test]
    fn test_proximal_gradient_estimate_lipschitz() {
        let f = QuadraticFunction::new(vec![vec![2.0]], vec![0.0], 0.0);
        let l_est = ProximalGradientSolver::estimate_lipschitz(&f, &[1.0], 1);
        assert!(l_est > 0.5, "L_est={l_est}");
    }
    #[test]
    fn test_sinkhorn_uniform_transport() {
        let mu = vec![0.5, 0.5];
        let nu = vec![0.5, 0.5];
        let cost = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
        let solver = SinkhornSolver::new(0.1, 200, 1e-8);
        let (gamma, w) = solver.solve(&mu, &nu, &cost);
        let total: f64 = gamma.iter().flat_map(|r| r.iter()).sum();
        assert!((total - 1.0).abs() < 1e-4, "total={total}");
        assert!(w >= 0.0, "w={w}");
    }
    #[test]
    fn test_sinkhorn_same_distribution() {
        let mu = vec![0.5, 0.5];
        let nu = vec![0.5, 0.5];
        let cost = vec![vec![0.0, 1.0], vec![1.0, 0.0]];
        let solver = SinkhornSolver::new(0.01, 500, 1e-10);
        let (_gamma, w) = solver.solve(&mu, &nu, &cost);
        assert!(w < 0.6, "w={w}");
    }
    #[test]
    fn test_sinkhorn_wasserstein2_1d_zero() {
        let pts = vec![0.0, 1.0];
        let wts = vec![0.5, 0.5];
        let w2 = SinkhornSolver::wasserstein2_1d(&pts, &wts, &pts, &wts);
        assert!(w2 < 0.2, "w2={w2}");
    }
    #[test]
    fn test_rip_identity_satisfies_rip() {
        let id = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let verifier = RipVerifier::new(1, 10);
        let (dl, du) = verifier.estimate_rip_constant(&id);
        assert!(du < 0.01, "delta_upper={du}");
        let _ = dl;
    }
    #[test]
    fn test_rip_soft_threshold() {
        let x = vec![3.0, -0.5, 0.2];
        let result = RipVerifier::soft_threshold(&x, 1.0);
        assert!((result[0] - 2.0).abs() < 1e-12);
        assert!(result[1].abs() < 1e-12);
        assert!(result[2].abs() < 1e-12);
    }
    #[test]
    fn test_rip_satisfies_rip_check() {
        let id = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let verifier = RipVerifier::new(1, 5);
        assert!(verifier.satisfies_rip(&id, 0.5));
    }
    #[test]
    fn test_online_learner_initial_decision() {
        let learner = OnlineLearner::new(0.1, 3);
        let x = learner.current_decision();
        assert_eq!(x.len(), 3);
        assert!(x.iter().all(|&xi| xi.abs() < 1e-12));
    }
    #[test]
    fn test_online_learner_update_accumulates() {
        let mut learner = OnlineLearner::new(0.1, 2);
        learner.update(&[1.0, 0.0]);
        let x = learner.current_decision();
        assert!((x[0] + 0.1).abs() < 1e-9, "x[0]={}", x[0]);
        assert!(x[1].abs() < 1e-9, "x[1]={}", x[1]);
    }
    #[test]
    fn test_online_learner_regret_bound_positive() {
        let mut learner = OnlineLearner::new(0.01, 2);
        for _ in 0..10 {
            learner.update(&[0.5, -0.5]);
        }
        let bound = learner.regret_bound(1.0, 1.0);
        assert!(bound >= 0.0, "bound={bound}");
    }
    #[test]
    fn test_online_learner_reset() {
        let mut learner = OnlineLearner::new(0.1, 2);
        learner.update(&[1.0, 2.0]);
        learner.reset();
        assert_eq!(learner.round, 0);
        let x = learner.current_decision();
        assert!(x.iter().all(|&xi| xi.abs() < 1e-12));
    }
}
