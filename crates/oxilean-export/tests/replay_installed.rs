//! Replay tests for what the replay INSTALLS: a term of the file only after
//! the kernel type-checked it, and the kernel's own term wherever the
//! kernel derives one (the other bundle tests are in `replay_bundles.rs`).
//!
//! * **A recursor record is never installed.** The recursor in the
//!   environment is the one the kernel derives, on both paths; the record
//!   is held against it — as written, or type-checked first and then by
//!   definitional equality. The fixtures `RecRuleTruth` / `RecRuleJunk` /
//!   `RecRuleControl` (see the fixture README): a rule right-hand side with
//!   an ILL-TYPED proof term used to pass a comparison by definitional
//!   equality alone, was installed, and proved `False` from no axiom.
//! * **A `quot` record and the axiom `Quot.sound`** are held against the
//!   kernel's canonical types the same way.
//! * **A recursor record is named as Lean's kernel names it** (`T.rec`, the
//!   component `rec` under `T`): the derived rules call the sibling
//!   recursors by name, and the fixtures `RecNameMixedStyle` /
//!   `RecNameFlat` (see the fixture README) show what a record under the
//!   one flat component `"T.rec"` did — up to a closed proof of `False`.
//!
//! (That a nested bundle's types and constructors are the declared ones is
//! pinned in the kernel: `oxilean-kernel`'s `derived_recursor` tests.)

use oxilean_kernel::Node;

use oxilean_export::{read_str, replay_file, ExportDecl, InductiveBundle, ReplayOutcome, Replayer};
use oxilean_kernel::{same_up_to_binders, BinderInfo, Expr, Level, Name};

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

/// The three-bucket split of a replayed file.
fn split(decls: &[ExportDecl]) -> (usize, usize, usize) {
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, decls);
    let count = |f: fn(&ReplayOutcome) -> bool| outcomes.iter().filter(|(_, o)| f(o)).count();
    (
        count(ReplayOutcome::is_checked),
        count(ReplayOutcome::is_unsupported),
        count(ReplayOutcome::is_rejected),
    )
}

/// `(fun (_ : Type) => e) (Prop Prop)` for a closed `e`: an ill-typed term
/// (a sort applied to a sort) that reduces to `e` in one step. Definitional
/// equality, which does not type-check what it reduces, finds it equal to
/// `e`.
fn behind_an_ill_typed_redex(e: &Expr) -> Expr {
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::App(
        Node::new(Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Node::new(type0),
            Node::new(e.clone()),
        )),
        Node::new(Expr::App(Node::new(prop.clone()), Node::new(prop))),
    )
}

/// One lie about one bundle of a fixture: the type whose bundle is edited,
/// the text the rejection must carry, and the edit.
type BundleLie = (&'static str, &'static str, fn(&mut InductiveBundle));

