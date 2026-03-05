//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Environment, Expr, Level, Name};

use super::types::{
    CoherenceResult, EtaCanonMap, EtaCategorizer, EtaCategory, EtaChangeKind, EtaChangeLog,
    EtaDepthTracker, EtaEqualityOracle, EtaExpanded, EtaGraph, EtaLongChecker, EtaLongStatus,
    EtaNormRunSummary, EtaNormalFormChecker, EtaNormalizationPass, EtaPassConfig, EtaRedex,
    EtaRedexCollector, EtaReduction, EtaRewriteEngine, EtaStateMachine, EtaStats,
    FieldBoundsChecker, FieldDescriptor, InjectivityChecker, KReductionTable, ProjectionRewrite,
    ProjectionRewriteSet, RecordUpdate, RecordUpdateBatch, ShapeEquivalence, SingletonKReducer,
    StructFlatteningPass, StructShape, StructureEta, StructureRegistry,
};

/// Extract the head `Const` name of an expression by stripping `App` nodes.
pub(super) fn head_const(expr: &Expr) -> Option<&Name> {
    match expr {
        Expr::Const(n, _) => Some(n),
        Expr::App(f, _) => head_const(f),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::declaration::{ConstantInfo, ConstantVal, ConstructorVal, InductiveVal};
    use crate::{BinderInfo, Environment, Expr, Level, Name};
    /// Build a minimal environment with a structure-like inductive called `MyPair`.
    ///
    /// MyPair has:
    /// - 0 params, 0 indices
    /// - 1 constructor `MyPair.mk` with 2 fields
    fn env_with_mypair() -> Environment {
        let mut env = Environment::new();
        let ind_name = Name::str("MyPair");
        let ctor_name = Name::str("MyPair.mk");
        let ctor_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("fst"),
            Box::new(Expr::Sort(Level::Zero)),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("snd"),
                Box::new(Expr::Sort(Level::Zero)),
                Box::new(Expr::Const(ind_name.clone(), vec![])),
            )),
        );
        let common_ctor = ConstantVal {
            name: ctor_name.clone(),
            level_params: vec![],
            ty: ctor_ty,
        };
        let ctor_val = ConstructorVal {
            common: common_ctor,
            induct: ind_name.clone(),
            cidx: 0,
            num_params: 0,
            num_fields: 2,
            is_unsafe: false,
        };
        let ind_ty = Expr::Sort(Level::succ(Level::Zero));
        let common_ind = ConstantVal {
            name: ind_name.clone(),
            level_params: vec![],
            ty: ind_ty,
        };
        let ind_val = InductiveVal {
            common: common_ind,
            num_params: 0,
            num_indices: 0,
            all: vec![ind_name.clone()],
            ctors: vec![ctor_name.clone()],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        };
        env.add_constant(ConstantInfo::Inductive(ind_val))
            .expect("value should be present");
        env.add_constant(ConstantInfo::Constructor(ctor_val))
            .expect("value should be present");
        env
    }
    /// Build an environment with a singleton type `Unit` (1 constructor, 0 fields).
    fn env_with_unit() -> Environment {
        let mut env = Environment::new();
        let ind_name = Name::str("Unit");
        let ctor_name = Name::str("Unit.unit");
        let ctor_ty = Expr::Const(ind_name.clone(), vec![]);
        let common_ctor = ConstantVal {
            name: ctor_name.clone(),
            level_params: vec![],
            ty: ctor_ty,
        };
        let ctor_val = ConstructorVal {
            common: common_ctor,
            induct: ind_name.clone(),
            cidx: 0,
            num_params: 0,
            num_fields: 0,
            is_unsafe: false,
        };
        let ind_ty = Expr::Sort(Level::succ(Level::Zero));
        let common_ind = ConstantVal {
            name: ind_name.clone(),
            level_params: vec![],
            ty: ind_ty,
        };
        let ind_val = InductiveVal {
            common: common_ind,
            num_params: 0,
            num_indices: 0,
            all: vec![ind_name.clone()],
            ctors: vec![ctor_name.clone()],
            num_nested: 0,
            is_rec: false,
            is_unsafe: false,
            is_reflexive: false,
            is_prop: false,
        };
        env.add_constant(ConstantInfo::Inductive(ind_val))
            .expect("value should be present");
        env.add_constant(ConstantInfo::Constructor(ctor_val))
            .expect("value should be present");
        env
    }
    #[test]
    fn test_is_structure_type_recognizes_mypair() {
        let env = env_with_mypair();
        let se = StructureEta::new(&env);
        let ty = Expr::Const(Name::str("MyPair"), vec![]);
        assert!(se.is_structure_type(&ty));
    }
    #[test]
    fn test_is_structure_type_rejects_unknown() {
        let env = Environment::new();
        let se = StructureEta::new(&env);
        let ty = Expr::Const(Name::str("Foo"), vec![]);
        assert!(!se.is_structure_type(&ty));
    }
    #[test]
    fn test_collect_field_types_mypair() {
        let env = env_with_mypair();
        let se = StructureEta::new(&env);
        let fields = se.collect_field_types(&Name::str("MyPair"));
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, Name::str("fst"));
        assert_eq!(fields[1].0, Name::str("snd"));
    }
    #[test]
    fn test_make_proj_chain_length() {
        let env = env_with_mypair();
        let se = StructureEta::new(&env);
        let base = Expr::Const(Name::str("e"), vec![]);
        let projs = se.make_proj_chain(&base, &Name::str("MyPair"), 2);
        assert_eq!(projs.len(), 2);
        assert!(matches!(& projs[0], Expr::Proj(n, 0, _) if n == & Name::str("MyPair")));
        assert!(matches!(& projs[1], Expr::Proj(n, 1, _) if n == & Name::str("MyPair")));
    }
    #[test]
    fn test_eta_expand_struct_produces_app_tree() {
        let env = env_with_mypair();
        let se = StructureEta::new(&env);
        let expr = Expr::Const(Name::str("e"), vec![]);
        let ty = Expr::Const(Name::str("MyPair"), vec![]);
        let expanded = se.eta_expand_struct(&expr, &ty);
        assert!(expanded.is_some(), "should expand MyPair expression");
        let result = expanded.expect("result should be present");
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_is_singleton_unit() {
        let env = env_with_unit();
        let reducer = SingletonKReducer::new(&env);
        let ty = Expr::Const(Name::str("Unit"), vec![]);
        assert!(reducer.is_singleton_type(&ty));
    }
    #[test]
    fn test_is_not_singleton_mypair() {
        let env = env_with_mypair();
        let reducer = SingletonKReducer::new(&env);
        let ty = Expr::Const(Name::str("MyPair"), vec![]);
        assert!(!reducer.is_singleton_type(&ty));
    }
    #[test]
    fn test_k_reduce_unit_returns_ctor() {
        let env = env_with_unit();
        let reducer = SingletonKReducer::new(&env);
        let expr = Expr::FVar(crate::FVarId::new(42));
        let ty = Expr::Const(Name::str("Unit"), vec![]);
        let result = reducer.k_reduce(&expr, &ty);
        assert!(result.is_some());
        let canonical = result.expect("canonical should be present");
        assert!(
            matches!(& canonical, Expr::Const(n, _) if n == & Name::str("Unit.unit")),
            "expected Unit.unit constructor, got {:?}",
            canonical
        );
    }
}
#[cfg(test)]
mod tests_struct_eta_extended {
    use super::*;
    #[test]
    fn test_field_descriptor() {
        let f = FieldDescriptor::new("x", 0, false);
        assert!(f.is_data());
        assert!(!f.is_prop);
        let fp = FieldDescriptor::new("proof", 1, true);
        assert!(!fp.is_data());
    }
    #[test]
    fn test_structure_registry() {
        let mut reg = StructureRegistry::new();
        reg.register(
            "Point",
            "Point.mk",
            vec![
                FieldDescriptor::new("x", 0, false),
                FieldDescriptor::new("y", 1, false),
            ],
        );
        reg.register(
            "Sigma",
            "Sigma.mk",
            vec![
                FieldDescriptor::new("fst", 0, false),
                FieldDescriptor::new("snd", 1, true),
            ],
        );
        assert_eq!(reg.len(), 2);
        assert_eq!(reg.field_count("Point"), 2);
        assert!(!reg.has_prop_fields("Point"));
        assert!(reg.has_prop_fields("Sigma"));
        assert_eq!(reg.field_count("Unknown"), 0);
        let projs = reg.projectors("Point");
        assert!(projs.contains(&"Point.x".to_string()));
        assert!(projs.contains(&"Point.y".to_string()));
    }
    #[test]
    fn test_eta_expanded() {
        let exp = EtaExpanded::new("Point.mk", 42, vec![100, 101]);
        assert_eq!(exp.arity(), 2);
        assert_eq!(exp.expr_id, 42);
    }
    #[test]
    fn test_eta_reduction() {
        let r = EtaReduction::valid(99, "Point.mk");
        assert!(r.is_valid);
        let inv = EtaReduction::invalid("Unknown");
        assert!(!inv.is_valid);
    }
    #[test]
    fn test_eta_stats() {
        let mut s = EtaStats::new();
        s.record_expansion();
        s.record_expansion();
        s.record_failed_expansion();
        s.record_reduction();
        assert!((s.expansion_rate() - 2.0 / 3.0).abs() < 1e-9);
        assert!((s.reduction_rate() - 1.0).abs() < 1e-9);
        let summary = s.summary();
        assert!(summary.contains("expansions=2"));
    }
    #[test]
    fn test_eta_normal_form_checker() {
        let mut reg = StructureRegistry::new();
        reg.register(
            "Prod",
            "Prod.mk",
            vec![
                FieldDescriptor::new("fst", 0, false),
                FieldDescriptor::new("snd", 1, false),
            ],
        );
        let checker = EtaNormalFormChecker::new(reg);
        assert!(checker.knows_structure("Prod"));
        assert!(!checker.knows_structure("Sum"));
        assert_eq!(checker.expected_arity("Prod"), Some(2));
        let valid_exp = EtaExpanded::new("Prod", 1, vec![10, 20]);
        let invalid_exp = EtaExpanded::new("Prod", 2, vec![10]);
        assert!(checker.check_expansion(&valid_exp));
        assert!(!checker.check_expansion(&invalid_exp));
    }
    #[test]
    fn test_eta_normalization_pass() {
        let mut pass = EtaNormalizationPass::new();
        pass.schedule(1);
        pass.schedule(2);
        pass.schedule(1);
        assert_eq!(pass.pending(), 2);
        let first = pass.next();
        assert_eq!(first, Some(1));
        let _second = pass.next();
        assert!(pass.is_done());
    }
    #[test]
    fn test_k_reduction_table() {
        let mut kt = KReductionTable::new();
        kt.set("Subsingleton", true);
        kt.set("Nonempty", true);
        kt.set("Point", false);
        assert!(kt.is_k_reducible("Subsingleton"));
        assert!(!kt.is_k_reducible("Point"));
        assert!(!kt.is_k_reducible("Unknown"));
        let names = kt.k_reducible_names();
        assert!(names.contains(&"Subsingleton"));
        assert!(!names.contains(&"Point"));
    }
}
#[cfg(test)]
mod tests_struct_eta_extended2 {
    use super::*;
    #[test]
    fn test_eta_depth_tracker() {
        let mut t = EtaDepthTracker::new();
        assert_eq!(t.depth(), 0);
        assert!(!t.is_nested());
        t.push("Prod");
        t.push("Sigma");
        assert_eq!(t.depth(), 2);
        assert!(t.is_nested());
        assert_eq!(t.current(), Some("Sigma"));
        assert!(t.contains("Prod"));
        assert!(!t.contains("Sum"));
        assert_eq!(t.path(), "Prod.Sigma");
        t.pop();
        assert_eq!(t.depth(), 1);
    }
    #[test]
    fn test_coherence_result() {
        let ok = CoherenceResult::ok();
        assert!(ok.is_coherent());
        let fail = CoherenceResult::fail("mismatch in field 2");
        assert!(!fail.is_coherent());
        assert_eq!(
            fail,
            CoherenceResult::Incoherent {
                reason: "mismatch in field 2".to_string()
            }
        );
    }
    #[test]
    fn test_projection_rewrite_set() {
        let mut set = ProjectionRewriteSet::new();
        set.add(ProjectionRewrite::new("Point.mk", "Point.x", 0));
        set.add(ProjectionRewrite::new("Point.mk", "Point.y", 1));
        assert_eq!(set.len(), 2);
        let r = set
            .find_by_projector("Point.x")
            .expect("r should be present");
        assert_eq!(r.field_index, 0);
        let rules = set.rules_for_ctor("Point.mk");
        assert_eq!(rules.len(), 2);
    }
    #[test]
    fn test_field_bounds_checker() {
        assert!(FieldBoundsChecker::check(3, 0).is_ok());
        assert!(FieldBoundsChecker::check(3, 2).is_ok());
        assert!(FieldBoundsChecker::check(3, 3).is_err());
    }
    #[test]
    fn test_field_bounds_checker_validate_set() {
        let mut reg = StructureRegistry::new();
        reg.register(
            "Pair",
            "Pair.mk",
            vec![
                FieldDescriptor::new("fst", 0, false),
                FieldDescriptor::new("snd", 1, false),
            ],
        );
        let mut set = ProjectionRewriteSet::new();
        set.add(ProjectionRewrite::new("Pair", "Pair.fst", 0));
        set.add(ProjectionRewrite::new("Pair", "Pair.snd", 1));
        set.add(ProjectionRewrite::new("Pair", "Pair.bad", 5));
        let errors = FieldBoundsChecker::validate_set(&set, &reg);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("out of bounds"));
    }
    #[test]
    fn test_eta_state_machine() {
        let mut sm = EtaStateMachine::new();
        assert!(!sm.is_expanding());
        sm.start("Prod", 2);
        assert!(sm.is_expanding());
        assert_eq!(sm.remaining(), 2);
        sm.process_field();
        assert!(sm.is_expanding());
        assert_eq!(sm.remaining(), 1);
        sm.process_field();
        assert!(sm.is_done());
        assert_eq!(sm.remaining(), 0);
    }
    #[test]
    fn test_projection_rewrite_as_rule() {
        let r = ProjectionRewrite::new("Prod.mk", "Prod.fst", 0);
        let rule = r.as_rule();
        assert!(rule.contains("Prod.fst"));
        assert!(rule.contains("Prod.mk"));
    }
}
#[cfg(test)]
mod tests_struct_eta_extended3 {
    use super::*;
    #[test]
    fn test_struct_shape_basics() {
        let ctor = StructShape::ctor("Prod.mk", 2);
        let proj = StructShape::proj(0);
        let other = StructShape::Other;
        assert!(ctor.is_ctor());
        assert!(!ctor.is_proj());
        assert!(proj.is_proj());
        assert!(!proj.is_ctor());
        assert_eq!(ctor.arity(), Some(2));
        assert_eq!(other.arity(), None);
    }
    #[test]
    fn test_eta_redex_collector() {
        let mut collector = EtaRedexCollector::new();
        collector.add(EtaRedex::new(vec![], "Point.mk", 1));
        collector.add(EtaRedex::new(vec![0, 1], "Sigma.mk", 2));
        assert_eq!(collector.count(), 2);
        assert!(collector.has_top_level());
        let sorted = collector.sorted_by_depth();
        assert_eq!(sorted[0].depth(), 0);
        assert_eq!(sorted[1].depth(), 2);
    }
    #[test]
    fn test_eta_redex_depth_limit() {
        let mut collector = EtaRedexCollector::with_max_depth(1);
        collector.add(EtaRedex::new(vec![0], "A.mk", 1));
        collector.add(EtaRedex::new(vec![0, 1], "B.mk", 2));
        assert_eq!(collector.count(), 1);
    }
    #[test]
    fn test_struct_flattening_pass() {
        let mut pass = StructFlatteningPass::new();
        for _ in 0..100 {
            pass.record_processed();
        }
        for _ in 0..42 {
            pass.record_flattened();
        }
        assert!((pass.flatten_rate() - 0.42).abs() < 1e-9);
    }
    #[test]
    fn test_shape_equivalence() {
        let mut reg = StructureRegistry::new();
        reg.register(
            "Prod",
            "Prod.mk",
            vec![
                FieldDescriptor::new("fst", 0, false),
                FieldDescriptor::new("snd", 1, false),
            ],
        );
        let oracle = ShapeEquivalence::new(reg);
        let a = StructShape::ctor("Prod.mk", 2);
        let b = StructShape::ctor("Prod.mk", 2);
        let c = StructShape::ctor("Sum.mk", 2);
        assert!(oracle.may_be_equal(&a, &b));
        assert!(!oracle.may_be_equal(&a, &c));
        let prod_shape = StructShape::ctor("Prod", 2);
        assert!(oracle.is_expandable(&prod_shape));
        let unknown = StructShape::ctor("Unknown", 1);
        assert!(!oracle.is_expandable(&unknown));
    }
}
#[cfg(test)]
mod tests_struct_eta_extended4 {
    use super::*;
    #[test]
    fn test_injectivity_checker() {
        let mut ic = InjectivityChecker::new();
        ic.mark_injective("Prod.mk");
        ic.mark_injective("List.cons");
        ic.mark_injective("Prod.mk");
        assert_eq!(ic.count(), 2);
        assert!(ic.is_injective("Prod.mk"));
        assert!(!ic.is_injective("Nat.succ"));
    }
    #[test]
    fn test_record_update() {
        let u = RecordUpdate::new(10, "Point", 0, 20);
        assert_eq!(u.expr_id, 10);
        assert_eq!(u.field_index, 0);
        assert!(u.describe().contains("Point.0"));
    }
    #[test]
    fn test_record_update_batch() {
        let mut batch = RecordUpdateBatch::new();
        batch.add(RecordUpdate::new(1, "P", 0, 100));
        batch.add(RecordUpdate::new(1, "P", 1, 200));
        batch.add(RecordUpdate::new(2, "P", 0, 300));
        assert_eq!(batch.len(), 3);
        let for_expr1 = batch.updates_for_expr(1);
        assert_eq!(for_expr1.len(), 2);
        batch.clear();
        assert!(batch.is_empty());
    }
    #[test]
    fn test_eta_norm_run_summary() {
        let mut s = EtaNormRunSummary::new();
        s.total_expressions = 100;
        s.eta_expansions = 30;
        s.eta_reductions = 20;
        s.unchanged = 50;
        assert_eq!(s.total_changes(), 50);
        assert!((s.change_rate() - 0.5).abs() < 1e-9);
        let f = s.format();
        assert!(f.contains("total=100"));
        assert!(f.contains("unchanged=50"));
    }
}
#[cfg(test)]
mod tests_struct_eta_extended5 {
    use super::*;
    #[test]
    fn test_eta_long_status() {
        let s = EtaLongStatus::EtaLong;
        assert!(s.is_eta_long());
        let n = EtaLongStatus::NotEtaLong;
        assert!(!n.is_eta_long());
    }
    #[test]
    fn test_eta_long_checker() {
        let mut chk = EtaLongChecker::new();
        chk.record(1, EtaLongStatus::EtaLong);
        chk.record(2, EtaLongStatus::NotEtaLong);
        chk.record(3, EtaLongStatus::Unknown);
        assert_eq!(chk.status(1), Some(EtaLongStatus::EtaLong));
        assert_eq!(chk.status(99), None);
        let (el, nel, unk) = chk.summary();
        assert_eq!(el, 1);
        assert_eq!(nel, 1);
        assert_eq!(unk, 1);
    }
    #[test]
    fn test_eta_category() {
        assert!(EtaCategory::Record.needs_eta());
        assert!(EtaCategory::Function.needs_eta());
        assert!(!EtaCategory::Primitive.needs_eta());
        assert_eq!(EtaCategory::Inductive.label(), "inductive");
    }
    #[test]
    fn test_eta_categorizer() {
        let mut cat = EtaCategorizer::new();
        cat.assign(1, EtaCategory::Record);
        cat.assign(2, EtaCategory::Primitive);
        cat.assign(3, EtaCategory::Function);
        let needs = cat.needs_eta_ids();
        assert!(needs.contains(&1));
        assert!(needs.contains(&3));
        assert!(!needs.contains(&2));
        let counts = cat.count_by_category();
        let record_count = counts
            .iter()
            .find(|(c, _)| *c == EtaCategory::Record)
            .map(|(_, n)| *n)
            .unwrap_or(0);
        assert_eq!(record_count, 1);
    }
}
#[cfg(test)]
mod tests_struct_eta_extended6 {
    use super::*;
    #[test]
    fn test_eta_pass_config() {
        let cfg = EtaPassConfig::default_config();
        assert!(cfg.any_enabled());
        assert!(cfg.do_expand && cfg.do_reduce);
        let minimal = EtaPassConfig::proj_only();
        assert!(minimal.any_enabled());
        assert!(!minimal.do_expand);
        assert_eq!(minimal.max_passes, 1);
    }
    #[test]
    fn test_eta_change_log() {
        let mut log = EtaChangeLog::new();
        log.record(1, EtaChangeKind::Expanded, 0);
        log.record(2, EtaChangeKind::Reduced, 0);
        log.record(1, EtaChangeKind::ProjRewritten, 1);
        assert_eq!(log.len(), 3);
        let expansions = log.changes_of_kind(EtaChangeKind::Expanded);
        assert_eq!(expansions.len(), 1);
        let for_expr1 = log.changes_for_expr(1);
        assert_eq!(for_expr1.len(), 2);
        let pass0 = log.changes_in_pass(0);
        assert_eq!(pass0.len(), 2);
    }
}
#[cfg(test)]
mod tests_struct_eta_extended7 {
    use super::*;
    #[test]
    fn test_eta_graph() {
        let mut g = EtaGraph::new();
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(4, 2);
        assert!(g.has_edge(1, 2));
        assert!(!g.has_edge(2, 1));
        let deps = g.dependencies_of(1);
        assert!(deps.contains(&2) && deps.contains(&3));
        let dependents = g.dependents_of(2);
        assert!(dependents.contains(&1) && dependents.contains(&4));
        g.remove_node(1);
        assert!(!g.has_edge(1, 2));
        assert_eq!(g.edge_count(), 1);
    }
    #[test]
    fn test_eta_canon_map() {
        let mut cm = EtaCanonMap::new();
        cm.insert(10, 5);
        cm.insert(11, 5);
        cm.insert(12, 7);
        assert_eq!(cm.canonical(10), 5);
        assert_eq!(cm.canonical(99), 99);
        let origs = cm.originals_of(5);
        assert!(origs.contains(&10) && origs.contains(&11));
        assert_eq!(origs.len(), 2);
    }
}
#[cfg(test)]
mod tests_struct_eta_engine {
    use super::*;
    #[test]
    fn test_eta_rewrite_engine() {
        let mut set = ProjectionRewriteSet::new();
        set.add(ProjectionRewrite::new("Prod.mk", "Prod.fst", 0));
        set.add(ProjectionRewrite::new("Prod.mk", "Prod.snd", 1));
        let mut engine = EtaRewriteEngine::new(set, 5);
        assert!(!engine.is_exhausted());
        let r = engine.apply_proj("Prod.fst");
        assert_eq!(r, Some(0));
        assert_eq!(engine.steps_taken(), 1);
        let r2 = engine.apply_proj("Prod.snd");
        assert_eq!(r2, Some(1));
        let r3 = engine.apply_proj("Unknown");
        assert_eq!(r3, None);
        engine.reset();
        assert_eq!(engine.steps_taken(), 0);
    }
    #[test]
    fn test_eta_equality_oracle() {
        let mut cm = EtaCanonMap::new();
        cm.insert(10, 5);
        cm.insert(11, 5);
        cm.insert(12, 7);
        let oracle = EtaEqualityOracle::new(cm);
        assert!(oracle.are_eta_equal(10, 11));
        assert!(!oracle.are_eta_equal(10, 12));
        assert!(oracle.are_eta_equal(99, 99));
        assert_eq!(oracle.class_count(), 2);
    }
}
