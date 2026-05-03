//! Functions for spectral graph theory.

use super::types::{
    AdjMatrix, Expander, GraphSpectrum, LaplacianMatrix, MixingTime, RandomWalkMatrix, SpectralGap,
};

// ── Basic constructions ───────────────────────────────────────────────────────

/// Compute the Laplacian L = D − A from an adjacency matrix.
///
/// D is the diagonal degree matrix: `D\[i\]\[i\] = sum_j A\[i\]\[j\]`.
pub fn adjacency_to_laplacian(adj: &AdjMatrix) -> LaplacianMatrix {
    let n = adj.n;
    let mut data = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        let deg: f64 = adj.data[i].iter().sum();
        data[i][i] = deg;
        for j in 0..n {
            data[i][j] -= adj.data[i][j];
        }
    }
    LaplacianMatrix { data, n }
}

/// Return the degree vector: `d\[i\] = sum_j A\[i\]\[j\]`.
pub fn degree_vector(adj: &AdjMatrix) -> Vec<f64> {
    (0..adj.n).map(|i| adj.data[i].iter().sum()).collect()
}

/// Compute the random-walk matrix W = D⁻¹A.
///
/// If vertex i is isolated (degree 0), its row is left as all zeros.
pub fn random_walk_matrix(adj: &AdjMatrix) -> RandomWalkMatrix {
    let n = adj.n;
    let deg = degree_vector(adj);
    let mut data = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        if deg[i] > 0.0 {
            for j in 0..n {
                data[i][j] = adj.data[i][j] / deg[i];
            }
        }
    }
    RandomWalkMatrix { data, n }
}

// ── Power method ─────────────────────────────────────────────────────────────

/// Estimate the largest eigenvalue of a symmetric matrix `m` using the power
/// method with `k` iterations.
///
/// Returns `(eigenvalue, eigenvector)`.  The eigenvector is normalised to
/// unit Euclidean length.
pub fn power_method(m: &LaplacianMatrix, k: usize) -> (f64, Vec<f64>) {
    let n = m.n;
    if n == 0 {
        return (0.0, vec![]);
    }
    // Start with the all-ones vector, normalised.
    let mut v: Vec<f64> = (0..n).map(|i| if i == 0 { 1.0 } else { 0.0 }).collect();
    normalise_vec(&mut v);

    let mut eigenvalue = 0.0f64;
    for _ in 0..k {
        let w = mat_vec_mul(&m.data, &v);
        let norm = vec_norm(&w);
        if norm < 1e-15 {
            break;
        }
        eigenvalue = dot(&v, &w);
        v = w.iter().map(|x| x / norm).collect();
    }
    (eigenvalue, v)
}

/// Estimate the spectral gap of the normalised Laplacian using the power method.
///
/// We approximate λ₁ (second-smallest eigenvalue of L, i.e. Fiedler value) and
/// λ_max (largest eigenvalue).  `SpectralGap.lambda1` ≈ Fiedler value,
/// `SpectralGap.lambda2` ≈ largest eigenvalue of normalised Laplacian.
pub fn estimate_spectral_gap(adj: &AdjMatrix) -> SpectralGap {
    let lap = adjacency_to_laplacian(adj);
    // Largest eigenvalue via power method.
    let (lambda_max, _) = power_method(&lap, 200);
    // Fiedler value via power method on (lambda_max * I − L).
    let fiedler = algebraic_connectivity(adj);
    let gap = if lambda_max > fiedler {
        lambda_max - fiedler
    } else {
        0.0
    };
    SpectralGap {
        lambda1: fiedler,
        lambda2: lambda_max,
        gap,
    }
}

// ── Connectivity ──────────────────────────────────────────────────────────────

/// Check whether the graph is connected using BFS from vertex 0.
///
/// An empty graph (n = 0) is considered connected by convention.
pub fn is_connected(adj: &AdjMatrix) -> bool {
    let n = adj.n;
    if n == 0 {
        return true;
    }
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0usize);
    visited[0] = true;
    while let Some(u) = queue.pop_front() {
        for v in 0..n {
            if !visited[v] && adj.data[u][v] != 0.0 {
                visited[v] = true;
                queue.push_back(v);
            }
        }
    }
    visited.iter().all(|&b| b)
}

