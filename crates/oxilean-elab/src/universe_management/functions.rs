//! Functions for universe-level management and polymorphism.

use std::collections::HashMap;

use super::types::{
    UniverseConstraint, UniverseCtx, UniverseError, UniverseLevel, UniverseSolution,
};

// ─── to_nat ──────────────────────────────────────────────────────────────────

impl UniverseLevel {
    /// Evaluate this level to a concrete `u64` if it contains no `Param` or
    /// `Metavar` nodes (i.e. it is ground).
    ///
    /// Returns `None` if the level contains parameters or unsolved metavariables.
    pub fn to_nat(&self) -> Option<u64> {
        match self {
            UniverseLevel::Zero => Some(0),
            UniverseLevel::Succ(inner) => inner.to_nat().map(|n| n + 1),
            UniverseLevel::Max(l, r) => {
                let lv = l.to_nat()?;
                let rv = r.to_nat()?;
                Some(lv.max(rv))
            }
            UniverseLevel::IMax(l, r) => {
                let lv = l.to_nat()?;
                let rv = r.to_nat()?;
                // IMax(u, 0) = 0, IMax(u, v+1) = Max(u, v+1)
                if rv == 0 {
                    Some(0)
                } else {
                    Some(lv.max(rv))
                }
            }
            UniverseLevel::Param(_) | UniverseLevel::Metavar(_) => None,
        }
    }
}

// ─── level_to_string / parse_level ──────────────────────────────────────────

/// Convert a universe level to a human-readable string.
pub fn level_to_string(l: &UniverseLevel) -> String {
    match l {
        UniverseLevel::Zero => "0".to_string(),
        UniverseLevel::Succ(inner) => {
            // Flatten consecutive Succs into "+N".
            let (base, n) = peel_succs(inner, 1);
            let base_str = level_to_string(base);
            if base_str == "0" {
                n.to_string()
            } else {
                format!("{base_str}+{n}")
            }
        }
        UniverseLevel::Max(l, r) => {
            format!("max({}, {})", level_to_string(l), level_to_string(r))
        }
        UniverseLevel::IMax(l, r) => {
            format!("imax({}, {})", level_to_string(l), level_to_string(r))
        }
        UniverseLevel::Param(name) => name.clone(),
        UniverseLevel::Metavar(id) => format!("?u{id}"),
    }
}

/// Peel consecutive `Succ` layers and return (base, count).
fn peel_succs(l: &UniverseLevel, count: u64) -> (&UniverseLevel, u64) {
    match l {
        UniverseLevel::Succ(inner) => peel_succs(inner, count + 1),
        other => (other, count),
    }
}

/// Build `Succ` applied `n` times to `base`.
fn apply_succs(base: UniverseLevel, n: u64) -> UniverseLevel {
    let mut result = base;
    for _ in 0..n {
        result = result.succ();
    }
    result
}

/// Parse a universe level from a simple string representation.
///
/// Supported syntax:
/// - Decimal integer → `Zero` / `Succ` chain
/// - `max(u, v)` or `imax(u, v)`
/// - Identifier → `Param` or `Metavar` (`?u<N>`)
/// - `u+N` → `Succ^N(Param("u"))`
pub fn parse_level(s: &str) -> Option<UniverseLevel> {
    let s = s.trim();
    // Decimal integer
    if let Ok(n) = s.parse::<u64>() {
        return Some(apply_succs(UniverseLevel::Zero, n));
    }
    // max(…, …) or imax(…, …)
    if let Some(rest) = s.strip_prefix("max(") {
        if let Some(inner) = rest.strip_suffix(')') {
            return parse_binary(inner, false);
        }
    }
    if let Some(rest) = s.strip_prefix("imax(") {
        if let Some(inner) = rest.strip_suffix(')') {
            return parse_binary(inner, true);
        }
    }
    // u+N
    if let Some(plus_pos) = s.rfind('+') {
        let base_str = s[..plus_pos].trim();
        let n_str = s[plus_pos + 1..].trim();
        if let Ok(n) = n_str.parse::<u64>() {
            let base = parse_level(base_str)?;
            return Some(apply_succs(base, n));
        }
    }
    // ?u<N> → Metavar
    if let Some(rest) = s.strip_prefix("?u") {
        if let Ok(id) = rest.parse::<u64>() {
            return Some(UniverseLevel::Metavar(id));
        }
    }
    // Plain identifier → Param
    if is_level_ident(s) {
        return Some(UniverseLevel::Param(s.to_string()));
    }
    None
}

