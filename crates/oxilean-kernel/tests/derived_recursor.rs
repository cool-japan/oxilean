//! What is installed is what the kernel checked, or what it derived.
//!
//! A term that arrives from outside is in the environment — and reaches
//! definitional equality, weak-head reduction or iota — only after the
//! kernel type-checked it. Where the kernel derives a term itself, the
//! derived term is what is installed, and a supplied declaration is only
//! held against it:
//!
//! * **A recursor.** `check_constant_info` adds the kernel's own recursor
//!   (its type, its rules, its counts, its `k` flag) under the supplied name
//!   and universe parameter names; the supplied type and rules are compared
//!   with it — as written, or type-checked first and then by definitional
//!   equality — and dropped. Before this, the supplied recursor was what
//!   was installed, after a comparison by definitional equality alone.
//!   Proof irrelevance equates two proofs whose types it reads off their
//!   heads, so a rule whose right-hand side carried an ILL-TYPED proof term
//!   passed, and at a constructor iota reduced to another value: a closed
//!   proof of `False` from no axiom (the export-level fixture `RecRuleJunk`
//!   is that proof; the first test below is its recursor).
//! * **The types of a block.** They are checked before any of them is a
//!   constant: a type of the block that mentions a type of the block names
//!   an unknown constant, for Lean's kernel and for this one.
//! * **A nested family.** What the nested derivation returns is the
//!   declared family: a declaration that names an auxiliary type of its own
//!   specialization is refused.
//!
//! The family used throughout:
//!
//! ```text
//! B : Type | t | f          P : B → Prop | mk : (b : B) → P b
//! T : Type | mk : (b : B) → (h : P b) → T
//! junk b := B.rec (motive := fun x => P x) (P.mk B.f) (P.mk B.f) b
//! ```
//!
//! `junk b` is ill typed — its first minor premise must have the type
//! `P B.t` and has the type `P B.f` — and its type, read off its head, is
//! `P b`.

use oxilean_kernel::declaration::{AxiomVal, ConstantInfo, ConstantVal, InductiveVal, RecursorVal};
use oxilean_kernel::instantiate::instantiate_type_lparams;
use oxilean_kernel::Node;
use oxilean_kernel::{
    add_inductive_family, check_and_derive_family, check_constant_info,
    check_constant_infos_atomic, same_up_to_binders, supplied_type_matches, verify_nested_bundle,
    BinderInfo, DerivedFamily, Environment, Expr, InductiveSpec, KernelError, Level, Name,
    TypeChecker,
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

fn pi(x: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(x),
        Node::new(dom),
        Node::new(body),
    )
}

fn lam(x: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(x),
        Node::new(dom),
        Node::new(body),
    )
}

fn bv(i: u32) -> Expr {
    Expr::BVar(i)
}

fn install(env: &mut Environment, lparams: &[&str], num_params: u32, specs: Vec<InductiveSpec>) {
    let lparams = lparams.iter().map(|p| Name::str(*p)).collect();
    if let Err(e) = add_inductive_family(env, lparams, num_params, specs) {
        panic!("the family must be accepted: {e:?}");
    }
}

fn axiom(env: &mut Environment, name: &str, ty: Expr) {
    let ci = ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: nm(name),
            level_params: vec![],
            ty,
        },
        is_unsafe: false,
    });
    if let Err(e) = check_constant_info(env, ci) {
        panic!("axiom {name} must be accepted: {e:?}");
    }
}

/// `B : Type | t | f` and `P : B → Prop | mk : (b : B) → P b`.
fn base_env() -> Environment {
    let mut env = Environment::new();
    install(
        &mut env,
        &[],
        0,
        vec![InductiveSpec::new(
            nm("B"),
            type0(),
            vec![(nm("B.t"), cnst("B")), (nm("B.f"), cnst("B"))],
        )],
    );
    install(
        &mut env,
        &[],
        0,
        vec![InductiveSpec::new(
            nm("P"),
            pi("b", cnst("B"), prop()),
            vec![(nm("P.mk"), pi("b", cnst("B"), app(cnst("P"), bv(0))))],
        )],
    );
    env
}

