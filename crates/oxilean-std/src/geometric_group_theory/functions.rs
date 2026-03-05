//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    AmalgamLetter, AmalgamWord, CayleyGraph, GrowthData, GrowthFunction, GrowthType, HNNSyllable,
    HNNWord, HyperbolicGroup, QIConstants, WordMetric,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn int_ty() -> Expr {
    cst("Int")
}
pub fn list_ty(a: Expr) -> Expr {
    app(cst("List"), a)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
/// CayleyGraph : Type — (G : Type) → GeneratingSet G → Type
pub fn cayley_graph_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(app(cst("GeneratingSet"), bvar(0)), type0()),
    )
}
/// GeneratingSet : Type → Type
pub fn generating_set_ty() -> Expr {
    arrow(type0(), type0())
}
/// GroupElement : CayleyGraph → Type
pub fn group_element_ty() -> Expr {
    arrow(cst("CayleyGraph"), type0())
}
/// cayley_edge : CayleyGraph → GroupElement → GroupElement → Bool
pub fn cayley_edge_ty() -> Expr {
    arrow(
        cst("CayleyGraph"),
        arrow(cst("GroupElement"), arrow(cst("GroupElement"), bool_ty())),
    )
}
/// cayley_label : CayleyGraph → GroupElement → GroupElement → GeneratingSet
pub fn cayley_label_ty() -> Expr {
    arrow(
        cst("CayleyGraph"),
        arrow(
            cst("GroupElement"),
            arrow(cst("GroupElement"), cst("GeneratingSet")),
        ),
    )
}
/// cayley_connected : CayleyGraph → Prop
pub fn cayley_connected_ty() -> Expr {
    arrow(cst("CayleyGraph"), prop())
}
/// WordLength : (G : Type) → G → GeneratingSet G → Nat
pub fn word_length_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(bvar(0), arrow(app(cst("GeneratingSet"), bvar(1)), nat_ty())),
    )
}
/// WordMetric : (G : Type) → GeneratingSet G → G → G → Nat
pub fn word_metric_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        arrow(
            app(cst("GeneratingSet"), bvar(0)),
            arrow(bvar(1), arrow(bvar(2), nat_ty())),
        ),
    )
}
/// WordMetricSpace : Type — metric space structure from word metric
pub fn word_metric_space_ty() -> Expr {
    type0()
}
/// word_metric_sym : WordMetric G S g h = WordMetric G S h g
pub fn word_metric_sym_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        type0(),
        pi(
            BinderInfo::Default,
            "S",
            app(cst("GeneratingSet"), bvar(0)),
            prop(),
        ),
    )
}
/// word_metric_triangle : WordMetric G S g k ≤ WordMetric G S g h + WordMetric G S h k
pub fn word_metric_triangle_ty() -> Expr {
    pi(BinderInfo::Default, "G", type0(), prop())
}
/// QuasiIsometry : MetricSpace → MetricSpace → Type
pub fn quasi_isometry_ty() -> Expr {
    arrow(cst("MetricSpace"), arrow(cst("MetricSpace"), type0()))
}
/// QuasiIsometryConstants : Type — (C, D) constants for QI
pub fn quasi_isometry_constants_ty() -> Expr {
    type0()
}
/// is_quasi_isometry : (X Y : MetricSpace) → (X → Y) → QuasiIsometryConstants → Prop
pub fn is_quasi_isometry_ty() -> Expr {
    arrow(
        cst("MetricSpace"),
        arrow(
            cst("MetricSpace"),
            arrow(
                arrow(cst("MetricSpace"), cst("MetricSpace")),
                arrow(cst("QuasiIsometryConstants"), prop()),
            ),
        ),
    )
}
/// quasi_isometry_equiv : MetricSpace → MetricSpace → Prop
pub fn quasi_isometry_equiv_ty() -> Expr {
    arrow(cst("MetricSpace"), arrow(cst("MetricSpace"), prop()))
}
/// quasi_isometry_refl : quasi_isometry_equiv X X
pub fn quasi_isometry_refl_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("MetricSpace"),
        app2(cst("quasi_isometry_equiv"), bvar(0), bvar(0)),
    )
}
/// quasi_isometry_sym : quasi_isometry_equiv X Y → quasi_isometry_equiv Y X
pub fn quasi_isometry_sym_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("MetricSpace"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("MetricSpace"),
            arrow(
                app2(cst("quasi_isometry_equiv"), bvar(1), bvar(0)),
                app2(cst("quasi_isometry_equiv"), bvar(1), bvar(2)),
            ),
        ),
    )
}
/// quasi_isometry_trans : quasi_isometry_equiv X Y → quasi_isometry_equiv Y Z → quasi_isometry_equiv X Z
pub fn quasi_isometry_trans_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("MetricSpace"),
        pi(
            BinderInfo::Default,
            "Y",
            cst("MetricSpace"),
            pi(
                BinderInfo::Default,
                "Z",
                cst("MetricSpace"),
                arrow(
                    app2(cst("quasi_isometry_equiv"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("quasi_isometry_equiv"), bvar(2), bvar(1)),
                        app2(cst("quasi_isometry_equiv"), bvar(4), bvar(2)),
                    ),
                ),
            ),
        ),
    )
}
/// GromovHyperbolicSpace : Type — δ-hyperbolic metric space
pub fn gromov_hyperbolic_space_ty() -> Expr {
    type0()
}
/// HyperbolicGroup : Type — finitely generated group with δ-hyperbolic Cayley graph
pub fn hyperbolic_group_ty() -> Expr {
    type0()
}
/// hyperbolicity_constant : GromovHyperbolicSpace → Real
pub fn hyperbolicity_constant_ty() -> Expr {
    arrow(cst("GromovHyperbolicSpace"), real_ty())
}
/// gromov_product : GromovHyperbolicSpace → GroupElement → GroupElement → GroupElement → Real
pub fn gromov_product_ty() -> Expr {
    arrow(
        cst("GromovHyperbolicSpace"),
        arrow(
            cst("GroupElement"),
            arrow(cst("GroupElement"), arrow(cst("GroupElement"), real_ty())),
        ),
    )
}
/// is_delta_hyperbolic : GromovHyperbolicSpace → Real → Prop
pub fn is_delta_hyperbolic_ty() -> Expr {
    arrow(cst("GromovHyperbolicSpace"), arrow(real_ty(), prop()))
}
/// thin_triangles : GromovHyperbolicSpace → Prop — every geodesic triangle is δ-thin
pub fn thin_triangles_ty() -> Expr {
    arrow(cst("GromovHyperbolicSpace"), prop())
}
/// GromovBoundary : GromovHyperbolicSpace → Type
pub fn gromov_boundary_ty() -> Expr {
    arrow(cst("GromovHyperbolicSpace"), type0())
}
/// boundary_compactification : GromovHyperbolicSpace → Type
pub fn boundary_compactification_ty() -> Expr {
    arrow(cst("GromovHyperbolicSpace"), type0())
}
/// free_group_is_hyperbolic : ∀ n : Nat, IsHyperbolic (FreeGroup n)
pub fn free_group_is_hyperbolic_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app(cst("IsHyperbolic"), app(cst("FreeGroup"), bvar(0))),
    )
}
/// CATZeroSpace : Type — non-positive curvature metric space
pub fn cat_zero_space_ty() -> Expr {
    type0()
}
/// CATZeroGroup : Type — group acting geometrically on a CAT(0) space
pub fn cat_zero_group_ty() -> Expr {
    type0()
}
/// cat_zero_comparison : CATZeroSpace → Prop — comparison triangle condition
pub fn cat_zero_comparison_ty() -> Expr {
    arrow(cst("CATZeroSpace"), prop())
}
/// unique_geodesic : CATZeroSpace → GroupElement → GroupElement → Prop
pub fn unique_geodesic_ty() -> Expr {
    arrow(
        cst("CATZeroSpace"),
        arrow(cst("GroupElement"), arrow(cst("GroupElement"), prop())),
    )
}
/// fixed_point_theorem : CATZeroSpace → Group → Prop — bounded subgroups fix a point
pub fn fixed_point_theorem_ty() -> Expr {
    arrow(cst("CATZeroSpace"), arrow(cst("Group"), prop()))
}
/// solvable_subgroup_theorem : CATZeroGroup → Prop — virtually abelian implies fixes flat
pub fn solvable_subgroup_theorem_ty() -> Expr {
    arrow(cst("CATZeroGroup"), prop())
}
/// EndsOfGroup : Group → Nat — Freudenthal compactification boundary components
pub fn ends_of_group_ty() -> Expr {
    arrow(cst("Group"), nat_ty())
}
/// stallings_theorem : Group → Prop — 2+ ends iff splits over finite subgroup
pub fn stallings_theorem_ty() -> Expr {
    arrow(cst("Group"), prop())
}
/// hopf_theorem : Group → Prop — ends ∈ {0,1,2,∞}
pub fn hopf_theorem_ty() -> Expr {
    arrow(cst("Group"), prop())
}
/// ends_free_group : ∀ n ≥ 2, EndsOfGroup (FreeGroup n) = ∞
pub fn ends_free_group_ty() -> Expr {
    pi(BinderInfo::Default, "n", nat_ty(), prop())
}
/// GrowthFunction : Group → GeneratingSet → Nat → Nat
pub fn growth_function_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("GeneratingSet"), arrow(nat_ty(), nat_ty())),
    )
}
/// GrowthType : Type — polynomial, intermediate, or exponential
pub fn growth_type_ty() -> Expr {
    type0()
}
/// has_polynomial_growth : Group → Nat → Prop
pub fn has_polynomial_growth_ty() -> Expr {
    arrow(cst("Group"), arrow(nat_ty(), prop()))
}
/// has_exponential_growth : Group → Prop
pub fn has_exponential_growth_ty() -> Expr {
    arrow(cst("Group"), prop())
}
/// gromov_polynomial_growth : Group → Prop — poly growth iff virtually nilpotent
pub fn gromov_polynomial_growth_ty() -> Expr {
    arrow(cst("Group"), prop())
}
/// milnor_wolf_theorem : Group → Prop — solvable groups have poly or exp growth
pub fn milnor_wolf_theorem_ty() -> Expr {
    arrow(cst("Group"), prop())
}
/// grigorchuk_group_intermediate : Prop — Grigorchuk group has intermediate growth
pub fn grigorchuk_group_intermediate_ty() -> Expr {
    prop()
}
/// growth_rate : Group → GeneratingSet → Real
pub fn growth_rate_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("GeneratingSet"), real_ty()))
}
/// GraphOfGroups : Type — graph with vertex/edge group assignments
pub fn graph_of_groups_ty() -> Expr {
    type0()
}
/// BasserreTree : GraphOfGroups → Type — universal covering tree
pub fn basser_tree_ty() -> Expr {
    arrow(cst("GraphOfGroups"), type0())
}
/// FundamentalGroupOfGraph : GraphOfGroups → Type
pub fn fundamental_group_of_graph_ty() -> Expr {
    arrow(cst("GraphOfGroups"), type0())
}
/// tree_action : Group → BasserreTree → Prop — group acts on the Bass-Serre tree
pub fn tree_action_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("BasserreTree"), prop()))
}
/// basser_serre_theorem : GraphOfGroups → Prop — fundamental group acts on tree
pub fn basser_serre_theorem_ty() -> Expr {
    arrow(cst("GraphOfGroups"), prop())
}
/// vertex_stabilizer : Group → BasserreTree → GroupElement → Type
pub fn vertex_stabilizer_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("BasserreTree"), arrow(cst("GroupElement"), type0())),
    )
}
/// AmalgamatedFreeProduct : Group → Group → Group → Type — A *_C B
pub fn amalgamated_free_product_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("Group"), arrow(cst("Group"), type0())),
    )
}
/// amalgam_injection_left : A → AmalgamatedFreeProduct A B C
pub fn amalgam_injection_left_ty() -> Expr {
    arrow(cst("Group"), cst("AmalgamatedFreeProduct"))
}
/// amalgam_injection_right : B → AmalgamatedFreeProduct A B C
pub fn amalgam_injection_right_ty() -> Expr {
    arrow(cst("Group"), cst("AmalgamatedFreeProduct"))
}
/// amalgam_universal : AmalgamatedFreeProduct → Group → Prop — universal property
pub fn amalgam_universal_ty() -> Expr {
    arrow(cst("AmalgamatedFreeProduct"), arrow(cst("Group"), prop()))
}
/// normal_form_amalgam : AmalgamatedFreeProduct → List GroupElement → Prop
pub fn normal_form_amalgam_ty() -> Expr {
    arrow(
        cst("AmalgamatedFreeProduct"),
        arrow(list_ty(cst("GroupElement")), prop()),
    )
}
/// mayer_vietoris_amalgam : AmalgamatedFreeProduct → Prop
pub fn mayer_vietoris_amalgam_ty() -> Expr {
    arrow(cst("AmalgamatedFreeProduct"), prop())
}
/// HNNExtension : Group → Group → Group → Type — A*_φ (stable letter t)
pub fn hnn_extension_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("Group"), arrow(cst("Group"), type0())),
    )
}
/// hnn_stable_letter : HNNExtension → GroupElement
pub fn hnn_stable_letter_ty() -> Expr {
    arrow(cst("HNNExtension"), cst("GroupElement"))
}
/// hnn_normal_form : HNNExtension → List GroupElement → Prop
pub fn hnn_normal_form_ty() -> Expr {
    arrow(
        cst("HNNExtension"),
        arrow(list_ty(cst("GroupElement")), prop()),
    )
}
/// hnn_universal : HNNExtension → Group → Prop
pub fn hnn_universal_ty() -> Expr {
    arrow(cst("HNNExtension"), arrow(cst("Group"), prop()))
}
/// brittons_lemma : HNNExtension → Prop — normal form uniqueness
pub fn brittons_lemma_ty() -> Expr {
    arrow(cst("HNNExtension"), prop())
}
/// hnn_torsion_free : HNNExtension → Prop — base group torsion-free implies HNN torsion-free
pub fn hnn_torsion_free_ty() -> Expr {
    arrow(cst("HNNExtension"), prop())
}
/// Register all geometric group theory axioms into the kernel environment.
pub fn build_geometric_group_theory_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("GeneratingSet", generating_set_ty()),
        ("CayleyGraph", type0()),
        ("GroupElement", group_element_ty()),
        ("cayley_edge", cayley_edge_ty()),
        ("cayley_label", cayley_label_ty()),
        ("cayley_connected", cayley_connected_ty()),
        ("WordLength", word_length_ty()),
        ("WordMetric", word_metric_ty()),
        ("WordMetricSpace", word_metric_space_ty()),
        ("word_metric_sym", word_metric_sym_ty()),
        ("word_metric_triangle", word_metric_triangle_ty()),
        ("MetricSpace", type0()),
        ("QuasiIsometry", quasi_isometry_ty()),
        ("QuasiIsometryConstants", quasi_isometry_constants_ty()),
        ("is_quasi_isometry", is_quasi_isometry_ty()),
        ("quasi_isometry_equiv", quasi_isometry_equiv_ty()),
        ("quasi_isometry_refl", quasi_isometry_refl_ty()),
        ("quasi_isometry_sym", quasi_isometry_sym_ty()),
        ("quasi_isometry_trans", quasi_isometry_trans_ty()),
        ("GromovHyperbolicSpace", gromov_hyperbolic_space_ty()),
        ("HyperbolicGroup", hyperbolic_group_ty()),
        ("IsHyperbolic", arrow(type0(), prop())),
        ("FreeGroup", arrow(nat_ty(), type0())),
        ("Group", type0()),
        ("hyperbolicity_constant", hyperbolicity_constant_ty()),
        ("gromov_product", gromov_product_ty()),
        ("is_delta_hyperbolic", is_delta_hyperbolic_ty()),
        ("thin_triangles", thin_triangles_ty()),
        ("GromovBoundary", gromov_boundary_ty()),
        ("boundary_compactification", boundary_compactification_ty()),
        ("free_group_is_hyperbolic", free_group_is_hyperbolic_ty()),
        ("CATZeroSpace", cat_zero_space_ty()),
        ("CATZeroGroup", cat_zero_group_ty()),
        ("cat_zero_comparison", cat_zero_comparison_ty()),
        ("unique_geodesic", unique_geodesic_ty()),
        ("fixed_point_theorem", fixed_point_theorem_ty()),
        ("solvable_subgroup_theorem", solvable_subgroup_theorem_ty()),
        ("EndsOfGroup", ends_of_group_ty()),
        ("stallings_theorem", stallings_theorem_ty()),
        ("hopf_theorem", hopf_theorem_ty()),
        ("ends_free_group", ends_free_group_ty()),
        ("GrowthFunction", growth_function_ty()),
        ("GrowthType", growth_type_ty()),
        ("has_polynomial_growth", has_polynomial_growth_ty()),
        ("has_exponential_growth", has_exponential_growth_ty()),
        ("gromov_polynomial_growth", gromov_polynomial_growth_ty()),
        ("milnor_wolf_theorem", milnor_wolf_theorem_ty()),
        (
            "grigorchuk_group_intermediate",
            grigorchuk_group_intermediate_ty(),
        ),
        ("growth_rate", growth_rate_ty()),
        ("GraphOfGroups", graph_of_groups_ty()),
        ("BasserreTree", type0()),
        ("FundamentalGroupOfGraph", fundamental_group_of_graph_ty()),
        ("tree_action", tree_action_ty()),
        ("basser_serre_theorem", basser_serre_theorem_ty()),
        ("vertex_stabilizer", vertex_stabilizer_ty()),
        ("AmalgamatedFreeProduct", type0()),
        ("amalgamated_free_product", amalgamated_free_product_ty()),
        ("amalgam_injection_left", amalgam_injection_left_ty()),
        ("amalgam_injection_right", amalgam_injection_right_ty()),
        ("amalgam_universal", amalgam_universal_ty()),
        ("normal_form_amalgam", normal_form_amalgam_ty()),
        ("mayer_vietoris_amalgam", mayer_vietoris_amalgam_ty()),
        ("HNNExtension", type0()),
        ("hnn_extension", hnn_extension_ty()),
        ("hnn_stable_letter", hnn_stable_letter_ty()),
        ("hnn_normal_form", hnn_normal_form_ty()),
        ("hnn_universal", hnn_universal_ty()),
        ("brittons_lemma", brittons_lemma_ty()),
        ("hnn_torsion_free", hnn_torsion_free_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// Compute the word length of a word (already reduced) over a generating set.
pub fn word_length(word: &[i32]) -> usize {
    word.len()
}
/// Freely reduce a word: cancel consecutive inverse pairs.
pub fn free_reduce(word: &[i32]) -> Vec<i32> {
    let mut reduced: Vec<i32> = Vec::with_capacity(word.len());
    for &letter in word {
        if let Some(&last) = reduced.last() {
            if last == -letter {
                reduced.pop();
                continue;
            }
        }
        reduced.push(letter);
    }
    reduced
}
/// Compute the word metric distance between two elements given as words
/// (in a free group, this equals the length of the reduced product g⁻¹h).
pub fn free_group_word_metric(g: &[i32], h: &[i32]) -> usize {
    let mut product: Vec<i32> = g.iter().rev().map(|&x| -x).collect();
    product.extend_from_slice(h);
    free_reduce(&product).len()
}
/// Check if a map (given as pairwise distances) is a quasi-isometry
/// with the given constants.
pub fn is_quasi_isometry(
    source_dists: &[f64],
    target_dists: &[f64],
    constants: &QIConstants,
) -> bool {
    if source_dists.len() != target_dists.len() {
        return false;
    }
    source_dists
        .iter()
        .zip(target_dists.iter())
        .all(|(&dx, &dy)| constants.check_bounds(dx, dy))
}
/// Gromov product of three points x, y, z based at a basepoint o.
/// (x|y)_o = 1/2 (d(o,x) + d(o,y) - d(x,y))
pub fn gromov_product(d_ox: f64, d_oy: f64, d_xy: f64) -> f64 {
    0.5 * (d_ox + d_oy - d_xy)
}
/// Check if a finite metric space (given as an n×n distance matrix) is
/// δ-hyperbolic using the four-point condition:
/// For all x,y,z,w: (x|z)_w ≥ min((x|y)_w, (y|z)_w) - δ.
pub fn is_delta_hyperbolic(dists: &[Vec<f64>], delta: f64) -> bool {
    let n = dists.len();
    for w in 0..n {
        for x in 0..n {
            for y in 0..n {
                for z in 0..n {
                    let xz_w = gromov_product(dists[w][x], dists[w][z], dists[x][z]);
                    let xy_w = gromov_product(dists[w][x], dists[w][y], dists[x][y]);
                    let yz_w = gromov_product(dists[w][y], dists[w][z], dists[y][z]);
                    let rhs = xy_w.min(yz_w) - delta;
                    if xz_w < rhs - 1e-10 {
                        return false;
                    }
                }
            }
        }
    }
    true
}
/// Estimate the hyperbolicity constant δ for a finite metric space.
/// Returns the smallest δ such that the four-point condition holds.
pub fn hyperbolicity_constant(dists: &[Vec<f64>]) -> f64 {
    let n = dists.len();
    let mut max_violation = 0.0_f64;
    for w in 0..n {
        for x in 0..n {
            for y in 0..n {
                for z in 0..n {
                    let xz_w = gromov_product(dists[w][x], dists[w][z], dists[x][z]);
                    let xy_w = gromov_product(dists[w][x], dists[w][y], dists[x][y]);
                    let yz_w = gromov_product(dists[w][y], dists[w][z], dists[y][z]);
                    let violation = xy_w.min(yz_w) - xz_w;
                    if violation > max_violation {
                        max_violation = violation;
                    }
                }
            }
        }
    }
    max_violation.max(0.0)
}
/// Compute the number of ends of a graph (as an approximation for the group).
///
/// The number of ends is the maximum number of infinite connected components
/// when a finite subgraph is removed. For a finite graph, it approximates the
/// number of boundary components at infinity.
///
/// Returns 0 for empty graphs, 1 if the graph remains connected after any
/// finite ball removal, and 2 or ∞ otherwise.
pub fn approximate_ends(graph: &CayleyGraph, removal_radius: usize) -> usize {
    if graph.nodes.is_empty() {
        return 0;
    }
    let surviving: Vec<usize> = (0..graph.nodes.len())
        .filter(|&i| graph.nodes[i].word.len() > removal_radius)
        .collect();
    if surviving.is_empty() {
        return 1;
    }
    let surviving_set: HashSet<usize> = surviving.iter().cloned().collect();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut components = 0usize;
    for &start in &surviving {
        if visited.contains(&start) {
            continue;
        }
        components += 1;
        let mut stack = vec![start];
        while let Some(u) = stack.pop() {
            if visited.contains(&u) {
                continue;
            }
            visited.insert(u);
            for &(v, _) in &graph.adjacency[u] {
                if surviving_set.contains(&v) && !visited.contains(&v) {
                    stack.push(v);
                }
            }
        }
    }
    components
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_env_has_key_axioms() {
        let env = build_geometric_group_theory_env();
        assert!(env.get(&Name::str("CayleyGraph")).is_some());
        assert!(env.get(&Name::str("WordMetric")).is_some());
        assert!(env.get(&Name::str("HyperbolicGroup")).is_some());
        assert!(env.get(&Name::str("GromovBoundary")).is_some());
        assert!(env.get(&Name::str("AmalgamatedFreeProduct")).is_some());
        assert!(env.get(&Name::str("HNNExtension")).is_some());
        assert!(env.get(&Name::str("GraphOfGroups")).is_some());
        assert!(env.get(&Name::str("EndsOfGroup")).is_some());
    }
    #[test]
    fn test_free_group_cayley_graph_radius_1() {
        let g = CayleyGraph::free_group(2, 1);
        assert_eq!(g.num_vertices(), 5);
        assert!(g.is_connected());
        assert!(g.is_vertex_transitive());
    }
    #[test]
    fn test_cayley_graph_word_distance() {
        let g = CayleyGraph::free_group(1, 4);
        let idx_id = g.node_map[&vec![]];
        let idx_1 = g.node_map[&vec![1]];
        let idx_2 = g.node_map[&vec![1, 1]];
        assert_eq!(g.word_distance(idx_id, idx_1), Some(1));
        assert_eq!(g.word_distance(idx_id, idx_2), Some(2));
        assert_eq!(g.word_distance(idx_1, idx_2), Some(1));
    }
    #[test]
    fn test_free_group_word_metric() {
        let g = vec![1i32, 2];
        let h = vec![1i32, 3];
        let d = free_group_word_metric(&g, &h);
        assert_eq!(d, 2);
        assert_eq!(free_group_word_metric(&g, &g), 0);
    }
    #[test]
    fn test_gromov_product_tree() {
        let prod = gromov_product(1.0, 1.0, 2.0);
        assert!((prod - 0.0).abs() < 1e-10);
        let prod2 = gromov_product(2.0, 3.0, 1.0);
        assert!((prod2 - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_delta_hyperbolic_tree() {
        let n = 5;
        let mut dists = vec![vec![0.0; n]; n];
        for i in 1..n {
            dists[0][i] = 1.0;
            dists[i][0] = 1.0;
        }
        for i in 1..n {
            for j in 1..n {
                if i != j {
                    dists[i][j] = 2.0;
                }
            }
        }
        assert!(is_delta_hyperbolic(&dists, 0.0));
        let delta = hyperbolicity_constant(&dists);
        assert!(delta < 1e-10);
    }
    #[test]
    fn test_growth_data_free_group() {
        let g = CayleyGraph::free_group(2, 5);
        let gdata = GrowthData::from_cayley_graph(&g, 5);
        assert_eq!(gdata.ball_sizes[0], 1);
        assert_eq!(gdata.ball_sizes[1], 5);
        let rate = gdata.exponential_growth_rate();
        assert!(rate > 1.5, "Free group should have exponential growth");
        assert_eq!(gdata.classify(), GrowthType::Exponential);
    }
    #[test]
    fn test_hnn_word() {
        let w = HNNWord::new(vec![
            HNNSyllable::Base("a".to_string()),
            HNNSyllable::StableLetter(1),
            HNNSyllable::Base("b".to_string()),
            HNNSyllable::StableLetter(-1),
        ]);
        assert_eq!(w.len(), 4);
        assert_eq!(w.t_length(), 0);
        assert!(!w.is_empty());
    }
    #[test]
    fn test_amalgam_word_factor_counts() {
        let w = AmalgamWord::new(vec![
            AmalgamLetter::LeftFactor("a".to_string()),
            AmalgamLetter::RightFactor("b".to_string()),
            AmalgamLetter::Amalgam("c".to_string()),
            AmalgamLetter::LeftFactor("a2".to_string()),
        ]);
        let (l, r, am) = w.factor_counts();
        assert_eq!(l, 2);
        assert_eq!(r, 1);
        assert_eq!(am, 1);
    }
}
pub fn ggt_ext_group_prop() -> Expr {
    arrow(cst("Group"), prop())
}
pub fn ggt_ext_metric_prop() -> Expr {
    arrow(cst("MetricSpace"), prop())
}
/// Busemann function: h_γ(x) = lim_{t→∞} (d(x, γ(t)) - t).
/// BusemannFunction : GromovHyperbolicSpace → GeodRay → GroupElement → Real
pub fn busemann_function_ty() -> Expr {
    arrow(
        cst("GromovHyperbolicSpace"),
        arrow(cst("GeodRay"), arrow(cst("GroupElement"), real_ty())),
    )
}
/// Geodesic ray: an isometric embedding γ : [0,∞) → X.
/// GeodRay : MetricSpace → Type
pub fn geod_ray_ty() -> Expr {
    arrow(cst("MetricSpace"), type0())
}
/// Boundary at infinity: equivalence classes of geodesic rays.
/// BoundaryAtInfinity : MetricSpace → Type
pub fn boundary_at_infinity_ty() -> Expr {
    arrow(cst("MetricSpace"), type0())
}
/// Visual metric on the Gromov boundary.
/// VisualMetric : GromovHyperbolicSpace → Real → MetricSpace
pub fn visual_metric_ty() -> Expr {
    arrow(
        cst("GromovHyperbolicSpace"),
        arrow(real_ty(), cst("MetricSpace")),
    )
}
/// Quasi-geodesic: a quasi-isometric image of an interval.
/// QuasiGeodesic : MetricSpace → Real → Real → Type
pub fn quasi_geodesic_ty() -> Expr {
    arrow(
        cst("MetricSpace"),
        arrow(real_ty(), arrow(real_ty(), type0())),
    )
}
/// Morse lemma: quasi-geodesics lie within a bounded neighbourhood of true geodesics.
/// MorseLemma : ∀ (X : GromovHyperbolicSpace) (C D : Real),
///   ∀ γ : QuasiGeodesic X C D, ∃ R, HausdorffDist γ (NearestGeodesic γ) ≤ R
pub fn morse_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "X",
        cst("GromovHyperbolicSpace"),
        pi(
            BinderInfo::Default,
            "C",
            real_ty(),
            pi(
                BinderInfo::Default,
                "D",
                real_ty(),
                app(
                    cst("MorsePropertyHolds"),
                    app3(cst("QuasiGeodesic"), bvar(2), bvar(1), bvar(0)),
                ),
            ),
        ),
    )
}
/// Svarc-Milnor lemma: if G acts geometrically on X, then G is QI to X.
/// SvarcMilnorLemma : ∀ (G : Group) (X : MetricSpace),
///   GeometricAction G X → QuasiIsometricTo G X
pub fn svarc_milnor_lemma_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        pi(
            BinderInfo::Default,
            "X",
            cst("MetricSpace"),
            arrow(
                app2(cst("GeometricAction"), bvar(1), bvar(0)),
                app2(cst("QuasiIsometricTo"), bvar(2), bvar(1)),
            ),
        ),
    )
}
/// Geometric action: proper, cocompact, isometric group action.
/// GeometricAction : Group → MetricSpace → Prop
pub fn geometric_action_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("MetricSpace"), prop()))
}
/// Coarse geometry: the study of metric spaces up to quasi-isometry.
/// CoarseEquivalence : MetricSpace → MetricSpace → Prop
pub fn coarse_equivalence_ty() -> Expr {
    arrow(cst("MetricSpace"), arrow(cst("MetricSpace"), prop()))
}
/// Coarse embedding: f : X → Y is a coarse embedding.
/// CoarseEmbedding : MetricSpace → MetricSpace → Type
pub fn coarse_embedding_ty() -> Expr {
    arrow(cst("MetricSpace"), arrow(cst("MetricSpace"), type0()))
}
/// Amenable group: has a left-invariant finitely additive probability measure.
/// AmenableGroup : Group → Prop
pub fn amenable_group_ty() -> Expr {
    ggt_ext_group_prop()
}
/// Følner sequence: a sequence of finite sets witnessing amenability.
/// FolnerSequence : Group → Type
pub fn folner_sequence_ty() -> Expr {
    arrow(cst("Group"), type0())
}
/// Day's theorem: a group is amenable iff it has a Følner sequence.
/// DayTheorem : ∀ G : Group, AmenableGroup G ↔ HasFolnerSequence G
pub fn day_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        app2(
            cst("Iff"),
            app(cst("AmenableGroup"), bvar(0)),
            app(cst("HasFolnerSequence"), bvar(1)),
        ),
    )
}
/// Von Neumann's theorem: groups with free subgroup are non-amenable.
/// VonNeumannTheorem : ∀ G : Group, ContainsFreeGroup G → ¬AmenableGroup G
pub fn von_neumann_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        arrow(
            app(cst("ContainsFreeGroup"), bvar(0)),
            arrow(app(cst("AmenableGroup"), bvar(1)), cst("False")),
        ),
    )
}
/// Kazhdan property (T): every unitary representation with almost invariant vectors has invariant vectors.
/// KazhdanPropertyT : Group → Prop
pub fn kazhdan_property_t_ty() -> Expr {
    ggt_ext_group_prop()
}
/// Property (T) implies finite generation.
/// KazhdanFiniteGeneration : ∀ G : Group, KazhdanT G → FinitelyGenerated G
pub fn kazhdan_finite_generation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        arrow(
            app(cst("KazhdanT"), bvar(0)),
            app(cst("FinitelyGenerated"), bvar(1)),
        ),
    )
}
/// Haagerup property (a-T-menable): the group has a proper affine isometric action on a Hilbert space.
/// HaagerupProperty : Group → Prop
pub fn haagerup_property_ty() -> Expr {
    ggt_ext_group_prop()
}
/// Property (T) and Haagerup property together imply finiteness.
/// TAndHaagerupImpliesFinite : ∀ G : Group, KazhdanT G → HaagerupProperty G → Finite G
pub fn t_and_haagerup_implies_finite_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        arrow(
            app(cst("KazhdanT"), bvar(0)),
            arrow(
                app(cst("HaagerupProperty"), bvar(1)),
                app(cst("Finite"), bvar(2)),
            ),
        ),
    )
}
/// Bass-Serre tree: universal covering tree of a graph of groups.
/// BasserreTreeOf : GraphOfGroups → Tree
pub fn basserre_tree_of_ty() -> Expr {
    arrow(cst("GraphOfGroups"), cst("Tree"))
}
/// Tree: a connected acyclic graph.
/// Tree : Type
pub fn tree_ty() -> Expr {
    type0()
}
/// Group acts on tree without inversion: ∀ g ∈ G, g·e ≠ ē for all edges e.
/// ActsWithoutInversion : Group → Tree → Prop
pub fn acts_without_inversion_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("Tree"), prop()))
}
/// Vertex stabilizer is conjugate to a vertex group.
/// StabilizerConjugateToVertexGroup : GraphOfGroups → Group → GroupElement → Prop
pub fn stabilizer_conjugate_ty() -> Expr {
    arrow(
        cst("GraphOfGroups"),
        arrow(cst("Group"), arrow(cst("GroupElement"), prop())),
    )
}
/// A group splits over a subgroup H: G = A *_H B or G = A*_H.
/// SplitsOverSubgroup : Group → Group → Prop
pub fn splits_over_subgroup_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("Group"), prop()))
}
/// Accessibility: a group is accessible iff it has finitely many splittings over finite subgroups.
/// AccessibleGroup : Group → Prop
pub fn accessible_group_ty() -> Expr {
    ggt_ext_group_prop()
}
/// Dunwoody's accessibility theorem: finitely presented groups are accessible.
/// DunwoodyAccessibility : ∀ G : Group, FinitelyPresented G → Accessible G
pub fn dunwoody_accessibility_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        arrow(
            app(cst("FinitelyPresented"), bvar(0)),
            app(cst("Accessible"), bvar(1)),
        ),
    )
}
/// JSJ decomposition: canonical splitting of a group over cyclic subgroups.
/// JSJDecomposition : Group → GraphOfGroups
pub fn jsj_decomposition_ty() -> Expr {
    arrow(cst("Group"), cst("GraphOfGroups"))
}
/// Rips machine: analysis of group actions on real trees.
/// RipsMachine : Group → RealTree → Prop
pub fn rips_machine_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("RealTree"), prop()))
}
/// Real tree (ℝ-tree): a uniquely geodesic 0-hyperbolic space.
/// RealTree : Type
pub fn real_tree_ty() -> Expr {
    type0()
}
/// Commensurability of groups: G and H are commensurable if they share a common finite-index subgroup.
/// Commensurable : Group → Group → Prop
pub fn commensurable_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("Group"), prop()))
}
/// Virtual isomorphism: groups that are isomorphic up to finite index.
/// VirtualIsomorphism : Group → Group → Prop
pub fn virtual_isomorphism_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("Group"), prop()))
}
/// Residually finite group: every non-trivial element is detected by a finite quotient.
/// ResiduallyFinite : Group → Prop
pub fn residually_finite_ty() -> Expr {
    ggt_ext_group_prop()
}
/// Subgroup separability (LERF): every finitely generated subgroup is separable.
/// LERF : Group → Prop
pub fn lerf_ty() -> Expr {
    ggt_ext_group_prop()
}
/// Word-hyperbolic group is residually finite (Agol-Wise-Haglund).
/// HyperbolicResiduallyFinite : ∀ G : Group, Hyperbolic G → ResiduallyFinite G
pub fn hyperbolic_residually_finite_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        arrow(
            app(cst("HyperbolicGroup2"), bvar(0)),
            app(cst("ResiduallyFinite"), bvar(1)),
        ),
    )
}
/// Dehn function: isoperimetric function measuring word problem complexity.
/// DehnFunction : Group → Nat → Nat
pub fn dehn_function_ty() -> Expr {
    arrow(cst("Group"), arrow(nat_ty(), nat_ty()))
}
/// Linear Dehn function: group has linear Dehn function iff hyperbolic.
/// LinearDehnFunctionIffHyperbolic : ∀ G : Group,
///   LinearDehnFunction G ↔ HyperbolicGroup G
pub fn linear_dehn_function_iff_hyperbolic_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        app2(
            cst("Iff"),
            app(cst("LinearDehnFunction"), bvar(0)),
            app(cst("IsHyperbolic"), bvar(1)),
        ),
    )
}
/// Distortion of a subgroup: how much longer paths in the ambient group can be.
/// Distortion : Group → Group → Nat → Nat
pub fn distortion_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("Group"), arrow(nat_ty(), nat_ty())))
}
/// Automatic group: group with automatic structure (regular language of normal forms).
/// AutomaticGroup : Group → Prop
pub fn automatic_group_ty() -> Expr {
    ggt_ext_group_prop()
}
/// Hyperbolic groups are automatic (Cannon-Epstein-Holt-Levy-Paterson-Thurston).
/// HyperbolicIsAutomatic : ∀ G : Group, Hyperbolic G → AutomaticGroup G
pub fn hyperbolic_is_automatic_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        arrow(
            app(cst("IsHyperbolic"), bvar(0)),
            app(cst("AutomaticGroup"), bvar(1)),
        ),
    )
}
/// Tits alternative: a linear group is either virtually solvable or contains a free group.
/// TitsAlternative : ∀ G : LinearGroup, VirtuallySolvable G ∨ ContainsFree G
pub fn tits_alternative_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("LinearGroup"),
        app2(
            cst("Or"),
            app(cst("VirtuallySolvable"), bvar(0)),
            app(cst("ContainsFreeSubgroup"), bvar(1)),
        ),
    )
}
/// Girth of a Cayley graph: length of the shortest non-trivial cycle.
/// CayleyGirth : Group → GeneratingSet → Nat
pub fn cayley_girth_ty() -> Expr {
    arrow(cst("Group"), arrow(cst("GeneratingSet"), nat_ty()))
}
/// Cayley graph with large girth exists for any k (by probabilistic method).
/// LargeGirthCayleyGraph : ∀ n : Nat, ∃ G : Group, CayleyGirth G > n
pub fn large_girth_cayley_graph_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        app(cst("ExistsGroupWithLargeGirth"), bvar(0)),
    )
}
/// Expander graph: a sparse highly-connected Cayley graph with spectral gap.
/// ExpanderGraph : CayleyGraph2 → Prop
pub fn expander_graph_ty() -> Expr {
    arrow(cst("CayleyGraph2"), prop())
}
/// Spectral gap: second eigenvalue of the Laplacian bounded away from 0.
/// SpectralGap : Group → GeneratingSet → Real → Prop
pub fn spectral_gap_ty() -> Expr {
    arrow(
        cst("Group"),
        arrow(cst("GeneratingSet"), arrow(real_ty(), prop())),
    )
}
/// Margulis expanders: explicit expander families via Kazhdan property T.
/// MargulisExpanders : ∀ G : Group, KazhdanT G → HasExpanderCayleyGraphs G
pub fn margulis_expanders_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Group"),
        arrow(
            app(cst("KazhdanT"), bvar(0)),
            app(cst("HasExpanderCayleyGraphs"), bvar(1)),
        ),
    )
}
/// Property (FW): every action on a CAT(0) cube complex has a fixed point.
/// PropertyFW : Group → Prop
pub fn property_fw_ty() -> Expr {
    ggt_ext_group_prop()
}
/// Median space: a metric space where every triple has a unique median.
/// MedianSpace : MetricSpace → Prop
pub fn median_space_ty() -> Expr {
    ggt_ext_metric_prop()
}
/// CAT(0) cube complex acts on median space (Roller duality).
/// CubicalMedianDuality : CATZeroSpace → Prop
pub fn cubical_median_duality_ty() -> Expr {
    arrow(cst("CATZeroSpace"), prop())
}
/// Register all extended geometric group theory axioms into the kernel environment.
pub fn register_geometric_group_theory_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("GeodRay", type0()),
        ("Tree", tree_ty()),
        ("RealTree", real_tree_ty()),
        ("LinearGroup", type0()),
        ("CayleyGraph2", type0()),
        ("HyperbolicGroup2", arrow(cst("Group"), prop())),
        ("Finite", arrow(cst("Group"), prop())),
        ("FinitelyGenerated", arrow(cst("Group"), prop())),
        ("FinitelyPresented", arrow(cst("Group"), prop())),
        ("Accessible", arrow(cst("Group"), prop())),
        ("KazhdanT", arrow(cst("Group"), prop())),
        ("ContainsFreeGroup", arrow(cst("Group"), prop())),
        ("HasFolnerSequence", arrow(cst("Group"), prop())),
        ("ContainsFreeSubgroup", arrow(cst("Group"), prop())),
        ("VirtuallySolvable", arrow(cst("Group"), prop())),
        ("LinearDehnFunction", arrow(cst("Group"), prop())),
        ("HasExpanderCayleyGraphs", arrow(cst("Group"), prop())),
        ("ExistsGroupWithLargeGirth", arrow(nat_ty(), prop())),
        ("MorsePropertyHolds", arrow(type0(), prop())),
        (
            "GeometricAction",
            arrow(cst("Group"), arrow(cst("MetricSpace"), prop())),
        ),
        (
            "QuasiIsometricTo",
            arrow(cst("Group"), arrow(cst("MetricSpace"), prop())),
        ),
        ("Or", arrow(prop(), arrow(prop(), prop()))),
        ("busemann_function", busemann_function_ty()),
        ("geod_ray", geod_ray_ty()),
        ("boundary_at_infinity", boundary_at_infinity_ty()),
        ("visual_metric", visual_metric_ty()),
        ("quasi_geodesic", quasi_geodesic_ty()),
        ("morse_lemma", morse_lemma_ty()),
        ("svarc_milnor_lemma", svarc_milnor_lemma_ty()),
        ("geometric_action", geometric_action_ty()),
        ("coarse_equivalence", coarse_equivalence_ty()),
        ("coarse_embedding", coarse_embedding_ty()),
        ("amenable_group", amenable_group_ty()),
        ("folner_sequence", folner_sequence_ty()),
        ("day_theorem", day_theorem_ty()),
        ("von_neumann_theorem", von_neumann_theorem_ty()),
        ("kazhdan_property_t", kazhdan_property_t_ty()),
        ("kazhdan_finite_generation", kazhdan_finite_generation_ty()),
        ("haagerup_property", haagerup_property_ty()),
        (
            "t_and_haagerup_implies_finite",
            t_and_haagerup_implies_finite_ty(),
        ),
        ("basserre_tree_of", basserre_tree_of_ty()),
        ("acts_without_inversion", acts_without_inversion_ty()),
        ("stabilizer_conjugate", stabilizer_conjugate_ty()),
        ("splits_over_subgroup", splits_over_subgroup_ty()),
        ("accessible_group", accessible_group_ty()),
        ("dunwoody_accessibility", dunwoody_accessibility_ty()),
        ("jsj_decomposition", jsj_decomposition_ty()),
        ("rips_machine", rips_machine_ty()),
        ("real_tree", real_tree_ty()),
        ("commensurable", commensurable_ty()),
        ("virtual_isomorphism", virtual_isomorphism_ty()),
        ("residually_finite", residually_finite_ty()),
        ("lerf", lerf_ty()),
        (
            "hyperbolic_residually_finite",
            hyperbolic_residually_finite_ty(),
        ),
        ("dehn_function", dehn_function_ty()),
        (
            "linear_dehn_function_iff_hyperbolic",
            linear_dehn_function_iff_hyperbolic_ty(),
        ),
        ("distortion", distortion_ty()),
        ("automatic_group", automatic_group_ty()),
        ("hyperbolic_is_automatic", hyperbolic_is_automatic_ty()),
        ("tits_alternative", tits_alternative_ty()),
        ("cayley_girth", cayley_girth_ty()),
        ("large_girth_cayley_graph", large_girth_cayley_graph_ty()),
        ("expander_graph", expander_graph_ty()),
        ("spectral_gap", spectral_gap_ty()),
        ("margulis_expanders", margulis_expanders_ty()),
        ("property_fw", property_fw_ty()),
        ("median_space", median_space_ty()),
        ("cubical_median_duality", cubical_median_duality_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to add {}: {:?}", name, e))?;
    }
    Ok(())
}
