//! End-to-end parse of all six lean4export fixtures, with exact per-record
//! counts, plus golden shape checks against `Nat.add_succ`.
//!
//! The counts below are ground truth, extracted from the fixture files
//! themselves (one JSON record per line, classified by discriminator key).
//! If a fixture changes, these numbers must be re-derived, not fudged.

use oxilean_export::{read_str, ExportDecl, ExportFile, ReadStats};
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

fn fixture(name: &str) -> String {
    let path = format!(
        "{}/../../tests/fixtures/lean4export/{name}.ndjson",
        env!("CARGO_MANIFEST_DIR")
    );
    match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => panic!("cannot read fixture {path}: {e}"),
    }
}

fn parse_fixture(name: &str) -> ExportFile {
    match read_str(&fixture(name)) {
        Ok(f) => f,
        Err(e) => panic!("fixture {name} must parse cleanly: {e}"),
    }
}

/// Exact expected counts for one fixture.
struct Expected {
    total_lines: usize,
    name_str: usize,
    name_num: usize,
    level_succ: usize,
    level_max: usize,
    level_imax: usize,
    level_param: usize,
    expr_bvar: usize,
    expr_sort: usize,
    expr_const: usize,
    expr_app: usize,
    expr_lam: usize,
    expr_forall: usize,
    expr_let: usize,
    expr_proj: usize,
    expr_nat_lit: usize,
    decl_axiom: usize,
    decl_def: usize,
    decl_thm: usize,
    decl_quot: usize,
    decl_inductive: usize,
}

fn assert_counts(fixture_name: &str, s: &ReadStats, e: &Expected) {
    let pairs: [(&str, usize, usize); 21] = [
        ("total_lines", s.total_lines, e.total_lines),
        ("name_str", s.name_str, e.name_str),
        ("name_num", s.name_num, e.name_num),
        ("level_succ", s.level_succ, e.level_succ),
        ("level_max", s.level_max, e.level_max),
        ("level_imax", s.level_imax, e.level_imax),
        ("level_param", s.level_param, e.level_param),
        ("expr_bvar", s.expr_bvar, e.expr_bvar),
        ("expr_sort", s.expr_sort, e.expr_sort),
        ("expr_const", s.expr_const, e.expr_const),
        ("expr_app", s.expr_app, e.expr_app),
        ("expr_lam", s.expr_lam, e.expr_lam),
        ("expr_forall", s.expr_forall, e.expr_forall),
        ("expr_let", s.expr_let, e.expr_let),
        ("expr_proj", s.expr_proj, e.expr_proj),
        ("expr_nat_lit", s.expr_nat_lit, e.expr_nat_lit),
        ("decl_axiom", s.decl_axiom, e.decl_axiom),
        ("decl_def", s.decl_def, e.decl_def),
        ("decl_thm", s.decl_thm, e.decl_thm),
        ("decl_quot", s.decl_quot, e.decl_quot),
        ("decl_inductive", s.decl_inductive, e.decl_inductive),
    ];
    for (field, got, want) in pairs {
        assert_eq!(
            got, want,
            "{fixture_name}: {field} mismatch (got {got}, want {want})"
        );
    }
    // No fixture uses strVal, mdata, or opaque records.
    assert_eq!(
        s.expr_str_lit, 0,
        "{fixture_name}: unexpected strVal records"
    );
    assert_eq!(s.expr_mdata, 0, "{fixture_name}: unexpected mdata records");
    assert_eq!(
        s.decl_opaque, 0,
        "{fixture_name}: unexpected opaque records"
    );
}

