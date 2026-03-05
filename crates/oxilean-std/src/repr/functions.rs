//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

#[allow(dead_code)]
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
#[allow(dead_code)]
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
#[allow(dead_code)]
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
#[allow(dead_code)]
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
}
#[allow(dead_code)]
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
#[allow(dead_code)]
pub fn lam(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(bi, Name::str(name), Box::new(dom), Box::new(body))
}
#[allow(dead_code)]
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
#[allow(dead_code)]
pub fn cst_u(s: &str, levels: Vec<Level>) -> Expr {
    Expr::Const(Name::str(s), levels)
}
#[allow(dead_code)]
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
#[allow(dead_code)]
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
#[allow(dead_code)]
pub fn type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
#[allow(dead_code)]
pub fn sort_u() -> Expr {
    Expr::Sort(Level::Param(Name::str("u")))
}
#[allow(dead_code)]
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
#[allow(dead_code)]
pub fn nat_ty() -> Expr {
    cst("Nat")
}
#[allow(dead_code)]
pub fn string_ty() -> Expr {
    cst("String")
}
#[allow(dead_code)]
pub fn bool_ty() -> Expr {
    cst("Bool")
}
#[allow(dead_code)]
pub fn format_ty() -> Expr {
    cst("Format")
}
#[allow(dead_code)]
pub fn list_of(ty: Expr) -> Expr {
    app(cst("List"), ty)
}
#[allow(dead_code)]
pub fn option_of(ty: Expr) -> Expr {
    app(cst("Option"), ty)
}
#[allow(dead_code)]
pub fn add_axiom(
    env: &mut Environment,
    name: &str,
    univ_params: Vec<Name>,
    ty: Expr,
) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params,
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Precedence constant: maximum precedence (1024).
#[allow(dead_code)]
pub const MAX_PREC: u64 = 1024;
/// Precedence constant: argument precedence (1025).
#[allow(dead_code)]
pub const ARG_PREC: u64 = 1025;
/// Build the Repr / Format / ToString environment.
#[allow(clippy::too_many_lines)]
pub fn build_repr_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(env, "Format", vec![], type1())?;
    add_axiom(env, "Format.nil", vec![], format_ty())?;
    let format_text_ty = pi(BinderInfo::Default, "s", string_ty(), format_ty());
    add_axiom(env, "Format.text", vec![], format_text_ty)?;
    let format_append_ty = pi(
        BinderInfo::Default,
        "a",
        format_ty(),
        pi(BinderInfo::Default, "b", format_ty(), format_ty()),
    );
    add_axiom(env, "Format.append", vec![], format_append_ty)?;
    let format_nest_ty = pi(
        BinderInfo::Default,
        "n",
        nat_ty(),
        pi(BinderInfo::Default, "doc", format_ty(), format_ty()),
    );
    add_axiom(env, "Format.nest", vec![], format_nest_ty)?;
    add_axiom(env, "Format.line", vec![], format_ty())?;
    let format_group_ty = pi(BinderInfo::Default, "doc", format_ty(), format_ty());
    add_axiom(env, "Format.group", vec![], format_group_ty)?;
    let format_fill_ty = pi(BinderInfo::Default, "doc", format_ty(), format_ty());
    add_axiom(env, "Format.fill", vec![], format_fill_ty)?;
    let format_bracket_ty = pi(
        BinderInfo::Default,
        "open",
        string_ty(),
        pi(
            BinderInfo::Default,
            "doc",
            format_ty(),
            pi(BinderInfo::Default, "close", string_ty(), format_ty()),
        ),
    );
    add_axiom(env, "Format.bracket", vec![], format_bracket_ty)?;
    let format_paren_ty = pi(BinderInfo::Default, "doc", format_ty(), format_ty());
    add_axiom(env, "Format.paren", vec![], format_paren_ty)?;
    let format_sbracket_ty = pi(BinderInfo::Default, "doc", format_ty(), format_ty());
    add_axiom(env, "Format.sbracket", vec![], format_sbracket_ty)?;
    let format_joinsep_ty = pi(
        BinderInfo::Default,
        "fmts",
        list_of(format_ty()),
        pi(BinderInfo::Default, "sep", format_ty(), format_ty()),
    );
    add_axiom(env, "Format.joinSep", vec![], format_joinsep_ty)?;
    let format_indent_ty = pi(BinderInfo::Default, "doc", format_ty(), format_ty());
    add_axiom(env, "Format.indentD", vec![], format_indent_ty)?;
    let format_prefixjoin_ty = pi(
        BinderInfo::Default,
        "prefix",
        format_ty(),
        pi(
            BinderInfo::Default,
            "fmts",
            list_of(format_ty()),
            format_ty(),
        ),
    );
    add_axiom(env, "Format.prefixJoin", vec![], format_prefixjoin_ty)?;
    let format_interpolate_ty = pi(
        BinderInfo::Default,
        "parts",
        list_of(format_ty()),
        format_ty(),
    );
    add_axiom(env, "Format.interpolate", vec![], format_interpolate_ty)?;
    let repr_ty = pi(BinderInfo::Default, "α", type1(), type2());
    add_axiom(env, "Repr", vec![], repr_ty)?;
    let repr_reprprec_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Repr"), bvar(0)),
            pi(
                BinderInfo::Default,
                "a",
                bvar(1),
                pi(BinderInfo::Default, "prec", nat_ty(), format_ty()),
            ),
        ),
    );
    add_axiom(env, "Repr.reprPrec", vec![], repr_reprprec_ty)?;
    let tostring_ty = pi(BinderInfo::Default, "α", type1(), type2());
    add_axiom(env, "ToString", vec![], tostring_ty)?;
    let tostring_tostring_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("ToString"), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), string_ty()),
        ),
    );
    add_axiom(env, "ToString.toString", vec![], tostring_tostring_ty)?;
    add_axiom(env, "instReprNat", vec![], app(cst("Repr"), nat_ty()))?;
    add_axiom(env, "instReprBool", vec![], app(cst("Repr"), bool_ty()))?;
    add_axiom(env, "instReprString", vec![], app(cst("Repr"), string_ty()))?;
    add_axiom(env, "instReprInt", vec![], app(cst("Repr"), cst("Int")))?;
    let inst_repr_list_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Repr"), bvar(0)),
            app(cst("Repr"), list_of(bvar(1))),
        ),
    );
    add_axiom(env, "instReprList", vec![], inst_repr_list_ty)?;
    let inst_repr_option_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Repr"), bvar(0)),
            app(cst("Repr"), option_of(bvar(1))),
        ),
    );
    add_axiom(env, "instReprOption", vec![], inst_repr_option_ty)?;
    let inst_repr_prod_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::Implicit,
            "β",
            type1(),
            pi(
                BinderInfo::InstImplicit,
                "_inst1",
                app(cst("Repr"), bvar(1)),
                pi(
                    BinderInfo::InstImplicit,
                    "_inst2",
                    app(cst("Repr"), bvar(1)),
                    app(cst("Repr"), app2(cst("Prod"), bvar(3), bvar(2))),
                ),
            ),
        ),
    );
    add_axiom(env, "instReprProd", vec![], inst_repr_prod_ty)?;
    add_axiom(
        env,
        "instToStringNat",
        vec![],
        app(cst("ToString"), nat_ty()),
    )?;
    add_axiom(
        env,
        "instToStringBool",
        vec![],
        app(cst("ToString"), bool_ty()),
    )?;
    add_axiom(
        env,
        "instToStringString",
        vec![],
        app(cst("ToString"), string_ty()),
    )?;
    add_axiom(
        env,
        "instToStringInt",
        vec![],
        app(cst("ToString"), cst("Int")),
    )?;
    add_axiom(env, "maxPrec", vec![], nat_ty())?;
    add_axiom(env, "argPrec", vec![], nat_ty())?;
    let repr_arg_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Repr"), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), format_ty()),
        ),
    );
    add_axiom(env, "reprArg", vec![], repr_arg_ty)?;
    let repr_str_ty = pi(BinderInfo::Default, "s", string_ty(), format_ty());
    add_axiom(env, "reprStr", vec![], repr_str_ty)?;
    let repr_prec0_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Repr"), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), format_ty()),
        ),
    );
    add_axiom(env, "reprPrec0", vec![], repr_prec0_ty)?;
    let pretty_ty = pi(BinderInfo::Default, "α", type1(), type2());
    add_axiom(env, "Pretty", vec![], pretty_ty)?;
    let pretty_pretty_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Pretty"), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), format_ty()),
        ),
    );
    add_axiom(env, "Pretty.pretty", vec![], pretty_pretty_ty)?;
    let has_format_ty = pi(BinderInfo::Default, "α", type1(), type2());
    add_axiom(env, "HasFormat", vec![], has_format_ty)?;
    let has_format_format_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("HasFormat"), bvar(0)),
            pi(BinderInfo::Default, "a", bvar(1), format_ty()),
        ),
    );
    add_axiom(env, "HasFormat.format", vec![], has_format_format_ty)?;
    let format_flatten_ty = pi(BinderInfo::Default, "doc", format_ty(), format_ty());
    add_axiom(env, "Format.flatten", vec![], format_flatten_ty)?;
    let format_isempty_ty = pi(BinderInfo::Default, "doc", format_ty(), bool_ty());
    add_axiom(env, "Format.isEmpty", vec![], format_isempty_ty)?;
    let inst_tostring_of_repr_ty = pi(
        BinderInfo::Implicit,
        "α",
        type1(),
        pi(
            BinderInfo::InstImplicit,
            "_inst",
            app(cst("Repr"), bvar(0)),
            app(cst("ToString"), bvar(1)),
        ),
    );
    add_axiom(env, "instToStringOfRepr", vec![], inst_tostring_of_repr_ty)?;
    let format_pretty_ty = pi(BinderInfo::Default, "doc", format_ty(), string_ty());
    add_axiom(env, "Format.pretty", vec![], format_pretty_ty)?;
    let format_prettywidth_ty = pi(
        BinderInfo::Default,
        "width",
        nat_ty(),
        pi(BinderInfo::Default, "doc", format_ty(), string_ty()),
    );
    add_axiom(env, "Format.prettyWidth", vec![], format_prettywidth_ty)?;
    let add_app_paren_ty = pi(
        BinderInfo::Default,
        "doc",
        format_ty(),
        pi(BinderInfo::Default, "prec", nat_ty(), format_ty()),
    );
    add_axiom(env, "Repr.addAppParen", vec![], add_app_paren_ty)?;
    add_axiom(env, "instReprFormat", vec![], app(cst("Repr"), format_ty()))?;
    add_axiom(
        env,
        "instToStringFormat",
        vec![],
        app(cst("ToString"), format_ty()),
    )?;
    add_axiom(
        env,
        "instPrettyFormat",
        vec![],
        app(cst("Pretty"), format_ty()),
    )?;
    Ok(())
}
/// Build `Format` type expression.
#[allow(dead_code)]
pub fn mk_format() -> Expr {
    format_ty()
}
/// Build `Format.nil`.
#[allow(dead_code)]
pub fn mk_format_nil() -> Expr {
    cst("Format.nil")
}
/// Build `Format.text s`.
#[allow(dead_code)]
pub fn mk_format_text(s: Expr) -> Expr {
    app(cst("Format.text"), s)
}
/// Build `Format.append a b`.
#[allow(dead_code)]
pub fn mk_format_append(a: Expr, b: Expr) -> Expr {
    app2(cst("Format.append"), a, b)
}
/// Build `Format.nest n doc`.
#[allow(dead_code)]
pub fn mk_format_nest(n: Expr, doc: Expr) -> Expr {
    app2(cst("Format.nest"), n, doc)
}
/// Build `Format.line`.
#[allow(dead_code)]
pub fn mk_format_line() -> Expr {
    cst("Format.line")
}
/// Build `Format.group doc`.
#[allow(dead_code)]
pub fn mk_format_group(doc: Expr) -> Expr {
    app(cst("Format.group"), doc)
}
/// Build `Format.fill doc`.
#[allow(dead_code)]
pub fn mk_format_fill(doc: Expr) -> Expr {
    app(cst("Format.fill"), doc)
}
/// Build `Format.bracket open doc close`.
#[allow(dead_code)]
pub fn mk_format_bracket(open: Expr, doc: Expr, close: Expr) -> Expr {
    app3(cst("Format.bracket"), open, doc, close)
}
/// Build `Format.paren doc`.
#[allow(dead_code)]
pub fn mk_format_paren(doc: Expr) -> Expr {
    app(cst("Format.paren"), doc)
}
/// Build `Format.sbracket doc`.
#[allow(dead_code)]
pub fn mk_format_sbracket(doc: Expr) -> Expr {
    app(cst("Format.sbracket"), doc)
}
/// Build `Format.joinSep fmts sep`.
#[allow(dead_code)]
pub fn mk_format_joinsep(fmts: Expr, sep: Expr) -> Expr {
    app2(cst("Format.joinSep"), fmts, sep)
}
/// Build `Format.indentD doc`.
#[allow(dead_code)]
pub fn mk_format_indent(doc: Expr) -> Expr {
    app(cst("Format.indentD"), doc)
}
/// Build `Format.prefixJoin prefix fmts`.
#[allow(dead_code)]
pub fn mk_format_prefixjoin(prefix: Expr, fmts: Expr) -> Expr {
    app2(cst("Format.prefixJoin"), prefix, fmts)
}
/// Build `Format.interpolate parts`.
#[allow(dead_code)]
pub fn mk_format_interpolate(parts: Expr) -> Expr {
    app(cst("Format.interpolate"), parts)
}
/// Build `Format.flatten doc`.
#[allow(dead_code)]
pub fn mk_format_flatten(doc: Expr) -> Expr {
    app(cst("Format.flatten"), doc)
}
/// Build `Repr α` type expression.
#[allow(dead_code)]
pub fn mk_repr(ty: Expr) -> Expr {
    app(cst("Repr"), ty)
}
/// Build `ToString α` type expression.
#[allow(dead_code)]
pub fn mk_to_string(ty: Expr) -> Expr {
    app(cst("ToString"), ty)
}
/// Build `Pretty α` type expression.
#[allow(dead_code)]
pub fn mk_pretty(ty: Expr) -> Expr {
    app(cst("Pretty"), ty)
}
/// Build `Repr.reprPrec val prec`.
#[allow(dead_code)]
pub fn mk_repr_prec(val: Expr, prec: Expr) -> Expr {
    app2(cst("Repr.reprPrec"), val, prec)
}
/// Build `ToString.toString val`.
#[allow(dead_code)]
pub fn mk_tostring_call(val: Expr) -> Expr {
    app(cst("ToString.toString"), val)
}
/// Build `reprArg val`.
#[allow(dead_code)]
pub fn mk_repr_arg(val: Expr) -> Expr {
    app(cst("reprArg"), val)
}
/// Build `reprStr s`.
#[allow(dead_code)]
pub fn mk_repr_str(s: Expr) -> Expr {
    app(cst("reprStr"), s)
}
/// Build `reprPrec0 val`.
#[allow(dead_code)]
pub fn mk_repr_prec0(val: Expr) -> Expr {
    app(cst("reprPrec0"), val)
}
/// Build `Pretty.pretty val`.
#[allow(dead_code)]
pub fn mk_pretty_call(val: Expr) -> Expr {
    app(cst("Pretty.pretty"), val)
}
/// Build `Format.pretty doc`.
#[allow(dead_code)]
pub fn mk_format_pretty(doc: Expr) -> Expr {
    app(cst("Format.pretty"), doc)
}
/// Build `Format.prettyWidth width doc`.
#[allow(dead_code)]
pub fn mk_format_prettywidth(width: Expr, doc: Expr) -> Expr {
    app2(cst("Format.prettyWidth"), width, doc)
}
/// Build `Repr.addAppParen doc prec`.
#[allow(dead_code)]
pub fn mk_add_app_paren(doc: Expr, prec: Expr) -> Expr {
    app2(cst("Repr.addAppParen"), doc, prec)
}
/// Build `Format.isEmpty doc`.
#[allow(dead_code)]
pub fn mk_format_isempty(doc: Expr) -> Expr {
    app(cst("Format.isEmpty"), doc)
}
/// Build `HasFormat.format val`.
#[allow(dead_code)]
pub fn mk_has_format_call(val: Expr) -> Expr {
    app(cst("HasFormat.format"), val)
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::Literal;
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        for name in &["String", "Nat", "Bool", "Int", "List", "Option", "Prod"] {
            let _ = env.add(Declaration::Axiom {
                name: Name::str(*name),
                univ_params: vec![],
                ty: type1(),
            });
        }
        env
    }
    #[test]
    fn test_build_repr_env() {
        let mut env = setup_env();
        assert!(build_repr_env(&mut env).is_ok());
        let expected = [
            "Format",
            "Format.nil",
            "Format.text",
            "Format.append",
            "Format.nest",
            "Format.line",
            "Format.group",
            "Format.fill",
            "Format.bracket",
            "Format.paren",
            "Format.sbracket",
            "Format.joinSep",
            "Format.indentD",
            "Format.prefixJoin",
            "Format.interpolate",
            "Repr",
            "Repr.reprPrec",
            "ToString",
            "ToString.toString",
            "instReprNat",
            "instReprBool",
            "instReprString",
            "instReprInt",
            "instReprList",
            "instReprOption",
            "instReprProd",
            "instToStringNat",
            "instToStringBool",
            "instToStringString",
            "instToStringInt",
            "maxPrec",
            "argPrec",
            "reprArg",
            "reprStr",
            "reprPrec0",
            "Pretty",
            "Pretty.pretty",
            "HasFormat",
            "HasFormat.format",
            "Format.flatten",
            "Format.isEmpty",
            "instToStringOfRepr",
            "Format.pretty",
            "Format.prettyWidth",
            "Repr.addAppParen",
            "instReprFormat",
            "instToStringFormat",
            "instPrettyFormat",
        ];
        for name in &expected {
            assert!(env.contains(&Name::str(*name)), "missing: {}", name);
        }
    }
    #[test]
    fn test_mk_format() {
        let e = mk_format();
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("Format"));
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_mk_format_nil() {
        let e = mk_format_nil();
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("Format.nil"));
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_mk_format_text() {
        let s = Expr::Lit(Literal::Str("hello".into()));
        let e = mk_format_text(s);
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.text"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_append() {
        let a = mk_format_nil();
        let b = mk_format_line();
        let e = mk_format_append(a, b);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Format.append"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_nest() {
        let n = Expr::Lit(Literal::Nat(2));
        let doc = mk_format_nil();
        let e = mk_format_nest(n, doc);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Format.nest"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_line() {
        let e = mk_format_line();
        if let Expr::Const(n, _) = &e {
            assert_eq!(*n, Name::str("Format.line"));
        } else {
            panic!("expected Const");
        }
    }
    #[test]
    fn test_mk_format_group() {
        let e = mk_format_group(mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.group"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_fill() {
        let e = mk_format_fill(mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.fill"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_bracket() {
        let open = Expr::Lit(Literal::Str("(".into()));
        let close = Expr::Lit(Literal::Str(")".into()));
        let e = mk_format_bracket(open, mk_format_nil(), close);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_format_paren() {
        let e = mk_format_paren(mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.paren"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_sbracket() {
        let e = mk_format_sbracket(mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.sbracket"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_joinsep() {
        let e = mk_format_joinsep(cst("fmts"), mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Format.joinSep"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_indent() {
        let e = mk_format_indent(mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.indentD"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_prefixjoin() {
        let e = mk_format_prefixjoin(mk_format_nil(), cst("fmts"));
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Format.prefixJoin"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_interpolate() {
        let e = mk_format_interpolate(cst("parts"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.interpolate"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_repr() {
        let e = mk_repr(nat_ty());
        if let Expr::App(f, a) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Repr"));
            }
            if let Expr::Const(n, _) = a.as_ref() {
                assert_eq!(*n, Name::str("Nat"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_to_string() {
        let e = mk_to_string(bool_ty());
        if let Expr::App(f, a) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("ToString"));
            }
            if let Expr::Const(n, _) = a.as_ref() {
                assert_eq!(*n, Name::str("Bool"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_repr_prec() {
        let val = cst("x");
        let prec = Expr::Lit(Literal::Nat(0));
        let e = mk_repr_prec(val, prec);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Repr.reprPrec"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_tostring_call() {
        let e = mk_tostring_call(cst("v"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("ToString.toString"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_repr_arg() {
        let e = mk_repr_arg(cst("a"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("reprArg"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_repr_str() {
        let s = Expr::Lit(Literal::Str("hello".into()));
        let e = mk_repr_str(s);
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("reprStr"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_repr_prec0() {
        let e = mk_repr_prec0(cst("x"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("reprPrec0"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_pretty() {
        let e = mk_pretty(nat_ty());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Pretty"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_pretty_call() {
        let e = mk_pretty_call(cst("x"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Pretty.pretty"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_has_format_call() {
        let e = mk_has_format_call(cst("x"));
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("HasFormat.format"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_flatten() {
        let e = mk_format_flatten(mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.flatten"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_isempty() {
        let e = mk_format_isempty(mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.isEmpty"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_pretty() {
        let e = mk_format_pretty(mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::Const(n, _) = f.as_ref() {
                assert_eq!(*n, Name::str("Format.pretty"));
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_format_prettywidth() {
        let w = Expr::Lit(Literal::Nat(80));
        let e = mk_format_prettywidth(w, mk_format_nil());
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Format.prettyWidth"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_mk_add_app_paren() {
        let doc = mk_format_nil();
        let prec = Expr::Lit(Literal::Nat(MAX_PREC));
        let e = mk_add_app_paren(doc, prec);
        if let Expr::App(f, _) = &e {
            if let Expr::App(g, _) = f.as_ref() {
                if let Expr::Const(n, _) = g.as_ref() {
                    assert_eq!(*n, Name::str("Repr.addAppParen"));
                }
            }
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_max_prec() {
        assert_eq!(MAX_PREC, 1024);
    }
    #[test]
    fn test_arg_prec() {
        assert_eq!(ARG_PREC, 1025);
    }
    #[test]
    fn test_format_decls_present() {
        let mut env = setup_env();
        build_repr_env(&mut env).expect("build_repr_env should succeed");
        assert!(env.contains(&Name::str("Format")));
        assert!(env.contains(&Name::str("Format.nil")));
        assert!(env.contains(&Name::str("Format.text")));
        assert!(env.contains(&Name::str("Format.append")));
        assert!(env.contains(&Name::str("Format.nest")));
        assert!(env.contains(&Name::str("Format.line")));
        assert!(env.contains(&Name::str("Format.group")));
        assert!(env.contains(&Name::str("Format.fill")));
    }
    #[test]
    fn test_repr_decls_present() {
        let mut env = setup_env();
        build_repr_env(&mut env).expect("build_repr_env should succeed");
        assert!(env.contains(&Name::str("Repr")));
        assert!(env.contains(&Name::str("Repr.reprPrec")));
        assert!(env.contains(&Name::str("instReprNat")));
        assert!(env.contains(&Name::str("instReprBool")));
        assert!(env.contains(&Name::str("instReprString")));
        assert!(env.contains(&Name::str("instReprInt")));
        assert!(env.contains(&Name::str("instReprList")));
        assert!(env.contains(&Name::str("instReprOption")));
        assert!(env.contains(&Name::str("instReprProd")));
    }
    #[test]
    fn test_tostring_decls_present() {
        let mut env = setup_env();
        build_repr_env(&mut env).expect("build_repr_env should succeed");
        assert!(env.contains(&Name::str("ToString")));
        assert!(env.contains(&Name::str("ToString.toString")));
        assert!(env.contains(&Name::str("instToStringNat")));
        assert!(env.contains(&Name::str("instToStringBool")));
        assert!(env.contains(&Name::str("instToStringString")));
        assert!(env.contains(&Name::str("instToStringInt")));
    }
    #[test]
    fn test_format_is_axiom() {
        let mut env = setup_env();
        build_repr_env(&mut env).expect("build_repr_env should succeed");
        let decl = env
            .get(&Name::str("Format"))
            .expect("declaration 'Format' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_repr_is_axiom() {
        let mut env = setup_env();
        build_repr_env(&mut env).expect("build_repr_env should succeed");
        let decl = env
            .get(&Name::str("Repr"))
            .expect("declaration 'Repr' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
}
#[allow(dead_code)]
pub fn rpr_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
#[allow(dead_code)]
pub fn rpr_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    rpr_ext_app(rpr_ext_app(f, a), b)
}
#[allow(dead_code)]
pub fn rpr_ext_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    rpr_ext_app(rpr_ext_app2(f, a, b), c)
}
#[allow(dead_code)]
pub fn rpr_ext_cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
#[allow(dead_code)]
pub fn rpr_ext_nat() -> Expr {
    rpr_ext_cst("Nat")
}
#[allow(dead_code)]
pub fn rpr_ext_bool() -> Expr {
    rpr_ext_cst("Bool")
}
#[allow(dead_code)]
pub fn rpr_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
#[allow(dead_code)]
pub fn rpr_ext_type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
#[allow(dead_code)]
pub fn rpr_ext_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
#[allow(dead_code)]
pub fn rpr_ext_arrow(dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(dom),
        Box::new(body),
    )
}
#[allow(dead_code)]
pub fn rpr_ext_bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// `BitfieldType : Type` — the type of bit-field descriptors.
///
/// A bit-field descriptor specifies an offset and width within a word.
#[allow(dead_code)]
pub fn axiom_bitfield_type_ty() -> Expr {
    rpr_ext_type1()
}
/// `BitfieldExtract : Nat → Nat → Nat → Nat`
///
/// Extract `width` bits starting at `offset` from a `word`.
/// `BitfieldExtract word offset width`.
#[allow(dead_code)]
pub fn axiom_bitfield_extract_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_nat(),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat())),
    )
}
/// `BitfieldInsert : Nat → Nat → Nat → Nat → Nat`
///
/// Insert `value` of `width` bits at `offset` into `word`.
/// `BitfieldInsert word offset width value`.
#[allow(dead_code)]
pub fn axiom_bitfield_insert_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_nat(),
        rpr_ext_arrow(
            rpr_ext_nat(),
            rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat())),
        ),
    )
}
/// `BitfieldRoundtrip : ∀ (w off wid val : Nat), BitfieldExtract (BitfieldInsert w off wid val) off wid = val`
///
/// Correctness: extracting after inserting returns the inserted value (when in range).
#[allow(dead_code)]
pub fn axiom_bitfield_roundtrip_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_prop()))
}
/// `BitfieldMask : Nat → Nat → Nat`
///
/// The bitmask for a field: `(1 << width) - 1` shifted to `offset`.
#[allow(dead_code)]
pub fn axiom_bitfield_mask_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat()))
}
/// `AlignmentOf : Type → Nat`
///
/// The alignment requirement (in bytes) of a type.
#[allow(dead_code)]
pub fn axiom_alignment_of_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_type1(), rpr_ext_nat())
}
/// `IsAligned : Nat → Nat → Prop`
///
/// `IsAligned addr alignment` — the address is aligned to the given boundary.
/// Equivalent to `addr % alignment = 0`.
#[allow(dead_code)]
pub fn axiom_is_aligned_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_prop()))
}
/// `AlignUp : Nat → Nat → Nat`
///
/// Round `addr` up to the nearest multiple of `alignment`.
#[allow(dead_code)]
pub fn axiom_align_up_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat()))
}
/// `AlignDown : Nat → Nat → Nat`
///
/// Round `addr` down to the nearest multiple of `alignment`.
#[allow(dead_code)]
pub fn axiom_align_down_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat()))
}
/// `AlignUpIdempotent : ∀ (addr align : Nat), AlignUp (AlignUp addr align) align = AlignUp addr align`
///
/// Rounding up is idempotent.
#[allow(dead_code)]
pub fn axiom_align_up_idempotent_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_prop()))
}
/// `CacheLineAlignment : Nat`
///
/// Typical cache-line size in bytes (64 on x86-64).
#[allow(dead_code)]
pub fn axiom_cache_line_alignment_ty() -> Expr {
    rpr_ext_nat()
}
/// `IsCacheLineAligned : Nat → Prop`
///
/// Check whether an address is cache-line aligned.
#[allow(dead_code)]
pub fn axiom_is_cache_line_aligned_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_prop())
}
/// `StructLayout : Type` — descriptor of a C-compatible struct layout.
///
/// Records field offsets, sizes, and total padded size.
#[allow(dead_code)]
pub fn axiom_struct_layout_ty() -> Expr {
    rpr_ext_type1()
}
/// `FieldOffset : StructLayout → Nat → Nat`
///
/// The byte offset of the n-th field within the struct.
#[allow(dead_code)]
pub fn axiom_field_offset_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("StructLayout"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat()),
    )
}
/// `StructSize : StructLayout → Nat`
///
/// Total size of the struct including trailing padding.
#[allow(dead_code)]
pub fn axiom_struct_size_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("StructLayout"), rpr_ext_nat())
}
/// `StructAligned : StructLayout → Nat → Prop`
///
/// The struct satisfies the given alignment requirement.
#[allow(dead_code)]
pub fn axiom_struct_aligned_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("StructLayout"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_prop()),
    )
}
/// `CABICompat : StructLayout → Prop`
///
/// The layout is compatible with the C ABI on the target platform.
#[allow(dead_code)]
pub fn axiom_c_abi_compat_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("StructLayout"), rpr_ext_prop())
}
/// `UnionType : Type` — the type of discriminated union type descriptors.
#[allow(dead_code)]
pub fn axiom_union_type_ty() -> Expr {
    rpr_ext_type1()
}
/// `UnionVariant : UnionType → Nat → Type`
///
/// The n-th variant type of a union.
#[allow(dead_code)]
pub fn axiom_union_variant_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("UnionType"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_type1()),
    )
}
/// `UnionSize : UnionType → Nat`
///
/// Total size of the union (maximum of all variant sizes).
#[allow(dead_code)]
pub fn axiom_union_size_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("UnionType"), rpr_ext_nat())
}
/// `UnionSafeAccess : UnionType → Nat → Prop`
///
/// Accessing the n-th variant is safe (discriminant matches).
#[allow(dead_code)]
pub fn axiom_union_safe_access_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("UnionType"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_prop()),
    )
}
/// `TwosComplement : Nat → Int → Nat`
///
/// Two's complement representation of a signed integer in `n` bits.
#[allow(dead_code)]
pub fn axiom_twos_complement_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_nat(),
        rpr_ext_arrow(rpr_ext_cst("Int"), rpr_ext_nat()),
    )
}
/// `TwosComplementInverse : Nat → Nat → Int`
///
/// Interpret an `n`-bit unsigned word as a two's complement signed integer.
#[allow(dead_code)]
pub fn axiom_twos_complement_inv_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_nat(),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_cst("Int")),
    )
}
/// `TwosComplementRoundtrip : ∀ (n : Nat) (x : Int), TwosComplementInverse n (TwosComplement n x) = x`
///
/// Round-trip property (for values in range).
#[allow(dead_code)]
pub fn axiom_twos_complement_roundtrip_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_nat(),
        rpr_ext_arrow(rpr_ext_cst("Int"), rpr_ext_prop()),
    )
}
/// `WrappingAdd : Nat → Nat → Nat → Nat`
///
/// Wrapping addition modulo `2^n`.
#[allow(dead_code)]
pub fn axiom_wrapping_add_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_nat(),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat())),
    )
}
/// `WrappingMul : Nat → Nat → Nat → Nat`
///
/// Wrapping multiplication modulo `2^n`.
#[allow(dead_code)]
pub fn axiom_wrapping_mul_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_nat(),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat())),
    )
}
/// `FloatType : Type` — the type of IEEE 754 floating-point descriptors.
#[allow(dead_code)]
pub fn axiom_float_type_ty() -> Expr {
    rpr_ext_type1()
}
/// `IsNaN : FloatType → Prop`
///
/// Predicate: the float is a Not-a-Number value.
#[allow(dead_code)]
pub fn axiom_is_nan_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("FloatType"), rpr_ext_prop())
}
/// `IsInfinite : FloatType → Prop`
///
/// Predicate: the float is positive or negative infinity.
#[allow(dead_code)]
pub fn axiom_is_infinite_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("FloatType"), rpr_ext_prop())
}
/// `IsSubnormal : FloatType → Prop`
///
/// Predicate: the float is a subnormal (denormal) value.
#[allow(dead_code)]
pub fn axiom_is_subnormal_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("FloatType"), rpr_ext_prop())
}
/// `IsFinite : FloatType → Prop`
///
/// Predicate: the float is finite (not NaN and not infinite).
#[allow(dead_code)]
pub fn axiom_is_finite_float_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("FloatType"), rpr_ext_prop())
}
/// `PositiveInfinity : FloatType`
///
/// The IEEE 754 positive infinity value.
#[allow(dead_code)]
pub fn axiom_positive_infinity_ty() -> Expr {
    rpr_ext_cst("FloatType")
}
/// `NegativeInfinity : FloatType`
///
/// The IEEE 754 negative infinity value.
#[allow(dead_code)]
pub fn axiom_negative_infinity_ty() -> Expr {
    rpr_ext_cst("FloatType")
}
/// `CanonicalNaN : FloatType`
///
/// The canonical quiet NaN value.
#[allow(dead_code)]
pub fn axiom_canonical_nan_ty() -> Expr {
    rpr_ext_cst("FloatType")
}
/// `NaNNotEqual : ∀ (x : FloatType), IsNaN x → x ≠ x`
///
/// IEEE 754: NaN is not equal to itself.
#[allow(dead_code)]
pub fn axiom_nan_not_equal_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("FloatType"),
        rpr_ext_arrow(rpr_ext_prop(), rpr_ext_prop()),
    )
}
/// `RoundingMode : Type` — the type of IEEE 754 rounding mode descriptors.
#[allow(dead_code)]
pub fn axiom_rounding_mode_ty() -> Expr {
    rpr_ext_type1()
}
/// `RoundToNearest : RoundingMode` — round to nearest, ties to even.
#[allow(dead_code)]
pub fn axiom_round_to_nearest_ty() -> Expr {
    rpr_ext_cst("RoundingMode")
}
/// `RoundTowardZero : RoundingMode` — truncation mode.
#[allow(dead_code)]
pub fn axiom_round_toward_zero_ty() -> Expr {
    rpr_ext_cst("RoundingMode")
}
/// `RoundTowardPositive : RoundingMode` — ceiling mode.
#[allow(dead_code)]
pub fn axiom_round_toward_positive_ty() -> Expr {
    rpr_ext_cst("RoundingMode")
}
/// `RoundTowardNegative : RoundingMode` — floor mode.
#[allow(dead_code)]
pub fn axiom_round_toward_negative_ty() -> Expr {
    rpr_ext_cst("RoundingMode")
}
/// `RoundedAdd : RoundingMode → FloatType → FloatType → FloatType`
///
/// IEEE 754 addition under a given rounding mode.
#[allow(dead_code)]
pub fn axiom_rounded_add_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("RoundingMode"),
        rpr_ext_arrow(
            rpr_ext_cst("FloatType"),
            rpr_ext_arrow(rpr_ext_cst("FloatType"), rpr_ext_cst("FloatType")),
        ),
    )
}
/// `SimdVectorType : Type` — the type of SIMD vector descriptors.
#[allow(dead_code)]
pub fn axiom_simd_vector_type_ty() -> Expr {
    rpr_ext_type1()
}
/// `SimdLaneCount : SimdVectorType → Nat`
///
/// Number of lanes in the SIMD vector.
#[allow(dead_code)]
pub fn axiom_simd_lane_count_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("SimdVectorType"), rpr_ext_nat())
}
/// `SimdLaneWidth : SimdVectorType → Nat`
///
/// Width of each lane in bytes.
#[allow(dead_code)]
pub fn axiom_simd_lane_width_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("SimdVectorType"), rpr_ext_nat())
}
/// `SimdVectorSize : SimdVectorType → Nat`
///
/// Total size of the vector: `LaneCount * LaneWidth`.
#[allow(dead_code)]
pub fn axiom_simd_vector_size_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("SimdVectorType"), rpr_ext_nat())
}
/// `SimdAligned : SimdVectorType → Nat → Prop`
///
/// The vector at a given address is properly aligned for SIMD operations.
#[allow(dead_code)]
pub fn axiom_simd_aligned_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("SimdVectorType"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_prop()),
    )
}
/// `Endianness : Type` — the type of byte-order descriptors.
#[allow(dead_code)]
pub fn axiom_endianness_ty() -> Expr {
    rpr_ext_type1()
}
/// `LittleEndian : Endianness` — least-significant byte first.
#[allow(dead_code)]
pub fn axiom_little_endian_ty() -> Expr {
    rpr_ext_cst("Endianness")
}
/// `BigEndian : Endianness` — most-significant byte first.
#[allow(dead_code)]
pub fn axiom_big_endian_ty() -> Expr {
    rpr_ext_cst("Endianness")
}
/// `ByteSwap : Nat → Nat → Nat`
///
/// Byte-swap a value of `n` bytes (convert between little- and big-endian).
#[allow(dead_code)]
pub fn axiom_byte_swap_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat()))
}
/// `ByteSwapInvolution : ∀ (n v : Nat), ByteSwap n (ByteSwap n v) = v`
///
/// Byte-swapping is its own inverse.
#[allow(dead_code)]
pub fn axiom_byte_swap_involution_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_prop()))
}
/// `TaggedPointer : Type` — a pointer with low-order bits repurposed for tags.
#[allow(dead_code)]
pub fn axiom_tagged_pointer_ty() -> Expr {
    rpr_ext_type1()
}
/// `TaggedPtrAddr : TaggedPointer → Nat`
///
/// Extract the aligned base address from a tagged pointer.
#[allow(dead_code)]
pub fn axiom_tagged_ptr_addr_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("TaggedPointer"), rpr_ext_nat())
}
/// `TaggedPtrTag : TaggedPointer → Nat`
///
/// Extract the tag bits from a tagged pointer.
#[allow(dead_code)]
pub fn axiom_tagged_ptr_tag_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("TaggedPointer"), rpr_ext_nat())
}
/// `FatPointer : Type` — a pointer paired with a length (e.g. Rust slice reference).
#[allow(dead_code)]
pub fn axiom_fat_pointer_ty() -> Expr {
    rpr_ext_type1()
}
/// `FatPtrAddr : FatPointer → Nat`
///
/// The data pointer component of a fat pointer.
#[allow(dead_code)]
pub fn axiom_fat_ptr_addr_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("FatPointer"), rpr_ext_nat())
}
/// `FatPtrLen : FatPointer → Nat`
///
/// The length component of a fat pointer.
#[allow(dead_code)]
pub fn axiom_fat_ptr_len_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("FatPointer"), rpr_ext_nat())
}
/// `VTablePtr : Type` — a virtual table (vtable) pointer.
#[allow(dead_code)]
pub fn axiom_vtable_ptr_ty() -> Expr {
    rpr_ext_type1()
}
/// `VTableEntry : VTablePtr → Nat → Nat`
///
/// The n-th function pointer slot in the vtable.
#[allow(dead_code)]
pub fn axiom_vtable_entry_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("VTablePtr"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat()),
    )
}
/// `VTableSize : VTablePtr → Nat`
///
/// Total number of method slots in the vtable.
#[allow(dead_code)]
pub fn axiom_vtable_size_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("VTablePtr"), rpr_ext_nat())
}
/// `DynObject : Type` — a trait object: (data ptr, vtable ptr) pair.
#[allow(dead_code)]
pub fn axiom_dyn_object_ty() -> Expr {
    rpr_ext_type1()
}
/// `StackFrame : Type` — descriptor for a call stack frame.
#[allow(dead_code)]
pub fn axiom_stack_frame_ty() -> Expr {
    rpr_ext_type1()
}
/// `FrameSize : StackFrame → Nat`
///
/// Total size of the stack frame in bytes.
#[allow(dead_code)]
pub fn axiom_frame_size_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("StackFrame"), rpr_ext_nat())
}
/// `ReturnAddress : StackFrame → Nat`
///
/// Offset of the saved return address within the frame.
#[allow(dead_code)]
pub fn axiom_return_address_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("StackFrame"), rpr_ext_nat())
}
/// `CallerSaved : Nat → Bool`
///
/// Is register n caller-saved in the SysV AMD64 ABI?
#[allow(dead_code)]
pub fn axiom_caller_saved_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_bool())
}
/// `CalleeSaved : Nat → Bool`
///
/// Is register n callee-saved in the SysV AMD64 ABI?
#[allow(dead_code)]
pub fn axiom_callee_saved_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_nat(), rpr_ext_bool())
}
/// `SysVABIRedZone : Nat`
///
/// Size of the SysV AMD64 red zone (128 bytes below the stack pointer).
#[allow(dead_code)]
pub fn axiom_sysv_red_zone_ty() -> Expr {
    rpr_ext_nat()
}
/// Register all extended repr/layout/ABI axioms into the given environment.
///
/// This extends `build_repr_env` with machine-level representation axioms
/// covering bit-fields, alignment, C struct layout, union safety,
/// two's complement, IEEE 754, SIMD, pointers, vtables, and calling conventions.
pub fn register_repr_extended(env: &mut Environment) -> Result<(), String> {
    let decls: &[(&str, Expr)] = &[
        ("BitfieldType", axiom_bitfield_type_ty()),
        ("BitfieldExtract", axiom_bitfield_extract_ty()),
        ("BitfieldInsert", axiom_bitfield_insert_ty()),
        ("BitfieldRoundtrip", axiom_bitfield_roundtrip_ty()),
        ("BitfieldMask", axiom_bitfield_mask_ty()),
        ("AlignmentOf", axiom_alignment_of_ty()),
        ("IsAligned", axiom_is_aligned_ty()),
        ("AlignUp", axiom_align_up_ty()),
        ("AlignDown", axiom_align_down_ty()),
        ("AlignUpIdempotent", axiom_align_up_idempotent_ty()),
        ("CacheLineAlignment", axiom_cache_line_alignment_ty()),
        ("IsCacheLineAligned", axiom_is_cache_line_aligned_ty()),
        ("StructLayout", axiom_struct_layout_ty()),
        ("FieldOffset", axiom_field_offset_ty()),
        ("StructSize", axiom_struct_size_ty()),
        ("StructAligned", axiom_struct_aligned_ty()),
        ("CABICompat", axiom_c_abi_compat_ty()),
        ("UnionType", axiom_union_type_ty()),
        ("UnionVariant", axiom_union_variant_ty()),
        ("UnionSize", axiom_union_size_ty()),
        ("UnionSafeAccess", axiom_union_safe_access_ty()),
        ("TwosComplement", axiom_twos_complement_ty()),
        ("TwosComplementInv", axiom_twos_complement_inv_ty()),
        ("TwosComplementRT", axiom_twos_complement_roundtrip_ty()),
        ("WrappingAdd", axiom_wrapping_add_ty()),
        ("WrappingMul", axiom_wrapping_mul_ty()),
        ("FloatType", axiom_float_type_ty()),
        ("IsNaN", axiom_is_nan_ty()),
        ("IsInfinite", axiom_is_infinite_ty()),
        ("IsSubnormal", axiom_is_subnormal_ty()),
        ("IsFiniteFloat", axiom_is_finite_float_ty()),
        ("PositiveInfinity", axiom_positive_infinity_ty()),
        ("NegativeInfinity", axiom_negative_infinity_ty()),
        ("CanonicalNaN", axiom_canonical_nan_ty()),
        ("NaNNotEqual", axiom_nan_not_equal_ty()),
        ("RoundingMode", axiom_rounding_mode_ty()),
        ("RoundToNearest", axiom_round_to_nearest_ty()),
        ("RoundTowardZero", axiom_round_toward_zero_ty()),
        ("RoundTowardPositive", axiom_round_toward_positive_ty()),
        ("RoundTowardNegative", axiom_round_toward_negative_ty()),
        ("RoundedAdd", axiom_rounded_add_ty()),
        ("SimdVectorType", axiom_simd_vector_type_ty()),
        ("SimdLaneCount", axiom_simd_lane_count_ty()),
        ("SimdLaneWidth", axiom_simd_lane_width_ty()),
        ("SimdVectorSize", axiom_simd_vector_size_ty()),
        ("SimdAligned", axiom_simd_aligned_ty()),
        ("Endianness", axiom_endianness_ty()),
        ("LittleEndian", axiom_little_endian_ty()),
        ("BigEndian", axiom_big_endian_ty()),
        ("ByteSwap", axiom_byte_swap_ty()),
        ("ByteSwapInvolution", axiom_byte_swap_involution_ty()),
        ("TaggedPointer", axiom_tagged_pointer_ty()),
        ("TaggedPtrAddr", axiom_tagged_ptr_addr_ty()),
        ("TaggedPtrTag", axiom_tagged_ptr_tag_ty()),
        ("FatPointer", axiom_fat_pointer_ty()),
        ("FatPtrAddr", axiom_fat_ptr_addr_ty()),
        ("FatPtrLen", axiom_fat_ptr_len_ty()),
        ("VTablePtr", axiom_vtable_ptr_ty()),
        ("VTableEntry", axiom_vtable_entry_ty()),
        ("VTableSize", axiom_vtable_size_ty()),
        ("DynObject", axiom_dyn_object_ty()),
        ("StackFrame", axiom_stack_frame_ty()),
        ("FrameSize", axiom_frame_size_ty()),
        ("ReturnAddress", axiom_return_address_ty()),
        ("CallerSaved", axiom_caller_saved_ty()),
        ("CalleeSaved", axiom_callee_saved_ty()),
        ("SysVABIRedZone", axiom_sysv_red_zone_ty()),
    ];
    for (name, ty) in decls {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
/// `StackCanary : Type` — a stack-smashing protection canary descriptor.
#[allow(dead_code)]
pub fn axiom_stack_canary_ty() -> Expr {
    rpr_ext_type1()
}
/// `CanaryValue : StackCanary → Nat`
///
/// The canary word value placed on the stack at function entry.
#[allow(dead_code)]
pub fn axiom_canary_value_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("StackCanary"), rpr_ext_nat())
}
/// `CanaryValid : StackCanary → Prop`
///
/// The canary is still intact (has not been overwritten).
#[allow(dead_code)]
pub fn axiom_canary_valid_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("StackCanary"), rpr_ext_prop())
}
/// `ShadowStack : Type` — a shadow stack pointer for return-address protection.
#[allow(dead_code)]
pub fn axiom_shadow_stack_ty() -> Expr {
    rpr_ext_type1()
}
/// `ShadowStackPush : ShadowStack → Nat → ShadowStack`
///
/// Push a return address onto the shadow stack.
#[allow(dead_code)]
pub fn axiom_shadow_stack_push_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("ShadowStack"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_cst("ShadowStack")),
    )
}
/// `ShadowStackPop : ShadowStack → ShadowStack × Nat`
///
/// Pop a return address from the shadow stack.
#[allow(dead_code)]
pub fn axiom_shadow_stack_pop_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("ShadowStack"),
        rpr_ext_app2(
            rpr_ext_cst("Prod"),
            rpr_ext_cst("ShadowStack"),
            rpr_ext_nat(),
        ),
    )
}
/// `DwarfEntry : Type` — a DWARF debug information entry (DIE).
#[allow(dead_code)]
pub fn axiom_dwarf_entry_ty() -> Expr {
    rpr_ext_type1()
}
/// `DwarfTag : DwarfEntry → Nat`
///
/// The DWARF tag of a debug entry (e.g. DW_TAG_function = 0x2e).
#[allow(dead_code)]
pub fn axiom_dwarf_tag_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("DwarfEntry"), rpr_ext_nat())
}
/// `UnwindTable : Type` — a `.eh_frame` / `.debug_frame` unwinding table.
#[allow(dead_code)]
pub fn axiom_unwind_table_ty() -> Expr {
    rpr_ext_type1()
}
/// `UnwindRule : UnwindTable → Nat → Nat`
///
/// The CFI rule for a given program counter and register number.
#[allow(dead_code)]
pub fn axiom_unwind_rule_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("UnwindTable"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat()),
    )
}
/// `RegisterFile : Type` — the register file of a CPU.
#[allow(dead_code)]
pub fn axiom_register_file_ty() -> Expr {
    rpr_ext_type1()
}
/// `RegValue : RegisterFile → Nat → Nat`
///
/// The value of the n-th register in the register file.
#[allow(dead_code)]
pub fn axiom_reg_value_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("RegisterFile"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_nat()),
    )
}
/// `RegWrite : RegisterFile → Nat → Nat → RegisterFile`
///
/// Write a value to a register, producing an updated register file.
#[allow(dead_code)]
pub fn axiom_reg_write_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("RegisterFile"),
        rpr_ext_arrow(
            rpr_ext_nat(),
            rpr_ext_arrow(rpr_ext_nat(), rpr_ext_cst("RegisterFile")),
        ),
    )
}
/// `RegReadAfterWrite : ∀ (rf : RegisterFile) (r v : Nat), RegValue (RegWrite rf r v) r = v`
///
/// Reading a register after writing it returns the written value.
#[allow(dead_code)]
pub fn axiom_reg_read_after_write_ty() -> Expr {
    rpr_ext_arrow(
        rpr_ext_cst("RegisterFile"),
        rpr_ext_arrow(rpr_ext_nat(), rpr_ext_arrow(rpr_ext_nat(), rpr_ext_prop())),
    )
}
/// `NumRegisters : RegisterFile → Nat`
///
/// The number of registers in the register file.
#[allow(dead_code)]
pub fn axiom_num_registers_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("RegisterFile"), rpr_ext_nat())
}
/// `MemoryModel : Type` — the memory consistency model of a hardware platform.
#[allow(dead_code)]
pub fn axiom_memory_model_ty() -> Expr {
    rpr_ext_type1()
}
/// `IsSequentiallyConsistent : MemoryModel → Prop`
///
/// The memory model is sequentially consistent (SC).
#[allow(dead_code)]
pub fn axiom_is_sc_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("MemoryModel"), rpr_ext_prop())
}
/// `IsTotalStoreOrder : MemoryModel → Prop`
///
/// The memory model is Total Store Order (TSO), as used by x86.
#[allow(dead_code)]
pub fn axiom_is_tso_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("MemoryModel"), rpr_ext_prop())
}
/// `IsRelaxed : MemoryModel → Prop`
///
/// The memory model has relaxed (weak) ordering, as on ARM or POWER.
#[allow(dead_code)]
pub fn axiom_is_relaxed_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("MemoryModel"), rpr_ext_prop())
}
/// `MemoryFence : MemoryModel → Prop`
///
/// A full memory barrier fence is available in this model.
#[allow(dead_code)]
pub fn axiom_memory_fence_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("MemoryModel"), rpr_ext_prop())
}
/// `ABI : Type` — a target ABI descriptor.
#[allow(dead_code)]
pub fn axiom_abi_ty() -> Expr {
    rpr_ext_type1()
}
/// `ABIIntSize : ABI → Nat`
///
/// Size of `int` in bytes for this ABI.
#[allow(dead_code)]
pub fn axiom_abi_int_size_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("ABI"), rpr_ext_nat())
}
/// `ABILongSize : ABI → Nat`
///
/// Size of `long` in bytes for this ABI.
#[allow(dead_code)]
pub fn axiom_abi_long_size_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("ABI"), rpr_ext_nat())
}
/// `ABIPtrSize : ABI → Nat`
///
/// Size of a pointer in bytes for this ABI.
#[allow(dead_code)]
pub fn axiom_abi_ptr_size_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("ABI"), rpr_ext_nat())
}
/// `ABIStackAlignment : ABI → Nat`
///
/// Required stack pointer alignment at function call boundaries.
#[allow(dead_code)]
pub fn axiom_abi_stack_alignment_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("ABI"), rpr_ext_nat())
}
/// `ABIArgRegCount : ABI → Nat`
///
/// Number of integer argument registers in this ABI (e.g. 6 for SysV AMD64).
#[allow(dead_code)]
pub fn axiom_abi_arg_reg_count_ty() -> Expr {
    rpr_ext_arrow(rpr_ext_cst("ABI"), rpr_ext_nat())
}
