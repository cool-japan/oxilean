//! Functions for the Proof Certificate System.
//!
//! Provides creation, verification, serialization, and deserialization of
//! `ProofCertificate` values using FNV-1a structural hashing.

use std::collections::HashMap;

use crate::declaration::ConstantInfo;
use crate::env::Environment;
use crate::Expr;

use super::types::{CertCheckResult, CertificateStore, ProofCertId, ProofCertificate, ProofStep};

// ---------------------------------------------------------------------------
// FNV-1a constants
// ---------------------------------------------------------------------------

const FNV1A_OFFSET: u64 = 14695981039346656037;
const FNV1A_PRIME: u64 = 1099511628211;

/// Update a FNV-1a running hash with a single byte.
#[inline]
fn fnv1a_byte(hash: u64, byte: u8) -> u64 {
    (hash ^ (byte as u64)).wrapping_mul(FNV1A_PRIME)
}

/// Update a FNV-1a running hash with a u32.
#[inline]
fn fnv1a_u32(hash: u64, val: u32) -> u64 {
    let bytes = val.to_le_bytes();
    let h = fnv1a_byte(hash, bytes[0]);
    let h = fnv1a_byte(h, bytes[1]);
    let h = fnv1a_byte(h, bytes[2]);
    fnv1a_byte(h, bytes[3])
}

/// Update a FNV-1a running hash with a u64.
#[inline]
fn fnv1a_u64(hash: u64, val: u64) -> u64 {
    let bytes = val.to_le_bytes();
    bytes.iter().fold(hash, |h, &b| fnv1a_byte(h, b))
}

/// Update a FNV-1a running hash with a string slice.
#[inline]
fn fnv1a_str(hash: u64, s: &str) -> u64 {
    s.as_bytes().iter().fold(hash, |h, &b| fnv1a_byte(h, b))
}

// ---------------------------------------------------------------------------
// Discriminants for Expr variants (for structurally hashing without Display)
// ---------------------------------------------------------------------------

const DISC_SORT: u8 = 0;
const DISC_BVAR: u8 = 1;
const DISC_FVAR: u8 = 2;
const DISC_CONST: u8 = 3;
const DISC_APP: u8 = 4;
const DISC_LAM: u8 = 5;
const DISC_PI: u8 = 6;
const DISC_LET: u8 = 7;
const DISC_LIT: u8 = 8;
const DISC_PROJ: u8 = 9;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute a structural FNV-1a hash of an expression.
///
/// The hash is deterministic and depends only on the structure of `e`.
/// It is designed to distinguish structurally distinct expressions efficiently,
/// though it is not a cryptographic hash.
pub fn hash_expr(e: &Expr) -> u64 {
    hash_expr_impl(e, FNV1A_OFFSET)
}