#[test]
fn nat_add_succ_parses_with_exact_counts() {
    // Format 3.0.0 export (Lean 4.27.0-rc1): the only fixture using the
    // array-wrapped {"def":[{...}]} / {"thm":[{...}]} payloads and the
    // inductiveVals/constructorVals/recursorVals inductive keys.
    let f = parse_fixture("Nat.add_succ");
    assert_eq!(f.meta.format_version, "3.0.0");
    assert_eq!(f.meta.format_major(), Some(3));
    assert_eq!(f.meta.exporter_name, "lean4export");
    assert_eq!(f.meta.lean_version, "4.27.0-rc1");
    assert_counts(
        "Nat.add_succ",
        &f.stats,
        &Expected {
            total_lines: 572,
            name_str: 88,
            name_num: 15,
            level_succ: 7,
            level_max: 4,
            level_imax: 0,
            level_param: 4,
            expr_bvar: 6,
            expr_sort: 12,
            expr_const: 41,
            expr_app: 172,
            expr_lam: 84,
            expr_forall: 115,
            expr_let: 0,
            expr_proj: 4,
            expr_nat_lit: 0,
            decl_axiom: 0,
            decl_def: 12,
            decl_thm: 1,
            decl_quot: 0,
            decl_inductive: 6,
        },
    );
    // 12 defs + 1 thm + 6 inductive bundles = 19 surfaced declarations.
    assert_eq!(f.decls.len(), 19);
}

#[test]
fn simple_add_parses_with_exact_counts() {
    let f = parse_fixture("simple_add");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_eq!(f.meta.lean_version, "4.32.0-rc1");
    assert_counts(
        "simple_add",
        &f.stats,
        &Expected {
            total_lines: 643,
            name_str: 96,
            name_num: 17,
            level_succ: 7,
            level_max: 4,
            level_imax: 0,
            level_param: 4,
            expr_bvar: 6,
            expr_sort: 12,
            expr_const: 48,
            expr_app: 195,
            expr_lam: 91,
            expr_forall: 131,
            expr_let: 0,
            expr_proj: 5,
            expr_nat_lit: 3,
            decl_axiom: 0,
            decl_def: 15,
            decl_thm: 1,
            decl_quot: 0,
            decl_inductive: 7,
        },
    );
    // 15 defs + 1 thm + 7 inductive bundles = 23 declarations (per README).
    assert_eq!(f.decls.len(), 23);
}

#[test]
fn point_swap_swap_parses_with_exact_counts() {
    let f = parse_fixture("point_swap_swap");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_counts(
        "point_swap_swap",
        &f.stats,
        &Expected {
            total_lines: 372,
            name_str: 69,
            name_num: 12,
            level_succ: 1,
            level_max: 0,
            level_imax: 0,
            level_param: 2,
            expr_bvar: 6,
            expr_sort: 4,
            expr_const: 28,
            expr_app: 115,
            expr_lam: 45,
            expr_forall: 71,
            expr_let: 0,
            expr_proj: 2,
            expr_nat_lit: 0,
            decl_axiom: 1,
            decl_def: 4,
            decl_thm: 6,
            decl_quot: 0,
            decl_inductive: 5,
        },
    );
    // 1 axiom + 4 defs + 6 thms + 5 inductive bundles = 16 (per README).
    assert_eq!(f.decls.len(), 16);
}

#[test]
fn parity_is_even_parses_with_exact_counts() {
    // Exercises the quot primitive: the full Quot package (type/ctor/lift/ind)
    // plus letE records.
    let f = parse_fixture("Parity.isEven");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_counts(
        "Parity.isEven",
        &f.stats,
        &Expected {
            total_lines: 7015,
            name_str: 759,
            name_num: 430,
            level_succ: 7,
            level_max: 5,
            level_imax: 0,
            level_param: 6,
            expr_bvar: 22,
            expr_sort: 15,
            expr_const: 294,
            expr_app: 3133,
            expr_lam: 1111,
            expr_forall: 1023,
            expr_let: 9,
            expr_proj: 16,
            expr_nat_lit: 3,
            decl_axiom: 0,
            decl_def: 112,
            decl_thm: 40,
            decl_quot: 4,
            decl_inductive: 25,
        },
    );
    // 112 defs + 40 thms + 4 quots + 25 inductive bundles = 181 (per README).
    assert_eq!(f.decls.len(), 181);
}

