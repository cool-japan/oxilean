//! Functions for the enhanced pretty-printer module.
//!
//! Top-level convenience functions that wrap `PrettyPrinter` for common use
//! cases, plus full test coverage.

use crate::{ConstantInfo, Expr, Level, Name};

use super::types::{IndentMode, PrettyConfig, PrettyPrinter};

// ── Convenience functions ─────────────────────────────────────────────────────

/// Pretty-print an expression with default configuration (unicode, width 100).
pub fn pp_expr(e: &Expr) -> String {
    PrettyPrinter::new().pp_expr(e)
}

/// Pretty-print an expression without unicode symbols.
pub fn pp_expr_ascii(e: &Expr) -> String {
    PrettyPrinter::new().with_unicode(false).pp_expr(e)
}

/// Pretty-print a type expression (same as `pp_expr` but named for clarity).
pub fn pp_type(t: &Expr) -> String {
    PrettyPrinter::new().pp_type(t)
}

/// Pretty-print a declaration with its full signature.
pub fn pp_decl(d: &ConstantInfo) -> String {
    PrettyPrinter::new().pp_decl(d)
}

/// Pretty-print a universe level.
pub fn pp_level(l: &Level) -> String {
    PrettyPrinter::new().pp_level(l)
}

/// Pretty-print with a specific configuration.
pub fn pp_with_config(e: &Expr, config: PrettyConfig) -> String {
    PrettyPrinter::with_config(config).pp_expr(e)
}

/// Pretty-print showing all universe annotations.
pub fn pp_with_universes(e: &Expr) -> String {
    PrettyPrinter::new().with_universes(true).pp_expr(e)
}

