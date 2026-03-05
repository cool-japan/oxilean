//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AgdaAnalysisCache, AgdaClause, AgdaConstantFoldingHelper, AgdaConstructor, AgdaData, AgdaDecl,
    AgdaDepGraph, AgdaDominatorTree, AgdaExpr, AgdaExtCache, AgdaExtConstFolder, AgdaExtDepGraph,
    AgdaExtDomTree, AgdaExtLiveness, AgdaExtPassConfig, AgdaExtPassPhase, AgdaExtPassRegistry,
    AgdaExtPassStats, AgdaExtWorklist, AgdaField, AgdaLivenessInfo, AgdaModule, AgdaPassConfig,
    AgdaPassPhase, AgdaPassRegistry, AgdaPassStats, AgdaPattern, AgdaRecord, AgdaWorklist,
    AgdaX2Cache, AgdaX2ConstFolder, AgdaX2DepGraph, AgdaX2DomTree, AgdaX2Liveness,
    AgdaX2PassConfig, AgdaX2PassPhase, AgdaX2PassRegistry, AgdaX2PassStats, AgdaX2Worklist,
};

/// Escape special characters in an Agda string literal.
pub(super) fn escape_agda_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}
/// Emit `(x : T) (y : U)` parameter list.
pub(super) fn emit_agda_params(params: &[(String, AgdaExpr)], indent: usize) -> String {
    params
        .iter()
        .map(|(x, ty)| format!("({} : {})", x, ty.emit(indent)))
        .collect::<Vec<_>>()
        .join(" ")
}
/// Build `AgdaExpr::Var`.
pub fn var(name: impl Into<String>) -> AgdaExpr {
    AgdaExpr::Var(name.into())
}
/// Build a left-associative application chain: `f a1 a2 ...`.
pub fn app(func: AgdaExpr, args: Vec<AgdaExpr>) -> AgdaExpr {
    args.into_iter()
        .fold(func, |acc, a| AgdaExpr::App(Box::new(acc), Box::new(a)))
}
/// Build `f a` (single-argument application).
pub fn app1(func: AgdaExpr, arg: AgdaExpr) -> AgdaExpr {
    AgdaExpr::App(Box::new(func), Box::new(arg))
}
/// Build a non-dependent Pi type: `A → B`.
pub fn arrow(dom: AgdaExpr, cod: AgdaExpr) -> AgdaExpr {
    AgdaExpr::Pi(None, Box::new(dom), Box::new(cod))
}
/// Build a dependent Pi type: `(x : A) → B`.
pub fn pi(x: impl Into<String>, dom: AgdaExpr, cod: AgdaExpr) -> AgdaExpr {
    AgdaExpr::Pi(Some(x.into()), Box::new(dom), Box::new(cod))
}
/// Build a lambda: `λ x → body`.
pub fn lam(x: impl Into<String>, body: AgdaExpr) -> AgdaExpr {
    AgdaExpr::Lambda(x.into(), Box::new(body))
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn nat() -> AgdaExpr {
        var("ℕ")
    }
    pub(super) fn set0() -> AgdaExpr {
        AgdaExpr::Set(None)
    }
    pub(super) fn bool_t() -> AgdaExpr {
        var("Bool")
    }
    #[test]
    pub(super) fn test_pattern_var() {
        assert_eq!(AgdaPattern::Var("n".into()).to_string(), "n");
    }
    #[test]
    pub(super) fn test_pattern_wildcard() {
        assert_eq!(AgdaPattern::Wildcard.to_string(), "_");
    }
    #[test]
    pub(super) fn test_pattern_con_nullary() {
        assert_eq!(AgdaPattern::Con("zero".into(), vec![]).to_string(), "zero");
    }
    #[test]
    pub(super) fn test_pattern_con_unary() {
        let p = AgdaPattern::Con("suc".into(), vec![AgdaPattern::Var("n".into())]);
        assert_eq!(p.to_string(), "(suc n)");
    }
    #[test]
    pub(super) fn test_pattern_absurd() {
        assert_eq!(AgdaPattern::Absurd.to_string(), "()");
    }
    #[test]
    pub(super) fn test_pattern_implicit() {
        let p = AgdaPattern::Implicit(Box::new(AgdaPattern::Var("A".into())));
        assert_eq!(p.to_string(), "{A}");
    }
    #[test]
    pub(super) fn test_pattern_as() {
        let p = AgdaPattern::As(
            "xs".into(),
            Box::new(AgdaPattern::Con(
                "_∷_".into(),
                vec![AgdaPattern::Var("x".into()), AgdaPattern::Var("xt".into())],
            )),
        );
        assert_eq!(p.to_string(), "xs@(_∷_ x xt)");
    }
    #[test]
    pub(super) fn test_expr_var() {
        assert_eq!(var("ℕ").emit(0), "ℕ");
    }
    #[test]
    pub(super) fn test_expr_num() {
        assert_eq!(AgdaExpr::Num(42).emit(0), "42");
    }
    #[test]
    pub(super) fn test_expr_str() {
        assert_eq!(AgdaExpr::Str("hello".into()).emit(0), "\"hello\"");
    }
    #[test]
    pub(super) fn test_expr_set() {
        assert_eq!(AgdaExpr::Set(None).emit(0), "Set");
        assert_eq!(AgdaExpr::Set(Some(0)).emit(0), "Set");
        assert_eq!(AgdaExpr::Set(Some(1)).emit(0), "Set₁");
    }
    #[test]
    pub(super) fn test_expr_prop() {
        assert_eq!(AgdaExpr::Prop.emit(0), "Prop");
    }
    #[test]
    pub(super) fn test_expr_hole() {
        assert_eq!(AgdaExpr::Hole.emit(0), "{! !}");
    }
    #[test]
    pub(super) fn test_expr_app_simple() {
        let t = app1(var("suc"), var("n"));
        assert_eq!(t.emit(0), "suc n");
    }
    #[test]
    pub(super) fn test_expr_app_chain() {
        let t = app(var("_+_"), vec![var("m"), var("n")]);
        assert_eq!(t.emit(0), "_+_ m n");
    }
    #[test]
    pub(super) fn test_expr_app_nested_parens() {
        let inner = app1(var("suc"), var("n"));
        let outer = app1(var("suc"), inner);
        assert_eq!(outer.emit(0), "suc (suc n)");
    }
    #[test]
    pub(super) fn test_expr_lambda() {
        let t = lam("n", app1(var("suc"), var("n")));
        assert_eq!(t.emit(0), "λ n → suc n");
    }
    #[test]
    pub(super) fn test_expr_arrow() {
        let t = arrow(nat(), nat());
        assert_eq!(t.emit(0), "ℕ → ℕ");
    }
    #[test]
    pub(super) fn test_expr_pi_dependent() {
        let t = pi("n", nat(), app1(var("Vec"), var("n")));
        assert_eq!(t.emit(0), "(n : ℕ) → Vec n");
    }
    #[test]
    pub(super) fn test_expr_implicit() {
        let t = AgdaExpr::Implicit(Box::new(var("A")));
        assert_eq!(t.emit(0), "{A}");
    }
    #[test]
    pub(super) fn test_expr_tuple() {
        let t = AgdaExpr::Tuple(vec![AgdaExpr::Num(1), AgdaExpr::Num(2)]);
        assert_eq!(t.emit(0), "(1 , 2)");
    }
    #[test]
    pub(super) fn test_expr_record_construction() {
        let t = AgdaExpr::Record(vec![
            ("fst".into(), AgdaExpr::Num(1)),
            ("snd".into(), AgdaExpr::Num(2)),
        ]);
        assert_eq!(t.emit(0), "record { fst = 1 ; snd = 2 }");
    }
    #[test]
    pub(super) fn test_expr_ascription() {
        let t = AgdaExpr::Ascription(Box::new(AgdaExpr::Num(0)), Box::new(nat()));
        assert_eq!(t.emit(0), "(0 : ℕ)");
    }
    #[test]
    pub(super) fn test_expr_if_then_else() {
        let t = AgdaExpr::IfThenElse(
            Box::new(var("b")),
            Box::new(AgdaExpr::Num(1)),
            Box::new(AgdaExpr::Num(0)),
        );
        assert_eq!(t.emit(0), "if b then 1 else 0");
    }
    #[test]
    pub(super) fn test_clause_base_case() {
        let c = AgdaClause {
            patterns: vec![AgdaPattern::Con("zero".into(), vec![])],
            rhs: Some(AgdaExpr::Num(0)),
            where_decls: vec![],
        };
        let s = c.emit_clause("factorial", 0);
        assert_eq!(s, "factorial zero = 0");
    }
    #[test]
    pub(super) fn test_clause_recursive() {
        let c = AgdaClause {
            patterns: vec![AgdaPattern::Con(
                "suc".into(),
                vec![AgdaPattern::Var("n".into())],
            )],
            rhs: Some(app(
                var("_*_"),
                vec![app1(var("suc"), var("n")), app1(var("factorial"), var("n"))],
            )),
            where_decls: vec![],
        };
        let s = c.emit_clause("factorial", 0);
        assert!(s.starts_with("factorial (suc n) ="));
    }
    #[test]
    pub(super) fn test_decl_func_type() {
        let d = AgdaDecl::FuncType {
            name: "double".into(),
            ty: arrow(nat(), nat()),
        };
        assert_eq!(d.emit(0), "double : ℕ → ℕ");
    }
    #[test]
    pub(super) fn test_decl_func_def_two_clauses() {
        let d = AgdaDecl::FuncDef {
            name: "_+_".into(),
            clauses: vec![
                AgdaClause {
                    patterns: vec![
                        AgdaPattern::Con("zero".into(), vec![]),
                        AgdaPattern::Var("m".into()),
                    ],
                    rhs: Some(var("m")),
                    where_decls: vec![],
                },
                AgdaClause {
                    patterns: vec![
                        AgdaPattern::Con("suc".into(), vec![AgdaPattern::Var("n".into())]),
                        AgdaPattern::Var("m".into()),
                    ],
                    rhs: Some(app1(var("suc"), app(var("_+_"), vec![var("n"), var("m")]))),
                    where_decls: vec![],
                },
            ],
        };
        let s = d.emit(0);
        assert!(s.contains("_+_ zero m = m"));
        assert!(s.contains("_+_ (suc n) m = suc (_+_ n m)"));
    }
    #[test]
    pub(super) fn test_decl_data_nat() {
        let d = AgdaDecl::DataDecl(AgdaData {
            name: "ℕ".into(),
            params: vec![],
            indices: set0(),
            constructors: vec![
                AgdaConstructor {
                    name: "zero".into(),
                    ty: var("ℕ"),
                },
                AgdaConstructor {
                    name: "suc".into(),
                    ty: arrow(var("ℕ"), var("ℕ")),
                },
            ],
        });
        let s = d.emit(0);
        assert!(s.contains("data ℕ : Set where"));
        assert!(s.contains("zero : ℕ"));
        assert!(s.contains("suc : ℕ → ℕ"));
    }
    #[test]
    pub(super) fn test_decl_data_list() {
        let d = AgdaDecl::DataDecl(AgdaData {
            name: "List".into(),
            params: vec![("A".into(), set0())],
            indices: set0(),
            constructors: vec![
                AgdaConstructor {
                    name: "[]".into(),
                    ty: app1(var("List"), var("A")),
                },
                AgdaConstructor {
                    name: "_∷_".into(),
                    ty: arrow(
                        var("A"),
                        arrow(app1(var("List"), var("A")), app1(var("List"), var("A"))),
                    ),
                },
            ],
        });
        let s = d.emit(0);
        assert!(s.contains("data List (A : Set) : Set where"));
        assert!(s.contains("[] : List A"));
        assert!(s.contains("_∷_ : A → List A → List A"));
    }
    #[test]
    pub(super) fn test_decl_record_sigma() {
        let d = AgdaDecl::RecordDecl(AgdaRecord {
            name: "Σ".into(),
            params: vec![("A".into(), set0()), ("B".into(), arrow(var("A"), set0()))],
            universe: set0(),
            constructor: Some("_,_".into()),
            fields: vec![
                AgdaField {
                    name: "fst".into(),
                    ty: var("A"),
                },
                AgdaField {
                    name: "snd".into(),
                    ty: app1(var("B"), var("fst")),
                },
            ],
            copattern_defs: vec![],
        });
        let s = d.emit(0);
        assert!(s.contains("record Σ (A : Set) (B : A → Set) : Set where"));
        assert!(s.contains("constructor _,_"));
        assert!(s.contains("fst : A"));
        assert!(s.contains("snd : B fst"));
    }
    #[test]
    pub(super) fn test_decl_postulate() {
        let d = AgdaDecl::Postulate(vec![(
            "LEM".into(),
            pi(
                "P",
                var("Prop"),
                app(var("_⊎_"), vec![var("P"), app1(var("¬"), var("P"))]),
            ),
        )]);
        let s = d.emit(0);
        assert!(s.starts_with("postulate"));
        assert!(s.contains("LEM : (P : Prop) → _⊎_ P (¬ P)"));
    }
    #[test]
    pub(super) fn test_decl_import() {
        let d = AgdaDecl::Import("Data.Nat".into());
        assert_eq!(d.emit(0), "import Data.Nat");
    }
    #[test]
    pub(super) fn test_decl_open() {
        let d = AgdaDecl::Open("Data.Nat".into());
        assert_eq!(d.emit(0), "open Data.Nat");
    }
    #[test]
    pub(super) fn test_decl_variable() {
        let d = AgdaDecl::Variable(vec![("A".into(), set0()), ("B".into(), set0())]);
        let s = d.emit(0);
        assert!(s.contains("variable"));
        assert!(s.contains("{A : Set}"));
        assert!(s.contains("{B : Set}"));
    }
    #[test]
    pub(super) fn test_decl_module() {
        let d = AgdaDecl::ModuleDecl {
            name: "Inner".into(),
            params: vec![],
            body: vec![AgdaDecl::FuncType {
                name: "id".into(),
                ty: arrow(bool_t(), bool_t()),
            }],
        };
        let s = d.emit(0);
        assert!(s.contains("module Inner where"));
        assert!(s.contains("id : Bool → Bool"));
    }
    #[test]
    pub(super) fn test_decl_pragma() {
        let d = AgdaDecl::Pragma("BUILTIN NATURAL ℕ".into());
        assert_eq!(d.emit(0), "{-# BUILTIN NATURAL ℕ #-}");
    }
    #[test]
    pub(super) fn test_decl_comment() {
        let d = AgdaDecl::Comment("Identity function".into());
        assert_eq!(d.emit(0), "-- Identity function");
    }
    #[test]
    pub(super) fn test_module_emit_basic() {
        let mut m = AgdaModule::new("MyModule");
        m.import("Data.Nat");
        m.open("Data.Nat");
        m.add(AgdaDecl::Comment("a function".into()));
        m.add(AgdaDecl::FuncType {
            name: "f".into(),
            ty: arrow(nat(), nat()),
        });
        m.add(AgdaDecl::FuncDef {
            name: "f".into(),
            clauses: vec![AgdaClause {
                patterns: vec![AgdaPattern::Var("n".into())],
                rhs: Some(var("n")),
                where_decls: vec![],
            }],
        });
        let s = m.emit();
        assert!(s.contains("module MyModule where"));
        assert!(s.contains("import Data.Nat"));
        assert!(s.contains("open Data.Nat"));
        assert!(s.contains("-- a function"));
        assert!(s.contains("f : ℕ → ℕ"));
        assert!(s.contains("f n = n"));
    }
    #[test]
    pub(super) fn test_module_parameterised() {
        let mut m = AgdaModule::new("Container");
        m.params.push(("A".into(), set0()));
        let s = m.emit();
        assert!(s.contains("module Container (A : Set) where"));
    }
    #[test]
    pub(super) fn test_module_full_identity_proof() {
        let mut m = AgdaModule::new("IdentityProof");
        m.import("Relation.Binary.PropositionalEquality");
        m.open("Relation.Binary.PropositionalEquality");
        m.add(AgdaDecl::FuncType {
            name: "refl-is-refl".into(),
            ty: pi(
                "A",
                set0(),
                pi("x", var("A"), app(var("_≡_"), vec![var("x"), var("x")])),
            ),
        });
        m.add(AgdaDecl::FuncDef {
            name: "refl-is-refl".into(),
            clauses: vec![AgdaClause {
                patterns: vec![AgdaPattern::Wildcard, AgdaPattern::Wildcard],
                rhs: Some(var("refl")),
                where_decls: vec![],
            }],
        });
        let s = m.emit();
        assert!(s.contains("refl-is-refl : (A : Set) → (x : A) → _≡_ x x"));
        assert!(s.contains("refl-is-refl _ _ = refl"));
    }
}
#[cfg(test)]
mod Agda_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = AgdaPassConfig::new("test_pass", AgdaPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = AgdaPassStats::new();
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
        let mut reg = AgdaPassRegistry::new();
        reg.register(AgdaPassConfig::new("pass_a", AgdaPassPhase::Analysis));
        reg.register(AgdaPassConfig::new("pass_b", AgdaPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = AgdaAnalysisCache::new(10);
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
        let mut wl = AgdaWorklist::new();
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
        let mut dt = AgdaDominatorTree::new(5);
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
        let mut liveness = AgdaLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(AgdaConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(AgdaConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(AgdaConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            AgdaConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(AgdaConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = AgdaDepGraph::new();
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
mod agdaext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_agdaext_phase_order() {
        assert_eq!(AgdaExtPassPhase::Early.order(), 0);
        assert_eq!(AgdaExtPassPhase::Middle.order(), 1);
        assert_eq!(AgdaExtPassPhase::Late.order(), 2);
        assert_eq!(AgdaExtPassPhase::Finalize.order(), 3);
        assert!(AgdaExtPassPhase::Early.is_early());
        assert!(!AgdaExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_agdaext_config_builder() {
        let c = AgdaExtPassConfig::new("p")
            .with_phase(AgdaExtPassPhase::Late)
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
    pub(super) fn test_agdaext_stats() {
        let mut s = AgdaExtPassStats::new();
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
    pub(super) fn test_agdaext_registry() {
        let mut r = AgdaExtPassRegistry::new();
        r.register(AgdaExtPassConfig::new("a").with_phase(AgdaExtPassPhase::Early));
        r.register(AgdaExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&AgdaExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_agdaext_cache() {
        let mut c = AgdaExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_agdaext_worklist() {
        let mut w = AgdaExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_agdaext_dom_tree() {
        let mut dt = AgdaExtDomTree::new(5);
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
    pub(super) fn test_agdaext_liveness() {
        let mut lv = AgdaExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_agdaext_const_folder() {
        let mut cf = AgdaExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_agdaext_dep_graph() {
        let mut g = AgdaExtDepGraph::new(4);
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
#[cfg(test)]
mod agdax2_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_agdax2_phase_order() {
        assert_eq!(AgdaX2PassPhase::Early.order(), 0);
        assert_eq!(AgdaX2PassPhase::Middle.order(), 1);
        assert_eq!(AgdaX2PassPhase::Late.order(), 2);
        assert_eq!(AgdaX2PassPhase::Finalize.order(), 3);
        assert!(AgdaX2PassPhase::Early.is_early());
        assert!(!AgdaX2PassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_agdax2_config_builder() {
        let c = AgdaX2PassConfig::new("p")
            .with_phase(AgdaX2PassPhase::Late)
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
    pub(super) fn test_agdax2_stats() {
        let mut s = AgdaX2PassStats::new();
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
    pub(super) fn test_agdax2_registry() {
        let mut r = AgdaX2PassRegistry::new();
        r.register(AgdaX2PassConfig::new("a").with_phase(AgdaX2PassPhase::Early));
        r.register(AgdaX2PassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&AgdaX2PassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_agdax2_cache() {
        let mut c = AgdaX2Cache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_agdax2_worklist() {
        let mut w = AgdaX2Worklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_agdax2_dom_tree() {
        let mut dt = AgdaX2DomTree::new(5);
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
    pub(super) fn test_agdax2_liveness() {
        let mut lv = AgdaX2Liveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_agdax2_const_folder() {
        let mut cf = AgdaX2ConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_agdax2_dep_graph() {
        let mut g = AgdaX2DepGraph::new(4);
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
