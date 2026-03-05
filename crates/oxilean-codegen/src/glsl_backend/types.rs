//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

/// std140 / std430 layout for UBOs.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlslBlockLayout {
    Std140,
    Std430,
    Shared,
}
impl GlslBlockLayout {
    /// Layout qualifier string.
    pub fn qualifier_str(self) -> &'static str {
        match self {
            GlslBlockLayout::Std140 => "std140",
            GlslBlockLayout::Std430 => "std430",
            GlslBlockLayout::Shared => "shared",
        }
    }
}
/// Statistics for GLSLExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GLSLExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl GLSLExtPassStats {
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
    pub fn merge(&mut self, o: &GLSLExtPassStats) {
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
pub struct GLSLConstantFoldingHelper;
impl GLSLConstantFoldingHelper {
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
/// A member of a GLSL uniform block.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GlslBlockMember {
    pub name: String,
    pub ty: GLSLType,
    pub array_size: Option<usize>,
}
#[allow(dead_code)]
impl GlslBlockMember {
    pub fn new(name: impl Into<String>, ty: GLSLType) -> Self {
        GlslBlockMember {
            name: name.into(),
            ty,
            array_size: None,
        }
    }
    pub fn array(name: impl Into<String>, ty: GLSLType, size: usize) -> Self {
        GlslBlockMember {
            name: name.into(),
            ty,
            array_size: Some(size),
        }
    }
}
#[allow(dead_code)]
pub struct GLSLPassRegistry {
    pub(super) configs: Vec<GLSLPassConfig>,
    pub(super) stats: std::collections::HashMap<String, GLSLPassStats>,
}
impl GLSLPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GLSLPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: GLSLPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), GLSLPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&GLSLPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&GLSLPassStats> {
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
pub struct GLSLDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl GLSLDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        GLSLDominatorTree {
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
/// A single field inside a GLSL `struct` definition.
#[derive(Debug, Clone)]
pub struct GLSLStructField {
    /// Field name.
    pub name: String,
    /// Field type.
    pub ty: GLSLType,
}
impl GLSLStructField {
    /// Create a new struct field.
    pub fn new(name: impl Into<String>, ty: GLSLType) -> Self {
        GLSLStructField {
            name: name.into(),
            ty,
        }
    }
}
/// GLSL type system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GLSLType {
    /// `void`
    Void,
    /// `bool`
    Bool,
    /// `int`
    Int,
    /// `uint`
    Uint,
    /// `float`
    Float,
    /// `double`
    Double,
    /// `vec2`
    Vec2,
    /// `vec3`
    Vec3,
    /// `vec4`
    Vec4,
    /// `ivec2`
    IVec2,
    /// `ivec3`
    IVec3,
    /// `ivec4`
    IVec4,
    /// `uvec2`
    UVec2,
    /// `uvec3`
    UVec3,
    /// `uvec4`
    UVec4,
    /// `bvec2`
    BVec2,
    /// `bvec3`
    BVec3,
    /// `bvec4`
    BVec4,
    /// `mat2`
    Mat2,
    /// `mat3`
    Mat3,
    /// `mat4`
    Mat4,
    /// `mat2x3`
    Mat2x3,
    /// `mat2x4`
    Mat2x4,
    /// `mat3x2`
    Mat3x2,
    /// `mat3x4`
    Mat3x4,
    /// `mat4x2`
    Mat4x2,
    /// `mat4x3`
    Mat4x3,
    /// `dmat4`
    DMat4,
    /// `sampler2D`
    Sampler2D,
    /// `sampler3D`
    Sampler3D,
    /// `samplerCube`
    SamplerCube,
    /// `sampler2DArray`
    Sampler2DArray,
    /// `sampler2DShadow`
    Sampler2DShadow,
    /// `isampler2D`
    ISampler2D,
    /// `usampler2D`
    USampler2D,
    /// `image2D`
    Image2D,
    /// `uimage2D`
    UImage2D,
    /// A named struct type.
    Struct(String),
    /// A fixed-length array of another type.
    Array(Box<GLSLType>, u32),
}
impl GLSLType {
    /// Return the GLSL keyword for this type.
    pub fn keyword(&self) -> String {
        match self {
            GLSLType::Void => "void".into(),
            GLSLType::Bool => "bool".into(),
            GLSLType::Int => "int".into(),
            GLSLType::Uint => "uint".into(),
            GLSLType::Float => "float".into(),
            GLSLType::Double => "double".into(),
            GLSLType::Vec2 => "vec2".into(),
            GLSLType::Vec3 => "vec3".into(),
            GLSLType::Vec4 => "vec4".into(),
            GLSLType::IVec2 => "ivec2".into(),
            GLSLType::IVec3 => "ivec3".into(),
            GLSLType::IVec4 => "ivec4".into(),
            GLSLType::UVec2 => "uvec2".into(),
            GLSLType::UVec3 => "uvec3".into(),
            GLSLType::UVec4 => "uvec4".into(),
            GLSLType::BVec2 => "bvec2".into(),
            GLSLType::BVec3 => "bvec3".into(),
            GLSLType::BVec4 => "bvec4".into(),
            GLSLType::Mat2 => "mat2".into(),
            GLSLType::Mat3 => "mat3".into(),
            GLSLType::Mat4 => "mat4".into(),
            GLSLType::Mat2x3 => "mat2x3".into(),
            GLSLType::Mat2x4 => "mat2x4".into(),
            GLSLType::Mat3x2 => "mat3x2".into(),
            GLSLType::Mat3x4 => "mat3x4".into(),
            GLSLType::Mat4x2 => "mat4x2".into(),
            GLSLType::Mat4x3 => "mat4x3".into(),
            GLSLType::DMat4 => "dmat4".into(),
            GLSLType::Sampler2D => "sampler2D".into(),
            GLSLType::Sampler3D => "sampler3D".into(),
            GLSLType::SamplerCube => "samplerCube".into(),
            GLSLType::Sampler2DArray => "sampler2DArray".into(),
            GLSLType::Sampler2DShadow => "sampler2DShadow".into(),
            GLSLType::ISampler2D => "isampler2D".into(),
            GLSLType::USampler2D => "usampler2D".into(),
            GLSLType::Image2D => "image2D".into(),
            GLSLType::UImage2D => "uimage2D".into(),
            GLSLType::Struct(name) => name.clone(),
            GLSLType::Array(elem, _) => elem.keyword(),
        }
    }
    /// Return the component (column) count for vector/matrix types, or 1.
    pub fn component_count(&self) -> u32 {
        match self {
            GLSLType::Vec2 | GLSLType::IVec2 | GLSLType::UVec2 | GLSLType::BVec2 => 2,
            GLSLType::Vec3 | GLSLType::IVec3 | GLSLType::UVec3 | GLSLType::BVec3 => 3,
            GLSLType::Vec4 | GLSLType::IVec4 | GLSLType::UVec4 | GLSLType::BVec4 => 4,
            GLSLType::Mat2 | GLSLType::Mat2x3 | GLSLType::Mat2x4 => 2,
            GLSLType::Mat3 | GLSLType::Mat3x2 | GLSLType::Mat3x4 => 3,
            GLSLType::Mat4 | GLSLType::Mat4x2 | GLSLType::Mat4x3 | GLSLType::DMat4 => 4,
            _ => 1,
        }
    }
    /// Return true if the type is a sampler or image type.
    pub fn is_opaque(&self) -> bool {
        matches!(
            self,
            GLSLType::Sampler2D
                | GLSLType::Sampler3D
                | GLSLType::SamplerCube
                | GLSLType::Sampler2DArray
                | GLSLType::Sampler2DShadow
                | GLSLType::ISampler2D
                | GLSLType::USampler2D
                | GLSLType::Image2D
                | GLSLType::UImage2D
        )
    }
    /// Return true if the type is a floating-point scalar or vector.
    pub fn is_float_like(&self) -> bool {
        matches!(
            self,
            GLSLType::Float | GLSLType::Double | GLSLType::Vec2 | GLSLType::Vec3 | GLSLType::Vec4
        )
    }
}
/// A simple type-inference context for GLSL expression analysis.
#[allow(dead_code)]
pub struct GlslTypeInference {
    pub(super) bindings: std::collections::HashMap<String, GLSLType>,
    pub(super) version: GLSLVersion,
}
#[allow(dead_code)]
impl GlslTypeInference {
    /// Create a new inference context for the given GLSL version.
    pub fn new(version: GLSLVersion) -> Self {
        GlslTypeInference {
            bindings: std::collections::HashMap::new(),
            version,
        }
    }
    /// Bind a name to a GLSL type.
    pub fn bind(&mut self, name: impl Into<String>, ty: GLSLType) {
        self.bindings.insert(name.into(), ty);
    }
    /// Look up the type of a name.
    pub fn lookup(&self, name: &str) -> Option<&GLSLType> {
        self.bindings.get(name)
    }
    /// Component count of a vector/matrix type.
    pub fn num_components(ty: &GLSLType) -> usize {
        match ty {
            GLSLType::Vec2 | GLSLType::IVec2 | GLSLType::BVec2 => 2,
            GLSLType::Vec3 | GLSLType::IVec3 | GLSLType::BVec3 => 3,
            GLSLType::Vec4 | GLSLType::IVec4 | GLSLType::BVec4 => 4,
            GLSLType::Mat2 => 4,
            GLSLType::Mat3 => 9,
            GLSLType::Mat4 => 16,
            _ => 1,
        }
    }
    /// Whether a type is available in this version.
    pub fn type_available(&self, ty: &GLSLType) -> bool {
        match ty {
            GLSLType::Uint => self.version.supports_uint(),
            _ => true,
        }
    }
    /// Return the version.
    pub fn version(&self) -> GLSLVersion {
        self.version
    }
    /// Number of active bindings.
    pub fn num_bindings(&self) -> usize {
        self.bindings.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GLSLPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl GLSLPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            GLSLPassPhase::Analysis => "analysis",
            GLSLPassPhase::Transformation => "transformation",
            GLSLPassPhase::Verification => "verification",
            GLSLPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, GLSLPassPhase::Transformation | GLSLPassPhase::Cleanup)
    }
}
/// Worklist for GLSLExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GLSLExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl GLSLExtWorklist {
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
/// Constant folding helper for GLSLExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GLSLExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl GLSLExtConstFolder {
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
/// A simple GLSL macro expander.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GlslMacroExpander {
    pub(super) defines: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl GlslMacroExpander {
    pub fn new() -> Self {
        GlslMacroExpander {
            defines: std::collections::HashMap::new(),
        }
    }
    pub fn define(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.defines.insert(name.into(), value.into());
    }
    pub fn undef(&mut self, name: &str) {
        self.defines.remove(name);
    }
    pub fn is_defined(&self, name: &str) -> bool {
        self.defines.contains_key(name)
    }
    pub fn value(&self, name: &str) -> Option<&str> {
        self.defines.get(name).map(|s| s.as_str())
    }
    pub fn emit_defines(&self) -> String {
        let mut names: Vec<&String> = self.defines.keys().collect();
        names.sort();
        names
            .iter()
            .map(|name| {
                let val = &self.defines[*name];
                if val.is_empty() {
                    format!("#define {}\n", name)
                } else {
                    format!("#define {} {}\n", name, val)
                }
            })
            .collect()
    }
    pub fn num_defines(&self) -> usize {
        self.defines.len()
    }
}
/// A GLSL `struct` type definition.
#[derive(Debug, Clone)]
pub struct GLSLStruct {
    /// Struct name.
    pub name: String,
    /// Fields in declaration order.
    pub fields: Vec<GLSLStructField>,
}
impl GLSLStruct {
    /// Create a new empty struct.
    pub fn new(name: impl Into<String>) -> Self {
        GLSLStruct {
            name: name.into(),
            fields: Vec::new(),
        }
    }
    /// Add a field.
    pub fn add_field(&mut self, name: impl Into<String>, ty: GLSLType) {
        self.fields.push(GLSLStructField::new(name, ty));
    }
    /// Emit the struct definition.
    pub fn emit(&self) -> String {
        let mut out = format!("struct {} {{\n", self.name);
        for f in &self.fields {
            out.push_str(&format!("    {} {};\n", f.ty, f.name));
        }
        out.push_str("};");
        out
    }
}
/// A GLSL function definition.
#[derive(Debug, Clone)]
pub struct GLSLFunction {
    /// Function name.
    pub name: String,
    /// Return type.
    pub return_type: GLSLType,
    /// Ordered list of parameters.
    pub params: Vec<GLSLVariable>,
    /// Body statements (each is emitted on its own line with indentation).
    pub body: Vec<String>,
}
impl GLSLFunction {
    /// Create a new function with an empty body.
    pub fn new(name: impl Into<String>, return_type: GLSLType) -> Self {
        GLSLFunction {
            name: name.into(),
            return_type,
            params: Vec::new(),
            body: Vec::new(),
        }
    }
    /// Add a parameter.
    pub fn add_param(&mut self, var: GLSLVariable) {
        self.params.push(var);
    }
    /// Append a statement to the body.
    pub fn add_statement(&mut self, stmt: impl Into<String>) {
        self.body.push(stmt.into());
    }
    /// Emit the full function definition.
    pub fn emit(&self) -> String {
        let params: Vec<String> = self.params.iter().map(|p| p.emit_param()).collect();
        let mut out = format!(
            "{} {}({}) {{\n",
            self.return_type,
            self.name,
            params.join(", ")
        );
        for stmt in &self.body {
            out.push_str(&format!("    {};\n", stmt));
        }
        out.push('}');
        out
    }
    /// Emit the forward declaration (prototype).
    pub fn emit_prototype(&self) -> String {
        let params: Vec<String> = self.params.iter().map(|p| p.emit_param()).collect();
        format!("{} {}({});", self.return_type, self.name, params.join(", "))
    }
}
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
    pub(super) vars: Vec<(String, GLSLType, Option<u32>)>,
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
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
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
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
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
    pub(super) const fn new(name: &'static str, cat: GlslBuiltinCategory, ver: u32) -> Self {
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
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
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
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
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
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
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
    pub(super) configs: Vec<GLSLExtPassConfig>,
    pub(super) stats: Vec<GLSLExtPassStats>,
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
    pub(super) entries: std::collections::HashMap<String, GLSLCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
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
