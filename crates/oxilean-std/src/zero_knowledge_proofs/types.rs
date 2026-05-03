//! Types for zero-knowledge proof systems.

/// A public statement in a ZK proof system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZkStatement {
    /// Human-readable description of the statement.
    pub description: String,
    /// List of public parameter names.
    pub public_params: Vec<String>,
}

/// The private witness (secret values) for a ZK proof.
#[derive(Clone, Debug)]
pub struct ZkWitness {
    /// Secret integer values known only to the prover.
    pub secret_values: Vec<i64>,
}

/// A Pedersen-like commitment: `value = g^x * h^r mod p`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commitment {
    /// The commitment value.
    pub value: u64,
    /// The randomness used to create the commitment.
    pub randomness: u64,
}

/// A Sigma-protocol proof (3-message proof: commit → challenge → response).
#[derive(Clone, Debug)]
pub struct ZkProof {
    /// Fiat-Shamir or interactive challenge.
    pub challenge: u64,
    /// Prover responses (one per secret).
    pub response: Vec<i64>,
    /// Prover commitments (one per secret).
    pub commitment: Vec<u64>,
}

/// Properties of a Sigma protocol instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigmaProtocol {
    /// Name of the protocol (e.g. "Schnorr").
    pub name: String,
    /// Whether the protocol satisfies completeness.
    pub completeness: bool,
    /// Whether the protocol satisfies soundness.
    pub soundness: bool,
    /// Whether the protocol satisfies (honest-verifier) zero-knowledge.
    pub zero_knowledge: bool,
}

/// Pedersen commitment parameters over a small prime field.
///
/// Commitment: `g^x * h^r mod p`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PedersenParams {
    /// Generator g.
    pub g: u64,
    /// Blinding generator h.
    pub h: u64,
    /// Prime modulus p.
    pub p: u64,
}

/// A Schnorr-like proof of knowledge of a discrete logarithm.
///
/// Proves: knows `x` such that `g^x ≡ y (mod p)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DlogProof {
    /// Commitment `R = g^k mod p` for random nonce k.
    pub commitment: u64,
    /// Fiat-Shamir challenge `c = H(R, y)`.
    pub challenge: u64,
    /// Response `s = k − c*x` (may be negative; reduce mod (p−1) for verification).
    pub response: i64,
}

/// A range proof decomposing a value into bits, each with its own DlogProof.
///
/// Proves: `lo ≤ x ≤ hi` by committing to each bit of `x − lo`.
#[derive(Clone, Debug)]
pub struct RangeProofBits {
    /// One DlogProof per bit of `x − lo`.
    pub bits: Vec<DlogProof>,
    /// The claimed range `[lo, hi]`.
    pub range: (u64, u64),
}
