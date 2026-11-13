//! The names a derived term mentions, and the guard of an install.
//!
//! The kernel installs its own recursors (see `derived_recursor.rs`), and a
//! derived rule calls the recursors of the sibling types BY NAME. Those
//! names must be the ones the install puts into the environment: a rule
//! that calls a recursor no one installed lets a later declaration define
//! that name with another type, and the reduct of the installed rule is then
//! ill typed — a closed proof of `False` from no axiom (the export-level
//! fixture `RecNameMixedStyle` is that proof: a mutual bundle whose records
//! named one recursor `A.rec`, the component `rec` under `A`, and the other
//! with the one flat component `"C.rec"`).
//!
//! The guard (`oxilean_kernel::inductive::installed`) holds every constant
//! a derived term names to the environment as it stands after the install:
//! it must resolve, and a name the derivation gives a recursor or a type of
//! the family must resolve to that constant of the family. It runs on both
//! paths — on the group `check_constant_infos_atomic` installs (the
//! ordinary path) and in `add_derived_family` before the nested family is
//! added — and a refusal leaves nothing of the group behind.
//!
//! The last test is of the nested path's other half: what it installs is the
//! derived recursor, not the record it was handed.

use oxilean_kernel::declaration::{AxiomVal, ConstantInfo, ConstantVal, RecursorVal};
use oxilean_kernel::expr_util::replace_expr;
use oxilean_kernel::Node;
use oxilean_kernel::{
    add_derived_family, add_inductive_family, check_and_derive_family,
    check_and_derive_family_nested, check_constant_info, check_constant_infos_atomic,
    restore_nested, same_up_to_binders, verify_nested_bundle, BinderInfo, DerivedFamily,
    Environment, Expr, InductiveSpec, KernelError, Level, Name,
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

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

fn apps(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, app)
}

fn pi(x: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(x),
        Node::new(dom),
        Node::new(body),
    )
}

fn bv(i: u32) -> Expr {
    Expr::BVar(i)
}

fn install(env: &mut Environment, num_params: u32, specs: Vec<InductiveSpec>) {
    if let Err(e) = add_inductive_family(env, vec![], num_params, specs) {
        panic!("the family must be accepted: {e:?}");
    }
}

fn axiom(env: &mut Environment, name: Name, ty: Expr) {
    let ci = ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: name.clone(),
            level_params: vec![],
            ty,
        },
        is_unsafe: false,
    });
    if let Err(e) = check_constant_info(env, ci) {
        panic!("axiom {name} must be accepted: {e:?}");
    }
}

/// `B : Type | t | f`, `P : B → Prop | mk : (b : B) → P b` and the
/// container `L α | nil | cons : α → L α → L α`.
fn base_env() -> Environment {
    let mut env = Environment::new();
    install(
        &mut env,
        0,
        vec![InductiveSpec::new(
            nm("B"),
            type0(),
            vec![(nm("B.t"), cnst("B")), (nm("B.f"), cnst("B"))],
        )],
    );
    install(
        &mut env,
        0,
        vec![InductiveSpec::new(
            nm("P"),
            pi("b", cnst("B"), prop()),
            vec![(nm("P.mk"), pi("b", cnst("B"), app(cnst("P"), bv(0))))],
        )],
    );
    let l = |arg: Expr| app(cnst("L"), arg);
    install(
        &mut env,
        1,
        vec![InductiveSpec::new(
            nm("L"),
            pi("α", type0(), type0()),
            vec![
                (nm("L.nil"), pi("α", type0(), l(bv(0)))),
                (
                    nm("L.cons"),
                    pi(
                        "α",
                        type0(),
                        pi("head", bv(0), pi("tail", l(bv(1)), l(bv(2)))),
                    ),
                ),
            ],
        )],
    );
    env
}

