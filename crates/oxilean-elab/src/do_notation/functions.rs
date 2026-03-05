//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Name};

use super::types::{
    DoBlock, DoBlockOptimizer, DoElabConfig, DoElabError, DoElabStats, DoElem, DoNestingLevel,
    DoNotationExpander, MonadChain, MonadInstance, ParseDoAction,
};

/// Result type for do-notation elaboration.
pub type ElabResult<T> = Result<T, DoElabError>;
/// Resolve the monad instance for a given type.
///
/// Looks up the `Monad` typeclass instance for the given type constructor,
/// extracting the `bind` and `pure` functions.
///
/// For example, resolving `IO` would yield:
/// - `bind_fn = IO.bind`
/// - `pure_fn = IO.pure`
pub fn resolve_monad_instance(type_: &Expr) -> ElabResult<MonadInstance> {
    let monad_name = extract_type_name(type_).ok_or_else(|| {
        DoElabError::NoMonadInstance(format!("cannot extract monad type name from {:?}", type_))
    })?;
    let instance = lookup_standard_monad(&monad_name, type_);
    match instance {
        Some(inst) => Ok(inst),
        None => build_generic_monad_instance(&monad_name, type_),
    }
}
/// Extract the type constructor name from a type expression.
fn extract_type_name(ty: &Expr) -> Option<Name> {
    match ty {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(func, _) => extract_type_name(func),
        _ => None,
    }
}
/// Look up a standard monad instance.
fn lookup_standard_monad(name: &Name, type_: &Expr) -> Option<MonadInstance> {
    let name_str = format!("{}", name);
    match name_str.as_str() {
        "IO" => {
            let bind = Expr::Const(Name::str("IO").append_str("bind"), vec![]);
            let pure_fn = Expr::Const(Name::str("IO").append_str("pure"), vec![]);
            let map = Expr::Const(Name::str("IO").append_str("map"), vec![]);
            Some(MonadInstance::new(bind, pure_fn, type_.clone(), name.clone()).with_map(map))
        }
        "Option" => {
            let bind = Expr::Const(Name::str("Option").append_str("bind"), vec![]);
            let pure_fn = Expr::Const(Name::str("Option").append_str("pure"), vec![]);
            let map = Expr::Const(Name::str("Option").append_str("map"), vec![]);
            Some(MonadInstance::new(bind, pure_fn, type_.clone(), name.clone()).with_map(map))
        }
        "List" => {
            let bind = Expr::Const(Name::str("List").append_str("bind"), vec![]);
            let pure_fn = Expr::Const(Name::str("List").append_str("pure"), vec![]);
            let map = Expr::Const(Name::str("List").append_str("map"), vec![]);
            Some(MonadInstance::new(bind, pure_fn, type_.clone(), name.clone()).with_map(map))
        }
        "StateM" | "State" => {
            let bind = Expr::Const(Name::str("StateM").append_str("bind"), vec![]);
            let pure_fn = Expr::Const(Name::str("StateM").append_str("pure"), vec![]);
            Some(MonadInstance::new(
                bind,
                pure_fn,
                type_.clone(),
                name.clone(),
            ))
        }
        "ReaderM" | "Reader" => {
            let bind = Expr::Const(Name::str("ReaderM").append_str("bind"), vec![]);
            let pure_fn = Expr::Const(Name::str("ReaderM").append_str("pure"), vec![]);
            Some(MonadInstance::new(
                bind,
                pure_fn,
                type_.clone(),
                name.clone(),
            ))
        }
        "ExceptT" | "Except" => {
            let bind = Expr::Const(Name::str("ExceptT").append_str("bind"), vec![]);
            let pure_fn = Expr::Const(Name::str("ExceptT").append_str("pure"), vec![]);
            Some(MonadInstance::new(
                bind,
                pure_fn,
                type_.clone(),
                name.clone(),
            ))
        }
        "Id" => {
            let bind = Expr::Const(Name::str("Id").append_str("bind"), vec![]);
            let pure_fn = Expr::Const(Name::str("Id").append_str("pure"), vec![]);
            Some(MonadInstance::new(
                bind,
                pure_fn,
                type_.clone(),
                name.clone(),
            ))
        }
        _ => None,
    }
}
/// Build a generic monad instance for a non-standard type.
fn build_generic_monad_instance(name: &Name, type_: &Expr) -> ElabResult<MonadInstance> {
    let bind = Expr::Const(name.clone().append_str("bind"), vec![]);
    let pure_fn = Expr::Const(name.clone().append_str("pure"), vec![]);
    Ok(MonadInstance::new(
        bind,
        pure_fn,
        type_.clone(),
        name.clone(),
    ))
}
/// Elaborate a do-notation block into a kernel expression.
///
/// This is the main entry point for do-notation elaboration. It:
/// 1. Validates the do block structure
/// 2. Resolves the monad instance
/// 3. Desugars each element into bind/pure chains
/// 4. Returns the resulting kernel expression
#[allow(clippy::too_many_arguments)]
pub fn elaborate_do(
    block: &DoBlock,
    expected_type: Option<&Expr>,
    config: &DoElabConfig,
) -> ElabResult<Expr> {
    block.validate()?;
    let monad_type = determine_monad_type(block, expected_type)?;
    let monad_inst = resolve_monad_instance(&monad_type)?;
    elaborate_do_sequence(&block.elems, &monad_inst, config, 0)
}
/// Determine the monad type from the block or expected type.
fn determine_monad_type(block: &DoBlock, expected_type: Option<&Expr>) -> ElabResult<Expr> {
    if let Some(monad) = &block.monad_type {
        return Ok(monad.clone());
    }
    if let Some(expected) = expected_type {
        if let Some(name) = extract_type_name(expected) {
            return Ok(Expr::Const(name, vec![]));
        }
    }
    Ok(Expr::Const(Name::str("IO"), vec![]))
}
/// Elaborate a sequence of do elements.
fn elaborate_do_sequence(
    elems: &[DoElem],
    monad_inst: &MonadInstance,
    config: &DoElabConfig,
    depth: usize,
) -> ElabResult<Expr> {
    if depth > config.max_depth {
        return Err(DoElabError::MaxDepthExceeded(depth));
    }
    if elems.is_empty() {
        let unit = Expr::Const(Name::str("Unit").append_str("unit"), vec![]);
        return Ok(monad_inst.mk_pure(&unit));
    }
    if elems.len() == 1 {
        return elaborate_single_elem(&elems[0], monad_inst, config, depth);
    }
    let first = &elems[0];
    let rest = &elems[1..];
    elaborate_do_elem(first, rest, monad_inst, config, depth)
}
/// Elaborate a single do element (the last one in a sequence).
fn elaborate_single_elem(
    elem: &DoElem,
    monad_inst: &MonadInstance,
    config: &DoElabConfig,
    depth: usize,
) -> ElabResult<Expr> {
    match elem {
        DoElem::Action(expr) => {
            if config.auto_return {
                Ok(expr.clone())
            } else {
                Ok(expr.clone())
            }
        }
        DoElem::Return(expr) => Ok(monad_inst.mk_pure(expr)),
        DoElem::If { cond, then_, else_ } => {
            let then_expr = elaborate_do_sequence(then_, monad_inst, config, depth + 1)?;
            let else_expr = elaborate_do_sequence(else_, monad_inst, config, depth + 1)?;
            Ok(build_ite(cond, &then_expr, &else_expr))
        }
        DoElem::Match { scrutinee, arms } => {
            let mut elaborated_arms = Vec::new();
            for (pat_name, body_elems) in arms {
                let body = elaborate_do_sequence(body_elems, monad_inst, config, depth + 1)?;
                elaborated_arms.push((pat_name.clone(), body));
            }
            Ok(build_match(scrutinee, &elaborated_arms))
        }
        DoElem::For {
            var,
            collection,
            body,
        } => {
            let body_expr = elaborate_single_elem(body, monad_inst, config, depth + 1)?;
            Ok(desugar_for_loop(var, collection, &body_expr, monad_inst))
        }
        DoElem::TryCatch {
            body,
            catch_var,
            catch_body,
        } => {
            let body_expr = elaborate_do_sequence(body, monad_inst, config, depth + 1)?;
            let catch_expr = elaborate_do_sequence(catch_body, monad_inst, config, depth + 1)?;
            Ok(desugar_try_catch(
                &body_expr,
                catch_var,
                &catch_expr,
                monad_inst,
            ))
        }
        DoElem::Unless { cond, body } => {
            let body_expr = elaborate_do_sequence(body, monad_inst, config, depth + 1)?;
            let unit = Expr::Const(Name::str("Unit").append_str("unit"), vec![]);
            let pure_unit = monad_inst.mk_pure(&unit);
            Ok(build_ite(cond, &pure_unit, &body_expr))
        }
        DoElem::Bind { .. } | DoElem::LetBind { .. } => Err(DoElabError::BindAtEnd(format!(
            "bind/let at end of do block (no continuation): {}",
            elem
        ))),
    }
}
/// Elaborate a do element with a continuation (the rest of the block).
fn elaborate_do_elem(
    elem: &DoElem,
    rest: &[DoElem],
    monad_inst: &MonadInstance,
    config: &DoElabConfig,
    depth: usize,
) -> ElabResult<Expr> {
    let continuation = elaborate_do_sequence(rest, monad_inst, config, depth + 1)?;
    match elem {
        DoElem::Bind { pat, ty, rhs } => {
            let var_ty = ty
                .as_ref()
                .cloned()
                .unwrap_or_else(|| Expr::Const(Name::str("_"), vec![]));
            Ok(monad_inst.mk_bind(pat, &var_ty, rhs, &continuation))
        }
        DoElem::LetBind { pat, ty, val } => {
            let var_ty = ty
                .as_ref()
                .cloned()
                .unwrap_or_else(|| Expr::Const(Name::str("_"), vec![]));
            Ok(Expr::Let(
                pat.clone(),
                Box::new(var_ty),
                Box::new(val.clone()),
                Box::new(continuation),
            ))
        }
        DoElem::Action(expr) => Ok(monad_inst.mk_seq(expr, &continuation)),
        DoElem::Return(expr) => {
            let pure_e = monad_inst.mk_pure(expr);
            Ok(monad_inst.mk_seq(&pure_e, &continuation))
        }
        DoElem::If { cond, then_, else_ } => {
            let then_expr = elaborate_do_sequence(then_, monad_inst, config, depth + 1)?;
            let else_expr = elaborate_do_sequence(else_, monad_inst, config, depth + 1)?;
            let if_expr = build_ite(cond, &then_expr, &else_expr);
            Ok(monad_inst.mk_seq(&if_expr, &continuation))
        }
        DoElem::Match { scrutinee, arms } => {
            let mut elaborated_arms = Vec::new();
            for (pat_name, body_elems) in arms {
                let body = elaborate_do_sequence(body_elems, monad_inst, config, depth + 1)?;
                elaborated_arms.push((pat_name.clone(), body));
            }
            let match_expr = build_match(scrutinee, &elaborated_arms);
            Ok(monad_inst.mk_seq(&match_expr, &continuation))
        }
        DoElem::For {
            var,
            collection,
            body,
        } => {
            let body_expr = elaborate_single_elem(body, monad_inst, config, depth + 1)?;
            let for_expr = desugar_for_loop(var, collection, &body_expr, monad_inst);
            Ok(monad_inst.mk_seq(&for_expr, &continuation))
        }
        DoElem::TryCatch {
            body,
            catch_var,
            catch_body,
        } => {
            let body_expr = elaborate_do_sequence(body, monad_inst, config, depth + 1)?;
            let catch_expr = elaborate_do_sequence(catch_body, monad_inst, config, depth + 1)?;
            let try_expr = desugar_try_catch(&body_expr, catch_var, &catch_expr, monad_inst);
            Ok(monad_inst.mk_seq(&try_expr, &continuation))
        }
        DoElem::Unless { cond, body } => {
            let body_expr = elaborate_do_sequence(body, monad_inst, config, depth + 1)?;
            let unit = Expr::Const(Name::str("Unit").append_str("unit"), vec![]);
            let pure_unit = monad_inst.mk_pure(&unit);
            let unless_expr = build_ite(cond, &pure_unit, &body_expr);
            Ok(monad_inst.mk_seq(&unless_expr, &continuation))
        }
    }
}
/// Desugar a for-loop into a `forM` application.
///
/// ```text
/// for x in coll do body
/// ```
/// becomes:
/// ```text
/// forM coll (fun x => body)
/// ```
pub fn desugar_for_loop(
    var: &Name,
    collection: &Expr,
    body: &Expr,
    _monad: &MonadInstance,
) -> Expr {
    let for_m = Expr::Const(Name::str("forM"), vec![]);
    let iter_fn = Expr::Lam(
        BinderInfo::Default,
        var.clone(),
        Box::new(Expr::Const(Name::str("_"), vec![])),
        Box::new(body.clone()),
    );
    let app1 = Expr::App(Box::new(for_m), Box::new(collection.clone()));
    Expr::App(Box::new(app1), Box::new(iter_fn))
}
/// Desugar a try-catch into a `tryCatch` application.
///
/// ```text
/// try body catch e => handler
/// ```
/// becomes:
/// ```text
/// tryCatch body (fun e => handler)
/// ```
pub fn desugar_try_catch(
    body: &Expr,
    catch_var: &Name,
    handler: &Expr,
    _monad: &MonadInstance,
) -> Expr {
    let try_catch = Expr::Const(Name::str("tryCatch"), vec![]);
    let handler_fn = Expr::Lam(
        BinderInfo::Default,
        catch_var.clone(),
        Box::new(Expr::Const(Name::str("_"), vec![])),
        Box::new(handler.clone()),
    );
    let app1 = Expr::App(Box::new(try_catch), Box::new(body.clone()));
    Expr::App(Box::new(app1), Box::new(handler_fn))
}
/// Build an if-then-else expression.
fn build_ite(cond: &Expr, then_: &Expr, else_: &Expr) -> Expr {
    let ite = Expr::Const(Name::str("ite"), vec![]);
    let app1 = Expr::App(Box::new(ite), Box::new(cond.clone()));
    let app2 = Expr::App(Box::new(app1), Box::new(then_.clone()));
    Expr::App(Box::new(app2), Box::new(else_.clone()))
}
/// Build a simple match expression (as nested if-then-else or recursor application).
fn build_match(scrutinee: &Expr, arms: &[(Name, Expr)]) -> Expr {
    if arms.is_empty() {
        return Expr::Const(Name::str("absurd"), vec![]);
    }
    if arms.len() == 1 {
        let (_pat, body) = &arms[0];
        return Expr::Let(
            Name::str("_"),
            Box::new(Expr::Const(Name::str("_"), vec![])),
            Box::new(scrutinee.clone()),
            Box::new(body.clone()),
        );
    }
    let match_name = Name::str("_match");
    let mut result = arms
        .last()
        .expect("arms is non-empty (checked above)")
        .1
        .clone();
    for (pat, body) in arms.iter().rev().skip(1) {
        let eq_check = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("BEq.beq"), vec![])),
                Box::new(scrutinee.clone()),
            )),
            Box::new(Expr::Const(pat.clone(), vec![])),
        );
        result = build_ite(&eq_check, body, &result);
    }
    Expr::Let(
        match_name,
        Box::new(Expr::Const(Name::str("_"), vec![])),
        Box::new(scrutinee.clone()),
        Box::new(result),
    )
}
/// Convert a parser-level `DoAction` list into elaborator-level `DoElem` list.
///
/// This bridges the gap between the parser's representation and the
/// elaborator's richer representation.
pub fn convert_do_actions(actions: &[ParseDoAction]) -> Vec<DoElem> {
    actions.iter().map(convert_single_action).collect()
}
fn convert_single_action(action: &ParseDoAction) -> DoElem {
    match action {
        ParseDoAction::Let(name, val) => DoElem::let_bind(Name::str(name), val.clone()),
        ParseDoAction::LetTyped(name, ty, val) => {
            DoElem::let_bind_typed(Name::str(name), ty.clone(), val.clone())
        }
        ParseDoAction::Bind(name, rhs) => DoElem::bind(Name::str(name), rhs.clone()),
        ParseDoAction::Expr(expr) => DoElem::action(expr.clone()),
        ParseDoAction::Return(expr) => DoElem::return_(expr.clone()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::do_notation::*;
    use oxilean_kernel::{Expr, Literal, Name};
    fn mk_nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn mk_io() -> Expr {
        Expr::Const(Name::str("IO"), vec![])
    }
    fn mk_option() -> Expr {
        Expr::Const(Name::str("Option"), vec![])
    }
    fn mk_action(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }
    fn mk_app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    fn mk_lit_nat(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    #[test]
    fn test_do_elem_bind() {
        let elem = DoElem::bind(Name::str("x"), mk_action("getLine"));
        assert!(elem.is_binding());
        assert!(!elem.is_terminal());
        assert!(format!("{}", elem).contains("<-"));
    }
    #[test]
    fn test_do_elem_bind_typed() {
        let elem = DoElem::bind_typed(Name::str("x"), mk_nat(), mk_action("getLine"));
        assert!(elem.is_binding());
        assert!(format!("{}", elem).contains(":"));
    }
    #[test]
    fn test_do_elem_let_bind() {
        let elem = DoElem::let_bind(Name::str("y"), mk_lit_nat(42));
        assert!(elem.is_binding());
        assert!(format!("{}", elem).contains("let"));
    }
    #[test]
    fn test_do_elem_action() {
        let elem = DoElem::action(mk_action("putStrLn"));
        assert!(elem.is_terminal());
        assert!(!elem.is_binding());
    }
    #[test]
    fn test_do_elem_return() {
        let elem = DoElem::return_(mk_lit_nat(0));
        assert!(elem.is_terminal());
    }
    #[test]
    fn test_do_elem_for_loop() {
        let elem = DoElem::for_loop(
            Name::str("x"),
            mk_action("items"),
            DoElem::action(mk_action("process")),
        );
        assert!(!elem.is_terminal());
        assert!(format!("{}", elem).contains("for"));
    }
    #[test]
    fn test_do_elem_unless() {
        let elem = DoElem::unless(mk_action("cond"), vec![DoElem::action(mk_action("body"))]);
        assert!(format!("{}", elem).contains("unless"));
    }
    #[test]
    fn test_do_elem_try_catch() {
        let elem = DoElem::try_catch(
            vec![DoElem::action(mk_action("risky"))],
            Name::str("e"),
            vec![DoElem::action(mk_action("handle"))],
        );
        assert!(format!("{}", elem).contains("try"));
        assert!(format!("{}", elem).contains("catch"));
    }
    #[test]
    fn test_do_block_new() {
        let block = DoBlock::new(vec![
            DoElem::bind(Name::str("x"), mk_action("getLine")),
            DoElem::action(mk_action("putStrLn")),
        ]);
        assert!(!block.is_empty());
        assert_eq!(block.len(), 2);
    }
    #[test]
    fn test_do_block_with_monad() {
        let block = DoBlock::with_monad(mk_io(), vec![DoElem::return_(mk_lit_nat(0))]);
        assert!(block.monad_type.is_some());
    }
    #[test]
    fn test_do_block_validate_ok() {
        let block = DoBlock::new(vec![
            DoElem::bind(Name::str("x"), mk_action("getLine")),
            DoElem::return_(mk_action("x")),
        ]);
        assert!(block.validate().is_ok());
    }
    #[test]
    fn test_do_block_validate_empty() {
        let block = DoBlock::new(vec![]);
        assert!(block.validate().is_err());
    }
    #[test]
    fn test_do_block_validate_bind_at_end() {
        let block = DoBlock::new(vec![DoElem::bind(Name::str("x"), mk_action("getLine"))]);
        let result = block.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            DoElabError::BindAtEnd(_) => {}
            other => panic!("expected BindAtEnd, got: {:?}", other),
        }
    }
    #[test]
    fn test_do_block_display() {
        let block = DoBlock::new(vec![
            DoElem::bind(Name::str("x"), mk_action("getLine")),
            DoElem::return_(mk_action("x")),
        ]);
        let display = format!("{}", block);
        assert!(display.contains("do"));
    }
    #[test]
    fn test_monad_instance_new() {
        let bind = Expr::Const(Name::str("IO.bind"), vec![]);
        let pure_fn = Expr::Const(Name::str("IO.pure"), vec![]);
        let inst = MonadInstance::new(bind, pure_fn, mk_io(), Name::str("IO"));
        assert!(inst.map_fn.is_none());
        assert!(inst.seq_fn.is_none());
        assert!(format!("{}", inst).contains("IO"));
    }
    #[test]
    fn test_monad_instance_mk_bind() {
        let bind = Expr::Const(Name::str("IO.bind"), vec![]);
        let pure_fn = Expr::Const(Name::str("IO.pure"), vec![]);
        let inst = MonadInstance::new(bind, pure_fn, mk_io(), Name::str("IO"));
        let result = inst.mk_bind(
            &Name::str("x"),
            &mk_nat(),
            &mk_action("getLine"),
            &mk_action("body"),
        );
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_monad_instance_mk_pure() {
        let bind = Expr::Const(Name::str("IO.bind"), vec![]);
        let pure_fn = Expr::Const(Name::str("IO.pure"), vec![]);
        let inst = MonadInstance::new(bind, pure_fn, mk_io(), Name::str("IO"));
        let result = inst.mk_pure(&mk_lit_nat(42));
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_monad_instance_mk_seq() {
        let bind = Expr::Const(Name::str("IO.bind"), vec![]);
        let pure_fn = Expr::Const(Name::str("IO.pure"), vec![]);
        let inst = MonadInstance::new(bind, pure_fn, mk_io(), Name::str("IO"));
        let result = inst.mk_seq(&mk_action("first"), &mk_action("second"));
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_monad_instance_mk_seq_with_seq_fn() {
        let bind = Expr::Const(Name::str("IO.bind"), vec![]);
        let pure_fn = Expr::Const(Name::str("IO.pure"), vec![]);
        let seq = Expr::Const(Name::str("IO.seq"), vec![]);
        let inst = MonadInstance::new(bind, pure_fn, mk_io(), Name::str("IO")).with_seq(seq);
        let result = inst.mk_seq(&mk_action("first"), &mk_action("second"));
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_monad_instance_mk_map() {
        let bind = Expr::Const(Name::str("IO.bind"), vec![]);
        let pure_fn = Expr::Const(Name::str("IO.pure"), vec![]);
        let map = Expr::Const(Name::str("IO.map"), vec![]);
        let inst = MonadInstance::new(bind, pure_fn, mk_io(), Name::str("IO")).with_map(map);
        let f = Expr::Const(Name::str("toString"), vec![]);
        let result = inst.mk_map(&f, &mk_action("getLine"));
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_resolve_io_monad() {
        let result = resolve_monad_instance(&mk_io());
        assert!(result.is_ok());
        let inst = result.expect("test operation should succeed");
        assert_eq!(inst.monad_name, Name::str("IO"));
    }
    #[test]
    fn test_resolve_option_monad() {
        let result = resolve_monad_instance(&mk_option());
        assert!(result.is_ok());
        let inst = result.expect("test operation should succeed");
        assert_eq!(inst.monad_name, Name::str("Option"));
    }
    #[test]
    fn test_resolve_list_monad() {
        let list = Expr::Const(Name::str("List"), vec![]);
        let result = resolve_monad_instance(&list);
        assert!(result.is_ok());
    }
    #[test]
    fn test_resolve_custom_monad() {
        let custom = Expr::Const(Name::str("MyMonad"), vec![]);
        let result = resolve_monad_instance(&custom);
        assert!(result.is_ok());
        let inst = result.expect("test operation should succeed");
        assert_eq!(inst.monad_name, Name::str("MyMonad"));
    }
    #[test]
    fn test_resolve_monad_from_app() {
        let state_s = mk_app(Expr::Const(Name::str("StateM"), vec![]), mk_nat());
        let result = resolve_monad_instance(&state_s);
        assert!(result.is_ok());
    }
    #[test]
    fn test_elaborate_simple_bind() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::bind(Name::str("x"), mk_action("getLine")),
                DoElem::return_(Expr::BVar(0)),
            ],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
        let expr = result.expect("test operation should succeed");
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_let_bind() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::let_bind(Name::str("x"), mk_lit_nat(42)),
                DoElem::return_(Expr::BVar(0)),
            ],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
        let expr = result.expect("test operation should succeed");
        assert!(matches!(expr, Expr::Let(_, _, _, _)));
    }
    #[test]
    fn test_elaborate_action_seq() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::action(mk_action("putStrLn")),
                DoElem::action(mk_action("putStrLn")),
            ],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_elaborate_single_return() {
        let block = DoBlock::with_monad(mk_io(), vec![DoElem::return_(mk_lit_nat(0))]);
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
        let expr = result.expect("test operation should succeed");
        assert!(matches!(expr, Expr::App(_, _)));
    }
    #[test]
    fn test_elaborate_for_loop() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![DoElem::for_loop(
                Name::str("x"),
                mk_action("items"),
                DoElem::action(mk_action("process")),
            )],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_elaborate_try_catch() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![DoElem::try_catch(
                vec![DoElem::action(mk_action("risky"))],
                Name::str("e"),
                vec![DoElem::return_(mk_lit_nat(0))],
            )],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_elaborate_nested_do() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::bind(Name::str("x"), mk_action("getLine")),
                DoElem::bind(Name::str("y"), mk_action("getLine")),
                DoElem::let_bind(Name::str("z"), mk_lit_nat(42)),
                DoElem::return_(Expr::BVar(0)),
            ],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_elaborate_if_in_do() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![DoElem::If {
                cond: mk_action("cond"),
                then_: vec![DoElem::return_(mk_lit_nat(1))],
                else_: vec![DoElem::return_(mk_lit_nat(0))],
            }],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_elaborate_unless() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![DoElem::unless(
                mk_action("cond"),
                vec![DoElem::action(mk_action("body"))],
            )],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_elaborate_empty_block() {
        let block = DoBlock::new(vec![]);
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_err());
    }
    #[test]
    fn test_elaborate_bind_at_end() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![DoElem::bind(Name::str("x"), mk_action("getLine"))],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_err());
    }
    #[test]
    fn test_desugar_for_loop() {
        let bind = Expr::Const(Name::str("IO.bind"), vec![]);
        let pure_fn = Expr::Const(Name::str("IO.pure"), vec![]);
        let inst = MonadInstance::new(bind, pure_fn, mk_io(), Name::str("IO"));
        let result = desugar_for_loop(
            &Name::str("x"),
            &mk_action("items"),
            &mk_action("process"),
            &inst,
        );
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_desugar_try_catch() {
        let bind = Expr::Const(Name::str("IO.bind"), vec![]);
        let pure_fn = Expr::Const(Name::str("IO.pure"), vec![]);
        let inst = MonadInstance::new(bind, pure_fn, mk_io(), Name::str("IO"));
        let result = desugar_try_catch(
            &mk_action("risky"),
            &Name::str("e"),
            &mk_action("handle"),
            &inst,
        );
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_config_defaults() {
        let config = DoElabConfig::new();
        assert!(config.lift_types);
        assert!(config.auto_return);
        assert!(config.strict_bind);
        assert!(config.allow_for);
        assert!(config.allow_try_catch);
        assert_eq!(config.max_depth, 100);
    }
    #[test]
    fn test_config_builder() {
        let config = DoElabConfig::new()
            .without_auto_return()
            .without_lift()
            .with_strict(false);
        assert!(!config.auto_return);
        assert!(!config.lift_types);
        assert!(!config.strict_bind);
    }
    #[test]
    fn test_error_display() {
        let err = DoElabError::EmptyDoBlock;
        assert!(format!("{}", err).contains("empty"));
        let err = DoElabError::NoMonadInstance("Foo".to_string());
        assert!(format!("{}", err).contains("Foo"));
        let err = DoElabError::MaxDepthExceeded(100);
        assert!(format!("{}", err).contains("100"));
    }
    #[test]
    fn test_expander_new() {
        let expander = DoNotationExpander::with_defaults();
        assert_eq!(expander.current_depth(), 0);
    }
    #[test]
    fn test_expander_fresh_name() {
        let mut expander = DoNotationExpander::with_defaults();
        let n1 = expander.fresh_name("x");
        let n2 = expander.fresh_name("x");
        assert_ne!(n1, n2);
    }
    #[test]
    fn test_expander_elaborate() {
        let mut expander = DoNotationExpander::with_defaults();
        let block = DoBlock::with_monad(mk_io(), vec![DoElem::return_(mk_lit_nat(42))]);
        let result = expander.elaborate(&block, None);
        assert!(result.is_ok());
    }
    #[test]
    fn test_expander_cache() {
        let mut expander = DoNotationExpander::with_defaults();
        let bind = Expr::Const(Name::str("IO.bind"), vec![]);
        let pure_fn = Expr::Const(Name::str("IO.pure"), vec![]);
        let inst = MonadInstance::new(bind, pure_fn, mk_io(), Name::str("IO"));
        expander.cache_monad(Name::str("IO"), inst);
        assert!(expander.lookup_monad(&Name::str("IO")).is_some());
        assert!(expander.lookup_monad(&Name::str("Unknown")).is_none());
    }
    #[test]
    fn test_expander_reset() {
        let mut expander = DoNotationExpander::with_defaults();
        expander.fresh_name("x");
        expander.fresh_name("y");
        expander.reset();
        assert_eq!(expander.current_depth(), 0);
    }
    #[test]
    fn test_convert_do_actions() {
        let actions = vec![
            ParseDoAction::Bind("x".to_string(), mk_action("getLine")),
            ParseDoAction::Let("y".to_string(), mk_lit_nat(42)),
            ParseDoAction::Return(mk_lit_nat(0)),
        ];
        let elems = convert_do_actions(&actions);
        assert_eq!(elems.len(), 3);
        assert!(elems[0].is_binding());
        assert!(elems[1].is_binding());
        assert!(elems[2].is_terminal());
    }
    #[test]
    fn test_convert_let_typed() {
        let actions = vec![ParseDoAction::LetTyped(
            "x".to_string(),
            mk_nat(),
            mk_lit_nat(42),
        )];
        let elems = convert_do_actions(&actions);
        assert_eq!(elems.len(), 1);
        match &elems[0] {
            DoElem::LetBind { ty, .. } => assert!(ty.is_some()),
            _ => panic!("expected LetBind"),
        }
    }
    #[test]
    fn test_convert_expr_action() {
        let actions = vec![ParseDoAction::Expr(mk_action("putStrLn"))];
        let elems = convert_do_actions(&actions);
        assert_eq!(elems.len(), 1);
        assert!(matches!(elems[0], DoElem::Action(_)));
    }
    #[test]
    fn test_elaborate_match_in_do() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![DoElem::Match {
                scrutinee: mk_action("val"),
                arms: vec![
                    (Name::str("some"), vec![DoElem::return_(mk_lit_nat(1))]),
                    (Name::str("none"), vec![DoElem::return_(mk_lit_nat(0))]),
                ],
            }],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_single_action_block() {
        let block = DoBlock::with_monad(mk_io(), vec![DoElem::action(mk_action("hello"))]);
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_multiple_binds() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::bind(Name::str("a"), mk_action("getLine")),
                DoElem::bind(Name::str("b"), mk_action("getLine")),
                DoElem::bind(Name::str("c"), mk_action("getLine")),
                DoElem::return_(Expr::BVar(0)),
            ],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_mixed_binds_and_lets() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::bind(Name::str("x"), mk_action("getLine")),
                DoElem::let_bind(Name::str("y"), mk_lit_nat(42)),
                DoElem::action(mk_app(mk_action("putStrLn"), Expr::BVar(1))),
                DoElem::return_(Expr::BVar(0)),
            ],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_for_with_continuation() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::for_loop(
                    Name::str("x"),
                    mk_action("items"),
                    DoElem::action(mk_action("process")),
                ),
                DoElem::return_(mk_lit_nat(0)),
            ],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_try_catch_with_continuation() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::try_catch(
                    vec![DoElem::action(mk_action("risky"))],
                    Name::str("e"),
                    vec![DoElem::return_(mk_lit_nat(0))],
                ),
                DoElem::return_(mk_lit_nat(1)),
            ],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
    #[test]
    fn test_option_monad_do() {
        let block = DoBlock::with_monad(
            mk_option(),
            vec![
                DoElem::bind(Name::str("x"), mk_action("lookup")),
                DoElem::return_(Expr::BVar(0)),
            ],
        );
        let config = DoElabConfig::new();
        let result = elaborate_do(&block, None, &config);
        assert!(result.is_ok());
    }
}
/// Counts the number of bind elements in a `DoBlock`.
#[allow(dead_code)]
pub fn count_binds(block: &DoBlock) -> usize {
    block
        .elems
        .iter()
        .filter(|e| matches!(e, DoElem::Bind { .. }))
        .count()
}
/// Counts the number of let-binding elements in a `DoBlock`.
#[allow(dead_code)]
pub fn count_lets(block: &DoBlock) -> usize {
    block
        .elems
        .iter()
        .filter(|e| matches!(e, DoElem::LetBind { .. }))
        .count()
}
/// Counts the number of pure action elements in a `DoBlock`.
#[allow(dead_code)]
pub fn count_actions(block: &DoBlock) -> usize {
    block
        .elems
        .iter()
        .filter(|e| matches!(e, DoElem::Action(_)))
        .count()
}
/// Collect all names bound by `<-` in a `DoBlock`.
#[allow(dead_code)]
pub fn bound_names(block: &DoBlock) -> Vec<Name> {
    block
        .elems
        .iter()
        .filter_map(|e| {
            if let DoElem::Bind { pat, .. } = e {
                Some(pat.clone())
            } else {
                None
            }
        })
        .collect()
}
/// Return `true` if a `DoBlock` has exactly one element that is a `Return`.
#[allow(dead_code)]
pub fn is_pure_return(block: &DoBlock) -> bool {
    block.elems.len() == 1 && matches!(block.elems[0], DoElem::Return(_))
}
/// Count the maximum nesting depth of control-flow inside a `DoBlock`.
#[allow(dead_code)]
pub fn max_do_nesting(block: &DoBlock) -> usize {
    block
        .elems
        .iter()
        .map(elem_nesting_depth)
        .max()
        .unwrap_or(0)
}
fn elem_nesting_depth(elem: &DoElem) -> usize {
    match elem {
        DoElem::Bind { .. } | DoElem::LetBind { .. } | DoElem::Action(_) | DoElem::Return(_) => 0,
        DoElem::For { body, .. } => 1 + elem_nesting_depth(body),
        DoElem::Unless { body, .. } | DoElem::TryCatch { body, .. } => {
            1 + body.iter().map(elem_nesting_depth).max().unwrap_or(0)
        }
        DoElem::If { then_, else_, .. } => {
            let t = 1 + then_.iter().map(elem_nesting_depth).max().unwrap_or(0);
            let e = 1 + else_.iter().map(elem_nesting_depth).max().unwrap_or(0);
            t.max(e)
        }
        DoElem::Match { arms, .. } => {
            1 + arms
                .iter()
                .flat_map(|(_, body)| body.iter())
                .map(elem_nesting_depth)
                .max()
                .unwrap_or(0)
        }
    }
}
#[cfg(test)]
mod do_notation_ext_tests {
    use super::*;
    use crate::do_notation::*;
    fn mk_io() -> Expr {
        Expr::Const(Name::str("IO"), vec![])
    }
    fn mk_lit(n: u64) -> Expr {
        Expr::Lit(oxilean_kernel::Literal::Nat(n))
    }
    fn mk_action(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_monad_chain_build_empty() {
        let chain = MonadChain::new(mk_io());
        assert!(chain.is_empty());
        let block = chain.build();
        assert!(block.is_empty());
    }
    #[test]
    fn test_monad_chain_bind_action() {
        let chain = MonadChain::new(mk_io())
            .bind(Name::str("x"), mk_action("readLine"))
            .action(mk_action("print"))
            .return_(mk_lit(0));
        assert_eq!(chain.len(), 3);
        let block = chain.build();
        assert_eq!(block.elems.len(), 3);
    }
    #[test]
    fn test_monad_chain_let_bind() {
        let chain = MonadChain::new(mk_io())
            .let_bind(Name::str("n"), mk_lit(42))
            .return_(mk_lit(0));
        assert_eq!(chain.len(), 2);
    }
    #[test]
    fn test_monad_chain_elaborate_ok() {
        let chain = MonadChain::new(mk_io())
            .bind(Name::str("x"), mk_action("getLine"))
            .return_(mk_lit(0));
        let cfg = DoElabConfig::new();
        let result = chain.elaborate(&cfg);
        assert!(result.is_ok());
    }
    #[test]
    fn test_monad_chain_monad() {
        let chain = MonadChain::new(mk_io());
        assert_eq!(chain.monad(), &mk_io());
    }
    #[test]
    fn test_count_binds() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::bind(Name::str("x"), mk_action("a")),
                DoElem::bind(Name::str("y"), mk_action("b")),
                DoElem::action(mk_action("c")),
            ],
        );
        assert_eq!(count_binds(&block), 2);
    }
    #[test]
    fn test_count_lets() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::let_bind(Name::str("x"), mk_lit(1)),
                DoElem::bind(Name::str("y"), mk_action("a")),
            ],
        );
        assert_eq!(count_lets(&block), 1);
    }
    #[test]
    fn test_count_actions() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::action(mk_action("a")),
                DoElem::action(mk_action("b")),
                DoElem::return_(mk_lit(0)),
            ],
        );
        assert_eq!(count_actions(&block), 2);
    }
    #[test]
    fn test_bound_names() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::bind(Name::str("x"), mk_action("a")),
                DoElem::bind(Name::str("y"), mk_action("b")),
                DoElem::action(mk_action("c")),
            ],
        );
        let names = bound_names(&block);
        assert_eq!(names, vec![Name::str("x"), Name::str("y")]);
    }
    #[test]
    fn test_is_pure_return() {
        let block = DoBlock::with_monad(mk_io(), vec![DoElem::return_(mk_lit(1))]);
        assert!(is_pure_return(&block));
        let block2 = DoBlock::with_monad(
            mk_io(),
            vec![DoElem::action(mk_action("a")), DoElem::return_(mk_lit(1))],
        );
        assert!(!is_pure_return(&block2));
    }
    #[test]
    fn test_optimizer_inline_pure_bind() {
        let pure_expr = Expr::App(
            Box::new(Expr::Const(Name::str("pure"), vec![])),
            Box::new(mk_lit(42)),
        );
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::bind(Name::str("x"), pure_expr),
                DoElem::return_(mk_lit(0)),
            ],
        );
        let opt = DoBlockOptimizer::all();
        let result = opt.optimize(block);
        assert_eq!(count_lets(&result), 1);
        assert_eq!(count_binds(&result), 0);
    }
    #[test]
    fn test_optimizer_no_change_without_pure() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![
                DoElem::bind(Name::str("x"), mk_action("getLine")),
                DoElem::return_(mk_lit(0)),
            ],
        );
        let opt = DoBlockOptimizer::all();
        let result = opt.optimize(block);
        assert_eq!(count_binds(&result), 1);
    }
    #[test]
    fn test_do_nesting_level_enter_leave() {
        let mut level = DoNestingLevel::new();
        assert!(!level.is_nested());
        level.enter(mk_io());
        assert_eq!(level.depth(), 1);
        assert!(level.is_nested());
        level.leave();
        assert!(!level.is_nested());
    }
    #[test]
    fn test_do_nesting_level_monad_stack() {
        let mut level = DoNestingLevel::new();
        level.enter(mk_io());
        assert_eq!(level.current_monad(), Some(&mk_io()));
        level.leave();
        assert_eq!(level.current_monad(), None);
    }
    #[test]
    fn test_do_elab_stats_record() {
        let mut s = DoElabStats::new();
        s.record_block(3, 1, true);
        s.record_block(0, 0, false);
        assert_eq!(s.blocks_elaborated, 2);
        assert_eq!(s.binds_desugared, 3);
        assert_eq!(s.failures, 1);
        assert!((s.success_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_do_elab_stats_merge() {
        let mut a = DoElabStats::new();
        a.record_block(2, 1, true);
        let mut b = DoElabStats::new();
        b.record_block(5, 2, true);
        a.merge(&b);
        assert_eq!(a.binds_desugared, 7);
    }
    #[test]
    fn test_do_elab_stats_for_try() {
        let mut s = DoElabStats::new();
        s.record_for_loop();
        s.record_for_loop();
        s.record_try_catch();
        assert_eq!(s.for_loops_desugared, 2);
        assert_eq!(s.try_catch_desugared, 1);
    }
    #[test]
    fn test_do_elab_stats_summary() {
        let mut s = DoElabStats::new();
        s.record_block(3, 1, true);
        let summary = s.summary();
        assert!(summary.contains("blocks=1"));
        assert!(summary.contains("binds=3"));
    }
    #[test]
    fn test_max_do_nesting_flat() {
        let block = DoBlock::with_monad(mk_io(), vec![DoElem::return_(mk_lit(0))]);
        assert_eq!(max_do_nesting(&block), 0);
    }
    #[test]
    fn test_max_do_nesting_for_loop() {
        let block = DoBlock::with_monad(
            mk_io(),
            vec![DoElem::for_loop(
                Name::str("x"),
                mk_lit(0),
                DoElem::return_(mk_lit(0)),
            )],
        );
        assert_eq!(max_do_nesting(&block), 1);
    }
}
