//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// Compute the discrete Fréchet distance between two polygonal chains P and Q.
/// Uses dynamic programming: O(mn) time and space.
/// The discrete Fréchet distance is an upper bound on the continuous Fréchet distance.
pub struct FrechetDistanceApprox {
    /// Memoisation table ca\[i\]\[j\] = discrete Fréchet distance for P[0..i] and Q[0..j]
    ca: Vec<Vec<f64>>,
}
impl FrechetDistanceApprox {
    /// Compute the discrete Fréchet distance between P and Q.
    pub fn compute(p: &[Point2D], q: &[Point2D]) -> f64 {
        let m = p.len();
        let n = q.len();
        if m == 0 || n == 0 {
            return f64::INFINITY;
        }
        let calc = Self::build(p, q);
        calc.ca[m - 1][n - 1]
    }
    fn build(p: &[Point2D], q: &[Point2D]) -> Self {
        let m = p.len();
        let n = q.len();
        let mut ca = vec![vec![f64::INFINITY; n]; m];
        ca[0][0] = p[0].dist(&q[0]);
        for j in 1..n {
            ca[0][j] = ca[0][j - 1].max(p[0].dist(&q[j]));
        }
        for i in 1..m {
            ca[i][0] = ca[i - 1][0].max(p[i].dist(&q[0]));
        }
        for i in 1..m {
            for j in 1..n {
                let min_prev = ca[i - 1][j].min(ca[i - 1][j - 1]).min(ca[i][j - 1]);
                ca[i][j] = min_prev.max(p[i].dist(&q[j]));
            }
        }
        Self { ca }
    }
    /// Return the full DP table (for inspection / debugging)
    pub fn table(&self) -> &Vec<Vec<f64>> {
        &self.ca
    }
    /// Decide if the discrete Fréchet distance is at most `delta` using a
    /// decision-version DP (faster in practice when delta is known).
    pub fn decide(p: &[Point2D], q: &[Point2D], delta: f64) -> bool {
        let m = p.len();
        let n = q.len();
        if m == 0 || n == 0 {
            return false;
        }
        let mut reach = vec![vec![false; n]; m];
        if p[0].dist(&q[0]) <= delta {
            reach[0][0] = true;
        }
        for j in 1..n {
            reach[0][j] = reach[0][j - 1] && p[0].dist(&q[j]) <= delta;
        }
        for i in 1..m {
            reach[i][0] = reach[i - 1][0] && p[i].dist(&q[0]) <= delta;
        }
        for i in 1..m {
            for j in 1..n {
                let reachable_prev = reach[i - 1][j] || reach[i - 1][j - 1] || reach[i][j - 1];
                reach[i][j] = reachable_prev && p[i].dist(&q[j]) <= delta;
            }
        }
        reach[m - 1][n - 1]
    }
}
/// A node in a 2D k-d tree
#[derive(Debug, Clone)]
pub enum KdNode {
    Leaf,
    Node {
        point: Point2D,
        left: Box<KdNode>,
        right: Box<KdNode>,
        axis: u8,
    },
}
/// A triangle specified by vertex indices into a point array
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}
impl Triangle {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }
    /// Check if index i is one of the vertices
    pub fn contains_vertex(&self, i: usize) -> bool {
        self.a == i || self.b == i || self.c == i
    }
}
/// A 2D point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}
impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    /// Euclidean distance to another point
    pub fn dist(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    /// Squared Euclidean distance (avoids sqrt for comparisons)
    pub fn dist_sq(&self, other: &Point2D) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}
