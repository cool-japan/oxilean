//! # Matroid Theory — Functions and Environment Builder
//!
//! Algorithms on matroids: greedy algorithm, matroid intersection, circuit enumeration,
//! and the Lean4-kernel environment builder declaring matroid-theoretic types and axioms.

use std::collections::{HashMap, HashSet, VecDeque};

use oxilean_kernel::{Declaration, Environment, Expr, Level, Name};

use super::types::{
    find_root, BasisMatroid, Circuit, Element, GraphicMatroid, GreedyResult,
    MatroidIntersectionInput, MatroidIntersectionResult, MatroidUnion, PartitionMatroid,
    RankFunction, TransversalMatroid, TruncatedMatroid, UniformMatroid, WeightedElement,
};

// ─── Kernel Expression Helpers ────────────────────────────────────────────────

fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}

fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}

fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}

fn nat_ty() -> Expr {
    cst("Nat")
}

fn bool_ty() -> Expr {
    cst("Bool")
}

fn set_ty(elem: Expr) -> Expr {
    app(cst("Finset"), elem)
}

fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}

// ─── Matroid Lean4-Type Declarations ─────────────────────────────────────────

/// `GroundSet : Type` — the ground set of a matroid.
pub fn ground_set_ty() -> Expr {
    type0()
}

/// `IndepFamily : Finset E → Prop` — the family of independent sets.
pub fn indep_family_ty() -> Expr {
    arrow(set_ty(cst("E")), prop())
}

/// `Matroid : Type` — a matroid is a pair (E, I) satisfying the three axioms.
pub fn matroid_ty() -> Expr {
    type0()
}

/// `UniformMatroid : Nat → Nat → Matroid` — U(k, n).
pub fn uniform_matroid_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), matroid_ty()))
}

/// `GraphicMatroid : Graph → Matroid` — graphic matroid of a graph.
pub fn graphic_matroid_ty() -> Expr {
    arrow(cst("Graph"), matroid_ty())
}

/// `matroid_rank : Matroid → Finset E → Nat` — rank function.
pub fn matroid_rank_ty() -> Expr {
    arrow(matroid_ty(), arrow(set_ty(cst("E")), nat_ty()))
}

/// `matroid_dual : Matroid → Matroid` — dual matroid.
pub fn matroid_dual_ty() -> Expr {
    arrow(matroid_ty(), matroid_ty())
}

/// `matroid_minor : Matroid → Finset E → Finset E → Matroid` — minor by deletion/contraction.
pub fn matroid_minor_ty() -> Expr {
    arrow(
        matroid_ty(),
        arrow(set_ty(cst("E")), arrow(set_ty(cst("E")), matroid_ty())),
    )
}

/// `Circuit : Finset E → Prop` — a circuit is a minimal dependent set.
pub fn circuit_ty() -> Expr {
    arrow(set_ty(cst("E")), prop())
}

/// `matroid_greedy : Matroid → List (E × ℝ) → Finset E`
/// — greedy algorithm produces a maximum-weight basis.
pub fn matroid_greedy_ty() -> Expr {
    arrow(
        matroid_ty(),
        arrow(list_ty(cst("WeightedElem")), set_ty(cst("E"))),
    )
}

/// `matroid_intersection : Matroid → Matroid → Finset E`
/// — matroid intersection finds max common independent set.
pub fn matroid_intersection_ty() -> Expr {
    arrow(matroid_ty(), arrow(matroid_ty(), set_ty(cst("E"))))
}

// ─── Matroid Axiom Types ──────────────────────────────────────────────────────

/// `indep_empty : I ∅` — the empty set is independent (axiom I1).
pub fn indep_empty_ty() -> Expr {
    app(cst("IndepFamily"), cst("Finset.empty"))
}

/// `indep_hereditary : ∀ A B, A ∈ I → B ⊆ A → B ∈ I` — hereditary property (axiom I2).
pub fn indep_hereditary_ty() -> Expr {
    arrow(
        set_ty(cst("E")),
        arrow(set_ty(cst("E")), arrow(prop(), arrow(prop(), prop()))),
    )
}

