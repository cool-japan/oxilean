//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    ExecError, GcArrayType, GcStructField, GcStructType, GcTypeRegistry, StackMachine,
    StreamingCompiler, ValidationResult, WasiEnvironment, WasiExitCode, WasmExport, WasmExternKind,
    WasmFuncType, WasmFunction, WasmGlobal, WasmImport, WasmInstruction, WasmMemory, WasmModule,
    WasmModuleLoader, WasmRuntime, WasmSectionId, WasmTable, WasmType, WasmValidator, WasmValue,
};

pub const WASM_MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
pub const WASM_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wasm_value_type_name() {
        assert_eq!(WasmValue::I32(0).type_name(), "i32");
        assert_eq!(WasmValue::F64(0.0).type_name(), "f64");
        assert_eq!(WasmValue::V128([0u8; 16]).type_name(), "v128");
    }
    #[test]
    fn test_wasm_value_accessors() {
        assert_eq!(WasmValue::I32(42).as_i32(), Some(42));
        assert_eq!(WasmValue::I64(99).as_i64(), Some(99));
        assert_eq!(WasmValue::I32(0).as_i64(), None);
    }
    #[test]
    fn test_wasm_memory_grow() {
        let mut mem = WasmMemory::new(1);
        assert_eq!(mem.page_count, 1);
        let old = mem.grow(2);
        assert_eq!(old, 1);
        assert_eq!(mem.page_count, 3);
        assert_eq!(mem.data.len(), 3 * WasmMemory::PAGE_SIZE);
    }
    #[test]
    fn test_wasm_memory_load_store_u32() {
        let mut mem = WasmMemory::new(1);
        assert!(mem.store_u32(0, 0xDEAD_BEEF));
        assert_eq!(mem.load_u32(0), Some(0xDEAD_BEEF));
        assert!(mem.load_u32(WasmMemory::PAGE_SIZE - 2).is_none());
    }
    #[test]
    fn test_wasm_memory_load_store_bytes() {
        let mut mem = WasmMemory::new(1);
        let data = b"hello";
        assert!(mem.store_bytes(10, data));
        assert_eq!(mem.load_bytes(10, 5), Some(data.as_slice()));
        assert!(!mem.store_bytes(WasmMemory::PAGE_SIZE - 2, data));
    }
    #[test]
    fn test_wasm_table_set_get() {
        let mut table = WasmTable::new(4);
        assert!(table.set(2, "my_func"));
        assert_eq!(table.get(2), Some("my_func"));
        assert_eq!(table.get(0), None);
    }
    #[test]
    fn test_wasm_module_export_and_call() {
        let mut module = WasmModule::new("test_mod", 1);
        module.add_export("add", "add_impl");
        assert_eq!(module.get_export("add"), Some("add_impl"));
        let result = module.call_function("add", &[WasmValue::I32(1), WasmValue::I32(2)]);
        assert!(result.is_ok());
        let err = module.call_function("missing", &[]);
        assert!(err.is_err());
    }
    #[test]
    fn test_wasm_runtime_load_and_list() {
        let mut rt = WasmRuntime::new();
        let mut m1 = WasmModule::new("alpha", 1);
        m1.add_export("foo", "foo_impl");
        rt.load_module(m1);
        rt.load_module(WasmModule::new("beta", 1));
        let mut names = rt.list_modules();
        names.sort_unstable();
        assert_eq!(names, vec!["alpha", "beta"]);
        assert!(rt.get_module("alpha").is_some());
        assert!(rt.get_module("gamma").is_none());
        let res = rt.call("alpha", "foo", &[]);
        assert!(res.is_ok());
        let err = rt.call("gamma", "bar", &[]);
        assert!(err.is_err());
    }
    #[test]
    fn test_wasm_type_name() {
        assert_eq!(WasmType::I32.name(), "i32");
        assert_eq!(WasmType::F64.name(), "f64");
        assert_eq!(WasmType::FuncRef.name(), "funcref");
    }
    #[test]
    fn test_wasm_type_is_numeric() {
        assert!(WasmType::I32.is_numeric());
        assert!(!WasmType::FuncRef.is_numeric());
    }
    #[test]
    fn test_wasm_type_from_str() {
        assert_eq!(WasmType::from_str("i32"), Some(WasmType::I32));
        assert_eq!(WasmType::from_str("unknown"), None);
    }
    #[test]
    fn test_wasm_type_default_value() {
        assert_eq!(WasmType::I32.default_value(), WasmValue::I32(0));
        assert_eq!(WasmType::I64.default_value(), WasmValue::I64(0));
    }
    #[test]
    fn test_wasm_func_type_arity() {
        let ft = WasmFuncType::new(vec![WasmType::I32, WasmType::I64], vec![WasmType::F64]);
        assert_eq!(ft.arity(), 2);
        assert_eq!(ft.result_count(), 1);
    }
    #[test]
    fn test_wasm_func_type_check_params() {
        let ft = WasmFuncType::new(vec![WasmType::I32], vec![]);
        assert!(ft.check_params(&[WasmValue::I32(5)]));
        assert!(!ft.check_params(&[WasmValue::I64(5)]));
    }
    #[test]
    fn test_wasm_func_type_display() {
        let ft = WasmFuncType::unit();
        let s = ft.display();
        assert!(s.contains("param") && s.contains("result"));
    }
    #[test]
    fn test_wasm_global_mutable() {
        let mut g = WasmGlobal::new("counter", WasmValue::I32(0), true);
        g.set(WasmValue::I32(42))
            .expect("test operation should succeed");
        assert_eq!(g.get(), &WasmValue::I32(42));
    }
    #[test]
    fn test_wasm_global_immutable_error() {
        let mut g = WasmGlobal::new("pi", WasmValue::F64(3.14), false);
        assert!(g.set(WasmValue::F64(0.0)).is_err());
    }
    #[test]
    fn test_wasm_extern_kind_name() {
        assert_eq!(WasmExternKind::Function.name(), "func");
        assert_eq!(WasmExternKind::Memory.name(), "memory");
    }
    #[test]
    fn test_wasm_instruction_mnemonic() {
        assert_eq!(WasmInstruction::I32Add.mnemonic(), "i32.add");
        assert_eq!(WasmInstruction::F64Mul.mnemonic(), "f64.mul");
        assert_eq!(WasmInstruction::Return.mnemonic(), "return");
    }
    #[test]
    fn test_wasm_instruction_is_binary_op() {
        assert!(WasmInstruction::I32Add.is_binary_op());
        assert!(WasmInstruction::F32Div.is_binary_op());
        assert!(!WasmInstruction::I32Eqz.is_binary_op());
    }
    #[test]
    fn test_wasm_instruction_is_control_flow() {
        assert!(WasmInstruction::Return.is_control_flow());
        assert!(WasmInstruction::Br(0).is_control_flow());
        assert!(!WasmInstruction::I32Add.is_control_flow());
    }
    #[test]
    fn test_wasm_function_locals_and_body() {
        let mut func = WasmFunction::new("add", 0);
        let l0 = func.add_local(WasmType::I32);
        let l1 = func.add_local(WasmType::I32);
        func.add_instruction(WasmInstruction::LocalGet(l0));
        func.add_instruction(WasmInstruction::LocalGet(l1));
        func.add_instruction(WasmInstruction::I32Add);
        assert_eq!(func.local_count(), 2);
        assert_eq!(func.instruction_count(), 3);
    }
    #[test]
    fn test_validator_exports_ok() {
        let v = WasmValidator::new();
        let mut exports = HashMap::new();
        exports.insert("main".to_string(), "main_impl".to_string());
        let mut funcs = HashMap::new();
        funcs.insert("main_impl".to_string(), WasmFunction::new("main_impl", 0));
        assert!(v.validate_exports(&exports, &funcs).is_valid);
    }
    #[test]
    fn test_validator_exports_missing_function() {
        let v = WasmValidator::new();
        let mut exports = HashMap::new();
        exports.insert("foo".to_string(), "nonexistent".to_string());
        let result = v.validate_exports(&exports, &HashMap::new());
        assert!(!result.is_valid);
        assert_eq!(result.error_count(), 1);
    }
    #[test]
    fn test_validator_block_balance_ok() {
        let v = WasmValidator::new();
        let mut func = WasmFunction::new("f", 0);
        func.add_instruction(WasmInstruction::Block { ty: None });
        func.add_instruction(WasmInstruction::Nop);
        func.add_instruction(WasmInstruction::End);
        assert!(v.validate_block_balance(&func).is_valid);
    }
    #[test]
    fn test_validator_block_balance_unmatched_end() {
        let v = WasmValidator::new();
        let mut func = WasmFunction::new("f", 0);
        func.add_instruction(WasmInstruction::End);
        assert!(!v.validate_block_balance(&func).is_valid);
    }
    #[test]
    fn test_validator_block_balance_unclosed() {
        let v = WasmValidator::new();
        let mut func = WasmFunction::new("f", 0);
        func.add_instruction(WasmInstruction::Block { ty: None });
        assert!(!v.validate_block_balance(&func).is_valid);
    }
    #[test]
    fn test_stack_machine_i32_arith() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I32Const(10),
            WasmInstruction::I32Const(3),
            WasmInstruction::I32Add,
        ])
        .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(13)
        );
    }
    #[test]
    fn test_stack_machine_i32_div_zero() {
        let mut sm = StackMachine::new(1);
        let err = sm
            .execute_linear(&[
                WasmInstruction::I32Const(10),
                WasmInstruction::I32Const(0),
                WasmInstruction::I32DivS,
            ])
            .unwrap_err();
        assert_eq!(err, ExecError::DivisionByZero);
    }
    #[test]
    fn test_stack_machine_i64_arith() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I64Const(100),
            WasmInstruction::I64Const(50),
            WasmInstruction::I64Sub,
        ])
        .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I64(50)
        );
    }
    #[test]
    fn test_stack_machine_f32_arith() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::F32Const(6.0),
            WasmInstruction::F32Const(7.0),
            WasmInstruction::F32Mul,
        ])
        .expect("test operation should succeed");
        if let WasmValue::F32(v) = sm.pop().expect("collection should not be empty") {
            assert!((v - 42.0).abs() < 1e-5);
        } else {
            panic!("expected F32");
        }
    }
    #[test]
    fn test_stack_machine_f64_arith() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::F64Const(10.0),
            WasmInstruction::F64Const(4.0),
            WasmInstruction::F64Div,
        ])
        .expect("test operation should succeed");
        if let WasmValue::F64(v) = sm.pop().expect("collection should not be empty") {
            assert!((v - 2.5).abs() < 1e-10);
        } else {
            panic!("expected F64");
        }
    }
    #[test]
    fn test_stack_machine_local_get_set() {
        let mut sm = StackMachine::new(1);
        sm.add_local(WasmValue::I32(0));
        sm.execute_linear(&[
            WasmInstruction::I32Const(99),
            WasmInstruction::LocalSet(0),
            WasmInstruction::LocalGet(0),
        ])
        .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(99)
        );
    }
    #[test]
    fn test_stack_machine_local_tee() {
        let mut sm = StackMachine::new(1);
        sm.add_local(WasmValue::I32(0));
        sm.execute_linear(&[WasmInstruction::I32Const(55), WasmInstruction::LocalTee(0)])
            .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(55)
        );
        assert_eq!(sm.locals[0], WasmValue::I32(55));
    }
    #[test]
    fn test_stack_machine_global_get_set() {
        let mut sm = StackMachine::new(1);
        sm.add_global(WasmValue::I64(0));
        sm.execute_linear(&[
            WasmInstruction::I64Const(777),
            WasmInstruction::GlobalSet(0),
            WasmInstruction::GlobalGet(0),
        ])
        .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I64(777)
        );
    }
    #[test]
    fn test_stack_machine_select() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I32Const(10),
            WasmInstruction::I32Const(20),
            WasmInstruction::I32Const(1),
            WasmInstruction::Select,
        ])
        .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(10)
        );
    }
    #[test]
    fn test_stack_machine_drop() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I32Const(42),
            WasmInstruction::I32Const(99),
            WasmInstruction::Drop,
        ])
        .expect("test operation should succeed");
        assert_eq!(sm.stack_depth(), 1);
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(42)
        );
    }
    #[test]
    fn test_stack_machine_memory_size_grow() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[WasmInstruction::MemorySize])
            .expect("execution should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(1)
        );
        sm.execute_linear(&[WasmInstruction::I32Const(2), WasmInstruction::MemoryGrow])
            .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(1)
        );
    }
    #[test]
    fn test_stack_machine_i32_eqz() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[WasmInstruction::I32Const(0), WasmInstruction::I32Eqz])
            .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(1)
        );
    }
    #[test]
    fn test_stack_machine_i32_comparisons() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I32Const(5),
            WasmInstruction::I32Const(10),
            WasmInstruction::I32LtS,
        ])
        .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(1)
        );
    }
    #[test]
    fn test_stack_machine_conversion_i32_wrap_i64() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I64Const(0x1_0000_0001_i64),
            WasmInstruction::I32WrapI64,
        ])
        .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(1)
        );
    }
    #[test]
    fn test_stack_machine_f32_reinterpret() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I32Const(0x3f800000),
            WasmInstruction::F32ReinterpretI32,
        ])
        .expect("test operation should succeed");
        if let WasmValue::F32(v) = sm.pop().expect("collection should not be empty") {
            assert!((v - 1.0).abs() < 1e-6);
        } else {
            panic!("expected F32");
        }
    }
    #[test]
    fn test_stack_machine_stats() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I32Const(1),
            WasmInstruction::I32Const(2),
            WasmInstruction::I32Add,
        ])
        .expect("test operation should succeed");
        assert_eq!(sm.stats().instruction_count, 3);
    }
    #[test]
    fn test_stack_machine_underflow() {
        let mut sm = StackMachine::new(1);
        assert_eq!(
            sm.execute_linear(&[WasmInstruction::I32Add]).unwrap_err(),
            ExecError::StackUnderflow
        );
    }
    #[test]
    fn test_stack_machine_memory_store_load() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I32Const(0),
            WasmInstruction::I32Const(12345),
            WasmInstruction::I32Store {
                align: 2,
                offset: 0,
            },
            WasmInstruction::I32Const(0),
            WasmInstruction::I32Load {
                align: 2,
                offset: 0,
            },
        ])
        .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(12345)
        );
    }
    #[test]
    fn test_module_loader_valid_header() {
        let mut loader = WasmModuleLoader::new();
        let mut data = WASM_MAGIC.to_vec();
        data.extend_from_slice(&WASM_VERSION);
        assert!(loader.check_header(&data));
    }
    #[test]
    fn test_module_loader_invalid_magic() {
        let mut loader = WasmModuleLoader::new();
        let data = vec![0x00u8, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00];
        assert!(!loader.check_header(&data));
    }
    #[test]
    fn test_module_loader_too_short() {
        let mut loader = WasmModuleLoader::new();
        assert!(!loader.check_header(&[0x00, 0x61]));
    }
    #[test]
    fn test_module_loader_has_section() {
        let mut loader = WasmModuleLoader::new();
        let mut data = WASM_MAGIC.to_vec();
        data.extend_from_slice(&WASM_VERSION);
        data.push(1);
        data.push(0);
        loader.parse_sections(&data);
        assert!(loader.has_section(WasmSectionId::Type));
        assert_eq!(loader.section_count(), 1);
    }
    #[test]
    fn test_section_id_from_byte() {
        assert_eq!(WasmSectionId::from_byte(1), Some(WasmSectionId::Type));
        assert_eq!(WasmSectionId::from_byte(7), Some(WasmSectionId::Export));
        assert_eq!(WasmSectionId::from_byte(99), None);
    }
    #[test]
    fn test_section_id_name() {
        assert_eq!(WasmSectionId::Code.name(), "code");
        assert_eq!(WasmSectionId::Import.name(), "import");
    }
    #[test]
    fn test_wasi_fd_write_stdout() {
        let mut env = WasiEnvironment::new();
        let r = env.fd_write(1, b"hello");
        assert_eq!(r.bytes_written, 5);
        assert_eq!(env.stdout_str(), "hello");
    }
    #[test]
    fn test_wasi_fd_write_stderr() {
        let mut env = WasiEnvironment::new();
        env.fd_write(2, b"error");
        assert_eq!(env.stderr_str(), "error");
    }
    #[test]
    fn test_wasi_fd_write_bad_fd() {
        let mut env = WasiEnvironment::new();
        let r = env.fd_write(99, b"data");
        assert_ne!(r.errno, 0);
    }
    #[test]
    fn test_wasi_proc_exit() {
        let mut env = WasiEnvironment::new();
        env.proc_exit(42);
        assert!(!env
            .exit_code
            .as_ref()
            .expect("type conversion should succeed")
            .is_success());
    }
    #[test]
    fn test_wasi_with_args_and_env() {
        let env = WasiEnvironment::new()
            .with_args(vec!["prog".into()])
            .with_env("HOME", "/root");
        assert_eq!(env.args_count(), 1);
        assert_eq!(env.get_env("HOME"), Some("/root"));
    }
    #[test]
    fn test_gc_struct_type_fields() {
        let mut st = GcStructType::new("Point");
        st.add_field(GcStructField::new("x", WasmType::F64, true));
        st.add_field(GcStructField::new("y", WasmType::F64, true));
        assert_eq!(st.field_count(), 2);
        assert_eq!(st.find_field("x"), Some(0));
        assert_eq!(st.find_field("z"), None);
    }
    #[test]
    fn test_gc_type_registry() {
        let mut reg = GcTypeRegistry::new();
        let si = reg.register_struct(GcStructType::new("Foo"));
        let ai = reg.register_array(GcArrayType::new("Bar", WasmType::I32, true));
        assert_eq!(si, 0);
        assert_eq!(ai, 0);
        assert_eq!(reg.struct_count(), 1);
        assert_eq!(reg.array_count(), 1);
    }
    #[test]
    fn test_streaming_compiler_valid() {
        let mut compiler = StreamingCompiler::new("stream_mod");
        let mut data = WASM_MAGIC.to_vec();
        data.extend_from_slice(&WASM_VERSION);
        compiler.feed(&data);
        compiler.feed(&[]);
        compiler.feed(&[]);
        assert!(compiler.finish().is_ok());
    }
    #[test]
    fn test_streaming_compiler_invalid_header() {
        let mut compiler = StreamingCompiler::new("bad");
        compiler.feed(&[0xFF; 8]);
        assert!(compiler.has_error());
        assert!(compiler.finish().is_err());
    }
    #[test]
    fn test_streaming_compiler_incremental() {
        let mut compiler = StreamingCompiler::new("inc");
        let mut data = WASM_MAGIC.to_vec();
        data.extend_from_slice(&WASM_VERSION);
        compiler.feed(&data[..4]);
        assert!(!compiler.is_done());
        compiler.feed(&data[4..]);
        compiler.feed(&[]);
        compiler.feed(&[]);
        assert!(compiler.is_done());
    }
    #[test]
    fn test_exec_error_display() {
        assert_eq!(format!("{}", ExecError::DivisionByZero), "division by zero");
        let e2 = ExecError::TypeMismatch {
            expected: "i32".into(),
            got: "f64".into(),
        };
        assert!(format!("{e2}").contains("type mismatch"));
        assert!(format!("{}", ExecError::OutOfBoundsMemory(0x1000)).contains("4096"));
    }
    #[test]
    fn test_wasi_exit_code() {
        assert!(WasiExitCode::success().is_success());
        assert!(!WasiExitCode::failure(1).is_success());
    }
    #[test]
    fn test_wasm_import_new() {
        let imp = WasmImport::new("env", "print", WasmExternKind::Function, 0);
        assert_eq!(imp.module, "env");
        assert_eq!(imp.name, "print");
        assert_eq!(imp.kind, WasmExternKind::Function);
    }
    #[test]
    fn test_wasm_export_new() {
        let exp = WasmExport::new("main", WasmExternKind::Function, 0);
        assert_eq!(exp.name, "main");
    }
    #[test]
    fn test_validation_result_with_error() {
        let r = ValidationResult::with_error("bad");
        assert!(!r.is_valid);
        assert_eq!(r.error_count(), 1);
    }
    #[test]
    fn test_stack_machine_peek() {
        let mut sm = StackMachine::new(1);
        sm.push(WasmValue::I32(7))
            .expect("test operation should succeed");
        assert_eq!(sm.peek(), Some(&WasmValue::I32(7)));
        assert_eq!(sm.stack_depth(), 1);
    }
    #[test]
    fn test_stack_machine_reset_stack() {
        let mut sm = StackMachine::new(1);
        sm.push(WasmValue::I32(1))
            .expect("test operation should succeed");
        sm.push(WasmValue::I32(2))
            .expect("test operation should succeed");
        sm.reset_stack();
        assert_eq!(sm.stack_depth(), 0);
    }
    #[test]
    fn test_wasm_func_type_unit() {
        let ft = WasmFuncType::unit();
        assert_eq!(ft.arity(), 0);
        assert_eq!(ft.result_count(), 0);
    }
    #[test]
    fn test_wasm_validator_function_locals() {
        let v = WasmValidator::new();
        let mut func = WasmFunction::new("f", 0);
        func.add_local(WasmType::I32);
        assert!(v.validate_function_locals(&func).is_valid);
        func.add_local(WasmType::FuncRef);
        assert!(!v.validate_function_locals(&func).is_valid);
    }
    #[test]
    fn test_wasm_validator_call_targets() {
        let v = WasmValidator::new();
        let mut func = WasmFunction::new("f", 0);
        func.add_instruction(WasmInstruction::Call(5));
        assert!(!v.validate_call_targets(&func, 3).is_valid);
        assert!(v.validate_call_targets(&func, 10).is_valid);
    }
    #[test]
    fn test_stack_machine_i64_eqz() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[WasmInstruction::I64Const(0), WasmInstruction::I64Eqz])
            .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(1)
        );
    }
    #[test]
    fn test_stack_machine_f64_sqrt() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[WasmInstruction::F64Const(9.0), WasmInstruction::F64Sqrt])
            .expect("test operation should succeed");
        if let WasmValue::F64(v) = sm.pop().expect("collection should not be empty") {
            assert!((v - 3.0).abs() < 1e-10);
        } else {
            panic!();
        }
    }
    #[test]
    fn test_stack_machine_i32_bitwise() {
        let mut sm = StackMachine::new(1);
        sm.execute_linear(&[
            WasmInstruction::I32Const(0b1010),
            WasmInstruction::I32Const(0b1100),
            WasmInstruction::I32And,
        ])
        .expect("test operation should succeed");
        assert_eq!(
            sm.pop().expect("collection should not be empty"),
            WasmValue::I32(0b1000)
        );
    }
}
