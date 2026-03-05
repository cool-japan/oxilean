//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use crate::native_backend::*;

use super::types::{
    AllocStrategy, AllocatorCodegen, ArrayCodegen, BigNatCodegen, ClosureCodegen, ClosureLayout,
    ClosureRepr, ExternalObjectCodegen, LayoutCache, LayoutComputer, ObjectLayout, ObjectTag,
    RcCodegen, RcStrategy, RcUseAnalysis, RuntimeConfig, RuntimeModuleBuilder, StringCodegen,
    StringLayout, ThunkCodegen, TypeInfo,
};

/// Align a value up to the given alignment.
pub(super) fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}
/// Whether an LCNF type requires boxing (heap allocation).
pub(super) fn is_boxed_type(ty: &LcnfType) -> bool {
    matches!(
        ty,
        LcnfType::Object
            | LcnfType::Var(_)
            | LcnfType::Fun(_, _)
            | LcnfType::Ctor(_, _)
            | LcnfType::LcnfString
    )
}
/// Size of a scalar (unboxed) LCNF type in bytes.
pub(super) fn scalar_type_size(ty: &LcnfType) -> usize {
    match ty {
        LcnfType::Nat => 8,
        LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => 0,
        _ => 8,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_object_tag_roundtrip() {
        for tag in [
            ObjectTag::Scalar,
            ObjectTag::Closure,
            ObjectTag::Array,
            ObjectTag::Struct,
            ObjectTag::External,
            ObjectTag::String,
            ObjectTag::BigNat,
            ObjectTag::Thunk,
        ] {
            let n = tag.to_u8();
            let back = ObjectTag::from_u8(n).expect("back should be valid");
            assert_eq!(tag, back);
        }
        assert!(ObjectTag::from_u8(255).is_none());
    }
    #[test]
    pub(super) fn test_object_tag_display() {
        assert_eq!(ObjectTag::Scalar.to_string(), "scalar");
        assert_eq!(ObjectTag::Closure.to_string(), "closure");
        assert_eq!(ObjectTag::Struct.to_string(), "struct");
    }
    #[test]
    pub(super) fn test_ctor_layout() {
        let layout = ObjectLayout::for_ctor(0, 2, 0);
        assert_eq!(layout.num_obj_fields, 2);
        assert_eq!(layout.scalar_size, 0);
        assert!(layout.total_size >= ObjectLayout::HEADER_SIZE + 16);
        assert_eq!(layout.alignment, 8);
        assert_eq!(layout.obj_field_offset(0), ObjectLayout::HEADER_SIZE);
        assert_eq!(layout.obj_field_offset(1), ObjectLayout::HEADER_SIZE + 8);
    }
    #[test]
    pub(super) fn test_closure_layout() {
        let layout = ObjectLayout::for_closure(2, 3);
        assert_eq!(layout.tag, ObjectTag::Closure);
        assert!(layout.total_size > ObjectLayout::HEADER_SIZE);
        assert_eq!(layout.num_obj_fields, 3);
    }
    #[test]
    pub(super) fn test_array_layout() {
        let layout = ObjectLayout::for_array(10);
        assert_eq!(layout.tag, ObjectTag::Array);
        assert_eq!(layout.num_obj_fields, 10);
    }
    #[test]
    pub(super) fn test_external_layout() {
        let layout = ObjectLayout::for_external();
        assert_eq!(layout.tag, ObjectTag::External);
        assert_eq!(layout.num_obj_fields, 0);
    }
    #[test]
    pub(super) fn test_layout_display() {
        let layout = ObjectLayout::for_ctor(0, 2, 8);
        let s = layout.to_string();
        assert!(s.contains("obj_fields=2"));
        assert!(s.contains("scalar=8"));
    }
    #[test]
    pub(super) fn test_align_up() {
        assert_eq!(align_up(0, 8), 0);
        assert_eq!(align_up(1, 8), 8);
        assert_eq!(align_up(8, 8), 8);
        assert_eq!(align_up(9, 8), 16);
        assert_eq!(align_up(16, 8), 16);
    }
    #[test]
    pub(super) fn test_layout_computer_ctor() {
        let mut computer = LayoutComputer::new();
        let layout = computer.compute_ctor_layout("Pair", 0, &[LcnfType::Nat, LcnfType::Object]);
        assert_eq!(layout.num_obj_fields, 1);
        assert!(layout.scalar_size > 0);
    }
    #[test]
    pub(super) fn test_layout_computer_cache() {
        let mut computer = LayoutComputer::new();
        let layout1 = computer.compute_ctor_layout("Nil", 0, &[]);
        let layout2 = computer.compute_ctor_layout("Nil", 0, &[]);
        assert_eq!(layout1, layout2);
    }
    #[test]
    pub(super) fn test_layout_computer_register_type() {
        let mut computer = LayoutComputer::new();
        computer.register_type(TypeInfo {
            name: "Bool".to_string(),
            constructors: vec![
                ("False".to_string(), 0, vec![]),
                ("True".to_string(), 1, vec![]),
            ],
            is_recursive: false,
        });
        let layouts = computer.compute_layout("Bool");
        assert_eq!(layouts.len(), 2);
    }
    #[test]
    pub(super) fn test_rc_codegen_inc() {
        let mut rc = RcCodegen::new(true);
        let insts = rc.emit_rc_inc(Register::virt(5));
        assert!(insts.len() >= 2);
        let has_call = insts.iter().any(|i| matches!(i, NativeInst::Call { .. }));
        assert!(has_call);
    }
    #[test]
    pub(super) fn test_rc_codegen_dec() {
        let mut rc = RcCodegen::new(true);
        let insts = rc.emit_rc_dec(Register::virt(5));
        assert!(insts.len() >= 2);
    }
    #[test]
    pub(super) fn test_rc_codegen_is_unique() {
        let mut rc = RcCodegen::new(true);
        let inst = rc.emit_rc_is_unique(Register::virt(5));
        assert!(matches!(inst, NativeInst::Call { .. }));
    }
    #[test]
    pub(super) fn test_rc_codegen_disabled() {
        let mut rc = RcCodegen::new(false);
        let insts = rc.emit_rc_inc(Register::virt(0));
        assert!(insts.len() == 1);
        assert!(matches!(insts[0], NativeInst::Comment(_)));
    }
    #[test]
    pub(super) fn test_rc_codegen_inc_n() {
        let mut rc = RcCodegen::new(true);
        let insts = rc.emit_rc_inc_n(Register::virt(5), 3);
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_allocator_codegen_system() {
        let mut alloc = AllocatorCodegen::new(AllocStrategy::System);
        let insts = alloc.emit_alloc(64, 8);
        assert!(insts.len() >= 2);
    }
    #[test]
    pub(super) fn test_allocator_codegen_free() {
        let mut alloc = AllocatorCodegen::new(AllocStrategy::System);
        let insts = alloc.emit_free(Register::virt(5));
        assert!(insts.len() >= 2);
    }
    #[test]
    pub(super) fn test_allocator_codegen_bump_no_free() {
        let mut alloc = AllocatorCodegen::new(AllocStrategy::Bump);
        let insts = alloc.emit_free(Register::virt(5));
        assert!(insts.len() == 1);
        assert!(matches!(insts[0], NativeInst::Comment(_)));
    }
    #[test]
    pub(super) fn test_allocator_alloc_ctor() {
        let mut alloc = AllocatorCodegen::new(AllocStrategy::LeanRuntime);
        let insts = alloc.emit_alloc_ctor(0, 2, 0);
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_allocator_alloc_closure() {
        let mut alloc = AllocatorCodegen::new(AllocStrategy::LeanRuntime);
        let insts = alloc.emit_alloc_closure("my_func", 2, 1);
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_closure_layout_new() {
        let layout = ClosureLayout::new(2, 3);
        assert_eq!(layout.arity, 2);
        assert_eq!(layout.num_captured, 3);
        assert!(layout.env_offset > ObjectLayout::HEADER_SIZE);
        assert_eq!(layout.env_size, 24);
    }
    #[test]
    pub(super) fn test_closure_layout_offsets() {
        let layout = ClosureLayout::new(1, 2);
        let off0 = layout.captured_var_offset(0);
        let off1 = layout.captured_var_offset(1);
        assert_eq!(off1 - off0, 8);
    }
    #[test]
    pub(super) fn test_closure_layout_display() {
        let layout = ClosureLayout::new(2, 3);
        let s = layout.to_string();
        assert!(s.contains("arity=2"));
        assert!(s.contains("captured=3"));
    }
    #[test]
    pub(super) fn test_closure_codegen_create() {
        let mut codegen = ClosureCodegen::new();
        let insts = codegen.emit_closure_create("add", 2, &[Register::virt(0), Register::virt(1)]);
        assert!(!insts.is_empty());
        let call_count = insts
            .iter()
            .filter(|i| matches!(i, NativeInst::Call { .. }))
            .count();
        assert!(call_count >= 3);
    }
    #[test]
    pub(super) fn test_closure_codegen_apply() {
        let mut codegen = ClosureCodegen::new();
        let insts = codegen.emit_closure_apply(Register::virt(0), &[Register::virt(1)]);
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_closure_codegen_partial_apply() {
        let mut codegen = ClosureCodegen::new();
        let insts = codegen.emit_partial_apply(Register::virt(0), &[Register::virt(1)]);
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_closure_codegen_partial_apply_empty() {
        let mut codegen = ClosureCodegen::new();
        let insts = codegen.emit_partial_apply(Register::virt(0), &[]);
        assert!(insts.len() == 1);
        assert!(matches!(insts[0], NativeInst::Comment(_)));
    }
    #[test]
    pub(super) fn test_runtime_config_default() {
        let cfg = RuntimeConfig::default();
        assert_eq!(cfg.rc_strategy, RcStrategy::Standard);
        assert_eq!(cfg.alloc_strategy, AllocStrategy::LeanRuntime);
        assert_eq!(cfg.closure_repr, ClosureRepr::Standard);
        assert!(!cfg.debug_checks);
    }
    #[test]
    pub(super) fn test_runtime_config_display() {
        let cfg = RuntimeConfig::default();
        let s = cfg.to_string();
        assert!(s.contains("Standard"));
        assert!(s.contains("LeanRuntime"));
    }
    #[test]
    pub(super) fn test_is_boxed_type() {
        assert!(is_boxed_type(&LcnfType::Object));
        assert!(is_boxed_type(&LcnfType::LcnfString));
        assert!(is_boxed_type(&LcnfType::Ctor("List".into(), vec![])));
        assert!(!is_boxed_type(&LcnfType::Nat));
        assert!(!is_boxed_type(&LcnfType::Unit));
    }
    #[test]
    pub(super) fn test_scalar_type_size() {
        assert_eq!(scalar_type_size(&LcnfType::Nat), 8);
        assert_eq!(scalar_type_size(&LcnfType::Unit), 0);
        assert_eq!(scalar_type_size(&LcnfType::Erased), 0);
    }
    #[test]
    pub(super) fn test_conditional_reset() {
        let mut rc = RcCodegen::new(true);
        let insts = rc.emit_conditional_reset(Register::virt(0), 2, 0);
        assert!(insts.len() >= 3);
    }
}
#[cfg(test)]
mod runtime_extended_tests {
    use super::*;
    pub(super) fn vid(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn mk_fun_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn mk_module(decls: Vec<LcnfFunDecl>) -> LcnfModule {
        LcnfModule {
            fun_decls: decls,
            extern_decls: vec![],
            name: "test".to_string(),
            metadata: LcnfModuleMetadata::default(),
        }
    }
    #[test]
    pub(super) fn test_string_layout_standard() {
        let layout = StringLayout::standard();
        assert!(!layout.is_sso);
        assert_eq!(layout.len_offset, ObjectLayout::HEADER_SIZE);
        assert!(layout.data_offset > layout.len_offset);
    }
    #[test]
    pub(super) fn test_string_layout_sso() {
        let layout = StringLayout::sso(15);
        assert!(layout.is_sso);
        assert_eq!(layout.sso_max_len, 15);
        assert!(layout.sso_total_size > 0);
    }
    #[test]
    pub(super) fn test_string_layout_alloc_size_standard() {
        let layout = StringLayout::standard();
        let size = layout.alloc_size(100);
        assert!(size >= layout.data_offset + 100);
    }
    #[test]
    pub(super) fn test_string_layout_alloc_size_sso_short() {
        let layout = StringLayout::sso(15);
        let size = layout.alloc_size(10);
        assert_eq!(size, layout.sso_total_size);
    }
    #[test]
    pub(super) fn test_string_layout_alloc_size_sso_long() {
        let layout = StringLayout::sso(15);
        let size_long = layout.alloc_size(100);
        assert!(size_long > layout.sso_total_size);
    }
    #[test]
    pub(super) fn test_string_layout_display() {
        let layout = StringLayout::sso(8);
        let s = layout.to_string();
        assert!(s.contains("sso=true"));
        assert!(s.contains("max_len=8"));
    }
    #[test]
    pub(super) fn test_array_codegen_alloc() {
        let mut codegen = ArrayCodegen::default();
        let insts = codegen.emit_alloc_array(16);
        assert!(!insts.is_empty());
        assert!(insts.iter().any(|i| matches!(i, NativeInst::Call { .. })));
    }
    #[test]
    pub(super) fn test_array_codegen_get() {
        let mut codegen = ArrayCodegen::default();
        let insts = codegen.emit_array_get(Register::virt(0), Register::virt(1));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_array_codegen_set() {
        let mut codegen = ArrayCodegen::default();
        let insts = codegen.emit_array_set(Register::virt(0), Register::virt(1), Register::virt(2));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_array_codegen_push() {
        let mut codegen = ArrayCodegen::default();
        let insts = codegen.emit_array_push(Register::virt(0), Register::virt(1));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_array_codegen_size() {
        let mut codegen = ArrayCodegen::default();
        let insts = codegen.emit_array_size(Register::virt(0));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_string_codegen_lit() {
        let mut codegen = StringCodegen::default();
        let insts = codegen.emit_string_lit("hello");
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_string_codegen_append() {
        let mut codegen = StringCodegen::default();
        let insts = codegen.emit_string_append(Register::virt(0), Register::virt(1));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_string_codegen_length() {
        let mut codegen = StringCodegen::default();
        let insts = codegen.emit_string_length(Register::virt(0));
        assert!(!insts.is_empty());
        assert!(insts.iter().any(|i| matches!(i, NativeInst::Load { .. })));
    }
    #[test]
    pub(super) fn test_string_codegen_eq() {
        let mut codegen = StringCodegen::default();
        let insts = codegen.emit_string_eq(Register::virt(0), Register::virt(1));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_thunk_codegen_alloc() {
        let mut codegen = ThunkCodegen::new();
        let insts = codegen.emit_alloc_thunk("my_lazy_fn");
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_thunk_codegen_force() {
        let mut codegen = ThunkCodegen::new();
        let insts = codegen.emit_force_thunk(Register::virt(5));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_thunk_codegen_is_evaluated() {
        let mut codegen = ThunkCodegen::new();
        let insts = codegen.emit_is_evaluated(Register::virt(3));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_bignat_codegen_add() {
        let mut codegen = BigNatCodegen::new();
        let insts = codegen.emit_add(Register::virt(0), Register::virt(1));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_bignat_codegen_mul() {
        let mut codegen = BigNatCodegen::new();
        let insts = codegen.emit_mul(Register::virt(0), Register::virt(1));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_bignat_codegen_sub() {
        let mut codegen = BigNatCodegen::new();
        let insts = codegen.emit_sub(Register::virt(0), Register::virt(1));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_bignat_codegen_div() {
        let mut codegen = BigNatCodegen::new();
        let insts = codegen.emit_div(Register::virt(0), Register::virt(1));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_bignat_codegen_cmp() {
        let mut codegen = BigNatCodegen::new();
        let insts = codegen.emit_cmp(Register::virt(0), Register::virt(1));
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_bignat_codegen_of_u64() {
        let mut codegen = BigNatCodegen::new();
        let insts = codegen.emit_of_u64(42);
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_external_codegen_alloc() {
        let mut codegen = ExternalObjectCodegen::new();
        let insts = codegen.emit_alloc_external(Register::virt(0), "my_finalizer");
        assert!(!insts.is_empty());
    }
    #[test]
    pub(super) fn test_external_codegen_get_data() {
        let mut codegen = ExternalObjectCodegen::new();
        let insts = codegen.emit_get_external_data(Register::virt(0));
        assert!(!insts.is_empty());
        assert!(insts.iter().any(|i| matches!(i, NativeInst::Load { .. })));
    }
    #[test]
    pub(super) fn test_runtime_module_builder_new() {
        let config = RuntimeConfig::default();
        let builder = RuntimeModuleBuilder::new(config);
        assert_eq!(builder.instruction_count(), 0);
    }
    #[test]
    pub(super) fn test_runtime_module_builder_emit_ctor() {
        let mut builder = RuntimeModuleBuilder::new(RuntimeConfig::default());
        builder.emit_ctor(0, 2, 0);
        assert!(builder.instruction_count() > 0);
    }
    #[test]
    pub(super) fn test_runtime_module_builder_emit_closure() {
        let mut builder = RuntimeModuleBuilder::new(RuntimeConfig::default());
        let env = vec![Register::virt(0), Register::virt(1)];
        builder.emit_closure("my_fn", 2, &env);
        assert!(builder.call_count() > 0);
    }
    #[test]
    pub(super) fn test_runtime_module_builder_emit_inc_dec() {
        let mut builder = RuntimeModuleBuilder::new(RuntimeConfig::default());
        builder.emit_inc(Register::virt(5));
        builder.emit_dec(Register::virt(5));
        assert!(builder.instruction_count() > 0);
    }
    #[test]
    pub(super) fn test_runtime_module_builder_emit_nat_add() {
        let mut builder = RuntimeModuleBuilder::new(RuntimeConfig::default());
        builder.emit_nat_add(Register::virt(0), Register::virt(1));
        assert!(!builder.instructions().is_empty());
    }
    #[test]
    pub(super) fn test_runtime_module_builder_emit_str_append() {
        let mut builder = RuntimeModuleBuilder::new(RuntimeConfig::default());
        builder.emit_str_append(Register::virt(0), Register::virt(1));
        assert!(!builder.instructions().is_empty());
    }
    #[test]
    pub(super) fn test_runtime_module_builder_comment_count() {
        let mut builder = RuntimeModuleBuilder::new(RuntimeConfig::default());
        builder.emit_ctor(0, 1, 0);
        assert!(builder.comment_count() > 0);
    }
    #[test]
    pub(super) fn test_rc_use_analysis_new() {
        let analysis = RcUseAnalysis::new();
        assert_eq!(analysis.use_count(vid(0)), 0);
    }
    #[test]
    pub(super) fn test_rc_use_analysis_single_use() {
        let mut analysis = RcUseAnalysis::new();
        let module = mk_module(vec![mk_fun_decl(
            "f",
            LcnfExpr::Return(LcnfArg::Var(vid(5))),
        )]);
        analysis.analyze_module(&module);
        assert_eq!(analysis.use_count(vid(5)), 1);
    }
    #[test]
    pub(super) fn test_rc_use_analysis_multi_use() {
        let mut analysis = RcUseAnalysis::new();
        let module = mk_module(vec![mk_fun_decl(
            "f",
            LcnfExpr::TailCall(
                LcnfArg::Var(vid(1)),
                vec![LcnfArg::Var(vid(1)), LcnfArg::Var(vid(2))],
            ),
        )]);
        analysis.analyze_module(&module);
        assert_eq!(analysis.use_count(vid(1)), 2);
        assert_eq!(analysis.use_count(vid(2)), 1);
    }
    #[test]
    pub(super) fn test_rc_use_analysis_multi_use_vars() {
        let mut analysis = RcUseAnalysis::new();
        analysis.analyze_module(&mk_module(vec![mk_fun_decl(
            "f",
            LcnfExpr::TailCall(
                LcnfArg::Var(vid(3)),
                vec![LcnfArg::Var(vid(3)), LcnfArg::Var(vid(3))],
            ),
        )]));
        let multi = analysis.multi_use_vars();
        assert!(multi.iter().any(|(v, _)| *v == vid(3)));
    }
    #[test]
    pub(super) fn test_layout_cache_new() {
        let cache = LayoutCache::new();
        assert_eq!(cache.ctor_count(), 0);
        assert_eq!(cache.closure_count(), 0);
    }
    #[test]
    pub(super) fn test_layout_cache_get_ctor() {
        let mut cache = LayoutCache::new();
        let layout = cache.get_ctor("Pair", 0, 2, 0);
        assert_eq!(layout.num_obj_fields, 2);
        assert_eq!(cache.ctor_count(), 1);
        let layout2 = cache.get_ctor("Pair", 0, 2, 0);
        assert_eq!(layout2.num_obj_fields, 2);
        assert_eq!(cache.ctor_count(), 1);
    }
    #[test]
    pub(super) fn test_layout_cache_get_closure() {
        let mut cache = LayoutCache::new();
        let layout = cache.get_closure(2, 3);
        assert_eq!(layout.arity, 2);
        assert_eq!(layout.num_captured, 3);
        assert_eq!(cache.closure_count(), 1);
        let layout2 = cache.get_closure(2, 3);
        assert_eq!(layout2.arity, 2);
        assert_eq!(cache.closure_count(), 1);
    }
    #[test]
    pub(super) fn test_layout_cache_clear() {
        let mut cache = LayoutCache::new();
        cache.get_ctor("Nil", 0, 0, 0);
        cache.get_closure(1, 1);
        cache.clear();
        assert_eq!(cache.ctor_count(), 0);
        assert_eq!(cache.closure_count(), 0);
    }
    #[test]
    pub(super) fn test_object_tag_invalid_from_u8() {
        assert!(ObjectTag::from_u8(8).is_none());
        assert!(ObjectTag::from_u8(100).is_none());
    }
    #[test]
    pub(super) fn test_object_tag_all_values() {
        let tags: Vec<ObjectTag> = (0u8..8).filter_map(ObjectTag::from_u8).collect();
        assert_eq!(tags.len(), 8);
    }
    #[test]
    pub(super) fn test_object_layout_scalar_offset() {
        let layout = ObjectLayout::for_ctor(0, 3, 16);
        let scalar_off = layout.scalar_offset();
        assert_eq!(scalar_off, ObjectLayout::HEADER_SIZE + 3 * 8);
    }
    #[test]
    pub(super) fn test_object_layout_for_external_size() {
        let layout = ObjectLayout::for_external();
        assert!(layout.total_size >= ObjectLayout::HEADER_SIZE + 24);
    }
    #[test]
    pub(super) fn test_closure_codegen_apply_5_args() {
        let mut codegen = ClosureCodegen::new();
        let args: Vec<Register> = (1..=5).map(Register::virt).collect();
        let insts = codegen.emit_closure_apply(Register::virt(0), &args);
        assert!(!insts.is_empty());
        let has_apply_n = insts.iter().any(|i| {
            if let NativeInst::Call {
                func: NativeValue::FRef(name),
                ..
            } = i
            {
                name.contains("lean_apply_n")
            } else {
                false
            }
        });
        assert!(has_apply_n);
    }
    #[test]
    pub(super) fn test_alloc_strategy_pool() {
        let mut alloc = AllocatorCodegen::new(AllocStrategy::Pool);
        let insts = alloc.emit_free(Register::virt(0));
        assert!(insts.iter().any(|i| matches!(i, NativeInst::Call { .. })));
    }
    #[test]
    pub(super) fn test_alloc_strategy_bump_alloc() {
        let mut alloc = AllocatorCodegen::new(AllocStrategy::Bump);
        let insts = alloc.emit_alloc(64, 8);
        assert!(insts.iter().any(|i| {
            if let NativeInst::Call {
                func: NativeValue::FRef(name),
                ..
            } = i
            {
                name == "bump_alloc"
            } else {
                false
            }
        }));
    }
}