/// Parse "l, r" as a binary level node.
fn parse_binary(inner: &str, is_imax: bool) -> Option<UniverseLevel> {
    // Find the top-level comma (not nested inside parens).
    let mut depth = 0usize;
    let mut split_pos = None;
    for (i, ch) in inner.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
            }
            ',' if depth == 0 => {
                split_pos = Some(i);
                break;
            }
            _ => {}
        }
    }
    let pos = split_pos?;
    let l = parse_level(&inner[..pos])?;
    let r = parse_level(&inner[pos + 1..])?;
    if is_imax {
        Some(UniverseLevel::IMax(Box::new(l), Box::new(r)))
    } else {
        Some(UniverseLevel::Max(Box::new(l), Box::new(r)))
    }
}

fn is_level_ident(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {
            chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
        }
        _ => false,
    }
}

// ─── normalize_level ─────────────────────────────────────────────────────────

/// Reduce a universe level to a canonical (normalised) form.
///
/// Applies the following reductions exhaustively:
/// - `max(u, u)` → `u`
/// - `max(0, u)` → `u`
/// - `max(u, 0)` → `u`
/// - `imax(_, 0)` → `0`
/// - `imax(u, v)` where `v ≠ 0` is ground → `max(u, v)`
/// - `succ(max(u, v))` → `max(succ(u), succ(v))`
pub fn normalize_level(l: &UniverseLevel) -> UniverseLevel {
    match l {
        UniverseLevel::Zero => UniverseLevel::Zero,
        UniverseLevel::Succ(inner) => {
            let ni = normalize_level(inner);
            // Push succ inside max for canonical form.
            match ni {
                UniverseLevel::Max(nl, nr) => {
                    UniverseLevel::Max(Box::new(nl.succ()), Box::new(nr.succ()))
                }
                other => other.succ(),
            }
        }
        UniverseLevel::Max(l, r) => {
            let nl = normalize_level(l);
            let nr = normalize_level(r);
            // max(u, u) → u
            if nl == nr {
                return nl;
            }
            // max(0, u) → u
            if nl == UniverseLevel::Zero {
                return nr;
            }
            // max(u, 0) → u
            if nr == UniverseLevel::Zero {
                return nl;
            }
            // If both are ground, compute directly.
            if let (Some(lv), Some(rv)) = (nl.to_nat(), nr.to_nat()) {
                return apply_succs(UniverseLevel::Zero, lv.max(rv));
            }
            UniverseLevel::Max(Box::new(nl), Box::new(nr))
        }
        UniverseLevel::IMax(l, r) => {
            let nl = normalize_level(l);
            let nr = normalize_level(r);
            // imax(_, 0) = 0
            if nr == UniverseLevel::Zero {
                return UniverseLevel::Zero;
            }
            // If r is ground and nonzero, reduce to max.
            if let Some(rv) = nr.to_nat() {
                if rv > 0 {
                    return normalize_level(&UniverseLevel::Max(Box::new(nl), Box::new(nr)));
                }
            }
            UniverseLevel::IMax(Box::new(nl), Box::new(nr))
        }
        UniverseLevel::Param(name) => UniverseLevel::Param(name.clone()),
        UniverseLevel::Metavar(id) => UniverseLevel::Metavar(*id),
    }
}

// ─── level_leq ───────────────────────────────────────────────────────────────

/// Check whether `l ≤ r`, returning `Some(true/false)` when decidable, or
/// `None` when parameters/metavars prevent a decision.
pub fn level_leq(l: &UniverseLevel, r: &UniverseLevel) -> Option<bool> {
    let nl = normalize_level(l);
    let nr = normalize_level(r);
    match (nl.to_nat(), nr.to_nat()) {
        (Some(lv), Some(rv)) => Some(lv <= rv),
        _ => {
            // Structural equal ⇒ always leq.
            if nl == nr {
                Some(true)
            } else {
                None
            }
        }
    }
}

