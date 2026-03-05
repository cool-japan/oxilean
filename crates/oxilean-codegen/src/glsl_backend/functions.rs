//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    GLSLAnalysisCache, GLSLBackend, GLSLConstantFoldingHelper, GLSLDepGraph, GLSLDominatorTree,
    GLSLExpr, GLSLExtCache, GLSLExtConstFolder, GLSLExtDepGraph, GLSLExtDomTree, GLSLExtLiveness,
    GLSLExtPassConfig, GLSLExtPassPhase, GLSLExtPassRegistry, GLSLExtPassStats, GLSLExtWorklist,
    GLSLFunction, GLSLLivenessInfo, GLSLPassConfig, GLSLPassPhase, GLSLPassRegistry, GLSLPassStats,
    GLSLQualifier, GLSLShader, GLSLShaderStage, GLSLStruct, GLSLType, GLSLVariable, GLSLVersion,
    GLSLWorklist, GlslBlockMember, GlslBuiltinCategory, GlslBuiltinFn, GlslComputeWorkgroup,
    GlslConstant, GlslIncludeGuard, GlslMacroExpander, GlslOutputVariableSet, GlslSwizzleValidator,
    GlslTypeInference, GlslUniformBlock,
};

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn backend_330() -> GLSLBackend {
        GLSLBackend::new(GLSLVersion::V330)
    }
    pub(super) fn backend_120() -> GLSLBackend {
        GLSLBackend::new(GLSLVersion::V120)
    }
    pub(super) fn backend_450() -> GLSLBackend {
        GLSLBackend::new(GLSLVersion::V450)
    }
    #[test]
    pub(super) fn test_version_numbers() {
        assert_eq!(GLSLVersion::V120.number(), 120);
        assert_eq!(GLSLVersion::V330.number(), 330);
        assert_eq!(GLSLVersion::V450.number(), 450);
        assert_eq!(GLSLVersion::V460.number(), 460);
    }
    #[test]
    pub(super) fn test_version_line_120() {
        let line = GLSLVersion::V120.version_line();
        assert_eq!(line, "#version 120");
    }
    #[test]
    pub(super) fn test_version_line_330() {
        let line = GLSLVersion::V330.version_line();
        assert_eq!(line, "#version 330 core");
    }
    #[test]
    pub(super) fn test_version_supports_layout() {
        assert!(!GLSLVersion::V120.supports_layout_location());
        assert!(GLSLVersion::V330.supports_layout_location());
        assert!(GLSLVersion::V450.supports_layout_location());
    }
    #[test]
    pub(super) fn test_version_supports_compute() {
        assert!(!GLSLVersion::V330.supports_compute());
        assert!(GLSLVersion::V450.supports_compute());
        assert!(GLSLVersion::V460.supports_compute());
    }
    #[test]
    pub(super) fn test_type_keywords() {
        assert_eq!(GLSLType::Float.keyword(), "float");
        assert_eq!(GLSLType::Vec3.keyword(), "vec3");
        assert_eq!(GLSLType::Mat4.keyword(), "mat4");
        assert_eq!(GLSLType::Sampler2D.keyword(), "sampler2D");
        assert_eq!(GLSLType::Uint.keyword(), "uint");
    }
    #[test]
    pub(super) fn test_type_is_opaque() {
        assert!(GLSLType::Sampler2D.is_opaque());
        assert!(GLSLType::Image2D.is_opaque());
        assert!(!GLSLType::Vec4.is_opaque());
        assert!(!GLSLType::Float.is_opaque());
    }
    #[test]
    pub(super) fn test_type_component_count() {
        assert_eq!(GLSLType::Vec2.component_count(), 2);
        assert_eq!(GLSLType::Vec4.component_count(), 4);
        assert_eq!(GLSLType::Float.component_count(), 1);
        assert_eq!(GLSLType::Mat4.component_count(), 4);
    }
    #[test]
    pub(super) fn test_array_type_display() {
        let arr = GLSLType::Array(Box::new(GLSLType::Float), 4);
        assert_eq!(format!("{}", arr), "float[4]");
    }
    #[test]
    pub(super) fn test_struct_type_keyword() {
        let s = GLSLType::Struct("Light".into());
        assert_eq!(s.keyword(), "Light");
    }
    #[test]
    pub(super) fn test_variable_emit_global_uniform() {
        let v = GLSLVariable::uniform("uMVP", GLSLType::Mat4);
        assert_eq!(v.emit_global(), "uniform mat4 uMVP;");
    }
    #[test]
    pub(super) fn test_variable_emit_global_input() {
        let v = GLSLVariable::input("aPosition", GLSLType::Vec3);
        assert_eq!(v.emit_global(), "in vec3 aPosition;");
    }
    #[test]
    pub(super) fn test_variable_emit_global_output() {
        let v = GLSLVariable::output("fragColor", GLSLType::Vec4);
        assert_eq!(v.emit_global(), "out vec4 fragColor;");
    }
    #[test]
    pub(super) fn test_variable_emit_global_layout_input() {
        let v = GLSLVariable::layout_input("aPos", GLSLType::Vec3, 0);
        assert_eq!(v.emit_global(), "layout(location = 0) in vec3 aPos;");
    }
    #[test]
    pub(super) fn test_variable_emit_param() {
        let v = GLSLVariable::new("n", GLSLType::Vec3, GLSLQualifier::InOut);
        assert_eq!(v.emit_param(), "inout vec3 n");
    }
    #[test]
    pub(super) fn test_struct_emit() {
        let mut s = GLSLStruct::new("Light");
        s.add_field("position", GLSLType::Vec3);
        s.add_field("color", GLSLType::Vec3);
        let emitted = s.emit();
        assert!(emitted.contains("struct Light {"));
        assert!(emitted.contains("vec3 position;"));
        assert!(emitted.contains("vec3 color;"));
        assert!(emitted.ends_with("};"));
    }
    #[test]
    pub(super) fn test_function_emit_empty() {
        let f = GLSLFunction::new("main", GLSLType::Void);
        let emitted = f.emit();
        assert!(emitted.contains("void main()"));
        assert!(emitted.contains("{}") || emitted.contains("{\n}"));
    }
    #[test]
    pub(super) fn test_function_emit_with_body() {
        let mut f = GLSLFunction::new("square", GLSLType::Float);
        f.add_param(GLSLVariable::new("x", GLSLType::Float, GLSLQualifier::None));
        f.add_statement("return x * x");
        let emitted = f.emit();
        assert!(emitted.contains("float square(float x)"));
        assert!(emitted.contains("return x * x;"));
    }
    #[test]
    pub(super) fn test_function_prototype() {
        let mut f = GLSLFunction::new("dot2", GLSLType::Float);
        f.add_param(GLSLVariable::new("a", GLSLType::Vec3, GLSLQualifier::None));
        f.add_param(GLSLVariable::new("b", GLSLType::Vec3, GLSLQualifier::None));
        let proto = f.emit_prototype();
        assert_eq!(proto, "float dot2(vec3 a, vec3 b);");
    }
    #[test]
    pub(super) fn test_emit_type() {
        let b = backend_330();
        assert_eq!(b.emit_type(&GLSLType::Vec4), "vec4");
        assert_eq!(b.emit_type(&GLSLType::Sampler2D), "sampler2D");
    }
    #[test]
    pub(super) fn test_vertex_shader_template_330() {
        let src = backend_330().vertex_shader_template();
        assert!(src.contains("#version 330 core"), "version line missing");
        assert!(src.contains("uMVP"), "MVP uniform missing");
        assert!(src.contains("gl_Position"), "position write missing");
        assert!(
            src.contains("layout(location = 0)"),
            "layout qualifier missing"
        );
    }
    #[test]
    pub(super) fn test_vertex_shader_template_120() {
        let src = backend_120().vertex_shader_template();
        assert!(src.contains("#version 120"), "version line missing");
        assert!(src.contains("uMVP"), "MVP uniform missing");
    }
    #[test]
    pub(super) fn test_fragment_shader_template() {
        let src = backend_330().fragment_shader_template();
        assert!(src.contains("#version 330 core"));
        assert!(src.contains("sampler2D uTexture"));
        assert!(src.contains("fragColor"));
        assert!(src.contains("texture(uTexture"));
    }
    #[test]
    pub(super) fn test_phong_vertex_template() {
        let src = backend_450().phong_vertex_template();
        assert!(src.contains("aNormal"));
        assert!(src.contains("uModel"));
        assert!(src.contains("uView"));
        assert!(src.contains("uProjection"));
        assert!(src.contains("gl_Position"));
    }
    #[test]
    pub(super) fn test_phong_fragment_template() {
        let src = backend_450().phong_fragment_template();
        assert!(src.contains("uLightPos"));
        assert!(src.contains("uViewPos"));
        assert!(src.contains("ambient"));
        assert!(src.contains("diffuse"));
        assert!(src.contains("specular"));
        assert!(src.contains("fragColor"));
    }
    #[test]
    pub(super) fn test_compute_shader_template() {
        let src = backend_450().compute_shader_template(64, 1, 1);
        assert!(src.contains("local_size_x = 64"));
        assert!(src.contains("local_size_y = 1"));
        assert!(src.contains("local_size_z = 1"));
        assert!(src.contains("gl_GlobalInvocationID"));
    }
    #[test]
    pub(super) fn test_expr_var() {
        let e = GLSLExpr::var("myVar");
        assert_eq!(e.emit(), "myVar");
    }
    #[test]
    pub(super) fn test_expr_float_literal() {
        let e = GLSLExpr::float(1.5);
        assert!(e.emit().starts_with("1.5"));
    }
    #[test]
    pub(super) fn test_expr_binop() {
        let e = GLSLExpr::binop("+", GLSLExpr::var("a"), GLSLExpr::var("b"));
        assert_eq!(e.emit(), "(a + b)");
    }
    #[test]
    pub(super) fn test_expr_call() {
        let e = GLSLExpr::call("normalize", vec![GLSLExpr::var("v")]);
        assert_eq!(e.emit(), "normalize(v)");
    }
    #[test]
    pub(super) fn test_expr_field() {
        let e = GLSLExpr::Field {
            base: Box::new(GLSLExpr::var("v")),
            field: "xyz".into(),
        };
        assert_eq!(e.emit(), "v.xyz");
    }
    #[test]
    pub(super) fn test_expr_index() {
        let e = GLSLExpr::Index {
            base: Box::new(GLSLExpr::var("arr")),
            index: Box::new(GLSLExpr::Literal("0".into())),
        };
        assert_eq!(e.emit(), "arr[0]");
    }
    #[test]
    pub(super) fn test_expr_ternary() {
        let e = GLSLExpr::Ternary {
            cond: Box::new(GLSLExpr::var("flag")),
            then_expr: Box::new(GLSLExpr::Literal("1.0".into())),
            else_expr: Box::new(GLSLExpr::Literal("0.0".into())),
        };
        assert_eq!(e.emit(), "(flag ? 1.0 : 0.0)");
    }
    #[test]
    pub(super) fn test_shader_with_extension() {
        let mut shader = GLSLShader::new(GLSLVersion::V450, GLSLShaderStage::Fragment);
        shader.add_extension("GL_ARB_bindless_texture");
        let src = backend_450().emit_shader(&shader);
        assert!(src.contains("#extension GL_ARB_bindless_texture : enable"));
    }
    #[test]
    pub(super) fn test_shader_with_define() {
        let mut shader = GLSLShader::new(GLSLVersion::V330, GLSLShaderStage::Vertex);
        shader.add_define("USE_LIGHTING");
        shader.add_define_value("MAX_LIGHTS", "8");
        let src = backend_330().emit_shader(&shader);
        assert!(src.contains("#define USE_LIGHTING"));
        assert!(src.contains("#define MAX_LIGHTS 8"));
    }
}
/// Catalogue of well-known GLSL built-in functions.
pub static GLSL_BUILTINS: &[GlslBuiltinFn] = &[
    GlslBuiltinFn::new("sin", GlslBuiltinCategory::Trigonometric, 110),
    GlslBuiltinFn::new("cos", GlslBuiltinCategory::Trigonometric, 110),
    GlslBuiltinFn::new("tan", GlslBuiltinCategory::Trigonometric, 110),
    GlslBuiltinFn::new("asin", GlslBuiltinCategory::Trigonometric, 110),
    GlslBuiltinFn::new("acos", GlslBuiltinCategory::Trigonometric, 110),
    GlslBuiltinFn::new("atan", GlslBuiltinCategory::Trigonometric, 110),
    GlslBuiltinFn::new("pow", GlslBuiltinCategory::Exponential, 110),
    GlslBuiltinFn::new("exp", GlslBuiltinCategory::Exponential, 110),
    GlslBuiltinFn::new("log", GlslBuiltinCategory::Exponential, 110),
    GlslBuiltinFn::new("sqrt", GlslBuiltinCategory::Exponential, 110),
    GlslBuiltinFn::new("inversesqrt", GlslBuiltinCategory::Exponential, 110),
    GlslBuiltinFn::new("abs", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("sign", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("floor", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("ceil", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("fract", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("min", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("max", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("clamp", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("mix", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("step", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("smoothstep", GlslBuiltinCategory::Common, 110),
    GlslBuiltinFn::new("length", GlslBuiltinCategory::GeometricVector, 110),
    GlslBuiltinFn::new("distance", GlslBuiltinCategory::GeometricVector, 110),
    GlslBuiltinFn::new("dot", GlslBuiltinCategory::GeometricVector, 110),
    GlslBuiltinFn::new("cross", GlslBuiltinCategory::GeometricVector, 110),
    GlslBuiltinFn::new("normalize", GlslBuiltinCategory::GeometricVector, 110),
    GlslBuiltinFn::new("reflect", GlslBuiltinCategory::GeometricVector, 110),
    GlslBuiltinFn::new("refract", GlslBuiltinCategory::GeometricVector, 110),
    GlslBuiltinFn::new("transpose", GlslBuiltinCategory::MatrixOp, 120),
    GlslBuiltinFn::new("determinant", GlslBuiltinCategory::MatrixOp, 150),
    GlslBuiltinFn::new("inverse", GlslBuiltinCategory::MatrixOp, 140),
    GlslBuiltinFn::new("texture", GlslBuiltinCategory::TextureSampling, 130),
    GlslBuiltinFn::new("atomicAdd", GlslBuiltinCategory::Atomic, 430),
    GlslBuiltinFn::new("atomicMin", GlslBuiltinCategory::Atomic, 430),
    GlslBuiltinFn::new("atomicMax", GlslBuiltinCategory::Atomic, 430),
    GlslBuiltinFn::new("atomicExchange", GlslBuiltinCategory::Atomic, 430),
    GlslBuiltinFn::new("atomicCompSwap", GlslBuiltinCategory::Atomic, 430),
];
/// Look up built-in functions available at a given GLSL version.
#[allow(dead_code)]
pub fn glsl_builtins_for_version(version: u32) -> Vec<&'static GlslBuiltinFn> {
    GLSL_BUILTINS
        .iter()
        .filter(|f| f.min_version <= version)
        .collect()
}
#[cfg(test)]
mod glsl_extra_tests {
    use super::*;
    #[test]
    pub(super) fn test_glsl_type_inference_bind_lookup() {
        let mut ctx = GlslTypeInference::new(GLSLVersion::V330);
        ctx.bind("pos", GLSLType::Vec4);
        assert_eq!(ctx.lookup("pos"), Some(&GLSLType::Vec4));
        assert_eq!(ctx.lookup("uv"), None);
        assert_eq!(ctx.num_bindings(), 1);
    }
    #[test]
    pub(super) fn test_glsl_constant_is_zero_one() {
        assert!(GlslConstant::Float(0.0).is_zero());
        assert!(!GlslConstant::Float(1.0).is_zero());
        assert!(GlslConstant::Int(1).is_one());
    }
    #[test]
    pub(super) fn test_glsl_constant_add() {
        let a = GlslConstant::Float(2.5);
        let b = GlslConstant::Float(1.5);
        assert_eq!(a.add(&b), Some(GlslConstant::Float(4.0)));
    }
    #[test]
    pub(super) fn test_glsl_uniform_block_emit() {
        let mut block = GlslUniformBlock::ubo("Matrices").with_binding(0);
        block.add_member(GlslBlockMember::new("model", GLSLType::Mat4));
        let src = block.emit();
        assert!(src.contains("uniform Matrices"));
        assert!(src.contains("mat4 model"));
        assert!(src.contains("binding = 0"));
        assert_eq!(block.num_members(), 1);
    }
    #[test]
    pub(super) fn test_glsl_compute_workgroup() {
        let wg = GlslComputeWorkgroup::planar(16, 16);
        assert_eq!(wg.total_threads(), 256);
        let layout = wg.emit_layout();
        assert!(layout.contains("local_size_x = 16"));
    }
    #[test]
    pub(super) fn test_glsl_macro_expander() {
        let mut exp = GlslMacroExpander::new();
        exp.define("PI", "3.14159");
        assert!(exp.is_defined("PI"));
        assert_eq!(exp.value("PI"), Some("3.14159"));
        let src = exp.emit_defines();
        assert!(src.contains("#define PI 3.14159"));
    }
    #[test]
    pub(super) fn test_glsl_swizzle_validator() {
        assert!(GlslSwizzleValidator::validate("xyz", 3).is_ok());
        assert!(GlslSwizzleValidator::validate("xw", 3).is_err());
        assert!(GlslSwizzleValidator::validate("", 4).is_err());
    }
    #[test]
    pub(super) fn test_glsl_output_variable_set() {
        let mut ovs = GlslOutputVariableSet::new();
        ovs.add("fragColor", GLSLType::Vec4, Some(0));
        assert_eq!(ovs.len(), 1);
        let src = ovs.emit(GLSLVersion::V330);
        assert!(src.contains("out vec4 fragColor"));
    }
    #[test]
    pub(super) fn test_glsl_include_guard() {
        let guard = GlslIncludeGuard::from_filename("utils.glsl");
        assert!(guard.open().contains("#ifndef"));
        assert!(guard.close().contains("#endif"));
    }
    #[test]
    pub(super) fn test_glsl_builtins_for_version() {
        let v110 = glsl_builtins_for_version(110);
        let v430 = glsl_builtins_for_version(430);
        assert!(v430.len() > v110.len());
        assert!(v110.iter().any(|f| f.name == "sin"));
        assert!(!v110.iter().any(|f| f.name == "atomicAdd"));
        assert!(v430.iter().any(|f| f.name == "atomicAdd"));
    }
}
#[cfg(test)]
mod GLSL_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = GLSLPassConfig::new("test_pass", GLSLPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = GLSLPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = GLSLPassRegistry::new();
        reg.register(GLSLPassConfig::new("pass_a", GLSLPassPhase::Analysis));
        reg.register(GLSLPassConfig::new("pass_b", GLSLPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = GLSLAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = GLSLWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = GLSLDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = GLSLLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(GLSLConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(GLSLConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(GLSLConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            GLSLConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(GLSLConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = GLSLDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
#[cfg(test)]
mod glslext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_glslext_phase_order() {
        assert_eq!(GLSLExtPassPhase::Early.order(), 0);
        assert_eq!(GLSLExtPassPhase::Middle.order(), 1);
        assert_eq!(GLSLExtPassPhase::Late.order(), 2);
        assert_eq!(GLSLExtPassPhase::Finalize.order(), 3);
        assert!(GLSLExtPassPhase::Early.is_early());
        assert!(!GLSLExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_glslext_config_builder() {
        let c = GLSLExtPassConfig::new("p")
            .with_phase(GLSLExtPassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_glslext_stats() {
        let mut s = GLSLExtPassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_glslext_registry() {
        let mut r = GLSLExtPassRegistry::new();
        r.register(GLSLExtPassConfig::new("a").with_phase(GLSLExtPassPhase::Early));
        r.register(GLSLExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&GLSLExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_glslext_cache() {
        let mut c = GLSLExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_glslext_worklist() {
        let mut w = GLSLExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_glslext_dom_tree() {
        let mut dt = GLSLExtDomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_glslext_liveness() {
        let mut lv = GLSLExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_glslext_const_folder() {
        let mut cf = GLSLExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_glslext_dep_graph() {
        let mut g = GLSLExtDepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
