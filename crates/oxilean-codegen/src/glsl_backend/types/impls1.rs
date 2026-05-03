//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls2::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// std140 / std430 layout for UBOs.
/// A folded GLSL constant value.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GlslConstant {
    Float(f64),
    Int(i64),
    Bool(bool),
    Vec(Vec<f64>),
}
#[allow(dead_code)]
impl GlslConstant {
    /// Emit as a GLSL literal string.
    pub fn to_glsl_literal(&self) -> String {
        match self {
            GlslConstant::Float(v) => {
                if v.fract() == 0.0 {
                    format!("{:.1}", v)
                } else {
                    format!("{}", v)
                }
            }
            GlslConstant::Int(v) => format!("{}", v),
            GlslConstant::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            GlslConstant::Vec(c) => {
                let inner: Vec<String> = c
                    .iter()
                    .map(|v| {
                        if v.fract() == 0.0 {
                            format!("{:.1}", v)
                        } else {
                            format!("{}", v)
                        }
                    })
                    .collect();
                format!("vec{}({})", c.len(), inner.join(", "))
            }
        }
    }
    /// Is this the additive identity?
    pub fn is_zero(&self) -> bool {
        match self {
            GlslConstant::Float(v) => *v == 0.0,
            GlslConstant::Int(v) => *v == 0,
            GlslConstant::Bool(b) => !b,
            GlslConstant::Vec(c) => c.iter().all(|x| *x == 0.0),
        }
    }
    /// Is this the multiplicative identity?
    pub fn is_one(&self) -> bool {
        match self {
            GlslConstant::Float(v) => *v == 1.0,
            GlslConstant::Int(v) => *v == 1,
            _ => false,
        }
    }
    /// Add two compatible constants.
    pub fn add(&self, other: &GlslConstant) -> Option<GlslConstant> {
        match (self, other) {
            (GlslConstant::Float(a), GlslConstant::Float(b)) => Some(GlslConstant::Float(a + b)),
            (GlslConstant::Int(a), GlslConstant::Int(b)) => Some(GlslConstant::Int(a + b)),
            _ => None,
        }
    }
}
/// Tracks output variables declared by a GLSL shader stage.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GlslOutputVariableSet {
    pub(crate) vars: Vec<(String, GLSLType, Option<u32>)>,
}
#[allow(dead_code)]
impl GlslOutputVariableSet {
    pub fn new() -> Self {
        GlslOutputVariableSet { vars: Vec::new() }
    }
    pub fn add(&mut self, name: impl Into<String>, ty: GLSLType, location: Option<u32>) {
        self.vars.push((name.into(), ty, location));
    }
    pub fn len(&self) -> usize {
        self.vars.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }
    pub fn emit(&self, version: GLSLVersion) -> String {
        let mut out = String::new();
        for (name, ty, loc) in &self.vars {
            if let Some(l) = loc {
                if version.supports_layout_location() {
                    out.push_str(&format!("layout(location = {}) ", l));
                }
            }
            out.push_str(&format!("out {} {};\n", ty.keyword(), name));
        }
        out
    }
}
/// Pass execution phase for GLSLExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GLSLExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl GLSLExtPassPhase {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GLSLCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Liveness analysis for GLSLExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GLSLExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl GLSLExtLiveness {
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
#[derive(Debug, Clone, Default)]
pub struct GLSLPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl GLSLPassStats {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GLSLWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}
impl GLSLWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GLSLWorklist {
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
pub struct GLSLDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}
impl GLSLDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GLSLDepGraph {
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
/// Metadata about a GLSL built-in function.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GlslBuiltinFn {
    pub name: &'static str,
    pub category: GlslBuiltinCategory,
    pub min_version: u32,
}
impl GlslBuiltinFn {
    pub(crate) const fn new(name: &'static str, cat: GlslBuiltinCategory, ver: u32) -> Self {
        GlslBuiltinFn {
            name,
            category: cat,
            min_version: ver,
        }
    }
}
/// A complete GLSL shader for a single pipeline stage.
#[derive(Debug, Clone)]
pub struct GLSLShader {
    /// Target GLSL version.
    pub version: GLSLVersion,
    /// Pipeline stage.
    pub stage: GLSLShaderStage,
    /// Extension enable directives (e.g. `"GL_ARB_shader_storage_buffer_object"`).
    pub extensions: Vec<String>,
    /// Global `#define` macros (name, optional value).
    pub defines: Vec<(String, Option<String>)>,
    /// Struct type definitions (emitted before globals).
    pub structs: Vec<GLSLStruct>,
    /// Global `in` variables (stage inputs).
    pub inputs: Vec<GLSLVariable>,
    /// Global `out` variables (stage outputs).
    pub outputs: Vec<GLSLVariable>,
    /// Global `uniform` variables.
    pub uniforms: Vec<GLSLVariable>,
    /// Additional global variables (e.g. shared memory in compute).
    pub globals: Vec<GLSLVariable>,
    /// Helper functions (emitted before `main`).
    pub functions: Vec<GLSLFunction>,
    /// Body statements of the implicit `void main()` function.
    pub main_body: Vec<String>,
}
impl GLSLShader {
    /// Create a new, empty shader for the given version and stage.
    pub fn new(version: GLSLVersion, stage: GLSLShaderStage) -> Self {
        GLSLShader {
            version,
            stage,
            extensions: Vec::new(),
            defines: Vec::new(),
            structs: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            uniforms: Vec::new(),
            globals: Vec::new(),
            functions: Vec::new(),
            main_body: Vec::new(),
        }
    }
    /// Add an extension directive.
    pub fn add_extension(&mut self, ext: impl Into<String>) {
        self.extensions.push(ext.into());
    }
    /// Add a `#define` macro without a value.
    pub fn add_define(&mut self, name: impl Into<String>) {
        self.defines.push((name.into(), None));
    }
    /// Add a `#define` macro with a value.
    pub fn add_define_value(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.defines.push((name.into(), Some(value.into())));
    }
    /// Add a struct definition.
    pub fn add_struct(&mut self, s: GLSLStruct) {
        self.structs.push(s);
    }
    /// Add a stage input.
    pub fn add_input(&mut self, v: GLSLVariable) {
        self.inputs.push(v);
    }
    /// Add a stage output.
    pub fn add_output(&mut self, v: GLSLVariable) {
        self.outputs.push(v);
    }
    /// Add a uniform variable.
    pub fn add_uniform(&mut self, v: GLSLVariable) {
        self.uniforms.push(v);
    }
    /// Add a global variable.
    pub fn add_global(&mut self, v: GLSLVariable) {
        self.globals.push(v);
    }
    /// Add a helper function.
    pub fn add_function(&mut self, f: GLSLFunction) {
        self.functions.push(f);
    }
    /// Append a statement to `main`.
    pub fn add_main_statement(&mut self, stmt: impl Into<String>) {
        self.main_body.push(stmt.into());
    }
}
/// Analysis cache for GLSLExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct GLSLExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl GLSLExtCache {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GLSLPassConfig {
    pub phase: GLSLPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl GLSLPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: GLSLPassPhase) -> Self {
        GLSLPassConfig {
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
pub struct GLSLLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl GLSLLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        GLSLLivenessInfo {
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
/// Code generation backend that emits GLSL source text.
pub struct GLSLBackend {
    /// Target GLSL version.
    pub version: GLSLVersion,
}
impl GLSLBackend {
    /// Create a new backend targeting the specified GLSL version.
    pub fn new(version: GLSLVersion) -> Self {
        GLSLBackend { version }
    }
    /// Return the GLSL keyword string for a type.
    pub fn emit_type(&self, ty: &GLSLType) -> String {
        ty.keyword()
    }
    /// Emit a complete GLSL shader as a source string.
    pub fn emit_shader(&self, shader: &GLSLShader) -> String {
        let mut out = String::new();
        out.push_str(&shader.version.version_line());
        out.push('\n');
        for ext in &shader.extensions {
            out.push_str(&format!("#extension {} : enable\n", ext));
        }
        if !shader.extensions.is_empty() {
            out.push('\n');
        }
        for (name, val) in &shader.defines {
            match val {
                Some(v) => out.push_str(&format!("#define {} {}\n", name, v)),
                None => out.push_str(&format!("#define {}\n", name)),
            }
        }
        if !shader.defines.is_empty() {
            out.push('\n');
        }
        for s in &shader.structs {
            out.push_str(&s.emit());
            out.push_str("\n\n");
        }
        for v in &shader.inputs {
            out.push_str(&v.emit_global());
            out.push('\n');
        }
        if !shader.inputs.is_empty() {
            out.push('\n');
        }
        for v in &shader.outputs {
            out.push_str(&v.emit_global());
            out.push('\n');
        }
        if !shader.outputs.is_empty() {
            out.push('\n');
        }
        for v in &shader.uniforms {
            out.push_str(&v.emit_global());
            out.push('\n');
        }
        if !shader.uniforms.is_empty() {
            out.push('\n');
        }
        for v in &shader.globals {
            out.push_str(&v.emit_global());
            out.push('\n');
        }
        if !shader.globals.is_empty() {
            out.push('\n');
        }
        for f in &shader.functions {
            out.push_str(&f.emit());
            out.push_str("\n\n");
        }
        out.push_str("void main() {\n");
        for stmt in &shader.main_body {
            out.push_str(&format!("    {};\n", stmt));
        }
        out.push_str("}\n");
        out
    }
    /// Build a minimal vertex shader template that passes a position through.
    pub fn vertex_shader_template(&self) -> String {
        let mut shader = GLSLShader::new(self.version, GLSLShaderStage::Vertex);
        if self.version.supports_layout_location() {
            shader.add_input(GLSLVariable::layout_input("aPosition", GLSLType::Vec3, 0));
            shader.add_input(GLSLVariable::layout_input("aTexCoord", GLSLType::Vec2, 1));
            shader.add_output(GLSLVariable::layout_output("vTexCoord", GLSLType::Vec2, 0));
        } else {
            shader.add_input(GLSLVariable::input("aPosition", GLSLType::Vec3));
            shader.add_input(GLSLVariable::input("aTexCoord", GLSLType::Vec2));
            shader.add_output(GLSLVariable::output("vTexCoord", GLSLType::Vec2));
        }
        shader.add_uniform(GLSLVariable::uniform("uMVP", GLSLType::Mat4));
        shader.add_main_statement("gl_Position = uMVP * vec4(aPosition, 1.0)");
        shader.add_main_statement("vTexCoord = aTexCoord");
        self.emit_shader(&shader)
    }
    /// Build a minimal fragment (pixel) shader template that samples a texture.
    pub fn fragment_shader_template(&self) -> String {
        let mut shader = GLSLShader::new(self.version, GLSLShaderStage::Fragment);
        if self.version.supports_layout_location() {
            shader.add_input(GLSLVariable::layout_input("vTexCoord", GLSLType::Vec2, 0));
            shader.add_output(GLSLVariable::layout_output("fragColor", GLSLType::Vec4, 0));
        } else {
            shader.add_input(GLSLVariable::input("vTexCoord", GLSLType::Vec2));
            shader.add_output(GLSLVariable::output("fragColor", GLSLType::Vec4));
        }
        shader.add_uniform(GLSLVariable::uniform("uTexture", GLSLType::Sampler2D));
        shader.add_main_statement("fragColor = texture(uTexture, vTexCoord)");
        self.emit_shader(&shader)
    }
    /// Build a Phong lighting vertex shader template.
    pub fn phong_vertex_template(&self) -> String {
        let mut shader = GLSLShader::new(self.version, GLSLShaderStage::Vertex);
        shader.add_input(GLSLVariable::layout_input("aPosition", GLSLType::Vec3, 0));
        shader.add_input(GLSLVariable::layout_input("aNormal", GLSLType::Vec3, 1));
        shader.add_output(GLSLVariable::layout_output("vNormal", GLSLType::Vec3, 0));
        shader.add_output(GLSLVariable::layout_output("vFragPos", GLSLType::Vec3, 1));
        shader.add_uniform(GLSLVariable::uniform("uModel", GLSLType::Mat4));
        shader.add_uniform(GLSLVariable::uniform("uView", GLSLType::Mat4));
        shader.add_uniform(GLSLVariable::uniform("uProjection", GLSLType::Mat4));
        shader.add_main_statement("vFragPos = vec3(uModel * vec4(aPosition, 1.0))");
        shader.add_main_statement("vNormal = mat3(transpose(inverse(uModel))) * aNormal");
        shader.add_main_statement("gl_Position = uProjection * uView * vec4(vFragPos, 1.0)");
        self.emit_shader(&shader)
    }
    /// Build a Phong lighting fragment shader template.
    pub fn phong_fragment_template(&self) -> String {
        let mut shader = GLSLShader::new(self.version, GLSLShaderStage::Fragment);
        shader.add_input(GLSLVariable::layout_input("vNormal", GLSLType::Vec3, 0));
        shader.add_input(GLSLVariable::layout_input("vFragPos", GLSLType::Vec3, 1));
        shader.add_output(GLSLVariable::layout_output("fragColor", GLSLType::Vec4, 0));
        shader.add_uniform(GLSLVariable::uniform("uLightPos", GLSLType::Vec3));
        shader.add_uniform(GLSLVariable::uniform("uViewPos", GLSLType::Vec3));
        shader.add_uniform(GLSLVariable::uniform("uLightColor", GLSLType::Vec3));
        shader.add_uniform(GLSLVariable::uniform("uObjectColor", GLSLType::Vec3));
        shader.add_main_statement("float ambientStrength = 0.1");
        shader.add_main_statement("vec3 ambient = ambientStrength * uLightColor");
        shader.add_main_statement("vec3 norm = normalize(vNormal)");
        shader.add_main_statement("vec3 lightDir = normalize(uLightPos - vFragPos)");
        shader.add_main_statement("float diff = max(dot(norm, lightDir), 0.0)");
        shader.add_main_statement("vec3 diffuse = diff * uLightColor");
        shader.add_main_statement("float specularStrength = 0.5");
        shader.add_main_statement("vec3 viewDir = normalize(uViewPos - vFragPos)");
        shader.add_main_statement("vec3 reflectDir = reflect(-lightDir, norm)");
        shader.add_main_statement("float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0)");
        shader.add_main_statement("vec3 specular = specularStrength * spec * uLightColor");
        shader.add_main_statement("vec3 result = (ambient + diffuse + specular) * uObjectColor");
        shader.add_main_statement("fragColor = vec4(result, 1.0)");
        self.emit_shader(&shader)
    }
    /// Build a compute shader template (GLSL 4.3+).
    pub fn compute_shader_template(&self, local_x: u32, local_y: u32, local_z: u32) -> String {
        let mut shader = GLSLShader::new(self.version, GLSLShaderStage::Compute);
        shader.add_extension("GL_ARB_compute_shader");
        shader.add_main_statement("uint idx = gl_GlobalInvocationID.x");
        shader.add_main_statement("// TODO: compute work here");
        let mut out = self.emit_shader(&shader);
        let layout_line = format!(
            "layout(local_size_x = {}, local_size_y = {}, local_size_z = {}) in;\n",
            local_x, local_y, local_z
        );
        if let Some(pos) = out.find('\n') {
            out.insert_str(pos + 1, &layout_line);
        }
        out
    }
}
/// Configuration for a GLSL compute shader work-group.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlslComputeWorkgroup {
    pub local_size_x: u32,
    pub local_size_y: u32,
    pub local_size_z: u32,
}
