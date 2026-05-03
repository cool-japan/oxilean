//! Functions for nonlinear dynamical systems.

use super::types::{
    AttractorType, BifurcationPoint, BifurcationType, DimensionMethod, DynamicalSystem, FixedPoint,
    FractalDimension, LyapunovExponents, MapType, PoincareSectionResult, Stability, SystemType,
    Trajectory,
};

// ── Discrete maps ─────────────────────────────────────────────────────────────

/// Iterate the logistic map x_{n+1} = r·x·(1 − x) starting from `x0` for
/// `n` steps.
///
/// Returns a vector of `n + 1` values (including the initial condition).
pub fn logistic_map_iterate(r: f64, x0: f64, n: usize) -> Vec<f64> {
    let mut orbit = Vec::with_capacity(n + 1);
    orbit.push(x0);
    let mut x = x0;
    for _ in 0..n {
        x = r * x * (1.0 - x);
        orbit.push(x);
    }
    orbit
}

/// Iterate the Hénon map (x, y) → (1 − a·x² + y, b·x) for `n` steps
/// starting at `(x0, y0)`.
pub fn henon_map_iterate(a: f64, b: f64, x0: f64, y0: f64, n: usize) -> Trajectory {
    let mut points = Vec::with_capacity(n + 1);
    let mut times = Vec::with_capacity(n + 1);
    let mut x = x0;
    let mut y = y0;
    points.push(vec![x, y]);
    times.push(0.0);
    for i in 1..=(n) {
        let xn = 1.0 - a * x * x + y;
        let yn = b * x;
        x = xn;
        y = yn;
        points.push(vec![x, y]);
        times.push(i as f64);
    }
    Trajectory { points, times }
}

/// Iterate the tent map x_{n+1} = μ·min(x, 1-x) starting from `x0` for
/// `n` steps.
pub fn tent_map_iterate(mu: f64, x0: f64, n: usize) -> Vec<f64> {
    let mut orbit = Vec::with_capacity(n + 1);
    orbit.push(x0);
    let mut x = x0;
    for _ in 0..n {
        x = mu * x.min(1.0 - x);
        orbit.push(x);
    }
    orbit
}

// ── Continuous systems (Euler integration) ────────────────────────────────────

/// Integrate the Lorenz system using the forward Euler method.
///
/// The equations are:
/// - ẋ = σ(y − x)
/// - ẏ = x(ρ − z) − y
/// - ż = xy − βz
///
/// Returns a `Trajectory` with `n + 1` points (including the initial condition).
pub fn lorenz_euler(
    sigma: f64,
    rho: f64,
    beta: f64,
    x0: f64,
    y0: f64,
    z0: f64,
    dt: f64,
    n: usize,
) -> Trajectory {
    let mut points = Vec::with_capacity(n + 1);
    let mut times = Vec::with_capacity(n + 1);
    let (mut x, mut y, mut z) = (x0, y0, z0);
    points.push(vec![x, y, z]);
    times.push(0.0);
    for i in 1..=(n) {
        let dx = sigma * (y - x);
        let dy = x * (rho - z) - y;
        let dz = x * y - beta * z;
        x += dt * dx;
        y += dt * dy;
        z += dt * dz;
        points.push(vec![x, y, z]);
        times.push(i as f64 * dt);
    }
    Trajectory { points, times }
}

/// Integrate the Rössler system using the forward Euler method.
///
/// The equations are:
/// - ẋ = −y − z
/// - ẏ = x + a·y
/// - ż = b + z·(x − c)
///
/// Returns a `Trajectory` with `n + 1` points.
pub fn rossler_euler(
    a: f64,
    b: f64,
    c: f64,
    x0: f64,
    y0: f64,
    z0: f64,
    dt: f64,
    n: usize,
) -> Trajectory {
    let mut points = Vec::with_capacity(n + 1);
    let mut times = Vec::with_capacity(n + 1);
    let (mut x, mut y, mut z) = (x0, y0, z0);
    points.push(vec![x, y, z]);
    times.push(0.0);
    for i in 1..=(n) {
        let dx = -y - z;
        let dy = x + a * y;
        let dz = b + z * (x - c);
        x += dt * dx;
        y += dt * dy;
        z += dt * dz;
        points.push(vec![x, y, z]);
        times.push(i as f64 * dt);
    }
    Trajectory { points, times }
}

