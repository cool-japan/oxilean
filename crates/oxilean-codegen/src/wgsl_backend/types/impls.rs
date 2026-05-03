//! Auto-generated module (split from types.rs)
//!
//! Second half of type definitions and impl blocks.

use super::defs::*;

/// Configuration for a render pipeline.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WGSLRenderPipeline {
    /// Name prefix for vertex/fragment entry points.
    pub name: String,
    /// Vertex input struct name.
    pub vertex_input: String,
    /// Vertex output / fragment input struct name.
    pub varying: String,
    /// Vertex shader body statements.
    pub vs_body: Vec<String>,
    /// Fragment shader body statements.
    pub fs_body: Vec<String>,
    /// Bindings shared by both stages.
    pub bindings: Vec<WGSLBinding>,
    /// Struct definitions.
    pub structs: Vec<WGSLStruct>,
}
impl WGSLRenderPipeline {
    /// Create a minimal render pipeline with given name.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        let name_str = name.into();
        WGSLRenderPipeline {
            vertex_input: format!("{}Input", name_str),
            varying: format!("{}Varying", name_str),
            name: name_str,
            vs_body: Vec::new(),
            fs_body: Vec::new(),
            bindings: Vec::new(),
            structs: Vec::new(),
        }
    }
    /// Emit the complete WGSL shader for this pipeline.
    #[allow(dead_code)]
    pub fn emit(&self) -> String {
        let mut shader = WGSLShader::new();
        for s in &self.structs {
            shader.add_struct(s.clone());
        }
        for b in &self.bindings {
            shader.add_binding(b.clone());
        }
        let vs_name = format!("{}_vs", self.name);
        let mut vs = WGSLFunction::vertex(&vs_name);
        vs.add_param(WGSLParam::new(
            "input",
            WGSLType::Struct(self.vertex_input.clone()),
        ));
        vs.set_return_type(WGSLType::Struct(self.varying.clone()));
        for stmt in &self.vs_body {
            vs.add_statement(stmt.clone());
        }
        shader.add_function(vs);
        let fs_name = format!("{}_fs", self.name);
        let mut fs = WGSLFunction::fragment(&fs_name);
        fs.add_param(WGSLParam::new(
            "varying",
            WGSLType::Struct(self.varying.clone()),
        ));
        fs.set_return_type_with_attrib(WGSLType::Vec4f, WGSLReturnAttrib::Location(0));
        for stmt in &self.fs_body {
            fs.add_statement(stmt.clone());
        }
        shader.add_function(fs);
        WGSLBackend::new().emit_shader(&shader)
    }
}
/// What kind of resource occupies a binding slot.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum WGSLResourceType {
    /// Uniform buffer.
    UniformBuffer,
    /// Storage buffer (read-only).
    StorageBufferReadOnly,
    /// Storage buffer (read-write).
    StorageBufferReadWrite,
    /// Sampled texture.
    SampledTexture,
    /// Storage texture.
    StorageTexture { format: String },
    /// Sampler.
    Sampler,
    /// Comparison sampler.
    ComparisonSampler,
}
/// WGSL address space (storage class equivalent).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WGSLAddressSpace {
    /// `function` — per-invocation, local to a function.
    Function,
    /// `private` — per-invocation, module scope.
    Private,
    /// `workgroup` — shared within a compute workgroup.
    Workgroup,
    /// `uniform` — read-only uniform buffer.
    Uniform,
    /// `storage` — read/write storage buffer.
    Storage,
    /// `handle` — opaque resources (textures, samplers).
    Handle,
}
/// Type checking result.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum WGSLTypeError {
    /// A binary operation received mismatched operand types.
    TypeMismatch { expected: WGSLType, found: WGSLType },
    /// An operand type is not valid for a given operation.
    InvalidOperandType { op: String, ty: WGSLType },
    /// A swizzle component is out of range.
    SwizzleOutOfRange { component: char, ty: WGSLType },
    /// Attempted to bind a non-host-shareable type.
    NonShareableBinding { ty: WGSLType },
}
/// Code generation backend that emits WGSL source text.
pub struct WGSLBackend;
impl WGSLBackend {
    /// Create a new WGSL backend.
    pub fn new() -> Self {
        WGSLBackend
    }
    /// Emit a complete WGSL module as source text.
    pub fn emit_shader(&self, shader: &WGSLShader) -> String {
        let mut out = String::new();
        for e in &shader.enables {
            out.push_str(&format!("enable {};\n", e));
        }
        if !shader.enables.is_empty() {
            out.push('\n');
        }
        for c in &shader.constants {
            out.push_str(&c.emit());
            out.push('\n');
        }
        if !shader.constants.is_empty() {
            out.push('\n');
        }
        for o in &shader.overrides {
            out.push_str(&o.emit());
            out.push('\n');
        }
        if !shader.overrides.is_empty() {
            out.push('\n');
        }
        for s in &shader.structs {
            out.push_str(&s.emit());
            out.push_str("\n\n");
        }
        for b in &shader.bindings {
            out.push_str(&b.emit());
            out.push('\n');
        }
        if !shader.bindings.is_empty() {
            out.push('\n');
        }
        for g in &shader.globals {
            out.push_str(&g.emit());
            out.push('\n');
        }
        if !shader.globals.is_empty() {
            out.push('\n');
        }
        for f in &shader.functions {
            out.push_str(&f.emit());
            out.push_str("\n\n");
        }
        out
    }
    /// Build a minimal triangle vertex + fragment shader pair as a single WGSL module.
    pub fn triangle_shader_template(&self) -> String {
        let mut shader = WGSLShader::new();
        let mut vo = WGSLStruct::new("VertexOutput");
        vo.add_field(WGSLStructField::builtin(
            "position",
            WGSLType::Vec4f,
            "position",
        ));
        vo.add_field(WGSLStructField::location("color", WGSLType::Vec4f, 0));
        shader.add_struct(vo);
        let mut vert = WGSLFunction::vertex("vs_main");
        vert.add_param(WGSLParam::with_builtin(
            "vertex_index",
            WGSLType::U32,
            "vertex_index",
        ));
        vert.set_return_type(WGSLType::Struct("VertexOutput".into()));
        vert.add_statement(
            "var positions = array<vec2<f32>, 3>(vec2(0.0, 0.5), vec2(-0.5, -0.5), vec2(0.5, -0.5))",
        );
        vert.add_statement(
            "var colors = array<vec4<f32>, 3>(vec4(1.0, 0.0, 0.0, 1.0), vec4(0.0, 1.0, 0.0, 1.0), vec4(0.0, 0.0, 1.0, 1.0))",
        );
        vert.add_statement("var out: VertexOutput");
        vert.add_statement("out.position = vec4<f32>(positions[vertex_index], 0.0, 1.0)");
        vert.add_statement("out.color = colors[vertex_index]");
        vert.add_statement("return out");
        shader.add_function(vert);
        let mut frag = WGSLFunction::fragment("fs_main");
        frag.add_param(WGSLParam::with_location(
            "in",
            WGSLType::Struct("VertexOutput".into()),
            0,
        ));
        frag.set_return_type_with_attrib(WGSLType::Vec4f, WGSLReturnAttrib::Location(0));
        frag.add_statement("return in.color");
        shader.add_function(frag);
        self.emit_shader(&shader)
    }
    /// Build a basic compute shader template for parallel data processing.
    pub fn compute_shader_template(&self) -> String {
        let mut shader = WGSLShader::new();
        shader.add_binding(WGSLBinding::storage(
            0,
            0,
            "input_data",
            WGSLType::RuntimeArray(Box::new(WGSLType::F32)),
            WGSLAccess::Read,
        ));
        shader.add_binding(WGSLBinding::storage(
            0,
            1,
            "output_data",
            WGSLType::RuntimeArray(Box::new(WGSLType::F32)),
            WGSLAccess::ReadWrite,
        ));
        let mut comp = WGSLFunction::compute("main", 64, 1, 1);
        comp.add_param(WGSLParam::with_builtin(
            "global_id",
            WGSLType::Vec3u,
            "global_invocation_id",
        ));
        comp.add_statement("let idx = global_id.x");
        comp.add_statement("output_data[idx] = input_data[idx] * 2.0");
        shader.add_function(comp);
        self.emit_shader(&shader)
    }
    /// Build a texture sampling shader template.
    pub fn texture_sample_template(&self) -> String {
        let mut shader = WGSLShader::new();
        let mut vo = WGSLStruct::new("VertexOutput");
        vo.add_field(WGSLStructField::builtin(
            "position",
            WGSLType::Vec4f,
            "position",
        ));
        vo.add_field(WGSLStructField::location("uv", WGSLType::Vec2f, 0));
        shader.add_struct(vo);
        shader.add_binding(WGSLBinding::new(0, 0, "t_diffuse", WGSLType::Texture2D));
        shader.add_binding(WGSLBinding::new(0, 1, "s_diffuse", WGSLType::Sampler));
        let mut ub = WGSLStruct::new("Uniforms");
        ub.add_field(WGSLStructField::new("transform", WGSLType::Mat4x4f));
        shader.add_struct(ub);
        shader.add_binding(WGSLBinding::new(
            1,
            0,
            "uniforms",
            WGSLType::Struct("Uniforms".into()),
        ));
        let mut vert = WGSLFunction::vertex("vs_main");
        vert.add_param(WGSLParam::with_location("position", WGSLType::Vec4f, 0));
        vert.add_param(WGSLParam::with_location("uv", WGSLType::Vec2f, 1));
        vert.set_return_type(WGSLType::Struct("VertexOutput".into()));
        vert.add_statement("var out: VertexOutput");
        vert.add_statement("out.position = uniforms.transform * position");
        vert.add_statement("out.uv = uv");
        vert.add_statement("return out");
        shader.add_function(vert);
        let mut frag = WGSLFunction::fragment("fs_main");
        frag.add_param(WGSLParam::with_location(
            "in",
            WGSLType::Struct("VertexOutput".into()),
            0,
        ));
        frag.set_return_type_with_attrib(WGSLType::Vec4f, WGSLReturnAttrib::Location(0));
        frag.add_statement("return textureSample(t_diffuse, s_diffuse, in.uv)");
        shader.add_function(frag);
        self.emit_shader(&shader)
    }
    /// Build a workgroup-shared-memory reduction compute shader.
    pub fn reduction_shader_template(&self, workgroup_size: u32) -> String {
        let mut shader = WGSLShader::new();
        let mut ws_override = WGSLOverride::new("WORKGROUP_SIZE", WGSLType::U32);
        ws_override.default_value = Some(workgroup_size.to_string());
        shader.add_override(ws_override);
        shader.add_binding(WGSLBinding::storage(
            0,
            0,
            "data",
            WGSLType::RuntimeArray(Box::new(WGSLType::F32)),
            WGSLAccess::Read,
        ));
        shader.add_binding(WGSLBinding::storage(
            0,
            1,
            "result",
            WGSLType::F32,
            WGSLAccess::ReadWrite,
        ));
        shader.add_global(WGSLGlobal::workgroup(
            "shared_data",
            WGSLType::Array(Box::new(WGSLType::F32), workgroup_size),
        ));
        let mut comp = WGSLFunction::compute("reduce", workgroup_size, 1, 1);
        comp.add_param(WGSLParam::with_builtin(
            "global_id",
            WGSLType::Vec3u,
            "global_invocation_id",
        ));
        comp.add_param(WGSLParam::with_builtin(
            "local_id",
            WGSLType::Vec3u,
            "local_invocation_id",
        ));
        comp.add_statement("let gid = global_id.x");
        comp.add_statement("let lid = local_id.x");
        comp.add_statement("shared_data[lid] = data[gid]");
        comp.add_statement("workgroupBarrier()");
        comp.add_statement("var stride = WORKGROUP_SIZE / 2u");
        comp.add_statement(
            "loop { if stride == 0u { break; } if lid < stride { shared_data[lid] += shared_data[lid + stride]; } workgroupBarrier(); stride /= 2u; }",
        );
        comp.add_statement("if lid == 0u { result = shared_data[0u]; }");
        shader.add_function(comp);
        self.emit_shader(&shader)
    }
}
/// A descriptor binding group layout.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WGSLBindingGroupLayout {
    /// Group index.
    pub group: u32,
    /// Entries in this group.
    pub entries: Vec<WGSLBindingEntry>,
}
impl WGSLBindingGroupLayout {
    /// Create a new empty binding group layout.
    #[allow(dead_code)]
    pub fn new(group: u32) -> Self {
        WGSLBindingGroupLayout {
            group,
            entries: Vec::new(),
        }
    }
    /// Add a binding entry.
    #[allow(dead_code)]
    pub fn add_entry(
        &mut self,
        binding: u32,
        resource_type: WGSLResourceType,
        visibility: WGSLStageVisibility,
    ) {
        self.entries.push(WGSLBindingEntry {
            binding,
            resource_type,
            visibility,
        });
    }
    /// Emit a comment block describing the layout.
    #[allow(dead_code)]
    pub fn emit_comment(&self) -> String {
        let mut out = format!("// BindGroup {} layout:\n", self.group);
        for e in &self.entries {
            out.push_str(&format!(
                "//   binding={} type={:?} visibility={}\n",
                e.binding, e.resource_type, e.visibility
            ));
        }
        out
    }
}
/// A simple WGSL expression for code generation purposes.
#[derive(Debug, Clone)]
pub enum WGSLExpr {
    /// A literal value token.
    Literal(String),
    /// A variable reference.
    Var(String),
    /// A binary infix operation.
    BinOp {
        op: String,
        lhs: Box<WGSLExpr>,
        rhs: Box<WGSLExpr>,
    },
    /// A unary prefix operation.
    UnaryOp { op: String, operand: Box<WGSLExpr> },
    /// A function or constructor call.
    Call { func: String, args: Vec<WGSLExpr> },
    /// A field access: `base.field`.
    Field { base: Box<WGSLExpr>, field: String },
    /// An array/vector index: `base[index]`.
    Index {
        base: Box<WGSLExpr>,
        index: Box<WGSLExpr>,
    },
}
impl WGSLExpr {
    /// Emit the expression as WGSL source text.
    pub fn emit(&self) -> String {
        match self {
            WGSLExpr::Literal(s) => s.clone(),
            WGSLExpr::Var(name) => name.clone(),
            WGSLExpr::BinOp { op, lhs, rhs } => {
                format!("({} {} {})", lhs.emit(), op, rhs.emit())
            }
            WGSLExpr::UnaryOp { op, operand } => format!("({}{})", op, operand.emit()),
            WGSLExpr::Call { func, args } => {
                let arg_strs: Vec<String> = args.iter().map(|a| a.emit()).collect();
                format!("{}({})", func, arg_strs.join(", "))
            }
            WGSLExpr::Field { base, field } => format!("{}.{}", base.emit(), field),
            WGSLExpr::Index { base, index } => {
                format!("{}[{}]", base.emit(), index.emit())
            }
        }
    }
    /// Convenience: binary operation.
    pub fn binop(op: impl Into<String>, lhs: WGSLExpr, rhs: WGSLExpr) -> Self {
        WGSLExpr::BinOp {
            op: op.into(),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
    /// Convenience: function call.
    pub fn call(func: impl Into<String>, args: Vec<WGSLExpr>) -> Self {
        WGSLExpr::Call {
            func: func.into(),
            args,
        }
    }
    /// Convenience: variable reference.
    pub fn var(name: impl Into<String>) -> Self {
        WGSLExpr::Var(name.into())
    }
    /// Convenience: f32 literal.
    pub fn f32_lit(v: f32) -> Self {
        WGSLExpr::Literal(format!("{:.6}", v))
    }
    /// Convenience: u32 literal.
    pub fn u32_lit(v: u32) -> Self {
        WGSLExpr::Literal(format!("{}u", v))
    }
}
/// Structural validation pass for WGSLShader objects.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct WGSLValidator {
    /// Accumulated errors.
    pub errors: Vec<String>,
    /// Accumulated warnings.
    pub warnings: Vec<String>,
}
impl WGSLValidator {
    /// Create a new validator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        WGSLValidator::default()
    }
    /// Validate a shader module. Returns `true` if no errors were found.
    #[allow(dead_code)]
    pub fn validate(&mut self, shader: &WGSLShader) -> bool {
        self.errors.clear();
        self.warnings.clear();
        let mut fn_names = std::collections::HashSet::new();
        for f in &shader.functions {
            if !fn_names.insert(f.name.clone()) {
                self.errors
                    .push(format!("duplicate function name: '{}'", f.name));
            }
        }
        let mut struct_names = std::collections::HashSet::new();
        for s in &shader.structs {
            if !struct_names.insert(s.name.clone()) {
                self.errors
                    .push(format!("duplicate struct name: '{}'", s.name));
            }
        }
        let mut binding_slots = std::collections::HashSet::new();
        for b in &shader.bindings {
            let key = (b.group, b.binding);
            if !binding_slots.insert(key) {
                self.errors.push(format!(
                    "duplicate binding @group({}) @binding({})",
                    b.group, b.binding
                ));
            }
        }
        if shader.functions.is_empty() {
            self.warnings.push("shader has no functions".to_string());
        }
        for f in &shader.functions {
            match f.entry_point {
                WGSLEntryPoint::Compute { .. } => {
                    if f.params.is_empty() {
                        self.warnings.push(format!(
                            "compute entry '{}' has no parameters (no global_invocation_id?)",
                            f.name
                        ));
                    }
                }
                _ => {}
            }
        }
        self.errors.is_empty()
    }
    /// Return true if validation passed.
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}
/// Attribute placed on the return type of an entry point.
#[derive(Debug, Clone)]
pub enum WGSLReturnAttrib {
    /// `@builtin(…)`
    Builtin(String),
    /// `@location(N)`
    Location(u32),
    /// No attribute.
    None,
}
impl WGSLReturnAttrib {
    /// Emit the attribute string preceding the return type.
    pub fn prefix(&self) -> String {
        match self {
            WGSLReturnAttrib::Builtin(b) => format!("@builtin({}) ", b),
            WGSLReturnAttrib::Location(n) => format!("@location({}) ", n),
            WGSLReturnAttrib::None => String::new(),
        }
    }
}
/// Helpers for constructing common WGSL primitive expressions and patterns.
#[allow(dead_code)]
pub struct WGSLPrimitiveHelper;
impl WGSLPrimitiveHelper {
    /// Generate a `vec2f(x, y)` constructor expression.
    #[allow(dead_code)]
    pub fn vec2f(x: f32, y: f32) -> String {
        format!("vec2f({}, {})", x, y)
    }
    /// Generate a `vec3f(x, y, z)` constructor expression.
    #[allow(dead_code)]
    pub fn vec3f(x: f32, y: f32, z: f32) -> String {
        format!("vec3f({}, {}, {})", x, y, z)
    }
    /// Generate a `vec4f(x, y, z, w)` constructor expression.
    #[allow(dead_code)]
    pub fn vec4f(x: f32, y: f32, z: f32, w: f32) -> String {
        format!("vec4f({}, {}, {}, {})", x, y, z, w)
    }
    /// Generate a `vec2u(x, y)` constructor expression.
    #[allow(dead_code)]
    pub fn vec2u(x: u32, y: u32) -> String {
        format!("vec2u({}u, {}u)", x, y)
    }
    /// Generate a `vec3u(x, y, z)` constructor expression.
    #[allow(dead_code)]
    pub fn vec3u(x: u32, y: u32, z: u32) -> String {
        format!("vec3u({}u, {}u, {}u)", x, y, z)
    }
    /// Generate a `mat4x4<f32>` identity matrix constructor.
    #[allow(dead_code)]
    pub fn mat4x4_identity() -> String {
        "mat4x4f(1.0,0.0,0.0,0.0, 0.0,1.0,0.0,0.0, 0.0,0.0,1.0,0.0, 0.0,0.0,0.0,1.0)".to_string()
    }
    /// Generate a `mat4x4f` perspective projection matrix.
    #[allow(dead_code)]
    pub fn perspective_matrix(fov_y_rad: f32, aspect: f32, near: f32, far: f32) -> String {
        let f = 1.0 / (fov_y_rad / 2.0).tan();
        let nf = 1.0 / (near - far);
        format!(
            "mat4x4f({f},0.0,0.0,0.0, 0.0,{f_a},0.0,0.0, 0.0,0.0,{nf_a},{b}, 0.0,0.0,-1.0,0.0)",
            f = f,
            f_a = f / aspect,
            nf_a = (near + far) * nf,
            b = 2.0 * far * near * nf,
        )
    }
    /// Generate an orthographic projection matrix.
    #[allow(dead_code)]
    pub fn ortho_matrix(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> String {
        let rl = right - left;
        let tb = top - bottom;
        let fn_ = far - near;
        format!(
            "mat4x4f({a},0.0,0.0,0.0, 0.0,{b},0.0,0.0, 0.0,0.0,{c},0.0, {tx},{ty},{tz},1.0)",
            a = 2.0 / rl,
            b = 2.0 / tb,
            c = -2.0 / fn_,
            tx = -(right + left) / rl,
            ty = -(top + bottom) / tb,
            tz = -(far + near) / fn_,
        )
    }
    /// Generate a swizzle expression (e.g., `v.xyz`).
    #[allow(dead_code)]
    pub fn swizzle(base: &str, components: &str) -> String {
        format!("{}.{}", base, components)
    }
    /// Generate a ternary select expression: `select(false_val, true_val, cond)`.
    #[allow(dead_code)]
    pub fn select(false_val: &str, true_val: &str, cond: &str) -> String {
        format!("select({}, {}, {})", false_val, true_val, cond)
    }
    /// Generate an atomicAdd expression.
    #[allow(dead_code)]
    pub fn atomic_add(ptr: &str, val: &str) -> String {
        format!("atomicAdd({}, {})", ptr, val)
    }
    /// Generate a workgroupBarrier() call statement.
    #[allow(dead_code)]
    pub fn barrier() -> &'static str {
        "workgroupBarrier()"
    }
}
/// Fluent builder for constructing complete WGSL shader modules.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct WGSLShaderBuilder {
    pub(crate) shader: WGSLShader,
    pub(crate) next_group: u32,
    pub(crate) next_binding: u32,
}
impl WGSLShaderBuilder {
    /// Create a new empty builder.
    #[allow(dead_code)]
    pub fn new() -> Self {
        WGSLShaderBuilder::default()
    }
    /// Enable an extension (e.g. `"f16"`).
    #[allow(dead_code)]
    pub fn enable(mut self, ext: impl Into<String>) -> Self {
        self.shader.add_enable(ext);
        self
    }
    /// Add a typed compile-time constant.
    #[allow(dead_code)]
    pub fn constant(
        mut self,
        name: impl Into<String>,
        ty: WGSLType,
        value: impl Into<String>,
    ) -> Self {
        self.shader
            .add_constant(WGSLConstant::typed(name, ty, value));
        self
    }
    /// Add a struct definition.
    #[allow(dead_code)]
    pub fn struct_def(mut self, s: WGSLStruct) -> Self {
        self.shader.add_struct(s);
        self
    }
    /// Add a uniform buffer binding (auto-increments group/binding).
    #[allow(dead_code)]
    pub fn uniform(mut self, name: impl Into<String>, ty: WGSLType) -> Self {
        let b = WGSLBinding::new(self.next_group, self.next_binding, name, ty);
        self.next_binding += 1;
        self.shader.add_binding(b);
        self
    }
    /// Advance to the next binding group.
    #[allow(dead_code)]
    pub fn next_group(mut self) -> Self {
        self.next_group += 1;
        self.next_binding = 0;
        self
    }
    /// Add a read-only storage buffer.
    #[allow(dead_code)]
    pub fn storage_read(mut self, name: impl Into<String>, elem_ty: WGSLType) -> Self {
        let b = WGSLBinding::storage(
            self.next_group,
            self.next_binding,
            name,
            WGSLType::RuntimeArray(Box::new(elem_ty)),
            WGSLAccess::Read,
        );
        self.next_binding += 1;
        self.shader.add_binding(b);
        self
    }
    /// Add a read-write storage buffer.
    #[allow(dead_code)]
    pub fn storage_rw(mut self, name: impl Into<String>, elem_ty: WGSLType) -> Self {
        let b = WGSLBinding::storage(
            self.next_group,
            self.next_binding,
            name,
            WGSLType::RuntimeArray(Box::new(elem_ty)),
            WGSLAccess::ReadWrite,
        );
        self.next_binding += 1;
        self.shader.add_binding(b);
        self
    }
    /// Add a texture2D binding.
    #[allow(dead_code)]
    pub fn texture2d(mut self, name: impl Into<String>) -> Self {
        let b = WGSLBinding::new(
            self.next_group,
            self.next_binding,
            name,
            WGSLType::Texture2D,
        );
        self.next_binding += 1;
        self.shader.add_binding(b);
        self
    }
    /// Add a sampler binding.
    #[allow(dead_code)]
    pub fn sampler(mut self, name: impl Into<String>) -> Self {
        let b = WGSLBinding::new(self.next_group, self.next_binding, name, WGSLType::Sampler);
        self.next_binding += 1;
        self.shader.add_binding(b);
        self
    }
    /// Add a workgroup-scope variable.
    #[allow(dead_code)]
    pub fn workgroup_var(mut self, name: impl Into<String>, ty: WGSLType) -> Self {
        self.shader.add_global(WGSLGlobal::workgroup(name, ty));
        self
    }
    /// Add a private-scope variable.
    #[allow(dead_code)]
    pub fn private_var(mut self, name: impl Into<String>, ty: WGSLType) -> Self {
        self.shader.add_global(WGSLGlobal::private(name, ty));
        self
    }
    /// Add a helper function.
    #[allow(dead_code)]
    pub fn helper(mut self, f: WGSLFunction) -> Self {
        self.shader.add_function(f);
        self
    }
    /// Consume the builder and return the completed shader.
    #[allow(dead_code)]
    pub fn build(self) -> WGSLShader {
        self.shader
    }
}
/// A module-scope variable (`var<address_space>`).
#[derive(Debug, Clone)]
pub struct WGSLGlobal {
    /// Variable name.
    pub name: String,
    /// Type.
    pub ty: WGSLType,
    /// Address space.
    pub address_space: WGSLAddressSpace,
    /// Optional access mode.
    pub access: Option<WGSLAccess>,
    /// Optional initialiser expression.
    pub initializer: Option<String>,
}
impl WGSLGlobal {
    /// Create a `var<private>` global.
    pub fn private(name: impl Into<String>, ty: WGSLType) -> Self {
        WGSLGlobal {
            name: name.into(),
            ty,
            address_space: WGSLAddressSpace::Private,
            access: None,
            initializer: None,
        }
    }
    /// Create a `var<workgroup>` shared memory variable.
    pub fn workgroup(name: impl Into<String>, ty: WGSLType) -> Self {
        WGSLGlobal {
            name: name.into(),
            ty,
            address_space: WGSLAddressSpace::Workgroup,
            access: None,
            initializer: None,
        }
    }
    /// Emit the declaration.
    pub fn emit(&self) -> String {
        let access_str = match self.access {
            Some(a) => format!(", {}", a),
            None => String::new(),
        };
        let init = match &self.initializer {
            Some(v) => format!(" = {}", v),
            None => String::new(),
        };
        format!(
            "var<{}{}> {}: {}{};",
            self.address_space, access_str, self.name, self.ty, init
        )
    }
}
/// A pipeline-overridable constant (`override`).
#[derive(Debug, Clone)]
pub struct WGSLOverride {
    /// Constant name.
    pub name: String,
    /// Type.
    pub ty: WGSLType,
    /// Optional `@id(N)` numeric ID.
    pub id: Option<u32>,
    /// Optional default value expression.
    pub default_value: Option<String>,
}
impl WGSLOverride {
    /// Create a new override with no id or default.
    pub fn new(name: impl Into<String>, ty: WGSLType) -> Self {
        WGSLOverride {
            name: name.into(),
            ty,
            id: None,
            default_value: None,
        }
    }
    /// Emit the override declaration.
    pub fn emit(&self) -> String {
        let id_attr = match self.id {
            Some(n) => format!("@id({}) ", n),
            None => String::new(),
        };
        let init = match &self.default_value {
            Some(v) => format!(" = {}", v),
            None => String::new(),
        };
        format!("{}override {}: {}{};", id_attr, self.name, self.ty, init)
    }
}
/// Standard WGSL built-in functions for code generation helpers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum WGSLBuiltinFunction {
    Abs,
    Acos,
    Acosh,
    Asin,
    Asinh,
    Atan,
    Atanh,
    Atan2,
    Ceil,
    Clamp,
    Cos,
    Cosh,
    Cross,
    Degrees,
    Distance,
    Dot,
    Exp,
    Exp2,
    FaceForward,
    Floor,
    Fma,
    Fract,
    Frexp,
    InverseSqrt,
    Ldexp,
    Length,
    Log,
    Log2,
    Max,
    Min,
    Mix,
    Modf,
    Normalize,
    Pow,
    Quantize,
    Radians,
    Reflect,
    Refract,
    Round,
    Saturate,
    Sign,
    Sin,
    Sinh,
    Smoothstep,
    Sqrt,
    Step,
    Tan,
    Tanh,
    Transpose,
    Trunc,
    CountLeadingZeros,
    CountOneBits,
    CountTrailingZeros,
    ExtractBits,
    FirstLeadingBit,
    FirstTrailingBit,
    InsertBits,
    ReverseBits,
    TextureDimensions,
    TextureGather,
    TextureGatherCompare,
    TextureLoad,
    TextureNumLayers,
    TextureNumLevels,
    TextureNumSamples,
    TextureSample,
    TextureSampleBias,
    TextureSampleCompare,
    TextureSampleCompareLevel,
    TextureSampleGrad,
    TextureSampleLevel,
    TextureStore,
    Dpdx,
    Dpdxcoarse,
    Dpdxfine,
    Dpdy,
    Dpdycoarse,
    Dpdyfine,
    Fwidth,
    FwidthCoarse,
    FwidthFine,
    AtomicLoad,
    AtomicStore,
    AtomicAdd,
    AtomicSub,
    AtomicMax,
    AtomicMin,
    AtomicAnd,
    AtomicOr,
    AtomicXor,
    AtomicExchange,
    AtomicCompareExchangeWeak,
    WorkgroupBarrier,
    StorageBarrier,
    TextureBarrier,
    Pack2x16float,
    Pack2x16snorm,
    Pack2x16unorm,
    Pack4x8snorm,
    Pack4x8unorm,
    Unpack2x16float,
    Unpack2x16snorm,
    Unpack2x16unorm,
    Unpack4x8snorm,
    Unpack4x8unorm,
}
impl WGSLBuiltinFunction {
    /// Return the WGSL name of this built-in function.
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            WGSLBuiltinFunction::Abs => "abs",
            WGSLBuiltinFunction::Acos => "acos",
            WGSLBuiltinFunction::Acosh => "acosh",
            WGSLBuiltinFunction::Asin => "asin",
            WGSLBuiltinFunction::Asinh => "asinh",
            WGSLBuiltinFunction::Atan => "atan",
            WGSLBuiltinFunction::Atanh => "atanh",
            WGSLBuiltinFunction::Atan2 => "atan2",
            WGSLBuiltinFunction::Ceil => "ceil",
            WGSLBuiltinFunction::Clamp => "clamp",
            WGSLBuiltinFunction::Cos => "cos",
            WGSLBuiltinFunction::Cosh => "cosh",
            WGSLBuiltinFunction::Cross => "cross",
            WGSLBuiltinFunction::Degrees => "degrees",
            WGSLBuiltinFunction::Distance => "distance",
            WGSLBuiltinFunction::Dot => "dot",
            WGSLBuiltinFunction::Exp => "exp",
            WGSLBuiltinFunction::Exp2 => "exp2",
            WGSLBuiltinFunction::FaceForward => "faceForward",
            WGSLBuiltinFunction::Floor => "floor",
            WGSLBuiltinFunction::Fma => "fma",
            WGSLBuiltinFunction::Fract => "fract",
            WGSLBuiltinFunction::Frexp => "frexp",
            WGSLBuiltinFunction::InverseSqrt => "inverseSqrt",
            WGSLBuiltinFunction::Ldexp => "ldexp",
            WGSLBuiltinFunction::Length => "length",
            WGSLBuiltinFunction::Log => "log",
            WGSLBuiltinFunction::Log2 => "log2",
            WGSLBuiltinFunction::Max => "max",
            WGSLBuiltinFunction::Min => "min",
            WGSLBuiltinFunction::Mix => "mix",
            WGSLBuiltinFunction::Modf => "modf",
            WGSLBuiltinFunction::Normalize => "normalize",
            WGSLBuiltinFunction::Pow => "pow",
            WGSLBuiltinFunction::Quantize => "quantizeToF16",
            WGSLBuiltinFunction::Radians => "radians",
            WGSLBuiltinFunction::Reflect => "reflect",
            WGSLBuiltinFunction::Refract => "refract",
            WGSLBuiltinFunction::Round => "round",
            WGSLBuiltinFunction::Saturate => "saturate",
            WGSLBuiltinFunction::Sign => "sign",
            WGSLBuiltinFunction::Sin => "sin",
            WGSLBuiltinFunction::Sinh => "sinh",
            WGSLBuiltinFunction::Smoothstep => "smoothstep",
            WGSLBuiltinFunction::Sqrt => "sqrt",
            WGSLBuiltinFunction::Step => "step",
            WGSLBuiltinFunction::Tan => "tan",
            WGSLBuiltinFunction::Tanh => "tanh",
            WGSLBuiltinFunction::Transpose => "transpose",
            WGSLBuiltinFunction::Trunc => "trunc",
            WGSLBuiltinFunction::CountLeadingZeros => "countLeadingZeros",
            WGSLBuiltinFunction::CountOneBits => "countOneBits",
            WGSLBuiltinFunction::CountTrailingZeros => "countTrailingZeros",
            WGSLBuiltinFunction::ExtractBits => "extractBits",
            WGSLBuiltinFunction::FirstLeadingBit => "firstLeadingBit",
            WGSLBuiltinFunction::FirstTrailingBit => "firstTrailingBit",
            WGSLBuiltinFunction::InsertBits => "insertBits",
            WGSLBuiltinFunction::ReverseBits => "reverseBits",
            WGSLBuiltinFunction::TextureDimensions => "textureDimensions",
            WGSLBuiltinFunction::TextureGather => "textureGather",
            WGSLBuiltinFunction::TextureGatherCompare => "textureGatherCompare",
            WGSLBuiltinFunction::TextureLoad => "textureLoad",
            WGSLBuiltinFunction::TextureNumLayers => "textureNumLayers",
            WGSLBuiltinFunction::TextureNumLevels => "textureNumLevels",
            WGSLBuiltinFunction::TextureNumSamples => "textureNumSamples",
            WGSLBuiltinFunction::TextureSample => "textureSample",
            WGSLBuiltinFunction::TextureSampleBias => "textureSampleBias",
            WGSLBuiltinFunction::TextureSampleCompare => "textureSampleCompare",
            WGSLBuiltinFunction::TextureSampleCompareLevel => "textureSampleCompareLevel",
            WGSLBuiltinFunction::TextureSampleGrad => "textureSampleGrad",
            WGSLBuiltinFunction::TextureSampleLevel => "textureSampleLevel",
            WGSLBuiltinFunction::TextureStore => "textureStore",
            WGSLBuiltinFunction::Dpdx => "dpdx",
            WGSLBuiltinFunction::Dpdxcoarse => "dpdxCoarse",
            WGSLBuiltinFunction::Dpdxfine => "dpdxFine",
            WGSLBuiltinFunction::Dpdy => "dpdy",
            WGSLBuiltinFunction::Dpdycoarse => "dpdyCoarse",
            WGSLBuiltinFunction::Dpdyfine => "dpdyFine",
            WGSLBuiltinFunction::Fwidth => "fwidth",
            WGSLBuiltinFunction::FwidthCoarse => "fwidthCoarse",
            WGSLBuiltinFunction::FwidthFine => "fwidthFine",
            WGSLBuiltinFunction::AtomicLoad => "atomicLoad",
            WGSLBuiltinFunction::AtomicStore => "atomicStore",
            WGSLBuiltinFunction::AtomicAdd => "atomicAdd",
            WGSLBuiltinFunction::AtomicSub => "atomicSub",
            WGSLBuiltinFunction::AtomicMax => "atomicMax",
            WGSLBuiltinFunction::AtomicMin => "atomicMin",
            WGSLBuiltinFunction::AtomicAnd => "atomicAnd",
            WGSLBuiltinFunction::AtomicOr => "atomicOr",
            WGSLBuiltinFunction::AtomicXor => "atomicXor",
            WGSLBuiltinFunction::AtomicExchange => "atomicExchange",
            WGSLBuiltinFunction::AtomicCompareExchangeWeak => "atomicCompareExchangeWeak",
            WGSLBuiltinFunction::WorkgroupBarrier => "workgroupBarrier",
            WGSLBuiltinFunction::StorageBarrier => "storageBarrier",
            WGSLBuiltinFunction::TextureBarrier => "textureBarrier",
            WGSLBuiltinFunction::Pack2x16float => "pack2x16float",
            WGSLBuiltinFunction::Pack2x16snorm => "pack2x16snorm",
            WGSLBuiltinFunction::Pack2x16unorm => "pack2x16unorm",
            WGSLBuiltinFunction::Pack4x8snorm => "pack4x8snorm",
            WGSLBuiltinFunction::Pack4x8unorm => "pack4x8unorm",
            WGSLBuiltinFunction::Unpack2x16float => "unpack2x16float",
            WGSLBuiltinFunction::Unpack2x16snorm => "unpack2x16snorm",
            WGSLBuiltinFunction::Unpack2x16unorm => "unpack2x16unorm",
            WGSLBuiltinFunction::Unpack4x8snorm => "unpack4x8snorm",
            WGSLBuiltinFunction::Unpack4x8unorm => "unpack4x8unorm",
        }
    }
    /// Generate a WGSL call expression string for this built-in.
    #[allow(dead_code)]
    pub fn call(&self, args: &[&str]) -> String {
        format!("{}({})", self.name(), args.join(", "))
    }
}
/// A complete WGSL shader module.
#[derive(Debug, Clone)]
pub struct WGSLShader {
    /// `enable` directives (e.g. `"f16"`, `"chromium_experimental_dp4a"`).
    pub enables: Vec<String>,
    /// Compile-time constants (`const`).
    pub constants: Vec<WGSLConstant>,
    /// Pipeline-overridable constants (`override`).
    pub overrides: Vec<WGSLOverride>,
    /// Struct definitions (emitted before bindings).
    pub structs: Vec<WGSLStruct>,
    /// Resource bindings (`@group … @binding … var`).
    pub bindings: Vec<WGSLBinding>,
    /// Module-scope variables (`var<private>`, `var<workgroup>`, …).
    pub globals: Vec<WGSLGlobal>,
    /// All functions (helpers first, entry points last by convention).
    pub functions: Vec<WGSLFunction>,
}
impl WGSLShader {
    /// Create an empty shader module.
    pub fn new() -> Self {
        WGSLShader {
            enables: Vec::new(),
            constants: Vec::new(),
            overrides: Vec::new(),
            structs: Vec::new(),
            bindings: Vec::new(),
            globals: Vec::new(),
            functions: Vec::new(),
        }
    }
    /// Add an `enable` directive.
    pub fn add_enable(&mut self, ext: impl Into<String>) {
        self.enables.push(ext.into());
    }
    /// Add a compile-time constant.
    pub fn add_constant(&mut self, c: WGSLConstant) {
        self.constants.push(c);
    }
    /// Add a pipeline-overridable constant.
    pub fn add_override(&mut self, o: WGSLOverride) {
        self.overrides.push(o);
    }
    /// Add a struct definition.
    pub fn add_struct(&mut self, s: WGSLStruct) {
        self.structs.push(s);
    }
    /// Add a resource binding.
    pub fn add_binding(&mut self, b: WGSLBinding) {
        self.bindings.push(b);
    }
    /// Add a module-scope variable.
    pub fn add_global(&mut self, g: WGSLGlobal) {
        self.globals.push(g);
    }
    /// Add a function.
    pub fn add_function(&mut self, f: WGSLFunction) {
        self.functions.push(f);
    }
}
