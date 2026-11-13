//! Replay tests for the inductive bundle and for what a failed record leaves
//! behind (the other replay tests are in `replay_smoke.rs`).
//!
//! * A bundle is installed **whole or not at all**: the hand-edited negative
//!   fixture `CtorListLie` — three inductive bundles that lie about their
//!   constructor lists → each `Rejected` as a whole, nothing of them in the
//!   environment, and every declaration that names one `Rejected` — and the
//!   same for a bundle refused at a constructor or at its recursor.
//! * What a bundle says beyond its declaration (`isRec`, `isReflexive`,
//!   `numNested`, `all`, one `isUnsafe` flag, one recursor per type, and on
//!   the nested path `numIndices` and every constructor field) is held to
//!   the kernel's derivation, one edited field at a time.
//! * The cascade from a failed record goes **by name**: a record that
//!   mentions a rejected (or an unsupported) name is rejected (deferred)
//!   whether or not a constant of that name is present, and a name of a
//!   failed record cannot be declared again.
//! * A record of a bundle is tied to the derived item it **names**: a
//!   recursor record is the recursor of the type it is named after (on the
//!   nested path under Lean's names, `T.rec` and `<first type>.rec_<k>`), a
//!   rule is the rule of the constructor at its place, a constructor record
//!   is the entry of its name in its type's list.
//! * A bundle cut short by a resource limit, at any point, leaves nothing
//!   behind and defers its dependents.
//! * A theorem record states a proposition; a second `quot` record for a
//!   primitive declares it again; a file's own `Nat.pred` is not computed
//!   by the reducer's rule for the builtin one.

use oxilean_kernel::Node;

use oxilean_export::{
    read_str, replay_file, ExportDecl, InductiveBundle, ReplayOutcome, Replayer,
    DEFERRED_DEPENDENCY,
};
use oxilean_kernel::{
    AxiomVal, ConstantVal, DefinitionSafety, DefinitionVal, Expr, InductiveVal, Level, Name,
    ReducibilityHint,
};

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

/// Replay a fixture and assert its exact three-bucket split.
fn assert_split(name: &str, checked: usize, unsupported: usize, rejected: usize) {
    let file = read_str(&fixture(name)).unwrap_or_else(|e| panic!("parse {name}: {e}"));
    let report = replay_file(&file);
    let bad: Vec<String> = report
        .entries
        .iter()
        .filter(|e| !e.outcome.is_checked())
        .map(|e| format!("{} {}: {:?}", e.kind, e.name, e.outcome))
        .collect();
    assert_eq!(
        (report.checked(), report.unsupported(), report.rejected()),
        (checked, unsupported, rejected),
        "{name}: split mismatch; non-checked entries: {bad:#?}"
    );
}

// ---------------------------------------------------------------------------
// A bundle is installed whole or not at all, and what the file says about it
// beyond its declaration is what the kernel derives.
// ---------------------------------------------------------------------------

/// The declarations of a fixture.
fn fixture_decls(name: &str) -> Vec<ExportDecl> {
    read_str(&fixture(name))
        .unwrap_or_else(|e| panic!("parse {name}: {e}"))
        .decls
}

/// The bundle among `decls` that declares the inductive type `ty`.
fn bundle_of<'a>(decls: &'a mut [ExportDecl], ty: &str) -> &'a mut InductiveBundle {
    decls
        .iter_mut()
        .find_map(|d| match d {
            ExportDecl::Inductive(b) if b.types.iter().any(|t| t.common.name.to_string() == ty) => {
                Some(b)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no bundle declares {ty}"))
}

/// Replay `decls` in order: the outcome of each, by primary name.
fn replay_each(replayer: &mut Replayer, decls: &[ExportDecl]) -> Vec<(String, ReplayOutcome)> {
    decls
        .iter()
        .map(|d| {
            let entry = replayer.replay(d);
            (entry.name.to_string(), entry.outcome)
        })
        .collect()
}

fn verdict<'a>(outcomes: &'a [(String, ReplayOutcome)], name: &str) -> &'a ReplayOutcome {
    outcomes
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, outcome)| outcome)
        .unwrap_or_else(|| panic!("no outcome for {name}"))
}

/// `outcome` is a rejection whose reason contains `needle`.
fn assert_rejected_for(outcome: &ReplayOutcome, needle: &str, what: &str) {
    match outcome {
        ReplayOutcome::Rejected { reason } => assert!(
            reason.contains(needle),
            "{what}: the reason must say `{needle}`, got: {reason}"
        ),
        other => panic!("{what}: must be REJECTED, got {other:?}"),
    }
}

/// `name : ty := value`, a safe definition written by hand.
fn definition(name: &str, ty: Expr, value: Expr) -> ExportDecl {
    ExportDecl::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty,
        },
        value,
        hints: ReducibilityHint::Abbrev,
        safety: DefinitionSafety::Safe,
        all: vec![Name::str(name)],
    })
}

/// `axiom name : Type`, written by hand.
fn type_axiom(name: &str) -> ExportDecl {
    ExportDecl::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        },
        is_unsafe: false,
    })
}

/// `name : Type := c`, a definition that mentions the constant `c` and
/// nothing else.
fn mentions(name: &str, c: Name) -> ExportDecl {
    definition(
        name,
        Expr::Sort(Level::succ(Level::zero())),
        Expr::Const(c, vec![]),
    )
}

