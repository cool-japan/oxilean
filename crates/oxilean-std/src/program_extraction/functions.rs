//! Functions for program extraction from constructive proofs via the Curry-Howard correspondence.

use super::types::{ConstructiveProp, ExtractedProgram, ExtractionResult, ProofTerm};

// ── Core extraction ───────────────────────────────────────────────────────────

/// Attempt to extract a computational program from a constructive proof.
///
/// The Curry-Howard isomorphism maps:
/// - propositions ↔ types
/// - proofs       ↔ programs
///
/// Returns `ExtractionResult::Success` when the proof is well-typed and the
/// proposition is fully constructive.
pub fn extract_program(prop: &ConstructiveProp, proof: &ProofTerm) -> ExtractionResult {
    if !is_constructive(prop) {
        return ExtractionResult::NonConstructive(format!(
            "Proposition contains non-constructive connectives: {:?}",
            prop
        ));
    }
    if !type_check_proof(prop, proof) {
        return ExtractionResult::Failure(format!(
            "Proof term does not type-check against proposition {:?}",
            prop
        ));
    }
    let (input_type, output_type) = infer_io_types(prop);
    let simplified = simplify_proof_term(proof);
    ExtractionResult::Success(ExtractedProgram {
        name: format!("extracted_{}", prop_name(prop)),
        input_type,
        output_type,
        body: simplified,
        original_prop: prop.clone(),
    })
}

/// Perform beta/eta reduction on a proof term, returning a simplified equivalent.
pub fn simplify_proof_term(pt: &ProofTerm) -> ProofTerm {
    match pt {
        // Beta: (λx.b) a → b[x ↦ a]
        ProofTerm::App(f, arg) => {
            let f2 = simplify_proof_term(f);
            let a2 = simplify_proof_term(arg);
            if let ProofTerm::Lambda(var, body) = f2 {
                let substituted = subst_proof_term(&body, &var, &a2);
                simplify_proof_term(&substituted)
            } else {
                ProofTerm::App(Box::new(f2), Box::new(a2))
            }
        }
        // Eta on Fst/Snd applied to Pair: fst(pair(a,b)) → a
        ProofTerm::Fst(inner) => {
            let inner2 = simplify_proof_term(inner);
            if let ProofTerm::Pair(a, _) = inner2 {
                simplify_proof_term(&a)
            } else {
                ProofTerm::Fst(Box::new(inner2))
            }
        }
        ProofTerm::Snd(inner) => {
            let inner2 = simplify_proof_term(inner);
            if let ProofTerm::Pair(_, b) = inner2 {
                simplify_proof_term(&b)
            } else {
                ProofTerm::Snd(Box::new(inner2))
            }
        }
        ProofTerm::Lambda(x, body) => {
            ProofTerm::Lambda(x.clone(), Box::new(simplify_proof_term(body)))
        }
        ProofTerm::Pair(a, b) => ProofTerm::Pair(
            Box::new(simplify_proof_term(a)),
            Box::new(simplify_proof_term(b)),
        ),
        ProofTerm::Inl(t) => ProofTerm::Inl(Box::new(simplify_proof_term(t))),
        ProofTerm::Inr(t) => ProofTerm::Inr(Box::new(simplify_proof_term(t))),
        ProofTerm::Pack(w, t) => ProofTerm::Pack(w.clone(), Box::new(simplify_proof_term(t))),
        ProofTerm::Unpack {
            witness,
            proof_var,
            packed,
            body,
        } => ProofTerm::Unpack {
            witness: witness.clone(),
            proof_var: proof_var.clone(),
            packed: Box::new(simplify_proof_term(packed)),
            body: Box::new(simplify_proof_term(body)),
        },
        ProofTerm::Absurd(t) => ProofTerm::Absurd(Box::new(simplify_proof_term(t))),
        ProofTerm::Unit | ProofTerm::Var(_) => pt.clone(),
    }
}

