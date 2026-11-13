//! Conditions Lean 4's kernel checks when a declaration is added, which a
//! reading of the whole list (2026-11-12) found missing or weaker here.
//! Every expectation below is what Lean 4 v4.32.0-rc1's own kernel answered
//! for the same declaration, handed to it from an empty environment
//! (`Environment.addDeclCore`), unless the test says otherwise.
//!
//! * **A recursor is the recursor of the type it is named after.** A
//!   supplied recursor is compared with the derived recursor of its own
//!   name and no other; in a mutual block `B.rec` is not a renamed copy of
//!   `A.rec`, nor the two exchanged. On the nested path the names are
//!   Lean's: `T.rec` for a type of the declaration, `<first type>.rec_<k>`
//!   for the `k`-th auxiliary type.
//! * **A recursor's rules are the constructors', in their order.**
//! * **A theorem states a proposition**, at both entry points
//!   (`check_declaration`, `check_constant_info`).
//! * **The shape of an inductive declaration.** An index of a
//!   constructor's result, or of a recursive field, does not mention a
//!   type of the family; the parameters of a type and of a constructor,
//!   and a constructor's fields and result, are read off the type as it is
//!   written, while a type's indices and sort may be behind a definition.
//! * **`Eq` has the shape the quotient primitives assume** before they are
//!   installed.
//! * **The kernel's own literal primitives** (`Nat.pred`, `Nat.blt`,
//!   `Nat.log2`, `String.length`, `String.append`, `String.beq`) are
//!   reduction rules of the builtin environment only. In an environment
//!   that declares a constant of one of these names itself, the constant
//!   reduces by its declaration; the operations Lean's kernel computes by
//!   name (`Nat.add`, ...) are computed by name here too.

use oxilean_kernel::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, DefinitionSafety, DefinitionVal, RecursorVal, TheoremVal,
};
use oxilean_kernel::expr_util::replace_expr;
use oxilean_kernel::Node;
use oxilean_kernel::{
    add_inductive_family, check_and_derive_family, check_and_derive_family_nested,
    check_constant_info, check_constant_infos_atomic, check_declaration, init_builtin_env,
    restore_nested, verify_nested_bundle, BinderInfo, Declaration, Environment, Expr,
    InductiveSpec, KernelError, Level, Literal, Name, ReducibilityHint, TypeChecker,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}

/// A hierarchical name written with dots.
fn nm(dotted: &str) -> Name {
    Name::from_str(dotted)
}

fn cnst(name: &str) -> Expr {
    Expr::Const(nm(name), vec![])
}

fn cnst_at(name: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(nm(name), levels)
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

fn apps(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, app)
}

fn pi(bi: BinderInfo, x: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(x), Node::new(dom), Node::new(body))
}

/// `a → b` for a closed `b`.
fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "x", a, b)
}

fn lam(x: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(x),
        Node::new(dom),
        Node::new(body),
    )
}

fn nat_lit(n: u64) -> Expr {
    Expr::Lit(Literal::nat(n))
}

fn u() -> Level {
    Level::param(Name::str("u"))
}

fn axiom(name: &str, lparams: &[&str], ty: Expr) -> ConstantInfo {
    ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: nm(name),
            level_params: lparams.iter().map(|p| Name::str(*p)).collect(),
            ty,
        },
        is_unsafe: false,
    })
}

fn definition(name: &str, ty: Expr, value: Expr) -> ConstantInfo {
    ConstantInfo::Definition(DefinitionVal {
        common: ConstantVal {
            name: nm(name),
            level_params: vec![],
            ty,
        },
        value,
        hints: ReducibilityHint::Regular(1),
        safety: DefinitionSafety::Safe,
        all: vec![nm(name)],
    })
}

fn theorem(name: &str, lparams: &[&str], ty: Expr, value: Expr) -> ConstantInfo {
    ConstantInfo::Theorem(TheoremVal {
        common: ConstantVal {
            name: nm(name),
            level_params: lparams.iter().map(|p| Name::str(*p)).collect(),
            ty,
        },
        value,
        all: vec![nm(name)],
    })
}

fn add(env: &mut Environment, ci: ConstantInfo) {
    let name = ci.name().clone();
    if let Err(e) = check_constant_info(env, ci) {
        panic!("{name} must check: {e:?}");
    }
}

fn message(e: &KernelError) -> String {
    format!("{e:?}")
}

/// The error of a group the kernel must refuse, with nothing left behind.
fn refused(env: &mut Environment, group: Vec<ConstantInfo>, what: &str) -> String {
    let names: Vec<Name> = group.iter().map(|ci| ci.name().clone()).collect();
    let err = match check_constant_infos_atomic(env, group) {
        Ok(()) => panic!("{what}: the group must be refused"),
        Err(e) => message(&e),
    };
    for n in &names {
        assert!(!env.contains(n), "{what}: {n} must not be installed");
    }
    err
}

/// `e` with every constant named `from` renamed `to`.
fn rename_const(e: &Expr, from: &Name, to: &Name) -> Expr {
    replace_expr(e, &mut |sub, _| match sub {
        Expr::Const(n, levels) if n == from => Some(Expr::Const(to.clone(), levels.clone())),
        _ => None,
    })
}