#[test]
fn a_bundle_that_lies_about_its_constructors_installs_nothing() {
    // A real lean4export file with THREE RECORDS EDITED BY HAND (see the
    // fixture README): `Two`, `L2` (recursive) and `TI` (indexed) each have
    // two constructors, and each type's `ctors` list was cut down to the
    // first. The four declarations after them read a field out of one of
    // the types with a raw projection, which Lean's kernel refuses for a
    // type that does not have exactly one constructor — and `two_eta`
    // states `∀ t : Two, t = Two.mk t.0`, false of `Two.nil`.
    //
    // The kernel refuses each bundle: the second constructor's record names
    // a type that does not list it. While the replay installed a bundle
    // member by member, the refusal came after the type and its first
    // constructor were in the environment, and they stayed: `two_proj` and
    // `two_eta` verified against a one-constructor `Two` (5 checked /
    // 0 unsupported / 5 rejected; the published 0.1.3 and 0.1.4), and with
    // projections out of recursive and indexed one-constructor types
    // admitted `l2_proj` and `ti_proj` did too (7 / 0 / 3).
    assert_split("CtorListLie", 3, 0, 7);

    let decls = fixture_decls("CtorListLie");
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);
    let ns = Name::from_str("CtorListLie");
    let name = |path: &[&str]| path.iter().fold(ns.clone(), |n, part| n.append_str(*part));

    // Each bundle is rejected for the constructor its type does not list ...
    for (ty, unlisted) in [("Two", "nil"), ("L2", "nil"), ("TI", "b")] {
        let what = format!("CtorListLie.{ty}");
        assert_rejected_for(verdict(&outcomes, &what), "is not listed", &what);
        assert_rejected_for(
            verdict(&outcomes, &what),
            &format!("CtorListLie.{ty}.{unlisted}"),
            &what,
        );
    }
    // ... and leaves NOTHING behind: not the type, not the constructor that
    // was checked before the refusal, not the other one, not the recursor.
    for path in [
        &["Two"][..],
        &["Two", "mk"],
        &["Two", "nil"],
        &["Two", "rec"],
        &["L2"],
        &["L2", "cons"],
        &["L2", "nil"],
        &["L2", "rec"],
        &["TI"],
        &["TI", "a"],
        &["TI", "b"],
        &["TI", "rec"],
    ] {
        let n = name(path);
        assert!(
            !replayer.env().contains(&n),
            "{n} is part of a rejected bundle and must not be in the environment"
        );
    }
    // The four declarations Lean's kernel rejects name a rejected bundle.
    for (probe, root) in [
        ("two_proj", "Two"),
        ("two_eta", "Two"),
        ("l2_proj", "L2"),
        ("ti_proj", "TI"),
    ] {
        let what = format!("CtorListLie.{probe}");
        assert_rejected_for(
            verdict(&outcomes, &what),
            &format!("depends on rejected declaration 'CtorListLie.{root}'"),
            &what,
        );
    }
    // Everything else in the file is sound and still checks.
    for sound in ["Nat", "Eq", "Bool"] {
        assert!(
            verdict(&outcomes, sound).is_checked(),
            "{sound}: {:?}",
            verdict(&outcomes, sound)
        );
    }
}

#[test]
fn a_bundle_refused_at_its_recursor_installs_nothing() {
    // The same lie carried one step further: `L2.nil`'s own record is taken
    // out of the bundle as well. Now every constructor record is listed, the
    // one-constructor family derives, the type and `L2.cons` pass — and the
    // file's recursor, the one Lean built for TWO constructors, is not the
    // derived one. The refusal comes at the LAST member of the bundle.
    let mut decls = fixture_decls("CtorListLie");
    bundle_of(&mut decls, "CtorListLie.L2")
        .ctors
        .retain(|c| c.common.name.to_string() != "CtorListLie.L2.nil");
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);

    assert_rejected_for(
        verdict(&outcomes, "CtorListLie.L2"),
        "recursor 'CtorListLie.L2.rec' does not match",
        "CtorListLie.L2",
    );
    let l2 = Name::from_str("CtorListLie.L2");
    for n in [
        l2.clone(),
        l2.clone().append_str("cons"),
        l2.clone().append_str("rec"),
    ] {
        assert!(
            !replayer.env().contains(&n),
            "{n}: a bundle refused at its recursor must leave neither its type nor its constructors"
        );
    }
    assert_rejected_for(
        verdict(&outcomes, "CtorListLie.l2_proj"),
        "depends on rejected declaration 'CtorListLie.L2'",
        "CtorListLie.l2_proj",
    );
}

#[test]
fn a_bundle_refused_at_a_constructor_leaves_no_type_behind() {
    // `Two` with its honest constructor list and ANOTHER record lying:
    // `Two.mk` claims no field. The family derives and the type passes; the
    // constructor is refused. A declaration that names the TYPE and nothing
    // else of the bundle must not find it.
    let mut decls = fixture_decls("CtorListLie");
    let two = bundle_of(&mut decls, "CtorListLie.Two");
    two.types[0].ctors = two.ctors.iter().map(|c| c.common.name.clone()).collect();
    two.ctors[0].num_fields = 0;
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);

    assert_rejected_for(
        verdict(&outcomes, "CtorListLie.Two"),
        "num_fields",
        "CtorListLie.Two",
    );
    let two_name = Name::from_str("CtorListLie.Two");
    assert!(
        !replayer.env().contains(&two_name),
        "the type of a bundle refused at a constructor must not stay in the environment"
    );
    let dependent = replayer.replay(&mentions("uses_the_type_only", two_name));
    assert_rejected_for(
        &dependent.outcome,
        "depends on rejected declaration 'CtorListLie.Two'",
        "a declaration that names only the type of a rejected bundle",
    );
}

#[test]
fn a_rejected_name_blocks_its_dependents_even_when_a_constant_of_that_name_is_present() {
    // The second line of defence, with no hole to help it: `A` is declared
    // and checked; a second record declares `A` again, which the kernel
    // refuses — and `A` is now a name of a rejected record WHILE a constant
    // `A` is in the environment. A declaration that mentions it is not
    // verified against whichever of the two the environment happens to hold.
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    assert!(replayer.replay(&type_axiom("A")).outcome.is_checked());
    assert_rejected_for(
        &replayer.replay(&type_axiom("A")).outcome,
        "duplicate declaration",
        "a second record for a checked name",
    );
    assert!(
        replayer.env().contains(&Name::str("A")),
        "the first `A` is still in the environment: that is the point of the test"
    );
    assert_rejected_for(
        &replayer.replay(&mentions("usesA", Name::str("A"))).outcome,
        "depends on rejected declaration 'A'",
        "a declaration that mentions a name of a rejected record",
    );
    // A projection names its structure without a constant standing for it.
    let through_a_projection = definition(
        "projectsA",
        Expr::Sort(Level::succ(Level::zero())),
        Expr::Proj(Name::str("A"), 0, Node::new(Expr::Sort(Level::zero()))),
    );
    assert_rejected_for(
        &replayer.replay(&through_a_projection).outcome,
        "depends on rejected declaration 'A'",
        "a declaration that names a rejected record as the structure of a projection",
    );
    // And the cascade goes on: `usesA` is a rejected name now.
    assert_rejected_for(
        &replayer
            .replay(&mentions("usesUsesA", Name::str("usesA")))
            .outcome,
        "depends on rejected declaration 'usesA'",
        "a dependent of a dependent",
    );
}

#[test]
fn a_rejected_name_cannot_be_declared_again() {
    // `B : Type := Type` is ill typed and rejected; nothing is installed, so
    // the kernel would take a second, well-typed `B`. A name is declared
    // once in a Lean environment: the second record is rejected, and so is
    // whatever mentions `B` afterwards.
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let ill_typed = definition("B", type0.clone(), type0);
    assert!(replayer.replay(&ill_typed).outcome.is_rejected());
    assert_rejected_for(
        &replayer.replay(&type_axiom("B")).outcome,
        "declares 'B' again",
        "a second record for a rejected name",
    );
    assert!(!replayer.env().contains(&Name::str("B")));
    assert_rejected_for(
        &replayer.replay(&mentions("usesB", Name::str("B"))).outcome,
        "depends on rejected declaration 'B'",
        "a declaration that mentions the twice-declared name",
    );
}

