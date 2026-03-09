//! Basic normalization and parsing tests (non-ignore).

use super::normalize::normalize_lean4_to_oxilean;
use super::normalize_2::normalize_bounded_quantifiers;
use super::normalize_3::{normalize_head_binders, normalize_subscript_indexing};
use super::test_infra::{extract_single_line_decls, try_parse_decl, try_parse_decl_err};

#[test]
fn test_normalization_basics() {
    let s = normalize_lean4_to_oxilean("fun x => x + 1");
    assert!(s.contains("->"), "Expected -> in: {s}");
    let s = normalize_lean4_to_oxilean("fun f : \u{2115} \u{2192} \u{2115} => f 0");
    assert!(s.contains("Nat"), "Expected Nat in: {s}");
    let s = normalize_lean4_to_oxilean("fun x \u{21A6} x");
    assert!(s.contains("->"), "Expected -> in: {s}");
}
#[test]
fn test_head_binder_normalization() {
    let s = normalize_head_binders("theorem foo (x : Nat) : x = x := rfl");
    assert!(
        s.contains("forall"),
        "Expected forall after head-binder normalization: {s}"
    );
    assert!(s.contains("(x : Nat)"), "Expected binder in forall: {s}");
    assert!(try_parse_decl(&s), "Normalized decl should parse: {s}");
    let s = normalize_head_binders("theorem bar {p : Prop} : p -> p := fun h -> h");
    assert!(s.contains("forall"), "Expected forall: {s}");
    assert!(
        try_parse_decl(&s),
        "Normalized implicit binder should parse: {s}"
    );
    let s = normalize_head_binders("theorem baz (n m : Nat) {p : Prop} : n = m -> p := sorry");
    assert!(s.contains("forall"), "Expected forall: {s}");
    assert!(try_parse_decl(&s), "Multiple binders should parse: {s}");
    let orig = "theorem no_binders : forall (n : Nat), n = n := rfl";
    let s = normalize_head_binders(orig);
    assert_eq!(s, orig, "No-binder theorem should be unchanged");
    let s = normalize_head_binders("theorem Nat.add_comm (n m : Nat) : n + m = m + n := sorry");
    assert!(
        s.contains("Nat_add_comm"),
        "Expected dotted name normalized to Nat_add_comm: {s}"
    );
    assert!(s.contains("forall"), "Expected forall: {s}");
    assert!(try_parse_decl(&s), "Dotted name theorem should parse: {s}");
    let s = normalize_head_binders("theorem univ.{u} : Prop := Prop");
    assert!(s.contains("univ"), "Should contain univ: {s}");
}
#[test]
fn test_tactic_proof_normalization() {
    let s = normalize_lean4_to_oxilean("theorem foo : forall (n : Nat), n = n := by rfl");
    assert!(s.contains(":= sorry"), "Expected sorry: {s}");
    assert!(
        try_parse_decl(&s),
        "Tactic-proof theorem should parse after sorry: {s}"
    );
    let s =
        normalize_lean4_to_oxilean("theorem bar : forall (n m : Nat), n + m = m + n := by simp");
    assert!(s.contains(":= sorry"), "Expected sorry: {s}");
    let s =
        normalize_lean4_to_oxilean("theorem Nat.add_comm (n m : Nat) : n + m = m + n := by ring");
    assert!(s.contains("Nat_add_comm"), "Expected Nat_add_comm: {s}");
    assert!(s.contains(":= sorry"), "Expected sorry: {s}");
    assert!(try_parse_decl(&s), "Should parse: {s}");
    let s = normalize_lean4_to_oxilean("@[simp] theorem foo2 : forall (n : Nat), n = n := by rfl");
    assert!(s.contains(":= sorry"), "Expected sorry: {s}");
    assert!(!s.starts_with("@["), "Attribute should be stripped: {s}");
    assert!(try_parse_decl(&s), "Attr+tactic theorem should parse: {s}");
    let s =
        normalize_lean4_to_oxilean("theorem foo3 (alpha : Sort*) : alpha -> alpha := fun h -> h");
    assert!(s.contains("Type"), "Expected Sort* -> Type: {s}");
    let s = normalize_lean4_to_oxilean(
        "theorem fact_iff {p : Prop} : Fact p \u{2194} p := \u{27E8}fun h \u{21A6} h.1, fun h \u{21A6} \u{27E8}h\u{27E9}\u{27E9}",
    );
    assert!(s.contains(":= sorry"), "Expected sorry for term proof: {s}");
    let s = normalize_lean4_to_oxilean("def id_fn : Nat -> Nat := fun n -> n");
    assert!(
        s.contains(":= sorry"),
        "Expected sorry for def term proof: {s}"
    );
}
#[test]
fn test_extract_decls_basic() {
    let content = r#"
theorem add_comm : n + m = m + n := by ring
def double (n : Nat) : Nat := n + n
-- comment
import Foo
open Bar
namespace Baz
theorem foo : True := trivial
"#;
    let decls = extract_single_line_decls(content);
    assert!(
        !decls.is_empty(),
        "Should extract at least some declarations"
    );
    for d in &decls {
        let ok = d.starts_with("theorem ")
            || d.starts_with("def ")
            || d.starts_with("lemma ")
            || d.starts_with("axiom ");
        assert!(ok, "Unexpected extraction: {d}");
    }
}
#[test]
fn test_parse_simple_oxilean_decls() {
    let decls = vec![
        "axiom em : forall (p : Prop), p \u{2228} \u{00AC} p",
        "theorem t1 : forall (n : Nat), n = n := rfl",
        "def id_nat : Nat -> Nat := fun n -> n",
        "theorem t2 : Prop := True",
    ];
    for decl in &decls {
        assert!(try_parse_decl(decl), "Failed to parse OxiLean decl: {decl}");
    }
}
#[test]
fn test_bounded_quantifier_normalization() {
    let s = normalize_bounded_quantifiers("ISup k < n + 1, u k");
    assert!(s.contains("fun k ->"), "Expected lambda form: {s}");
    assert!(!s.contains("< n + 1"), "Bound should be dropped: {s}");
    let s = normalize_bounded_quantifiers("IInf k \u{2264} n, f k");
    assert!(s.contains("fun k ->"), "Expected lambda form for IInf: {s}");
}
#[test]
fn test_subscript_normalization() {
    let s = normalize_subscript_indexing("l[0]");
    assert_eq!(s, "l", "Subscript should be dropped: {s}");
    let s = normalize_subscript_indexing("xs[n]");
    assert_eq!(s, "xs", "Variable subscript should be dropped: {s}");
    let s = normalize_subscript_indexing("[1, 2, 3]");
    assert_eq!(s, "[1, 2, 3]", "List literal should be preserved: {s}");
}
#[test]
fn test_paren_dot_field_normalization() {
    // (expr).field patterns: convert to function-call form instead of stripping
    let decl1 = "theorem Cont.then_eval {k k' : Cont} {v} : (k.then k').eval v = k.eval v >>= k'.eval := by cases k <;> simp";
    let n1 = normalize_lean4_to_oxilean(decl1);
    assert!(try_parse_decl(&n1), "Should parse (expr).field: {n1}");

    let decl2 = "theorem stepRet_then {k k' : Cont} {v} : stepRet (k.then k') v = (stepRet k v).then k' := by cases k <;> simp";
    let n2 = normalize_lean4_to_oxilean(decl2);
    assert!(try_parse_decl(&n2), "Should parse (expr).field: {n2}");
}
#[test]
fn test_pi_forall_inside_integral_binder() {
    // Test: integral with typed binder `(x : ℝ)` should produce `fun x ->`, not `fun (x ->`
    let decl = "theorem foo : \u{2A0D} (x : \u{211D}) in a..b, f x = 0 := sorry";
    let n = normalize_lean4_to_oxilean(decl);
    assert!(
        !n.contains("fun (x ->"),
        "Integral binder should not have paren around var: {n}"
    );
    assert!(
        try_parse_decl(&n),
        "Integral with typed binder should parse: {n}"
    );
}
#[test]
fn test_nested_norm_notation() {
    // Test: nested ‖...‖ should produce correct nesting, not garbled toggle
    let decl = "theorem foo : \u{2016}fderiv (fun x => \u{2016}f x\u{2016}) x\u{2016} = 0 := sorry";
    let n = normalize_lean4_to_oxilean(decl);
    // Should contain nested (Norm ...(Norm ...)...)
    assert!(
        !n.contains("->  )"),
        "Nested norm should not produce empty fun body: {n}"
    );
    assert!(
        n.contains("(Norm") && n.matches("(Norm").count() >= 2,
        "Should have nested Norm: {n}"
    );
}
#[test]
fn test_def_without_return_type() {
    let s = normalize_head_binders("def Xor' (a b : Prop) := sorry");
    assert!(s.starts_with("def Xor"), "Should start with def Xor: {s}");
    assert!(s.contains(":= sorry"), "Should have sorry: {s}");
    assert!(
        !s.contains("(a b : Prop)"),
        "Binders should be dropped: {s}"
    );
    assert!(
        try_parse_decl(&s),
        "def without return type should parse after normalization: {s}"
    );
}

