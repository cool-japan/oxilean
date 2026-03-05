//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::{WASM_MAGIC, WASM_VERSION};
use std::collections::HashMap;

/// A WebAssembly value type (for function type signatures).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}
impl WasmType {
    pub fn name(&self) -> &str {
        match self {
            WasmType::I32 => "i32",
            WasmType::I64 => "i64",
            WasmType::F32 => "f32",
            WasmType::F64 => "f64",
            WasmType::V128 => "v128",
            WasmType::FuncRef => "funcref",
            WasmType::ExternRef => "externref",
        }
    }
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            WasmType::I32 | WasmType::I64 | WasmType::F32 | WasmType::F64
        )
    }
    pub fn is_reference(&self) -> bool {
        matches!(self, WasmType::FuncRef | WasmType::ExternRef)
    }
    pub fn default_value(&self) -> WasmValue {
        match self {
            WasmType::I32 => WasmValue::I32(0),
            WasmType::I64 => WasmValue::I64(0),
            WasmType::F32 => WasmValue::F32(0.0),
            WasmType::F64 => WasmValue::F64(0.0),
            WasmType::V128 => WasmValue::V128([0u8; 16]),
            WasmType::FuncRef => WasmValue::I32(0),
            WasmType::ExternRef => WasmValue::I32(0),
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "i32" => Some(WasmType::I32),
            "i64" => Some(WasmType::I64),
            "f32" => Some(WasmType::F32),
            "f64" => Some(WasmType::F64),
            "v128" => Some(WasmType::V128),
            "funcref" => Some(WasmType::FuncRef),
            "externref" => Some(WasmType::ExternRef),
            _ => None,
        }
    }
}
/// A WASM import descriptor.
#[derive(Debug, Clone)]
pub struct WasmImport {
    pub module: String,
    pub name: String,
    pub kind: WasmExternKind,
    pub index: u32,
}
impl WasmImport {
    pub fn new(module: &str, name: &str, kind: WasmExternKind, index: u32) -> Self {
        Self {
            module: module.to_string(),
            name: name.to_string(),
            kind,
            index,
        }
    }
}
pub struct WasiEnvironment {
    pub args: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub exit_code: Option<WasiExitCode>,
}
impl WasiEnvironment {
    pub fn new() -> Self {
        Self {
            args: vec![],
            env_vars: HashMap::new(),
            stdout: vec![],
            stderr: vec![],
            exit_code: None,
        }
    }
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }
    pub fn fd_write(&mut self, fd: u32, data: &[u8]) -> WasiFdWriteResult {
        match fd {
            1 => {
                self.stdout.extend_from_slice(data);
                WasiFdWriteResult {
                    bytes_written: data.len(),
                    errno: 0,
                }
            }
            2 => {
                self.stderr.extend_from_slice(data);
                WasiFdWriteResult {
                    bytes_written: data.len(),
                    errno: 0,
                }
            }
            _ => WasiFdWriteResult {
                bytes_written: 0,
                errno: 8,
            },
        }
    }
    pub fn proc_exit(&mut self, code: i32) {
        self.exit_code = Some(WasiExitCode(code));
    }
    pub fn stdout_str(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }
    pub fn stderr_str(&self) -> String {
        String::from_utf8_lossy(&self.stderr).into_owned()
    }
    pub fn args_count(&self) -> usize {
        self.args.len()
    }
    pub fn get_env(&self, key: &str) -> Option<&str> {
        self.env_vars.get(key).map(|s| s.as_str())
    }
}
/// Linear memory for a WASM module.
pub struct WasmMemory {
    pub data: Vec<u8>,
    pub page_count: usize,
}
impl WasmMemory {
    pub const PAGE_SIZE: usize = 65536;
    pub fn new(initial_pages: usize) -> Self {
        Self {
            data: vec![0u8; initial_pages * Self::PAGE_SIZE],
            page_count: initial_pages,
        }
    }
    /// Grow memory by `pages` pages.  Returns old page count or -1 on failure.
    pub fn grow(&mut self, pages: usize) -> i32 {
        let max_pages = (i32::MAX as usize) / Self::PAGE_SIZE;
        let new_count = self
            .page_count
            .checked_add(pages)
            .filter(|&c| c <= max_pages);
        match new_count {
            Some(c) => {
                let old = self.page_count as i32;
                self.data.resize(c * Self::PAGE_SIZE, 0);
                self.page_count = c;
                old
            }
            None => -1,
        }
    }
    pub fn load_u32(&self, offset: usize) -> Option<u32> {
        let end = offset.checked_add(4)?;
        if end > self.data.len() {
            return None;
        }
        Some(u32::from_le_bytes(
            self.data[offset..end]
                .try_into()
                .expect("slice is exactly 4 bytes as guaranteed by the bounds check above"),
        ))
    }
    pub fn store_u32(&mut self, offset: usize, value: u32) -> bool {
        let end = match offset.checked_add(4) {
            Some(e) if e <= self.data.len() => e,
            _ => return false,
        };
        self.data[offset..end].copy_from_slice(&value.to_le_bytes());
        true
    }
    pub fn load_bytes(&self, offset: usize, len: usize) -> Option<&[u8]> {
        let end = offset.checked_add(len)?;
        if end > self.data.len() {
            return None;
        }
        Some(&self.data[offset..end])
    }
    pub fn store_bytes(&mut self, offset: usize, data: &[u8]) -> bool {
        let end = match offset.checked_add(data.len()) {
            Some(e) if e <= self.data.len() => e,
            _ => return false,
        };
        self.data[offset..end].copy_from_slice(data);
        true
    }
}
pub struct WasmModuleLoader {
    pub sections: Vec<WasmSectionHeader>,
    pub errors: Vec<String>,
}
impl WasmModuleLoader {
    pub fn new() -> Self {
        Self {
            sections: vec![],
            errors: vec![],
        }
    }
    pub fn check_header(&mut self, data: &[u8]) -> bool {
        if data.len() < 8 {
            self.errors.push("binary too short".into());
            return false;
        }
        if data[0..4] != WASM_MAGIC {
            self.errors
                .push(format!("invalid magic: {:02x?}", &data[0..4]));
            return false;
        }
        if data[4..8] != WASM_VERSION {
            self.errors
                .push(format!("unsupported version: {:02x?}", &data[4..8]));
            return false;
        }
        true
    }
    pub fn parse_sections(&mut self, data: &[u8]) -> bool {
        if !self.check_header(data) {
            return false;
        }
        let mut pos = 8usize;
        while pos < data.len() {
            let id_byte = data[pos];
            pos += 1;
            let id = match WasmSectionId::from_byte(id_byte) {
                Some(id) => id,
                None => {
                    self.errors.push(format!("unknown section id: {id_byte}"));
                    break;
                }
            };
            if pos >= data.len() {
                self.errors.push("truncated section size".into());
                break;
            }
            let size = data[pos] as u32;
            pos += 1;
            self.sections.push(WasmSectionHeader {
                id,
                size,
                offset: pos,
            });
            pos += size as usize;
        }
        self.errors.is_empty()
    }
    pub fn build_module(&self, name: &str) -> WasmModule {
        WasmModule::new(name, 1)
    }
    pub fn has_section(&self, id: WasmSectionId) -> bool {
        self.sections.iter().any(|s| s.id == id)
    }
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
}
#[derive(Debug, Clone)]
pub struct GcArrayType {
    pub name: String,
    pub element_ty: WasmType,
    pub mutable: bool,
}
impl GcArrayType {
    pub fn new(name: &str, element_ty: WasmType, mutable: bool) -> Self {
        Self {
            name: name.to_string(),
            element_ty,
            mutable,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExecError {
    StackUnderflow,
    TypeMismatch { expected: String, got: String },
    DivisionByZero,
    Unreachable,
    OutOfBoundsMemory(usize),
    UndefinedLocal(u32),
    UndefinedGlobal(u32),
    CallStackOverflow,
    Custom(String),
}
pub struct StackMachine {
    pub stack: Vec<WasmValue>,
    pub locals: Vec<WasmValue>,
    pub globals: Vec<WasmValue>,
    pub memory: WasmMemory,
    pub max_stack_depth: usize,
    pub instruction_count: u64,
}
impl StackMachine {
    pub fn new(memory_pages: usize) -> Self {
        Self {
            stack: Vec::new(),
            locals: Vec::new(),
            globals: Vec::new(),
            memory: WasmMemory::new(memory_pages),
            max_stack_depth: 1024,
            instruction_count: 0,
        }
    }
    pub fn push(&mut self, v: WasmValue) -> Result<(), ExecError> {
        if self.stack.len() >= self.max_stack_depth {
            return Err(ExecError::CallStackOverflow);
        }
        self.stack.push(v);
        Ok(())
    }
    pub fn pop(&mut self) -> Result<WasmValue, ExecError> {
        self.stack.pop().ok_or(ExecError::StackUnderflow)
    }
    pub fn pop_i32(&mut self) -> Result<i32, ExecError> {
        match self.pop()? {
            WasmValue::I32(v) => Ok(v),
            other => Err(ExecError::TypeMismatch {
                expected: "i32".into(),
                got: other.type_name().into(),
            }),
        }
    }
    pub fn pop_i64(&mut self) -> Result<i64, ExecError> {
        match self.pop()? {
            WasmValue::I64(v) => Ok(v),
            other => Err(ExecError::TypeMismatch {
                expected: "i64".into(),
                got: other.type_name().into(),
            }),
        }
    }
    pub fn pop_f32(&mut self) -> Result<f32, ExecError> {
        match self.pop()? {
            WasmValue::F32(v) => Ok(v),
            other => Err(ExecError::TypeMismatch {
                expected: "f32".into(),
                got: other.type_name().into(),
            }),
        }
    }
    pub fn pop_f64(&mut self) -> Result<f64, ExecError> {
        match self.pop()? {
            WasmValue::F64(v) => Ok(v),
            other => Err(ExecError::TypeMismatch {
                expected: "f64".into(),
                got: other.type_name().into(),
            }),
        }
    }
    #[allow(clippy::too_many_lines)]
    pub fn execute_one(&mut self, instr: &WasmInstruction) -> Result<(), ExecError> {
        self.instruction_count += 1;
        match instr {
            WasmInstruction::Nop => {}
            WasmInstruction::Unreachable => return Err(ExecError::Unreachable),
            WasmInstruction::I32Const(v) => self.push(WasmValue::I32(*v))?,
            WasmInstruction::I64Const(v) => self.push(WasmValue::I64(*v))?,
            WasmInstruction::F32Const(v) => self.push(WasmValue::F32(*v))?,
            WasmInstruction::F64Const(v) => self.push(WasmValue::F64(*v))?,
            WasmInstruction::I32Add => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a.wrapping_add(b)))?;
            }
            WasmInstruction::I32Sub => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a.wrapping_sub(b)))?;
            }
            WasmInstruction::I32Mul => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a.wrapping_mul(b)))?;
            }
            WasmInstruction::I32DivS => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                if b == 0 {
                    return Err(ExecError::DivisionByZero);
                }
                self.push(WasmValue::I32(a.wrapping_div(b)))?;
            }
            WasmInstruction::I32DivU => {
                let b = self.pop_i32()? as u32;
                let a = self.pop_i32()? as u32;
                if b == 0 {
                    return Err(ExecError::DivisionByZero);
                }
                self.push(WasmValue::I32((a / b) as i32))?;
            }
            WasmInstruction::I32RemS => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                if b == 0 {
                    return Err(ExecError::DivisionByZero);
                }
                self.push(WasmValue::I32(a.wrapping_rem(b)))?;
            }
            WasmInstruction::I32RemU => {
                let b = self.pop_i32()? as u32;
                let a = self.pop_i32()? as u32;
                if b == 0 {
                    return Err(ExecError::DivisionByZero);
                }
                self.push(WasmValue::I32((a % b) as i32))?;
            }
            WasmInstruction::I32And => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a & b))?;
            }
            WasmInstruction::I32Or => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a | b))?;
            }
            WasmInstruction::I32Xor => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a ^ b))?;
            }
            WasmInstruction::I32Shl => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a.wrapping_shl(b as u32)))?;
            }
            WasmInstruction::I32ShrS => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a.wrapping_shr(b as u32)))?;
            }
            WasmInstruction::I32ShrU => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::I32(a.wrapping_shr(b as u32) as i32))?;
            }
            WasmInstruction::I32Rotl => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::I32(a.rotate_left(b as u32) as i32))?;
            }
            WasmInstruction::I32Rotr => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::I32(a.rotate_right(b as u32) as i32))?;
            }
            WasmInstruction::I32Eqz => {
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(if a == 0 { 1 } else { 0 }))?;
            }
            WasmInstruction::I32Clz => {
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a.leading_zeros() as i32))?;
            }
            WasmInstruction::I32Ctz => {
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a.trailing_zeros() as i32))?;
            }
            WasmInstruction::I32Popcnt => {
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(a.count_ones() as i32))?;
            }
            WasmInstruction::I32Eq => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(if a == b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32Ne => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(if a != b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32LtS => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(if a < b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32LtU => {
                let b = self.pop_i32()? as u32;
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::I32(if a < b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32GtS => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(if a > b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32GtU => {
                let b = self.pop_i32()? as u32;
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::I32(if a > b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32LeS => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(if a <= b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32LeU => {
                let b = self.pop_i32()? as u32;
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::I32(if a <= b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32GeS => {
                let b = self.pop_i32()?;
                let a = self.pop_i32()?;
                self.push(WasmValue::I32(if a >= b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32GeU => {
                let b = self.pop_i32()? as u32;
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::I32(if a >= b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64Add => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a.wrapping_add(b)))?;
            }
            WasmInstruction::I64Sub => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a.wrapping_sub(b)))?;
            }
            WasmInstruction::I64Mul => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a.wrapping_mul(b)))?;
            }
            WasmInstruction::I64DivS => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                if b == 0 {
                    return Err(ExecError::DivisionByZero);
                }
                self.push(WasmValue::I64(a.wrapping_div(b)))?;
            }
            WasmInstruction::I64DivU => {
                let b = self.pop_i64()? as u64;
                let a = self.pop_i64()? as u64;
                if b == 0 {
                    return Err(ExecError::DivisionByZero);
                }
                self.push(WasmValue::I64((a / b) as i64))?;
            }
            WasmInstruction::I64RemS => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                if b == 0 {
                    return Err(ExecError::DivisionByZero);
                }
                self.push(WasmValue::I64(a.wrapping_rem(b)))?;
            }
            WasmInstruction::I64RemU => {
                let b = self.pop_i64()? as u64;
                let a = self.pop_i64()? as u64;
                if b == 0 {
                    return Err(ExecError::DivisionByZero);
                }
                self.push(WasmValue::I64((a % b) as i64))?;
            }
            WasmInstruction::I64And => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a & b))?;
            }
            WasmInstruction::I64Or => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a | b))?;
            }
            WasmInstruction::I64Xor => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a ^ b))?;
            }
            WasmInstruction::I64Shl => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a.wrapping_shl(b as u32)))?;
            }
            WasmInstruction::I64ShrS => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a.wrapping_shr(b as u32)))?;
            }
            WasmInstruction::I64ShrU => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()? as u64;
                self.push(WasmValue::I64(a.wrapping_shr(b as u32) as i64))?;
            }
            WasmInstruction::I64Rotl => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()? as u64;
                self.push(WasmValue::I64(a.rotate_left(b as u32) as i64))?;
            }
            WasmInstruction::I64Rotr => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()? as u64;
                self.push(WasmValue::I64(a.rotate_right(b as u32) as i64))?;
            }
            WasmInstruction::I64Eqz => {
                let a = self.pop_i64()?;
                self.push(WasmValue::I32(if a == 0 { 1 } else { 0 }))?;
            }
            WasmInstruction::I64Clz => {
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a.leading_zeros() as i64))?;
            }
            WasmInstruction::I64Ctz => {
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a.trailing_zeros() as i64))?;
            }
            WasmInstruction::I64Popcnt => {
                let a = self.pop_i64()?;
                self.push(WasmValue::I64(a.count_ones() as i64))?;
            }
            WasmInstruction::I64Eq => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I32(if a == b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64Ne => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I32(if a != b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64LtS => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I32(if a < b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64LtU => {
                let b = self.pop_i64()? as u64;
                let a = self.pop_i64()? as u64;
                self.push(WasmValue::I32(if a < b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64GtS => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I32(if a > b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64GtU => {
                let b = self.pop_i64()? as u64;
                let a = self.pop_i64()? as u64;
                self.push(WasmValue::I32(if a > b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64LeS => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I32(if a <= b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64LeU => {
                let b = self.pop_i64()? as u64;
                let a = self.pop_i64()? as u64;
                self.push(WasmValue::I32(if a <= b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64GeS => {
                let b = self.pop_i64()?;
                let a = self.pop_i64()?;
                self.push(WasmValue::I32(if a >= b { 1 } else { 0 }))?;
            }
            WasmInstruction::I64GeU => {
                let b = self.pop_i64()? as u64;
                let a = self.pop_i64()? as u64;
                self.push(WasmValue::I32(if a >= b { 1 } else { 0 }))?;
            }
            WasmInstruction::F32Add => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a + b))?;
            }
            WasmInstruction::F32Sub => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a - b))?;
            }
            WasmInstruction::F32Mul => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a * b))?;
            }
            WasmInstruction::F32Div => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a / b))?;
            }
            WasmInstruction::F32Abs => {
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a.abs()))?;
            }
            WasmInstruction::F32Neg => {
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(-a))?;
            }
            WasmInstruction::F32Ceil => {
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a.ceil()))?;
            }
            WasmInstruction::F32Floor => {
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a.floor()))?;
            }
            WasmInstruction::F32Trunc => {
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a.trunc()))?;
            }
            WasmInstruction::F32Nearest => {
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a.round()))?;
            }
            WasmInstruction::F32Sqrt => {
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a.sqrt()))?;
            }
            WasmInstruction::F32Min => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a.min(b)))?;
            }
            WasmInstruction::F32Max => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a.max(b)))?;
            }
            WasmInstruction::F32Copysign => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::F32(a.copysign(b)))?;
            }
            WasmInstruction::F32Eq => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::I32(if a == b { 1 } else { 0 }))?;
            }
            WasmInstruction::F32Ne => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::I32(if a != b { 1 } else { 0 }))?;
            }
            WasmInstruction::F32Lt => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::I32(if a < b { 1 } else { 0 }))?;
            }
            WasmInstruction::F32Gt => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::I32(if a > b { 1 } else { 0 }))?;
            }
            WasmInstruction::F32Le => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::I32(if a <= b { 1 } else { 0 }))?;
            }
            WasmInstruction::F32Ge => {
                let b = self.pop_f32()?;
                let a = self.pop_f32()?;
                self.push(WasmValue::I32(if a >= b { 1 } else { 0 }))?;
            }
            WasmInstruction::F64Add => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a + b))?;
            }
            WasmInstruction::F64Sub => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a - b))?;
            }
            WasmInstruction::F64Mul => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a * b))?;
            }
            WasmInstruction::F64Div => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a / b))?;
            }
            WasmInstruction::F64Abs => {
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a.abs()))?;
            }
            WasmInstruction::F64Neg => {
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(-a))?;
            }
            WasmInstruction::F64Ceil => {
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a.ceil()))?;
            }
            WasmInstruction::F64Floor => {
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a.floor()))?;
            }
            WasmInstruction::F64Trunc => {
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a.trunc()))?;
            }
            WasmInstruction::F64Nearest => {
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a.round()))?;
            }
            WasmInstruction::F64Sqrt => {
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a.sqrt()))?;
            }
            WasmInstruction::F64Min => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a.min(b)))?;
            }
            WasmInstruction::F64Max => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a.max(b)))?;
            }
            WasmInstruction::F64Copysign => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::F64(a.copysign(b)))?;
            }
            WasmInstruction::F64Eq => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::I32(if a == b { 1 } else { 0 }))?;
            }
            WasmInstruction::F64Ne => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::I32(if a != b { 1 } else { 0 }))?;
            }
            WasmInstruction::F64Lt => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::I32(if a < b { 1 } else { 0 }))?;
            }
            WasmInstruction::F64Gt => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::I32(if a > b { 1 } else { 0 }))?;
            }
            WasmInstruction::F64Le => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::I32(if a <= b { 1 } else { 0 }))?;
            }
            WasmInstruction::F64Ge => {
                let b = self.pop_f64()?;
                let a = self.pop_f64()?;
                self.push(WasmValue::I32(if a >= b { 1 } else { 0 }))?;
            }
            WasmInstruction::I32WrapI64 => {
                let a = self.pop_i64()?;
                self.push(WasmValue::I32(a as i32))?;
            }
            WasmInstruction::I64ExtendI32S => {
                let a = self.pop_i32()?;
                self.push(WasmValue::I64(a as i64))?;
            }
            WasmInstruction::I64ExtendI32U => {
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::I64(a as i64))?;
            }
            WasmInstruction::I32TruncF32S => {
                let a = self.pop_f32()?;
                self.push(WasmValue::I32(a as i32))?;
            }
            WasmInstruction::I32TruncF32U => {
                let a = self.pop_f32()?;
                self.push(WasmValue::I32(a as u32 as i32))?;
            }
            WasmInstruction::I32TruncF64S => {
                let a = self.pop_f64()?;
                self.push(WasmValue::I32(a as i32))?;
            }
            WasmInstruction::I32TruncF64U => {
                let a = self.pop_f64()?;
                self.push(WasmValue::I32(a as u32 as i32))?;
            }
            WasmInstruction::I64TruncF32S => {
                let a = self.pop_f32()?;
                self.push(WasmValue::I64(a as i64))?;
            }
            WasmInstruction::I64TruncF32U => {
                let a = self.pop_f32()?;
                self.push(WasmValue::I64(a as u64 as i64))?;
            }
            WasmInstruction::I64TruncF64S => {
                let a = self.pop_f64()?;
                self.push(WasmValue::I64(a as i64))?;
            }
            WasmInstruction::I64TruncF64U => {
                let a = self.pop_f64()?;
                self.push(WasmValue::I64(a as u64 as i64))?;
            }
            WasmInstruction::F32ConvertI32S => {
                let a = self.pop_i32()?;
                self.push(WasmValue::F32(a as f32))?;
            }
            WasmInstruction::F32ConvertI32U => {
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::F32(a as f32))?;
            }
            WasmInstruction::F32ConvertI64S => {
                let a = self.pop_i64()?;
                self.push(WasmValue::F32(a as f32))?;
            }
            WasmInstruction::F32ConvertI64U => {
                let a = self.pop_i64()? as u64;
                self.push(WasmValue::F32(a as f32))?;
            }
            WasmInstruction::F32DemoteF64 => {
                let a = self.pop_f64()?;
                self.push(WasmValue::F32(a as f32))?;
            }
            WasmInstruction::F64ConvertI32S => {
                let a = self.pop_i32()?;
                self.push(WasmValue::F64(a as f64))?;
            }
            WasmInstruction::F64ConvertI32U => {
                let a = self.pop_i32()? as u32;
                self.push(WasmValue::F64(a as f64))?;
            }
            WasmInstruction::F64ConvertI64S => {
                let a = self.pop_i64()?;
                self.push(WasmValue::F64(a as f64))?;
            }
            WasmInstruction::F64ConvertI64U => {
                let a = self.pop_i64()? as u64;
                self.push(WasmValue::F64(a as f64))?;
            }
            WasmInstruction::F64PromoteF32 => {
                let a = self.pop_f32()?;
                self.push(WasmValue::F64(a as f64))?;
            }
            WasmInstruction::I32ReinterpretF32 => {
                let a = self.pop_f32()?;
                self.push(WasmValue::I32(a.to_bits() as i32))?;
            }
            WasmInstruction::I64ReinterpretF64 => {
                let a = self.pop_f64()?;
                self.push(WasmValue::I64(a.to_bits() as i64))?;
            }
            WasmInstruction::F32ReinterpretI32 => {
                let a = self.pop_i32()?;
                self.push(WasmValue::F32(f32::from_bits(a as u32)))?;
            }
            WasmInstruction::F64ReinterpretI64 => {
                let a = self.pop_i64()?;
                self.push(WasmValue::F64(f64::from_bits(a as u64)))?;
            }
            WasmInstruction::LocalGet(idx) => {
                let v = self
                    .locals
                    .get(*idx as usize)
                    .cloned()
                    .ok_or(ExecError::UndefinedLocal(*idx))?;
                self.push(v)?;
            }
            WasmInstruction::LocalSet(idx) => {
                let v = self.pop()?;
                let i = *idx as usize;
                if i >= self.locals.len() {
                    return Err(ExecError::UndefinedLocal(*idx));
                }
                self.locals[i] = v;
            }
            WasmInstruction::LocalTee(idx) => {
                let v = self.pop()?;
                let i = *idx as usize;
                if i >= self.locals.len() {
                    return Err(ExecError::UndefinedLocal(*idx));
                }
                self.locals[i] = v.clone();
                self.push(v)?;
            }
            WasmInstruction::GlobalGet(idx) => {
                let v = self
                    .globals
                    .get(*idx as usize)
                    .cloned()
                    .ok_or(ExecError::UndefinedGlobal(*idx))?;
                self.push(v)?;
            }
            WasmInstruction::GlobalSet(idx) => {
                let v = self.pop()?;
                let i = *idx as usize;
                if i >= self.globals.len() {
                    return Err(ExecError::UndefinedGlobal(*idx));
                }
                self.globals[i] = v;
            }
            WasmInstruction::Drop => {
                self.pop()?;
            }
            WasmInstruction::Select => {
                let cond = self.pop_i32()?;
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                self.push(if cond != 0 { v1 } else { v2 })?;
            }
            WasmInstruction::MemorySize => {
                self.push(WasmValue::I32(self.memory.page_count as i32))?;
            }
            WasmInstruction::MemoryGrow => {
                let pages = self.pop_i32()? as usize;
                let r = self.memory.grow(pages);
                self.push(WasmValue::I32(r))?;
            }
            WasmInstruction::I32Load { offset, .. } => {
                let addr = self.pop_i32()? as usize + *offset as usize;
                let v = self
                    .memory
                    .load_u32(addr)
                    .ok_or(ExecError::OutOfBoundsMemory(addr))?;
                self.push(WasmValue::I32(v as i32))?;
            }
            WasmInstruction::I32Store { offset, .. } => {
                let v = self.pop_i32()?;
                let addr = self.pop_i32()? as usize + *offset as usize;
                if !self.memory.store_u32(addr, v as u32) {
                    return Err(ExecError::OutOfBoundsMemory(addr));
                }
            }
            WasmInstruction::I64Load { offset, .. } => {
                let addr = self.pop_i32()? as usize + *offset as usize;
                let lo = self
                    .memory
                    .load_u32(addr)
                    .ok_or(ExecError::OutOfBoundsMemory(addr))? as u64;
                let hi = self
                    .memory
                    .load_u32(addr + 4)
                    .ok_or(ExecError::OutOfBoundsMemory(addr + 4))? as u64;
                self.push(WasmValue::I64((hi << 32 | lo) as i64))?;
            }
            WasmInstruction::I64Store { offset, .. } => {
                let v = self.pop_i64()? as u64;
                let addr = self.pop_i32()? as usize + *offset as usize;
                if !self.memory.store_u32(addr, (v & 0xFFFF_FFFF) as u32) {
                    return Err(ExecError::OutOfBoundsMemory(addr));
                }
                if !self.memory.store_u32(addr + 4, (v >> 32) as u32) {
                    return Err(ExecError::OutOfBoundsMemory(addr + 4));
                }
            }
            WasmInstruction::F32Load { offset, .. } => {
                let addr = self.pop_i32()? as usize + *offset as usize;
                let bits = self
                    .memory
                    .load_u32(addr)
                    .ok_or(ExecError::OutOfBoundsMemory(addr))?;
                self.push(WasmValue::F32(f32::from_bits(bits)))?;
            }
            WasmInstruction::F32Store { offset, .. } => {
                let v = self.pop_f32()?;
                let addr = self.pop_i32()? as usize + *offset as usize;
                if !self.memory.store_u32(addr, v.to_bits()) {
                    return Err(ExecError::OutOfBoundsMemory(addr));
                }
            }
            WasmInstruction::F64Load { offset, .. } => {
                let addr = self.pop_i32()? as usize + *offset as usize;
                let lo = self
                    .memory
                    .load_u32(addr)
                    .ok_or(ExecError::OutOfBoundsMemory(addr))? as u64;
                let hi = self
                    .memory
                    .load_u32(addr + 4)
                    .ok_or(ExecError::OutOfBoundsMemory(addr + 4))? as u64;
                self.push(WasmValue::F64(f64::from_bits(hi << 32 | lo)))?;
            }
            WasmInstruction::F64Store { offset, .. } => {
                let v = self.pop_f64()?;
                let addr = self.pop_i32()? as usize + *offset as usize;
                let bits = v.to_bits();
                if !self.memory.store_u32(addr, (bits & 0xFFFF_FFFF) as u32) {
                    return Err(ExecError::OutOfBoundsMemory(addr));
                }
                if !self.memory.store_u32(addr + 4, (bits >> 32) as u32) {
                    return Err(ExecError::OutOfBoundsMemory(addr + 4));
                }
            }
            WasmInstruction::Block { .. }
            | WasmInstruction::Loop { .. }
            | WasmInstruction::If { .. }
            | WasmInstruction::Else
            | WasmInstruction::End
            | WasmInstruction::Br(_)
            | WasmInstruction::BrIf(_)
            | WasmInstruction::BrTable { .. }
            | WasmInstruction::Return
            | WasmInstruction::Call(_)
            | WasmInstruction::CallIndirect { .. } => {}
        }
        Ok(())
    }
    pub fn execute_linear(&mut self, instrs: &[WasmInstruction]) -> Result<(), ExecError> {
        for instr in instrs {
            self.execute_one(instr)?;
        }
        Ok(())
    }
    pub fn peek(&self) -> Option<&WasmValue> {
        self.stack.last()
    }
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }
    pub fn reset_stack(&mut self) {
        self.stack.clear();
    }
    pub fn add_local(&mut self, value: WasmValue) -> u32 {
        let idx = self.locals.len() as u32;
        self.locals.push(value);
        idx
    }
    pub fn add_global(&mut self, value: WasmValue) -> u32 {
        let idx = self.globals.len() as u32;
        self.globals.push(value);
        idx
    }
    pub fn stats(&self) -> StackMachineStats {
        StackMachineStats {
            instruction_count: self.instruction_count,
            stack_depth: self.stack.len(),
            local_count: self.locals.len(),
            global_count: self.globals.len(),
            memory_pages: self.memory.page_count,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WasiFdWriteResult {
    pub bytes_written: usize,
    pub errno: i32,
}
/// A WASM export descriptor.
#[derive(Debug, Clone)]
pub struct WasmExport {
    pub name: String,
    pub kind: WasmExternKind,
    pub index: u32,
}
impl WasmExport {
    pub fn new(name: &str, kind: WasmExternKind, index: u32) -> Self {
        Self {
            name: name.to_string(),
            kind,
            index,
        }
    }
}
#[derive(Debug, Clone)]
pub struct WasmSectionHeader {
    pub id: WasmSectionId,
    pub size: u32,
    pub offset: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmExternKind {
    Function,
    Table,
    Memory,
    Global,
}
impl WasmExternKind {
    pub fn name(&self) -> &str {
        match self {
            WasmExternKind::Function => "func",
            WasmExternKind::Table => "table",
            WasmExternKind::Memory => "memory",
            WasmExternKind::Global => "global",
        }
    }
}
#[derive(Debug, Clone)]
pub struct WasmFunction {
    pub name: String,
    pub type_idx: u32,
    pub locals: Vec<WasmType>,
    pub body: Vec<WasmInstruction>,
}
impl WasmFunction {
    pub fn new(name: &str, type_idx: u32) -> Self {
        Self {
            name: name.to_string(),
            type_idx,
            locals: vec![],
            body: vec![],
        }
    }
    pub fn add_local(&mut self, ty: WasmType) -> u32 {
        let idx = self.locals.len() as u32;
        self.locals.push(ty);
        idx
    }
    pub fn add_instruction(&mut self, instr: WasmInstruction) {
        self.body.push(instr);
    }
    pub fn instruction_count(&self) -> usize {
        self.body.len()
    }
    pub fn local_count(&self) -> usize {
        self.locals.len()
    }
}
#[derive(Debug, Clone)]
pub struct GcStructField {
    pub name: String,
    pub ty: WasmType,
    pub mutable: bool,
}
impl GcStructField {
    pub fn new(name: &str, ty: WasmType, mutable: bool) -> Self {
        Self {
            name: name.to_string(),
            ty,
            mutable,
        }
    }
}
pub struct GcTypeRegistry {
    pub structs: Vec<GcStructType>,
    pub arrays: Vec<GcArrayType>,
}
impl GcTypeRegistry {
    pub fn new() -> Self {
        Self {
            structs: vec![],
            arrays: vec![],
        }
    }
    pub fn register_struct(&mut self, ty: GcStructType) -> u32 {
        let idx = self.structs.len() as u32;
        self.structs.push(ty);
        idx
    }
    pub fn register_array(&mut self, ty: GcArrayType) -> u32 {
        let idx = self.arrays.len() as u32;
        self.arrays.push(ty);
        idx
    }
    pub fn get_struct(&self, idx: u32) -> Option<&GcStructType> {
        self.structs.get(idx as usize)
    }
    pub fn get_array(&self, idx: u32) -> Option<&GcArrayType> {
        self.arrays.get(idx as usize)
    }
    pub fn struct_count(&self) -> usize {
        self.structs.len()
    }
    pub fn array_count(&self) -> usize {
        self.arrays.len()
    }
}
/// A WebAssembly function type (signature): parameter types and result types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmFuncType {
    pub params: Vec<WasmType>,
    pub results: Vec<WasmType>,
}
impl WasmFuncType {
    pub fn new(params: Vec<WasmType>, results: Vec<WasmType>) -> Self {
        Self { params, results }
    }
    pub fn unit() -> Self {
        Self::new(vec![], vec![])
    }
    pub fn arity(&self) -> usize {
        self.params.len()
    }
    pub fn result_count(&self) -> usize {
        self.results.len()
    }
    pub fn check_params(&self, values: &[WasmValue]) -> bool {
        if values.len() != self.params.len() {
            return false;
        }
        values
            .iter()
            .zip(self.params.iter())
            .all(|(v, t)| v.type_name() == t.name())
    }
    pub fn display(&self) -> String {
        let params: Vec<&str> = self.params.iter().map(|t| t.name()).collect();
        let results: Vec<&str> = self.results.iter().map(|t| t.name()).collect();
        format!(
            "(param {}) (result {})",
            params.join(" "),
            results.join(" ")
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamingState {
    AwaitingHeader,
    ReadingSections,
    Compiling,
    Done,
    Error(String),
}
/// A WebAssembly instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum WasmInstruction {
    Unreachable,
    Nop,
    Block { ty: Option<WasmType> },
    Loop { ty: Option<WasmType> },
    If { ty: Option<WasmType> },
    Else,
    End,
    Br(u32),
    BrIf(u32),
    BrTable { targets: Vec<u32>, default: u32 },
    Return,
    Call(u32),
    CallIndirect { type_idx: u32, table_idx: u32 },
    Drop,
    Select,
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),
    I32Load { align: u32, offset: u32 },
    I64Load { align: u32, offset: u32 },
    F32Load { align: u32, offset: u32 },
    F64Load { align: u32, offset: u32 },
    I32Store { align: u32, offset: u32 },
    I64Store { align: u32, offset: u32 },
    F32Store { align: u32, offset: u32 },
    F64Store { align: u32, offset: u32 },
    MemorySize,
    MemoryGrow,
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
}
impl WasmInstruction {
    pub fn mnemonic(&self) -> &str {
        match self {
            WasmInstruction::Unreachable => "unreachable",
            WasmInstruction::Nop => "nop",
            WasmInstruction::Block { .. } => "block",
            WasmInstruction::Loop { .. } => "loop",
            WasmInstruction::If { .. } => "if",
            WasmInstruction::Else => "else",
            WasmInstruction::End => "end",
            WasmInstruction::Br(_) => "br",
            WasmInstruction::BrIf(_) => "br_if",
            WasmInstruction::BrTable { .. } => "br_table",
            WasmInstruction::Return => "return",
            WasmInstruction::Call(_) => "call",
            WasmInstruction::CallIndirect { .. } => "call_indirect",
            WasmInstruction::Drop => "drop",
            WasmInstruction::Select => "select",
            WasmInstruction::LocalGet(_) => "local.get",
            WasmInstruction::LocalSet(_) => "local.set",
            WasmInstruction::LocalTee(_) => "local.tee",
            WasmInstruction::GlobalGet(_) => "global.get",
            WasmInstruction::GlobalSet(_) => "global.set",
            WasmInstruction::I32Load { .. } => "i32.load",
            WasmInstruction::I64Load { .. } => "i64.load",
            WasmInstruction::F32Load { .. } => "f32.load",
            WasmInstruction::F64Load { .. } => "f64.load",
            WasmInstruction::I32Store { .. } => "i32.store",
            WasmInstruction::I64Store { .. } => "i64.store",
            WasmInstruction::F32Store { .. } => "f32.store",
            WasmInstruction::F64Store { .. } => "f64.store",
            WasmInstruction::MemorySize => "memory.size",
            WasmInstruction::MemoryGrow => "memory.grow",
            WasmInstruction::I32Const(_) => "i32.const",
            WasmInstruction::I64Const(_) => "i64.const",
            WasmInstruction::F32Const(_) => "f32.const",
            WasmInstruction::F64Const(_) => "f64.const",
            WasmInstruction::I32Add => "i32.add",
            WasmInstruction::I32Sub => "i32.sub",
            WasmInstruction::I32Mul => "i32.mul",
            WasmInstruction::I32DivS => "i32.div_s",
            WasmInstruction::I32DivU => "i32.div_u",
            WasmInstruction::I32And => "i32.and",
            WasmInstruction::I32Or => "i32.or",
            WasmInstruction::I32Xor => "i32.xor",
            WasmInstruction::I32Eq => "i32.eq",
            WasmInstruction::I32Ne => "i32.ne",
            WasmInstruction::I32Eqz => "i32.eqz",
            WasmInstruction::I64Add => "i64.add",
            WasmInstruction::I64Sub => "i64.sub",
            WasmInstruction::I64Mul => "i64.mul",
            WasmInstruction::I64And => "i64.and",
            WasmInstruction::I64Or => "i64.or",
            WasmInstruction::I64Xor => "i64.xor",
            WasmInstruction::I64Eqz => "i64.eqz",
            WasmInstruction::F32Add => "f32.add",
            WasmInstruction::F32Sub => "f32.sub",
            WasmInstruction::F32Mul => "f32.mul",
            WasmInstruction::F32Div => "f32.div",
            WasmInstruction::F64Add => "f64.add",
            WasmInstruction::F64Sub => "f64.sub",
            WasmInstruction::F64Mul => "f64.mul",
            WasmInstruction::F64Div => "f64.div",
            WasmInstruction::I32WrapI64 => "i32.wrap_i64",
            WasmInstruction::I64ExtendI32S => "i64.extend_i32_s",
            WasmInstruction::I64ExtendI32U => "i64.extend_i32_u",
            WasmInstruction::F32ConvertI32S => "f32.convert_i32_s",
            WasmInstruction::F64PromoteF32 => "f64.promote_f32",
            WasmInstruction::F32DemoteF64 => "f32.demote_f64",
            _ => "unknown",
        }
    }
    pub fn is_control_flow(&self) -> bool {
        matches!(
            self,
            WasmInstruction::Block { .. }
                | WasmInstruction::Loop { .. }
                | WasmInstruction::If { .. }
                | WasmInstruction::Else
                | WasmInstruction::End
                | WasmInstruction::Br(_)
                | WasmInstruction::BrIf(_)
                | WasmInstruction::BrTable { .. }
                | WasmInstruction::Return
                | WasmInstruction::Unreachable
        )
    }
    pub fn is_binary_op(&self) -> bool {
        matches!(
            self,
            WasmInstruction::I32Add
                | WasmInstruction::I32Sub
                | WasmInstruction::I32Mul
                | WasmInstruction::I32DivS
                | WasmInstruction::I32DivU
                | WasmInstruction::I32RemS
                | WasmInstruction::I32RemU
                | WasmInstruction::I32And
                | WasmInstruction::I32Or
                | WasmInstruction::I32Xor
                | WasmInstruction::I32Shl
                | WasmInstruction::I32ShrS
                | WasmInstruction::I32ShrU
                | WasmInstruction::I32Rotl
                | WasmInstruction::I32Rotr
                | WasmInstruction::I64Add
                | WasmInstruction::I64Sub
                | WasmInstruction::I64Mul
                | WasmInstruction::I64DivS
                | WasmInstruction::I64DivU
                | WasmInstruction::I64RemS
                | WasmInstruction::I64RemU
                | WasmInstruction::I64And
                | WasmInstruction::I64Or
                | WasmInstruction::I64Xor
                | WasmInstruction::I64Shl
                | WasmInstruction::I64ShrS
                | WasmInstruction::I64ShrU
                | WasmInstruction::I64Rotl
                | WasmInstruction::I64Rotr
                | WasmInstruction::F32Add
                | WasmInstruction::F32Sub
                | WasmInstruction::F32Mul
                | WasmInstruction::F32Div
                | WasmInstruction::F32Min
                | WasmInstruction::F32Max
                | WasmInstruction::F32Copysign
                | WasmInstruction::F64Add
                | WasmInstruction::F64Sub
                | WasmInstruction::F64Mul
                | WasmInstruction::F64Div
                | WasmInstruction::F64Min
                | WasmInstruction::F64Max
                | WasmInstruction::F64Copysign
        )
    }
}
#[derive(Debug, Clone)]
pub struct GcStructType {
    pub name: String,
    pub fields: Vec<GcStructField>,
}
impl GcStructType {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: vec![],
        }
    }
    pub fn add_field(&mut self, field: GcStructField) -> u32 {
        let idx = self.fields.len() as u32;
        self.fields.push(field);
        idx
    }
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
    pub fn get_field(&self, idx: u32) -> Option<&GcStructField> {
        self.fields.get(idx as usize)
    }
    pub fn find_field(&self, name: &str) -> Option<u32> {
        self.fields
            .iter()
            .position(|f| f.name == name)
            .map(|i| i as u32)
    }
}
#[derive(Debug, Clone)]
pub struct StackMachineStats {
    pub instruction_count: u64,
    pub stack_depth: usize,
    pub local_count: usize,
    pub global_count: usize,
    pub memory_pages: usize,
}
/// A WebAssembly global variable.
#[derive(Debug, Clone)]
pub struct WasmGlobal {
    pub value: WasmValue,
    pub mutable: bool,
    pub name: String,
}
impl WasmGlobal {
    pub fn new(name: &str, value: WasmValue, mutable: bool) -> Self {
        Self {
            value,
            mutable,
            name: name.to_string(),
        }
    }
    pub fn set(&mut self, value: WasmValue) -> Result<(), String> {
        if !self.mutable {
            return Err(format!("global `{}` is immutable", self.name));
        }
        self.value = value;
        Ok(())
    }
    pub fn get(&self) -> &WasmValue {
        &self.value
    }
}
/// Function table for indirect calls.
pub struct WasmTable {
    pub elements: Vec<Option<String>>,
    pub max_size: Option<usize>,
}
impl WasmTable {
    pub fn new(initial: usize) -> Self {
        Self {
            elements: vec![None; initial],
            max_size: None,
        }
    }
    pub fn get(&self, idx: usize) -> Option<&str> {
        self.elements.get(idx)?.as_deref()
    }
    pub fn set(&mut self, idx: usize, func: &str) -> bool {
        if let Some(max) = self.max_size {
            if idx >= max {
                return false;
            }
        }
        if idx >= self.elements.len() {
            self.elements.resize(idx + 1, None);
        }
        self.elements[idx] = Some(func.to_string());
        true
    }
}
/// Runtime that hosts multiple WASM modules.
pub struct WasmRuntime {
    pub modules: HashMap<String, WasmModule>,
}
impl WasmRuntime {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
    pub fn load_module(&mut self, module: WasmModule) {
        self.modules.insert(module.name.clone(), module);
    }
    pub fn get_module(&self, name: &str) -> Option<&WasmModule> {
        self.modules.get(name)
    }
    pub fn call(
        &self,
        module_name: &str,
        func_name: &str,
        args: &[WasmValue],
    ) -> Result<Vec<WasmValue>, String> {
        let module = self
            .modules
            .get(module_name)
            .ok_or_else(|| format!("module `{module_name}` not loaded"))?;
        module.call_function(func_name, args)
    }
    pub fn list_modules(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.modules.keys().map(|s| s.as_str()).collect();
        names.sort_unstable();
        names
    }
}
pub struct StreamingCompiler {
    pub buffer: Vec<u8>,
    pub state: StreamingState,
    pub bytes_consumed: usize,
    pub module_name: String,
}
impl StreamingCompiler {
    pub fn new(module_name: &str) -> Self {
        Self {
            buffer: vec![],
            state: StreamingState::AwaitingHeader,
            bytes_consumed: 0,
            module_name: module_name.to_string(),
        }
    }
    pub fn feed(&mut self, chunk: &[u8]) {
        self.buffer.extend_from_slice(chunk);
        self.advance();
    }
    fn advance(&mut self) {
        match &self.state {
            StreamingState::AwaitingHeader => {
                if self.buffer.len() >= 8 {
                    if self.buffer[0..4] == WASM_MAGIC && self.buffer[4..8] == WASM_VERSION {
                        self.bytes_consumed = 8;
                        self.state = StreamingState::ReadingSections;
                    } else {
                        self.state = StreamingState::Error("invalid WASM header".to_string());
                    }
                }
            }
            StreamingState::ReadingSections => {
                if self.bytes_consumed >= self.buffer.len() {
                    self.state = StreamingState::Compiling;
                }
            }
            StreamingState::Compiling => {
                self.state = StreamingState::Done;
            }
            _ => {}
        }
    }
    pub fn is_done(&self) -> bool {
        self.state == StreamingState::Done
    }
    pub fn has_error(&self) -> bool {
        matches!(self.state, StreamingState::Error(_))
    }
    pub fn finish(&mut self) -> Result<WasmModule, String> {
        if let StreamingState::Error(ref msg) = self.state {
            return Err(msg.clone());
        }
        if self.state != StreamingState::Done && self.state != StreamingState::Compiling {
            self.state = StreamingState::Done;
        }
        Ok(WasmModule::new(&self.module_name, 1))
    }
}
/// A loaded WASM module.
pub struct WasmModule {
    pub name: String,
    pub memory: WasmMemory,
    pub table: WasmTable,
    pub exports: HashMap<String, String>,
}
impl WasmModule {
    pub fn new(name: &str, memory_pages: usize) -> Self {
        Self {
            name: name.to_string(),
            memory: WasmMemory::new(memory_pages),
            table: WasmTable::new(16),
            exports: HashMap::new(),
        }
    }
    pub fn add_export(&mut self, name: &str, target: &str) {
        self.exports.insert(name.to_string(), target.to_string());
    }
    pub fn get_export(&self, name: &str) -> Option<&str> {
        self.exports.get(name).map(|s| s.as_str())
    }
    /// Stub: always returns an empty result vector.
    pub fn call_function(&self, name: &str, _args: &[WasmValue]) -> Result<Vec<WasmValue>, String> {
        if !self.exports.contains_key(name) {
            return Err(format!(
                "function `{name}` not found in module `{}`",
                self.name
            ));
        }
        Ok(vec![])
    }
}
/// A WebAssembly value.
#[derive(Debug, Clone, PartialEq)]
pub enum WasmValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128([u8; 16]),
}
impl WasmValue {
    pub fn as_i32(&self) -> Option<i32> {
        if let WasmValue::I32(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        if let WasmValue::I64(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    pub fn type_name(&self) -> &str {
        match self {
            WasmValue::I32(_) => "i32",
            WasmValue::I64(_) => "i64",
            WasmValue::F32(_) => "f32",
            WasmValue::F64(_) => "f64",
            WasmValue::V128(_) => "v128",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmSectionId {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12,
}
impl WasmSectionId {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(WasmSectionId::Custom),
            1 => Some(WasmSectionId::Type),
            2 => Some(WasmSectionId::Import),
            3 => Some(WasmSectionId::Function),
            4 => Some(WasmSectionId::Table),
            5 => Some(WasmSectionId::Memory),
            6 => Some(WasmSectionId::Global),
            7 => Some(WasmSectionId::Export),
            8 => Some(WasmSectionId::Start),
            9 => Some(WasmSectionId::Element),
            10 => Some(WasmSectionId::Code),
            11 => Some(WasmSectionId::Data),
            12 => Some(WasmSectionId::DataCount),
            _ => None,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            WasmSectionId::Custom => "custom",
            WasmSectionId::Type => "type",
            WasmSectionId::Import => "import",
            WasmSectionId::Function => "function",
            WasmSectionId::Table => "table",
            WasmSectionId::Memory => "memory",
            WasmSectionId::Global => "global",
            WasmSectionId::Export => "export",
            WasmSectionId::Start => "start",
            WasmSectionId::Element => "element",
            WasmSectionId::Code => "code",
            WasmSectionId::Data => "data",
            WasmSectionId::DataCount => "data_count",
        }
    }
}
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}
impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            is_valid: true,
            errors: vec![],
        }
    }
    pub fn with_error(msg: impl Into<String>) -> Self {
        Self {
            is_valid: false,
            errors: vec![msg.into()],
        }
    }
    pub fn add_error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
        self.is_valid = false;
    }
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasiExitCode(pub i32);
impl WasiExitCode {
    pub fn success() -> Self {
        WasiExitCode(0)
    }
    pub fn failure(code: i32) -> Self {
        WasiExitCode(code)
    }
    pub fn is_success(&self) -> bool {
        self.0 == 0
    }
}
pub struct WasmValidator;
impl WasmValidator {
    pub fn new() -> Self {
        WasmValidator
    }
    pub fn validate_exports(
        &self,
        exports: &HashMap<String, String>,
        functions: &HashMap<String, WasmFunction>,
    ) -> ValidationResult {
        let mut result = ValidationResult::ok();
        for (export_name, target) in exports {
            if !functions.contains_key(target.as_str()) {
                result.add_error(format!(
                    "export `{export_name}` references undefined function `{target}`"
                ));
            }
        }
        result
    }
    pub fn validate_function_locals(&self, func: &WasmFunction) -> ValidationResult {
        let mut result = ValidationResult::ok();
        for (i, local) in func.locals.iter().enumerate() {
            if local.is_reference() {
                result.add_error(format!(
                    "function `{}` local {i}: reference types in locals not yet supported",
                    func.name
                ));
            }
        }
        result
    }
    pub fn validate_call_targets(
        &self,
        func: &WasmFunction,
        num_functions: usize,
    ) -> ValidationResult {
        let mut result = ValidationResult::ok();
        for instr in &func.body {
            if let WasmInstruction::Call(idx) = instr {
                if (*idx as usize) >= num_functions {
                    result.add_error(format!(
                        "function `{}`: call to out-of-bounds function index {idx}",
                        func.name
                    ));
                }
            }
        }
        result
    }
    pub fn validate_block_balance(&self, func: &WasmFunction) -> ValidationResult {
        let mut depth: i64 = 0;
        let mut result = ValidationResult::ok();
        for instr in &func.body {
            match instr {
                WasmInstruction::Block { .. }
                | WasmInstruction::Loop { .. }
                | WasmInstruction::If { .. } => depth += 1,
                WasmInstruction::End => {
                    if depth == 0 {
                        result.add_error(format!(
                            "function `{}`: unmatched `end` instruction",
                            func.name
                        ));
                    } else {
                        depth -= 1;
                    }
                }
                _ => {}
            }
        }
        if depth != 0 {
            result.add_error(format!(
                "function `{}`: {depth} unclosed block(s)",
                func.name
            ));
        }
        result
    }
}