/// Apply each lie to a fresh copy of the fixture and assert that the bundle
/// is rejected for it and that nothing of the bundle is installed.
fn assert_each_lie_is_rejected(fixture_name: &str, lies: &[BundleLie]) {
    for (ty, needle, lie) in lies {
        let mut decls = fixture_decls(fixture_name);
        let bundle = bundle_of(&mut decls, ty);
        lie(bundle);
        let names: Vec<Name> = bundle
            .types
            .iter()
            .map(|t| t.common.name.clone())
            .chain(bundle.ctors.iter().map(|c| c.common.name.clone()))
            .chain(bundle.recs.iter().map(|r| r.common.name.clone()))
            .collect();
        let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
        let outcomes = replay_each(&mut replayer, &decls);
        let what = format!("{fixture_name}: {ty}, `{needle}`");
        assert_rejected_for(verdict(&outcomes, ty), needle, &what);
        for n in &names {
            assert!(
                !replayer.env().contains(n),
                "{what}: {n} must not be in the environment"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The three fixtures
// ---------------------------------------------------------------------------

#[test]
fn the_truthful_family_and_the_control_split_as_leans_kernel_judges_them() {
    // `RecRuleTruth`: B, P, T, Eq, False, True, get, val, fst, get_eq,
    // val_eq and `honest_val_is_t : val (T.mk B.t (P.mk B.t)) = B.t` verify;
    // `bad_val_is_f`, `bad_f_eq_t` and `boom : False` are refused, by Lean's
    // kernel and here (12 · 0 · 3).
    let report = replay_file(&read_str(&fixture("RecRuleTruth")).unwrap_or_else(|e| panic!("{e}")));
    assert_eq!(
        (report.checked(), report.unsupported(), report.rejected()),
        (12, 0, 3)
    );
    let rejected: Vec<String> = report
        .entries
        .iter()
        .filter(|e| e.outcome.is_rejected())
        .map(|e| e.name.to_string())
        .collect();
    assert_eq!(rejected, ["bad_val_is_f", "bad_f_eq_t", "boom"]);

    // `RecRuleControl`: an ill-typed term of the same making, as the value
    // of a theorem and of a definition, is refused where the kernel infers
    // its type (3 · 0 · 2).
    let report =
        replay_file(&read_str(&fixture("RecRuleControl")).unwrap_or_else(|e| panic!("{e}")));
    assert_eq!(
        (report.checked(), report.unsupported(), report.rejected()),
        (3, 0, 2)
    );
    for entry in &report.entries {
        let junk = matches!(
            entry.name.to_string().as_str(),
            "ctrl_thm_junk_is_proof" | "ctrl_def_junk"
        );
        assert_eq!(entry.outcome.is_rejected(), junk, "{}", entry.name);
    }
}

#[test]
fn a_recursor_rule_with_an_ill_typed_proof_term_is_refused_and_proves_nothing() {
    // `RecRuleJunk` is `RecRuleTruth` with ONE thing changed: the
    // right-hand side of the only rule of `T.rec`, from
    //     fun motive mk b h => mk b h
    // to
    //     fun motive mk b h => mk b (B.rec (motive := fun x => P x) (P.mk B.f) (P.mk B.f) b)
    // The inserted term is ill typed (its first minor premise must have the
    // type `P B.t` and has the type `P B.f`); its type read off its head is
    // `P b`, so proof irrelevance takes it for `h`. Through 0.1.4 — and up
    // to the last comparison by definitional equality alone — the record
    // was accepted AND INSTALLED, `val (T.mk B.t (P.mk B.t))` reduced to
    // `B.f`, and the file verified `bad_f_eq_t : B.f = B.t` and
    // `boom : False`: 14 verified · 0 unsupported · 1 rejected, the one
    // rejected being the honest `honest_val_is_t`.
    let decls = fixture_decls("RecRuleJunk");
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);
    assert_rejected_for(
        verdict(&outcomes, "T"),
        "recursor 'T.rec' does not match the kernel-derived recursor for 'T'",
        "the bundle whose recursor rule carries an ill-typed term",
    );
    for name in ["T", "T.mk", "T.rec"] {
        assert!(
            !replayer.env().contains(&Name::from_str(name)),
            "{name} must not be in the environment"
        );
    }
    for (name, root) in [
        ("val", "T"),
        ("fst", "T"),
        ("val_eq", "T"),
        ("honest_val_is_t", "val"),
        ("bad_val_is_f", "val"),
        ("bad_f_eq_t", "val_eq"),
        ("boom", "bad_f_eq_t"),
    ] {
        assert_rejected_for(
            verdict(&outcomes, name),
            &format!("depends on rejected declaration '{root}'"),
            name,
        );
    }
    for name in ["B", "P", "Eq", "False", "True", "get", "get_eq"] {
        assert!(verdict(&outcomes, name).is_checked(), "{name}");
    }
    assert_eq!(split(&decls), (7, 0, 8));
}

// ---------------------------------------------------------------------------
// What is installed for a recursor
// ---------------------------------------------------------------------------

/// The rule right-hand side `fun motive mk b h => mk b h` of `T.rec` with
/// its last argument replaced by `arg` (a term under the four binders).
fn rule_with_proof(rhs: &Expr, arg: Expr) -> Expr {
    match rhs {
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            ty.clone(),
            Node::new(rule_with_proof(body, arg)),
        ),
        Expr::App(f, _) => Expr::App(f.clone(), Node::new(arg)),
        other => panic!("not the rule of T.mk: {other}"),
    }
}

#[test]
fn what_is_installed_for_a_recursor_is_the_kernels_derivation() {
    // A WELL-TYPED right-hand side that is the kernel's only up to proof
    // irrelevance: `fun motive mk b h => mk b (P.mk b)`. The checked
    // comparison finds it to be the derived rule and the bundle is accepted
    // — and the rule in the environment is the kernel's, `mk b h`, not the
    // file's. The rest of the file is judged as on the truthful one.
    let mut decls = fixture_decls("RecRuleTruth");
    let bundle = bundle_of(&mut decls, "T");
    let truthful = bundle.recs[0].rules[0].rhs.clone();
    let p_mk_b = Expr::App(
        Node::new(Expr::Const(Name::from_str("P.mk"), vec![])),
        Node::new(Expr::BVar(1)),
    );
    let supplied = rule_with_proof(&truthful, p_mk_b);
    assert!(!same_up_to_binders(&supplied, &truthful));
    bundle.recs[0].rules[0].rhs = supplied.clone();

    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);
    assert!(
        verdict(&outcomes, "T").is_checked(),
        "a well-typed rule that is the kernel's up to proof irrelevance: {:?}",
        verdict(&outcomes, "T")
    );
    let installed = replayer
        .env()
        .get_recursor_val(&Name::from_str("T.rec"))
        .unwrap_or_else(|| panic!("T.rec must be installed"));
    assert!(
        same_up_to_binders(&installed.rules[0].rhs, &truthful),
        "the installed rule is the kernel's: {}",
        installed.rules[0].rhs
    );
    assert!(
        !same_up_to_binders(&installed.rules[0].rhs, &supplied),
        "the file's rule is not what is installed"
    );
    assert!(verdict(&outcomes, "honest_val_is_t").is_checked());
    assert!(verdict(&outcomes, "val_eq").is_checked());
    for name in ["bad_val_is_f", "bad_f_eq_t", "boom"] {
        assert!(verdict(&outcomes, name).is_rejected(), "{name}");
    }
}

#[test]
fn a_recursor_record_is_type_checked_before_it_is_compared() {
    // An ill-typed term that definitional equality reduces away, in a rule
    // and in the recursor's type, on the ordinary path (a single type and a
    // mutual block) and on the nested path (a type's recursor and the
    // auxiliary one). Each was accepted while the record was compared by
    // definitional equality alone.
    const NOT_DERIVED: &str = "does not match the kernel-derived recursor";
    const NOT_DERIVED_NESTED: &str = "does not match the kernel-derived nested recursor";
    assert_each_lie_is_rejected(
        "RecRuleTruth",
        &[
            ("T", NOT_DERIVED, |b| {
                b.recs[0].rules[0].rhs = behind_an_ill_typed_redex(&b.recs[0].rules[0].rhs);
            }),
            ("T", NOT_DERIVED, |b| {
                b.recs[0].common.ty = behind_an_ill_typed_redex(&b.recs[0].common.ty);
            }),
        ],
    );
    assert_each_lie_is_rejected(
        "Tree_Forest_full",
        &[
            ("Tree", NOT_DERIVED, |b| {
                b.recs[1].rules[0].rhs = behind_an_ill_typed_redex(&b.recs[1].rules[0].rhs);
            }),
            ("Tree", NOT_DERIVED, |b| {
                b.recs[0].common.ty = behind_an_ill_typed_redex(&b.recs[0].common.ty);
            }),
        ],
    );
    assert_each_lie_is_rejected(
        "ProjRecIndexed",
        &[
            ("ProjRecIndexed.Tree", NOT_DERIVED_NESTED, |b| {
                b.recs[0].rules[0].rhs = behind_an_ill_typed_redex(&b.recs[0].rules[0].rhs);
            }),
            ("ProjRecIndexed.Tree", NOT_DERIVED_NESTED, |b| {
                b.recs[1].rules[1].rhs = behind_an_ill_typed_redex(&b.recs[1].rules[1].rhs);
            }),
            ("ProjRecIndexed.Tree", NOT_DERIVED_NESTED, |b| {
                b.recs[1].common.ty = behind_an_ill_typed_redex(&b.recs[1].common.ty);
            }),
        ],
    );
    // The fixtures as they are verify whole.
    assert_eq!(split(&fixture_decls("Tree_Forest_full")), (37, 0, 0));
    assert_eq!(split(&fixture_decls("ProjRecIndexed")), (22, 0, 0));
}

// ---------------------------------------------------------------------------
// The quotient records and `Quot.sound`
// ---------------------------------------------------------------------------

#[test]
fn a_quot_record_and_quot_sound_are_type_checked_before_they_are_compared() {
    let name_of = |d: &ExportDecl| d.primary_name().to_string();

    // The record of `Quot.mk` with its type behind an ill-typed redex: the
    // canonical primitive is what the kernel installs whatever the record
    // says, and a record that is not it is refused.
    let mut decls = fixture_decls("QuotPartialApp");
    for d in decls.iter_mut() {
        if let ExportDecl::Quot(q) = d {
            if q.common.name.to_string() == "Quot.mk" {
                q.common.ty = behind_an_ill_typed_redex(&q.common.ty);
            }
        }
    }
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);
    assert_rejected_for(
        verdict(&outcomes, "Quot.mk"),
        "does not have the canonical type of its primitive",
        "a quot record whose type is ill typed and reduces to the canonical one",
    );
    assert!(verdict(&outcomes, "Quot").is_checked());

    // The axiom `Quot.sound` likewise: refused by the comparison, which now
    // type-checks the type before it reduces it.
    let mut decls = fixture_decls("QuotPartialApp");
    for d in decls.iter_mut() {
        if let ExportDecl::Axiom(a) = d {
            if a.common.name.to_string() == "Quot.sound" {
                a.common.ty = behind_an_ill_typed_redex(&a.common.ty);
            }
        }
    }
    assert!(decls.iter().any(|d| name_of(d) == "Quot.sound"));
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);
    assert_rejected_for(
        verdict(&outcomes, "Quot.sound"),
        "does not have the canonical quotient soundness type",
        "Quot.sound with a type that is ill typed and reduces to the canonical one",
    );
    assert!(!replayer.env().contains(&Name::from_str("Quot.sound")));

    // As exported, every record of the fixture verifies.
    assert_eq!(split(&fixture_decls("QuotPartialApp")), (66, 0, 0));
}

