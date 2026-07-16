//! Malformed-input suite: every broken input must produce a precise,
//! three-bucket [`ExportError`] — never a panic, never a stack overflow.

use oxilean_export::{json, read_str, ExportError};

const META: &str = r#"{"meta":{"exporter":{"name":"lean4export","version":"3.1.0"},"format":{"version":"3.1.0"},"lean":{"githash":"x","version":"4.32.0-rc1"}}}"#;

fn with_meta(lines: &[&str]) -> String {
    let mut s = String::from(META);
    for l in lines {
        s.push('\n');
        s.push_str(l);
    }
    s
}

fn expect_malformed(input: &str) -> ExportError {
    match read_str(input) {
        Ok(_) => panic!("input must be rejected: {input:?}"),
        Err(e) => {
            assert!(e.is_malformed(), "expected Malformed, got {e:?}");
            e
        }
    }
}

// --- Truncated / invalid JSON ---------------------------------------------

#[test]
fn truncated_json_line_is_malformed() {
    let e = expect_malformed(&with_meta(&[r#"{"in":1,"str":{"pre":0,"#]));
    assert_eq!(e.position().line, 2, "error must point at the broken line");
}

#[test]
fn truncated_meta_is_malformed() {
    let e = expect_malformed(r#"{"meta":{"exporter""#);
    assert_eq!(e.position().line, 1);
}

#[test]
fn empty_input_is_malformed() {
    let e = expect_malformed("");
    assert!(format!("{e}").contains("no meta record"));
}

#[test]
fn garbage_bytes_are_malformed() {
    expect_malformed("\u{0}\u{1}\u{2}not json at all");
}

#[test]
fn json_scalar_line_is_malformed() {
    // A valid JSON value that is not an object is still a malformed record.
    expect_malformed(&with_meta(&["42"]));
}

#[test]
fn duplicate_json_keys_are_malformed() {
    expect_malformed(&with_meta(&[
        r#"{"in":1,"in":2,"str":{"pre":0,"str":"a"}}"#,
    ]));
}

// --- String escapes ---------------------------------------------------------

#[test]
fn bad_escape_is_malformed() {
    let e = expect_malformed(&with_meta(&[r#"{"in":1,"str":{"pre":0,"str":"\x"}}"#]));
    assert_eq!(e.position().line, 2);
}

#[test]
fn lone_high_surrogate_is_malformed() {
    expect_malformed(&with_meta(&[r#"{"in":1,"str":{"pre":0,"str":"\ud800"}}"#]));
}

#[test]
fn lone_low_surrogate_is_malformed() {
    expect_malformed(&with_meta(&[
        r#"{"in":1,"str":{"pre":0,"str":"\udc00 tail"}}"#,
    ]));
}

#[test]
fn valid_surrogate_pair_decodes() {
    // Positive control: a valid surrogate pair must round-trip (this is the
    // U+1F600 emoji), proving the escape decoder handles pairs.
    let input = with_meta(&[r#"{"in":1,"str":{"pre":0,"str":"😀"}}"#]);
    let f = read_str(&input).unwrap_or_else(|e| panic!("surrogate pair must parse: {e}"));
    assert_eq!(f.stats.name_str, 1);
}

#[test]
fn unescaped_control_char_is_malformed() {
    expect_malformed(&with_meta(&[
        "{\"in\":1,\"str\":{\"pre\":0,\"str\":\"a\u{1}b\"}}",
    ]));
}

// --- Reference discipline ---------------------------------------------------

#[test]
fn forward_reference_is_malformed() {
    // app refers to expr indices that are not yet defined.
    let e = expect_malformed(&with_meta(&[r#"{"app":{"fn":1,"arg":2},"ie":0}"#]));
    assert!(format!("{e}").contains("not yet defined"), "got: {e}");
}

#[test]
fn forward_name_reference_is_malformed() {
    // Name prefix index 7 does not exist yet.
    expect_malformed(&with_meta(&[r#"{"in":1,"str":{"pre":7,"str":"x"}}"#]));
}

#[test]
fn duplicate_index_is_malformed() {
    let e = expect_malformed(&with_meta(&[
        r#"{"in":1,"str":{"pre":0,"str":"a"}}"#,
        r#"{"in":1,"str":{"pre":0,"str":"b"}}"#,
    ]));
    assert!(format!("{e}").contains("duplicate"), "got: {e}");
    assert_eq!(e.position().line, 3);
}

#[test]
fn non_contiguous_index_is_malformed() {
    // Index 5 assigned when 1 is expected: the tables are dense per spec.
    let e = expect_malformed(&with_meta(&[r#"{"in":5,"str":{"pre":0,"str":"a"}}"#]));
    assert!(format!("{e}").contains("non-contiguous"), "got: {e}");
}

#[test]
fn seeded_index_zero_cannot_be_redefined() {
    // Index 0 is pre-seeded (anonymous name); re-assigning it is a duplicate.
    let e = expect_malformed(&with_meta(&[r#"{"in":0,"str":{"pre":0,"str":"a"}}"#]));
    assert!(format!("{e}").contains("duplicate"), "got: {e}");
}

#[test]
fn negative_index_is_malformed() {
    expect_malformed(&with_meta(&[r#"{"in":-1,"str":{"pre":0,"str":"a"}}"#]));
}

// --- Meta validation --------------------------------------------------------

#[test]
fn wrong_major_version_is_unsupported_with_named_feature() {
    let input = r#"{"meta":{"exporter":{"name":"lean4export","version":"4.0.0"},"format":{"version":"4.0.0"},"lean":{"githash":"x","version":"y"}}}"#;
    match read_str(input) {
        Ok(_) => panic!("major version 4 must be rejected"),
        Err(ExportError::Unsupported { feature, .. }) => {
            assert!(
                feature.contains("future NDJSON format major version"),
                "feature must be named: {feature}"
            );
        }
        Err(other) => panic!("expected Unsupported, got {other:?}"),
    }
}

#[test]
fn legacy_major_version_is_unsupported_with_named_feature() {
    let input = r#"{"meta":{"exporter":{"name":"lean4export","version":"1.0.0"},"format":{"version":"1.0.0"},"lean":{"githash":"x","version":"y"}}}"#;
    match read_str(input) {
        Err(ExportError::Unsupported { feature, .. }) => {
            assert!(
                feature.contains("legacy"),
                "feature must be named: {feature}"
            );
        }
        other => panic!("expected Unsupported, got {other:?}"),
    }
}

#[test]
fn unparseable_version_is_malformed() {
    let input = r#"{"meta":{"exporter":{"name":"x","version":"v"},"format":{"version":"three"},"lean":{"githash":"x","version":"y"}}}"#;
    expect_malformed(input);
}

#[test]
fn missing_meta_fields_are_malformed() {
    expect_malformed(r#"{"meta":{"exporter":{"name":"x"}}}"#);
}

#[test]
fn first_record_not_meta_is_malformed() {
    expect_malformed(r#"{"in":1,"str":{"pre":0,"str":"a"}}"#);
}

#[test]
fn format_3_0_0_is_accepted() {
    // Positive control: major version 3 minors are accepted (the Nat.add_succ
    // fixture is 3.0.0).
    let input = r#"{"meta":{"exporter":{"name":"lean4export","version":"3.0.0"},"format":{"version":"3.0.0"},"lean":{"githash":"x","version":"y"}}}"#;
    let f = read_str(input).unwrap_or_else(|e| panic!("3.0.0 must parse: {e}"));
    assert_eq!(f.meta.format_major(), Some(3));
}

// --- Record-shape violations -------------------------------------------------

#[test]
fn unknown_discriminator_is_malformed() {
    let e = expect_malformed(&with_meta(&[r#"{"frobnicate":{"a":1}}"#]));
    assert!(format!("{e}").contains("frobnicate"), "got: {e}");
}

#[test]
fn natval_as_number_is_malformed() {
    // Per spec natVal is a decimal STRING; a JSON number is a spec violation.
    expect_malformed(&with_meta(&[r#"{"ie":0,"natVal":123}"#]));
}

#[test]
fn natval_with_junk_digits_is_malformed() {
    expect_malformed(&with_meta(&[r#"{"ie":0,"natVal":"12a3"}"#]));
}

#[test]
fn natval_with_sign_is_malformed() {
    expect_malformed(&with_meta(&[r#"{"ie":0,"natVal":"-5"}"#]));
}

#[test]
fn bad_binder_info_is_malformed() {
    let e = expect_malformed(&with_meta(&[
        r#"{"il":1,"succ":0}"#,
        r#"{"ie":0,"sort":1}"#,
        r#"{"ie":1,"lam":{"binderInfo":"superImplicit","body":0,"name":0,"type":0}}"#,
    ]));
    assert!(format!("{e}").contains("binderInfo"), "got: {e}");
}

#[test]
fn bad_quot_kind_is_malformed() {
    expect_malformed(&with_meta(&[
        r#"{"il":1,"succ":0}"#,
        r#"{"ie":0,"sort":1}"#,
        r#"{"quot":{"kind":"squash","levelParams":[],"name":0,"type":0}}"#,
    ]));
}

#[test]
fn bad_hints_is_malformed() {
    expect_malformed(&with_meta(&[
        r#"{"il":1,"succ":0}"#,
        r#"{"ie":0,"sort":1}"#,
        r#"{"def":{"all":[0],"hints":"sometimes","levelParams":[],"name":0,"safety":"safe","type":0,"value":0}}"#,
    ]));
}

#[test]
fn bad_safety_is_malformed() {
    expect_malformed(&with_meta(&[
        r#"{"il":1,"succ":0}"#,
        r#"{"ie":0,"sort":1}"#,
        r#"{"def":{"all":[0],"hints":"abbrev","levelParams":[],"name":0,"safety":"mostly","type":0,"value":0}}"#,
    ]));
}

#[test]
fn max_with_three_elements_is_malformed() {
    expect_malformed(&with_meta(&[r#"{"il":1,"max":[0,0,0]}"#]));
}

#[test]
fn bvar_out_of_u32_range_is_malformed() {
    expect_malformed(&with_meta(&[r#"{"bvar":4294967296,"ie":0}"#]));
}

// --- Depth-limit: nesting hits the limit, not the stack ----------------------

#[test]
fn giant_array_nesting_hits_depth_limit_not_stack() {
    // 200k-deep array: a recursive parser would overflow the stack; ours must
    // return a depth error.
    let n = 200_000;
    let mut s = String::with_capacity(2 * n);
    for _ in 0..n {
        s.push('[');
    }
    for _ in 0..n {
        s.push(']');
    }
    let err = json::parse(s.as_bytes()).expect_err("deep nesting must be rejected");
    assert!(err.message.contains("MAX_DEPTH"), "got: {}", err.message);
}

#[test]
fn giant_object_nesting_hits_depth_limit_not_stack() {
    let n = 200_000;
    let mut s = String::new();
    for _ in 0..n {
        s.push_str("{\"k\":");
    }
    s.push('1');
    for _ in 0..n {
        s.push('}');
    }
    let err = json::parse(s.as_bytes()).expect_err("deep nesting must be rejected");
    assert!(err.message.contains("MAX_DEPTH"), "got: {}", err.message);
}

#[test]
fn giant_nesting_through_full_reader_is_malformed() {
    // The same guarantee must hold through the NDJSON pipeline.
    let n = 200_000;
    let mut line = String::from(r#"{"ie":0,"mdata":{"expr":"#);
    for _ in 0..n {
        line.push('[');
    }
    for _ in 0..n {
        line.push(']');
    }
    line.push_str("}}");
    let e = expect_malformed(&with_meta(&[&line]));
    assert_eq!(e.position().line, 2);
}

#[test]
fn nesting_within_limit_parses() {
    // Positive control: depth just under the limit is fine.
    let n = 100;
    let mut s = String::new();
    for _ in 0..n {
        s.push('[');
    }
    s.push('1');
    for _ in 0..n {
        s.push(']');
    }
    assert!(json::parse(s.as_bytes()).is_ok());
}

// --- JSON parser conformance --------------------------------------------------

#[test]
fn json_rejects_trailing_bytes() {
    assert!(json::parse(b"{} extra").is_err());
    assert!(json::parse(b"1 2").is_err());
}

#[test]
fn json_rejects_leading_zero_numbers() {
    assert!(json::parse(b"[01]").is_err());
}

#[test]
fn json_parses_floats_and_negatives() {
    // The export format itself never uses floats, but the JSON layer accepts
    // them (spec-complete minimal JSON).
    assert!(matches!(
        json::parse(b"-1.5e3"),
        Ok(json::JsonValue::Float(_))
    ));
    assert!(matches!(json::parse(b"-7"), Ok(json::JsonValue::Int(s)) if s == "-7"));
}

#[test]
fn json_parses_booleans_and_null() {
    assert!(matches!(
        json::parse(b"true"),
        Ok(json::JsonValue::Bool(true))
    ));
    assert!(matches!(
        json::parse(b"false"),
        Ok(json::JsonValue::Bool(false))
    ));
    assert!(matches!(json::parse(b"null"), Ok(json::JsonValue::Null)));
}

#[test]
fn json_all_simple_escapes_decode() {
    let v = json::parse(br#""\" \\ \/ \b \f \n \r \t A""#)
        .unwrap_or_else(|e| panic!("escapes must parse: {}", e.message));
    assert_eq!(v.as_str(), Some("\" \\ / \u{8} \u{c} \n \r \t A"));
}

#[test]
fn json_error_positions_are_byte_offsets() {
    let err = json::parse(b"[1, oops]").expect_err("must fail");
    assert_eq!(err.offset, 4, "error at the 'o' of oops");
}

// --- Sharing bombs: budgeted materialization, never OOM ----------------------

#[test]
fn doubling_app_bomb_materializes_as_shared_dag_not_exploded() {
    // 128 doubling app records (node i = App(i-1, i-1)) denote a tree of 2^128
    // nodes but a DAG of only 129 DISTINCT nodes. Structural sharing (wave5)
    // materializes each distinct id exactly once as a shared `Rc`, so the
    // "bomb" is a small shared DAG: no exponential expansion, no OOM, and it is
    // NOT rejected — it is a legal, bounded declaration.
    //
    // (The exponential blow-up can still be provoked by *un-sharing* the DAG
    // during kernel reduction; that path is bounded independently by the
    // per-declaration clone-fuel at verify time — see oxilean-kernel::fuel —
    // never by the reader, which only materializes.)
    let mut lines = vec![
        r#"{"in":1,"str":{"pre":0,"str":"bomb"}}"#.to_string(),
        r#"{"il":1,"succ":0}"#.to_string(),
        r#"{"ie":0,"sort":1}"#.to_string(),
    ];
    for i in 1..=128u32 {
        let prev = i - 1;
        lines.push(format!(
            r#"{{"app":{{"fn":{prev},"arg":{prev}}},"ie":{i}}}"#
        ));
    }
    lines.push(r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":1,"type":128}}"#.to_string());
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let input = with_meta(&refs);

    let start = std::time::Instant::now();
    let file = read_str(&input).expect("shared bomb materializes cleanly");
    // Not rejected: sharing turns the bomb into a bounded DAG.
    assert_eq!(
        file.stats.decl_oversized, 0,
        "sharing defuses the expr bomb"
    );
    match &file.decls[..] {
        [oxilean_export::ExportDecl::Axiom(v)] => {
            assert_eq!(v.common.name.to_string(), "bomb");
        }
        other => panic!("expected one Axiom decl, got {other:?}"),
    }
    // Bounded by DISTINCT nodes (~129 exprs + a couple of level nodes), NOT the
    // 2^128-node tree. The exact figure may drift with later sharing stages;
    // the guarantee under test is "small, not exponential".
    assert!(
        file.stats.materialized_nodes < 1_000,
        "materialized {} nodes — must be the DAG's distinct-node count, not the tree",
        file.stats.materialized_nodes
    );
    assert!(
        start.elapsed() < std::time::Duration::from_secs(5),
        "shared materialization must be O(distinct nodes)"
    );
}

#[test]
fn doubling_level_bomb_is_budget_capped_not_oom() {
    // Same bomb at the level layer, via max[k,k] doubling under a sort.
    let mut lines = vec![r#"{"in":1,"str":{"pre":0,"str":"bomb"}}"#.to_string()];
    lines.push(r#"{"il":1,"succ":0}"#.to_string());
    for i in 2..=128usize {
        let prev = i - 1;
        lines.push(format!(r#"{{"il":{i},"max":[{prev},{prev}]}}"#));
    }
    lines.push(r#"{"ie":0,"sort":128}"#.to_string());
    lines.push(r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":1,"type":0}}"#.to_string());
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let input = with_meta(&refs);

    // C22: recovered as a named Oversized declaration, like the app bomb.
    let file = read_str(&input).expect("level bomb decl is skipped, not a file error");
    assert_eq!(file.stats.decl_oversized, 1);
    match &file.decls[..] {
        [oxilean_export::ExportDecl::Oversized(v)] => {
            assert_eq!(v.feature, oxilean_export::DECL_BUDGET_FEATURE);
            assert_eq!(v.names[0].to_string(), "bomb");
        }
        other => panic!("expected one Oversized decl, got {other:?}"),
    }
}

#[test]
fn bomb_without_decl_parses_fine() {
    // The bomb records themselves are legal; only materialization is budgeted.
    // A file that never references them from a declaration parses cleanly.
    let mut lines = vec![
        r#"{"il":1,"succ":0}"#.to_string(),
        r#"{"ie":0,"sort":1}"#.to_string(),
    ];
    for i in 1..=128u32 {
        let prev = i - 1;
        lines.push(format!(
            r#"{{"app":{{"fn":{prev},"arg":{prev}}},"ie":{i}}}"#
        ));
    }
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    let input = with_meta(&refs);
    let f = read_str(&input).unwrap_or_else(|e| panic!("bomb records alone must parse: {e}"));
    assert_eq!(f.stats.expr_app, 128);
    assert_eq!(f.stats.materialized_nodes, 0, "nothing materialized");
}

#[test]
fn budget_is_configurable_via_limits() {
    // A tiny budget rejects even a modest declaration; a large budget accepts.
    let input = with_meta(&[
        r#"{"in":1,"str":{"pre":0,"str":"c"}}"#,
        r#"{"il":1,"succ":0}"#,
        r#"{"ie":0,"sort":1}"#,
        r#"{"app":{"fn":0,"arg":0},"ie":1}"#,
        r#"{"axiom":{"isUnsafe":false,"levelParams":[],"name":1,"type":1}}"#,
    ]);
    let tiny = oxilean_export::Limits {
        materialize_budget: 2,
        decl_materialize_budget: u64::MAX,
    };
    match oxilean_export::read_with_limits(std::io::Cursor::new(input.as_str()), tiny) {
        Err(ExportError::Unsupported { feature, .. }) => {
            assert_eq!(feature, oxilean_export::BUDGET_FEATURE);
        }
        other => panic!("tiny budget must trip: {other:?}"),
    }
    // The same tiny bound applied PER DECLARATION is recovered instead (C22).
    let tiny_decl = oxilean_export::Limits {
        materialize_budget: 1 << 20,
        decl_materialize_budget: 2,
    };
    let f = oxilean_export::read_with_limits(std::io::Cursor::new(input.as_str()), tiny_decl)
        .expect("per-decl breach is a named skip, not a file error");
    assert_eq!(f.stats.decl_oversized, 1);
    let big = oxilean_export::Limits {
        materialize_budget: 1 << 20,
        decl_materialize_budget: 1 << 20,
    };
    let f = oxilean_export::read_with_limits(std::io::Cursor::new(input.as_str()), big)
        .unwrap_or_else(|e| panic!("large budget must pass: {e}"));
    // The axiom type is app(sort, sort) where both children are the SAME expr
    // id. Structural sharing (wave5) materializes that id once, so the reader
    // builds 4 distinct nodes — the app, the one shared sort, and its two level
    // nodes (succ, zero) — rather than the 7-node exploded tree.
    assert_eq!(f.stats.materialized_nodes, 4);
}