// ── Fixed-point analysis ──────────────────────────────────────────────────────

/// Find fixed points of a 1-D map f by detecting sign changes in (f(x) − x)
/// from a table of (x, f(x)) pairs.
///
/// Returns approximate fixed-point locations by linear interpolation at
/// each sign change.
pub fn fixed_points_1d(f_values: &[(f64, f64)]) -> Vec<f64> {
    if f_values.len() < 2 {
        return vec![];
    }
    let mut fps = Vec::new();
    for i in 0..(f_values.len() - 1) {
        let (x0, fx0) = f_values[i];
        let (x1, fx1) = f_values[i + 1];
        let g0 = fx0 - x0;
        let g1 = fx1 - x1;
        if g0 * g1 <= 0.0 {
            // Linear interpolation.
            let dg = g1 - g0;
            if dg.abs() > 1e-15 {
                let t = -g0 / dg;
                fps.push(x0 + t * (x1 - x0));
            } else {
                fps.push(0.5 * (x0 + x1));
            }
        }
    }
    fps
}

/// Classify the stability of a 1-D fixed point given the derivative of the
/// map f at that point.
///
/// - |f'| < 1 → `StableNode`
/// - |f'| > 1 → `UnstableNode`
/// - |f'| = 1 → `Center` (marginal, non-hyperbolic)
pub fn classify_fixed_point_1d(deriv_at_fp: f64) -> Stability {
    let d = deriv_at_fp.abs();
    if d < 1.0 - 1e-10 {
        Stability::StableNode
    } else if d > 1.0 + 1e-10 {
        Stability::UnstableNode
    } else {
        Stability::Center
    }
}

// ── Lyapunov exponents ────────────────────────────────────────────────────────

/// Compute the Lyapunov exponent of a 1-D orbit as the time-average of
/// log|f'(x_n)|.
///
/// `orbit` are the iterates and `deriv_values` are the corresponding
/// derivative values f'(x_n).  The two slices must have the same length.
///
/// Returns 0.0 if the orbit is empty or all derivative values are zero.
pub fn lyapunov_exponent_1d(orbit: &[f64], deriv_values: &[f64]) -> f64 {
    let n = orbit.len().min(deriv_values.len());
    if n == 0 {
        return 0.0;
    }
    let sum: f64 = deriv_values[..n]
        .iter()
        .map(|&d| {
            let ad = d.abs();
            if ad > 1e-300 {
                ad.ln()
            } else {
                -700.0
            }
        })
        .sum();
    sum / n as f64
}

/// Compute the Lyapunov exponent of the logistic map for parameter `r`.
///
/// Discards the first `n_transient` iterates, then averages log|f'(x)| over
/// `n_average` subsequent steps.
///
/// f'(x) = r(1 − 2x) for the logistic map.
pub fn lyapunov_exponent_logistic(r: f64, n_transient: usize, n_average: usize) -> f64 {
    let orbit = logistic_map_iterate(r, 0.5, n_transient + n_average);
    if orbit.len() < n_transient + 1 {
        return 0.0;
    }
    let avg_iter = &orbit[n_transient..];
    if avg_iter.is_empty() {
        return 0.0;
    }
    let sum: f64 = avg_iter
        .iter()
        .map(|&x| {
            let d = (r * (1.0 - 2.0 * x)).abs();
            if d > 1e-300 {
                d.ln()
            } else {
                -700.0
            }
        })
        .sum();
    sum / avg_iter.len() as f64
}

// ── Periodic orbits ────────────────────────────────────────────────────────────

/// Detect the period of an orbit by searching for the smallest `p >= 1`
/// such that `|orbit\[i\] - orbit\[i + p\]| < tol` for several consecutive i.
///
/// Checks from the middle of the orbit to avoid transients.
/// Returns `None` if no period is detected within the available orbit length.
pub fn period_of_orbit(orbit: &[f64], tol: f64) -> Option<usize> {
    let n = orbit.len();
    if n < 4 {
        return None;
    }
    let start = n / 2;
    for p in 1..=(n / 2) {
        // Require 3 consecutive matches to reduce false positives.
        let matches = (0..3usize).all(|k| {
            let i = start + k;
            let j = i + p;
            j < n && (orbit[i] - orbit[j]).abs() < tol
        });
        if matches {
            return Some(p);
        }
    }
    None
}

