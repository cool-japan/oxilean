//! End-to-end CLI integration tests.
//!
//! These run the *built* `oxilean-verify` binary via `std::process::Command`
//! (no external test-harness crate — the product's dependency closure stays
//! kernel + export only, and the tests stay in that spirit). We pin:
//!
//! * exit-code correctness (0 = ran/zero-rejected, 1 = any rejected,
//!   2 = usage/IO/malformed),
//! * the exact three-bucket split for every committed fixture (Wave-3b full
//!   replay landed, so the numbers are now stable truth and MUST agree with
//!   `oxilean-export`'s own `replay_smoke` pins — see
//!   `fixture_three_bucket_splits_are_pinned`),
//! * the summary-line format,
//! * JSON well-formedness and the presence of the provenance pins,
//! * determinism (two `--no-color` runs are byte-identical once the timing
//!   columns — which are legitimately non-deterministic — are normalised).

use std::path::PathBuf;
use std::process::{Command, Output};

/// Absolute path to the built binary, provided by cargo to integration tests.
fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_oxilean-verify")
}

/// Absolute path to a committed fixture under the workspace `tests/fixtures`.
fn fixture(name: &str) -> PathBuf {
    // CARGO_MANIFEST_DIR is crates/oxilean-verify; fixtures live at the
    // workspace root's tests/fixtures/lean4export.
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("../../tests/fixtures/lean4export");
    p.push(format!("{name}.ndjson"));
    p
}

/// A per-test scratch directory unique to this test binary + a suffix.
fn scratch_path(suffix: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("oxilean-verify-it-{}-{suffix}", std::process::id()));
    p
}