/// `T : Type | mk : (b : B) → (h : P b) → T`.
fn t_spec() -> InductiveSpec {
    InductiveSpec::new(
        nm("T"),
        type0(),
        vec![(
            nm("T.mk"),
            pi("b", cnst("B"), pi("h", app(cnst("P"), bv(0)), cnst("T"))),
        )],
    )
}

/// `Q : (b : B) → P b → Type | mk : (b : B) → (h : P b) → Q b h`: a family
/// with a PROOF index.
fn q_spec() -> InductiveSpec {
    InductiveSpec::new(
        nm("Q"),
        pi("b", cnst("B"), pi("h", app(cnst("P"), bv(0)), type0())),
        vec![(
            nm("Q.mk"),
            pi(
                "b",
                cnst("B"),
                pi(
                    "h",
                    app(cnst("P"), bv(0)),
                    apps(cnst("Q"), vec![bv(1), bv(0)]),
                ),
            ),
        )],
    )
}

fn derive(
    env: &Environment,
    lparams: &[&str],
    num_params: u32,
    spec: InductiveSpec,
) -> DerivedFamily {
    let lparams: Vec<Name> = lparams.iter().map(|p| Name::str(*p)).collect();
    match check_and_derive_family(env, &lparams, num_params, &[spec]) {
        Ok(fam) => fam,
        Err(e) => panic!("the family must be derivable: {e:?}"),
    }
}

/// The one recursor the kernel derives for a one-type family.
fn derived_recursor(fam: &DerivedFamily) -> RecursorVal {
    match fam.recursors.as_slice() {
        [ConstantInfo::Recursor(rv)] => rv.clone(),
        other => panic!("one recursor expected, got {}", other.len()),
    }
}

/// The family as a declaration from outside carries it: its types and
/// constructors, and the recursor `supplied` in place of the derived one.
fn group_with(fam: &DerivedFamily, supplied: RecursorVal) -> Vec<ConstantInfo> {
    let mut group = fam.inductives.clone();
    group.extend(fam.constructors.iter().cloned());
    group.push(ConstantInfo::Recursor(supplied));
    group
}

/// The ill-typed proof of `P b` for the `b` that is bound variable `i`.
fn junk(i: u32) -> Expr {
    apps(
        cnst_at("B.rec", vec![Level::zero()]),
        vec![
            lam("x", cnst("B"), app(cnst("P"), bv(0))),
            app(cnst("P.mk"), cnst("B.f")),
            app(cnst("P.mk"), cnst("B.f")),
            bv(i),
        ],
    )
}

/// `(fun (_ : Type) => e) (Prop Prop)` for a closed `e`: an ill-typed term
/// (a sort applied to a sort) that reduces to `e` in one step.
fn behind_an_ill_typed_redex(e: &Expr) -> Expr {
    app(lam("x", type0(), e.clone()), app(prop(), prop()))
}

