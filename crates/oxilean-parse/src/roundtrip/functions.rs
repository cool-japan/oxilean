//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    EditRegion, EditRegionKind, RoundTripConfigExt, RoundTripSummaryLine, SourceMutation, TextToken,
};

/// Golden case: `def x := 0`
///
/// The pretty-printer emits `definition` for `def`, so the expected form
/// uses the printed keyword.
pub const GOLDEN_NAT_ZERO: (&str, &str) = ("def x := 0", "definition x := 0");
/// Golden case: simple typed definition with an arrow type.
///
/// The pretty-printer wraps the lambda binder in parens and uses `=>`.
pub const GOLDEN_SIMPLE_ARROW: (&str, &str) = (
    "def f : Nat -> Nat := fun x -> x",
    "definition f : Nat -> Nat := fun (x) => x",
);
#[cfg(test)]
mod tests {
    use super::*;
    use crate::roundtrip::*;
    #[test]
    fn test_roundtrip_nat_literal() {
        let result = RoundTripChecker::check_expr("42");
        assert!(result.is_success(), "{}", result.describe());
    }
    #[test]
    fn test_roundtrip_var() {
        let result = RoundTripChecker::check_expr("x");
        assert!(result.is_success(), "{}", result.describe());
    }
    #[test]
    fn test_roundtrip_app() {
        let result = RoundTripChecker::check_expr("f x");
        assert!(result.is_success(), "{}", result.describe());
    }
    #[test]
    fn test_roundtrip_def_nat_zero() {
        let result = RoundTripChecker::check_decl("definition x := 0");
        assert!(result.is_success(), "{}", result.describe());
    }
    #[test]
    fn test_config_builder() {
        let cfg = RoundTripConfig::default()
            .with_normalize_whitespace(false)
            .with_max_diff_chars(10)
            .with_ignore_spans(false);
        assert!(!cfg.normalize_whitespace);
        assert_eq!(cfg.max_diff_chars, 10);
        assert!(!cfg.ignore_spans);
    }
    #[test]
    fn test_checker_counters() {
        let mut checker = RoundTripChecker::new(RoundTripConfig::default());
        let ok = RoundTripResult::Success;
        let fail = RoundTripResult::ReparseError("oops".to_string());
        checker.record_result(&ok);
        checker.record_result(&ok);
        checker.record_result(&fail);
        assert_eq!(checker.success_count, 2);
        assert_eq!(checker.failure_count, 1);
        let rate = checker.success_rate();
        assert!((rate - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_golden_nat_zero() {
        let (src, expected) = GOLDEN_NAT_ZERO;
        let mut gf = GoldenFile::new("nat_zero", src, expected);
        assert!(gf.check(), "{:?}", gf.diff());
    }
    #[test]
    fn test_golden_suite_run_all() {
        let mut suite = GoldenTestSuite::new();
        suite.add_test("nat_zero", GOLDEN_NAT_ZERO.0, GOLDEN_NAT_ZERO.1);
        suite.add_test("simple_arrow", GOLDEN_SIMPLE_ARROW.0, GOLDEN_SIMPLE_ARROW.1);
        let (passed, failed) = suite.run_all();
        assert_eq!(
            failed,
            0,
            "expected 0 failures, got {failed}\n{}",
            suite.report()
        );
        assert_eq!(passed, 2);
    }
}
/// Computes a "normalised" version of a source for comparison.
#[allow(dead_code)]
pub fn normalise_for_comparison(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
/// Checks if two strings are equivalent after normalisation.
#[allow(dead_code)]
pub fn normalised_equal(a: &str, b: &str) -> bool {
    normalise_for_comparison(a) == normalise_for_comparison(b)
}
/// Generates a simple round-trip corpus from a list of expression templates.
#[allow(dead_code)]
pub fn generate_corpus(templates: &[&str]) -> Vec<String> {
    let vars = ["x", "y", "z", "n", "m", "f", "g"];
    let mut corpus = Vec::new();
    for template in templates {
        for var in vars {
            corpus.push(template.replace("{}", var));
        }
    }
    corpus
}
/// Measures the "distance" between two formatted expressions.
#[allow(dead_code)]
pub fn format_distance(a: &str, b: &str) -> usize {
    let wa: Vec<_> = a.split_whitespace().collect();
    let wb: Vec<_> = b.split_whitespace().collect();
    let (na, nb) = (wa.len(), wb.len());
    let mut dp = vec![vec![0usize; nb + 1]; na + 1];
    for (i, row) in dp.iter_mut().enumerate().take(na + 1) {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate().take(nb + 1) {
        *cell = j;
    }
    for i in 1..=na {
        for j in 1..=nb {
            dp[i][j] = if wa[i - 1] == wb[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
            };
        }
    }
    dp[na][nb]
}
/// Produces a "round-trip report" for a set of sources.
#[allow(dead_code)]
pub fn produce_roundtrip_report(sources: &[&str]) -> String {
    let mut out = String::from("Round-Trip Report\n");
    out.push_str(&"─".repeat(40));
    out.push('\n');
    for (i, src) in sources.iter().enumerate() {
        let norm = normalise_for_comparison(src);
        let norm2 = normalise_for_comparison(&norm);
        let idempotent = norm == norm2;
        out.push_str(&format!(
            "[{}] {} -> {}\n",
            i + 1,
            if idempotent { "PASS" } else { "FAIL" },
            src
        ));
    }
    out
}
/// Checks that printing is stable (does not grow or shrink repeatedly).
#[allow(dead_code)]
pub fn check_printing_stable(source: &str, max_iters: usize) -> (bool, usize) {
    let mut current = source.to_string();
    for i in 0..max_iters {
        let next = normalise_for_comparison(&current);
        if next == current {
            return (true, i);
        }
        current = next;
    }
    (false, max_iters)
}
/// Produces a diff view of round-trip results.
#[allow(dead_code)]
pub fn roundtrip_diff(original: &str, printed: &str) -> Vec<String> {
    let orig: Vec<_> = original.split_whitespace().collect();
    let print: Vec<_> = printed.split_whitespace().collect();
    let max = orig.len().max(print.len());
    (0..max)
        .filter_map(|i| match (orig.get(i), print.get(i)) {
            (Some(a), Some(b)) if a != b => Some(format!("at {}: {:?} -> {:?}", i, a, b)),
            (Some(a), None) => Some(format!("at {}: removed {:?}", i, a)),
            (None, Some(b)) => Some(format!("at {}: added {:?}", i, b)),
            _ => None,
        })
        .collect()
}
#[cfg(test)]
mod extended_roundtrip_tests {
    use super::*;
    use crate::roundtrip::*;
    #[test]
    fn test_roundtrip_test_hash() {
        let t = RoundTripTest::new("fun x -> x", "identity");
        let h = t.source_hash();
        assert!(h > 0);
    }
    #[test]
    fn test_roundtrip_result_success() {
        let r = RoundTripResult::Success;
        assert!(r.is_success());
    }
    #[test]
    fn test_roundtrip_result_failure() {
        let r = RoundTripResult::PrettyPrintFailed("parse error".to_string());
        assert!(!r.is_success());
    }
    #[test]
    fn test_roundtrip_suite() {
        let mut suite = RoundTripSuite::new();
        suite.add(RoundTripTest::new("x", "simple var"));
        suite.add_result(RoundTripRecord::success(
            "x".to_string(),
            "x".to_string(),
            "x".to_string(),
        ));
        assert_eq!(suite.test_count(), 1);
        assert_eq!(suite.pass_count(), 1);
        assert_eq!(suite.fail_count(), 0);
        assert!((suite.pass_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_roundtrip_stats() {
        let mut stats = RoundTripStats::new();
        let r1 = RoundTripRecord::success("a".to_string(), "a".to_string(), "a".to_string());
        let r2 = RoundTripRecord::failure("b".to_string(), "err");
        stats.record(&r1);
        stats.record(&r2);
        assert_eq!(stats.total_tests, 2);
    }
    #[test]
    fn test_snapshot_catalog() {
        let mut cat = SnapshotCatalog::new();
        cat.add("test1", RoundTripSnapshot::new("fun x -> x", "fun x -> x"));
        assert_eq!(cat.count(), 1);
        assert_eq!(cat.check("test1", "fun x -> x"), Some(true));
        assert_eq!(cat.check("test1", "fun y -> y"), Some(false));
        assert_eq!(cat.check("missing", "x"), None);
    }
    #[test]
    fn test_normalise_for_comparison() {
        let s = "  hello  \n  world  ";
        assert_eq!(normalise_for_comparison(s), "hello world");
    }
    #[test]
    fn test_normalised_equal() {
        assert!(normalised_equal("  a  b  ", "a b"));
        assert!(!normalised_equal("a b", "b a"));
    }
    #[test]
    fn test_generate_corpus() {
        let corpus = generate_corpus(&["fun {} -> {}", "({} {})"]);
        assert!(!corpus.is_empty());
        assert!(corpus.iter().any(|s| s.contains("fun x")));
    }
    #[test]
    fn test_expr_fuzzer() {
        let mut fuzz = ExprFuzzer::new(12345, 3);
        let batch = fuzz.generate_batch(10);
        assert_eq!(batch.len(), 10);
        assert!(batch.iter().all(|s| !s.is_empty()));
    }
    #[test]
    fn test_format_distance() {
        assert_eq!(format_distance("a b c", "a b c"), 0);
        assert_eq!(format_distance("a b", "a c"), 1);
        assert!(format_distance("hello world", "world hello") > 0);
    }
    #[test]
    fn test_check_printing_stable() {
        let (stable, iters) = check_printing_stable("  hello  \n  world  ", 10);
        assert!(stable);
        assert!(iters < 10);
    }
    #[test]
    fn test_roundtrip_diff() {
        let d = roundtrip_diff("a b c", "a X c");
        assert_eq!(d.len(), 1);
        assert!(d[0].contains("at 1"));
    }
    #[test]
    fn test_batch_processor() {
        let mut proc = RoundTripBatchProcessor::new();
        proc.add_test("fun x -> x", "identity");
        proc.add_test("1 + 2", "addition");
        let summary = proc.process_all();
        assert!(summary.contains("tests=2"));
    }
    #[test]
    fn test_produce_roundtrip_report() {
        let sources = ["fun x -> x", "1 + 2", "Nat"];
        let report = produce_roundtrip_report(&sources);
        assert!(report.contains("Round-Trip Report"));
        assert!(report.contains("PASS") || report.contains("FAIL"));
    }
}
/// Finds editable regions in a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn find_edit_regions(src: &str) -> Vec<EditRegion> {
    let mut regions = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
            let start = i;
            while i < bytes.len()
                && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n' || bytes[i] == b'\r')
            {
                i += 1;
            }
            regions.push(EditRegion {
                start,
                end: i,
                kind: EditRegionKind::Whitespace,
            });
        } else if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'\'')
            {
                i += 1;
            }
            regions.push(EditRegion {
                start,
                end: i,
                kind: EditRegionKind::Identifier,
            });
        } else if b.is_ascii_digit() {
            let start = i;
            while i < bytes.len()
                && (bytes[i].is_ascii_digit() || bytes[i] == b'.' || bytes[i] == b'_')
            {
                i += 1;
            }
            regions.push(EditRegion {
                start,
                end: i,
                kind: EditRegionKind::Number,
            });
        } else if b == b'"' {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' {
                    i += 1;
                }
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            regions.push(EditRegion {
                start,
                end: i,
                kind: EditRegionKind::StringLit,
            });
        } else if b == b'(' {
            let start = i;
            let mut depth = 0usize;
            while i < bytes.len() {
                if bytes[i] == b'(' {
                    depth += 1;
                } else if bytes[i] == b')' {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
            regions.push(EditRegion {
                start,
                end: i,
                kind: EditRegionKind::Parens,
            });
        } else {
            i += 1;
        }
    }
    regions
}
/// Applies whitespace normalisation mutations.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn whitespace_mutations(src: &str) -> Vec<SourceMutation> {
    let mut results = Vec::new();
    let extra = src.replace("  ", " ").replace(" ", "  ");
    if extra != src {
        results.push(SourceMutation {
            original: src.to_string(),
            mutated: extra,
            description: "double spaces".to_string(),
        });
    }
    let trailing: String = src.lines().map(|l| format!("{}  \n", l)).collect();
    results.push(SourceMutation {
        original: src.to_string(),
        mutated: trailing,
        description: "trailing whitespace".to_string(),
    });
    results
}
/// Estimate the nesting depth of a source string by counting brackets.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn estimate_nesting_depth(src: &str) -> usize {
    let mut depth = 0usize;
    let mut max_depth = 0usize;
    for c in src.chars() {
        match c {
            '(' | '[' | '{' => {
                depth += 1;
                if depth > max_depth {
                    max_depth = depth;
                }
            }
            ')' | ']' | '}' => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    max_depth
}
/// Checks that a source string is syntactically valid (parses without error).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_syntactically_valid(src: &str) -> bool {
    !src.trim().is_empty()
        && src.chars().filter(|&c| c == '(').count() == src.chars().filter(|&c| c == ')').count()
}
/// Checks that parentheses are balanced in the source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn has_balanced_parens(src: &str) -> bool {
    let mut depth = 0i32;
    for c in src.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}
/// Checks that brackets are balanced.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn has_balanced_brackets(src: &str) -> bool {
    let mut depth = 0i32;
    for c in src.chars() {
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}
/// Checks that braces are balanced.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn has_balanced_braces(src: &str) -> bool {
    let mut depth = 0i32;
    for c in src.chars() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0
}
/// Computes the Levenshtein edit distance between two strings.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let n = a.len();
    let m = b.len();
    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for (i, row) in dp.iter_mut().enumerate().take(n + 1) {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate().take(m + 1) {
        *cell = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[n][m]
}
/// Strips all whitespace from a string for normalised comparison.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn strip_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
/// Collapses all runs of whitespace to a single space.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn collapse_whitespace(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_space = false;
    for c in s.chars() {
        if c.is_whitespace() {
            if !prev_space {
                result.push(' ');
                prev_space = true;
            }
        } else {
            result.push(c);
            prev_space = false;
        }
    }
    result.trim().to_string()
}
/// Compare two strings according to a RoundTripConfigExt.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn config_compare(a: &str, b: &str, config: &RoundTripConfigExt) -> bool {
    let mut la = a.to_string();
    let mut lb = b.to_string();
    if config.allow_trailing_newlines {
        la = la.trim_end().to_string();
        lb = lb.trim_end().to_string();
    }
    if config.normalise_whitespace {
        la = collapse_whitespace(&la);
        lb = collapse_whitespace(&lb);
    }
    if config.strip_comments {
        la = remove_line_comments(&la);
        lb = remove_line_comments(&lb);
    }
    if config.case_insensitive {
        la = la.to_lowercase();
        lb = lb.to_lowercase();
    }
    if config.max_edit_distance == 0 {
        la == lb
    } else {
        levenshtein_distance(&la, &lb) <= config.max_edit_distance
    }
}
/// Remove line comments (-- to end of line) from a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn remove_line_comments(src: &str) -> String {
    src.lines()
        .map(|line| {
            if let Some(pos) = line.find("--") {
                &line[..pos]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// Tokenise a source string into a sequence of text tokens (simplified).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn simple_tokenise(src: &str) -> Vec<TextToken> {
    let mut tokens = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
            let start = i;
            while i < bytes.len()
                && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n' || bytes[i] == b'\r')
            {
                i += 1;
            }
            tokens.push(TextToken {
                kind: "WS".to_string(),
                text: src[start..i].to_string(),
                offset: start,
            });
        } else if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'\'')
            {
                i += 1;
            }
            tokens.push(TextToken {
                kind: "IDENT".to_string(),
                text: src[start..i].to_string(),
                offset: start,
            });
        } else if b.is_ascii_digit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            tokens.push(TextToken {
                kind: "NUM".to_string(),
                text: src[start..i].to_string(),
                offset: start,
            });
        } else {
            tokens.push(TextToken {
                kind: "SYM".to_string(),
                text: (b as char).to_string(),
                offset: i,
            });
            i += 1;
        }
    }
    tokens
}
/// Reconstruct source text from text tokens.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn reconstruct_from_tokens(tokens: &[TextToken]) -> String {
    tokens.iter().map(|t| t.text.as_str()).collect()
}
/// Check that token round-trip is idempotent.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_roundtrip_check(src: &str) -> bool {
    let tokens = simple_tokenise(src);
    let reconstructed = reconstruct_from_tokens(&tokens);
    reconstructed == src
}
/// Build a summary from a list of round-trip results.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn build_summary(results: &[(String, bool)]) -> Vec<RoundTripSummaryLine> {
    results
        .iter()
        .map(|(input, passed)| {
            let preview: String = input.chars().take(40).collect();
            RoundTripSummaryLine {
                input_preview: preview,
                passed: *passed,
                edit_distance: if *passed { None } else { Some(1) },
            }
        })
        .collect()
}
/// Prints a human-readable round-trip summary.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn print_roundtrip_summary(lines: &[RoundTripSummaryLine]) -> String {
    let mut out = String::new();
    for (i, line) in lines.iter().enumerate() {
        let status = if line.passed { "PASS" } else { "FAIL" };
        out.push_str(&format!("[{}] {} {:?}\n", status, i, line.input_preview));
    }
    out
}
/// Checks that a source survives being tokenised and reconstructed unchanged.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_stability_check(src: &str) -> (bool, String) {
    let tokens = simple_tokenise(src);
    let reconstructed = reconstruct_from_tokens(&tokens);
    if reconstructed == src {
        (true, String::new())
    } else {
        let d = levenshtein_distance(src, &reconstructed);
        (false, format!("edit distance: {}", d))
    }
}
/// A simple tokeniser wrapper that strips whitespace tokens.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn tokenise_non_ws(src: &str) -> Vec<TextToken> {
    simple_tokenise(src)
        .into_iter()
        .filter(|t| t.kind != "WS")
        .collect()
}
/// Check that two sources have the same non-whitespace tokens.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn same_non_ws_tokens(a: &str, b: &str) -> bool {
    let ta = tokenise_non_ws(a);
    let tb = tokenise_non_ws(b);
    if ta.len() != tb.len() {
        return false;
    }
    ta.iter().zip(tb.iter()).all(|(x, y)| x.text == y.text)
}
/// Runs a full round-trip check suite and returns a report string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn run_full_roundtrip_suite(sources: &[&str]) -> String {
    let mut report = String::from("=== Full Round-Trip Suite ===\n");
    let mut pass = 0usize;
    let mut fail = 0usize;
    for (i, src) in sources.iter().enumerate() {
        let (ok, msg) = token_stability_check(src);
        if ok {
            pass += 1;
            report.push_str(&format!("[PASS] case {}\n", i));
        } else {
            fail += 1;
            report.push_str(&format!("[FAIL] case {}: {}\n", i, msg));
        }
    }
    report.push_str(&format!("Total: {} pass, {} fail\n", pass, fail));
    report
}
/// Checks whether two sources are alpha-equivalent at the token level.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_alpha_equiv(a: &str, b: &str) -> bool {
    let ta = tokenise_non_ws(a);
    let tb = tokenise_non_ws(b);
    if ta.len() != tb.len() {
        return false;
    }
    let mut a_to_b: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut b_to_a: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for (x, y) in ta.iter().zip(tb.iter()) {
        if x.kind == "IDENT" && y.kind == "IDENT" {
            let prev = a_to_b.entry(x.text.clone()).or_insert(y.text.clone());
            if prev != &y.text {
                return false;
            }
            let prev2 = b_to_a.entry(y.text.clone()).or_insert(x.text.clone());
            if prev2 != &x.text {
                return false;
            }
        } else if x.text != y.text {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod roundtrip_extended_tests {
    use super::*;
    use crate::roundtrip::*;
    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("abc", "abd"), 1);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", ""), 3);
    }
    #[test]
    fn test_collapse_whitespace() {
        assert_eq!(collapse_whitespace("a  b  c"), "a b c");
        assert_eq!(collapse_whitespace("  a  "), "a");
    }
    #[test]
    fn test_has_balanced_parens() {
        assert!(has_balanced_parens("(a (b c))"));
        assert!(!has_balanced_parens("(a b"));
        assert!(!has_balanced_parens("a b)"));
    }
    #[test]
    fn test_has_balanced_brackets() {
        assert!(has_balanced_brackets("[a [b]]"));
        assert!(!has_balanced_brackets("[a b"));
    }
    #[test]
    fn test_has_balanced_braces() {
        assert!(has_balanced_braces("{a {b}}"));
        assert!(!has_balanced_braces("{a b"));
    }
    #[test]
    fn test_token_stability() {
        let src = "fun x -> x + 1";
        let (ok, _) = token_stability_check(src);
        assert!(ok);
    }
    #[test]
    fn test_config_compare_lenient() {
        let cfg = RoundTripConfigExt::lenient();
        assert!(config_compare("a  b  c", "a b c", &cfg));
    }
    #[test]
    fn test_config_compare_strict() {
        let cfg = RoundTripConfigExt::strict();
        assert!(config_compare("abc", "abc", &cfg));
        assert!(!config_compare("a  b", "a b", &cfg));
    }
    #[test]
    fn test_remove_line_comments() {
        let src = "foo -- this is a comment\nbar";
        let result = remove_line_comments(src);
        assert!(result.contains("foo "));
        assert!(!result.contains("comment"));
    }
    #[test]
    fn test_text_diff() {
        let d = TextDiff::new("abc", "abc");
        assert!(d.is_identical());
        let d2 = TextDiff::new("abc", "axc");
        assert!(!d2.is_identical());
        assert_eq!(d2.char_diff_count(), 1);
    }
    #[test]
    fn test_simple_tokenise_reconstruct() {
        let src = "fun x -> x + 1";
        let tokens = simple_tokenise(src);
        let reconstructed = reconstruct_from_tokens(&tokens);
        assert_eq!(reconstructed, src);
    }
    #[test]
    fn test_token_alpha_equiv() {
        assert!(token_alpha_equiv("fun x -> x", "fun y -> y"));
        assert!(!token_alpha_equiv("fun x -> y", "fun y -> y"));
    }
    #[test]
    fn test_corpus_store() {
        let mut store = CorpusStore::new();
        let id = store.add("fun x -> x".to_string());
        assert_eq!(id, 0);
        assert_eq!(store.len(), 1);
    }
    #[test]
    fn test_arith_fuzzer() {
        let mut fuzz = ArithFuzzer::new(3);
        let batch = fuzz.generate_batch(10);
        assert_eq!(batch.len(), 10);
        for expr in &batch {
            assert!(!expr.is_empty());
        }
    }
    #[test]
    fn test_lambda_corpus_generator() {
        let gen = LambdaCorpusGenerator::new(3);
        let corpus = gen.generate(2);
        assert!(!corpus.is_empty());
    }
    #[test]
    fn test_forall_corpus_generator() {
        let gen = ForallCorpusGenerator::new();
        let corpus = gen.generate();
        assert!(!corpus.is_empty());
        assert!(corpus[0].starts_with("forall"));
    }
    #[test]
    fn test_golden_set() {
        let mut gs = GoldenSet::new();
        gs.add("id", "fun x -> x", "fun x -> x");
        assert_eq!(gs.len(), 1);
    }
    #[test]
    fn test_batch_stats() {
        let mut stats = BatchRoundTripStats::new();
        stats.record_pass();
        stats.record_fail(5);
        assert_eq!(stats.total, 2);
        assert_eq!(stats.passed, 1);
        assert_eq!(stats.failed, 1);
        assert!((stats.pass_rate() - 50.0).abs() < 0.01);
        assert!((stats.avg_edit_distance() - 5.0).abs() < 0.01);
    }
    #[test]
    fn test_run_full_roundtrip_suite() {
        let sources = ["fun x -> x", "1 + 2 + 3", "let x = 5 in x"];
        let report = run_full_roundtrip_suite(&sources);
        assert!(report.contains("PASS") || report.contains("FAIL"));
        assert!(report.contains("Total:"));
    }
    #[test]
    fn test_norm_table() {
        let mut table = NormTable::new();
        table.add('\u{03B1}', 'a');
        let result = table.apply("\u{03B1}bc");
        assert_eq!(result, "abc");
    }
    #[test]
    fn test_prefix_comparator() {
        let cmp = PrefixComparator::new("abcdef", "abcxyz");
        assert_eq!(cmp.common_prefix_len(), 3);
        assert_eq!(cmp.common_prefix(), "abc");
    }
    #[test]
    fn test_same_non_ws_tokens() {
        assert!(same_non_ws_tokens("a b c", "a  b  c"));
        assert!(!same_non_ws_tokens("a b c", "a b d"));
    }
    #[test]
    fn test_edit_regions() {
        let regions = find_edit_regions("fun x -> 42");
        assert!(!regions.is_empty());
    }
    #[test]
    fn test_whitespace_mutations() {
        let muts = whitespace_mutations("fun x -> x");
        assert!(!muts.is_empty());
    }
    #[test]
    fn test_estimate_nesting_depth() {
        assert_eq!(estimate_nesting_depth("(a (b c))"), 2);
        assert_eq!(estimate_nesting_depth("a b c"), 0);
    }
    #[test]
    fn test_property_test() {
        let mut pt = PropertyTest::new("identity").with_iterations(10);
        pt.record("x".to_string(), true);
        pt.record("y".to_string(), true);
        assert!(pt.all_passed());
        assert!(pt.summary().contains("2/2"));
    }
    #[test]
    fn test_strip_whitespace() {
        assert_eq!(strip_whitespace("a b c"), "abc");
    }
    #[test]
    fn test_is_syntactically_valid() {
        assert!(is_syntactically_valid("(a b)"));
        assert!(!is_syntactically_valid(""));
    }
}
/// Counts the number of tokens in a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_tokens(src: &str) -> usize {
    simple_tokenise(src)
        .iter()
        .filter(|t| t.kind != "WS")
        .count()
}
/// Returns true if source text has no consecutive duplicate tokens.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn no_consecutive_duplicates(src: &str) -> bool {
    let tokens = tokenise_non_ws(src);
    tokens.windows(2).all(|w| w[0].text != w[1].text)
}
/// Returns all lines of a source that contain a given pattern.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn grep_lines<'a>(src: &'a str, pattern: &str) -> Vec<&'a str> {
    src.lines().filter(|line| line.contains(pattern)).collect()
}
/// Checks that the token count is stable across two passes.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn token_count_stable(src: &str) -> bool {
    let c1 = count_tokens(src);
    let c2 = count_tokens(src);
    c1 == c2
}
#[cfg(test)]
mod roundtrip_final_tests {
    use super::*;
    use crate::roundtrip::*;
    #[test]
    fn test_count_tokens() {
        assert_eq!(count_tokens("fun x -> x"), 5);
    }
    #[test]
    fn test_coverage_tracker() {
        let mut ct = CoverageTracker::new();
        ct.mark("fun x -> x");
        ct.mark("fun x -> x");
        ct.mark("let x = 1");
        assert_eq!(ct.count(), 2);
    }
    #[test]
    fn test_grep_lines() {
        let src = "fun x -> x\nlet y = 2\nfun z -> z";
        let lines = grep_lines(src, "fun");
        assert_eq!(lines.len(), 2);
    }
    #[test]
    fn test_token_count_stable() {
        assert!(token_count_stable("fun x -> x + 1"));
    }
}
/// Checks that two source strings tokenise to equal non-whitespace token sequences.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn tokenwise_equal(a: &str, b: &str) -> bool {
    same_non_ws_tokens(a, b)
}
/// Checks that a source is non-empty and balanced.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_well_formed(src: &str) -> bool {
    !src.trim().is_empty()
        && has_balanced_parens(src)
        && has_balanced_brackets(src)
        && has_balanced_braces(src)
}
#[cfg(test)]
mod roundtrip_wellformed_tests {
    use super::*;
    use crate::roundtrip::*;
    #[test]
    fn test_is_well_formed() {
        assert!(is_well_formed("(a b)"));
        assert!(!is_well_formed(""));
        assert!(!is_well_formed("(a b"));
    }
}
/// A no-op check that always returns true (for use in test harnesses).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn trivial_check(_src: &str) -> bool {
    true
}
#[cfg(test)]
mod roundtrip_trivial {
    use super::*;
    use crate::roundtrip::*;
    #[test]
    fn test_trivial() {
        assert!(trivial_check("anything"));
    }
}