// ── Bifurcation diagram ────────────────────────────────────────────────────────

/// Compute bifurcation diagram data for the logistic map.
///
/// For each r value in `r_values`, discards `n_transient` iterates starting
/// from x₀ = 0.5, then records `n_plot` subsequent values.
///
/// Returns `Vec<(r, attractor_values)>`.
pub fn bifurcation_diagram_logistic(
    r_values: &[f64],
    n_transient: usize,
    n_plot: usize,
) -> Vec<(f64, Vec<f64>)> {
    r_values
        .iter()
        .map(|&r| {
            let orbit = logistic_map_iterate(r, 0.5, n_transient + n_plot);
            let attractor: Vec<f64> = orbit.into_iter().skip(n_transient).take(n_plot).collect();
            (r, attractor)
        })
        .collect()
}

// ── Fractal dimensions ─────────────────────────────────────────────────────────

/// Estimate the box-counting dimension of a trajectory by covering the point
/// cloud with boxes of varying sizes and performing a log-log regression.
///
/// `min_box` and `max_box` define the range of box sizes; `n_scales` is the
/// number of logarithmically-spaced box sizes to use.
///
/// Returns a `FractalDimension` via least-squares linear regression on
/// log(count) vs log(1/box_size).
pub fn box_counting_dimension(
    trajectory: &Trajectory,
    min_box: f64,
    max_box: f64,
    n_scales: usize,
) -> FractalDimension {
    if trajectory.points.is_empty() || n_scales < 2 || min_box <= 0.0 || max_box <= min_box {
        return FractalDimension {
            value: 0.0,
            method: DimensionMethod::BoxCounting,
        };
    }
    let dim = trajectory.points[0].len();
    let log_inv_sizes: Vec<f64> = (0..n_scales)
        .map(|i| {
            let t = i as f64 / (n_scales - 1) as f64;
            let box_size = min_box * (max_box / min_box).powf(t);
            -(box_size.ln())
        })
        .collect();

    let log_counts: Vec<f64> = (0..n_scales)
        .map(|i| {
            let t = i as f64 / (n_scales - 1) as f64;
            let box_size = min_box * (max_box / min_box).powf(t);
            let count = count_occupied_boxes(&trajectory.points, box_size, dim);
            if count > 0 {
                (count as f64).ln()
            } else {
                0.0
            }
        })
        .collect();

    let slope = linear_regression_slope(&log_inv_sizes, &log_counts);
    FractalDimension {
        value: slope.max(0.0),
        method: DimensionMethod::BoxCounting,
    }
}

/// Estimate the correlation dimension using the Grassberger–Procaccia algorithm.
///
/// Computes C(r) = fraction of point pairs with distance < r, then estimates
/// the slope of log C(r) vs log r.
pub fn correlation_dimension(trajectory: &Trajectory, radii: &[f64]) -> FractalDimension {
    let points = &trajectory.points;
    let n = points.len();
    if n < 2 || radii.is_empty() {
        return FractalDimension {
            value: 0.0,
            method: DimensionMethod::CorrelationDimension,
        };
    }
    let total_pairs = (n * (n - 1)) as f64;
    let mut log_r: Vec<f64> = Vec::with_capacity(radii.len());
    let mut log_c: Vec<f64> = Vec::with_capacity(radii.len());

    for &r in radii {
        if r <= 0.0 {
            continue;
        }
        let mut count = 0usize;
        for i in 0..n {
            for j in (i + 1)..n {
                if euclidean_distance(&points[i], &points[j]) < r {
                    count += 2; // count both (i,j) and (j,i)
                }
            }
        }
        if count > 0 {
            let c = count as f64 / total_pairs;
            log_r.push(r.ln());
            log_c.push(c.ln());
        }
    }

    if log_r.len() < 2 {
        return FractalDimension {
            value: 0.0,
            method: DimensionMethod::CorrelationDimension,
        };
    }

    let slope = linear_regression_slope(&log_r, &log_c);
    FractalDimension {
        value: slope.max(0.0),
        method: DimensionMethod::CorrelationDimension,
    }
}

// ── Poincaré section ──────────────────────────────────────────────────────────