/// The mutual block `A | mk : (b : B) → P b → C → A ;  C | base | step : A → C`.
fn mutual_specs() -> Vec<InductiveSpec> {
    vec![
        InductiveSpec::new(
            nm("A"),
            type0(),
            vec![(
                nm("A.mk"),
                pi(
                    "b",
                    cnst("B"),
                    pi("h", app(cnst("P"), bv(0)), pi("c", cnst("C"), cnst("A"))),
                ),
            )],
        ),
        InductiveSpec::new(
            nm("C"),
            type0(),
            vec![
                (nm("C.base"), cnst("C")),
                (nm("C.step"), pi("a", cnst("A"), cnst("C"))),
            ],
        ),
    ]
}

fn derive_mutual(env: &Environment, specs: &[InductiveSpec]) -> DerivedFamily {
    match check_and_derive_family(env, &[], 0, specs) {
        Ok(fam) => fam,
        Err(e) => panic!("the mutual block must be derivable: {e:?}"),
    }
}

/// The mutual block's types and constructors with the recursor records
/// `recs`.
fn mutual_group(fam: &DerivedFamily, recs: Vec<ConstantInfo>) -> Vec<ConstantInfo> {
    let mut group = fam.inductives.clone();
    group.extend(fam.constructors.iter().cloned());
    group.extend(recs);
    group
}

/// `err` is the guard's refusal and says `needle`.
fn assert_guard_refused(res: Result<(), KernelError>, needle: &str, what: &str) {
    match res {
        Err(KernelError::InvalidRecursor(msg)) => assert!(
            msg.contains("the kernel derived names") && msg.contains(needle),
            "{what}: {msg}"
        ),
        other => panic!("{what}: the guard must refuse the group, got {other:?}"),
    }
}

fn assert_none_installed(env: &Environment, names: &[Name], what: &str) {
    for n in names {
        assert!(
            !env.contains(n),
            "{what}: {n} must not be in the environment"
        );
    }
}

/// `N | mk : <fields> → L N → N` — a family nested through `L` — with the
/// recursor records Lean's kernel makes for it (`N.rec`, `N.rec_1`): the
/// derivation, restored over `L`, under Lean's names.
fn nested_with_records(env: &Environment, spec: &InductiveSpec) -> Vec<RecursorVal> {
    let specs = std::slice::from_ref(spec);
    let (raw, exp) = match check_and_derive_family_nested(env, &[], 0, specs) {
        Ok((raw, Some(exp))) => (raw, exp),
        other => panic!("the family must derive as a nested one: {other:?}"),
    };
    let restored = restore_nested(raw, &exp);
    let lean_name = |position: usize| {
        if position == 0 {
            Name::mk_str(spec.name.clone(), "rec")
        } else {
            Name::mk_str(spec.name.clone(), format!("rec_{position}"))
        }
    };
    let map: Vec<(Name, Name)> = restored
        .recursors
        .iter()
        .enumerate()
        .map(|(i, ci)| (ci.name().clone(), lean_name(i)))
        .collect();
    let rename = |e: &Expr| {
        replace_expr(e, &mut |sub, _| match sub {
            Expr::Const(n, ls) => map
                .iter()
                .find(|(from, _)| from == n)
                .map(|(_, to)| Expr::Const(to.clone(), ls.clone())),
            _ => None,
        })
    };
    restored
        .recursors
        .iter()
        .enumerate()
        .map(|(i, ci)| {
            let mut rv = match ci {
                ConstantInfo::Recursor(rv) => rv.clone(),
                other => panic!("a recursor expected, got {other:?}"),
            };
            rv.common.name = lean_name(i);
            rv.common.ty = rename(&rv.common.ty);
            for rule in &mut rv.rules {
                rule.rhs = rename(&rule.rhs);
            }
            rv
        })
        .collect()
}

fn n_spec(fields: &[(&str, Expr)]) -> InductiveSpec {
    let mut ctor_ty = pi("kids", app(cnst("L"), cnst("N")), cnst("N"));
    for (x, ty) in fields.iter().rev() {
        ctor_ty = pi(x, ty.clone(), ctor_ty);
    }
    InductiveSpec::new(nm("N"), type0(), vec![(nm("N.mk"), ctor_ty)])
}