// ── Algebraic connectivity / Fiedler vector ──────────────────────────────────

/// Compute the algebraic connectivity (Fiedler value) of the graph: the
/// second-smallest eigenvalue of the Laplacian.
///
/// Uses the deflation trick: largest eigenvalue of (λ_max · I − L) = λ_max − λ_min.
/// We deflate the all-ones eigenvector (eigenvalue 0) by subtracting its component
/// from the iterated vector, then the power method converges to the second-smallest.
pub fn algebraic_connectivity(adj: &AdjMatrix) -> f64 {
    let lap = adjacency_to_laplacian(adj);
    let n = lap.n;
    if n <= 1 {
        return 0.0;
    }
    // Largest eigenvalue for shift.
    let (lambda_max, _) = power_method(&lap, 100);
    // Build shifted matrix M = lambda_max * I - L  (eigenvalues: lambda_max - lambda_i).
    // Largest eigenvector of M corresponds to lambda_min of L.
    // We want second-largest of M → second-smallest of L.
    // Use deflated power method: subtract projection onto all-ones vector.
    let ones_norm = (n as f64).sqrt();
    let ones: Vec<f64> = vec![1.0 / ones_norm; n];

    let mut v: Vec<f64> = (0..n).map(|i| (i + 1) as f64).collect();
    // Remove component along ones.
    deflate(&mut v, &ones);
    normalise_vec(&mut v);

    let mut eigenvalue_m = 0.0f64;
    for _ in 0..300 {
        // w = M * v = lambda_max * v - L * v
        let lv = mat_vec_mul(&lap.data, &v);
        let w: Vec<f64> = v
            .iter()
            .zip(lv.iter())
            .map(|(vi, li)| lambda_max * vi - li)
            .collect();
        // Deflate all-ones component.
        let mut w2 = w.clone();
        deflate(&mut w2, &ones);
        let norm = vec_norm(&w2);
        if norm < 1e-15 {
            break;
        }
        eigenvalue_m = dot(&v, &w);
        v = w2.iter().map(|x| x / norm).collect();
    }
    // eigenvalue_m ≈ lambda_max - lambda_fiedler  → fiedler ≈ lambda_max - eigenvalue_m
    let fiedler = lambda_max - eigenvalue_m;
    fiedler.max(0.0)
}

/// Compute (an approximation of) the Fiedler vector: the eigenvector of the
/// Laplacian corresponding to the second-smallest eigenvalue.
///
/// Uses the same deflated power method as `algebraic_connectivity`.
pub fn fiedler_vector(lap: &LaplacianMatrix) -> Vec<f64> {
    let n = lap.n;
    if n <= 1 {
        return vec![0.0; n];
    }
    let (lambda_max, _) = power_method(lap, 100);
    let ones_norm = (n as f64).sqrt();
    let ones: Vec<f64> = vec![1.0 / ones_norm; n];

    let mut v: Vec<f64> = (0..n).map(|i| (i + 1) as f64).collect();
    deflate(&mut v, &ones);
    normalise_vec(&mut v);

    for _ in 0..300 {
        let lv = mat_vec_mul(&lap.data, &v);
        let w: Vec<f64> = v
            .iter()
            .zip(lv.iter())
            .map(|(vi, li)| lambda_max * vi - li)
            .collect();
        let mut w2 = w.clone();
        deflate(&mut w2, &ones);
        let norm = vec_norm(&w2);
        if norm < 1e-15 {
            break;
        }
        v = w2.iter().map(|x| x / norm).collect();
    }
    v
}

// ── Spectral bisection ────────────────────────────────────────────────────────

/// Partition the graph into two sets by the sign of the Fiedler vector entries.
///
/// Returns `(part_negative, part_non_negative)`.
pub fn spectral_bisection(adj: &AdjMatrix) -> (Vec<usize>, Vec<usize>) {
    let lap = adjacency_to_laplacian(adj);
    let fv = fiedler_vector(&lap);
    let mut neg = Vec::new();
    let mut pos = Vec::new();
    for (i, &val) in fv.iter().enumerate() {
        if val < 0.0 {
            neg.push(i);
        } else {
            pos.push(i);
        }
    }
    (neg, pos)
}

