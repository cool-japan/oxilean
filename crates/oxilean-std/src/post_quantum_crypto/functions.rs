//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    DilithiumParams, DilithiumVariant, FalconParams, FalconVariant, HashBasedSignature,
    IsogenyCrypto, KyberVariant, LWESampleGenerator, LatticeBasisReducer, McElieceParams,
    ModularLatticeArithmetic, RingPoly, SphincsParams, ToyLWE, ToyLamport, ToyMerkleTree,
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
pub fn list_nat() -> Expr {
    app(cst("List"), nat_ty())
}
pub fn bytes_ty() -> Expr {
    list_nat()
}
/// `Lattice : Nat → Type`
///
/// An n-dimensional integer lattice L ⊆ Z^n, represented by its basis matrix.
pub fn lattice_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `LatticeVector : Nat → Type`
///
/// A vector in Z^n (element of a lattice).
pub fn lattice_vector_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ShortestVector : Lattice n → LatticeVector n → Prop`
///
/// Shortest vector problem (SVP): the given vector is a shortest non-zero
/// vector in the lattice. SVP is NP-hard under randomized reductions.
pub fn shortest_vector_ty() -> Expr {
    arrow(
        app(cst("Lattice"), nat_ty()),
        arrow(app(cst("LatticeVector"), nat_ty()), prop()),
    )
}
/// `ClosestVector : Lattice n → LatticeVector n → LatticeVector n → Prop`
///
/// Closest vector problem (CVP): find the lattice vector closest to a target.
/// CVP is NP-hard and the basis of LWE hardness.
pub fn closest_vector_ty() -> Expr {
    arrow(
        app(cst("Lattice"), nat_ty()),
        arrow(
            app(cst("LatticeVector"), nat_ty()),
            arrow(app(cst("LatticeVector"), nat_ty()), prop()),
        ),
    )
}
/// `GaussianHeuristic : Nat → Nat → Prop`
///
/// The Gaussian heuristic: the length of the shortest lattice vector
/// is approximately sqrt(n / (2πe)) * det(L)^(1/n).
pub fn gaussian_heuristic_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `LWEDistribution : Nat → Nat → Type`
///
/// The LWE distribution over Z_q^n: samples (a, b = ⟨a,s⟩ + e mod q)
/// where a is uniform, s is a secret vector, and e is drawn from
/// a discrete Gaussian error distribution χ.
pub fn lwe_distribution_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `LWEHardness : Nat → Nat → Prop`
///
/// Decision LWE hardness: no polynomial-time adversary can distinguish
/// LWE samples from uniformly random samples over Z_q^n × Z_q.
/// Under quantum reductions, hardness follows from worst-case SVP on random lattices.
pub fn lwe_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `LWEEncryption : Nat → Nat → Type`
///
/// An LWE-based public-key encryption scheme parameterized by (n, q):
/// keygen samples s ← χ^n; public key includes (A, As + e).
pub fn lwe_encryption_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `LWERegev : Nat → Nat → Prop`
///
/// Regev's theorem: LWE with parameters (n, q, χ_σ) is hard if
/// the Shortest Independent Vectors Problem (SIVP) is hard in the
/// worst case for γ-approximate factors where γ = Õ(n·q/σ).
pub fn lwe_regev_theorem_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `RingLWEDistribution : Nat → Nat → Type`
///
/// Ring-LWE distribution over Rq = Zq[x]/(x^n + 1):
/// samples (a, b = a·s + e) where a is uniform in Rq,
/// s is secret in Rq, e is drawn from a Gaussian χ over Rq.
pub fn rlwe_distribution_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `RingLWEHardness : Nat → Nat → Prop`
///
/// Decision Ring-LWE hardness: indistinguishability from uniform in Rq × Rq,
/// assuming ideal lattice problems are hard.
pub fn rlwe_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `RingLWEToLWEReduction : Nat → Nat → Prop`
///
/// There is a polynomial-time reduction from worst-case problems on
/// ideal lattices to Ring-LWE (Lyubashevsky-Peikert-Regev 2010).
pub fn rlwe_to_lwe_reduction_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `NTRUParams : Nat → Nat → Type`
///
/// NTRU parameters (N, q): polynomial ring Z[x]/(x^N - 1), modulus q.
/// Keys f, g are small (ternary or binary) polynomials in this ring.
pub fn ntru_params_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `NTRUPublicKey : NTRUParams → Type`
///
/// NTRU public key h = p·g·f^{-1} mod q in the ring Z_q[x]/(x^N - 1).
pub fn ntru_public_key_ty() -> Expr {
    arrow(app2(cst("NTRUParams"), nat_ty(), nat_ty()), type0())
}
/// `NTRUHardness : Nat → Nat → Prop`
///
/// NTRU hardness assumption: distinguishing the NTRU distribution from
/// uniform is computationally hard. Related to shortest vector problems
/// on NTRU lattices.
pub fn ntru_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `NTRUCorrectness : Nat → Nat → Prop`
///
/// NTRU decryption correctness: if parameters are chosen so the decryption
/// polynomial has small coefficients, decryption recovers the plaintext.
pub fn ntru_correctness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `KyberParams : Type`
///
/// CRYSTALS-Kyber parameter set: security level k (2, 3, or 4),
/// modulus q = 3329, polynomial degree n = 256.
pub fn kyber_params_ty() -> Expr {
    type0()
}
/// `KyberKeyPair : KyberParams → Type`
///
/// Kyber key pair: (ek, dk) where ek is the encapsulation key
/// and dk is the decapsulation key.
pub fn kyber_key_pair_ty() -> Expr {
    arrow(kyber_params_ty(), type0())
}
/// `KyberIND_CCA2 : KyberParams → Prop`
///
/// IND-CCA2 security of Kyber: under the Module-LWE assumption,
/// Kyber-KEM is IND-CCA2 secure in the random oracle model.
pub fn kyber_ind_cca2_ty() -> Expr {
    arrow(kyber_params_ty(), prop())
}
/// `KyberMLWEHardness : KyberParams → Prop`
///
/// Module-LWE hardness underlying Kyber: the Module-LWE problem
/// with the Kyber parameters is computationally hard.
pub fn kyber_mlwe_hardness_ty() -> Expr {
    arrow(kyber_params_ty(), prop())
}
/// `DilithiumParams : Type`
///
/// CRYSTALS-Dilithium parameter set: module rank (k, l),
/// modulus q = 8380417, security category.
pub fn dilithium_params_ty() -> Expr {
    type0()
}
/// `DilithiumSignature : DilithiumParams → Type`
///
/// A Dilithium signature: (z, h, c̃) where z is the response polynomial,
/// h is the hint vector, and c̃ is the challenge hash.
pub fn dilithium_signature_ty() -> Expr {
    arrow(dilithium_params_ty(), type0())
}
/// `DilithiumEUF_CMA : DilithiumParams → Prop`
///
/// EUF-CMA security of Dilithium under Module-LWE and Module-SIS assumptions.
pub fn dilithium_euf_cma_ty() -> Expr {
    arrow(dilithium_params_ty(), prop())
}
/// `FalconParams : Type`
///
/// Falcon parameter set: NTRU degree n (512 or 1024), modulus q = 12289.
/// Uses fast Fourier sampling over NTRU lattices (GPV framework).
pub fn falcon_params_ty() -> Expr {
    type0()
}
/// `FalconSignature : FalconParams → Type`
///
/// A Falcon signature: a short vector in the NTRU lattice,
/// output by the fast Fourier sampling algorithm.
pub fn falcon_signature_ty() -> Expr {
    arrow(falcon_params_ty(), type0())
}
/// `FalconEUF_CMA : FalconParams → Prop`
///
/// EUF-CMA security of Falcon under the NTRU hardness assumption
/// in the random oracle model.
pub fn falcon_euf_cma_ty() -> Expr {
    arrow(falcon_params_ty(), prop())
}
/// `LamportParams : Nat → Type`
///
/// Lamport one-time signature parameters with n-bit hash output.
/// Key generation: for each bit i ∈ {0, ..., n-1}, sample two pre-images
/// (x[i][0], x[i][1]); public key is (H(x[i][0]), H(x[i][1])).
pub fn lamport_params_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `LamportOneTimeUse : LamportParams → Prop`
///
/// One-time security: reusing a Lamport key for two different messages
/// reveals both pre-images for at least one bit position, breaking security.
pub fn lamport_one_time_use_ty() -> Expr {
    arrow(app(cst("LamportParams"), nat_ty()), prop())
}
/// `MerkleTree : Nat → Type`
///
/// A Merkle hash tree of depth d: leaves are Lamport one-time public keys,
/// internal nodes are hash values of their children.
pub fn merkle_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MerkleSignature : MerkleTree → Type`
///
/// A Merkle signature: one-time signature plus authentication path
/// (sibling hashes from leaf to root).
pub fn merkle_signature_ty() -> Expr {
    arrow(app(cst("MerkleTree"), nat_ty()), type0())
}
/// `MerkleSignatureCorrectness : Nat → Prop`
///
/// Merkle signature correctness: verifying a signature with the
/// authentication path reconstructs the root hash.
pub fn merkle_signature_correctness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SPHINCSPlusParams : Type`
///
/// SPHINCS+ parameter set: hypertree with d layers, each a Merkle tree
/// of height h/d, using FORS for few-time signing.
pub fn sphincs_plus_params_ty() -> Expr {
    type0()
}
/// `SPHINCSPlusStateless : SPHINCSPlusParams → Prop`
///
/// SPHINCS+ is stateless: unlike plain Merkle signatures, SPHINCS+
/// uses a pseudorandom key index derivation so no state is needed.
pub fn sphincs_plus_stateless_ty() -> Expr {
    arrow(sphincs_plus_params_ty(), prop())
}
/// `SPHINCSPlusEUF_CMA : SPHINCSPlusParams → Prop`
///
/// EUF-CMA security of SPHINCS+ in the (quantum) random oracle model,
/// under the one-wayness of the hash function only.
pub fn sphincs_plus_euf_cma_ty() -> Expr {
    arrow(sphincs_plus_params_ty(), prop())
}
/// `BinaryGoppaCode : Nat → Nat → Nat → Type`
///
/// A binary Goppa code with parameters (n, k, t): length n, dimension k,
/// capable of correcting t errors. The hidden code structure is used in McEliece.
pub fn binary_goppa_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `McEliecePublicKey : BinaryGoppaCode → Type`
///
/// McEliece public key: a disguised generator matrix obtained by
/// scrambling the Goppa code's generator matrix with random invertible S
/// and permutation P: Gpub = S · G · P.
pub fn mceliece_public_key_ty() -> Expr {
    arrow(
        app3(cst("BinaryGoppaCode"), nat_ty(), nat_ty(), nat_ty()),
        type0(),
    )
}
/// `McElieceHardness : Nat → Nat → Nat → Prop`
///
/// McEliece hardness: decoding a random linear code is NP-complete.
/// The disguised Goppa code generator matrix is computationally
/// indistinguishable from a random matrix.
pub fn mceliece_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `McElieceCryptosystem : Type`
///
/// The complete McEliece cryptosystem: key generation using a random
/// binary Goppa code, encryption by adding a random error vector of
/// weight t, decryption using the Patterson algorithm.
pub fn mceliece_cryptosystem_ty() -> Expr {
    type0()
}
/// `SupersingularCurve : Type`
///
/// A supersingular elliptic curve over a finite field F_{p^2}.
/// Supersingular curves form a smaller class and have a rich
/// isogeny graph structure used in isogeny-based cryptography.
pub fn supersingular_curve_ty() -> Expr {
    type0()
}
/// `Isogeny : SupersingularCurve → SupersingularCurve → Nat → Type`
///
/// An isogeny of degree l between supersingular curves E1 and E2:
/// a non-constant rational map φ: E1 → E2 that is a group homomorphism.
pub fn isogeny_ty() -> Expr {
    arrow(
        supersingular_curve_ty(),
        arrow(supersingular_curve_ty(), arrow(nat_ty(), type0())),
    )
}
/// `SIDHKeyExchange : Type`
///
/// SIDH (Supersingular Isogeny Diffie-Hellman) key exchange protocol.
///
/// NOTE: Classic SIKE (based on SIDH) was completely broken by the
/// Castryck-Decru-Maino-Martindale-Panny attacks (2022), which exploit
/// the auxiliary torsion point information in SIDH.
///
/// Remaining active areas: CSIDH (commutative SIDH, broken for some params),
/// SQISign (based on quaternion algebra endomorphisms), and other variants.
pub fn sidh_key_exchange_ty() -> Expr {
    type0()
}
/// `SIDHBrokenByCastryckDecru : Prop`
///
/// Formal statement that the classical SIKE/SIDH is broken:
/// the Castryck-Decru attack recovers private keys in polynomial time
/// from the SIDH public key information including auxiliary torsion points.
pub fn sidh_broken_ty() -> Expr {
    prop()
}
/// `CSIDHHardness : Nat → Prop`
///
/// CSIDH hardness: the commutative isogeny problem over F_p is assumed
/// hard. CSIDH (Castryck-Lange-Martindale-Panny-Renes 2018) remains
/// an active research direction for post-quantum key exchange.
pub fn csidh_hardness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `QuantumRandomOracle : Type`
///
/// A quantum random oracle: a function H: {0,1}^* → {0,1}^n that
/// adversaries can query in quantum superposition.
pub fn quantum_random_oracle_ty() -> Expr {
    type0()
}
/// `QROMSecurity : QuantumRandomOracle → Prop`
///
/// Security in the QROM: a cryptographic primitive is QROM-secure if
/// it remains secure even when the adversary can query the random oracle
/// in quantum superposition, as modeled by Boneh et al. (2011).
pub fn qrom_security_ty() -> Expr {
    arrow(quantum_random_oracle_ty(), prop())
}
/// `FiatShamirQROM : Prop`
///
/// Fiat-Shamir in the QROM (Liu-Zhandry 2019): the Fiat-Shamir transform
/// applied to a Sigma protocol with quantum-unique responses gives a
/// QROM-secure NIZK, provided the underlying Sigma protocol has certain
/// extractability properties.
pub fn fiat_shamir_qrom_ty() -> Expr {
    prop()
}
/// `QROMForkingLemma : Prop`
///
/// The quantum forking lemma (Don-Fehr-Majenz-Schaffner 2022):
/// quantum counterpart of the classical forking lemma, enabling
/// proofs of knowledge extraction in the QROM.
pub fn qrom_forking_lemma_ty() -> Expr {
    prop()
}
/// `SISHardness : Nat → Nat → Nat → Prop`
///
/// Short Integer Solution (SIS) hardness: given a matrix A ∈ Z_q^{m×n},
/// find a short nonzero vector z with Az = 0 mod q and ||z|| ≤ β.
/// SIS is hard under worst-case assumptions on short lattice vectors.
pub fn sis_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `SISCryptosystem : Nat → Nat → Type`
///
/// A cryptographic scheme based on SIS: digital signatures whose
/// security reduces to the hardness of finding short solutions.
pub fn sis_cryptosystem_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ModuleLWEDistribution : Nat → Nat → Nat → Type`
///
/// Module-LWE distribution over rank-k modules over Rq:
/// samples (A, b = As + e) where A is a matrix of polynomials,
/// s is a secret vector of polynomials, and e is a small error vector.
pub fn module_lwe_distribution_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `ModuleLWEHardness : Nat → Nat → Prop`
///
/// Module-LWE hardness: the computational indistinguishability problem
/// on module lattices, interpolating between LWE (k=1) and Ring-LWE (k=n).
pub fn module_lwe_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ModuleSISHardness : Nat → Nat → Prop`
///
/// Module-SIS hardness: finding short solutions in module lattices,
/// underlying the security of Dilithium signature scheme.
pub fn module_sis_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `NiederreiterCryptosystem : Nat → Nat → Nat → Type`
///
/// The Niederreiter cryptosystem (dual to McEliece): public key is a
/// parity-check matrix H_pub, encryption maps syndromes to codewords.
/// Parameters: (n, k, t) for code length, dimension, and error capability.
pub fn niederreiter_cryptosystem_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `GoppaCodeDecodability : Nat → Nat → Nat → Prop`
///
/// A binary Goppa code with parameters (n, k, t) can efficiently
/// correct up to t errors using the Patterson algorithm.
pub fn goppa_code_decodability_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), prop())))
}
/// `CodeBasedSignatureScheme : Type`
///
/// A signature scheme whose security reduces to the syndrome decoding
/// problem for random linear codes (NP-hard in the worst case).
pub fn code_based_signature_ty() -> Expr {
    type0()
}
/// `SyndromDecodingHardness : Nat → Nat → Prop`
///
/// The syndrome decoding problem: given a parity-check matrix H and
/// a syndrome s, find a vector e of weight ≤ t with He = s.
/// This problem is NP-complete and underlies code-based cryptography.
pub fn syndrome_decoding_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `XMSSParams : Type`
///
/// XMSS (eXtended Merkle Signature Scheme) parameters:
/// tree height h, security parameter n, Winternitz parameter w.
/// XMSS is a stateful multi-tree hash-based signature scheme (RFC 8391).
pub fn xmss_params_ty() -> Expr {
    type0()
}
/// `XMSSSignature : XMSSParams → Type`
///
/// An XMSS signature: consists of the WOTS+ signature for the message,
/// plus the authentication path in the Merkle tree.
pub fn xmss_signature_ty() -> Expr {
    arrow(xmss_params_ty(), type0())
}
/// `XMSSStateful : XMSSParams → Prop`
///
/// XMSS is stateful: each leaf key can be used exactly once.
/// The signer must maintain a counter of the next unused leaf index.
pub fn xmss_stateful_ty() -> Expr {
    arrow(xmss_params_ty(), prop())
}
/// `LMSParams : Type`
///
/// LMS (Leighton-Micali Signature) parameters: tree height H,
/// hash function output length m, LM-OTS parameters.
/// LMS is the NIST-standardized stateful hash-based signature (SP 800-208).
pub fn lms_params_ty() -> Expr {
    type0()
}
/// `LMSSignature : LMSParams → Type`
///
/// An LMS signature: includes the LM-OTS one-time signature plus
/// the Merkle authentication path to the root.
pub fn lms_signature_ty() -> Expr {
    arrow(lms_params_ty(), type0())
}
/// `HashBasedEUF_CMA : Nat → Prop`
///
/// EUF-CMA security of hash-based signature schemes (XMSS, LMS, SPHINCS+):
/// under the one-wayness and collision-resistance of the hash function,
/// the scheme is existentially unforgeable under chosen message attacks.
pub fn hash_based_euf_cma_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CSIDHPublicKey : Nat → Type`
///
/// CSIDH (Commutative Supersingular Isogeny Diffie-Hellman) public key:
/// a supersingular elliptic curve over F_p together with its j-invariant.
pub fn csidh_public_key_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `CSIDHGroupAction : Nat → Prop`
///
/// The ideal class group cl(O) acts freely and transitively on the set
/// of supersingular curves over F_p with endomorphism ring O.
/// This group action is the hardness foundation of CSIDH.
pub fn csidh_group_action_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SQISignParams : Type`
///
/// SQISign parameters: a supersingular elliptic curve E_0 defined over F_{p^2},
/// together with a special endomorphism computed via quaternion algebra.
/// SQISign is a hash-and-sign isogeny-based signature scheme.
pub fn sqisign_params_ty() -> Expr {
    type0()
}
/// `SQISignEUF_CMA : SQISignParams → Prop`
///
/// SQISign EUF-CMA security under the quaternion endomorphism problem:
/// computing an endomorphism of a given degree is computationally hard.
pub fn sqisign_euf_cma_ty() -> Expr {
    arrow(sqisign_params_ty(), prop())
}
/// `EndomorphismRingHardness : Nat → Prop`
///
/// Computing the endomorphism ring of a supersingular elliptic curve
/// is believed hard even for quantum adversaries, forming the security
/// basis for SQISign and related schemes.
pub fn endomorphism_ring_hardness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MQHardness : Nat → Nat → Prop`
///
/// Multivariate Quadratic (MQ) hardness: solving a system of m quadratic
/// equations in n variables over a finite field is NP-complete.
/// This forms the hardness foundation for multivariate signature schemes.
pub fn mq_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `RainbowSignature : Nat → Nat → Type`
///
/// Rainbow multi-layer signature scheme: a layered variant of the
/// Oil-and-Vinegar scheme with multiple layers of oil and vinegar variables.
/// (Note: certain parameter sets were broken by the Beullens 2022 attack.)
pub fn rainbow_signature_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `UOVSignature : Nat → Nat → Type`
///
/// Unbalanced Oil and Vinegar (UOV) signature scheme:
/// the signer uses the structure of oil/vinegar variable separation to
/// invert the public map efficiently.
pub fn uov_signature_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `UOVSecurity : Nat → Nat → Prop`
///
/// UOV security: breaking UOV requires solving the MQ problem or
/// finding the oil subspace from the public key.
pub fn uov_security_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MultivariateKeyRecoveryHardness : Nat → Prop`
///
/// Key recovery hardness for multivariate schemes: recovering the
/// secret trapdoor (the oil space or private map) from the public key
/// is computationally hard under standard assumptions.
pub fn multivariate_key_recovery_hardness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LatticeZKProof : Nat → Type`
///
/// A zero-knowledge proof system based on lattice problems:
/// the prover demonstrates knowledge of a short witness without
/// revealing it, using rejection sampling over lattice distributions.
pub fn lattice_zk_proof_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `LatticeSigmaProtocol : Nat → Prop`
///
/// A Sigma protocol for lattice relations: 3-move protocol
/// (commit, challenge, respond) where the prover shows knowledge
/// of a short vector satisfying a lattice equation.
pub fn lattice_sigma_protocol_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LatticeSoundness : Nat → Prop`
///
/// Soundness of lattice-based ZK: a cheating prover without knowledge
/// of the short witness cannot succeed except with negligible probability.
pub fn lattice_soundness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `RejectionSamplingLemma : Nat → Prop`
///
/// Rejection sampling (Lyubashevsky 2009): given a distribution D
/// and a target distribution D', output D' by rejection sampling.
/// Used in Dilithium and lattice Sigma protocols to hide the secret.
pub fn rejection_sampling_lemma_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LatticeZKSuccinctness : Nat → Prop`
///
/// Succinct lattice ZK proofs: using polynomial commitments and
/// inner product arguments, lattice proofs can achieve sub-linear
/// communication complexity.
pub fn lattice_zk_succinctness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `BKZAlgorithm : Nat → Nat → Type`
///
/// BKZ (Block Korkin-Zolotarev) lattice basis reduction algorithm
/// with block size β and lattice dimension n. BKZ-β reduces to
/// within O(β^{n/β}) of the shortest vector in polynomial time for fixed β.
pub fn bkz_algorithm_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BKZQualityBound : Nat → Nat → Prop`
///
/// BKZ quality bound: the first basis vector after BKZ-β reduction
/// has length approximately δ^{n-1} · det(L)^{1/n} where
/// δ_β = (β/(2πe))^{1/(2β)} is the Hermite factor.
pub fn bkz_quality_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SievingAlgorithm : Nat → Type`
///
/// Lattice sieving algorithm (e.g., GaussSieve, HashSieve):
/// finds short vectors in an n-dimensional lattice with heuristic
/// running time 2^{cn} for a constant c ≈ 0.292 (BKZ 2.0).
pub fn sieving_algorithm_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SievingComplexity : Nat → Prop`
///
/// Sieving complexity: the best known quantum sieving algorithms
/// run in time 2^{0.265n} (Laarhoven 2015), which determines
/// the security levels of NIST PQC lattice schemes.
pub fn sieving_complexity_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LatticeEnumeration : Nat → Nat → Type`
///
/// Lattice enumeration algorithm: exhaustively searches a pruned
/// tree of lattice vectors to find the shortest vector.
/// Runs in time 2^{O(n log n / 2)} in practice.
pub fn lattice_enumeration_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `GramSchmidtOrthogonalization : Nat → Type`
///
/// Gram-Schmidt orthogonalization of a lattice basis B:
/// computes B* = GSO(B) used in LLL and BKZ reduction quality bounds.
pub fn gram_schmidt_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MLKEMParams : Type`
///
/// ML-KEM (Module Lattice Key Encapsulation Mechanism) parameter set,
/// formerly known as CRYSTALS-Kyber, standardized as FIPS 203.
/// Provides IND-CCA2 KEM security based on Module-LWE.
pub fn ml_kem_params_ty() -> Expr {
    type0()
}
/// `MLKEMCorrectness : MLKEMParams → Prop`
///
/// ML-KEM correctness: decapsulation always recovers the shared secret
/// that was encapsulated, i.e., Dec(dk, Enc(ek, r)) = K.
pub fn ml_kem_correctness_ty() -> Expr {
    arrow(ml_kem_params_ty(), prop())
}
/// `MLDSAParams : Type`
///
/// ML-DSA (Module Lattice Digital Signature Algorithm) parameter set,
/// formerly CRYSTALS-Dilithium, standardized as FIPS 204.
/// Provides EUF-CMA security based on Module-LWE and Module-SIS.
pub fn ml_dsa_params_ty() -> Expr {
    type0()
}
/// `MLDSADetachedSignature : MLDSAParams → Type`
///
/// An ML-DSA detached signature: does not include the message,
/// allowing verification with a separately transmitted message.
pub fn ml_dsa_detached_signature_ty() -> Expr {
    arrow(ml_dsa_params_ty(), type0())
}
/// `SLHDSAParams : Type`
///
/// SLH-DSA (Stateless Hash-based Digital Signature Algorithm) parameter set,
/// formerly SPHINCS+, standardized as FIPS 205.
/// Provides hash-based EUF-CMA security without number-theoretic assumptions.
pub fn slh_dsa_params_ty() -> Expr {
    type0()
}
/// `SLHDSAStatelesness : SLHDSAParams → Prop`
///
/// SLH-DSA statelessness: no signing state needs to be maintained across calls.
/// The randomness is derived pseudorandomly from the secret key and message.
pub fn slh_dsa_statelessness_ty() -> Expr {
    arrow(slh_dsa_params_ty(), prop())
}
/// `NISTPQCStandardSecurity : Type`
///
/// The security levels of NIST PQC standards:
/// Level 1 ≥ AES-128, Level 3 ≥ AES-192, Level 5 ≥ AES-256
/// (classical and quantum security).
pub fn nist_pqc_security_level_ty() -> Expr {
    type0()
}
/// `QROMReduction : Nat → Prop`
///
/// QROM reduction: a security proof that reduces the advantage of any
/// quantum adversary attacking a scheme to the difficulty of solving
/// an underlying hard problem, when oracles are quantum-accessible.
pub fn qrom_reduction_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `OnewayToHidingLemma : Prop`
///
/// One-way-to-hiding (O2H) lemma (Ambainis-Hamburg-Unruh 2019):
/// if H and H' differ at one point, any quantum algorithm distinguishing
/// them gives a quantum algorithm finding the differing point.
/// Key tool for QROM proofs of OAEP and Fujisaki-Okamoto transforms.
pub fn o2h_lemma_ty() -> Expr {
    prop()
}
/// `FujisakiOkamotoQROM : Prop`
///
/// Fujisaki-Okamoto transform in the QROM (Hofheinz-Hövelmanns-Kiltz 2017):
/// the FO transform converts an IND-CPA KEM into an IND-CCA2 KEM,
/// and the proof goes through in the QROM using O2H or compressed oracles.
pub fn fujisaki_okamoto_qrom_ty() -> Expr {
    prop()
}
/// `QuantumSecureHashFunction : Nat → Prop`
///
/// A quantum-secure hash function with output length n remains
/// collision-resistant against BHT attack needing O(2^{n/3}) quantum queries.
pub fn quantum_secure_hash_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `HybridKEM : Type`
///
/// A hybrid KEM combining a classical KEM (e.g., ECDH) and a PQC KEM
/// (e.g., ML-KEM): the shared secret is derived from both, so security
/// holds if either component is secure.
pub fn hybrid_kem_ty() -> Expr {
    type0()
}
/// `HybridKEMSecurity : HybridKEM → Prop`
///
/// Hybrid KEM security: the hybrid is IND-CCA2 if at least one of the
/// component KEMs is IND-CCA2. This covers the transition period
/// when quantum attacks are not yet practical but must be anticipated.
pub fn hybrid_kem_security_ty() -> Expr {
    arrow(hybrid_kem_ty(), prop())
}
/// `PQCMigrationStrategy : Type`
///
/// A PQC migration strategy describes the phased deployment of
/// post-quantum cryptography: hybrid deployment, legacy support,
/// algorithm agility, and performance considerations.
pub fn pqc_migration_strategy_ty() -> Expr {
    type0()
}
/// `CryptoAgility : Type`
///
/// Crypto-agility: the ability to switch cryptographic algorithms
/// without changing the overall system design, crucial for PQC migration.
pub fn crypto_agility_ty() -> Expr {
    type0()
}
/// `TLSPQCExtension : Prop`
///
/// The TLS 1.3 PQC extension: hybrid key exchange combining X25519
/// with ML-KEM-768, providing post-quantum security for TLS connections
/// while maintaining backward compatibility.
pub fn tls_pqc_extension_ty() -> Expr {
    prop()
}
/// Register all post-quantum cryptography axioms into the kernel environment.
pub fn build_post_quantum_crypto_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("PQC.Lattice", lattice_ty()),
        ("PQC.LatticeVector", lattice_vector_ty()),
        ("PQC.ShortestVector", shortest_vector_ty()),
        ("PQC.ClosestVector", closest_vector_ty()),
        ("PQC.GaussianHeuristic", gaussian_heuristic_ty()),
        ("PQC.LWEDistribution", lwe_distribution_ty()),
        ("PQC.LWEHardness", lwe_hardness_ty()),
        ("PQC.LWEEncryption", lwe_encryption_ty()),
        ("PQC.LWERegevTheorem", lwe_regev_theorem_ty()),
        ("PQC.RingLWEDistribution", rlwe_distribution_ty()),
        ("PQC.RingLWEHardness", rlwe_hardness_ty()),
        ("PQC.RingLWEToLWEReduction", rlwe_to_lwe_reduction_ty()),
        ("PQC.NTRUParams", ntru_params_ty()),
        ("PQC.NTRUPublicKey", ntru_public_key_ty()),
        ("PQC.NTRUHardness", ntru_hardness_ty()),
        ("PQC.NTRUCorrectness", ntru_correctness_ty()),
        ("PQC.KyberParams", kyber_params_ty()),
        ("PQC.KyberKeyPair", kyber_key_pair_ty()),
        ("PQC.KyberIND_CCA2", kyber_ind_cca2_ty()),
        ("PQC.KyberMLWEHardness", kyber_mlwe_hardness_ty()),
        ("PQC.DilithiumParams", dilithium_params_ty()),
        ("PQC.DilithiumSignature", dilithium_signature_ty()),
        ("PQC.DilithiumEUF_CMA", dilithium_euf_cma_ty()),
        ("PQC.FalconParams", falcon_params_ty()),
        ("PQC.FalconSignature", falcon_signature_ty()),
        ("PQC.FalconEUF_CMA", falcon_euf_cma_ty()),
        ("PQC.LamportParams", lamport_params_ty()),
        ("PQC.LamportOneTimeUse", lamport_one_time_use_ty()),
        ("PQC.MerkleTree", merkle_tree_ty()),
        ("PQC.MerkleSignature", merkle_signature_ty()),
        (
            "PQC.MerkleSignatureCorrectness",
            merkle_signature_correctness_ty(),
        ),
        ("PQC.SPHINCSPlusParams", sphincs_plus_params_ty()),
        ("PQC.SPHINCSPlusStateless", sphincs_plus_stateless_ty()),
        ("PQC.SPHINCSPlusEUF_CMA", sphincs_plus_euf_cma_ty()),
        ("PQC.BinaryGoppaCode", binary_goppa_code_ty()),
        ("PQC.McEliecePublicKey", mceliece_public_key_ty()),
        ("PQC.McElieceHardness", mceliece_hardness_ty()),
        ("PQC.McElieceCryptosystem", mceliece_cryptosystem_ty()),
        ("PQC.SupersingularCurve", supersingular_curve_ty()),
        ("PQC.Isogeny", isogeny_ty()),
        ("PQC.SIDHKeyExchange", sidh_key_exchange_ty()),
        ("PQC.SIDHBrokenByCastryckDecru", sidh_broken_ty()),
        ("PQC.CSIDHHardness", csidh_hardness_ty()),
        ("PQC.QuantumRandomOracle", quantum_random_oracle_ty()),
        ("PQC.QROMSecurity", qrom_security_ty()),
        ("PQC.FiatShamirQROM", fiat_shamir_qrom_ty()),
        ("PQC.QROMForkingLemma", qrom_forking_lemma_ty()),
        ("PQC.SISHardness", sis_hardness_ty()),
        ("PQC.SISCryptosystem", sis_cryptosystem_ty()),
        ("PQC.ModuleLWEDistribution", module_lwe_distribution_ty()),
        ("PQC.ModuleLWEHardness", module_lwe_hardness_ty()),
        ("PQC.ModuleSISHardness", module_sis_hardness_ty()),
        (
            "PQC.NiederreiterCryptosystem",
            niederreiter_cryptosystem_ty(),
        ),
        ("PQC.GoppaCodeDecodability", goppa_code_decodability_ty()),
        ("PQC.CodeBasedSignature", code_based_signature_ty()),
        (
            "PQC.SyndromeDecodingHardness",
            syndrome_decoding_hardness_ty(),
        ),
        ("PQC.XMSSParams", xmss_params_ty()),
        ("PQC.XMSSSignature", xmss_signature_ty()),
        ("PQC.XMSSStateful", xmss_stateful_ty()),
        ("PQC.LMSParams", lms_params_ty()),
        ("PQC.LMSSignature", lms_signature_ty()),
        ("PQC.HashBasedEUF_CMA", hash_based_euf_cma_ty()),
        ("PQC.CSIDHPublicKey", csidh_public_key_ty()),
        ("PQC.CSIDHGroupAction", csidh_group_action_ty()),
        ("PQC.SQISignParams", sqisign_params_ty()),
        ("PQC.SQISignEUF_CMA", sqisign_euf_cma_ty()),
        (
            "PQC.EndomorphismRingHardness",
            endomorphism_ring_hardness_ty(),
        ),
        ("PQC.MQHardness", mq_hardness_ty()),
        ("PQC.RainbowSignature", rainbow_signature_ty()),
        ("PQC.UOVSignature", uov_signature_ty()),
        ("PQC.UOVSecurity", uov_security_ty()),
        (
            "PQC.MultivariateKeyRecoveryHardness",
            multivariate_key_recovery_hardness_ty(),
        ),
        ("PQC.LatticeZKProof", lattice_zk_proof_ty()),
        ("PQC.LatticeSigmaProtocol", lattice_sigma_protocol_ty()),
        ("PQC.LatticeSoundness", lattice_soundness_ty()),
        ("PQC.RejectionSamplingLemma", rejection_sampling_lemma_ty()),
        ("PQC.LatticeZKSuccinctness", lattice_zk_succinctness_ty()),
        ("PQC.BKZAlgorithm", bkz_algorithm_ty()),
        ("PQC.BKZQualityBound", bkz_quality_bound_ty()),
        ("PQC.SievingAlgorithm", sieving_algorithm_ty()),
        ("PQC.SievingComplexity", sieving_complexity_ty()),
        ("PQC.LatticeEnumeration", lattice_enumeration_ty()),
        ("PQC.GramSchmidt", gram_schmidt_ty()),
        ("PQC.MLKEMParams", ml_kem_params_ty()),
        ("PQC.MLKEMCorrectness", ml_kem_correctness_ty()),
        ("PQC.MLDSAParams", ml_dsa_params_ty()),
        ("PQC.MLDSADetachedSignature", ml_dsa_detached_signature_ty()),
        ("PQC.SLHDSAParams", slh_dsa_params_ty()),
        ("PQC.SLHDSAStatelessness", slh_dsa_statelessness_ty()),
        ("PQC.NISTPQCSecurityLevel", nist_pqc_security_level_ty()),
        ("PQC.QROMReduction", qrom_reduction_ty()),
        ("PQC.O2HLemma", o2h_lemma_ty()),
        ("PQC.FujisakiOkamotoQROM", fujisaki_okamoto_qrom_ty()),
        ("PQC.QuantumSecureHash", quantum_secure_hash_ty()),
        ("PQC.HybridKEM", hybrid_kem_ty()),
        ("PQC.HybridKEMSecurity", hybrid_kem_security_ty()),
        ("PQC.PQCMigrationStrategy", pqc_migration_strategy_ty()),
        ("PQC.CryptoAgility", crypto_agility_ty()),
        ("PQC.TLSPQCExtension", tls_pqc_extension_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
/// Simple 2-to-1 hash for Merkle tree construction.
pub(super) fn merkle_hash(a: u64, b: u64) -> u64 {
    a.wrapping_mul(0x9e3779b97f4a7c15)
        .wrapping_add(b)
        .rotate_left(17)
        .wrapping_add(0x6c62272e07bb0142)
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_post_quantum_crypto_env(&mut env);
        assert!(
            result.is_ok(),
            "build_post_quantum_crypto_env failed: {:?}",
            result
        );
    }
    #[test]
    fn test_kyber_variants() {
        assert_eq!(KyberVariant::Kyber512.k(), 2);
        assert_eq!(KyberVariant::Kyber768.k(), 3);
        assert_eq!(KyberVariant::Kyber1024.k(), 4);
        assert_eq!(KyberVariant::Kyber512.security_bits(), 128);
        assert_eq!(KyberVariant::Kyber768.security_bits(), 192);
        assert_eq!(KyberVariant::Kyber1024.security_bits(), 256);
    }
    #[test]
    fn test_dilithium_variants() {
        let (k2, l2) = DilithiumVariant::Dilithium2.dimensions();
        assert_eq!((k2, l2), (4, 4));
        let (k5, l5) = DilithiumVariant::Dilithium5.dimensions();
        assert_eq!((k5, l5), (8, 7));
        assert!(
            DilithiumVariant::Dilithium3.signature_bytes()
                > DilithiumVariant::Dilithium2.signature_bytes()
        );
    }
    #[test]
    fn test_falcon_variants() {
        assert_eq!(FalconVariant::Falcon512.degree(), 512);
        assert_eq!(FalconVariant::Falcon1024.degree(), 1024);
        assert_eq!(FalconVariant::Falcon512.modulus(), 12289);
        assert_eq!(FalconVariant::Falcon1024.modulus(), 12289);
    }
    #[test]
    fn test_ring_poly_add_mul() {
        let _n = 4;
        let q = 17i64;
        let a = RingPoly {
            coeffs: vec![1, 2, 3, 4],
            q,
        };
        let b = RingPoly {
            coeffs: vec![4, 3, 2, 1],
            q,
        };
        let sum = a.add(&b);
        assert_eq!(
            sum.coeffs,
            vec![5, 5, 5, 5],
            "Polynomial addition should be coefficient-wise"
        );
        let diff = a.sub(&b);
        assert_eq!(diff.coeffs[0], -3);
        assert_eq!(diff.coeffs[2], 1);
        let one = RingPoly {
            coeffs: vec![0, 1, 0, 0],
            q,
        };
        let two = RingPoly {
            coeffs: vec![0, 1, 0, 0],
            q,
        };
        let prod = one.mul(&two);
        assert_eq!(
            prod.coeffs[2], 1,
            "x * x = x^2 coefficient at index 2 should be 1"
        );
    }
    #[test]
    fn test_lamport_sign_verify() {
        let lamport = ToyLamport { n: 8 };
        let kp = lamport.keygen(12345);
        let msg_bits: Vec<u8> = vec![1, 0, 1, 1, 0, 0, 1, 0];
        let sig = lamport.sign(&kp, &msg_bits);
        assert!(
            lamport.verify(&kp, &msg_bits, &sig),
            "Valid Lamport signature should verify"
        );
        let mut bad_bits = msg_bits.clone();
        bad_bits[3] = 1 - bad_bits[3];
        assert!(
            !lamport.verify(&kp, &bad_bits, &sig),
            "Tampered message should not verify"
        );
    }
    #[test]
    fn test_merkle_tree() {
        let leaves = [100u64, 200, 300, 400];
        let tree = ToyMerkleTree::build(leaves);
        for i in 0..4 {
            let path = tree.auth_path(i);
            assert!(
                tree.verify_leaf(leaves[i], i, path),
                "Merkle auth path verification failed for leaf {}",
                i
            );
        }
        let path0 = tree.auth_path(0);
        assert!(
            !tree.verify_leaf(999, 0, path0),
            "Wrong leaf value should fail Merkle verification"
        );
    }
    #[test]
    fn test_axioms_registered() {
        let mut env = Environment::new();
        build_post_quantum_crypto_env(&mut env)
            .expect("build_post_quantum_crypto_env should succeed");
        let expected = [
            "PQC.LWEHardness",
            "PQC.RingLWEHardness",
            "PQC.NTRUHardness",
            "PQC.KyberIND_CCA2",
            "PQC.DilithiumEUF_CMA",
            "PQC.FalconEUF_CMA",
            "PQC.SPHINCSPlusEUF_CMA",
            "PQC.McElieceHardness",
            "PQC.SIDHBrokenByCastryckDecru",
            "PQC.QROMForkingLemma",
        ];
        for name in &expected {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "Expected axiom '{}' not found in environment",
                name
            );
        }
    }
    #[test]
    fn test_new_axioms_registered() {
        let mut env = Environment::new();
        build_post_quantum_crypto_env(&mut env)
            .expect("build_post_quantum_crypto_env should succeed");
        let expected = [
            "PQC.SISHardness",
            "PQC.ModuleLWEHardness",
            "PQC.ModuleSISHardness",
            "PQC.NiederreiterCryptosystem",
            "PQC.SyndromeDecodingHardness",
            "PQC.XMSSParams",
            "PQC.LMSParams",
            "PQC.HashBasedEUF_CMA",
            "PQC.CSIDHGroupAction",
            "PQC.SQISignEUF_CMA",
            "PQC.EndomorphismRingHardness",
            "PQC.MQHardness",
            "PQC.UOVSecurity",
            "PQC.LatticeZKProof",
            "PQC.RejectionSamplingLemma",
            "PQC.BKZAlgorithm",
            "PQC.SievingComplexity",
            "PQC.GramSchmidt",
            "PQC.MLKEMParams",
            "PQC.MLDSAParams",
            "PQC.SLHDSAParams",
            "PQC.NISTPQCSecurityLevel",
            "PQC.O2HLemma",
            "PQC.FujisakiOkamotoQROM",
            "PQC.HybridKEM",
            "PQC.HybridKEMSecurity",
            "PQC.TLSPQCExtension",
        ];
        for name in &expected {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "Expected axiom '{}' not found in environment",
                name
            );
        }
    }
    #[test]
    fn test_lwe_sample_generator() {
        let mut gen = LWESampleGenerator::new(8, 97, 3, 42);
        let s: Vec<i64> = vec![1, 0, 1, 1, 0, 0, 1, 0];
        let samples = gen.sample_many(&s, 10);
        assert_eq!(samples.len(), 10);
        for (a, bval) in &samples {
            assert_eq!(a.len(), 8);
            assert!(a.iter().all(|&x| x >= 0 && x < 97));
            assert!(*bval >= 0 && *bval < 97);
        }
    }
    #[test]
    fn test_lattice_basis_reducer() {
        let reducer = LatticeBasisReducer::new(0.75);
        let b0 = [3i64, 1];
        let b1 = [1i64, 3];
        let (r0, r1) = reducer.reduce_2d(b0, b1);
        let norm_orig = LatticeBasisReducer::norm_sq(b0) + LatticeBasisReducer::norm_sq(b1);
        let norm_red = LatticeBasisReducer::norm_sq(r0) + LatticeBasisReducer::norm_sq(r1);
        assert!(norm_red <= norm_orig, "Reduced basis should not be longer");
        assert!(reducer.is_lll_reduced_2d(r0, r1));
    }
    #[test]
    fn test_hash_based_signature_wots() {
        let wots = HashBasedSignature::new(4, 8);
        let kp = wots.keygen(999);
        let msg: Vec<usize> = vec![0, 1, 2, 3, 0, 1, 2, 3];
        let sig = wots.sign(&kp, &msg);
        assert!(
            wots.verify(&kp, &msg, &sig),
            "WOTS+ signature should verify"
        );
        let bad_msg: Vec<usize> = vec![0, 1, 2, 3, 0, 1, 2, 0];
        assert!(
            !wots.verify(&kp, &bad_msg, &sig),
            "Wrong message should not verify"
        );
    }
    #[test]
    fn test_modular_lattice_arithmetic() {
        let arith = ModularLatticeArithmetic::new(4, 17);
        let a = vec![1i64, 2, 3, 4];
        let b = vec![4i64, 3, 2, 1];
        let sum = arith.poly_add(&a, &b);
        assert_eq!(sum, vec![5, 5, 5, 5]);
        let diff = arith.poly_sub(&a, &b);
        assert_eq!(diff[0], arith.reduce(1 - 4));
        assert_eq!(diff[2], arith.reduce(3 - 2));
        let x_poly = vec![0i64, 1, 0, 0];
        let prod = arith.poly_mul(&x_poly, &x_poly);
        assert_eq!(prod[2], 1);
        assert_eq!(prod[0], 0);
        let small = vec![0i64, 1, -1, 2];
        assert_eq!(arith.inf_norm(&small), 2);
        assert!(arith.is_small(&small, 2));
        assert!(!arith.is_small(&small, 1));
        let l2 = arith.l2_norm_sq(&small);
        assert_eq!(l2, 0 + 1 + 1 + 4);
    }
    #[test]
    fn test_lwe_sample_consistency() {
        let n = 8;
        let q = 97i64;
        let b = 3i64;
        let mut gen = LWESampleGenerator::new(n, q, b, 7);
        let s: Vec<i64> = vec![1, 0, 0, 1, 0, 1, 1, 0];
        let lwe = ToyLWE { n, q, b };
        for m_bit in 0u8..=1 {
            let mask: Vec<bool> = (0..n).map(|i| (i % 3) == 0).collect();
            let a_rows: Vec<Vec<i64>> = (0..n).map(|_| gen.sample_a()).collect();
            let b_vec: Vec<i64> = a_rows
                .iter()
                .map(|row| {
                    let inner: i64 = row
                        .iter()
                        .zip(&s)
                        .map(|(ai, si)| ai * si)
                        .sum::<i64>()
                        .rem_euclid(q);
                    (inner + gen.sample_error()).rem_euclid(q)
                })
                .collect();
            let (c1, c2) = lwe.encrypt(&a_rows, &b_vec, m_bit, &mask);
            let decrypted = lwe.decrypt(&c1, c2, &s);
            assert_eq!(decrypted, m_bit, "LWE decrypt should recover bit {}", m_bit);
        }
    }
}
#[cfg(test)]
mod extended_pqc_tests {
    use super::*;
    #[test]
    fn test_dilithium_params() {
        let d2 = DilithiumParams::dilithium2();
        assert_eq!(d2.security_level, 2);
        assert_eq!(d2.n, 256);
        assert!(d2.public_key_size() > 0);
    }
    #[test]
    fn test_falcon_params() {
        let f512 = FalconParams::falcon512();
        assert_eq!(f512.signature_size, 666);
        assert!(f512.description().contains("Falcon-512"));
    }
    #[test]
    fn test_sphincs() {
        let s = SphincsParams::sha2_128s();
        assert!(s.is_stateless());
        assert_eq!(s.n, 16);
    }
    #[test]
    fn test_mceliece() {
        let m = McElieceParams::kem_348864();
        assert_eq!(m.error_capability(), 64);
        assert!(m.rate() > 0.7 && m.rate() < 0.9);
    }
    #[test]
    fn test_isogeny_crypto() {
        let c = IsogenyCrypto::csidh_512();
        assert!(c.is_post_sike);
        assert!(IsogenyCrypto::note().contains("SIKE"));
    }
}
