//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    WGSLAccess, WGSLAddressSpace, WGSLBackend, WGSLBinding, WGSLConstant, WGSLExpr, WGSLFunction,
    WGSLGlobal, WGSLOverride, WGSLParam, WGSLShader, WGSLStruct, WGSLStructField, WGSLType,
};

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn backend() -> WGSLBackend {
        WGSLBackend::new()
    }
    #[test]
    pub(super) fn test_type_keywords_scalars() {
        assert_eq!(WGSLType::Bool.keyword(), "bool");
        assert_eq!(WGSLType::I32.keyword(), "i32");
        assert_eq!(WGSLType::U32.keyword(), "u32");
        assert_eq!(WGSLType::F32.keyword(), "f32");
        assert_eq!(WGSLType::F16.keyword(), "f16");
    }
    #[test]
    pub(super) fn test_type_keywords_vectors() {
        assert_eq!(WGSLType::Vec2f.keyword(), "vec2<f32>");
        assert_eq!(WGSLType::Vec3f.keyword(), "vec3<f32>");
        assert_eq!(WGSLType::Vec4f.keyword(), "vec4<f32>");
        assert_eq!(WGSLType::Vec2i.keyword(), "vec2<i32>");
        assert_eq!(WGSLType::Vec3u.keyword(), "vec3<u32>");
    }
    #[test]
    pub(super) fn test_type_keywords_matrices() {
        assert_eq!(WGSLType::Mat4x4f.keyword(), "mat4x4<f32>");
        assert_eq!(WGSLType::Mat2x2f.keyword(), "mat2x2<f32>");
        assert_eq!(WGSLType::Mat3x3f.keyword(), "mat3x3<f32>");
    }
    #[test]
    pub(super) fn test_type_keywords_texture_sampler() {
        assert_eq!(WGSLType::Texture2D.keyword(), "texture_2d<f32>");
        assert_eq!(WGSLType::Sampler.keyword(), "sampler");
        assert_eq!(WGSLType::SamplerComparison.keyword(), "sampler_comparison");
        assert_eq!(WGSLType::TextureDepth2D.keyword(), "texture_depth_2d");
    }
    #[test]
    pub(super) fn test_type_storage_texture() {
        let ty = WGSLType::TextureStorage2D {
            format: "rgba8unorm".into(),
            access: "write".into(),
        };
        assert_eq!(ty.keyword(), "texture_storage_2d<rgba8unorm, write>");
    }
    #[test]
    pub(super) fn test_type_array() {
        let ty = WGSLType::Array(Box::new(WGSLType::F32), 16);
        assert_eq!(ty.keyword(), "array<f32, 16>");
    }
    #[test]
    pub(super) fn test_type_runtime_array() {
        let ty = WGSLType::RuntimeArray(Box::new(WGSLType::U32));
        assert_eq!(ty.keyword(), "array<u32>");
    }
    #[test]
    pub(super) fn test_type_ptr() {
        let ty = WGSLType::Ptr {
            address_space: WGSLAddressSpace::Function,
            inner: Box::new(WGSLType::F32),
        };
        assert_eq!(ty.keyword(), "ptr<function, f32>");
    }
    #[test]
    pub(super) fn test_type_is_opaque() {
        assert!(WGSLType::Texture2D.is_opaque());
        assert!(WGSLType::Sampler.is_opaque());
        assert!(!WGSLType::F32.is_opaque());
        assert!(!WGSLType::Vec4f.is_opaque());
    }
    #[test]
    pub(super) fn test_type_is_float_like() {
        assert!(WGSLType::F32.is_float_like());
        assert!(WGSLType::Vec3f.is_float_like());
        assert!(!WGSLType::I32.is_float_like());
        assert!(!WGSLType::U32.is_float_like());
    }
    #[test]
    pub(super) fn test_binding_emit_texture() {
        let b = WGSLBinding::new(0, 0, "my_tex", WGSLType::Texture2D);
        let emitted = b.emit();
        assert_eq!(
            emitted,
            "@group(0) @binding(0) var my_tex: texture_2d<f32>;"
        );
    }
    #[test]
    pub(super) fn test_binding_emit_storage() {
        let b = WGSLBinding::storage(
            0,
            1,
            "buf",
            WGSLType::RuntimeArray(Box::new(WGSLType::F32)),
            WGSLAccess::ReadWrite,
        );
        let emitted = b.emit();
        assert!(emitted.contains("@group(0) @binding(1)"));
        assert!(emitted.contains("read_write"));
        assert!(emitted.contains("buf"));
    }
    #[test]
    pub(super) fn test_struct_emit() {
        let mut s = WGSLStruct::new("VertexOutput");
        s.add_field(WGSLStructField::builtin(
            "position",
            WGSLType::Vec4f,
            "position",
        ));
        s.add_field(WGSLStructField::location("color", WGSLType::Vec4f, 0));
        let emitted = s.emit();
        assert!(emitted.contains("struct VertexOutput {"));
        assert!(emitted.contains("@builtin(position)"));
        assert!(emitted.contains("@location(0)"));
    }
    #[test]
    pub(super) fn test_function_entry_point_vertex() {
        let f = WGSLFunction::vertex("vs_main");
        let emitted = f.emit();
        assert!(emitted.contains("@vertex"));
        assert!(emitted.contains("fn vs_main()"));
    }
    #[test]
    pub(super) fn test_function_entry_point_fragment() {
        let f = WGSLFunction::fragment("fs_main");
        let emitted = f.emit();
        assert!(emitted.contains("@fragment"));
        assert!(emitted.contains("fn fs_main()"));
    }
    #[test]
    pub(super) fn test_function_entry_point_compute() {
        let f = WGSLFunction::compute("main", 64, 1, 1);
        let emitted = f.emit();
        assert!(emitted.contains("@compute"));
        assert!(emitted.contains("@workgroup_size(64, 1, 1)"));
        assert!(emitted.contains("fn main()"));
    }
    #[test]
    pub(super) fn test_function_helper_with_body() {
        let mut f = WGSLFunction::helper("square");
        f.add_param(WGSLParam::new("x", WGSLType::F32));
        f.set_return_type(WGSLType::F32);
        f.add_statement("return x * x");
        let emitted = f.emit();
        assert!(emitted.contains("fn square(x: f32) -> f32"));
        assert!(emitted.contains("return x * x;"));
    }
    #[test]
    pub(super) fn test_constant_typed() {
        let c = WGSLConstant::typed("PI", WGSLType::F32, "3.14159265");
        assert_eq!(c.emit(), "const PI: f32 = 3.14159265;");
    }
    #[test]
    pub(super) fn test_constant_inferred() {
        let c = WGSLConstant::inferred("MAX_ITER", "100u");
        assert_eq!(c.emit(), "const MAX_ITER = 100u;");
    }
    #[test]
    pub(super) fn test_override_with_default() {
        let mut o = WGSLOverride::new("SAMPLE_COUNT", WGSLType::U32);
        o.id = Some(0);
        o.default_value = Some("4u".into());
        assert_eq!(o.emit(), "@id(0) override SAMPLE_COUNT: u32 = 4u;");
    }
    #[test]
    pub(super) fn test_global_private() {
        let g = WGSLGlobal::private("count", WGSLType::U32);
        assert_eq!(g.emit(), "var<private> count: u32;");
    }
    #[test]
    pub(super) fn test_global_workgroup() {
        let g = WGSLGlobal::workgroup("shared_mem", WGSLType::Array(Box::new(WGSLType::F32), 64));
        assert_eq!(g.emit(), "var<workgroup> shared_mem: array<f32, 64>;");
    }
    #[test]
    pub(super) fn test_triangle_shader_template() {
        let src = backend().triangle_shader_template();
        assert!(src.contains("@vertex"), "vertex entry point missing");
        assert!(src.contains("@fragment"), "fragment entry point missing");
        assert!(src.contains("VertexOutput"), "struct missing");
        assert!(
            src.contains("@builtin(position)"),
            "position builtin missing"
        );
        assert!(src.contains("fn vs_main"), "vertex fn missing");
        assert!(src.contains("fn fs_main"), "fragment fn missing");
    }
    #[test]
    pub(super) fn test_compute_shader_template() {
        let src = backend().compute_shader_template();
        assert!(src.contains("@compute"), "compute attribute missing");
        assert!(src.contains("@workgroup_size"), "workgroup_size missing");
        assert!(
            src.contains("global_invocation_id"),
            "builtin global_invocation_id missing"
        );
        assert!(src.contains("output_data"), "output buffer missing");
    }
    #[test]
    pub(super) fn test_texture_sample_template() {
        let src = backend().texture_sample_template();
        assert!(src.contains("t_diffuse"), "texture binding missing");
        assert!(src.contains("s_diffuse"), "sampler binding missing");
        assert!(src.contains("textureSample"), "textureSample call missing");
        assert!(src.contains("mat4x4<f32>"), "transform matrix missing");
    }
    #[test]
    pub(super) fn test_reduction_shader_template() {
        let src = backend().reduction_shader_template(256);
        assert!(src.contains("@compute"), "compute attribute missing");
        assert!(src.contains("workgroupBarrier"), "barrier missing");
        assert!(src.contains("shared_data"), "shared memory missing");
        assert!(src.contains("256"), "workgroup size missing");
    }
    #[test]
    pub(super) fn test_shader_with_enable() {
        let mut shader = WGSLShader::new();
        shader.add_enable("f16");
        let src = backend().emit_shader(&shader);
        assert!(src.contains("enable f16;"));
    }
    #[test]
    pub(super) fn test_expr_var() {
        let e = WGSLExpr::var("myVar");
        assert_eq!(e.emit(), "myVar");
    }
    #[test]
    pub(super) fn test_expr_f32_literal() {
        let e = WGSLExpr::f32_lit(2.5);
        assert!(e.emit().starts_with("2.5"));
    }
    #[test]
    pub(super) fn test_expr_u32_literal() {
        let e = WGSLExpr::u32_lit(42);
        assert_eq!(e.emit(), "42u");
    }
    #[test]
    pub(super) fn test_expr_binop() {
        let e = WGSLExpr::binop("*", WGSLExpr::var("a"), WGSLExpr::var("b"));
        assert_eq!(e.emit(), "(a * b)");
    }
    #[test]
    pub(super) fn test_expr_call() {
        let e = WGSLExpr::call("dot", vec![WGSLExpr::var("u"), WGSLExpr::var("v")]);
        assert_eq!(e.emit(), "dot(u, v)");
    }
    #[test]
    pub(super) fn test_expr_field() {
        let e = WGSLExpr::Field {
            base: Box::new(WGSLExpr::var("v")),
            field: "xyz".into(),
        };
        assert_eq!(e.emit(), "v.xyz");
    }
    #[test]
    pub(super) fn test_expr_index() {
        let e = WGSLExpr::Index {
            base: Box::new(WGSLExpr::var("arr")),
            index: Box::new(WGSLExpr::u32_lit(3)),
        };
        assert_eq!(e.emit(), "arr[3u]");
    }
    #[test]
    pub(super) fn test_address_space_display() {
        assert_eq!(format!("{}", WGSLAddressSpace::Workgroup), "workgroup");
        assert_eq!(format!("{}", WGSLAddressSpace::Storage), "storage");
        assert_eq!(format!("{}", WGSLAddressSpace::Uniform), "uniform");
    }
    #[test]
    pub(super) fn test_access_display() {
        assert_eq!(format!("{}", WGSLAccess::Read), "read");
        assert_eq!(format!("{}", WGSLAccess::Write), "write");
        assert_eq!(format!("{}", WGSLAccess::ReadWrite), "read_write");
    }
}