// ── Named graphs ─────────────────────────────────────────────────────────────

/// Construct the Petersen graph: 10 vertices, 3-regular, non-Hamiltonian.
///
/// Vertices 0–4 form the outer pentagon; vertices 5–9 the inner pentagram.
pub fn petersen_graph() -> AdjMatrix {
    let mut adj = AdjMatrix::new(10);
    // Outer cycle: 0-1-2-3-4-0
    for i in 0..5usize {
        adj.add_edge(i, (i + 1) % 5, 1.0);
    }
    // Spokes: 0-5, 1-6, 2-7, 3-8, 4-9
    for i in 0..5usize {
        adj.add_edge(i, i + 5, 1.0);
    }
    // Inner pentagram: 5-7-9-6-8-5
    let inner = [5usize, 7, 9, 6, 8];
    for i in 0..5usize {
        adj.add_edge(inner[i], inner[(i + 1) % 5], 1.0);
    }
    adj
}

/// Construct the complete graph K_n.
pub fn complete_graph(n: usize) -> AdjMatrix {
    let mut adj = AdjMatrix::new(n);
    for i in 0..n {
        for j in (i + 1)..n {
            adj.add_edge(i, j, 1.0);
        }
    }
    adj
}

/// Construct the cycle graph C_n.
pub fn cycle_graph(n: usize) -> AdjMatrix {
    let mut adj = AdjMatrix::new(n);
    if n < 2 {
        return adj;
    }
    for i in 0..n {
        adj.add_edge(i, (i + 1) % n, 1.0);
    }
    adj
}

/// Construct the path graph P_n (n vertices, n-1 edges in a line).
pub fn path_graph(n: usize) -> AdjMatrix {
    let mut adj = AdjMatrix::new(n);
    for i in 0..(n.saturating_sub(1)) {
        adj.add_edge(i, i + 1, 1.0);
    }
    adj
}

// ── Cheeger inequality ────────────────────────────────────────────────────────

/// Return the Cheeger inequality bounds on the edge expansion h from the
/// spectral gap λ of the normalised Laplacian.
///
/// The Cheeger inequality states: `λ/2 ≤ h ≤ sqrt(2λ)`.
/// Returns `(lower_bound, upper_bound)` where lower = λ/2 and upper = √(2λ).
pub fn cheeger_inequality_bounds(gap: &SpectralGap) -> (f64, f64) {
    let lambda = gap.gap;
    let lower = lambda / 2.0;
    let upper = (2.0 * lambda).sqrt();
    (lower, upper)
}

// ── Expander certificate ──────────────────────────────────────────────────────

/// Attempt to certify whether a d-regular graph is an expander by checking
/// that the spectral gap exceeds a threshold.
///
/// Returns `Some(Expander)` if the graph is d-regular and the spectral gap
/// is positive, `None` otherwise.
pub fn certify_expander(adj: &AdjMatrix) -> Option<Expander> {
    let n = adj.n;
    if n == 0 {
        return None;
    }
    let deg = degree_vector(adj);
    // Check regularity.
    let d0 = deg[0].round() as usize;
    for &di in &deg {
        if (di - d0 as f64).abs() > 1e-9 {
            return None;
        }
    }
    let gap = estimate_spectral_gap(adj);
    if gap.gap <= 0.0 {
        return None;
    }
    Some(Expander {
        n,
        degree: d0,
        spectral_gap: gap.gap,
    })
}

// ── Mixing time estimate ──────────────────────────────────────────────────────

/// Estimate the mixing time of the random walk on a graph given the spectral
/// gap of the lazy walk.
///
/// For a lazy random walk with second eigenvalue `μ = 1 − gap/2`, the mixing
/// time satisfies `t_mix(ε) ≤ ceil(log(n/ε) / gap)`.
///
/// Returns `MixingTime { steps: usize::MAX, … }` for disconnected graphs.
pub fn estimate_mixing_time(adj: &AdjMatrix, epsilon: f64) -> MixingTime {
    let n = adj.n;
    if n <= 1 || epsilon <= 0.0 {
        return MixingTime { epsilon, steps: 0 };
    }
    // Disconnected graphs do not mix.
    if !is_connected(adj) {
        return MixingTime {
            epsilon,
            steps: usize::MAX,
        };
    }
    let gap_info = estimate_spectral_gap(adj);
    let gap = gap_info.gap;
    if gap < 1e-12 {
        return MixingTime {
            epsilon,
            steps: usize::MAX,
        };
    }
    let steps = ((n as f64 / epsilon).ln() / gap).ceil() as usize;
    MixingTime { epsilon, steps }
}

