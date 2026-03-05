//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ConstructorFieldInfo, InductiveInfo, InductiveKind, ObtainResult, PatternParser, RcasesConfig,
    RcasesEngine, RcasesPattern, RcasesResult,
};
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};
use std::collections::{HashSet, VecDeque};

/// Parse an rcases pattern from a string.
///
/// Supports the following syntax:
/// - `x` — single variable
/// - `_` — discard
/// - `⟨a, b, c⟩` or `<a, b, c>` — tuple destructuring
/// - `a | b | c` — alternatives
/// - `(p : ty)` — typed pattern
/// - Nested combinations of the above
///
/// # Examples
///
/// ```ignore
/// let pat = parse_rcases_pattern("⟨a, b⟩").expect("pat should be present");
/// let pat = parse_rcases_pattern("a | b").expect("pat should be present");
/// let pat = parse_rcases_pattern("⟨a, ⟨b, c⟩⟩").expect("pat should be present");
/// ```
pub fn parse_rcases_pattern(input: &str) -> Result<RcasesPattern, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("empty pattern".to_string());
    }
    let mut parser = PatternParser::new(input);
    let pat = parser.parse_top()?;
    parser.skip_ws();
    if !parser.at_end() {
        return Err(format!(
            "unexpected trailing characters at position {}: '{}'",
            parser.pos,
            &input[parser.pos..]
        ));
    }
    Ok(pat)
}
/// Analyze the head type of an expression to extract inductive type information.
pub(super) fn analyze_inductive_type(target: &Expr, ctx: &MetaContext) -> Option<InductiveInfo> {
    let head = get_head_const(target)?;
    let head_str = format!("{}", head);
    if let Some(oxilean_kernel::ConstantInfo::Inductive(ind)) = ctx.env().find(&head) {
        let mut constructors = Vec::new();
        for ctor_name in &ind.ctors {
            let num_fields = if let Some(oxilean_kernel::ConstantInfo::Constructor(cv)) =
                ctx.env().find(ctor_name)
            {
                cv.num_fields
            } else {
                0
            };
            let field_names: Vec<Name> = (0..num_fields)
                .map(|i| Name::str(format!("field_{}", i)))
                .collect();
            let is_recursive = check_recursive_ctor(ctor_name, &head, ctx);
            constructors.push(ConstructorFieldInfo {
                ctor_name: ctor_name.clone(),
                num_fields,
                field_names,
                is_recursive,
            });
        }
        return Some(InductiveInfo {
            name: head.clone(),
            is_structure: constructors.len() == 1,
            num_params: ind.num_params,
            constructors,
        });
    }
    get_well_known_inductive_info(&head_str)
}
/// Check if a constructor is recursive (contains the inductive type).
pub(super) fn check_recursive_ctor(
    ctor_name: &Name,
    induct_name: &Name,
    ctx: &MetaContext,
) -> bool {
    let ctor_str = format!("{}", ctor_name);
    let ind_str = format!("{}", induct_name);
    match ind_str.as_str() {
        "Nat" => ctor_str.contains("succ"),
        "List" => ctor_str.contains("cons"),
        _ => {
            if let Some(oxilean_kernel::ConstantInfo::Constructor(cv)) = ctx.env().find(ctor_name) {
                cv.num_fields > 0 && format!("{}", cv.induct).contains(&ind_str)
            } else {
                false
            }
        }
    }
}
/// Get well-known inductive type info for common types.
pub(super) fn get_well_known_inductive_info(name: &str) -> Option<InductiveInfo> {
    match name {
        "And" => Some(InductiveInfo {
            name: Name::str("And"),
            constructors: vec![ConstructorFieldInfo {
                ctor_name: Name::str("And.intro"),
                num_fields: 2,
                field_names: vec![Name::str("left"), Name::str("right")],
                is_recursive: false,
            }],
            is_structure: true,
            num_params: 2,
        }),
        "Or" => Some(InductiveInfo {
            name: Name::str("Or"),
            constructors: vec![
                ConstructorFieldInfo {
                    ctor_name: Name::str("Or.inl"),
                    num_fields: 1,
                    field_names: vec![Name::str("h")],
                    is_recursive: false,
                },
                ConstructorFieldInfo {
                    ctor_name: Name::str("Or.inr"),
                    num_fields: 1,
                    field_names: vec![Name::str("h")],
                    is_recursive: false,
                },
            ],
            is_structure: false,
            num_params: 2,
        }),
        "Exists" | "Sigma" => Some(InductiveInfo {
            name: Name::str(name),
            constructors: vec![ConstructorFieldInfo {
                ctor_name: Name::str("Sigma.mk"),
                num_fields: 2,
                field_names: vec![Name::str("fst"), Name::str("snd")],
                is_recursive: false,
            }],
            is_structure: true,
            num_params: 2,
        }),
        "Prod" => Some(InductiveInfo {
            name: Name::str("Prod"),
            constructors: vec![ConstructorFieldInfo {
                ctor_name: Name::str("Prod.mk"),
                num_fields: 2,
                field_names: vec![Name::str("fst"), Name::str("snd")],
                is_recursive: false,
            }],
            is_structure: true,
            num_params: 2,
        }),
        "Nat" => Some(InductiveInfo {
            name: Name::str("Nat"),
            constructors: vec![
                ConstructorFieldInfo {
                    ctor_name: Name::str("Nat.zero"),
                    num_fields: 0,
                    field_names: vec![],
                    is_recursive: false,
                },
                ConstructorFieldInfo {
                    ctor_name: Name::str("Nat.succ"),
                    num_fields: 1,
                    field_names: vec![Name::str("n")],
                    is_recursive: true,
                },
            ],
            is_structure: false,
            num_params: 0,
        }),
        "Bool" => Some(InductiveInfo {
            name: Name::str("Bool"),
            constructors: vec![
                ConstructorFieldInfo {
                    ctor_name: Name::str("Bool.true"),
                    num_fields: 0,
                    field_names: vec![],
                    is_recursive: false,
                },
                ConstructorFieldInfo {
                    ctor_name: Name::str("Bool.false"),
                    num_fields: 0,
                    field_names: vec![],
                    is_recursive: false,
                },
            ],
            is_structure: false,
            num_params: 0,
        }),
        "List" => Some(InductiveInfo {
            name: Name::str("List"),
            constructors: vec![
                ConstructorFieldInfo {
                    ctor_name: Name::str("List.nil"),
                    num_fields: 0,
                    field_names: vec![],
                    is_recursive: false,
                },
                ConstructorFieldInfo {
                    ctor_name: Name::str("List.cons"),
                    num_fields: 2,
                    field_names: vec![Name::str("head"), Name::str("tail")],
                    is_recursive: true,
                },
            ],
            is_structure: false,
            num_params: 1,
        }),
        "Option" => Some(InductiveInfo {
            name: Name::str("Option"),
            constructors: vec![
                ConstructorFieldInfo {
                    ctor_name: Name::str("Option.none"),
                    num_fields: 0,
                    field_names: vec![],
                    is_recursive: false,
                },
                ConstructorFieldInfo {
                    ctor_name: Name::str("Option.some"),
                    num_fields: 1,
                    field_names: vec![Name::str("val")],
                    is_recursive: false,
                },
            ],
            is_structure: false,
            num_params: 1,
        }),
        "True" => Some(InductiveInfo {
            name: Name::str("True"),
            constructors: vec![ConstructorFieldInfo {
                ctor_name: Name::str("True.intro"),
                num_fields: 0,
                field_names: vec![],
                is_recursive: false,
            }],
            is_structure: true,
            num_params: 0,
        }),
        "False" => Some(InductiveInfo {
            name: Name::str("False"),
            constructors: vec![],
            is_structure: false,
            num_params: 0,
        }),
        "Unit" => Some(InductiveInfo {
            name: Name::str("Unit"),
            constructors: vec![ConstructorFieldInfo {
                ctor_name: Name::str("Unit.unit"),
                num_fields: 0,
                field_names: vec![],
                is_recursive: false,
            }],
            is_structure: true,
            num_params: 0,
        }),
        "PSigma" => Some(InductiveInfo {
            name: Name::str("PSigma"),
            constructors: vec![ConstructorFieldInfo {
                ctor_name: Name::str("PSigma.mk"),
                num_fields: 2,
                field_names: vec![Name::str("fst"), Name::str("snd")],
                is_recursive: false,
            }],
            is_structure: true,
            num_params: 2,
        }),
        "PProd" => Some(InductiveInfo {
            name: Name::str("PProd"),
            constructors: vec![ConstructorFieldInfo {
                ctor_name: Name::str("PProd.mk"),
                num_fields: 2,
                field_names: vec![Name::str("fst"), Name::str("snd")],
                is_recursive: false,
            }],
            is_structure: true,
            num_params: 2,
        }),
        "Subtype" => Some(InductiveInfo {
            name: Name::str("Subtype"),
            constructors: vec![ConstructorFieldInfo {
                ctor_name: Name::str("Subtype.mk"),
                num_fields: 2,
                field_names: vec![Name::str("val"), Name::str("property")],
                is_recursive: false,
            }],
            is_structure: true,
            num_params: 2,
        }),
        "Iff" => Some(InductiveInfo {
            name: Name::str("Iff"),
            constructors: vec![ConstructorFieldInfo {
                ctor_name: Name::str("Iff.intro"),
                num_fields: 2,
                field_names: vec![Name::str("mp"), Name::str("mpr")],
                is_recursive: false,
            }],
            is_structure: true,
            num_params: 2,
        }),
        _ => None,
    }
}
/// Validate that a pattern is compatible with the given inductive info.
/// Returns an error message if the pattern is incompatible.
pub(super) fn validate_pattern_compat(
    pattern: &RcasesPattern,
    info: &InductiveInfo,
) -> Result<(), String> {
    match pattern {
        RcasesPattern::Alts(alts) => {
            if !info.is_sum_like() && alts.len() > 1 {
                return Err(format!(
                    "pattern has {} alternatives, but {} has only {} constructor(s)",
                    alts.len(),
                    info.name,
                    info.num_constructors()
                ));
            }
            if info.is_sum_like() && alts.len() > info.num_constructors() {
                return Err(format!(
                    "pattern has {} alternatives, but {} has only {} constructors",
                    alts.len(),
                    info.name,
                    info.num_constructors()
                ));
            }
            Ok(())
        }
        RcasesPattern::Tuple(pats) => {
            if info.is_sum_like() {
                let any_match = info
                    .constructors
                    .iter()
                    .any(|c| c.num_fields as usize >= pats.len());
                if !any_match && !pats.is_empty() {
                    return Err(format!(
                        "tuple pattern with {} elements doesn't match any constructor of {}",
                        pats.len(),
                        info.name
                    ));
                }
            }
            Ok(())
        }
        RcasesPattern::One(_) | RcasesPattern::Clear => Ok(()),
        RcasesPattern::Typed(p, _) => validate_pattern_compat(p, info),
        RcasesPattern::Nested(p) => validate_pattern_compat(p, info),
    }
}
/// Align a pattern with constructors, producing one sub-pattern per constructor.
///
/// For a sum type Or with constructors [inl, inr]:
/// - Pattern `a | b` gives [a, b]
/// - Pattern `a` gives [a, a] (same pattern for all)
///
/// For a product type And with constructor [intro]:
/// - Pattern `⟨a, b⟩` gives [⟨a, b⟩]
/// - Pattern `x` gives [x]
pub(super) fn align_pattern_with_constructors(
    pattern: &RcasesPattern,
    info: &InductiveInfo,
) -> Vec<RcasesPattern> {
    let n = info.num_constructors();
    if n == 0 {
        return Vec::new();
    }
    match pattern {
        RcasesPattern::Alts(alts) => {
            let mut result = Vec::with_capacity(n);
            for i in 0..n {
                if i < alts.len() {
                    result.push(alts[i].clone());
                } else {
                    result.push(RcasesPattern::Clear);
                }
            }
            result
        }
        _ => vec![pattern.clone(); n],
    }
}
/// Generate variable names for a constructor's fields based on the pattern.
pub(super) fn generate_field_names(
    pattern: &RcasesPattern,
    ctor_info: &ConstructorFieldInfo,
    config: &RcasesConfig,
) -> Vec<Name> {
    let num = ctor_info.num_fields as usize;
    if num == 0 {
        return Vec::new();
    }
    match pattern {
        RcasesPattern::Tuple(pats) => {
            let mut names = Vec::with_capacity(num);
            for i in 0..num {
                let name = if i < pats.len() {
                    field_name_from_pattern(&pats[i], i, ctor_info, config)
                } else if config.use_constructor_names && i < ctor_info.field_names.len() {
                    ctor_info.field_names[i].clone()
                } else {
                    Name::str(format!("x_{}", i))
                };
                names.push(name);
            }
            names
        }
        RcasesPattern::One(name) => {
            if num == 1 {
                vec![Name::str(name.clone())]
            } else {
                (0..num)
                    .map(|i| Name::str(format!("{}_{}", name, i)))
                    .collect()
            }
        }
        RcasesPattern::Clear => (0..num).map(|i| Name::str(format!("_x_{}", i))).collect(),
        _ => {
            if config.use_constructor_names && !ctor_info.field_names.is_empty() {
                let mut names = Vec::with_capacity(num);
                for i in 0..num {
                    if i < ctor_info.field_names.len() {
                        names.push(ctor_info.field_names[i].clone());
                    } else {
                        names.push(Name::str(format!("x_{}", i)));
                    }
                }
                names
            } else {
                (0..num).map(|i| Name::str(format!("x_{}", i))).collect()
            }
        }
    }
}
/// Extract a field name from a single pattern element.
pub(super) fn field_name_from_pattern(
    pat: &RcasesPattern,
    idx: usize,
    ctor_info: &ConstructorFieldInfo,
    config: &RcasesConfig,
) -> Name {
    match pat {
        RcasesPattern::One(name) => Name::str(name.clone()),
        RcasesPattern::Clear => Name::str(format!("_x_{}", idx)),
        RcasesPattern::Typed(inner, _) => field_name_from_pattern(inner, idx, ctor_info, config),
        _ => {
            if config.use_constructor_names && idx < ctor_info.field_names.len() {
                ctor_info.field_names[idx].clone()
            } else {
                Name::str(format!("x_{}", idx))
            }
        }
    }
}
/// `rcases h with pattern` — recursive case analysis with destructuring.
///
/// Performs recursive case analysis on the target expression, applying
/// the given pattern to destructure the result.
///
/// # Arguments
///
/// * `pattern` - The destructuring pattern to apply
/// * `target` - The expression to destructure
/// * `state` - The current tactic state
/// * `ctx` - The meta context
///
/// # Examples
///
/// ```ignore
/// // Given h : A /\ B
/// // rcases h with ⟨a, b⟩
/// let pat = RcasesPattern::tuple(vec![
///     RcasesPattern::one("a"),
///     RcasesPattern::one("b"),
/// ]);
/// let result = tac_rcases(&pat, &h, &mut state, &mut ctx)?;
/// ```
pub fn tac_rcases(
    pattern: &RcasesPattern,
    target: &Expr,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<RcasesResult> {
    let config = RcasesConfig::default();
    tac_rcases_with_config(pattern, target, state, ctx, &config)
}
/// `rcases` with explicit configuration.
pub fn tac_rcases_with_config(
    pattern: &RcasesPattern,
    target: &Expr,
    state: &mut TacticState,
    ctx: &mut MetaContext,
    config: &RcasesConfig,
) -> TacticResult<RcasesResult> {
    let goal_id = state.current_goal()?;
    let target = ctx.instantiate_mvars(target);
    if let Some(info) = analyze_inductive_type(&target, ctx) {
        if let Err(msg) = validate_pattern_compat(pattern, &info) {
            return Err(TacticError::Failed(format!("rcases: {}", msg)));
        }
    }
    let mut engine = RcasesEngine::new(config.clone());
    let hyps = ctx.get_local_hyps();
    for (name, _) in &hyps {
        engine.register_name(&format!("{}", name));
    }
    let result = engine.process_one(pattern, &target, goal_id, state, ctx)?;
    if !result.goals.is_empty() {
        state.replace_goal(result.goals.clone());
    }
    Ok(result)
}
/// `rcases` on multiple targets simultaneously.
///
/// Applies multiple patterns to multiple targets in sequence.
/// Each pattern is applied to the corresponding target.
pub fn tac_rcases_many(
    patterns: &[RcasesPattern],
    targets: &[Expr],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<RcasesResult> {
    if patterns.len() != targets.len() {
        return Err(TacticError::Failed(format!(
            "rcases: expected {} patterns for {} targets",
            targets.len(),
            patterns.len()
        )));
    }
    let mut combined = RcasesResult::empty();
    for (pattern, target) in patterns.iter().zip(targets.iter()) {
        let result = tac_rcases(pattern, target, state, ctx)?;
        combined.merge(result);
    }
    Ok(combined)
}
/// `obtain ⟨a, b⟩ : ty := proof` — introduce and destructure.
///
/// The obtain tactic creates a new hypothesis of the given type,
/// with a proof obligation, and then destructures it using the pattern.
///
/// # Arguments
///
/// * `pattern` - The destructuring pattern
/// * `ty` - The type of the value to obtain
/// * `state` - The current tactic state
/// * `ctx` - The meta context
pub fn tac_obtain(
    pattern: &RcasesPattern,
    ty: &Expr,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<ObtainResult> {
    let goal_id = state.current_goal()?;
    let goal_ty = ctx
        .get_mvar_type(goal_id)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let goal_ty = ctx.instantiate_mvars(&goal_ty);
    let (proof_goal, proof_expr) = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
    let (body_goal, body_expr) = ctx.mk_fresh_expr_mvar(goal_ty.clone(), MetavarKind::Natural);
    let obtain_name = match pattern {
        RcasesPattern::One(name) => Name::str(name.clone()),
        _ => Name::str("this"),
    };
    let proof = Expr::App(
        Box::new(Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            obtain_name,
            Box::new(ty.clone()),
            Box::new(body_expr.clone()),
        )),
        Box::new(proof_expr),
    );
    ctx.assign_mvar(goal_id, proof);
    state.replace_goal(vec![body_goal]);
    let rcases_result = if pattern.is_simple() {
        let mut bindings = Vec::new();
        if let RcasesPattern::One(name) = pattern {
            bindings.push((Name::str(name.clone()), ty.clone()));
        }
        RcasesResult {
            goals: vec![body_goal],
            bindings,
            patterns_used: vec![pattern.clone()],
        }
    } else {
        tac_rcases(pattern, ty, state, ctx)?
    };
    let mut remaining = rcases_result.goals.clone();
    if !ctx.is_mvar_assigned(proof_goal) {
        remaining.push(proof_goal);
    }
    state.replace_goal(remaining.clone());
    Ok(ObtainResult {
        proof_goal,
        remaining_goals: remaining,
        bindings: rcases_result.bindings,
        patterns_used: rcases_result.patterns_used,
    })
}
/// `rintro ⟨a, b⟩` — intro followed by rcases.
///
/// Like `intro` but immediately destructures the introduced hypothesis
/// using the given pattern. This is a common shorthand:
///
/// ```text
/// rintro ⟨a, b⟩  ===  intro h; rcases h with ⟨a, b⟩
/// ```
///
/// Multiple patterns are applied to successive Pi binders.
pub fn tac_rintro(
    patterns: &[RcasesPattern],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<RcasesResult> {
    let mut combined = RcasesResult::empty();
    for pattern in patterns {
        let goal = state.current_goal()?;
        let target = ctx
            .get_mvar_type(goal)
            .cloned()
            .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
        let target = ctx.instantiate_mvars(&target);
        match &target {
            Expr::Pi(bi, binder_name, domain, body) => {
                if pattern.is_simple() {
                    let intro_name = match pattern {
                        RcasesPattern::One(name) => Name::str(name.clone()),
                        RcasesPattern::Clear => Name::str("_rintro"),
                        _ => binder_name.clone(),
                    };
                    let fvar_id = ctx.mk_local_decl(
                        intro_name.clone(),
                        *domain.clone(),
                        oxilean_kernel::BinderInfo::Default,
                    );
                    let new_target = substitute_bvar(body, 0, &Expr::FVar(fvar_id));
                    let (new_goal_id, new_goal_expr) =
                        ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
                    let proof = Expr::Lam(
                        *bi,
                        intro_name.clone(),
                        domain.clone(),
                        Box::new(new_goal_expr),
                    );
                    ctx.assign_mvar(goal, proof);
                    state.replace_goal(vec![new_goal_id]);
                    if let RcasesPattern::One(name) = pattern {
                        combined
                            .bindings
                            .push((Name::str(name.clone()), *domain.clone()));
                    }
                    combined.goals.push(new_goal_id);
                    combined.patterns_used.push(pattern.clone());
                } else {
                    let temp_name = Name::str("_rintro_tmp");
                    let fvar_id = ctx.mk_local_decl(
                        temp_name.clone(),
                        *domain.clone(),
                        oxilean_kernel::BinderInfo::Default,
                    );
                    let new_target = substitute_bvar(body, 0, &Expr::FVar(fvar_id));
                    let (new_goal_id, new_goal_expr) =
                        ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
                    let proof = Expr::Lam(*bi, temp_name, domain.clone(), Box::new(new_goal_expr));
                    ctx.assign_mvar(goal, proof);
                    state.replace_goal(vec![new_goal_id]);
                    let target_expr = *domain.clone();
                    let rcases_result = tac_rcases(pattern, &target_expr, state, ctx)?;
                    combined.merge(rcases_result);
                }
            }
            _ => {
                return Err(TacticError::GoalMismatch(
                    "rintro requires a Pi/forall goal".into(),
                ));
            }
        }
    }
    Ok(combined)
}
/// Get the head constant of an expression (traversing applications).
pub(super) fn get_head_const(expr: &Expr) -> Option<Name> {
    let mut e = expr;
    while let Expr::App(f, _) = e {
        e = f;
    }
    match e {
        Expr::Const(name, _) => Some(name.clone()),
        _ => None,
    }
}
/// Get the arguments of an application expression.
pub(super) fn get_app_args(expr: &Expr) -> Vec<Expr> {
    let mut args = Vec::new();
    collect_app_args(expr, &mut args);
    args
}
/// Collect application arguments in left-to-right order.
pub(super) fn collect_app_args(expr: &Expr, args: &mut Vec<Expr>) {
    if let Expr::App(f, a) = expr {
        collect_app_args(f, args);
        args.push(*a.clone());
    }
}
/// Substitute BVar(idx) with a replacement expression.
pub(super) fn substitute_bvar(expr: &Expr, idx: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(n) => {
            if *n == idx {
                replacement.clone()
            } else if *n > idx {
                Expr::BVar(n - 1)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => {
            let f2 = substitute_bvar(f, idx, replacement);
            let a2 = substitute_bvar(a, idx, replacement);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty2 = substitute_bvar(ty, idx, replacement);
            let body2 = substitute_bvar(body, idx + 1, replacement);
            Expr::Lam(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty2 = substitute_bvar(ty, idx, replacement);
            let body2 = substitute_bvar(body, idx + 1, replacement);
            Expr::Pi(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = substitute_bvar(ty, idx, replacement);
            let val2 = substitute_bvar(val, idx, replacement);
            let body2 = substitute_bvar(body, idx + 1, replacement);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, i, e) => {
            let e2 = substitute_bvar(e, idx, replacement);
            Expr::Proj(name.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Build a projection expression for extracting a field from a structure.
///
/// For a structure `S`, `S.field` becomes `Expr::Proj("S", field_idx, e)`.
pub(super) fn mk_projection(struct_name: &Name, field_idx: u32, expr: Expr) -> Expr {
    Expr::Proj(struct_name.clone(), field_idx, Box::new(expr))
}
/// Build a `casesOn` application for an inductive type.
pub(super) fn mk_cases_on(inductive_name: &Name, major: Expr, branches: Vec<Expr>) -> Expr {
    let cases_name = Name::str(format!("{}.casesOn", inductive_name));
    let mut result = Expr::Const(cases_name, vec![Level::zero()]);
    result = Expr::App(Box::new(result), Box::new(major));
    for branch in branches {
        result = Expr::App(Box::new(result), Box::new(branch));
    }
    result
}
/// Check if an expression references a specific name.
pub(super) fn expr_mentions_name(expr: &Expr, name: &Name) -> bool {
    match expr {
        Expr::Const(n, _) => n == name,
        Expr::App(f, a) => expr_mentions_name(f, name) || expr_mentions_name(a, name),
        Expr::Lam(_, n, ty, body) => {
            n == name || expr_mentions_name(ty, name) || expr_mentions_name(body, name)
        }
        Expr::Pi(_, n, ty, body) => {
            n == name || expr_mentions_name(ty, name) || expr_mentions_name(body, name)
        }
        Expr::Let(n, ty, val, body) => {
            n == name
                || expr_mentions_name(ty, name)
                || expr_mentions_name(val, name)
                || expr_mentions_name(body, name)
        }
        Expr::Proj(_, _, e) => expr_mentions_name(e, name),
        _ => false,
    }
}
pub(super) fn classify_inductive(info: &InductiveInfo) -> InductiveKind {
    if info.constructors.is_empty() {
        InductiveKind::Empty
    } else if info.constructors.len() == 1 {
        if info.constructors[0].num_fields == 0 {
            InductiveKind::Unit
        } else {
            InductiveKind::Product
        }
    } else {
        InductiveKind::Sum
    }
}
/// Try to extract the type name from a target expression.
pub(super) fn extract_type_name(target: &Expr) -> Option<String> {
    get_head_const(target).map(|n| format!("{}", n))
}
/// Split a pattern at the top level into alternatives.
///
/// If the pattern is already `Alts(...)`, returns the alternatives.
/// Otherwise wraps it in a single-element list.
pub(super) fn split_alts(pattern: &RcasesPattern) -> Vec<&RcasesPattern> {
    match pattern {
        RcasesPattern::Alts(alts) => alts.iter().collect(),
        _ => vec![pattern],
    }
}
/// Merge the bindings from multiple branches, resolving name conflicts.
pub(super) fn merge_branch_bindings(branches: &[Vec<(Name, Expr)>]) -> Vec<(Name, Expr)> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    for branch in branches {
        for (name, expr) in branch {
            let name_str = format!("{}", name);
            if !seen.contains(&name_str) {
                seen.insert(name_str);
                result.push((name.clone(), expr.clone()));
            }
        }
    }
    result
}
/// Count the total number of goals that will be produced by a pattern on an
/// inductive type.
pub(super) fn count_expected_goals(pattern: &RcasesPattern, info: &InductiveInfo) -> usize {
    let kind = classify_inductive(info);
    match kind {
        InductiveKind::Empty => 0,
        InductiveKind::Unit => 1,
        InductiveKind::Product => match pattern {
            RcasesPattern::Tuple(pats) => {
                let mut count = 1;
                for p in pats {
                    if !p.is_simple() {
                        count += p.count_bindings().max(1) - 1;
                    }
                }
                count
            }
            _ => 1,
        },
        InductiveKind::Sum => info.num_constructors(),
    }
}
/// Build a lambda abstraction that introduces fields for a constructor.
pub(super) fn mk_field_lambda(field_names: &[Name], field_types: &[Expr], body: Expr) -> Expr {
    let mut result = body;
    for (name, ty) in field_names.iter().zip(field_types.iter()).rev() {
        result = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            name.clone(),
            Box::new(ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Check if a pattern is compatible with the given number of constructors.
pub(super) fn check_arity_compat(pattern: &RcasesPattern, num_ctors: usize) -> Result<(), String> {
    if let RcasesPattern::Alts(alts) = pattern {
        if alts.len() > num_ctors {
            return Err(format!(
                "pattern has {} alternatives but type has only {} constructors",
                alts.len(),
                num_ctors
            ));
        }
    }
    Ok(())
}
/// Try to decompose a type into its head and arguments.
///
/// For example, `And P Q` becomes `("And", [P, Q])`.
pub(super) fn decompose_app(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let head = decompose_app_impl(expr, &mut args);
    (head, args)
}
pub(super) fn decompose_app_impl(expr: &Expr, args: &mut Vec<Expr>) -> Expr {
    match expr {
        Expr::App(f, a) => {
            let head = decompose_app_impl(f, args);
            args.push(*a.clone());
            head
        }
        _ => expr.clone(),
    }
}
/// Build an application from a head and arguments.
pub(super) fn mk_app(head: Expr, args: &[Expr]) -> Expr {
    let mut result = head;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg.clone()));
    }
    result
}
/// Extract the field types for a constructor by peeling its Pi-chain type.
///
/// Skips `num_params` leading Pi binders (the inductive type parameters) and
/// then collects the domain types of the next `num_fields` Pi binders.
/// Falls back to `Sort(Level::zero())` for any binder that cannot be determined.
pub(super) fn get_ctor_field_types(ctor_name: &Name, ctx: &MetaContext) -> Vec<Expr> {
    if let Some(oxilean_kernel::ConstantInfo::Constructor(cv)) = ctx.env().find(ctor_name) {
        let num_params = cv.num_params as usize;
        let num_fields = cv.num_fields as usize;
        let mut e = &cv.common.ty;
        for _ in 0..num_params {
            match e {
                Expr::Pi(_, _, _, body) => e = body,
                _ => return vec![Expr::Sort(Level::zero()); num_fields],
            }
        }
        let mut field_types = Vec::with_capacity(num_fields);
        for _ in 0..num_fields {
            match e {
                Expr::Pi(_, _, dom, body) => {
                    field_types.push(dom.as_ref().clone());
                    e = body;
                }
                _ => break,
            }
        }
        while field_types.len() < num_fields {
            field_types.push(Expr::Sort(Level::zero()));
        }
        field_types
    } else {
        Vec::new()
    }
}
/// A helper for building pattern match branches.
///
/// Each branch wraps the body in lambdas for the constructor fields,
/// producing a function `fun (f1 : T1) (f2 : T2) => body`.
pub(super) fn mk_branch(
    ctor: &ConstructorFieldInfo,
    body: Expr,
    config: &RcasesConfig,
    field_types: &[Expr],
) -> Expr {
    if ctor.num_fields == 0 {
        return body;
    }
    let mut result = body;
    for i in (0..ctor.num_fields as usize).rev() {
        let name = if config.use_constructor_names && i < ctor.field_names.len() {
            ctor.field_names[i].clone()
        } else {
            Name::str(format!("x_{}", i))
        };
        let ty = field_types
            .get(i)
            .cloned()
            .unwrap_or_else(|| Expr::Sort(Level::zero()));
        result = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            name,
            Box::new(ty),
            Box::new(result),
        );
    }
    result
}
/// Resolve a pattern string into an `RcasesPattern`, with error context.
pub(super) fn resolve_pattern(input: &str, context: &str) -> TacticResult<RcasesPattern> {
    parse_rcases_pattern(input).map_err(|e| {
        TacticError::Failed(format!(
            "{}: failed to parse pattern '{}': {}",
            context, input, e
        ))
    })
}
/// Generate default patterns for an inductive type.
///
/// Produces a pattern that names every field with a fresh name.
pub(super) fn default_pattern_for_inductive(info: &InductiveInfo) -> RcasesPattern {
    let kind = classify_inductive(info);
    match kind {
        InductiveKind::Empty => RcasesPattern::Clear,
        InductiveKind::Unit => RcasesPattern::Clear,
        InductiveKind::Product => {
            let ctor = &info.constructors[0];
            let pats: Vec<RcasesPattern> = ctor
                .field_names
                .iter()
                .map(|n| RcasesPattern::One(format!("{}", n)))
                .collect();
            if pats.is_empty() {
                RcasesPattern::Clear
            } else {
                RcasesPattern::Tuple(pats)
            }
        }
        InductiveKind::Sum => {
            let alts: Vec<RcasesPattern> = info
                .constructors
                .iter()
                .map(|ctor| {
                    if ctor.num_fields == 0 {
                        RcasesPattern::Clear
                    } else {
                        let pats: Vec<RcasesPattern> = ctor
                            .field_names
                            .iter()
                            .map(|n| RcasesPattern::One(format!("{}", n)))
                            .collect();
                        if pats.len() == 1 {
                            pats.into_iter().next().expect("pats has exactly 1 element")
                        } else {
                            RcasesPattern::Tuple(pats)
                        }
                    }
                })
                .collect();
            RcasesPattern::Alts(alts)
        }
    }
}
/// Check if two patterns are structurally equal (ignoring type annotations).
pub(super) fn patterns_equal_ignore_types(p1: &RcasesPattern, p2: &RcasesPattern) -> bool {
    match (p1, p2) {
        (RcasesPattern::One(a), RcasesPattern::One(b)) => a == b,
        (RcasesPattern::Clear, RcasesPattern::Clear) => true,
        (RcasesPattern::Tuple(a), RcasesPattern::Tuple(b)) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b.iter())
                    .all(|(x, y)| patterns_equal_ignore_types(x, y))
        }
        (RcasesPattern::Alts(a), RcasesPattern::Alts(b)) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b.iter())
                    .all(|(x, y)| patterns_equal_ignore_types(x, y))
        }
        (RcasesPattern::Typed(a, _), RcasesPattern::Typed(b, _)) => {
            patterns_equal_ignore_types(a, b)
        }
        (RcasesPattern::Nested(a), RcasesPattern::Nested(b)) => patterns_equal_ignore_types(a, b),
        (RcasesPattern::Typed(a, _), b) => patterns_equal_ignore_types(a, b),
        (a, RcasesPattern::Typed(b, _)) => patterns_equal_ignore_types(a, b),
        _ => false,
    }
}
/// Compute the "depth" of a pattern (maximum nesting level).
pub(super) fn pattern_depth(pat: &RcasesPattern) -> usize {
    match pat {
        RcasesPattern::One(_) | RcasesPattern::Clear => 0,
        RcasesPattern::Tuple(pats) => 1 + pats.iter().map(pattern_depth).max().unwrap_or(0),
        RcasesPattern::Alts(pats) => 1 + pats.iter().map(pattern_depth).max().unwrap_or(0),
        RcasesPattern::Typed(p, _) => pattern_depth(p),
        RcasesPattern::Nested(p) => 1 + pattern_depth(p),
    }
}
/// Rename all variable bindings in a pattern using a renaming function.
fn rename_pattern<F>(pat: &RcasesPattern, f: &F) -> RcasesPattern
where
    F: Fn(&str) -> String,
{
    match pat {
        RcasesPattern::One(name) => RcasesPattern::One(f(name)),
        RcasesPattern::Clear => RcasesPattern::Clear,
        RcasesPattern::Tuple(pats) => {
            RcasesPattern::Tuple(pats.iter().map(|p| rename_pattern(p, f)).collect())
        }
        RcasesPattern::Alts(pats) => {
            RcasesPattern::Alts(pats.iter().map(|p| rename_pattern(p, f)).collect())
        }
        RcasesPattern::Typed(p, ty) => {
            RcasesPattern::Typed(Box::new(rename_pattern(p, f)), ty.clone())
        }
        RcasesPattern::Nested(p) => RcasesPattern::Nested(Box::new(rename_pattern(p, f))),
    }
}
/// Substitute a pattern variable with another pattern.
pub(super) fn substitute_pattern(
    pat: &RcasesPattern,
    var_name: &str,
    replacement: &RcasesPattern,
) -> RcasesPattern {
    match pat {
        RcasesPattern::One(name) if name == var_name => replacement.clone(),
        RcasesPattern::One(_) | RcasesPattern::Clear => pat.clone(),
        RcasesPattern::Tuple(pats) => RcasesPattern::Tuple(
            pats.iter()
                .map(|p| substitute_pattern(p, var_name, replacement))
                .collect(),
        ),
        RcasesPattern::Alts(pats) => RcasesPattern::Alts(
            pats.iter()
                .map(|p| substitute_pattern(p, var_name, replacement))
                .collect(),
        ),
        RcasesPattern::Typed(p, ty) => RcasesPattern::Typed(
            Box::new(substitute_pattern(p, var_name, replacement)),
            ty.clone(),
        ),
        RcasesPattern::Nested(p) => {
            RcasesPattern::Nested(Box::new(substitute_pattern(p, var_name, replacement)))
        }
    }
}
/// Check if a pattern contains a specific variable name.
pub(super) fn pattern_contains_var(pat: &RcasesPattern, var_name: &str) -> bool {
    match pat {
        RcasesPattern::One(name) => name == var_name,
        RcasesPattern::Clear => false,
        RcasesPattern::Tuple(pats) | RcasesPattern::Alts(pats) => {
            pats.iter().any(|p| pattern_contains_var(p, var_name))
        }
        RcasesPattern::Typed(p, _) => pattern_contains_var(p, var_name),
        RcasesPattern::Nested(p) => pattern_contains_var(p, var_name),
    }
}
/// Collect all free variable names in a pattern into a set.
pub(super) fn collect_pattern_vars(pat: &RcasesPattern) -> HashSet<String> {
    let mut vars = HashSet::new();
    collect_pattern_vars_impl(pat, &mut vars);
    vars
}
pub(super) fn collect_pattern_vars_impl(pat: &RcasesPattern, vars: &mut HashSet<String>) {
    match pat {
        RcasesPattern::One(name) => {
            vars.insert(name.clone());
        }
        RcasesPattern::Clear => {}
        RcasesPattern::Tuple(pats) | RcasesPattern::Alts(pats) => {
            for p in pats {
                collect_pattern_vars_impl(p, vars);
            }
        }
        RcasesPattern::Typed(p, _) => collect_pattern_vars_impl(p, vars),
        RcasesPattern::Nested(p) => collect_pattern_vars_impl(p, vars),
    }
}
/// Validate that a pattern has no duplicate variable names.
pub(super) fn check_no_duplicate_vars(pat: &RcasesPattern) -> Result<(), String> {
    let mut seen = HashSet::new();
    check_no_duplicate_vars_impl(pat, &mut seen)
}
pub(super) fn check_no_duplicate_vars_impl(
    pat: &RcasesPattern,
    seen: &mut HashSet<String>,
) -> Result<(), String> {
    match pat {
        RcasesPattern::One(name) => {
            if !seen.insert(name.clone()) {
                Err(format!("duplicate variable name '{}' in pattern", name))
            } else {
                Ok(())
            }
        }
        RcasesPattern::Clear => Ok(()),
        RcasesPattern::Tuple(pats) => {
            for p in pats {
                check_no_duplicate_vars_impl(p, seen)?;
            }
            Ok(())
        }
        RcasesPattern::Alts(pats) => {
            for p in pats {
                let mut branch_seen = seen.clone();
                check_no_duplicate_vars_impl(p, &mut branch_seen)?;
            }
            Ok(())
        }
        RcasesPattern::Typed(p, _) => check_no_duplicate_vars_impl(p, seen),
        RcasesPattern::Nested(p) => check_no_duplicate_vars_impl(p, seen),
    }
}
/// Simplify a pattern by collapsing unnecessary nesting.
///
/// - `⟨x⟩` where x is not a tuple -> `x`
/// - `x | ` (single alt) -> `x`
/// - Nested clear patterns are collapsed
pub(super) fn simplify_pattern(pat: &RcasesPattern) -> RcasesPattern {
    match pat {
        RcasesPattern::Tuple(pats) if pats.len() == 1 => simplify_pattern(&pats[0]),
        RcasesPattern::Alts(pats) if pats.len() == 1 => simplify_pattern(&pats[0]),
        RcasesPattern::Tuple(pats) => {
            RcasesPattern::Tuple(pats.iter().map(simplify_pattern).collect())
        }
        RcasesPattern::Alts(pats) => {
            RcasesPattern::Alts(pats.iter().map(simplify_pattern).collect())
        }
        RcasesPattern::Typed(p, ty) => {
            RcasesPattern::Typed(Box::new(simplify_pattern(p)), ty.clone())
        }
        RcasesPattern::Nested(p) => {
            let inner = simplify_pattern(p);
            if inner.is_simple() {
                inner
            } else {
                RcasesPattern::Nested(Box::new(inner))
            }
        }
        other => other.clone(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::rcases::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_parse_single_var() {
        let pat = parse_rcases_pattern("x").expect("pat should be present");
        assert_eq!(pat, RcasesPattern::One("x".to_string()));
    }
    #[test]
    fn test_parse_underscore() {
        let pat = parse_rcases_pattern("_").expect("pat should be present");
        assert_eq!(pat, RcasesPattern::Clear);
    }
    #[test]
    fn test_parse_tuple_angle_brackets() {
        let pat = parse_rcases_pattern("<a, b, c>").expect("pat should be present");
        assert_eq!(
            pat,
            RcasesPattern::Tuple(vec![
                RcasesPattern::One("a".to_string()),
                RcasesPattern::One("b".to_string()),
                RcasesPattern::One("c".to_string()),
            ])
        );
    }
    #[test]
    fn test_parse_alternatives() {
        let pat = parse_rcases_pattern("a | b").expect("pat should be present");
        assert_eq!(
            pat,
            RcasesPattern::Alts(vec![
                RcasesPattern::One("a".to_string()),
                RcasesPattern::One("b".to_string()),
            ])
        );
    }
    #[test]
    fn test_parse_nested_tuple() {
        let pat = parse_rcases_pattern("<a, <b, c>>").expect("pat should be present");
        assert_eq!(
            pat,
            RcasesPattern::Tuple(vec![
                RcasesPattern::One("a".to_string()),
                RcasesPattern::Tuple(vec![
                    RcasesPattern::One("b".to_string()),
                    RcasesPattern::One("c".to_string()),
                ]),
            ])
        );
    }
    #[test]
    fn test_parse_typed_pattern() {
        let pat = parse_rcases_pattern("(x : Nat)").expect("pat should be present");
        match pat {
            RcasesPattern::Typed(inner, ty) => {
                assert_eq!(*inner, RcasesPattern::One("x".to_string()));
                assert_eq!(ty, Expr::Const(Name::str("Nat"), vec![]));
            }
            _ => panic!("expected typed pattern"),
        }
    }
    #[test]
    fn test_parse_empty_input() {
        let result = parse_rcases_pattern("");
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_complex_pattern() {
        let pat = parse_rcases_pattern("<a, b> | <c, d>").expect("pat should be present");
        assert_eq!(
            pat,
            RcasesPattern::Alts(vec![
                RcasesPattern::Tuple(vec![
                    RcasesPattern::One("a".to_string()),
                    RcasesPattern::One("b".to_string()),
                ]),
                RcasesPattern::Tuple(vec![
                    RcasesPattern::One("c".to_string()),
                    RcasesPattern::One("d".to_string()),
                ]),
            ])
        );
    }
    #[test]
    fn test_pattern_is_simple() {
        assert!(RcasesPattern::One("x".to_string()).is_simple());
        assert!(RcasesPattern::Clear.is_simple());
        assert!(!RcasesPattern::Tuple(vec![]).is_simple());
        assert!(!RcasesPattern::Alts(vec![]).is_simple());
    }
    #[test]
    fn test_pattern_count_bindings() {
        let pat = RcasesPattern::Tuple(vec![
            RcasesPattern::One("a".to_string()),
            RcasesPattern::One("b".to_string()),
            RcasesPattern::Clear,
        ]);
        assert_eq!(pat.count_bindings(), 2);
    }
    #[test]
    fn test_pattern_collect_names() {
        let pat = RcasesPattern::Tuple(vec![
            RcasesPattern::One("a".to_string()),
            RcasesPattern::Tuple(vec![
                RcasesPattern::One("b".to_string()),
                RcasesPattern::One("c".to_string()),
            ]),
        ]);
        let names = pat.collect_names();
        assert_eq!(names, vec!["a", "b", "c"]);
    }
    #[test]
    fn test_pattern_display() {
        let pat = RcasesPattern::Tuple(vec![
            RcasesPattern::One("a".to_string()),
            RcasesPattern::One("b".to_string()),
        ]);
        let s = format!("{}", pat);
        assert!(s.contains("a"));
        assert!(s.contains("b"));
    }
    #[test]
    fn test_analyze_and() {
        let ctx = mk_ctx();
        let target = Expr::Const(Name::str("And"), vec![]);
        let info = analyze_inductive_type(&target, &ctx);
        assert!(info.is_some());
        let info = info.expect("info should be present");
        assert_eq!(info.constructors.len(), 1);
        assert_eq!(info.constructors[0].num_fields, 2);
        assert!(info.is_product_like());
    }
    #[test]
    fn test_analyze_or() {
        let ctx = mk_ctx();
        let target = Expr::Const(Name::str("Or"), vec![]);
        let info = analyze_inductive_type(&target, &ctx);
        assert!(info.is_some());
        let info = info.expect("info should be present");
        assert_eq!(info.constructors.len(), 2);
        assert!(info.is_sum_like());
    }
    #[test]
    fn test_classify_inductive() {
        let and_info = get_well_known_inductive_info("And").expect("and_info should be present");
        assert_eq!(classify_inductive(&and_info), InductiveKind::Product);
        let or_info = get_well_known_inductive_info("Or").expect("or_info should be present");
        assert_eq!(classify_inductive(&or_info), InductiveKind::Sum);
        let true_info = get_well_known_inductive_info("True").expect("true_info should be present");
        assert_eq!(classify_inductive(&true_info), InductiveKind::Unit);
        let false_info =
            get_well_known_inductive_info("False").expect("false_info should be present");
        assert_eq!(classify_inductive(&false_info), InductiveKind::Empty);
    }
    #[test]
    fn test_rcases_simple_binding() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let pat = RcasesPattern::One("h".to_string());
        let target = Expr::Const(Name::str("Nat"), vec![]);
        let result =
            tac_rcases(&pat, &target, &mut state, &mut ctx).expect("result should be present");
        assert_eq!(result.num_bindings(), 1);
        assert_eq!(result.bindings[0].0, Name::str("h"));
    }
    #[test]
    fn test_rcases_and_destructure() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let pat = RcasesPattern::Tuple(vec![
            RcasesPattern::One("a".to_string()),
            RcasesPattern::One("b".to_string()),
        ]);
        let target = Expr::Const(Name::str("And"), vec![]);
        let result =
            tac_rcases(&pat, &target, &mut state, &mut ctx).expect("result should be present");
        assert!(result.num_bindings() >= 2);
    }
    #[test]
    fn test_rcases_or_destructure() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let pat = RcasesPattern::Alts(vec![
            RcasesPattern::One("a".to_string()),
            RcasesPattern::One("b".to_string()),
        ]);
        let target = Expr::Const(Name::str("Or"), vec![]);
        let result =
            tac_rcases(&pat, &target, &mut state, &mut ctx).expect("result should be present");
        assert_eq!(result.goals.len(), 2);
    }
    #[test]
    fn test_obtain_simple() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Q"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let pat = RcasesPattern::One("h".to_string());
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let result = tac_obtain(&pat, &ty, &mut state, &mut ctx).expect("result should be present");
        assert!(!ctx.is_mvar_assigned(result.proof_goal));
        assert!(!result.bindings.is_empty());
    }
    #[test]
    fn test_rintro_simple() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let goal_ty = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty.clone()),
            Box::new(nat_ty),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let pats = vec![RcasesPattern::One("x".to_string())];
        let result = tac_rintro(&pats, &mut state, &mut ctx).expect("result should be present");
        assert_eq!(result.num_bindings(), 1);
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_rintro_not_pi() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let pats = vec![RcasesPattern::One("x".to_string())];
        let result = tac_rintro(&pats, &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_simplify_pattern() {
        let pat = RcasesPattern::Tuple(vec![RcasesPattern::One("x".to_string())]);
        let simplified = simplify_pattern(&pat);
        assert_eq!(simplified, RcasesPattern::One("x".to_string()));
        let pat2 = RcasesPattern::Alts(vec![RcasesPattern::One("x".to_string())]);
        let simplified2 = simplify_pattern(&pat2);
        assert_eq!(simplified2, RcasesPattern::One("x".to_string()));
    }
    #[test]
    fn test_pattern_depth() {
        assert_eq!(pattern_depth(&RcasesPattern::One("x".into())), 0);
        assert_eq!(
            pattern_depth(&RcasesPattern::Tuple(vec![RcasesPattern::One("x".into())])),
            1
        );
        assert_eq!(
            pattern_depth(&RcasesPattern::Tuple(vec![RcasesPattern::Tuple(vec![
                RcasesPattern::One("x".into())
            ])])),
            2
        );
    }
    #[test]
    fn test_check_no_duplicate_vars() {
        let pat = RcasesPattern::Tuple(vec![
            RcasesPattern::One("a".into()),
            RcasesPattern::One("b".into()),
        ]);
        assert!(check_no_duplicate_vars(&pat).is_ok());
        let dup = RcasesPattern::Tuple(vec![
            RcasesPattern::One("a".into()),
            RcasesPattern::One("a".into()),
        ]);
        assert!(check_no_duplicate_vars(&dup).is_err());
    }
    #[test]
    fn test_pattern_contains_var() {
        let pat = RcasesPattern::Tuple(vec![
            RcasesPattern::One("a".into()),
            RcasesPattern::Tuple(vec![RcasesPattern::One("b".into()), RcasesPattern::Clear]),
        ]);
        assert!(pattern_contains_var(&pat, "a"));
        assert!(pattern_contains_var(&pat, "b"));
        assert!(!pattern_contains_var(&pat, "c"));
    }
    #[test]
    fn test_rename_pattern() {
        let pat = RcasesPattern::Tuple(vec![
            RcasesPattern::One("a".into()),
            RcasesPattern::One("b".into()),
        ]);
        let renamed = rename_pattern(&pat, &|name| format!("{}_new", name));
        assert_eq!(
            renamed,
            RcasesPattern::Tuple(vec![
                RcasesPattern::One("a_new".into()),
                RcasesPattern::One("b_new".into()),
            ])
        );
    }
    #[test]
    fn test_default_pattern_for_inductive() {
        let and_info = get_well_known_inductive_info("And").expect("and_info should be present");
        let pat = default_pattern_for_inductive(&and_info);
        match &pat {
            RcasesPattern::Tuple(pats) => {
                assert_eq!(pats.len(), 2);
            }
            _ => panic!("expected tuple pattern for And"),
        }
        let or_info = get_well_known_inductive_info("Or").expect("or_info should be present");
        let pat = default_pattern_for_inductive(&or_info);
        match &pat {
            RcasesPattern::Alts(alts) => {
                assert_eq!(alts.len(), 2);
            }
            _ => panic!("expected alts pattern for Or"),
        }
    }
    #[test]
    fn test_patterns_equal_ignore_types() {
        let p1 = RcasesPattern::One("x".into());
        let p2 = RcasesPattern::Typed(
            Box::new(RcasesPattern::One("x".into())),
            Expr::Sort(Level::zero()),
        );
        assert!(patterns_equal_ignore_types(&p1, &p2));
        assert!(patterns_equal_ignore_types(&p2, &p1));
    }
    #[test]
    fn test_decompose_app() {
        let e = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("f"), vec![])),
                Box::new(Expr::Const(Name::str("a"), vec![])),
            )),
            Box::new(Expr::Const(Name::str("b"), vec![])),
        );
        let (head, args) = decompose_app(&e);
        assert_eq!(head, Expr::Const(Name::str("f"), vec![]));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_mk_app() {
        let head = Expr::Const(Name::str("f"), vec![]);
        let args = vec![
            Expr::Const(Name::str("a"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
        ];
        let result = mk_app(head, &args);
        let (head2, args2) = decompose_app(&result);
        assert_eq!(head2, Expr::Const(Name::str("f"), vec![]));
        assert_eq!(args2.len(), 2);
    }
    #[test]
    fn test_rcases_many_mismatch() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let pats = vec![RcasesPattern::One("a".into())];
        let targets = vec![
            Expr::Const(Name::str("Nat"), vec![]),
            Expr::Const(Name::str("Bool"), vec![]),
        ];
        let result = tac_rcases_many(&pats, &targets, &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_pad_pattern() {
        let pat = RcasesPattern::Tuple(vec![RcasesPattern::One("a".into())]);
        let padded = pat.pad_to(3);
        match &padded {
            RcasesPattern::Tuple(pats) => {
                assert_eq!(pats.len(), 3);
                assert_eq!(pats[0], RcasesPattern::One("a".into()));
                assert_eq!(pats[1], RcasesPattern::Clear);
                assert_eq!(pats[2], RcasesPattern::Clear);
            }
            _ => panic!("expected tuple"),
        }
    }
    #[test]
    fn test_substitute_pattern() {
        let pat = RcasesPattern::Tuple(vec![
            RcasesPattern::One("a".into()),
            RcasesPattern::One("b".into()),
        ]);
        let result = substitute_pattern(
            &pat,
            "a",
            &RcasesPattern::Tuple(vec![
                RcasesPattern::One("x".into()),
                RcasesPattern::One("y".into()),
            ]),
        );
        match &result {
            RcasesPattern::Tuple(pats) => {
                assert!(matches!(&pats[0], RcasesPattern::Tuple(_)));
                assert_eq!(pats[1], RcasesPattern::One("b".into()));
            }
            _ => panic!("expected tuple"),
        }
    }
}
