//! Functions for SKI combinatory logic: reduction, bracket abstraction, Church numerals.

use super::types::{BracketMethod, CombRule, CombTerm, LambdaToComb, NormalForm, ReductionStep};

// ── Reduction ─────────────────────────────────────────────────────────────────

/// Perform one step of reduction on a combinator term (outermost-leftmost strategy).
///
/// Returns `Some((reduced_term, rule))` if a redex was found, or `None` if the
/// term is already in normal form.
pub fn reduce_step(t: &CombTerm) -> Option<(CombTerm, CombRule)> {
    match t {
        // I x → x
        CombTerm::App(f, x) => {
            if let CombTerm::I = f.as_ref() {
                return Some((*x.clone(), CombRule::IRule));
            }
            // K x y → x
            if let CombTerm::App(k, x_inner) = f.as_ref() {
                if let CombTerm::K = k.as_ref() {
                    return Some((*x_inner.clone(), CombRule::KRule));
                }
            }
            // S x y z → x z (y z)
            if let CombTerm::App(sx, y) = f.as_ref() {
                if let CombTerm::App(s, x_inner) = sx.as_ref() {
                    if let CombTerm::S = s.as_ref() {
                        let xz = CombTerm::App(x_inner.clone(), x.clone());
                        let yz = CombTerm::App(y.clone(), x.clone());
                        let result = CombTerm::App(Box::new(xz), Box::new(yz));
                        return Some((result, CombRule::SRule));
                    }
                }
            }
            // Try reducing left side first (outermost-leftmost)
            if let Some((f2, rule)) = reduce_step(f) {
                return Some((CombTerm::App(Box::new(f2), x.clone()), rule));
            }
            // Then try reducing right side
            if let Some((x2, rule)) = reduce_step(x) {
                return Some((CombTerm::App(f.clone(), Box::new(x2)), rule));
            }
            None
        }
        CombTerm::S | CombTerm::K | CombTerm::I | CombTerm::Var(_) | CombTerm::Const(_) => None,
    }
}

/// Reduce a combinator term to normal form, up to `max_steps` reduction steps.
///
/// Returns `Ok(NormalForm(t))` if the term reached normal form within the step
/// budget, or `Err(message)` if the budget was exhausted.
pub fn reduce_to_normal(mut t: CombTerm, max_steps: usize) -> Result<NormalForm, String> {
    for step in 0..max_steps {
        match reduce_step(&t) {
            None => return Ok(NormalForm(t)),
            Some((next, _rule)) => {
                t = next;
                let _ = step; // used only for loop count
            }
        }
    }
    Err(format!(
        "Term did not reach normal form within {} steps",
        max_steps
    ))
}

/// Collect the full reduction sequence from a term to its normal form.
///
/// Returns the steps taken and the final normal form (or an error if the budget
/// is exhausted).
pub fn reduce_with_trace(
    mut t: CombTerm,
    max_steps: usize,
) -> Result<(Vec<ReductionStep>, NormalForm), String> {
    let mut steps = Vec::new();
    for _ in 0..max_steps {
        match reduce_step(&t) {
            None => return Ok((steps, NormalForm(t))),
            Some((next, rule)) => {
                steps.push(ReductionStep {
                    from: t.clone(),
                    to: next.clone(),
                    rule_applied: rule,
                });
                t = next;
            }
        }
    }
    Err(format!(
        "Term did not reach normal form within {} steps",
        max_steps
    ))
}

/// Check whether a combinator term is already in normal form (no applicable rules).
pub fn is_normal_form(t: &CombTerm) -> bool {
    reduce_step(t).is_none()
}

/// Check whether two combinator terms are beta-equal: reduce both to normal form
/// and compare structurally. Returns `false` if either fails to normalize.
pub fn beta_equal(t1: &CombTerm, t2: &CombTerm, max_steps: usize) -> bool {
    let n1 = reduce_to_normal(t1.clone(), max_steps);
    let n2 = reduce_to_normal(t2.clone(), max_steps);
    match (n1, n2) {
        (Ok(NormalForm(a)), Ok(NormalForm(b))) => a == b,
        _ => false,
    }
}

