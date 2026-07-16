//! Regression tests for kernel literal reduction with the BigNat payload.
//!
//! These exercise the real kernel path (`Reducer::whnf_env` /
//! `DefEqChecker::is_def_eq`), pinning:
//!
//! 1. the u64-overflow soundness fix (2^64-scale arithmetic is now exact),
//! 2. exact Lean semantics for every special-cased operation,
//! 3. argument WHNF before literal folding (nested arithmetic folds),
//! 4. exact-arity firing (over-application stays stuck, args never dropped),
//! 5. the pow/shiftLeft memory guard (stuck, never wrong),
//! 6. genuine `Bool.true` / `Bool.false` constants from `Nat.beq`/`Nat.ble`.

use oxilean_kernel::bignat::BigNat;
use oxilean_kernel::Node;
use oxilean_kernel::{init_builtin_env, DefEqChecker, Environment, Expr, Literal, Name, Reducer};

fn env() -> Environment {
    let mut env = Environment::new();
    match init_builtin_env(&mut env) {
        Ok(()) => env,
        Err(e) => unreachable!("builtin env must initialize: {e}"),
    }
}

fn nat(n: u64) -> Expr {
    Expr::Lit(Literal::nat(n))
}

fn big(s: &str) -> Expr {
    match BigNat::from_decimal_str(s) {
        Some(v) => Expr::Lit(Literal::Nat(v)),
        None => unreachable!("test literal must parse: {s}"),
    }
}

fn op2(name: &str, a: Expr, b: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::App(
            Node::new(Expr::Const(Name::str(name), vec![])),
            Node::new(a),
        )),
        Node::new(b),
    )
}

fn op1(name: &str, a: Expr) -> Expr {
    Expr::App(
        Node::new(Expr::Const(Name::str(name), vec![])),
        Node::new(a),
    )
}

fn whnf(e: &Expr) -> Expr {
    let env = env();
    let mut reducer = Reducer::new();
    reducer.whnf_env(e, &env)
}

fn bool_const(b: bool) -> Expr {
    Expr::Const(
        Name::str(if b { "Bool.true" } else { "Bool.false" }),
        vec![],
    )
}

// ── 1. overflow soundness regressions ────────────────────────────────────────

#[test]
fn overflow_regression_mul_2_64_scale_is_correct() {
    // The old u64 kernel reduced Nat.mul (2^32) (2^32) to literal 0 in
    // release builds — a proof of `2^32 * 2^32 = 0` would have been accepted.
    let e = op2("Nat.mul", nat(1 << 32), nat(1 << 32));
    assert_eq!(whnf(&e), big("18446744073709551616"));
    // And it must NOT be defeq to 0.
    let env = env();
    let mut dec = DefEqChecker::new(&env);
    assert!(!dec.is_def_eq(&e, &nat(0)));
    assert!(dec.is_def_eq(&e, &big("18446744073709551616")));
}

#[test]
fn overflow_regression_add_u64_max() {
    // Debug builds used to panic here; release builds wrapped to 0.
    let e = op2("Nat.add", nat(u64::MAX), nat(1));
    assert_eq!(whnf(&e), big("18446744073709551616"));
}

#[test]
fn overflow_regression_pow_2_64() {
    // u64::pow used to panic (debug) / wrap (release).
    let e = op2("Nat.pow", nat(2), nat(64));
    assert_eq!(whnf(&e), big("18446744073709551616"));
    let e100 = op2("Nat.pow", nat(2), nat(100));
    assert_eq!(whnf(&e100), big("1267650600228229401496703205376"));
}

#[test]
fn overflow_regression_shift_left_keeps_high_bits() {
    // (2^63) <<< 1 used to reduce to 0.
    let e = op2("Nat.shiftLeft", nat(1 << 63), nat(1));
    assert_eq!(whnf(&e), big("18446744073709551616"));
}

// ── 2. exact Lean semantics ──────────────────────────────────────────────────

#[test]
fn lean_semantics_div_mod_zero() {
    assert_eq!(whnf(&op2("Nat.div", nat(7), nat(0))), nat(0)); // x / 0 = 0
    assert_eq!(whnf(&op2("Nat.mod", nat(7), nat(0))), nat(7)); // x % 0 = x
    assert_eq!(whnf(&op2("Nat.mod", nat(10), nat(3))), nat(1));
    assert_eq!(whnf(&op2("Nat.div", nat(10), nat(3))), nat(3));
}

#[test]
fn lean_semantics_sub_truncated() {
    assert_eq!(whnf(&op2("Nat.sub", nat(3), nat(5))), nat(0));
    assert_eq!(whnf(&op2("Nat.sub", nat(5), nat(3))), nat(2));
}

#[test]
fn lean_semantics_pow_zero_zero() {
    assert_eq!(whnf(&op2("Nat.pow", nat(0), nat(0))), nat(1)); // 0^0 = 1
}

