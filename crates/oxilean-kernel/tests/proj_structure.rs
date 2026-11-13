//! Which inductive type a projection may name, and how its operand's type
//! must look (Lean 4 kernel `infer_proj`), driven through
//! `TypeChecker::infer_type`, `check_constant_info` and
//! `DefEqChecker::is_def_eq` — the paths a declaration is checked on.
//!
//! `Proj(S, i, e)` is typed for every inductive type `S` with exactly one
//! constructor: `S` may be recursive (an ordinary recursive `structure`),
//! may have indices, and may sit in a mutual block. The operand's type must
//! be `S` applied to its parameters and indices, and only the parameters
//! instantiate the constructor's telescope. Structure eta, eta inside iota
//! and unit-like equality are narrower — one constructor, no indices, not
//! recursive — and stay where they were.
//!
//! Every inductive type below is built by `add_inductive_family`, so
//! `is_rec`, `num_indices` and the recursors are the kernel's own
//! derivation. Every verdict is the one Lean 4's kernel (v4.32.0-rc1) gives
//! on the same term over the same kind of type (measured 2026-11-12); the
//! export-level counterpart is the fixture
//! `tests/fixtures/lean4export/ProjRecIndexed.ndjson`.

use oxilean_kernel::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, DefinitionSafety, DefinitionVal,
};
use oxilean_kernel::Node;
use oxilean_kernel::{
    add_inductive_family, check_constant_info, BinderInfo, DefEqChecker, Environment, Expr,
    InductiveSpec, KernelError, Level, Name, ReducibilityHint, TypeChecker,
};

// ---------------------------------------------------------------------------
// Expression-building helpers
// ---------------------------------------------------------------------------

fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

fn lvl1() -> Level {
    Level::succ(Level::zero())
}

fn type0() -> Expr {
    Expr::Sort(lvl1())
}

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn cnst_lvls(name: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::str(name), levels)
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

fn apps(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, app)
}

fn pi(name: &str, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Node::new(dom),
        Node::new(cod),
    )
}

fn lam(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str(name),
        Node::new(dom),
        Node::new(body),
    )
}

fn proj(struct_name: &str, idx: u32, e: Expr) -> Expr {
    Expr::Proj(Name::str(struct_name), idx, Node::new(e))
}

fn bvar(i: u32) -> Expr {
    Expr::BVar(i)
}

// ---------------------------------------------------------------------------
// Environment-building helpers
// ---------------------------------------------------------------------------

fn common(name: &str, ty: Expr) -> ConstantVal {
    ConstantVal {
        name: Name::str(name),
        level_params: vec![],
        ty,
    }
}

fn add_axiom(env: &mut Environment, name: &str, ty: Expr) {
    env.add_constant(ConstantInfo::Axiom(AxiomVal {
        common: common(name, ty),
        is_unsafe: false,
    }))
    .expect("add axiom");
}

/// One member of an inductive family; its recursor is named `<name>.rec`
/// as one flat atom, like every other name here.
fn spec(name: &str, ty: Expr, ctors: Vec<(&str, Expr)>) -> InductiveSpec {
    InductiveSpec::new(
        Name::str(name),
        ty,
        ctors
            .into_iter()
            .map(|(ctor, ctor_ty)| (Name::str(ctor), ctor_ty))
            .collect(),
    )
    .with_rec_name(Name::str(format!("{name}.rec")))
}

/// Check a family and install what the kernel derives for it.
fn family(
    env: &mut Environment,
    level_params: &[&str],
    num_params: u32,
    specs: Vec<InductiveSpec>,
) {
    let what: Vec<String> = specs.iter().map(|s| s.name.to_string()).collect();
    add_inductive_family(
        env,
        level_params.iter().map(|p| Name::str(*p)).collect(),
        num_params,
        specs,
    )
    .unwrap_or_else(|e| panic!("the family {what:?} must be accepted: {e}"));
}

/// A definition, through the declaration-checking path.
fn definition(name: &str, ty: Expr, value: Expr) -> ConstantInfo {
    ConstantInfo::Definition(DefinitionVal {
        common: common(name, ty),
        value,
        hints: ReducibilityHint::Abbrev,
        safety: DefinitionSafety::Safe,
        all: vec![Name::str(name)],
    })
}

/// `fun _ : N => T`.
fn const_t() -> Expr {
    lam("_n", cnst("N"), cnst("T"))
}