#[test]
fn test_pi_forall_inside_fun_binder() {
    // Simpler case: just check that (f : ∀ i, F i) -> f i parses correctly
    let simple = "theorem t : forall (g : (fun (f : \u{2200} i, F i) -> f i) Type), g = g := sorry";
    let ok_s = try_parse_decl(simple);
    println!("Simple forall-in-fun: OK={ok_s}: {simple}");

    // The actual failure pattern from mathlib4
    let decl = "theorem t (i : \u{03B9}) (s' : Set (\u{2200} i, F' i)) : DifferentiableOn (\u{1D55C} := \u{1D55C}) (fun f : \u{2200} i, F' i => f i) s' := by have := Fintype.ofFinite \u{03B9}";
    let normalized = normalize_lean4_to_oxilean(decl);
    println!("Pi-in-fun: {normalized}");
    let ok = try_parse_decl(&normalized);
    println!("Parse OK: {ok}");
    // Just check that ∀ i, body is not broken; if it is, this test fails
    // so we can fix the normalization
    assert!(
        !normalized.contains("\u{2200} i,)"),
        "Should not have broken forall i,): {normalized}"
    );
}

#[test]
fn test_forall_inside_binder() {
    // (f : ∀ i, F i) — ∀ inside a typed binder
    let s1 = "theorem t : forall (f : \u{2200} i, F i), f = f := sorry";
    let ok1 = try_parse_decl(s1);
    println!("Forall-in-binder: OK={ok1}: {s1}");
    // If this doesn't parse, we need to wrap: (f : (∀ i, F i))
    if !ok1 {
        let s2 = "theorem t : forall (f : (\u{2200} i, F i)), f = f := sorry";
        let ok2 = try_parse_decl(s2);
        println!("Wrapped forall: OK={ok2}: {s2}");
        assert!(ok2, "Wrapped forall should parse");
    }
}

