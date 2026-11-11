//! `simplify_trivial_case` against the case-by-case statement of the same
//! rewrite.

use super::*;
use crate::lcnf::LcnfLit;

/// A single-alternative case without a default becomes a chain of projections;
/// everything else is rewritten in its children.
fn reference_simplify(expr: LcnfExpr) -> LcnfExpr {
    match expr {
        LcnfExpr::Case {
            scrutinee,
            alts,
            default: None,
            ..
        } if alts.len() == 1 => {
            let alt = alts.into_iter().next().expect(
                "alts has exactly one element; guaranteed by pattern guard alts.len() == 1",
            );
            let mut result = reference_simplify(alt.body);
            for (idx, param) in alt.params.iter().enumerate().rev() {
                result = LcnfExpr::Let {
                    id: param.id,
                    name: param.name.clone(),
                    ty: param.ty.clone(),
                    value: LcnfLetValue::Proj(alt.ctor_name.clone(), idx as u32, scrutinee),
                    body: Box::new(result),
                };
            }
            result
        }
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body: Box::new(reference_simplify(*body)),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts: alts
                .into_iter()
                .map(|a| LcnfAlt {
                    ctor_name: a.ctor_name,
                    ctor_tag: a.ctor_tag,
                    params: a.params,
                    body: reference_simplify(a.body),
                })
                .collect(),
            default: default.map(|d| Box::new(reference_simplify(*d))),
        },
        other => other,
    }
}

fn ret(n: u64) -> LcnfExpr {
    LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(n)))
}

fn param(id: u64) -> LcnfParam {
    LcnfParam {
        id: LcnfVarId(id),
        name: format!("p{id}"),
        ty: LcnfType::Nat,
        erased: false,
        borrowed: false,
    }
}

fn alt(tag: u32, params: Vec<LcnfParam>, body: LcnfExpr) -> LcnfAlt {
    LcnfAlt {
        ctor_name: format!("C{tag}"),
        ctor_tag: tag,
        params,
        body,
    }
}

fn case(alts: Vec<LcnfAlt>, default: Option<LcnfExpr>) -> LcnfExpr {
    LcnfExpr::Case {
        scrutinee: LcnfVarId(1),
        scrutinee_ty: LcnfType::Object,
        alts,
        default: default.map(Box::new),
    }
}

fn let_nat(id: u64, n: u64, body: LcnfExpr) -> LcnfExpr {
    LcnfExpr::Let {
        id: LcnfVarId(id),
        name: format!("x{id}"),
        ty: LcnfType::Nat,
        value: LcnfLetValue::Lit(LcnfLit::Nat(n)),
        body: Box::new(body),
    }
}

fn samples() -> Vec<LcnfExpr> {
    let single = |body: LcnfExpr| case(vec![alt(0, vec![param(10), param(11)], body)], None);
    vec![
        ret(1),
        LcnfExpr::Unreachable,
        case(vec![], None),
        case(vec![], Some(ret(2))),
        single(ret(3)),
        single(single(ret(4))),
        case(vec![alt(0, vec![], ret(5))], None),
        case(vec![alt(0, vec![param(12)], ret(5))], Some(ret(6))),
        case(
            vec![alt(0, vec![], ret(7)), alt(1, vec![param(13)], ret(8))],
            None,
        ),
        case(
            vec![alt(0, vec![], single(ret(9))), alt(1, vec![], ret(10))],
            Some(single(ret(11))),
        ),
        let_nat(20, 1, single(let_nat(21, 2, ret(12)))),
        case(
            vec![alt(0, vec![], let_nat(22, 3, single(ret(13))))],
            Some(ret(14)),
        ),
    ]
}

#[test]
fn simplify_trivial_case_agrees_with_the_reference_rewrite() {
    for expr in samples() {
        assert_eq!(
            simplify_trivial_case(expr.clone()),
            reference_simplify(expr)
        );
    }
}

#[test]
fn a_single_alternative_without_default_becomes_projections() {
    let expr = case(vec![alt(0, vec![param(10), param(11)], ret(3))], None);
    let simplified = simplify_trivial_case(expr);
    // Two projections from the scrutinee, outermost for field 0, then the body.
    match simplified {
        LcnfExpr::Let {
            id, value, body, ..
        } => {
            assert_eq!(id, LcnfVarId(10));
            assert_eq!(value, LcnfLetValue::Proj("C0".to_string(), 0, LcnfVarId(1)));
            match *body {
                LcnfExpr::Let {
                    id, value, body, ..
                } => {
                    assert_eq!(id, LcnfVarId(11));
                    assert_eq!(value, LcnfLetValue::Proj("C0".to_string(), 1, LcnfVarId(1)));
                    assert_eq!(*body, ret(3));
                }
                other => panic!("expected the second projection, got {other:?}"),
            }
        }
        other => panic!("expected the first projection, got {other:?}"),
    }
}

#[test]
fn a_single_alternative_with_a_default_stays_a_case() {
    let expr = case(vec![alt(0, vec![param(12)], ret(5))], Some(ret(6)));
    assert_eq!(simplify_trivial_case(expr.clone()), expr);
}

#[test]
fn two_alternatives_stay_a_case_with_simplified_children() {
    let inner = case(vec![alt(0, vec![], ret(1))], None);
    let expr = case(vec![alt(0, vec![], inner), alt(1, vec![], ret(2))], None);
    let expected = case(vec![alt(0, vec![], ret(1)), alt(1, vec![], ret(2))], None);
    assert_eq!(simplify_trivial_case(expr), expected);
}
