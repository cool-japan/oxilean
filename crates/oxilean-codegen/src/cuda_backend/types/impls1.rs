use std::collections::{HashMap, HashSet, VecDeque};

use super::defs::*;

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

impl CudaBackend {
    /// Create a new backend with 4-space indentation.
    pub fn new() -> Self {
        CudaBackend { indent_width: 4 }
    }
    /// Create a backend with a custom indent width.
    pub fn with_indent(indent_width: usize) -> Self {
        CudaBackend { indent_width }
    }
    pub(crate) fn indent(&self, depth: usize) -> String {
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
    pub(crate) fn emit_if_else(
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
    pub(crate) fn emit_for_loop(
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
    pub(crate) fn emit_while(&self, cond: &CudaExpr, body: &[CudaStmt], depth: usize) -> String {
        let ind = self.indent(depth);
        let mut out = format!("{}while ({}) {{\n", ind, cond.emit());
        for s in body {
            out.push_str(&self.emit_stmt(s, depth + 1));
            out.push('\n');
        }
        out.push_str(&format!("{}}}", ind));
        out
    }
    pub(crate) fn emit_kernel_launch(
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
    pub(crate) fn emit_device_function(&self, f: &DeviceFunction) -> String {
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
    pub(crate) fn emit_kernel(&self, k: &CudaKernel) -> String {
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

impl SharedMemDecl {
    pub(crate) fn emit(&self) -> String {
        match &self.size {
            Some(sz) => format!("__shared__ {} {}[{}];", self.ty, self.name, sz.emit()),
            None => format!("extern __shared__ {} {}[];", self.ty, self.name),
        }
    }
}

impl CudaExpr {
    pub(crate) fn emit(&self) -> String {
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
    pub(crate) fn emit(&self) -> String {
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
