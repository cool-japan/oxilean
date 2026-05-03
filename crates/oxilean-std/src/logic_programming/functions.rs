//! Functions for Prolog-style logic programming: unification, SLD resolution, parsing.

use std::collections::HashMap;

use super::types::{
    functor_arity, LpClause, LpDatabase, LpTerm, Query, ResolutionResult, SolveConfig, Substitution,
};

// ── Variable renaming ─────────────────────────────────────────────────────────

/// Rename all variables in a clause with a unique suffix to avoid name collisions.
fn rename_clause(clause: &LpClause, stamp: usize) -> LpClause {
    let suffix = format!("_{stamp}");
    LpClause {
        head: rename_term(&clause.head, &suffix),
        body: clause
            .body
            .iter()
            .map(|t| rename_term(t, &suffix))
            .collect(),
    }
}

fn rename_term(t: &LpTerm, suffix: &str) -> LpTerm {
    match t {
        LpTerm::Var(v) => LpTerm::Var(format!("{v}{suffix}")),
        LpTerm::Atom(_) | LpTerm::Integer(_) | LpTerm::Float(_) => t.clone(),
        LpTerm::Compound { functor, args } => LpTerm::Compound {
            functor: functor.clone(),
            args: args.iter().map(|a| rename_term(a, suffix)).collect(),
        },
        LpTerm::List(items, tail) => LpTerm::List(
            items.iter().map(|a| rename_term(a, suffix)).collect(),
            tail.as_ref().map(|t| Box::new(rename_term(t, suffix))),
        ),
    }
}

// ── Occurs check ──────────────────────────────────────────────────────────────

/// Check whether variable `var` occurs in `term` under substitution `subst`.
///
/// Used to detect circular bindings (e.g., X = f(X)).
pub fn occurs_check(var: &str, term: &LpTerm, subst: &Substitution) -> bool {
    match term {
        LpTerm::Var(v) => {
            if v == var {
                return true;
            }
            match subst.lookup(v) {
                Some(t) => occurs_check(var, &t.clone(), subst),
                None => false,
            }
        }
        LpTerm::Atom(_) | LpTerm::Integer(_) | LpTerm::Float(_) => false,
        LpTerm::Compound { args, .. } => args.iter().any(|a| occurs_check(var, a, subst)),
        LpTerm::List(items, tail) => {
            items.iter().any(|a| occurs_check(var, a, subst))
                || tail.as_ref().map_or(false, |t| occurs_check(var, t, subst))
        }
    }
}

// ── Apply substitution ────────────────────────────────────────────────────────

/// Apply a substitution to a term, recursively dereferencing variables.
pub fn apply_subst(term: &LpTerm, subst: &Substitution) -> LpTerm {
    match term {
        LpTerm::Var(v) => match subst.lookup(v) {
            None => term.clone(),
            Some(t) => {
                let t2 = t.clone();
                // Avoid infinite recursion on self-referential vars
                if t2 == LpTerm::Var(v.clone()) {
                    t2
                } else {
                    apply_subst(&t2, subst)
                }
            }
        },
        LpTerm::Atom(_) | LpTerm::Integer(_) | LpTerm::Float(_) => term.clone(),
        LpTerm::Compound { functor, args } => LpTerm::Compound {
            functor: functor.clone(),
            args: args.iter().map(|a| apply_subst(a, subst)).collect(),
        },
        LpTerm::List(items, tail) => LpTerm::List(
            items.iter().map(|a| apply_subst(a, subst)).collect(),
            tail.as_ref().map(|t| Box::new(apply_subst(t, subst))),
        ),
    }
}

// ── Unification ───────────────────────────────────────────────────────────────

/// Compute the most general unifier of `t1` and `t2` under `subst`.
///
/// Returns `Some(new_subst)` on success, `None` on failure.
pub fn unify(t1: &LpTerm, t2: &LpTerm, subst: &Substitution) -> Option<Substitution> {
    let t1 = apply_subst(t1, subst);
    let t2 = apply_subst(t2, subst);
    unify_walked(&t1, &t2, subst)
}

fn unify_walked(t1: &LpTerm, t2: &LpTerm, subst: &Substitution) -> Option<Substitution> {
    match (t1, t2) {
        // Two identical atoms/ints/floats
        (LpTerm::Atom(a), LpTerm::Atom(b)) if a == b => Some(subst.clone()),
        (LpTerm::Integer(a), LpTerm::Integer(b)) if a == b => Some(subst.clone()),
        (LpTerm::Float(a), LpTerm::Float(b)) if a == b => Some(subst.clone()),

        // Variable on left
        (LpTerm::Var(v), t) => {
            let t = apply_subst(t, subst);
            if let LpTerm::Var(v2) = &t {
                if v == v2 {
                    return Some(subst.clone());
                }
            }
            // No occurs check by default; check only if needed
            let mut new_subst = subst.clone();
            new_subst.bind(v.clone(), t);
            Some(new_subst)
        }

        // Variable on right
        (t, LpTerm::Var(v)) => {
            let t = apply_subst(t, subst);
            let mut new_subst = subst.clone();
            new_subst.bind(v.clone(), t);
            Some(new_subst)
        }

        // Compound terms
        (
            LpTerm::Compound {
                functor: f1,
                args: a1,
            },
            LpTerm::Compound {
                functor: f2,
                args: a2,
            },
        ) => {
            if f1 != f2 || a1.len() != a2.len() {
                return None;
            }
            let mut s = subst.clone();
            for (x, y) in a1.iter().zip(a2.iter()) {
                s = unify(x, y, &s)?;
            }
            Some(s)
        }

        // Lists
        (LpTerm::List(items1, tail1), LpTerm::List(items2, tail2)) => {
            unify_lists(items1, tail1.as_deref(), items2, tail2.as_deref(), subst)
        }

        // Atom as nil vs empty list
        (LpTerm::Atom(a), LpTerm::List(items, None)) if a == "[]" && items.is_empty() => {
            Some(subst.clone())
        }
        (LpTerm::List(items, None), LpTerm::Atom(a)) if a == "[]" && items.is_empty() => {
            Some(subst.clone())
        }

        _ => None,
    }
}