fn verified_nested(env: &Environment, spec: &InductiveSpec, recs: &[RecursorVal]) -> DerivedFamily {
    match verify_nested_bundle(env, &[], 0, std::slice::from_ref(spec), recs) {
        Ok(fam) => fam,
        Err(e) => panic!("the nested bundle must verify: {e:?}"),
    }
}

fn without_recursor(fam: &DerivedFamily, dropped: &str) -> Vec<ConstantInfo> {
    let mut group = fam.inductives.clone();
    group.extend(fam.constructors.iter().cloned());
    group.extend(
        fam.recursors
            .iter()
            .filter(|ci| ci.name() != &nm(dropped))
            .cloned(),
    );
    group
}

/// `f` applied to what is under the leading lambdas of `e`.
fn under_lambdas(e: &Expr, f: &dyn Fn(&Expr) -> Expr) -> Expr {
    match e {
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            ty.clone(),
            Node::new(under_lambdas(body, f)),
        ),
        other => f(other),
    }
}

/// The application `e` with its argument number `i` replaced.
fn with_arg(e: &Expr, i: usize, arg: Expr) -> Expr {
    let mut args = Vec::new();
    let mut head = e;
    while let Expr::App(f, a) = head {
        args.push((**a).clone());
        head = f;
    }
    args.reverse();
    assert!(
        i < args.len(),
        "the application has {} arguments",
        args.len()
    );
    args[i] = arg;
    apps(head.clone(), args)
}

// ---------------------------------------------------------------------------
// The ordinary path
// ---------------------------------------------------------------------------

#[test]
fn a_mutual_group_whose_records_mix_two_naming_styles_is_refused_by_the_guard() {
    // Lean's kernel names the recursor of `A` `A.rec`: the component `rec`
    // under `A`. This kernel's own tests also write the one flat component
    // `"A.rec"`, and the derivation then names the whole family in that
    // style. A group with ONE record of each style used to be installed as
    // derived: `C.rec`, whose rule for `C.step` calls the hierarchical
    // `A.rec`, and `"A.rec"`, whose rule for `A.mk` calls the flat
    // `"C.rec"` — neither callee installed.
    let env0 = base_env();
    let specs = mutual_specs();
    let hierarchical = derive_mutual(&env0, &specs);
    let flat_specs: Vec<InductiveSpec> = specs
        .iter()
        .cloned()
        .map(|s| {
            let flat = Name::str(format!("{}.rec", s.name));
            s.with_rec_name(flat)
        })
        .collect();
    let flat = derive_mutual(&env0, &flat_specs);
    let mixed = vec![flat.recursors[0].clone(), hierarchical.recursors[1].clone()];
    let names: Vec<Name> = mutual_group(&hierarchical, mixed.clone())
        .iter()
        .map(|ci| ci.name().clone())
        .collect();

    let mut env = env0.clone();
    assert_guard_refused(
        check_constant_infos_atomic(&mut env, mutual_group(&hierarchical, mixed.clone())),
        "the recursor '«A.rec»' the kernel derived names '«C.rec»': it is not in the environment \
         after the install; the bundle is not installed",
        "records of two naming styles",
    );
    assert_none_installed(&env, &names, "records of two naming styles");

    // The constant the derivation meant: callees of those names that are
    // there already — axioms of another type, the hierarchical `A.rec` and
    // the flat `"C.rec"` — are not the recursors the installed rules call.
    let mut env = env0.clone();
    axiom(&mut env, nm("A.rec"), cnst("B"));
    axiom(&mut env, Name::str("C.rec"), cnst("B"));
    assert_guard_refused(
        check_constant_infos_atomic(&mut env, mutual_group(&hierarchical, mixed)),
        "is not one",
        "callees of those names declared before",
    );
    assert_none_installed(&env, &names, "callees of those names declared before");

    // Either style throughout is the family as derived, and is installed.
    for (style, fam) in [("hierarchical", &hierarchical), ("flat", &flat)] {
        let mut env = env0.clone();
        if let Err(e) =
            check_constant_infos_atomic(&mut env, mutual_group(fam, fam.recursors.clone()))
        {
            panic!("the {style} family must be accepted: {e:?}");
        }
        for ci in &fam.recursors {
            assert!(env.contains(ci.name()), "{style}: {} installed", ci.name());
        }
    }
}

