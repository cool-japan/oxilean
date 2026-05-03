//! Functions for tactic reflection — inspect and construct tactic expressions at meta-level.

use super::types::{GoalRepr, ReflectionCtx, RewriteDir, TacticRepr, TacticScript};

// ─── TacticRepr: rendering ───────────────────────────────────────────────────

impl TacticRepr {
    /// Render this tactic as Lean4-like tactic syntax.
    pub fn to_lean_string(&self) -> String {
        match self {
            TacticRepr::Apply { expr } => format!("apply {expr}"),
            TacticRepr::Intro { names } => {
                if names.is_empty() {
                    "intro".to_string()
                } else {
                    format!("intro {}", names.join(" "))
                }
            }
            TacticRepr::Rewrite { hyp, dir } => match dir {
                RewriteDir::LeftToRight => format!("rw [{hyp}]"),
                RewriteDir::RightToLeft => format!("rw [← {hyp}]"),
            },
            TacticRepr::Have { name, type_, proof } => {
                format!("have {name} : {type_} := by {}", proof.to_lean_string())
            }
            TacticRepr::Exact { expr } => format!("exact {expr}"),
            TacticRepr::Simp { lemmas } => {
                if lemmas.is_empty() {
                    "simp".to_string()
                } else {
                    format!("simp [{}]", lemmas.join(", "))
                }
            }
            TacticRepr::Seq(steps) => steps
                .iter()
                .map(|t| t.to_lean_string())
                .collect::<Vec<_>>()
                .join("\n"),
            TacticRepr::Alt(alts) => alts
                .iter()
                .map(|t| t.to_lean_string())
                .collect::<Vec<_>>()
                .join(" <|> "),
            TacticRepr::Repeat(inner) => format!("repeat ({})", inner.to_lean_string()),
            TacticRepr::Try(inner) => format!("try {}", inner.to_lean_string()),
            TacticRepr::Focus { goal_idx, tac } => {
                format!("focus {} ({})", goal_idx, tac.to_lean_string())
            }
            TacticRepr::Raw(s) => s.clone(),
        }
    }