/// The environment of every test. Base constants: `N B : Type` (data) with
/// `n0 n1 : N` and `b0 : B`; `T : Prop` with `t0 : T`; `F : N → Type` with
/// `f1 : F n1`; `G : T → Prop`.
///
/// In `Type`:
/// * `Pair` with `Pair.mk : N → B → Pair` — structure-like, the control;
/// * `Str` with `Str.cons : N → Str → Str` — recursive;
/// * `I1 : N → Type` with `I1.mk : B → I1 n0` — one index;
/// * `PI (α : Type) : N → Type` with `PI.mk : (a : α) → (n : N) → PI α n`
///   — a parameter and an index;
/// * `PIq (q : Prop) : N → Type` with `PIq.mk : (h : q) → (n : N) → PIq q n`
///   — the same with a proof field whose type is the parameter;
/// * `DepI : N → Type` with `DepI.mk : (n : N) → (f : F n) → DepI n` — a
///   later field whose type depends on an earlier one;
/// * `PIu.{u} (α : Sort u) : N → Sort (max 1 u)` with
///   `PIu.mk : (a : α) → PIu α n0` — universe-polymorphic;
/// * `U1 : N → Type` with `U1.mk : U1 n0` — an index and no field;
/// * `MA` / `MB`, mutual: `MA.mk : N → MB → MA`; `MB.cons : MA → MB → MB`,
///   `MB.nil`;
/// * `Two` with two constructors, `Two.b : T → Two` and `Two.a` (the FIRST
///   constructor of `Two` and of `MB` has a field 0: a checker that read
///   the first of several constructors would type the projection);
/// * `Emp` with none.
///
/// In `Prop`:
/// * `RP` with `RP.mk : RP → RP` — recursive, a proof field;
/// * `RPD` with `RPD.mk : N → RPD → RPD` — recursive, a data field;
/// * `IPp : N → Prop` with `IPp.mk : T → IPp n0` — indexed, a proof field;
/// * `IPd : N → Prop` with `IPd.mk : (n : N) → IPd n` — indexed, a data
///   field that is the index (a syntactic subsingleton: its recursor
///   eliminates into `Type`, its projection is still refused);
/// * `IEx : N → Prop` with `IEx.mk : (i : N) → (n : N) → IEx i` — an
///   existential with an index;
/// * `IPdep (q : N → Prop) : N → Prop` with
///   `IPdep.mk : (n : N) → (h : q n) → IPdep q n` — a proof field whose
///   type depends on a data field;
/// * `IPpp : N → Prop` with `IPpp.mk : (h : T) → (g : G h) → IPpp n0` — a
///   proof field whose type depends on a proof field;
/// * `RPdep (q : N → Prop) : Prop` with
///   `RPdep.mk : (n : N) → (h : q n) → (r : RPdep q) → RPdep q` — recursive,
///   with a proof field whose type depends on a data field.
///
/// And one operand of each, a definition `StrAlias := Str` with
/// `sa : StrAlias`, and `fs : N → Str`.
fn env() -> Environment {
    let mut env = Environment::new();
    let u = Level::param(Name::str("u"));
    add_axiom(&mut env, "N", type0());
    add_axiom(&mut env, "n0", cnst("N"));
    add_axiom(&mut env, "n1", cnst("N"));
    add_axiom(&mut env, "B", type0());
    add_axiom(&mut env, "b0", cnst("B"));
    add_axiom(&mut env, "T", prop());
    add_axiom(&mut env, "t0", cnst("T"));
    add_axiom(&mut env, "F", pi("_n", cnst("N"), type0()));
    add_axiom(&mut env, "f1", app(cnst("F"), cnst("n1")));
    add_axiom(&mut env, "G", pi("_h", cnst("T"), prop()));
    let n_to_type = || pi("_i", cnst("N"), type0());
    let n_to_prop = || pi("_i", cnst("N"), prop());

    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "Pair",
            type0(),
            vec![(
                "Pair.mk",
                pi("fst", cnst("N"), pi("snd", cnst("B"), cnst("Pair"))),
            )],
        )],
    );
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "Str",
            type0(),
            vec![(
                "Str.cons",
                pi("head", cnst("N"), pi("tail", cnst("Str"), cnst("Str"))),
            )],
        )],
    );
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "I1",
            n_to_type(),
            vec![("I1.mk", pi("b", cnst("B"), app(cnst("I1"), cnst("n0"))))],
        )],
    );
    // PI : (α : Type) → N → Type;  PI.mk : (α : Type) → (a : α) → (n : N) → PI α n
    family(
        &mut env,
        &[],
        1,
        vec![spec(
            "PI",
            pi("alpha", type0(), n_to_type()),
            vec![(
                "PI.mk",
                pi(
                    "alpha",
                    type0(),
                    pi(
                        "a",
                        bvar(0),
                        pi("n", cnst("N"), apps(cnst("PI"), vec![bvar(2), bvar(0)])),
                    ),
                ),
            )],
        )],
    );
    // PIq : (q : Prop) → N → Type;  PIq.mk : (q : Prop) → (h : q) → (n : N) → PIq q n
    family(
        &mut env,
        &[],
        1,
        vec![spec(
            "PIq",
            pi("q", prop(), n_to_type()),
            vec![(
                "PIq.mk",
                pi(
                    "q",
                    prop(),
                    pi(
                        "h",
                        bvar(0),
                        pi("n", cnst("N"), apps(cnst("PIq"), vec![bvar(2), bvar(0)])),
                    ),
                ),
            )],
        )],
    );
    // DepI.mk : (n : N) → (f : F n) → DepI n
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "DepI",
            n_to_type(),
            vec![(
                "DepI.mk",
                pi(
                    "n",
                    cnst("N"),
                    pi("f", app(cnst("F"), bvar(0)), app(cnst("DepI"), bvar(1))),
                ),
            )],
        )],
    );
    // PIu.{u} : (α : Sort u) → N → Sort (max 1 u);  PIu.mk : (α : Sort u) → (a : α) → PIu α n0
    family(
        &mut env,
        &["u"],
        1,
        vec![spec(
            "PIu",
            pi(
                "alpha",
                Expr::Sort(u.clone()),
                pi("_i", cnst("N"), Expr::Sort(Level::max(lvl1(), u.clone()))),
            ),
            vec![(
                "PIu.mk",
                pi(
                    "alpha",
                    Expr::Sort(u.clone()),
                    pi(
                        "a",
                        bvar(0),
                        apps(cnst_lvls("PIu", vec![u.clone()]), vec![bvar(1), cnst("n0")]),
                    ),
                ),
            )],
        )],
    );
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "U1",
            n_to_type(),
            vec![("U1.mk", app(cnst("U1"), cnst("n0")))],
        )],
    );
    family(
        &mut env,
        &[],
        0,
        vec![
            spec(
                "MA",
                type0(),
                vec![("MA.mk", pi("n", cnst("N"), pi("b", cnst("MB"), cnst("MA"))))],
            ),
            spec(
                "MB",
                type0(),
                vec![
                    (
                        "MB.cons",
                        pi("a", cnst("MA"), pi("b", cnst("MB"), cnst("MB"))),
                    ),
                    ("MB.nil", cnst("MB")),
                ],
            ),
        ],
    );
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "Two",
            type0(),
            vec![
                ("Two.b", pi("h", cnst("T"), cnst("Two"))),
                ("Two.a", cnst("Two")),
            ],
        )],
    );
    family(&mut env, &[], 0, vec![spec("Emp", type0(), vec![])]);

    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "RP",
            prop(),
            vec![("RP.mk", pi("h", cnst("RP"), cnst("RP")))],
        )],
    );
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "RPD",
            prop(),
            vec![(
                "RPD.mk",
                pi("n", cnst("N"), pi("h", cnst("RPD"), cnst("RPD"))),
            )],
        )],
    );
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "IPp",
            n_to_prop(),
            vec![("IPp.mk", pi("h", cnst("T"), app(cnst("IPp"), cnst("n0"))))],
        )],
    );
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "IPd",
            n_to_prop(),
            vec![("IPd.mk", pi("n", cnst("N"), app(cnst("IPd"), bvar(0))))],
        )],
    );
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "IEx",
            n_to_prop(),
            vec![(
                "IEx.mk",
                pi(
                    "i",
                    cnst("N"),
                    pi("n", cnst("N"), app(cnst("IEx"), bvar(1))),
                ),
            )],
        )],
    );
    // IPdep : (q : N → Prop) → N → Prop
    // IPdep.mk : (q : N → Prop) → (n : N) → (h : q n) → IPdep q n
    family(
        &mut env,
        &[],
        1,
        vec![spec(
            "IPdep",
            pi("q", n_to_prop(), n_to_prop()),
            vec![(
                "IPdep.mk",
                pi(
                    "q",
                    n_to_prop(),
                    pi(
                        "n",
                        cnst("N"),
                        pi(
                            "h",
                            app(bvar(1), bvar(0)),
                            apps(cnst("IPdep"), vec![bvar(2), bvar(1)]),
                        ),
                    ),
                ),
            )],
        )],
    );
    // RPdep : (q : N → Prop) → Prop
    // RPdep.mk : (q : N → Prop) → (n : N) → (h : q n) → (r : RPdep q) → RPdep q
    family(
        &mut env,
        &[],
        1,
        vec![spec(
            "RPdep",
            pi("q", n_to_prop(), prop()),
            vec![(
                "RPdep.mk",
                pi(
                    "q",
                    n_to_prop(),
                    pi(
                        "n",
                        cnst("N"),
                        pi(
                            "h",
                            app(bvar(1), bvar(0)),
                            pi(
                                "r",
                                app(cnst("RPdep"), bvar(2)),
                                app(cnst("RPdep"), bvar(3)),
                            ),
                        ),
                    ),
                ),
            )],
        )],
    );
    // IPpp.mk : (h : T) → (g : G h) → IPpp n0
    family(
        &mut env,
        &[],
        0,
        vec![spec(
            "IPpp",
            n_to_prop(),
            vec![(
                "IPpp.mk",
                pi(
                    "h",
                    cnst("T"),
                    pi("g", app(cnst("G"), bvar(0)), app(cnst("IPpp"), cnst("n0"))),
                ),
            )],
        )],
    );

    add_axiom(&mut env, "pr", cnst("Pair"));
    add_axiom(&mut env, "st", cnst("Str"));
    add_axiom(&mut env, "v0", app(cnst("I1"), cnst("n0")));
    add_axiom(&mut env, "v1", app(cnst("I1"), cnst("n1")));
    add_axiom(
        &mut env,
        "pv",
        apps(cnst("PI"), vec![cnst("B"), cnst("n1")]),
    );
    add_axiom(
        &mut env,
        "pq",
        apps(cnst("PIq"), vec![cnst("T"), cnst("n1")]),
    );
    add_axiom(&mut env, "dv", app(cnst("DepI"), cnst("n1")));
    add_axiom(
        &mut env,
        "pu",
        apps(cnst_lvls("PIu", vec![lvl1()]), vec![cnst("N"), cnst("n1")]),
    );
    add_axiom(
        &mut env,
        "pp",
        apps(
            cnst_lvls("PIu", vec![Level::zero()]),
            vec![cnst("T"), cnst("n0")],
        ),
    );
    add_axiom(&mut env, "u1a", app(cnst("U1"), cnst("n0")));
    add_axiom(&mut env, "u1b", app(cnst("U1"), cnst("n0")));
    add_axiom(&mut env, "ma", cnst("MA"));
    add_axiom(&mut env, "mb", cnst("MB"));
    add_axiom(&mut env, "two", cnst("Two"));
    add_axiom(&mut env, "emp", cnst("Emp"));
    add_axiom(&mut env, "rp", cnst("RP"));
    add_axiom(&mut env, "rpd", cnst("RPD"));
    add_axiom(&mut env, "ipp", app(cnst("IPp"), cnst("n0")));
    add_axiom(&mut env, "ipd", app(cnst("IPd"), cnst("n1")));
    add_axiom(&mut env, "iex", app(cnst("IEx"), cnst("n0")));
    add_axiom(
        &mut env,
        "ipdep",
        apps(cnst("IPdep"), vec![const_t(), cnst("n0")]),
    );
    add_axiom(&mut env, "ippp", app(cnst("IPpp"), cnst("n0")));
    add_axiom(&mut env, "rpdep", app(cnst("RPdep"), const_t()));
    add_axiom(&mut env, "fs", pi("_n", cnst("N"), cnst("Str")));
    check_constant_info(&mut env, definition("StrAlias", type0(), cnst("Str")))
        .expect("StrAlias := Str");
    add_axiom(&mut env, "sa", cnst("StrAlias"));
    env
}

