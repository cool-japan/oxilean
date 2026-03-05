//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Level, Name};
use std::collections::{HashMap, HashSet};

use super::types::{
    CommandDecl, CommandElabThroughput, CommandError, CommandHistory, CommandHistoryEntry,
    CommandPipeline, CommandResult, CommandStage, CommandState, DeclInfo, OptionValue,
    SectionState, UnivConstraint, UnivConstraintSet, ValidationResult,
};

/// Elaborate a `section Name` command.
///
/// Opens a new section scope. Variables declared inside will be abstracted
/// over when the section is closed.
pub fn elaborate_section_cmd(
    name: &str,
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    let section = SectionState::new(name.to_string());
    state.section_stack.push(section);
    Ok(CommandResult::state_change())
}
/// Elaborate an `end Name` command.
///
/// Closes the current section. All declarations inside are abstracted over
/// the section variables. Returns the list of abstracted declarations.
pub fn elaborate_end_cmd(
    name: &str,
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    let section = state
        .section_stack
        .pop()
        .ok_or(CommandError::NoOpenSection)?;
    if section.name != name {
        return Err(CommandError::SectionMismatch {
            expected: section.name.clone(),
            got: name.to_string(),
        });
    }
    let mut result = CommandResult::state_change();
    for decl_name in &section.decl_names {
        if let Some(decl_info) = state.declared_names.get(decl_name) {
            let used_vars =
                collect_used_section_vars(&decl_info.ty, decl_info.val.as_ref(), &section.vars);
            if !used_vars.is_empty() {
                let abstracted_ty = abstract_section_vars(&decl_info.ty, &used_vars);
                let abstracted_val = decl_info
                    .val
                    .as_ref()
                    .map(|v| abstract_section_vars(v, &used_vars));
                result.add_decl(CommandDecl {
                    name: decl_name.clone(),
                    ty: abstracted_ty,
                    val: abstracted_val,
                    is_universe: false,
                });
            }
        }
    }
    result.add_message(format!("end section '{}'", name));
    Ok(result)
}
/// Collect section variables that are actually used in a declaration.
///
/// Performs free variable analysis: a section variable is "used" if
/// its name appears as a `Const` reference in the type/value, or
/// conservatively if any `FVar` nodes are present (since we cannot
/// map `FVarId`s back to section variable names without a context).
fn collect_used_section_vars(
    ty: &Expr,
    val: Option<&Expr>,
    section_vars: &[(Name, Expr, BinderInfo)],
) -> Vec<(Name, Expr, BinderInfo)> {
    let mut fvar_ids: HashSet<u64> = HashSet::new();
    let mut const_names: HashSet<String> = HashSet::new();
    collect_fvars_and_consts(ty, &mut fvar_ids, &mut const_names);
    if let Some(v) = val {
        collect_fvars_and_consts(v, &mut fvar_ids, &mut const_names);
    }
    if !fvar_ids.is_empty() {
        return section_vars.to_vec();
    }
    section_vars
        .iter()
        .filter(|(name, _, _)| const_names.contains(&name.to_string()))
        .cloned()
        .collect()
}
/// Recursively collect free variable IDs *and* referenced constant names.
fn collect_fvars_and_consts(expr: &Expr, fvars: &mut HashSet<u64>, consts: &mut HashSet<String>) {
    match expr {
        Expr::FVar(fv) => {
            fvars.insert(fv.0);
        }
        Expr::Const(name, _) => {
            consts.insert(name.to_string());
        }
        Expr::App(f, a) => {
            collect_fvars_and_consts(f, fvars, consts);
            collect_fvars_and_consts(a, fvars, consts);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars_and_consts(ty, fvars, consts);
            collect_fvars_and_consts(body, fvars, consts);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars_and_consts(ty, fvars, consts);
            collect_fvars_and_consts(val, fvars, consts);
            collect_fvars_and_consts(body, fvars, consts);
        }
        Expr::Proj(_, _, e) => {
            collect_fvars_and_consts(e, fvars, consts);
        }
        Expr::Sort(_) | Expr::BVar(_) | Expr::Lit(_) => {}
    }
}
/// Recursively collect free variable IDs from an expression.
#[allow(dead_code)]
fn collect_fvars(expr: &Expr, fvars: &mut HashSet<u64>) {
    match expr {
        Expr::FVar(fv) => {
            fvars.insert(fv.0);
        }
        Expr::App(f, a) => {
            collect_fvars(f, fvars);
            collect_fvars(a, fvars);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars(ty, fvars);
            collect_fvars(body, fvars);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars(ty, fvars);
            collect_fvars(val, fvars);
            collect_fvars(body, fvars);
        }
        Expr::Proj(_, _, e) => {
            collect_fvars(e, fvars);
        }
        Expr::Sort(_) | Expr::BVar(_) | Expr::Const(_, _) | Expr::Lit(_) => {}
    }
}
/// Abstract over section variables by wrapping in Pi/Lambda binders.
///
/// For a declaration `def f : T := e` with section variable `(n : Nat)`,
/// produces `def f : (n : Nat) -> T := fun (n : Nat) => e`.
pub fn abstract_section_vars(expr: &Expr, section_vars: &[(Name, Expr, BinderInfo)]) -> Expr {
    let mut result = expr.clone();
    for (name, ty, binder_info) in section_vars.iter().rev() {
        result = Expr::Pi(
            *binder_info,
            name.clone(),
            Box::new(ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Abstract a value expression over section variables using lambdas.
pub fn abstract_section_vars_lam(expr: &Expr, section_vars: &[(Name, Expr, BinderInfo)]) -> Expr {
    let mut result = expr.clone();
    for (name, ty, binder_info) in section_vars.iter().rev() {
        result = Expr::Lam(
            *binder_info,
            name.clone(),
            Box::new(ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Elaborate a `namespace Name` command.
///
/// Pushes the namespace component onto the current namespace stack.
pub fn elaborate_namespace_cmd(
    name: &str,
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    state.current_namespace.push(name.to_string());
    Ok(CommandResult::state_change())
}
/// Elaborate an `end namespace` command.
///
/// Pops the current namespace.
pub fn elaborate_end_namespace_cmd(
    name: &str,
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    if let Some(last) = state.current_namespace.last() {
        if last != name {
            return Err(CommandError::SectionMismatch {
                expected: last.clone(),
                got: name.to_string(),
            });
        }
    } else {
        return Err(CommandError::NamespaceNotFound(name.to_string()));
    }
    state.current_namespace.pop();
    Ok(CommandResult::state_change())
}
/// Elaborate an `open Name` command.
///
/// Opens a namespace so its names can be used unqualified.
pub fn elaborate_open_cmd(
    names: &[String],
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    for name in names {
        let ns: Vec<String> = name.split('.').map(String::from).collect();
        if !state.is_namespace_open(&ns) {
            state.open_namespaces.push(ns);
        }
    }
    Ok(CommandResult::state_change())
}
/// Close an opened namespace.
pub fn elaborate_close_open_cmd(
    name: &str,
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    let ns: Vec<String> = name.split('.').map(String::from).collect();
    state.open_namespaces.retain(|open| open != &ns);
    Ok(CommandResult::state_change())
}
/// Elaborate a `variable` command.
///
/// Declares a section-scoped variable. The variable is added to the
/// current section's variable list.
pub fn elaborate_variable_cmd(
    name: &str,
    type_expr: &Expr,
    binder_info: BinderInfo,
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    let var_name = Name::str(name);
    if let Some(section) = state.current_section() {
        if section.has_var(&var_name) {
            return Err(CommandError::DuplicateVariable(name.to_string()));
        }
    }
    if let Some(section) = state.current_section_mut() {
        section.add_variable(var_name, type_expr.clone(), binder_info);
    } else {
    }
    Ok(CommandResult::state_change())
}
/// Elaborate multiple variables at once.
pub fn elaborate_variables_cmd(
    vars: &[(String, Expr, BinderInfo)],
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    for (name, ty, bi) in vars {
        elaborate_variable_cmd(name, ty, *bi, state)?;
    }
    Ok(CommandResult::state_change())
}
/// Elaborate a `universe` command.
///
/// Declares one or more universe level variables.
pub fn elaborate_universe_cmd(
    names: &[String],
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    let mut result = CommandResult::state_change();
    for name_str in names {
        let name = Name::str(name_str);
        if state.is_universe_var(&name) {
            return Err(CommandError::DuplicateUniverse(name_str.clone()));
        }
        if let Some(section) = state.current_section_mut() {
            section.add_level_param(name.clone());
        } else {
            state.universe_vars.push(name.clone());
        }
        result.add_decl(CommandDecl {
            name: name.clone(),
            ty: Expr::Sort(Level::succ(Level::Param(name))),
            val: None,
            is_universe: true,
        });
    }
    Ok(result)
}
/// Elaborate a `set_option` command.
///
/// Sets a configuration option to the given value.
pub fn elaborate_set_option_cmd(
    name: &str,
    value: &OptionValue,
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    if !state.options.contains_key(name) {}
    if let Some(existing) = state.options.get(name) {
        match (existing, value) {
            (OptionValue::Bool(_), OptionValue::Bool(_))
            | (OptionValue::Nat(_), OptionValue::Nat(_))
            | (OptionValue::Str(_), OptionValue::Str(_)) => {}
            _ => {
                return Err(CommandError::InvalidOptionValue {
                    option: name.to_string(),
                    value: format!("{}", value),
                });
            }
        }
    }
    state.options.insert(name.to_string(), value.clone());
    Ok(CommandResult::with_message(format!(
        "set_option {} {}",
        name, value
    )))
}
/// Elaborate a `#check` command.
///
/// Type-checks the expression and prints its type.
pub fn elaborate_check_cmd(
    expr: &Expr,
    state: &CommandState,
) -> Result<CommandResult, CommandError> {
    let _ = state;
    let expr_str = pretty_expr(expr);
    let ty_description = describe_type(expr);
    Ok(CommandResult::with_message(format!(
        "{} : {}",
        expr_str, ty_description
    )))
}
/// Produce a human-readable, single-line string for an expression.
fn pretty_expr(expr: &Expr) -> String {
    match expr {
        Expr::Sort(Level::Zero) => "Prop".to_string(),
        Expr::Sort(_) => "Type".to_string(),
        Expr::Lit(oxilean_kernel::Literal::Nat(n)) => format!("{}", n),
        Expr::Lit(oxilean_kernel::Literal::Str(s)) => format!("\"{}\"", s),
        Expr::Const(name, _) => name.to_string(),
        Expr::BVar(i) => format!("#{}", i),
        Expr::FVar(fv) => format!("_fvar{}", fv.0),
        Expr::Lam(_, param_name, ty, body) => {
            format!(
                "fun ({} : {}) => {}",
                param_name,
                pretty_expr(ty),
                pretty_expr(body)
            )
        }
        Expr::Pi(_, param_name, domain, codomain) => {
            let dom = pretty_expr(domain);
            let cod = pretty_expr(codomain);
            format!("({} : {}) -> {}", param_name, dom, cod)
        }
        Expr::App(f, a) => format!("({} {})", pretty_expr(f), pretty_expr(a)),
        Expr::Let(name, ty, val, body) => {
            format!(
                "let {} : {} := {}; {}",
                name,
                pretty_expr(ty),
                pretty_expr(val),
                pretty_expr(body)
            )
        }
        Expr::Proj(struct_name, idx, e) => {
            format!("{}.{} ({})", struct_name, idx, pretty_expr(e))
        }
    }
}
/// Produce a human-readable type description for an expression.
///
/// Returns the syntactic type of the expression when it can be determined
/// without a full type-inference pass. Falls back to descriptive strings
/// for complex cases.
fn describe_type(expr: &Expr) -> String {
    match expr {
        Expr::Sort(Level::Zero) => "Prop".to_string(),
        Expr::Sort(_) => "Type".to_string(),
        Expr::Lit(oxilean_kernel::Literal::Nat(_)) => "Nat".to_string(),
        Expr::Lit(oxilean_kernel::Literal::Str(_)) => "String".to_string(),
        Expr::Lam(_, _, param_ty, _) => format!("{} -> _", pretty_expr(param_ty)),
        Expr::Pi(_, _, domain, codomain) => {
            format!("{} -> {}", pretty_expr(domain), pretty_expr(codomain))
        }
        Expr::Const(name, _) => {
            let n = name.to_string();
            match n.as_str() {
                "Nat" | "Int" | "Bool" | "String" | "Float" => "Type".to_string(),
                "True" | "False" => "Prop".to_string(),
                _ => format!("?{}", n),
            }
        }
        Expr::App(f, _) => {
            let head = pretty_expr(f);
            format!("({} _)", head)
        }
        Expr::Let(_, _, _, body) => describe_type(body),
        Expr::Proj(name, idx, _) => format!("_.{}.{}", name, idx),
        Expr::FVar(_) | Expr::BVar(_) => "_".to_string(),
    }
}
/// Elaborate a `#eval` command.
///
/// Evaluates the expression and prints the result.
pub fn elaborate_eval_cmd(
    expr: &Expr,
    state: &CommandState,
) -> Result<CommandResult, CommandError> {
    let _ = state;
    let result = evaluate_simple(expr);
    Ok(CommandResult::with_message(result))
}
/// Simple evaluation for literals and constants.
///
/// Reduces literal arithmetic and string operations; falls back to
/// `pretty_expr` for complex expressions.
fn evaluate_simple(expr: &Expr) -> String {
    match expr {
        Expr::Lit(oxilean_kernel::Literal::Nat(n)) => format!("{}", n),
        Expr::Lit(oxilean_kernel::Literal::Str(s)) => format!("\"{}\"", s),
        Expr::Const(name, _) => name.to_string(),
        Expr::App(f, a) => {
            if let Expr::App(op, lhs) = f.as_ref() {
                if let Expr::Const(op_name, _) = op.as_ref() {
                    let lv = evaluate_simple(lhs);
                    let rv = evaluate_simple(a);
                    let op_str = op_name.to_string();
                    if let (Ok(l), Ok(r)) = (lv.parse::<u64>(), rv.parse::<u64>()) {
                        match op_str.as_str() {
                            "HAdd.hAdd" | "Nat.add" => return format!("{}", l + r),
                            "HMul.hMul" | "Nat.mul" => return format!("{}", l * r),
                            "HSub.hSub" | "Nat.sub" => {
                                return format!("{}", l.saturating_sub(r));
                            }
                            _ => {}
                        }
                    }
                    if op_str == "String.append" {
                        if let (Some(ls), Some(rs)) = (
                            lv.strip_prefix('"').and_then(|s| s.strip_suffix('"')),
                            rv.strip_prefix('"').and_then(|s| s.strip_suffix('"')),
                        ) {
                            return format!("\"{}{}\"", ls, rs);
                        }
                    }
                    return format!("({} {} {})", op_str, lv, rv);
                }
            }
            format!("({} {})", evaluate_simple(f), evaluate_simple(a))
        }
        _ => pretty_expr(expr),
    }
}
/// Elaborate a `#print` command.
///
/// Prints the definition of a name.
pub fn elaborate_print_cmd(
    name: &str,
    state: &CommandState,
) -> Result<CommandResult, CommandError> {
    let kernel_name = Name::str(name);
    if let Some(info) = state.lookup_decl(&kernel_name) {
        let mut msg = format!("{} : {:?}", name, info.ty);
        if let Some(ref val) = info.val {
            msg.push_str(&format!(" := {:?}", val));
        }
        if info.is_theorem {
            msg = format!("theorem {}", msg);
        } else {
            msg = format!("def {}", msg);
        }
        return Ok(CommandResult::with_message(msg));
    }
    let qualified = state.qualify_name(name);
    if let Some(info) = state.lookup_decl(&qualified) {
        let mut msg = format!("{} : {:?}", qualified, info.ty);
        if let Some(ref val) = info.val {
            msg.push_str(&format!(" := {:?}", val));
        }
        return Ok(CommandResult::with_message(msg));
    }
    Err(CommandError::NameNotFound(name.to_string()))
}
/// Elaborate an `attribute [attr] name` command.
pub fn elaborate_attribute_cmd(
    attrs: &[String],
    _name: &str,
    state: &mut CommandState,
) -> Result<CommandResult, CommandError> {
    for attr in attrs {
        if !state.pending_attributes.contains(attr) {
            state.pending_attributes.push(attr.clone());
        }
    }
    Ok(CommandResult::state_change())
}
/// Consume and return pending attributes.
pub fn take_pending_attributes(state: &mut CommandState) -> Vec<String> {
    std::mem::take(&mut state.pending_attributes)
}
/// Resolve a name using the current namespace and open namespaces.
///
/// Tries:
/// 1. Fully qualified name.
/// 2. Current namespace + name.
/// 3. Each open namespace + name.
pub fn resolve_name(name: &str, state: &CommandState) -> Vec<Name> {
    let mut candidates = Vec::new();
    let fq = Name::str(name);
    if state.declared_names.contains_key(&fq) {
        candidates.push(fq);
    }
    let qualified = state.qualify_name(name);
    if state.declared_names.contains_key(&qualified) && !candidates.contains(&qualified) {
        candidates.push(qualified);
    }
    for ns in &state.open_namespaces {
        let ns_name = Name::str(format!("{}.{}", ns.join("."), name));
        if state.declared_names.contains_key(&ns_name) && !candidates.contains(&ns_name) {
            candidates.push(ns_name);
        }
    }
    candidates
}
/// Check if a name resolves to exactly one definition.
pub fn resolve_unique_name(name: &str, state: &CommandState) -> Result<Name, CommandError> {
    let candidates = resolve_name(name, state);
    match candidates.len() {
        0 => Err(CommandError::NameNotFound(name.to_string())),
        1 => Ok(candidates
            .into_iter()
            .next()
            .expect("candidates has exactly one element")),
        _ => Err(CommandError::ElabError(format!(
            "ambiguous name '{}': multiple definitions found",
            name
        ))),
    }
}
/// Scoped state modification: run a closure with temporary state changes,
/// then restore the state.
pub fn with_scope<F, R>(state: &mut CommandState, f: F) -> R
where
    F: FnOnce(&mut CommandState) -> R,
{
    let saved_ns = state.current_namespace.clone();
    let saved_open = state.open_namespaces.clone();
    let saved_attrs = state.pending_attributes.clone();
    let result = f(state);
    state.current_namespace = saved_ns;
    state.open_namespaces = saved_open;
    state.pending_attributes = saved_attrs;
    result
}
/// Enter a scoped namespace for the duration of a closure.
pub fn with_namespace<F, R>(ns: &str, state: &mut CommandState, f: F) -> R
where
    F: FnOnce(&mut CommandState) -> R,
{
    state.current_namespace.push(ns.to_string());
    let result = f(state);
    state.current_namespace.pop();
    result
}
/// Enter a scoped section for the duration of a closure.
pub fn with_section<F, R>(name: &str, state: &mut CommandState, f: F) -> R
where
    F: FnOnce(&mut CommandState) -> R,
{
    state
        .section_stack
        .push(SectionState::new(name.to_string()));
    let result = f(state);
    state.section_stack.pop();
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_elab::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn prop_ty() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn type_ty() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    #[test]
    fn test_error_display() {
        let e = CommandError::SectionMismatch {
            expected: "A".into(),
            got: "B".into(),
        };
        assert!(format!("{}", e).contains("mismatch"));
        let e = CommandError::NoOpenSection;
        assert!(format!("{}", e).contains("no section"));
        let e = CommandError::NamespaceNotFound("Foo".into());
        assert!(format!("{}", e).contains("Foo"));
        let e = CommandError::DuplicateVariable("x".into());
        assert!(format!("{}", e).contains("duplicate"));
        let e = CommandError::DuplicateUniverse("u".into());
        assert!(format!("{}", e).contains("duplicate"));
        let e = CommandError::UnknownOption("pp.all".into());
        assert!(format!("{}", e).contains("unknown"));
        let e = CommandError::InvalidOptionValue {
            option: "pp.all".into(),
            value: "42".into(),
        };
        assert!(format!("{}", e).contains("invalid"));
        let e = CommandError::NameNotFound("foo".into());
        assert!(format!("{}", e).contains("not found"));
        let e = CommandError::ElabError("oops".into());
        assert!(format!("{}", e).contains("oops"));
    }
    #[test]
    fn test_command_result_empty() {
        let r = CommandResult::empty();
        assert!(r.messages.is_empty());
        assert!(r.declarations.is_empty());
        assert!(!r.state_modified);
    }
    #[test]
    fn test_command_result_with_message() {
        let r = CommandResult::with_message("hello".to_string());
        assert_eq!(r.messages.len(), 1);
        assert_eq!(r.messages[0], "hello");
    }
    #[test]
    fn test_option_value_display() {
        assert_eq!(format!("{}", OptionValue::Bool(true)), "true");
        assert_eq!(format!("{}", OptionValue::Nat(42)), "42");
        assert_eq!(format!("{}", OptionValue::Str("hi".into())), "\"hi\"");
    }
    #[test]
    fn test_section_state() {
        let mut section = SectionState::new("Test".to_string());
        assert_eq!(section.name, "Test");
        assert_eq!(section.num_vars(), 0);
        section.add_variable(Name::str("n"), nat_ty(), BinderInfo::Default);
        assert_eq!(section.num_vars(), 1);
        assert!(section.has_var(&Name::str("n")));
        assert!(!section.has_var(&Name::str("m")));
        section.add_level_param(Name::str("u"));
        assert_eq!(section.num_level_params(), 1);
        section.record_decl(Name::str("foo"));
        assert_eq!(section.decl_names.len(), 1);
        let var_names = section.var_names();
        assert_eq!(var_names.len(), 1);
        assert_eq!(var_names[0], &Name::str("n"));
    }
    #[test]
    fn test_section_state_lookup() {
        let mut section = SectionState::new("S".to_string());
        section.add_variable(Name::str("x"), nat_ty(), BinderInfo::Default);
        section.add_variable(Name::str("y"), prop_ty(), BinderInfo::Implicit);
        let v = section.lookup_var(&Name::str("x"));
        assert!(v.is_some());
        let (n, _, bi) = v.expect("test operation should succeed");
        assert_eq!(n, &Name::str("x"));
        assert_eq!(*bi, BinderInfo::Default);
        assert!(section.lookup_var(&Name::str("z")).is_none());
    }
    #[test]
    fn test_command_state_new() {
        let state = CommandState::new();
        assert!(state.current_namespace.is_empty());
        assert!(state.open_namespaces.is_empty());
        assert!(!state.has_open_section());
        assert!(!state.options.is_empty());
    }
    #[test]
    fn test_command_state_namespace() {
        let state = CommandState::new();
        assert_eq!(state.namespace_str(), "");
        let mut state = state;
        state.current_namespace.push("Foo".to_string());
        state.current_namespace.push("Bar".to_string());
        assert_eq!(state.namespace_str(), "Foo.Bar");
        let qn = state.qualify_name("baz");
        assert_eq!(qn, Name::str("Foo.Bar.baz"));
    }
    #[test]
    fn test_command_state_section_vars() {
        let mut state = CommandState::new();
        elaborate_section_cmd("S1", &mut state).expect("elaboration should succeed");
        assert!(state.has_open_section());
        assert_eq!(state.section_depth(), 1);
        elaborate_variable_cmd("n", &nat_ty(), BinderInfo::Default, &mut state)
            .expect("elaboration should succeed");
        assert!(state.is_section_var(&Name::str("n")));
        assert!(!state.is_section_var(&Name::str("m")));
        let vars = state.all_section_vars();
        assert_eq!(vars.len(), 1);
        elaborate_section_cmd("S2", &mut state).expect("elaboration should succeed");
        assert_eq!(state.section_depth(), 2);
        elaborate_variable_cmd("m", &nat_ty(), BinderInfo::Implicit, &mut state)
            .expect("elaboration should succeed");
        let vars = state.all_section_vars();
        assert_eq!(vars.len(), 2);
        elaborate_end_cmd("S2", &mut state).expect("elaboration should succeed");
        assert_eq!(state.section_depth(), 1);
        let vars = state.all_section_vars();
        assert_eq!(vars.len(), 1);
        elaborate_end_cmd("S1", &mut state).expect("elaboration should succeed");
        assert_eq!(state.section_depth(), 0);
    }
    #[test]
    fn test_section_open_close() {
        let mut state = CommandState::new();
        let r = elaborate_section_cmd("Test", &mut state).expect("elaboration should succeed");
        assert!(r.state_modified);
        assert!(state.has_open_section());
        let r = elaborate_end_cmd("Test", &mut state).expect("elaboration should succeed");
        assert!(r.state_modified);
        assert!(!state.has_open_section());
    }
    #[test]
    fn test_section_mismatch() {
        let mut state = CommandState::new();
        elaborate_section_cmd("A", &mut state).expect("elaboration should succeed");
        let result = elaborate_end_cmd("B", &mut state);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CommandError::SectionMismatch { .. }
        ));
    }
    #[test]
    fn test_end_no_section() {
        let mut state = CommandState::new();
        let result = elaborate_end_cmd("A", &mut state);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommandError::NoOpenSection));
    }
    #[test]
    fn test_namespace_open_close() {
        let mut state = CommandState::new();
        elaborate_namespace_cmd("Foo", &mut state).expect("elaboration should succeed");
        assert_eq!(state.namespace_str(), "Foo");
        assert_eq!(state.namespace_depth(), 1);
        elaborate_namespace_cmd("Bar", &mut state).expect("elaboration should succeed");
        assert_eq!(state.namespace_str(), "Foo.Bar");
        assert_eq!(state.namespace_depth(), 2);
        elaborate_end_namespace_cmd("Bar", &mut state).expect("elaboration should succeed");
        assert_eq!(state.namespace_str(), "Foo");
        elaborate_end_namespace_cmd("Foo", &mut state).expect("elaboration should succeed");
        assert_eq!(state.namespace_str(), "");
    }
    #[test]
    fn test_namespace_mismatch() {
        let mut state = CommandState::new();
        elaborate_namespace_cmd("Foo", &mut state).expect("elaboration should succeed");
        let result = elaborate_end_namespace_cmd("Bar", &mut state);
        assert!(result.is_err());
    }
    #[test]
    fn test_end_namespace_empty() {
        let mut state = CommandState::new();
        let result = elaborate_end_namespace_cmd("Foo", &mut state);
        assert!(result.is_err());
    }
    #[test]
    fn test_open_cmd() {
        let mut state = CommandState::new();
        elaborate_open_cmd(&["Nat".to_string(), "List".to_string()], &mut state)
            .expect("elaboration should succeed");
        assert!(state.is_namespace_open(&["Nat".to_string()]));
        assert!(state.is_namespace_open(&["List".to_string()]));
        assert!(!state.is_namespace_open(&["Foo".to_string()]));
    }
    #[test]
    fn test_open_cmd_dotted() {
        let mut state = CommandState::new();
        elaborate_open_cmd(&["Mathlib.Data.Nat".to_string()], &mut state)
            .expect("elaboration should succeed");
        assert!(state.is_namespace_open(&[
            "Mathlib".to_string(),
            "Data".to_string(),
            "Nat".to_string()
        ]));
    }
    #[test]
    fn test_open_cmd_idempotent() {
        let mut state = CommandState::new();
        elaborate_open_cmd(&["Nat".to_string()], &mut state).expect("elaboration should succeed");
        elaborate_open_cmd(&["Nat".to_string()], &mut state).expect("elaboration should succeed");
        assert_eq!(state.open_namespaces.len(), 1);
    }
    #[test]
    fn test_close_open_cmd() {
        let mut state = CommandState::new();
        elaborate_open_cmd(&["Nat".to_string()], &mut state).expect("elaboration should succeed");
        elaborate_close_open_cmd("Nat", &mut state).expect("elaboration should succeed");
        assert!(state.open_namespaces.is_empty());
    }
    #[test]
    fn test_variable_cmd() {
        let mut state = CommandState::new();
        elaborate_section_cmd("S", &mut state).expect("elaboration should succeed");
        let result = elaborate_variable_cmd("n", &nat_ty(), BinderInfo::Default, &mut state);
        assert!(result.is_ok());
        assert!(state.is_section_var(&Name::str("n")));
    }
    #[test]
    fn test_variable_cmd_duplicate() {
        let mut state = CommandState::new();
        elaborate_section_cmd("S", &mut state).expect("elaboration should succeed");
        elaborate_variable_cmd("n", &nat_ty(), BinderInfo::Default, &mut state)
            .expect("elaboration should succeed");
        let result = elaborate_variable_cmd("n", &nat_ty(), BinderInfo::Default, &mut state);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CommandError::DuplicateVariable(_)
        ));
    }
    #[test]
    fn test_variables_cmd() {
        let mut state = CommandState::new();
        elaborate_section_cmd("S", &mut state).expect("elaboration should succeed");
        let vars = vec![
            ("x".to_string(), nat_ty(), BinderInfo::Default),
            ("y".to_string(), nat_ty(), BinderInfo::Default),
        ];
        elaborate_variables_cmd(&vars, &mut state).expect("elaboration should succeed");
        assert!(state.is_section_var(&Name::str("x")));
        assert!(state.is_section_var(&Name::str("y")));
    }
    #[test]
    fn test_universe_cmd() {
        let mut state = CommandState::new();
        let result = elaborate_universe_cmd(&["u".to_string(), "v".to_string()], &mut state);
        assert!(result.is_ok());
        let r = result.expect("test operation should succeed");
        assert_eq!(r.declarations.len(), 2);
        assert!(state.is_universe_var(&Name::str("u")));
        assert!(state.is_universe_var(&Name::str("v")));
    }
    #[test]
    fn test_universe_cmd_duplicate() {
        let mut state = CommandState::new();
        elaborate_universe_cmd(&["u".to_string()], &mut state).expect("elaboration should succeed");
        let result = elaborate_universe_cmd(&["u".to_string()], &mut state);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CommandError::DuplicateUniverse(_)
        ));
    }
    #[test]
    fn test_universe_in_section() {
        let mut state = CommandState::new();
        elaborate_section_cmd("S", &mut state).expect("elaboration should succeed");
        elaborate_universe_cmd(&["u".to_string()], &mut state).expect("elaboration should succeed");
        assert!(state.is_universe_var(&Name::str("u")));
        let all = state.all_universe_vars();
        assert!(all.contains(&Name::str("u")));
        elaborate_end_cmd("S", &mut state).expect("elaboration should succeed");
        assert!(!state.is_universe_var(&Name::str("u")));
    }
    #[test]
    fn test_set_option_bool() {
        let mut state = CommandState::new();
        let result = elaborate_set_option_cmd("pp.all", &OptionValue::Bool(true), &mut state);
        assert!(result.is_ok());
        assert_eq!(state.get_option("pp.all"), Some(&OptionValue::Bool(true)));
    }
    #[test]
    fn test_set_option_nat() {
        let mut state = CommandState::new();
        let result =
            elaborate_set_option_cmd("maxHeartbeats", &OptionValue::Nat(400000), &mut state);
        assert!(result.is_ok());
        assert_eq!(
            state.get_option("maxHeartbeats"),
            Some(&OptionValue::Nat(400000))
        );
    }
    #[test]
    fn test_set_option_type_mismatch() {
        let mut state = CommandState::new();
        let result = elaborate_set_option_cmd("pp.all", &OptionValue::Nat(42), &mut state);
        assert!(result.is_err());
    }
    #[test]
    fn test_set_option_new() {
        let mut state = CommandState::new();
        let result =
            elaborate_set_option_cmd("custom.option", &OptionValue::Bool(true), &mut state);
        assert!(result.is_ok());
        assert_eq!(
            state.get_option("custom.option"),
            Some(&OptionValue::Bool(true))
        );
    }
    #[test]
    fn test_check_cmd_nat() {
        let state = CommandState::new();
        let result = elaborate_check_cmd(&Expr::Lit(oxilean_kernel::Literal::Nat(42)), &state);
        assert!(result.is_ok());
        assert!(!result
            .expect("test operation should succeed")
            .messages
            .is_empty());
    }
    #[test]
    fn test_check_cmd_sort() {
        let state = CommandState::new();
        let result = elaborate_check_cmd(&prop_ty(), &state);
        assert!(result.is_ok());
    }
    #[test]
    fn test_eval_cmd_nat() {
        let state = CommandState::new();
        let result = elaborate_eval_cmd(&Expr::Lit(oxilean_kernel::Literal::Nat(42)), &state);
        assert!(result.is_ok());
        let r = result.expect("test operation should succeed");
        assert!(r.messages[0].contains("42"));
    }
    #[test]
    fn test_eval_cmd_string() {
        let state = CommandState::new();
        let result = elaborate_eval_cmd(
            &Expr::Lit(oxilean_kernel::Literal::Str("hello".to_string())),
            &state,
        );
        assert!(result.is_ok());
        let r = result.expect("test operation should succeed");
        assert!(r.messages[0].contains("hello"));
    }
    #[test]
    fn test_print_cmd_found() {
        let mut state = CommandState::new();
        state.register_decl(
            Name::str("foo"),
            DeclInfo {
                ty: nat_ty(),
                val: Some(Expr::Lit(oxilean_kernel::Literal::Nat(42))),
                namespace: vec![],
                is_theorem: false,
                univ_params: vec![],
            },
        );
        let result = elaborate_print_cmd("foo", &state);
        assert!(result.is_ok());
        let r = result.expect("test operation should succeed");
        assert!(r.messages[0].contains("foo"));
    }
    #[test]
    fn test_print_cmd_not_found() {
        let state = CommandState::new();
        let result = elaborate_print_cmd("nonexistent", &state);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommandError::NameNotFound(_)));
    }
    #[test]
    fn test_print_cmd_theorem() {
        let mut state = CommandState::new();
        state.register_decl(
            Name::str("my_thm"),
            DeclInfo {
                ty: prop_ty(),
                val: None,
                namespace: vec![],
                is_theorem: true,
                univ_params: vec![],
            },
        );
        let result = elaborate_print_cmd("my_thm", &state);
        assert!(result.is_ok());
        let r = result.expect("test operation should succeed");
        assert!(r.messages[0].contains("theorem"));
    }
    #[test]
    fn test_abstract_section_vars_pi() {
        let body = nat_ty();
        let vars = vec![(Name::str("n"), nat_ty(), BinderInfo::Default)];
        let result = abstract_section_vars(&body, &vars);
        assert!(result.is_pi());
    }
    #[test]
    fn test_abstract_section_vars_lam() {
        let body = Expr::BVar(0);
        let vars = vec![(Name::str("n"), nat_ty(), BinderInfo::Default)];
        let result = abstract_section_vars_lam(&body, &vars);
        assert!(result.is_lambda());
    }
    #[test]
    fn test_abstract_section_vars_multiple() {
        let body = nat_ty();
        let vars = vec![
            (Name::str("a"), nat_ty(), BinderInfo::Default),
            (Name::str("b"), nat_ty(), BinderInfo::Implicit),
        ];
        let result = abstract_section_vars(&body, &vars);
        assert!(result.is_pi());
        if let Expr::Pi(_, _, _, inner) = &result {
            assert!(inner.is_pi());
        }
    }
    #[test]
    fn test_abstract_section_vars_empty() {
        let body = nat_ty();
        let result = abstract_section_vars(&body, &[]);
        assert_eq!(result, body);
    }
    #[test]
    fn test_resolve_name_simple() {
        let mut state = CommandState::new();
        state.register_decl(
            Name::str("foo"),
            DeclInfo {
                ty: nat_ty(),
                val: None,
                namespace: vec![],
                is_theorem: false,
                univ_params: vec![],
            },
        );
        let candidates = resolve_name("foo", &state);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0], Name::str("foo"));
    }
    #[test]
    fn test_resolve_name_qualified() {
        let mut state = CommandState::new();
        state.current_namespace.push("Ns".to_string());
        state.register_decl(
            Name::str("Ns.bar"),
            DeclInfo {
                ty: nat_ty(),
                val: None,
                namespace: vec!["Ns".to_string()],
                is_theorem: false,
                univ_params: vec![],
            },
        );
        let candidates = resolve_name("bar", &state);
        assert!(candidates.contains(&Name::str("Ns.bar")));
    }
    #[test]
    fn test_resolve_name_open_namespace() {
        let mut state = CommandState::new();
        state.open_namespaces.push(vec!["Nat".to_string()]);
        state.register_decl(
            Name::str("Nat.add"),
            DeclInfo {
                ty: nat_ty(),
                val: None,
                namespace: vec!["Nat".to_string()],
                is_theorem: false,
                univ_params: vec![],
            },
        );
        let candidates = resolve_name("add", &state);
        assert!(candidates.contains(&Name::str("Nat.add")));
    }
    #[test]
    fn test_resolve_unique_name_ok() {
        let mut state = CommandState::new();
        state.register_decl(
            Name::str("foo"),
            DeclInfo {
                ty: nat_ty(),
                val: None,
                namespace: vec![],
                is_theorem: false,
                univ_params: vec![],
            },
        );
        let result = resolve_unique_name("foo", &state);
        assert!(result.is_ok());
    }
    #[test]
    fn test_resolve_unique_name_not_found() {
        let state = CommandState::new();
        let result = resolve_unique_name("nonexistent", &state);
        assert!(result.is_err());
    }
    #[test]
    fn test_with_namespace() {
        let mut state = CommandState::new();
        let ns_inside = with_namespace("Foo", &mut state, |s| s.namespace_str());
        assert_eq!(ns_inside, "Foo");
        assert_eq!(state.namespace_str(), "");
    }
    #[test]
    fn test_with_section() {
        let mut state = CommandState::new();
        let depth_inside = with_section("S", &mut state, |s| s.section_depth());
        assert_eq!(depth_inside, 1);
        assert_eq!(state.section_depth(), 0);
    }
    #[test]
    fn test_with_scope() {
        let mut state = CommandState::new();
        state.current_namespace.push("Original".to_string());
        with_scope(&mut state, |s| {
            s.current_namespace.push("Temp".to_string());
            assert_eq!(s.namespace_str(), "Original.Temp");
        });
        assert_eq!(state.namespace_str(), "Original");
    }
    #[test]
    fn test_attribute_cmd() {
        let mut state = CommandState::new();
        elaborate_attribute_cmd(
            &["simp".to_string(), "ext".to_string()],
            "my_lemma",
            &mut state,
        )
        .expect("test operation should succeed");
        assert_eq!(state.pending_attributes.len(), 2);
        let attrs = take_pending_attributes(&mut state);
        assert_eq!(attrs.len(), 2);
        assert!(state.pending_attributes.is_empty());
    }
    #[test]
    fn test_describe_type() {
        assert_eq!(describe_type(&prop_ty()), "Prop");
        assert_eq!(describe_type(&type_ty()), "Type");
        assert_eq!(
            describe_type(&Expr::Lit(oxilean_kernel::Literal::Nat(42))),
            "Nat"
        );
        assert_eq!(
            describe_type(&Expr::Lit(oxilean_kernel::Literal::Str("hi".into()))),
            "String"
        );
    }
    #[test]
    fn test_collect_fvars() {
        let mut fvars = HashSet::new();
        let expr = Expr::App(
            Box::new(Expr::FVar(oxilean_kernel::FVarId::new(1))),
            Box::new(Expr::FVar(oxilean_kernel::FVarId::new(2))),
        );
        collect_fvars(&expr, &mut fvars);
        assert!(fvars.contains(&1));
        assert!(fvars.contains(&2));
    }
    #[test]
    fn test_collect_fvars_lambda() {
        let mut fvars = HashSet::new();
        let expr = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::FVar(oxilean_kernel::FVarId::new(10))),
            Box::new(Expr::FVar(oxilean_kernel::FVarId::new(20))),
        );
        collect_fvars(&expr, &mut fvars);
        assert!(fvars.contains(&10));
        assert!(fvars.contains(&20));
    }
    #[test]
    fn test_default_state() {
        let state = CommandState::default();
        assert!(state.current_namespace.is_empty());
        assert!(state.get_option("pp.all").is_some());
    }
    #[test]
    fn test_all_universe_vars() {
        let mut state = CommandState::new();
        elaborate_universe_cmd(&["u".to_string()], &mut state).expect("elaboration should succeed");
        elaborate_section_cmd("S", &mut state).expect("elaboration should succeed");
        elaborate_universe_cmd(&["v".to_string()], &mut state).expect("elaboration should succeed");
        let all = state.all_universe_vars();
        assert!(all.contains(&Name::str("u")));
        assert!(all.contains(&Name::str("v")));
    }
    #[test]
    fn test_evaluate_simple_nat() {
        assert_eq!(
            evaluate_simple(&Expr::Lit(oxilean_kernel::Literal::Nat(7))),
            "7"
        );
    }
    #[test]
    fn test_evaluate_simple_string() {
        assert_eq!(
            evaluate_simple(&Expr::Lit(oxilean_kernel::Literal::Str("hi".into()))),
            "\"hi\""
        );
    }
    #[test]
    fn test_evaluate_simple_const() {
        assert_eq!(
            evaluate_simple(&Expr::Const(Name::str("Nat.zero"), vec![])),
            "Nat.zero"
        );
    }
    #[test]
    fn test_lookup_section_var_nested() {
        let mut state = CommandState::new();
        elaborate_section_cmd("Outer", &mut state).expect("elaboration should succeed");
        elaborate_variable_cmd("a", &nat_ty(), BinderInfo::Default, &mut state)
            .expect("elaboration should succeed");
        elaborate_section_cmd("Inner", &mut state).expect("elaboration should succeed");
        elaborate_variable_cmd("b", &nat_ty(), BinderInfo::Implicit, &mut state)
            .expect("elaboration should succeed");
        assert!(state.lookup_section_var(&Name::str("a")).is_some());
        assert!(state.lookup_section_var(&Name::str("b")).is_some());
        assert!(state.lookup_section_var(&Name::str("c")).is_none());
        elaborate_end_cmd("Inner", &mut state).expect("elaboration should succeed");
        assert!(state.lookup_section_var(&Name::str("a")).is_some());
        assert!(state.lookup_section_var(&Name::str("b")).is_none());
        elaborate_end_cmd("Outer", &mut state).expect("elaboration should succeed");
    }
}
/// Validate a single `CommandDecl` against a `CommandState`.
#[allow(dead_code)]
pub fn validate_decl(decl: &CommandDecl, state: &CommandState) -> ValidationResult {
    let mut result = ValidationResult::ok(decl.name.clone());
    if state.lookup_section_var(&decl.name).is_some() {
        result.add_error(CommandError::ElabError(format!(
            "name '{}' already declared as a section variable",
            decl.name,
        )));
    }
    if decl.name == Name::str("_") {
        result.add_warning("declaration name '_' is a wildcard — likely unintentional");
    }
    result
}
#[cfg(test)]
mod command_elab_ext_tests {
    use super::*;
    use crate::command_elab::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_validation_result_ok() {
        let r = ValidationResult::ok(Name::str("f"));
        assert!(r.is_ok());
        assert!(r.passed);
        assert_eq!(r.num_diagnostics(), 0);
    }
    #[test]
    fn test_validation_result_err() {
        let r = ValidationResult::err(Name::str("f"), CommandError::ElabError("bad".to_string()));
        assert!(!r.is_ok());
        assert!(!r.passed);
        assert_eq!(r.errors.len(), 1);
    }
    #[test]
    fn test_validation_result_add_warning() {
        let mut r = ValidationResult::ok(Name::str("f"));
        r.add_warning("unused import");
        assert_eq!(r.warnings.len(), 1);
        assert!(r.is_ok());
    }
    #[test]
    fn test_validation_result_add_error() {
        let mut r = ValidationResult::ok(Name::str("f"));
        r.add_error(CommandError::ElabError("bad".to_string()));
        assert!(!r.is_ok());
        assert!(!r.passed);
    }
    #[test]
    fn test_univ_constraint_set_add_len() {
        let mut s = UnivConstraintSet::new();
        s.add(UnivConstraint::IsParam(Level::zero()));
        s.add(UnivConstraint::EqLevel(Level::zero(), Level::zero()));
        assert_eq!(s.len(), 2);
    }
    #[test]
    fn test_univ_constraint_set_params() {
        let mut s = UnivConstraintSet::new();
        s.add(UnivConstraint::IsParam(Level::zero()));
        s.add(UnivConstraint::EqLevel(Level::zero(), Level::zero()));
        assert_eq!(s.params().len(), 1);
    }
    #[test]
    fn test_univ_constraint_set_deduplicate() {
        let mut s = UnivConstraintSet::new();
        s.add(UnivConstraint::IsParam(Level::zero()));
        s.add(UnivConstraint::IsParam(Level::zero()));
        s.deduplicate();
        assert_eq!(s.len(), 1);
    }
    #[test]
    fn test_command_stage_display() {
        assert_eq!(format!("{}", CommandStage::Parse), "Parse");
        assert_eq!(format!("{}", CommandStage::Done), "Done");
    }
    #[test]
    fn test_command_stage_ordering() {
        assert!(CommandStage::Parse < CommandStage::Resolve);
        assert!(CommandStage::TypeCheck < CommandStage::Done);
    }
    #[test]
    fn test_command_pipeline_advance_to_done() {
        let mut p = CommandPipeline::new(Name::str("f"));
        for _ in 0..6 {
            p.advance();
        }
        assert!(p.is_done());
        assert!(p.is_success());
    }
    #[test]
    fn test_command_pipeline_abort() {
        let mut p = CommandPipeline::new(Name::str("f"));
        p.advance();
        p.abort(CommandError::ElabError("bad".to_string()));
        assert!(p.is_done());
        assert!(!p.is_success());
        assert!(p.error.is_some());
    }
    #[test]
    fn test_command_pipeline_warn() {
        let mut p = CommandPipeline::new(Name::str("f"));
        p.warn("unused variable");
        assert_eq!(p.warnings.len(), 1);
    }
    #[test]
    fn test_command_history_record_success() {
        let mut h = CommandHistory::new();
        h.record(CommandHistoryEntry::success(Name::str("f")));
        assert_eq!(h.len(), 1);
        assert_eq!(h.successes(), 1);
        assert_eq!(h.failures(), 0);
        assert!((h.success_rate() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_command_history_record_failure() {
        let mut h = CommandHistory::new();
        h.record(CommandHistoryEntry::failure(
            Name::str("g"),
            CommandStage::TypeCheck,
        ));
        assert_eq!(h.failures(), 1);
        assert!((h.success_rate() - 0.0).abs() < 1e-10);
        let failed = h.failed_entries();
        assert_eq!(failed.len(), 1);
    }
    #[test]
    fn test_command_history_empty() {
        let h = CommandHistory::new();
        assert!(h.is_empty());
        assert!((h.success_rate() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_validate_decl_ok() {
        let state = CommandState::new();
        let decl = CommandDecl {
            name: Name::str("myFunc"),
            ty: nat_ty(),
            val: None,
            is_universe: false,
        };
        let r = validate_decl(&decl, &state);
        assert!(r.is_ok());
    }
}
#[cfg(test)]
mod command_throughput_tests {
    use super::*;
    use crate::command_elab::*;
    #[test]
    fn test_throughput_record_and_rate() {
        let mut t = CommandElabThroughput::new();
        t.record(true, 100);
        t.record(true, 200);
        t.record(false, 50);
        assert_eq!(t.total, 3);
        assert_eq!(t.succeeded, 2);
        assert_eq!(t.failed, 1);
        assert!((t.success_rate() - 2.0 / 3.0).abs() < 1e-9);
        assert!((t.avg_us() - 350.0 / 3.0).abs() < 1e-6);
    }
    #[test]
    fn test_throughput_merge() {
        let mut a = CommandElabThroughput::new();
        a.record(true, 100);
        let mut b = CommandElabThroughput::new();
        b.record(false, 200);
        a.merge(&b);
        assert_eq!(a.total, 2);
        assert_eq!(a.total_us, 300);
    }
    #[test]
    fn test_throughput_empty() {
        let t = CommandElabThroughput::new();
        assert!((t.success_rate() - 1.0).abs() < 1e-9);
        assert!((t.avg_us() - 0.0).abs() < 1e-9);
    }
    #[test]
    fn test_throughput_summary_contains_rate() {
        let mut t = CommandElabThroughput::new();
        t.record(true, 50);
        let s = t.summary();
        assert!(s.contains("100.0%"));
    }
}