#[test]
fn an_unsupported_name_defers_its_dependents_even_when_a_constant_of_that_name_is_present() {
    // The same second line of defence for the unsupported bucket: `C` is
    // declared and checked, then a record the reader would not materialize
    // announces the name `C` (an oversized record carries its names only).
    // `C` is now a deferred name with a constant `C` present. A dependent is
    // deferred, not verified; with a rejected name beside the deferred one
    // it is rejected.
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    assert!(replayer.replay(&type_axiom("C")).outcome.is_checked());
    let oversized = ExportDecl::Oversized(oxilean_export::OversizedVal {
        names: vec![Name::str("C")],
        feature: oxilean_export::DECL_BUDGET_FEATURE,
    });
    assert_eq!(
        replayer.replay(&oversized).outcome,
        ReplayOutcome::Unsupported {
            feature: oxilean_export::DECL_BUDGET_FEATURE
        }
    );
    assert!(replayer.env().contains(&Name::str("C")));
    assert_eq!(
        replayer.replay(&mentions("usesC", Name::str("C"))).outcome,
        ReplayOutcome::Unsupported {
            feature: DEFERRED_DEPENDENCY
        },
        "a declaration that mentions a name of an unsupported record is deferred"
    );

    // Rejected beside deferred: rejected.
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    assert!(replayer
        .replay(&definition("D", type0.clone(), type0.clone()))
        .outcome
        .is_rejected());
    let both = definition(
        "usesCandD",
        type0,
        Expr::App(
            Node::new(Expr::Const(Name::str("C"), vec![])),
            Node::new(Expr::Const(Name::str("D"), vec![])),
        ),
    );
    assert_rejected_for(
        &replayer.replay(&both).outcome,
        "depends on rejected declaration 'D'",
        "a declaration that mentions a deferred and a rejected name",
    );
}

/// One lie about one bundle of a fixture: the type whose bundle is edited,
/// the text the rejection must carry, and the edit.
type BundleLie = (&'static str, &'static str, fn(&mut InductiveBundle));

/// Apply each lie to a fresh copy of the fixture and assert that the bundle
/// is rejected for it, that nothing of the bundle is installed, and that no
/// declaration of the file that is not checked is anything but rejected.
fn assert_each_lie_is_rejected(fixture_name: &str, lies: &[BundleLie]) {
    for (ty, needle, lie) in lies {
        let mut decls = fixture_decls(fixture_name);
        lie(bundle_of(&mut decls, ty));
        let names: Vec<Name> = match decls.iter().find(
            |d| matches!(d, ExportDecl::Inductive(b) if b.types.iter().any(|t| t.common.name.to_string() == *ty)),
        ) {
            Some(ExportDecl::Inductive(b)) => b
                .types
                .iter()
                .map(|t| t.common.name.clone())
                .chain(b.ctors.iter().map(|c| c.common.name.clone()))
                .chain(b.recs.iter().map(|r| r.common.name.clone()))
                .collect(),
            _ => panic!("no bundle declares {ty}"),
        };
        let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
        let outcomes = replay_each(&mut replayer, &decls);
        let what = format!("{fixture_name}: {ty} with a wrong `{needle}`");
        assert_rejected_for(verdict(&outcomes, ty), needle, &what);
        for n in &names {
            assert!(
                !replayer.env().contains(n),
                "{what}: {n} must not be in the environment"
            );
        }
        for (n, outcome) in &outcomes {
            assert!(
                outcome.is_checked() || outcome.is_rejected(),
                "{what}: {n} is neither checked nor rejected: {outcome:?}"
            );
        }
    }
}

#[test]
fn what_a_bundle_says_beyond_its_declaration_is_held_to_the_derivation() {
    // A Lean declaration of an inductive family is its universe parameters,
    // its number of parameters, and each type's name, type and constructors,
    // with one `isUnsafe` flag. The rest of a bundle is what Lean's kernel
    // computes from that, so a file that says otherwise is not the export of
    // a Lean environment. Each edit below changes ONE such field of the
    // ordinary-Lean fixture `ProjRecIndexed` (22 checked untouched): the
    // bundle must be rejected for it and leave nothing behind.
    //
    // `Stream` is directly recursive, under a function (so it is reflexive
    // as well), `Tagged α : Nat → Type` is not
    // recursive and has an index, `Tree` is nested through `List` (and so
    // takes the nested path, where the kernel-derived family is what gets
    // installed and the file's fields were not looked at before).
    assert_each_lie_is_rejected(
        "ProjRecIndexed",
        &[
            // `isRec` is what structure eta, unit-like equality and eta
            // inside iota ask through `is_structure_like`.
            ("ProjRecIndexed.Stream", "isRec", |b| {
                b.types[0].is_rec = false
            }),
            ("ProjRecIndexed.Tagged", "isRec", |b| {
                b.types[0].is_rec = true
            }),
            ("ProjRecIndexed.Tree", "isRec", |b| {
                b.types[0].is_rec = false
            }),
            // `Stream` is reflexive (`tail : Unit → Stream`); `Tagged` is not.
            ("ProjRecIndexed.Stream", "isReflexive", |b| {
                b.types[0].is_reflexive = false
            }),
            ("ProjRecIndexed.Tagged", "isReflexive", |b| {
                b.types[0].is_reflexive = true
            }),
            ("ProjRecIndexed.Stream", "all", |b| {
                b.types[0].all.push(Name::str("Nat"))
            }),
            ("ProjRecIndexed.Stream", "all", |b| b.types[0].all.clear()),
            // `numNested`, in both directions: flagged on a family with no
            // nested occurrence, cleared on a family that has one.
            ("ProjRecIndexed.Stream", "numNested", |b| {
                b.types[0].num_nested = 1
            }),
            ("ProjRecIndexed.Tree", "numNested", |b| {
                b.types[0].num_nested = 0
            }),
            ("ProjRecIndexed.Tree", "numNested", |b| {
                b.types[0].num_nested = 2
            }),
            // One `isUnsafe` flag for the whole declaration.
            ("ProjRecIndexed.Stream", "isUnsafe", |b| {
                b.ctors[0].is_unsafe = true
            }),
            ("ProjRecIndexed.Tree", "isUnsafe", |b| {
                b.recs[0].is_unsafe = true
            }),
            // The kernel derives one recursor per type.
            ("ProjRecIndexed.Stream", "recursor record", |b| {
                b.recs.clear()
            }),
            // The nested path: none of these was compared before.
            ("ProjRecIndexed.Tree", "numIndices", |b| {
                b.types[0].num_indices = 1
            }),
            ("ProjRecIndexed.Tree", "cidx", |b| b.ctors[0].cidx = 1),
            ("ProjRecIndexed.Tree", "numParams", |b| {
                b.ctors[0].num_params = 1
            }),
            ("ProjRecIndexed.Tree", "numFields", |b| {
                b.ctors[0].num_fields = 3
            }),
            ("ProjRecIndexed.Tree", "induct", |b| {
                b.ctors[0].induct = Name::str("Nat")
            }),
            ("ProjRecIndexed.Tree", "levelParams", |b| {
                b.ctors[0].common.level_params = vec![Name::str("u")]
            }),
        ],
    );
    // A mutual family: every type's `all` is the whole group, in order.
    assert_each_lie_is_rejected(
        "Tree_Forest",
        &[
            ("Tree", "all", |b| b.types[0].all.truncate(1)),
            ("Tree", "all", |b| b.types[1].all.reverse()),
        ],
    );
}

