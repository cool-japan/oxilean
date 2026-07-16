//! Huge `natVal` payloads must round-trip exactly through the kernel's BigNat
//! (decimal string in → BigNat → decimal string out), never through u64/f64.

use oxilean_export::{read_str, ExportDecl};
use oxilean_kernel::{BigNat, Expr, Literal};

const META: &str = r#"{"meta":{"exporter":{"name":"lean4export","version":"3.1.0"},"format":{"version":"3.1.0"},"lean":{"githash":"x","version":"4.32.0-rc1"}}}"#;

/// Wrap a natVal literal in a minimal valid export file (an axiom whose type is
/// the literal expression — the reader converts without type-checking).
fn file_with_natval(decimal: &str) -> String {
    format!(
        "{META}\n{}\n{}\n{}",
        r#"{"in":1,"str":{"pre":0,"str":"bigConst"}}"#,
        format_args!(r#"{{"ie":0,"natVal":"{decimal}"}}"#),
        r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":1,"type":0}}"#,
    )
}

fn extract_nat(input: &str) -> BigNat {
    let f = read_str(input).unwrap_or_else(|e| panic!("must parse: {e}"));
    assert_eq!(f.decls.len(), 1);
    let ExportDecl::Axiom(ax) = &f.decls[0] else {
        panic!("expected axiom");
    };
    let Expr::Lit(Literal::Nat(n)) = &ax.common.ty else {
        panic!("expected nat literal, got {:?}", ax.common.ty);
    };
    n.clone()
}

#[test]
fn ten_to_the_thirty_roundtrips_through_bignat() {
    // 10^30 — 31 digits, far beyond u64::MAX (~1.8 * 10^19) and not exactly
    // representable as f64.
    let decimal = "1000000000000000000000000000000";
    let n = extract_nat(&file_with_natval(decimal));

    // Exact round-trip back to decimal.
    assert_eq!(n.to_string(), decimal);
    // Matches an independently-constructed BigNat.
    let expected =
        BigNat::from_decimal_str(decimal).unwrap_or_else(|| panic!("BigNat must accept {decimal}"));
    assert_eq!(n, expected);
    // Provably not squeezed through u64.
    assert_eq!(n.to_u64(), None, "10^30 must not fit u64");
    // 10^30 needs 100 bits => two 64-bit limbs.
    assert_eq!(n.limb_count(), 2);
}

#[test]
fn u64_boundary_values_roundtrip() {
    for decimal in ["0", "1", "18446744073709551615", "18446744073709551616"] {
        let n = extract_nat(&file_with_natval(decimal));
        assert_eq!(n.to_string(), decimal, "round-trip failure for {decimal}");
    }
}

#[test]
fn leading_zeros_normalize_but_preserve_value() {
    // BigNat::from_decimal_str accepts leading zeros; the value (not the
    // spelling) is what round-trips.
    let n = extract_nat(&file_with_natval("000123"));
    assert_eq!(n.to_string(), "123");
    assert_eq!(n.to_u64(), Some(123));
}

#[test]
fn kilodigit_natval_roundtrips() {
    // A 1000-digit literal: 10^999.
    let mut decimal = String::from("1");
    decimal.push_str(&"0".repeat(999));
    let n = extract_nat(&file_with_natval(&decimal));
    assert_eq!(n.to_string(), decimal);
    assert!(n.bit_length() > 3300, "10^999 is a ~3320-bit number");
}
