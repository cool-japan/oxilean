//! Replay tests: every fixture is replayed into a fresh, **empty** environment
//! and the three-bucket split is pinned EXACTLY — the split is a product
//! number, and nothing may be silently skipped.
//!
//! Coverage:
//! * all six fixtures fully checked (inductives, mutual groups, structures
//!   with eta/proj, quotients, kernel-re-derived recursors);
//! * a rejected control (corrupted proof term → `Rejected`, not
//!   `Unsupported`);
//! * `Quot.sound` axiom validation against the kernel's canonical type;
//! * nested inductives → the named `Unsupported` feature, with the
//!   dependency cascade;
//! * the corpus-scale streaming replay API (`replay_streaming`,
//!   `ReplayLimits`).

use oxilean_kernel::Node;
use std::time::Duration;

use oxilean_export::{
    read_str, replay_decl, replay_file, replay_streaming, ExportDecl, InductiveBundle, Limits,
    ReplayLimits, ReplayOutcome, Replayer, BUDGET_FEATURE, DEFERRED_DEPENDENCY, NESTED_INDUCTIVES,
    RESOURCE_LIMIT,
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

/// A synthetic bundle for `inductive T` flagged as nested by the exporter.
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
fn nested_inductive_flag_is_named_unsupported_and_cascades() {
    let mut replayer = Replayer::new().unwrap_or_else(|e| panic!("replayer: {e}"));
    let entry = replayer.replay(&nested_flagged_bundle());
    assert_eq!(
        entry.outcome,
        ReplayOutcome::Unsupported {
            feature: NESTED_INDUCTIVES
        },
        "exporter-flagged nested inductive is the named unsupported feature"
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