fn run(args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .env("NO_COLOR", "1") // keep output stable in CI regardless of tty
        .output()
        .expect("failed to spawn oxilean-verify binary")
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

fn code(o: &Output) -> i32 {
    o.status.code().unwrap_or(-1)
}

/// The synthetic export used to force exactly one REJECTED declaration: an
/// axiom, a def that checks, and a def with a `Sort`-level mismatch the kernel
/// must reject. (Mirrors oxilean-export's replay smoke test, kept independent so
/// the exit-code alarm is exercised through the real binary.)
const REJECTED_EXPORT: &str = concat!(
    r#"{"meta":{"exporter":{"name":"lean4export","version":"3.1.0"},"format":{"version":"3.1.0"},"lean":{"githash":"x","version":"4.32.0-rc1"}}}"#,
    "\n",
    r#"{"in":1,"str":{"pre":0,"str":"myAx"}}"#,
    "\n",
    r#"{"in":2,"str":{"pre":0,"str":"myDef"}}"#,
    "\n",
    r#"{"in":3,"str":{"pre":0,"str":"bad"}}"#,
    "\n",
    r#"{"il":1,"succ":0}"#,
    "\n",
    r#"{"il":2,"succ":1}"#,
    "\n",
    r#"{"ie":0,"sort":1}"#,
    "\n",
    r#"{"ie":1,"sort":2}"#,
    "\n",
    r#"{"const":{"name":1,"us":[]},"ie":2}"#,
    "\n",
    r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":1,"type":0}}"#,
    "\n",
    r#"{"def":{"all":[2],"hints":{"regular":1},"levelParams":[],"name":2,"safety":"safe","type":0,"value":2}}"#,
    "\n",
    r#"{"def":{"all":[3],"hints":"abbrev","levelParams":[],"name":3,"safety":"safe","type":1,"value":2}}"#,
    "\n",
);

/// A valid header followed by a truncated JSON object — malformed *input*, which
/// must be exit 2, never the rejection alarm.
const MALFORMED_EXPORT: &str = concat!(
    r#"{"meta":{"exporter":{"name":"lean4export","version":"3.1.0"},"format":{"version":"3.1.0"},"lean":{"githash":"x","version":"4.32.0-rc1"}}}"#,
    "\n",
    r#"{"in":1,"str":{"pre":0,"str":"Eq""#,
    "\n",
);

fn write_scratch(suffix: &str, contents: &str) -> PathBuf {
    let path = scratch_path(suffix);
    std::fs::write(&path, contents).expect("write scratch fixture");
    path
}

// ---------------------------------------------------------------------------
// Exit codes
// ---------------------------------------------------------------------------

#[test]
fn happy_path_exits_zero() {
    // simple_add has zero rejections regardless of replay coverage, so exit 0.
    let out = run(&[fixture("simple_add").to_str().unwrap()]);
    assert_eq!(code(&out), 0, "stderr: {}", stderr(&out));
}

#[test]
fn rejected_declaration_exits_one() {
    let path = write_scratch("rejected", REJECTED_EXPORT);
    let out = run(&[path.to_str().unwrap()]);
    let _ = std::fs::remove_file(&path);
    assert_eq!(code(&out), 1, "a rejection must trip the alarm (exit 1)");
    assert!(
        stdout(&out).contains("rejected"),
        "streamed output must mention the rejection: {}",
        stdout(&out)
    );
}

#[test]
fn declaration_lean_kernel_rejects_exits_one() {
    // The committed negative fixture: a real lean4export file whose theorem
    // `KGuardCollision.bad` Lean's own kernel rejects (see the fixture
    // README). It must trip the alarm through the real binary. Before the
    // free-variable id fix this run printed `4 verified · 0 unsupported ·
    // 0 rejected` and exited 0.
    let out = run(&["--quiet", fixture("KGuardCollision").to_str().unwrap()]);
    assert_eq!(
        code(&out),
        1,
        "a declaration Lean's kernel rejects must exit 1; stdout: {}",
        stdout(&out)
    );
    assert_eq!(
        parse_summary_counts(&stdout(&out)),
        (3, 0, 1),
        "KGuardCollision: 3 sound declarations verify, the bad theorem is rejected"
    );

    // And the rejection is attributed to that theorem.
    let out = run(&[fixture("KGuardCollision").to_str().unwrap()]);
    let text = stdout(&out);
    let bad_line = text
        .lines()
        .find(|l| l.contains("KGuardCollision.bad"))
        .unwrap_or_else(|| panic!("no line for KGuardCollision.bad in: {text}"));
    assert!(
        bad_line.contains("rejected"),
        "KGuardCollision.bad must be the rejected declaration: {bad_line}"
    );
}

#[test]
fn projection_out_of_a_proposition_exits_one() {
    // The committed negative fixture `ProjOutOfProp` (see the fixture
    // README): a definition that reads the witness out of a proof of an
    // existential, and a closed, axiom-free proof of `False` built on that
    // projection. Lean's own kernel rejects both. Before the kernel had the
    // side condition on projections this run printed `7 verified ·
    // 0 unsupported · 0 rejected` and exited 0.
    let out = run(&["--quiet", fixture("ProjOutOfProp").to_str().unwrap()]);
    assert_eq!(
        code(&out),
        1,
        "a declaration Lean's kernel rejects must exit 1; stdout: {}",
        stdout(&out)
    );
    assert_eq!(
        parse_summary_counts(&stdout(&out)),
        (5, 0, 2),
        "ProjOutOfProp: the 5 sound declarations verify, the definition and the proof of False are rejected"
    );

    // And each rejection is attributed to its declaration, under the
    // default limits and under the corpus preset alike.
    for limits in [&[][..], &["--limits", "corpus"][..]] {
        let fixture_path = fixture("ProjOutOfProp");
        let mut args: Vec<&str> = limits.to_vec();
        args.push(fixture_path.to_str().unwrap());
        let out = run(&args);
        assert_eq!(code(&out), 1, "limits {limits:?}: stdout: {}", stdout(&out));
        let text = stdout(&out);
        for bad in ["ProjOutOfProp.wit", "ProjOutOfProp.false_of_proj"] {
            let bad_line = text
                .lines()
                .find(|l| l.split_whitespace().any(|word| word == bad))
                .unwrap_or_else(|| panic!("no line for {bad} in: {text}"));
            assert!(
                bad_line.contains("rejected"),
                "{bad} must be a rejected declaration (limits {limits:?}): {bad_line}"
            );
        }
    }
}

#[test]
fn projections_out_of_recursive_and_indexed_inductives_exit_zero() {
    // The committed positive fixture `ProjRecIndexed` (see the fixture
    // README): ordinary Lean with two recursive structures and an indexed
    // one-constructor family, and the projections out of them. Lean's
    // kernel accepted every declaration. While the kernel asked of a
    // projection's inductive type the condition of structure eta (no
    // recursion, no indices) this run printed `14 verified · 0 unsupported ·
    // 8 rejected` and exited 1 — a false alarm on every recursive structure.
    for limits in [&[][..], &["--limits", "corpus"][..]] {
        let fixture_path = fixture("ProjRecIndexed");
        let mut args: Vec<&str> = limits.to_vec();
        args.push(fixture_path.to_str().unwrap());
        let out = run(&args);
        let text = stdout(&out);
        assert_eq!(
            code(&out),
            0,
            "limits {limits:?}: ordinary Lean must not trip the alarm; stdout: {text}"
        );
        assert_eq!(
            parse_summary_counts(&text),
            (22, 0, 0),
            "limits {limits:?}: ProjRecIndexed: all 22 declarations verify"
        );
        // Each projection, and the theorem that uses them, is a verified
        // declaration of its own (not merely absent from the rejections).
        for good in [
            "ProjRecIndexed.Tree.label",
            "ProjRecIndexed.Tree.kids",
            "ProjRecIndexed.Stream.head",
            "ProjRecIndexed.Stream.tail",
            "ProjRecIndexed.Tagged.val",
            "ProjRecIndexed.Tagged.tag",
            "ProjRecIndexed.proj_mk",
        ] {
            let line = text
                .lines()
                .find(|l| l.split_whitespace().any(|word| word == good))
                .unwrap_or_else(|| panic!("no line for {good} in: {text}"));
            assert!(
                line.contains('\u{2713}') && !line.contains("rejected"),
                "{good} must be a verified declaration (limits {limits:?}): {line}"
            );
        }
    }
}

#[test]
fn a_bundle_that_lies_about_its_constructors_exits_one() {
    // The committed negative fixture `CtorListLie` (see the fixture README):
    // a real export in which three inductive types were cut down, by hand,
    // to their first constructor, followed by four declarations that read a
    // field out of them with a projection Lean's kernel refuses — one of
    // them the false statement `∀ t : Two, t = Two.mk t.0`. Each bundle is
    // refused. While a refused bundle stayed half installed this run printed
    // `5 verified · 0 unsupported · 5 rejected`, with `two_proj` and
    // `two_eta` among the verified.
    let bundles = ["CtorListLie.Two", "CtorListLie.L2", "CtorListLie.TI"];
    let dependents = [
        "CtorListLie.two_proj",
        "CtorListLie.two_eta",
        "CtorListLie.l2_proj",
        "CtorListLie.ti_proj",
    ];
    for limits in [&[][..], &["--limits", "corpus"][..]] {
        let fixture_path = fixture("CtorListLie");
        let mut args: Vec<&str> = limits.to_vec();
        args.push(fixture_path.to_str().unwrap());
        let out = run(&args);
        let text = stdout(&out);
        assert_eq!(code(&out), 1, "limits {limits:?}: stdout: {text}");
        assert_eq!(
            parse_summary_counts(&text),
            (3, 0, 7),
            "limits {limits:?}: `Nat`, `Eq` and `Bool` verify; the three bundles and the four \
             declarations that name them are rejected"
        );
        let line_of = |name: &str| {
            text.lines()
                .find(|l| l.split_whitespace().any(|word| word == name))
                .unwrap_or_else(|| panic!("no line for {name} in: {text}"))
        };
        for bundle in bundles {
            let line = line_of(bundle);
            assert!(
                line.contains("rejected") && line.contains("is not listed"),
                "{bundle} must be rejected for the constructor it does not list \
                 (limits {limits:?}): {line}"
            );
        }
        for dependent in dependents {
            let line = line_of(dependent);
            assert!(
                line.contains("rejected") && line.contains("depends on rejected declaration"),
                "{dependent} must be rejected as a dependent of a rejected bundle \
                 (limits {limits:?}): {line}"
            );
        }
    }
}

#[test]
fn a_recursor_rule_that_is_not_the_kernels_exits_one_and_proves_nothing() {
    // The committed negative fixture `RecRuleJunk` (see the fixture README):
    // a file written by hand in which ONE thing differs from what Lean's
    // kernel makes of the same declarations — the right-hand side of the
    // only rule of `T.rec` carries an ill-typed proof term. The bundle is
    // refused, and with it every declaration that names it: `val`, `fst`,
    // `val_eq`, and the statements made about them, among them
    // `bad_f_eq_t : B.f = B.t` and `boom : False`. While a recursor record
    // was accepted by definitional equality alone and then installed, this
    // run printed `14 verified · 0 unsupported · 1 rejected`, with `boom`
    // among the verified and the honest `honest_val_is_t` the one rejected.
    let dependents = [
        "val",
        "fst",
        "val_eq",
        "honest_val_is_t",
        "bad_val_is_f",
        "bad_f_eq_t",
        "boom",
    ];
    for limits in [&[][..], &["--limits", "corpus"][..]] {
        let fixture_path = fixture("RecRuleJunk");
        let mut args: Vec<&str> = limits.to_vec();
        args.push(fixture_path.to_str().unwrap());
        let out = run(&args);
        let text = stdout(&out);
        assert_eq!(code(&out), 1, "limits {limits:?}: stdout: {text}");
        assert_eq!(
            parse_summary_counts(&text),
            (7, 0, 8),
            "limits {limits:?}: `B`, `P`, `Eq`, `False`, `True`, `get` and `get_eq` verify; the \
             bundle `T` and the seven declarations that name it or one another are rejected"
        );
        let line_of = |name: &str| {
            text.lines()
                .find(|l| l.split_whitespace().any(|word| word == name))
                .unwrap_or_else(|| panic!("no line for {name} in: {text}"))
        };
        let bundle = line_of("T");
        assert!(
            bundle.contains("rejected")
                && bundle.contains("does not match the kernel-derived recursor"),
            "T must be rejected for its recursor (limits {limits:?}): {bundle}"
        );
        for dependent in dependents {
            let line = line_of(dependent);
            assert!(
                line.contains("rejected") && line.contains("depends on rejected declaration"),
                "{dependent} must be rejected as a dependent of the rejected bundle \
                 (limits {limits:?}): {line}"
            );
        }
    }

    // The same family as Lean's kernel makes it, and an ill-typed term of
    // the same making as the value of a theorem and of a definition: this
    // build and Lean's kernel agree on every declaration of both files.
    for (name, split) in [("RecRuleTruth", (12, 0, 3)), ("RecRuleControl", (3, 0, 2))] {
        let out = run(&["--quiet", fixture(name).to_str().unwrap()]);
        assert_eq!(code(&out), 1, "{name}: stdout: {}", stdout(&out));
        assert_eq!(parse_summary_counts(&stdout(&out)), split, "{name}");
    }
}

#[test]
fn a_recursor_record_under_a_name_lean_never_gives_exits_one_and_proves_nothing() {
    // The committed negative fixtures `RecNameMixedStyle` and `RecNameFlat`
    // (see the fixture README): a recursor record named with the one flat
    // component `"A.rec"` (printed `«A.rec»`) where Lean's kernel names the
    // recursor of `A` `A.rec`, the component `rec` under `A`. In the mutual
    // block of `RecNameMixedStyle` the derived rules then called recursors
    // that were never installed, the file defined one of them, and this run
    // printed `13 verified · 0 unsupported · 0 rejected` with `boom : False`
    // among the verified, exit 0. Now the bundle is refused for its name.
    for limits in [&[][..], &["--limits", "corpus"][..]] {
        for (name, ty, split, dependents) in [
            (
                "RecNameMixedStyle",
                "A",
                (8, 0, 5),
                &["X", "get_X_is_f", "bad_f_eq_t", "boom"][..],
            ),
            (
                "RecNameFlat",
                "T",
                (7, 0, 8),
                &["val", "fst", "val_eq", "honest_val_is_t", "boom"][..],
            ),
        ] {
            let fixture_path = fixture(name);
            let mut args: Vec<&str> = limits.to_vec();
            args.push(fixture_path.to_str().unwrap());
            let out = run(&args);
            let text = stdout(&out);
            assert_eq!(code(&out), 1, "{name}, limits {limits:?}: stdout: {text}");
            assert_eq!(
                parse_summary_counts(&text),
                split,
                "{name}, limits {limits:?}"
            );
            let line_of = |word: &str| {
                text.lines()
                    .find(|l| l.split_whitespace().any(|w| w == word))
                    .unwrap_or_else(|| panic!("no line for {word} in: {text}"))
            };
            let bundle = line_of(ty);
            assert!(
                bundle.contains("rejected")
                    && bundle.contains(&format!("recursor record '«{ty}.rec»'"))
                    && bundle.contains("is not named as Lean's kernel names a recursor"),
                "{name}: {ty} must be rejected for its recursor's name (limits {limits:?}): \
                 {bundle}"
            );
            for dependent in dependents {
                let line = line_of(dependent);
                assert!(
                    line.contains("rejected") && line.contains("depends on rejected declaration"),
                    "{name}: {dependent} must be rejected as a dependent (limits {limits:?}): \
                     {line}"
                );
            }
        }
    }

    // The mutual block as Lean's kernel makes it: this build and Lean's
    // kernel agree on every declaration (9 · 0 · 4).
    let out = run(&[
        "--quiet",
        fixture("RecNameMixedStyleTruth").to_str().unwrap(),
    ]);
    assert_eq!(code(&out), 1, "stdout: {}", stdout(&out));
    assert_eq!(parse_summary_counts(&stdout(&out)), (9, 0, 4));
}

#[test]
fn congruence_first_fixture_verifies_all_but_the_theorem_lean_rejects() {
    // The second committed negative fixture (see the fixture README):
    // `CongruenceFirst.bad` equates one function applied to unequal
    // arguments, and Lean's own kernel rejects it. The three sound theorems
    // of the file need the arguments compared before the function is
    // unfolded; they verify under the DEFAULT limits (before that they were
    // `unsupported` even under `--limits corpus`).
    let out = run(&["--quiet", fixture("CongruenceFirst").to_str().unwrap()]);
    assert_eq!(
        code(&out),
        1,
        "a declaration Lean's kernel rejects must exit 1; stdout: {}",
        stdout(&out)
    );
    assert_eq!(
        parse_summary_counts(&stdout(&out)),
        (184, 0, 1),
        "CongruenceFirst: 184 sound declarations verify, the bad theorem is rejected"
    );

    let out = run(&[fixture("CongruenceFirst").to_str().unwrap()]);
    let text = stdout(&out);
    let line_of = |name: &str| -> &str {
        text.lines()
            .find(|l| l.split_whitespace().any(|word| word == name))
            .unwrap_or_else(|| panic!("no line for {name} in: {text}"))
    };
    assert!(
        line_of("CongruenceFirst.bad").contains("rejected"),
        "CongruenceFirst.bad must be the rejected declaration: {}",
        line_of("CongruenceFirst.bad")
    );
    for name in [
        "CongruenceFirst.same_function_equal_args",
        "CongruenceFirst.mod_stuck_divisor",
        "CongruenceFirst.short_fuel",
    ] {
        assert!(
            !line_of(name).contains("rejected") && !line_of(name).contains("unsupported"),
            "{name} must verify: {}",
            line_of(name)
        );
    }
}

#[test]
fn deep_fixtures_verify_on_the_tools_own_stack_and_their_false_theorems_are_rejected() {
    // The third and fourth committed negative fixtures (see the fixture
    // README). `NestedReduction` needs a reduction nested four hundred deep,
    // `DeepTerm` walks a term three hundred levels deep; both are a matter
    // of stack, and the tool runs on a thread with a stack of its own
    // (`--stack-size`, 512 MiB by default) under no limit on either. Each
    // file ends with a FALSE theorem Lean's kernel rejects, which takes the
    // same depth to find out.
    for (name, verified, bad) in [
        ("NestedReduction", 64, "NestedReduction.bad"),
        ("DeepTerm", 5, "DeepTerm.bad"),
    ] {
        let out = run(&["--quiet", fixture(name).to_str().unwrap()]);
        assert_eq!(
            code(&out),
            1,
            "{name}: a declaration Lean's kernel rejects must exit 1; stdout: {}",
            stdout(&out)
        );
        assert_eq!(
            parse_summary_counts(&stdout(&out)),
            (verified, 0, 1),
            "{name}: every sound declaration verifies, the false theorem is rejected"
        );
        let out = run(&[fixture(name).to_str().unwrap()]);
        let text = stdout(&out);
        let bad_line = text
            .lines()
            .find(|l| l.split_whitespace().any(|word| word == bad))
            .unwrap_or_else(|| panic!("no line for {bad} in: {text}"));
        assert!(
            bad_line.contains("rejected"),
            "{bad} must be the rejected declaration: {bad_line}"
        );
    }
}

#[test]
fn malformed_input_exits_two_and_is_not_a_rejection() {
    let path = write_scratch("malformed", MALFORMED_EXPORT);
    let out = run(&[path.to_str().unwrap()]);
    let _ = std::fs::remove_file(&path);
    assert_eq!(
        code(&out),
        2,
        "malformed input is a usage/IO error, not exit 1"
    );
    let err = stderr(&out);
    assert!(
        err.contains("broken file") || err.contains("broken export"),
        "message must say the file is broken, not that a proof was rejected: {err}"
    );
    // The per-declaration streaming/summary output must NOT show a rejection:
    // a broken file produces no verdicts at all.
    let out_text = stdout(&out);
    assert!(
        !out_text.contains('\u{2717}'),
        "a broken file must not stream a rejection glyph: {out_text}"
    );
}

#[test]
fn missing_file_exits_two() {
    let out = run(&["/nonexistent/does-not-exist.ndjson"]);
    assert_eq!(code(&out), 2);
    assert!(stderr(&out).contains("cannot read"));
}

#[test]
fn no_args_exits_two() {
    let out = run(&[]);
    assert_eq!(code(&out), 2);
    assert!(stderr(&out).contains("no input files"));
}

#[test]
fn unknown_flag_exits_two() {
    let out = run(&["--nope", "x.ndjson"]);
    assert_eq!(code(&out), 2);
    assert!(stderr(&out).contains("unknown option"));
}

// ---------------------------------------------------------------------------
// Help / version
// ---------------------------------------------------------------------------

#[test]
fn help_exits_zero_and_documents_exit_codes() {
    let out = run(&["--help"]);
    assert_eq!(code(&out), 0);
    let text = stdout(&out);
    assert!(text.contains("USAGE"));
    assert!(text.contains("EXIT CODES"));
    assert!(text.contains("--json"));
    assert!(text.contains("--fail-fast"));
}

#[test]
fn version_exits_zero_and_names_pins() {
    let out = run(&["--version"]);
    assert_eq!(code(&out), 0);
    let text = stdout(&out);
    assert!(text.contains("oxilean-verify"));
    assert!(text.contains("lean4export commit"));
    assert!(text.contains("Lean toolchain"));
}

// ---------------------------------------------------------------------------
// Per-fixture three-bucket pins (must agree with oxilean-export replay_smoke)
// ---------------------------------------------------------------------------

/// Parse `(verified, unsupported, rejected)` out of the summary line.
fn parse_summary_counts(text: &str) -> (usize, usize, usize) {
    let summary = text
        .lines()
        .find(|l| l.contains("verified") && l.contains("unsupported") && l.contains("rejected"))
        .expect("a summary line must be printed");
    let count_before = |word: &str| -> usize {
        let idx = summary.find(word).expect("bucket word present");
        summary[..idx]
            .split_whitespace()
            .last()
            .and_then(|tok| tok.parse::<usize>().ok())
            .unwrap_or_else(|| panic!("no count before {word:?} in summary line: {summary}"))
    };
    (
        count_before("verified"),
        count_before("unsupported"),
        count_before("rejected"),
    )
}

/// The Wave-3b ground truth: with full inductive/quotient/recursor replay in
/// the kernel, every declaration in every committed fixture verifies. These
/// numbers are pinned in `oxilean-export`'s `tests/replay_smoke.rs`; the two
/// suites must never disagree. A regression here (anything landing in
/// `unsupported` or — worse — `rejected`) is a kernel/replay bug, not noise:
/// investigate, never re-pin to make the table pretty.
#[test]
fn fixture_three_bucket_splits_are_pinned() {
    let expected: [(&str, usize, usize, usize); 11] = [
        ("simple_add", 23, 0, 0),
        ("Nat.add_succ", 19, 0, 0),
        ("Tree_Forest", 1, 0, 0),
        ("Tree_Forest_full", 37, 0, 0),
        ("point_swap_swap", 16, 0, 0),
        ("Parity.isEven", 181, 0, 0),
        ("Syntax", 269, 0, 0),
        ("QuotPartialApp", 66, 0, 0),
        ("ReductionChain", 36, 0, 0),
        ("SharedSubterms", 60, 0, 0),
        ("ProjRecIndexed", 22, 0, 0),
    ];
    for (name, verified, unsupported, rejected) in expected {
        let out = run(&["--quiet", fixture(name).to_str().unwrap()]);
        assert_eq!(
            code(&out),
            0,
            "{name}: clean fixture must exit 0; stderr: {}",
            stderr(&out)
        );
        let counts = parse_summary_counts(&stdout(&out));
        assert_eq!(
            counts,
            (verified, unsupported, rejected),
            "{name}: three-bucket split drifted from the pinned truth \
             (must match oxilean-export replay_smoke)"
        );
    }
}

// ---------------------------------------------------------------------------
// Summary-line format
// ---------------------------------------------------------------------------

#[test]
fn summary_line_format_uses_three_named_buckets() {
    let out = run(&[fixture("simple_add").to_str().unwrap()]);
    assert_eq!(code(&out), 0);
    let text = stdout(&out);
    // Find the summary line (mechanics, not counts): "<n> verified · <n>
    // unsupported · <n> rejected".
    let summary = text
        .lines()
        .find(|l| l.contains("verified") && l.contains("unsupported") && l.contains("rejected"))
        .expect("a summary line must be printed");
    assert!(
        summary.contains('\u{00b7}'),
        "uses the middle-dot separator"
    );
    // Buckets are kept separate: three distinct words, never a merged figure.
    assert!(summary.contains("verified"));
    assert!(summary.contains("unsupported"));
    assert!(summary.contains("rejected"));
}

#[test]
fn quiet_suppresses_per_decl_lines_but_keeps_summary() {
    let out = run(&["--quiet", fixture("simple_add").to_str().unwrap()]);
    assert_eq!(code(&out), 0);
    let text = stdout(&out);
    // No per-decl glyph lines.
    assert!(
        !text.contains('\u{2713}'),
        "no verified glyph in quiet mode"
    );
    assert!(
        !text.contains('\u{2298}'),
        "no unsupported glyph in quiet mode"
    );
    // But the summary is still present.
    assert!(text.contains("verified") && text.contains("rejected"));
}

// ---------------------------------------------------------------------------
// JSON report
// ---------------------------------------------------------------------------

#[test]
fn json_report_is_well_formed_and_has_pins() {
    let out = run(&[
        "--quiet",
        "--json",
        "-",
        fixture("simple_add").to_str().unwrap(),
    ]);
    assert_eq!(code(&out), 0, "stderr: {}", stderr(&out));
    let json = stdout(&out);

    // Balanced braces/brackets — a cheap well-formedness check without a JSON
    // parser dependency.
    assert_balanced(&json);

    // The provenance pins the brief requires are present.
    for needle in [
        "\"tool\"",
        "\"name\": \"oxilean-verify\"",
        "\"pins\"",
        "\"lean4export_commit\"",
        "3de59f10bc4b4a0f2de698597aeb1246caa0df0a",
        "\"lean_toolchain\"",
        "\"reader_format_version\"",
        "\"input\"",
        "\"totals\"",
        "\"verified\"",
        "\"unsupported\"",
        "\"rejected\"",
        "\"unsupported_features\"",
    ] {
        assert!(
            json.contains(needle),
            "JSON report missing {needle}:\n{json}"
        );
    }
}

#[test]
fn json_full_adds_per_decl_list() {
    let out = run(&[
        "--quiet",
        "--json-full",
        "--json",
        "-",
        fixture("simple_add").to_str().unwrap(),
    ]);
    assert_eq!(code(&out), 0);
    let json = stdout(&out);
    assert!(
        json.contains("\"decls\""),
        "json-full must include a decls array"
    );
    assert_balanced(&json);
}

#[test]
fn json_report_can_be_written_to_a_file() {
    let json_path = scratch_path("report.json");
    let out = run(&[
        "--quiet",
        "--json",
        json_path.to_str().unwrap(),
        fixture("simple_add").to_str().unwrap(),
    ]);
    assert_eq!(code(&out), 0);
    let written = std::fs::read_to_string(&json_path).expect("report file must exist");
    let _ = std::fs::remove_file(&json_path);
    assert_balanced(&written);
    assert!(written.contains("\"totals\""));
}

// ---------------------------------------------------------------------------
// Determinism
// ---------------------------------------------------------------------------

#[test]
fn two_runs_are_byte_identical_after_timing_normalisation() {
    let f = fixture("simple_add");
    let a = run(&["--no-color", f.to_str().unwrap()]);
    let b = run(&["--no-color", f.to_str().unwrap()]);
    assert_eq!(code(&a), code(&b));
    let na = normalise_timing(&stdout(&a));
    let nb = normalise_timing(&stdout(&b));
    assert_eq!(
        na, nb,
        "streaming output must be deterministic modulo timing"
    );
}

#[test]
fn json_reports_are_deterministic_after_timing_normalisation() {
    let f = fixture("simple_add");
    let a = run(&["--quiet", "--json", "-", f.to_str().unwrap()]);
    let b = run(&["--quiet", "--json", "-", f.to_str().unwrap()]);
    let na = normalise_timing(&stdout(&a));
    let nb = normalise_timing(&stdout(&b));
    assert_eq!(na, nb, "JSON report must be deterministic modulo wall time");
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Assert the string has balanced `{}` and `[]` (ignoring braces inside JSON
/// string literals). A lightweight structural check that needs no JSON crate.
fn assert_balanced(s: &str) {
    let mut depth_curly: i64 = 0;
    let mut depth_square: i64 = 0;
    let mut in_string = false;
    let mut escaped = false;
    for ch in s.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' => depth_curly += 1,
            '}' => depth_curly -= 1,
            '[' => depth_square += 1,
            ']' => depth_square -= 1,
            _ => {}
        }
        assert!(
            depth_curly >= 0 && depth_square >= 0,
            "unbalanced close in JSON"
        );
    }
    assert_eq!(depth_curly, 0, "unbalanced curly braces in JSON");
    assert_eq!(depth_square, 0, "unbalanced square brackets in JSON");
    assert!(!in_string, "unterminated string in JSON");
}

/// Replace the two timing surfaces (per-decl `N.N ms` and the JSON `wall_ms`
/// value) so the deterministic parts of the output can be compared.
fn normalise_timing(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for line in s.lines() {
        let mut line = mask_ms_column(line);
        line = mask_wall_ms(&line);
        line = mask_micros(&line);
        out.push_str(&line);
        out.push('\n');
    }
    out
}

/// Replace a trailing `<digits>.<digit> ms` timing column with a fixed token.
fn mask_ms_column(line: &str) -> String {
    if let Some(idx) = line.rfind(" ms") {
        // Walk left over the number "d.d".
        let bytes = line.as_bytes();
        let mut start = idx;
        while start > 0 {
            let c = bytes[start - 1];
            if c.is_ascii_digit() || c == b'.' || c == b' ' {
                start -= 1;
            } else {
                break;
            }
        }
        // Only treat it as a timing column if what precedes looks like a number.
        let candidate = &line[start..idx];
        if candidate.chars().any(|c| c.is_ascii_digit()) {
            let mut s = String::new();
            s.push_str(&line[..start]);
            s.push_str(" <MS>");
            return s;
        }
    }
    line.to_string()
}

/// Replace `"wall_ms": <n>` with a fixed token.
fn mask_wall_ms(line: &str) -> String {
    mask_json_int_field(line, "\"wall_ms\":")
}

/// Replace `"micros": <n>` with a fixed token (present under --json-full).
fn mask_micros(line: &str) -> String {
    mask_json_int_field(line, "\"micros\":")
}

fn mask_json_int_field(line: &str, key: &str) -> String {
    if let Some(pos) = line.find(key) {
        let after = pos + key.len();
        let rest = &line[after..];
        // Skip spaces then digits.
        let trimmed = rest.trim_start();
        let lead = rest.len() - trimmed.len();
        let digits_end = trimmed
            .char_indices()
            .find(|(_, c)| !c.is_ascii_digit())
            .map_or(trimmed.len(), |(i, _)| i);
        let tail = &trimmed[digits_end..];
        let mut s = String::new();
        s.push_str(&line[..after]);
        s.push_str(&" ".repeat(lead.max(1)));
        s.push_str("<N>");
        s.push_str(tail);
        return s;
    }
    line.to_string()
}