/// Pretty-print at a specific line width.
pub fn pp_at_width(e: &Expr, width: usize) -> String {
    PrettyPrinter::new().with_width(width).pp_expr(e)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::types::PrettyConfig;
    use crate::{
        AxiomVal, BinderInfo, ConstantVal, DefinitionSafety, DefinitionVal, Level, Literal, Name,
        ReducibilityHint,
    };

    fn lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }

    fn const_expr(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }

    fn prop() -> Expr {
        Expr::Sort(Level::zero())
    }

    fn type0() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }

    fn type1() -> Expr {
        Expr::Sort(Level::succ(Level::succ(Level::zero())))
    }

    // ── Sort / universe ─────────────────────────────────────────────────────

    #[test]
    fn test_pp_prop() {
        assert_eq!(pp_expr(&prop()), "Prop");
    }

    #[test]
    fn test_pp_type() {
        assert_eq!(pp_expr(&type0()), "Type");
    }

    #[test]
    fn test_pp_type1() {
        assert_eq!(pp_expr(&type1()), "Type 1");
    }

    #[test]
    fn test_pp_sort_param() {
        let u = Level::param(Name::str("u"));
        let expr = Expr::Sort(u);
        assert_eq!(pp_expr(&expr), "Sort u");
    }

    // ── Literals ────────────────────────────────────────────────────────────

    #[test]
    fn test_pp_nat_lit() {
        assert_eq!(pp_expr(&lit(0)), "0");
        assert_eq!(pp_expr(&lit(42)), "42");
        assert_eq!(pp_expr(&lit(9999)), "9999");
    }

    #[test]
    fn test_pp_str_lit() {
        let s = Expr::Lit(Literal::Str("hello".to_string()));
        let out = pp_expr(&s);
        assert!(out.contains("hello"), "output was: {}", out);
    }

    // ── Constants ───────────────────────────────────────────────────────────

    #[test]
    fn test_pp_const_no_levels() {
        assert_eq!(pp_expr(&const_expr("Nat")), "Nat");
        assert_eq!(pp_expr(&const_expr("Bool")), "Bool");
    }

    #[test]
    fn test_pp_const_with_levels_hidden() {
        let e = Expr::Const(Name::str("List"), vec![Level::zero()]);
        // Default: universes hidden
        assert_eq!(pp_expr(&e), "List");
    }

    #[test]
    fn test_pp_const_with_levels_shown() {
        let e = Expr::Const(Name::str("List"), vec![Level::zero()]);
        let out = pp_with_universes(&e);
        assert!(out.contains("List"), "output was: {}", out);
        assert!(out.contains("0"), "output was: {}", out);
    }

    // ── Applications ────────────────────────────────────────────────────────

    #[test]
    fn test_pp_app_simple() {
        let app = Expr::App(Box::new(const_expr("f")), Box::new(lit(1)));
        assert_eq!(pp_expr(&app), "f 1");
    }

    #[test]
    fn test_pp_app_nested() {
        // f 1 2 = App(App(f, 1), 2)
        let inner = Expr::App(Box::new(const_expr("f")), Box::new(lit(1)));
        let app = Expr::App(Box::new(inner), Box::new(lit(2)));
        assert_eq!(pp_expr(&app), "f 1 2");
    }

    #[test]
    fn test_pp_app_of_app() {
        // (f g) h — f applied to g, then to h
        let fg = Expr::App(Box::new(const_expr("f")), Box::new(const_expr("g")));
        let fgh = Expr::App(Box::new(fg), Box::new(const_expr("h")));
        assert_eq!(pp_expr(&fgh), "f g h");
    }

    // ── Lambda ──────────────────────────────────────────────────────────────

    #[test]
    fn test_pp_lambda_unicode() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(const_expr("Nat")),
            Box::new(Expr::BVar(0)),
        );
        let out = pp_expr(&lam);
        assert!(out.starts_with("λ"), "output was: {}", out);
        assert!(out.contains("Nat"), "output was: {}", out);
    }

    #[test]
    fn test_pp_lambda_ascii() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(const_expr("Nat")),
            Box::new(Expr::BVar(0)),
        );
        let out = pp_expr_ascii(&lam);
        assert!(out.starts_with("fun"), "output was: {}", out);
    }

    #[test]
    fn test_pp_lambda_nested_collapses() {
        // λ x y, body — should NOT be printed as λ x, λ y, body
        let body = Expr::BVar(0);
        let inner_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(const_expr("Nat")),
            Box::new(body),
        );
        let outer_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(const_expr("Nat")),
            Box::new(inner_lam),
        );
        let out = pp_expr(&outer_lam);
        // Should have both binders in one lambda
        assert!(out.contains("x"), "output was: {}", out);
        assert!(out.contains("y"), "output was: {}", out);
        // Should only have one λ symbol
        let lambda_count = out.matches('λ').count();
        assert_eq!(lambda_count, 1, "Expected 1 λ but got: {}", out);
    }

    // ── Pi / arrow ──────────────────────────────────────────────────────────

    #[test]
    fn test_pp_pi_nondep_arrow_unicode() {
        // Π _ : Nat, Nat  (non-dependent, _ is anonymous)
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(const_expr("Nat")),
            Box::new(const_expr("Nat")),
        );
        let out = pp_expr(&pi);
        assert!(out.contains("→"), "output was: {}", out);
        assert!(out.contains("Nat"), "output was: {}", out);
    }

    #[test]
    fn test_pp_pi_nondep_arrow_ascii() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(const_expr("Nat")),
            Box::new(const_expr("Nat")),
        );
        let out = pp_expr_ascii(&pi);
        assert!(out.contains("->"), "output was: {}", out);
    }

    #[test]
    fn test_pp_pi_dep_forall_unicode() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(const_expr("Nat")),
            Box::new(const_expr("Bool")),
        );
        let out = pp_expr(&pi);
        assert!(out.starts_with("∀"), "output was: {}", out);
        assert!(out.contains("n"), "output was: {}", out);
        assert!(out.contains("Nat"), "output was: {}", out);
        assert!(out.contains("Bool"), "output was: {}", out);
    }

    #[test]
    fn test_pp_pi_dep_forall_ascii() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(const_expr("Nat")),
            Box::new(const_expr("Bool")),
        );
        let out = pp_expr_ascii(&pi);
        assert!(out.starts_with("forall"), "output was: {}", out);
    }

    // ── Let ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_pp_let() {
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(const_expr("Nat")),
            Box::new(lit(42)),
            Box::new(Expr::BVar(0)),
        );
        let out = pp_expr(&let_expr);
        assert!(out.contains("let"), "output was: {}", out);
        assert!(out.contains("x"), "output was: {}", out);
        assert!(out.contains("Nat"), "output was: {}", out);
        assert!(out.contains("42"), "output was: {}", out);
    }

    // ── Projection ──────────────────────────────────────────────────────────

    #[test]
    fn test_pp_proj() {
        let proj = Expr::Proj(Name::str("fst"), 0, Box::new(const_expr("pair")));
        let out = pp_expr(&proj);
        assert!(out.contains("pair"), "output was: {}", out);
        assert!(out.contains("fst"), "output was: {}", out);
    }

    // ── BVar / FVar ─────────────────────────────────────────────────────────

    #[test]
    fn test_pp_bvar_default_hidden() {
        let e = Expr::BVar(3);
        let pp = PrettyPrinter::new();
        assert_eq!(pp.pp_expr(&e), "_");
    }

    #[test]
    fn test_pp_bvar_show_indices() {
        let e = Expr::BVar(3);
        let pp = PrettyPrinter::with_config(PrettyConfig {
            show_bvar_indices: true,
            ..Default::default()
        });
        assert_eq!(pp.pp_expr(&e), "#3");
    }

    // ── Level printing ──────────────────────────────────────────────────────

    #[test]
    fn test_pp_level_zero() {
        assert_eq!(pp_level(&Level::zero()), "0");
    }

    #[test]
    fn test_pp_level_succ() {
        let l = Level::succ(Level::zero());
        assert_eq!(pp_level(&l), "1");
        let l2 = Level::succ(Level::succ(Level::zero()));
        assert_eq!(pp_level(&l2), "2");
    }

    #[test]
    fn test_pp_level_max() {
        let u = Level::param(Name::str("u"));
        let v = Level::param(Name::str("v"));
        let m = Level::max(u, v);
        let out = pp_level(&m);
        assert!(out.contains("max"), "output was: {}", out);
        assert!(out.contains("u"), "output was: {}", out);
        assert!(out.contains("v"), "output was: {}", out);
    }

    #[test]
    fn test_pp_level_param() {
        let p = Level::param(Name::str("w"));
        assert_eq!(pp_level(&p), "w");
    }

    // ── Declaration printing ────────────────────────────────────────────────

    fn make_constant_val(name: &str, ty: Expr) -> ConstantVal {
        ConstantVal {
            name: Name::str(name),
            level_params: vec![],
            ty,
        }
    }

    #[test]
    fn test_pp_decl_axiom() {
        let ax = ConstantInfo::Axiom(AxiomVal {
            common: make_constant_val("myAxiom", const_expr("Prop")),
            is_unsafe: false,
        });
        let out = pp_decl(&ax);
        assert!(out.contains("axiom"), "output was: {}", out);
        assert!(out.contains("myAxiom"), "output was: {}", out);
        assert!(out.contains("Prop"), "output was: {}", out);
    }

    #[test]
    fn test_pp_decl_definition() {
        let def = ConstantInfo::Definition(DefinitionVal {
            common: make_constant_val("answer", const_expr("Nat")),
            value: lit(42),
            hints: ReducibilityHint::Regular(0),
            safety: DefinitionSafety::Safe,
            all: vec![],
        });
        let out = pp_decl(&def);
        assert!(out.contains("def"), "output was: {}", out);
        assert!(out.contains("answer"), "output was: {}", out);
        assert!(out.contains("Nat"), "output was: {}", out);
        assert!(out.contains("42"), "output was: {}", out);
    }

    #[test]
    fn test_pp_decl_theorem_no_body() {
        use crate::TheoremVal;
        let thm = ConstantInfo::Theorem(TheoremVal {
            common: make_constant_val("myThm", prop()),
            value: lit(0),
            all: vec![],
        });
        let out = pp_decl(&thm);
        assert!(out.contains("theorem"), "output was: {}", out);
        assert!(out.contains("myThm"), "output was: {}", out);
        // Default: proof body hidden
        assert!(
            !out.contains(":= 0"),
            "should hide body, output was: {}",
            out
        );
    }

    #[test]
    fn test_pp_decl_theorem_with_body() {
        use crate::TheoremVal;
        let thm = ConstantInfo::Theorem(TheoremVal {
            common: make_constant_val("myThm", prop()),
            value: lit(0),
            all: vec![],
        });
        let pp = PrettyPrinter::with_config(PrettyConfig {
            show_proof_bodies: true,
            ..Default::default()
        });
        let out = pp.pp_decl(&thm);
        assert!(out.contains(":="), "output was: {}", out);
        assert!(out.contains("0"), "output was: {}", out);
    }

    // ── Width / IndentMode ──────────────────────────────────────────────────

    #[test]
    fn test_with_width() {
        let e = lit(1);
        let out = pp_at_width(&e, 20);
        assert_eq!(out, "1");
    }

    #[test]
    fn test_indent_mode_spaces() {
        let mode = IndentMode::Spaces(4);
        assert_eq!(mode.render(2), "        ");
    }

    #[test]
    fn test_indent_mode_tabs() {
        let mode = IndentMode::Tabs;
        assert_eq!(mode.render(3), "\t\t\t");
    }

    #[test]
    fn test_indent_mode_spaces_zero() {
        let mode = IndentMode::Spaces(2);
        assert_eq!(mode.render(0), "");
    }

    // ── Config presets ──────────────────────────────────────────────────────

    #[test]
    fn test_config_verbose() {
        let cfg = PrettyConfig::verbose();
        assert!(cfg.show_universes);
        assert!(cfg.show_bvar_indices);
        assert!(cfg.show_proof_bodies);
    }

    #[test]
    fn test_config_ascii() {
        let cfg = PrettyConfig::ascii();
        assert!(!cfg.unicode);
    }

    #[test]
    fn test_config_compact() {
        let cfg = PrettyConfig::compact();
        assert_eq!(cfg.width, 0);
    }

    // ── Max depth ───────────────────────────────────────────────────────────

    #[test]
    fn test_max_depth_truncates() {
        // Build a deeply nested let expression (each level increments depth)
        let mut e: Expr = lit(0);
        for i in 0..10u64 {
            e = Expr::Let(
                Name::str(&format!("x{}", i)),
                Box::new(const_expr("Nat")),
                Box::new(lit(i)),
                Box::new(e),
            );
        }
        let pp = PrettyPrinter::with_config(PrettyConfig {
            max_depth: 3,
            ..Default::default()
        });
        let out = pp.pp_expr(&e);
        // At depth > 3, the printer should emit the ellipsis
        assert!(
            out.contains("…") || out.contains("..."),
            "output was: {}",
            out
        );
    }

    // ── pp_with_config ──────────────────────────────────────────────────────

    #[test]
    fn test_pp_with_config_ascii_no_universes() {
        let e = Expr::Const(Name::str("Nat"), vec![Level::zero()]);
        let cfg = PrettyConfig::ascii();
        let out = pp_with_config(&e, cfg);
        assert_eq!(out, "Nat");
    }

    // ── Roundtrip: pp_type same as pp_expr ─────────────────────────────────

    #[test]
    fn test_pp_type_same_as_pp_expr() {
        let t = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(const_expr("Nat")),
            Box::new(const_expr("Bool")),
        );
        assert_eq!(pp_type(&t), pp_expr(&t));
    }

    // ── Level params in decl ────────────────────────────────────────────────

    #[test]
    fn test_decl_with_level_params() {
        let ax = ConstantInfo::Axiom(AxiomVal {
            common: ConstantVal {
                name: Name::str("univAx"),
                level_params: vec![Name::str("u"), Name::str("v")],
                ty: prop(),
            },
            is_unsafe: false,
        });
        let out = pp_decl(&ax);
        assert!(out.contains("u"), "output was: {}", out);
        assert!(out.contains("v"), "output was: {}", out);
    }
}
