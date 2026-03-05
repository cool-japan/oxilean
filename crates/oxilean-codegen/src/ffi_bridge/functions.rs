//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{BTreeMap, HashMap};

use super::types::{
    CallingConvention, FFIAnalysisCache, FFIConstantFoldingHelper, FFIDepGraph, FFIDominatorTree,
    FFILivenessInfo, FFIPassConfig, FFIPassPhase, FFIPassRegistry, FFIPassStats, FFIWorklist,
    FfiBridge, FfiDecl, FfiError, FfiExtConfig, FfiExtDiagCollector, FfiExtDiagMsg,
    FfiExtEmitStats, FfiExtEventLog, FfiExtFeatures, FfiExtIdGen, FfiExtIncrKey, FfiExtNameScope,
    FfiExtPassTiming, FfiExtProfiler, FfiExtSourceBuffer, FfiExtVersion, FfiMarshalInfo,
    FfiNativeType, FfiOutput, FfiTypeMap,
};

/// Determine how to marshal an LCNF type across the FFI boundary.
pub fn marshal_type(lcnf_ty: &LcnfType) -> FfiMarshalInfo {
    match lcnf_ty {
        LcnfType::Nat => FfiMarshalInfo::with_conversion(
            FfiNativeType::U64,
            "lean_unbox(${arg})",
            "lean_box(${result})",
        ),
        LcnfType::LcnfString => FfiMarshalInfo {
            native_type: FfiNativeType::CStr,
            to_native: "lean_string_cstr(${arg})".to_string(),
            from_native: "lean_mk_string(${result})".to_string(),
            is_trivial: false,
            needs_free: false,
        },
        LcnfType::Object => FfiMarshalInfo::trivial(FfiNativeType::LeanObject),
        LcnfType::Var(_) => FfiMarshalInfo::trivial(FfiNativeType::LeanObject),
        LcnfType::Fun(_, _) => FfiMarshalInfo::trivial(FfiNativeType::LeanObject),
        LcnfType::Ctor(_, _) => FfiMarshalInfo::trivial(FfiNativeType::LeanObject),
        LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => {
            FfiMarshalInfo::trivial(FfiNativeType::Void)
        }
    }
}
/// Validate an FFI declaration for ABI compatibility.
pub fn validate_ffi_decl(decl: &FfiDecl) -> Result<(), FfiError> {
    for (pname, pty) in &decl.params {
        validate_ffi_type(pty, pname)?;
    }
    validate_ffi_type(&decl.ret_type, "return")?;
    for (pname, pty) in &decl.params {
        if let FfiNativeType::Struct(name, fields) = pty {
            let size: usize = fields.iter().map(|(_, ty)| ty.size_bytes()).sum();
            if size > 128 {
                return Err(FfiError::StructTooLarge {
                    name: name.clone(),
                    size,
                });
            }
            for (fname, fty) in fields {
                validate_ffi_type(fty, &format!("{}.{}", pname, fname))?;
            }
        }
    }
    Ok(())
}
/// Validate a single FFI type.
pub(super) fn validate_ffi_type(ty: &FfiNativeType, _context: &str) -> Result<(), FfiError> {
    match ty {
        FfiNativeType::Void
        | FfiNativeType::I8
        | FfiNativeType::I16
        | FfiNativeType::I32
        | FfiNativeType::I64
        | FfiNativeType::U8
        | FfiNativeType::U16
        | FfiNativeType::U32
        | FfiNativeType::U64
        | FfiNativeType::F32
        | FfiNativeType::F64
        | FfiNativeType::Bool
        | FfiNativeType::SizeT
        | FfiNativeType::CStr
        | FfiNativeType::OpaquePtr
        | FfiNativeType::LeanObject => Ok(()),
        FfiNativeType::Ptr(inner) => validate_ffi_type(inner, _context),
        FfiNativeType::Struct(_, fields) => {
            for (_, fty) in fields {
                validate_ffi_type(fty, _context)?;
            }
            Ok(())
        }
        FfiNativeType::FnPtr(params, ret) => {
            for p in params {
                validate_ffi_type(p, _context)?;
            }
            validate_ffi_type(ret, _context)
        }
    }
}
/// Generate marshalling code to convert a native C value to a Lean object.
pub(super) fn marshal_native_to_lean(native_ty: &FfiNativeType, var_name: &str) -> FfiMarshalInfo {
    match native_ty {
        FfiNativeType::U64 | FfiNativeType::SizeT => FfiMarshalInfo::with_conversion(
            FfiNativeType::LeanObject,
            &format!("lean_box({})", var_name),
            "lean_unbox(${result})",
        ),
        FfiNativeType::I64 => FfiMarshalInfo::with_conversion(
            FfiNativeType::LeanObject,
            &format!("lean_box((size_t){})", var_name),
            "lean_unbox(${result})".to_string().as_str(),
        ),
        FfiNativeType::CStr => FfiMarshalInfo::with_conversion(
            FfiNativeType::LeanObject,
            &format!("lean_mk_string({})", var_name),
            "lean_string_cstr(${result})".to_string().as_str(),
        ),
        FfiNativeType::Bool => FfiMarshalInfo::with_conversion(
            FfiNativeType::LeanObject,
            &format!("lean_box({} ? 1 : 0)", var_name),
            "lean_unbox(${result}) != 0".to_string().as_str(),
        ),
        FfiNativeType::LeanObject => FfiMarshalInfo::trivial(FfiNativeType::LeanObject),
        FfiNativeType::OpaquePtr => FfiMarshalInfo::with_conversion(
            FfiNativeType::LeanObject,
            &format!("lean_alloc_external(NULL, NULL, (void*){})", var_name),
            "lean_get_external_data(${result})".to_string().as_str(),
        ),
        _ => FfiMarshalInfo::with_conversion(
            FfiNativeType::LeanObject,
            &format!("lean_box((size_t){})", var_name),
            "lean_unbox(${result})".to_string().as_str(),
        ),
    }
}
/// Generate marshalling code to convert a Lean object to a native C value.
pub(super) fn marshal_lean_to_native(native_ty: &FfiNativeType, var_name: &str) -> FfiMarshalInfo {
    match native_ty {
        FfiNativeType::U64 | FfiNativeType::SizeT => FfiMarshalInfo::with_conversion(
            native_ty.clone(),
            "lean_box(${arg})",
            &format!("lean_unbox({})", var_name),
        ),
        FfiNativeType::I64 => FfiMarshalInfo::with_conversion(
            native_ty.clone(),
            "lean_box((size_t)${arg})".to_string().as_str(),
            &format!("(int64_t)lean_unbox({})", var_name),
        ),
        FfiNativeType::CStr => FfiMarshalInfo {
            native_type: native_ty.clone(),
            to_native: "lean_mk_string(${arg})".to_string(),
            from_native: format!("lean_string_cstr({})", var_name),
            is_trivial: false,
            needs_free: false,
        },
        FfiNativeType::Bool => FfiMarshalInfo::with_conversion(
            native_ty.clone(),
            "lean_box(${arg} ? 1 : 0)".to_string().as_str(),
            &format!("lean_unbox({}) != 0", var_name),
        ),
        FfiNativeType::LeanObject => FfiMarshalInfo::trivial(FfiNativeType::LeanObject),
        FfiNativeType::Void => FfiMarshalInfo::trivial(FfiNativeType::Void),
        _ => FfiMarshalInfo::with_conversion(
            native_ty.clone(),
            "lean_box((size_t)${arg})".to_string().as_str(),
            &format!("({})(lean_unbox({}))", native_ty, var_name),
        ),
    }
}
/// Mangle a Lean/OxiLean name to a valid C identifier.
pub(super) fn mangle_lean_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 8);
    result.push_str("l_");
    for c in name.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => result.push(c),
            '.' => result.push_str("__"),
            _ => result.push_str(&format!("_u{:04x}_", c as u32)),
        }
    }
    result
}
/// Convert LCNF extern declarations to FFI declarations.
pub fn extern_decls_to_ffi(extern_decls: &[LcnfExternDecl]) -> Vec<FfiDecl> {
    extern_decls
        .iter()
        .map(|ext| {
            let params: Vec<(String, FfiNativeType)> = ext
                .params
                .iter()
                .filter(|p| !p.erased)
                .map(|p| {
                    let marshal = marshal_type(&p.ty);
                    (p.name.clone(), marshal.native_type)
                })
                .collect();
            let ret_marshal = marshal_type(&ext.ret_type);
            FfiDecl {
                name: ext.name.clone(),
                extern_name: mangle_lean_name(&ext.name),
                params,
                ret_type: ret_marshal.native_type,
                calling_conv: CallingConvention::C,
                is_unsafe: true,
            }
        })
        .collect()
}
/// Generate FFI bindings for an LCNF module's extern declarations.
pub fn generate_module_ffi(module: &LcnfModule) -> FfiOutput {
    let ffi_decls = extern_decls_to_ffi(&module.extern_decls);
    let bridge = FfiBridge::new();
    bridge.generate_bindings(&ffi_decls)
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn mk_ffi_decl(
        name: &str,
        params: Vec<(&str, FfiNativeType)>,
        ret: FfiNativeType,
    ) -> FfiDecl {
        FfiDecl {
            name: name.to_string(),
            extern_name: mangle_lean_name(name),
            params: params
                .into_iter()
                .map(|(n, t)| (n.to_string(), t))
                .collect(),
            ret_type: ret,
            calling_conv: CallingConvention::C,
            is_unsafe: true,
        }
    }
    #[test]
    pub(super) fn test_calling_convention_display() {
        assert_eq!(CallingConvention::C.to_string(), "\"C\"");
        assert_eq!(CallingConvention::Rust.to_string(), "\"Rust\"");
        assert_eq!(CallingConvention::System.to_string(), "\"system\"");
    }
    #[test]
    pub(super) fn test_ffi_native_type_display() {
        assert_eq!(FfiNativeType::Void.to_string(), "void");
        assert_eq!(FfiNativeType::I64.to_string(), "int64_t");
        assert_eq!(FfiNativeType::U64.to_string(), "uint64_t");
        assert_eq!(FfiNativeType::CStr.to_string(), "const char*");
        assert_eq!(FfiNativeType::LeanObject.to_string(), "lean_object*");
    }
    #[test]
    pub(super) fn test_ffi_native_type_to_rust() {
        assert_eq!(FfiNativeType::I64.to_rust_type(), "i64");
        assert_eq!(FfiNativeType::U64.to_rust_type(), "u64");
        assert_eq!(FfiNativeType::Bool.to_rust_type(), "bool");
        assert_eq!(FfiNativeType::Void.to_rust_type(), "()");
        assert_eq!(FfiNativeType::F64.to_rust_type(), "f64");
    }
    #[test]
    pub(super) fn test_ffi_native_type_size() {
        assert_eq!(FfiNativeType::Void.size_bytes(), 0);
        assert_eq!(FfiNativeType::I8.size_bytes(), 1);
        assert_eq!(FfiNativeType::I32.size_bytes(), 4);
        assert_eq!(FfiNativeType::I64.size_bytes(), 8);
        assert_eq!(
            FfiNativeType::Ptr(Box::new(FfiNativeType::I32)).size_bytes(),
            8
        );
    }
    #[test]
    pub(super) fn test_generate_extern_block() {
        let decls = vec![mk_ffi_decl(
            "math.sqrt",
            vec![("x", FfiNativeType::F64)],
            FfiNativeType::F64,
        )];
        let bridge = FfiBridge::new();
        let code = bridge.generate_extern_block(&decls);
        assert!(code.contains("extern \"C\""));
        assert!(code.contains("fn l_math__sqrt"));
        assert!(code.contains("f64"));
    }
    #[test]
    pub(super) fn test_generate_bindings() {
        let decls = vec![mk_ffi_decl(
            "puts",
            vec![("s", FfiNativeType::CStr)],
            FfiNativeType::I32,
        )];
        let bridge = FfiBridge::new();
        let output = bridge.generate_bindings(&decls);
        assert!(!output.rust_bindings.is_empty());
        assert!(!output.c_header.is_empty());
        assert_eq!(output.wrapper_fns.len(), 1);
    }
    #[test]
    pub(super) fn test_marshal_type_nat() {
        let info = marshal_type(&LcnfType::Nat);
        assert_eq!(info.native_type, FfiNativeType::U64);
        assert!(!info.is_trivial);
    }
    #[test]
    pub(super) fn test_marshal_type_string() {
        let info = marshal_type(&LcnfType::LcnfString);
        assert_eq!(info.native_type, FfiNativeType::CStr);
        assert!(!info.is_trivial);
    }
    #[test]
    pub(super) fn test_marshal_type_object() {
        let info = marshal_type(&LcnfType::Object);
        assert_eq!(info.native_type, FfiNativeType::LeanObject);
        assert!(info.is_trivial);
    }
    #[test]
    pub(super) fn test_marshal_type_unit() {
        let info = marshal_type(&LcnfType::Unit);
        assert_eq!(info.native_type, FfiNativeType::Void);
        assert!(info.is_trivial);
    }
    #[test]
    pub(super) fn test_validate_ffi_decl_valid() {
        let decl = mk_ffi_decl(
            "add",
            vec![("a", FfiNativeType::I64), ("b", FfiNativeType::I64)],
            FfiNativeType::I64,
        );
        assert!(validate_ffi_decl(&decl).is_ok());
    }
    #[test]
    pub(super) fn test_validate_ffi_decl_large_struct() {
        let huge_fields: Vec<(String, FfiNativeType)> = (0..20)
            .map(|i| (format!("f{}", i), FfiNativeType::I64))
            .collect();
        let decl = mk_ffi_decl(
            "huge_fn",
            vec![("s", FfiNativeType::Struct("Huge".into(), huge_fields))],
            FfiNativeType::Void,
        );
        let result = validate_ffi_decl(&decl);
        assert!(result.is_err());
        if let Err(FfiError::StructTooLarge { .. }) = result {
        } else {
            panic!("expected StructTooLarge error");
        }
    }
    #[test]
    pub(super) fn test_ffi_type_map_default() {
        let map = FfiTypeMap::new();
        assert!(map.to_native("Nat").is_some());
        assert!(map.to_native("String").is_some());
        assert!(map.to_native("Bool").is_some());
    }
    #[test]
    pub(super) fn test_ffi_type_map_custom() {
        let mut map = FfiTypeMap::new();
        map.register(
            "MyType",
            FfiNativeType::Struct("MyType".into(), vec![]),
            LcnfType::Object,
        );
        assert!(map.to_native("MyType").is_some());
    }
    #[test]
    pub(super) fn test_mangle_lean_name() {
        assert_eq!(mangle_lean_name("Nat.add"), "l_Nat__add");
        assert_eq!(mangle_lean_name("main"), "l_main");
    }
    #[test]
    pub(super) fn test_generate_c_wrapper() {
        let decl = mk_ffi_decl(
            "my_func",
            vec![("n", FfiNativeType::U64)],
            FfiNativeType::U64,
        );
        let bridge = FfiBridge::new();
        let wrapper = bridge.generate_c_wrapper(&decl);
        assert!(wrapper.contains("_wrapper"));
        assert!(wrapper.contains("lean_unbox"));
    }
    #[test]
    pub(super) fn test_ffi_error_display() {
        let err = FfiError::UnsupportedType("MyType".into());
        assert!(err.to_string().contains("MyType"));
        let err = FfiError::ParamCountMismatch {
            expected: 2,
            found: 3,
        };
        assert!(err.to_string().contains("2"));
        assert!(err.to_string().contains("3"));
    }
    #[test]
    pub(super) fn test_extern_decls_to_ffi() {
        let extern_decls = vec![LcnfExternDecl {
            name: "io.println".to_string(),
            params: vec![LcnfParam {
                id: LcnfVarId(0),
                name: "s".to_string(),
                ty: LcnfType::LcnfString,
                erased: false,
                borrowed: false,
            }],
            ret_type: LcnfType::Unit,
        }];
        let ffi_decls = extern_decls_to_ffi(&extern_decls);
        assert_eq!(ffi_decls.len(), 1);
        assert_eq!(ffi_decls[0].name, "io.println");
        assert!(ffi_decls[0].is_unsafe);
    }
    #[test]
    pub(super) fn test_generate_module_ffi_empty() {
        let module = LcnfModule::default();
        let output = generate_module_ffi(&module);
        assert!(!output.c_header.is_empty());
    }
    #[test]
    pub(super) fn test_ffi_decl_display() {
        let decl = mk_ffi_decl(
            "add",
            vec![("a", FfiNativeType::I64), ("b", FfiNativeType::I64)],
            FfiNativeType::I64,
        );
        let s = decl.to_string();
        assert!(s.contains("add"));
        assert!(s.contains("int64_t"));
    }
    #[test]
    pub(super) fn test_marshal_info_display() {
        let info = FfiMarshalInfo::trivial(FfiNativeType::I64);
        let s = info.to_string();
        assert!(s.contains("int64_t"));
        let info2 =
            FfiMarshalInfo::with_conversion(FfiNativeType::U64, "lean_unbox(x)", "lean_box(r)");
        let s2 = info2.to_string();
        assert!(s2.contains("lean_unbox"));
    }
}
#[cfg(test)]
mod tests_ffi_ext_extra {
    use super::*;
    #[test]
    pub(super) fn test_ffi_ext_config() {
        let mut cfg = FfiExtConfig::new();
        cfg.set("mode", "release");
        cfg.set("verbose", "true");
        assert_eq!(cfg.get("mode"), Some("release"));
        assert!(cfg.get_bool("verbose"));
        assert!(cfg.get_int("mode").is_none());
        assert_eq!(cfg.len(), 2);
    }
    #[test]
    pub(super) fn test_ffi_ext_source_buffer() {
        let mut buf = FfiExtSourceBuffer::new();
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
    pub(super) fn test_ffi_ext_name_scope() {
        let mut scope = FfiExtNameScope::new();
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
    pub(super) fn test_ffi_ext_diag_collector() {
        let mut col = FfiExtDiagCollector::new();
        col.emit(FfiExtDiagMsg::warning("pass_a", "slow"));
        col.emit(FfiExtDiagMsg::error("pass_b", "fatal"));
        assert!(col.has_errors());
        assert_eq!(col.errors().len(), 1);
        assert_eq!(col.warnings().len(), 1);
        col.clear();
        assert!(col.is_empty());
    }
    #[test]
    pub(super) fn test_ffi_ext_id_gen() {
        let mut gen = FfiExtIdGen::new();
        assert_eq!(gen.next_id(), 0);
        assert_eq!(gen.next_id(), 1);
        gen.skip(10);
        assert_eq!(gen.next_id(), 12);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    pub(super) fn test_ffi_ext_incr_key() {
        let k1 = FfiExtIncrKey::new(100, 200);
        let k2 = FfiExtIncrKey::new(100, 200);
        let k3 = FfiExtIncrKey::new(999, 200);
        assert!(k1.matches(&k2));
        assert!(!k1.matches(&k3));
    }
    #[test]
    pub(super) fn test_ffi_ext_profiler() {
        let mut p = FfiExtProfiler::new();
        p.record(FfiExtPassTiming::new("pass_a", 1000, 50, 200, 100));
        p.record(FfiExtPassTiming::new("pass_b", 500, 30, 100, 200));
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
    pub(super) fn test_ffi_ext_event_log() {
        let mut log = FfiExtEventLog::new(3);
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
    pub(super) fn test_ffi_ext_version() {
        let v = FfiExtVersion::new(1, 2, 3).with_pre("alpha");
        assert!(!v.is_stable());
        assert_eq!(format!("{}", v), "1.2.3-alpha");
        let stable = FfiExtVersion::new(2, 0, 0);
        assert!(stable.is_stable());
        assert!(stable.is_compatible_with(&FfiExtVersion::new(2, 0, 0)));
        assert!(!stable.is_compatible_with(&FfiExtVersion::new(3, 0, 0)));
    }
    #[test]
    pub(super) fn test_ffi_ext_features() {
        let mut f = FfiExtFeatures::new();
        f.enable("sse2");
        f.enable("avx2");
        assert!(f.is_enabled("sse2"));
        assert!(!f.is_enabled("avx512"));
        f.disable("avx2");
        assert!(!f.is_enabled("avx2"));
        let mut g = FfiExtFeatures::new();
        g.enable("sse2");
        g.enable("neon");
        let union = f.union(&g);
        assert!(union.is_enabled("sse2") && union.is_enabled("neon"));
        let inter = f.intersection(&g);
        assert!(inter.is_enabled("sse2"));
    }
    #[test]
    pub(super) fn test_ffi_ext_emit_stats() {
        let mut s = FfiExtEmitStats::new();
        s.bytes_emitted = 50_000;
        s.items_emitted = 500;
        s.elapsed_ms = 100;
        assert!(s.is_clean());
        assert!((s.throughput_bps() - 500_000.0).abs() < 1.0);
        let disp = format!("{}", s);
        assert!(disp.contains("bytes=50000"));
    }
}
/// FFI pass version
#[allow(dead_code)]
pub const FFI_PASS_VERSION: &str = "2.0.0";
/// FFI version
#[allow(dead_code)]
pub const FFI_BRIDGE_VERSION: &str = "2.0.0";
/// FFI max params
#[allow(dead_code)]
pub const FFI_MAX_PARAMS: usize = 64;
/// FFI alignment check
#[allow(dead_code)]
pub fn ffi_check_alignment(size: u64, align: u64) -> bool {
    align.is_power_of_two() && size % align == 0
}
/// FFI type size (simplified)
#[allow(dead_code)]
pub fn ffi_type_size_bytes(type_name: &str) -> Option<u64> {
    match type_name {
        "int8_t" | "uint8_t" | "char" => Some(1),
        "int16_t" | "uint16_t" => Some(2),
        "int32_t" | "uint32_t" | "float" | "int" => Some(4),
        "int64_t" | "uint64_t" | "double" | "long long" => Some(8),
        "size_t" | "ptrdiff_t" | "void*" => Some(8),
        _ => None,
    }
}
/// FFI backend version
#[allow(dead_code)]
pub const FFI_BACKEND_PASS_VERSION: &str = "2.0.0";
#[cfg(test)]
mod FFI_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = FFIPassConfig::new("test_pass", FFIPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = FFIPassStats::new();
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
        let mut reg = FFIPassRegistry::new();
        reg.register(FFIPassConfig::new("pass_a", FFIPassPhase::Analysis));
        reg.register(FFIPassConfig::new("pass_b", FFIPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = FFIAnalysisCache::new(10);
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
        let mut wl = FFIWorklist::new();
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
        let mut dt = FFIDominatorTree::new(5);
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
        let mut liveness = FFILivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(FFIConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(FFIConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(FFIConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            FFIConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(FFIConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = FFIDepGraph::new();
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
/// FFI final version
#[allow(dead_code)]
pub const FFI_FINAL_VERSION: &str = "2.0.0";
/// FFI max error codes
#[allow(dead_code)]
pub const FFI_MAX_ERROR_CODES: usize = 256;
