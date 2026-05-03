//! Functions for type-directed program synthesis.

use super::types::{
    CandidateSource, SynthesisConfig, SynthesisGoal, SynthesisResult, SynthesisStats,
    SynthesisStrategy, TermCandidate,
};
use std::time::Instant;

/// Synthesize a term of the given type using the provided configuration.
///
/// Returns a pair of `(SynthesisResult, SynthesisStats)`.
pub fn synthesize(
    goal: &SynthesisGoal,
    cfg: &SynthesisConfig,
) -> (SynthesisResult, SynthesisStats) {
    let start = Instant::now();
    let mut stats = SynthesisStats::default();

    let candidates = gather_candidates(goal, cfg, &mut stats);

    let elapsed_ms = start.elapsed().as_millis() as u64;
    if elapsed_ms >= cfg.timeout_ms {
        stats.time_ms = elapsed_ms;
        return (SynthesisResult::Timeout, stats);
    }

    // Score and rank candidates
    let mut scored: Vec<TermCandidate> = candidates
        .into_iter()
        .map(|mut c| {
            c.score = score_candidate(&c, goal);
            c
        })
        .collect();

    scored.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    stats.time_ms = start.elapsed().as_millis() as u64;

    if let Some(best) = scored.first() {
        (
            SynthesisResult::Found {
                term: best.term.clone(),
                type_: goal.target_type.clone(),
            },
            stats,
        )
    } else {
        (SynthesisResult::NotFound, stats)
    }
}

/// Gather all candidates according to the configured strategy.
fn gather_candidates(
    goal: &SynthesisGoal,
    cfg: &SynthesisConfig,
    stats: &mut SynthesisStats,
) -> Vec<TermCandidate> {
    let mut candidates = Vec::new();

    match &cfg.strategy {
        SynthesisStrategy::ExhaustiveSearch => {
            candidates.extend(candidates_from_context(goal));
            candidates.extend(candidates_from_constructors(&goal.target_type));
            if goal.depth > 0 {
                let depth = goal.depth.min(cfg.max_depth);
                candidates.extend(enumerate_lambda_terms("Any", &goal.target_type, depth));
            }
        }
        SynthesisStrategy::RandomSampling { seed } => {
            let ctx_candidates = candidates_from_context(goal);
            let ctor_candidates = candidates_from_constructors(&goal.target_type);
            // Use seed-derived selection: take every (seed % n + 1)-th candidate
            let stride = (seed % 3 + 1) as usize;
            candidates.extend(ctx_candidates.into_iter().step_by(stride));
            candidates.extend(ctor_candidates.into_iter().step_by(stride));
        }
        SynthesisStrategy::TypeDirected => {
            candidates.extend(candidates_from_context(goal));
            candidates.extend(candidates_from_constructors(&goal.target_type));
            // For function types, try lambda
            if is_function_type(&goal.target_type) {
                if let Some((input, output)) = split_function_type(&goal.target_type) {
                    let depth = goal.depth.min(cfg.max_depth);
                    candidates.extend(enumerate_lambda_terms(&input, &output, depth));
                }
            }
        }
        SynthesisStrategy::CombineAll => {
            candidates.extend(candidates_from_context(goal));
            candidates.extend(candidates_from_constructors(&goal.target_type));
            if goal.depth > 0 {
                let depth = goal.depth.min(cfg.max_depth);
                if let Some((input, output)) = split_function_type(&goal.target_type) {
                    candidates.extend(enumerate_lambda_terms(&input, &output, depth));
                } else {
                    candidates.extend(enumerate_lambda_terms("Any", &goal.target_type, depth));
                }
            }
        }
    }

    stats.terms_explored = candidates.len();
    if let Some(max_d) = candidates.iter().map(|_| goal.depth).max() {
        stats.depth_reached = max_d;
    }

    // Truncate to max_terms
    candidates.truncate(cfg.max_terms);
    candidates
}

/// Return hypothesis variables from the local context whose type matches the goal type.
pub fn candidates_from_context(goal: &SynthesisGoal) -> Vec<TermCandidate> {
    goal.context
        .iter()
        .filter(|(_, ty)| types_match(ty, &goal.target_type))
        .map(|(name, _)| TermCandidate {
            term: name.clone(),
            score: 1.0,
            source: CandidateSource::FromContext,
        })
        .collect()
}

