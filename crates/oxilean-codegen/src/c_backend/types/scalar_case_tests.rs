//! `CBackend::emit_scalar_case` against a statement of the same case ladder.

use super::*;

fn reference_scalar_case(
    backend: &mut CBackend,
    scrut_name: &str,
    alts: &[LcnfAlt],
    default: &Option<Box<LcnfExpr>>,
    ret_ty: &LcnfType,
) -> Vec<CStmt> {
    let mut stmts = Vec::new();
    if alts.is_empty() {
        if let Some(def) = default {
            stmts.extend(backend.emit_expr(def, ret_ty));
        } else {
            stmts.push(CStmt::Expr(CExpr::call(
                "lean_internal_panic_unreachable",
                vec![],
            )));
        }
        return stmts;
    }
    let mut remaining = alts.to_vec();
    remaining.reverse();
    let first = remaining
        .pop()
        .expect("alts is non-empty after reverse; guaranteed by caller");
    let mut result = {
        let cond = CExpr::binop(
            CBinOp::Eq,
            CExpr::var(scrut_name),
            CExpr::UIntLit(first.ctor_tag as u64),
        );
        let then_body = backend.emit_expr(&first.body, ret_ty);
        let else_body = if remaining.is_empty() {
            if let Some(def) = default {
                backend.emit_expr(def, ret_ty)
            } else {
                vec![CStmt::Expr(CExpr::call(
                    "lean_internal_panic_unreachable",
                    vec![],
                ))]
            }
        } else {
            Vec::new()
        };
        CStmt::If {
            cond,
            then_body,
            else_body,
        }
    };
    while let Some(alt) = remaining.pop() {
        let cond = CExpr::binop(
            CBinOp::Eq,
            CExpr::var(scrut_name),
            CExpr::UIntLit(alt.ctor_tag as u64),
        );
        let then_body = backend.emit_expr(&alt.body, ret_ty);
        let else_body = if remaining.is_empty() {
            if let Some(def) = default {
                backend.emit_expr(def, ret_ty)
            } else {
                vec![result]
            }
        } else {
            vec![result]
        };
        result = CStmt::If {
            cond,
            then_body,
            else_body,
        };
    }
    stmts.push(result);
    stmts
}

fn ret(n: u64) -> LcnfExpr {
    LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(n)))
}

fn alt(tag: u32, body: u64) -> LcnfAlt {
    LcnfAlt {
        ctor_name: format!("C{tag}"),
        ctor_tag: tag,
        params: Vec::new(),
        body: ret(body),
    }
}

fn check(alts: &[LcnfAlt], default: Option<LcnfExpr>) {
    let default = default.map(Box::new);
    let mut actual_backend = CBackend::default_backend();
    let mut reference_backend = CBackend::default_backend();
    let actual = actual_backend.emit_scalar_case("s", alts, &default, &LcnfType::Nat);
    let reference =
        reference_scalar_case(&mut reference_backend, "s", alts, &default, &LcnfType::Nat);
    assert_eq!(actual, reference);
    assert!(!actual.is_empty());
}

#[test]
fn no_alternatives_without_default_is_the_unreachable_marker() {
    let mut backend = CBackend::default_backend();
    let stmts = backend.emit_scalar_case("s", &[], &None, &LcnfType::Nat);
    assert_eq!(
        stmts,
        vec![CStmt::Expr(CExpr::call(
            "lean_internal_panic_unreachable",
            vec![]
        ))]
    );
}

#[test]
fn no_alternatives_with_default_is_the_default_body() {
    let default = Some(Box::new(ret(9)));
    let mut backend = CBackend::default_backend();
    let stmts = backend.emit_scalar_case("s", &[], &default, &LcnfType::Nat);
    let mut other = CBackend::default_backend();
    assert_eq!(stmts, other.emit_expr(&ret(9), &LcnfType::Nat));
}

#[test]
fn alternative_counts_zero_to_four_with_and_without_default() {
    for count in 0..=4u32 {
        let alts: Vec<LcnfAlt> = (0..count).map(|t| alt(t, 100 + t as u64)).collect();
        check(&alts, None);
        check(&alts, Some(ret(7)));
    }
}

#[test]
fn a_single_alternative_is_one_if_with_the_default_or_marker_as_else() {
    let mut backend = CBackend::default_backend();
    let with_default =
        backend.emit_scalar_case("s", &[alt(2, 5)], &Some(Box::new(ret(6))), &LcnfType::Nat);
    match with_default.as_slice() {
        [CStmt::If {
            cond,
            then_body,
            else_body,
        }] => {
            assert_eq!(
                *cond,
                CExpr::binop(CBinOp::Eq, CExpr::var("s"), CExpr::UIntLit(2))
            );
            assert!(!then_body.is_empty());
            assert!(!else_body.is_empty());
        }
        other => panic!("expected one if statement, got {other:?}"),
    }
    let without_default = backend.emit_scalar_case("s", &[alt(2, 5)], &None, &LcnfType::Nat);
    match without_default.as_slice() {
        [CStmt::If { else_body, .. }] => assert_eq!(
            else_body,
            &vec![CStmt::Expr(CExpr::call(
                "lean_internal_panic_unreachable",
                vec![]
            ))]
        ),
        other => panic!("expected one if statement, got {other:?}"),
    }
}
