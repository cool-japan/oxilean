//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ContinuityChecker, DiffDatabase, DiffRecord, ExtFunPropGoal, FunPropCache, FunPropConfig,
    FunPropDatabase, FunPropExtConfig200, FunPropExtConfig201, FunPropExtConfigVal200,
    FunPropExtConfigVal201, FunPropExtDiag200, FunPropExtDiag201, FunPropExtDiff200,
    FunPropExtDiff201, FunPropExtPass200, FunPropExtPass201, FunPropExtPipeline200,
    FunPropExtPipeline201, FunPropExtResult200, FunPropExtResult201, FunPropGoal, FunPropProof,
    FunPropRegistry, FunPropRule, FunPropSolver, FunPropStrength, FunPropTactic, FunPropTrace,
    FunProperty, LipschitzInfo, MeasDatabase, MeasRecord, MeasurabilityKind, PropLattice,
    TacticFunPropAnalysisPass, TacticFunPropConfig, TacticFunPropConfigValue,
    TacticFunPropDiagnostics, TacticFunPropDiff, TacticFunPropPipeline, TacticFunPropResult,
};
#[allow(unused_imports)]
use crate::basic::{MVarId, MetaContext};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::fun_prop::*;
    #[test]
    fn test_fun_property_variants() {
        assert_eq!(FunProperty::Continuous, FunProperty::Continuous);
        assert_ne!(FunProperty::Continuous, FunProperty::Measurable);
        assert_ne!(FunProperty::Monotone, FunProperty::Antitone);
        let all = [
            FunProperty::Continuous,
            FunProperty::Measurable,
            FunProperty::Differentiable,
            FunProperty::BoundedLinearMap,
            FunProperty::Monotone,
            FunProperty::Antitone,
            FunProperty::StrictMono,
            FunProperty::Injective,
            FunProperty::Surjective,
            FunProperty::Bijective,
        ];
        assert_eq!(all.len(), 10);
    }
    #[test]
    fn test_fun_prop_rule_new() {
        let rule = FunPropRule::new("continuity", FunProperty::Continuous);
        assert_eq!(rule.name, "continuity");
        assert_eq!(rule.property, FunProperty::Continuous);
        assert!(rule.applies_to.is_empty());
    }
    #[test]
    fn test_fun_prop_database_new() {
        let db = FunPropDatabase::new();
        assert!(db.rules.is_empty());
    }
    #[test]
    fn test_database_standard_rules() {
        let db = FunPropDatabase::with_standard_rules();
        assert!(db.can_prove(&FunProperty::Continuous, "id"));
        assert!(db.can_prove(&FunProperty::Measurable, "const"));
        assert!(db.can_prove(&FunProperty::Monotone, "id"));
        assert!(!db.find_rules(&FunProperty::Continuous).is_empty());
        assert!(!db.find_rules(&FunProperty::Measurable).is_empty());
    }
    #[test]
    fn test_can_prove() {
        let mut db = FunPropDatabase::new();
        let mut rule = FunPropRule::new("my_lemma", FunProperty::Injective);
        rule.add_target("f");
        db.add_rule(rule);
        assert!(db.can_prove(&FunProperty::Injective, "f"));
        assert!(!db.can_prove(&FunProperty::Injective, "g"));
        assert!(!db.can_prove(&FunProperty::Surjective, "f"));
    }
    #[test]
    fn test_tactic_prove_continuous() {
        let tac = FunPropTactic::with_db(FunPropDatabase::with_standard_rules());
        let proof = tac.prove_continuous("id");
        assert!(proof.is_some(), "expected a proof for Continuous id");
        let fail = tac.prove_continuous("unknown_fn_xyz");
        assert!(fail.is_none());
    }
    #[test]
    fn test_apply_composition_rule() {
        let tac = FunPropTactic::new();
        let r = tac.apply_composition_rule(FunProperty::Continuous, FunProperty::Continuous);
        assert_eq!(r, Some(FunProperty::Continuous));
        let r2 = tac.apply_composition_rule(FunProperty::Antitone, FunProperty::Antitone);
        assert_eq!(r2, Some(FunProperty::Monotone));
        let r3 = tac.apply_composition_rule(FunProperty::Surjective, FunProperty::Continuous);
        assert!(r3.is_none());
        let r4 = tac.apply_composition_rule(FunProperty::Injective, FunProperty::Injective);
        assert_eq!(r4, Some(FunProperty::Injective));
    }
    #[test]
    fn test_tactic_config() {
        let config = FunPropConfig {
            max_depth: 5,
            use_simp: false,
            verbose: true,
        };
        let tac = FunPropTactic::new().with_config(config.clone());
        assert_eq!(tac.config.max_depth, 5);
        assert!(!tac.config.use_simp);
        assert!(tac.config.verbose);
        let default_cfg = FunPropConfig::default();
        assert_eq!(default_cfg.max_depth, 10);
        assert!(default_cfg.use_simp);
        assert!(!default_cfg.verbose);
    }
}
/// Compose two function property strengths.
/// If f has property P and g has property Q, what does g∘f have?
#[allow(dead_code)]
pub fn compose_properties(f_prop: &FunPropStrength, g_prop: &FunPropStrength) -> FunPropStrength {
    if f_prop <= g_prop {
        f_prop.clone()
    } else {
        g_prop.clone()
    }
}
/// What is the property of f + g if f has p and g has q?
#[allow(dead_code)]
pub fn sum_properties(p: &FunPropStrength, q: &FunPropStrength) -> FunPropStrength {
    p.meets(q)
}
/// What is the property of f * g?
#[allow(dead_code)]
pub fn product_properties(p: &FunPropStrength, q: &FunPropStrength) -> FunPropStrength {
    p.meets(q)
}
/// Check if a property transfers through a pointwise operation.
#[allow(dead_code)]
pub fn pointwise_property(
    op_prop: &FunPropStrength,
    arg_prop: &FunPropStrength,
) -> FunPropStrength {
    compose_properties(arg_prop, op_prop)
}
#[cfg(test)]
mod fun_prop_extended_tests {
    use super::*;
    use crate::tactic::fun_prop::*;
    #[test]
    fn test_cache_lookup_miss() {
        let mut cache = FunPropCache::new();
        assert!(cache.lookup("sin", "continuous").is_none());
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_cache_insert_lookup() {
        let mut cache = FunPropCache::new();
        cache.insert("sin", "continuous", true);
        assert_eq!(cache.lookup("sin", "continuous"), Some(true));
        assert_eq!(cache.hits, 1);
    }
    #[test]
    fn test_cache_hit_rate() {
        let mut cache = FunPropCache::new();
        cache.insert("f", "P", true);
        cache.lookup("f", "P");
        cache.lookup("g", "Q");
        assert!((cache.hit_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_fun_prop_strength_ordering() {
        assert!(FunPropStrength::Analytic > FunPropStrength::Continuous);
        assert!(FunPropStrength::Continuous > FunPropStrength::Measurable);
    }
    #[test]
    fn test_fun_prop_strength_implies() {
        assert!(FunPropStrength::Analytic.implies(&FunPropStrength::Continuous));
        assert!(!FunPropStrength::Continuous.implies(&FunPropStrength::Analytic));
    }
    #[test]
    fn test_fun_prop_registry_default() {
        let reg = FunPropRegistry::new();
        assert!(reg.has_property("sin", &FunPropStrength::Continuous));
        assert!(reg.has_property("sqrt", &FunPropStrength::Continuous));
    }
    #[test]
    fn test_fun_prop_registry_strongest() {
        let reg = FunPropRegistry::new();
        let s = reg.strongest_property("exp");
        assert_eq!(s, FunPropStrength::Analytic);
    }
    #[test]
    fn test_compose_properties() {
        let p = FunPropStrength::Continuous;
        let q = FunPropStrength::Measurable;
        let r = compose_properties(&p, &q);
        assert_eq!(r, FunPropStrength::Measurable);
    }
    #[test]
    fn test_sum_properties() {
        let p = FunPropStrength::Analytic;
        let q = FunPropStrength::Continuous;
        let r = sum_properties(&p, &q);
        assert_eq!(r, FunPropStrength::Continuous);
    }
    #[test]
    fn test_fun_prop_goal() {
        let goal = FunPropGoal::new("f", "continuous");
        assert_eq!(goal.func_name, "f");
        assert!(goal.description().contains("continuous"));
    }
    #[test]
    fn test_fun_prop_solver_sin() {
        let mut solver = FunPropSolver::new();
        let goal = FunPropGoal::new("sin", "continuous");
        assert!(solver.solve(&goal).is_some());
    }
    #[test]
    fn test_fun_prop_solver_unknown() {
        let mut solver = FunPropSolver::new();
        let goal = FunPropGoal::new("my_weird_fn", "continuous");
        assert!(solver.solve(&goal).is_none());
    }
    #[test]
    fn test_fun_prop_trace() {
        let mut trace = FunPropTrace::new();
        trace.log("start");
        trace.log("lookup registry");
        assert_eq!(trace.num_steps(), 2);
        assert!(trace.summarize().contains("start"));
    }
    #[test]
    fn test_fun_prop_strength_name() {
        assert_eq!(FunPropStrength::Continuous.name(), "continuous");
        assert_eq!(FunPropStrength::Analytic.name(), "analytic");
    }
    #[test]
    fn test_fun_prop_registry_register_custom() {
        let mut reg = FunPropRegistry::new();
        reg.register("my_fn", FunPropStrength::Differentiable);
        assert!(reg.has_property("my_fn", &FunPropStrength::Continuous));
    }
}
/// Build a standard library of differentiability records.
#[allow(dead_code)]
pub fn standard_diff_database() -> DiffDatabase {
    let mut db = DiffDatabase::new();
    db.register(DiffRecord::new("sin").diff_analytic());
    db.register(DiffRecord::new("cos").diff_analytic());
    db.register(DiffRecord::new("exp").diff_analytic());
    db.register(DiffRecord::new("log").diff_smooth());
    db.register(DiffRecord::new("sqrt").diff_smooth());
    db.register(DiffRecord::new("abs").with_derivative(1.0));
    db.register(DiffRecord::new("id").diff_analytic().with_derivative(1.0));
    db.register(
        DiffRecord::new("const_zero")
            .diff_analytic()
            .with_derivative(0.0),
    );
    db.register(
        DiffRecord::new("const_one")
            .diff_analytic()
            .with_derivative(0.0),
    );
    db
}
/// Extended composition: check if composition of f and g has a property.
#[allow(dead_code)]
pub fn has_property_composition(
    f_props: &[FunPropStrength],
    g_props: &[FunPropStrength],
    target: &FunPropStrength,
) -> bool {
    f_props.contains(target) && g_props.contains(target)
}
/// Check if a function property is preserved under pointwise limits.
#[allow(dead_code)]
pub fn preserved_under_limits(prop: &FunPropStrength) -> bool {
    match prop {
        FunPropStrength::Continuous => true,
        FunPropStrength::Measurable => true,
        FunPropStrength::Differentiable => false,
        FunPropStrength::SmoothInfinite => false,
        FunPropStrength::Analytic => false,
        FunPropStrength::Integrable => true,
        FunPropStrength::Unknown => false,
    }
}
/// Get the "strength ordering": stronger props imply weaker ones.
#[allow(dead_code)]
pub fn is_stronger_than(stronger: &FunPropStrength, weaker: &FunPropStrength) -> bool {
    match (stronger, weaker) {
        (FunPropStrength::SmoothInfinite, FunPropStrength::Differentiable) => true,
        (FunPropStrength::SmoothInfinite, FunPropStrength::Continuous) => true,
        (FunPropStrength::SmoothInfinite, FunPropStrength::Measurable) => true,
        (FunPropStrength::Differentiable, FunPropStrength::Continuous) => true,
        (FunPropStrength::Differentiable, FunPropStrength::Measurable) => true,
        (FunPropStrength::Continuous, FunPropStrength::Measurable) => true,
        (a, b) => a == b,
    }
}
/// Attempt to prove a function property goal.
#[allow(dead_code)]
pub fn prove_fun_prop(goal: &ExtFunPropGoal, registry: &FunPropRegistry) -> FunPropProof {
    if registry.has_property(&goal.func_name, &goal.property) {
        return FunPropProof::Direct(format!("{} is {:?}", goal.func_name, goal.property));
    }
    FunPropProof::Failed(format!(
        "Cannot prove {:?} for {}",
        goal.property, goal.func_name
    ))
}
#[cfg(test)]
mod fun_prop_ext_tests {
    use super::*;
    use crate::tactic::fun_prop::*;
    #[test]
    fn test_diff_record_new() {
        let rec = DiffRecord::new("sin").diff_analytic();
        assert!(rec.is_smooth);
        assert!(rec.is_analytic);
    }
    #[test]
    fn test_diff_database_lookup() {
        let db = standard_diff_database();
        assert!(db.diff_is_smooth("sin"));
        assert!(db.diff_is_analytic("exp"));
        assert!(!db.diff_is_smooth("abs"));
    }
    #[test]
    fn test_diff_database_size() {
        let db = standard_diff_database();
        assert!(db.num_records() >= 5);
    }
    #[test]
    fn test_meas_record() {
        let rec = MeasRecord::borel("indicator");
        assert_eq!(rec.kind, MeasurabilityKind::BorelMeas);
        assert!(!rec.is_strongly_measurable);
    }
    #[test]
    fn test_meas_database_register() {
        let mut db = MeasDatabase::new();
        db.register(MeasRecord::strongly("f"));
        assert!(db.is_measurable("f"));
        assert!(!db.is_measurable("g"));
    }
    #[test]
    fn test_lipschitz_compose() {
        let f = LipschitzInfo::new("f", 2.0);
        let g = LipschitzInfo::new("g", 0.3);
        let fg = f.lip_compose(&g);
        assert!((fg.constant - 0.6).abs() < 1e-10);
        assert!(fg.is_contraction);
    }
    #[test]
    fn test_lipschitz_non_expansive() {
        let f = LipschitzInfo::new("proj", 1.0);
        assert!(f.is_non_expansive());
    }
    #[test]
    fn test_is_stronger_than() {
        assert!(is_stronger_than(
            &FunPropStrength::SmoothInfinite,
            &FunPropStrength::Continuous
        ));
        assert!(is_stronger_than(
            &FunPropStrength::Differentiable,
            &FunPropStrength::Measurable
        ));
        assert!(!is_stronger_than(
            &FunPropStrength::Continuous,
            &FunPropStrength::Differentiable
        ));
    }
    #[test]
    fn test_preserved_under_limits() {
        assert!(preserved_under_limits(&FunPropStrength::Continuous));
        assert!(!preserved_under_limits(&FunPropStrength::SmoothInfinite));
    }
    #[test]
    fn test_has_property_composition() {
        let f_props = vec![FunPropStrength::Continuous, FunPropStrength::SmoothInfinite];
        let g_props = vec![FunPropStrength::Continuous, FunPropStrength::SmoothInfinite];
        assert!(has_property_composition(
            &f_props,
            &g_props,
            &FunPropStrength::Continuous
        ));
        assert!(!has_property_composition(
            &f_props,
            &[],
            &FunPropStrength::Continuous
        ));
    }
    #[test]
    fn test_fun_prop_proof_direct() {
        let mut reg = FunPropRegistry::new();
        reg.register("sin", FunPropStrength::SmoothInfinite);
        let goal = ExtFunPropGoal {
            func_name: "sin".to_string(),
            property: FunPropStrength::Continuous,
        };
        let proof = prove_fun_prop(&goal, &reg);
        assert!(proof.is_success());
    }
    #[test]
    fn test_fun_prop_proof_failed() {
        let reg = FunPropRegistry::new();
        let goal = ExtFunPropGoal {
            func_name: "unknown".to_string(),
            property: FunPropStrength::SmoothInfinite,
        };
        let proof = prove_fun_prop(&goal, &reg);
        assert!(!proof.is_success());
        assert!(proof.failure_reason().is_some());
    }
    #[test]
    fn test_continuity_checker() {
        let mut checker = ContinuityChecker::new(0.1, 0.1);
        let result = checker.check(|x| x * x, 0.0, &[0.05, -0.05, 0.09]);
        assert!(result);
        assert_eq!(checker.checks_passed, 3);
    }
    #[test]
    fn test_prop_lattice_implied_by() {
        let lat = PropLattice::standard();
        let implied = lat.implied_by(&FunPropStrength::SmoothInfinite);
        assert!(implied.contains(&FunPropStrength::Continuous));
        assert!(implied.contains(&FunPropStrength::Measurable));
    }
    #[test]
    fn test_prop_lattice_join() {
        let lat = PropLattice::standard();
        let props = vec![FunPropStrength::Continuous, FunPropStrength::Measurable];
        let j = lat.lattice_join(&props);
        assert!(j.is_some());
    }
    #[test]
    fn test_prop_lattice_meet() {
        let lat = PropLattice::standard();
        let props = vec![
            FunPropStrength::SmoothInfinite,
            FunPropStrength::Differentiable,
        ];
        let m = lat.lattice_meet(&props);
        assert!(m.is_some());
    }
    #[test]
    fn test_fun_prop_registry_compose() {
        let fp = compose_properties(
            &FunPropStrength::Differentiable,
            &FunPropStrength::Differentiable,
        );
        assert_eq!(fp, FunPropStrength::Differentiable);
    }
    #[test]
    fn test_sum_product_properties() {
        let fa = FunPropStrength::Continuous;
        let fb = FunPropStrength::Continuous;
        let sum = sum_properties(&fa, &fb);
        assert_eq!(sum, FunPropStrength::Continuous);
        let prod = product_properties(&fa, &fb);
        assert_eq!(prod, FunPropStrength::Continuous);
    }
    #[test]
    fn test_fun_prop_trace() {
        let mut trace = FunPropTrace::new();
        trace.log("step1");
        trace.log("step2");
        assert_eq!(trace.steps.len(), 2);
    }
    #[test]
    fn test_fun_prop_solver_new() {
        let solver = FunPropSolver::new();
        assert_eq!(solver.max_depth, 10);
    }
}
#[cfg(test)]
mod tacticfunprop_analysis_tests {
    use super::*;
    use crate::tactic::fun_prop::*;
    #[test]
    fn test_tacticfunprop_result_ok() {
        let r = TacticFunPropResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticfunprop_result_err() {
        let r = TacticFunPropResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticfunprop_result_partial() {
        let r = TacticFunPropResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticfunprop_result_skipped() {
        let r = TacticFunPropResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticfunprop_analysis_pass_run() {
        let mut p = TacticFunPropAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticfunprop_analysis_pass_empty_input() {
        let mut p = TacticFunPropAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticfunprop_analysis_pass_success_rate() {
        let mut p = TacticFunPropAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticfunprop_analysis_pass_disable() {
        let mut p = TacticFunPropAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticfunprop_pipeline_basic() {
        let mut pipeline = TacticFunPropPipeline::new("main_pipeline");
        pipeline.add_pass(TacticFunPropAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticFunPropAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticfunprop_pipeline_disabled_pass() {
        let mut pipeline = TacticFunPropPipeline::new("partial");
        let mut p = TacticFunPropAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticFunPropAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticfunprop_diff_basic() {
        let mut d = TacticFunPropDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticfunprop_diff_summary() {
        let mut d = TacticFunPropDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticfunprop_config_set_get() {
        let mut cfg = TacticFunPropConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticfunprop_config_read_only() {
        let mut cfg = TacticFunPropConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticfunprop_config_remove() {
        let mut cfg = TacticFunPropConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticfunprop_diagnostics_basic() {
        let mut diag = TacticFunPropDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticfunprop_diagnostics_max_errors() {
        let mut diag = TacticFunPropDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticfunprop_diagnostics_clear() {
        let mut diag = TacticFunPropDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticfunprop_config_value_types() {
        let b = TacticFunPropConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticFunPropConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticFunPropConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticFunPropConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticFunPropConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod fun_prop_ext_tests_200 {
    use super::*;
    use crate::tactic::fun_prop::*;
    #[test]
    fn test_fun_prop_ext_result_ok_200() {
        let r = FunPropExtResult200::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_fun_prop_ext_result_err_200() {
        let r = FunPropExtResult200::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_fun_prop_ext_result_partial_200() {
        let r = FunPropExtResult200::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_fun_prop_ext_result_skipped_200() {
        let r = FunPropExtResult200::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_fun_prop_ext_pass_run_200() {
        let mut p = FunPropExtPass200::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_fun_prop_ext_pass_empty_200() {
        let mut p = FunPropExtPass200::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_fun_prop_ext_pass_rate_200() {
        let mut p = FunPropExtPass200::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_fun_prop_ext_pass_disable_200() {
        let mut p = FunPropExtPass200::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_fun_prop_ext_pipeline_basic_200() {
        let mut pipeline = FunPropExtPipeline200::new("main_pipeline");
        pipeline.add_pass(FunPropExtPass200::new("pass1"));
        pipeline.add_pass(FunPropExtPass200::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_fun_prop_ext_pipeline_disabled_200() {
        let mut pipeline = FunPropExtPipeline200::new("partial");
        let mut p = FunPropExtPass200::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(FunPropExtPass200::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_fun_prop_ext_diff_basic_200() {
        let mut d = FunPropExtDiff200::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_fun_prop_ext_config_set_get_200() {
        let mut cfg = FunPropExtConfig200::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_fun_prop_ext_config_read_only_200() {
        let mut cfg = FunPropExtConfig200::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_fun_prop_ext_config_remove_200() {
        let mut cfg = FunPropExtConfig200::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_fun_prop_ext_diagnostics_basic_200() {
        let mut diag = FunPropExtDiag200::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_fun_prop_ext_diagnostics_max_errors_200() {
        let mut diag = FunPropExtDiag200::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_fun_prop_ext_diagnostics_clear_200() {
        let mut diag = FunPropExtDiag200::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_fun_prop_ext_config_value_types_200() {
        let b = FunPropExtConfigVal200::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = FunPropExtConfigVal200::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = FunPropExtConfigVal200::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = FunPropExtConfigVal200::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = FunPropExtConfigVal200::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod fun_prop_ext_tests_201 {
    use super::*;
    use crate::tactic::fun_prop::*;
    #[test]
    fn test_fun_prop_ext_result_ok_200() {
        let r = FunPropExtResult201::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_fun_prop_ext_result_err_200() {
        let r = FunPropExtResult201::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_fun_prop_ext_result_partial_200() {
        let r = FunPropExtResult201::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_fun_prop_ext_result_skipped_200() {
        let r = FunPropExtResult201::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_fun_prop_ext_pass_run_200() {
        let mut p = FunPropExtPass201::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_fun_prop_ext_pass_empty_200() {
        let mut p = FunPropExtPass201::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_fun_prop_ext_pass_rate_200() {
        let mut p = FunPropExtPass201::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_fun_prop_ext_pass_disable_200() {
        let mut p = FunPropExtPass201::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_fun_prop_ext_pipeline_basic_200() {
        let mut pipeline = FunPropExtPipeline201::new("main_pipeline");
        pipeline.add_pass(FunPropExtPass201::new("pass1"));
        pipeline.add_pass(FunPropExtPass201::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_fun_prop_ext_pipeline_disabled_200() {
        let mut pipeline = FunPropExtPipeline201::new("partial");
        let mut p = FunPropExtPass201::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(FunPropExtPass201::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_fun_prop_ext_diff_basic_200() {
        let mut d = FunPropExtDiff201::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_fun_prop_ext_config_set_get_200() {
        let mut cfg = FunPropExtConfig201::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_fun_prop_ext_config_read_only_200() {
        let mut cfg = FunPropExtConfig201::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_fun_prop_ext_config_remove_200() {
        let mut cfg = FunPropExtConfig201::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_fun_prop_ext_diagnostics_basic_200() {
        let mut diag = FunPropExtDiag201::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_fun_prop_ext_diagnostics_max_errors_200() {
        let mut diag = FunPropExtDiag201::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_fun_prop_ext_diagnostics_clear_200() {
        let mut diag = FunPropExtDiag201::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_fun_prop_ext_config_value_types_200() {
        let b = FunPropExtConfigVal201::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = FunPropExtConfigVal201::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = FunPropExtConfigVal201::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = FunPropExtConfigVal201::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = FunPropExtConfigVal201::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}

/// `continuity` — prove continuity goals of the form `Continuous f`.
///
/// Extracts the function expression from the goal string and queries the
/// [`FunPropTactic`] database (initialised with standard rules) for a proof
/// of `FunProperty::Continuous`.  If found, the goal is closed with the
/// proof-term string embedded as a named constant; otherwise fails.
pub fn tac_continuity(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("continuity: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let target_str = target.to_string();

    // Extract the function argument from a goal of the form "(Continuous f)".
    let fn_expr = extract_fun_prop_arg(&target_str, "Continuous");

    let tac = FunPropTactic::with_db(FunPropDatabase::with_standard_rules());
    match tac.prove_continuous(fn_expr.trim()) {
        Some(proof_str) => {
            let proof = Expr::Const(Name::str(&proof_str), vec![]);
            state.close_goal(proof, ctx)?;
            Ok(())
        }
        None => Err(TacticError::Failed(format!(
            "continuity: could not prove `Continuous {}` automatically",
            fn_expr.trim()
        ))),
    }
}

/// `measurability` — prove measurability goals of the form `Measurable f`.
///
/// Mirrors [`tac_continuity`] but queries for `FunProperty::Measurable`.
pub fn tac_measurability(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("measurability: goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let target_str = target.to_string();

    let fn_expr = extract_fun_prop_arg(&target_str, "Measurable");

    let tac = FunPropTactic::with_db(FunPropDatabase::with_standard_rules());
    match tac.prove_measurable(fn_expr.trim()) {
        Some(proof_str) => {
            let proof = Expr::Const(Name::str(&proof_str), vec![]);
            state.close_goal(proof, ctx)?;
            Ok(())
        }
        None => Err(TacticError::Failed(format!(
            "measurability: could not prove `Measurable {}` automatically",
            fn_expr.trim()
        ))),
    }
}

/// Extract the function argument from a goal like `(Continuous f)` or `(Measurable f)`.
///
/// Looks for the pattern `"KEYWORD f"` in `goal_str` and returns the remainder
/// after `KEYWORD `.  Falls back to the whole string if the keyword is absent.
fn extract_fun_prop_arg<'a>(goal_str: &'a str, keyword: &str) -> &'a str {
    let search = format!("{} ", keyword);
    if let Some(idx) = goal_str.find(&search) {
        let after = &goal_str[idx + search.len()..];
        // Strip trailing `)` from Display output like `(Continuous id)`.
        after.trim_end_matches(')').trim()
    } else {
        goal_str
    }
}
