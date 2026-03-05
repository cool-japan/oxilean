//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Standard interface templates (SRAM, APB, AHB, AXI4-Lite stubs).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ChiselInterfaceTemplate {
    SramPort { addr_bits: u32, data_bits: u32 },
    ApbPort { addr_bits: u32, data_bits: u32 },
    AhbLitePort { addr_bits: u32, data_bits: u32 },
    Axi4LitePort { addr_bits: u32, data_bits: u32 },
}
#[allow(dead_code)]
impl ChiselInterfaceTemplate {
    /// Emit the IO bundle fields for this interface.
    pub fn emit_ports(&self, prefix: &str, is_master: bool) -> String {
        match self {
            ChiselInterfaceTemplate::SramPort {
                addr_bits,
                data_bits,
            } => {
                let a_dir = if is_master { "Output" } else { "Input" };
                let d_out_dir = if is_master { "Output" } else { "Input" };
                let d_in_dir = if is_master { "Input" } else { "Output" };
                format!(
                    "    val {p}_addr  = {a}(UInt({ab}.W))\n\
                     val {p}_wen   = {a}(Bool())\n\
                     val {p}_wdata = {do}(UInt({db}.W))\n\
                     val {p}_rdata = {di}(UInt({db}.W))\n\
                     val {p}_cs    = {a}(Bool())\n",
                    p = prefix, a = a_dir, do = d_out_dir, di = d_in_dir, ab = addr_bits,
                    db = data_bits
                )
            }
            ChiselInterfaceTemplate::ApbPort {
                addr_bits,
                data_bits,
            } => {
                let m_dir = if is_master { "Output" } else { "Input" };
                let s_dir = if is_master { "Input" } else { "Output" };
                format!(
                    "    val {p}_paddr  = {m}(UInt({ab}.W))\n\
                     val {p}_psel   = {m}(Bool())\n\
                     val {p}_penable= {m}(Bool())\n\
                     val {p}_pwrite = {m}(Bool())\n\
                     val {p}_pwdata = {m}(UInt({db}.W))\n\
                     val {p}_prdata = {s}(UInt({db}.W))\n\
                     val {p}_pready = {s}(Bool())\n\
                     val {p}_pslverr= {s}(Bool())\n",
                    p = prefix,
                    m = m_dir,
                    s = s_dir,
                    ab = addr_bits,
                    db = data_bits
                )
            }
            ChiselInterfaceTemplate::AhbLitePort {
                addr_bits,
                data_bits,
            } => {
                format!(
                    "    /* AHB-Lite {p} {ab}b addr {db}b data */\n\
                     val {p}_haddr  = Output(UInt({ab}.W))\n\
                     val {p}_htrans = Output(UInt(2.W))\n\
                     val {p}_hwrite = Output(Bool())\n\
                     val {p}_hwdata = Output(UInt({db}.W))\n\
                     val {p}_hrdata = Input(UInt({db}.W))\n\
                     val {p}_hready = Input(Bool())\n\
                     val {p}_hresp  = Input(Bool())\n",
                    p = prefix,
                    ab = addr_bits,
                    db = data_bits
                )
            }
            ChiselInterfaceTemplate::Axi4LitePort {
                addr_bits,
                data_bits,
            } => {
                format!(
                    "    /* AXI4-Lite {p} {ab}b addr {db}b data */\n\
                     val {p}_awvalid = Output(Bool())\n\
                     val {p}_awready = Input(Bool())\n\
                     val {p}_awaddr  = Output(UInt({ab}.W))\n\
                     val {p}_wvalid  = Output(Bool())\n\
                     val {p}_wready  = Input(Bool())\n\
                     val {p}_wdata   = Output(UInt({db}.W))\n\
                     val {p}_wstrb   = Output(UInt({ws}.W))\n\
                     val {p}_bvalid  = Input(Bool())\n\
                     val {p}_bready  = Output(Bool())\n\
                     val {p}_bresp   = Input(UInt(2.W))\n\
                     val {p}_arvalid = Output(Bool())\n\
                     val {p}_arready = Input(Bool())\n\
                     val {p}_araddr  = Output(UInt({ab}.W))\n\
                     val {p}_rvalid  = Input(Bool())\n\
                     val {p}_rready  = Output(Bool())\n\
                     val {p}_rdata   = Input(UInt({db}.W))\n\
                     val {p}_rresp   = Input(UInt(2.W))\n",
                    p = prefix,
                    ab = addr_bits,
                    db = data_bits,
                    ws = data_bits / 8
                )
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl ChiselDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ChiselDepGraph {
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
/// SRAM wrapper descriptor.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselSRAMWrapper {
    pub name: String,
    pub depth: u32,
    pub data_width: u32,
    pub port_type: SramPortType,
    pub has_mask: bool,
    pub mask_granularity: u32,
    pub use_sync_read: bool,
    pub pipeline_read: bool,
}
#[allow(dead_code)]
impl ChiselSRAMWrapper {
    pub fn single_port(name: impl Into<String>, depth: u32, data_width: u32) -> Self {
        Self {
            name: name.into(),
            depth,
            data_width,
            port_type: SramPortType::SinglePort,
            has_mask: false,
            mask_granularity: 8,
            use_sync_read: true,
            pipeline_read: false,
        }
    }
    pub fn simple_dual_port(name: impl Into<String>, depth: u32, data_width: u32) -> Self {
        let mut s = Self::single_port(name, depth, data_width);
        s.port_type = SramPortType::SimpleDualPort;
        s
    }
    pub fn true_dual_port(name: impl Into<String>, depth: u32, data_width: u32) -> Self {
        let mut s = Self::single_port(name, depth, data_width);
        s.port_type = SramPortType::TrueDualPort;
        s
    }
    pub fn with_mask(mut self, granularity: u32) -> Self {
        self.has_mask = true;
        self.mask_granularity = granularity;
        self
    }
    pub fn with_pipeline_read(mut self) -> Self {
        self.pipeline_read = true;
        self
    }
    pub fn addr_width(&self) -> u32 {
        if self.depth == 0 {
            return 1;
        }
        (self.depth as f64).log2().ceil() as u32
    }
    pub fn mask_width(&self) -> u32 {
        if !self.has_mask {
            return 0;
        }
        self.data_width / self.mask_granularity
    }
    /// Emit the Chisel SRAMInterface module class.
    pub fn emit(&self) -> String {
        let aw = self.addr_width();
        let dw = self.data_width;
        let mw = self.mask_width();
        let mut out = format!("class {} extends Module {{\n", self.name);
        out.push_str("  val io = IO(new Bundle {\n");
        out.push_str(&format!("    val waddr = Input(UInt({}.W))\n", aw));
        out.push_str(&format!("    val wdata = Input(UInt({}.W))\n", dw));
        out.push_str("    val wen   = Input(Bool())\n");
        if self.has_mask {
            out.push_str(&format!("    val wmask = Input(UInt({}.W))\n", mw));
        }
        match self.port_type {
            SramPortType::SimpleDualPort | SramPortType::TrueDualPort => {
                out.push_str(&format!("    val raddr = Input(UInt({}.W))\n", aw));
                out.push_str("    val ren   = Input(Bool())\n");
                out.push_str(&format!("    val rdata = Output(UInt({}.W))\n", dw));
            }
            SramPortType::SinglePort => {
                out.push_str(&format!("    val raddr = Input(UInt({}.W))\n", aw));
                out.push_str(&format!("    val rdata = Output(UInt({}.W))\n", dw));
            }
        }
        out.push_str("  })\n\n");
        out.push_str(&format!(
            "  val mem = SyncReadMem({}, UInt({}.W))\n\n",
            self.depth, dw
        ));
        out.push_str("  when (io.wen) {\n");
        if self.has_mask {
            out.push_str("    mem.write(io.waddr, io.wdata, io.wmask.asBools)\n");
        } else {
            out.push_str("    mem.write(io.waddr, io.wdata)\n");
        }
        out.push_str("  }\n\n");
        if self.pipeline_read {
            out.push_str("  val raddr_r = RegNext(io.raddr)\n");
            out.push_str("  io.rdata := mem.read(raddr_r)\n");
        } else {
            out.push_str("  io.rdata := mem.read(io.raddr)\n");
        }
        out.push_str("}\n");
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, ChiselCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl ChiselAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        ChiselAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&ChiselCacheEntry> {
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
            ChiselCacheEntry {
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
pub struct ChiselPassRegistry {
    pub(super) configs: Vec<ChiselPassConfig>,
    pub(super) stats: std::collections::HashMap<String, ChiselPassStats>,
}
impl ChiselPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ChiselPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: ChiselPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), ChiselPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ChiselPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&ChiselPassStats> {
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
/// FIRRTL / Chisel annotation kinds.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ChiselAnnotationKind {
    DontTouch,
    ForceNameAnnotation,
    SynthBlackBox,
    InlineInstance,
    NoDedupAnnotation,
    LoadMemoryAnnotation { file: String },
}
#[allow(dead_code)]
impl ChiselAnnotationKind {
    pub fn scala_annotation(&self, target: &str) -> String {
        match self {
            ChiselAnnotationKind::DontTouch => {
                format!(
                    "annotate(new ChiselAnnotation {{ def toFirrtl = DontTouchAnnotation({})}}\n)",
                    target
                )
            }
            ChiselAnnotationKind::ForceNameAnnotation => {
                format!(
                    "annotate(new ChiselAnnotation {{ def toFirrtl = ForcedName(\"{}\", {})}}\n)",
                    target, target
                )
            }
            ChiselAnnotationKind::SynthBlackBox => {
                format!("// synthesis black box: {}\n", target)
            }
            ChiselAnnotationKind::InlineInstance => {
                format!(
                    "annotate(new ChiselAnnotation {{ def toFirrtl = InlineAnnotation({})}}\n)",
                    target
                )
            }
            ChiselAnnotationKind::NoDedupAnnotation => {
                format!(
                    "annotate(new ChiselAnnotation {{ def toFirrtl = NoDedupAnnotation({})}}\n)",
                    target
                )
            }
            ChiselAnnotationKind::LoadMemoryAnnotation { file } => {
                format!("loadMemoryFromFile({}, {:?})\n", target, file)
            }
        }
    }
}
/// Chisel hardware data types.
#[derive(Debug, Clone, PartialEq)]
pub enum ChiselType {
    /// `UInt(width.W)` — unsigned integer of `width` bits
    UInt(u32),
    /// `SInt(width.W)` — signed integer of `width` bits
    SInt(u32),
    /// `Bool()` — single-bit boolean
    Bool,
    /// `new Bundle { ... }` — named-field aggregate
    Bundle(Vec<(String, Box<ChiselType>)>),
    /// `Vec(count, gen)` — hardware vector
    Vec(u32, Box<ChiselType>),
    /// `Clock()` — clock signal
    Clock,
    /// `Reset()` — reset signal
    Reset,
    /// `AsyncReset()` — asynchronous reset
    AsyncReset,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ChiselPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl ChiselPassStats {
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
/// A Chisel hardware expression.
#[derive(Debug, Clone, PartialEq)]
pub enum ChiselExpr {
    /// Integer literal: `value.U(width.W)`
    ULit(u64, u32),
    /// Signed literal: `value.S(width.W)`
    SLit(i64, u32),
    /// `true.B` or `false.B`
    BoolLit(bool),
    /// Signal reference
    Var(String),
    /// `io.name`
    Io(String),
    /// `reg.name` (Register field)
    RegField(String),
    /// Binary operation: `lhs op rhs`
    BinOp(Box<ChiselExpr>, String, Box<ChiselExpr>),
    /// Unary operation: `op(operand)`
    UnOp(String, Box<ChiselExpr>),
    /// Mux: `Mux(sel, t, f)`
    Mux(Box<ChiselExpr>, Box<ChiselExpr>, Box<ChiselExpr>),
    /// Bit extraction: `expr(hi, lo)`
    BitSlice(Box<ChiselExpr>, u32, u32),
    /// Cat: `Cat(a, b, ...)`
    Cat(Vec<ChiselExpr>),
    /// Method call: `receiver.method(args...)`
    MethodCall(Box<ChiselExpr>, String, Vec<ChiselExpr>),
}
/// SRAM port type.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SramPortType {
    SinglePort,
    SimpleDualPort,
    TrueDualPort,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ChiselPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl ChiselPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ChiselPassPhase::Analysis => "analysis",
            ChiselPassPhase::Transformation => "transformation",
            ChiselPassPhase::Verification => "verification",
            ChiselPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            ChiselPassPhase::Transformation | ChiselPassPhase::Cleanup
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl ChiselLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        ChiselLivenessInfo {
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
/// Code-generation backend for Chisel 3 / Chisel 5.
#[derive(Debug, Clone, Default)]
pub struct ChiselBackend;
impl ChiselBackend {
    /// Create a new `ChiselBackend`.
    pub fn new() -> Self {
        ChiselBackend
    }
    /// Emit the Chisel type string for a `ChiselType`.
    pub fn emit_type(&self, ty: &ChiselType) -> String {
        ty.to_string()
    }
    /// Emit the `val io = IO(new Bundle { ... })` declaration for a port list.
    pub fn io_bundle(&self, ports: &[ChiselPort]) -> String {
        let mut s = String::from("val io = IO(new Bundle {\n");
        for port in ports {
            s.push_str(&format!(
                "    val {name} = {dir}({ty})\n",
                name = port.name,
                dir = port.direction(),
                ty = self.emit_type(&port.ty),
            ));
        }
        s.push_str("  })");
        s
    }
    /// Emit a complete Chisel module class.
    pub fn emit_module(&self, module: &ChiselModule) -> String {
        let mut out = String::new();
        out.push_str("// Generated by OxiLean ChiselBackend\n");
        out.push_str("import chisel3._\n");
        out.push_str("import chisel3.util._\n\n");
        out.push_str(&format!("class {} extends Module {{\n", module.name));
        if !module.ports.is_empty() {
            out.push_str("  ");
            out.push_str(&self.io_bundle(&module.ports));
            out.push_str("\n\n");
        }
        for stmt in &module.body {
            out.push_str("  ");
            out.push_str(stmt);
            out.push('\n');
        }
        out.push_str("}\n");
        out
    }
    /// Emit a `when` block: `when(cond) { body }`.
    pub fn when_stmt(&self, cond: &str, body: &str) -> String {
        format!("when ({cond}) {{\n    {body}\n  }}")
    }
    /// Emit a `when` / `otherwise` block.
    pub fn when_otherwise(&self, cond: &str, when_body: &str, other_body: &str) -> String {
        format!("when ({cond}) {{\n    {when_body}\n  }} .otherwise {{\n    {other_body}\n  }}")
    }
    /// Emit a register declaration: `val r = RegInit(init_val)`.
    pub fn reg_init(&self, name: &str, ty: &ChiselType, init: &str) -> String {
        format!("val {name} = RegInit({init}.U.asTypeOf({ty}))")
    }
    /// Emit a register declaration without reset: `val r = Reg(gen)`.
    pub fn reg_no_reset(&self, name: &str, ty: &ChiselType) -> String {
        format!("val {name} = Reg({ty})")
    }
    /// Emit a wire declaration: `val w = Wire(gen)`.
    pub fn wire_decl(&self, name: &str, ty: &ChiselType) -> String {
        format!("val {name} = Wire({ty})")
    }
    /// Emit a connection: `lhs := rhs`.
    pub fn connect(&self, lhs: &str, rhs: &str) -> String {
        format!("{lhs} := {rhs}")
    }
    /// Emit a `printf` debug statement.
    pub fn printf(&self, fmt_str: &str, args: &[&str]) -> String {
        if args.is_empty() {
            format!("printf(\"{fmt_str}\\n\")")
        } else {
            let arg_str = args.join(", ");
            format!("printf(\"{fmt_str}\\n\", {arg_str})")
        }
    }
    /// Emit a `assert` statement.
    pub fn assert_stmt(&self, cond: &str, msg: &str) -> String {
        format!("assert({cond}, \"{msg}\")")
    }
    /// Emit a `Mux` expression string.
    pub fn mux_expr(&self, sel: &str, t: &str, f: &str) -> String {
        format!("Mux({sel}, {t}, {f})")
    }
    /// Emit a `Cat` expression string.
    pub fn cat_expr(&self, parts: &[&str]) -> String {
        format!("Cat({})", parts.join(", "))
    }
    /// Emit a `fill` (replicate) expression.
    pub fn fill_expr(&self, count: u32, value: &str) -> String {
        format!("Fill({count}, {value})")
    }
    /// Emit a submodule instantiation.
    pub fn instantiate(&self, class: &str, inst_name: &str) -> String {
        format!("val {inst_name} = Module(new {class}())")
    }
    /// Emit a `ChiselExpr` as a string.
    pub fn emit_expr(&self, expr: &ChiselExpr) -> String {
        expr.to_string()
    }
}
impl ChiselBackend {
    /// Emit a DontCare assignment.
    #[allow(dead_code)]
    pub fn dont_care(&self, signal: &str) -> String {
        format!("{} := DontCare", signal)
    }
    /// Emit an Irrevocable (valid+ready, never de-asserts valid) port.
    #[allow(dead_code)]
    pub fn irrevocable_port(&self, name: &str, data_type: &ChiselType, is_output: bool) -> String {
        let dir = if is_output {
            "Irrevocable"
        } else {
            "Flipped(Irrevocable"
        };
        let close = if is_output { "" } else { ")" };
        format!("val {} = {}({}){}", name, dir, data_type, close)
    }
    /// Emit a combinational ROM using a Vec\[UInt\] initialized from a list.
    #[allow(dead_code)]
    pub fn comb_rom(&self, name: &str, _data_type: &ChiselType, values: &[&str]) -> String {
        let entries: Vec<String> = values.iter().map(|v| format!("{}.U", v)).collect();
        format!("val {} = VecInit(Seq({}))\n", name, entries.join(", "))
    }
    /// Emit a Mux1H (one-hot mux).
    #[allow(dead_code)]
    pub fn mux1h(&self, _sel: &str, cases: &[(&str, &str)]) -> String {
        let entries: Vec<String> = cases
            .iter()
            .map(|(s, v)| format!("{} -> {}", s, v))
            .collect();
        format!("Mux1H(Seq({}))", entries.join(", "))
    }
    /// Emit a log2Ceil computation.
    #[allow(dead_code)]
    pub fn log2_ceil(&self, n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            (n as f64).log2().ceil() as u32
        }
    }
    /// Emit a log2Floor computation.
    #[allow(dead_code)]
    pub fn log2_floor(&self, n: u32) -> u32 {
        if n == 0 {
            0
        } else {
            (n as f64).log2().floor() as u32
        }
    }
    /// Check if n is a power of two.
    #[allow(dead_code)]
    pub fn is_pow2(&self, n: u32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
    /// Emit a cover statement (Chisel formal verification).
    #[allow(dead_code)]
    pub fn cover_stmt(&self, cond: &str, msg: &str) -> String {
        format!("cover({}, \"{}\", \"{}\")", cond, msg, msg)
    }
    /// Emit an assume statement.
    #[allow(dead_code)]
    pub fn assume_stmt(&self, cond: &str) -> String {
        format!("assume({}.B, \"assumption\")", cond)
    }
    /// Emit a ChiselSim-compatible printf for simulation only.
    #[allow(dead_code)]
    pub fn sim_printf(&self, fmt_str: &str, args: &[&str]) -> String {
        if args.is_empty() {
            format!("printf(p\"{}\", cf\"\")", fmt_str)
        } else {
            format!("printf(p\"{}\", {})", fmt_str, args.join(", "))
        }
    }
    /// Emit a chisel3.util.Fill call (replicate a bit pattern).
    #[allow(dead_code)]
    pub fn fill(&self, n: u32, expr: &str) -> String {
        format!("Fill({}, {})", n, expr)
    }
    /// Emit a Cat call (concatenate signals).
    #[allow(dead_code)]
    pub fn cat(&self, signals: &[&str]) -> String {
        format!("Cat({})", signals.join(", "))
    }
    /// Emit a PopCount call.
    #[allow(dead_code)]
    pub fn popcount(&self, signal: &str) -> String {
        format!("PopCount({})", signal)
    }
    /// Emit a priority encoder (OHToUInt).
    #[allow(dead_code)]
    pub fn oh_to_uint(&self, one_hot: &str) -> String {
        format!("OHToUInt({})", one_hot)
    }
    /// Emit a UIntToOH call.
    #[allow(dead_code)]
    pub fn uint_to_oh(&self, n: &str, width: u32) -> String {
        format!("UIntToOH({}, {})", n, width)
    }
    /// Emit a Reverse call (bit-reversal).
    #[allow(dead_code)]
    pub fn reverse(&self, signal: &str) -> String {
        format!("Reverse({})", signal)
    }
    /// Emit a MuxCase expression.
    #[allow(dead_code)]
    pub fn mux_case(&self, default: &str, cases: &[(&str, &str)]) -> String {
        let entries: Vec<String> = cases
            .iter()
            .map(|(c, v)| format!("({}) -> ({})", c, v))
            .collect();
        format!("MuxCase({}, Seq({}))", default, entries.join(", "))
    }
    /// Emit a ShiftRegister instantiation.
    #[allow(dead_code)]
    pub fn shift_register(&self, data: &str, n: u32, reset_val: &str) -> String {
        format!("ShiftRegister({}, {}, {}.U)", data, n, reset_val)
    }
    /// Emit an arbiter (RRArbiter) for n inputs of a given type.
    #[allow(dead_code)]
    pub fn round_robin_arbiter(&self, name: &str, data_type: &ChiselType, n: u32) -> String {
        format!(
            "val {} = Module(new RRArbiter({}, {}))\n",
            name, data_type, n
        )
    }
    /// Emit a fixed-priority arbiter.
    #[allow(dead_code)]
    pub fn priority_arbiter(&self, name: &str, data_type: &ChiselType, n: u32) -> String {
        format!("val {} = Module(new Arbiter({}, {}))\n", name, data_type, n)
    }
    /// Emit a Chisel FIFO queue module instantiation.
    #[allow(dead_code)]
    pub fn queue_module(&self, name: &str, data_type: &ChiselType, depth: u32) -> String {
        format!(
            "val {} = Module(new Queue({}, {}))\n",
            name, data_type, depth
        )
    }
    /// Emit a when/elsewhen/otherwise chain.
    #[allow(dead_code)]
    pub fn when_chain(&self, cond_body: &[(&str, &str)], otherwise: Option<&str>) -> String {
        let mut out = String::new();
        for (i, (cond, body)) in cond_body.iter().enumerate() {
            if i == 0 {
                out.push_str(&format!("when ({}) {{\n  {}\n}}", cond, body));
            } else {
                out.push_str(&format!(".elsewhen ({}) {{\n  {}\n}}", cond, body));
            }
        }
        if let Some(other) = otherwise {
            out.push_str(&format!(".otherwise {{\n  {}\n}}", other));
        }
        out.push('\n');
        out
    }
    /// Emit a counter that wraps at max_val.
    #[allow(dead_code)]
    pub fn counter(&self, name: &str, max_val: u32, enable: &str) -> String {
        let width = self.log2_ceil(max_val + 1);
        format!(
            "val ({}_count, {}_wrap) = Counter({}, {})\n",
            name, name, enable, max_val
        ) + &format!("// {} is {}-bit counter up to {}\n", name, width, max_val)
    }
    /// Emit a reset synchronizer (2-FF synchronizer for reset deassertion).
    #[allow(dead_code)]
    pub fn reset_sync(&self, name: &str, async_rst: &str) -> String {
        format!(
            "val {}_sync = withReset({}.asAsyncReset) {{\n  RegNext(RegNext(true.B, false.B), false.B)\n}}\n",
            name, async_rst
        )
    }
    /// Emit a clock domain crossing (CDC) handshake wrapper comment.
    #[allow(dead_code)]
    pub fn cdc_handshake_comment(&self, from_clk: &str, to_clk: &str) -> String {
        format!(
            "/* CDC: {} -> {} — use async FIFO or req/ack handshake here */\n",
            from_clk, to_clk
        )
    }
    /// Emit a BlackBox module class stub.
    #[allow(dead_code)]
    pub fn blackbox_stub(&self, name: &str, params: &[(&str, &str)]) -> String {
        let param_str: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("\"{}\" -> {}", k, v))
            .collect();
        if params.is_empty() {
            format!(
                "class {} extends BlackBox {{\n  val io = IO(new Bundle {{}})\n}}\n",
                name
            )
        } else {
            format!(
                "class {} extends BlackBox(Map({})) {{\n  val io = IO(new Bundle {{}})\n}}\n",
                name,
                param_str.join(", ")
            )
        }
    }
}
#[allow(dead_code)]
pub struct ChiselConstantFoldingHelper;
impl ChiselConstantFoldingHelper {
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
/// Generates a multi-stage pipeline register chain.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselPipelineRegisterChain {
    pub stages: Vec<PipelineStage>,
    pub reset_val: String,
}
#[allow(dead_code)]
impl ChiselPipelineRegisterChain {
    pub fn new(reset_val: impl Into<String>) -> Self {
        Self {
            stages: Vec::new(),
            reset_val: reset_val.into(),
        }
    }
    pub fn stage(mut self, s: PipelineStage) -> Self {
        self.stages.push(s);
        self
    }
    /// Emit Chisel register declarations for all stages.
    pub fn emit_registers(&self) -> String {
        let mut out = String::new();
        for (i, stage) in self.stages.iter().enumerate() {
            out.push_str(&format!(
                "val {} = RegInit({}.U.asTypeOf({}))\n",
                stage.name, self.reset_val, stage.data_type
            ));
            if stage.has_valid {
                out.push_str(&format!("val {}_valid = RegInit(false.B)\n", stage.name));
            }
            if stage.has_stall {
                out.push_str(&format!(
                    "val {}_stall = WireDefault(false.B)\n",
                    stage.name
                ));
            }
            if i + 1 < self.stages.len() {
                let next = &self.stages[i + 1];
                out.push_str(&format!(
                    "when (!{}_stall) {{ {} := {} }}\n",
                    next.name, next.name, stage.name
                ));
                if stage.has_valid && next.has_valid {
                    out.push_str(&format!(
                        "when (!{}_stall) {{ {}_valid := {}_valid }}\n",
                        next.name, next.name, stage.name
                    ));
                }
            }
        }
        out
    }
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl ChiselWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ChiselWorklist {
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
pub struct ChiselDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl ChiselDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        ChiselDominatorTree {
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
/// A single I/O port in a Chisel module's IO bundle.
#[derive(Debug, Clone, PartialEq)]
pub struct ChiselPort {
    /// Port name
    pub name: String,
    /// Chisel hardware type
    pub ty: ChiselType,
    /// `true` → `Output(...)`, `false` → `Input(...)`
    pub is_output: bool,
}
impl ChiselPort {
    /// Create an input port.
    pub fn input(name: impl Into<String>, ty: ChiselType) -> Self {
        ChiselPort {
            name: name.into(),
            ty,
            is_output: false,
        }
    }
    /// Create an output port.
    pub fn output(name: impl Into<String>, ty: ChiselType) -> Self {
        ChiselPort {
            name: name.into(),
            ty,
            is_output: true,
        }
    }
    /// Direction keyword: `"Input"` or `"Output"`.
    pub fn direction(&self) -> &'static str {
        if self.is_output {
            "Output"
        } else {
            "Input"
        }
    }
}
/// AXI-Stream endpoint direction.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum StreamDirection {
    Producer,
    Consumer,
}
/// A module wrapper with AXI-Stream interface(s).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselStreamingModule {
    pub name: String,
    pub data_width: u32,
    pub direction: StreamDirection,
    pub has_tlast: bool,
    pub has_tkeep: bool,
    pub has_tid: bool,
    pub id_width: u32,
    pub has_tuser: bool,
    pub user_width: u32,
    pub body: Vec<String>,
}
#[allow(dead_code)]
impl ChiselStreamingModule {
    pub fn producer(name: impl Into<String>, data_width: u32) -> Self {
        Self {
            name: name.into(),
            data_width,
            direction: StreamDirection::Producer,
            has_tlast: false,
            has_tkeep: false,
            has_tid: false,
            id_width: 4,
            has_tuser: false,
            user_width: 1,
            body: Vec::new(),
        }
    }
    pub fn consumer(name: impl Into<String>, data_width: u32) -> Self {
        let mut m = Self::producer(name, data_width);
        m.direction = StreamDirection::Consumer;
        m
    }
    pub fn with_tlast(mut self) -> Self {
        self.has_tlast = true;
        self
    }
    pub fn with_tkeep(mut self) -> Self {
        self.has_tkeep = true;
        self
    }
    pub fn with_tid(mut self, width: u32) -> Self {
        self.has_tid = true;
        self.id_width = width;
        self
    }
    pub fn with_tuser(mut self, width: u32) -> Self {
        self.has_tuser = true;
        self.user_width = width;
        self
    }
    pub fn add_stmt(mut self, s: impl Into<String>) -> Self {
        self.body.push(s.into());
        self
    }
    pub fn emit(&self) -> String {
        let is_prod = self.direction == StreamDirection::Producer;
        let tdata_dir = if is_prod { "Output" } else { "Input" };
        let tvalid_dir = if is_prod { "Output" } else { "Input" };
        let tready_dir = if is_prod { "Input" } else { "Output" };
        let mut out = format!(
            "class {} extends Module {{\n  val io = IO(new Bundle {{\n",
            self.name
        );
        out.push_str(&format!(
            "    val tdata  = {}(UInt({}.W))\n",
            tdata_dir, self.data_width
        ));
        out.push_str(&format!("    val tvalid = {}(Bool())\n", tvalid_dir));
        out.push_str(&format!("    val tready = {}(Bool())\n", tready_dir));
        if self.has_tlast {
            out.push_str(&format!("    val tlast  = {}(Bool())\n", tdata_dir));
        }
        if self.has_tkeep {
            let kw = self.data_width / 8;
            out.push_str(&format!("    val tkeep  = {}(UInt({}.W))\n", tdata_dir, kw));
        }
        if self.has_tid {
            out.push_str(&format!(
                "    val tid    = {}(UInt({}.W))\n",
                tdata_dir, self.id_width
            ));
        }
        if self.has_tuser {
            out.push_str(&format!(
                "    val tuser  = {}(UInt({}.W))\n",
                tdata_dir, self.user_width
            ));
        }
        out.push_str("  })\n\n");
        for stmt in &self.body {
            out.push_str(&format!("  {}\n", stmt));
        }
        out.push_str("}\n");
        out
    }
}
/// A Chisel Decoupled / ReadyValid interface bundle descriptor.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselReadyValidBundle {
    pub data_type: ChiselType,
    pub has_last: bool,
    pub has_keep: bool,
    pub keep_width: u32,
}
#[allow(dead_code)]
impl ChiselReadyValidBundle {
    pub fn new(data_type: ChiselType) -> Self {
        Self {
            data_type,
            has_last: false,
            has_keep: false,
            keep_width: 0,
        }
    }
    pub fn with_last(mut self) -> Self {
        self.has_last = true;
        self
    }
    pub fn with_keep(mut self, width: u32) -> Self {
        self.has_keep = true;
        self.keep_width = width;
        self
    }
    /// Emit the Decoupled\[T\] port declaration.
    pub fn emit_decoupled(&self, name: &str, is_output: bool) -> String {
        let dir = if is_output {
            "Decoupled"
        } else {
            "Flipped(Decoupled"
        };
        let close = if is_output { "" } else { ")" };
        let mut out = format!("    val {} = {}({}){}\n", name, dir, self.data_type, close);
        if self.has_last {
            out.push_str(&format!(
                "    val {}_last = if (is_output) Output(Bool()) else Input(Bool())\n",
                name
            ));
        }
        if self.has_keep {
            out.push_str(&format!(
                "    val {}_keep = if (is_output) Output(UInt({}.W)) else Input(UInt({}.W))\n",
                name, self.keep_width, self.keep_width
            ));
        }
        out
    }
    /// Emit fire condition (valid && ready).
    pub fn emit_fire(&self, port_name: &str) -> String {
        format!(
            "val {}_fire = {}.valid && {}.ready\n",
            port_name, port_name, port_name
        )
    }
    /// Emit a queue buffer for this channel.
    pub fn emit_queue(&self, input: &str, output: &str, depth: u32) -> String {
        format!(
            "val {}_q = Queue({}, {})\n{}_q <> {}\n{} <> {}_q\n",
            input, input, depth, input, input, output, input
        )
    }
}
/// A complete Chisel module definition.
#[derive(Debug, Clone)]
pub struct ChiselModule {
    /// Class name for the Chisel module
    pub name: String,
    /// I/O ports
    pub ports: Vec<ChiselPort>,
    /// Body statements (pre-formatted Scala strings)
    pub body: Vec<String>,
}
impl ChiselModule {
    /// Construct a new, empty Chisel module.
    pub fn new(name: impl Into<String>) -> Self {
        ChiselModule {
            name: name.into(),
            ports: Vec::new(),
            body: Vec::new(),
        }
    }
    /// Add an input port.
    pub fn add_input(&mut self, name: impl Into<String>, ty: ChiselType) {
        self.ports.push(ChiselPort::input(name, ty));
    }
    /// Add an output port.
    pub fn add_output(&mut self, name: impl Into<String>, ty: ChiselType) {
        self.ports.push(ChiselPort::output(name, ty));
    }
    /// Append a body statement.
    pub fn add_stmt(&mut self, stmt: impl Into<String>) {
        self.body.push(stmt.into());
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChiselPassConfig {
    pub phase: ChiselPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl ChiselPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: ChiselPassPhase) -> Self {
        ChiselPassConfig {
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
/// Describes one stage of a pipeline register chain.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub name: String,
    pub data_type: ChiselType,
    pub has_valid: bool,
    pub has_stall: bool,
}
#[allow(dead_code)]
impl PipelineStage {
    pub fn new(name: impl Into<String>, data_type: ChiselType) -> Self {
        Self {
            name: name.into(),
            data_type,
            has_valid: false,
            has_stall: false,
        }
    }
    pub fn with_valid(mut self) -> Self {
        self.has_valid = true;
        self
    }
    pub fn with_stall(mut self) -> Self {
        self.has_stall = true;
        self
    }
}