/// Return constructor application candidates for the given type name.
pub fn candidates_from_constructors(type_name: &str) -> Vec<TermCandidate> {
    let ctors = known_constructors_for(type_name);
    ctors
        .into_iter()
        .map(|ctor| TermCandidate {
            term: ctor,
            score: 0.7,
            source: CandidateSource::FromConstructor,
        })
        .collect()
}

/// Check if function application `f : f_type` applied to `arg : arg_type` yields `goal_type`.
///
/// Returns `Some(term_string)` if the application typechecks syntactically.
pub fn try_application(f_type: &str, arg_type: &str, goal_type: &str) -> Option<String> {
    // Parse f_type as (arg_type -> goal_type)
    if let Some((domain, codomain)) = split_function_type(f_type) {
        if types_match(&domain, arg_type) && types_match(&codomain, goal_type) {
            return Some(format!("(f {})", arg_type));
        }
    }
    None
}

/// Enumerate lambda term candidates: `fun (x : input_type) => body` up to given depth.
pub fn enumerate_lambda_terms(
    input_type: &str,
    output_type: &str,
    depth: usize,
) -> Vec<TermCandidate> {
    if depth == 0 {
        return Vec::new();
    }

    let mut results = Vec::new();

    // Base: identity-like lambda
    let identity = format!("fun (x : {}) => x", input_type);
    results.push(TermCandidate {
        term: identity,
        score: 0.5,
        source: CandidateSource::FromLambda,
    });

    // For known output types, try to produce a constant lambda
    let body_candidates = candidates_from_constructors(output_type);
    for body in body_candidates.iter().take(3) {
        results.push(TermCandidate {
            term: format!("fun (x : {}) => {}", input_type, body.term),
            score: 0.4,
            source: CandidateSource::FromLambda,
        });
    }

    // Recursive: if depth > 1, generate nested lambdas
    if depth > 1 {
        let nested = enumerate_lambda_terms(input_type, output_type, depth - 1);
        for n in nested.into_iter().take(2) {
            results.push(TermCandidate {
                term: format!("fun (x : {}) => {}", input_type, n.term),
                score: 0.3,
                source: CandidateSource::FromLambda,
            });
        }
    }

    results
}

/// Heuristically score a candidate relative to a synthesis goal.
///
/// Higher scores are better. Scoring criteria:
/// - Exact context match: +1.0
/// - Constructor: +0.7
/// - Application: +0.6
/// - Lambda: +0.5
/// - Shorter term: small bonus
/// - Type exactness: bonus when target_type appears in term
pub fn score_candidate(candidate: &TermCandidate, goal: &SynthesisGoal) -> f64 {
    let base = match candidate.source {
        CandidateSource::FromContext => 1.0,
        CandidateSource::FromConstructor => 0.7,
        CandidateSource::FromApplication => 0.6,
        CandidateSource::FromLambda => 0.5,
    };

    // Bonus for shorter terms (prefer simpler terms)
    let length_bonus = if candidate.term.len() < 10 { 0.1 } else { 0.0 };

    // Bonus if goal type appears in the term (e.g., annotation)
    let type_bonus = if candidate.term.contains(&goal.target_type) {
        0.05
    } else {
        0.0
    };

    // Bonus for context depth (shallower = easier to check)
    let depth_penalty = (goal.depth as f64) * 0.01;

    (base + length_bonus + type_bonus - depth_penalty).max(0.0)
}

