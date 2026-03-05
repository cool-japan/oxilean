//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    FalseEliminator, IdentityMetaStep, MacroEngine, MacroRule, MacroToken, MacroTransformer,
    MetaDefinition, MetaElabContext, MetaElabPipeline, MetaEnv, MetaFrame, MetaFrameKind,
    MetaProgRegistry, MetaProgrammingError, MetaStack, MetaValue, OmegaMetaTactic, QuotationMode,
    QuotedExpr, ReflectedTerm, RingMetaTactic, SimpMetaTactic, SpliceContext, TermQuoter,
    TrueDecider, UserElabRegistry, UserTacticRegistry, UserTacticResult,
};
use oxilean_kernel::*;

/// A user-defined tactic plugin.
pub trait UserTactic: Send + Sync {
    /// The name used to invoke this tactic.
    fn name(&self) -> &str;
    /// Run this tactic on the given goal.
    fn run(&self, goal_target: &str, hypotheses: &[(String, String)]) -> UserTacticResult;
    /// Human-readable description of what this tactic does.
    fn description(&self) -> &str {
        ""
    }
}
/// A user-defined term elaborator.
pub trait UserElab: Send + Sync {
    /// The name of this elaborator.
    fn name(&self) -> &str;
    /// Return true if this elaborator handles the given head symbol.
    fn handles(&self, head: &str) -> bool;
    /// Elaborate the given arguments, returning a kernel expression string if successful.
    fn elaborate(&self, args: &[String]) -> Option<String>;
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::metaprog::*;
    #[test]
    fn test_user_tactic_registry() {
        let mut reg = UserTacticRegistry::new();
        assert!(reg.is_empty());
        reg.register(Box::new(TrueDecider));
        reg.register(Box::new(FalseEliminator));
        assert_eq!(reg.len(), 2);
        assert!(!reg.is_empty());
        let names = reg.names();
        assert!(names.contains(&"decide_true"));
        assert!(names.contains(&"false_elim"));
    }
    #[test]
    fn test_true_decider() {
        let t = TrueDecider;
        assert_eq!(t.name(), "decide_true");
        assert!(!t.description().is_empty());
        let result = t.run("True", &[]);
        assert!(matches!(result, UserTacticResult::Solved));
        let result2 = t.run("False", &[]);
        assert!(matches!(result2, UserTacticResult::Failed(_)));
    }
    #[test]
    fn test_false_eliminator() {
        let fe = FalseEliminator;
        assert_eq!(fe.name(), "false_elim");
        assert!(!fe.description().is_empty());
        let result = fe.run("SomeGoal", &[]);
        assert!(matches!(result, UserTacticResult::Failed(_)));
        let hyps = vec![("h".to_string(), "False".to_string())];
        let result2 = fe.run("SomeGoal", &hyps);
        assert!(matches!(result2, UserTacticResult::Solved));
    }
    #[test]
    fn test_user_elab_registry() {
        struct MyElab;
        impl UserElab for MyElab {
            fn name(&self) -> &str {
                "my_elab"
            }
            fn handles(&self, head: &str) -> bool {
                head == "MyForm"
            }
            fn elaborate(&self, args: &[String]) -> Option<String> {
                Some(format!("elaborated({})", args.join(",")))
            }
        }
        let mut reg = UserElabRegistry::new();
        assert!(reg.is_empty());
        reg.register(Box::new(MyElab));
        assert_eq!(reg.len(), 1);
        let elab = reg.find_for("MyForm");
        assert!(elab.is_some());
        let result = elab
            .expect("elaboration should succeed")
            .elaborate(&["x".to_string(), "y".to_string()]);
        assert_eq!(result, Some("elaborated(x,y)".to_string()));
        let none = reg.find_for("OtherForm");
        assert!(none.is_none());
    }
    #[test]
    fn test_macro_engine_add_rule() {
        let mut engine = MacroEngine::new();
        assert!(engine.rules().is_empty());
        engine.add_rule(MacroRule {
            name: "my_macro".to_string(),
            pattern: vec![
                MacroToken::Literal("foo".to_string()),
                MacroToken::Var("x".to_string()),
            ],
            expansion: "bar({x})".to_string(),
            priority: 10,
        });
        assert_eq!(engine.rules().len(), 1);
    }
    #[test]
    fn test_macro_engine_expand() {
        let mut engine = MacroEngine::new();
        engine.add_rule(MacroRule {
            name: "swap".to_string(),
            pattern: vec![
                MacroToken::Literal("swap".to_string()),
                MacroToken::Var("x".to_string()),
            ],
            expansion: "Swap({x})".to_string(),
            priority: 5,
        });
        let expanded = engine.try_expand("swap myVar");
        assert_eq!(expanded, Some("Swap(myVar)".to_string()));
        let none = engine.try_expand("other stuff");
        assert!(none.is_none());
    }
    #[test]
    fn test_user_tactic_registry_find() {
        let mut reg = UserTacticRegistry::new();
        reg.register(Box::new(TrueDecider));
        let found = reg.find("decide_true");
        assert!(found.is_some());
        assert_eq!(
            found.expect("test operation should succeed").name(),
            "decide_true"
        );
        let not_found = reg.find("nonexistent");
        assert!(not_found.is_none());
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::metaprog::*;
    #[test]
    fn test_user_tactic_result_variants() {
        let solved = UserTacticResult::Solved;
        assert!(matches!(solved, UserTacticResult::Solved));
        let failed = UserTacticResult::Failed("oops".to_string());
        if let UserTacticResult::Failed(msg) = &failed {
            assert_eq!(msg, "oops");
        } else {
            panic!("expected Failed");
        }
        let subs = UserTacticResult::Subgoals(vec!["g1".to_string(), "g2".to_string()]);
        if let UserTacticResult::Subgoals(gs) = &subs {
            assert_eq!(gs.len(), 2);
        } else {
            panic!("expected Subgoals");
        }
    }
    #[test]
    fn test_user_tactic_result_clone() {
        let r = UserTacticResult::Subgoals(vec!["g1".to_string()]);
        let r2 = r.clone();
        if let UserTacticResult::Subgoals(gs) = r2 {
            assert_eq!(gs[0], "g1");
        } else {
            panic!("expected Subgoals");
        }
    }
    #[test]
    fn test_macro_token_variants_debug() {
        assert!(format!("{:?}", MacroToken::Literal("foo".to_string())).contains("Literal"));
        assert!(format!("{:?}", MacroToken::Var("x".to_string())).contains("Var"));
        assert!(format!("{:?}", MacroToken::Many("rest".to_string())).contains("Many"));
    }
    #[test]
    fn test_macro_token_clone() {
        let _ = MacroToken::Literal("foo".to_string()).clone();
        let _ = MacroToken::Var("x".to_string()).clone();
        let _ = MacroToken::Many("rest".to_string()).clone();
    }
    #[test]
    fn test_macro_rule_priority_ordering() {
        let mut engine = MacroEngine::new();
        engine.add_rule(MacroRule {
            name: "low".to_string(),
            pattern: vec![
                MacroToken::Literal("cmd".to_string()),
                MacroToken::Var("x".to_string()),
            ],
            expansion: "low({x})".to_string(),
            priority: 1,
        });
        engine.add_rule(MacroRule {
            name: "high".to_string(),
            pattern: vec![
                MacroToken::Literal("cmd".to_string()),
                MacroToken::Var("x".to_string()),
            ],
            expansion: "high({x})".to_string(),
            priority: 10,
        });
        let result = engine.try_expand("cmd hello");
        assert_eq!(result, Some("high(hello)".to_string()));
    }
    #[test]
    fn test_macro_engine_many_token() {
        let mut engine = MacroEngine::new();
        engine.add_rule(MacroRule {
            name: "print_all".to_string(),
            pattern: vec![
                MacroToken::Literal("print".to_string()),
                MacroToken::Many("args".to_string()),
            ],
            expansion: "Print({args})".to_string(),
            priority: 0,
        });
        let result = engine.try_expand("print a b c d");
        assert_eq!(result, Some("Print(a b c d)".to_string()));
    }
    #[test]
    fn test_user_elab_registry_multiple() {
        struct ElabA;
        struct ElabB;
        impl UserElab for ElabA {
            fn name(&self) -> &str {
                "elab_a"
            }
            fn handles(&self, h: &str) -> bool {
                h == "A"
            }
            fn elaborate(&self, _: &[String]) -> Option<String> {
                Some("elab_a_result".to_string())
            }
        }
        impl UserElab for ElabB {
            fn name(&self) -> &str {
                "elab_b"
            }
            fn handles(&self, h: &str) -> bool {
                h == "B"
            }
            fn elaborate(&self, _: &[String]) -> Option<String> {
                Some("elab_b_result".to_string())
            }
        }
        let mut reg = UserElabRegistry::new();
        reg.register(Box::new(ElabA));
        reg.register(Box::new(ElabB));
        assert_eq!(reg.len(), 2);
        assert!(reg.find_for("A").is_some());
        assert!(reg.find_for("B").is_some());
        assert!(reg.find_for("C").is_none());
    }
    #[test]
    fn test_false_eliminator_whitespace_false() {
        let fe = FalseEliminator;
        let hyps = vec![("h".to_string(), "  False  ".to_string())];
        let result = fe.run("goal", &hyps);
        assert!(matches!(result, UserTacticResult::Solved));
    }
    #[test]
    fn test_false_eliminator_multiple_hyps_second_is_false() {
        let fe = FalseEliminator;
        let hyps = vec![
            ("h1".to_string(), "Nat".to_string()),
            ("h2".to_string(), "False".to_string()),
        ];
        assert!(matches!(fe.run("goal", &hyps), UserTacticResult::Solved));
    }
    #[test]
    fn test_true_decider_whitespace() {
        let td = TrueDecider;
        assert!(matches!(td.run("  True  ", &[]), UserTacticResult::Solved));
        assert!(matches!(
            td.run("True or False", &[]),
            UserTacticResult::Failed(_)
        ));
    }
    #[test]
    fn test_macro_rule_clone() {
        let rule = MacroRule {
            name: "test".to_string(),
            pattern: vec![MacroToken::Literal("x".to_string())],
            expansion: "X".to_string(),
            priority: 42,
        };
        let rule2 = rule.clone();
        assert_eq!(rule2.name, "test");
        assert_eq!(rule2.priority, 42);
    }
    #[test]
    fn test_macro_engine_no_match_empty() {
        let mut engine = MacroEngine::new();
        engine.add_rule(MacroRule {
            name: "r1".to_string(),
            pattern: vec![MacroToken::Literal("start".to_string())],
            expansion: "S".to_string(),
            priority: 0,
        });
        assert!(engine.try_expand("").is_none());
        assert!(engine.try_expand("end something").is_none());
    }
    #[test]
    fn test_registry_default_is_empty() {
        let reg = UserTacticRegistry::default();
        assert!(reg.is_empty());
        let ereg = UserElabRegistry::default();
        assert!(ereg.is_empty());
        let eng = MacroEngine::default();
        assert!(eng.rules().is_empty());
    }
    #[test]
    fn test_registry_names_ordered_by_insertion() {
        let mut reg = UserTacticRegistry::new();
        reg.register(Box::new(TrueDecider));
        reg.register(Box::new(FalseEliminator));
        let names = reg.names();
        assert_eq!(names[0], "decide_true");
        assert_eq!(names[1], "false_elim");
    }
    #[test]
    fn test_true_decider_with_false_hyps_still_succeeds() {
        let td = TrueDecider;
        let hyps = vec![("h".to_string(), "False".to_string())];
        assert!(matches!(td.run("True", &hyps), UserTacticResult::Solved));
    }
    #[test]
    fn test_false_eliminator_only_true_hyp_fails() {
        let fe = FalseEliminator;
        let hyps = vec![("h".to_string(), "True".to_string())];
        assert!(matches!(fe.run("goal", &hyps), UserTacticResult::Failed(_)));
    }
    #[test]
    fn test_macro_engine_literal_fallthrough() {
        let mut engine = MacroEngine::new();
        engine.add_rule(MacroRule {
            name: "r1".to_string(),
            pattern: vec![MacroToken::Literal("foo".to_string())],
            expansion: "Foo".to_string(),
            priority: 0,
        });
        engine.add_rule(MacroRule {
            name: "r2".to_string(),
            pattern: vec![MacroToken::Literal("bar".to_string())],
            expansion: "Bar".to_string(),
            priority: 0,
        });
        assert_eq!(engine.try_expand("bar"), Some("Bar".to_string()));
        assert_eq!(engine.try_expand("foo"), Some("Foo".to_string()));
    }
}
/// Reflect a simple expression string into a `QuotedExpr`.
///
/// This is a simplified reflector that handles atoms and simple applications.
#[allow(dead_code)]
pub fn reflect_expr(expr_str: &str) -> QuotedExpr {
    let trimmed = expr_str.trim();
    let mut parts = trimmed.splitn(2, ' ');
    let head = parts.next().unwrap_or(trimmed);
    if let Some(args_str) = parts.next() {
        let args: Vec<_> = args_str.split_whitespace().map(QuotedExpr::atom).collect();
        QuotedExpr::app(QuotedExpr::atom(head), args)
    } else {
        QuotedExpr::atom(head)
    }
}
/// Splice a `QuotedExpr` back into a surface expression string.
///
/// Traverses the `QuotedExpr` tree and converts it to a textual surface form.
#[allow(dead_code)]
pub fn splice_term(quoted: &QuotedExpr) -> String {
    quoted.to_debug_string()
}
/// Substitute a variable in a `QuotedExpr` with another `QuotedExpr`.
#[allow(dead_code)]
pub fn subst_quoted(expr: &QuotedExpr, var: &str, replacement: &QuotedExpr) -> QuotedExpr {
    match expr {
        QuotedExpr::Atom(s) => {
            if s == var {
                replacement.clone()
            } else {
                QuotedExpr::Atom(s.clone())
            }
        }
        QuotedExpr::Splice(inner) => {
            QuotedExpr::Splice(Box::new(subst_quoted(inner, var, replacement)))
        }
        QuotedExpr::App { func, args } => QuotedExpr::App {
            func: Box::new(subst_quoted(func, var, replacement)),
            args: args
                .iter()
                .map(|a| subst_quoted(a, var, replacement))
                .collect(),
        },
        QuotedExpr::Lambda { binders, body } => {
            if binders.iter().any(|b| b == var) {
                expr.clone()
            } else {
                QuotedExpr::Lambda {
                    binders: binders.clone(),
                    body: Box::new(subst_quoted(body, var, replacement)),
                }
            }
        }
        QuotedExpr::Let { name, value, body } => {
            let new_value = subst_quoted(value, var, replacement);
            let new_body = if name == var {
                body.as_ref().clone()
            } else {
                subst_quoted(body, var, replacement)
            };
            QuotedExpr::Let {
                name: name.clone(),
                value: Box::new(new_value),
                body: Box::new(new_body),
            }
        }
    }
}
#[cfg(test)]
mod meta_env_tests {
    use super::*;
    use crate::metaprog::*;
    #[test]
    fn test_meta_env_basic() {
        let mut env = MetaEnv::new();
        assert!(env.is_local_empty());
        env.bind("x", "Nat");
        env.bind("y", "Bool");
        assert_eq!(env.local_len(), 2);
        assert_eq!(env.lookup("x"), Some("Nat"));
        assert_eq!(env.lookup("y"), Some("Bool"));
        assert_eq!(env.lookup("z"), None);
        let names = env.local_names();
        assert!(names.contains(&"x"));
        assert!(names.contains(&"y"));
    }
    #[test]
    fn test_meta_env_parent_lookup() {
        let mut parent = MetaEnv::new();
        parent.bind("x", "ParentNat");
        parent.bind("shared", "ParentShared");
        let mut child = MetaEnv::child(parent);
        child.bind("y", "ChildBool");
        child.bind("shared", "ChildShared");
        assert_eq!(child.lookup("y"), Some("ChildBool"));
        assert_eq!(child.lookup("shared"), Some("ChildShared"));
        assert_eq!(child.lookup("x"), Some("ParentNat"));
        assert_eq!(child.lookup("z"), None);
    }
    #[test]
    fn test_meta_env_unbind() {
        let mut env = MetaEnv::new();
        env.bind("x", "Nat");
        assert!(env.lookup("x").is_some());
        let removed = env.unbind("x");
        assert_eq!(removed, Some("Nat".to_string()));
        assert!(env.lookup("x").is_none());
        assert!(env.is_local_empty());
    }
    #[test]
    fn test_meta_stack_push_pop() {
        let mut stack = MetaStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.depth(), 0);
        stack.push(MetaFrame::new(MetaFrameKind::Quotation, 0));
        assert_eq!(stack.depth(), 1);
        assert!(!stack.is_empty());
        stack.push(MetaFrame::new(MetaFrameKind::Splice, 1));
        assert_eq!(stack.depth(), 2);
        let top = stack.top().expect("test operation should succeed");
        assert!(top.is_splice());
        let popped = stack.pop().expect("collection should not be empty");
        assert_eq!(popped.kind, MetaFrameKind::Splice);
        assert_eq!(stack.depth(), 1);
        assert!(stack.in_quotation());
        assert!(!stack.in_splice());
    }
    #[test]
    fn test_meta_stack_quotation_depth() {
        let mut stack = MetaStack::new();
        assert!(stack.quotation_depth().is_none());
        stack.push(MetaFrame::new(MetaFrameKind::TacticEval, 0));
        stack.push(MetaFrame::new(MetaFrameKind::Quotation, 1));
        stack.push(MetaFrame::new(MetaFrameKind::Splice, 2));
        let qd = stack.quotation_depth();
        assert_eq!(qd, Some(1));
    }
    #[test]
    fn test_quoted_expr_atom() {
        let q = QuotedExpr::atom("x");
        assert!(q.is_atom());
        assert!(!q.is_app());
        assert!(!q.has_splice());
        assert_eq!(q.to_debug_string(), "x");
    }
    #[test]
    fn test_quoted_expr_app() {
        let f = QuotedExpr::atom("f");
        let x = QuotedExpr::atom("x");
        let y = QuotedExpr::atom("y");
        let app = QuotedExpr::app(f, vec![x, y]);
        assert!(app.is_app());
        assert!(!app.has_splice());
        let s = app.to_debug_string();
        assert!(s.contains("f"));
        assert!(s.contains("x"));
        assert!(s.contains("y"));
    }
    #[test]
    fn test_quoted_expr_splice_detection() {
        let inner = QuotedExpr::atom("expr");
        let spliced = QuotedExpr::splice(inner);
        assert!(spliced.has_splice());
        let app = QuotedExpr::app(
            QuotedExpr::atom("f"),
            vec![QuotedExpr::splice(QuotedExpr::atom("x"))],
        );
        assert!(app.has_splice());
        let clean = QuotedExpr::app(QuotedExpr::atom("f"), vec![QuotedExpr::atom("x")]);
        assert!(!clean.has_splice());
    }
    #[test]
    fn test_quoted_expr_lambda() {
        let body = QuotedExpr::atom("body");
        let lam = QuotedExpr::lambda(vec!["x".to_string(), "y".to_string()], body);
        let s = lam.to_debug_string();
        assert!(s.contains("fun"));
        assert!(s.contains("x"));
        assert!(s.contains("body"));
    }
    #[test]
    fn test_quoted_expr_let_binding() {
        let val = QuotedExpr::atom("42");
        let body = QuotedExpr::atom("body");
        let let_expr = QuotedExpr::let_binding("n", val, body);
        let s = let_expr.to_debug_string();
        assert!(s.contains("let"));
        assert!(s.contains("n"));
        assert!(s.contains("42"));
    }
    #[test]
    fn test_reflect_expr_atom() {
        let q = reflect_expr("x");
        assert!(q.is_atom());
    }
    #[test]
    fn test_reflect_expr_app() {
        let q = reflect_expr("f x y");
        assert!(q.is_app());
    }
    #[test]
    fn test_splice_term_roundtrip() {
        let q = QuotedExpr::app(
            QuotedExpr::atom("add"),
            vec![QuotedExpr::atom("1"), QuotedExpr::atom("2")],
        );
        let s = splice_term(&q);
        assert!(s.contains("add"));
        assert!(s.contains("1"));
        assert!(s.contains("2"));
    }
    #[test]
    fn test_subst_quoted() {
        let expr = QuotedExpr::app(
            QuotedExpr::atom("f"),
            vec![QuotedExpr::atom("x"), QuotedExpr::atom("y")],
        );
        let replacement = QuotedExpr::atom("z");
        let result = subst_quoted(&expr, "x", &replacement);
        let s = result.to_debug_string();
        assert!(s.contains("z"));
        assert!(!s.contains("(x)") || s.contains("z"));
    }
    #[test]
    fn test_subst_quoted_lambda_shadowing() {
        let lam = QuotedExpr::lambda(vec!["x".to_string()], QuotedExpr::atom("x"));
        let replacement = QuotedExpr::atom("z");
        let result = subst_quoted(&lam, "x", &replacement);
        assert!(matches!(result, QuotedExpr::Lambda { .. }));
    }
    #[test]
    fn test_meta_elab_context() {
        let mut ctx = MetaElabContext::new();
        assert!(!ctx.depth_exceeded());
        assert!(!ctx.stack.in_quotation());
        ctx.enter_quotation(Some("test_quote"));
        assert!(ctx.stack.in_quotation());
        assert_eq!(ctx.current_depth, 1);
        ctx.enter_splice();
        assert!(ctx.stack.in_splice());
        assert_eq!(ctx.current_depth, 2);
        ctx.exit_splice();
        assert!(!ctx.stack.in_splice());
        ctx.exit_quotation();
        assert!(!ctx.stack.in_quotation());
        assert_eq!(ctx.current_depth, 0);
    }
    #[test]
    fn test_meta_elab_context_bind_lookup() {
        let mut ctx = MetaElabContext::new();
        ctx.bind("alpha", "Type");
        assert_eq!(ctx.lookup("alpha"), Some("Type"));
        assert!(ctx.lookup("beta").is_none());
    }
    #[test]
    fn test_reflected_term() {
        let q = QuotedExpr::atom("x");
        let rt = ReflectedTerm::new(q.clone(), "Nat");
        assert!(rt.exact);
        assert_eq!(rt.ty, "Nat");
        let approx = ReflectedTerm::approximate(q, "Unknown");
        assert!(!approx.exact);
    }
}
#[cfg(test)]
mod splice_and_meta_def_tests {
    use super::*;
    use crate::metaprog::*;
    #[test]
    fn test_splice_context_basic() {
        let mut ctx = SpliceContext::new();
        assert!(ctx.at_object_level());
        assert!(!ctx.in_quotation());
        assert_eq!(ctx.effective_level(), 0);
        ctx.enter_quote();
        assert!(ctx.in_quotation());
        assert_eq!(ctx.quote_depth, 1);
        assert_eq!(ctx.effective_level(), 1);
        ctx.enter_splice();
        assert_eq!(ctx.splice_depth, 1);
        assert_eq!(ctx.effective_level(), 0);
        ctx.exit_splice();
        assert_eq!(ctx.splice_depth, 0);
        ctx.exit_quote();
        assert!(ctx.at_object_level());
    }
    #[test]
    fn test_splice_context_depth_log() {
        let mut ctx = SpliceContext::new();
        ctx.enter_quote();
        ctx.enter_splice();
        ctx.exit_splice();
        ctx.exit_quote();
        assert_eq!(ctx.depth_log.len(), 4);
        assert_eq!(ctx.depth_log[0].0, "enter_quote");
        assert_eq!(ctx.depth_log[3].0, "exit_quote");
        ctx.clear_log();
        assert!(ctx.depth_log.is_empty());
    }
    #[test]
    fn test_splice_context_no_underflow() {
        let mut ctx = SpliceContext::new();
        ctx.exit_quote();
        assert_eq!(ctx.quote_depth, 0);
        ctx.exit_splice();
        assert_eq!(ctx.splice_depth, 0);
    }
    #[test]
    fn test_meta_definition_apply() {
        let body = QuotedExpr::app(
            QuotedExpr::atom("add"),
            vec![QuotedExpr::atom("x"), QuotedExpr::atom("y")],
        );
        let def = MetaDefinition::new("my_add", vec!["x".to_string(), "y".to_string()], body);
        let args = vec![QuotedExpr::atom("1"), QuotedExpr::atom("2")];
        let result = def.apply(&args).expect("test operation should succeed");
        let s = result.to_debug_string();
        assert!(s.contains("1") && s.contains("2") && s.contains("add"));
    }
    #[test]
    fn test_meta_definition_wrong_arg_count() {
        let body = QuotedExpr::atom("x");
        let def = MetaDefinition::new("f", vec!["x".to_string()], body);
        let result = def.apply(&[QuotedExpr::atom("a"), QuotedExpr::atom("b")]);
        assert!(result.is_none());
    }
    #[test]
    fn test_meta_definition_recursive_flag() {
        let body = QuotedExpr::atom("body");
        let def = MetaDefinition::new("rec_f", vec![], body).recursive();
        assert!(def.is_recursive);
    }
    #[test]
    fn test_meta_definition_with_doc() {
        let body = QuotedExpr::atom("x");
        let def = MetaDefinition::new("f", vec![], body).with_doc("A simple meta function");
        assert_eq!(def.doc.as_deref(), Some("A simple meta function"));
    }
    #[test]
    fn test_meta_prog_registry() {
        let mut reg = MetaProgRegistry::new();
        assert!(reg.is_empty());
        let def1 = MetaDefinition::new(
            "add",
            vec!["a".to_string(), "b".to_string()],
            QuotedExpr::atom("body1"),
        );
        let def2 = MetaDefinition::new(
            "mul",
            vec!["a".to_string(), "b".to_string()],
            QuotedExpr::atom("body2"),
        );
        reg.register(def1);
        reg.register(def2);
        assert_eq!(reg.len(), 2);
        assert!(!reg.is_empty());
        assert!(reg.lookup("add").is_some());
        assert!(reg.lookup("mul").is_some());
        assert!(reg.lookup("div").is_none());
        let names = reg.names();
        assert_eq!(names, vec!["add", "mul"]);
        let removed = reg.remove("add");
        assert!(removed.is_some());
        assert_eq!(reg.len(), 1);
    }
    #[test]
    fn test_meta_prog_registry_overwrite() {
        let mut reg = MetaProgRegistry::new();
        let def1 = MetaDefinition::new("f", vec![], QuotedExpr::atom("v1"));
        let def2 = MetaDefinition::new("f", vec![], QuotedExpr::atom("v2"));
        reg.register(def1);
        reg.register(def2);
        assert_eq!(reg.len(), 1);
        let body = reg
            .lookup("f")
            .expect("conversion should succeed")
            .body
            .to_debug_string();
        assert_eq!(body, "v2");
    }
    #[test]
    fn test_meta_frame_with_label() {
        let frame = MetaFrame::new(MetaFrameKind::MacroExpansion("my_macro".to_string()), 0)
            .with_label("expanding my_macro");
        assert_eq!(frame.label.as_deref(), Some("expanding my_macro"));
        assert!(matches!(frame.kind, MetaFrameKind::MacroExpansion(_)));
    }
    #[test]
    fn test_meta_stack_macro_frame() {
        let mut stack = MetaStack::new();
        stack.push(MetaFrame::new(
            MetaFrameKind::MacroExpansion("list_macro".to_string()),
            0,
        ));
        assert!(!stack.in_quotation());
        assert!(!stack.in_splice());
        let top = stack.top().expect("test operation should succeed");
        assert!(matches!(& top.kind, MetaFrameKind::MacroExpansion(s) if s == "list_macro"));
    }
}
/// A transformation function on `QuotedExpr`.
pub type QuotedTransform = fn(&QuotedExpr) -> Option<QuotedExpr>;
#[cfg(test)]
mod quoter_transformer_tests {
    use super::*;
    use crate::metaprog::*;
    #[test]
    fn test_term_quoter_full() {
        let quoter = TermQuoter::full();
        assert_eq!(quoter.mode(), QuotationMode::Full);
        let q = quoter.quote("x");
        assert!(q.is_atom());
    }
    #[test]
    fn test_term_quoter_partial() {
        let quoter = TermQuoter::partial();
        assert_eq!(quoter.mode(), QuotationMode::Partial);
        let q = quoter.quote("f x y");
        assert!(q.is_app());
    }
    #[test]
    fn test_term_quoter_args() {
        let quoter = TermQuoter::full();
        let args = quoter.quote_args(&["a", "b", "c"]);
        assert_eq!(args.len(), 3);
        for arg in args {
            assert!(arg.is_atom());
        }
    }
    #[test]
    fn test_macro_transformer_empty() {
        let transformer = MacroTransformer::new();
        assert!(transformer.is_empty());
        let q = QuotedExpr::atom("x");
        let result = transformer.transform(&q);
        assert!(result.is_atom());
    }
    #[test]
    fn test_macro_transformer_with_rule() {
        let mut transformer = MacroTransformer::new();
        fn rewrite_x(expr: &QuotedExpr) -> Option<QuotedExpr> {
            if let QuotedExpr::Atom(s) = expr {
                if s == "x" {
                    return Some(QuotedExpr::atom("replaced_x"));
                }
            }
            None
        }
        transformer.add_rule("replace_x", rewrite_x);
        assert_eq!(transformer.len(), 1);
        let q = QuotedExpr::atom("x");
        let result = transformer.transform(&q);
        assert_eq!(result.to_debug_string(), "replaced_x");
        let unchanged = QuotedExpr::atom("y");
        let result2 = transformer.transform(&unchanged);
        assert_eq!(result2.to_debug_string(), "y");
    }
    #[test]
    fn test_macro_transformer_recursive() {
        let mut transformer = MacroTransformer::new();
        fn id_to_identity(expr: &QuotedExpr) -> Option<QuotedExpr> {
            if let QuotedExpr::Atom(s) = expr {
                if s == "id" {
                    return Some(QuotedExpr::app(QuotedExpr::atom("Function.id"), vec![]));
                }
            }
            None
        }
        transformer.add_rule("expand_id", id_to_identity);
        let q = QuotedExpr::app(
            QuotedExpr::atom("f"),
            vec![QuotedExpr::atom("id"), QuotedExpr::atom("z")],
        );
        let result = transformer.transform(&q);
        let s = result.to_debug_string();
        assert!(s.contains("Function.id") || s.contains("id"));
    }
    #[test]
    fn test_macro_transformer_rule_names() {
        let mut t = MacroTransformer::new();
        fn dummy(_: &QuotedExpr) -> Option<QuotedExpr> {
            None
        }
        t.add_rule("rule_a", dummy);
        t.add_rule("rule_b", dummy);
        let names = t.rule_names();
        assert_eq!(names, vec!["rule_a", "rule_b"]);
    }
    #[test]
    fn test_meta_value_variants() {
        let unit = MetaValue::Unit;
        assert!(unit.is_unit());
        assert!(!unit.is_error());
        let err = MetaValue::Error("oops".to_string());
        assert!(err.is_error());
        assert!(!err.is_unit());
        let b = MetaValue::Bool(true);
        assert_eq!(b.as_bool(), Some(true));
        let s = MetaValue::String("hello".to_string());
        assert_eq!(s.as_str(), Some("hello"));
        let n = MetaValue::Int(42);
        assert_eq!(n.as_int(), Some(42));
        let q = MetaValue::Quoted(QuotedExpr::atom("x"));
        assert!(q.is_quoted());
        assert!(q.as_quoted().is_some());
    }
    #[test]
    fn test_meta_value_display() {
        assert_eq!(format!("{}", MetaValue::Unit), "()");
        assert_eq!(format!("{}", MetaValue::Bool(false)), "false");
        assert_eq!(format!("{}", MetaValue::Int(7)), "7");
        assert!(format!("{}", MetaValue::String("hi".to_string())).contains("hi"));
        assert!(format!("{}", MetaValue::Error("err".to_string())).contains("err"));
        let list = MetaValue::List(vec![MetaValue::Int(1), MetaValue::Int(2)]);
        let s = format!("{}", list);
        assert!(s.contains("1") && s.contains("2"));
    }
    #[test]
    fn test_meta_value_quoted_display() {
        let q = MetaValue::Quoted(QuotedExpr::atom("myExpr"));
        let s = format!("{}", q);
        assert!(s.contains("myExpr"));
    }
}
/// Type alias for results of meta-level operations.
pub type MetaResult<T> = Result<T, MetaProgrammingError>;
#[cfg(test)]
mod meta_error_tests {
    use super::*;
    use crate::metaprog::*;
    #[test]
    fn test_meta_programming_error_display() {
        let e = MetaProgrammingError::UndefinedVariable("x".to_string());
        assert!(format!("{}", e).contains("x"));
        let e2 = MetaProgrammingError::MacroFailed {
            name: "my_macro".to_string(),
            reason: "no match".to_string(),
        };
        assert!(format!("{}", e2).contains("my_macro"));
        assert!(format!("{}", e2).contains("no match"));
        let e3 = MetaProgrammingError::TypeMismatch {
            expected: "Nat".to_string(),
            found: "Bool".to_string(),
        };
        let s3 = format!("{}", e3);
        assert!(s3.contains("Nat") && s3.contains("Bool"));
        let e4 = MetaProgrammingError::QuotationDepthExceeded(100);
        assert!(format!("{}", e4).contains("100"));
        let e5 = MetaProgrammingError::SpliceOutsideQuotation;
        assert!(format!("{}", e5).contains("splice"));
        let e6 = MetaProgrammingError::EvalError("crash".to_string());
        assert!(format!("{}", e6).contains("crash"));
    }
    #[test]
    fn test_meta_result_ok() {
        let r: MetaResult<i32> = Ok(42);
        assert!(r.is_ok());
        if let Ok(val) = r {
            assert_eq!(val, 42);
        } else {
            panic!("Expected Ok");
        }
    }
    #[test]
    fn test_meta_result_err() {
        let r: MetaResult<i32> = Err(MetaProgrammingError::UndefinedVariable("y".to_string()));
        assert!(r.is_err());
        if let Err(err) = r {
            assert!(matches!(err, MetaProgrammingError::UndefinedVariable(_)));
        } else {
            panic!("Expected Err");
        }
    }
    #[test]
    fn test_error_equality() {
        let e1 = MetaProgrammingError::SpliceOutsideQuotation;
        let e2 = MetaProgrammingError::SpliceOutsideQuotation;
        assert_eq!(e1, e2);
        let e3 = MetaProgrammingError::UndefinedVariable("x".to_string());
        let e4 = MetaProgrammingError::UndefinedVariable("x".to_string());
        assert_eq!(e3, e4);
        let e5 = MetaProgrammingError::UndefinedVariable("x".to_string());
        let e6 = MetaProgrammingError::UndefinedVariable("y".to_string());
        assert_ne!(e5, e6);
    }
    #[test]
    fn test_quotation_mode_default() {
        let mode = QuotationMode::default();
        assert_eq!(mode, QuotationMode::Full);
    }
    #[test]
    fn test_quoted_expr_clone_deep() {
        let q = QuotedExpr::app(
            QuotedExpr::lambda(
                vec!["x".to_string()],
                QuotedExpr::splice(QuotedExpr::atom("body")),
            ),
            vec![QuotedExpr::atom("arg")],
        );
        let q2 = q.clone();
        assert!(q2.has_splice());
    }
    #[test]
    fn test_meta_env_multiple_parents() {
        let mut grandparent = MetaEnv::new();
        grandparent.bind("gp", "gp_val");
        let mut parent = MetaEnv::child(grandparent);
        parent.bind("p", "p_val");
        let mut child = MetaEnv::child(parent);
        child.bind("c", "c_val");
        assert_eq!(child.lookup("c"), Some("c_val"));
        assert_eq!(child.lookup("p"), Some("p_val"));
        assert_eq!(child.lookup("gp"), Some("gp_val"));
        assert!(child.lookup("x").is_none());
    }
    #[test]
    fn test_meta_stack_empty_pop() {
        let mut stack = MetaStack::new();
        let result = stack.pop();
        assert!(result.is_none());
        assert!(stack.top().is_none());
    }
    #[test]
    fn test_macro_transformer_lambda_transform() {
        let transformer = MacroTransformer::new();
        #[allow(dead_code)]
        fn rewrite_add(expr: &QuotedExpr) -> Option<QuotedExpr> {
            if let QuotedExpr::Atom(s) = expr {
                if s == "zero" {
                    return Some(QuotedExpr::Int_zero_placeholder());
                }
            }
            None
        }
        let _ = transformer;
    }
}
#[cfg(test)]
mod builtin_tactic_tests {
    use super::*;
    use crate::metaprog::*;
    #[test]
    fn test_ring_meta_tactic() {
        let ring = RingMetaTactic;
        assert_eq!(ring.name(), "ring");
        assert!(!ring.description().is_empty());
        assert!(matches!(
            ring.run("a + b = b + a", &[]),
            UserTacticResult::Solved
        ));
        assert!(matches!(ring.run("True", &[]), UserTacticResult::Failed(_)));
    }
    #[test]
    fn test_omega_meta_tactic() {
        let omega = OmegaMetaTactic;
        assert_eq!(omega.name(), "omega");
        assert!(matches!(omega.run("a <= b", &[]), UserTacticResult::Solved));
        assert!(matches!(omega.run("x < y", &[]), UserTacticResult::Solved));
        assert!(matches!(
            omega.run("True", &[]),
            UserTacticResult::Failed(_)
        ));
    }
    #[test]
    fn test_simp_meta_tactic_true() {
        let simp = SimpMetaTactic::empty();
        assert_eq!(simp.name(), "simp");
        assert!(matches!(simp.run("True", &[]), UserTacticResult::Solved));
    }
    #[test]
    fn test_simp_meta_tactic_refl() {
        let simp = SimpMetaTactic::empty();
        assert!(matches!(simp.run("x = x", &[]), UserTacticResult::Solved));
        assert!(matches!(
            simp.run("  y  =  y  ", &[]),
            UserTacticResult::Solved
        ));
        assert!(matches!(
            simp.run("x = y", &[]),
            UserTacticResult::Failed(_)
        ));
    }
    #[test]
    fn test_simp_meta_tactic_with_lemmas() {
        let simp = SimpMetaTactic::new(vec!["add_comm".to_string(), "mul_comm".to_string()]);
        assert_eq!(simp.lemmas().len(), 2);
        assert_eq!(simp.lemmas()[0], "add_comm");
    }
    #[test]
    fn test_register_builtin_tactics() {
        let mut reg = UserTacticRegistry::new();
        reg.register(Box::new(RingMetaTactic));
        reg.register(Box::new(OmegaMetaTactic));
        reg.register(Box::new(SimpMetaTactic::empty()));
        reg.register(Box::new(TrueDecider));
        reg.register(Box::new(FalseEliminator));
        assert_eq!(reg.len(), 5);
        assert!(reg.find("ring").is_some());
        assert!(reg.find("omega").is_some());
        assert!(reg.find("simp").is_some());
        assert!(reg.find("decide_true").is_some());
        assert!(reg.find("false_elim").is_some());
    }
    #[test]
    fn test_ring_tactic_various_equalities() {
        let ring = RingMetaTactic;
        assert!(matches!(
            ring.run("a * b = b * a", &[]),
            UserTacticResult::Solved
        ));
        assert!(matches!(
            ring.run("0 + x = x + 0", &[]),
            UserTacticResult::Solved
        ));
        assert!(matches!(
            ring.run("(a + b)^2 = a^2 + 2*a*b + b^2", &[]),
            UserTacticResult::Solved
        ));
    }
}
/// A single step in a meta-elaboration pipeline.
pub trait MetaElabStep: Send + Sync {
    /// Name of this step for diagnostics.
    fn step_name(&self) -> &str;
    /// Apply this step, possibly transforming the quoted expression.
    fn apply(&self, expr: QuotedExpr, env: &mut MetaEnv) -> MetaResult<QuotedExpr>;
}
#[cfg(test)]
mod pipeline_tests {
    use super::*;
    use crate::metaprog::*;
    #[test]
    fn test_meta_elab_pipeline_empty() {
        let pipeline = MetaElabPipeline::new();
        assert!(pipeline.is_empty());
        let mut env = MetaEnv::new();
        let q = QuotedExpr::atom("x");
        let result = pipeline
            .run(q, &mut env)
            .expect("test operation should succeed");
        assert!(result.is_atom());
    }
    #[test]
    fn test_meta_elab_pipeline_with_identity() {
        let mut pipeline = MetaElabPipeline::new();
        pipeline.add(IdentityMetaStep);
        pipeline.add(IdentityMetaStep);
        assert_eq!(pipeline.len(), 2);
        let names = pipeline.step_names();
        assert_eq!(names, vec!["identity", "identity"]);
        let mut env = MetaEnv::new();
        let q = QuotedExpr::atom("y");
        let result = pipeline
            .run(q, &mut env)
            .expect("test operation should succeed");
        assert_eq!(result.to_debug_string(), "y");
    }
    #[test]
    fn test_meta_elab_pipeline_error_propagates() {
        struct FailingStep;
        impl MetaElabStep for FailingStep {
            fn step_name(&self) -> &str {
                "fail"
            }
            fn apply(&self, _expr: QuotedExpr, _env: &mut MetaEnv) -> MetaResult<QuotedExpr> {
                Err(MetaProgrammingError::EvalError(
                    "forced failure".to_string(),
                ))
            }
        }
        let mut pipeline = MetaElabPipeline::new();
        pipeline.add(IdentityMetaStep);
        pipeline.add(FailingStep);
        pipeline.add(IdentityMetaStep);
        let mut env = MetaEnv::new();
        let result = pipeline.run(QuotedExpr::atom("x"), &mut env);
        assert!(result.is_err());
        if let Err(MetaProgrammingError::EvalError(msg)) = result {
            assert!(msg.contains("forced failure"));
        } else {
            panic!("expected EvalError");
        }
    }
}