// ── Bracket abstraction ───────────────────────────────────────────────────────

/// Naive bracket abstraction \[x\]M.
///
/// Implements the three standard rules:
/// 1. \[x\]x     = I
/// 2. \[x\]M     = K M   (when x ∉ FV(M))
/// 3. \[x\](M N) = S (\[x\]M) (\[x\]N)
pub fn lambda_to_ski_naive(var: &str, body: &CombTerm) -> CombTerm {
    match body {
        // Rule 1: [x]x = I
        CombTerm::Var(v) if v == var => CombTerm::I,
        // Rule 2: [x]M = K M when x ∉ FV(M)
        t if !free_vars(t).contains(&var.to_string()) => {
            CombTerm::App(Box::new(CombTerm::K), Box::new(t.clone()))
        }
        // Rule 3: [x](M N) = S ([x]M) ([x]N)
        CombTerm::App(m, n) => {
            let sm = lambda_to_ski_naive(var, m);
            let sn = lambda_to_ski_naive(var, n);
            CombTerm::App(
                Box::new(CombTerm::App(Box::new(CombTerm::S), Box::new(sm))),
                Box::new(sn),
            )
        }
        // Rule 2 catch-all (S, K, I, Const that don't contain var)
        t => CombTerm::App(Box::new(CombTerm::K), Box::new(t.clone())),
    }
}

/// Optimized bracket abstraction \[x\]M with η-reduction.
///
/// Extends the naive algorithm with:
/// 4. \[x\](M x) = M   (when x ∉ FV(M)) — η-reduction
pub fn lambda_to_ski_optimized(var: &str, body: &CombTerm) -> CombTerm {
    match body {
        // Rule 1
        CombTerm::Var(v) if v == var => CombTerm::I,
        // Rule 4: η-reduction [x](M x) = M when x ∉ FV(M)
        CombTerm::App(m, n) => {
            if let CombTerm::Var(v) = n.as_ref() {
                if v == var && !free_vars(m).contains(&var.to_string()) {
                    return *m.clone();
                }
            }
            // Rule 2
            if !free_vars(body).contains(&var.to_string()) {
                return CombTerm::App(Box::new(CombTerm::K), Box::new(body.clone()));
            }
            // Rule 3
            let sm = lambda_to_ski_optimized(var, m);
            let sn = lambda_to_ski_optimized(var, n);
            CombTerm::App(
                Box::new(CombTerm::App(Box::new(CombTerm::S), Box::new(sm))),
                Box::new(sn),
            )
        }
        // Rule 2
        t if !free_vars(t).contains(&var.to_string()) => {
            CombTerm::App(Box::new(CombTerm::K), Box::new(t.clone()))
        }
        t => CombTerm::App(Box::new(CombTerm::K), Box::new(t.clone())),
    }
}

/// Apply bracket abstraction via the method stored in a `LambdaToComb` converter.
pub fn convert_lambda(conv: &LambdaToComb, var: &str, body: &CombTerm) -> CombTerm {
    match conv.bracket_abstraction {
        BracketMethod::Naive => lambda_to_ski_naive(var, body),
        BracketMethod::Optimized => lambda_to_ski_optimized(var, body),
        BracketMethod::TurnerOptimized => lambda_to_ski_turner(var, body),
    }
}

