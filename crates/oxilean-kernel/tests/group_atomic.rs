//! A declaration that arrives as several constants is added whole or not at
//! all, and the flags of an inductive family are the ones Lean 4's kernel
//! computes.
//!
//! * `check_constant_infos_atomic` checks a group in order and, when a
//!   member is refused, takes the earlier members out again — where
//!   `check_constant_infos` leaves them. An inductive type that lists one
//!   constructor, followed by the records of two, is the case the export
//!   replay met: left half installed, the one-constructor type answered for
//!   its name, and a projection out of it (or structure eta on it) checked.
//! * `Environment::add_constants` adds a group only when every name is free.
//! * `expr_util::find_mentioned_name` finds the declarations a term names —
//!   as a constant, and as the structure of a projection — looking at each
//!   shared subterm once.
//! * `is_rec` and `is_reflexive` of a derived family are read off the
//!   constructor types AS WRITTEN and hold for the family as a whole. The
//!   expected pairs below are what Lean 4 v4.32.0-rc1 printed for the same
//!   declarations handed to its kernel as raw `Declaration.inductDecl`
//!   values (measured 2026-11-12): a field of type `Konst T`, where `Konst`
//!   drops its argument, makes the family recursive although the field gets
//!   no induction hypothesis; a field of type `FnTo T`, a function into `T`
//!   only after unfolding, does not make it reflexive.

use oxilean_kernel::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, ConstructorVal, DefinitionSafety, DefinitionVal,
    InductiveVal,
};
use oxilean_kernel::expr_util::find_mentioned_name;
use oxilean_kernel::Node;
use oxilean_kernel::{
    check_and_derive_family, check_constant_info, check_constant_infos,
    check_constant_infos_atomic, BinderInfo, Environment, Expr, InductiveSpec, Level, Name,
    ReducibilityHint,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}

fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Node::new(f), Node::new(a))
}

/// `a → b`.
fn arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Node::new(a),
        Node::new(b),
    )
}

fn axiom(name: &str, ty: Expr) -> ConstantInfo {
    ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty,
        },
        is_unsafe: false,
    })
}

/// An environment with `A : Type`.
fn env_with_a() -> Environment {
    let mut env = Environment::new();
    check_constant_info(&mut env, axiom("A", type0())).expect("A : Type");
    env
}

/// `inductive Two | mk : A → Two | nil : Two` as the constants an export
/// carries — the type and the records of BOTH constructors — with the
/// type's constructor list as given.
fn two_group(listed: &[&str]) -> Vec<ConstantInfo> {
    let two = Name::str("Two");
    let ctor = |name: &str, ty: Expr, cidx: u32, num_fields: u32| {
        ConstantInfo::Constructor(ConstructorVal {
            common: ConstantVal {
                name: Name::str(name),
                level_params: vec![],
                ty,
            },
            induct: two.clone(),
            cidx,
            num_params: 0,
            num_fields,
            is_unsafe: false,
        })
    };
    vec![
        ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: two.clone(),
                level_params: vec![],
                ty: type0(),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![two.clone()],
            ctors: listed.iter().map(|c| Name::str(*c)).collect(),
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }),
        ctor("Two.mk", arrow(cnst("A"), cnst("Two")), 0, 1),
        ctor("Two.nil", cnst("Two"), 1, 0),
    ]
}

const TWO_NAMES: [&str; 3] = ["Two", "Two.mk", "Two.nil"];

// ---------------------------------------------------------------------------
// A group is added whole or not at all
// ---------------------------------------------------------------------------

#[test]
fn a_group_that_checks_is_added_whole() {
    let mut env = env_with_a();
    check_constant_infos_atomic(&mut env, two_group(&["Two.mk", "Two.nil"]))
        .expect("the honest group checks");
    for name in TWO_NAMES {
        assert!(env.contains(&Name::str(name)), "{name} must be installed");
    }
}

#[test]
fn a_group_refused_at_a_later_member_leaves_nothing_behind() {
    // The type lists ONE constructor; the record of the second is refused
    // ("not listed") after the type and the first constructor were checked
    // and added.
    let mut env = env_with_a();
    let before = env.len();
    let err = check_constant_infos_atomic(&mut env, two_group(&["Two.mk"]))
        .expect_err("a constructor its type does not list must be refused");
    assert!(
        format!("{err:?}").contains("is not listed"),
        "refused for the unlisted constructor: {err:?}"
    );
    for name in TWO_NAMES {
        assert!(
            !env.contains(&Name::str(name)),
            "{name}: a refused group must leave no member in the environment"
        );
    }
    assert_eq!(env.len(), before, "the environment is what it was");
    assert!(env.contains(&Name::str("A")), "what was there before stays");

    // The names are free again: the honest group can still be added.
    check_constant_infos_atomic(&mut env, two_group(&["Two.mk", "Two.nil"]))
        .expect("the honest group checks after the refused one");
    assert_eq!(
        env.get_inductive_val(&Name::str("Two"))
            .map(|iv| iv.ctors.len()),
        Some(2)
    );
}