// ─── max_level / imax_level ──────────────────────────────────────────────────

/// Compute the (normalised) maximum of two universe levels.
pub fn max_level(l: &UniverseLevel, r: &UniverseLevel) -> UniverseLevel {
    normalize_level(&UniverseLevel::Max(
        Box::new(l.clone()),
        Box::new(r.clone()),
    ))
}

/// Compute the (normalised) impredicative maximum of two universe levels.
///
/// `imax(u, 0) = 0`, `imax(u, v+1) = max(u, v+1)`.
pub fn imax_level(l: &UniverseLevel, r: &UniverseLevel) -> UniverseLevel {
    normalize_level(&UniverseLevel::IMax(
        Box::new(l.clone()),
        Box::new(r.clone()),
    ))
}

// ─── instantiate_params ──────────────────────────────────────────────────────

/// Substitute named universe parameters with concrete levels.
pub fn instantiate_params(
    l: &UniverseLevel,
    params: &HashMap<String, UniverseLevel>,
) -> UniverseLevel {
    match l {
        UniverseLevel::Zero => UniverseLevel::Zero,
        UniverseLevel::Succ(inner) => instantiate_params(inner, params).succ(),
        UniverseLevel::Max(a, b) => UniverseLevel::Max(
            Box::new(instantiate_params(a, params)),
            Box::new(instantiate_params(b, params)),
        ),
        UniverseLevel::IMax(a, b) => UniverseLevel::IMax(
            Box::new(instantiate_params(a, params)),
            Box::new(instantiate_params(b, params)),
        ),
        UniverseLevel::Param(name) => match params.get(name) {
            Some(level) => level.clone(),
            None => l.clone(),
        },
        UniverseLevel::Metavar(id) => UniverseLevel::Metavar(*id),
    }
}

// ─── fresh_metavar ───────────────────────────────────────────────────────────

/// Allocate a fresh universe metavariable in the context.
pub fn fresh_metavar(ctx: &mut UniverseCtx) -> UniverseLevel {
    let id = ctx.next_meta_id;
    ctx.next_meta_id += 1;
    UniverseLevel::Metavar(id)
}

// ─── unify_levels ────────────────────────────────────────────────────────────

/// Attempt to unify two universe levels, recording assignments in `ctx`.
///
/// On success the two levels become definitionally equal under the new
/// assignments.  Returns an error if unification would be cyclic or
/// inconsistent.
pub fn unify_levels(
    l: &UniverseLevel,
    r: &UniverseLevel,
    ctx: &mut UniverseCtx,
) -> Result<(), UniverseError> {
    let l = resolve_metavar(l, ctx);
    let r = resolve_metavar(r, ctx);

    if l == r {
        return Ok(());
    }

    match (&l, &r) {
        (UniverseLevel::Metavar(id), other) | (other, UniverseLevel::Metavar(id)) => {
            let id = *id;
            let other = other.clone();
            // Occurs check.
            if occurs(id, &other) {
                return Err(UniverseError::Cycle);
            }
            ctx.assign(id, other);
            Ok(())
        }
        (UniverseLevel::Succ(li), UniverseLevel::Succ(ri)) => unify_levels(li, ri, ctx),
        (UniverseLevel::Max(ll, lr), UniverseLevel::Max(rl, rr)) => {
            unify_levels(ll, rl, ctx)?;
            unify_levels(lr, rr, ctx)
        }
        (UniverseLevel::IMax(ll, lr), UniverseLevel::IMax(rl, rr)) => {
            unify_levels(ll, rl, ctx)?;
            unify_levels(lr, rr, ctx)
        }
        _ => {
            // Try normalising both sides; if equal after normalisation, ok.
            let nl = normalize_level(&l);
            let nr = normalize_level(&r);
            if nl == nr {
                Ok(())
            } else {
                Err(UniverseError::Inconsistent(UniverseConstraint::Eq(l, r)))
            }
        }
    }
}

