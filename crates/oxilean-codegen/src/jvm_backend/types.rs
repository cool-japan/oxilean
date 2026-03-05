//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::functions::access_flags;
use super::functions::JvmResult;

use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl JVMLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        JVMLivenessInfo {
            live_in: vec![std::collections::HashSet::new(); block_count],
            live_out: vec![std::collections::HashSet::new(); block_count],
            defs: vec![std::collections::HashSet::new(); block_count],
            uses: vec![std::collections::HashSet::new(); block_count],
        }
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, block: usize, var: u32) {
        if block < self.defs.len() {
            self.defs[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, block: usize, var: u32) {
        if block < self.uses.len() {
            self.uses[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn is_live_in(&self, block: usize, var: u32) -> bool {
        self.live_in
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn is_live_out(&self, block: usize, var: u32) -> bool {
        self.live_out
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
}
/// Analysis cache for JVMExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct JVMExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl JVMExtCache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
/// Constant folding helper for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JVMExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl JVMExtConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
/// Code-generation errors for the JVM backend.
#[derive(Debug, Clone)]
pub enum JvmCodegenError {
    /// An unsupported LCNF construct was encountered.
    Unsupported(String),
    /// A name lookup failed.
    UnknownVar(String),
    /// Internal invariant violation.
    Internal(String),
}
/// The JVM backend that compiles LCNF to a `JvmClass` IR.
pub struct JvmBackend {
    pub(super) config: JvmConfig,
    /// Counter for generating unique label names.
    pub(super) label_counter: u32,
    /// Mapping from LCNF variable names to local-variable slot numbers.
    pub(super) locals: HashMap<String, u16>,
    /// Next available local-variable slot.
    pub(super) next_local: u16,
}
impl JvmBackend {
    /// Create a new backend with the given configuration.
    pub fn new(config: JvmConfig) -> Self {
        JvmBackend {
            config,
            label_counter: 0,
            locals: HashMap::new(),
            next_local: 0,
        }
    }
    /// Create a backend with the default configuration.
    pub fn default_backend() -> Self {
        JvmBackend::new(JvmConfig::default())
    }
    pub(super) fn fresh_label(&mut self, prefix: &str) -> String {
        let n = self.label_counter;
        self.label_counter += 1;
        format!("{}_{}", prefix, n)
    }
    /// Allocate a local-variable slot for `name` with the given type.
    pub(super) fn alloc_local(&mut self, name: &str, ty: &JvmType) -> u16 {
        let slot = self.next_local;
        self.locals.insert(name.to_string(), slot);
        self.next_local += ty.slot_size() as u16;
        slot
    }
    /// Look up the slot for `name`, allocating one (as Object) if absent.
    pub(super) fn get_or_alloc_local(&mut self, name: &str) -> u16 {
        if let Some(&slot) = self.locals.get(name) {
            return slot;
        }
        self.alloc_local(name, &JvmType::Object("java/lang/Object".to_string()))
    }
    /// Emit instructions that push a literal value onto the operand stack.
    pub fn emit_literal(&self, lit: &LcnfLit) -> JvmResult<Vec<JvmInstruction>> {
        let instrs = match lit {
            LcnfLit::Nat(n) => {
                let v = *n as i64;
                if v == 0 {
                    vec![JvmInstruction::new(JvmOpcode::Lconst(0))]
                } else if v == 1 {
                    vec![JvmInstruction::new(JvmOpcode::Lconst(1))]
                } else {
                    vec![JvmInstruction::new(JvmOpcode::Ldc(0))]
                }
            }
            LcnfLit::Str(_) => vec![JvmInstruction::new(JvmOpcode::Ldc(0))],
        };
        Ok(instrs)
    }
    /// Emit a simple binary arithmetic instruction for `int` operands.
    pub fn emit_binop(&self, op: &str) -> JvmResult<JvmInstruction> {
        let opcode = match op {
            "add" | "+" => JvmOpcode::Iadd,
            "sub" | "-" => JvmOpcode::Isub,
            "mul" | "*" => JvmOpcode::Imul,
            "div" | "/" => JvmOpcode::Idiv,
            "rem" | "%" => JvmOpcode::Irem,
            "ladd" => JvmOpcode::Ladd,
            "lsub" => JvmOpcode::Lsub,
            "lmul" => JvmOpcode::Lmul,
            "ldiv" => JvmOpcode::Ldiv,
            "dadd" => JvmOpcode::Dadd,
            "dsub" => JvmOpcode::Dsub,
            "dmul" => JvmOpcode::Dmul,
            "ddiv" => JvmOpcode::Ddiv,
            _ => return Err(JvmCodegenError::Unsupported(format!("binary op `{}`", op))),
        };
        Ok(JvmInstruction::new(opcode))
    }
    /// Build an `invokevirtual` instruction.
    pub fn emit_invokevirtual(&self, class: &str, name: &str, descriptor: &str) -> JvmInstruction {
        JvmInstruction::new(JvmOpcode::Invokevirtual {
            class: class.to_string(),
            name: name.to_string(),
            descriptor: descriptor.to_string(),
        })
    }
    /// Build an `invokestatic` instruction.
    pub fn emit_invokestatic(&self, class: &str, name: &str, descriptor: &str) -> JvmInstruction {
        JvmInstruction::new(JvmOpcode::Invokestatic {
            class: class.to_string(),
            name: name.to_string(),
            descriptor: descriptor.to_string(),
        })
    }
    /// Emit a `new` + `dup` + `invokespecial <init>` sequence (default ctor).
    pub fn emit_new_default(&self, class: &str) -> Vec<JvmInstruction> {
        vec![
            JvmInstruction::new(JvmOpcode::New(class.to_string())),
            JvmInstruction::new(JvmOpcode::Dup),
            JvmInstruction::new(JvmOpcode::Invokespecial {
                class: class.to_string(),
                name: "<init>".to_string(),
                descriptor: "()V".to_string(),
            }),
        ]
    }
    /// Emit a typed load instruction for the given local-variable slot.
    pub fn emit_load(&self, slot: u16, ty: &JvmType) -> JvmInstruction {
        let opcode = match ty {
            JvmType::Int | JvmType::Boolean | JvmType::Byte | JvmType::Short | JvmType::Char => {
                JvmOpcode::Iload(slot)
            }
            JvmType::Long => JvmOpcode::Lload(slot),
            JvmType::Float => JvmOpcode::Fload(slot),
            JvmType::Double => JvmOpcode::Dload(slot),
            _ => JvmOpcode::Aload(slot),
        };
        JvmInstruction::new(opcode)
    }
    /// Emit a typed store instruction for the given local-variable slot.
    pub fn emit_store(&self, slot: u16, ty: &JvmType) -> JvmInstruction {
        let opcode = match ty {
            JvmType::Int | JvmType::Boolean | JvmType::Byte | JvmType::Short | JvmType::Char => {
                JvmOpcode::Istore(slot)
            }
            JvmType::Long => JvmOpcode::Lstore(slot),
            JvmType::Float => JvmOpcode::Fstore(slot),
            JvmType::Double => JvmOpcode::Dstore(slot),
            _ => JvmOpcode::Astore(slot),
        };
        JvmInstruction::new(opcode)
    }
    /// Emit a typed return instruction.
    pub fn emit_return(&self, ty: &JvmType) -> JvmInstruction {
        let opcode = match ty {
            JvmType::Void => JvmOpcode::Return_,
            JvmType::Int | JvmType::Boolean | JvmType::Byte | JvmType::Short | JvmType::Char => {
                JvmOpcode::Ireturn
            }
            JvmType::Long => JvmOpcode::Lreturn,
            JvmType::Float => JvmOpcode::Freturn,
            JvmType::Double => JvmOpcode::Dreturn,
            _ => JvmOpcode::Areturn,
        };
        JvmInstruction::new(opcode)
    }
    /// Generate a canonical `<clinit>` that writes a fresh `INSTANCE` field.
    pub fn emit_clinit(&self, class_name: &str) -> JvmMethod {
        let mut code = Vec::new();
        code.extend(self.emit_new_default(class_name));
        code.push(JvmInstruction::new(JvmOpcode::Putstatic {
            class: class_name.to_string(),
            name: "INSTANCE".to_string(),
            descriptor: format!("L{};", class_name),
        }));
        code.push(JvmInstruction::new(JvmOpcode::Return_));
        JvmMethod::new("<clinit>", "()V", access_flags::STATIC, code, 3, 0)
    }
    /// Generate a default `<init>` constructor that delegates to `super()`.
    pub fn emit_default_init(&self, superclass: &str) -> JvmMethod {
        let code = vec![
            JvmInstruction::new(JvmOpcode::Aload(0)),
            JvmInstruction::new(JvmOpcode::Invokespecial {
                class: superclass.to_string(),
                name: "<init>".to_string(),
                descriptor: "()V".to_string(),
            }),
            JvmInstruction::new(JvmOpcode::Return_),
        ];
        JvmMethod::new("<init>", "()V", access_flags::PUBLIC, code, 1, 1)
    }
    /// Compile an LCNF function declaration into a `JvmClass`.
    pub fn emit_fun_decl(&mut self, decl: &LcnfFunDecl) -> JvmResult<JvmClass> {
        let class_name = self.mangle_name(&decl.name);
        let mut cls = JvmClass::new(&class_name);
        cls.major_version = self.config.class_version;
        self.locals.clear();
        self.next_local = 0;
        for param in &decl.params {
            self.alloc_local(
                &param.name,
                &JvmType::Object("java/lang/Object".to_string()),
            );
        }
        let mut code = Vec::new();
        self.emit_lcnf_expr(&decl.body, &mut code)?;
        code.push(JvmInstruction::new(JvmOpcode::Areturn));
        let max_locals = self.next_local.max(1);
        let method = JvmMethod::new(
            "apply",
            "()Ljava/lang/Object;",
            access_flags::PUBLIC | access_flags::STATIC,
            code,
            8,
            max_locals,
        );
        cls.add_method(method);
        cls.add_method(self.emit_default_init("java/lang/Object"));
        Ok(cls)
    }
    /// Recursively emit JVM instructions for an LCNF expression.
    pub(super) fn emit_lcnf_expr(
        &mut self,
        expr: &LcnfExpr,
        out: &mut Vec<JvmInstruction>,
    ) -> JvmResult<()> {
        match expr {
            LcnfExpr::Let {
                id: _,
                ty: _,
                name,
                value,
                body,
            } => {
                self.emit_let_value(value, out)?;
                let slot = self.alloc_local(name, &JvmType::Object("java/lang/Object".to_string()));
                out.push(JvmInstruction::new(JvmOpcode::Astore(slot)));
                self.emit_lcnf_expr(body, out)?;
            }
            LcnfExpr::Return(arg) => {
                self.emit_lcnf_arg(arg, out);
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                let scrut_slot = self.get_or_alloc_local(&format!("_x{}", scrutinee.0));
                out.push(JvmInstruction::new(JvmOpcode::Aload(scrut_slot)));
                let end_label = self.fresh_label("case_end");
                for alt in alts {
                    let skip_label = self.fresh_label("alt_skip");
                    out.push(JvmInstruction::new(JvmOpcode::Dup));
                    out.push(JvmInstruction::new(JvmOpcode::Instanceof(
                        alt.ctor_name.clone(),
                    )));
                    out.push(JvmInstruction::new(JvmOpcode::Ifeq(0)));
                    let branch_idx = out.len() - 1;
                    for (i, param) in alt.params.iter().enumerate() {
                        out.push(JvmInstruction::new(JvmOpcode::Dup));
                        out.push(JvmInstruction::new(JvmOpcode::Getfield {
                            class: alt.ctor_name.clone(),
                            name: format!("field{}", i),
                            descriptor: "Ljava/lang/Object;".to_string(),
                        }));
                        let slot = self.alloc_local(
                            &param.name,
                            &JvmType::Object("java/lang/Object".to_string()),
                        );
                        out.push(JvmInstruction::new(JvmOpcode::Astore(slot)));
                    }
                    out.push(JvmInstruction::new(JvmOpcode::Pop));
                    self.emit_lcnf_expr(&alt.body, out)?;
                    out.push(JvmInstruction::new(JvmOpcode::Goto(0)));
                    let goto_end_idx = out.len() - 1;
                    out.push(JvmInstruction::new(JvmOpcode::Label(skip_label)));
                    let skip_offset = (out.len() - branch_idx) as i16;
                    if let JvmOpcode::Ifeq(ref mut off) = out[branch_idx].opcode {
                        *off = skip_offset;
                    }
                    let end_offset = (out.len() - goto_end_idx) as i16;
                    if let JvmOpcode::Goto(ref mut off) = out[goto_end_idx].opcode {
                        *off = end_offset;
                    }
                }
                out.push(JvmInstruction::new(JvmOpcode::Pop));
                if let Some(def) = default {
                    self.emit_lcnf_expr(def, out)?;
                } else {
                    out.push(JvmInstruction::new(JvmOpcode::AconstNull));
                    out.push(JvmInstruction::new(JvmOpcode::Athrow));
                }
                out.push(JvmInstruction::new(JvmOpcode::Label(end_label)));
            }
            LcnfExpr::Unreachable => {
                out.push(JvmInstruction::new(JvmOpcode::AconstNull));
                out.push(JvmInstruction::new(JvmOpcode::Athrow));
            }
            LcnfExpr::TailCall(func, args) => {
                self.emit_lcnf_arg(func, out);
                for arg in args {
                    self.emit_lcnf_arg(arg, out);
                }
                out.push(self.emit_invokevirtual(
                    "oxilean/runtime/Closure",
                    "apply",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                ));
            }
        }
        Ok(())
    }
    /// Emit instructions for a `LcnfLetValue`.
    pub(super) fn emit_let_value(
        &mut self,
        val: &LcnfLetValue,
        out: &mut Vec<JvmInstruction>,
    ) -> JvmResult<()> {
        match val {
            LcnfLetValue::Lit(lit) => {
                let instrs = self.emit_literal(lit)?;
                out.extend(instrs);
            }
            LcnfLetValue::FVar(id) => {
                let slot = self.get_or_alloc_local(&format!("_x{}", id.0));
                out.push(JvmInstruction::new(JvmOpcode::Aload(slot)));
            }
            LcnfLetValue::App(func, args) => {
                self.emit_lcnf_arg(func, out);
                for arg in args {
                    self.emit_lcnf_arg(arg, out);
                }
                out.push(self.emit_invokevirtual(
                    "oxilean/runtime/Closure",
                    "apply",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                ));
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let cls = self.mangle_name(name);
                out.extend(self.emit_new_default(&cls.clone()));
                for (i, arg) in args.iter().enumerate() {
                    out.push(JvmInstruction::new(JvmOpcode::Dup));
                    self.emit_lcnf_arg(arg, out);
                    out.push(JvmInstruction::new(JvmOpcode::Putfield {
                        class: cls.clone(),
                        name: format!("field{}", i),
                        descriptor: "Ljava/lang/Object;".to_string(),
                    }));
                }
            }
            LcnfLetValue::Proj(_field_name, idx, var) => {
                let slot = self.get_or_alloc_local(&format!("_x{}", var.0));
                out.push(JvmInstruction::new(JvmOpcode::Aload(slot)));
                out.push(JvmInstruction::new(JvmOpcode::Getfield {
                    class: "oxilean/runtime/Record".to_string(),
                    name: format!("field{}", idx),
                    descriptor: "Ljava/lang/Object;".to_string(),
                }));
            }
            LcnfLetValue::Erased => {
                out.push(JvmInstruction::new(JvmOpcode::AconstNull));
            }
            LcnfLetValue::Reset(var) => {
                let slot = self.get_or_alloc_local(&format!("_x{}", var.0));
                out.push(JvmInstruction::new(JvmOpcode::Aload(slot)));
            }
            LcnfLetValue::Reuse(_slot_var, name, _tag, args) => {
                let cls = self.mangle_name(name);
                out.extend(self.emit_new_default(&cls.clone()));
                for (i, arg) in args.iter().enumerate() {
                    out.push(JvmInstruction::new(JvmOpcode::Dup));
                    self.emit_lcnf_arg(arg, out);
                    out.push(JvmInstruction::new(JvmOpcode::Putfield {
                        class: cls.clone(),
                        name: format!("field{}", i),
                        descriptor: "Ljava/lang/Object;".to_string(),
                    }));
                }
            }
        }
        Ok(())
    }
    /// Emit instructions that push an `LcnfArg` onto the operand stack.
    pub(super) fn emit_lcnf_arg(&mut self, arg: &LcnfArg, out: &mut Vec<JvmInstruction>) {
        match arg {
            LcnfArg::Var(id) => {
                let slot = self.get_or_alloc_local(&format!("_x{}", id.0));
                out.push(JvmInstruction::new(JvmOpcode::Aload(slot)));
            }
            LcnfArg::Lit(lit) => {
                if let Ok(instrs) = self.emit_literal(lit) {
                    out.extend(instrs);
                } else {
                    out.push(JvmInstruction::new(JvmOpcode::AconstNull));
                }
            }
            LcnfArg::Erased | LcnfArg::Type(_) => {
                out.push(JvmInstruction::new(JvmOpcode::AconstNull));
            }
        }
    }
    /// Convert an OxiLean qualified name to a JVM binary class name.
    pub(super) fn mangle_name(&self, name: &str) -> String {
        let pkg = self.config.package.replace('.', "/");
        let cls = name.replace('.', "_").replace("::", "_");
        format!("{}/{}", pkg, cls)
    }
}
/// A single JVM instruction together with optional debug metadata.
#[derive(Debug, Clone)]
pub struct JvmInstruction {
    /// The actual opcode (and its inline operands).
    pub opcode: JvmOpcode,
    /// Optional source-line number for debugging.
    pub line: Option<u32>,
}
impl JvmInstruction {
    /// Create an instruction without line-number information.
    pub fn new(opcode: JvmOpcode) -> Self {
        JvmInstruction { opcode, line: None }
    }
    /// Create an instruction with a source-line number.
    pub fn with_line(opcode: JvmOpcode, line: u32) -> Self {
        JvmInstruction {
            opcode,
            line: Some(line),
        }
    }
}
/// A representative subset of JVM bytecode opcodes.
///
/// Names follow the JVM specification closely, with `_` appended where the
/// mnemonic is a Rust keyword (e.g. `Return_`).
#[derive(Debug, Clone, PartialEq)]
pub enum JvmOpcode {
    /// Push `null` reference onto the operand stack.
    AconstNull,
    /// Push `int` constant −1 through 5 (`iconst_<i>`).
    Iconst(i32),
    /// Push `long` constant 0 or 1 (`lconst_<l>`).
    Lconst(i64),
    /// Push `float` constant 0.0, 1.0, or 2.0 (`fconst_<f>`).
    Fconst(f32),
    /// Push `double` constant 0.0 or 1.0 (`dconst_<d>`).
    Dconst(f64),
    /// Push `byte` immediate as `int` (`bipush`).
    Bipush(i8),
    /// Push `short` immediate as `int` (`sipush`).
    Sipush(i16),
    /// Load constant from constant pool (`ldc` / `ldc_w` / `ldc2_w`).
    Ldc(u16),
    /// Load `int` from local variable `n` (`iload`).
    Iload(u16),
    /// Load `long` from local variable `n` (`lload`).
    Lload(u16),
    /// Load `float` from local variable `n` (`fload`).
    Fload(u16),
    /// Load `double` from local variable `n` (`dload`).
    Dload(u16),
    /// Load reference from local variable `n` (`aload`).
    Aload(u16),
    /// Store `int` to local variable `n` (`istore`).
    Istore(u16),
    /// Store `long` to local variable `n` (`lstore`).
    Lstore(u16),
    /// Store `float` to local variable `n` (`fstore`).
    Fstore(u16),
    /// Store `double` to local variable `n` (`dstore`).
    Dstore(u16),
    /// Store reference to local variable `n` (`astore`).
    Astore(u16),
    /// Load `int` from array (`iaload`).
    Iaload,
    /// Load reference from array (`aaload`).
    Aaload,
    /// Store `int` into array (`iastore`).
    Iastore,
    /// Store reference into array (`aastore`).
    Aastore,
    /// Discard top value (`pop`).
    Pop,
    /// Discard top one or two values (`pop2`).
    Pop2,
    /// Duplicate the top value (`dup`).
    Dup,
    /// Swap the two top values (`swap`).
    Swap,
    /// Add two `int` values (`iadd`).
    Iadd,
    /// Subtract two `int` values (`isub`).
    Isub,
    /// Multiply two `int` values (`imul`).
    Imul,
    /// Divide two `int` values (`idiv`).
    Idiv,
    /// `int` remainder (`irem`).
    Irem,
    /// Negate `int` (`ineg`).
    Ineg,
    /// Add two `long` values (`ladd`).
    Ladd,
    /// Subtract two `long` values (`lsub`).
    Lsub,
    /// Multiply two `long` values (`lmul`).
    Lmul,
    /// Divide two `long` values (`ldiv`).
    Ldiv,
    /// Add two `double` values (`dadd`).
    Dadd,
    /// Subtract two `double` values (`dsub`).
    Dsub,
    /// Multiply two `double` values (`dmul`).
    Dmul,
    /// Divide two `double` values (`ddiv`).
    Ddiv,
    /// Convert `int` to `long` (`i2l`).
    I2l,
    /// Convert `int` to `double` (`i2d`).
    I2d,
    /// Convert `long` to `int` (`l2i`).
    L2i,
    /// Convert `double` to `int` (`d2i`).
    D2i,
    /// Compare two `int` values and branch if equal (`if_icmpeq`).
    IfIcmpeq(i16),
    /// Compare two `int` values and branch if not equal (`if_icmpne`).
    IfIcmpne(i16),
    /// Compare two `int` values and branch if less than (`if_icmplt`).
    IfIcmplt(i16),
    /// Compare two `int` values and branch if greater than or equal (`if_icmpge`).
    IfIcmpge(i16),
    /// Compare two `int` values and branch if greater than (`if_icmpgt`).
    IfIcmpgt(i16),
    /// Compare two `int` values and branch if less than or equal (`if_icmple`).
    IfIcmple(i16),
    /// Branch if reference is `null` (`ifnull`).
    Ifnull(i16),
    /// Branch if reference is not `null` (`ifnonnull`).
    Ifnonnull(i16),
    /// Branch if `int` is zero (`ifeq`).
    Ifeq(i16),
    /// Branch if `int` is non-zero (`ifne`).
    Ifne(i16),
    /// Compare two `long` values; push −1, 0, or 1 (`lcmp`).
    Lcmp,
    /// Unconditional branch (`goto`).
    Goto(i16),
    /// Return `void` from method (`return`).
    Return_,
    /// Return `int` from method (`ireturn`).
    Ireturn,
    /// Return `long` from method (`lreturn`).
    Lreturn,
    /// Return `float` from method (`freturn`).
    Freturn,
    /// Return `double` from method (`dreturn`).
    Dreturn,
    /// Return reference from method (`areturn`).
    Areturn,
    /// Throw an exception (`athrow`).
    Athrow,
    /// Get instance field value (`getfield`).
    Getfield {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Set instance field value (`putfield`).
    Putfield {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Get static field value (`getstatic`).
    Getstatic {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Set static field value (`putstatic`).
    Putstatic {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Invoke instance method (`invokevirtual`).
    Invokevirtual {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Invoke interface method (`invokeinterface`).
    Invokeinterface {
        class: String,
        name: String,
        descriptor: String,
        count: u8,
    },
    /// Invoke a special (constructor / private / super) method (`invokespecial`).
    Invokespecial {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Invoke a static method (`invokestatic`).
    Invokestatic {
        class: String,
        name: String,
        descriptor: String,
    },
    /// Create new object (`new`).
    New(String),
    /// Create new array of primitive type (`newarray`).
    Newarray(JvmType),
    /// Create new array of reference type (`anewarray`).
    Anewarray(String),
    /// Get array length (`arraylength`).
    Arraylength,
    /// Check whether object is instance of class (`instanceof`).
    Instanceof(String),
    /// Cast object to class, throwing if incompatible (`checkcast`).
    Checkcast(String),
    /// Label pseudo-instruction used for branch target resolution.
    Label(String),
    /// Increment local variable by constant (`iinc`).
    Iinc { index: u16, constant: i16 },
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum JVMPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl JVMPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            JVMPassPhase::Analysis => "analysis",
            JVMPassPhase::Transformation => "transformation",
            JVMPassPhase::Verification => "verification",
            JVMPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, JVMPassPhase::Transformation | JVMPassPhase::Cleanup)
    }
}
/// An entry in the class-file constant pool.
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantPoolEntry {
    /// CONSTANT_Utf8 — string data
    Utf8(String),
    /// CONSTANT_Integer — 32-bit integer constant
    Integer(i32),
    /// CONSTANT_Long — 64-bit integer constant
    Long(i64),
    /// CONSTANT_Float — 32-bit float constant
    Float(f32),
    /// CONSTANT_Double — 64-bit float constant
    Double(f64),
    /// CONSTANT_Class — reference to a class by Utf8 index
    Class { name_index: u16 },
    /// CONSTANT_String — reference to a Utf8 string value
    StringRef { string_index: u16 },
    /// CONSTANT_Fieldref
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// CONSTANT_Methodref
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// CONSTANT_InterfaceMethodref
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// CONSTANT_NameAndType
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMPassConfig {
    pub phase: JVMPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl JVMPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: JVMPassPhase) -> Self {
        JVMPassConfig {
            phase,
            enabled: true,
            max_iterations: 10,
            debug_output: false,
            pass_name: name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[allow(dead_code)]
    pub fn max_iter(mut self, n: u32) -> Self {
        self.max_iterations = n;
        self
    }
}
#[allow(dead_code)]
pub struct JVMConstantFoldingHelper;
impl JVMConstantFoldingHelper {
    #[allow(dead_code)]
    pub fn fold_add_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn fold_sub_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn fold_mul_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn fold_div_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_add_f64(a: f64, b: f64) -> f64 {
        a + b
    }
    #[allow(dead_code)]
    pub fn fold_mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }
    #[allow(dead_code)]
    pub fn fold_neg_i64(a: i64) -> Option<i64> {
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn fold_not_bool(a: bool) -> bool {
        !a
    }
    #[allow(dead_code)]
    pub fn fold_and_bool(a: bool, b: bool) -> bool {
        a && b
    }
    #[allow(dead_code)]
    pub fn fold_or_bool(a: bool, b: bool) -> bool {
        a || b
    }
    #[allow(dead_code)]
    pub fn fold_shl_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shl(b)
    }
    #[allow(dead_code)]
    pub fn fold_shr_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shr(b)
    }
    #[allow(dead_code)]
    pub fn fold_rem_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            Some(a % b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_bitand_i64(a: i64, b: i64) -> i64 {
        a & b
    }
    #[allow(dead_code)]
    pub fn fold_bitor_i64(a: i64, b: i64) -> i64 {
        a | b
    }
    #[allow(dead_code)]
    pub fn fold_bitxor_i64(a: i64, b: i64) -> i64 {
        a ^ b
    }
    #[allow(dead_code)]
    pub fn fold_bitnot_i64(a: i64) -> i64 {
        !a
    }
}
/// Liveness analysis for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JVMExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl JVMExtLiveness {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            live_in: vec![Vec::new(); n],
            live_out: vec![Vec::new(); n],
            defs: vec![Vec::new(); n],
            uses: vec![Vec::new(); n],
        }
    }
    #[allow(dead_code)]
    pub fn live_in(&self, b: usize, v: usize) -> bool {
        self.live_in.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn live_out(&self, b: usize, v: usize) -> bool {
        self.live_out
            .get(b)
            .map(|s| s.contains(&v))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, b: usize, v: usize) {
        if let Some(s) = self.defs.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, b: usize, v: usize) {
        if let Some(s) = self.uses.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn var_is_used_in_block(&self, b: usize, v: usize) -> bool {
        self.uses.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn var_is_def_in_block(&self, b: usize, v: usize) -> bool {
        self.defs.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
}
/// Worklist for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl JVMExtWorklist {
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            items: std::collections::VecDeque::new(),
            present: vec![false; capacity],
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_back(id);
        }
    }
    #[allow(dead_code)]
    pub fn push_front(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_front(id);
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<usize> {
        let id = self.items.pop_front()?;
        if id < self.present.len() {
            self.present[id] = false;
        }
        Some(id)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, id: usize) -> bool {
        id < self.present.len() && self.present[id]
    }
    #[allow(dead_code)]
    pub fn drain_all(&mut self) -> Vec<usize> {
        let v: Vec<usize> = self.items.drain(..).collect();
        for &id in &v {
            if id < self.present.len() {
                self.present[id] = false;
            }
        }
        v
    }
}
/// Complete representation of a JVM class file (IR level).
#[derive(Debug, Clone)]
pub struct JvmClass {
    /// Binary class name using `/` separators (e.g. `"com/example/Foo"`).
    pub name: String,
    /// Superclass binary name (`"java/lang/Object"` by default).
    pub superclass: String,
    /// Implemented interfaces (binary names).
    pub interfaces: Vec<String>,
    /// Instance and static fields.
    pub fields: Vec<JvmField>,
    /// Methods.
    pub methods: Vec<JvmMethod>,
    /// Class-level access flags.
    pub access_flags: u16,
    /// Constant pool.
    pub constant_pool: ConstantPool,
    /// Class-file major version (e.g. 65 = Java 21).
    pub major_version: u16,
    /// Source file attribute (optional).
    pub source_file: Option<String>,
}
impl JvmClass {
    pub fn new(name: &str) -> Self {
        JvmClass {
            name: name.to_string(),
            superclass: "java/lang/Object".to_string(),
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            access_flags: access_flags::PUBLIC | access_flags::SUPER,
            constant_pool: ConstantPool::new(),
            major_version: 65,
            source_file: None,
        }
    }
    /// Add a field definition.
    pub fn add_field(&mut self, field: JvmField) {
        self.fields.push(field);
    }
    /// Add a method definition.
    pub fn add_method(&mut self, method: JvmMethod) {
        self.methods.push(method);
    }
    /// Add an implemented interface.
    pub fn add_interface(&mut self, iface: &str) {
        self.interfaces.push(iface.to_string());
    }
    /// Set the superclass binary name.
    pub fn set_superclass(&mut self, super_name: &str) {
        self.superclass = super_name.to_string();
    }
    /// Render a human-readable summary (not real bytecode).
    pub fn summary(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("class {} extends {}", self.name, self.superclass));
        if !self.interfaces.is_empty() {
            out.push_str(&format!(" implements {}", self.interfaces.join(", ")));
        }
        out.push_str(" {\n");
        for f in &self.fields {
            out.push_str(&format!("  field {} : {}\n", f.name, f.descriptor));
        }
        for m in &self.methods {
            out.push_str(&format!(
                "  method {} {} ({} instructions)\n",
                m.name,
                m.descriptor,
                m.code.len()
            ));
        }
        out.push('}');
        out
    }
}
/// Configuration for JVMExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMExtPassConfig {
    pub name: String,
    pub phase: JVMExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl JVMExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: JVMExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: JVMExtPassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
/// Dominator tree for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl JVMExtDomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
/// Pass registry for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct JVMExtPassRegistry {
    pub(super) configs: Vec<JVMExtPassConfig>,
    pub(super) stats: Vec<JVMExtPassStats>,
}
impl JVMExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: JVMExtPassConfig) {
        self.stats.push(JVMExtPassStats::new());
        self.configs.push(c);
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> Option<&JVMExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&JVMExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&JVMExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &JVMExtPassPhase) -> Vec<&JVMExtPassConfig> {
        self.configs
            .iter()
            .filter(|c| c.enabled && &c.phase == ph)
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_nodes_visited(&self) -> usize {
        self.stats.iter().map(|s| s.nodes_visited).sum()
    }
    #[allow(dead_code)]
    pub fn any_changed(&self) -> bool {
        self.stats.iter().any(|s| s.changed)
    }
}
/// Pass execution phase for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JVMExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl JVMExtPassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
/// A single JVM method (Code attribute + metadata).
#[derive(Debug, Clone)]
pub struct JvmMethod {
    /// Simple method name (e.g. `"<init>"`, `"apply"`).
    pub name: String,
    /// Method descriptor string (e.g. `"(I)V"`).
    pub descriptor: String,
    /// Access flags bitmask.
    pub access_flags: u16,
    /// Bytecode instructions.
    pub code: Vec<JvmInstruction>,
    /// Maximum operand-stack depth.
    pub max_stack: u16,
    /// Maximum number of local variables (including `this`).
    pub max_locals: u16,
    /// Exception table entries.
    pub exceptions: Vec<ExceptionEntry>,
}
impl JvmMethod {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        descriptor: &str,
        access_flags: u16,
        code: Vec<JvmInstruction>,
        max_stack: u16,
        max_locals: u16,
    ) -> Self {
        JvmMethod {
            name: name.to_string(),
            descriptor: descriptor.to_string(),
            access_flags,
            code,
            max_stack,
            max_locals,
            exceptions: Vec::new(),
        }
    }
    /// Add an exception handler entry.
    pub fn add_exception(&mut self, entry: ExceptionEntry) {
        self.exceptions.push(entry);
    }
    /// Is this method abstract (no Code attribute)?
    pub fn is_abstract(&self) -> bool {
        self.access_flags & access_flags::ABSTRACT != 0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, JVMCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl JVMAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        JVMAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&JVMCacheEntry> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        if self.entries.len() >= self.max_size {
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.clone(),
            JVMCacheEntry {
                key,
                data,
                timestamp: 0,
                valid: true,
            },
        );
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.valid = false;
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Statistics for JVMExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JVMExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl JVMExtPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &JVMExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Dependency graph for JVMExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl JVMExtDepGraph {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            edge_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n && to < self.n {
            if !self.adj[from].contains(&to) {
                self.adj[from].push(to);
                self.rev[to].push(from);
                self.edge_count += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn succs(&self, n: usize) -> &[usize] {
        self.adj.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn preds(&self, n: usize) -> &[usize] {
        self.rev.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut deg: Vec<usize> = (0..self.n).map(|i| self.rev[i].len()).collect();
        let mut q: std::collections::VecDeque<usize> =
            (0..self.n).filter(|&i| deg[i] == 0).collect();
        let mut out = Vec::with_capacity(self.n);
        while let Some(u) = q.pop_front() {
            out.push(u);
            for &v in &self.adj[u] {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == self.n {
            Some(out)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topo_sort().is_none()
    }
    #[allow(dead_code)]
    pub fn reachable(&self, start: usize) -> Vec<usize> {
        let mut vis = vec![false; self.n];
        let mut stk = vec![start];
        let mut out = Vec::new();
        while let Some(u) = stk.pop() {
            if u < self.n && !vis[u] {
                vis[u] = true;
                out.push(u);
                for &v in &self.adj[u] {
                    if !vis[v] {
                        stk.push(v);
                    }
                }
            }
        }
        out
    }
    #[allow(dead_code)]
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        for i in 0..self.n {
            if !visited[i] {
                let mut stk = vec![(i, 0usize)];
                while let Some((u, idx)) = stk.last_mut() {
                    if !visited[*u] {
                        visited[*u] = true;
                    }
                    if *idx < self.adj[*u].len() {
                        let v = self.adj[*u][*idx];
                        *idx += 1;
                        if !visited[v] {
                            stk.push((v, 0));
                        }
                    } else {
                        order.push(*u);
                        stk.pop();
                    }
                }
            }
        }
        let mut comp = vec![usize::MAX; self.n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        for &start in order.iter().rev() {
            if comp[start] == usize::MAX {
                let cid = components.len();
                let mut component = Vec::new();
                let mut stk = vec![start];
                while let Some(u) = stk.pop() {
                    if comp[u] == usize::MAX {
                        comp[u] = cid;
                        component.push(u);
                        for &v in &self.rev[u] {
                            if comp[v] == usize::MAX {
                                stk.push(v);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }
        components
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.n
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}
/// One row of the JVM exception table.
#[derive(Debug, Clone)]
pub struct ExceptionEntry {
    /// Start of the try region (instruction index, not byte offset).
    pub start: u16,
    /// Exclusive end of the try region.
    pub end: u16,
    /// Handler instruction index.
    pub handler: u16,
    /// Catch type class name (`None` → finally / catch-all).
    pub catch_type: Option<String>,
}
/// Method descriptor helper.
#[derive(Debug, Clone)]
pub struct MethodDescriptor {
    /// Parameter types.
    pub params: Vec<JvmType>,
    /// Return type.
    pub return_type: JvmType,
}
impl MethodDescriptor {
    pub fn new(params: Vec<JvmType>, return_type: JvmType) -> Self {
        MethodDescriptor {
            params,
            return_type,
        }
    }
}
/// JVM field/method descriptor types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JvmType {
    /// `B` — signed 8-bit integer
    Byte,
    /// `S` — signed 16-bit integer
    Short,
    /// `I` — signed 32-bit integer
    Int,
    /// `J` — signed 64-bit integer
    Long,
    /// `F` — 32-bit IEEE 754 float
    Float,
    /// `D` — 64-bit IEEE 754 double
    Double,
    /// `Z` — boolean
    Boolean,
    /// `C` — UTF-16 code unit
    Char,
    /// `V` — no return value
    Void,
    /// `Lclass/name;` — object reference
    Object(String),
    /// `[T` — array of T
    Array(Box<JvmType>),
    /// Erased generic type variable (represented as `java/lang/Object`)
    Generic(String),
}
impl JvmType {
    /// Returns the JVM field descriptor string (e.g. `"I"`, `"[Ljava/lang/String;"`).
    pub fn descriptor(&self) -> String {
        match self {
            JvmType::Byte => "B".to_string(),
            JvmType::Short => "S".to_string(),
            JvmType::Int => "I".to_string(),
            JvmType::Long => "J".to_string(),
            JvmType::Float => "F".to_string(),
            JvmType::Double => "D".to_string(),
            JvmType::Boolean => "Z".to_string(),
            JvmType::Char => "C".to_string(),
            JvmType::Void => "V".to_string(),
            JvmType::Object(cls) => format!("L{};", cls),
            JvmType::Array(inner) => format!("[{}", inner.descriptor()),
            JvmType::Generic(_) => "Ljava/lang/Object;".to_string(),
        }
    }
    /// Returns the size in JVM local-variable / stack slots (1 or 2).
    pub fn slot_size(&self) -> usize {
        match self {
            JvmType::Long | JvmType::Double => 2,
            _ => 1,
        }
    }
    /// `true` for the two "wide" primitive types (Long, Double).
    pub fn is_wide(&self) -> bool {
        self.slot_size() == 2
    }
    /// `true` for reference types (Object, Array, Generic).
    pub fn is_reference(&self) -> bool {
        matches!(
            self,
            JvmType::Object(_) | JvmType::Array(_) | JvmType::Generic(_)
        )
    }
    /// `true` for integer-category types (Byte, Short, Int, Boolean, Char).
    pub fn is_int_category(&self) -> bool {
        matches!(
            self,
            JvmType::Byte | JvmType::Short | JvmType::Int | JvmType::Boolean | JvmType::Char
        )
    }
}
#[allow(dead_code)]
pub struct JVMPassRegistry {
    pub(super) configs: Vec<JVMPassConfig>,
    pub(super) stats: std::collections::HashMap<String, JVMPassStats>,
}
impl JVMPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JVMPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: JVMPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), JVMPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&JVMPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&JVMPassStats> {
        self.stats.get(name)
    }
    #[allow(dead_code)]
    pub fn total_passes(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn enabled_count(&self) -> usize {
        self.enabled_passes().len()
    }
    #[allow(dead_code)]
    pub fn update_stats(&mut self, name: &str, changes: u64, time_ms: u64, iter: u32) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, time_ms, iter);
        }
    }
}
/// Configuration for the JVM backend.
#[derive(Debug, Clone)]
pub struct JvmConfig {
    /// Package prefix (e.g. `"com.example"`).
    pub package: String,
    /// Java class-file major version (default 65 = Java 21).
    pub class_version: u16,
    /// Emit debug line-number tables.
    pub emit_line_numbers: bool,
    /// Whether to generate sealed-interface hierarchies for ADT types.
    pub sealed_adt: bool,
}
/// Minimal constant pool builder.
#[derive(Debug, Clone, Default)]
pub struct ConstantPool {
    pub(super) entries: Vec<ConstantPoolEntry>,
}
impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool {
            entries: Vec::new(),
        }
    }
    /// Add an entry and return its 1-based index.
    pub fn add(&mut self, entry: ConstantPoolEntry) -> u16 {
        self.entries.push(entry);
        self.entries.len() as u16
    }
    /// Find or add a CONSTANT_Utf8 entry, returning its index.
    pub fn utf8(&mut self, s: &str) -> u16 {
        for (i, e) in self.entries.iter().enumerate() {
            if let ConstantPoolEntry::Utf8(v) = e {
                if v == s {
                    return (i + 1) as u16;
                }
            }
        }
        self.add(ConstantPoolEntry::Utf8(s.to_string()))
    }
    /// Find or add a CONSTANT_Class entry.
    pub fn class(&mut self, name: &str) -> u16 {
        let name_index = self.utf8(name);
        for (i, e) in self.entries.iter().enumerate() {
            if let ConstantPoolEntry::Class { name_index: ni } = e {
                if *ni == name_index {
                    return (i + 1) as u16;
                }
            }
        }
        self.add(ConstantPoolEntry::Class { name_index })
    }
    /// All constant pool entries (1-indexed in the class file).
    pub fn entries(&self) -> &[ConstantPoolEntry] {
        &self.entries
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl JVMWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JVMWorklist {
            items: std::collections::VecDeque::new(),
            in_worklist: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: u32) -> bool {
        if self.in_worklist.insert(item) {
            self.items.push_back(item);
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        let item = self.items.pop_front()?;
        self.in_worklist.remove(&item);
        Some(item)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, item: u32) -> bool {
        self.in_worklist.contains(&item)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl JVMDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JVMDepGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_dep(&mut self, dep: u32, dependent: u32) {
        self.add_node(dep);
        self.add_node(dependent);
        self.edges.push((dep, dependent));
    }
    #[allow(dead_code)]
    pub fn dependents_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(d, _)| *d == node)
            .map(|(_, dep)| *dep)
            .collect()
    }
    #[allow(dead_code)]
    pub fn dependencies_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(_, dep)| *dep == node)
            .map(|(d, _)| *d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Vec<u32> {
        let mut in_degree: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for &n in &self.nodes {
            in_degree.insert(n, 0);
        }
        for (_, dep) in &self.edges {
            *in_degree.entry(*dep).or_insert(0) += 1;
        }
        let mut queue: std::collections::VecDeque<u32> = self
            .nodes
            .iter()
            .filter(|&&n| in_degree[&n] == 0)
            .copied()
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            for dep in self.dependents_of(node) {
                let cnt = in_degree.entry(dep).or_insert(0);
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    queue.push_back(dep);
                }
            }
        }
        result
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topological_sort().len() < self.nodes.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JVMDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl JVMDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        JVMDominatorTree {
            idom: vec![None; size],
            dom_children: vec![Vec::new(); size],
            dom_depth: vec![0; size],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, idom: u32) {
        self.idom[node] = Some(idom);
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let mut cur = b;
        loop {
            match self.idom[cur] {
                Some(parent) if parent as usize == a => return true,
                Some(parent) if parent as usize == cur => return false,
                Some(parent) => cur = parent as usize,
                None => return false,
            }
        }
    }
    #[allow(dead_code)]
    pub fn depth(&self, node: usize) -> u32 {
        self.dom_depth.get(node).copied().unwrap_or(0)
    }
}
/// A field (instance or static) inside a JVM class.
#[derive(Debug, Clone)]
pub struct JvmField {
    /// Field name.
    pub name: String,
    /// Field descriptor (e.g. `"I"`, `"Ljava/lang/String;"`).
    pub descriptor: String,
    /// Access flags bitmask.
    pub access_flags: u16,
    /// Optional constant-value attribute (for static final fields).
    pub constant_value: Option<ConstantPoolEntry>,
}
impl JvmField {
    pub fn new(name: &str, ty: &JvmType, access_flags: u16) -> Self {
        JvmField {
            name: name.to_string(),
            descriptor: ty.descriptor(),
            access_flags,
            constant_value: None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JVMPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl JVMPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_run(&mut self, changes: u64, time_ms: u64, iterations: u32) {
        self.total_runs += 1;
        self.successful_runs += 1;
        self.total_changes += changes;
        self.time_ms += time_ms;
        self.iterations_used = iterations;
    }
    #[allow(dead_code)]
    pub fn average_changes_per_run(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.total_changes as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.successful_runs as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Runs: {}/{}, Changes: {}, Time: {}ms",
            self.successful_runs, self.total_runs, self.total_changes, self.time_ms
        )
    }
}