/// What the tests rely on: the kernel's own derivation says which of the
/// types are recursive and which have indices, and none of them is
/// structure-like except the control.
#[test]
fn the_environment_is_what_the_tests_say() {
    let env = env();
    let facts = |name: &str| {
        let iv = env
            .get_inductive_val(&Name::str(name))
            .unwrap_or_else(|| panic!("{name} is an inductive type"));
        (iv.num_params, iv.num_indices, iv.is_rec, iv.ctors.len())
    };
    assert_eq!(facts("Pair"), (0, 0, false, 1));
    assert_eq!(facts("Str"), (0, 0, true, 1));
    assert_eq!(facts("I1"), (0, 1, false, 1));
    assert_eq!(facts("PI"), (1, 1, false, 1));
    assert_eq!(facts("PIq"), (1, 1, false, 1));
    assert_eq!(facts("DepI"), (0, 1, false, 1));
    assert_eq!(facts("PIu"), (1, 1, false, 1));
    assert_eq!(facts("U1"), (0, 1, false, 1));
    assert_eq!(facts("MA"), (0, 0, true, 1));
    assert_eq!(facts("MB"), (0, 0, true, 2));
    assert_eq!(facts("Two"), (0, 0, false, 2));
    assert_eq!(facts("Emp"), (0, 0, false, 0));
    assert_eq!(facts("RP"), (0, 0, true, 1));
    assert_eq!(facts("RPD"), (0, 0, true, 1));
    assert_eq!(facts("IPp"), (0, 1, false, 1));
    assert_eq!(facts("IPd"), (0, 1, false, 1));
    assert_eq!(facts("IEx"), (0, 1, false, 1));
    assert_eq!(facts("IPdep"), (1, 1, false, 1));
    assert_eq!(facts("IPpp"), (0, 1, false, 1));
    assert_eq!(facts("RPdep"), (1, 0, true, 1));
    assert!(env.is_structure_like(&Name::str("Pair")));
    for name in [
        "Str", "I1", "PI", "PIq", "DepI", "PIu", "U1", "MA", "RP", "RPD", "IPp", "IPd", "IEx",
        "IPdep", "IPpp", "RPdep",
    ] {
        assert!(
            !env.is_structure_like(&Name::str(name)),
            "{name} has one constructor and is NOT structure-like: no eta for it"
        );
    }
}