fn unify_lists(
    items1: &[LpTerm],
    tail1: Option<&LpTerm>,
    items2: &[LpTerm],
    tail2: Option<&LpTerm>,
    subst: &Substitution,
) -> Option<Substitution> {
    match (items1, items2) {
        ([], []) => {
            // Both exhausted — unify the tails
            match (tail1, tail2) {
                (None, None) => Some(subst.clone()),
                (Some(t1), Some(t2)) => unify(t1, t2, subst),
                (None, Some(t)) => unify(&LpTerm::atom("[]"), t, subst),
                (Some(t), None) => unify(t, &LpTerm::atom("[]"), subst),
            }
        }
        ([], _) => {
            // items1 exhausted; items2 has remaining — tail1 must unify with remaining list
            let rest = LpTerm::List(items2.to_vec(), tail2.cloned().map(|t| Box::new(t.clone())));
            match tail1 {
                None => None, // proper list vs longer list
                Some(t) => unify(t, &rest, subst),
            }
        }
        (_, []) => {
            // items2 exhausted; items1 has remaining — tail2 must unify with remaining list
            let rest = LpTerm::List(items1.to_vec(), tail1.cloned().map(|t| Box::new(t.clone())));
            match tail2 {
                None => None,
                Some(t) => unify(&rest, t, subst),
            }
        }
        ([h1, rest1 @ ..], [h2, rest2 @ ..]) => {
            let s = unify(h1, h2, subst)?;
            unify_lists(rest1, tail1, rest2, tail2, &s)
        }
    }
}

// ── Unification with occurs check ─────────────────────────────────────────────

/// Unify with the occurs check enabled (sound but slower).
pub fn unify_with_occurs_check(
    t1: &LpTerm,
    t2: &LpTerm,
    subst: &Substitution,
) -> Option<Substitution> {
    let t1w = apply_subst(t1, subst);
    let t2w = apply_subst(t2, subst);
    unify_oc_walked(&t1w, &t2w, subst)
}

fn unify_oc_walked(t1: &LpTerm, t2: &LpTerm, subst: &Substitution) -> Option<Substitution> {
    match (t1, t2) {
        (LpTerm::Atom(a), LpTerm::Atom(b)) if a == b => Some(subst.clone()),
        (LpTerm::Integer(a), LpTerm::Integer(b)) if a == b => Some(subst.clone()),
        (LpTerm::Float(a), LpTerm::Float(b)) if a == b => Some(subst.clone()),

        (LpTerm::Var(v), t) => {
            let t = apply_subst(t, subst);
            if let LpTerm::Var(v2) = &t {
                if v == v2 {
                    return Some(subst.clone());
                }
            }
            if occurs_check(v, &t, subst) {
                return None;
            }
            let mut s = subst.clone();
            s.bind(v.clone(), t);
            Some(s)
        }

        (t, LpTerm::Var(v)) => {
            let t = apply_subst(t, subst);
            if occurs_check(v, &t, subst) {
                return None;
            }
            let mut s = subst.clone();
            s.bind(v.clone(), t);
            Some(s)
        }

        (
            LpTerm::Compound {
                functor: f1,
                args: a1,
            },
            LpTerm::Compound {
                functor: f2,
                args: a2,
            },
        ) => {
            if f1 != f2 || a1.len() != a2.len() {
                return None;
            }
            let mut s = subst.clone();
            for (x, y) in a1.iter().zip(a2.iter()) {
                s = unify_with_occurs_check(x, y, &s)?;
            }
            Some(s)
        }

        (LpTerm::List(i1, t1), LpTerm::List(i2, t2)) => {
            unify_lists(i1, t1.as_deref(), i2, t2.as_deref(), subst)
        }

        (LpTerm::Atom(a), LpTerm::List(items, None)) if a == "[]" && items.is_empty() => {
            Some(subst.clone())
        }
        (LpTerm::List(items, None), LpTerm::Atom(a)) if a == "[]" && items.is_empty() => {
            Some(subst.clone())
        }

        _ => None,
    }
}

// ── SLD Resolution ────────────────────────────────────────────────────────────

/// Collect all solutions to a query using SLD resolution.
pub fn resolve(query: &Query, db: &LpDatabase, cfg: &SolveConfig) -> Vec<Substitution> {
    let mut results = Vec::new();
    let mut counter = 0usize;
    sld_resolve(
        query.goals.clone(),
        &Substitution::new(),
        db,
        cfg,
        0,
        &mut counter,
        &mut results,
    );
    results
}

/// Solve the query, returning the first solution or failure.
pub fn solve_one(query: &Query, db: &LpDatabase, cfg: &SolveConfig) -> ResolutionResult {
    let mut results = Vec::new();
    let mut counter = 0usize;
    let one_cfg = SolveConfig {
        max_solutions: 1,
        ..cfg.clone()
    };
    sld_resolve(
        query.goals.clone(),
        &Substitution::new(),
        db,
        &one_cfg,
        0,
        &mut counter,
        &mut results,
    );
    match results.into_iter().next() {
        Some(s) => ResolutionResult::Success(s),
        None => ResolutionResult::Failure,
    }
}

