//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::types::{
    CILAnalysisCache, CILConstantFoldingHelper, CILDepGraph, CILDominatorTree, CILLivenessInfo,
    CILPassConfig, CILPassPhase, CILPassRegistry, CILPassStats, CILWorklist, CilAssembly,
    CilBackend, CilClass, CilExtConfig, CilExtDiagCollector, CilExtDiagMsg, CilExtEmitStats,
    CilExtEventLog, CilExtFeatures, CilExtIdGen, CilExtIncrKey, CilExtNameScope, CilExtPassTiming,
    CilExtProfiler, CilExtSourceBuffer, CilExtVersion, CilInstr, CilLiteral, CilMethod, CilType,
};

/// Format a single CIL instruction as IL assembly text.
pub fn emit_cil_instr(instr: &CilInstr) -> std::string::String {
    match instr {
        CilInstr::Nop => "nop".to_string(),
        CilInstr::LdlocS(i) => format!("ldloc.s {}", i),
        CilInstr::StlocS(i) => format!("stloc.s {}", i),
        CilInstr::LdlocaS(i) => format!("ldloca.s {}", i),
        CilInstr::LdargS(i) => format!("ldarg.s {}", i),
        CilInstr::StargS(i) => format!("starg.s {}", i),
        CilInstr::LdargaS(i) => format!("ldarga.s {}", i),
        CilInstr::LdcI4(n) => format!("ldc.i4 {}", n),
        CilInstr::LdcI4S(n) => format!("ldc.i4.s {}", n),
        CilInstr::LdcI4Small(n) => format!("ldc.i4.{}", n),
        CilInstr::LdcI8(n) => format!("ldc.i8 {}", n),
        CilInstr::LdcR4(v) => format!("ldc.r4 {}", v),
        CilInstr::LdcR8(v) => format!("ldc.r8 {}", v),
        CilInstr::Ldnull => "ldnull".to_string(),
        CilInstr::Ldstr(s) => format!("ldstr \"{}\"", s.replace('"', "\\\"")),
        CilInstr::Ldsflda(f) => format!("ldsflda {}", f),
        CilInstr::Ldsfld(f) => format!("ldsfld {}", f),
        CilInstr::Stsfld(f) => format!("stsfld {}", f),
        CilInstr::Add => "add".to_string(),
        CilInstr::AddOvf => "add.ovf".to_string(),
        CilInstr::Sub => "sub".to_string(),
        CilInstr::SubOvf => "sub.ovf".to_string(),
        CilInstr::Mul => "mul".to_string(),
        CilInstr::MulOvf => "mul.ovf".to_string(),
        CilInstr::Div => "div".to_string(),
        CilInstr::DivUn => "div.un".to_string(),
        CilInstr::Rem => "rem".to_string(),
        CilInstr::RemUn => "rem.un".to_string(),
        CilInstr::Neg => "neg".to_string(),
        CilInstr::And => "and".to_string(),
        CilInstr::Or => "or".to_string(),
        CilInstr::Xor => "xor".to_string(),
        CilInstr::Not => "not".to_string(),
        CilInstr::Shl => "shl".to_string(),
        CilInstr::Shr => "shr".to_string(),
        CilInstr::ShrUn => "shr.un".to_string(),
        CilInstr::Ceq => "ceq".to_string(),
        CilInstr::Cgt => "cgt".to_string(),
        CilInstr::CgtUn => "cgt.un".to_string(),
        CilInstr::Clt => "clt".to_string(),
        CilInstr::CltUn => "clt.un".to_string(),
        CilInstr::Br(lbl) => format!("br {}", lbl),
        CilInstr::Brfalse(lbl) => format!("brfalse {}", lbl),
        CilInstr::Brtrue(lbl) => format!("brtrue {}", lbl),
        CilInstr::Beq(lbl) => format!("beq {}", lbl),
        CilInstr::BneUn(lbl) => format!("bne.un {}", lbl),
        CilInstr::Blt(lbl) => format!("blt {}", lbl),
        CilInstr::Bgt(lbl) => format!("bgt {}", lbl),
        CilInstr::Ble(lbl) => format!("ble {}", lbl),
        CilInstr::Bge(lbl) => format!("bge {}", lbl),
        CilInstr::Switch(labels) => format!("switch ({})", labels.join(", ")),
        CilInstr::Ret => "ret".to_string(),
        CilInstr::Throw => "throw".to_string(),
        CilInstr::Rethrow => "rethrow".to_string(),
        CilInstr::Label(lbl) => format!("{}:", lbl),
        CilInstr::Call(m) => format!("call {}", m),
        CilInstr::Callvirt(m) => format!("callvirt {}", m),
        CilInstr::TailCall(m) => format!("tail. call {}", m),
        CilInstr::Calli(sig) => format!("calli {}", sig),
        CilInstr::Ldftn(m) => format!("ldftn {}", m),
        CilInstr::Ldvirtftn(m) => format!("ldvirtftn {}", m),
        CilInstr::Newobj(m) => format!("newobj {}", m),
        CilInstr::Ldobj(t) => format!("ldobj {}", t),
        CilInstr::Stobj(t) => format!("stobj {}", t),
        CilInstr::Ldfld(fld) => format!("ldfld {}", fld),
        CilInstr::Stfld(fld) => format!("stfld {}", fld),
        CilInstr::Ldflda(fld) => format!("ldflda {}", fld),
        CilInstr::Box_(t) => format!("box {}", t),
        CilInstr::Unbox(t) => format!("unbox {}", t),
        CilInstr::UnboxAny(t) => format!("unbox.any {}", t),
        CilInstr::Isinst(t) => format!("isinst {}", t),
        CilInstr::Castclass(t) => format!("castclass {}", t),
        CilInstr::Initobj(t) => format!("initobj {}", t),
        CilInstr::Sizeof(t) => format!("sizeof {}", t),
        CilInstr::Ldtoken(t) => format!("ldtoken {}", t),
        CilInstr::Newarr(t) => format!("newarr {}", t),
        CilInstr::Ldlen => "ldlen".to_string(),
        CilInstr::Ldelem(t) => format!("ldelem {}", t),
        CilInstr::Stelem(t) => format!("stelem {}", t),
        CilInstr::Ldelema(t) => format!("ldelema {}", t),
        CilInstr::Dup => "dup".to_string(),
        CilInstr::Pop => "pop".to_string(),
        CilInstr::ConvI4 => "conv.i4".to_string(),
        CilInstr::ConvI8 => "conv.i8".to_string(),
        CilInstr::ConvR4 => "conv.r4".to_string(),
        CilInstr::ConvR8 => "conv.r8".to_string(),
        CilInstr::ConvU4 => "conv.u4".to_string(),
        CilInstr::ConvU8 => "conv.u8".to_string(),
        CilInstr::LdindI4 => "ldind.i4".to_string(),
        CilInstr::StindI4 => "stind.i4".to_string(),
        CilInstr::Localloc => "localloc".to_string(),
        CilInstr::Comment(s) => format!("// {}", s),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn make_backend() -> CilBackend {
        CilBackend::new("TestAssembly")
    }
    #[test]
    pub(super) fn test_cil_type_display_primitives() {
        assert_eq!(CilType::Void.to_string(), "void");
        assert_eq!(CilType::Bool.to_string(), "bool");
        assert_eq!(CilType::Int32.to_string(), "int32");
        assert_eq!(CilType::Int64.to_string(), "int64");
        assert_eq!(CilType::Float32.to_string(), "float32");
        assert_eq!(CilType::Float64.to_string(), "float64");
        assert_eq!(CilType::String.to_string(), "string");
        assert_eq!(CilType::Object.to_string(), "object");
    }
    #[test]
    pub(super) fn test_cil_type_display_class() {
        let ty = CilType::class_in("System.Collections", "ArrayList");
        assert_eq!(ty.to_string(), "class System.Collections.ArrayList");
    }
    #[test]
    pub(super) fn test_cil_type_display_array() {
        let ty = CilType::Array(Box::new(CilType::Int32));
        assert_eq!(ty.to_string(), "int32[]");
    }
    #[test]
    pub(super) fn test_cil_type_is_value_type() {
        assert!(CilType::Int32.is_value_type());
        assert!(CilType::Bool.is_value_type());
        assert!(CilType::Float64.is_value_type());
        assert!(!CilType::String.is_value_type());
        assert!(!CilType::Object.is_value_type());
    }
    #[test]
    pub(super) fn test_cil_assembly_creation() {
        let mut asm = CilAssembly::new("MyApp");
        asm.set_entry_point("Program", "Main");
        assert_eq!(asm.name, "MyApp");
        assert_eq!(
            asm.entry_point,
            Some(("Program".to_string(), "Main".to_string()))
        );
        assert_eq!(asm.version, (1, 0, 0, 0));
    }
    #[test]
    pub(super) fn test_cil_class_full_name() {
        let c = CilClass::new("MyNamespace", "MyClass");
        assert_eq!(c.full_name(), "MyNamespace.MyClass");
        let c2 = CilClass::new("", "Bare");
        assert_eq!(c2.full_name(), "Bare");
    }
    #[test]
    pub(super) fn test_cil_method_add_local() {
        let mut m = CilMethod::new_static("foo", CilType::Int32);
        let idx = m.add_local(CilType::Int32, Some("x".to_string()));
        assert_eq!(idx, 0);
        let idx2 = m.add_local(CilType::Bool, None);
        assert_eq!(idx2, 1);
        assert_eq!(m.locals.len(), 2);
    }
    #[test]
    pub(super) fn test_emit_cil_instr_basics() {
        assert_eq!(emit_cil_instr(&CilInstr::Nop), "nop");
        assert_eq!(emit_cil_instr(&CilInstr::Ret), "ret");
        assert_eq!(emit_cil_instr(&CilInstr::Add), "add");
        assert_eq!(emit_cil_instr(&CilInstr::LdcI4(42)), "ldc.i4 42");
        assert_eq!(
            emit_cil_instr(&CilInstr::Ldstr("hello".to_string())),
            "ldstr \"hello\""
        );
        assert_eq!(emit_cil_instr(&CilInstr::Dup), "dup");
        assert_eq!(emit_cil_instr(&CilInstr::Pop), "pop");
    }
    #[test]
    pub(super) fn test_emit_ilasm_contains_assembly_name() {
        let mut backend = make_backend();
        let mut class = CilClass::new("Test", "Program");
        let mut method = CilMethod::new_static("Main", CilType::Void);
        method.emit(CilInstr::Ret);
        class.add_method(method);
        backend.assembly.add_class(class);
        backend.assembly.set_entry_point("Test.Program", "Main");
        let il = backend.emit_ilasm();
        assert!(il.contains("TestAssembly"));
        assert!(il.contains("Program"));
        assert!(il.contains("Main"));
    }
    #[test]
    pub(super) fn test_lcnf_to_cil_type() {
        let backend = make_backend();
        assert_eq!(backend.lcnf_to_cil_type(&LcnfType::Nat), CilType::UInt64);
        assert_eq!(
            backend.lcnf_to_cil_type(&LcnfType::Var("Bool".to_string())),
            CilType::Bool
        );
        assert_eq!(
            backend.lcnf_to_cil_type(&LcnfType::LcnfString),
            CilType::String
        );
        assert_eq!(backend.lcnf_to_cil_type(&LcnfType::Erased), CilType::Object);
        assert_eq!(backend.lcnf_to_cil_type(&LcnfType::Unit), CilType::Void);
    }
    #[test]
    pub(super) fn test_cil_literal_display() {
        assert_eq!(CilLiteral::Bool(true).to_string(), "true");
        assert_eq!(CilLiteral::Int32(42).to_string(), "42");
        assert_eq!(CilLiteral::Null.to_string(), "null");
        assert_eq!(CilLiteral::String("hi".to_string()).to_string(), "\"hi\"");
    }
}
#[cfg(test)]
mod tests_cil_ext_extra {
    use super::*;
    #[test]
    pub(super) fn test_cil_ext_config() {
        let mut cfg = CilExtConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_cil_ext_source_buffer() {
        let mut buf = CilExtSourceBuffer::new();
        buf.push_line("fn main() {");
        buf.indent();
        buf.push_line("println!(\"hello\");");
        buf.dedent();
        buf.push_line("}");
        assert!(buf.as_str().contains("fn main()"));
        assert!(buf.as_str().contains("    println!"));
        assert_eq!(buf.line_count(), 3);
        buf.reset();
        assert!(buf.is_empty());
    }
    #[test]
    pub(super) fn test_cil_ext_name_scope() {
        let mut scope = CilExtNameScope::new();
        assert!(scope.declare("x"));
        assert!(!scope.declare("x"));
        assert!(scope.is_declared("x"));
        let scope = scope.push_scope();
        assert_eq!(scope.depth(), 1);
        let mut scope = scope.pop_scope();
        assert_eq!(scope.depth(), 0);
        scope.declare("y");
        assert_eq!(scope.len(), 2);
    }
    #[test]
    pub(super) fn test_cil_ext_diag_collector() {
        let mut col = CilExtDiagCollector::new();
        col.emit(CilExtDiagMsg::warning("pass_a", "slow"));
        col.emit(CilExtDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_cil_ext_id_gen() {
        let mut gen = CilExtIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_cil_ext_incr_key() {
        let k1 = CilExtIncrKey::new(100, 200);
        let k2 = CilExtIncrKey::new(100, 200);
        let k3 = CilExtIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_cil_ext_profiler() {
        let mut p = CilExtProfiler::new();
        p.record(CilExtPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(CilExtPassTiming::new("pass_b", 500, 30, 100, 200));
        assert_eq!(p.total_elapsed_us(), 1500);
        assert_eq!(
            p.slowest_pass()
                .expect("slowest pass should exist")
                .pass_name,
            "pass_a"
        );
        assert_eq!(p.profitable_passes().len(), 1);
    }
    #[test]
    pub(super) fn test_cil_ext_event_log() {
        let mut log = CilExtEventLog::new(3);
        log.push("event1");
        log.push("event2");
        log.push("event3");
        assert_eq!(log.len(), 3);
        log.push("event4");
        assert_eq!(log.len(), 3);
        assert_eq!(
            log.iter()
                .next()
                .expect("iterator should have next element"),
            "event2"
        );
    }
    #[test]
    pub(super) fn test_cil_ext_version() {
        let v = CilExtVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = CilExtVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&CilExtVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&CilExtVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_cil_ext_features() {
        let mut f = CilExtFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = CilExtFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_cil_ext_emit_stats() {
        let mut s = CilExtEmitStats::new();
        s.bytes_emitted = 50_000;
        s.items_emitted = 500;
        s.elapsed_ms = 100;
        assert!(s.is_clean());
        assert!((s.throughput_bps() - 500_000.0).abs() < 1.0);
        let disp = format!("{}", s);
        assert!(disp.contains("bytes=50000"));
    }
}
#[cfg(test)]
mod CIL_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = CILPassConfig::new("test_pass", CILPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = CILPassStats::new();
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
        let mut reg = CILPassRegistry::new();
        reg.register(CILPassConfig::new("pass_a", CILPassPhase::Analysis));
        reg.register(CILPassConfig::new("pass_b", CILPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = CILAnalysisCache::new(10);
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
        let mut wl = CILWorklist::new();
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
        let mut dt = CILDominatorTree::new(5);
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
        let mut liveness = CILLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(CILConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(CILConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(CILConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            CILConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(CILConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = CILDepGraph::new();
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