/// A synthetic bundle for `inductive T : Type` with no constructor that the
/// file flags as nested — a flag that is not true of it.
fn nested_flagged_bundle() -> ExportDecl {
    ExportDecl::Inductive(InductiveBundle {
        types: vec![InductiveVal {
            common: ConstantVal {
                name: Name::str("T"),
                level_params: vec![],
                ty: Expr::Sort(Level::succ(Level::zero())),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("T")],
            ctors: vec![],
            num_nested: 1,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }],
        ctors: vec![],
        recs: vec![],
    })
}

#[test]
fn a_family_flagged_nested_with_no_nested_occurrence_is_rejected() {
    // `inductive T : Type` with no constructor and `numNested = 1`: the
    // nested stage finds nothing to un-nest, and the family checks as an
    // ordinary one — so the file's count is not the kernel's. That is a
    // rejection like any other field that differs from the derivation, not
    // the unsupported feature (which is for nesting the kernel cannot
    // un-nest), and a dependent is rejected with it.
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let entry = replayer.replay(&nested_flagged_bundle());
    assert_rejected_for(
        &entry.outcome,
        "numNested",
        "a nested flag on a family that is not nested",
    );
    assert!(!replayer.env().contains(&Name::str("T")));
    assert_rejected_for(
        &replayer.replay(&mentions("usesT", Name::str("T"))).outcome,
        "depends on rejected declaration 'T'",
        "a dependent of the rejected family",
    );
}

// ---------------------------------------------------------------------------
// A record of a bundle is tied to the derived item it names.
//
// The key by which a record is paired with what the kernel derives is its
// NAME: a type record with the type of that name, a constructor record with
// the entry of that name in its type's list (whose position is its `cidx`),
// a recursor record with the recursor Lean's kernel gives that name
// (`T.rec` for a type of the declaration; `<first type>.rec_<k>` for the
// `k`-th auxiliary type of a nested family), a rule with the constructor at
// its position. Each derived item is matched by exactly one record and each
// record matches exactly one item.
// ---------------------------------------------------------------------------

/// `prefix.last` for a dotted prefix.
fn dotted(path: &str) -> Name {
    Name::from_str(path)
}

/// Rename constants throughout the recursor records of `b` — in their names,
/// in their types and in the right-hand sides of their rules. The pairs are
/// applied at once (through names no declaration has), so a pair of
/// opposite renamings exchanges two names.
fn rename_in_recursors(b: &mut InductiveBundle, map: &[(Name, Name)]) {
    use oxilean_kernel::expr_util::replace_expr;
    let rename = |e: &Expr, from: &Name, to: &Name| -> Expr {
        replace_expr(e, &mut |sub, _| match sub {
            Expr::Const(n, levels) if n == from => Some(Expr::Const(to.clone(), levels.clone())),
            _ => None,
        })
    };
    let through = |e: &Expr| -> Expr {
        let mut cur = e.clone();
        for (i, (from, _)) in map.iter().enumerate() {
            cur = rename(&cur, from, &Name::str(format!("«tmp{i}»")));
        }
        for (i, (_, to)) in map.iter().enumerate() {
            cur = rename(&cur, &Name::str(format!("«tmp{i}»")), to);
        }
        cur
    };
    for r in &mut b.recs {
        if let Some((_, to)) = map.iter().find(|(from, _)| *from == r.common.name) {
            r.common.name = to.clone();
        }
        r.common.ty = through(&r.common.ty);
        for rule in &mut r.rules {
            rule.rhs = through(&rule.rhs);
        }
    }
}

#[test]
fn a_recursor_record_is_the_recursor_of_the_type_it_is_named_after() {
    // The mutual, non-nested family `Tree` / `Forest` (ordinary path). The
    // kernel compared a recursor record with ANY derived recursor of the
    // family, and the replay only counted the records: each of the first
    // three edits below was accepted (0.1.3 through the first 0.1.5 patch).
    assert_each_lie_is_rejected(
        "Tree_Forest",
        &[
            // `Forest.rec` is a copy of `Tree.rec` under `Forest.rec`'s name:
            // two recursors of `Tree`, none of `Forest`.
            (
                "Tree",
                "'Forest.rec' does not match the kernel-derived recursor for 'Forest'",
                |b| {
                    let mut copy = b.recs[0].clone();
                    copy.common.name = b.recs[1].common.name.clone();
                    b.recs[1] = copy;
                },
            ),
            // The two names exchanged.
            (
                "Tree",
                "does not match the kernel-derived recursor for",
                |b| {
                    let (tree, forest) =
                        (b.recs[0].common.name.clone(), b.recs[1].common.name.clone());
                    b.recs[0].common.name = forest;
                    b.recs[1].common.name = tree;
                },
            ),
            // The two names exchanged throughout the two records (every
            // reference in their types and rules follows): each record is
            // then a derived recursor under the other's name.
            (
                "Tree",
                "does not match the kernel-derived recursor for",
                |b| {
                    let (tree, forest) = (dotted("Tree.rec"), dotted("Forest.rec"));
                    rename_in_recursors(b, &[(tree.clone(), forest.clone()), (forest, tree)]);
                },
            ),
            // A recursor named after no type of the family: the replay
            // refuses a record under a name Lean's kernel never gives before
            // the kernel derives anything (the kernel itself refuses it as
            // "not named after a type of its family").
            (
                "Tree",
                "recursor record 'Forest.recx' of inductive bundle 'Tree' is not named as Lean's \
                 kernel names a recursor of this declaration: expected Tree.rec or Forest.rec",
                |b| b.recs[1].common.name = dotted("Forest.recx"),
            ),
            // Two records of one name: the count is right, the second
            // declares `Tree.rec` again.
            ("Tree", "duplicate declaration: Tree.rec", |b| {
                b.recs[1] = b.recs[0].clone()
            }),
        ],
    );
    // The recursor of a single type under another name, its own references
    // renamed with it (`Stream.rec` -> `Stream.recursor`): the kernel took
    // the record's own name for the derived recursor's. (Refused by the
    // replay's name rule before the kernel sees it.)
    assert_each_lie_is_rejected(
        "ProjRecIndexed",
        &[(
            "ProjRecIndexed.Stream",
            "recursor record 'ProjRecIndexed.Stream.recursor' of inductive bundle \
             'ProjRecIndexed.Stream' is not named as Lean's kernel names a recursor",
            |b| {
                let rec = dotted("ProjRecIndexed.Stream.rec");
                rename_in_recursors(b, &[(rec, dotted("ProjRecIndexed.Stream.recursor"))]);
            },
        )],
    );
}