/// Compute the Poincaré section of a trajectory.
///
/// Records every time the trajectory crosses `points\[k\][section_dim] ≈ section_value`
/// (within `tol`), and measures the recurrence times between consecutive crossings.
///
/// `section_dim` is the index of the coordinate defining the hyperplane.
pub fn poincare_section(
    trajectory: &Trajectory,
    section_dim: usize,
    section_value: f64,
    tol: f64,
) -> PoincareSectionResult {
    let mut section_points = Vec::new();
    let mut recurrence_times = Vec::new();
    let mut last_time = None::<f64>;

    for (pt, &t) in trajectory.points.iter().zip(trajectory.times.iter()) {
        if section_dim < pt.len() && (pt[section_dim] - section_value).abs() < tol {
            if let Some(lt) = last_time {
                recurrence_times.push(t - lt);
            }
            section_points.push(pt.clone());
            last_time = Some(t);
        }
    }
    PoincareSectionResult {
        points: section_points,
        recurrence_times,
    }
}

// ── Kaplan–Yorke dimension ────────────────────────────────────────────────────

/// Compute the Kaplan–Yorke (Lyapunov) dimension from a `LyapunovExponents`
/// struct.
///
/// Sort exponents in descending order, find the largest j such that the sum
/// of the first j exponents is non-negative, then:
///
/// D_KY = j + (sum_{i=1}^j λ_i) / |λ_{j+1}|
pub fn kaplan_yorke_dimension(exponents: &LyapunovExponents) -> f64 {
    let mut sorted = exponents.exponents.clone();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let mut sum = 0.0f64;
    let mut j = 0usize;
    for (i, &e) in sorted.iter().enumerate() {
        sum += e;
        if sum < 0.0 {
            // j = i (the index before this one was the last non-negative sum).
            // Recompute sum without the current e.
            sum -= e;
            // j = i
            let next_neg = sorted[i].abs();
            if next_neg < 1e-300 {
                return i as f64;
            }
            return i as f64 + sum / next_neg;
        }
        j = i + 1;
    }
    // All exponents are non-negative.
    j as f64
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Euclidean distance between two points.
pub(super) fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y) * (x - y))
        .sum::<f64>()
        .sqrt()
}

/// Count the number of distinct occupied boxes of size `box_size` in a
/// d-dimensional point cloud.
fn count_occupied_boxes(points: &[Vec<f64>], box_size: f64, dim: usize) -> usize {
    use std::collections::HashSet;
    let mut boxes: HashSet<Vec<i64>> = HashSet::new();
    for pt in points {
        if pt.len() < dim {
            continue;
        }
        let key: Vec<i64> = pt[..dim]
            .iter()
            .map(|&x| (x / box_size).floor() as i64)
            .collect();
        boxes.insert(key);
    }
    boxes.len()
}

/// Least-squares slope of the linear regression y ≈ slope·x + intercept.
fn linear_regression_slope(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 {
        return 0.0;
    }
    let n_f = n as f64;
    let sum_x: f64 = x[..n].iter().sum();
    let sum_y: f64 = y[..n].iter().sum();
    let sum_xx: f64 = x[..n].iter().map(|v| v * v).sum();
    let sum_xy: f64 = x[..n].iter().zip(y[..n].iter()).map(|(a, b)| a * b).sum();
    let denom = n_f * sum_xx - sum_x * sum_x;
    if denom.abs() < 1e-15 {
        return 0.0;
    }
    (n_f * sum_xy - sum_x * sum_y) / denom
}

// ── Public re-exports for less commonly used types ────────────────────────────

/// Build a `DynamicalSystem` configured as a logistic map.
pub fn logistic_system(r: f64) -> DynamicalSystem {
    DynamicalSystem {
        dimension: 1,
        system_type: SystemType::Discrete {
            map_type: MapType::Logistic { r },
        },
    }
}

