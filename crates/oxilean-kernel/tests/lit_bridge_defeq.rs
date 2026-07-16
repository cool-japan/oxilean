//! Definitional-equality literal ↔ constructor-form bridge (Wave-3 C6).
//!
//! Pins Lean's `natLitExt?` / `strLitExt?` on the real kernel def-eq path
//! (`DefEqChecker::is_def_eq`):
//!
//! * `NatLit 0`  ≡ `Nat.zero`,  `NatLit (n+1)` ≡ `Nat.succ e`  (both orientations);
//! * `StrLit s`  ≡ `String.mk (List Char)`  (both orientations);
//! * mixed *stuck* cases (`NatLit 5` vs `Nat.succ (FVar)`) stay **false**,
//!   never wrong;
//! * `Nat.rec` on a term that only WHNFs to `Nat.succ (NatLit k)` still
//!   reduces;
//! * the succ-peel is **magnitude-bounded**: a huge literal against a shallow
//!   succ-tower fails fast and is never eagerly materialised.

use oxilean_kernel::bignat::BigNat;
use oxilean_kernel::Node;
use oxilean_kernel::{
    init_builtin_env, DefEqChecker, Environment, Expr, FVarId, Literal, Name, Reducer,
};

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

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn app(f: Expr, arg: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(arg))
}

fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}

fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}

/// `Nat.succ e`.
fn succ(e: Expr) -> Expr {
    app(cnst("Nat.succ"), e)
}

/// `Nat.zero`.
fn zero() -> Expr {
    cnst("Nat.zero")
}

fn defeq(a: &Expr, b: &Expr) -> bool {
    let env = env();
    let mut dec = DefEqChecker::new(&env);
    dec.is_def_eq(a, b)
}

fn whnf(e: &Expr) -> Expr {
    let env = env();
    Reducer::new().whnf_env(e, &env)
}

// ── 1. NatLit 0 ≡ Nat.zero, both orientations ────────────────────────────────

#[test]
fn nat_lit_zero_eq_const_zero_both_orientations() {
    assert!(defeq(&nat(0), &zero()), "NatLit 0 =?= Nat.zero");
    assert!(defeq(&zero(), &nat(0)), "Nat.zero =?= NatLit 0");
}

// ── 2. NatLit 5 ≡ succ^5 zero, both orientations ─────────────────────────────

/// `succ (succ (succ (succ (succ Nat.zero))))`. Note: this WHNFs all the way
/// to `NatLit 5` because each `Nat.succ` applied to a literal (or `Nat.zero`)
/// folds — so this exercises the reduction path plus the def-eq bridge.
fn succ_tower_over_zero(n: usize) -> Expr {
    let mut e = zero();
    for _ in 0..n {
        e = succ(e);
    }
    e
}

#[test]
fn nat_lit_five_eq_succ_tower_both_orientations() {
    let tower = succ_tower_over_zero(5);
    // Sanity: the tower reduces to the literal 5 on its own.
    assert_eq!(whnf(&tower), nat(5));
    assert!(defeq(&nat(5), &tower), "NatLit 5 =?= succ^5 zero");
    assert!(defeq(&tower, &nat(5)), "succ^5 zero =?= NatLit 5");
    // And it is NOT defeq to a different literal.
    assert!(!defeq(&nat(4), &tower));
    assert!(!defeq(&nat(6), &tower));
}

// ── 3. NatLit 5 ≡ succ (NatLit 4) ────────────────────────────────────────────

#[test]
fn nat_lit_five_eq_succ_of_lit_four() {
    // `Nat.succ (NatLit 4)` WHNFs to `NatLit 5` (arg-folding), then the
    // both-literal fast path decides — but assert the def-eq verdict directly.
    let e = succ(nat(4));
    assert_eq!(whnf(&e), nat(5));
    assert!(defeq(&nat(5), &e), "NatLit 5 =?= succ (NatLit 4)");
    assert!(defeq(&e, &nat(5)), "succ (NatLit 4) =?= NatLit 5");
    assert!(!defeq(&nat(6), &e));
}

// ── 4. Mixed stuck cases stay false, never wrong ─────────────────────────────

fn fvar(n: u64) -> Expr {
    Expr::FVar(FVarId::new(n))
}