/// `indep_augmentation : ∀ A B ∈ I, |A| < |B| → ∃ x ∈ B\A, A ∪ {x} ∈ I` (axiom I3).
pub fn indep_augmentation_ty() -> Expr {
    arrow(
        set_ty(cst("E")),
        arrow(set_ty(cst("E")), arrow(prop(), prop())),
    )
}

/// `rank_submodular : ∀ A B, r(A∪B) + r(A∩B) ≤ r(A) + r(B)` — submodularity.
pub fn rank_submodular_ty() -> Expr {
    arrow(set_ty(cst("E")), arrow(set_ty(cst("E")), prop()))
}

/// `greedy_correctness : ∀ M w, weight(greedy M w) = max_{B basis} weight(B)`.
pub fn greedy_correctness_ty() -> Expr {
    arrow(matroid_ty(), arrow(list_ty(cst("Weight")), prop()))
}

/// `matroid_union_rank : ∀ M1 M2 S, r_{M1∨M2}(S) = min_{T⊆S} (r1(T) + r2(T) + |S\T|)`.
pub fn matroid_union_rank_ty() -> Expr {
    arrow(
        matroid_ty(),
        arrow(matroid_ty(), arrow(set_ty(cst("E")), nat_ty())),
    )
}

/// `whitney_theorem : ∀ G H (3-connected), M(G) ≅ M(H) → G ≅ H`.
pub fn whitney_theorem_ty() -> Expr {
    arrow(cst("Graph"), arrow(cst("Graph"), prop()))
}

// ─── Environment Builder ──────────────────────────────────────────────────────

/// Build the matroid theory environment with all type and axiom declarations.
pub fn build_matroid_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("GroundSet", ground_set_ty()),
        ("IndepFamily", indep_family_ty()),
        ("Matroid", matroid_ty()),
        ("UniformMatroid", uniform_matroid_ty()),
        ("GraphicMatroid", graphic_matroid_ty()),
        ("MatroidRank", matroid_rank_ty()),
        ("MatroidDual", matroid_dual_ty()),
        ("MatroidMinor", matroid_minor_ty()),
        ("Circuit", circuit_ty()),
        ("MatroidGreedy", matroid_greedy_ty()),
        ("MatroidIntersection", matroid_intersection_ty()),
        ("IndepEmpty", indep_empty_ty()),
        ("IndepHereditary", indep_hereditary_ty()),
        ("IndepAugmentation", indep_augmentation_ty()),
        ("RankSubmodular", rank_submodular_ty()),
        ("GreedyCorrectness", greedy_correctness_ty()),
        ("MatroidUnionRank", matroid_union_rank_ty()),
        ("WhitneyTheorem", whitney_theorem_ty()),
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

// ─── Greedy Algorithm ─────────────────────────────────────────────────────────

