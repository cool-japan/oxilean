//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls1::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// std140 / std430 layout for UBOs.
#[allow(dead_code)]
impl GlslComputeWorkgroup {
    pub fn linear(x: u32) -> Self {
        GlslComputeWorkgroup {
            local_size_x: x,
            local_size_y: 1,
            local_size_z: 1,
        }
    }
    pub fn planar(x: u32, y: u32) -> Self {
        GlslComputeWorkgroup {
            local_size_x: x,
            local_size_y: y,
            local_size_z: 1,
        }
    }
    pub fn volumetric(x: u32, y: u32, z: u32) -> Self {
        GlslComputeWorkgroup {
            local_size_x: x,
            local_size_y: y,
            local_size_z: z,
        }
    }
    pub fn total_threads(&self) -> u32 {
        self.local_size_x * self.local_size_y * self.local_size_z
    }
    pub fn emit_layout(&self) -> String {
        format!(
            "layout(local_size_x = {}, local_size_y = {}, local_size_z = {}) in;\n",
            self.local_size_x, self.local_size_y, self.local_size_z
        )
    }
}
/// Dominator tree for GLSLExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GLSLExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl GLSLExtDomTree {
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
/// Generates include guards for GLSL headers.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GlslIncludeGuard {
    pub macro_name: String,
}
#[allow(dead_code)]
impl GlslIncludeGuard {
    pub fn from_filename(filename: &str) -> Self {
        let macro_name = filename
            .replace('.', "_")
            .replace('/', "_")
            .replace('-', "_")
            .to_uppercase();
        GlslIncludeGuard {
            macro_name: format!("{}_GUARD", macro_name),
        }
    }
    pub fn open(&self) -> String {
        format!("#ifndef {0}\n#define {0}\n", self.macro_name)
    }
    pub fn close(&self) -> String {
        format!("#endif // {}\n", self.macro_name)
    }
}
/// Dependency graph for GLSLExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GLSLExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}
impl GLSLExtDepGraph {
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
/// The pipeline stage that a GLSL shader implements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GLSLShaderStage {
    /// Vertex shader — runs once per vertex.
    Vertex,
    /// Fragment (pixel) shader — runs once per fragment.
    Fragment,
    /// Geometry shader — runs once per primitive.
    Geometry,
    /// Tessellation control shader.
    TessControl,
    /// Tessellation evaluation shader.
    TessEval,
    /// Compute shader (GLSL 4.3+).
    Compute,
}
/// Storage / parameter qualifier for a GLSL variable declaration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GLSLQualifier {
    /// No qualifier (local variable inside a function).
    None,
    /// `in` — stage input.
    In,
    /// `out` — stage output.
    Out,
    /// `inout` — read/write function parameter.
    InOut,
    /// `uniform` — uniform variable (set from the host).
    Uniform,
    /// `const` — compile-time constant.
    Const,
    /// `flat` — flat interpolation.
    Flat,
    /// `centroid in` — centroid-interpolated input.
    CentroidIn,
    /// `centroid out` — centroid-interpolated output.
    CentroidOut,
    /// `layout(…) in` with a custom layout string.
    LayoutIn(String),
    /// `layout(…) out` with a custom layout string.
    LayoutOut(String),
    /// `layout(…) uniform` with a custom layout string.
    LayoutUniform(String),
}
impl GLSLQualifier {
    /// Emit the qualifier tokens that precede the type name.
    pub fn prefix(&self) -> String {
        match self {
            GLSLQualifier::None => String::new(),
            GLSLQualifier::In => "in ".into(),
            GLSLQualifier::Out => "out ".into(),
            GLSLQualifier::InOut => "inout ".into(),
            GLSLQualifier::Uniform => "uniform ".into(),
            GLSLQualifier::Const => "const ".into(),
            GLSLQualifier::Flat => "flat in ".into(),
            GLSLQualifier::CentroidIn => "centroid in ".into(),
            GLSLQualifier::CentroidOut => "centroid out ".into(),
            GLSLQualifier::LayoutIn(l) => format!("layout({}) in ", l),
            GLSLQualifier::LayoutOut(l) => format!("layout({}) out ", l),
            GLSLQualifier::LayoutUniform(l) => format!("layout({}) uniform ", l),
        }
    }
}
/// A GLSL uniform buffer object.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GlslUniformBlock {
    pub block_name: String,
    pub instance_name: Option<String>,
    pub layout: GlslBlockLayout,
    pub binding: Option<u32>,
    pub members: Vec<GlslBlockMember>,
    pub is_ssbo: bool,
}
#[allow(dead_code)]
impl GlslUniformBlock {
    pub fn ubo(block_name: impl Into<String>) -> Self {
        GlslUniformBlock {
            block_name: block_name.into(),
            instance_name: None,
            layout: GlslBlockLayout::Std140,
            binding: None,
            members: Vec::new(),
            is_ssbo: false,
        }
    }
    pub fn with_binding(mut self, b: u32) -> Self {
        self.binding = Some(b);
        self
    }
    pub fn add_member(&mut self, m: GlslBlockMember) {
        self.members.push(m);
    }
    pub fn num_members(&self) -> usize {
        self.members.len()
    }
    pub fn emit(&self) -> String {
        let layout_str = self.layout.qualifier_str();
        let mut out = if let Some(b) = self.binding {
            format!("layout({}, binding = {}) ", layout_str, b)
        } else {
            format!("layout({}) ", layout_str)
        };
        let kw = if self.is_ssbo { "buffer" } else { "uniform" };
        out.push_str(&format!("{} {} {{\n", kw, self.block_name));
        for m in &self.members {
            let ty_str = m.ty.keyword();
            if let Some(sz) = m.array_size {
                out.push_str(&format!("    {} {}[{}];\n", ty_str, m.name, sz));
            } else {
                out.push_str(&format!("    {} {};\n", ty_str, m.name));
            }
        }
        if let Some(ref inst) = self.instance_name {
            out.push_str(&format!("}} {};\n", inst));
        } else {
            out.push_str("};\n");
        }
        out
    }
}
/// Supported GLSL version targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GLSLVersion {
    /// GLSL 1.20 — OpenGL 2.1 (legacy)
    V120,
    /// GLSL 3.30 — OpenGL 3.3 (core profile, widespread)
    V330,
    /// GLSL 4.50 — OpenGL 4.5 (DSA, direct state access era)
    V450,
    /// GLSL 4.60 — OpenGL 4.6 / SPIR-V interop
    V460,
}
impl GLSLVersion {
    /// Return the integer version number used in `#version` directives.
    pub fn number(self) -> u32 {
        match self {
            GLSLVersion::V120 => 120,
            GLSLVersion::V330 => 330,
            GLSLVersion::V450 => 450,
            GLSLVersion::V460 => 460,
        }
    }
    /// Return the profile string appended after the version number.
    ///
    /// GLSL 1.20 predates the core/compatibility split; later versions
    /// default to `core`.
    pub fn profile(self) -> &'static str {
        match self {
            GLSLVersion::V120 => "",
            _ => " core",
        }
    }
    /// Emit the full `#version` line.
    pub fn version_line(self) -> String {
        format!("#version {}{}", self.number(), self.profile())
    }
    /// Whether this version supports explicit `layout(location = …)` qualifiers.
    pub fn supports_layout_location(self) -> bool {
        self.number() >= 330
    }
    /// Whether this version supports compute shaders.
    pub fn supports_compute(self) -> bool {
        self.number() >= 430
    }
    /// Whether this version supports `uint` / `uvec*` types.
    pub fn supports_uint(self) -> bool {
        self.number() >= 130
    }
}
/// GLSL precision qualifier (relevant for mediump / highp / lowp).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GLSLPrecision {
    Low,
    Medium,
    High,
}
/// Pass registry for GLSLExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GLSLExtPassRegistry {
    pub(crate) configs: Vec<GLSLExtPassConfig>,
    pub(crate) stats: Vec<GLSLExtPassStats>,
}
impl GLSLExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: GLSLExtPassConfig) {
        self.stats.push(GLSLExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&GLSLExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&GLSLExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&GLSLExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &GLSLExtPassPhase) -> Vec<&GLSLExtPassConfig> {
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
/// Category of a GLSL built-in function.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlslBuiltinCategory {
    Trigonometric,
    Exponential,
    Common,
    GeometricVector,
    MatrixOp,
    TextureSampling,
    Atomic,
}
/// A simple GLSL expression, sufficient for code-generation purposes.
#[derive(Debug, Clone)]
pub enum GLSLExpr {
    /// A literal number or boolean token.
    Literal(String),
    /// A variable reference.
    Var(String),
    /// A binary infix operation.
    BinOp {
        op: String,
        lhs: Box<GLSLExpr>,
        rhs: Box<GLSLExpr>,
    },
    /// A unary prefix operation.
    UnaryOp { op: String, operand: Box<GLSLExpr> },
    /// A function/constructor call.
    Call { func: String, args: Vec<GLSLExpr> },
    /// A field access (e.g. `v.xy`).
    Field { base: Box<GLSLExpr>, field: String },
    /// An array index access.
    Index {
        base: Box<GLSLExpr>,
        index: Box<GLSLExpr>,
    },
    /// A ternary `cond ? then : else` expression.
    Ternary {
        cond: Box<GLSLExpr>,
        then_expr: Box<GLSLExpr>,
        else_expr: Box<GLSLExpr>,
    },
}
impl GLSLExpr {
    /// Emit this expression as a GLSL source string.
    pub fn emit(&self) -> String {
        match self {
            GLSLExpr::Literal(s) => s.clone(),
            GLSLExpr::Var(name) => name.clone(),
            GLSLExpr::BinOp { op, lhs, rhs } => {
                format!("({} {} {})", lhs.emit(), op, rhs.emit())
            }
            GLSLExpr::UnaryOp { op, operand } => format!("({}{})", op, operand.emit()),
            GLSLExpr::Call { func, args } => {
                let arg_strs: Vec<String> = args.iter().map(|a| a.emit()).collect();
                format!("{}({})", func, arg_strs.join(", "))
            }
            GLSLExpr::Field { base, field } => format!("{}.{}", base.emit(), field),
            GLSLExpr::Index { base, index } => {
                format!("{}[{}]", base.emit(), index.emit())
            }
            GLSLExpr::Ternary {
                cond,
                then_expr,
                else_expr,
            } => {
                format!(
                    "({} ? {} : {})",
                    cond.emit(),
                    then_expr.emit(),
                    else_expr.emit()
                )
            }
        }
    }
    /// Convenience: build a binary operation.
    pub fn binop(op: impl Into<String>, lhs: GLSLExpr, rhs: GLSLExpr) -> Self {
        GLSLExpr::BinOp {
            op: op.into(),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
    /// Convenience: build a function/constructor call.
    pub fn call(func: impl Into<String>, args: Vec<GLSLExpr>) -> Self {
        GLSLExpr::Call {
            func: func.into(),
            args,
        }
    }
    /// Convenience: build a variable expression.
    pub fn var(name: impl Into<String>) -> Self {
        GLSLExpr::Var(name.into())
    }
    /// Convenience: build a float literal.
    pub fn float(v: f32) -> Self {
        GLSLExpr::Literal(format!("{:.6}", v))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GLSLAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, GLSLCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}
impl GLSLAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        GLSLAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&GLSLCacheEntry> {
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
            GLSLCacheEntry {
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
/// A single GLSL variable (global or function parameter).
#[derive(Debug, Clone)]
pub struct GLSLVariable {
    /// Variable name.
    pub name: String,
    /// GLSL type.
    pub ty: GLSLType,
    /// Storage / parameter qualifier.
    pub qualifier: GLSLQualifier,
    /// Optional initialiser expression (e.g. for `const` variables).
    pub initializer: Option<String>,
}
impl GLSLVariable {
    /// Create a new variable with the given name, type, and qualifier.
    pub fn new(name: impl Into<String>, ty: GLSLType, qualifier: GLSLQualifier) -> Self {
        GLSLVariable {
            name: name.into(),
            ty,
            qualifier,
            initializer: None,
        }
    }
    /// Create a uniform variable.
    pub fn uniform(name: impl Into<String>, ty: GLSLType) -> Self {
        Self::new(name, ty, GLSLQualifier::Uniform)
    }
    /// Create a stage input variable.
    pub fn input(name: impl Into<String>, ty: GLSLType) -> Self {
        Self::new(name, ty, GLSLQualifier::In)
    }
    /// Create a stage output variable.
    pub fn output(name: impl Into<String>, ty: GLSLType) -> Self {
        Self::new(name, ty, GLSLQualifier::Out)
    }
    /// Create a layout-qualified input variable with the given location index.
    pub fn layout_input(name: impl Into<String>, ty: GLSLType, location: u32) -> Self {
        Self::new(
            name,
            ty,
            GLSLQualifier::LayoutIn(format!("location = {}", location)),
        )
    }
    /// Create a layout-qualified output variable with the given location index.
    pub fn layout_output(name: impl Into<String>, ty: GLSLType, location: u32) -> Self {
        Self::new(
            name,
            ty,
            GLSLQualifier::LayoutOut(format!("location = {}", location)),
        )
    }
    /// Emit the declaration as a top-level global statement.
    pub fn emit_global(&self) -> String {
        match &self.initializer {
            Some(init) => {
                format!(
                    "{}{} {} = {};",
                    self.qualifier.prefix(),
                    self.ty,
                    self.name,
                    init
                )
            }
            None => format!("{}{} {};", self.qualifier.prefix(), self.ty, self.name),
        }
    }
    /// Emit the declaration as a function parameter (no semicolon, no initializer).
    pub fn emit_param(&self) -> String {
        format!("{}{} {}", self.qualifier.prefix(), self.ty, self.name)
    }
}
/// Validates GLSL swizzle expressions.
#[allow(dead_code)]
pub struct GlslSwizzleValidator;
impl GlslSwizzleValidator {
    /// Validate a swizzle mask for a vector of `components` elements.
    pub fn validate(mask: &str, components: usize) -> Result<usize, String> {
        if mask.is_empty() || mask.len() > 4 {
            return Err(format!(
                "swizzle mask length {} is not in [1,4]",
                mask.len()
            ));
        }
        let xyzw = ['x', 'y', 'z', 'w'];
        let rgba = ['r', 'g', 'b', 'a'];
        let stpq = ['s', 't', 'p', 'q'];
        let first = mask
            .chars()
            .next()
            .expect("mask is non-empty; checked at function entry");
        let set: &[char] = if xyzw.contains(&first) {
            &xyzw[..components]
        } else if rgba.contains(&first) {
            &rgba[..components]
        } else if stpq.contains(&first) {
            &stpq[..components]
        } else {
            return Err(format!("unknown swizzle character '{}'", first));
        };
        for ch in mask.chars() {
            if !set.contains(&ch) {
                return Err(format!(
                    "swizzle char '{}' out of range for {}-component vector",
                    ch, components
                ));
            }
        }
        Ok(mask.len())
    }
}
/// Configuration for GLSLExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GLSLExtPassConfig {
    pub name: String,
    pub phase: GLSLExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl GLSLExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: GLSLExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: GLSLExtPassPhase) -> Self {
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
