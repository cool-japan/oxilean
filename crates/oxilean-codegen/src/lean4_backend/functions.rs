//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    L4AnalysisCache, L4ConstantFoldingHelper, L4DepGraph, L4DominatorTree, L4ExtCache,
    L4ExtConstFolder, L4ExtDepGraph, L4ExtDomTree, L4ExtLiveness, L4ExtPassConfig, L4ExtPassPhase,
    L4ExtPassRegistry, L4ExtPassStats, L4ExtWorklist, L4LivenessInfo, L4PassConfig, L4PassPhase,
    L4PassRegistry, L4PassStats, L4Worklist, Lean4Abbrev, Lean4Axiom, Lean4Backend, Lean4CalcStep,
    Lean4Constructor, Lean4Decl, Lean4Def, Lean4DoStmt, Lean4Expr, Lean4File, Lean4Inductive,
    Lean4Instance, Lean4Pattern, Lean4Structure, Lean4Theorem, Lean4Type,
};

/// Wrap a type in parens if it's complex (for use as an argument).
pub(super) fn paren_type(ty: &Lean4Type) -> std::string::String {
    match ty {
        Lean4Type::App(_, _)
        | Lean4Type::Fun(_, _)
        | Lean4Type::ForAll(_, _, _)
        | Lean4Type::List(_)
        | Lean4Type::Option(_)
        | Lean4Type::Prod(_, _)
        | Lean4Type::Sum(_, _)
        | Lean4Type::IO(_)
        | Lean4Type::Array(_)
        | Lean4Type::Fin(_) => format!("({})", ty),
        _ => ty.to_string(),
    }
}
/// Wrap a type in parens for product/sum (avoids ambiguity with →).
pub(super) fn paren_complex_type(ty: &Lean4Type) -> std::string::String {
    match ty {
        Lean4Type::Fun(_, _) | Lean4Type::ForAll(_, _, _) => format!("({})", ty),
        _ => ty.to_string(),
    }
}
/// Wrap a function domain in parens if it's also a function.
pub(super) fn paren_fun_type(ty: &Lean4Type) -> std::string::String {
    match ty {
        Lean4Type::Fun(_, _) | Lean4Type::ForAll(_, _, _) => format!("({})", ty),
        _ => ty.to_string(),
    }
}
/// Wrap an expression in parens if it is complex.
pub(super) fn paren_expr(e: &Lean4Expr) -> std::string::String {
    match e {
        Lean4Expr::App(_, _)
        | Lean4Expr::Lambda(_, _, _)
        | Lean4Expr::If(_, _, _)
        | Lean4Expr::Let(_, _, _, _)
        | Lean4Expr::LetRec(_, _, _)
        | Lean4Expr::Match(_, _)
        | Lean4Expr::Have(_, _, _, _)
        | Lean4Expr::Show(_, _)
        | Lean4Expr::Do(_)
        | Lean4Expr::Calc(_)
        | Lean4Expr::ByTactic(_) => format!("({})", e),
        _ => e.to_string(),
    }
}
/// Wrap a pattern in parens if complex.
pub(super) fn paren_pattern(p: &Lean4Pattern) -> std::string::String {
    match p {
        Lean4Pattern::Ctor(_, args) if !args.is_empty() => format!("({})", p),
        Lean4Pattern::Or(_, _) => format!("({})", p),
        _ => p.to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_type_nat() {
        assert_eq!(Lean4Type::Nat.to_string(), "Nat");
    }
    #[test]
    pub(super) fn test_type_bool() {
        assert_eq!(Lean4Type::Bool.to_string(), "Bool");
    }
    #[test]
    pub(super) fn test_type_prop() {
        assert_eq!(Lean4Type::Prop.to_string(), "Prop");
    }
    #[test]
    pub(super) fn test_type_type_zero() {
        assert_eq!(Lean4Type::Type(0).to_string(), "Type");
    }
    #[test]
    pub(super) fn test_type_type_nonzero() {
        assert_eq!(Lean4Type::Type(1).to_string(), "Type 1");
    }
    #[test]
    pub(super) fn test_type_list() {
        let ty = Lean4Type::List(Box::new(Lean4Type::Nat));
        assert_eq!(ty.to_string(), "List Nat");
    }
    #[test]
    pub(super) fn test_type_option() {
        let ty = Lean4Type::Option(Box::new(Lean4Type::Int));
        assert_eq!(ty.to_string(), "Option Int");
    }
    #[test]
    pub(super) fn test_type_prod() {
        let ty = Lean4Type::Prod(Box::new(Lean4Type::Nat), Box::new(Lean4Type::Bool));
        assert_eq!(ty.to_string(), "Nat × Bool");
    }
    #[test]
    pub(super) fn test_type_fun() {
        let ty = Lean4Type::Fun(Box::new(Lean4Type::Nat), Box::new(Lean4Type::Bool));
        assert_eq!(ty.to_string(), "Nat → Bool");
    }
    #[test]
    pub(super) fn test_type_forall() {
        let ty = Lean4Type::ForAll(
            "n".to_string(),
            Box::new(Lean4Type::Nat),
            Box::new(Lean4Type::Prop),
        );
        assert_eq!(ty.to_string(), "∀ (n : Nat), Prop");
    }
    #[test]
    pub(super) fn test_type_io() {
        let ty = Lean4Type::IO(Box::new(Lean4Type::Unit));
        assert_eq!(ty.to_string(), "IO Unit");
    }
    #[test]
    pub(super) fn test_type_array() {
        let ty = Lean4Type::Array(Box::new(Lean4Type::Float));
        assert_eq!(ty.to_string(), "Array Float");
    }
    #[test]
    pub(super) fn test_expr_var() {
        let e = Lean4Expr::Var("x".to_string());
        assert_eq!(e.to_string(), "x");
    }
    #[test]
    pub(super) fn test_expr_nat_lit() {
        let e = Lean4Expr::NatLit(42);
        assert_eq!(e.to_string(), "42");
    }
    #[test]
    pub(super) fn test_expr_bool_lit() {
        let e = Lean4Expr::BoolLit(true);
        assert_eq!(e.to_string(), "true");
    }
    #[test]
    pub(super) fn test_expr_str_lit() {
        let e = Lean4Expr::StrLit("hello".to_string());
        assert_eq!(e.to_string(), "\"hello\"");
    }
    #[test]
    pub(super) fn test_expr_str_lit_escaping() {
        let e = Lean4Expr::StrLit("say \"hi\"".to_string());
        assert!(e.to_string().contains("\\\""));
    }
    #[test]
    pub(super) fn test_expr_sorry() {
        let e = Lean4Expr::Sorry;
        assert_eq!(e.to_string(), "sorry");
    }
    #[test]
    pub(super) fn test_expr_hole() {
        let e = Lean4Expr::Hole;
        assert_eq!(e.to_string(), "_");
    }
    #[test]
    pub(super) fn test_expr_app() {
        let e = Lean4Expr::App(
            Box::new(Lean4Expr::Var("f".to_string())),
            Box::new(Lean4Expr::Var("x".to_string())),
        );
        assert_eq!(e.to_string(), "f x");
    }
    #[test]
    pub(super) fn test_expr_lambda_untyped() {
        let e = Lean4Expr::Lambda(
            "x".to_string(),
            None,
            Box::new(Lean4Expr::Var("x".to_string())),
        );
        assert_eq!(e.to_string(), "fun x => x");
    }
    #[test]
    pub(super) fn test_expr_lambda_typed() {
        let e = Lean4Expr::Lambda(
            "x".to_string(),
            Some(Box::new(Lean4Type::Nat)),
            Box::new(Lean4Expr::Var("x".to_string())),
        );
        assert_eq!(e.to_string(), "fun (x : Nat) => x");
    }
    #[test]
    pub(super) fn test_expr_if() {
        let e = Lean4Expr::If(
            Box::new(Lean4Expr::BoolLit(true)),
            Box::new(Lean4Expr::NatLit(1)),
            Box::new(Lean4Expr::NatLit(0)),
        );
        assert_eq!(e.to_string(), "if true then 1 else 0");
    }
    #[test]
    pub(super) fn test_expr_tuple() {
        let e = Lean4Expr::Tuple(vec![Lean4Expr::NatLit(1), Lean4Expr::NatLit(2)]);
        assert_eq!(e.to_string(), "(1, 2)");
    }
    #[test]
    pub(super) fn test_expr_anonymous_ctor() {
        let e = Lean4Expr::AnonymousCtor(vec![Lean4Expr::NatLit(1), Lean4Expr::NatLit(2)]);
        assert_eq!(e.to_string(), "⟨1, 2⟩");
    }
    #[test]
    pub(super) fn test_expr_by_tactic() {
        let e = Lean4Expr::ByTactic(vec!["simp".to_string(), "exact h".to_string()]);
        assert_eq!(e.to_string(), "by simp; exact h");
    }
    #[test]
    pub(super) fn test_pattern_wildcard() {
        assert_eq!(Lean4Pattern::Wildcard.to_string(), "_");
    }
    #[test]
    pub(super) fn test_pattern_var() {
        assert_eq!(Lean4Pattern::Var("x".to_string()).to_string(), "x");
    }
    #[test]
    pub(super) fn test_pattern_ctor_noargs() {
        let p = Lean4Pattern::Ctor("none".to_string(), vec![]);
        assert_eq!(p.to_string(), "none");
    }
    #[test]
    pub(super) fn test_pattern_ctor_with_args() {
        let p = Lean4Pattern::Ctor("some".to_string(), vec![Lean4Pattern::Var("x".to_string())]);
        assert_eq!(p.to_string(), "some x");
    }
    #[test]
    pub(super) fn test_pattern_tuple() {
        let p = Lean4Pattern::Tuple(vec![
            Lean4Pattern::Var("a".to_string()),
            Lean4Pattern::Var("b".to_string()),
        ]);
        assert_eq!(p.to_string(), "(a, b)");
    }
    #[test]
    pub(super) fn test_pattern_or() {
        let p = Lean4Pattern::Or(
            Box::new(Lean4Pattern::Lit("0".to_string())),
            Box::new(Lean4Pattern::Lit("1".to_string())),
        );
        assert_eq!(p.to_string(), "0 | 1");
    }
    #[test]
    pub(super) fn test_def_emit_simple() {
        let def = Lean4Def::simple(
            "id",
            vec![("x".to_string(), Lean4Type::Nat)],
            Lean4Type::Nat,
            Lean4Expr::Var("x".to_string()),
        );
        let out = def.emit();
        assert!(out.contains("def id"));
        assert!(out.contains("(x : Nat)"));
        assert!(out.contains(": Nat"));
        assert!(out.contains(":="));
        assert!(out.contains("x"));
    }
    #[test]
    pub(super) fn test_def_emit_with_attribute() {
        let mut def = Lean4Def::simple("myFn", vec![], Lean4Type::Nat, Lean4Expr::NatLit(0));
        def.attributes.push("simp".to_string());
        let out = def.emit();
        assert!(out.contains("@[simp]"));
    }
    #[test]
    pub(super) fn test_def_emit_noncomputable() {
        let mut def = Lean4Def::simple(
            "realVal",
            vec![],
            Lean4Type::Float,
            Lean4Expr::FloatLit(3.14),
        );
        def.is_noncomputable = true;
        let out = def.emit();
        assert!(out.contains("noncomputable"));
    }
    #[test]
    pub(super) fn test_def_emit_private() {
        let mut def = Lean4Def::simple("helper", vec![], Lean4Type::Nat, Lean4Expr::NatLit(0));
        def.is_private = true;
        let out = def.emit();
        assert!(out.contains("private"));
    }
    #[test]
    pub(super) fn test_theorem_emit_tactic() {
        let thm = Lean4Theorem::tactic(
            "add_zero",
            vec![("n".to_string(), Lean4Type::Nat)],
            Lean4Type::Custom("n + 0 = n".to_string()),
            vec!["simp".to_string()],
        );
        let out = thm.emit();
        assert!(out.contains("theorem add_zero"));
        assert!(out.contains("(n : Nat)"));
        assert!(out.contains("by simp"));
    }
    #[test]
    pub(super) fn test_theorem_emit_term_mode() {
        let thm = Lean4Theorem::term_mode(
            "trivial_thm",
            vec![],
            Lean4Type::Custom("True".to_string()),
            Lean4Expr::Var("trivial".to_string()),
        );
        let out = thm.emit();
        assert!(out.contains("theorem trivial_thm"));
        assert!(out.contains("trivial"));
    }
    #[test]
    pub(super) fn test_inductive_emit_simple() {
        let ind = Lean4Inductive::simple(
            "Color",
            vec![
                Lean4Constructor::positional("red", vec![]),
                Lean4Constructor::positional("green", vec![]),
                Lean4Constructor::positional("blue", vec![]),
            ],
        );
        let out = ind.emit();
        assert!(out.contains("inductive Color"));
        assert!(out.contains("| red"));
        assert!(out.contains("| green"));
        assert!(out.contains("| blue"));
    }
    #[test]
    pub(super) fn test_inductive_emit_with_fields() {
        let ind = Lean4Inductive::simple(
            "Expr",
            vec![
                Lean4Constructor::positional("lit", vec![Lean4Type::Nat]),
                Lean4Constructor::positional(
                    "add",
                    vec![
                        Lean4Type::Custom("Expr".to_string()),
                        Lean4Type::Custom("Expr".to_string()),
                    ],
                ),
            ],
        );
        let out = ind.emit();
        assert!(out.contains("inductive Expr"));
        assert!(out.contains("| lit"));
        assert!(out.contains("Nat"));
    }
    #[test]
    pub(super) fn test_structure_emit() {
        let s = Lean4Structure::simple(
            "Point",
            vec![
                ("x".to_string(), Lean4Type::Float),
                ("y".to_string(), Lean4Type::Float),
            ],
        );
        let out = s.emit();
        assert!(out.contains("structure Point"));
        assert!(out.contains("x : Float"));
        assert!(out.contains("y : Float"));
    }
    #[test]
    pub(super) fn test_file_emit_imports() {
        let file = Lean4File::new()
            .with_import("Mathlib.Data.Nat.Basic")
            .with_open("Nat");
        let out = file.emit();
        assert!(out.contains("import Mathlib.Data.Nat.Basic"));
        assert!(out.contains("open Nat"));
    }
    #[test]
    pub(super) fn test_file_emit_with_def() {
        let mut file = Lean4File::new();
        let def = Lean4Def::simple(
            "double",
            vec![("n".to_string(), Lean4Type::Nat)],
            Lean4Type::Nat,
            Lean4Expr::App(
                Box::new(Lean4Expr::App(
                    Box::new(Lean4Expr::Var("HAdd.hAdd".to_string())),
                    Box::new(Lean4Expr::Var("n".to_string())),
                )),
                Box::new(Lean4Expr::Var("n".to_string())),
            ),
        );
        file.add_decl(Lean4Decl::Def(def));
        let out = file.emit();
        assert!(out.contains("def double"));
        assert!(out.contains("(n : Nat)"));
    }
    #[test]
    pub(super) fn test_backend_compile_kernel_decl() {
        let mut backend = Lean4Backend::new();
        backend.compile_kernel_decl(
            "identity",
            vec![("x".to_string(), Lean4Type::Nat)],
            Lean4Type::Nat,
            Lean4Expr::Var("x".to_string()),
        );
        let out = backend.emit_file();
        assert!(out.contains("def identity"));
    }
    #[test]
    pub(super) fn test_backend_add_theorem() {
        let mut backend = Lean4Backend::new();
        backend.add_theorem(
            "nat_eq_refl",
            vec![("n".to_string(), Lean4Type::Nat)],
            Lean4Type::Custom("n = n".to_string()),
            vec!["rfl".to_string()],
        );
        let out = backend.emit_file();
        assert!(out.contains("theorem nat_eq_refl"));
        assert!(out.contains("by rfl"));
    }
    #[test]
    pub(super) fn test_backend_add_inductive() {
        let mut backend = Lean4Backend::new();
        let ind = Lean4Inductive::simple(
            "Tree",
            vec![
                Lean4Constructor::positional("leaf", vec![]),
                Lean4Constructor::positional(
                    "node",
                    vec![
                        Lean4Type::Custom("Tree".to_string()),
                        Lean4Type::Nat,
                        Lean4Type::Custom("Tree".to_string()),
                    ],
                ),
            ],
        );
        backend.add_inductive(ind);
        let out = backend.emit_file();
        assert!(out.contains("inductive Tree"));
        assert!(out.contains("leaf"));
        assert!(out.contains("node"));
    }
    #[test]
    pub(super) fn test_namespace_emit() {
        let decl = Lean4Decl::Namespace(
            "MyNS".to_string(),
            vec![Lean4Decl::Def(Lean4Def::simple(
                "foo",
                vec![],
                Lean4Type::Nat,
                Lean4Expr::NatLit(0),
            ))],
        );
        let out = decl.emit();
        assert!(out.contains("namespace MyNS"));
        assert!(out.contains("end MyNS"));
        assert!(out.contains("def foo"));
    }
    #[test]
    pub(super) fn test_section_emit() {
        let decl = Lean4Decl::Section(
            "Aux".to_string(),
            vec![Lean4Decl::Check(Lean4Expr::NatLit(42))],
        );
        let out = decl.emit();
        assert!(out.contains("section Aux"));
        assert!(out.contains("end Aux"));
        assert!(out.contains("#check"));
    }
    #[test]
    pub(super) fn test_axiom_emit() {
        let ax = Lean4Axiom {
            name: "classical".to_string(),
            args: vec![("p".to_string(), Lean4Type::Prop)],
            ty: Lean4Type::Custom("p ∨ ¬p".to_string()),
            doc_comment: Some("Law of excluded middle".to_string()),
        };
        let out = ax.emit();
        assert!(out.contains("axiom classical"));
        assert!(out.contains("(p : Prop)"));
        assert!(out.contains("Law of excluded middle"));
    }
    #[test]
    pub(super) fn test_abbrev_emit() {
        let a = Lean4Abbrev {
            name: "MyNat".to_string(),
            args: vec![],
            ty: Some(Lean4Type::Type(0)),
            body: Lean4Expr::Var("Nat".to_string()),
        };
        let out = a.emit();
        assert!(out.contains("abbrev MyNat"));
        assert!(out.contains("Nat"));
    }
    #[test]
    pub(super) fn test_instance_emit() {
        let inst = Lean4Instance {
            name: Some("instAddNat".to_string()),
            ty: Lean4Type::Custom("Add Nat".to_string()),
            args: vec![],
            body: Lean4Expr::StructLit(
                "".to_string(),
                vec![("add".to_string(), Lean4Expr::Var("Nat.add".to_string()))],
            ),
        };
        let out = inst.emit();
        assert!(out.contains("instance instAddNat"));
        assert!(out.contains("Add Nat"));
    }
    #[test]
    pub(super) fn test_do_notation() {
        let stmt = Lean4DoStmt::Bind(
            "line".to_string(),
            None,
            Box::new(Lean4Expr::Var("IO.getLine".to_string())),
        );
        let s = stmt.to_string();
        assert!(s.contains("let line ←"));
        assert!(s.contains("IO.getLine"));
    }
    #[test]
    pub(super) fn test_calc_step() {
        let step = Lean4CalcStep {
            lhs: Lean4Expr::Var("a".to_string()),
            relation: "=".to_string(),
            rhs: Lean4Expr::Var("b".to_string()),
            justification: Lean4Expr::Var("h".to_string()),
        };
        let s = step.to_string();
        assert!(s.contains("a = b := h"));
    }
    #[test]
    pub(super) fn test_module_doc_emit() {
        let mut file = Lean4File::new();
        file.module_doc = Some("This is a module.".to_string());
        let out = file.emit();
        assert!(out.contains("/-!"));
        assert!(out.contains("This is a module."));
        assert!(out.contains("-/"));
    }
    #[test]
    pub(super) fn test_structure_with_extends() {
        let mut s = Lean4Structure::simple("Point3D", vec![("z".to_string(), Lean4Type::Float)]);
        s.extends.push("Point".to_string());
        let out = s.emit();
        assert!(out.contains("extends Point"));
    }
    #[test]
    pub(super) fn test_inductive_with_derives() {
        let mut ind = Lean4Inductive::simple(
            "MyType",
            vec![Lean4Constructor::positional("mk", vec![Lean4Type::Nat])],
        );
        ind.derives.push("Repr".to_string());
        ind.derives.push("DecidableEq".to_string());
        let out = ind.emit();
        assert!(out.contains("deriving Repr, DecidableEq"));
    }
}
#[cfg(test)]
mod L4_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = L4PassConfig::new("test_pass", L4PassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = L4PassStats::new();
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
        let mut reg = L4PassRegistry::new();
        reg.register(L4PassConfig::new("pass_a", L4PassPhase::Analysis));
        reg.register(L4PassConfig::new("pass_b", L4PassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = L4AnalysisCache::new(10);
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
        let mut wl = L4Worklist::new();
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
        let mut dt = L4DominatorTree::new(5);
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
        let mut liveness = L4LivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(L4ConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(L4ConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(L4ConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            L4ConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(L4ConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = L4DepGraph::new();
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
mod l4ext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_l4ext_phase_order() {
        assert_eq!(L4ExtPassPhase::Early.order(), 0);
        assert_eq!(L4ExtPassPhase::Middle.order(), 1);
        assert_eq!(L4ExtPassPhase::Late.order(), 2);
        assert_eq!(L4ExtPassPhase::Finalize.order(), 3);
        assert!(L4ExtPassPhase::Early.is_early());
        assert!(!L4ExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_l4ext_config_builder() {
        let c = L4ExtPassConfig::new("p")
            .with_phase(L4ExtPassPhase::Late)
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
    pub(super) fn test_l4ext_stats() {
        let mut s = L4ExtPassStats::new();
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
    pub(super) fn test_l4ext_registry() {
        let mut r = L4ExtPassRegistry::new();
        r.register(L4ExtPassConfig::new("a").with_phase(L4ExtPassPhase::Early));
        r.register(L4ExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&L4ExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_l4ext_cache() {
        let mut c = L4ExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_l4ext_worklist() {
        let mut w = L4ExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_l4ext_dom_tree() {
        let mut dt = L4ExtDomTree::new(5);
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
    pub(super) fn test_l4ext_liveness() {
        let mut lv = L4ExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_l4ext_const_folder() {
        let mut cf = L4ExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_l4ext_dep_graph() {
        let mut g = L4ExtDepGraph::new(4);
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
