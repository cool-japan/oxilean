use super::super::functions::access_flags;
use super::super::functions::JvmResult;
use crate::lcnf::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use super::defs::*;

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
    pub(crate) fn fresh_label(&mut self, prefix: &str) -> String {
        let n = self.label_counter;
        self.label_counter += 1;
        format!("{}_{}", prefix, n)
    }
    /// Allocate a local-variable slot for `name` with the given type.
    pub(crate) fn alloc_local(&mut self, name: &str, ty: &JvmType) -> u16 {
        let slot = self.next_local;
        self.locals.insert(name.to_string(), slot);
        self.next_local += ty.slot_size() as u16;
        slot
    }
    /// Look up the slot for `name`, allocating one (as Object) if absent.
    pub(crate) fn get_or_alloc_local(&mut self, name: &str) -> u16 {
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
    pub(crate) fn emit_lcnf_expr(
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
    pub(crate) fn emit_let_value(
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
    pub(crate) fn emit_lcnf_arg(&mut self, arg: &LcnfArg, out: &mut Vec<JvmInstruction>) {
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
    pub(crate) fn mangle_name(&self, name: &str) -> String {
        let pkg = self.config.package.replace('.', "/");
        let cls = name.replace('.', "_").replace("::", "_");
        format!("{}/{}", pkg, cls)
    }
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