    /// Parse a Lean4-like tactic string into a `TacticRepr`.
    ///
    /// Returns `None` if the string cannot be recognised as a known tactic.
    pub fn parse(s: &str) -> Option<TacticRepr> {
        let s = s.trim();

        // Seq: newline-separated tactics
        if s.contains('\n') {
            let steps: Vec<TacticRepr> = s
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .filter_map(TacticRepr::parse)
                .collect();
            if steps.is_empty() {
                return None;
            }
            return Some(TacticRepr::Seq(steps));
        }

        // Alt: <|>-separated
        if s.contains(" <|> ") {
            let alts: Vec<TacticRepr> = s
                .split(" <|> ")
                .filter_map(|p| TacticRepr::parse(p.trim()))
                .collect();
            if alts.is_empty() {
                return None;
            }
            return Some(TacticRepr::Alt(alts));
        }

        // repeat (...)
        if let Some(inner) = s.strip_prefix("repeat (").and_then(|r| r.strip_suffix(')')) {
            return TacticRepr::parse(inner).map(|t| TacticRepr::Repeat(Box::new(t)));
        }

        // try ...
        if let Some(rest) = s.strip_prefix("try ") {
            return TacticRepr::parse(rest).map(|t| TacticRepr::Try(Box::new(t)));
        }

        // focus N (...)
        if let Some(rest) = s.strip_prefix("focus ") {
            let parts: Vec<&str> = rest.splitn(2, ' ').collect();
            if parts.len() == 2 {
                if let Ok(idx) = parts[0].parse::<usize>() {
                    let inner_s = parts[1]
                        .trim()
                        .strip_prefix('(')
                        .and_then(|r| r.strip_suffix(')'))
                        .unwrap_or(parts[1]);
                    if let Some(tac) = TacticRepr::parse(inner_s) {
                        return Some(TacticRepr::Focus {
                            goal_idx: idx,
                            tac: Box::new(tac),
                        });
                    }
                }
            }
        }

        // apply ...
        if let Some(expr) = s.strip_prefix("apply ") {
            return Some(TacticRepr::Apply {
                expr: expr.trim().to_string(),
            });
        }

        // intro [names...]
        if let Some(rest) = s.strip_prefix("intro") {
            let rest = rest.trim();
            let names: Vec<String> = if rest.is_empty() {
                Vec::new()
            } else {
                rest.split_whitespace().map(String::from).collect()
            };
            return Some(TacticRepr::Intro { names });
        }

        // rw [← hyp] or rw [hyp]
        if let Some(rest) = s.strip_prefix("rw [") {
            let inner = rest.strip_suffix(']').unwrap_or(rest);
            let inner = inner.trim();
            if let Some(hyp) = inner.strip_prefix("← ") {
                return Some(TacticRepr::Rewrite {
                    hyp: hyp.trim().to_string(),
                    dir: RewriteDir::RightToLeft,
                });
            }
            return Some(TacticRepr::Rewrite {
                hyp: inner.to_string(),
                dir: RewriteDir::LeftToRight,
            });
        }

        // have name : type := by proof
        if let Some(rest) = s.strip_prefix("have ") {
            if let Some((lhs, rhs)) = rest.split_once(" := by ") {
                if let Some((name, type_)) = lhs.split_once(" : ") {
                    if let Some(proof) = TacticRepr::parse(rhs.trim()) {
                        return Some(TacticRepr::Have {
                            name: name.trim().to_string(),
                            type_: type_.trim().to_string(),
                            proof: Box::new(proof),
                        });
                    }
                }
            }
        }

        // exact ...
        if let Some(expr) = s.strip_prefix("exact ") {
            return Some(TacticRepr::Exact {
                expr: expr.trim().to_string(),
            });
        }

        // simp [a, b, ...] or simp
        if s == "simp" {
            return Some(TacticRepr::Simp { lemmas: Vec::new() });
        }
        if let Some(rest) = s.strip_prefix("simp [") {
            let inner = rest.strip_suffix(']').unwrap_or(rest);
            let lemmas = inner
                .split(',')
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
            return Some(TacticRepr::Simp { lemmas });
        }

        // Fallback: raw string
        if !s.is_empty() {
            return Some(TacticRepr::Raw(s.to_string()));
        }

        None
    }
}

// ─── Proof state simulation ───────────────────────────────────────────────────

