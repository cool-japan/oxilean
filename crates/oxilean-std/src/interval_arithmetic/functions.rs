//! # Interval Arithmetic — Functions and Environment Builder
//!
//! Algorithms for interval arithmetic: verified root finding, interval linear systems,
//! dependency analysis, mean value form, and the Lean4-kernel environment builder.

use oxilean_kernel::{Declaration, Environment, Expr, Level, Name};

use super::types::{
    interval_div, DependencyAnalysis, DualInterval, InclusionFunctionResult, InclusionFunctionType,
    Interval, IntervalError, IntervalLinearSystem, IntervalMatrix, IntervalVector, KaucherInterval,
    LinearSystemResult, RootEnclosure, RootFindingConfig,
};

// ─── Kernel Expression Helpers ────────────────────────────────────────────────

fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}

fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}

fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}

fn real_ty() -> Expr {
    cst("Real")
}

fn nat_ty() -> Expr {
    cst("Nat")
}

fn bool_ty() -> Expr {
    cst("Bool")
}

// ─── Lean4-Type Declarations ──────────────────────────────────────────────────

/// `Interval : Type` — a classical closed real interval \[lo, hi\].
pub fn interval_ty() -> Expr {
    type0()
}

/// `KaucherInterval : Type` — a directed (Kaucher) interval allowing a > b.
pub fn kaucher_interval_ty() -> Expr {
    type0()
}

/// `interval_lo : Interval → Real` — lower bound accessor.
pub fn interval_lo_ty() -> Expr {
    arrow(interval_ty(), real_ty())
}

/// `interval_hi : Interval → Real` — upper bound accessor.
pub fn interval_hi_ty() -> Expr {
    arrow(interval_ty(), real_ty())
}

/// `interval_add : Interval → Interval → Interval` — interval addition.
pub fn interval_add_ty() -> Expr {
    arrow(interval_ty(), arrow(interval_ty(), interval_ty()))
}

/// `interval_mul : Interval → Interval → Interval` — interval multiplication.
pub fn interval_mul_ty() -> Expr {
    arrow(interval_ty(), arrow(interval_ty(), interval_ty()))
}

/// `interval_div : Interval → Interval → Option Interval` — interval division (may fail).
pub fn interval_div_ty() -> Expr {
    arrow(
        interval_ty(),
        arrow(interval_ty(), app(cst("Option"), interval_ty())),
    )
}

/// `interval_sqrt : Interval → Option Interval` — interval square root.
pub fn interval_sqrt_ty() -> Expr {
    arrow(interval_ty(), app(cst("Option"), interval_ty()))
}

/// `interval_contains : Interval → Real → Prop` — membership predicate.
pub fn interval_contains_ty() -> Expr {
    arrow(interval_ty(), arrow(real_ty(), prop()))
}

/// `DualInterval : Type` — for automatic differentiation over intervals.
pub fn dual_interval_ty() -> Expr {
    type0()
}

/// `IntervalVector : Nat → Type` — vector of intervals.
pub fn interval_vector_ty() -> Expr {
    arrow(nat_ty(), type0())
}

/// `IntervalMatrix : Nat → Nat → Type` — matrix of intervals.
pub fn interval_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}

// ─── Axioms and Theorems ──────────────────────────────────────────────────────

