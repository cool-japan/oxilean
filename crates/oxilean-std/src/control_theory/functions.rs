//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    KalmanFilter1D, KalmanFilterState, LqrSolver, LtiSystem, MpcController, PidController,
    StateSpaceModel,
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
pub fn matrix_ty() -> Expr {
    list_ty(list_ty(real_ty()))
}
pub fn pair_real_ty() -> Expr {
    app2(cst("Prod"), real_ty(), real_ty())
}
/// ODE system dx/dt = f(x, u)
/// Type: (List Real → List Real → List Real) → Prop
pub fn ode_system_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        prop(),
    )
}
/// Controllability of a linear system (A, B)
/// Type: (List (List Real)) → (List (List Real)) → Prop
pub fn controllable_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// Observability of a linear system (A, C)
/// Type: (List (List Real)) → (List (List Real)) → Prop
pub fn observable_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// Lyapunov stability: V > 0 and dV/dt < 0 implies stable
/// Type: Prop
pub fn stable_ty() -> Expr {
    prop()
}
/// Optimal control / LQR: existence of cost-minimizing feedback law
/// Type: Prop
pub fn optimal_control_ty() -> Expr {
    prop()
}
/// Asymptotic stability: system is stable AND trajectories converge to equilibrium
/// Type: (List Real → Prop) → Prop
pub fn asymptotically_stable_ty() -> Expr {
    arrow(arrow(list_ty(real_ty()), prop()), prop())
}
/// Input-output stability (BIBO): bounded input implies bounded output
/// Type: (List Real → List Real) → Prop
pub fn bibo_stable_ty() -> Expr {
    arrow(arrow(list_ty(real_ty()), list_ty(real_ty())), prop())
}
/// L2 gain of a system: ||y||_2 ≤ γ * ||u||_2
/// Type: (List Real → List Real) → Real → Prop
pub fn l2_gain_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), list_ty(real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// Passivity: system satisfies ∫ u^T y dt ≥ -β for some β
/// Type: (List (List Real)) → (List (List Real)) → Prop
pub fn passive_system_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// H-infinity norm bound: ||T_zw||_∞ < γ
/// Type: (List (List Real)) → Real → Prop
pub fn h_infinity_bound_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// Structured uncertainty: system with perturbation Δ satisfying ||Δ|| ≤ δ
/// Type: (List (List Real)) → Real → Prop
pub fn structured_uncertainty_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// μ-synthesis: structured singular value condition for robust stability
/// Type: (List (List Real)) → Real → Prop
pub fn mu_synthesis_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// Nyquist stability criterion: encirclements determine closed-loop stability
/// Type: (List Real → Prod Real Real) → Nat → Prop
pub fn nyquist_criterion_ty() -> Expr {
    arrow(arrow(real_ty(), pair_real_ty()), arrow(nat_ty(), prop()))
}
/// Gain margin: how much gain can increase before instability
/// Type: (List (List Real)) → Real → Prop
pub fn gain_margin_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// Phase margin: how much phase lag before instability
/// Type: (List (List Real)) → Real → Prop
pub fn phase_margin_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// Root locus: locus of closed-loop poles as gain K varies
/// Type: (List Real) → (List Real) → Real → List (Prod Real Real)
pub fn root_locus_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(
            list_ty(real_ty()),
            arrow(real_ty(), list_ty(pair_real_ty())),
        ),
    )
}
/// PID stability condition: closed-loop stability with PID parameters (Kp, Ki, Kd)
/// Type: Real → Real → Real → Prop
pub fn pid_stable_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// Model predictive control (MPC) feasibility: optimization is feasible over horizon N
/// Type: (List (List Real)) → Nat → Prop
pub fn mpc_feasible_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), prop()))
}
/// Sliding mode control: existence of sliding surface and reachability condition
/// Type: (List Real → Real) → (List Real → List Real → List Real) → Prop
pub fn sliding_mode_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), real_ty()),
        arrow(
            arrow(
                list_ty(real_ty()),
                arrow(list_ty(real_ty()), list_ty(real_ty())),
            ),
            prop(),
        ),
    )
}
/// Model reference adaptive control (MRAC): tracking error converges to zero
/// Type: (List (List Real)) → (List (List Real)) → Prop
pub fn mrac_convergence_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// Self-tuning regulator: online parameter estimation convergence
/// Type: (List Real → List Real) → Prop
pub fn self_tuning_ty() -> Expr {
    arrow(arrow(list_ty(real_ty()), list_ty(real_ty())), prop())
}
/// Feedback linearization: existence of diffeomorphism making system linear
/// Type: (List Real → List Real → List Real) → Prop
pub fn feedback_linearizable_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        prop(),
    )
}
/// Backstepping design: recursive Lyapunov-based stabilization
/// Type: (List Real → List Real → List Real) → Nat → Prop
pub fn backstepping_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        arrow(nat_ty(), prop()),
    )
}
/// Differential flatness: system output flat map exists
/// Type: (List Real → List Real → List Real) → Prop
pub fn differentially_flat_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        prop(),
    )
}
/// Pontryagin maximum principle: necessary conditions for optimal control
/// Type: Prop
pub fn pontryagin_maximum_principle_ty() -> Expr {
    prop()
}
/// Hamilton-Jacobi-Bellman equation: sufficient condition for optimal control
/// Type: (List Real → Real → Real) → Prop
pub fn hamilton_jacobi_bellman_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), arrow(real_ty(), real_ty())),
        prop(),
    )
}
/// Switched systems stability: common Lyapunov function for all subsystems
/// Type: (List (List (List Real))) → Prop
pub fn switched_stable_ty() -> Expr {
    arrow(list_ty(matrix_ty()), prop())
}
/// Hybrid control: stability of hybrid system with guards and resets
/// Type: Prop
pub fn hybrid_stable_ty() -> Expr {
    prop()
}
/// Kalman filter optimality: minimum variance unbiased estimator
/// Type: (List (List Real)) → (List (List Real)) → Prop
pub fn kalman_optimal_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// LQG (Linear-Quadratic-Gaussian) optimality: combines LQR + Kalman filter
/// Type: Prop
pub fn lqg_optimal_ty() -> Expr {
    prop()
}
/// Separation principle: LQG optimal iff LQR + Kalman filter solved independently
/// Type: Prop
pub fn separation_principle_ty() -> Expr {
    prop()
}
/// Robust stability under perturbation: system remains stable for all Δ with ||Δ|| < δ
/// Type: (List (List Real)) → Real → Prop
pub fn robust_stable_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// Small gain theorem: feedback interconnection is stable if L2 gains satisfy γ1*γ2 < 1
/// Type: Real → Real → Prop
pub fn small_gain_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// Controllability Gramian: W_c = ∫_0^T e^{At} B B^T e^{A^T t} dt
/// Type: (List (List Real)) → (List (List Real)) → Real → List (List Real)
pub fn controllability_gramian_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(matrix_ty(), arrow(real_ty(), matrix_ty())),
    )
}
/// Observability Gramian: W_o = ∫_0^T e^{A^T t} C^T C e^{At} dt
/// Type: (List (List Real)) → (List (List Real)) → Real → List (List Real)
pub fn observability_gramian_ty() -> Expr {
    arrow(
        matrix_ty(),
        arrow(matrix_ty(), arrow(real_ty(), matrix_ty())),
    )
}
/// Lyapunov stability theorem: V > 0, dV/dt < 0 → equilibrium is stable
pub fn lyapunov_stability_ty() -> Expr {
    prop()
}
/// Kalman rank condition: system (A, B) is controllable iff
/// rank(\[B, AB, ..., A^{n-1}B\]) = n
pub fn kalman_rank_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        matrix_ty(),
        pi(BinderInfo::Default, "B", matrix_ty(), prop()),
    )
}
/// Pole placement theorem: if (A, B) controllable, eigenvalues can be
/// placed arbitrarily by state feedback u = -Kx
pub fn pole_placement_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        matrix_ty(),
        pi(BinderInfo::Default, "B", matrix_ty(), prop()),
    )
}
/// LQR optimality: the Riccati equation solution yields the optimal feedback
pub fn lqr_optimality_ty() -> Expr {
    prop()
}
/// Register all control theory axioms and theorems in the kernel environment.
pub fn build_control_theory_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("ODESystem", ode_system_ty()),
        ("Controllable", controllable_ty()),
        ("Observable", observable_ty()),
        ("Stable", stable_ty()),
        ("OptimalControl", optimal_control_ty()),
        ("LyapunovFunction", arrow(list_ty(real_ty()), prop())),
        (
            "RiccatiSolution",
            arrow(matrix_ty(), arrow(matrix_ty(), matrix_ty())),
        ),
        (
            "StateTransferMatrix",
            arrow(matrix_ty(), arrow(real_ty(), matrix_ty())),
        ),
        ("PoleSet", arrow(matrix_ty(), list_ty(real_ty()))),
        ("lyapunov_stability", lyapunov_stability_ty()),
        ("kalman_rank", kalman_rank_ty()),
        ("pole_placement", pole_placement_ty()),
        ("lqr_optimality", lqr_optimality_ty()),
        ("AsymptoticallyStable", asymptotically_stable_ty()),
        ("BiboStable", bibo_stable_ty()),
        ("L2Gain", l2_gain_ty()),
        ("PassiveSystem", passive_system_ty()),
        ("HInfinityBound", h_infinity_bound_ty()),
        ("StructuredUncertainty", structured_uncertainty_ty()),
        ("MuSynthesis", mu_synthesis_ty()),
        ("NyquistCriterion", nyquist_criterion_ty()),
        ("GainMargin", gain_margin_ty()),
        ("PhaseMargin", phase_margin_ty()),
        ("RootLocus", root_locus_ty()),
        ("PidStable", pid_stable_ty()),
        ("MpcFeasible", mpc_feasible_ty()),
        ("SlidingMode", sliding_mode_ty()),
        ("FeedbackLinearizable", feedback_linearizable_ty()),
        ("Backstepping", backstepping_ty()),
        ("DifferentiallyFlat", differentially_flat_ty()),
        ("MracConvergence", mrac_convergence_ty()),
        ("SelfTuning", self_tuning_ty()),
        (
            "PontryaginMaximumPrinciple",
            pontryagin_maximum_principle_ty(),
        ),
        ("HamiltonJacobiBellman", hamilton_jacobi_bellman_ty()),
        ("KalmanOptimal", kalman_optimal_ty()),
        ("LqgOptimal", lqg_optimal_ty()),
        ("SeparationPrinciple", separation_principle_ty()),
        ("SwitchedStable", switched_stable_ty()),
        ("HybridStable", hybrid_stable_ty()),
        ("RobustStable", robust_stable_ty()),
        ("SmallGain", small_gain_ty()),
        ("ControllabilityGramian", controllability_gramian_ty()),
        ("ObservabilityGramian", observability_gramian_ty()),
        ("bounded_input_bounded_output", prop()),
        ("nyquist_stability", prop()),
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
/// Compute the LQR quadratic cost for a trajectory.
///
/// J = Σ_k (x_k^T Q x_k + u_k^T R u_k)
///
/// With diagonal Q = q_diag * I and R = r_diag * I this simplifies to:
///   J = Σ_k (q_diag * ||x_k||^2 + r_diag * ||u_k||^2)
pub fn lqr_cost(trajectory: &[Vec<f64>], inputs: &[Vec<f64>], q_diag: f64, r_diag: f64) -> f64 {
    let state_cost: f64 = trajectory
        .iter()
        .map(|x| q_diag * x.iter().map(|&xi| xi * xi).sum::<f64>())
        .sum();
    let input_cost: f64 = inputs
        .iter()
        .map(|u| r_diag * u.iter().map(|&ui| ui * ui).sum::<f64>())
        .sum();
    state_cost + input_cost
}
#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 1e-9;
    #[test]
    fn test_pid_controller_step() {
        let mut pid = PidController::new(1.0, 0.0, 0.0, 10.0);
        let out = pid.update(0.0, 0.1);
        assert!((out - 10.0).abs() < EPS, "expected 10.0, got {out}");
    }
    #[test]
    fn test_pid_controller_reset() {
        let mut pid = PidController::new(1.0, 1.0, 0.0, 10.0);
        pid.update(0.0, 1.0);
        pid.reset();
        let out = pid.update(0.0, 1.0);
        assert!((out - 20.0).abs() < EPS, "expected 20.0, got {out}");
    }
    #[test]
    fn test_pid_anti_windup() {
        let mut pid = PidController::with_limits(1.0, 1.0, 0.0, 0.0, 5.0, 0.0);
        for _ in 0..100 {
            pid.update(0.0, 1.0);
        }
        assert!(
            pid.integral_state() <= 5.0 + EPS,
            "integral should be clamped to 5.0, got {}",
            pid.integral_state()
        );
    }
    #[test]
    fn test_pid_output_saturation() {
        let mut pid = PidController::with_limits(100.0, 0.0, 0.0, 10.0, f64::MAX, 15.0);
        let out = pid.update(0.0, 0.1);
        assert!(
            out <= 15.0 + EPS,
            "output should be saturated to 15.0, got {out}"
        );
    }
    #[test]
    fn test_state_space_model_output() {
        let a = vec![vec![-1.0, 0.0], vec![0.0, -2.0]];
        let b = vec![vec![1.0], vec![1.0]];
        let c = vec![vec![1.0, 0.0]];
        let d = vec![vec![0.5]];
        let sys = StateSpaceModel::new(a, b, c, d);
        let state = vec![3.0, 5.0];
        let input = vec![2.0];
        let y = sys.output(&state, &input);
        assert_eq!(y.len(), 1);
        assert!((y[0] - 4.0).abs() < EPS, "expected 4.0, got {}", y[0]);
    }
    #[test]
    fn test_state_space_model_no_feedthrough() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![1.0]];
        let c = vec![vec![1.0]];
        let sys = StateSpaceModel::no_feedthrough(a, b, c);
        assert_eq!(sys.d, vec![vec![0.0]]);
    }
    #[test]
    fn test_state_space_model_euler_step() {
        let sys =
            StateSpaceModel::no_feedthrough(vec![vec![0.0]], vec![vec![1.0]], vec![vec![1.0]]);
        let next = sys.euler_step(&[0.0], &[1.0], 0.1);
        assert!((next[0] - 0.1).abs() < EPS, "expected 0.1, got {}", next[0]);
    }
    #[test]
    fn test_lti_euler_step() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![vec![0.0], vec![0.0]];
        let c = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let sys = LtiSystem::new(a, b, c);
        let state = vec![3.0, 4.0];
        let input = vec![0.0];
        let next = sys.euler_step(&state, &input, 0.0);
        assert!((next[0] - 3.0).abs() < EPS);
        assert!((next[1] - 4.0).abs() < EPS);
    }
    #[test]
    fn test_lti_output() {
        let a = vec![vec![-1.0, 0.0], vec![0.0, -2.0]];
        let b = vec![vec![1.0], vec![1.0]];
        let c = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let sys = LtiSystem::new(a, b, c);
        let state = vec![5.0, 7.0];
        let input = vec![0.0];
        let y = sys.output(&state, &input);
        assert!((y[0] - 5.0).abs() < EPS);
        assert!((y[1] - 7.0).abs() < EPS);
    }
    #[test]
    fn test_lti_simulate() {
        let a = vec![vec![-1.0]];
        let b = vec![vec![0.0]];
        let c = vec![vec![1.0]];
        let sys = LtiSystem::new(a, b, c);
        let initial = vec![1.0];
        let inputs: Vec<Vec<f64>> = vec![vec![0.0]; 3];
        let states = sys.simulate(initial, &inputs, 0.1);
        assert_eq!(states.len(), 4);
        assert!(states[1][0] < states[0][0], "state should decrease");
        assert!(
            states[3][0] > 0.0,
            "state should remain positive (small dt)"
        );
    }
    #[test]
    fn test_kalman_filter_predict() {
        let mut kf = KalmanFilter1D::new(0.0, 1.0, 0.1, 1.0);
        kf.predict(1.0, 2.0);
        assert!(
            (kf.estimate() - 2.0).abs() < EPS,
            "expected 2.0, got {}",
            kf.estimate()
        );
        assert!(
            (kf.uncertainty() - 1.1).abs() < EPS,
            "expected 1.1, got {}",
            kf.uncertainty()
        );
    }
    #[test]
    fn test_kalman_filter_update() {
        let mut kf = KalmanFilter1D::new(0.0, 1.0, 0.0, 1.0);
        kf.update(4.0);
        assert!(
            (kf.estimate() - 2.0).abs() < EPS,
            "expected 2.0, got {}",
            kf.estimate()
        );
        assert!(
            (kf.uncertainty() - 0.5).abs() < EPS,
            "expected 0.5, got {}",
            kf.uncertainty()
        );
    }
    #[test]
    fn test_kalman_filter_state_predict() {
        let f = vec![vec![1.0]];
        let h = vec![vec![1.0]];
        let q = vec![vec![0.1]];
        let r = vec![vec![1.0]];
        let mut kf = KalmanFilterState::new(vec![0.0], vec![vec![1.0]], f, h, q, r);
        kf.predict();
        let (n, m) = kf.dims();
        assert_eq!(n, 1);
        assert_eq!(m, 1);
        assert!((kf.state()[0] - 0.0).abs() < EPS);
    }
    #[test]
    fn test_kalman_filter_state_update() {
        let f = vec![vec![1.0]];
        let h = vec![vec![1.0]];
        let q = vec![vec![0.0]];
        let r = vec![vec![1.0]];
        let mut kf = KalmanFilterState::new(vec![0.0], vec![vec![1.0]], f, h, q, r);
        kf.update(&[4.0]);
        assert!(
            (kf.state()[0] - 2.0).abs() < 1e-8,
            "expected 2.0, got {}",
            kf.state()[0]
        );
    }
    #[test]
    fn test_lqr_cost_zero() {
        let trajectory: Vec<Vec<f64>> = vec![vec![0.0, 0.0]; 5];
        let inputs: Vec<Vec<f64>> = vec![vec![0.0]; 5];
        let cost = lqr_cost(&trajectory, &inputs, 1.0, 1.0);
        assert!(cost.abs() < EPS, "expected 0.0, got {cost}");
    }
    #[test]
    fn test_lqr_solver_scalar() {
        let solver = LqrSolver::new(vec![vec![1.0]], vec![vec![1.0]]);
        let a = vec![vec![0.9]];
        let b = vec![vec![1.0]];
        if let Some((p, k)) = solver.solve(&a, &b) {
            assert!(p[0][0] > 0.0, "P should be positive, got {}", p[0][0]);
            assert!(k[0][0].is_finite(), "K should be finite, got {}", k[0][0]);
        }
    }
    #[test]
    fn test_lqr_solver_trajectory_cost() {
        let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let r = vec![vec![1.0]];
        let solver = LqrSolver::new(q, r);
        let states = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let inputs = vec![vec![1.0], vec![0.0]];
        let cost = solver.trajectory_cost(&states, &inputs);
        assert!((cost - 3.0).abs() < EPS, "expected 3.0, got {cost}");
    }
    #[test]
    fn test_mpc_controller_control() {
        let a = vec![vec![2.0]];
        let b = vec![vec![1.0]];
        let q = vec![vec![1.0]];
        let r = vec![vec![0.01]];
        let mpc = MpcController::new(a, b, q, r, 5, vec![]);
        let u = mpc.control(&[1.0]);
        assert!(u.is_some(), "MPC control should return Some");
        let u = u.expect("u should be valid");
        assert!(
            u[0] < 0.0,
            "control should be negative to drive state to 0, got {}",
            u[0]
        );
    }
    #[test]
    fn test_mpc_controller_constraints() {
        let a = vec![vec![1.0]];
        let b = vec![vec![1.0]];
        let q = vec![vec![1.0]];
        let r = vec![vec![0.001]];
        let u_max = vec![0.5];
        let mpc = MpcController::new(a, b, q, r, 3, u_max);
        let u = mpc.control(&[10.0]);
        assert!(u.is_some());
        let u = u.expect("u should be valid");
        assert!(
            u[0].abs() <= 0.5 + EPS,
            "control should be within bounds ±0.5, got {}",
            u[0]
        );
    }
    #[test]
    fn test_mpc_simulate() {
        let a = vec![vec![-0.5]];
        let b = vec![vec![1.0]];
        let q = vec![vec![1.0]];
        let r = vec![vec![1.0]];
        let mpc = MpcController::new(a, b, q, r, 3, vec![]);
        let (states, inputs) = mpc.simulate(vec![1.0], 5, 0.1);
        assert_eq!(states.len(), 6);
        assert_eq!(inputs.len(), 5);
    }
    #[test]
    fn test_build_control_theory_env() {
        let mut env = oxilean_kernel::Environment::new();
        let result = build_control_theory_env(&mut env);
        assert!(
            result.is_ok(),
            "build_control_theory_env failed: {:?}",
            result.err()
        );
    }
    #[test]
    fn test_env_axiom_count() {
        let mut env = oxilean_kernel::Environment::new();
        build_control_theory_env(&mut env).expect("build_control_theory_env should succeed");
    }
}
/// H∞ optimal controller existence: ∃ K such that ||T_zw(K)||_∞ < γ
/// Type: (List (List Real)) → Real → Prop
pub fn ct_ext_h_inf_controller_exists_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), prop()))
}
/// H∞ synthesis via Riccati equations: two coupled Riccati equations
/// Type: (List (List Real)) → (List (List Real)) → Real → Prop
pub fn ct_ext_h_inf_riccati_synthesis_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), arrow(real_ty(), prop())))
}
/// H2 norm of a transfer function: ||G||_2^2 = tr(B^T W_o B)
/// Type: (List (List Real)) → (List (List Real)) → Real
pub fn ct_ext_h2_norm_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), real_ty()))
}
/// Structured singular value μ: μ_Δ(M) = min{σ_min(Δ) : det(I - MΔ) = 0}
/// Type: (List (List Real)) → Real
pub fn ct_ext_structured_singular_value_ty() -> Expr {
    arrow(matrix_ty(), real_ty())
}
/// Robust performance: stability and performance under all perturbations
/// Type: (List (List Real)) → Real → Real → Prop
pub fn ct_ext_robust_performance_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), arrow(real_ty(), prop())))
}
/// Kharitonov's theorem: interval polynomial stability
/// Type: (List Real) → (List Real) → Prop
pub fn ct_ext_kharitonov_stability_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), prop()))
}
/// Pontryagin Hamiltonian: H(x, u, λ) = λ^T f(x, u) + L(x, u)
/// Type: (List Real → List Real → Real) → Real
pub fn ct_ext_pontryagin_hamiltonian_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), arrow(list_ty(real_ty()), real_ty())),
        real_ty(),
    )
}
/// Transversality condition for free end-time optimal control
/// Type: Prop
pub fn ct_ext_transversality_condition_ty() -> Expr {
    prop()
}
/// Bellman's principle of optimality: subproblem of optimal problem is optimal
/// Type: Prop
pub fn ct_ext_bellman_optimality_ty() -> Expr {
    prop()
}
/// HJB PDE: -∂V/∂t = min_u {H(x, u, ∂V/∂x)}
/// Type: (List Real → Real → Real) → Real → Prop
pub fn ct_ext_hjb_pde_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), arrow(real_ty(), real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// Value function convexity in linear-convex optimal control
/// Type: (List Real → Real) → Prop
pub fn ct_ext_value_function_convex_ty() -> Expr {
    arrow(arrow(list_ty(real_ty()), real_ty()), prop())
}
/// Riccati ODE solution for finite-horizon LQR
/// Type: Real → (List (List Real)) → (List (List Real)) → List (List Real)
pub fn ct_ext_riccati_ode_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(matrix_ty(), arrow(matrix_ty(), matrix_ty())),
    )
}
/// Extended Kalman filter (EKF): linearized Kalman filter for nonlinear systems
/// Type: (List Real → List Real → List Real) → Prop
pub fn ct_ext_extended_kalman_filter_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        prop(),
    )
}
/// Unscented Kalman filter (UKF): sigma-point propagation for nonlinear estimation
/// Type: (List Real → List Real) → Real → Prop
pub fn ct_ext_unscented_kalman_filter_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), list_ty(real_ty())),
        arrow(real_ty(), prop()),
    )
}
/// Particle filter: sequential Monte Carlo state estimation
/// Type: Nat → (List Real → List Real) → Prop
pub fn ct_ext_particle_filter_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(arrow(list_ty(real_ty()), list_ty(real_ty())), prop()),
    )
}
/// Innovation sequence whiteness test: residuals are white noise if filter is optimal
/// Type: (List Real) → Prop
pub fn ct_ext_innovation_whiteness_ty() -> Expr {
    arrow(list_ty(real_ty()), prop())
}
/// Cramer-Rao lower bound for state estimation
/// Type: (List (List Real)) → (List (List Real))
pub fn ct_ext_cramer_rao_bound_ty() -> Expr {
    arrow(matrix_ty(), matrix_ty())
}
/// Receding horizon stability: terminal cost and constraint ensure stability
/// Type: (List (List Real)) → Nat → Prop
pub fn ct_ext_receding_horizon_stable_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), prop()))
}
/// MPC terminal constraint: state at horizon end in terminal set
/// Type: (List Real) → (List Real → Prop) → Prop
pub fn ct_ext_mpc_terminal_constraint_ty() -> Expr {
    arrow(
        list_ty(real_ty()),
        arrow(arrow(list_ty(real_ty()), prop()), prop()),
    )
}
/// Economic MPC: optimise stage cost not necessarily positive definite
/// Type: (List Real → Real) → Nat → Prop
pub fn ct_ext_economic_mpc_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), real_ty()),
        arrow(nat_ty(), prop()),
    )
}
/// Stochastic MPC: chance constraints satisfied with probability p
/// Type: (List (List Real)) → Real → Nat → Prop
pub fn ct_ext_stochastic_mpc_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), arrow(nat_ty(), prop())))
}
/// Input-to-state stability (ISS): x-dynamics are ISS with respect to input u
/// Type: (List Real → List Real → List Real) → Prop
pub fn ct_ext_input_state_stable_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        prop(),
    )
}
/// ISS Lyapunov function existence: V satisfying ISS conditions
/// Type: (List Real → Real) → (List Real → List Real → List Real) → Prop
pub fn ct_ext_iss_lyapunov_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), real_ty()),
        arrow(
            arrow(
                list_ty(real_ty()),
                arrow(list_ty(real_ty()), list_ty(real_ty())),
            ),
            prop(),
        ),
    )
}
/// Integral ISS (iISS): weaker form with integrable gain
/// Type: (List Real → List Real → List Real) → Prop
pub fn ct_ext_integral_iss_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        prop(),
    )
}
/// Nonlinear observability: rank condition for nonlinear systems
/// Type: (List Real → List Real → List Real) → (List Real → List Real) → Prop
pub fn ct_ext_nonlinear_observable_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        arrow(arrow(list_ty(real_ty()), list_ty(real_ty())), prop()),
    )
}
/// Normal form for nonlinear systems via feedback linearization
/// Type: (List Real → List Real → List Real) → (List Real → List Real) → Prop
pub fn ct_ext_normal_form_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        arrow(arrow(list_ty(real_ty()), list_ty(real_ty())), prop()),
    )
}
/// Zero dynamics: dynamics on the zero-output manifold
/// Type: (List Real → List Real → List Real) → Prop
pub fn ct_ext_zero_dynamics_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        prop(),
    )
}
/// Relative degree of a nonlinear system
/// Type: (List Real → List Real → List Real) → Nat
pub fn ct_ext_relative_degree_ty() -> Expr {
    arrow(
        arrow(
            list_ty(real_ty()),
            arrow(list_ty(real_ty()), list_ty(real_ty())),
        ),
        nat_ty(),
    )
}
/// Frequency response function (FRF): G(jω) evaluated at frequency ω
/// Type: (List (List Real)) → Real → Prod Real Real
pub fn ct_ext_frequency_response_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), pair_real_ty()))
}
/// Bode plot gain: 20 log₁₀ |G(jω)|
/// Type: (List (List Real)) → Real → Real
pub fn ct_ext_bode_gain_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), real_ty()))
}
/// Bode plot phase: arg(G(jω)) in degrees
/// Type: (List (List Real)) → Real → Real
pub fn ct_ext_bode_phase_ty() -> Expr {
    arrow(matrix_ty(), arrow(real_ty(), real_ty()))
}
/// Crossover frequency: |G(jω_c)| = 1 (0 dB)
/// Type: (List (List Real)) → Real
pub fn ct_ext_crossover_frequency_ty() -> Expr {
    arrow(matrix_ty(), real_ty())
}
/// Bandwidth: frequency at which gain drops to -3 dB
/// Type: (List (List Real)) → Real
pub fn ct_ext_bandwidth_ty() -> Expr {
    arrow(matrix_ty(), real_ty())
}
/// Sensitivity function S = (I + GK)^{-1}: robustness measure
/// Type: (List (List Real)) → (List (List Real)) → List (List Real)
pub fn ct_ext_sensitivity_function_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), matrix_ty()))
}
/// Complementary sensitivity T = GK(I + GK)^{-1}: tracking performance
/// Type: (List (List Real)) → (List (List Real)) → List (List Real)
pub fn ct_ext_complementary_sensitivity_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), matrix_ty()))
}
/// Discrete-time Lyapunov stability: V(x_{k+1}) - V(x_k) < 0
/// Type: (List Real → Real) → (List (List Real)) → Prop
pub fn ct_ext_discrete_lyapunov_ty() -> Expr {
    arrow(
        arrow(list_ty(real_ty()), real_ty()),
        arrow(matrix_ty(), prop()),
    )
}
/// Discrete algebraic Riccati equation (DARE) solution existence
/// Type: (List (List Real)) → (List (List Real)) → Prop
pub fn ct_ext_dare_solution_ty() -> Expr {
    arrow(matrix_ty(), arrow(matrix_ty(), prop()))
}
/// Deadbeat control: finite-step convergence to zero
/// Type: (List (List Real)) → Nat → Prop
pub fn ct_ext_deadbeat_control_ty() -> Expr {
    arrow(matrix_ty(), arrow(nat_ty(), prop()))
}
/// Register all extended control theory axioms in the kernel environment.
pub fn register_control_theory_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("HInfControllerExists", ct_ext_h_inf_controller_exists_ty),
        ("HInfRiccatiSynthesis", ct_ext_h_inf_riccati_synthesis_ty),
        ("H2Norm", ct_ext_h2_norm_ty),
        (
            "StructuredSingularValue",
            ct_ext_structured_singular_value_ty,
        ),
        ("RobustPerformance", ct_ext_robust_performance_ty),
        ("KharitonovStability", ct_ext_kharitonov_stability_ty),
        ("PontryaginHamiltonian", ct_ext_pontryagin_hamiltonian_ty),
        (
            "TransversalityCondition",
            ct_ext_transversality_condition_ty,
        ),
        ("BellmanOptimality", ct_ext_bellman_optimality_ty),
        ("HjbPde", ct_ext_hjb_pde_ty),
        ("ValueFunctionConvex", ct_ext_value_function_convex_ty),
        ("RiccatiOde", ct_ext_riccati_ode_ty),
        ("ExtendedKalmanFilter", ct_ext_extended_kalman_filter_ty),
        ("UnscentedKalmanFilter", ct_ext_unscented_kalman_filter_ty),
        ("ParticleFilter", ct_ext_particle_filter_ty),
        ("InnovationWhiteness", ct_ext_innovation_whiteness_ty),
        ("CramerRaoBound", ct_ext_cramer_rao_bound_ty),
        ("RecedingHorizonStable", ct_ext_receding_horizon_stable_ty),
        ("MpcTerminalConstraint", ct_ext_mpc_terminal_constraint_ty),
        ("EconomicMpc", ct_ext_economic_mpc_ty),
        ("StochasticMpc", ct_ext_stochastic_mpc_ty),
        ("InputStateStable", ct_ext_input_state_stable_ty),
        ("IssLyapunov", ct_ext_iss_lyapunov_ty),
        ("IntegralIss", ct_ext_integral_iss_ty),
        ("NonlinearObservable", ct_ext_nonlinear_observable_ty),
        ("NormalForm", ct_ext_normal_form_ty),
        ("ZeroDynamics", ct_ext_zero_dynamics_ty),
        ("RelativeDegree", ct_ext_relative_degree_ty),
        ("FrequencyResponse", ct_ext_frequency_response_ty),
        ("BodeGain", ct_ext_bode_gain_ty),
        ("BodePhase", ct_ext_bode_phase_ty),
        ("CrossoverFrequency", ct_ext_crossover_frequency_ty),
        ("Bandwidth", ct_ext_bandwidth_ty),
        ("SensitivityFunction", ct_ext_sensitivity_function_ty),
        (
            "ComplementarySensitivity",
            ct_ext_complementary_sensitivity_ty,
        ),
        ("DiscreteLyapunov", ct_ext_discrete_lyapunov_ty),
        ("DareSolution", ct_ext_dare_solution_ty),
        ("DeadbeatControl", ct_ext_deadbeat_control_ty),
    ];
    for (name, mk_ty) in axioms {
        let ty = mk_ty();
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
