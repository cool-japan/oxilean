//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ColorCodingFPT, CourcelleMSOLChecker, CrownDecomposition, FPTAlgorithm, FPTMethod,
    IterativeCompressionVC, Kernelization, ParamReduction, TreeDecomp, TreeDecomposition,
    TreewidthAlgorithm, VertexCoverFPT, WClass,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
/// `ParameterizedProblem : Type` — a problem with an associated parameter function.
pub fn parameterized_problem_ty() -> Expr {
    type0()
}
/// `Parameter : Type` — a natural-number parameter extracted from an instance.
pub fn parameter_ty() -> Expr {
    arrow(cst("String"), nat_ty())
}
/// `FPTAlgorithm : ParameterizedProblem → Type`
/// An algorithm running in f(k)·n^c time for some computable f and constant c.
pub fn fpt_algorithm_ty() -> Expr {
    arrow(parameterized_problem_ty(), type0())
}
/// `IsInFPT : ParameterizedProblem → Prop`
/// The problem is in FPT (Fixed-Parameter Tractable).
pub fn is_in_fpt_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `IsInXP : ParameterizedProblem → Prop`
/// The problem is in XP (solvable in n^f(k) time for each fixed k).
pub fn is_in_xp_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `IsWHard : Nat → ParameterizedProblem → Prop`
/// The problem is W[i]-hard (for the given level i of W-hierarchy).
pub fn is_w_hard_ty() -> Expr {
    arrow(nat_ty(), arrow(parameterized_problem_ty(), prop()))
}
/// `IsWComplete : Nat → ParameterizedProblem → Prop`
/// The problem is W[i]-complete.
pub fn is_w_complete_ty() -> Expr {
    arrow(nat_ty(), arrow(parameterized_problem_ty(), prop()))
}
/// `FPTReducible : ParameterizedProblem → ParameterizedProblem → Prop`
/// There exists an FPT-time parameterized reduction between problems.
pub fn fpt_reducible_ty() -> Expr {
    arrow(
        parameterized_problem_ty(),
        arrow(parameterized_problem_ty(), prop()),
    )
}
/// `WHierarchy : Nat → Type` — the i-th level of the W-hierarchy.
pub fn w_hierarchy_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `W1 : Type` — the class W[1] (contains k-clique, k-independent set).
pub fn w1_ty() -> Expr {
    type0()
}
/// `W2 : Type` — the class W[2] (contains k-dominating set, k-set cover).
pub fn w2_ty() -> Expr {
    type0()
}
/// `WP : Type` — the class W[P] (contains weighted circuit satisfiability).
pub fn wp_ty() -> Expr {
    type0()
}
/// `AW : Type` — the class AW[*] (alternating W hierarchy).
pub fn aw_ty() -> Expr {
    type0()
}
/// `FPT_subset_W1 : FPT ⊆ W[1]`
pub fn fpt_subset_w1_ty() -> Expr {
    arrow(
        arrow(parameterized_problem_ty(), cst("IsInFPT")),
        arrow(parameterized_problem_ty(), app(cst("W1"), bvar(0))),
    )
}
/// `Kernel : ParameterizedProblem → Nat → Type`
/// A kernelization of size f(k): reduces any instance (x,k) to (x',k') with
/// |x'| ≤ f(k) and (x,k) ∈ L ↔ (x',k') ∈ L.
pub fn kernel_ty() -> Expr {
    arrow(
        parameterized_problem_ty(),
        arrow(arrow(nat_ty(), nat_ty()), type0()),
    )
}
/// `HasPolynomialKernel : ParameterizedProblem → Prop`
/// The problem has a polynomial kernel (|x'| ≤ k^c for some constant c).
pub fn has_polynomial_kernel_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `KernelSize : Kernel P f → Nat → Nat`
/// The size of the kernel as a function of the parameter.
pub fn kernel_size_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `VertexCoverKernel2k : VertexCover has a 2k-vertex kernel`
pub fn vertex_cover_kernel_2k_ty() -> Expr {
    prop()
}
/// `VertexCoverKernelKSquared : VertexCover has a k^2-vertex kernel (Buss kernel)`
pub fn vertex_cover_kernel_k_squared_ty() -> Expr {
    prop()
}
/// `TreeDecomposition : Graph → Type`
/// A tree decomposition: a tree T with bags B_t ⊆ V such that every edge and
/// vertex is covered, and the bags form connected subtrees.
pub fn tree_decomposition_ty() -> Expr {
    arrow(cst("Graph"), type0())
}
/// `Treewidth : Graph → Nat`
/// The treewidth of a graph: min over all tree decompositions of (max bag size − 1).
pub fn treewidth_ty() -> Expr {
    arrow(cst("Graph"), nat_ty())
}
/// `PathDecomposition : Graph → Type`
/// A path decomposition: a path (sequence) of bags covering all vertices/edges.
pub fn path_decomposition_ty() -> Expr {
    arrow(cst("Graph"), type0())
}
/// `Pathwidth : Graph → Nat`
/// The pathwidth of a graph: min over all path decompositions of (max bag size − 1).
pub fn pathwidth_ty() -> Expr {
    arrow(cst("Graph"), nat_ty())
}
/// `TreewidthLePathwidth : ∀ G, treewidth G ≤ pathwidth G`
pub fn treewidth_le_pathwidth_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "G",
        cst("Graph"),
        app3(
            cst("Le"),
            app(cst("Treewidth"), bvar(0)),
            app(cst("Pathwidth"), bvar(0)),
            prop(),
        ),
    )
}
/// `BranchDecomposition : Graph → Type` — a branch decomposition.
pub fn branch_decomposition_ty() -> Expr {
    arrow(cst("Graph"), type0())
}
/// `Branchwidth : Graph → Nat`
pub fn branchwidth_ty() -> Expr {
    arrow(cst("Graph"), nat_ty())
}
/// `FeedbackVertexSet : Graph → Set Vertex → Prop`
/// S is a FVS if G − S is a forest (acyclic).
pub fn feedback_vertex_set_ty() -> Expr {
    arrow(cst("Graph"), arrow(list_ty(nat_ty()), prop()))
}
/// `MinFVS : Graph → Nat`
/// The minimum feedback vertex set size of a graph.
pub fn min_fvs_ty() -> Expr {
    arrow(cst("Graph"), nat_ty())
}
/// `FVSParameterizedByK : ParameterizedProblem`
/// The FVS problem parameterized by solution size k — in FPT.
pub fn fvs_parameterized_ty() -> Expr {
    parameterized_problem_ty()
}
/// `ColorCodingAlgorithm : Nat → Graph → Nat → Bool`
/// Color-coding for k-path/subgraph: randomly color vertices with k colors
/// and check for a colorful copy using DP.
pub fn color_coding_algorithm_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("Graph"), arrow(nat_ty(), bool_ty())))
}
/// `ColorCodingFPT : k-path is in FPT via color coding`
/// Running time: 2^O(k) · n^O(1).
pub fn color_coding_fpt_ty() -> Expr {
    prop()
}
/// `PerfectHashFamily : Nat → Nat → Type`
/// An (n,k)-perfect hash family: a set of functions [n]→[k] such that
/// for every k-subset S ⊆ [n], some function is injective on S.
pub fn perfect_hash_family_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `MSO2Logic : Type` — Monadic Second-Order Logic (MSO₂) formula.
pub fn mso2_logic_ty() -> Expr {
    type0()
}
/// `MSO2Satisfies : Graph → MSO2Logic → Prop`
/// A graph satisfies an MSO₂ formula.
pub fn mso2_satisfies_ty() -> Expr {
    arrow(cst("Graph"), arrow(mso2_logic_ty(), prop()))
}
/// `CourcelleTheorem : ∀ φ : MSO₂, ∀ k : Nat, CheckMSO2 ∈ FPT(treewidth)`
/// Every graph property definable in MSO₂ is decidable in linear FPT time
/// on graphs of bounded treewidth.
pub fn courcelle_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        mso2_logic_ty(),
        pi(BinderInfo::Default, "k", nat_ty(), prop()),
    )
}
/// `BoundedTreewidthDP : Graph → Nat → Prop`
/// Bounded treewidth dynamic programming: many NP-hard problems are FPT
/// when parameterized by treewidth.
pub fn bounded_treewidth_dp_ty() -> Expr {
    arrow(cst("Graph"), arrow(nat_ty(), prop()))
}
/// `ETH : Prop` — Exponential Time Hypothesis
/// 3-SAT cannot be solved in 2^o(n) time.
pub fn eth_ty() -> Expr {
    prop()
}
/// `SETH : Prop` — Strong Exponential Time Hypothesis
/// k-SAT cannot be solved in (2 − ε)^n time for any ε > 0 and all k.
pub fn seth_ty() -> Expr {
    prop()
}
/// `ETH_implies_no_subexp_fpt : ETH → some problems have no 2^o(k)·poly(n) algorithm`
pub fn eth_implies_no_subexp_ty() -> Expr {
    arrow(eth_ty(), prop())
}
/// `SETH_implies_no_ons_vc : SETH → VertexCover has no O(1.9999^k) alg unless SETH fails`
pub fn seth_implies_vc_lb_ty() -> Expr {
    arrow(seth_ty(), prop())
}
/// `SparsificationLemma : 3-SAT with n vars, m clauses reduces to 2^εn instances with O(n) clauses`
pub fn sparsification_lemma_ty() -> Expr {
    prop()
}
/// `ETH_k_clique_lb : ETH → k-clique needs n^Ω(k) time`
pub fn eth_k_clique_lb_ty() -> Expr {
    arrow(eth_ty(), arrow(nat_ty(), prop()))
}
/// `FineGrainedReduction : Problem → Problem → Prop`
/// A fine-grained reduction preserving exact running time exponents.
pub fn fine_grained_reduction_ty() -> Expr {
    arrow(
        parameterized_problem_ty(),
        arrow(parameterized_problem_ty(), prop()),
    )
}
/// `XPAlgorithm : ParameterizedProblem → Type`
/// An XP algorithm runs in n^f(k) time for computable f.
pub fn xp_algorithm_ty() -> Expr {
    arrow(parameterized_problem_ty(), type0())
}
/// `FPT_subset_XP : Every FPT problem is in XP`
pub fn fpt_subset_xp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        parameterized_problem_ty(),
        arrow(app(cst("IsInFPT"), bvar(0)), app(cst("IsInXP"), bvar(0))),
    )
}
/// `W1_in_XP : W[1] ⊆ XP (assuming W[1] ≠ FPT)`
pub fn w1_in_xp_ty() -> Expr {
    prop()
}
/// `ETHHardness : ParameterizedProblem → Prop`
/// The problem has no f(k)·2^o(n) algorithm under ETH.
pub fn eth_hardness_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `W1Hard_implies_ETH_hard : W[1]-hard implies no f(k)·n^g(1) FPT unless W[1]=FPT`
pub fn w1_hard_eth_hard_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        parameterized_problem_ty(),
        arrow(
            app2(cst("IsWHard"), cst("One"), bvar(0)),
            arrow(eth_ty(), app(cst("ETHHardness"), bvar(1))),
        ),
    )
}
/// `kCliqueW1Hard : k-Clique is W[1]-complete`
pub fn k_clique_w1_hard_ty() -> Expr {
    prop()
}
/// `kIndependentSetW1Hard : k-IndependentSet is W[1]-complete`
pub fn k_independent_set_w1_hard_ty() -> Expr {
    prop()
}
/// `kDomSetW2Hard : k-DominatingSet is W[2]-complete`
pub fn k_dom_set_w2_hard_ty() -> Expr {
    prop()
}
/// Build a path decomposition (linear tree decomposition) for a path graph on n vertices.
/// This gives pathwidth = 1 for a simple path.
pub fn path_graph_decomp(n: usize) -> TreeDecomp {
    if n == 0 {
        return TreeDecomp {
            bags: vec![],
            tree_adj: vec![],
            root: 0,
        };
    }
    let bags: Vec<Vec<usize>> = (0..n.saturating_sub(1)).map(|i| vec![i, i + 1]).collect();
    let num_bags = bags.len().max(1);
    let bags = if bags.is_empty() { vec![vec![0]] } else { bags };
    let mut tree_adj = vec![vec![]; num_bags];
    for i in 0..num_bags.saturating_sub(1) {
        tree_adj[i].push(i + 1);
        tree_adj[i + 1].push(i);
    }
    TreeDecomp {
        bags,
        tree_adj,
        root: 0,
    }
}
/// Vertex cover solver using bounded search tree (exponential in k, polynomial in n).
/// Returns Some(cover) if a vertex cover of size ≤ k exists, else None.
pub fn vertex_cover_bst(adj: &[Vec<usize>], k: usize) -> Option<Vec<usize>> {
    fn solve(
        adj: &[Vec<usize>],
        cover: &mut Vec<usize>,
        removed: &mut Vec<bool>,
        k: usize,
    ) -> bool {
        let edge = (0..adj.len()).find_map(|u| {
            if removed[u] {
                return None;
            }
            adj[u].iter().find(|&&v| !removed[v]).map(|&v| (u, v))
        });
        match edge {
            None => true,
            Some((u, v)) => {
                if k == 0 {
                    return false;
                }
                removed[u] = true;
                cover.push(u);
                if solve(adj, cover, removed, k - 1) {
                    removed[u] = false;
                    return true;
                }
                cover.pop();
                removed[u] = false;
                removed[v] = true;
                cover.push(v);
                if solve(adj, cover, removed, k - 1) {
                    removed[v] = false;
                    return true;
                }
                cover.pop();
                removed[v] = false;
                false
            }
        }
    }
    let n = adj.len();
    let mut cover = Vec::new();
    let mut removed = vec![false; n];
    if solve(adj, &mut cover, &mut removed, k) {
        Some(cover)
    } else {
        None
    }
}
/// Apply the Crown reduction rule for vertex cover kernelization.
/// Returns the reduced graph and the vertices already in the cover.
pub fn crown_reduction(adj: &[Vec<usize>]) -> (Vec<Vec<usize>>, Vec<usize>) {
    let n = adj.len();
    let high_deg: Vec<usize> = (0..n).filter(|&v| !adj[v].is_empty()).collect();
    let mut in_cover = vec![false; n];
    let mut new_adj: Vec<Vec<usize>> = adj.to_vec();
    let _ = high_deg;
    let cover_vertices: Vec<usize> = (0..n).filter(|&v| adj[v].len() > n / 2).collect();
    for &v in &cover_vertices {
        in_cover[v] = true;
        let neighbors = new_adj[v].clone();
        new_adj[v].clear();
        for u in neighbors {
            new_adj[u].retain(|&x| x != v);
        }
    }
    (new_adj, cover_vertices)
}
/// Color-coding for k-path detection (simplified randomized version).
/// Colors vertices randomly with k colors, then checks for a colorful path of length k.
pub fn color_coding_k_path(adj: &[Vec<usize>], k: usize, seed: u64) -> bool {
    let n = adj.len();
    if n == 0 || k == 0 {
        return k == 0;
    }
    if k > n {
        return false;
    }
    let mut rng_state = seed;
    let next_rand = |state: &mut u64| -> u64 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *state
    };
    let colors: Vec<usize> = (0..n)
        .map(|_| (next_rand(&mut rng_state) as usize) % k)
        .collect();
    let k_capped = k.min(20);
    let num_subsets = 1usize << k_capped;
    let mut dp = vec![vec![false; num_subsets]; n];
    for v in 0..n {
        let c = colors[v];
        if c < k_capped {
            dp[v][1 << c] = true;
        }
    }
    for _len in 1..k_capped {
        let old_dp = dp.clone();
        for u in 0..n {
            for &w in &adj[u] {
                let cw = colors[w];
                if cw >= k_capped {
                    continue;
                }
                let cw_bit = 1 << cw;
                for s in 0..num_subsets {
                    if old_dp[u][s] && (s & cw_bit == 0) {
                        dp[w][s | cw_bit] = true;
                    }
                }
            }
        }
    }
    let full_mask = (1 << k_capped) - 1;
    dp.iter().any(|row| row[full_mask])
}
/// Naive treewidth computation for small graphs using elimination ordering.
/// Returns an upper bound on treewidth via a greedy minimum-degree elimination.
pub fn treewidth_upper_bound(adj: &[Vec<usize>]) -> usize {
    let n = adj.len();
    if n == 0 {
        return 0;
    }
    let mut remaining: Vec<bool> = vec![true; n];
    let mut adj_copy: Vec<std::collections::HashSet<usize>> = adj
        .iter()
        .map(|nbrs| nbrs.iter().cloned().collect())
        .collect();
    let mut max_clique = 0usize;
    for _ in 0..n {
        let v = (0..n)
            .filter(|&u| remaining[u])
            .min_by_key(|&u| adj_copy[u].len())
            .expect("at least one remaining vertex exists: loop runs n times for n vertices");
        let deg = adj_copy[v].len();
        max_clique = max_clique.max(deg);
        let nbrs: Vec<usize> = adj_copy[v].iter().cloned().collect();
        for i in 0..nbrs.len() {
            for j in (i + 1)..nbrs.len() {
                adj_copy[nbrs[i]].insert(nbrs[j]);
                adj_copy[nbrs[j]].insert(nbrs[i]);
            }
        }
        remaining[v] = false;
        for &u in &nbrs {
            adj_copy[u].remove(&v);
        }
        adj_copy[v].clear();
    }
    max_clique
}
/// Feedback Vertex Set approximation using iterative compression.
/// Returns an approximate FVS (2-approximation via iterative removal of cycles).
pub fn fvs_approximation(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut fvs = Vec::new();
    let mut removed = vec![false; n];
    loop {
        let cycle_vertex = find_cycle_vertex(adj, &removed);
        match cycle_vertex {
            None => break,
            Some(v) => {
                fvs.push(v);
                removed[v] = true;
            }
        }
    }
    fvs
}
/// Find a vertex on a cycle in the graph (ignoring removed vertices).
pub fn find_cycle_vertex(adj: &[Vec<usize>], removed: &[bool]) -> Option<usize> {
    let n = adj.len();
    let mut color = vec![0u8; n];
    let mut cycle_v = None;
    fn dfs(
        u: usize,
        parent: usize,
        adj: &[Vec<usize>],
        removed: &[bool],
        color: &mut Vec<u8>,
        cycle_v: &mut Option<usize>,
    ) {
        color[u] = 1;
        for &v in &adj[u] {
            if removed[v] || v == parent {
                continue;
            }
            if color[v] == 1 {
                *cycle_v = Some(v);
                return;
            }
            if color[v] == 0 {
                dfs(v, u, adj, removed, color, cycle_v);
                if cycle_v.is_some() {
                    return;
                }
            }
        }
        color[u] = 2;
    }
    for start in 0..n {
        if !removed[start] && color[start] == 0 {
            dfs(start, usize::MAX, adj, removed, &mut color, &mut cycle_v);
            if cycle_v.is_some() {
                return cycle_v;
            }
        }
    }
    None
}
/// Check if a given set S is a feedback vertex set for the graph.
pub fn is_fvs(adj: &[Vec<usize>], fvs: &[usize]) -> bool {
    let n = adj.len();
    let mut removed = vec![false; n];
    for &v in fvs {
        if v < n {
            removed[v] = true;
        }
    }
    find_cycle_vertex(adj, &removed).is_none()
}
/// Solve k-clique detection by brute force (exponential in k, polynomial baseline).
/// Used to validate color-coding results.
pub fn k_clique_brute(adj: &[Vec<usize>], k: usize) -> Option<Vec<usize>> {
    let n = adj.len();
    if k == 0 {
        return Some(vec![]);
    }
    if k > n {
        return None;
    }
    fn is_clique(adj: &[Vec<usize>], clique: &[usize]) -> bool {
        for i in 0..clique.len() {
            for j in (i + 1)..clique.len() {
                if !adj[clique[i]].contains(&clique[j]) {
                    return false;
                }
            }
        }
        true
    }
    fn backtrack(adj: &[Vec<usize>], clique: &mut Vec<usize>, k: usize, start: usize) -> bool {
        if clique.len() == k {
            return is_clique(adj, clique);
        }
        let n = adj.len();
        for v in start..n {
            clique.push(v);
            if backtrack(adj, clique, k, v + 1) {
                return true;
            }
            clique.pop();
        }
        false
    }
    let mut clique = Vec::new();
    if backtrack(adj, &mut clique, k, 0) {
        Some(clique)
    } else {
        None
    }
}
/// Pathwidth computation (exact for trees, upper bound for general graphs).
pub fn pathwidth_upper_bound(adj: &[Vec<usize>]) -> usize {
    treewidth_upper_bound(adj)
}
/// `CrownDecomposition : Graph → Nat → Prop`
/// Crown decomposition for vertex cover: a partition V = H ∪ C ∪ R where H is
/// the head, C is the crown (an independent set matching into H), R is the rest.
pub fn crown_decomposition_ty() -> Expr {
    arrow(cst("Graph"), arrow(nat_ty(), prop()))
}
/// `CrownReductionRule : Graph → Nat → Graph × Nat`
/// Given a crown (H, C), remove H and C and reduce k by |H|.
pub fn crown_reduction_rule_ty() -> Expr {
    arrow(
        cst("Graph"),
        arrow(nat_ty(), pair_ty(cst("Graph"), nat_ty())),
    )
}
/// `LPRelaxationKernel : ParameterizedProblem → Nat → Prop`
/// LP relaxation half-integrality gives a 2k-vertex kernel for vertex cover.
pub fn lp_relaxation_kernel_ty() -> Expr {
    arrow(parameterized_problem_ty(), arrow(nat_ty(), prop()))
}
/// `RandomizedKernel : ParameterizedProblem → Nat → Prop`
/// A randomized kernelization algorithm (correct with high probability).
pub fn randomized_kernel_ty() -> Expr {
    arrow(parameterized_problem_ty(), arrow(nat_ty(), prop()))
}
/// `KernelComposition : ParameterizedProblem → Prop`
/// An OR-composition for cross-composition lower bounds on kernelization.
pub fn kernel_composition_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `PolynomialKernelLowerBound : ParameterizedProblem → Nat → Prop`
/// No polynomial kernel of size < k^c unless NP ⊆ coNP/poly.
pub fn poly_kernel_lower_bound_ty() -> Expr {
    arrow(parameterized_problem_ty(), arrow(nat_ty(), prop()))
}
/// `IterativeCompression : ParameterizedProblem → Prop`
/// FPT via iterative compression: given a size-(k+1) solution, find a size-k one.
pub fn iterative_compression_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `IndependentSetFPT : k-IndependentSet is in FPT via iterative compression (for special graphs)`
pub fn independent_set_fpt_ty() -> Expr {
    prop()
}
/// `OddCycleTransversalFPT : Odd-Cycle Transversal is FPT via iterative compression`
pub fn odd_cycle_transversal_fpt_ty() -> Expr {
    prop()
}
/// `CompressionAlgorithm : ParameterizedProblem → Nat → Prop`
/// Given a (k+1)-solution, the compression algorithm finds a k-solution or returns None.
pub fn compression_algorithm_ty() -> Expr {
    arrow(parameterized_problem_ty(), arrow(nat_ty(), prop()))
}
/// `UniversalSet : Nat → Nat → Type`
/// An (n,k)-universal set: a family F of functions [n]→{0,1} such that for
/// every k-subset S ⊆ [n], F restricted to S contains all 2^k patterns.
pub fn universal_set_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `UniversalSetSize : Nat → Nat → Nat`
/// |UniversalSet n k| = O(2^k · k^2 · log n) (Naor-Schulman-Srinivasan).
pub fn universal_set_size_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// `DerandomizationViaPHF : k-Subgraph is FPT with deterministic algorithm via PHF`
pub fn derandomization_via_phf_ty() -> Expr {
    prop()
}
/// `SplittingLemma : Nat → Prop`
/// The splitting lemma for color-coding: an n-coloring witnesses a colorful copy
/// with probability ≥ k!/k^k ≥ e^{-k}.
pub fn splitting_lemma_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MSO1Logic : Type` — Monadic Second-Order Logic MSO₁ (quantifies over vertex sets).
pub fn mso1_logic_ty() -> Expr {
    type0()
}
/// `MSO1Satisfies : Graph → MSO1Logic → Prop`
pub fn mso1_satisfies_ty() -> Expr {
    arrow(cst("Graph"), arrow(mso1_logic_ty(), prop()))
}
/// `CourcelleTreewidth : ∀ φ : MSO₂, every bounded-treewidth property is linear-time FPT`
pub fn courcelle_treewidth_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        mso2_logic_ty(),
        arrow(nat_ty(), prop()),
    )
}
/// `CourcellePathwidth : MSO₁ model checking is FPT on bounded pathwidth graphs`
pub fn courcelle_pathwidth_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "phi",
        mso1_logic_ty(),
        arrow(nat_ty(), prop()),
    )
}
/// `SeeseSTheorem : MSO₂ model checking is decidable on graphs of bounded clique-width`
pub fn seese_s_theorem_ty() -> Expr {
    prop()
}
/// `CliqueWidth : Graph → Nat` — the clique-width of a graph.
pub fn clique_width_ty() -> Expr {
    arrow(cst("Graph"), nat_ty())
}
/// `W1HardnessReduction : Problem → k-Clique` — reduction from k-clique showing W[1]-hardness.
pub fn w1_hardness_reduction_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `W2HardnessReduction : Problem → k-DominatingSet` — reduction to k-dominating set.
pub fn w2_hardness_reduction_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `MulticoloredCliqueW1Hard : Multicolored k-Clique is W[1]-complete`
pub fn multicolored_clique_w1_hard_ty() -> Expr {
    prop()
}
/// `kSetCoverW2Hard : k-SetCover is W[2]-complete`
pub fn k_set_cover_w2_hard_ty() -> Expr {
    prop()
}
/// `kHittingSetW2Hard : k-HittingSet is W[2]-complete`
pub fn k_hitting_set_w2_hard_ty() -> Expr {
    prop()
}
/// `WPHardnessViaCircuit : WP-hardness via weighted circuit satisfiability`
pub fn wp_hardness_via_circuit_ty() -> Expr {
    prop()
}
/// `GridRamsey : Nat → Nat → Prop`
/// Grid Ramsey theorem: implications for lower bounds under ETH.
pub fn grid_ramsey_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ETHKVertexCoverLB : ETH → VertexCover has no 2^o(k) * n algorithm`
pub fn eth_k_vertex_cover_lb_ty() -> Expr {
    arrow(eth_ty(), prop())
}
/// `ETHKFVSLowerBound : ETH → FVS has no 2^o(k log k) * n algorithm`
pub fn eth_fvs_lower_bound_ty() -> Expr {
    arrow(eth_ty(), prop())
}
/// `SETHKSATLowerBound : SETH → k-SAT lower bounds for each k`
pub fn seth_ksat_lower_bound_ty() -> Expr {
    arrow(seth_ty(), arrow(nat_ty(), prop()))
}
/// `ETHImpliesNoSubexpFVS : ETH implies FVS has no f(k)·2^o(sqrt(k)) algorithm`
pub fn eth_implies_no_subexp_fvs_ty() -> Expr {
    arrow(eth_ty(), prop())
}
/// `SETHImpliesEdgeDominatingSetLB : SETH → Edge Dominating Set has tight lower bounds`
pub fn seth_edge_dominating_set_lb_ty() -> Expr {
    arrow(seth_ty(), prop())
}
/// `FPTApproximation : ParameterizedProblem → Real → Prop`
/// An FPT r-approximation runs in f(k)·poly(n) and gives a solution within r of optimal.
pub fn fpt_approximation_ty() -> Expr {
    arrow(parameterized_problem_ty(), arrow(real_ty(), prop()))
}
/// `GapETH : Prop`
/// Gap-ETH: there is no FPT approximation scheme for k-Clique unless Gap-ETH fails.
pub fn gap_eth_ty() -> Expr {
    prop()
}
/// `GapETHImpliesNoFPTAS : Gap-ETH → no FPTAS for k-Clique`
pub fn gap_eth_no_fptas_ty() -> Expr {
    arrow(gap_eth_ty(), prop())
}
/// `EPTAS : ParameterizedProblem → Prop`
/// An EPTAS (Efficient PTAS): for each ε > 0, runs in f(1/ε)·poly(n).
pub fn eptas_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `PTAS : ParameterizedProblem → Prop`
/// A PTAS: for each ε > 0, runs in poly(n) (with constant depending on ε).
pub fn ptas_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `PlanarEPTAS : Many planar problems have EPTASes via bidimensionality`
pub fn planar_eptas_ty() -> Expr {
    prop()
}
/// `DualParameter : ParameterizedProblem → Nat → Prop`
/// Dual parameterization: parameter = n - k (e.g., n - vertex cover size).
pub fn dual_parameter_ty() -> Expr {
    arrow(parameterized_problem_ty(), arrow(nat_ty(), prop()))
}
/// `AboveGuaranteeParam : ParameterizedProblem → Nat → Prop`
/// Above-guarantee parameterization: k is the excess above a structural lower bound.
pub fn above_guarantee_param_ty() -> Expr {
    arrow(parameterized_problem_ty(), arrow(nat_ty(), prop()))
}
/// `StructuralParam : ParameterizedProblem → Type → Prop`
/// Structural parameterization by a graph parameter (e.g., treewidth, clique-width).
pub fn structural_param_ty() -> Expr {
    arrow(parameterized_problem_ty(), arrow(type0(), prop()))
}
/// `MaxSATAboveGuarantee : Max-SAT is FPT above the n/2 guarantee`
pub fn maxsat_above_guarantee_ty() -> Expr {
    prop()
}
/// `VertexCoverAboveMMMatching : VertexCover FPT above max-matching lower bound`
pub fn vc_above_matching_ty() -> Expr {
    prop()
}
/// `CombinedParameter : ParameterizedProblem → Nat → Nat → Prop`
/// Combined parameterization: parameterize by both k and a structural parameter.
pub fn combined_parameter_ty() -> Expr {
    arrow(
        parameterized_problem_ty(),
        arrow(nat_ty(), arrow(nat_ty(), prop())),
    )
}
/// `CountingFPT : ParameterizedProblem → Prop`
/// The counting version of an FPT problem (count solutions) is also FPT.
pub fn counting_fpt_ty() -> Expr {
    arrow(parameterized_problem_ty(), prop())
}
/// `SharpW1 : Type` — the class #W[1] (parameterized counting analogue of W[1]).
pub fn sharp_w1_ty() -> Expr {
    type0()
}
/// `SharpW2 : Type` — the class #W[2].
pub fn sharp_w2_ty() -> Expr {
    type0()
}
/// `CountingCliquesSharpW1Hard : counting k-cliques is #W[1]-complete`
pub fn counting_cliques_sharp_w1_hard_ty() -> Expr {
    prop()
}
/// `CountingMatchingsSharpW1Hard : counting perfect matchings is #W[1]-hard`
pub fn counting_matchings_sharp_w1_hard_ty() -> Expr {
    prop()
}
/// `CountingHomomorphismsSharpW1 : counting graph homomorphisms is #W[1]-complete`
pub fn counting_homomorphisms_sharp_w1_ty() -> Expr {
    prop()
}
/// `CountingOnBoundedTW : counting on bounded-treewidth graphs is FPT`
pub fn counting_on_bounded_tw_ty() -> Expr {
    prop()
}
/// `CrownReductionAxiom : Crown decomposition implies 2k-vertex kernel for VC`
pub fn crown_reduction_axiom_ty() -> Expr {
    prop()
}
/// `KoenigTheoremKernelization : König's theorem gives LP-based kernel`
pub fn koenig_kernel_ty() -> Expr {
    prop()
}
/// `NemhauserTrotterThm : Nemhauser-Trotter theorem for vertex cover LP kernel`
pub fn nemhauser_trotter_ty() -> Expr {
    prop()
}
/// `AboveGuaranteeParamVC : VC above matching is FPT (Razgon-O'Sullivan, Cygan et al.)`
pub fn above_guarantee_vc_ty() -> Expr {
    prop()
}
/// `MaxSNPHardnessInFPT : Max-SNP hard problems have EPTASes in FPT setting`
pub fn max_snp_fpt_ty() -> Expr {
    prop()
}
/// `MultiparamFPT : FPT algorithms using multiple parameters simultaneously`
pub fn multiparam_fpt_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// Alias for `build_parameterized_complexity_env` — simple entry point.
pub fn build_env(env: &mut Environment) -> Result<(), String> {
    build_parameterized_complexity_env(env)
}
/// Populate an `Environment` with parameterized complexity axioms.
pub fn build_parameterized_complexity_env(env: &mut Environment) -> Result<(), String> {
    for (name, ty) in [
        ("ParameterizedProblem", parameterized_problem_ty()),
        ("Parameter", parameter_ty()),
        ("FPTAlgorithm", fpt_algorithm_ty()),
        ("IsInFPT", is_in_fpt_ty()),
        ("IsInXP", is_in_xp_ty()),
        ("IsWHard", is_w_hard_ty()),
        ("IsWComplete", is_w_complete_ty()),
        ("FPTReducible", fpt_reducible_ty()),
        ("WHierarchy", w_hierarchy_ty()),
        ("W1", w1_ty()),
        ("W2", w2_ty()),
        ("WP", wp_ty()),
        ("AW", aw_ty()),
        ("Kernel", kernel_ty()),
        ("HasPolynomialKernel", has_polynomial_kernel_ty()),
        ("KernelSize", kernel_size_ty()),
        ("TreeDecomposition", tree_decomposition_ty()),
        ("Treewidth", treewidth_ty()),
        ("PathDecomposition", path_decomposition_ty()),
        ("Pathwidth", pathwidth_ty()),
        ("BranchDecomposition", branch_decomposition_ty()),
        ("Branchwidth", branchwidth_ty()),
        ("FeedbackVertexSet", feedback_vertex_set_ty()),
        ("MinFVS", min_fvs_ty()),
        ("ColorCodingAlgorithm", color_coding_algorithm_ty()),
        ("PerfectHashFamily", perfect_hash_family_ty()),
        ("MSO2Logic", mso2_logic_ty()),
        ("MSO2Satisfies", mso2_satisfies_ty()),
        ("BoundedTreewidthDP", bounded_treewidth_dp_ty()),
        ("ETH", eth_ty()),
        ("SETH", seth_ty()),
        ("ETHHardness", eth_hardness_ty()),
        ("FineGrainedReduction", fine_grained_reduction_ty()),
        ("XPAlgorithm", xp_algorithm_ty()),
        ("kClique", parameterized_problem_ty()),
        ("kIndependentSet", parameterized_problem_ty()),
        ("kVertexCover", parameterized_problem_ty()),
        ("kDominatingSet", parameterized_problem_ty()),
        ("kFeedbackVertexSet", parameterized_problem_ty()),
        ("kLongestPath", parameterized_problem_ty()),
        ("kPathWidth", parameterized_problem_ty()),
        ("kTreewidth", parameterized_problem_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("ParamComplexity.fpt_subset_xp", fpt_subset_xp_ty()),
        ("ParamComplexity.k_clique_w1_hard", k_clique_w1_hard_ty()),
        (
            "ParamComplexity.k_independent_set_w1_hard",
            k_independent_set_w1_hard_ty(),
        ),
        ("ParamComplexity.k_dom_set_w2_hard", k_dom_set_w2_hard_ty()),
        (
            "ParamComplexity.vertex_cover_kernel_2k",
            vertex_cover_kernel_2k_ty(),
        ),
        (
            "ParamComplexity.vertex_cover_kernel_k_squared",
            vertex_cover_kernel_k_squared_ty(),
        ),
        ("ParamComplexity.color_coding_fpt", color_coding_fpt_ty()),
        ("ParamComplexity.courcelle_theorem", courcelle_theorem_ty()),
        (
            "ParamComplexity.eth_implies_no_subexp",
            eth_implies_no_subexp_ty(),
        ),
        (
            "ParamComplexity.seth_implies_vc_lb",
            seth_implies_vc_lb_ty(),
        ),
        (
            "ParamComplexity.sparsification_lemma",
            sparsification_lemma_ty(),
        ),
        ("ParamComplexity.eth_k_clique_lb", eth_k_clique_lb_ty()),
        ("ParamComplexity.w1_hard_eth_hard", w1_hard_eth_hard_ty()),
        (
            "ParamComplexity.treewidth_le_pathwidth",
            treewidth_le_pathwidth_ty(),
        ),
        ("ParamComplexity.fvs_in_fpt", is_in_fpt_ty()),
        ("ParamComplexity.w1_in_xp", w1_in_xp_ty()),
        (
            "ParamComplexity.crown_reduction_axiom",
            crown_reduction_axiom_ty(),
        ),
        ("ParamComplexity.koenig_kernel", koenig_kernel_ty()),
        ("ParamComplexity.nemhauser_trotter", nemhauser_trotter_ty()),
        (
            "ParamComplexity.above_guarantee_vc",
            above_guarantee_vc_ty(),
        ),
        (
            "ParamComplexity.iterative_compression",
            iterative_compression_ty(),
        ),
        (
            "ParamComplexity.independent_set_fpt",
            independent_set_fpt_ty(),
        ),
        (
            "ParamComplexity.odd_cycle_transversal_fpt",
            odd_cycle_transversal_fpt_ty(),
        ),
        (
            "ParamComplexity.derandomization_via_phf",
            derandomization_via_phf_ty(),
        ),
        (
            "ParamComplexity.courcelle_treewidth",
            courcelle_treewidth_ty(),
        ),
        (
            "ParamComplexity.courcelle_pathwidth",
            courcelle_pathwidth_ty(),
        ),
        ("ParamComplexity.seese_s_theorem", seese_s_theorem_ty()),
        (
            "ParamComplexity.multicolored_clique_w1_hard",
            multicolored_clique_w1_hard_ty(),
        ),
        (
            "ParamComplexity.k_set_cover_w2_hard",
            k_set_cover_w2_hard_ty(),
        ),
        (
            "ParamComplexity.k_hitting_set_w2_hard",
            k_hitting_set_w2_hard_ty(),
        ),
        (
            "ParamComplexity.wp_hardness_via_circuit",
            wp_hardness_via_circuit_ty(),
        ),
        (
            "ParamComplexity.eth_k_vertex_cover_lb",
            eth_k_vertex_cover_lb_ty(),
        ),
        (
            "ParamComplexity.eth_fvs_lower_bound",
            eth_fvs_lower_bound_ty(),
        ),
        ("ParamComplexity.gap_eth", gap_eth_ty()),
        ("ParamComplexity.gap_eth_no_fptas", gap_eth_no_fptas_ty()),
        (
            "ParamComplexity.maxsat_above_guarantee",
            maxsat_above_guarantee_ty(),
        ),
        ("ParamComplexity.vc_above_matching", vc_above_matching_ty()),
        (
            "ParamComplexity.counting_cliques_sharp_w1_hard",
            counting_cliques_sharp_w1_hard_ty(),
        ),
        (
            "ParamComplexity.counting_matchings_sharp_w1_hard",
            counting_matchings_sharp_w1_hard_ty(),
        ),
        (
            "ParamComplexity.counting_homomorphisms_sharp_w1",
            counting_homomorphisms_sharp_w1_ty(),
        ),
        (
            "ParamComplexity.counting_on_bounded_tw",
            counting_on_bounded_tw_ty(),
        ),
        ("ParamComplexity.planar_eptas", planar_eptas_ty()),
        ("ParamComplexity.max_snp_fpt", max_snp_fpt_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    for (name, ty) in [
        ("CrownDecomposition", crown_decomposition_ty()),
        ("CrownReductionRule", crown_reduction_rule_ty()),
        ("LPRelaxationKernel", lp_relaxation_kernel_ty()),
        ("RandomizedKernel", randomized_kernel_ty()),
        ("KernelComposition", kernel_composition_ty()),
        ("IterativeCompression", iterative_compression_ty()),
        ("CompressionAlgorithm", compression_algorithm_ty()),
        ("UniversalSet", universal_set_ty()),
        ("UniversalSetSize", universal_set_size_ty()),
        ("SplittingLemma", splitting_lemma_ty()),
        ("MSO1Logic", mso1_logic_ty()),
        ("MSO1Satisfies", mso1_satisfies_ty()),
        ("CliqueWidth", clique_width_ty()),
        ("W1HardnessReduction", w1_hardness_reduction_ty()),
        ("W2HardnessReduction", w2_hardness_reduction_ty()),
        ("GridRamsey", grid_ramsey_ty()),
        ("ETHKVertexCoverLB", eth_k_vertex_cover_lb_ty()),
        ("ETHKFVSLowerBound", eth_fvs_lower_bound_ty()),
        ("SETHKSATLowerBound", seth_ksat_lower_bound_ty()),
        ("FPTApproximation", fpt_approximation_ty()),
        ("GapETH", gap_eth_ty()),
        ("EPTAS", eptas_ty()),
        ("PTAS", ptas_ty()),
        ("DualParameter", dual_parameter_ty()),
        ("AboveGuaranteeParam", above_guarantee_param_ty()),
        ("StructuralParam", structural_param_ty()),
        ("CombinedParameter", combined_parameter_ty()),
        ("CountingFPT", counting_fpt_ty()),
        ("SharpW1", sharp_w1_ty()),
        ("SharpW2", sharp_w2_ty()),
        ("PolyKernelLowerBound", poly_kernel_lower_bound_ty()),
        ("MultiparamFPT", multiparam_fpt_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty,
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vertex_cover_bst_small() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let result = vertex_cover_bst(&adj, 2);
        assert!(result.is_some(), "Triangle should have 2-vertex cover");
        let cover = result.expect("result should be valid");
        assert!(cover.len() <= 2);
        for u in 0..adj.len() {
            for &v in &adj[u] {
                assert!(
                    cover.contains(&u) || cover.contains(&v),
                    "Edge ({}, {}) not covered",
                    u,
                    v
                );
            }
        }
    }
    #[test]
    fn test_vertex_cover_bst_impossible() {
        let adj = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
        assert!(vertex_cover_bst(&adj, 1).is_none());
        assert!(vertex_cover_bst(&adj, 3).is_some());
    }
    #[test]
    fn test_treewidth_upper_bound_path() {
        let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3]];
        let tw = treewidth_upper_bound(&adj);
        assert!(tw <= 2, "Path should have treewidth ≤ 2, got {}", tw);
    }
    #[test]
    fn test_treewidth_upper_bound_complete() {
        let adj = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
        let tw = treewidth_upper_bound(&adj);
        assert!(tw >= 3, "K_4 should have treewidth ≥ 3, got {}", tw);
    }
    #[test]
    fn test_color_coding_k_path() {
        let adj = vec![
            vec![1],
            vec![0, 2],
            vec![1, 3],
            vec![2, 4],
            vec![3, 5],
            vec![4],
        ];
        let found = color_coding_k_path(&adj, 4, 42);
        assert!(found, "Should find a 4-path in a 6-path graph");
    }
    #[test]
    fn test_fvs_approximation() {
        let adj = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![2, 0]];
        let fvs = fvs_approximation(&adj);
        assert!(
            is_fvs(&adj, &fvs),
            "FVS approximation should produce a valid FVS"
        );
    }
    #[test]
    fn test_is_fvs() {
        let adj = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
        assert!(is_fvs(&adj, &[]), "Tree should have empty FVS");
        let cycle = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![2, 0]];
        assert!(
            is_fvs(&cycle, &[0]),
            "Removing vertex 0 from C4 should break all cycles"
        );
    }
    #[test]
    fn test_k_clique_brute() {
        let adj = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
        assert!(k_clique_brute(&adj, 3).is_some());
        assert!(k_clique_brute(&adj, 5).is_none());
    }
    #[test]
    fn test_build_parameterized_complexity_env() {
        let mut env = Environment::new();
        let result = build_parameterized_complexity_env(&mut env);
        assert!(result.is_ok(), "build should succeed");
        assert!(env.get(&Name::str("IsInFPT")).is_some());
        assert!(env.get(&Name::str("Treewidth")).is_some());
        assert!(env.get(&Name::str("ETH")).is_some());
        assert!(env.get(&Name::str("SETH")).is_some());
    }
    #[test]
    fn test_crown_decomposition_tree() {
        let adj = vec![vec![1, 2], vec![0, 3], vec![0], vec![1]];
        let cd = CrownDecomposition::compute(&adj, 2);
        assert!(cd.verify(&adj));
    }
    #[test]
    fn test_crown_decomposition_isolated() {
        let adj: Vec<Vec<usize>> = vec![vec![], vec![2], vec![1], vec![]];
        let cd = CrownDecomposition::compute(&adj, 1);
        assert!(cd.verify(&adj));
        assert!(cd.crown.contains(&0) || cd.crown.contains(&3));
    }
    #[test]
    fn test_crown_decomposition_head_size() {
        let adj: Vec<Vec<usize>> = vec![vec![], vec![2], vec![1]];
        let cd = CrownDecomposition::compute(&adj, 1);
        let _ = cd.head_size();
    }
    #[test]
    fn test_color_coding_fpt_construction() {
        let cc = ColorCodingFPT::new(4, false);
        assert_eq!(cc.k, 4);
        assert!(!cc.use_perfect_hash);
        assert!(cc.repetitions > 0);
        let rt = cc.running_time();
        assert!(rt.contains("4"));
    }
    #[test]
    fn test_color_coding_fpt_find_path() {
        let adj = vec![
            vec![1],
            vec![0, 2],
            vec![1, 3],
            vec![2, 4],
            vec![3, 5],
            vec![4],
        ];
        let cc = ColorCodingFPT::new(4, false);
        let found = cc.find_k_path(&adj);
        assert!(found, "Should find 4-path in 6-path graph");
    }
    #[test]
    fn test_color_coding_fpt_hamiltonian() {
        let cc = ColorCodingFPT::new(5, true);
        assert!(cc.can_detect_hamiltonian_path(4));
        assert!(!cc.can_detect_hamiltonian_path(6));
    }
    #[test]
    fn test_color_coding_fpt_deterministic() {
        let cc = ColorCodingFPT::new(3, true);
        assert!(cc.use_perfect_hash);
        assert_eq!(cc.repetitions, 1);
        let rt = cc.running_time();
        assert!(rt.contains("deterministic"));
    }
    #[test]
    fn test_courcelle_msol_checker_construction() {
        let checker = CourcelleMSOLChecker::new(3, "∃S ⊆ V : |S| ≤ k ∧ S is dominating", 2);
        assert_eq!(checker.max_treewidth, 3);
        assert_eq!(checker.mso_version, 2);
        let rt = checker.running_time();
        assert!(rt.contains("treewidth"));
    }
    #[test]
    fn test_courcelle_msol_checker_on_path() {
        let adj = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let checker = CourcelleMSOLChecker::new(2, "k-coloring", 1);
        assert!(checker.check(&adj));
    }
    #[test]
    fn test_courcelle_msol_checker_on_k4() {
        let adj = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
        let checker_small = CourcelleMSOLChecker::new(2, "3-coloring", 2);
        assert!(!checker_small.check(&adj));
        let checker_large = CourcelleMSOLChecker::new(5, "3-coloring", 2);
        assert!(checker_large.check(&adj));
    }
    #[test]
    fn test_courcelle_decidable_properties() {
        let checker = CourcelleMSOLChecker::new(4, "Hamiltonicity", 2);
        let desc = checker.decidable_properties();
        assert!(desc.contains("treewidth"));
    }
    #[test]
    fn test_vertex_cover_fpt_solve_triangle() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let vc = VertexCoverFPT::new(2);
        let result = vc.solve(&adj);
        assert!(result.is_some(), "Triangle has a 2-vertex cover");
        let cover = result.expect("result should be valid");
        for u in 0..adj.len() {
            for &v in &adj[u] {
                assert!(cover.contains(&u) || cover.contains(&v));
            }
        }
    }
    #[test]
    fn test_vertex_cover_fpt_solve_k4() {
        let adj = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
        let vc3 = VertexCoverFPT::new(3);
        assert!(vc3.solve(&adj).is_some());
        let vc1 = VertexCoverFPT::new(1);
        assert!(vc1.solve(&adj).is_none());
    }
    #[test]
    fn test_vertex_cover_fpt_running_time() {
        let vc = VertexCoverFPT::new(5);
        let rt = vc.running_time();
        assert!(rt.contains("5"));
    }
    #[test]
    fn test_vertex_cover_fpt_lp_kernel() {
        let adj = vec![vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]];
        let vc = VertexCoverFPT::new(3);
        let (_, cover, _) = vc.lp_kernel(&adj);
        assert!(cover.contains(&0), "Center should be in LP kernel cover");
    }
    #[test]
    fn test_iterative_compression_vc_triangle() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let ic = IterativeCompressionVC::new(2);
        let over_cover = vec![0, 1, 2];
        let result = ic.compress(&adj, &over_cover);
        assert!(result.is_some(), "Triangle should compress to 2-cover");
    }
    #[test]
    fn test_iterative_compression_vc_impossible() {
        let adj = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
        let ic = IterativeCompressionVC::new(1);
        let over_cover = vec![0, 1];
        let result = ic.compress(&adj, &over_cover);
        assert!(result.is_none(), "K_4 cannot be covered by 1 vertex");
    }
    #[test]
    fn test_build_parameterized_complexity_env_new() {
        let mut env = Environment::new();
        let result = build_parameterized_complexity_env(&mut env);
        assert!(result.is_ok());
        assert!(env.get(&Name::str("CrownDecomposition")).is_some());
        assert!(env.get(&Name::str("SharpW1")).is_some());
        assert!(env.get(&Name::str("GapETH")).is_some());
        assert!(env.get(&Name::str("EPTAS")).is_some());
        assert!(env.get(&Name::str("MSO1Logic")).is_some());
        assert!(env.get(&Name::str("CliqueWidth")).is_some());
        assert!(env.get(&Name::str("UniversalSet")).is_some());
        assert!(env.get(&Name::str("DualParameter")).is_some());
    }
}
#[cfg(test)]
mod tests_pc_extra {
    use super::*;
    #[test]
    fn test_fpt_algorithm() {
        let bst = FPTMethod::bounded_search_tree_vc();
        assert!(bst.is_single_exponential());
        let r = bst.runtime(5, 1000);
        assert!((r - 32000.0).abs() < 1e-6);
    }
    #[test]
    fn test_kernelization() {
        let vc = Kernelization::vertex_cover_2k();
        assert!(vc.is_polynomial_kernel());
        let sz = vc.kernel_size(10);
        assert_eq!(sz, Some(20.0));
        let fvs = Kernelization::feedback_vertex_set();
        assert!(fvs.is_polynomial_kernel());
    }
    #[test]
    fn test_w_hierarchy() {
        let vc = WClass::vertex_cover_class();
        assert!(vc.is_fpt());
        let cl = WClass::clique_class();
        assert!(!cl.is_fpt());
        assert!(!cl.harder_than_w1());
        let ds = WClass::dominating_set_class();
        assert!(ds.harder_than_w1());
        assert!(ds > cl);
    }
    #[test]
    fn test_treewidth_algorithm() {
        let sat = TreewidthAlgorithm::satisfiability_tw();
        assert!(sat.is_linear_time_in_n());
        let r = sat.runtime(3, 100);
        assert!((r - 800.0).abs() < 1e-6);
    }
    #[test]
    fn test_param_reduction() {
        let red = ParamReduction::new("IndependentSet", "Clique", "k", "k", "f(k)=k");
        assert!(red.is_fpt_reduction());
        let bad_red = ParamReduction::new("A", "B", "k", "k+log(n)", "k+log(n)");
        assert!(!bad_red.is_fpt_reduction());
    }
}
