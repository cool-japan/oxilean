//! Replay tests: every fixture is replayed into a fresh, **empty** environment
//! and the three-bucket split is pinned EXACTLY — the split is a product
//! number, and nothing may be silently skipped.
//!
//! Coverage:
//! * the fixtures of sound declarations fully checked (inductives, mutual groups, structures
//!   with eta/proj, quotients, kernel-re-derived recursors, nested
//!   inductives, point-free quotient maps);
//! * a rejected control (corrupted proof term → `Rejected`, not
//!   `Unsupported`);
//! * a negative fixture: a real export of a declaration Lean's own kernel
//!   rejects → `Rejected`;
//! * a second one: a projection out of a proof into data, and a closed proof
//!   of `False` built on it → both `Rejected`;
//! * projections out of recursive structures and out of an indexed
//!   one-constructor family, in ordinary Lean → all checked;
//! * (in `replay_bundles.rs`: a bundle is installed whole or not at all,
//!   what it says beyond its declaration is held to the kernel's
//!   derivation, and the cascade from a failed record goes by name);
//! * a long head-reduction chain: verified under a per-declaration budget,
//!   and without one a named `Unsupported` — never `Rejected`;
//! * a term that is small as a shared DAG and 2^60 nodes as a tree: verified
//!   for a few thousand units of the budget;
//! * `f a = f b` decided by comparing the arguments, for a function whose
//!   unfolding never ends — and the same shape with unequal arguments,
//!   which Lean's kernel rejects, `Rejected`;
//! * the two limits of a checking thread with a small stack — on the nesting
//!   of reductions and on the depth of terms: what is over one is a named
//!   `Unsupported`, and a false theorem Lean's kernel rejects is never
//!   `Checked` under any limit;
//! * `Quot.sound` axiom validation against the kernel's canonical type;
//! * nested inductives → the named `Unsupported` feature, with the
//!   dependency cascade;
//! * the corpus-scale streaming replay API (`replay_streaming`,
//!   `ReplayLimits`).

use oxilean_kernel::Node;
use std::time::Duration;

use oxilean_export::{
    read_str, replay_decl, replay_file, replay_streaming, ExportDecl, InductiveBundle, Limits,
    ReplayLimits, ReplayOutcome, Replayer, BUDGET_FEATURE, CORPUS_DECL_FUEL, DEFAULT_DECL_FUEL,
    DEFERRED_DEPENDENCY, NESTED_INDUCTIVES, REDUCTION_LIMIT, RESOURCE_LIMIT, TERM_DEPTH_LIMIT,
};
use oxilean_kernel::{
    init_builtin_env, AxiomVal, BinderInfo, ConstantVal, ConstructorVal, DefinitionSafety,
    DefinitionVal, Environment, Expr, InductiveVal, Level, Name, ReducibilityHint,
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
    assert_eq!(
        report.entries.len(),
        checked + unsupported + rejected,
        "{name}: one outcome per declaration"
    );
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

const META: &str = r#"{"meta":{"exporter":{"name":"lean4export","version":"3.1.0"},"format":{"version":"3.1.0"},"lean":{"githash":"x","version":"4.32.0-rc1"}}}"#;

// ---------------------------------------------------------------------------
// Full-fixture replays: every declaration checks, none skipped.
// ---------------------------------------------------------------------------

#[test]
fn simple_add_replay_all_checked() {
    // 15 defs + 1 thm + 7 inductive bundles = 23 declarations, all checked:
    // inductive replay (Wave 3b) removed the entire former unsupported bucket
    // (was 1 checked / 22 unsupported pre-Wave-3b).
    assert_split("simple_add", 23, 0, 0);

    // The theorem itself is the last declaration and is fully checked.
    let file = read_str(&fixture("simple_add")).unwrap_or_else(|e| panic!("parse: {e}"));
    let report = replay_file(&file);
    let last = report.entries.last().unwrap_or_else(|| panic!("non-empty"));
    assert_eq!(last.name.to_string(), "simple_add");
    assert_eq!(last.kind, "thm");
    assert!(last.outcome.is_checked());
}

#[test]
fn nat_add_succ_replay_all_checked() {
    // Format 3.0.0 export: 12 defs + 1 thm + 6 inductive bundles.
    assert_split("Nat.add_succ", 19, 0, 0);
}

#[test]
fn tree_forest_mutual_inductive_checked() {
    // A single record bundling the Tree/Forest mutual group.
    assert_split("Tree_Forest", 1, 0, 0);

    // The environment ends up with both types, all four constructors, and the
    // two kernel-verified recursors of the mutual group.
    let file = read_str(&fixture("Tree_Forest")).unwrap_or_else(|e| panic!("parse: {e}"));
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    for d in &file.decls {
        let entry = replayer.replay(d);
        assert!(
            entry.outcome.is_checked(),
            "{}: {:?}",
            entry.name,
            entry.outcome
        );
    }
    let env = replayer.env();
    for n in ["Tree", "Forest"] {
        assert!(
            env.get_inductive_val(&Name::from_str(n)).is_some(),
            "{n} must be an inductive"
        );
        let rec = Name::from_str(n).append_str("rec");
        assert!(
            env.get_recursor_val(&rec).is_some(),
            "{n}.rec must be installed (kernel-verified)"
        );
    }
}

#[test]
fn syntax_nested_inductive_all_checked() {
    // The real Lean core `Lean.Syntax` bundle — Array-then-List double nesting
    // (`numNested = 2`), exported recursors `Lean.Syntax.rec` / `.rec_1` (Array)
    // / `.rec_2` (List), 3 motives / 7 minors — plus its transitive
    // dependencies. Every declaration verifies: the nested-to-mutual derivation
    // reproduces Lean's exact exported recursors up to def-eq.
    assert_split("Syntax", 269, 0, 0);

    let file = read_str(&fixture("Syntax")).unwrap_or_else(|e| panic!("parse: {e}"));
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    for d in &file.decls {
        let entry = replayer.replay(d);
        assert!(
            entry.outcome.is_checked(),
            "{}: {:?}",
            entry.name,
            entry.outcome
        );
    }
    let env = replayer.env();
    let syntax = Name::from_str("Lean").append_str("Syntax");
    let iv = env
        .get_inductive_val(&syntax)
        .expect("Lean.Syntax must be installed as an inductive");
    assert_eq!(iv.num_nested, 2, "Lean.Syntax carries numNested = 2");
    for suffix in ["rec", "rec_1", "rec_2"] {
        assert!(
            env.get_recursor_val(&syntax.clone().append_str(suffix))
                .is_some(),
            "Lean.Syntax.{suffix} must be installed (kernel-verified nested recursor)"
        );
    }
}

#[test]
fn tree_forest_full_mutual_defs_checked() {
    // 30 defs (mutual Tree.size/Forest.size chain) + 7 inductive bundles.
    assert_split("Tree_Forest_full", 37, 0, 0);
}

#[test]
fn point_swap_swap_structures_proj_eta_checked() {
    // Structures, projections, and definitional structure eta
    // (`p ≡ Point.mk (Point.x p) (Point.y p)` under a binder).
    assert_split("point_swap_swap", 16, 0, 0);
}

#[test]
fn parity_is_even_quotients_all_checked() {
    // 112 defs + 40 thms + 4 quot records + 25 inductive bundles = 181.
    assert_split("Parity.isEven", 181, 0, 0);

    // All four quot primitives are CHECKED (installed by the kernel's
    // add_quot and validated against the canonical types).
    let file = read_str(&fixture("Parity.isEven")).unwrap_or_else(|e| panic!("parse: {e}"));
    let report = replay_file(&file);
    let quots: Vec<(String, bool)> = report
        .entries
        .iter()
        .filter(|e| e.kind == "quot")
        .map(|e| (e.name.to_string(), e.outcome.is_checked()))
        .collect();
    assert_eq!(quots.len(), 4);
    for (name, ok) in &quots {
        assert!(ok, "quot primitive {name} must check");
    }
    let names: Vec<&str> = quots.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["Quot", "Quot.mk", "Quot.lift", "Quot.ind"]);
}