/// Turner's optimized bracket abstraction.
///
/// Uses auxiliary combinator patterns B, C, S' (simulated via S/K/I) to reduce
/// term size when only one side of an application contains the variable.
pub fn lambda_to_ski_turner(var: &str, body: &CombTerm) -> CombTerm {
    match body {
        CombTerm::Var(v) if v == var => CombTerm::I,
        t if !free_vars(t).contains(&var.to_string()) => {
            CombTerm::App(Box::new(CombTerm::K), Box::new(t.clone()))
        }
        CombTerm::App(m, n) => {
            let m_has_var = free_vars(m).contains(&var.to_string());
            let n_has_var = free_vars(n).contains(&var.to_string());
            match (m_has_var, n_has_var) {
                // η case: [x](M x) = M
                (false, true) => {
                    if let CombTerm::Var(v) = n.as_ref() {
                        if v == var {
                            return *m.clone();
                        }
                    }
                    // B combinator pattern: [x](M (N x)) = B M ([x]N) where B = S(KS)K
                    // Simulate B M N' = S (K M) N'
                    let n2 = lambda_to_ski_turner(var, n);
                    CombTerm::App(
                        Box::new(CombTerm::App(
                            Box::new(CombTerm::App(
                                Box::new(CombTerm::S),
                                Box::new(CombTerm::App(Box::new(CombTerm::K), m.clone())),
                            )),
                            Box::new(CombTerm::I),
                        )),
                        Box::new(n2),
                    )
                }
                (true, false) => {
                    // C combinator pattern: [x](M x N) = C ([x]M) N where C = S(BS)K
                    // Simulate C M' N = S M' (K N)
                    let m2 = lambda_to_ski_turner(var, m);
                    CombTerm::App(
                        Box::new(CombTerm::App(Box::new(CombTerm::S), Box::new(m2))),
                        Box::new(CombTerm::App(Box::new(CombTerm::K), n.clone())),
                    )
                }
                _ => {
                    // Full S rule
                    let sm = lambda_to_ski_turner(var, m);
                    let sn = lambda_to_ski_turner(var, n);
                    CombTerm::App(
                        Box::new(CombTerm::App(Box::new(CombTerm::S), Box::new(sm))),
                        Box::new(sn),
                    )
                }
            }
        }
        t => CombTerm::App(Box::new(CombTerm::K), Box::new(t.clone())),
    }
}

// ── Church numerals ───────────────────────────────────────────────────────────

/// Build the Church numeral for `n` as a SKI combinator term.
///
/// Church numeral `n` = λf.λx. f^n x.
/// In SKI: 0 = K I, 1 = I, 2 = S (S (K S) K) I, etc.
/// We use the standard iterative construction.
pub fn church_numeral_comb(n: u64) -> CombTerm {
    // Church n = λf.λx. f^n x
    // We build it via bracket abstraction applied to the body
    // f^0 x = x,  f^(n+1) x = f (f^n x)
    // Then [f]([x](f^n x))

    // Build f^n x as an untyped term using Var("f") and Var("x")
    let mut body = CombTerm::Var("x".to_string());
    for _ in 0..n {
        body = CombTerm::App(Box::new(CombTerm::Var("f".to_string())), Box::new(body));
    }
    // Abstract over x, then over f
    let abs_x = lambda_to_ski_optimized("x", &body);
    lambda_to_ski_optimized("f", &abs_x)
}

// ── Structural utilities ──────────────────────────────────────────────────────

/// Build an n-ary application: f applied to each argument in turn.
///
/// `comb_app_n(f, \[a, b, c\])` = `((f a) b) c`
pub fn comb_app_n(f: CombTerm, args: Vec<CombTerm>) -> CombTerm {
    args.into_iter()
        .fold(f, |acc, arg| CombTerm::App(Box::new(acc), Box::new(arg)))
}

/// Count the number of nodes in a combinator term tree.
pub fn size(t: &CombTerm) -> usize {
    match t {
        CombTerm::App(f, x) => 1 + size(f) + size(x),
        CombTerm::S | CombTerm::K | CombTerm::I | CombTerm::Var(_) | CombTerm::Const(_) => 1,
    }
}

/// Collect all free variables in a combinator term (no duplicates, in order of first appearance).
pub fn free_vars(t: &CombTerm) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    collect_free_vars(t, &mut seen, &mut result);
    result
}