/// `f` applied to what is under the leading binders of `e` (lambdas or
/// Pis), the binders kept.
fn under_binders(e: &Expr, f: &dyn Fn(&Expr) -> Expr) -> Expr {
    match e {
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            ty.clone(),
            Node::new(under_binders(body, f)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            ty.clone(),
            Node::new(under_binders(body, f)),
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

fn assert_refused_as_not_the_derived_one(res: Result<(), KernelError>, what: &str) {
    match res {
        Err(KernelError::InvalidRecursor(msg)) => assert!(
            msg.contains("does not match the kernel-derived recursor"),
            "{what}: {msg}"
        ),
        other => panic!("{what}: must be refused as not the derived recursor, got {other:?}"),
    }
}

fn installed_recursor<'a>(env: &'a Environment, name: &str) -> &'a RecursorVal {
    env.get_recursor_val(&nm(name))
        .unwrap_or_else(|| panic!("{name} must be in the environment"))
}

// ---------------------------------------------------------------------------
// The recursor
// ---------------------------------------------------------------------------

#[test]
fn a_supplied_rule_with_an_ill_typed_proof_term_is_refused() {
    // The rule of `T.mk` is `fun motive mk b h => mk b h`; the supplied one
    // is `fun motive mk b h => mk b (junk b)`. The two are equal for a
    // definitional equality that takes `junk b` for a proof of `P b`. While
    // that comparison was all there was, the supplied recursor was
    // installed, and `T.rec .. (T.mk B.t (P.mk B.t))` reduced through
    // `junk B.t` to `mk B.t (P.mk B.f)`: Lean's kernel never sees the rule
    // (it derives its own), and refuses what was proved with it.
    let mut env = base_env();
    let fam = derive(&env, &[], 0, t_spec());
    let mut supplied = derived_recursor(&fam);
    supplied.rules[0].rhs =
        under_binders(&supplied.rules[0].rhs, &|body| with_arg(body, 1, junk(1)));

    assert_refused_as_not_the_derived_one(
        check_constant_infos_atomic(&mut env, group_with(&fam, supplied)),
        "a rule with an ill-typed proof term",
    );
    for name in ["T", "T.mk", "T.rec"] {
        assert!(
            !env.contains(&nm(name)),
            "{name}: a refused family leaves nothing behind"
        );
    }
}

#[test]
fn the_recursor_that_is_added_is_the_derived_one_not_the_supplied_one() {
    // A WELL-TYPED rule that is the kernel's only up to proof irrelevance:
    // `fun motive mk b h => mk b (P.mk b)`. It is the derived recursor, by
    // the checked comparison, and the family is accepted — and what is in
    // the environment afterwards is the kernel's rule, `mk b h`.
    let mut env = base_env();
    let fam = derive(&env, &[], 0, t_spec());
    let derived = derived_recursor(&fam);
    let mut supplied = derived.clone();
    supplied.rules[0].rhs = under_binders(&derived.rules[0].rhs, &|body| {
        with_arg(body, 1, app(cnst("P.mk"), bv(1)))
    });
    assert!(
        !same_up_to_binders(&supplied.rules[0].rhs, &derived.rules[0].rhs),
        "the supplied rule is not the kernel's as written"
    );

    if let Err(e) = check_constant_infos_atomic(&mut env, group_with(&fam, supplied.clone())) {
        panic!("a well-typed rule equal to the kernel's must be accepted: {e:?}");
    }
    let installed = installed_recursor(&env, "T.rec");
    assert_eq!(
        installed, &derived,
        "the constant that is added is the derivation"
    );
    assert_ne!(installed.rules[0].rhs, supplied.rules[0].rhs);

    // Iota runs on the kernel's rule: the field comes back as it was given.
    axiom(&mut env, "h0", app(cnst("P"), cnst("B.t")));
    axiom(
        &mut env,
        "m",
        pi("b", cnst("B"), pi("h", app(cnst("P"), bv(0)), cnst("B"))),
    );
    let major = apps(cnst("T.mk"), vec![cnst("B.t"), cnst("h0")]);
    let e = apps(
        cnst_at("T.rec", vec![Level::succ(Level::zero())]),
        vec![lam("t", cnst("T"), cnst("B")), cnst("m"), major],
    );
    assert_eq!(
        TypeChecker::new(&env).whnf(&e),
        apps(cnst("m"), vec![cnst("B.t"), cnst("h0")]),
        "`T.rec m (T.mk B.t h0)` is `m B.t h0`, not `m B.t (P.mk B.t)`"
    );
}

#[test]
fn a_supplied_rule_is_type_checked_before_it_is_compared() {
    // `(fun _ => rhs) (Prop Prop)` reduces to the kernel's right-hand side
    // in one step, so definitional equality — which does not type-check
    // what it reduces — finds the two equal. The supplied term is ill typed
    // and is refused before that comparison is made.
    let mut env = base_env();
    let fam = derive(&env, &[], 0, t_spec());
    let derived = derived_recursor(&fam);
    let mut supplied = derived.clone();
    supplied.rules[0].rhs = behind_an_ill_typed_redex(&derived.rules[0].rhs);
    assert!(
        TypeChecker::new(&env).is_def_eq(&derived.rules[0].rhs, &supplied.rules[0].rhs),
        "the premise of the test: definitional equality alone accepts the term"
    );

    assert_refused_as_not_the_derived_one(
        check_constant_infos_atomic(&mut env, group_with(&fam, supplied)),
        "a rule behind an ill-typed redex",
    );
    assert!(!env.contains(&nm("T")));
}

#[test]
fn a_supplied_recursor_type_is_type_checked_before_it_is_compared() {
    // The recursor's TYPE is a term of the declaration too. In a family
    // with a proof index, `Q : (b : B) → P b → Type`, the conclusion of
    // `Q.rec` is `motive b a t`; written `motive b (junk b) t` it is equal
    // to the kernel's by proof irrelevance, and — installed — gave
    // `Q.rec (motive := fun b h _ => get b h = b) .. (Q.mk B.t (P.mk B.t))`
    // the type `B.f = B.t`.
    let mut env = base_env();
    let fam = derive(&env, &[], 0, q_spec());
    let derived = derived_recursor(&fam);

    // In the conclusion (under motive, mk, b, a, t: `b` is variable 2).
    let mut in_conclusion = derived.clone();
    in_conclusion.common.ty = under_binders(&derived.common.ty, &|body| with_arg(body, 1, junk(2)));
    assert_refused_as_not_the_derived_one(
        check_constant_infos_atomic(&mut env, group_with(&fam, in_conclusion)),
        "an ill-typed proof term in the conclusion of the recursor's type",
    );

    // The whole type behind an ill-typed redex.
    let mut behind_redex = derived.clone();
    behind_redex.common.ty = behind_an_ill_typed_redex(&derived.common.ty);
    assert_refused_as_not_the_derived_one(
        check_constant_infos_atomic(&mut env, group_with(&fam, behind_redex)),
        "a recursor type behind an ill-typed redex",
    );
    assert!(!env.contains(&nm("Q")));

    // The derived recursor itself is accepted, and is what is installed.
    if let Err(e) = check_constant_infos_atomic(&mut env, group_with(&fam, derived.clone())) {
        panic!("the derived recursor must be accepted: {e:?}");
    }
    assert_eq!(installed_recursor(&env, "Q.rec"), &derived);
}

#[test]
fn the_installed_recursor_keeps_the_supplied_universe_parameter_names() {
    // `Box.{w} (α : Sort w) : Sort (max 1 w) | mk : α → Box α`; the kernel
    // derives `Box.rec.{u, w}`. A declaration that calls the two `v` and
    // `w'` throughout is the same recursor — universe parameters are
    // instantiated by position — and is installed under those names. One
    // that names both `w` is not a declaration at all.
    let w = Level::param(Name::str("w"));
    let box_spec = || {
        InductiveSpec::new(
            nm("Box"),
            pi(
                "α",
                Expr::Sort(w.clone()),
                Expr::Sort(Level::max(Level::succ(Level::zero()), w.clone())),
            ),
            vec![(
                nm("Box.mk"),
                pi(
                    "α",
                    Expr::Sort(w.clone()),
                    pi("a", bv(0), app(cnst_at("Box", vec![w.clone()]), bv(1))),
                ),
            )],
        )
    };
    let mut env = Environment::new();
    let fam = derive(&env, &["w"], 1, box_spec());
    let derived = derived_recursor(&fam);
    assert_eq!(derived.common.level_params.len(), 2);

    let from = derived.common.level_params.clone();
    let to = vec![Name::str("v"), Name::str("w'")];
    let levels: Vec<Level> = to.iter().cloned().map(Level::param).collect();
    let mut renamed = derived.clone();
    renamed.common.level_params = to.clone();
    renamed.common.ty = instantiate_type_lparams(&derived.common.ty, &from, &levels);
    for rule in &mut renamed.rules {
        rule.rhs = instantiate_type_lparams(&rule.rhs, &from, &levels);
    }

    let mut twice = derived.clone();
    twice.common.level_params = vec![Name::str("w"), Name::str("w")];
    assert!(
        check_constant_infos_atomic(&mut env, group_with(&fam, twice)).is_err(),
        "a universe parameter name twice"
    );
    assert!(!env.contains(&nm("Box")));

    if let Err(e) = check_constant_infos_atomic(&mut env, group_with(&fam, renamed.clone())) {
        panic!("the derived recursor under other universe parameter names: {e:?}");
    }
    let installed = installed_recursor(&env, "Box.rec");
    assert_eq!(installed.common.level_params, to);
    assert_eq!(
        installed, &renamed,
        "the derivation, under the supplied names"
    );
}

// ---------------------------------------------------------------------------
// A type that arrives from outside, against a type the kernel made
// ---------------------------------------------------------------------------

#[test]
fn a_supplied_type_is_type_checked_before_it_is_compared() {
    let env = Environment::new();
    let kernel = pi("α", type0(), type0());
    // The same type with another binder name: the kernel's, as written.
    assert!(supplied_type_matches(
        &env,
        &kernel,
        &pi("β", type0(), type0())
    ));
    // A well-typed term that reduces to it: `(fun (x : Type 1) => x) (Type → Type)`.
    let type1 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let well_typed = app(lam("x", type1, bv(0)), kernel.clone());
    assert!(supplied_type_matches(&env, &kernel, &well_typed));
    // An ill-typed term that reduces to it: equal for definitional
    // equality, and not the kernel's type.
    let ill_typed = behind_an_ill_typed_redex(&kernel);
    assert!(TypeChecker::new(&env).is_def_eq(&kernel, &ill_typed));
    assert!(!supplied_type_matches(&env, &kernel, &ill_typed));
    // A term that is no type at all.
    assert!(!supplied_type_matches(
        &env,
        &kernel,
        &lam("x", type0(), bv(0))
    ));
}

// ---------------------------------------------------------------------------
// The types of a block
// ---------------------------------------------------------------------------

#[test]
fn the_types_of_a_block_are_checked_before_any_of_them_is_a_constant() {
    // mutual  C : Type | c0 ;  A : C → Type | mk : (c : C) → A c
    // Lean's kernel: "unknown constant 'C'" — the type of `A` is checked
    // before `C` is declared. Taken for an ordinary mutual family it would
    // be an inductive-inductive definition.
    let c_spec = || InductiveSpec::new(nm("C"), type0(), vec![(nm("C.c0"), cnst("C"))]);
    let a_spec = || {
        InductiveSpec::new(
            nm("A"),
            pi("c", cnst("C"), type0()),
            vec![(nm("A.mk"), pi("c", cnst("C"), app(cnst("A"), bv(0))))],
        )
    };
    let env = Environment::new();
    for specs in [vec![c_spec(), a_spec()], vec![a_spec(), c_spec()]] {
        match check_and_derive_family(&env, &[], 0, &specs) {
            Err(KernelError::InvalidInductive(msg)) => assert!(
                msg.contains("'A'") && msg.contains("mentions a type of the block"),
                "{msg}"
            ),
            other => panic!("a block whose type mentions the block: {other:?}"),
        }
    }
    // The same block one constant at a time (`check_constant_info`): `C` is
    // in the environment by the time the type of `A` is looked at, and `A`
    // is refused all the same.
    let mut env = Environment::new();
    let block = vec![nm("C"), nm("A")];
    let inductive = |name: &str, ty: Expr, ctor: &str, num_indices: u32| {
        ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: nm(name),
                level_params: vec![],
                ty,
            },
            num_params: 0,
            num_indices,
            all: block.clone(),
            ctors: vec![nm(ctor)],
            num_nested: 0,
            is_rec: true,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        })
    };
    if let Err(e) = check_constant_info(&mut env, inductive("C", type0(), "C.c0", 0)) {
        panic!("`C : Type` by itself is a type of the block that mentions none: {e:?}");
    }
    match check_constant_info(
        &mut env,
        inductive("A", pi("c", cnst("C"), type0()), "A.mk", 1),
    ) {
        Err(KernelError::InvalidInductive(msg)) => {
            assert!(msg.contains("mentions a type of the block"), "{msg}")
        }
        other => panic!("`A : C → Type` in the block of `C`: {other:?}"),
    }
    // Declared one after the other, the two are ordinary Lean.
    let mut env = Environment::new();
    install(&mut env, &[], 0, vec![c_spec()]);
    install(&mut env, &[], 0, vec![a_spec()]);
    assert!(env.contains(&nm("A.rec")));
}