fn sld_resolve(
    goals: Vec<LpTerm>,
    subst: &Substitution,
    db: &LpDatabase,
    cfg: &SolveConfig,
    depth: usize,
    counter: &mut usize,
    results: &mut Vec<Substitution>,
) {
    if results.len() >= cfg.max_solutions {
        return;
    }
    if depth > cfg.max_depth {
        return;
    }

    if goals.is_empty() {
        results.push(subst.clone());
        return;
    }

    let goal = apply_subst(&goals[0], subst);
    let rest_goals = goals[1..].to_vec();

    // Built-in predicates
    if handle_builtin(&goal, subst, db, cfg, depth, counter, results, &rest_goals) {
        return;
    }

    // User-defined predicates
    let matching: Vec<LpClause> = db.matching_clauses(&goal).into_iter().cloned().collect();

    for clause in &matching {
        if results.len() >= cfg.max_solutions {
            break;
        }
        *counter += 1;
        let renamed = rename_clause(clause, *counter);
        let unifier = if cfg.occurs_check {
            unify_with_occurs_check(&goal, &renamed.head, subst)
        } else {
            unify(&goal, &renamed.head, subst)
        };
        if let Some(new_subst) = unifier {
            let mut new_goals = renamed.body.clone();
            new_goals.extend(rest_goals.clone());
            sld_resolve(new_goals, &new_subst, db, cfg, depth + 1, counter, results);
        }
    }
}

/// Handle built-in predicates. Returns true if the goal was handled (even if it failed).
fn handle_builtin(
    goal: &LpTerm,
    subst: &Substitution,
    db: &LpDatabase,
    cfg: &SolveConfig,
    depth: usize,
    counter: &mut usize,
    results: &mut Vec<Substitution>,
    rest_goals: &[LpTerm],
) -> bool {
    match goal {
        // true/0 — always succeeds
        LpTerm::Atom(a) if a == "true" => {
            sld_resolve(
                rest_goals.to_vec(),
                subst,
                db,
                cfg,
                depth + 1,
                counter,
                results,
            );
            true
        }
        // fail/0 — always fails
        LpTerm::Atom(a) if a == "fail" || a == "false" => true,
        // =(X, Y) — unification
        LpTerm::Compound { functor, args } if functor == "=" && args.len() == 2 => {
            if let Some(s) = unify(&args[0], &args[1], subst) {
                sld_resolve(
                    rest_goals.to_vec(),
                    &s,
                    db,
                    cfg,
                    depth + 1,
                    counter,
                    results,
                );
            }
            true
        }
        // \=(X, Y) — negation of unification
        LpTerm::Compound { functor, args } if functor == "\\=" && args.len() == 2 => {
            if unify(&args[0], &args[1], subst).is_none() {
                sld_resolve(
                    rest_goals.to_vec(),
                    subst,
                    db,
                    cfg,
                    depth + 1,
                    counter,
                    results,
                );
            }
            true
        }
        // is/2 — arithmetic evaluation (limited)
        LpTerm::Compound { functor, args } if functor == "is" && args.len() == 2 => {
            if let Some(val) = eval_arith(&args[1], subst) {
                if let Some(s) = unify(&args[0], &val, subst) {
                    sld_resolve(
                        rest_goals.to_vec(),
                        &s,
                        db,
                        cfg,
                        depth + 1,
                        counter,
                        results,
                    );
                }
            }
            true
        }
        // =:=/2 — arithmetic equality
        LpTerm::Compound { functor, args } if functor == "=:=" && args.len() == 2 => {
            let v1 = eval_arith(&args[0], subst);
            let v2 = eval_arith(&args[1], subst);
            if v1 == v2 && v1.is_some() {
                sld_resolve(
                    rest_goals.to_vec(),
                    subst,
                    db,
                    cfg,
                    depth + 1,
                    counter,
                    results,
                );
            }
            true
        }
        // </2, >/2, =</2, >=/2 — arithmetic comparison
        LpTerm::Compound { functor, args }
            if (functor == "<" || functor == ">" || functor == "=<" || functor == ">=")
                && args.len() == 2 =>
        {
            let v1 = eval_arith_f64(&args[0], subst);
            let v2 = eval_arith_f64(&args[1], subst);
            let ok = match (v1, v2) {
                (Some(a), Some(b)) => match functor.as_str() {
                    "<" => a < b,
                    ">" => a > b,
                    "=<" => a <= b,
                    ">=" => a >= b,
                    _ => false,
                },
                _ => false,
            };
            if ok {
                sld_resolve(
                    rest_goals.to_vec(),
                    subst,
                    db,
                    cfg,
                    depth + 1,
                    counter,
                    results,
                );
            }
            true
        }
        // not/1, \+/1 — negation as failure
        LpTerm::Compound { functor, args }
            if (functor == "not" || functor == "\\+") && args.len() == 1 =>
        {
            let inner_q = Query::single(args[0].clone());
            let inner_cfg = SolveConfig {
                max_solutions: 1,
                ..cfg.clone()
            };
            let inner_results = resolve(&inner_q, db, &inner_cfg);
            if inner_results.is_empty() {
                sld_resolve(
                    rest_goals.to_vec(),
                    subst,
                    db,
                    cfg,
                    depth + 1,
                    counter,
                    results,
                );
            }
            true
        }
        // call/1 — meta-call
        LpTerm::Compound { functor, args } if functor == "call" && args.len() == 1 => {
            let new_goal = apply_subst(&args[0], subst);
            let mut new_goals = vec![new_goal];
            new_goals.extend(rest_goals.to_vec());
            sld_resolve(new_goals, subst, db, cfg, depth + 1, counter, results);
            true
        }
        _ => false,
    }
}