#[test]
fn lean_semantics_log2() {
    assert_eq!(whnf(&op1("Nat.log2", nat(0))), nat(0)); // log2 0 = 0
    assert_eq!(whnf(&op1("Nat.log2", nat(1))), nat(0));
    assert_eq!(whnf(&op1("Nat.log2", nat(1024))), nat(10));
    assert_eq!(whnf(&op1("Nat.log2", big("18446744073709551616"))), nat(64));
}

#[test]
fn lean_semantics_string_length_counts_chars_not_bytes() {
    // "é" is 2 bytes but 1 character; the old kernel returned 2.
    let e = op1("String.length", Expr::Lit(Literal::Str("é".to_string())));
    assert_eq!(whnf(&e), nat(1));
    let e2 = op1(
        "String.length",
        Expr::Lit(Literal::Str("日本語🎌".to_string())),
    );
    assert_eq!(whnf(&e2), nat(4));
    let e3 = op1(
        "String.length",
        Expr::Lit(Literal::Str("hello".to_string())),
    );
    assert_eq!(whnf(&e3), nat(5));
}

#[test]
fn lean_semantics_gcd_bitwise_shiftright() {
    assert_eq!(whnf(&op2("Nat.gcd", nat(12), nat(8))), nat(4));
    assert_eq!(
        whnf(&op2("Nat.land", nat(0b1100), nat(0b1010))),
        nat(0b1000)
    );
    assert_eq!(whnf(&op2("Nat.lor", nat(0b1100), nat(0b1010))), nat(0b1110));
    assert_eq!(whnf(&op2("Nat.xor", nat(0b1100), nat(0b1010))), nat(0b0110));
    // shiftRight by >= bit-length is 0, including shift counts >= 64.
    assert_eq!(whnf(&op2("Nat.shiftRight", nat(u64::MAX), nat(64))), nat(0));
    assert_eq!(whnf(&op2("Nat.shiftRight", nat(u64::MAX), nat(63))), nat(1));
    assert_eq!(
        whnf(&op2("Nat.shiftRight", big("18446744073709551616"), nat(64))),
        nat(1)
    );
}

#[test]
fn lean_semantics_succ_pred() {
    assert_eq!(
        whnf(&op1("Nat.succ", nat(u64::MAX))),
        big("18446744073709551616")
    );
    assert_eq!(whnf(&op1("Nat.pred", nat(0))), nat(0));
    assert_eq!(
        whnf(&op1("Nat.pred", big("18446744073709551616"))),
        nat(u64::MAX)
    );
}

#[test]
fn lean_semantics_nat_zero_const_as_literal_arg() {
    // Lean's is_nat_lit_ext: the constant Nat.zero acts as literal 0.
    let zero_const = Expr::Const(Name::str("Nat.zero"), vec![]);
    assert_eq!(whnf(&op2("Nat.add", zero_const.clone(), nat(5))), nat(5));
    assert_eq!(whnf(&op1("Nat.succ", zero_const)), nat(1));
}

// ── 3. nested folding (argument WHNF) ────────────────────────────────────────

#[test]
fn nested_literal_arithmetic_folds() {
    // Nat.add (Nat.add 1 2) 3 never folded before (args were not WHNF'd).
    let inner = op2("Nat.add", nat(1), nat(2));
    let e = op2("Nat.add", inner, nat(3));
    assert_eq!(whnf(&e), nat(6));
}

#[test]
fn deeply_nested_succ_folds() {
    let e = op1("Nat.succ", op1("Nat.succ", op1("Nat.succ", nat(3))));
    assert_eq!(whnf(&e), nat(6));
}

#[test]
fn nested_mixed_ops_fold() {
    // (10 - 4) * (2 + 3) % 7 = 30 % 7 = 2
    let e = op2(
        "Nat.mod",
        op2(
            "Nat.mul",
            op2("Nat.sub", nat(10), nat(4)),
            op2("Nat.add", nat(2), nat(3)),
        ),
        nat(7),
    );
    assert_eq!(whnf(&e), nat(2));
}

#[test]
fn nested_def_eq_decides() {
    let env = env();
    let mut dec = DefEqChecker::new(&env);
    let lhs = op2("Nat.add", op2("Nat.mul", nat(6), nat(7)), nat(0));
    assert!(dec.is_def_eq(&lhs, &nat(42)));
    assert!(!dec.is_def_eq(&lhs, &nat(43)));
}

// ── 4. exact arity: over-application stays stuck ─────────────────────────────

#[test]
fn over_applied_nat_op_is_stuck_and_keeps_args() {
    // (Nat.add 1 2) 3 — ill-typed over-application. The old code folded the
    // first two args and DROPPED the third. It must now stay stuck with all
    // three arguments intact.
    let e = Expr::App(Node::new(op2("Nat.add", nat(1), nat(2))), Node::new(nat(3)));
    let r = whnf(&e);
    assert_eq!(r, e, "over-applied literal op must stay stuck, args intact");
}