// ---------------------------------------------------------------------------
// The nested path
// ---------------------------------------------------------------------------

#[test]
fn a_nested_family_whose_recursor_names_a_sibling_it_does_not_install_is_refused() {
    // `N | mk : L N → N`: the kernel derives `N.rec` and the auxiliary
    // `N.rec_1` (for `L N`), and each one's rules call the other. The family
    // `verify_nested_bundle` returns is added by `add_derived_family`, after
    // the guard. A family without one of the two — an installed recursor
    // naming a missing sibling — is refused and leaves nothing behind.
    let env0 = base_env();
    let spec = n_spec(&[]);
    let recs = nested_with_records(&env0, &spec);
    let fam = verified_nested(&env0, &spec, &recs);
    let names: Vec<Name> = fam
        .clone()
        .into_constant_infos()
        .iter()
        .map(|ci| ci.name().clone())
        .collect();

    for dropped in ["N.rec_1", "N.rec"] {
        let mut env = env0.clone();
        let what = format!("{dropped} not installed");
        assert_guard_refused(
            add_derived_family(&mut env, without_recursor(&fam, dropped)),
            "not in the environment after the install",
            &what,
        );
        assert_none_installed(&env, &names, &what);
    }

    // The constant the derivation meant: an axiom `N.rec_1` declared before
    // is not the auxiliary recursor `N.rec`'s rule calls.
    let mut env = env0.clone();
    axiom(&mut env, nm("N.rec_1"), cnst("B"));
    assert_guard_refused(
        add_derived_family(&mut env, without_recursor(&fam, "N.rec_1")),
        "is not one",
        "an auxiliary recursor's name declared before",
    );
    assert!(!env.contains(&nm("N")), "N must not be installed");

    // The whole family is installed.
    let mut env = env0.clone();
    if let Err(e) = add_derived_family(&mut env, fam.into_constant_infos()) {
        panic!("the derived nested family must be installed: {e:?}");
    }
    for n in &names {
        assert!(env.contains(n), "{n} installed");
    }
}

#[test]
fn a_recursor_of_another_family_under_a_name_a_rule_calls_is_not_the_one_meant() {
    // `N.rec`'s rule for `N.mk` calls the auxiliary recursor `N.rec_1` by
    // name. Here the family is handed to the nested path's install WITHOUT
    // its own `N.rec_1`, and the name `N.rec_1` is already a RECURSOR in the
    // environment -- of another family: `L.rec` under that name, put there
    // with `Environment::add_constant`. A recursor is not enough: the
    // constant the rule calls must be a recursor of `N`'s family, and the
    // group is refused with nothing of it installed.
    let env0 = base_env();
    let spec = n_spec(&[]);
    let recs = nested_with_records(&env0, &spec);
    let fam = verified_nested(&env0, &spec, &recs);
    let group = without_recursor(&fam, "N.rec_1");
    let names: Vec<Name> = group.iter().map(|ci| ci.name().clone()).collect();

    let mut env = env0.clone();
    let mut foreign = env
        .get_recursor_val(&nm("L.rec"))
        .cloned()
        .unwrap_or_else(|| panic!("L.rec must be in the environment"));
    foreign.common.name = nm("N.rec_1");
    if let Err(e) = env.add_constant(ConstantInfo::Recursor(foreign)) {
        panic!("a constant N.rec_1 must be addable: {e:?}");
    }
    assert_guard_refused(
        add_derived_family(&mut env, group),
        "the recursor 'N.rec' the kernel derived names 'N.rec_1': the derivation names a recursor \
         of its family so, and the constant of that name is not one",
        "a recursor of another family under the name N.rec_1",
    );
    assert_none_installed(
        &env,
        &names,
        "a recursor of another family under the name N.rec_1",
    );
    assert!(
        matches!(env.get_recursor_val(&nm("N.rec_1")), Some(rv) if rv.all == vec![nm("L")]),
        "the recursor of L under the name N.rec_1 is left as it was"
    );
}

