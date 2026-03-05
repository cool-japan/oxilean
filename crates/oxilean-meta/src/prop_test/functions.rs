//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CoverageTracker, ExprStats, Histogram, LabelledFail, PropCheckResult, PropResult, PropStats,
    PropTestAnalysisPass, PropTestConfig, PropTestConfigValue, PropTestDiagnostics, PropTestDiff,
    PropTestExtConfig700, PropTestExtConfigVal700, PropTestExtDiag700, PropTestExtDiff700,
    PropTestExtPass700, PropTestExtPipeline700, PropTestExtResult700, PropTestPipeline,
    PropTestResult, PropTestSuiteExt, PropertyBatch, RegressionCase, RegressionSuite,
    RegressionTestExt, Rng,
};
use oxilean_kernel::{BinderInfo, Expr, Level, Literal, Name};

/// Trait for types that can be randomly generated
pub trait Arbitrary {
    fn arbitrary(rng: &mut Rng, size: usize) -> Self;
}
impl Arbitrary for bool {
    fn arbitrary(rng: &mut Rng, _: usize) -> Self {
        rng.next_bool()
    }
}
impl Arbitrary for u64 {
    fn arbitrary(rng: &mut Rng, _: usize) -> Self {
        rng.next_u64() % 100
    }
}
impl Arbitrary for String {
    fn arbitrary(rng: &mut Rng, size: usize) -> Self {
        let len = rng.next_usize(size.min(8) + 1);
        let chars = b"abcdefghijklmnopqrstuvwxyz";
        (0..len)
            .map(|_| chars[rng.next_usize(chars.len())] as char)
            .collect()
    }
}
/// Generate a random small Expr for testing
pub fn arbitrary_expr(rng: &mut Rng, depth: usize) -> Expr {
    if depth == 0 {
        match rng.next_u32(4) {
            0 => Expr::BVar(rng.next_u32(3)),
            1 => Expr::Const(Name::str(format!("x{}", rng.next_u32(5))), vec![]),
            2 => Expr::Lit(Literal::Nat(rng.next_u64() % 10)),
            _ => Expr::Sort(Level::Zero),
        }
    } else {
        match rng.next_u32(6) {
            0 => Expr::BVar(rng.next_u32(3)),
            1 => Expr::Const(Name::str(format!("f{}", rng.next_u32(3))), vec![]),
            2 => Expr::App(
                Box::new(arbitrary_expr(rng, depth - 1)),
                Box::new(arbitrary_expr(rng, depth - 1)),
            ),
            3 => Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::Sort(Level::Zero)),
                Box::new(arbitrary_expr(rng, depth - 1)),
            ),
            4 => Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::Sort(Level::Zero)),
                Box::new(arbitrary_expr(rng, depth - 1)),
            ),
            _ => Expr::Lit(Literal::Nat(rng.next_u64() % 10)),
        }
    }
}
/// Run a property test
pub fn check_property<F: Fn(&mut Rng) -> Option<bool>>(
    trials: u32,
    seed: u64,
    description: &str,
    prop: F,
) -> PropResult {
    let mut rng = Rng::new(seed);
    let mut passes = 0u32;
    let mut vacuous = true;
    for trial in 0..trials {
        match prop(&mut rng) {
            None => continue,
            Some(true) => {
                passes += 1;
                vacuous = false;
            }
            Some(false) => {
                return PropResult::Fail {
                    trial,
                    counterexample: format!("property '{}' failed at trial {}", description, trial),
                };
            }
        }
    }
    if vacuous {
        PropResult::Vacuous
    } else {
        PropResult::Pass { trials: passes }
    }
}
/// Common expression properties to test
pub mod properties {
    use super::*;
    /// Count AST nodes in an expression
    pub fn node_count(e: &Expr) -> usize {
        match e {
            Expr::App(f, a) => 1 + node_count(f) + node_count(a),
            Expr::Lam(_, _, ty, b) => 1 + node_count(ty) + node_count(b),
            Expr::Pi(_, _, ty, b) => 1 + node_count(ty) + node_count(b),
            Expr::Let(_, ty, v, b) => 1 + node_count(ty) + node_count(v) + node_count(b),
            _ => 1,
        }
    }
    /// Property: node_count >= 1 for all expressions
    pub fn prop_node_count_positive(rng: &mut Rng) -> Option<bool> {
        let e = arbitrary_expr(rng, 3);
        Some(node_count(&e) >= 1)
    }
    /// Property: a BVar(i) has exactly 1 node
    pub fn prop_bvar_single_node(rng: &mut Rng) -> Option<bool> {
        let i = rng.next_u32(5);
        let e = Expr::BVar(i);
        Some(node_count(&e) == 1)
    }
    /// Property: App(f, a) has nodes = 1 + nodes(f) + nodes(a)
    pub fn prop_app_node_count(rng: &mut Rng) -> Option<bool> {
        let f = arbitrary_expr(rng, 2);
        let a = arbitrary_expr(rng, 2);
        let nf = node_count(&f);
        let na = node_count(&a);
        let app = Expr::App(Box::new(f), Box::new(a));
        Some(node_count(&app) == 1 + nf + na)
    }
}
#[cfg(test)]
mod tests {
    use super::properties::{node_count, prop_app_node_count, prop_node_count_positive};
    use super::*;
    use crate::prop_test::*;
    #[test]
    fn test_rng_basic() {
        let mut rng = Rng::new(42);
        let v1 = rng.next_u64();
        let v2 = rng.next_u64();
        assert_ne!(v1, v2);
        let mut rng2 = Rng::new(42);
        assert_eq!(rng2.next_u64(), v1);
        assert_eq!(rng2.next_u64(), v2);
    }
    #[test]
    fn test_rng_next_usize_bound() {
        let mut rng = Rng::new(12345);
        for max in [1usize, 2, 5, 10, 100] {
            for _ in 0..50 {
                let v = rng.next_usize(max);
                assert!(v < max, "v={} is not < max={}", v, max);
            }
        }
    }
    #[test]
    fn test_arbitrary_bool() {
        let mut rng = Rng::new(999);
        let mut trues = 0u32;
        let mut falses = 0u32;
        for _ in 0..100 {
            if bool::arbitrary(&mut rng, 0) {
                trues += 1;
            } else {
                falses += 1;
            }
        }
        assert!(trues > 0, "never got true");
        assert!(falses > 0, "never got false");
    }
    #[test]
    fn test_arbitrary_string() {
        let mut rng = Rng::new(7777);
        for _ in 0..20 {
            let s = String::arbitrary(&mut rng, 8);
            for c in s.chars() {
                assert!(c.is_ascii_lowercase(), "unexpected char: {}", c);
            }
            assert!(s.len() <= 9, "string too long: {}", s.len());
        }
    }
    #[test]
    fn test_arbitrary_expr() {
        let mut rng = Rng::new(314159);
        for _ in 0..20 {
            let e = arbitrary_expr(&mut rng, 0);
            let is_base = matches!(
                e,
                Expr::BVar(_) | Expr::Const(..) | Expr::Lit(_) | Expr::Sort(_)
            );
            assert!(is_base, "depth-0 expr should be base case: {:?}", e);
        }
        let mut saw_app = false;
        for _ in 0..200 {
            let e = arbitrary_expr(&mut rng, 2);
            if matches!(e, Expr::App(..)) {
                saw_app = true;
                break;
            }
        }
        assert!(saw_app, "expected at least one App at depth 2");
    }
    #[test]
    fn test_check_property_pass() {
        let result = check_property(200, 42, "node_count_positive", prop_node_count_positive);
        assert!(result.is_pass(), "expected Pass, got: {:?}", result);
    }
    #[test]
    fn test_check_property_fail() {
        let result = check_property(10, 0, "always_false", |_rng| Some(false));
        assert!(result.is_fail(), "expected Fail for always-false property");
        if let PropResult::Fail { trial, .. } = result {
            assert_eq!(trial, 0, "should fail on first trial");
        }
    }
    #[test]
    fn test_prop_app_node_count() {
        let result = check_property(100, 271828, "app_node_count", prop_app_node_count);
        assert!(result.is_pass(), "expected Pass, got: {:?}", result);
    }
    #[test]
    fn test_prop_bvar_single_node() {
        let mut rng = Rng::new(55555);
        for _ in 0..50 {
            let result = properties::prop_bvar_single_node(&mut rng);
            assert_eq!(result, Some(true));
        }
    }
    #[test]
    fn test_node_count_lam_pi() {
        let ty = Expr::Sort(Level::Zero);
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty.clone()),
            Box::new(body.clone()),
        );
        assert_eq!(node_count(&lam), 3);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(ty),
            Box::new(body),
        );
        assert_eq!(node_count(&pi), 3);
    }
}
impl Arbitrary for u32 {
    fn arbitrary(rng: &mut Rng, _size: usize) -> Self {
        rng.next_u32(1000)
    }
}
impl Arbitrary for i64 {
    fn arbitrary(rng: &mut Rng, _size: usize) -> Self {
        let raw = rng.next_u64() % 200;
        raw as i64 - 100
    }
}
impl Arbitrary for usize {
    fn arbitrary(rng: &mut Rng, size: usize) -> Self {
        rng.next_usize(size.max(1))
    }
}
impl<T: Arbitrary> Arbitrary for Vec<T> {
    fn arbitrary(rng: &mut Rng, size: usize) -> Self {
        let len = rng.next_usize(size.min(8) + 1);
        (0..len).map(|_| T::arbitrary(rng, size / 2 + 1)).collect()
    }
}
impl<T: Arbitrary> Arbitrary for Option<T> {
    fn arbitrary(rng: &mut Rng, size: usize) -> Self {
        if rng.next_bool() {
            Some(T::arbitrary(rng, size))
        } else {
            None
        }
    }
}
/// Trait for types that can produce smaller counterexamples.
pub trait Shrink: Sized {
    /// Return a list of candidates strictly smaller than `self`.
    fn shrink(&self) -> Vec<Self>;
}
impl Shrink for u64 {
    fn shrink(&self) -> Vec<Self> {
        if *self == 0 {
            return vec![];
        }
        vec![0, *self / 2]
    }
}
impl Shrink for i64 {
    fn shrink(&self) -> Vec<Self> {
        if *self == 0 {
            return vec![];
        }
        let mut out = vec![0i64];
        if *self > 0 {
            out.push(*self - 1);
            out.push(*self / 2);
        } else {
            out.push(*self + 1);
            out.push(*self / 2);
        }
        out
    }
}
impl Shrink for bool {
    fn shrink(&self) -> Vec<Self> {
        if *self {
            vec![false]
        } else {
            vec![]
        }
    }
}
impl Shrink for String {
    fn shrink(&self) -> Vec<Self> {
        if self.is_empty() {
            return vec![];
        }
        let mut out = Vec::new();
        for i in 0..self.char_indices().count() {
            let s: String = self
                .chars()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, c)| c)
                .collect();
            out.push(s);
        }
        out
    }
}
impl<T: Shrink + Clone> Shrink for Vec<T> {
    fn shrink(&self) -> Vec<Self> {
        if self.is_empty() {
            return vec![];
        }
        let mut out = Vec::new();
        for i in 0..self.len() {
            let mut v = self.clone();
            v.remove(i);
            out.push(v);
        }
        for (i, elem) in self.iter().enumerate() {
            for smaller in elem.shrink() {
                let mut v = self.clone();
                v[i] = smaller;
                out.push(v);
            }
        }
        out
    }
}
/// Find the smallest counterexample by repeated shrinking.
pub fn shrink_counterexample<T, F>(mut x: T, prop: F) -> T
where
    T: Shrink,
    F: Fn(&T) -> bool,
{
    loop {
        let candidates = x.shrink();
        let mut found = false;
        for candidate in candidates {
            if !prop(&candidate) {
                x = candidate;
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
    }
    x
}
/// Run a property test and return detailed failure information.
pub fn check_property_labelled<F>(
    trials: u32,
    seed: u64,
    prop_name: &str,
    prop: F,
) -> Result<u32, LabelledFail>
where
    F: Fn(&mut Rng, u32) -> Option<(bool, String)>,
{
    let mut rng = Rng::new(seed);
    let mut passes = 0u32;
    for trial in 0..trials {
        match prop(&mut rng, trial) {
            None => continue,
            Some((true, _)) => passes += 1,
            Some((false, description)) => {
                return Err(LabelledFail {
                    trial,
                    label: prop_name.to_string(),
                    description,
                });
            }
        }
    }
    Ok(passes)
}
/// Classify generated expressions by top-level constructor.
pub fn expr_distribution(rng: &mut Rng, n: usize, depth: usize) -> Histogram {
    let mut hist = Histogram::new();
    for _ in 0..n {
        let e = arbitrary_expr(rng, depth);
        let label = match &e {
            Expr::BVar(_) => "BVar",
            Expr::FVar(_) => "FVar",
            Expr::Sort(_) => "Sort",
            Expr::Const(_, _) => "Const",
            Expr::App(_, _) => "App",
            Expr::Lam(_, _, _, _) => "Lam",
            Expr::Pi(_, _, _, _) => "Pi",
            Expr::Let(_, _, _, _) => "Let",
            Expr::Lit(_) => "Lit",
            Expr::Proj(_, _, _) => "Proj",
        };
        hist.record(label);
    }
    hist
}
/// Compute the depth of an expression tree.
pub fn expr_depth(e: &Expr) -> usize {
    match e {
        Expr::App(f, a) => 1 + expr_depth(f).max(expr_depth(a)),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        _ => 1,
    }
}
/// Generate an arbitrary `Level`.
pub fn arbitrary_level(rng: &mut Rng, depth: usize) -> Level {
    if depth == 0 {
        return Level::Zero;
    }
    match rng.next_u32(4) {
        0 => Level::Zero,
        1 => Level::Succ(Box::new(arbitrary_level(rng, depth - 1))),
        2 => Level::Max(
            Box::new(arbitrary_level(rng, depth - 1)),
            Box::new(arbitrary_level(rng, depth - 1)),
        ),
        _ => Level::Param(Name::str(format!("u{}", rng.next_u32(3)))),
    }
}
/// Generate an arbitrary closed expression (wrapped in lambda layers).
pub fn arbitrary_closed_expr(rng: &mut Rng, depth: usize) -> Expr {
    let inner = arbitrary_expr(rng, depth);
    let ty = Expr::Sort(Level::Zero);
    let b2 = Expr::Lam(
        BinderInfo::Default,
        Name::str("z"),
        Box::new(ty.clone()),
        Box::new(inner),
    );
    let b1 = Expr::Lam(
        BinderInfo::Default,
        Name::str("y"),
        Box::new(ty.clone()),
        Box::new(b2),
    );
    Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(ty),
        Box::new(b1),
    )
}
/// Generate an application chain `f a0 a1 ... a_{n-1}`.
pub fn arbitrary_app_chain(rng: &mut Rng, depth: usize) -> Expr {
    let head = Expr::Const(Name::str(format!("f{}", rng.next_u32(3))), vec![]);
    (0..depth).fold(head, |acc, i| {
        Expr::App(
            Box::new(acc),
            Box::new(Expr::Const(Name::str(format!("a{}", i)), vec![])),
        )
    })
}
/// Generate an arbitrary Pi type of the given depth.
pub fn arbitrary_pi_type(rng: &mut Rng, depth: usize) -> Expr {
    if depth == 0 {
        return Expr::Sort(Level::Zero);
    }
    Expr::Pi(
        BinderInfo::Default,
        Name::str(format!("a{}", rng.next_u32(4))),
        Box::new(arbitrary_pi_type(rng, depth - 1)),
        Box::new(arbitrary_pi_type(rng, depth - 1)),
    )
}
pub(super) fn count_expr_constructors_stats(e: &Expr, stats: &mut ExprStats) {
    match e {
        Expr::BVar(_) => stats.bvar_count += 1,
        Expr::Const(_, _) => stats.const_count += 1,
        Expr::App(f, a) => {
            stats.app_count += 1;
            count_expr_constructors_stats(f, stats);
            count_expr_constructors_stats(a, stats);
        }
        Expr::Lam(_, _, ty, body) => {
            stats.lam_count += 1;
            count_expr_constructors_stats(ty, stats);
            count_expr_constructors_stats(body, stats);
        }
        Expr::Pi(_, _, ty, body) => {
            stats.pi_count += 1;
            count_expr_constructors_stats(ty, stats);
            count_expr_constructors_stats(body, stats);
        }
        Expr::Let(_, ty, val, body) => {
            count_expr_constructors_stats(ty, stats);
            count_expr_constructors_stats(val, stats);
            count_expr_constructors_stats(body, stats);
        }
        Expr::Lit(_) => stats.lit_count += 1,
        _ => {}
    }
}
/// Additional expression property tests.
pub mod more_properties {
    use super::properties;
    use super::*;
    /// Property: depth of App(f, a) = 1 + max(depth(f), depth(a)).
    pub fn prop_app_depth(rng: &mut Rng) -> Option<bool> {
        let f = arbitrary_expr(rng, 2);
        let a = arbitrary_expr(rng, 2);
        let df = expr_depth(&f);
        let da = expr_depth(&a);
        let app = Expr::App(Box::new(f), Box::new(a));
        Some(expr_depth(&app) == 1 + df.max(da))
    }
    /// Property: arbitrary_closed_expr produces a Lam.
    pub fn prop_closed_is_lam(rng: &mut Rng) -> Option<bool> {
        let e = arbitrary_closed_expr(rng, 2);
        Some(matches!(e, Expr::Lam(_, _, _, _)))
    }
    /// Property: app chain of depth n has exactly n nested Apps.
    pub fn prop_app_chain_arity(rng: &mut Rng) -> Option<bool> {
        let n = rng.next_usize(5) + 1;
        let chain = arbitrary_app_chain(rng, n);
        let mut count = 0usize;
        let mut cur = &chain;
        while let Expr::App(f, _) = cur {
            count += 1;
            cur = f;
        }
        Some(count == n)
    }
    /// Property: Pi node count = 1 + count(ty) + count(body).
    pub fn prop_pi_node_count(rng: &mut Rng) -> Option<bool> {
        let ty = arbitrary_expr(rng, 2);
        let body = arbitrary_expr(rng, 2);
        let nt = properties::node_count(&ty);
        let nb = properties::node_count(&body);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        Some(properties::node_count(&pi) == 1 + nt + nb)
    }
    /// Property: Lam node count = 1 + count(ty) + count(body).
    pub fn prop_lam_node_count(rng: &mut Rng) -> Option<bool> {
        let ty = arbitrary_expr(rng, 2);
        let body = arbitrary_expr(rng, 2);
        let nt = properties::node_count(&ty);
        let nb = properties::node_count(&body);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        Some(properties::node_count(&lam) == 1 + nt + nb)
    }
    /// Property: expr_depth >= 1 for all expressions.
    pub fn prop_depth_positive(rng: &mut Rng) -> Option<bool> {
        let e = arbitrary_expr(rng, 3);
        Some(expr_depth(&e) >= 1)
    }
    /// Property: arbitrary_level(_, 0) is always Zero.
    pub fn prop_level_depth0_is_zero(rng: &mut Rng) -> Option<bool> {
        let l = arbitrary_level(rng, 0);
        Some(matches!(l, Level::Zero))
    }
}
/// Run a property test gathering detailed statistics.
pub fn check_property_with_stats<F>(trials: u32, seed: u64, prop: F) -> (PropResult, PropStats)
where
    F: Fn(&mut Rng) -> Option<bool>,
{
    let mut rng = Rng::new(seed);
    let mut stats = PropStats::new();
    let mut vacuous_count = 0u32;
    for trial in 0..trials {
        stats.total += 1;
        match prop(&mut rng) {
            None => {
                vacuous_count += 1;
                stats.vacuous += 1;
            }
            Some(true) => stats.passed += 1,
            Some(false) => {
                stats.failed += 1;
                return (
                    PropResult::Fail {
                        trial,
                        counterexample: format!("failed at trial {}", trial),
                    },
                    stats,
                );
            }
        }
    }
    let result = if vacuous_count == trials {
        PropResult::Vacuous
    } else {
        PropResult::Pass {
            trials: stats.passed,
        }
    };
    (result, stats)
}
#[cfg(test)]
mod extended_tests {
    use super::more_properties::*;
    use super::*;
    use crate::prop_test::*;
    #[test]
    fn test_shrink_u64() {
        let v: u64 = 16;
        let shrunk = v.shrink();
        assert!(!shrunk.is_empty());
        assert!(shrunk.iter().all(|&x| x < v));
    }
    #[test]
    fn test_shrink_i64_positive() {
        let v: i64 = 10;
        let shrunk = v.shrink();
        assert!(!shrunk.is_empty());
        assert!(shrunk.iter().all(|&x| x < v));
    }
    #[test]
    fn test_shrink_bool_true() {
        assert_eq!(true.shrink(), vec![false]);
        assert!(false.shrink().is_empty());
    }
    #[test]
    fn test_shrink_string() {
        let s = "abc".to_string();
        let shrunk = s.shrink();
        assert!(!shrunk.is_empty());
        for candidate in &shrunk {
            assert!(candidate.len() < s.len());
        }
    }
    #[test]
    fn test_shrink_vec() {
        let v: Vec<u64> = vec![1, 2, 3];
        let shrunk = v.shrink();
        assert!(!shrunk.is_empty());
        for sv in &shrunk {
            assert!(sv.len() < v.len() || sv.iter().zip(v.iter()).any(|(a, b)| a < b));
        }
    }
    #[test]
    fn test_shrink_counterexample_finds_minimum() {
        let minimal = shrink_counterexample(10u64, |&x| x < 5);
        assert!(minimal >= 5);
    }
    #[test]
    fn test_expr_stats() {
        let mut rng = Rng::new(123456);
        let stats = ExprStats::from_samples(&mut rng, 50, 3);
        assert_eq!(stats.count, 50);
        assert!(stats.min_nodes >= 1);
        assert!(stats.max_nodes >= stats.min_nodes);
        assert!(stats.avg_nodes() >= 1.0);
        assert!(stats.avg_depth() >= 1.0);
    }
    #[test]
    fn test_histogram() {
        let mut hist = Histogram::new();
        hist.record("a");
        hist.record("b");
        hist.record("a");
        assert_eq!(hist.total(), 3);
        assert!((hist.fraction("a") - 2.0 / 3.0).abs() < 1e-9);
        assert!((hist.fraction("b") - 1.0 / 3.0).abs() < 1e-9);
        assert_eq!(hist.fraction("c"), 0.0);
    }
    #[test]
    fn test_expr_distribution() {
        let mut rng = Rng::new(777);
        let hist = expr_distribution(&mut rng, 200, 2);
        assert_eq!(hist.total(), 200);
        assert!(*hist.buckets.get("App").unwrap_or(&0) > 0);
    }
    #[test]
    fn test_property_batch() {
        let mut batch = PropertyBatch::new();
        batch.add("prop1", 1, 50);
        batch.add("prop2", 2, 50);
        let (passes, failures) = batch.run_all(|_rng| Some(true));
        assert_eq!(passes, 2);
        assert_eq!(failures, 0);
    }
    #[test]
    fn test_check_property_labelled_pass() {
        let result = check_property_labelled(100, 42, "always_true", |_rng, _trial| {
            Some((true, String::new()))
        });
        assert!(result.is_ok());
        assert_eq!(result.expect("result should be valid"), 100);
    }
    #[test]
    fn test_check_property_labelled_fail() {
        let result = check_property_labelled(100, 42, "always_false", |_rng, _trial| {
            Some((false, "ce".to_string()))
        });
        assert!(result.is_err());
        let fail = result.unwrap_err();
        assert_eq!(fail.label, "always_false");
        assert_eq!(fail.trial, 0);
    }
    #[test]
    fn test_prop_app_depth() {
        let result = check_property(100, 11111, "app_depth", prop_app_depth);
        assert!(result.is_pass());
    }
    #[test]
    fn test_prop_closed_is_lam() {
        let result = check_property(50, 22222, "closed_is_lam", prop_closed_is_lam);
        assert!(result.is_pass());
    }
    #[test]
    fn test_prop_app_chain_arity() {
        let result = check_property(100, 33333, "app_chain_arity", prop_app_chain_arity);
        assert!(result.is_pass());
    }
    #[test]
    fn test_prop_pi_node_count() {
        let result = check_property(100, 44444, "pi_node_count", prop_pi_node_count);
        assert!(result.is_pass());
    }
    #[test]
    fn test_prop_lam_node_count() {
        let result = check_property(100, 55555, "lam_node_count", prop_lam_node_count);
        assert!(result.is_pass());
    }
    #[test]
    fn test_prop_depth_positive() {
        let result = check_property(100, 66666, "depth_positive", prop_depth_positive);
        assert!(result.is_pass());
    }
    #[test]
    fn test_prop_level_depth0_is_zero() {
        let result = check_property(50, 77777, "level_zero", prop_level_depth0_is_zero);
        assert!(result.is_pass());
    }
    #[test]
    fn test_arbitrary_pi_type() {
        let mut rng = Rng::new(5555);
        for _ in 0..20 {
            let e = arbitrary_pi_type(&mut rng, 2);
            match &e {
                Expr::Pi(_, _, _, _) | Expr::Sort(_) => {}
                other => panic!("unexpected: {:?}", other),
            }
        }
    }
    #[test]
    fn test_arbitrary_level() {
        let mut rng = Rng::new(6666);
        for _ in 0..20 {
            let l = arbitrary_level(&mut rng, 0);
            assert!(matches!(l, Level::Zero));
        }
        let mut saw_nonzero = false;
        for _ in 0..100 {
            let l = arbitrary_level(&mut rng, 2);
            if !matches!(l, Level::Zero) {
                saw_nonzero = true;
                break;
            }
        }
        assert!(saw_nonzero, "expected non-zero Level at depth 2");
    }
    #[test]
    fn test_rng_next_f64_range() {
        let mut rng = Rng::new(98765);
        for _ in 0..100 {
            let v = rng.next_f64();
            assert!((0.0..=1.0).contains(&v));
        }
    }
    #[test]
    fn test_arbitrary_vec_u64() {
        let mut rng = Rng::new(55555);
        let v: Vec<u64> = Vec::arbitrary(&mut rng, 5);
        assert!(v.len() <= 6);
    }
    #[test]
    fn test_arbitrary_option_some_or_none() {
        let mut rng = Rng::new(77777);
        let mut saw_some = false;
        let mut saw_none = false;
        for _ in 0..100 {
            let o: Option<u64> = Option::arbitrary(&mut rng, 1);
            match o {
                Some(_) => saw_some = true,
                None => saw_none = true,
            }
        }
        assert!(saw_some);
        assert!(saw_none);
    }
    #[test]
    fn test_coverage_tracker() {
        let mut cov = CoverageTracker::new();
        cov.hit("branch_a");
        cov.hit("branch_a");
        cov.hit("branch_b");
        assert_eq!(cov.count("branch_a"), 2);
        assert_eq!(cov.count("branch_b"), 1);
        assert_eq!(cov.count("branch_c"), 0);
        assert!(cov.covers_all(&["branch_a", "branch_b"]));
        assert!(!cov.covers_all(&["branch_a", "branch_b", "branch_c"]));
        let missing = cov.missing(&["branch_a", "branch_c"]);
        assert_eq!(missing, vec!["branch_c"]);
    }
    #[test]
    fn test_regression_suite() {
        let mut suite = RegressionSuite::new();
        suite.add_case(RegressionCase::new("issue_42", 42, 0));
        suite.add_case(RegressionCase::new("issue_99", 99, 0));
        assert_eq!(suite.count_passing(|_rng| Some(true)), 2);
        assert_eq!(suite.count_passing(|_rng| Some(false)), 0);
    }
    #[test]
    fn test_prop_stats_pass_rate() {
        let mut s = PropStats::new();
        s.total = 10;
        s.passed = 8;
        s.failed = 2;
        assert!((s.pass_rate() - 0.8).abs() < 1e-9);
        assert!((s.fail_rate() - 0.2).abs() < 1e-9);
        assert!(!s.is_passing());
    }
    #[test]
    fn test_prop_stats_all_pass() {
        let mut s = PropStats::new();
        s.total = 100;
        s.passed = 100;
        assert!(s.is_passing());
    }
    #[test]
    fn test_check_property_with_stats_pass() {
        let (result, stats) = check_property_with_stats(50, 42, |_rng| Some(true));
        assert!(result.is_pass());
        assert_eq!(stats.passed, 50);
        assert_eq!(stats.failed, 0);
    }
    #[test]
    fn test_check_property_with_stats_fail() {
        let (result, stats) = check_property_with_stats(50, 42, |_rng| Some(false));
        assert!(result.is_fail());
        assert_eq!(stats.failed, 1);
    }
}
/// Generate a random ASCII identifier of given max length.
#[allow(dead_code)]
pub fn gen_ident_ext(rng: &mut Rng, max_len: usize) -> String {
    let chars = b"abcdefghijklmnopqrstuvwxyz";
    let len = 1 + rng.next_usize(max_len.max(1));
    (0..len)
        .map(|_| chars[rng.next_usize(chars.len())] as char)
        .collect()
}
/// Generate a random boolean.
#[allow(dead_code)]
pub fn gen_bool_ext(rng: &mut Rng) -> bool {
    rng.next_u64() % 2 == 0
}
/// Generate a pair of i64 values in [-max, max].
#[allow(dead_code)]
pub fn gen_pair_i64_ext(rng: &mut Rng, max: i64) -> (i64, i64) {
    let max_u = (max as u64 * 2 + 1).max(1);
    let a = (rng.next_u64() % max_u) as i64 - max;
    let b = (rng.next_u64() % max_u) as i64 - max;
    (a, b)
}
/// Generate a sequence of i64 values.
#[allow(dead_code)]
pub fn gen_i64_sequence_ext(rng: &mut Rng, n: usize, max: i64) -> Vec<i64> {
    let max_u = (max as u64 * 2 + 1).max(1);
    (0..n)
        .map(|_| (rng.next_u64() % max_u) as i64 - max)
        .collect()
}
/// Shrink an i64 value towards 0.
#[allow(dead_code)]
pub fn shrink_i64(v: i64) -> Vec<i64> {
    if v == 0 {
        return vec![];
    }
    let mut result = vec![0i64];
    if v > 0 {
        result.push(v - 1);
    }
    if v < 0 {
        result.push(v + 1);
    }
    if v.abs() > 2 {
        result.push(v / 2);
    }
    result
}
/// Shrink a string towards empty.
#[allow(dead_code)]
pub fn shrink_string(s: &str) -> Vec<String> {
    if s.is_empty() {
        return vec![];
    }
    let mut result = vec!["".to_string()];
    if s.len() > 1 {
        result.push(s[..s.len() - 1].to_string());
        result.push(s[1..].to_string());
    }
    result
}
/// Shrink a `Vec<i64>`.
#[allow(dead_code)]
pub fn shrink_vec_i64(v: &[i64]) -> Vec<Vec<i64>> {
    if v.is_empty() {
        return vec![];
    }
    let mut result = vec![vec![]];
    if v.len() > 1 {
        result.push(v[..v.len() - 1].to_vec());
    }
    if let Some(&first) = v.first() {
        for s in shrink_i64(first) {
            let mut new_v = v.to_vec();
            new_v[0] = s;
            result.push(new_v);
        }
    }
    result
}
/// Run a property n times with different seeds.
#[allow(dead_code)]
pub fn run_property_check<F>(prop: F, n: usize, base_seed: u64) -> (usize, usize, usize)
where
    F: Fn(&mut Rng) -> PropCheckResult,
{
    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;
    for i in 0..n {
        let mut rng = Rng::new(base_seed.wrapping_add(i as u64 * 997));
        match prop(&mut rng) {
            PropCheckResult::Passed => passed += 1,
            PropCheckResult::Failed { .. } => failed += 1,
            PropCheckResult::Skipped { .. } => skipped += 1,
        }
    }
    (passed, failed, skipped)
}
/// Count free BVars (bound variable references with no binding).
#[allow(dead_code)]
pub fn count_free_bvars_ext(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) => 1,
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
        Expr::App(f, a) => count_free_bvars_ext(f) + count_free_bvars_ext(a),
        Expr::Lam(_, _, t, b) | Expr::Pi(_, _, t, b) | Expr::Let(_, _, t, b) => {
            count_free_bvars_ext(t) + count_free_bvars_ext(b)
        }
        Expr::Proj(_, _, e) => count_free_bvars_ext(e),
    }
}
/// Check if expression has any metavariables.
#[allow(dead_code)]
pub fn has_mvar_ext(expr: &Expr) -> bool {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, a) => has_mvar_ext(f) || has_mvar_ext(a),
        Expr::Lam(_, _, t, b) | Expr::Pi(_, _, t, b) | Expr::Let(_, _, t, b) => {
            has_mvar_ext(t) || has_mvar_ext(b)
        }
        Expr::Proj(_, _, e) => has_mvar_ext(e),
    }
}
/// Check if expression is "ground" (no BVars, no MVars).
#[allow(dead_code)]
pub fn is_ground_ext(expr: &Expr) -> bool {
    !has_mvar_ext(expr) && count_free_bvars_ext(expr) == 0
}
/// Compute the depth of a level expression.
#[allow(dead_code)]
pub fn level_depth_ext(l: &Level) -> usize {
    match l {
        Level::Zero | Level::Param(_) | Level::MVar(_) => 0,
        Level::Succ(inner) => 1 + level_depth_ext(inner),
        Level::Max(a, b) | Level::IMax(a, b) => 1 + level_depth_ext(a).max(level_depth_ext(b)),
    }
}
/// Check if a level has any metavariables.
#[allow(dead_code)]
pub fn level_has_mvar_ext(l: &Level) -> bool {
    match l {
        Level::MVar(_) => true,
        Level::Zero | Level::Param(_) => false,
        Level::Succ(inner) => level_has_mvar_ext(inner),
        Level::Max(a, b) | Level::IMax(a, b) => level_has_mvar_ext(a) || level_has_mvar_ext(b),
    }
}
/// Generate an arbitrary level up to depth n (new unique name).
#[allow(dead_code)]
pub fn gen_arbitrary_level(rng: &mut Rng, depth: usize) -> Level {
    if depth == 0 {
        match rng.next_usize(3) {
            0 => Level::Zero,
            1 => Level::Succ(Box::new(Level::Zero)),
            _ => Level::Param(Name::str("u")),
        }
    } else {
        match rng.next_usize(5) {
            0 => Level::Zero,
            1 => Level::Succ(Box::new(gen_arbitrary_level(rng, depth - 1))),
            2 => Level::Max(
                Box::new(gen_arbitrary_level(rng, depth - 1)),
                Box::new(gen_arbitrary_level(rng, depth - 1)),
            ),
            3 => Level::IMax(
                Box::new(gen_arbitrary_level(rng, depth - 1)),
                Box::new(gen_arbitrary_level(rng, depth - 1)),
            ),
            _ => Level::Param(Name::str("u")),
        }
    }
}
#[cfg(test)]
mod prop_test_ext_2 {
    use super::*;
    use crate::prop_test::*;
    #[test]
    fn test_shrink_i64_zero() {
        assert!(shrink_i64(0).is_empty());
    }
    #[test]
    fn test_shrink_i64_positive() {
        let s = shrink_i64(5);
        assert!(s.contains(&0));
        assert!(s.contains(&4));
    }
    #[test]
    fn test_shrink_i64_negative() {
        let s = shrink_i64(-3);
        assert!(s.contains(&0));
        assert!(s.contains(&-2));
    }
    #[test]
    fn test_shrink_string_empty() {
        assert!(shrink_string("").is_empty());
    }
    #[test]
    fn test_shrink_string_single() {
        let s = shrink_string("a");
        assert!(s.contains(&"".to_string()));
    }
    #[test]
    fn test_shrink_vec_i64_empty() {
        let v: Vec<i64> = vec![];
        assert!(shrink_vec_i64(&v).is_empty());
    }
    #[test]
    fn test_shrink_vec_i64_singleton() {
        let v = vec![5i64];
        let s = shrink_vec_i64(&v);
        assert!(s.contains(&vec![]));
    }
    #[test]
    fn test_gen_ident_ext() {
        let mut rng = Rng::new(42);
        let id = gen_ident_ext(&mut rng, 5);
        assert!(!id.is_empty());
        assert!(id.chars().all(|c| c.is_ascii_lowercase()));
    }
    #[test]
    fn test_gen_pair_i64_ext() {
        let mut rng = Rng::new(42);
        let (a, b) = gen_pair_i64_ext(&mut rng, 100);
        assert!((-100..=100).contains(&a));
        assert!((-100..=100).contains(&b));
    }
    #[test]
    fn test_gen_i64_sequence_ext() {
        let mut rng = Rng::new(7);
        let seq = gen_i64_sequence_ext(&mut rng, 10, 50);
        assert_eq!(seq.len(), 10);
    }
    #[test]
    fn test_prop_check_result_passed() {
        let r = PropCheckResult::Passed;
        assert!(r.is_passed());
        assert!(!r.is_failed());
    }
    #[test]
    fn test_run_property_check_all_pass() {
        let (pass, fail, skip) = run_property_check(|_| PropCheckResult::Passed, 10, 42);
        assert_eq!(pass, 10);
        assert_eq!(fail, 0);
        assert_eq!(skip, 0);
    }
    #[test]
    fn test_count_free_bvars_ext() {
        let e = Expr::BVar(0);
        assert_eq!(count_free_bvars_ext(&e), 1);
    }
    #[test]
    fn test_has_mvar_ext() {
        let e = Expr::Const(Name::str("Foo"), vec![]);
        assert!(!has_mvar_ext(&e));
    }
    #[test]
    fn test_is_ground_ext() {
        let e = Expr::Const(Name::str("foo"), vec![]);
        assert!(is_ground_ext(&e));
    }
    #[test]
    fn test_prop_test_suite_ext() {
        let mut suite = PropTestSuiteExt::new("test");
        suite.add_test("always true", || true);
        assert!(suite.all_pass());
        assert_eq!(suite.num_tests(), 1);
    }
    #[test]
    fn test_level_depth_ext() {
        let l = Level::Succ(Box::new(Level::Succ(Box::new(Level::Zero))));
        assert_eq!(level_depth_ext(&l), 2);
    }
    #[test]
    fn test_level_has_mvar_ext() {
        use oxilean_kernel::LevelMVarId;
        let l = Level::MVar(LevelMVarId(0));
        assert!(level_has_mvar_ext(&l));
    }
    #[test]
    fn test_gen_arbitrary_level() {
        let mut rng = Rng::new(42);
        let l = gen_arbitrary_level(&mut rng, 2);
        assert!(level_depth_ext(&l) <= 3);
    }
    #[test]
    fn test_regression_test_ext() {
        let mut test: RegressionTestExt<i32> = RegressionTestExt::new("t", "1+1", 2);
        test.set_actual(2);
        assert!(test.is_pass());
    }
    #[test]
    fn test_gen_bool_ext() {
        let mut rng = Rng::new(1);
        let _ = gen_bool_ext(&mut rng);
    }
}
#[cfg(test)]
mod proptest_analysis_tests {
    use super::*;
    use crate::prop_test::*;
    #[test]
    fn test_proptest_result_ok() {
        let r = PropTestResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_proptest_result_err() {
        let r = PropTestResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_proptest_result_partial() {
        let r = PropTestResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_proptest_result_skipped() {
        let r = PropTestResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_proptest_analysis_pass_run() {
        let mut p = PropTestAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_proptest_analysis_pass_empty_input() {
        let mut p = PropTestAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_proptest_analysis_pass_success_rate() {
        let mut p = PropTestAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_proptest_analysis_pass_disable() {
        let mut p = PropTestAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_proptest_pipeline_basic() {
        let mut pipeline = PropTestPipeline::new("main_pipeline");
        pipeline.add_pass(PropTestAnalysisPass::new("pass1"));
        pipeline.add_pass(PropTestAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_proptest_pipeline_disabled_pass() {
        let mut pipeline = PropTestPipeline::new("partial");
        let mut p = PropTestAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(PropTestAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_proptest_diff_basic() {
        let mut d = PropTestDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_proptest_diff_summary() {
        let mut d = PropTestDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_proptest_config_set_get() {
        let mut cfg = PropTestConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_proptest_config_read_only() {
        let mut cfg = PropTestConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_proptest_config_remove() {
        let mut cfg = PropTestConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_proptest_diagnostics_basic() {
        let mut diag = PropTestDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_proptest_diagnostics_max_errors() {
        let mut diag = PropTestDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_proptest_diagnostics_clear() {
        let mut diag = PropTestDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_proptest_config_value_types() {
        let b = PropTestConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = PropTestConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = PropTestConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = PropTestConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = PropTestConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod prop_test_ext_tests_700 {
    use super::*;
    use crate::prop_test::*;
    #[test]
    fn test_prop_test_ext_result_ok_700() {
        let r = PropTestExtResult700::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_prop_test_ext_result_err_700() {
        let r = PropTestExtResult700::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_prop_test_ext_result_partial_700() {
        let r = PropTestExtResult700::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_prop_test_ext_result_skipped_700() {
        let r = PropTestExtResult700::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_prop_test_ext_pass_run_700() {
        let mut p = PropTestExtPass700::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_prop_test_ext_pass_empty_700() {
        let mut p = PropTestExtPass700::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_prop_test_ext_pass_rate_700() {
        let mut p = PropTestExtPass700::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_prop_test_ext_pass_disable_700() {
        let mut p = PropTestExtPass700::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_prop_test_ext_pipeline_basic_700() {
        let mut pipeline = PropTestExtPipeline700::new("main_pipeline");
        pipeline.add_pass(PropTestExtPass700::new("pass1"));
        pipeline.add_pass(PropTestExtPass700::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_prop_test_ext_pipeline_disabled_700() {
        let mut pipeline = PropTestExtPipeline700::new("partial");
        let mut p = PropTestExtPass700::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(PropTestExtPass700::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_prop_test_ext_diff_basic_700() {
        let mut d = PropTestExtDiff700::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_prop_test_ext_config_set_get_700() {
        let mut cfg = PropTestExtConfig700::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_prop_test_ext_config_read_only_700() {
        let mut cfg = PropTestExtConfig700::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_prop_test_ext_config_remove_700() {
        let mut cfg = PropTestExtConfig700::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_prop_test_ext_diagnostics_basic_700() {
        let mut diag = PropTestExtDiag700::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_prop_test_ext_diagnostics_max_errors_700() {
        let mut diag = PropTestExtDiag700::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_prop_test_ext_diagnostics_clear_700() {
        let mut diag = PropTestExtDiag700::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_prop_test_ext_config_value_types_700() {
        let b = PropTestExtConfigVal700::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = PropTestExtConfigVal700::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = PropTestExtConfigVal700::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = PropTestExtConfigVal700::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = PropTestExtConfigVal700::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