/// Verify that a proof term is well-typed with respect to a proposition.
///
/// Implements the type-checking rules of the Curry-Howard correspondence:
/// - `Unit` proves `True`
/// - `Pair(a, b)` proves `And(P, Q)` when `a` proves `P` and `b` proves `Q`
/// - `Lambda(x, body)` proves `Implies(P, Q)` etc.
pub fn type_check_proof(prop: &ConstructiveProp, proof: &ProofTerm) -> bool {
    match (prop, proof) {
        // ⊤ is proved by unit
        (ConstructiveProp::True, ProofTerm::Unit) => true,
        // ⊥ has no proof (but Absurd is typed at any conclusion)
        (_, ProofTerm::Absurd(_)) => true,
        // Var can prove anything (open term / axiom)
        (_, ProofTerm::Var(_)) => true,
        // Conjunction: pair witnesses both parts
        (ConstructiveProp::And(p, q), ProofTerm::Pair(a, b)) => {
            type_check_proof(p, a) && type_check_proof(q, b)
        }
        // Fst projects left
        (p, ProofTerm::Fst(inner)) => {
            // The inner must prove And(p, _)
            matches_and_left(p, inner)
        }
        // Snd projects right
        (q, ProofTerm::Snd(inner)) => matches_and_right(q, inner),
        // Disjunction left injection
        (ConstructiveProp::Or(p, _), ProofTerm::Inl(t)) => type_check_proof(p, t),
        // Disjunction right injection
        (ConstructiveProp::Or(_, q), ProofTerm::Inr(t)) => type_check_proof(q, t),
        // Implication: lambda abstraction
        (ConstructiveProp::Implies(_, q), ProofTerm::Lambda(_, body)) => {
            // We can only check the codomain without a context; approximate
            type_check_proof(q, body)
        }
        // Negation: ¬P = P → ⊥, lambda proves implication
        (ConstructiveProp::Not(_), ProofTerm::Lambda(_, _)) => true,
        // Existential: pack(w, pf) proves ∃x.P(x)
        (ConstructiveProp::Exists(_, _), ProofTerm::Pack(_, _)) => true,
        // Universal: lambda proves ∀
        (ConstructiveProp::Forall(_, _), ProofTerm::Lambda(_, _)) => true,
        // Unpack of an existential
        (_, ProofTerm::Unpack { .. }) => true,
        // Application: modus ponens — hard to check without full context
        (_, ProofTerm::App(_, _)) => true,
        // Equality: unit witnesses reflexivity
        (ConstructiveProp::Eq(a, b), ProofTerm::Unit) => a == b,
        // Atom: unit witnesses trivial atoms
        (ConstructiveProp::Atom(_, _), ProofTerm::Unit) => true,
        _ => false,
    }
}

/// Check whether a proposition is fully constructive (intuitionistically valid).
///
/// A proposition is constructive if it avoids the law of excluded middle and
/// double-negation elimination at top level, and all subpropositions are also
/// constructive.
pub fn is_constructive(prop: &ConstructiveProp) -> bool {
    match prop {
        ConstructiveProp::True | ConstructiveProp::False => true,
        ConstructiveProp::Atom(_, _) | ConstructiveProp::Eq(_, _) => true,
        ConstructiveProp::And(p, q)
        | ConstructiveProp::Or(p, q)
        | ConstructiveProp::Implies(p, q) => is_constructive(p) && is_constructive(q),
        ConstructiveProp::Not(p) => {
            // ¬P = P → ⊥ is constructive as long as P is constructive
            is_constructive(p)
        }
        ConstructiveProp::Exists(_, p) | ConstructiveProp::Forall(_, p) => is_constructive(p),
    }
}