#[test]
fn the_member_by_member_entry_point_leaves_the_earlier_members() {
    // What `check_constant_infos_atomic` is for: the same refused group
    // through `check_constant_infos` leaves a `Two` with one constructor.
    let mut env = env_with_a();
    assert!(check_constant_infos(&mut env, two_group(&["Two.mk"])).is_err());
    assert!(env.contains(&Name::str("Two")));
    assert!(env.contains(&Name::str("Two.mk")));
    assert!(!env.contains(&Name::str("Two.nil")));
    assert_eq!(
        env.get_inductive_val(&Name::str("Two"))
            .map(|iv| iv.ctors.len()),
        Some(1),
        "a type the kernel never accepted as a whole, with one constructor"
    );
}

#[test]
fn add_constants_adds_a_group_only_when_every_name_is_free() {
    let mut env = env_with_a();
    let before = env.len();

    // A name of the group is already declared.
    let clash = vec![axiom("B", type0()), axiom("A", type0())];
    assert!(env.add_constants(clash).is_err());
    assert!(
        !env.contains(&Name::str("B")),
        "nothing of the group is added"
    );
    assert_eq!(env.len(), before);

    // A name occurs twice in the group.
    let twice = vec![
        axiom("B", type0()),
        axiom("C", type0()),
        axiom("B", type0()),
    ];
    assert!(env.add_constants(twice).is_err());
    assert!(!env.contains(&Name::str("B")) && !env.contains(&Name::str("C")));
    assert_eq!(env.len(), before);

    // Every name free: all of them.
    env.add_constants(vec![axiom("B", type0()), axiom("C", type0())])
        .expect("free names");
    assert!(env.contains(&Name::str("B")) && env.contains(&Name::str("C")));
    assert_eq!(env.len(), before + 2);
}

// ---------------------------------------------------------------------------
// The declarations a term names
// ---------------------------------------------------------------------------

#[test]
fn a_term_names_its_constants_and_the_structures_of_its_projections() {
    // f (S.0 c)
    let term = app(
        cnst("f"),
        Expr::Proj(Name::str("S"), 0, Node::new(cnst("c"))),
    );
    let finds = |wanted: &str| {
        find_mentioned_name(&term, |n| *n == Name::str(wanted)).map(ToString::to_string)
    };
    assert_eq!(finds("f").as_deref(), Some("f"));
    assert_eq!(finds("c").as_deref(), Some("c"));
    assert_eq!(
        finds("S").as_deref(),
        Some("S"),
        "a projection names its structure though no constant stands for it"
    );
    assert_eq!(finds("absent"), None);
    // Left to right: the first name the predicate holds for.
    assert_eq!(
        find_mentioned_name(&term, |_| true)
            .map(ToString::to_string)
            .as_deref(),
        Some("f")
    );
}

#[test]
fn a_shared_subterm_is_looked_at_once() {
    // t0 = leaf; t(k+1) = t(k) t(k), sharing: 64 nodes that are 2^64 leaves
    // as a tree. The walk asks about `leaf` once.
    let mut term = cnst("leaf");
    for _ in 0..64 {
        let shared = Node::new(term);
        term = Expr::App(shared.clone(), shared);
    }
    let mut asked = 0u32;
    let found = find_mentioned_name(&term, |_| {
        asked += 1;
        // A walk that follows every path would ask 2^64 times: fail at the
        // second look instead of never finishing.
        assert!(asked <= 1, "the shared constant was looked at again");
        false
    });
    assert!(found.is_none());
    assert_eq!(asked, 1, "one constant node, however many paths lead to it");
}

// ---------------------------------------------------------------------------
// `is_rec` and `is_reflexive` as Lean's kernel computes them
// ---------------------------------------------------------------------------