#[test]
fn under_applied_nat_op_is_stuck() {
    let e = op1("Nat.add", nat(1));
    assert_eq!(whnf(&e), e);
}

#[test]
fn non_literal_args_stay_stuck() {
    let x = Expr::Const(Name::str("x"), vec![]);
    let e = op2("Nat.add", x, nat(1));
    assert_eq!(whnf(&e), e);
}

// ── 5. memory guard: stuck, never wrong ──────────────────────────────────────

#[test]
fn pow_huge_exponent_is_stuck_not_one() {
    // Old kernel: exponent truncated `as u32`, so 2 ^ 2^32 reduced to 1.
    let e = op2("Nat.pow", nat(2), nat(1 << 32));
    let r = whnf(&e);
    assert_eq!(r, e, "over-bound pow must stay stuck");
    // In particular it is NOT defeq to 1 — that was the soundness hole.
    let env = env();
    let mut dec = DefEqChecker::new(&env);
    assert!(!dec.is_def_eq(&e, &nat(1)));
}

#[test]
fn shift_left_huge_amount_is_stuck() {
    let e = op2("Nat.shiftLeft", nat(1), big("999999999999999999999"));
    assert_eq!(whnf(&e), e);
    // But 0 <<< huge = 0 (result is small, computed exactly).
    let z = op2("Nat.shiftLeft", nat(0), big("999999999999999999999"));
    assert_eq!(whnf(&z), nat(0));
}

// ── 6. Bool results are genuine constants ────────────────────────────────────

#[test]
fn beq_ble_produce_genuine_bool_constants() {
    assert_eq!(whnf(&op2("Nat.beq", nat(5), nat(5))), bool_const(true));
    assert_eq!(whnf(&op2("Nat.beq", nat(5), nat(6))), bool_const(false));
    assert_eq!(whnf(&op2("Nat.ble", nat(5), nat(6))), bool_const(true));
    assert_eq!(whnf(&op2("Nat.ble", nat(6), nat(5))), bool_const(false));
    assert_eq!(whnf(&op2("Nat.blt", nat(5), nat(6))), bool_const(true));
    // Multi-limb comparison.
    assert_eq!(
        whnf(&op2(
            "Nat.beq",
            big("18446744073709551616"),
            big("18446744073709551616")
        )),
        bool_const(true)
    );
    assert_eq!(
        whnf(&op2("Nat.ble", big("18446744073709551616"), nat(u64::MAX))),
        bool_const(false)
    );
}

#[test]
fn string_ops_reduce() {
    let s = |t: &str| Expr::Lit(Literal::Str(t.to_string()));
    assert_eq!(whnf(&op2("String.append", s("foo"), s("bar"))), s("foobar"));
    assert_eq!(whnf(&op2("String.beq", s("a"), s("a"))), bool_const(true));
    assert_eq!(whnf(&op2("String.beq", s("a"), s("b"))), bool_const(false));
}

// ── def_eq fast paths on literals ────────────────────────────────────────────

#[test]
fn def_eq_literal_fast_paths() {
    let env = env();
    let mut dec = DefEqChecker::new(&env);
    // NatLit fast path via BigNat equality, beyond u64.
    let a = big("340282366920938463463374607431768211456");
    let b = big("340282366920938463463374607431768211456");
    let c = big("340282366920938463463374607431768211457");
    assert!(dec.is_def_eq(&a, &b));
    assert!(!dec.is_def_eq(&a, &c));
    // StrLit fast path.
    let s1 = Expr::Lit(Literal::Str("日本".to_string()));
    let s2 = Expr::Lit(Literal::Str("日本".to_string()));
    let s3 = Expr::Lit(Literal::Str("日本語".to_string()));
    assert!(dec.is_def_eq(&s1, &s2));
    assert!(!dec.is_def_eq(&s1, &s3));
}

// ── big-literal round-trip through the kernel's own serialization ────────────

#[test]
fn big_literal_survives_kernel_serialization() {
    use oxilean_kernel::export::functions::deserialize_module;
    use oxilean_kernel::{serialize_module, ExportedModule};
    let mut module = ExportedModule::new("test".to_string());
    module.declarations.push((
        Name::str("big"),
        oxilean_kernel::Declaration::Definition {
            name: Name::str("big"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val: big("340282366920938463463374607431768211456"),
            hint: oxilean_kernel::ReducibilityHint::Regular(1),
        },
    ));
    let bytes = serialize_module(&module);
    let restored = match deserialize_module(&bytes) {
        Ok(m) => m,
        Err(e) => unreachable!("round-trip must succeed: {e}"),
    };
    assert_eq!(restored.declarations.len(), 1);
    match &restored.declarations[0].1 {
        oxilean_kernel::Declaration::Definition { val, .. } => {
            assert_eq!(val, &big("340282366920938463463374607431768211456"));
        }
        other => unreachable!("expected Definition, got {other:?}"),
    }
}