/// A recursor with every reference to a constant in `map` (its own name
/// included) renamed.
fn rename_in_recursor(rv: &RecursorVal, map: &[(Name, Name)]) -> RecursorVal {
    let mut out = rv.clone();
    for (from, to) in map {
        if rv.common.name == *from {
            out.common.name = to.clone();
        }
    }
    // Two passes through a name no declaration has, so that a swap works.
    let through = |e: &Expr| -> Expr {
        let mut cur = e.clone();
        for (i, (from, _)) in map.iter().enumerate() {
            cur = rename_const(&cur, from, &Name::str(format!("«tmp{i}»")));
        }
        for (i, (_, to)) in map.iter().enumerate() {
            cur = rename_const(&cur, &Name::str(format!("«tmp{i}»")), to);
        }
        cur
    };
    out.common.ty = through(&rv.common.ty);
    for rule in &mut out.rules {
        rule.rhs = through(&rule.rhs);
    }
    out
}

// ---------------------------------------------------------------------------
// A recursor is the recursor of the type it is named after
// ---------------------------------------------------------------------------

/// `mutual inductive A | a0 | a1 : B → A; inductive B | b0 | b1 : A → B`
/// as the constants an export carries (types, constructors, recursors).
fn mutual_ab() -> Vec<ConstantInfo> {
    let specs = vec![
        InductiveSpec::new(
            nm("A"),
            type0(),
            vec![
                (nm("A.a0"), cnst("A")),
                (nm("A.a1"), arrow(cnst("B"), cnst("A"))),
            ],
        ),
        InductiveSpec::new(
            nm("B"),
            type0(),
            vec![
                (nm("B.b0"), cnst("B")),
                (nm("B.b1"), arrow(cnst("A"), cnst("B"))),
            ],
        ),
    ];
    match check_and_derive_family(&Environment::new(), &[], 0, &specs) {
        Ok(fam) => fam.into_constant_infos(),
        Err(e) => panic!("A / B must derive: {e:?}"),
    }
}

/// The position of the recursor `name` in a group.
fn recursor_at(group: &[ConstantInfo], name: &str) -> usize {
    group
        .iter()
        .position(|ci| matches!(ci, ConstantInfo::Recursor(rv) if rv.common.name == nm(name)))
        .unwrap_or_else(|| panic!("no recursor {name} in the group"))
}

fn recursor(group: &[ConstantInfo], name: &str) -> RecursorVal {
    match &group[recursor_at(group, name)] {
        ConstantInfo::Recursor(rv) => rv.clone(),
        other => panic!("{name} is not a recursor: {other:?}"),
    }
}

#[test]
fn the_derived_mutual_family_checks_as_a_group() {
    let mut env = Environment::new();
    check_constant_infos_atomic(&mut env, mutual_ab()).expect("the honest group checks");
    for name in ["A", "B", "A.a1", "B.b1", "A.rec", "B.rec"] {
        assert!(env.contains(&nm(name)), "{name} must be installed");
    }
}

#[test]
fn a_recursor_that_is_a_renamed_copy_of_its_sibling_is_refused() {
    // `B.rec` := the recursor of `A` under `B.rec`'s name. It IS a derived
    // recursor of the family, so a comparison with "any derived recursor"
    // accepts it — and a term that applies `B.rec` to an `A` then checks.
    let mut group = mutual_ab();
    let mut copy = recursor(&group, "A.rec");
    copy.common.name = nm("B.rec");
    let at = recursor_at(&group, "B.rec");
    group[at] = ConstantInfo::Recursor(copy);
    let err = refused(&mut Environment::new(), group, "B.rec a copy of A.rec");
    assert!(
        err.contains("'B.rec' does not match the kernel-derived recursor for 'B'"),
        "{err}"
    );
}

#[test]
fn two_recursors_with_their_names_exchanged_are_refused() {
    let mut group = mutual_ab();
    let (a, b) = (recursor_at(&group, "A.rec"), recursor_at(&group, "B.rec"));
    let (mut ra, mut rb) = (recursor(&group, "A.rec"), recursor(&group, "B.rec"));
    ra.common.name = nm("B.rec");
    rb.common.name = nm("A.rec");
    group[a] = ConstantInfo::Recursor(rb);
    group[b] = ConstantInfo::Recursor(ra);
    let err = refused(&mut Environment::new(), group, "A.rec and B.rec exchanged");
    assert!(
        err.contains("'A.rec' does not match the kernel-derived recursor for 'A'"),
        "{err}"
    );
}

#[test]
fn a_recursor_named_after_no_type_of_its_family_is_refused() {
    // In a mutual block, and for a single type: Lean's kernel names the
    // recursor of `T` `T.rec`.
    let mut group = mutual_ab();
    let at = recursor_at(&group, "B.rec");
    let mut rb = recursor(&group, "B.rec");
    rb.common.name = nm("B.recursor");
    group[at] = ConstantInfo::Recursor(rb);
    let err = refused(&mut Environment::new(), group, "B.rec named B.recursor");
    assert!(
        err.contains("'B.recursor' is not named after a type of its family"),
        "{err}"
    );

    let spec = InductiveSpec::new(
        nm("N"),
        type0(),
        vec![
            (nm("N.zero"), cnst("N")),
            (nm("N.succ"), arrow(cnst("N"), cnst("N"))),
        ],
    );
    let mut single = check_and_derive_family(&Environment::new(), &[], 0, &[spec])
        .expect("N derives")
        .into_constant_infos();
    let at = recursor_at(&single, "N.rec");
    let renamed = rename_in_recursor(
        &recursor(&single, "N.rec"),
        &[(nm("N.rec"), nm("N.recursor"))],
    );
    single[at] = ConstantInfo::Recursor(renamed);
    let err = refused(
        &mut Environment::new(),
        single,
        "N.rec named N.recursor throughout",
    );
    assert!(
        err.contains("'N.recursor' is not named after a type of its family"),
        "{err}"
    );
}

