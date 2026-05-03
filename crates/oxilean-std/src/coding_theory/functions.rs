//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Declaration, Environment, Expr, Name};

use super::types::{
    BICMCapacityEstimator, BinaryVector, BurstErrorDetector, CSMeasurementMatrix, ChannelCapacity,
    ConvolutionalEncoder, FountainCode, GF2m, HammingCode, HammingCode74, LinearCode, PolarCode,
    PolarCodeBEC, ProductCode, ReedMullerCode, ReedSolomonCode, TurboCode,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(oxilean_kernel::Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(oxilean_kernel::Level::succ(oxilean_kernel::Level::zero()))
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// `LinearCode : Nat → Nat → Nat → Type` — an (n, k, d) linear code.
pub fn linear_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `GeneratorMatrix : Nat → Nat → Type` — a k×n generator matrix G.
pub fn generator_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ParityCheckMatrix : Nat → Nat → Type` — an (n-k)×n parity-check matrix H.
pub fn parity_check_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `HammingBound : Nat → Nat → Nat → Prop` — sphere-packing (Hamming) bound.
pub fn hamming_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `SingletonBound : Nat → Nat → Nat → Prop` — Singleton bound d ≤ n - k + 1.
pub fn singleton_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `PlotkinBound : Nat → Nat → Nat → Prop` — Plotkin bound on max codewords.
pub fn plotkin_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `shannon_channel_coding : ∀ (R C : Real), R < C → ReliableComm R` —
/// reliable communication is possible for any rate below channel capacity.
pub fn shannon_channel_coding_ty() -> Expr {
    arrow(cst("Real"), arrow(cst("Real"), prop()))
}
/// `noisy_channel : ∀ (p : Real), BSCCapacity p = 1 - BinaryEntropy p` —
/// binary symmetric channel capacity is 1 - H(p).
pub fn noisy_channel_ty() -> Expr {
    arrow(cst("Real"), prop())
}
/// `hamming_perfect : ∀ (r : Nat), PerfectCode (HammingCode r)` —
/// Hamming codes (2^r-1, 2^r-r-1, 3) meet the Hamming bound exactly.
pub fn hamming_perfect_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `reed_solomon_mds : ∀ (n k : Nat), MDS (ReedSolomon n k)` —
/// Reed-Solomon codes are maximum distance separable (meet the Singleton bound).
pub fn reed_solomon_mds_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SystematicCode : Nat → Nat → Nat → Type` — an (n, k, d) code in systematic form [I_k | P].
pub fn systematic_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `dual_code_parity_check_is_generator : Nat → Nat → Prop` —
/// the parity-check matrix of C is the generator matrix of the dual code C^⊥.
pub fn dual_code_parity_check_is_generator_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `systematic_encoding : Nat → Nat → Prop` —
/// every linear code is equivalent to a code in systematic form.
pub fn systematic_encoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `HammingCodeOptimal : Nat → Prop` —
/// Hamming codes are the unique perfect single-error-correcting codes (up to equivalence).
pub fn hamming_code_optimal_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ExtendedHammingCode : Nat → Nat → Nat → Type` —
/// extended Hamming code (2^r, 2^r - r - 1, 4) obtained by adding an overall parity bit.
pub fn extended_hamming_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `ReedSolomonMinDistance : Nat → Nat → Prop` —
/// minimum distance of RS(n, k) over GF(q) is n - k + 1.
pub fn reed_solomon_min_distance_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ReedSolomonEvalEncoding : Nat → Nat → Type` —
/// RS encoding: evaluate a degree-(k-1) polynomial at n distinct field points.
pub fn reed_solomon_eval_encoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BCHCode : Nat → Nat → Nat → Type` —
/// BCH code of length n, design distance δ, over GF(q^m).
pub fn bch_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `BCHDesignDistance : Nat → Nat → Nat → Prop` —
/// BCH bound: minimum distance of BCH code with design distance δ is ≥ δ.
pub fn bch_design_distance_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `BCHRootsExtensionField : Nat → Nat → Prop` —
/// roots of BCH generator polynomial lie in an extension field GF(q^m).
pub fn bch_roots_extension_field_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `LDPCCode : Nat → Nat → Type` — LDPC code defined by sparse parity-check matrix.
pub fn ldpc_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `GallagerLDPC : Nat → Nat → Nat → Type` —
/// Gallager-constructed LDPC code with column weight j and row weight k.
pub fn gallager_ldpc_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `BeliefPropagationDecoding : Nat → Nat → Prop` —
/// belief propagation on the Tanner graph of an LDPC code achieves capacity on BEC.
pub fn belief_propagation_decoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `PolarCode : Nat → Type` — polar code of block length N = 2^n.
pub fn polar_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PolarCodeCapacityAchieving : Nat → Prop` —
/// polar codes achieve the capacity of any binary-input memoryless symmetric channel.
pub fn polar_code_capacity_achieving_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ChannelPolarization : Nat → Prop` —
/// after N successive cancellation steps, synthetic channels polarize to noiseless or pure-noise.
pub fn channel_polarization_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `TurboCode : Nat → Nat → Type` — turbo code constructed from two recursive systematic codes.
pub fn turbo_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `TurboCodeNearCapacity : Nat → Nat → Prop` —
/// turbo codes can operate within a small fraction of Shannon capacity.
pub fn turbo_code_near_capacity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ConvolutionalCode : Nat → Nat → Nat → Type` —
/// convolutional code with rate k/n and constraint length K.
pub fn convolutional_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `ViterbiAlgorithm : Nat → Nat → Prop` —
/// Viterbi algorithm achieves maximum-likelihood decoding for convolutional codes.
pub fn viterbi_algorithm_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ExpanderCode : Nat → Nat → Type` — code based on bipartite expander graph.
pub fn expander_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ExpanderCodeLinearTimeDecoding : Nat → Nat → Prop` —
/// expander codes admit linear-time decoding algorithms.
pub fn expander_code_linear_time_decoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `FountainCode : Nat → Type` — rateless erasure code (e.g., Luby Transform).
pub fn fountain_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `LubyTransformCode : Nat → Type` — LT (Luby Transform) fountain code.
pub fn luby_transform_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `RaptorCode : Nat → Type` — Raptor code: pre-coded LT code with linear encoding/decoding.
pub fn raptor_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `NetworkCodingCapacity : Nat → Nat → Prop` —
/// max-flow min-cut theorem: multicast capacity equals minimum cut capacity.
pub fn network_coding_capacity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `LinearNetworkCoding : Nat → Nat → Prop` —
/// linear network coding suffices to achieve multicast capacity.
pub fn linear_network_coding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SpaceTimeCode : Nat → Nat → Nat → Type` —
/// space-time block code with n_t transmit antennas, n_r receive antennas, rate R.
pub fn space_time_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `AlamoutiScheme : Prop` —
/// Alamouti scheme achieves full diversity order 2 with 2 transmit antennas, simple decoding.
pub fn alamouti_scheme_ty() -> Expr {
    prop()
}
/// `LatticeCode : Nat → Type` — lattice code in R^n.
pub fn lattice_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MinkowskiLatticeTheorem : Nat → Prop` —
/// Minkowski's theorem: a convex body of volume > 2^n contains a nonzero lattice point.
pub fn minkowski_lattice_theorem_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ListDecoding : Nat → Nat → Nat → Type` —
/// list decoder that outputs a list of codewords within Hamming distance e.
pub fn list_decoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `GuruswamiSudanDecoding : Nat → Nat → Prop` —
/// Guruswami-Sudan algorithm list-decodes RS codes up to 1 - √(k/n) fraction of errors.
pub fn guruswami_sudan_decoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ParvareshVardyDecoding : Nat → Nat → Prop` —
/// Parvaresh-Vardy codes improve GS list-decoding radius using correlated polynomials.
pub fn parvaresh_vardy_decoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `AlgebraicGeometryCode : Nat → Nat → Type` —
/// AG code (Goppa code) constructed from an algebraic curve over GF(q).
pub fn algebraic_geometry_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `GoppaCodeMinDistance : Nat → Nat → Prop` —
/// Goppa code minimum distance satisfies d ≥ n - k (Goppa bound).
pub fn goppa_code_min_distance_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `TsfasmanVladutZink : Nat → Prop` —
/// Tsfasman-Vladut-Zink theorem: AG codes exceed Gilbert-Varshamov bound for large alphabet.
pub fn tsfasman_vladut_zink_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `StabilizerCode : Nat → Nat → Type` —
/// quantum stabilizer code encoding k logical qubits into n physical qubits.
pub fn stabilizer_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `CSSCode : Nat → Nat → Type` —
/// Calderbank-Shor-Steane (CSS) code constructed from two classical codes C1 ⊇ C2.
pub fn css_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ToricCode : Nat → Type` —
/// Kitaev toric code on an L×L torus: encodes 2 logical qubits, distance L.
pub fn toric_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `QuantumHammingBound : Nat → Nat → Prop` —
/// quantum Hamming (sphere-packing) bound for non-degenerate codes.
pub fn quantum_hamming_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `QuantumSingleton : Nat → Nat → Prop` —
/// quantum Singleton (Knill-Laflamme) bound: d ≤ n/2 - k/2 + 1 for non-degenerate codes.
pub fn quantum_singleton_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `EliasBassalygo : Nat → Nat → Nat → Prop` —
/// Elias-Bassalygo bound on maximum code size A(n, d).
pub fn elias_bassalygo_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `MRRWBound : Nat → Nat → Nat → Prop` —
/// McEliece-Rodemich-Rumsey-Welch (MRRW) linear-programming bound.
pub fn mrrw_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `GilbertVarshamov : Nat → Nat → Nat → Prop` —
/// Gilbert-Varshamov lower bound: there exists a linear code with d ≥ δ
/// and rate ≥ 1 - H(δ/n).
pub fn gilbert_varshamov_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `GreismerBound : Nat → Nat → Nat → Prop` —
/// Griesmer bound on minimum length of a binary linear code with given k and d.
pub fn griesmer_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `CapacityAchievingSequence : Nat → Prop` —
/// a sequence of codes achieves capacity if rate → C and error probability → 0.
pub fn capacity_achieving_sequence_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ShannonEntropy : Nat → cst(Real)` — `H : Nat → Real`, discrete Shannon entropy.
pub fn shannon_entropy_ty() -> Expr {
    arrow(nat_ty(), cst("Real"))
}
/// `MutualInformation : Nat → Nat → cst(Real)` — `I(X;Y) : Nat → Nat → Real`.
pub fn mutual_information_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), cst("Real")))
}
/// `RegeneratingCode : Nat → Nat → Nat → Nat → Type` —
/// (n, k, d, β) regenerating code for distributed storage.
pub fn regenerating_code_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0()))),
    )
}
/// `MinimumStorageRegenerating : Nat → Nat → Nat → Prop` —
/// MSR point: minimizes storage per node subject to exact repair.
pub fn minimum_storage_regenerating_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `MinimumBandwidthRegenerating : Nat → Nat → Nat → Prop` —
/// MBR point: minimizes repair bandwidth subject to exact repair.
pub fn minimum_bandwidth_regenerating_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `LocallyDecodableCode : Nat → Nat → Nat → Type` —
/// LDC with codeword length N, message length K, query complexity q.
pub fn locally_decodable_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `LocallyDecodableCodeBound : Nat → Nat → Nat → Prop` —
/// lower bound on codeword length for q-query LDCs.
pub fn locally_decodable_code_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `TrellisCodedModulation : Nat → Nat → Type` —
/// TCM scheme combining coding and modulation with bandwidth efficiency b bits/s/Hz.
pub fn trellis_coded_modulation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `UngerboeckCodedModulation : Nat → Prop` —
/// Ungerboeck's set-partitioning TCM achieves coding gain without bandwidth expansion.
pub fn ungerboeck_coded_modulation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SurfaceCode : Nat → Type` —
/// Kitaev surface code on an L×L planar lattice with distance L.
pub fn surface_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SurfaceCodeThreshold : Prop` —
/// surface codes have a fault-tolerance threshold above which logical error rates
/// decrease exponentially with code distance.
pub fn surface_code_threshold_ty() -> Expr {
    prop()
}
/// `ColorCode : Nat → Type` —
/// topological color code on a 2-colex (2-colorable complex) of size parameter n.
pub fn color_code_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ColorCodeTransversalGates : Nat → Prop` —
/// 2D color codes support transversal implementation of the full Clifford group.
pub fn color_code_transversal_gates_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `QuantumLDPCCode : Nat → Nat → Type` —
/// quantum LDPC code with n physical qubits and k logical qubits,
/// defined by sparse check matrices.
pub fn quantum_ldpc_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `QuantumLDPCGoodCode : Nat → Prop` —
/// good quantum LDPC codes have linear distance and linear rate simultaneously.
pub fn quantum_ldpc_good_code_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SpatiallyCoupledCode : Nat → Nat → Type` —
/// spatially coupled ensemble of LDPC codes achieving MAP threshold.
pub fn spatially_coupled_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ThresholdSaturation : Nat → Prop` —
/// threshold saturation: BP threshold of spatially coupled codes equals MAP threshold.
pub fn threshold_saturation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ReedMullerCode : Nat → Nat → Type` —
/// Reed-Muller code RM(r, m) of order r in m variables.
pub fn reed_muller_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ReedMullerDecoding : Nat → Nat → Prop` —
/// Reed-Muller codes of order 1 are first-order Reed-Muller codes
/// decodable by majority logic.
pub fn reed_muller_decoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ReedMullerCapacityAchieving : Prop` —
/// Reed-Muller codes achieve capacity on the binary erasure channel.
pub fn reed_muller_capacity_achieving_ty() -> Expr {
    prop()
}
/// `UniversallyDecodableMatrix : Nat → Nat → Type` —
/// UDM of dimension n×m: rows form a code universal for combinatorial channels.
pub fn universally_decodable_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `UDMCapacityAchieving : Nat → Nat → Prop` —
/// UDMs achieve capacity of the compound channel.
pub fn udm_capacity_achieving_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `CompressedSensingMatrix : Nat → Nat → Type` —
/// m×n measurement matrix satisfying the restricted isometry property.
pub fn compressed_sensing_matrix_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `RestrictedIsometryProperty : Nat → Nat → Prop` —
/// RIP: every set of s columns of the measurement matrix is nearly orthonormal.
pub fn restricted_isometry_property_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `BasisPursuitRecovery : Nat → Nat → Prop` —
/// basis pursuit (L1 minimization) recovers k-sparse signals from m ≥ O(k log(n/k))
/// measurements.
pub fn basis_pursuit_recovery_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MIMOCapacity : Nat → Nat → Prop` —
/// MIMO channel capacity: C = log det(I + (SNR/n_t) H H^†) for n_t transmit, n_r receive
/// antennas.
pub fn mimo_capacity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MIMOSpacetimeDiversity : Nat → Nat → Nat → Prop` —
/// diversity-multiplexing tradeoff: d(r) = (n_t - r)(n_r - r) for MIMO channel.
pub fn mimo_spacetime_diversity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `ProductCode : Nat → Nat → Nat → Nat → Type` —
/// tensor product code C1 ⊗ C2 with parameters (n1*n2, k1*k2, d1*d2).
pub fn product_code_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0()))),
    )
}
/// `ConcatenatedCode : Nat → Nat → Type` —
/// Forney concatenated code with outer code of length n and inner code of length m.
pub fn concatenated_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ConcatenatedCodeDecoding : Nat → Nat → Prop` —
/// concatenated codes decoded by generalized minimum distance (GMD) decoding
/// can correct a fraction of errors.
pub fn concatenated_code_decoding_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `InterleavedCode : Nat → Nat → Type` —
/// interleaved code with interleaving depth d over block length n.
pub fn interleaved_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BurstErrorCorrectionInterleaving : Nat → Nat → Prop` —
/// interleaving converts burst errors into random errors correctable by inner code.
pub fn burst_error_correction_interleaving_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `PolarCodeBECCapacity : Nat → Prop` —
/// polar codes achieve BEC capacity with block-error rate O(2^{-N^{0.5}}).
pub fn polar_code_bec_capacity_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LDPCCapacityBEC : Nat → Prop` —
/// degree-optimized LDPC codes achieve BEC capacity under belief propagation.
pub fn ldpc_capacity_bec_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SpaghettiCodeAwgn : Nat → Prop` —
/// LDPC codes with optimized degree distributions approach AWGN capacity.
pub fn ldpc_capacity_awgn_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CRCPolarCode : Nat → Nat → Type` —
/// CRC-aided polar code with CRC length r and block length N = 2^n.
pub fn crc_polar_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `PolarCodeSuccessiveCancellation : Nat → Prop` —
/// successive cancellation (SC) decoding of polar codes achieves O(N log N) complexity.
pub fn polar_code_successive_cancellation_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `PolarCodeSCL : Nat → Nat → Prop` —
/// successive cancellation list (SCL) decoding of polar codes with list size L.
pub fn polar_code_scl_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `BitInterleavedCodedModulation : Nat → Nat → Type` —
/// BICM scheme with code rate R and modulation order M.
pub fn bit_interleaved_coded_modulation_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BICMCapacity : Nat → Nat → Prop` —
/// BICM capacity equals the sum of mutual informations of individual bit channels.
pub fn bicm_capacity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `NonbinaryLDPC : Nat → Nat → Type` —
/// LDPC code over GF(q) with block length n and q = 2^m.
pub fn nonbinary_ldpc_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `NonbinaryTurboCode : Nat → Nat → Type` —
/// turbo code over GF(q): two RSC codes over GF(q) with interleaver.
pub fn nonbinary_turbo_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `QaryPolarCode : Nat → Nat → Type` —
/// polar code for q-ary symmetric channel with block length N and alphabet size q.
pub fn qary_polar_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `WozencraftEnsemble : Nat → Nat → Prop` —
/// Wozencraft ensemble of random linear codes: most codes achieve GV bound.
pub fn wozencraft_ensemble_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `RandomLinearCodeGV : Nat → Nat → Prop` —
/// random linear codes over GF(q) achieve the Gilbert-Varshamov bound with high
/// probability.
pub fn random_linear_code_gv_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MacWilliamsTransform : Nat → Nat → Prop` —
/// MacWilliams identity: weight enumerator of C^⊥ is the MacWilliams transform of
/// weight enumerator of C.
pub fn mac_williams_transform_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `WeightEnumerator : Nat → Nat → Type` —
/// weight enumerator polynomial W_C(x, y) = Σ_{c ∈ C} x^{n-wt(c)} y^{wt(c)}.
pub fn weight_enumerator_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BlahutArimoto : Nat → Prop` —
/// Blahut-Arimoto algorithm computes channel capacity and rate-distortion function.
pub fn blahut_arimoto_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `RateDistortionFunction : Nat → Nat → Prop` —
/// Shannon rate-distortion theorem: R(D) = min_{p(x̂|x): E\[d(X,X̂)\]≤D} I(X; X̂).
pub fn rate_distortion_function_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ChannelCapacityConverse : Nat → Prop` —
/// converse to channel coding theorem: no code with rate R > C and vanishing error.
pub fn channel_capacity_converse_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// Populate `env` with all coding theory axioms and theorems.
pub fn build_coding_theory_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("LinearCode", linear_code_ty()),
        ("GeneratorMatrix", generator_matrix_ty()),
        ("ParityCheckMatrix", parity_check_matrix_ty()),
        ("HammingBound", hamming_bound_ty()),
        ("SingletonBound", singleton_bound_ty()),
        ("PlotkinBound", plotkin_bound_ty()),
        ("shannon_channel_coding", shannon_channel_coding_ty()),
        ("noisy_channel", noisy_channel_ty()),
        ("hamming_perfect", hamming_perfect_ty()),
        ("reed_solomon_mds", reed_solomon_mds_ty()),
        ("BinaryEntropy", arrow(cst("Real"), cst("Real"))),
        ("BSCCapacity", arrow(cst("Real"), cst("Real"))),
        ("BECCapacity", arrow(cst("Real"), cst("Real"))),
        ("AWGNCapacity", arrow(cst("Real"), cst("Real"))),
        ("HammingCode", arrow(nat_ty(), type0())),
        ("ReedSolomon", arrow(nat_ty(), arrow(nat_ty(), type0()))),
        ("PerfectCode", arrow(type0(), prop())),
        ("MDS", arrow(type0(), prop())),
        ("ReliableComm", arrow(cst("Real"), prop())),
        ("SystematicCode", systematic_code_ty()),
        (
            "dual_code_parity_check_is_generator",
            dual_code_parity_check_is_generator_ty(),
        ),
        ("systematic_encoding", systematic_encoding_ty()),
        ("HammingCodeOptimal", hamming_code_optimal_ty()),
        ("ExtendedHammingCode", extended_hamming_code_ty()),
        ("ReedSolomonMinDistance", reed_solomon_min_distance_ty()),
        ("ReedSolomonEvalEncoding", reed_solomon_eval_encoding_ty()),
        ("BCHCode", bch_code_ty()),
        ("BCHDesignDistance", bch_design_distance_ty()),
        ("BCHRootsExtensionField", bch_roots_extension_field_ty()),
        ("LDPCCode", ldpc_code_ty()),
        ("GallagerLDPC", gallager_ldpc_ty()),
        (
            "BeliefPropagationDecoding",
            belief_propagation_decoding_ty(),
        ),
        ("PolarCode", polar_code_ty()),
        (
            "PolarCodeCapacityAchieving",
            polar_code_capacity_achieving_ty(),
        ),
        ("ChannelPolarization", channel_polarization_ty()),
        ("TurboCode", turbo_code_ty()),
        ("TurboCodeNearCapacity", turbo_code_near_capacity_ty()),
        ("ConvolutionalCode", convolutional_code_ty()),
        ("ViterbiAlgorithm", viterbi_algorithm_ty()),
        ("ExpanderCode", expander_code_ty()),
        (
            "ExpanderCodeLinearTimeDecoding",
            expander_code_linear_time_decoding_ty(),
        ),
        ("FountainCode", fountain_code_ty()),
        ("LubyTransformCode", luby_transform_code_ty()),
        ("RaptorCode", raptor_code_ty()),
        ("NetworkCodingCapacity", network_coding_capacity_ty()),
        ("LinearNetworkCoding", linear_network_coding_ty()),
        ("SpaceTimeCode", space_time_code_ty()),
        ("AlamoutiScheme", alamouti_scheme_ty()),
        ("LatticeCode", lattice_code_ty()),
        ("MinkowskiLatticeTheorem", minkowski_lattice_theorem_ty()),
        ("ListDecoding", list_decoding_ty()),
        ("GuruswamiSudanDecoding", guruswami_sudan_decoding_ty()),
        ("ParvareshVardyDecoding", parvaresh_vardy_decoding_ty()),
        ("AlgebraicGeometryCode", algebraic_geometry_code_ty()),
        ("GoppaCodeMinDistance", goppa_code_min_distance_ty()),
        ("TsfasmanVladutZink", tsfasman_vladut_zink_ty()),
        ("StabilizerCode", stabilizer_code_ty()),
        ("CSSCode", css_code_ty()),
        ("ToricCode", toric_code_ty()),
        ("QuantumHammingBound", quantum_hamming_bound_ty()),
        ("QuantumSingleton", quantum_singleton_ty()),
        ("EliasBassalygo", elias_bassalygo_ty()),
        ("MRRWBound", mrrw_bound_ty()),
        ("GilbertVarshamov", gilbert_varshamov_ty()),
        ("GreismerBound", griesmer_bound_ty()),
        (
            "CapacityAchievingSequence",
            capacity_achieving_sequence_ty(),
        ),
        ("ShannonEntropy", shannon_entropy_ty()),
        ("MutualInformation", mutual_information_ty()),
        ("RegeneratingCode", regenerating_code_ty()),
        (
            "MinimumStorageRegenerating",
            minimum_storage_regenerating_ty(),
        ),
        (
            "MinimumBandwidthRegenerating",
            minimum_bandwidth_regenerating_ty(),
        ),
        ("LocallyDecodableCode", locally_decodable_code_ty()),
        (
            "LocallyDecodableCodeBound",
            locally_decodable_code_bound_ty(),
        ),
        ("TrellisCodedModulation", trellis_coded_modulation_ty()),
        (
            "UngerboeckCodedModulation",
            ungerboeck_coded_modulation_ty(),
        ),
        ("SurfaceCode", surface_code_ty()),
        ("SurfaceCodeThreshold", surface_code_threshold_ty()),
        ("ColorCode", color_code_ty()),
        (
            "ColorCodeTransversalGates",
            color_code_transversal_gates_ty(),
        ),
        ("QuantumLDPCCode", quantum_ldpc_code_ty()),
        ("QuantumLDPCGoodCode", quantum_ldpc_good_code_ty()),
        ("SpatiallyCoupledCode", spatially_coupled_code_ty()),
        ("ThresholdSaturation", threshold_saturation_ty()),
        ("ReedMullerCode", reed_muller_code_ty()),
        ("ReedMullerDecoding", reed_muller_decoding_ty()),
        (
            "ReedMullerCapacityAchieving",
            reed_muller_capacity_achieving_ty(),
        ),
        (
            "UniversallyDecodableMatrix",
            universally_decodable_matrix_ty(),
        ),
        ("UDMCapacityAchieving", udm_capacity_achieving_ty()),
        ("CompressedSensingMatrix", compressed_sensing_matrix_ty()),
        (
            "RestrictedIsometryProperty",
            restricted_isometry_property_ty(),
        ),
        ("BasisPursuitRecovery", basis_pursuit_recovery_ty()),
        ("MIMOCapacity", mimo_capacity_ty()),
        ("MIMOSpacetimeDiversity", mimo_spacetime_diversity_ty()),
        ("ProductCode", product_code_ty()),
        ("ConcatenatedCode", concatenated_code_ty()),
        ("ConcatenatedCodeDecoding", concatenated_code_decoding_ty()),
        ("InterleavedCode", interleaved_code_ty()),
        (
            "BurstErrorCorrectionInterleaving",
            burst_error_correction_interleaving_ty(),
        ),
        ("PolarCodeBECCapacity", polar_code_bec_capacity_ty()),
        ("LDPCCapacityBEC", ldpc_capacity_bec_ty()),
        ("LDPCCapacityAWGN", ldpc_capacity_awgn_ty()),
        ("CRCPolarCode", crc_polar_code_ty()),
        (
            "PolarCodeSuccessiveCancellation",
            polar_code_successive_cancellation_ty(),
        ),
        ("PolarCodeSCL", polar_code_scl_ty()),
        (
            "BitInterleavedCodedModulation",
            bit_interleaved_coded_modulation_ty(),
        ),
        ("BICMCapacity", bicm_capacity_ty()),
        ("NonbinaryLDPC", nonbinary_ldpc_ty()),
        ("NonbinaryTurboCode", nonbinary_turbo_code_ty()),
        ("QaryPolarCode", qary_polar_code_ty()),
        ("WozencraftEnsemble", wozencraft_ensemble_ty()),
        ("RandomLinearCodeGV", random_linear_code_gv_ty()),
        ("MacWilliamsTransform", mac_williams_transform_ty()),
        ("WeightEnumerator", weight_enumerator_ty()),
        ("BlahutArimoto", blahut_arimoto_ty()),
        ("RateDistortionFunction", rate_distortion_function_ty()),
        ("ChannelCapacityConverse", channel_capacity_converse_ty()),
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
/// Volume of a Hamming ball of radius `t` in `{0,1}^n`: Σ_{i=0}^{t} C(n,i).
pub fn hamming_ball_volume(n: usize, t: usize) -> usize {
    let mut vol = 0usize;
    let mut binom = 1usize;
    for i in 0..=t {
        vol = vol.saturating_add(binom);
        if i < t {
            binom = binom.saturating_mul(n - i) / (i + 1);
        }
    }
    vol
}
/// Approximation of erfc(x) = (2/√π) ∫_x^∞ e^{-t²} dt.
///
/// Uses the rational approximation from Abramowitz & Stegun §7.1.26
/// with maximum absolute error < 1.5 × 10^{-7}.
pub fn erfc_approx(x: f64) -> f64 {
    if x < 0.0 {
        return 2.0 - erfc_approx(-x);
    }
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let poly = t
        * (0.254829592
            + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    poly * (-x * x).exp()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_vector_hamming_weight() {
        let v = BinaryVector::from_bits(vec![true, false, true, true, false]);
        assert_eq!(v.hamming_weight(), 3);
        let z = BinaryVector::new(4);
        assert_eq!(z.hamming_weight(), 0);
    }
    #[test]
    fn test_binary_vector_xor_dot() {
        let a = BinaryVector::from_bits(vec![true, false, true]);
        let b = BinaryVector::from_bits(vec![true, true, false]);
        let c = a.xor(&b);
        assert_eq!(c.bits, vec![false, true, true]);
        assert!(a.dot(&b));
        let x = BinaryVector::from_bits(vec![true, false, false]);
        let y = BinaryVector::from_bits(vec![false, true, false]);
        assert!(!x.dot(&y));
    }
    #[test]
    fn test_binary_vector_hamming_distance() {
        let a = BinaryVector::from_bits(vec![true, false, true, false]);
        let b = BinaryVector::from_bits(vec![false, false, true, true]);
        assert_eq!(a.hamming_distance(&b), 2);
    }
    #[test]
    fn test_linear_code_rate_redundancy() {
        let code = LinearCode::new(7, 4, 3);
        assert!((code.rate() - 4.0 / 7.0).abs() < 1e-10);
        assert_eq!(code.redundancy(), 3);
        assert_eq!(code.corrects_errors(), 1);
        assert_eq!(code.detects_errors(), 2);
    }
    #[test]
    fn test_singleton_bound_mds() {
        let mds = LinearCode::new(7, 4, 4);
        assert!(mds.meets_singleton_bound());
        let not_mds = LinearCode::new(7, 4, 3);
        assert!(!not_mds.meets_singleton_bound());
    }
    #[test]
    fn test_hamming_code_parameters() {
        let ham = HammingCode::new(3);
        let lc = ham.to_linear_code();
        assert_eq!(lc.n, 7);
        assert_eq!(lc.k, 4);
        assert_eq!(lc.d_min, 3);
        assert_eq!(lc.corrects_errors(), 1);
    }
    #[test]
    fn test_hamming_code_is_perfect() {
        let ham = HammingCode::new(3);
        let lc = ham.to_linear_code();
        assert!(lc.meets_hamming_bound());
    }
    #[test]
    fn test_hamming_74_encode_is_codeword() {
        let ham = HammingCode74::new();
        let msg = BinaryVector::from_bits(vec![true, false, true, true]);
        let cw = ham.encode(&msg);
        assert_eq!(cw.bits.len(), 7);
        assert!(ham.inner.is_codeword(&cw));
    }
    #[test]
    fn test_hamming_74_single_error_correction() {
        let ham = HammingCode74::new();
        let msg = BinaryVector::from_bits(vec![true, false, true, false]);
        let cw = ham.encode(&msg);
        let mut received = cw.clone();
        received.bits[2] ^= true;
        let corrected = ham.correct(&received);
        assert_eq!(corrected.bits, cw.bits);
    }
    #[test]
    fn test_linear_code_matrix_encode_syndrome() {
        let ham = HammingCode74::new();
        let msg = BinaryVector::from_bits(vec![false, true, false, true]);
        let cw = ham.encode(&msg);
        let syn = ham.syndrome(&cw);
        assert_eq!(syn.hamming_weight(), 0);
    }
    #[test]
    fn test_reed_solomon_encode() {
        let rs = ReedSolomonCode::new(7, 3, 11);
        let cw = rs.encode(&[1, 2, 3]);
        assert_eq!(cw.len(), 7);
        assert!(cw.iter().all(|&v| v < 11));
        assert_eq!(rs.min_distance(), 5);
        assert_eq!(rs.error_correction_capability(), 2);
    }
    #[test]
    fn test_reed_solomon_zero_message() {
        let rs = ReedSolomonCode::new(5, 3, 7);
        let cw = rs.encode(&[0, 0, 0]);
        assert!(cw.iter().all(|&v| v == 0));
    }
    #[test]
    fn test_convolutional_encoder_basic() {
        let mut enc = ConvolutionalEncoder::new(1, 2, 3, vec![0b111, 0b101]);
        enc.reset();
        let out = enc.encode_bit(true);
        assert_eq!(out.len(), 2);
        assert_eq!(out, vec![true, true]);
    }
    #[test]
    fn test_convolutional_encoder_flush() {
        let mut enc = ConvolutionalEncoder::new(1, 2, 3, vec![0b111, 0b101]);
        enc.reset();
        let _ = enc.encode(&[true, false, true]);
        let tail = enc.flush();
        assert_eq!(tail.len(), 4);
    }
    #[test]
    fn test_channel_capacity_bsc() {
        assert!((ChannelCapacity::bsc_capacity(0.0) - 1.0).abs() < 1e-10);
        assert!(ChannelCapacity::bsc_capacity(0.5).abs() < 1e-10);
        assert!((ChannelCapacity::bsc_capacity(1.0) - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_channel_capacity_bec_awgn() {
        assert!((ChannelCapacity::bec_capacity(0.5) - 0.5).abs() < 1e-10);
        assert!(ChannelCapacity::awgn_capacity(0.0).abs() < 1e-10);
        assert!((ChannelCapacity::awgn_capacity(1.0) - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_q_ary_entropy() {
        assert!(ChannelCapacity::q_ary_entropy(0.0, 4).abs() < 1e-10);
        assert!(ChannelCapacity::q_ary_entropy(1.0, 4).abs() < 1e-10);
        let p = 0.3;
        let bh = ChannelCapacity::binary_entropy(p);
        let qh = ChannelCapacity::q_ary_entropy(p, 2);
        assert!((bh - qh).abs() < 1e-8, "bh={bh} qh={qh}");
    }
    #[test]
    fn test_gf2m_arithmetic() {
        let gf = GF2m::new(3, 11);
        assert_eq!(gf.size, 8);
        assert_eq!(gf.pow(0), 1);
        assert_eq!(gf.pow(1), 2);
        assert_eq!(gf.pow(2), 4);
        assert_eq!(gf.pow(7), gf.pow(0));
        assert_eq!(gf.mul(1, 1), 1);
        assert_eq!(gf.add(3, 5), 6);
        assert_eq!(gf.mul(gf.inv(2), 2), 1);
    }
    #[test]
    fn test_burst_error_detector() {
        let det = BurstErrorDetector::new(7, 2, vec![true, true, true]);
        let zero_cw = BinaryVector::new(7);
        assert!(det.is_valid(&zero_cw));
        let mut errored = BinaryVector::new(7);
        errored.bits[3] = true;
        let syn = det.compute_syndrome(&errored);
        assert_ne!(syn.hamming_weight(), 0);
    }
    #[test]
    fn test_build_coding_theory_env() {
        let mut env = Environment::new();
        build_coding_theory_env(&mut env);
        assert!(env.get(&Name::str("LinearCode")).is_some());
        assert!(env.get(&Name::str("shannon_channel_coding")).is_some());
        assert!(env.get(&Name::str("hamming_perfect")).is_some());
        assert!(env.get(&Name::str("reed_solomon_mds")).is_some());
        assert!(env.get(&Name::str("BCHCode")).is_some());
        assert!(env.get(&Name::str("PolarCode")).is_some());
        assert!(env.get(&Name::str("TurboCode")).is_some());
        assert!(env.get(&Name::str("LDPCCode")).is_some());
        assert!(env.get(&Name::str("StabilizerCode")).is_some());
        assert!(env.get(&Name::str("CSSCode")).is_some());
        assert!(env.get(&Name::str("ToricCode")).is_some());
        assert!(env.get(&Name::str("EliasBassalygo")).is_some());
        assert!(env.get(&Name::str("MRRWBound")).is_some());
        assert!(env.get(&Name::str("GilbertVarshamov")).is_some());
        assert!(env.get(&Name::str("GuruswamiSudanDecoding")).is_some());
        assert!(env.get(&Name::str("RegeneratingCode")).is_some());
        assert!(env.get(&Name::str("LocallyDecodableCode")).is_some());
        assert!(env.get(&Name::str("AlamoutiScheme")).is_some());
        assert!(env.get(&Name::str("TrellisCodedModulation")).is_some());
        assert!(env.get(&Name::str("SurfaceCode")).is_some());
        assert!(env.get(&Name::str("ColorCode")).is_some());
        assert!(env.get(&Name::str("QuantumLDPCCode")).is_some());
        assert!(env.get(&Name::str("ReedMullerCode")).is_some());
        assert!(env.get(&Name::str("CompressedSensingMatrix")).is_some());
        assert!(env.get(&Name::str("RestrictedIsometryProperty")).is_some());
        assert!(env.get(&Name::str("MIMOCapacity")).is_some());
        assert!(env.get(&Name::str("ProductCode")).is_some());
        assert!(env.get(&Name::str("WeightEnumerator")).is_some());
        assert!(env.get(&Name::str("BlahutArimoto")).is_some());
        assert!(env.get(&Name::str("RateDistortionFunction")).is_some());
        assert!(env.get(&Name::str("ChannelCapacityConverse")).is_some());
    }
    #[test]
    fn test_reed_muller_code_parameters() {
        let rm = ReedMullerCode::new(1, 3);
        assert_eq!(rm.block_length(), 8);
        assert_eq!(rm.min_distance(), 4);
        assert_eq!(rm.dimension(), 4);
        assert!(rm.is_first_order());
        assert_eq!(rm.error_correction_capability(), 1);
        let rm2 = ReedMullerCode::new(2, 4);
        assert_eq!(rm2.block_length(), 16);
        assert_eq!(rm2.min_distance(), 4);
        assert_eq!(rm2.dimension(), 11);
        assert!(!rm2.is_first_order());
    }
    #[test]
    fn test_product_code_parameters() {
        let c1 = LinearCode::new(7, 4, 3);
        let c2 = LinearCode::new(5, 3, 3);
        let pc = ProductCode::new(c1, c2);
        assert_eq!(pc.block_length(), 35);
        assert_eq!(pc.dimension(), 12);
        assert_eq!(pc.min_distance(), 9);
        let rate = pc.rate();
        assert!((rate - 12.0 / 35.0).abs() < 1e-10);
    }
    #[test]
    fn test_polar_code_bec_construction() {
        let polar = PolarCodeBEC::new(4, 8, 0.5);
        assert_eq!(polar.block_length(), 16);
        assert_eq!(polar.dimension(), 8);
        let rate = polar.rate();
        assert!((rate - 0.5).abs() < 1e-10);
        for &z in &polar.erasure_probs {
            assert!(z >= 0.0 && z <= 1.0 + 1e-12);
        }
        let frac = polar.fraction_good(0.01);
        assert!(frac >= 0.0 && frac <= 1.0);
    }
    #[test]
    fn test_cs_measurement_matrix() {
        let data = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let phi = CSMeasurementMatrix::from_data(3, 3, data);
        assert!(phi.mutual_coherence() < 1e-10);
        let y = phi.apply(&[1.0, 2.0, 3.0]);
        assert!((y[0] - 1.0).abs() < 1e-10);
        assert!((y[1] - 2.0).abs() < 1e-10);
        assert!((y[2] - 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_bicm_capacity_bpsk() {
        let bpsk = BICMCapacityEstimator::new(2);
        assert_eq!(bpsk.bits_per_symbol, 1);
        assert_eq!(bpsk.spectral_efficiency_upper_bound(), 1.0);
        let cap = bpsk.approximate_capacity_db(30.0);
        assert!(cap > 0.99 && cap <= 1.0);
        let cap_low = bpsk.approximate_capacity_db(-10.0);
        assert!(cap_low >= 0.0 && cap_low < 0.5);
    }
    #[test]
    fn test_bicm_capacity_qam() {
        let qam16 = BICMCapacityEstimator::new(16);
        assert_eq!(qam16.bits_per_symbol, 4);
        assert_eq!(qam16.spectral_efficiency_upper_bound(), 4.0);
        let cap = qam16.approximate_capacity_db(20.0);
        assert!(cap > 0.0 && cap <= 4.0);
    }
}
#[allow(dead_code)]
pub fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let mut result = 1usize;
    for i in 0..k.min(n - k) {
        result = result.saturating_mul(n - i);
        result /= i + 1;
    }
    result
}
#[cfg(test)]
mod tests_coding_extra {
    use super::*;
    #[test]
    fn test_reed_solomon() {
        let rs = ReedSolomonCode::new(7, 4, 8);
        assert_eq!(rs.distance(), 4);
        assert!(rs.is_mds());
        assert_eq!(rs.error_correction_capacity(), 1);
        assert_eq!(rs.erasure_correction_capacity(), 3);
        assert!((rs.rate() - 4.0 / 7.0).abs() < 1e-9);
    }
    #[test]
    fn test_turbo_code() {
        let tc = TurboCode::standard_3gpp(1024);
        assert!(tc.overall_rate() > 0.0 && tc.overall_rate() < 1.0);
    }
    #[test]
    fn test_polar_code() {
        let pc = PolarCode::new(1024, 512);
        assert!((pc.rate() - 0.5).abs() < 1e-9);
        assert!(pc.is_capacity_achieving());
        assert!(pc.successive_cancellation_complexity() > 0);
    }
    #[test]
    fn test_fountain_code() {
        let lt = FountainCode::lt_code(1000);
        assert!(lt.is_rateless());
        let needed = lt.n_symbols_to_decode();
        assert!(needed > 1000);
        let raptor = FountainCode::raptor_code(1000);
        assert!(raptor.decoding_complexity() < lt.decoding_complexity());
    }
    #[test]
    fn test_linear_code() {
        let hamming = LinearCode::new(7, 4, 3);
        assert!(hamming.satisfies_singleton_bound());
        assert!(hamming.satisfies_hamming_bound());
        assert!(hamming.is_perfect());
    }
}
