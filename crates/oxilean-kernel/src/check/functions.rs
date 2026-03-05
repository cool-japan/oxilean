//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::declaration::{ConstantInfo, ConstructorVal, InductiveVal, QuotVal, RecursorVal};
use crate::env::EnvError;
use crate::{Declaration, Environment, KernelError, TypeChecker};
#[cfg(test)]
use crate::{Expr, Name};

use super::types::{
    BatchCheckResult, CheckConfig, CheckStats, ConfigNode, DecisionNode, DeclKind, DeclSummary,
    Either2, EnvBuilder, FlatSubstitution, FocusStack, LabelSet, NonEmptyVec, PathBuf, RewriteRule,
    RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch,
    StringPool, TempEnvScope, TokenBucket, TransformStat, TransitiveClosure, VersionedRecord,
    WellFormedResult, WindowIterator, WriteOnce,
};

/// Check a declaration and add it to the environment.
///
/// Validates that the declaration is well-formed and type-correct, then
/// inserts it into the environment. Returns an error if the declaration
/// fails type checking or if the name is already declared.
#[allow(clippy::result_large_err)]
pub fn check_declaration(env: &mut Environment, decl: Declaration) -> Result<(), KernelError> {
    let name = decl.name().clone();
    match &decl {
        Declaration::Axiom { ty, .. } => {
            let mut tc = TypeChecker::new(env);
            tc.ensure_sort(ty)?;
        }
        Declaration::Definition { ty, val, .. } => {
            let mut tc = TypeChecker::new(env);
            tc.ensure_sort(ty)?;
            let val_ty = tc.infer_type(val)?;
            tc.check_type(val, &val_ty, ty)?;
        }
        Declaration::Theorem { ty, val, .. } => {
            let mut tc = TypeChecker::new(env);
            tc.ensure_sort(ty)?;
            let val_ty = tc.infer_type(val)?;
            tc.check_type(val, &val_ty, ty)?;
        }
        Declaration::Opaque { ty, val, .. } => {
            let mut tc = TypeChecker::new(env);
            tc.ensure_sort(ty)?;
            let val_ty = tc.infer_type(val)?;
            tc.check_type(val, &val_ty, ty)?;
        }
    }
    env.add(decl).map_err(|e| match e {
        EnvError::DuplicateDeclaration(_) => {
            KernelError::Other(format!("duplicate declaration: {}", name))
        }
        EnvError::NotFound(_) => KernelError::Other(format!("declaration not found: {}", name)),
    })
}
/// Check multiple declarations in sequence.
///
/// Processes the declarations in the given order, stopping at the first
/// error. Each successfully-checked declaration is added to the environment
/// so that later declarations can refer to earlier ones.
#[allow(clippy::result_large_err)]
pub fn check_declarations(
    env: &mut Environment,
    decls: Vec<Declaration>,
) -> Result<(), KernelError> {
    for decl in decls {
        check_declaration(env, decl)?;
    }
    Ok(())
}
/// Check a `ConstantInfo` (inductive, constructor, recursor, or quotient) and add
/// it to the environment.
///
/// Unlike `check_declaration`, this function handles the richer LEAN 4-style
/// constant info types that include inductive types, constructors, recursors, and
/// quotient type components.
#[allow(clippy::result_large_err)]
pub fn check_constant_info(env: &mut Environment, ci: ConstantInfo) -> Result<(), KernelError> {
    let name = ci.name().clone();
    match &ci {
        ConstantInfo::Axiom(av) => {
            let mut tc = TypeChecker::new(env);
            tc.ensure_sort(&av.common.ty)?;
        }
        ConstantInfo::Definition(dv) => {
            let mut tc = TypeChecker::new(env);
            tc.ensure_sort(&dv.common.ty)?;
            let val_ty = tc.infer_type(&dv.value)?;
            tc.check_type(&dv.value, &val_ty, &dv.common.ty)?;
        }
        ConstantInfo::Theorem(tv) => {
            let mut tc = TypeChecker::new(env);
            tc.ensure_sort(&tv.common.ty)?;
            let val_ty = tc.infer_type(&tv.value)?;
            tc.check_type(&tv.value, &val_ty, &tv.common.ty)?;
        }
        ConstantInfo::Opaque(ov) => {
            let mut tc = TypeChecker::new(env);
            tc.ensure_sort(&ov.common.ty)?;
            let val_ty = tc.infer_type(&ov.value)?;
            tc.check_type(&ov.value, &val_ty, &ov.common.ty)?;
        }
        ConstantInfo::Inductive(iv) => {
            check_inductive_val(env, iv)?;
        }
        ConstantInfo::Constructor(cv) => {
            check_constructor_val(env, cv)?;
        }
        ConstantInfo::Recursor(rv) => {
            check_recursor_val(env, rv)?;
        }
        ConstantInfo::Quotient(qv) => {
            check_quotient_val(env, qv)?;
        }
    }
    env.add_constant(ci).map_err(|e| match e {
        crate::env::EnvError::DuplicateDeclaration(_) => {
            KernelError::Other(format!("duplicate declaration: {}", name))
        }
        crate::env::EnvError::NotFound(_) => {
            KernelError::Other(format!("declaration not found: {}", name))
        }
    })
}
/// Check multiple `ConstantInfo` declarations, stopping on the first error.
#[allow(clippy::result_large_err)]
pub fn check_constant_infos(
    env: &mut Environment,
    cis: Vec<ConstantInfo>,
) -> Result<(), KernelError> {
    for ci in cis {
        check_constant_info(env, ci)?;
    }
    Ok(())
}
/// Validate an `InductiveVal`: verify the type is a sort.
///
/// Constructor types are checked separately via `check_constructor_val` when
/// each constructor `ConstantInfo` is added. Empty inductives (0 constructors,
/// like `Empty`) are valid.
#[allow(clippy::result_large_err)]
pub(super) fn check_inductive_val(
    env: &mut Environment,
    iv: &InductiveVal,
) -> Result<(), KernelError> {
    let mut tc = TypeChecker::new(env);
    tc.ensure_sort(&iv.common.ty)?;
    Ok(())
}
/// Validate a `ConstructorVal`: verify the constructor's type is well-formed and
/// that it returns the parent inductive type.
#[allow(clippy::result_large_err)]
pub(super) fn check_constructor_val(
    env: &mut Environment,
    cv: &ConstructorVal,
) -> Result<(), KernelError> {
    {
        let mut tc = TypeChecker::new(env);
        tc.ensure_sort(&cv.common.ty)?;
    }
    if !constructor_returns_inductive(&cv.common.ty, &cv.induct) {
        return Err(KernelError::InvalidInductive(format!(
            "constructor `{}` does not return inductive type `{}`",
            cv.common.name, cv.induct
        )));
    }
    Ok(())
}
/// Validate a `RecursorVal`: verify the recursor type is well-formed.
#[allow(clippy::result_large_err)]
pub(super) fn check_recursor_val(
    env: &mut Environment,
    rv: &RecursorVal,
) -> Result<(), KernelError> {
    let mut tc = TypeChecker::new(env);
    tc.ensure_sort(&rv.common.ty)?;
    Ok(())
}
/// Validate a `QuotVal`: verify the quotient component type is well-formed.
#[allow(clippy::result_large_err)]
pub(super) fn check_quotient_val(env: &mut Environment, qv: &QuotVal) -> Result<(), KernelError> {
    let mut tc = TypeChecker::new(env);
    tc.ensure_sort(&qv.common.ty)?;
    Ok(())
}
/// Check whether a constructor type (Pi-type) ultimately returns the named
/// inductive type (possibly applied to arguments).
pub(super) fn constructor_returns_inductive(ty: &crate::Expr, ind_name: &crate::Name) -> bool {
    use crate::Expr;
    match ty {
        Expr::Const(n, _) => n == ind_name,
        Expr::App(f, _) => constructor_returns_inductive(f, ind_name),
        Expr::Pi(_, _, _, cod) => constructor_returns_inductive(cod, ind_name),
        _ => false,
    }
}
/// Summarize a declaration without checking it.
pub fn summarize_declaration(decl: &Declaration) -> DeclSummary {
    match decl {
        Declaration::Axiom {
            name, univ_params, ..
        } => DeclSummary {
            name: format!("{}", name),
            kind: DeclKind::Axiom,
            has_body: false,
            num_univ_params: univ_params.len(),
        },
        Declaration::Definition {
            name, univ_params, ..
        } => DeclSummary {
            name: format!("{}", name),
            kind: DeclKind::Definition,
            has_body: true,
            num_univ_params: univ_params.len(),
        },
        Declaration::Theorem {
            name, univ_params, ..
        } => DeclSummary {
            name: format!("{}", name),
            kind: DeclKind::Theorem,
            has_body: true,
            num_univ_params: univ_params.len(),
        },
        Declaration::Opaque {
            name, univ_params, ..
        } => DeclSummary {
            name: format!("{}", name),
            kind: DeclKind::Opaque,
            has_body: true,
            num_univ_params: univ_params.len(),
        },
    }
}
/// Check multiple declarations, collecting all errors rather than stopping at first.
///
/// Unlike `check_declarations`, this function continues even after encountering
/// errors, and returns a summary of all results.
pub fn check_declarations_collect_errors(
    env: &mut Environment,
    decls: Vec<Declaration>,
) -> BatchCheckResult {
    let mut num_ok = 0;
    let mut num_failed = 0;
    let mut errors = Vec::new();
    for decl in decls {
        let name = format!("{}", decl.name());
        match check_declaration(env, decl) {
            Ok(()) => num_ok += 1,
            Err(e) => {
                num_failed += 1;
                errors.push((name, format!("{:?}", e)));
            }
        }
    }
    BatchCheckResult {
        num_ok,
        num_failed,
        errors,
    }
}
/// Check multiple declarations and collect statistics.
///
/// Returns `(stats, first_error)` where `first_error` is `Some(err)` if any
/// declaration failed, and `None` if all succeeded.
#[allow(clippy::result_large_err)]
pub fn check_declarations_with_stats(
    env: &mut Environment,
    decls: Vec<Declaration>,
) -> (CheckStats, Option<KernelError>) {
    let mut stats = CheckStats::new();
    for decl in decls {
        let kind = summarize_declaration(&decl).kind;
        match check_declaration(env, decl) {
            Ok(()) => match kind {
                DeclKind::Axiom => stats.num_axioms += 1,
                DeclKind::Definition => stats.num_definitions += 1,
                DeclKind::Theorem => stats.num_theorems += 1,
                DeclKind::Opaque => stats.num_opaques += 1,
            },
            Err(e) => {
                stats.num_failures += 1;
                return (stats, Some(e));
            }
        }
    }
    (stats, None)
}
/// Check that a declaration name is valid (non-empty, no invalid characters).
pub fn is_valid_decl_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.' || c == '\'')
        && !name.starts_with('.')
        && !name.ends_with('.')
}
/// Check that a declaration's universe parameter list has no duplicates.
pub fn has_unique_univ_params(params: &[crate::Name]) -> bool {
    let mut seen = std::collections::HashSet::new();
    params.iter().all(|p| seen.insert(format!("{}", p)))
}
/// Check that two universe parameter lists are compatible (same length, same names).
pub fn univ_params_compatible(p1: &[crate::Name], p2: &[crate::Name]) -> bool {
    p1.len() == p2.len()
        && p1
            .iter()
            .zip(p2.iter())
            .all(|(a, b)| format!("{}", a) == format!("{}", b))
}
/// Format a declaration summary for display.
pub fn format_decl_summary(summary: &DeclSummary) -> String {
    let body_str = if summary.has_body { " := ..." } else { "" };
    let params_str = if summary.num_univ_params > 0 {
        format!(".{{u{}}}", summary.num_univ_params)
    } else {
        String::new()
    };
    format!(
        "{} {}{}{} : ...",
        summary.kind, summary.name, params_str, body_str
    )
}
/// Format a batch check result for display.
pub fn format_batch_result(result: &BatchCheckResult) -> String {
    if result.all_ok() {
        format!("All {} declarations OK", result.total())
    } else {
        let mut s = format!(
            "{}/{} declarations OK. Failures:\n",
            result.num_ok,
            result.total()
        );
        for msg in result.error_messages() {
            s.push_str("  - ");
            s.push_str(&msg);
            s.push('\n');
        }
        s
    }
}
/// Check the integrity of an environment by re-type-checking all declarations.
///
/// This is an expensive operation intended for debugging and testing. It verifies
/// that every declaration in the environment is still consistent.
pub fn verify_environment_integrity(env: &Environment) -> Vec<String> {
    let mut issues = Vec::new();
    for name in env
        .constant_names()
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
    {
        if let Some(info) = env.get(&name) {
            let ty = info.ty();
            if !is_structurally_valid_expr(ty) {
                issues.push(format!("{}: type is not structurally valid", name));
            }
        }
    }
    issues
}
/// Perform a basic structural validity check on an expression.
///
/// This checks that bound variable indices are in range, but does not
/// perform full type checking.
pub fn is_structurally_valid_expr(expr: &crate::Expr) -> bool {
    check_expr_bvars(expr, 0)
}
pub(super) fn check_expr_bvars(expr: &crate::Expr, depth: u32) -> bool {
    use crate::Expr;
    match expr {
        Expr::BVar(i) => *i < depth,
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => true,
        Expr::App(f, a) => check_expr_bvars(f, depth) && check_expr_bvars(a, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            check_expr_bvars(ty, depth) && check_expr_bvars(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            check_expr_bvars(ty, depth)
                && check_expr_bvars(val, depth)
                && check_expr_bvars(body, depth + 1)
        }
        Expr::Proj(_, _, e) => check_expr_bvars(e, depth),
    }
}
/// Collect the names of all constants referenced in a declaration.
///
/// This includes references in both the type and the value (if present).
pub fn declaration_dependencies(decl: &Declaration) -> Vec<crate::Name> {
    use crate::collect_const_names;
    let mut deps = collect_const_names(decl.ty());
    if let Some(val) = decl.value() {
        deps.extend(collect_const_names(val));
    }
    deps.sort_by(|a, b| format!("{}", a).cmp(&format!("{}", b)));
    deps.dedup_by(|a, b| format!("{}", a) == format!("{}", b));
    deps
}
/// Check that all dependencies of a declaration exist in the environment.
///
/// Returns the names of any missing dependencies.
pub fn check_dependencies_exist(env: &Environment, decl: &Declaration) -> Vec<crate::Name> {
    let deps = declaration_dependencies(decl);
    deps.into_iter()
        .filter(|name| env.get(name).is_none())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Level, Literal, ReducibilityHint};
    fn mk_type0() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    fn mk_nat_axiom() -> Declaration {
        Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: mk_type0(),
        }
    }
    fn mk_nat_const() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_check_axiom() {
        let mut env = Environment::new();
        let nat_ty = Expr::Sort(Level::succ(Level::zero()));
        let axiom = Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: nat_ty,
        };
        check_declaration(&mut env, axiom).expect("value should be present");
        assert!(env.contains(&Name::str("Nat")));
    }
    #[test]
    fn test_check_definition() {
        let mut env = Environment::new();
        env.add(mk_nat_axiom()).expect("value should be present");
        let nat_ty = mk_nat_const();
        let val = Expr::Lit(Literal::Nat(42));
        let def = Declaration::Definition {
            name: Name::str("answer"),
            univ_params: vec![],
            ty: nat_ty,
            val,
            hint: ReducibilityHint::Regular(1),
        };
        check_declaration(&mut env, def).expect("value should be present");
        assert!(env.contains(&Name::str("answer")));
    }
    #[test]
    fn test_type_mismatch() {
        let mut env = Environment::new();
        let type0 = mk_type0();
        env.add(Declaration::Axiom {
            name: Name::str("String"),
            univ_params: vec![],
            ty: type0.clone(),
        })
        .expect("value should be present");
        env.add(Declaration::Axiom {
            name: Name::str("Nat"),
            univ_params: vec![],
            ty: type0,
        })
        .expect("value should be present");
        let string_ty = Expr::Const(Name::str("String"), vec![]);
        let val = Expr::Lit(Literal::Nat(42));
        let def = Declaration::Definition {
            name: Name::str("bad"),
            univ_params: vec![],
            ty: string_ty,
            val,
            hint: ReducibilityHint::Regular(1),
        };
        let result = check_declaration(&mut env, def);
        assert!(result.is_err());
    }
    #[test]
    fn test_check_declarations_sequence() {
        let mut env = Environment::new();
        let decls = vec![
            Declaration::Axiom {
                name: Name::str("A"),
                univ_params: vec![],
                ty: mk_type0(),
            },
            Declaration::Axiom {
                name: Name::str("B"),
                univ_params: vec![],
                ty: mk_type0(),
            },
        ];
        check_declarations(&mut env, decls).expect("value should be present");
        assert!(env.contains(&Name::str("A")));
        assert!(env.contains(&Name::str("B")));
    }
    #[test]
    fn test_decl_kind_display() {
        assert_eq!(DeclKind::Axiom.as_str(), "axiom");
        assert_eq!(DeclKind::Definition.as_str(), "def");
        assert_eq!(DeclKind::Theorem.as_str(), "theorem");
        assert_eq!(DeclKind::Opaque.as_str(), "opaque");
        assert!(DeclKind::Theorem.is_proven());
        assert!(DeclKind::Axiom.is_assumed());
        assert!(DeclKind::Definition.is_definition());
        assert!(!DeclKind::Axiom.is_definition());
    }
    #[test]
    fn test_summarize_declaration() {
        let decl = mk_nat_axiom();
        let summary = summarize_declaration(&decl);
        assert_eq!(summary.name, "Nat");
        assert_eq!(summary.kind, DeclKind::Axiom);
        assert!(!summary.has_body);
        assert_eq!(summary.num_univ_params, 0);
    }
    #[test]
    fn test_is_valid_decl_name() {
        assert!(is_valid_decl_name("Nat"));
        assert!(is_valid_decl_name("Nat.succ"));
        assert!(is_valid_decl_name("foo_bar"));
        assert!(!is_valid_decl_name(""));
        assert!(!is_valid_decl_name(".leading_dot"));
        assert!(!is_valid_decl_name("trailing_dot."));
    }
    #[test]
    fn test_has_unique_univ_params() {
        let u = Name::str("u");
        let v = Name::str("v");
        let params = vec![u.clone(), v.clone()];
        assert!(has_unique_univ_params(&params));
        let dup_params = vec![u.clone(), u.clone()];
        assert!(!has_unique_univ_params(&dup_params));
    }
    #[test]
    fn test_check_config_default() {
        let cfg = CheckConfig::default();
        assert!(cfg.check_type_is_sort);
        assert!(cfg.check_value_type);
        assert!(cfg.check_no_free_vars);
    }
    #[test]
    fn test_check_config_lenient() {
        let cfg = CheckConfig::lenient();
        assert!(cfg.check_type_is_sort);
        assert!(!cfg.check_value_type);
    }
    #[test]
    fn test_check_config_strict() {
        let cfg = CheckConfig::strict();
        assert!(!cfg.allow_axioms);
        assert!(cfg.max_depth > 0);
    }
    #[test]
    fn test_batch_check_all_ok() {
        let mut env = Environment::new();
        let decls = vec![
            Declaration::Axiom {
                name: Name::str("X"),
                univ_params: vec![],
                ty: mk_type0(),
            },
            Declaration::Axiom {
                name: Name::str("Y"),
                univ_params: vec![],
                ty: mk_type0(),
            },
        ];
        let result = check_declarations_collect_errors(&mut env, decls);
        assert!(result.all_ok());
        assert_eq!(result.num_ok, 2);
        assert_eq!(result.num_failed, 0);
    }
    #[test]
    fn test_batch_check_with_error() {
        let mut env = Environment::new();
        let decls = vec![
            Declaration::Axiom {
                name: Name::str("M"),
                univ_params: vec![],
                ty: mk_type0(),
            },
            Declaration::Definition {
                name: Name::str("bad_def"),
                univ_params: vec![],
                ty: Expr::Const(Name::str("UndefinedType"), vec![]),
                val: Expr::Lit(Literal::Nat(0)),
                hint: ReducibilityHint::Regular(1),
            },
        ];
        let result = check_declarations_collect_errors(&mut env, decls);
        assert!(!result.all_ok());
        assert_eq!(result.num_ok, 1);
        assert_eq!(result.num_failed, 1);
    }
    #[test]
    fn test_env_builder_clean() {
        let mut builder = EnvBuilder::new();
        let added = builder.add(Declaration::Axiom {
            name: Name::str("P"),
            univ_params: vec![],
            ty: mk_type0(),
        });
        assert!(added);
        assert!(builder.is_clean());
        assert_eq!(builder.stats().num_axioms, 1);
    }
    #[test]
    fn test_env_builder_with_error() {
        let mut builder = EnvBuilder::new();
        let added = builder.add(Declaration::Definition {
            name: Name::str("oops"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("NoSuchType"), vec![]),
            val: Expr::Lit(Literal::Nat(0)),
            hint: ReducibilityHint::Regular(1),
        });
        assert!(!added);
        assert!(!builder.is_clean());
        assert_eq!(builder.errors().len(), 1);
    }
    #[test]
    fn test_well_formed_result_display() {
        let ok = WellFormedResult::Ok;
        assert!(ok.is_ok());
        assert_eq!(ok.description(), "well-formed");
        let mismatch = WellFormedResult::TypeMismatch {
            description: "Nat vs String".to_string(),
        };
        assert!(!mismatch.is_ok());
        assert!(mismatch.description().contains("Nat vs String"));
    }
    #[test]
    fn test_is_structurally_valid_expr() {
        let bad = Expr::BVar(0);
        assert!(!is_structurally_valid_expr(&bad));
        let sort = Expr::Sort(Level::zero());
        assert!(is_structurally_valid_expr(&sort));
        let lam = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert!(is_structurally_valid_expr(&lam));
    }
    #[test]
    fn test_format_decl_summary() {
        let summary = DeclSummary {
            name: "Nat.add".to_string(),
            kind: DeclKind::Definition,
            has_body: true,
            num_univ_params: 0,
        };
        let s = format_decl_summary(&summary);
        assert!(s.contains("Nat.add"));
        assert!(s.contains("def"));
    }
    #[test]
    fn test_check_stats_totals() {
        let stats = CheckStats {
            num_axioms: 2,
            num_definitions: 3,
            num_theorems: 1,
            num_opaques: 0,
            num_failures: 1,
        };
        assert_eq!(stats.total(), 6);
        assert_eq!(stats.num_ok(), 6);
    }
    #[test]
    fn test_format_batch_result_all_ok() {
        let result = BatchCheckResult {
            num_ok: 3,
            num_failed: 0,
            errors: vec![],
        };
        let s = format_batch_result(&result);
        assert!(s.contains("3"));
    }
    #[test]
    fn test_temp_env_scope() {
        let mut env = Environment::new();
        {
            let mut scope = TempEnvScope::new(&mut env);
            let ok = scope.add(Declaration::Axiom {
                name: Name::str("TempDecl"),
                univ_params: vec![],
                ty: mk_type0(),
            });
            assert!(ok.is_ok());
            assert_eq!(scope.num_added(), 1);
        }
        assert!(env.contains(&Name::str("TempDecl")));
    }
    #[allow(dead_code)]
    fn mk_type1() -> Expr {
        Expr::Sort(Level::succ(Level::succ(Level::zero())))
    }
    #[test]
    fn test_check_constant_info_axiom() {
        use crate::declaration::{AxiomVal, ConstantVal};
        let mut env = Environment::new();
        let ci = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str("MyAxiom"),
                level_params: vec![],
                ty: mk_type0(),
            },
            is_unsafe: false,
        });
        check_constant_info(&mut env, ci).expect("value should be present");
        assert!(env.contains(&Name::str("MyAxiom")));
    }
    #[test]
    fn test_check_constant_info_inductive_simple() {
        use crate::declaration::{ConstantVal, InductiveVal};
        let mut env = Environment::new();
        let iv = InductiveVal {
            common: ConstantVal {
                name: Name::str("Bool"),
                level_params: vec![],
                ty: mk_type0(),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("Bool")],
            ctors: vec![Name::str("Bool.true"), Name::str("Bool.false")],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        };
        let ci = ConstantInfo::Inductive(iv);
        check_constant_info(&mut env, ci).expect("value should be present");
        assert!(env.contains(&Name::str("Bool")));
    }
    #[test]
    fn test_check_constant_info_inductive_empty() {
        use crate::declaration::{ConstantVal, InductiveVal};
        let mut env = Environment::new();
        let iv = InductiveVal {
            common: ConstantVal {
                name: Name::str("Empty"),
                level_params: vec![],
                ty: mk_type0(),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("Empty")],
            ctors: vec![],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        };
        let ci = ConstantInfo::Inductive(iv);
        check_constant_info(&mut env, ci).expect("value should be present");
        assert!(env.contains(&Name::str("Empty")));
    }
    #[test]
    fn test_check_constant_info_constructor_valid() {
        use crate::declaration::{ConstantVal, ConstructorVal, InductiveVal};
        let mut env = Environment::new();
        let ind_ci = ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: Name::str("Bool"),
                level_params: vec![],
                ty: mk_type0(),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("Bool")],
            ctors: vec![Name::str("Bool.true")],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        });
        check_constant_info(&mut env, ind_ci).expect("value should be present");
        let ctor_ci = ConstantInfo::Constructor(ConstructorVal {
            common: ConstantVal {
                name: Name::str("Bool.true"),
                level_params: vec![],
                ty: Expr::Const(Name::str("Bool"), vec![]),
            },
            induct: Name::str("Bool"),
            cidx: 0,
            num_params: 0,
            num_fields: 0,
            is_unsafe: false,
        });
        check_constant_info(&mut env, ctor_ci).expect("value should be present");
        assert!(env.contains(&Name::str("Bool.true")));
    }
    #[test]
    fn test_check_constant_info_constructor_wrong_return_type() {
        use crate::declaration::{ConstantVal, ConstructorVal, InductiveVal};
        let mut env = Environment::new();
        env.add_constant(ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: Name::str("Bool"),
                level_params: vec![],
                ty: mk_type0(),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("Bool")],
            ctors: vec![Name::str("Bool.bad")],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }))
        .expect("value should be present");
        env.add_constant(ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: Name::str("Nat"),
                level_params: vec![],
                ty: mk_type0(),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("Nat")],
            ctors: vec![],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }))
        .expect("value should be present");
        let ctor_ci = ConstantInfo::Constructor(ConstructorVal {
            common: ConstantVal {
                name: Name::str("Bool.bad"),
                level_params: vec![],
                ty: Expr::Const(Name::str("Nat"), vec![]),
            },
            induct: Name::str("Bool"),
            cidx: 0,
            num_params: 0,
            num_fields: 0,
            is_unsafe: false,
        });
        let result = check_constant_info(&mut env, ctor_ci);
        assert!(result.is_err(), "expected error for wrong return type");
    }
    #[test]
    fn test_check_constant_info_recursor() {
        use crate::declaration::{ConstantVal, InductiveVal, RecursorVal};
        let mut env = Environment::new();
        env.add_constant(ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: Name::str("Bool"),
                level_params: vec![],
                ty: mk_type0(),
            },
            num_params: 0,
            num_indices: 0,
            all: vec![Name::str("Bool")],
            ctors: vec![],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        }))
        .expect("value should be present");
        let rec_ty = mk_type0();
        let rec_ci = ConstantInfo::Recursor(RecursorVal {
            common: ConstantVal {
                name: Name::str("Bool.rec"),
                level_params: vec![],
                ty: rec_ty,
            },
            all: vec![Name::str("Bool")],
            num_params: 0,
            num_indices: 0,
            num_motives: 1,
            num_minors: 2,
            rules: vec![],
            k: false,
            is_unsafe: false,
        });
        check_constant_info(&mut env, rec_ci).expect("value should be present");
        assert!(env.contains(&Name::str("Bool.rec")));
    }
    #[test]
    fn test_check_constant_info_quotient() {
        use crate::declaration::{ConstantVal, QuotKind, QuotVal};
        let mut env = Environment::new();
        let quot_ci = ConstantInfo::Quotient(QuotVal {
            common: ConstantVal {
                name: Name::str("Quot"),
                level_params: vec![],
                ty: mk_type0(),
            },
            kind: QuotKind::Type,
        });
        check_constant_info(&mut env, quot_ci).expect("value should be present");
        assert!(env.contains(&Name::str("Quot")));
    }
    #[test]
    fn test_check_constant_infos_sequence() {
        use crate::declaration::{AxiomVal, ConstantVal};
        let mut env = Environment::new();
        let cis = vec![
            ConstantInfo::Axiom(AxiomVal {
                common: ConstantVal {
                    name: Name::str("P"),
                    level_params: vec![],
                    ty: mk_type0(),
                },
                is_unsafe: false,
            }),
            ConstantInfo::Axiom(AxiomVal {
                common: ConstantVal {
                    name: Name::str("Q"),
                    level_params: vec![],
                    ty: mk_type0(),
                },
                is_unsafe: false,
            }),
        ];
        check_constant_infos(&mut env, cis).expect("value should be present");
        assert!(env.contains(&Name::str("P")));
        assert!(env.contains(&Name::str("Q")));
    }
    #[test]
    fn test_check_constant_info_duplicate() {
        use crate::declaration::{AxiomVal, ConstantVal};
        let mut env = Environment::new();
        let make_ci = || {
            ConstantInfo::Axiom(AxiomVal {
                common: ConstantVal {
                    name: Name::str("Dup"),
                    level_params: vec![],
                    ty: mk_type0(),
                },
                is_unsafe: false,
            })
        };
        check_constant_info(&mut env, make_ci()).expect("value should be present");
        let result = check_constant_info(&mut env, make_ci());
        assert!(result.is_err(), "duplicate should fail");
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