#[test]
fn nat_lit_vs_succ_fvar_is_stuck_false() {
    // `succ (FVar)` cannot reduce (FVar is not a literal), so after WHNF the
    // term is `Nat.succ (FVar)`. The bridge peels ONE layer to
    // `NatLit 4 =?= FVar`, which is undecidable → false. Crucially NOT true.
    let e = succ(fvar(0));
    assert!(!defeq(&nat(5), &e), "NatLit 5 vs succ (FVar) must be false");
    assert!(!defeq(&e, &nat(5)), "succ (FVar) vs NatLit 5 must be false");
    // Zero vs a succ-headed term is a definite mismatch.
    assert!(!defeq(&nat(0), &e));
    assert!(!defeq(&e, &nat(0)));
}

#[test]
fn nat_lit_zero_vs_succ_is_false() {
    // 0 is never a successor.
    let e = succ(fvar(0));
    assert!(!defeq(&nat(0), &e));
    // And a positive literal is never `Nat.zero`.
    assert!(!defeq(&nat(3), &zero()));
    assert!(!defeq(&zero(), &nat(3)));
}

#[test]
fn nat_lit_vs_unrelated_const_is_false_not_stuck_wrong() {
    // A bare unrelated constant is neither zero nor succ-headed: the bridge
    // returns "not applicable" and the term is not defeq to any literal.
    let x = cnst("SomeOpaque");
    assert!(!defeq(&nat(0), &x));
    assert!(!defeq(&nat(7), &x));
    assert!(!defeq(&x, &nat(0)));
}

// ── 5. Nat.rec on a term that WHNFs to succ(NatLit k) ────────────────────────

#[test]
fn nat_rec_on_reducible_succ_of_lit_reduces() {
    let env = env();
    let u = oxilean_kernel::Level::succ(oxilean_kernel::Level::zero());
    let nat_ty = cnst("Nat");
    // motive := fun _ => Nat
    let motive = Expr::Lam(
        oxilean_kernel::BinderInfo::Default,
        Name::str("_"),
        Node::new(nat_ty.clone()),
        Node::new(nat_ty.clone()),
    );
    // succ minor := fun n ih => Nat.succ ih  → this counts, giving `id` on Nat.
    let minor_succ = Expr::Lam(
        oxilean_kernel::BinderInfo::Default,
        Name::str("n"),
        Node::new(nat_ty.clone()),
        Node::new(Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            Name::str("ih"),
            Node::new(nat_ty.clone()),
            Node::new(succ(Expr::BVar(0))),
        )),
    );
    // major := Nat.succ (NatLit 4) — reducible to NatLit 5.
    let major = succ(nat(4));
    let rec_app = {
        let mut e = Expr::Const(Name::str("Nat.rec"), vec![u]);
        for a in [motive, nat(100), minor_succ, major] {
            e = app(e, a);
        }
        e
    };
    // rec_add 100 5 = 105.
    let mut reducer = Reducer::new();
    assert_eq!(reducer.whnf_env(&rec_app, &env), nat(105));
}

// ── 6. String bridge, both orientations ──────────────────────────────────────

/// Build `String.mk (List.cons Char (Char.ofNat c₀) (… List.nil Char))` from
/// the characters of `s`, mirroring `reduce::iota::str_lit_to_ctor`.
fn string_mk_of(s: &str) -> Expr {
    let char_ty = cnst("Char");
    let mut list = app(
        Expr::Const(Name::str("List.nil"), vec![oxilean_kernel::Level::zero()]),
        char_ty.clone(),
    );
    for c in s.chars().rev() {
        let code = Expr::Lit(Literal::nat(c as u32 as u64));
        let ch = app(cnst("Char.ofNat"), code);
        list = app3(
            Expr::Const(Name::str("List.cons"), vec![oxilean_kernel::Level::zero()]),
            char_ty.clone(),
            ch,
            list,
        );
    }
    app(cnst("String.mk"), list)
}

fn str_lit(s: &str) -> Expr {
    Expr::Lit(Literal::Str(s.to_string()))
}

#[test]
fn string_lit_eq_string_mk_both_orientations() {
    for s in ["", "a", "hello", "日本語"] {
        let mk = string_mk_of(s);
        assert!(
            defeq(&str_lit(s), &mk),
            "StrLit {s:?} =?= String.mk (chars)"
        );
        assert!(
            defeq(&mk, &str_lit(s)),
            "String.mk (chars) =?= StrLit {s:?}"
        );
    }
}

