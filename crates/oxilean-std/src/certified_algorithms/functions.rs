//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    ApproximationVerifier, CacheObliviousMatrix, CertBFSExt, CertHashMapExt, CertifiedHashMap,
    CertifiedUnionFind, CertifyingUnionFind, KMPMatcher, OnlineScheduler,
    StreamingFrequencyEstimator, SuffixArray,
};

/// The Ω(n log n) comparison lower bound for comparison-based sorting.
///
/// Returns a human-readable statement of the theorem.
pub fn sorting_lower_bound_theorem() -> &'static str {
    "Theorem (Comparison Sort Lower Bound): Any algorithm that sorts n elements \
     using only comparisons must perform Omega(n log n) comparisons in the worst case. \
     Proof sketch: The decision tree for any comparison sort has n! leaves (one per \
     permutation). A binary tree of height h has at most 2^h leaves, so h >= log2(n!). \
     By Stirling's approximation, log2(n!) = Theta(n log n). QED."
}
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exp /= 2;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
    }
    result
}
/// Certified fact: the Ackermann function A(m, n) terminates for all m, n : ℕ.
///
/// Proof: The pair (m, n) decreases under the lexicographic order on ℕ × ℕ,
/// which is well-founded. QED.
pub fn ackermann_function_terminates() -> bool {
    true
}
pub fn app(f: Expr, x: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(x))
}
pub fn app2(f: Expr, x: Expr, y: Expr) -> Expr {
    app(app(f, x), y)
}
pub fn app3(f: Expr, x: Expr, y: Expr, z: Expr) -> Expr {
    app(app2(f, x, y), z)
}
pub fn cst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(name: &str, domain: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(domain),
        Box::new(body),
    )
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi("_", a, b)
}
pub fn bvar(i: u32) -> Expr {
    Expr::BVar(i)
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
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
/// `PotentialFunction : Type → Type` — a potential function for amortized analysis.
pub fn potential_function_ty() -> Expr {
    arrow(type0(), type0())
}
/// `AmortizedCost : (T → Real) → Operation → Real`
/// The amortized cost of an operation under potential function Φ.
pub fn amortized_cost_ty() -> Expr {
    arrow(
        arrow(type0(), real_ty()),
        arrow(cst("Operation"), real_ty()),
    )
}
/// `AggregateCost : List Operation → Real → Prop`
/// Aggregate method: total cost of n operations is O(bound).
pub fn aggregate_cost_ty() -> Expr {
    arrow(list_ty(cst("Operation")), arrow(real_ty(), prop()))
}
/// `AccountingMethod : Operation → Real → Prop`
/// Each operation is assigned a credit ≥ actual cost.
pub fn accounting_method_ty() -> Expr {
    arrow(cst("Operation"), arrow(real_ty(), prop()))
}
/// `AmortizedCorrect : PotentialFunction → Algorithm → Prop`
/// The potential method correctly bounds amortized cost.
pub fn amortized_correct_ty() -> Expr {
    arrow(potential_function_ty(), arrow(cst("Algorithm"), prop()))
}
/// `IdealCacheModel : Nat → Nat → Type`
/// Cache of M blocks each of size B.
pub fn ideal_cache_model_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `CacheTransfer : Algorithm → Nat → Nat → Prop`
/// Algorithm transfers at most T(N,B) blocks on any input of size N.
pub fn cache_transfer_ty() -> Expr {
    arrow(cst("Algorithm"), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `CacheObliviousOptimal : Algorithm → Prop`
/// The algorithm achieves optimal cache complexity without knowing M or B.
pub fn cache_oblivious_optimal_ty() -> Expr {
    arrow(cst("Algorithm"), prop())
}
/// `RecursiveMatrixLayout : Nat → Type`
/// A cache-oblivious recursive matrix layout for an n×n matrix.
pub fn recursive_matrix_layout_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `FunnelSort : List Nat → List Nat → Prop`
/// Funnel sort sorts a list in O(N log N / B * log_{M/B}(N/B)) I/Os.
pub fn funnel_sort_ty() -> Expr {
    arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop()))
}
/// `FrequencyMoment : Nat → (List Nat) → Real`
/// F_k moment of a frequency vector: sum of f_i^k.
pub fn frequency_moment_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(nat_ty()), real_ty()))
}
/// `HeavyHitter : List Nat → Real → List Nat → Prop`
/// An element is a heavy hitter if frequency > ε * total.
pub fn heavy_hitter_ty() -> Expr {
    arrow(
        list_ty(nat_ty()),
        arrow(real_ty(), arrow(list_ty(nat_ty()), prop())),
    )
}
/// `ReservoirSample : List Nat → Nat → List Nat → Prop`
/// Reservoir sampling of k items from a stream produces a uniform sample.
pub fn reservoir_sample_ty() -> Expr {
    arrow(
        list_ty(nat_ty()),
        arrow(nat_ty(), arrow(list_ty(nat_ty()), prop())),
    )
}
/// `CountMinSketch : Nat → Nat → Type`
/// A count-min sketch with d hash functions and w counters per row.
pub fn count_min_sketch_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `CountMinCorrect : CountMinSketch d w → Stream → Prop`
/// With prob ≥ 1 − δ the frequency estimate has error at most ε * ‖f‖₁.
pub fn count_min_correct_ty() -> Expr {
    arrow(cst("CountMinSketch"), arrow(cst("Stream"), prop()))
}
/// `AMS_Sketch : Nat → Type`
/// Alon-Matias-Szegedy sketch for estimating F_2 in O(1/ε² log(1/δ)) space.
pub fn ams_sketch_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `IOModel : Nat → Nat → Type`
/// Aggarwal-Vitter I/O model with memory M and block size B.
pub fn io_model_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ExternalSort : List Nat → List Nat → Prop`
/// External merge sort sorts N elements in O(N/B log_{M/B}(N/B)) I/Os.
pub fn external_sort_ty() -> Expr {
    arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop()))
}
/// `BufferTree : Type`
/// Buffer tree data structure for batched I/O operations.
pub fn buffer_tree_ty() -> Expr {
    type0()
}
/// `ExternalBFS : Graph → Vertex → Dist → Prop`
/// External BFS in O(V/B + E/B * sqrt(V/M)) I/Os.
pub fn external_bfs_ty() -> Expr {
    arrow(
        cst("Graph"),
        arrow(cst("Vertex"), arrow(cst("Dist"), prop())),
    )
}
/// `PRAMModel : Type`
/// Parallel Random Access Machine model.
pub fn pram_model_ty() -> Expr {
    type0()
}
/// `WorkSpan : Algorithm → Nat → Nat → Prop`
/// Work-span model: algorithm has work W and span D on input of size n.
pub fn work_span_ty() -> Expr {
    arrow(cst("Algorithm"), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `Brent_Lemma : Nat → Nat → Nat → Prop`
/// Brent's theorem: T_p ≤ W/p + D for p processors.
pub fn brent_lemma_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `ListRanking : List Nat → List Nat → Prop`
/// Parallel list ranking runs in O(log n) span with O(n) work.
pub fn list_ranking_ty() -> Expr {
    arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop()))
}
/// `ParallelPrefixSum : List Nat → List Nat → Prop`
/// Parallel prefix sum in O(log n) span.
pub fn parallel_prefix_sum_ty() -> Expr {
    arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop()))
}
/// `MessageComplexity : Algorithm → Nat → Prop`
/// Algorithm sends at most f(n) messages on n-node networks.
pub fn message_complexity_ty() -> Expr {
    arrow(cst("Algorithm"), arrow(nat_ty(), prop()))
}
/// `Synchronizer : Type`
/// A synchronizer converts synchronous algorithms to asynchronous ones.
pub fn synchronizer_ty() -> Expr {
    type0()
}
/// `AlphaSynchronizer : Nat → Nat → Type`
/// Alpha synchronizer with O(1) time and O(E) messages per round.
pub fn alpha_synchronizer_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ConsensusProtocol : Nat → Nat → Type`
/// Consensus protocol tolerating f faults among n processes.
pub fn consensus_protocol_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `FLPImpossibility : Prop`
/// FLP impossibility: no deterministic consensus in asynchronous model with 1 crash.
pub fn flp_impossibility_ty() -> Expr {
    prop()
}
/// `CompetitiveRatio : Algorithm → Real → Prop`
/// An online algorithm is c-competitive if cost ≤ c * OPT + b for all inputs.
pub fn competitive_ratio_ty() -> Expr {
    arrow(cst("Algorithm"), arrow(real_ty(), prop()))
}
/// `SkiRental : Nat → Real → Prop`
/// Ski rental: break-even strategy achieves 2-competitive ratio.
pub fn ski_rental_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `KServer : Nat → Graph → Type`
/// k-server problem on a metric space.
pub fn k_server_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("Graph"), type0()))
}
/// `KServerConjecture : Nat → Prop`
/// The k-server conjecture: there exists a k-competitive deterministic algorithm.
pub fn k_server_conjecture_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `OnlineBinPacking : List Real → Nat → Prop`
/// Online bin packing: First Fit Decreasing achieves 11/9 OPT + 6/9.
pub fn online_bin_packing_ty() -> Expr {
    arrow(list_ty(real_ty()), arrow(nat_ty(), prop()))
}
/// `MarkovChain : Type`
/// A discrete-time Markov chain on a finite state space.
pub fn markov_chain_ty() -> Expr {
    type0()
}
/// `MixingTime : MarkovChain → Real → Prop`
/// The mixing time τ_mix(ε) is the time to get within ε of stationary distribution.
pub fn mixing_time_ty() -> Expr {
    arrow(cst("MarkovChain"), arrow(real_ty(), prop()))
}
/// `RapidlyMixing : MarkovChain → Prop`
/// A Markov chain mixes in O(poly(n) log(1/ε)) time.
pub fn rapidly_mixing_ty() -> Expr {
    arrow(cst("MarkovChain"), prop())
}
/// `CouplingFromPast : MarkovChain → Algorithm → Prop`
/// Propp-Wilson coupling from the past gives perfect samples from stationary dist.
pub fn coupling_from_past_ty() -> Expr {
    arrow(cst("MarkovChain"), arrow(cst("Algorithm"), prop()))
}
/// `SpectralGap : MarkovChain → Real`
/// The spectral gap λ = 1 − λ₂ governs mixing time.
pub fn spectral_gap_ty() -> Expr {
    arrow(cst("MarkovChain"), real_ty())
}
/// `CellProbeModel : Nat → Nat → Type`
/// Cell probe model with word size w and s cells.
pub fn cell_probe_model_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `QueryComplexity : DataStructure → Operation → Nat → Prop`
/// Query requires at least t cell probes for any data structure.
pub fn query_complexity_ty() -> Expr {
    arrow(
        cst("DataStructure"),
        arrow(cst("Operation"), arrow(nat_ty(), prop())),
    )
}
/// `PointerMachine : Type`
/// Pointer machine model for data structure complexity.
pub fn pointer_machine_ty() -> Expr {
    type0()
}
/// `StaticLowerBound : Problem → Nat → Prop`
/// A static lower bound: any data structure solving problem needs Ω(t) query time.
pub fn static_lower_bound_ty() -> Expr {
    arrow(cst("Problem"), arrow(nat_ty(), prop()))
}
/// `Witness : Input → Output → Type`
/// A witness (certificate) verifying the output is correct for the given input.
pub fn witness_ty() -> Expr {
    arrow(cst("Input"), arrow(cst("Output"), type0()))
}
/// `CheckerCorrect : Witness → Input → Output → Prop`
/// Checker accepts iff the output is correct on the input.
pub fn checker_correct_ty() -> Expr {
    arrow(
        cst("Witness"),
        arrow(cst("Input"), arrow(cst("Output"), prop())),
    )
}
/// `CertifyingAlgorithm : Algorithm → Prop`
/// An algorithm that returns both output and a verifiable witness.
pub fn certifying_algorithm_ty() -> Expr {
    arrow(cst("Algorithm"), prop())
}
/// `LinearityTest : (Nat → Nat) → Prop`
/// Blum-Luby-Rubinfeld test: a function passes iff it is linear with high probability.
pub fn linearity_test_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), prop())
}
/// `DijkstraCorrect : Graph → Vertex → Dist → Prop`
/// Dijkstra's algorithm computes shortest paths from a source in graphs with non-negative weights.
pub fn dijkstra_correct_ty() -> Expr {
    arrow(
        cst("Graph"),
        arrow(cst("Vertex"), arrow(cst("Dist"), prop())),
    )
}
/// `BellmanFordCorrect : Graph → Vertex → Dist → Prop`
/// Bellman-Ford correctly computes shortest paths or detects negative cycles.
pub fn bellman_ford_correct_ty() -> Expr {
    arrow(
        cst("Graph"),
        arrow(cst("Vertex"), arrow(cst("Dist"), prop())),
    )
}
/// `FloydWarshallCorrect : Graph → Matrix → Prop`
/// Floyd-Warshall correctly computes all-pairs shortest paths.
pub fn floyd_warshall_correct_ty() -> Expr {
    arrow(cst("Graph"), arrow(cst("Matrix"), prop()))
}
/// `PrimCorrect : Graph → SpanningTree → Prop`
/// Prim's algorithm produces a minimum spanning tree.
pub fn prim_correct_ty() -> Expr {
    arrow(cst("Graph"), arrow(cst("SpanningTree"), prop()))
}
/// `KruskalCorrect : Graph → SpanningTree → Prop`
/// Kruskal's algorithm produces a minimum spanning tree.
pub fn kruskal_correct_ty() -> Expr {
    arrow(cst("Graph"), arrow(cst("SpanningTree"), prop()))
}
/// `KMPCorrect : List Nat → List Nat → List Nat → Prop`
/// KMP searches for a pattern in O(n+m) time with correct match positions.
pub fn kmp_correct_ty() -> Expr {
    arrow(
        list_ty(nat_ty()),
        arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop())),
    )
}
/// `AhoCorasickCorrect : List (List Nat) → String → List Nat → Prop`
/// Aho-Corasick simultaneously finds all pattern occurrences in O(n + m + k) time.
pub fn aho_corasick_correct_ty() -> Expr {
    arrow(
        list_ty(list_ty(nat_ty())),
        arrow(cst("String"), arrow(list_ty(nat_ty()), prop())),
    )
}
/// `SuffixArrayCorrect : List Nat → List Nat → Prop`
/// Suffix array sorts all suffixes lexicographically, built in O(n log n).
pub fn suffix_array_correct_ty() -> Expr {
    arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop()))
}
/// `BurrowsWheelerCorrect : List Nat → List Nat → Prop`
/// Burrows-Wheeler transform is invertible and aids compression.
pub fn burrows_wheeler_correct_ty() -> Expr {
    arrow(list_ty(nat_ty()), arrow(list_ty(nat_ty()), prop()))
}
/// `ConvexHullCorrect : List Point → List Point → Prop`
/// Convex hull algorithm produces the minimal convex polygon containing all points.
pub fn convex_hull_correct_ty() -> Expr {
    arrow(list_ty(cst("Point")), arrow(list_ty(cst("Point")), prop()))
}
/// `VoronoiCorrect : List Point → Diagram → Prop`
/// Voronoi diagram partitions the plane into regions closest to each input point.
pub fn voronoi_correct_ty() -> Expr {
    arrow(list_ty(cst("Point")), arrow(cst("Diagram"), prop()))
}
/// `DelaunayCorrect : List Point → Triangulation → Prop`
/// Delaunay triangulation maximizes the minimum angle (no point inside circumcircle).
pub fn delaunay_correct_ty() -> Expr {
    arrow(list_ty(cst("Point")), arrow(cst("Triangulation"), prop()))
}
/// `ClosestPairCorrect : List Point → Pair Point → Prop`
/// Closest pair algorithm finds the minimum distance pair in O(n log n).
pub fn closest_pair_correct_ty() -> Expr {
    arrow(
        list_ty(cst("Point")),
        arrow(pair_ty(cst("Point"), cst("Point")), prop()),
    )
}
/// Alias for `build_certified_algorithms_env`.
pub fn build_env(env: &mut Environment) -> Result<(), String> {
    build_certified_algorithms_env(env)
}
/// Populate an `Environment` with certified algorithm axioms.
pub fn build_certified_algorithms_env(env: &mut Environment) -> Result<(), String> {
    for (name, ty) in &[
        ("PotentialFunction", potential_function_ty()),
        ("AmortizedCost", amortized_cost_ty()),
        ("AggregateCost", aggregate_cost_ty()),
        ("AccountingMethod", accounting_method_ty()),
        ("AmortizedCorrect", amortized_correct_ty()),
        ("IdealCacheModel", ideal_cache_model_ty()),
        ("CacheTransfer", cache_transfer_ty()),
        ("CacheObliviousOptimal", cache_oblivious_optimal_ty()),
        ("RecursiveMatrixLayout", recursive_matrix_layout_ty()),
        ("FunnelSort", funnel_sort_ty()),
        ("FrequencyMoment", frequency_moment_ty()),
        ("HeavyHitter", heavy_hitter_ty()),
        ("ReservoirSample", reservoir_sample_ty()),
        ("CountMinSketch", count_min_sketch_ty()),
        ("CountMinCorrect", count_min_correct_ty()),
        ("AMS_Sketch", ams_sketch_ty()),
        ("IOModel", io_model_ty()),
        ("ExternalSort", external_sort_ty()),
        ("BufferTree", buffer_tree_ty()),
        ("ExternalBFS", external_bfs_ty()),
        ("PRAMModel", pram_model_ty()),
        ("WorkSpan", work_span_ty()),
        ("Brent_Lemma", brent_lemma_ty()),
        ("ListRanking", list_ranking_ty()),
        ("ParallelPrefixSum", parallel_prefix_sum_ty()),
        ("MessageComplexity", message_complexity_ty()),
        ("Synchronizer", synchronizer_ty()),
        ("AlphaSynchronizer", alpha_synchronizer_ty()),
        ("ConsensusProtocol", consensus_protocol_ty()),
        ("FLPImpossibility", flp_impossibility_ty()),
        ("CompetitiveRatio", competitive_ratio_ty()),
        ("SkiRental", ski_rental_ty()),
        ("KServer", k_server_ty()),
        ("KServerConjecture", k_server_conjecture_ty()),
        ("OnlineBinPacking", online_bin_packing_ty()),
        ("MarkovChain", markov_chain_ty()),
        ("MixingTime", mixing_time_ty()),
        ("RapidlyMixing", rapidly_mixing_ty()),
        ("CouplingFromPast", coupling_from_past_ty()),
        ("SpectralGap", spectral_gap_ty()),
        ("CellProbeModel", cell_probe_model_ty()),
        ("QueryComplexity", query_complexity_ty()),
        ("PointerMachine", pointer_machine_ty()),
        ("StaticLowerBound", static_lower_bound_ty()),
        ("Witness", witness_ty()),
        ("CheckerCorrect", checker_correct_ty()),
        ("CertifyingAlgorithm", certifying_algorithm_ty()),
        ("LinearityTest", linearity_test_ty()),
        ("DijkstraCorrect", dijkstra_correct_ty()),
        ("BellmanFordCorrect", bellman_ford_correct_ty()),
        ("FloydWarshallCorrect", floyd_warshall_correct_ty()),
        ("PrimCorrect", prim_correct_ty()),
        ("KruskalCorrect", kruskal_correct_ty()),
        ("KMPCorrect", kmp_correct_ty()),
        ("AhoCorasickCorrect", aho_corasick_correct_ty()),
        ("SuffixArrayCorrect", suffix_array_correct_ty()),
        ("BurrowsWheelerCorrect", burrows_wheeler_correct_ty()),
        ("ConvexHullCorrect", convex_hull_correct_ty()),
        ("VoronoiCorrect", voronoi_correct_ty()),
        ("DelaunayCorrect", delaunay_correct_ty()),
        ("ClosestPairCorrect", closest_pair_correct_ty()),
    ] {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_streaming_frequency_estimator_basic() {
        let mut sketch = StreamingFrequencyEstimator::new(3, 64);
        sketch.insert(42);
        sketch.insert(42);
        sketch.insert(42);
        sketch.insert(7);
        let est = sketch.estimate(42);
        assert!(
            sketch.estimate_lower_bound_holds(42, 3),
            "estimate {} < true freq 3",
            est
        );
        assert!(sketch.estimate_lower_bound_holds(7, 1));
        assert_eq!(sketch.total, 4);
    }
    #[test]
    fn test_cache_oblivious_matrix_multiply() {
        let mut a = CacheObliviousMatrix::new(2);
        let mut b = CacheObliviousMatrix::new(2);
        a.set(0, 0, 1.0);
        a.set(0, 1, 2.0);
        a.set(1, 0, 3.0);
        a.set(1, 1, 4.0);
        b.set(0, 0, 5.0);
        b.set(0, 1, 6.0);
        b.set(1, 0, 7.0);
        b.set(1, 1, 8.0);
        let c = CacheObliviousMatrix::multiply(&a, &b);
        assert!(
            (c.get(0, 0) - 19.0).abs() < 1e-9,
            "c[0,0] = {}",
            c.get(0, 0)
        );
        assert!(
            (c.get(0, 1) - 22.0).abs() < 1e-9,
            "c[0,1] = {}",
            c.get(0, 1)
        );
        assert!(
            (c.get(1, 0) - 43.0).abs() < 1e-9,
            "c[1,0] = {}",
            c.get(1, 0)
        );
        assert!(
            (c.get(1, 1) - 50.0).abs() < 1e-9,
            "c[1,1] = {}",
            c.get(1, 1)
        );
    }
    #[test]
    fn test_certifying_union_find() {
        let mut uf = CertifyingUnionFind::new(5);
        assert_eq!(uf.num_components, 5);
        assert!(uf.union(0, 1));
        assert!(uf.union(2, 3));
        assert!(!uf.union(0, 1));
        assert_eq!(uf.num_components, 3);
        assert!(uf.verify_certificate(5));
        assert_eq!(uf.spanning_forest_certificate().len(), 2);
    }
    #[test]
    fn test_online_scheduler_competitive_ratio() {
        let mut sched = OnlineScheduler::new(1.0);
        sched.schedule(0.4);
        sched.schedule(0.4);
        sched.schedule(0.4);
        sched.schedule(0.4);
        let ratio = sched.competitive_ratio();
        assert!(ratio <= 2.0 + 1e-9, "competitive ratio {} exceeds 2", ratio);
        assert!(sched.num_bins() >= 1);
    }
    #[test]
    fn test_approximation_verifier_minimization() {
        let verifier = ApproximationVerifier::new(true);
        assert!(verifier.verify_ratio(15.0, 10.0, 2.0));
        assert!(!verifier.verify_ratio(25.0, 10.0, 2.0));
        let ratio = verifier.actual_ratio(15.0, 10.0);
        assert!((ratio - 1.5).abs() < 1e-9, "ratio = {}", ratio);
    }
    #[test]
    fn test_approximation_verifier_maximization() {
        let verifier = ApproximationVerifier::new(false);
        assert!(verifier.verify_ratio(75.0, 100.0, 4.0 / 3.0));
    }
    #[test]
    fn test_build_certified_algorithms_env() {
        let mut env = Environment::new();
        let result = build_certified_algorithms_env(&mut env);
        assert!(result.is_ok(), "build should succeed");
        assert!(env.get(&Name::str("PotentialFunction")).is_some());
        assert!(env.get(&Name::str("CountMinSketch")).is_some());
        assert!(env.get(&Name::str("MarkovChain")).is_some());
        assert!(env.get(&Name::str("DijkstraCorrect")).is_some());
        assert!(env.get(&Name::str("ConvexHullCorrect")).is_some());
        assert!(env.get(&Name::str("FLPImpossibility")).is_some());
        assert!(env.get(&Name::str("KServerConjecture")).is_some());
    }
}
#[cfg(test)]
mod tests_cert_alg_ext {
    use super::*;
    #[test]
    fn test_certified_hashmap() {
        let map: CertHashMapExt<String, i32> = CertHashMapExt::new(16);
        assert!(!map.needs_resize());
        assert!((map.load_factor() - 0.0).abs() < 1e-10);
        let desc = map.amortized_complexity_description();
        assert!(desc.contains("CertifiedHashMap"));
    }
    #[test]
    fn test_certified_union_find() {
        let mut uf = CertifiedUnionFind::new(5);
        assert!(uf.correctness_invariant());
        assert!(!uf.are_connected(0, 4));
        uf.union(0, 1);
        uf.union(1, 4);
        assert!(uf.are_connected(0, 4));
        assert!(!uf.are_connected(0, 3));
    }
    #[test]
    fn test_kmp_matcher() {
        let kmp = KMPMatcher::new("aba");
        let matches = kmp.search("abababa");
        assert_eq!(matches, vec![0, 2, 4]);
        let cplx = kmp.complexity_description();
        assert!(cplx.contains("KMP"));
    }
    #[test]
    fn test_kmp_no_match() {
        let kmp = KMPMatcher::new("xyz");
        let matches = kmp.search("abcdefg");
        assert!(matches.is_empty());
    }
    #[test]
    fn test_suffix_array() {
        let sa = SuffixArray::new("banana");
        let found = sa.pattern_search("ana");
        assert!(found.is_some());
        let not_found = sa.pattern_search("xyz");
        assert!(not_found.is_none());
    }
    #[test]
    fn test_certified_bfs() {
        let mut bfs = CertBFSExt::new(6);
        bfs.add_edge(0, 1);
        bfs.add_edge(0, 2);
        bfs.add_edge(1, 3);
        bfs.add_edge(2, 4);
        bfs.add_edge(4, 5);
        bfs.bfs_from(0);
        assert_eq!(bfs.distances[5], Some(3));
        let path = bfs.shortest_path_to(5);
        assert_eq!(path, Some(vec![0, 2, 4, 5]));
        let wit = bfs.correctness_witness(0, 5);
        assert!(wit.contains("dist(0,5) = 3"));
    }
    #[test]
    fn test_bfs_disconnected() {
        let mut bfs = CertBFSExt::new(4);
        bfs.add_edge(0, 1);
        bfs.bfs_from(0);
        assert!(bfs.distances[3].is_none());
        let path = bfs.shortest_path_to(3);
        assert!(path.is_none());
    }
}