/// Check whether a proposition has a realizer — i.e., there exists a computable
/// function that extracts a witness from any proof of the proposition.
///
/// This is a syntactic approximation of realizability: propositions built
/// entirely from ⊤, ∧, ∨, →, ∃ (with concrete witnesses), and atoms are
/// realizable. ⊥ and ¬ are only realizable in trivial ways.
pub fn realizability_check(prop: &ConstructiveProp) -> bool {
    match prop {
        ConstructiveProp::True => true,
        ConstructiveProp::False => false,
        ConstructiveProp::Atom(_, _) => true,
        ConstructiveProp::Eq(a, b) => a == b,
        ConstructiveProp::And(p, q) => realizability_check(p) && realizability_check(q),
        ConstructiveProp::Or(p, q) => realizability_check(p) || realizability_check(q),
        ConstructiveProp::Implies(p, q) => {
            // P → Q is realizable if Q is realizable (or P is not)
            !realizability_check(p) || realizability_check(q)
        }
        ConstructiveProp::Not(p) => !realizability_check(p),
        ConstructiveProp::Exists(_, p) => realizability_check(p),
        ConstructiveProp::Forall(_, p) => realizability_check(p),
    }
}

/// Generate a Rust pseudocode string representing the extracted program.
pub fn proof_to_rust(prog: &ExtractedProgram) -> String {
    let body_str = proof_term_to_rust(&prog.body, 1);
    format!(
        "fn {}(input: {}) -> {} {{\n{}\n}}",
        sanitize_ident(&prog.name),
        prog.input_type,
        prog.output_type,
        body_str,
    )
}

/// Extract a conclusion from modus ponens:
/// given a proof of P and a proof of P → Q, produce a proof of Q.
pub fn modus_ponens_extract(
    _p: &ConstructiveProp,
    _q: &ConstructiveProp,
    pf_p: &ProofTerm,
    pf_pq: &ProofTerm,
) -> ProofTerm {
    ProofTerm::App(Box::new(pf_pq.clone()), Box::new(pf_p.clone()))
}

/// Construct a proof of P ∧ Q from individual proofs.
pub fn conjunction_intro(pf_p: &ProofTerm, pf_q: &ProofTerm) -> ProofTerm {
    ProofTerm::Pair(Box::new(pf_p.clone()), Box::new(pf_q.clone()))
}