#[test]
fn quot_partial_app_point_free_quotient_maps_all_checked() {
    // 1 axiom (`Quot.sound`) + 33 defs + 16 thms + 4 quot records + 12
    // inductive bundles = 66, all checked.
    //
    // The fixture is written in the point-free style of Mathlib's `Quot.map` /
    // `Quotient.map'`: a head step (a definition unfolding, a structure
    // projection) exposes a `Quot.lift` or a recursor that is short of its
    // major premise, and the major is an argument of the ENCLOSING
    // application. Before the head-reduction fix the reducer left such an
    // application unreduced and the three `rfl` theorems named below were
    // REJECTED (63 checked / 0 unsupported / 3 rejected) — the class of
    // Mathlib's `AddCon.addGroup._proof_1`.
    assert_split("QuotPartialApp", 66, 0, 0);

    let file = read_str(&fixture("QuotPartialApp")).unwrap_or_else(|e| panic!("parse: {e}"));
    let report = replay_file(&file);
    for name in [
        // A constant whose value is `Quot.lift f h`.
        "QuotPartialApp.succQ_mk",
        // An instance field holding `Quotient.map' f h`.
        "QuotPartialApp.neg_mk",
        // A constant whose value is `Nat.rec z s`.
        "QuotPartialApp.pred_succ",
        // Controls: these checked before the fix too.
        "QuotPartialApp.succQ'_mk",
        "QuotPartialApp.add_mk",
    ] {
        let entry = report
            .entries
            .iter()
            .find(|e| e.name.to_string() == name)
            .unwrap_or_else(|| panic!("{name} must be in the fixture"));
        assert_eq!(entry.kind, "thm", "{name} is a theorem");
        assert!(entry.outcome.is_checked(), "{name}: {:?}", entry.outcome);
    }
}

/// Replay `ReductionChain` with the given per-declaration fuel and return
/// the outcome of each declaration by name.
fn replay_reduction_chain(per_decl_fuel: Option<u64>) -> Vec<(String, ReplayOutcome)> {
    let file = read_str(&fixture("ReductionChain")).unwrap_or_else(|e| panic!("parse: {e}"));
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    replayer.set_per_decl_fuel(per_decl_fuel);
    file.decls
        .iter()
        .map(|d| {
            let entry = replayer.replay(d);
            (entry.name.to_string(), entry.outcome)
        })
        .collect()
}

#[test]
fn reduction_chain_verifies_under_a_per_declaration_budget() {
    // `chain_20000` (`chain 20000 = true` by `decide +kernel`) is one
    // head-reduction chain of tens of thousands of cheap steps. The reducer's
    // cap cuts it off after 10,000, and the check fails. Under a budget — the
    // CLI always arms one — the replayer then repeats the check with the cap
    // lifted, the budget bounds the chain, and the chain is far below it:
    // every declaration checks. Before the fix the first failure was the
    // verdict and this valid theorem was REJECTED (34 checked / 0
    // unsupported / 2 rejected).
    let outcomes = replay_reduction_chain(Some(DEFAULT_DECL_FUEL));
    assert_eq!(outcomes.len(), 36);
    let bad: Vec<&(String, ReplayOutcome)> =
        outcomes.iter().filter(|(_, o)| !o.is_checked()).collect();
    assert!(bad.is_empty(), "all 36 must check: {bad:#?}");
    for name in [
        "ReductionChain.chain_1000",
        "ReductionChain.chain_20000._proof_1_1",
        "ReductionChain.chain_20000",
    ] {
        assert!(
            outcomes.iter().any(|(n, _)| n == name),
            "{name} must be in the fixture"
        );
    }
}

#[test]
fn reduction_chain_without_a_budget_is_a_named_limit_never_a_rejection() {
    // With no budget armed nothing but the reducer's own cap would stop a
    // runaway chain, so the check is not repeated without it and
    // `chain_20000` is not reduced to the end. That is a resource limit, and
    // it must be REPORTED as one:
    // the theorem's proof lands in `unsupported` under REDUCTION_LIMIT, the
    // theorem that uses it follows as a deferred dependency, and nothing is
    // rejected. `chain_1000`, short enough for the cap, still checks.
    assert_split("ReductionChain", 34, 2, 0);

    let outcomes = replay_reduction_chain(None);
    let outcome = |name: &str| -> &ReplayOutcome {
        outcomes
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, o)| o)
            .unwrap_or_else(|| panic!("{name} must be in the fixture"))
    };
    assert!(outcome("ReductionChain.chain_1000").is_checked());
    assert_eq!(
        outcome("ReductionChain.chain_20000._proof_1_1"),
        &ReplayOutcome::Unsupported {
            feature: REDUCTION_LIMIT
        }
    );
    assert_eq!(
        outcome("ReductionChain.chain_20000"),
        &ReplayOutcome::Unsupported {
            feature: DEFERRED_DEPENDENCY
        }
    );
}

// ---------------------------------------------------------------------------
// Rejected control: a corrupted proof term must be REJECTED, not unsupported.
// ---------------------------------------------------------------------------