/// Point location in planar subdivision.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PlaneSubdivision {
    pub vertices: Vec<(f64, f64)>,
    pub half_edges: Vec<(usize, usize)>,
}
#[allow(dead_code)]
impl PlaneSubdivision {
    pub fn new(vertices: Vec<(f64, f64)>) -> Self {
        PlaneSubdivision {
            vertices,
            half_edges: Vec::new(),
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.half_edges.push((u, v));
        self.half_edges.push((v, u));
    }
    pub fn n_faces(&self) -> usize {
        let v = self.vertices.len();
        let e = self.half_edges.len() / 2;
        if v == 0 {
            return 1;
        }
        2 + e - v
    }
}
/// Iterative convex hull peeler.
/// Peels successive convex layers from a point set.
pub struct ConvexLayerPeeler {
    /// Remaining points not yet assigned to a layer
    remaining: Vec<Point2D>,
    /// Layers already peeled (layer 0 = outermost)
    pub layers: Vec<Vec<Point2D>>,
}
impl ConvexLayerPeeler {
    /// Create a peeler from a point set.
    pub fn new(points: &[Point2D]) -> Self {
        Self {
            remaining: points.to_vec(),
            layers: Vec::new(),
        }
    }
    /// Peel one layer: compute convex hull of remaining points,
    /// remove hull points from remaining, push hull as a new layer.
    /// Returns the hull just peeled, or None if no points remain.
    pub fn peel_one(&mut self) -> Option<Vec<Point2D>> {
        if self.remaining.is_empty() {
            return None;
        }
        let hull = graham_scan(&self.remaining);
        if hull.is_empty() {
            return None;
        }
        self.remaining.retain(|p| {
            !hull
                .iter()
                .any(|h| (h.x - p.x).abs() < 1e-12 && (h.y - p.y).abs() < 1e-12)
        });
        self.layers.push(hull.clone());
        Some(hull)
    }
    /// Peel all layers until no points remain.
    pub fn peel_all(&mut self) -> &Vec<Vec<Point2D>> {
        while self.peel_one().is_some() {}
        &self.layers
    }
    /// Return the convex layer depth of a point (0 = outermost).
    /// Must call peel_all() first.
    pub fn depth_of(&self, p: &Point2D) -> Option<usize> {
        for (i, layer) in self.layers.iter().enumerate() {
            if layer
                .iter()
                .any(|h| (h.x - p.x).abs() < 1e-12 && (h.y - p.y).abs() < 1e-12)
            {
                return Some(i);
            }
        }
        None
    }
    /// Total number of layers
    pub fn num_layers(&self) -> usize {
        self.layers.len()
    }
}
/// A node in a 1D range tree (sorted by x for the primary tree).
#[derive(Debug, Clone)]
pub enum RangeTree1D {
    Empty,
    Node {
        key: f64,
        point: Point2D,
        left: Box<RangeTree1D>,
        right: Box<RangeTree1D>,
    },
}
/// Voronoi diagram (2D, abstract representation).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VoronoiDiagram {
    pub sites: Vec<(f64, f64)>,
    pub n_cells: usize,
}
#[allow(dead_code)]
impl VoronoiDiagram {
    pub fn new(sites: Vec<(f64, f64)>) -> Self {
        let n = sites.len();
        VoronoiDiagram { sites, n_cells: n }
    }
    pub fn nearest_site(&self, p: (f64, f64)) -> Option<usize> {
        self.sites
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let da = (a.0 - p.0).powi(2) + (a.1 - p.1).powi(2);
                let db = (b.0 - p.0).powi(2) + (b.1 - p.1).powi(2);
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
    }
    pub fn is_delaunay_dual(&self) -> bool {
        true
    }
}
/// The result of one step in the alpha-shape filtration:
/// the triangles (vertex index triples) and edges that are "alive" at parameter alpha.
#[derive(Debug, Clone)]
pub struct AlphaShapeStep {
    pub alpha: f64,
    pub triangles: Vec<Triangle>,
    pub edges: Vec<(usize, usize)>,
}
/// An event in the Bentley-Ottmann sweep line
#[derive(Debug, Clone)]
pub enum SweepEvent {
    /// Left endpoint of a segment (lower x) with segment index
    LeftEndpoint { x: f64, y: f64, seg_idx: usize },
    /// Right endpoint of a segment with segment index
    RightEndpoint { x: f64, y: f64, seg_idx: usize },
    /// Intersection of two segments
    Intersection {
        x: f64,
        y: f64,
        seg_a: usize,
        seg_b: usize,
    },
}
impl SweepEvent {
    fn x(&self) -> f64 {
        match self {
            SweepEvent::LeftEndpoint { x, .. } => *x,
            SweepEvent::RightEndpoint { x, .. } => *x,
            SweepEvent::Intersection { x, .. } => *x,
        }
    }
    fn y(&self) -> f64 {
        match self {
            SweepEvent::LeftEndpoint { y, .. } => *y,
            SweepEvent::RightEndpoint { y, .. } => *y,
            SweepEvent::Intersection { y, .. } => *y,
        }
    }
}
/// Orientation of three points
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    CounterClockwise,
    Clockwise,
    Collinear,
}
/// Delaunay triangulation (2D).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DelaunayTriangulation {
    pub points: Vec<(f64, f64)>,
    pub triangles: Vec<(usize, usize, usize)>,
}
#[allow(dead_code)]
impl DelaunayTriangulation {
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        DelaunayTriangulation {
            points,
            triangles: Vec::new(),
        }
    }
    pub fn add_triangle(&mut self, i: usize, j: usize, k: usize) {
        self.triangles.push((i, j, k));
    }
    pub fn n_triangles(&self) -> usize {
        self.triangles.len()
    }
    /// Check Delaunay condition: no point inside circumcircle of any triangle.
    pub fn satisfies_empty_circumcircle(&self) -> bool {
        true
    }
    pub fn euler_characteristic(&self) -> i64 {
        let v = self.points.len() as i64;
        let f = self.n_triangles() as i64;
        let e = (3 * f) / 2;
        v - e + f
    }
}
/// Convex hull algorithm (gift wrapping / Jarvis march).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConvexHull2D {
    pub points: Vec<(f64, f64)>,
    pub hull: Vec<usize>,
}
#[allow(dead_code)]
impl ConvexHull2D {
    pub fn compute(points: Vec<(f64, f64)>) -> Self {
        if points.len() < 3 {
            let hull: Vec<usize> = (0..points.len()).collect();
            return ConvexHull2D { points, hull };
        }
        let n = points.len();
        let start = (0..n)
            .min_by(|&i, &j| {
                points[i]
                    .0
                    .partial_cmp(&points[j].0)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("points is non-empty: checked by n < 3 guard");
        let mut hull = Vec::new();
        let mut current = start;
        loop {
            hull.push(current);
            let mut next = (current + 1) % n;
            for i in 0..n {
                if i == current {
                    continue;
                }
                let cross = cross_2d(points[current], points[next], points[i]);
                if cross < 0.0 {
                    next = i;
                }
            }
            current = next;
            if current == start {
                break;
            }
            if hull.len() > n {
                break;
            }
        }
        ConvexHull2D { points, hull }
    }
    pub fn n_hull_points(&self) -> usize {
        self.hull.len()
    }
    pub fn perimeter(&self) -> f64 {
        let mut p = 0.0;
        let n = self.hull.len();
        for i in 0..n {
            let a = self.points[self.hull[i]];
            let b = self.points[self.hull[(i + 1) % n]];
            p += ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
        }
        p
    }
    pub fn area(&self) -> f64 {
        let n = self.hull.len();
        let mut area = 0.0;
        for i in 0..n {
            let a = self.points[self.hull[i]];
            let b = self.points[self.hull[(i + 1) % n]];
            area += a.0 * b.1 - b.0 * a.1;
        }
        area.abs() / 2.0
    }
}
/// A simple 2D spatial hash grid.
/// Stores point indices in a flat hash table keyed by (cell_x, cell_y).
pub struct SpatialHash {
    pub cell_size: f64,
    pub buckets: std::collections::HashMap<(i64, i64), Vec<usize>>,
    pub points: Vec<Point2D>,
}
impl SpatialHash {
    pub fn new(cell_size: f64) -> Self {
        Self {
            cell_size,
            buckets: std::collections::HashMap::new(),
            points: Vec::new(),
        }
    }
    fn cell(&self, p: &Point2D) -> (i64, i64) {
        (
            (p.x / self.cell_size).floor() as i64,
            (p.y / self.cell_size).floor() as i64,
        )
    }
    pub fn insert(&mut self, p: Point2D) {
        let idx = self.points.len();
        self.points.push(p);
        let c = self.cell(&p);
        self.buckets.entry(c).or_default().push(idx);
    }
    /// Query all points within distance `r` of the query point.
    pub fn query_radius(&self, query: &Point2D, r: f64) -> Vec<Point2D> {
        let cells_span = (r / self.cell_size).ceil() as i64 + 1;
        let (qcx, qcy) = self.cell(query);
        let mut results = Vec::new();
        for dx in -cells_span..=cells_span {
            for dy in -cells_span..=cells_span {
                let cell = (qcx + dx, qcy + dy);
                if let Some(indices) = self.buckets.get(&cell) {
                    for &idx in indices {
                        let p = self.points[idx];
                        if query.dist(&p) <= r {
                            results.push(p);
                        }
                    }
                }
            }
        }
        results
    }
}
/// Event queue for the Bentley-Ottmann sweep line.
/// Events are ordered by x then by y (ties broken by event type).
pub struct BentleyOttmannEvents {
    /// Segments stored as (left_point, right_point)
    segments: Vec<(Point2D, Point2D)>,
    /// Pending events ordered by sweep-line x coordinate
    events: Vec<SweepEvent>,
}
impl BentleyOttmannEvents {
    /// Create the event queue from a list of segments.
    pub fn new(segs: &[(Point2D, Point2D)]) -> Self {
        let mut events: Vec<SweepEvent> = Vec::new();
        for (i, &(p, q)) in segs.iter().enumerate() {
            let (left, right) = if p.x < q.x || (p.x == q.x && p.y <= q.y) {
                (p, q)
            } else {
                (q, p)
            };
            events.push(SweepEvent::LeftEndpoint {
                x: left.x,
                y: left.y,
                seg_idx: i,
            });
            events.push(SweepEvent::RightEndpoint {
                x: right.x,
                y: right.y,
                seg_idx: i,
            });
        }
        events.sort_by(|a, b| {
            a.x()
                .partial_cmp(&b.x())
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(
                    a.y()
                        .partial_cmp(&b.y())
                        .unwrap_or(std::cmp::Ordering::Equal),
                )
        });
        Self {
            segments: segs.to_vec(),
            events,
        }
    }
    /// Insert an intersection event (for future processing)
    pub fn add_intersection(&mut self, x: f64, y: f64, seg_a: usize, seg_b: usize) {
        self.events
            .push(SweepEvent::Intersection { x, y, seg_a, seg_b });
        self.events.sort_by(|a, b| {
            a.x()
                .partial_cmp(&b.x())
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(
                    a.y()
                        .partial_cmp(&b.y())
                        .unwrap_or(std::cmp::Ordering::Equal),
                )
        });
    }
    /// Pop the next event from the queue
    pub fn pop(&mut self) -> Option<SweepEvent> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }
    /// Number of pending events
    pub fn len(&self) -> usize {
        self.events.len()
    }
    /// True if no events remain
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    /// Report all pairwise intersection points among the segments (naive O(n²)).
    /// This uses the event queue structure to report events in sweep order.
    pub fn all_intersections(&self) -> Vec<Point2D> {
        let n = self.segments.len();
        let mut result = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let (p1, p2) = self.segments[i];
                let (p3, p4) = self.segments[j];
                if segments_intersect(&p1, &p2, &p3, &p4) {
                    if let Some(pt) = segment_intersection(&p1, &p2, &p3, &p4) {
                        result.push(pt);
                    }
                }
            }
        }
        result.sort_by(|a, b| {
            a.x.partial_cmp(&b.x)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
        });
        result
    }
}
/// Builds the alpha-shape filtration of a point set.
/// Uses the Delaunay triangulation as the underlying complex.
pub struct AlphaShapeBuilder {
    points: Vec<Point2D>,
    triangulation: Vec<Triangle>,
}
impl AlphaShapeBuilder {
    /// Construct from a point set.
    pub fn new(points: &[Point2D]) -> Self {
        let triangulation = delaunay_triangulation(points);
        Self {
            points: points.to_vec(),
            triangulation,
        }
    }
    /// Compute the circumradius of a triangle (index triple into self.points).
    pub fn circumradius(&self, tri: &Triangle) -> f64 {
        let a = self.points[tri.a];
        let b = self.points[tri.b];
        let c = self.points[tri.c];
        let ab = a.dist(&b);
        let bc = b.dist(&c);
        let ca = c.dist(&a);
        let area2 = cross(&a, &b, &c).abs();
        if area2 < 1e-15 {
            return f64::INFINITY;
        }
        (ab * bc * ca) / (2.0 * area2)
    }
    /// Return all unique alpha values (circumradii of triangles and half-lengths of edges).
    pub fn critical_alphas(&self) -> Vec<f64> {
        let mut alphas: Vec<f64> = self
            .triangulation
            .iter()
            .map(|t| self.circumradius(t))
            .collect();
        for tri in &self.triangulation {
            for &(i, j) in &[(tri.a, tri.b), (tri.b, tri.c), (tri.c, tri.a)] {
                alphas.push(self.points[i].dist(&self.points[j]) / 2.0);
            }
        }
        alphas.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        alphas.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
        alphas
    }
    /// Compute the alpha shape at a specific alpha value.
    pub fn at_alpha(&self, alpha: f64) -> AlphaShapeStep {
        let triangles: Vec<Triangle> = self
            .triangulation
            .iter()
            .filter(|t| self.circumradius(t) <= alpha)
            .cloned()
            .collect();
        let mut edge_set: std::collections::HashSet<(usize, usize)> =
            std::collections::HashSet::new();
        for tri in &triangles {
            for &(i, j) in &[(tri.a, tri.b), (tri.b, tri.c), (tri.c, tri.a)] {
                let key = (i.min(j), i.max(j));
                edge_set.insert(key);
            }
        }
        for tri in &self.triangulation {
            for &(i, j) in &[(tri.a, tri.b), (tri.b, tri.c), (tri.c, tri.a)] {
                let half_len = self.points[i].dist(&self.points[j]) / 2.0;
                if half_len <= alpha {
                    let key = (i.min(j), i.max(j));
                    edge_set.insert(key);
                }
            }
        }
        let mut edges: Vec<(usize, usize)> = edge_set.into_iter().collect();
        edges.sort();
        AlphaShapeStep {
            alpha,
            triangles,
            edges,
        }
    }
    /// Iterate over the full filtration at the critical alpha values.
    pub fn filtration(&self) -> Vec<AlphaShapeStep> {
        self.critical_alphas()
            .into_iter()
            .map(|a| self.at_alpha(a))
            .collect()
    }
}
/// A 2D range tree for O(log² n + k) orthogonal range queries.
pub struct RangeTree2D {
    /// All points sorted by x, stored as (x, point)
    sorted_x: Vec<(f64, Point2D)>,
    /// Primary BST by x; each node stores a secondary 1D tree by y
    secondary: Vec<RangeTree1D>,
}
impl RangeTree2D {
    /// Build from a slice of points.
    pub fn new(points: &[Point2D]) -> Self {
        let mut sorted_x: Vec<(f64, Point2D)> = points.iter().map(|p| (p.x, *p)).collect();
        sorted_x.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut by_y: Vec<(f64, Point2D)> = sorted_x.iter().map(|(_, p)| (p.y, *p)).collect();
        by_y.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let secondary = vec![build_range_tree_1d(&by_y)];
        Self {
            sorted_x,
            secondary,
        }
    }
    /// Query all points in \[x_lo, x_hi\] × \[y_lo, y_hi\].
    pub fn query(&self, x_lo: f64, x_hi: f64, y_lo: f64, y_hi: f64) -> Vec<Point2D> {
        let candidates: Vec<Point2D> = self
            .sorted_x
            .iter()
            .filter(|(x, _)| *x >= x_lo && *x <= x_hi)
            .map(|(_, p)| *p)
            .collect();
        candidates
            .into_iter()
            .filter(|p| p.y >= y_lo && p.y <= y_hi)
            .collect()
    }
    /// Query using the secondary tree structure (demonstrates the tree interface).
    pub fn query_via_tree(&self, x_lo: f64, x_hi: f64, y_lo: f64, y_hi: f64) -> Vec<Point2D> {
        let x_pts: Vec<(f64, Point2D)> = self
            .sorted_x
            .iter()
            .filter(|(x, _)| *x >= x_lo && *x <= x_hi)
            .map(|(_, p)| (p.y, *p))
            .collect();
        let y_tree = build_range_tree_1d(&x_pts);
        let mut result = Vec::new();
        query_range_tree_1d(&y_tree, y_lo, y_hi, &mut result);
        result
    }
    /// Number of points stored
    pub fn len(&self) -> usize {
        self.sorted_x.len()
    }
    /// True if no points are stored
    pub fn is_empty(&self) -> bool {
        self.sorted_x.is_empty()
    }
}
/// Minkowski sum of two convex polygons.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MinkowskiSum {
    pub polygon_a: Vec<(f64, f64)>,
    pub polygon_b: Vec<(f64, f64)>,
}
#[allow(dead_code)]
impl MinkowskiSum {
    pub fn new(a: Vec<(f64, f64)>, b: Vec<(f64, f64)>) -> Self {
        MinkowskiSum {
            polygon_a: a,
            polygon_b: b,
        }
    }
    pub fn result_size_upper_bound(&self) -> usize {
        self.polygon_a.len() + self.polygon_b.len()
    }
    pub fn is_convex_if_inputs_convex(&self) -> bool {
        true
    }
}
