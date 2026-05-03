//! Implementation blocks (part 2)

use super::super::functions::*;
use super::defs::*;
use crate::lcnf::*;
use std::collections::{HashMap, HashSet, VecDeque};

impl WasmPassStats {
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
impl WasmExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: WasmExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: WasmExtPassPhase) -> Self {
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
impl WasmExtPassStats {
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
    pub fn merge(&mut self, o: &WasmExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
impl WasmAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        WasmAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&WasmCacheEntry> {
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
            WasmCacheEntry {
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
impl WasmExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: WasmExtPassConfig) {
        self.stats.push(WasmExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&WasmExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&WasmExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&WasmExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &WasmExtPassPhase) -> Vec<&WasmExtPassConfig> {
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
impl WasmPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        WasmPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: WasmPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), WasmPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&WasmPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&WasmPassStats> {
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
impl WasmX2Worklist {
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
impl WasmX2ConstFolder {
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
impl WasmBackend {
    /// Create a new WasmBackend instance.
    pub fn new() -> Self {
        Self::default()
    }
    /// Generate a fresh local variable name.
    pub(super) fn fresh_local(&mut self) -> String {
        let n = self.local_counter;
        self.local_counter += 1;
        format!("_t{}", n)
    }
    /// Map an LCNF type to a WebAssembly value type.
    pub(crate) fn lcnf_type_to_wasm(ty: &LcnfType) -> WasmType {
        match ty {
            LcnfType::Nat => WasmType::I64,
            LcnfType::Erased | LcnfType::Irrelevant | LcnfType::Unit => WasmType::I32,
            LcnfType::LcnfString => WasmType::I32,
            LcnfType::Fun(_, _) => WasmType::I32,
            LcnfType::Ctor(_, _) => WasmType::I32,
            LcnfType::Object => WasmType::I32,
            LcnfType::Var(_) => WasmType::I32,
        }
    }
    /// Compile a single LCNF expression to a list of WASM instructions.
    /// The value is left on the stack.
    pub(super) fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        locals: &mut Vec<(String, WasmType)>,
    ) -> Vec<WasmInstr> {
        match expr {
            LcnfExpr::Let {
                id,
                ty,
                value,
                body,
                ..
            } => {
                let local_name = format!("x{}", id.0);
                let val_type = Self::lcnf_type_to_wasm(ty);
                locals.push((local_name.clone(), val_type));
                let mut instrs = self.compile_let_value(value, locals);
                instrs.push(WasmInstr::LocalSet(local_name));
                instrs.extend(self.compile_expr(body, locals));
                instrs
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => self.compile_case(scrutinee, alts, default.as_deref(), locals),
            LcnfExpr::Return(arg) => self.compile_arg(arg),
            LcnfExpr::Unreachable => vec![WasmInstr::Unreachable],
            LcnfExpr::TailCall(func, args) => {
                let mut instrs = Vec::new();
                for arg in args {
                    instrs.extend(self.compile_arg(arg));
                }
                match func {
                    LcnfArg::Var(_) => {
                        instrs.extend(self.compile_arg(func));
                        instrs.push(WasmInstr::CallIndirect);
                    }
                    _ => {
                        instrs.push(WasmInstr::CallIndirect);
                    }
                }
                instrs
            }
        }
    }
    /// Compile an LCNF argument to WASM instructions.
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> Vec<WasmInstr> {
        match arg {
            LcnfArg::Var(id) => vec![WasmInstr::LocalGet(format!("x{}", id.0))],
            LcnfArg::Lit(LcnfLit::Nat(n)) => vec![WasmInstr::I64Const(*n as i64)],
            LcnfArg::Lit(LcnfLit::Str(_)) => vec![WasmInstr::I32Const(0)],
            LcnfArg::Erased => vec![WasmInstr::I32Const(0)],
            LcnfArg::Type(_) => vec![WasmInstr::I32Const(0)],
        }
    }
    /// Compile an LCNF let-value binding.
    pub(super) fn compile_let_value(
        &mut self,
        val: &LcnfLetValue,
        _locals: &mut Vec<(String, WasmType)>,
    ) -> Vec<WasmInstr> {
        match val {
            LcnfLetValue::Lit(lit) => match lit {
                LcnfLit::Nat(n) => vec![WasmInstr::I64Const(*n as i64)],
                LcnfLit::Str(_) => vec![WasmInstr::I32Const(0)],
            },
            LcnfLetValue::FVar(id) => vec![WasmInstr::LocalGet(format!("x{}", id.0))],
            LcnfLetValue::App(func, args) => {
                let mut instrs = Vec::new();
                for arg in args {
                    instrs.extend(self.compile_arg(arg));
                }
                match func {
                    LcnfArg::Var(_) => {
                        instrs.extend(self.compile_arg(func));
                        instrs.push(WasmInstr::CallIndirect);
                    }
                    _ => {
                        instrs.push(WasmInstr::CallIndirect);
                    }
                }
                instrs
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let mut instrs = Vec::new();
                for arg in args {
                    instrs.extend(self.compile_arg(arg));
                }
                instrs.push(WasmInstr::Call(mangle_wasm_name(name)));
                instrs
            }
            LcnfLetValue::Proj(_name, _idx, var) => {
                vec![WasmInstr::LocalGet(format!("x{}", var.0))]
            }
            LcnfLetValue::Erased => vec![WasmInstr::I32Const(0)],
            LcnfLetValue::Reset(var) => vec![WasmInstr::LocalGet(format!("x{}", var.0))],
            LcnfLetValue::Reuse(_slot, name, _tag, args) => {
                let mut instrs = Vec::new();
                for arg in args {
                    instrs.extend(self.compile_arg(arg));
                }
                instrs.push(WasmInstr::Call(mangle_wasm_name(name)));
                instrs
            }
        }
    }
    /// Compile a case expression to WASM instructions.
    pub(super) fn compile_case(
        &mut self,
        scrutinee: &LcnfVarId,
        alts: &[LcnfAlt],
        default: Option<&LcnfExpr>,
        locals: &mut Vec<(String, WasmType)>,
    ) -> Vec<WasmInstr> {
        let mut instrs = Vec::new();
        if alts.is_empty() {
            if let Some(def) = default {
                instrs.extend(self.compile_expr(def, locals));
            } else {
                instrs.push(WasmInstr::Unreachable);
            }
            return instrs;
        }
        instrs.push(WasmInstr::LocalGet(format!("x{}", scrutinee.0)));
        instrs.push(WasmInstr::Drop);
        let body_instrs = if let Some(first_alt) = alts.first() {
            self.compile_expr(&first_alt.body, locals)
        } else if let Some(def) = default {
            self.compile_expr(def, locals)
        } else {
            vec![WasmInstr::Unreachable]
        };
        instrs.extend(body_instrs);
        instrs
    }
    /// Compile a single LCNF function declaration to a `WasmFunc`.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> WasmFunc {
        let name = mangle_wasm_name(&decl.name);
        let params: Vec<(String, WasmType)> = decl
            .params
            .iter()
            .map(|p| {
                let pname = format!("x{}", p.id.0);
                let pty = Self::lcnf_type_to_wasm(&p.ty);
                (pname, pty)
            })
            .collect();
        let results = vec![Self::lcnf_type_to_wasm(&decl.ret_type)];
        let mut locals: Vec<(String, WasmType)> = Vec::new();
        let body = self.compile_expr(&decl.body, &mut locals);
        WasmFunc {
            name,
            params,
            results,
            locals,
            body,
        }
    }
    /// Compile a slice of LCNF function declarations and emit a WAT string.
    pub fn emit_module(decls: &[LcnfFunDecl]) -> Result<String, String> {
        let mut backend = WasmBackend::new();
        let mut module = WasmModule::new();
        module.memory = Some(1);
        module.imports.push(WasmImport {
            module: "env".to_string(),
            name: "memory".to_string(),
            kind: WasmImportKind::Memory { min_pages: 1 },
        });
        module.imports.push(WasmImport {
            module: "oxilean".to_string(),
            name: "alloc".to_string(),
            kind: WasmImportKind::Func {
                params: vec![WasmType::I32],
                results: vec![WasmType::I32],
            },
        });
        for decl in decls {
            let func = backend.compile_decl(decl);
            let func_name = func.name.clone();
            module.functions.push(func);
            module.exports.push(func_name);
        }
        Ok(module.to_wat())
    }
}
impl WasmX2Cache {
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
impl WasmExtPassPhase {
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
impl WasmExtConstFolder {
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
impl WasmExtLiveness {
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
impl WasmConstantFoldingHelper {
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
impl WasmPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            WasmPassPhase::Analysis => "analysis",
            WasmPassPhase::Transformation => "transformation",
            WasmPassPhase::Verification => "verification",
            WasmPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, WasmPassPhase::Transformation | WasmPassPhase::Cleanup)
    }
}
impl WasmX2Liveness {
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