/// Evaluate an arithmetic expression to an integer term.
fn eval_arith(t: &LpTerm, subst: &Substitution) -> Option<LpTerm> {
    let t = apply_subst(t, subst);
    match &t {
        LpTerm::Integer(n) => Some(LpTerm::Integer(*n)),
        LpTerm::Float(f) => Some(LpTerm::Float(*f)),
        LpTerm::Compound { functor, args } if args.len() == 2 => {
            let a = eval_arith_f64(&args[0], subst)?;
            let b = eval_arith_f64(&args[1], subst)?;
            let result = match functor.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        return None;
                    }
                    a / b
                }
                "mod" => {
                    if b == 0.0 {
                        return None;
                    }
                    a % b
                }
                "**" | "^" => a.powf(b),
                _ => return None,
            };
            // Return integer if both operands were integers and result is whole
            if result.fract() == 0.0 && functor != "/" {
                Some(LpTerm::Integer(result as i64))
            } else {
                Some(LpTerm::Float(result))
            }
        }
        LpTerm::Compound { functor, args } if args.len() == 1 => {
            let a = eval_arith_f64(&args[0], subst)?;
            let result = match functor.as_str() {
                "abs" => a.abs(),
                "sqrt" => a.sqrt(),
                "floor" => a.floor(),
                "ceiling" => a.ceil(),
                "round" => a.round(),
                "-" => -a,
                _ => return None,
            };
            if result.fract() == 0.0 {
                Some(LpTerm::Integer(result as i64))
            } else {
                Some(LpTerm::Float(result))
            }
        }
        _ => None,
    }
}

fn eval_arith_f64(t: &LpTerm, subst: &Substitution) -> Option<f64> {
    match eval_arith(t, subst)? {
        LpTerm::Integer(n) => Some(n as f64),
        LpTerm::Float(f) => Some(f),
        _ => None,
    }
}

// ── LpDatabase methods (query_all) ────────────────────────────────────────────

impl LpDatabase {
    /// Collect all solutions to a single-goal query.
    pub fn query_all(&self, goal: LpTerm, cfg: &SolveConfig) -> Vec<Substitution> {
        let q = Query::single(goal);
        resolve(&q, self, cfg)
    }
}

// ── Term pretty-printing ──────────────────────────────────────────────────────

/// Pretty-print a term to a Prolog-style string.
pub fn term_to_string(t: &LpTerm) -> String {
    match t {
        LpTerm::Atom(s) => {
            // Quote atoms that need it
            if needs_quoting(s) {
                format!("'{}'", s.replace('\'', "\\'"))
            } else {
                s.clone()
            }
        }
        LpTerm::Var(v) => v.clone(),
        LpTerm::Integer(n) => n.to_string(),
        LpTerm::Float(f) => format!("{f}"),
        LpTerm::Compound { functor, args } => {
            let args_str: Vec<String> = args.iter().map(term_to_string).collect();
            format!("{}({})", functor, args_str.join(","))
        }
        LpTerm::List(items, tail) => {
            let items_str: Vec<String> = items.iter().map(term_to_string).collect();
            let body = items_str.join(",");
            match tail {
                None => format!("[{body}]"),
                Some(t) => format!("[{body}|{}]", term_to_string(t)),
            }
        }
    }
}

fn needs_quoting(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    let mut chars = s.chars();
    let first = match chars.next() {
        Some(c) => c,
        None => return true,
    };
    // Atoms starting with lowercase letter and containing only alnum/_
    if first.is_ascii_lowercase() && s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return false;
    }
    // Operators don't need quoting
    if s.chars().all(|c| "+-*/\\^<>=~:.?@#&".contains(c)) {
        return false;
    }
    // Special atoms
    matches!(s, "[]" | "{}" | "!" | ";" | "," | "|")
        || s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

// ── Simple term parser ────────────────────────────────────────────────────────

/// Parse a simple Prolog term from a string.
///
/// Supports atoms, variables, integers, floats, compound terms, and lists.
/// Does not support full operator syntax.
pub fn parse_term(s: &str) -> Option<LpTerm> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    parse_term_inner(s)
}

fn parse_term_inner(s: &str) -> Option<LpTerm> {
    let s = s.trim();

    // List
    if s.starts_with('[') && s.ends_with(']') {
        return parse_list(&s[1..s.len() - 1]);
    }

    // Quoted atom
    if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 {
        return Some(LpTerm::Atom(s[1..s.len() - 1].replace("\\'", "'")));
    }

    // Check for compound: find the outermost '('
    if let Some(paren_pos) = find_outer_paren(s) {
        let functor = s[..paren_pos].trim().to_string();
        let args_str = &s[paren_pos + 1..s.len() - 1];
        let args = split_args(args_str)
            .into_iter()
            .map(|a| parse_term_inner(a.trim()))
            .collect::<Option<Vec<_>>>()?;
        return Some(LpTerm::Compound { functor, args });
    }

    // Integer
    if let Ok(n) = s.parse::<i64>() {
        return Some(LpTerm::Integer(n));
    }

    // Float
    if let Ok(f) = s.parse::<f64>() {
        return Some(LpTerm::Float(f));
    }

    // Variable: starts with uppercase or '_'
    let first = s.chars().next()?;
    if first.is_uppercase() || first == '_' {
        return Some(LpTerm::Var(s.to_string()));
    }

    // Atom
    Some(LpTerm::Atom(s.to_string()))
}

