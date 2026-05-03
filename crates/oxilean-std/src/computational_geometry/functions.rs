//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AlphaShapeBuilder, BentleyOttmannEvents, ConvexHull2D, ConvexLayerPeeler,
    DelaunayTriangulation, FrechetDistanceApprox, KdNode, MinkowskiSum, Orientation,
    PlaneSubdivision, Point2D, RangeTree1D, RangeTree2D, SpatialHash, Triangle, VoronoiDiagram,
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
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
pub fn point2d_ty() -> Expr {
    pair_ty(real_ty(), real_ty())
}
pub fn list_point2d_ty() -> Expr {
    list_ty(point2d_ty())
}
pub fn list_nat_ty() -> Expr {
    list_ty(nat_ty())
}
/// `Point2D : Type` — a 2D point (x, y) ∈ ℝ²
pub fn point2d_kernel_ty() -> Expr {
    pair_ty(real_ty(), real_ty())
}
/// `Point3D : Type` — a 3D point (x, y, z) ∈ ℝ³
pub fn point3d_kernel_ty() -> Expr {
    pair_ty(real_ty(), pair_ty(real_ty(), real_ty()))
}
/// `Segment2D : Type` — a line segment in ℝ² defined by two endpoints
pub fn segment2d_ty() -> Expr {
    pair_ty(point2d_ty(), point2d_ty())
}
/// `Triangle2D : Type` — a triangle in ℝ² defined by three vertices
pub fn triangle2d_ty() -> Expr {
    pair_ty(point2d_ty(), pair_ty(point2d_ty(), point2d_ty()))
}
/// `Polygon2D : List Point2D -> Type` — polygon given as an ordered vertex list
pub fn polygon2d_ty() -> Expr {
    arrow(list_point2d_ty(), type0())
}
/// `ConvexHull : List Point2D -> List Point2D`
/// The convex hull function mapping a point set to its extremal vertices
pub fn convex_hull_fn_ty() -> Expr {
    arrow(list_point2d_ty(), list_point2d_ty())
}
/// `IsConvex : List Point2D -> Prop` — polygon is convex
pub fn is_convex_ty() -> Expr {
    arrow(list_point2d_ty(), prop())
}
/// `IsOnConvexHull : Point2D -> List Point2D -> Prop`
/// A point is on the convex hull of the set
pub fn is_on_convex_hull_ty() -> Expr {
    arrow(point2d_ty(), arrow(list_point2d_ty(), prop()))
}
/// `Triangulation : List Point2D -> List (Nat × Nat × Nat)`
/// A triangulation maps a point set to a list of triangles (vertex index triples)
pub fn triangulation_ty() -> Expr {
    let triple = pair_ty(nat_ty(), pair_ty(nat_ty(), nat_ty()));
    arrow(list_point2d_ty(), list_ty(triple))
}
/// `DelaunayTriangulation : List Point2D -> List (Nat × Nat × Nat)`
/// Delaunay triangulation: maximises minimum angle, empty circumcircle property
pub fn delaunay_triangulation_ty() -> Expr {
    triangulation_ty()
}
/// `IsDelaunay : List Point2D -> List (Nat × Nat × Nat) -> Prop`
/// Predicate: the triangulation satisfies the Delaunay condition
pub fn is_delaunay_ty() -> Expr {
    let triple = pair_ty(nat_ty(), pair_ty(nat_ty(), nat_ty()));
    arrow(list_point2d_ty(), arrow(list_ty(triple), prop()))
}
/// `VoronoiCell : List Point2D -> Nat -> (Point2D -> Prop)`
/// The i-th Voronoi cell: set of points closest to site i
pub fn voronoi_cell_ty() -> Expr {
    arrow(
        list_point2d_ty(),
        arrow(nat_ty(), arrow(point2d_ty(), prop())),
    )
}
/// `VoronoiDiagram : List Point2D -> List (List Point2D)`
/// Maps sites to their Voronoi cell boundary polygon vertices
pub fn voronoi_diagram_ty() -> Expr {
    arrow(list_point2d_ty(), list_ty(list_point2d_ty()))
}
/// `CircumcircleCenter : Point2D -> Point2D -> Point2D -> Point2D`
/// The circumcenter of three points
pub fn circumcircle_center_ty() -> Expr {
    arrow(
        point2d_ty(),
        arrow(point2d_ty(), arrow(point2d_ty(), point2d_ty())),
    )
}
/// `PointLocation : (List Point2D) -> Point2D -> Nat`
/// Returns the index of the region containing the query point
pub fn point_location_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(point2d_ty(), nat_ty()))
}
/// `KdTree : Nat -> Type` — a k-d tree for points in ℝ^k
pub fn kd_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `KdTreeQuery : KdTree -> Point2D -> Real -> List Point2D`
/// Range query: return all points within distance r of the query point
pub fn kd_tree_query_ty() -> Expr {
    arrow(
        type0(),
        arrow(point2d_ty(), arrow(real_ty(), list_point2d_ty())),
    )
}
/// `NearestNeighbour : (List Point2D) -> Point2D -> Point2D`
/// Returns the point in the set closest to the query
pub fn nearest_neighbour_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(point2d_ty(), point2d_ty()))
}
/// `ClosestPair : (List Point2D) -> (Point2D × Point2D)`
/// Returns the closest pair of distinct points
pub fn closest_pair_fn_ty() -> Expr {
    arrow(list_point2d_ty(), pair_ty(point2d_ty(), point2d_ty()))
}
/// `SegmentsIntersect : Segment2D -> Segment2D -> Prop`
/// Predicate: two line segments share a common point
pub fn segments_intersect_ty() -> Expr {
    arrow(segment2d_ty(), arrow(segment2d_ty(), prop()))
}
/// `SegmentIntersectionPoint : Segment2D -> Segment2D -> Point2D`
/// The intersection point of two segments (if it exists)
pub fn segment_intersection_point_ty() -> Expr {
    arrow(segment2d_ty(), arrow(segment2d_ty(), point2d_ty()))
}
/// `AnySegmentsIntersect : (List Segment2D) -> Prop`
/// Shamos-Hoey: at least one pair of segments intersects
pub fn any_segments_intersect_ty() -> Expr {
    arrow(list_ty(segment2d_ty()), prop())
}
/// `PolygonArea : List Point2D -> Real`
/// Signed area of a simple polygon (shoelace formula)
pub fn polygon_area_ty() -> Expr {
    arrow(list_point2d_ty(), real_ty())
}
/// `PolygonCentroid : List Point2D -> Point2D`
/// Centroid (centre of mass) of a simple polygon
pub fn polygon_centroid_ty() -> Expr {
    arrow(list_point2d_ty(), point2d_ty())
}
/// `PointInPolygon : Point2D -> List Point2D -> Prop`
/// Ray casting: point is strictly inside the polygon
pub fn point_in_polygon_ty() -> Expr {
    arrow(point2d_ty(), arrow(list_point2d_ty(), prop()))
}
/// `PolygonPerimeter : List Point2D -> Real`
pub fn polygon_perimeter_ty() -> Expr {
    arrow(list_point2d_ty(), real_ty())
}
/// `SpatialHash : Nat -> Type` — spatial hash map with grid resolution n
pub fn spatial_hash_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SpatialHashInsert : SpatialHash -> Point2D -> SpatialHash`
pub fn spatial_hash_insert_ty() -> Expr {
    arrow(type0(), arrow(point2d_ty(), type0()))
}
/// `Hyperplane : Nat -> Type` — a hyperplane in ℝ^d defined by a normal vector and offset
pub fn hyperplane_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `HyperplaneArrangement : List (Hyperplane d) -> Type`
/// A finite collection of hyperplanes inducing a cell decomposition
pub fn hyperplane_arrangement_ty() -> Expr {
    arrow(list_ty(type0()), type0())
}
/// `ZoneTheoremBound : Nat -> Nat`
/// Zone theorem: for a hyperplane h in an arrangement of n hyperplanes in ℝ^d,
/// the complexity of the zone of h is O(n^{d-1})
pub fn zone_theorem_bound_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `ArrangementFaceCount : Nat -> Nat -> Nat`
/// f(n, d) = number of faces (cells, facets, ...) in a simple arrangement
/// of n hyperplanes in ℝ^d
pub fn arrangement_face_count_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `CellOfArrangement : HyperplaneArrangement -> Point -> Nat`
/// Returns the index of the cell containing a given point
pub fn cell_of_arrangement_ty() -> Expr {
    arrow(type0(), arrow(point2d_ty(), nat_ty()))
}
/// `AlphaComplex : List Point2D -> Real -> Type`
/// The alpha complex at parameter alpha: a subcomplex of the Delaunay triangulation
pub fn alpha_complex_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(real_ty(), type0()))
}
/// `CechComplex : List Point2D -> Real -> Type`
/// The Čech complex at radius r: nerve of the union of balls of radius r
pub fn cech_complex_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(real_ty(), type0()))
}
/// `PersistenceDiagram : Type`
/// A persistence diagram: multiset of (birth, death) pairs encoding homology lifetimes
pub fn persistence_diagram_ty() -> Expr {
    list_ty(pair_ty(real_ty(), real_ty()))
}
/// `ComputePersistence : (Real -> Type) -> PersistenceDiagram`
/// Compute persistent homology from a filtration
pub fn compute_persistence_ty() -> Expr {
    arrow(arrow(real_ty(), type0()), persistence_diagram_ty())
}
/// `AlphaShapeFiltration : List Point2D -> List (Real × AlphaComplex)`
/// The full filtration from alpha = 0 to alpha = infinity
pub fn alpha_shape_filtration_ty() -> Expr {
    arrow(list_point2d_ty(), list_ty(pair_ty(real_ty(), type0())))
}
/// `KineticCertificate : Type`
/// A combinatorial predicate on moving objects that holds over a time interval
pub fn kinetic_certificate_ty() -> Expr {
    type0()
}
/// `KineticTournament : Nat -> Type`
/// A kinetic tournament on n moving points maintains the maximum
pub fn kinetic_tournament_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `KineticHeap : Nat -> Type`
/// A kinetic heap maintains the max of n objects with trajectories
pub fn kinetic_heap_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EventQueue : Type -> Type`
/// A priority queue of future events ordered by event time
pub fn event_queue_ty() -> Expr {
    arrow(type0(), type0())
}
/// `KineticConvexHull : Nat -> Type`
/// Maintains the convex hull of n moving points in the plane
pub fn kinetic_convex_hull_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ParametricSearchProblem : Type`
/// A decision problem parameterised by a real value lambda,
/// monotone in lambda (used by Cole's technique)
pub fn parametric_search_problem_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `ColeParallelBinarySearch : (Real -> Prop) -> Real`
/// Cole's parametric search: find the critical lambda in O(T_seq * log n) time
pub fn cole_parallel_binary_search_ty() -> Expr {
    arrow(arrow(real_ty(), prop()), real_ty())
}
/// `ParallelBinarySearch : List (Real × Real) -> List Real`
/// Simultaneous binary search over k sorted arrays
pub fn parallel_binary_search_ty() -> Expr {
    arrow(list_ty(pair_ty(real_ty(), real_ty())), list_ty(real_ty()))
}
/// `PointHyperplaneDual : Point2D -> (Real -> Prop)`
/// Point-hyperplane duality: maps point (a,b) to the hyperplane y = ax - b
pub fn point_hyperplane_dual_ty() -> Expr {
    arrow(point2d_ty(), arrow(real_ty(), prop()))
}
/// `GaleTransform : List Point2D -> List Point2D`
/// The Gale transform of a point configuration in general position
pub fn gale_transform_ty() -> Expr {
    arrow(list_point2d_ty(), list_point2d_ty())
}
/// `DualArrangement : HyperplaneArrangement -> List Point2D`
/// Convert a hyperplane arrangement to its dual point set
pub fn dual_arrangement_ty() -> Expr {
    arrow(type0(), list_point2d_ty())
}
/// `SweepLineState : Type`
/// The ordered sequence of segments active at the current sweep-line position
pub fn sweep_line_state_ty() -> Expr {
    list_ty(pair_ty(point2d_ty(), point2d_ty()))
}
/// `ShamosHoeyAlgorithm : List Segment2D -> Bool`
/// Returns true iff any two segments intersect — O(n log n)
pub fn shamos_hoey_algorithm_ty() -> Expr {
    arrow(list_ty(segment2d_ty()), bool_ty())
}
/// `BentleyOttmannOutput : List Segment2D -> List Point2D`
/// Reports all k intersection points of n segments in O((n+k) log n) time
pub fn bentley_ottmann_output_ty() -> Expr {
    arrow(list_ty(segment2d_ty()), list_point2d_ty())
}
/// `BentleyOttmannCorrectness : BentleyOttmannOutput s = AllIntersections s`
/// Correctness theorem for Bentley-Ottmann
pub fn bentley_ottmann_correctness_ty() -> Expr {
    arrow(list_ty(segment2d_ty()), prop())
}
/// `SteinerPoint : List Point2D -> Point2D`
/// A Steiner point inserted to improve mesh quality
pub fn steiner_point_ty() -> Expr {
    arrow(list_point2d_ty(), point2d_ty())
}
/// `DelaunayRefinement : List Point2D -> Real -> List Point2D`
/// Ruppert/Chew Delaunay refinement: insert Steiner points until
/// all angles >= min_angle (typically 20.7 degrees)
pub fn delaunay_refinement_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(real_ty(), list_point2d_ty()))
}
/// `MinAngleGuarantee : List Point2D -> Real -> Prop`
/// All triangles in the refined mesh have minimum angle >= threshold
pub fn min_angle_guarantee_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(real_ty(), prop()))
}
/// `TriangulationQuality : List Point2D -> Real`
/// The minimum angle over all triangles in a triangulation
pub fn triangulation_quality_ty() -> Expr {
    arrow(list_point2d_ty(), real_ty())
}
/// `VCDimension : (Point2D -> Prop) -> Nat`
/// VC dimension of a set system (concept class) on ℝ²
pub fn vc_dimension_ty() -> Expr {
    arrow(arrow(point2d_ty(), prop()), nat_ty())
}
/// `EpsilonNet : List Point2D -> Real -> List Point2D`
/// An epsilon-net: a subset R such that every heavy range contains a point of R
pub fn epsilon_net_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(real_ty(), list_point2d_ty()))
}
/// `HausslerPackingLemma : Nat -> Nat -> Nat`
/// Haussler packing lemma: upper bound on number of distinct projections
pub fn haussler_packing_lemma_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `EpsilonNetBound : Nat -> Real -> Nat`
/// Upper bound on epsilon-net size: O((d/eps) log (d/eps)) for VC-dim d
pub fn epsilon_net_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), nat_ty()))
}
/// `RangeTree : Nat -> Type`
/// A range tree for d-dimensional orthogonal range searching
pub fn range_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RangeTreeQuery : RangeTree -> (Real × Real) -> (Real × Real) -> List Point2D`
/// Orthogonal range query: report all points in axis-aligned box \[x1,x2\] × \[y1,y2\]
pub fn range_tree_query_ty() -> Expr {
    arrow(
        type0(),
        arrow(
            pair_ty(real_ty(), real_ty()),
            arrow(pair_ty(real_ty(), real_ty()), list_point2d_ty()),
        ),
    )
}
/// `FractionalCascading : List (List Real) -> Real -> List Nat`
/// Fractional cascading: simultaneously search k sorted arrays in O(k + log n) time
pub fn fractional_cascading_ty() -> Expr {
    arrow(list_ty(list_ty(real_ty())), arrow(real_ty(), list_nat_ty()))
}
/// `OrthogonalRangeSearching : List Point2D -> (Real × Real) -> (Real × Real) -> List Point2D`
/// 2D orthogonal range searching using a range tree
pub fn orthogonal_range_searching_ty() -> Expr {
    arrow(
        list_point2d_ty(),
        arrow(
            pair_ty(real_ty(), real_ty()),
            arrow(pair_ty(real_ty(), real_ty()), list_point2d_ty()),
        ),
    )
}
/// `SegmentTree : Type`
/// A segment tree storing intervals for stabbing/window queries
pub fn segment_tree_ty() -> Expr {
    type0()
}
/// `StabbingQuery : SegmentTree -> Real -> List (Real × Real)`
/// Report all intervals containing the query point
pub fn stabbing_query_ty() -> Expr {
    arrow(
        type0(),
        arrow(real_ty(), list_ty(pair_ty(real_ty(), real_ty()))),
    )
}
/// `IntervalIntersectionQuery : SegmentTree -> (Real × Real) -> List (Real × Real)`
/// Report all intervals overlapping a query interval
pub fn interval_intersection_query_ty() -> Expr {
    arrow(
        type0(),
        arrow(
            pair_ty(real_ty(), real_ty()),
            list_ty(pair_ty(real_ty(), real_ty())),
        ),
    )
}
/// `ConvexLayers : List Point2D -> List (List Point2D)`
/// Onion peeling: peel successive convex hulls until the set is exhausted
pub fn convex_layers_ty() -> Expr {
    arrow(list_point2d_ty(), list_ty(list_point2d_ty()))
}
/// `ConvexLayerDepth : List Point2D -> Point2D -> Nat`
/// The convex layer (depth) of a point: 0 = outermost hull
pub fn convex_layer_depth_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(point2d_ty(), nat_ty()))
}
/// `TukeyDepth : List Point2D -> Point2D -> Nat`
/// Tukey (halfspace) depth of a point with respect to a point set
pub fn tukey_depth_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(point2d_ty(), nat_ty()))
}
/// `KCenterClustering : List Point2D -> Nat -> List Point2D`
/// k-center clustering: find k centers minimising the maximum cluster radius
pub fn k_center_clustering_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(nat_ty(), list_point2d_ty()))
}
/// `KMedianClustering : List Point2D -> Nat -> List Point2D`
/// k-median clustering: find k centers minimising sum of distances
pub fn k_median_clustering_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(nat_ty(), list_point2d_ty()))
}
/// `FacilityLocation : List Point2D -> Real -> List Point2D`
/// Metric facility location: open facilities to minimise opening cost + assignment cost
pub fn facility_location_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(real_ty(), list_point2d_ty()))
}
/// `KCenterApproxRatio : Nat`
/// Best known approximation ratio for k-center: 2 (Gonzalez 1985)
pub fn k_center_approx_ratio_ty() -> Expr {
    nat_ty()
}
/// `HausdorffDistance : List Point2D -> List Point2D -> Real`
/// Hausdorff distance: max of directed Hausdorff distances
pub fn hausdorff_distance_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(list_point2d_ty(), real_ty()))
}
/// `DirectedHausdorff : List Point2D -> List Point2D -> Real`
/// Directed Hausdorff distance from A to B: max_{a in A} min_{b in B} d(a,b)
pub fn directed_hausdorff_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(list_point2d_ty(), real_ty()))
}
/// `FrechetDistance : List Point2D -> List Point2D -> Real`
/// Fréchet distance between two polygonal curves
pub fn frechet_distance_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(list_point2d_ty(), real_ty()))
}
/// `DiscreteFrechetDistance : List Point2D -> List Point2D -> Real`
/// Discrete Fréchet distance (dog-leash metric on vertices)
pub fn discrete_frechet_distance_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(list_point2d_ty(), real_ty()))
}
/// `ShapeMatchingOptimal : List Point2D -> List Point2D -> Real`
/// Minimum-cost shape matching under rigid motions
pub fn shape_matching_optimal_ty() -> Expr {
    arrow(list_point2d_ty(), arrow(list_point2d_ty(), real_ty()))
}
/// `ConfigurationSpace : Type -> Type`
/// The configuration space C-space of a robot: maps robot description to its C-space
pub fn configuration_space_ty() -> Expr {
    arrow(type0(), type0())
}
/// `FreeSpace : Type -> (Type -> Prop)`
/// The free configuration space: obstacle-free subset of C-space
pub fn free_space_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// `ProbabilisticRoadmap : Type -> Nat -> Type`
/// A probabilistic roadmap: random samples in free C-space connected by a graph
pub fn probabilistic_roadmap_ty() -> Expr {
    arrow(type0(), arrow(nat_ty(), type0()))
}
/// `RRTPath : Type -> Point2D -> Point2D -> List Point2D`
/// Rapidly-exploring random tree path from start to goal in free space
pub fn rrt_path_ty() -> Expr {
    arrow(
        type0(),
        arrow(point2d_ty(), arrow(point2d_ty(), list_point2d_ty())),
    )
}
/// `MotionPlanningCompleteness : Type -> Prop`
/// Completeness of a motion planner: finds a path iff one exists
pub fn motion_planning_completeness_ty() -> Expr {
    arrow(type0(), prop())
}
/// Build the computational geometry kernel environment.
pub fn build_computational_geometry_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Point2D", point2d_kernel_ty()),
        ("Point3D", point3d_kernel_ty()),
        ("Segment2D", segment2d_ty()),
        ("Triangle2D", triangle2d_ty()),
        ("Polygon2D", polygon2d_ty()),
        ("ConvexHullFn", convex_hull_fn_ty()),
        ("IsConvex", is_convex_ty()),
        ("IsOnConvexHull", is_on_convex_hull_ty()),
        ("TriangulationFn", triangulation_ty()),
        ("DelaunayTriangulationFn", delaunay_triangulation_ty()),
        ("IsDelaunay", is_delaunay_ty()),
        ("VoronoiCell", voronoi_cell_ty()),
        ("VoronoiDiagramFn", voronoi_diagram_ty()),
        ("CircumcircleCenter", circumcircle_center_ty()),
        ("PointLocationFn", point_location_ty()),
        ("KdTree", kd_tree_ty()),
        ("KdTreeQuery", kd_tree_query_ty()),
        ("NearestNeighbourFn", nearest_neighbour_ty()),
        ("ClosestPairFn", closest_pair_fn_ty()),
        ("SegmentsIntersect", segments_intersect_ty()),
        ("SegmentIntersectionPoint", segment_intersection_point_ty()),
        ("AnySegmentsIntersect", any_segments_intersect_ty()),
        ("PolygonArea", polygon_area_ty()),
        ("PolygonCentroid", polygon_centroid_ty()),
        ("PointInPolygon", point_in_polygon_ty()),
        ("PolygonPerimeter", polygon_perimeter_ty()),
        ("SpatialHash", spatial_hash_ty()),
        ("SpatialHashInsert", spatial_hash_insert_ty()),
        ("ConvexHullCorrectness", arrow(list_point2d_ty(), prop())),
        ("DelaunayMaxMinAngle", arrow(list_point2d_ty(), prop())),
        ("DelaunayUniqueness", arrow(list_point2d_ty(), prop())),
        ("VoronoiDelaunayDuality", arrow(list_point2d_ty(), prop())),
        (
            "ShamosHoeyCorrectness",
            arrow(list_ty(segment2d_ty()), prop()),
        ),
        (
            "ClosestPairOptimality",
            arrow(list_point2d_ty(), arrow(real_ty(), prop())),
        ),
        ("GrahamScanComplexity", nat_ty()),
        ("DelaunayComplexity", nat_ty()),
        ("FortuneAlgorithmComplexity", nat_ty()),
        ("HyperplaneTy", hyperplane_ty()),
        ("HyperplaneArrangementTy", hyperplane_arrangement_ty()),
        ("ZoneTheoremBound", zone_theorem_bound_ty()),
        ("ArrangementFaceCount", arrangement_face_count_ty()),
        ("CellOfArrangement", cell_of_arrangement_ty()),
        ("AlphaComplex", alpha_complex_ty()),
        ("CechComplex", cech_complex_ty()),
        ("PersistenceDiagramTy", persistence_diagram_ty()),
        ("ComputePersistence", compute_persistence_ty()),
        ("AlphaShapeFiltration", alpha_shape_filtration_ty()),
        ("KineticCertificate", kinetic_certificate_ty()),
        ("KineticTournament", kinetic_tournament_ty()),
        ("KineticHeap", kinetic_heap_ty()),
        ("EventQueue", event_queue_ty()),
        ("KineticConvexHull", kinetic_convex_hull_ty()),
        ("ParametricSearchProblem", parametric_search_problem_ty()),
        ("ColeParallelBinarySearch", cole_parallel_binary_search_ty()),
        ("ParallelBinarySearch", parallel_binary_search_ty()),
        ("PointHyperplaneDual", point_hyperplane_dual_ty()),
        ("GaleTransform", gale_transform_ty()),
        ("DualArrangement", dual_arrangement_ty()),
        ("SweepLineState", sweep_line_state_ty()),
        ("ShamosHoeyAlgorithm", shamos_hoey_algorithm_ty()),
        ("BentleyOttmannOutput", bentley_ottmann_output_ty()),
        (
            "BentleyOttmannCorrectness",
            bentley_ottmann_correctness_ty(),
        ),
        ("SteinerPoint", steiner_point_ty()),
        ("DelaunayRefinement", delaunay_refinement_ty()),
        ("MinAngleGuarantee", min_angle_guarantee_ty()),
        ("TriangulationQuality", triangulation_quality_ty()),
        ("VCDimension", vc_dimension_ty()),
        ("EpsilonNet", epsilon_net_ty()),
        ("HausslerPackingLemma", haussler_packing_lemma_ty()),
        ("EpsilonNetBound", epsilon_net_bound_ty()),
        ("RangeTree", range_tree_ty()),
        ("RangeTreeQuery", range_tree_query_ty()),
        ("FractionalCascading", fractional_cascading_ty()),
        ("OrthogonalRangeSearching", orthogonal_range_searching_ty()),
        ("SegmentTree", segment_tree_ty()),
        ("StabbingQuery", stabbing_query_ty()),
        (
            "IntervalIntersectionQuery",
            interval_intersection_query_ty(),
        ),
        ("ConvexLayers", convex_layers_ty()),
        ("ConvexLayerDepth", convex_layer_depth_ty()),
        ("TukeyDepth", tukey_depth_ty()),
        ("KCenterClustering", k_center_clustering_ty()),
        ("KMedianClustering", k_median_clustering_ty()),
        ("FacilityLocation", facility_location_ty()),
        ("KCenterApproxRatio", k_center_approx_ratio_ty()),
        ("HausdorffDistance", hausdorff_distance_ty()),
        ("DirectedHausdorff", directed_hausdorff_ty()),
        ("FrechetDistance", frechet_distance_ty()),
        ("DiscreteFrechetDistance", discrete_frechet_distance_ty()),
        ("ShapeMatchingOptimal", shape_matching_optimal_ty()),
        ("ConfigurationSpace", configuration_space_ty()),
        ("FreeSpace", free_space_ty()),
        ("ProbabilisticRoadmap", probabilistic_roadmap_ty()),
        ("RRTPath", rrt_path_ty()),
        (
            "MotionPlanningCompleteness",
            motion_planning_completeness_ty(),
        ),
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
/// 2D cross product of vectors (p1-p0) × (p2-p0)
/// Positive: p2 is to the left of p0→p1 (counter-clockwise)
/// Negative: p2 is to the right (clockwise)
/// Zero: collinear
pub fn cross(p0: &Point2D, p1: &Point2D, p2: &Point2D) -> f64 {
    (p1.x - p0.x) * (p2.y - p0.y) - (p1.y - p0.y) * (p2.x - p0.x)
}
/// Compute the orientation of three points
pub fn orientation(p: &Point2D, q: &Point2D, r: &Point2D) -> Orientation {
    let val = cross(p, q, r);
    if val > 1e-12 {
        Orientation::CounterClockwise
    } else if val < -1e-12 {
        Orientation::Clockwise
    } else {
        Orientation::Collinear
    }
}
/// Compute the convex hull of a set of points using Graham scan.
/// Returns vertices in counter-clockwise order.
pub fn graham_scan(points: &[Point2D]) -> Vec<Point2D> {
    let n = points.len();
    if n < 3 {
        return points.to_vec();
    }
    let pivot_idx = points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.y.partial_cmp(&b.y)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal))
        })
        .map(|(i, _)| i)
        .expect("points is non-empty: checked by caller");
    let pivot = points[pivot_idx];
    let mut sorted: Vec<Point2D> = points.to_vec();
    sorted.swap(0, pivot_idx);
    let p0 = sorted[0];
    sorted[1..].sort_by(|a, b| {
        let ca = cross(&p0, a, b);
        if ca.abs() < 1e-12 {
            p0.dist_sq(a)
                .partial_cmp(&p0.dist_sq(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        } else if ca > 0.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let mut hull: Vec<Point2D> = Vec::with_capacity(n);
    hull.push(sorted[0]);
    hull.push(sorted[1]);
    for i in 2..n {
        while hull.len() >= 2 {
            let m = hull.len();
            let cr = cross(&hull[m - 2], &hull[m - 1], &sorted[i]);
            if cr <= 0.0 {
                hull.pop();
            } else {
                break;
            }
        }
        hull.push(sorted[i]);
    }
    let _ = pivot;
    hull
}
/// Compute the convex hull using the Jarvis march (gift wrapping) algorithm.
/// O(nh) where h is the hull size.
pub fn jarvis_march(points: &[Point2D]) -> Vec<Point2D> {
    let n = points.len();
    if n < 3 {
        return points.to_vec();
    }
    let start = points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.x.partial_cmp(&b.x)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
        })
        .map(|(i, _)| i)
        .expect("points is non-empty: checked by n < 3 guard");
    let mut hull = Vec::new();
    let mut current = start;
    loop {
        hull.push(points[current]);
        let mut next = 0usize;
        for i in 1..n {
            if next == current {
                next = i;
                continue;
            }
            let c = cross(&points[current], &points[next], &points[i]);
            if c < -1e-12 {
                next = i;
            } else if c.abs() <= 1e-12 {
                if points[current].dist_sq(&points[i]) > points[current].dist_sq(&points[next]) {
                    next = i;
                }
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
    hull
}
/// Find the closest pair of points in O(n log n) time.
/// Returns (distance, point_a, point_b).
pub fn closest_pair(points: &[Point2D]) -> Option<(f64, Point2D, Point2D)> {
    if points.len() < 2 {
        return None;
    }
    let mut sorted_x: Vec<Point2D> = points.to_vec();
    sorted_x.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
    let result = closest_pair_rec(&sorted_x);
    Some(result)
}
pub fn closest_pair_rec(pts: &[Point2D]) -> (f64, Point2D, Point2D) {
    let n = pts.len();
    if n <= 3 {
        return brute_force_closest(pts);
    }
    let mid = n / 2;
    let mid_x = pts[mid].x;
    let (d_left, pa_l, pb_l) = closest_pair_rec(&pts[..mid]);
    let (d_right, pa_r, pb_r) = closest_pair_rec(&pts[mid..]);
    let (mut best_d, mut best_a, mut best_b) = if d_left <= d_right {
        (d_left, pa_l, pb_l)
    } else {
        (d_right, pa_r, pb_r)
    };
    let strip: Vec<Point2D> = pts
        .iter()
        .filter(|p| (p.x - mid_x).abs() < best_d)
        .cloned()
        .collect();
    let mut strip_y = strip.clone();
    strip_y.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal));
    for i in 0..strip_y.len() {
        for j in (i + 1)..strip_y.len() {
            if strip_y[j].y - strip_y[i].y >= best_d {
                break;
            }
            let d = strip_y[i].dist(&strip_y[j]);
            if d < best_d {
                best_d = d;
                best_a = strip_y[i];
                best_b = strip_y[j];
            }
        }
    }
    (best_d, best_a, best_b)
}
pub fn brute_force_closest(pts: &[Point2D]) -> (f64, Point2D, Point2D) {
    let mut best_d = f64::INFINITY;
    let mut best_a = pts[0];
    let mut best_b = pts[1.min(pts.len() - 1)];
    for i in 0..pts.len() {
        for j in (i + 1)..pts.len() {
            let d = pts[i].dist(&pts[j]);
            if d < best_d {
                best_d = d;
                best_a = pts[i];
                best_b = pts[j];
            }
        }
    }
    (best_d, best_a, best_b)
}
/// Compute the signed area of a simple polygon using the shoelace formula.
/// Positive for CCW, negative for CW.
pub fn polygon_signed_area(vertices: &[Point2D]) -> f64 {
    let n = vertices.len();
    if n < 3 {
        return 0.0;
    }
    let mut area = 0.0f64;
    for i in 0..n {
        let j = (i + 1) % n;
        area += vertices[i].x * vertices[j].y;
        area -= vertices[j].x * vertices[i].y;
    }
    area / 2.0
}
/// Compute the (positive) area of a simple polygon.
pub fn polygon_area(vertices: &[Point2D]) -> f64 {
    polygon_signed_area(vertices).abs()
}
/// Compute the centroid of a simple polygon.
pub fn polygon_centroid(vertices: &[Point2D]) -> Option<Point2D> {
    let n = vertices.len();
    if n < 3 {
        return None;
    }
    let area = polygon_signed_area(vertices);
    if area.abs() < 1e-14 {
        return None;
    }
    let mut cx = 0.0f64;
    let mut cy = 0.0f64;
    for i in 0..n {
        let j = (i + 1) % n;
        let cross = vertices[i].x * vertices[j].y - vertices[j].x * vertices[i].y;
        cx += (vertices[i].x + vertices[j].x) * cross;
        cy += (vertices[i].y + vertices[j].y) * cross;
    }
    let inv = 1.0 / (6.0 * area);
    Some(Point2D::new(cx * inv, cy * inv))
}
/// Compute the perimeter of a polygon.
pub fn polygon_perimeter(vertices: &[Point2D]) -> f64 {
    let n = vertices.len();
    if n < 2 {
        return 0.0;
    }
    (0..n)
        .map(|i| vertices[i].dist(&vertices[(i + 1) % n]))
        .sum()
}
/// Test if a point is strictly inside a simple polygon using ray casting.
pub fn point_in_polygon(p: &Point2D, polygon: &[Point2D]) -> bool {
    let n = polygon.len();
    if n < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let vi = &polygon[i];
        let vj = &polygon[j];
        let crosses_y = (vi.y > p.y) != (vj.y > p.y);
        if crosses_y {
            let x_intersect = (vj.x - vi.x) * (p.y - vi.y) / (vj.y - vi.y) + vi.x;
            if p.x < x_intersect {
                inside = !inside;
            }
        }
        j = i;
    }
    inside
}
/// Test if a polygon (given as CCW ordered vertices) is convex.
pub fn is_convex_polygon(vertices: &[Point2D]) -> bool {
    let n = vertices.len();
    if n < 3 {
        return true;
    }
    let mut sign = 0i32;
    for i in 0..n {
        let j = (i + 1) % n;
        let k = (i + 2) % n;
        let c = cross(&vertices[i], &vertices[j], &vertices[k]);
        if c > 1e-12 {
            if sign == -1 {
                return false;
            }
            sign = 1;
        } else if c < -1e-12 {
            if sign == 1 {
                return false;
            }
            sign = -1;
        }
    }
    true
}
/// Test if two line segments (p1,p2) and (p3,p4) intersect.
/// Uses orientation-based predicates.
pub fn segments_intersect(p1: &Point2D, p2: &Point2D, p3: &Point2D, p4: &Point2D) -> bool {
    let d1 = cross(p3, p4, p1);
    let d2 = cross(p3, p4, p2);
    let d3 = cross(p1, p2, p3);
    let d4 = cross(p1, p2, p4);
    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }
    if d1.abs() < 1e-12 && on_segment(p3, p4, p1) {
        return true;
    }
    if d2.abs() < 1e-12 && on_segment(p3, p4, p2) {
        return true;
    }
    if d3.abs() < 1e-12 && on_segment(p1, p2, p3) {
        return true;
    }
    if d4.abs() < 1e-12 && on_segment(p1, p2, p4) {
        return true;
    }
    false
}
/// Check if point `r` lies on segment (p, q) given collinearity.
pub fn on_segment(p: &Point2D, q: &Point2D, r: &Point2D) -> bool {
    r.x >= p.x.min(q.x) - 1e-12
        && r.x <= p.x.max(q.x) + 1e-12
        && r.y >= p.y.min(q.y) - 1e-12
        && r.y <= p.y.max(q.y) + 1e-12
}
/// Compute the intersection point of two segments (if they intersect).
/// Returns None if they are parallel or do not intersect.
pub fn segment_intersection(
    p1: &Point2D,
    p2: &Point2D,
    p3: &Point2D,
    p4: &Point2D,
) -> Option<Point2D> {
    let denom = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
    if denom.abs() < 1e-12 {
        return None;
    }
    let t_num = (p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x);
    let t = t_num / denom;
    Some(Point2D::new(
        p1.x + t * (p2.x - p1.x),
        p1.y + t * (p2.y - p1.y),
    ))
}
/// Test if point `p` is inside or on the circumcircle of triangle (pa, pb, pc).
/// Assumes the triangle is in CCW order.
pub fn in_circumcircle(pa: &Point2D, pb: &Point2D, pc: &Point2D, p: &Point2D) -> bool {
    let ax = pa.x - p.x;
    let ay = pa.y - p.y;
    let bx = pb.x - p.x;
    let by = pb.y - p.y;
    let cx = pc.x - p.x;
    let cy = pc.y - p.y;
    let det = ax * (by * (cx * cx + cy * cy) - cy * (bx * bx + by * by))
        - ay * (bx * (cx * cx + cy * cy) - cx * (bx * bx + by * by))
        + (ax * ax + ay * ay) * (bx * cy - by * cx);
    det > 1e-12
}
/// Compute the circumcenter of three points.
pub fn circumcenter(a: &Point2D, b: &Point2D, c: &Point2D) -> Option<Point2D> {
    let ax = b.x - a.x;
    let ay = b.y - a.y;
    let bx = c.x - a.x;
    let by = c.y - a.y;
    let d = 2.0 * (ax * by - ay * bx);
    if d.abs() < 1e-14 {
        return None;
    }
    let ux = (by * (ax * ax + ay * ay) - ay * (bx * bx + by * by)) / d;
    let uy = (ax * (bx * bx + by * by) - bx * (ax * ax + ay * ay)) / d;
    Some(Point2D::new(a.x + ux, a.y + uy))
}
/// Bowyer-Watson incremental Delaunay triangulation.
/// Returns a list of triangles (index triples) for the input point set.
pub fn delaunay_triangulation(points: &[Point2D]) -> Vec<Triangle> {
    let n = points.len();
    if n < 3 {
        return vec![];
    }
    let min_x = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    let dx = max_x - min_x;
    let dy = max_y - min_y;
    let delta_max = dx.max(dy);
    let mid_x = (min_x + max_x) / 2.0;
    let mid_y = (min_y + max_y) / 2.0;
    let mut pts: Vec<Point2D> = points.to_vec();
    let st0 = Point2D::new(mid_x - 20.0 * delta_max, mid_y - delta_max);
    let st1 = Point2D::new(mid_x, mid_y + 20.0 * delta_max);
    let st2 = Point2D::new(mid_x + 20.0 * delta_max, mid_y - delta_max);
    pts.push(st0);
    pts.push(st1);
    pts.push(st2);
    let si0 = n;
    let si1 = n + 1;
    let si2 = n + 2;
    let mut triangulation: Vec<Triangle> = vec![Triangle::new(si0, si1, si2)];
    for i in 0..n {
        let p = pts[i];
        let mut bad: Vec<Triangle> = Vec::new();
        let mut good: Vec<Triangle> = Vec::new();
        for &tri in &triangulation {
            let ta = pts[tri.a];
            let tb = pts[tri.b];
            let tc = pts[tri.c];
            if in_circumcircle(&ta, &tb, &tc, &p) {
                bad.push(tri);
            } else {
                good.push(tri);
            }
        }
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for tri in &bad {
            let tri_edges = [(tri.a, tri.b), (tri.b, tri.c), (tri.c, tri.a)];
            for &(u, v) in &tri_edges {
                let already = edges
                    .iter()
                    .any(|&(eu, ev)| (eu == u && ev == v) || (eu == v && ev == u));
                if already {
                    edges.retain(|&(eu, ev)| !((eu == u && ev == v) || (eu == v && ev == u)));
                } else {
                    edges.push((u, v));
                }
            }
        }
        triangulation = good;
        for (u, v) in edges {
            let pa = pts[u];
            let pb = pts[v];
            if cross(&pa, &pb, &p) > 0.0 {
                triangulation.push(Triangle::new(u, v, i));
            } else {
                triangulation.push(Triangle::new(v, u, i));
            }
        }
    }
    triangulation
        .retain(|t| !t.contains_vertex(si0) && !t.contains_vertex(si1) && !t.contains_vertex(si2));
    triangulation
}
/// Build a k-d tree from a slice of points.
pub fn build_kd_tree(points: &[Point2D]) -> KdNode {
    build_kd_tree_rec(&mut points.to_vec(), 0)
}
pub fn build_kd_tree_rec(points: &mut Vec<Point2D>, depth: usize) -> KdNode {
    if points.is_empty() {
        return KdNode::Leaf;
    }
    let axis = (depth % 2) as u8;
    if axis == 0 {
        points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal));
    }
    let median = points.len() / 2;
    let point = points[median];
    let mut left_pts: Vec<Point2D> = points[..median].to_vec();
    let mut right_pts: Vec<Point2D> = points[(median + 1)..].to_vec();
    KdNode::Node {
        point,
        left: Box::new(build_kd_tree_rec(&mut left_pts, depth + 1)),
        right: Box::new(build_kd_tree_rec(&mut right_pts, depth + 1)),
        axis,
    }
}
/// Find the nearest neighbour in a k-d tree to a query point.
pub fn kd_nearest(tree: &KdNode, query: &Point2D) -> Option<Point2D> {
    let mut best: Option<(f64, Point2D)> = None;
    kd_nearest_rec(tree, query, &mut best);
    best.map(|(_, p)| p)
}
pub fn kd_nearest_rec(node: &KdNode, query: &Point2D, best: &mut Option<(f64, Point2D)>) {
    match node {
        KdNode::Leaf => {}
        KdNode::Node {
            point,
            left,
            right,
            axis,
        } => {
            let d = query.dist(point);
            if best.map_or(true, |(bd, _)| d < bd) {
                *best = Some((d, *point));
            }
            let (near, far) =
                if (*axis == 0 && query.x < point.x) || (*axis == 1 && query.y < point.y) {
                    (left.as_ref(), right.as_ref())
                } else {
                    (right.as_ref(), left.as_ref())
                };
            kd_nearest_rec(near, query, best);
            let split_diff = if *axis == 0 {
                (query.x - point.x).abs()
            } else {
                (query.y - point.y).abs()
            };
            if best.map_or(true, |(bd, _)| split_diff < bd) {
                kd_nearest_rec(far, query, best);
            }
        }
    }
}
/// Range query: find all points within distance `r` of the query in the k-d tree.
pub fn kd_range_query(tree: &KdNode, query: &Point2D, r: f64) -> Vec<Point2D> {
    let mut results = Vec::new();
    kd_range_rec(tree, query, r, &mut results);
    results
}
pub fn kd_range_rec(node: &KdNode, query: &Point2D, r: f64, results: &mut Vec<Point2D>) {
    match node {
        KdNode::Leaf => {}
        KdNode::Node {
            point,
            left,
            right,
            axis,
        } => {
            if query.dist(point) <= r {
                results.push(*point);
            }
            let split_diff = if *axis == 0 {
                (query.x - point.x).abs()
            } else {
                (query.y - point.y).abs()
            };
            kd_range_rec(left, query, r, results);
            if split_diff <= r {
                kd_range_rec(right, query, r, results);
            }
        }
    }
}
/// Build a 1D range tree (BST by `key`) from a sorted slice.
pub fn build_range_tree_1d(pts: &[(f64, Point2D)]) -> RangeTree1D {
    if pts.is_empty() {
        return RangeTree1D::Empty;
    }
    let mid = pts.len() / 2;
    let (key, point) = pts[mid];
    RangeTree1D::Node {
        key,
        point,
        left: Box::new(build_range_tree_1d(&pts[..mid])),
        right: Box::new(build_range_tree_1d(&pts[(mid + 1)..])),
    }
}
/// Query a 1D range tree for all points with key in \[lo, hi\].
pub fn query_range_tree_1d(node: &RangeTree1D, lo: f64, hi: f64, result: &mut Vec<Point2D>) {
    match node {
        RangeTree1D::Empty => {}
        RangeTree1D::Node {
            key,
            point,
            left,
            right,
        } => {
            if *key >= lo && *key <= hi {
                result.push(*point);
            }
            if lo < *key {
                query_range_tree_1d(left, lo, hi, result);
            }
            if hi > *key {
                query_range_tree_1d(right, lo, hi, result);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn pts(coords: &[(f64, f64)]) -> Vec<Point2D> {
        coords.iter().map(|&(x, y)| Point2D::new(x, y)).collect()
    }
    #[test]
    fn test_convex_hull_square() {
        let points = pts(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.5, 0.5)]);
        let hull = graham_scan(&points);
        assert_eq!(
            hull.len(),
            4,
            "square hull should have 4 points, got {}",
            hull.len()
        );
    }
    #[test]
    fn test_convex_hull_collinear() {
        let points = pts(&[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (1.0, 1.0)]);
        let hull = graham_scan(&points);
        let has_middle = hull
            .iter()
            .any(|p| (p.x - 1.0).abs() < 1e-9 && p.y.abs() < 1e-9);
        assert!(!has_middle, "collinear middle point should not be on hull");
    }
    #[test]
    fn test_polygon_area_unit_square() {
        let square = pts(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        let area = polygon_area(&square);
        assert!(
            (area - 1.0).abs() < 1e-10,
            "unit square area = 1, got {area}"
        );
    }
    #[test]
    fn test_polygon_centroid_square() {
        let square = pts(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        let c = polygon_centroid(&square).expect("operation should succeed");
        assert!(
            (c.x - 0.5).abs() < 1e-10 && (c.y - 0.5).abs() < 1e-10,
            "centroid of unit square should be (0.5, 0.5), got ({}, {})",
            c.x,
            c.y
        );
    }
    #[test]
    fn test_point_in_polygon() {
        let square = pts(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        assert!(
            point_in_polygon(&Point2D::new(2.0, 2.0), &square),
            "center should be inside"
        );
        assert!(
            !point_in_polygon(&Point2D::new(5.0, 2.0), &square),
            "outside point should not be inside"
        );
    }
    #[test]
    fn test_segments_intersect_crossing() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(2.0, 2.0);
        let p3 = Point2D::new(0.0, 2.0);
        let p4 = Point2D::new(2.0, 0.0);
        assert!(
            segments_intersect(&p1, &p2, &p3, &p4),
            "crossing segments should intersect"
        );
    }
    #[test]
    fn test_segments_no_intersect_parallel() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(1.0, 0.0);
        let p3 = Point2D::new(0.0, 1.0);
        let p4 = Point2D::new(1.0, 1.0);
        assert!(
            !segments_intersect(&p1, &p2, &p3, &p4),
            "parallel segments should not intersect"
        );
    }
    #[test]
    fn test_closest_pair() {
        let points = pts(&[(0.0, 0.0), (5.0, 5.0), (1.0, 0.1), (10.0, 10.0)]);
        let (d, _a, _b) = closest_pair(&points).expect("operation should succeed");
        let expected = ((0.0 - 1.0f64).powi(2) + (0.0 - 0.1f64).powi(2)).sqrt();
        assert!(
            (d - expected).abs() < 1e-9,
            "closest pair distance: expected {expected}, got {d}"
        );
    }
    #[test]
    fn test_kd_tree_nearest() {
        let points = pts(&[(0.0, 0.0), (3.0, 4.0), (1.0, 1.0), (7.0, 2.0)]);
        let tree = build_kd_tree(&points);
        let query = Point2D::new(1.1, 1.1);
        let nearest = kd_nearest(&tree, &query).expect("Point2D::new should succeed");
        assert!(
            (nearest.x - 1.0).abs() < 1e-9 && (nearest.y - 1.0).abs() < 1e-9,
            "nearest to (1.1,1.1) should be (1.0,1.0), got ({},{})",
            nearest.x,
            nearest.y
        );
    }
    #[test]
    fn test_bentley_ottmann_events_no_intersection() {
        let segs = vec![
            (Point2D::new(0.0, 0.0), Point2D::new(2.0, 0.0)),
            (Point2D::new(0.0, 1.0), Point2D::new(2.0, 1.0)),
        ];
        let eq = BentleyOttmannEvents::new(&segs);
        let intersections = eq.all_intersections();
        assert!(
            intersections.is_empty(),
            "parallel segments should not intersect"
        );
    }
    #[test]
    fn test_bentley_ottmann_events_crossing() {
        let segs = vec![
            (Point2D::new(0.0, 0.0), Point2D::new(2.0, 2.0)),
            (Point2D::new(0.0, 2.0), Point2D::new(2.0, 0.0)),
        ];
        let eq = BentleyOttmannEvents::new(&segs);
        let intersections = eq.all_intersections();
        assert_eq!(intersections.len(), 1, "exactly one intersection expected");
        let p = &intersections[0];
        assert!(
            (p.x - 1.0).abs() < 1e-9 && (p.y - 1.0).abs() < 1e-9,
            "intersection should be (1,1), got ({},{})",
            p.x,
            p.y
        );
    }
    #[test]
    fn test_bentley_ottmann_event_queue_ordering() {
        let segs = vec![
            (Point2D::new(3.0, 0.0), Point2D::new(5.0, 2.0)),
            (Point2D::new(0.0, 1.0), Point2D::new(2.0, 1.0)),
        ];
        let eq = BentleyOttmannEvents::new(&segs);
        assert_eq!(eq.len(), 4, "4 endpoint events expected");
    }
    #[test]
    fn test_alpha_shape_builder_trivial() {
        let pts_data = pts(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0), (2.0, 2.0)]);
        let builder = AlphaShapeBuilder::new(&pts_data);
        let alphas = builder.critical_alphas();
        let max_finite = alphas
            .iter()
            .filter(|a| a.is_finite())
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        if max_finite > 0.0 {
            let step = builder.at_alpha(max_finite);
            assert!(
                !step.triangles.is_empty(),
                "should have at least one triangle at max finite alpha={max_finite}"
            );
        }
        let step_inf = builder.at_alpha(f64::INFINITY);
        let _ = step_inf;
    }
    #[test]
    fn test_alpha_shape_builder_filtration_monotone() {
        let pts_data = pts(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.5, 0.3)]);
        let builder = AlphaShapeBuilder::new(&pts_data);
        let filt = builder.filtration();
        let mut prev_count = 0usize;
        for step in &filt {
            assert!(
                step.triangles.len() >= prev_count,
                "filtration must be monotone: at alpha={} have {} triangles, prev {}",
                step.alpha,
                step.triangles.len(),
                prev_count
            );
            prev_count = step.triangles.len();
        }
    }
    #[test]
    fn test_range_tree_2d_basic() {
        let points = pts(&[(1.0, 1.0), (2.0, 3.0), (3.0, 2.0), (5.0, 5.0), (4.0, 1.0)]);
        let tree = RangeTree2D::new(&points);
        let mut result = tree.query(1.5, 3.5, 1.5, 3.5);
        result.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
        assert_eq!(
            result.len(),
            2,
            "should find 2 points in [1.5,3.5]x[1.5,3.5], got {:?}",
            result
        );
    }
    #[test]
    fn test_range_tree_2d_via_tree() {
        let points = pts(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0), (2.0, 1.0)]);
        let tree = RangeTree2D::new(&points);
        let result = tree.query_via_tree(1.0, 3.0, 1.0, 4.0);
        assert!(!result.is_empty(), "should find points in range");
    }
    #[test]
    fn test_range_tree_2d_empty_result() {
        let points = pts(&[(10.0, 10.0), (20.0, 20.0)]);
        let tree = RangeTree2D::new(&points);
        let result = tree.query(0.0, 5.0, 0.0, 5.0);
        assert!(result.is_empty(), "no points in small range");
    }
    #[test]
    fn test_frechet_distance_identical_curves() {
        let curve = pts(&[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)]);
        let d = FrechetDistanceApprox::compute(&curve, &curve);
        assert!(
            d < 1e-9,
            "Fréchet distance between identical curves should be 0, got {d}"
        );
    }
    #[test]
    fn test_frechet_distance_parallel_curves() {
        let p = pts(&[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)]);
        let q = pts(&[(0.0, 1.0), (1.0, 1.0), (2.0, 1.0)]);
        let d = FrechetDistanceApprox::compute(&p, &q);
        assert!(
            (d - 1.0).abs() < 1e-9,
            "parallel curves at distance 1.0 should have Fréchet distance 1.0, got {d}"
        );
    }
    #[test]
    fn test_frechet_decision_true() {
        let p = pts(&[(0.0, 0.0), (1.0, 0.0)]);
        let q = pts(&[(0.0, 0.5), (1.0, 0.5)]);
        assert!(
            FrechetDistanceApprox::decide(&p, &q, 1.0),
            "Fréchet distance should be <= 1.0"
        );
    }
    #[test]
    fn test_frechet_decision_false() {
        let p = pts(&[(0.0, 0.0), (1.0, 0.0)]);
        let q = pts(&[(0.0, 2.0), (1.0, 2.0)]);
        assert!(
            !FrechetDistanceApprox::decide(&p, &q, 1.5),
            "Fréchet distance 2.0 should not be <= 1.5"
        );
    }
    #[test]
    fn test_convex_layer_peeler_basic() {
        let points = pts(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0), (2.0, 2.0)]);
        let mut peeler = ConvexLayerPeeler::new(&points);
        peeler.peel_all();
        assert!(
            peeler.num_layers() >= 2,
            "should have at least 2 layers, got {}",
            peeler.num_layers()
        );
    }
    #[test]
    fn test_convex_layer_peeler_all_hull() {
        let points = pts(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
        let mut peeler = ConvexLayerPeeler::new(&points);
        peeler.peel_all();
        assert_eq!(peeler.num_layers(), 1, "all hull points = 1 layer");
    }
    #[test]
    fn test_convex_layer_peeler_depth() {
        let points = pts(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0), (2.0, 2.0)]);
        let mut peeler = ConvexLayerPeeler::new(&points);
        peeler.peel_all();
        let inner = Point2D::new(2.0, 2.0);
        let d = peeler.depth_of(&inner);
        assert!(
            d.map_or(false, |depth| depth >= 1),
            "interior point should have depth >= 1"
        );
    }
    #[test]
    fn test_build_computational_geometry_env_extended() {
        use oxilean_kernel::Environment;
        let mut env = Environment::new();
        build_computational_geometry_env(&mut env);
        for name in [
            "AlphaComplex",
            "CechComplex",
            "KineticTournament",
            "BentleyOttmannOutput",
            "VCDimension",
            "EpsilonNet",
            "RangeTree",
            "FractionalCascading",
            "HausdorffDistance",
            "FrechetDistance",
            "ConfigurationSpace",
            "ConvexLayers",
            "KCenterClustering",
            "DelaunayRefinement",
        ] {
            assert!(
                env.get(&oxilean_kernel::Name::str(name)).is_some(),
                "axiom '{}' should be registered",
                name
            );
        }
    }
}
#[allow(dead_code)]
pub fn cross_2d(o: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}
#[cfg(test)]
mod tests_cg_extra {
    use super::*;
    #[test]
    fn test_voronoi_nearest_site() {
        let v = VoronoiDiagram::new(vec![(0.0, 0.0), (5.0, 0.0), (2.5, 5.0)]);
        let nearest = v
            .nearest_site((1.0, 0.0))
            .expect("VoronoiDiagram::new should succeed");
        assert_eq!(nearest, 0, "Nearest to (1,0) should be site 0 at origin");
    }
    #[test]
    fn test_convex_hull() {
        let points = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.5, 0.5)];
        let hull = ConvexHull2D::compute(points);
        assert_eq!(hull.n_hull_points(), 4, "Square should have 4 hull points");
        let area = hull.area();
        assert!((area - 1.0).abs() < 1e-9, "Area should be 1.0, got {area}");
    }
    #[test]
    fn test_minkowski_sum() {
        let square = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let ms = MinkowskiSum::new(square.clone(), square.clone());
        assert_eq!(ms.result_size_upper_bound(), 8);
        assert!(ms.is_convex_if_inputs_convex());
    }
    #[test]
    fn test_plane_subdivision_euler() {
        let mut sub = PlaneSubdivision::new(vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)]);
        sub.add_edge(0, 1);
        sub.add_edge(1, 2);
        sub.add_edge(2, 0);
        assert_eq!(sub.n_faces(), 2);
    }
}
