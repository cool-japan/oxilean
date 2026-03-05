//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::prologgoalbuilder_type::PrologGoalBuilder;
use super::types::{
    DcgRhs, DcgRule, PrologArith, PrologAssertionBuilder, PrologBackend, PrologClause,
    PrologClauseBuilder, PrologConstraints, PrologDCGBuilder, PrologDirective,
    PrologMetaPredicates, PrologMode, PrologModule, PrologModuleBuilder, PrologPredicate,
    PrologPredicateBuilder, PrologSnippets, PrologTerm, PrologType, PrologTypeSig,
};
use std::fmt;

pub(super) fn fmt_dcg_seq(f: &mut fmt::Formatter<'_>, parts: &[DcgRhs]) -> fmt::Result {
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        write!(f, "{}", p)?;
    }
    Ok(())
}
/// `atom(s)` — shorthand for `PrologTerm::Atom`.
pub fn atom(s: impl Into<String>) -> PrologTerm {
    PrologTerm::Atom(s.into())
}
/// `var(s)` — shorthand for `PrologTerm::Variable`.
pub fn var(s: impl Into<String>) -> PrologTerm {
    PrologTerm::Variable(s.into())
}
/// `int(n)` — shorthand for `PrologTerm::Integer`.
pub fn int(n: i64) -> PrologTerm {
    PrologTerm::Integer(n)
}
/// `float(x)` — shorthand for `PrologTerm::Float`.
pub fn float_term(x: f64) -> PrologTerm {
    PrologTerm::Float(x)
}
/// `compound(f, args)` — shorthand for `PrologTerm::Compound`.
pub fn compound(functor: impl Into<String>, args: Vec<PrologTerm>) -> PrologTerm {
    PrologTerm::Compound(functor.into(), args)
}
/// `list(elems)` — shorthand for a proper list.
pub fn list(elems: Vec<PrologTerm>) -> PrologTerm {
    PrologTerm::List(elems, None)
}
/// `op_term(op, l, r)` — shorthand for `PrologTerm::Op`.
pub fn op_term(op: impl Into<String>, l: PrologTerm, r: PrologTerm) -> PrologTerm {
    PrologTerm::Op(op.into(), Box::new(l), Box::new(r))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_atom_display() {
        assert_eq!(format!("{}", atom("foo")), "foo");
        assert_eq!(format!("{}", atom("Hello")), "'Hello'");
        assert_eq!(format!("{}", atom("hello world")), "'hello world'");
        assert_eq!(format!("{}", PrologTerm::Nil), "[]");
        assert_eq!(format!("{}", PrologTerm::Cut), "!");
        assert_eq!(format!("{}", PrologTerm::Anon), "_");
    }
    #[test]
    pub(super) fn test_integer_float_display() {
        assert_eq!(format!("{}", int(42)), "42");
        assert_eq!(format!("{}", int(-7)), "-7");
        assert_eq!(format!("{}", float_term(3.14)), "3.14");
        assert_eq!(format!("{}", float_term(1.0)), "1.0");
        let ft = PrologTerm::Float(1.0);
        let s = format!("{}", ft);
        assert!(s == "1.0" || s == "1", "got: {}", s);
    }
    #[test]
    pub(super) fn test_compound_display() {
        let t = compound("f", vec![atom("a"), var("X"), int(1)]);
        assert_eq!(format!("{}", t), "f(a, X, 1)");
    }
    #[test]
    pub(super) fn test_list_display() {
        let t = list(vec![int(1), int(2), int(3)]);
        assert_eq!(format!("{}", t), "[1, 2, 3]");
        let partial = PrologTerm::list_partial(vec![var("H")], var("T"));
        assert_eq!(format!("{}", partial), "[H|T]");
        assert_eq!(format!("{}", PrologTerm::Nil), "[]");
    }
    #[test]
    pub(super) fn test_op_display() {
        let t = op_term("is", var("X"), op_term("+", var("Y"), int(1)));
        let s = format!("{}", t);
        assert!(s.contains("is"), "got: {}", s);
        assert!(s.contains('+'), "got: {}", s);
    }
    #[test]
    pub(super) fn test_fact_emit() {
        let f = PrologClause::fact(compound(
            "member",
            vec![var("X"), list(vec![var("X"), var("_")])],
        ));
        let s = f.emit();
        assert!(s.ends_with('.'));
        assert!(s.contains("member"));
        assert!(!s.contains(":-"));
    }
    #[test]
    pub(super) fn test_rule_emit() {
        let head = compound(
            "member",
            vec![var("X"), compound(".", vec![var("_"), var("T")])],
        );
        let body = vec![compound("member", vec![var("X"), var("T")])];
        let r = PrologClause::rule(head, body);
        let s = r.emit();
        assert!(s.contains(":-"), "got: {}", s);
        assert!(s.ends_with('.'));
        assert!(s.contains("member(X, T)"));
    }
    #[test]
    pub(super) fn test_rule_multigoal_emit() {
        let head = compound(
            "append",
            vec![
                compound(".", vec![var("H"), var("T")]),
                var("L"),
                compound(".", vec![var("H"), var("R")]),
            ],
        );
        let body = vec![compound("append", vec![var("T"), var("L"), var("R")])];
        let r = PrologClause::rule(head, body);
        let s = r.emit();
        assert!(s.contains(":-"));
        assert!(s.ends_with('.'));
    }
    #[test]
    pub(super) fn test_predicate_emit() {
        let mut pred = PrologPredicate::new("length", 2).dynamic().exported();
        pred.add_clause(PrologClause::fact(compound(
            "length",
            vec![PrologTerm::Nil, int(0)],
        )));
        pred.add_clause(PrologClause::rule(
            compound(
                "length",
                vec![compound(".", vec![var("_"), var("T")]), var("N")],
            ),
            vec![
                compound("length", vec![var("T"), var("N1")]),
                op_term("is", var("N"), op_term("+", var("N1"), int(1))),
            ],
        ));
        let s = pred.emit();
        assert!(s.contains(":- dynamic length/2."));
        assert!(s.contains("length([], 0)."));
        assert!(s.contains(":-"));
        assert_eq!(pred.indicator(), "length/2");
    }
    #[test]
    pub(super) fn test_directive_emit() {
        let d = PrologDirective::Module("lists".into(), vec!["member/2".into(), "append/3".into()]);
        let s = d.emit();
        assert!(s.starts_with(":- module(lists,"));
        assert!(s.contains("member/2"));
        assert!(s.contains("append/3"));
        let d2 = PrologDirective::UseModuleLibrary("lists".into());
        assert_eq!(d2.emit(), ":- use_module(library(lists)).");
        let d3 = PrologDirective::Dynamic("fact".into(), 1);
        assert_eq!(d3.emit(), ":- dynamic fact/1.");
        let d4 = PrologDirective::Op(700, "xfx".into(), "===".into());
        assert_eq!(d4.emit(), ":- op(700, xfx, ===).");
    }
    #[test]
    pub(super) fn test_dcg_rule_emit() {
        let rule = DcgRule {
            lhs: compound("sentence", vec![var("S")]),
            rhs: vec![
                DcgRhs::NonTerminal(compound("np", vec![var("S")])),
                DcgRhs::NonTerminal(atom("vp")),
            ],
            guards: vec![],
            comment: None,
        };
        let s = rule.emit();
        assert!(s.contains("-->"), "got: {}", s);
        assert!(s.contains("np(S)"), "got: {}", s);
        assert!(s.contains("vp"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_dcg_terminals_emit() {
        let rule = DcgRule {
            lhs: atom("greeting"),
            rhs: vec![
                DcgRhs::Terminals(vec![atom("hello")]),
                DcgRhs::NonTerminal(atom("name")),
            ],
            guards: vec![],
            comment: Some("Match a greeting phrase".into()),
        };
        let s = rule.emit();
        assert!(s.starts_with("% Match a greeting phrase"));
        assert!(s.contains("[hello]"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_module_emit() {
        let backend = PrologBackend::swi();
        let mut module = PrologModule::new("mylist");
        module.export("member/2");
        module.export("append/3");
        module.directive(PrologDirective::UseModuleLibrary("lists".into()));
        module.blank();
        let mut member_pred = PrologPredicate::new("member", 2).exported();
        member_pred.add_clause(PrologClause::fact(compound(
            "member",
            vec![var("X"), list(vec![var("X"), var("_")])],
        )));
        member_pred.add_clause(PrologClause::rule(
            compound(
                "member",
                vec![var("X"), compound(".", vec![var("_"), var("T")])],
            ),
            vec![compound("member", vec![var("X"), var("T")])],
        ));
        module.predicate(member_pred);
        let s = backend.emit_module(&module);
        assert!(s.contains(":- module(mylist,"), "got: {}", s);
        assert!(s.contains("member/2"), "got: {}", s);
        assert!(s.contains("append/3"), "got: {}", s);
        assert!(s.contains(":- use_module(library(lists))."), "got: {}", s);
        assert!(s.contains("member(X, [X, _])."), "got: {}", s);
    }
    #[test]
    pub(super) fn test_swi_preamble() {
        let backend = PrologBackend::swi();
        let preamble = backend.build_swi_preamble(
            "utils",
            &[("helper", 1), ("transform", 2)],
            &["lists", "aggregate"],
        );
        assert!(preamble.contains(":- module(utils,"));
        assert!(preamble.contains("helper/1"));
        assert!(preamble.contains("transform/2"));
        assert!(preamble.contains(":- use_module(library(lists))."));
        assert!(preamble.contains(":- use_module(library(aggregate))."));
    }
}
/// Negation-as-failure term: `\+(Goal)`.
#[allow(dead_code)]
pub fn not_provable(goal: PrologTerm) -> PrologTerm {
    PrologTerm::PrefixOp("\\+".to_string(), Box::new(goal))
}
/// Conditional term: `(Cond -> Then ; Else)`.
#[allow(dead_code)]
pub fn if_then_else(cond: PrologTerm, then_: PrologTerm, else_: PrologTerm) -> PrologTerm {
    PrologTerm::Op(
        ";".to_string(),
        Box::new(PrologTerm::Op(
            "->".to_string(),
            Box::new(cond),
            Box::new(then_),
        )),
        Box::new(else_),
    )
}
/// Build `(A, B)` — conjunction term.
#[allow(dead_code)]
pub fn conjunction(a: PrologTerm, b: PrologTerm) -> PrologTerm {
    PrologTerm::Op(",".to_string(), Box::new(a), Box::new(b))
}
/// Build `(A ; B)` — disjunction term.
#[allow(dead_code)]
pub fn disjunction(a: PrologTerm, b: PrologTerm) -> PrologTerm {
    PrologTerm::Op(";".to_string(), Box::new(a), Box::new(b))
}
/// Build `X = Y` — unification goal.
#[allow(dead_code)]
pub fn unify(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("=".to_string(), Box::new(x), Box::new(y))
}
/// Build `X \= Y` — non-unification goal.
#[allow(dead_code)]
pub fn not_unify(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("\\=".to_string(), Box::new(x), Box::new(y))
}
/// Build `X == Y` — strict equality.
#[allow(dead_code)]
pub fn strict_eq(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("==".to_string(), Box::new(x), Box::new(y))
}
/// Build `X \== Y` — strict inequality.
#[allow(dead_code)]
pub fn strict_neq(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("\\==".to_string(), Box::new(x), Box::new(y))
}
/// Build `X is Expr` — arithmetic evaluation.
#[allow(dead_code)]
pub fn is_eval(x: PrologTerm, expr: PrologTerm) -> PrologTerm {
    PrologTerm::Op("is".to_string(), Box::new(x), Box::new(expr))
}
/// Build `X =:= Y` — arithmetic equality.
#[allow(dead_code)]
pub fn arith_eq(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("=:=".to_string(), Box::new(x), Box::new(y))
}
/// Build `X =\= Y` — arithmetic inequality.
#[allow(dead_code)]
pub fn arith_neq(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("=\\=".to_string(), Box::new(x), Box::new(y))
}
/// Build `X < Y`.
#[allow(dead_code)]
pub fn arith_lt(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("<".to_string(), Box::new(x), Box::new(y))
}
/// Build `X > Y`.
#[allow(dead_code)]
pub fn arith_gt(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op(">".to_string(), Box::new(x), Box::new(y))
}
/// Build `X @< Y` — term order less-than.
#[allow(dead_code)]
pub fn term_lt(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("@<".to_string(), Box::new(x), Box::new(y))
}
/// Build `X @> Y` — term order greater-than.
#[allow(dead_code)]
pub fn term_gt(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("@>".to_string(), Box::new(x), Box::new(y))
}
/// Build `X + Y`.
#[allow(dead_code)]
pub fn arith_add(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("+".to_string(), Box::new(x), Box::new(y))
}
/// Build `X - Y`.
#[allow(dead_code)]
pub fn arith_sub(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("-".to_string(), Box::new(x), Box::new(y))
}
/// Build `X * Y`.
#[allow(dead_code)]
pub fn arith_mul(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("*".to_string(), Box::new(x), Box::new(y))
}
/// Build `X mod Y`.
#[allow(dead_code)]
pub fn arith_mod(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("mod".to_string(), Box::new(x), Box::new(y))
}
/// Build `X div Y` — integer division.
#[allow(dead_code)]
pub fn arith_div(x: PrologTerm, y: PrologTerm) -> PrologTerm {
    PrologTerm::Op("div".to_string(), Box::new(x), Box::new(y))
}
#[cfg(test)]
mod extended_prolog_tests {
    use super::*;
    #[test]
    pub(super) fn test_not_provable() {
        let t = not_provable(atom("foo"));
        let s = format!("{}", t);
        assert!(s.contains("\\+"));
        assert!(s.contains("foo"));
    }
    #[test]
    pub(super) fn test_conjunction() {
        let t = conjunction(atom("a"), atom("b"));
        let s = format!("{}", t);
        assert!(s.contains("a"));
        assert!(s.contains("b"));
        assert!(s.contains(","));
    }
    #[test]
    pub(super) fn test_disjunction() {
        let t = disjunction(atom("a"), atom("b"));
        let s = format!("{}", t);
        assert!(s.contains(";"));
    }
    #[test]
    pub(super) fn test_unify_ops() {
        let t = unify(var("X"), int(5));
        let s = format!("{}", t);
        assert!(s.contains('='));
        let t2 = not_unify(var("X"), var("Y"));
        assert!(format!("{}", t2).contains("\\="));
    }
    #[test]
    pub(super) fn test_strict_eq_neq() {
        let t = strict_eq(var("X"), var("Y"));
        assert!(format!("{}", t).contains("=="));
        let t2 = strict_neq(var("X"), var("Y"));
        assert!(format!("{}", t2).contains("\\=="));
    }
    #[test]
    pub(super) fn test_arith_ops() {
        let t = is_eval(var("X"), arith_add(var("Y"), int(1)));
        let s = format!("{}", t);
        assert!(s.contains("is"));
        assert!(s.contains('+'));
        let sub = arith_sub(var("A"), var("B"));
        assert!(format!("{}", sub).contains('-'));
        let mul = arith_mul(var("A"), var("B"));
        assert!(format!("{}", mul).contains('*'));
        let md = arith_mod(var("A"), int(2));
        assert!(format!("{}", md).contains("mod"));
        let dv = arith_div(var("A"), int(2));
        assert!(format!("{}", dv).contains("div"));
    }
    #[test]
    pub(super) fn test_if_then_else() {
        let t = if_then_else(atom("cond"), atom("then"), atom("els"));
        let s = format!("{}", t);
        assert!(s.contains("->") || s.contains(';'));
    }
    #[test]
    pub(super) fn test_goal_builder() {
        let goals = PrologGoalBuilder::new()
            .is(var("N"), arith_add(var("M"), int(1)))
            .writeln(var("N"))
            .cut()
            .build();
        assert_eq!(goals.len(), 3);
        let clause = PrologGoalBuilder::new()
            .member(var("X"), var("List"))
            .to_clause(compound("find_member", vec![var("X"), var("List")]));
        assert!(!clause.body.is_empty());
    }
    #[test]
    pub(super) fn test_goal_builder_list_ops() {
        let goals = PrologGoalBuilder::new()
            .length(var("L"), var("N"))
            .append(var("A"), var("B"), var("C"))
            .reverse(var("L"), var("R"))
            .sort(var("L"), var("S"))
            .msort(var("L"), var("M"))
            .build();
        assert_eq!(goals.len(), 5);
    }
    #[test]
    pub(super) fn test_goal_builder_meta() {
        let goals = PrologGoalBuilder::new()
            .maplist1(atom("atom"), var("List"))
            .maplist2(atom("succ"), var("L"), var("R"))
            .include(atom("integer"), var("L"), var("Ints"))
            .exclude(atom("integer"), var("L"), var("NonInts"))
            .build();
        assert_eq!(goals.len(), 4);
    }
    #[test]
    pub(super) fn test_arith_builder() {
        let a = PrologArith::add(var("X"), int(1));
        assert!(format!("{}", a).contains('+'));
        let b = PrologArith::abs(var("X"));
        assert!(format!("{}", b).contains("abs"));
        let m = PrologArith::max(int(3), int(5));
        assert!(format!("{}", m).contains("max"));
        let pw = PrologArith::pow(var("X"), int(2));
        assert!(format!("{}", pw).contains('^'));
        let ba = PrologArith::bitand(var("X"), var("Y"));
        assert!(format!("{}", ba).contains("/\\"));
        let bo = PrologArith::bitor(var("X"), var("Y"));
        assert!(format!("{}", bo).contains("\\/"));
    }
    #[test]
    pub(super) fn test_prolog_type_display() {
        assert_eq!(format!("{}", PrologType::Integer), "integer");
        assert_eq!(format!("{}", PrologType::Atom), "atom");
        assert_eq!(format!("{}", PrologType::Boolean), "boolean");
        let list_ty = PrologType::List(Box::new(PrologType::Integer));
        assert!(format!("{}", list_ty).contains("list(integer)"));
    }
    #[test]
    pub(super) fn test_prolog_mode_display() {
        assert_eq!(format!("{}", PrologMode::In), "+");
        assert_eq!(format!("{}", PrologMode::Out), "-");
        assert_eq!(format!("{}", PrologMode::InOut), "?");
        assert_eq!(format!("{}", PrologMode::Meta), ":");
    }
    #[test]
    pub(super) fn test_prolog_type_sig_pldoc() {
        let sig = PrologTypeSig {
            name: "append".to_string(),
            params: vec![
                (PrologMode::In, PrologType::List(Box::new(PrologType::Term))),
                (PrologMode::In, PrologType::List(Box::new(PrologType::Term))),
                (
                    PrologMode::Out,
                    PrologType::List(Box::new(PrologType::Term)),
                ),
            ],
            description: Some("Append two lists.".to_string()),
        };
        let s = sig.emit_pldoc();
        assert!(s.contains("%% append/3"));
        assert!(s.contains("list"));
        let s2 = sig.emit_pred_directive();
        assert!(s2.contains(":- pred"));
    }
    #[test]
    pub(super) fn test_meta_predicates() {
        let t = PrologMetaPredicates::maplist(atom("write"), var("L"));
        assert!(format!("{}", t).contains("maplist"));
        let t2 = PrologMetaPredicates::findall(var("X"), atom("goal"), var("Bag"));
        assert!(format!("{}", t2).contains("findall"));
        let t3 = PrologMetaPredicates::once(atom("goal"));
        assert!(format!("{}", t3).contains("once"));
        let t4 = PrologMetaPredicates::forall(atom("cond"), atom("act"));
        assert!(format!("{}", t4).contains("forall"));
    }
    #[test]
    pub(super) fn test_assertion_builder() {
        let t = PrologAssertionBuilder::assertz_fact(compound("fact", vec![int(1)]));
        assert!(format!("{}", t).contains("assertz"));
        let t2 = PrologAssertionBuilder::retractall(compound("fact", vec![var("_")]));
        assert!(format!("{}", t2).contains("retractall"));
        let t3 = PrologAssertionBuilder::abolish("fact", 1);
        assert!(format!("{}", t3).contains("abolish"));
    }
    #[test]
    pub(super) fn test_constraints() {
        let c1 = PrologConstraints::clp_eq(var("X"), int(5));
        assert!(format!("{}", c1).contains("#="));
        let c2 = PrologConstraints::clp_lt(var("X"), int(10));
        assert!(format!("{}", c2).contains("#<"));
        let c3 = PrologConstraints::all_different(vec![var("X"), var("Y"), var("Z")]);
        assert!(format!("{}", c3).contains("all_different"));
        let c4 = PrologConstraints::label(vec![var("X")]);
        assert!(format!("{}", c4).contains("label"));
    }
    #[test]
    pub(super) fn test_snippets_member() {
        let pred = PrologSnippets::member_predicate();
        let s = pred.emit();
        assert!(s.contains("member"));
        assert_eq!(pred.arity, 2);
    }
    #[test]
    pub(super) fn test_snippets_append() {
        let pred = PrologSnippets::append_predicate();
        let s = pred.emit();
        assert!(s.contains("append"));
        assert_eq!(pred.arity, 3);
    }
    #[test]
    pub(super) fn test_snippets_length() {
        let pred = PrologSnippets::length_predicate();
        let s = pred.emit();
        assert!(s.contains("my_length"));
    }
    #[test]
    pub(super) fn test_snippets_max_list() {
        let pred = PrologSnippets::max_list_predicate();
        let s = pred.emit();
        assert!(s.contains("max_list"));
    }
    #[test]
    pub(super) fn test_snippets_sum_list() {
        let pred = PrologSnippets::sum_list_predicate();
        let s = pred.emit();
        assert!(s.contains("sum_list"));
    }
    #[test]
    pub(super) fn test_snippets_last() {
        let pred = PrologSnippets::last_predicate();
        let s = pred.emit();
        assert!(s.contains("my_last"));
    }
    #[test]
    pub(super) fn test_module_builder() {
        let s = PrologModuleBuilder::new("utils")
            .export("member/2")
            .use_library("lists")
            .add_predicate(PrologSnippets::member_predicate())
            .blank()
            .section("List utilities")
            .comment("End of module")
            .emit();
        assert!(s.contains(":- module(utils,"));
        assert!(s.contains("member/2"));
        assert!(s.contains(":- use_module(library(lists))."));
    }
    #[test]
    pub(super) fn test_clause_builder_fact() {
        let clause = PrologClauseBuilder::head(compound("hello", vec![atom("world")]))
            .comment("A greeting fact")
            .build();
        let s = clause.emit();
        assert!(s.contains("hello"));
        assert!(s.ends_with('.'));
        assert!(s.contains("A greeting fact"));
    }
    #[test]
    pub(super) fn test_clause_builder_rule() {
        let clause = PrologClauseBuilder::head(compound("greet", vec![var("X")]))
            .goal(compound(
                "format",
                vec![atom("Hello ~w!~n"), PrologTerm::list(vec![var("X")])],
            ))
            .build();
        let s = clause.emit();
        assert!(s.contains("greet(X) :-"));
        assert!(s.ends_with('.'));
    }
    #[test]
    pub(super) fn test_predicate_builder() {
        let s = PrologPredicateBuilder::new("count", 2)
            .dynamic()
            .exported()
            .doc("Count occurrences.")
            .fact(compound("count", vec![PrologTerm::Nil, int(0)]))
            .rule(
                compound("count", vec![var("L"), var("N")]),
                vec![compound("length", vec![var("L"), var("N")])],
            )
            .emit();
        assert!(s.contains(":- dynamic count/2."));
        assert!(s.contains("count([], 0)."));
    }
    #[test]
    pub(super) fn test_dcg_builder() {
        let s = PrologDCGBuilder::lhs(atom("sentence"))
            .nonterminal(atom("np"))
            .nonterminal(atom("vp"))
            .comment("A simple sentence rule")
            .emit();
        assert!(s.contains("-->"));
        assert!(s.contains("np"));
        assert!(s.contains("vp"));
        assert!(s.contains("A simple sentence rule"));
    }
    #[test]
    pub(super) fn test_dcg_builder_terminals() {
        let s = PrologDCGBuilder::lhs(atom("greeting"))
            .terminals(vec![atom("hello")])
            .nonterminal(atom("name"))
            .emit();
        assert!(s.contains("[hello]"));
        assert!(s.contains("name"));
    }
    #[test]
    pub(super) fn test_dcg_builder_with_guard() {
        let s = PrologDCGBuilder::lhs(compound("number", vec![var("N")]))
            .nonterminal(compound("digit", vec![var("D")]))
            .guard(is_eval(var("N"), var("D")))
            .emit();
        assert!(s.contains("digit"));
        assert!(s.contains('{'));
    }
    #[test]
    pub(super) fn test_nth0_predicate() {
        let pred = PrologSnippets::nth0_predicate();
        let s = pred.emit();
        assert!(s.contains("my_nth0"));
        assert_eq!(pred.arity, 3);
    }
    #[test]
    pub(super) fn test_flatten_predicate() {
        let pred = PrologSnippets::flatten_predicate();
        let s = pred.emit();
        assert!(s.contains("my_flatten"));
        assert!(s.contains("is_list"));
    }
    #[test]
    pub(super) fn test_term_order_ops() {
        let lt = term_lt(var("X"), var("Y"));
        assert!(format!("{}", lt).contains("@<"));
        let gt = term_gt(var("X"), var("Y"));
        assert!(format!("{}", gt).contains("@>"));
    }
    #[test]
    pub(super) fn test_arith_eq_neq() {
        let eq = arith_eq(var("X"), var("Y"));
        assert!(format!("{}", eq).contains("=:="));
        let neq = arith_neq(var("X"), var("Y"));
        assert!(format!("{}", neq).contains("=\\="));
    }
    #[test]
    pub(super) fn test_arith_lt_gt() {
        let lt = arith_lt(var("X"), int(10));
        assert!(format!("{}", lt).contains('<'));
        let gt = arith_gt(var("X"), int(0));
        assert!(format!("{}", gt).contains('>'));
    }
}
