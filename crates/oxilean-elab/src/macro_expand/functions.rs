//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Level, Literal, Name};
use std::collections::HashMap;

use super::types::{
    HygieneMode, IdentityStep, MacroAst, MacroDef, MacroEnvironment, MacroError, MacroExpander,
    MacroExpansionConfig, MacroExpansionReport, MacroExpansionResult, MacroExpansionStats,
    MacroExtensionMarker, MacroHygieneMap, MacroInterpreter, MacroKind, MacroNamespace,
    MacroPattern, MacroPatternMatcher, MacroPipeline, MacroRegistry, MacroRule, MacroScopeStack,
    MacroTemplate, MacroTraceEntry, MacroTracer, MacroValidationError, MacroValidator,
};

/// Recursively match a pattern against an expression, accumulating bindings.
pub fn match_pattern_impl(
    pattern: &MacroPattern,
    expr: &Expr,
    bindings: &mut HashMap<Name, Expr>,
) -> bool {
    match pattern {
        MacroPattern::Var(name) => {
            if let Some(existing) = bindings.get(name) {
                *existing == *expr
            } else {
                bindings.insert(name.clone(), expr.clone());
                true
            }
        }
        MacroPattern::Exact(name) => matches!(expr, Expr::Const(n, _) if n == name),
        MacroPattern::App(f_pat, a_pat) => {
            if let Expr::App(f, a) = expr {
                match_pattern_impl(f_pat, f, bindings) && match_pattern_impl(a_pat, a, bindings)
            } else {
                false
            }
        }
        MacroPattern::Lit(s) => matches!(expr, Expr::Lit(Literal::Str(t)) if t == s),
        MacroPattern::Seq(pats) => {
            if pats.is_empty() {
                return true;
            }
            if pats.len() == 1 {
                return match_pattern_impl(&pats[0], expr, bindings);
            }
            let mut spine = Vec::new();
            flatten_app_spine(expr, &mut spine);
            if spine.len() != pats.len() {
                return false;
            }
            for (pat, e) in pats.iter().zip(spine.iter()) {
                if !match_pattern_impl(pat, e, bindings) {
                    return false;
                }
            }
            true
        }
        MacroPattern::Optional(inner) => {
            let _ = match_pattern_impl(inner, expr, bindings);
            true
        }
        MacroPattern::Many(inner) => {
            let _ = match_pattern_impl(inner, expr, bindings);
            true
        }
    }
}
/// Flatten an application spine: `((f a) b)` → `[f, a, b]`.
fn flatten_app_spine<'a>(expr: &'a Expr, out: &mut Vec<&'a Expr>) {
    match expr {
        Expr::App(f, a) => {
            flatten_app_spine(f, out);
            out.push(a);
        }
        _ => {
            out.push(expr);
        }
    }
}
/// Substitute bindings into a template to produce an expression.
pub fn substitute_template_impl(
    template: &MacroTemplate,
    bindings: &HashMap<Name, Expr>,
) -> Result<Expr, MacroError> {
    match template {
        MacroTemplate::Expr(e) => Ok(e.clone()),
        MacroTemplate::Var(name) => bindings.get(name).cloned().ok_or_else(|| {
            MacroError::ExpansionError(format!("Unbound template variable: {}", name))
        }),
        MacroTemplate::App(f, a) => {
            let f_expr = substitute_template_impl(f, bindings)?;
            let a_expr = substitute_template_impl(a, bindings)?;
            Ok(Expr::App(Box::new(f_expr), Box::new(a_expr)))
        }
        MacroTemplate::Seq(parts) => {
            if parts.is_empty() {
                return Err(MacroError::ExpansionError(
                    "Empty template sequence".to_string(),
                ));
            }
            let mut result = substitute_template_impl(&parts[0], bindings)?;
            for part in &parts[1..] {
                let arg = substitute_template_impl(part, bindings)?;
                result = Expr::App(Box::new(result), Box::new(arg));
            }
            Ok(result)
        }
        MacroTemplate::Splice(name) => bindings.get(name).cloned().ok_or_else(|| {
            MacroError::ExpansionError(format!("Unbound splice variable: {}", name))
        }),
        MacroTemplate::Quote(inner) => {
            let inner_expr = substitute_template_impl(inner, bindings)?;
            Ok(quote_expr(&inner_expr))
        }
    }
}
/// Substitute all occurrences of a named constant with a replacement expression.
pub fn substitute_name_in_expr(expr: &Expr, name: &Name, replacement: &Expr) -> Expr {
    match expr {
        Expr::Const(n, _) if n == name => replacement.clone(),
        Expr::App(f, a) => {
            let f2 = substitute_name_in_expr(f, name, replacement);
            let a2 = substitute_name_in_expr(a, name, replacement);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = substitute_name_in_expr(ty, name, replacement);
            let body2 = substitute_name_in_expr(body, name, replacement);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = substitute_name_in_expr(ty, name, replacement);
            let body2 = substitute_name_in_expr(body, name, replacement);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(n, ty, val, body) => {
            let ty2 = substitute_name_in_expr(ty, name, replacement);
            let val2 = substitute_name_in_expr(val, name, replacement);
            let body2 = substitute_name_in_expr(body, name, replacement);
            Expr::Let(n.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        _ => expr.clone(),
    }
}
/// Rename a name for hygienic expansion, appending a scope ID.
pub fn hygiene_rename(name: &Name, scope_id: u64) -> Name {
    match name {
        Name::Anonymous => Name::Anonymous,
        Name::Str(parent, s) => {
            if s.starts_with('_') {
                name.clone()
            } else {
                Name::Str(parent.clone(), format!("{}_hyg{}", s, scope_id))
            }
        }
        Name::Num(parent, n) => Name::Num(parent.clone(), *n),
    }
}
/// Quote an expression: produce a representation of the expression as data.
///
/// In a real system, this would produce a `Syntax` term. Here we produce
/// a simplified encoding using `Expr.const`, `Expr.app`, etc.
#[allow(dead_code)]
pub fn quote_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::Const(name, _) => {
            let name_lit = Expr::Lit(Literal::Str(name.to_string()));
            Expr::App(
                Box::new(Expr::Const(Name::str("Expr.const"), vec![])),
                Box::new(name_lit),
            )
        }
        Expr::App(f, a) => {
            let qf = quote_expr(f);
            let qa = quote_expr(a);
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Expr.app"), vec![])),
                    Box::new(qf),
                )),
                Box::new(qa),
            )
        }
        Expr::Lit(lit) => Expr::App(
            Box::new(Expr::Const(Name::str("Expr.lit"), vec![])),
            Box::new(Expr::Lit(lit.clone())),
        ),
        Expr::BVar(n) => Expr::App(
            Box::new(Expr::Const(Name::str("Expr.bvar"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(*n as u64))),
        ),
        Expr::FVar(fid) => Expr::App(
            Box::new(Expr::Const(Name::str("Expr.fvar"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(fid.0))),
        ),
        Expr::Sort(level) => {
            let level_repr = match level {
                Level::Zero => Expr::Lit(Literal::Nat(0)),
                _ => Expr::Lit(Literal::Str(format!("{}", level))),
            };
            Expr::App(
                Box::new(Expr::Const(Name::str("Expr.sort"), vec![])),
                Box::new(level_repr),
            )
        }
        Expr::Lam(_, name, ty, body) => {
            let qname = Expr::Lit(Literal::Str(name.to_string()));
            let qty = quote_expr(ty);
            let qbody = quote_expr(body);
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Expr.lam"), vec![])),
                        Box::new(qname),
                    )),
                    Box::new(qty),
                )),
                Box::new(qbody),
            )
        }
        Expr::Pi(_, name, ty, body) => {
            let qname = Expr::Lit(Literal::Str(name.to_string()));
            let qty = quote_expr(ty);
            let qbody = quote_expr(body);
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Expr.pi"), vec![])),
                        Box::new(qname),
                    )),
                    Box::new(qty),
                )),
                Box::new(qbody),
            )
        }
        Expr::Let(name, ty, val, body) => {
            let qname = Expr::Lit(Literal::Str(name.to_string()));
            let qty = quote_expr(ty);
            let qval = quote_expr(val);
            let qbody = quote_expr(body);
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Expr.letE"), vec![])),
                            Box::new(qname),
                        )),
                        Box::new(qty),
                    )),
                    Box::new(qval),
                )),
                Box::new(qbody),
            )
        }
        Expr::Proj(name, idx, e) => {
            let qname = Expr::Lit(Literal::Str(name.to_string()));
            let qidx = Expr::Lit(Literal::Nat(*idx as u64));
            let qe = quote_expr(e);
            Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Expr.proj"), vec![])),
                        Box::new(qname),
                    )),
                    Box::new(qidx),
                )),
                Box::new(qe),
            )
        }
    }
}
/// Unquote an expression: evaluate a quoted representation back to an expression.
///
/// This is a simplified version: it handles the encoding produced by `quote_expr`.
#[allow(dead_code)]
pub fn unquote_expr(quoted: &Expr) -> Result<Expr, MacroError> {
    match quoted {
        Expr::App(head, arg) => match head.as_ref() {
            Expr::Const(n, _) if n == &Name::str("Expr.const") => {
                if let Expr::Lit(Literal::Str(s)) = arg.as_ref() {
                    Ok(Expr::Const(Name::str(s.as_str()), vec![]))
                } else {
                    Err(MacroError::ExpansionError(
                        "Invalid Expr.const argument".to_string(),
                    ))
                }
            }
            Expr::Const(n, _) if n == &Name::str("Expr.lit") => {
                if let Expr::Lit(lit) = arg.as_ref() {
                    Ok(Expr::Lit(lit.clone()))
                } else {
                    Err(MacroError::ExpansionError(
                        "Invalid Expr.lit argument".to_string(),
                    ))
                }
            }
            Expr::Const(n, _) if n == &Name::str("Expr.bvar") => {
                if let Expr::Lit(Literal::Nat(idx)) = arg.as_ref() {
                    Ok(Expr::BVar(*idx as u32))
                } else {
                    Err(MacroError::ExpansionError(
                        "Invalid Expr.bvar argument".to_string(),
                    ))
                }
            }
            Expr::App(inner_head, inner_arg) => {
                if let Expr::Const(n, _) = inner_head.as_ref() {
                    if n == &Name::str("Expr.app") {
                        let f = unquote_expr(inner_arg)?;
                        let a = unquote_expr(arg)?;
                        return Ok(Expr::App(Box::new(f), Box::new(a)));
                    }
                }
                Err(MacroError::ExpansionError(
                    "Cannot unquote complex expression".to_string(),
                ))
            }
            _ => Err(MacroError::ExpansionError(
                "Cannot unquote expression".to_string(),
            )),
        },
        _ => Err(MacroError::ExpansionError(format!(
            "Cannot unquote non-application: {:?}",
            quoted
        ))),
    }
}
/// Create a quotation template.
#[allow(dead_code)]
pub fn mk_quotation(template: MacroTemplate) -> MacroTemplate {
    MacroTemplate::Quote(Box::new(template))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::macro_expand::*;
    use oxilean_kernel::BinderInfo;
    #[test]
    fn test_expander_create() {
        let expander = MacroExpander::new();
        assert_eq!(expander.all_macros().len(), 0);
    }
    #[test]
    fn test_register_macro() {
        let mut expander = MacroExpander::new();
        let macro_def = MacroDef::simple(Name::str("test"), vec![], Expr::Lit(Literal::Nat(42)));
        expander.register(macro_def);
        assert!(expander.is_macro(&Name::str("test")));
    }
    #[test]
    fn test_expand_no_macro() {
        let expander = MacroExpander::new();
        let expr = Expr::Lit(Literal::Nat(42));
        let expanded = expander.expand(&expr);
        assert!(expanded.is_ok());
        assert_eq!(expanded.expect("macro expansion should succeed"), expr);
    }
    #[test]
    fn test_expand_simple() {
        let mut expander = MacroExpander::new();
        let template = Expr::Lit(Literal::Nat(42));
        let macro_def = MacroDef {
            name: Name::str("answer"),
            params: vec![],
            template: template.clone(),
            kind: MacroKind::TermMacro,
            doc: None,
            rules: Vec::new(),
            scope: None,
            hygiene: HygieneMode::Hygienic,
        };
        expander.register(macro_def);
        let expr = Expr::Const(Name::str("answer"), vec![]);
        let expanded = expander.expand(&expr);
        assert!(expanded.is_ok());
        assert_eq!(expanded.expect("macro expansion should succeed"), template);
    }
    #[test]
    fn test_depth_exceeded() {
        let mut expander = MacroExpander::new();
        expander.set_max_depth(5);
        let macro_def = MacroDef::simple(
            Name::str("loop"),
            vec![],
            Expr::Const(Name::str("loop"), vec![]),
        );
        expander.register(macro_def);
        let expr = Expr::Const(Name::str("loop"), vec![]);
        let result = expander.expand_checked(&expr);
        assert!(matches!(result, Err(MacroError::DepthExceeded)));
    }
    #[test]
    fn test_match_var_pattern() {
        let expander = MacroExpander::new();
        let pattern = MacroPattern::Var(Name::str("x"));
        let expr = Expr::Lit(Literal::Nat(42));
        let bindings = expander.match_pattern(&pattern, &expr);
        assert!(bindings.is_some());
        let b = bindings.expect("test operation should succeed");
        assert_eq!(b.get(&Name::str("x")), Some(&Expr::Lit(Literal::Nat(42))));
    }
    #[test]
    fn test_match_exact_pattern() {
        let expander = MacroExpander::new();
        let pattern = MacroPattern::Exact(Name::str("Nat.zero"));
        let expr = Expr::Const(Name::str("Nat.zero"), vec![]);
        assert!(expander.match_pattern(&pattern, &expr).is_some());
        let other = Expr::Const(Name::str("Nat.succ"), vec![]);
        assert!(expander.match_pattern(&pattern, &other).is_none());
    }
    #[test]
    fn test_match_app_pattern() {
        let expander = MacroExpander::new();
        let pattern = MacroPattern::App(
            Box::new(MacroPattern::Exact(Name::str("f"))),
            Box::new(MacroPattern::Var(Name::str("arg"))),
        );
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(10))),
        );
        let bindings = expander.match_pattern(&pattern, &expr);
        assert!(bindings.is_some());
        let b = bindings.expect("test operation should succeed");
        assert_eq!(b.get(&Name::str("arg")), Some(&Expr::Lit(Literal::Nat(10))));
    }
    #[test]
    fn test_match_lit_pattern() {
        let expander = MacroExpander::new();
        let pattern = MacroPattern::Lit("hello".to_string());
        let expr = Expr::Lit(Literal::Str("hello".to_string()));
        assert!(expander.match_pattern(&pattern, &expr).is_some());
        let other = Expr::Lit(Literal::Str("world".to_string()));
        assert!(expander.match_pattern(&pattern, &other).is_none());
    }
    #[test]
    fn test_match_var_consistency() {
        let expander = MacroExpander::new();
        let pattern = MacroPattern::App(
            Box::new(MacroPattern::Var(Name::str("x"))),
            Box::new(MacroPattern::Var(Name::str("x"))),
        );
        let same = Expr::App(
            Box::new(Expr::Lit(Literal::Nat(1))),
            Box::new(Expr::Lit(Literal::Nat(1))),
        );
        assert!(expander.match_pattern(&pattern, &same).is_some());
        let diff = Expr::App(
            Box::new(Expr::Lit(Literal::Nat(1))),
            Box::new(Expr::Lit(Literal::Nat(2))),
        );
        assert!(expander.match_pattern(&pattern, &diff).is_none());
    }
    #[test]
    fn test_substitute_expr_template() {
        let expander = MacroExpander::new();
        let template = MacroTemplate::Expr(Expr::Lit(Literal::Nat(99)));
        let bindings = HashMap::new();
        let result = expander.substitute_template(&template, &bindings);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed"),
            Expr::Lit(Literal::Nat(99))
        );
    }
    #[test]
    fn test_substitute_var_template() {
        let expander = MacroExpander::new();
        let template = MacroTemplate::Var(Name::str("x"));
        let mut bindings = HashMap::new();
        bindings.insert(Name::str("x"), Expr::Lit(Literal::Nat(7)));
        let result = expander.substitute_template(&template, &bindings);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed"),
            Expr::Lit(Literal::Nat(7))
        );
    }
    #[test]
    fn test_substitute_unbound_var() {
        let expander = MacroExpander::new();
        let template = MacroTemplate::Var(Name::str("missing"));
        let bindings = HashMap::new();
        let result = expander.substitute_template(&template, &bindings);
        assert!(result.is_err());
    }
    #[test]
    fn test_substitute_app_template() {
        let expander = MacroExpander::new();
        let template = MacroTemplate::App(
            Box::new(MacroTemplate::Expr(Expr::Const(Name::str("f"), vec![]))),
            Box::new(MacroTemplate::Var(Name::str("x"))),
        );
        let mut bindings = HashMap::new();
        bindings.insert(Name::str("x"), Expr::Lit(Literal::Nat(5)));
        let result = expander.substitute_template(&template, &bindings);
        assert!(result.is_ok());
        let expanded = result.expect("macro expansion should succeed");
        assert!(matches!(expanded, Expr::App(..)));
    }
    #[test]
    fn test_hygiene_rename() {
        let name = Name::str("x");
        let renamed = hygiene_rename(&name, 42);
        assert_eq!(renamed, Name::str("x_hyg42"));
    }
    #[test]
    fn test_hygiene_preserve_underscore() {
        let name = Name::str("_tmp");
        let renamed = hygiene_rename(&name, 42);
        assert_eq!(renamed, Name::str("_tmp"));
    }
    #[test]
    fn test_apply_hygiene_lambda() {
        let mut expander = MacroExpander::new();
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let result = expander.apply_hygiene(&lam, 1);
        match &result {
            Expr::Lam(_, name, _, _) => {
                assert_eq!(*name, Name::str("x_hyg1"));
            }
            _ => panic!("Expected Lam"),
        }
    }
    #[test]
    fn test_expand_with_args_positional() {
        let mut expander = MacroExpander::new();
        let macro_def = MacroDef::simple(
            Name::str("id"),
            vec![Name::str("x")],
            Expr::Const(Name::str("x"), vec![]),
        );
        expander.register(macro_def);
        let result = expander.expand_with_args(&Name::str("id"), &[Expr::Lit(Literal::Nat(42))]);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed"),
            Expr::Lit(Literal::Nat(42))
        );
    }
    #[test]
    fn test_expand_with_args_undefined() {
        let expander = MacroExpander::new();
        let result = expander.expand_with_args(&Name::str("nonexistent"), &[]);
        assert!(matches!(result, Err(MacroError::UndefinedMacro(_))));
    }
    #[test]
    fn test_is_terminal_no_macros() {
        let expander = MacroExpander::new();
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(1))),
        );
        assert!(expander.is_terminal(&expr));
    }
    #[test]
    fn test_is_terminal_with_macro() {
        let mut expander = MacroExpander::new();
        expander.register(MacroDef::simple(
            Name::str("m"),
            vec![],
            Expr::Lit(Literal::Nat(0)),
        ));
        let expr = Expr::Const(Name::str("m"), vec![]);
        assert!(!expander.is_terminal(&expr));
    }
    #[test]
    fn test_trace_expansion() {
        let mut expander = MacroExpander::new();
        expander.register(MacroDef::simple(
            Name::str("step1"),
            vec![],
            Expr::Const(Name::str("step2"), vec![]),
        ));
        expander.register(MacroDef::simple(
            Name::str("step2"),
            vec![],
            Expr::Lit(Literal::Nat(42)),
        ));
        let expr = Expr::Const(Name::str("step1"), vec![]);
        let trace = expander.trace_expansion(&expr);
        assert!(trace.len() >= 2);
        assert_eq!(trace[0], Expr::Const(Name::str("step1"), vec![]));
    }
    #[test]
    fn test_expand_fully() {
        let mut expander = MacroExpander::new();
        expander.register(MacroDef::simple(
            Name::str("a"),
            vec![],
            Expr::Const(Name::str("b"), vec![]),
        ));
        expander.register(MacroDef::simple(
            Name::str("b"),
            vec![],
            Expr::Lit(Literal::Nat(99)),
        ));
        let expr = Expr::Const(Name::str("a"), vec![]);
        let result = expander.expand_fully(&expr);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed"),
            Expr::Lit(Literal::Nat(99))
        );
    }
    #[test]
    fn test_quote_const() {
        let expr = Expr::Const(Name::str("Nat.zero"), vec![]);
        let quoted = quote_expr(&expr);
        match &quoted {
            Expr::App(f, a) => {
                assert!(matches!(f.as_ref(), Expr::Const(n, _) if n == &
                    Name::str("Expr.const")));
                assert_eq!(*a.as_ref(), Expr::Lit(Literal::Str("Nat.zero".to_string())));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_quote_lit() {
        let expr = Expr::Lit(Literal::Nat(42));
        let quoted = quote_expr(&expr);
        match &quoted {
            Expr::App(f, a) => {
                assert!(matches!(f.as_ref(), Expr::Const(n, _) if n == &
                    Name::str("Expr.lit")));
                assert_eq!(*a.as_ref(), Expr::Lit(Literal::Nat(42)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_unquote_const() {
        let quoted = Expr::App(
            Box::new(Expr::Const(Name::str("Expr.const"), vec![])),
            Box::new(Expr::Lit(Literal::Str("Foo".to_string()))),
        );
        let result = unquote_expr(&quoted);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed"),
            Expr::Const(Name::str("Foo"), vec![])
        );
    }
    #[test]
    fn test_unquote_lit() {
        let quoted = Expr::App(
            Box::new(Expr::Const(Name::str("Expr.lit"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(7))),
        );
        let result = unquote_expr(&quoted);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed"),
            Expr::Lit(Literal::Nat(7))
        );
    }
    #[test]
    fn test_roundtrip_quote_unquote_const() {
        let original = Expr::Const(Name::str("Nat.add"), vec![]);
        let quoted = quote_expr(&original);
        let unquoted = unquote_expr(&quoted).expect("test operation should succeed");
        assert_eq!(original, unquoted);
    }
    #[test]
    fn test_rule_based_macro() {
        let mut expander = MacroExpander::new();
        let rule = MacroRule {
            pattern: MacroPattern::Exact(Name::str("myMacro")),
            template: MacroTemplate::Expr(Expr::Lit(Literal::Nat(100))),
        };
        let macro_def =
            MacroDef::with_rules(Name::str("myMacro"), MacroKind::TermMacro, vec![rule]);
        expander.register(macro_def);
        let expr = Expr::Const(Name::str("myMacro"), vec![]);
        let result = expander.expand_checked(&expr);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed"),
            Expr::Lit(Literal::Nat(100))
        );
    }
    #[test]
    fn test_fresh_name() {
        let mut expander = MacroExpander::new();
        let n1 = expander.fresh_name("x");
        let n2 = expander.fresh_name("x");
        assert_ne!(n1, n2);
    }
    #[test]
    fn test_macro_error_display() {
        let e = MacroError::DepthExceeded;
        assert_eq!(e.to_string(), "macro expansion depth exceeded");
        let e2 = MacroError::UndefinedMacro("foo".to_string());
        assert!(e2.to_string().contains("foo"));
    }
    #[test]
    fn test_expand_in_app() {
        let mut expander = MacroExpander::new();
        expander.register(MacroDef::simple(
            Name::str("m"),
            vec![],
            Expr::Lit(Literal::Nat(1)),
        ));
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("m"), vec![])),
        );
        let result = expander
            .expand(&expr)
            .expect("macro expansion should succeed");
        match &result {
            Expr::App(_, a) => {
                assert_eq!(*a.as_ref(), Expr::Lit(Literal::Nat(1)));
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_substitute_quote_template() {
        let expander = MacroExpander::new();
        let template = MacroTemplate::Quote(Box::new(MacroTemplate::Var(Name::str("x"))));
        let mut bindings = HashMap::new();
        bindings.insert(Name::str("x"), Expr::Lit(Literal::Nat(5)));
        let result = expander.substitute_template(&template, &bindings);
        assert!(result.is_ok());
        let quoted = result.expect("test operation should succeed");
        match &quoted {
            Expr::App(f, a) => {
                assert!(matches!(f.as_ref(), Expr::Const(n, _) if n == &
                    Name::str("Expr.lit")));
                assert_eq!(*a.as_ref(), Expr::Lit(Literal::Nat(5)));
            }
            _ => panic!("Expected quoted literal"),
        }
    }
}
#[cfg(test)]
mod macro_expand_extended_tests {
    use super::*;
    use crate::macro_expand::*;
    fn make_macro_def(name: &str) -> MacroDef {
        let rule = MacroRule {
            pattern: MacroPattern::Var(Name::str("_")),
            template: MacroTemplate::Var(Name::str("x")),
        };
        MacroDef {
            name: Name::str(name),
            kind: MacroKind::NotationMacro,
            rules: vec![rule],
            hygiene: HygieneMode::Hygienic,
            params: Vec::new(),
            template: Expr::BVar(0),
            doc: None,
            scope: None,
        }
    }
    #[test]
    fn test_macro_tracer_record() {
        let mut tracer = MacroTracer::new(10);
        tracer.record(MacroTraceEntry::Entered {
            macro_name: Name::str("myMacro"),
            depth: 1,
        });
        tracer.record(MacroTraceEntry::Expanded { result_size: 5 });
        tracer.record(MacroTraceEntry::Error {
            message: "oops".to_string(),
        });
        assert_eq!(tracer.expansion_count(), 1);
        assert_eq!(tracer.error_count(), 1);
        assert_eq!(tracer.entries().len(), 3);
    }
    #[test]
    fn test_macro_tracer_disabled() {
        let mut tracer = MacroTracer::disabled();
        tracer.record(MacroTraceEntry::Matched { rule_index: 0 });
        assert!(tracer.is_empty());
    }
    #[test]
    fn test_macro_tracer_ring_buffer() {
        let mut tracer = MacroTracer::new(3);
        for i in 0..5 {
            tracer.record(MacroTraceEntry::Expanded { result_size: i });
        }
        assert_eq!(tracer.entries().len(), 3);
    }
    #[test]
    fn test_macro_environment_lookup() {
        let mut env = MacroEnvironment::new();
        env.bind(Name::str("x"), Expr::BVar(0));
        assert!(env.lookup(&Name::str("x")).is_some());
        assert!(env.lookup(&Name::str("y")).is_none());
        assert_eq!(env.local_count(), 1);
    }
    #[test]
    fn test_macro_environment_child_lookup() {
        let mut parent = MacroEnvironment::new();
        parent.bind(Name::str("x"), Expr::BVar(0));
        let mut child = MacroEnvironment::child(parent);
        child.bind(Name::str("y"), Expr::BVar(1));
        assert!(child.lookup(&Name::str("x")).is_some());
        assert!(child.lookup(&Name::str("y")).is_some());
        assert_eq!(child.depth(), 1);
    }
    #[test]
    fn test_macro_registry_register_lookup() {
        let mut reg = MacroRegistry::new();
        reg.register(make_macro_def("myMacro"));
        assert!(reg.has_macro(&Name::str("myMacro")));
        assert!(!reg.has_macro(&Name::str("other")));
        assert_eq!(reg.count(), 1);
    }
    #[test]
    fn test_macro_registry_remove() {
        let mut reg = MacroRegistry::new();
        reg.register(make_macro_def("m1"));
        assert!(reg.remove(&Name::str("m1")));
        assert!(!reg.remove(&Name::str("m1")));
        assert!(reg.is_empty());
    }
    #[test]
    fn test_macro_pattern_matcher() {
        let matcher = MacroPatternMatcher::new();
        let wild = MacroPattern::Var(Name::str("_"));
        assert!(matcher.matches_shallow(&wild, &Expr::BVar(0)));
        let var = MacroPattern::Var(Name::str("x"));
        assert!(matcher.matches_shallow(&var, &Expr::BVar(5)));
    }
    #[test]
    fn test_macro_expansion_stats() {
        let mut stats = MacroExpansionStats::new();
        stats.record_success(3, 2);
        stats.record_success(5, 4);
        stats.record_failure();
        assert_eq!(stats.total_calls, 3);
        assert_eq!(stats.successful, 2);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.max_depth_seen, 4);
        assert!((stats.mean_expansions() - 4.0).abs() < 1e-9);
        assert!((stats.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_macro_scope_stack() {
        let mut stack = MacroScopeStack::new();
        assert!(stack.is_empty());
        stack.push_scope();
        stack.add_to_current(make_macro_def("m1"));
        stack.push_scope();
        stack.add_to_current(make_macro_def("m2"));
        assert_eq!(stack.depth(), 2);
        let visible = stack.visible_macros();
        assert_eq!(visible.len(), 2);
        let _popped = stack.pop_scope();
        assert_eq!(stack.depth(), 1);
    }
    #[test]
    fn test_macro_hygiene_map() {
        let mut hmap = MacroHygieneMap::new("__fresh__");
        let fresh1 = hmap.fresh_name(&Name::str("x"));
        let fresh2 = hmap.fresh_name(&Name::str("y"));
        assert_ne!(fresh1, fresh2);
        assert_eq!(hmap.rename_count(), 2);
        let applied = hmap.apply(&Name::str("x"));
        assert_eq!(applied, fresh1);
        let unchanged = hmap.apply(&Name::str("z"));
        assert_eq!(unchanged, Name::str("z"));
    }
    #[test]
    fn test_macro_validator_empty_rules() {
        let def = MacroDef {
            name: Name::str("bad"),
            kind: MacroKind::NotationMacro,
            rules: vec![],
            hygiene: HygieneMode::Hygienic,
            params: Vec::new(),
            template: Expr::BVar(0),
            doc: None,
            scope: None,
        };
        let v = MacroValidator::new();
        assert!(!v.is_valid(&def));
        let errors = v.validate(&def);
        assert!(matches!(errors[0], MacroValidationError::EmptyRuleList));
    }
    #[test]
    fn test_macro_validator_duplicate_priority() {
        let rule1 = MacroRule {
            pattern: MacroPattern::Var(Name::str("_")),
            template: MacroTemplate::Var(Name::str("x")),
        };
        let rule2 = MacroRule {
            pattern: MacroPattern::Var(Name::str("_")),
            template: MacroTemplate::Var(Name::str("y")),
        };
        let def = MacroDef {
            name: Name::str("dup"),
            kind: MacroKind::NotationMacro,
            rules: vec![rule1, rule2],
            hygiene: HygieneMode::Hygienic,
            params: Vec::new(),
            template: Expr::BVar(0),
            doc: None,
            scope: None,
        };
        let v = MacroValidator::new();
        let errors = v.validate(&def);
        assert!(!errors.is_empty());
    }
    #[test]
    fn test_expansion_result_builder() {
        let result = MacroExpansionResult::new(Expr::BVar(0))
            .with_expansions(3)
            .with_warning("deprecated syntax");
        assert_eq!(result.expansions_applied, 3);
        assert!(result.has_warnings());
    }
}
#[allow(dead_code)]
pub trait MacroTransformStep: Send + Sync {
    fn step_name(&self) -> &'static str;
    fn transform(&self, expr: Expr) -> Result<Expr, MacroError>;
}
#[cfg(test)]
mod macro_extend_tests2 {
    use super::*;
    use crate::macro_expand::*;
    #[test]
    fn test_macro_pipeline_identity() {
        let pipeline = MacroPipeline::new().add_step(IdentityStep);
        let expr = Expr::BVar(0);
        let result = pipeline.run(expr.clone());
        assert!(result.is_ok());
        assert_eq!(result.expect("test operation should succeed"), expr);
    }
    #[test]
    fn test_macro_pipeline_step_names() {
        let pipeline = MacroPipeline::new()
            .add_step(IdentityStep)
            .add_step(IdentityStep);
        assert_eq!(pipeline.step_names(), vec!["identity", "identity"]);
        assert_eq!(pipeline.len(), 2);
    }
    #[test]
    fn test_macro_namespace_lookup() {
        let mut ns = MacroNamespace::new("Std");
        let rule = MacroRule {
            pattern: MacroPattern::Var(Name::str("x")),
            template: MacroTemplate::Var(Name::str("x")),
        };
        let def = MacroDef {
            name: Name::str("Std.myMacro"),
            kind: MacroKind::NotationMacro,
            rules: vec![rule],
            hygiene: HygieneMode::Hygienic,
            params: Vec::new(),
            template: Expr::BVar(0),
            doc: None,
            scope: None,
        };
        ns.register(def);
        assert_eq!(ns.count(), 1);
        assert!(ns.lookup("Std.myMacro").is_some());
        assert_eq!(ns.qualified_name("foo"), "Std.foo");
    }
    #[test]
    fn test_macro_ast_operations() {
        let ast = MacroAst::list(vec![MacroAst::atom("a"), MacroAst::num(42), MacroAst::Nil]);
        assert_eq!(ast.len(), 3);
        assert!(ast.head().expect("test operation should succeed").is_atom());
    }
    #[test]
    fn test_macro_ast_cons() {
        let cons = MacroAst::Cons(Box::new(MacroAst::atom("head")), Box::new(MacroAst::Nil));
        assert!(cons.head().is_some());
        assert!(cons.tail().expect("test operation should succeed").is_nil());
    }
    #[test]
    fn test_macro_interpreter_define_eval() {
        let mut interp = MacroInterpreter::new(10);
        interp.define(Name::str("x"), MacroAst::num(5));
        assert!(interp.is_defined(&Name::str("x")));
        let val = interp
            .eval_atom(&Name::str("x"))
            .expect("test operation should succeed");
        assert_eq!(*val, MacroAst::num(5));
    }
    #[test]
    fn test_macro_interpreter_depth_limit() {
        let mut interp = MacroInterpreter::new(2);
        assert!(interp.push_call());
        assert!(interp.push_call());
        assert!(!interp.push_call());
        interp.pop_call();
        assert!(interp.push_call());
    }
}
#[cfg(test)]
mod macro_config_tests {
    use super::*;
    use crate::macro_expand::*;
    #[test]
    fn test_expansion_config_builder() {
        let cfg = MacroExpansionConfig::new()
            .with_max_depth(50)
            .with_trace()
            .without_cache();
        assert_eq!(cfg.max_depth, 50);
        assert!(cfg.enable_trace);
        assert!(!cfg.enable_caching);
    }
    #[test]
    fn test_expansion_report() {
        let mut report = MacroExpansionReport::new();
        report.record_application(Name::str("myMacro"));
        report.record_warning("deprecated usage");
        assert_eq!(report.applied_count(), 1);
        assert!(!report.has_errors());
        report.record_error("expansion failed");
        assert!(report.has_errors());
    }
}
#[cfg(test)]
mod macro_final_tests {
    use super::*;
    use crate::macro_expand::*;
    #[test]
    fn test_macro_extension_marker() {
        let _m = MacroExtensionMarker;
    }
}
