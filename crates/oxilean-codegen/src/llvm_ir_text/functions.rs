//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    CallingConv, IcmpPred, LITAnalysisCache, LITConstantFoldingHelper, LITDepGraph,
    LITDominatorTree, LITLivenessInfo, LITPassConfig, LITPassPhase, LITPassRegistry, LITPassStats,
    LITWorklist, LlvmIrBlock, LlvmIrFunction, LlvmIrGlobal, LlvmIrInstr, LlvmIrModule, LlvmIrParam,
    LlvmIrTextEmitter, LlvmIrType, LlvmIrValue, RegisterAllocator,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_type_display_primitives() {
        assert_eq!(LlvmIrType::Void.to_string(), "void");
        assert_eq!(LlvmIrType::I1.to_string(), "i1");
        assert_eq!(LlvmIrType::I32.to_string(), "i32");
        assert_eq!(LlvmIrType::I64.to_string(), "i64");
        assert_eq!(LlvmIrType::Float.to_string(), "float");
        assert_eq!(LlvmIrType::Double.to_string(), "double");
        assert_eq!(LlvmIrType::Ptr.to_string(), "ptr");
        assert_eq!(LlvmIrType::IArb(24).to_string(), "i24");
    }
    #[test]
    pub(super) fn test_type_display_compound() {
        let arr = LlvmIrType::Array(4, Box::new(LlvmIrType::I32));
        assert_eq!(arr.to_string(), "[4 x i32]");
        let st = LlvmIrType::Struct(vec![LlvmIrType::I32, LlvmIrType::I64]);
        assert_eq!(st.to_string(), "{ i32, i64 }");
        let vec_ty = LlvmIrType::Vector(8, Box::new(LlvmIrType::Float));
        assert_eq!(vec_ty.to_string(), "<8 x float>");
        let named = LlvmIrType::Named("MyStruct".to_string());
        assert_eq!(named.to_string(), "%MyStruct");
        let func_ty = LlvmIrType::Func {
            ret: Box::new(LlvmIrType::I32),
            params: vec![LlvmIrType::I64, LlvmIrType::Ptr],
            variadic: false,
        };
        assert_eq!(func_ty.to_string(), "i32 (i64, ptr)");
        let variadic_func = LlvmIrType::Func {
            ret: Box::new(LlvmIrType::I32),
            params: vec![LlvmIrType::Ptr],
            variadic: true,
        };
        assert_eq!(variadic_func.to_string(), "i32 (ptr, ...)");
    }
    #[test]
    pub(super) fn test_emit_arithmetic_instrs() {
        let emitter = LlvmIrTextEmitter::new();
        let add = LlvmIrInstr::Add {
            dest: "r0".to_string(),
            ty: LlvmIrType::I32,
            lhs: LlvmIrValue::Register("a".to_string()),
            rhs: LlvmIrValue::Register("b".to_string()),
            nsw: true,
            nuw: false,
        };
        assert_eq!(emitter.emit_instr(&add), "%r0 = add nsw i32 %a, %b");
        let mul = LlvmIrInstr::Mul {
            dest: "r1".to_string(),
            ty: LlvmIrType::I64,
            lhs: LlvmIrValue::ConstInt(42),
            rhs: LlvmIrValue::Register("x".to_string()),
            nsw: false,
            nuw: false,
        };
        assert_eq!(emitter.emit_instr(&mul), "%r1 = mul i64 42, %x");
        let icmp = LlvmIrInstr::Icmp {
            dest: "cmp".to_string(),
            pred: IcmpPred::Slt,
            ty: LlvmIrType::I32,
            lhs: LlvmIrValue::Register("n".to_string()),
            rhs: LlvmIrValue::ConstInt(0),
        };
        assert_eq!(emitter.emit_instr(&icmp), "%cmp = icmp slt i32 %n, 0");
    }
    #[test]
    pub(super) fn test_emit_memory_instrs() {
        let emitter = LlvmIrTextEmitter::new();
        let alloca = LlvmIrInstr::Alloca {
            dest: "ptr0".to_string(),
            ty: LlvmIrType::I32,
            align: Some(4),
        };
        assert_eq!(emitter.emit_instr(&alloca), "%ptr0 = alloca i32, align 4");
        let load = LlvmIrInstr::Load {
            dest: "val".to_string(),
            ty: LlvmIrType::I64,
            ptr: LlvmIrValue::Register("ptr0".to_string()),
            align: Some(8),
            volatile: false,
        };
        assert_eq!(
            emitter.emit_instr(&load),
            "%val = load i64, ptr %ptr0, align 8"
        );
        let store = LlvmIrInstr::Store {
            ty: LlvmIrType::I32,
            val: LlvmIrValue::ConstInt(99),
            ptr: LlvmIrValue::Register("p".to_string()),
            align: None,
            volatile: false,
        };
        assert_eq!(emitter.emit_instr(&store), "store i32 99, ptr %p");
    }
    #[test]
    pub(super) fn test_emit_control_flow_instrs() {
        let emitter = LlvmIrTextEmitter::new();
        let br = LlvmIrInstr::BrConditional {
            cond: LlvmIrValue::Register("c".to_string()),
            true_dest: "then".to_string(),
            false_dest: "else".to_string(),
        };
        assert_eq!(
            emitter.emit_instr(&br),
            "br i1 %c, label %then, label %else"
        );
        let ret = LlvmIrInstr::Ret {
            ty: LlvmIrType::I32,
            val: LlvmIrValue::ConstInt(0),
        };
        assert_eq!(emitter.emit_instr(&ret), "ret i32 0");
        let ret_void = LlvmIrInstr::RetVoid;
        assert_eq!(emitter.emit_instr(&ret_void), "ret void");
        let phi = LlvmIrInstr::Phi {
            dest: "v".to_string(),
            ty: LlvmIrType::I32,
            incoming: vec![
                (LlvmIrValue::ConstInt(0), "entry".to_string()),
                (LlvmIrValue::Register("r1".to_string()), "loop".to_string()),
            ],
        };
        assert_eq!(
            emitter.emit_instr(&phi),
            "%v = phi i32 [ 0, %entry ], [ %r1, %loop ]"
        );
    }
    #[test]
    pub(super) fn test_emit_call_instr() {
        let emitter = LlvmIrTextEmitter::new();
        let call = LlvmIrInstr::Call {
            dest: Some("result".to_string()),
            ret_ty: LlvmIrType::I32,
            func: LlvmIrValue::Global("printf".to_string()),
            args: vec![
                (LlvmIrType::Ptr, LlvmIrValue::Register("fmt".to_string())),
                (LlvmIrType::I32, LlvmIrValue::ConstInt(42)),
            ],
            tail: false,
            cc: CallingConv::C,
        };
        assert_eq!(
            emitter.emit_instr(&call),
            "%result = call i32 @printf(ptr %fmt, i32 42)"
        );
        let void_call = LlvmIrInstr::Call {
            dest: None,
            ret_ty: LlvmIrType::Void,
            func: LlvmIrValue::Global("exit".to_string()),
            args: vec![(LlvmIrType::I32, LlvmIrValue::ConstInt(0))],
            tail: true,
            cc: CallingConv::C,
        };
        assert_eq!(
            emitter.emit_instr(&void_call),
            "tail call void @exit(i32 0)"
        );
    }
    #[test]
    pub(super) fn test_emit_full_module() {
        let emitter = LlvmIrTextEmitter::new();
        let mut module = LlvmIrModule::new("test_module");
        module.set_target_triple("x86_64-pc-linux-gnu");
        module.set_data_layout(
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
        );
        module.add_global(LlvmIrGlobal::constant(
            "CONST",
            LlvmIrType::I32,
            LlvmIrValue::ConstInt(42),
        ));
        let mut func = LlvmIrFunction::new(
            "add",
            LlvmIrType::I32,
            vec![
                LlvmIrParam::new(LlvmIrType::I32, "a"),
                LlvmIrParam::new(LlvmIrType::I32, "b"),
            ],
        );
        let mut entry = LlvmIrBlock::new(
            "entry",
            LlvmIrInstr::Ret {
                ty: LlvmIrType::I32,
                val: LlvmIrValue::Register("sum".to_string()),
            },
        );
        entry.push(LlvmIrInstr::Add {
            dest: "sum".to_string(),
            ty: LlvmIrType::I32,
            lhs: LlvmIrValue::Register("a".to_string()),
            rhs: LlvmIrValue::Register("b".to_string()),
            nsw: false,
            nuw: false,
        });
        func.add_block(entry);
        module.add_function(func);
        let ir = emitter.emit(&module);
        assert!(ir.contains("target triple = \"x86_64-pc-linux-gnu\""));
        assert!(ir.contains("@CONST = constant i32 42"));
        assert!(ir.contains("define i32 @add(i32 %a, i32 %b)"));
        assert!(ir.contains("%sum = add i32 %a, %b"));
        assert!(ir.contains("ret i32 %sum"));
    }
    #[test]
    pub(super) fn test_register_allocator() {
        let mut alloc = RegisterAllocator::new();
        assert_eq!(alloc.next_reg(), "_r0");
        assert_eq!(alloc.next_reg(), "_r1");
        assert_eq!(alloc.next_reg(), "_r2");
        let named = alloc.named("tmp");
        assert_eq!(named, "tmp_3");
        alloc.reset();
        assert_eq!(alloc.next_reg(), "_r0");
    }
    #[test]
    pub(super) fn test_emit_conversion_instrs() {
        let emitter = LlvmIrTextEmitter::new();
        let zext = LlvmIrInstr::Zext {
            dest: "z".to_string(),
            val: LlvmIrValue::Register("x".to_string()),
            from_ty: LlvmIrType::I32,
            to_ty: LlvmIrType::I64,
        };
        assert_eq!(emitter.emit_instr(&zext), "%z = zext i32 %x to i64");
        let bitcast = LlvmIrInstr::Bitcast {
            dest: "bc".to_string(),
            val: LlvmIrValue::Register("p".to_string()),
            from_ty: LlvmIrType::Ptr,
            to_ty: LlvmIrType::Ptr,
        };
        assert_eq!(emitter.emit_instr(&bitcast), "%bc = bitcast ptr %p to ptr");
        let select = LlvmIrInstr::Select {
            dest: "s".to_string(),
            cond: LlvmIrValue::Register("flag".to_string()),
            ty: LlvmIrType::I32,
            true_val: LlvmIrValue::ConstInt(1),
            false_val: LlvmIrValue::ConstInt(0),
        };
        assert_eq!(
            emitter.emit_instr(&select),
            "%s = select i1 %flag, i32 1, i32 0"
        );
    }
}
#[cfg(test)]
mod LIT_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = LITPassConfig::new("test_pass", LITPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = LITPassStats::new();
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
        let mut reg = LITPassRegistry::new();
        reg.register(LITPassConfig::new("pass_a", LITPassPhase::Analysis));
        reg.register(LITPassConfig::new("pass_b", LITPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = LITAnalysisCache::new(10);
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
        let mut wl = LITWorklist::new();
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
        let mut dt = LITDominatorTree::new(5);
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
        let mut liveness = LITLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(LITConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(LITConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(LITConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            LITConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(LITConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = LITDepGraph::new();
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