#[test]
fn corrupted_proof_term_is_rejected_not_unsupported() {
    // Corrupt the final theorem of simple_add: make its proof term be its own
    // statement (`"value"` index := `"type"` index). The statement is a Prop,
    // not a proof of itself, so the kernel must reject it.
    let original = fixture("simple_add");
    let corrupted: String = original
        .lines()
        .map(|line| {
            if line.starts_with("{\"thm\"") {
                let ty = extract_field(line, "\"type\":");
                line.replace(
                    &format!("\"value\":{}", extract_field(line, "\"value\":")),
                    &format!("\"value\":{ty}"),
                )
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(original, corrupted, "corruption must change the file");

    let file = read_str(&corrupted).unwrap_or_else(|e| panic!("parse: {e}"));
    let report = replay_file(&file);
    assert_eq!(report.entries.len(), 23);
    assert_eq!(
        (report.checked(), report.unsupported(), report.rejected()),
        (22, 0, 1),
        "exactly the corrupted theorem is rejected"
    );
    let last = report.entries.last().unwrap_or_else(|| panic!("non-empty"));
    assert_eq!(last.name.to_string(), "simple_add");
    match &last.outcome {
        ReplayOutcome::Rejected { reason } => {
            assert!(!reason.is_empty(), "rejection carries a reason");
        }
        other => panic!("corrupted proof must be REJECTED, got {other:?}"),
    }
}

/// Replay a fixture with the given per-declaration fuel; the outcome and the
/// fuel charged for each declaration, by name.
fn replay_with_fuel(name: &str, per_decl_fuel: Option<u64>) -> Vec<(String, ReplayOutcome, u64)> {
    let file = read_str(&fixture(name)).unwrap_or_else(|e| panic!("parse {name}: {e}"));
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    replayer.set_per_decl_fuel(per_decl_fuel);
    file.decls
        .iter()
        .map(|d| {
            let entry = replayer.replay(d);
            let used = oxilean_kernel::fuel::used();
            (entry.name.to_string(), entry.outcome, used)
        })
        .collect()
}

#[test]
fn shared_subterms_tower_checks_for_the_work_it_takes_not_its_size_as_a_tree() {
    // `tower` is sixty `let`s, each used twice by the next: a few hundred
    // shared nodes, 2^60 additions as a tree. Lean's kernel checks it at
    // once. Through 0.1.4's first performance pass the per-declaration budget
    // was charged for the TREE — a substitution that shared an untouched
    // subterm was billed that subterm's size as a tree — so `tower` ran out
    // of even the corpus budget (2^26) and was `unsupported`, with the
    // theorem deferred behind it (57 checked / 3 unsupported / 0 rejected).
    //
    // Under the DEFAULT budget, and for a vanishing part of it: what is
    // charged is the subterms a substitution reaches. (Always replayed under
    // a budget here: without one, a kernel that walked the tree again would
    // not fail this test, it would never return from it.)
    let outcomes = replay_with_fuel("SharedSubterms", Some(DEFAULT_DECL_FUEL));
    assert_eq!(outcomes.len(), 60);
    let bad: Vec<&(String, ReplayOutcome, u64)> = outcomes
        .iter()
        .filter(|(_, o, _)| !o.is_checked())
        .collect();
    assert!(bad.is_empty(), "all 60 must check: {bad:#?}");
    for name in [
        "SharedSubterms.tower",
        "SharedSubterms.tower_eq._proof_1_1",
        "SharedSubterms.tower_eq",
    ] {
        let (_, outcome, used) = outcomes
            .iter()
            .find(|(n, _, _)| n == name)
            .unwrap_or_else(|| panic!("{name} must be in the fixture"));
        assert!(outcome.is_checked(), "{name}: {outcome:?}");
        assert!(
            *used < 100_000,
            "{name} charged {used} units; a 60-level tower is a few thousand"
        );
    }
}

#[test]
fn congruence_first_equal_arguments_decide_without_unfolding() {
    // `slow 1000000000 (k + 0) = slow 1000000000 k` and
    // `(2^64 - 1) % USize.size = (2^64 - 1) % 2 ^ numBits`, both by `rfl`:
    // the same definition applied to definitionally equal arguments. Lean's
    // kernel compares the arguments before it unfolds anything. Reducing
    // both sides first — what this kernel did — unfolds a recursion that is
    // stuck at every step and compares the two results one unfolding at a
    // time: both theorems ran out of the corpus budget (182 checked / 2
    // unsupported / 1 rejected), as did the declarations of Lean core `Init`
    // that compare such terms.
    //
    // The fourth theorem of the file, `bad`, is the same shape with UNEQUAL
    // arguments; Lean's kernel rejects it, and so must this one. (Replayed
    // under a budget, as `SharedSubterms` is and for the same reason.)
    let outcomes = replay_with_fuel("CongruenceFirst", Some(DEFAULT_DECL_FUEL));
    assert_eq!(outcomes.len(), 185);
    let split = (
        outcomes.iter().filter(|(_, o, _)| o.is_checked()).count(),
        outcomes
            .iter()
            .filter(|(_, o, _)| o.is_unsupported())
            .count(),
        outcomes.iter().filter(|(_, o, _)| o.is_rejected()).count(),
    );
    assert_eq!(split, (184, 0, 1), "CongruenceFirst: split mismatch");
    for name in [
        "CongruenceFirst.same_function_equal_args",
        "CongruenceFirst.mod_stuck_divisor",
        "CongruenceFirst.short_fuel",
    ] {
        let (_, outcome, used) = outcomes
            .iter()
            .find(|(n, _, _)| n == name)
            .unwrap_or_else(|| panic!("{name} must be in the fixture"));
        assert!(outcome.is_checked(), "{name}: {outcome:?}");
        assert!(
            *used < 100_000,
            "{name} charged {used} units; comparing the arguments is a few hundred"
        );
    }
}

#[test]
fn congruence_first_unequal_arguments_are_rejected() {
    // `CongruenceFirst.bad : slow 5 k = slow 6 k := Eq.refl (slow 5 k)` —
    // a declaration Lean's own kernel REJECTS (added with
    // `debug.skipKernelTC`; see the fixture README). Comparing the
    // arguments of the two `slow` applications finds `5` and `6` unequal,
    // which decides nothing; the comparison that follows must find the two
    // sides different, under the default budget and under the corpus one.
    for per_decl_fuel in [Some(DEFAULT_DECL_FUEL), Some(CORPUS_DECL_FUEL)] {
        let outcomes = replay_with_fuel("CongruenceFirst", per_decl_fuel);
        let (_, outcome, _) = outcomes
            .iter()
            .find(|(n, _, _)| n == "CongruenceFirst.bad")
            .unwrap_or_else(|| panic!("CongruenceFirst.bad must be in the fixture"));
        assert!(
            matches!(outcome, ReplayOutcome::Rejected { .. }),
            "a declaration Lean's kernel rejects must be REJECTED              (budget {per_decl_fuel:?}), got {outcome:?}"
        );
        // Everything else in the file is sound and checks.
        for (name, outcome, _) in outcomes
            .iter()
            .filter(|(n, _, _)| n != "CongruenceFirst.bad")
        {
            assert!(outcome.is_checked(), "{name}: {outcome:?}");
        }
    }
}

/// Replay a fixture under the default budget on a thread of its own, with
/// the given limits on the nesting of reductions and on the depth of terms
/// (`None` = none, as on the thread of the command-line tool). The thread
/// has a deep stack: with no limit, how deep the kernel may recurse is a
/// matter of stack, and a test thread has two megabytes of it.
fn replay_with_stack_limits(
    name: &'static str,
    nesting: Option<u32>,
    depth: Option<u32>,
) -> Vec<(String, ReplayOutcome, u64)> {
    std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(move || {
            oxilean_kernel::fuel::set_reduction_nesting_limit(nesting);
            oxilean_kernel::fuel::set_term_depth_limit(depth);
            replay_with_fuel(name, Some(DEFAULT_DECL_FUEL))
        })
        .unwrap_or_else(|e| panic!("cannot spawn the replay thread: {e}"))
        .join()
        .unwrap_or_else(|_| panic!("the replay thread panicked"))
}

fn outcome_of<'a>(outcomes: &'a [(String, ReplayOutcome, u64)], name: &str) -> &'a ReplayOutcome {
    outcomes
        .iter()
        .find(|(n, _, _)| n == name)
        .map(|(_, outcome, _)| outcome)
        .unwrap_or_else(|| panic!("{name} must be in the fixture"))
}

fn split_of(outcomes: &[(String, ReplayOutcome, u64)]) -> (usize, usize, usize) {
    (
        outcomes.iter().filter(|(_, o, _)| o.is_checked()).count(),
        outcomes
            .iter()
            .filter(|(_, o, _)| o.is_unsupported())
            .count(),
        outcomes.iter().filter(|(_, o, _)| o.is_rejected()).count(),
    )
}

/// What must hold of a replay under ANY limit of a small stack: the one
/// declaration Lean's kernel rejects (`bad`) is never checked, nothing else
/// is ever rejected, and whatever is unsupported names one of `features`.
fn assert_a_limit_is_not_a_verdict(
    outcomes: &[(String, ReplayOutcome, u64)],
    bad: &str,
    features: &[&str],
    limits: &str,
) {
    for (name, outcome, _) in outcomes {
        match outcome {
            ReplayOutcome::Checked => assert_ne!(
                name, bad,
                "{limits}: a declaration Lean's kernel rejects was CHECKED"
            ),
            ReplayOutcome::Rejected { reason } => assert_eq!(
                name, bad,
                "{limits}: a sound declaration was REJECTED: {reason}"
            ),
            ReplayOutcome::Unsupported { feature } => assert!(
                features.contains(feature),
                "{limits}: {name} is unsupported for an unexpected reason: {feature}"
            ),
        }
    }
}

#[test]
fn nested_reduction_checks_on_a_deep_stack_and_its_false_theorem_is_rejected() {
    // `count 400 0 = 400` by kernel evaluation: a reduction nested about
    // four hundred deep (each step of the structural recursion is found
    // inside the previous one). With no limit on the nesting — the thread of
    // the command-line tool — it is checked, and `count 400 0 = 401`, which
    // Lean's kernel rejects and which takes the same four hundred levels to
    // find out, is rejected.
    let outcomes = replay_with_stack_limits("NestedReduction", None, None);
    assert_eq!(outcomes.len(), 65);
    assert_eq!(split_of(&outcomes), (64, 0, 1), "NestedReduction: split");
    for name in [
        "NestedReduction.deep._proof_1_1",
        "NestedReduction.deep",
        "NestedReduction.shallow",
    ] {
        assert!(outcome_of(&outcomes, name).is_checked(), "{name}");
    }
    assert!(
        matches!(
            outcome_of(&outcomes, "NestedReduction.bad"),
            ReplayOutcome::Rejected { .. }
        ),
        "a declaration Lean's kernel rejects must be REJECTED, got {:?}",
        outcome_of(&outcomes, "NestedReduction.bad")
    );
}

#[test]
fn a_limit_on_nested_reductions_is_a_named_limit_and_never_a_verdict() {
    // The limit a WebAssembly build of the checker sets (256): the reduction
    // four hundred deep is not carried out. The theorem that needs it is
    // unsupported — its proof by name, the theorem behind it as a
    // dependency — and so is the FALSE theorem: with the reduction cut
    // short the checker cannot tell `400` from `401`, and must say so.
    // Forty levels are within the limit.
    let outcomes = replay_with_stack_limits("NestedReduction", Some(256), None);
    assert_eq!(split_of(&outcomes), (62, 3, 0), "NestedReduction under 256");
    assert_eq!(
        *outcome_of(&outcomes, "NestedReduction.deep._proof_1_1"),
        ReplayOutcome::Unsupported {
            feature: REDUCTION_LIMIT
        }
    );
    assert_eq!(
        *outcome_of(&outcomes, "NestedReduction.deep"),
        ReplayOutcome::Unsupported {
            feature: DEFERRED_DEPENDENCY
        }
    );
    assert_eq!(
        *outcome_of(&outcomes, "NestedReduction.bad"),
        ReplayOutcome::Unsupported {
            feature: REDUCTION_LIMIT
        }
    );
    assert!(outcome_of(&outcomes, "NestedReduction.shallow").is_checked());

    // And under every limit, from none of the stack to all of it: the false
    // theorem is never checked and no sound declaration is ever rejected.
    for limit in [
        0, 1, 2, 3, 5, 8, 13, 21, 34, 44, 45, 55, 89, 144, 233, 377, 400, 401, 402, 403, 404, 405,
        406, 610, 1000,
    ] {
        let outcomes = replay_with_stack_limits("NestedReduction", Some(limit), None);
        assert_a_limit_is_not_a_verdict(
            &outcomes,
            "NestedReduction.bad",
            &[REDUCTION_LIMIT, DEFERRED_DEPENDENCY],
            &format!("nesting limit {limit}"),
        );
        if limit >= 610 {
            // Room for all of it: the verdicts of a deep stack.
            assert_eq!(split_of(&outcomes), (64, 0, 1), "nesting limit {limit}");
        }
    }
}

#[test]
fn deep_term_checks_on_a_deep_stack_and_its_false_theorem_is_rejected() {
    // `Nat.succ` applied three hundred times, written out as a term; that
    // the same tower over a definition of zero equals it; and — the theorem
    // Lean's kernel rejects — that it equals a tower one level taller.
    let outcomes = replay_with_stack_limits("DeepTerm", None, None);
    assert_eq!(outcomes.len(), 6);
    assert_eq!(split_of(&outcomes), (5, 0, 1), "DeepTerm: split");
    assert!(
        matches!(
            outcome_of(&outcomes, "DeepTerm.bad"),
            ReplayOutcome::Rejected { .. }
        ),
        "a declaration Lean's kernel rejects must be REJECTED, got {:?}",
        outcome_of(&outcomes, "DeepTerm.bad")
    );
    // The limits a WebAssembly build sets (256 nested reductions, terms 512
    // deep): three hundred levels are within the second, and the sound
    // declarations check. Telling the tower from the one a level taller
    // takes a reduction nested three hundred deep — over the first — so the
    // false theorem is neither rejected nor, ever, checked.
    let outcomes = replay_with_stack_limits("DeepTerm", Some(256), Some(512));
    assert_eq!(split_of(&outcomes), (5, 1, 0), "DeepTerm under (256, 512)");
    assert_eq!(
        *outcome_of(&outcomes, "DeepTerm.bad"),
        ReplayOutcome::Unsupported {
            feature: REDUCTION_LIMIT
        }
    );
}

#[test]
fn a_limit_on_term_depth_is_a_named_limit_and_never_a_verdict() {
    // A limit of a hundred levels: the three declarations with a tower in
    // them are not read into memory at all (the reader knows the depth of
    // an expression from its table), and each is unsupported by name — the
    // false theorem as well. What does not mention the tower is checked.
    let outcomes = replay_with_stack_limits("DeepTerm", None, Some(100));
    assert_eq!(split_of(&outcomes), (3, 3, 0), "DeepTerm under depth 100");
    for name in ["DeepTerm.tower", "DeepTerm.tower_eq", "DeepTerm.bad"] {
        assert_eq!(
            *outcome_of(&outcomes, name),
            ReplayOutcome::Unsupported {
                feature: TERM_DEPTH_LIMIT
            },
            "{name}"
        );
    }
    for name in ["Nat", "Eq", "DeepTerm.z"] {
        assert!(outcome_of(&outcomes, name).is_checked(), "{name}");
    }

    // Under every limit: the false theorem is never checked and no sound
    // declaration is ever rejected. (The tower is 301 levels, the two
    // equations 303.)
    for limit in [
        0, 1, 2, 3, 5, 10, 50, 100, 200, 299, 300, 301, 302, 303, 304, 305, 512, 4096,
    ] {
        let outcomes = replay_with_stack_limits("DeepTerm", None, Some(limit));
        assert_a_limit_is_not_a_verdict(
            &outcomes,
            "DeepTerm.bad",
            &[TERM_DEPTH_LIMIT, DEFERRED_DEPENDENCY],
            &format!("term-depth limit {limit}"),
        );
        let tower = outcome_of(&outcomes, "DeepTerm.tower");
        assert_eq!(tower.is_checked(), limit >= 301, "depth limit {limit}");
        if limit >= 303 {
            assert_eq!(split_of(&outcomes), (5, 0, 1), "term-depth limit {limit}");
        }
    }
}

#[test]
fn a_term_that_grows_past_the_depth_limit_while_checking_spends_the_budget() {
    // Nothing in `NestedReduction` is two hundred levels deep as written.
    // But `count 400 0` hands its second argument on with one more `+ 1`
    // around it at every step, and the kernel does not evaluate an argument
    // it has not been asked for: after four hundred steps that argument is a
    // term eight hundred levels deep.
    let outcomes = replay_with_stack_limits("NestedReduction", None, Some(200));
    if oxilean_kernel::fuel::TERM_DEPTH_TRACKED {
        // A kernel that tracks the depth of its terms stops when the term
        // crosses the limit: a named resource limit, for the sound theorem
        // and for the false one alike.
        assert_eq!(split_of(&outcomes), (62, 3, 0));
        assert_eq!(
            *outcome_of(&outcomes, "NestedReduction.deep._proof_1_1"),
            ReplayOutcome::Unsupported {
                feature: TERM_DEPTH_LIMIT
            }
        );
        assert_eq!(
            *outcome_of(&outcomes, "NestedReduction.bad"),
            ReplayOutcome::Unsupported {
                feature: TERM_DEPTH_LIMIT
            }
        );
        assert!(outcome_of(&outcomes, "NestedReduction.shallow").is_checked());
    } else {
        // One that does not (the default build) can only refuse what is too
        // deep as written; this file is not, and checks as with no limit.
        assert_eq!(split_of(&outcomes), (64, 0, 1));
    }
    // In either build, and under any limit: never a verdict.
    for limit in [0, 1, 4, 16, 64, 100, 150, 400, 799, 800, 801, 900, 2000] {
        let outcomes = replay_with_stack_limits("NestedReduction", None, Some(limit));
        assert_a_limit_is_not_a_verdict(
            &outcomes,
            "NestedReduction.bad",
            &[TERM_DEPTH_LIMIT, DEFERRED_DEPENDENCY],
            &format!("term-depth limit {limit}"),
        );
    }
    // Both limits at once, as a WebAssembly build has them.
    let outcomes = replay_with_stack_limits("NestedReduction", Some(256), Some(512));
    assert_a_limit_is_not_a_verdict(
        &outcomes,
        "NestedReduction.bad",
        &[TERM_DEPTH_LIMIT, REDUCTION_LIMIT, DEFERRED_DEPENDENCY],
        "nesting 256, term depth 512",
    );
    assert_eq!(split_of(&outcomes), (62, 3, 0));
}

#[test]
fn k_guard_collision_theorem_is_rejected() {
    // A real lean4export file whose one theorem Lean's own kernel REJECTS
    // (it was added with `debug.skipKernelTC`; see the fixture README).
    //
    // `KGuardCollision.bad` claims `Eq.rec m h = m` by `Eq.refl m` for a
    // hypothesis `h : (fun y1 y2 y3 => c) = (fun y1 y2 y3 => y3)`. That holds
    // by reflexivity only if K-like reduction fires on the variable `h`,
    // which requires its two endpoints to be definitionally equal — they are
    // not. The kernel checks that side condition in a nested checker seeded
    // with the surrounding locals; before the free-variable id fix the nested
    // def-eq opened `y3` with the id of the surrounding `c`, the endpoints
    // opened to the same variable, K fired, and this file replayed as
    // 4 checked / 0 unsupported / 0 rejected — a wrong ACCEPT.
    assert_split("KGuardCollision", 3, 0, 1);

    let file = read_str(&fixture("KGuardCollision")).unwrap_or_else(|e| panic!("parse: {e}"));
    let report = replay_file(&file);
    let bad = report
        .entries
        .iter()
        .find(|e| e.name.to_string() == "KGuardCollision.bad")
        .unwrap_or_else(|| panic!("KGuardCollision.bad must be in the fixture"));
    assert_eq!(bad.kind, "thm");
    assert!(
        matches!(bad.outcome, ReplayOutcome::Rejected { .. }),
        "a declaration Lean's kernel rejects must be REJECTED, got {:?}",
        bad.outcome
    );
    // Everything else in the file is sound and still checks.
    for entry in report
        .entries
        .iter()
        .filter(|e| e.name.to_string() != "KGuardCollision.bad")
    {
        assert!(
            entry.outcome.is_checked(),
            "{}: {:?}",
            entry.name,
            entry.outcome
        );
    }
}

#[test]
fn projection_out_of_a_proposition_is_rejected() {
    // A real lean4export file with two declarations Lean's own kernel
    // REJECTS (added with `debug.skipKernelTC`; see the fixture README).
    //
    // Both rest on `h.1` for `h : ∃ n : Nat, True` — `Expr.proj Exists 0 h`,
    // the witness read out of a proof. `ProjOutOfProp.wit` is that function
    // as a definition; `ProjOutOfProp.false_of_proj` is a closed proof of
    // `False` from it with no axiom: any two proofs of the existential are
    // definitionally equal, so their witnesses are, and the witnesses of
    // `⟨0, _⟩` and `⟨1, _⟩` reduce to `0` and `1`.
    //
    // Lean's kernel refuses the projection: out of a proposition only a
    // proposition may be projected. Before the kernel had that condition
    // this file replayed as 7 checked / 0 unsupported / 0 rejected — a wrong
    // ACCEPT of `False`.
    assert_split("ProjOutOfProp", 5, 0, 2);

    let file = read_str(&fixture("ProjOutOfProp")).unwrap_or_else(|e| panic!("parse: {e}"));
    let report = replay_file(&file);
    let bad = [
        ("ProjOutOfProp.wit", "def"),
        ("ProjOutOfProp.false_of_proj", "thm"),
    ];
    for (name, kind) in bad {
        let entry = report
            .entries
            .iter()
            .find(|e| e.name.to_string() == name)
            .unwrap_or_else(|| panic!("{name} must be in the fixture"));
        assert_eq!(entry.kind, kind);
        match &entry.outcome {
            ReplayOutcome::Rejected { reason } => assert!(
                reason.contains("invalid projection"),
                "{name} must be rejected for its projection, got: {reason}"
            ),
            other => panic!(
                "{name}: a declaration Lean's kernel rejects must be REJECTED, got {other:?}"
            ),
        }
    }
    // Everything else in the file is sound and still checks.
    for entry in report
        .entries
        .iter()
        .filter(|e| bad.iter().all(|(name, _)| e.name.to_string() != *name))
    {
        assert!(
            entry.outcome.is_checked(),
            "{}: {:?}",
            entry.name,
            entry.outcome
        );
    }
}

#[test]
fn projections_out_of_recursive_and_indexed_inductives_are_checked() {
    // A real lean4export file of ORDINARY Lean (see the fixture README):
    // nothing in it was added without Lean's kernel check, so every
    // declaration must be checked here too.
    //
    // `Tree` is a recursive `structure` nested through `List`, `Stream` a
    // recursive `structure` whose recursion is direct, `Tagged α : Nat →
    // Type` a one-constructor family with an index. Their projections —
    // `Tree.label`, `Tree.kids`, `Stream.head`, `Stream.tail`, generated by
    // Lean, and `Tagged.val`, `Tagged.tag`, written with the anonymous
    // projection notation — are six `Expr.proj` out of types that are NOT
    // structure-like. Lean's kernel asks of a projection's inductive type
    // one constructor and nothing more; while the kernel asked the
    // condition of structure eta instead, this file replayed as 14 checked /
    // 0 unsupported / 8 rejected — a wrong REJECT of every recursive
    // structure (also in the published 0.1.3 and 0.1.4).
    assert_split("ProjRecIndexed", 22, 0, 0);

    let file = read_str(&fixture("ProjRecIndexed")).unwrap_or_else(|e| panic!("parse: {e}"));
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    for d in &file.decls {
        let entry = replayer.replay(d);
        assert!(
            entry.outcome.is_checked(),
            "{}: {:?}",
            entry.name,
            entry.outcome
        );
    }
    // The three types are what the comment says: none is structure-like,
    // and that is why none of them has structure eta.
    let env = replayer.env();
    let ns = Name::from_str("ProjRecIndexed");
    let facts = |leaf: &str| {
        let name = ns.clone().append_str(leaf);
        let iv = env
            .get_inductive_val(&name)
            .unwrap_or_else(|| panic!("{name} must be installed as an inductive"));
        (
            iv.ctors.len(),
            iv.num_params,
            iv.num_indices,
            iv.is_rec,
            iv.num_nested,
            env.is_structure_like(&name),
        )
    };
    assert_eq!(facts("Tree"), (1, 0, 0, true, 1, false));
    assert_eq!(facts("Stream"), (1, 0, 0, true, 0, false));
    assert_eq!(facts("Tagged"), (1, 1, 1, false, 0, false));
    // The theorem that uses all six projections is the last declaration.
    let report = replay_file(&file);
    let last = report.entries.last().unwrap_or_else(|| panic!("non-empty"));
    assert_eq!(last.name.to_string(), "ProjRecIndexed.proj_mk");
    assert_eq!(last.kind, "thm");
    assert!(last.outcome.is_checked());
}

/// Extract the digits following `key` in `line`.
fn extract_field(line: &str, key: &str) -> String {
    let start = match line.find(key) {
        Some(i) => i + key.len(),
        None => panic!("field {key} not found in {line}"),
    };
    line[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect()
}

// ---------------------------------------------------------------------------
// Real checking, not rubber-stamping (synthetic ax/def accept + reject).
// ---------------------------------------------------------------------------

#[test]
fn synthetic_axiom_def_check_and_reject() {
    // A fully-supported synthetic file: an axiom, a def that type-checks
    // against it, and a def the kernel must REJECT (Sort-level mismatch).
    let input = format!(
        "{META}\n{}",
        [
            r#"{"in":1,"str":{"pre":0,"str":"myAx"}}"#,
            r#"{"in":2,"str":{"pre":0,"str":"myDef"}}"#,
            r#"{"in":3,"str":{"pre":0,"str":"bad"}}"#,
            r#"{"il":1,"succ":0}"#,
            r#"{"il":2,"succ":1}"#,
            r#"{"ie":0,"sort":1}"#,
            r#"{"ie":1,"sort":2}"#,
            r#"{"const":{"name":1,"us":[]},"ie":2}"#,
            r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":1,"type":0}}"#,
            r#"{"def":{"all":[2],"hints":{"regular":1},"levelParams":[],"name":2,"safety":"safe","type":0,"value":2}}"#,
            r#"{"def":{"all":[3],"hints":"abbrev","levelParams":[],"name":3,"safety":"safe","type":1,"value":2}}"#,
        ]
        .join("\n")
    );
    let file = read_str(&input).unwrap_or_else(|e| panic!("parse: {e}"));
    let report = replay_file(&file);

    assert_eq!(report.entries.len(), 3);
    assert_eq!(report.entries[0].name.to_string(), "myAx");
    assert!(report.entries[0].outcome.is_checked(), "axiom must check");
    assert_eq!(report.entries[1].name.to_string(), "myDef");
    assert!(
        report.entries[1].outcome.is_checked(),
        "def with matching type must check: {:?}",
        report.entries[1].outcome
    );
    assert_eq!(report.entries[2].name.to_string(), "bad");
    assert!(
        report.entries[2].outcome.is_rejected(),
        "def with Sort-level mismatch must be rejected: {:?}",
        report.entries[2].outcome
    );
}

// ---------------------------------------------------------------------------
// Quot.sound: the axiom is validated against the kernel's canonical type.
// ---------------------------------------------------------------------------

fn builtin_env() -> Environment {
    let mut env = Environment::new();
    init_builtin_env(&mut env).unwrap_or_else(|e| panic!("builtin env: {e}"));
    env
}

fn quot_sound_axiom(level_params: Vec<Name>, ty: Expr) -> ExportDecl {
    ExportDecl::Axiom(AxiomVal {
        common: ConstantVal {
            name: oxilean_kernel::env::quot_sound_name(),
            level_params,
            ty,
        },
        is_unsafe: false,
    })
}

#[test]
fn quot_sound_with_canonical_type_checks() {
    // The builtin env has Eq + the four quot primitives (via add_quot).
    let mut env = builtin_env();
    let decl = quot_sound_axiom(
        oxilean_kernel::env::canonical_quot_sound_level_params(),
        oxilean_kernel::env::canonical_quot_sound_type(),
    );
    let entry = replay_decl(&mut env, &decl);
    assert!(
        entry.outcome.is_checked(),
        "canonical Quot.sound must check: {:?}",
        entry.outcome
    );
}

#[test]
fn quot_sound_tolerates_level_param_renaming() {
    // Level parameter NAMES are not semantic: `Quot.sound.{v}` with the
    // canonical type instantiated at `v` must check.
    let mut env = builtin_env();
    let v = Name::str("v");
    let ty = oxilean_kernel::instantiate::instantiate_type_lparams(
        &oxilean_kernel::env::canonical_quot_sound_type(),
        &oxilean_kernel::env::canonical_quot_sound_level_params(),
        &[Level::param(v.clone())],
    );
    let entry = replay_decl(&mut env, &quot_sound_axiom(vec![v], ty));
    assert!(
        entry.outcome.is_checked(),
        "renamed-param Quot.sound must check: {:?}",
        entry.outcome
    );
}

#[test]
fn quot_sound_with_bogus_type_is_rejected() {
    // An export cannot smuggle an arbitrary assertion under the trusted name
    // `Quot.sound`: here `∀ {α : Sort u}, α → α` (a valid axiom TYPE, but not
    // the canonical soundness statement) must be rejected.
    let mut env = builtin_env();
    let bogus = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Node::new(Expr::Sort(Level::param(Name::str("u")))),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Node::new(Expr::BVar(0)),
            Node::new(Expr::BVar(1)),
        )),
    );
    let entry = replay_decl(&mut env, &quot_sound_axiom(vec![Name::str("u")], bogus));
    match &entry.outcome {
        ReplayOutcome::Rejected { reason } => {
            assert!(
                reason.contains("canonical"),
                "reason names the canonical-type requirement: {reason}"
            );
        }
        other => panic!("bogus Quot.sound must be REJECTED, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Nested inductives: the named unsupported feature + the dependency cascade.
// ---------------------------------------------------------------------------

/// `Box : Type → Type` with `Box.mk : (α : Type) → α → Box α`, as a bundle
/// with the recursor the kernel derives for it.
fn box_bundle() -> ExportDecl {
    use oxilean_kernel::{check_and_derive_family, ConstantInfo, InductiveSpec, RecursorVal};
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let box_name = Name::str("Box");
    let mk_name = box_name.clone().append_str("mk");
    let box_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Node::new(type0.clone()),
        Node::new(type0.clone()),
    );
    let mk_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Node::new(type0),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Node::new(Expr::BVar(0)),
            Node::new(Expr::App(
                Node::new(Expr::Const(box_name.clone(), vec![])),
                Node::new(Expr::BVar(1)),
            )),
        )),
    );
    let spec = InductiveSpec::new(
        box_name.clone(),
        box_ty.clone(),
        vec![(mk_name.clone(), mk_ty.clone())],
    );
    let derived = check_and_derive_family(&Environment::new(), &[], 1, &[spec])
        .unwrap_or_else(|e| panic!("derive Box: {e:?}"));
    let recs: Vec<RecursorVal> = derived
        .recursors
        .iter()
        .filter_map(|ci| match ci {
            ConstantInfo::Recursor(rv) => Some(rv.clone()),
            _ => None,
        })
        .collect();
    ExportDecl::Inductive(InductiveBundle {
        types: vec![InductiveVal {
            common: ConstantVal {
                name: box_name.clone(),
                level_params: vec![],
                ty: box_ty,
            },
            num_params: 1,
            num_indices: 0,
            all: vec![box_name.clone()],
            ctors: vec![mk_name.clone()],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }],
        ctors: vec![ConstructorVal {
            common: ConstantVal {
                name: mk_name,
                level_params: vec![],
                ty: mk_ty,
            },
            induct: box_name,
            cidx: 0,
            num_params: 1,
            num_fields: 1,
            is_unsafe: false,
        }],
        recs,
    })
}