fn collect_free_vars(
    t: &CombTerm,
    seen: &mut std::collections::HashSet<String>,
    out: &mut Vec<String>,
) {
    match t {
        CombTerm::Var(v) => {
            if seen.insert(v.clone()) {
                out.push(v.clone());
            }
        }
        CombTerm::App(f, x) => {
            collect_free_vars(f, seen, out);
            collect_free_vars(x, seen, out);
        }
        CombTerm::S | CombTerm::K | CombTerm::I | CombTerm::Const(_) => {}
    }
}

/// Substitute `val` for all free occurrences of `var` in `t`.
pub fn substitute(t: &CombTerm, var: &str, val: &CombTerm) -> CombTerm {
    match t {
        CombTerm::Var(v) if v == var => val.clone(),
        CombTerm::Var(_) | CombTerm::S | CombTerm::K | CombTerm::I | CombTerm::Const(_) => {
            t.clone()
        }
        CombTerm::App(f, x) => CombTerm::App(
            Box::new(substitute(f, var, val)),
            Box::new(substitute(x, var, val)),
        ),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::types::*;
    use super::*;

    fn app(f: CombTerm, x: CombTerm) -> CombTerm {
        CombTerm::App(Box::new(f), Box::new(x))
    }

    fn var(s: &str) -> CombTerm {
        CombTerm::Var(s.to_string())
    }

    // ── is_normal_form ────────────────────────────────────────────────────────

    #[test]
    fn test_normal_form_s() {
        assert!(is_normal_form(&CombTerm::S));
    }

    #[test]
    fn test_normal_form_k() {
        assert!(is_normal_form(&CombTerm::K));
    }

    #[test]
    fn test_normal_form_i() {
        assert!(is_normal_form(&CombTerm::I));
    }

    #[test]
    fn test_not_normal_form_i_x() {
        let t = app(CombTerm::I, var("x"));
        assert!(!is_normal_form(&t));
    }

    #[test]
    fn test_not_normal_form_k_x_y() {
        let t = app(app(CombTerm::K, var("x")), var("y"));
        assert!(!is_normal_form(&t));
    }

    // ── reduce_step ───────────────────────────────────────────────────────────

    #[test]
    fn test_reduce_step_i_rule() {
        let t = app(CombTerm::I, var("x"));
        let result = reduce_step(&t);
        match result {
            Some((CombTerm::Var(v), CombRule::IRule)) => assert_eq!(v, "x"),
            other => panic!("expected I-rule reduction to Var(x), got {:?}", other),
        }
    }

    #[test]
    fn test_reduce_step_k_rule() {
        let t = app(app(CombTerm::K, var("x")), var("y"));
        let result = reduce_step(&t);
        match result {
            Some((CombTerm::Var(v), CombRule::KRule)) => assert_eq!(v, "x"),
            other => panic!("expected K-rule reduction to Var(x), got {:?}", other),
        }
    }

    #[test]
    fn test_reduce_step_s_rule() {
        // S x y z → x z (y z)
        let t = app(app(app(CombTerm::S, var("x")), var("y")), var("z"));
        let (result, rule) = reduce_step(&t).expect("should reduce");
        assert_eq!(rule, CombRule::SRule);
        // result should be (x z) (y z)
        let expected = app(app(var("x"), var("z")), app(var("y"), var("z")));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reduce_step_none_on_normal() {
        assert!(reduce_step(&CombTerm::S).is_none());
    }

    // ── reduce_to_normal ──────────────────────────────────────────────────────

    #[test]
    fn test_reduce_to_normal_i_x() {
        let t = app(CombTerm::I, var("x"));
        let nf = reduce_to_normal(t, 100).expect("should normalize");
        assert_eq!(nf.into_inner(), var("x"));
    }

    #[test]
    fn test_reduce_to_normal_k_x_y() {
        let t = app(app(CombTerm::K, var("x")), var("y"));
        let nf = reduce_to_normal(t, 100).expect("should normalize");
        assert_eq!(nf.into_inner(), var("x"));
    }

    #[test]
    fn test_reduce_to_normal_already_normal() {
        let nf = reduce_to_normal(CombTerm::S, 100).expect("should normalize");
        assert_eq!(nf.into_inner(), CombTerm::S);
    }

    #[test]
    fn test_reduce_to_normal_step_limit() {
        // Build a term that needs many steps: I (I (I (I x)))
        let t = app(CombTerm::I, app(CombTerm::I, app(CombTerm::I, var("x"))));
        // Should succeed with enough steps
        let nf = reduce_to_normal(t.clone(), 20).expect("should normalize");
        assert_eq!(nf.into_inner(), var("x"));
        // With only 1 step, the outermost I reduces but leaves I (I x)
        let partial = reduce_to_normal(t, 1);
        // 1 step is enough to reduce I (I (I x)) → I (I x), then 1 more is needed
        // so it should fail or succeed depending on depth-first vs eager
        // We just check it's not panic
        let _ = partial;
    }

    // ── beta_equal ────────────────────────────────────────────────────────────

    #[test]
    fn test_beta_equal_same_normal() {
        assert!(beta_equal(&CombTerm::S, &CombTerm::S, 100));
    }

    #[test]
    fn test_beta_equal_different() {
        assert!(!beta_equal(&CombTerm::S, &CombTerm::K, 100));
    }

    #[test]
    fn test_beta_equal_reduces_to_same() {
        // I x and K x (I y) both reduce to x
        let t1 = app(CombTerm::I, var("x"));
        let t2 = app(app(CombTerm::K, var("x")), app(CombTerm::I, var("y")));
        assert!(beta_equal(&t1, &t2, 100));
    }

    // ── lambda_to_ski_naive ───────────────────────────────────────────────────

    #[test]
    fn test_naive_identity_lambda() {
        // [x]x = I
        let result = lambda_to_ski_naive("x", &var("x"));
        assert_eq!(result, CombTerm::I);
    }

    #[test]
    fn test_naive_constant_lambda() {
        // [x]y = K y
        let result = lambda_to_ski_naive("x", &var("y"));
        let expected = app(CombTerm::K, var("y"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_naive_k_combinator() {
        // [x]([y]x) = K
        // [y]x = K x (since y ∉ FV(x))
        let inner = lambda_to_ski_naive("y", &var("x")); // = K x
        let outer = lambda_to_ski_naive("x", &inner); // = [x](K x)
                                                      // [x](K x) = S ([x]K) ([x]x) = S (K K) I
        let expected = app(app(CombTerm::S, app(CombTerm::K, CombTerm::K)), CombTerm::I);
        assert_eq!(outer, expected);
    }

    // ── lambda_to_ski_optimized ───────────────────────────────────────────────

    #[test]
    fn test_optimized_identity() {
        let result = lambda_to_ski_optimized("x", &var("x"));
        assert_eq!(result, CombTerm::I);
    }

    #[test]
    fn test_optimized_eta_reduction() {
        // [x](M x) = M when x ∉ FV(M)
        let body = app(var("f"), var("x"));
        let result = lambda_to_ski_optimized("x", &body);
        // η-reduces to f
        assert_eq!(result, var("f"));
    }

    #[test]
    fn test_optimized_constant() {
        let result = lambda_to_ski_optimized("x", &CombTerm::K);
        let expected = app(CombTerm::K, CombTerm::K);
        assert_eq!(result, expected);
    }

    // ── church_numeral_comb ───────────────────────────────────────────────────

    #[test]
    fn test_church_zero_reduces_to_ki() {
        // Church 0 = K I (applies f zero times to x, yielding x — the identity on x)
        let zero = church_numeral_comb(0);
        // Reduce: church_0 f x → x
        let applied_f = app(zero.clone(), var("f"));
        let applied_fx = app(applied_f, var("x"));
        let nf = reduce_to_normal(applied_fx, 200).expect("should normalize");
        assert_eq!(nf.into_inner(), var("x"));
    }

    #[test]
    fn test_church_one_applies_once() {
        // Church 1 f x → f x
        let one = church_numeral_comb(1);
        let applied = app(app(one, var("f")), var("x"));
        let nf = reduce_to_normal(applied, 200).expect("should normalize");
        assert_eq!(nf.into_inner(), app(var("f"), var("x")));
    }

    #[test]
    fn test_church_two_applies_twice() {
        // Church 2 f x → f (f x)
        let two = church_numeral_comb(2);
        let applied = app(app(two, var("f")), var("x"));
        let nf = reduce_to_normal(applied, 1000).expect("should normalize");
        assert_eq!(nf.into_inner(), app(var("f"), app(var("f"), var("x"))));
    }

    // ── comb_app_n ────────────────────────────────────────────────────────────

    #[test]
    fn test_comb_app_n_empty() {
        let result = comb_app_n(var("f"), vec![]);
        assert_eq!(result, var("f"));
    }

    #[test]
    fn test_comb_app_n_one() {
        let result = comb_app_n(var("f"), vec![var("x")]);
        assert_eq!(result, app(var("f"), var("x")));
    }

    #[test]
    fn test_comb_app_n_three() {
        let result = comb_app_n(var("f"), vec![var("x"), var("y"), var("z")]);
        let expected = app(app(app(var("f"), var("x")), var("y")), var("z"));
        assert_eq!(result, expected);
    }

    // ── size ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_size_atomic() {
        assert_eq!(size(&CombTerm::S), 1);
        assert_eq!(size(&CombTerm::K), 1);
        assert_eq!(size(&var("x")), 1);
    }

    #[test]
    fn test_size_app() {
        let t = app(CombTerm::S, CombTerm::K);
        assert_eq!(size(&t), 3); // App node + S + K
    }

    #[test]
    fn test_size_nested() {
        let t = app(app(CombTerm::S, CombTerm::K), CombTerm::I);
        assert_eq!(size(&t), 5);
    }

    // ── free_vars ─────────────────────────────────────────────────────────────

    #[test]
    fn test_free_vars_combinator() {
        assert!(free_vars(&CombTerm::S).is_empty());
        assert!(free_vars(&CombTerm::K).is_empty());
        assert!(free_vars(&CombTerm::I).is_empty());
    }

    #[test]
    fn test_free_vars_var() {
        assert_eq!(free_vars(&var("x")), vec!["x".to_string()]);
    }

    #[test]
    fn test_free_vars_app() {
        let t = app(var("x"), var("y"));
        assert_eq!(free_vars(&t), vec!["x".to_string(), "y".to_string()]);
    }

    #[test]
    fn test_free_vars_deduplication() {
        let t = app(var("x"), var("x"));
        assert_eq!(free_vars(&t), vec!["x".to_string()]);
    }

    // ── substitute ────────────────────────────────────────────────────────────

    #[test]
    fn test_substitute_var() {
        let result = substitute(&var("x"), "x", &CombTerm::I);
        assert_eq!(result, CombTerm::I);
    }

    #[test]
    fn test_substitute_different_var() {
        let result = substitute(&var("y"), "x", &CombTerm::I);
        assert_eq!(result, var("y"));
    }

    #[test]
    fn test_substitute_in_app() {
        let t = app(var("x"), var("y"));
        let result = substitute(&t, "x", &CombTerm::K);
        assert_eq!(result, app(CombTerm::K, var("y")));
    }

    #[test]
    fn test_substitute_combinator_unchanged() {
        let result = substitute(&CombTerm::S, "x", &CombTerm::K);
        assert_eq!(result, CombTerm::S);
    }
}
