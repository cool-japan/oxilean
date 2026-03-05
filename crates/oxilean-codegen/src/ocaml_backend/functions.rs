//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BigarrayKind, BigarrayLayout, DuneExecutable, DuneLibrary, OCamlAnalysisCache,
    OCamlConstantFoldingHelper, OCamlDepGraph, OCamlDominatorTree, OCamlLivenessInfo,
    OCamlPassConfig, OCamlPassPhase, OCamlPassRegistry, OCamlPassStats, OCamlWorklist,
    OcamlBackend, OcamlDefinition, OcamlEffect, OcamlExpr, OcamlFunctor, OcamlGadt,
    OcamlLetBinding, OcamlLit, OcamlModule, OcamlPattern, OcamlPpxAttr, OcamlRecordField,
    OcamlSigItem, OcamlSignature, OcamlTestCase, OcamlTestSuite, OcamlType, OcamlTypeDecl,
    OcamlTypeDef,
};

pub(super) fn format_ocaml_expr(expr: &OcamlExpr, indent: usize) -> std::string::String {
    let pad = " ".repeat(indent);
    let ipad = " ".repeat(indent + 2);
    match expr {
        OcamlExpr::Lit(lit) => lit.to_string(),
        OcamlExpr::Var(name) => name.clone(),
        OcamlExpr::BinOp(op, lhs, rhs) => {
            format!(
                "({} {} {})",
                format_ocaml_expr(lhs, indent),
                op,
                format_ocaml_expr(rhs, indent)
            )
        }
        OcamlExpr::UnaryOp(op, expr) => {
            format!("({} {})", op, format_ocaml_expr(expr, indent))
        }
        OcamlExpr::App(func, args) => {
            let mut s = format!("({}", format_ocaml_expr(func, indent));
            for arg in args {
                s.push(' ');
                s.push_str(&format_ocaml_expr(arg, indent));
            }
            s.push(')');
            s
        }
        OcamlExpr::Lambda(params, body) => {
            format!(
                "(fun {} -> {})",
                params.join(" "),
                format_ocaml_expr(body, indent)
            )
        }
        OcamlExpr::Let(name, val, body) => {
            format!(
                "let {} = {} in\n{}{}",
                name,
                format_ocaml_expr(val, indent),
                ipad,
                format_ocaml_expr(body, indent + 2)
            )
        }
        OcamlExpr::LetRec(name, params, val, body) => {
            let param_str = if params.is_empty() {
                std::string::String::new()
            } else {
                format!(" {}", params.join(" "))
            };
            format!(
                "let rec {}{} = {} in\n{}{}",
                name,
                param_str,
                format_ocaml_expr(val, indent),
                ipad,
                format_ocaml_expr(body, indent + 2)
            )
        }
        OcamlExpr::IfThenElse(cond, then_e, else_e) => {
            format!(
                "if {} then\n{}{}\n{}else\n{}{}",
                format_ocaml_expr(cond, indent),
                ipad,
                format_ocaml_expr(then_e, indent + 2),
                pad,
                ipad,
                format_ocaml_expr(else_e, indent + 2)
            )
        }
        OcamlExpr::Match(scrutinee, arms) => {
            let mut s = format!("match {} with\n", format_ocaml_expr(scrutinee, indent));
            for (pat, body) in arms {
                s.push_str(&format!(
                    "{}| {} -> {}\n",
                    ipad,
                    pat,
                    format_ocaml_expr(body, indent + 2)
                ));
            }
            s
        }
        OcamlExpr::Tuple(elems) => {
            let parts: Vec<_> = elems.iter().map(|e| format_ocaml_expr(e, indent)).collect();
            format!("({})", parts.join(", "))
        }
        OcamlExpr::List(elems) => {
            let parts: Vec<_> = elems.iter().map(|e| format_ocaml_expr(e, indent)).collect();
            format!("[{}]", parts.join("; "))
        }
        OcamlExpr::Record(fields) => {
            let mut s = "{ ".to_string();
            for (i, (name, val)) in fields.iter().enumerate() {
                if i > 0 {
                    s.push_str("; ");
                }
                s.push_str(&format!("{} = {}", name, format_ocaml_expr(val, indent)));
            }
            s.push_str(" }");
            s
        }
        OcamlExpr::Field(obj, field) => {
            format!("{}.{}", format_ocaml_expr(obj, indent), field)
        }
        OcamlExpr::Module(path, expr) => {
            format!("{}.{}", path, format_ocaml_expr(expr, indent))
        }
        OcamlExpr::Begin(exprs) => {
            let parts: Vec<_> = exprs
                .iter()
                .map(|e| format!("{}{}", ipad, format_ocaml_expr(e, indent + 2)))
                .collect();
            format!("begin\n{}\n{}end", parts.join(";\n"), pad)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_ocaml_type_primitives() {
        assert_eq!(OcamlType::Int.to_string(), "int");
        assert_eq!(OcamlType::Float.to_string(), "float");
        assert_eq!(OcamlType::Bool.to_string(), "bool");
        assert_eq!(OcamlType::Char.to_string(), "char");
        assert_eq!(OcamlType::String.to_string(), "string");
        assert_eq!(OcamlType::Unit.to_string(), "unit");
    }
    #[test]
    pub(super) fn test_ocaml_type_list() {
        let t = OcamlType::List(Box::new(OcamlType::Int));
        assert_eq!(t.to_string(), "int list");
    }
    #[test]
    pub(super) fn test_ocaml_type_option() {
        let t = OcamlType::Option(Box::new(OcamlType::String));
        assert_eq!(t.to_string(), "string option");
    }
    #[test]
    pub(super) fn test_ocaml_type_result() {
        let t = OcamlType::Result(Box::new(OcamlType::Int), Box::new(OcamlType::String));
        assert_eq!(t.to_string(), "(int, string) result");
    }
    #[test]
    pub(super) fn test_ocaml_type_fun() {
        let t = OcamlType::Fun(Box::new(OcamlType::Int), Box::new(OcamlType::Bool));
        assert_eq!(t.to_string(), "int -> bool");
    }
    #[test]
    pub(super) fn test_ocaml_type_fun_chain() {
        let t = OcamlType::Fun(
            Box::new(OcamlType::Int),
            Box::new(OcamlType::Fun(
                Box::new(OcamlType::Int),
                Box::new(OcamlType::Int),
            )),
        );
        assert_eq!(t.to_string(), "int -> int -> int");
    }
    #[test]
    pub(super) fn test_ocaml_type_tuple() {
        let t = OcamlType::Tuple(vec![OcamlType::Int, OcamlType::String, OcamlType::Bool]);
        assert_eq!(t.to_string(), "int * string * bool");
    }
    #[test]
    pub(super) fn test_ocaml_type_polymorphic() {
        let t = OcamlType::Polymorphic("a".to_string());
        assert_eq!(t.to_string(), "'a");
    }
    #[test]
    pub(super) fn test_ocaml_type_array() {
        let t = OcamlType::Array(Box::new(OcamlType::Float));
        assert_eq!(t.to_string(), "float array");
    }
    #[test]
    pub(super) fn test_ocaml_lit_int() {
        assert_eq!(OcamlLit::Int(42).to_string(), "42");
        assert_eq!(OcamlLit::Int(-7).to_string(), "-7");
    }
    #[test]
    pub(super) fn test_ocaml_lit_float() {
        assert_eq!(OcamlLit::Float(3.14).to_string(), "3.14");
        assert_eq!(OcamlLit::Float(1.0).to_string(), "1.0");
    }
    #[test]
    pub(super) fn test_ocaml_lit_string() {
        assert_eq!(OcamlLit::Str("hello".to_string()).to_string(), "\"hello\"");
        assert_eq!(OcamlLit::Str("a\"b".to_string()).to_string(), "\"a\\\"b\"");
    }
    #[test]
    pub(super) fn test_ocaml_lit_bool_unit() {
        assert_eq!(OcamlLit::Bool(true).to_string(), "true");
        assert_eq!(OcamlLit::Bool(false).to_string(), "false");
        assert_eq!(OcamlLit::Unit.to_string(), "()");
    }
    #[test]
    pub(super) fn test_ocaml_pattern_wildcard() {
        assert_eq!(OcamlPattern::Wildcard.to_string(), "_");
    }
    #[test]
    pub(super) fn test_ocaml_pattern_var() {
        assert_eq!(OcamlPattern::Var("x".to_string()).to_string(), "x");
    }
    #[test]
    pub(super) fn test_ocaml_pattern_tuple() {
        let p = OcamlPattern::Tuple(vec![
            OcamlPattern::Var("a".to_string()),
            OcamlPattern::Var("b".to_string()),
        ]);
        assert_eq!(p.to_string(), "(a, b)");
    }
    #[test]
    pub(super) fn test_ocaml_pattern_cons() {
        let p = OcamlPattern::Cons(
            Box::new(OcamlPattern::Var("h".to_string())),
            Box::new(OcamlPattern::Var("t".to_string())),
        );
        assert_eq!(p.to_string(), "h :: t");
    }
    #[test]
    pub(super) fn test_ocaml_pattern_ctor_with_args() {
        let p = OcamlPattern::Ctor("Some".to_string(), vec![OcamlPattern::Var("x".to_string())]);
        assert_eq!(p.to_string(), "Some x");
    }
    #[test]
    pub(super) fn test_ocaml_pattern_ctor_no_args() {
        let p = OcamlPattern::Ctor("None".to_string(), vec![]);
        assert_eq!(p.to_string(), "None");
    }
    #[test]
    pub(super) fn test_ocaml_pattern_list() {
        let p = OcamlPattern::List(vec![
            OcamlPattern::Const(OcamlLit::Int(1)),
            OcamlPattern::Const(OcamlLit::Int(2)),
        ]);
        assert_eq!(p.to_string(), "[1; 2]");
    }
    #[test]
    pub(super) fn test_ocaml_pattern_or() {
        let p = OcamlPattern::Or(
            Box::new(OcamlPattern::Const(OcamlLit::Int(0))),
            Box::new(OcamlPattern::Const(OcamlLit::Int(1))),
        );
        assert_eq!(p.to_string(), "0 | 1");
    }
    #[test]
    pub(super) fn test_ocaml_expr_lambda() {
        let e = OcamlExpr::Lambda(
            vec!["x".to_string(), "y".to_string()],
            Box::new(OcamlExpr::BinOp(
                "+".to_string(),
                Box::new(OcamlExpr::Var("x".to_string())),
                Box::new(OcamlExpr::Var("y".to_string())),
            )),
        );
        assert_eq!(e.to_string(), "(fun x y -> (x + y))");
    }
    #[test]
    pub(super) fn test_ocaml_expr_if_then_else() {
        let e = OcamlExpr::IfThenElse(
            Box::new(OcamlExpr::Var("b".to_string())),
            Box::new(OcamlExpr::Lit(OcamlLit::Int(1))),
            Box::new(OcamlExpr::Lit(OcamlLit::Int(0))),
        );
        let s = e.to_string();
        assert!(s.contains("if b then"));
        assert!(s.contains("else"));
    }
    #[test]
    pub(super) fn test_ocaml_expr_match() {
        let e = OcamlExpr::Match(
            Box::new(OcamlExpr::Var("x".to_string())),
            vec![
                (
                    OcamlPattern::Ctor(
                        "Some".to_string(),
                        vec![OcamlPattern::Var("v".to_string())],
                    ),
                    OcamlExpr::Var("v".to_string()),
                ),
                (
                    OcamlPattern::Ctor("None".to_string(), vec![]),
                    OcamlExpr::Lit(OcamlLit::Int(0)),
                ),
            ],
        );
        let s = e.to_string();
        assert!(s.contains("match x with"));
        assert!(s.contains("| Some v ->"));
        assert!(s.contains("| None ->"));
    }
    #[test]
    pub(super) fn test_ocaml_expr_tuple() {
        let e = OcamlExpr::Tuple(vec![
            OcamlExpr::Lit(OcamlLit::Int(1)),
            OcamlExpr::Lit(OcamlLit::Bool(true)),
        ]);
        assert_eq!(e.to_string(), "(1, true)");
    }
    #[test]
    pub(super) fn test_ocaml_expr_list() {
        let e = OcamlExpr::List(vec![
            OcamlExpr::Lit(OcamlLit::Int(1)),
            OcamlExpr::Lit(OcamlLit::Int(2)),
            OcamlExpr::Lit(OcamlLit::Int(3)),
        ]);
        assert_eq!(e.to_string(), "[1; 2; 3]");
    }
    #[test]
    pub(super) fn test_ocaml_expr_record() {
        let e = OcamlExpr::Record(vec![
            (
                "name".to_string(),
                OcamlExpr::Lit(OcamlLit::Str("Alice".to_string())),
            ),
            ("age".to_string(), OcamlExpr::Lit(OcamlLit::Int(30))),
        ]);
        let s = e.to_string();
        assert!(s.contains("name = \"Alice\""));
        assert!(s.contains("age = 30"));
    }
    #[test]
    pub(super) fn test_ocaml_typedef_variant() {
        let td = OcamlTypeDef {
            name: "expr".to_string(),
            type_params: vec![],
            decl: OcamlTypeDecl::Variant(vec![
                ("Lit".to_string(), vec![OcamlType::Int]),
                (
                    "Add".to_string(),
                    vec![
                        OcamlType::Custom("expr".to_string()),
                        OcamlType::Custom("expr".to_string()),
                    ],
                ),
                (
                    "Mul".to_string(),
                    vec![
                        OcamlType::Custom("expr".to_string()),
                        OcamlType::Custom("expr".to_string()),
                    ],
                ),
            ]),
        };
        let s = td.to_string();
        assert!(s.contains("type expr ="));
        assert!(s.contains("| Lit of int"));
        assert!(s.contains("| Add of expr * expr"));
        assert!(s.contains("| Mul of expr * expr"));
    }
    #[test]
    pub(super) fn test_ocaml_typedef_record() {
        let td = OcamlTypeDef {
            name: "person".to_string(),
            type_params: vec![],
            decl: OcamlTypeDecl::Record(vec![
                OcamlRecordField {
                    name: "name".to_string(),
                    ty: OcamlType::String,
                    mutable: false,
                },
                OcamlRecordField {
                    name: "age".to_string(),
                    ty: OcamlType::Int,
                    mutable: true,
                },
            ]),
        };
        let s = td.to_string();
        assert!(s.contains("type person = {"));
        assert!(s.contains("name: string;"));
        assert!(s.contains("mutable age: int;"));
    }
    #[test]
    pub(super) fn test_ocaml_typedef_polymorphic() {
        let td = OcamlTypeDef {
            name: "tree".to_string(),
            type_params: vec!["a".to_string()],
            decl: OcamlTypeDecl::Variant(vec![
                ("Leaf".to_string(), vec![]),
                (
                    "Node".to_string(),
                    vec![
                        OcamlType::Custom("'a tree".to_string()),
                        OcamlType::Polymorphic("a".to_string()),
                        OcamlType::Custom("'a tree".to_string()),
                    ],
                ),
            ]),
        };
        let s = td.to_string();
        assert!(s.contains("'a tree"));
        assert!(s.contains("| Leaf"));
        assert!(s.contains("| Node"));
    }
    #[test]
    pub(super) fn test_ocaml_let_binding_basic() {
        let lb = OcamlLetBinding {
            is_rec: false,
            name: "pi".to_string(),
            params: vec![],
            body: OcamlExpr::Lit(OcamlLit::Float(3.14)),
            type_annotation: Some(OcamlType::Float),
        };
        let s = lb.to_string();
        assert!(s.contains("let pi : float ="));
        assert!(s.contains("3.14"));
    }
    #[test]
    pub(super) fn test_ocaml_let_binding_recursive() {
        let lb = OcamlLetBinding {
            is_rec: true,
            name: "fib".to_string(),
            params: vec![("n".to_string(), Some(OcamlType::Int))],
            body: OcamlExpr::IfThenElse(
                Box::new(OcamlExpr::BinOp(
                    "<=".to_string(),
                    Box::new(OcamlExpr::Var("n".to_string())),
                    Box::new(OcamlExpr::Lit(OcamlLit::Int(1))),
                )),
                Box::new(OcamlExpr::Var("n".to_string())),
                Box::new(OcamlExpr::BinOp(
                    "+".to_string(),
                    Box::new(OcamlExpr::App(
                        Box::new(OcamlExpr::Var("fib".to_string())),
                        vec![OcamlExpr::BinOp(
                            "-".to_string(),
                            Box::new(OcamlExpr::Var("n".to_string())),
                            Box::new(OcamlExpr::Lit(OcamlLit::Int(1))),
                        )],
                    )),
                    Box::new(OcamlExpr::App(
                        Box::new(OcamlExpr::Var("fib".to_string())),
                        vec![OcamlExpr::BinOp(
                            "-".to_string(),
                            Box::new(OcamlExpr::Var("n".to_string())),
                            Box::new(OcamlExpr::Lit(OcamlLit::Int(2))),
                        )],
                    )),
                )),
            ),
            type_annotation: Some(OcamlType::Int),
        };
        let s = lb.to_string();
        assert!(s.contains("let rec fib"));
        assert!(s.contains("(n : int)"));
    }
    #[test]
    pub(super) fn test_ocaml_signature() {
        let sig = OcamlSignature {
            name: "STACK".to_string(),
            items: vec![
                OcamlSigItem::Type(OcamlTypeDef {
                    name: "t".to_string(),
                    type_params: vec!["a".to_string()],
                    decl: OcamlTypeDecl::Abstract,
                }),
                OcamlSigItem::Val(
                    "push".to_string(),
                    OcamlType::Fun(
                        Box::new(OcamlType::Polymorphic("a".to_string())),
                        Box::new(OcamlType::Fun(
                            Box::new(OcamlType::Custom("'a t".to_string())),
                            Box::new(OcamlType::Custom("'a t".to_string())),
                        )),
                    ),
                ),
                OcamlSigItem::Val(
                    "pop".to_string(),
                    OcamlType::Fun(
                        Box::new(OcamlType::Custom("'a t".to_string())),
                        Box::new(OcamlType::Option(Box::new(OcamlType::Tuple(vec![
                            OcamlType::Polymorphic("a".to_string()),
                            OcamlType::Custom("'a t".to_string()),
                        ])))),
                    ),
                ),
            ],
        };
        let s = sig.to_string();
        assert!(s.contains("module type STACK = sig"));
        assert!(s.contains("val push :"));
        assert!(s.contains("val pop :"));
        assert!(s.contains("end"));
    }
    #[test]
    pub(super) fn test_ocaml_module_emit() {
        let mut m = OcamlModule::new("Math");
        m.add(OcamlDefinition::Let(OcamlLetBinding {
            is_rec: false,
            name: "square".to_string(),
            params: vec![("x".to_string(), Some(OcamlType::Int))],
            body: OcamlExpr::BinOp(
                "*".to_string(),
                Box::new(OcamlExpr::Var("x".to_string())),
                Box::new(OcamlExpr::Var("x".to_string())),
            ),
            type_annotation: Some(OcamlType::Int),
        }));
        let src = m.emit();
        assert!(src.contains("let square"));
        assert!(src.contains("(x : int)"));
        assert!(src.contains(": int ="));
    }
    #[test]
    pub(super) fn test_ocaml_backend_make_adt() {
        let backend = OcamlBackend::new("Ast");
        let td = backend.make_adt(
            "token",
            vec![],
            vec![
                ("Int", vec![OcamlType::Int]),
                ("Ident", vec![OcamlType::String]),
                ("Plus", vec![]),
                ("Minus", vec![]),
            ],
        );
        let s = td.to_string();
        assert!(s.contains("type token ="));
        assert!(s.contains("| Int of int"));
        assert!(s.contains("| Ident of string"));
        assert!(s.contains("| Plus"));
        assert!(s.contains("| Minus"));
    }
    #[test]
    pub(super) fn test_ocaml_backend_emit_mli() {
        let mut backend = OcamlBackend::new("MyLib");
        backend.add_definition(OcamlDefinition::Let(OcamlLetBinding {
            is_rec: false,
            name: "add".to_string(),
            params: vec![
                ("a".to_string(), Some(OcamlType::Int)),
                ("b".to_string(), Some(OcamlType::Int)),
            ],
            body: OcamlExpr::BinOp(
                "+".to_string(),
                Box::new(OcamlExpr::Var("a".to_string())),
                Box::new(OcamlExpr::Var("b".to_string())),
            ),
            type_annotation: Some(OcamlType::Int),
        }));
        let mli = backend.emit_mli();
        assert!(mli.contains("val add :"));
    }
    #[test]
    pub(super) fn test_ocaml_nested_module() {
        let mut inner = OcamlModule::new("Inner");
        inner.is_top_level = false;
        inner.add(OcamlDefinition::Let(OcamlLetBinding {
            is_rec: false,
            name: "x".to_string(),
            params: vec![],
            body: OcamlExpr::Lit(OcamlLit::Int(42)),
            type_annotation: None,
        }));
        let s = inner.emit();
        assert!(s.contains("module Inner = struct"));
        assert!(s.contains("let x"));
        assert!(s.contains("end"));
    }
    #[test]
    pub(super) fn test_ocaml_exception() {
        let def = OcamlDefinition::Exception("ParseError".to_string(), Some(OcamlType::String));
        assert_eq!(def.to_string(), "exception ParseError of string");
    }
    #[test]
    pub(super) fn test_ocaml_open() {
        let def = OcamlDefinition::Open("List".to_string());
        assert_eq!(def.to_string(), "open List");
    }
    #[test]
    pub(super) fn test_ocaml_begin_end() {
        let e = OcamlExpr::Begin(vec![
            OcamlExpr::Lit(OcamlLit::Unit),
            OcamlExpr::Lit(OcamlLit::Int(42)),
        ]);
        let s = e.to_string();
        assert!(s.contains("begin"));
        assert!(s.contains("end"));
    }
}
/// Emit code that packs a module into a first-class value.
#[allow(dead_code)]
pub fn emit_ocaml_pack_module(module_name: &str, module_type: &str) -> std::string::String {
    format!("(module {} : {})", module_name, module_type)
}
/// Emit code that unpacks a first-class module.
#[allow(dead_code)]
pub fn emit_ocaml_unpack_module(expr: &str, module_type: &str, name: &str) -> std::string::String {
    format!("let (module {}) = ({} : {}) in", name, expr, module_type)
}
/// Emit an OCaml `lazy` expression.
#[allow(dead_code)]
pub fn emit_ocaml_lazy(expr: &str) -> std::string::String {
    format!("lazy ({})", expr)
}
/// Emit an OCaml `Lazy.force` call.
#[allow(dead_code)]
pub fn emit_ocaml_lazy_force(var: &str) -> std::string::String {
    format!("Lazy.force {}", var)
}
/// Emit OCaml memoization via a Hashtbl.
#[allow(dead_code)]
pub fn emit_ocaml_memoize(
    fn_name: &str,
    key_type: &OcamlType,
    val_type: &OcamlType,
) -> std::string::String {
    format!(
        "let {name}_memo : ({key}, {val}) Hashtbl.t = Hashtbl.create 16\n\
         let {name} key =\n  match Hashtbl.find_opt {name}_memo key with\n  \
         | Some v -> v\n  | None ->\n    let v = {name}_impl key in\n    \
         Hashtbl.add {name}_memo key v; v\n",
        name = fn_name,
        key = key_type,
        val = val_type
    )
}
/// Helper to emit an OCaml CPS-transformed function.
#[allow(dead_code)]
pub fn emit_ocaml_cps_fn(fn_name: &str, params: &[&str], body: &str) -> std::string::String {
    let params_with_k: Vec<std::string::String> = params
        .iter()
        .map(|p| p.to_string())
        .chain(std::iter::once("k".to_string()))
        .collect();
    format!(
        "let {name} {params} = {body}\n",
        name = fn_name,
        params = params_with_k.join(" "),
        body = body
    )
}
/// Emit an OCaml CPS call.
#[allow(dead_code)]
pub fn emit_ocaml_cps_call(fn_name: &str, args: &[&str], cont: &str) -> std::string::String {
    let all_args: Vec<std::string::String> = args
        .iter()
        .map(|a| a.to_string())
        .chain(std::iter::once(format!("(fun result -> {})", cont)))
        .collect();
    format!("{} {}", fn_name, all_args.join(" "))
}
/// Emit an OCaml `Seq` generation function.
#[allow(dead_code)]
pub fn emit_ocaml_seq_of_list(list_expr: &str) -> std::string::String {
    format!("List.to_seq ({})", list_expr)
}
/// Emit an OCaml `Seq.map` call.
#[allow(dead_code)]
pub fn emit_ocaml_seq_map(fn_expr: &str, seq_expr: &str) -> std::string::String {
    format!("Seq.map ({}) ({})", fn_expr, seq_expr)
}
/// Emit an OCaml `Seq.filter` call.
#[allow(dead_code)]
pub fn emit_ocaml_seq_filter(pred_expr: &str, seq_expr: &str) -> std::string::String {
    format!("Seq.filter ({}) ({})", pred_expr, seq_expr)
}
/// Emit an OCaml `Seq.fold_left` call.
#[allow(dead_code)]
pub fn emit_ocaml_seq_fold(fn_expr: &str, init: &str, seq_expr: &str) -> std::string::String {
    format!("Seq.fold_left ({}) ({}) ({})", fn_expr, init, seq_expr)
}
/// Emit an OCaml `Format.printf` call.
#[allow(dead_code)]
pub fn emit_ocaml_printf(fmt: &str, args: &[&str]) -> std::string::String {
    if args.is_empty() {
        format!("Format.printf \"{}\"", fmt)
    } else {
        format!("Format.printf \"{}\" {}", fmt, args.join(" "))
    }
}
/// Emit an OCaml `Format.asprintf` (format to string).
#[allow(dead_code)]
pub fn emit_ocaml_asprintf(fmt: &str, args: &[&str]) -> std::string::String {
    if args.is_empty() {
        format!("Format.asprintf \"{}\"", fmt)
    } else {
        format!("Format.asprintf \"{}\" {}", fmt, args.join(" "))
    }
}
/// Emit OCaml `List.map` with an anonymous function.
#[allow(dead_code)]
pub fn emit_ocaml_list_map(param: &str, body: &str, list: &str) -> std::string::String {
    format!("List.map (fun {} -> {}) ({})", param, body, list)
}
/// Emit OCaml `List.filter`.
#[allow(dead_code)]
pub fn emit_ocaml_list_filter(param: &str, pred: &str, list: &str) -> std::string::String {
    format!("List.filter (fun {} -> {}) ({})", param, pred, list)
}
/// Emit OCaml `List.fold_left`.
#[allow(dead_code)]
pub fn emit_ocaml_list_fold(
    acc: &str,
    elem: &str,
    body: &str,
    init: &str,
    list: &str,
) -> std::string::String {
    format!(
        "List.fold_left (fun {} {} -> {}) ({}) ({})",
        acc, elem, body, init, list
    )
}
/// Emit OCaml `String.concat`.
#[allow(dead_code)]
pub fn emit_ocaml_string_concat(sep: &str, list_expr: &str) -> std::string::String {
    format!("String.concat \"{}\" ({})", sep, list_expr)
}
/// Emit OCaml `Array.init`.
#[allow(dead_code)]
pub fn emit_ocaml_array_init(n: &str, fn_expr: &str) -> std::string::String {
    format!("Array.init ({}) ({})", n, fn_expr)
}
/// Emit OCaml `Hashtbl.find_opt`.
#[allow(dead_code)]
pub fn emit_ocaml_hashtbl_find_opt(tbl: &str, key: &str) -> std::string::String {
    format!("Hashtbl.find_opt {} {}", tbl, key)
}
/// Emit OCaml Bigarray.Array1 creation.
#[allow(dead_code)]
pub fn emit_ocaml_bigarray1_create(
    kind: BigarrayKind,
    layout: BigarrayLayout,
    size: &str,
) -> std::string::String {
    format!(
        "Bigarray.Array1.create {} {} ({})",
        kind.kind_name(),
        layout.layout_name(),
        size
    )
}
/// Emit OCaml Bigarray.Array2 creation.
#[allow(dead_code)]
pub fn emit_ocaml_bigarray2_create(
    kind: BigarrayKind,
    layout: BigarrayLayout,
    rows: &str,
    cols: &str,
) -> std::string::String {
    format!(
        "Bigarray.Array2.create {} {} ({}) ({})",
        kind.kind_name(),
        layout.layout_name(),
        rows,
        cols
    )
}
#[cfg(test)]
mod ocaml_extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_ocaml_effect_decl() {
        let eff = OcamlEffect::new("Read", vec![OcamlType::String], OcamlType::Int);
        let decl = eff.emit_decl();
        assert!(decl.contains("Effect.t"), "missing Effect.t: {}", decl);
        assert!(decl.contains("Read"), "missing name: {}", decl);
        assert!(decl.contains("string"), "missing param: {}", decl);
    }
    #[test]
    pub(super) fn test_ocaml_effect_perform() {
        let eff = OcamlEffect::new("Get", vec![], OcamlType::Int);
        let perform = eff.emit_perform(&[]);
        assert!(
            perform.contains("Effect.perform Get"),
            "wrong perform: {}",
            perform
        );
        let eff2 = OcamlEffect::new("Put", vec![OcamlType::Int], OcamlType::Unit);
        let perform2 = eff2.emit_perform(&["42"]);
        assert!(
            perform2.contains("Effect.perform (Put 42)"),
            "wrong perform: {}",
            perform2
        );
    }
    #[test]
    pub(super) fn test_ocaml_gadt_emit() {
        let gadt = OcamlGadt::new("expr", vec!["a"])
            .add_variant("Int", vec![OcamlType::Int], "'a expr")
            .add_variant("Bool", vec![OcamlType::Bool], "'a expr")
            .add_variant(
                "Add",
                vec![
                    OcamlType::Custom("int expr".to_string()),
                    OcamlType::Custom("int expr".to_string()),
                ],
                "int expr",
            );
        let s = gadt.emit();
        assert!(s.contains("type"), "missing type: {}", s);
        assert!(s.contains("| Int"), "missing Int: {}", s);
        assert!(s.contains("| Bool"), "missing Bool: {}", s);
        assert!(s.contains("| Add"), "missing Add: {}", s);
    }
    #[test]
    pub(super) fn test_ocaml_functor_emit() {
        let f = OcamlFunctor::new("Make")
            .add_param("K", "Map.OrderedType")
            .add_param("V", "sig type t end")
            .add_def(OcamlDefinition::Open("K".to_string()));
        let s = f.emit();
        assert!(s.contains("module Make"), "missing module: {}", s);
        assert!(
            s.contains("(K : Map.OrderedType)"),
            "missing param K: {}",
            s
        );
        assert!(s.contains("open K"), "missing open: {}", s);
        assert!(s.contains("end"), "missing end: {}", s);
    }
    #[test]
    pub(super) fn test_ocaml_ppx_attr() {
        let deriving = OcamlPpxAttr::deriving(&["show", "eq", "ord"]);
        let s = deriving.emit();
        assert!(s.contains("[@deriving"), "missing attr: {}", s);
        assert!(s.contains("show"), "missing show: {}", s);
        assert!(s.contains("eq"), "missing eq: {}", s);
        let inline = OcamlPpxAttr::new("inline").emit_double();
        assert!(inline.contains("[@@inline]"), "wrong inline: {}", inline);
    }
    #[test]
    pub(super) fn test_dune_library_emit() {
        let lib = DuneLibrary::new("mylib")
            .public_name("my-package.mylib")
            .add_module("Foo")
            .add_module("Bar")
            .add_dep("core")
            .add_dep("async")
            .add_ppx("ppx_deriving.show")
            .with_inline_tests();
        let s = lib.emit();
        assert!(s.contains("(library"), "missing library: {}", s);
        assert!(s.contains("(name mylib)"), "missing name: {}", s);
        assert!(
            s.contains("(public_name my-package.mylib)"),
            "missing public: {}",
            s
        );
        assert!(s.contains("core"), "missing dep: {}", s);
        assert!(s.contains("(inline_tests)"), "missing tests: {}", s);
    }
    #[test]
    pub(super) fn test_dune_executable_emit() {
        let exe = DuneExecutable::new("main")
            .add_dep("mylib")
            .add_dep("cmdliner");
        let s = exe.emit();
        assert!(s.contains("(executable"), "missing exe: {}", s);
        assert!(s.contains("(name main)"), "missing name: {}", s);
        assert!(s.contains("mylib"), "missing dep: {}", s);
    }
    #[test]
    pub(super) fn test_ounit2_test_case_emit() {
        let tc = OcamlTestCase::assert_equal("test_add", "42", "add 20 22");
        let s = tc.emit_ounit();
        assert!(s.contains("\"test_add\""), "missing name: {}", s);
        assert!(s.contains("assert_equal"), "missing assert: {}", s);
        assert!(s.contains("(42)"), "missing expected: {}", s);
        assert!(s.contains("add 20 22"), "missing actual: {}", s);
    }
    #[test]
    pub(super) fn test_ounit2_suite_emit() {
        let suite = OcamlTestSuite::new("arithmetic")
            .add(OcamlTestCase::assert_equal("add", "4", "2 + 2"))
            .add(OcamlTestCase::new("true_is_true", "assert_bool \"\" true"));
        let s = suite.emit_ounit();
        assert!(s.contains("open OUnit2"), "missing open: {}", s);
        assert!(s.contains("let suite"), "missing suite: {}", s);
        assert!(s.contains("arithmetic"), "missing name: {}", s);
        assert!(s.contains("run_test_tt_main"), "missing runner: {}", s);
    }
    #[test]
    pub(super) fn test_ocaml_list_helpers() {
        let map = emit_ocaml_list_map("x", "x * 2", "numbers");
        assert!(map.contains("List.map"), "missing map: {}", map);
        assert!(map.contains("fun x ->"), "missing fun: {}", map);
        let filter = emit_ocaml_list_filter("x", "x > 0", "numbers");
        assert!(filter.contains("List.filter"), "missing filter: {}", filter);
        let fold = emit_ocaml_list_fold("acc", "x", "acc + x", "0", "numbers");
        assert!(fold.contains("List.fold_left"), "missing fold: {}", fold);
    }
    #[test]
    pub(super) fn test_ocaml_bigarray_create() {
        let code =
            emit_ocaml_bigarray1_create(BigarrayKind::Float32, BigarrayLayout::CLayout, "1024");
        assert!(
            code.contains("Bigarray.Array1.create"),
            "missing create: {}",
            code
        );
        assert!(code.contains("float32"), "missing kind: {}", code);
        assert!(code.contains("c_layout"), "missing layout: {}", code);
        assert!(code.contains("1024"), "missing size: {}", code);
    }
    #[test]
    pub(super) fn test_ocaml_memoize_emit() {
        let code = emit_ocaml_memoize("fib", &OcamlType::Int, &OcamlType::Int);
        assert!(code.contains("fib_memo"), "missing memo tbl: {}", code);
        assert!(code.contains("Hashtbl.create"), "missing create: {}", code);
        assert!(code.contains("Hashtbl.find_opt"), "missing find: {}", code);
        assert!(code.contains("Hashtbl.add"), "missing add: {}", code);
    }
    #[test]
    pub(super) fn test_ocaml_cps_helpers() {
        let fn_code = emit_ocaml_cps_fn("add_cps", &["x", "y"], "k (x + y)");
        assert!(fn_code.contains("add_cps"), "missing fn name: {}", fn_code);
        assert!(fn_code.contains(" k"), "missing continuation: {}", fn_code);
        assert!(fn_code.contains("k (x + y)"), "missing body: {}", fn_code);
        let call_code = emit_ocaml_cps_call("add_cps", &["2", "3"], "print_int result");
        assert!(call_code.contains("add_cps"), "missing fn: {}", call_code);
        assert!(
            call_code.contains("fun result ->"),
            "missing cont: {}",
            call_code
        );
    }
    #[test]
    pub(super) fn test_ocaml_seq_helpers() {
        let s = emit_ocaml_seq_of_list("[1;2;3]");
        assert!(s.contains("List.to_seq"), "missing to_seq: {}", s);
        let m = emit_ocaml_seq_map("fun x -> x * 2", "my_seq");
        assert!(m.contains("Seq.map"), "missing Seq.map: {}", m);
        let f = emit_ocaml_seq_fold("fun acc x -> acc + x", "0", "my_seq");
        assert!(f.contains("Seq.fold_left"), "missing Seq.fold_left: {}", f);
    }
    #[test]
    pub(super) fn test_bigarray_kind_names() {
        assert_eq!(BigarrayKind::Float32.kind_name(), "Bigarray.float32");
        assert_eq!(BigarrayKind::Float64.kind_name(), "Bigarray.float64");
        assert_eq!(BigarrayKind::Int32.element_type(), "int32");
        assert_eq!(
            BigarrayLayout::FortranLayout.layout_name(),
            "Bigarray.fortran_layout"
        );
    }
}
#[cfg(test)]
mod OCaml_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = OCamlPassConfig::new("test_pass", OCamlPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = OCamlPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = OCamlPassRegistry::new();
        reg.register(OCamlPassConfig::new("pass_a", OCamlPassPhase::Analysis));
        reg.register(OCamlPassConfig::new("pass_b", OCamlPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = OCamlAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = OCamlWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = OCamlDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = OCamlLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(OCamlConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(OCamlConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(OCamlConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            OCamlConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(OCamlConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = OCamlDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