fn hash_expr_impl(e: &Expr, seed: u64) -> u64 {
    match e {
        Expr::Sort(level) => {
            let h = fnv1a_byte(seed, DISC_SORT);
            // Hash the level via its debug representation as a stable string
            let repr = format!("{:?}", level);
            fnv1a_str(h, &repr)
        }
        Expr::BVar(idx) => {
            let h = fnv1a_byte(seed, DISC_BVAR);
            fnv1a_u32(h, *idx)
        }
        Expr::FVar(fvar_id) => {
            let h = fnv1a_byte(seed, DISC_FVAR);
            let repr = format!("{:?}", fvar_id);
            fnv1a_str(h, &repr)
        }
        Expr::Const(name, levels) => {
            let h = fnv1a_byte(seed, DISC_CONST);
            let h = fnv1a_str(h, &format!("{}", name));
            levels
                .iter()
                .fold(h, |acc, lvl| fnv1a_str(acc, &format!("{:?}", lvl)))
        }
        Expr::App(f, arg) => {
            let h = fnv1a_byte(seed, DISC_APP);
            let h = hash_expr_impl(f, h);
            hash_expr_impl(arg, h)
        }
        Expr::Lam(binfo, name, ty, body) => {
            let h = fnv1a_byte(seed, DISC_LAM);
            let h = fnv1a_byte(h, *binfo as u8);
            let h = fnv1a_str(h, &format!("{}", name));
            let h = hash_expr_impl(ty, h);
            hash_expr_impl(body, h)
        }
        Expr::Pi(binfo, name, ty, body) => {
            let h = fnv1a_byte(seed, DISC_PI);
            let h = fnv1a_byte(h, *binfo as u8);
            let h = fnv1a_str(h, &format!("{}", name));
            let h = hash_expr_impl(ty, h);
            hash_expr_impl(body, h)
        }
        Expr::Let(name, ty, val, body) => {
            let h = fnv1a_byte(seed, DISC_LET);
            let h = fnv1a_str(h, &format!("{}", name));
            let h = hash_expr_impl(ty, h);
            let h = hash_expr_impl(val, h);
            hash_expr_impl(body, h)
        }
        Expr::Lit(lit) => {
            let h = fnv1a_byte(seed, DISC_LIT);
            fnv1a_str(h, &format!("{:?}", lit))
        }
        Expr::Proj(name, idx, expr) => {
            let h = fnv1a_byte(seed, DISC_PROJ);
            let h = fnv1a_str(h, &format!("{}", name));
            let h = fnv1a_u32(h, *idx);
            hash_expr_impl(expr, h)
        }
    }
}

/// Compute a structural FNV-1a hash of a `ConstantInfo` declaration.
///
/// The hash covers the declaration name, its type expression, and its
/// optional value expression (for definitions/theorems).
pub fn hash_declaration(decl: &ConstantInfo) -> u64 {
    let mut h = FNV1A_OFFSET;
    h = fnv1a_str(h, &format!("{}", decl.name()));
    h = hash_expr_impl(decl.ty(), h);
    if let Some(val) = decl.value() {
        h = hash_expr_impl(val, h);
    }
    h
}

/// Create a new `ProofCertificate` for the given declaration.
///
/// The certificate ID is derived from the combined hash of the type and proof
/// expressions. The `verified_at` timestamp is set to 0 in no-std/deterministic
/// contexts; callers may override it by replacing the field.
pub fn create_certificate(
    decl_name: &str,
    type_expr: &Expr,
    proof_expr: &Expr,
) -> ProofCertificate {
    let type_hash = hash_expr(type_expr);
    let proof_hash = hash_expr(proof_expr);
    // Derive a stable certificate ID from the two hashes.
    let id_raw = fnv1a_u64(fnv1a_u64(FNV1A_OFFSET, type_hash), proof_hash);
    ProofCertificate {
        id: ProofCertId::new(id_raw),
        decl_name: decl_name.to_string(),
        type_hash,
        proof_hash,
        reduction_steps: Vec::new(),
        verified_at: 0,
    }
}

/// Verify a certificate against the live environment.
///
/// Checks that:
/// 1. The declaration named in the certificate exists in `env`.
/// 2. The stored type hash matches the recomputed hash of the live type.
/// 3. The stored proof hash matches the recomputed hash of the live value (if any).
/// 4. The reduction step sequence is well-formed (no Iota steps with ctor_idx == u32::MAX).
pub fn verify_certificate(cert: &ProofCertificate, env: &Environment) -> CertCheckResult {
    use crate::Name;

    let name = Name::from_str(cert.decl_name.as_str());
    let ci = match env.find(&name) {
        Some(ci) => ci,
        None => return CertCheckResult::MissingDecl(cert.decl_name.clone()),
    };

    // Verify type hash.
    let live_type_hash = hash_expr(ci.ty());
    if live_type_hash != cert.type_hash {
        return CertCheckResult::HashMismatch {
            expected: cert.type_hash,
            actual: live_type_hash,
        };
    }

    // Verify proof hash (if the declaration carries a value).
    if let Some(val) = ci.value() {
        let live_proof_hash = hash_expr(val);
        if live_proof_hash != cert.proof_hash {
            return CertCheckResult::HashMismatch {
                expected: cert.proof_hash,
                actual: live_proof_hash,
            };
        }
    }

    // Validate reduction step well-formedness.
    if !validate_steps(&cert.reduction_steps) {
        return CertCheckResult::InvalidSteps;
    }

    CertCheckResult::Valid
}

