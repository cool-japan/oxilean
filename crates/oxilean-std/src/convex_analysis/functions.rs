//! Functions for convex analysis.

use super::types::{
    ConvexFunction, FunctionKind, OptimizationResult, ProjectionResult, SeparatingHyperplane,
    SubgradientResult,
};

// ── Internal helpers ─────────────────────────────────────────────────────────

/// Dot product of two slices.
fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Euclidean norm of a slice.
fn norm2(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// Matrix–vector product  w = M * v  where M is n×n.
fn mat_vec(m: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
    m.iter()
        .map(|row| row.iter().zip(v.iter()).map(|(a, b)| a * b).sum())
        .collect()
}

// ── Convex combination test ───────────────────────────────────────────────────

/// Check whether `target` is a convex combination of `points` with given `weights`.
///
/// Requirements: `weights.len() == points.len()`, all weights >= 0 and sum to 1.
/// Returns `false` if dimensions mismatch or weights do not form a valid distribution.
pub fn is_convex_combination(points: &[Vec<f64>], weights: &[f64], target: &[f64]) -> bool {
    if points.is_empty() || weights.len() != points.len() {
        return false;
    }
    // Weights must be non-negative and sum to ~1.
    let weight_sum: f64 = weights.iter().sum();
    if (weight_sum - 1.0).abs() > 1e-9 {
        return false;
    }
    if weights.iter().any(|&w| w < -1e-12) {
        return false;
    }
    let dim = target.len();
    if points.iter().any(|p| p.len() != dim) {
        return false;
    }
    // Compute combination.
    let mut combo = vec![0.0f64; dim];
    for (p, &w) in points.iter().zip(weights.iter()) {
        for (c, pi) in combo.iter_mut().zip(p.iter()) {
            *c += w * pi;
        }
    }
    let diff: f64 = combo
        .iter()
        .zip(target.iter())
        .map(|(c, t)| (c - t) * (c - t))
        .sum::<f64>()
        .sqrt();
    diff < 1e-9
}

// ── Projections ───────────────────────────────────────────────────────────────

/// Project `point` onto the half-space { x : <normal, x> <= offset }.
///
/// If the point already satisfies the constraint the projection is the point
/// itself.  Otherwise the projection is onto the bounding hyperplane.
pub fn project_to_halfspace(point: &[f64], normal: &[f64], offset: f64) -> ProjectionResult {
    let n2: f64 = normal.iter().map(|x| x * x).sum();
    if n2 < 1e-15 {
        return ProjectionResult {
            projected: point.to_vec(),
            distance: 0.0,
        };
    }
    let violation = dot(point, normal) - offset;
    if violation <= 0.0 {
        // Already feasible.
        return ProjectionResult {
            projected: point.to_vec(),
            distance: 0.0,
        };
    }
    let lambda = violation / n2;
    let projected: Vec<f64> = point
        .iter()
        .zip(normal.iter())
        .map(|(xi, ni)| xi - lambda * ni)
        .collect();
    let distance = (lambda * lambda * n2).sqrt();
    ProjectionResult {
        projected,
        distance,
    }
}

/// Project `point` onto the probability simplex
/// Δ = { x : x_i >= 0, sum x_i = 1 }
/// using the O(n log n) algorithm of Duchi et al. (2008).
pub fn project_to_simplex(point: &[f64]) -> ProjectionResult {
    let n = point.len();
    if n == 0 {
        return ProjectionResult {
            projected: vec![],
            distance: 0.0,
        };
    }
    let mut u: Vec<f64> = point.to_vec();
    u.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let mut cssv = 0.0f64;
    let mut rho = 0usize;
    for (j, &uj) in u.iter().enumerate() {
        cssv += uj;
        let theta = (cssv - 1.0) / (j as f64 + 1.0);
        if uj > theta {
            rho = j;
        }
    }
    let cssv_rho: f64 = u[..=rho].iter().sum();
    let theta = (cssv_rho - 1.0) / (rho as f64 + 1.0);
    let projected: Vec<f64> = point.iter().map(|xi| (xi - theta).max(0.0)).collect();
    let distance = norm2(
        &point
            .iter()
            .zip(projected.iter())
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>(),
    );
    ProjectionResult {
        projected,
        distance,
    }
}

/// Project `point` onto the Euclidean ball B(center, radius).
pub fn project_to_ball(point: &[f64], center: &[f64], radius: f64) -> ProjectionResult {
    let diff: Vec<f64> = point
        .iter()
        .zip(center.iter())
        .map(|(p, c)| p - c)
        .collect();
    let d = norm2(&diff);
    if d <= radius {
        return ProjectionResult {
            projected: point.to_vec(),
            distance: 0.0,
        };
    }
    let scale = radius / d;
    let projected: Vec<f64> = center
        .iter()
        .zip(diff.iter())
        .map(|(c, di)| c + scale * di)
        .collect();
    let distance = d - radius;
    ProjectionResult {
        projected,
        distance,
    }
}

// ── Supporting hyperplane ─────────────────────────────────────────────────────

/// Compute the supporting hyperplane of a convex set at `point` given the
/// outward normal direction `set_normal`.
///
/// The hyperplane is H = { x : <set_normal, x> = <set_normal, point> }.
pub fn supporting_hyperplane(point: &[f64], set_normal: &[f64]) -> SeparatingHyperplane {
    let offset = dot(point, set_normal);
    SeparatingHyperplane {
        normal: set_normal.to_vec(),
        offset,
    }
}

// ── Convex hull (2-D, Graham scan) ───────────────────────────────────────────

/// Compute the convex hull of a set of 2-D points using the Graham scan
/// algorithm.  Returns vertices in counter-clockwise order.
pub fn convex_hull_2d(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    if points.len() < 3 {
        let mut out = points.to_vec();
        out.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        });
        return out;
    }
    // Find pivot: lowest y, then leftmost x.
    let pivot_idx = points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.1.partial_cmp(&b.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        })
        .map(|(i, _)| i)
        .unwrap_or(0);
    let pivot = points[pivot_idx];

    let mut rest: Vec<(f64, f64)> = points
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != pivot_idx)
        .map(|(_, &p)| p)
        .collect();

    // Sort by polar angle with respect to pivot.
    rest.sort_by(|&a, &b| {
        let cross = cross2(pivot, a, b);
        if cross.abs() < 1e-12 {
            let da = dist2(pivot, a);
            let db = dist2(pivot, b);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        } else if cross > 0.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut stack: Vec<(f64, f64)> = vec![pivot];
    for &p in &rest {
        while stack.len() >= 2 {
            let n = stack.len();
            let c = cross2(stack[n - 2], stack[n - 1], p);
            if c <= 0.0 {
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(p);
    }
    stack
}

/// 2-D cross product (o→a) × (o→b).
fn cross2(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

/// Squared Euclidean distance between two 2-D points.
fn dist2(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)
}

// ── Subgradients ──────────────────────────────────────────────────────────────

/// Compute a subgradient of the quadratic f(x) = (1/2) x^T Q x + b^T x + c
/// at point `x`.
///
/// The gradient is ∇f(x) = Q x + b (quadratics are differentiable everywhere).
pub fn subgradient_quadratic(q: &[Vec<f64>], b: &[f64], c: f64, x: &[f64]) -> SubgradientResult {
    let qx = mat_vec(q, x);
    let subgradient: Vec<f64> = qx.iter().zip(b.iter()).map(|(qi, bi)| qi + bi).collect();
    let value: f64 = 0.5 * dot(x, &qx) + dot(b, x) + c;
    SubgradientResult {
        point: x.to_vec(),
        subgradient,
        value,
    }
}

/// Evaluate a `FunctionKind` at point `x`, returning (value, subgradient).
fn eval_with_subgradient(func_kind: &FunctionKind, x: &[f64]) -> (f64, Vec<f64>) {
    let n = x.len();
    match func_kind {
        FunctionKind::Quadratic { q, b, c } => {
            let qx = mat_vec(q, x);
            let value = 0.5 * dot(x, &qx) + dot(b, x) + c;
            let grad: Vec<f64> = qx.iter().zip(b.iter()).map(|(qi, bi)| qi + bi).collect();
            (value, grad)
        }
        FunctionKind::Linear { a, b } => {
            let value = dot(a, x) + b;
            (value, a.clone())
        }
        FunctionKind::Norm { p } => {
            let p = *p;
            if (p - 2.0).abs() < 1e-12 {
                let v = norm2(x);
                let grad: Vec<f64> = if v < 1e-15 {
                    vec![0.0; n]
                } else {
                    x.iter().map(|xi| xi / v).collect()
                };
                (v, grad)
            } else if (p - 1.0).abs() < 1e-12 {
                let v: f64 = x.iter().map(|xi| xi.abs()).sum();
                let grad: Vec<f64> = x
                    .iter()
                    .map(|xi| {
                        if *xi > 0.0 {
                            1.0
                        } else if *xi < 0.0 {
                            -1.0
                        } else {
                            0.0
                        }
                    })
                    .collect();
                (v, grad)
            } else {
                // General Lp norm via chain rule.
                let v: f64 = x
                    .iter()
                    .map(|xi| xi.abs().powf(p))
                    .sum::<f64>()
                    .powf(1.0 / p);
                let grad: Vec<f64> = if v < 1e-15 {
                    vec![0.0; n]
                } else {
                    x.iter()
                        .map(|xi| xi.signum() * xi.abs().powf(p - 1.0) * v.powf(1.0 - p))
                        .collect()
                };
                (v, grad)
            }
        }
        FunctionKind::Indicator => (0.0, vec![0.0; n]),
        FunctionKind::MaxAffine { pieces } => {
            let mut best_val = f64::NEG_INFINITY;
            let mut best_idx = 0;
            for (i, (a, b)) in pieces.iter().enumerate() {
                let val = dot(a, x) + b;
                if val > best_val {
                    best_val = val;
                    best_idx = i;
                }
            }
            let subg = if pieces.is_empty() {
                vec![0.0; n]
            } else {
                pieces[best_idx].0.clone()
            };
            (best_val, subg)
        }
    }
}

/// Subgradient descent minimisation.
///
/// Uses the step sizes `step_sizes\[k\]` (cycled if shorter than `max_iter`).
/// Tracks the best function value seen and returns that point.
pub fn subgradient_descent(
    func_kind: &FunctionKind,
    x0: Vec<f64>,
    step_sizes: &[f64],
    max_iter: usize,
) -> OptimizationResult {
    if step_sizes.is_empty() || x0.is_empty() {
        return OptimizationResult {
            optimal_point: x0,
            optimal_value: f64::INFINITY,
            iterations: 0,
            converged: false,
        };
    }
    let n = x0.len();
    let mut x = x0;
    let (mut best_val, _) = eval_with_subgradient(func_kind, &x);
    let mut best_x = x.clone();

    for k in 0..max_iter {
        let step = step_sizes[k % step_sizes.len()];
        let (val, g) = eval_with_subgradient(func_kind, &x);
        let g_norm = norm2(&g);
        if g_norm < 1e-12 {
            return OptimizationResult {
                optimal_point: best_x,
                optimal_value: best_val,
                iterations: k + 1,
                converged: true,
            };
        }
        if val < best_val {
            best_val = val;
            best_x = x.clone();
        }
        // x_{k+1} = x_k - step * g / ||g||
        x = x
            .iter()
            .zip(g.iter())
            .map(|(xi, gi)| xi - step * gi / g_norm)
            .collect();
        let _ = n; // suppress unused warning
    }
    let (final_val, _) = eval_with_subgradient(func_kind, &x);
    if final_val < best_val {
        best_val = final_val;
        best_x = x;
    }
    OptimizationResult {
        optimal_point: best_x,
        optimal_value: best_val,
        iterations: max_iter,
        converged: false,
    }
}

// ── Proximal gradient ─────────────────────────────────────────────────────────

/// Evaluate the proximal operator of `prox_kind` with step `t` at point `v`.
///
/// Only `Norm { p: 1.0 }` (soft thresholding) and `Indicator` (projection
/// onto simplex) are supported analytically here; other kinds fall back to a
/// gradient step.
fn prox_step(prox_kind: &FunctionKind, v: &[f64], t: f64) -> Vec<f64> {
    match prox_kind {
        FunctionKind::Norm { p } if (p - 1.0).abs() < 1e-12 => {
            // Soft-thresholding: prox_{t||.||_1}(v)_i = sign(v_i) max(|v_i|-t, 0)
            v.iter()
                .map(|vi| vi.signum() * (vi.abs() - t).max(0.0))
                .collect()
        }
        FunctionKind::Indicator => {
            // Projection onto probability simplex.
            project_to_simplex(v).projected
        }
        FunctionKind::Linear { a, b: _ } => {
            // prox_{t(a^T x + b)}(v) = v - t*a  (unconstrained linear prox)
            v.iter().zip(a.iter()).map(|(vi, ai)| vi - t * ai).collect()
        }
        _ => {
            // Generic: gradient step as fallback (for quadratic / max-affine).
            let (_, g) = eval_with_subgradient(prox_kind, v);
            v.iter().zip(g.iter()).map(|(vi, gi)| vi - t * gi).collect()
        }
    }
}

/// Proximal gradient method:
///   x_{k+1} = prox_{step * prox_kind}( x_k - step * ∇func_kind(x_k) )
///
/// Suitable for minimising `func_kind + prox_kind` when `func_kind` is smooth.
pub fn proximal_gradient(
    func_kind: &FunctionKind,
    prox_kind: &FunctionKind,
    x0: Vec<f64>,
    step: f64,
    max_iter: usize,
) -> OptimizationResult {
    if x0.is_empty() || step <= 0.0 {
        return OptimizationResult {
            optimal_point: x0,
            optimal_value: f64::INFINITY,
            iterations: 0,
            converged: false,
        };
    }
    let mut x = x0;
    let tol = 1e-8;

    for k in 0..max_iter {
        let (_, g) = eval_with_subgradient(func_kind, &x);
        // Gradient step on smooth part.
        let v: Vec<f64> = x
            .iter()
            .zip(g.iter())
            .map(|(xi, gi)| xi - step * gi)
            .collect();
        // Proximal step on non-smooth part.
        let x_new = prox_step(prox_kind, &v, step);
        // Convergence check.
        let diff = norm2(
            &x.iter()
                .zip(x_new.iter())
                .map(|(a, b)| a - b)
                .collect::<Vec<_>>(),
        );
        x = x_new;
        if diff < tol {
            let (val, _) = eval_with_subgradient(func_kind, &x);
            return OptimizationResult {
                optimal_point: x,
                optimal_value: val,
                iterations: k + 1,
                converged: true,
            };
        }
    }
    let (val, _) = eval_with_subgradient(func_kind, &x);
    OptimizationResult {
        optimal_point: x,
        optimal_value: val,
        iterations: max_iter,
        converged: false,
    }
}

// ── Fenchel conjugate ─────────────────────────────────────────────────────────

/// Evaluate f*(y) for the linear function f(x) = a^T x + b.
///
/// The conjugate of a linear function is the indicator of {a}:
///   f*(y) = 0  if y = a,  +∞ otherwise.
/// Here we return 0 when ||y - a|| < 1e-9, else +∞.
pub fn fenchel_conjugate_linear(a: &[f64], b: f64, y: &[f64]) -> f64 {
    if a.len() != y.len() {
        return f64::INFINITY;
    }
    let diff = norm2(
        &a.iter()
            .zip(y.iter())
            .map(|(ai, yi)| yi - ai)
            .collect::<Vec<_>>(),
    );
    if diff < 1e-9 {
        // f*(y) = sup_x { <y, x> - a^T x - b } = sup_x { <y-a, x> } - b = 0 - b = -b
        -b
    } else {
        f64::INFINITY
    }
}

// ── Moreau envelope ───────────────────────────────────────────────────────────

/// Compute the Moreau envelope:
///   e_λ f(x) = min_u { f(u) + (1 / 2λ) ||u - x||^2 }
///
/// Uses a simple gradient descent on the composite objective.
/// Returns an approximation for general `FunctionKind`.
pub fn moreau_envelope(point: &[f64], func: &ConvexFunction, lambda: f64) -> f64 {
    if lambda <= 0.0 {
        let (val, _) = eval_with_subgradient(&func.kind, point);
        return val;
    }
    // Use prox_step as the exact minimiser for supported kinds.
    let u = prox_step(&func.kind, point, lambda);
    let (fu, _) = eval_with_subgradient(&func.kind, &u);
    let diff = norm2(
        &u.iter()
            .zip(point.iter())
            .map(|(ui, xi)| ui - xi)
            .collect::<Vec<_>>(),
    );
    fu + diff * diff / (2.0 * lambda)
}

// ── Bregman divergence ────────────────────────────────────────────────────────

/// Compute the KL divergence as a Bregman divergence generated by
/// φ(p) = sum_i p_i log(p_i):
///   D_φ(p || q) = sum_i p_i log(p_i / q_i) - p_i + q_i.
///
/// Both `p` and `q` must have the same length and non-negative entries.
/// Returns +∞ when q_i = 0 and p_i > 0.
pub fn bregman_divergence(p: &[f64], q: &[f64]) -> f64 {
    if p.len() != q.len() {
        return f64::INFINITY;
    }
    p.iter()
        .zip(q.iter())
        .map(|(&pi, &qi)| {
            if pi < 0.0 || qi < 0.0 {
                return f64::INFINITY;
            }
            if pi == 0.0 {
                // 0 * log(0/q) = 0; -0 + q_i = q_i >= 0.
                return qi;
            }
            if qi == 0.0 {
                return f64::INFINITY;
            }
            pi * (pi / qi).ln() - pi + qi
        })
        .fold(0.0f64, |acc, v| {
            if v.is_infinite() {
                f64::INFINITY
            } else {
                acc + v
            }
        })
}

// ── KKT conditions ────────────────────────────────────────────────────────────

/// Check first-order KKT (stationarity) conditions:
///   grad + sum_i mu_i * a_i = 0
///
/// where `active_constraints\[i\]` is the gradient of the i-th active inequality
/// constraint and `multipliers\[i\] >= 0` are the Lagrange multipliers.
pub fn check_kkt_conditions(
    grad: &[f64],
    active_constraints: &[Vec<f64>],
    multipliers: &[f64],
) -> bool {
    if active_constraints.len() != multipliers.len() {
        return false;
    }
    // Multipliers must be non-negative.
    if multipliers.iter().any(|&mu| mu < -1e-9) {
        return false;
    }
    let n = grad.len();
    let mut residual = grad.to_vec();
    for (ai, &mu) in active_constraints.iter().zip(multipliers.iter()) {
        if ai.len() != n {
            return false;
        }
        for (r, aij) in residual.iter_mut().zip(ai.iter()) {
            *r += mu * aij;
        }
    }
    norm2(&residual) < 1e-8
}

// ── Spectral estimates ────────────────────────────────────────────────────────

/// Estimate the minimum eigenvalue of Q via the Gershgorin circle theorem.
///
/// For each row i: λ_min >= Q\[i\]\[i\] - sum_{j≠i} |Q\[i\]\[j\]|.
/// Returns the maximum of these lower bounds.
pub fn strong_convexity_parameter(q: &[Vec<f64>]) -> f64 {
    let n = q.len();
    if n == 0 {
        return 0.0;
    }
    q.iter()
        .enumerate()
        .map(|(i, row)| {
            let radius: f64 = row
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, qij)| qij.abs())
                .sum();
            let diag = if i < row.len() { row[i] } else { 0.0 };
            diag - radius
        })
        .fold(f64::INFINITY, f64::min)
}

/// Estimate the maximum eigenvalue of Q via the Gershgorin circle theorem.
///
/// For each row i: λ_max <= Q\[i\]\[i\] + sum_{j≠i} |Q\[i\]\[j\]|.
/// Returns the minimum of these upper bounds.
pub fn lipschitz_constant_quadratic(q: &[Vec<f64>]) -> f64 {
    let n = q.len();
    if n == 0 {
        return 0.0;
    }
    q.iter()
        .enumerate()
        .map(|(i, row)| {
            let radius: f64 = row
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, qij)| qij.abs())
                .sum();
            let diag = if i < row.len() { row[i] } else { 0.0 };
            diag + radius
        })
        .fold(f64::NEG_INFINITY, f64::max)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    // 1. Convex combination: barycenter of triangle vertices.
    #[test]
    fn test_is_convex_combination_centroid() {
        let pts = vec![vec![0.0, 0.0], vec![3.0, 0.0], vec![0.0, 3.0]];
        let w = vec![1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0];
        let target = vec![1.0, 1.0];
        assert!(is_convex_combination(&pts, &w, &target));
    }

    // 2. Convex combination: wrong weights (negative).
    #[test]
    fn test_is_convex_combination_negative_weight() {
        let pts = vec![vec![0.0], vec![1.0]];
        let w = vec![-0.1, 1.1];
        assert!(!is_convex_combination(&pts, &w, &[0.9]));
    }

    // 3. Project onto half-space: point outside.
    #[test]
    fn test_project_to_halfspace_outside() {
        // Half-space x1 <= 1 (normal=(1,0), offset=1), point=(3,0).
        let proj = project_to_halfspace(&[3.0, 0.0], &[1.0, 0.0], 1.0);
        assert!(approx(proj.projected[0], 1.0, 1e-9));
        assert!(approx(proj.projected[1], 0.0, 1e-9));
        assert!(approx(proj.distance, 2.0, 1e-9));
    }

    // 4. Project onto half-space: point inside.
    #[test]
    fn test_project_to_halfspace_inside() {
        let proj = project_to_halfspace(&[0.5, 0.0], &[1.0, 0.0], 1.0);
        assert!(approx(proj.projected[0], 0.5, 1e-9));
        assert!(approx(proj.distance, 0.0, 1e-9));
    }

    // 5. Project onto simplex: uniform vector.
    #[test]
    fn test_project_to_simplex_uniform() {
        let proj = project_to_simplex(&[0.25, 0.25, 0.25, 0.25]);
        let sum: f64 = proj.projected.iter().sum();
        assert!(approx(sum, 1.0, 1e-9));
        assert!(approx(proj.distance, 0.0, 1e-6));
    }

    // 6. Project onto simplex: outside.
    #[test]
    fn test_project_to_simplex_outside() {
        let proj = project_to_simplex(&[3.0, 0.0, 0.0]);
        let sum: f64 = proj.projected.iter().sum();
        assert!(approx(sum, 1.0, 1e-9));
        assert!(proj.projected.iter().all(|&xi| xi >= -1e-12));
    }

    // 7. Project onto ball: outside.
    #[test]
    fn test_project_to_ball_outside() {
        let proj = project_to_ball(&[5.0, 0.0], &[0.0, 0.0], 1.0);
        assert!(approx(norm2(&proj.projected), 1.0, 1e-9));
        assert!(approx(proj.distance, 4.0, 1e-9));
    }

    // 8. Project onto ball: inside.
    #[test]
    fn test_project_to_ball_inside() {
        let proj = project_to_ball(&[0.5, 0.0], &[0.0, 0.0], 1.0);
        assert!(approx(proj.projected[0], 0.5, 1e-9));
        assert!(approx(proj.distance, 0.0, 1e-9));
    }

    // 9. Supporting hyperplane offset equals dot(normal, point).
    #[test]
    fn test_supporting_hyperplane_offset() {
        let pt = vec![1.0, 2.0, 3.0];
        let n = vec![1.0, 0.0, 0.0];
        let h = supporting_hyperplane(&pt, &n);
        assert!(approx(h.offset, 1.0, 1e-12));
    }

    // 10. Convex hull of square: 4 corners.
    #[test]
    fn test_convex_hull_square() {
        let pts = vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
            (0.5, 0.5), // interior
        ];
        let hull = convex_hull_2d(&pts);
        assert_eq!(hull.len(), 4);
    }

    // 11. Subgradient of quadratic (identity Q, zero b) at x=(1,0) is (1,0).
    #[test]
    fn test_subgradient_quadratic_identity() {
        let q = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![0.0, 0.0];
        let x = vec![1.0, 0.0];
        let res = subgradient_quadratic(&q, &b, 0.0, &x);
        assert!(approx(res.subgradient[0], 1.0, 1e-12));
        assert!(approx(res.subgradient[1], 0.0, 1e-12));
        assert!(approx(res.value, 0.5, 1e-12));
    }

    // 12. Subgradient descent on f(x)=x^2 converges towards 0.
    #[test]
    fn test_subgradient_descent_quadratic() {
        let q = vec![vec![2.0]]; // f(x) = x^2, ∇f = 2x
        let b = vec![0.0];
        let kind = FunctionKind::Quadratic { q, b, c: 0.0 };
        let steps: Vec<f64> = (1..=200).map(|k| 1.0 / (k as f64).sqrt()).collect();
        let res = subgradient_descent(&kind, vec![10.0], &steps, 200);
        assert!(res.optimal_value < 1.0, "value={}", res.optimal_value);
    }

    // 13. Proximal gradient on f(x)=||x||^2, prox=indicator (simplex).
    #[test]
    fn test_proximal_gradient_simplex() {
        let q = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
        let b = vec![-2.0, -2.0];
        let func = FunctionKind::Quadratic { q, b, c: 2.0 };
        let prox = FunctionKind::Indicator;
        let res = proximal_gradient(&func, &prox, vec![0.5, 0.5], 0.01, 1000);
        // Solution is on the simplex
        let sum: f64 = res.optimal_point.iter().sum();
        assert!(approx(sum, 1.0, 1e-3));
    }

    // 14. Fenchel conjugate of linear: y = a gives finite value.
    #[test]
    fn test_fenchel_conjugate_linear_at_a() {
        let a = vec![1.0, 2.0];
        let val = fenchel_conjugate_linear(&a, 3.0, &[1.0, 2.0]);
        assert!(approx(val, -3.0, 1e-9));
    }

    // 15. Fenchel conjugate of linear: y != a gives +inf.
    #[test]
    fn test_fenchel_conjugate_linear_away() {
        let a = vec![1.0, 0.0];
        let val = fenchel_conjugate_linear(&a, 0.0, &[0.0, 1.0]);
        assert!(val.is_infinite());
    }

    // 16. Moreau envelope <= f(x) for Norm{1}.
    #[test]
    fn test_moreau_envelope_norm1() {
        use super::super::types::{ConvexSet, SetDescription};
        let domain = ConvexSet::new(
            2,
            SetDescription::HalfSpace {
                normal: vec![0.0, 0.0],
                offset: 1e9,
            },
        );
        let func = ConvexFunction::new(domain, FunctionKind::Norm { p: 1.0 });
        let pt = vec![2.0, -1.0];
        let env = moreau_envelope(&pt, &func, 0.5);
        let (fx, _) = eval_with_subgradient(&func.kind, &pt);
        assert!(env <= fx + 1e-9);
    }

    // 17. Bregman divergence: KL(p || p) = 0.
    #[test]
    fn test_bregman_divergence_self() {
        let p = vec![0.3, 0.5, 0.2];
        let d = bregman_divergence(&p, &p);
        assert!(approx(d, 0.0, 1e-9));
    }

    // 18. Bregman divergence: non-negative.
    #[test]
    fn test_bregman_divergence_nonneg() {
        let p = vec![0.4, 0.6];
        let q = vec![0.2, 0.8];
        let d = bregman_divergence(&p, &q);
        assert!(d >= -1e-12);
    }

    // 19. KKT: zero gradient and no constraints satisfied trivially.
    #[test]
    fn test_kkt_trivial() {
        let grad = vec![0.0, 0.0];
        assert!(check_kkt_conditions(&grad, &[], &[]));
    }

    // 20. KKT: stationarity with one active constraint.
    #[test]
    fn test_kkt_one_active_constraint() {
        // grad = [1.0] + mu * [-1.0] = 0  =>  mu = 1.0
        let grad = vec![1.0];
        let active = vec![vec![-1.0]];
        let multipliers = vec![1.0];
        assert!(check_kkt_conditions(&grad, &active, &multipliers));
    }

    // 21. Strong convexity parameter of 2I is 2.
    #[test]
    fn test_strong_convexity_parameter() {
        let q = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
        let m = strong_convexity_parameter(&q);
        assert!(approx(m, 2.0, 1e-12));
    }

    // 22. Lipschitz constant of 5I is 5.
    #[test]
    fn test_lipschitz_constant_quadratic() {
        let q = vec![vec![5.0, 0.0], vec![0.0, 5.0]];
        let l = lipschitz_constant_quadratic(&q);
        assert!(approx(l, 5.0, 1e-12));
    }

    // 23. Convex hull of collinear points.
    #[test]
    fn test_convex_hull_collinear() {
        let pts = vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)];
        let hull = convex_hull_2d(&pts);
        // Collinear: 3 points, all on boundary of degenerate hull.
        assert!(!hull.is_empty());
    }

    // 24. MaxAffine subgradient is a piece's normal.
    #[test]
    fn test_eval_max_affine() {
        let pieces = vec![(vec![1.0, 0.0], 0.0), (vec![0.0, 1.0], 0.0)];
        let kind = FunctionKind::MaxAffine { pieces };
        let (val, g) = eval_with_subgradient(&kind, &[3.0, 1.0]);
        assert!(approx(val, 3.0, 1e-12));
        assert!(approx(g[0], 1.0, 1e-12));
        assert!(approx(g[1], 0.0, 1e-12));
    }
}
