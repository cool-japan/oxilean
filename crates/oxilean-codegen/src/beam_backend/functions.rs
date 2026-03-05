//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::{
    AttributeBuilder, BeamBackend, BeamConstPool, BeamDeadEliminator, BeamEndian, BeamExpr,
    BeamFunction, BeamInstr, BeamLinker, BeamModule, BeamPattern, BeamPrinter, BeamProcess,
    BeamReg, BeamType, BeamTypeCtx, EtsAccess, EtsTable, EtsType, GenServerSpec, PatternNormalizer,
    TailCallInfo, XRegAllocator,
};

/// Format a single BEAM instruction as a string.
pub fn emit_instr(instr: &BeamInstr) -> String {
    match instr {
        BeamInstr::Label(l) => format!("label {}.", l),
        BeamInstr::FuncInfo {
            module,
            function,
            arity,
        } => {
            format!("func_info '{}' '{}' {}.", module, function, arity)
        }
        BeamInstr::Call { arity, label } => format!("call {} {}.", arity, label),
        BeamInstr::CallLast {
            arity,
            label,
            deallocate,
        } => {
            format!("call_last {} {} {}.", arity, label, deallocate)
        }
        BeamInstr::CallExt { arity, destination } => {
            format!(
                "call_ext {} {{extfunc, '{}', '{}', {}}}.",
                arity, destination.module, destination.function, destination.arity
            )
        }
        BeamInstr::CallExtLast {
            arity,
            destination,
            deallocate,
        } => {
            format!(
                "call_ext_last {} {{extfunc, '{}', '{}', {}}} {}.",
                arity, destination.module, destination.function, destination.arity, deallocate
            )
        }
        BeamInstr::CallFun { arity } => format!("call_fun {}.", arity),
        BeamInstr::Move { src, dst } => format!("move {} {}.", src, dst),
        BeamInstr::PutTuple { arity, dst } => format!("put_tuple {} {}.", arity, dst),
        BeamInstr::Put(val) => format!("put {}.", val),
        BeamInstr::GetTupleElement { src, index, dst } => {
            format!("get_tuple_element {} {} {}.", src, index, dst)
        }
        BeamInstr::SetTupleElement {
            value,
            tuple,
            index,
        } => {
            format!("set_tuple_element {} {} {}.", value, tuple, index)
        }
        BeamInstr::IsEq { fail, lhs, rhs } => format!("is_eq {} {} {}.", fail, lhs, rhs),
        BeamInstr::IsEqExact { fail, lhs, rhs } => {
            format!("is_eq_exact {} {} {}.", fail, lhs, rhs)
        }
        BeamInstr::IsNe { fail, lhs, rhs } => format!("is_ne {} {} {}.", fail, lhs, rhs),
        BeamInstr::IsLt { fail, lhs, rhs } => format!("is_lt {} {} {}.", fail, lhs, rhs),
        BeamInstr::IsGe { fail, lhs, rhs } => format!("is_ge {} {} {}.", fail, lhs, rhs),
        BeamInstr::IsInteger { fail, arg } => format!("is_integer {} {}.", fail, arg),
        BeamInstr::IsFloat { fail, arg } => format!("is_float {} {}.", fail, arg),
        BeamInstr::IsAtom { fail, arg } => format!("is_atom {} {}.", fail, arg),
        BeamInstr::IsNil { fail, arg } => format!("is_nil {} {}.", fail, arg),
        BeamInstr::IsList { fail, arg } => format!("is_list {} {}.", fail, arg),
        BeamInstr::IsTuple { fail, arg } => format!("is_tuple {} {}.", fail, arg),
        BeamInstr::IsBinary { fail, arg } => format!("is_binary {} {}.", fail, arg),
        BeamInstr::IsFunction { fail, arg } => format!("is_function {} {}.", fail, arg),
        BeamInstr::Jump(l) => format!("jump {}.", l),
        BeamInstr::Return => "return.".to_string(),
        BeamInstr::Send => "send.".to_string(),
        BeamInstr::RemoveMessage => "remove_message.".to_string(),
        BeamInstr::LoopRec { fail, dst } => format!("loop_rec {} {}.", fail, dst),
        BeamInstr::Wait(l) => format!("wait {}.", l),
        BeamInstr::WaitTimeout { fail, timeout } => {
            format!("wait_timeout {} {}.", fail, timeout)
        }
        BeamInstr::GcBif {
            name,
            fail,
            live,
            args,
            dst,
        } => {
            let args_str = args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "gc_bif '{}' {} {} [{}] {}.",
                name, fail, live, args_str, dst
            )
        }
        BeamInstr::Bif0 { name, dst } => format!("bif '{}' {}.", name, dst),
        BeamInstr::Allocate { stack_need, live } => {
            format!("allocate {} {}.", stack_need, live)
        }
        BeamInstr::Deallocate(n) => format!("deallocate {}.", n),
        BeamInstr::Init(r) => format!("init {}.", r),
        BeamInstr::MakeFun2(idx) => format!("make_fun2 {}.", idx),
        BeamInstr::GetList { src, head, tail } => {
            format!("get_list {} {} {}.", src, head, tail)
        }
        BeamInstr::PutList { head, tail, dst } => {
            format!("put_list {} {} {}.", head, tail, dst)
        }
        BeamInstr::Raise { class, reason } => format!("raise {} {}.", class, reason),
        BeamInstr::TryBegin { label, reg } => format!("try {} {}.", label, reg),
        BeamInstr::TryEnd(r) => format!("try_end {}.", r),
        BeamInstr::TryCase(r) => format!("try_case {}.", r),
        BeamInstr::Comment(s) => format!("%% {}", s),
    }
}
/// Sanitize a string to be a valid Erlang atom (lowercase, no special chars).
pub fn sanitize_atom(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_alphanumeric() || ch == '_' {
            if i == 0 && ch.is_uppercase() {
                for c in ch.to_lowercase() {
                    out.push(c);
                }
            } else {
                out.push(ch);
            }
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push_str("unnamed");
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn make_backend() -> BeamBackend {
        BeamBackend::new("test_module")
    }
    #[test]
    pub(super) fn test_beam_type_display_primitives() {
        assert_eq!(BeamType::Integer.to_string(), "integer()");
        assert_eq!(BeamType::Float.to_string(), "float()");
        assert_eq!(BeamType::Atom.to_string(), "atom()");
        assert_eq!(BeamType::Pid.to_string(), "pid()");
        assert_eq!(BeamType::Any.to_string(), "any()");
        assert_eq!(BeamType::None.to_string(), "none()");
    }
    #[test]
    pub(super) fn test_beam_type_display_compound() {
        let list_ty = BeamType::List(Box::new(BeamType::Integer));
        assert_eq!(list_ty.to_string(), "list(integer())");
        let tuple_ty = BeamType::Tuple(vec![BeamType::Atom, BeamType::Integer]);
        assert_eq!(tuple_ty.to_string(), "{atom(), integer()}");
        let map_ty = BeamType::Map(Box::new(BeamType::Atom), Box::new(BeamType::Any));
        assert_eq!(map_ty.to_string(), "#{atom()  => any()}");
    }
    #[test]
    pub(super) fn test_beam_type_fun_display() {
        let fun_ty = BeamType::Fun(
            vec![BeamType::Integer, BeamType::Integer],
            Box::new(BeamType::Integer),
        );
        assert_eq!(
            fun_ty.to_string(),
            "fun((integer(), integer()) -> integer())"
        );
    }
    #[test]
    pub(super) fn test_beam_module_creation() {
        let mut m = BeamModule::new("my_app");
        m.add_attribute("author", "oxilean");
        assert_eq!(m.name, "my_app");
        assert_eq!(m.attributes.len(), 1);
        assert_eq!(m.exports.len(), 0);
    }
    #[test]
    pub(super) fn test_beam_module_add_exported_function() {
        let mut m = BeamModule::new("my_mod");
        let mut f = BeamFunction::new("add", 2);
        f.export();
        m.add_function(f);
        assert_eq!(m.exports.len(), 1);
        assert_eq!(m.export_list(), vec!["add/2"]);
    }
    #[test]
    pub(super) fn test_emit_literal_nat() {
        let backend = make_backend();
        let lit = LcnfLit::Nat(42);
        match backend.emit_literal(&lit) {
            BeamExpr::LitInt(n) => assert_eq!(n, 42),
            other => panic!("Expected LitInt, got {:?}", other),
        }
    }
    #[test]
    pub(super) fn test_emit_literal_str() {
        let backend = make_backend();
        match backend.emit_literal(&LcnfLit::Str("hello".to_string())) {
            BeamExpr::LitString(s) => assert_eq!(s, "hello"),
            other => panic!("Expected LitString, got {:?}", other),
        }
    }
    #[test]
    pub(super) fn test_sanitize_atom() {
        assert_eq!(sanitize_atom("hello"), "hello");
        assert_eq!(sanitize_atom("Hello"), "hello");
        assert_eq!(sanitize_atom("my.func"), "my_func");
        assert_eq!(sanitize_atom(""), "unnamed");
        assert_eq!(sanitize_atom("add_two"), "add_two");
    }
    #[test]
    pub(super) fn test_emit_asm_contains_module() {
        let mut backend = make_backend();
        let mut f = BeamFunction::new("main", 0);
        f.export();
        backend.module.add_function(f);
        let asm = backend.emit_asm();
        assert!(asm.contains("test_module"));
        assert!(asm.contains("main"));
    }
    #[test]
    pub(super) fn test_emit_instr_move() {
        let instr = BeamInstr::Move {
            src: BeamReg::X(0),
            dst: BeamReg::Y(0),
        };
        assert_eq!(emit_instr(&instr), "move x(0) y(0).");
    }
    #[test]
    pub(super) fn test_emit_instr_call() {
        let instr = BeamInstr::Call { arity: 2, label: 5 };
        assert_eq!(emit_instr(&instr), "call 2 5.");
    }
    #[test]
    pub(super) fn test_beam_function_key() {
        let f = BeamFunction::new("factorial", 1);
        assert_eq!(f.key(), "factorial/1");
    }
    #[test]
    pub(super) fn test_emit_var_arg() {
        let mut backend = make_backend();
        let arg = LcnfArg::Var(LcnfVarId(7));
        match backend.emit_arg(&arg) {
            BeamExpr::Var(name) => assert!(name.contains('7')),
            other => panic!("Expected Var, got {:?}", other),
        }
    }
}
/// Check structural equality of two normalized patterns.
#[allow(dead_code)]
pub fn patterns_structurally_equal(a: &BeamPattern, b: &BeamPattern) -> bool {
    match (a, b) {
        (BeamPattern::Wildcard, BeamPattern::Wildcard) => true,
        (BeamPattern::Var(_), BeamPattern::Var(_)) => true,
        (BeamPattern::Nil, BeamPattern::Nil) => true,
        (BeamPattern::LitInt(x), BeamPattern::LitInt(y)) => x == y,
        (BeamPattern::LitAtom(x), BeamPattern::LitAtom(y)) => x == y,
        (BeamPattern::Cons(ah, at), BeamPattern::Cons(bh, bt)) => {
            patterns_structurally_equal(ah, bh) && patterns_structurally_equal(at, bt)
        }
        (BeamPattern::Tuple(ap), BeamPattern::Tuple(bp)) => {
            ap.len() == bp.len()
                && ap
                    .iter()
                    .zip(bp.iter())
                    .all(|(x, y)| patterns_structurally_equal(x, y))
        }
        _ => false,
    }
}
/// Analyse a BeamExpr for tail calls (returns info for `func_name`).
#[allow(dead_code)]
pub fn analyse_tail_calls(expr: &BeamExpr, func_name: &str) -> TailCallInfo {
    let mut info = TailCallInfo::new();
    collect_tail_calls_expr(expr, func_name, &mut info);
    info
}
pub(super) fn collect_tail_calls_expr(expr: &BeamExpr, func_name: &str, info: &mut TailCallInfo) {
    match expr {
        BeamExpr::Apply { fun, .. } => {
            if let BeamExpr::Var(name) = fun.as_ref() {
                if name == func_name {
                    info.add_self_tail();
                } else {
                    info.add_external_tail(name.clone());
                }
            }
        }
        BeamExpr::Call { func, .. } => {
            if func == func_name {
                info.add_self_tail();
            } else {
                info.add_external_tail(func.clone());
            }
        }
        BeamExpr::Let { body, .. } => {
            collect_tail_calls_expr(body, func_name, info);
        }
        BeamExpr::Case { clauses, .. } => {
            for clause in clauses {
                collect_tail_calls_expr(&clause.body, func_name, info);
            }
        }
        BeamExpr::Seq(_, second) => {
            collect_tail_calls_expr(second, func_name, info);
        }
        _ => {}
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_ets_table_type_display() {
        assert_eq!(EtsType::Set.to_string(), "set");
        assert_eq!(EtsType::OrderedSet.to_string(), "ordered_set");
        assert_eq!(EtsType::Bag.to_string(), "bag");
        assert_eq!(EtsType::DuplicateBag.to_string(), "duplicate_bag");
    }
    #[test]
    pub(super) fn test_ets_access_display() {
        assert_eq!(EtsAccess::Private.to_string(), "private");
        assert_eq!(EtsAccess::Protected.to_string(), "protected");
        assert_eq!(EtsAccess::Public.to_string(), "public");
    }
    #[test]
    pub(super) fn test_ets_table_emit_new() {
        let table = EtsTable::new_set("my_table");
        let expr = table.emit_new();
        match expr {
            BeamExpr::Call { func, .. } => assert_eq!(func, "new"),
            _ => panic!("expected Call"),
        }
    }
    #[test]
    pub(super) fn test_ets_table_emit_insert() {
        let table = EtsTable::new_set("test");
        let tuple = BeamExpr::Tuple(vec![BeamExpr::LitAtom("key".into()), BeamExpr::LitInt(42)]);
        match table.emit_insert(tuple) {
            BeamExpr::Call { func, .. } => assert_eq!(func, "insert"),
            _ => panic!("expected Call"),
        }
    }
    #[test]
    pub(super) fn test_ets_table_emit_lookup() {
        let table = EtsTable::new_set("test");
        let key = BeamExpr::LitAtom("key".into());
        match table.emit_lookup(key) {
            BeamExpr::Call { func, .. } => assert_eq!(func, "lookup"),
            _ => panic!("expected Call"),
        }
    }
    #[test]
    pub(super) fn test_ets_table_emit_delete() {
        let table = EtsTable::new_set("test");
        let key = BeamExpr::LitAtom("key".into());
        match table.emit_delete(key) {
            BeamExpr::Call { func, .. } => assert_eq!(func, "delete"),
            _ => panic!("expected Call"),
        }
    }
    #[test]
    pub(super) fn test_beam_process_spawn_expr() {
        let proc = BeamProcess::new("p1", "my_mod", "start")
            .with_arg(BeamExpr::LitInt(0))
            .linked();
        let spawn = proc.emit_spawn();
        match spawn {
            BeamExpr::Call { func, .. } => assert_eq!(func, "spawn_link"),
            _ => panic!("expected Call"),
        }
    }
    #[test]
    pub(super) fn test_beam_process_spawn_no_link() {
        let proc = BeamProcess::new("p2", "mod", "init");
        match proc.emit_spawn() {
            BeamExpr::Call { func, .. } => assert_eq!(func, "spawn"),
            _ => panic!("expected Call"),
        }
    }
    #[test]
    pub(super) fn test_gen_server_generate_module() {
        let spec = GenServerSpec::new("counter", BeamExpr::LitInt(0));
        let module = spec.generate_module();
        assert_eq!(module.name, "counter");
        assert!(!module.functions.is_empty());
        assert!(module.functions.iter().any(|f| f.name == "init"));
        assert!(module.functions.iter().any(|f| f.name == "handle_call"));
    }
    #[test]
    pub(super) fn test_gen_server_emit_init() {
        let spec = GenServerSpec::new("myserver", BeamExpr::LitAtom("idle".into()));
        let s = spec.emit_init();
        assert!(s.contains("init"));
        assert!(s.contains("ok"));
    }
    #[test]
    pub(super) fn test_beam_type_ctx_bind_lookup() {
        let mut ctx = BeamTypeCtx::new();
        ctx.bind("x", BeamType::Integer);
        assert_eq!(ctx.lookup("x"), Some(&BeamType::Integer));
        assert_eq!(ctx.lookup("y"), None);
    }
    #[test]
    pub(super) fn test_beam_type_ctx_infer_literals() {
        let ctx = BeamTypeCtx::new();
        assert_eq!(ctx.infer(&BeamExpr::LitInt(1)), BeamType::Integer);
        assert_eq!(ctx.infer(&BeamExpr::LitFloat(1.0)), BeamType::Float);
        assert_eq!(ctx.infer(&BeamExpr::LitAtom("ok".into())), BeamType::Atom);
    }
    #[test]
    pub(super) fn test_beam_type_ctx_infer_var() {
        let mut ctx = BeamTypeCtx::new();
        ctx.bind("n", BeamType::Integer);
        assert_eq!(ctx.infer(&BeamExpr::Var("n".into())), BeamType::Integer);
        assert_eq!(ctx.infer(&BeamExpr::Var("unknown".into())), BeamType::Any);
    }
    #[test]
    pub(super) fn test_beam_type_ctx_infer_tuple() {
        let ctx = BeamTypeCtx::new();
        let expr = BeamExpr::Tuple(vec![BeamExpr::LitAtom("ok".into()), BeamExpr::LitInt(1)]);
        let ty = ctx.infer(&expr);
        assert!(matches!(ty, BeamType::Tuple(_)));
    }
    #[test]
    pub(super) fn test_beam_type_ctx_merge() {
        let mut a = BeamTypeCtx::new();
        a.bind("x", BeamType::Integer);
        let mut b = BeamTypeCtx::new();
        b.bind("x", BeamType::Float);
        b.bind("y", BeamType::Atom);
        let merged = a.merge(&b);
        assert_eq!(merged.lookup("x"), Some(&BeamType::Any));
        assert_eq!(merged.lookup("y"), Some(&BeamType::Atom));
    }
    #[test]
    pub(super) fn test_pattern_normalizer_wildcard() {
        let mut norm = PatternNormalizer::new();
        let p = norm.normalize(BeamPattern::Var("_Whatever".into()));
        assert!(matches!(p, BeamPattern::Var(_)));
    }
    #[test]
    pub(super) fn test_patterns_structurally_equal() {
        let a = BeamPattern::Tuple(vec![
            BeamPattern::LitAtom("ok".into()),
            BeamPattern::Var("x".into()),
        ]);
        let b = BeamPattern::Tuple(vec![
            BeamPattern::LitAtom("ok".into()),
            BeamPattern::Var("y".into()),
        ]);
        assert!(patterns_structurally_equal(&a, &b));
    }
    #[test]
    pub(super) fn test_patterns_structurally_not_equal() {
        let a = BeamPattern::LitInt(1);
        let b = BeamPattern::LitInt(2);
        assert!(!patterns_structurally_equal(&a, &b));
    }
    #[test]
    pub(super) fn test_beam_linker_merge() {
        let mut m1 = BeamModule::new("mod_a");
        m1.add_function(BeamFunction::new("foo", 0));
        let mut m2 = BeamModule::new("mod_b");
        m2.add_function(BeamFunction::new("bar", 1));
        let mut linker = BeamLinker::new("combined");
        linker.add_module(m1);
        linker.add_module(m2);
        let merged = linker.link();
        assert_eq!(merged.name, "combined");
        assert_eq!(merged.functions.len(), 2);
    }
    #[test]
    pub(super) fn test_beam_linker_rename() {
        let mut m = BeamModule::new("mod");
        m.add_function(BeamFunction::new("helper", 1));
        let mut linker = BeamLinker::new("output");
        linker.rename("mod", "helper", "mod_helper");
        linker.add_module(m);
        let merged = linker.link();
        assert!(merged.functions.iter().any(|f| f.name == "mod_helper"));
    }
    #[test]
    pub(super) fn test_beam_dead_eliminator_seed_and_eliminate() {
        let mut module = BeamModule::new("test");
        let mut exported = BeamFunction::new("public_fn", 0);
        exported.export();
        module.add_function(exported);
        module.add_function(BeamFunction::new("private_fn", 0));
        let mut elim = BeamDeadEliminator::new();
        elim.seed_exports(&module);
        let after = elim.eliminate(module.clone());
        assert!(!after.functions.is_empty());
    }
    #[test]
    pub(super) fn test_beam_const_pool_intern() {
        let mut pool = BeamConstPool::new();
        let idx1 = pool.intern(BeamExpr::LitInt(42), "forty_two");
        let idx2 = pool.intern(BeamExpr::LitInt(99), "ninety_nine");
        let idx1_again = pool.intern(BeamExpr::LitInt(0), "forty_two");
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(idx1_again, idx1);
        assert_eq!(pool.len(), 2);
    }
    #[test]
    pub(super) fn test_beam_const_pool_get() {
        let mut pool = BeamConstPool::new();
        let idx = pool.intern(BeamExpr::LitAtom("hello".into()), "greeting");
        let retrieved = pool.get(idx);
        assert!(retrieved.is_some());
        assert!(
            matches!(retrieved.expect("value should be Some/Ok"), BeamExpr::LitAtom(s) if s == "hello")
        );
    }
    #[test]
    pub(super) fn test_attribute_builder() {
        let attrs = AttributeBuilder::new()
            .vsn("1.0.0")
            .author("oxilean")
            .compile("debug_info")
            .build();
        assert_eq!(attrs.len(), 3);
        assert_eq!(attrs[0], ("vsn".into(), "1.0.0".into()));
    }
    #[test]
    pub(super) fn test_attribute_builder_apply() {
        let mut module = BeamModule::new("mymod");
        AttributeBuilder::new().author("test").apply(&mut module);
        assert_eq!(module.attributes.len(), 1);
    }
    #[test]
    pub(super) fn test_tail_call_info_self() {
        let mut info = TailCallInfo::new();
        info.add_self_tail();
        assert!(info.is_tail_recursive);
        assert_eq!(info.self_tail_calls, 1);
        assert!(info.has_tail_calls());
    }
    #[test]
    pub(super) fn test_analyse_tail_calls_apply() {
        let expr = BeamExpr::Apply {
            fun: Box::new(BeamExpr::Var("fact".into())),
            args: vec![BeamExpr::LitInt(10)],
        };
        let info = analyse_tail_calls(&expr, "fact");
        assert_eq!(info.self_tail_calls, 1);
    }
    #[test]
    pub(super) fn test_analyse_tail_calls_let() {
        let expr = BeamExpr::Let {
            var: "x".into(),
            value: Box::new(BeamExpr::LitInt(1)),
            body: Box::new(BeamExpr::Apply {
                fun: Box::new(BeamExpr::Var("go".into())),
                args: vec![],
            }),
        };
        let info = analyse_tail_calls(&expr, "go");
        assert!(info.is_tail_recursive);
    }
    #[test]
    pub(super) fn test_beam_printer_lit_int() {
        let mut p = BeamPrinter::new();
        p.print_expr(&BeamExpr::LitInt(42));
        assert_eq!(p.finish(), "42");
    }
    #[test]
    pub(super) fn test_beam_printer_var() {
        let mut p = BeamPrinter::new();
        p.print_expr(&BeamExpr::Var("X".into()));
        assert_eq!(p.finish(), "X");
    }
    #[test]
    pub(super) fn test_beam_printer_tuple() {
        let mut p = BeamPrinter::new();
        p.print_expr(&BeamExpr::Tuple(vec![
            BeamExpr::LitAtom("ok".into()),
            BeamExpr::LitInt(0),
        ]));
        let out = p.finish();
        assert!(out.contains("'ok'"));
        assert!(out.contains('0'));
    }
    #[test]
    pub(super) fn test_x_reg_allocator_alloc() {
        let mut alloc = XRegAllocator::new();
        let r0 = alloc.alloc("x");
        let r1 = alloc.alloc("y");
        assert_eq!(r0, 0);
        assert_eq!(r1, 1);
        assert_eq!(alloc.get("x"), Some(0));
        assert_eq!(alloc.registers_used(), 2);
    }
    #[test]
    pub(super) fn test_x_reg_allocator_reset() {
        let mut alloc = XRegAllocator::new();
        alloc.alloc("a");
        alloc.alloc("b");
        alloc.reset();
        assert_eq!(alloc.registers_used(), 0);
        assert_eq!(alloc.get("a"), None);
    }
    #[test]
    pub(super) fn test_beam_type_union_display() {
        let u = BeamType::Union(vec![BeamType::Atom, BeamType::Integer]);
        let s = u.to_string();
        assert!(s.contains("atom()"));
        assert!(s.contains("integer()"));
    }
    #[test]
    pub(super) fn test_beam_type_named_display() {
        let t = BeamType::Named("my_type".into());
        assert_eq!(t.to_string(), "my_type()");
    }
    #[test]
    pub(super) fn test_beam_endian_display() {
        assert_eq!(BeamEndian::Big.to_string(), "big");
        assert_eq!(BeamEndian::Little.to_string(), "little");
        assert_eq!(BeamEndian::Native.to_string(), "native");
    }
}
