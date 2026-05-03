//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#[allow(unused_imports)]
use super::impls::*;

/// Lightweight WGSL type checker.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct WGSLTypeChecker;
impl WGSLTypeChecker {
    /// Create a new type checker.
    #[allow(dead_code)]
    pub fn new() -> Self {
        WGSLTypeChecker
    }
    /// Check that two types match for a binary arithmetic operation.
    #[allow(dead_code)]
    pub fn check_binop(
        &self,
        _op: &str,
        lhs: &WGSLType,
        rhs: &WGSLType,
    ) -> Result<WGSLType, WGSLTypeError> {
        if lhs == rhs {
            Ok(lhs.clone())
        } else {
            Err(WGSLTypeError::TypeMismatch {
                expected: lhs.clone(),
                found: rhs.clone(),
            })
        }
    }
    /// Check that a type is valid for atomic operations.
    #[allow(dead_code)]
    pub fn check_atomic(&self, ty: &WGSLType) -> Result<(), WGSLTypeError> {
        match ty {
            WGSLType::I32 | WGSLType::U32 => Ok(()),
            _ => Err(WGSLTypeError::InvalidOperandType {
                op: "atomic".to_string(),
                ty: ty.clone(),
            }),
        }
    }
    /// Check that a type is host-shareable (can be used in uniform/storage buffers).
    #[allow(dead_code)]
    pub fn check_host_shareable(&self, ty: &WGSLType) -> Result<(), WGSLTypeError> {
        match ty {
            WGSLType::Bool
            | WGSLType::Sampler
            | WGSLType::SamplerComparison
            | WGSLType::TextureDepth2D
            | WGSLType::Texture2D
            | WGSLType::Texture2DArray
            | WGSLType::TextureCube
            | WGSLType::Texture3D
            | WGSLType::TextureMultisampled2D => {
                Err(WGSLTypeError::NonShareableBinding { ty: ty.clone() })
            }
            _ => Ok(()),
        }
    }
    /// Return the component type of a vector, or the type itself if scalar.
    #[allow(dead_code)]
    pub fn element_type(ty: &WGSLType) -> &WGSLType {
        match ty {
            WGSLType::Vec2f
            | WGSLType::Vec3f
            | WGSLType::Vec4f
            | WGSLType::Mat2x2f
            | WGSLType::Mat3x3f
            | WGSLType::Mat4x4f
            | WGSLType::Mat2x4f
            | WGSLType::Mat4x2f => &WGSLType::F32,
            WGSLType::Vec2i | WGSLType::Vec3i | WGSLType::Vec4i => &WGSLType::I32,
            WGSLType::Vec2u | WGSLType::Vec3u | WGSLType::Vec4u => &WGSLType::U32,
            WGSLType::Vec2b => &WGSLType::Bool,
            _ => ty,
        }
    }
    /// Return the number of vector components, or 1 for scalars.
    #[allow(dead_code)]
    pub fn vector_width(ty: &WGSLType) -> u32 {
        match ty {
            WGSLType::Vec2f | WGSLType::Vec2i | WGSLType::Vec2u | WGSLType::Vec2b => 2,
            WGSLType::Vec3f | WGSLType::Vec3i | WGSLType::Vec3u => 3,
            WGSLType::Vec4f | WGSLType::Vec4i | WGSLType::Vec4u => 4,
            _ => 1,
        }
    }
}
/// A module-scope compile-time constant (`const`).
#[derive(Debug, Clone)]
pub struct WGSLConstant {
    /// Constant name.
    pub name: String,
    /// Optional explicit type.
    pub ty: Option<WGSLType>,
    /// Value expression.
    pub value: String,
}
impl WGSLConstant {
    /// Create a typed constant.
    pub fn typed(name: impl Into<String>, ty: WGSLType, value: impl Into<String>) -> Self {
        WGSLConstant {
            name: name.into(),
            ty: Some(ty),
            value: value.into(),
        }
    }
    /// Create an untyped (inferred) constant.
    pub fn inferred(name: impl Into<String>, value: impl Into<String>) -> Self {
        WGSLConstant {
            name: name.into(),
            ty: None,
            value: value.into(),
        }
    }
    /// Emit the constant declaration.
    pub fn emit(&self) -> String {
        match &self.ty {
            Some(ty) => format!("const {}: {} = {};", self.name, ty, self.value),
            None => format!("const {} = {};", self.name, self.value),
        }
    }
}
/// A WGSL `struct` type definition.
#[derive(Debug, Clone)]
pub struct WGSLStruct {
    /// Struct name.
    pub name: String,
    /// Fields in declaration order.
    pub fields: Vec<WGSLStructField>,
}
impl WGSLStruct {
    /// Create a new empty struct.
    pub fn new(name: impl Into<String>) -> Self {
        WGSLStruct {
            name: name.into(),
            fields: Vec::new(),
        }
    }
    /// Add a field to the struct.
    pub fn add_field(&mut self, field: WGSLStructField) {
        self.fields.push(field);
    }
    /// Emit the `struct { … }` definition.
    pub fn emit(&self) -> String {
        let mut out = format!("struct {} {{\n", self.name);
        for f in &self.fields {
            out.push_str(&f.emit());
            out.push('\n');
        }
        out.push('}');
        out
    }
}
/// A WGSL statement (higher-level than raw strings).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum WGSLStatement {
    /// `let name: ty = expr;`
    Let {
        name: String,
        ty: Option<WGSLType>,
        init: String,
    },
    /// `var name: ty = expr;`
    Var {
        name: String,
        ty: Option<WGSLType>,
        init: Option<String>,
    },
    /// `lhs = rhs;`
    Assign { lhs: String, rhs: String },
    /// `lhs op= rhs;` (compound assignment)
    CompoundAssign {
        lhs: String,
        op: String,
        rhs: String,
    },
    /// `if (cond) { ... } else { ... }`
    If {
        cond: String,
        then_stmts: Vec<WGSLStatement>,
        else_stmts: Vec<WGSLStatement>,
    },
    /// `for (init; cond; update) { ... }`
    For {
        init: Option<Box<WGSLStatement>>,
        cond: Option<String>,
        update: Option<Box<WGSLStatement>>,
        body: Vec<WGSLStatement>,
    },
    /// `while (cond) { ... }`
    While {
        cond: String,
        body: Vec<WGSLStatement>,
    },
    /// `loop { ... continuing { ... } }`
    Loop {
        body: Vec<WGSLStatement>,
        continuing: Vec<WGSLStatement>,
    },
    /// `switch (expr) { case v: { ... } default: { ... } }`
    Switch {
        expr: String,
        cases: Vec<(String, Vec<WGSLStatement>)>,
        default: Vec<WGSLStatement>,
    },
    /// `return expr;`
    Return(Option<String>),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
    /// `discard;`
    Discard,
    /// Raw string statement.
    Raw(String),
    /// Function call as a statement.
    Call { func: String, args: Vec<String> },
}
impl WGSLStatement {
    /// Emit the statement as indented WGSL source text.
    #[allow(dead_code)]
    pub fn emit(&self, indent: usize) -> String {
        let pad = "    ".repeat(indent);
        match self {
            WGSLStatement::Let { name, ty, init } => {
                let ty_str = ty.as_ref().map(|t| format!(": {}", t)).unwrap_or_default();
                format!("{}let {}{} = {};", pad, name, ty_str, init)
            }
            WGSLStatement::Var { name, ty, init } => {
                let ty_str = ty.as_ref().map(|t| format!(": {}", t)).unwrap_or_default();
                let init_str = init
                    .as_ref()
                    .map(|i| format!(" = {}", i))
                    .unwrap_or_default();
                format!("{}var {}{}{};", pad, name, ty_str, init_str)
            }
            WGSLStatement::Assign { lhs, rhs } => format!("{}{} = {};", pad, lhs, rhs),
            WGSLStatement::CompoundAssign { lhs, op, rhs } => {
                format!("{}{} {}= {};", pad, lhs, op, rhs)
            }
            WGSLStatement::If {
                cond,
                then_stmts,
                else_stmts,
            } => {
                let mut out = format!("{}if ({}) {{\n", pad, cond);
                for s in then_stmts {
                    out.push_str(&s.emit(indent + 1));
                    out.push('\n');
                }
                if else_stmts.is_empty() {
                    out.push_str(&format!("{}}}", pad));
                } else {
                    out.push_str(&format!("{}}} else {{\n", pad));
                    for s in else_stmts {
                        out.push_str(&s.emit(indent + 1));
                        out.push('\n');
                    }
                    out.push_str(&format!("{}}}", pad));
                }
                out
            }
            WGSLStatement::For {
                init,
                cond,
                update,
                body,
            } => {
                let init_str = init
                    .as_ref()
                    .map(|s| s.emit(0).trim_end_matches(';').to_string())
                    .unwrap_or_default();
                let cond_str = cond.as_deref().unwrap_or("");
                let update_str = update
                    .as_ref()
                    .map(|s| s.emit(0).trim_end_matches(';').to_string())
                    .unwrap_or_default();
                let mut out = format!(
                    "{}for ({}; {}; {}) {{\n",
                    pad, init_str, cond_str, update_str
                );
                for s in body {
                    out.push_str(&s.emit(indent + 1));
                    out.push('\n');
                }
                out.push_str(&format!("{}}}", pad));
                out
            }
            WGSLStatement::While { cond, body } => {
                let mut out = format!("{}while ({}) {{\n", pad, cond);
                for s in body {
                    out.push_str(&s.emit(indent + 1));
                    out.push('\n');
                }
                out.push_str(&format!("{}}}", pad));
                out
            }
            WGSLStatement::Loop { body, continuing } => {
                let mut out = format!("{}loop {{\n", pad);
                for s in body {
                    out.push_str(&s.emit(indent + 1));
                    out.push('\n');
                }
                if !continuing.is_empty() {
                    out.push_str(&format!("{}    continuing {{\n", pad));
                    for s in continuing {
                        out.push_str(&s.emit(indent + 2));
                        out.push('\n');
                    }
                    out.push_str(&format!("{}    }}\n", pad));
                }
                out.push_str(&format!("{}}}", pad));
                out
            }
            WGSLStatement::Switch {
                expr,
                cases,
                default,
            } => {
                let mut out = format!("{}switch ({}) {{\n", pad, expr);
                for (val, stmts) in cases {
                    out.push_str(&format!("{}    case {}: {{\n", pad, val));
                    for s in stmts {
                        out.push_str(&s.emit(indent + 2));
                        out.push('\n');
                    }
                    out.push_str(&format!("{}    }}\n", pad));
                }
                out.push_str(&format!("{}    default: {{\n", pad));
                for s in default {
                    out.push_str(&s.emit(indent + 2));
                    out.push('\n');
                }
                out.push_str(&format!("{}    }}\n", pad));
                out.push_str(&format!("{}}}", pad));
                out
            }
            WGSLStatement::Return(Some(expr)) => format!("{}return {};", pad, expr),
            WGSLStatement::Return(None) => format!("{}return;", pad),
            WGSLStatement::Break => format!("{}break;", pad),
            WGSLStatement::Continue => format!("{}continue;", pad),
            WGSLStatement::Discard => format!("{}discard;", pad),
            WGSLStatement::Raw(s) => format!("{}{};", pad, s),
            WGSLStatement::Call { func, args } => {
                format!("{}{}({});", pad, func, args.join(", "))
            }
        }
    }
}
/// Parameters for a compute kernel.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WGSLComputeKernelParams {
    /// Kernel entry-point name.
    pub name: String,
    /// Workgroup X dimension.
    pub wg_x: u32,
    /// Workgroup Y dimension.
    pub wg_y: u32,
    /// Workgroup Z dimension.
    pub wg_z: u32,
    /// Whether to include `local_invocation_id` builtin parameter.
    pub use_local_id: bool,
    /// Whether to include `workgroup_id` builtin parameter.
    pub use_workgroup_id: bool,
    /// Whether to include `num_workgroups` builtin parameter.
    pub use_num_workgroups: bool,
}
/// A single function parameter in WGSL.
#[derive(Debug, Clone)]
pub struct WGSLParam {
    /// Parameter name.
    pub name: String,
    /// Parameter type.
    pub ty: WGSLType,
    /// Optional `@builtin(…)` attribute.
    pub builtin: Option<String>,
    /// Optional `@location(N)` attribute.
    pub location: Option<u32>,
}
impl WGSLParam {
    /// Create a plain parameter.
    pub fn new(name: impl Into<String>, ty: WGSLType) -> Self {
        WGSLParam {
            name: name.into(),
            ty,
            builtin: None,
            location: None,
        }
    }
    /// Create a parameter with a `@builtin(…)` attribute.
    pub fn with_builtin(name: impl Into<String>, ty: WGSLType, builtin: impl Into<String>) -> Self {
        WGSLParam {
            name: name.into(),
            ty,
            builtin: Some(builtin.into()),
            location: None,
        }
    }
    /// Create a parameter with a `@location(N)` attribute.
    pub fn with_location(name: impl Into<String>, ty: WGSLType, loc: u32) -> Self {
        WGSLParam {
            name: name.into(),
            ty,
            builtin: None,
            location: Some(loc),
        }
    }
    /// Emit the parameter declaration (no trailing comma).
    pub fn emit(&self) -> String {
        let mut attrs = String::new();
        if let Some(b) = &self.builtin {
            attrs.push_str(&format!("@builtin({}) ", b));
        }
        if let Some(loc) = self.location {
            attrs.push_str(&format!("@location({}) ", loc));
        }
        format!("{}{}: {}", attrs, self.name, self.ty)
    }
}
/// The entry-point type for a WGSL function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WGSLEntryPoint {
    /// Not an entry point (helper function).
    None,
    /// `@vertex`
    Vertex,
    /// `@fragment`
    Fragment,
    /// `@compute @workgroup_size(x, y, z)`
    Compute { x: u32, y: u32, z: u32 },
}
impl WGSLEntryPoint {
    /// Emit the attribute line(s) preceding the `fn` keyword.
    pub fn attribute(&self) -> String {
        match self {
            WGSLEntryPoint::None => String::new(),
            WGSLEntryPoint::Vertex => "@vertex\n".into(),
            WGSLEntryPoint::Fragment => "@fragment\n".into(),
            WGSLEntryPoint::Compute { x, y, z } => {
                format!("@compute @workgroup_size({}, {}, {})\n", x, y, z)
            }
        }
    }
}
/// WGSL type system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WGSLType {
    /// `bool`
    Bool,
    /// `i32`
    I32,
    /// `u32`
    U32,
    /// `f32`
    F32,
    /// `f16` (requires `enable f16;`)
    F16,
    /// `vec2<f32>`
    Vec2f,
    /// `vec3<f32>`
    Vec3f,
    /// `vec4<f32>`
    Vec4f,
    /// `vec2<i32>`
    Vec2i,
    /// `vec3<i32>`
    Vec3i,
    /// `vec4<i32>`
    Vec4i,
    /// `vec2<u32>`
    Vec2u,
    /// `vec3<u32>`
    Vec3u,
    /// `vec4<u32>`
    Vec4u,
    /// `vec2<bool>`
    Vec2b,
    /// `mat2x2<f32>`
    Mat2x2f,
    /// `mat3x3<f32>`
    Mat3x3f,
    /// `mat4x4<f32>`
    Mat4x4f,
    /// `mat2x4<f32>`
    Mat2x4f,
    /// `mat4x2<f32>`
    Mat4x2f,
    /// `texture_2d<f32>`
    Texture2D,
    /// `texture_2d_array<f32>`
    Texture2DArray,
    /// `texture_cube<f32>`
    TextureCube,
    /// `texture_3d<f32>`
    Texture3D,
    /// `texture_depth_2d`
    TextureDepth2D,
    /// `texture_storage_2d<rgba8unorm, write>`
    TextureStorage2D { format: String, access: String },
    /// `texture_multisampled_2d<f32>`
    TextureMultisampled2D,
    /// `sampler`
    Sampler,
    /// `sampler_comparison`
    SamplerComparison,
    /// A named struct type.
    Struct(String),
    /// A fixed-length array: `array<T, N>`.
    Array(Box<WGSLType>, u32),
    /// A runtime-sized array: `array<T>`.
    RuntimeArray(Box<WGSLType>),
    /// A pointer type: `ptr<address_space, T>`.
    Ptr {
        address_space: WGSLAddressSpace,
        inner: Box<WGSLType>,
    },
    /// `atomic<u32>`
    AtomicU32,
    /// `atomic<i32>`
    AtomicI32,
}
impl WGSLType {
    /// Return the WGSL source representation of this type.
    pub fn keyword(&self) -> String {
        match self {
            WGSLType::Bool => "bool".into(),
            WGSLType::I32 => "i32".into(),
            WGSLType::U32 => "u32".into(),
            WGSLType::F32 => "f32".into(),
            WGSLType::F16 => "f16".into(),
            WGSLType::Vec2f => "vec2<f32>".into(),
            WGSLType::Vec3f => "vec3<f32>".into(),
            WGSLType::Vec4f => "vec4<f32>".into(),
            WGSLType::Vec2i => "vec2<i32>".into(),
            WGSLType::Vec3i => "vec3<i32>".into(),
            WGSLType::Vec4i => "vec4<i32>".into(),
            WGSLType::Vec2u => "vec2<u32>".into(),
            WGSLType::Vec3u => "vec3<u32>".into(),
            WGSLType::Vec4u => "vec4<u32>".into(),
            WGSLType::Vec2b => "vec2<bool>".into(),
            WGSLType::Mat2x2f => "mat2x2<f32>".into(),
            WGSLType::Mat3x3f => "mat3x3<f32>".into(),
            WGSLType::Mat4x4f => "mat4x4<f32>".into(),
            WGSLType::Mat2x4f => "mat2x4<f32>".into(),
            WGSLType::Mat4x2f => "mat4x2<f32>".into(),
            WGSLType::Texture2D => "texture_2d<f32>".into(),
            WGSLType::Texture2DArray => "texture_2d_array<f32>".into(),
            WGSLType::TextureCube => "texture_cube<f32>".into(),
            WGSLType::Texture3D => "texture_3d<f32>".into(),
            WGSLType::TextureDepth2D => "texture_depth_2d".into(),
            WGSLType::TextureStorage2D { format, access } => {
                format!("texture_storage_2d<{}, {}>", format, access)
            }
            WGSLType::TextureMultisampled2D => "texture_multisampled_2d<f32>".into(),
            WGSLType::Sampler => "sampler".into(),
            WGSLType::SamplerComparison => "sampler_comparison".into(),
            WGSLType::Struct(name) => name.clone(),
            WGSLType::Array(elem, n) => format!("array<{}, {}>", elem.keyword(), n),
            WGSLType::RuntimeArray(elem) => format!("array<{}>", elem.keyword()),
            WGSLType::Ptr {
                address_space,
                inner,
            } => {
                format!("ptr<{}, {}>", address_space, inner.keyword())
            }
            WGSLType::AtomicU32 => "atomic<u32>".into(),
            WGSLType::AtomicI32 => "atomic<i32>".into(),
        }
    }
    /// Return true for opaque texture/sampler types.
    pub fn is_opaque(&self) -> bool {
        matches!(
            self,
            WGSLType::Texture2D
                | WGSLType::Texture2DArray
                | WGSLType::TextureCube
                | WGSLType::Texture3D
                | WGSLType::TextureDepth2D
                | WGSLType::TextureStorage2D { .. }
                | WGSLType::TextureMultisampled2D
                | WGSLType::Sampler
                | WGSLType::SamplerComparison
        )
    }
    /// Return true for floating-point scalar and vector types.
    pub fn is_float_like(&self) -> bool {
        matches!(
            self,
            WGSLType::F32 | WGSLType::F16 | WGSLType::Vec2f | WGSLType::Vec3f | WGSLType::Vec4f
        )
    }
}
/// A `@group(G) @binding(B)` resource variable.
#[derive(Debug, Clone)]
pub struct WGSLBinding {
    /// Bind group index.
    pub group: u32,
    /// Binding slot within the group.
    pub binding: u32,
    /// Variable name.
    pub name: String,
    /// Resource type.
    pub ty: WGSLType,
    /// Optional access mode (for storage buffers).
    pub access: Option<WGSLAccess>,
}
impl WGSLBinding {
    /// Create a new binding with no access mode.
    pub fn new(group: u32, binding: u32, name: impl Into<String>, ty: WGSLType) -> Self {
        WGSLBinding {
            group,
            binding,
            name: name.into(),
            ty,
            access: None,
        }
    }
    /// Create a storage binding with an explicit access mode.
    pub fn storage(
        group: u32,
        binding: u32,
        name: impl Into<String>,
        ty: WGSLType,
        access: WGSLAccess,
    ) -> Self {
        WGSLBinding {
            group,
            binding,
            name: name.into(),
            ty,
            access: Some(access),
        }
    }
    /// Emit the `@group(…) @binding(…) var …` declaration.
    pub fn emit(&self) -> String {
        let access_str = match &self.access {
            Some(a) => format!("<{}>", a),
            None => String::new(),
        };
        format!(
            "@group({}) @binding({}) var{} {}: {};",
            self.group, self.binding, access_str, self.name, self.ty
        )
    }
}
/// High-level compute kernel builder.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct WGSLComputeKernel {
    /// Kernel parameters.
    pub params: WGSLComputeKernelParams,
    /// Bindings used by this kernel.
    pub bindings: Vec<WGSLBinding>,
    /// Shared (workgroup) memory declarations.
    pub shared_vars: Vec<WGSLGlobal>,
    /// Body statements.
    pub body: Vec<WGSLStatement>,
}
impl WGSLComputeKernel {
    /// Create a new compute kernel.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, wg_x: u32, wg_y: u32, wg_z: u32) -> Self {
        WGSLComputeKernel {
            params: WGSLComputeKernelParams {
                name: name.into(),
                wg_x,
                wg_y,
                wg_z,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    /// Add a body statement.
    #[allow(dead_code)]
    pub fn push(&mut self, stmt: WGSLStatement) {
        self.body.push(stmt);
    }
    /// Add a workgroup-shared array.
    #[allow(dead_code)]
    pub fn add_shared_array(&mut self, name: impl Into<String>, elem_ty: WGSLType, size: u32) {
        self.shared_vars.push(WGSLGlobal::workgroup(
            name,
            WGSLType::Array(Box::new(elem_ty), size),
        ));
    }
    /// Emit the kernel as a WGSLFunction ready to be added to a shader.
    #[allow(dead_code)]
    pub fn emit_function(&self) -> WGSLFunction {
        let p = &self.params;
        let mut func = WGSLFunction::compute(&p.name, p.wg_x, p.wg_y, p.wg_z);
        func.add_param(WGSLParam {
            name: "global_id".to_string(),
            ty: WGSLType::Vec3u,
            builtin: Some("global_invocation_id".to_string()),
            location: None,
        });
        if p.use_local_id {
            func.add_param(WGSLParam {
                name: "local_id".to_string(),
                ty: WGSLType::Vec3u,
                builtin: Some("local_invocation_id".to_string()),
                location: None,
            });
        }
        if p.use_workgroup_id {
            func.add_param(WGSLParam {
                name: "wg_id".to_string(),
                ty: WGSLType::Vec3u,
                builtin: Some("workgroup_id".to_string()),
                location: None,
            });
        }
        if p.use_num_workgroups {
            func.add_param(WGSLParam {
                name: "num_wgs".to_string(),
                ty: WGSLType::Vec3u,
                builtin: Some("num_workgroups".to_string()),
                location: None,
            });
        }
        for stmt in &self.body {
            func.add_statement(stmt.emit(0));
        }
        func
    }
    /// Emit the kernel as a complete WGSL shader module.
    #[allow(dead_code)]
    pub fn emit_shader(&self) -> String {
        let mut shader = WGSLShader::new();
        for b in &self.bindings {
            shader.add_binding(b.clone());
        }
        for g in &self.shared_vars {
            shader.add_global(g.clone());
        }
        shader.add_function(self.emit_function());
        WGSLBackend::new().emit_shader(&shader)
    }
}
/// Collection of reusable WGSL code snippets for common algorithms.
#[allow(dead_code)]
pub struct WGSLSnippets;
impl WGSLSnippets {
    /// Emit a linear map from [in_lo, in_hi] to [out_lo, out_hi].
    #[allow(dead_code)]
    pub fn linear_map(val: &str, in_lo: f32, in_hi: f32, out_lo: f32, out_hi: f32) -> String {
        format!(
            "mix({out_lo}, {out_hi}, ({val} - {in_lo}) / ({in_hi} - {in_lo}))",
            val = val,
            in_lo = in_lo,
            in_hi = in_hi,
            out_lo = out_lo,
            out_hi = out_hi,
        )
    }
    /// Emit a 2D rotation matrix applied to a vec2f.
    #[allow(dead_code)]
    pub fn rotate2d(v: &str, angle: &str) -> String {
        format!(
            "vec2f(cos({a}) * {v}.x - sin({a}) * {v}.y, sin({a}) * {v}.x + cos({a}) * {v}.y)",
            v = v,
            a = angle
        )
    }
    /// Emit an sRGB gamma correction (linear -> sRGB).
    #[allow(dead_code)]
    pub fn linear_to_srgb(c: &str) -> String {
        format!(
            "select({c} * 12.92, pow({c}, vec4f(1.0 / 2.4)) * 1.055 - vec4f(0.055), {c} <= vec4f(0.0031308))",
            c = c
        )
    }
    /// Emit an sRGB -> linear conversion.
    #[allow(dead_code)]
    pub fn srgb_to_linear(c: &str) -> String {
        format!(
            "select({c} / 12.92, pow(({c} + vec4f(0.055)) / vec4f(1.055), vec4f(2.4)), {c} <= vec4f(0.04045))",
            c = c
        )
    }
    /// Emit a Blinn-Phong specular term.
    #[allow(dead_code)]
    pub fn blinn_phong(normal: &str, halfway: &str, shininess: &str) -> String {
        format!(
            "pow(max(dot({n}, {h}), 0.0), {s})",
            n = normal,
            h = halfway,
            s = shininess
        )
    }
    /// Emit a simple hash function for a u32 (Wang hash).
    #[allow(dead_code)]
    pub fn wang_hash(seed: &str) -> String {
        format!(
            "(({s} ^ 61u) ^ ({s} >> 16u)) * 9u ^ (({s} ^ 61u) ^ ({s} >> 16u)) >> 4u ^ (({s} ^ 61u) ^ ({s} >> 16u)) * 0x27d4eb2du",
            s = seed
        )
    }
    /// Emit a PCG random number step.
    #[allow(dead_code)]
    pub fn pcg_next(state: &str) -> String {
        format!(
            "let _pcg_tmp = {s} * 747796405u + 2891336453u; let _pcg_word = ((_pcg_tmp >> ((_pcg_tmp >> 28u) + 4u)) ^ _pcg_tmp) * 277803737u; (_pcg_word >> 22u) ^ _pcg_word",
            s = state
        )
    }
    /// Emit an RGB-to-HSV conversion expression for a vec3f.
    #[allow(dead_code)]
    pub fn rgb_to_hsv_fn() -> String {
        r"fn rgb_to_hsv(c: vec3f) -> vec3f {
    let k = vec4f(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    let p = mix(vec4f(c.bg, k.wz), vec4f(c.gb, k.xy), step(c.b, c.g));
    let q = mix(vec4f(p.xyw, c.r), vec4f(c.r, p.yzx), step(p.x, c.r));
    let d = q.x - min(q.w, q.y);
    let e = 1.0e-10;
    return vec3f(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}"
        .to_string()
    }
    /// Emit an HSV-to-RGB conversion function.
    #[allow(dead_code)]
    pub fn hsv_to_rgb_fn() -> String {
        r"fn hsv_to_rgb(c: vec3f) -> vec3f {
    let k = vec4f(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p = abs(fract(c.xxx + k.xyz) * 6.0 - k.www);
    return c.z * mix(k.xxx, clamp(p - k.xxx, vec3f(0.0), vec3f(1.0)), c.y);
}"
        .to_string()
    }
    /// Emit a Gaussian blur weight at offset `i` with standard deviation `sigma`.
    #[allow(dead_code)]
    pub fn gaussian_weight(i: i32, sigma: f32) -> f32 {
        let x = i as f32;
        let denom = (2.0 * std::f32::consts::PI * sigma * sigma).sqrt();
        (-(x * x) / (2.0 * sigma * sigma)).exp() / denom
    }
    /// Emit a separable Gaussian blur kernel helper function.
    #[allow(dead_code)]
    pub fn gaussian_blur_fn(radius: i32, sigma: f32, horizontal: bool) -> String {
        let dir = if horizontal {
            "vec2f(1.0, 0.0)"
        } else {
            "vec2f(0.0, 1.0)"
        };
        let weights: Vec<f32> = (-radius..=radius)
            .map(|i| Self::gaussian_weight(i, sigma))
            .collect();
        let total: f32 = weights.iter().sum();
        let norm_weights: Vec<f32> = weights.iter().map(|w| w / total).collect();
        let mut body = format!(
            "fn gaussian_blur_{}(tex: texture_2d<f32>, samp: sampler, uv: vec2f, texel_size: vec2f) -> vec4f {{\n",
            if horizontal { "h" } else { "v" }
        );
        body.push_str("    var result = vec4f(0.0);\n");
        for (idx, i) in (-radius..=radius).enumerate() {
            body.push_str(&format!(
                "    result += textureSample(tex, samp, uv + {} * {} * texel_size) * {}f;\n",
                dir, i, norm_weights[idx]
            ));
        }
        body.push_str("    return result;\n}");
        body
    }
}
/// Which shader stages can see a resource.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum WGSLStageVisibility {
    Vertex,
    Fragment,
    Compute,
    VertexFragment,
    All,
}
/// Describes a single binding entry in a binding group layout.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WGSLBindingEntry {
    /// Binding slot index within the group.
    pub binding: u32,
    /// The type of resource at this binding.
    pub resource_type: WGSLResourceType,
    /// Visibility: which shader stages can access this binding.
    pub visibility: WGSLStageVisibility,
}
/// A field inside a WGSL `struct`.
#[derive(Debug, Clone)]
pub struct WGSLStructField {
    /// Field name.
    pub name: String,
    /// Field type.
    pub ty: WGSLType,
    /// Optional built-in attribute (e.g. `position`, `vertex_index`).
    pub builtin: Option<String>,
    /// Optional `@location(N)` attribute.
    pub location: Option<u32>,
    /// Optional interpolation attribute (e.g. `flat`).
    pub interpolate: Option<String>,
}
impl WGSLStructField {
    /// Create a plain field with no attributes.
    pub fn new(name: impl Into<String>, ty: WGSLType) -> Self {
        WGSLStructField {
            name: name.into(),
            ty,
            builtin: None,
            location: None,
            interpolate: None,
        }
    }
    /// Create a field with a `@builtin(…)` attribute.
    pub fn builtin(name: impl Into<String>, ty: WGSLType, builtin: impl Into<String>) -> Self {
        WGSLStructField {
            name: name.into(),
            ty,
            builtin: Some(builtin.into()),
            location: None,
            interpolate: None,
        }
    }
    /// Create a field with a `@location(N)` attribute.
    pub fn location(name: impl Into<String>, ty: WGSLType, loc: u32) -> Self {
        WGSLStructField {
            name: name.into(),
            ty,
            builtin: None,
            location: Some(loc),
            interpolate: None,
        }
    }
    /// Emit the field declaration with its attributes.
    pub fn emit(&self) -> String {
        let mut attrs = String::new();
        if let Some(b) = &self.builtin {
            attrs.push_str(&format!("@builtin({}) ", b));
        }
        if let Some(loc) = self.location {
            attrs.push_str(&format!("@location({}) ", loc));
        }
        if let Some(interp) = &self.interpolate {
            attrs.push_str(&format!("@interpolate({}) ", interp));
        }
        format!("    {}{}: {},", attrs, self.name, self.ty)
    }
}
/// Metrics about a WGSL shader module.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct WGSLCodeMetrics {
    /// Total number of functions.
    pub num_functions: usize,
    /// Number of entry-point functions.
    pub num_entry_points: usize,
    /// Total number of struct definitions.
    pub num_structs: usize,
    /// Total number of resource bindings.
    pub num_bindings: usize,
    /// Total number of module-scope variables.
    pub num_globals: usize,
    /// Total number of constants.
    pub num_constants: usize,
    /// Total number of overrides.
    pub num_overrides: usize,
    /// Approximate number of statements across all functions.
    pub total_statements: usize,
    /// Number of enable extensions.
    pub num_enables: usize,
}
impl WGSLCodeMetrics {
    /// Compute metrics from a shader.
    #[allow(dead_code)]
    pub fn compute(shader: &WGSLShader) -> Self {
        let num_entry_points = shader
            .functions
            .iter()
            .filter(|f| !matches!(f.entry_point, WGSLEntryPoint::None))
            .count();
        let total_statements = shader.functions.iter().map(|f| f.body.len()).sum();
        WGSLCodeMetrics {
            num_functions: shader.functions.len(),
            num_entry_points,
            num_structs: shader.structs.len(),
            num_bindings: shader.bindings.len(),
            num_globals: shader.globals.len(),
            num_constants: shader.constants.len(),
            num_overrides: shader.overrides.len(),
            total_statements,
            num_enables: shader.enables.len(),
        }
    }
    /// Return a human-readable summary of the metrics.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "functions={} entry_points={} structs={} bindings={} globals={} constants={} overrides={} statements={} enables={}",
            self.num_functions, self.num_entry_points, self.num_structs, self
            .num_bindings, self.num_globals, self.num_constants, self.num_overrides, self
            .total_statements, self.num_enables,
        )
    }
}
/// Resource access mode for storage buffers and storage textures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WGSLAccess {
    Read,
    Write,
    ReadWrite,
}
/// A WGSL function (helper or entry point).
#[derive(Debug, Clone)]
pub struct WGSLFunction {
    /// Function name.
    pub name: String,
    /// Entry-point kind (or `None` for helpers).
    pub entry_point: WGSLEntryPoint,
    /// Ordered parameters.
    pub params: Vec<WGSLParam>,
    /// Return type (`None` means no return value / void).
    pub return_type: Option<WGSLType>,
    /// Optional attribute on the return type.
    pub return_attrib: WGSLReturnAttrib,
    /// Body statements.
    pub body: Vec<String>,
}
impl WGSLFunction {
    /// Create a helper function with no entry-point annotation.
    pub fn helper(name: impl Into<String>) -> Self {
        WGSLFunction {
            name: name.into(),
            entry_point: WGSLEntryPoint::None,
            params: Vec::new(),
            return_type: None,
            return_attrib: WGSLReturnAttrib::None,
            body: Vec::new(),
        }
    }
    /// Create a `@vertex` entry-point function.
    pub fn vertex(name: impl Into<String>) -> Self {
        WGSLFunction {
            name: name.into(),
            entry_point: WGSLEntryPoint::Vertex,
            params: Vec::new(),
            return_type: None,
            return_attrib: WGSLReturnAttrib::None,
            body: Vec::new(),
        }
    }
    /// Create a `@fragment` entry-point function.
    pub fn fragment(name: impl Into<String>) -> Self {
        WGSLFunction {
            name: name.into(),
            entry_point: WGSLEntryPoint::Fragment,
            params: Vec::new(),
            return_type: None,
            return_attrib: WGSLReturnAttrib::None,
            body: Vec::new(),
        }
    }
    /// Create a `@compute @workgroup_size(x, y, z)` entry-point function.
    pub fn compute(name: impl Into<String>, x: u32, y: u32, z: u32) -> Self {
        WGSLFunction {
            name: name.into(),
            entry_point: WGSLEntryPoint::Compute { x, y, z },
            params: Vec::new(),
            return_type: None,
            return_attrib: WGSLReturnAttrib::None,
            body: Vec::new(),
        }
    }
    /// Add a parameter.
    pub fn add_param(&mut self, param: WGSLParam) {
        self.params.push(param);
    }
    /// Set the return type.
    pub fn set_return_type(&mut self, ty: WGSLType) {
        self.return_type = Some(ty);
    }
    /// Set the return type with an attribute.
    pub fn set_return_type_with_attrib(&mut self, ty: WGSLType, attrib: WGSLReturnAttrib) {
        self.return_type = Some(ty);
        self.return_attrib = attrib;
    }
    /// Append a body statement (without trailing semicolon — the emitter adds it).
    pub fn add_statement(&mut self, stmt: impl Into<String>) {
        self.body.push(stmt.into());
    }
    /// Emit the complete function definition.
    pub fn emit(&self) -> String {
        let mut out = self.entry_point.attribute();
        let params: Vec<String> = self.params.iter().map(|p| p.emit()).collect();
        let ret = match &self.return_type {
            Some(ty) => format!(" -> {}{}", self.return_attrib.prefix(), ty),
            None => String::new(),
        };
        out.push_str(&format!(
            "fn {}({}){} {{\n",
            self.name,
            params.join(", "),
            ret
        ));
        for stmt in &self.body {
            out.push_str(&format!("    {};\n", stmt));
        }
        out.push('}');
        out
    }
}