/// A projection the rule on its target or its operand refuses.
fn assert_invalid(result: Result<Expr, KernelError>, what: &str) {
    match result {
        Err(e) => {
            let text = e.to_string();
            assert!(
                text.contains("invalid projection") || text.contains("out of range"),
                "{what}: refused, but not as a projection: {text}"
            );
        }
        Ok(ty) => panic!("{what}: a projection Lean's kernel refuses was given the type {ty}"),
    }
}

/// A projection the side condition on propositions refuses.
fn assert_no_data_out_of_a_proof(result: Result<Expr, KernelError>, what: &str) {
    match result {
        Err(e) => {
            let text = e.to_string();
            assert!(
                text.contains("invalid projection") && text.contains("proposition"),
                "{what}: refused, but not by the side condition on propositions: {text}"
            );
        }
        Ok(ty) => panic!("{what}: data projected out of a proof was given the type {ty}"),
    }
}

// ---------------------------------------------------------------------------
// Accepted: one constructor is all a projection asks of the inductive type
// ---------------------------------------------------------------------------

/// `st.0 : N`, `st.1 : Str`, `st.1.1.0 : N` — a recursive inductive type
/// (what an ordinary recursive `structure` is).
#[test]
fn the_fields_of_a_recursive_inductive_are_typed() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let head = tc
        .infer_type(&proj("Str", 0, cnst("st")))
        .expect("Str has one constructor: its field 0 is typed");
    assert_eq!(head, cnst("N"));
    let tail = tc
        .infer_type(&proj("Str", 1, cnst("st")))
        .expect("Str has one constructor: its field 1 is typed");
    assert_eq!(tail, cnst("Str"));
    let deep = proj("Str", 0, proj("Str", 1, proj("Str", 1, cnst("st"))));
    assert_eq!(tc.infer_type(&deep).expect("st.1.1.0"), cnst("N"));
}

/// `v0.0 : B` for `v0 : I1 n0`, and `v1.0 : B` for `v1 : I1 n1` — the family
/// at another index than the one the constructor builds.
#[test]
fn the_field_of_an_indexed_family_is_typed_at_every_index() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let at_ctor_index = tc
        .infer_type(&proj("I1", 0, cnst("v0")))
        .expect("I1 has one constructor: its field is typed");
    assert_eq!(at_ctor_index, cnst("B"));
    let at_other_index = tc
        .infer_type(&proj("I1", 0, cnst("v1")))
        .expect("the index of the operand's type plays no part");
    assert_eq!(at_other_index, cnst("B"));
}

/// `pv.0 : B` and `pv.1 : N` for `pv : PI B n1`. The constructor's telescope
/// is `(α : Type) (a : α) (n : N)`: instantiated with the parameter `B`
/// alone, field 0 is `B`. Read with the index as a second constructor
/// argument, field 0 would be `N`.
#[test]
fn only_the_parameters_instantiate_the_constructor() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let val = tc
        .infer_type(&proj("PI", 0, cnst("pv")))
        .expect("PI field 0 is typed");
    assert_eq!(val, cnst("B"));
    let tag = tc
        .infer_type(&proj("PI", 1, cnst("pv")))
        .expect("PI field 1 is typed");
    assert_eq!(tag, cnst("N"));
}