#[test]
fn a_recursor_under_a_siblings_name_cannot_stand_in_for_the_sibling() {
    // What the lie buys. `claims : <the type of Tree.rec> := Forest.rec`
    // says that the constant `Forest.rec` eliminates a `Tree`. In the
    // truthful environment the kernel refuses it (and so does Lean's). In a
    // file whose `Forest.rec` record is a renamed copy of `Tree.rec` it
    // verified: the bundle was accepted as recorded.
    let claims = |decls: &mut [ExportDecl]| -> ExportDecl {
        let b = bundle_of(decls, "Tree");
        let tree_rec = b.recs[0].clone();
        let levels = tree_rec
            .common
            .level_params
            .iter()
            .cloned()
            .map(Level::param)
            .collect();
        ExportDecl::Definition(DefinitionVal {
            common: ConstantVal {
                name: Name::str("claims"),
                level_params: tree_rec.common.level_params.clone(),
                ty: tree_rec.common.ty.clone(),
            },
            value: Expr::Const(dotted("Forest.rec"), levels),
            hints: ReducibilityHint::Abbrev,
            safety: DefinitionSafety::Safe,
            all: vec![Name::str("claims")],
        })
    };

    // Truthful: the bundle checks, the claim is refused by the kernel.
    let mut decls = fixture_decls("Tree_Forest");
    let claim = claims(&mut decls);
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);
    assert!(verdict(&outcomes, "Tree").is_checked());
    let truthful = replayer.replay(&claim).outcome;
    assert!(
        truthful.is_rejected(),
        "Forest.rec does not have the type of Tree.rec: {truthful:?}"
    );

    // The lie: the bundle is refused, and the claim with it.
    let mut decls = fixture_decls("Tree_Forest");
    let claim = claims(&mut decls);
    {
        let b = bundle_of(&mut decls, "Tree");
        let mut copy = b.recs[0].clone();
        copy.common.name = b.recs[1].common.name.clone();
        b.recs[1] = copy;
    }
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);
    assert_rejected_for(
        verdict(&outcomes, "Tree"),
        "does not match the kernel-derived recursor for 'Forest'",
        "the bundle whose Forest.rec is a copy of Tree.rec",
    );
    assert_rejected_for(
        &replayer.replay(&claim).outcome,
        "depends on rejected declaration",
        "a declaration that uses the copied recursor as Forest's",
    );
}

#[test]
fn the_rules_of_a_recursor_record_are_its_constructors_in_order() {
    // A rule is paired with a constructor by its place: rule `i` is the
    // rule of the type's `i`-th constructor, as Lean's kernel writes them.
    // The first edit (the same two rules in the other order) was accepted:
    // a rule used to be looked up by its constructor's name.
    assert_each_lie_is_rejected(
        "Tree_Forest",
        &[
            ("Tree", "'Tree.rec' does not match", |b| {
                b.recs[0].rules.reverse()
            }),
            // The `ctor` names of the two rules exchanged.
            ("Tree", "'Tree.rec' does not match", |b| {
                let first = b.recs[0].rules[0].ctor.clone();
                b.recs[0].rules[0].ctor = b.recs[0].rules[1].ctor.clone();
                b.recs[0].rules[1].ctor = first;
            }),
            // A rule that names a constructor of the other type.
            ("Tree", "'Tree.rec' does not match", |b| {
                b.recs[0].rules[0].ctor = dotted("Forest.nil")
            }),
            // One rule twice, the other missing; and one rule short.
            ("Tree", "'Tree.rec' does not match", |b| {
                b.recs[0].rules[1] = b.recs[0].rules[0].clone()
            }),
            ("Tree", "'Tree.rec' does not match", |b| {
                b.recs[0].rules.pop();
            }),
        ],
    );
    // The nested path: the rules of the auxiliary recursor.
    assert_each_lie_is_rejected(
        "ProjRecIndexed",
        &[(
            "ProjRecIndexed.Tree",
            "does not match the kernel-derived nested recursor",
            |b| b.recs[1].rules.reverse(),
        )],
    );
}

#[test]
fn a_constructor_record_is_tied_to_its_name_its_type_and_its_place() {
    // The constructor records of `Tree` / `Forest`, in the file's order:
    // `Tree.leaf` (0), `Tree.node` (1), `Forest.nil` (2), `Forest.cons` (3).
    // A constructor record is found by its NAME in the list of the type it
    // names as `induct`, and its `cidx` is its place there; these were
    // refused before this round as well, and are pinned here.
    assert_each_lie_is_rejected(
        "Tree_Forest",
        &[
            // The record of `Tree.leaf` under the name `Tree.node`.
            (
                "Tree",
                "lists constructor 'Tree.leaf' but the bundle does not contain it",
                |b| b.ctors[0].common.name = b.ctors[1].common.name.clone(),
            ),
            // Two records of one type with their names exchanged.
            ("Tree", "cidx", |b| {
                let first = b.ctors[0].common.name.clone();
                b.ctors[0].common.name = b.ctors[1].common.name.clone();
                b.ctors[1].common.name = first;
            }),
            // ... with their types exchanged (another declaration: the
            // recorded field counts and recursors are the old ones).
            ("Tree", "num_fields", |b| {
                let first = b.ctors[0].common.ty.clone();
                b.ctors[0].common.ty = b.ctors[1].common.ty.clone();
                b.ctors[1].common.ty = first;
            }),
            // Two records of the two types with their names exchanged.
            ("Tree", "Tree.leaf", |b| {
                let first = b.ctors[0].common.name.clone();
                b.ctors[0].common.name = b.ctors[2].common.name.clone();
                b.ctors[2].common.name = first;
            }),
            // A record that names the other type as its inductive.
            ("Tree", "Tree.leaf", |b| {
                b.ctors[0].induct = Name::str("Forest")
            }),
        ],
    );
}