// ---------------------------------------------------------------------------
// The declaration itself: a constructor's type and an index's type
// ---------------------------------------------------------------------------

#[test]
fn an_ill_typed_proof_term_in_a_constructor_type_or_an_index_type_is_refused() {
    // The same `junk b` where the declaration states it: in the index of a
    // constructor's result type (`Q2.mk : (b : B) → Q2 b (junk b)`) and in
    // the type of an index of an inductive type (`Q3 : (b : B) →
    // Q b (junk b) → Type`). Unlike a recursor these are the file's terms
    // and are installed as written, so they must be type-checked before
    // anything else looks at them; Lean's kernel refuses both. The
    // well-typed relatives (`h`, `P.mk b` in place of `junk b`) are accepted.
    let q2 = |index: Expr| {
        InductiveSpec::new(
            nm("Q2"),
            pi("b", cnst("B"), pi("h", app(cnst("P"), bv(0)), type0())),
            vec![(
                nm("Q2.mk"),
                pi("b", cnst("B"), apps(cnst("Q2"), vec![bv(0), index])),
            )],
        )
    };
    let q3 = |index: Expr| {
        InductiveSpec::new(
            nm("Q3"),
            pi(
                "b",
                cnst("B"),
                pi("q", apps(cnst("Q"), vec![bv(0), index]), type0()),
            ),
            vec![],
        )
    };
    let mut env = base_env();
    install(&mut env, &[], 0, vec![q_spec()]);
    let p_mk_b = app(cnst("P.mk"), bv(0));
    for (what, junk_spec, honest_spec) in [
        ("a constructor's type", q2(junk(0)), q2(p_mk_b.clone())),
        ("the type of an index", q3(junk(0)), q3(p_mk_b.clone())),
    ] {
        let name = junk_spec.name.clone();
        assert!(
            check_and_derive_family(&env, &[], 0, std::slice::from_ref(&junk_spec)).is_err(),
            "{what}: an ill-typed proof term must be refused"
        );
        let mut work = env.clone();
        assert!(
            add_inductive_family(&mut work, vec![], 0, vec![junk_spec]).is_err(),
            "{what}: an ill-typed proof term must be refused when the family is added"
        );
        assert!(!work.contains(&name), "{what}: nothing of it is added");
        let mut work = env.clone();
        if let Err(e) = add_inductive_family(&mut work, vec![], 0, vec![honest_spec]) {
            panic!("{what}: the well-typed relative must be accepted: {e:?}");
        }
        assert!(work.contains(&name));
    }
}