/// `dv.1 : F dv.0` for `dv : DepI n1` — the type of the later field
/// mentions the earlier FIELD, never the index `n1`, although the two are
/// provably equal.
#[test]
fn a_later_field_mentions_the_earlier_field_and_not_the_index() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_eq!(
        tc.infer_type(&proj("DepI", 0, cnst("dv"))).expect("dv.0"),
        cnst("N")
    );
    let f = tc
        .infer_type(&proj("DepI", 1, cnst("dv")))
        .expect("DepI field 1 is typed");
    assert_eq!(f, app(cnst("F"), proj("DepI", 0, cnst("dv"))));
    assert!(
        !tc.is_def_eq(&f, &app(cnst("F"), cnst("n1"))),
        "`F dv.0` is not `F n1` by definition"
    );
}

/// `pu.0 : N` for `pu : PIu.{1} N n1` and `pp.0 : T` for
/// `pp : PIu.{0} T n0` — the universe levels of the operand's type
/// instantiate the constructor. `PIu.{0} T n0` is a TYPE whose parameter is
/// a proposition: its field is a proof and no side condition applies.
#[test]
fn a_universe_polymorphic_indexed_family_is_typed() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let data = tc
        .infer_type(&proj("PIu", 0, cnst("pu")))
        .expect("PIu.{1} N n1 field 0");
    assert_eq!(data, cnst("N"));
    let proof = tc
        .infer_type(&proj("PIu", 0, cnst("pp")))
        .expect("PIu.{0} T n0 field 0");
    assert_eq!(proof, cnst("T"));
}

/// The proof fields of recursive and of indexed propositions: `rp.0 : RP`,
/// `rpd.1 : RPD` (after a data field it does not depend on), `ipp.0 : T`,
/// `ippp.0 : T` and `ippp.1 : G ippp.0` (a proof field whose type depends on
/// a PROOF field).
#[test]
fn the_proof_fields_of_recursive_and_indexed_propositions_are_typed() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_eq!(
        tc.infer_type(&proj("RP", 0, cnst("rp"))).expect("rp.0"),
        cnst("RP")
    );
    assert_eq!(
        tc.infer_type(&proj("RPD", 1, cnst("rpd"))).expect("rpd.1"),
        cnst("RPD")
    );
    assert_eq!(
        tc.infer_type(&proj("IPp", 0, cnst("ipp"))).expect("ipp.0"),
        cnst("T")
    );
    assert_eq!(
        tc.infer_type(&proj("IPpp", 0, cnst("ippp")))
            .expect("ippp.0"),
        cnst("T")
    );
    assert_eq!(
        tc.infer_type(&proj("IPpp", 1, cnst("ippp")))
            .expect("ippp.1"),
        app(cnst("G"), proj("IPpp", 0, cnst("ippp")))
    );
}

/// `ma.0 : N` and `ma.1 : MB` — one constructor inside a mutual block.
#[test]
fn one_constructor_inside_a_mutual_block_is_typed() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_eq!(
        tc.infer_type(&proj("MA", 0, cnst("ma"))).expect("ma.0"),
        cnst("N")
    );
    assert_eq!(
        tc.infer_type(&proj("MA", 1, cnst("ma"))).expect("ma.1"),
        cnst("MB")
    );
}

/// `sa.0 : N` for `sa : StrAlias` — the operand's type only REDUCES to the
/// family.
#[test]
fn an_operand_whose_type_reduces_to_the_family_is_typed() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_eq!(
        tc.infer_type(&proj("Str", 0, cnst("sa"))).expect("sa.0"),
        cnst("N")
    );
}

/// The declaration-checking path: `head : Str → N := fun s => s.0` and
/// `field : (n : N) → I1 n → B := fun n v => v.0` are accepted and
/// installed — what the generated projection of a recursive `structure` is.
#[test]
fn projection_functions_are_accepted_as_declarations() {
    let mut env = env();
    let head = definition(
        "head",
        pi("s", cnst("Str"), cnst("N")),
        lam("s", cnst("Str"), proj("Str", 0, bvar(0))),
    );
    check_constant_info(&mut env, head).expect("fun s => s.0 is a function Str → N");
    assert!(env.find(&Name::str("head")).is_some());
    let field = definition(
        "field",
        pi("n", cnst("N"), pi("v", app(cnst("I1"), bvar(0)), cnst("B"))),
        lam(
            "n",
            cnst("N"),
            lam("v", app(cnst("I1"), bvar(0)), proj("I1", 0, bvar(0))),
        ),
    );
    check_constant_info(&mut env, field).expect("fun n v => v.0 is a function (n : N) → I1 n → B");
    assert!(env.find(&Name::str("field")).is_some());
}