#[test]
fn string_lit_neq_wrong_string_mk() {
    let mk_hello = string_mk_of("hello");
    assert!(!defeq(&str_lit("world"), &mk_hello), "different content");
    assert!(!defeq(&str_lit("hell"), &mk_hello), "prefix, shorter");
    assert!(!defeq(&str_lit("helloo"), &mk_hello), "longer");
    // empty vs non-empty
    assert!(!defeq(&str_lit(""), &mk_hello));
    assert!(!defeq(&str_lit("hello"), &string_mk_of("")));
}

// ── 7. big-literal magnitude guard ───────────────────────────────────────────

#[test]
fn big_literal_vs_shallow_succ_tower_fails_fast() {
    // NatLit 10^9 vs `succ (succ (FVar))`: the bridge peels only TWO succ
    // layers (decrementing the literal each time) then hits the non-succ FVar
    // and stops — it MUST NOT materialise 10^9 succ nodes. A wall-clock guard
    // makes an accidental eager expansion (which would take seconds/OOM)
    // observable as a hang/failure.
    let billion = big("1000000000");
    let e = succ(succ(fvar(0)));
    let start = std::time::Instant::now();
    assert!(!defeq(&billion, &e), "10^9 vs shallow succ tower is false");
    assert!(!defeq(&e, &billion));
    let elapsed = start.elapsed();
    assert!(
        elapsed < std::time::Duration::from_secs(2),
        "succ-peel must be bounded by the tower depth (2), not the literal \
         magnitude (10^9); took {elapsed:?}"
    );
}

#[test]
fn big_literal_vs_matching_succ_of_big_lit_is_true() {
    // NatLit (10^9) vs `Nat.succ (NatLit (10^9 - 1))`: one peel, then the
    // both-literal fast path decides — true, and cheap.
    let billion = big("1000000000");
    let billion_minus_one = big("999999999");
    let e = succ(billion_minus_one);
    // `succ (NatLit (10^9-1))` WHNFs to `NatLit 10^9`.
    assert_eq!(whnf(&e), billion);
    assert!(defeq(&billion, &e));
    assert!(defeq(&e, &billion));
}

// ── v4.32 String model: the bridge goes through `String.ofList` ─────────────
//
// Since the UTF-8 `String` refactor (Lean ≥ 4.28; pinned corpus v4.32.0-rc1)
// the constructor is `String.ofByteArray (bytes) (validity)`, and the
// kernel's `string_lit_to_constructor` builds the *function* application
// `String.ofList (l : List Char)`; `try_string_lit_expansion` fires exactly
// when the other side is an `String.ofList`-headed application. These pins
// use an env where `String.ofList` is opaque (an axiom), so the bridge must
// decide via congruence on the char lists — exactly what the corpus needs
// for e.g. `String.push_induction` (`motive (String.ofList List.nil)` vs
// `motive ""`).

/// Hierarchical name `a.b.c`.
fn hname(dotted: &str) -> Name {
    Name::from_str(dotted)
}

fn hcnst(dotted: &str) -> Expr {
    Expr::Const(hname(dotted), vec![])
}

/// An env in the hierarchical (lean4export) naming style with the v4.32
/// String model surface: `String`, `Char`, `List`, `List.nil`, `List.cons`,
/// `Char.ofNat`, and an OPAQUE `String.ofList : List Char → String`.
fn v432_env() -> Environment {
    use oxilean_kernel::{check_constant_info, AxiomVal, ConstantInfo, ConstantVal, Level};
    let mut env = Environment::new();
    let sort1 = Expr::Sort(Level::succ(Level::zero()));
    let mut ax = |name: Name, ty: Expr| {
        let ci = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: name.clone(),
                level_params: vec![],
                ty,
            },
            is_unsafe: false,
        });
        match check_constant_info(&mut env, ci) {
            Ok(()) => {}
            Err(e) => unreachable!("axiom {name}: {e:?}"),
        }
    };
    let pi = |ty: Expr, body: Expr| {
        Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("_"),
            Node::new(ty),
            Node::new(body),
        )
    };
    ax(hname("String"), sort1.clone());
    ax(hname("Char"), sort1.clone());
    ax(hname("Nat"), sort1.clone());
    // List : Sort 1 -> Sort 1 (monomorphised at level zero for the test).
    ax(hname("List"), pi(sort1.clone(), sort1.clone()));
    // List.nil.{0} : (A : Sort 1) -> List A   (levels ignored by the axiom).
    let list_char = app(
        Expr::Const(hname("List"), vec![]),
        Expr::Const(hname("Char"), vec![]),
    );
    ax(hname("List.nil"), pi(sort1.clone(), list_char.clone()));
    ax(
        hname("List.cons"),
        pi(
            sort1.clone(),
            pi(hcnst("Char"), pi(list_char.clone(), list_char.clone())),
        ),
    );
    ax(hname("Char.ofNat"), pi(hcnst("Nat"), hcnst("Char")));
    ax(hname("String.ofList"), pi(list_char, hcnst("String")));
    env
}

