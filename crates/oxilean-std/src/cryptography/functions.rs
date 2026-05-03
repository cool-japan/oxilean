//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    DiffieHellmanSim, HashChain, ModularArithmetic, RsaKeyGen, ShamirSecretShare, ToyDiffieHellman,
    ToyRsa,
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
/// `List Nat` — used to represent byte arrays / messages / hashes
pub fn bytes_ty() -> Expr {
    app(cst("List"), nat_ty())
}
/// `HashFunction : (List Nat) → (List Nat)` — a hash function mapping messages to digests.
pub fn hash_function_ty() -> Expr {
    arrow(bytes_ty(), bytes_ty())
}
/// `SymmetricKey : List Nat` — a symmetric key (byte sequence).
pub fn symmetric_key_ty() -> Expr {
    bytes_ty()
}
/// `PublicKey : Type` — an abstract public key type.
pub fn public_key_ty() -> Expr {
    type0()
}
/// `PrivateKey : Type` — an abstract private key type.
pub fn private_key_ty() -> Expr {
    type0()
}
/// `Signature : List Nat` — a digital signature (byte sequence).
pub fn signature_ty() -> Expr {
    bytes_ty()
}
/// `Ciphertext : List Nat` — an encrypted message (byte sequence).
pub fn ciphertext_ty() -> Expr {
    bytes_ty()
}
/// `Group : Type` — a cyclic group used in discrete-logarithm-based cryptography.
pub fn group_ty() -> Expr {
    type0()
}
/// `RsaParams : Type` — an RSA parameter triple (n, e, d).
pub fn rsa_params_ty() -> Expr {
    type0()
}
/// `OneWayFunction : (List Nat) → (List Nat)`
///
/// A function that is computationally hard to invert (pre-image resistance).
/// OWFs are the foundational assumption underlying most symmetric cryptography.
pub fn one_way_function_ty() -> Expr {
    arrow(bytes_ty(), bytes_ty())
}
/// `CollisionResistant : HashFunction → Prop`
///
/// A hash function H is collision-resistant if it is computationally infeasible
/// to find distinct messages x ≠ y such that H(x) = H(y).
pub fn collision_resistant_ty() -> Expr {
    arrow(hash_function_ty(), prop())
}
/// `IndCpa : Prop`
///
/// IND-CPA (indistinguishability under chosen-plaintext attack) security.
/// A public-key encryption scheme is IND-CPA secure if no polynomial-time
/// adversary can distinguish encryptions of two chosen plaintexts with
/// advantage non-negligibly better than 1/2.
pub fn ind_cpa_ty() -> Expr {
    prop()
}
/// `IndCca : Prop`
///
/// IND-CCA (indistinguishability under chosen-ciphertext attack) security.
/// Stronger than IND-CPA: the adversary also has access to a decryption oracle
/// (but cannot query the challenge ciphertext itself).
pub fn ind_cca_ty() -> Expr {
    prop()
}
/// `EufCma : Prop`
///
/// EUF-CMA (existential unforgeability under chosen-message attack) security.
/// A signature scheme is EUF-CMA secure if no polynomial-time adversary can
/// produce a valid signature on a new message after seeing signatures on
/// polynomially many chosen messages.
pub fn euf_cma_ty() -> Expr {
    prop()
}
/// `DiscreteLogHard : Prop`
///
/// The discrete logarithm problem is computationally hard in the group G:
/// given g and g^x, it is infeasible to recover x in polynomial time.
pub fn discrete_log_hard_ty() -> Expr {
    prop()
}
/// `RsaHard : Prop`
///
/// The RSA hardness assumption: given (n, e, c = m^e mod n), it is
/// computationally infeasible to recover m without knowing the factorization
/// of n (or equivalently, the private exponent d).
pub fn rsa_hard_ty() -> Expr {
    prop()
}
/// `RsaCorrectness : Prop`
///
/// For properly chosen RSA parameters (n = p*q, e*d ≡ 1 mod λ(n)),
/// decryption is the inverse of encryption: (m^e)^d ≡ m (mod n).
pub fn rsa_correctness_ty() -> Expr {
    prop()
}
/// `DhCorrectness : Prop`
///
/// Diffie-Hellman correctness: in a cyclic group G with generator g,
/// (g^a)^b = (g^b)^a, so both parties derive the same shared secret.
pub fn dh_correctness_ty() -> Expr {
    prop()
}
/// `BirthdayBound : Prop`
///
/// Birthday paradox bound for hash collision probability:
/// after q queries to a random oracle with n-bit output, the collision
/// probability is approximately q*(q-1)/2^(n+1) ≈ q²/2^n.
pub fn birthday_bound_ty() -> Expr {
    prop()
}
/// `TrapdoorFunction : Type`
///
/// A trapdoor one-way function: a function f that is easy to compute but
/// computationally hard to invert without the trapdoor information t.
/// Given t, inversion becomes efficient: f^{-1}(t, y) = x such that f(x) = y.
/// RSA and discrete exponentiation are canonical trapdoor functions.
pub fn trapdoor_function_ty() -> Expr {
    type0()
}
/// `TrapdoorInvertible : TrapdoorFunction → Prop`
///
/// Correctness of a trapdoor function: given the trapdoor, inversion succeeds
/// with probability 1. Without the trapdoor, inversion succeeds with only
/// negligible probability in the security parameter.
pub fn trapdoor_invertible_ty() -> Expr {
    arrow(trapdoor_function_ty(), prop())
}
/// `GoldreichLevinHardCoreBit : OneWayFunction → Prop`
///
/// Goldreich-Levin theorem (1989): if f is a one-way function then
/// b(x, r) = ⟨x, r⟩ mod 2 is a hard-core bit for f(x) paired with r.
/// That is, given f(x) and r, no efficient algorithm can predict b(x, r)
/// with advantage non-negligibly better than 1/2.
pub fn goldreich_levin_ty() -> Expr {
    arrow(one_way_function_ty(), prop())
}
/// `PseudorandomGenerator : Nat → Nat → Type`
///
/// A pseudorandom generator (PRG) stretching l-bit seeds to p-bit outputs (p > l).
/// No polynomial-time distinguisher can tell PRG output from truly random bits
/// with non-negligible advantage.
pub fn prg_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `PrgSecure : PseudorandomGenerator → Prop`
///
/// Computational indistinguishability: PRG(s) for uniform s is computationally
/// indistinguishable from uniform over {0,1}^p.
pub fn prg_secure_ty() -> Expr {
    arrow(app2(cst("PRG"), nat_ty(), nat_ty()), prop())
}
/// `PseudorandomFunction : Nat → Nat → Type`
///
/// A pseudorandom function family (PRF): a family F_k: {0,1}^n → {0,1}^m
/// parameterized by key k ∈ {0,1}^κ. No polynomial-time oracle adversary
/// can distinguish F_k from a truly random function.
pub fn prf_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `PrfSecure : PseudorandomFunction → Prop`
///
/// PRF security: the PRF family is computationally indistinguishable from
/// a random oracle, even for adaptive queries.
pub fn prf_secure_ty() -> Expr {
    arrow(app2(cst("PRF"), nat_ty(), nat_ty()), prop())
}
/// `PreimageResistant : HashFunction → Prop`
///
/// Second-preimage resistance: given x, it is computationally infeasible
/// to find x' ≠ x such that H(x') = H(x). This is a weaker property
/// than collision resistance but stronger than one-wayness.
pub fn preimage_resistant_ty() -> Expr {
    arrow(hash_function_ty(), prop())
}
/// `SecondPreimageResistant : HashFunction → Prop`
///
/// Second preimage resistance (target collision resistance): given a random x,
/// no efficient adversary can find x' ≠ x with H(x') = H(x).
/// Strictly implied by collision resistance.
pub fn second_preimage_resistant_ty() -> Expr {
    arrow(hash_function_ty(), prop())
}
/// `RandomOracleModel : Type`
///
/// The random oracle model (ROM): idealized model where the hash function H
/// is replaced by a truly random oracle. Security proofs in the ROM give
/// strong heuristic guarantees, though ROM ≠ standard model in general.
pub fn random_oracle_model_ty() -> Expr {
    type0()
}
/// `SignatureScheme : Type`
///
/// A digital signature scheme: triple (KeyGen, Sign, Verify) where
/// KeyGen() → (pk, sk), Sign(sk, m) → σ, Verify(pk, m, σ) → Bool.
pub fn signature_scheme_ty() -> Expr {
    type0()
}
/// `SignatureCorrectness : SignatureScheme → Prop`
///
/// Correctness: for all (pk, sk) ← KeyGen() and all messages m,
/// Verify(pk, m, Sign(sk, m)) = true.
pub fn signature_correctness_ty() -> Expr {
    arrow(signature_scheme_ty(), prop())
}
/// `EcdsaSignature : Type`
///
/// ECDSA (Elliptic Curve Digital Signature Algorithm) signature: a pair
/// (r, s) ∈ Z_n × Z_n* derived from an elliptic curve point and the
/// hash of the message.
pub fn ecdsa_signature_ty() -> Expr {
    type0()
}
/// `EcdsaCorrectness : Prop`
///
/// ECDSA correctness: if (r, s) is a valid ECDSA signature for message m
/// under key sk, then the verification algorithm with corresponding pk accepts.
pub fn ecdsa_correctness_ty() -> Expr {
    prop()
}
/// `EcdsaUnforgeability : Prop`
///
/// ECDSA unforgeability under ECDLP hardness: given a valid ECDSA implementation,
/// forging signatures is reducible to solving the elliptic curve discrete
/// logarithm problem, which is assumed computationally intractable.
pub fn ecdsa_unforgeability_ty() -> Expr {
    prop()
}
/// `PublicKeyEncScheme : Type`
///
/// A public-key encryption scheme: (KeyGen, Enc, Dec) where
/// KeyGen() → (pk, sk), Enc(pk, m) → c, Dec(sk, c) → m.
pub fn pke_scheme_ty() -> Expr {
    type0()
}
/// `PkeCorrectness : PublicKeyEncScheme → Prop`
///
/// PKE correctness: Dec(sk, Enc(pk, m)) = m for all messages m and valid key pairs.
pub fn pke_correctness_ty() -> Expr {
    arrow(pke_scheme_ty(), prop())
}
/// `IndCca2 : Prop`
///
/// IND-CCA2 (indistinguishability under adaptive chosen-ciphertext attack).
/// The strongest standard notion of PKE security: the adversary can adaptively
/// query a decryption oracle both before and after receiving the challenge
/// ciphertext (but not on the challenge itself).
pub fn ind_cca2_ty() -> Expr {
    prop()
}
/// `RsaOaep : Type`
///
/// RSA-OAEP (Optimal Asymmetric Encryption Padding): a padding scheme that
/// transforms textbook RSA into an IND-CCA2 secure scheme in the random oracle
/// model (Bellare-Rogaway 1994 / PKCS#1 v2.1 / RFC 3447).
pub fn rsa_oaep_ty() -> Expr {
    type0()
}
/// `RsaOaepIndCca2 : Prop`
///
/// RSA-OAEP is IND-CCA2 secure in the random oracle model, assuming the RSA
/// problem is hard. This was proven by Fujisaki, Okamoto, Pointcheval, Stern (2001).
pub fn rsa_oaep_ind_cca2_ty() -> Expr {
    prop()
}
/// `EllipticCurve : Nat → Type`
///
/// An elliptic curve E over a finite field F_p, parameterized by the field
/// characteristic p. Points (x, y) satisfy y² = x³ + ax + b (Weierstrass form).
pub fn elliptic_curve_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `EllipticCurvePoint : EllipticCurve → Type`
///
/// A point on an elliptic curve, including the point at infinity O (identity).
pub fn ec_point_ty() -> Expr {
    arrow(app(cst("EllipticCurve"), nat_ty()), type0())
}
/// `EcGroupLaw : EllipticCurve → Prop`
///
/// The elliptic curve group law: points on E form an abelian group under
/// the chord-and-tangent addition rule, with the point at infinity as identity.
pub fn ec_group_law_ty() -> Expr {
    arrow(app(cst("EllipticCurve"), nat_ty()), prop())
}
/// `EcdlpHard : Nat → Prop`
///
/// ECDLP hardness: the elliptic curve discrete logarithm problem is
/// computationally hard over a curve of the given prime order.
/// Given P and Q = kP, finding k is infeasible for well-chosen curves.
/// The best known algorithms run in O(sqrt(p)) time (baby-step giant-step / Pollard rho).
pub fn ecdlp_hard_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `EcdhCorrectness : Prop`
///
/// ECDH (Elliptic Curve Diffie-Hellman) key exchange correctness:
/// Alice computes a*(b*G) and Bob computes b*(a*G); both equal ab*G.
/// The shared secret is derived from this common point.
pub fn ecdh_correctness_ty() -> Expr {
    prop()
}
/// `EcdhHardness : Prop`
///
/// ECDH hardness (ECDHP): given G, aG, bG, computing abG is computationally
/// hard, assuming ECDLP is hard. The computational ECDH problem reduces
/// to ECDLP on most curves.
pub fn ecdh_hardness_ty() -> Expr {
    prop()
}
/// `BilinearMap : Type`
///
/// A bilinear pairing e: G1 × G2 → GT, where G1, G2, GT are cyclic groups
/// of prime order q. The map satisfies:
///   - Bilinearity: e(aP, bQ) = e(P, Q)^{ab}
///   - Non-degeneracy: e(P, Q) ≠ 1_GT for generators P, Q
/// Used in IBE, BLS signatures, zk-SNARKs, etc.
pub fn bilinear_map_ty() -> Expr {
    type0()
}
/// `PairingBilinearity : BilinearMap → Prop`
///
/// Bilinearity axiom: e(aP, bQ) = e(P, Q)^{ab} for all a, b ∈ Z_q.
pub fn pairing_bilinearity_ty() -> Expr {
    arrow(bilinear_map_ty(), prop())
}
/// `BdhHard : Prop`
///
/// BDH hardness (Bilinear Diffie-Hellman): given (P, aP, bP, cP) in G,
/// computing e(P, P)^{abc} is computationally hard. This is the foundation
/// of pairing-based IBE (Boneh-Franklin 2001).
pub fn bdh_hard_ty() -> Expr {
    prop()
}
/// `BdddhHard : Prop`
///
/// Decisional BDH hardness (DBDH): distinguishing e(P,P)^{abc} from a
/// random element in GT is computationally hard. Used in BLS signature security proofs.
pub fn bdddh_hard_ty() -> Expr {
    prop()
}
/// `BlsSignature : BilinearMap → Type`
///
/// BLS (Boneh-Lynn-Shacham) signature: σ = sk·H(m), verified via
/// e(σ, G) = e(H(m), pk). Supports efficient signature aggregation.
pub fn bls_signature_ty() -> Expr {
    arrow(bilinear_map_ty(), type0())
}
/// `BlsUnforgeability : BilinearMap → Prop`
///
/// BLS EUF-CMA security: forging a BLS signature is as hard as solving
/// the co-CDH problem in the bilinear group, in the random oracle model.
pub fn bls_unforgeability_ty() -> Expr {
    arrow(bilinear_map_ty(), prop())
}
/// `SigmaProtocol : Type`
///
/// A Sigma protocol (3-move honest-verifier zero-knowledge proof):
/// (P → V: commitment a; V → P: challenge e; P → V: response z).
/// Used as a building block for non-interactive ZK via Fiat-Shamir.
pub fn sigma_protocol_ty() -> Expr {
    type0()
}
/// `ZkCompleteness : SigmaProtocol → Prop`
///
/// Completeness: an honest prover with a valid witness always convinces
/// the verifier. Pr[Verify(transcript) = 1 | honest prover] = 1.
pub fn zk_completeness_ty() -> Expr {
    arrow(sigma_protocol_ty(), prop())
}
/// `ZkSoundness : SigmaProtocol → Prop`
///
/// Soundness (knowledge soundness / proof of knowledge): a cheating prover
/// without a valid witness can convince the verifier with at most negligible
/// probability. An extractor can recover the witness from two accepting transcripts.
pub fn zk_soundness_ty() -> Expr {
    arrow(sigma_protocol_ty(), prop())
}
/// `ZkZeroKnowledge : SigmaProtocol → Prop`
///
/// Zero-knowledge property: the verifier's view can be efficiently simulated
/// without the witness. The proof reveals nothing beyond the truth of the statement.
pub fn zk_zero_knowledge_ty() -> Expr {
    arrow(sigma_protocol_ty(), prop())
}
/// `FiatShamirTransform : SigmaProtocol → Type`
///
/// The Fiat-Shamir transform: converts an interactive Sigma protocol into
/// a non-interactive ZK argument (NIZK) by replacing the verifier's random
/// challenge with a hash of the commitment.
pub fn fiat_shamir_transform_ty() -> Expr {
    arrow(sigma_protocol_ty(), type0())
}
/// `FiatShamirSoundness : SigmaProtocol → Prop`
///
/// Soundness of the Fiat-Shamir transform in the random oracle model:
/// the resulting NIZK is computationally sound (non-malleable argument).
pub fn fiat_shamir_soundness_ty() -> Expr {
    arrow(sigma_protocol_ty(), prop())
}
/// `IpEqPspace : Prop`
///
/// IP = PSPACE (Shamir 1992): the class of languages decidable by
/// interactive proof systems equals PSPACE. This landmark result shows
/// interactive proofs are far more powerful than one might expect.
pub fn ip_eq_pspace_ty() -> Expr {
    prop()
}
/// `SnarkCorrectness : Prop`
///
/// SNARK correctness: a succinct non-interactive argument of knowledge (SNARK)
/// satisfies completeness, computational soundness (argument), and succinctness
/// (proof size and verification time are poly-logarithmic in witness size).
pub fn snark_correctness_ty() -> Expr {
    prop()
}
/// `CommitmentScheme : Type`
///
/// A commitment scheme (Com, Open): Com(m, r) → c (hiding phase),
/// Open(c, m, r) → Bool (binding phase). Instantiated by Pedersen or
/// hash-based commitments.
pub fn commitment_scheme_ty() -> Expr {
    type0()
}
/// `CommitmentHiding : CommitmentScheme → Prop`
///
/// Hiding property: the commitment c = Com(m, r) with fresh randomness r
/// reveals no information about m to a computationally bounded adversary.
pub fn commitment_hiding_ty() -> Expr {
    arrow(commitment_scheme_ty(), prop())
}
/// `CommitmentBinding : CommitmentScheme → Prop`
///
/// Binding property: it is computationally infeasible to open a commitment
/// to two different messages m ≠ m', i.e., find (m, r), (m', r') such that
/// Com(m, r) = Com(m', r').
pub fn commitment_binding_ty() -> Expr {
    arrow(commitment_scheme_ty(), prop())
}
/// `PedersenCommitment : Type`
///
/// Pedersen commitment scheme: Com(m, r) = g^m * h^r in a cyclic group.
/// Perfectly hiding (statistically), computationally binding under DLH.
pub fn pedersen_commitment_ty() -> Expr {
    type0()
}
/// `ObliviousTransfer : Type`
///
/// 1-out-of-2 oblivious transfer (OT): sender has (m0, m1); receiver chooses
/// bit b and receives m_b without the sender learning b, and without the receiver
/// learning m_{1-b}. OT is complete for two-party computation.
pub fn oblivious_transfer_ty() -> Expr {
    type0()
}
/// `OtCorrectness : ObliviousTransfer → Prop`
///
/// OT correctness: the receiver obtains the correct message m_b, the sender
/// does not learn the choice bit b, and the receiver learns nothing about m_{1-b}.
pub fn ot_correctness_ty() -> Expr {
    arrow(oblivious_transfer_ty(), prop())
}
/// `SecureMpc : Nat → Type`
///
/// Secure multi-party computation (MPC) for n parties: jointly compute a
/// function f(x1, ..., xn) where party i holds xi, without revealing xi to
/// other parties beyond what is implied by the output.
pub fn secure_mpc_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MpcSecurity : SecureMpc → Prop`
///
/// MPC security (simulation-based definition): the real-world execution
/// is computationally indistinguishable from an ideal execution with a
/// trusted party, for any polynomial-time adversary corrupting a minority
/// of parties (honest-majority setting).
pub fn mpc_security_ty() -> Expr {
    arrow(app(cst("SecureMPC"), nat_ty()), prop())
}
/// `SomewhatHomomorphicEncryption : Type`
///
/// Somewhat homomorphic encryption (SHE): supports a limited number of
/// homomorphic additions and multiplications on ciphertexts, such that
/// Dec(Enc(a) ⊕ Enc(b)) = a + b and Dec(Enc(a) ⊗ Enc(b)) = a * b,
/// up to a bounded circuit depth.
pub fn she_ty() -> Expr {
    type0()
}
/// `SheCorrectness : SHE → Prop`
///
/// SHE correctness: homomorphic evaluation of a circuit C preserves
/// the plaintext computation up to the scheme's supported depth.
pub fn she_correctness_ty() -> Expr {
    arrow(she_ty(), prop())
}
/// `FullyHomomorphicEncryption : Type`
///
/// Fully homomorphic encryption (FHE — Gentry 2009): supports evaluation
/// of arbitrary circuits on ciphertexts. Constructed by bootstrapping SHE.
/// Enables "computing on encrypted data" with no circuit depth restriction.
pub fn fhe_ty() -> Expr {
    type0()
}
/// `FheCorrectness : FHE → Prop`
///
/// FHE correctness: for any Boolean circuit C,
/// Dec(sk, Eval(pk, C, Enc(pk, x1), ..., Enc(pk, xn))) = C(x1, ..., xn).
pub fn fhe_correctness_ty() -> Expr {
    arrow(fhe_ty(), prop())
}
/// `BootstrappingTheorem : Prop`
///
/// Gentry's bootstrapping theorem: a SHE scheme that can evaluate its own
/// decryption circuit (with one extra multiplication) can be bootstrapped
/// into a FHE scheme.
pub fn bootstrapping_theorem_ty() -> Expr {
    prop()
}
/// `ShamirSecretSharing : Nat → Nat → Type`
///
/// Shamir's (k, n)-threshold secret sharing: a secret s is split into n shares
/// such that any k shares can reconstruct s (via Lagrange interpolation over F_p),
/// but any k-1 shares reveal no information about s.
pub fn shamir_secret_sharing_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ShamirPerfectSecrecy : Nat → Nat → Prop`
///
/// Perfect secrecy of Shamir's scheme: any set of fewer than k shares is
/// statistically independent of the secret. This follows from the fact that
/// a degree-(k-1) polynomial over F_p is determined by k points, and any
/// value for f(0) is equally likely given only k-1 points.
pub fn shamir_perfect_secrecy_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ShamirReconstruction : Nat → Nat → Prop`
///
/// Shamir reconstruction correctness: any k shares can reconstruct the secret
/// by Lagrange interpolation of the underlying degree-(k-1) polynomial.
pub fn shamir_reconstruction_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ThresholdScheme : Nat → Nat → Type`
///
/// General (k, n)-threshold scheme: a secret sharing scheme where any k-subset
/// of n participants can recover the secret, and any (k-1)-subset cannot.
pub fn threshold_scheme_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `HashChain : Nat → Type`
///
/// A hash chain of length n: h_0 = genesis, h_{i+1} = H(h_i || data_i).
/// Hash chains are used in blockchain ledgers, one-time passwords (S/KEY),
/// and certificate transparency logs.
pub fn hash_chain_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `HashChainIntegrity : HashChain → Prop`
///
/// Hash chain integrity: if H is collision-resistant, then any
/// tampering with block i invalidates all subsequent hashes.
/// Formally, finding a valid tampered chain is as hard as finding a
/// collision in H.
pub fn hash_chain_integrity_ty() -> Expr {
    arrow(app(cst("HashChain"), nat_ty()), prop())
}
/// `MerkleTree : Nat → Type`
///
/// A Merkle hash tree of depth d: a binary tree whose leaves are data blocks,
/// internal nodes hold the hash of their children's hashes, and the root
/// is a compact commitment to all leaves.
pub fn merkle_tree_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `MerkleInclusionProof : MerkleTree → Prop`
///
/// Merkle inclusion proof: a path of sibling hashes from a leaf to the root
/// proves a data block is included in the committed set, in O(log n) time
/// and space. Security reduces to collision resistance of H.
pub fn merkle_inclusion_proof_ty() -> Expr {
    arrow(app(cst("MerkleTree"), nat_ty()), prop())
}
/// `BlockchainConsensus : Type`
///
/// A blockchain consensus protocol: agreement on an append-only log secured
/// by proof-of-work (hash puzzle), proof-of-stake, or other mechanisms.
/// Nakamoto consensus (Bitcoin) uses longest-chain rule.
pub fn blockchain_consensus_ty() -> Expr {
    type0()
}
/// Register all cryptography axioms and theorems into the kernel environment.
///
/// This populates the environment with:
/// - Type formers for cryptographic objects (original + new)
/// - Security property propositions (as axioms)
/// - Correctness theorems (as axioms, to be proved externally)
pub fn build_cryptography_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("Crypto.HashFunction", hash_function_ty()),
        ("Crypto.SymmetricKey", symmetric_key_ty()),
        ("Crypto.PublicKey", public_key_ty()),
        ("Crypto.PrivateKey", private_key_ty()),
        ("Crypto.Signature", signature_ty()),
        ("Crypto.Ciphertext", ciphertext_ty()),
        ("Crypto.Group", group_ty()),
        ("Crypto.RsaParams", rsa_params_ty()),
        ("Crypto.OneWayFunction", one_way_function_ty()),
        ("Crypto.CollisionResistant", collision_resistant_ty()),
        ("Crypto.IndCpa", ind_cpa_ty()),
        ("Crypto.IndCca", ind_cca_ty()),
        ("Crypto.EufCma", euf_cma_ty()),
        ("Crypto.DiscreteLogHard", discrete_log_hard_ty()),
        ("Crypto.RsaHard", rsa_hard_ty()),
        ("Crypto.RsaCorrectness", rsa_correctness_ty()),
        ("Crypto.DhCorrectness", dh_correctness_ty()),
        ("Crypto.BirthdayBound", birthday_bound_ty()),
        ("Crypto.TrapdoorFunction", trapdoor_function_ty()),
        ("Crypto.TrapdoorInvertible", trapdoor_invertible_ty()),
        ("Crypto.GoldreichLevinHardCoreBit", goldreich_levin_ty()),
        ("Crypto.PRG", prg_ty()),
        ("Crypto.PrgSecure", prg_secure_ty()),
        ("Crypto.PRF", prf_ty()),
        ("Crypto.PrfSecure", prf_secure_ty()),
        ("Crypto.PreimageResistant", preimage_resistant_ty()),
        (
            "Crypto.SecondPreimageResistant",
            second_preimage_resistant_ty(),
        ),
        ("Crypto.RandomOracleModel", random_oracle_model_ty()),
        ("Crypto.SignatureScheme", signature_scheme_ty()),
        ("Crypto.SignatureCorrectness", signature_correctness_ty()),
        ("Crypto.EcdsaSignature", ecdsa_signature_ty()),
        ("Crypto.EcdsaCorrectness", ecdsa_correctness_ty()),
        ("Crypto.EcdsaUnforgeability", ecdsa_unforgeability_ty()),
        ("Crypto.PKEScheme", pke_scheme_ty()),
        ("Crypto.PkeCorrectness", pke_correctness_ty()),
        ("Crypto.IndCca2", ind_cca2_ty()),
        ("Crypto.RsaOaep", rsa_oaep_ty()),
        ("Crypto.RsaOaepIndCca2", rsa_oaep_ind_cca2_ty()),
        ("Crypto.EllipticCurve", elliptic_curve_ty()),
        ("Crypto.ECPoint", ec_point_ty()),
        ("Crypto.EcGroupLaw", ec_group_law_ty()),
        ("Crypto.EcdlpHard", ecdlp_hard_ty()),
        ("Crypto.EcdhCorrectness", ecdh_correctness_ty()),
        ("Crypto.EcdhHardness", ecdh_hardness_ty()),
        ("Crypto.BilinearMap", bilinear_map_ty()),
        ("Crypto.PairingBilinearity", pairing_bilinearity_ty()),
        ("Crypto.BdhHard", bdh_hard_ty()),
        ("Crypto.BdddhHard", bdddh_hard_ty()),
        ("Crypto.BlsSignature", bls_signature_ty()),
        ("Crypto.BlsUnforgeability", bls_unforgeability_ty()),
        ("Crypto.SigmaProtocol", sigma_protocol_ty()),
        ("Crypto.ZkCompleteness", zk_completeness_ty()),
        ("Crypto.ZkSoundness", zk_soundness_ty()),
        ("Crypto.ZkZeroKnowledge", zk_zero_knowledge_ty()),
        ("Crypto.FiatShamirTransform", fiat_shamir_transform_ty()),
        ("Crypto.FiatShamirSoundness", fiat_shamir_soundness_ty()),
        ("Crypto.IpEqPspace", ip_eq_pspace_ty()),
        ("Crypto.SnarkCorrectness", snark_correctness_ty()),
        ("Crypto.CommitmentScheme", commitment_scheme_ty()),
        ("Crypto.CommitmentHiding", commitment_hiding_ty()),
        ("Crypto.CommitmentBinding", commitment_binding_ty()),
        ("Crypto.PedersenCommitment", pedersen_commitment_ty()),
        ("Crypto.ObliviousTransfer", oblivious_transfer_ty()),
        ("Crypto.OtCorrectness", ot_correctness_ty()),
        ("Crypto.SecureMPC", secure_mpc_ty()),
        ("Crypto.MpcSecurity", mpc_security_ty()),
        ("Crypto.SHE", she_ty()),
        ("Crypto.SheCorrectness", she_correctness_ty()),
        ("Crypto.FHE", fhe_ty()),
        ("Crypto.FheCorrectness", fhe_correctness_ty()),
        ("Crypto.BootstrappingTheorem", bootstrapping_theorem_ty()),
        ("Crypto.ShamirSecretSharing", shamir_secret_sharing_ty()),
        ("Crypto.ShamirPerfectSecrecy", shamir_perfect_secrecy_ty()),
        ("Crypto.ShamirReconstruction", shamir_reconstruction_ty()),
        ("Crypto.ThresholdScheme", threshold_scheme_ty()),
        ("Crypto.HashChain", hash_chain_ty()),
        ("Crypto.HashChainIntegrity", hash_chain_integrity_ty()),
        ("Crypto.MerkleTree", merkle_tree_ty()),
        ("Crypto.MerkleInclusionProof", merkle_inclusion_proof_ty()),
        ("Crypto.BlockchainConsensus", blockchain_consensus_ty()),
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
/// Fast modular exponentiation: computes `base^exp mod modulus`.
///
/// Uses repeated squaring (binary method) in O(log exp) multiplications.
///
/// # WARNING
/// This is an educational implementation. For production use, employ a
/// constant-time implementation from a vetted cryptography library.
pub fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result: u128 = 1;
    let mut base = base as u128 % modulus as u128;
    let mut exp = exp;
    let modulus = modulus as u128;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result as u64
}
/// Extended Euclidean algorithm.
///
/// Returns `(gcd, x, y)` such that `a*x + b*y = gcd(a, b)`.
///
/// # WARNING
/// Educational implementation only.
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = extended_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}
/// Modular inverse of `a` modulo `m`, if it exists.
///
/// Returns `Some(x)` such that `a*x ≡ 1 (mod m)`, or `None` if gcd(a, m) ≠ 1.
///
/// # WARNING
/// Educational implementation only.
pub fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let (g, x, _) = extended_gcd(a as i64, m as i64);
    if g != 1 {
        return None;
    }
    Some(((x % m as i64 + m as i64) % m as i64) as u64)
}
/// One round of the SHA-256 compression function (educational model).
///
/// Updates the 8-word working state `[a, b, c, d, e, f, g, h]` using
/// the message schedule word `w` and round constant `k`.
///
/// # WARNING
/// This is a simplified educational illustration. The full SHA-256 algorithm
/// requires a complete message schedule, 64 rounds, and proper IV initialisation.
/// Do NOT use this for any real hashing.
pub fn sha256_compress_round(state: &mut [u32; 8], w: u32, k: u32) {
    let [a, b, c, d, e, f, g, h] = *state;
    let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
    let ch = (e & f) ^ ((!e) & g);
    let temp1 = h
        .wrapping_add(s1)
        .wrapping_add(ch)
        .wrapping_add(k)
        .wrapping_add(w);
    let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
    let maj = (a & b) ^ (a & c) ^ (b & c);
    let temp2 = s0.wrapping_add(maj);
    state[7] = g;
    state[6] = f;
    state[5] = e;
    state[4] = d.wrapping_add(temp1);
    state[3] = c;
    state[2] = b;
    state[1] = a;
    state[0] = temp1.wrapping_add(temp2);
}
/// Simple polynomial rolling hash over a byte slice.
///
/// Computes `Σ data\[i\] * BASE^i mod MOD`. This is a common non-cryptographic
/// hash used in string algorithms (e.g., Rabin-Karp).
///
/// # WARNING
/// This is NOT a cryptographic hash function. It provides no security
/// guarantees whatsoever.
pub fn simple_hash(data: &[u8]) -> u64 {
    const BASE: u128 = 131;
    const MOD: u128 = (1 << 61) - 1;
    let mut hash: u128 = 0;
    let mut power: u128 = 1;
    for &byte in data {
        hash = (hash + (byte as u128) * power) % MOD;
        power = power * BASE % MOD;
    }
    hash as u64
}
/// Caesar cipher encryption: shifts each byte by `shift` (mod 256).
///
/// # WARNING
/// The Caesar cipher has zero security. It is broken by simple frequency
/// analysis and is included here purely for educational illustration.
pub fn caesar_cipher(text: &[u8], shift: u8) -> Vec<u8> {
    text.iter().map(|&b| b.wrapping_add(shift)).collect()
}
/// Caesar cipher decryption: shifts each byte back by `shift` (mod 256).
///
/// # WARNING
/// Educational only. Not secure.
pub fn caesar_decipher(text: &[u8], shift: u8) -> Vec<u8> {
    text.iter().map(|&b| b.wrapping_sub(shift)).collect()
}
/// Vigenère cipher encryption: XORs each byte with the repeating key.
///
/// # WARNING
/// The Vigenère cipher (and this XOR variant) is trivially broken when the
/// key length is known or guessable. It is included for historical/educational
/// purposes only.
pub fn vigenere_cipher(text: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        return text.to_vec();
    }
    text.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
}
/// Miller-Rabin primality test with explicit witnesses.
///
/// Returns `true` if `n` is (probably) prime for all given witnesses,
/// `false` if `n` is definitely composite.
///
/// For deterministic results up to 3,215,031,751, use witnesses `[2, 3, 5, 7]`.
/// For up to 3,474,749,660,383, use `[2, 3, 5, 7, 11, 13]`.
///
/// # WARNING
/// This is a probabilistic test. With random witnesses the false-positive
/// probability per witness is at most 1/4. For production use, employ a
/// fully deterministic implementation with appropriate witness sets.
pub fn miller_rabin(n: u64, witnesses: &[u64]) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d = n - 1;
    let mut r = 0u32;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    'witness: for &a in witnesses {
        if a >= n {
            continue;
        }
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'witness;
        }
        for _ in 0..r - 1 {
            x = mod_exp(x, 2, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_mod_exp() {
        assert_eq!(mod_exp(2, 10, 1000), 24);
        assert_eq!(mod_exp(5, 0, 7), 1);
        assert_eq!(mod_exp(0, 5, 7), 0);
        assert_eq!(mod_exp(3, 6, 7), 1);
    }
    #[test]
    fn test_extended_gcd() {
        let (g, x, y) = extended_gcd(35, 15);
        assert_eq!(g, 5);
        assert_eq!(35 * x + 15 * y, g);
        let (g2, x2, y2) = extended_gcd(48, 18);
        assert_eq!(g2, 6);
        assert_eq!(48 * x2 + 18 * y2, g2);
    }
    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 7), Some(5));
        assert_eq!(mod_inverse(2, 9), Some(5));
        assert_eq!(mod_inverse(4, 6), None);
    }
    #[test]
    fn test_toy_rsa() {
        let rsa = ToyRsa::generate(61, 53).expect("RSA generation should succeed for p=61, q=53");
        let message = 42u64;
        assert!(message < rsa.n, "message must be smaller than modulus");
        let ciphertext = rsa.encrypt(message);
        let recovered = rsa.decrypt(ciphertext);
        assert_eq!(
            recovered, message,
            "RSA decrypt(encrypt(m)) must equal m; got {} for message {}",
            recovered, message
        );
        let msg2 = 100u64;
        assert!(msg2 < rsa.n);
        assert_eq!(rsa.decrypt(rsa.encrypt(msg2)), msg2);
    }
    #[test]
    fn test_toy_dh() {
        let dh = ToyDiffieHellman { p: 23, g: 5 };
        let alice_private = 6u64;
        let bob_private = 15u64;
        let alice_public = dh.public_key(alice_private);
        let bob_public = dh.public_key(bob_private);
        let alice_secret = dh.shared_secret(bob_public, alice_private);
        let bob_secret = dh.shared_secret(alice_public, bob_private);
        assert_eq!(
            alice_secret, bob_secret,
            "Diffie-Hellman shared secrets must match: Alice got {}, Bob got {}",
            alice_secret, bob_secret
        );
        assert_eq!(alice_secret, 2);
    }
    #[test]
    fn test_miller_rabin() {
        let primes = [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 97, 101, 7919];
        let witnesses = [2u64, 3, 5, 7];
        for &p in &primes {
            assert!(
                miller_rabin(p, &witnesses),
                "{} is prime but Miller-Rabin returned false",
                p
            );
        }
        let composites = [1u64, 4, 6, 8, 9, 10, 15, 21, 25, 100, 561];
        for &c in &composites {
            assert!(
                !miller_rabin(c, &witnesses),
                "{} is composite but Miller-Rabin returned true",
                c
            );
        }
    }
    #[test]
    fn test_caesar() {
        let plaintext = b"hello";
        let shift = 3u8;
        let ciphertext = caesar_cipher(plaintext, shift);
        let decrypted = caesar_decipher(&ciphertext, shift);
        assert_eq!(
            decrypted, plaintext,
            "Caesar decipher(cipher(text, k), k) must return original text"
        );
        assert_eq!(ciphertext[0], b'k');
    }
    #[test]
    fn test_vigenere() {
        let plaintext = b"attackatdawn";
        let key = b"lemon";
        let ciphertext = vigenere_cipher(plaintext, key);
        let decrypted = vigenere_cipher(&ciphertext, key);
        assert_eq!(decrypted, plaintext);
    }
    #[test]
    fn test_simple_hash() {
        assert_eq!(simple_hash(b"hello"), simple_hash(b"hello"));
        assert_ne!(simple_hash(b"hello"), simple_hash(b"world"));
        assert_eq!(simple_hash(b""), 0);
    }
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_cryptography_env(&mut env);
        assert!(
            result.is_ok(),
            "build_cryptography_env should succeed: {:?}",
            result
        );
    }
    #[test]
    fn test_modular_arithmetic() {
        let ma = ModularArithmetic::new(17);
        assert_eq!(ma.add(10, 9), 2);
        assert_eq!(ma.sub(3, 5), 15);
        assert_eq!(ma.mul(4, 5), 3);
        assert_eq!(ma.pow(2, 8), 1);
        assert_eq!(ma.inv(3), Some(6));
        assert_eq!(ma.legendre(4), 1);
        let ma7 = ModularArithmetic::new(7);
        assert_eq!(ma7.legendre(3), -1);
    }
    #[test]
    fn test_rsa_keygen() {
        let (rsa, p, q) =
            RsaKeyGen::generate_from_seed(500).expect("RsaKeyGen should succeed for seed=500");
        assert!(miller_rabin(p, &[2, 3, 5, 7]), "p should be prime");
        assert!(miller_rabin(q, &[2, 3, 5, 7]), "q should be prime");
        assert_eq!(rsa.n, p * q);
        let msg = 7u64;
        assert!(msg < rsa.n);
        assert_eq!(rsa.decrypt(rsa.encrypt(msg)), msg);
    }
    #[test]
    fn test_diffie_hellman_sim() {
        let sim = DiffieHellmanSim::new(23, 5, 6, 15);
        assert!(sim.secrets_match(), "DH shared secrets must match");
        assert_eq!(sim.alice_shared_secret(), 2);
        assert_eq!(sim.bob_shared_secret(), 2);
    }
    #[test]
    fn test_hash_chain() {
        let mut chain = HashChain::new(12345);
        let data = [111u64, 222, 333];
        for &d in &data {
            chain.append(d);
        }
        assert_eq!(chain.chain.len(), 4);
        assert!(chain.verify(&data), "Hash chain verification should pass");
        let mut tampered_chain = chain.clone();
        tampered_chain.chain[1] = tampered_chain.chain[1].wrapping_add(1);
        assert!(
            !tampered_chain.verify(&data),
            "Tampered chain should fail verification"
        );
    }
    #[test]
    fn test_shamir_secret_share() {
        let sss = ShamirSecretShare::new(97, 2, 3);
        let secret = 42u64;
        let shares = sss.share(secret, 12345);
        assert_eq!(shares.len(), 3);
        let from_01 = sss.reconstruct(&shares[..2]);
        let from_12 = sss.reconstruct(&[shares[1], shares[2]]);
        let from_02 = sss.reconstruct(&[shares[0], shares[2]]);
        assert_eq!(
            from_01,
            Some(secret),
            "shares[0,1] should reconstruct secret"
        );
        assert_eq!(
            from_12,
            Some(secret),
            "shares[1,2] should reconstruct secret"
        );
        assert_eq!(
            from_02,
            Some(secret),
            "shares[0,2] should reconstruct secret"
        );
        let from_all = sss.reconstruct(&shares);
        assert_eq!(
            from_all,
            Some(secret),
            "all 3 shares should reconstruct secret"
        );
        let from_one = sss.reconstruct(&shares[..1]);
        assert_eq!(from_one, None, "fewer than k shares should return None");
    }
    #[test]
    fn test_axioms_registered() {
        let mut env = Environment::new();
        build_cryptography_env(&mut env).expect("build_cryptography_env should succeed");
        let expected = [
            "Crypto.TrapdoorFunction",
            "Crypto.GoldreichLevinHardCoreBit",
            "Crypto.PRG",
            "Crypto.PRF",
            "Crypto.PreimageResistant",
            "Crypto.EcdsaUnforgeability",
            "Crypto.IndCca2",
            "Crypto.RsaOaepIndCca2",
            "Crypto.EllipticCurve",
            "Crypto.EcdlpHard",
            "Crypto.EcdhCorrectness",
            "Crypto.BilinearMap",
            "Crypto.BdhHard",
            "Crypto.BlsUnforgeability",
            "Crypto.SigmaProtocol",
            "Crypto.ZkCompleteness",
            "Crypto.ZkSoundness",
            "Crypto.ZkZeroKnowledge",
            "Crypto.FiatShamirTransform",
            "Crypto.IpEqPspace",
            "Crypto.SnarkCorrectness",
            "Crypto.CommitmentScheme",
            "Crypto.CommitmentHiding",
            "Crypto.CommitmentBinding",
            "Crypto.ObliviousTransfer",
            "Crypto.SecureMPC",
            "Crypto.FHE",
            "Crypto.BootstrappingTheorem",
            "Crypto.ShamirSecretSharing",
            "Crypto.ShamirPerfectSecrecy",
            "Crypto.HashChain",
            "Crypto.MerkleTree",
            "Crypto.BlockchainConsensus",
        ];
        for name in &expected {
            assert!(
                env.get(&Name::str(*name)).is_some(),
                "Expected axiom '{}' not found in environment",
                name
            );
        }
    }
}
/// `LweHardness : Nat → Nat → Prop`
///
/// Learning With Errors (LWE) hardness (Regev 2005): given (A, b = As + e mod q)
/// where s is a secret vector and e is a small-norm error vector, it is
/// computationally hard to recover s. LWE is the foundation of most
/// lattice-based public-key cryptography.
pub fn cry_ext_lwe_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `RlweHardness : Nat → Prop`
///
/// Ring-LWE (RLWE) hardness: the LWE problem instantiated over polynomial rings
/// Z_q\[x\]/(f(x)). More efficient than standard LWE while maintaining
/// comparable security. Basis of CRYSTALS-Kyber and CRYSTALS-Dilithium.
pub fn cry_ext_rlwe_hardness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LweEncScheme : Nat → Nat → Type`
///
/// LWE-based public-key encryption scheme: a lattice-based PKE with
/// security reduction to LWE. Encryption adds error to hide the plaintext.
pub fn cry_ext_lwe_enc_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `LweEncCorrectness : Nat → Nat → Prop`
///
/// LWE encryption correctness: decryption recovers the plaintext when
/// the error magnitude is bounded. The noise must stay below the
/// correctness threshold throughout computation.
pub fn cry_ext_lwe_enc_correct_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `KyberKem : Nat → Type`
///
/// CRYSTALS-Kyber key encapsulation mechanism: an IND-CCA2 secure KEM
/// based on Module-LWE. Selected by NIST for post-quantum standardization
/// (ML-KEM, FIPS 203). Parameterized by security level k ∈ {2, 3, 4}.
pub fn cry_ext_kyber_kem_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `KyberIndCca2 : Nat → Prop`
///
/// Kyber IND-CCA2 security: under the Module-LWE assumption, no
/// polynomial-time adversary can break the IND-CCA2 security of Kyber
/// with non-negligible advantage.
pub fn cry_ext_kyber_ind_cca2_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `NtruHardness : Nat → Prop`
///
/// NTRU hardness assumption: given (h = f^{-1} * g mod q) in the ring
/// Z\[x\]/(x^n - 1), it is computationally hard to recover the short
/// polynomials f and g. NTRU was one of the first lattice-based cryptosystems.
pub fn cry_ext_ntru_hardness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `DilithiumSig : Nat → Type`
///
/// CRYSTALS-Dilithium digital signature scheme: a lattice-based signature
/// based on Module-LWE and Module-SIS. Selected by NIST for standardization
/// (ML-DSA, FIPS 204).
pub fn cry_ext_dilithium_sig_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `DilithiumEufCma : Nat → Prop`
///
/// EUF-CMA security of Dilithium: forging signatures is as hard as
/// solving Module-LWE and Module-SIS problems.
pub fn cry_ext_dilithium_euf_cma_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LinearCode : Nat → Nat → Nat → Type`
///
/// A linear error-correcting code \[n, k, d\]: a k-dimensional subspace of F_2^n
/// with minimum Hamming distance d. Can correct up to ⌊(d-1)/2⌋ errors.
pub fn cry_ext_linear_code_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `McElieceEncryption : Nat → Nat → Nat → Type`
///
/// McEliece cryptosystem (1978): public-key encryption based on the hardness
/// of decoding a random linear code. The public key is a scrambled generator
/// matrix; security relies on the NP-hardness of general decoding.
pub fn cry_ext_mceliece_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(nat_ty(), type0())))
}
/// `McElieceHardness : Nat → Nat → Prop`
///
/// McEliece hardness: decoding a random linear code is NP-hard in the
/// worst case and believed hard on average. This provides post-quantum
/// security (no known quantum speedup beyond sqrt).
pub fn cry_ext_mceliece_hardness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SyndromeDecode : Nat → Nat → Prop`
///
/// Syndrome decoding problem (SDP): given a parity-check matrix H and
/// syndrome s, find a low-weight vector e such that He = s. This is
/// NP-complete and underlies code-based cryptography.
pub fn cry_ext_syndrome_decode_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `XmssSig : Nat → Type`
///
/// XMSS (eXtended Merkle Signature Scheme): a stateful hash-based signature
/// scheme with security based only on the collision resistance of the hash
/// function. Standardized in RFC 8391. Parameterized by tree height h.
pub fn cry_ext_xmss_sig_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `XmssEufCma : Nat → Prop`
///
/// EUF-CMA security of XMSS: security reduces to the collision resistance
/// and second-preimage resistance of the underlying hash function.
/// No additional hardness assumptions required.
pub fn cry_ext_xmss_euf_cma_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SphincsSig : Type`
///
/// SPHINCS+: a stateless hash-based signature scheme. Unlike XMSS, it
/// requires no state management. Selected by NIST for standardization
/// (SLH-DSA, FIPS 205). Security based solely on hash function properties.
pub fn cry_ext_sphincs_sig_ty() -> Expr {
    type0()
}
/// `SphincsEufCma : Prop`
///
/// EUF-CMA security of SPHINCS+: the scheme is secure under the assumption
/// that the underlying hash function is a secure random oracle.
pub fn cry_ext_sphincs_euf_cma_ty() -> Expr {
    prop()
}
/// `WotsPlus : Nat → Type`
///
/// WOTS+ (Winternitz One-Time Signature): a one-time signature scheme
/// used as a building block in XMSS and SPHINCS+.
/// Parameterized by the Winternitz parameter w.
pub fn cry_ext_wots_plus_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `Isogeny : Nat → Nat → Type`
///
/// An isogeny between two elliptic curves E1 and E2: a non-constant rational
/// map φ: E1 → E2 that is also a group homomorphism. The degree of the
/// isogeny is the size of its kernel.
pub fn cry_ext_isogeny_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `SidhHardness : Nat → Prop`
///
/// SIDH (Supersingular Isogeny Diffie-Hellman) hardness: computing the
/// shared secret from public parameters requires finding an isogeny between
/// supersingular elliptic curves. Note: SIKE was broken in 2022 by
/// Castryck-Decru; modern variants aim to repair this.
pub fn cry_ext_sidh_hardness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SupersingularIsogenyCurve : Nat → Type`
///
/// A supersingular elliptic curve over F_p^2: these form a special class
/// with desirable properties for isogeny-based cryptography, including
/// a well-structured isogeny graph (Ramanujan graph).
pub fn cry_ext_supersingular_curve_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ZkSnark : Type`
///
/// A zk-SNARK (zero-knowledge succinct non-interactive argument of knowledge):
/// a proof system with (1) completeness, (2) computational knowledge soundness,
/// (3) zero-knowledge, and (4) succinctness (|proof| = polylog(|circuit|)).
pub fn cry_ext_zk_snark_ty() -> Expr {
    type0()
}
/// `ZkSnarkSuccinctness : ZkSnark → Prop`
///
/// Succinctness: the proof size and verification time are poly-logarithmic
/// in the witness size and circuit complexity.
pub fn cry_ext_zk_snark_succinctness_ty() -> Expr {
    arrow(cry_ext_zk_snark_ty(), prop())
}
/// `ZkSnarkKnowledgeSoundness : ZkSnark → Prop`
///
/// Knowledge soundness (argument of knowledge): if a prover can produce
/// an accepting proof, then an extractor can efficiently recover a valid
/// witness from the prover's internal state.
pub fn cry_ext_zk_snark_knowledge_ty() -> Expr {
    arrow(cry_ext_zk_snark_ty(), prop())
}
/// `ZkStark : Type`
///
/// A zk-STARK (zero-knowledge scalable transparent argument of knowledge):
/// like SNARKs but with transparent (no trusted) setup and post-quantum
/// security via hash functions. Verification is O(polylog n).
pub fn cry_ext_zk_stark_ty() -> Expr {
    type0()
}
/// `ZkStarkSoundness : ZkStark → Prop`
///
/// STARK soundness: security relies only on collision-resistant hash
/// functions, providing post-quantum security without trusted setup.
pub fn cry_ext_zk_stark_soundness_ty() -> Expr {
    arrow(cry_ext_zk_stark_ty(), prop())
}
/// `SchnorrIdentification : Type`
///
/// Schnorr identification protocol (1989): a 3-move honest-verifier ZK
/// proof of knowledge of a discrete logarithm. The basis of Schnorr
/// signatures and EdDSA. Secure under the discrete log assumption.
pub fn cry_ext_schnorr_id_ty() -> Expr {
    type0()
}
/// `SchnorrSoundness : SchnorrIdentification → Prop`
///
/// Special soundness of Schnorr: given two accepting transcripts (a, e, z)
/// and (a, e', z') with e ≠ e', an extractor can compute the witness x.
pub fn cry_ext_schnorr_soundness_ty() -> Expr {
    arrow(cry_ext_schnorr_id_ty(), prop())
}
/// `NizkProof : Type`
///
/// Non-interactive zero-knowledge (NIZK) proof: a single-message proof
/// obtained from an interactive proof via the Fiat-Shamir transform or
/// through a CRS (common reference string).
pub fn cry_ext_nizk_proof_ty() -> Expr {
    type0()
}
/// `NizkSimulationSoundness : NizkProof → Prop`
///
/// Simulation soundness: a NIZK is simulation sound if an adversary cannot
/// produce a valid proof for a false statement, even after seeing simulated
/// proofs for false statements. Stronger than standard soundness.
pub fn cry_ext_nizk_sim_soundness_ty() -> Expr {
    arrow(cry_ext_nizk_proof_ty(), prop())
}
/// `BlakleySecretSharing : Nat → Nat → Type`
///
/// Blakley's threshold secret sharing (1979): geometrically represents
/// the secret as a point in (k-1)-dimensional space. Each share is a
/// hyperplane through that point; k hyperplanes intersect at the secret.
pub fn cry_ext_blakley_ss_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `BlakleyCorrectness : Nat → Nat → Prop`
///
/// Correctness of Blakley's scheme: any k hyperplanes (shares) uniquely
/// determine the intersection point (the secret).
pub fn cry_ext_blakley_correct_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `GarbledCircuit : Type`
///
/// A garbled circuit (Yao 1986): an encoding of a Boolean circuit C such
/// that a party can evaluate C(x) on an encrypted input x without learning
/// anything about x or the circuit beyond C(x). Foundation of secure 2PC.
pub fn cry_ext_garbled_circuit_ty() -> Expr {
    type0()
}
/// `GarbledCircuitSecurity : GarbledCircuit → Prop`
///
/// Security of garbled circuits: the garbling reveals no information
/// about the circuit inputs or intermediate wire values, only the output.
/// Proven secure in the random oracle model.
pub fn cry_ext_garbled_circuit_sec_ty() -> Expr {
    arrow(cry_ext_garbled_circuit_ty(), prop())
}
/// `ObliviousRam : Nat → Type`
///
/// Oblivious RAM (ORAM — Goldreich-Ostrovsky 1996): a protocol for accessing
/// memory such that the access pattern reveals no information about which
/// locations are being accessed. Storage overhead: O(log^2 n) per access.
pub fn cry_ext_oram_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `OramSecurity : Nat → Prop`
///
/// ORAM security: the sequence of physical memory accesses is computationally
/// indistinguishable from a fixed access pattern independent of the actual
/// logical accesses.
pub fn cry_ext_oram_security_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `VerifiableSecretSharing : Nat → Nat → Type`
///
/// Verifiable secret sharing (VSS — Chor et al. 1985): extends Shamir's
/// secret sharing so that shareholders can verify their shares are consistent
/// (detecting a cheating dealer). Based on Pedersen commitments.
pub fn cry_ext_vss_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `VssCorrectness : Nat → Nat → Prop`
///
/// VSS correctness: honest shareholders can always reconstruct the secret,
/// and dishonest shareholders' invalid shares are detected with overwhelming probability.
pub fn cry_ext_vss_correct_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `PedersenHiding : Prop`
///
/// Perfect hiding property of Pedersen commitments: given a commitment
/// c = g^m * h^r, the distribution over c is independent of m (statistically
/// hiding). This holds unconditionally.
pub fn cry_ext_pedersen_hiding_ty() -> Expr {
    prop()
}
/// `PedersenBinding : Prop`
///
/// Computational binding of Pedersen commitments: no polynomial-time
/// adversary can open a commitment to two different messages. Security
/// reduces to the discrete logarithm problem.
pub fn cry_ext_pedersen_binding_ty() -> Expr {
    prop()
}
/// `ElgamalEncryption : Type`
///
/// ElGamal encryption (1985): a public-key scheme based on the computational
/// Diffie-Hellman assumption. Ciphertext is (g^r, m * g^{ar}) for public key
/// g^a and randomness r. Multiplicatively homomorphic.
pub fn cry_ext_elgamal_ty() -> Expr {
    type0()
}
/// `ElgamalHomomorphism : ElgamalEncryption → Prop`
///
/// ElGamal multiplicative homomorphism: Enc(m1) * Enc(m2) = Enc(m1 * m2).
/// This allows multiplying plaintexts by operating on ciphertexts.
pub fn cry_ext_elgamal_hom_ty() -> Expr {
    arrow(cry_ext_elgamal_ty(), prop())
}
/// `ElgamalIndCpa : Prop`
///
/// IND-CPA security of ElGamal: under the Decisional Diffie-Hellman (DDH)
/// assumption, ElGamal is IND-CPA secure. Not IND-CCA secure as-is due to
/// the homomorphic property.
pub fn cry_ext_elgamal_ind_cpa_ty() -> Expr {
    prop()
}
/// `PaillierEncryption : Type`
///
/// Paillier encryption (1999): a public-key scheme with additive homomorphism.
/// Enc(m1) * Enc(m2) = Enc(m1 + m2). Security based on the Composite
/// Residuosity assumption.
pub fn cry_ext_paillier_ty() -> Expr {
    type0()
}
/// `PaillierAdditiveHom : PaillierEncryption → Prop`
///
/// Additive homomorphism of Paillier: Enc(m1) * Enc(m2) mod n^2 decrypts
/// to m1 + m2 mod n. Enables privacy-preserving summation.
pub fn cry_ext_paillier_add_hom_ty() -> Expr {
    arrow(cry_ext_paillier_ty(), prop())
}
/// `ThresholdSignature : Nat → Nat → Type`
///
/// A (t, n)-threshold signature scheme: n parties share a signing key such
/// that any t parties can collaboratively produce a valid signature, but
/// fewer than t parties cannot sign. Used in multi-sig wallets and distributed key management.
pub fn cry_ext_threshold_sig_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ThresholdSigCorrectness : Nat → Nat → Prop`
///
/// Threshold signature correctness: any t-subset of parties can produce
/// a signature verifiable under the group public key.
pub fn cry_ext_threshold_sig_correct_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ThresholdSigSecurity : Nat → Nat → Prop`
///
/// Security of threshold signatures: a coalition of fewer than t corrupted
/// parties cannot forge a signature, under standard cryptographic assumptions.
pub fn cry_ext_threshold_sig_sec_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `FrostsignatureScheme : Nat → Type`
///
/// FROST (Flexible Round-Optimized Schnorr Threshold) signature: a
/// round-optimized threshold Schnorr signature scheme. Produces standard
/// Schnorr signatures compatible with EdDSA infrastructure.
pub fn cry_ext_frost_sig_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// Register all extended cryptography axioms into the kernel environment.
///
/// Adds axioms for: LWE/RLWE lattice hardness, NTRU, Kyber, Dilithium,
/// McEliece, XMSS, SPHINCS+, isogeny-based crypto, zk-SNARKs, zk-STARKs,
/// Schnorr, NIZK, Blakley, garbled circuits, ORAM, VSS, Pedersen, ElGamal,
/// Paillier, threshold signatures, FROST.
pub fn register_cryptography_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("Crypto.LweHardness", cry_ext_lwe_hardness_ty()),
        ("Crypto.RlweHardness", cry_ext_rlwe_hardness_ty()),
        ("Crypto.LweEncScheme", cry_ext_lwe_enc_ty()),
        ("Crypto.LweEncCorrectness", cry_ext_lwe_enc_correct_ty()),
        ("Crypto.KyberKem", cry_ext_kyber_kem_ty()),
        ("Crypto.KyberIndCca2", cry_ext_kyber_ind_cca2_ty()),
        ("Crypto.NtruHardness", cry_ext_ntru_hardness_ty()),
        ("Crypto.DilithiumSig", cry_ext_dilithium_sig_ty()),
        ("Crypto.DilithiumEufCma", cry_ext_dilithium_euf_cma_ty()),
        ("Crypto.LinearCode", cry_ext_linear_code_ty()),
        ("Crypto.McElieceEncryption", cry_ext_mceliece_ty()),
        ("Crypto.McElieceHardness", cry_ext_mceliece_hardness_ty()),
        ("Crypto.SyndromeDecode", cry_ext_syndrome_decode_ty()),
        ("Crypto.XmssSig", cry_ext_xmss_sig_ty()),
        ("Crypto.XmssEufCma", cry_ext_xmss_euf_cma_ty()),
        ("Crypto.SphincsSig", cry_ext_sphincs_sig_ty()),
        ("Crypto.SphincsEufCma", cry_ext_sphincs_euf_cma_ty()),
        ("Crypto.WotsPlus", cry_ext_wots_plus_ty()),
        ("Crypto.Isogeny", cry_ext_isogeny_ty()),
        ("Crypto.SidhHardness", cry_ext_sidh_hardness_ty()),
        (
            "Crypto.SupersingularCurve",
            cry_ext_supersingular_curve_ty(),
        ),
        ("Crypto.ZkSnark", cry_ext_zk_snark_ty()),
        (
            "Crypto.ZkSnarkSuccinctness",
            cry_ext_zk_snark_succinctness_ty(),
        ),
        (
            "Crypto.ZkSnarkKnowledgeSoundness",
            cry_ext_zk_snark_knowledge_ty(),
        ),
        ("Crypto.ZkStark", cry_ext_zk_stark_ty()),
        ("Crypto.ZkStarkSoundness", cry_ext_zk_stark_soundness_ty()),
        ("Crypto.SchnorrIdentification", cry_ext_schnorr_id_ty()),
        ("Crypto.SchnorrSoundness", cry_ext_schnorr_soundness_ty()),
        ("Crypto.NizkProof", cry_ext_nizk_proof_ty()),
        (
            "Crypto.NizkSimulationSoundness",
            cry_ext_nizk_sim_soundness_ty(),
        ),
        ("Crypto.BlakleySecretSharing", cry_ext_blakley_ss_ty()),
        ("Crypto.BlakleyCorrectness", cry_ext_blakley_correct_ty()),
        ("Crypto.GarbledCircuit", cry_ext_garbled_circuit_ty()),
        (
            "Crypto.GarbledCircuitSecurity",
            cry_ext_garbled_circuit_sec_ty(),
        ),
        ("Crypto.ObliviousRam", cry_ext_oram_ty()),
        ("Crypto.OramSecurity", cry_ext_oram_security_ty()),
        ("Crypto.VerifiableSecretSharing", cry_ext_vss_ty()),
        ("Crypto.VssCorrectness", cry_ext_vss_correct_ty()),
        ("Crypto.PedersenHiding", cry_ext_pedersen_hiding_ty()),
        ("Crypto.PedersenBinding", cry_ext_pedersen_binding_ty()),
        ("Crypto.ElgamalEncryption", cry_ext_elgamal_ty()),
        ("Crypto.ElgamalHomomorphism", cry_ext_elgamal_hom_ty()),
        ("Crypto.ElgamalIndCpa", cry_ext_elgamal_ind_cpa_ty()),
        ("Crypto.PaillierEncryption", cry_ext_paillier_ty()),
        ("Crypto.PaillierAdditiveHom", cry_ext_paillier_add_hom_ty()),
        ("Crypto.ThresholdSignature", cry_ext_threshold_sig_ty()),
        (
            "Crypto.ThresholdSigCorrectness",
            cry_ext_threshold_sig_correct_ty(),
        ),
        (
            "Crypto.ThresholdSigSecurity",
            cry_ext_threshold_sig_sec_ty(),
        ),
        ("Crypto.FrostSignature", cry_ext_frost_sig_ty()),
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