/// `T : Type → Type` with `T.mk : (α : Type) → Box (T α) → T α`, flagged
/// nested as Lean's kernel computes it: a nested occurrence in a family WITH
/// A PARAMETER, a shape the nested stage does not un-nest yet.
fn nested_family_with_a_parameter() -> ExportDecl {
    let type0 = Expr::Sort(Level::succ(Level::zero()));
    let t_name = Name::str("T");
    let mk_name = t_name.clone().append_str("mk");
    let t_of = |arg: u32| {
        Expr::App(
            Node::new(Expr::Const(t_name.clone(), vec![])),
            Node::new(Expr::BVar(arg)),
        )
    };
    let t_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Node::new(type0.clone()),
        Node::new(type0.clone()),
    );
    let mk_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Node::new(type0),
        Node::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Node::new(Expr::App(
                Node::new(Expr::Const(Name::str("Box"), vec![])),
                Node::new(t_of(0)),
            )),
            Node::new(t_of(1)),
        )),
    );
    ExportDecl::Inductive(InductiveBundle {
        types: vec![InductiveVal {
            common: ConstantVal {
                name: t_name.clone(),
                level_params: vec![],
                ty: t_ty,
            },
            num_params: 1,
            num_indices: 0,
            all: vec![t_name.clone()],
            ctors: vec![mk_name.clone()],
            num_nested: 1,
            is_rec: true,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }],
        ctors: vec![ConstructorVal {
            common: ConstantVal {
                name: mk_name,
                level_params: vec![],
                ty: mk_ty,
            },
            induct: t_name,
            cidx: 0,
            num_params: 1,
            num_fields: 1,
            is_unsafe: false,
        }],
        recs: vec![],
    })
}