fn parse_list(inner: &str) -> Option<LpTerm> {
    let inner = inner.trim();
    if inner.is_empty() {
        return Some(LpTerm::atom("[]"));
    }

    // Find '|' at depth 0
    let mut depth = 0i32;
    let mut bar_pos = None;
    let bytes = inner.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b'|' if depth == 0 => {
                bar_pos = Some(i);
                break;
            }
            _ => {}
        }
    }

    if let Some(pos) = bar_pos {
        let items_str = &inner[..pos];
        let tail_str = inner[pos + 1..].trim();
        let items = split_args(items_str)
            .into_iter()
            .map(|a| parse_term_inner(a.trim()))
            .collect::<Option<Vec<_>>>()?;
        let tail = parse_term_inner(tail_str)?;
        Some(LpTerm::List(items, Some(Box::new(tail))))
    } else {
        let items = split_args(inner)
            .into_iter()
            .map(|a| parse_term_inner(a.trim()))
            .collect::<Option<Vec<_>>>()?;
        Some(LpTerm::list(items))
    }
}

/// Find the position of the outermost '(' in a compound term like `f(...)`.
fn find_outer_paren(s: &str) -> Option<usize> {
    let mut depth = 0i32;
    for (i, c) in s.char_indices() {
        match c {
            '(' if depth == 0 => {
                // Must be preceded by functor name
                if i > 0 && s.ends_with(')') {
                    return Some(i);
                }
                return None;
            }
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
    }
    None
}

/// Split a comma-separated argument list, respecting parentheses and brackets.
fn split_args(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b',' if depth == 0 => {
                parts.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    if start <= s.len() {
        let tail = s[start..].trim();
        if !tail.is_empty() {
            parts.push(&s[start..]);
        }
    }
    parts
}

/// Parse a Horn clause from a string: `head :- b1, b2.` or `head.`
pub fn parse_clause(s: &str) -> Option<LpClause> {
    let s = s.trim().trim_end_matches('.');
    if let Some(pos) = s.find(":-") {
        let head_str = s[..pos].trim();
        let body_str = s[pos + 2..].trim();
        let head = parse_term(head_str)?;
        let body = split_args(body_str)
            .into_iter()
            .map(|a| parse_term(a.trim()))
            .collect::<Option<Vec<_>>>()?;
        Some(LpClause::rule(head, body))
    } else {
        let head = parse_term(s)?;
        Some(LpClause::fact(head))
    }
}

// ── Classic Prolog library predicates ────────────────────────────────────────

/// Populate a database with classic Prolog predicates: member/2, append/3, reverse/3, length/2, last/2.
pub fn load_standard_predicates(db: &mut LpDatabase) {
    // member(X, [X|_]).
    db.add_fact(LpTerm::compound(
        "member",
        vec![
            LpTerm::var("X"),
            LpTerm::list_with_tail(vec![LpTerm::var("X")], LpTerm::var("_T")),
        ],
    ));
    // member(X, [_|T]) :- member(X, T).
    db.add_rule(
        LpTerm::compound(
            "member",
            vec![
                LpTerm::var("X"),
                LpTerm::list_with_tail(vec![LpTerm::var("_H")], LpTerm::var("T")),
            ],
        ),
        vec![LpTerm::compound(
            "member",
            vec![LpTerm::var("X"), LpTerm::var("T")],
        )],
    );

    // append([], L, L).
    db.add_fact(LpTerm::compound(
        "append",
        vec![LpTerm::atom("[]"), LpTerm::var("L"), LpTerm::var("L")],
    ));
    // append([H|T], L, [H|R]) :- append(T, L, R).
    db.add_rule(
        LpTerm::compound(
            "append",
            vec![
                LpTerm::list_with_tail(vec![LpTerm::var("H")], LpTerm::var("T")),
                LpTerm::var("L"),
                LpTerm::list_with_tail(vec![LpTerm::var("H")], LpTerm::var("R")),
            ],
        ),
        vec![LpTerm::compound(
            "append",
            vec![LpTerm::var("T"), LpTerm::var("L"), LpTerm::var("R")],
        )],
    );

    // reverse([], Acc, Acc).
    db.add_fact(LpTerm::compound(
        "reverse_acc",
        vec![LpTerm::atom("[]"), LpTerm::var("Acc"), LpTerm::var("Acc")],
    ));
    // reverse_acc([H|T], Acc, Rev) :- reverse_acc(T, [H|Acc], Rev).
    db.add_rule(
        LpTerm::compound(
            "reverse_acc",
            vec![
                LpTerm::list_with_tail(vec![LpTerm::var("H")], LpTerm::var("T")),
                LpTerm::var("Acc"),
                LpTerm::var("Rev"),
            ],
        ),
        vec![LpTerm::compound(
            "reverse_acc",
            vec![
                LpTerm::var("T"),
                LpTerm::list_with_tail(vec![LpTerm::var("H")], LpTerm::var("Acc")),
                LpTerm::var("Rev"),
            ],
        )],
    );
    // reverse(L, R) :- reverse_acc(L, [], R).
    db.add_rule(
        LpTerm::compound("reverse", vec![LpTerm::var("L"), LpTerm::var("R")]),
        vec![LpTerm::compound(
            "reverse_acc",
            vec![LpTerm::var("L"), LpTerm::atom("[]"), LpTerm::var("R")],
        )],
    );

    // length([], 0).
    db.add_fact(LpTerm::compound(
        "length",
        vec![LpTerm::atom("[]"), LpTerm::Integer(0)],
    ));
    // length([_|T], N) :- length(T, N1), N is N1 + 1.
    db.add_rule(
        LpTerm::compound(
            "length",
            vec![
                LpTerm::list_with_tail(vec![LpTerm::var("_H2")], LpTerm::var("T2")),
                LpTerm::var("N"),
            ],
        ),
        vec![
            LpTerm::compound("length", vec![LpTerm::var("T2"), LpTerm::var("N1")]),
            LpTerm::compound(
                "is",
                vec![
                    LpTerm::var("N"),
                    LpTerm::compound("+", vec![LpTerm::var("N1"), LpTerm::Integer(1)]),
                ],
            ),
        ],
    );

    // last([X], X).
    db.add_fact(LpTerm::compound(
        "last",
        vec![LpTerm::list(vec![LpTerm::var("X")]), LpTerm::var("X")],
    ));
    // last([_|T], X) :- last(T, X).
    db.add_rule(
        LpTerm::compound(
            "last",
            vec![
                LpTerm::list_with_tail(vec![LpTerm::var("_HL")], LpTerm::var("TL")),
                LpTerm::var("XL"),
            ],
        ),
        vec![LpTerm::compound(
            "last",
            vec![LpTerm::var("TL"), LpTerm::var("XL")],
        )],
    );

    // nat/1 — natural number generator (bounded by depth)
    // nat(0).
    db.add_fact(LpTerm::compound("nat", vec![LpTerm::Integer(0)]));
    // nat(N) :- nat(N1), N is N1 + 1.
    db.add_rule(
        LpTerm::compound("nat", vec![LpTerm::var("N")]),
        vec![
            LpTerm::compound("nat", vec![LpTerm::var("N1")]),
            LpTerm::compound(
                "is",
                vec![
                    LpTerm::var("N"),
                    LpTerm::compound("+", vec![LpTerm::var("N1"), LpTerm::Integer(1)]),
                ],
            ),
        ],
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_subst() -> Substitution {
        Substitution::new()
    }

    /// Flatten a (possibly nested) list representation into a Vec of elements.
    fn flatten_list(t: &LpTerm) -> Vec<LpTerm> {
        let mut result = Vec::new();
        flatten_list_into(t, &mut result);
        result
    }

    fn flatten_list_into(t: &LpTerm, out: &mut Vec<LpTerm>) {
        match t {
            LpTerm::Atom(a) if a == "[]" => {}
            LpTerm::List(items, tail) => {
                for item in items {
                    out.push(item.clone());
                }
                if let Some(tl) = tail {
                    flatten_list_into(tl, out);
                }
            }
            _ => out.push(t.clone()),
        }
    }

    fn default_cfg() -> SolveConfig {
        SolveConfig::default()
    }

    fn std_db() -> LpDatabase {
        let mut db = LpDatabase::new();
        load_standard_predicates(&mut db);
        db
    }

    // ── Unification tests ────────────────────────────────────────────────────

    #[test]
    fn test_unify_atoms_equal() {
        let s = unify(&LpTerm::atom("foo"), &LpTerm::atom("foo"), &empty_subst());
        assert!(s.is_some());
    }

    #[test]
    fn test_unify_atoms_different() {
        let s = unify(&LpTerm::atom("foo"), &LpTerm::atom("bar"), &empty_subst());
        assert!(s.is_none());
    }

    #[test]
    fn test_unify_var_atom() {
        let s = unify(&LpTerm::var("X"), &LpTerm::atom("hello"), &empty_subst());
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.lookup("X"), Some(&LpTerm::atom("hello")));
    }

    #[test]
    fn test_unify_compound() {
        let t1 = LpTerm::compound("f", vec![LpTerm::var("X"), LpTerm::Integer(1)]);
        let t2 = LpTerm::compound("f", vec![LpTerm::atom("a"), LpTerm::Integer(1)]);
        let s = unify(&t1, &t2, &empty_subst());
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.lookup("X"), Some(&LpTerm::atom("a")));
    }

    #[test]
    fn test_unify_compound_arity_mismatch() {
        let t1 = LpTerm::compound("f", vec![LpTerm::var("X")]);
        let t2 = LpTerm::compound("f", vec![LpTerm::var("X"), LpTerm::var("Y")]);
        assert!(unify(&t1, &t2, &empty_subst()).is_none());
    }

    #[test]
    fn test_unify_list() {
        let t1 = LpTerm::list(vec![LpTerm::var("X"), LpTerm::Integer(2)]);
        let t2 = LpTerm::list(vec![LpTerm::Integer(1), LpTerm::Integer(2)]);
        let s = unify(&t1, &t2, &empty_subst());
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(apply_subst(&LpTerm::var("X"), &s), LpTerm::Integer(1));
    }

    #[test]
    fn test_unify_list_different_length() {
        let t1 = LpTerm::list(vec![LpTerm::Integer(1)]);
        let t2 = LpTerm::list(vec![LpTerm::Integer(1), LpTerm::Integer(2)]);
        assert!(unify(&t1, &t2, &empty_subst()).is_none());
    }

    #[test]
    fn test_unify_integers() {
        let s = unify(&LpTerm::Integer(42), &LpTerm::Integer(42), &empty_subst());
        assert!(s.is_some());
        let s = unify(&LpTerm::Integer(1), &LpTerm::Integer(2), &empty_subst());
        assert!(s.is_none());
    }

    // ── Apply substitution tests ─────────────────────────────────────────────

    #[test]
    fn test_apply_subst_var() {
        let mut s = Substitution::new();
        s.bind("X", LpTerm::atom("hello"));
        assert_eq!(apply_subst(&LpTerm::var("X"), &s), LpTerm::atom("hello"));
    }

    #[test]
    fn test_apply_subst_compound() {
        let mut s = Substitution::new();
        s.bind("X", LpTerm::Integer(5));
        let t = LpTerm::compound("f", vec![LpTerm::var("X"), LpTerm::Integer(1)]);
        let result = apply_subst(&t, &s);
        assert_eq!(
            result,
            LpTerm::compound("f", vec![LpTerm::Integer(5), LpTerm::Integer(1)])
        );
    }

    // ── Occurs check tests ───────────────────────────────────────────────────

    #[test]
    fn test_occurs_check_direct() {
        let s = empty_subst();
        assert!(occurs_check("X", &LpTerm::var("X"), &s));
    }

    #[test]
    fn test_occurs_check_in_compound() {
        let s = empty_subst();
        let t = LpTerm::compound("f", vec![LpTerm::var("X")]);
        assert!(occurs_check("X", &t, &s));
    }

    #[test]
    fn test_occurs_check_not_present() {
        let s = empty_subst();
        let t = LpTerm::compound("f", vec![LpTerm::var("Y")]);
        assert!(!occurs_check("X", &t, &s));
    }

    #[test]
    fn test_occurs_check_prevents_circular() {
        let s = empty_subst();
        let t = LpTerm::compound("f", vec![LpTerm::var("X")]);
        let result = unify_with_occurs_check(&LpTerm::var("X"), &t, &s);
        assert!(result.is_none());
    }

    // ── Resolution: member/2 ─────────────────────────────────────────────────

    #[test]
    fn test_member_first() {
        let db = std_db();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "member",
            vec![
                LpTerm::Integer(1),
                LpTerm::list(vec![
                    LpTerm::Integer(1),
                    LpTerm::Integer(2),
                    LpTerm::Integer(3),
                ]),
            ],
        ));
        let results = resolve(&q, &db, &cfg);
        assert!(!results.is_empty(), "member(1, [1,2,3]) should succeed");
    }

    #[test]
    fn test_member_middle() {
        let db = std_db();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "member",
            vec![
                LpTerm::Integer(2),
                LpTerm::list(vec![
                    LpTerm::Integer(1),
                    LpTerm::Integer(2),
                    LpTerm::Integer(3),
                ]),
            ],
        ));
        let results = resolve(&q, &db, &cfg);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_member_not_found() {
        let db = std_db();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "member",
            vec![
                LpTerm::Integer(99),
                LpTerm::list(vec![LpTerm::Integer(1), LpTerm::Integer(2)]),
            ],
        ));
        let results = resolve(&q, &db, &cfg);
        assert!(results.is_empty());
    }

    #[test]
    fn test_member_enumerate() {
        let db = std_db();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "member",
            vec![
                LpTerm::var("X"),
                LpTerm::list(vec![
                    LpTerm::atom("a"),
                    LpTerm::atom("b"),
                    LpTerm::atom("c"),
                ]),
            ],
        ));
        let results = resolve(&q, &db, &cfg);
        assert_eq!(results.len(), 3, "Should enumerate all 3 members");
    }

    // ── Resolution: append/3 ─────────────────────────────────────────────────

    #[test]
    fn test_append_concrete() {
        let db = std_db();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "append",
            vec![
                LpTerm::list(vec![LpTerm::Integer(1), LpTerm::Integer(2)]),
                LpTerm::list(vec![LpTerm::Integer(3)]),
                LpTerm::var("R"),
            ],
        ));
        let results = resolve(&q, &db, &cfg);
        assert_eq!(results.len(), 1);
        let r = apply_subst(&LpTerm::var("R"), &results[0]);
        let flat = flatten_list(&r);
        assert_eq!(
            flat,
            vec![LpTerm::Integer(1), LpTerm::Integer(2), LpTerm::Integer(3)]
        );
    }

    #[test]
    fn test_append_split() {
        // append(X, Y, [1,2]) — find all splits
        let db = std_db();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "append",
            vec![
                LpTerm::var("X"),
                LpTerm::var("Y"),
                LpTerm::list(vec![LpTerm::Integer(1), LpTerm::Integer(2)]),
            ],
        ));
        let results = resolve(&q, &db, &cfg);
        // Should find: ([], [1,2]), ([1], [2]), ([1,2], [])
        assert_eq!(results.len(), 3);
    }

    // ── Resolution: reverse/2 ────────────────────────────────────────────────

    #[test]
    fn test_reverse() {
        let db = std_db();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "reverse",
            vec![
                LpTerm::list(vec![
                    LpTerm::Integer(1),
                    LpTerm::Integer(2),
                    LpTerm::Integer(3),
                ]),
                LpTerm::var("R"),
            ],
        ));
        let results = resolve(&q, &db, &cfg);
        assert_eq!(results.len(), 1);
        let r = apply_subst(&LpTerm::var("R"), &results[0]);
        let flat = flatten_list(&r);
        assert_eq!(
            flat,
            vec![LpTerm::Integer(3), LpTerm::Integer(2), LpTerm::Integer(1)]
        );
    }

    // ── Built-in: true/fail ──────────────────────────────────────────────────

    #[test]
    fn test_builtin_true() {
        let db = LpDatabase::new();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::atom("true"));
        let results = resolve(&q, &db, &cfg);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_builtin_fail() {
        let db = LpDatabase::new();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::atom("fail"));
        let results = resolve(&q, &db, &cfg);
        assert!(results.is_empty());
    }

    // ── Built-in: =/2 unification ────────────────────────────────────────────

    #[test]
    fn test_builtin_unify() {
        let db = LpDatabase::new();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "=",
            vec![LpTerm::var("X"), LpTerm::Integer(42)],
        ));
        let results = resolve(&q, &db, &cfg);
        assert_eq!(results.len(), 1);
        let val = apply_subst(&LpTerm::var("X"), &results[0]);
        assert_eq!(val, LpTerm::Integer(42));
    }

    // ── Built-in: is/2 arithmetic ────────────────────────────────────────────

    #[test]
    fn test_builtin_is_add() {
        let db = LpDatabase::new();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "is",
            vec![
                LpTerm::var("X"),
                LpTerm::compound("+", vec![LpTerm::Integer(3), LpTerm::Integer(4)]),
            ],
        ));
        let results = resolve(&q, &db, &cfg);
        assert_eq!(results.len(), 1);
        let val = apply_subst(&LpTerm::var("X"), &results[0]);
        assert_eq!(val, LpTerm::Integer(7));
    }

    #[test]
    fn test_builtin_is_mul() {
        let db = LpDatabase::new();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "is",
            vec![
                LpTerm::var("X"),
                LpTerm::compound("*", vec![LpTerm::Integer(6), LpTerm::Integer(7)]),
            ],
        ));
        let results = resolve(&q, &db, &cfg);
        assert_eq!(results.len(), 1);
        let val = apply_subst(&LpTerm::var("X"), &results[0]);
        assert_eq!(val, LpTerm::Integer(42));
    }

    // ── Built-in: comparison ─────────────────────────────────────────────────

    #[test]
    fn test_builtin_less_than() {
        let db = LpDatabase::new();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "<",
            vec![LpTerm::Integer(3), LpTerm::Integer(5)],
        ));
        let results = resolve(&q, &db, &cfg);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_builtin_less_than_false() {
        let db = LpDatabase::new();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "<",
            vec![LpTerm::Integer(5), LpTerm::Integer(3)],
        ));
        let results = resolve(&q, &db, &cfg);
        assert!(results.is_empty());
    }

    // ── Built-in: \+/1 negation ──────────────────────────────────────────────

    #[test]
    fn test_negation_as_failure() {
        let db = LpDatabase::new();
        let cfg = default_cfg();
        // \+(fail) should succeed
        let q = Query::single(LpTerm::compound("\\+", vec![LpTerm::atom("fail")]));
        let results = resolve(&q, &db, &cfg);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_negation_as_failure_fail() {
        let db = LpDatabase::new();
        let cfg = default_cfg();
        // \+(true) should fail
        let q = Query::single(LpTerm::compound("\\+", vec![LpTerm::atom("true")]));
        let results = resolve(&q, &db, &cfg);
        assert!(results.is_empty());
    }

    // ── solve_one ────────────────────────────────────────────────────────────

    #[test]
    fn test_solve_one_success() {
        let db = std_db();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "member",
            vec![
                LpTerm::Integer(1),
                LpTerm::list(vec![LpTerm::Integer(1), LpTerm::Integer(2)]),
            ],
        ));
        match solve_one(&q, &db, &cfg) {
            ResolutionResult::Success(_) => {}
            _ => panic!("Expected success"),
        }
    }

    #[test]
    fn test_solve_one_failure() {
        let db = std_db();
        let cfg = default_cfg();
        let q = Query::single(LpTerm::compound(
            "member",
            vec![
                LpTerm::Integer(99),
                LpTerm::list(vec![LpTerm::Integer(1), LpTerm::Integer(2)]),
            ],
        ));
        match solve_one(&q, &db, &cfg) {
            ResolutionResult::Failure => {}
            _ => panic!("Expected failure"),
        }
    }

    // ── Term pretty-printing ─────────────────────────────────────────────────

    #[test]
    fn test_term_to_string_atom() {
        assert_eq!(term_to_string(&LpTerm::atom("hello")), "hello");
    }

    #[test]
    fn test_term_to_string_var() {
        assert_eq!(term_to_string(&LpTerm::var("X")), "X");
    }

    #[test]
    fn test_term_to_string_integer() {
        assert_eq!(term_to_string(&LpTerm::Integer(42)), "42");
    }

    #[test]
    fn test_term_to_string_compound() {
        let t = LpTerm::compound("f", vec![LpTerm::Integer(1), LpTerm::atom("a")]);
        assert_eq!(term_to_string(&t), "f(1,a)");
    }

    #[test]
    fn test_term_to_string_list() {
        let t = LpTerm::list(vec![LpTerm::Integer(1), LpTerm::Integer(2)]);
        assert_eq!(term_to_string(&t), "[1,2]");
    }

    // ── Simple parser ────────────────────────────────────────────────────────

    #[test]
    fn test_parse_term_atom() {
        assert_eq!(parse_term("foo"), Some(LpTerm::atom("foo")));
    }

    #[test]
    fn test_parse_term_var() {
        assert_eq!(parse_term("X"), Some(LpTerm::var("X")));
    }

    #[test]
    fn test_parse_term_integer() {
        assert_eq!(parse_term("42"), Some(LpTerm::Integer(42)));
    }

    #[test]
    fn test_parse_term_compound() {
        let t = parse_term("f(a,b)");
        assert_eq!(
            t,
            Some(LpTerm::compound(
                "f",
                vec![LpTerm::atom("a"), LpTerm::atom("b")]
            ))
        );
    }

    #[test]
    fn test_parse_list_empty() {
        assert_eq!(parse_term("[]"), Some(LpTerm::atom("[]")));
    }

    #[test]
    fn test_parse_list_items() {
        let t = parse_term("[1,2,3]");
        assert_eq!(
            t,
            Some(LpTerm::list(vec![
                LpTerm::Integer(1),
                LpTerm::Integer(2),
                LpTerm::Integer(3)
            ]))
        );
    }

    #[test]
    fn test_parse_clause_fact() {
        let c = parse_clause("foo(a).");
        assert!(c.is_some());
        let c = c.unwrap();
        assert!(c.is_fact());
    }

    #[test]
    fn test_parse_clause_rule() {
        let c = parse_clause("member(X,[X|_]).");
        assert!(c.is_some());
        let c = c.unwrap();
        // head should be member/2
        assert!(c.is_fact()); // no :- in this one
    }

    // ── query_all convenience ────────────────────────────────────────────────

    #[test]
    fn test_query_all() {
        let db = std_db();
        let cfg = default_cfg();
        let goal = LpTerm::compound(
            "member",
            vec![
                LpTerm::var("X"),
                LpTerm::list(vec![LpTerm::atom("a"), LpTerm::atom("b")]),
            ],
        );
        let results = db.query_all(goal, &cfg);
        assert_eq!(results.len(), 2);
    }
}
