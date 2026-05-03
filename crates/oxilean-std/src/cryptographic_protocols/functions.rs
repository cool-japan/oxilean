//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BlindSignatureScheme, CommitmentScheme, GarbledGate, MPCProtocol, MpcShare, OTVariant,
    ObliviousTransfer, PaillierHomomorphic, PedersenCommitment, PedersenParams, SchnorrParams,
    SecretSharing, ShamirSS, ShamirSecretSharingExtended, ZKProofSystem,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn list_nat() -> Expr {
    app(cst("List"), nat_ty())
}
pub fn msg_ty() -> Expr {
    list_nat()
}
/// `DYMessage : Type`
///
/// The message algebra of the Dolev-Yao model:
/// atoms (nonces, keys, identities), pairs `⟨m1, m2⟩`, symmetric
/// encryption `{m}k`, and public-key encryption `{m}pk`.
pub fn dy_message_ty() -> Expr {
    type0()
}
/// `DYPrincipal : Type`
///
/// A principal (agent) in a protocol run: honest or dishonest.
pub fn dy_principal_ty() -> Expr {
    type0()
}
/// `DYKnowledge : DYPrincipal → List DYMessage → Type`
///
/// The knowledge set of a principal: the set of messages they can derive
/// from a given set of intercepted messages using Dolev-Yao operations.
pub fn dy_knowledge_ty() -> Expr {
    arrow(
        dy_principal_ty(),
        arrow(app(cst("List"), dy_message_ty()), type0()),
    )
}
/// `DYDerivable : List DYMessage → DYMessage → Prop`
///
/// `DYDerivable S m` holds iff the Dolev-Yao intruder can derive message `m`
/// from the set `S` using the standard operations:
/// decompose pairs, encrypt/decrypt with known keys, compose pairs.
pub fn dy_derivable_ty() -> Expr {
    arrow(
        app(cst("List"), dy_message_ty()),
        arrow(dy_message_ty(), prop()),
    )
}
/// `DYAttack : Protocol → Prop`
///
/// A Dolev-Yao attack on a protocol: the intruder, by intercepting and
/// replaying messages, can violate some security property.
pub fn dy_attack_ty() -> Expr {
    arrow(cst("Protocol"), prop())
}
/// `DYIntruder : Type`
///
/// The Dolev-Yao intruder model: full control over the network, can
/// eavesdrop, block, replay, modify, and inject messages.
pub fn dy_intruder_ty() -> Expr {
    type0()
}
/// `Confidentiality : Protocol → Principal → Message → Prop`
///
/// A message `m` remains confidential in protocol `P` for principal `A`:
/// no adversary can learn `m` by participating in or observing `P`.
pub fn confidentiality_ty() -> Expr {
    arrow(
        cst("Protocol"),
        arrow(dy_principal_ty(), arrow(msg_ty(), prop())),
    )
}
/// `Integrity : Protocol → Message → Prop`
///
/// Message integrity in protocol `P`: the received message equals the
/// sent message; it has not been altered in transit.
pub fn integrity_ty() -> Expr {
    arrow(cst("Protocol"), arrow(msg_ty(), prop()))
}
/// `Authentication : Protocol → Principal → Principal → Prop`
///
/// Entity authentication: after completing protocol `P`, principal `B`
/// is assured they are communicating with `A` (not an impostor).
pub fn authentication_ty() -> Expr {
    arrow(
        cst("Protocol"),
        arrow(dy_principal_ty(), arrow(dy_principal_ty(), prop())),
    )
}
/// `NonRepudiation : Protocol → Principal → Message → Prop`
///
/// Non-repudiation: principal `A` cannot later deny having sent message `m`
/// in protocol `P` (typically achieved via digital signatures).
pub fn non_repudiation_ty() -> Expr {
    arrow(
        cst("Protocol"),
        arrow(dy_principal_ty(), arrow(msg_ty(), prop())),
    )
}
/// `FreshNonce : Message → Prop`
///
/// A nonce is fresh: it has not appeared in any previous protocol run,
/// making replay attacks detectable.
pub fn fresh_nonce_ty() -> Expr {
    arrow(msg_ty(), prop())
}
/// `ProtocolCompose : Protocol → Protocol → Protocol`
///
/// Sequential composition of two protocols: run P1 then P2.
pub fn protocol_compose_ty() -> Expr {
    arrow(cst("Protocol"), arrow(cst("Protocol"), cst("Protocol")))
}
/// `ProtocolParallel : Protocol → Protocol → Protocol`
///
/// Parallel composition: run P1 and P2 concurrently (possibly interleaved).
pub fn protocol_parallel_ty() -> Expr {
    arrow(cst("Protocol"), arrow(cst("Protocol"), cst("Protocol")))
}
/// `SecurityPreservedUnderComposition : Protocol → Protocol → Prop`
///
/// If P1 and P2 are each secure in isolation, their sequential composition
/// is also secure (modulo the UC composition theorem).
pub fn security_preserved_composition_ty() -> Expr {
    arrow(cst("Protocol"), arrow(cst("Protocol"), prop()))
}
/// `SigmaProtocol : Type`
///
/// A Sigma protocol Σ = (P, V, challenge_space) for a relation R:
/// - Prover sends commitment `a`
/// - Verifier sends random challenge `e`
/// - Prover sends response `z`
/// - Verifier accepts/rejects based on (x, a, e, z)
pub fn sigma_protocol_ty() -> Expr {
    type0()
}
/// `SigmaCommitment : SigmaProtocol → Type`
///
/// The commitment message type (first move) in a Sigma protocol.
pub fn sigma_commitment_ty() -> Expr {
    arrow(sigma_protocol_ty(), type0())
}
/// `SigmaChallenge : SigmaProtocol → Type`
///
/// The challenge type (second move): a random element from the challenge space.
pub fn sigma_challenge_ty() -> Expr {
    arrow(sigma_protocol_ty(), type0())
}
/// `SigmaResponse : SigmaProtocol → Type`
///
/// The response type (third move) in a Sigma protocol.
pub fn sigma_response_ty() -> Expr {
    arrow(sigma_protocol_ty(), type0())
}
/// `SigmaCompleteness : SigmaProtocol → Prop`
///
/// Completeness of a Sigma protocol: an honest prover who knows a valid
/// witness always convinces the honest verifier.
pub fn sigma_completeness_ty() -> Expr {
    arrow(sigma_protocol_ty(), prop())
}
/// `SigmaSpecialSoundness : SigmaProtocol → Prop`
///
/// Special soundness: given two accepting transcripts `(a, e, z)` and
/// `(a, e', z')` with `e ≠ e'` and same commitment `a`, one can efficiently
/// extract a witness for the statement.
pub fn sigma_special_soundness_ty() -> Expr {
    arrow(sigma_protocol_ty(), prop())
}
/// `SigmaHVZK : SigmaProtocol → Prop`
///
/// Honest-Verifier Zero-Knowledge: there exists a simulator that, given
/// only the public input, produces transcripts indistinguishable from
/// real ones (when the verifier behaves honestly).
pub fn sigma_hvzk_ty() -> Expr {
    arrow(sigma_protocol_ty(), prop())
}
/// `ZKProof : Type`
///
/// A zero-knowledge proof system (P, V) for a language L:
/// - Completeness: honest prover convinces honest verifier for x ∈ L
/// - Soundness: cheating prover fails with high probability for x ∉ L
/// - Zero-knowledge: verifier learns nothing beyond x ∈ L
pub fn zk_proof_ty() -> Expr {
    type0()
}
/// `ZKCompleteness : ZKProof → Prop`
///
/// If `x ∈ L` and the prover knows witness `w`, the verifier accepts.
pub fn zk_completeness_ty() -> Expr {
    arrow(zk_proof_ty(), prop())
}
/// `ZKSoundness : ZKProof → Prop`
///
/// If `x ∉ L`, no computationally bounded cheating prover can convince
/// the verifier to accept (except with negligible probability).
pub fn zk_soundness_ty() -> Expr {
    arrow(zk_proof_ty(), prop())
}
/// `ZKZeroKnowledge : ZKProof → Prop`
///
/// The verifier's view can be simulated without a witness: the interaction
/// reveals no information about the witness beyond the statement being true.
pub fn zk_zero_knowledge_ty() -> Expr {
    arrow(zk_proof_ty(), prop())
}
/// `NIZK : Type`
///
/// Non-Interactive Zero-Knowledge proof in the random oracle model.
/// Obtained from interactive ZK via the Fiat-Shamir transform:
/// replace verifier's random challenge with a hash of the commitment.
pub fn nizk_ty() -> Expr {
    type0()
}
/// `FiatShamirTransform : SigmaProtocol → NIZK`
///
/// The Fiat-Shamir heuristic converts any Sigma protocol into a NIZK
/// in the random oracle model by hashing (statement ∥ commitment) for the challenge.
pub fn fiat_shamir_transform_ty() -> Expr {
    arrow(sigma_protocol_ty(), nizk_ty())
}
/// `ZKSnark : Type`
///
/// A zk-SNARK: Succinct Non-interactive ARgument of Knowledge.
/// - Succinct: proof size is sublinear in the circuit size
/// - Non-interactive: single message from prover to verifier
/// - Argument of Knowledge: knowledge extractor exists (with CRS)
pub fn zk_snark_ty() -> Expr {
    type0()
}
/// `SnarkSuccinctness : ZKSnark → Prop`
///
/// Succinctness: proof verification time is poly(|x|, log|C|)
/// where |C| is the circuit size and |x| is the statement size.
pub fn snark_succinctness_ty() -> Expr {
    arrow(zk_snark_ty(), prop())
}
/// `CommitmentScheme : Type`
///
/// A commitment scheme (Setup, Commit, Open, Verify):
/// - Commit(m, r) → c  (commit to m with randomness r)
/// - Verify(c, m, r) → Bool  (verify opening)
pub fn commitment_scheme_ty() -> Expr {
    type0()
}
/// `CommitHiding : CommitmentScheme → Prop`
///
/// Hiding: the commitment `c = Commit(m, r)` reveals no information
/// about `m` to a computationally bounded adversary.
pub fn commit_hiding_ty() -> Expr {
    arrow(commitment_scheme_ty(), prop())
}
/// `CommitBinding : CommitmentScheme → Prop`
///
/// Binding: it is computationally infeasible to open a commitment to
/// two different values `(m, r)` and `(m', r')` with `m ≠ m'`.
pub fn commit_binding_ty() -> Expr {
    arrow(commitment_scheme_ty(), prop())
}
/// `PedersenCommitment : CommitmentScheme`
///
/// Pedersen commitment in group G with generators g, h:
/// `Commit(m, r) = g^m · h^r`
/// - Perfectly hiding (information-theoretically secure)
/// - Computationally binding (under discrete-log assumption)
pub fn pedersen_commitment_ty() -> Expr {
    commitment_scheme_ty()
}
/// `ObliviousTransfer : Type`
///
/// 1-of-2 Oblivious Transfer: the sender has secrets (s0, s1),
/// the receiver has choice bit b ∈ {0, 1}.
/// The receiver obtains s_b; the sender learns nothing about b.
pub fn oblivious_transfer_ty() -> Expr {
    type0()
}
/// `OTReceiverPrivacy : ObliviousTransfer → Prop`
///
/// Receiver privacy: the sender cannot determine which of the two
/// secrets the receiver chose to obtain.
pub fn ot_receiver_privacy_ty() -> Expr {
    arrow(oblivious_transfer_ty(), prop())
}
/// `OTSenderPrivacy : ObliviousTransfer → Prop`
///
/// Sender privacy: the receiver learns exactly one secret and gains
/// no information about the other (beyond what is leaked by s0 = s1).
pub fn ot_sender_privacy_ty() -> Expr {
    arrow(oblivious_transfer_ty(), prop())
}
/// `OTExtension : Nat → Nat → Prop`
///
/// OT extension: given `k` base OTs, one can realize `n >> k` OTs
/// using only symmetric-key operations (Ishai-Kilian-Nissim-Petrank).
pub fn ot_extension_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ShamirSecretSharing : Nat → Nat → Type`
///
/// Shamir's (t, n)-threshold secret sharing scheme:
/// - Split a secret into n shares
/// - Any t shares suffice to reconstruct the secret
/// - Any t-1 shares reveal nothing (information-theoretically)
pub fn shamir_secret_sharing_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `SecretSharingThreshold : ShamirSecretSharing t n → Prop`
///
/// Threshold correctness: any set of t or more shares uniquely
/// determines the secret via Lagrange interpolation over Fp.
pub fn secret_sharing_threshold_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `SecretSharingPerfectPrivacy : ShamirSecretSharing t n → Prop`
///
/// Perfect privacy: any t-1 shares are identically distributed
/// regardless of the secret value (information-theoretic security).
pub fn secret_sharing_privacy_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `MPCProtocol : Type`
///
/// A multi-party computation protocol: n parties jointly compute
/// a function f(x1, ..., xn) where each party i holds input xi.
pub fn mpc_protocol_ty() -> Expr {
    type0()
}
/// `MPCSemiHonestSecurity : MPCProtocol → Prop`
///
/// Semi-honest (passive) security: a simulator exists that, given
/// a party's input and output, can reproduce its view of the protocol.
/// Protects against adversaries who follow the protocol honestly.
pub fn mpc_semi_honest_security_ty() -> Expr {
    arrow(mpc_protocol_ty(), prop())
}
/// `MPCMaliciousSecurity : MPCProtocol → Prop`
///
/// Malicious (active) security: security holds even when corrupted
/// parties deviate arbitrarily from the protocol specification.
pub fn mpc_malicious_security_ty() -> Expr {
    arrow(mpc_protocol_ty(), prop())
}
/// `GarbledCircuit : Type`
///
/// Yao's garbled circuit: a garbling of a boolean circuit that allows
/// one party to learn f(x, y) without revealing their respective inputs.
pub fn garbled_circuit_ty() -> Expr {
    type0()
}
/// `GarbledCircuitCorrectness : GarbledCircuit → Prop`
///
/// Correctness: evaluating the garbled circuit with the garbled input
/// produces the correct output of the underlying boolean function.
pub fn garbled_circuit_correctness_ty() -> Expr {
    arrow(garbled_circuit_ty(), prop())
}
/// `GarbledCircuitSecurity : GarbledCircuit → Prop`
///
/// Privacy (garbling security): the evaluator learns only the output,
/// nothing about the garbler's input beyond what the output reveals.
pub fn garbled_circuit_security_ty() -> Expr {
    arrow(garbled_circuit_ty(), prop())
}
/// `IdealFunctionality : Type`
///
/// An ideal functionality F in the UC framework: a trusted third party
/// that receives inputs from all parties and delivers outputs.
/// Security is defined by emulating this ideal world in the real world.
pub fn ideal_functionality_ty() -> Expr {
    type0()
}
/// `UCSecure : MPCProtocol → IdealFunctionality → Prop`
///
/// UC security: protocol π UC-realizes functionality F if for every
/// real-world adversary A there exists an ideal-world simulator S such that
/// no environment Z can distinguish the real and ideal executions.
pub fn uc_secure_ty() -> Expr {
    arrow(mpc_protocol_ty(), arrow(ideal_functionality_ty(), prop()))
}
/// `UCCompositionTheorem : Prop`
///
/// Universal Composition Theorem (Canetti 2001):
/// If π UC-realizes F and ρ uses F as a sub-protocol, then the
/// composed protocol ρ^π (replacing F-calls with π) UC-realizes
/// the same functionality as ρ.
pub fn uc_composition_theorem_ty() -> Expr {
    prop()
}
/// `HybridModel : IdealFunctionality → Type`
///
/// The hybrid model: a computational model where parties can access
/// an ideal functionality F as a trusted oracle (sub-protocol).
pub fn hybrid_model_ty() -> Expr {
    arrow(ideal_functionality_ty(), type0())
}
/// `UCSimulator : MPCProtocol → IdealFunctionality → Type`
///
/// A UC simulator: given an ideal-world adversary (environment + simulator),
/// it translates between the real and ideal executions.
pub fn uc_simulator_ty() -> Expr {
    arrow(mpc_protocol_ty(), arrow(ideal_functionality_ty(), type0()))
}
/// `UCEnvironment : Type`
///
/// The environment Z in the UC framework: it provides inputs to parties,
/// observes outputs, and tries to distinguish real from ideal executions.
pub fn uc_environment_ty() -> Expr {
    type0()
}
/// `UCIndistinguishable : MPCProtocol → IdealFunctionality → Prop`
///
/// Computational indistinguishability between real and ideal executions:
/// no polynomial-time environment can tell apart the two worlds.
pub fn uc_indistinguishable_ty() -> Expr {
    arrow(mpc_protocol_ty(), arrow(ideal_functionality_ty(), prop()))
}
/// `GMWProtocol : Nat → Type`
///
/// The GMW (Goldreich-Micali-Wigderson) n-party MPC protocol.
/// Achieves semi-honest security for any polynomial-time function.
pub fn gmw_protocol_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `GMWMaliciousSecure : GMWProtocol n → Prop`
///
/// Malicious security of GMW via zero-knowledge proofs at each gate.
pub fn gmw_malicious_secure_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `OTExtensionCorrectness : Nat → Nat → Prop`
///
/// IKNP OT extension correctness: the extended OTs are functionally
/// equivalent to base OTs from the receiver's perspective.
pub fn ot_extension_correctness_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `YaoGarbledCircuitPrivacy : GarbledCircuit → Prop`
///
/// Yao's privacy theorem: the evaluator's view in a garbled circuit evaluation
/// is simulable given only the circuit output and a "garbled input" (labels).
pub fn yao_garbled_privacy_ty() -> Expr {
    arrow(garbled_circuit_ty(), prop())
}
/// `KZGCommitment : Type`
///
/// KZG (Kate-Zaverucha-Goldberg) polynomial commitment scheme:
/// - Commit to a polynomial f ∈ F_p\[X\] of degree ≤ d
/// - Open at any point z with a short proof π
/// - Verification: pairing check e(C - f(z)·G, H) = e(π, (τ-z)·H)
pub fn kzg_commitment_ty() -> Expr {
    type0()
}
/// `KZGBinding : KZGCommitment → Prop`
///
/// Binding property of KZG under the d-Strong Diffie-Hellman assumption:
/// a committer cannot open to two different polynomials.
pub fn kzg_binding_ty() -> Expr {
    arrow(kzg_commitment_ty(), prop())
}
/// `VectorCommitment : Type`
///
/// A vector commitment scheme VC = (Setup, Commit, Open, Verify):
/// - Commit to a vector (m_1, ..., m_n) → C
/// - Open position i with proof π_i
/// - Binding at every position (position-binding)
pub fn vector_commitment_ty() -> Expr {
    type0()
}
/// `VectorCommitmentPositionBinding : VectorCommitment → Prop`
///
/// Position-binding: it is infeasible to open position i to two different values.
pub fn vector_commitment_position_binding_ty() -> Expr {
    arrow(vector_commitment_ty(), prop())
}
/// `Groth16Proof : Type`
///
/// Groth16 zk-SNARK: the most deployed SNARK in practice.
/// - Trusted setup phase produces a common reference string (CRS)
/// - Proof size: 3 group elements (≈192 bytes for BN254)
/// - Verification: 3 pairing operations
pub fn groth16_proof_ty() -> Expr {
    type0()
}
/// `Groth16Soundness : Groth16Proof → Prop`
///
/// Computational knowledge soundness of Groth16 under the d-PKE assumption:
/// any prover producing a valid proof must "know" a valid witness.
pub fn groth16_soundness_ty() -> Expr {
    arrow(groth16_proof_ty(), prop())
}
/// `PlonkProof : Type`
///
/// PLONK universal zk-SNARK: supports a universal trusted setup.
/// Uses permutation arguments and custom gates; proof size O(log n).
pub fn plonk_proof_ty() -> Expr {
    type0()
}
/// `PlonkUniversalSetup : PlonkProof → Prop`
///
/// Universality of PLONK: a single structured reference string suffices
/// for all circuits of size ≤ N (updateable, no per-circuit setup).
pub fn plonk_universal_setup_ty() -> Expr {
    arrow(plonk_proof_ty(), prop())
}
/// `FRIProtocol : Type`
///
/// FRI (Fast Reed-Solomon Interactive Oracle Proof of Proximity):
/// the polynomial commitment underlying STARKs.
/// Achieves transparent setup (no trusted CRS).
pub fn fri_protocol_ty() -> Expr {
    type0()
}
/// `FRISoundness : FRIProtocol → Prop`
///
/// FRI soundness: if the committed function is not close to a low-degree
/// polynomial, the verifier rejects with high probability.
pub fn fri_soundness_ty() -> Expr {
    arrow(fri_protocol_ty(), prop())
}
/// `StarkProof : Type`
///
/// STARK (Scalable Transparent ARgument of Knowledge):
/// - Transparent setup (no trusted ceremony)
/// - Post-quantum secure (hash-based)
/// - Proof size O(log^2 n), verification O(log^2 n)
pub fn stark_proof_ty() -> Expr {
    type0()
}
/// `StarkTransparency : StarkProof → Prop`
///
/// STARKs are transparent: all randomness comes from a public hash function
/// (simulated via Fiat-Shamir), no trusted setup required.
pub fn stark_transparency_ty() -> Expr {
    arrow(stark_proof_ty(), prop())
}
/// `ThresholdSignature : Nat → Nat → Type`
///
/// A (t, n)-threshold signature scheme:
/// - n parties hold shares of a secret key
/// - Any t parties can jointly sign a message
/// - Any t-1 parties learn nothing about the key or can forge signatures
pub fn threshold_signature_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ThresholdSignatureUnforgeability : ThresholdSignature t n → Prop`
///
/// Unforgeability: fewer than t parties cannot produce a valid signature.
pub fn threshold_sig_unforgeability_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `DistributedKeyGeneration : Nat → Type`
///
/// DKG (Distributed Key Generation): n parties jointly generate a shared
/// public key and distribute secret key shares, without any trusted dealer.
pub fn distributed_key_gen_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `DKGSecrecy : DistributedKeyGeneration n → Prop`
///
/// Secrecy of DKG: no threshold-minus-one coalition learns the secret key.
pub fn dkg_secrecy_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `BlindSignature : Type`
///
/// A blind signature scheme: signer signs a message without seeing it.
/// The resulting signature is valid under the signer's key for the
/// original (unblinded) message.
pub fn blind_signature_ty() -> Expr {
    type0()
}
/// `BlindnessProperty : BlindSignature → Prop`
///
/// Blindness: the signer's view during the signing protocol is
/// computationally independent of the message being signed.
pub fn blindness_property_ty() -> Expr {
    arrow(blind_signature_ty(), prop())
}
/// `BlindSignatureUnforgeability : BlindSignature → Prop`
///
/// One-more unforgeability: a user who interacts with the signer ℓ times
/// cannot produce ℓ+1 valid message-signature pairs.
pub fn blind_sig_unforgeability_ty() -> Expr {
    arrow(blind_signature_ty(), prop())
}
/// `RingSignature : Type`
///
/// A ring signature allows any member of a group (ring) to sign on behalf
/// of the group without revealing which member signed.
pub fn ring_signature_ty() -> Expr {
    type0()
}
/// `RingSignatureAnonymity : RingSignature → Prop`
///
/// Anonymity: the signer's identity is computationally hidden among all
/// ring members (even against the other ring members).
pub fn ring_sig_anonymity_ty() -> Expr {
    arrow(ring_signature_ty(), prop())
}
/// `LinkabilityProperty : RingSignature → Prop`
///
/// LSAG (Linkable SAG) linkability: two signatures by the same key
/// on different messages can be detected as linked (via a "key image").
pub fn linkability_property_ty() -> Expr {
    arrow(ring_signature_ty(), prop())
}
/// `GroupSignature : Type`
///
/// A group signature scheme: group members sign anonymously; a designated
/// opener can reveal the signer's identity when needed.
pub fn group_signature_ty() -> Expr {
    type0()
}
/// `GroupSigAnonymity : GroupSignature → Prop`
///
/// Full anonymity: no adversary can identify the signer among group members.
pub fn group_sig_anonymity_ty() -> Expr {
    arrow(group_signature_ty(), prop())
}
/// `GroupSigUnlinkability : GroupSignature → Prop`
///
/// Unlinkability: two signatures by the same member cannot be linked
/// without the opener's key.
pub fn group_sig_unlinkability_ty() -> Expr {
    arrow(group_signature_ty(), prop())
}
/// `GroupSigOpenability : GroupSignature → Prop`
///
/// Openability: the designated opener can always identify the signer
/// from any valid signature.
pub fn group_sig_openability_ty() -> Expr {
    arrow(group_signature_ty(), prop())
}
/// `AnonymousCredential : Type`
///
/// An anonymous credential (e.g., CL credentials, BBS+):
/// - Issuer certifies attributes (a_1, ..., a_k) to a user
/// - User proves possession of a valid credential for attributes
///   satisfying a predicate, without revealing the credential or linkage
pub fn anonymous_credential_ty() -> Expr {
    type0()
}
/// `CredentialUnlinkability : AnonymousCredential → Prop`
///
/// Unlinkability across showings: two presentations of the same credential
/// are computationally unlinkable.
pub fn credential_unlinkability_ty() -> Expr {
    arrow(anonymous_credential_ty(), prop())
}
/// `SelectiveDisclosure : AnonymousCredential → Prop`
///
/// Selective disclosure: the user can prove knowledge of specific attributes
/// without revealing the remaining attributes or the credential itself.
pub fn selective_disclosure_ty() -> Expr {
    arrow(anonymous_credential_ty(), prop())
}
/// `PrivateSetIntersection : Type`
///
/// PSI protocol: two parties each hold a set; at the end, one (or both)
/// learns the intersection without revealing elements not in the intersection.
pub fn private_set_intersection_ty() -> Expr {
    type0()
}
/// `PSICorrectness : PrivateSetIntersection → Prop`
///
/// Correctness: the output equals the true set intersection.
pub fn psi_correctness_ty() -> Expr {
    arrow(private_set_intersection_ty(), prop())
}
/// `PSIPrivacy : PrivateSetIntersection → Prop`
///
/// Privacy: neither party learns elements of the other's set beyond
/// those in the intersection.
pub fn psi_privacy_ty() -> Expr {
    arrow(private_set_intersection_ty(), prop())
}
/// `EVotingScheme : Type`
///
/// An electronic voting scheme with:
/// - Ballot casting: voter encrypts and submits a ballot
/// - Tallying: homomorphic decryption or mix-net reveals the tally
/// - Verification: anyone can verify correctness
pub fn e_voting_scheme_ty() -> Expr {
    type0()
}
/// `VotingVerifiability : EVotingScheme → Prop`
///
/// End-to-end verifiability: voters can verify their ballot was counted;
/// anyone can verify the tally is correctly computed from all ballots.
pub fn voting_verifiability_ty() -> Expr {
    arrow(e_voting_scheme_ty(), prop())
}
/// `VotingBallotPrivacy : EVotingScheme → Prop`
///
/// Ballot privacy: an adversary controlling the tallier learns nothing
/// about individual votes beyond the election result.
pub fn voting_ballot_privacy_ty() -> Expr {
    arrow(e_voting_scheme_ty(), prop())
}
/// `MixNetSecurity : Type → Prop`
///
/// Mix-net security: a sequence of re-encryption shuffles is correct
/// and the permutation applied is computationally hidden.
pub fn mix_net_security_ty() -> Expr {
    arrow(type0(), prop())
}
/// `AKEProtocol : Type`
///
/// An authenticated key exchange protocol (AKE):
/// two parties establish a shared session key with mutual authentication.
pub fn ake_protocol_ty() -> Expr {
    type0()
}
/// `AKEForwardSecrecy : AKEProtocol → Prop`
///
/// Forward secrecy (perfect forward secrecy): compromise of long-term keys
/// does not compromise past session keys.
pub fn ake_forward_secrecy_ty() -> Expr {
    arrow(ake_protocol_ty(), prop())
}
/// `AKEMutualAuthentication : AKEProtocol → Prop`
///
/// Mutual authentication: both parties are assured of each other's identity.
pub fn ake_mutual_auth_ty() -> Expr {
    arrow(ake_protocol_ty(), prop())
}
/// `SignalProtocolSecurity : AKEProtocol → Prop`
///
/// Signal protocol security: combines X3DH key agreement with the Double
/// Ratchet algorithm for forward secrecy and break-in recovery.
pub fn signal_protocol_security_ty() -> Expr {
    arrow(ake_protocol_ty(), prop())
}
/// `MPCWithAbort : MPCProtocol → Prop`
///
/// Security with abort: even if corrupted parties abort after seeing output,
/// honest parties' inputs remain hidden.
pub fn mpc_with_abort_ty() -> Expr {
    arrow(mpc_protocol_ty(), prop())
}
/// `FairMPC : MPCProtocol → Prop`
///
/// Fairness: either all parties receive the output or none do;
/// a corrupted party cannot abort after seeing the result while
/// preventing honest parties from learning it.
pub fn fair_mpc_ty() -> Expr {
    arrow(mpc_protocol_ty(), prop())
}
/// `GuaranteedOutput : MPCProtocol → Prop`
///
/// Guaranteed output delivery: honest parties always receive the output,
/// regardless of adversarial behavior (stronger than fairness).
pub fn guaranteed_output_ty() -> Expr {
    arrow(mpc_protocol_ty(), prop())
}
/// `ConsensusProtocol : Type`
///
/// A distributed consensus protocol (e.g., Nakamoto, BFT, Tendermint):
/// parties agree on a common value despite failures or adversarial behavior.
pub fn consensus_protocol_ty() -> Expr {
    type0()
}
/// `ConsensusConsistency : ConsensusProtocol → Prop`
///
/// Consistency (safety): all honest parties agree on the same value.
pub fn consensus_consistency_ty() -> Expr {
    arrow(consensus_protocol_ty(), prop())
}
/// `ConsensuLiveness : ConsensusProtocol → Prop`
///
/// Liveness (progress): honest parties eventually reach agreement.
pub fn consensus_liveness_ty() -> Expr {
    arrow(consensus_protocol_ty(), prop())
}
/// `NakamotoConsensus : ConsensusProtocol`
///
/// Nakamoto's longest-chain consensus: honest miners follow the chain
/// with most proof-of-work; security holds when >50% hash rate is honest.
pub fn nakamoto_consensus_ty() -> Expr {
    consensus_protocol_ty()
}
/// `MLKEMScheme : Type`
///
/// ML-KEM (formerly Kyber): NIST-standardized lattice-based KEM.
/// Security based on Module-LWE hardness assumption.
pub fn ml_kem_scheme_ty() -> Expr {
    type0()
}
/// `MLKEMInd_CCA2 : MLKEMScheme → Prop`
///
/// IND-CCA2 security of ML-KEM: ciphertext indistinguishability under
/// adaptive chosen-ciphertext attacks, assuming Module-LWE is hard.
pub fn ml_kem_ind_cca2_ty() -> Expr {
    arrow(ml_kem_scheme_ty(), prop())
}
/// `MLDSAScheme : Type`
///
/// ML-DSA (formerly Dilithium): NIST-standardized lattice-based signature.
/// Security based on Module-LWE and Module-SIS hardness.
pub fn ml_dsa_scheme_ty() -> Expr {
    type0()
}
/// `MLDSAEUFCMA : MLDSAScheme → Prop`
///
/// EUF-CMA security of ML-DSA: existential unforgeability under
/// adaptive chosen-message attacks.
pub fn ml_dsa_euf_cma_ty() -> Expr {
    arrow(ml_dsa_scheme_ty(), prop())
}
/// `SPHINCSPlusScheme : Type`
///
/// SPHINCS+: NIST-standardized hash-based signature scheme.
/// Stateless, based on XMSS and FORS; security reduces to hash function security.
pub fn sphincs_plus_scheme_ty() -> Expr {
    type0()
}
/// `SPHINCSPlusMinimalAssumptions : SPHINCSPlusScheme → Prop`
///
/// SPHINCS+ achieves EUF-CMA under minimal assumptions (collision-resistance
/// of the underlying hash function), making it the most conservative PQ option.
pub fn sphincs_plus_minimal_assumptions_ty() -> Expr {
    arrow(sphincs_plus_scheme_ty(), prop())
}
/// `HomomorphicEncryption : Type`
///
/// A homomorphic encryption scheme supporting operations on ciphertexts
/// that correspond to operations on plaintexts.
pub fn homomorphic_encryption_ty() -> Expr {
    type0()
}
/// `PHESecurity : HomomorphicEncryption → Prop`
///
/// Semantic security (IND-CPA) of partially homomorphic encryption.
pub fn phe_security_ty() -> Expr {
    arrow(homomorphic_encryption_ty(), prop())
}
/// `FHEBootstrapping : HomomorphicEncryption → Prop`
///
/// FHE bootstrapping: refreshing a ciphertext to reduce noise, enabling
/// arbitrarily deep circuit evaluation (Gentry's blueprint).
pub fn fhe_bootstrapping_ty() -> Expr {
    arrow(homomorphic_encryption_ty(), prop())
}
/// Register all cryptographic protocol axioms into the kernel environment.
pub fn build_cryptographic_protocols_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("Proto.DYMessage", dy_message_ty()),
        ("Proto.DYPrincipal", dy_principal_ty()),
        ("Proto.DYKnowledge", dy_knowledge_ty()),
        ("Proto.DYDerivable", dy_derivable_ty()),
        ("Proto.DYAttack", dy_attack_ty()),
        ("Proto.DYIntruder", dy_intruder_ty()),
        ("Proto.Protocol", type0()),
        ("Proto.Confidentiality", confidentiality_ty()),
        ("Proto.Integrity", integrity_ty()),
        ("Proto.Authentication", authentication_ty()),
        ("Proto.NonRepudiation", non_repudiation_ty()),
        ("Proto.FreshNonce", fresh_nonce_ty()),
        ("Proto.ProtocolCompose", protocol_compose_ty()),
        ("Proto.ProtocolParallel", protocol_parallel_ty()),
        (
            "Proto.SecurityPreservedUnderComposition",
            security_preserved_composition_ty(),
        ),
        ("Proto.SigmaProtocol", sigma_protocol_ty()),
        ("Proto.SigmaCommitment", sigma_commitment_ty()),
        ("Proto.SigmaChallenge", sigma_challenge_ty()),
        ("Proto.SigmaResponse", sigma_response_ty()),
        ("Proto.SigmaCompleteness", sigma_completeness_ty()),
        ("Proto.SigmaSpecialSoundness", sigma_special_soundness_ty()),
        ("Proto.SigmaHVZK", sigma_hvzk_ty()),
        ("Proto.ZKProof", zk_proof_ty()),
        ("Proto.ZKCompleteness", zk_completeness_ty()),
        ("Proto.ZKSoundness", zk_soundness_ty()),
        ("Proto.ZKZeroKnowledge", zk_zero_knowledge_ty()),
        ("Proto.NIZK", nizk_ty()),
        ("Proto.FiatShamirTransform", fiat_shamir_transform_ty()),
        ("Proto.ZKSnark", zk_snark_ty()),
        ("Proto.SnarkSuccinctness", snark_succinctness_ty()),
        ("Proto.CommitmentScheme", commitment_scheme_ty()),
        ("Proto.CommitHiding", commit_hiding_ty()),
        ("Proto.CommitBinding", commit_binding_ty()),
        ("Proto.ObliviousTransfer", oblivious_transfer_ty()),
        ("Proto.OTReceiverPrivacy", ot_receiver_privacy_ty()),
        ("Proto.OTSenderPrivacy", ot_sender_privacy_ty()),
        ("Proto.OTExtension", ot_extension_ty()),
        ("Proto.ShamirSecretSharing", shamir_secret_sharing_ty()),
        (
            "Proto.SecretSharingThreshold",
            secret_sharing_threshold_ty(),
        ),
        (
            "Proto.SecretSharingPerfectPrivacy",
            secret_sharing_privacy_ty(),
        ),
        ("Proto.MPCProtocol", mpc_protocol_ty()),
        ("Proto.MPCSemiHonestSecurity", mpc_semi_honest_security_ty()),
        ("Proto.MPCMaliciousSecurity", mpc_malicious_security_ty()),
        ("Proto.GarbledCircuit", garbled_circuit_ty()),
        (
            "Proto.GarbledCircuitCorrectness",
            garbled_circuit_correctness_ty(),
        ),
        (
            "Proto.GarbledCircuitSecurity",
            garbled_circuit_security_ty(),
        ),
        ("Proto.IdealFunctionality", ideal_functionality_ty()),
        ("Proto.UCSecure", uc_secure_ty()),
        ("Proto.UCCompositionTheorem", uc_composition_theorem_ty()),
        ("Proto.HybridModel", hybrid_model_ty()),
        ("Proto.UCSimulator", uc_simulator_ty()),
        ("Proto.UCEnvironment", uc_environment_ty()),
        ("Proto.UCIndistinguishable", uc_indistinguishable_ty()),
        ("Proto.GMWProtocol", gmw_protocol_ty()),
        ("Proto.GMWMaliciousSecure", gmw_malicious_secure_ty()),
        (
            "Proto.OTExtensionCorrectness",
            ot_extension_correctness_ty(),
        ),
        ("Proto.YaoGarbledCircuitPrivacy", yao_garbled_privacy_ty()),
        ("Proto.KZGCommitment", kzg_commitment_ty()),
        ("Proto.KZGBinding", kzg_binding_ty()),
        ("Proto.VectorCommitment", vector_commitment_ty()),
        (
            "Proto.VectorCommitmentPositionBinding",
            vector_commitment_position_binding_ty(),
        ),
        ("Proto.Groth16Proof", groth16_proof_ty()),
        ("Proto.Groth16Soundness", groth16_soundness_ty()),
        ("Proto.PlonkProof", plonk_proof_ty()),
        ("Proto.PlonkUniversalSetup", plonk_universal_setup_ty()),
        ("Proto.FRIProtocol", fri_protocol_ty()),
        ("Proto.FRISoundness", fri_soundness_ty()),
        ("Proto.StarkProof", stark_proof_ty()),
        ("Proto.StarkTransparency", stark_transparency_ty()),
        ("Proto.ThresholdSignature", threshold_signature_ty()),
        (
            "Proto.ThresholdSignatureUnforgeability",
            threshold_sig_unforgeability_ty(),
        ),
        ("Proto.DistributedKeyGeneration", distributed_key_gen_ty()),
        ("Proto.DKGSecrecy", dkg_secrecy_ty()),
        ("Proto.BlindSignature", blind_signature_ty()),
        ("Proto.BlindnessProperty", blindness_property_ty()),
        (
            "Proto.BlindSignatureUnforgeability",
            blind_sig_unforgeability_ty(),
        ),
        ("Proto.RingSignature", ring_signature_ty()),
        ("Proto.RingSignatureAnonymity", ring_sig_anonymity_ty()),
        ("Proto.LinkabilityProperty", linkability_property_ty()),
        ("Proto.GroupSignature", group_signature_ty()),
        ("Proto.GroupSigAnonymity", group_sig_anonymity_ty()),
        ("Proto.GroupSigUnlinkability", group_sig_unlinkability_ty()),
        ("Proto.GroupSigOpenability", group_sig_openability_ty()),
        ("Proto.AnonymousCredential", anonymous_credential_ty()),
        (
            "Proto.CredentialUnlinkability",
            credential_unlinkability_ty(),
        ),
        ("Proto.SelectiveDisclosure", selective_disclosure_ty()),
        (
            "Proto.PrivateSetIntersection",
            private_set_intersection_ty(),
        ),
        ("Proto.PSICorrectness", psi_correctness_ty()),
        ("Proto.PSIPrivacy", psi_privacy_ty()),
        ("Proto.EVotingScheme", e_voting_scheme_ty()),
        ("Proto.VotingVerifiability", voting_verifiability_ty()),
        ("Proto.VotingBallotPrivacy", voting_ballot_privacy_ty()),
        ("Proto.MixNetSecurity", mix_net_security_ty()),
        ("Proto.AKEProtocol", ake_protocol_ty()),
        ("Proto.AKEForwardSecrecy", ake_forward_secrecy_ty()),
        ("Proto.AKEMutualAuthentication", ake_mutual_auth_ty()),
        (
            "Proto.SignalProtocolSecurity",
            signal_protocol_security_ty(),
        ),
        ("Proto.MPCWithAbort", mpc_with_abort_ty()),
        ("Proto.FairMPC", fair_mpc_ty()),
        ("Proto.GuaranteedOutput", guaranteed_output_ty()),
        ("Proto.ConsensusProtocol", consensus_protocol_ty()),
        ("Proto.ConsensusConsistency", consensus_consistency_ty()),
        ("Proto.ConsensusLiveness", consensus_liveness_ty()),
        ("Proto.MLKEMScheme", ml_kem_scheme_ty()),
        ("Proto.MLKEMInd_CCA2", ml_kem_ind_cca2_ty()),
        ("Proto.MLDSAScheme", ml_dsa_scheme_ty()),
        ("Proto.MLDSAEUFCMA", ml_dsa_euf_cma_ty()),
        ("Proto.SPHINCSPlusScheme", sphincs_plus_scheme_ty()),
        (
            "Proto.SPHINCSPlusMinimalAssumptions",
            sphincs_plus_minimal_assumptions_ty(),
        ),
        ("Proto.HomomorphicEncryption", homomorphic_encryption_ty()),
        ("Proto.PHESecurity", phe_security_ty()),
        ("Proto.FHEBootstrapping", fhe_bootstrapping_ty()),
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
/// Fast modular exponentiation: `base^exp mod m`.
pub fn mod_exp(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut result: u128 = 1;
    let mut b = base as u128 % m as u128;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * b % m as u128;
        }
        exp >>= 1;
        b = b * b % m as u128;
    }
    base = result as u64;
    base
}
/// Extended GCD: returns (gcd, x, y) with a*x + b*y = gcd.
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = ext_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}
/// Modular inverse of a mod m (if gcd(a,m)=1).
pub fn mod_inv(a: u64, m: u64) -> Option<u64> {
    let (g, x, _) = ext_gcd(a as i64, m as i64);
    if g != 1 {
        return None;
    }
    Some(((x % m as i64 + m as i64) % m as i64) as u64)
}
/// Toy "encryption": H(key_a XOR key_b) XOR plaintext (using identity hash).
pub fn toy_encrypt(key_a: u64, key_b: u64, plaintext: u64) -> u64 {
    let key = key_a
        .wrapping_mul(0x9e3779b97f4a7c15)
        .wrapping_add(key_b.wrapping_mul(0x6c62272e07bb0142));
    plaintext ^ key
}
/// Create a tiny Paillier instance with p=7, q=13 (n=91).
///
/// Parameters: λ = lcm(6, 12) = 12; g = 92; μ = 38.
/// These satisfy gcd(λ, n) = 1, which is required for correct decryption.
///
/// Pre-computed: L(g^λ mod n^2) = 12, μ = 12^{-1} mod 91 = 38.
pub fn tiny_paillier() -> PaillierHomomorphic {
    let p: u64 = 7;
    let q: u64 = 13;
    let n = p * q;
    let n_sq = (n as u128) * (n as u128);
    let g = n + 1;
    let lambda = 12u64;
    let mu = 38u64;
    PaillierHomomorphic {
        n,
        n_sq,
        g,
        lambda,
        mu,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Environment;
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_cryptographic_protocols_env(&mut env);
        assert!(
            result.is_ok(),
            "build_cryptographic_protocols_env failed: {:?}",
            result
        );
    }
    #[test]
    fn test_schnorr_proof() {
        let params = SchnorrParams { p: 23, q: 11, g: 2 };
        let secret_x = 5u64;
        let y = mod_exp(params.g, secret_x, params.p);
        assert_eq!(y, 9);
        let r = 7u64;
        let challenge = 3u64;
        let transcript = params.prove(secret_x, r, challenge);
        assert!(
            params.verify(&transcript, y),
            "Schnorr verification failed for valid proof"
        );
    }
    #[test]
    fn test_schnorr_invalid_witness() {
        let params = SchnorrParams { p: 23, q: 11, g: 2 };
        let secret_x = 5u64;
        let wrong_x = 6u64;
        let y = mod_exp(params.g, secret_x, params.p);
        let r = 7u64;
        let challenge = 3u64;
        let bad_transcript = params.prove(wrong_x, r, challenge);
        assert!(
            !params.verify(&bad_transcript, y),
            "Schnorr should reject proof with wrong witness"
        );
    }
    #[test]
    fn test_pedersen_commitment() {
        let params = PedersenParams {
            p: 23,
            q: 11,
            g: 2,
            h: 3,
        };
        let m = 4u64;
        let r = 6u64;
        let c = params.commit(m, r);
        assert!(
            params.verify(c, m, r),
            "Pedersen commitment should verify correctly"
        );
        assert!(
            !params.verify(c, m + 1, r),
            "Different message should not verify"
        );
    }
    #[test]
    fn test_pedersen_homomorphic() {
        let params = PedersenParams {
            p: 23,
            q: 11,
            g: 2,
            h: 3,
        };
        let (m1, r1) = (2u64, 3u64);
        let (m2, r2) = (1u64, 4u64);
        let c1 = params.commit(m1, r1);
        let c2 = params.commit(m2, r2);
        let c_sum = params.add_commitments(c1, c2);
        let c_direct = params.commit((m1 + m2) % params.q, (r1 + r2) % params.q);
        assert_eq!(c_sum, c_direct, "Pedersen commitment should be homomorphic");
    }
    #[test]
    fn test_shamir_secret_sharing() {
        let ss = ShamirSS { p: 97, t: 3, n: 5 };
        let secret = 42u64;
        let coeffs = [7u64, 13u64];
        let shares = ss.split(secret, &coeffs);
        assert_eq!(shares.len(), 5, "Should produce 5 shares");
        let reconstructed = ss.reconstruct(&shares[..3]);
        assert_eq!(
            reconstructed, secret,
            "Reconstructed secret should match original"
        );
        let reconstructed2 = ss.reconstruct(&shares[2..5]);
        assert_eq!(
            reconstructed2, secret,
            "Any t shares should reconstruct secret"
        );
    }
    #[test]
    fn test_mpc_xor_share() {
        let s0 = MpcShare {
            party: 0,
            share: true,
        };
        let s1 = MpcShare {
            party: 1,
            share: false,
        };
        assert!(
            MpcShare::reconstruct(&s0, &s1),
            "XOR shares should reconstruct to true"
        );
        let a0 = MpcShare {
            party: 0,
            share: true,
        };
        let b0 = MpcShare {
            party: 0,
            share: false,
        };
        let result = MpcShare::xor_gate(&a0, &b0);
        assert!(
            result.share,
            "XOR(true, false) share for party 0 should be true"
        );
    }
    #[test]
    fn test_dy_derivable_axiom_registered() {
        let mut env = Environment::new();
        build_cryptographic_protocols_env(&mut env)
            .expect("build_cryptographic_protocols_env should succeed");
        assert!(
            env.get(&Name::str("Proto.DYDerivable")).is_some(),
            "Proto.DYDerivable should be registered"
        );
        assert!(
            env.get(&Name::str("Proto.UCCompositionTheorem")).is_some(),
            "Proto.UCCompositionTheorem should be registered"
        );
    }
    #[test]
    fn test_uc_composition_is_prop() {
        let ty = uc_composition_theorem_ty();
        assert_eq!(ty, prop(), "UC composition theorem should have type Prop");
    }
    #[test]
    fn test_new_axioms_registered() {
        let mut env = Environment::new();
        build_cryptographic_protocols_env(&mut env)
            .expect("build_cryptographic_protocols_env should succeed");
        assert!(env.get(&Name::str("Proto.UCSimulator")).is_some());
        assert!(env.get(&Name::str("Proto.UCEnvironment")).is_some());
        assert!(env.get(&Name::str("Proto.UCIndistinguishable")).is_some());
        assert!(env.get(&Name::str("Proto.Groth16Proof")).is_some());
        assert!(env.get(&Name::str("Proto.PlonkProof")).is_some());
        assert!(env.get(&Name::str("Proto.FRIProtocol")).is_some());
        assert!(env.get(&Name::str("Proto.StarkProof")).is_some());
        assert!(env.get(&Name::str("Proto.MLKEMScheme")).is_some());
        assert!(env.get(&Name::str("Proto.MLDSAScheme")).is_some());
        assert!(env.get(&Name::str("Proto.SPHINCSPlusScheme")).is_some());
        assert!(env.get(&Name::str("Proto.RingSignature")).is_some());
        assert!(env.get(&Name::str("Proto.GroupSignature")).is_some());
        assert!(env.get(&Name::str("Proto.BlindSignature")).is_some());
        assert!(env.get(&Name::str("Proto.AnonymousCredential")).is_some());
        assert!(env
            .get(&Name::str("Proto.PrivateSetIntersection"))
            .is_some());
        assert!(env.get(&Name::str("Proto.EVotingScheme")).is_some());
        assert!(env.get(&Name::str("Proto.ConsensusProtocol")).is_some());
        assert!(env.get(&Name::str("Proto.HomomorphicEncryption")).is_some());
        assert!(env.get(&Name::str("Proto.FHEBootstrapping")).is_some());
    }
    #[test]
    fn test_pedersen_commitment_struct() {
        let pc = PedersenCommitment {
            p: 23,
            q: 11,
            g: 2,
            h: 3,
        };
        let m = 3u64;
        let r = 5u64;
        let c = pc.commit(m, r);
        assert!(
            pc.verify(c, m, r),
            "PedersenCommitment verify should succeed"
        );
        assert!(
            !pc.verify(c, m + 1, r),
            "PedersenCommitment verify should fail for wrong m"
        );
    }
    #[test]
    fn test_pedersen_commitment_batch() {
        let pc = PedersenCommitment {
            p: 23,
            q: 11,
            g: 2,
            h: 3,
        };
        let pairs = [(1u64, 2u64), (3u64, 4u64), (5u64, 6u64)];
        let commitments = pc.batch_commit(&pairs);
        assert_eq!(commitments.len(), 3);
        for (i, &(m, r)) in pairs.iter().enumerate() {
            assert!(pc.verify(commitments[i], m, r));
        }
    }
    #[test]
    fn test_pedersen_commitment_homomorphic() {
        let pc = PedersenCommitment {
            p: 23,
            q: 11,
            g: 2,
            h: 3,
        };
        let (m1, r1) = (2u64, 3u64);
        let (m2, r2) = (1u64, 4u64);
        let c1 = pc.commit(m1, r1);
        let c2 = pc.commit(m2, r2);
        let c_add = pc.add(c1, c2);
        let c_direct = pc.commit((m1 + m2) % pc.q, (r1 + r2) % pc.q);
        assert_eq!(c_add, c_direct, "Homomorphic add should be correct");
    }
    #[test]
    fn test_shamir_extended_share_reconstruct() {
        let ss = ShamirSecretSharingExtended { p: 97, t: 3, n: 5 };
        let secret = 42u64;
        let coeffs = [7u64, 13u64];
        let shares = ss.share(secret, &coeffs);
        assert_eq!(shares.len(), 5);
        let reconstructed = ss.reconstruct(&shares[..3]);
        assert_eq!(
            reconstructed, secret,
            "Extended Shamir reconstruction should match secret"
        );
        let reconstructed2 = ss.reconstruct(&shares[2..5]);
        assert_eq!(
            reconstructed2, secret,
            "Any t shares should reconstruct the secret"
        );
    }
    #[test]
    fn test_garbled_and_gate() {
        let labels_a = [10u64, 20u64];
        let labels_b = [30u64, 40u64];
        let labels_out = [50u64, 60u64];
        let gate = GarbledGate::garble_and(labels_a, labels_b, labels_out);
        let out = gate
            .evaluate(labels_a[0], labels_b[0])
            .expect("evaluate should succeed");
        assert!(!gate.is_output_one(out), "AND(0,0) should be 0");
        let out = gate
            .evaluate(labels_a[1], labels_b[1])
            .expect("evaluate should succeed");
        assert!(gate.is_output_one(out), "AND(1,1) should be 1");
        let out = gate
            .evaluate(labels_a[1], labels_b[0])
            .expect("evaluate should succeed");
        assert!(!gate.is_output_one(out), "AND(1,0) should be 0");
    }
    #[test]
    fn test_garbled_or_gate() {
        let labels_a = [100u64, 200u64];
        let labels_b = [300u64, 400u64];
        let labels_out = [500u64, 600u64];
        let gate = GarbledGate::garble_or(labels_a, labels_b, labels_out);
        let out = gate
            .evaluate(labels_a[0], labels_b[0])
            .expect("evaluate should succeed");
        assert!(!gate.is_output_one(out), "OR(0,0) should be 0");
        let out = gate
            .evaluate(labels_a[0], labels_b[1])
            .expect("evaluate should succeed");
        assert!(gate.is_output_one(out), "OR(0,1) should be 1");
        let out = gate
            .evaluate(labels_a[1], labels_b[0])
            .expect("evaluate should succeed");
        assert!(gate.is_output_one(out), "OR(1,0) should be 1");
    }
    #[test]
    fn test_paillier_homomorphic_encrypt_decrypt() {
        let ph = tiny_paillier();
        let m = 7u64;
        let r = 2u64;
        let c = ph.encrypt(m, r);
        let decrypted = ph.decrypt(c);
        assert_eq!(
            decrypted,
            m % ph.n,
            "Paillier decrypt should recover plaintext"
        );
    }
    #[test]
    fn test_paillier_homomorphic_add() {
        let ph = tiny_paillier();
        let m1 = 3u64;
        let m2 = 4u64;
        let c1 = ph.encrypt(m1, 2);
        let c2 = ph.encrypt(m2, 3);
        let c_sum = ph.add_ciphertexts(c1, c2);
        let decrypted = ph.decrypt(c_sum);
        assert_eq!(
            decrypted,
            (m1 + m2) % ph.n,
            "Homomorphic Paillier add should recover sum"
        );
    }
    #[test]
    fn test_blind_signature_full_protocol() {
        let bs = BlindSignatureScheme { n: 55, e: 3, d: 27 };
        let m = 7u64;
        let r = 8u64;
        let blinded = bs.blind(m, r);
        let s_prime = bs.sign_blinded(blinded);
        let s = bs.unblind(s_prime, r);
        assert!(
            bs.verify(m, s),
            "Blind signature verification should succeed after unblinding"
        );
    }
    #[test]
    fn test_stark_and_groth16_are_type0() {
        assert_eq!(stark_proof_ty(), type0(), "StarkProof should be Type");
        assert_eq!(groth16_proof_ty(), type0(), "Groth16Proof should be Type");
    }
    #[test]
    fn test_consensus_liveness_is_prop() {
        let ty = consensus_liveness_ty();
        assert_ne!(ty, type0(), "ConsensusLiveness should not be Type");
    }
}
#[cfg(test)]
mod tests_crypto_extra {
    use super::*;
    #[test]
    fn test_zk_proof_systems() {
        let groth = ZKProofSystem::groth16();
        assert!(groth.is_non_interactive());
        assert!(groth.is_succinct());
        let plonk = ZKProofSystem::plonk();
        assert!(plonk.is_non_interactive());
        let schnorr = ZKProofSystem::schnorr();
        assert!(!schnorr.is_non_interactive());
    }
    #[test]
    fn test_commitment_scheme() {
        let ped = CommitmentScheme::pedersen();
        assert!(ped.is_perfectly_hiding);
        assert!(ped.is_homomorphic);
        let sha = CommitmentScheme::sha256_hash();
        assert!(!sha.is_homomorphic);
    }
    #[test]
    fn test_oblivious_transfer() {
        let ot = ObliviousTransfer::new(OTVariant::OneOutOfTwo, 128);
        assert!(ot.is_fundamental());
        assert_eq!(ot.n_messages(), 2);
        let ot_n = ObliviousTransfer::new(OTVariant::OneOutOfN(10), 128);
        assert_eq!(ot_n.n_messages(), 10);
    }
    #[test]
    fn test_mpc_protocol() {
        let bgw = MPCProtocol::bgw(4, 1);
        assert!(bgw.is_optimal_corruption_threshold());
        assert!(bgw.is_secure_against_majority_corruption());
    }
    #[test]
    fn test_secret_sharing() {
        let ss = SecretSharing::shamir_2_of_3();
        assert_eq!(ss.threshold, 2);
        assert_eq!(ss.n_shares, 3);
        assert!(ss.is_perfect());
        assert_eq!(ss.min_shares_needed(), 2);
    }
}