#[test]
fn nested_inductive_flag_is_named_unsupported_and_cascades() {
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let container = replayer.replay(&box_bundle());
    assert!(
        container.outcome.is_checked(),
        "the container must check: {:?}",
        container.outcome
    );
    let entry = replayer.replay(&nested_family_with_a_parameter());
    assert_eq!(
        entry.outcome,
        ReplayOutcome::Unsupported {
            feature: NESTED_INDUCTIVES
        },
        "a nested inductive the kernel cannot un-nest is the named unsupported feature"
    );

    // A definition depending on `T` cascades to a *dependency* deferral —
    // still unsupported, never a rejection.
    let dependent = ExportDecl::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::str("usesT"),
            level_params: vec![],
            ty: Expr::Sort(Level::succ(Level::zero())),
        },
        value: Expr::Const(Name::str("T"), vec![]),
        hints: ReducibilityHint::Abbrev,
        safety: DefinitionSafety::Safe,
        all: vec![Name::str("usesT")],
    });
    let entry = replayer.replay(&dependent);
    assert_eq!(
        entry.outcome,
        ReplayOutcome::Unsupported {
            feature: DEFERRED_DEPENDENCY
        },
        "dependents of unsupported declarations defer, not reject"
    );
}

#[test]
fn kernel_detected_nested_inductive_is_named_unsupported() {
    // `inductive T | mk : List T → T` with num_nested = 0: the kernel's
    // strict-positivity pass classifies the occurrence under `List` as a
    // nested inductive (`UnsupportedNestedInductive`), which replay maps to
    // the same named feature. Uses the builtin env (has `List`).
    let mut env = builtin_env();
    let t = Expr::Const(Name::str("T"), vec![]);
    let list_t = Expr::App(
        Node::new(Expr::Const(Name::str("List"), vec![Level::zero()])),
        Node::new(t.clone()),
    );
    let bundle = ExportDecl::Inductive(InductiveBundle {
        types: vec![InductiveVal {
            common: ConstantVal {
                name: Name::str("T"),
                level_params: vec![],
                ty: Expr::Sort(Level::succ(Level::zero())),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("T")],
            ctors: vec![Name::str("T.mk")],
            num_nested: 0,
            is_rec: true,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }],
        ctors: vec![ConstructorVal {
            common: ConstantVal {
                name: Name::str("T.mk"),
                level_params: vec![],
                ty: Expr::Pi(
                    BinderInfo::Default,
                    Name::str("l"),
                    Node::new(list_t),
                    Node::new(t),
                ),
            },
            induct: Name::str("T"),
            cidx: 0,
            num_params: 0,
            num_fields: 1,
            is_unsafe: false,
        }],
        recs: vec![],
    });
    let entry = replay_decl(&mut env, &bundle);
    assert_eq!(
        entry.outcome,
        ReplayOutcome::Unsupported {
            feature: NESTED_INDUCTIVES
        },
        "kernel-detected nesting is unsupported, not rejected"
    );
}