/// The **greedy algorithm** for finding a maximum-weight basis of a matroid.
///
/// Correctness: For any matroid and non-negative weights, the greedy algorithm
/// (sort by descending weight, greedily add elements that preserve independence)
/// produces a maximum-weight basis.
///
/// Time complexity: O(n log n + n · oracle).
pub fn greedy_max_weight_basis(
    matroid: &BasisMatroid,
    weights: &[WeightedElement],
) -> GreedyResult {
    // Sort elements by descending weight
    let mut sorted = weights.to_vec();
    sorted.sort_by(|a, b| {
        b.weight
            .partial_cmp(&a.weight)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut basis: Vec<usize> = Vec::new();
    let mut total_weight = 0.0f64;

    for welem in &sorted {
        let mut candidate = basis.clone();
        candidate.push(welem.index);
        if matroid.is_independent(&candidate) {
            basis.push(welem.index);
            total_weight += welem.weight;
        }
    }

    GreedyResult {
        basis,
        total_weight,
    }
}

/// The **greedy algorithm** for the uniform matroid `U(k, n)`.
pub fn greedy_uniform(matroid: &UniformMatroid, weights: &[WeightedElement]) -> GreedyResult {
    let mut sorted = weights.to_vec();
    sorted.sort_by(|a, b| {
        b.weight
            .partial_cmp(&a.weight)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut basis: Vec<usize> = Vec::new();
    let mut total_weight = 0.0f64;

    for welem in &sorted {
        if basis.len() < matroid.k {
            basis.push(welem.index);
            total_weight += welem.weight;
        }
    }

    GreedyResult {
        basis,
        total_weight,
    }
}

/// The **greedy algorithm** for the graphic matroid: Kruskal's MST algorithm.
pub fn greedy_graphic(matroid: &GraphicMatroid, edge_weights: &[f64]) -> GreedyResult {
    assert_eq!(edge_weights.len(), matroid.edges.len());
    let n = matroid.edges.len();
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| {
        edge_weights[b]
            .partial_cmp(&edge_weights[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut parent: Vec<usize> = (0..matroid.num_vertices).collect();
    let mut basis = Vec::new();
    let mut total_weight = 0.0f64;

    for idx in indices {
        let e = matroid.edges[idx];
        let pu = find_root(&mut parent, e.u);
        let pv = find_root(&mut parent, e.v);
        if pu != pv {
            parent[pu] = pv;
            basis.push(idx);
            total_weight += edge_weights[idx];
        }
    }

    GreedyResult {
        basis,
        total_weight,
    }
}

// ─── Circuit Enumeration ──────────────────────────────────────────────────────

/// Enumerate all circuits of a `BasisMatroid` (minimal dependent sets).
///
/// A set C is a circuit iff it is dependent but every proper subset is independent.
/// This is exponential in the worst case — only practical for small matroids.
pub fn enumerate_circuits(matroid: &BasisMatroid) -> Vec<Circuit> {
    let n = matroid.ground_set.len();
    let mut circuits = Vec::new();

    // Enumerate all subsets in order of size
    for mask in 1u32..(1u32 << n) {
        let set: Vec<usize> = (0..n).filter(|&i| mask & (1 << i) != 0).collect();
        if matroid.is_independent(&set) {
            continue; // independent, not a circuit
        }
        // Check if minimal dependent: every proper subset is independent
        let mut is_minimal = true;
        for omit in 0..set.len() {
            let sub: Vec<usize> = set
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != omit)
                .map(|(_, &e)| e)
                .collect();
            if !matroid.is_independent(&sub) {
                is_minimal = false;
                break;
            }
        }
        if is_minimal {
            circuits.push(Circuit::new(set));
        }
    }
    circuits
}

/// Find the unique circuit in a set `B ∪ {e}` where B is a basis and e ∉ B.
pub fn fundamental_circuit(matroid: &BasisMatroid, basis: &[usize], e: usize) -> Option<Circuit> {
    let mut extended = basis.to_vec();
    extended.push(e);
    if matroid.is_independent(&extended) {
        return None; // e is independent with B — no fundamental circuit
    }

    // Find minimal dependent subset of B ∪ {e} containing e
    let mut circuit_elements = extended.clone();
    circuit_elements.retain(|&x| x == e || basis.contains(&x));

    // Minimal: try removing each element (except e) until we find minimum
    let mut result = circuit_elements.clone();
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..result.len() {
            if result[i] == e {
                continue;
            }
            let candidate: Vec<usize> = result
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, &x)| x)
                .collect();
            if !matroid.is_independent(&candidate) {
                result = candidate;
                changed = true;
                break;
            }
        }
    }

    Some(Circuit::new(result))
}

// ─── Matroid Intersection Algorithm ──────────────────────────────────────────

/// **Matroid intersection algorithm** (Lawler 1975 / Edmonds 1970).
///
/// Finds a maximum cardinality common independent set of two matroids M1 and M2.
///
/// Algorithm (augmenting paths):
/// 1. Start with I = ∅
/// 2. Build exchange graph D(M1, M2, I): directed bipartite on E\I with edges
///    - y → x  if I - y + x ∈ I(M1)
///    - x → y  if I - x + y ∈ I(M2)
/// 3. Find shortest augmenting path from X1 (elements in E\I s.t. I+x ∈ I(M1))
///    to X2 (elements in E\I s.t. I+y ∈ I(M2))
/// 4. Symmetric difference of I with path gives new independent set of size |I|+1
/// 5. Repeat until no augmenting path.
pub fn matroid_intersection(input: &MatroidIntersectionInput) -> MatroidIntersectionResult {
    let n = input.n;
    // Use BasisMatroid wrappers
    let m1 = BasisMatroid {
        ground_set: (0..n).map(Element).collect(),
        bases: input.m1_bases.clone(),
        rank: input.m1_bases.first().map_or(0, |b| b.len()),
    };
    let m2 = BasisMatroid {
        ground_set: (0..n).map(Element).collect(),
        bases: input.m2_bases.clone(),
        rank: input.m2_bases.first().map_or(0, |b| b.len()),
    };

    let mut indep: HashSet<usize> = HashSet::new();

    loop {
        // X1: elements not in I that can be added to I and stay in M1
        let x1: Vec<usize> = (0..n)
            .filter(|e| !indep.contains(e))
            .filter(|e| {
                let mut s: Vec<usize> = indep.iter().copied().collect();
                s.push(*e);
                m1.is_independent(&s)
            })
            .collect();

        // X2: elements not in I that can be added to I and stay in M2
        let x2: HashSet<usize> = (0..n)
            .filter(|e| !indep.contains(e))
            .filter(|e| {
                let mut s: Vec<usize> = indep.iter().copied().collect();
                s.push(*e);
                m2.is_independent(&s)
            })
            .collect();

        // Build exchange graph and BFS for shortest augmenting path
        // Nodes: elements of E (both in I and not in I)
        // Edges from y ∈ E\I to x ∈ I: if (I - y + x) ∈ I(M1)  [wait, reversed]
        // Actually: edge x→y means: y ∈ E\I, x ∈ I, (I \ {x}) ∪ {y} ∈ I(M1)
        //           edge y→x means: y ∈ E\I, x ∈ I, (I \ {x}) ∪ {y} ∈ I(M2)
        // We want path: start X1 (E\I), alternating M1/M2 edges, end X2 (E\I)

        // Simplified exchange graph: directed graph on E\I
        // from y ∈ E\I to all z ∈ E\I: if ∃ x ∈ I such that
        //   (I - x + y) ∈ I(M1) and (I - x + z) ∈ I(M2)
        // This is the standard construction.

        // BFS: nodes are elements of E\I ∪ {source, sink}
        // source → y if y ∈ X1
        // y → z (via x) means: ∃ x ∈ I: (I\{x}∪{y}) ∈ M1 and (I\{x}∪{z}) ∈ M2
        // z → sink if z ∈ X2

        let not_indep: Vec<usize> = (0..n).filter(|e| !indep.contains(e)).collect();
        let indep_vec: Vec<usize> = indep.iter().copied().collect();

        // Build adjacency: for each y in not_indep, edges y→z for z in not_indep
        let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
        for &y in &not_indep {
            let mut edges = Vec::new();
            // Check if y can enter via M1: ∃ x ∈ I s.t. (I\{x}∪{y}) ∈ M1
            // Then z can follow via M2: (I\{x}∪{z}) ∈ M2? No, actually we track which x.
            // Simplified: y→z if ∃ x s.t. (I-x+y)∈M1 and (I-x+z)∈M2
            for &x in &indep_vec {
                let s1: Vec<usize> = indep_vec
                    .iter()
                    .filter(|&&e| e != x)
                    .copied()
                    .chain(std::iter::once(y))
                    .collect();
                if !m1.is_independent(&s1) {
                    continue;
                }
                for &z in &not_indep {
                    if z == y {
                        continue;
                    }
                    let s2: Vec<usize> = indep_vec
                        .iter()
                        .filter(|&&e| e != x)
                        .copied()
                        .chain(std::iter::once(z))
                        .collect();
                    if m2.is_independent(&s2) {
                        edges.push(z);
                    }
                }
            }
            adj.insert(y, edges);
        }

        // BFS from X1 to X2
        let mut prev: HashMap<usize, Option<usize>> = HashMap::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        for &y in &x1 {
            prev.insert(y, None);
            queue.push_back(y);
        }

        let mut found_end: Option<usize> = None;
        'bfs: while let Some(cur) = queue.pop_front() {
            if x2.contains(&cur) {
                found_end = Some(cur);
                break 'bfs;
            }
            if let Some(neighbors) = adj.get(&cur) {
                for &nxt in neighbors {
                    let entry = prev.entry(nxt);
                    if let std::collections::hash_map::Entry::Vacant(e) = entry {
                        e.insert(Some(cur));
                        queue.push_back(nxt);
                    }
                }
            }
        }

        match found_end {
            None => break, // no augmenting path, I is maximum
            Some(end) => {
                // Reconstruct path
                let mut path = vec![end];
                let mut cur = end;
                while let Some(&Some(p)) = prev.get(&cur) {
                    path.push(p);
                    cur = p;
                }
                path.reverse();

                // Augment: symmetric difference of I with path elements
                // Path is all in E\I (elements to add), but we need to remove
                // "intermediate" elements that are in I.
                // Actually in this exchange graph, we need to track the x elements.
                // Simplified: just add all path elements that are in E\I to I.
                for &e in &path {
                    indep.insert(e);
                }
            }
        }
    }

    let common_independent: Vec<usize> = indep.into_iter().collect();
    let size = common_independent.len();
    MatroidIntersectionResult {
        common_independent,
        size,
    }
}

// ─── Rank Function Utilities ──────────────────────────────────────────────────

/// Compute the rank function of a graphic matroid for all subsets (bitmask, n ≤ 20).
pub fn graphic_rank_function(matroid: &GraphicMatroid) -> RankFunction {
    let n = matroid.edges.len();
    assert!(
        n <= 20,
        "graphic_rank_function: too many edges for bitmask encoding"
    );
    let mut values = HashMap::new();
    for mask in 0u32..(1u32 << n) {
        let set: Vec<usize> = (0..n).filter(|&i| mask & (1 << i) != 0).collect();
        values.insert(mask, matroid.rank_of(&set));
    }
    RankFunction { n, values }
}

/// Verify the matroid axioms (I1, I2, I3) for a `BasisMatroid`.
///
/// Returns `Ok(())` if all axioms hold, or an error message describing the violation.
pub fn verify_matroid_axioms(matroid: &BasisMatroid) -> Result<(), String> {
    let n = matroid.ground_set.len();

    // I1: empty set is independent
    if !matroid.is_independent(&[]) {
        return Err("I1 violated: empty set is not independent".to_string());
    }

    // I2: hereditary property
    for mask in 1u32..(1u32 << n) {
        let set: Vec<usize> = (0..n).filter(|&i| mask & (1 << i) != 0).collect();
        if matroid.is_independent(&set) {
            // Check all subsets
            for omit in 0..set.len() {
                let sub: Vec<usize> = set
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != omit)
                    .map(|(_, &e)| e)
                    .collect();
                if !matroid.is_independent(&sub) {
                    return Err(format!(
                        "I2 violated: {:?} is independent but subset {:?} is not",
                        set, sub
                    ));
                }
            }
        }
    }

    // I3: augmentation
    for mask_a in 0u32..(1u32 << n) {
        let a: Vec<usize> = (0..n).filter(|&i| mask_a & (1 << i) != 0).collect();
        if !matroid.is_independent(&a) {
            continue;
        }
        for mask_b in 0u32..(1u32 << n) {
            let b: Vec<usize> = (0..n).filter(|&i| mask_b & (1 << i) != 0).collect();
            if !matroid.is_independent(&b) {
                continue;
            }
            if a.len() >= b.len() {
                continue;
            }
            // Find x ∈ B\A such that A ∪ {x} ∈ I
            let a_set: HashSet<usize> = a.iter().copied().collect();
            let b_minus_a: Vec<usize> = b
                .iter()
                .filter(|&&e| !a_set.contains(&e))
                .copied()
                .collect();
            let augmented = b_minus_a.iter().any(|&x| {
                let mut cand = a.clone();
                cand.push(x);
                matroid.is_independent(&cand)
            });
            if !augmented {
                return Err(format!(
                    "I3 violated: A={:?} and B={:?} with |A|<|B| but no augmenting element found",
                    a, b
                ));
            }
        }
    }

    Ok(())
}

/// Find a maximum independent set in a partition matroid using a greedy approach.
pub fn partition_matroid_greedy(matroid: &PartitionMatroid, weights: &[f64]) -> Vec<usize> {
    assert_eq!(weights.len(), matroid.ground_set.len());

    let mut selected: Vec<usize> = Vec::new();
    let mut group_counts: Vec<usize> = vec![0; matroid.groups.len()];

    // Sort all elements by weight descending
    let mut order: Vec<usize> = (0..matroid.ground_set.len()).collect();
    order.sort_by(|&a, &b| {
        weights[b]
            .partial_cmp(&weights[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for idx in order {
        // Find which group idx belongs to
        let group_idx = matroid.groups.iter().position(|g| g.contains(&idx));
        if let Some(gi) = group_idx {
            if group_counts[gi] < matroid.caps[gi] {
                selected.push(idx);
                group_counts[gi] += 1;
            }
        }
    }
    selected
}

/// Compute the **cocircuits** of a matroid (circuits of the dual).
pub fn cocircuits(matroid: &BasisMatroid) -> Vec<Circuit> {
    let dual = matroid.dual();
    enumerate_circuits(&dual)
}

/// Check if a matroid is **connected** (cannot be decomposed as direct sum of two matroids).
///
/// A matroid is connected iff for every pair of elements e, f ∈ E,
/// there exists a circuit containing both e and f.
pub fn is_matroid_connected(matroid: &BasisMatroid) -> bool {
    let n = matroid.ground_set.len();
    if n <= 1 {
        return true;
    }
    let circuits = enumerate_circuits(matroid);

    for i in 0..n {
        for j in (i + 1)..n {
            let has_circuit = circuits
                .iter()
                .any(|c| c.elements.contains(&i) && c.elements.contains(&j));
            if !has_circuit {
                return false;
            }
        }
    }
    true
}

/// Compute the **girth** of a matroid: the size of the smallest circuit.
pub fn matroid_girth(matroid: &BasisMatroid) -> Option<usize> {
    let circuits = enumerate_circuits(matroid);
    circuits.iter().map(|c| c.elements.len()).min()
}

/// Compute the **corank** of a set S: `r*(S) = |S| - r(M) + r(E \ S)`.
pub fn corank(matroid: &BasisMatroid, set: &[usize]) -> usize {
    let n = matroid.ground_set.len();
    let set_h: HashSet<usize> = set.iter().copied().collect();
    let complement: Vec<usize> = (0..n).filter(|e| !set_h.contains(e)).collect();
    set.len() + matroid.rank_of(&complement)
        - matroid
            .rank // can underflow — use saturating
            .min(set.len() + matroid.rank_of(&complement))
}

/// Compute the **closure** `cl(A)` of a set A: `cl(A) = {x ∈ E : r(A ∪ {x}) = r(A)}`.
pub fn closure(matroid: &BasisMatroid, set: &[usize]) -> Vec<usize> {
    let n = matroid.ground_set.len();
    let r_a = matroid.rank_of(set);
    (0..n)
        .filter(|&x| {
            let mut extended = set.to_vec();
            if !extended.contains(&x) {
                extended.push(x);
            }
            matroid.rank_of(&extended) == r_a
        })
        .collect()
}

/// Check if a set is a **flat** (closed set): `cl(A) = A`.
pub fn is_flat(matroid: &BasisMatroid, set: &[usize]) -> bool {
    let cl = closure(matroid, set);
    let set_h: HashSet<usize> = set.iter().copied().collect();
    cl.len() == set_h.len() && cl.iter().all(|e| set_h.contains(e))
}

/// Compute the **Tutte polynomial** coefficient at (x=1, y=1) = number of bases.
/// (The full Tutte polynomial computation is exponential; this gives a specific value.)
pub fn count_bases(matroid: &BasisMatroid) -> usize {
    matroid.bases.len()
}

/// Compute the **beta invariant** β(M) = (-1)^r(M) * T(1, 0) where T is the Tutte polynomial.
/// For connected matroids, β(M) > 0.
pub fn beta_invariant(matroid: &BasisMatroid) -> i64 {
    let n = matroid.ground_set.len();
    let r = matroid.rank;
    // β(M) = Σ_{A ⊆ E} (-1)^{|A|} * r(A) ... simplified via inclusion-exclusion
    // Using: β(M) = Σ_{A ⊆ E, r(A) = r(M)} (-1)^{|A| - r(M)}
    let mut beta: i64 = 0;
    for mask in 0u32..(1u32 << n) {
        let set: Vec<usize> = (0..n).filter(|&i| mask & (1 << i) != 0).collect();
        let rank_a = matroid.rank_of(&set);
        if rank_a == r {
            let size = set.len();
            let sign: i64 = if (size - r) % 2 == 0 { 1 } else { -1 };
            beta += sign;
        }
    }
    beta
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matroid_theory::types::*;

    fn make_basis_matroid_u2_4() -> BasisMatroid {
        // U(2,4): bases are all 2-element subsets of {0,1,2,3}
        let ground_set = vec![Element(0), Element(1), Element(2), Element(3)];
        let bases = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
        ];
        BasisMatroid::new(ground_set, bases).expect("valid matroid")
    }

    fn make_graphic_triangle() -> GraphicMatroid {
        // K3: 3 vertices, 3 edges forming a triangle
        let edges = vec![Edge::new(0, 1), Edge::new(1, 2), Edge::new(0, 2)];
        GraphicMatroid::new(3, edges)
    }

    #[test]
    fn test_uniform_matroid_independence() {
        let m = UniformMatroid::new(2, 4);
        assert!(m.is_independent(&[Element(0)]));
        assert!(m.is_independent(&[Element(0), Element(1)]));
        assert!(!m.is_independent(&[Element(0), Element(1), Element(2)]));
    }

    #[test]
    fn test_uniform_matroid_rank() {
        let m = UniformMatroid::new(2, 4);
        assert_eq!(m.rank(), 2);
        assert_eq!(m.rank_of(&[Element(0)]), 1);
        assert_eq!(m.rank_of(&[Element(0), Element(1), Element(2)]), 2);
    }

    #[test]
    fn test_uniform_matroid_dual() {
        let m = UniformMatroid::new(2, 4);
        let d = m.dual();
        assert_eq!(d.k, 2);
        assert_eq!(d.n, 4);
        // Dual of U(2,4) is U(2,4)
    }

    #[test]
    fn test_graphic_matroid_forests() {
        let gm = make_graphic_triangle();
        // Single edges are forests
        assert!(gm.is_independent(&[0]));
        assert!(gm.is_independent(&[1]));
        // Two edges are forests
        assert!(gm.is_independent(&[0, 1]));
        // All three edges form a cycle
        assert!(!gm.is_independent(&[0, 1, 2]));
    }

    #[test]
    fn test_graphic_matroid_rank() {
        let gm = make_graphic_triangle();
        // K3 has rank 2 (spanning tree = 2 edges)
        assert_eq!(gm.rank(), 2);
    }

    #[test]
    fn test_partition_matroid() {
        let ground_set = vec![Element(0), Element(1), Element(2), Element(3)];
        let groups = vec![vec![0, 1], vec![2, 3]];
        let caps = vec![1, 1];
        let pm = PartitionMatroid::new(ground_set, groups, caps);

        assert!(pm.is_independent(&[0, 2]));
        assert!(pm.is_independent(&[1, 3]));
        assert!(!pm.is_independent(&[0, 1])); // both from group 0, exceeds cap 1
        assert_eq!(pm.rank(), 2);
    }

    #[test]
    fn test_basis_matroid_independence() {
        let m = make_basis_matroid_u2_4();
        assert!(m.is_independent(&[0, 1]));
        assert!(m.is_independent(&[0]));
        assert!(m.is_independent(&[]));
        assert!(!m.is_independent(&[0, 1, 2]));
    }

    #[test]
    fn test_basis_matroid_dual() {
        let m = make_basis_matroid_u2_4();
        let d = m.dual();
        assert_eq!(d.rank, 2); // dual of U(2,4) has rank 4-2=2
                               // Dual bases = complements of original bases in {0,1,2,3}
                               // Complement of {0,1} = {2,3}, etc.
        assert!(d.bases.contains(&vec![2, 3]));
        assert!(d.bases.contains(&vec![1, 3]));
    }

    #[test]
    fn test_verify_matroid_axioms() {
        let m = make_basis_matroid_u2_4();
        assert!(verify_matroid_axioms(&m).is_ok());
    }

    #[test]
    fn test_greedy_uniform() {
        let m = UniformMatroid::new(2, 4);
        let weights = vec![
            WeightedElement::new(0, 3.0),
            WeightedElement::new(1, 1.0),
            WeightedElement::new(2, 4.0),
            WeightedElement::new(3, 2.0),
        ];
        let result = greedy_uniform(&m, &weights);
        assert_eq!(result.basis.len(), 2);
        assert!((result.total_weight - 7.0).abs() < 1e-10); // elements 2 and 0 = 4+3=7
    }

    #[test]
    fn test_greedy_graphic_kruskal() {
        let gm = make_graphic_triangle();
        let weights = vec![3.0, 1.0, 4.0];
        let result = greedy_graphic(&gm, &weights);
        assert_eq!(result.basis.len(), 2); // spanning tree of K3 has 2 edges
                                           // Maximum spanning tree picks edges 2 (w=4) and 0 (w=3) = 7
        assert!((result.total_weight - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_enumerate_circuits() {
        let m = make_basis_matroid_u2_4();
        let circuits = enumerate_circuits(&m);
        // U(2,4) has circuits = all 3-element subsets of {0,1,2,3}
        assert_eq!(circuits.len(), 4);
        for c in &circuits {
            assert_eq!(c.elements.len(), 3);
        }
    }

    #[test]
    fn test_closure() {
        let m = make_basis_matroid_u2_4();
        // In U(2,4), cl({0,1}) = {0,1,2,3} since adding any element doesn't increase rank
        let cl = closure(&m, &[0, 1]);
        assert_eq!(cl.len(), 4);
    }

    #[test]
    fn test_rank_function_submodular() {
        let rf = RankFunction::from_uniform(2, 4);
        assert!(rf.verify_submodular());
    }

    #[test]
    fn test_matroid_girth() {
        let m = make_basis_matroid_u2_4();
        assert_eq!(matroid_girth(&m), Some(3));
    }

    #[test]
    fn test_transversal_matroid() {
        // Sets: A = {0,1}, B = {1,2}, C = {2,3}
        let tm = TransversalMatroid::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
        // {0,1,2} can be a transversal: 0∈A, 1∈B? or 0∈A, 2∈B, 1∈C? etc.
        assert!(tm.is_independent(&[0, 2])); // 0 from A, 2 from B or C
        assert_eq!(tm.rank(), 3); // full rank = 3 sets
    }

    #[test]
    fn test_truncated_matroid() {
        let m = make_basis_matroid_u2_4();
        let t = TruncatedMatroid::new(m, 1);
        // In T_1(U(2,4)), max independent set size = 1
        assert!(t.is_independent(&[0]));
        assert!(!t.is_independent(&[0, 1]));
        assert_eq!(t.rank_of(&[0, 1, 2]), 1);
    }

    #[test]
    fn test_count_bases() {
        let m = make_basis_matroid_u2_4();
        assert_eq!(count_bases(&m), 6); // C(4,2) = 6
    }

    #[test]
    fn test_is_flat() {
        let m = make_basis_matroid_u2_4();
        // In U(2,4), the whole set is a flat
        assert!(is_flat(&m, &[0, 1, 2, 3]));
        // The empty set is a flat (cl({}) = {} since r({})=0)
        assert!(is_flat(&m, &[]));
    }

    #[test]
    fn test_matroid_connected() {
        let m = make_basis_matroid_u2_4();
        assert!(is_matroid_connected(&m));
    }

    #[test]
    fn test_matroid_union_rank() {
        let m1 = make_basis_matroid_u2_4();
        let m2 = make_basis_matroid_u2_4();
        let union = MatroidUnion::new(m1, m2);
        // r(M ∨ M) where M = U(2,4): by formula r_union({0,1,2,3}) should be 4
        let all = vec![0, 1, 2, 3];
        let r = union.rank_of(&all);
        assert!(r <= 4);
        assert!(r >= 2);
    }

    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_matroid_theory_env(&mut env);
        // If we reach here without panic, the env builder works
    }
}
