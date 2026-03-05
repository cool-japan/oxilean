//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    IdrisBackend, IdrisConstructor, IdrisData, IdrisDecl, IdrisExpr, IdrisFunction, IdrisImport,
    IdrisInterface, IdrisLiteral, IdrisModule, IdrisPattern, IdrisRecord, IdrisType, Visibility,
};

/// `ty_var(s)` — a type variable.
pub fn ty_var(s: impl Into<String>) -> IdrisType {
    IdrisType::Var(s.into())
}
/// `ty_data(name, args)` — a named data type application.
pub fn ty_data(name: impl Into<String>, args: Vec<IdrisType>) -> IdrisType {
    IdrisType::Data(name.into(), args)
}
/// `ty_fn(a, b)` — a function type `a -> b`.
pub fn ty_fn(a: IdrisType, b: IdrisType) -> IdrisType {
    IdrisType::Function(Box::new(a), Box::new(b))
}
/// `ty_pi(x, a, b)` — a dependent function type `(x : a) -> b`.
pub fn ty_pi(x: impl Into<String>, a: IdrisType, b: IdrisType) -> IdrisType {
    IdrisType::Pi(x.into(), Box::new(a), Box::new(b))
}
/// `expr_var(s)` — a variable expression.
pub fn expr_var(s: impl Into<String>) -> IdrisExpr {
    IdrisExpr::Var(s.into())
}
/// `expr_app(f, x)` — function application.
pub fn expr_app(f: IdrisExpr, x: IdrisExpr) -> IdrisExpr {
    IdrisExpr::App(Box::new(f), Box::new(x))
}
/// `expr_lam(params, body)` — lambda expression.
pub fn expr_lam(params: Vec<String>, body: IdrisExpr) -> IdrisExpr {
    IdrisExpr::Lam(params, Box::new(body))
}
/// `expr_int(n)` — integer literal.
pub fn expr_int(n: i64) -> IdrisExpr {
    IdrisExpr::Lit(IdrisLiteral::Int(n))
}
/// `expr_str(s)` — string literal.
pub fn expr_str(s: impl Into<String>) -> IdrisExpr {
    IdrisExpr::Lit(IdrisLiteral::Str(s.into()))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_type_display_basic() {
        assert_eq!(format!("{}", IdrisType::Type), "Type");
        assert_eq!(format!("{}", IdrisType::Nat), "Nat");
        assert_eq!(format!("{}", IdrisType::Bool), "Bool");
        assert_eq!(format!("{}", IdrisType::String), "String");
        assert_eq!(format!("{}", IdrisType::Unit), "()");
    }
    #[test]
    pub(super) fn test_type_display_function() {
        let t = ty_fn(IdrisType::Nat, IdrisType::Bool);
        assert_eq!(format!("{}", t), "Nat -> Bool");
        let t2 = ty_fn(ty_fn(IdrisType::Nat, IdrisType::Nat), IdrisType::Bool);
        let s = format!("{}", t2);
        assert!(s.contains("->"), "got: {}", s);
        assert!(s.starts_with('(') || s.contains("Nat -> Nat"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_type_display_list_vect() {
        let lt = IdrisType::List(Box::new(IdrisType::Nat));
        assert_eq!(format!("{}", lt), "List Nat");
        let vt = IdrisType::Vect(
            Box::new(IdrisExpr::Lit(IdrisLiteral::Nat(3))),
            Box::new(IdrisType::Integer),
        );
        let s = format!("{}", vt);
        assert!(s.starts_with("Vect"), "got: {}", s);
        assert!(s.contains('3'), "got: {}", s);
    }
    #[test]
    pub(super) fn test_type_display_pi() {
        let t = ty_pi(
            "n",
            IdrisType::Nat,
            ty_data("Vec", vec![ty_var("n"), IdrisType::Integer]),
        );
        let s = format!("{}", t);
        assert!(s.contains("(n : Nat)"), "got: {}", s);
        assert!(s.contains("->"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_expr_basic() {
        assert_eq!(format!("{}", expr_var("x")), "x");
        assert_eq!(format!("{}", expr_int(42)), "42");
        assert_eq!(format!("{}", expr_str("hello")), "\"hello\"");
        assert_eq!(format!("{}", IdrisExpr::Refl), "Refl");
        assert_eq!(format!("{}", IdrisExpr::Hole("h".into())), "?h");
    }
    #[test]
    pub(super) fn test_expr_app_lam() {
        let app = expr_app(expr_var("succ"), expr_int(0));
        assert_eq!(format!("{}", app), "succ 0");
        let lam = expr_lam(
            vec!["x".into(), "y".into()],
            expr_app(expr_var("add"), expr_var("x")),
        );
        let s = format!("{}", lam);
        assert!(s.starts_with('\\'), "got: {}", s);
        assert!(s.contains("=>"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_expr_case_of() {
        let scrutinee = expr_var("n");
        let alts = vec![
            (IdrisPattern::Con("Z".into(), vec![]), expr_int(0)),
            (
                IdrisPattern::Con("S".into(), vec![IdrisPattern::Var("k".into())]),
                expr_var("k"),
            ),
        ];
        let ce = IdrisExpr::CaseOf(Box::new(scrutinee), alts);
        let s = format!("{}", ce);
        assert!(s.contains("case n of"), "got: {}", s);
        assert!(s.contains("Z"), "got: {}", s);
        assert!(s.contains("(S k)"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_function_emit_simple() {
        let f = IdrisFunction::new(
            "double",
            ty_fn(IdrisType::Nat, IdrisType::Nat),
            expr_app(expr_app(expr_var("(*"), expr_int(2)), expr_var("n")),
        );
        let s = f.emit(0);
        assert!(s.contains("double : Nat -> Nat"), "got: {}", s);
        assert!(s.contains("double ="), "got: {}", s);
    }
    #[test]
    pub(super) fn test_function_emit_clauses() {
        let f = IdrisFunction::with_clauses(
            "isZero",
            ty_fn(IdrisType::Nat, IdrisType::Bool),
            vec![
                (
                    vec![IdrisPattern::Con("Z".into(), vec![])],
                    IdrisExpr::Lit(IdrisLiteral::True),
                ),
                (
                    vec![IdrisPattern::Con("S".into(), vec![IdrisPattern::Wildcard])],
                    IdrisExpr::Lit(IdrisLiteral::False),
                ),
            ],
        );
        let s = f.emit(0);
        assert!(s.contains("isZero : Nat -> Bool"), "got: {}", s);
        assert!(s.contains("isZero Z = True"), "got: {}", s);
        assert!(s.contains("isZero (S _) = False"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_data_emit() {
        let d = IdrisData {
            name: "MyNat".into(),
            params: vec![],
            kind: IdrisType::Type,
            constructors: vec![
                IdrisConstructor {
                    name: "MZ".into(),
                    ty: ty_data("MyNat", vec![]),
                    doc: None,
                },
                IdrisConstructor {
                    name: "MS".into(),
                    ty: ty_fn(ty_data("MyNat", vec![]), ty_data("MyNat", vec![])),
                    doc: None,
                },
            ],
            visibility: Visibility::PublicExport,
            doc: None,
        };
        let s = d.emit(0);
        assert!(
            s.contains("public export data MyNat : Type where"),
            "got: {}",
            s
        );
        assert!(s.contains("MZ : MyNat"), "got: {}", s);
        assert!(s.contains("MS : MyNat -> MyNat"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_record_emit() {
        let r = IdrisRecord {
            name: "Point".into(),
            params: vec![("a".into(), IdrisType::Type)],
            kind: IdrisType::Type,
            constructor: "MkPoint".into(),
            fields: vec![("x".into(), ty_var("a")), ("y".into(), ty_var("a"))],
            visibility: Visibility::Export,
            doc: Some("A 2D point".into()),
        };
        let s = r.emit(0);
        assert!(s.contains("||| A 2D point"), "got: {}", s);
        assert!(
            s.contains("export record Point (a : Type) : Type where"),
            "got: {}",
            s
        );
        assert!(s.contains("constructor MkPoint"), "got: {}", s);
        assert!(s.contains("x : a"), "got: {}", s);
        assert!(s.contains("y : a"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_interface_emit() {
        let iface = IdrisInterface {
            name: "Container".into(),
            constraints: vec![],
            params: vec![("f".into(), ty_fn(IdrisType::Type, IdrisType::Type))],
            methods: vec![
                ("empty".into(), ty_data("f", vec![ty_var("a")])),
                (
                    "insert".into(),
                    ty_fn(
                        ty_var("a"),
                        ty_fn(
                            ty_data("f", vec![ty_var("a")]),
                            ty_data("f", vec![ty_var("a")]),
                        ),
                    ),
                ),
            ],
            defaults: vec![],
            visibility: Visibility::PublicExport,
            doc: None,
        };
        let s = iface.emit(0);
        assert!(
            s.contains("public export interface Container"),
            "got: {}",
            s
        );
        assert!(s.contains("empty :"), "got: {}", s);
        assert!(s.contains("insert :"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_module_emit() {
        let backend = IdrisBackend::new();
        let mut m = IdrisModule::new(vec!["Data".into(), "MyVec".into()]);
        m.import(IdrisImport::new(vec!["Data".into(), "Nat".into()]));
        m.import(IdrisImport::public_import(vec![
            "Data".into(),
            "List".into(),
        ]));
        m.add(IdrisDecl::Data(IdrisData {
            name: "Vec".into(),
            params: vec![("n".into(), IdrisType::Nat), ("a".into(), IdrisType::Type)],
            kind: IdrisType::Type,
            constructors: vec![
                IdrisConstructor {
                    name: "Nil".into(),
                    ty: ty_data("Vec", vec![ty_data("Z", vec![]), ty_var("a")]),
                    doc: None,
                },
                IdrisConstructor {
                    name: "Cons".into(),
                    ty: ty_fn(
                        ty_var("a"),
                        ty_fn(
                            ty_data("Vec", vec![ty_var("n"), ty_var("a")]),
                            ty_data("Vec", vec![ty_data("S", vec![ty_var("n")]), ty_var("a")]),
                        ),
                    ),
                    doc: None,
                },
            ],
            visibility: Visibility::PublicExport,
            doc: Some("Length-indexed vector".into()),
        }));
        let s = backend.emit_module(&m);
        assert!(s.contains("module Data.MyVec"), "got: {}", s);
        assert!(s.contains("import Data.Nat"), "got: {}", s);
        assert!(s.contains("public import Data.List"), "got: {}", s);
        assert!(s.contains("||| Length-indexed vector"), "got: {}", s);
        assert!(s.contains("public export data Vec"), "got: {}", s);
        assert!(s.contains("Nil :"), "got: {}", s);
        assert!(s.contains("Cons :"), "got: {}", s);
    }
    #[test]
    pub(super) fn test_namespace_and_mutual_emit() {
        let backend = IdrisBackend::proof_mode();
        let mut m = IdrisModule::new(vec!["Proof".into(), "Even".into()]);
        let even_decl = IdrisDecl::Mutual(vec![
            IdrisDecl::Func(IdrisFunction::with_clauses(
                "isEven",
                ty_fn(IdrisType::Nat, IdrisType::Bool),
                vec![
                    (
                        vec![IdrisPattern::Con("Z".into(), vec![])],
                        IdrisExpr::Lit(IdrisLiteral::True),
                    ),
                    (
                        vec![IdrisPattern::Con(
                            "S".into(),
                            vec![IdrisPattern::Var("n".into())],
                        )],
                        expr_app(expr_var("isOdd"), expr_var("n")),
                    ),
                ],
            )),
            IdrisDecl::Func(IdrisFunction::with_clauses(
                "isOdd",
                ty_fn(IdrisType::Nat, IdrisType::Bool),
                vec![
                    (
                        vec![IdrisPattern::Con("Z".into(), vec![])],
                        IdrisExpr::Lit(IdrisLiteral::False),
                    ),
                    (
                        vec![IdrisPattern::Con(
                            "S".into(),
                            vec![IdrisPattern::Var("n".into())],
                        )],
                        expr_app(expr_var("isEven"), expr_var("n")),
                    ),
                ],
            )),
        ]);
        m.add(even_decl);
        let s = backend.emit_module(&m);
        assert!(s.starts_with("-- AUTO-GENERATED"), "got: {}", s);
        assert!(s.contains("%default total"), "got: {}", s);
        assert!(s.contains("module Proof.Even"), "got: {}", s);
        assert!(s.contains("mutual"), "got: {}", s);
        assert!(s.contains("isEven"), "got: {}", s);
        assert!(s.contains("isOdd"), "got: {}", s);
    }
}
