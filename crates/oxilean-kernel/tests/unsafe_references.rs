//! A safe declaration may not use an `unsafe` constant, nor a `partial`
//! definition; a partial one may not use an unsafe constant; an unsafe one
//! may use anything. This is Lean 4's `infer_constant` ("invalid
//! declaration, it uses unsafe declaration", "safe declaration must not
//! contain partial declaration"). Without it an unsafe constant — which Lean
//! admits without a termination check — could stand behind a safe theorem.

use oxilean_kernel::declaration::{
    AxiomVal, ConstantInfo, ConstantVal, DefinitionSafety, DefinitionVal, TheoremVal,
};
use oxilean_kernel::{
    check_constant_info, check_reference_safety, safety_of, Environment, Expr, KernelError, Level,
    Name, ReducibilityHint,
};

fn prop() -> Expr {
    Expr::Sort(Level::zero())
}

fn cnst(name: &str) -> Expr {
    Expr::Const(Name::from_str(name), vec![])
}

fn axiom(name: &str, ty: Expr, is_unsafe: bool) -> ConstantInfo {
    ConstantInfo::Axiom(AxiomVal {
        common: ConstantVal {
            name: Name::from_str(name),
            level_params: vec![],
            ty,
        },
        is_unsafe,
    })
}

fn definition(name: &str, ty: Expr, value: Expr, safety: DefinitionSafety) -> ConstantInfo {
    ConstantInfo::Definition(DefinitionVal {
        common: ConstantVal {
            name: Name::from_str(name),
            level_params: vec![],
            ty,
        },
        value,
        hints: ReducibilityHint::Regular(1),
        safety,
        all: vec![Name::from_str(name)],
    })
}

fn theorem(name: &str, ty: Expr, value: Expr) -> ConstantInfo {
    ConstantInfo::Theorem(TheoremVal {
        common: ConstantVal {
            name: Name::from_str(name),
            level_params: vec![],
            ty,
        },
        value,
        all: vec![Name::from_str(name)],
    })
}

/// `P : Prop`, a safe `h : P`, an unsafe `bad : P`, and a partial
/// `part : P := h`.
fn env() -> Environment {
    let mut env = Environment::new();
    for ci in [
        axiom("P", prop(), false),
        axiom("h", cnst("P"), false),
        axiom("bad", cnst("P"), true),
        definition("part", cnst("P"), cnst("h"), DefinitionSafety::Partial),
    ] {
        let name = ci.name().clone();
        if let Err(e) = check_constant_info(&mut env, ci) {
            panic!("{name} must check: {e:?}");
        }
    }
    env
}

fn refused(env: &mut Environment, ci: ConstantInfo) -> String {
    let name = ci.name().clone();
    match check_constant_info(env, ci) {
        Ok(()) => panic!("{name} must be refused"),
        Err(KernelError::Other(msg)) => {
            assert!(!env.contains(&name), "{name} must not be installed");
            msg
        }
        Err(other) => panic!("{name}: unexpected error {other:?}"),
    }
}

#[test]
fn a_safe_theorem_may_not_use_an_unsafe_constant() {
    let mut env = env();
    let msg = refused(&mut env, theorem("t", cnst("P"), cnst("bad")));
    assert_eq!(
        msg,
        "invalid declaration 't': it uses unsafe declaration 'bad'"
    );
}

#[test]
fn a_safe_definition_may_not_use_a_partial_one() {
    let mut env = env();
    let msg = refused(
        &mut env,
        definition("d", cnst("P"), cnst("part"), DefinitionSafety::Safe),
    );
    assert_eq!(
        msg,
        "invalid declaration 'd': it uses partial declaration 'part'"
    );
    let msg = refused(&mut env, theorem("t", cnst("P"), cnst("part")));
    assert!(msg.contains("partial declaration 'part'"), "{msg}");
}

#[test]
fn a_safe_type_may_not_mention_an_unsafe_constant_either() {
    // The value of `f : P → P := fun _ => bad` uses the unsafe constant.
    let mut env = env();
    let ty_mentions_bad = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("x"),
        oxilean_kernel::Node::new(cnst("P")),
        oxilean_kernel::Node::new(cnst("P")),
    );
    let msg = refused(
        &mut env,
        definition(
            "f",
            ty_mentions_bad.clone(),
            Expr::Lam(
                oxilean_kernel::BinderInfo::Default,
                Name::str("x"),
                oxilean_kernel::Node::new(cnst("P")),
                oxilean_kernel::Node::new(cnst("bad")),
            ),
            DefinitionSafety::Safe,
        ),
    );
    assert!(msg.contains("unsafe declaration 'bad'"), "{msg}");
    // The TYPE of `axiom a : Q bad` (with a safe `Q : P → Prop`) uses it.
    let eq_like = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("x"),
        oxilean_kernel::Node::new(cnst("P")),
        oxilean_kernel::Node::new(prop()),
    );
    let mut env2 = env;
    if let Err(e) = check_constant_info(&mut env2, axiom("Q", eq_like, false)) {
        panic!("Q must check: {e:?}");
    }
    let q_bad = Expr::App(
        oxilean_kernel::Node::new(cnst("Q")),
        oxilean_kernel::Node::new(cnst("bad")),
    );
    let msg = refused(&mut env2, axiom("a", q_bad, false));
    assert!(msg.contains("unsafe declaration 'bad'"), "{msg}");
}

#[test]
fn a_partial_definition_may_use_a_partial_one_but_not_an_unsafe_one() {
    let mut env = env();
    if let Err(e) = check_constant_info(
        &mut env,
        definition("p2", cnst("P"), cnst("part"), DefinitionSafety::Partial),
    ) {
        panic!("p2 must check: {e:?}");
    }
    let msg = refused(
        &mut env,
        definition("p3", cnst("P"), cnst("bad"), DefinitionSafety::Partial),
    );
    assert!(msg.contains("unsafe declaration 'bad'"), "{msg}");
}

#[test]
fn an_unsafe_declaration_may_use_anything() {
    let mut env = env();
    for ci in [
        definition("u1", cnst("P"), cnst("bad"), DefinitionSafety::Unsafe),
        definition("u2", cnst("P"), cnst("part"), DefinitionSafety::Unsafe),
        axiom("u3", cnst("P"), true),
    ] {
        let name = ci.name().clone();
        if let Err(e) = check_constant_info(&mut env, ci) {
            panic!("{name} must check: {e:?}");
        }
    }
}

#[test]
fn safety_follows_the_declaration() {
    assert_eq!(
        safety_of(&axiom("x", prop(), true)),
        DefinitionSafety::Unsafe
    );
    assert_eq!(
        safety_of(&axiom("x", prop(), false)),
        DefinitionSafety::Safe
    );
    assert_eq!(
        safety_of(&definition("x", prop(), prop(), DefinitionSafety::Partial)),
        DefinitionSafety::Partial
    );
    assert_eq!(
        safety_of(&theorem("x", prop(), prop())),
        DefinitionSafety::Safe
    );
    // An unknown constant is left to the type checker.
    let env = Environment::new();
    assert!(check_reference_safety(
        &env,
        &Name::str("x"),
        DefinitionSafety::Safe,
        &[&cnst("nowhere")]
    )
    .is_ok());
}