/// Transitively resolve a universe level through metavar assignments.
fn resolve_metavar(l: &UniverseLevel, ctx: &UniverseCtx) -> UniverseLevel {
    match l {
        UniverseLevel::Metavar(id) => {
            if let Some(assigned) = ctx.lookup(*id) {
                resolve_metavar(&assigned.clone(), ctx)
            } else {
                l.clone()
            }
        }
        UniverseLevel::Succ(inner) => {
            let resolved = resolve_metavar(inner, ctx);
            resolved.succ()
        }
        UniverseLevel::Max(a, b) => UniverseLevel::Max(
            Box::new(resolve_metavar(a, ctx)),
            Box::new(resolve_metavar(b, ctx)),
        ),
        UniverseLevel::IMax(a, b) => UniverseLevel::IMax(
            Box::new(resolve_metavar(a, ctx)),
            Box::new(resolve_metavar(b, ctx)),
        ),
        other => other.clone(),
    }
}

/// Occurs check: returns `true` if metavar `id` appears in `l`.
fn occurs(id: u64, l: &UniverseLevel) -> bool {
    match l {
        UniverseLevel::Metavar(other_id) => *other_id == id,
        UniverseLevel::Succ(inner) => occurs(id, inner),
        UniverseLevel::Max(a, b) | UniverseLevel::IMax(a, b) => occurs(id, a) || occurs(id, b),
        _ => false,
    }
}

// ─── solve_constraints ───────────────────────────────────────────────────────

/// Attempt to solve all constraints in `ctx` and return a `UniverseSolution`.
///
/// The strategy is:
/// 1. For `Eq(Param(name), level)` or `Eq(level, Param(name))` constraints:
///    directly record a named-parameter assignment.
/// 2. For `Eq` constraints involving metavars: unify via the metavar context.
/// 3. For `Leq` / `Lt` constraints: verify when both sides are ground.
pub fn solve_constraints(ctx: &UniverseCtx) -> Result<UniverseSolution, UniverseError> {
    let mut working_ctx = ctx.clone();
    // Named-parameter assignments built up during solving.
    let mut param_assignments: HashMap<String, UniverseLevel> = HashMap::new();

    // Pass 1: handle equality constraints.
    for constraint in &ctx.constraints {
        if let UniverseConstraint::Eq(l, r) = constraint {
            // Direct Param = level assignment.
            match (l, r) {
                (UniverseLevel::Param(name), other) => {
                    let resolved = resolve_metavar(other, &working_ctx);
                    let normal = normalize_level(&resolved);
                    param_assignments.insert(name.clone(), normal);
                    continue;
                }
                (other, UniverseLevel::Param(name)) => {
                    let resolved = resolve_metavar(other, &working_ctx);
                    let normal = normalize_level(&resolved);
                    param_assignments.insert(name.clone(), normal);
                    continue;
                }
                _ => {}
            }
            // General metavar unification.
            unify_levels(l, r, &mut working_ctx)?;
        }
    }

    // Pass 2: check inequality constraints under the new assignments.
    for constraint in &ctx.constraints {
        let apply_param = |lv: &UniverseLevel| -> UniverseLevel {
            let resolved_meta = resolve_metavar(lv, &working_ctx);
            // Also substitute known param assignments.
            instantiate_params(&resolved_meta, &param_assignments)
        };
        match constraint {
            UniverseConstraint::Leq(l, r) => {
                let nl = normalize_level(&apply_param(l));
                let nr = normalize_level(&apply_param(r));
                if let Some(false) = level_leq(&nl, &nr) {
                    return Err(UniverseError::Inconsistent(constraint.clone()));
                }
            }
            UniverseConstraint::Lt(l, r) => {
                // l < r  ⟺  l+1 ≤ r
                let nl = normalize_level(&apply_param(l).succ());
                let nr = normalize_level(&apply_param(r));
                if let Some(false) = level_leq(&nl, &nr) {
                    return Err(UniverseError::Inconsistent(constraint.clone()));
                }
            }
            UniverseConstraint::Eq(_, _) => {}
        }
    }

    // Build solution: for each named parameter, use explicit assignment if
    // available, otherwise resolve through metavar context.
    let mut assignments: HashMap<String, UniverseLevel> = HashMap::new();
    for param in &ctx.params {
        if let Some(level) = param_assignments.get(param) {
            assignments.insert(param.clone(), level.clone());
        } else {
            let param_level = UniverseLevel::Param(param.clone());
            let resolved = resolve_metavar(&param_level, &working_ctx);
            let normal = normalize_level(&resolved);
            assignments.insert(param.clone(), normal);
        }
    }

    Ok(UniverseSolution::minimal(assignments))
}