#[test]
fn test_bigdirectsum_fun_normalization() {
    // ⨁ fun (i : ι), M i → BigDirectSum fun (i : ι) -> M i → should NOT become "BigDirectSum un"
    let s = normalize_lean4_to_oxilean("def x : \u{2A01} fun (i : \u{03B9}), M i := sorry");
    println!("BigDirectSum normalized: {s}");
    assert!(
        !s.contains("BigDirectSum un"),
        "Should not eat 'f' from 'fun': {s}"
    );
    assert!(try_parse_decl(&s), "Should parse: {s}");
}

#[test]
#[ignore]
fn test_diag_error_categories() {
    use std::collections::HashMap;
    use std::path::Path;
    let mathlib_root = match super::test_infra::mathlib4_root() {
        Some(r) => r,
        None => return,
    };
    let root = Path::new(&mathlib_root);
    let mut error_counts: HashMap<String, usize> = HashMap::new();
    let mut error_examples: HashMap<String, Vec<String>> = HashMap::new();
    let mut total = 0usize;
    let mut ok = 0usize;
    fn walk(
        dir: &Path,
        ec: &mut HashMap<String, usize>,
        ee: &mut HashMap<String, Vec<String>>,
        total: &mut usize,
        ok: &mut usize,
        fc: &mut usize,
        max: usize,
    ) {
        if *fc >= max {
            return;
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        let mut sorted: Vec<_> = entries.flatten().collect();
        sorted.sort_by_key(|e| e.path());
        for entry in sorted {
            if *fc >= max {
                break;
            }
            let path = entry.path();
            if path.is_dir() {
                walk(&path, ec, ee, total, ok, fc, max);
            } else if path.extension().and_then(|e| e.to_str()) == Some("lean") {
                *fc += 1;
                let content = match std::fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                let decls = extract_single_line_decls(&content);
                for decl in &decls {
                    let normalized = normalize_lean4_to_oxilean(decl);
                    *total += 1;
                    match try_parse_decl_err(&normalized) {
                        Ok(()) => {
                            *ok += 1;
                        }
                        Err(msg) => {
                            // Bucket by first ~50 chars of error
                            let key: String = msg.chars().take(80).collect();
                            *ec.entry(key.clone()).or_insert(0) += 1;
                            let exs = ee.entry(key).or_default();
                            if exs.len() < 3 {
                                let short: String = normalized.chars().take(200).collect();
                                exs.push(short);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut fc = 0;
    walk(
        root,
        &mut error_counts,
        &mut error_examples,
        &mut total,
        &mut ok,
        &mut fc,
        10000,
    );
    println!("\nTotal: {total}, OK: {ok}, Fail: {}", total - ok);
    let mut sorted: Vec<_> = error_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    for (msg, count) in &sorted {
        println!("\n{count:>5} | {msg}");
        if let Some(exs) = error_examples.get(msg) {
            for ex in exs {
                println!("      EX: {ex}");
            }
        }
    }
}

/// Test match-in-type normalization: match without with clause.
#[test]
fn test_match_in_type_normalization() {
    use super::normalize::normalize_lean4_to_oxilean;
    use super::test_infra::try_parse_decl_err;

    // Match in type where `with` clause gets stripped by proof replacement
    let samples = vec![
        (
            "theorem cmp_veblen (o1 o2 a b) : cmp (veblen o1 a) (veblen o2 b) = match cmp o1 o2 with | .lt => cmp a (veblen o2 b) | .eq => cmp a b | .gt => cmp (veblen o1 a) b := sorry",
            true,
        ),
        (
            "theorem xor_range (n : Nat) : (List.range (n + 1)).foldl xor 0 = match Fin.ofNat 4 n with | 0 => n | 1 => 1 | 2 => n + 1 | _ => 0 := sorry",
            true,
        ),
    ];

    for (decl, expected_ok) in &samples {
        let normalized = normalize_lean4_to_oxilean(decl);
        let result = try_parse_decl_err(&normalized);
        let ok = result.is_ok();
        assert_eq!(
            ok,
            *expected_ok,
            "Expected {expected_ok} for:\n  INPUT: {decl}\n  NORM:  {normalized}\n  ERR:   {}",
            result.err().unwrap_or_default()
        );
    }
}

/// Test that `by let` in type position is stripped.
#[test]
fn test_by_let_in_type_stripped() {
    let input =
        "def algEquiv (e : α ≃ β) [Semiring β] [Algebra R β] : by let semiring := Equiv.semiring e";
    let normalized = normalize_lean4_to_oxilean(input);
    eprintln!("NORM: {normalized}");
    let result = try_parse_decl_err(&normalized);
    assert!(
        result.is_ok(),
        "Should parse: {normalized}\nErr: {}",
        result.err().unwrap_or_default()
    );
}

/// Test that `let _ :=` in type position is stripped.
#[test]
fn test_let_in_type_stripped() {
    let input = "theorem smul_toAlgebra (r : R) (s : S) : let _ := RingHom.toAlgebra i";
    let normalized = normalize_lean4_to_oxilean(input);
    eprintln!("NORM: {normalized}");
    let result = try_parse_decl_err(&normalized);
    assert!(
        result.is_ok(),
        "Should parse: {normalized}\nErr: {}",
        result.err().unwrap_or_default()
    );
}

/// Test that PSigma/Sigma binder normalization works.
#[test]
fn test_sigma_binder_normalization() {
    // Bare binder: PSigma x : T, body
    let input = "def oreCondition : forall (r : R) (s : S), PSigma r' : R, PSigma s' : S, s' * r = r' * s := sorry";
    let normalized = normalize_lean4_to_oxilean(input);
    eprintln!("NORM: {normalized}");
    assert!(
        !normalized.contains("PSigma r'"),
        "PSigma bare binder should be wrapped: {normalized}"
    );
    let result = try_parse_decl_err(&normalized);
    assert!(
        result.is_ok(),
        "Should parse: {normalized}\nErr: {}",
        result.err().unwrap_or_default()
    );
}

/// Test with exact failing PSigma sample from diagnostic.
#[test]
fn test_psigma_ore_div() {
    // From Mathlib/GroupTheory/OreLocalization/Basic.lean
    let input = "def oreDivSMulChar' (r₁ : R) (r₂ : X) (s₁ s₂ : S) : Σ' r' : R, Σ' s' : S, s' * r₁ = r' * s₂ ∧ (r₁ /ₒ s₁) • (r₂ /ₒ s₂) = r' • r₂ /ₒ (s' * s₁) := sorry";
    let normalized = normalize_lean4_to_oxilean(input);
    eprintln!("NORM: {normalized}");
    let result = try_parse_decl_err(&normalized);
    assert!(
        result.is_ok(),
        "Should parse: {normalized}\nErr: {}",
        result.err().unwrap_or_default()
    );
}

/// Diagnostic: categorize all failures across Mathlib4.
#[test]
#[ignore]
fn diag_categorize_failures() {
    use std::collections::HashMap;
    use std::path::Path;

    let root = match super::test_infra::mathlib4_root() {
        Some(r) => r,
        None => {
            println!("MATHLIB4_ROOT not set");
            return;
        }
    };

    let mut error_cats: HashMap<String, usize> = HashMap::new();
    let mut samples: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut total_failures = 0usize;
    let mut file_count = 0usize;

    fn scan(
        dir: &Path,
        fc: &mut usize,
        tf: &mut usize,
        cats: &mut HashMap<String, usize>,
        samps: &mut HashMap<String, Vec<(String, String)>>,
        max: usize,
    ) {
        if *fc >= max {
            return;
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            if *fc >= max {
                break;
            }
            let path = entry.path();
            if path.is_dir() {
                scan(&path, fc, tf, cats, samps, max);
            } else if path.extension().and_then(|e| e.to_str()) == Some("lean") {
                let content = match std::fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                *fc += 1;
                let decls = super::test_infra::extract_single_line_decls(&content);
                for decl in &decls {
                    let normalized = normalize_lean4_to_oxilean(decl);
                    if !super::test_infra::try_parse_decl(&normalized) {
                        *tf += 1;
                        let err = match super::test_infra::try_parse_decl_err(&normalized) {
                            Err(e) => e,
                            Ok(_) => "ok_on_retry".into(),
                        };
                        let cat = if err.starts_with("lex_error") {
                            "LEX_ERROR".into()
                        } else if err.contains("expected") {
                            let w: Vec<&str> = err.splitn(4, ' ').collect();
                            format!("EXPECT_{}", w.get(1).unwrap_or(&"?"))
                        } else {
                            err.chars().take(50).collect::<String>()
                        };
                        *cats.entry(cat.clone()).or_insert(0) += 1;
                        let s = samps.entry(cat).or_default();
                        if s.len() < 3 {
                            let snip = if normalized.len() > 150 {
                                format!(
                                    "{}...",
                                    &normalized[..normalized
                                        .char_indices()
                                        .nth(150)
                                        .map(|(i, _)| i)
                                        .unwrap_or(normalized.len())]
                                )
                            } else {
                                normalized.clone()
                            };
                            s.push((snip, err));
                        }
                    }
                }
            }
        }
    }

    let root_path = Path::new(&root);
    // root is already .../mathlib4/Mathlib — go up one level to scan siblings
    let parent = root_path.parent().unwrap_or(root_path);
    scan(
        root_path,
        &mut file_count,
        &mut total_failures,
        &mut error_cats,
        &mut samples,
        50000,
    );
    let archive_dir = parent.join("Archive");
    scan(
        &archive_dir,
        &mut file_count,
        &mut total_failures,
        &mut error_cats,
        &mut samples,
        50000,
    );
    let counter_dir = parent.join("Counterexamples");
    scan(
        &counter_dir,
        &mut file_count,
        &mut total_failures,
        &mut error_cats,
        &mut samples,
        50000,
    );

    println!("\n=== FAILURE DIAGNOSIS ({total_failures} failures in {file_count} files) ===\n");
    let mut sorted: Vec<_> = error_cats.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (cat, count) in &sorted {
        println!("[{count:4}] {cat}");
        if let Some(s) = samples.get(*cat) {
            for (snippet, err) in s {
                println!("       ERR: {err}");
                println!("       SRC: {snippet}");
                println!();
            }
        }
    }
}