/// Simulate applying a tactic to a proof state.
///
/// Returns a new `ReflectionCtx` representing the state after the tactic.
/// Returns `Err` if the tactic cannot be applied (e.g., no goals, bad index).
pub fn apply_tactic(ctx: &ReflectionCtx, tac: &TacticRepr) -> Result<ReflectionCtx, String> {
    if ctx.goals.is_empty() {
        return Err("No goals remaining".to_string());
    }

    let focus = ctx.focus;
    if focus >= ctx.goals.len() {
        return Err(format!(
            "Focus index {focus} out of range (only {} goals)",
            ctx.goals.len()
        ));
    }

    match tac {
        TacticRepr::Exact { .. } => {
            // Close the focused goal.
            let mut new_goals = ctx.goals.clone();
            new_goals.remove(focus);
            let new_focus = if new_goals.is_empty() {
                0
            } else {
                focus.min(new_goals.len() - 1)
            };
            Ok(ReflectionCtx {
                goals: new_goals,
                focus: new_focus,
            })
        }

        TacticRepr::Intro { names } => {
            // Each intro name adds a hypothesis to the focused goal and strips a Pi from target.
            let mut goal = ctx.goals[focus].clone();
            for name in names {
                // Simulate stripping a Pi: target must look like "A → B" or "∀ x : A, B"
                if let Some((dom, cod)) = split_arrow(&goal.target) {
                    goal.hyps.push((name.clone(), dom));
                    goal.target = cod;
                } else {
                    return Err(format!(
                        "Cannot intro '{name}': goal '{}' is not an implication",
                        goal.target
                    ));
                }
            }
            let mut new_goals = ctx.goals.clone();
            new_goals[focus] = goal;
            Ok(ReflectionCtx {
                goals: new_goals,
                focus,
            })
        }

        TacticRepr::Apply { expr } => {
            // Simulate apply: the goal is replaced by a new sub-goal for each argument.
            // For this reflective simulation, we just replace the target with "?arg of {expr}".
            let mut new_goals = ctx.goals.clone();
            let old_hyps = new_goals[focus].hyps.clone();
            new_goals[focus] = GoalRepr {
                hyps: old_hyps,
                target: format!("?arg_of_{}", sanitise_ident(expr)),
            };
            Ok(ReflectionCtx {
                goals: new_goals,
                focus,
            })
        }

        TacticRepr::Rewrite { hyp, dir } => {
            // Simulate rewrite: mark the goal as rewritten.
            let mut new_goals = ctx.goals.clone();
            let old_target = new_goals[focus].target.clone();
            let dir_sym = match dir {
                RewriteDir::LeftToRight => "→",
                RewriteDir::RightToLeft => "←",
            };
            new_goals[focus].target = format!("rw[{dir_sym}{hyp}]({old_target})");
            Ok(ReflectionCtx {
                goals: new_goals,
                focus,
            })
        }

        TacticRepr::Have { name, type_, proof } => {
            // First check the proof sub-tactic can be applied to a sub-context.
            let sub_ctx = ReflectionCtx::new(vec![GoalRepr::simple(type_.clone())]);
            let after_proof = apply_tactic(&sub_ctx, proof)?;
            if !after_proof.is_complete() {
                return Err(format!(
                    "'have' proof did not close the sub-goal for '{name}'"
                ));
            }
            // Add hypothesis to focused goal.
            let mut new_goals = ctx.goals.clone();
            new_goals[focus].hyps.push((name.clone(), type_.clone()));
            Ok(ReflectionCtx {
                goals: new_goals,
                focus,
            })
        }

        TacticRepr::Simp { .. } => {
            // Simulate simp: closes the focused goal (optimistic simulation).
            let mut new_goals = ctx.goals.clone();
            new_goals.remove(focus);
            let new_focus = if new_goals.is_empty() {
                0
            } else {
                focus.min(new_goals.len() - 1)
            };
            Ok(ReflectionCtx {
                goals: new_goals,
                focus: new_focus,
            })
        }

        TacticRepr::Seq(steps) => {
            // Apply each step in sequence.
            let mut current = ctx.clone();
            for step in steps {
                if current.is_complete() {
                    break;
                }
                current = apply_tactic(&current, step)?;
            }
            Ok(current)
        }

        TacticRepr::Alt(alts) => {
            // Try each alternative, return the first that succeeds.
            let mut last_err = String::from("No alternatives provided");
            for alt in alts {
                match apply_tactic(ctx, alt) {
                    Ok(new_ctx) => return Ok(new_ctx),
                    Err(e) => last_err = e,
                }
            }
            Err(format!("All alternatives failed; last error: {last_err}"))
        }

        TacticRepr::Repeat(inner) => {
            // Apply until failure, return last successful state.
            let mut current = ctx.clone();
            loop {
                if current.is_complete() {
                    break;
                }
                match apply_tactic(&current, inner) {
                    Ok(next) => {
                        if next == current {
                            // No progress — stop to avoid infinite loop.
                            break;
                        }
                        current = next;
                    }
                    Err(_) => break,
                }
            }
            Ok(current)
        }

        TacticRepr::Try(inner) => {
            // Apply inner; if it fails, return original state.
            Ok(apply_tactic(ctx, inner).unwrap_or_else(|_| ctx.clone()))
        }

        TacticRepr::Focus { goal_idx, tac } => {
            if *goal_idx >= ctx.goals.len() {
                return Err(format!(
                    "Focus index {goal_idx} out of range (only {} goals)",
                    ctx.goals.len()
                ));
            }
            let sub_ctx = ReflectionCtx {
                goals: ctx.goals.clone(),
                focus: *goal_idx,
            };
            apply_tactic(&sub_ctx, tac)
        }

        TacticRepr::Raw(s) => {
            // Raw tactics are treated as a no-op in simulation.
            Err(format!("Cannot simulate raw tactic: '{s}'"))
        }
    }
}