#[test]
fn a_derived_rule_keyed_by_a_constant_that_is_not_a_constructor_is_refused() {
    // Iota finds a recursor's rule by the constructor it is keyed by. The
    // family of `N` handed to the nested path's install with the rule of
    // `N.rec` keyed by `B` -- a constant of the environment, an inductive
    // type, not a constructor -- is refused, and nothing of it is
    // installed; the family as derived is installed.
    let env0 = base_env();
    let spec = n_spec(&[]);
    let recs = nested_with_records(&env0, &spec);
    let fam = verified_nested(&env0, &spec, &recs);
    let honest = fam.into_constant_infos();
    let names: Vec<Name> = honest.iter().map(|ci| ci.name().clone()).collect();

    let mut group = honest.clone();
    let mut keyed = 0;
    for ci in &mut group {
        if let ConstantInfo::Recursor(rv) = ci {
            if rv.common.name == nm("N.rec") {
                rv.rules[0].ctor = nm("B");
                keyed += 1;
            }
        }
    }
    assert_eq!(keyed, 1, "the family carries N.rec, with a rule");
    let mut env = env0.clone();
    assert_guard_refused(
        add_derived_family(&mut env, group),
        "the recursor 'N.rec' the kernel derived names 'B': a rule is keyed by it, and it is not a \
         constructor in the environment after the install",
        "a rule keyed by B",
    );
    assert_none_installed(&env, &names, "a rule keyed by B");

    let mut env = env0.clone();
    if let Err(e) = add_derived_family(&mut env, honest) {
        panic!("the family as derived must be installed: {e:?}");
    }
}

#[test]
fn the_nested_path_installs_the_derived_recursor_not_the_record() {
    // `N | mk : (b : B) → (h : P b) → L N → N`. The record of `N.rec` is the
    // derivation with ONE change, well typed: its rule for `N.mk` passes
    // `P.mk b` where the kernel's passes the field `h` — the kernel's rule up
    // to proof irrelevance. The checked comparison accepts it; what the
    // family carries, and what the environment then holds, is the kernel's
    // rule.
    let env0 = base_env();
    let spec = n_spec(&[("b", cnst("B")), ("h", app(cnst("P"), bv(0)))]);
    let honest = nested_with_records(&env0, &spec);
    let derived_rhs = honest[0].rules[0].rhs.clone();
    let mut records = honest.clone();
    // Under the lambdas: motives, minors, then the fields `b h kids`; the
    // body is `mk b h kids ih`, and `b` is the bound variable 2.
    records[0].rules[0].rhs = under_lambdas(&derived_rhs, &|body| {
        with_arg(body, 1, app(cnst("P.mk"), bv(2)))
    });
    let edited_rhs = records[0].rules[0].rhs.clone();
    assert!(
        !same_up_to_binders(&edited_rhs, &derived_rhs),
        "the record's rule is not the kernel's as written"
    );

    let fam = verified_nested(&env0, &spec, &records);
    let carried = fam
        .recursors
        .iter()
        .find_map(|ci| match ci {
            ConstantInfo::Recursor(rv) if rv.common.name == nm("N.rec") => Some(rv.clone()),
            _ => None,
        })
        .unwrap_or_else(|| panic!("the family carries N.rec"));
    assert!(
        same_up_to_binders(&carried.rules[0].rhs, &derived_rhs),
        "the family to install carries the kernel's rule"
    );

    let mut env = env0.clone();
    if let Err(e) = add_derived_family(&mut env, fam.into_constant_infos()) {
        panic!("the family must be installed: {e:?}");
    }
    let installed = env
        .get_recursor_val(&nm("N.rec"))
        .unwrap_or_else(|| panic!("N.rec must be in the environment"));
    assert!(
        same_up_to_binders(&installed.rules[0].rhs, &derived_rhs),
        "the environment holds the kernel's rule"
    );
    assert!(
        !same_up_to_binders(&installed.rules[0].rhs, &edited_rhs),
        "the environment does not hold the record's rule"
    );
}
