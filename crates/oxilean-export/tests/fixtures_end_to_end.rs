//! End-to-end parse of all fourteen lean4export fixtures, with exact per-record
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
    assert_counts_with_opaque(fixture_name, s, e, 0);
}

/// [`assert_counts`] for a fixture that has `decl_opaque` opaque records.
fn assert_counts_with_opaque(fixture_name: &str, s: &ReadStats, e: &Expected, decl_opaque: usize) {
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
    // No fixture uses strVal or mdata records; one has opaque records.
    assert_eq!(
        s.expr_str_lit, 0,
        "{fixture_name}: unexpected strVal records"
    );
    assert_eq!(s.expr_mdata, 0, "{fixture_name}: unexpected mdata records");
    assert_eq!(s.decl_opaque, decl_opaque, "{fixture_name}: opaque records");
}

#[test]
fn nat_add_succ_parses_with_exact_counts() {
    // Format 3.0.0 export (Lean 4.27.0-rc1): the only fixture using the
    // array-wrapped {"def":[{...}]} / {"thm":[{...}]} payloads and the
    // inductiveVals/constructorVals/recursorVals inductive keys.
    let f = parse_fixture("Nat.add_succ");
    assert_eq!(
        f.meta.format_version, "3.0.0",
        "Nat.add_succ.ndjson must be lean4export's own format-3.0.0 example \
         (see tests/fixtures/lean4export/README.md), not a re-export with the \
         pinned toolchain; restore it with \
         `git checkout -- tests/fixtures/lean4export/Nat.add_succ.ndjson`"
    );
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

#[test]
fn syntax_parses_with_exact_counts() {
    // Lean core's `Lean.Syntax` with its transitive dependencies: the nested
    // inductive fixture (`numNested = 2`).
    let f = parse_fixture("Syntax");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_eq!(f.meta.lean_version, "4.32.0-rc1");
    assert_counts(
        "Syntax",
        &f.stats,
        &Expected {
            total_lines: 10040,
            name_str: 1110,
            name_num: 594,
            level_succ: 9,
            level_max: 8,
            level_imax: 0,
            level_param: 7,
            expr_bvar: 22,
            expr_sort: 18,
            expr_const: 454,
            expr_app: 4281,
            expr_lam: 1709,
            expr_forall: 1503,
            expr_let: 11,
            expr_proj: 25,
            expr_nat_lit: 19,
            decl_axiom: 0,
            decl_def: 177,
            decl_thm: 47,
            decl_quot: 0,
            decl_inductive: 45,
        },
    );
    // 177 defs + 47 thms + 45 inductive bundles = 269 (per README).
    assert_eq!(f.decls.len(), 269);
}

#[test]
fn quot_partial_app_parses_with_exact_counts() {
    // The point-free quotient fixture: the full Quot package, the `Quot.sound`
    // axiom, and structure projections.
    let f = parse_fixture("QuotPartialApp");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_eq!(f.meta.lean_version, "4.32.0-rc1");
    assert_counts(
        "QuotPartialApp",
        &f.stats,
        &Expected {
            total_lines: 1930,
            name_str: 228,
            name_num: 65,
            level_succ: 7,
            level_max: 5,
            level_imax: 0,
            level_param: 9,
            expr_bvar: 13,
            expr_sort: 18,
            expr_const: 148,
            expr_app: 702,
            expr_lam: 255,
            expr_forall: 402,
            expr_let: 0,
            expr_proj: 10,
            expr_nat_lit: 1,
            decl_axiom: 1,
            decl_def: 33,
            decl_thm: 16,
            decl_quot: 4,
            decl_inductive: 12,
        },
    );
    // 1 axiom + 33 defs + 16 thms + 4 quots + 12 inductive bundles = 66 (per README).
    assert_eq!(f.decls.len(), 66);
}

#[test]
fn k_guard_collision_parses_with_exact_counts() {
    // The negative fixture: one axiom, the `Eq` and `Nat` bundles, and one
    // theorem that Lean's kernel rejects. It PARSES like any other export —
    // being wrong is the replayer's finding, not the reader's.
    let f = parse_fixture("KGuardCollision");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_eq!(f.meta.lean_version, "4.32.0-rc1");
    assert_counts(
        "KGuardCollision",
        &f.stats,
        &Expected {
            total_lines: 165,
            name_str: 35,
            name_num: 4,
            level_succ: 1,
            level_max: 0,
            level_imax: 0,
            level_param: 2,
            expr_bvar: 7,
            expr_sort: 4,
            expr_const: 10,
            expr_app: 42,
            expr_lam: 28,
            expr_forall: 27,
            expr_let: 0,
            expr_proj: 0,
            expr_nat_lit: 0,
            decl_axiom: 1,
            decl_def: 0,
            decl_thm: 1,
            decl_quot: 0,
            decl_inductive: 2,
        },
    );
    // 1 axiom + 1 thm + 2 inductive bundles = 4 (per README).
    assert_eq!(f.decls.len(), 4);
}

#[test]
fn reduction_chain_parses_with_exact_counts() {
    // The long head-reduction chain fixture (`decide +kernel` over `Nat.rec`).
    let f = parse_fixture("ReductionChain");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_eq!(f.meta.lean_version, "4.32.0-rc1");
    assert_counts(
        "ReductionChain",
        &f.stats,
        &Expected {
            total_lines: 992,
            name_str: 160,
            name_num: 53,
            level_succ: 2,
            level_max: 1,
            level_imax: 0,
            level_param: 5,
            expr_bvar: 8,
            expr_sort: 9,
            expr_const: 62,
            expr_app: 344,
            expr_lam: 153,
            expr_forall: 155,
            expr_let: 0,
            expr_proj: 1,
            expr_nat_lit: 2,
            decl_axiom: 0,
            decl_def: 22,
            decl_thm: 7,
            decl_quot: 0,
            decl_inductive: 7,
        },
    );
    // 22 defs + 7 thms + 7 inductive bundles = 36 (per README).
    assert_eq!(f.decls.len(), 36);
}

#[test]
fn shared_subterms_parses_with_exact_counts() {
    // The let-tower fixture: sixty `let`s, each used twice by the next.
    let f = parse_fixture("SharedSubterms");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_eq!(f.meta.lean_version, "4.32.0-rc1");
    assert_counts(
        "SharedSubterms",
        &f.stats,
        &Expected {
            total_lines: 2240,
            name_str: 322,
            name_num: 129,
            level_succ: 7,
            level_max: 4,
            level_imax: 0,
            level_param: 6,
            expr_bvar: 14,
            expr_sort: 14,
            expr_const: 107,
            expr_app: 812,
            expr_lam: 356,
            expr_forall: 335,
            expr_let: 64,
            expr_proj: 7,
            expr_nat_lit: 2,
            decl_axiom: 0,
            decl_def: 43,
            decl_thm: 7,
            decl_quot: 0,
            decl_inductive: 10,
        },
    );
    // 43 defs + 7 thms + 10 inductive bundles = 60 (per README).
    assert_eq!(f.decls.len(), 60);
}

#[test]
fn congruence_first_parses_with_exact_counts() {
    // Three `rfl` theorems that need congruence before unfolding, and one
    // theorem Lean's kernel rejects. The only fixture with `opaque` records
    // (`CongruenceFirst.k` and `System.Platform.getNumBits`).
    let f = parse_fixture("CongruenceFirst");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_eq!(f.meta.lean_version, "4.32.0-rc1");
    assert_counts_with_opaque(
        "CongruenceFirst",
        &f.stats,
        &Expected {
            total_lines: 7018,
            name_str: 764,
            name_num: 430,
            level_succ: 8,
            level_max: 4,
            level_imax: 0,
            level_param: 7,
            expr_bvar: 22,
            expr_sort: 16,
            expr_const: 297,
            expr_app: 3137,
            expr_lam: 1127,
            expr_forall: 983,
            expr_let: 9,
            expr_proj: 18,
            expr_nat_lit: 10,
            decl_axiom: 0,
            decl_def: 117,
            decl_thm: 39,
            decl_quot: 0,
            decl_inductive: 27,
        },
        2,
    );
    // 117 defs + 39 thms + 2 opaques + 27 inductive bundles = 185 (per README).
    assert_eq!(f.decls.len(), 185);
}

#[test]
fn nested_reduction_parses_with_exact_counts() {
    // Two theorems by kernel evaluation of a structural recursion (four
    // hundred steps and forty), and one theorem Lean's kernel rejects.
    let f = parse_fixture("NestedReduction");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_eq!(f.meta.lean_version, "4.32.0-rc1");
    assert_counts(
        "NestedReduction",
        &f.stats,
        &Expected {
            total_lines: 2237,
            name_str: 279,
            name_num: 141,
            level_succ: 7,
            level_max: 4,
            level_imax: 0,
            level_param: 6,
            expr_bvar: 14,
            expr_sort: 14,
            expr_const: 110,
            expr_app: 862,
            expr_lam: 375,
            expr_forall: 344,
            expr_let: 3,
            expr_proj: 7,
            expr_nat_lit: 5,
            decl_axiom: 0,
            decl_def: 45,
            decl_thm: 10,
            decl_quot: 0,
            decl_inductive: 10,
        },
    );
    // 45 defs + 10 thms + 10 inductive bundles = 65 (per README).
    assert_eq!(f.decls.len(), 65);
}

#[test]
fn deep_term_parses_with_exact_counts() {
    // A term three hundred applications deep, an equation between two such
    // terms, and one equation Lean's kernel rejects. 633 of the file's 720
    // records are applications.
    let f = parse_fixture("DeepTerm");
    assert_eq!(f.meta.format_version, "3.1.0");
    assert_eq!(f.meta.lean_version, "4.32.0-rc1");
    assert_counts(
        "DeepTerm",
        &f.stats,
        &Expected {
            total_lines: 720,
            name_str: 26,
            name_num: 1,
            level_succ: 1,
            level_max: 0,
            level_imax: 0,
            level_param: 2,
            expr_bvar: 5,
            expr_sort: 4,
            expr_const: 9,
            expr_app: 633,
            expr_lam: 11,
            expr_forall: 21,
            expr_let: 0,
            expr_proj: 0,
            expr_nat_lit: 0,
            decl_axiom: 0,
            decl_def: 2,
            decl_thm: 2,
            decl_quot: 0,
            decl_inductive: 2,
        },
    );
    // 2 defs + 2 thms + 2 inductive bundles = 6 (per README).
    assert_eq!(f.decls.len(), 6);
    assert_eq!(f.stats.decl_oversized, 0, "no limit on this thread");
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