// ---------------------------------------------------------------------------
// The names of recursor records
// ---------------------------------------------------------------------------

/// The verdicts of a fixture, record by record, and the replayer after it.
fn replayed(name: &str) -> (Replayer, Vec<(String, ReplayOutcome)>) {
    let decls = fixture_decls(name);
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let outcomes = replay_each(&mut replayer, &decls);
    (replayer, outcomes)
}

fn tally(outcomes: &[(String, ReplayOutcome)]) -> (usize, usize, usize) {
    let count = |f: fn(&ReplayOutcome) -> bool| outcomes.iter().filter(|(_, o)| f(o)).count();
    (
        count(ReplayOutcome::is_checked),
        count(ReplayOutcome::is_unsupported),
        count(ReplayOutcome::is_rejected),
    )
}

#[test]
fn a_recursor_record_under_a_name_lean_never_gives_is_refused_and_proves_nothing() {
    // `RecNameMixedStyle` (see the fixture README): the mutual block
    //     A | mk : (b : B) → P b → C → A ;  C | base | step : A → C
    // as Lean's kernel makes it, with ONE record edited: the recursor of
    // `A` is named with the one flat component `"A.rec"` (printed `«A.rec»`)
    // instead of Lean's `A.rec`, the component `rec` under `A`, and its two
    // recursor constants with it. The kernel derives a family's recursors in
    // the style of the record's name, and the rules of a mutual family call
    // the siblings by name: the two records got rules that called `A.rec`
    // and `«C.rec»`, neither of them installed. The file then DEFINES the
    // hierarchical `A.rec` (a definition of another type), and `C.rec`'s
    // iota at `C.step` ran into it: `X : P B.t` reduced to a proof of
    // `P B.f`, and `bad_f_eq_t : B.f = B.t` and `boom : False` verified —
    // 13 verified · 0 unsupported · 0 rejected through 0.1.4 and in every
    // build before this one.
    let (replayer, outcomes) = replayed("RecNameMixedStyle");
    assert_rejected_for(
        verdict(&outcomes, "A"),
        "recursor record '«A.rec»' of inductive bundle 'A' is not named as Lean's kernel \
         names a recursor of this declaration: the recursor of 'A' is 'A.rec'",
        "the bundle with a flat recursor name",
    );
    for dependent in ["X", "get_X_is_f", "bad_f_eq_t", "boom"] {
        assert_rejected_for(
            verdict(&outcomes, dependent),
            "depends on rejected declaration",
            dependent,
        );
    }
    // Nothing of the bundle is installed — neither of its recursors, under
    // either spelling — and `A.rec`, the name Lean's kernel would have given
    // the recursor of `A`, is free: the definition of that name is checked
    // like any other and verifies (Lean's replay, which keeps its own
    // constants for the declaration, finds the name taken).
    let env = replayer.env();
    for n in ["A", "C", "A.mk", "C.base", "C.step", "C.rec"] {
        assert!(
            !env.contains(&Name::from_str(n)),
            "{n} must not be in the environment"
        );
    }
    assert!(!env.contains(&Name::str("A.rec")));
    assert!(!env.contains(&Name::str("C.rec")));
    assert!(env.get_recursor_val(&Name::from_str("A.rec")).is_none());
    assert!(verdict(&outcomes, "A.rec").is_checked());
    assert_eq!(tally(&outcomes), (8, 0, 5));

    // The same declarations with the bundle as Lean's kernel makes it: the
    // definition `A.rec` declares a name the kernel has given, and `X`'s
    // value, `get_X_is_f`, `bad_f_eq_t` and `boom` are refused, as by Lean's
    // kernel (9 · 0 · 4).
    let (_, truth) = replayed("RecNameMixedStyleTruth");
    assert_eq!(tally(&truth), (9, 0, 4));
    assert!(verdict(&truth, "A").is_checked());
    assert!(verdict(&truth, "boom").is_rejected());
}