// ---------------------------------------------------------------------------
// A nested family
// ---------------------------------------------------------------------------

#[test]
fn a_nested_declaration_may_not_name_an_auxiliary_type_of_its_own_nesting() {
    // L α | nil | cons : α → L α → L α ;  N | a : L N → N | b : L N → N.
    // While the nested family is checked, the occurrence `L N` is the
    // auxiliary type `N._nested_0`. A declaration that WRITES that name —
    // `b : N._nested_0 → N` — type-checks there, and restoration would
    // install it as `b : L N → N`. Lean's kernel: "unknown constant".
    let mut env = Environment::new();
    let l = |arg: Expr| app(cnst("L"), arg);
    install(
        &mut env,
        &[],
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
    let n_spec = |b_field: Expr| {
        InductiveSpec::new(
            nm("N"),
            type0(),
            vec![
                (nm("N.a"), pi("l", l(cnst("N")), cnst("N"))),
                (nm("N.b"), pi("l", b_field, cnst("N"))),
            ],
        )
    };
    // With no recursor supplied, the honest declaration gets as far as the
    // recursors: its family is the declared one.
    match verify_nested_bundle(&env, &[], 0, &[n_spec(l(cnst("N")))], &[]) {
        Err(KernelError::InvalidRecursor(msg)) => {
            assert!(msg.contains("derived 2 recursors"), "{msg}")
        }
        other => panic!("the honest nested declaration: {other:?}"),
    }
    // The one that names the auxiliary type does not.
    match verify_nested_bundle(&env, &[], 0, &[n_spec(cnst("N._nested_0"))], &[]) {
        Err(KernelError::InvalidInductive(msg)) => assert!(
            msg.contains("'N.b'") && msg.contains("auxiliary type of its own nesting"),
            "{msg}"
        ),
        other => panic!("a declaration that names its auxiliary type: {other:?}"),
    }
}
