//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl CUDAWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CUDAWorklist {
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
/// A CUDA kernel (`__global__` function).
#[derive(Debug, Clone, PartialEq)]
pub struct CudaKernel {
    /// Kernel name
    pub name: String,
    /// Parameter list
    pub params: Vec<CudaParam>,
    /// Shared memory declarations (emitted at the top of the kernel body)
    pub shared_mem_decls: Vec<SharedMemDecl>,
    /// Kernel body statements
    pub body: Vec<CudaStmt>,
    /// Optional `__launch_bounds__` annotation
    pub launch_bounds: Option<LaunchBounds>,
}
impl CudaKernel {
    /// Create a new kernel with no launch bounds.
    pub fn new(name: impl Into<String>) -> Self {
        CudaKernel {
            name: name.into(),
            params: Vec::new(),
            shared_mem_decls: Vec::new(),
            body: Vec::new(),
            launch_bounds: None,
        }
    }
    /// Append a parameter.
    pub fn add_param(mut self, p: CudaParam) -> Self {
        self.params.push(p);
        self
    }
    /// Append a shared-memory declaration.
    pub fn add_shared(mut self, s: SharedMemDecl) -> Self {
        self.shared_mem_decls.push(s);
        self
    }
    /// Append a body statement.
    pub fn add_stmt(mut self, s: CudaStmt) -> Self {
        self.body.push(s);
        self
    }
    /// Set launch bounds.
    pub fn with_launch_bounds(mut self, lb: LaunchBounds) -> Self {
        self.launch_bounds = Some(lb);
        self
    }
}
/// CUDA kernel launch configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct LaunchConfig {
    /// Grid dimensions (number of blocks)
    pub grid: CudaExpr,
    /// Block dimensions (threads per block)
    pub block: CudaExpr,
    /// Dynamic shared memory bytes (0 if none)
    pub shared_mem: CudaExpr,
    /// CUDA stream (None → default stream)
    pub stream: Option<CudaExpr>,
}
impl LaunchConfig {
    /// Create a simple 1-D launch config with no dynamic shared memory.
    pub fn simple_1d(grid: CudaExpr, block: CudaExpr) -> Self {
        LaunchConfig {
            grid,
            block,
            shared_mem: CudaExpr::LitInt(0),
            stream: None,
        }
    }
}
/// Top-level CUDA module representing a single `.cu` file.
#[derive(Debug, Clone, PartialEq)]
pub struct CudaModule {
    /// `#include` directives (just the header names, e.g. `"cuda_runtime.h"`)
    pub includes: Vec<String>,
    /// `__constant__` memory declarations at file scope
    pub constant_decls: Vec<(CudaType, String, Option<CudaExpr>)>,
    /// `__device__` (or `__host__ __device__`) helper functions
    pub device_functions: Vec<DeviceFunction>,
    /// `__global__` kernels
    pub kernels: Vec<CudaKernel>,
    /// Host-side code (helper functions, `main`, etc.) as raw strings
    pub host_code: Vec<String>,
}
impl CudaModule {
    /// Create an empty module with standard CUDA includes.
    pub fn new() -> Self {
        CudaModule {
            includes: vec![
                "cuda_runtime.h".to_string(),
                "device_launch_parameters.h".to_string(),
            ],
            constant_decls: Vec::new(),
            device_functions: Vec::new(),
            kernels: Vec::new(),
            host_code: Vec::new(),
        }
    }
    /// Add an `#include` (just the name; angle brackets / quotes are added by emitter).
    pub fn add_include(mut self, header: impl Into<String>) -> Self {
        self.includes.push(header.into());
        self
    }
    /// Declare a `__constant__` variable at file scope.
    pub fn add_constant(
        mut self,
        ty: CudaType,
        name: impl Into<String>,
        init: Option<CudaExpr>,
    ) -> Self {
        self.constant_decls.push((ty, name.into(), init));
        self
    }
    /// Add a device function.
    pub fn add_device_function(mut self, f: DeviceFunction) -> Self {
        self.device_functions.push(f);
        self
    }
    /// Add a kernel.
    pub fn add_kernel(mut self, k: CudaKernel) -> Self {
        self.kernels.push(k);
        self
    }
    /// Append raw host-side C++ code.
    pub fn add_host_code(mut self, code: impl Into<String>) -> Self {
        self.host_code.push(code.into());
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CUDAPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl CUDAPassStats {
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
/// Worklist for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl CUDAExtWorklist {
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
/// Emitter state for producing CUDA `.cu` source code.
pub struct CudaBackend {
    pub(super) indent_width: usize,
}
impl CudaBackend {
    /// Create a new backend with 4-space indentation.
    pub fn new() -> Self {
        CudaBackend { indent_width: 4 }
    }
    /// Create a backend with a custom indent width.
    pub fn with_indent(indent_width: usize) -> Self {
        CudaBackend { indent_width }
    }
    pub(super) fn indent(&self, depth: usize) -> String {
        " ".repeat(self.indent_width * depth)
    }
    /// Emit a CUDA expression to a string.
    pub fn emit_expr(&self, expr: &CudaExpr) -> String {
        expr.emit()
    }
    /// Emit a single statement at the given indentation depth.
    pub fn emit_stmt(&self, stmt: &CudaStmt, depth: usize) -> String {
        let ind = self.indent(depth);
        match stmt {
            CudaStmt::VarDecl { ty, name, init } => match init {
                Some(expr) => format!("{}{} {} = {};", ind, ty, name, expr.emit()),
                None => format!("{}{} {};", ind, ty, name),
            },
            CudaStmt::Assign { lhs, rhs } => {
                format!("{}{} = {};", ind, lhs.emit(), rhs.emit())
            }
            CudaStmt::CompoundAssign { lhs, op, rhs } => {
                format!("{}{} {}= {};", ind, lhs.emit(), op, rhs.emit())
            }
            CudaStmt::IfElse {
                cond,
                then_body,
                else_body,
            } => self.emit_if_else(cond, then_body, else_body.as_deref(), depth),
            CudaStmt::ForLoop {
                init,
                cond,
                step,
                body,
            } => self.emit_for_loop(init, cond, step, body, depth),
            CudaStmt::WhileLoop { cond, body } => self.emit_while(cond, body, depth),
            CudaStmt::KernelLaunch { name, config, args } => {
                self.emit_kernel_launch(name, config, args, depth)
            }
            CudaStmt::CudaMalloc { ptr, size } => {
                format!("{}cudaMalloc((void**)&{}, {});", ind, ptr, size.emit())
            }
            CudaStmt::CudaMemcpy {
                dst,
                src,
                size,
                kind,
            } => {
                format!(
                    "{}cudaMemcpy({}, {}, {}, {});",
                    ind,
                    dst.emit(),
                    src.emit(),
                    size.emit(),
                    kind
                )
            }
            CudaStmt::CudaFree(ptr) => format!("{}cudaFree({});", ind, ptr.emit()),
            CudaStmt::Return(Some(expr)) => format!("{}return {};", ind, expr.emit()),
            CudaStmt::Return(None) => format!("{}return;", ind),
            CudaStmt::Expr(expr) => format!("{}{};", ind, expr.emit()),
            CudaStmt::DeviceSync => format!("{}cudaDeviceSynchronize();", ind),
            CudaStmt::CheckError(expr) => format!("{}CUDA_CHECK({});", ind, expr.emit()),
            CudaStmt::Block(stmts) => {
                let mut out = format!("{}{{\n", ind);
                for s in stmts {
                    out.push_str(&self.emit_stmt(s, depth + 1));
                    out.push('\n');
                }
                out.push_str(&format!("{}}}", ind));
                out
            }
            CudaStmt::Break => format!("{}break;", ind),
            CudaStmt::Continue => format!("{}continue;", ind),
        }
    }
    pub(super) fn emit_if_else(
        &self,
        cond: &CudaExpr,
        then_body: &[CudaStmt],
        else_body: Option<&[CudaStmt]>,
        depth: usize,
    ) -> String {
        let ind = self.indent(depth);
        let inner = self.indent(depth + 1);
        let mut out = format!("{}if ({}) {{\n", ind, cond.emit());
        for s in then_body {
            out.push_str(&self.emit_stmt(s, depth + 1));
            out.push('\n');
        }
        out.push_str(&format!("{}}}", ind));
        if let Some(eb) = else_body {
            out.push_str(" else {\n");
            for s in eb {
                out.push_str(&self.emit_stmt(s, depth + 1));
                out.push('\n');
            }
            out.push_str(&format!("{}}}", ind));
        }
        let _ = inner;
        out
    }
    pub(super) fn emit_for_loop(
        &self,
        init: &CudaStmt,
        cond: &CudaExpr,
        step: &CudaExpr,
        body: &[CudaStmt],
        depth: usize,
    ) -> String {
        let ind = self.indent(depth);
        let init_str = self.emit_stmt(init, 0).trim().to_string();
        let init_header = init_str.trim_end_matches(';');
        let mut out = format!(
            "{}for ({}; {}; {}) {{\n",
            ind,
            init_header,
            cond.emit(),
            step.emit()
        );
        for s in body {
            out.push_str(&self.emit_stmt(s, depth + 1));
            out.push('\n');
        }
        out.push_str(&format!("{}}}", ind));
        out
    }
    pub(super) fn emit_while(&self, cond: &CudaExpr, body: &[CudaStmt], depth: usize) -> String {
        let ind = self.indent(depth);
        let mut out = format!("{}while ({}) {{\n", ind, cond.emit());
        for s in body {
            out.push_str(&self.emit_stmt(s, depth + 1));
            out.push('\n');
        }
        out.push_str(&format!("{}}}", ind));
        out
    }
    pub(super) fn emit_kernel_launch(
        &self,
        name: &str,
        config: &LaunchConfig,
        args: &[CudaExpr],
        depth: usize,
    ) -> String {
        let ind = self.indent(depth);
        let grid = config.grid.emit();
        let block = config.block.emit();
        let shmem = config.shared_mem.emit();
        let stream = config
            .stream
            .as_ref()
            .map(|s| s.emit())
            .unwrap_or_else(|| "0".to_string());
        let arg_strs: Vec<String> = args.iter().map(|a| a.emit()).collect();
        format!(
            "{}{}<<<{}, {}, {}, {}>>>({});",
            ind,
            name,
            grid,
            block,
            shmem,
            stream,
            arg_strs.join(", ")
        )
    }
    pub(super) fn emit_device_function(&self, f: &DeviceFunction) -> String {
        let quals: Vec<String> = f.qualifiers.iter().map(|q| format!("{}", q)).collect();
        let inline_str = if f.is_inline { "inline " } else { "" };
        let qual_str = quals.join(" ");
        let params: Vec<String> = f.params.iter().map(|p| p.emit()).collect();
        let mut out = format!(
            "{}{} {} {}({}) {{\n",
            inline_str,
            qual_str,
            f.ret,
            f.name,
            params.join(", ")
        );
        for s in &f.body {
            out.push_str(&self.emit_stmt(s, 1));
            out.push('\n');
        }
        out.push('}');
        out
    }
    pub(super) fn emit_kernel(&self, k: &CudaKernel) -> String {
        let lb = k
            .launch_bounds
            .as_ref()
            .map(|lb| format!("{} ", lb.emit()))
            .unwrap_or_default();
        let params: Vec<String> = k.params.iter().map(|p| p.emit()).collect();
        let mut out = format!(
            "__global__ {}void {}({}) {{\n",
            lb,
            k.name,
            params.join(", ")
        );
        for smd in &k.shared_mem_decls {
            out.push_str(&format!("    {}\n", smd.emit()));
        }
        for s in &k.body {
            out.push_str(&self.emit_stmt(s, 1));
            out.push('\n');
        }
        out.push('}');
        out
    }
    /// Emit the full `.cu` file as a `String`.
    pub fn emit_module(&self, module: &CudaModule) -> String {
        let mut out = String::new();
        for inc in &module.includes {
            let is_std = !inc.contains('/') && !inc.ends_with(".cuh");
            if is_std {
                out.push_str(&format!("#include <{}>\n", inc));
            } else {
                out.push_str(&format!("#include \"{}\"\n", inc));
            }
        }
        if !module.includes.is_empty() {
            out.push('\n');
        }
        out.push_str(
            "#define CUDA_CHECK(err) \\\n\
             do { \\\n\
             \tcudaError_t _err = (err); \\\n\
             \tif (_err != cudaSuccess) { \\\n\
             \t\tfprintf(stderr, \"CUDA error %s:%d: %s\\n\", \\\n\
             \t\t\t__FILE__, __LINE__, cudaGetErrorString(_err)); \\\n\
             \t\texit(EXIT_FAILURE); \\\n\
             \t} \\\n\
             } while(0)\n\n",
        );
        for (ty, name, init) in &module.constant_decls {
            match init {
                Some(expr) => out.push_str(&format!(
                    "__constant__ {} {} = {};\n",
                    ty,
                    name,
                    expr.emit()
                )),
                None => out.push_str(&format!("__constant__ {} {};\n", ty, name)),
            }
        }
        if !module.constant_decls.is_empty() {
            out.push('\n');
        }
        for f in &module.device_functions {
            out.push_str(&self.emit_device_function(f));
            out.push_str("\n\n");
        }
        for k in &module.kernels {
            out.push_str(&self.emit_kernel(k));
            out.push_str("\n\n");
        }
        for block in &module.host_code {
            out.push_str(block);
            out.push_str("\n\n");
        }
        out
    }
}
/// A `__shared__` memory declaration inside a kernel.
#[derive(Debug, Clone, PartialEq)]
pub struct SharedMemDecl {
    /// Element type
    pub ty: CudaType,
    /// Variable name
    pub name: String,
    /// Array size (None for dynamic shared memory)
    pub size: Option<CudaExpr>,
}
impl SharedMemDecl {
    pub(super) fn emit(&self) -> String {
        match &self.size {
            Some(sz) => format!("__shared__ {} {}[{}];", self.ty, self.name, sz.emit()),
            None => format!("extern __shared__ {} {}[];", self.ty, self.name),
        }
    }
}
/// Unary prefix operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CudaUnOp {
    Neg,
    Not,
    BitNot,
    Deref,
    AddrOf,
}
/// CUDA C++ expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum CudaExpr {
    /// Integer literal: `42`
    LitInt(i64),
    /// Float literal: `3.14f`
    LitFloat(f64),
    /// Boolean literal: `true` / `false`
    LitBool(bool),
    /// Named variable or parameter: `x`
    Var(String),
    /// `threadIdx.x`, `threadIdx.y`, `threadIdx.z`
    ThreadIdx(char),
    /// `blockIdx.x`, `blockIdx.y`, `blockIdx.z`
    BlockIdx(char),
    /// `blockDim.x`, `blockDim.y`, `blockDim.z`
    BlockDim(char),
    /// `gridDim.x`, `gridDim.y`, `gridDim.z`
    GridDim(char),
    /// `__syncthreads()`
    SyncThreads,
    /// `atomicAdd(addr, val)` — atomic addition
    AtomicAdd(Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicSub(addr, val)`
    AtomicSub(Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicExch(addr, val)`
    AtomicExch(Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicCAS(addr, compare, val)`
    AtomicCas(Box<CudaExpr>, Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicMax(addr, val)`
    AtomicMax(Box<CudaExpr>, Box<CudaExpr>),
    /// `atomicMin(addr, val)`
    AtomicMin(Box<CudaExpr>, Box<CudaExpr>),
    /// Binary operation: `a + b`
    BinOp(Box<CudaExpr>, CudaBinOp, Box<CudaExpr>),
    /// Unary operation: `!a`
    UnOp(CudaUnOp, Box<CudaExpr>),
    /// Array subscript: `arr[idx]`
    Index(Box<CudaExpr>, Box<CudaExpr>),
    /// Struct member access: `s.field`
    Member(Box<CudaExpr>, String),
    /// Pointer member access: `p->field`
    PtrMember(Box<CudaExpr>, String),
    /// C-style cast: `(T)expr`
    Cast(CudaType, Box<CudaExpr>),
    /// Function call: `func(args...)`
    Call(String, Vec<CudaExpr>),
    /// Ternary conditional: `cond ? then : else`
    Ternary(Box<CudaExpr>, Box<CudaExpr>, Box<CudaExpr>),
    /// `__ldg(&x)` — read-only cache load
    Ldg(Box<CudaExpr>),
    /// `__shfl_down_sync(mask, var, delta)`
    ShflDownSync(Box<CudaExpr>, Box<CudaExpr>, Box<CudaExpr>),
    /// `__shfl_xor_sync(mask, var, laneMask)`
    ShflXorSync(Box<CudaExpr>, Box<CudaExpr>, Box<CudaExpr>),
    /// `warpSize` builtin
    WarpSize,
    /// `__ballot_sync(mask, predicate)`
    BallotSync(Box<CudaExpr>, Box<CudaExpr>),
    /// `__popc(x)` — popcount
    Popc(Box<CudaExpr>),
}
impl CudaExpr {
    pub(super) fn emit(&self) -> String {
        match self {
            CudaExpr::LitInt(n) => n.to_string(),
            CudaExpr::LitFloat(f) => {
                let s = format!("{:.6}", f);
                format!("{}f", s)
            }
            CudaExpr::LitBool(b) => if *b { "true" } else { "false" }.to_string(),
            CudaExpr::Var(name) => name.clone(),
            CudaExpr::ThreadIdx(c) => format!("threadIdx.{}", c),
            CudaExpr::BlockIdx(c) => format!("blockIdx.{}", c),
            CudaExpr::BlockDim(c) => format!("blockDim.{}", c),
            CudaExpr::GridDim(c) => format!("gridDim.{}", c),
            CudaExpr::SyncThreads => "__syncthreads()".to_string(),
            CudaExpr::AtomicAdd(addr, val) => {
                format!("atomicAdd({}, {})", addr.emit(), val.emit())
            }
            CudaExpr::AtomicSub(addr, val) => {
                format!("atomicSub({}, {})", addr.emit(), val.emit())
            }
            CudaExpr::AtomicExch(addr, val) => {
                format!("atomicExch({}, {})", addr.emit(), val.emit())
            }
            CudaExpr::AtomicCas(addr, cmp, val) => {
                format!("atomicCAS({}, {}, {})", addr.emit(), cmp.emit(), val.emit())
            }
            CudaExpr::AtomicMax(addr, val) => {
                format!("atomicMax({}, {})", addr.emit(), val.emit())
            }
            CudaExpr::AtomicMin(addr, val) => {
                format!("atomicMin({}, {})", addr.emit(), val.emit())
            }
            CudaExpr::BinOp(lhs, op, rhs) => {
                format!("({} {} {})", lhs.emit(), op, rhs.emit())
            }
            CudaExpr::UnOp(op, expr) => format!("({}{})", op, expr.emit()),
            CudaExpr::Index(base, idx) => format!("{}[{}]", base.emit(), idx.emit()),
            CudaExpr::Member(base, field) => format!("{}.{}", base.emit(), field),
            CudaExpr::PtrMember(base, field) => format!("{}->{}", base.emit(), field),
            CudaExpr::Cast(ty, expr) => format!("(({}){})", ty, expr.emit()),
            CudaExpr::Call(name, args) => {
                let arg_strs: Vec<String> = args.iter().map(|a| a.emit()).collect();
                format!("{}({})", name, arg_strs.join(", "))
            }
            CudaExpr::Ternary(cond, then, els) => {
                format!("({} ? {} : {})", cond.emit(), then.emit(), els.emit())
            }
            CudaExpr::Ldg(addr) => format!("__ldg({})", addr.emit()),
            CudaExpr::ShflDownSync(mask, var, delta) => {
                format!(
                    "__shfl_down_sync({}, {}, {})",
                    mask.emit(),
                    var.emit(),
                    delta.emit()
                )
            }
            CudaExpr::ShflXorSync(mask, var, lane_mask) => {
                format!(
                    "__shfl_xor_sync({}, {}, {})",
                    mask.emit(),
                    var.emit(),
                    lane_mask.emit()
                )
            }
            CudaExpr::WarpSize => "warpSize".to_string(),
            CudaExpr::BallotSync(mask, pred) => {
                format!("__ballot_sync({}, {})", mask.emit(), pred.emit())
            }
            CudaExpr::Popc(x) => format!("__popc({})", x.emit()),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDALivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl CUDALivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        CUDALivenessInfo {
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
/// CUDA statement AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum CudaStmt {
    /// Variable declaration with optional initializer:
    /// `CudaType name [ = init ];`
    VarDecl {
        ty: CudaType,
        name: String,
        init: Option<CudaExpr>,
    },
    /// Simple assignment: `lhs = rhs;`
    Assign { lhs: CudaExpr, rhs: CudaExpr },
    /// Compound assignment: `lhs += rhs;` etc.
    CompoundAssign {
        lhs: CudaExpr,
        op: CudaBinOp,
        rhs: CudaExpr,
    },
    /// If / optional else:
    IfElse {
        cond: CudaExpr,
        then_body: Vec<CudaStmt>,
        else_body: Option<Vec<CudaStmt>>,
    },
    /// C-style for loop:
    /// `for (init; cond; step) { body }`
    ForLoop {
        init: Box<CudaStmt>,
        cond: CudaExpr,
        step: CudaExpr,
        body: Vec<CudaStmt>,
    },
    /// While loop: `while (cond) { body }`
    WhileLoop { cond: CudaExpr, body: Vec<CudaStmt> },
    /// CUDA kernel launch: `name<<<grid, block, shmem, stream>>>(args...);`
    KernelLaunch {
        name: String,
        config: LaunchConfig,
        args: Vec<CudaExpr>,
    },
    /// `cudaMalloc((void**)&ptr, size);`
    CudaMalloc { ptr: String, size: CudaExpr },
    /// `cudaMemcpy(dst, src, size, kind);`
    CudaMemcpy {
        dst: CudaExpr,
        src: CudaExpr,
        size: CudaExpr,
        kind: MemcpyKind,
    },
    /// `cudaFree(ptr);`
    CudaFree(CudaExpr),
    /// `return expr;`
    Return(Option<CudaExpr>),
    /// Raw expression statement: `expr;`
    Expr(CudaExpr),
    /// `cudaDeviceSynchronize();`
    DeviceSync,
    /// `cudaCheckError()` macro invocation
    CheckError(CudaExpr),
    /// Block of statements grouped with `{}`
    Block(Vec<CudaStmt>),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
}
/// A parameter in a CUDA kernel or device function.
#[derive(Debug, Clone, PartialEq)]
pub struct CudaParam {
    /// CUDA type
    pub ty: CudaType,
    /// Parameter name
    pub name: String,
    /// Whether the parameter is `const`
    pub is_const: bool,
    /// Optional qualifier such as `__restrict__`
    pub qualifier: Option<CudaQualifier>,
}
impl CudaParam {
    /// Create a plain parameter.
    pub fn new(ty: CudaType, name: impl Into<String>) -> Self {
        CudaParam {
            ty,
            name: name.into(),
            is_const: false,
            qualifier: None,
        }
    }
    /// Mark parameter as `const`.
    pub fn with_const(mut self) -> Self {
        self.is_const = true;
        self
    }
    /// Add a CUDA qualifier (e.g. `__restrict__`).
    pub fn with_qualifier(mut self, q: CudaQualifier) -> Self {
        self.qualifier = Some(q);
        self
    }
    pub(super) fn emit(&self) -> String {
        let mut parts = Vec::new();
        if self.is_const {
            parts.push("const".to_string());
        }
        if let Some(q) = &self.qualifier {
            parts.push(format!("{}", q));
        }
        parts.push(format!("{}", self.ty));
        parts.push(self.name.clone());
        parts.join(" ")
    }
}
/// A `__device__` (or `__host__ __device__`) helper function.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceFunction {
    /// Function name
    pub name: String,
    /// Qualifiers (should include at least `Device`)
    pub qualifiers: Vec<CudaQualifier>,
    /// Return type
    pub ret: CudaType,
    /// Parameter list
    pub params: Vec<CudaParam>,
    /// Body statements
    pub body: Vec<CudaStmt>,
    /// Whether the function is `inline`
    pub is_inline: bool,
}
impl DeviceFunction {
    /// Create a plain `__device__` function.
    pub fn new(name: impl Into<String>, ret: CudaType) -> Self {
        DeviceFunction {
            name: name.into(),
            qualifiers: vec![CudaQualifier::Device],
            ret,
            params: Vec::new(),
            body: Vec::new(),
            is_inline: false,
        }
    }
    /// Create a `__host__ __device__` function.
    pub fn host_device(name: impl Into<String>, ret: CudaType) -> Self {
        DeviceFunction {
            name: name.into(),
            qualifiers: vec![CudaQualifier::Host, CudaQualifier::Device],
            ret,
            params: Vec::new(),
            body: Vec::new(),
            is_inline: false,
        }
    }
    /// Mark as `inline`.
    pub fn with_inline(mut self) -> Self {
        self.is_inline = true;
        self
    }
    /// Append a parameter.
    pub fn add_param(mut self, p: CudaParam) -> Self {
        self.params.push(p);
        self
    }
    /// Append a body statement.
    pub fn add_stmt(mut self, s: CudaStmt) -> Self {
        self.body.push(s);
        self
    }
}
/// CUDA / C++ type representation used in generated `.cu` files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CudaType {
    /// `int`
    Int,
    /// `long`
    Long,
    /// `float`
    Float,
    /// `double`
    Double,
    /// `__half` (CUDA half-precision float)
    Half,
    /// `bool`
    Bool,
    /// `dim3` (three-component grid/block dimension)
    Dim3,
    /// `size_t`
    DimT,
    /// `cudaError_t`
    CudaErrorT,
    /// Pointer to inner type: `T*`
    Pointer(Box<CudaType>),
    /// `__shared__` qualified type (used internally for shared-mem decls)
    Shared(Box<CudaType>),
    /// `__constant__` qualified type
    Constant(Box<CudaType>),
    /// `__device__` qualified type
    Device(Box<CudaType>),
    /// Void: `void`
    Void,
    /// Unsigned int: `unsigned int`
    UInt,
    /// Named struct or typedef
    Named(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDACacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Constant folding helper for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CUDAExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl CUDAExtConstFolder {
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
/// Pass execution phase for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CUDAExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl CUDAExtPassPhase {
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
/// Kind of `cudaMemcpy` transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemcpyKind {
    /// `cudaMemcpyHostToDevice`
    HostToDevice,
    /// `cudaMemcpyDeviceToHost`
    DeviceToHost,
    /// `cudaMemcpyDeviceToDevice`
    DeviceToDevice,
    /// `cudaMemcpyHostToHost`
    HostToHost,
}
/// Analysis cache for CUDAExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct CUDAExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl CUDAExtCache {
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
/// Liveness analysis for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CUDAExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl CUDAExtLiveness {
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
#[allow(dead_code)]
pub struct CUDAPassRegistry {
    pub(super) configs: Vec<CUDAPassConfig>,
    pub(super) stats: std::collections::HashMap<String, CUDAPassStats>,
}
impl CUDAPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CUDAPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: CUDAPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), CUDAPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&CUDAPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&CUDAPassStats> {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, CUDACacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl CUDAAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        CUDAAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&CUDACacheEntry> {
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
            CUDACacheEntry {
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
/// Dependency graph for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl CUDAExtDepGraph {
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
/// Statistics for CUDAExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CUDAExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl CUDAExtPassStats {
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
    pub fn merge(&mut self, o: &CUDAExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CUDAPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl CUDAPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            CUDAPassPhase::Analysis => "analysis",
            CUDAPassPhase::Transformation => "transformation",
            CUDAPassPhase::Verification => "verification",
            CUDAPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, CUDAPassPhase::Transformation | CUDAPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAPassConfig {
    pub phase: CUDAPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl CUDAPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: CUDAPassPhase) -> Self {
        CUDAPassConfig {
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
#[derive(Debug, Clone)]
pub struct CUDADepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl CUDADepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CUDADepGraph {
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
pub struct CUDAConstantFoldingHelper;
impl CUDAConstantFoldingHelper {
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
/// Configuration for CUDAExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAExtPassConfig {
    pub name: String,
    pub phase: CUDAExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl CUDAExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: CUDAExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: CUDAExtPassPhase) -> Self {
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
/// CUDA function / variable qualifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CudaQualifier {
    /// `__global__` — kernel callable from host, runs on device
    Global,
    /// `__device__` — callable/usable only on device
    Device,
    /// `__host__` — callable only from host (default)
    Host,
    /// `__shared__` — shared memory within a thread block
    Shared,
    /// `__constant__` — read-only constant memory
    Constant,
    /// `__managed__` — accessible from both host and device
    Managed,
    /// `__restrict__` — pointer alias hint
    Restrict,
    /// `volatile` — volatile memory access
    Volatile,
}
/// Optional launch-bounds hint: `__launch_bounds__(maxThreads[, minBlocks])`.
#[derive(Debug, Clone, PartialEq)]
pub struct LaunchBounds {
    /// Maximum threads per block
    pub max_threads: u32,
    /// Minimum blocks per multiprocessor (optional)
    pub min_blocks: Option<u32>,
}
impl LaunchBounds {
    /// Create launch bounds with only a max-thread count.
    pub fn new(max_threads: u32) -> Self {
        LaunchBounds {
            max_threads,
            min_blocks: None,
        }
    }
    /// Create launch bounds with both max-threads and min-blocks.
    pub fn with_min_blocks(max_threads: u32, min_blocks: u32) -> Self {
        LaunchBounds {
            max_threads,
            min_blocks: Some(min_blocks),
        }
    }
    pub(super) fn emit(&self) -> String {
        match self.min_blocks {
            Some(mb) => format!("__launch_bounds__({}, {})", self.max_threads, mb),
            None => format!("__launch_bounds__({})", self.max_threads),
        }
    }
}
/// Dominator tree for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDAExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl CUDAExtDomTree {
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
/// Binary operators available in CUDA C++ expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CudaBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CUDADominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl CUDADominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        CUDADominatorTree {
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
/// Pass registry for CUDAExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CUDAExtPassRegistry {
    pub(super) configs: Vec<CUDAExtPassConfig>,
    pub(super) stats: Vec<CUDAExtPassStats>,
}
impl CUDAExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: CUDAExtPassConfig) {
        self.stats.push(CUDAExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&CUDAExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&CUDAExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&CUDAExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &CUDAExtPassPhase) -> Vec<&CUDAExtPassConfig> {
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