#[test]
fn flagged_nested_inductive_with_matching_recursors_checks() {
    // `inductive NestedT | mk : List NestedT → NestedT`, flagged nested, with the
    // kernel-derived (restored) recursors supplied as the export would carry
    // them, under Lean's names: the nested-aware replay path specializes,
    // derives, restores, matches every exported recursor with the derived
    // one of its name by def-eq, and verifies to Checked.
    use oxilean_kernel::{
        check_and_derive_family_nested, restore_nested, ConstantInfo, InductiveSpec, RecursorVal,
    };
    let env = builtin_env();
    let t = Expr::Const(Name::str("NestedT"), vec![]);
    let list_t = Expr::App(
        Node::new(Expr::Const(Name::str("List"), vec![Level::zero()])),
        Node::new(t.clone()),
    );
    let mk_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("l"),
        Node::new(list_t),
        Node::new(t.clone()),
    );

    // Recursors the export would carry (kernel-derived, restored over real List).
    let spec = InductiveSpec::new(
        Name::str("NestedT"),
        Expr::Sort(Level::succ(Level::zero())),
        vec![(Name::str("NestedT.mk"), mk_ty.clone())],
    );
    let (raw, exp) = check_and_derive_family_nested(&env, &[], 0, &[spec]).expect("derive");
    let restored = restore_nested(raw, &exp.expect("nested"));
    let recs: Vec<RecursorVal> = restored
        .recursors
        .iter()
        .filter_map(|ci| match ci {
            ConstantInfo::Recursor(rv) => Some(rv.clone()),
            _ => None,
        })
        .collect();
    // The derivation's name for the auxiliary recursor is provisional
    // (`NestedT._nested_0.rec`); an export carries the names Lean's kernel
    // gives, `NestedT.rec` and `NestedT.rec_1`, and the nested path pairs a
    // record with the derivation by that name.
    let lean_names = [
        Name::str("NestedT").append_str("rec"),
        Name::str("NestedT").append_str("rec_1"),
    ];
    let provisional: Vec<Name> = recs.iter().map(|r| r.common.name.clone()).collect();
    let rename = |e: &Expr| -> Expr {
        oxilean_kernel::expr_util::replace_expr(e, &mut |sub, _| match sub {
            Expr::Const(n, levels) => provisional
                .iter()
                .position(|p| p == n)
                .map(|i| Expr::Const(lean_names[i].clone(), levels.clone())),
            _ => None,
        })
    };
    let recs: Vec<RecursorVal> = recs
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let mut r = r.clone();
            r.common.name = lean_names[i].clone();
            r.common.ty = rename(&r.common.ty);
            for rule in &mut r.rules {
                rule.rhs = rename(&rule.rhs);
            }
            r
        })
        .collect();

    let bundle = ExportDecl::Inductive(InductiveBundle {
        types: vec![InductiveVal {
            common: ConstantVal {
                name: Name::str("NestedT"),
                level_params: vec![],
                ty: Expr::Sort(Level::succ(Level::zero())),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("NestedT")],
            ctors: vec![Name::str("NestedT.mk")],
            num_nested: 1,
            is_rec: true,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }],
        ctors: vec![ConstructorVal {
            common: ConstantVal {
                name: Name::str("NestedT.mk"),
                level_params: vec![],
                ty: mk_ty,
            },
            induct: Name::str("NestedT"),
            cidx: 0,
            num_params: 0,
            num_fields: 1,
            is_unsafe: false,
        }],
        recs,
    });

    let mut env = env;
    let entry = replay_decl(&mut env, &bundle);
    assert!(
        entry.outcome.is_checked(),
        "a valid nested inductive with matching recursors must verify: {:?}",
        entry.outcome
    );
}