// ─── tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universe_management::types::{
        UniverseConstraint, UniverseCtx, UniverseError, UniverseLevel,
    };

    // Helpers
    fn zero() -> UniverseLevel {
        UniverseLevel::Zero
    }
    fn one() -> UniverseLevel {
        zero().succ()
    }
    fn two() -> UniverseLevel {
        one().succ()
    }
    fn param(name: &str) -> UniverseLevel {
        UniverseLevel::Param(name.to_string())
    }
    fn meta(id: u64) -> UniverseLevel {
        UniverseLevel::Metavar(id)
    }

    // --- to_nat ---

    #[test]
    fn test_to_nat_zero() {
        assert_eq!(zero().to_nat(), Some(0));
    }

    #[test]
    fn test_to_nat_succ() {
        assert_eq!(one().to_nat(), Some(1));
        assert_eq!(two().to_nat(), Some(2));
    }

    #[test]
    fn test_to_nat_max() {
        let l = UniverseLevel::Max(Box::new(one()), Box::new(two()));
        assert_eq!(l.to_nat(), Some(2));
    }

    #[test]
    fn test_to_nat_imax_zero_right() {
        let l = UniverseLevel::IMax(Box::new(two()), Box::new(zero()));
        assert_eq!(l.to_nat(), Some(0));
    }

    #[test]
    fn test_to_nat_imax_nonzero_right() {
        let l = UniverseLevel::IMax(Box::new(one()), Box::new(two()));
        assert_eq!(l.to_nat(), Some(2));
    }

    #[test]
    fn test_to_nat_param_none() {
        assert_eq!(param("u").to_nat(), None);
    }

    #[test]
    fn test_to_nat_meta_none() {
        assert_eq!(meta(0).to_nat(), None);
    }

    // --- level_to_string ---

    #[test]
    fn test_to_string_zero() {
        assert_eq!(level_to_string(&zero()), "0");
    }

    #[test]
    fn test_to_string_one() {
        assert_eq!(level_to_string(&one()), "1");
    }

    #[test]
    fn test_to_string_param() {
        assert_eq!(level_to_string(&param("u")), "u");
    }

    #[test]
    fn test_to_string_meta() {
        assert_eq!(level_to_string(&meta(3)), "?u3");
    }

    #[test]
    fn test_to_string_max() {
        let l = max_level(&one(), &two());
        assert_eq!(l.to_nat(), Some(2));
    }

    // --- parse_level ---

    #[test]
    fn test_parse_zero() {
        assert_eq!(parse_level("0"), Some(zero()));
    }

    #[test]
    fn test_parse_three() {
        assert_eq!(parse_level("3"), Some(apply_succs(zero(), 3)));
    }

    #[test]
    fn test_parse_param() {
        assert_eq!(parse_level("u"), Some(param("u")));
    }

    #[test]
    fn test_parse_meta() {
        assert_eq!(parse_level("?u5"), Some(meta(5)));
    }

    #[test]
    fn test_parse_plus() {
        let result = parse_level("u+1");
        assert_eq!(result, Some(param("u").succ()));
    }

    #[test]
    fn test_parse_max() {
        let result = parse_level("max(1, 2)");
        assert!(result.is_some());
        assert_eq!(result.unwrap().to_nat(), Some(2));
    }

    #[test]
    fn test_parse_imax_zero() {
        let result = parse_level("imax(2, 0)");
        assert!(result.is_some());
        assert_eq!(normalize_level(&result.unwrap()), zero());
    }

    // --- normalize_level ---

    #[test]
    fn test_normalize_max_self() {
        let l = UniverseLevel::Max(Box::new(one()), Box::new(one()));
        assert_eq!(normalize_level(&l), one());
    }

    #[test]
    fn test_normalize_max_zero_left() {
        let l = UniverseLevel::Max(Box::new(zero()), Box::new(two()));
        assert_eq!(normalize_level(&l), two());
    }

    #[test]
    fn test_normalize_imax_zero_right() {
        let l = UniverseLevel::IMax(Box::new(two()), Box::new(zero()));
        assert_eq!(normalize_level(&l), zero());
    }

    // --- level_leq ---

    #[test]
    fn test_leq_concrete() {
        assert_eq!(level_leq(&zero(), &one()), Some(true));
        assert_eq!(level_leq(&two(), &one()), Some(false));
        assert_eq!(level_leq(&one(), &one()), Some(true));
    }

    #[test]
    fn test_leq_same_param() {
        // param("u") ≤ param("u") — structurally equal
        assert_eq!(level_leq(&param("u"), &param("u")), Some(true));
    }

    #[test]
    fn test_leq_different_params_unknown() {
        assert_eq!(level_leq(&param("u"), &param("v")), None);
    }

    // --- max_level / imax_level ---

    #[test]
    fn test_max_level_concrete() {
        let result = max_level(&one(), &two());
        assert_eq!(result.to_nat(), Some(2));
    }

    #[test]
    fn test_imax_level_zero() {
        let result = imax_level(&two(), &zero());
        assert_eq!(result, zero());
    }

    // --- instantiate_params ---

    #[test]
    fn test_instantiate_param() {
        let mut map = HashMap::new();
        map.insert("u".to_string(), two());
        let l = param("u").succ();
        let result = instantiate_params(&l, &map);
        assert_eq!(result.to_nat(), Some(3));
    }

    #[test]
    fn test_instantiate_no_match() {
        let map = HashMap::new();
        let l = param("u");
        let result = instantiate_params(&l, &map);
        assert_eq!(result, param("u"));
    }

    // --- fresh_metavar ---

    #[test]
    fn test_fresh_metavar_increments() {
        let mut ctx = UniverseCtx::new();
        let m0 = fresh_metavar(&mut ctx);
        let m1 = fresh_metavar(&mut ctx);
        assert_eq!(m0, meta(0));
        assert_eq!(m1, meta(1));
    }

    // --- unify_levels ---

    #[test]
    fn test_unify_same_levels() {
        let mut ctx = UniverseCtx::new();
        assert!(unify_levels(&one(), &one(), &mut ctx).is_ok());
    }

    #[test]
    fn test_unify_metavar_assignment() {
        let mut ctx = UniverseCtx::new();
        let m = fresh_metavar(&mut ctx);
        unify_levels(&m, &two(), &mut ctx).expect("unify should succeed");
        assert_eq!(ctx.lookup(0), Some(&two()));
    }

    #[test]
    fn test_unify_cycle_error() {
        let mut ctx = UniverseCtx::new();
        let m = fresh_metavar(&mut ctx);
        let l = m.clone().succ();
        let err = unify_levels(&m, &l, &mut ctx).expect_err("should fail");
        assert_eq!(err, UniverseError::Cycle);
    }

    #[test]
    fn test_unify_inconsistent() {
        let mut ctx = UniverseCtx::new();
        let err = unify_levels(&one(), &two(), &mut ctx).expect_err("should fail");
        assert!(matches!(err, UniverseError::Inconsistent(_)));
    }

    // --- solve_constraints ---

    #[test]
    fn test_solve_eq_constraint() {
        let mut ctx = UniverseCtx::with_params(vec!["u".to_string()]);
        ctx.add_constraint(UniverseConstraint::Eq(param("u"), two()));
        let sol = solve_constraints(&ctx).expect("should solve");
        assert_eq!(sol.get("u"), Some(&two()));
    }

    #[test]
    fn test_solve_leq_ok() {
        let ctx = UniverseCtx::new();
        // No constraints → trivially ok.
        let sol = solve_constraints(&ctx).expect("should solve");
        assert!(sol.is_empty());
    }

    #[test]
    fn test_solve_inconsistent_leq() {
        let mut ctx = UniverseCtx::new();
        // 2 ≤ 1 is false.
        ctx.add_constraint(UniverseConstraint::Leq(two(), one()));
        let err = solve_constraints(&ctx).expect_err("should fail");
        assert!(matches!(err, UniverseError::Inconsistent(_)));
    }
}