#[test]
fn the_rules_of_a_recursor_are_the_constructors_in_order() {
    // The two rules of `A.rec` in the other order: the same rules, by
    // name — and not what Lean's kernel makes.
    let mut group = mutual_ab();
    let at = recursor_at(&group, "A.rec");
    let mut ra = recursor(&group, "A.rec");
    ra.rules.reverse();
    group[at] = ConstantInfo::Recursor(ra);
    let err = refused(
        &mut Environment::new(),
        group,
        "the rules of A.rec reversed",
    );
    assert!(err.contains("'A.rec' does not match"), "{err}");

    // A rule that names a constructor of the sibling.
    let mut group = mutual_ab();
    let mut ra = recursor(&group, "A.rec");
    ra.rules[0].ctor = nm("B.b0");
    group[at] = ConstantInfo::Recursor(ra);
    let err = refused(&mut Environment::new(), group, "a rule of A.rec for B.b0");
    assert!(err.contains("'A.rec' does not match"), "{err}");

    // One rule twice, the other missing.
    let mut group = mutual_ab();
    let mut ra = recursor(&group, "A.rec");
    ra.rules[1] = ra.rules[0].clone();
    group[at] = ConstantInfo::Recursor(ra);
    let err = refused(&mut Environment::new(), group, "the rule of A.a0 twice");
    assert!(err.contains("'A.rec' does not match"), "{err}");
}