// ---------------------------------------------------------------------------
// Corpus-scale streaming replay with explicit limits.
// ---------------------------------------------------------------------------

#[test]
fn replay_streaming_matches_replay_file() {
    let text = fixture("Parity.isEven");
    let run = replay_streaming(std::io::Cursor::new(text.as_str()), ReplayLimits::corpus())
        .unwrap_or_else(|e| panic!("stream: {e}"));
    assert_eq!(run.report.entries.len(), 181);
    assert_eq!(
        (
            run.report.checked(),
            run.report.unsupported(),
            run.report.rejected()
        ),
        (181, 0, 0)
    );
    assert_eq!(run.stats.decls_replayed, 181);
    assert!(run.stats.max_decl_time.is_some());
    assert!(
        run.stats.over_time_budget.is_empty(),
        "no budget configured, nothing flagged"
    );
    assert_eq!(run.meta.format_version, "3.1.0");
    assert_eq!(run.read_stats.total_decls(), 181);
}

#[test]
fn replay_streaming_advisory_time_budget_flags_decls() {
    // A zero budget flags every declaration — outcomes stay unchanged
    // (the budget is advisory, checked after each declaration completes).
    let text = fixture("simple_add");
    let limits = ReplayLimits {
        read: Limits::default(),
        per_decl_time_budget: Some(Duration::ZERO),
        per_decl_fuel: None,
    };
    let run = replay_streaming(std::io::Cursor::new(text.as_str()), limits)
        .unwrap_or_else(|e| panic!("stream: {e}"));
    assert_eq!(run.report.checked(), 23, "outcomes unchanged by the budget");
    assert_eq!(run.stats.over_time_budget.len(), 23, "every decl flagged");
}