#[test]
fn nested_recursor_records_carry_leans_names() {
    // `ProjRecIndexed.Tree` is nested through `List`: its bundle has
    // `Tree.rec` and the auxiliary `Tree.rec_1`. The nested path paired a
    // record with a derived recursor by the constructors its rules fire
    // on, and gave the derived recursor the record's name — so the first
    // two edits were accepted: a file could call the eliminator of
    // `List Tree` `Tree.rec`.
    assert_each_lie_is_rejected(
        "ProjRecIndexed",
        &[
            // The two names exchanged throughout the bundle.
            (
                "ProjRecIndexed.Tree",
                "does not match the kernel-derived nested recursor",
                |b| {
                    let (rec, aux) = (
                        dotted("ProjRecIndexed.Tree.rec"),
                        dotted("ProjRecIndexed.Tree.rec_1"),
                    );
                    rename_in_recursors(b, &[(rec.clone(), aux.clone()), (aux, rec)]);
                },
            ),
            // The auxiliary recursor under a name Lean does not give it.
            (
                "ProjRecIndexed.Tree",
                "has no recursor named 'ProjRecIndexed.Tree.rec_1'",
                |b| {
                    let aux = dotted("ProjRecIndexed.Tree.rec_1");
                    rename_in_recursors(b, &[(aux, dotted("ProjRecIndexed.Tree.rec_7"))]);
                },
            ),
            // The two record names exchanged and nothing else.
            (
                "ProjRecIndexed.Tree",
                "does not match the kernel-derived nested recursor",
                |b| {
                    let rec = b.recs[0].common.name.clone();
                    b.recs[0].common.name = b.recs[1].common.name.clone();
                    b.recs[1].common.name = rec;
                },
            ),
        ],
    );
}

#[test]
fn an_auxiliary_recursor_record_is_numbered_from_one() {
    // Lean's kernel names the recursor of the `k`-th auxiliary type of a
    // nested declaration `<first type>.rec_<k>`, `k` from 1, in decimal.
    // `ProjRecIndexed.Tree.rec_1` renamed `rec_0` throughout its bundle is
    // refused by the name rule -- before anything is derived -- with the
    // names expected.
    assert_each_lie_is_rejected(
        "ProjRecIndexed",
        &[(
            "ProjRecIndexed.Tree",
            "recursor record 'ProjRecIndexed.Tree.rec_0' of inductive bundle 'ProjRecIndexed.Tree' \
             is not named as Lean's kernel names a recursor of this declaration: expected \
             ProjRecIndexed.Tree.rec (or ProjRecIndexed.Tree.rec_<k> for the k-th auxiliary type of a \
             nested declaration)",
            |b| {
                let aux = dotted("ProjRecIndexed.Tree.rec_1");
                rename_in_recursors(b, &[(aux, dotted("ProjRecIndexed.Tree.rec_0"))]);
            },
        )],
    );
}

/// A replayer that holds `ProjRecIndexed` (Lean's `List` among it), every
/// declaration of it verified.
fn replayer_with_list() -> Replayer {
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    for (name, outcome) in replay_each(&mut replayer, &fixture_decls("ProjRecIndexed")) {
        assert!(outcome.is_checked(), "ProjRecIndexed: {name}: {outcome:?}");
    }
    replayer
}

/// The bundle this kernel derives for the mutual block nested through
/// `List`, `A | mk : List B → A ;  B | leaf : B | node : A → B`: the
/// kernel's derivation, restored over `List`, with its recursors under the
/// names Lean's kernel gives them -- `A.rec`, `B.rec`, and `A.rec_1` for
/// the auxiliary type `List B` (the first type's name for every auxiliary
/// recursor of the block).
fn mutual_nested_bundle(replayer: &Replayer) -> InductiveBundle {
    use oxilean_kernel::{
        check_and_derive_family_nested, restore_nested, BinderInfo, ConstantInfo, InductiveSpec,
    };
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let c = |n: &str| Expr::Const(dotted(n), vec![]);
    let list_of = |e: Expr| {
        Expr::App(
            Node::new(Expr::Const(dotted("List"), vec![Level::zero()])),
            Node::new(e),
        )
    };
    let arrow = |a: Expr, b: Expr| {
        Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(a),
            Node::new(b),
        )
    };
    let specs = vec![
        InductiveSpec::new(
            dotted("A"),
            type0.clone(),
            vec![(dotted("A.mk"), arrow(list_of(c("B")), c("A")))],
        ),
        InductiveSpec::new(
            dotted("B"),
            type0,
            vec![
                (dotted("B.leaf"), c("B")),
                (dotted("B.node"), arrow(c("A"), c("B"))),
            ],
        ),
    ];
    let (raw, exp) = match check_and_derive_family_nested(replayer.env(), &[], 0, &specs) {
        Ok((raw, Some(exp))) => (raw, exp),
        other => panic!("the block must derive as a nested one: {other:?}"),
    };
    let restored = restore_nested(raw, &exp);
    let lean_name = |position: usize| match specs.get(position) {
        Some(spec) => Name::mk_str(spec.name.clone(), "rec"),
        None => Name::mk_str(
            specs[0].name.clone(),
            format!("rec_{}", position + 1 - specs.len()),
        ),
    };
    let map: Vec<(Name, Name)> = restored
        .recursors
        .iter()
        .enumerate()
        .map(|(i, ci)| (ci.name().clone(), lean_name(i)))
        .collect();
    let mut bundle = InductiveBundle {
        types: vec![],
        ctors: vec![],
        recs: vec![],
    };
    for ci in restored.into_constant_infos() {
        match ci {
            ConstantInfo::Inductive(v) => bundle.types.push(v),
            ConstantInfo::Constructor(v) => bundle.ctors.push(v),
            ConstantInfo::Recursor(v) => bundle.recs.push(v),
            other => panic!("not part of a family: {other:?}"),
        }
    }
    rename_in_recursors(&mut bundle, &map);
    bundle
}

#[test]
fn an_auxiliary_recursor_record_is_named_under_the_first_type_of_a_mutual_block() {
    // In a mutual nested block every auxiliary recursor is named under the
    // FIRST type of the block. The bundle as Lean's kernel makes it
    // (`A.rec`, `B.rec`, `A.rec_1`) verifies; the same with `A.rec_1`
    // renamed `B.rec_1` throughout -- a name of the shape `rec_<k>` under
    // another type of the block -- is refused by the name rule, before
    // anything is derived, and nothing of it is installed.
    let mut replayer = replayer_with_list();
    let honest = mutual_nested_bundle(&replayer);
    let rec_names: Vec<String> = honest
        .recs
        .iter()
        .map(|r| r.common.name.to_string())
        .collect();
    assert_eq!(
        rec_names,
        ["A.rec", "B.rec", "A.rec_1"],
        "the records of the block"
    );
    let outcome = replayer
        .replay(&ExportDecl::Inductive(honest.clone()))
        .outcome;
    assert!(
        outcome.is_checked(),
        "the block as the kernel makes it: {outcome:?}"
    );
    assert!(replayer.env().contains(&dotted("A.rec_1")));

    let mut lie = honest;
    rename_in_recursors(&mut lie, &[(dotted("A.rec_1"), dotted("B.rec_1"))]);
    let mut replayer = replayer_with_list();
    assert_rejected_for(
        &replayer.replay(&ExportDecl::Inductive(lie)).outcome,
        "recursor record 'B.rec_1' of inductive bundle 'A' is not named as Lean's kernel names a \
         recursor of this declaration: expected A.rec or B.rec (or A.rec_<k> for the k-th auxiliary \
         type of a nested declaration)",
        "the auxiliary recursor of a mutual nested block under its second type",
    );
    for n in [
        "A", "B", "A.mk", "B.leaf", "B.node", "A.rec", "B.rec", "A.rec_1", "B.rec_1",
    ] {
        assert!(
            !replayer.env().contains(&dotted(n)),
            "{n} must not be in the environment"
        );
    }
}

