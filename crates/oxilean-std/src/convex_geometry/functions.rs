//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    CircumscribedSphere, ConvexFunction, ConvexSet, ConvexValuation, CrossPolytope,
    DelaunayTriangulation, FacePoset, HPolytope, HadwigerNumber, HellyTheoremChecker,
    IntrinsicVolume, JohnEllipsoidApprox, LatticePolytope, LogConcaveSequence,
    LorentzianPolynomial, MinkowskiSumComputer, MixedVolume, MixedVolumeEstimator, PowerDiagram,
    TilingTheory, VPolytope, VoronoiDiagram, Zonotope, ZonotopeData,
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
pub fn fn_ty(dom: Expr, cod: Expr) -> Expr {
    arrow(dom, cod)
}
pub fn vec_ty() -> Expr {
    list_ty(real_ty())
}
pub fn mat_ty() -> Expr {
    list_ty(list_ty(real_ty()))
}
/// `ConvexSet : (List Real -> Prop) -> Prop`
/// A closed convex subset C ⊆ ℝ^n, represented as a characteristic predicate.
pub fn convex_set_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), prop())
}
/// `ConvexFunction : (List Real -> Real) -> Prop`
/// f: ℝ^n → ℝ satisfying f(λx+(1-λ)y) ≤ λf(x)+(1-λ)f(y) for all λ ∈ [0,1].
pub fn convex_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `SupportFunction : (List Real -> Prop) -> List Real -> Real`
/// h_C(y) = sup_{x∈C} ⟨x,y⟩, the support function of a convex set C.
pub fn support_function_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), fn_ty(vec_ty(), real_ty()))
}
/// `MinkowskiSum : (List Real -> Prop) -> (List Real -> Prop) -> (List Real -> Prop)`
/// A + B = {a+b : a∈A, b∈B}, the Minkowski sum of two convex sets.
pub fn minkowski_sum_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred.clone(), arrow(set_pred.clone(), set_pred))
}
/// `Projection : Prop`
/// Every point x has a unique nearest point in a closed convex set (projection theorem).
pub fn projection_ty() -> Expr {
    prop()
}
/// `NormalCone : (List Real -> Prop) -> List Real -> (List Real -> Prop)`
/// N_C(x) = {y : ⟨y, z-x⟩ ≤ 0 for all z ∈ C}, the normal cone at x.
pub fn normal_cone_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred.clone(), arrow(vec_ty(), set_pred))
}
/// `Subdifferential : (List Real -> Real) -> List Real -> (List Real -> Prop)`
/// ∂f(x) = {g : f(y) ≥ f(x) + ⟨g, y-x⟩ for all y}, the subdifferential of f at x.
pub fn subdifferential_ty() -> Expr {
    let f_ty = fn_ty(vec_ty(), real_ty());
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(f_ty, arrow(vec_ty(), set_pred))
}
/// `IsPolyhedral : (List Real -> Prop) -> Prop`
/// Predicate asserting that a convex set is polyhedral (finitely many halfspaces).
pub fn is_polyhedral_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), prop())
}
/// `VPolytope : List (List Real) -> (List Real -> Prop)`
/// V-representation: conv{v_1,...,v_n}. Maps a list of vertices to the polytope predicate.
pub fn v_polytope_ty() -> Expr {
    arrow(mat_ty(), fn_ty(vec_ty(), prop()))
}
/// `HPolytope : List (List Real) -> List Real -> (List Real -> Prop)`
/// H-representation: {x : Ax ≤ b}. Maps (A, b) to the polytope predicate.
pub fn h_polytope_ty() -> Expr {
    arrow(mat_ty(), arrow(vec_ty(), fn_ty(vec_ty(), prop())))
}
/// `DoubleDescription : Prop`
/// Fourier-Motzkin / vertex enumeration: V-rep and H-rep are equivalent for polytopes.
pub fn double_description_ty() -> Expr {
    prop()
}
/// `FacePoset : (List Real -> Prop) -> Prop`
/// The lattice of faces of a polytope ordered by inclusion.
pub fn face_poset_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), prop())
}
/// `FVector : (List Real -> Prop) -> List Nat`
/// f-vector (f_0, f_1, ..., f_{d-1}) counting faces of each dimension.
pub fn f_vector_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), list_ty(nat_ty()))
}
/// `IsSimplicial : Prop`
/// Predicate: every proper face is a simplex.
pub fn is_simplicial_ty() -> Expr {
    prop()
}
/// `IsSimple : Prop`
/// Predicate: every vertex is contained in exactly d facets (dual of simplicial).
pub fn is_simple_ty() -> Expr {
    prop()
}
/// `VoronoiDiagram : List (List Real) -> Nat -> (List Real -> Prop)`
/// Partition into Voronoi cells V(p_i) for a given set of sites.
pub fn voronoi_diagram_ty() -> Expr {
    arrow(mat_ty(), arrow(nat_ty(), fn_ty(vec_ty(), prop())))
}
/// `DelaunayTriangulation : Prop`
/// Dual to the Voronoi diagram; maximises the minimum angle among triangulations.
pub fn delaunay_triangulation_ty() -> Expr {
    prop()
}
/// `PowerDiagram : Prop`
/// Weighted Voronoi diagram (Laguerre tessellation) with power distances.
pub fn power_diagram_ty() -> Expr {
    prop()
}
/// `CircumscribedSphere : Prop`
/// Smallest enclosing ball (minimax problem) for a finite point set.
pub fn circumscribed_sphere_ty() -> Expr {
    prop()
}
/// `NearestNeighbor : List (List Real) -> List Real -> Nat`
/// Index of the nearest site to a query point.
pub fn nearest_neighbor_ty() -> Expr {
    arrow(mat_ty(), fn_ty(vec_ty(), nat_ty()))
}
/// `DelaunayProperty : Prop`
/// The circumsphere of each Delaunay simplex contains no other site in its interior.
pub fn delaunay_property_ty() -> Expr {
    prop()
}
/// `LloydIteration : Prop`
/// Lloyd's algorithm iterates Voronoi + centroid to compute optimal quantisations.
pub fn lloyd_iteration_ty() -> Expr {
    prop()
}
/// `MixedVolume : List (List Real -> Prop) -> Real`
/// V(K_1,...,K_n) from polarisation of the volume polynomial.
pub fn mixed_volume_ty() -> Expr {
    arrow(list_ty(fn_ty(vec_ty(), prop())), real_ty())
}
/// `BrunnMinkowskiIneq : Prop`
/// V(A+B)^{1/n} ≥ V(A)^{1/n} + V(B)^{1/n} for convex bodies A, B ⊆ ℝ^n.
pub fn brunn_minkowski_ineq_ty() -> Expr {
    prop()
}
/// `AlexandrovFenchelIneq : Prop`
/// Mixed volume inequalities: V(K_1,...,K_n)² ≥ V(K_1,K_1,K_3,...) · V(K_2,K_2,K_3,...).
pub fn alexandrov_fenchel_ineq_ty() -> Expr {
    prop()
}
/// `IsoperimetricInequality : Prop`
/// V^{n-1} ≤ c_n · S^n: volume vs. surface-area isoperimetric inequality.
pub fn isoperimetric_inequality_ty() -> Expr {
    prop()
}
/// `HadwigerNumber : (List Real -> Prop) -> Nat`
/// Minimum number of translates of the interior of K needed to cover K.
pub fn hadwiger_number_ty() -> Expr {
    arrow(fn_ty(vec_ty(), prop()), nat_ty())
}
/// `CrossPolytope : Nat -> (List Real -> Prop)`
/// Standard cross polytope in ℝ^n: {x : Σ|x_i| ≤ 1}, dual of the hypercube.
pub fn cross_polytope_ty() -> Expr {
    arrow(nat_ty(), fn_ty(vec_ty(), prop()))
}
/// `Zonotope : List (List Real) -> (List Real -> Prop)`
/// Minkowski sum of line segments {c + Σ λ_i g_i : λ_i ∈ [-1,1]}.
pub fn zonotope_ty() -> Expr {
    arrow(mat_ty(), fn_ty(vec_ty(), prop()))
}
/// `TilingTheory : Prop`
/// Lattice tilings, Keller's conjecture: every lattice tiling by translates shares a facet.
pub fn tiling_theory_ty() -> Expr {
    prop()
}
/// `MinkowskiLatticeTheorem : Prop`
/// Minkowski's theorem: a symmetric convex body of volume > 2^n contains a nonzero lattice point.
pub fn minkowski_lattice_theorem_ty() -> Expr {
    prop()
}
/// `SuccessiveMinima : (List Real -> Prop) -> List Real -> List Real`
/// λ_i(K, Λ) = inf{r : r·K contains i linearly independent lattice points}.
pub fn successive_minima_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, arrow(vec_ty(), vec_ty()))
}
/// `MinkowskiSecondTheorem : Prop`
/// Minkowski's second theorem: product of successive minima λ_1···λ_n ≤ 2^n det(Λ) / Vol(K).
pub fn minkowski_second_theorem_ty() -> Expr {
    prop()
}
/// `PackingDensity : (List Real -> Prop) -> Real`
/// δ(K) = supremum of densities of lattice packings by translates of K.
pub fn packing_density_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, real_ty())
}
/// `CoveringDensity : (List Real -> Prop) -> Real`
/// θ(K) = infimum of densities of lattice coverings by K.
pub fn covering_density_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, real_ty())
}
/// `Quermassintegral : (List Real -> Prop) -> Nat -> Real`
/// W_j(K): the j-th quermassintegral (intrinsic volume up to scaling).
pub fn quermassintegral_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, arrow(nat_ty(), real_ty()))
}
/// `IntrinsicVolume : (List Real -> Prop) -> Nat -> Real`
/// V_j(K): j-th intrinsic volume. V_0 = 1, V_n = Vol.
pub fn intrinsic_volume_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, arrow(nat_ty(), real_ty()))
}
/// `MeanWidth : (List Real -> Prop) -> Real`
/// w(K) = ∫_{S^{n-1}} h_K(u) dσ(u) / κ_{n-1}, the mean width of K.
pub fn mean_width_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, real_ty())
}
/// `SteinerFormula : Prop`
/// Steiner formula: Vol(K + t B^n) = Σ_j C(n,j) W_j(K) t^j.
pub fn steiner_formula_ty() -> Expr {
    prop()
}
/// `LogConcaveMeasure : (List Real -> Real) -> Prop`
/// A measure μ is log-concave if μ(λA+(1-λ)B) ≥ μ(A)^λ μ(B)^{1-λ}.
pub fn log_concave_measure_ty() -> Expr {
    arrow(fn_ty(vec_ty(), real_ty()), prop())
}
/// `PrekopaLeindler : Prop`
/// Prékopa-Leindler inequality: the 1D version of Brunn-Minkowski for functions.
pub fn prekopa_leindler_ty() -> Expr {
    prop()
}
/// `BallsConcentration : Prop`
/// Concentration of measure on spheres: Lipschitz functions concentrate around their median.
pub fn balls_concentration_ty() -> Expr {
    prop()
}
/// `Valuation : (List Real -> Prop) -> Real`
/// A valuation μ: K^n → ℝ satisfies μ(K∪L) + μ(K∩L) = μ(K) + μ(L).
pub fn valuation_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, real_ty())
}
/// `HadwigerThm : Prop`
/// Hadwiger's theorem: every continuous valuation is a linear combination of intrinsic volumes.
pub fn hadwiger_thm_ty() -> Expr {
    prop()
}
/// `KinematicFormula : Prop`
/// Kinematic formula: integral over rigid motions of the intersection measure.
pub fn kinematic_formula_ty() -> Expr {
    prop()
}
/// `BanachMazurDistance : (List Real -> Prop) -> (List Real -> Prop) -> Real`
/// d_BM(K, L) = inf{λ : ∃ linear T, K ⊆ T(L) ⊆ λK}.
pub fn banach_mazur_distance_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred.clone(), arrow(set_pred, real_ty()))
}
/// `BanachMazurCompactness : Prop`
/// The Banach-Mazur compactum: the space of all n-dimensional normed spaces under d_BM.
pub fn banach_mazur_compactness_ty() -> Expr {
    prop()
}
/// `JohnEllipsoid : (List Real -> Prop) -> List (List Real) -> Real`
/// John's theorem: every convex body K contains a maximal volume ellipsoid E with E ⊆ K ⊆ n·E.
pub fn john_ellipsoid_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, arrow(mat_ty(), real_ty()))
}
/// `MinimalEnclosingEllipsoid : (List Real -> Prop) -> Prop`
/// The Löwner-John ellipsoid (minimum volume enclosing ellipsoid).
pub fn minimal_enclosing_ellipsoid_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, prop())
}
/// `NormalFan : (List Real -> Prop) -> Prop`
/// The normal fan of a polytope: a complete polyhedral fan dual to the face structure.
pub fn normal_fan_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, prop())
}
/// `GaussMap : (List Real -> Prop) -> (List Real -> List Real)`
/// Gauss map: maps each boundary point to its outer unit normal.
pub fn gauss_map_ty() -> Expr {
    let set_pred = fn_ty(vec_ty(), prop());
    arrow(set_pred, fn_ty(vec_ty(), vec_ty()))
}
/// `HellyThm : Nat -> Prop`
/// Helly's theorem: if every d+1 members of a finite family of convex sets intersect, all do.
pub fn helly_thm_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `FractionalHelly : Nat -> Real -> Prop`
/// Fractional Helly (Katchalski-Liu): if αC(n,d+1) subfamilies intersect, a large fraction do.
pub fn fractional_helly_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `ColorfulHelly : Nat -> Prop`
/// Colorful Helly (Barany-Lovász): a colorful Helly theorem for families colored by n+1 colors.
pub fn colorful_helly_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `RadonPartition : List (List Real) -> Prop`
/// Radon's theorem: every d+2 points can be split into two disjoint sets whose convex hulls intersect.
pub fn radon_partition_ty() -> Expr {
    arrow(mat_ty(), prop())
}
/// `HamSandwichTheorem : Nat -> Prop`
/// Ham-sandwich theorem: any n measurable sets in ℝ^n can be simultaneously bisected by a hyperplane.
pub fn ham_sandwich_thm_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CenterpointThm : Nat -> Prop`
/// Centerpoint theorem: every finite point set in ℝ^d has a point piercing many simplices.
pub fn centerpoint_thm_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CaratheodoryThm : Nat -> Prop`
/// Carathéodory's theorem: every point in conv(S)⊆ℝ^d lies in conv of at most d+1 points of S.
pub fn caratheodory_thm_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `BorsukPartition : Nat -> Prop`
/// Borsuk's problem: can every bounded set in ℝ^n be divided into n+1 parts of smaller diameter?
pub fn borsuk_partition_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ChromaticNumberEuclidean : Nat -> Nat`
/// χ(ℝ^n, 1): chromatic number of the unit-distance graph in ℝ^n.
pub fn chromatic_euclidean_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// Populate an [`Environment`] with all convex-geometry axioms.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("ConvexSet", convex_set_ty()),
        ("ConvexFunction", convex_function_ty()),
        ("SupportFunction", support_function_ty()),
        ("MinkowskiSum", minkowski_sum_ty()),
        ("Projection", projection_ty()),
        ("NormalCone", normal_cone_ty()),
        ("Subdifferential", subdifferential_ty()),
        ("IsPolyhedral", is_polyhedral_ty()),
        ("VPolytope", v_polytope_ty()),
        ("HPolytope", h_polytope_ty()),
        ("DoubleDescription", double_description_ty()),
        ("FacePoset", face_poset_ty()),
        ("FVector", f_vector_ty()),
        ("IsSimplicial", is_simplicial_ty()),
        ("IsSimple", is_simple_ty()),
        ("VoronoiDiagram", voronoi_diagram_ty()),
        ("DelaunayTriangulation", delaunay_triangulation_ty()),
        ("PowerDiagram", power_diagram_ty()),
        ("CircumscribedSphere", circumscribed_sphere_ty()),
        ("NearestNeighbor", nearest_neighbor_ty()),
        ("DelaunayProperty", delaunay_property_ty()),
        ("LloydIteration", lloyd_iteration_ty()),
        ("MixedVolume", mixed_volume_ty()),
        ("BrunnMinkowskiIneq", brunn_minkowski_ineq_ty()),
        ("AlexandrovFenchelIneq", alexandrov_fenchel_ineq_ty()),
        ("IsoperimetricInequality", isoperimetric_inequality_ty()),
        ("HadwigerNumber", hadwiger_number_ty()),
        ("CrossPolytope", cross_polytope_ty()),
        ("Zonotope", zonotope_ty()),
        ("TilingTheory", tiling_theory_ty()),
        ("MinkowskiLatticeTheorem", minkowski_lattice_theorem_ty()),
        ("SuccessiveMinima", successive_minima_ty()),
        ("MinkowskiSecondTheorem", minkowski_second_theorem_ty()),
        ("PackingDensity", packing_density_ty()),
        ("CoveringDensity", covering_density_ty()),
        ("Quermassintegral", quermassintegral_ty()),
        ("IntrinsicVolume", intrinsic_volume_ty()),
        ("MeanWidth", mean_width_ty()),
        ("SteinerFormula", steiner_formula_ty()),
        ("LogConcaveMeasure", log_concave_measure_ty()),
        ("PrekopaLeindler", prekopa_leindler_ty()),
        ("BallsConcentration", balls_concentration_ty()),
        ("Valuation", valuation_ty()),
        ("HadwigerThm", hadwiger_thm_ty()),
        ("KinematicFormula", kinematic_formula_ty()),
        ("BanachMazurDistance", banach_mazur_distance_ty()),
        ("BanachMazurCompactness", banach_mazur_compactness_ty()),
        ("JohnEllipsoid", john_ellipsoid_ty()),
        (
            "MinimalEnclosingEllipsoid",
            minimal_enclosing_ellipsoid_ty(),
        ),
        ("NormalFan", normal_fan_ty()),
        ("GaussMap", gauss_map_ty()),
        ("HellyThm", helly_thm_ty()),
        ("FractionalHelly", fractional_helly_ty()),
        ("ColorfulHelly", colorful_helly_ty()),
        ("RadonPartition", radon_partition_ty()),
        ("HamSandwichTheorem", ham_sandwich_thm_ty()),
        ("CenterpointThm", centerpoint_thm_ty()),
        ("CaratheodoryThm", caratheodory_thm_ty()),
        ("BorsukPartition", borsuk_partition_ty()),
        ("ChromaticNumberEuclidean", chromatic_euclidean_ty()),
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
/// Simple point-in-convex-hull test via linear inequalities (only for simplices).
pub fn is_in_convex_hull(vertices: &[Vec<f64>], point: &[f64]) -> bool {
    if vertices.is_empty() {
        return false;
    }
    let d = point.len();
    vertices.iter().any(|v| {
        v.iter()
            .zip(point.iter())
            .all(|(vi, pi)| (vi - pi).abs() <= 0.5)
    }) || {
        let within: bool = (0..d).all(|j| {
            let lo = vertices.iter().map(|v| v[j]).fold(f64::INFINITY, f64::min);
            let hi = vertices
                .iter()
                .map(|v| v[j])
                .fold(f64::NEG_INFINITY, f64::max);
            point[j] >= lo - 1e-10 && point[j] <= hi + 1e-10
        });
        within
    }
}
pub fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k);
    let mut result = 1_usize;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}
/// Stirling approximation for Gamma: Γ(x+1) ≈ √(2πx)(x/e)^x.
pub fn stirling_gamma(x: f64) -> f64 {
    if x <= 0.0 {
        return 1.0;
    }
    let x_eff = x.max(1.0);
    (2.0 * std::f64::consts::PI * x_eff).sqrt() * (x_eff / std::f64::consts::E).powf(x_eff)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("ConvexSet")).is_some());
        assert!(env.get(&Name::str("VPolytope")).is_some());
        assert!(env.get(&Name::str("VoronoiDiagram")).is_some());
        assert!(env.get(&Name::str("BrunnMinkowskiIneq")).is_some());
        assert!(env.get(&Name::str("CrossPolytope")).is_some());
    }
    #[test]
    fn test_convex_set_support_function() {
        let cs = ConvexSet::new(
            2,
            vec![
                vec![0.0, 0.0],
                vec![1.0, 0.0],
                vec![1.0, 1.0],
                vec![0.0, 1.0],
            ],
        );
        let h = cs.support_function(&[1.0, 0.0]);
        assert!((h - 1.0).abs() < 1e-12, "h_C(e1) = 1, got {h}");
        let h2 = cs.support_function(&[1.0, 1.0]);
        assert!((h2 - 2.0).abs() < 1e-12, "h_C(1,1) = 2, got {h2}");
    }
    #[test]
    fn test_minkowski_sum() {
        let a = ConvexSet::new(2, vec![vec![0.0, 0.0], vec![1.0, 0.0]]);
        let b = ConvexSet::new(2, vec![vec![0.0, 0.0], vec![0.0, 1.0]]);
        let c = a.minkowski_sum(&b);
        assert_eq!(c.vertices.len(), 4);
    }
    #[test]
    fn test_h_polytope_contains() {
        let a = vec![
            vec![-1.0, 0.0],
            vec![1.0, 0.0],
            vec![0.0, -1.0],
            vec![0.0, 1.0],
        ];
        let b = vec![0.0, 1.0, 0.0, 1.0];
        let poly = HPolytope::new(a, b);
        assert!(poly.contains(&[0.5, 0.5]));
        assert!(!poly.contains(&[1.5, 0.5]));
    }
    #[test]
    fn test_face_poset_euler() {
        let fp = FacePoset::new(vec![4, 6, 4]);
        assert_eq!(fp.euler_characteristic(), 2);
    }
    #[test]
    fn test_voronoi_nearest_neighbor() {
        let sites = vec![vec![0.0, 0.0], vec![3.0, 0.0], vec![0.0, 3.0]];
        let vd = VoronoiDiagram::new(sites);
        assert_eq!(vd.nearest_neighbor(&[0.5, 0.5]), 0);
        assert_eq!(vd.nearest_neighbor(&[2.5, 0.5]), 1);
    }
    #[test]
    fn test_delaunay_property() {
        let sites = vec![
            vec![0.0, 0.0],
            vec![1.0, 0.0],
            vec![0.5, 1.0],
            vec![2.0, 1.0],
        ];
        let triangles = vec![[0, 1, 2], [1, 2, 3]];
        let dt = DelaunayTriangulation::new(sites, triangles);
        let _ = dt.check_delaunay_property();
    }
    #[test]
    fn test_cross_polytope() {
        let cp = CrossPolytope::new(3);
        assert_eq!(cp.vertices().len(), 6);
        assert!(cp.contains(&[0.0, 0.0, 0.9]));
        assert!(!cp.contains(&[1.0, 0.5, 0.0]));
        let vol = cp.volume();
        assert!(
            (vol - 8.0 / 6.0).abs() < 1e-12,
            "vol(B_1^3) = 4/3, got {vol}"
        );
    }
    #[test]
    fn test_cross_polytope_f_vector() {
        let cp = CrossPolytope::new(2);
        let fv = cp.f_vector();
        assert_eq!(fv[0], 4, "f_0=4 vertices for n=2 cross polytope");
        assert_eq!(fv[1], 4, "f_1=4 edges for n=2 cross polytope");
    }
    #[test]
    fn test_zonotope_contains() {
        let z = Zonotope::new(vec![0.0, 0.0], vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
        assert!(z.contains_approx(&[0.5, 0.5]));
    }
    #[test]
    fn test_zonotope_volume_2d() {
        let z = Zonotope::new(vec![0.0, 0.0], vec![vec![1.0, 0.0], vec![0.0, 1.0]]);
        let v = z.volume_2d();
        assert!((v - 4.0).abs() < 1e-12, "volume = 4, got {v}");
    }
    #[test]
    fn test_circumscribed_sphere() {
        let points = vec![
            vec![1.0, 0.0],
            vec![-1.0, 0.0],
            vec![0.0, 1.0],
            vec![0.0, -1.0],
        ];
        let sphere = CircumscribedSphere::compute(&points)
            .expect("CircumscribedSphere::compute should succeed");
        assert!(
            (sphere.radius - 1.0).abs() < 1e-12,
            "radius = 1, got {}",
            sphere.radius
        );
        for p in &points {
            assert!(sphere.contains(p), "all points should be inside");
        }
    }
    #[test]
    fn test_brunn_minkowski() {
        let mv = MixedVolume::new(2);
        assert!(mv.check_brunn_minkowski(1.0, 1.0, 4.0));
    }
    #[test]
    fn test_isoperimetric_2d() {
        let area = std::f64::consts::PI;
        let perimeter = 2.0 * std::f64::consts::PI;
        assert!(MixedVolume::check_isoperimetric_2d(area, perimeter));
        assert!(MixedVolume::check_isoperimetric_2d(1.0, 4.0));
    }
    #[test]
    fn test_hadwiger_upper_bound() {
        let h2 = HadwigerNumber::new(2);
        assert_eq!(h2.upper_bound(), 2);
        let h3 = HadwigerNumber::new(3);
        assert_eq!(h3.upper_bound(), 6);
    }
    #[test]
    fn test_tiling_theory() {
        let lattice = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let tt = TilingTheory::new(2, lattice);
        assert!(tt.is_valid_lattice());
        assert!((tt.fundamental_volume_2d() - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_power_diagram() {
        let sites = vec![(vec![0.0, 0.0], 0.0), (vec![2.0, 0.0], 1.0)];
        let pd = PowerDiagram::new(sites);
        assert_eq!(pd.nearest_power_neighbor(&[1.0, 0.0]), 1);
    }
    #[test]
    fn test_lloyd_iteration() {
        let sites = vec![vec![0.0], vec![10.0]];
        let vd = VoronoiDiagram::new(sites);
        let samples: Vec<Vec<f64>> = (0..20).map(|i| vec![i as f64]).collect();
        let new_sites = vd.lloyd_iteration(&samples);
        assert_eq!(new_sites.len(), 2);
        assert!(
            new_sites[0][0] < new_sites[1][0],
            "centroids should be ordered"
        );
    }
    #[test]
    fn test_v_polytope_f_vector() {
        let verts = vec![
            vec![1.0, 0.0, 0.0],
            vec![-1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let vp = VPolytope::new(verts);
        let fv = vp.f_vector();
        assert_eq!(fv[0], 4, "4 vertices");
    }
    #[test]
    fn test_build_env_new_axioms() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("MinkowskiLatticeTheorem")).is_some());
        assert!(env.get(&Name::str("SuccessiveMinima")).is_some());
        assert!(env.get(&Name::str("MinkowskiSecondTheorem")).is_some());
        assert!(env.get(&Name::str("PackingDensity")).is_some());
        assert!(env.get(&Name::str("CoveringDensity")).is_some());
        assert!(env.get(&Name::str("Quermassintegral")).is_some());
        assert!(env.get(&Name::str("IntrinsicVolume")).is_some());
        assert!(env.get(&Name::str("MeanWidth")).is_some());
        assert!(env.get(&Name::str("SteinerFormula")).is_some());
        assert!(env.get(&Name::str("LogConcaveMeasure")).is_some());
        assert!(env.get(&Name::str("PrekopaLeindler")).is_some());
        assert!(env.get(&Name::str("BallsConcentration")).is_some());
        assert!(env.get(&Name::str("Valuation")).is_some());
        assert!(env.get(&Name::str("HadwigerThm")).is_some());
        assert!(env.get(&Name::str("KinematicFormula")).is_some());
        assert!(env.get(&Name::str("BanachMazurDistance")).is_some());
        assert!(env.get(&Name::str("BanachMazurCompactness")).is_some());
        assert!(env.get(&Name::str("JohnEllipsoid")).is_some());
        assert!(env.get(&Name::str("MinimalEnclosingEllipsoid")).is_some());
        assert!(env.get(&Name::str("NormalFan")).is_some());
        assert!(env.get(&Name::str("GaussMap")).is_some());
        assert!(env.get(&Name::str("HellyThm")).is_some());
        assert!(env.get(&Name::str("FractionalHelly")).is_some());
        assert!(env.get(&Name::str("ColorfulHelly")).is_some());
        assert!(env.get(&Name::str("RadonPartition")).is_some());
        assert!(env.get(&Name::str("HamSandwichTheorem")).is_some());
        assert!(env.get(&Name::str("CenterpointThm")).is_some());
        assert!(env.get(&Name::str("CaratheodoryThm")).is_some());
        assert!(env.get(&Name::str("BorsukPartition")).is_some());
        assert!(env.get(&Name::str("ChromaticNumberEuclidean")).is_some());
    }
    #[test]
    fn test_mixed_volume_estimator_intrinsic() {
        let est = MixedVolumeEstimator::new(3);
        assert!((est.intrinsic_volume_hypercube(0) - 1.0).abs() < 1e-12);
        assert!((est.intrinsic_volume_hypercube(1) - 3.0).abs() < 1e-12);
        assert!((est.intrinsic_volume_hypercube(2) - 3.0).abs() < 1e-12);
        assert!((est.intrinsic_volume_hypercube(3) - 1.0).abs() < 1e-12);
        assert_eq!(est.intrinsic_volume_hypercube(4), 0.0);
    }
    #[test]
    fn test_mixed_volume_estimator_steiner() {
        let est = MixedVolumeEstimator::new(2);
        let coeffs = est.steiner_coefficients_hypercube();
        assert_eq!(coeffs.len(), 3);
        assert!((coeffs[0] - 1.0).abs() < 1e-12, "V_2=1 got {}", coeffs[0]);
        assert!((coeffs[1] - 2.0).abs() < 1e-12, "V_1=2 got {}", coeffs[1]);
        assert!((coeffs[2] - 1.0).abs() < 1e-12, "V_0=1 got {}", coeffs[2]);
    }
    #[test]
    fn test_mixed_volume_brunn_minkowski() {
        let est = MixedVolumeEstimator::new(2);
        assert!(est.check_brunn_minkowski(1.0, 1.0, 4.0));
        assert!(est.check_brunn_minkowski(1.0, 4.0, 9.0));
    }
    #[test]
    fn test_mixed_volume_mean_width() {
        let est = MixedVolumeEstimator::new(4);
        let mw = est.mean_width_hypercube();
        assert!((mw - 2.0).abs() < 1e-12, "mean width = 2, got {mw}");
    }
    #[test]
    fn test_minkowski_sum_computer_compute() {
        let msc = MinkowskiSumComputer::new(2);
        let a = vec![vec![0.0, 0.0], vec![1.0, 0.0]];
        let b = vec![vec![0.0, 0.0], vec![0.0, 1.0]];
        let s = msc.compute(&a, &b);
        assert_eq!(s.len(), 4, "2x2 product");
        let h = msc.support_function_sum(&a, &b, &[1.0, 1.0]);
        assert!((h - 2.0).abs() < 1e-12, "h_{{A+B}}(1,1)=2, got {h}");
    }
    #[test]
    fn test_minkowski_sum_computer_dilate() {
        let msc = MinkowskiSumComputer::new(2);
        let verts = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let dilated = msc.dilate(&verts, 3.0);
        assert!((dilated[0][0] - 3.0).abs() < 1e-12);
        assert!((dilated[1][1] - 3.0).abs() < 1e-12);
    }
    #[test]
    fn test_minkowski_sum_computer_translate() {
        let msc = MinkowskiSumComputer::new(2);
        let verts = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let translated = msc.translate(&verts, &[2.0, 3.0]);
        assert!((translated[0][0] - 2.0).abs() < 1e-12);
        assert!((translated[1][1] - 4.0).abs() < 1e-12);
    }
    #[test]
    fn test_john_ellipsoid_from_points() {
        let points = vec![
            vec![1.0, 0.0],
            vec![-1.0, 0.0],
            vec![0.0, 2.0],
            vec![0.0, -2.0],
        ];
        let ell = JohnEllipsoidApprox::from_points(&points)
            .expect("JohnEllipsoidApprox::from_points should succeed");
        assert!(
            ell.centre[0].abs() < 1e-12,
            "centre_x=0, got {}",
            ell.centre[0]
        );
        assert!(
            ell.centre[1].abs() < 1e-12,
            "centre_y=0, got {}",
            ell.centre[1]
        );
        assert!(ell.axes[0] > 0.5 && ell.axes[0] < 2.0);
        assert!(ell.axes[1] > 1.0 && ell.axes[1] < 3.0);
    }
    #[test]
    fn test_john_ellipsoid_contains() {
        let points = vec![
            vec![1.0, 0.0],
            vec![-1.0, 0.0],
            vec![0.0, 1.0],
            vec![0.0, -1.0],
        ];
        let ell = JohnEllipsoidApprox::from_points(&points)
            .expect("JohnEllipsoidApprox::from_points should succeed");
        assert!(ell.contains(&[0.0, 0.0]));
    }
    #[test]
    fn test_john_ellipsoid_volume_2d() {
        let points = vec![
            vec![1.0, 0.0],
            vec![-1.0, 0.0],
            vec![0.0, 1.0],
            vec![0.0, -1.0],
        ];
        let ell = JohnEllipsoidApprox::from_points(&points)
            .expect("JohnEllipsoidApprox::from_points should succeed");
        let vol = ell.volume();
        assert!(vol > 0.0, "volume > 0, got {vol}");
    }
    #[test]
    fn test_john_ellipsoid_outer_bound() {
        let points = vec![
            vec![2.0, 0.0],
            vec![-2.0, 0.0],
            vec![0.0, 2.0],
            vec![0.0, -2.0],
        ];
        let ell = JohnEllipsoidApprox::from_points(&points)
            .expect("JohnEllipsoidApprox::from_points should succeed");
        let outer = ell.johns_outer_bound();
        for j in 0..2 {
            assert!(
                (outer[j] - 2.0 * ell.axes[j]).abs() < 1e-12,
                "outer[{j}] = 2*axes[{j}]"
            );
        }
    }
    #[test]
    fn test_helly_all_intersect_1d() {
        let mut chk = HellyTheoremChecker::new(1);
        chk.add_box(vec![0.0], vec![2.0]);
        chk.add_box(vec![1.0], vec![3.0]);
        chk.add_box(vec![0.5], vec![2.5]);
        assert!(chk.all_intersect(), "all three intervals share [1, 2]");
    }
    #[test]
    fn test_helly_not_all_intersect() {
        let mut chk = HellyTheoremChecker::new(1);
        chk.add_box(vec![0.0], vec![1.0]);
        chk.add_box(vec![2.0], vec![3.0]);
        assert!(!chk.all_intersect(), "disjoint intervals");
    }
    #[test]
    fn test_helly_pairwise_condition_1d() {
        let mut chk = HellyTheoremChecker::new(1);
        chk.add_box(vec![0.0], vec![3.0]);
        chk.add_box(vec![1.0], vec![4.0]);
        chk.add_box(vec![2.0], vec![5.0]);
        assert!(chk.verify_helly_condition_1d());
    }
    #[test]
    fn test_helly_fraction_pairwise() {
        let mut chk = HellyTheoremChecker::new(1);
        chk.add_box(vec![0.0], vec![2.0]);
        chk.add_box(vec![1.0], vec![3.0]);
        chk.add_box(vec![5.0], vec![6.0]);
        let frac = chk.fraction_pairwise_intersecting();
        assert!((frac - 1.0 / 3.0).abs() < 1e-12, "fraction=1/3, got {frac}");
    }
    #[test]
    fn test_radon_partition_1d() {
        assert!(HellyTheoremChecker::has_radon_partition_1d(&[
            1.0, 5.0, 3.0
        ]));
        assert!(!HellyTheoremChecker::has_radon_partition_1d(&[1.0, 2.0]));
    }
    #[test]
    fn test_alexandrov_fenchel_check() {
        let est = MixedVolumeEstimator::new(2);
        assert!(est.check_alexandrov_fenchel_2d(2.0, 2.0, 2.0));
        assert!(est.check_alexandrov_fenchel_2d(2.0, 4.0, 3.0));
    }
}
#[cfg(test)]
mod tests_convex_geom_ext {
    use super::*;
    #[test]
    fn test_log_concave_sequence() {
        let lc = LogConcaveSequence::new(vec![1.0, 4.0, 6.0, 4.0, 1.0]);
        assert!(lc.is_log_concave);
        let mason = lc.mason_conjecture_statement();
        assert!(mason.contains("Mason"));
        let bh = lc.branden_huh_lorentzian_connection();
        assert!(bh.contains("Lorentzian"));
    }
    #[test]
    fn test_log_concave_negative() {
        let not_lc = LogConcaveSequence::new(vec![1.0, 1.0, 5.0, 1.0, 1.0]);
        assert!(!not_lc.is_log_concave);
    }
    #[test]
    fn test_lorentzian_polynomial() {
        let lp = LorentzianPolynomial::new(2, 3);
        assert!(lp.hessian_is_psd());
        assert!(lp.implies_log_concavity());
        let hodge = lp.connection_to_hodge_theory();
        assert!(hodge.contains("Hodge"));
    }
    #[test]
    fn test_convex_valuation() {
        let chi = ConvexValuation::euler_characteristic();
        assert_eq!(chi.homogeneity_degree, Some(0));
        let hadwiger = chi.hadwiger_theorem();
        assert!(hadwiger.contains("Hadwiger"));
        let mcm = chi.mcmullen_decomposition();
        assert!(mcm.contains("McMullen"));
    }
    #[test]
    fn test_intrinsic_volume() {
        let iv = IntrinsicVolume::new(3, 1);
        let kinematic = iv.kinematic_formula();
        assert!(kinematic.contains("Kinematic"));
        let steiner = iv.steiner_formula_contribution();
        assert!(steiner.contains("Steiner"));
    }
    #[test]
    fn test_lattice_polytope() {
        let simp = LatticePolytope::simplex(2);
        assert_eq!(simp.num_lattice_points(1), 3);
        let vol = simp.volume_from_ehrhart();
        assert!(vol > 0.0);
    }
    #[test]
    fn test_zonotope() {
        let gens = vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]];
        let z = ZonotopeData::new(gens);
        assert!(z.is_centrally_symmetric());
        let vol = z.volume_formula();
        assert!(vol.contains("Vol(Z)"));
        let tiling = z.tilings_of_space_description();
        assert!(tiling.contains("Zonotopes"));
    }
}