#[test]
fn replay_streaming_respects_reader_node_budget() {
    // A tiny CUMULATIVE materialization budget surfaces the reader's named
    // Unsupported error (the untrusted-input bound), never an OOM or a wrong
    // replay. The per-declaration cap is lifted here so the cumulative check
    // stays the one that fires (C22 gave single declarations their own,
    // recoverable cap — tested separately below).
    let text = fixture("Parity.isEven");
    let limits = ReplayLimits {
        read: Limits {
            materialize_budget: 10,
            decl_materialize_budget: u64::MAX,
        },
        per_decl_time_budget: None,
        per_decl_fuel: None,
    };
    match replay_streaming(std::io::Cursor::new(text.as_str()), limits) {
        Err(e) => {
            assert!(e.is_unsupported(), "budget errors are Unsupported: {e}");
            assert!(format!("{e}").contains(BUDGET_FEATURE.split(' ').next().unwrap_or("")),);
        }
        Ok(_) => panic!("a 10-node budget cannot fit Parity.isEven"),
    }
}

#[test]
fn oversized_decl_is_named_unsupported_and_cascades_never_rejected() {
    // C22 regression pin for the per-declaration materialization budget: a
    // declaration whose materialization exceeds the PER-DECLARATION budget must
    // become the named `DECL_BUDGET_FEATURE` unsupported bucket (skipped BEFORE
    // any type-checking), and every declaration depending on it must cascade as
    // DEFERRED_DEPENDENCY — never a rejection, never an abort of the file, never
    // an OOM.
    //
    // Under structural sharing (wave5) the budget is counted in DISTINCT nodes,
    // so a sharing bomb no longer trips it (it materializes as a small DAG —
    // see `doubling_app_bomb_...` in malformed_input.rs). The original C22
    // referent (Init decl #42,086,
    // `WellFounded.partialExtrinsicFix₃_eq_partialExtrinsicFix`) had a huge
    // *tree* but a modest *distinct-node* count, so it no longer hits this path
    // at all — it now materializes as a bounded DAG and is checked normally.
    // To still exercise the Oversized-cascade MECHANISM we use a genuinely
    // wide term: a non-doubling app chain of 70 distinct nodes (each
    // `App(prev, sort)`), whose distinct-node count (73) exceeds a 64-node cap.
    // Because Oversized skips materialization entirely, the term's ill-typedness
    // is irrelevant — it is never checked.
    const META: &str = r#"{"meta":{"exporter":{"name":"lean4export","version":"3.1.0"},"format":{"version":"3.1.0"},"lean":{"githash":"x","version":"4.32.0-rc1"}}}"#;
    let mut lines = vec![
        META.to_string(),
        r#"{"in":1,"str":{"pre":0,"str":"big"}}"#.to_string(),
        r#"{"in":2,"str":{"pre":0,"str":"dep"}}"#.to_string(),
        r#"{"il":1,"succ":0}"#.to_string(),
        r#"{"ie":0,"sort":1}"#.to_string(),
    ];
    // Non-doubling chain: ie k = App(ie_{k-1}, ie_0). Each ie_k is a distinct
    // node (no sharing collapse), so ie 70 has 70 App + 1 sort + 2 level = 73
    // distinct nodes.
    for i in 1..=70u32 {
        let prev = i - 1;
        lines.push(format!(r#"{{"app":{{"fn":{prev},"arg":0}},"ie":{i}}}"#));
    }
    // dep's type: app(const big, sort) — tiny, but mentions `big`.
    lines.push(r#"{"const":{"name":1,"us":[]},"ie":71}"#.to_string());
    lines.push(r#"{"app":{"fn":71,"arg":0},"ie":72}"#.to_string());
    lines.push(r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":1,"type":70}}"#.to_string());
    lines.push(r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":2,"type":72}}"#.to_string());
    let text = lines.join("\n");
    let limits = ReplayLimits {
        read: Limits {
            materialize_budget: oxilean_export::DEFAULT_MATERIALIZE_BUDGET,
            decl_materialize_budget: 64,
        },
        per_decl_time_budget: None,
        per_decl_fuel: None,
    };
    let run = replay_streaming(std::io::Cursor::new(text.as_str()), limits)
        .expect("oversized declarations must not abort the read");
    assert_eq!(
        run.report.rejected(),
        0,
        "oversized/cascaded declarations must NEVER be rejected"
    );
    let outcome_of = |n: &str| {
        run.report
            .entries
            .iter()
            .find(|e| e.name.to_string() == n)
            .unwrap_or_else(|| panic!("entry {n} missing"))
            .outcome
            .clone()
    };
    match outcome_of("big") {
        ReplayOutcome::Unsupported { feature } => {
            assert_eq!(feature, oxilean_export::DECL_BUDGET_FEATURE);
        }
        other => panic!("big must be the named oversized bucket, got {other:?}"),
    }
    match outcome_of("dep") {
        ReplayOutcome::Unsupported { feature } => {
            assert_eq!(
                feature, DEFERRED_DEPENDENCY,
                "dependents of an oversized decl cascade as deferred"
            );
        }
        other => panic!("dep must cascade as deferred, got {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Deterministic per-declaration resource fuel (C16).
// ---------------------------------------------------------------------------

#[test]
fn tiny_fuel_budget_yields_named_unsupported_never_rejected() {
    // With a deliberately tiny clone-fuel budget, declarations that need any
    // real reduction work land in the NAMED resource-limit bucket (or the
    // dependency cascade), and NOTHING is rejected: the kernel degrades
    // conservatively, and a resource cutoff is not an alarm.
    let file = read_str(&fixture("simple_add")).unwrap_or_else(|e| panic!("parse: {e}"));
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    replayer.set_per_decl_fuel(Some(50));
    let mut resource_limited = 0usize;
    let mut rejected = 0usize;
    for decl in &file.decls {
        match replayer.replay(decl).outcome {
            ReplayOutcome::Unsupported { feature } if feature == RESOURCE_LIMIT => {
                resource_limited += 1;
            }
            ReplayOutcome::Rejected { .. } => rejected += 1,
            _ => {}
        }
    }
    assert!(
        resource_limited > 0,
        "a 50-node budget must trip on simple_add"
    );
    assert_eq!(rejected, 0, "fuel exhaustion must NEVER reject");
}

#[test]
fn corpus_fuel_budget_leaves_fixtures_untouched() {
    // The corpus preset's budget is far above anything a fixture needs: the
    // pinned 23/0/0 split is unchanged, and the run is deterministic.
    let text = fixture("simple_add");
    let run = replay_streaming(std::io::Cursor::new(text.as_str()), ReplayLimits::corpus())
        .unwrap_or_else(|e| panic!("stream: {e}"));
    assert_eq!(
        (
            run.report.checked(),
            run.report.unsupported(),
            run.report.rejected()
        ),
        (23, 0, 0)
    );
}