#[test]
fn tree_forest_parses_with_exact_counts() {
    // One inductive record bundling the Tree/Forest mutual group.
    let f = parse_fixture("Tree_Forest");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_counts(
        "Tree_Forest",
        &f.stats,
        &Expected {
            total_lines: 166,
            name_str: 25,
            name_num: 2,
            level_succ: 1,
            level_max: 0,
            level_imax: 0,
            level_param: 1,
            expr_bvar: 10,
            expr_sort: 2,
            expr_const: 8,
            expr_app: 49,
            expr_lam: 32,
            expr_forall: 34,
            expr_let: 0,
            expr_proj: 0,
            expr_nat_lit: 0,
            decl_axiom: 0,
            decl_def: 0,
            decl_thm: 0,
            decl_quot: 0,
            decl_inductive: 1,
        },
    );
    assert_eq!(f.decls.len(), 1);

    // The bundle carries both types of the mutual group, each listing the
    // other in `all`.
    let ExportDecl::Inductive(bundle) = &f.decls[0] else {
        panic!("Tree_Forest decl 0 must be an inductive bundle");
    };
    assert_eq!(bundle.types.len(), 2);
    assert_eq!(bundle.ctors.len(), 4);
    assert_eq!(bundle.recs.len(), 2);
    let names: Vec<String> = bundle
        .types
        .iter()
        .map(|t| t.common.name.to_string())
        .collect();
    assert!(
        names[0].contains("Tree"),
        "first type should be Tree: {names:?}"
    );
    assert!(
        names[1].contains("Forest"),
        "second type should be Forest: {names:?}"
    );
    for t in &bundle.types {
        assert_eq!(t.all.len(), 2, "mutual group `all` must list both types");
    }
}

#[test]
fn tree_forest_full_parses_with_exact_counts() {
    let f = parse_fixture("Tree_Forest_full");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_counts(
        "Tree_Forest_full",
        &f.stats,
        &Expected {
            total_lines: 1265,
            name_str: 138,
            name_num: 35,
            level_succ: 7,
            level_max: 4,
            level_imax: 0,
            level_param: 4,
            expr_bvar: 10,
            expr_sort: 11,
            expr_const: 74,
            expr_app: 471,
            expr_lam: 236,
            expr_forall: 225,
            expr_let: 0,
            expr_proj: 10,
            expr_nat_lit: 2,
            decl_axiom: 0,
            decl_def: 30,
            decl_thm: 0,
            decl_quot: 0,
            decl_inductive: 7,
        },
    );
    assert_eq!(f.decls.len(), 37);
}

// --- Golden shape checks: Nat.add_succ's thm record -----------------------
//
// The theorem is:
//     theorem Nat.add_succ (n m : Nat) : n + Nat.succ m = Nat.succ (n + m)
// Its exported type is
//     forallE(n : Nat). forallE(m : Nat).
//         Eq.{1} Nat (HAdd.hAdd ... n (Nat.succ m)) (Nat.succ (HAdd.hAdd ... n m))
// and its value is `fun (n m : Nat) => rfl.{1} Nat <lhs>` (after the exporter's
// dependency chain).

/// Unfold an application spine: `f a b c` → (f, [a, b, c]).
fn spine(e: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut head = e;
    let mut args = Vec::new();
    while let Expr::App(f, a) = head {
        args.push(a.as_ref());
        head = f.as_ref();
    }
    args.reverse();
    (head, args)
}

