//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Literal, Name};

use super::types::{
    AttributeRegistry, ClassInstance, CoercionExt, CoercionRegistryExt, DeclAttribute, DeclKind,
    ElabConfig, ElabErrorCode, ElabMetrics, ElabNote, ElabNoteSet, ElabPipelineRegistry, ElabStage,
    ElabStats, EnvSnapshotManager, InstanceRegistry, NamespaceManager, ProofHistory,
    ProofStateSnapshot, Reducibility, UniverseCheckMode,
};

/// Well-known attribute names used in elaboration.
#[allow(dead_code)]
pub mod attr_names {
    /// `@[simp]` marks a lemma for use by simp.
    pub const SIMP: &str = "simp";
    /// `@[reducible]` marks a definition as always unfolded.
    pub const REDUCIBLE: &str = "reducible";
    /// `@[semireducible]` default reducibility.
    pub const SEMIREDUCIBLE: &str = "semireducible";
    /// `@[irreducible]` never unfolded.
    pub const IRREDUCIBLE: &str = "irreducible";
    /// `@[inline]` hint to inline during code generation.
    pub const INLINE: &str = "inline";
    /// `@[instance]` typeclass instance.
    pub const INSTANCE: &str = "instance";
    /// `@[class]` typeclass definition.
    pub const CLASS: &str = "class";
    /// `@[derive]` automatic instance derivation.
    pub const DERIVE: &str = "derive";
    /// `@[ext]` extensionality lemma.
    pub const EXT: &str = "ext";
    /// `@[norm_cast]` for norm_cast / push_cast tactics.
    pub const NORM_CAST: &str = "norm_cast";
    /// `@[protected]` name requires qualified access.
    pub const PROTECTED: &str = "protected";
    /// `@[macro]` macro definition.
    pub const MACRO: &str = "macro";
}
#[cfg(test)]
mod lib_tests {
    use super::*;
    #[test]
    fn test_elab_config_default() {
        let cfg = ElabConfig::default();
        assert_eq!(cfg.max_depth, 512);
        assert!(cfg.kernel_check);
        assert!(!cfg.allow_sorry);
    }
    #[test]
    fn test_elab_config_interactive() {
        let cfg = ElabConfig::interactive();
        assert!(cfg.allow_sorry);
        assert!(!cfg.strict_instances);
    }
    #[test]
    fn test_elab_config_strict() {
        let cfg = ElabConfig::strict();
        assert!(!cfg.allow_sorry);
        assert!(cfg.strict_instances);
        assert!(cfg.kernel_check);
    }
    #[test]
    fn test_elab_config_batch() {
        let cfg = ElabConfig::batch();
        assert!(!cfg.allow_sorry);
        assert!(!cfg.trace_elaboration);
    }
    #[test]
    fn test_elab_config_debug() {
        let cfg = ElabConfig::debug();
        assert!(cfg.trace_elaboration);
    }
    #[test]
    fn test_elab_stats_default() {
        let s = ElabStats::new();
        assert_eq!(s.num_decls, 0);
        assert!(s.all_mvars_solved());
        assert_eq!(s.mvar_solve_rate(), 1.0);
    }
    #[test]
    fn test_elab_stats_merge() {
        let mut s1 = ElabStats {
            num_decls: 3,
            num_mvars_created: 5,
            num_mvars_solved: 5,
            ..Default::default()
        };
        let s2 = ElabStats {
            num_decls: 2,
            num_mvars_created: 3,
            num_mvars_solved: 2,
            max_depth_reached: 100,
            ..Default::default()
        };
        s1.merge(&s2);
        assert_eq!(s1.num_decls, 5);
        assert_eq!(s1.num_mvars_created, 8);
        assert_eq!(s1.max_depth_reached, 100);
    }
    #[test]
    fn test_elab_stats_mvar_rate() {
        let s = ElabStats {
            num_mvars_created: 10,
            num_mvars_solved: 8,
            ..Default::default()
        };
        let rate = s.mvar_solve_rate();
        assert!((rate - 0.8).abs() < 1e-10);
        assert!(!s.all_mvars_solved());
    }
    #[test]
    fn test_elab_error_codes_display() {
        assert_eq!(format!("{}", ElabErrorCode::TypeMismatch), "type mismatch");
        assert_eq!(format!("{}", ElabErrorCode::UnknownName), "unknown name");
        assert_eq!(format!("{}", ElabErrorCode::TacticFailed), "tactic failed");
    }
    #[test]
    fn test_elab_stage_order() {
        let stages = ElabStage::all_in_order();
        assert_eq!(stages.len(), 9);
        assert_eq!(stages[0], ElabStage::NameResolution);
        assert_eq!(stages[8], ElabStage::KernelValidation);
    }
    #[test]
    fn test_elab_stage_names() {
        assert_eq!(ElabStage::Unification.name(), "unification");
        assert_eq!(ElabStage::KernelValidation.name(), "kernel_validation");
    }
    #[test]
    fn test_attr_names() {
        assert_eq!(attr_names::SIMP, "simp");
        assert_eq!(attr_names::INSTANCE, "instance");
        assert_eq!(attr_names::DERIVE, "derive");
    }
    #[test]
    fn test_elab_error_other() {
        assert_eq!(format!("{}", ElabErrorCode::Other), "elaboration error");
    }
    #[test]
    fn test_all_error_variants_display() {
        let variants = [
            ElabErrorCode::UnknownName,
            ElabErrorCode::TypeMismatch,
            ElabErrorCode::UnsolvedMvar,
            ElabErrorCode::AmbiguousInstance,
            ElabErrorCode::NoInstance,
            ElabErrorCode::UnificationFailed,
            ElabErrorCode::IllTyped,
            ElabErrorCode::TacticFailed,
            ElabErrorCode::NonExhaustiveMatch,
            ElabErrorCode::SyntaxError,
            ElabErrorCode::KernelRejected,
            ElabErrorCode::SorryNotAllowed,
            ElabErrorCode::RecursionLimit,
            ElabErrorCode::MutualCycle,
            ElabErrorCode::Other,
        ];
        for v in &variants {
            assert!(!format!("{}", v).is_empty());
        }
    }
}
#[cfg(test)]
mod pipeline_tests {
    use super::*;
    #[test]
    fn test_pipeline_registry_empty() {
        let reg = ElabPipelineRegistry::new();
        assert_eq!(reg.num_pre_passes(), 0);
        assert_eq!(reg.num_post_passes(), 0);
        assert!(reg.all_passes().is_empty());
    }
    #[test]
    fn test_pipeline_registry_add_passes() {
        let mut reg = ElabPipelineRegistry::new();
        reg.add_pre_pass("normalize");
        reg.add_post_pass("kernel_check");
        reg.add_tactic_pass("simp_prep");
        assert_eq!(reg.num_pre_passes(), 1);
        assert_eq!(reg.num_post_passes(), 1);
        assert_eq!(reg.num_tactic_passes(), 1);
        assert_eq!(reg.all_passes().len(), 3);
    }
}
/// Names of all well-known tactics supported by the elaborator.
#[allow(dead_code)]
pub mod tactic_names {
    /// Introduce a binder into the context.
    pub const INTRO: &str = "intro";
    /// Introduce multiple binders at once.
    pub const INTROS: &str = "intros";
    /// Apply a lemma to the goal.
    pub const APPLY: &str = "apply";
    /// Provide an exact proof term.
    pub const EXACT: &str = "exact";
    /// Close goal by reflexivity.
    pub const REFL: &str = "refl";
    /// Assumption — close by hypothesis.
    pub const ASSUMPTION: &str = "assumption";
    /// Trivially close a trivial goal.
    pub const TRIVIAL: &str = "trivial";
    /// Placeholder proof.
    pub const SORRY: &str = "sorry";
    /// Rewrite goal using equality.
    pub const RW: &str = "rw";
    /// Simplify using simp lemmas.
    pub const SIMP: &str = "simp";
    /// Simp using all hypotheses.
    pub const SIMP_ALL: &str = "simp_all";
    /// Case split.
    pub const CASES: &str = "cases";
    /// Induction.
    pub const INDUCTION: &str = "induction";
    /// Apply first constructor.
    pub const CONSTRUCTOR: &str = "constructor";
    /// Apply left constructor of Or.
    pub const LEFT: &str = "left";
    /// Apply right constructor of Or.
    pub const RIGHT: &str = "right";
    /// Provide existential witness.
    pub const EXISTSI: &str = "existsi";
    /// Use witness (alias for existsi).
    pub const USE: &str = "use";
    /// Push negation inward.
    pub const PUSH_NEG: &str = "push_neg";
    /// By contradiction.
    pub const BY_CONTRA: &str = "by_contra";
    /// Contrapositive.
    pub const CONTRAPOSE: &str = "contrapose";
    /// Split an iff/and goal.
    pub const SPLIT: &str = "split";
    /// Exfalso: change goal to False.
    pub const EXFALSO: &str = "exfalso";
    /// Linear arithmetic.
    pub const LINARITH: &str = "linarith";
    /// Ring simplification.
    pub const RING: &str = "ring";
    /// Norm_cast.
    pub const NORM_CAST: &str = "norm_cast";
    /// Clear a hypothesis.
    pub const CLEAR: &str = "clear";
    /// Have: introduce a new hypothesis with proof.
    pub const HAVE: &str = "have";
    /// Obtain: like cases but with pattern.
    pub const OBTAIN: &str = "obtain";
    /// Show: change the goal type.
    pub const SHOW: &str = "show";
    /// Revert: move hypotheses back to goal.
    pub const REVERT: &str = "revert";
    /// Specialize an applied hypothesis.
    pub const SPECIALIZE: &str = "specialize";
    /// Rename a hypothesis.
    pub const RENAME: &str = "rename";
}
/// Check whether a string is a known tactic name.
#[allow(dead_code)]
pub fn is_known_tactic(name: &str) -> bool {
    matches!(
        name,
        "intro"
            | "intros"
            | "apply"
            | "exact"
            | "refl"
            | "assumption"
            | "trivial"
            | "sorry"
            | "rw"
            | "simp"
            | "simp_all"
            | "cases"
            | "induction"
            | "constructor"
            | "left"
            | "right"
            | "existsi"
            | "use"
            | "push_neg"
            | "by_contra"
            | "by_contradiction"
            | "contrapose"
            | "split"
            | "exfalso"
            | "linarith"
            | "ring"
            | "norm_cast"
            | "clear"
            | "have"
            | "obtain"
            | "show"
            | "revert"
            | "specialize"
            | "rename"
            | "repeat"
            | "first"
            | "try"
            | "all_goals"
            | "any_goals"
            | "field_simp"
            | "push_cast"
            | "exact_mod_cast"
    )
}
/// Return the category of a tactic (proof-search, rewriting, etc.).
#[allow(dead_code)]
pub fn tactic_category(name: &str) -> &'static str {
    match name {
        "intro" | "intros" | "revert" | "clear" | "rename" | "obtain" | "have" | "show" => {
            "context"
        }
        "apply" | "exact" | "assumption" | "trivial" | "sorry" | "refl" => "proof-search",
        "rw" | "simp" | "simp_all" | "field_simp" | "ring" | "linarith" | "norm_cast"
        | "push_cast" | "exact_mod_cast" => "rewriting",
        "cases" | "induction" | "constructor" | "left" | "right" | "existsi" | "use" | "split"
        | "exfalso" => "structure",
        "push_neg" | "by_contra" | "by_contradiction" | "contrapose" => "logic",
        "repeat" | "first" | "try" | "all_goals" | "any_goals" => "combinator",
        "specialize" => "context",
        _ => "unknown",
    }
}
#[cfg(test)]
mod elab_lib_extra_tests {
    use super::*;
    #[test]
    fn test_elab_note_hint() {
        let n = ElabNote::Hint("use norm_num".to_string());
        assert_eq!(n.prefix(), "hint");
        assert!(!n.is_warning_like());
    }
    #[test]
    fn test_elab_note_warning() {
        let n = ElabNote::Warning("unsupported construct".to_string());
        assert!(n.is_warning_like());
    }
    #[test]
    fn test_elab_note_sorry() {
        let n = ElabNote::SorryUsed {
            declaration: "myTheorem".to_string(),
        };
        assert!(n.is_warning_like());
        assert_eq!(n.message(), "myTheorem");
    }
    #[test]
    fn test_elab_note_display() {
        let n = ElabNote::Info("no issues".to_string());
        let s = format!("{}", n);
        assert!(s.contains("info"));
    }
    #[test]
    fn test_elab_note_set_add_warning() {
        let mut ns = ElabNoteSet::new();
        ns.add_warning("potential issue");
        assert!(ns.has_warnings());
        assert_eq!(ns.len(), 1);
    }
    #[test]
    fn test_elab_note_set_merge() {
        let mut a = ElabNoteSet::new();
        a.add_hint("hint 1");
        let mut b = ElabNoteSet::new();
        b.add_info("info 1");
        a.merge(b);
        assert_eq!(a.len(), 2);
    }
    #[test]
    fn test_elab_note_set_clear() {
        let mut ns = ElabNoteSet::new();
        ns.add_sorry("myThm");
        ns.clear();
        assert!(ns.is_empty());
    }
    #[test]
    fn test_is_known_tactic() {
        assert!(is_known_tactic("intro"));
        assert!(is_known_tactic("simp"));
        assert!(is_known_tactic("ring"));
        assert!(!is_known_tactic("unknownTac"));
    }
    #[test]
    fn test_tactic_category() {
        assert_eq!(tactic_category("intro"), "context");
        assert_eq!(tactic_category("simp"), "rewriting");
        assert_eq!(tactic_category("cases"), "structure");
        assert_eq!(tactic_category("push_neg"), "logic");
        assert_eq!(tactic_category("repeat"), "combinator");
    }
    #[test]
    fn test_reducibility_ordering() {
        assert!(Reducibility::Reducible < Reducibility::SemiReducible);
        assert!(Reducibility::SemiReducible < Reducibility::Irreducible);
    }
    #[test]
    fn test_reducibility_attr_names() {
        assert_eq!(Reducibility::Reducible.attr_name(), "reducible");
        assert_eq!(Reducibility::Irreducible.attr_name(), "irreducible");
    }
    #[test]
    fn test_reducibility_default() {
        assert_eq!(Reducibility::default(), Reducibility::SemiReducible);
    }
    #[test]
    fn test_tactic_names_intro() {
        assert_eq!(tactic_names::INTRO, "intro");
        assert_eq!(tactic_names::SORRY, "sorry");
    }
    #[test]
    fn test_elab_note_warnings_filter() {
        let mut ns = ElabNoteSet::new();
        ns.add_hint("h1");
        ns.add_warning("w1");
        ns.add_sorry("decl");
        let warns = ns.warnings();
        assert_eq!(warns.len(), 2);
    }
}
/// A named elaboration pass that transforms an expression.
#[allow(dead_code)]
pub trait ElabPass {
    /// Name of this pass.
    fn name(&self) -> &str;
    /// Run the pass on an expression, returning the (possibly transformed) result.
    fn run(&self, expr: oxilean_kernel::Expr) -> Result<oxilean_kernel::Expr, String>;
    /// Whether this pass is enabled by default.
    fn enabled_by_default(&self) -> bool {
        true
    }
}
/// Format a kernel expression as a human-readable string.
#[allow(dead_code)]
pub fn pretty_expr(expr: &oxilean_kernel::Expr) -> String {
    match expr {
        Expr::Sort(l) => format!("Sort({:?})", l),
        Expr::BVar(i) => format!("#{}", i),
        Expr::FVar(fv) => format!("@{}", fv.0),
        Expr::Const(name, _) => name.to_string(),
        Expr::App(f, a) => format!("({} {})", pretty_expr(f), pretty_expr(a)),
        Expr::Lam(_, name, _ty, body) => {
            format!("(fun {} => {})", name, pretty_expr(body))
        }
        Expr::Pi(_, name, ty, body) => {
            format!(
                "(({} : {}) -> {})",
                name,
                pretty_expr(ty),
                pretty_expr(body)
            )
        }
        Expr::Let(name, _ty, val, body) => {
            format!(
                "(let {} := {} in {})",
                name,
                pretty_expr(val),
                pretty_expr(body)
            )
        }
        Expr::Lit(lit) => {
            use oxilean_kernel::Literal;
            match lit {
                Literal::Nat(n) => format!("{}", n),
                Literal::Str(s) => format!("{:?}", s),
            }
        }
        Expr::Proj(name, idx, inner) => {
            format!("{}.{} ({})", name, idx, pretty_expr(inner))
        }
    }
}
/// Format a list of expressions as a comma-separated string.
#[allow(dead_code)]
pub fn pretty_expr_list(exprs: &[oxilean_kernel::Expr]) -> String {
    exprs.iter().map(pretty_expr).collect::<Vec<_>>().join(", ")
}
/// Check if a declaration name looks like a recursive definition.
///
/// This is a heuristic check — actual recursion analysis happens in the kernel.
#[allow(dead_code)]
pub fn might_be_recursive(name: &oxilean_kernel::Name, body: &oxilean_kernel::Expr) -> bool {
    fn contains_name(expr: &Expr, target: &oxilean_kernel::Name) -> bool {
        match expr {
            Expr::Const(n, _) => n == target,
            Expr::App(f, a) => contains_name(f, target) || contains_name(a, target),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                contains_name(ty, target) || contains_name(body, target)
            }
            Expr::Let(_, ty, val, b) => {
                contains_name(ty, target) || contains_name(val, target) || contains_name(b, target)
            }
            Expr::Proj(_, _, inner) => contains_name(inner, target),
            _ => false,
        }
    }
    contains_name(body, name)
}
/// Extended tactic name constants.
#[allow(dead_code)]
pub mod tactic_names_ext {
    /// `norm_num` — numeric normalization.
    pub const NORM_NUM: &str = "norm_num";
    /// `omega` — linear arithmetic over integers.
    pub const OMEGA: &str = "omega";
    /// `decide` — decidable proposition checker.
    pub const DECIDE: &str = "decide";
    /// `native_decide` — faster decide using native code.
    pub const NATIVE_DECIDE: &str = "native_decide";
    /// `aesop` — automated proof search.
    pub const AESOP: &str = "aesop";
    /// `tauto` — propositional tautology prover.
    pub const TAUTO: &str = "tauto";
    /// `fin_cases` — case split on finite types.
    pub const FIN_CASES: &str = "fin_cases";
    /// `interval_cases` — case split on integer intervals.
    pub const INTERVAL_CASES: &str = "interval_cases";
    /// `gcongr` — generalized congruence.
    pub const GCONGR: &str = "gcongr";
    /// `positivity` — prove positivity of expressions.
    pub const POSITIVITY: &str = "positivity";
    /// `polyrith` — polynomial arithmetic.
    pub const POLYRITH: &str = "polyrith";
    /// `linear_combination` — linear combination proof.
    pub const LINEAR_COMBINATION: &str = "linear_combination";
    /// `ext` — extensionality.
    pub const EXT: &str = "ext";
    /// `funext` — function extensionality.
    pub const FUNEXT: &str = "funext";
    /// `congr` — congruence.
    pub const CONGR: &str = "congr";
    /// `unfold` — unfold a definition.
    pub const UNFOLD: &str = "unfold";
    /// `change` — change goal to definitionally equal form.
    pub const CHANGE: &str = "change";
    /// `subst` — substitute a hypothesis.
    pub const SUBST: &str = "subst";
    /// `symm` — symmetry of equality.
    pub const SYMM: &str = "symm";
    /// `trans` — transitivity.
    pub const TRANS: &str = "trans";
    /// `calc` — calculation proof.
    pub const CALC: &str = "calc";
    /// `rcases` — recursive case split.
    pub const RCASES: &str = "rcases";
    /// `rintro` — recursive intro.
    pub const RINTRO: &str = "rintro";
    /// `refine` — partial proof.
    pub const REFINE: &str = "refine";
    /// `ac_rfl` — AC-refl.
    pub const AC_RFL: &str = "ac_rfl";
}
/// Check if a tactic name is a Mathlib-style extended tactic.
#[allow(dead_code)]
pub fn is_mathlib_tactic(name: &str) -> bool {
    matches!(
        name,
        "norm_num"
            | "omega"
            | "decide"
            | "native_decide"
            | "aesop"
            | "tauto"
            | "fin_cases"
            | "interval_cases"
            | "gcongr"
            | "positivity"
            | "polyrith"
            | "linear_combination"
            | "ext"
            | "funext"
            | "congr"
            | "unfold"
            | "change"
            | "subst"
            | "symm"
            | "trans"
            | "calc"
            | "rcases"
            | "rintro"
            | "refine"
            | "ac_rfl"
    )
}
#[cfg(test)]
mod lib_extended_tests {
    use super::*;
    use oxilean_kernel::Name;
    #[test]
    fn test_elab_config_defaults() {
        let cfg = ElabConfig::default();
        assert!(!cfg.allow_sorry);
        assert!(cfg.kernel_check);
        assert!(cfg.proof_irrelevance);
        assert!(cfg.auto_implicit);
    }
    #[test]
    fn test_elab_config_strict() {
        let cfg = ElabConfig::strict();
        assert!(!cfg.allow_sorry);
        assert!(cfg.strict_instances);
        assert!(cfg.kernel_check);
    }
    #[test]
    fn test_elab_config_interactive() {
        let cfg = ElabConfig::interactive();
        assert!(cfg.allow_sorry);
        assert!(!cfg.strict_instances);
    }
    #[test]
    fn test_elab_config_batch() {
        let cfg = ElabConfig::batch();
        assert!(!cfg.allow_sorry);
        assert!(cfg.strict_instances);
        assert!(!cfg.trace_elaboration);
    }
    #[test]
    fn test_elab_metrics_solve_rate() {
        let mut m = ElabMetrics::new();
        m.metavars_created = 10;
        m.metavars_solved = 8;
        let rate = m.solve_rate();
        assert!((rate - 0.8).abs() < 1e-10);
    }
    #[test]
    fn test_elab_metrics_solve_rate_zero() {
        let m = ElabMetrics::new();
        assert_eq!(m.solve_rate(), 1.0);
    }
    #[test]
    fn test_elab_metrics_merge() {
        let mut a = ElabMetrics::new();
        a.declarations_elaborated = 5;
        let mut b = ElabMetrics::new();
        b.declarations_elaborated = 3;
        a.merge(&b);
        assert_eq!(a.declarations_elaborated, 8);
    }
    #[test]
    fn test_decl_kind_keyword() {
        assert_eq!(DeclKind::Def.keyword(), "def");
        assert_eq!(DeclKind::Theorem.keyword(), "theorem");
        assert_eq!(DeclKind::Axiom.keyword(), "axiom");
    }
    #[test]
    fn test_decl_kind_produces_term() {
        assert!(DeclKind::Def.produces_term());
        assert!(DeclKind::Theorem.produces_term());
        assert!(!DeclKind::Inductive.produces_term());
        assert!(!DeclKind::Namespace.produces_term());
    }
    #[test]
    fn test_decl_kind_requires_proof() {
        assert!(DeclKind::Theorem.requires_proof());
        assert!(!DeclKind::Def.requires_proof());
    }
    #[test]
    fn test_decl_kind_is_computable() {
        assert!(DeclKind::Def.is_computable());
        assert!(!DeclKind::Noncomputable.is_computable());
        assert!(!DeclKind::Axiom.is_computable());
    }
    #[test]
    fn test_proof_history_undo_redo() {
        let mut h = ProofHistory::new();
        assert!(h.is_empty());
        h.push(ProofStateSnapshot::new(0, "start", 2, vec![]));
        h.push(ProofStateSnapshot::new(1, "step1", 1, vec![]));
        h.push(ProofStateSnapshot::new(2, "step2", 0, vec![]));
        assert_eq!(h.len(), 3);
        let prev = h.undo();
        assert!(prev.is_some());
        assert_eq!(prev.expect("test operation should succeed").id, 1);
        let next = h.redo();
        assert!(next.is_some());
        assert_eq!(next.expect("test operation should succeed").id, 2);
    }
    #[test]
    fn test_proof_history_current() {
        let mut h = ProofHistory::new();
        h.push(ProofStateSnapshot::new(0, "start", 1, vec![]));
        assert!(h.current().is_some());
        assert_eq!(h.current().expect("test operation should succeed").id, 0);
        assert!(!h
            .current()
            .expect("test operation should succeed")
            .is_complete());
    }
    #[test]
    fn test_coercion_registry_find() {
        let mut reg = CoercionRegistryExt::new();
        let c = CoercionExt::new(Name::str("Nat"), Name::str("Int"), Name::str("Int.ofNat"));
        reg.register(c);
        assert!(reg.find(&Name::str("Nat"), &Name::str("Int")).is_some());
        assert!(reg.find(&Name::str("Int"), &Name::str("Nat")).is_none());
    }
    #[test]
    fn test_coercion_apply() {
        let c = CoercionExt::new(Name::str("Nat"), Name::str("Int"), Name::str("Int.ofNat"));
        let nat_expr = Expr::Const(Name::str("zero"), vec![]);
        let coerced = c.apply(nat_expr);
        assert!(matches!(coerced, Expr::App(_, _)));
    }
    #[test]
    fn test_instance_registry() {
        let mut reg = InstanceRegistry::new();
        let inst = ClassInstance::new(Name::str("Add"), Name::str("instAddNat")).as_default();
        reg.register(inst);
        assert_eq!(reg.instances_of(&Name::str("Add")).len(), 1);
        assert!(reg.default_instance(&Name::str("Add")).is_some());
    }
    #[test]
    fn test_attribute_registry() {
        let mut reg = AttributeRegistry::new();
        let attr = DeclAttribute::new("simp", Name::str("myLemma")).with_arg("all");
        reg.register(attr);
        assert_eq!(reg.attrs_of(&Name::str("myLemma")).len(), 1);
        assert_eq!(reg.decls_with("simp").len(), 1);
    }
    #[test]
    fn test_namespace_manager() {
        let mut nm = NamespaceManager::new();
        assert_eq!(nm.depth(), 0);
        nm.open(Name::str("Nat"));
        assert_eq!(nm.depth(), 1);
        let q = nm.qualify("succ");
        assert!(q.to_string().contains("succ"));
        nm.close();
        assert_eq!(nm.depth(), 0);
    }
    #[test]
    fn test_pretty_expr() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let s = pretty_expr(&nat);
        assert_eq!(s, "Nat");
        let bvar = Expr::BVar(2);
        let s2 = pretty_expr(&bvar);
        assert!(s2.contains('2'));
    }
    #[test]
    fn test_pretty_expr_list() {
        let exprs = vec![
            Expr::Const(Name::str("a"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
        ];
        let s = pretty_expr_list(&exprs);
        assert!(s.contains("a"));
        assert!(s.contains("b"));
        assert!(s.contains(','));
    }
    #[test]
    fn test_might_be_recursive_yes() {
        let name = Name::str("fib");
        let body = Expr::App(
            Box::new(Expr::Const(Name::str("fib"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert!(might_be_recursive(&name, &body));
    }
    #[test]
    fn test_might_be_recursive_no() {
        let name = Name::str("fib");
        let body = Expr::Const(Name::str("Nat.succ"), vec![]);
        assert!(!might_be_recursive(&name, &body));
    }
    #[test]
    fn test_is_mathlib_tactic() {
        assert!(is_mathlib_tactic("omega"));
        assert!(is_mathlib_tactic("norm_num"));
        assert!(is_mathlib_tactic("aesop"));
        assert!(!is_mathlib_tactic("intro"));
        assert!(!is_mathlib_tactic("unknown"));
    }
    #[test]
    fn test_tactic_names_ext_constants() {
        assert_eq!(tactic_names_ext::OMEGA, "omega");
        assert_eq!(tactic_names_ext::NORM_NUM, "norm_num");
        assert_eq!(tactic_names_ext::EXT, "ext");
    }
    #[test]
    fn test_env_snapshot_manager() {
        let mut mgr = EnvSnapshotManager::new();
        assert!(mgr.is_empty());
        let id1 = mgr.take(10, "after module A");
        let _id2 = mgr.take(20, "after module B");
        assert_eq!(mgr.len(), 2);
        let snap = mgr.get(id1).expect("key should exist");
        assert_eq!(snap.decl_count, 10);
        let latest = mgr.latest().expect("test operation should succeed");
        assert_eq!(latest.decl_count, 20);
    }
    #[test]
    fn test_universe_check_mode_equality() {
        assert_eq!(UniverseCheckMode::Full, UniverseCheckMode::Full);
        assert_ne!(UniverseCheckMode::Full, UniverseCheckMode::Skip);
    }
    #[test]
    fn test_coercion_registry_remove_from() {
        let mut reg = CoercionRegistryExt::new();
        reg.register(CoercionExt::new(
            Name::str("Nat"),
            Name::str("Int"),
            Name::str("f"),
        ));
        reg.register(CoercionExt::new(
            Name::str("Nat"),
            Name::str("Real"),
            Name::str("g"),
        ));
        reg.register(CoercionExt::new(
            Name::str("Int"),
            Name::str("Real"),
            Name::str("h"),
        ));
        assert_eq!(reg.len(), 3);
        reg.remove_from(&Name::str("Nat"));
        assert_eq!(reg.len(), 1);
    }
    #[test]
    fn test_instance_registry_remove_class() {
        let mut reg = InstanceRegistry::new();
        reg.register(ClassInstance::new(Name::str("Add"), Name::str("addNat")));
        reg.register(ClassInstance::new(Name::str("Add"), Name::str("addInt")));
        reg.register(ClassInstance::new(Name::str("Mul"), Name::str("mulNat")));
        assert_eq!(reg.len(), 3);
        reg.remove_class(&Name::str("Add"));
        assert_eq!(reg.len(), 1);
    }
    #[test]
    fn test_decl_kind_display() {
        let s = format!("{}", DeclKind::Def);
        assert_eq!(s, "def");
    }
}
