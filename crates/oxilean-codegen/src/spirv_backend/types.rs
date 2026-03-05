//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

/// Image format for OpTypeImage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageFormat {
    Unknown,
    Rgba32f,
    Rgba16f,
    R32f,
    Rgba8,
    R32i,
    R32ui,
}
/// SPIR-V decoration values.
#[derive(Debug, Clone, PartialEq)]
pub enum Decoration {
    RelaxedPrecision,
    SpecId(u32),
    Block,
    BufferBlock,
    RowMajor,
    ColMajor,
    ArrayStride(u32),
    MatrixStride(u32),
    GlslShared,
    GlslPacked,
    CPacked,
    BuiltIn(BuiltIn),
    NoPerspective,
    Flat,
    Patch,
    Centroid,
    Sample,
    Invariant,
    Restrict,
    Aliased,
    Volatile,
    Constant,
    Coherent,
    NonWritable,
    NonReadable,
    Uniform,
    UniformId(u32),
    SaturatedConversion,
    Stream(u32),
    Location(u32),
    Component(u32),
    Index(u32),
    Binding(u32),
    DescriptorSet(u32),
    Offset(u32),
    XfbBuffer(u32),
    XfbStride(u32),
    FuncParamAttr(u32),
    FPRoundingMode(u32),
    FPFastMathMode(u32),
    LinkageAttributes(String, u32),
    NoContraction,
    InputAttachmentIndex(u32),
    Alignment(u32),
    MaxByteOffset(u64),
    AlignmentId(u32),
    MaxByteOffsetId(u32),
}
/// SPIR-V addressing models.
#[derive(Debug, Clone, PartialEq)]
pub enum AddressingModel {
    Logical,
    Physical32,
    Physical64,
    PhysicalStorageBuffer64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SPIRVPassConfig {
    pub phase: SPIRVPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl SPIRVPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: SPIRVPassPhase) -> Self {
        SPIRVPassConfig {
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
/// A single SPIR-V instruction.
///
/// Every instruction has an opcode, optional result ID, and a list of
/// word operands. The result type ID is stored as the first operand
/// when the instruction produces a value.
#[derive(Debug, Clone, PartialEq)]
pub struct SpirVInstruction {
    /// Result ID assigned to this instruction (None for side-effecting ops)
    pub result_id: Option<u32>,
    /// Result type ID (present iff result_id is Some)
    pub result_type_id: Option<u32>,
    /// The SPIR-V opcode
    pub opcode: SpirVOp,
    /// Additional word operands (IDs, literals, enumerants)
    pub operands: Vec<u32>,
}
impl SpirVInstruction {
    /// Create an instruction that produces a result value.
    pub fn with_result(
        result_id: u32,
        result_type_id: u32,
        opcode: SpirVOp,
        operands: Vec<u32>,
    ) -> Self {
        Self {
            result_id: Some(result_id),
            result_type_id: Some(result_type_id),
            opcode,
            operands,
        }
    }
    /// Create an instruction without a result (e.g., OpStore, OpReturn).
    pub fn no_result(opcode: SpirVOp, operands: Vec<u32>) -> Self {
        Self {
            result_id: None,
            result_type_id: None,
            opcode,
            operands,
        }
    }
    /// Emit the instruction as a SPIR-V assembly text line.
    pub fn emit_text(&self) -> String {
        let result_part = match (self.result_id, self.result_type_id) {
            (Some(rid), Some(rtid)) => format!("%{} = %{} ", rid, rtid),
            _ => String::new(),
        };
        let ops: Vec<String> = self.operands.iter().map(|o| format!("%{}", o)).collect();
        if ops.is_empty() {
            format!("{}{}", result_part, self.opcode)
        } else {
            format!("{}{} {}", result_part, self.opcode, ops.join(" "))
        }
    }
    /// Estimate the word count of this instruction.
    pub fn word_count(&self) -> u32 {
        let base = 1;
        let result_words = if self.result_id.is_some() { 2 } else { 0 };
        base + result_words + self.operands.len() as u32
    }
}
#[allow(dead_code)]
pub struct SPIRVPassRegistry {
    pub(super) configs: Vec<SPIRVPassConfig>,
    pub(super) stats: std::collections::HashMap<String, SPIRVPassStats>,
}
impl SPIRVPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SPIRVPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: SPIRVPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), SPIRVPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&SPIRVPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&SPIRVPassStats> {
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
/// SPIR-V execution models (shader stage).
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionModel {
    Vertex,
    TessellationControl,
    TessellationEvaluation,
    Geometry,
    Fragment,
    GLCompute,
    Kernel,
    TaskNV,
    MeshNV,
    RayGenerationKHR,
    IntersectionKHR,
    AnyHitKHR,
    ClosestHitKHR,
    MissKHR,
    CallableKHR,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SPIRVCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A basic block within a SPIR-V function.
#[derive(Debug, Clone)]
pub struct SpirVBasicBlock {
    /// Label ID for this block (OpLabel)
    pub label_id: u32,
    /// Instructions within this block (excluding the label)
    pub instructions: Vec<SpirVInstruction>,
}
impl SpirVBasicBlock {
    /// Create a new empty basic block.
    pub fn new(label_id: u32) -> Self {
        Self {
            label_id,
            instructions: Vec::new(),
        }
    }
    /// Add an instruction.
    pub fn push(&mut self, instr: SpirVInstruction) {
        self.instructions.push(instr);
    }
    /// Emit the basic block as SPIR-V assembly text.
    pub fn emit_text(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("%{} = OpLabel", self.label_id));
        for instr in &self.instructions {
            lines.push(format!("  {}", instr.emit_text()));
        }
        lines.join("\n")
    }
    /// Count the instructions in this block.
    pub fn instr_count(&self) -> usize {
        self.instructions.len()
    }
}
/// SPIR-V capabilities (OpCapability).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpirVCapability {
    Matrix,
    Shader,
    Geometry,
    Tessellation,
    Addresses,
    Linkage,
    Kernel,
    Vector16,
    Float16Buffer,
    Float16,
    Float64,
    Int64,
    Int64Atomics,
    ImageBasic,
    ImageReadWrite,
    ImageMipmap,
    Sampled1D,
    Image1D,
    SampledCubeArray,
    SampledBuffer,
    ImageBuffer,
    ImageMSArray,
    StorageImageExtendedFormats,
    ImageQuery,
    DerivativeControl,
    InterpolationFunction,
    TransformFeedback,
    GeometryStreams,
    StorageImageReadWithoutFormat,
    StorageImageWriteWithoutFormat,
    MultiViewport,
    SubgroupDispatch,
    NamedBarrier,
    PipeStorage,
    GroupNonUniform,
    GroupNonUniformVote,
    GroupNonUniformArithmetic,
    GroupNonUniformBallot,
    GroupNonUniformShuffle,
    GroupNonUniformShuffleRelative,
    VulkanMemoryModel,
    PhysicalStorageBufferAddresses,
    DemoteToHelperInvocation,
    AtomicFloat32AddExt,
    AtomicFloat64AddExt,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SPIRVAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, SPIRVCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl SPIRVAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        SPIRVAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&SPIRVCacheEntry> {
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
            SPIRVCacheEntry {
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
/// SPIR-V memory models.
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryModel {
    Simple,
    GLSL450,
    OpenCL,
    Vulkan,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SPIRVDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl SPIRVDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SPIRVDepGraph {
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
#[derive(Debug, Clone, Default)]
pub struct SPIRVPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl SPIRVPassStats {
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
/// SPIR-V type system.
///
/// Each type variant corresponds to one or more SPIR-V OpType* instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum SpirVType {
    /// OpTypeVoid — no value
    Void,
    /// OpTypeBool — true/false
    Bool,
    /// OpTypeInt — signed/unsigned integer, width in bits
    Int {
        /// Bit width: 8, 16, 32, or 64
        width: u32,
        /// 1 = signed, 0 = unsigned
        signed: bool,
    },
    /// OpTypeFloat — IEEE floating-point, width in bits (16, 32, 64)
    Float {
        /// Bit width: 16, 32, or 64
        width: u32,
    },
    /// OpTypeVector — fixed-length homogeneous vector
    Vector {
        /// Element type (must be scalar numeric)
        element: Box<SpirVType>,
        /// Number of components (2, 3, or 4)
        count: u32,
    },
    /// OpTypeMatrix — column-major matrix of vectors
    Matrix {
        /// Column type (must be a vector)
        column_type: Box<SpirVType>,
        /// Number of columns
        column_count: u32,
    },
    /// OpTypeArray — fixed-length array
    Array {
        /// Element type
        element: Box<SpirVType>,
        /// Length (must be a constant)
        length: u32,
    },
    /// OpTypeRuntimeArray — runtime-length array (descriptor binding)
    RuntimeArray(Box<SpirVType>),
    /// OpTypeStruct — aggregate of named/indexed members
    Struct(Vec<SpirVType>),
    /// OpTypePointer — typed pointer with storage class
    Pointer {
        /// Storage class (e.g., Uniform, Input, Output, Function)
        storage_class: StorageClass,
        /// Pointed-to type
        pointee: Box<SpirVType>,
    },
    /// OpTypeFunction — function signature
    Function {
        /// Return type
        return_type: Box<SpirVType>,
        /// Parameter types
        param_types: Vec<SpirVType>,
    },
    /// OpTypeImage — image type for sampling/storage
    Image {
        /// Sampled element type
        sampled_type: Box<SpirVType>,
        /// Image dimensionality
        dim: ImageDim,
        /// Depth (0=no depth, 1=depth, 2=unknown)
        depth: u32,
        /// Arrayed (0=no, 1=yes)
        arrayed: u32,
        /// Multisampled (0=no, 1=yes)
        ms: u32,
        /// Sampled (1=sampled, 2=storage, 0=unknown)
        sampled: u32,
        /// Image format
        format: ImageFormat,
    },
    /// OpTypeSampler — opaque sampler type
    Sampler,
    /// OpTypeSampledImage — combined image+sampler
    SampledImage(Box<SpirVType>),
}
/// The SPIR-V code generation backend for OxiLean.
///
/// Translates OxiLean expressions and declarations into SPIR-V modules
/// targeting Vulkan compute or graphics shaders.
#[derive(Debug)]
pub struct SpirVBackend {
    /// The SPIR-V module being built
    pub module: SpirVModule,
    /// Map from type key to allocated type ID
    pub(super) type_cache: HashMap<String, u32>,
    /// Map from symbol name to ID
    pub(super) symbol_table: HashMap<String, u32>,
    /// ID of the imported GLSL.std.450 extended instruction set
    pub(super) glsl_ext_id: Option<u32>,
    /// ID of the void type
    pub(super) void_type_id: Option<u32>,
    /// ID of the bool type
    pub(super) bool_type_id: Option<u32>,
}
impl SpirVBackend {
    /// Create a new SPIR-V backend.
    pub fn new() -> Self {
        Self {
            module: SpirVModule::new(),
            type_cache: HashMap::new(),
            symbol_table: HashMap::new(),
            glsl_ext_id: None,
            void_type_id: None,
            bool_type_id: None,
        }
    }
    /// Enable the Shader capability and GLSL 450 memory model (typical Vulkan setup).
    pub fn configure_for_vulkan(&mut self) {
        self.module.add_capability(SpirVCapability::Shader);
        self.module.memory_model = (AddressingModel::Logical, MemoryModel::GLSL450);
        let glsl_id = self.module.import_ext_inst("GLSL.std.450");
        self.glsl_ext_id = Some(glsl_id);
    }
    /// Configure for OpenCL/compute kernels.
    pub fn configure_for_opencl(&mut self) {
        self.module.add_capability(SpirVCapability::Kernel);
        self.module.add_capability(SpirVCapability::Addresses);
        self.module.memory_model = (AddressingModel::Physical64, MemoryModel::OpenCL);
    }
    /// Declare or retrieve the void type.
    pub fn get_void_type(&mut self) -> u32 {
        if let Some(id) = self.void_type_id {
            return id;
        }
        let id = self.module.fresh_id();
        self.module.add_type(SpirVInstruction::with_result(
            id,
            0,
            SpirVOp::Capability(SpirVCapability::Shader),
            vec![],
        ));
        self.void_type_id = Some(id);
        id
    }
    /// Declare a scalar integer type.
    pub fn declare_int_type(&mut self, width: u32, signed: bool) -> u32 {
        let key = format!("int_{}_{}", width, if signed { "s" } else { "u" });
        if let Some(&id) = self.type_cache.get(&key) {
            return id;
        }
        let id = self.module.fresh_id();
        self.module.add_type(SpirVInstruction::with_result(
            id,
            0,
            SpirVOp::Constant(width as u64),
            vec![if signed { 1 } else { 0 }],
        ));
        self.type_cache.insert(key, id);
        id
    }
    /// Declare a scalar float type.
    pub fn declare_float_type(&mut self, width: u32) -> u32 {
        let key = format!("float_{}", width);
        if let Some(&id) = self.type_cache.get(&key) {
            return id;
        }
        let id = self.module.fresh_id();
        self.module.add_type(SpirVInstruction::with_result(
            id,
            0,
            SpirVOp::Constant(width as u64),
            vec![],
        ));
        self.type_cache.insert(key, id);
        id
    }
    /// Declare a vector type.
    pub fn declare_vector_type(&mut self, element_type_id: u32, count: u32) -> u32 {
        let key = format!("vec_{}_{}", element_type_id, count);
        if let Some(&id) = self.type_cache.get(&key) {
            return id;
        }
        let id = self.module.fresh_id();
        self.module.add_type(SpirVInstruction::with_result(
            id,
            0,
            SpirVOp::CompositeConstruct,
            vec![element_type_id, count],
        ));
        self.type_cache.insert(key, id);
        id
    }
    /// Declare a pointer type.
    pub fn declare_pointer_type(
        &mut self,
        storage_class: StorageClass,
        pointee_type_id: u32,
    ) -> u32 {
        let key = format!("ptr_{:?}_{}", storage_class, pointee_type_id);
        if let Some(&id) = self.type_cache.get(&key) {
            return id;
        }
        let id = self.module.fresh_id();
        self.module.add_type(SpirVInstruction::with_result(
            id,
            0,
            SpirVOp::Variable(storage_class),
            vec![pointee_type_id],
        ));
        self.type_cache.insert(key, id);
        id
    }
    /// Declare a function type.
    pub fn declare_function_type(&mut self, return_type_id: u32, param_type_ids: Vec<u32>) -> u32 {
        let key = format!("fn_{}_[{}]", return_type_id, {
            let s: Vec<String> = param_type_ids.iter().map(|i| i.to_string()).collect();
            s.join(",")
        });
        if let Some(&id) = self.type_cache.get(&key) {
            return id;
        }
        let id = self.module.fresh_id();
        let mut operands = vec![return_type_id];
        operands.extend_from_slice(&param_type_ids);
        self.module.add_type(SpirVInstruction::with_result(
            id,
            0,
            SpirVOp::Function,
            operands,
        ));
        self.type_cache.insert(key, id);
        id
    }
    /// Declare an integer constant.
    pub fn declare_int_constant(&mut self, type_id: u32, value: u64) -> u32 {
        let id = self.module.fresh_id();
        self.module.add_constant(SpirVInstruction::with_result(
            id,
            type_id,
            SpirVOp::Constant(value),
            vec![value as u32, (value >> 32) as u32],
        ));
        id
    }
    /// Declare a float constant.
    pub fn declare_float_constant(&mut self, type_id: u32, value: f32) -> u32 {
        let id = self.module.fresh_id();
        let bits = value.to_bits();
        self.module.add_constant(SpirVInstruction::with_result(
            id,
            type_id,
            SpirVOp::Constant(bits as u64),
            vec![bits],
        ));
        id
    }
    /// Declare a bool constant.
    pub fn declare_bool_constant(&mut self, type_id: u32, value: bool) -> u32 {
        let id = self.module.fresh_id();
        let op = if value {
            SpirVOp::ConstantTrue
        } else {
            SpirVOp::ConstantFalse
        };
        self.module
            .add_constant(SpirVInstruction::with_result(id, type_id, op, vec![]));
        id
    }
    /// Declare a global variable.
    pub fn declare_global_variable(
        &mut self,
        name: impl Into<String>,
        type_id: u32,
        storage_class: StorageClass,
        decorations: Vec<Decoration>,
    ) -> u32 {
        let name = name.into();
        let id = self.module.fresh_id();
        self.module.set_name(id, name.clone());
        for deco in decorations {
            self.module.decorate(id, deco);
        }
        self.module.add_global_var(SpirVInstruction::with_result(
            id,
            type_id,
            SpirVOp::Variable(storage_class),
            vec![],
        ));
        self.symbol_table.insert(name, id);
        id
    }
    /// Begin building a new function.
    pub fn begin_function(
        &mut self,
        name: impl Into<String>,
        return_type_id: u32,
        param_type_ids: Vec<u32>,
    ) -> SpirVFunction {
        let name = name.into();
        let func_id = self.module.fresh_id();
        let func_type_id = self.declare_function_type(return_type_id, param_type_ids.clone());
        self.module.set_name(func_id, name.clone());
        self.symbol_table.insert(name.clone(), func_id);
        let mut func = SpirVFunction::new(func_id, Some(name), return_type_id, func_type_id);
        for &pt in &param_type_ids {
            let pid = self.module.fresh_id();
            func.add_param(pid, pt);
        }
        func
    }
    /// Finalize and add a function to the module.
    pub fn finish_function(&mut self, func: SpirVFunction) {
        self.module.add_function(func);
    }
    /// Emit the module as SPIR-V assembly text.
    pub fn emit_text(&self) -> String {
        self.module.emit_text()
    }
    /// Emit a minimal valid SPIR-V binary header (magic + version + generator + bound + schema).
    pub fn emit_binary_header(&self) -> Vec<u32> {
        vec![
            0x0723_0203,
            self.module.version,
            self.module.generator,
            self.module.bound,
            0,
        ]
    }
    /// Get the number of functions in the module.
    pub fn function_count(&self) -> usize {
        self.module.functions.len()
    }
    /// Look up a symbol by name.
    pub fn lookup_symbol(&self, name: &str) -> Option<u32> {
        self.symbol_table.get(name).copied()
    }
    /// Emit a compute shader skeleton for the given kernel name and local size.
    pub fn emit_compute_kernel(
        &mut self,
        kernel_name: impl Into<String>,
        local_size_x: u32,
        local_size_y: u32,
        local_size_z: u32,
    ) -> u32 {
        let kernel_name = kernel_name.into();
        self.configure_for_vulkan();
        self.module.add_capability(SpirVCapability::Shader);
        let void_id = self.module.fresh_id();
        let func_type_id = self.module.fresh_id();
        let func_id = self.module.fresh_id();
        let entry_block_id = self.module.fresh_id();
        self.module.set_name(func_id, kernel_name.clone());
        let mut func =
            SpirVFunction::new(func_id, Some(kernel_name.clone()), void_id, func_type_id);
        func.set_entry_point(ExecutionModel::GLCompute);
        let mut entry_block = SpirVBasicBlock::new(entry_block_id);
        entry_block.push(SpirVInstruction::no_result(SpirVOp::Return, vec![]));
        func.add_block(entry_block);
        self.module
            .add_entry_point(ExecutionModel::GLCompute, func_id, &kernel_name, vec![]);
        self.module.add_execution_mode(
            func_id,
            ExecutionMode::LocalSize(local_size_x, local_size_y, local_size_z),
        );
        self.module.add_function(func);
        self.symbol_table.insert(kernel_name, func_id);
        func_id
    }
}
/// A complete SPIR-V module.
///
/// Contains all declarations required to produce a valid SPIR-V binary:
/// capabilities, extensions, memory model, entry points, type declarations,
/// global variables, constants, and function bodies.
#[derive(Debug, Clone)]
pub struct SpirVModule {
    /// SPIR-V version (encoded as major<<16 | minor<<8)
    pub version: u32,
    /// Generator magic number
    pub generator: u32,
    /// Bound (next available ID)
    pub bound: u32,
    /// Declared capabilities
    pub capabilities: Vec<SpirVCapability>,
    /// Extension strings
    pub extensions: Vec<String>,
    /// Extended instruction set imports (name -> id)
    pub ext_inst_imports: HashMap<String, u32>,
    /// Addressing and memory model
    pub memory_model: (AddressingModel, MemoryModel),
    /// Entry points: (execution_model, function_id, name, interface_vars)
    pub entry_points: Vec<(ExecutionModel, u32, String, Vec<u32>)>,
    /// Execution modes per entry point
    pub execution_modes: Vec<(u32, ExecutionMode)>,
    /// Debug names (id -> name)
    pub debug_names: HashMap<u32, String>,
    /// Decorations (id -> list of decorations)
    pub decorations: HashMap<u32, Vec<Decoration>>,
    /// Type instructions (ordered by dependency)
    pub types: Vec<SpirVInstruction>,
    /// Constants
    pub constants: Vec<SpirVInstruction>,
    /// Global variables
    pub global_vars: Vec<SpirVInstruction>,
    /// Function definitions
    pub functions: Vec<SpirVFunction>,
}
impl SpirVModule {
    /// Create a new empty SPIR-V module targeting SPIR-V 1.6.
    pub fn new() -> Self {
        Self {
            version: (1 << 16) | (6 << 8),
            generator: 0x000D_0001,
            bound: 1,
            capabilities: Vec::new(),
            extensions: Vec::new(),
            ext_inst_imports: HashMap::new(),
            memory_model: (AddressingModel::Logical, MemoryModel::GLSL450),
            entry_points: Vec::new(),
            execution_modes: Vec::new(),
            debug_names: HashMap::new(),
            decorations: HashMap::new(),
            types: Vec::new(),
            constants: Vec::new(),
            global_vars: Vec::new(),
            functions: Vec::new(),
        }
    }
    /// Allocate a fresh ID.
    pub fn fresh_id(&mut self) -> u32 {
        let id = self.bound;
        self.bound += 1;
        id
    }
    /// Add a capability.
    pub fn add_capability(&mut self, cap: SpirVCapability) {
        if !self.capabilities.contains(&cap) {
            self.capabilities.push(cap);
        }
    }
    /// Add an extension.
    pub fn add_extension(&mut self, ext: impl Into<String>) {
        let ext = ext.into();
        if !self.extensions.contains(&ext) {
            self.extensions.push(ext);
        }
    }
    /// Import an extended instruction set and return its ID.
    pub fn import_ext_inst(&mut self, name: impl Into<String>) -> u32 {
        let name = name.into();
        if let Some(&id) = self.ext_inst_imports.get(&name) {
            return id;
        }
        let id = self.fresh_id();
        self.ext_inst_imports.insert(name, id);
        id
    }
    /// Add an entry point.
    pub fn add_entry_point(
        &mut self,
        model: ExecutionModel,
        func_id: u32,
        name: impl Into<String>,
        interface_vars: Vec<u32>,
    ) {
        self.entry_points
            .push((model, func_id, name.into(), interface_vars));
    }
    /// Add an execution mode for an entry point function.
    pub fn add_execution_mode(&mut self, func_id: u32, mode: ExecutionMode) {
        self.execution_modes.push((func_id, mode));
    }
    /// Assign a debug name to an ID.
    pub fn set_name(&mut self, id: u32, name: impl Into<String>) {
        self.debug_names.insert(id, name.into());
    }
    /// Add a decoration to an ID.
    pub fn decorate(&mut self, id: u32, decoration: Decoration) {
        self.decorations.entry(id).or_default().push(decoration);
    }
    /// Add a type instruction.
    pub fn add_type(&mut self, instr: SpirVInstruction) {
        self.types.push(instr);
    }
    /// Add a constant instruction.
    pub fn add_constant(&mut self, instr: SpirVInstruction) {
        self.constants.push(instr);
    }
    /// Add a global variable.
    pub fn add_global_var(&mut self, instr: SpirVInstruction) {
        self.global_vars.push(instr);
    }
    /// Add a function definition.
    pub fn add_function(&mut self, func: SpirVFunction) {
        self.functions.push(func);
    }
    /// Emit the module as SPIR-V assembly text.
    pub fn emit_text(&self) -> String {
        let mut lines = Vec::new();
        lines.push("; SPIR-V Module (OxiLean codegen)".to_string());
        lines.push(format!(
            "; Version: {}.{}",
            (self.version >> 16) & 0xFF,
            (self.version >> 8) & 0xFF
        ));
        lines.push(format!("; Bound: {}", self.bound));
        lines.push(String::new());
        for cap in &self.capabilities {
            lines.push(format!("OpCapability {:?}", cap));
        }
        for ext in &self.extensions {
            lines.push(format!("OpExtension \"{}\"", ext));
        }
        let mut sorted_imports: Vec<(&String, &u32)> = self.ext_inst_imports.iter().collect();
        sorted_imports.sort_by_key(|(_, &id)| id);
        for (name, id) in sorted_imports {
            lines.push(format!("%{} = OpExtInstImport \"{}\"", id, name));
        }
        lines.push(format!(
            "OpMemoryModel {:?} {:?}",
            self.memory_model.0, self.memory_model.1
        ));
        for (model, func_id, name, iface) in &self.entry_points {
            let iface_str: Vec<String> = iface.iter().map(|id| format!("%{}", id)).collect();
            lines.push(format!(
                "OpEntryPoint {:?} %{} \"{}\" {}",
                model,
                func_id,
                name,
                iface_str.join(" ")
            ));
        }
        for (func_id, mode) in &self.execution_modes {
            lines.push(format!("OpExecutionMode %{} {:?}", func_id, mode));
        }
        lines.push(String::new());
        lines.push("; Debug names".to_string());
        let mut sorted_names: Vec<(&u32, &String)> = self.debug_names.iter().collect();
        sorted_names.sort_by_key(|(&id, _)| id);
        for (id, name) in sorted_names {
            lines.push(format!("OpName %{} \"{}\"", id, name));
        }
        lines.push(String::new());
        lines.push("; Decorations".to_string());
        let mut sorted_decos: Vec<(&u32, &Vec<Decoration>)> = self.decorations.iter().collect();
        sorted_decos.sort_by_key(|(&id, _)| id);
        for (id, decos) in sorted_decos {
            for deco in decos {
                lines.push(format!("OpDecorate %{} {:?}", id, deco));
            }
        }
        lines.push(String::new());
        lines.push("; Types".to_string());
        for ty in &self.types {
            lines.push(ty.emit_text());
        }
        lines.push(String::new());
        lines.push("; Constants".to_string());
        for c in &self.constants {
            lines.push(c.emit_text());
        }
        lines.push(String::new());
        lines.push("; Global variables".to_string());
        for gv in &self.global_vars {
            lines.push(gv.emit_text());
        }
        lines.push(String::new());
        lines.push("; Functions".to_string());
        for func in &self.functions {
            lines.push(func.emit_text());
            lines.push(String::new());
        }
        lines.join("\n")
    }
    /// Estimate the total word count of the module binary.
    pub fn estimate_word_count(&self) -> u32 {
        let mut count = 5u32;
        for ty in &self.types {
            count += ty.word_count();
        }
        for c in &self.constants {
            count += c.word_count();
        }
        for gv in &self.global_vars {
            count += gv.word_count();
        }
        for func in &self.functions {
            count += 5 + 1;
            count += func.params.len() as u32 * 3;
            for block in &func.blocks {
                count += 2;
                for instr in &block.instructions {
                    count += instr.word_count();
                }
            }
        }
        count
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SPIRVLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl SPIRVLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        SPIRVLivenessInfo {
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
/// SPIR-V storage classes for pointer types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StorageClass {
    /// Shader uniform block binding
    Uniform,
    /// Storage buffer binding
    StorageBuffer,
    /// Push constant block
    PushConstant,
    /// Input variable (vertex/fragment attribute)
    Input,
    /// Output variable (vertex position, fragment color)
    Output,
    /// Function-local variable
    Function,
    /// Private (per-invocation, not shared)
    Private,
    /// Workgroup shared memory (compute)
    Workgroup,
    /// Cross-workgroup (global) memory
    CrossWorkgroup,
    /// Image (for storage images)
    Image,
    /// Generic pointer (OpenCL)
    Generic,
}
/// SPIR-V built-in variables.
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltIn {
    Position,
    PointSize,
    ClipDistance,
    CullDistance,
    VertexId,
    InstanceId,
    PrimitiveId,
    InvocationId,
    Layer,
    ViewportIndex,
    TessLevelOuter,
    TessLevelInner,
    TessCoord,
    PatchVertices,
    FragCoord,
    PointCoord,
    FrontFacing,
    SampleId,
    SamplePosition,
    SampleMask,
    FragDepth,
    HelperInvocation,
    NumWorkgroups,
    WorkgroupSize,
    WorkgroupId,
    LocalInvocationId,
    GlobalInvocationId,
    LocalInvocationIndex,
    WorkDim,
    GlobalSize,
    EnqueuedWorkgroupSize,
    GlobalOffset,
    GlobalLinearId,
    SubgroupSize,
    SubgroupMaxSize,
    NumSubgroups,
    NumEnqueuedSubgroups,
    SubgroupId,
    SubgroupLocalInvocationId,
    VertexIndex,
    InstanceIndex,
}
/// GLSLstd450 extended instruction opcodes (a representative subset).
#[derive(Debug, Clone, PartialEq)]
pub enum GlslStd450Op {
    Round,
    RoundEven,
    Trunc,
    FAbs,
    SAbs,
    FSign,
    SSign,
    Floor,
    Ceil,
    Fract,
    Radians,
    Degrees,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Atan2,
    Sinh,
    Cosh,
    Exp,
    Log,
    Exp2,
    Log2,
    Sqrt,
    InverseSqrt,
    Pow,
    FMin,
    UMin,
    SMin,
    FMax,
    UMax,
    SMax,
    FClamp,
    UClamp,
    SClamp,
    FMix,
    Step,
    SmoothStep,
    Fma,
    Length,
    Distance,
    Cross,
    Normalize,
    Reflect,
    Refract,
    FaceForward,
    MatrixInverse,
    ModfStruct,
    FrexpStruct,
    LdexpStruct,
    PackSnorm4x8,
    PackUnorm4x8,
    UnpackSnorm4x8,
    UnpackUnorm4x8,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SPIRVDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl SPIRVDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        SPIRVDominatorTree {
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
/// Image dimensionality for OpTypeImage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageDim {
    Dim1D,
    Dim2D,
    Dim3D,
    Cube,
    Rect,
    Buffer,
    SubpassData,
}
#[allow(dead_code)]
pub struct SPIRVConstantFoldingHelper;
impl SPIRVConstantFoldingHelper {
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
/// SPIR-V opcodes (a representative subset).
///
/// Each variant corresponds to a SPIR-V instruction from the specification.
/// Numeric opcode values follow the SPIR-V 1.6 spec (Table 1).
#[derive(Debug, Clone, PartialEq)]
pub enum SpirVOp {
    /// OpVariable: declare a variable with storage class
    Variable(StorageClass),
    /// OpLoad: load from a pointer
    Load,
    /// OpStore: store to a pointer
    Store,
    /// OpAccessChain: compute a pointer into a composite
    AccessChain,
    /// OpCopyObject: copy a value
    CopyObject,
    /// OpIAdd: integer addition
    IAdd,
    /// OpISub: integer subtraction
    ISub,
    /// OpIMul: integer multiplication
    IMul,
    /// OpSDiv: signed integer division
    SDiv,
    /// OpUDiv: unsigned integer division
    UDiv,
    /// OpSMod: signed modulo
    SMod,
    /// OpUMod: unsigned modulo
    UMod,
    /// OpSNegate: signed negation
    SNegate,
    /// OpFAdd: float addition
    FAdd,
    /// OpFSub: float subtraction
    FSub,
    /// OpFMul: float multiplication
    FMul,
    /// OpFDiv: float division
    FDiv,
    /// OpFMod: float modulo
    FMod,
    /// OpFNegate: float negation
    FNegate,
    /// OpFRem: float remainder
    FRem,
    /// OpIEqual: integer equality
    IEqual,
    /// OpINotEqual: integer inequality
    INotEqual,
    /// OpSLessThan: signed less-than
    SLessThan,
    /// OpSLessThanEqual: signed less-than-or-equal
    SLessThanEqual,
    /// OpSGreaterThan: signed greater-than
    SGreaterThan,
    /// OpULessThan: unsigned less-than
    ULessThan,
    /// OpFOrdEqual: float ordered equal
    FOrdEqual,
    /// OpFOrdLessThan: float ordered less-than
    FOrdLessThan,
    /// OpFOrdGreaterThan: float ordered greater-than
    FOrdGreaterThan,
    /// OpLogicalAnd: logical and
    LogicalAnd,
    /// OpLogicalOr: logical or
    LogicalOr,
    /// OpLogicalNot: logical not
    LogicalNot,
    /// OpLogicalEqual: logical equality
    LogicalEqual,
    /// OpBitwiseAnd: bitwise and
    BitwiseAnd,
    /// OpBitwiseOr: bitwise or
    BitwiseOr,
    /// OpBitwiseXor: bitwise xor
    BitwiseXor,
    /// OpNot: bitwise not
    Not,
    /// OpShiftLeftLogical: logical shift left
    ShiftLeftLogical,
    /// OpShiftRightLogical: logical shift right
    ShiftRightLogical,
    /// OpShiftRightArithmetic: arithmetic shift right
    ShiftRightArithmetic,
    /// OpConvertFToS: float-to-signed-int
    ConvertFToS,
    /// OpConvertFToU: float-to-unsigned-int
    ConvertFToU,
    /// OpConvertSToF: signed-int-to-float
    ConvertSToF,
    /// OpConvertUToF: unsigned-int-to-float
    ConvertUToF,
    /// OpFConvert: float-to-float conversion
    FConvert,
    /// OpSConvert: signed integer bit-width conversion
    SConvert,
    /// OpUConvert: unsigned integer bit-width conversion
    UConvert,
    /// OpBitcast: reinterpret bits as another type
    Bitcast,
    /// OpCompositeConstruct: build a vector/matrix/struct/array
    CompositeConstruct,
    /// OpCompositeExtract: extract a component from a composite
    CompositeExtract,
    /// OpCompositeInsert: insert a value into a composite
    CompositeInsert,
    /// OpVectorShuffle: permute vector components
    VectorShuffle,
    /// OpVectorExtractDynamic: extract at runtime index
    VectorExtractDynamic,
    /// OpVectorInsertDynamic: insert at runtime index
    VectorInsertDynamic,
    /// OpMatrixTimesVector: mat * vec
    MatrixTimesVector,
    /// OpVectorTimesMatrix: vec * mat
    VectorTimesMatrix,
    /// OpMatrixTimesMatrix: mat * mat
    MatrixTimesMatrix,
    /// OpMatrixTimesScalar: mat * scalar
    MatrixTimesScalar,
    /// OpDot: dot product of two vectors
    Dot,
    /// OpOuterProduct: outer product of two vectors
    OuterProduct,
    /// OpTranspose: matrix transpose
    Transpose,
    /// OpLabel: basic block label
    Label,
    /// OpBranch: unconditional branch
    Branch,
    /// OpBranchConditional: conditional branch
    BranchConditional,
    /// OpSwitch: switch on integer
    Switch,
    /// OpReturn: return from function with void
    Return,
    /// OpReturnValue: return a value
    ReturnValue,
    /// OpUnreachable: mark unreachable code
    Unreachable,
    /// OpPhi: SSA phi node
    Phi,
    /// OpLoopMerge: loop structure hint
    LoopMerge,
    /// OpSelectionMerge: selection structure hint
    SelectionMerge,
    /// OpFunction: begin a function definition
    Function,
    /// OpFunctionParameter: declare a parameter
    FunctionParameter,
    /// OpFunctionEnd: end a function definition
    FunctionEnd,
    /// OpFunctionCall: call a function
    FunctionCall,
    /// OpImageSampleImplicitLod: sample image with implicit LOD
    ImageSampleImplicitLod,
    /// OpImageSampleExplicitLod: sample image with explicit LOD
    ImageSampleExplicitLod,
    /// OpImageLoad: load from a storage image
    ImageLoad,
    /// OpImageStore: store to a storage image
    ImageStore,
    /// OpAtomicLoad: atomic load
    AtomicLoad,
    /// OpAtomicStore: atomic store
    AtomicStore,
    /// OpAtomicIAdd: atomic integer add
    AtomicIAdd,
    /// OpAtomicISub: atomic integer subtract
    AtomicISub,
    /// OpAtomicCompareExchange: compare and swap
    AtomicCompareExchange,
    /// OpExtInst (GLSLstd450) — standard math functions
    ExtInstGlsl(GlslStd450Op),
    /// OpCapability: declare a SPIR-V capability
    Capability(SpirVCapability),
    /// OpExtension: import a SPIR-V extension
    Extension(String),
    /// OpExtInstImport: import extended instruction set
    ExtInstImport(String),
    /// OpMemoryModel: set addressing + memory model
    MemoryModel(AddressingModel, MemoryModel),
    /// OpEntryPoint: declare a shader entry point
    EntryPoint(ExecutionModel, String),
    /// OpExecutionMode: declare execution mode for entry point
    ExecutionMode(ExecutionMode),
    /// OpDecorate: decorate an ID with metadata
    Decorate(Decoration),
    /// OpMemberDecorate: decorate a struct member
    MemberDecorate(u32, Decoration),
    /// OpName: assign debug name to an ID
    Name(String),
    /// OpConstant: scalar constant value
    Constant(u64),
    /// OpConstantComposite: composite constant
    ConstantComposite,
    /// OpConstantTrue: boolean true
    ConstantTrue,
    /// OpConstantFalse: boolean false
    ConstantFalse,
    /// OpTypeForwardPointer: forward declaration of pointer type
    TypeForwardPointer(StorageClass),
    /// OpControlBarrier: synchronization barrier
    ControlBarrier,
    /// OpMemoryBarrier: memory barrier
    MemoryBarrier,
}
/// SPIR-V execution modes.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionMode {
    Invocations(u32),
    SpacingEqual,
    SpacingFractionalEven,
    SpacingFractionalOdd,
    VertexOrderCw,
    VertexOrderCcw,
    PixelCenterInteger,
    OriginUpperLeft,
    OriginLowerLeft,
    EarlyFragmentTests,
    PointMode,
    Xfb,
    DepthReplacing,
    DepthGreater,
    DepthLess,
    DepthUnchanged,
    LocalSize(u32, u32, u32),
    LocalSizeHint(u32, u32, u32),
    InputPoints,
    InputLines,
    InputLinesAdjacency,
    Triangles,
    InputTrianglesAdjacency,
    Quads,
    Isolines,
    OutputVertices(u32),
    OutputPoints,
    OutputLineStrip,
    OutputTriangleStrip,
}
/// A SPIR-V function definition.
#[derive(Debug, Clone)]
pub struct SpirVFunction {
    /// Function result ID
    pub id: u32,
    /// Function return type ID
    pub return_type_id: u32,
    /// Function type ID (full signature)
    pub function_type_id: u32,
    /// Debug name (from OpName)
    pub name: Option<String>,
    /// Parameter IDs (one per OpFunctionParameter)
    pub params: Vec<(u32, u32)>,
    /// Basic blocks (first is entry block)
    pub blocks: Vec<SpirVBasicBlock>,
    /// Whether this is an entry point
    pub is_entry_point: bool,
    /// Execution model (if entry point)
    pub execution_model: Option<ExecutionModel>,
}
impl SpirVFunction {
    /// Create a new function.
    pub fn new(id: u32, name: Option<String>, return_type_id: u32, function_type_id: u32) -> Self {
        Self {
            id,
            return_type_id,
            function_type_id,
            name,
            params: Vec::new(),
            blocks: Vec::new(),
            is_entry_point: false,
            execution_model: None,
        }
    }
    /// Add a parameter (param_id, type_id).
    pub fn add_param(&mut self, param_id: u32, type_id: u32) {
        self.params.push((param_id, type_id));
    }
    /// Add a basic block.
    pub fn add_block(&mut self, block: SpirVBasicBlock) {
        self.blocks.push(block);
    }
    /// Mark as an entry point.
    pub fn set_entry_point(&mut self, model: ExecutionModel) {
        self.is_entry_point = true;
        self.execution_model = Some(model);
    }
    /// Emit the function as SPIR-V assembly text.
    pub fn emit_text(&self) -> String {
        let mut lines = Vec::new();
        let name_comment = self
            .name
            .as_deref()
            .map(|n| format!(" ; {}", n))
            .unwrap_or_default();
        lines.push(format!(
            "%{} = OpFunction %{} None %{}{}",
            self.id, self.return_type_id, self.function_type_id, name_comment
        ));
        for (pid, tid) in &self.params {
            lines.push(format!("  %{} = OpFunctionParameter %{}", pid, tid));
        }
        for block in &self.blocks {
            lines.push(block.emit_text());
        }
        lines.push("OpFunctionEnd".to_string());
        lines.join("\n")
    }
    /// Get the number of instructions across all blocks.
    pub fn total_instrs(&self) -> usize {
        self.blocks.iter().map(|b| b.instr_count()).sum()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SPIRVPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl SPIRVPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            SPIRVPassPhase::Analysis => "analysis",
            SPIRVPassPhase::Transformation => "transformation",
            SPIRVPassPhase::Verification => "verification",
            SPIRVPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            SPIRVPassPhase::Transformation | SPIRVPassPhase::Cleanup
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SPIRVWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl SPIRVWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SPIRVWorklist {
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
