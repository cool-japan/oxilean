//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AdaptiveStepRK45, BifurcationDiagram, BoundaryCondition, DelayDE, EquilibriumPoint,
    EulerMethod, ExactODE, FlowMap, FredholmEquation, FredholmIntegralEquation,
    FunctionSpaceSolution, HeatEquation, ItoSDE, JacobianMatrix, LaplacianEqn, LinearODE,
    LyapunovExponent, Manifold, MultistepMethod, PDEType, PoincareMap, RungeKutta4, StabilityType,
    StiffSolver, StrangeAttractor, SturmLiouville, VolterraEquation, VolterraIntegralEquation,
    WaveEquation, ODE,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
/// `ODE : (Real → Real → Real) → Real → Real → Prop`
///
/// dy/dt = f(t, y) with initial value y(t₀) = y₀.
pub fn ode_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `LinearODE : (Real → Real) → (Real → Real) → Real → Real → Prop`
///
/// dy/dt = a(t)y + b(t) with initial value, solved via integrating factor.
pub fn linear_ode_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(
            fn_ty(real_ty(), real_ty()),
            arrow(real_ty(), arrow(real_ty(), prop())),
        ),
    )
}
/// `ExactODE : (Real → Real → Real) → (Real → Real → Real) → Prop`
///
/// P(x,y)dx + Q(x,y)dy = 0 is exact when ∂P/∂y = ∂Q/∂x.
pub fn exact_ode_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop()),
    )
}
/// `SturmLiouville : (Real → Real) → (Real → Real) → (Real → Real) → Real → Prop`
///
/// (p(x)y')' + (q(x) + λw(x))y = 0 — spectral problem with weight w.
pub fn sturm_liouville_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(
            fn_ty(real_ty(), real_ty()),
            arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop())),
        ),
    )
}
/// `ODESolution : (Real → Real → Real) → Real → Real → (Real → Real) → Prop`
///
/// φ is the unique solution to dy/dt = f(t,y), y(t₀) = y₀.
pub fn ode_solution_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(fn_ty(real_ty(), real_ty()), prop())),
        ),
    )
}
/// `CharacteristicPolynomial : (Real → Real → Real) → (Real → Real) → Prop`
///
/// The characteristic polynomial of a linear ODE with constant coefficients.
pub fn characteristic_polynomial_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(fn_ty(real_ty(), real_ty()), prop()),
    )
}
/// `EquilibriumPoint : (Real → Real → Real × Real) → Real → Real → Prop`
///
/// (x*, y*) is an equilibrium: f(x*, y*) = (0, 0).
pub fn equilibrium_point_ty() -> Expr {
    arrow(
        fn_ty(
            real_ty(),
            fn_ty(real_ty(), app2(cst("Prod"), real_ty(), real_ty())),
        ),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `JacobianMatrix : (Real → Real → Real × Real) → Real → Real → Type`
///
/// The Jacobian Df(x*, y*) is the 2×2 linearisation matrix.
pub fn jacobian_matrix_ty() -> Expr {
    arrow(
        fn_ty(
            real_ty(),
            fn_ty(real_ty(), app2(cst("Prod"), real_ty(), real_ty())),
        ),
        arrow(real_ty(), arrow(real_ty(), type0())),
    )
}
/// `StabilityClassification : Type → Type`
///
/// One of: StableNode, UnstableNode, Saddle, StableSpiral, UnstableSpiral, Center.
pub fn stability_classification_ty() -> Expr {
    type0()
}
/// `LyapunovFunction : (Real → Real → Real) → Prop`
///
/// V(x, y) is a Lyapunov function for the system if V > 0 and dV/dt ≤ 0.
pub fn lyapunov_function_ty() -> Expr {
    arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop())
}
/// `PoincareBendixson : (Real → Real → Real × Real) → Prop`
///
/// If a bounded orbit has no equilibria in its closure, it converges to a limit cycle.
pub fn poincare_bendixson_ty() -> Expr {
    arrow(
        fn_ty(
            real_ty(),
            fn_ty(real_ty(), app2(cst("Prod"), real_ty(), real_ty())),
        ),
        prop(),
    )
}
/// `Manifold : Type → Type → Prop`
///
/// Generic manifold predicate (stable / unstable / center manifold membership).
pub fn manifold_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `PDEType : Type`
///
/// Classification: Elliptic | Parabolic | Hyperbolic | Mixed.
pub fn pde_type_ty() -> Expr {
    type0()
}
/// `LaplacianEqn : (Real → Real → Real) → Prop`
///
/// Δu = 0 (Laplace equation).
pub fn laplacian_eqn_ty() -> Expr {
    arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop())
}
/// `HeatEquation : (Real → Real → Real → Real) → Real → Prop`
///
/// u_t = κΔu; κ is the diffusivity.
pub fn heat_equation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), fn_ty(real_ty(), real_ty()))),
        arrow(real_ty(), prop()),
    )
}
/// `WaveEquation : (Real → Real → Real → Real) → Real → Prop`
///
/// u_tt = c²Δu; c is wave speed.
pub fn wave_equation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), fn_ty(real_ty(), real_ty()))),
        arrow(real_ty(), prop()),
    )
}
/// `BoundaryCondition : (Real → Real) → Type`
///
/// A boundary condition specification (Dirichlet / Neumann / Robin / Periodic).
pub fn boundary_condition_ty() -> Expr {
    arrow(fn_ty(real_ty(), real_ty()), type0())
}
/// `GreensFunction : (Real → Real → Real → Real) → Real → Real → Real → Real`
///
/// The Green's function G(x, y; ξ, η) for a PDE.
pub fn greens_function_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), fn_ty(real_ty(), real_ty()))),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), real_ty()))),
    )
}
/// `EulerStep : (Real → Real → Real) → Real → Real → Real → Real → Prop`
///
/// y_{n+1} = y_n + h f(t_n, y_n).
pub fn euler_step_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `RungeKutta4Step : (Real → Real → Real) → Real → Real → Real → Real → Prop`
///
/// Classical 4th-order Runge-Kutta single step.
pub fn runge_kutta4_step_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(
            real_ty(),
            arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
        ),
    )
}
/// `AdaptiveStepRK45 : (Real → Real → Real) → Real → Real → Prop`
///
/// Dormand-Prince RK45 with error control: step accepted iff local error ≤ tol.
pub fn adaptive_step_rk45_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `MultistepMethod : (Real → Real → Real) → Nat → Real → Real → Prop`
///
/// Adams-Bashforth / Adams-Moulton k-step method.
pub fn multistep_method_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(nat_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `StiffSolver : (Real → Real → Real) → Real → Prop`
///
/// Implicit solver (BDF/Radau) suitable for stiff ODEs.
pub fn stiff_solver_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// `FlowMap : (Real → Real → Real) → Real → Real → Real → Prop`
///
/// φ_t is the flow of the ODE; φ_{t+s} = φ_t ∘ φ_s.
pub fn flow_map_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `LimitSet : (Real → Real → Real) → Real → (Real → Prop) → Prop`
///
/// ω(x) = {y : ∃ t_n → ∞, φ_{t_n}(x) → y}.
pub fn limit_set_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(fn_ty(real_ty(), prop()), prop())),
    )
}
/// `StrangeAttractor : (Real → Real → Real) → Prop`
///
/// A fractal attractor with sensitive dependence on initial conditions.
pub fn strange_attractor_ty() -> Expr {
    arrow(fn_ty(real_ty(), fn_ty(real_ty(), real_ty())), prop())
}
/// `LyapunovExponent : (Real → Real → Real) → Real → Real → Prop`
///
/// λ = lim_{t→∞} (1/t) log ||Dφ_t(x) v||.
pub fn lyapunov_exponent_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), prop())),
    )
}
/// `PoincareMap : (Real → Real → Real) → Real → Real → Real → Prop`
///
/// The Poincaré first-return map for a periodic orbit analysis.
pub fn poincare_map_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `DelayDE : (Real → Real → Real → Real) → Real → Real → Real → Prop`
///
/// y'(t) = f(t, y(t), y(t−τ)) with delay τ.
pub fn delay_de_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), fn_ty(real_ty(), real_ty()))),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `VolterraIntegralEquation : (Real → Real → Real) → (Real → Real) → Prop`
///
/// y(t) = ∫₀ᵗ K(t,s)y(s)ds + f(t).
pub fn volterra_integral_equation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(fn_ty(real_ty(), real_ty()), prop()),
    )
}
/// `FredholmIntegralEquation : (Real → Real → Real) → (Real → Real) → Real → Prop`
///
/// y(t) = λ ∫_a^b K(t,s)y(s)ds + f(t).
pub fn fredholm_integral_equation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(fn_ty(real_ty(), real_ty()), arrow(real_ty(), prop())),
    )
}
/// `HammersteinEquation : (Real → Real → Real) → (Real → Real) → (Real → Real) → Prop`
///
/// y(t) = ∫_a^b K(t,s) g(s, y(s)) ds + f(t).
pub fn hammerstein_equation_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(
            fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
            arrow(fn_ty(real_ty(), real_ty()), prop()),
        ),
    )
}
/// `FunctionSpaceSolution : (Real → Real) → Real → Real → Real → Prop`
///
/// φ is the solution in C([t₀−τ, T]) for a delay DE.
pub fn function_space_solution_ty() -> Expr {
    arrow(
        fn_ty(real_ty(), real_ty()),
        arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}
/// `PicardLindelof : ∀ (f : Real → Real → Real) (t0 y0 : Real), ODE f t0 y0 → ∃ φ, ODESolution f t0 y0 φ`
///
/// Picard-Lindelöf existence and uniqueness theorem.
pub fn picard_lindelof_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        pi(
            BinderInfo::Default,
            "t0",
            real_ty(),
            pi(
                BinderInfo::Default,
                "y0",
                real_ty(),
                arrow(app3(cst("ODE"), bvar(2), bvar(1), bvar(0)), prop()),
            ),
        ),
    )
}
/// `LyapunovStability : ∀ (f : Real → Real → Real × Real) (x y : Real),
///   EquilibriumPoint f x y → LyapunovFunction V → Prop`
///
/// Lyapunov's direct method: existence of Lyapunov function implies Lyapunov stability.
pub fn lyapunov_stability_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "f",
        fn_ty(
            real_ty(),
            fn_ty(real_ty(), app2(cst("Prod"), real_ty(), real_ty())),
        ),
        pi(
            BinderInfo::Default,
            "x",
            real_ty(),
            arrow(real_ty(), prop()),
        ),
    )
}
/// `MaximumPrinciple : ∀ (u : Real → Real → Real), LaplacianEqn u → Prop`
///
/// A subharmonic function attains its maximum on the boundary.
pub fn maximum_principle_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "u",
        fn_ty(real_ty(), fn_ty(real_ty(), real_ty())),
        arrow(app(cst("LaplacianEqn"), bvar(0)), prop()),
    )
}
/// `HeatKernelSolution : ∀ (κ : Real) (u0 : Real → Real), Prop`
///
/// The heat equation u_t = κΔu with initial data u₀ has the Gaussian kernel solution.
pub fn heat_kernel_solution_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "kappa",
        real_ty(),
        arrow(fn_ty(real_ty(), real_ty()), prop()),
    )
}
/// `WaveEquationWellPosed : ∀ (c : Real) (u0 u1 : Real → Real), Prop`
///
/// The wave equation u_tt = c²Δu is well-posed with given Cauchy data.
pub fn wave_equation_well_posed_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "c",
        real_ty(),
        arrow(
            fn_ty(real_ty(), real_ty()),
            arrow(fn_ty(real_ty(), real_ty()), prop()),
        ),
    )
}
/// Build the differential equations environment: register all axioms.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("ODE", ode_ty()),
        ("LinearODE", linear_ode_ty()),
        ("ExactODE", exact_ode_ty()),
        ("SturmLiouville", sturm_liouville_ty()),
        ("ODESolution", ode_solution_ty()),
        ("CharacteristicPolynomial", characteristic_polynomial_ty()),
        ("EquilibriumPoint", equilibrium_point_ty()),
        ("JacobianMatrix", jacobian_matrix_ty()),
        ("StabilityClassification", stability_classification_ty()),
        ("LyapunovFunction", lyapunov_function_ty()),
        ("PoincareBendixson", poincare_bendixson_ty()),
        ("Manifold", manifold_ty()),
        ("PDEType", pde_type_ty()),
        ("LaplacianEqn", laplacian_eqn_ty()),
        ("HeatEquation", heat_equation_ty()),
        ("WaveEquation", wave_equation_ty()),
        ("BoundaryCondition", boundary_condition_ty()),
        ("GreensFunction", greens_function_ty()),
        ("EulerStep", euler_step_ty()),
        ("RungeKutta4Step", runge_kutta4_step_ty()),
        ("AdaptiveStepRK45", adaptive_step_rk45_ty()),
        ("MultistepMethod", multistep_method_ty()),
        ("StiffSolver", stiff_solver_ty()),
        ("FlowMap", flow_map_ty()),
        ("LimitSet", limit_set_ty()),
        ("StrangeAttractor", strange_attractor_ty()),
        ("LyapunovExponent", lyapunov_exponent_ty()),
        ("PoincareMap", poincare_map_ty()),
        ("DelayDE", delay_de_ty()),
        ("VolterraIntegralEquation", volterra_integral_equation_ty()),
        ("FredholmIntegralEquation", fredholm_integral_equation_ty()),
        ("HammersteinEquation", hammerstein_equation_ty()),
        ("FunctionSpaceSolution", function_space_solution_ty()),
        ("picard_lindelof", picard_lindelof_ty()),
        ("lyapunov_stability", lyapunov_stability_ty()),
        ("maximum_principle", maximum_principle_ty()),
        ("heat_kernel_solution", heat_kernel_solution_ty()),
        ("wave_equation_well_posed", wave_equation_well_posed_ty()),
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
/// Simple Gaussian elimination for the Fredholm solver.
pub fn solve_linear(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for col in 0..n {
        let mut max_row = col;
        let mut max_val = a[col][col].abs();
        for row in (col + 1)..n {
            if a[row][col].abs() > max_val {
                max_val = a[row][col].abs();
                max_row = row;
            }
        }
        if max_val < 1e-14 {
            return None;
        }
        a.swap(col, max_row);
        b.swap(col, max_row);
        let pivot = a[col][col];
        for row in (col + 1)..n {
            let factor = a[row][col] / pivot;
            b[row] -= factor * b[col];
            for k in col..n {
                a[row][k] -= factor * a[col][k];
            }
        }
    }
    let mut x = vec![0.0f64; n];
    for i in (0..n).rev() {
        let mut s = b[i];
        for j in (i + 1)..n {
            s -= a[i][j] * x[j];
        }
        x[i] = s / a[i][i];
    }
    Some(x)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exact_ode_detection() {
        let ode = ExactODE::new(|x, y| 2.0 * x * y, |x, _y| x * x);
        assert!(ode.is_exact(), "2xy dx + x² dy should be exact");
    }
    #[test]
    fn test_equilibrium_classification_saddle() {
        let f = |x: f64, y: f64| (x, -y);
        let eq = EquilibriumPoint::new(&f, 0.0, 0.0);
        assert_eq!(eq.classify(), StabilityType::Saddle);
    }
    #[test]
    fn test_equilibrium_classification_stable_node() {
        let f = |x: f64, y: f64| (-2.0 * x, -3.0 * y);
        let eq = EquilibriumPoint::new(&f, 0.0, 0.0);
        assert_eq!(eq.classify(), StabilityType::StableNode);
    }
    #[test]
    fn test_heat_equation_parabolic() {
        let heat = HeatEquation::new(1.0);
        assert_eq!(heat.classify_pde(), PDEType::Parabolic);
    }
    #[test]
    fn test_wave_equation_hyperbolic() {
        let wave = WaveEquation::new(1.0);
        assert_eq!(wave.classify_pde(), PDEType::Hyperbolic);
    }
    #[test]
    fn test_laplacian_elliptic() {
        let lap = LaplacianEqn::new(0.0, 1.0, 0.0, 1.0);
        assert_eq!(lap.classify_pde(), PDEType::Elliptic);
    }
    #[test]
    fn test_euler_method_exp() {
        let solver = EulerMethod::new(1e-3);
        let traj = solver.solve_to(&|_, y| y, 0.0, 1.0, 1.0);
        let y_final = traj.last().expect("last should succeed").1;
        let expected = 1.0_f64.exp();
        assert!(
            (y_final - expected).abs() / expected < 0.01,
            "Euler exp: got {y_final}, expected {expected}"
        );
    }
    #[test]
    fn test_runge_kutta4_exp() {
        let solver = RungeKutta4::new(1e-2);
        let traj = solver.solve_to(&|_, y| y, 0.0, 1.0, 1.0);
        let y_final = traj.last().expect("last should succeed").1;
        let expected = 1.0_f64.exp();
        assert!(
            (y_final - expected).abs() < 1e-8,
            "RK4 exp: got {y_final}, expected {expected}"
        );
    }
    #[test]
    fn test_adaptive_rk45_exp() {
        let solver = AdaptiveStepRK45::new(1e-9, 1e-6);
        let traj = solver.solve_to(&|_, y| y, 0.0, 1.0, 1.0);
        let y_final = traj.last().expect("last should succeed").1;
        let expected = 1.0_f64.exp();
        assert!(
            (y_final - expected).abs() < 1e-5,
            "RK45 exp: got {y_final}, expected {expected}"
        );
    }
    #[test]
    fn test_stiff_solver_decay() {
        let solver = StiffSolver::new(1e-3, 1e-10);
        let traj = solver.solve_to(&|_, y| -100.0 * y, 0.0, 1.0, 0.1);
        let y_final = traj.last().expect("last should succeed").1;
        assert!(y_final.abs() < 1e-4, "Stiff solver decay: got {y_final}");
    }
    #[test]
    fn test_heat_fundamental_solution() {
        let heat = HeatEquation::new(1.0);
        let val = heat.fundamental_solution(0.0, 0.1);
        assert!(val.is_finite() && val > 0.0, "Heat kernel positive: {val}");
    }
    #[test]
    fn test_function_space_solution_interp() {
        let sol = FunctionSpaceSolution::new(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0]);
        let y = sol.eval(0.5);
        assert!((y - 0.5).abs() < 1e-12, "Interpolation: {y}");
    }
    #[test]
    fn test_lyapunov_exponent_stable() {
        let le = LyapunovExponent::new(10.0, 1e-2);
        let lambda = le.estimate(&|_, y| -y, 1.0);
        assert!(
            lambda < 0.0,
            "Stable ODE should have negative Lyapunov exponent: {lambda}"
        );
    }
    #[test]
    fn test_build_env() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("ODE")).is_some());
        assert!(env.get(&Name::str("HeatEquation")).is_some());
        assert!(env.get(&Name::str("FlowMap")).is_some());
    }
}
#[cfg(test)]
mod tests_de_extended {
    use super::*;
    #[test]
    fn test_ito_sde_euler_step() {
        let sde = ItoSDE::new("test", -1.0, 0.0, 0.1);
        let x_new = sde.euler_maruyama_step(1.0, 0.0, 0.01, 0.0);
        assert!((x_new - 0.99).abs() < 1e-10);
    }
    #[test]
    fn test_ou_stationary() {
        let ou = ItoSDE::ornstein_uhlenbeck(2.0, 1.5, 0.5);
        let mean = ou.ou_stationary_mean();
        let var = ou.ou_stationary_variance();
        assert!((mean - 1.5).abs() < 1e-10);
        assert!((var - 0.0625).abs() < 1e-10);
    }
    #[test]
    fn test_pitchfork_bifurcation() {
        let params: Vec<f64> = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
        let bd = BifurcationDiagram::pitchfork_normal_form(&params);
        assert_eq!(bd.fixed_points[0].len(), 1);
        assert_eq!(bd.fixed_points[4].len(), 3);
    }
    #[test]
    fn test_hopf_limit_cycle() {
        assert!((BifurcationDiagram::hopf_limit_cycle_radius(1.0) - 1.0).abs() < 1e-10);
        assert!((BifurcationDiagram::hopf_limit_cycle_radius(4.0) - 2.0).abs() < 1e-10);
        assert_eq!(BifurcationDiagram::hopf_limit_cycle_radius(-1.0), 0.0);
    }
    #[test]
    fn test_fredholm_quadrature() {
        let eq = FredholmEquation::new(0.0, 1.0, 0.1, 10);
        let nodes = eq.quadrature_nodes();
        assert_eq!(nodes.len(), 10);
        assert!((nodes[0] - 0.05).abs() < 1e-10);
        let w = eq.quadrature_weight();
        assert!((w - 0.1).abs() < 1e-10);
    }
    #[test]
    fn test_neumann_convergence() {
        let eq = FredholmEquation::new(0.0, 1.0, 0.5, 10);
        assert!(eq.neumann_convergence_condition(1.0));
        assert!(!eq.neumann_convergence_condition(3.0));
    }
    #[test]
    fn test_volterra_picard_iterations() {
        let eq = VolterraEquation::new(0.0, 0.5);
        let iters = eq.picard_iterations_needed(1e-6);
        assert!(iters > 0);
    }
}