fn v432_defeq(a: &Expr, b: &Expr) -> bool {
    let env = v432_env();
    let mut dec = DefEqChecker::new(&env);
    dec.is_def_eq(a, b)
}

/// `List.nil.{0} Char` in the hierarchical style, as `str_lit_expansion`
/// builds it.
fn nil_char() -> Expr {
    app(
        Expr::Const(hname("List.nil"), vec![oxilean_kernel::Level::zero()]),
        hcnst("Char"),
    )
}

fn cons_char(head: Expr, tail: Expr) -> Expr {
    app3(
        Expr::Const(hname("List.cons"), vec![oxilean_kernel::Level::zero()]),
        hcnst("Char"),
        head,
        tail,
    )
}

fn char_of(c: char) -> Expr {
    app(hcnst("Char.ofNat"), nat(c as u32 as u64))
}

#[test]
fn empty_str_lit_eq_of_list_nil_both_orientations() {
    let of_list_nil = app(hcnst("String.ofList"), nil_char());
    assert!(
        v432_defeq(&str_lit(""), &of_list_nil),
        "\"\" =?= String.ofList (List.nil Char)"
    );
    assert!(
        v432_defeq(&of_list_nil, &str_lit("")),
        "String.ofList (List.nil Char) =?= \"\""
    );
}

#[test]
fn str_lit_eq_of_list_cons_chars() {
    let of_list_ab = app(
        hcnst("String.ofList"),
        cons_char(char_of('a'), cons_char(char_of('b'), nil_char())),
    );
    assert!(
        v432_defeq(&str_lit("ab"), &of_list_ab),
        "\"ab\" =?= String.ofList ['a','b']"
    );
}

#[test]
fn str_lit_of_list_mismatches_stay_false() {
    let of_list_nil = app(hcnst("String.ofList"), nil_char());
    let of_list_a = app(hcnst("String.ofList"), cons_char(char_of('a'), nil_char()));
    assert!(
        !v432_defeq(&str_lit("a"), &of_list_nil),
        "\"a\" must NOT equal String.ofList []"
    );
    assert!(
        !v432_defeq(&str_lit(""), &of_list_a),
        "\"\" must NOT equal String.ofList ['a']"
    );
    assert!(
        !v432_defeq(&str_lit("b"), &of_list_a),
        "\"b\" must NOT equal String.ofList ['a']"
    );
}

// ── v4.32 `reduce_proj_core`: Proj on a string literal expands first ─────────
//
// In the OLD-model builtin env (one-field `String.mk : List Char → String`),
// `Proj String 0 (StrLit s)` must reduce to the char list — pinning that the
// projection path expands string literals at all (Lean's `reduce_proj_core`
// does this before projecting, whatever the model).

#[test]
fn proj_on_string_literal_projects_the_char_list() {
    let e = Expr::Proj(Name::str("String"), 0, Node::new(str_lit("a")));
    let reduced = whnf(&e);
    // Old-model expansion: String.mk (List.cons Char (Char.ofNat 97) (List.nil Char))
    // → proj 0 → the list.
    let expected = app3(
        Expr::Const(Name::str("List.cons"), vec![oxilean_kernel::Level::zero()]),
        cnst("Char"),
        app(cnst("Char.ofNat"), nat(97)),
        app(
            Expr::Const(Name::str("List.nil"), vec![oxilean_kernel::Level::zero()]),
            cnst("Char"),
        ),
    );
    assert_eq!(
        reduced, expected,
        "Proj String 0 \"a\" must expand-and-project, got {reduced:?}"
    );
}
