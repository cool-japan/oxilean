//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CoqBranch, CoqClass, CoqConstructor, CoqDecl, CoqField, CoqFixPoint, CoqInductive, CoqInstance,
    CoqModule, CoqProof, CoqRecord, CoqSort, CoqTactic, CoqTerm,
};

/// Emit a list of binders as `(x : T) (y : U)`.
pub(super) fn emit_binders(binders: &[(String, CoqTerm)], indent: usize) -> String {
    binders
        .iter()
        .map(|(x, ty)| format!("({} : {})", x, ty.emit(indent)))
        .collect::<Vec<_>>()
        .join(" ")
}
/// Escape special characters in a Coq string literal.
pub(super) fn escape_coq_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\"\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out
}
/// Build `CoqTerm::Var`.
pub fn var(name: impl Into<String>) -> CoqTerm {
    CoqTerm::Var(name.into())
}
/// Build `CoqTerm::App`.
pub fn app(func: CoqTerm, args: Vec<CoqTerm>) -> CoqTerm {
    CoqTerm::App(Box::new(func), args)
}
/// Build a unary application: `f x`.
pub fn app1(func: CoqTerm, arg: CoqTerm) -> CoqTerm {
    CoqTerm::App(Box::new(func), vec![arg])
}
/// Build `CoqTerm::Forall`.
pub fn forall(binders: Vec<(String, CoqTerm)>, body: CoqTerm) -> CoqTerm {
    CoqTerm::Forall(binders, Box::new(body))
}
/// Build `CoqTerm::Prod` (non-dependent arrow).
pub fn arrow(dom: CoqTerm, cod: CoqTerm) -> CoqTerm {
    CoqTerm::Prod(Box::new(dom), Box::new(cod))
}
/// Build `CoqTerm::Lambda`.
pub fn lambda(binders: Vec<(String, CoqTerm)>, body: CoqTerm) -> CoqTerm {
    CoqTerm::Lambda(binders, Box::new(body))
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn nat() -> CoqTerm {
        var("nat")
    }
    pub(super) fn bool_t() -> CoqTerm {
        var("bool")
    }
    pub(super) fn prop() -> CoqTerm {
        CoqTerm::Sort(CoqSort::Prop)
    }
    #[test]
    pub(super) fn test_sort_prop() {
        assert_eq!(CoqSort::Prop.to_string(), "Prop");
    }
    #[test]
    pub(super) fn test_sort_set() {
        assert_eq!(CoqSort::Set.to_string(), "Set");
    }
    #[test]
    pub(super) fn test_sort_type_univ() {
        assert_eq!(CoqSort::Type(None).to_string(), "Type");
    }
    #[test]
    pub(super) fn test_sort_type_indexed() {
        assert_eq!(CoqSort::Type(Some(2)).to_string(), "Type@{u2}");
    }
    #[test]
    pub(super) fn test_var() {
        assert_eq!(var("nat").emit(0), "nat");
    }
    #[test]
    pub(super) fn test_num() {
        assert_eq!(CoqTerm::Num(42).emit(0), "42");
        assert_eq!(CoqTerm::Num(-1).emit(0), "-1");
    }
    #[test]
    pub(super) fn test_str_literal() {
        assert_eq!(CoqTerm::Str("hello".into()).emit(0), "\"hello\"");
    }
    #[test]
    pub(super) fn test_str_escape() {
        assert_eq!(CoqTerm::Str("say \"hi\"".into()).emit(0), r#""say ""hi""""#);
    }
    #[test]
    pub(super) fn test_hole() {
        assert_eq!(CoqTerm::Hole.emit(0), "_");
    }
    #[test]
    pub(super) fn test_app_simple() {
        let t = app(var("S"), vec![var("n")]);
        assert_eq!(t.emit(0), "S n");
    }
    #[test]
    pub(super) fn test_app_nested() {
        let inner = app(var("plus"), vec![var("n"), var("m")]);
        let outer = app(var("S"), vec![inner]);
        assert_eq!(outer.emit(0), "S (plus n m)");
    }
    #[test]
    pub(super) fn test_arrow_type() {
        let t = arrow(nat(), nat());
        assert_eq!(t.emit(0), "nat -> nat");
    }
    #[test]
    pub(super) fn test_forall_type() {
        let t = forall(
            vec![("n".into(), nat()), ("m".into(), nat())],
            app(
                var("eq"),
                vec![
                    app(var("plus"), vec![var("n"), var("m")]),
                    app(var("plus"), vec![var("m"), var("n")]),
                ],
            ),
        );
        let s = t.emit(0);
        assert!(s.starts_with("forall (n : nat) (m : nat),"));
    }
    #[test]
    pub(super) fn test_lambda() {
        let t = lambda(vec![("n".into(), nat())], app(var("S"), vec![var("n")]));
        let s = t.emit(0);
        assert!(s.contains("fun (n : nat) =>"));
        assert!(s.contains("S n"));
    }
    #[test]
    pub(super) fn test_let_binding() {
        let t = CoqTerm::Let(
            "x".into(),
            Some(Box::new(nat())),
            Box::new(CoqTerm::Num(3)),
            Box::new(app(var("S"), vec![var("x")])),
        );
        let s = t.emit(0);
        assert!(s.contains("let x : nat := 3 in"));
    }
    #[test]
    pub(super) fn test_tuple() {
        let t = CoqTerm::Tuple(vec![CoqTerm::Num(1), CoqTerm::Num(2)]);
        assert_eq!(t.emit(0), "(1, 2)");
    }
    #[test]
    pub(super) fn test_list_literal() {
        let t = CoqTerm::List(vec![CoqTerm::Num(1), CoqTerm::Num(2), CoqTerm::Num(3)]);
        assert_eq!(t.emit(0), "[1; 2; 3]");
    }
    #[test]
    pub(super) fn test_if_then_else() {
        let t = CoqTerm::IfThenElse(
            Box::new(var("b")),
            Box::new(CoqTerm::Num(1)),
            Box::new(CoqTerm::Num(0)),
        );
        assert_eq!(t.emit(0), "if b then 1 else 0");
    }
    #[test]
    pub(super) fn test_match_expr() {
        let t = CoqTerm::Match(
            Box::new(var("n")),
            None,
            vec![
                CoqBranch {
                    constructor: "O".into(),
                    args: vec![],
                    body: CoqTerm::Num(0),
                },
                CoqBranch {
                    constructor: "S".into(),
                    args: vec!["n'".into()],
                    body: app(var("S"), vec![app(var("S"), vec![var("n'")])]),
                },
            ],
        );
        let s = t.emit(0);
        assert!(s.contains("| O => 0"));
        assert!(s.contains("| S n' =>"));
    }
    #[test]
    pub(super) fn test_tactic_intro() {
        let t = CoqTactic::Intro(vec!["n".into(), "m".into()]);
        assert_eq!(t.emit(0), "intro n m");
    }
    #[test]
    pub(super) fn test_tactic_apply() {
        let t = CoqTactic::Apply(var("plus_comm"));
        assert_eq!(t.emit(0), "apply plus_comm");
    }
    #[test]
    pub(super) fn test_tactic_rewrite_fwd() {
        let t = CoqTactic::Rewrite(false, var("H"));
        assert_eq!(t.emit(0), "rewrite -> H");
    }
    #[test]
    pub(super) fn test_tactic_rewrite_bwd() {
        let t = CoqTactic::Rewrite(true, var("H"));
        assert_eq!(t.emit(0), "rewrite <- H");
    }
    #[test]
    pub(super) fn test_tactic_induction() {
        let t = CoqTactic::Induction("n".into());
        assert_eq!(t.emit(0), "induction n");
    }
    #[test]
    pub(super) fn test_tactic_then() {
        let t = CoqTactic::Then(
            Box::new(CoqTactic::Induction("n".into())),
            Box::new(CoqTactic::Auto),
        );
        assert_eq!(t.emit(0), "induction n; auto");
    }
    #[test]
    pub(super) fn test_tactic_exists() {
        let t = CoqTactic::Exists(CoqTerm::Num(42));
        assert_eq!(t.emit(0), "exists 42");
    }
    #[test]
    pub(super) fn test_tactic_specialize() {
        let t = CoqTactic::Specialize(var("H"), vec![var("n")]);
        assert_eq!(t.emit(0), "specialize (H n)");
    }
    #[test]
    pub(super) fn test_proof_reflexivity() {
        let p = CoqProof::new(vec![CoqTactic::Reflexivity]);
        let s = p.emit(0);
        assert!(s.starts_with("Proof."));
        assert!(s.contains("reflexivity."));
        assert!(s.ends_with("Qed."));
    }
    #[test]
    pub(super) fn test_proof_admitted() {
        let p = CoqProof::admitted();
        assert!(p.emit(0).ends_with("Admitted."));
    }
    #[test]
    pub(super) fn test_decl_definition() {
        let d = CoqDecl::Definition {
            name: "double".into(),
            params: vec![("n".into(), nat())],
            ty: Some(nat()),
            body: app(var("plus"), vec![var("n"), var("n")]),
        };
        let s = d.emit();
        assert!(s.starts_with("Definition double"));
        assert!(s.contains(":= plus n n."));
    }
    #[test]
    pub(super) fn test_decl_axiom() {
        let d = CoqDecl::Axiom {
            name: "classic".into(),
            ty: forall(
                vec![("P".into(), prop())],
                app(var("or"), vec![var("P"), app(var("not"), vec![var("P")])]),
            ),
        };
        let s = d.emit();
        assert!(s.starts_with("Axiom classic :"));
    }
    #[test]
    pub(super) fn test_decl_inductive_nat() {
        let d = CoqDecl::Inductive(CoqInductive {
            name: "MyNat".into(),
            params: vec![],
            sort: CoqSort::Set,
            constructors: vec![
                CoqConstructor {
                    name: "MyO".into(),
                    ty: var("MyNat"),
                },
                CoqConstructor {
                    name: "MyS".into(),
                    ty: arrow(var("MyNat"), var("MyNat")),
                },
            ],
        });
        let s = d.emit();
        assert!(s.contains("Inductive MyNat : Set :="));
        assert!(s.contains("| MyO : MyNat"));
        assert!(s.contains("| MyS : MyNat -> MyNat"));
    }
    #[test]
    pub(super) fn test_decl_fixpoint() {
        let d = CoqDecl::Fixpoint(vec![CoqFixPoint {
            name: "add".into(),
            params: vec![("n".into(), nat()), ("m".into(), nat())],
            return_type: Some(nat()),
            struct_arg: Some("n".into()),
            body: CoqTerm::Match(
                Box::new(var("n")),
                None,
                vec![
                    CoqBranch {
                        constructor: "O".into(),
                        args: vec![],
                        body: var("m"),
                    },
                    CoqBranch {
                        constructor: "S".into(),
                        args: vec!["n'".into()],
                        body: app(var("S"), vec![app(var("add"), vec![var("n'"), var("m")])]),
                    },
                ],
            ),
        }]);
        let s = d.emit();
        assert!(s.starts_with("Fixpoint add"));
        assert!(s.contains("{struct n}"));
    }
    #[test]
    pub(super) fn test_decl_record() {
        let d = CoqDecl::RecordDecl(CoqRecord {
            name: "Point".into(),
            params: vec![],
            sort: CoqSort::Set,
            constructor: Some("mkPoint".into()),
            fields: vec![
                CoqField {
                    name: "x".into(),
                    ty: var("R"),
                },
                CoqField {
                    name: "y".into(),
                    ty: var("R"),
                },
            ],
        });
        let s = d.emit();
        assert!(s.contains("Record Point"));
        assert!(s.contains("x : R;"));
        assert!(s.contains("y : R;"));
    }
    #[test]
    pub(super) fn test_decl_class() {
        let d = CoqDecl::ClassDecl(CoqClass {
            name: "Eq".into(),
            params: vec![("A".into(), CoqTerm::Sort(CoqSort::Type(None)))],
            methods: vec![CoqField {
                name: "eqb".into(),
                ty: arrow(var("A"), arrow(var("A"), bool_t())),
            }],
        });
        let s = d.emit();
        assert!(s.contains("Class Eq"));
        assert!(s.contains("eqb : A -> A -> bool;"));
    }
    #[test]
    pub(super) fn test_decl_instance() {
        let d = CoqDecl::Instance(CoqInstance {
            name: "NatEq".into(),
            class: app1(var("Eq"), nat()),
            methods: vec![("eqb".into(), var("Nat.eqb"))],
        });
        let s = d.emit();
        assert!(s.contains("Instance NatEq : Eq nat"));
        assert!(s.contains("eqb := Nat.eqb;"));
    }
    #[test]
    pub(super) fn test_decl_theorem() {
        let d = CoqDecl::Theorem {
            name: "plus_O_n".into(),
            params: vec![],
            ty: forall(
                vec![("n".into(), nat())],
                app(
                    var("eq"),
                    vec![app(var("plus"), vec![CoqTerm::Num(0), var("n")]), var("n")],
                ),
            ),
            proof: CoqProof::new(vec![
                CoqTactic::Intro(vec!["n".into()]),
                CoqTactic::Simpl,
                CoqTactic::Reflexivity,
            ]),
        };
        let s = d.emit();
        assert!(s.starts_with("Theorem plus_O_n"));
        assert!(s.contains("intro n."));
        assert!(s.contains("Qed."));
    }
    #[test]
    pub(super) fn test_module_emit() {
        let mut m = CoqModule::new("Example");
        m.require("Coq.Arith.Arith");
        m.open_scope("nat_scope");
        m.add(CoqDecl::Comment("simple definition".into()));
        m.add(CoqDecl::Definition {
            name: "one".into(),
            params: vec![],
            ty: Some(nat()),
            body: app(var("S"), vec![var("O")]),
        });
        let s = m.emit();
        assert!(s.contains("Require Import Coq.Arith.Arith."));
        assert!(s.contains("Open Scope nat_scope."));
        assert!(s.contains("(* simple definition *)"));
        assert!(s.contains("Definition one"));
    }
    #[test]
    pub(super) fn test_module_full_example() {
        let mut m = CoqModule::new("ListExample");
        m.require("Coq.Lists.List");
        m.open_scope("list_scope");
        m.add(CoqDecl::Fixpoint(vec![CoqFixPoint {
            name: "my_length".into(),
            params: vec![
                ("A".into(), CoqTerm::Sort(CoqSort::Type(None))),
                ("l".into(), app1(var("list"), var("A"))),
            ],
            return_type: Some(nat()),
            struct_arg: Some("l".into()),
            body: CoqTerm::Match(
                Box::new(var("l")),
                None,
                vec![
                    CoqBranch {
                        constructor: "nil".into(),
                        args: vec![],
                        body: CoqTerm::Num(0),
                    },
                    CoqBranch {
                        constructor: "cons".into(),
                        args: vec!["_".into(), "t".into()],
                        body: app(
                            var("S"),
                            vec![app(var("my_length"), vec![var("A"), var("t")])],
                        ),
                    },
                ],
            ),
        }]));
        let s = m.emit();
        assert!(s.contains("Fixpoint my_length"));
        assert!(s.contains("{struct l}"));
        assert!(s.contains("| nil => 0"));
    }
}
/// Coq final diagnostics helper
#[allow(dead_code)]
pub fn coq_emit_comment(text: &str) -> String {
    format!("(* {} *)", text)
}
#[allow(dead_code)]
pub fn coq_emit_section_separator(title: &str) -> String {
    format!("(* ==================== {} ==================== *)", title)
}
#[allow(dead_code)]
pub fn coq_emit_requires(mods: &[&str]) -> String {
    format!("Require Import {}.", mods.join(" "))
}