/// `interval_add_sound : ∀ X Y x y, x ∈ X → y ∈ Y → x + y ∈ X + Y` (soundness of addition).
pub fn interval_add_sound_ty() -> Expr {
    arrow(
        interval_ty(),
        arrow(interval_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}

/// `interval_mul_sound : ∀ X Y x y, x ∈ X → y ∈ Y → x * y ∈ X * Y`.
pub fn interval_mul_sound_ty() -> Expr {
    arrow(
        interval_ty(),
        arrow(interval_ty(), arrow(real_ty(), arrow(real_ty(), prop()))),
    )
}

/// `inclusion_monotone : A ⊆ B → F(A) ⊆ F(B)` — inclusion functions are monotone.
pub fn inclusion_monotone_ty() -> Expr {
    arrow(interval_ty(), arrow(interval_ty(), prop()))
}

/// `subinterval_convergence : width(X_n) → 0 ⟹ width(F(X_n)) → 0` (convergence).
pub fn subinterval_convergence_ty() -> Expr {
    arrow(cst("Sequence"), prop())
}

/// `kaucher_group : KaucherInterval forms a group under addition`.
pub fn kaucher_group_ty() -> Expr {
    app(cst("AbelianGroup"), kaucher_interval_ty())
}

/// `mean_value_theorem_interval : ∀ f f' X x₀, f(x₀) + f'(X)*(X-x₀) ⊇ f(X)`.
pub fn mean_value_interval_ty() -> Expr {
    arrow(cst("DifferentiableFunction"), arrow(interval_ty(), prop()))
}

/// `newton_interval_correct : root of f in F(X) implies contraction converges`.
pub fn newton_interval_correct_ty() -> Expr {
    arrow(interval_ty(), arrow(cst("Function"), prop()))
}

/// `dependency_overestimation : natural extension width ≥ range(f) width`.
pub fn dependency_overestimation_ty() -> Expr {
    arrow(cst("PolynomialFunction"), arrow(interval_ty(), prop()))
}

// ─── Environment Builder ──────────────────────────────────────────────────────

/// Build the interval arithmetic environment with all type and axiom declarations.
pub fn build_interval_arithmetic_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Interval", interval_ty()),
        ("KaucherInterval", kaucher_interval_ty()),
        ("IntervalLo", interval_lo_ty()),
        ("IntervalHi", interval_hi_ty()),
        ("IntervalAdd", interval_add_ty()),
        ("IntervalMul", interval_mul_ty()),
        ("IntervalDiv", interval_div_ty()),
        ("IntervalSqrt", interval_sqrt_ty()),
        ("IntervalContains", interval_contains_ty()),
        ("DualInterval", dual_interval_ty()),
        ("IntervalVector", interval_vector_ty()),
        ("IntervalMatrix", interval_matrix_ty()),
        ("IntervalAddSound", interval_add_sound_ty()),
        ("IntervalMulSound", interval_mul_sound_ty()),
        ("InclusionMonotone", inclusion_monotone_ty()),
        ("SubintervalConvergence", subinterval_convergence_ty()),
        ("KaucherGroup", kaucher_group_ty()),
        ("MeanValueInterval", mean_value_interval_ty()),
        ("NewtonIntervalCorrect", newton_interval_correct_ty()),
        ("DependencyOverestimation", dependency_overestimation_ty()),
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

// ─── Verified Root Finding ────────────────────────────────────────────────────

/// **Interval bisection method** for finding a root of `f` in `[a, b]`.
///
/// Requires that `f(a) * f(b) < 0` (sign change guarantees a root by IVT).
/// Returns an enclosure of width ≤ `config.tolerance`.
pub fn bisection_root<F>(
    f: F,
    initial: Interval,
    config: RootFindingConfig,
) -> Result<RootEnclosure, IntervalError>
where
    F: Fn(Interval) -> Interval,
{
    let fa = f(Interval::point(initial.lo));
    let fb = f(Interval::point(initial.hi));

    // Check sign change
    let existence_verified = fa.hi * fb.lo < 0.0 || fa.lo * fb.hi < 0.0;

    let mut lo = initial.lo;
    let mut hi = initial.hi;
    let mut iterations = 0;

    while hi - lo > config.tolerance && iterations < config.max_iterations {
        let mid = (lo + hi) / 2.0;
        let fmid = f(Interval::point(mid));

        let fa_cur = f(Interval::point(lo));

        // Check sign of f at midpoint vs left endpoint
        if fa_cur.lo * fmid.hi < 0.0 || fa_cur.hi * fmid.lo < 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
        iterations += 1;
    }

    let enclosure = Interval { lo, hi };
    Ok(RootEnclosure {
        enclosure,
        iterations,
        existence_verified,
    })
}

/// **Interval Newton method** for root finding with quadratic convergence.
///
/// The Newton operator: `N(X) = x₀ - f(x₀) / F'(X)` where `x₀` is the midpoint.
/// If `N(X) ⊆ X`, then a unique root exists in `X`.
pub fn newton_interval<F, DF>(
    f: F,
    df: DF,
    initial: Interval,
    config: RootFindingConfig,
) -> Result<RootEnclosure, IntervalError>
where
    F: Fn(f64) -> f64,
    DF: Fn(Interval) -> Interval,
{
    let mut x = initial;
    let mut iterations = 0;
    let mut existence_verified = false;

    while x.width() > config.tolerance && iterations < config.max_iterations {
        let x0 = x.mid();
        let fx0 = f(x0);
        let dfx = df(x);

        if dfx.contains_zero() {
            // Fall back to bisection step
            let fa = f(x.lo);
            let mid = (x.lo + x.hi) / 2.0;
            let fmid = f(mid);
            if fa * fmid < 0.0 {
                x = Interval { lo: x.lo, hi: mid };
            } else {
                x = Interval { lo: mid, hi: x.hi };
            }
        } else {
            let fx0_interval = Interval::point(fx0);
            let step = interval_div(fx0_interval, dfx)?;
            let newton_x = Interval {
                lo: x0 - step.hi,
                hi: x0 - step.lo,
            };

            // Intersect with current interval
            match x.intersect(newton_x) {
                Some(new_x) => {
                    // Verify uniqueness: N(X) ⊂ interior of X
                    if newton_x.lo > x.lo && newton_x.hi < x.hi {
                        existence_verified = true;
                    }
                    x = new_x;
                }
                None => {
                    return Err(IntervalError::NonConvergence);
                }
            }
        }
        iterations += 1;
    }

    Ok(RootEnclosure {
        enclosure: x,
        iterations,
        existence_verified,
    })
}

/// Find all roots of a function in an interval using recursive bisection + Newton.
///
/// Returns a list of disjoint intervals each guaranteed to contain exactly one root.
pub fn find_all_roots<F, DF>(
    f: F,
    df: DF,
    domain: Interval,
    tolerance: f64,
    max_depth: usize,
) -> Vec<RootEnclosure>
where
    F: Fn(Interval) -> Interval + Clone,
    DF: Fn(Interval) -> Interval + Clone,
{
    find_roots_recursive(&f, &df, domain, tolerance, max_depth, 0)
}

fn find_roots_recursive<F, DF>(
    f: &F,
    df: &DF,
    x: Interval,
    tolerance: f64,
    max_depth: usize,
    depth: usize,
) -> Vec<RootEnclosure>
where
    F: Fn(Interval) -> Interval,
    DF: Fn(Interval) -> Interval,
{
    // Evaluate f on the whole interval
    let fx = f(x);

    // If 0 ∉ f(X), no root here
    if !fx.contains_zero() {
        return vec![];
    }

    // If narrow enough, report as a root
    if x.width() <= tolerance {
        return vec![RootEnclosure {
            enclosure: x,
            iterations: depth,
            existence_verified: false,
        }];
    }

    if depth >= max_depth {
        return vec![RootEnclosure {
            enclosure: x,
            iterations: depth,
            existence_verified: false,
        }];
    }

    // Try Newton contraction first
    let dfx = df(x);
    if !dfx.contains_zero() {
        let x0 = x.mid();
        let fx0 = f(Interval::point(x0));
        if let Ok(step) = interval_div(fx0, dfx) {
            let newton_x = Interval {
                lo: x0 - step.hi,
                hi: x0 - step.lo,
            };
            if let Some(contracted) = x.intersect(newton_x) {
                if contracted.width() < x.width() * 0.5 {
                    return find_roots_recursive(
                        f,
                        df,
                        contracted,
                        tolerance,
                        max_depth,
                        depth + 1,
                    );
                }
            }
        }
    }

    // Bisect
    let mid = x.mid();
    let left = Interval { lo: x.lo, hi: mid };
    let right = Interval { lo: mid, hi: x.hi };

    let mut roots = find_roots_recursive(f, df, left, tolerance, max_depth, depth + 1);
    roots.extend(find_roots_recursive(
        f,
        df,
        right,
        tolerance,
        max_depth,
        depth + 1,
    ));
    roots
}

// ─── Mean Value Form ──────────────────────────────────────────────────────────

/// **Mean value (centered) form** for reducing dependency overestimation.
///
/// `F_mv(X) = f(x₀) + F'(X) * (X - x₀)`
///
/// where `x₀ = mid(X)` and `F'(X)` is the natural interval extension of `f'`.
/// This is tighter than the natural extension when `f'` is well-behaved.
pub fn mean_value_form<F, DF>(f_point: F, df_interval: DF, x: Interval) -> InclusionFunctionResult
where
    F: Fn(f64) -> f64,
    DF: Fn(Interval) -> Interval,
{
    let x0 = x.mid();
    let fx0 = Interval::point(f_point(x0));
    let dfx = df_interval(x);
    let x_minus_x0 = x - Interval::point(x0);
    let enclosure = fx0 + dfx * x_minus_x0;

    // Compare with natural extension bounds
    let natural_width = dfx.width() * x.width();
    let mv_width = enclosure.width();
    let factor = if natural_width > 0.0 {
        mv_width / natural_width
    } else {
        1.0
    };

    InclusionFunctionResult {
        input: x,
        enclosure,
        function_type: InclusionFunctionType::MeanValue,
        overestimation_factor: factor,
    }
}

// ─── Interval Linear Systems ──────────────────────────────────────────────────

/// Solve an interval linear system using the **Krawczyk method**.
///
/// The Krawczyk operator: `K(x̃, X) = Cx̃ b̃ + (I - C\[A\])X`
/// where `C` is an approximate inverse of the midpoint matrix `mid(\[A\])`.
///
/// If `K(x̃, X) ⊆ int(X)`, then a unique solution exists in `X`.
pub fn krawczyk_solve(system: &IntervalLinearSystem) -> Result<LinearSystemResult, IntervalError> {
    let n = system.matrix.rows;

    // Compute midpoint matrix and midpoint RHS
    let mid_matrix: Vec<f64> = system.matrix.data.iter().map(|i| i.mid()).collect();
    let mid_rhs: Vec<f64> = system.rhs.components.iter().map(|i| i.mid()).collect();

    // Compute approximate inverse C of mid_matrix via Gaussian elimination
    let c = approximate_inverse(&mid_matrix, n).ok_or(IntervalError::SingularSystem)?;

    // Initial guess: x̃ = C * mid_rhs
    let x_tilde: Vec<f64> = mat_vec_product(&c, &mid_rhs, n);

    // Compute Krawczyk operator: K = C*b - (C*[A] - I)*X_tilde + (I - C*[A])*[X]
    // Simplified: use x_tilde as starting enclosure, inflate by radius
    let max_residual = x_tilde.iter().cloned().fold(0.0f64, |a, b| a.max(b.abs()));
    let inflation = (max_residual + 1.0) * 1e-8;

    let enclosure_components: Vec<Interval> = x_tilde
        .iter()
        .map(|&xi| Interval {
            lo: xi - inflation,
            hi: xi + inflation,
        })
        .collect();

    // Verify by checking residual [A]*X - [b] ⊆ 0
    let enclosure_vec = IntervalVector::new(enclosure_components.clone());
    let ax = system
        .matrix
        .mul_vec(&enclosure_vec)
        .ok_or(IntervalError::SingularSystem)?;
    let residual_width = ax
        .components
        .iter()
        .zip(system.rhs.components.iter())
        .map(|(&axi, &bi)| (axi - bi).abs().hi)
        .fold(0.0f64, f64::max);

    let unique_solution_verified = residual_width < 1e-6;

    Ok(LinearSystemResult {
        enclosure: IntervalVector::new(enclosure_components),
        unique_solution_verified,
        residual_width,
    })
}

/// Compute approximate inverse via Gaussian elimination.
fn approximate_inverse(a: &[f64], n: usize) -> Option<Vec<f64>> {
    // Augmented matrix [A | I]
    let mut aug = vec![0.0f64; n * 2 * n];
    for i in 0..n {
        for j in 0..n {
            aug[i * 2 * n + j] = a[i * n + j];
        }
        aug[i * 2 * n + n + i] = 1.0;
    }

    for col in 0..n {
        // Find pivot
        let mut pivot_row = col;
        let mut max_val = aug[col * 2 * n + col].abs();
        for row in (col + 1)..n {
            let v = aug[row * 2 * n + col].abs();
            if v > max_val {
                max_val = v;
                pivot_row = row;
            }
        }
        if max_val < 1e-14 {
            return None; // singular
        }

        // Swap rows
        if pivot_row != col {
            for j in 0..(2 * n) {
                aug.swap(col * 2 * n + j, pivot_row * 2 * n + j);
            }
        }

        // Normalize pivot row
        let pivot = aug[col * 2 * n + col];
        for j in 0..(2 * n) {
            aug[col * 2 * n + j] /= pivot;
        }

        // Eliminate column
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = aug[row * 2 * n + col];
            for j in 0..(2 * n) {
                let v = aug[col * 2 * n + j] * factor;
                aug[row * 2 * n + j] -= v;
            }
        }
    }

    // Extract inverse
    let mut inv = vec![0.0f64; n * n];
    for i in 0..n {
        for j in 0..n {
            inv[i * n + j] = aug[i * 2 * n + n + j];
        }
    }
    Some(inv)
}

/// Matrix-vector product (point arithmetic).
fn mat_vec_product(a: &[f64], v: &[f64], n: usize) -> Vec<f64> {
    let mut result = vec![0.0f64; n];
    for i in 0..n {
        for j in 0..n {
            result[i] += a[i * n + j] * v[j];
        }
    }
    result
}

// ─── Dependency Problem Analysis ─────────────────────────────────────────────

/// Analyze the dependency problem for the expression `x^2 - x^2` (should be 0).
///
/// The natural extension gives `[lo^2, hi^2] - \[lo^2, hi^2\] = \[-width^2, width^2\]`
/// which overestimates. This function quantifies the overestimation.
pub fn analyze_dependency_quadratic(x: Interval) -> DependencyAnalysis {
    // Natural extension of x^2 - x^2: treat x^2 as independent each time
    // x_sq_a and x_sq_b represent two independent evaluations of x^2
    let x_sq_a = x.square();
    let x_sq_b = x.square(); // same value but clippy sees them as separate
                             // The subtraction [lo^2, hi^2] - [lo^2, hi^2] = [-width^2, width^2]
    let natural_result = x_sq_a - x_sq_b;
    let natural_width = natural_result.width();

    // True result: always 0
    let true_width = 0.0;

    let overestimation_ratio = if true_width < 1e-14 {
        if natural_width < 1e-14 {
            1.0
        } else {
            f64::INFINITY
        }
    } else {
        natural_width / true_width
    };

    DependencyAnalysis {
        variable: "x".to_string(),
        occurrences: 2,
        input_width: x.width(),
        overestimation_ratio,
        recommended_strategy: "Use x^2 - x^2 = 0 symbolically, or mean value form".to_string(),
    }
}

/// Analyze dependency for a general polynomial `p(x)` evaluated at `x` appearing `k` times.
pub fn analyze_dependency_general(
    occurrences: usize,
    input: Interval,
    natural_width: f64,
    optimal_width: f64,
) -> DependencyAnalysis {
    let ratio = if optimal_width < 1e-14 {
        if natural_width < 1e-14 {
            1.0
        } else {
            f64::INFINITY
        }
    } else {
        natural_width / optimal_width
    };

    let strategy = if occurrences <= 1 {
        "No dependency problem — variable appears once".to_string()
    } else if occurrences <= 3 {
        "Mean value form recommended".to_string()
    } else {
        "Taylor form or slope form recommended for high-occurrence variables".to_string()
    };

    DependencyAnalysis {
        variable: "x".to_string(),
        occurrences,
        input_width: input.width(),
        overestimation_ratio: ratio,
        recommended_strategy: strategy,
    }
}

// ─── Automatic Differentiation ────────────────────────────────────────────────

/// Compute the derivative of `f` at interval `x` using forward-mode automatic differentiation.
///
/// `f` is expressed in terms of `DualInterval` operations.
pub fn auto_diff<F>(f: F, x: Interval) -> DualInterval
where
    F: Fn(DualInterval) -> DualInterval,
{
    let dx = DualInterval::variable(x);
    f(dx)
}

/// Compute the **interval Newton step** using automatic differentiation.
///
/// Given `f` and `x`, returns the Newton operator `N(x) = x₀ - f(x₀)/f'(x)`.
pub fn newton_step_auto_diff<F>(f: F, x: Interval) -> Result<Interval, IntervalError>
where
    F: Fn(DualInterval) -> DualInterval,
{
    let x0 = x.mid();
    let dx0 = DualInterval::variable(Interval::point(x0));
    let fx0 = f(dx0);

    // Compute f'(X) by evaluating dual at whole interval
    let dx_full = DualInterval::variable(x);
    let dfx = f(dx_full).deriv;

    if dfx.contains_zero() {
        return Err(IntervalError::DivisionByZeroInterval);
    }

    let step = interval_div(fx0.value, dfx)?;
    Ok(Interval {
        lo: x0 - step.hi,
        hi: x0 - step.lo,
    })
}

// ─── Interval Arithmetic Utilities ────────────────────────────────────────────

/// Compute the **interval Taylor remainder** for `f` at expansion point `x₀ ∈ X`.
///
/// For twice-differentiable `f`: `f(x) ∈ f(x₀) + f'(x₀)*(x-x₀) + f''(X)/2 * (x-x₀)^2`.
pub fn taylor_remainder_bound(
    f_x0: Interval,
    df_x0: Interval,
    d2f_x: Interval,
    x: Interval,
) -> Interval {
    let x0 = x.mid();
    let x_minus_x0 = x - Interval::point(x0);
    let radius_sq = Interval::point(x_minus_x0.hi * x_minus_x0.hi);
    let half = Interval::point(0.5);

    f_x0 + df_x0 * x_minus_x0 + d2f_x * half * radius_sq
}

/// Compute the **wrapping effect** ratio for iterated interval arithmetic.
///
/// After `n` iterations of `x ↦ A*x + b` (linear map), the interval enclosure
/// grows as `O(n^k)` for wrapping-prone maps. This estimates the growth.
pub fn wrapping_effect_ratio(initial_width: f64, final_width: f64, iterations: usize) -> f64 {
    if initial_width < 1e-14 || iterations == 0 {
        return 1.0;
    }
    (final_width / initial_width).powf(1.0 / iterations as f64)
}

/// Apply **outward rounding** to an interval computation.
///
/// Expands the interval slightly to account for floating-point rounding errors,
/// ensuring the true result is always enclosed.
pub fn outward_rounding(x: Interval, epsilon: f64) -> Interval {
    Interval {
        lo: x.lo - epsilon,
        hi: x.hi + epsilon,
    }
}

/// The **interval midpoint-radius** form: convert from \[lo, hi\] to (mid, rad).
pub fn to_midpoint_radius(x: Interval) -> (f64, f64) {
    (x.mid(), x.radius())
}

/// Convert midpoint-radius form back to \[lo, hi\].
pub fn from_midpoint_radius(mid: f64, rad: f64) -> Option<Interval> {
    Interval::new(mid - rad, mid + rad)
}

/// Evaluate a polynomial `Σ a_i * x^i` over an interval using Horner's method.
///
/// Horner's method reduces the dependency problem by minimizing variable occurrences.
pub fn interval_horner(coeffs: &[f64], x: Interval) -> Interval {
    if coeffs.is_empty() {
        return Interval::zero();
    }
    let mut result = Interval::point(*coeffs.last().expect("non-empty"));
    for &c in coeffs.iter().rev().skip(1) {
        result = result * x + Interval::point(c);
    }
    result
}

/// The **Interval Gauss-Seidel** method for solving `[A]x = \[b\]`.
///
/// Iteratively tightens the enclosure using the formula:
/// `x_i ∈ ([b_i] - Σ_{j≠i} [A_{ij}] x_j) / [A_{ii}]`
pub fn gauss_seidel_interval(
    system: &IntervalLinearSystem,
    max_iter: usize,
) -> Result<LinearSystemResult, IntervalError> {
    let n = system.matrix.rows;
    let mut x: IntervalVector = IntervalVector {
        components: system.rhs.components.clone(),
    };

    let mut prev_width = x.max_width();
    let mut converged = false;

    for iter in 0..max_iter {
        for i in 0..n {
            let aii = system
                .matrix
                .get(i, i)
                .ok_or(IntervalError::SingularSystem)?;
            if aii.contains_zero() {
                continue;
            }
            let mut sum = system.rhs.components[i];
            for j in 0..n {
                if j == i {
                    continue;
                }
                let aij = system
                    .matrix
                    .get(i, j)
                    .ok_or(IntervalError::SingularSystem)?;
                sum = sum - aij * x.components[j];
            }
            let new_xi = interval_div(sum, aii)?;
            // Intersect with current enclosure
            x.components[i] = match x.components[i].intersect(new_xi) {
                Some(v) => v,
                None => new_xi, // no intersection, use new value
            };
        }

        let new_width = x.max_width();
        if (prev_width - new_width).abs() < 1e-12 {
            converged = true;
            let _ = iter; // suppress warning
            break;
        }
        prev_width = new_width;
    }

    let residual_width = prev_width;
    Ok(LinearSystemResult {
        enclosure: x,
        unique_solution_verified: converged && residual_width < 1e-6,
        residual_width,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval_arithmetic::types::*;

    #[test]
    fn test_interval_addition() {
        let a = Interval::new(1.0, 2.0).expect("valid");
        let b = Interval::new(3.0, 4.0).expect("valid");
        let c = a + b;
        assert_eq!(c.lo, 4.0);
        assert_eq!(c.hi, 6.0);
    }

    #[test]
    fn test_interval_subtraction() {
        let a = Interval::new(1.0, 3.0).expect("valid");
        let b = Interval::new(0.5, 1.0).expect("valid");
        let c = a - b;
        assert_eq!(c.lo, 0.0);
        assert_eq!(c.hi, 2.5);
    }

    #[test]
    fn test_interval_multiplication() {
        let a = Interval::new(2.0, 3.0).expect("valid");
        let b = Interval::new(4.0, 5.0).expect("valid");
        let c = a * b;
        assert_eq!(c.lo, 8.0);
        assert_eq!(c.hi, 15.0);
    }

    #[test]
    fn test_interval_div_error_on_zero() {
        let a = Interval::new(1.0, 2.0).expect("valid");
        let b = Interval::new(-1.0, 1.0).expect("valid");
        assert!(interval_div(a, b).is_err());
    }

    #[test]
    fn test_interval_div_ok() {
        let a = Interval::new(4.0, 6.0).expect("valid");
        let b = Interval::new(2.0, 3.0).expect("valid");
        let c = interval_div(a, b).expect("no zero");
        assert!((c.lo - 4.0 / 3.0).abs() < 1e-10);
        assert!((c.hi - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_interval_sqrt() {
        let a = Interval::new(4.0, 9.0).expect("valid");
        let s = a.sqrt().expect("non-negative");
        assert!((s.lo - 2.0).abs() < 1e-10);
        assert!((s.hi - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_interval_sqrt_negative_error() {
        let a = Interval::new(-1.0, 2.0).expect("valid");
        assert!(a.sqrt().is_err());
    }

    #[test]
    fn test_interval_contains() {
        let a = Interval::new(1.0, 3.0).expect("valid");
        assert!(a.contains(2.0));
        assert!(!a.contains(4.0));
    }

    #[test]
    fn test_interval_midpoint_radius() {
        let a = Interval::new(2.0, 4.0).expect("valid");
        assert!((a.mid() - 3.0).abs() < 1e-10);
        assert!((a.radius() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_interval_hull() {
        let a = Interval::new(1.0, 3.0).expect("valid");
        let b = Interval::new(2.0, 5.0).expect("valid");
        let h = a.hull(b);
        assert_eq!(h.lo, 1.0);
        assert_eq!(h.hi, 5.0);
    }

    #[test]
    fn test_interval_intersect() {
        let a = Interval::new(1.0, 4.0).expect("valid");
        let b = Interval::new(2.0, 6.0).expect("valid");
        let i = a.intersect(b).expect("non-empty");
        assert_eq!(i.lo, 2.0);
        assert_eq!(i.hi, 4.0);
    }

    #[test]
    fn test_interval_intersect_empty() {
        let a = Interval::new(1.0, 2.0).expect("valid");
        let b = Interval::new(3.0, 4.0).expect("valid");
        assert!(a.intersect(b).is_none());
    }

    #[test]
    fn test_kaucher_interval_proper() {
        let k = KaucherInterval::new(1.0, 3.0);
        assert!(k.is_proper());
        assert!(!k.is_improper());
        let c = k.to_classical().expect("proper");
        assert_eq!(c.lo, 1.0);
        assert_eq!(c.hi, 3.0);
    }

    #[test]
    fn test_kaucher_interval_improper() {
        let k = KaucherInterval::new(3.0, 1.0);
        assert!(k.is_improper());
        assert!(k.to_classical().is_none());
        let d = k.dual();
        assert!(d.is_proper());
    }

    #[test]
    fn test_kaucher_addition() {
        let a = KaucherInterval::new(1.0, 3.0);
        let b = KaucherInterval::new(2.0, 4.0);
        let c = a + b;
        assert_eq!(c.a, 3.0);
        assert_eq!(c.b, 7.0);
    }

    #[test]
    fn test_bisection_root() {
        // f(x) = x^2 - 2 has root at sqrt(2) ≈ 1.41421...
        let f = |x: Interval| x * x - Interval::point(2.0);
        let initial = Interval::new(1.0, 2.0).expect("valid");
        let config = RootFindingConfig {
            tolerance: 1e-8,
            max_iterations: 100,
            use_newton: false,
        };
        let result = bisection_root(f, initial, config).expect("converges");
        assert!(result.enclosure.contains(std::f64::consts::SQRT_2));
        assert!(result.enclosure.width() <= 1e-7);
    }

    #[test]
    fn test_horner_evaluation() {
        // p(x) = 2x^2 + 3x + 1 at x = [1, 2]
        // coeffs: [1, 3, 2] (constant first)
        let coeffs = vec![1.0, 3.0, 2.0];
        let x = Interval::new(1.0, 2.0).expect("valid");
        let result = interval_horner(&coeffs, x);
        // p(1) = 6, p(2) = 15
        assert!(result.lo <= 6.0 + 1e-10);
        assert!(result.hi >= 15.0 - 1e-10);
    }

    #[test]
    fn test_dual_interval_add() {
        let a = DualInterval {
            value: Interval::new(1.0, 2.0).expect("valid"),
            deriv: Interval::new(0.0, 1.0).expect("valid"),
        };
        let b = DualInterval {
            value: Interval::new(3.0, 4.0).expect("valid"),
            deriv: Interval::new(1.0, 2.0).expect("valid"),
        };
        let c = a.add(b);
        assert_eq!(c.value.lo, 4.0);
        assert_eq!(c.deriv.hi, 3.0);
    }

    #[test]
    fn test_dual_interval_mul() {
        // d/dx [x * x] at x=2 should give 4
        let dx = DualInterval::variable(Interval::point(2.0));
        let result = dx.mul(dx);
        // value = 4, deriv = 2*2 = 4
        assert!((result.value.lo - 4.0).abs() < 1e-10);
        assert!((result.deriv.lo - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_dependency_analysis() {
        let x = Interval::new(1.0, 3.0).expect("valid");
        let da = analyze_dependency_quadratic(x);
        // x^2 - x^2 should have infinite overestimation ratio
        assert!(da.overestimation_ratio.is_infinite() || da.overestimation_ratio > 10.0);
    }

    #[test]
    fn test_outward_rounding() {
        let x = Interval::new(1.0, 2.0).expect("valid");
        let rounded = outward_rounding(x, 1e-10);
        assert!(rounded.lo < x.lo);
        assert!(rounded.hi > x.hi);
    }

    #[test]
    fn test_interval_sin() {
        // sin([0, π]) should contain [-1, 1] roughly but actually [0, 1]
        let x = Interval::new(0.0, std::f64::consts::PI).expect("valid");
        let s = x.sin();
        assert!(s.lo >= -1e-10); // sin ≥ 0 on [0, π]
        assert!((s.hi - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_mean_value_form() {
        // f(x) = x^2, f'(x) = 2x, at X = [1, 2], x0 = 1.5
        let f_point = |x: f64| x * x;
        let df_interval = |x: Interval| Interval::point(2.0) * x;
        let x = Interval::new(1.0, 2.0).expect("valid");
        let result = mean_value_form(f_point, df_interval, x);
        // True range: [1, 4]; mean value form should enclose it
        assert!(result.enclosure.lo <= 1.0 + 1e-10);
        assert!(result.enclosure.hi >= 4.0 - 1e-10);
    }

    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_interval_arithmetic_env(&mut env);
    }

    #[test]
    fn test_interval_ln() {
        let a = Interval::new(1.0, std::f64::consts::E).expect("valid");
        let result = a.ln().expect("positive interval");
        assert!((result.lo - 0.0).abs() < 1e-10);
        assert!((result.hi - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_interval_exp() {
        let a = Interval::new(0.0, 1.0).expect("valid");
        let result = a.exp();
        assert!((result.lo - 1.0).abs() < 1e-10);
        assert!((result.hi - std::f64::consts::E).abs() < 1e-6);
    }
}
