//! Types for the Proof Certificate System.
//!
//! A proof certificate is a compact, checkable record that a term has been verified.
//! Certificates enable exporting/importing proofs without re-checking from scratch.

use std::collections::HashMap;

/// Unique identifier for a proof certificate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProofCertId(pub u64);

impl ProofCertId {
    /// Create a new certificate ID from a raw value.
    pub fn new(id: u64) -> Self {
        ProofCertId(id)
    }

    /// Get the raw numeric value.
    pub fn raw(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for ProofCertId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cert#{}", self.0)
    }
}

/// A single step in a proof reduction sequence.
///
/// These steps record the high-level reduction moves taken during kernel
/// type-checking, enabling efficient replay or auditing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProofStep {
    /// β-reduction: substitute the argument into the body.
    ///
    /// `redex_depth` records the nesting depth at which this reduction occurred.
    Beta {
        /// De Bruijn depth of the beta-redex site.
        redex_depth: u32,
    },
    /// δ-reduction: unfold a named definition.
    Delta {
        /// Name of the definition that was unfolded.
        name: String,
    },
    /// ζ-reduction: substitute a let-binding.
    Zeta,
    /// ι-reduction: reduce a recursor application to a branch.
    Iota {
        /// Name of the recursor that fired.
        recursor: String,
        /// Index of the constructor branch selected.
        ctor_idx: u32,
    },
    /// η-reduction: contract `λ x, f x` to `f`.
    Eta,
    /// Universe-level substitution.
    SubstLevel {
        /// Parameter names substituted.
        params: Vec<String>,
    },
    /// Appeal to a local assumption (hypothesis) in the context.
    Assumption,
}

impl std::fmt::Display for ProofStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProofStep::Beta { redex_depth } => write!(f, "Beta(depth={})", redex_depth),
            ProofStep::Delta { name } => write!(f, "Delta({})", name),
            ProofStep::Zeta => write!(f, "Zeta"),
            ProofStep::Iota { recursor, ctor_idx } => {
                write!(f, "Iota({}, ctor={})", recursor, ctor_idx)
            }
            ProofStep::Eta => write!(f, "Eta"),
            ProofStep::SubstLevel { params } => write!(f, "SubstLevel({:?})", params),
            ProofStep::Assumption => write!(f, "Assumption"),
        }
    }
}

/// A compact verification record for a single declaration.
///
/// Certificates record the structural hashes of the type and proof term, together
/// with the sequence of reduction steps performed during kernel type-checking.
/// They can be stored externally and replayed cheaply via hash verification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofCertificate {
    /// Unique certificate identifier (derived from content at creation time).
    pub id: ProofCertId,
    /// Name of the declaration this certificate vouches for.
    pub decl_name: String,
    /// FNV-1a structural hash of the declaration's type expression.
    pub type_hash: u64,
    /// FNV-1a structural hash of the proof term expression.
    pub proof_hash: u64,
    /// Ordered sequence of reduction steps taken during type-checking.
    pub reduction_steps: Vec<ProofStep>,
    /// Unix timestamp (seconds) at which the certificate was created.
    pub verified_at: u64,
}

impl ProofCertificate {
    /// Return the number of reduction steps recorded.
    pub fn step_count(&self) -> usize {
        self.reduction_steps.len()
    }

    /// Return true if this certificate records no reduction steps.
    pub fn is_trivial(&self) -> bool {
        self.reduction_steps.is_empty()
    }
}

impl std::fmt::Display for ProofCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProofCert {{ id: {}, decl: {}, type_hash: {:016x}, proof_hash: {:016x}, steps: {} }}",
            self.id,
            self.decl_name,
            self.type_hash,
            self.proof_hash,
            self.reduction_steps.len()
        )
    }
}

/// Persistent store of proof certificates, keyed by declaration name.
#[derive(Clone, Debug, Default)]
pub struct CertificateStore {
    /// Map from declaration name to its certificate.
    pub certs: HashMap<String, ProofCertificate>,
}

impl CertificateStore {
    /// Create an empty certificate store.
    pub fn new() -> Self {
        CertificateStore {
            certs: HashMap::new(),
        }
    }

    /// Return the number of certificates in the store.
    pub fn len(&self) -> usize {
        self.certs.len()
    }

    /// Return true if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.certs.is_empty()
    }

    /// Iterate over all (name, certificate) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &ProofCertificate)> {
        self.certs.iter()
    }
}

/// The result of checking a proof certificate against a live environment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CertCheckResult {
    /// The certificate is consistent with the environment.
    Valid,
    /// The stored hash does not match the recomputed hash.
    HashMismatch {
        /// The hash stored in the certificate.
        expected: u64,
        /// The hash recomputed from the environment.
        actual: u64,
    },
    /// The declaration named in the certificate is absent from the environment.
    MissingDecl(String),
    /// The reduction steps are internally inconsistent or ill-formed.
    InvalidSteps,
}

impl std::fmt::Display for CertCheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CertCheckResult::Valid => write!(f, "Valid"),
            CertCheckResult::HashMismatch { expected, actual } => {
                write!(
                    f,
                    "HashMismatch {{ expected: {:016x}, actual: {:016x} }}",
                    expected, actual
                )
            }
            CertCheckResult::MissingDecl(name) => write!(f, "MissingDecl({})", name),
            CertCheckResult::InvalidSteps => write!(f, "InvalidSteps"),
        }
    }
}