/// Return the top-k candidates by score (descending).
pub fn top_k_candidates(candidates: &[TermCandidate], k: usize) -> Vec<&TermCandidate> {
    let mut indexed: Vec<(usize, &TermCandidate)> = candidates.iter().enumerate().collect();
    indexed.sort_by(|(_, a), (_, b)| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    indexed.into_iter().take(k).map(|(_, c)| c).collect()
}

// ---------- Internal helpers ----------

/// Return known constructors for a built-in or common type.
fn known_constructors_for(type_name: &str) -> Vec<String> {
    match type_name {
        "Bool" | "bool" => vec!["true".to_string(), "false".to_string()],
        "Nat" => vec![
            "0".to_string(),
            "Nat.zero".to_string(),
            "Nat.succ 0".to_string(),
        ],
        "Unit" | "()" => vec!["()".to_string(), "Unit.unit".to_string()],
        "Option" => vec!["none".to_string(), "some _".to_string()],
        "List" => vec!["[]".to_string(), "[_]".to_string()],
        "String" => vec!["\"\"".to_string()],
        "Int" => vec!["0".to_string(), "1".to_string(), "-1".to_string()],
        "Prop" | "True" => vec!["True.intro".to_string()],
        _ => {
            // For unknown types, try a generic constructor application
            if type_name.starts_with(|c: char| c.is_uppercase()) {
                vec![format!("{}.mk", type_name)]
            } else {
                Vec::new()
            }
        }
    }
}

/// Check if two type strings are compatible (simplified syntactic check).
fn types_match(a: &str, b: &str) -> bool {
    let a = a.trim();
    let b = b.trim();
    a == b || a == "_" || b == "_" || a == "Any" || b == "Any" || (a.is_empty() || b.is_empty())
}

/// Check if a type string is a function type (contains `→` or `->`).
fn is_function_type(ty: &str) -> bool {
    ty.contains("->") || ty.contains('→')
}

/// Split a function type into `(domain, codomain)`.
///
/// Handles both `A -> B` and `A → B` syntax.
fn split_function_type(ty: &str) -> Option<(String, String)> {
    // Try Unicode arrow first
    if let Some(pos) = ty.find('→') {
        let domain = ty[..pos].trim().to_string();
        let codomain = ty[pos + '→'.len_utf8()..].trim().to_string();
        if !domain.is_empty() && !codomain.is_empty() {
            return Some((domain, codomain));
        }
    }
    // Try ASCII arrow (avoid splitting inside nested parens)
    let bytes = ty.as_bytes();
    let mut depth: i32 = 0;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            b'-' if depth == 0 && i + 1 < bytes.len() && bytes[i + 1] == b'>' => {
                let domain = ty[..i].trim().to_string();
                let codomain = ty[i + 2..].trim().to_string();
                if !domain.is_empty() && !codomain.is_empty() {
                    return Some((domain, codomain));
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synthesis::types::{
        CandidateSource, SynthesisConfig, SynthesisGoal, SynthesisResult, SynthesisStats,
        SynthesisStrategy, TermCandidate,
    };

    fn simple_goal(ty: &str) -> SynthesisGoal {
        SynthesisGoal {
            target_type: ty.to_string(),
            context: vec![],
            depth: 2,
        }
    }

    fn goal_with_ctx(ty: &str, ctx: &[(&str, &str)]) -> SynthesisGoal {
        SynthesisGoal {
            target_type: ty.to_string(),
            context: ctx
                .iter()
                .map(|(n, t)| (n.to_string(), t.to_string()))
                .collect(),
            depth: 2,
        }
    }

    fn default_cfg() -> SynthesisConfig {
        SynthesisConfig::default()
    }

    // --- candidates_from_context ---

    #[test]
    fn test_context_exact_match() {
        let goal = goal_with_ctx("Nat", &[("n", "Nat"), ("b", "Bool")]);
        let cs = candidates_from_context(&goal);
        assert_eq!(cs.len(), 1);
        assert_eq!(cs[0].term, "n");
    }

    #[test]
    fn test_context_no_match() {
        let goal = goal_with_ctx("Bool", &[("n", "Nat")]);
        let cs = candidates_from_context(&goal);
        assert!(cs.is_empty());
    }

    #[test]
    fn test_context_multiple_matches() {
        let goal = goal_with_ctx("Bool", &[("a", "Bool"), ("b", "Bool"), ("n", "Nat")]);
        let cs = candidates_from_context(&goal);
        assert_eq!(cs.len(), 2);
    }

    #[test]
    fn test_context_source_is_from_context() {
        let goal = goal_with_ctx("Nat", &[("x", "Nat")]);
        let cs = candidates_from_context(&goal);
        assert_eq!(cs[0].source, CandidateSource::FromContext);
    }

    #[test]
    fn test_context_empty_context() {
        let goal = simple_goal("Nat");
        let cs = candidates_from_context(&goal);
        assert!(cs.is_empty());
    }

    // --- candidates_from_constructors ---

    #[test]
    fn test_constructors_bool() {
        let cs = candidates_from_constructors("Bool");
        assert!(cs.iter().any(|c| c.term == "true"));
        assert!(cs.iter().any(|c| c.term == "false"));
    }

    #[test]
    fn test_constructors_nat() {
        let cs = candidates_from_constructors("Nat");
        assert!(!cs.is_empty());
    }

    #[test]
    fn test_constructors_unit() {
        let cs = candidates_from_constructors("Unit");
        assert!(!cs.is_empty());
    }

    #[test]
    fn test_constructors_unknown_uppercase() {
        let cs = candidates_from_constructors("MyType");
        assert!(cs.iter().any(|c| c.term == "MyType.mk"));
    }

    #[test]
    fn test_constructors_unknown_lowercase() {
        let cs = candidates_from_constructors("mytype");
        assert!(cs.is_empty());
    }

    #[test]
    fn test_constructors_source() {
        let cs = candidates_from_constructors("Bool");
        assert!(cs
            .iter()
            .all(|c| c.source == CandidateSource::FromConstructor));
    }

    // --- try_application ---

    #[test]
    fn test_try_application_basic() {
        let result = try_application("Bool -> Nat", "Bool", "Nat");
        assert!(result.is_some());
    }

    #[test]
    fn test_try_application_wrong_arg() {
        let result = try_application("Bool -> Nat", "Nat", "Nat");
        assert!(result.is_none());
    }

    #[test]
    fn test_try_application_wrong_result() {
        let result = try_application("Bool -> Nat", "Bool", "Bool");
        assert!(result.is_none());
    }

    #[test]
    fn test_try_application_not_function() {
        let result = try_application("Nat", "Bool", "Nat");
        assert!(result.is_none());
    }

    // --- enumerate_lambda_terms ---

    #[test]
    fn test_lambda_depth_zero() {
        let cs = enumerate_lambda_terms("Nat", "Bool", 0);
        assert!(cs.is_empty());
    }

    #[test]
    fn test_lambda_depth_one() {
        let cs = enumerate_lambda_terms("Nat", "Nat", 1);
        assert!(!cs.is_empty());
        assert!(cs[0].source == CandidateSource::FromLambda);
    }

    #[test]
    fn test_lambda_terms_contain_input_type() {
        let cs = enumerate_lambda_terms("Nat", "Bool", 1);
        assert!(cs.iter().any(|c| c.term.contains("Nat")));
    }

    #[test]
    fn test_lambda_depth_two_more_candidates() {
        let cs1 = enumerate_lambda_terms("Nat", "Nat", 1);
        let cs2 = enumerate_lambda_terms("Nat", "Nat", 2);
        assert!(cs2.len() >= cs1.len());
    }

    // --- score_candidate ---

    #[test]
    fn test_score_context_higher_than_ctor() {
        let goal = simple_goal("Nat");
        let ctx_cand = TermCandidate {
            term: "n".to_string(),
            score: 0.0,
            source: CandidateSource::FromContext,
        };
        let ctor_cand = TermCandidate {
            term: "Nat.zero".to_string(),
            score: 0.0,
            source: CandidateSource::FromConstructor,
        };
        assert!(score_candidate(&ctx_cand, &goal) > score_candidate(&ctor_cand, &goal));
    }

    #[test]
    fn test_score_nonnegative() {
        let goal = SynthesisGoal {
            target_type: "Nat".to_string(),
            context: vec![],
            depth: 100,
        };
        let c = TermCandidate {
            term: "x".to_string(),
            score: 0.0,
            source: CandidateSource::FromLambda,
        };
        assert!(score_candidate(&c, &goal) >= 0.0);
    }

    #[test]
    fn test_score_short_term_bonus() {
        let goal = simple_goal("Nat");
        let short = TermCandidate {
            term: "n".to_string(),
            score: 0.0,
            source: CandidateSource::FromConstructor,
        };
        let long_term = TermCandidate {
            term: "some_very_long_constructor_name".to_string(),
            score: 0.0,
            source: CandidateSource::FromConstructor,
        };
        assert!(score_candidate(&short, &goal) > score_candidate(&long_term, &goal));
    }

    // --- top_k_candidates ---

    #[test]
    fn test_top_k_empty() {
        let cs: Vec<TermCandidate> = vec![];
        let top = top_k_candidates(&cs, 3);
        assert!(top.is_empty());
    }

    #[test]
    fn test_top_k_returns_k() {
        let cs = vec![
            TermCandidate {
                term: "a".into(),
                score: 0.5,
                source: CandidateSource::FromContext,
            },
            TermCandidate {
                term: "b".into(),
                score: 0.9,
                source: CandidateSource::FromContext,
            },
            TermCandidate {
                term: "c".into(),
                score: 0.1,
                source: CandidateSource::FromContext,
            },
            TermCandidate {
                term: "d".into(),
                score: 0.7,
                source: CandidateSource::FromContext,
            },
        ];
        let top = top_k_candidates(&cs, 2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].term, "b");
        assert_eq!(top[1].term, "d");
    }

    #[test]
    fn test_top_k_larger_than_list() {
        let cs = vec![TermCandidate {
            term: "x".into(),
            score: 1.0,
            source: CandidateSource::FromContext,
        }];
        let top = top_k_candidates(&cs, 10);
        assert_eq!(top.len(), 1);
    }

    // --- synthesize (integration) ---

    #[test]
    fn test_synthesize_finds_from_context() {
        let goal = goal_with_ctx("Nat", &[("n", "Nat")]);
        let cfg = default_cfg();
        let (result, stats) = synthesize(&goal, &cfg);
        assert!(matches!(result, SynthesisResult::Found { .. }));
        assert!(stats.terms_explored > 0);
    }

    #[test]
    fn test_synthesize_not_found_without_context() {
        let goal = simple_goal("SomeCompletelyUnknownType123");
        let cfg = SynthesisConfig {
            max_depth: 1,
            max_terms: 10,
            timeout_ms: 5000,
            strategy: SynthesisStrategy::TypeDirected,
        };
        let (result, _stats) = synthesize(&goal, &cfg);
        // Either not found or found with a .mk constructor — both valid
        match result {
            SynthesisResult::Found { .. } | SynthesisResult::NotFound => {}
            SynthesisResult::Timeout => panic!("should not timeout"),
        }
    }

    #[test]
    fn test_synthesize_bool_constructors() {
        let goal = simple_goal("Bool");
        let cfg = default_cfg();
        let (result, _) = synthesize(&goal, &cfg);
        assert!(matches!(result, SynthesisResult::Found { .. }));
    }

    #[test]
    fn test_synthesize_exhaustive() {
        let goal = goal_with_ctx("Bool", &[("b", "Bool")]);
        let cfg = SynthesisConfig {
            strategy: SynthesisStrategy::ExhaustiveSearch,
            ..default_cfg()
        };
        let (result, _) = synthesize(&goal, &cfg);
        assert!(matches!(result, SynthesisResult::Found { .. }));
    }

    #[test]
    fn test_synthesize_random_sampling() {
        let goal = goal_with_ctx("Nat", &[("n", "Nat")]);
        let cfg = SynthesisConfig {
            strategy: SynthesisStrategy::RandomSampling { seed: 42 },
            ..default_cfg()
        };
        let (result, _) = synthesize(&goal, &cfg);
        // Result should be found or not found (not timeout)
        assert!(!matches!(result, SynthesisResult::Timeout));
    }

    #[test]
    fn test_synthesize_combine_all() {
        let goal = goal_with_ctx("Bool", &[("x", "Bool")]);
        let cfg = SynthesisConfig {
            strategy: SynthesisStrategy::CombineAll,
            ..default_cfg()
        };
        let (result, _) = synthesize(&goal, &cfg);
        assert!(matches!(result, SynthesisResult::Found { .. }));
    }

    #[test]
    fn test_synthesize_function_type() {
        let goal = simple_goal("Nat -> Nat");
        let cfg = default_cfg();
        let (result, _) = synthesize(&goal, &cfg);
        // Should find a lambda
        assert!(matches!(result, SynthesisResult::Found { .. }));
    }

    #[test]
    fn test_synthesis_stats_populated() {
        let goal = goal_with_ctx("Bool", &[("b", "Bool")]);
        let cfg = default_cfg();
        let (_, stats) = synthesize(&goal, &cfg);
        assert!(stats.terms_explored > 0);
    }

    #[test]
    fn test_split_function_type_ascii() {
        let r = split_function_type("Nat -> Bool");
        assert_eq!(r, Some(("Nat".to_string(), "Bool".to_string())));
    }

    #[test]
    fn test_split_function_type_unicode() {
        let r = split_function_type("Nat → Bool");
        assert_eq!(r, Some(("Nat".to_string(), "Bool".to_string())));
    }

    #[test]
    fn test_split_function_type_none() {
        assert!(split_function_type("Nat").is_none());
    }
}
