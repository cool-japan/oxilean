//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    BlockRef, CraneliftBackend, CraneliftBlock, CraneliftDataObject, CraneliftFunction,
    CraneliftInstResult, CraneliftInstr, CraneliftModule, CraneliftType, CraneliftValue, IntCC,
    MemFlags, Signature,
};

pub(super) fn emit_values(vals: &[CraneliftValue]) -> String {
    vals.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
pub(super) fn emit_block_call(block: &BlockRef, args: &[CraneliftValue]) -> String {
    if args.is_empty() {
        block.to_string()
    } else {
        format!("{}({})", block, emit_values(args))
    }
}
pub(super) fn emit_instr(instr: &CraneliftInstr) -> String {
    match instr {
        CraneliftInstr::Iconst(ty, n) => format!("iconst.{} {}", ty, n),
        CraneliftInstr::Bconst(b) => format!("bconst.b1 {}", b),
        CraneliftInstr::F32Const(v) => format!("f32const {}", v),
        CraneliftInstr::F64Const(v) => format!("f64const {}", v),
        CraneliftInstr::Iadd(a, b) => format!("iadd {}, {}", a, b),
        CraneliftInstr::Isub(a, b) => format!("isub {}, {}", a, b),
        CraneliftInstr::Imul(a, b) => format!("imul {}, {}", a, b),
        CraneliftInstr::Sdiv(a, b) => format!("sdiv {}, {}", a, b),
        CraneliftInstr::Udiv(a, b) => format!("udiv {}, {}", a, b),
        CraneliftInstr::Srem(a, b) => format!("srem {}, {}", a, b),
        CraneliftInstr::Urem(a, b) => format!("urem {}, {}", a, b),
        CraneliftInstr::Ineg(a) => format!("ineg {}", a),
        CraneliftInstr::Iabs(a) => format!("iabs {}", a),
        CraneliftInstr::IaddImm(a, n) => format!("iadd_imm {}, {}", a, n),
        CraneliftInstr::ImulImm(a, n) => format!("imul_imm {}, {}", a, n),
        CraneliftInstr::Band(a, b) => format!("band {}, {}", a, b),
        CraneliftInstr::Bor(a, b) => format!("bor {}, {}", a, b),
        CraneliftInstr::Bxor(a, b) => format!("bxor {}, {}", a, b),
        CraneliftInstr::Bnot(a) => format!("bnot {}", a),
        CraneliftInstr::Ishl(a, b) => format!("ishl {}, {}", a, b),
        CraneliftInstr::Sshr(a, b) => format!("sshr {}, {}", a, b),
        CraneliftInstr::Ushr(a, b) => format!("ushr {}, {}", a, b),
        CraneliftInstr::Rotl(a, b) => format!("rotl {}, {}", a, b),
        CraneliftInstr::Rotr(a, b) => format!("rotr {}, {}", a, b),
        CraneliftInstr::Clz(a) => format!("clz {}", a),
        CraneliftInstr::Ctz(a) => format!("ctz {}", a),
        CraneliftInstr::Popcnt(a) => format!("popcnt {}", a),
        CraneliftInstr::Fadd(a, b) => format!("fadd {}, {}", a, b),
        CraneliftInstr::Fsub(a, b) => format!("fsub {}, {}", a, b),
        CraneliftInstr::Fmul(a, b) => format!("fmul {}, {}", a, b),
        CraneliftInstr::Fdiv(a, b) => format!("fdiv {}, {}", a, b),
        CraneliftInstr::Fneg(a) => format!("fneg {}", a),
        CraneliftInstr::Fabs(a) => format!("fabs {}", a),
        CraneliftInstr::Sqrt(a) => format!("sqrt {}", a),
        CraneliftInstr::Fma(a, b, c) => format!("fma {}, {}, {}", a, b, c),
        CraneliftInstr::Fmin(a, b) => format!("fmin {}, {}", a, b),
        CraneliftInstr::Fmax(a, b) => format!("fmax {}, {}", a, b),
        CraneliftInstr::Floor(a) => format!("floor {}", a),
        CraneliftInstr::Ceil(a) => format!("ceil {}", a),
        CraneliftInstr::FTrunc(a) => format!("trunc {}", a),
        CraneliftInstr::Nearest(a) => format!("nearest {}", a),
        CraneliftInstr::Icmp(cc, a, b) => format!("icmp {} {}, {}", cc, a, b),
        CraneliftInstr::Fcmp(cc, a, b) => format!("fcmp {} {}, {}", cc, a, b),
        CraneliftInstr::Select(c, t, f) => format!("select {}, {}, {}", c, t, f),
        CraneliftInstr::Sextend(ty, v) => format!("sextend.{} {}", ty, v),
        CraneliftInstr::Uextend(ty, v) => format!("uextend.{} {}", ty, v),
        CraneliftInstr::Ireduce(ty, v) => format!("ireduce.{} {}", ty, v),
        CraneliftInstr::Fpromote(ty, v) => format!("fpromote.{} {}", ty, v),
        CraneliftInstr::Fdemote(ty, v) => format!("fdemote.{} {}", ty, v),
        CraneliftInstr::FcvtToSint(ty, v) => format!("fcvt_to_sint.{} {}", ty, v),
        CraneliftInstr::FcvtToUint(ty, v) => format!("fcvt_to_uint.{} {}", ty, v),
        CraneliftInstr::FcvtFromSint(ty, v) => format!("fcvt_from_sint.{} {}", ty, v),
        CraneliftInstr::FcvtFromUint(ty, v) => format!("fcvt_from_uint.{} {}", ty, v),
        CraneliftInstr::Bitcast(ty, v) => format!("bitcast.{} {}", ty, v),
        CraneliftInstr::Load(ty, flags, addr, offset) => {
            let flags_str = flags.to_string();
            if flags_str.is_empty() {
                format!("load.{} {}+{}", ty, addr, offset)
            } else {
                format!("load.{} {} {}+{}", ty, flags_str, addr, offset)
            }
        }
        CraneliftInstr::Store(flags, val, addr, offset) => {
            let flags_str = flags.to_string();
            if flags_str.is_empty() {
                format!("store {}, {}+{}", val, addr, offset)
            } else {
                format!("store {} {}, {}+{}", flags_str, val, addr, offset)
            }
        }
        CraneliftInstr::StackAddr(ty, ss) => format!("stack_addr.{} ss{}", ty, ss),
        CraneliftInstr::GlobalValue(ty, gv) => format!("global_value.{} gv{}", ty, gv),
        CraneliftInstr::Jump(block, args) => {
            format!("jump {}", emit_block_call(block, args))
        }
        CraneliftInstr::Brif(cond, t_block, t_args, f_block, f_args) => {
            format!(
                "brif {}, {}, {}",
                cond,
                emit_block_call(t_block, t_args),
                emit_block_call(f_block, f_args)
            )
        }
        CraneliftInstr::BrTable(v, default, targets) => {
            let tgts = targets
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("br_table {}, {}, [{}]", v, default, tgts)
        }
        CraneliftInstr::Return(vals) => format!("return {}", emit_values(vals)),
        CraneliftInstr::Trap(code) => format!("trap {}", code),
        CraneliftInstr::Trapif(cc, v, code) => format!("trapif {} {}, {}", cc, v, code),
        CraneliftInstr::Unreachable => "unreachable".to_string(),
        CraneliftInstr::Call(func, args) => {
            format!("call {}({})", func, emit_values(args))
        }
        CraneliftInstr::CallIndirect(sig, callee, args) => {
            format!(
                "call_indirect sig{}, {}({})",
                sig,
                callee,
                emit_values(args)
            )
        }
        CraneliftInstr::ReturnCall(func, args) => {
            format!("return_call {}({})", func, emit_values(args))
        }
        CraneliftInstr::FuncAddr(ty, name) => format!("func_addr.{} {}", ty, name),
        CraneliftInstr::Null(ty) => format!("null.{}", ty),
        CraneliftInstr::Splat(ty, v) => format!("splat.{} {}", ty, v),
        CraneliftInstr::ExtractLane(v, lane) => format!("extractlane {}, {}", v, lane),
        CraneliftInstr::InsertLane(v, lane, elem) => {
            format!("insertlane {}, {}, {}", v, lane, elem)
        }
        CraneliftInstr::Copy(v) => format!("copy {}", v),
        CraneliftInstr::Nop => "nop".to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_cranelift_type_display() {
        assert_eq!(CraneliftType::I64.to_string(), "i64");
        assert_eq!(CraneliftType::F32.to_string(), "f32");
        assert_eq!(CraneliftType::F64.to_string(), "f64");
        assert_eq!(CraneliftType::B1.to_string(), "b1");
        assert_eq!(CraneliftType::R64.to_string(), "r64");
        assert_eq!(CraneliftType::I128.to_string(), "i128");
    }
    #[test]
    pub(super) fn test_cranelift_type_byte_width() {
        assert_eq!(CraneliftType::I8.byte_width(), Some(1));
        assert_eq!(CraneliftType::I32.byte_width(), Some(4));
        assert_eq!(CraneliftType::I64.byte_width(), Some(8));
        assert_eq!(CraneliftType::F64.byte_width(), Some(8));
        assert_eq!(CraneliftType::Void.byte_width(), None);
        assert_eq!(
            CraneliftType::Vector(Box::new(CraneliftType::I32), 4).byte_width(),
            Some(16)
        );
    }
    #[test]
    pub(super) fn test_cranelift_value_display() {
        let v = CraneliftValue::new(0, CraneliftType::I64);
        assert_eq!(v.to_string(), "v0");
        let v2 = CraneliftValue::new(42, CraneliftType::F32);
        assert_eq!(v2.to_string(), "v42");
    }
    #[test]
    pub(super) fn test_cranelift_instr_emit() {
        let v0 = CraneliftValue::new(0, CraneliftType::I64);
        let v1 = CraneliftValue::new(1, CraneliftType::I64);
        let iconst = CraneliftInstResult::with_result(
            v0.clone(),
            CraneliftInstr::Iconst(CraneliftType::I64, 42),
        );
        assert_eq!(iconst.emit(), "v0 = iconst.i64 42");
        let iadd = CraneliftInstResult::with_result(
            CraneliftValue::new(2, CraneliftType::I64),
            CraneliftInstr::Iadd(v0.clone(), v1.clone()),
        );
        assert_eq!(iadd.emit(), "v2 = iadd v0, v1");
        let icmp = CraneliftInstResult::with_result(
            CraneliftValue::new(3, CraneliftType::B1),
            CraneliftInstr::Icmp(IntCC::SignedLessThan, v0.clone(), v1.clone()),
        );
        assert_eq!(icmp.emit(), "v3 = icmp slt v0, v1");
    }
    #[test]
    pub(super) fn test_cranelift_block_emit() {
        let mut block = CraneliftBlock::new(0);
        let v0 = CraneliftValue::new(0, CraneliftType::I64);
        let v1 = CraneliftValue::new(1, CraneliftType::I64);
        let v2 = CraneliftValue::new(2, CraneliftType::I64);
        block.push_with_result(v0.clone(), CraneliftInstr::Iconst(CraneliftType::I64, 10));
        block.push_with_result(v1.clone(), CraneliftInstr::Iconst(CraneliftType::I64, 32));
        block.push_with_result(v2.clone(), CraneliftInstr::Iadd(v0, v1));
        block.push_void(CraneliftInstr::Return(vec![v2]));
        let out = block.emit();
        assert!(out.contains("block0:"), "missing label: {}", out);
        assert!(
            out.contains("iconst.i64 10"),
            "missing first iconst: {}",
            out
        );
        assert!(
            out.contains("iconst.i64 32"),
            "missing second iconst: {}",
            out
        );
        assert!(out.contains("iadd v0, v1"), "missing iadd: {}", out);
        assert!(out.contains("return v2"), "missing return: {}", out);
        assert!(block.is_terminated(), "block should be terminated");
    }
    #[test]
    pub(super) fn test_cranelift_function_build() {
        let sig = Signature::c_like(
            vec![CraneliftType::I64, CraneliftType::I64],
            vec![CraneliftType::I64],
        );
        let mut func = CraneliftFunction::new("add", sig);
        let entry = func.new_block();
        assert_eq!(entry, 0);
        let v_a = func.fresh_value(CraneliftType::I64);
        let v_b = func.fresh_value(CraneliftType::I64);
        let v_r = func.fresh_value(CraneliftType::I64);
        if let Some(block) = func.block_mut(entry) {
            block.push_with_result(v_r.clone(), CraneliftInstr::Iadd(v_a, v_b));
            block.push_void(CraneliftInstr::Return(vec![v_r]));
        }
        let out = func.emit();
        assert!(
            out.contains("function %add"),
            "missing func header: {}",
            out
        );
        assert!(out.contains("iadd"), "missing iadd: {}", out);
        assert!(out.contains("return"), "missing return: {}", out);
    }
    #[test]
    pub(super) fn test_cranelift_backend_simple_function() {
        let mut backend = CraneliftBackend::new("test_module");
        let sig = Signature::c_like(vec![CraneliftType::I64], vec![CraneliftType::I64]);
        backend.begin_function("double", sig);
        let v_input = CraneliftValue::new(0, CraneliftType::I64);
        let v_two = backend
            .iconst(CraneliftType::I64, 2)
            .expect("v_two iconst should succeed");
        let v_result = backend
            .imul(v_input, v_two)
            .expect("v_result imul should succeed");
        backend.emit_return(vec![v_result.clone()]);
        backend.end_function();
        let ir = backend.emit_module();
        assert!(ir.contains("function %double"), "missing func: {}", ir);
        assert!(ir.contains("iconst.i64 2"), "missing iconst: {}", ir);
        assert!(ir.contains("imul"), "missing imul: {}", ir);
        assert!(ir.contains("return"), "missing return: {}", ir);
    }
    #[test]
    pub(super) fn test_cranelift_brif_control_flow() {
        let sig = Signature::c_like(vec![CraneliftType::I64], vec![CraneliftType::I64]);
        let mut func = CraneliftFunction::new("max_zero", sig);
        let entry = func.new_block();
        let then_block = func.new_block();
        let else_block = func.new_block();
        let v_param = func.fresh_value(CraneliftType::I64);
        let v_zero = func.fresh_value(CraneliftType::I64);
        let v_cond = func.fresh_value(CraneliftType::B1);
        if let Some(b) = func.block_mut(entry) {
            b.params.push(v_param.clone());
            b.push_with_result(
                v_zero.clone(),
                CraneliftInstr::Iconst(CraneliftType::I64, 0),
            );
            b.push_with_result(
                v_cond.clone(),
                CraneliftInstr::Icmp(IntCC::SignedGreaterThan, v_param.clone(), v_zero.clone()),
            );
            b.push_void(CraneliftInstr::Brif(
                v_cond,
                BlockRef::new(then_block),
                vec![],
                BlockRef::new(else_block),
                vec![],
            ));
        }
        if let Some(b) = func.block_mut(then_block) {
            b.push_void(CraneliftInstr::Return(vec![v_param.clone()]));
        }
        if let Some(b) = func.block_mut(else_block) {
            b.push_void(CraneliftInstr::Return(vec![v_zero]));
        }
        let out = func.emit();
        assert!(out.contains("brif"), "missing brif: {}", out);
        assert!(out.contains("block1"), "missing then block: {}", out);
        assert!(out.contains("block2"), "missing else block: {}", out);
        assert!(out.contains("icmp sgt"), "missing icmp sgt: {}", out);
    }
    #[test]
    pub(super) fn test_cranelift_module_emit() {
        let mut module = CraneliftModule::new("math");
        module.target = "aarch64-unknown-linux-gnu".to_string();
        module.add_func_decl(
            "printf",
            Signature::c_like(vec![CraneliftType::I64], vec![]),
        );
        let data = CraneliftDataObject::readonly("greeting", b"hello\0".to_vec(), 1);
        module.add_data_object(data);
        let out = module.emit();
        assert!(
            out.contains("aarch64-unknown-linux-gnu"),
            "missing target: {}",
            out
        );
        assert!(out.contains("declare %printf"), "missing decl: {}", out);
        assert!(out.contains("rodata %greeting"), "missing data: {}", out);
    }
    #[test]
    pub(super) fn test_cranelift_mem_ops() {
        let sig = Signature::c_like(vec![CraneliftType::I64], vec![CraneliftType::I64]);
        let mut func = CraneliftFunction::new("load_and_double", sig);
        let entry = func.new_block();
        let v_ptr = func.fresh_value(CraneliftType::I64);
        let v_val = func.fresh_value(CraneliftType::I64);
        let v_two = func.fresh_value(CraneliftType::I64);
        let v_result = func.fresh_value(CraneliftType::I64);
        let v_new = func.fresh_value(CraneliftType::I64);
        if let Some(b) = func.block_mut(entry) {
            b.params.push(v_ptr.clone());
            b.push_with_result(
                v_val.clone(),
                CraneliftInstr::Load(CraneliftType::I64, MemFlags::trusted(), v_ptr.clone(), 0),
            );
            b.push_with_result(v_two.clone(), CraneliftInstr::Iconst(CraneliftType::I64, 2));
            b.push_with_result(v_result.clone(), CraneliftInstr::Imul(v_val, v_two.clone()));
            b.push_void(CraneliftInstr::Store(
                MemFlags::trusted(),
                v_result.clone(),
                v_ptr.clone(),
                0,
            ));
            b.push_with_result(v_new, CraneliftInstr::Iadd(v_result.clone(), v_two));
            b.push_void(CraneliftInstr::Return(vec![v_result]));
        }
        let out = func.emit();
        assert!(out.contains("load.i64"), "missing load: {}", out);
        assert!(out.contains("store"), "missing store: {}", out);
        assert!(out.contains("imul"), "missing imul: {}", out);
    }
}