/// A projection of a constructor application reduces to the field, for a
/// recursive type and for an indexed family alike — also inside a type:
/// `(DepI.mk n1 f1).1 : F (DepI.mk n1 f1).0`, which is `F n1`.
#[test]
fn a_projection_of_a_constructor_application_reduces() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    let cons = apps(cnst("Str.cons"), vec![cnst("n0"), cnst("st")]);
    assert!(tc.is_def_eq(&proj("Str", 0, cons.clone()), &cnst("n0")));
    assert!(tc.is_def_eq(&proj("Str", 1, cons.clone()), &cnst("st")));
    assert!(
        !tc.is_def_eq(&proj("Str", 0, cons), &cnst("n1")),
        "the field, and nothing else"
    );
    let i1 = app(cnst("I1.mk"), cnst("b0"));
    assert!(tc.is_def_eq(&proj("I1", 0, i1), &cnst("b0")));
    let pi_mk = apps(cnst("PI.mk"), vec![cnst("B"), cnst("b0"), cnst("n1")]);
    assert!(tc.is_def_eq(&proj("PI", 0, pi_mk.clone()), &cnst("b0")));
    assert!(tc.is_def_eq(&proj("PI", 1, pi_mk), &cnst("n1")));
    let dep = apps(cnst("DepI.mk"), vec![cnst("n1"), cnst("f1")]);
    let ty = tc
        .infer_type(&proj("DepI", 1, dep.clone()))
        .expect("(DepI.mk n1 f1).1 is typed");
    assert_eq!(ty, app(cnst("F"), proj("DepI", 0, dep.clone())));
    assert!(
        tc.is_def_eq(&ty, &app(cnst("F"), cnst("n1"))),
        "F (DepI.mk n1 f1).0 reduces to F n1"
    );
    assert!(tc.is_def_eq(&proj("DepI", 1, dep), &cnst("f1")));
}

// ---------------------------------------------------------------------------
// Refused: what the inductive type may not be, and the operand's type
// ---------------------------------------------------------------------------

/// `two.0` and `mb.0` — two constructors, alone and inside a mutual block.
/// Field 0 exists in the first constructor of each; the projection is
/// refused for the number of constructors, not for its index.
#[test]
fn two_constructors_are_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    for (name, operand) in [("Two", "two"), ("MB", "mb")] {
        match tc.infer_type(&proj(name, 0, cnst(operand))) {
            Err(e) => assert!(
                e.to_string().contains("constructors"),
                "{name} field 0: refused, but not for its two constructors: {e}"
            ),
            Ok(ty) => {
                panic!("{name} field 0: a type with two constructors was given the type {ty}")
            }
        }
    }
}

/// `emp.0` — no constructor.
#[test]
fn no_constructor_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_invalid(tc.infer_type(&proj("Emp", 0, cnst("emp"))), "Emp field 0");
}

/// A field index past the last field: `st.2`, `v0.1`, `pv.2`, and `u1a.0`
/// for a constructor with no field at all.
#[test]
fn a_field_index_out_of_range_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_invalid(tc.infer_type(&proj("Str", 2, cnst("st"))), "Str field 2");
    assert_invalid(tc.infer_type(&proj("I1", 1, cnst("v0"))), "I1 field 1");
    assert_invalid(tc.infer_type(&proj("PI", 2, cnst("pv"))), "PI field 2");
    assert_invalid(tc.infer_type(&proj("U1", 0, cnst("u1a"))), "U1 field 0");
    // The index is checked against the constructor's field count before the
    // telescope is walked: the refusal names the range, whatever the walk
    // past the last field would have answered.
    for (name, idx, operand) in [("Str", 2, "st"), ("I1", 1, "v0"), ("U1", 0, "u1a")] {
        match tc.infer_type(&proj(name, idx, cnst(operand))) {
            Err(e) => assert!(
                e.to_string().contains("out of range"),
                "{name} field {idx}: refused, but not for the field index: {e}"
            ),
            Ok(ty) => panic!("{name} field {idx}: given the type {ty}"),
        }
    }
}

/// The name must be the family of the operand's type: another family's
/// name, the name of a definition that unfolds to the family, and the name
/// of a constant that is no inductive type are all refused.
#[test]
fn the_name_of_another_constant_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_invalid(
        tc.infer_type(&proj("I1", 0, cnst("st"))),
        "I1 named, the operand a Str",
    );
    assert_invalid(
        tc.infer_type(&proj("Str", 0, cnst("v0"))),
        "Str named, the operand an I1 n0",
    );
    assert_invalid(
        tc.infer_type(&proj("StrAlias", 0, cnst("st"))),
        "a definition named",
    );
    assert_invalid(tc.infer_type(&proj("N", 0, cnst("n0"))), "an axiom named");
}

/// `fs.0` for `fs : N → Str` — the operand's type is a function type into
/// the family, not the family.
#[test]
fn an_operand_that_is_a_function_into_the_family_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_invalid(tc.infer_type(&proj("Str", 0, cnst("fs"))), "fs.0");
}

/// `fun v => v.0 : PI B n1 → N` and `fun v => v.1 : (v : DepI n1) → F n1`
/// are rejected as declarations: the index is not a constructor argument,
/// and a later field's type names the earlier field.
#[test]
fn a_declaration_that_reads_the_index_as_a_constructor_argument_is_rejected() {
    let mut env = env();
    let pi_b_n1 = || apps(cnst("PI"), vec![cnst("B"), cnst("n1")]);
    let bad = definition(
        "bad",
        pi("v", pi_b_n1(), cnst("N")),
        lam("v", pi_b_n1(), proj("PI", 0, bvar(0))),
    );
    assert!(
        check_constant_info(&mut env, bad).is_err(),
        "field 0 of PI B n1 is a B, not an N"
    );
    assert!(env.find(&Name::str("bad")).is_none());
    let dep_n1 = || app(cnst("DepI"), cnst("n1"));
    let bad_dep = definition(
        "bad_dep",
        pi("v", dep_n1(), app(cnst("F"), cnst("n1"))),
        lam("v", dep_n1(), proj("DepI", 1, bvar(0))),
    );
    assert!(
        check_constant_info(&mut env, bad_dep).is_err(),
        "field 1 of v : DepI n1 is an F v.0, not an F n1"
    );
    assert!(env.find(&Name::str("bad_dep")).is_none());
}

