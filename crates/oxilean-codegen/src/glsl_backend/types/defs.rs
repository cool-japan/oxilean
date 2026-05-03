//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::impls1::*;
use super::impls2::*;
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
    pub(crate) configs: Vec<GLSLPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, GLSLPassStats>,
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
    pub(crate) bindings: std::collections::HashMap<String, GLSLType>,
    pub(crate) version: GLSLVersion,
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
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
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
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
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
    pub(crate) defines: std::collections::HashMap<String, String>,
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
