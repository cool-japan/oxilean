//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A persistence barcode: a collection of persistence intervals.
#[derive(Debug, Clone)]
pub struct PersistenceBarcode {
    /// All intervals, sorted by dimension then by birth.
    pub intervals: Vec<PersistenceInterval>,
}
impl PersistenceBarcode {
    /// Create an empty barcode.
    pub fn new() -> Self {
        PersistenceBarcode {
            intervals: Vec::new(),
        }
    }
    /// Add an interval.
    pub fn add(&mut self, dim: usize, birth: f64, death: Option<f64>) {
        self.intervals.push(PersistenceInterval {
            dimension: dim,
            birth,
            death,
        });
    }
    /// Return all intervals in dimension `k`.
    pub fn in_dimension(&self, k: usize) -> Vec<&PersistenceInterval> {
        self.intervals
            .iter()
            .filter(|iv| iv.dimension == k)
            .collect()
    }
    /// Betti number β_k at filtration value `t`:
    /// the number of intervals [b, d) in dimension k with b ≤ t < d (or d = None).
    pub fn betti_at(&self, k: usize, t: f64) -> usize {
        self.in_dimension(k)
            .iter()
            .filter(|iv| {
                iv.birth <= t
                    && match iv.death {
                        Some(d) => t < d,
                        None => true,
                    }
            })
            .count()
    }
    /// Total persistence (sum of lifetimes, ignoring essential classes).
    pub fn total_persistence(&self, k: usize) -> f64 {
        self.in_dimension(k)
            .iter()
            .filter_map(|iv| iv.death.map(|d| d - iv.birth))
            .sum()
    }
    /// Bottleneck distance between two barcodes in dimension k (approximate,
    /// O(n²) matching using the matching of intervals by midpoint).
    pub fn bottleneck_distance(d1: &PersistenceBarcode, d2: &PersistenceBarcode, k: usize) -> f64 {
        let iv1 = d1.in_dimension(k);
        let iv2 = d2.in_dimension(k);
        let fin1: Vec<(f64, f64)> = iv1
            .iter()
            .filter_map(|iv| iv.death.map(|d| (iv.birth, d)))
            .collect();
        let fin2: Vec<(f64, f64)> = iv2
            .iter()
            .filter_map(|iv| iv.death.map(|d| (iv.birth, d)))
            .collect();
        let dist =
            |a: (f64, f64), b: (f64, f64)| -> f64 { (a.0 - b.0).abs().max((a.1 - b.1).abs()) };
        let mut used = vec![false; fin2.len()];
        let mut max_d = 0.0f64;
        for &a in &fin1 {
            let mut best = f64::INFINITY;
            let mut best_j = None;
            for (j, &b) in fin2.iter().enumerate() {
                if !used[j] {
                    let d = dist(a, b);
                    if d < best {
                        best = d;
                        best_j = Some(j);
                    }
                }
            }
            if let Some(j) = best_j {
                used[j] = true;
                max_d = max_d.max(best);
            } else {
                max_d = max_d.max((a.1 - a.0) / 2.0);
            }
        }
        for (j, &b) in fin2.iter().enumerate() {
            if !used[j] {
                max_d = max_d.max((b.1 - b.0) / 2.0);
            }
        }
        max_d
    }
}
/// Chern character data for a complex vector bundle given by its Chern classes.
#[derive(Debug, Clone)]
pub struct ChernCharacterData {
    /// Rank of the bundle.
    pub rank: usize,
    /// Chern classes c_1, c_2, …, c_r stored as rational numbers (numerator, denominator).
    pub chern_classes: Vec<(i64, i64)>,
}
impl ChernCharacterData {
    /// Construct from a list of Chern class values (as integers = in H^{2k}(X; Z)).
    pub fn new(rank: usize, chern_classes: Vec<i64>) -> Self {
        ChernCharacterData {
            rank,
            chern_classes: chern_classes.into_iter().map(|c| (c, 1)).collect(),
        }
    }
    /// The first Chern class c_1 (or 0 if not set).
    pub fn c1(&self) -> i64 {
        self.chern_classes.first().map(|&(n, _)| n).unwrap_or(0)
    }
    /// The second Chern class c_2 (or 0).
    pub fn c2(&self) -> i64 {
        self.chern_classes.get(1).map(|&(n, _)| n).unwrap_or(0)
    }
    /// Todd class td_1 = c_1 / 2 expressed as a fraction (numerator, denominator).
    pub fn todd1(&self) -> (i64, i64) {
        let c1 = self.c1();
        (c1, 2)
    }
    /// Todd class td_2 = (c_1² + c_2) / 12 as a fraction.
    pub fn todd2(&self) -> (i64, i64) {
        let c1 = self.c1();
        let c2 = self.c2();
        (c1 * c1 + c2, 12)
    }
    /// Chern character ch_0 = rank.
    pub fn ch0(&self) -> i64 {
        self.rank as i64
    }
    /// Chern character ch_1 = c_1.
    pub fn ch1(&self) -> i64 {
        self.c1()
    }
    /// Chern character ch_2 = (c_1² - 2·c_2) / 2 as a fraction.
    pub fn ch2(&self) -> (i64, i64) {
        let c1 = self.c1();
        let c2 = self.c2();
        (c1 * c1 - 2 * c2, 2)
    }
}
/// An abstract simplicial complex given by its maximal simplices.
#[derive(Debug, Clone)]
pub struct SimplicialComplex {
    /// Name of the complex (for display).
    pub name: String,
    /// All simplices sorted by dimension.
    /// A k-simplex is represented as a sorted `Vec<usize>` of vertex indices.
    pub simplices: Vec<Vec<usize>>,
}
impl SimplicialComplex {
    /// Create an empty simplicial complex.
    pub fn new(name: &str) -> Self {
        SimplicialComplex {
            name: name.to_string(),
            simplices: Vec::new(),
        }
    }
    /// Add a simplex (given as a sorted list of vertex indices) and all its faces.
    pub fn add_simplex(&mut self, simplex: Vec<usize>) {
        let mut s = simplex;
        s.sort_unstable();
        self.add_all_faces(s);
        self.simplices.sort_unstable();
        self.simplices.dedup();
    }
    fn add_all_faces(&mut self, s: Vec<usize>) {
        if s.is_empty() {
            return;
        }
        if !self.simplices.contains(&s) {
            self.simplices.push(s.clone());
        }
        for i in 0..s.len() {
            let face: Vec<usize> = s[..i].iter().chain(s[i + 1..].iter()).copied().collect();
            if !face.is_empty() {
                self.add_all_faces(face);
            }
        }
    }
    /// Return all k-simplices.
    pub fn k_simplices(&self, k: usize) -> Vec<&Vec<usize>> {
        self.simplices.iter().filter(|s| s.len() == k + 1).collect()
    }
    /// Compute the boundary matrix ∂_k : C_k → C_{k-1} as a Vec<Vec<i32>>.
    /// Rows index (k-1)-simplices; columns index k-simplices.
    /// The sign convention: ∂[v_0,…,v_k] = Σ_i (-1)^i [v_0,…,v̂_i,…,v_k].
    pub fn boundary_matrix(&self, k: usize) -> Vec<Vec<i32>> {
        let k_sims = self.k_simplices(k);
        let km1_sims = self.k_simplices(k.saturating_sub(1));
        if k == 0 || km1_sims.is_empty() {
            return Vec::<Vec<i32>>::new();
        }
        let rows = km1_sims.len();
        let cols = k_sims.len();
        let mut mat = vec![vec![0i32; cols]; rows];
        for (col, ks) in k_sims.iter().enumerate() {
            for i in 0..ks.len() {
                let face: Vec<usize> = ks[..i].iter().chain(ks[i + 1..].iter()).copied().collect();
                if let Some(row) = km1_sims.iter().position(|s| **s == face) {
                    let sign: i32 = if i % 2 == 0 { 1 } else { -1 };
                    mat[row][col] += sign;
                }
            }
        }
        mat
    }
    /// Compute Betti numbers b_k = dim ker ∂_k - dim im ∂_{k+1}.
    pub fn betti_numbers(&self) -> Vec<u32> {
        let max_dim = self.simplices.iter().map(|s| s.len()).max().unwrap_or(0);
        if max_dim == 0 {
            return vec![];
        }
        let n = max_dim;
        let mut betti = vec![0u32; n];
        for k in 0..n {
            let rank_ck = self.k_simplices(k).len();
            let bm_k = self.boundary_matrix(k);
            let r_dk = matrix_rank(&bm_k);
            let bm_k1 = self.boundary_matrix(k + 1);
            let r_dk1 = matrix_rank(&bm_k1);
            let ker_dim = rank_ck.saturating_sub(r_dk);
            betti[k] = ker_dim.saturating_sub(r_dk1) as u32;
        }
        betti
    }
    /// The boundary of the standard n-simplex (= S^{n-1} triangulated).
    pub fn sphere_triangulation(n: usize) -> Self {
        let mut sc = SimplicialComplex::new(&format!("S^{}", n));
        let verts: Vec<usize> = (0..=n).collect();
        let full: Vec<usize> = verts.clone();
        for size in 1..=n {
            for combo in combinations(&full, size) {
                sc.simplices.push(combo);
            }
        }
        sc.simplices.sort_unstable();
        sc.simplices.dedup();
        sc
    }
    /// The torus as a minimal triangulation (7 vertices, 21 edges, 14 triangles).
    pub fn torus_triangulation() -> Self {
        let mut sc = SimplicialComplex::new("T^2 (7-vertex)");
        let triangles: &[[usize; 3]] = &[
            [0, 1, 2],
            [0, 2, 6],
            [0, 1, 5],
            [0, 5, 4],
            [0, 4, 6],
            [1, 2, 3],
            [1, 3, 5],
            [2, 3, 4],
            [2, 4, 6],
            [3, 4, 5],
            [3, 5, 6],
            [3, 4, 6],
            [1, 4, 5],
            [1, 4, 6],
        ];
        for tri in triangles {
            sc.add_simplex(tri.to_vec());
        }
        sc
    }
}
/// A finitely generated abelian group described by its invariant factors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomologyGroup {
    /// Free part rank (number of ℤ summands).
    pub free_rank: u32,
    /// Torsion invariant factors d₁ | d₂ | … | d_k, all > 1.
    pub torsion: Vec<u64>,
}
impl HomologyGroup {
    /// The trivial group.
    pub fn trivial() -> Self {
        HomologyGroup {
            free_rank: 0,
            torsion: vec![],
        }
    }
    /// ℤ (the integers).
    pub fn integers() -> Self {
        HomologyGroup {
            free_rank: 1,
            torsion: vec![],
        }
    }
    /// ℤ^r.
    pub fn free(r: u32) -> Self {
        HomologyGroup {
            free_rank: r,
            torsion: vec![],
        }
    }
    /// ℤ/nℤ.
    pub fn cyclic(n: u64) -> Self {
        HomologyGroup {
            free_rank: 0,
            torsion: vec![n],
        }
    }
    /// Human-readable description.
    pub fn description(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if self.free_rank > 0 {
            if self.free_rank == 1 {
                parts.push("Z".to_string());
            } else {
                parts.push(format!("Z^{}", self.free_rank));
            }
        }
        for &t in &self.torsion {
            parts.push(format!("Z/{t}Z"));
        }
        if parts.is_empty() {
            "0".to_string()
        } else {
            parts.join(" x ")
        }
    }
}
/// A word in the braid group B_n, given as a sequence of generators.
#[derive(Debug, Clone)]
pub struct BraidWord {
    /// Number of strands.
    pub strands: usize,
    /// The sequence of generators.
    pub generators: Vec<BraidGenerator>,
}
impl BraidWord {
    /// Create the trivial braid (identity).
    pub fn identity(strands: usize) -> Self {
        BraidWord {
            strands,
            generators: Vec::new(),
        }
    }
    /// Append a generator σ_i (positive or negative).
    pub fn push(&mut self, index: usize, positive: bool) {
        assert!(
            index >= 1 && index < self.strands,
            "index must be in 1..n-1"
        );
        self.generators.push(BraidGenerator { index, positive });
    }
    /// Cancel consecutive inverse generators: σ_i σ_i^{-1} = id.
    pub fn free_reduce(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            let mut i = 0;
            while i + 1 < self.generators.len() {
                let a = &self.generators[i];
                let b = &self.generators[i + 1];
                if a.index == b.index && a.positive != b.positive {
                    self.generators.remove(i + 1);
                    self.generators.remove(i);
                    changed = true;
                    if i > 0 {
                        i -= 1;
                    }
                } else {
                    i += 1;
                }
            }
        }
    }
    /// Length of the word after free reduction.
    pub fn length(&self) -> usize {
        let mut w = self.clone();
        w.free_reduce();
        w.generators.len()
    }
    /// Algebraic (signed) length: Σ sign(g_i).
    pub fn algebraic_length(&self) -> i64 {
        self.generators
            .iter()
            .map(|g| if g.positive { 1i64 } else { -1i64 })
            .sum()
    }
    /// Permutation induced by the braid (maps strand positions 0..n-1).
    pub fn permutation(&self) -> Vec<usize> {
        let mut perm: Vec<usize> = (0..self.strands).collect();
        for g in &self.generators {
            let i = g.index - 1;
            if g.positive {
                perm.swap(i, i + 1);
            } else {
                perm.swap(i, i + 1);
            }
        }
        perm
    }
    /// Returns `true` if this braid lies in the pure braid group (trivial permutation).
    pub fn is_pure(&self) -> bool {
        let id: Vec<usize> = (0..self.strands).collect();
        self.permutation() == id
    }
}
/// The Vietoris–Rips complex of a finite metric space at scale ε.
///
/// VR_ε(X) is the abstract simplicial complex whose simplices are finite
/// subsets S ⊆ X with diam(S) ≤ ε.
#[derive(Debug, Clone)]
pub struct VietorisRipsComplex {
    /// The underlying point cloud.
    pub points: Vec<EuclideanPoint>,
    /// The scale parameter ε.
    pub epsilon: f64,
    /// Maximum homological dimension computed.
    pub max_dim: usize,
}
impl VietorisRipsComplex {
    /// Build the Vietoris–Rips complex from a point cloud at scale `epsilon`,
    /// up to simplices of dimension `max_dim`.
    pub fn build(points: Vec<EuclideanPoint>, epsilon: f64, max_dim: usize) -> Self {
        VietorisRipsComplex {
            points,
            epsilon,
            max_dim,
        }
    }
    /// Enumerate all simplices up to `max_dim`.
    pub fn simplices(&self) -> Vec<Vec<usize>> {
        let n = self.points.len();
        let mut result = Vec::new();
        for i in 0..n {
            result.push(vec![i]);
        }
        for dim in 1..=self.max_dim {
            for combo in combinations(&(0..n).collect::<Vec<_>>(), dim + 1) {
                if self.is_simplex(&combo) {
                    result.push(combo);
                }
            }
        }
        result
    }
    /// Check whether a subset of points forms a simplex (all pairwise distances ≤ ε).
    pub fn is_simplex(&self, indices: &[usize]) -> bool {
        for i in 0..indices.len() {
            for j in (i + 1)..indices.len() {
                if self.points[indices[i]].distance(&self.points[indices[j]]) > self.epsilon {
                    return false;
                }
            }
        }
        true
    }
    /// Convert to a `SimplicialComplex` for homology computation.
    pub fn to_simplicial_complex(&self) -> SimplicialComplex {
        let mut sc = SimplicialComplex::new(&format!("VR(eps={})", self.epsilon));
        for simplex in self.simplices() {
            if simplex.len() > 1 {
                sc.add_simplex(simplex);
            } else {
                sc.simplices.push(simplex);
            }
        }
        sc.simplices.sort_unstable();
        sc.simplices.dedup();
        sc
    }
}
/// A single cell in a CW complex.
#[derive(Debug, Clone)]
pub struct CwCell {
    /// Dimension of the cell (0-cell, 1-cell, 2-cell, …).
    pub dimension: u32,
    /// Human-readable label.
    pub label: String,
    /// Boundary attaching data: list of `(cell_index, degree)` pairs.
    pub attaching: Vec<(usize, i32)>,
}
/// Abstract description of a homotopy group.
#[derive(Debug, Clone)]
pub struct HomotopyGroupData {
    /// Name of the space.
    pub space: String,
    /// Description of the base point.
    pub base_point: String,
    /// Degree n of the group πₙ.
    pub degree: u32,
    /// Human-readable description, e.g. "Z", "Z/2Z", "trivial", "unknown".
    pub description: String,
    /// Whether the group is abelian.
    pub is_abelian: bool,
}
/// A filtered simplicial complex (a nested sequence of subcomplexes).
#[derive(Debug, Clone)]
pub struct FiltrationComplex {
    /// Simplices in order of filtration value (ties broken by dimension then lex).
    pub simplices: Vec<FilteredSimplex>,
}
impl FiltrationComplex {
    /// Create an empty filtration.
    pub fn new() -> Self {
        FiltrationComplex {
            simplices: Vec::new(),
        }
    }
    /// Add a simplex at a given filtration value.
    pub fn add(&mut self, vertices: Vec<usize>, value: f64) {
        let mut v = vertices;
        v.sort_unstable();
        self.simplices.push(FilteredSimplex {
            vertices: v,
            filtration_value: value,
        });
    }
    /// Sort simplices: first by filtration value, then by dimension (ascending),
    /// then lexicographically.  This gives a valid filtration ordering.
    pub fn sort(&mut self) {
        self.simplices.sort_by(|a, b| {
            a.filtration_value
                .partial_cmp(&b.filtration_value)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.vertices.len().cmp(&b.vertices.len()))
                .then(a.vertices.cmp(&b.vertices))
        });
    }
    /// Build a Vietoris–Rips filtration from a point cloud for a sequence of
    /// ε values up to `max_epsilon` with `steps` steps and up to `max_dim`
    /// simplices per scale.
    pub fn from_vietoris_rips(
        points: &[EuclideanPoint],
        max_epsilon: f64,
        steps: usize,
        max_dim: usize,
    ) -> Self {
        let mut filt = FiltrationComplex::new();
        let n = points.len();
        for i in 0..n {
            filt.add(vec![i], 0.0);
        }
        for i in 0..n {
            for j in (i + 1)..n {
                let d = points[i].distance(&points[j]);
                if d <= max_epsilon {
                    filt.add(vec![i, j], d);
                }
            }
        }
        if max_dim >= 2 {
            let step_size = if steps > 0 {
                max_epsilon / steps as f64
            } else {
                max_epsilon
            };
            for k in 2..=max_dim {
                for combo in combinations(&(0..n).collect::<Vec<_>>(), k + 1) {
                    let mut max_edge = 0.0f64;
                    let mut valid = true;
                    for a in 0..combo.len() {
                        for b in (a + 1)..combo.len() {
                            let d = points[combo[a]].distance(&points[combo[b]]);
                            if d > max_epsilon {
                                valid = false;
                                break;
                            }
                            if d > max_edge {
                                max_edge = d;
                            }
                        }
                        if !valid {
                            break;
                        }
                    }
                    if valid {
                        let rounded = if step_size > 0.0 {
                            (max_edge / step_size).ceil() * step_size
                        } else {
                            max_edge
                        };
                        filt.add(combo, rounded);
                    }
                }
            }
        }
        filt.sort();
        filt
    }
}
/// An interval [birth, death) in a persistence barcode.
/// `death` is `None` for an essential (infinite) class.
#[derive(Debug, Clone, PartialEq)]
pub struct PersistenceInterval {
    /// Dimension (homological degree).
    pub dimension: usize,
    /// Birth time in the filtration.
    pub birth: f64,
    /// Death time (`None` = essential class, persists forever).
    pub death: Option<f64>,
}
impl PersistenceInterval {
    /// Lifetime (persistence) of the interval; returns `f64::INFINITY` for
    /// essential classes.
    pub fn lifetime(&self) -> f64 {
        match self.death {
            Some(d) => d - self.birth,
            None => f64::INFINITY,
        }
    }
    /// Returns `true` if this is an essential (infinite) class.
    pub fn is_essential(&self) -> bool {
        self.death.is_none()
    }
}
/// A chain complex C_n →^{d_n} C_{n-1} → … stored as boundary matrices.
pub struct ChainComplex {
    /// `boundary_matrices[k]` is the matrix of d_k: C_k → C_{k-1},
    /// stored as a `rank(C_{k-1}) × rank(C_k)` integer matrix.
    pub boundary_matrices: Vec<Vec<Vec<i32>>>,
    /// `ranks[k]` = rank(C_k), the number of k-cells.
    pub ranks: Vec<usize>,
}
impl ChainComplex {
    /// Build the chain complex from a CW complex using the attaching degrees.
    pub fn from_cw(cw: &CwComplex) -> Self {
        let max_dim = cw.cells.iter().map(|c| c.dimension).max().unwrap_or(0);
        let mut cells_by_dim: Vec<Vec<usize>> = vec![Vec::new(); max_dim as usize + 1];
        for (idx, cell) in cw.cells.iter().enumerate() {
            cells_by_dim[cell.dimension as usize].push(idx);
        }
        let ranks: Vec<usize> = cells_by_dim.iter().map(|v| v.len()).collect();
        let mut boundary_matrices: Vec<Vec<Vec<i32>>> = Vec::new();
        for k in 0..=(max_dim as usize) {
            let cols = cells_by_dim[k].len();
            let rows = if k == 0 { 0 } else { cells_by_dim[k - 1].len() };
            let mut mat = vec![vec![0i32; cols]; rows];
            if k > 0 {
                let mut row_of: std::collections::HashMap<usize, usize> =
                    std::collections::HashMap::new();
                for (row, &g) in cells_by_dim[k - 1].iter().enumerate() {
                    row_of.insert(g, row);
                }
                for (col, &g) in cells_by_dim[k].iter().enumerate() {
                    for &(nbr, deg) in &cw.cells[g].attaching {
                        if let Some(&row) = row_of.get(&nbr) {
                            mat[row][col] += deg;
                        }
                    }
                }
            }
            boundary_matrices.push(mat);
        }
        ChainComplex {
            boundary_matrices,
            ranks,
        }
    }
    /// Number of dimensions represented (= max_dim + 1).
    pub fn n_dimensions(&self) -> usize {
        self.boundary_matrices.len()
    }
    /// Compute Betti numbers b_k = dim ker(d_k) − dim im(d_{k+1}).
    ///
    /// We use a simple Gaussian elimination over the rationals (integer Gauss).
    pub fn betti_numbers(&self) -> Vec<u32> {
        let n = self.n_dimensions();
        let mut betti = vec![0u32; n];
        for k in 0..n {
            let rank_ck = self.ranks[k];
            let r_dk = matrix_rank(&self.boundary_matrices[k]);
            let r_dk1 = if k + 1 < n {
                matrix_rank(&self.boundary_matrices[k + 1])
            } else {
                0
            };
            let ker_dim = rank_ck.saturating_sub(r_dk);
            betti[k] = ker_dim.saturating_sub(r_dk1) as u32;
        }
        betti
    }
    /// Euler characteristic computed from Betti numbers: Σ_k (−1)^k b_k.
    pub fn euler_characteristic(&self) -> i64 {
        self.betti_numbers()
            .iter()
            .enumerate()
            .map(|(k, &b)| {
                let sign: i64 = if k % 2 == 0 { 1 } else { -1 };
                sign * b as i64
            })
            .sum()
    }
}
/// A finite CW complex.
#[derive(Debug, Clone)]
pub struct CwComplex {
    /// Name of the space (e.g., "S^2", "T^2").
    pub name: String,
    /// Ordered list of cells.
    pub cells: Vec<CwCell>,
}
impl CwComplex {
    /// Create an empty CW complex with the given name.
    pub fn new(name: &str) -> Self {
        CwComplex {
            name: name.to_string(),
            cells: Vec::new(),
        }
    }
    /// Append a new cell of the given dimension.
    pub fn add_cell(&mut self, dim: u32, label: &str, attaching: Vec<(usize, i32)>) {
        self.cells.push(CwCell {
            dimension: dim,
            label: label.to_string(),
            attaching,
        });
    }
    /// Count the number of cells of dimension `dim`.
    pub fn n_cells(&self, dim: u32) -> usize {
        self.cells.iter().filter(|c| c.dimension == dim).count()
    }
    /// Euler characteristic χ = Σ_k (−1)^k · #{k-cells}.
    pub fn euler_characteristic(&self) -> i64 {
        let mut max_dim = 0u32;
        for c in &self.cells {
            if c.dimension > max_dim {
                max_dim = c.dimension;
            }
        }
        (0..=max_dim)
            .map(|k| {
                let sign: i64 = if k % 2 == 0 { 1 } else { -1 };
                sign * self.n_cells(k) as i64
            })
            .sum()
    }
    /// S^n: one 0-cell `pt` and one n-cell `e_n`.
    pub fn sphere(n: u32) -> Self {
        let mut cw = CwComplex::new(&format!("S^{n}"));
        cw.add_cell(0, "pt", vec![]);
        if n > 0 {
            cw.add_cell(n, &format!("e_{n}"), vec![]);
        }
        cw
    }
    /// T² (torus): one 0-cell, two 1-cells (a, b), one 2-cell.
    pub fn torus() -> Self {
        let mut cw = CwComplex::new("T^2");
        cw.add_cell(0, "pt", vec![]);
        cw.add_cell(1, "a", vec![(0, 0)]);
        cw.add_cell(1, "b", vec![(0, 0)]);
        cw.add_cell(2, "F", vec![(1, 1), (2, 1), (1, -1), (2, -1)]);
        cw
    }
    /// RP²: one 0-cell, one 1-cell, one 2-cell.
    pub fn real_projective_plane() -> Self {
        let mut cw = CwComplex::new("RP^2");
        cw.add_cell(0, "pt", vec![]);
        cw.add_cell(1, "a", vec![(0, 0)]);
        cw.add_cell(2, "F", vec![(1, 2)]);
        cw
    }
    /// Klein bottle: one 0-cell, two 1-cells (a, b), one 2-cell.
    pub fn klein_bottle() -> Self {
        let mut cw = CwComplex::new("Klein bottle");
        cw.add_cell(0, "pt", vec![]);
        cw.add_cell(1, "a", vec![(0, 0)]);
        cw.add_cell(1, "b", vec![(0, 0)]);
        cw.add_cell(2, "F", vec![(1, 1), (2, 1), (1, 1), (2, -1)]);
        cw
    }
    /// D^n (disk): one 0-cell and one n-cell.
    pub fn disk(n: u32) -> Self {
        let mut cw = CwComplex::new(&format!("D^{n}"));
        cw.add_cell(0, "pt", vec![]);
        if n > 0 {
            cw.add_cell(n, &format!("d_{n}"), vec![]);
        }
        cw
    }
}
/// A point in ℝ^d.
#[derive(Debug, Clone)]
pub struct EuclideanPoint {
    /// Coordinates.
    pub coords: Vec<f64>,
}
impl EuclideanPoint {
    /// Create a new point with the given coordinates.
    pub fn new(coords: Vec<f64>) -> Self {
        EuclideanPoint { coords }
    }
    /// Euclidean distance to another point.
    pub fn distance(&self, other: &EuclideanPoint) -> f64 {
        self.coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt()
    }
}
/// A cohomology cochain: a map from k-simplices to ℤ.
#[derive(Debug, Clone)]
pub struct Cochain {
    /// Dimension of this cochain.
    pub degree: usize,
    /// Values on ordered simplices (indexed by position in the sorted simplex list).
    pub values: Vec<i32>,
}
impl Cochain {
    /// Create a zero cochain.
    pub fn zero(sc: &SimplicialComplex, degree: usize) -> Self {
        let k_sims = sc.k_simplices(degree);
        Cochain {
            degree,
            values: vec![0; k_sims.len()],
        }
    }
    /// Evaluate on a simplex (by index in the sorted list).
    pub fn eval(&self, simplex_idx: usize) -> i32 {
        self.values.get(simplex_idx).copied().unwrap_or(0)
    }
}
/// A generator in the braid group B_n.
/// σ_i is the standard positive braid generator; σ_i^{-1} is its inverse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BraidGenerator {
    /// Index i (1-based): σ_i crosses strand i over strand i+1.
    pub index: usize,
    /// `true` for σ_i, `false` for σ_i^{-1}.
    pub positive: bool,
}
/// A simplex with an associated filtration value (birth time).
#[derive(Debug, Clone)]
pub struct FilteredSimplex {
    /// The simplex as sorted vertex indices.
    pub vertices: Vec<usize>,
    /// Filtration value at which this simplex appears.
    pub filtration_value: f64,
}
