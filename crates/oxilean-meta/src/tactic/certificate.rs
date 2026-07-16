//! Proof certificate types produced by decision-procedure tactics.
//!
//! A `ProofCertificate` carries the witness produced by a tactic (e.g., omega)
//! so the elaborator can attempt real proof-term reconstruction instead of
//! inserting a `sorry` placeholder.

use crate::tactic::linear_combination::{FarkasCert, Rat};
use crate::tactic::omega::OmegaProof;
use oxilean_kernel::Expr;

/// A polyrith certificate: per-hypothesis polynomial coefficients.
///
/// Given `h_i : p_i = 0` for each entry, the certificate claims:
/// `goal_poly = Σ entries[i].coeff * p_i` (as a polynomial identity).
#[derive(Debug, Clone)]
pub struct PolyrithCert {
    /// The goal polynomial (in canonical string form for display purposes).
    pub goal: String,
    /// Per-hypothesis coefficients.  `constraint_index` references the
    /// hypothesis index in the elaboration context; `coeff` is the rational
    /// multiplier found by the Gröbner basis solver.
    pub entries: Vec<PolyrithCertEntry>,
    /// True iff OxiZ-math independently confirmed the algebraic identity
    /// via Buchberger's algorithm on the same polynomial system.
    pub validated: bool,
}

/// A single entry in a `PolyrithCert`.
#[derive(Debug, Clone)]
pub struct PolyrithCertEntry {
    /// Index into the hypothesis list (0-based).
    pub constraint_index: usize,
    /// Rational coefficient for this hypothesis in the polynomial combination.
    pub coeff: Rat,
}

/// A proof certificate produced by a decision-procedure tactic.
#[derive(Debug, Clone)]
pub enum ProofCertificate {
    /// Certificate from the omega linear arithmetic decision procedure.
    Omega(OmegaProof),
    /// Certificate from the Farkas/linarith procedure.
    Linarith(FarkasCert),
    /// Certificate from the polyrith Gröbner basis procedure.
    Polyrith(PolyrithCert),
    /// A proof term already verified meta-side; elab should re-verify with
    /// TypeChecker and use directly.
    Direct(Expr),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::linear_combination::{ConOrient, FarkasCert, FarkasCertEntry, Rat};
    use crate::tactic::omega::{OmegaProof, OmegaStep};

    #[test]
    fn test_polyrith_cert_construction() {
        let cert = PolyrithCert {
            goal: "x + y - 3".to_string(),
            entries: vec![
                PolyrithCertEntry {
                    constraint_index: 0,
                    coeff: Rat { numer: 1, denom: 1 },
                },
                PolyrithCertEntry {
                    constraint_index: 1,
                    coeff: Rat { numer: 1, denom: 1 },
                },
            ],
            validated: false,
        };
        assert_eq!(cert.entries.len(), 2);
        assert_eq!(cert.goal, "x + y - 3");
    }

    #[test]
    fn test_polyrith_cert_debug_round_trip() {
        let cert = PolyrithCert {
            goal: "a^2 - b^2".to_string(),
            entries: vec![PolyrithCertEntry {
                constraint_index: 0,
                coeff: Rat { numer: 2, denom: 3 },
            }],
            validated: false,
        };
        let debug_str = format!("{:?}", cert);
        assert!(debug_str.contains("PolyrithCert"));
        assert!(debug_str.contains("a^2 - b^2"));
    }

    #[test]
    fn test_polyrith_cert_entry_debug() {
        let entry = PolyrithCertEntry {
            constraint_index: 5,
            coeff: Rat {
                numer: -1,
                denom: 2,
            },
        };
        let debug_str = format!("{:?}", entry);
        assert!(debug_str.contains("PolyrithCertEntry"));
        assert!(debug_str.contains("5"));
    }

    #[test]
    fn test_proof_certificate_polyrith_variant() {
        let cert = PolyrithCert {
            goal: "test".to_string(),
            entries: vec![],
            validated: false,
        };
        let pc = ProofCertificate::Polyrith(cert);
        assert!(matches!(pc, ProofCertificate::Polyrith(_)));
    }

    #[test]
    fn test_proof_certificate_linarith_variant() {
        let farkas = FarkasCert {
            entries: vec![FarkasCertEntry {
                constraint_index: 0,
                multiplier: Rat { numer: 1, denom: 1 },
                orient: ConOrient::GeZero,
            }],
            combined_rhs: Rat { numer: 1, denom: 1 },
            sources: vec![],
        };
        let pc = ProofCertificate::Linarith(farkas);
        assert!(matches!(pc, ProofCertificate::Linarith(_)));
    }

    #[test]
    fn test_proof_certificate_omega_variant() {
        let proof = OmegaProof {
            steps: vec![OmegaStep::Contradiction { value: -1 }],
        };
        let pc = ProofCertificate::Omega(proof);
        assert!(matches!(pc, ProofCertificate::Omega(_)));
    }

    #[test]
    fn test_polyrith_cert_empty_entries() {
        let cert = PolyrithCert {
            goal: "0".to_string(),
            entries: vec![],
            validated: false,
        };
        assert!(cert.entries.is_empty());
    }

    #[test]
    fn test_polyrith_cert_clone() {
        let cert = PolyrithCert {
            goal: "x - 1".to_string(),
            entries: vec![PolyrithCertEntry {
                constraint_index: 0,
                coeff: Rat { numer: 1, denom: 1 },
            }],
            validated: false,
        };
        let cloned = cert.clone();
        assert_eq!(cloned.goal, cert.goal);
        assert_eq!(cloned.entries.len(), cert.entries.len());
    }
}