/// `inductive NestedT | mk : List NestedT → NestedT` in the builtin
/// environment: the spec, and the recursors an export carries — the
/// kernel's own derivation under Lean's names, `NestedT.rec` and
/// `NestedT.rec_1`.
fn nested_t() -> (Environment, InductiveSpec, Vec<RecursorVal>) {
    let mut env = Environment::new();
    init_builtin_env(&mut env).expect("builtin env");
    let t = Expr::Const(Name::str("NestedT"), vec![]);
    let list_t = app(
        Expr::Const(Name::str("List"), vec![Level::zero()]),
        t.clone(),
    );
    let spec = InductiveSpec::new(
        Name::str("NestedT"),
        type0(),
        vec![(Name::str("NestedT.mk"), arrow(list_t, t))],
    );
    let (raw, exp) = check_and_derive_family_nested(&env, &[], 0, std::slice::from_ref(&spec))
        .expect("NestedT derives");
    let restored = restore_nested(raw, &exp.expect("NestedT is nested"));
    let provisional: Vec<RecursorVal> = restored
        .recursors
        .iter()
        .filter_map(|ci| match ci {
            ConstantInfo::Recursor(rv) => Some(rv.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(provisional.len(), 2, "one recursor and one auxiliary");
    let lean_names = [
        Name::str("NestedT").append_str("rec"),
        Name::str("NestedT").append_str("rec_1"),
    ];
    let map: Vec<(Name, Name)> = provisional
        .iter()
        .zip(&lean_names)
        .map(|(rv, lean)| (rv.common.name.clone(), lean.clone()))
        .collect();
    let recs = provisional
        .iter()
        .map(|rv| rename_in_recursor(rv, &map))
        .collect();
    (env, spec, recs)
}

#[test]
fn a_mutual_nested_family_with_one_container_is_verified() {
    // `mutual inductive A | mk : List B → A; inductive B | mk : List A → B`:
    // two auxiliary types, `List B` and `List A`, with the SAME
    // constructors. Lean names their recursors `A.rec_1` and `A.rec_2`.
    // Paired by the constructors their rules fire on, both derived
    // auxiliary recursors met the same record, and the honest bundle was
    // refused ("exported recursor 'B.rec' does not match the
    // kernel-derived nested recursor": measured on 0.1.4 with the bundle
    // Lean's own kernel makes).
    let mut env = Environment::new();
    init_builtin_env(&mut env).expect("builtin env");
    let list_of = |t: &str| {
        app(
            Expr::Const(Name::str("List"), vec![Level::zero()]),
            Expr::Const(Name::str(t), vec![]),
        )
    };
    let specs = vec![
        InductiveSpec::new(
            Name::str("A"),
            type0(),
            vec![(
                Name::str("A.mk"),
                arrow(list_of("B"), Expr::Const(Name::str("A"), vec![])),
            )],
        ),
        InductiveSpec::new(
            Name::str("B"),
            type0(),
            vec![(
                Name::str("B.mk"),
                arrow(list_of("A"), Expr::Const(Name::str("B"), vec![])),
            )],
        ),
    ];
    let (raw, exp) = check_and_derive_family_nested(&env, &[], 0, &specs).expect("A / B derive");
    let restored = restore_nested(raw, &exp.expect("A / B are nested"));
    let provisional: Vec<RecursorVal> = restored
        .recursors
        .iter()
        .filter_map(|ci| match ci {
            ConstantInfo::Recursor(rv) => Some(rv.clone()),
            _ => None,
        })
        .collect();
    let lean_names = [
        Name::str("A").append_str("rec"),
        Name::str("B").append_str("rec"),
        Name::str("A").append_str("rec_1"),
        Name::str("A").append_str("rec_2"),
    ];
    assert_eq!(provisional.len(), lean_names.len());
    let map: Vec<(Name, Name)> = provisional
        .iter()
        .zip(&lean_names)
        .map(|(rv, lean)| (rv.common.name.clone(), lean.clone()))
        .collect();
    let recs: Vec<RecursorVal> = provisional
        .iter()
        .map(|rv| rename_in_recursor(rv, &map))
        .collect();
    let family = verify_nested_bundle(&env, &[], 0, &specs, &recs).expect("the honest bundle");
    let names: Vec<String> = family
        .recursors
        .iter()
        .map(|ci| ci.name().to_string())
        .collect();
    assert_eq!(names, ["A.rec", "B.rec", "A.rec_1", "A.rec_2"]);
}

#[test]
fn a_nested_family_is_verified_under_leans_recursor_names() {
    let (env, spec, recs) = nested_t();
    let family = verify_nested_bundle(&env, &[], 0, &[spec], &recs).expect("the honest bundle");
    let names: Vec<String> = family
        .recursors
        .iter()
        .map(|ci| ci.name().to_string())
        .collect();
    assert_eq!(names, ["NestedT.rec", "NestedT.rec_1"]);
}

#[test]
fn nested_recursors_exchanged_throughout_the_bundle_are_refused() {
    // `NestedT.rec` and `NestedT.rec_1` exchanged in the two names AND in
    // every reference inside the recursors' types and rules: each record is
    // then, internally, a derived recursor under the other's name. Pairing
    // the records with the derivation by the constructors their rules fire
    // on accepted this; a later term would have eliminated a `List NestedT`
    // with the constant called `NestedT.rec`.
    let (env, spec, recs) = nested_t();
    let (a, b) = (recs[0].common.name.clone(), recs[1].common.name.clone());
    let swapped: Vec<RecursorVal> = recs
        .iter()
        .map(|rv| rename_in_recursor(rv, &[(a.clone(), b.clone()), (b.clone(), a.clone())]))
        .collect();
    let err = verify_nested_bundle(&env, &[], 0, std::slice::from_ref(&spec), &swapped)
        .expect_err("exchanged throughout");
    assert!(
        message(&err).contains("does not match the kernel-derived nested recursor"),
        "{err:?}"
    );

    // The auxiliary recursor under a name Lean's kernel does not give it.
    let other = Name::str("NestedT").append_str("rec_7");
    let renamed: Vec<RecursorVal> = recs
        .iter()
        .map(|rv| rename_in_recursor(rv, &[(b.clone(), other.clone())]))
        .collect();
    let err = verify_nested_bundle(&env, &[], 0, &[spec], &renamed)
        .expect_err("the auxiliary recursor renamed");
    assert!(
        message(&err).contains("has no recursor named 'NestedT.rec_1'"),
        "{err:?}"
    );
}

// ---------------------------------------------------------------------------
// A theorem states a proposition
// ---------------------------------------------------------------------------

/// `P0 : Prop`, `p0 : P0`; `T1 : Type`, `t1 : T1`; `PI.{u} : Sort (imax u 0)`,
/// `pi.{u} : PI`; `PM : Sort (max 0 0)`, `pm : PM`; `S : Type := Prop`,
/// `Q : S`, `q : Q`; `TU.{u} : Sort u`, `tu.{u} : TU`.
fn theorem_env() -> Environment {
    let mut env = Environment::new();
    add(&mut env, axiom("P0", &[], prop()));
    add(&mut env, axiom("p0", &[], cnst("P0")));
    add(&mut env, axiom("T1", &[], type0()));
    add(&mut env, axiom("t1", &[], cnst("T1")));
    add(
        &mut env,
        axiom("PI", &["u"], Expr::Sort(Level::imax(u(), Level::zero()))),
    );
    add(&mut env, axiom("pi", &["u"], cnst_at("PI", vec![u()])));
    add(
        &mut env,
        axiom(
            "PM",
            &[],
            Expr::Sort(Level::max(Level::zero(), Level::zero())),
        ),
    );
    add(&mut env, axiom("pm", &[], cnst("PM")));
    add(&mut env, definition("S", type0(), prop()));
    add(&mut env, axiom("Q", &[], cnst("S")));
    add(&mut env, axiom("q", &[], cnst("Q")));
    add(&mut env, axiom("TU", &["u"], Expr::Sort(u())));
    add(&mut env, axiom("tu", &["u"], cnst_at("TU", vec![u()])));
    env
}

/// Add `theorem name.{lparams} : ty := value` through both entry points,
/// each on its own copy of the environment; the two must agree.
fn theorem_verdict(name: &str, lparams: &[&str], ty: Expr, value: Expr) -> Result<(), String> {
    let mut by_constant_info = theorem_env();
    let a = check_constant_info(
        &mut by_constant_info,
        theorem(name, lparams, ty.clone(), value.clone()),
    )
    .map_err(|e| message(&e));
    let mut by_declaration = theorem_env();
    let b = check_declaration(
        &mut by_declaration,
        Declaration::Theorem {
            name: nm(name),
            univ_params: lparams.iter().map(|p| Name::str(*p)).collect(),
            ty,
            val: value,
        },
    )
    .map_err(|e| message(&e));
    assert_eq!(a, b, "{name}: the two entry points must agree");
    assert_eq!(
        by_constant_info.contains(&nm(name)),
        a.is_ok(),
        "{name}: installed exactly when accepted"
    );
    a
}

#[test]
fn a_theorem_whose_type_is_a_proposition_is_accepted() {
    // In `Sort 0`.
    theorem_verdict("t_sort0", &[], cnst("P0"), cnst("p0")).expect("a type in Sort 0");
    // In `Sort (imax u 0)`: zero for every `u`. Lean accepts.
    theorem_verdict(
        "t_imax_u_0",
        &["u"],
        cnst_at("PI", vec![u()]),
        cnst_at("pi", vec![u()]),
    )
    .expect("a type in Sort (imax u 0)");
    // Behind a definition that unfolds to `Prop`. Lean accepts.
    theorem_verdict("t_behind_def", &[], cnst("Q"), cnst("q"))
        .expect("a type whose sort is behind a definition");
    // `∀ (α : Sort u) (x : α), P0`: the sort of a Pi type is an `imax`.
    let ty = pi(
        BinderInfo::Default,
        "α",
        Expr::Sort(u()),
        pi(BinderInfo::Default, "x", Expr::BVar(0), cnst("P0")),
    );
    let value = lam("α", Expr::Sort(u()), lam("x", Expr::BVar(0), cnst("p0")));
    theorem_verdict("t_pi", &["u"], ty, value).expect("a Pi into a proposition");
}

#[test]
fn a_theorem_whose_type_is_not_a_proposition_is_refused() {
    // `theorem t : T1 := t1` with `T1 : Type` — Lean: "(kernel) type of
    // theorem 't' is not a proposition".
    let err = theorem_verdict("t_type", &[], cnst("T1"), cnst("t1")).expect_err("a type in Sort 1");
    assert!(
        err.contains("type of theorem 't_type' is not a proposition"),
        "{err}"
    );
    // `theorem t : Type := Prop`.
    let err = theorem_verdict("t_sort", &[], type0(), prop()).expect_err("a sort as the type");
    assert!(err.contains("is not a proposition"), "{err}");
    // `theorem t.{u} : TU.{u} := tu.{u}` with `TU.{u} : Sort u`: a
    // proposition only at `u = 0`.
    let err = theorem_verdict(
        "t_sort_u",
        &["u"],
        cnst_at("TU", vec![u()]),
        cnst_at("tu", vec![u()]),
    )
    .expect_err("a type in Sort u");
    assert!(err.contains("is not a proposition"), "{err}");
    // The same value under the same type is a perfectly good definition.
    let mut env = theorem_env();
    check_constant_info(&mut env, definition("d_type", cnst("T1"), cnst("t1")))
        .expect("as a definition it checks");
}

#[test]
fn a_universe_that_is_zero_only_after_normalisation_counts_as_prop() {
    // KNOWN DIFFERENCE from Lean's kernel, in the accepting direction:
    // `PM : Sort (max 0 0)` is a universe level no Lean elaborator writes.
    // Lean's kernel holds a level without a parameter as it was written
    // and compares it with `0` syntactically, so it refuses
    // `theorem t : PM := pm` ("type of theorem 't' is not a proposition")
    // while it accepts the same statement as a definition, and accepts
    // `Sort (imax u 0)` (which it simplifies when it instantiates `u`).
    // This kernel compares the sort with `Prop` up to the equivalence of
    // universe levels — `max 0 0 = 0` — and accepts.
    theorem_verdict("t_max_0_0", &[], cnst("PM"), cnst("pm"))
        .expect("a type in Sort (max 0 0) is a proposition");
}

// ---------------------------------------------------------------------------
// `Eq` has the shape the quotient primitives assume
// ---------------------------------------------------------------------------

/// `{α : Sort u} → α → α → codomain`, over the level `lvl`, with the first
/// binder as given.
fn eq_type(first: BinderInfo, lvl: Level, codomain: Expr) -> Expr {
    pi(
        first,
        "α",
        Expr::Sort(lvl),
        pi(
            BinderInfo::Default,
            "a",
            Expr::BVar(0),
            pi(BinderInfo::Default, "b", Expr::BVar(1), codomain),
        ),
    )
}

/// `{α : Sort u} → (a : α) → @Eq α a a`.
fn eq_refl_type(first: BinderInfo, lvl: Level, levels: Vec<Level>) -> Expr {
    pi(
        first,
        "α",
        Expr::Sort(lvl),
        pi(
            BinderInfo::Default,
            "a",
            Expr::BVar(0),
            apps(
                cnst_at("Eq", levels),
                vec![Expr::BVar(1), Expr::BVar(0), Expr::BVar(0)],
            ),
        ),
    )
}

/// Declare `Eq` as given (the kernel checks it and derives its recursor),
/// then ask for the quotient primitives.
fn add_quot_over(
    lparams: &[&str],
    num_params: u32,
    ty: Expr,
    ctors: Vec<(&str, Expr)>,
) -> Result<(), String> {
    let mut env = Environment::new();
    let spec = InductiveSpec::new(
        nm("Eq"),
        ty,
        ctors.into_iter().map(|(n, t)| (nm(n), t)).collect(),
    );
    add_inductive_family(
        &mut env,
        lparams.iter().map(|p| Name::str(*p)).collect(),
        num_params,
        vec![spec],
    )
    .unwrap_or_else(|e| panic!("this `Eq` is a valid inductive type: {e:?}"));
    let result = env.add_quot().map_err(|e| e.to_string());
    assert_eq!(
        env.quot_initialized(),
        result.is_ok(),
        "the flag follows the result"
    );
    assert_eq!(
        env.contains(&nm("Quot")),
        result.is_ok(),
        "nothing installed on a refusal"
    );
    result
}

#[test]
fn quotients_are_installed_over_leans_eq() {
    add_quot_over(
        &["u"],
        2,
        eq_type(BinderInfo::Implicit, u(), prop()),
        vec![(
            "Eq.refl",
            eq_refl_type(BinderInfo::Implicit, u(), vec![u()]),
        )],
    )
    .expect("Lean's Eq");
}

#[test]
fn quotients_are_installed_over_an_eq_that_differs_where_lean_does_not_look() {
    // Lean's kernel compares the two types up to binder names and binder
    // information, and does not look at the constructor's name nor at the
    // split of `Eq`'s arguments into parameters and indices (measured: it
    // installs the primitives over each of these three).
    add_quot_over(
        &["u"],
        2,
        eq_type(BinderInfo::Default, u(), prop()),
        vec![("Eq.refl", eq_refl_type(BinderInfo::Default, u(), vec![u()]))],
    )
    .expect("an explicit first binder");
    add_quot_over(
        &["u"],
        2,
        eq_type(BinderInfo::Implicit, u(), prop()),
        vec![("Eq.mk", eq_refl_type(BinderInfo::Implicit, u(), vec![u()]))],
    )
    .expect("a constructor of another name");
    add_quot_over(
        &["u"],
        1,
        eq_type(BinderInfo::Implicit, u(), prop()),
        vec![(
            "Eq.refl",
            eq_refl_type(BinderInfo::Implicit, u(), vec![u()]),
        )],
    )
    .expect("the left-hand side an index");
}

#[test]
fn quotients_are_refused_over_an_eq_of_another_shape() {
    let v = || Level::param(Name::str("v"));
    // Two universe parameters (the second unused).
    let err = add_quot_over(
        &["u", "v"],
        2,
        eq_type(BinderInfo::Implicit, u(), prop()),
        vec![(
            "Eq.refl",
            eq_refl_type(BinderInfo::Implicit, u(), vec![u(), v()]),
        )],
    )
    .expect_err("Eq.{u, v}");
    assert!(err.contains("exactly one universe parameter"), "{err}");

    // Over `Type u` only: `{α : Sort (u+1)} → α → α → Prop`.
    let su = || Level::succ(u());
    let err = add_quot_over(
        &["u"],
        2,
        eq_type(BinderInfo::Implicit, su(), prop()),
        vec![(
            "Eq.refl",
            eq_refl_type(BinderInfo::Implicit, su(), vec![u()]),
        )],
    )
    .expect_err("Eq over Type u");
    assert!(err.contains("`Eq` must have the type"), "{err}");

    // Unary: `{α : Sort u} → α → Prop`, `refl : (a : α) → Eq a`. The
    // canonical type of `Quot.lift` applies `Eq` to three arguments.
    let unary = pi(
        BinderInfo::Implicit,
        "α",
        Expr::Sort(u()),
        pi(BinderInfo::Default, "a", Expr::BVar(0), prop()),
    );
    let unary_refl = pi(
        BinderInfo::Implicit,
        "α",
        Expr::Sort(u()),
        pi(
            BinderInfo::Default,
            "a",
            Expr::BVar(0),
            apps(cnst_at("Eq", vec![u()]), vec![Expr::BVar(1), Expr::BVar(0)]),
        ),
    );
    let err =
        add_quot_over(&["u"], 2, unary, vec![("Eq.refl", unary_refl)]).expect_err("a unary Eq");
    assert!(err.contains("`Eq` must have the type"), "{err}");

    // `refl : (a b : α) → Eq a b`: one constructor, in `Prop`, and every
    // two elements are "equal" — `Quot.sound` would then say nothing and
    // the hypothesis of `Quot.lift` would hold of every function.
    let relates_all = pi(
        BinderInfo::Implicit,
        "α",
        Expr::Sort(u()),
        pi(
            BinderInfo::Default,
            "a",
            Expr::BVar(0),
            pi(
                BinderInfo::Default,
                "b",
                Expr::BVar(1),
                apps(
                    cnst_at("Eq", vec![u()]),
                    vec![Expr::BVar(2), Expr::BVar(1), Expr::BVar(0)],
                ),
            ),
        ),
    );
    let err = add_quot_over(
        &["u"],
        1,
        eq_type(BinderInfo::Implicit, u(), prop()),
        vec![("Eq.refl", relates_all)],
    )
    .expect_err("a constructor that relates any two elements");
    assert!(
        err.contains("`Eq`'s constructor must have the type"),
        "{err}"
    );

    // In `Type`, with two constructors, with none: refused before this
    // round as well.
    let err = add_quot_over(
        &["u"],
        2,
        eq_type(BinderInfo::Implicit, u(), type0()),
        vec![(
            "Eq.refl",
            eq_refl_type(BinderInfo::Implicit, u(), vec![u()]),
        )],
    )
    .expect_err("Eq in Type");
    assert!(err.contains("`Eq` must have the type"), "{err}");
    let err = add_quot_over(
        &["u"],
        2,
        eq_type(BinderInfo::Implicit, u(), prop()),
        vec![],
    )
    .expect_err("Eq with no constructor");
    assert!(err.contains("exactly one constructor"), "{err}");
}

// ---------------------------------------------------------------------------
// The kernel's own literal primitives
// ---------------------------------------------------------------------------

/// An environment as an export gives it: `Nat`, `Bool` and `Eq` declared by
/// the "file", nothing builtin.
fn replayed_env() -> Environment {
    let mut env = Environment::new();
    let family = |env: &mut Environment, lparams: Vec<Name>, np: u32, spec: InductiveSpec| {
        add_inductive_family(env, lparams, np, vec![spec])
            .unwrap_or_else(|e| panic!("prelude: {e:?}"));
    };
    family(
        &mut env,
        vec![],
        0,
        InductiveSpec::new(
            nm("Nat"),
            type0(),
            vec![
                (nm("Nat.zero"), cnst("Nat")),
                (nm("Nat.succ"), arrow(cnst("Nat"), cnst("Nat"))),
            ],
        ),
    );
    family(
        &mut env,
        vec![],
        0,
        InductiveSpec::new(
            nm("Bool"),
            type0(),
            vec![
                (nm("Bool.false"), cnst("Bool")),
                (nm("Bool.true"), cnst("Bool")),
            ],
        ),
    );
    family(
        &mut env,
        vec![Name::str("u")],
        2,
        InductiveSpec::new(
            nm("Eq"),
            eq_type(BinderInfo::Implicit, u(), prop()),
            vec![(
                nm("Eq.refl"),
                eq_refl_type(BinderInfo::Implicit, u(), vec![u()]),
            )],
        ),
    );
    env
}

fn level1() -> Level {
    Level::succ(Level::zero())
}

/// `theorem name : @Eq ty lhs rhs := @Eq.refl ty rhs`.
fn by_refl(name: &str, ty: Expr, lhs: Expr, rhs: Expr) -> ConstantInfo {
    theorem(
        name,
        &[],
        apps(
            cnst_at("Eq", vec![level1()]),
            vec![ty.clone(), lhs, rhs.clone()],
        ),
        apps(cnst_at("Eq.refl", vec![level1()]), vec![ty, rhs]),
    )
}

fn nat_fn(arity: u32, result: Expr) -> Expr {
    (0..arity).fold(result, |acc, _| arrow(cnst("Nat"), acc))
}

#[test]
fn an_operation_leans_kernel_computes_by_name_is_computed_by_name() {
    // SHARED LIMIT, stated: `Nat.add` is an axiom here, and `Nat.add 7 5 =
    // 12` checks by `Eq.refl` — in Lean's kernel as well (measured). Both
    // kernels believe an environment about the operations of Lean's
    // extension.
    let mut env = replayed_env();
    assert!(!env.has_literal_primitives());
    add(&mut env, axiom("Nat.add", &[], nat_fn(2, cnst("Nat"))));
    let lhs = apps(cnst("Nat.add"), vec![nat_lit(7), nat_lit(5)]);
    check_constant_info(&mut env, by_refl("t", cnst("Nat"), lhs, nat_lit(12)))
        .expect("Nat.add on literals, by name");
}

#[test]
fn an_axiom_named_like_a_kernel_primitive_is_not_computed() {
    // `Nat.pred`, `Nat.blt`, `Nat.log2` declared as AXIOMS by the file:
    // Lean's kernel computes none of them by name, and each of these
    // theorems is refused by it.
    for (op, arity, args, result, result_ty) in [
        ("Nat.pred", 1, vec![nat_lit(5)], nat_lit(4), cnst("Nat")),
        ("Nat.log2", 1, vec![nat_lit(1024)], nat_lit(10), cnst("Nat")),
        (
            "Nat.blt",
            2,
            vec![nat_lit(5), nat_lit(7)],
            cnst("Bool.true"),
            cnst("Bool"),
        ),
    ] {
        let mut env = replayed_env();
        add(&mut env, axiom(op, &[], nat_fn(arity, result_ty.clone())));
        let lhs = apps(cnst(op), args);
        let verdict = check_constant_info(&mut env, by_refl("t", result_ty, lhs, result));
        assert!(
            verdict.is_err(),
            "{op} is an axiom of the file: nothing computes it"
        );
    }
}

#[test]
fn a_definition_named_like_a_kernel_primitive_reduces_by_its_definition() {
    // `def Nat.pred (n : Nat) : Nat := n` — not Lean's `Nat.pred`. By the
    // definition `Nat.pred 5 = 5`; the reducer's own rule would say `4`.
    let mut env = replayed_env();
    add(
        &mut env,
        definition(
            "Nat.pred",
            nat_fn(1, cnst("Nat")),
            lam("n", cnst("Nat"), Expr::BVar(0)),
        ),
    );
    let lhs = || app(cnst("Nat.pred"), nat_lit(5));
    check_constant_info(
        &mut env,
        by_refl("by_definition", cnst("Nat"), lhs(), nat_lit(5)),
    )
    .expect("Nat.pred 5 = 5 by the file's definition");
    let verdict = check_constant_info(
        &mut env,
        by_refl("by_the_reducers_rule", cnst("Nat"), lhs(), nat_lit(4)),
    );
    assert!(
        verdict.is_err(),
        "Nat.pred 5 = 4 does not hold of the file's Nat.pred"
    );
}

#[test]
fn the_builtin_environment_keeps_its_literal_primitives() {
    // There the six operations are declared without a definition, and the
    // reducer's rule is what they mean.
    let mut env = Environment::new();
    init_builtin_env(&mut env).expect("builtin env");
    assert!(env.has_literal_primitives());
    let blt = apps(
        Expr::Const(Name::str("Nat.blt"), vec![]),
        vec![nat_lit(5), nat_lit(7)],
    );
    let mut tc = TypeChecker::new(&env);
    assert_eq!(
        tc.whnf(&blt),
        Expr::Const(Name::str("Bool.true"), vec![]),
        "Nat.blt 5 7 in the builtin environment"
    );
    let log2 = app(Expr::Const(Name::str("Nat.log2"), vec![]), nat_lit(1024));
    assert_eq!(tc.whnf(&log2), nat_lit(10));
}

// ---------------------------------------------------------------------------
// The shape of an inductive declaration
// ---------------------------------------------------------------------------

fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}

/// The kernel's verdict on a family, checked against `env`.
fn family_verdict(
    env: &Environment,
    num_params: u32,
    specs: &[InductiveSpec],
) -> Result<(), String> {
    check_and_derive_family(env, &[], num_params, specs)
        .map(|_| ())
        .map_err(|e| message(&e))
}

/// `Nat`, `True` and `False` as a file declares them.
fn small_prelude() -> Environment {
    let mut env = Environment::new();
    for spec in [
        InductiveSpec::new(
            nm("Nat"),
            type0(),
            vec![
                (nm("Nat.zero"), cnst("Nat")),
                (nm("Nat.succ"), arrow(cnst("Nat"), cnst("Nat"))),
            ],
        ),
        InductiveSpec::new(nm("True"), prop(), vec![(nm("True.intro"), cnst("True"))]),
        InductiveSpec::new(nm("False"), prop(), vec![]),
    ] {
        add_inductive_family(&mut env, vec![], 0, vec![spec])
            .unwrap_or_else(|e| panic!("prelude: {e:?}"));
    }
    env
}

#[test]
fn an_index_that_mentions_the_family_is_refused() {
    // `J : Type 1 → Type 1`.
    let j = |arg: Expr| app(cnst("J"), arg);
    let family = |mk: Expr| {
        vec![InductiveSpec::new(
            nm("J"),
            arrow(type1(), type1()),
            vec![(nm("J.mk"), mk)],
        )]
    };
    let env = Environment::new();
    // Control: `mk : J Type → J Type`.
    family_verdict(&env, 0, &family(arrow(j(type0()), j(type0())))).expect("indices without J");
    // `mk : J (J Type)` — Lean: "invalid return type for 'J.mk'".
    let err =
        family_verdict(&env, 0, &family(j(j(type0())))).expect_err("the result's index mentions J");
    assert!(
        err.contains("index terms that do not mention the family"),
        "{err}"
    );
    // `mk : J (J Type) → J Type` — Lean: "arg #1 of 'J.mk' contains a non
    // valid occurrence of the datatypes being declared".
    let err = family_verdict(&env, 0, &family(arrow(j(j(type0())), j(type0()))))
        .expect_err("a recursive field's index mentions J");
    assert!(err.contains("invalid occurrence of 'J'"), "{err}");

    // What the condition stands against: `I : Prop → Prop` with
    // `mk : I (I True → False)`. Every other condition holds of it, and
    // with propositional extensionality it is inconsistent (see
    // `as_valid_ind_app`).
    let env = small_prelude();
    let i_true = app(cnst("I"), cnst("True"));
    let paradox = InductiveSpec::new(
        nm("I"),
        arrow(prop(), prop()),
        vec![(nm("I.mk"), app(cnst("I"), arrow(i_true, cnst("False"))))],
    );
    let err =
        family_verdict(&env, 0, &[paradox]).expect_err("I : Prop → Prop | mk : I (I True → False)");
    assert!(
        err.contains("index terms that do not mention the family"),
        "{err}"
    );
}

#[test]
fn the_parameters_of_an_inductive_type_are_binders_as_written() {
    // `Fam : Type 1 := Nat → Type`, `T : Fam`, `mk : (n : Nat) → T n`.
    let mut env = small_prelude();
    add(
        &mut env,
        definition("Fam", type1(), arrow(cnst("Nat"), type0())),
    );
    let family = || {
        vec![InductiveSpec::new(
            nm("T"),
            cnst("Fam"),
            vec![(
                nm("T.mk"),
                pi(
                    BinderInfo::Default,
                    "n",
                    cnst("Nat"),
                    app(cnst("T"), Expr::BVar(0)),
                ),
            )],
        )]
    };
    // With no parameter, `n` is an index behind the definition: Lean
    // accepts (and records `numIndices = 1`).
    family_verdict(&env, 0, &family()).expect("an index behind a definition");
    // With one parameter: Lean, "invalid inductive datatype declaration,
    // incorrect number of parameters".
    let err = family_verdict(&env, 1, &family()).expect_err("a parameter behind a definition");
    assert!(err.contains("fewer than 1 parameter binders"), "{err}");
}

#[test]
fn a_constructor_type_is_read_as_it_is_written() {
    let mut env = small_prelude();
    // `Id α := α`, `Fn α := Nat → α`, `Pt β := (n : Nat) → β n`.
    add(
        &mut env,
        definition(
            "Id",
            arrow(type0(), type0()),
            lam("α", type0(), Expr::BVar(0)),
        ),
    );
    add(
        &mut env,
        definition(
            "Fn",
            arrow(type0(), type0()),
            lam("α", type0(), arrow(cnst("Nat"), Expr::BVar(1))),
        ),
    );
    let nat_family = arrow(cnst("Nat"), type0());
    add(
        &mut env,
        definition(
            "Pt",
            arrow(nat_family.clone(), type0()),
            lam(
                "β",
                nat_family.clone(),
                pi(
                    BinderInfo::Default,
                    "n",
                    cnst("Nat"),
                    app(Expr::BVar(1), Expr::BVar(0)),
                ),
            ),
        ),
    );
    let one = |ty: Expr, mk: Expr| vec![InductiveSpec::new(nm("T"), ty, vec![(nm("T.mk"), mk)])];

    // The result behind a definition: `mk : Id T` — Lean, "invalid return
    // type for 'T.mk'"; written out, `mk : T`.
    family_verdict(&env, 0, &one(type0(), cnst("T"))).expect("mk : T");
    let err =
        family_verdict(&env, 0, &one(type0(), app(cnst("Id"), cnst("T")))).expect_err("mk : Id T");
    assert!(err.contains("must return 'T'"), "{err}");

    // A field behind a definition: `mk : Fn T`; written out, `mk : Nat → T`.
    family_verdict(&env, 0, &one(type0(), arrow(cnst("Nat"), cnst("T")))).expect("mk : Nat → T");
    let err =
        family_verdict(&env, 0, &one(type0(), app(cnst("Fn"), cnst("T")))).expect_err("mk : Fn T");
    assert!(err.contains("must return 'T'"), "{err}");

    // The parameter behind a definition: `T (n : Nat) : Type`, `mk : Pt T`
    // — Lean, "incorrect number of parameters"; written out,
    // `mk : (n : Nat) → T n`.
    let written = pi(
        BinderInfo::Default,
        "n",
        cnst("Nat"),
        app(cnst("T"), Expr::BVar(0)),
    );
    family_verdict(&env, 1, &one(nat_family.clone(), written)).expect("mk : (n : Nat) → T n");
    let err = family_verdict(&env, 1, &one(nat_family, app(cnst("Pt"), cnst("T"))))
        .expect_err("mk : Pt T");
    assert!(
        err.contains("does not bind the 1 family parameter"),
        "{err}"
    );
}