// ---------------------------------------------------------------------------
// A bundle cut short by a resource limit leaves nothing behind.
// ---------------------------------------------------------------------------

#[test]
fn a_bundle_cut_short_by_a_resource_limit_leaves_nothing_behind() {
    use oxilean_export::RESOURCE_LIMIT;
    use oxilean_kernel::{check_and_derive_family, fuel, Environment, InductiveSpec};

    // `Tree` / `Forest` (two types, four constructors, two recursors) under
    // every per-declaration budget from nothing to enough, in even steps.
    // The members are checked one after the other against the environment
    // that holds the earlier ones, so a budget that runs out between two
    // members has found some of them added.
    let decls = fixture_decls("Tree_Forest");
    let ExportDecl::Inductive(bundle) = &decls[0] else {
        panic!("Tree_Forest is one inductive bundle");
    };
    let names: Vec<Name> = bundle
        .types
        .iter()
        .map(|t| t.common.name.clone())
        .chain(bundle.ctors.iter().map(|c| c.common.name.clone()))
        .chain(bundle.recs.iter().map(|r| r.common.name.clone()))
        .collect();

    // What the derivation alone costs, and what the whole replay costs.
    let specs: Vec<InductiveSpec> = bundle
        .types
        .iter()
        .map(|t| {
            let ctors = t
                .ctors
                .iter()
                .filter_map(|c| bundle.ctors.iter().find(|cv| &cv.common.name == c))
                .map(|cv| (cv.common.name.clone(), cv.common.ty.clone()))
                .collect();
            InductiveSpec::new(t.common.name.clone(), t.common.ty.clone(), ctors)
        })
        .collect();
    fuel::set_budget(Some(u64::MAX / 2));
    check_and_derive_family(
        &Environment::new(),
        &bundle.types[0].common.level_params,
        bundle.types[0].num_params,
        &specs,
    )
    .expect("the family derives");
    let derivation = fuel::used();
    let mut unlimited = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    unlimited.set_per_decl_fuel(Some(u64::MAX / 2));
    assert!(unlimited.replay(&decls[0]).outcome.is_checked());
    let whole = fuel::used();
    fuel::set_budget(None);
    assert!(
        derivation > 0 && derivation < whole,
        "the members are checked after the derivation ({derivation} of {whole} units)"
    );

    // `usesTree : <the type of Tree> := Tree`, a dependent that names the
    // family's first type and nothing else of the file.
    let tree = &bundle.types[0];
    let uses_tree = ExportDecl::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::str("usesTree"),
            level_params: tree.common.level_params.clone(),
            ty: tree.common.ty.clone(),
        },
        value: Expr::Const(
            tree.common.name.clone(),
            tree.common
                .level_params
                .iter()
                .cloned()
                .map(Level::param)
                .collect(),
        ),
        hints: ReducibilityHint::Abbrev,
        safety: DefinitionSafety::Safe,
        all: vec![Name::str("usesTree")],
    });

    let step = (whole / 400).max(1);
    let mut cut_among_the_members = 0;
    let mut budget = 1;
    while budget <= whole + step {
        let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
        replayer.set_per_decl_fuel(Some(budget));
        let outcome = replayer.replay(&decls[0]).outcome;
        let dependent = replayer.replay(&uses_tree).outcome;
        match outcome {
            ReplayOutcome::Checked => {
                assert!(replayer.env().contains(&Name::str("Tree")));
                assert!(dependent.is_checked(), "budget {budget}: {dependent:?}");
            }
            ReplayOutcome::Unsupported { feature } => {
                assert_eq!(feature, RESOURCE_LIMIT, "budget {budget}");
                for n in &names {
                    assert!(
                        !replayer.env().contains(n),
                        "budget {budget}: {n} was left behind by a bundle that was cut short"
                    );
                }
                assert_eq!(
                    dependent,
                    ReplayOutcome::Unsupported {
                        feature: DEFERRED_DEPENDENCY
                    },
                    "budget {budget}: a dependent of the bundle that was cut short"
                );
                if budget > derivation {
                    cut_among_the_members += 1;
                }
            }
            ReplayOutcome::Rejected { reason } => {
                panic!("budget {budget}: a resource limit is never a rejection: {reason}")
            }
        }
        budget += step;
    }
    assert!(
        cut_among_the_members > 0,
        "no budget between the derivation's cost ({derivation}) and the whole ({whole}) cut the \
         bundle short: the sweep did not reach the members"
    );
}

// ---------------------------------------------------------------------------
// A declaration Lean's kernel refuses is refused whatever records come
// with it.
// ---------------------------------------------------------------------------

/// The bundle a file would carry for a family: the kernel's own derivation
/// WITHOUT its checks, so that it exists for a family the checks refuse.
fn bundle_as_derived(specs: &[oxilean_kernel::InductiveSpec]) -> ExportDecl {
    use oxilean_kernel::{derive_family_unchecked, ConstantInfo};
    let family = derive_family_unchecked(&[], 0, specs).unwrap_or_else(|e| panic!("derive: {e:?}"));
    let mut bundle = InductiveBundle {
        types: vec![],
        ctors: vec![],
        recs: vec![],
    };
    for ci in family.into_constant_infos() {
        match ci {
            ConstantInfo::Inductive(v) => bundle.types.push(v),
            ConstantInfo::Constructor(v) => bundle.ctors.push(v),
            ConstantInfo::Recursor(v) => bundle.recs.push(v),
            other => panic!("not part of a family: {other:?}"),
        }
    }
    ExportDecl::Inductive(bundle)
}

