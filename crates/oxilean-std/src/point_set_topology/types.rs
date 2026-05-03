//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// Paracompactness and partitions of unity.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PartitionOfUnity {
    pub space_name: String,
    pub open_cover_name: String,
}
impl PartitionOfUnity {
    #[allow(dead_code)]
    pub fn new(space: &str, cover: &str) -> Self {
        Self {
            space_name: space.to_string(),
            open_cover_name: cover.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn existence_condition(&self) -> String {
        format!(
            "Partition of unity subordinate to {} on {}: exists iff space is paracompact",
            self.open_cover_name, self.space_name
        )
    }
    #[allow(dead_code)]
    pub fn metric_spaces_are_paracompact(&self) -> bool {
        true
    }
}
/// Filter on a set.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Filter {
    pub base_set_name: String,
    pub is_ultrafilter: bool,
    pub is_principal: bool,
    pub generator: Option<String>,
}
impl Filter {
    #[allow(dead_code)]
    pub fn principal(set: &str, element: &str) -> Self {
        Self {
            base_set_name: set.to_string(),
            is_ultrafilter: false,
            is_principal: true,
            generator: Some(element.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn free_ultrafilter(set: &str) -> Self {
        Self {
            base_set_name: set.to_string(),
            is_ultrafilter: true,
            is_principal: false,
            generator: None,
        }
    }
    #[allow(dead_code)]
    pub fn converges_in_compact_space(&self) -> bool {
        self.is_ultrafilter
    }
}
/// The quotient topology induced by an equivalence relation on a
/// finite topological space.
///
/// Points are merged into equivalence classes; a subset of classes is open
/// iff its preimage is open in the original space.
#[derive(Debug, Clone)]
pub struct QuotientTopology {
    /// Number of equivalence classes.
    pub num_classes: usize,
    /// `class_of\[p\]` is the class index of point `p`.
    pub class_of: Vec<usize>,
    /// Open sets of the quotient (characteristic vectors over classes).
    pub open_sets: Vec<Vec<bool>>,
}
impl QuotientTopology {
    /// Build the quotient topology from a topological space and an
    /// equivalence relation given as a `Vec<usize>` mapping each point
    /// to its class index (0-based, contiguous).
    pub fn new(ambient: &TopologicalSpace, class_of: Vec<usize>) -> Self {
        assert_eq!(class_of.len(), ambient.points);
        let num_classes = class_of.iter().copied().max().map_or(0, |m| m + 1);
        let mut open_sets: Vec<Vec<bool>> = Vec::new();
        for mask in 0u64..(1u64 << num_classes) {
            let subset: Vec<bool> = (0..num_classes).map(|c| (mask >> c) & 1 == 1).collect();
            let preimage: Vec<bool> = (0..ambient.points).map(|p| subset[class_of[p]]).collect();
            if ambient.is_open(&preimage) && !open_sets.contains(&subset) {
                open_sets.push(subset);
            }
        }
        Self {
            num_classes,
            class_of,
            open_sets,
        }
    }
    /// Return `true` if `set` (over classes) is open.
    pub fn is_open(&self, set: &[bool]) -> bool {
        self.open_sets.iter().any(|o| o.as_slice() == set)
    }
}
/// Separation axioms (T0 through T4).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SeparationAxiom {
    T0,
    T1,
    T2,
    T2Half,
    T3,
    T3Half,
    T4,
}
impl SeparationAxiom {
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            Self::T0 => "T0 (Kolmogorov)",
            Self::T1 => "T1 (Frechet)",
            Self::T2 => "T2 (Hausdorff)",
            Self::T2Half => "T2.5 (Urysohn)",
            Self::T3 => "T3 (Regular + T1)",
            Self::T3Half => "T3.5 (Tychonoff)",
            Self::T4 => "T4 (Normal + T1)",
        }
    }
    #[allow(dead_code)]
    pub fn urysohn_lemma_applies(&self) -> bool {
        *self >= Self::T4
    }
    #[allow(dead_code)]
    pub fn tietze_extension_applies(&self) -> bool {
        *self >= Self::T4
    }
}
/// Metric space with a fixed metric (represented by name).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetricSpace2 {
    pub name: String,
    pub is_complete: bool,
    pub is_separable: bool,
    pub dimension: Option<usize>,
}
impl MetricSpace2 {
    #[allow(dead_code)]
    pub fn euclidean(n: usize) -> Self {
        Self {
            name: format!("R^{n}"),
            is_complete: true,
            is_separable: true,
            dimension: Some(n),
        }
    }
    #[allow(dead_code)]
    pub fn cantor_space() -> Self {
        Self {
            name: "Cantor set".to_string(),
            is_complete: true,
            is_separable: true,
            dimension: Some(0),
        }
    }
    #[allow(dead_code)]
    pub fn hilbert_space() -> Self {
        Self {
            name: "l^2 (Hilbert space)".to_string(),
            is_complete: true,
            is_separable: true,
            dimension: None,
        }
    }
    #[allow(dead_code)]
    pub fn baire_category_theorem_applies(&self) -> bool {
        self.is_complete
    }
    #[allow(dead_code)]
    pub fn is_polish_space(&self) -> bool {
        self.is_complete && self.is_separable
    }
}
/// Covering space and fundamental group data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CoveringSpaceData {
    pub base_space: String,
    pub cover_space: String,
    pub num_sheets: Option<usize>,
}
impl CoveringSpaceData {
    #[allow(dead_code)]
    pub fn new(base: &str, cover: &str, sheets: Option<usize>) -> Self {
        Self {
            base_space: base.to_string(),
            cover_space: cover.to_string(),
            num_sheets: sheets,
        }
    }
    #[allow(dead_code)]
    pub fn universal_cover(base: &str) -> Self {
        Self {
            base_space: base.to_string(),
            cover_space: format!("universal_cover({})", base),
            num_sheets: None,
        }
    }
    #[allow(dead_code)]
    pub fn is_universal(&self) -> bool {
        self.num_sheets.is_none()
    }
    #[allow(dead_code)]
    pub fn deck_transformations_description(&self) -> String {
        if self.is_universal() {
            format!(
                "Deck(cover) = pi_1({}) acts freely and properly discontinuously",
                self.base_space
            )
        } else {
            format!("Deck group has order <= {:?}", self.num_sheets)
        }
    }
}
/// Baire category theorem application.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BaireApplication {
    pub space_name: String,
    pub meager_sets: Vec<String>,
    pub dense_g_delta: String,
}
impl BaireApplication {
    #[allow(dead_code)]
    pub fn new(space: &str, meager: Vec<&str>, complement: &str) -> Self {
        Self {
            space_name: space.to_string(),
            meager_sets: meager.into_iter().map(String::from).collect(),
            dense_g_delta: complement.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn nowhere_dense_union_is_meager(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn generic_property_description(&self) -> String {
        format!(
            "{} is a generic (comeager) subset of {}",
            self.dense_g_delta, self.space_name
        )
    }
}
/// Homotopy equivalence data.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HomotopyEquivalence {
    pub space_a: String,
    pub space_b: String,
    pub homotopy_type: String,
}
impl HomotopyEquivalence {
    #[allow(dead_code)]
    pub fn new(a: &str, b: &str, ht: &str) -> Self {
        Self {
            space_a: a.to_string(),
            space_b: b.to_string(),
            homotopy_type: ht.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn contractible(space: &str) -> Self {
        Self::new(space, "point", "contractible")
    }
    #[allow(dead_code)]
    pub fn same_homology_groups(&self) -> bool {
        true
    }
    #[allow(dead_code)]
    pub fn same_homotopy_groups(&self) -> bool {
        true
    }
}
/// A finite topological space represented by its open sets as characteristic
/// vectors over `points` many points.
#[derive(Debug, Clone)]
pub struct TopologicalSpace {
    /// Number of points in the underlying set.
    pub points: usize,
    /// Each open set is a characteristic vector of length `points`.
    pub open_sets: Vec<Vec<bool>>,
}
impl TopologicalSpace {
    /// Create a new topological space with `n` points.
    ///
    /// The indiscrete topology (∅ and the whole space) is installed by
    /// default because it satisfies the open-set axioms.
    pub fn new(n: usize) -> Self {
        let empty = vec![false; n];
        let whole = vec![true; n];
        Self {
            points: n,
            open_sets: vec![empty, whole],
        }
    }
    /// Attempt to add a new open set (characteristic vector).
    ///
    /// The set is admitted only when the collection remains closed under
    /// binary unions and finite intersections.  The empty set and whole
    /// space are always present, so those axioms are pre-satisfied.
    pub fn add_open_set(&mut self, chars: Vec<bool>) {
        if chars.len() != self.points {
            return;
        }
        let mut candidate = self.open_sets.clone();
        candidate.push(chars);
        let n = candidate.len();
        for i in 0..n {
            for j in 0..n {
                let u: Vec<bool> = (0..self.points)
                    .map(|k| candidate[i][k] || candidate[j][k])
                    .collect();
                if !candidate.contains(&u) {
                    candidate.push(u);
                }
                let v: Vec<bool> = (0..self.points)
                    .map(|k| candidate[i][k] && candidate[j][k])
                    .collect();
                if !candidate.contains(&v) {
                    candidate.push(v);
                }
            }
        }
        self.open_sets = candidate;
    }
    /// Return `true` if `set` is one of the open sets.
    pub fn is_open(&self, set: &[bool]) -> bool {
        self.open_sets.iter().any(|o| o.as_slice() == set)
    }
    /// Return `true` if the complement of `set` is open (i.e. `set` is closed).
    pub fn is_closed(&self, set: &[bool]) -> bool {
        let complement: Vec<bool> = set.iter().map(|&b| !b).collect();
        self.is_open(&complement)
    }
    /// Compute the closure of `set` — the smallest closed superset.
    pub fn closure(&self, set: &[bool]) -> Vec<bool> {
        let mut result = vec![true; self.points];
        for open in &self.open_sets {
            let closed: Vec<bool> = open.iter().map(|&b| !b).collect();
            let is_superset = (0..self.points).all(|k| !set[k] || closed[k]);
            if is_superset {
                for k in 0..self.points {
                    result[k] = result[k] && closed[k];
                }
            }
        }
        result
    }
    /// Compute the interior of `set` — the largest open subset.
    pub fn interior(&self, set: &[bool]) -> Vec<bool> {
        let mut result = vec![false; self.points];
        for open in &self.open_sets {
            let is_subset = (0..self.points).all(|k| !open[k] || set[k]);
            if is_subset {
                for k in 0..self.points {
                    result[k] = result[k] || open[k];
                }
            }
        }
        result
    }
    /// Compute the boundary of `set`: closure minus interior.
    pub fn boundary(&self, set: &[bool]) -> Vec<bool> {
        let cl = self.closure(set);
        let int = self.interior(set);
        (0..self.points).map(|k| cl[k] && !int[k]).collect()
    }
    /// Check the T2 (Hausdorff) separation axiom for the finite topology.
    pub fn is_hausdorff(&self) -> bool {
        for i in 0..self.points {
            for j in (i + 1)..self.points {
                let found = self.open_sets.iter().any(|u| {
                    u[i] && !u[j]
                        && self
                            .open_sets
                            .iter()
                            .any(|v| v[j] && !v[i] && (0..self.points).all(|k| !(u[k] && v[k])))
                });
                if !found {
                    return false;
                }
            }
        }
        true
    }
    /// Check the T0 (Kolmogorov) separation axiom.
    ///
    /// For every pair of distinct points, some open set distinguishes them.
    pub fn is_t0(&self) -> bool {
        for i in 0..self.points {
            for j in (i + 1)..self.points {
                let distinguished = self.open_sets.iter().any(|u| u[i] != u[j]);
                if !distinguished {
                    return false;
                }
            }
        }
        true
    }
    /// Check the T1 separation axiom: every singleton is closed.
    pub fn is_t1(&self) -> bool {
        for p in 0..self.points {
            let mut singleton = vec![false; self.points];
            singleton[p] = true;
            if !self.is_closed(&singleton) {
                return false;
            }
        }
        true
    }
    /// Count connected components using union-find over the specialisation order.
    ///
    /// Two points are in the same component if they cannot be separated by
    /// any open set (i.e. every open set contains both or neither).
    pub fn connected_components(&self) -> Vec<usize> {
        let mut component = (0..self.points).collect::<Vec<_>>();
        fn find(comp: &mut Vec<usize>, x: usize) -> usize {
            if comp[x] != x {
                comp[x] = find(comp, comp[x]);
            }
            comp[x]
        }
        for i in 0..self.points {
            for j in (i + 1)..self.points {
                let always_together = self.open_sets.iter().all(|u| u[i] == u[j]);
                if always_together {
                    let ri = find(&mut component, i);
                    let rj = find(&mut component, j);
                    if ri != rj {
                        component[rj] = ri;
                    }
                }
            }
        }
        for i in 0..self.points {
            find(&mut component, i);
        }
        component
    }
    /// Return the number of connected components.
    pub fn num_connected_components(&self) -> usize {
        let comp = self.connected_components();
        let roots: std::collections::HashSet<usize> = comp.into_iter().collect();
        roots.len()
    }
    /// Check whether the space is connected (one component).
    pub fn is_connected(&self) -> bool {
        self.num_connected_components() == 1
    }
}
/// A finite metric space represented by an explicit distance matrix.
#[derive(Debug, Clone)]
pub struct MetricSpace {
    /// Number of points.
    pub n: usize,
    /// Distance matrix: `dist\[i\]\[j\]` is d(i, j).
    pub dist: Vec<Vec<f64>>,
}
impl MetricSpace {
    /// Create a new metric space with `n` points; all distances are 0.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            dist: vec![vec![0.0; n]; n],
        }
    }
    /// Set the distance between points `i` and `j` (symmetrically).
    pub fn set_dist(&mut self, i: usize, j: usize, d: f64) {
        if i < self.n && j < self.n {
            self.dist[i][j] = d;
            self.dist[j][i] = d;
        }
    }
    /// Return d(i, j).
    pub fn distance(&self, i: usize, j: usize) -> f64 {
        if i < self.n && j < self.n {
            self.dist[i][j]
        } else {
            f64::INFINITY
        }
    }
    /// Return all points within `radius` of `center`.
    pub fn ball(&self, center: usize, radius: f64) -> Vec<usize> {
        (0..self.n)
            .filter(|&p| self.dist[center][p] <= radius)
            .collect()
    }
    /// Every finite metric space is compact (trivially true).
    pub fn is_compact_finite(&self) -> bool {
        true
    }
    /// Return the diameter: sup d(x, y) over all pairs.
    pub fn diameter(&self) -> f64 {
        let mut d = 0.0_f64;
        for i in 0..self.n {
            for j in 0..self.n {
                if self.dist[i][j] > d {
                    d = self.dist[i][j];
                }
            }
        }
        d
    }
    /// Check (path-)connectedness in the finite setting using BFS.
    pub fn is_connected(&self) -> bool {
        if self.n == 0 {
            return true;
        }
        let mut visited = vec![false; self.n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0_usize);
        visited[0] = true;
        while let Some(cur) = queue.pop_front() {
            for next in 0..self.n {
                if !visited[next] && self.dist[cur][next] > 0.0 {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
        visited.iter().all(|&v| v)
    }
    /// Compute a minimum spanning tree length (Prim's algorithm).
    ///
    /// Returns the sum of edge weights in the MST.
    pub fn mst_length(&self) -> f64 {
        if self.n <= 1 {
            return 0.0;
        }
        let mut in_tree = vec![false; self.n];
        let mut min_edge = vec![f64::INFINITY; self.n];
        min_edge[0] = 0.0;
        let mut total = 0.0;
        for _ in 0..self.n {
            let u = (0..self.n)
                .filter(|&v| !in_tree[v])
                .min_by(|&a, &b| {
                    min_edge[a]
                        .partial_cmp(&min_edge[b])
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .expect("at least one vertex not in tree: loop runs n times for n vertices");
            in_tree[u] = true;
            total += min_edge[u];
            for v in 0..self.n {
                if !in_tree[v] && self.dist[u][v] < min_edge[v] {
                    min_edge[v] = self.dist[u][v];
                }
            }
        }
        total
    }
    /// Build the metric topology: the open sets are generated by open balls.
    pub fn to_topology(&self) -> TopologicalSpace {
        let mut ts = TopologicalSpace::new(self.n);
        let radii: Vec<f64> = {
            let mut rs: Vec<f64> = self
                .dist
                .iter()
                .flatten()
                .copied()
                .filter(|&d| d > 0.0)
                .collect();
            rs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            rs.dedup();
            rs
        };
        for p in 0..self.n {
            for &r in &radii {
                let mut ball = vec![false; self.n];
                for q in 0..self.n {
                    if self.dist[p][q] < r {
                        ball[q] = true;
                    }
                }
                ts.add_open_set(ball);
            }
        }
        ts
    }
}
/// Net (generalized sequence) in a topological space.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Net {
    pub directed_set_name: String,
    pub space_name: String,
    pub is_convergent: bool,
    pub limit_description: Option<String>,
}
impl Net {
    #[allow(dead_code)]
    pub fn new(ds: &str, space: &str) -> Self {
        Self {
            directed_set_name: ds.to_string(),
            space_name: space.to_string(),
            is_convergent: false,
            limit_description: None,
        }
    }
    #[allow(dead_code)]
    pub fn set_limit(&mut self, limit: &str) {
        self.is_convergent = true;
        self.limit_description = Some(limit.to_string());
    }
    #[allow(dead_code)]
    pub fn kelley_theorem_description(&self) -> String {
        "A space is compact iff every net has a convergent subnet (Kelley)".to_string()
    }
}
/// Topological dimension (Lebesgue covering dimension).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TopologicalDimension {
    pub space_name: String,
    pub lebesgue_covering_dim: Option<i64>,
    pub inductive_dim: Option<i64>,
    pub hausdorff_dim: Option<f64>,
}
impl TopologicalDimension {
    #[allow(dead_code)]
    pub fn new(space: &str, lcd: i64, ind: i64, hd: f64) -> Self {
        Self {
            space_name: space.to_string(),
            lebesgue_covering_dim: Some(lcd),
            inductive_dim: Some(ind),
            hausdorff_dim: Some(hd),
        }
    }
    #[allow(dead_code)]
    pub fn for_rn(n: usize) -> Self {
        Self::new(&format!("R^{n}"), n as i64, n as i64, n as f64)
    }
    #[allow(dead_code)]
    pub fn cantor_set() -> Self {
        Self::new("Cantor set", 0, 0, 2.0_f64.ln() / 3.0_f64.ln())
    }
    #[allow(dead_code)]
    pub fn sierpinski_triangle() -> Self {
        Self::new("Sierpinski triangle", 1, 1, 3.0_f64.ln() / 2.0_f64.ln())
    }
    #[allow(dead_code)]
    pub fn all_dimensions_agree(&self) -> bool {
        match (self.lebesgue_covering_dim, self.inductive_dim) {
            (Some(l), Some(i)) => l == i,
            _ => false,
        }
    }
}
/// The subspace topology induced on a subset `S` of a topological space.
///
/// Open sets are intersections of open sets of the ambient space with `S`.
#[derive(Debug, Clone)]
pub struct SubspaceTopology {
    /// The characteristic vector of the subspace within the ambient space.
    pub subspace: Vec<bool>,
    /// Open sets of the subspace (as indices into the subspace points).
    pub open_sets: Vec<Vec<bool>>,
    /// Number of points in the subspace.
    pub size: usize,
}
impl SubspaceTopology {
    /// Construct the subspace topology from an ambient `TopologicalSpace`
    /// and a characteristic vector `subspace`.
    pub fn new(ambient: &TopologicalSpace, subspace: Vec<bool>) -> Self {
        assert_eq!(subspace.len(), ambient.points);
        let indices: Vec<usize> = (0..ambient.points).filter(|&k| subspace[k]).collect();
        let size = indices.len();
        let mut open_sets: Vec<Vec<bool>> = Vec::new();
        for open in &ambient.open_sets {
            let restricted: Vec<bool> = indices.iter().map(|&k| open[k]).collect();
            if !open_sets.contains(&restricted) {
                open_sets.push(restricted);
            }
        }
        Self {
            subspace,
            open_sets,
            size,
        }
    }
    /// Return `true` if `set` (as a characteristic vector over the subspace) is open.
    pub fn is_open(&self, set: &[bool]) -> bool {
        self.open_sets.iter().any(|o| o.as_slice() == set)
    }
}
/// A (set-theoretic) map between finite spaces, represented as a lookup table.
#[derive(Debug, Clone)]
pub struct ContinuousMap {
    /// Number of elements in the domain.
    pub domain_size: usize,
    /// Number of elements in the codomain.
    pub codomain_size: usize,
    /// `mapping\[x\]` is the image of `x`; must have length `domain_size`.
    pub mapping: Vec<usize>,
}
impl ContinuousMap {
    /// Construct a new map.
    pub fn new(domain: usize, codomain: usize, f: Vec<usize>) -> Self {
        Self {
            domain_size: domain,
            codomain_size: codomain,
            mapping: f,
        }
    }
    /// Apply the map to `x`, returning `None` if `x` is out of range.
    pub fn apply(&self, x: usize) -> Option<usize> {
        self.mapping.get(x).copied()
    }
    /// Return `true` if the map is injective (one-to-one).
    pub fn is_injective(&self) -> bool {
        let mut seen = std::collections::HashSet::new();
        for &y in &self.mapping {
            if !seen.insert(y) {
                return false;
            }
        }
        true
    }
    /// Return `true` if the map is surjective (onto).
    pub fn is_surjective(&self) -> bool {
        let image: std::collections::HashSet<usize> = self.mapping.iter().copied().collect();
        image.len() == self.codomain_size
    }
    /// Return `true` if the map is bijective.
    pub fn is_bijective(&self) -> bool {
        self.is_injective() && self.is_surjective()
    }
    /// Compose `self` with `other`: compute `other ∘ self` (apply self first).
    pub fn compose(&self, other: &ContinuousMap) -> Option<ContinuousMap> {
        if self.codomain_size != other.domain_size {
            return None;
        }
        let composed: Vec<usize> = self.mapping.iter().map(|&y| other.mapping[y]).collect();
        Some(ContinuousMap::new(
            self.domain_size,
            other.codomain_size,
            composed,
        ))
    }
    /// Check topological continuity with respect to given topologies on
    /// domain and codomain.
    ///
    /// A map f is continuous iff for every open V ⊆ codomain, the preimage
    /// f⁻¹(V) is open in the domain.
    pub fn is_continuous(
        &self,
        domain_top: &TopologicalSpace,
        codomain_top: &TopologicalSpace,
    ) -> bool {
        assert_eq!(self.domain_size, domain_top.points);
        assert_eq!(self.codomain_size, codomain_top.points);
        for v in &codomain_top.open_sets {
            let preimage: Vec<bool> = (0..self.domain_size).map(|x| v[self.mapping[x]]).collect();
            if !domain_top.is_open(&preimage) {
                return false;
            }
        }
        true
    }
}
/// A finite topology together with its open-set lattice structure,
/// supporting joins (unions), meets (intersections), and density queries.
#[derive(Debug, Clone)]
pub struct FiniteTopology {
    inner: TopologicalSpace,
}
impl FiniteTopology {
    /// Build the discrete topology on `n` points (all subsets are open).
    pub fn discrete(n: usize) -> Self {
        let mut ts = TopologicalSpace::new(n);
        for p in 0..n {
            let mut s = vec![false; n];
            s[p] = true;
            ts.add_open_set(s);
        }
        Self { inner: ts }
    }
    /// Build the indiscrete topology on `n` points.
    pub fn indiscrete(n: usize) -> Self {
        Self {
            inner: TopologicalSpace::new(n),
        }
    }
    /// Return the number of open sets (the size of the topology lattice).
    pub fn lattice_size(&self) -> usize {
        self.inner.open_sets.len()
    }
    /// Join (union) of two open sets, returned as an open set index.
    pub fn join(&self, a: &[bool], b: &[bool]) -> Vec<bool> {
        (0..self.inner.points).map(|k| a[k] || b[k]).collect()
    }
    /// Meet (intersection) of two open sets.
    pub fn meet(&self, a: &[bool], b: &[bool]) -> Vec<bool> {
        (0..self.inner.points).map(|k| a[k] && b[k]).collect()
    }
    /// Return the number of points in the space.
    pub fn num_points(&self) -> usize {
        self.inner.points
    }
    /// Return a reference to the underlying `TopologicalSpace`.
    pub fn space(&self) -> &TopologicalSpace {
        &self.inner
    }
    /// Return the closure of a subset.
    pub fn closure(&self, set: &[bool]) -> Vec<bool> {
        self.inner.closure(set)
    }
    /// Return the interior of a subset.
    pub fn interior(&self, set: &[bool]) -> Vec<bool> {
        self.inner.interior(set)
    }
    /// Return the boundary of a subset.
    pub fn boundary(&self, set: &[bool]) -> Vec<bool> {
        self.inner.boundary(set)
    }
    /// Check whether `set` is dense: its closure is the whole space.
    pub fn is_dense(&self, set: &[bool]) -> bool {
        let cl = self.closure(set);
        cl.iter().all(|&b| b)
    }
}
/// The product topology on X × Y.
///
/// A basic open set is of the form U × V where U ⊆ X and V ⊆ Y are open.
/// We represent points in X × Y as pairs (i, j) linearised as `i * |Y| + j`.
#[derive(Debug, Clone)]
pub struct ProductTopology {
    /// Number of points in factor X.
    pub nx: usize,
    /// Number of points in factor Y.
    pub ny: usize,
    /// Open sets of the product topology.
    pub open_sets: Vec<Vec<bool>>,
}
impl ProductTopology {
    /// Build the product topology from the two factor topological spaces.
    pub fn new(x: &TopologicalSpace, y: &TopologicalSpace) -> Self {
        let nx = x.points;
        let ny = y.points;
        let total = nx * ny;
        let mut basic: Vec<Vec<bool>> = Vec::new();
        for u in &x.open_sets {
            for v in &y.open_sets {
                let prod: Vec<bool> = (0..total)
                    .map(|idx| {
                        let i = idx / ny;
                        let j = idx % ny;
                        u[i] && v[j]
                    })
                    .collect();
                if !basic.contains(&prod) {
                    basic.push(prod);
                }
            }
        }
        let mut open_sets = basic.clone();
        loop {
            let len = open_sets.len();
            let snapshot = open_sets.clone();
            for i in 0..snapshot.len() {
                for j in 0..snapshot.len() {
                    let u: Vec<bool> = (0..total)
                        .map(|k| snapshot[i][k] || snapshot[j][k])
                        .collect();
                    if !open_sets.contains(&u) {
                        open_sets.push(u);
                    }
                }
            }
            if open_sets.len() == len {
                break;
            }
        }
        Self { nx, ny, open_sets }
    }
    /// Return `true` if the product set (given as a flat characteristic vector
    /// of length `nx * ny`) is open in the product topology.
    pub fn is_open(&self, set: &[bool]) -> bool {
        self.open_sets.iter().any(|o| o.as_slice() == set)
    }
    /// Total number of points in X × Y.
    pub fn total_points(&self) -> usize {
        self.nx * self.ny
    }
}
/// Compactness witnesses.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactnessType {
    Compact,
    LocallyCompact,
    SigmaCompact,
    Lindelof,
    ParaCompact,
    NotCompact,
}
impl CompactnessType {
    #[allow(dead_code)]
    pub fn implies_lindelof(&self) -> bool {
        matches!(self, Self::Compact | Self::SigmaCompact | Self::Lindelof)
    }
    #[allow(dead_code)]
    pub fn heine_borel_in_rn(&self) -> bool {
        matches!(self, Self::Compact)
    }
}
/// Quotient topology.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct QuotientTopology2 {
    pub space_name: String,
    pub equivalence_relation: String,
    pub quotient_name: String,
}
impl QuotientTopology2 {
    #[allow(dead_code)]
    pub fn new(space: &str, relation: &str, quotient: &str) -> Self {
        Self {
            space_name: space.to_string(),
            equivalence_relation: relation.to_string(),
            quotient_name: quotient.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn circle_from_interval() -> Self {
        Self::new("[0,1]", "0 ~ 1", "S^1")
    }
    #[allow(dead_code)]
    pub fn torus_from_square() -> Self {
        Self::new("[0,1]x[0,1]", "(x,0)~(x,1), (0,y)~(1,y)", "T^2")
    }
    #[allow(dead_code)]
    pub fn is_hausdorff_iff(&self) -> String {
        format!(
            "Quotient {} is Hausdorff iff the graph of ~ is closed in {}x{}",
            self.quotient_name, self.space_name, self.space_name
        )
    }
}
/// Computes connected components of a finite topological space.
///
/// Two points are *topologically indistinguishable* (and hence in the same
/// component for the coarsest partition) when every open set contains both
/// or neither of them.  This matches the quasi-component decomposition for
/// finite spaces, which coincides with the connected-component decomposition.
#[derive(Debug, Clone)]
pub struct ConnectedComponents {
    /// The component label for each point.
    pub labels: Vec<usize>,
    /// Total number of components.
    pub count: usize,
}
impl ConnectedComponents {
    /// Compute connected components of `space`.
    pub fn compute(space: &TopologicalSpace) -> Self {
        let labels = space.connected_components();
        let mut map = std::collections::HashMap::new();
        let mut next_id = 0usize;
        let mut relabelled = vec![0usize; space.points];
        for (p, &raw) in labels.iter().enumerate() {
            let id = map.entry(raw).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });
            relabelled[p] = *id;
        }
        Self {
            labels: relabelled,
            count: next_id,
        }
    }
    /// Return the set of points in component `c` as a characteristic vector.
    pub fn component_set(&self, c: usize) -> Vec<bool> {
        self.labels.iter().map(|&l| l == c).collect()
    }
    /// Return the number of components.
    pub fn num_components(&self) -> usize {
        self.count
    }
}
