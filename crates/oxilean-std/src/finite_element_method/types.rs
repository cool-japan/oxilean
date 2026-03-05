//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// 1D staggered grid for a simplified mixed formulation.
/// Velocity lives at cell interfaces; pressure lives at cell centers.
#[derive(Debug, Clone)]
pub struct StaggeredGrid1D {
    /// Pressure nodes (cell centers): n nodes.
    pub p_nodes: Vec<f64>,
    /// Velocity nodes (cell interfaces): n+1 nodes.
    pub u_nodes: Vec<f64>,
    /// Cell width.
    pub h: f64,
    /// Number of cells.
    pub n: usize,
}
impl StaggeredGrid1D {
    /// Create a uniform staggered grid on [0,1] with n cells.
    pub fn uniform(n: usize) -> Self {
        let h = 1.0 / n as f64;
        let p_nodes = (0..n).map(|i| (i as f64 + 0.5) * h).collect();
        let u_nodes = (0..=n).map(|i| i as f64 * h).collect();
        StaggeredGrid1D {
            p_nodes,
            u_nodes,
            h,
            n,
        }
    }
    /// Divergence of velocity: div u_i = (u_{i+1} - u_i) / h for cell i.
    pub fn divergence(&self, u: &[f64]) -> Vec<f64> {
        (0..self.n).map(|i| (u[i + 1] - u[i]) / self.h).collect()
    }
    /// Gradient of pressure: grad p_i = (p_i - p_{i-1}) / h at interface i.
    pub fn grad_pressure(&self, p: &[f64]) -> Vec<f64> {
        let mut gp = vec![0.0; self.n + 1];
        for i in 1..self.n {
            gp[i] = (p[i] - p[i - 1]) / self.h;
        }
        gp
    }
    /// The discrete inf-sup constant for this grid: β_h = 1 (exact for P0/P1 staggered).
    pub fn inf_sup_constant(&self) -> f64 {
        1.0
    }
}
/// A 1D DG mesh with P0 elements (piecewise constants).
///
/// Used to demonstrate DG assembly for the advection equation u_t + a u_x = 0.
#[derive(Debug, Clone)]
pub struct DGMesh1D {
    /// Cell centers.
    pub centers: Vec<f64>,
    /// Cell width (uniform).
    pub h: f64,
    /// Left endpoint.
    pub a: f64,
    /// Right endpoint.
    pub b: f64,
    /// Number of cells.
    pub n: usize,
}
impl DGMesh1D {
    /// Create a uniform DG mesh on [a, b] with n cells.
    pub fn uniform(a: f64, b: f64, n: usize) -> Self {
        let h = (b - a) / n as f64;
        let centers = (0..n).map(|i| a + (i as f64 + 0.5) * h).collect();
        DGMesh1D {
            centers,
            h,
            a,
            b,
            n,
        }
    }
    /// Upwind flux for advection speed c.
    /// For c > 0: flux at interface = c * u_left;  for c < 0: c * u_right.
    pub fn upwind_flux(&self, u: &[f64], c: f64, i: usize) -> f64 {
        if c >= 0.0 {
            c * u[i]
        } else {
            c * u[(i + 1) % self.n]
        }
    }
    /// One explicit Euler DG step for u_t + c u_x = 0 (periodic BCs).
    pub fn dg_step(&self, u: &[f64], c: f64, dt: f64) -> Vec<f64> {
        let n = self.n;
        let mut u_new = u.to_vec();
        for i in 0..n {
            let flux_right = self.upwind_flux(u, c, i);
            let flux_left = self.upwind_flux(u, c, (i + n - 1) % n);
            u_new[i] = u[i] - (dt / self.h) * (flux_right - flux_left);
        }
        u_new
    }
    /// L1 norm of the solution.
    pub fn l1_norm(&self, u: &[f64]) -> f64 {
        u.iter().map(|x| x.abs() * self.h).sum()
    }
}
/// Solver for mixed FEM saddle-point systems:
///
///   [ A   B^T ] [ u ]   [ f ]
///   [ B   0   ] [ p ] = [ g ]
///
/// using the preconditioned Uzawa iteration:
///   u_{k+1} = u_k - A⁻¹ (A u_k + B^T p_k - f)
///   p_{k+1} = p_k - ω B u_{k+1} - ω g
pub struct MixedFEMSolver {
    /// The A block (velocity stiffness).
    pub a: DenseMatrix,
    /// The B block (divergence / constraint operator), stored as dense.
    pub b: DenseMatrix,
    /// Number of velocity DOFs.
    pub n_u: usize,
    /// Number of pressure DOFs.
    pub n_p: usize,
    /// Uzawa relaxation parameter ω.
    pub omega: f64,
}
impl MixedFEMSolver {
    /// Create a new mixed FEM solver from the A and B blocks.
    pub fn new(a: DenseMatrix, b: DenseMatrix, omega: f64) -> Self {
        let n_u = a.n;
        let n_p = b.n;
        MixedFEMSolver {
            a,
            b,
            n_u,
            n_p,
            omega,
        }
    }
    /// Compute B^T * p (n_u vector).
    fn bt_times_p(&self, p: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; self.n_u];
        for i in 0..self.n_p {
            for j in 0..self.n_u {
                result[j] += self.b.get(i, j) * p[i];
            }
        }
        result
    }
    /// Compute B * u (n_p vector).
    fn b_times_u(&self, u: &[f64]) -> Vec<f64> {
        (0..self.n_p)
            .map(|i| (0..self.n_u).map(|j| self.b.get(i, j) * u[j]).sum())
            .collect()
    }
    /// Solve the saddle-point system using the Uzawa iteration.
    ///
    /// `f` has length `n_u`, `g` has length `n_p`.
    /// Returns `(u, p)`.
    pub fn solve(&self, f: &[f64], g: &[f64], tol: f64, max_iter: usize) -> (Vec<f64>, Vec<f64>) {
        let mut u = vec![0.0; self.n_u];
        let mut p = vec![0.0; self.n_p];
        for _ in 0..max_iter {
            let btp = self.bt_times_p(&p);
            let rhs_u: Vec<f64> = (0..self.n_u).map(|i| f[i] - btp[i]).collect();
            u = conjugate_gradient(&self.a, &rhs_u, tol * 1e-3, max_iter);
            let bu = self.b_times_u(&u);
            for i in 0..self.n_p {
                p[i] += self.omega * (bu[i] - g[i]);
            }
            let res: f64 = bu
                .iter()
                .zip(g.iter())
                .map(|(b, g)| (b - g).powi(2))
                .sum::<f64>()
                .sqrt();
            if res < tol {
                break;
            }
        }
        (u, p)
    }
}
/// Interior penalty DG stiffness assembler for a 2D triangular mesh.
///
/// Assembles the SIPG bilinear form:
///   a_{IP}(u,v) = Σ_K ∫_K ∇u·∇v dx
///               - Σ_e ∫_e ({∇u}·n [v] + {∇v}·n [u]) ds
///               + Σ_e (σ/h_e) ∫_e [u][v] ds
///
/// Implemented as a dense global matrix for small meshes.
pub struct DGMethodStiffness {
    /// The mesh.
    pub mesh: TriangularMesh2D,
    /// Interior penalty parameter σ (must be > σ₀ for coercivity).
    pub sigma: f64,
}
impl DGMethodStiffness {
    /// Create a new DG stiffness assembler.
    pub fn new(mesh: TriangularMesh2D, sigma: f64) -> Self {
        DGMethodStiffness { mesh, sigma }
    }
    /// Assemble the DG stiffness matrix.
    ///
    /// For P1 DG: each triangle has 3 independent DOFs.
    /// Global DOF numbering: triangle k, local node a → DOF index 3*k + a.
    pub fn assemble(&self) -> DenseMatrix {
        let n_tri = self.mesh.num_triangles();
        let n_dof = 3 * n_tri;
        let mut k_global = DenseMatrix::zeros(n_dof);
        for tri_idx in 0..n_tri {
            let [i0, i1, i2] = self.mesh.triangles[tri_idx];
            let fmap = AffineTriangleMap::new(
                self.mesh.vertices[i0],
                self.mesh.vertices[i1],
                self.mesh.vertices[i2],
            );
            let det_j = fmap.det_j();
            let area = det_j / 2.0;
            let grads_phys = [
                fmap.transform_grad(p1_grad(0)),
                fmap.transform_grad(p1_grad(1)),
                fmap.transform_grad(p1_grad(2)),
            ];
            for a in 0..3 {
                for b in 0..3 {
                    let dot =
                        grads_phys[a][0] * grads_phys[b][0] + grads_phys[a][1] * grads_phys[b][1];
                    let global_a = 3 * tri_idx + a;
                    let global_b = 3 * tri_idx + b;
                    k_global.add(global_a, global_b, dot * area);
                }
            }
        }
        let mut edge_map: std::collections::HashMap<(usize, usize), Vec<(usize, usize)>> =
            std::collections::HashMap::new();
        for tri_idx in 0..n_tri {
            let nodes = self.mesh.triangles[tri_idx];
            for e in 0..3 {
                let a = nodes[e];
                let b = nodes[(e + 1) % 3];
                let key = (a.min(b), a.max(b));
                edge_map.entry(key).or_default().push((tri_idx, e));
            }
        }
        for ((_va, _vb), tris) in &edge_map {
            if tris.len() != 2 {
                continue;
            }
            let (tri_l, _edge_l) = tris[0];
            let (tri_r, _edge_r) = tris[1];
            let [il0, il1, il2] = self.mesh.triangles[tri_l];
            let area_l = self.mesh.triangle_area(tri_l);
            let area_r = self.mesh.triangle_area(tri_r);
            let h_e = (2.0 * (area_l + area_r) / 3.0).sqrt();
            let penalty = self.sigma / h_e;
            let nodes_l = [il0, il1, il2];
            for (a, _node_a) in nodes_l.iter().enumerate() {
                let global_a = 3 * tri_l + a;
                k_global.add(global_a, global_a, penalty / 6.0);
            }
            let [ir0, ir1, ir2] = self.mesh.triangles[tri_r];
            let nodes_r = [ir0, ir1, ir2];
            for (a, _node_a) in nodes_r.iter().enumerate() {
                let global_a = 3 * tri_r + a;
                k_global.add(global_a, global_a, penalty / 6.0);
            }
        }
        k_global
    }
    /// Check coercivity: all diagonal entries of the assembled matrix are positive.
    pub fn is_coercive(&self) -> bool {
        let mat = self.assemble();
        (0..mat.n).all(|i| mat.get(i, i) > 0.0)
    }
}
/// Finite element mesh in 2D (triangular elements).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FEMesh2D {
    pub nodes: Vec<(f64, f64)>,
    pub elements: Vec<(usize, usize, usize)>,
    pub boundary_nodes: Vec<usize>,
}
#[allow(dead_code)]
impl FEMesh2D {
    pub fn new(nodes: Vec<(f64, f64)>, elements: Vec<(usize, usize, usize)>) -> Self {
        FEMesh2D {
            nodes,
            elements,
            boundary_nodes: Vec::new(),
        }
    }
    pub fn n_nodes(&self) -> usize {
        self.nodes.len()
    }
    pub fn n_elements(&self) -> usize {
        self.elements.len()
    }
    pub fn n_dof(&self) -> usize {
        self.n_nodes()
    }
    pub fn element_area(&self, elem_idx: usize) -> f64 {
        let (i, j, k) = self.elements[elem_idx];
        let (x1, y1) = self.nodes[i];
        let (x2, y2) = self.nodes[j];
        let (x3, y3) = self.nodes[k];
        ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs() / 2.0
    }
    pub fn total_area(&self) -> f64 {
        (0..self.n_elements()).map(|i| self.element_area(i)).sum()
    }
    pub fn aspect_ratio(&self, elem_idx: usize) -> f64 {
        let (i, j, k) = self.elements[elem_idx];
        let a = dist(self.nodes[i], self.nodes[j]);
        let b = dist(self.nodes[j], self.nodes[k]);
        let c = dist(self.nodes[k], self.nodes[i]);
        let max_edge = a.max(b).max(c);
        let area = self.element_area(elem_idx);
        if area < 1e-15 {
            return f64::INFINITY;
        }
        max_edge.powi(2) / (2.0 * (3.0f64.sqrt()) * area)
    }
}
/// Sparse stiffness matrix assembly.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StiffnessMatrix {
    pub n: usize,
    pub entries: Vec<(usize, usize, f64)>,
}
#[allow(dead_code)]
impl StiffnessMatrix {
    pub fn new(n: usize) -> Self {
        StiffnessMatrix {
            n,
            entries: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, row: usize, col: usize, val: f64) {
        self.entries.push((row, col, val));
    }
    pub fn n_nonzeros(&self) -> usize {
        self.entries.len()
    }
    pub fn apply(&self, x: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; self.n];
        for &(row, col, val) in &self.entries {
            if row < self.n && col < x.len() {
                result[row] += val * x[col];
            }
        }
        result
    }
    pub fn frobenius_norm(&self) -> f64 {
        self.entries
            .iter()
            .map(|(_, _, v)| v.powi(2))
            .sum::<f64>()
            .sqrt()
    }
}
/// The affine map Fₖ: K̂ → K for triangle with vertices p0, p1, p2.
/// Fₖ(ξ,η) = p0 + B * (ξ,η)ᵀ where B = [p1-p0 | p2-p0].
#[derive(Debug, Clone)]
pub struct AffineTriangleMap {
    /// First column of B: p1 - p0.
    pub b0: [f64; 2],
    /// Second column of B: p2 - p0.
    pub b1: [f64; 2],
    /// Translation vector p0.
    pub origin: [f64; 2],
}
impl AffineTriangleMap {
    /// Build the affine map for a triangle with given vertex coordinates.
    pub fn new(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> Self {
        AffineTriangleMap {
            b0: [p1[0] - p0[0], p1[1] - p0[1]],
            b1: [p2[0] - p0[0], p2[1] - p0[1]],
            origin: p0,
        }
    }
    /// Apply the map: Fₖ(ξ,η) = origin + B*(ξ,η)ᵀ.
    pub fn apply(&self, xi: f64, eta: f64) -> [f64; 2] {
        [
            self.origin[0] + self.b0[0] * xi + self.b1[0] * eta,
            self.origin[1] + self.b0[1] * xi + self.b1[1] * eta,
        ]
    }
    /// Jacobian determinant |det B| = 2 * area(K).
    pub fn det_j(&self) -> f64 {
        (self.b0[0] * self.b1[1] - self.b0[1] * self.b1[0]).abs()
    }
    /// Inverse transpose of B (for gradient transformation):
    /// If û = B⁻ᵀ ∇_x̂ φ̂ then ∇_x φ = B⁻ᵀ ∇_{x̂} φ̂.
    pub fn inv_bt(&self) -> [[f64; 2]; 2] {
        let det = self.b0[0] * self.b1[1] - self.b0[1] * self.b1[0];
        let inv_det = 1.0 / det;
        [
            [self.b1[1] * inv_det, -self.b0[1] * inv_det],
            [-self.b1[0] * inv_det, self.b0[0] * inv_det],
        ]
    }
    /// Transform reference gradient [gxi, geta] to physical gradient [gx, gy].
    pub fn transform_grad(&self, g_ref: [f64; 2]) -> [f64; 2] {
        let bt_inv = self.inv_bt();
        [
            bt_inv[0][0] * g_ref[0] + bt_inv[0][1] * g_ref[1],
            bt_inv[1][0] * g_ref[0] + bt_inv[1][1] * g_ref[1],
        ]
    }
}
/// Compute the Nitsche boundary penalty term contribution for a 1D Poisson problem.
///
/// The penalty parameter γ must satisfy γ > γ₀ (depends on the polynomial degree).
pub struct NitscheData1D {
    /// Penalty parameter γ.
    pub gamma: f64,
    /// Mesh size h at the boundary.
    pub h: f64,
    /// Value of the Dirichlet boundary condition g.
    pub g: f64,
}
impl NitscheData1D {
    /// Create Nitsche data with given parameters.
    pub fn new(gamma: f64, h: f64, g: f64) -> Self {
        NitscheData1D { gamma, h, g }
    }
    /// Penalty parameter γ/h appearing in the Nitsche bilinear form.
    pub fn penalty(&self) -> f64 {
        self.gamma / self.h
    }
    /// Test whether the coercivity threshold γ > 1 is satisfied.
    pub fn is_coercive(&self) -> bool {
        self.gamma > 1.0
    }
}
/// Adaptive mesh refinement using newest-vertex bisection.
///
/// Given a mesh and a per-element indicator η_K, refines all elements
/// with η_K ≥ θ * max_K η_K (maximum strategy).
pub struct AdaptiveMeshRefinement {
    /// The current mesh.
    pub mesh: TriangularMesh2D,
    /// Refinement threshold (fraction of maximum indicator).
    pub theta: f64,
}
impl AdaptiveMeshRefinement {
    /// Create an AMR instance with given threshold θ ∈ (0,1].
    pub fn new(mesh: TriangularMesh2D, theta: f64) -> Self {
        AdaptiveMeshRefinement { mesh, theta }
    }
    /// Compute per-element error indicators (residual-based).
    ///
    /// For simplicity, uses the element area as a surrogate indicator.
    pub fn compute_indicators(&self) -> Vec<f64> {
        (0..self.mesh.num_triangles())
            .map(|k| self.mesh.triangle_area(k))
            .collect()
    }
    /// Refine all triangles in the marked set using uniform 1→4 refinement.
    ///
    /// Each marked triangle is split into 4 sub-triangles by connecting edge midpoints.
    pub fn refine_marked(&mut self, marked: &[bool]) {
        let mut new_vertices = self.mesh.vertices.clone();
        let mut new_is_boundary = self.mesh.is_boundary.clone();
        let mut new_triangles = Vec::new();
        let mut edge_midpoints: std::collections::HashMap<(usize, usize), usize> =
            std::collections::HashMap::new();
        let n_tri = self.mesh.num_triangles();
        for tri_idx in 0..n_tri {
            let [i0, i1, i2] = self.mesh.triangles[tri_idx];
            if !marked[tri_idx] {
                new_triangles.push([i0, i1, i2]);
                continue;
            }
            let edges = [(i0, i1), (i1, i2), (i2, i0)];
            let mut mids = [0usize; 3];
            for (e, (a, b)) in edges.iter().enumerate() {
                let key = ((*a).min(*b), (*a).max(*b));
                let mid_idx = *edge_midpoints.entry(key).or_insert_with(|| {
                    let [x0, y0] = new_vertices[*a];
                    let [x1, y1] = new_vertices[*b];
                    let mid = [(x0 + x1) / 2.0, (y0 + y1) / 2.0];
                    new_vertices.push(mid);
                    let on_bdy = new_is_boundary[*a] && new_is_boundary[*b];
                    new_is_boundary.push(on_bdy);
                    new_vertices.len() - 1
                });
                mids[e] = mid_idx;
            }
            let [m01, m12, m20] = mids;
            new_triangles.push([i0, m01, m20]);
            new_triangles.push([m01, i1, m12]);
            new_triangles.push([m20, m12, i2]);
            new_triangles.push([m01, m12, m20]);
        }
        self.mesh = TriangularMesh2D {
            vertices: new_vertices,
            triangles: new_triangles,
            is_boundary: new_is_boundary,
        };
    }
    /// Perform one AFEM step: compute indicators, mark, refine.
    ///
    /// Returns the number of refined elements.
    pub fn step(&mut self) -> usize {
        let indicators = self.compute_indicators();
        let max_eta = indicators.iter().cloned().fold(0.0_f64, f64::max);
        let threshold = self.theta * max_eta;
        let marked: Vec<bool> = indicators.iter().map(|&e| e >= threshold).collect();
        let count = marked.iter().filter(|&&m| m).count();
        self.refine_marked(&marked);
        count
    }
    /// Run up to `max_steps` adaptive refinement steps.
    pub fn run(&mut self, max_steps: usize) -> Vec<usize> {
        (0..max_steps).map(|_| self.step()).collect()
    }
}
/// Finite element solver (simplified Poisson equation -Δu = f).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PoissonFESolver {
    pub mesh: FEMesh2D,
    pub dirichlet_bc: Vec<(usize, f64)>,
}
#[allow(dead_code)]
impl PoissonFESolver {
    pub fn new(mesh: FEMesh2D) -> Self {
        PoissonFESolver {
            mesh,
            dirichlet_bc: Vec::new(),
        }
    }
    pub fn add_dirichlet_bc(&mut self, node: usize, val: f64) {
        self.dirichlet_bc.push((node, val));
    }
    pub fn n_free_dofs(&self) -> usize {
        let bc_nodes: std::collections::HashSet<usize> =
            self.dirichlet_bc.iter().map(|(n, _)| *n).collect();
        self.mesh.n_nodes() - bc_nodes.len()
    }
    pub fn estimated_condition_number(&self) -> f64 {
        let h = (1.0 / self.mesh.n_elements() as f64).sqrt();
        1.0 / (h * h)
    }
}
/// Sparse matrix in CSR-like form (stored as dense for small systems).
#[derive(Debug, Clone)]
pub struct DenseMatrix {
    /// Number of rows (= columns for square matrices).
    pub n: usize,
    /// Matrix entries stored row-major.
    pub data: Vec<f64>,
}
impl DenseMatrix {
    /// Create a zero n×n matrix.
    pub fn zeros(n: usize) -> Self {
        DenseMatrix {
            n,
            data: vec![0.0; n * n],
        }
    }
    /// Get entry (i, j).
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.n + j]
    }
    /// Add value to entry (i, j).
    pub fn add(&mut self, i: usize, j: usize, val: f64) {
        self.data[i * self.n + j] += val;
    }
    /// Matrix-vector product A * x.
    pub fn matvec(&self, x: &[f64]) -> Vec<f64> {
        assert_eq!(x.len(), self.n);
        let mut y = vec![0.0; self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                y[i] += self.get(i, j) * x[j];
            }
        }
        y
    }
}
/// A 2D triangular mesh with vertices and triangular elements.
#[derive(Debug, Clone)]
pub struct TriangularMesh2D {
    /// Vertex coordinates: vertices[i] = (x, y).
    pub vertices: Vec<[f64; 2]>,
    /// Triangular elements: triangles[k] = [i, j, l] (vertex indices, CCW).
    pub triangles: Vec<[usize; 3]>,
    /// Boundary node flags.
    pub is_boundary: Vec<bool>,
}
impl TriangularMesh2D {
    /// Create a structured triangular mesh on [0,1]² with n subdivisions per side.
    pub fn unit_square(n: usize) -> Self {
        let h = 1.0 / n as f64;
        let mut vertices = Vec::new();
        let mut is_boundary = Vec::new();
        for j in 0..=n {
            for i in 0..=n {
                let x = i as f64 * h;
                let y = j as f64 * h;
                vertices.push([x, y]);
                let on_bdy = i == 0 || i == n || j == 0 || j == n;
                is_boundary.push(on_bdy);
            }
        }
        let mut triangles = Vec::new();
        for j in 0..n {
            for i in 0..n {
                let v00 = j * (n + 1) + i;
                let v10 = v00 + 1;
                let v01 = v00 + (n + 1);
                let v11 = v01 + 1;
                triangles.push([v00, v10, v01]);
                triangles.push([v10, v11, v01]);
            }
        }
        TriangularMesh2D {
            vertices,
            triangles,
            is_boundary,
        }
    }
    /// Number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }
    /// Number of triangles.
    pub fn num_triangles(&self) -> usize {
        self.triangles.len()
    }
    /// Area of triangle k (positive if vertices are CCW).
    pub fn triangle_area(&self, k: usize) -> f64 {
        let [i, j, l] = self.triangles[k];
        let [x0, y0] = self.vertices[i];
        let [x1, y1] = self.vertices[j];
        let [x2, y2] = self.vertices[l];
        0.5 * ((x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0)).abs()
    }
    /// Total mesh area (sum of element areas).
    pub fn total_area(&self) -> f64 {
        (0..self.num_triangles())
            .map(|k| self.triangle_area(k))
            .sum()
    }
    /// Mesh size h = max element diameter (here: max edge length).
    pub fn mesh_size(&self) -> f64 {
        let mut h_max: f64 = 0.0;
        for tri in &self.triangles {
            let [i, j, l] = *tri;
            let edges = [(i, j), (j, l), (l, i)];
            for (a, b) in edges {
                let [x0, y0] = self.vertices[a];
                let [x1, y1] = self.vertices[b];
                let d = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
                if d > h_max {
                    h_max = d;
                }
            }
        }
        h_max
    }
}
/// Gauss quadrature rule for integration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GaussQuadrature {
    pub points: Vec<f64>,
    pub weights: Vec<f64>,
    pub degree_of_exactness: usize,
}
#[allow(dead_code)]
impl GaussQuadrature {
    pub fn new(points: Vec<f64>, weights: Vec<f64>, degree: usize) -> Self {
        assert_eq!(points.len(), weights.len());
        GaussQuadrature {
            points,
            weights,
            degree_of_exactness: degree,
        }
    }
    /// Gauss-Legendre 2-point rule on [-1, 1].
    pub fn gauss_legendre_2() -> Self {
        let p = 1.0 / (3.0f64).sqrt();
        GaussQuadrature::new(vec![-p, p], vec![1.0, 1.0], 3)
    }
    /// Gauss-Legendre 3-point rule on [-1, 1].
    pub fn gauss_legendre_3() -> Self {
        GaussQuadrature::new(
            vec![-0.7745967, 0.0, 0.7745967],
            vec![0.5555556, 0.8888889, 0.5555556],
            5,
        )
    }
    pub fn integrate<F: Fn(f64) -> f64>(&self, f: F) -> f64 {
        self.points
            .iter()
            .zip(self.weights.iter())
            .map(|(&x, &w)| w * f(x))
            .sum()
    }
    pub fn n_points(&self) -> usize {
        self.points.len()
    }
}
/// Abstract Galerkin stiffness matrix builder for bilinear forms of the type
///   a(u, v) = ∫_Ω k(x) ∇u · ∇v dx
/// with a scalar (possibly spatially variable) coefficient k(x).
pub struct GalerkinStiffnessMatrix {
    /// Diffusion coefficient function k(x, y).
    pub coeff: Box<dyn Fn(f64, f64) -> f64>,
    /// The mesh.
    pub mesh: TriangularMesh2D,
}
impl GalerkinStiffnessMatrix {
    /// Construct with constant diffusion coefficient.
    pub fn constant(k: f64, mesh: TriangularMesh2D) -> Self {
        GalerkinStiffnessMatrix {
            coeff: Box::new(move |_x, _y| k),
            mesh,
        }
    }
    /// Construct with spatially variable diffusion coefficient.
    pub fn variable(coeff: impl Fn(f64, f64) -> f64 + 'static, mesh: TriangularMesh2D) -> Self {
        GalerkinStiffnessMatrix {
            coeff: Box::new(coeff),
            mesh,
        }
    }
    /// Assemble the global stiffness matrix.
    ///
    /// Computes A_{ij} = ∫_Ω k(x) ∇φ_j · ∇φ_i dx using quadrature on each element.
    pub fn assemble(&self) -> DenseMatrix {
        let n_v = self.mesh.num_vertices();
        let mut k_global = DenseMatrix::zeros(n_v);
        let quad = reference_triangle_quadrature();
        for tri_idx in 0..self.mesh.num_triangles() {
            let [i0, i1, i2] = self.mesh.triangles[tri_idx];
            let p0 = self.mesh.vertices[i0];
            let p1 = self.mesh.vertices[i1];
            let p2 = self.mesh.vertices[i2];
            let fmap = AffineTriangleMap::new(p0, p1, p2);
            let det_j = fmap.det_j();
            let area = det_j / 2.0;
            let grads_phys = [
                fmap.transform_grad(p1_grad(0)),
                fmap.transform_grad(p1_grad(1)),
                fmap.transform_grad(p1_grad(2)),
            ];
            let cx = (p0[0] + p1[0] + p2[0]) / 3.0;
            let cy = (p0[1] + p1[1] + p2[1]) / 3.0;
            let k_val = (self.coeff)(cx, cy);
            let mut ke = [[0.0f64; 3]; 3];
            for a in 0..3 {
                for b in 0..3 {
                    let dot =
                        grads_phys[a][0] * grads_phys[b][0] + grads_phys[a][1] * grads_phys[b][1];
                    ke[a][b] = k_val * dot * area;
                }
            }
            let _ = &quad;
            let local_nodes = [i0, i1, i2];
            for a in 0..3 {
                for b in 0..3 {
                    k_global.add(local_nodes[a], local_nodes[b], ke[a][b]);
                }
            }
        }
        k_global
    }
}
/// Interior edge in a 1D DG mesh (shared between two elements).
#[derive(Debug, Clone)]
pub struct DGEdge1D {
    /// Left element index.
    pub left: usize,
    /// Right element index.
    pub right: usize,
    /// Position of the interface node.
    pub x: f64,
}