#[test]
fn an_inductive_whose_index_mentions_it_is_rejected_with_its_recursor_supplied() {
    use oxilean_kernel::InductiveSpec;
    // `inductive I : Prop → Prop | mk : I (I True → False)`: the type
    // occurs in the index of its own constructor's result. Lean's kernel
    // refuses the declaration ("invalid return type for 'I.mk'"), so no
    // Lean export carries it; with propositional extensionality it is
    // inconsistent. This kernel's derivation accepted it (0.1.3 through
    // the first 0.1.5 patch), so a file that supplied the recursor the
    // derivation gives had the family verified.
    let prop = Expr::Sort(Level::zero());
    let c = |n: &str| Expr::Const(dotted(n), vec![]);
    let app = |f: Expr, a: Expr| Expr::App(Node::new(f), Node::new(a));
    let arrow = |a: Expr, b: Expr| {
        Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Node::new(a),
            Node::new(b),
        )
    };
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    for spec in [
        InductiveSpec::new(
            dotted("True"),
            prop.clone(),
            vec![(dotted("True.intro"), c("True"))],
        ),
        InductiveSpec::new(dotted("False"), prop.clone(), vec![]),
    ] {
        let outcome = replayer.replay(&bundle_as_derived(&[spec])).outcome;
        assert!(outcome.is_checked(), "the prelude: {outcome:?}");
    }
    let i_true = app(c("I"), c("True"));
    let paradox = InductiveSpec::new(
        dotted("I"),
        arrow(prop.clone(), prop),
        vec![(dotted("I.mk"), app(c("I"), arrow(i_true, c("False"))))],
    );
    assert_rejected_for(
        &replayer.replay(&bundle_as_derived(&[paradox])).outcome,
        "index terms that do not mention the family",
        "I : Prop → Prop | mk : I (I True → False)",
    );
    for n in ["I", "I.mk", "I.rec"] {
        assert!(
            !replayer.env().contains(&dotted(n)),
            "{n} must not be installed"
        );
    }
    assert_rejected_for(
        &replayer.replay(&mentions("usesI", dotted("I"))).outcome,
        "depends on rejected declaration 'I'",
        "a declaration that names the refused family",
    );
}

// ---------------------------------------------------------------------------
// A theorem states a proposition; a name is declared once (quot records);
// the kernel's own literal primitives.
// ---------------------------------------------------------------------------

/// `theorem name : ty := value`, written by hand.
fn theorem(name: &str, ty: Expr, value: Expr) -> ExportDecl {
    ExportDecl::Theorem(oxilean_kernel::TheoremVal {
        common: ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty,
        },
        value,
        all: vec![Name::str(name)],
    })
}

/// `axiom name : ty`, written by hand.
fn axiom(name: Name, ty: Expr) -> ExportDecl {
    ExportDecl::Axiom(AxiomVal {
        common: ConstantVal {
            name,
            level_params: vec![],
            ty,
        },
        is_unsafe: false,
    })
}

#[test]
fn a_theorem_record_whose_type_is_not_a_proposition_is_rejected() {
    // The two declarations Lean's kernel refuses with "type of theorem ...
    // is not a proposition": `theorem t01 : N := n` for a type `N`, and
    // `theorem t03 : Type := N`. Both verified (0.1.3 through the first
    // 0.1.5 patch): only the record KIND says "theorem", and the kernel
    // checked a theorem as it checks a definition.
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let n_ty = Expr::Const(Name::str("N"), vec![]);
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    assert!(replayer.replay(&type_axiom("N")).outcome.is_checked());
    assert!(replayer
        .replay(&axiom(Name::str("n"), n_ty.clone()))
        .outcome
        .is_checked());
    assert_rejected_for(
        &replayer
            .replay(&theorem(
                "t01",
                n_ty.clone(),
                Expr::Const(Name::str("n"), vec![]),
            ))
            .outcome,
        "type of theorem 't01' is not a proposition",
        "theorem t01 : N := n",
    );
    assert!(!replayer.env().contains(&Name::str("t01")));
    assert_rejected_for(
        &replayer
            .replay(&mentions("uses_t01", Name::str("t01")))
            .outcome,
        "depends on rejected declaration 't01'",
        "a definition that uses the refused theorem",
    );
    assert_rejected_for(
        &replayer
            .replay(&theorem("t03", type0.clone(), n_ty.clone()))
            .outcome,
        "type of theorem 't03' is not a proposition",
        "theorem t03 : Type := N",
    );
    // The same statement and value as a definition is what it always was.
    assert!(replayer
        .replay(&definition(
            "d01",
            n_ty,
            Expr::Const(Name::str("n"), vec![])
        ))
        .outcome
        .is_checked());
}

#[test]
fn a_second_quot_record_for_a_primitive_declares_it_again() {
    // The first `quot` record makes the kernel install all four primitives,
    // so the environment cannot say which of them the file has declared;
    // the replayer keeps that. A second record for `Quot` verified (and was
    // counted as one more verified declaration).
    let decls = fixture_decls("QuotPartialApp");
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);
    for (n, outcome) in &outcomes {
        assert!(outcome.is_checked(), "{n}: {outcome:?}");
    }
    let quot = decls
        .iter()
        .find(|d| matches!(d, ExportDecl::Quot(q) if q.common.name == Name::str("Quot")))
        .unwrap_or_else(|| panic!("the fixture has a quot record for Quot"));
    assert_rejected_for(
        &replayer.replay(quot).outcome,
        "declares 'Quot' again",
        "a second quot record for Quot",
    );
    // As with a second record of any other name, the name is then one the
    // file did not declare in a way the replay accepts.
    assert_rejected_for(
        &replayer
            .replay(&mentions("usesQuot", Name::str("Quot")))
            .outcome,
        "depends on rejected declaration 'Quot'",
        "a declaration after the second record",
    );
}

#[test]
fn a_file_that_declares_nat_pred_is_believed_about_it_and_not_the_reducer() {
    // The reducer has rules of its own for `Nat.pred`, `Nat.blt`,
    // `Nat.log2` and three `String` operations: they belong to the builtin
    // environment, which declares these operations without a definition.
    // Lean's kernel computes none of them by name. In a replayed file
    // `Nat.pred` is what the file declares — here an axiom, so nothing
    // computes it, and `Nat.pred 5 = 4` by `Eq.refl` must not check (it
    // verified, 0.1.3 through the first 0.1.5 patch).
    let decls = fixture_decls("QuotPartialApp");
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    for (n, outcome) in replay_each(&mut replayer, &decls) {
        assert!(outcome.is_checked(), "{n}: {outcome:?}");
    }
    let nat = Expr::Const(Name::str("Nat"), vec![]);
    let pred = dotted("Nat.pred");
    assert!(replayer
        .replay(&axiom(
            pred.clone(),
            Expr::Pi(
                oxilean_kernel::BinderInfo::Default,
                Name::str("n"),
                Node::new(nat.clone()),
                Node::new(nat.clone()),
            ),
        ))
        .outcome
        .is_checked());
    let lit = |n: u64| Expr::Lit(oxilean_kernel::Literal::nat(n));
    let app = |f: Expr, a: Expr| Expr::App(Node::new(f), Node::new(a));
    let one = Level::succ(Level::zero());
    // `@Eq Nat (Nat.pred 5) 4 := @Eq.refl Nat 4`
    let statement = app(
        app(
            app(Expr::Const(Name::str("Eq"), vec![one.clone()]), nat.clone()),
            app(Expr::Const(pred, vec![]), lit(5)),
        ),
        lit(4),
    );
    let proof = app(app(Expr::Const(dotted("Eq.refl"), vec![one]), nat), lit(4));
    let outcome = replayer
        .replay(&theorem("pred_five", statement, proof))
        .outcome;
    assert!(
        outcome.is_rejected(),
        "Nat.pred is an axiom of this file: {outcome:?}"
    );
}