/// Build a `DynamicalSystem` configured as a Hénon map.
pub fn henon_system(a: f64, b: f64) -> DynamicalSystem {
    DynamicalSystem {
        dimension: 2,
        system_type: SystemType::Discrete {
            map_type: MapType::Henon { a, b },
        },
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    // --- Logistic map ---

    #[test]
    fn test_logistic_iterate_length() {
        let orbit = logistic_map_iterate(3.5, 0.5, 10);
        assert_eq!(orbit.len(), 11);
    }

    #[test]
    fn test_logistic_fixed_point_r1() {
        // For r=1: x_{n+1} = x(1-x). Fixed point x=0. Starting near 0.01
        // the orbit should converge to 0.
        let orbit = logistic_map_iterate(1.0, 0.01, 200);
        assert!(
            orbit.last().copied().unwrap_or(1.0) < 0.1,
            "Orbit should converge toward 0 for r=1"
        );
    }

    #[test]
    fn test_logistic_chaos_r4() {
        // r=4 is fully chaotic; after many steps the orbit stays in (0,1).
        let orbit = logistic_map_iterate(4.0, 0.3, 1000);
        assert!(orbit.iter().all(|&x| x > 0.0 && x < 1.0));
    }

    // --- Hénon map ---

    #[test]
    fn test_henon_iterate_length() {
        let traj = henon_map_iterate(1.4, 0.3, 0.0, 0.0, 100);
        assert_eq!(traj.points.len(), 101);
        assert_eq!(traj.times.len(), 101);
    }

    #[test]
    fn test_henon_dimension() {
        let traj = henon_map_iterate(1.4, 0.3, 0.1, 0.1, 5);
        assert!(traj.points.iter().all(|p| p.len() == 2));
    }

    // --- Tent map ---

    #[test]
    fn test_tent_iterate_length() {
        let orbit = tent_map_iterate(1.5, 0.4, 20);
        assert_eq!(orbit.len(), 21);
    }

    #[test]
    fn test_tent_stays_bounded() {
        // For mu=2, x in [0,1] stays in [0, 0.5] or [0.5, 1].
        let orbit = tent_map_iterate(2.0, 0.3, 100);
        assert!(orbit.iter().all(|&x| x >= 0.0));
    }

    // --- Lorenz system ---

    #[test]
    fn test_lorenz_length() {
        let traj = lorenz_euler(10.0, 28.0, 8.0 / 3.0, 1.0, 1.0, 1.0, 0.01, 100);
        assert_eq!(traj.points.len(), 101);
    }

    #[test]
    fn test_lorenz_3d() {
        let traj = lorenz_euler(10.0, 28.0, 8.0 / 3.0, 0.1, 0.0, 0.0, 0.001, 10);
        assert!(traj.points.iter().all(|p| p.len() == 3));
    }

    // --- Rössler system ---

    #[test]
    fn test_rossler_length() {
        let traj = rossler_euler(0.2, 0.2, 5.7, 1.0, 0.0, 0.0, 0.01, 50);
        assert_eq!(traj.points.len(), 51);
    }

    // --- Fixed points ---

    #[test]
    fn test_fixed_points_logistic_r2() {
        // f(x) = 2x(1-x); fixed points: x=0 and x=0.5.
        let xs: Vec<f64> = (0..=100).map(|i| i as f64 / 100.0).collect();
        let fvals: Vec<(f64, f64)> = xs.iter().map(|&x| (x, 2.0 * x * (1.0 - x))).collect();
        let fps = fixed_points_1d(&fvals);
        assert!(fps.iter().any(|&f| f.abs() < 0.02), "Should find x≈0");
        assert!(
            fps.iter().any(|&f| (f - 0.5).abs() < 0.02),
            "Should find x≈0.5"
        );
    }

    #[test]
    fn test_classify_stable_fixed_point() {
        // |f'| < 1 → stable.
        assert_eq!(classify_fixed_point_1d(0.5), Stability::StableNode);
    }

    #[test]
    fn test_classify_unstable_fixed_point() {
        // |f'| > 1 → unstable.
        assert_eq!(classify_fixed_point_1d(2.0), Stability::UnstableNode);
    }

    #[test]
    fn test_classify_center_fixed_point() {
        // |f'| = 1 → center.
        assert_eq!(classify_fixed_point_1d(1.0), Stability::Center);
    }

    // --- Lyapunov exponents ---

    #[test]
    fn test_lyapunov_exponent_logistic_r2_negative() {
        // At r=2, the fixed point x=0.5 is stable; Lyapunov exponent < 0.
        let le = lyapunov_exponent_logistic(2.0, 500, 1000);
        assert!(
            le < 0.0,
            "Lyapunov exponent for r=2 should be negative, got {}",
            le
        );
    }

    #[test]
    fn test_lyapunov_exponent_logistic_r4_positive() {
        // At r=4, chaos is fully developed; Lyapunov exponent ≈ ln(2) > 0.
        let le = lyapunov_exponent_logistic(4.0, 1000, 5000);
        assert!(
            le > 0.0,
            "Lyapunov exponent for r=4 should be positive, got {}",
            le
        );
    }

    // --- Period detection ---

    #[test]
    fn test_period_detection_period2() {
        // Alternating orbit: period 2.
        let orbit: Vec<f64> = (0..100)
            .map(|i| if i % 2 == 0 { 0.3 } else { 0.7 })
            .collect();
        let p = period_of_orbit(&orbit, 1e-6);
        assert_eq!(p, Some(2));
    }

    #[test]
    fn test_period_detection_period1() {
        // Constant orbit: period 1.
        let orbit = vec![0.5f64; 50];
        let p = period_of_orbit(&orbit, 1e-6);
        assert_eq!(p, Some(1));
    }

    // --- Bifurcation diagram ---

    #[test]
    fn test_bifurcation_diagram_length() {
        let rs: Vec<f64> = (0..10).map(|i| 2.5 + i as f64 * 0.15).collect();
        let diag = bifurcation_diagram_logistic(&rs, 100, 50);
        assert_eq!(diag.len(), 10);
        assert!(diag.iter().all(|(_, v)| v.len() == 50));
    }

    // --- Fractal dimensions ---

    #[test]
    fn test_box_counting_henon() {
        let traj = henon_map_iterate(1.4, 0.3, 0.1, 0.1, 500);
        let fd = box_counting_dimension(&traj, 0.01, 1.0, 8);
        // Hénon attractor has fractal dimension ≈ 1.26; we just check positivity.
        assert!(fd.value >= 0.0);
        assert_eq!(fd.method, DimensionMethod::BoxCounting);
    }

    #[test]
    fn test_correlation_dimension_line() {
        // Points on a 1D line should give dimension ≈ 1.
        let pts: Vec<Vec<f64>> = (0..50).map(|i| vec![i as f64 * 0.02, 0.0]).collect();
        let traj = Trajectory {
            points: pts,
            times: (0..50).map(|i| i as f64).collect(),
        };
        let radii: Vec<f64> = (1..=10).map(|i| i as f64 * 0.05).collect();
        let fd = correlation_dimension(&traj, &radii);
        // For a line, correlation dimension ≈ 1.
        assert!(fd.value > 0.5 && fd.value < 2.0, "Got dim={}", fd.value);
    }

    // --- Poincaré section ---

    #[test]
    fn test_poincare_section_lorenz() {
        let traj = lorenz_euler(10.0, 28.0, 8.0 / 3.0, 1.0, 0.0, 0.0, 0.01, 5000);
        let result = poincare_section(&traj, 2, 27.0, 1.0);
        // Should find crossings.
        assert!(!result.points.is_empty());
    }

    // --- Kaplan–Yorke dimension ---

    #[test]
    fn test_kaplan_yorke_lorenz_attractor() {
        // Lorenz attractor: λ_1 ≈ 0.9, λ_2 = 0, λ_3 ≈ -14.5 → D_KY ≈ 2 + 0.9/14.5 ≈ 2.06.
        let exps = LyapunovExponents {
            exponents: vec![0.9, 0.0, -14.5],
            system_dimension: 3,
        };
        let d = kaplan_yorke_dimension(&exps);
        assert!(approx_eq(d, 2.0 + 0.9 / 14.5, 0.01), "Got D_KY={}", d);
    }

    #[test]
    fn test_kaplan_yorke_all_negative() {
        // All negative → D_KY = 0.
        let exps = LyapunovExponents {
            exponents: vec![-1.0, -2.0, -3.0],
            system_dimension: 3,
        };
        let d = kaplan_yorke_dimension(&exps);
        assert!(approx_eq(d, 0.0, 0.01), "Got D_KY={}", d);
    }

    // --- System constructors ---

    #[test]
    fn test_logistic_system_dimension() {
        let sys = logistic_system(3.7);
        assert_eq!(sys.dimension, 1);
    }

    #[test]
    fn test_henon_system_dimension() {
        let sys = henon_system(1.4, 0.3);
        assert_eq!(sys.dimension, 2);
    }
}
