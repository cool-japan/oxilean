//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;
use oxilean_kernel::{Environment, Expr, FVarId, Level, Name};
use oxilean_parse::{Located, MatchArm, Pattern, SurfaceExpr};
use std::collections::HashMap;

use super::types::{
    ColumnHeuristic, DecisionNode, DecisionTree, DecisionTreeAnalyzer, ElabPattern,
    ExhaustivenessResult, LiteralSet, MatchCoverage, MatchElaborator, MatchEquation, MatchResult,
    MissingPattern, PatternCompiler, PatternKind, PatternMatcher, PatternMatrix, PatternNormalizer,
    PatternPrinter, PatternStats, PatternSubstitution,
};

/// Build a decision node from a surface-level pattern.
pub fn build_node_from_pattern(pat: &Pattern, rhs: Expr, fallback: DecisionNode) -> DecisionNode {
    match pat {
        Pattern::Wild | Pattern::Var(_) => DecisionNode::Leaf(rhs),
        Pattern::Ctor(name, _sub_pats) => DecisionNode::Switch {
            var: Name::str("_x"),
            branches: vec![(Name::str(name), Box::new(DecisionNode::Leaf(rhs)))],
            default: Some(Box::new(fallback)),
        },
        Pattern::Lit(lit) => DecisionNode::Switch {
            var: Name::str("_x"),
            branches: vec![(
                Name::str(format!("{:?}", lit)),
                Box::new(DecisionNode::Leaf(rhs)),
            )],
            default: Some(Box::new(fallback)),
        },
        Pattern::Or(p1, _p2) => build_node_from_pattern(&p1.value, rhs, fallback),
    }
}
/// Build a decision node from an elaborated pattern.
pub fn build_node_from_elab_pattern(
    pat: &ElabPattern,
    rhs: Expr,
    fallback: DecisionNode,
) -> DecisionNode {
    match pat {
        ElabPattern::Wild | ElabPattern::Var(_, _, _) | ElabPattern::As(_, _, _) => {
            DecisionNode::Leaf(rhs)
        }
        ElabPattern::Ctor(name, _sub_pats, _ty) => DecisionNode::Switch {
            var: Name::str("_x"),
            branches: vec![(name.clone(), Box::new(DecisionNode::Leaf(rhs)))],
            default: Some(Box::new(fallback)),
        },
        ElabPattern::Lit(lit) => DecisionNode::Switch {
            var: Name::str("_x"),
            branches: vec![(
                Name::str(format!("{:?}", lit)),
                Box::new(DecisionNode::Leaf(rhs)),
            )],
            default: Some(Box::new(fallback)),
        },
        ElabPattern::Or(p1, _p2) => build_node_from_elab_pattern(p1, rhs, fallback),
        ElabPattern::Inaccessible(_) => DecisionNode::Leaf(rhs),
    }
}
/// Compile a decision node to a kernel expression.
pub fn compile_node(node: &DecisionNode) -> Expr {
    match node {
        DecisionNode::Leaf(expr) => expr.clone(),
        DecisionNode::Switch {
            var: _,
            branches,
            default,
        } => {
            if let Some((_, first_branch)) = branches.first() {
                compile_node(first_branch)
            } else if let Some(default_node) = default {
                compile_node(default_node)
            } else {
                Expr::BVar(0)
            }
        }
        DecisionNode::Guard {
            condition: _,
            then,
            else_: _,
        } => compile_node(then),
        DecisionNode::Fail => Expr::BVar(0),
    }
}
/// Elaborate a pattern match expression.
pub fn elaborate_match(
    ctx: &mut ElabContext,
    scrutinee: &Located<SurfaceExpr>,
    arms: &[(Located<Pattern>, Located<SurfaceExpr>)],
) -> Result<MatchResult, String> {
    let _scrutinee_expr = surface_to_placeholder(&scrutinee.value);
    let match_arms: Vec<MatchArm> = arms
        .iter()
        .map(|(pat, rhs)| MatchArm {
            pattern: pat.clone(),
            guard: None,
            rhs: rhs.clone(),
        })
        .collect();
    let mut equations = Vec::new();
    for (i, arm) in match_arms.iter().enumerate() {
        let (elab_pat, bindings) = elaborate_pattern(
            ctx,
            &arm.pattern.value,
            &Expr::Sort(Level::succ(Level::zero())),
        )?;
        for (fvar, name, ty) in &bindings {
            ctx.push_local(name.clone(), ty.clone(), None);
            let _ = fvar;
        }
        let rhs = surface_to_placeholder(&arm.rhs.value);
        for _ in &bindings {
            ctx.pop_local();
        }
        equations.push(MatchEquation {
            patterns: vec![elab_pat],
            rhs,
            arm_idx: i,
        });
    }
    let patterns: Vec<Located<Pattern>> = arms.iter().map(|(p, _)| p.clone()).collect();
    let exhaustiveness = check_exhaustive_full(&patterns);
    let redundant = check_redundant_full(&patterns);
    let tree = DecisionTree::new(arms)?;
    let compiled_expr = tree.compile();
    Ok(MatchResult {
        expr: compiled_expr,
        defs: Vec::new(),
        equations,
        missing_patterns: exhaustiveness.missing,
        redundant_arms: redundant,
    })
}
/// Check if patterns are exhaustive (simple version).
pub fn check_exhaustive(patterns: &[Located<Pattern>]) -> Result<(), String> {
    let result = check_exhaustive_full(patterns);
    if result.is_exhaustive {
        Ok(())
    } else {
        let missing_names: Vec<String> = result
            .missing
            .iter()
            .map(|m| format!("{}", m.ctor_name))
            .collect();
        Err(format!(
            "non-exhaustive patterns: missing {}",
            missing_names.join(", ")
        ))
    }
}
/// Check for redundant patterns (simple version).
pub fn check_redundant(patterns: &[Located<Pattern>]) -> Vec<usize> {
    check_redundant_full(patterns)
}
/// Perform a full exhaustiveness check on the given patterns.
///
/// The algorithm collects all constructors mentioned in the patterns,
/// then checks that every constructor is covered. If no constructors
/// are used (all wildcards/vars), the patterns are trivially exhaustive.
pub fn check_exhaustive_full(patterns: &[Located<Pattern>]) -> ExhaustivenessResult {
    if patterns.is_empty() {
        return ExhaustivenessResult {
            is_exhaustive: false,
            missing: vec![MissingPattern {
                ctor_name: Name::str("_"),
                sub_patterns: vec![ElabPattern::Wild],
            }],
        };
    }
    for pat in patterns {
        if is_irrefutable_pattern(&pat.value) {
            return ExhaustivenessResult {
                is_exhaustive: true,
                missing: Vec::new(),
            };
        }
    }
    let mut mentioned_ctors: Vec<String> = Vec::new();
    for pat in patterns {
        collect_constructors(&pat.value, &mut mentioned_ctors);
    }
    if !mentioned_ctors.is_empty() {
        if let Some(all_ctors) = infer_ctor_set_from_names(&mentioned_ctors) {
            return check_exhaustive_with_ctors(patterns, &all_ctors);
        }
    }
    ExhaustivenessResult {
        is_exhaustive: true,
        missing: Vec::new(),
    }
}
/// Infer the complete sibling constructor set for a type given the constructors
/// that were mentioned in the pattern match. Returns `None` if the type is
/// not a known built-in with a fixed, finite set of constructors.
fn infer_ctor_set_from_names(mentioned: &[String]) -> Option<Vec<Name>> {
    let normalise = |s: &str| -> String {
        if let Some(pos) = s.rfind('.') {
            s[pos + 1..].to_ascii_lowercase()
        } else {
            s.to_ascii_lowercase()
        }
    };
    let mut norm_set: std::collections::HashSet<String> =
        mentioned.iter().map(|s| normalise(s)).collect();
    if norm_set.contains("true") || norm_set.contains("false") {
        return Some(vec![Name::str("true"), Name::str("false")]);
    }
    if norm_set.contains("some") || norm_set.contains("none") {
        return Some(vec![Name::str("some"), Name::str("none")]);
    }
    if norm_set.contains("inl") || norm_set.contains("inr") {
        return Some(vec![Name::str("inl"), Name::str("inr")]);
    }
    if norm_set.contains("ok") || norm_set.contains("err") || norm_set.contains("error") {
        return Some(vec![Name::str("ok"), Name::str("error")]);
    }
    if norm_set.contains("zero") || norm_set.contains("succ") {
        return Some(vec![Name::str("zero"), Name::str("succ")]);
    }
    if norm_set.contains("nil") || norm_set.contains("cons") {
        return Some(vec![Name::str("nil"), Name::str("cons")]);
    }
    if norm_set.contains("lt") || norm_set.contains("eq") || norm_set.contains("gt") {
        norm_set.insert("lt".to_string());
        norm_set.insert("eq".to_string());
        norm_set.insert("gt".to_string());
        return Some(vec![Name::str("lt"), Name::str("eq"), Name::str("gt")]);
    }
    if norm_set.contains("unit") || norm_set.contains("mk") {
        return Some(vec![Name::str("unit")]);
    }
    None
}
/// Check exhaustiveness against a known set of constructors.
#[allow(dead_code)]
pub fn check_exhaustive_with_ctors(
    patterns: &[Located<Pattern>],
    all_ctors: &[Name],
) -> ExhaustivenessResult {
    if patterns.is_empty() {
        let missing: Vec<MissingPattern> = all_ctors
            .iter()
            .map(|c| MissingPattern {
                ctor_name: c.clone(),
                sub_patterns: Vec::new(),
            })
            .collect();
        return ExhaustivenessResult {
            is_exhaustive: false,
            missing,
        };
    }
    for pat in patterns {
        if is_irrefutable_pattern(&pat.value) {
            return ExhaustivenessResult {
                is_exhaustive: true,
                missing: Vec::new(),
            };
        }
    }
    let mut covered: Vec<Name> = Vec::new();
    for pat in patterns {
        collect_ctor_names(&pat.value, &mut covered);
    }
    let missing: Vec<MissingPattern> = all_ctors
        .iter()
        .filter(|c| !covered.contains(c))
        .map(|c| MissingPattern {
            ctor_name: c.clone(),
            sub_patterns: Vec::new(),
        })
        .collect();
    ExhaustivenessResult {
        is_exhaustive: missing.is_empty(),
        missing,
    }
}
/// Collect constructor name references from a pattern.
fn collect_constructors(pat: &Pattern, ctors: &mut Vec<String>) {
    match pat {
        Pattern::Ctor(name, sub_pats) => {
            if !ctors.contains(name) {
                ctors.push(name.clone());
            }
            for sp in sub_pats {
                collect_constructors(&sp.value, ctors);
            }
        }
        Pattern::Or(p1, p2) => {
            collect_constructors(&p1.value, ctors);
            collect_constructors(&p2.value, ctors);
        }
        Pattern::Wild | Pattern::Var(_) | Pattern::Lit(_) => {}
    }
}
/// Collect `Name`-typed constructor names from a pattern.
fn collect_ctor_names(pat: &Pattern, names: &mut Vec<Name>) {
    match pat {
        Pattern::Ctor(name, sub_pats) => {
            let n = Name::str(name);
            if !names.contains(&n) {
                names.push(n);
            }
            for sp in sub_pats {
                collect_ctor_names(&sp.value, names);
            }
        }
        Pattern::Or(p1, p2) => {
            collect_ctor_names(&p1.value, names);
            collect_ctor_names(&p2.value, names);
        }
        Pattern::Wild | Pattern::Var(_) | Pattern::Lit(_) => {}
    }
}
/// Perform a full redundancy check, returning indices of unreachable arms.
///
/// An arm is redundant if every pattern it matches is already matched by
/// a preceding arm.
pub fn check_redundant_full(patterns: &[Located<Pattern>]) -> Vec<usize> {
    let mut redundant = Vec::new();
    let mut seen_catch_all = false;
    for (i, pat) in patterns.iter().enumerate() {
        if seen_catch_all {
            redundant.push(i);
        } else if is_irrefutable_pattern(&pat.value) {
            seen_catch_all = true;
        } else {
            let is_subsumed = patterns[..i]
                .iter()
                .any(|prev| pattern_subsumes(&prev.value, &pat.value));
            if is_subsumed {
                redundant.push(i);
            }
        }
    }
    redundant
}
/// Check if pattern `a` subsumes pattern `b` (i.e., every value matching `b`
/// also matches `a`).
fn pattern_subsumes(a: &Pattern, b: &Pattern) -> bool {
    match (a, b) {
        (Pattern::Wild, _) | (Pattern::Var(_), _) => true,
        (Pattern::Ctor(na, pa), Pattern::Ctor(nb, pb)) if na == nb && pa.len() == pb.len() => pa
            .iter()
            .zip(pb.iter())
            .all(|(sa, sb)| pattern_subsumes(&sa.value, &sb.value)),
        (Pattern::Lit(la), Pattern::Lit(lb)) => la == lb,
        (Pattern::Or(a1, a2), _) => {
            pattern_subsumes(&a1.value, b) || pattern_subsumes(&a2.value, b)
        }
        _ => false,
    }
}
/// Elaborate a surface-level pattern into an `ElabPattern`, returning
#[allow(clippy::type_complexity)]
/// the elaborated pattern and a list of bindings introduced.
pub fn elaborate_pattern(
    _ctx: &mut ElabContext,
    pat: &Pattern,
    expected_ty: &Expr,
) -> Result<(ElabPattern, Vec<(FVarId, Name, Expr)>), String> {
    elaborate_pattern_inner(pat, expected_ty, &mut 0)
}
/// Inner recursive helper for pattern elaboration.
#[allow(clippy::type_complexity)]
fn elaborate_pattern_inner(
    pat: &Pattern,
    expected_ty: &Expr,
    next_fvar: &mut u64,
) -> Result<(ElabPattern, Vec<(FVarId, Name, Expr)>), String> {
    match pat {
        Pattern::Wild => Ok((ElabPattern::Wild, Vec::new())),
        Pattern::Var(name) => {
            let fvar = FVarId(*next_fvar);
            *next_fvar += 1;
            let n = Name::str(name);
            let bindings = vec![(fvar, n.clone(), expected_ty.clone())];
            Ok((ElabPattern::Var(fvar, n, expected_ty.clone()), bindings))
        }
        Pattern::Ctor(name, sub_pats) => {
            let mut all_bindings = Vec::new();
            let mut elab_sub = Vec::new();
            for sp in sub_pats {
                let sub_ty = Expr::Sort(Level::succ(Level::zero()));
                let (elab_p, bindings) = elaborate_pattern_inner(&sp.value, &sub_ty, next_fvar)?;
                all_bindings.extend(bindings);
                elab_sub.push(elab_p);
            }
            Ok((
                ElabPattern::Ctor(Name::str(name), elab_sub, expected_ty.clone()),
                all_bindings,
            ))
        }
        Pattern::Lit(lit) => {
            let kernel_lit = convert_literal(lit);
            Ok((ElabPattern::Lit(kernel_lit), Vec::new()))
        }
        Pattern::Or(p1, p2) => {
            let (elab_p1, bindings1) = elaborate_pattern_inner(&p1.value, expected_ty, next_fvar)?;
            let (elab_p2, bindings2) = elaborate_pattern_inner(&p2.value, expected_ty, next_fvar)?;
            let mut all_bindings = bindings1;
            all_bindings.extend(bindings2);
            Ok((
                ElabPattern::Or(Box::new(elab_p1), Box::new(elab_p2)),
                all_bindings,
            ))
        }
    }
}
/// Convert a parse literal to a kernel literal.
fn convert_literal(lit: &oxilean_parse::Literal) -> oxilean_kernel::Literal {
    match lit {
        oxilean_parse::Literal::Nat(n) => oxilean_kernel::Literal::Nat(*n),
        oxilean_parse::Literal::String(s) => oxilean_kernel::Literal::Str(s.clone()),
        oxilean_parse::Literal::Char(_) => oxilean_kernel::Literal::Str("?".to_string()),
        oxilean_parse::Literal::Float(_) => oxilean_kernel::Literal::Nat(0),
    }
}
/// Extract all variables bound by a pattern.
pub fn pattern_vars(pat: &ElabPattern) -> Vec<(Name, FVarId)> {
    let mut result = Vec::new();
    collect_pattern_vars(pat, &mut result);
    result
}
/// Recursive helper for `pattern_vars`.
fn collect_pattern_vars(pat: &ElabPattern, out: &mut Vec<(Name, FVarId)>) {
    match pat {
        ElabPattern::Wild | ElabPattern::Lit(_) | ElabPattern::Inaccessible(_) => {}
        ElabPattern::Var(fvar, name, _) => {
            out.push((name.clone(), *fvar));
        }
        ElabPattern::Ctor(_, sub_pats, _) => {
            for sp in sub_pats {
                collect_pattern_vars(sp, out);
            }
        }
        ElabPattern::Or(p1, p2) => {
            collect_pattern_vars(p1, out);
            collect_pattern_vars(p2, out);
        }
        ElabPattern::As(fvar, name, inner) => {
            out.push((name.clone(), *fvar));
            collect_pattern_vars(inner, out);
        }
    }
}
/// Compute the nesting depth of a pattern.
pub fn pattern_depth(pat: &ElabPattern) -> usize {
    match pat {
        ElabPattern::Wild | ElabPattern::Lit(_) | ElabPattern::Inaccessible(_) => 0,
        ElabPattern::Var(_, _, _) => 0,
        ElabPattern::Ctor(_, sub_pats, _) => {
            1 + sub_pats.iter().map(pattern_depth).max().unwrap_or(0)
        }
        ElabPattern::Or(p1, p2) => pattern_depth(p1).max(pattern_depth(p2)),
        ElabPattern::As(_, _, inner) => pattern_depth(inner),
    }
}
/// Check if a pattern is irrefutable (always matches).
pub fn is_irrefutable(pat: &ElabPattern) -> bool {
    match pat {
        ElabPattern::Wild | ElabPattern::Var(_, _, _) => true,
        ElabPattern::As(_, _, inner) => is_irrefutable(inner),
        ElabPattern::Or(p1, p2) => is_irrefutable(p1) || is_irrefutable(p2),
        ElabPattern::Ctor(_, _, _) | ElabPattern::Lit(_) | ElabPattern::Inaccessible(_) => false,
    }
}
/// Check if a surface-level pattern is irrefutable.
fn is_irrefutable_pattern(pat: &Pattern) -> bool {
    match pat {
        Pattern::Wild | Pattern::Var(_) => true,
        Pattern::Or(p1, p2) => {
            is_irrefutable_pattern(&p1.value) || is_irrefutable_pattern(&p2.value)
        }
        Pattern::Ctor(_, _) | Pattern::Lit(_) => false,
    }
}
/// Get all constructors for a type from the environment.
#[allow(dead_code)]
pub fn constructors_for_type(env: &Environment, ty: &Expr) -> Vec<Name> {
    let head = match ty {
        Expr::Const(name, _) => name.clone(),
        Expr::App(fun, _) => match fun.as_ref() {
            Expr::Const(name, _) => name.clone(),
            _ => return Vec::new(),
        },
        _ => return Vec::new(),
    };
    if let Some(ind_val) = env.get_inductive_val(&head) {
        ind_val.ctors.clone()
    } else {
        Vec::new()
    }
}
/// Specialize a matrix for a given constructor (free function wrapper).
#[allow(dead_code)]
pub fn specialize_matrix(
    matrix: &PatternMatrix,
    col: usize,
    ctor: &Name,
    arity: usize,
) -> PatternMatrix {
    matrix.specialize(col, ctor, arity)
}
/// Compute the default matrix (free function wrapper).
#[allow(dead_code)]
pub fn default_matrix(matrix: &PatternMatrix, col: usize) -> PatternMatrix {
    matrix.default_matrix(col)
}
/// Compile a match expression from a scrutinee, its type, and a decision tree.
#[allow(dead_code)]
pub fn compile_match(
    _scrutinee: &Expr,
    _scrutinee_ty: &Expr,
    tree: &DecisionTree,
    _env: &Environment,
) -> Expr {
    tree.compile()
}
/// Simple surface-to-kernel conversion for placeholder purposes.
///
/// Converts the most common surface expression forms to kernel `Expr`.
/// Complex forms (dependent types, type-class applications, etc.) fall
/// back to `Expr::Sort(1)` (i.e. `Type`) as a conservative placeholder.
pub fn surface_to_placeholder(surf: &SurfaceExpr) -> Expr {
    match surf {
        SurfaceExpr::Var(name) => Expr::Const(Name::str(name), Vec::new()),
        SurfaceExpr::Sort(oxilean_parse::SortKind::Prop) => Expr::Sort(Level::zero()),
        SurfaceExpr::Sort(_) => Expr::Sort(Level::succ(Level::zero())),
        SurfaceExpr::Lit(oxilean_parse::Literal::Nat(n)) => {
            Expr::Lit(oxilean_kernel::Literal::Nat(*n))
        }
        SurfaceExpr::Lit(oxilean_parse::Literal::String(s)) => {
            Expr::Lit(oxilean_kernel::Literal::Str(s.clone()))
        }
        SurfaceExpr::App(f, a) => Expr::App(
            Box::new(surface_to_placeholder(&f.value)),
            Box::new(surface_to_placeholder(&a.value)),
        ),
        SurfaceExpr::Ann(e, _ty) => surface_to_placeholder(&e.value),
        SurfaceExpr::Proj(base, field) => Expr::Proj(
            Name::str(field),
            0,
            Box::new(surface_to_placeholder(&base.value)),
        ),
        SurfaceExpr::Hole => Expr::Sort(Level::succ(Level::zero())),
        _ => Expr::Sort(Level::succ(Level::zero())),
    }
}
/// Count the number of pattern variables in a surface pattern.
#[allow(dead_code)]
fn count_pattern_vars(pat: &Pattern) -> usize {
    match pat {
        Pattern::Wild => 0,
        Pattern::Var(_) => 1,
        Pattern::Ctor(_, sub) => sub.iter().map(|p| count_pattern_vars(&p.value)).sum(),
        Pattern::Lit(_) => 0,
        Pattern::Or(a, b) => count_pattern_vars(&a.value) + count_pattern_vars(&b.value),
    }
}
/// Check if two elaborated patterns are structurally equal.
#[allow(dead_code)]
fn elab_pattern_eq(a: &ElabPattern, b: &ElabPattern) -> bool {
    a == b
}
/// Collect all constructor names from an elaborated pattern.
#[allow(dead_code)]
fn elab_pattern_ctors(pat: &ElabPattern) -> Vec<Name> {
    let mut result = Vec::new();
    collect_elab_ctors(pat, &mut result);
    result
}
/// Helper: collect constructor names from an elaborated pattern.
fn collect_elab_ctors(pat: &ElabPattern, out: &mut Vec<Name>) {
    match pat {
        ElabPattern::Ctor(name, sub, _) => {
            if !out.contains(name) {
                out.push(name.clone());
            }
            for s in sub {
                collect_elab_ctors(s, out);
            }
        }
        ElabPattern::Or(a, b) => {
            collect_elab_ctors(a, out);
            collect_elab_ctors(b, out);
        }
        ElabPattern::As(_, _, inner) => collect_elab_ctors(inner, out),
        ElabPattern::Wild
        | ElabPattern::Var(_, _, _)
        | ElabPattern::Lit(_)
        | ElabPattern::Inaccessible(_) => {}
    }
}
/// Flatten nested or-patterns into a list.
#[allow(dead_code)]
fn flatten_or_pattern(pat: &ElabPattern) -> Vec<&ElabPattern> {
    match pat {
        ElabPattern::Or(a, b) => {
            let mut result = flatten_or_pattern(a);
            result.extend(flatten_or_pattern(b));
            result
        }
        other => vec![other],
    }
}
/// Count the total number of sub-patterns in an elaborated pattern.
#[allow(dead_code)]
fn elab_pattern_size(pat: &ElabPattern) -> usize {
    match pat {
        ElabPattern::Wild | ElabPattern::Lit(_) | ElabPattern::Inaccessible(_) => 1,
        ElabPattern::Var(_, _, _) => 1,
        ElabPattern::Ctor(_, sub, _) => 1 + sub.iter().map(elab_pattern_size).sum::<usize>(),
        ElabPattern::Or(a, b) => elab_pattern_size(a) + elab_pattern_size(b),
        ElabPattern::As(_, _, inner) => 1 + elab_pattern_size(inner),
    }
}
/// Check if an elaborated pattern contains any or-patterns.
#[allow(dead_code)]
fn has_or_pattern(pat: &ElabPattern) -> bool {
    match pat {
        ElabPattern::Or(_, _) => true,
        ElabPattern::Ctor(_, sub, _) => sub.iter().any(has_or_pattern),
        ElabPattern::As(_, _, inner) => has_or_pattern(inner),
        _ => false,
    }
}
/// Create an elaborated constructor pattern with no sub-patterns.
#[allow(dead_code)]
fn mk_ctor_pattern(name: &str, ty: Expr) -> ElabPattern {
    ElabPattern::Ctor(Name::str(name), Vec::new(), ty)
}
/// Create an elaborated variable pattern.
#[allow(dead_code)]
fn mk_var_pattern(name: &str, id: u64, ty: Expr) -> ElabPattern {
    ElabPattern::Var(FVarId(id), Name::str(name), ty)
}
/// Map constructors to their arities from an environment.
#[allow(dead_code)]
fn ctor_arities(env: &Environment, type_name: &Name) -> HashMap<Name, usize> {
    let mut result = HashMap::new();
    if let Some(ind) = env.get_inductive_val(type_name) {
        for ctor_name in &ind.ctors {
            if let Some(ctor_val) = env.get_constructor_val(ctor_name) {
                result.insert(ctor_name.clone(), ctor_val.num_fields as usize);
            }
        }
    }
    result
}
/// Check if a pattern matrix column is all wildcards.
#[allow(dead_code)]
fn column_all_wild(matrix: &PatternMatrix, col: usize) -> bool {
    matrix.rows.iter().all(|row| {
        col < row.patterns.len()
            && matches!(
                &row.patterns[col],
                ElabPattern::Wild | ElabPattern::Var(_, _, _)
            )
    })
}
/// Substitute a variable binding in an expression (placeholder).
#[allow(dead_code)]
fn subst_var(_expr: &Expr, _fvar: FVarId, _replacement: &Expr) -> Expr {
    _expr.clone()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern_match::*;
    use oxilean_parse::Span;
    fn mk_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn mk_wild() -> Located<Pattern> {
        Located::new(Pattern::Wild, mk_span())
    }
    fn mk_var_pat(name: &str) -> Located<Pattern> {
        Located::new(Pattern::Var(name.to_string()), mk_span())
    }
    fn mk_ctor_pat(name: &str, sub: Vec<Located<Pattern>>) -> Located<Pattern> {
        Located::new(Pattern::Ctor(name.to_string(), sub), mk_span())
    }
    fn mk_lit_pat(n: u64) -> Located<Pattern> {
        Located::new(Pattern::Lit(oxilean_parse::Literal::Nat(n)), mk_span())
    }
    fn mk_or_pat(a: Located<Pattern>, b: Located<Pattern>) -> Located<Pattern> {
        Located::new(Pattern::Or(Box::new(a), Box::new(b)), mk_span())
    }
    fn mk_rhs(n: u64) -> Located<SurfaceExpr> {
        Located::new(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(n)), mk_span())
    }
    fn placeholder_ty() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    #[test]
    fn test_elaborate_wild_pattern() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let (elab, bindings) = elaborate_pattern(&mut ctx, &Pattern::Wild, &placeholder_ty())
            .expect("elaboration should succeed");
        assert!(matches!(elab, ElabPattern::Wild));
        assert!(bindings.is_empty());
    }
    #[test]
    fn test_elaborate_var_pattern() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let (elab, bindings) =
            elaborate_pattern(&mut ctx, &Pattern::Var("x".to_string()), &placeholder_ty())
                .expect("elaboration should succeed");
        match elab {
            ElabPattern::Var(_, name, _) => assert_eq!(name, Name::str("x")),
            _ => panic!("expected Var pattern"),
        }
        assert_eq!(bindings.len(), 1);
    }
    #[test]
    fn test_elaborate_ctor_pattern() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let sub = vec![Located::new(Pattern::Var("a".to_string()), mk_span())];
        let pat = Pattern::Ctor("Some".to_string(), sub);
        let (elab, bindings) = elaborate_pattern(&mut ctx, &pat, &placeholder_ty())
            .expect("elaboration should succeed");
        match elab {
            ElabPattern::Ctor(name, subs, _) => {
                assert_eq!(name, Name::str("Some"));
                assert_eq!(subs.len(), 1);
            }
            _ => panic!("expected Ctor pattern"),
        }
        assert_eq!(bindings.len(), 1);
    }
    #[test]
    fn test_elaborate_lit_pattern() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let pat = Pattern::Lit(oxilean_parse::Literal::Nat(42));
        let (elab, bindings) = elaborate_pattern(&mut ctx, &pat, &placeholder_ty())
            .expect("elaboration should succeed");
        match elab {
            ElabPattern::Lit(oxilean_kernel::Literal::Nat(42)) => {}
            _ => panic!("expected Lit(42) pattern"),
        }
        assert!(bindings.is_empty());
    }
    #[test]
    fn test_elaborate_or_pattern() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let p1 = Located::new(Pattern::Lit(oxilean_parse::Literal::Nat(1)), mk_span());
        let p2 = Located::new(Pattern::Lit(oxilean_parse::Literal::Nat(2)), mk_span());
        let pat = Pattern::Or(Box::new(p1), Box::new(p2));
        let (elab, _) = elaborate_pattern(&mut ctx, &pat, &placeholder_ty())
            .expect("elaboration should succeed");
        assert!(matches!(elab, ElabPattern::Or(_, _)));
    }
    #[test]
    fn test_exhaustive_wildcard() {
        let patterns = vec![mk_wild()];
        let result = check_exhaustive_full(&patterns);
        assert!(result.is_exhaustive);
        assert!(result.missing.is_empty());
    }
    #[test]
    fn test_exhaustive_var() {
        let patterns = vec![mk_var_pat("x")];
        let result = check_exhaustive_full(&patterns);
        assert!(result.is_exhaustive);
    }
    #[test]
    fn test_exhaustive_empty() {
        let patterns: Vec<Located<Pattern>> = Vec::new();
        let result = check_exhaustive_full(&patterns);
        assert!(!result.is_exhaustive);
        assert!(!result.missing.is_empty());
    }
    #[test]
    fn test_exhaustive_with_ctors() {
        let patterns = vec![mk_ctor_pat("Some", vec![mk_wild()])];
        let all_ctors = vec![Name::str("Some"), Name::str("None")];
        let result = check_exhaustive_with_ctors(&patterns, &all_ctors);
        assert!(!result.is_exhaustive);
        assert_eq!(result.missing.len(), 1);
        assert_eq!(result.missing[0].ctor_name, Name::str("None"));
    }
    #[test]
    fn test_exhaustive_with_ctors_complete() {
        let patterns = vec![
            mk_ctor_pat("Some", vec![mk_wild()]),
            mk_ctor_pat("None", vec![]),
        ];
        let all_ctors = vec![Name::str("Some"), Name::str("None")];
        let result = check_exhaustive_with_ctors(&patterns, &all_ctors);
        assert!(result.is_exhaustive);
    }
    #[test]
    fn test_check_exhaustive_simple_ok() {
        let patterns = vec![mk_wild()];
        assert!(check_exhaustive(&patterns).is_ok());
    }
    #[test]
    fn test_check_exhaustive_simple_empty() {
        let patterns: Vec<Located<Pattern>> = Vec::new();
        assert!(check_exhaustive(&patterns).is_err());
    }
    #[test]
    fn test_no_redundancy() {
        let patterns = vec![
            mk_ctor_pat("Some", vec![mk_wild()]),
            mk_ctor_pat("None", vec![]),
        ];
        let redundant = check_redundant_full(&patterns);
        assert!(redundant.is_empty());
    }
    #[test]
    fn test_redundant_after_wildcard() {
        let patterns = vec![mk_wild(), mk_ctor_pat("Some", vec![mk_wild()])];
        let redundant = check_redundant_full(&patterns);
        assert_eq!(redundant, vec![1]);
    }
    #[test]
    fn test_redundant_duplicate_pattern() {
        let patterns = vec![mk_lit_pat(1), mk_lit_pat(1), mk_wild()];
        let redundant = check_redundant_full(&patterns);
        assert_eq!(redundant, vec![1]);
    }
    #[test]
    fn test_redundant_after_var() {
        let patterns = vec![mk_var_pat("x"), mk_lit_pat(42)];
        let redundant = check_redundant_full(&patterns);
        assert_eq!(redundant, vec![1]);
    }
    #[test]
    fn test_or_pattern_exhaustiveness() {
        let patterns = vec![mk_or_pat(mk_lit_pat(1), mk_lit_pat(2)), mk_wild()];
        let result = check_exhaustive_full(&patterns);
        assert!(result.is_exhaustive);
    }
    #[test]
    fn test_check_redundant_empty() {
        let patterns: Vec<Located<Pattern>> = Vec::new();
        let redundant = check_redundant(&patterns);
        assert!(redundant.is_empty());
    }
    #[test]
    fn test_decision_tree_empty() {
        let arms: Vec<(Located<Pattern>, Located<SurfaceExpr>)> = Vec::new();
        let tree = DecisionTree::new(&arms).expect("test operation should succeed");
        let _expr = tree.compile();
    }
    #[test]
    fn test_decision_tree_single_wild() {
        let arms = vec![(mk_wild(), mk_rhs(42))];
        let tree = DecisionTree::new(&arms).expect("test operation should succeed");
        let expr = tree.compile();
        assert!(matches!(expr, Expr::Lit(oxilean_kernel::Literal::Nat(42))));
    }
    #[test]
    fn test_decision_tree_ctor() {
        let arms = vec![
            (mk_ctor_pat("True", vec![]), mk_rhs(1)),
            (mk_ctor_pat("False", vec![]), mk_rhs(0)),
        ];
        let tree = DecisionTree::new(&arms).expect("test operation should succeed");
        let _expr = tree.compile();
    }
    #[test]
    fn test_decision_tree_from_equations() {
        let equations = vec![MatchEquation {
            patterns: vec![ElabPattern::Wild],
            rhs: Expr::Lit(oxilean_kernel::Literal::Nat(99)),
            arm_idx: 0,
        }];
        let tree = DecisionTree::from_equations(&equations).expect("test operation should succeed");
        let expr = tree.compile();
        assert!(matches!(expr, Expr::Lit(oxilean_kernel::Literal::Nat(99))));
    }
    #[test]
    fn test_nested_ctor_pattern() {
        let inner = mk_ctor_pat("Pair", vec![mk_var_pat("a"), mk_var_pat("b")]);
        let outer = mk_ctor_pat("Some", vec![inner]);
        let arms = vec![(outer, mk_rhs(1)), (mk_wild(), mk_rhs(0))];
        let tree = DecisionTree::new(&arms).expect("test operation should succeed");
        let _expr = tree.compile();
    }
    #[test]
    fn test_elaborate_nested_pattern() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let inner = Located::new(Pattern::Var("x".to_string()), mk_span());
        let pat = Pattern::Ctor("Some".to_string(), vec![inner]);
        let (elab, bindings) = elaborate_pattern(&mut ctx, &pat, &placeholder_ty())
            .expect("elaboration should succeed");
        match elab {
            ElabPattern::Ctor(_, subs, _) => {
                assert_eq!(subs.len(), 1);
                assert!(matches!(&subs[0], ElabPattern::Var(_, _, _)));
            }
            _ => panic!("expected Ctor"),
        }
        assert_eq!(bindings.len(), 1);
    }
    #[test]
    fn test_match_arms_with_guards() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut compiler = PatternCompiler::new();
        let arms = vec![
            MatchArm {
                pattern: mk_var_pat("x"),
                guard: Some(Located::new(
                    SurfaceExpr::Var("cond".to_string()),
                    mk_span(),
                )),
                rhs: mk_rhs(1),
            },
            MatchArm {
                pattern: mk_wild(),
                guard: None,
                rhs: mk_rhs(0),
            },
        ];
        let ty = placeholder_ty();
        let result = compiler.from_match_arms(&mut ctx, &ty, &arms);
        assert!(result.is_ok());
    }
    #[test]
    fn test_pattern_vars_wild() {
        let vars = pattern_vars(&ElabPattern::Wild);
        assert!(vars.is_empty());
    }
    #[test]
    fn test_pattern_vars_var() {
        let pat = ElabPattern::Var(FVarId(0), Name::str("x"), placeholder_ty());
        let vars = pattern_vars(&pat);
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].0, Name::str("x"));
    }
    #[test]
    fn test_pattern_vars_ctor() {
        let pat = ElabPattern::Ctor(
            Name::str("Pair"),
            vec![
                ElabPattern::Var(FVarId(0), Name::str("a"), placeholder_ty()),
                ElabPattern::Var(FVarId(1), Name::str("b"), placeholder_ty()),
            ],
            placeholder_ty(),
        );
        let vars = pattern_vars(&pat);
        assert_eq!(vars.len(), 2);
    }
    #[test]
    fn test_pattern_vars_as() {
        let inner = ElabPattern::Ctor(Name::str("C"), vec![], placeholder_ty());
        let pat = ElabPattern::As(FVarId(0), Name::str("x"), Box::new(inner));
        let vars = pattern_vars(&pat);
        assert_eq!(vars.len(), 1);
    }
    #[test]
    fn test_is_irrefutable_wild() {
        assert!(is_irrefutable(&ElabPattern::Wild));
    }
    #[test]
    fn test_is_irrefutable_var() {
        assert!(is_irrefutable(&ElabPattern::Var(
            FVarId(0),
            Name::str("x"),
            placeholder_ty()
        )));
    }
    #[test]
    fn test_is_irrefutable_ctor() {
        assert!(!is_irrefutable(&ElabPattern::Ctor(
            Name::str("Some"),
            vec![],
            placeholder_ty()
        )));
    }
    #[test]
    fn test_is_irrefutable_lit() {
        assert!(!is_irrefutable(&ElabPattern::Lit(
            oxilean_kernel::Literal::Nat(42)
        )));
    }
    #[test]
    fn test_is_irrefutable_as_wild() {
        let inner = ElabPattern::Wild;
        let pat = ElabPattern::As(FVarId(0), Name::str("x"), Box::new(inner));
        assert!(is_irrefutable(&pat));
    }
    #[test]
    fn test_is_irrefutable_or_with_wild() {
        let pat = ElabPattern::Or(
            Box::new(ElabPattern::Ctor(Name::str("A"), vec![], placeholder_ty())),
            Box::new(ElabPattern::Wild),
        );
        assert!(is_irrefutable(&pat));
    }
    #[test]
    fn test_pattern_depth_wild() {
        assert_eq!(pattern_depth(&ElabPattern::Wild), 0);
    }
    #[test]
    fn test_pattern_depth_nested() {
        let inner = ElabPattern::Ctor(Name::str("B"), vec![], placeholder_ty());
        let outer = ElabPattern::Ctor(Name::str("A"), vec![inner], placeholder_ty());
        assert_eq!(pattern_depth(&outer), 2);
    }
    #[test]
    fn test_compiler_create() {
        let compiler = PatternCompiler::new();
        assert_eq!(compiler.next_var, 0);
    }
    #[test]
    fn test_fresh_var() {
        let mut compiler = PatternCompiler::new();
        let v1 = compiler.fresh_var();
        let v2 = compiler.fresh_var();
        assert_ne!(v1, v2);
    }
    #[test]
    fn test_compiler_compile_empty() {
        let mut compiler = PatternCompiler::new();
        let arms: Vec<(Located<Pattern>, Located<SurfaceExpr>)> = Vec::new();
        let tree = compiler
            .compile(&arms)
            .expect("test operation should succeed");
        let _expr = tree.compile();
    }
    #[test]
    fn test_pattern_matrix_from_equations() {
        let equations = vec![
            MatchEquation {
                patterns: vec![ElabPattern::Ctor(
                    Name::str("Some"),
                    vec![ElabPattern::Wild],
                    placeholder_ty(),
                )],
                rhs: Expr::BVar(0),
                arm_idx: 0,
            },
            MatchEquation {
                patterns: vec![ElabPattern::Ctor(
                    Name::str("None"),
                    vec![],
                    placeholder_ty(),
                )],
                rhs: Expr::BVar(1),
                arm_idx: 1,
            },
        ];
        let matrix = PatternMatrix::from_equations(&equations);
        assert_eq!(matrix.rows.len(), 2);
        assert_eq!(matrix.num_cols, 1);
        assert!(!matrix.is_empty());
    }
    #[test]
    fn test_pattern_matrix_specialize() {
        let equations = vec![
            MatchEquation {
                patterns: vec![ElabPattern::Ctor(
                    Name::str("Some"),
                    vec![ElabPattern::Wild],
                    placeholder_ty(),
                )],
                rhs: Expr::BVar(0),
                arm_idx: 0,
            },
            MatchEquation {
                patterns: vec![ElabPattern::Wild],
                rhs: Expr::BVar(1),
                arm_idx: 1,
            },
        ];
        let matrix = PatternMatrix::from_equations(&equations);
        let specialized = matrix.specialize(0, &Name::str("Some"), 1);
        assert_eq!(specialized.rows.len(), 2);
    }
    #[test]
    fn test_pattern_matrix_default() {
        let equations = vec![
            MatchEquation {
                patterns: vec![ElabPattern::Ctor(Name::str("A"), vec![], placeholder_ty())],
                rhs: Expr::BVar(0),
                arm_idx: 0,
            },
            MatchEquation {
                patterns: vec![ElabPattern::Wild],
                rhs: Expr::BVar(1),
                arm_idx: 1,
            },
        ];
        let matrix = PatternMatrix::from_equations(&equations);
        let default = matrix.default_matrix(0);
        assert_eq!(default.rows.len(), 1);
    }
    #[test]
    fn test_pattern_matrix_best_column() {
        let equations = vec![MatchEquation {
            patterns: vec![
                ElabPattern::Wild,
                ElabPattern::Ctor(Name::str("A"), vec![], placeholder_ty()),
            ],
            rhs: Expr::BVar(0),
            arm_idx: 0,
        }];
        let matrix = PatternMatrix::from_equations(&equations);
        let best = matrix.best_column();
        assert_eq!(best, 1);
    }
    #[test]
    fn test_elaborate_match_simple() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let scrutinee = Located::new(SurfaceExpr::Var("x".to_string()), mk_span());
        let arms = vec![(mk_wild(), mk_rhs(0))];
        let result = elaborate_match(&mut ctx, &scrutinee, &arms);
        assert!(result.is_ok());
        let mr = result.expect("test operation should succeed");
        assert!(mr.redundant_arms.is_empty());
        assert!(mr.missing_patterns.is_empty());
    }
    #[test]
    fn test_count_pattern_vars_surface() {
        assert_eq!(count_pattern_vars(&Pattern::Wild), 0);
        assert_eq!(count_pattern_vars(&Pattern::Var("x".to_string())), 1);
        assert_eq!(
            count_pattern_vars(&Pattern::Ctor(
                "P".to_string(),
                vec![
                    Located::new(Pattern::Var("a".to_string()), mk_span()),
                    Located::new(Pattern::Var("b".to_string()), mk_span()),
                ]
            )),
            2
        );
    }
    #[test]
    fn test_elab_pattern_size() {
        assert_eq!(elab_pattern_size(&ElabPattern::Wild), 1);
        let nested = ElabPattern::Ctor(
            Name::str("A"),
            vec![ElabPattern::Wild, ElabPattern::Wild],
            placeholder_ty(),
        );
        assert_eq!(elab_pattern_size(&nested), 3);
    }
    #[test]
    fn test_has_or_pattern() {
        assert!(!has_or_pattern(&ElabPattern::Wild));
        let or_pat = ElabPattern::Or(Box::new(ElabPattern::Wild), Box::new(ElabPattern::Wild));
        assert!(has_or_pattern(&or_pat));
    }
    #[test]
    fn test_flatten_or() {
        let or_pat = ElabPattern::Or(
            Box::new(ElabPattern::Lit(oxilean_kernel::Literal::Nat(1))),
            Box::new(ElabPattern::Or(
                Box::new(ElabPattern::Lit(oxilean_kernel::Literal::Nat(2))),
                Box::new(ElabPattern::Lit(oxilean_kernel::Literal::Nat(3))),
            )),
        );
        let flat = flatten_or_pattern(&or_pat);
        assert_eq!(flat.len(), 3);
    }
    #[test]
    fn test_elab_pattern_ctors() {
        let pat = ElabPattern::Ctor(
            Name::str("A"),
            vec![ElabPattern::Ctor(Name::str("B"), vec![], placeholder_ty())],
            placeholder_ty(),
        );
        let ctors = elab_pattern_ctors(&pat);
        assert_eq!(ctors.len(), 2);
        assert!(ctors.contains(&Name::str("A")));
        assert!(ctors.contains(&Name::str("B")));
    }
    #[test]
    fn test_constructors_for_type_empty() {
        let env = Environment::new();
        let ty = Expr::Const(Name::str("Unknown"), Vec::new());
        let ctors = constructors_for_type(&env, &ty);
        assert!(ctors.is_empty());
    }
    #[test]
    fn test_mk_helper_patterns() {
        let ty = placeholder_ty();
        let ctor = mk_ctor_pattern("Some", ty.clone());
        assert!(matches!(ctor, ElabPattern::Ctor(_, _, _)));
        let var = mk_var_pattern("x", 0, ty);
        assert!(matches!(var, ElabPattern::Var(_, _, _)));
    }
    #[test]
    fn test_pattern_subsumes() {
        assert!(pattern_subsumes(
            &Pattern::Wild,
            &Pattern::Var("x".to_string())
        ));
        assert!(pattern_subsumes(
            &Pattern::Wild,
            &Pattern::Lit(oxilean_parse::Literal::Nat(1))
        ));
        assert!(!pattern_subsumes(
            &Pattern::Lit(oxilean_parse::Literal::Nat(1)),
            &Pattern::Lit(oxilean_parse::Literal::Nat(2))
        ));
        assert!(pattern_subsumes(
            &Pattern::Lit(oxilean_parse::Literal::Nat(1)),
            &Pattern::Lit(oxilean_parse::Literal::Nat(1))
        ));
    }
}
/// Helper: elaborate a pattern using an explicit counter.
#[allow(dead_code, clippy::type_complexity)]
pub fn elab_pattern_with_counter(
    pat: &Pattern,
    ty: &Expr,
    counter: &mut u64,
) -> Result<(ElabPattern, Vec<(FVarId, Name, Expr)>), String> {
    match pat {
        Pattern::Wild => Ok((ElabPattern::Wild, Vec::new())),
        Pattern::Var(name) => {
            let fv = FVarId(*counter);
            *counter += 1;
            let n = Name::str(name);
            Ok((
                ElabPattern::Var(fv, n.clone(), ty.clone()),
                vec![(fv, n, ty.clone())],
            ))
        }
        Pattern::Ctor(name, sub) => {
            let mut all_bindings = Vec::new();
            let mut elab_sub = Vec::new();
            for sp in sub {
                let sub_ty = Expr::Sort(Level::succ(Level::zero()));
                let (ep, binds) = elab_pattern_with_counter(&sp.value, &sub_ty, counter)?;
                all_bindings.extend(binds);
                elab_sub.push(ep);
            }
            Ok((
                ElabPattern::Ctor(Name::str(name), elab_sub, ty.clone()),
                all_bindings,
            ))
        }
        Pattern::Lit(lit) => {
            let klit = convert_lit_helper(lit);
            Ok((ElabPattern::Lit(klit), Vec::new()))
        }
        Pattern::Or(a, b) => {
            let (ea, ba) = elab_pattern_with_counter(&a.value, ty, counter)?;
            let (eb, bb) = elab_pattern_with_counter(&b.value, ty, counter)?;
            let mut all = ba;
            all.extend(bb);
            Ok((ElabPattern::Or(Box::new(ea), Box::new(eb)), all))
        }
    }
}
/// Convert a surface literal to a kernel literal (internal helper variant).
#[allow(dead_code)]
fn convert_lit_helper(lit: &oxilean_parse::Literal) -> oxilean_kernel::Literal {
    match lit {
        oxilean_parse::Literal::Nat(n) => oxilean_kernel::Literal::Nat(*n),
        oxilean_parse::Literal::String(s) => oxilean_kernel::Literal::Str(s.clone()),
        oxilean_parse::Literal::Char(_) => oxilean_kernel::Literal::Str("?".to_string()),
        oxilean_parse::Literal::Float(_) => oxilean_kernel::Literal::Nat(0),
    }
}
/// Check structural equivalence of two elaborated patterns (ignoring FVarIds).
#[allow(dead_code)]
pub fn patterns_structurally_equal(a: &ElabPattern, b: &ElabPattern) -> bool {
    match (a, b) {
        (ElabPattern::Wild, ElabPattern::Wild) => true,
        (ElabPattern::Var(_, _, _), ElabPattern::Var(_, _, _)) => true,
        (ElabPattern::Lit(la), ElabPattern::Lit(lb)) => la == lb,
        (ElabPattern::Ctor(na, sa, _), ElabPattern::Ctor(nb, sb, _)) => {
            na == nb
                && sa.len() == sb.len()
                && sa
                    .iter()
                    .zip(sb.iter())
                    .all(|(x, y)| patterns_structurally_equal(x, y))
        }
        (ElabPattern::Or(a1, a2), ElabPattern::Or(b1, b2)) => {
            patterns_structurally_equal(a1, b1) && patterns_structurally_equal(a2, b2)
        }
        (ElabPattern::As(_, _, ai), ElabPattern::As(_, _, bi)) => {
            patterns_structurally_equal(ai, bi)
        }
        (ElabPattern::Inaccessible(_), ElabPattern::Inaccessible(_)) => true,
        _ => false,
    }
}
/// Convert a pattern to a placeholder kernel expression (for code generation).
#[allow(dead_code)]
pub fn pattern_to_expr(pat: &ElabPattern) -> Expr {
    match pat {
        ElabPattern::Wild => Expr::Sort(Level::zero()),
        ElabPattern::Var(fv, _, ty) => {
            let _ = fv;
            ty.clone()
        }
        ElabPattern::Lit(lit) => Expr::Lit(lit.clone()),
        ElabPattern::Ctor(name, _, _) => Expr::Const(name.clone(), vec![]),
        ElabPattern::Or(a, _) => pattern_to_expr(a),
        ElabPattern::As(_, _, inner) => pattern_to_expr(inner),
        ElabPattern::Inaccessible(e) => e.clone(),
    }
}
/// Count the total number of constructor patterns in a list of patterns.
#[allow(dead_code)]
pub fn count_ctor_patterns(patterns: &[ElabPattern]) -> usize {
    patterns.iter().map(count_ctor_in_pattern).sum()
}
fn count_ctor_in_pattern(pat: &ElabPattern) -> usize {
    match pat {
        ElabPattern::Ctor(_, sub, _) => 1 + sub.iter().map(count_ctor_in_pattern).sum::<usize>(),
        ElabPattern::Or(a, b) => count_ctor_in_pattern(a) + count_ctor_in_pattern(b),
        ElabPattern::As(_, _, inner) => count_ctor_in_pattern(inner),
        _ => 0,
    }
}
/// Count the total number of literal patterns in a list.
#[allow(dead_code)]
pub fn count_lit_patterns(patterns: &[ElabPattern]) -> usize {
    patterns.iter().map(count_lit_in_pattern).sum()
}
fn count_lit_in_pattern(pat: &ElabPattern) -> usize {
    match pat {
        ElabPattern::Lit(_) => 1,
        ElabPattern::Ctor(_, sub, _) => sub.iter().map(count_lit_in_pattern).sum(),
        ElabPattern::Or(a, b) => count_lit_in_pattern(a) + count_lit_in_pattern(b),
        ElabPattern::As(_, _, inner) => count_lit_in_pattern(inner),
        _ => 0,
    }
}
/// Build a wildcard pattern for a given expected type.
#[allow(dead_code)]
pub fn wild_pattern_for(ty: &Expr) -> ElabPattern {
    let _ = ty;
    ElabPattern::Wild
}
/// Build an as-pattern from a variable ID, name, and inner pattern.
#[allow(dead_code)]
pub fn mk_as_pattern(id: u64, name: &str, inner: ElabPattern) -> ElabPattern {
    ElabPattern::As(FVarId(id), Name::str(name), Box::new(inner))
}
/// Build an inaccessible pattern from an expression.
#[allow(dead_code)]
pub fn mk_inaccessible(expr: Expr) -> ElabPattern {
    ElabPattern::Inaccessible(expr)
}
/// Check if two pattern lists are pairwise structurally equal.
#[allow(dead_code)]
pub fn pattern_lists_equal(a: &[ElabPattern], b: &[ElabPattern]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b.iter())
            .all(|(x, y)| patterns_structurally_equal(x, y))
}
/// Collect all literal values from a list of patterns.
#[allow(dead_code)]
pub fn collect_literals(patterns: &[ElabPattern]) -> Vec<oxilean_kernel::Literal> {
    let mut result = Vec::new();
    for pat in patterns {
        collect_lit_recursive(pat, &mut result);
    }
    result
}
fn collect_lit_recursive(pat: &ElabPattern, out: &mut Vec<oxilean_kernel::Literal>) {
    match pat {
        ElabPattern::Lit(l) => {
            if !out.contains(l) {
                out.push(l.clone());
            }
        }
        ElabPattern::Ctor(_, sub, _) => {
            for s in sub {
                collect_lit_recursive(s, out);
            }
        }
        ElabPattern::Or(a, b) => {
            collect_lit_recursive(a, out);
            collect_lit_recursive(b, out);
        }
        ElabPattern::As(_, _, inner) => collect_lit_recursive(inner, out),
        _ => {}
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::pattern_match::*;
    use oxilean_kernel::Level;
    use oxilean_parse::Span;
    fn mk_span() -> Span {
        Span::new(0, 1, 1, 1)
    }
    fn nat_ty() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    fn mk_nat_lit(n: u64) -> ElabPattern {
        ElabPattern::Lit(oxilean_kernel::Literal::Nat(n))
    }
    fn mk_str_lit(s: &str) -> ElabPattern {
        ElabPattern::Lit(oxilean_kernel::Literal::Str(s.to_string()))
    }
    #[test]
    fn test_pattern_kind_of() {
        assert_eq!(PatternKind::of(&ElabPattern::Wild), PatternKind::Wild);
        assert_eq!(
            PatternKind::of(&ElabPattern::Var(FVarId(0), Name::str("x"), nat_ty())),
            PatternKind::Variable
        );
        assert_eq!(
            PatternKind::of(&ElabPattern::Ctor(Name::str("A"), vec![], nat_ty())),
            PatternKind::Constructor
        );
        assert_eq!(PatternKind::of(&mk_nat_lit(0)), PatternKind::Literal);
    }
    #[test]
    fn test_pattern_kind_can_bind() {
        assert!(PatternKind::Variable.can_bind());
        assert!(PatternKind::As.can_bind());
        assert!(!PatternKind::Wild.can_bind());
        assert!(!PatternKind::Literal.can_bind());
    }
    #[test]
    fn test_pattern_kind_is_catch_all() {
        assert!(PatternKind::Wild.is_catch_all());
        assert!(PatternKind::Variable.is_catch_all());
        assert!(!PatternKind::Constructor.is_catch_all());
    }
    #[test]
    fn test_pattern_kind_display() {
        let s = format!("{}", PatternKind::Constructor);
        assert_eq!(s, "Constructor");
    }
    #[test]
    fn test_pattern_stats_from_patterns() {
        let patterns = vec![
            ElabPattern::Wild,
            ElabPattern::Var(FVarId(0), Name::str("x"), nat_ty()),
            mk_nat_lit(42),
            ElabPattern::Ctor(Name::str("Some"), vec![mk_nat_lit(1)], nat_ty()),
        ];
        let stats = PatternStats::from_patterns(&patterns);
        assert_eq!(stats.total, 4);
        assert_eq!(stats.wilds, 1);
        assert_eq!(stats.variables, 1);
        assert_eq!(stats.literals, 2);
        assert_eq!(stats.constructors, 1);
        assert!(stats.max_depth >= 1);
    }
    #[test]
    fn test_pattern_normalizer_as_wildcard() {
        let norm = PatternNormalizer::new();
        let wild = ElabPattern::Wild;
        let as_wild = ElabPattern::As(FVarId(0), Name::str("x"), Box::new(wild.clone()));
        let result = norm.normalize(&as_wild);
        assert!(matches!(result, ElabPattern::Wild));
    }
    #[test]
    fn test_pattern_normalizer_ctor() {
        let norm = PatternNormalizer::new();
        let ctor = ElabPattern::Ctor(
            Name::str("Pair"),
            vec![ElabPattern::Wild, ElabPattern::Wild],
            nat_ty(),
        );
        let result = norm.normalize(&ctor);
        assert!(matches!(result, ElabPattern::Ctor(_, _, _)));
    }
    #[test]
    fn test_pattern_normalizer_identical_or() {
        let norm = PatternNormalizer::new();
        let wild1 = ElabPattern::Wild;
        let wild2 = ElabPattern::Wild;
        let or_pat = ElabPattern::Or(Box::new(wild1), Box::new(wild2));
        let result = norm.normalize(&or_pat);
        assert!(matches!(result, ElabPattern::Wild));
    }
    #[test]
    fn test_literal_set_nats() {
        let mut ls = LiteralSet::new();
        ls.add_literal(&oxilean_kernel::Literal::Nat(1));
        ls.add_literal(&oxilean_kernel::Literal::Nat(2));
        ls.add_literal(&oxilean_kernel::Literal::Nat(1));
        assert_eq!(ls.nats.len(), 2);
        assert!(ls.covers_nat(1));
        assert!(!ls.covers_nat(3));
    }
    #[test]
    fn test_literal_set_wildcard() {
        let mut ls = LiteralSet::new();
        ls.add_wildcard();
        assert!(ls.covers_nat(99999));
    }
    #[test]
    fn test_literal_set_strings() {
        let mut ls = LiteralSet::new();
        ls.add_literal(&oxilean_kernel::Literal::Str("hello".to_string()));
        ls.add_literal(&oxilean_kernel::Literal::Str("hello".to_string()));
        assert_eq!(ls.strings.len(), 1);
        assert_eq!(ls.num_specific(), 1);
    }
    #[test]
    fn test_match_coverage_basic() {
        let mut cov = MatchCoverage::new(3);
        assert!(!cov.is_full_coverage());
        assert_eq!(cov.coverage_pct(), 0.0);
        cov.record_hit(0);
        cov.record_hit(2);
        assert_eq!(cov.arms_hit(), 2);
        let uncov = cov.uncovered_arms();
        assert_eq!(uncov, vec![1]);
    }
    #[test]
    fn test_match_coverage_full() {
        let mut cov = MatchCoverage::new(2);
        cov.record_hit(0);
        cov.record_hit(1);
        assert!(cov.is_full_coverage());
        assert_eq!(cov.coverage_pct(), 1.0);
    }
    #[test]
    fn test_match_coverage_oob() {
        let mut cov = MatchCoverage::new(2);
        cov.record_hit(99);
        assert_eq!(cov.arms_hit(), 0);
    }
    #[test]
    fn test_match_coverage_zero_arms() {
        let cov = MatchCoverage::new(0);
        assert_eq!(cov.coverage_pct(), 1.0);
    }
    #[test]
    fn test_pattern_substitution_bind_get() {
        let mut subst = PatternSubstitution::new();
        let fv = FVarId(42);
        let expr = Expr::Const(Name::str("x"), vec![]);
        subst.bind(fv, expr.clone());
        assert_eq!(subst.len(), 1);
        assert!(!subst.is_empty());
        assert_eq!(subst.get(&fv), Some(&expr));
        assert!(subst.get(&FVarId(99)).is_none());
    }
    #[test]
    fn test_pattern_substitution_merge() {
        let mut s1 = PatternSubstitution::new();
        let mut s2 = PatternSubstitution::new();
        s1.bind(FVarId(0), Expr::Const(Name::str("a"), vec![]));
        s2.bind(FVarId(1), Expr::Const(Name::str("b"), vec![]));
        s1.merge(&s2);
        assert_eq!(s1.len(), 2);
    }
    #[test]
    fn test_match_elaborator_fresh_fvar() {
        let mut me = MatchElaborator::new();
        let fv1 = me.fresh_fvar();
        let fv2 = me.fresh_fvar();
        assert_ne!(fv1, fv2);
    }
    #[test]
    fn test_match_elaborator_elab_pattern() {
        let mut me = MatchElaborator::new();
        let (ep, bindings) = me
            .elab_pattern(&Pattern::Var("x".to_string()), &nat_ty())
            .expect("test operation should succeed");
        assert!(matches!(ep, ElabPattern::Var(_, _, _)));
        assert_eq!(bindings.len(), 1);
    }
    #[test]
    fn test_column_heuristic_leftmost() {
        let h = ColumnHeuristic::LeftMost;
        let matrix = PatternMatrix {
            rows: vec![],
            num_cols: 5,
        };
        assert_eq!(h.select_column(&matrix), 0);
    }
    #[test]
    fn test_patterns_structurally_equal() {
        let a = ElabPattern::Wild;
        let b = ElabPattern::Wild;
        assert!(patterns_structurally_equal(&a, &b));
        let c = ElabPattern::Var(FVarId(0), Name::str("x"), nat_ty());
        let d = ElabPattern::Var(FVarId(99), Name::str("y"), nat_ty());
        assert!(patterns_structurally_equal(&c, &d));
        assert!(!patterns_structurally_equal(&a, &c));
    }
    #[test]
    fn test_patterns_structurally_equal_ctor() {
        let a = ElabPattern::Ctor(Name::str("Some"), vec![ElabPattern::Wild], nat_ty());
        let b = ElabPattern::Ctor(Name::str("Some"), vec![ElabPattern::Wild], nat_ty());
        assert!(patterns_structurally_equal(&a, &b));
        let c = ElabPattern::Ctor(Name::str("None"), vec![], nat_ty());
        assert!(!patterns_structurally_equal(&a, &c));
    }
    #[test]
    fn test_pattern_printer_wild() {
        let pp = PatternPrinter::new();
        assert_eq!(pp.print(&ElabPattern::Wild), "_");
    }
    #[test]
    fn test_pattern_printer_var() {
        let pp = PatternPrinter::new();
        let s = pp.print(&ElabPattern::Var(FVarId(0), Name::str("foo"), nat_ty()));
        assert_eq!(s, "foo");
    }
    #[test]
    fn test_pattern_printer_lit_nat() {
        let pp = PatternPrinter::new();
        let s = pp.print(&mk_nat_lit(42));
        assert_eq!(s, "42");
    }
    #[test]
    fn test_pattern_printer_lit_str() {
        let pp = PatternPrinter::new();
        let s = pp.print(&mk_str_lit("hello"));
        assert_eq!(s, "\"hello\"");
    }
    #[test]
    fn test_pattern_printer_ctor() {
        let pp = PatternPrinter::new();
        let ctor = ElabPattern::Ctor(
            Name::str("Pair"),
            vec![mk_nat_lit(1), mk_nat_lit(2)],
            nat_ty(),
        );
        let s = pp.print(&ctor);
        assert!(s.contains("Pair"));
        assert!(s.contains("1"));
        assert!(s.contains("2"));
    }
    #[test]
    fn test_pattern_printer_or() {
        let pp = PatternPrinter::new();
        let or_pat = ElabPattern::Or(Box::new(mk_nat_lit(1)), Box::new(mk_nat_lit(2)));
        let s = pp.print(&or_pat);
        assert!(s.contains("|"));
    }
    #[test]
    fn test_pattern_printer_all() {
        let pp = PatternPrinter::new();
        let patterns = vec![ElabPattern::Wild, mk_nat_lit(0)];
        let s = pp.print_all(&patterns);
        assert!(s.contains("arm 0"));
        assert!(s.contains("arm 1"));
    }
    #[test]
    fn test_pattern_to_expr_wild() {
        let expr = pattern_to_expr(&ElabPattern::Wild);
        assert!(matches!(expr, Expr::Sort(_)));
    }
    #[test]
    fn test_pattern_to_expr_lit() {
        let expr = pattern_to_expr(&mk_nat_lit(5));
        assert!(matches!(expr, Expr::Lit(oxilean_kernel::Literal::Nat(5))));
    }
    #[test]
    fn test_pattern_matcher_literals() {
        let matcher = PatternMatcher::new();
        let lit = oxilean_kernel::Literal::Nat(7);
        assert!(matcher.matches_literal(&ElabPattern::Wild, &lit));
        assert!(matcher.matches_literal(&mk_nat_lit(7), &lit));
        assert!(!matcher.matches_literal(&mk_nat_lit(8), &lit));
        let or_pat = ElabPattern::Or(Box::new(mk_nat_lit(7)), Box::new(mk_nat_lit(8)));
        assert!(matcher.matches_literal(&or_pat, &lit));
    }
    #[test]
    fn test_pattern_matcher_first_match() {
        let matcher = PatternMatcher::new();
        let arms = vec![mk_nat_lit(1), mk_nat_lit(2), ElabPattern::Wild];
        let lit = oxilean_kernel::Literal::Nat(2);
        assert_eq!(matcher.first_match_idx(&arms, &lit), Some(1));
        let lit2 = oxilean_kernel::Literal::Nat(99);
        assert_eq!(matcher.first_match_idx(&arms, &lit2), Some(2));
    }
    #[test]
    fn test_pattern_matcher_all_match_idxs() {
        let matcher = PatternMatcher::new();
        let arms = vec![mk_nat_lit(1), ElabPattern::Wild, mk_nat_lit(1)];
        let lit = oxilean_kernel::Literal::Nat(1);
        let idxs = matcher.all_match_idxs(&arms, &lit);
        assert_eq!(idxs, vec![0, 1, 2]);
    }
    #[test]
    fn test_decision_tree_analyzer() {
        let mut compiler = PatternCompiler::new();
        use oxilean_parse::Literal as PL;
        let pats = vec![
            (
                Located::new(Pattern::Lit(PL::Nat(0)), mk_span()),
                Located::new(SurfaceExpr::Lit(PL::Nat(0)), mk_span()),
            ),
            (
                Located::new(Pattern::Wild, mk_span()),
                Located::new(SurfaceExpr::Lit(PL::Nat(1)), mk_span()),
            ),
        ];
        let tree = compiler
            .compile(&pats)
            .expect("test operation should succeed");
        assert_eq!(DecisionTreeAnalyzer::num_arms(&tree), 2);
        assert!(DecisionTreeAnalyzer::has_default_arm(&tree));
    }
    #[test]
    fn test_count_ctor_patterns() {
        let patterns = vec![
            ElabPattern::Ctor(
                Name::str("A"),
                vec![ElabPattern::Ctor(Name::str("B"), vec![], nat_ty())],
                nat_ty(),
            ),
            ElabPattern::Wild,
        ];
        let count = count_ctor_patterns(&patterns);
        assert_eq!(count, 2);
    }
    #[test]
    fn test_count_lit_patterns() {
        let patterns = vec![
            mk_nat_lit(1),
            mk_nat_lit(2),
            ElabPattern::Ctor(Name::str("X"), vec![mk_nat_lit(3)], nat_ty()),
        ];
        let count = count_lit_patterns(&patterns);
        assert_eq!(count, 3);
    }
    #[test]
    fn test_wild_pattern_for() {
        let ty = Expr::Sort(Level::zero());
        let pat = wild_pattern_for(&ty);
        assert!(matches!(pat, ElabPattern::Wild));
    }
    #[test]
    fn test_mk_as_pattern() {
        let inner = ElabPattern::Wild;
        let as_pat = mk_as_pattern(10, "alias", inner);
        assert!(matches!(as_pat, ElabPattern::As(FVarId(10), _, _)));
    }
    #[test]
    fn test_mk_inaccessible() {
        let e = Expr::Const(Name::str("x"), vec![]);
        let pat = mk_inaccessible(e.clone());
        assert!(matches!(pat, ElabPattern::Inaccessible(_)));
        assert_eq!(pattern_to_expr(&pat), e);
    }
    #[test]
    fn test_pattern_lists_equal() {
        let a = vec![ElabPattern::Wild, mk_nat_lit(1)];
        let b = vec![ElabPattern::Wild, mk_nat_lit(1)];
        assert!(pattern_lists_equal(&a, &b));
        let c = vec![ElabPattern::Wild, mk_nat_lit(2)];
        assert!(!pattern_lists_equal(&a, &c));
    }
    #[test]
    fn test_collect_literals() {
        let patterns = vec![
            mk_nat_lit(1),
            mk_nat_lit(2),
            mk_str_lit("foo"),
            ElabPattern::Wild,
        ];
        let lits = collect_literals(&patterns);
        assert_eq!(lits.len(), 3);
    }
    #[test]
    fn test_collect_literals_nested() {
        let patterns = vec![ElabPattern::Ctor(
            Name::str("P"),
            vec![mk_nat_lit(5), mk_str_lit("bar")],
            nat_ty(),
        )];
        let lits = collect_literals(&patterns);
        assert_eq!(lits.len(), 2);
    }
    #[test]
    fn test_elab_pattern_with_counter_or() {
        let pat = Pattern::Or(
            Box::new(Located::new(
                Pattern::Lit(oxilean_parse::Literal::Nat(1)),
                mk_span(),
            )),
            Box::new(Located::new(
                Pattern::Lit(oxilean_parse::Literal::Nat(2)),
                mk_span(),
            )),
        );
        let ty = Expr::Sort(Level::succ(Level::zero()));
        let mut counter = 0u64;
        let (ep, bindings) =
            elab_pattern_with_counter(&pat, &ty, &mut counter).expect("elaboration should succeed");
        assert!(matches!(ep, ElabPattern::Or(_, _)));
        assert!(bindings.is_empty());
    }
    #[test]
    fn test_pattern_subsumption_or() {
        let or_pat = Located::new(
            Pattern::Or(
                Box::new(Located::new(Pattern::Wild, mk_span())),
                Box::new(Located::new(
                    Pattern::Lit(oxilean_parse::Literal::Nat(1)),
                    mk_span(),
                )),
            ),
            mk_span(),
        );
        let lit_pat = Located::new(Pattern::Lit(oxilean_parse::Literal::Nat(1)), mk_span());
        let patterns = vec![or_pat, lit_pat];
        let redundant = check_redundant_full(&patterns);
        assert!(redundant.contains(&1));
    }
    #[test]
    fn test_is_irrefutable_elab() {
        assert!(is_irrefutable(&ElabPattern::Wild));
        assert!(is_irrefutable(&ElabPattern::Var(
            FVarId(0),
            Name::str("x"),
            nat_ty()
        )));
        assert!(!is_irrefutable(&mk_nat_lit(0)));
        assert!(!is_irrefutable(&ElabPattern::Ctor(
            Name::str("Some"),
            vec![],
            nat_ty()
        )));
    }
    #[test]
    fn test_pattern_depth_nested() {
        let deep = ElabPattern::Ctor(
            Name::str("A"),
            vec![ElabPattern::Ctor(
                Name::str("B"),
                vec![mk_nat_lit(1)],
                nat_ty(),
            )],
            nat_ty(),
        );
        assert_eq!(pattern_depth(&deep), 2);
    }
    #[test]
    fn test_pattern_vars_as() {
        let as_pat = ElabPattern::As(
            FVarId(0),
            Name::str("outer"),
            Box::new(ElabPattern::Var(FVarId(1), Name::str("inner"), nat_ty())),
        );
        let vars = pattern_vars(&as_pat);
        assert_eq!(vars.len(), 2);
    }
    #[test]
    fn test_env_lookup_constructors() {
        let env = Environment::new();
        let ty = Expr::Const(Name::str("Nat"), Vec::new());
        let ctors = constructors_for_type(&env, &ty);
        assert!(ctors.is_empty());
    }
}
