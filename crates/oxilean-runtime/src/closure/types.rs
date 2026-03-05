//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::object::RtObject;
use std::collections::{BTreeSet, HashMap, VecDeque};

/// A simplified lambda lifter that identifies free variables.
#[allow(dead_code)]
pub struct LambdaLifter {
    next_id: u32,
    lifted: Vec<LiftedLambda>,
}
#[allow(dead_code)]
impl LambdaLifter {
    /// Create a new lambda lifter.
    pub fn new() -> Self {
        Self {
            next_id: 0,
            lifted: Vec::new(),
        }
    }
    /// Lift a lambda with the given free variables and arity.
    pub fn lift(&mut self, free_vars: Vec<String>, arity: u32) -> LiftedLambda {
        let id = self.next_id;
        self.next_id += 1;
        let lifted = LiftedLambda {
            id,
            free_vars,
            arity,
        };
        self.lifted.push(lifted.clone());
        lifted
    }
    /// Number of lifted lambdas.
    pub fn len(&self) -> usize {
        self.lifted.len()
    }
    /// Whether any lambdas have been lifted.
    pub fn is_empty(&self) -> bool {
        self.lifted.is_empty()
    }
    /// Access all lifted lambdas.
    pub fn all(&self) -> &[LiftedLambda] {
        &self.lifted
    }
    /// Total free variables across all lifted lambdas.
    pub fn total_free_vars(&self) -> usize {
        self.lifted.iter().map(|l| l.free_vars.len()).sum()
    }
}
/// Result of applying arguments to a PAP.
#[derive(Clone, Debug)]
pub enum PapResult {
    /// Still under-applied (need more arguments).
    UnderApplied(Pap),
    /// Exactly saturated (ready to call).
    Saturated {
        /// The closure to call.
        closure: Closure,
        /// All arguments.
        args: Vec<RtObject>,
    },
    /// Over-applied (extra arguments left over).
    OverApplied {
        /// The closure to call.
        closure: Closure,
        /// Exact arguments for the call.
        args: Vec<RtObject>,
        /// Remaining arguments to apply to the result.
        remaining_args: Vec<RtObject>,
    },
}
/// Result of a batch thunk forcing operation.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BatchForceResult {
    pub forced: usize,
    pub failed: usize,
    pub already_evaluated: usize,
}
/// An arena of closures that supports allocation and reuse.
#[allow(dead_code)]
pub struct ClosureArena {
    closures: Vec<Option<Closure>>,
    free: Vec<usize>,
    alloc_count: u64,
    free_count: u64,
}
#[allow(dead_code)]
impl ClosureArena {
    /// Create an empty closure arena.
    pub fn new() -> Self {
        Self {
            closures: Vec::new(),
            free: Vec::new(),
            alloc_count: 0,
            free_count: 0,
        }
    }
    /// Insert a closure and return its handle.
    pub fn alloc(&mut self, c: Closure) -> ClosureHandle {
        self.alloc_count += 1;
        if let Some(idx) = self.free.pop() {
            self.closures[idx] = Some(c);
            ClosureHandle(idx)
        } else {
            let idx = self.closures.len();
            self.closures.push(Some(c));
            ClosureHandle(idx)
        }
    }
    /// Release a closure back to the pool.
    pub fn free(&mut self, h: ClosureHandle) {
        if let Some(slot) = self.closures.get_mut(h.0) {
            if slot.take().is_some() {
                self.free.push(h.0);
                self.free_count += 1;
            }
        }
    }
    /// Get a reference to a closure by handle.
    pub fn get(&self, h: ClosureHandle) -> Option<&Closure> {
        self.closures.get(h.0)?.as_ref()
    }
    /// Get a mutable reference.
    pub fn get_mut(&mut self, h: ClosureHandle) -> Option<&mut Closure> {
        self.closures.get_mut(h.0)?.as_mut()
    }
    /// Number of live closures.
    pub fn live_count(&self) -> usize {
        self.closures.iter().filter(|s| s.is_some()).count()
    }
    /// Capacity (live + free slots).
    pub fn capacity(&self) -> usize {
        self.closures.len()
    }
    /// Total allocations.
    pub fn alloc_count(&self) -> u64 {
        self.alloc_count
    }
    /// Total frees.
    pub fn free_count(&self) -> u64 {
        self.free_count
    }
    /// Iterate over live closures with their handles.
    pub fn iter(&self) -> impl Iterator<Item = (ClosureHandle, &Closure)> {
        self.closures
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|c| (ClosureHandle(i), c)))
    }
}
/// Information about a function call at runtime.
#[derive(Clone, Debug)]
pub struct CallInfo {
    /// The call convention used.
    pub convention: CallConvention,
    /// The function being called.
    pub fn_ptr: FnPtr,
    /// Number of arguments.
    pub num_args: u16,
    /// Whether this is a recursive call.
    pub is_recursive: bool,
    /// Caller's name (for debugging).
    pub caller_name: Option<String>,
    /// Callee's name (for debugging).
    pub callee_name: Option<String>,
}
impl CallInfo {
    /// Create a new call info.
    pub fn new(convention: CallConvention, fn_ptr: FnPtr, num_args: u16) -> Self {
        CallInfo {
            convention,
            fn_ptr,
            num_args,
            is_recursive: false,
            caller_name: None,
            callee_name: None,
        }
    }
}
/// Handle to a closure in the arena.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClosureHandle(pub usize);
/// Result of closure conversion for a function.
#[derive(Clone, Debug)]
pub struct ClosureConversionResult {
    /// The converted closure.
    pub closure: Closure,
    /// Free variables that were captured.
    pub captured_vars: Vec<String>,
    /// Whether the function was eta-expanded during conversion.
    pub eta_expanded: bool,
    /// Whether any mutual recursion was detected.
    pub has_mutual_rec: bool,
}
impl ClosureConversionResult {
    /// Create a new conversion result.
    pub fn new(closure: Closure) -> Self {
        ClosureConversionResult {
            closure,
            captured_vars: Vec::new(),
            eta_expanded: false,
            has_mutual_rec: false,
        }
    }
    /// Add a captured variable.
    pub fn add_captured(&mut self, var: String) {
        self.captured_vars.push(var);
    }
}
/// A directed graph of closure environment dependencies.
/// Edges mean "closure A captures closure B".
#[allow(dead_code)]
pub struct EnvGraph {
    edges: HashMap<u32, Vec<u32>>,
    node_count: u32,
}
#[allow(dead_code)]
impl EnvGraph {
    /// Create an empty environment graph.
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            node_count: 0,
        }
    }
    /// Add a new node (closure id).
    pub fn add_node(&mut self) -> u32 {
        let id = self.node_count;
        self.node_count += 1;
        self.edges.insert(id, Vec::new());
        id
    }
    /// Add a capture edge: `from` captures `to`.
    pub fn add_edge(&mut self, from: u32, to: u32) {
        self.edges.entry(from).or_default().push(to);
    }
    /// Get the direct captures of a closure.
    pub fn captures(&self, id: u32) -> &[u32] {
        self.edges.get(&id).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Compute the transitive closure of captures (all transitively captured closures).
    pub fn transitive_captures(&self, id: u32) -> Vec<u32> {
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![id];
        let mut result = Vec::new();
        while let Some(curr) = stack.pop() {
            for &dep in self.captures(curr) {
                if visited.insert(dep) {
                    result.push(dep);
                    stack.push(dep);
                }
            }
        }
        result
    }
    /// Detect cycles (a closure transitively capturing itself).
    pub fn has_cycle(&self) -> bool {
        for &start in self.edges.keys() {
            if self.transitive_captures(start).contains(&start) {
                return true;
            }
        }
        false
    }
    /// Number of nodes.
    pub fn node_count(&self) -> u32 {
        self.node_count
    }
    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }
}
/// The call stack.
pub struct CallStack {
    /// Stack frames.
    frames: Vec<StackFrame>,
    /// Maximum stack depth.
    max_depth: usize,
}
impl CallStack {
    /// Create a new call stack.
    pub fn new() -> Self {
        CallStack {
            frames: Vec::new(),
            max_depth: 10_000,
        }
    }
    /// Create with a custom maximum depth.
    pub fn with_max_depth(max_depth: usize) -> Self {
        CallStack {
            frames: Vec::new(),
            max_depth,
        }
    }
    /// Push a frame onto the stack.
    pub fn push(&mut self, frame: StackFrame) -> Result<(), StackOverflow> {
        if self.frames.len() >= self.max_depth {
            return Err(StackOverflow {
                depth: self.frames.len(),
                max_depth: self.max_depth,
            });
        }
        self.frames.push(frame);
        Ok(())
    }
    /// Pop a frame from the stack.
    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }
    /// Get the current (top) frame.
    pub fn current(&self) -> Option<&StackFrame> {
        self.frames.last()
    }
    /// Get the current (top) frame mutably.
    pub fn current_mut(&mut self) -> Option<&mut StackFrame> {
        self.frames.last_mut()
    }
    /// Current stack depth.
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Check if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
    /// Get all frame names (for stack traces).
    pub fn trace(&self) -> Vec<String> {
        self.frames
            .iter()
            .rev()
            .map(|f| f.name.clone().unwrap_or_else(|| format!("{}", f.fn_ptr)))
            .collect()
    }
    /// Set the maximum depth.
    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }
}
/// Builder for constructing closures with captured environments.
pub struct ClosureBuilder {
    /// Function pointer.
    fn_ptr: FnPtr,
    /// Arity.
    arity: u16,
    /// Captured environment values.
    env: Vec<RtObject>,
    /// Name.
    name: Option<String>,
    /// Whether recursive.
    is_recursive: bool,
    /// Whether known.
    is_known: bool,
}
impl ClosureBuilder {
    /// Create a new closure builder.
    pub fn new(fn_ptr: FnPtr, arity: u16) -> Self {
        ClosureBuilder {
            fn_ptr,
            arity,
            env: Vec::new(),
            name: None,
            is_recursive: false,
            is_known: false,
        }
    }
    /// Set the closure name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    /// Add a captured value to the environment.
    pub fn capture(mut self, value: RtObject) -> Self {
        self.env.push(value);
        self
    }
    /// Add multiple captured values.
    pub fn capture_many(mut self, values: Vec<RtObject>) -> Self {
        self.env.extend(values);
        self
    }
    /// Mark as recursive.
    pub fn recursive(mut self) -> Self {
        self.is_recursive = true;
        self
    }
    /// Mark as a known function.
    pub fn known(mut self) -> Self {
        self.is_known = true;
        self
    }
    /// Build the closure.
    pub fn build(self) -> Closure {
        Closure {
            fn_ptr: self.fn_ptr,
            arity: self.arity,
            env: self.env,
            name: self.name,
            is_recursive: self.is_recursive,
            is_known: self.is_known,
        }
    }
}
/// A queue of partial applications waiting to be saturated.
#[allow(dead_code)]
pub struct PapQueue {
    queue: std::collections::VecDeque<Pap>,
    max_size: usize,
    enqueue_count: u64,
    saturated_count: u64,
}
#[allow(dead_code)]
impl PapQueue {
    /// Create a queue with a maximum capacity.
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: std::collections::VecDeque::new(),
            max_size,
            enqueue_count: 0,
            saturated_count: 0,
        }
    }
    /// Enqueue a PAP. Returns false if the queue is full.
    pub fn enqueue(&mut self, pap: Pap) -> bool {
        if self.queue.len() >= self.max_size {
            return false;
        }
        self.queue.push_back(pap);
        self.enqueue_count += 1;
        true
    }
    /// Dequeue the next PAP.
    pub fn dequeue(&mut self) -> Option<Pap> {
        self.queue.pop_front()
    }
    /// Apply additional args to the front PAP.
    pub fn apply_front(&mut self, args: Vec<RtObject>) -> Option<PapResult> {
        let pap = self.dequeue()?;
        let result = pap.apply(&args);
        if matches!(result, PapResult::Saturated { .. }) {
            self.saturated_count += 1;
        }
        Some(result)
    }
    /// Current queue length.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    /// Total PAPs enqueued.
    pub fn enqueue_count(&self) -> u64 {
        self.enqueue_count
    }
    /// Total saturated PAPs.
    pub fn saturated_count(&self) -> u64 {
        self.saturated_count
    }
}
/// A partial application (PAP).
///
/// Created when a closure is applied to fewer arguments than its arity.
/// The PAP stores the original closure and the arguments applied so far.
#[derive(Clone, Debug)]
pub struct Pap {
    /// The original closure being partially applied.
    pub closure: Closure,
    /// Arguments applied so far.
    pub args: Vec<RtObject>,
}
impl Pap {
    /// Create a new PAP.
    pub fn new(closure: Closure, args: Vec<RtObject>) -> Self {
        Pap { closure, args }
    }
    /// Number of arguments still needed.
    pub fn remaining_arity(&self) -> u16 {
        self.closure.arity.saturating_sub(self.args.len() as u16)
    }
    /// Total arity of the underlying closure.
    pub fn total_arity(&self) -> u16 {
        self.closure.arity
    }
    /// Number of arguments applied so far.
    pub fn num_applied(&self) -> usize {
        self.args.len()
    }
    /// Check if the PAP is fully saturated (should not happen, but defensive).
    pub fn is_saturated(&self) -> bool {
        self.args.len() >= self.closure.arity as usize
    }
    /// Apply additional arguments. Returns either a new PAP or indicates saturation.
    pub fn apply(&self, new_args: &[RtObject]) -> PapResult {
        let total_args = self.args.len() + new_args.len();
        let arity = self.closure.arity as usize;
        if total_args < arity {
            let mut all_args = self.args.clone();
            all_args.extend_from_slice(new_args);
            PapResult::UnderApplied(Pap::new(self.closure.clone(), all_args))
        } else if total_args == arity {
            let mut all_args = self.args.clone();
            all_args.extend_from_slice(new_args);
            PapResult::Saturated {
                closure: self.closure.clone(),
                args: all_args,
            }
        } else {
            let mut exact_args = self.args.clone();
            let needed = arity - self.args.len();
            exact_args.extend_from_slice(&new_args[..needed]);
            let remaining = new_args[needed..].to_vec();
            PapResult::OverApplied {
                closure: self.closure.clone(),
                args: exact_args,
                remaining_args: remaining,
            }
        }
    }
    /// Get all arguments (environment + applied).
    pub fn all_args(&self) -> Vec<RtObject> {
        let mut args = self.closure.env.clone();
        args.extend(self.args.iter().cloned());
        args
    }
}
/// Optimization report for a closure.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ClosureOptReport {
    pub inlined_paps: usize,
    pub specialized_arities: usize,
    pub env_reduced_vars: usize,
    pub removed_dead_closures: usize,
}
#[allow(dead_code)]
impl ClosureOptReport {
    /// Whether any optimizations were applied.
    pub fn has_changes(&self) -> bool {
        self.inlined_paps > 0
            || self.specialized_arities > 0
            || self.env_reduced_vars > 0
            || self.removed_dead_closures > 0
    }
    /// Total optimization actions.
    pub fn total(&self) -> usize {
        self.inlined_paps
            + self.specialized_arities
            + self.env_reduced_vars
            + self.removed_dead_closures
    }
}
/// A record of a potential inline candidate.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InlineCandidate {
    pub fn_id: u32,
    pub call_site_count: u32,
    pub env_size: usize,
    pub arity: u32,
}
#[allow(dead_code)]
impl InlineCandidate {
    /// Score: lower is more profitable to inline.
    pub fn inline_score(&self) -> f64 {
        self.env_size as f64 / (self.call_site_count as f64 + 1.0)
    }
    /// Whether the candidate is a leaf (no recursive calls).
    pub fn is_leaf_candidate(&self) -> bool {
        self.env_size <= 4 && self.arity <= 3
    }
}
/// A function pointer in the runtime.
///
/// Functions are identified by an index into the global function table.
/// This allows us to represent function pointers without actual raw pointers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FnPtr {
    /// Index into the global function table.
    pub index: u32,
    /// Module that defines this function (for multi-module support).
    pub module_id: u16,
}
impl FnPtr {
    /// Create a new function pointer.
    pub fn new(index: u32) -> Self {
        FnPtr {
            index,
            module_id: 0,
        }
    }
    /// Create a function pointer with module ID.
    pub fn with_module(index: u32, module_id: u16) -> Self {
        FnPtr { index, module_id }
    }
    /// Null/invalid function pointer.
    pub fn null() -> Self {
        FnPtr {
            index: u32::MAX,
            module_id: 0,
        }
    }
    /// Check if this is a null pointer.
    pub fn is_null(&self) -> bool {
        self.index == u32::MAX
    }
}
/// A stack frame for function calls.
#[derive(Clone, Debug)]
pub struct StackFrame {
    /// The function being executed.
    pub fn_ptr: FnPtr,
    /// Local variables.
    pub locals: Vec<RtObject>,
    /// Arguments.
    pub args: Vec<RtObject>,
    /// Captured environment (if executing a closure).
    pub env: Vec<RtObject>,
    /// Return address (instruction pointer offset).
    pub return_ip: u32,
    /// Frame name (for debugging).
    pub name: Option<String>,
}
impl StackFrame {
    /// Create a new stack frame.
    pub fn new(fn_ptr: FnPtr, args: Vec<RtObject>, env: Vec<RtObject>, frame_size: usize) -> Self {
        StackFrame {
            fn_ptr,
            locals: vec![RtObject::unit(); frame_size],
            args,
            env,
            return_ip: 0,
            name: None,
        }
    }
    /// Get a local variable.
    pub fn get_local(&self, index: usize) -> Option<&RtObject> {
        self.locals.get(index)
    }
    /// Set a local variable.
    pub fn set_local(&mut self, index: usize, value: RtObject) -> bool {
        if index < self.locals.len() {
            self.locals[index] = value;
            true
        } else {
            false
        }
    }
    /// Get an argument.
    pub fn get_arg(&self, index: usize) -> Option<&RtObject> {
        self.args.get(index)
    }
    /// Get a captured environment value.
    pub fn get_env(&self, index: usize) -> Option<&RtObject> {
        self.env.get(index)
    }
}
/// The result of lifting a lambda to a closed form.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LiftedLambda {
    /// The lifted function's identifier.
    pub id: u32,
    /// Names of free variables captured from the outer scope.
    pub free_vars: Vec<String>,
    /// Arity of the function.
    pub arity: u32,
}
/// A simple call inliner that collects candidates.
#[allow(dead_code)]
pub struct CallInliner {
    candidates: Vec<InlineCandidate>,
    inlined_count: u64,
}
#[allow(dead_code)]
impl CallInliner {
    /// Create a new inliner.
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
            inlined_count: 0,
        }
    }
    /// Register a candidate.
    pub fn register(&mut self, candidate: InlineCandidate) {
        self.candidates.push(candidate);
    }
    /// Get the top-N candidates by inline score.
    pub fn top_candidates(&self, n: usize) -> Vec<&InlineCandidate> {
        let mut sorted: Vec<&InlineCandidate> = self.candidates.iter().collect();
        sorted.sort_by(|a, b| {
            a.inline_score()
                .partial_cmp(&b.inline_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted.truncate(n);
        sorted
    }
    /// Mark a candidate as inlined.
    pub fn record_inline(&mut self, fn_id: u32) {
        self.candidates.retain(|c| c.fn_id != fn_id);
        self.inlined_count += 1;
    }
    /// Total inlines performed.
    pub fn inlined_count(&self) -> u64 {
        self.inlined_count
    }
    /// Remaining candidate count.
    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }
}
/// An entry in the global function table.
///
/// Each compiled function has an entry that describes its properties.
#[derive(Clone, Debug)]
pub struct FunctionEntry {
    /// The function's name.
    pub name: String,
    /// Total arity.
    pub arity: u16,
    /// Number of environment variables expected (for closures).
    pub env_size: u16,
    /// Preferred call convention.
    pub convention: CallConvention,
    /// Whether this function is recursive.
    pub is_recursive: bool,
    /// Whether this function is a built-in.
    pub is_builtin: bool,
    /// Whether this function has been inlined at all call sites.
    pub is_inlined: bool,
    /// Stack frame size (in words) needed for local variables.
    pub frame_size: u16,
    /// Parameter names (for debugging).
    pub param_names: Vec<String>,
}
impl FunctionEntry {
    /// Create a new function entry.
    pub fn new(name: String, arity: u16) -> Self {
        FunctionEntry {
            name,
            arity,
            env_size: 0,
            convention: CallConvention::ClosureCall,
            is_recursive: false,
            is_builtin: false,
            is_inlined: false,
            frame_size: 0,
            param_names: Vec::new(),
        }
    }
    /// Create a built-in function entry.
    pub fn builtin(name: String, arity: u16) -> Self {
        FunctionEntry {
            name,
            arity,
            env_size: 0,
            convention: CallConvention::BuiltinCall,
            is_recursive: false,
            is_builtin: true,
            is_inlined: false,
            frame_size: 0,
            param_names: Vec::new(),
        }
    }
}
/// Stack overflow error.
#[derive(Clone, Debug)]
pub struct StackOverflow {
    /// Current depth when overflow occurred.
    pub depth: usize,
    /// Maximum allowed depth.
    pub max_depth: usize,
}
/// The global function table.
///
/// Maps function indices to their entries. Built during compilation
/// and used at runtime for function dispatch.
pub struct FunctionTable {
    /// Entries indexed by function pointer index.
    pub(super) entries: Vec<FunctionEntry>,
    /// Name-to-index lookup.
    name_index: HashMap<String, u32>,
}
impl FunctionTable {
    /// Create a new empty function table.
    pub fn new() -> Self {
        FunctionTable {
            entries: Vec::new(),
            name_index: HashMap::new(),
        }
    }
    /// Create with pre-allocated capacity.
    pub fn with_capacity(cap: usize) -> Self {
        FunctionTable {
            entries: Vec::with_capacity(cap),
            name_index: HashMap::with_capacity(cap),
        }
    }
    /// Register a new function and return its pointer.
    pub fn register(&mut self, entry: FunctionEntry) -> FnPtr {
        let index = self.entries.len() as u32;
        self.name_index.insert(entry.name.clone(), index);
        self.entries.push(entry);
        FnPtr::new(index)
    }
    /// Look up a function entry by pointer.
    pub fn get(&self, ptr: FnPtr) -> Option<&FunctionEntry> {
        self.entries.get(ptr.index as usize)
    }
    /// Look up a function entry by name.
    pub fn get_by_name(&self, name: &str) -> Option<(FnPtr, &FunctionEntry)> {
        self.name_index
            .get(name)
            .and_then(|&idx| self.entries.get(idx as usize).map(|e| (FnPtr::new(idx), e)))
    }
    /// Number of registered functions.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Iterate over all entries.
    pub fn iter(&self) -> impl Iterator<Item = (FnPtr, &FunctionEntry)> {
        self.entries
            .iter()
            .enumerate()
            .map(|(i, e)| (FnPtr::new(i as u32), e))
    }
    /// Register built-in functions.
    pub fn register_builtins(&mut self) {
        self.register(FunctionEntry::builtin("Nat.add".to_string(), 2));
        self.register(FunctionEntry::builtin("Nat.sub".to_string(), 2));
        self.register(FunctionEntry::builtin("Nat.mul".to_string(), 2));
        self.register(FunctionEntry::builtin("Nat.div".to_string(), 2));
        self.register(FunctionEntry::builtin("Nat.mod".to_string(), 2));
        self.register(FunctionEntry::builtin("Nat.beq".to_string(), 2));
        self.register(FunctionEntry::builtin("Nat.ble".to_string(), 2));
        self.register(FunctionEntry::builtin("Bool.and".to_string(), 2));
        self.register(FunctionEntry::builtin("Bool.or".to_string(), 2));
        self.register(FunctionEntry::builtin("Bool.not".to_string(), 1));
        self.register(FunctionEntry::builtin("String.append".to_string(), 2));
        self.register(FunctionEntry::builtin("String.length".to_string(), 1));
        self.register(FunctionEntry::builtin("String.mk".to_string(), 1));
        self.register(FunctionEntry::builtin("IO.println".to_string(), 2));
        self.register(FunctionEntry::builtin("IO.getLine".to_string(), 1));
        self.register(FunctionEntry::builtin("IO.pure".to_string(), 2));
        self.register(FunctionEntry::builtin("IO.bind".to_string(), 4));
        self.register(FunctionEntry::builtin("Array.mk".to_string(), 1));
        self.register(FunctionEntry::builtin("Array.push".to_string(), 2));
        self.register(FunctionEntry::builtin("Array.get!".to_string(), 2));
        self.register(FunctionEntry::builtin("Array.set!".to_string(), 3));
        self.register(FunctionEntry::builtin("Array.size".to_string(), 1));
    }
}
/// A serializer for closure environments (mapping string names to u64 values).
#[allow(dead_code)]
pub struct ClosureSerializer;
#[allow(dead_code)]
impl ClosureSerializer {
    /// Serialize an environment map to bytes.
    pub fn serialize_env(env: &HashMap<String, u64>) -> Vec<u8> {
        let mut out = Vec::new();
        let count = env.len() as u32;
        out.extend_from_slice(&count.to_le_bytes());
        let mut pairs: Vec<(&String, &u64)> = env.iter().collect();
        pairs.sort_by_key(|(k, _)| k.as_str());
        for (key, val) in pairs {
            let key_bytes = key.as_bytes();
            out.extend_from_slice(&(key_bytes.len() as u32).to_le_bytes());
            out.extend_from_slice(key_bytes);
            out.extend_from_slice(&val.to_le_bytes());
        }
        out
    }
    /// Deserialize an environment map from bytes.
    pub fn deserialize_env(data: &[u8]) -> Option<HashMap<String, u64>> {
        if data.len() < 4 {
            return None;
        }
        let count = u32::from_le_bytes(data[0..4].try_into().ok()?) as usize;
        let mut pos = 4;
        let mut env = HashMap::new();
        for _ in 0..count {
            if pos + 4 > data.len() {
                return None;
            }
            let key_len = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?) as usize;
            pos += 4;
            if pos + key_len > data.len() {
                return None;
            }
            let key = std::str::from_utf8(&data[pos..pos + key_len])
                .ok()?
                .to_string();
            pos += key_len;
            if pos + 8 > data.len() {
                return None;
            }
            let val = u64::from_le_bytes(data[pos..pos + 8].try_into().ok()?);
            pos += 8;
            env.insert(key, val);
        }
        Some(env)
    }
}
/// A basic closure optimizer.
#[allow(dead_code)]
pub struct ClosureOptimizer {
    inline_threshold: usize,
}
#[allow(dead_code)]
impl ClosureOptimizer {
    /// Create an optimizer with the given inlining threshold (env size).
    pub fn new(inline_threshold: usize) -> Self {
        Self { inline_threshold }
    }
    /// Attempt to reduce a closure's environment by removing unused captures.
    pub fn reduce_env(
        &self,
        closure: &mut Closure,
        used_names: &std::collections::HashSet<String>,
    ) -> usize {
        let before = closure.env.len();
        closure.env.retain(|obj| {
            let _ = obj;
            let _ = used_names;
            true
        });
        before - closure.env.len()
    }
    /// Specialize a PAP by precomputing partial argument count.
    pub fn optimize_pap(&self, pap: &Pap) -> Option<Closure> {
        if pap.args.len() == pap.closure.arity as usize {
            Some(Closure {
                fn_ptr: pap.closure.fn_ptr,
                arity: 0,
                env: pap
                    .closure
                    .env
                    .iter()
                    .chain(pap.args.iter())
                    .cloned()
                    .collect(),
                name: pap.closure.name.clone(),
                is_recursive: pap.closure.is_recursive,
                is_known: pap.closure.is_known,
            })
        } else {
            None
        }
    }
    /// Inline PAP optimization across the function table.
    pub fn inline_paps(&self, table: &mut FunctionTable) -> usize {
        let _ = table;
        0
    }
}
/// A flat, heap-allocated closure that can be serialized (no dyn fn ptrs).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FlatClosure {
    /// Function index (into a global function table).
    pub fn_index: u32,
    /// Arity.
    pub arity: u32,
    /// Environment values (limited to simple types here: u64).
    pub env: Vec<u64>,
}
#[allow(dead_code)]
impl FlatClosure {
    /// Create a new flat closure.
    pub fn new(fn_index: u32, arity: u32, env: Vec<u64>) -> Self {
        Self {
            fn_index,
            arity,
            env,
        }
    }
    /// Serialize to bytes.
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.fn_index.to_le_bytes());
        out.extend_from_slice(&self.arity.to_le_bytes());
        let env_len = self.env.len() as u32;
        out.extend_from_slice(&env_len.to_le_bytes());
        for v in &self.env {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }
    /// Deserialize from bytes.
    pub fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < 12 {
            return None;
        }
        let fn_index = u32::from_le_bytes(data[0..4].try_into().ok()?);
        let arity = u32::from_le_bytes(data[4..8].try_into().ok()?);
        let env_len = u32::from_le_bytes(data[8..12].try_into().ok()?) as usize;
        if data.len() < 12 + env_len * 8 {
            return None;
        }
        let mut env = Vec::with_capacity(env_len);
        for i in 0..env_len {
            let off = 12 + i * 8;
            let v = u64::from_le_bytes(data[off..off + 8].try_into().ok()?);
            env.push(v);
        }
        Some(Self {
            fn_index,
            arity,
            env,
        })
    }
    /// Whether this is a thunk (arity == 0).
    pub fn is_thunk(&self) -> bool {
        self.arity == 0
    }
    /// Env size.
    pub fn env_size(&self) -> usize {
        self.env.len()
    }
}
/// Call convention for a function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallConvention {
    /// Standard closure call (through the closure mechanism).
    ClosureCall,
    /// Direct call (function is known at compile time).
    DirectCall,
    /// Tail call (reuse the current stack frame).
    TailCall,
    /// Indirect call (through a function pointer).
    IndirectCall,
    /// Built-in operation (handled specially by the runtime).
    BuiltinCall,
}
impl CallConvention {
    /// Check if this convention supports tail call optimization.
    pub fn supports_tco(&self) -> bool {
        matches!(self, CallConvention::TailCall | CallConvention::DirectCall)
    }
    /// Check if this is a direct (non-closure) call.
    pub fn is_direct(&self) -> bool {
        matches!(self, CallConvention::DirectCall | CallConvention::TailCall)
    }
}
/// Statistics about closure operations.
#[derive(Clone, Debug, Default)]
pub struct ClosureStats {
    /// Number of closures created.
    pub closures_created: u64,
    /// Number of PAPs created.
    pub paps_created: u64,
    /// Number of exact calls.
    pub exact_calls: u64,
    /// Number of under-applications.
    pub under_applications: u64,
    /// Number of over-applications.
    pub over_applications: u64,
    /// Number of tail calls.
    pub tail_calls: u64,
    /// Number of direct calls.
    pub direct_calls: u64,
    /// Number of built-in calls.
    pub builtin_calls: u64,
    /// Peak stack depth.
    pub peak_stack_depth: usize,
}
impl ClosureStats {
    /// Create new empty statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a closure creation.
    pub fn record_closure_created(&mut self) {
        self.closures_created += 1;
    }
    /// Record a PAP creation.
    pub fn record_pap_created(&mut self) {
        self.paps_created += 1;
    }
    /// Record an exact call.
    pub fn record_exact_call(&mut self) {
        self.exact_calls += 1;
    }
    /// Record an under-application.
    pub fn record_under_application(&mut self) {
        self.under_applications += 1;
    }
    /// Record an over-application.
    pub fn record_over_application(&mut self) {
        self.over_applications += 1;
    }
    /// Record a tail call.
    pub fn record_tail_call(&mut self) {
        self.tail_calls += 1;
    }
    /// Record a direct call.
    pub fn record_direct_call(&mut self) {
        self.direct_calls += 1;
    }
    /// Record a built-in call.
    pub fn record_builtin_call(&mut self) {
        self.builtin_calls += 1;
    }
    /// Update peak stack depth.
    pub fn update_peak_depth(&mut self, depth: usize) {
        if depth > self.peak_stack_depth {
            self.peak_stack_depth = depth;
        }
    }
    /// Total number of calls.
    pub fn total_calls(&self) -> u64 {
        self.exact_calls
            + self.under_applications
            + self.over_applications
            + self.tail_calls
            + self.direct_calls
            + self.builtin_calls
    }
    /// Reset all statistics.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
/// A registry that maps string names to closures.
#[allow(dead_code)]
pub struct ClosureRegistry {
    map: HashMap<String, Closure>,
    access_counts: HashMap<String, u64>,
}
#[allow(dead_code)]
impl ClosureRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            access_counts: HashMap::new(),
        }
    }
    /// Register a closure with a name.
    pub fn register(&mut self, name: &str, c: Closure) {
        self.map.insert(name.to_string(), c);
        self.access_counts.insert(name.to_string(), 0);
    }
    /// Look up a closure by name.
    pub fn lookup(&mut self, name: &str) -> Option<&Closure> {
        if let Some(count) = self.access_counts.get_mut(name) {
            *count += 1;
        }
        self.map.get(name)
    }
    /// Remove a closure.
    pub fn unregister(&mut self, name: &str) -> Option<Closure> {
        self.access_counts.remove(name);
        self.map.remove(name)
    }
    /// Number of registered closures.
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// Access count for a name.
    pub fn access_count(&self, name: &str) -> u64 {
        self.access_counts.get(name).copied().unwrap_or(0)
    }
    /// Most accessed closure.
    pub fn most_accessed(&self) -> Option<&str> {
        self.access_counts
            .iter()
            .max_by_key(|(_, &c)| c)
            .map(|(name, _)| name.as_str())
    }
    /// Iterate over all registered names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.map.keys().map(|s| s.as_str())
    }
}
/// Estimates the memory footprint of a closure.
#[allow(dead_code)]
pub struct ClosureSizeEstimator {
    pub(super) ptr_size: usize,
    pub(super) object_overhead: usize,
}
#[allow(dead_code)]
impl ClosureSizeEstimator {
    /// Create an estimator for a given pointer size.
    pub fn new(ptr_size: usize) -> Self {
        Self {
            ptr_size,
            object_overhead: 16,
        }
    }
    /// Estimate closure heap bytes.
    pub fn estimate_closure(&self, c: &Closure) -> usize {
        self.object_overhead + self.ptr_size + 4 + 4 + c.env.len() * self.ptr_size
    }
    /// Estimate PAP heap bytes.
    pub fn estimate_pap(&self, pap: &Pap) -> usize {
        self.object_overhead + self.estimate_closure(&pap.closure) + pap.args.len() * self.ptr_size
    }
    /// Estimate a flat closure.
    pub fn estimate_flat(&self, fc: &FlatClosure) -> usize {
        self.object_overhead + 4 + 4 + fc.env.len() * 8
    }
}
/// A set of variable names captured by a closure.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct CaptureSet {
    pub(super) vars: std::collections::BTreeSet<String>,
}
#[allow(dead_code)]
impl CaptureSet {
    /// Create an empty capture set.
    pub fn new() -> Self {
        Self {
            vars: std::collections::BTreeSet::new(),
        }
    }
    /// Insert a variable into the capture set.
    pub fn capture(&mut self, name: &str) {
        self.vars.insert(name.to_string());
    }
    /// Remove a variable (e.g., it's now bound locally).
    pub fn bind(&mut self, name: &str) {
        self.vars.remove(name);
    }
    /// Whether a variable is captured.
    pub fn is_captured(&self, name: &str) -> bool {
        self.vars.contains(name)
    }
    /// Number of captured variables.
    pub fn len(&self) -> usize {
        self.vars.len()
    }
    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }
    /// Iterate over captured variable names in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.vars.iter().map(|s| s.as_str())
    }
    /// Union with another capture set.
    pub fn union(&mut self, other: &CaptureSet) {
        for name in &other.vars {
            self.vars.insert(name.clone());
        }
    }
    /// Difference: remove all variables in `other` from self.
    pub fn difference(&mut self, other: &CaptureSet) {
        for name in &other.vars {
            self.vars.remove(name);
        }
    }
}
/// A runtime closure with captured environment.
///
/// The closure captures free variables in a flat array. The function body
/// is represented by a `FnPtr` into the code table.
#[derive(Clone, Debug)]
pub struct Closure {
    /// Pointer to the function body.
    pub fn_ptr: FnPtr,
    /// Total arity (number of parameters the function expects).
    pub arity: u16,
    /// Captured environment values.
    pub env: Vec<RtObject>,
    /// Name of the closure (for debugging).
    pub name: Option<String>,
    /// Whether this closure is recursive.
    pub is_recursive: bool,
    /// Whether this closure has been marked as a known function
    /// (can be called directly without going through the closure mechanism).
    pub is_known: bool,
}
impl Closure {
    /// Create a new closure.
    pub fn new(fn_ptr: FnPtr, arity: u16, env: Vec<RtObject>) -> Self {
        Closure {
            fn_ptr,
            arity,
            env,
            name: None,
            is_recursive: false,
            is_known: false,
        }
    }
    /// Create a closure with a name.
    pub fn named(name: String, fn_ptr: FnPtr, arity: u16, env: Vec<RtObject>) -> Self {
        Closure {
            fn_ptr,
            arity,
            env,
            name: Some(name),
            is_recursive: false,
            is_known: false,
        }
    }
    /// Create a simple closure with no captured environment.
    pub fn simple(fn_ptr: FnPtr, arity: u16) -> Self {
        Closure::new(fn_ptr, arity, Vec::new())
    }
    /// Number of captured environment variables.
    pub fn env_size(&self) -> usize {
        self.env.len()
    }
    /// Get a captured value by index.
    pub fn get_env(&self, index: usize) -> Option<&RtObject> {
        self.env.get(index)
    }
    /// Set a captured value by index.
    pub fn set_env(&mut self, index: usize, value: RtObject) -> bool {
        if index < self.env.len() {
            self.env[index] = value;
            true
        } else {
            false
        }
    }
    /// Extend the environment with additional values.
    pub fn extend_env(&mut self, values: &[RtObject]) {
        self.env.extend_from_slice(values);
    }
    /// Mark as recursive.
    pub fn set_recursive(&mut self) {
        self.is_recursive = true;
    }
    /// Mark as a known function.
    pub fn set_known(&mut self) {
        self.is_known = true;
    }
}
/// A group of mutually recursive closures.
///
/// All closures in the group share a common environment that includes
/// references to each other.
#[derive(Clone, Debug)]
pub struct MutualRecGroup {
    /// The closures in the group.
    pub closures: Vec<Closure>,
    /// Shared environment that all closures can access.
    pub shared_env: Vec<RtObject>,
}
impl MutualRecGroup {
    /// Create a new mutual recursive group.
    pub fn new(closures: Vec<Closure>, shared_env: Vec<RtObject>) -> Self {
        MutualRecGroup {
            closures,
            shared_env,
        }
    }
    /// Get a specific closure by index.
    pub fn get_closure(&self, index: usize) -> Option<&Closure> {
        self.closures.get(index)
    }
    /// Number of closures in the group.
    pub fn size(&self) -> usize {
        self.closures.len()
    }
}