#[test]
fn a_recursor_record_under_a_flat_name_is_refused_with_the_name_lean_gives() {
    // `RecNameFlat` (see the fixture README): the family of `RecRuleTruth`
    // with the record of `T.rec` named with the one flat component
    // `"T.rec"`. It was accepted — and installed under that name — so `val`
    // and `fst`, which name Lean's `T.rec`, were refused for an unknown
    // constant, with `val_eq` and `honest_val_is_t` after them, where Lean's
    // kernel accepts all four. Now the bundle is refused for its name and
    // the four are refused as its dependents.
    let (replayer, outcomes) = replayed("RecNameFlat");
    assert_rejected_for(
        verdict(&outcomes, "T"),
        "recursor record '«T.rec»' of inductive bundle 'T' is not named as Lean's kernel \
         names a recursor of this declaration: the recursor of 'T' is 'T.rec'",
        "the bundle with a flat recursor name",
    );
    for dependent in ["val", "fst", "val_eq", "honest_val_is_t"] {
        assert_rejected_for(
            verdict(&outcomes, dependent),
            "depends on rejected declaration",
            dependent,
        );
    }
    assert!(!replayer.env().contains(&Name::str("T.rec")));
    assert!(!replayer.env().contains(&Name::from_str("T.rec")));
    assert_eq!(tally(&outcomes), (7, 0, 8));
}