// ---------------------------------------------------------------------------
// Refused: no data out of a proof, for recursive and indexed propositions
// ---------------------------------------------------------------------------

/// `rpd.0` and `rpdep.0` — the data fields of recursive propositions — and
/// `rpdep.1`, a proof whose type `q rpdep.0` depends on the data field.
/// `rpdep.2 : RPdep q`, the recursive proof field, is refused as well, as
/// Lean's kernel refuses it: its own type mentions no field, but it comes
/// after a data field that the rest of the telescope depends on.
#[test]
fn data_out_of_a_recursive_proposition_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_no_data_out_of_a_proof(tc.infer_type(&proj("RPD", 0, cnst("rpd"))), "RPD field 0");
    assert_no_data_out_of_a_proof(
        tc.infer_type(&proj("RPdep", 0, cnst("rpdep"))),
        "RPdep field 0",
    );
    assert_no_data_out_of_a_proof(
        tc.infer_type(&proj("RPdep", 1, cnst("rpdep"))),
        "RPdep field 1",
    );
    assert_no_data_out_of_a_proof(
        tc.infer_type(&proj("RPdep", 2, cnst("rpdep"))),
        "RPdep field 2",
    );
}

/// `ipd.0`, `iex.0`, `iex.1`, `ipdep.0` — the data fields of indexed
/// propositions. `IPd`'s field IS its index, so its recursor eliminates
/// into `Type`; the projection is refused all the same. The proposition is
/// recognised although its sort stands behind an index in the declared
/// type.
#[test]
fn data_out_of_an_indexed_proposition_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_no_data_out_of_a_proof(tc.infer_type(&proj("IPd", 0, cnst("ipd"))), "IPd field 0");
    assert_no_data_out_of_a_proof(tc.infer_type(&proj("IEx", 0, cnst("iex"))), "IEx field 0");
    assert_no_data_out_of_a_proof(tc.infer_type(&proj("IEx", 1, cnst("iex"))), "IEx field 1");
    assert_no_data_out_of_a_proof(
        tc.infer_type(&proj("IPdep", 0, cnst("ipdep"))),
        "IPdep field 0",
    );
}

/// `ipdep.1` — a proof, but its type `q ipdep.0` depends on the data field;
/// refused also by an infer-only checker, which does not type the arguments
/// of the application `(fun _ => T) ipdep.0`.
#[test]
fn a_proof_field_that_depends_on_a_data_field_of_an_indexed_proposition_is_refused() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert_no_data_out_of_a_proof(
        tc.infer_type(&proj("IPdep", 1, cnst("ipdep"))),
        "IPdep field 1",
    );
    let mut infer_only = TypeChecker::new_infer_only(&env);
    assert_no_data_out_of_a_proof(
        infer_only.infer_type(&proj("IPdep", 1, cnst("ipdep"))),
        "IPdep field 1, infer-only",
    );
}

/// The witness function of the indexed existential is rejected as a
/// declaration and not installed.
#[test]
fn the_witness_of_an_indexed_existential_is_rejected_as_a_declaration() {
    let mut env = env();
    let iex_n0 = || app(cnst("IEx"), cnst("n0"));
    let wit = definition(
        "wit",
        pi("h", iex_n0(), cnst("N")),
        lam("h", iex_n0(), proj("IEx", 1, bvar(0))),
    );
    assert!(
        check_constant_info(&mut env, wit).is_err(),
        "a function that reads the witness out of a proof must be rejected"
    );
    assert!(env.find(&Name::str("wit")).is_none());
}

// ---------------------------------------------------------------------------
// The def-eq checker's own inference follows the same rule
// ---------------------------------------------------------------------------

/// `rp.0` is a proof of `RP`, as `rp` is: equal by proof irrelevance — which
/// needs the def-eq checker to type the projection out of a recursive type.
#[test]
fn the_def_eq_checker_types_a_projection_out_of_a_recursive_inductive() {
    let env = env();
    let mut dc = DefEqChecker::new(&env);
    assert!(
        dc.is_def_eq(&proj("RP", 0, cnst("rp")), &cnst("rp")),
        "two proofs of RP are definitionally equal"
    );
}

/// `ipp.0` and `pq.0` are proofs of `T`, as `t0` is. For `pq : PIq T n1`
/// the field's type is the PARAMETER `T`; with the index read as a
/// constructor argument it would be `N`, and nothing would be equal.
#[test]
fn the_def_eq_checker_types_a_projection_out_of_an_indexed_family() {
    let env = env();
    let mut dc = DefEqChecker::new(&env);
    assert!(
        dc.is_def_eq(&proj("IPp", 0, cnst("ipp")), &cnst("t0")),
        "ipp.0 and t0 are proofs of T"
    );
    assert!(
        dc.is_def_eq(&proj("PIq", 0, cnst("pq")), &cnst("t0")),
        "pq.0 and t0 are proofs of T"
    );
}

/// The def-eq checker gives no type to a projection the type checker
/// refuses: `ipdep.1` (indexed) and `rpdep.1` (recursive) would be typed
/// `(fun _ => T) x.0`, a proposition, and proof irrelevance would equate
/// them with any proof of `T`.
#[test]
fn the_def_eq_checker_gives_a_refused_projection_no_type() {
    let env = env();
    let mut dc = DefEqChecker::new(&env);
    assert!(
        !dc.is_def_eq(&proj("IPdep", 1, cnst("ipdep")), &cnst("t0")),
        "proof irrelevance fired on a projection out of an indexed proposition \
         that the type checker refuses"
    );
    assert!(
        !dc.is_def_eq(&proj("RPdep", 1, cnst("rpdep")), &cnst("t0")),
        "proof irrelevance fired on a projection out of a recursive proposition \
         that the type checker refuses"
    );
    assert!(
        !dc.is_def_eq(&proj("Two", 0, cnst("two")), &cnst("t0")),
        "a projection out of a type with two constructors has no type \
         (typed by its first constructor it would be a proof of T)"
    );
}