#[test]
fn nat_add_succ_thm_golden_shape() {
    let f = parse_fixture("Nat.add_succ");
    let thm = f
        .decls
        .iter()
        .find_map(|d| match d {
            ExportDecl::Theorem(t) => Some(t),
            _ => None,
        })
        .unwrap_or_else(|| panic!("fixture must contain exactly one thm"));

    // Name, universe list, mutual group.
    let expected_name = Name::str("Nat").append_str("add_succ");
    assert_eq!(thm.common.name, expected_name);
    assert_eq!(thm.common.name.to_string(), "Nat.add_succ");
    assert!(
        thm.common.level_params.is_empty(),
        "Nat.add_succ has no universe params"
    );
    assert_eq!(thm.all, vec![expected_name.clone()]);

    // Type: forallE(n : Nat). forallE(m : Nat). <Eq application>
    let nat = Expr::Const(Name::str("Nat"), vec![]);
    let Expr::Pi(bi1, n1, dom1, body1) = &thm.common.ty else {
        panic!("thm type must be a Pi, got {:?}", thm.common.ty);
    };
    assert_eq!(*bi1, BinderInfo::Default);
    assert_eq!(*n1, Name::str("n"));
    assert_eq!(**dom1, nat);

    let Expr::Pi(bi2, n2, dom2, body2) = body1.as_ref() else {
        panic!("thm type body must be a Pi");
    };
    assert_eq!(*bi2, BinderInfo::Default);
    assert_eq!(*n2, Name::str("m"));
    assert_eq!(**dom2, nat);

    // Body: Eq.{1} Nat lhs rhs — a 3-argument application of Eq at level
    // succ(zero), first argument Nat.
    let (head, args) = spine(body2);
    assert_eq!(
        *head,
        Expr::Const(Name::str("Eq"), vec![Level::succ(Level::zero())]),
        "Eq head with universe list [succ zero]"
    );
    assert_eq!(args.len(), 3, "Eq applied to type + lhs + rhs");
    assert_eq!(*args[0], nat);

    // lhs is `HAdd.hAdd ... n (Nat.succ m)`: 6-argument spine headed by
    // HAdd.hAdd, last argument Nat.succ applied to bvar(0) (= m).
    let (lhs_head, lhs_args) = spine(args[1]);
    assert_eq!(
        lhs_head.as_const_name().map(ToString::to_string).as_deref(),
        Some("HAdd.hAdd")
    );
    assert_eq!(lhs_args.len(), 6);
    let (succ_head, succ_args) = spine(lhs_args[5]);
    assert_eq!(
        *succ_head,
        Expr::Const(Name::str("Nat").append_str("succ"), vec![])
    );
    assert_eq!(succ_args, vec![&Expr::BVar(0)]);
    assert_eq!(
        *lhs_args[4],
        Expr::BVar(1),
        "5th HAdd argument is n (bvar 1)"
    );

    // rhs is `Nat.succ (HAdd.hAdd ... n m)`.
    let (rhs_head, rhs_args) = spine(args[2]);
    assert_eq!(
        *rhs_head,
        Expr::Const(Name::str("Nat").append_str("succ"), vec![])
    );
    assert_eq!(rhs_args.len(), 1);
    let (inner_head, inner_args) = spine(rhs_args[0]);
    assert_eq!(
        inner_head
            .as_const_name()
            .map(ToString::to_string)
            .as_deref(),
        Some("HAdd.hAdd")
    );
    assert_eq!(inner_args.len(), 6);
    assert_eq!(*inner_args[4], Expr::BVar(1));
    assert_eq!(*inner_args[5], Expr::BVar(0));

    // Value: fun (n m : Nat) => rfl.{1} Nat <term> — outer binders n then m,
    // body headed by rfl with one universe argument and two applied args.
    let Expr::Lam(_, vn1, vdom1, vbody1) = &thm.value else {
        panic!("thm value must be a Lam");
    };
    assert_eq!(*vn1, Name::str("n"));
    assert_eq!(**vdom1, nat);
    let Expr::Lam(_, vn2, vdom2, vbody2) = vbody1.as_ref() else {
        panic!("thm value inner must be a Lam");
    };
    assert_eq!(*vn2, Name::str("m"));
    assert_eq!(**vdom2, nat);
    let (val_head, val_args) = spine(vbody2);
    assert_eq!(
        *val_head,
        Expr::Const(Name::str("rfl"), vec![Level::succ(Level::zero())])
    );
    assert_eq!(
        val_args.len(),
        2,
        "rfl applied to the type and the lhs term"
    );
    assert_eq!(*val_args[0], nat);
}