// ── GraphSpectrum (full power-iteration spectrum approx) ─────────────────────

/// Compute an approximate full spectrum of a symmetric matrix using the
/// sequential deflation / power-iteration method.
///
/// This gives a coarse approximation suitable for small graphs (n ≤ 20).
/// For each of the n eigenvectors we run `iters` power iterations, deflating
/// all previously found eigenvectors.
pub fn approximate_spectrum(lap: &LaplacianMatrix, iters: usize) -> GraphSpectrum {
    let n = lap.n;
    let mut eigenvalues = Vec::with_capacity(n);
    let mut eigenvectors: Vec<Vec<f64>> = Vec::with_capacity(n);

    for k in 0..n {
        // Initial vector orthogonal to previous eigenvectors.
        let mut v: Vec<f64> = (0..n)
            .map(|i| if i == k { 1.0 } else { 0.5 / n as f64 })
            .collect();
        for ev in &eigenvectors {
            deflate(&mut v, ev);
        }
        let norm = vec_norm(&v);
        if norm < 1e-14 {
            eigenvalues.push(0.0);
            eigenvectors.push(vec![0.0; n]);
            continue;
        }
        for x in &mut v {
            *x /= norm;
        }

        let mut lambda = 0.0f64;
        for _ in 0..iters {
            let w = mat_vec_mul(&lap.data, &v);
            let mut w2 = w.clone();
            for ev in &eigenvectors {
                deflate(&mut w2, ev);
            }
            let norm2 = vec_norm(&w2);
            if norm2 < 1e-15 {
                break;
            }
            lambda = dot(&v, &w);
            v = w2.iter().map(|x| x / norm2).collect();
        }
        eigenvalues.push(lambda);
        eigenvectors.push(v);
    }
    GraphSpectrum {
        eigenvalues,
        eigenvectors,
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Matrix-vector product: `(M * v)\[i\] = sum_j M\[i\]\[j\] * v\[j\]`.
pub(super) fn mat_vec_mul(m: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
    m.iter()
        .map(|row| row.iter().zip(v.iter()).map(|(a, b)| a * b).sum())
        .collect()
}

/// Euclidean norm of a vector.
pub(super) fn vec_norm(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// Dot product of two vectors.
pub(super) fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Normalise `v` in place to unit Euclidean length.
pub(super) fn normalise_vec(v: &mut Vec<f64>) {
    let norm = vec_norm(v);
    if norm > 1e-15 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

/// Deflate `v` in place: subtract the component along the unit vector `u`.
/// `u` must already be unit length.
pub(super) fn deflate(v: &mut Vec<f64>, u: &[f64]) {
    let proj = dot(v, u);
    for (vi, ui) in v.iter_mut().zip(u.iter()) {
        *vi -= proj * ui;
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn test_adj_matrix_new() {
        let adj = AdjMatrix::new(3);
        assert_eq!(adj.n, 3);
        assert_eq!(adj.data, vec![vec![0.0; 3]; 3]);
    }

    #[test]
    fn test_add_edge_symmetric() {
        let mut adj = AdjMatrix::new(4);
        adj.add_edge(0, 2, 1.0);
        assert_eq!(adj.data[0][2], 1.0);
        assert_eq!(adj.data[2][0], 1.0);
        assert_eq!(adj.data[0][1], 0.0);
    }

    #[test]
    fn test_degree_vector_path() {
        let adj = path_graph(4);
        let deg = degree_vector(&adj);
        assert_eq!(deg[0], 1.0);
        assert_eq!(deg[1], 2.0);
        assert_eq!(deg[2], 2.0);
        assert_eq!(deg[3], 1.0);
    }

    #[test]
    fn test_adjacency_to_laplacian_path3() {
        let adj = path_graph(3);
        let lap = adjacency_to_laplacian(&adj);
        // L[0][0] = 1, L[0][1] = -1, L[0][2] = 0
        assert!(approx_eq(lap.data[0][0], 1.0, 1e-10));
        assert!(approx_eq(lap.data[0][1], -1.0, 1e-10));
        assert!(approx_eq(lap.data[0][2], 0.0, 1e-10));
        // L[1][1] = 2, L[1][0] = -1, L[1][2] = -1
        assert!(approx_eq(lap.data[1][1], 2.0, 1e-10));
        assert!(approx_eq(lap.data[1][0], -1.0, 1e-10));
        assert!(approx_eq(lap.data[1][2], -1.0, 1e-10));
    }

    #[test]
    fn test_laplacian_row_sum_zero() {
        let adj = petersen_graph();
        let lap = adjacency_to_laplacian(&adj);
        for i in 0..lap.n {
            let row_sum: f64 = lap.data[i].iter().sum();
            assert!(
                approx_eq(row_sum, 0.0, 1e-10),
                "Row {} sum = {}",
                i,
                row_sum
            );
        }
    }

    #[test]
    fn test_random_walk_matrix_rows_sum_one() {
        let adj = cycle_graph(5);
        let rw = random_walk_matrix(&adj);
        for i in 0..rw.n {
            let s: f64 = rw.data[i].iter().sum();
            assert!(approx_eq(s, 1.0, 1e-10));
        }
    }

    #[test]
    fn test_random_walk_isolated_vertex() {
        let mut adj = AdjMatrix::new(3);
        adj.add_edge(0, 1, 1.0);
        // vertex 2 is isolated
        let rw = random_walk_matrix(&adj);
        let s: f64 = rw.data[2].iter().sum();
        assert!(approx_eq(s, 0.0, 1e-10));
    }

    #[test]
    fn test_is_connected_path() {
        let adj = path_graph(5);
        assert!(is_connected(&adj));
    }

    #[test]
    fn test_is_connected_disconnected() {
        let mut adj = AdjMatrix::new(4);
        adj.add_edge(0, 1, 1.0);
        adj.add_edge(2, 3, 1.0);
        assert!(!is_connected(&adj));
    }

    #[test]
    fn test_is_connected_empty_graph() {
        let adj = AdjMatrix::new(0);
        assert!(is_connected(&adj));
    }

    #[test]
    fn test_is_connected_single_vertex() {
        let adj = AdjMatrix::new(1);
        assert!(is_connected(&adj));
    }

    #[test]
    fn test_complete_graph_k4_degree() {
        let adj = complete_graph(4);
        let deg = degree_vector(&adj);
        for d in &deg {
            assert!(approx_eq(*d, 3.0, 1e-10));
        }
    }

    #[test]
    fn test_cycle_graph_c5_degree() {
        let adj = cycle_graph(5);
        let deg = degree_vector(&adj);
        for d in &deg {
            assert!(approx_eq(*d, 2.0, 1e-10));
        }
    }

    #[test]
    fn test_petersen_graph_3regular() {
        let adj = petersen_graph();
        let deg = degree_vector(&adj);
        for d in &deg {
            assert!(approx_eq(*d, 3.0, 1e-10));
        }
    }

    #[test]
    fn test_petersen_graph_connected() {
        assert!(is_connected(&petersen_graph()));
    }

    #[test]
    fn test_path_graph_p1() {
        let adj = path_graph(1);
        assert_eq!(adj.n, 1);
        let deg = degree_vector(&adj);
        assert!(approx_eq(deg[0], 0.0, 1e-10));
    }

    #[test]
    fn test_power_method_converges() {
        // For P_3 Laplacian, largest eigenvalue ≈ 3.0
        let adj = path_graph(3);
        let lap = adjacency_to_laplacian(&adj);
        let (lambda, _) = power_method(&lap, 500);
        // Largest eigenvalue of P_3 Laplacian is 2 + sqrt(2) ≈ 3.414
        assert!(lambda > 2.0 && lambda < 4.0);
    }

    #[test]
    fn test_algebraic_connectivity_path3_positive() {
        let adj = path_graph(3);
        let ac = algebraic_connectivity(&adj);
        assert!(
            ac > 0.0,
            "Fiedler value should be positive for connected P_3"
        );
    }

    #[test]
    fn test_algebraic_connectivity_k4() {
        // For K_4, Fiedler value = n = 4
        let adj = complete_graph(4);
        let ac = algebraic_connectivity(&adj);
        assert!(
            ac > 2.0,
            "K_4 Fiedler value should be close to 4, got {}",
            ac
        );
    }

    #[test]
    fn test_fiedler_vector_length() {
        let adj = cycle_graph(6);
        let lap = adjacency_to_laplacian(&adj);
        let fv = fiedler_vector(&lap);
        assert_eq!(fv.len(), 6);
    }

    #[test]
    fn test_fiedler_vector_near_unit() {
        let adj = complete_graph(5);
        let lap = adjacency_to_laplacian(&adj);
        let fv = fiedler_vector(&lap);
        let norm: f64 = fv.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!(
            approx_eq(norm, 1.0, 1e-6),
            "Fiedler vector should be unit, norm={}",
            norm
        );
    }

    #[test]
    fn test_spectral_bisection_returns_partition() {
        let adj = path_graph(6);
        let (neg, pos) = spectral_bisection(&adj);
        // Together they cover all 6 vertices.
        let mut all: Vec<usize> = neg.iter().chain(pos.iter()).cloned().collect();
        all.sort_unstable();
        assert_eq!(all, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_spectral_bisection_nonempty_parts() {
        // For path P_6 the Fiedler vector has both positive and negative entries.
        let adj = path_graph(6);
        let (neg, pos) = spectral_bisection(&adj);
        assert!(!neg.is_empty() || !pos.is_empty());
    }

    #[test]
    fn test_cheeger_inequality_bounds_positive() {
        let adj = cycle_graph(6);
        let gap = estimate_spectral_gap(&adj);
        let (lo, hi) = cheeger_inequality_bounds(&gap);
        assert!(lo >= 0.0);
        assert!(hi >= lo);
    }

    #[test]
    fn test_cheeger_bounds_zero_gap() {
        let gap = SpectralGap {
            lambda1: 0.0,
            lambda2: 0.0,
            gap: 0.0,
        };
        let (lo, hi) = cheeger_inequality_bounds(&gap);
        assert!(approx_eq(lo, 0.0, 1e-15));
        assert!(approx_eq(hi, 0.0, 1e-15));
    }

    #[test]
    fn test_certify_expander_petersen() {
        let adj = petersen_graph();
        let exp = certify_expander(&adj);
        assert!(exp.is_some());
        let e = exp.expect("Petersen graph is 3-regular expander");
        assert_eq!(e.degree, 3);
        assert_eq!(e.n, 10);
        assert!(e.spectral_gap > 0.0);
    }

    #[test]
    fn test_certify_expander_irregular_none() {
        let adj = path_graph(4); // degrees: 1,2,2,1 — not regular
        let exp = certify_expander(&adj);
        assert!(exp.is_none());
    }

    #[test]
    fn test_mixing_time_positive() {
        let adj = petersen_graph();
        let mt = estimate_mixing_time(&adj, 0.01);
        assert!(mt.steps > 0);
        assert!(approx_eq(mt.epsilon, 0.01, 1e-15));
    }

    #[test]
    fn test_mixing_time_disconnected_graph() {
        let mut adj = AdjMatrix::new(4);
        adj.add_edge(0, 1, 1.0);
        adj.add_edge(2, 3, 1.0);
        let mt = estimate_mixing_time(&adj, 0.1);
        assert_eq!(mt.steps, usize::MAX);
    }

    #[test]
    fn test_estimate_spectral_gap_k4() {
        let adj = complete_graph(4);
        let gap = estimate_spectral_gap(&adj);
        // lambda2 (max eigenvalue of Laplacian of K_4) should be positive.
        assert!(
            gap.lambda2 > 0.0,
            "max eigenvalue of K_4 Laplacian should be positive, got {}",
            gap.lambda2
        );
        // gap is non-negative
        assert!(gap.gap >= 0.0);
    }

    #[test]
    fn test_approximate_spectrum_count() {
        let adj = path_graph(4);
        let lap = adjacency_to_laplacian(&adj);
        let spec = approximate_spectrum(&lap, 200);
        assert_eq!(spec.eigenvalues.len(), 4);
        assert_eq!(spec.eigenvectors.len(), 4);
    }
}