/// Split `"A → B"` into `("A", "B")`.  Returns `None` if not an arrow type.
fn split_arrow(target: &str) -> Option<(String, String)> {
    // Look for " → " (Lean4 arrow)
    if let Some(pos) = target.find(" → ") {
        let dom = target[..pos].trim().to_string();
        let cod = target[pos + " → ".len()..].trim().to_string();
        return Some((dom, cod));
    }
    // Also handle ASCII arrow " -> "
    if let Some(pos) = target.find(" -> ") {
        let dom = target[..pos].trim().to_string();
        let cod = target[pos + " -> ".len()..].trim().to_string();
        return Some((dom, cod));
    }
    None
}

/// Sanitise an expression to be used as an identifier fragment.
fn sanitise_ident(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

// ─── Script combinators ───────────────────────────────────────────────────────

/// Combine two tactics into a two-step `Seq`.
pub fn combine_tactics(t1: TacticRepr, t2: TacticRepr) -> TacticRepr {
    TacticRepr::Seq(vec![t1, t2])
}

/// Build a `Seq` from a list of tactics, flattening any nested `Seq` nodes.
pub fn sequence(tactics: Vec<TacticRepr>) -> TacticRepr {
    let flat = flatten_seq(tactics);
    match flat.len() {
        0 => TacticRepr::Seq(Vec::new()),
        1 => flat
            .into_iter()
            .next()
            .unwrap_or(TacticRepr::Seq(Vec::new())),
        _ => TacticRepr::Seq(flat),
    }
}

/// Recursively flatten nested `Seq` nodes into a single level.
fn flatten_seq(tactics: Vec<TacticRepr>) -> Vec<TacticRepr> {
    let mut result = Vec::new();
    for t in tactics {
        match t {
            TacticRepr::Seq(inner) => result.extend(flatten_seq(inner)),
            other => result.push(other),
        }
    }
    result
}

// ─── Proof state queries ──────────────────────────────────────────────────────

/// Return true if the proof state has no remaining goals.
pub fn is_complete(ctx: &ReflectionCtx) -> bool {
    ctx.is_complete()
}

/// Return the number of remaining proof goals.
pub fn goal_count(ctx: &ReflectionCtx) -> usize {
    ctx.goal_count()
}

// ─── Script parsing and optimisation ─────────────────────────────────────────

/// Parse a multi-line tactic script string into a `TacticScript`.
///
/// Each non-empty line is parsed as a separate tactic.
pub fn script_from_string(s: &str) -> TacticScript {
    let steps: Vec<TacticRepr> = s
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("--"))
        .filter_map(TacticRepr::parse)
        .collect();
    TacticScript::from_steps(steps)
}

/// Optimise a tactic script by:
/// - Removing redundant `Try` wrappers around infallible tactics (`Exact`, `Simp`).
/// - Flattening nested `Seq` nodes.
/// - Removing empty `Seq`/`Alt` nodes.
pub fn optimize_script(script: TacticScript) -> TacticScript {
    let steps = script.steps.into_iter().map(optimize_tactic).collect();
    TacticScript::from_steps(flatten_seq(steps))
}