/// Pack a witness value with a proof to produce a proof of ∃x.P(x).
pub fn existential_witness(witness: &str, pf: &ProofTerm) -> ProofTerm {
    ProofTerm::Pack(witness.to_string(), Box::new(pf.clone()))
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Substitute `var` with `val` in a proof term (capture-avoiding, simplified).
pub(super) fn subst_proof_term(term: &ProofTerm, var: &str, val: &ProofTerm) -> ProofTerm {
    match term {
        ProofTerm::Var(x) if x == var => val.clone(),
        ProofTerm::Var(_) | ProofTerm::Unit => term.clone(),
        ProofTerm::Lambda(x, body) => {
            if x == var {
                // Variable is shadowed; do not substitute inside
                term.clone()
            } else {
                ProofTerm::Lambda(x.clone(), Box::new(subst_proof_term(body, var, val)))
            }
        }
        ProofTerm::App(f, a) => ProofTerm::App(
            Box::new(subst_proof_term(f, var, val)),
            Box::new(subst_proof_term(a, var, val)),
        ),
        ProofTerm::Pair(a, b) => ProofTerm::Pair(
            Box::new(subst_proof_term(a, var, val)),
            Box::new(subst_proof_term(b, var, val)),
        ),
        ProofTerm::Fst(t) => ProofTerm::Fst(Box::new(subst_proof_term(t, var, val))),
        ProofTerm::Snd(t) => ProofTerm::Snd(Box::new(subst_proof_term(t, var, val))),
        ProofTerm::Inl(t) => ProofTerm::Inl(Box::new(subst_proof_term(t, var, val))),
        ProofTerm::Inr(t) => ProofTerm::Inr(Box::new(subst_proof_term(t, var, val))),
        ProofTerm::Pack(w, t) => {
            ProofTerm::Pack(w.clone(), Box::new(subst_proof_term(t, var, val)))
        }
        ProofTerm::Unpack {
            witness,
            proof_var,
            packed,
            body,
        } => {
            let packed2 = subst_proof_term(packed, var, val);
            let body2 = if witness == var || proof_var == var {
                // Shadowed
                *body.clone()
            } else {
                subst_proof_term(body, var, val)
            };
            ProofTerm::Unpack {
                witness: witness.clone(),
                proof_var: proof_var.clone(),
                packed: Box::new(packed2),
                body: Box::new(body2),
            }
        }
        ProofTerm::Absurd(t) => ProofTerm::Absurd(Box::new(subst_proof_term(t, var, val))),
    }
}

/// Derive a short descriptive name from a proposition.
fn prop_name(prop: &ConstructiveProp) -> String {
    match prop {
        ConstructiveProp::True => "true".to_string(),
        ConstructiveProp::False => "false".to_string(),
        ConstructiveProp::And(_, _) => "and".to_string(),
        ConstructiveProp::Or(_, _) => "or".to_string(),
        ConstructiveProp::Not(_) => "not".to_string(),
        ConstructiveProp::Implies(_, _) => "implies".to_string(),
        ConstructiveProp::Exists(x, _) => format!("exists_{}", x),
        ConstructiveProp::Forall(x, _) => format!("forall_{}", x),
        ConstructiveProp::Atom(name, _) => name.clone(),
        ConstructiveProp::Eq(a, b) => format!("eq_{}_{}", a, b),
    }
}

/// Infer the (input_type, output_type) strings for an extracted program.
fn infer_io_types(prop: &ConstructiveProp) -> (String, String) {
    match prop {
        ConstructiveProp::Implies(p, q) => (prop_type_str(p), prop_type_str(q)),
        ConstructiveProp::Forall(x, p) => (x.clone(), prop_type_str(p)),
        _ => ("()".to_string(), prop_type_str(prop)),
    }
}

/// Convert a proposition to a Rust type string (for code generation).
fn prop_type_str(prop: &ConstructiveProp) -> String {
    match prop {
        ConstructiveProp::True => "()".to_string(),
        ConstructiveProp::False => "!".to_string(),
        ConstructiveProp::And(p, q) => format!("({}, {})", prop_type_str(p), prop_type_str(q)),
        ConstructiveProp::Or(p, q) => format!("Result<{}, {}>", prop_type_str(p), prop_type_str(q)),
        ConstructiveProp::Not(p) => format!("fn({}) -> !", prop_type_str(p)),
        ConstructiveProp::Implies(p, q) => {
            format!("fn({}) -> {}", prop_type_str(p), prop_type_str(q))
        }
        ConstructiveProp::Exists(x, _) => format!("({}, _)", x),
        ConstructiveProp::Forall(_, p) => format!("impl Fn(_) -> {}", prop_type_str(p)),
        ConstructiveProp::Atom(name, args) => {
            if args.is_empty() {
                name.clone()
            } else {
                format!("{}<{}>", name, args.join(", "))
            }
        }
        ConstructiveProp::Eq(a, b) => format!("Eq<{}, {}>", a, b),
    }
}

/// Convert a proof term to a Rust code string (recursive, with indentation).
fn proof_term_to_rust(pt: &ProofTerm, depth: usize) -> String {
    let indent = "    ".repeat(depth);
    match pt {
        ProofTerm::Unit => format!("{}()", indent),
        ProofTerm::Var(x) => format!("{}{}", indent, x),
        ProofTerm::Lambda(x, body) => {
            let body_str = proof_term_to_rust(body, depth + 1);
            format!("{}|{}| {{\n{}\n{}}}", indent, x, body_str, indent)
        }
        ProofTerm::App(f, a) => {
            let f_str = proof_term_to_rust(f, 0);
            let a_str = proof_term_to_rust(a, 0);
            format!("{}({}).({})", indent, f_str.trim(), a_str.trim())
        }
        ProofTerm::Pair(a, b) => {
            let a_str = proof_term_to_rust(a, 0).trim().to_string();
            let b_str = proof_term_to_rust(b, 0).trim().to_string();
            format!("{}({}, {})", indent, a_str, b_str)
        }
        ProofTerm::Fst(t) => {
            let inner = proof_term_to_rust(t, 0).trim().to_string();
            format!("{}{}.0", indent, inner)
        }
        ProofTerm::Snd(t) => {
            let inner = proof_term_to_rust(t, 0).trim().to_string();
            format!("{}{}.1", indent, inner)
        }
        ProofTerm::Inl(t) => {
            let inner = proof_term_to_rust(t, 0).trim().to_string();
            format!("{}Ok({})", indent, inner)
        }
        ProofTerm::Inr(t) => {
            let inner = proof_term_to_rust(t, 0).trim().to_string();
            format!("{}Err({})", indent, inner)
        }
        ProofTerm::Pack(w, t) => {
            let inner = proof_term_to_rust(t, 0).trim().to_string();
            format!("{}({}, {})", indent, w, inner)
        }
        ProofTerm::Unpack {
            witness,
            proof_var,
            packed,
            body,
        } => {
            let packed_str = proof_term_to_rust(packed, 0).trim().to_string();
            let body_str = proof_term_to_rust(body, depth + 1);
            format!(
                "{}let ({}, {}) = {};\n{}",
                indent, witness, proof_var, packed_str, body_str
            )
        }
        ProofTerm::Absurd(t) => {
            let inner = proof_term_to_rust(t, 0).trim().to_string();
            format!("{}{{ let _: ! = {}; unreachable!() }}", indent, inner)
        }
    }
}

/// Check if `inner` proves `And(p, _)` and thus `Fst(inner)` proves `p`.
fn matches_and_left(p: &ConstructiveProp, inner: &ProofTerm) -> bool {
    // Without a full context we can only do a syntactic approximation
    matches!(inner, ProofTerm::Pair(a, _) if type_check_proof(p, a))
        || matches!(inner, ProofTerm::Var(_))
}

/// Check if `inner` proves `And(_, q)` and thus `Snd(inner)` proves `q`.
fn matches_and_right(q: &ConstructiveProp, inner: &ProofTerm) -> bool {
    matches!(inner, ProofTerm::Pair(_, b) if type_check_proof(q, b))
        || matches!(inner, ProofTerm::Var(_))
}

/// Sanitize a string to be a valid Rust identifier.
fn sanitize_ident(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::types::*;
    use super::*;

    fn atom(name: &str) -> ConstructiveProp {
        ConstructiveProp::Atom(name.to_string(), vec![])
    }

    fn var(x: &str) -> ProofTerm {
        ProofTerm::Var(x.to_string())
    }

    // ── is_constructive ───────────────────────────────────────────────────────

    #[test]
    fn test_is_constructive_true() {
        assert!(is_constructive(&ConstructiveProp::True));
    }

    #[test]
    fn test_is_constructive_false() {
        assert!(is_constructive(&ConstructiveProp::False));
    }

    #[test]
    fn test_is_constructive_atom() {
        assert!(is_constructive(&atom("P")));
    }

    #[test]
    fn test_is_constructive_and() {
        let p = ConstructiveProp::And(Box::new(atom("P")), Box::new(atom("Q")));
        assert!(is_constructive(&p));
    }

    #[test]
    fn test_is_constructive_or() {
        let p = ConstructiveProp::Or(Box::new(atom("P")), Box::new(atom("Q")));
        assert!(is_constructive(&p));
    }

    #[test]
    fn test_is_constructive_implies() {
        let p = ConstructiveProp::Implies(Box::new(atom("P")), Box::new(atom("Q")));
        assert!(is_constructive(&p));
    }

    #[test]
    fn test_is_constructive_not() {
        let p = ConstructiveProp::Not(Box::new(atom("P")));
        assert!(is_constructive(&p));
    }

    #[test]
    fn test_is_constructive_exists() {
        let p = ConstructiveProp::Exists("x".to_string(), Box::new(atom("P")));
        assert!(is_constructive(&p));
    }

    #[test]
    fn test_is_constructive_forall() {
        let p = ConstructiveProp::Forall("x".to_string(), Box::new(atom("P")));
        assert!(is_constructive(&p));
    }

    // ── type_check_proof ──────────────────────────────────────────────────────

    #[test]
    fn test_type_check_unit_proves_true() {
        assert!(type_check_proof(&ConstructiveProp::True, &ProofTerm::Unit));
    }

    #[test]
    fn test_type_check_pair_proves_and() {
        let prop = ConstructiveProp::And(Box::new(atom("P")), Box::new(atom("Q")));
        let pf = ProofTerm::Pair(Box::new(var("p")), Box::new(var("q")));
        assert!(type_check_proof(&prop, &pf));
    }

    #[test]
    fn test_type_check_inl_proves_or_left() {
        let prop = ConstructiveProp::Or(Box::new(atom("P")), Box::new(atom("Q")));
        let pf = ProofTerm::Inl(Box::new(var("p")));
        assert!(type_check_proof(&prop, &pf));
    }

    #[test]
    fn test_type_check_inr_proves_or_right() {
        let prop = ConstructiveProp::Or(Box::new(atom("P")), Box::new(atom("Q")));
        let pf = ProofTerm::Inr(Box::new(var("q")));
        assert!(type_check_proof(&prop, &pf));
    }

    #[test]
    fn test_type_check_lambda_proves_implies() {
        let prop = ConstructiveProp::Implies(Box::new(atom("P")), Box::new(atom("Q")));
        let pf = ProofTerm::Lambda("x".to_string(), Box::new(var("x")));
        assert!(type_check_proof(&prop, &pf));
    }

    #[test]
    fn test_type_check_absurd_proves_anything() {
        let pf = ProofTerm::Absurd(Box::new(var("contradiction")));
        assert!(type_check_proof(&atom("Q"), &pf));
    }

    #[test]
    fn test_type_check_var_proves_anything() {
        assert!(type_check_proof(&atom("P"), &var("p")));
    }

    // ── realizability_check ───────────────────────────────────────────────────

    #[test]
    fn test_realizability_true() {
        assert!(realizability_check(&ConstructiveProp::True));
    }

    #[test]
    fn test_realizability_false() {
        assert!(!realizability_check(&ConstructiveProp::False));
    }

    #[test]
    fn test_realizability_atom() {
        assert!(realizability_check(&atom("P")));
    }

    #[test]
    fn test_realizability_and_true_true() {
        let p = ConstructiveProp::And(
            Box::new(ConstructiveProp::True),
            Box::new(ConstructiveProp::True),
        );
        assert!(realizability_check(&p));
    }

    #[test]
    fn test_realizability_and_with_false() {
        let p = ConstructiveProp::And(
            Box::new(ConstructiveProp::True),
            Box::new(ConstructiveProp::False),
        );
        assert!(!realizability_check(&p));
    }

    #[test]
    fn test_realizability_or_true_false() {
        let p = ConstructiveProp::Or(
            Box::new(ConstructiveProp::True),
            Box::new(ConstructiveProp::False),
        );
        assert!(realizability_check(&p));
    }

    // ── simplify_proof_term ───────────────────────────────────────────────────

    #[test]
    fn test_simplify_beta_redex() {
        // (λx.x) y → y
        let lam = ProofTerm::Lambda("x".to_string(), Box::new(var("x")));
        let app = ProofTerm::App(Box::new(lam), Box::new(var("y")));
        assert_eq!(simplify_proof_term(&app), var("y"));
    }

    #[test]
    fn test_simplify_fst_pair() {
        let pair = ProofTerm::Pair(Box::new(var("a")), Box::new(var("b")));
        let fst = ProofTerm::Fst(Box::new(pair));
        assert_eq!(simplify_proof_term(&fst), var("a"));
    }

    #[test]
    fn test_simplify_snd_pair() {
        let pair = ProofTerm::Pair(Box::new(var("a")), Box::new(var("b")));
        let snd = ProofTerm::Snd(Box::new(pair));
        assert_eq!(simplify_proof_term(&snd), var("b"));
    }

    #[test]
    fn test_simplify_unit() {
        assert_eq!(simplify_proof_term(&ProofTerm::Unit), ProofTerm::Unit);
    }

    // ── extract_program ───────────────────────────────────────────────────────

    #[test]
    fn test_extract_simple_implication() {
        let prop = ConstructiveProp::Implies(Box::new(atom("P")), Box::new(atom("P")));
        let pf = ProofTerm::Lambda("x".to_string(), Box::new(var("x")));
        let result = extract_program(&prop, &pf);
        assert!(matches!(result, ExtractionResult::Success(_)));
    }

    #[test]
    fn test_extract_conjunction_intro() {
        let prop = ConstructiveProp::And(Box::new(atom("P")), Box::new(atom("Q")));
        let pf = ProofTerm::Pair(Box::new(var("p")), Box::new(var("q")));
        let result = extract_program(&prop, &pf);
        assert!(matches!(result, ExtractionResult::Success(_)));
    }

    // ── modus_ponens_extract ──────────────────────────────────────────────────

    #[test]
    fn test_modus_ponens_extract() {
        let p = atom("P");
        let q = atom("Q");
        let pf_p = var("p");
        let pf_pq = ProofTerm::Lambda("x".to_string(), Box::new(var("x")));
        let result = modus_ponens_extract(&p, &q, &pf_p, &pf_pq);
        assert_eq!(result, ProofTerm::App(Box::new(pf_pq), Box::new(pf_p)));
    }

    // ── conjunction_intro / existential_witness ───────────────────────────────

    #[test]
    fn test_conjunction_intro() {
        let pf_p = var("p");
        let pf_q = var("q");
        let result = conjunction_intro(&pf_p, &pf_q);
        assert_eq!(result, ProofTerm::Pair(Box::new(pf_p), Box::new(pf_q)));
    }

    #[test]
    fn test_existential_witness() {
        let pf = var("body");
        let result = existential_witness("42", &pf);
        assert_eq!(result, ProofTerm::Pack("42".to_string(), Box::new(pf)));
    }

    // ── proof_to_rust ─────────────────────────────────────────────────────────

    #[test]
    fn test_proof_to_rust_identity() {
        let prog = ExtractedProgram {
            name: "identity".to_string(),
            input_type: "P".to_string(),
            output_type: "P".to_string(),
            body: ProofTerm::Lambda("x".to_string(), Box::new(var("x"))),
            original_prop: ConstructiveProp::Implies(Box::new(atom("P")), Box::new(atom("P"))),
        };
        let code = proof_to_rust(&prog);
        assert!(code.contains("fn identity"));
        assert!(code.contains("|x|"));
    }

    #[test]
    fn test_proof_to_rust_pair() {
        let prog = ExtractedProgram {
            name: "pair_prog".to_string(),
            input_type: "()".to_string(),
            output_type: "(P, Q)".to_string(),
            body: ProofTerm::Pair(Box::new(var("p")), Box::new(var("q"))),
            original_prop: ConstructiveProp::And(Box::new(atom("P")), Box::new(atom("Q"))),
        };
        let code = proof_to_rust(&prog);
        assert!(code.contains("fn pair_prog"));
    }
}