/// Validate that a sequence of reduction steps is internally well-formed.
///
/// Currently checks:
/// - `Iota` steps must not use `u32::MAX` as a constructor index (sentinel).
/// - `Delta` and `Iota` step names must be non-empty.
fn validate_steps(steps: &[ProofStep]) -> bool {
    for step in steps {
        match step {
            ProofStep::Iota { recursor, ctor_idx }
                if recursor.is_empty() || *ctor_idx == u32::MAX =>
            {
                return false;
            }
            ProofStep::Delta { name } if name.is_empty() => {
                return false;
            }
            _ => {}
        }
    }
    true
}

impl CertificateStore {
    /// Add a certificate to the store, overwriting any existing entry
    /// for the same declaration name.
    pub fn add(&mut self, cert: ProofCertificate) {
        self.certs.insert(cert.decl_name.clone(), cert);
    }

    /// Look up a certificate by declaration name.
    pub fn get(&self, name: &str) -> Option<&ProofCertificate> {
        self.certs.get(name)
    }

    /// Verify every certificate in the store against the given environment.
    ///
    /// Returns a map from declaration name to verification result.
    pub fn verify_all(&self, env: &Environment) -> HashMap<String, CertCheckResult> {
        self.certs
            .iter()
            .map(|(name, cert)| (name.clone(), verify_certificate(cert, env)))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Serialization / deserialization
// ---------------------------------------------------------------------------

/// Serialize a `ProofCertificate` to a compact text format.
///
/// Format:
/// ```text
/// OXICERT v1
/// id:<id>
/// decl:<decl_name>
/// type_hash:<hex>
/// proof_hash:<hex>
/// verified_at:<timestamp>
/// steps:<count>
/// <step_0>
/// <step_1>
/// ...
/// ```
pub fn serialize_cert(cert: &ProofCertificate) -> String {
    let mut out = String::with_capacity(256);
    out.push_str("OXICERT v1\n");
    out.push_str(&format!("id:{}\n", cert.id.raw()));
    out.push_str(&format!("decl:{}\n", cert.decl_name));
    out.push_str(&format!("type_hash:{:016x}\n", cert.type_hash));
    out.push_str(&format!("proof_hash:{:016x}\n", cert.proof_hash));
    out.push_str(&format!("verified_at:{}\n", cert.verified_at));
    out.push_str(&format!("steps:{}\n", cert.reduction_steps.len()));
    for step in &cert.reduction_steps {
        out.push_str(&serialize_step(step));
        out.push('\n');
    }
    out
}

/// Serialize a single `ProofStep` to its text representation.
fn serialize_step(step: &ProofStep) -> String {
    match step {
        ProofStep::Beta { redex_depth } => format!("Beta {}", redex_depth),
        ProofStep::Delta { name } => format!("Delta {}", name),
        ProofStep::Zeta => "Zeta".to_string(),
        ProofStep::Iota { recursor, ctor_idx } => format!("Iota {} {}", recursor, ctor_idx),
        ProofStep::Eta => "Eta".to_string(),
        ProofStep::SubstLevel { params } => format!("SubstLevel {}", params.join(",")),
        ProofStep::Assumption => "Assumption".to_string(),
    }
}

/// Deserialize a `ProofCertificate` from the compact text format produced by
/// [`serialize_cert`].
///
/// Returns `Err(String)` with a human-readable message on any parse failure.
pub fn deserialize_cert(s: &str) -> Result<ProofCertificate, String> {
    let mut lines = s.lines();

    // Header
    let header = lines.next().ok_or("missing header line")?;
    if header.trim() != "OXICERT v1" {
        return Err(format!("unrecognised header: {:?}", header));
    }

    macro_rules! next_field {
        ($prefix:literal) => {{
            let line = lines
                .next()
                .ok_or_else(|| format!("missing field '{}'", $prefix))?;
            if !line.starts_with($prefix) {
                return Err(format!("expected '{}', got {:?}", $prefix, line));
            }
            &line[$prefix.len()..]
        }};
    }

    let id_raw: u64 = next_field!("id:")
        .parse()
        .map_err(|e| format!("bad id: {}", e))?;
    let decl_name = next_field!("decl:").to_string();
    let type_hash = u64::from_str_radix(next_field!("type_hash:"), 16)
        .map_err(|e| format!("bad type_hash: {}", e))?;
    let proof_hash = u64::from_str_radix(next_field!("proof_hash:"), 16)
        .map_err(|e| format!("bad proof_hash: {}", e))?;
    let verified_at: u64 = next_field!("verified_at:")
        .parse()
        .map_err(|e| format!("bad verified_at: {}", e))?;
    let step_count: usize = next_field!("steps:")
        .parse()
        .map_err(|e| format!("bad steps count: {}", e))?;

    let mut reduction_steps = Vec::with_capacity(step_count);
    for i in 0..step_count {
        let line = lines.next().ok_or_else(|| format!("missing step {}", i))?;
        let step = deserialize_step(line).map_err(|e| format!("error in step {}: {}", i, e))?;
        reduction_steps.push(step);
    }

    Ok(ProofCertificate {
        id: ProofCertId::new(id_raw),
        decl_name,
        type_hash,
        proof_hash,
        reduction_steps,
        verified_at,
    })
}

/// Deserialize a single `ProofStep` from a text line.
fn deserialize_step(line: &str) -> Result<ProofStep, String> {
    let mut parts = line.splitn(2, ' ');
    let tag = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("").trim();

    match tag {
        "Beta" => {
            let depth: u32 = rest.parse().map_err(|e| format!("bad Beta depth: {}", e))?;
            Ok(ProofStep::Beta { redex_depth: depth })
        }
        "Delta" => {
            if rest.is_empty() {
                return Err("Delta requires a name".to_string());
            }
            Ok(ProofStep::Delta {
                name: rest.to_string(),
            })
        }
        "Zeta" => Ok(ProofStep::Zeta),
        "Iota" => {
            let mut iota_parts = rest.splitn(2, ' ');
            let recursor = iota_parts
                .next()
                .filter(|s| !s.is_empty())
                .ok_or("Iota requires recursor name")?
                .to_string();
            let ctor_idx: u32 = iota_parts
                .next()
                .ok_or("Iota requires ctor_idx")?
                .parse()
                .map_err(|e| format!("bad Iota ctor_idx: {}", e))?;
            Ok(ProofStep::Iota { recursor, ctor_idx })
        }
        "Eta" => Ok(ProofStep::Eta),
        "SubstLevel" => {
            let params = if rest.is_empty() {
                Vec::new()
            } else {
                rest.split(',').map(|s| s.to_string()).collect()
            };
            Ok(ProofStep::SubstLevel { params })
        }
        "Assumption" => Ok(ProofStep::Assumption),
        other => Err(format!("unknown proof step tag: {:?}", other)),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expr, Level, Name};

    fn prop() -> Expr {
        Expr::Sort(Level::Zero)
    }

    fn type0() -> Expr {
        Expr::Sort(Level::succ(Level::Zero))
    }

    fn const_expr(name: &str) -> Expr {
        Expr::Const(Name::from_str(name), vec![])
    }

    fn bvar(n: u32) -> Expr {
        Expr::BVar(n)
    }

    // --- hash_expr tests ---

    #[test]
    fn test_hash_sort_prop() {
        let h = hash_expr(&prop());
        assert_ne!(h, 0);
    }

    #[test]
    fn test_hash_sort_type0() {
        let h0 = hash_expr(&prop());
        let h1 = hash_expr(&type0());
        assert_ne!(h0, h1, "Prop and Type should hash differently");
    }

    #[test]
    fn test_hash_bvar_distinct() {
        let h0 = hash_expr(&bvar(0));
        let h1 = hash_expr(&bvar(1));
        assert_ne!(h0, h1);
    }

    #[test]
    fn test_hash_const() {
        let h_nat = hash_expr(&const_expr("Nat"));
        let h_int = hash_expr(&const_expr("Int"));
        assert_ne!(h_nat, h_int);
    }

    #[test]
    fn test_hash_app() {
        let f = const_expr("f");
        let a = const_expr("a");
        let app = Expr::App(Box::new(f), Box::new(a));
        let h = hash_expr(&app);
        assert_ne!(h, 0);
    }

    #[test]
    fn test_hash_lam() {
        let ty = prop();
        let body = bvar(0);
        let lam = Expr::Lam(
            crate::BinderInfo::Default,
            Name::from_str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let h = hash_expr(&lam);
        assert_ne!(h, 0);
    }

    #[test]
    fn test_hash_pi() {
        let ty = prop();
        let body = bvar(0);
        let pi = Expr::Pi(
            crate::BinderInfo::Default,
            Name::from_str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let h = hash_expr(&pi);
        assert_ne!(h, 0);
    }

    #[test]
    fn test_hash_deterministic() {
        let e = Expr::App(Box::new(const_expr("Nat.succ")), Box::new(bvar(0)));
        assert_eq!(hash_expr(&e), hash_expr(&e));
    }

    #[test]
    fn test_hash_let() {
        let e = Expr::Let(
            Name::from_str("x"),
            Box::new(prop()),
            Box::new(bvar(0)),
            Box::new(bvar(1)),
        );
        let h = hash_expr(&e);
        assert_ne!(h, 0);
    }

    #[test]
    fn test_hash_proj() {
        let e = Expr::Proj(Name::from_str("Prod.fst"), 0, Box::new(const_expr("p")));
        let h = hash_expr(&e);
        assert_ne!(h, 0);
    }

    // --- create_certificate tests ---

    #[test]
    fn test_create_certificate_fields() {
        let ty = prop();
        let pf = bvar(0);
        let cert = create_certificate("MyThm", &ty, &pf);
        assert_eq!(cert.decl_name, "MyThm");
        assert_eq!(cert.type_hash, hash_expr(&ty));
        assert_eq!(cert.proof_hash, hash_expr(&pf));
        assert!(cert.reduction_steps.is_empty());
    }

    #[test]
    fn test_create_certificate_id_stable() {
        let ty = prop();
        let pf = bvar(0);
        let c1 = create_certificate("T", &ty, &pf);
        let c2 = create_certificate("T", &ty, &pf);
        assert_eq!(c1.id, c2.id);
    }

    #[test]
    fn test_create_certificate_distinct_names() {
        let ty = prop();
        let pf = bvar(0);
        let c1 = create_certificate("Thm1", &ty, &pf);
        let c2 = create_certificate("Thm2", &ty, &pf);
        // Same type/proof → same type_hash/proof_hash, but decl_name differs.
        assert_eq!(c1.type_hash, c2.type_hash);
        assert_ne!(c1.decl_name, c2.decl_name);
    }

    // --- serialize / deserialize tests ---

    #[test]
    fn test_roundtrip_empty_steps() {
        let cert = create_certificate("RoundTripThm", &prop(), &bvar(0));
        let s = serialize_cert(&cert);
        let cert2 = deserialize_cert(&s).expect("deserialize should succeed");
        assert_eq!(cert, cert2);
    }

    #[test]
    fn test_roundtrip_with_steps() {
        let mut cert = create_certificate("StepThm", &type0(), &const_expr("Nat"));
        cert.reduction_steps = vec![
            ProofStep::Beta { redex_depth: 2 },
            ProofStep::Delta {
                name: "Nat.add".to_string(),
            },
            ProofStep::Zeta,
            ProofStep::Iota {
                recursor: "Nat.rec".to_string(),
                ctor_idx: 1,
            },
            ProofStep::Eta,
            ProofStep::SubstLevel {
                params: vec!["u".to_string(), "v".to_string()],
            },
            ProofStep::Assumption,
        ];
        let s = serialize_cert(&cert);
        let cert2 = deserialize_cert(&s).expect("deserialize should succeed");
        assert_eq!(cert, cert2);
    }

    #[test]
    fn test_serialize_header() {
        let cert = create_certificate("T", &prop(), &bvar(0));
        let s = serialize_cert(&cert);
        assert!(s.starts_with("OXICERT v1\n"));
    }

    #[test]
    fn test_deserialize_bad_header() {
        let result = deserialize_cert("BADHEADER\nid:0\ndecl:X\ntype_hash:0000000000000000\nproof_hash:0000000000000000\nverified_at:0\nsteps:0\n");
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_missing_field() {
        // Missing proof_hash and later fields.
        let result = deserialize_cert("OXICERT v1\nid:1\ndecl:T\ntype_hash:0000000000000001\n");
        assert!(result.is_err());
    }

    // --- CertificateStore tests ---

    #[test]
    fn test_store_add_get() {
        let mut store = CertificateStore::new();
        let cert = create_certificate("Thm", &prop(), &bvar(0));
        store.add(cert.clone());
        let got = store.get("Thm").expect("should find cert");
        assert_eq!(got.decl_name, "Thm");
    }

    #[test]
    fn test_store_get_missing() {
        let store = CertificateStore::new();
        assert!(store.get("Nonexistent").is_none());
    }

    #[test]
    fn test_store_overwrite() {
        let mut store = CertificateStore::new();
        let c1 = create_certificate("Thm", &prop(), &bvar(0));
        let c2 = create_certificate("Thm", &type0(), &bvar(1));
        store.add(c1);
        store.add(c2.clone());
        let got = store.get("Thm").expect("should find updated cert");
        assert_eq!(got.type_hash, c2.type_hash);
    }

    #[test]
    fn test_store_len() {
        let mut store = CertificateStore::new();
        assert_eq!(store.len(), 0);
        store.add(create_certificate("A", &prop(), &bvar(0)));
        store.add(create_certificate("B", &type0(), &bvar(0)));
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_store_is_empty() {
        let store = CertificateStore::new();
        assert!(store.is_empty());
    }

    // --- verify_certificate tests ---

    #[test]
    fn test_verify_missing_decl() {
        let env = Environment::new();
        let cert = create_certificate("Missing", &prop(), &bvar(0));
        let result = verify_certificate(&cert, &env);
        assert_eq!(result, CertCheckResult::MissingDecl("Missing".to_string()));
    }

    // --- validate_steps tests ---

    #[test]
    fn test_validate_steps_empty() {
        assert!(validate_steps(&[]));
    }

    #[test]
    fn test_validate_steps_invalid_iota() {
        let steps = vec![ProofStep::Iota {
            recursor: "Nat.rec".to_string(),
            ctor_idx: u32::MAX,
        }];
        assert!(!validate_steps(&steps));
    }

    #[test]
    fn test_validate_steps_empty_delta_name() {
        let steps = vec![ProofStep::Delta {
            name: String::new(),
        }];
        assert!(!validate_steps(&steps));
    }

    #[test]
    fn test_validate_steps_valid_mixed() {
        let steps = vec![
            ProofStep::Beta { redex_depth: 0 },
            ProofStep::Delta {
                name: "foo".to_string(),
            },
            ProofStep::Zeta,
            ProofStep::Eta,
            ProofStep::Assumption,
        ];
        assert!(validate_steps(&steps));
    }

    // --- CertCheckResult display ---

    #[test]
    fn test_cert_check_result_display_valid() {
        let r = CertCheckResult::Valid;
        assert_eq!(format!("{}", r), "Valid");
    }

    #[test]
    fn test_cert_check_result_display_mismatch() {
        let r = CertCheckResult::HashMismatch {
            expected: 0xdeadbeef,
            actual: 0xcafe,
        };
        let s = format!("{}", r);
        assert!(s.contains("HashMismatch"));
    }
}