/// Recursively optimise a single tactic.
fn optimize_tactic(tac: TacticRepr) -> TacticRepr {
    match tac {
        TacticRepr::Try(inner) => {
            let opt = optimize_tactic(*inner);
            // Remove Try around always-succeeding tactics.
            match opt {
                TacticRepr::Exact { .. } | TacticRepr::Simp { .. } => opt,
                other => TacticRepr::Try(Box::new(other)),
            }
        }
        TacticRepr::Seq(steps) => {
            let optimised: Vec<TacticRepr> = steps.into_iter().map(optimize_tactic).collect();
            let flat = flatten_seq(optimised);
            match flat.len() {
                0 => TacticRepr::Seq(Vec::new()),
                1 => flat
                    .into_iter()
                    .next()
                    .unwrap_or(TacticRepr::Seq(Vec::new())),
                _ => TacticRepr::Seq(flat),
            }
        }
        TacticRepr::Alt(alts) => {
            let optimised: Vec<TacticRepr> = alts.into_iter().map(optimize_tactic).collect();
            if optimised.len() == 1 {
                optimised
                    .into_iter()
                    .next()
                    .unwrap_or(TacticRepr::Alt(Vec::new()))
            } else {
                TacticRepr::Alt(optimised)
            }
        }
        TacticRepr::Repeat(inner) => TacticRepr::Repeat(Box::new(optimize_tactic(*inner))),
        TacticRepr::Have { name, type_, proof } => TacticRepr::Have {
            name,
            type_,
            proof: Box::new(optimize_tactic(*proof)),
        },
        TacticRepr::Focus { goal_idx, tac } => TacticRepr::Focus {
            goal_idx,
            tac: Box::new(optimize_tactic(*tac)),
        },
        other => other,
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic_reflection::types::{GoalRepr, ReflectionCtx, RewriteDir, TacticRepr};

    fn simple_ctx(target: &str) -> ReflectionCtx {
        ReflectionCtx::new(vec![GoalRepr::simple(target)])
    }

    fn arrow_ctx(dom: &str, cod: &str) -> ReflectionCtx {
        ReflectionCtx::new(vec![GoalRepr::simple(format!("{dom} → {cod}"))])
    }

    // ── to_lean_string ──────────────────────────────────────────────────────

    #[test]
    fn test_apply_to_string() {
        let t = TacticRepr::Apply {
            expr: "Nat.succ_pos".to_string(),
        };
        assert_eq!(t.to_lean_string(), "apply Nat.succ_pos");
    }

    #[test]
    fn test_intro_to_string_no_names() {
        let t = TacticRepr::Intro { names: vec![] };
        assert_eq!(t.to_lean_string(), "intro");
    }

    #[test]
    fn test_intro_to_string_names() {
        let t = TacticRepr::Intro {
            names: vec!["x".into(), "y".into()],
        };
        assert_eq!(t.to_lean_string(), "intro x y");
    }

    #[test]
    fn test_rewrite_ltr_to_string() {
        let t = TacticRepr::Rewrite {
            hyp: "h".into(),
            dir: RewriteDir::LeftToRight,
        };
        assert_eq!(t.to_lean_string(), "rw [h]");
    }

    #[test]
    fn test_rewrite_rtl_to_string() {
        let t = TacticRepr::Rewrite {
            hyp: "h".into(),
            dir: RewriteDir::RightToLeft,
        };
        assert_eq!(t.to_lean_string(), "rw [← h]");
    }

    #[test]
    fn test_exact_to_string() {
        let t = TacticRepr::Exact { expr: "rfl".into() };
        assert_eq!(t.to_lean_string(), "exact rfl");
    }

    #[test]
    fn test_simp_empty_to_string() {
        let t = TacticRepr::Simp { lemmas: vec![] };
        assert_eq!(t.to_lean_string(), "simp");
    }

    #[test]
    fn test_simp_lemmas_to_string() {
        let t = TacticRepr::Simp {
            lemmas: vec!["add_comm".into(), "mul_one".into()],
        };
        assert_eq!(t.to_lean_string(), "simp [add_comm, mul_one]");
    }

    #[test]
    fn test_try_to_string() {
        let inner = TacticRepr::Exact { expr: "rfl".into() };
        let t = TacticRepr::Try(Box::new(inner));
        assert_eq!(t.to_lean_string(), "try exact rfl");
    }

    #[test]
    fn test_repeat_to_string() {
        let inner = TacticRepr::Simp { lemmas: vec![] };
        let t = TacticRepr::Repeat(Box::new(inner));
        assert_eq!(t.to_lean_string(), "repeat (simp)");
    }

    #[test]
    fn test_raw_to_string() {
        let t = TacticRepr::Raw("omega".into());
        assert_eq!(t.to_lean_string(), "omega");
    }

    // ── parse ───────────────────────────────────────────────────────────────

    #[test]
    fn test_parse_apply() {
        let t = TacticRepr::parse("apply foo").expect("parse apply");
        assert_eq!(t, TacticRepr::Apply { expr: "foo".into() });
    }

    #[test]
    fn test_parse_intro_no_names() {
        let t = TacticRepr::parse("intro").expect("parse intro");
        assert_eq!(t, TacticRepr::Intro { names: vec![] });
    }

    #[test]
    fn test_parse_intro_names() {
        let t = TacticRepr::parse("intro x y z").expect("parse intro names");
        assert_eq!(
            t,
            TacticRepr::Intro {
                names: vec!["x".into(), "y".into(), "z".into()]
            }
        );
    }

    #[test]
    fn test_parse_rw_ltr() {
        let t = TacticRepr::parse("rw [h]").expect("parse rw ltr");
        assert_eq!(
            t,
            TacticRepr::Rewrite {
                hyp: "h".into(),
                dir: RewriteDir::LeftToRight
            }
        );
    }

    #[test]
    fn test_parse_rw_rtl() {
        let t = TacticRepr::parse("rw [← h]").expect("parse rw rtl");
        assert_eq!(
            t,
            TacticRepr::Rewrite {
                hyp: "h".into(),
                dir: RewriteDir::RightToLeft
            }
        );
    }

    #[test]
    fn test_parse_exact() {
        let t = TacticRepr::parse("exact rfl").expect("parse exact");
        assert_eq!(t, TacticRepr::Exact { expr: "rfl".into() });
    }

    #[test]
    fn test_parse_simp_empty() {
        let t = TacticRepr::parse("simp").expect("parse simp");
        assert_eq!(t, TacticRepr::Simp { lemmas: vec![] });
    }

    #[test]
    fn test_parse_simp_lemmas() {
        let t = TacticRepr::parse("simp [add_comm, mul_one]").expect("parse simp lemmas");
        assert_eq!(
            t,
            TacticRepr::Simp {
                lemmas: vec!["add_comm".into(), "mul_one".into()]
            }
        );
    }

    #[test]
    fn test_parse_try() {
        let t = TacticRepr::parse("try exact rfl").expect("parse try");
        assert_eq!(
            t,
            TacticRepr::Try(Box::new(TacticRepr::Exact { expr: "rfl".into() }))
        );
    }

    #[test]
    fn test_parse_raw_fallback() {
        let t = TacticRepr::parse("omega").expect("parse raw fallback");
        assert_eq!(t, TacticRepr::Raw("omega".into()));
    }

    #[test]
    fn test_parse_empty_returns_none() {
        assert!(TacticRepr::parse("").is_none());
        assert!(TacticRepr::parse("   ").is_none());
    }

    // ── apply_tactic ────────────────────────────────────────────────────────

    #[test]
    fn test_exact_closes_goal() {
        let ctx = simple_ctx("P");
        let t = TacticRepr::Exact { expr: "h".into() };
        let next = apply_tactic(&ctx, &t).expect("exact closes goal");
        assert!(next.is_complete());
    }

    #[test]
    fn test_simp_closes_goal() {
        let ctx = simple_ctx("P");
        let t = TacticRepr::Simp { lemmas: vec![] };
        let next = apply_tactic(&ctx, &t).expect("simp closes goal");
        assert!(next.is_complete());
    }

    #[test]
    fn test_intro_strips_arrow() {
        let ctx = arrow_ctx("P", "Q");
        let t = TacticRepr::Intro {
            names: vec!["h".into()],
        };
        let next = apply_tactic(&ctx, &t).expect("intro strips arrow");
        assert_eq!(next.goals[0].target, "Q");
        assert!(next.goals[0].hyps.iter().any(|(n, _)| n == "h"));
    }

    #[test]
    fn test_intro_fails_on_non_arrow() {
        let ctx = simple_ctx("Nat");
        let t = TacticRepr::Intro {
            names: vec!["x".into()],
        };
        assert!(apply_tactic(&ctx, &t).is_err());
    }

    #[test]
    fn test_seq_applies_in_order() {
        let ctx = arrow_ctx("A", "B → C");
        let t = TacticRepr::Seq(vec![
            TacticRepr::Intro {
                names: vec!["a".into()],
            },
            TacticRepr::Intro {
                names: vec!["b".into()],
            },
        ]);
        let next = apply_tactic(&ctx, &t).expect("seq applies");
        assert_eq!(next.goals[0].target, "C");
    }

    #[test]
    fn test_alt_picks_first_success() {
        let ctx = simple_ctx("P");
        let t = TacticRepr::Alt(vec![
            TacticRepr::Exact {
                expr: "proof".into(),
            },
            TacticRepr::Raw("omega".into()),
        ]);
        let next = apply_tactic(&ctx, &t).expect("alt picks first");
        assert!(next.is_complete());
    }

    #[test]
    fn test_apply_tactic_on_empty_fails() {
        let ctx = ReflectionCtx::empty();
        let t = TacticRepr::Exact { expr: "x".into() };
        assert!(apply_tactic(&ctx, &t).is_err());
    }

    // ── combine / sequence ───────────────────────────────────────────────────

    #[test]
    fn test_combine_tactics() {
        let t1 = TacticRepr::Intro {
            names: vec!["x".into()],
        };
        let t2 = TacticRepr::Exact { expr: "x".into() };
        let combined = combine_tactics(t1.clone(), t2.clone());
        assert_eq!(combined, TacticRepr::Seq(vec![t1, t2]));
    }

    #[test]
    fn test_sequence_flattens() {
        let inner = TacticRepr::Seq(vec![
            TacticRepr::Exact { expr: "a".into() },
            TacticRepr::Exact { expr: "b".into() },
        ]);
        let outer = vec![inner, TacticRepr::Simp { lemmas: vec![] }];
        let flat = sequence(outer);
        match flat {
            TacticRepr::Seq(steps) => assert_eq!(steps.len(), 3),
            other => panic!("Expected Seq, got {other:?}"),
        }
    }

    // ── script_from_string / optimize_script ─────────────────────────────────

    #[test]
    fn test_script_from_string() {
        let s = "intro x\nexact x\n";
        let script = script_from_string(s);
        assert_eq!(script.len(), 2);
    }

    #[test]
    fn test_script_skips_comments() {
        let s = "-- this is a comment\nintro x\n";
        let script = script_from_string(s);
        assert_eq!(script.len(), 1);
    }

    #[test]
    fn test_optimize_removes_try_around_exact() {
        let script = TacticScript::from_steps(vec![TacticRepr::Try(Box::new(TacticRepr::Exact {
            expr: "rfl".into(),
        }))]);
        let opt = optimize_script(script);
        assert_eq!(opt.steps[0], TacticRepr::Exact { expr: "rfl".into() });
    }

    #[test]
    fn test_optimize_removes_try_around_simp() {
        let script = TacticScript::from_steps(vec![TacticRepr::Try(Box::new(TacticRepr::Simp {
            lemmas: vec![],
        }))]);
        let opt = optimize_script(script);
        assert_eq!(opt.steps[0], TacticRepr::Simp { lemmas: vec![] });
    }

    #[test]
    fn test_optimize_flattens_seq() {
        let script = TacticScript::from_steps(vec![TacticRepr::Seq(vec![
            TacticRepr::Seq(vec![TacticRepr::Exact { expr: "a".into() }]),
            TacticRepr::Simp { lemmas: vec![] },
        ])]);
        let opt = optimize_script(script);
        assert_eq!(opt.steps.len(), 2);
    }

    #[test]
    fn test_is_complete_empty_ctx() {
        let ctx = ReflectionCtx::empty();
        assert!(is_complete(&ctx));
    }

    #[test]
    fn test_goal_count() {
        let ctx = ReflectionCtx::new(vec![GoalRepr::simple("A"), GoalRepr::simple("B")]);
        assert_eq!(goal_count(&ctx), 2);
    }
}
