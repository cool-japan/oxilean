//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Literal, Name};

use super::functions::*;
use super::functions_3::{
    apply_extended_simp_rules, apply_simp_rules, has_farkas_certificate, is_const_named,
    parse_simp_only_lemmas, parse_sym_lin_cons, tactic_fin_cases_impl, tactic_simp,
    try_close_numeric, try_linarith_with_hyps, try_ring_norm,
};
use super::types::{NumCmp, SymLinCon, TacticError, TacticState, TypeShape};

/// Evaluate a single tactic given as a string reference.
///
/// Dispatches tactic names to their implementations.
pub fn eval_tactic(state: &TacticState, tactic_ref: &str) -> TacticResult {
    let trimmed = tactic_ref.trim();
    let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
    let tactic_name = parts[0];
    let args_str = if parts.len() > 1 { parts[1].trim() } else { "" };
    match tactic_name {
        "intro" => {
            let name = if args_str.is_empty() {
                Name::str("h")
            } else {
                Name::str(args_str)
            };
            tactic_intro(state, name)
        }
        "intros" => {
            let names: Vec<Name> = if args_str.is_empty() {
                vec![Name::str("h")]
            } else {
                args_str.split_whitespace().map(Name::str).collect()
            };
            tactic_intros(state, &names)
        }
        "exact" => {
            let expr = parse_simple_expr(args_str)?;
            tactic_exact(state, expr)
        }
        "assumption" => tactic_assumption(state),
        "refl" => tactic_refl(state),
        "trivial" => tactic_trivial(state),
        "sorry" => tactic_sorry(state),
        "exfalso" => tactic_exfalso(state),
        "constructor" => tactic_constructor(state),
        "split" => tactic_split(state),
        "left" => tactic_left(state),
        "right" => tactic_right(state),
        "clear" => {
            if args_str.is_empty() {
                return Err(TacticError::InvalidArg(
                    "clear requires a hypothesis name".to_string(),
                ));
            }
            tactic_clear(state, &Name::str(args_str))
        }
        "rename" => {
            let rename_parts: Vec<&str> = args_str.split_whitespace().collect();
            if rename_parts.len() < 2 {
                return Err(TacticError::InvalidArg(
                    "rename requires old and new names".to_string(),
                ));
            }
            tactic_rename(
                state,
                &Name::str(rename_parts[0]),
                Name::str(rename_parts[1]),
            )
        }
        "revert" => {
            if args_str.is_empty() {
                return Err(TacticError::InvalidArg(
                    "revert requires a hypothesis name".to_string(),
                ));
            }
            tactic_revert(state, &Name::str(args_str))
        }
        "apply" => {
            let expr = parse_simple_expr(args_str)?;
            let resolved = if let Expr::Const(ref name, _) = expr {
                if let Ok(goal) = get_focused_goal(state) {
                    if let Some(hyp_ty) = goal.find_hypothesis(name) {
                        hyp_ty.clone()
                    } else {
                        expr.clone()
                    }
                } else {
                    expr.clone()
                }
            } else {
                expr.clone()
            };
            tactic_apply(state, resolved)
        }
        "exists" | "use" => {
            let witness = parse_simple_expr(args_str)?;
            tactic_exists(state, witness)
        }
        "have" => {
            if let Some(colon_pos) = args_str.find(" : ") {
                let hyp_name = args_str[..colon_pos].trim();
                let ty_str = args_str[colon_pos + 3..].trim();
                let ty_expr = parse_simple_expr(ty_str)
                    .unwrap_or_else(|_| Expr::Const(Name::str("_"), vec![]));
                tactic_have(state, Name::str(hyp_name), ty_expr, None)
            } else {
                tactic_sorry(state)
            }
        }
        "show" => {
            let target = parse_simple_expr(args_str)?;
            let goal = get_focused_goal(state)?;
            let mut new_goal = goal.clone();
            new_goal.target = target;
            new_goal.mvar_id = fresh_mvar_id();
            replace_focused(state, vec![new_goal])
        }
        "simp" => {
            if args_str.starts_with("at ") {
                let hyp_name = args_str.strip_prefix("at ").unwrap_or(args_str).trim();
                let goal = get_focused_goal(state)?;
                let hyp_ty = goal
                    .find_hypothesis(&Name::str(hyp_name))
                    .ok_or_else(|| {
                        TacticError::InvalidArg(format!(
                            "simp at: hypothesis '{}' not found",
                            hyp_name
                        ))
                    })?
                    .clone();
                let simplified = apply_simp_rules(&beta_reduce(&hyp_ty));
                if simplified == hyp_ty {
                    return Ok(state.clone());
                }
                let mut new_state = state.clone();
                if is_const_named(&simplified, "False") {
                    return replace_focused(&new_state, vec![]);
                }
                for (n, t) in new_state.goals[0].hypotheses.iter_mut() {
                    if n == &Name::str(hyp_name) {
                        *t = simplified.clone();
                        break;
                    }
                }
                new_state.goals[0].mvar_id = fresh_mvar_id();
                return Ok(new_state);
            }
            let lemma_strings = if args_str.starts_with("only") || args_str.starts_with('[') {
                parse_simp_only_lemmas(args_str)
            } else {
                vec![]
            };
            let lemma_refs: Vec<&str> = lemma_strings.iter().map(|s| s.as_str()).collect();
            tactic_simp(state, &lemma_refs)
        }
        "simp_all" => {
            let goal = get_focused_goal(state)?;
            let hyp_names: Vec<String> =
                goal.hypotheses.iter().map(|(n, _)| n.to_string()).collect();
            let hyp_refs: Vec<&str> = hyp_names.iter().map(|s| s.as_str()).collect();
            tactic_simp(state, &hyp_refs)
        }
        "field_simp" => tactic_simp(state, &[]),
        "aesop" | "tauto" => {
            if let Ok(s) = tactic_trivial(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_assumption(state) {
                return Ok(s);
            }
            if let Ok(s) = try_close_numeric(state) {
                return Ok(s);
            }
            if let Ok(s) = try_linarith_with_hyps(state) {
                return Ok(s);
            }
            if let Ok(s) = try_ring_norm(state) {
                return Ok(s);
            }
            if let Ok(after_intro) = eval_tactic(state, "intro h_aesop") {
                if let Ok(s) = tactic_trivial(&after_intro)
                    .or_else(|_| tactic_assumption(&after_intro))
                    .or_else(|_| try_linarith_with_hyps(&after_intro))
                {
                    return Ok(s);
                }
            }
            if let Ok(after_ctor) = tactic_constructor(state) {
                let all_closed = after_ctor.goals.iter().all(|g| {
                    let sub = TacticState {
                        goals: vec![g.clone()],
                        solved: vec![],
                    };
                    tactic_assumption(&sub)
                        .or_else(|_| tactic_trivial(&sub))
                        .is_ok()
                });
                if all_closed {
                    let mut cur = after_ctor.clone();
                    let mut ok = true;
                    for _ in 0..cur.goals.len() {
                        match tactic_assumption(&cur).or_else(|_| tactic_trivial(&cur)) {
                            Ok(s) => cur = s,
                            Err(_) => {
                                ok = false;
                                break;
                            }
                        }
                    }
                    if ok && cur.is_complete() {
                        return Ok(cur);
                    }
                }
            }
            tactic_simp(state, &[]).or_else(|_| tactic_sorry(state))
        }
        "rw" | "rewrite" => {
            let (rw_spec, at_hyp) = if let Some(at_pos) = args_str.find(" at ") {
                (&args_str[..at_pos], Some(args_str[at_pos + 4..].trim()))
            } else {
                (args_str, None)
            };
            let inner = rw_spec
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim();
            let items: Vec<&str> = inner
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            if items.is_empty() {
                return tactic_sorry(state);
            }
            let mut current_state = state.clone();
            for item in items {
                let (forward, hyp_name_str) = if let Some(rest) = item.strip_prefix('\u{2190}') {
                    (false, rest.trim())
                } else if let Some(rest) = item.strip_prefix("<-") {
                    (false, rest.trim())
                } else {
                    (true, item)
                };
                let goal = get_focused_goal(&current_state)?;
                let eq_hyp_ty = goal
                    .find_hypothesis(&Name::str(hyp_name_str))
                    .ok_or_else(|| {
                        TacticError::InvalidArg(format!(
                            "rw: hypothesis '{}' not found",
                            hyp_name_str
                        ))
                    })?
                    .clone();
                let (from, to) = extract_eq_sides(&eq_hyp_ty, forward).ok_or_else(|| {
                    TacticError::TypeMismatch(format!(
                        "rw: '{}' is not an equality hypothesis",
                        hyp_name_str
                    ))
                })?;
                if let Some(target_hyp) = at_hyp {
                    let hyp_ty = goal
                        .find_hypothesis(&Name::str(target_hyp))
                        .ok_or_else(|| {
                            TacticError::InvalidArg(format!(
                                "rw at: hypothesis '{}' not found",
                                target_hyp
                            ))
                        })?
                        .clone();
                    let new_hyp_ty = rewrite_in_expr(&hyp_ty, &from, &to);
                    if new_hyp_ty == hyp_ty {
                        return Err(TacticError::TypeMismatch(
                            "rw at: pattern not found in hypothesis".to_string(),
                        ));
                    }
                    let mut new_state = current_state.clone();
                    for (n, t) in new_state.goals[0].hypotheses.iter_mut() {
                        if n == &Name::str(target_hyp) {
                            *t = new_hyp_ty.clone();
                            break;
                        }
                    }
                    for (n, t, _) in new_state.goals[0].local_ctx.iter_mut() {
                        if n == &Name::str(target_hyp) {
                            *t = new_hyp_ty.clone();
                            break;
                        }
                    }
                    new_state.goals[0].mvar_id = fresh_mvar_id();
                    current_state = new_state;
                } else {
                    let new_target = rewrite_in_expr(&goal.target, &from, &to);
                    if new_target == goal.target {
                        return Err(TacticError::TypeMismatch(
                            "rw: pattern not found in goal".to_string(),
                        ));
                    }
                    let mut new_state = current_state.clone();
                    new_state.goals[0].target = new_target;
                    new_state.goals[0].mvar_id = fresh_mvar_id();
                    current_state = new_state;
                }
            }
            Ok(current_state)
        }
        "obtain" => {
            let hyp = if let Some(pos) = args_str.rfind(":=") {
                args_str[pos + 2..].trim()
            } else {
                args_str.split_whitespace().last().unwrap_or(args_str)
            };
            if hyp.is_empty() {
                return Err(TacticError::InvalidArg(
                    "obtain requires a hypothesis name".to_string(),
                ));
            }
            tactic_cases(state, &Name::str(hyp))
        }
        "cases" => {
            if args_str.is_empty() {
                return Err(TacticError::InvalidArg(
                    "cases requires a hypothesis name".to_string(),
                ));
            }
            let hyp = args_str.split_whitespace().next().unwrap_or(args_str);
            tactic_cases(state, &Name::str(hyp))
        }
        "induction" => {
            if args_str.is_empty() {
                return Err(TacticError::InvalidArg(
                    "induction requires a hypothesis name".to_string(),
                ));
            }
            let hyp = args_str.split_whitespace().next().unwrap_or(args_str);
            tactic_induction(state, &Name::str(hyp))
        }
        "decide" => {
            if let Ok(s) = tactic_trivial(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_refl(state) {
                return Ok(s);
            }
            let goal = get_focused_goal(state)?;
            if is_refl_target(&goal.target) {
                return replace_focused(state, vec![]);
            }
            if let Ok(s) = try_close_numeric(state) {
                return Ok(s);
            }
            {
                let goal = get_focused_goal(state)?;
                if decide_prop_eval(&goal.target) {
                    return replace_focused(state, vec![]);
                }
            }
            {
                let goal = get_focused_goal(state)?;
                let (head, args) = decompose_app_tactic(&goal.target);
                if let Expr::Const(n, _) = head {
                    let hn = n.to_string();
                    if (hn == "Dvd.dvd" || hn == "Nat.dvd") && args.len() >= 2 {
                        if let (Some(k), Some(m)) = (
                            eval_nat_expr(args[args.len() - 2]),
                            eval_nat_expr(args[args.len() - 1]),
                        ) {
                            if k == 0 || m % k == 0 {
                                return replace_focused(state, vec![]);
                            }
                        }
                    }
                    if hn == "Nat.Prime" && !args.is_empty() {
                        if let Some(p) = eval_nat_expr(args[args.len() - 1]) {
                            if is_prime(p) {
                                return replace_focused(state, vec![]);
                            }
                        }
                    }
                }
            }
            tactic_sorry(state)
        }
        "push_neg" => {
            if args_str.starts_with("at ") {
                let hyp_name = args_str.strip_prefix("at ").unwrap_or(args_str).trim();
                let goal = get_focused_goal(state)?;
                let hyp_ty = goal
                    .find_hypothesis(&Name::str(hyp_name))
                    .ok_or_else(|| {
                        TacticError::InvalidArg(format!(
                            "push_neg at: hypothesis '{}' not found",
                            hyp_name
                        ))
                    })?
                    .clone();
                let new_ty = push_negations(&hyp_ty);
                if new_ty == hyp_ty {
                    return Ok(state.clone());
                }
                let mut new_state = state.clone();
                for (n, t) in new_state.goals[0].hypotheses.iter_mut() {
                    if n == &Name::str(hyp_name) {
                        *t = new_ty.clone();
                        break;
                    }
                }
                new_state.goals[0].mvar_id = fresh_mvar_id();
                Ok(new_state)
            } else {
                tactic_push_neg(state)
            }
        }
        "by_contra" | "by_contradiction" => {
            let name = if args_str.is_empty() {
                Name::str("h")
            } else {
                Name::str(args_str.split_whitespace().next().unwrap_or("h"))
            };
            tactic_by_contra(state, name)
        }
        "contrapose" => tactic_contrapose(state),
        "norm_cast" | "exact_mod_cast" | "push_cast" => {
            if let Ok(s) = tactic_refl(state) {
                return Ok(s);
            }
            tactic_sorry(state)
        }
        "rfl" => tactic_refl(state),
        "omega" => {
            if let Ok(s) = tactic_trivial(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_refl(state) {
                return Ok(s);
            }
            let goal = get_focused_goal(state)?;
            if is_refl_target(&goal.target) {
                return replace_focused(state, vec![]);
            }
            if let Ok(s) = try_close_numeric(state) {
                return Ok(s);
            }
            if let Ok(s) = try_linarith_with_hyps(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_assumption(state) {
                return Ok(s);
            }
            tactic_sorry(state)
        }
        "norm_num" | "positivity" => {
            if let Ok(s) = tactic_trivial(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_refl(state) {
                return Ok(s);
            }
            {
                let goal = get_focused_goal(state)?;
                let (head, args) = decompose_app_tactic(&goal.target);
                if let Expr::Const(n, _) = head {
                    let hn = n.to_string();
                    if (hn == "Nat.le" || hn == "LE.le")
                        && args.len() >= 2
                        && matches!(args[args.len() - 2], Expr::Lit(Literal::Nat(0)))
                    {
                        return replace_focused(state, vec![]);
                    }
                    if (hn == "Nat.ge" || hn == "GE.ge")
                        && args.len() >= 2
                        && matches!(args[args.len() - 1], Expr::Lit(Literal::Nat(0)))
                    {
                        return replace_focused(state, vec![]);
                    }
                    if (hn == "Nat.lt" || hn == "LT.lt")
                        && args.len() >= 2
                        && matches!(args[args.len() - 2], Expr::Lit(Literal::Nat(0)))
                    {
                        let rhs = args[args.len() - 1];
                        let is_positive = eval_nat_expr(rhs).map(|v| v > 0).unwrap_or(false)
                            || matches!(
                                rhs, Expr::App(f, _) if is_const_named(f, "Nat.succ")
                            );
                        if is_positive {
                            return replace_focused(state, vec![]);
                        }
                    }
                    if hn == "Ne" && args.len() >= 2 {
                        if let (Some(l), Some(r)) = (
                            eval_nat_expr(args[args.len() - 2]),
                            eval_nat_expr(args[args.len() - 1]),
                        ) {
                            if l != r {
                                return replace_focused(state, vec![]);
                            }
                        }
                    }
                    if hn == "Eq" && args.len() >= 2 {
                        if let (Some(l), Some(r)) = (
                            eval_nat_expr(args[args.len() - 2]),
                            eval_nat_expr(args[args.len() - 1]),
                        ) {
                            if l == r {
                                return replace_focused(state, vec![]);
                            }
                        }
                    }
                }
            }
            if let Ok(s) = try_close_numeric(state) {
                return Ok(s);
            }
            {
                let goal = get_focused_goal(state)?;
                let (head, args) = decompose_app_tactic(&goal.target);
                if let Expr::Const(n, _) = head {
                    let hn = n.to_string();
                    if (hn == "Dvd.dvd" || hn == "Nat.dvd" || hn == "Dvd") && args.len() >= 2 {
                        if let (Some(k), Some(m)) = (
                            eval_nat_expr(args[args.len() - 2]),
                            eval_nat_expr(args[args.len() - 1]),
                        ) {
                            if k == 0 || m % k == 0 {
                                return replace_focused(state, vec![]);
                            }
                        }
                    }
                    if hn == "Nat.Prime" && !args.is_empty() {
                        if let Some(p) = eval_nat_expr(args[args.len() - 1]) {
                            if is_prime(p) {
                                return replace_focused(state, vec![]);
                            }
                        }
                    }
                    if hn == "Nat.Coprime" && args.len() >= 2 {
                        if let (Some(a), Some(b)) = (
                            eval_nat_expr(args[args.len() - 2]),
                            eval_nat_expr(args[args.len() - 1]),
                        ) {
                            if nat_gcd(a, b) == 1 {
                                return replace_focused(state, vec![]);
                            }
                        }
                    }
                }
            }
            if let Ok(s) = try_ring_norm(state) {
                return Ok(s);
            }
            tactic_assumption(state).or_else(|_| tactic_sorry(state))
        }
        "fin_cases" | "interval_cases" => {
            let subject = if args_str.is_empty() {
                None
            } else {
                Some(args_str)
            };
            tactic_fin_cases_impl(state, subject)
        }
        "ring" => {
            if let Ok(s) = tactic_trivial(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_refl(state) {
                return Ok(s);
            }
            if let Ok(s) = try_ring_norm(state) {
                return Ok(s);
            }
            tactic_simp(state, &[]).or_else(|_| tactic_sorry(state))
        }
        "linarith" | "nlinarith" => {
            if let Ok(s) = tactic_trivial(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_refl(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_assumption(state) {
                return Ok(s);
            }
            if let Ok(s) = try_close_numeric(state) {
                return Ok(s);
            }
            if let Ok(s) = try_linarith_with_hyps(state) {
                return Ok(s);
            }
            tactic_sorry(state)
        }
        "exact?" | "apply?" | "simp?" | "rw?" => tactic_sorry(state),
        "repeat" => {
            if args_str.is_empty() {
                return Err(TacticError::InvalidArg(
                    "repeat requires a tactic".to_string(),
                ));
            }
            let mut current = state.clone();
            let mut iterations: u32 = 0;
            while let Ok(new_state) = eval_tactic(&current, args_str) {
                current = new_state;
                iterations += 1;
                if iterations > 100 || current.is_complete() {
                    break;
                }
            }
            Ok(current)
        }
        "first" => {
            let alternatives: Vec<&str> = args_str
                .split('|')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            for alt in &alternatives {
                if let Ok(new_state) = eval_tactic(state, alt) {
                    return Ok(new_state);
                }
            }
            Err(TacticError::UnknownTactic(
                "first: all alternatives failed".to_string(),
            ))
        }
        "try" => match eval_tactic(state, args_str) {
            Ok(new_state) => Ok(new_state),
            Err(_) => Ok(state.clone()),
        },
        "all_goals" => {
            if args_str.is_empty() {
                return Err(TacticError::InvalidArg(
                    "all_goals requires a tactic".to_string(),
                ));
            }
            let mut new_state = state.clone();
            let num_goals = new_state.num_goals();
            for _i in 0..num_goals {
                if new_state.is_complete() {
                    break;
                }
                match eval_tactic(&new_state, args_str) {
                    Ok(s) => new_state = s,
                    Err(e) => return Err(e),
                }
            }
            Ok(new_state)
        }
        "symm" | "symmetry" => {
            let goal = get_focused_goal(state)?;
            if let Some((lhs, rhs)) = extract_eq_sides(&goal.target, true) {
                let mut new_goal = goal.clone();
                new_goal.mvar_id = fresh_mvar_id();
                let eq_c = Expr::Const(Name::str("Eq"), vec![]);
                let swapped = if let Expr::App(f1, _) = &goal.target {
                    if let Expr::App(eq_ty, _lhs_orig) = f1.as_ref() {
                        Expr::App(
                            Box::new(Expr::App(eq_ty.clone(), Box::new(rhs.clone()))),
                            Box::new(lhs.clone()),
                        )
                    } else {
                        Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::App(Box::new(eq_c), Box::new(rhs.clone()))),
                                Box::new(rhs.clone()),
                            )),
                            Box::new(lhs.clone()),
                        )
                    }
                } else {
                    return Err(TacticError::TypeMismatch(
                        "symm: unexpected Eq structure".to_string(),
                    ));
                };
                new_goal.target = swapped;
                replace_focused(state, vec![new_goal])
            } else {
                Err(TacticError::TypeMismatch(
                    "symm: goal is not an equality".to_string(),
                ))
            }
        }
        "trans" => {
            let goal = get_focused_goal(state)?;
            let mid_expr = if args_str.is_empty() {
                return tactic_sorry(state);
            } else {
                parse_simple_expr(args_str)?
            };
            if let Expr::App(f1, rhs) = &goal.target {
                if let Expr::App(eq_ty, lhs) = f1.as_ref() {
                    let goal1_target = Expr::App(
                        Box::new(Expr::App(eq_ty.clone(), lhs.clone())),
                        Box::new(mid_expr.clone()),
                    );
                    let goal2_target = Expr::App(
                        Box::new(Expr::App(eq_ty.clone(), Box::new(mid_expr))),
                        rhs.clone(),
                    );
                    let mut g1 = goal.clone();
                    g1.target = goal1_target;
                    g1.mvar_id = fresh_mvar_id();
                    let mut g2 = goal.clone();
                    g2.target = goal2_target;
                    g2.mvar_id = fresh_mvar_id();
                    replace_focused(state, vec![g1, g2])
                } else {
                    Err(TacticError::TypeMismatch(
                        "trans: malformed equality goal".to_string(),
                    ))
                }
            } else {
                Err(TacticError::TypeMismatch(
                    "trans: goal is not an equality".to_string(),
                ))
            }
        }
        "congr" | "congr!" => {
            let goal = get_focused_goal(state)?;
            if let Some((lhs, rhs)) = extract_eq_sides(&goal.target, true) {
                if let (Expr::App(fl, al), Expr::App(fr, ar)) = (&lhs, &rhs) {
                    if fl == fr {
                        let sub_eq = Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                                    Box::new(*al.clone()),
                                )),
                                al.clone(),
                            )),
                            ar.clone(),
                        );
                        let mut new_goal = goal.clone();
                        new_goal.target = sub_eq;
                        new_goal.mvar_id = fresh_mvar_id();
                        return replace_focused(state, vec![new_goal]);
                    }
                }
                tactic_refl(state).or_else(|_| tactic_sorry(state))
            } else {
                Err(TacticError::TypeMismatch(
                    "congr: goal is not an equality".to_string(),
                ))
            }
        }
        "subst" => {
            let goal = get_focused_goal(state)?;
            let hyp_name = if args_str.is_empty() {
                return Err(TacticError::InvalidArg(
                    "subst requires a hypothesis name".to_string(),
                ));
            } else {
                Name::str(args_str.trim())
            };
            let hyp_ty = goal
                .find_hypothesis(&hyp_name)
                .ok_or_else(|| {
                    TacticError::InvalidArg(format!("subst: hypothesis '{}' not found", hyp_name))
                })?
                .clone();
            if let Some((lhs, rhs)) = extract_eq_sides(&hyp_ty, true) {
                let new_target = rewrite_in_expr(&goal.target, &lhs, &rhs);
                let mut new_goal = goal.clone();
                new_goal.target = new_target;
                new_goal.mvar_id = fresh_mvar_id();
                remove_hypothesis(&mut new_goal, &hyp_name);
                replace_focused(state, vec![new_goal])
            } else {
                Err(TacticError::TypeMismatch(
                    "subst: hypothesis is not an equality".to_string(),
                ))
            }
        }
        "specialize" => {
            if args_str.is_empty() {
                return Err(TacticError::InvalidArg(
                    "specialize requires a hypothesis name and arguments".to_string(),
                ));
            }
            let parts: Vec<&str> = args_str.splitn(2, ' ').collect();
            let hyp_name = Name::str(parts[0].trim());
            let arg_str = if parts.len() > 1 { parts[1].trim() } else { "" };
            let goal = get_focused_goal(state)?;
            let hyp_ty = goal
                .find_hypothesis(&hyp_name)
                .ok_or_else(|| {
                    TacticError::InvalidArg(format!("specialize: '{}' not found", hyp_name))
                })?
                .clone();
            if let Expr::Pi(_, _, _, body) = &hyp_ty {
                if arg_str.is_empty() {
                    return Err(TacticError::InvalidArg(
                        "specialize: missing argument".to_string(),
                    ));
                }
                let arg = parse_simple_expr(arg_str)?;
                let new_ty = substitute_bvar_0(body, &arg);
                let mut new_goal = goal.clone();
                new_goal.mvar_id = fresh_mvar_id();
                for (n, t) in &mut new_goal.hypotheses {
                    if *n == hyp_name {
                        *t = new_ty.clone();
                        break;
                    }
                }
                replace_focused(state, vec![new_goal])
            } else {
                Err(TacticError::TypeMismatch(
                    "specialize: hypothesis is not universally quantified".to_string(),
                ))
            }
        }
        "gcongr" => {
            if let Ok(s) = try_linarith_with_hyps(state) {
                return Ok(s);
            }
            tactic_sorry(state)
        }
        "contradiction" | "exfalso!" => {
            let goal = get_focused_goal(state)?;
            for (_, ty) in &goal.hypotheses {
                if is_const_named(ty, "False") {
                    return replace_focused(state, vec![]);
                }
            }
            let hyps = goal.hypotheses.clone();
            for (_, ty_i) in &hyps {
                for (_, ty_j) in &hyps {
                    if let Expr::App(f, p) = ty_j {
                        if is_const_named(f, "Not") && p.as_ref() == ty_i {
                            return replace_focused(state, vec![]);
                        }
                    }
                }
            }
            let mut all_cons: Vec<SymLinCon> = vec![];
            for (_, ty) in &hyps {
                if let Some(cs) = parse_sym_lin_cons(ty) {
                    all_cons.extend(cs);
                }
            }
            if has_farkas_certificate(&all_cons) {
                return replace_focused(state, vec![]);
            }
            Err(TacticError::TypeMismatch(
                "contradiction: no contradiction found in hypotheses".to_string(),
            ))
        }
        "absurd" => {
            if args_str.is_empty() {
                return tactic_sorry(state);
            }
            let parts: Vec<&str> = args_str.split_whitespace().collect();
            let hyp1_name = Name::str(parts[0]);
            let goal = get_focused_goal(state)?;
            let hyp1_ty = match goal.find_hypothesis(&hyp1_name) {
                Some(ty) => ty.clone(),
                None => return tactic_sorry(state),
            };
            for (_, ty) in &goal.hypotheses {
                if let Expr::App(f, p) = ty {
                    if is_const_named(f, "Not") && p.as_ref() == &hyp1_ty {
                        return replace_focused(state, vec![]);
                    }
                }
            }
            if parts.len() >= 2 {
                let hyp2_name = Name::str(parts[1]);
                if let Some(Expr::App(f, p)) = goal.find_hypothesis(&hyp2_name) {
                    if is_const_named(f, "Not") && p.as_ref() == &hyp1_ty {
                        return replace_focused(state, vec![]);
                    }
                }
            }
            tactic_sorry(state)
        }
        "funext" | "ext" => {
            let var_name = if args_str.is_empty() {
                "x"
            } else {
                args_str.split_whitespace().next().unwrap_or("x")
            };
            let goal = get_focused_goal(state)?;
            if let Some((lhs, rhs)) = extract_eq_sides(&goal.target, true) {
                let x_var = Expr::Const(Name::str(var_name), vec![]);
                let new_lhs = Expr::App(Box::new(lhs), Box::new(x_var.clone()));
                let new_rhs = Expr::App(Box::new(rhs), Box::new(x_var.clone()));
                let eq_target = Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Eq"), vec![])),
                            Box::new(new_lhs.clone()),
                        )),
                        Box::new(new_lhs),
                    )),
                    Box::new(new_rhs),
                );
                let mut new_goal = goal.clone();
                new_goal.target = eq_target;
                new_goal.mvar_id = fresh_mvar_id();
                new_goal.add_hypothesis(Name::str(var_name), Expr::Const(Name::str("_"), vec![]));
                return replace_focused(state, vec![new_goal]);
            }
            if matches!(&goal.target, Expr::Pi(_, _, _, _)) {
                return tactic_intro(state, Name::str(var_name));
            }
            tactic_sorry(state)
        }
        "simp_rw" => {
            let lemma_strings = parse_simp_only_lemmas(args_str);
            let lemma_refs: Vec<&str> = lemma_strings.iter().map(|s| s.as_str()).collect();
            tactic_simp(state, &lemma_refs)
        }
        "apply_fun" => {
            let goal = get_focused_goal(state)?;
            let f_name = args_str.split_whitespace().next().unwrap_or("");
            if f_name.is_empty() {
                return tactic_sorry(state);
            }
            if let Some((lhs, rhs)) = extract_eq_sides(&goal.target, true) {
                let (fl, al) = decompose_app_tactic(&lhs);
                let (fr, ar) = decompose_app_tactic(&rhs);
                let fl_name = if let Expr::Const(n, _) = fl {
                    n.to_string()
                } else {
                    String::new()
                };
                let fr_name = if let Expr::Const(n, _) = fr {
                    n.to_string()
                } else {
                    String::new()
                };
                if fl_name == f_name && fr_name == f_name && !al.is_empty() && !ar.is_empty() {
                    let a_expr = al[al.len() - 1].clone();
                    let b_expr = ar[ar.len() - 1].clone();
                    let new_eq = Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Eq"), vec![])),
                                Box::new(a_expr.clone()),
                            )),
                            Box::new(a_expr),
                        )),
                        Box::new(b_expr),
                    );
                    let mut new_goal = goal.clone();
                    new_goal.target = new_eq;
                    new_goal.mvar_id = fresh_mvar_id();
                    return replace_focused(state, vec![new_goal]);
                }
            }
            tactic_sorry(state)
        }
        "auto" | "solve_by_elim" => {
            let depth = if args_str.is_empty() {
                3usize
            } else {
                args_str.trim().parse::<usize>().unwrap_or(3)
            };
            auto_search(state, depth).or_else(|_| tactic_sorry(state))
        }
        "refine" => {
            if args_str.contains('_') || args_str.contains("?_") {
                tactic_sorry(state)
            } else {
                let expr = parse_simple_expr(args_str)
                    .unwrap_or_else(|_| Expr::Const(Name::str("_"), vec![]));
                tactic_exact(state, expr).or_else(|_| tactic_sorry(state))
            }
        }
        "next" => {
            let mut new_state = state.clone();
            new_state.rotate(1);
            Ok(new_state)
        }
        "case" => {
            let target_tag = args_str.trim();
            if target_tag.is_empty() {
                return Ok(state.clone());
            }
            let mut new_state = state.clone();
            let n = new_state.goals.len();
            for _i in 0..n {
                if let Some(tag) = &new_state.goals[0].tag {
                    if tag.contains(target_tag) {
                        return Ok(new_state);
                    }
                }
                if new_state.goals[0].name.to_string().contains(target_tag) {
                    return Ok(new_state);
                }
                new_state.rotate(1);
            }
            Ok(state.clone())
        }
        "rename_i" => {
            let names: Vec<&str> = args_str.split_whitespace().collect();
            let goal = get_focused_goal(state)?;
            let mut new_goal = goal.clone();
            new_goal.mvar_id = fresh_mvar_id();
            let mut idx = 0;
            for (n, _) in &mut new_goal.hypotheses {
                let s = n.to_string();
                if (s.starts_with('†') || s.starts_with('✝') || s.starts_with('_'))
                    && idx < names.len()
                {
                    *n = Name::str(names[idx]);
                    idx += 1;
                }
            }
            replace_focused(state, vec![new_goal])
        }
        "<;>" | "focus_all" => eval_tactic(state, &format!("all_goals {}", args_str)),
        "mod_cast" => {
            if let Ok(s) = tactic_refl(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_assumption(state) {
                return Ok(s);
            }
            if let Ok(s) = try_close_numeric(state) {
                return Ok(s);
            }
            tactic_sorry(state)
        }
        "zify" => {
            if let Ok(s) = try_linarith_with_hyps(state) {
                return Ok(s);
            }
            if let Ok(s) = try_close_numeric(state) {
                return Ok(s);
            }
            Ok(state.clone())
        }
        "squeeze_simp" => tactic_simp(state, &[]),
        "cc" | "congruence" => {
            if let Ok(s) = tactic_refl(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_assumption(state) {
                return Ok(s);
            }
            tactic_simp(state, &[]).or_else(|_| tactic_sorry(state))
        }
        "with_reducible" | "set_option" | "unhygienic" | "native_decide" => {
            if !args_str.is_empty() {
                eval_tactic(state, args_str).or_else(|_| tactic_sorry(state))
            } else {
                Ok(state.clone())
            }
        }
        "done" => {
            if state.is_complete() {
                Ok(state.clone())
            } else {
                Err(TacticError::TypeMismatch(format!(
                    "done: {} goal(s) remain",
                    state.num_goals()
                )))
            }
        }
        "assumption'" => tactic_assumption(state)
            .or_else(|_| tactic_refl(state))
            .or_else(|_| try_close_numeric(state)),
        "simp at *" => {
            let goal = get_focused_goal(state)?;
            let mut new_goal = goal.clone();
            new_goal.mvar_id = fresh_mvar_id();
            let hyp_names: Vec<Name> = new_goal.hypotheses.iter().map(|(n, _)| n.clone()).collect();
            for hyp_name in &hyp_names {
                let hyp_ty = new_goal.find_hypothesis(hyp_name).cloned();
                if let Some(ty) = hyp_ty {
                    let simplified =
                        apply_extended_simp_rules(&apply_simp_rules(&beta_reduce(&ty)));
                    if is_const_named(&simplified, "False") {
                        return replace_focused(state, vec![]);
                    }
                    if is_const_named(&simplified, "True") {
                        remove_hypothesis(&mut new_goal, hyp_name);
                    } else if simplified != ty {
                        for (n, t) in &mut new_goal.hypotheses {
                            if n == hyp_name {
                                *t = simplified.clone();
                                break;
                            }
                        }
                    }
                }
            }
            let reduced =
                apply_extended_simp_rules(&apply_simp_rules(&beta_reduce(&new_goal.target)));
            new_goal.target = reduced.clone();
            let mut new_state = state.clone();
            new_state.goals[0] = new_goal;
            if is_const_named(&reduced, "True") || is_refl_target(&reduced) {
                return replace_focused(&new_state, vec![]);
            }
            Ok(new_state)
        }
        "push_neg at *" => {
            let goal = get_focused_goal(state)?;
            let mut new_goal = goal.clone();
            new_goal.mvar_id = fresh_mvar_id();
            for (_, ty) in &mut new_goal.hypotheses {
                *ty = push_negations(ty);
            }
            let new_target = push_negations(&new_goal.target);
            new_goal.target = new_target.clone();
            let mut new_state = state.clone();
            new_state.goals[0] = new_goal;
            if is_const_named(&new_target, "True") {
                return replace_focused(&new_state, vec![]);
            }
            Ok(new_state)
        }
        "change" => {
            let target = parse_simple_expr(args_str)
                .unwrap_or_else(|_| Expr::Const(Name::str(args_str), vec![]));
            let goal = get_focused_goal(state)?;
            let mut new_goal = goal.clone();
            new_goal.target = target;
            new_goal.mvar_id = fresh_mvar_id();
            replace_focused(state, vec![new_goal])
        }
        "linear_combination" => {
            if let Ok(s) = try_ring_norm(state) {
                return Ok(s);
            }
            if let Ok(s) = try_linarith_with_hyps(state) {
                return Ok(s);
            }
            if let Ok(s) = try_close_numeric(state) {
                return Ok(s);
            }
            tactic_sorry(state)
        }
        "set" => tactic_sorry(state),
        "convert" => {
            if let Ok(s) = try_ring_norm(state) {
                return Ok(s);
            }
            if let Ok(s) = tactic_simp(state, &[]) {
                return Ok(s);
            }
            if let Ok(s) = try_linarith_with_hyps(state) {
                return Ok(s);
            }
            tactic_sorry(state)
        }
        "le_refl" | "ge_refl" | "Nat.le_refl" => {
            let goal = get_focused_goal(state)?;
            if let Some((lhs, cmp, rhs)) = parse_numeric_comparison(&goal.target) {
                if (cmp == NumCmp::Le || cmp == NumCmp::Ge) && lhs == rhs {
                    return replace_focused(state, vec![]);
                }
            }
            let (head, args) = decompose_app_tactic(&goal.target);
            if let Expr::Const(n, _) = head {
                let name = n.to_string();
                if (name == "Nat.le" || name == "LE.le" || name == "Nat.ge" || name == "GE.ge")
                    && args.len() >= 2
                    && args[args.len() - 2] == args[args.len() - 1]
                {
                    return replace_focused(state, vec![]);
                }
            }
            tactic_refl(state).or_else(|_| tactic_sorry(state))
        }
        _ => Err(TacticError::UnknownTactic(tactic_name.to_string())),
    }
}
/// Depth-limited proof search (for `auto` tactic).
fn auto_search(state: &TacticState, depth: usize) -> TacticResult {
    if state.is_complete() {
        return Ok(state.clone());
    }
    if let Ok(s) = tactic_trivial(state) {
        return Ok(s);
    }
    if let Ok(s) = tactic_refl(state) {
        return Ok(s);
    }
    if let Ok(s) = tactic_assumption(state) {
        return Ok(s);
    }
    if let Ok(s) = try_close_numeric(state) {
        return Ok(s);
    }
    if depth == 0 {
        return Err(TacticError::TypeMismatch(
            "auto: depth limit reached".to_string(),
        ));
    }
    if let Ok(s) = eval_tactic(state, "contradiction") {
        return Ok(s);
    }
    if let Ok(s) = try_ring_norm(state) {
        return Ok(s);
    }
    if let Ok(s) = try_linarith_with_hyps(state) {
        return Ok(s);
    }
    let goal = match get_focused_goal(state) {
        Ok(g) => g.clone(),
        Err(e) => return Err(e),
    };
    if matches!(&goal.target, Expr::Pi(_, _, _, _)) {
        if let Ok(after_intro) = tactic_intro(state, Name::str("h_auto")) {
            if let Ok(s) = auto_search(&after_intro, depth - 1) {
                return Ok(s);
            }
        }
    }
    match analyze_type(&goal.target) {
        TypeShape::And(_, _) | TypeShape::Iff(_, _) | TypeShape::Prod(_, _) => {
            if let Ok(after_ctor) = tactic_constructor(state) {
                if let Ok(s) = auto_search_all_goals(&after_ctor, depth - 1) {
                    return Ok(s);
                }
            }
        }
        TypeShape::Or(_, _) => {
            if let Ok(after_l) = tactic_left(state) {
                if let Ok(s) = auto_search(&after_l, depth - 1) {
                    return Ok(s);
                }
            }
            if let Ok(after_r) = tactic_right(state) {
                if let Ok(s) = auto_search(&after_r, depth - 1) {
                    return Ok(s);
                }
            }
        }
        TypeShape::False => {
            return replace_focused(state, vec![]);
        }
        _ => {}
    }
    let hyps = goal.hypotheses.clone();
    for (_, hyp_ty) in &hyps {
        if let Ok(after_apply) = tactic_apply(state, hyp_ty.clone()) {
            if let Ok(s) = auto_search_all_goals(&after_apply, depth - 1) {
                return Ok(s);
            }
        }
    }
    if depth >= 2 {
        for (hyp_name, _) in &hyps {
            if let Ok(after_cases) = tactic_cases(state, hyp_name) {
                if let Ok(s) = auto_search_all_goals(&after_cases, depth - 1) {
                    return Ok(s);
                }
            }
        }
    }
    Err(TacticError::TypeMismatch(
        "auto: failed to find proof".to_string(),
    ))
}
/// Apply auto_search to every open goal in sequence.
fn auto_search_all_goals(state: &TacticState, depth: usize) -> TacticResult {
    let mut current = state.clone();
    let num_goals = current.num_goals();
    for _ in 0..num_goals {
        if current.is_complete() {
            break;
        }
        current = auto_search(&current, depth)?;
    }
    Ok(current)
}
/// Rewrite all occurrences of `from` in `expr` with `to`.
pub(super) fn rewrite_in_expr(expr: &Expr, from: &Expr, to: &Expr) -> Expr {
    if expr == from {
        return to.clone();
    }
    match expr {
        Expr::App(f, a) => Expr::App(
            Box::new(rewrite_in_expr(f, from, to)),
            Box::new(rewrite_in_expr(a, from, to)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(rewrite_in_expr(ty, from, to)),
            Box::new(rewrite_in_expr(body, from, to)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(rewrite_in_expr(ty, from, to)),
            Box::new(rewrite_in_expr(body, from, to)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(rewrite_in_expr(ty, from, to)),
            Box::new(rewrite_in_expr(val, from, to)),
            Box::new(rewrite_in_expr(body, from, to)),
        ),
        _ => expr.clone(),
    }
}
/// Extract the two sides from an `Iff A B` expression.
pub(super) fn extract_iff_sides(expr: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(f, b) = expr {
        if let Expr::App(iff, a) = f.as_ref() {
            if is_const_named(iff, "Iff") {
                return Some((*a.clone(), *b.clone()));
            }
        }
    }
    None
}
/// Extract the lhs and rhs from an equality expression `Eq _ lhs rhs`.
///
/// If `forward` is true, returns `(lhs, rhs)`; otherwise `(rhs, lhs)`.
pub(super) fn extract_eq_sides(expr: &Expr, forward: bool) -> Option<(Expr, Expr)> {
    if let Expr::App(f1, rhs) = expr {
        if let Expr::App(f2, lhs) = f1.as_ref() {
            if let Expr::App(eq, _ty) = f2.as_ref() {
                if matches!(eq.as_ref(), Expr::Const(n, _) if n == & Name::str("Eq")) {
                    return if forward {
                        Some((lhs.as_ref().clone(), rhs.as_ref().clone()))
                    } else {
                        Some((rhs.as_ref().clone(), lhs.as_ref().clone()))
                    };
                }
            }
        }
    }
    None
}
/// Parse a very simple expression from a string (for tactic arguments).
fn parse_simple_expr(s: &str) -> Result<Expr, TacticError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(TacticError::InvalidArg(
            "expected expression, got empty string".to_string(),
        ));
    }
    if let Ok(n) = s.parse::<u64>() {
        return Ok(Expr::Lit(Literal::Nat(n)));
    }
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        return Ok(Expr::Lit(Literal::Str(s[1..s.len() - 1].to_string())));
    }
    Ok(Expr::Const(Name::str(s), vec![]))
}
/// Apply beta reduction to `expr`: `(fun x -> body) arg` → `body[arg/x]`.
pub(super) fn beta_reduce(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, arg) => {
            let f_red = beta_reduce(f);
            let arg_red = beta_reduce(arg);
            if let Expr::Lam(_, _, _, body) = &f_red {
                beta_reduce(&substitute_bvar_0(body, &arg_red))
            } else {
                Expr::App(Box::new(f_red), Box::new(arg_red))
            }
        }
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(beta_reduce(ty)),
            Box::new(beta_reduce(body)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(beta_reduce(ty)),
            Box::new(beta_reduce(body)),
        ),
        Expr::Let(n, ty, val, body) => {
            let val_red = beta_reduce(val);
            let body_red = beta_reduce(body);
            Expr::Let(
                n.clone(),
                Box::new(beta_reduce(ty)),
                Box::new(val_red),
                Box::new(body_red),
            )
        }
        _ => expr.clone(),
    }
}
/// Substitute `BVar(0)` with `replacement` in `expr` (shift other BVars down by 1).
pub(super) fn substitute_bvar_0(expr: &Expr, replacement: &Expr) -> Expr {
    subst_bvar(expr, 0, replacement)
}
pub(super) fn subst_bvar(expr: &Expr, depth: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i == depth {
                replacement.clone()
            } else if *i > depth {
                Expr::BVar(i - 1)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(subst_bvar(f, depth, replacement)),
            Box::new(subst_bvar(a, depth, replacement)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(subst_bvar(ty, depth, replacement)),
            Box::new(subst_bvar(body, depth + 1, replacement)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(subst_bvar(ty, depth, replacement)),
            Box::new(subst_bvar(body, depth + 1, replacement)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(subst_bvar(ty, depth, replacement)),
            Box::new(subst_bvar(val, depth, replacement)),
            Box::new(subst_bvar(body, depth + 1, replacement)),
        ),
        _ => expr.clone(),
    }
}
/// Lift free variables in `expr` by `amount` (for correct substitution under binders).
#[allow(dead_code)]
fn lift_bvars(expr: &Expr, amount: u32) -> Expr {
    lift_bvars_above(expr, 0, amount)
}
#[allow(dead_code)]
fn lift_bvars_above(expr: &Expr, threshold: u32, amount: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= threshold {
                Expr::BVar(i + amount)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(lift_bvars_above(f, threshold, amount)),
            Box::new(lift_bvars_above(a, threshold, amount)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(lift_bvars_above(ty, threshold, amount)),
            Box::new(lift_bvars_above(body, threshold + 1, amount)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(lift_bvars_above(ty, threshold, amount)),
            Box::new(lift_bvars_above(body, threshold + 1, amount)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(lift_bvars_above(ty, threshold, amount)),
            Box::new(lift_bvars_above(val, threshold, amount)),
            Box::new(lift_bvars_above(body, threshold + 1, amount)),
        ),
        _ => expr.clone(),
    }
}
/// Evaluate a closed Nat arithmetic expression to a concrete `u64`, if possible.
///
/// Handles: `Lit(Nat(n))`, `Nat.succ`, `Nat.add`, `Nat.mul`, `Nat.sub`,
/// `Nat.div`, `Nat.mod`, `HAdd.hAdd` (with type-class wrapper stripped),
/// `HMul.hMul` likewise.
pub(super) fn eval_nat_expr(expr: &Expr) -> Option<u64> {
    match expr {
        Expr::Lit(Literal::Nat(n)) => Some(*n),
        Expr::App(f, a) => {
            if let Expr::Const(name, _) = f.as_ref() {
                let s = name.to_string();
                if s == "Nat.succ" {
                    return eval_nat_expr(a).map(|n| n + 1);
                }
            }
            let (head, args) = decompose_app_tactic(expr);
            if let Expr::Const(name, _) = head {
                let s = name.to_string();
                match s.as_str() {
                    "Nat.add" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return Some(lhs + rhs);
                    }
                    "Nat.mul" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return Some(lhs * rhs);
                    }
                    "Nat.sub" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return Some(lhs.saturating_sub(rhs));
                    }
                    "Nat.div" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return if rhs == 0 { Some(0) } else { Some(lhs / rhs) };
                    }
                    "Nat.mod" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return if rhs == 0 { Some(lhs) } else { Some(lhs % rhs) };
                    }
                    "Nat.pow" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        if rhs <= 63 {
                            return Some(lhs.pow(rhs as u32));
                        }
                        return None;
                    }
                    "HAdd.hAdd" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return Some(lhs + rhs);
                    }
                    "HMul.hMul" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return Some(lhs * rhs);
                    }
                    "HSub.hSub" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return Some(lhs.saturating_sub(rhs));
                    }
                    "HDiv.hDiv" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return if rhs == 0 { Some(0) } else { Some(lhs / rhs) };
                    }
                    "HMod.hMod" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return if rhs == 0 { Some(lhs) } else { Some(lhs % rhs) };
                    }
                    "HPow.hPow" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        if rhs <= 63 {
                            return Some(lhs.saturating_pow(rhs as u32));
                        }
                        return None;
                    }
                    "Nat.max" | "Max.max" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return Some(lhs.max(rhs));
                    }
                    "Nat.min" | "Min.min" if args.len() >= 2 => {
                        let lhs = eval_nat_expr(args[args.len() - 2])?;
                        let rhs = eval_nat_expr(args[args.len() - 1])?;
                        return Some(lhs.min(rhs));
                    }
                    "Nat.gcd" | "GCD.gcd" if args.len() >= 2 => {
                        let a = eval_nat_expr(args[args.len() - 2])?;
                        let b = eval_nat_expr(args[args.len() - 1])?;
                        return Some(nat_gcd(a, b));
                    }
                    "Nat.lcm" if args.len() >= 2 => {
                        let a = eval_nat_expr(args[args.len() - 2])?;
                        let b = eval_nat_expr(args[args.len() - 1])?;
                        let g = nat_gcd(a, b);
                        return if g == 0 { Some(0) } else { Some(a / g * b) };
                    }
                    "Nat.factorial" | "Nat.fact" if !args.is_empty() => {
                        let n = eval_nat_expr(args[args.len() - 1])?;
                        if n > 20 {
                            return None;
                        }
                        return Some((1..=n).product());
                    }
                    "Nat.choose" | "Nat.binomial" if args.len() >= 2 => {
                        let n = eval_nat_expr(args[args.len() - 2])?;
                        let k = eval_nat_expr(args[args.len() - 1])?;
                        if k > n || n > 62 {
                            return None;
                        }
                        let k = k.min(n - k);
                        let mut result: u64 = 1;
                        for i in 0..k {
                            result = result.saturating_mul(n - i) / (i + 1);
                        }
                        return Some(result);
                    }
                    "Nat.sqrt" if !args.is_empty() => {
                        let n = eval_nat_expr(args[args.len() - 1])?;
                        return Some((n as f64).sqrt() as u64);
                    }
                    "Nat.succ_pred" if !args.is_empty() => {
                        return eval_nat_expr(args[args.len() - 1]);
                    }
                    _ => {}
                }
            }
            None
        }
        _ => None,
    }
}
/// Euclidean GCD for u64.
fn nat_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// Trial-division primality test for small concrete `n`.
/// Returns `true` iff `n` is prime.
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
/// Decompose an application spine into `(head, [arg0, arg1, …, argN])`.
pub(super) fn decompose_app_tactic(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut cur = expr;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f;
    }
    args.reverse();
    (cur, args)
}
/// Try to parse the goal as a numeric comparison `lhs OP rhs` where both sides
/// can be evaluated to concrete `u64`s.  Returns `Some((lhs_val, cmp, rhs_val))`
/// on success.
///
/// Recognised forms (after spine decomposition):
/// - `Eq _ lhs rhs`           → Eq
/// - `Not (Eq _ lhs rhs)`     → Ne
/// - `Nat.lt lhs rhs`         → Lt  (also `LT.lt _ inst lhs rhs`)
/// - `Nat.le lhs rhs`         → Le  (also `LE.le _ inst lhs rhs`)
/// - `GT.gt _ inst lhs rhs`   → Gt
/// - `GE.ge _ inst lhs rhs`   → Ge
pub(super) fn parse_numeric_comparison(target: &Expr) -> Option<(u64, NumCmp, u64)> {
    let try_binary = |args: &Vec<&Expr>| -> Option<(u64, u64)> {
        if args.len() < 2 {
            return None;
        }
        let lhs = eval_nat_expr(args[args.len() - 2])?;
        let rhs = eval_nat_expr(args[args.len() - 1])?;
        Some((lhs, rhs))
    };
    if let Expr::App(f, inner) = target {
        if let Expr::Const(n, _) = f.as_ref() {
            if n.to_string() == "Not" {
                let (h, a) = decompose_app_tactic(inner);
                if let Expr::Const(hn, _) = h {
                    if hn.to_string() == "Eq" {
                        if let Some((lhs, rhs)) = try_binary(&a) {
                            return Some((lhs, NumCmp::Ne, rhs));
                        }
                    }
                }
            }
        }
    }
    let (head, args) = decompose_app_tactic(target);
    let head_name = if let Expr::Const(n, _) = head {
        n.to_string()
    } else {
        return None;
    };
    match head_name.as_str() {
        "Eq" => {
            let (lhs, rhs) = try_binary(&args)?;
            Some((lhs, NumCmp::Eq, rhs))
        }
        "Ne" | "Nat.ne" | "BEq.ne" => {
            let (lhs, rhs) = try_binary(&args)?;
            Some((lhs, NumCmp::Ne, rhs))
        }
        "Nat.lt" | "LT.lt" => {
            let (lhs, rhs) = try_binary(&args)?;
            Some((lhs, NumCmp::Lt, rhs))
        }
        "Nat.le" | "LE.le" => {
            let (lhs, rhs) = try_binary(&args)?;
            Some((lhs, NumCmp::Le, rhs))
        }
        "Nat.gt" | "GT.gt" => {
            let (lhs, rhs) = try_binary(&args)?;
            Some((lhs, NumCmp::Gt, rhs))
        }
        "Nat.ge" | "GE.ge" => {
            let (lhs, rhs) = try_binary(&args)?;
            Some((lhs, NumCmp::Ge, rhs))
        }
        _ => None,
    }
}
