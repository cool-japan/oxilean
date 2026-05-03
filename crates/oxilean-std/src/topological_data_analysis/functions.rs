//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet};

use super::types::{
    AlphaComplex, BarCode, CechComplex, ContourTree, CoverElement, DiscreteMorseFunction,
    FormanCriticalSimplex, MapperGraph, MapperResult, MorseComplex, MorsePair, PersistenceDiagram,
    PersistenceInterval, PersistencePair, PersistentHomologyRepresentation, ReducedBoundaryMatrix,
    ReebGraph, ReebNodeType, Simplex, SimplicialComplex, TDASummaryStatistic,
    TomographicProjection, VietorisRipsComplex,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(a: Expr) -> Expr {
    app(cst("List"), a)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
/// Simplex : Type — a sorted list of vertex indices
pub fn simplex_ty() -> Expr {
    type0()
}
/// SimplicialComplex : Type
pub fn simplicial_complex_ty() -> Expr {
    type0()
}
/// VietorisRipsComplex : Type — built from pairwise distances with threshold ε
pub fn vietoris_rips_ty() -> Expr {
    type0()
}
/// CechComplex : Type — nerve of balls of radius ε
pub fn cech_complex_ty() -> Expr {
    type0()
}
/// AlphaComplex : Type — intersection of Voronoi cells
pub fn alpha_complex_ty() -> Expr {
    type0()
}
/// dimension : SimplicialComplex → Nat
pub fn dimension_ty() -> Expr {
    arrow(cst("SimplicialComplex"), nat_ty())
}
/// euler_characteristic : SimplicialComplex → Int
pub fn euler_characteristic_ty() -> Expr {
    arrow(cst("SimplicialComplex"), cst("Int"))
}
/// faces : Simplex → List Simplex
pub fn faces_ty() -> Expr {
    arrow(cst("Simplex"), list_ty(cst("Simplex")))
}
/// boundary_matrix : SimplicialComplex → Nat → List (List Int)  (∂_k)
pub fn boundary_matrix_ty() -> Expr {
    arrow(
        cst("SimplicialComplex"),
        arrow(nat_ty(), list_ty(list_ty(cst("Int")))),
    )
}
/// PersistenceInterval : Type — [birth, death) pair
pub fn persistence_interval_ty() -> Expr {
    type0()
}
/// PersistenceDiagram : Type — multiset of (birth, death) pairs
pub fn persistence_diagram_ty() -> Expr {
    type0()
}
/// BarCode : Type — intervals representing homology classes
pub fn barcode_ty() -> Expr {
    type0()
}
/// PersistencePair : Type — (σ_birth, σ_death) simplex pair
pub fn persistence_pair_ty() -> Expr {
    type0()
}
/// ReducedBoundaryMatrix : Type — column-reduced boundary matrix
pub fn reduced_boundary_matrix_ty() -> Expr {
    type0()
}
/// persistent_betti : PersistenceDiagram → Nat → Nat → Nat
pub fn persistent_betti_ty() -> Expr {
    arrow(
        cst("PersistenceDiagram"),
        arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
    )
}
/// bottleneck_distance : PersistenceDiagram → PersistenceDiagram → Real
pub fn bottleneck_distance_ty() -> Expr {
    arrow(
        cst("PersistenceDiagram"),
        arrow(cst("PersistenceDiagram"), real_ty()),
    )
}
/// wasserstein_distance : PersistenceDiagram → PersistenceDiagram → Real → Real
pub fn wasserstein_distance_ty() -> Expr {
    arrow(
        cst("PersistenceDiagram"),
        arrow(cst("PersistenceDiagram"), arrow(real_ty(), real_ty())),
    )
}
/// reduce_boundary_matrix : List (List Int) → ReducedBoundaryMatrix
pub fn reduce_boundary_matrix_ty() -> Expr {
    arrow(list_ty(list_ty(cst("Int"))), cst("ReducedBoundaryMatrix"))
}
/// CoverElement : Type — preimage interval + clustering result
pub fn cover_element_ty() -> Expr {
    type0()
}
/// MapperGraph : Type — nerve of a cover
pub fn mapper_graph_ty() -> Expr {
    type0()
}
/// MapperResult : Type — graph + coloring + lens function
pub fn mapper_result_ty() -> Expr {
    type0()
}
/// TomographicProjection : Type — filter function h : X → ℝ
pub fn tomographic_projection_ty() -> Expr {
    type0()
}
/// mapper_run : List (List Real) → Nat → Real → MapperResult
pub fn mapper_run_ty() -> Expr {
    arrow(
        list_ty(list_ty(real_ty())),
        arrow(nat_ty(), arrow(real_ty(), cst("MapperResult"))),
    )
}
/// DiscreteMorseFunction : Type
pub fn discrete_morse_function_ty() -> Expr {
    type0()
}
/// MorsePair : Type — (σ, τ) gradient pair
pub fn morse_pair_ty() -> Expr {
    type0()
}
/// MorseComplex : Type — complex of critical simplices
pub fn morse_complex_ty() -> Expr {
    type0()
}
/// FormanCriticalSimplex : Type — unpaired simplex
pub fn forman_critical_simplex_ty() -> Expr {
    type0()
}
/// critical_cells : DiscreteMorseFunction → List Simplex
pub fn critical_cells_ty() -> Expr {
    arrow(cst("DiscreteMorseFunction"), list_ty(cst("Simplex")))
}
/// morse_index : FormanCriticalSimplex → Nat
pub fn morse_index_ty() -> Expr {
    arrow(cst("FormanCriticalSimplex"), nat_ty())
}
/// is_gradient_pair : Simplex → Simplex → DiscreteMorseFunction → Bool
pub fn is_gradient_pair_ty() -> Expr {
    arrow(
        cst("Simplex"),
        arrow(
            cst("Simplex"),
            arrow(cst("DiscreteMorseFunction"), bool_ty()),
        ),
    )
}
/// ReebGraph : Type
pub fn reeb_graph_ty() -> Expr {
    type0()
}
/// ContourTree : Type — ordered Reeb graph
pub fn contour_tree_ty() -> Expr {
    type0()
}
/// PersistentHomologyRepresentation : Type
pub fn persistent_homology_repr_ty() -> Expr {
    type0()
}
/// TDASummaryStatistic : Type — Betti numbers, persistence entropy, landscape
pub fn tda_summary_statistic_ty() -> Expr {
    type0()
}
/// reeb_graph_from_function : SimplicialComplex → (Nat → Real) → ReebGraph
pub fn reeb_graph_from_function_ty() -> Expr {
    arrow(
        cst("SimplicialComplex"),
        arrow(arrow(nat_ty(), real_ty()), cst("ReebGraph")),
    )
}
/// betti_numbers : SimplicialComplex → List Nat
pub fn betti_numbers_ty() -> Expr {
    arrow(cst("SimplicialComplex"), list_ty(nat_ty()))
}
/// persistence_entropy : PersistenceDiagram → Real
pub fn persistence_entropy_ty() -> Expr {
    arrow(cst("PersistenceDiagram"), real_ty())
}
/// persistence_landscape : PersistenceDiagram → Nat → Real → Real
pub fn persistence_landscape_ty() -> Expr {
    arrow(
        cst("PersistenceDiagram"),
        arrow(nat_ty(), arrow(real_ty(), real_ty())),
    )
}
/// Register all TDA axioms into the kernel environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("Simplex", simplex_ty()),
        ("SimplicialComplex", simplicial_complex_ty()),
        ("VietorisRipsComplex", vietoris_rips_ty()),
        ("CechComplex", cech_complex_ty()),
        ("AlphaComplex", alpha_complex_ty()),
        ("dimension", dimension_ty()),
        ("euler_characteristic", euler_characteristic_ty()),
        ("faces", faces_ty()),
        ("boundary_matrix", boundary_matrix_ty()),
        ("PersistenceInterval", persistence_interval_ty()),
        ("PersistenceDiagram", persistence_diagram_ty()),
        ("BarCode", barcode_ty()),
        ("PersistencePair", persistence_pair_ty()),
        ("ReducedBoundaryMatrix", reduced_boundary_matrix_ty()),
        ("persistent_betti", persistent_betti_ty()),
        ("bottleneck_distance", bottleneck_distance_ty()),
        ("wasserstein_distance", wasserstein_distance_ty()),
        ("reduce_boundary_matrix", reduce_boundary_matrix_ty()),
        ("CoverElement", cover_element_ty()),
        ("MapperGraph", mapper_graph_ty()),
        ("MapperResult", mapper_result_ty()),
        ("TomographicProjection", tomographic_projection_ty()),
        ("mapper_run", mapper_run_ty()),
        ("DiscreteMorseFunction", discrete_morse_function_ty()),
        ("MorsePair", morse_pair_ty()),
        ("MorseComplex", morse_complex_ty()),
        ("FormanCriticalSimplex", forman_critical_simplex_ty()),
        ("critical_cells", critical_cells_ty()),
        ("morse_index", morse_index_ty()),
        ("is_gradient_pair", is_gradient_pair_ty()),
        ("ReebGraph", reeb_graph_ty()),
        ("ContourTree", contour_tree_ty()),
        (
            "PersistentHomologyRepresentation",
            persistent_homology_repr_ty(),
        ),
        ("TDASummaryStatistic", tda_summary_statistic_ty()),
        ("reeb_graph_from_function", reeb_graph_from_function_ty()),
        ("betti_numbers", betti_numbers_ty()),
        ("persistence_entropy", persistence_entropy_ty()),
        ("persistence_landscape", persistence_landscape_ty()),
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
/// Run the Mapper algorithm.
pub fn run_mapper(
    filter: &TomographicProjection,
    num_intervals: usize,
    overlap: f64,
) -> MapperResult {
    let n = filter.values.len();
    let lo = filter.min_val();
    let hi = filter.max_val();
    if n == 0 || hi <= lo || num_intervals == 0 {
        return MapperResult {
            graph: MapperGraph::new(),
            cover: vec![],
            filter_values: filter.values.clone(),
        };
    }
    let step = (hi - lo) / num_intervals as f64;
    let half_overlap = step * overlap / 2.0;
    let mut cover: Vec<CoverElement> = (0..num_intervals)
        .map(|i| {
            let lower = lo + i as f64 * step - half_overlap;
            let upper = lo + (i + 1) as f64 * step + half_overlap;
            let points: Vec<usize> = (0..n)
                .filter(|&j| filter.values[j] >= lower && filter.values[j] < upper)
                .collect();
            CoverElement::new(i, lower, upper, points)
        })
        .collect();
    for elem in &mut cover {
        if elem.points.len() <= 1 {
            elem.num_clusters = elem.points.len();
            continue;
        }
        let m = elem.points.len();
        let mut parent: Vec<usize> = (0..m).collect();
        fn find_p(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find_p(parent, parent[x]);
            }
            parent[x]
        }
        for a in 0..m {
            for b in (a + 1)..m {
                let fa = filter.values[elem.points[a]];
                let fb = filter.values[elem.points[b]];
                if (fa - fb).abs() < step / 2.0 {
                    let pa = find_p(&mut parent, a);
                    let pb = find_p(&mut parent, b);
                    if pa != pb {
                        parent[pa] = pb;
                    }
                }
            }
        }
        let roots: HashSet<usize> = (0..m).map(|i| find_p(&mut parent, i)).collect();
        let root_list: Vec<usize> = roots.into_iter().collect();
        elem.clusters = (0..m)
            .map(|i| {
                root_list
                    .iter()
                    .position(|&r| r == find_p(&mut parent, i))
                    .unwrap_or(0)
            })
            .collect();
        elem.num_clusters = root_list.len();
    }
    let mut graph = MapperGraph::new();
    let mut node_map: HashMap<(usize, usize), usize> = HashMap::new();
    for (ci, elem) in cover.iter().enumerate() {
        for cl in 0..elem.num_clusters {
            let pts: Vec<usize> = elem
                .points
                .iter()
                .enumerate()
                .filter(|(idx, _)| elem.clusters[*idx] == cl)
                .map(|(_, &p)| p)
                .collect();
            let color = if pts.is_empty() {
                0.0
            } else {
                pts.iter().map(|&p| filter.values[p]).sum::<f64>() / pts.len() as f64
            };
            let node_id = graph.add_node(ci, cl, color);
            node_map.insert((ci, cl), node_id);
        }
    }
    for ci in 0..cover.len() {
        for cj in (ci + 1)..cover.len() {
            let common: HashSet<usize> = cover[ci]
                .points
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&cover[cj].points.iter().copied().collect::<HashSet<_>>())
                .copied()
                .collect();
            for &p in &common {
                let idx_i = cover[ci]
                    .points
                    .iter()
                    .position(|&q| q == p)
                    .expect("p is in cover[ci].points: p came from the intersection");
                let idx_j = cover[cj]
                    .points
                    .iter()
                    .position(|&q| q == p)
                    .expect("p is in cover[cj].points: p came from the intersection");
                let cli = cover[ci].clusters[idx_i];
                let clj = cover[cj].clusters[idx_j];
                if let (Some(&ni), Some(&nj)) = (node_map.get(&(ci, cli)), node_map.get(&(cj, clj)))
                {
                    if !graph.edges.contains(&(ni, nj)) && !graph.edges.contains(&(nj, ni)) {
                        graph.add_edge(ni, nj);
                    }
                }
            }
        }
    }
    MapperResult {
        graph,
        cover,
        filter_values: filter.values.clone(),
    }
}
/// `ZigzagFiltration : Type` — a filtration with inclusions going both ways.
pub fn zigzag_filtration_ty() -> Expr {
    type0()
}
/// `ZigzagPersistenceModule : Type` — module over a zigzag poset.
pub fn zigzag_persistence_module_ty() -> Expr {
    type0()
}
/// `ZigzagBarcode : Type` — decomposition of a zigzag module into intervals.
pub fn zigzag_barcode_ty() -> Expr {
    type0()
}
/// `zigzag_decomposition_theorem : ZigzagPersistenceModule → ZigzagBarcode → Prop`
/// Every zigzag persistence module decomposes (uniquely up to isomorphism) into
/// interval modules.
pub fn zigzag_decomposition_theorem_ty() -> Expr {
    arrow(
        cst("ZigzagPersistenceModule"),
        arrow(cst("ZigzagBarcode"), prop()),
    )
}
/// `ExtendedPersistenceDiagram : Type`
/// Extended persistence extends the filtration to a combined up-down filtration,
/// capturing relative homology classes.
pub fn extended_persistence_diagram_ty() -> Expr {
    type0()
}
/// `extended_persistence_stability : ExtendedPersistenceDiagram → ExtendedPersistenceDiagram → Real → Prop`
/// Stability of extended persistence diagrams under perturbations of the Morse function.
pub fn extended_persistence_stability_ty() -> Expr {
    arrow(
        cst("ExtendedPersistenceDiagram"),
        arrow(cst("ExtendedPersistenceDiagram"), arrow(real_ty(), prop())),
    )
}
/// `MultidimensionalPersistenceModule : Nat → Type`
/// A persistence module indexed by ℕⁿ.
pub fn multidim_persistence_module_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MultidimensionalBarcode : Type`
/// A barcode-like invariant for multidimensional persistence (ranks of maps).
pub fn multidim_barcode_ty() -> Expr {
    type0()
}
/// `rank_invariant : MultidimensionalPersistenceModule → (Nat → Nat → Nat) → Prop`
/// The rank invariant β(s, t) = rank of the map H_*(X_s) → H_*(X_t).
pub fn rank_invariant_ty() -> Expr {
    arrow(
        app(cst("MultidimensionalPersistenceModule"), nat_ty()),
        arrow(arrow(nat_ty(), arrow(nat_ty(), nat_ty())), prop()),
    )
}
/// `multidim_persistence_classification : MultidimensionalPersistenceModule → Prop`
/// Multidimensional persistence is not completely classifiable by discrete invariants
/// (Carlsson-Zomorodian hardness theorem).
pub fn multidim_persistence_classification_ty() -> Expr {
    arrow(
        app(cst("MultidimensionalPersistenceModule"), nat_ty()),
        prop(),
    )
}
/// `InterleavingDistance : Type`
/// The interleaving distance between two persistence modules.
pub fn interleaving_distance_ty() -> Expr {
    type0()
}
/// `vietoris_rips_stability : Real → PersistenceDiagram → PersistenceDiagram → Prop`
/// Stability theorem: d_B(dgm(VR(X,ε)), dgm(VR(Y,ε))) ≤ 2 * d_GH(X, Y).
pub fn vietoris_rips_stability_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(
            cst("PersistenceDiagram"),
            arrow(cst("PersistenceDiagram"), prop()),
        ),
    )
}
/// `interleaving_distance_stability : InterleavingDistance → PersistenceDiagram → PersistenceDiagram → Prop`
/// The algebraic stability theorem: d_I(M, N) bounds d_B(dgm(M), dgm(N)).
pub fn interleaving_distance_stability_ty() -> Expr {
    arrow(
        cst("InterleavingDistance"),
        arrow(
            cst("PersistenceDiagram"),
            arrow(cst("PersistenceDiagram"), prop()),
        ),
    )
}
/// `algebraic_stability_theorem : PersistenceDiagram → PersistenceDiagram → InterleavingDistance → Prop`
/// The algebraic stability theorem: d_B ≤ d_I for any two persistence modules.
pub fn algebraic_stability_theorem_ty() -> Expr {
    arrow(
        cst("PersistenceDiagram"),
        arrow(
            cst("PersistenceDiagram"),
            arrow(cst("InterleavingDistance"), prop()),
        ),
    )
}
/// `bottleneck_distance_stability : SimplicialComplex → SimplicialComplex → Real → Prop`
/// The bottleneck distance between persistence diagrams of filtrations is stable
/// under perturbations of the filter function.
pub fn bottleneck_distance_stability_ty() -> Expr {
    arrow(
        cst("SimplicialComplex"),
        arrow(cst("SimplicialComplex"), arrow(real_ty(), prop())),
    )
}
/// `nerve_theorem : SimplicialComplex → Prop`
/// The nerve theorem: if all non-empty intersections of cover elements are contractible,
/// the nerve is homotopy equivalent to the union.
pub fn nerve_theorem_ty() -> Expr {
    arrow(cst("SimplicialComplex"), prop())
}
/// `cech_complex_nerve_theorem : CechComplex → SimplicialComplex → Prop`
/// The Čech complex is the nerve of the cover by balls; under convexity it is homotopy
/// equivalent to the underlying space.
pub fn cech_complex_nerve_theorem_ty() -> Expr {
    arrow(cst("CechComplex"), arrow(cst("SimplicialComplex"), prop()))
}
/// `CechRipsInterleaving : Real → PersistenceDiagram → PersistenceDiagram → Prop`
/// The Čech and Vietoris-Rips filtrations are interleaved:
/// VR(X, ε) ⊆ Č(X, ε) ⊆ VR(X, 2ε).
pub fn cech_rips_interleaving_ty() -> Expr {
    arrow(
        real_ty(),
        arrow(
            cst("PersistenceDiagram"),
            arrow(cst("PersistenceDiagram"), prop()),
        ),
    )
}
/// `WitnessComplex : Type`
/// A witness complex approximates the Čech complex using a landmark set.
pub fn witness_complex_ty() -> Expr {
    type0()
}
/// `WeakWitnessComplex : Type`
/// The weak witness complex: a simplex σ is in W(L, X) if each landmark is a
/// nearest neighbor of some witness for σ.
pub fn weak_witness_complex_ty() -> Expr {
    type0()
}
/// `witness_complex_approximation : WitnessComplex → CechComplex → Real → Prop`
/// Witness complexes approximate Čech complexes up to a scale factor.
pub fn witness_complex_approximation_ty() -> Expr {
    arrow(
        cst("WitnessComplex"),
        arrow(cst("CechComplex"), arrow(real_ty(), prop())),
    )
}
/// `DelaunayTriangulation : Type`
/// The Delaunay triangulation of a point set in ℝᵈ.
pub fn delaunay_triangulation_ty() -> Expr {
    type0()
}
/// `delaunay_triangulation_stability : DelaunayTriangulation → Real → Prop`
/// Small perturbations of the input points produce topologically equivalent
/// Delaunay triangulations (for general position inputs).
pub fn delaunay_triangulation_stability_ty() -> Expr {
    arrow(cst("DelaunayTriangulation"), arrow(real_ty(), prop()))
}
/// `alpha_complex_delaunay_sub : AlphaComplex → DelaunayTriangulation → Prop`
/// An alpha complex is a sub-complex of the Delaunay triangulation.
pub fn alpha_complex_delaunay_sub_ty() -> Expr {
    arrow(
        cst("AlphaComplex"),
        arrow(cst("DelaunayTriangulation"), prop()),
    )
}
/// `Sheaf : Type → Type`
/// A sheaf of abelian groups over a topological space.
pub fn sheaf_ty() -> Expr {
    arrow(type0(), type0())
}
/// `SheafCohomology : Type`
/// Sheaf cohomology groups H^k(X; F).
pub fn sheaf_cohomology_ty() -> Expr {
    type0()
}
/// `Cosheaf : Type → Type`
/// A cosheaf (co-presheaf satisfying the co-gluing axiom) over a topological space.
pub fn cosheaf_ty() -> Expr {
    arrow(type0(), type0())
}
/// `CosheafHomology : Type`
/// Homology of a cosheaf.
pub fn cosheaf_homology_ty() -> Expr {
    type0()
}
/// `sheaf_to_persistence : SimplicialComplex → SheafCohomology → PersistenceDiagram → Prop`
/// Persistence diagrams can be interpreted as invariants of constructible sheaves.
pub fn sheaf_to_persistence_ty() -> Expr {
    arrow(
        cst("SimplicialComplex"),
        arrow(
            cst("SheafCohomology"),
            arrow(cst("PersistenceDiagram"), prop()),
        ),
    )
}
/// `six_functor_formalism : Sheaf → Prop`
/// The six-functor formalism for constructible sheaves: f_*, f_!, f^*, f^!, ⊗, RHom.
pub fn six_functor_formalism_ty() -> Expr {
    arrow(app(cst("Sheaf"), type0()), prop())
}
/// `CubicalComplex : Type`
/// A cubical complex: a complex built from elementary intervals \[k, k+1\] in ℝᵈ.
pub fn cubical_complex_ty() -> Expr {
    type0()
}
/// `ElementaryCube : Type`
/// An elementary cube: a product of unit intervals and points.
pub fn elementary_cube_ty() -> Expr {
    type0()
}
/// `cubical_homology : CubicalComplex → Nat → Type`
/// The k-th cubical homology group of a cubical complex.
pub fn cubical_homology_ty() -> Expr {
    arrow(cst("CubicalComplex"), arrow(nat_ty(), type0()))
}
/// `cubical_boundary_operator : CubicalComplex → Nat → (ElementaryCube → ElementaryCube) → Prop`
/// The cubical boundary operator ∂_k for cubical homology.
pub fn cubical_boundary_operator_ty() -> Expr {
    arrow(
        cst("CubicalComplex"),
        arrow(
            nat_ty(),
            arrow(arrow(cst("ElementaryCube"), cst("ElementaryCube")), prop()),
        ),
    )
}
/// `cubical_vietoris_rips_comparison : CubicalComplex → SimplicialComplex → Prop`
/// Cubical homology agrees with simplicial homology for cubical sets.
pub fn cubical_vietoris_rips_comparison_ty() -> Expr {
    arrow(
        cst("CubicalComplex"),
        arrow(cst("SimplicialComplex"), prop()),
    )
}
/// `MorseSmalePair : Type`
/// A Morse-Smale pair (f, g) where f is a Morse function and g is a Riemannian metric.
pub fn morse_smale_pair_ty() -> Expr {
    type0()
}
/// `MorseSmaleComplex : Type`
/// The Morse-Smale complex decomposes the manifold into descending/ascending cells.
pub fn morse_smale_complex_ty() -> Expr {
    type0()
}
/// `morse_smale_decomposition : SimplicialComplex → DiscreteMorseFunction → MorseSmaleComplex → Prop`
/// The Morse-Smale complex decomposes the space by gradient flow regions.
pub fn morse_smale_decomposition_ty() -> Expr {
    arrow(
        cst("SimplicialComplex"),
        arrow(
            cst("DiscreteMorseFunction"),
            arrow(cst("MorseSmaleComplex"), prop()),
        ),
    )
}
/// `morse_smale_cancellation : MorseSmaleComplex → Nat → Nat → Prop`
/// Cancellation theorem: a pair of critical cells of adjacent indices can be cancelled
/// if they are connected by a unique gradient path.
pub fn morse_smale_cancellation_ty() -> Expr {
    arrow(
        cst("MorseSmaleComplex"),
        arrow(nat_ty(), arrow(nat_ty(), prop())),
    )
}
/// `forman_weak_morse_inequality : SimplicialComplex → DiscreteMorseFunction → Nat → Prop`
/// Weak Morse inequality: c_k ≥ β_k (number of critical k-cells ≥ k-th Betti number).
pub fn forman_weak_morse_inequality_ty() -> Expr {
    arrow(
        cst("SimplicialComplex"),
        arrow(cst("DiscreteMorseFunction"), arrow(nat_ty(), prop())),
    )
}
/// `forman_strong_morse_inequality : SimplicialComplex → DiscreteMorseFunction → Nat → Prop`
/// Strong Morse inequality: Σ (-1)^k c_k = χ (alternating sum = Euler characteristic).
pub fn forman_strong_morse_inequality_ty() -> Expr {
    arrow(
        cst("SimplicialComplex"),
        arrow(cst("DiscreteMorseFunction"), arrow(nat_ty(), prop())),
    )
}
/// `PersistentCohomology : Type`
/// Persistent cohomology: a dual theory tracking cohomology classes.
pub fn persistent_cohomology_ty() -> Expr {
    type0()
}
/// `CupProduct : PersistentCohomology → PersistentCohomology → PersistentCohomology`
/// The cup product structure on persistent cohomology.
pub fn cup_product_ty() -> Expr {
    arrow(
        cst("PersistentCohomology"),
        arrow(cst("PersistentCohomology"), cst("PersistentCohomology")),
    )
}
/// `persistence_cohomology_duality : PersistenceDiagram → PersistentCohomology → Prop`
/// Persistent (co)homology duality: cohomological persistence diagram equals homological
/// persistence diagram for manifolds (Poincaré duality).
pub fn persistence_cohomology_duality_ty() -> Expr {
    arrow(
        cst("PersistenceDiagram"),
        arrow(cst("PersistentCohomology"), prop()),
    )
}
/// `mapper_nerve_approximation : MapperResult → SimplicialComplex → Real → Prop`
/// The Mapper graph approximates the Reeb graph up to a resolution parameter.
pub fn mapper_nerve_approximation_ty() -> Expr {
    arrow(
        cst("MapperResult"),
        arrow(cst("SimplicialComplex"), arrow(real_ty(), prop())),
    )
}
/// `mapper_statistical_consistency : MapperResult → Real → Prop`
/// Statistical consistency of the Mapper algorithm: as the sample size grows and
/// resolution increases, the Mapper graph converges to the Reeb graph.
pub fn mapper_statistical_consistency_ty() -> Expr {
    arrow(cst("MapperResult"), arrow(real_ty(), prop()))
}
/// `mapper_cover_refinement : MapperResult → MapperResult → Prop`
/// Cover refinement monotonicity: refining the cover increases topological resolution.
pub fn mapper_cover_refinement_ty() -> Expr {
    arrow(cst("MapperResult"), arrow(cst("MapperResult"), prop()))
}
/// Register all extended TDA axioms into the kernel environment.
pub fn register_tda_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("ZigzagFiltration", zigzag_filtration_ty()),
        ("ZigzagPersistenceModule", zigzag_persistence_module_ty()),
        ("ZigzagBarcode", zigzag_barcode_ty()),
        (
            "zigzag_decomposition_theorem",
            zigzag_decomposition_theorem_ty(),
        ),
        (
            "ExtendedPersistenceDiagram",
            extended_persistence_diagram_ty(),
        ),
        (
            "extended_persistence_stability",
            extended_persistence_stability_ty(),
        ),
        ("MultidimensionalBarcode", multidim_barcode_ty()),
        ("rank_invariant", rank_invariant_ty()),
        (
            "multidim_persistence_classification",
            multidim_persistence_classification_ty(),
        ),
        ("InterleavingDistance", interleaving_distance_ty()),
        ("vietoris_rips_stability", vietoris_rips_stability_ty()),
        (
            "interleaving_distance_stability",
            interleaving_distance_stability_ty(),
        ),
        (
            "algebraic_stability_theorem",
            algebraic_stability_theorem_ty(),
        ),
        (
            "bottleneck_distance_stability",
            bottleneck_distance_stability_ty(),
        ),
        ("nerve_theorem", nerve_theorem_ty()),
        (
            "cech_complex_nerve_theorem",
            cech_complex_nerve_theorem_ty(),
        ),
        ("CechRipsInterleaving", cech_rips_interleaving_ty()),
        ("WitnessComplex", witness_complex_ty()),
        ("WeakWitnessComplex", weak_witness_complex_ty()),
        (
            "witness_complex_approximation",
            witness_complex_approximation_ty(),
        ),
        ("DelaunayTriangulation", delaunay_triangulation_ty()),
        (
            "delaunay_triangulation_stability",
            delaunay_triangulation_stability_ty(),
        ),
        (
            "alpha_complex_delaunay_sub",
            alpha_complex_delaunay_sub_ty(),
        ),
        ("SheafCohomology", sheaf_cohomology_ty()),
        ("CosheafHomology", cosheaf_homology_ty()),
        ("sheaf_to_persistence", sheaf_to_persistence_ty()),
        ("six_functor_formalism", six_functor_formalism_ty()),
        ("CubicalComplex", cubical_complex_ty()),
        ("ElementaryCube", elementary_cube_ty()),
        ("cubical_homology", cubical_homology_ty()),
        ("cubical_boundary_operator", cubical_boundary_operator_ty()),
        (
            "cubical_vietoris_rips_comparison",
            cubical_vietoris_rips_comparison_ty(),
        ),
        ("MorseSmalePair", morse_smale_pair_ty()),
        ("MorseSmaleComplex", morse_smale_complex_ty()),
        ("morse_smale_decomposition", morse_smale_decomposition_ty()),
        ("morse_smale_cancellation", morse_smale_cancellation_ty()),
        (
            "forman_weak_morse_inequality",
            forman_weak_morse_inequality_ty(),
        ),
        (
            "forman_strong_morse_inequality",
            forman_strong_morse_inequality_ty(),
        ),
        ("PersistentCohomology", persistent_cohomology_ty()),
        ("cup_product", cup_product_ty()),
        (
            "persistence_cohomology_duality",
            persistence_cohomology_duality_ty(),
        ),
        (
            "mapper_nerve_approximation",
            mapper_nerve_approximation_ty(),
        ),
        (
            "mapper_statistical_consistency",
            mapper_statistical_consistency_ty(),
        ),
        ("mapper_cover_refinement", mapper_cover_refinement_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add {name}: {e:?}"))?;
    }
    Ok(())
}
pub(super) fn tda_ext_euclidean_dist(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simplex_faces() {
        let s = Simplex::new(vec![0, 1, 2]);
        assert_eq!(s.dimension(), 2);
        let faces = s.faces();
        assert_eq!(faces.len(), 3);
    }
    #[test]
    fn test_simplicial_complex_euler() {
        let mut sc = SimplicialComplex::new();
        sc.add_simplex(Simplex::new(vec![0, 1, 2]));
        let chi = sc.euler_characteristic();
        assert_eq!(chi, 1);
    }
    #[test]
    fn test_vietoris_rips() {
        let dist = vec![
            vec![0.0, 1.0, 1.0],
            vec![1.0, 0.0, 1.0],
            vec![1.0, 1.0, 0.0],
        ];
        let vr = VietorisRipsComplex::build(&dist, 1.0, 2);
        assert!(vr.complex.simplices.contains(&Simplex::new(vec![0, 1])));
        assert!(vr.complex.simplices.contains(&Simplex::new(vec![0, 1, 2])));
    }
    #[test]
    fn test_persistence_diagram() {
        let mut diag = PersistenceDiagram::new();
        diag.add(PersistenceInterval::new(0, 0.0, 1.0));
        diag.add(PersistenceInterval::new(0, 0.0, f64::INFINITY));
        diag.add(PersistenceInterval::new(1, 0.5, 1.5));
        assert_eq!(diag.persistent_betti(0, 0.5), 2);
        assert_eq!(diag.persistent_betti(1, 1.0), 1);
    }
    #[test]
    fn test_bottleneck_distance_same() {
        let mut d1 = PersistenceDiagram::new();
        d1.add(PersistenceInterval::new(0, 0.0, 1.0));
        let mut d2 = PersistenceDiagram::new();
        d2.add(PersistenceInterval::new(0, 0.0, 1.0));
        assert!((d1.bottleneck_distance(&d2) - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_reduced_boundary_matrix() {
        let mat = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]];
        let reduced = ReducedBoundaryMatrix::reduce(&mat);
        assert_eq!(reduced.num_rows, 3);
    }
    #[test]
    fn test_mapper_single_point() {
        let filter = TomographicProjection {
            name: "x".to_string(),
            values: vec![0.0],
        };
        let result = run_mapper(&filter, 2, 0.2);
        assert!(result.graph.nodes.len() <= 2);
    }
    #[test]
    fn test_morse_complex() {
        let mut sc = SimplicialComplex::new();
        sc.add_simplex(Simplex::new(vec![0, 1]));
        sc.add_simplex(Simplex::new(vec![1, 2]));
        let mut mf = DiscreteMorseFunction::new();
        mf.set_value(&Simplex::new(vec![0]), 0.0);
        mf.set_value(&Simplex::new(vec![1]), 1.0);
        mf.set_value(&Simplex::new(vec![2]), 2.0);
        mf.set_value(&Simplex::new(vec![0, 1]), 0.5);
        mf.set_value(&Simplex::new(vec![1, 2]), 1.5);
        let mc = MorseComplex::build(&sc, &mf);
        assert!(!mc.critical_simplices.is_empty());
    }
    #[test]
    fn test_reeb_graph() {
        let mut rg = ReebGraph::new();
        let n0 = rg.add_node(0.0, ReebNodeType::Minimum);
        let n1 = rg.add_node(1.0, ReebNodeType::Maximum);
        rg.add_edge(n0, n1);
        assert_eq!(rg.minima().len(), 1);
        assert_eq!(rg.maxima().len(), 1);
    }
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("SimplicialComplex")).is_some());
        assert!(env.get(&Name::str("PersistenceDiagram")).is_some());
        assert!(env.get(&Name::str("ReebGraph")).is_some());
        assert!(env.get(&Name::str("MorseComplex")).is_some());
    }
}