// ---------------------------------------------------------------------------
// What did not move: structure eta, eta inside iota, unit-like equality
// ---------------------------------------------------------------------------

/// `Str.rec.{1} (fun _ => N) (fun h t ih => h) major`.
fn str_rec(major: Expr) -> Expr {
    apps(
        cnst_lvls("Str.rec", vec![lvl1()]),
        vec![
            lam("_s", cnst("Str"), cnst("N")),
            lam(
                "h",
                cnst("N"),
                lam("t", cnst("Str"), lam("ih", cnst("N"), bvar(2))),
            ),
            major,
        ],
    )
}

/// `I1.rec.{1} (fun _ _ => B) (fun b => b) n0 major`.
fn i1_rec(major: Expr) -> Expr {
    apps(
        cnst_lvls("I1.rec", vec![lvl1()]),
        vec![
            lam(
                "i",
                cnst("N"),
                lam("_v", app(cnst("I1"), bvar(0)), cnst("B")),
            ),
            lam("b", cnst("B"), bvar(0)),
            cnst("n0"),
            major,
        ],
    )
}

/// The control: `pr = Pair.mk pr.0 pr.1` holds by structure eta for the
/// structure-like `Pair`, in the type checker and in the def-eq checker.
#[test]
fn structure_eta_holds_for_a_structure_like_inductive() {
    let env = env();
    let expansion = apps(
        cnst("Pair.mk"),
        vec![proj("Pair", 0, cnst("pr")), proj("Pair", 1, cnst("pr"))],
    );
    let mut tc = TypeChecker::new(&env);
    assert!(tc.is_def_eq(&cnst("pr"), &expansion));
    assert!(tc.is_def_eq(&expansion, &cnst("pr")));
    let mut dc = DefEqChecker::new(&env);
    assert!(dc.is_def_eq(&cnst("pr"), &expansion));
}

/// `st = Str.cons st.0 st.1` does NOT hold by definition: the projections
/// are typed, structure eta is still not applied to a recursive type.
#[test]
fn no_structure_eta_for_a_recursive_inductive() {
    let env = env();
    let expansion = apps(
        cnst("Str.cons"),
        vec![proj("Str", 0, cnst("st")), proj("Str", 1, cnst("st"))],
    );
    let mut tc = TypeChecker::new(&env);
    tc.infer_type(&expansion)
        .expect("the expansion itself is well typed");
    assert!(!tc.is_def_eq(&cnst("st"), &expansion));
    assert!(!tc.is_def_eq(&expansion, &cnst("st")));
    let mut dc = DefEqChecker::new(&env);
    assert!(!dc.is_def_eq(&cnst("st"), &expansion));
    assert!(!dc.is_def_eq(&expansion, &cnst("st")));
}

/// `v0 = I1.mk v0.0` does NOT hold by definition for the indexed family.
#[test]
fn no_structure_eta_for_an_indexed_family() {
    let env = env();
    let expansion = app(cnst("I1.mk"), proj("I1", 0, cnst("v0")));
    let mut tc = TypeChecker::new(&env);
    tc.infer_type(&expansion)
        .expect("the expansion itself is well typed");
    assert!(!tc.is_def_eq(&cnst("v0"), &expansion));
    assert!(!tc.is_def_eq(&expansion, &cnst("v0")));
    let mut dc = DefEqChecker::new(&env);
    assert!(!dc.is_def_eq(&cnst("v0"), &expansion));
}

/// A recursor applied to a VARIABLE of a recursive type stays stuck (no eta
/// inside iota), and reduces on a constructor application.
#[test]
fn a_recursor_is_stuck_on_a_variable_of_a_recursive_inductive() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    tc.infer_type(&str_rec(cnst("st")))
        .expect("the recursor application is well typed");
    assert!(
        !tc.is_def_eq(&str_rec(cnst("st")), &proj("Str", 0, cnst("st"))),
        "Str.rec … st must not reduce through an eta expansion of st"
    );
    let cons = apps(cnst("Str.cons"), vec![cnst("n0"), cnst("st")]);
    assert!(tc.is_def_eq(&str_rec(cons), &cnst("n0")));
}

/// The same for the indexed family.
#[test]
fn a_recursor_is_stuck_on_a_variable_of_an_indexed_family() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    tc.infer_type(&i1_rec(cnst("v0")))
        .expect("the recursor application is well typed");
    assert!(
        !tc.is_def_eq(&i1_rec(cnst("v0")), &proj("I1", 0, cnst("v0"))),
        "I1.rec … v0 must not reduce through an eta expansion of v0"
    );
    assert!(tc.is_def_eq(&i1_rec(app(cnst("I1.mk"), cnst("b0"))), &cnst("b0")));
}

/// Two elements of `U1 n0` — one constructor, no field, an index — are NOT
/// equal by definition: unit-like equality asks for no indices.
#[test]
fn no_unit_like_equality_for_an_indexed_family() {
    let env = env();
    let mut tc = TypeChecker::new(&env);
    assert!(!tc.is_def_eq(&cnst("u1a"), &cnst("u1b")));
    let mut dc = DefEqChecker::new(&env);
    assert!(!dc.is_def_eq(&cnst("u1a"), &cnst("u1b")));
}
