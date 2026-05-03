//! Proof Certificate System for OxiLean Kernel.
//!
//! A proof certificate is a compact, checkable record that a term has been
//! verified by the kernel. Certificates enable exporting and importing proofs
//! without re-checking from scratch: the consumer verifies structural hashes
//! and, optionally, replays reduction steps.
//!
//! # Overview
//!
//! 1. **Create** — call [`create_certificate`] immediately after the kernel
//!    accepts a declaration to produce a `ProofCertificate`.
//! 2. **Store** — add certificates to a [`CertificateStore`] for persistence.
//! 3. **Verify** — call [`verify_certificate`] (or [`CertificateStore::verify_all`])
//!    to check that stored hashes still match the live environment.
//! 4. **Serialize / deserialize** — use [`serialize_cert`] / [`deserialize_cert`]
//!    to persist certificates as text (e.g. in `.oxicert` files).

pub mod functions;
pub mod types;

pub use functions::{
    create_certificate, deserialize_cert, hash_declaration, hash_expr, serialize_cert,
    verify_certificate,
};
pub use types::{CertCheckResult, CertificateStore, ProofCertId, ProofCertificate, ProofStep};
