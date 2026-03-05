//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::types::{
    CBAnalysisCache, CBConstantFoldingHelper, CBDepGraph, CBDominatorTree, CBLivenessInfo,
    CBPassConfig, CBPassPhase, CBPassRegistry, CBPassStats, CBWorklist, CBackend, CBinOp,
    CCodeWriter, CDecl, CEmitConfig, CEmitStats, CExpr, COutput, CStmt, CType, CUnaryOp,
    ClosureInfo, StructLayout,
};

/// Emit a single C statement to the writer.
pub(super) fn emit_stmt(w: &mut CCodeWriter, stmt: &CStmt) {
    match stmt {
        CStmt::VarDecl { ty, name, init } => {
            if let Some(init_expr) = init {
                w.writeln(&format!("{} {} = {};", ty, name, init_expr));
            } else {
                w.writeln(&format!("{} {};", ty, name));
            }
        }
        CStmt::Assign(lhs, rhs) => {
            w.writeln(&format!("{} = {};", lhs, rhs));
        }
        CStmt::If {
            cond,
            then_body,
            else_body,
        } => {
            w.writeln(&format!("if ({}) {{", cond));
            w.indent();
            for s in then_body {
                emit_stmt(w, s);
            }
            w.dedent();
            if else_body.is_empty() {
                w.writeln("}");
            } else {
                w.writeln("} else {");
                w.indent();
                for s in else_body {
                    emit_stmt(w, s);
                }
                w.dedent();
                w.writeln("}");
            }
        }
        CStmt::Switch {
            scrutinee,
            cases,
            default,
        } => {
            w.writeln(&format!("switch ({}) {{", scrutinee));
            w.indent();
            for (tag, body) in cases {
                w.writeln(&format!("case {}:", tag));
                w.indent();
                for s in body {
                    emit_stmt(w, s);
                }
                w.writeln("break;");
                w.dedent();
            }
            if !default.is_empty() {
                w.writeln("default:");
                w.indent();
                for s in default {
                    emit_stmt(w, s);
                }
                w.writeln("break;");
                w.dedent();
            }
            w.dedent();
            w.writeln("}");
        }
        CStmt::While { cond, body } => {
            w.writeln(&format!("while ({}) {{", cond));
            w.indent();
            for s in body {
                emit_stmt(w, s);
            }
            w.dedent();
            w.writeln("}");
        }
        CStmt::Return(expr) => {
            if let Some(e) = expr {
                w.writeln(&format!("return {};", e));
            } else {
                w.writeln("return;");
            }
        }
        CStmt::Block(stmts) => {
            w.writeln("{");
            w.indent();
            for s in stmts {
                emit_stmt(w, s);
            }
            w.dedent();
            w.writeln("}");
        }
        CStmt::Expr(e) => {
            w.writeln(&format!("{};", e));
        }
        CStmt::Comment(text) => {
            w.writeln(&format!("/* {} */", text));
        }
        CStmt::Blank => {
            w.write_blank();
        }
        CStmt::Label(name) => {
            let saved = w.indent_level;
            w.indent_level = 0;
            w.writeln(&format!("{}:", name));
            w.indent_level = saved;
        }
        CStmt::Goto(name) => {
            w.writeln(&format!("goto {};", name));
        }
        CStmt::Break => {
            w.writeln("break;");
        }
    }
}
/// Emit a C declaration to the writer.
pub(super) fn emit_decl(w: &mut CCodeWriter, decl: &CDecl) {
    match decl {
        CDecl::Function {
            ret_type,
            name,
            params,
            body,
            is_static,
        } => {
            let static_kw = if *is_static { "static " } else { "" };
            let params_str = format_params(params);
            w.writeln(&format!(
                "{}{} {}({}) {{",
                static_kw, ret_type, name, params_str
            ));
            w.indent();
            for s in body {
                emit_stmt(w, s);
            }
            w.dedent();
            w.writeln("}");
            w.write_blank();
        }
        CDecl::Struct { name, fields } => {
            w.writeln(&format!("typedef struct {} {{", name));
            w.indent();
            for (ty, fname) in fields {
                w.writeln(&format!("{} {};", ty, fname));
            }
            w.dedent();
            w.writeln(&format!("}} {};", name));
            w.write_blank();
        }
        CDecl::Typedef { original, alias } => {
            w.writeln(&format!("typedef {} {};", original, alias));
        }
        CDecl::Global {
            ty,
            name,
            init,
            is_static,
        } => {
            let static_kw = if *is_static { "static " } else { "" };
            if let Some(init_expr) = init {
                w.writeln(&format!("{}{} {} = {};", static_kw, ty, name, init_expr));
            } else {
                w.writeln(&format!("{}{} {};", static_kw, ty, name));
            }
        }
        CDecl::ForwardDecl {
            ret_type,
            name,
            params,
        } => {
            let params_str = format_params(params);
            w.writeln(&format!("{} {}({});", ret_type, name, params_str));
        }
    }
}
/// Format a parameter list as a comma-separated string.
pub(super) fn format_params(params: &[(CType, String)]) -> String {
    if params.is_empty() {
        return "void".to_string();
    }
    params
        .iter()
        .map(|(ty, name)| format!("{} {}", ty, name))
        .collect::<Vec<_>>()
        .join(", ")
}
/// Mangle an LCNF name into a valid C identifier.
pub(super) fn mangle_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 8);
    result.push_str("_oxl_");
    for c in name.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => result.push(c),
            '.' => result.push_str("__"),
            '<' => result.push_str("_lt_"),
            '>' => result.push_str("_gt_"),
            ' ' => result.push('_'),
            _ => {
                result.push_str(&format!("_u{:04x}_", c as u32));
            }
        }
    }
    result
}
/// Convert an LCNF variable ID to a C variable name.
pub(super) fn var_name(id: LcnfVarId) -> String {
    format!("_x{}", id.0)
}
/// Map an LCNF type to a C type.
pub(super) fn lcnf_type_to_ctype(ty: &LcnfType) -> CType {
    match ty {
        LcnfType::Erased | LcnfType::Irrelevant | LcnfType::Unit => CType::Void,
        LcnfType::Nat => CType::SizeT,
        LcnfType::LcnfString => CType::Ptr(Box::new(CType::Char)),
        LcnfType::Object => CType::LeanObject,
        LcnfType::Var(_) => CType::LeanObject,
        LcnfType::Fun(params, ret) => {
            let c_params: Vec<CType> = params.iter().map(lcnf_type_to_ctype).collect();
            let c_ret = lcnf_type_to_ctype(ret);
            CType::FnPtr(c_params, Box::new(c_ret))
        }
        LcnfType::Ctor(name, _) => CType::Ptr(Box::new(CType::Struct(mangle_name(name)))),
    }
}
/// Determine whether an LCNF type is "scalar" (unboxed in C).
pub(super) fn is_scalar_type(ty: &LcnfType) -> bool {
    matches!(
        ty,
        LcnfType::Nat | LcnfType::Erased | LcnfType::Unit | LcnfType::Irrelevant
    )
}
/// Determine whether a C type needs reference counting.
pub(super) fn needs_rc(cty: &CType) -> bool {
    matches!(cty, CType::LeanObject | CType::Ptr(_))
}
/// Generate a `lean_inc_ref(x)` call statement.
pub(super) fn lean_inc_ref(var: &str) -> CStmt {
    CStmt::Expr(CExpr::call("lean_inc_ref", vec![CExpr::var(var)]))
}
/// Generate a `lean_dec_ref(x)` call statement.
pub(super) fn lean_dec_ref(var: &str) -> CStmt {
    CStmt::Expr(CExpr::call("lean_dec_ref", vec![CExpr::var(var)]))
}
/// Generate a `lean_is_exclusive(x)` expression.
pub(super) fn lean_is_exclusive(var: &str) -> CExpr {
    CExpr::call("lean_is_exclusive", vec![CExpr::var(var)])
}
/// Generate a `lean_box(n)` expression (box a scalar).
pub(super) fn lean_box(expr: CExpr) -> CExpr {
    CExpr::call("lean_box", vec![expr])
}
/// Generate a `lean_unbox(obj)` expression (unbox to scalar).
pub(super) fn lean_unbox(expr: CExpr) -> CExpr {
    CExpr::call("lean_unbox", vec![expr])
}
/// Generate a `lean_alloc_ctor(tag, num_objs, scalar_sz)` call.
pub(super) fn lean_alloc_ctor(tag: u32, num_objs: usize, scalar_sz: usize) -> CExpr {
    CExpr::call(
        "lean_alloc_ctor",
        vec![
            CExpr::UIntLit(tag as u64),
            CExpr::UIntLit(num_objs as u64),
            CExpr::UIntLit(scalar_sz as u64),
        ],
    )
}
/// Generate a `lean_ctor_get(obj, i)` expression.
pub(super) fn lean_ctor_get(obj: &str, idx: usize) -> CExpr {
    CExpr::call(
        "lean_ctor_get",
        vec![CExpr::var(obj), CExpr::UIntLit(idx as u64)],
    )
}
/// Generate a `lean_ctor_set(obj, i, val)` statement.
pub(super) fn lean_ctor_set(obj: &str, idx: usize, val: CExpr) -> CStmt {
    CStmt::Expr(CExpr::call(
        "lean_ctor_set",
        vec![CExpr::var(obj), CExpr::UIntLit(idx as u64), val],
    ))
}
/// Generate `lean_obj_tag(obj)` expression.
pub(super) fn lean_obj_tag(obj: &str) -> CExpr {
    CExpr::call("lean_obj_tag", vec![CExpr::var(obj)])
}
/// Generate a closure struct declaration.
pub(super) fn gen_closure_struct(info: &ClosureInfo) -> CDecl {
    let mut fields = vec![
        (CType::LeanObject, "m_header".to_string()),
        (
            CType::FnPtr(vec![], Box::new(CType::LeanObject)),
            info.fn_ptr_field.clone(),
        ),
        (CType::U8, "m_arity".to_string()),
        (CType::U8, "m_num_fixed".to_string()),
    ];
    for (fname, fty) in &info.env_fields {
        fields.push((fty.clone(), fname.clone()));
    }
    CDecl::Struct {
        name: info.struct_name.clone(),
        fields,
    }
}
/// Generate code to allocate and initialize a closure.
pub(super) fn gen_closure_create(
    info: &ClosureInfo,
    fn_name: &str,
    env_vars: &[String],
) -> Vec<CStmt> {
    let mut stmts = Vec::new();
    let closure_var = format!("_closure_{}", info.struct_name);
    stmts.push(CStmt::VarDecl {
        ty: CType::LeanObject,
        name: closure_var.clone(),
        init: Some(CExpr::call(
            "lean_alloc_closure",
            vec![
                CExpr::Cast(
                    CType::Ptr(Box::new(CType::Void)),
                    Box::new(CExpr::var(fn_name)),
                ),
                CExpr::UIntLit(info.arity as u64),
                CExpr::UIntLit(env_vars.len() as u64),
            ],
        )),
    });
    for (i, env_var) in env_vars.iter().enumerate() {
        stmts.push(CStmt::Expr(CExpr::call(
            "lean_closure_set",
            vec![
                CExpr::var(&closure_var),
                CExpr::UIntLit(i as u64),
                CExpr::var(env_var),
            ],
        )));
    }
    stmts
}
/// Generate code to apply a closure to arguments.
pub(super) fn gen_closure_apply(closure_var: &str, args: &[CExpr], result_var: &str) -> Vec<CStmt> {
    let mut stmts = Vec::new();
    match args.len() {
        0 => {
            stmts.push(CStmt::VarDecl {
                ty: CType::LeanObject,
                name: result_var.to_string(),
                init: Some(CExpr::call(
                    "lean_apply_1",
                    vec![
                        CExpr::var(closure_var),
                        CExpr::call("lean_box", vec![CExpr::UIntLit(0)]),
                    ],
                )),
            });
        }
        1 => {
            stmts.push(CStmt::VarDecl {
                ty: CType::LeanObject,
                name: result_var.to_string(),
                init: Some(CExpr::call(
                    "lean_apply_1",
                    vec![CExpr::var(closure_var), args[0].clone()],
                )),
            });
        }
        2 => {
            stmts.push(CStmt::VarDecl {
                ty: CType::LeanObject,
                name: result_var.to_string(),
                init: Some(CExpr::call(
                    "lean_apply_2",
                    vec![CExpr::var(closure_var), args[0].clone(), args[1].clone()],
                )),
            });
        }
        _ => {
            let mut current = CExpr::var(closure_var);
            for (i, arg) in args.iter().enumerate() {
                let tmp = if i == args.len() - 1 {
                    result_var.to_string()
                } else {
                    format!("_app_tmp_{}", i)
                };
                stmts.push(CStmt::VarDecl {
                    ty: CType::LeanObject,
                    name: tmp.clone(),
                    init: Some(CExpr::call("lean_apply_1", vec![current, arg.clone()])),
                });
                current = CExpr::var(&tmp);
            }
        }
    }
    stmts
}
/// Generate a standard C header preamble with runtime includes.
pub(super) fn generate_header_preamble(module_name: &str) -> String {
    let guard = module_name.to_uppercase().replace('.', "_");
    format!(
        "#ifndef {guard}_H\n\
         #define {guard}_H\n\
         \n\
         #include <stdint.h>\n\
         #include <stddef.h>\n\
         #include <stdbool.h>\n\
         #include \"lean_runtime.h\"\n\
         \n",
    )
}
/// Generate a standard C header epilogue.
pub(super) fn generate_header_epilogue(module_name: &str) -> String {
    let guard = module_name.to_uppercase().replace('.', "_");
    format!("\n#endif /* {guard}_H */\n")
}
/// Generate the standard C source preamble with includes.
pub(super) fn generate_source_preamble(module_name: &str) -> String {
    format!(
        "#include \"{module_name}.h\"\n\
         \n\
         /* Generated by OxiLean C backend */\n\
         \n",
    )
}
/// Convenience function: compile an LCNF module to C source code.
pub fn compile_to_c(module: &LcnfModule, config: CEmitConfig) -> COutput {
    let mut backend = CBackend::new(config);
    backend.emit_module(module)
}
/// Convenience function: compile with default configuration.
pub fn compile_to_c_default(module: &LcnfModule) -> COutput {
    compile_to_c(module, CEmitConfig::default())
}
/// Compute the size in bytes of a C type (for layout purposes).
pub(super) fn c_type_size(ty: &CType) -> usize {
    match ty {
        CType::Void => 0,
        CType::Bool | CType::Char | CType::U8 => 1,
        CType::Int | CType::UInt | CType::SizeT | CType::LeanObject => 8,
        CType::Ptr(_) => 8,
        CType::FnPtr(_, _) => 8,
        CType::Array(elem, count) => c_type_size(elem) * count,
        CType::Struct(_) => 8,
    }
}
/// Compute the alignment of a C type.
pub(super) fn c_type_align(ty: &CType) -> usize {
    match ty {
        CType::Void => 1,
        CType::Bool | CType::Char | CType::U8 => 1,
        CType::Int | CType::UInt | CType::SizeT | CType::LeanObject => 8,
        CType::Ptr(_) => 8,
        CType::FnPtr(_, _) => 8,
        CType::Array(elem, _) => c_type_align(elem),
        CType::Struct(_) => 8,
    }
}
/// Compute the layout of a struct given its fields.
pub(super) fn compute_struct_layout(name: &str, fields: &[(CType, String)]) -> StructLayout {
    let mut offset = 0usize;
    let mut max_align = 1usize;
    let mut layout_fields = Vec::new();
    for (ty, fname) in fields {
        let align = c_type_align(ty);
        let size = c_type_size(ty);
        max_align = max_align.max(align);
        let padding = (align - (offset % align)) % align;
        offset += padding;
        layout_fields.push((fname.clone(), ty.clone(), offset));
        offset += size;
    }
    let final_padding = (max_align - (offset % max_align)) % max_align;
    offset += final_padding;
    StructLayout {
        name: name.to_string(),
        fields: layout_fields,
        total_size: offset,
        alignment: max_align,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn vid(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn mk_param(n: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: vid(n),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn mk_fun_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![mk_param(0, "n")],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    #[test]
    pub(super) fn test_ctype_display() {
        assert_eq!(CType::Void.to_string(), "void");
        assert_eq!(CType::Int.to_string(), "int64_t");
        assert_eq!(CType::UInt.to_string(), "uint64_t");
        assert_eq!(CType::Bool.to_string(), "uint8_t");
        assert_eq!(CType::SizeT.to_string(), "size_t");
        assert_eq!(CType::LeanObject.to_string(), "lean_object*");
    }
    #[test]
    pub(super) fn test_ctype_ptr_display() {
        let ptr = CType::Ptr(Box::new(CType::Int));
        assert_eq!(ptr.to_string(), "int64_t*");
    }
    #[test]
    pub(super) fn test_cbinop_display() {
        assert_eq!(CBinOp::Add.to_string(), "+");
        assert_eq!(CBinOp::Eq.to_string(), "==");
        assert_eq!(CBinOp::And.to_string(), "&&");
    }
    #[test]
    pub(super) fn test_cunaryop_display() {
        assert_eq!(CUnaryOp::Neg.to_string(), "-");
        assert_eq!(CUnaryOp::Not.to_string(), "!");
    }
    #[test]
    pub(super) fn test_cexpr_var() {
        let e = CExpr::Var("x".to_string());
        assert_eq!(e.to_string(), "x");
    }
    #[test]
    pub(super) fn test_cexpr_call() {
        let e = CExpr::call("f", vec![CExpr::var("x"), CExpr::IntLit(42)]);
        assert_eq!(e.to_string(), "f(x, 42LL)");
    }
    #[test]
    pub(super) fn test_cexpr_binop() {
        let e = CExpr::binop(CBinOp::Add, CExpr::var("a"), CExpr::var("b"));
        assert_eq!(e.to_string(), "(a + b)");
    }
    #[test]
    pub(super) fn test_mangle_name() {
        assert_eq!(mangle_name("Nat.add"), "_oxl_Nat__add");
        assert_eq!(mangle_name("foo"), "_oxl_foo");
    }
    #[test]
    pub(super) fn test_var_name() {
        assert_eq!(var_name(LcnfVarId(42)), "_x42");
    }
    #[test]
    pub(super) fn test_lcnf_type_to_ctype() {
        assert_eq!(lcnf_type_to_ctype(&LcnfType::Nat), CType::SizeT);
        assert_eq!(lcnf_type_to_ctype(&LcnfType::Object), CType::LeanObject);
        assert_eq!(lcnf_type_to_ctype(&LcnfType::Unit), CType::Void);
    }
    #[test]
    pub(super) fn test_is_scalar_type() {
        assert!(is_scalar_type(&LcnfType::Nat));
        assert!(is_scalar_type(&LcnfType::Unit));
        assert!(!is_scalar_type(&LcnfType::Object));
        assert!(!is_scalar_type(&LcnfType::Ctor("List".into(), vec![])));
    }
    #[test]
    pub(super) fn test_emit_simple_function() {
        let body = LcnfExpr::Return(LcnfArg::Var(vid(0)));
        let decl = mk_fun_decl("identity", body);
        let mut backend = CBackend::default_backend();
        let c_decl = backend.emit_fun_decl(&decl);
        if let CDecl::Function { name, body, .. } = &c_decl {
            assert!(name.contains("identity"));
            assert!(body.iter().any(|s| matches!(s, CStmt::Return(_))));
        } else {
            panic!("expected Function declaration");
        }
    }
    #[test]
    pub(super) fn test_emit_case_expression() {
        let body = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Ctor("Bool".into(), vec![]),
            alts: vec![
                LcnfAlt {
                    ctor_name: "False".into(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                },
                LcnfAlt {
                    ctor_name: "True".into(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                },
            ],
            default: None,
        };
        let decl = mk_fun_decl("to_nat", body);
        let mut backend = CBackend::default_backend();
        let c_decl = backend.emit_fun_decl(&decl);
        if let CDecl::Function { body, .. } = &c_decl {
            let has_switch = body.iter().any(|s| matches!(s, CStmt::Switch { .. }));
            assert!(has_switch, "expected a switch statement in the body");
        } else {
            panic!("expected Function");
        }
    }
    #[test]
    pub(super) fn test_emit_rc_calls() {
        let body = LcnfExpr::Let {
            id: vid(1),
            name: "result".to_string(),
            ty: LcnfType::Ctor("Pair".into(), vec![]),
            value: LcnfLetValue::Ctor(
                "Pair".into(),
                0,
                vec![LcnfArg::Var(vid(0)), LcnfArg::Var(vid(0))],
            ),
            body: Box::new(LcnfExpr::Return(LcnfArg::Var(vid(1)))),
        };
        let decl = mk_fun_decl("mk_pair", body);
        let mut backend = CBackend::new(CEmitConfig {
            use_rc: true,
            ..CEmitConfig::default()
        });
        let _c_decl = backend.emit_fun_decl(&decl);
    }
    #[test]
    pub(super) fn test_emit_module() {
        let decl = mk_fun_decl("main", LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))));
        let module = LcnfModule {
            fun_decls: vec![decl],
            extern_decls: vec![],
            name: "test".to_string(),
            metadata: LcnfModuleMetadata::default(),
        };
        let mut backend = CBackend::default_backend();
        let output = backend.emit_module(&module);
        assert!(!output.header.is_empty());
        assert!(!output.source.is_empty());
        assert!(output.header.contains("#ifndef"));
        assert!(output.header.contains("#endif"));
    }
    #[test]
    pub(super) fn test_c_emit_config_default() {
        let cfg = CEmitConfig::default();
        assert!(cfg.emit_comments);
        assert!(cfg.inline_small);
        assert!(cfg.use_rc);
    }
    #[test]
    pub(super) fn test_c_emit_stats_display() {
        let stats = CEmitStats {
            functions_emitted: 5,
            structs_emitted: 2,
            ..Default::default()
        };
        let s = stats.to_string();
        assert!(s.contains("fns=5"));
        assert!(s.contains("structs=2"));
    }
    #[test]
    pub(super) fn test_struct_layout() {
        let fields = vec![
            (CType::U8, "tag".to_string()),
            (CType::UInt, "value".to_string()),
        ];
        let layout = compute_struct_layout("TestStruct", &fields);
        assert!(layout.total_size > 0);
        assert_eq!(layout.alignment, 8);
        assert_eq!(layout.fields.len(), 2);
    }
    #[test]
    pub(super) fn test_format_params() {
        let params = vec![
            (CType::Int, "x".to_string()),
            (CType::Bool, "flag".to_string()),
        ];
        assert_eq!(format_params(&params), "int64_t x, uint8_t flag");
        assert_eq!(format_params(&[]), "void");
    }
    #[test]
    pub(super) fn test_compile_to_c_default() {
        let module = LcnfModule {
            fun_decls: vec![mk_fun_decl(
                "test_fn",
                LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42))),
            )],
            extern_decls: vec![],
            name: "test_mod".to_string(),
            metadata: LcnfModuleMetadata::default(),
        };
        let output = compile_to_c_default(&module);
        assert!(!output.source.is_empty());
        assert!(!output.declarations.is_empty());
    }
    #[test]
    pub(super) fn test_closure_struct_generation() {
        let info = ClosureInfo {
            struct_name: "Closure_add".to_string(),
            fn_ptr_field: "fn_ptr".to_string(),
            env_fields: vec![("captured_x".to_string(), CType::LeanObject)],
            arity: 1,
        };
        let decl = gen_closure_struct(&info);
        if let CDecl::Struct { name, fields } = &decl {
            assert_eq!(name, "Closure_add");
            assert!(fields.len() >= 4);
        } else {
            panic!("expected Struct declaration");
        }
    }
    #[test]
    pub(super) fn test_emit_let_chain() {
        let body = LcnfExpr::Let {
            id: vid(1),
            name: "a".to_string(),
            ty: LcnfType::Nat,
            value: LcnfLetValue::Lit(LcnfLit::Nat(42)),
            body: Box::new(LcnfExpr::Let {
                id: vid(2),
                name: "b".to_string(),
                ty: LcnfType::Nat,
                value: LcnfLetValue::App(LcnfArg::Var(vid(99)), vec![LcnfArg::Var(vid(1))]),
                body: Box::new(LcnfExpr::Return(LcnfArg::Var(vid(2)))),
            }),
        };
        let decl = mk_fun_decl("chain", body);
        let mut backend = CBackend::default_backend();
        let c_decl = backend.emit_fun_decl(&decl);
        if let CDecl::Function { body, .. } = &c_decl {
            let var_decl_count = body
                .iter()
                .filter(|s| matches!(s, CStmt::VarDecl { .. }))
                .count();
            assert!(var_decl_count >= 2);
        } else {
            panic!("expected Function");
        }
    }
    #[test]
    pub(super) fn test_needs_rc() {
        assert!(needs_rc(&CType::LeanObject));
        assert!(needs_rc(&CType::Ptr(Box::new(CType::Int))));
        assert!(!needs_rc(&CType::Int));
        assert!(!needs_rc(&CType::SizeT));
        assert!(!needs_rc(&CType::Void));
    }
    #[test]
    pub(super) fn test_c_type_size() {
        assert_eq!(c_type_size(&CType::Void), 0);
        assert_eq!(c_type_size(&CType::U8), 1);
        assert_eq!(c_type_size(&CType::Int), 8);
        assert_eq!(c_type_size(&CType::UInt), 8);
        assert_eq!(c_type_size(&CType::Ptr(Box::new(CType::Int))), 8);
    }
    #[test]
    pub(super) fn test_cexpr_string_lit() {
        let e = CExpr::StringLit("hello world".to_string());
        assert_eq!(e.to_string(), "\"hello world\"");
    }
    #[test]
    pub(super) fn test_cexpr_null() {
        assert_eq!(CExpr::Null.to_string(), "NULL");
    }
    #[test]
    pub(super) fn test_cstmt_comment() {
        let mut w = CCodeWriter::new("  ");
        emit_stmt(&mut w, &CStmt::Comment("test comment".to_string()));
        assert!(w.result().contains("/* test comment */"));
    }
    #[test]
    pub(super) fn test_emit_tail_call() {
        let body = LcnfExpr::TailCall(
            LcnfArg::Var(vid(99)),
            vec![LcnfArg::Var(vid(0)), LcnfArg::Lit(LcnfLit::Nat(1))],
        );
        let decl = mk_fun_decl("rec_fn", body);
        let mut backend = CBackend::default_backend();
        let c_decl = backend.emit_fun_decl(&decl);
        if let CDecl::Function { body, .. } = &c_decl {
            assert!(body.iter().any(|s| matches!(s, CStmt::Return(_))));
        } else {
            panic!("expected Function");
        }
    }
    #[test]
    pub(super) fn test_emit_unreachable() {
        let body = LcnfExpr::Unreachable;
        let decl = mk_fun_decl("unreachable_fn", body);
        let mut backend = CBackend::default_backend();
        let c_decl = backend.emit_fun_decl(&decl);
        if let CDecl::Function { body, .. } = &c_decl {
            let has_panic = body.iter().any(|s| {
                if let CStmt::Expr(CExpr::Call(name, _)) = s {
                    name.contains("panic")
                } else {
                    false
                }
            });
            assert!(has_panic, "expected panic call for unreachable");
        } else {
            panic!("expected Function");
        }
    }
}
#[cfg(test)]
mod CB_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = CBPassConfig::new("test_pass", CBPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = CBPassStats::new();
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
        let mut reg = CBPassRegistry::new();
        reg.register(CBPassConfig::new("pass_a", CBPassPhase::Analysis));
        reg.register(CBPassConfig::new("pass_b", CBPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = CBAnalysisCache::new(10);
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
        let mut wl = CBWorklist::new();
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
        let mut dt = CBDominatorTree::new(5);
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
        let mut liveness = CBLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(CBConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(CBConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(CBConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            CBConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(CBConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = CBDepGraph::new();
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
