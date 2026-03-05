//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use oxilean_kernel::{BinderInfo, Expr, Level, Literal, Name};
use std::collections::{HashMap, HashSet, VecDeque};

/// Statistics collected during the conversion process.
#[derive(Clone, Debug, Default)]
pub struct ConversionStats {
    /// Total number of kernel expressions visited.
    pub exprs_visited: usize,
    /// Number of let bindings generated (ANF intermediaries).
    pub let_bindings_generated: usize,
    /// Number of lambdas lifted to top level.
    pub lambdas_lifted: usize,
    /// Number of proof terms erased.
    pub proofs_erased: usize,
    /// Number of type arguments erased.
    pub types_erased: usize,
    /// Number of closures converted.
    pub closures_converted: usize,
    /// Maximum nesting depth reached during conversion.
    pub max_depth: usize,
    /// Number of tail calls detected.
    pub tail_calls_detected: usize,
    /// Number of fresh variables allocated.
    pub fresh_vars_allocated: usize,
    /// Number of free variable computations performed.
    pub free_var_computations: usize,
}
/// Internal state for the kernel Expr to LCNF conversion.
///
/// Maintains fresh variable counters, name mappings, lifted function
/// declarations, and accumulated metadata.
pub struct ToLcnfState {
    /// Next fresh variable counter.
    pub(super) next_var: u64,
    /// Map from kernel name strings to LCNF variable IDs.
    pub(super) name_map: HashMap<String, LcnfVarId>,
    /// Map from kernel name strings to LCNF types.
    pub(super) type_map: HashMap<String, LcnfType>,
    /// Accumulated lifted function declarations.
    pub(super) lifted_funs: Vec<LcnfFunDecl>,
    /// The conversion configuration.
    pub(super) config: ToLcnfConfig,
    /// Current nesting depth (for depth limit checks).
    pub(super) depth: usize,
    /// Accumulated module metadata.
    pub(super) metadata: LcnfModuleMetadata,
    /// Conversion statistics.
    pub(super) stats: ConversionStats,
    /// De Bruijn index to variable ID mapping.
    pub(super) bvar_stack: Vec<LcnfVarId>,
    /// De Bruijn index to name hint mapping.
    pub(super) bvar_names: Vec<String>,
    /// Set of known proof-sorted names for quick lookup.
    pub(super) proof_names: HashSet<String>,
    /// Set of known type-sorted names for quick lookup.
    pub(super) type_names: HashSet<String>,
    /// Counter for generating lifted function names.
    pub(super) lift_counter: u64,
    /// Pending let bindings accumulated during ANF conversion.
    pub(super) pending_lets: VecDeque<(LcnfVarId, String, LcnfType, LcnfLetValue)>,
    /// Maximum allowed conversion depth.
    pub(super) max_depth: usize,
}
impl ToLcnfState {
    /// Create a new conversion state with the given configuration.
    pub(super) fn new(config: &ToLcnfConfig) -> Self {
        ToLcnfState {
            next_var: 0,
            name_map: HashMap::new(),
            type_map: HashMap::new(),
            lifted_funs: Vec::new(),
            config: config.clone(),
            depth: 0,
            metadata: LcnfModuleMetadata::default(),
            stats: ConversionStats::default(),
            bvar_stack: Vec::new(),
            bvar_names: Vec::new(),
            proof_names: HashSet::new(),
            type_names: HashSet::new(),
            lift_counter: 0,
            pending_lets: VecDeque::new(),
            max_depth: 1024,
        }
    }
    /// Generate a fresh LCNF variable ID.
    pub(super) fn fresh_var(&mut self) -> LcnfVarId {
        let id = LcnfVarId(self.next_var);
        self.next_var += 1;
        self.stats.fresh_vars_allocated += 1;
        id
    }
    /// Generate a fresh variable with a name hint, registering it in the name map.
    pub(super) fn fresh_named_var(&mut self, hint: &str) -> LcnfVarId {
        let id = self.fresh_var();
        let name = if self.config.debug_names {
            format!("{}_{}", hint, id.0)
        } else {
            format!("_x{}", id.0)
        };
        self.name_map.insert(name, id);
        id
    }
    /// Generate a fresh name for a lifted function.
    pub(super) fn fresh_lift_name(&mut self, base: &str) -> String {
        let name = format!("{}_lifted_{}", base, self.lift_counter);
        self.lift_counter += 1;
        name
    }
    /// Push a de Bruijn variable binding.
    pub(super) fn push_bvar(&mut self, id: LcnfVarId, name: &str) {
        self.bvar_stack.push(id);
        self.bvar_names.push(name.to_string());
    }
    /// Pop a de Bruijn variable binding.
    pub(super) fn pop_bvar(&mut self) {
        self.bvar_stack.pop();
        self.bvar_names.pop();
    }
    /// Look up a de Bruijn index in the current scope.
    pub(super) fn lookup_bvar(&self, idx: u32) -> Option<LcnfVarId> {
        let stack_len = self.bvar_stack.len();
        if (idx as usize) < stack_len {
            Some(self.bvar_stack[stack_len - 1 - idx as usize])
        } else {
            None
        }
    }
    /// Look up a de Bruijn index name hint in the current scope.
    pub(super) fn lookup_bvar_name(&self, idx: u32) -> Option<&str> {
        let stack_len = self.bvar_names.len();
        if (idx as usize) < stack_len {
            Some(&self.bvar_names[stack_len - 1 - idx as usize])
        } else {
            None
        }
    }
    /// Look up a name in the name map to find its variable ID.
    pub(super) fn lookup_name(&self, name: &str) -> Option<LcnfVarId> {
        self.name_map.get(name).copied()
    }
    /// Register a name as proof-sorted (will be erased if erase_proofs is enabled).
    pub(super) fn mark_as_proof(&mut self, name: &str) {
        self.proof_names.insert(name.to_string());
    }
    /// Check if a name is known to be proof-sorted.
    pub(super) fn is_proof_name(&self, name: &str) -> bool {
        self.proof_names.contains(name)
    }
    /// Register a name as type-sorted (will be erased if erase_types is enabled).
    pub(super) fn mark_as_type(&mut self, name: &str) {
        self.type_names.insert(name.to_string());
    }
    /// Check if a name is known to be type-sorted.
    pub(super) fn is_type_name(&self, name: &str) -> bool {
        self.type_names.contains(name)
    }
    /// Enter a deeper nesting level, checking the depth limit.
    pub(super) fn enter_depth(&mut self) -> Result<(), ConversionError> {
        self.depth += 1;
        if self.depth > self.stats.max_depth {
            self.stats.max_depth = self.depth;
        }
        if self.depth > self.max_depth {
            return Err(ConversionError::DepthLimitExceeded(self.depth));
        }
        Ok(())
    }
    /// Leave the current nesting level.
    pub(super) fn leave_depth(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
    /// Wrap a terminal expression with any accumulated pending let bindings.
    pub(super) fn wrap_pending_lets(&mut self, terminal: LcnfExpr) -> LcnfExpr {
        let mut result = terminal;
        while let Some((id, name, ty, value)) = self.pending_lets.pop_back() {
            result = LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body: Box::new(result),
            };
        }
        result
    }
    /// Emit a let binding and return the variable ID referring to it.
    pub(super) fn emit_let(&mut self, hint: &str, ty: LcnfType, value: LcnfLetValue) -> LcnfVarId {
        let id = self.fresh_named_var(hint);
        let name = if self.config.debug_names {
            format!("{}_{}", hint, id.0)
        } else {
            format!("_x{}", id.0)
        };
        self.pending_lets.push_back((id, name, ty, value));
        self.stats.let_bindings_generated += 1;
        self.metadata.let_bindings += 1;
        id
    }
    /// Get the conversion statistics so far.
    pub(super) fn get_stats(&self) -> &ConversionStats {
        &self.stats
    }
    /// Finalize and return all accumulated lifted function declarations.
    pub(super) fn take_lifted_funs(&mut self) -> Vec<LcnfFunDecl> {
        std::mem::take(&mut self.lifted_funs)
    }
}
/// Context for the lambda lifting pass.
pub struct LambdaLifter {
    /// Accumulated lifted function declarations.
    pub(super) lifted: Vec<LcnfFunDecl>,
    /// Counter for unique lifted function names.
    pub(super) lift_counter: u64,
    /// Mapping from original variable IDs to their replacement after lifting.
    pub(super) var_remap: HashMap<LcnfVarId, LcnfVarId>,
    /// Maximum inline size (lambdas smaller than this stay inline).
    pub(super) max_inline_size: usize,
}
impl LambdaLifter {
    pub(super) fn new(max_inline_size: usize) -> Self {
        LambdaLifter {
            lifted: Vec::new(),
            lift_counter: 0,
            var_remap: HashMap::new(),
            max_inline_size,
        }
    }
    /// Generate a unique name for a lifted function.
    pub(super) fn fresh_name(&mut self, base: &str) -> String {
        let name = format!("{}_ll_{}", base, self.lift_counter);
        self.lift_counter += 1;
        name
    }
    /// Run the lambda lifting pass on a module's function declarations.
    pub(super) fn lift_module(&mut self, decls: &mut Vec<LcnfFunDecl>) {
        for decl in decls.iter_mut() {
            self.lift_body(&mut decl.body, &decl.name);
        }
        decls.append(&mut self.lifted);
    }
    /// Recursively process an LCNF expression, lifting lambdas.
    pub(super) fn lift_body(&mut self, expr: &mut LcnfExpr, parent_name: &str) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                self.lift_let_value(value, parent_name);
                self.lift_body(body, parent_name);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    self.lift_body(&mut alt.body, parent_name);
                }
                if let Some(def) = default.as_mut() {
                    self.lift_body(def, parent_name);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
    /// Process a let-value for potential lambda lifting.
    pub(super) fn lift_let_value(&mut self, value: &mut LcnfLetValue, _parent_name: &str) {
        match value {
            LcnfLetValue::App(func, args) => {
                self.remap_arg(func);
                for arg in args.iter_mut() {
                    self.remap_arg(arg);
                }
            }
            LcnfLetValue::Ctor(_, _, args) => {
                for arg in args.iter_mut() {
                    self.remap_arg(arg);
                }
            }
            LcnfLetValue::Proj(_, _, var) => {
                if let Some(remapped) = self.var_remap.get(var) {
                    *var = *remapped;
                }
            }
            LcnfLetValue::FVar(var) => {
                if let Some(remapped) = self.var_remap.get(var) {
                    *var = *remapped;
                }
            }
            LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::Reset(_)
            | LcnfLetValue::Reuse(_, _, _, _) => {}
        }
    }
    /// Remap a variable reference if it was previously lifted.
    pub(super) fn remap_arg(&self, arg: &mut LcnfArg) {
        if let LcnfArg::Var(id) = arg {
            if let Some(remapped) = self.var_remap.get(id) {
                *id = *remapped;
            }
        }
    }
    /// Compute the set of free LCNF variable IDs in an expression.
    pub(super) fn free_vars_of_expr(&self, expr: &LcnfExpr) -> HashSet<LcnfVarId> {
        let mut free = HashSet::new();
        let mut bound = HashSet::new();
        self.collect_free_lcnf(expr, &mut free, &mut bound);
        free
    }
    /// Recursively collect free variables in an LCNF expression.
    pub(super) fn collect_free_lcnf(
        &self,
        expr: &LcnfExpr,
        free: &mut HashSet<LcnfVarId>,
        bound: &mut HashSet<LcnfVarId>,
    ) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                self.collect_free_let_value(value, free, bound);
                bound.insert(*id);
                self.collect_free_lcnf(body, free, bound);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                if !bound.contains(scrutinee) {
                    free.insert(*scrutinee);
                }
                for alt in alts {
                    let mut alt_bound = bound.clone();
                    for p in &alt.params {
                        alt_bound.insert(p.id);
                    }
                    self.collect_free_lcnf(&alt.body, free, &mut alt_bound);
                }
                if let Some(def) = default {
                    self.collect_free_lcnf(def, free, bound);
                }
            }
            LcnfExpr::Return(arg) => {
                self.collect_free_arg(arg, free, bound);
            }
            LcnfExpr::TailCall(func, args) => {
                self.collect_free_arg(func, free, bound);
                for a in args {
                    self.collect_free_arg(a, free, bound);
                }
            }
            LcnfExpr::Unreachable => {}
        }
    }
    /// Collect free variables in a let-value.
    pub(super) fn collect_free_let_value(
        &self,
        value: &LcnfLetValue,
        free: &mut HashSet<LcnfVarId>,
        bound: &HashSet<LcnfVarId>,
    ) {
        match value {
            LcnfLetValue::App(func, args) => {
                self.collect_free_arg(func, free, bound);
                for a in args {
                    self.collect_free_arg(a, free, bound);
                }
            }
            LcnfLetValue::Proj(_, _, var) => {
                if !bound.contains(var) {
                    free.insert(*var);
                }
            }
            LcnfLetValue::Ctor(_, _, args) => {
                for a in args {
                    self.collect_free_arg(a, free, bound);
                }
            }
            LcnfLetValue::FVar(var) => {
                if !bound.contains(var) {
                    free.insert(*var);
                }
            }
            LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::Reset(_)
            | LcnfLetValue::Reuse(_, _, _, _) => {}
        }
    }
    /// Collect free variables in an argument.
    pub(super) fn collect_free_arg(
        &self,
        arg: &LcnfArg,
        free: &mut HashSet<LcnfVarId>,
        bound: &HashSet<LcnfVarId>,
    ) {
        if let LcnfArg::Var(id) = arg {
            if !bound.contains(id) {
                free.insert(*id);
            }
        }
    }
}
/// Configuration for the kernel Expr to LCNF conversion.
///
/// Controls which passes are enabled and how aggressively they are applied.
#[derive(Clone, Debug)]
pub struct ToLcnfConfig {
    /// Whether to erase proof terms (Prop-sorted expressions).
    pub erase_proofs: bool,
    /// Whether to erase type arguments.
    pub erase_types: bool,
    /// Whether to perform lambda lifting.
    pub lambda_lift: bool,
    /// Maximum size (in AST nodes) for a lambda to be left inline.
    pub max_inline_size: usize,
    /// Whether to generate debug-friendly names.
    pub debug_names: bool,
}
impl ToLcnfConfig {
    /// Create a config with all passes enabled.
    pub fn full() -> Self {
        ToLcnfConfig {
            erase_proofs: true,
            erase_types: true,
            lambda_lift: true,
            max_inline_size: 8,
            debug_names: false,
        }
    }
    /// Create a config with no passes enabled (raw conversion only).
    pub fn minimal() -> Self {
        ToLcnfConfig {
            erase_proofs: false,
            erase_types: false,
            lambda_lift: false,
            max_inline_size: 0,
            debug_names: true,
        }
    }
    /// Create a config for debugging (names preserved, minimal erasure).
    pub fn debug() -> Self {
        ToLcnfConfig {
            erase_proofs: false,
            erase_types: false,
            lambda_lift: false,
            max_inline_size: 0,
            debug_names: true,
        }
    }
}
/// Context for the proof erasure pass.
pub struct ProofEraser {
    /// Set of variable IDs known to be proof terms.
    pub(super) proof_vars: HashSet<LcnfVarId>,
    /// Number of proofs erased.
    pub(super) erased_count: usize,
}
impl ProofEraser {
    pub(super) fn new() -> Self {
        ProofEraser {
            proof_vars: HashSet::new(),
            erased_count: 0,
        }
    }
    /// Run proof erasure on a function declaration.
    pub(super) fn erase_decl(&mut self, decl: &mut LcnfFunDecl) {
        for param in &decl.params {
            if param.ty == LcnfType::Irrelevant || param.erased {
                self.proof_vars.insert(param.id);
            }
        }
        self.erase_expr(&mut decl.body);
        for param in &mut decl.params {
            if self.proof_vars.contains(&param.id) {
                param.erased = true;
            }
        }
    }
    /// Recursively erase proof terms in an expression.
    pub(super) fn erase_expr(&mut self, expr: &mut LcnfExpr) {
        match expr {
            LcnfExpr::Let {
                id,
                ty,
                value,
                body,
                ..
            } => {
                if *ty == LcnfType::Irrelevant {
                    self.proof_vars.insert(*id);
                    *value = LcnfLetValue::Erased;
                    self.erased_count += 1;
                } else {
                    self.erase_let_value(value);
                }
                self.erase_expr(body);
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                if self.proof_vars.contains(scrutinee) {
                    if let Some(alt) = alts.first_mut() {
                        self.erase_expr(&mut alt.body);
                    }
                    if let Some(def) = default.as_mut() {
                        self.erase_expr(def);
                    }
                } else {
                    for alt in alts.iter_mut() {
                        for p in &alt.params {
                            if p.ty == LcnfType::Irrelevant || p.erased {
                                self.proof_vars.insert(p.id);
                            }
                        }
                        self.erase_expr(&mut alt.body);
                    }
                    if let Some(def) = default.as_mut() {
                        self.erase_expr(def);
                    }
                }
            }
            LcnfExpr::Return(arg) => {
                self.erase_arg(arg);
            }
            LcnfExpr::TailCall(func, args) => {
                self.erase_arg(func);
                for a in args.iter_mut() {
                    self.erase_arg(a);
                }
            }
            LcnfExpr::Unreachable => {}
        }
    }
    /// Erase proof references in a let-value.
    pub(super) fn erase_let_value(&mut self, value: &mut LcnfLetValue) {
        match value {
            LcnfLetValue::App(func, args) => {
                self.erase_arg(func);
                for a in args.iter_mut() {
                    self.erase_arg(a);
                }
            }
            LcnfLetValue::Ctor(_, _, args) => {
                for a in args.iter_mut() {
                    self.erase_arg(a);
                }
            }
            LcnfLetValue::Proj(_, _, var) => {
                if self.proof_vars.contains(var) {
                    *value = LcnfLetValue::Erased;
                    self.erased_count += 1;
                }
            }
            LcnfLetValue::FVar(var) => {
                if self.proof_vars.contains(var) {
                    *value = LcnfLetValue::Erased;
                    self.erased_count += 1;
                }
            }
            LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::Reset(_)
            | LcnfLetValue::Reuse(_, _, _, _) => {}
        }
    }
    /// Erase a proof argument reference.
    pub(super) fn erase_arg(&mut self, arg: &mut LcnfArg) {
        if let LcnfArg::Var(id) = arg {
            if self.proof_vars.contains(id) {
                *arg = LcnfArg::Erased;
                self.erased_count += 1;
            }
        }
    }
}
/// Context for closure conversion.
///
/// Closure conversion makes all captured variables explicit by replacing
/// closures with pairs of (function pointer, environment struct).
pub struct ClosureConverter {
    /// Counter for closure struct names.
    pub(super) closure_counter: u64,
    /// Generated closure struct declarations (name -> field types).
    pub(super) closure_structs: HashMap<String, Vec<(String, LcnfType)>>,
    /// Number of closures converted.
    pub(super) converted_count: usize,
}
impl ClosureConverter {
    pub(super) fn new() -> Self {
        ClosureConverter {
            closure_counter: 0,
            closure_structs: HashMap::new(),
            converted_count: 0,
        }
    }
    /// Generate a fresh closure struct name.
    pub(super) fn fresh_closure_name(&mut self) -> String {
        let name = format!("Closure_{}", self.closure_counter);
        self.closure_counter += 1;
        name
    }
    /// Run closure conversion on a module.
    pub(super) fn convert_module(&mut self, module: &mut LcnfModule) {
        for decl in &mut module.fun_decls {
            if decl.is_lifted {
                self.convert_decl(decl);
            }
        }
    }
    /// Convert closures in a function declaration.
    pub(super) fn convert_decl(&mut self, decl: &mut LcnfFunDecl) {
        let bound: HashSet<LcnfVarId> = decl.params.iter().map(|p| p.id).collect();
        self.convert_expr(&mut decl.body, &bound);
    }
    /// Recursively convert closures in an expression.
    pub(super) fn convert_expr(&mut self, expr: &mut LcnfExpr, bound: &HashSet<LcnfVarId>) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                self.convert_let_value(value, bound);
                let mut new_bound = bound.clone();
                new_bound.insert(*id);
                self.convert_expr(body, &new_bound);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    let mut alt_bound = bound.clone();
                    for p in &alt.params {
                        alt_bound.insert(p.id);
                    }
                    self.convert_expr(&mut alt.body, &alt_bound);
                }
                if let Some(def) = default.as_mut() {
                    self.convert_expr(def, bound);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
    /// Convert closures in a let-value.
    pub(super) fn convert_let_value(
        &mut self,
        value: &mut LcnfLetValue,
        _bound: &HashSet<LcnfVarId>,
    ) {
        if let LcnfLetValue::App(_, args) = value {
            let has_captured = args.iter().any(|a| matches!(a, LcnfArg::Var(_)));
            if has_captured {
                self.converted_count += 1;
            }
        }
    }
    /// Build a closure environment constructor expression.
    pub(super) fn build_closure_env(
        &mut self,
        captured: &[(LcnfVarId, LcnfType)],
    ) -> (String, LcnfLetValue) {
        let closure_name = self.fresh_closure_name();
        let fields: Vec<(String, LcnfType)> = captured
            .iter()
            .enumerate()
            .map(|(i, (_, ty))| (format!("cap_{}", i), ty.clone()))
            .collect();
        self.closure_structs.insert(closure_name.clone(), fields);
        let args: Vec<LcnfArg> = captured.iter().map(|(id, _)| LcnfArg::Var(*id)).collect();
        let ctor_val = LcnfLetValue::Ctor(closure_name.clone(), 0, args);
        (closure_name, ctor_val)
    }
}
/// Errors that can occur during kernel-to-LCNF conversion.
#[derive(Clone, Debug)]
pub enum ConversionError {
    /// Encountered an unsupported expression form.
    UnsupportedExpr(String),
    /// A free variable was not found in the current scope.
    UnboundVariable(String),
    /// The depth limit for recursive conversion was exceeded.
    DepthLimitExceeded(usize),
    /// An invalid binder configuration was encountered.
    InvalidBinder(String),
    /// A type conversion error.
    TypeConversionError(String),
    /// Lambda lifting failed for the given reason.
    LambdaLiftError(String),
    /// Closure conversion failed.
    ClosureConversionError(String),
    /// ANF conversion produced an invalid result.
    AnfConversionError(String),
    /// Proof erasure encountered an unexpected form.
    ProofErasureError(String),
    /// General internal error.
    InternalError(String),
}