/// `Nat : Type`, and three definitions on types: `Konst α := Nat`,
/// `IdT α := α`, `FnTo α := Nat → α`.
fn flags_env() -> Environment {
    let mut env = Environment::new();
    check_constant_info(&mut env, axiom("Nat", type0())).expect("Nat");
    let ty_to_ty = arrow(type0(), type0());
    let lam = |body: Expr| {
        Expr::Lam(
            BinderInfo::Default,
            Name::str("α"),
            Node::new(type0()),
            Node::new(body),
        )
    };
    for (name, value) in [
        ("Konst", lam(cnst("Nat"))),
        ("IdT", lam(Expr::BVar(0))),
        ("FnTo", lam(arrow(cnst("Nat"), Expr::BVar(1)))),
    ] {
        let def = ConstantInfo::Definition(DefinitionVal {
            common: ConstantVal {
                name: Name::str(name),
                level_params: vec![],
                ty: ty_to_ty.clone(),
            },
            value,
            hints: ReducibilityHint::Abbrev,
            safety: DefinitionSafety::Safe,
            all: vec![Name::str(name)],
        });
        check_constant_info(&mut env, def).unwrap_or_else(|e| panic!("{name}: {e:?}"));
    }
    env
}

/// `(is_rec, is_reflexive)` of every type of the family the kernel derives
/// from `types`: each a name and the type of the one field of its one
/// constructor `mk : field → T`.
fn derived_flags(types: &[(&str, Expr)]) -> Vec<(bool, bool)> {
    let specs: Vec<InductiveSpec> = types
        .iter()
        .map(|(name, field)| {
            InductiveSpec::new(
                Name::str(*name),
                type0(),
                vec![(
                    Name::str(format!("{name}.mk")),
                    arrow(field.clone(), cnst(name)),
                )],
            )
        })
        .collect();
    let derived = check_and_derive_family(&flags_env(), &[], 0, &specs)
        .unwrap_or_else(|e| panic!("{:?}: {e:?}", types.iter().map(|t| t.0).collect::<Vec<_>>()));
    derived
        .inductives
        .iter()
        .map(|ci| {
            let iv = ci.to_inductive_val().expect("an inductive");
            (iv.is_rec, iv.is_reflexive)
        })
        .collect()
}

#[test]
fn the_flags_of_a_single_type_are_read_off_the_constructor_types_as_written() {
    let t = || cnst("T");
    let nat = || cnst("Nat");
    // (field type, is_rec, is_reflexive) — Lean's FACT lines, in order:
    // A01, A02, A03, W01, W02, W03, W04, W05, W06.
    let cases: Vec<(&str, Expr, bool, bool)> = vec![
        ("not recursive: Nat", nat(), false, false),
        ("directly recursive: T", t(), true, false),
        ("reflexive: Nat → T", arrow(nat(), t()), true, true),
        // Named in the field's type, gone once the type is reduced: Lean
        // records the family as recursive all the same.
        ("Konst T", app(cnst("Konst"), t()), true, false),
        (
            "(fun _ => Nat) T",
            app(
                Expr::Lam(
                    BinderInfo::Default,
                    Name::str("a"),
                    Node::new(type0()),
                    Node::new(nat()),
                ),
                t(),
            ),
            true,
            false,
        ),
        ("IdT T", app(cnst("IdT"), t()), true, false),
        // A function into the family only after unfolding: recursive, and
        // NOT reflexive for Lean (the field's type is not written as a
        // function type).
        ("FnTo T", app(cnst("FnTo"), t()), true, false),
        (
            "Nat → IdT T",
            arrow(nat(), app(cnst("IdT"), t())),
            true,
            true,
        ),
        // Written as a function type and naming the family: reflexive for
        // Lean, although the occurrence is gone once the type is reduced.
        (
            "Nat → Konst T",
            arrow(nat(), app(cnst("Konst"), t())),
            true,
            true,
        ),
    ];
    for (what, field, is_rec, is_reflexive) in cases {
        assert_eq!(
            derived_flags(&[("T", field)]),
            vec![(is_rec, is_reflexive)],
            "mk : {what} → T"
        );
    }
}

#[test]
fn the_flags_hold_for_a_mutual_family_as_a_whole() {
    let nat = || cnst("Nat");
    // Only the second type refers to the family: both are recursive.
    assert_eq!(
        derived_flags(&[("MA", nat()), ("MB", cnst("MB"))]),
        vec![(true, false), (true, false)]
    );
    // The first refers to the second, the second to nothing.
    assert_eq!(
        derived_flags(&[("MC", cnst("MD")), ("MD", nat())]),
        vec![(true, false), (true, false)]
    );
    // Neither refers to either.
    assert_eq!(
        derived_flags(&[("ME", nat()), ("MF", nat())]),
        vec![(false, false), (false, false)]
    );
    // The second takes a function into the first: both are reflexive.
    assert_eq!(
        derived_flags(&[("MG", nat()), ("MH", arrow(nat(), cnst("MG")))]),
        vec![(true, true), (true, true)]
    );
}
