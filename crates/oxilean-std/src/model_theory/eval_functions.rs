//! First-order model theory: evaluation, model checking, and theorem-building functions.
//!
//! This module implements:
//! - Term and formula evaluation under a structure and variable assignment
//! - Model-checking (satisfiability) for sentences and theories
//! - Standard theory builders (group theory, ring theory, DLO)
//! - Elementary embedding verification
//! - Löwenheim–Skolem downward theorem (elementary substructure)

use super::eval_types::{
    ElementaryEmbedding, FoAssignment, FoSignature, FoStructure, FoTheory, FoUltrafilter, Formula,
    StructureInterp, Term,
};

// ── Term evaluation ───────────────────────────────────────────────────────────

/// Evaluate a term `t` in structure `struc` under variable assignment `assign`.
///
/// Returns the domain element (as a String) that `t` denotes, or None if
/// evaluation fails (unbound variable, undefined function, etc.).
pub fn eval_term(term: &Term, struc: &FoStructure, assign: &FoAssignment) -> Option<String> {
    match term {
        Term::Var(x) => assign.get(x).cloned(),
        Term::Const(c) => {
            // Look for a Constant interpretation.
            match struc.interp(c) {
                Some(StructureInterp::Constant(val)) => Some(val.clone()),
                _ => {
                    // Constant might also just be a domain element by name.
                    if struc.contains(c) {
                        Some(c.clone())
                    } else {
                        None
                    }
                }
            }
        }
        Term::App(f, args) => {
            // Evaluate all arguments first.
            let mut evaluated_args = Vec::with_capacity(args.len());
            for arg in args {
                let val = eval_term(arg, struc, assign)?;
                evaluated_args.push(val);
            }
            // Look up the function interpretation.
            match struc.interp(f) {
                Some(StructureInterp::Function(table)) => {
                    // Find the entry matching our arguments.
                    table
                        .iter()
                        .find(|(params, _)| params.as_slice() == evaluated_args.as_slice())
                        .map(|(_, result)| result.clone())
                }
                _ => None,
            }
        }
    }
}

// ── Formula evaluation ────────────────────────────────────────────────────────

/// Evaluate a formula `phi` in structure `struc` under variable assignment `assign`.
///
/// Returns Some(true/false) if evaluation succeeds (all terms defined),
/// or None if evaluation fails (e.g., undefined term).
pub fn eval_formula(formula: &Formula, struc: &FoStructure, assign: &FoAssignment) -> Option<bool> {
    match formula {
        Formula::Atom(rel, terms) => {
            // Evaluate each term argument.
            let mut vals = Vec::with_capacity(terms.len());
            for t in terms {
                let v = eval_term(t, struc, assign)?;
                vals.push(v);
            }
            // Look up the relation interpretation.
            match struc.interp(rel) {
                Some(StructureInterp::Relation(tuples)) => {
                    Some(tuples.iter().any(|tup| tup.as_slice() == vals.as_slice()))
                }
                _ => Some(false),
            }
        }

        Formula::Equal(t1, t2) => {
            let v1 = eval_term(t1, struc, assign)?;
            let v2 = eval_term(t2, struc, assign)?;
            Some(v1 == v2)
        }

        Formula::Neg(phi) => {
            let b = eval_formula(phi, struc, assign)?;
            Some(!b)
        }

        Formula::And(phi, psi) => {
            let b1 = eval_formula(phi, struc, assign)?;
            if !b1 {
                return Some(false); // Short-circuit.
            }
            eval_formula(psi, struc, assign)
        }

        Formula::Or(phi, psi) => {
            let b1 = eval_formula(phi, struc, assign)?;
            if b1 {
                return Some(true); // Short-circuit.
            }
            eval_formula(psi, struc, assign)
        }

        Formula::Implies(phi, psi) => {
            let b1 = eval_formula(phi, struc, assign)?;
            if !b1 {
                return Some(true); // False premise ⇒ true.
            }
            eval_formula(psi, struc, assign)
        }

        Formula::Forall(x, phi) => {
            // ∀x.φ holds iff φ holds for every domain element.
            for elem in &struc.domain {
                let new_assign = assign.extend(x, elem);
                let b = eval_formula(phi, struc, &new_assign)?;
                if !b {
                    return Some(false);
                }
            }
            Some(true)
        }

        Formula::Exists(x, phi) => {
            // ∃x.φ holds iff φ holds for some domain element.
            for elem in &struc.domain {
                let new_assign = assign.extend(x, elem);
                let b = eval_formula(phi, struc, &new_assign)?;
                if b {
                    return Some(true);
                }
            }
            Some(false)
        }
    }
}

/// Check whether a sentence (formula with no free variables) is satisfied by a structure.
///
/// Returns true iff `struc ⊨ sentence`. Uses the empty assignment.
pub fn satisfies(struc: &FoStructure, sentence: &Formula) -> bool {
    let assign = FoAssignment::new();
    eval_formula(sentence, struc, &assign).unwrap_or(false)
}

/// Check whether a structure is a model of a first-order theory.
///
/// `struc` is a model of `theory` iff it satisfies every axiom in `theory`.
pub fn is_fo_model(struc: &FoStructure, theory: &FoTheory) -> bool {
    theory.axioms.iter().all(|axiom| satisfies(struc, axiom))
}

// ── Standard theory builders ──────────────────────────────────────────────────

/// Build the first-order theory of groups (associativity, identity, inverses).
///
/// Signature: functions { mul/2, inv/1 }, constants { e }.
///
/// Axioms:
/// 1. ∀x. mul(e, x) = x           (left identity)
/// 2. ∀x. mul(x, e) = x           (right identity)
/// 3. ∀x. mul(inv(x), x) = e      (left inverse)
/// 4. ∀x∀y∀z. mul(mul(x,y),z) = mul(x,mul(y,z))  (associativity)
pub fn group_theory() -> FoTheory {
    let mut sig = FoSignature::new();
    sig.add_function("mul", 2);
    sig.add_function("inv", 1);
    sig.add_constant("e");

    let e = Term::Const("e".to_string());

    // mul(e, x) = x
    let ax1 = Formula::forall(
        "x",
        Formula::Equal(
            Term::App(
                "mul".to_string(),
                vec![e.clone(), Term::Var("x".to_string())],
            ),
            Term::Var("x".to_string()),
        ),
    );

    // mul(x, e) = x
    let ax2 = Formula::forall(
        "x",
        Formula::Equal(
            Term::App(
                "mul".to_string(),
                vec![Term::Var("x".to_string()), e.clone()],
            ),
            Term::Var("x".to_string()),
        ),
    );

    // mul(inv(x), x) = e
    let ax3 = Formula::forall(
        "x",
        Formula::Equal(
            Term::App(
                "mul".to_string(),
                vec![
                    Term::App("inv".to_string(), vec![Term::Var("x".to_string())]),
                    Term::Var("x".to_string()),
                ],
            ),
            e.clone(),
        ),
    );

    // mul(mul(x,y),z) = mul(x,mul(y,z))
    let ax4 = Formula::forall(
        "x",
        Formula::forall(
            "y",
            Formula::forall(
                "z",
                Formula::Equal(
                    Term::App(
                        "mul".to_string(),
                        vec![
                            Term::App(
                                "mul".to_string(),
                                vec![Term::Var("x".to_string()), Term::Var("y".to_string())],
                            ),
                            Term::Var("z".to_string()),
                        ],
                    ),
                    Term::App(
                        "mul".to_string(),
                        vec![
                            Term::Var("x".to_string()),
                            Term::App(
                                "mul".to_string(),
                                vec![Term::Var("y".to_string()), Term::Var("z".to_string())],
                            ),
                        ],
                    ),
                ),
            ),
        ),
    );

    FoTheory::new(vec![ax1, ax2, ax3, ax4], sig)
}

/// Build the first-order theory of (commutative) rings (with 1).
///
/// Signature: functions { add/2, mul/2, neg/1 }, constants { 0, 1 }.
///
/// Axioms (8):
/// 1–4: (R, +, 0, neg) is an abelian group
/// 5: mul(1, x) = x
/// 6: mul(x, 1) = x
/// 7: mul(mul(x,y),z) = mul(x,mul(y,z))  (multiplicative associativity)
/// 8: mul(x, add(y,z)) = add(mul(x,y), mul(x,z))  (left distributivity)
pub fn ring_theory() -> FoTheory {
    let mut sig = FoSignature::new();
    sig.add_function("add", 2);
    sig.add_function("mul", 2);
    sig.add_function("neg", 1);
    sig.add_constant("0");
    sig.add_constant("1");

    let zero = Term::Const("0".to_string());
    let one = Term::Const("1".to_string());

    // add(0, x) = x
    let ax1 = Formula::forall(
        "x",
        Formula::Equal(
            Term::App(
                "add".to_string(),
                vec![zero.clone(), Term::Var("x".to_string())],
            ),
            Term::Var("x".to_string()),
        ),
    );

    // add(x, 0) = x
    let ax2 = Formula::forall(
        "x",
        Formula::Equal(
            Term::App(
                "add".to_string(),
                vec![Term::Var("x".to_string()), zero.clone()],
            ),
            Term::Var("x".to_string()),
        ),
    );

    // add(neg(x), x) = 0
    let ax3 = Formula::forall(
        "x",
        Formula::Equal(
            Term::App(
                "add".to_string(),
                vec![
                    Term::App("neg".to_string(), vec![Term::Var("x".to_string())]),
                    Term::Var("x".to_string()),
                ],
            ),
            zero.clone(),
        ),
    );

    // add(x, y) = add(y, x) — commutativity of addition
    let ax4 = Formula::forall(
        "x",
        Formula::forall(
            "y",
            Formula::Equal(
                Term::App(
                    "add".to_string(),
                    vec![Term::Var("x".to_string()), Term::Var("y".to_string())],
                ),
                Term::App(
                    "add".to_string(),
                    vec![Term::Var("y".to_string()), Term::Var("x".to_string())],
                ),
            ),
        ),
    );

    // mul(1, x) = x
    let ax5 = Formula::forall(
        "x",
        Formula::Equal(
            Term::App(
                "mul".to_string(),
                vec![one.clone(), Term::Var("x".to_string())],
            ),
            Term::Var("x".to_string()),
        ),
    );

    // mul(x, 1) = x
    let ax6 = Formula::forall(
        "x",
        Formula::Equal(
            Term::App(
                "mul".to_string(),
                vec![Term::Var("x".to_string()), one.clone()],
            ),
            Term::Var("x".to_string()),
        ),
    );

    // mul(mul(x,y),z) = mul(x,mul(y,z))
    let ax7 = Formula::forall(
        "x",
        Formula::forall(
            "y",
            Formula::forall(
                "z",
                Formula::Equal(
                    Term::App(
                        "mul".to_string(),
                        vec![
                            Term::App(
                                "mul".to_string(),
                                vec![Term::Var("x".to_string()), Term::Var("y".to_string())],
                            ),
                            Term::Var("z".to_string()),
                        ],
                    ),
                    Term::App(
                        "mul".to_string(),
                        vec![
                            Term::Var("x".to_string()),
                            Term::App(
                                "mul".to_string(),
                                vec![Term::Var("y".to_string()), Term::Var("z".to_string())],
                            ),
                        ],
                    ),
                ),
            ),
        ),
    );

    // mul(x, add(y,z)) = add(mul(x,y), mul(x,z))
    let ax8 = Formula::forall(
        "x",
        Formula::forall(
            "y",
            Formula::forall(
                "z",
                Formula::Equal(
                    Term::App(
                        "mul".to_string(),
                        vec![
                            Term::Var("x".to_string()),
                            Term::App(
                                "add".to_string(),
                                vec![Term::Var("y".to_string()), Term::Var("z".to_string())],
                            ),
                        ],
                    ),
                    Term::App(
                        "add".to_string(),
                        vec![
                            Term::App(
                                "mul".to_string(),
                                vec![Term::Var("x".to_string()), Term::Var("y".to_string())],
                            ),
                            Term::App(
                                "mul".to_string(),
                                vec![Term::Var("x".to_string()), Term::Var("z".to_string())],
                            ),
                        ],
                    ),
                ),
            ),
        ),
    );

    FoTheory::new(vec![ax1, ax2, ax3, ax4, ax5, ax6, ax7, ax8], sig)
}

/// Build the first-order theory of dense linear orders without endpoints (DLO).
///
/// Signature: relations { lt/2 }.
///
/// Axioms:
/// 1. Irreflexivity:    ∀x. ¬(x < x)
/// 2. Transitivity:     ∀x∀y∀z. (x<y ∧ y<z) → x<z
/// 3. Totality:         ∀x∀y. x<y ∨ y<x ∨ x=y
/// 4. Density:          ∀x∀y. x<y → ∃z. x<z ∧ z<y
/// 5. No least element: ∀x. ∃y. y<x
/// 6. No greatest element: ∀x. ∃y. x<y
pub fn dense_linear_order() -> FoTheory {
    let mut sig = FoSignature::new();
    sig.add_relation("lt", 2);

    // Helper: lt(a, b) as an Atom.
    let lt = |a: &str, b: &str| {
        Formula::Atom(
            "lt".to_string(),
            vec![Term::Var(a.to_string()), Term::Var(b.to_string())],
        )
    };

    // 1. Irreflexivity: ∀x. ¬(x < x)
    let ax1 = Formula::forall("x", Formula::neg(lt("x", "x")));

    // 2. Transitivity: ∀x∀y∀z. (x<y ∧ y<z) → x<z
    let ax2 = Formula::forall(
        "x",
        Formula::forall(
            "y",
            Formula::forall(
                "z",
                Formula::implies(Formula::and(lt("x", "y"), lt("y", "z")), lt("x", "z")),
            ),
        ),
    );

    // 3. Totality: ∀x∀y. x<y ∨ y<x ∨ x=y
    let ax3 = Formula::forall(
        "x",
        Formula::forall(
            "y",
            Formula::or(
                lt("x", "y"),
                Formula::or(
                    lt("y", "x"),
                    Formula::Equal(Term::Var("x".to_string()), Term::Var("y".to_string())),
                ),
            ),
        ),
    );

    // 4. Density: ∀x∀y. x<y → ∃z. x<z ∧ z<y
    let ax4 = Formula::forall(
        "x",
        Formula::forall(
            "y",
            Formula::implies(
                lt("x", "y"),
                Formula::exists("z", Formula::and(lt("x", "z"), lt("z", "y"))),
            ),
        ),
    );

    // 5. No least element: ∀x. ∃y. y<x
    let ax5 = Formula::forall("x", Formula::exists("y", lt("y", "x")));

    // 6. No greatest element: ∀x. ∃y. x<y
    let ax6 = Formula::forall("x", Formula::exists("y", lt("x", "y")));

    FoTheory::new(vec![ax1, ax2, ax3, ax4, ax5, ax6], sig)
}

// ── Consistency and model search ──────────────────────────────────────────────

/// Check whether a theory is (trivially) consistent.
///
/// A theory T is inconsistent iff it proves ⊥. We check for a simple syntactic
/// inconsistency: the simultaneous presence of φ and ¬φ among the axioms.
/// Returns true if no such contradiction is found (i.e., the theory appears consistent).
///
/// **Note**: This is a *sound but incomplete* check — it may return true even for
/// inconsistent theories whose inconsistency requires non-trivial proof.
pub fn is_consistent(theory: &FoTheory) -> bool {
    // Check: no axiom φ such that Neg(φ) is also an axiom.
    for axiom in &theory.axioms {
        let negated = Formula::neg(axiom.clone());
        if theory.axioms.contains(&negated) {
            return false;
        }
    }
    true
}

/// Search for a small finite model (1 or 2 elements) of the theory plus extra formulas.
///
/// This implements a brute-force search over domain sizes 1 and 2.
/// For each domain size n, we build a minimal "trivial" structure (with all
/// functions returning the first element, and all relations empty), then
/// try to satisfy both the theory axioms and the extra formulas.
///
/// Returns Some(structure) if found, None otherwise.
pub fn compactness_witness(theory: &FoTheory, extra: &[Formula]) -> Option<FoStructure> {
    let all_formulas: Vec<&Formula> = theory.axioms.iter().chain(extra.iter()).collect();

    // Try domain sizes 1 and 2.
    for size in 1..=2usize {
        let domain: Vec<String> = (0..size).map(|i| format!("d{i}")).collect();
        let struc = build_trivial_structure(&domain, &theory.signature);
        let assign = FoAssignment::new();
        let ok = all_formulas
            .iter()
            .all(|phi| eval_formula(phi, &struc, &assign).unwrap_or(false));
        if ok {
            return Some(struc);
        }
    }
    None
}

/// Build a trivial structure over the given domain: all functions return `domain\[0\]`,
/// all constants are `domain\[0\]`, and all relations are empty.
fn build_trivial_structure(domain: &[String], sig: &FoSignature) -> FoStructure {
    let mut struc = FoStructure::new(domain.to_vec());
    let default_elem = domain.first().cloned().unwrap_or_else(|| "d0".to_string());

    // Interpret each constant as the first domain element.
    for c in &sig.constants {
        struc.add_interp(c, StructureInterp::Constant(default_elem.clone()));
    }

    // Interpret each function symbol as the constant function returning domain[0].
    for (fname, arity) in &sig.functions {
        let mut table: Vec<(Vec<String>, String)> = Vec::new();
        // Build all argument tuples of the given arity.
        let tuples = cartesian_product_strings(domain, *arity);
        for args in tuples {
            table.push((args, default_elem.clone()));
        }
        struc.add_interp(fname, StructureInterp::Function(table));
    }

    // Relations: all empty (no tuples satisfy any relation).
    for (rname, _) in &sig.relations {
        struc.add_interp(rname, StructureInterp::Relation(vec![]));
    }

    struc
}

/// Generate all n-tuples from a slice of strings (Cartesian product).
fn cartesian_product_strings(domain: &[String], arity: usize) -> Vec<Vec<String>> {
    if arity == 0 {
        return vec![vec![]];
    }
    let mut result: Vec<Vec<String>> = vec![vec![]];
    for _ in 0..arity {
        let mut new_result = Vec::new();
        for existing in &result {
            for elem in domain {
                let mut extended = existing.clone();
                extended.push(elem.clone());
                new_result.push(extended);
            }
        }
        result = new_result;
    }
    result
}

// ── Elementary embeddings ─────────────────────────────────────────────────────

/// Check whether an elementary embedding is valid w.r.t. a theory.
///
/// An embedding e: source → target is elementary if for every sentence φ,
/// source satisfies φ iff target satisfies φ.
/// We verify this for all axioms of the theory.
pub fn check_elementary_embedding(
    embed: &ElementaryEmbedding,
    source: &FoStructure,
    target: &FoStructure,
    theory: &FoTheory,
) -> bool {
    // The embedding is between structures indexed by embed.source and embed.target.
    // We verify: for all axioms, source ⊨ φ iff target ⊨ φ.
    let _ = embed; // Indices are informational in this concrete representation.
    theory
        .axioms
        .iter()
        .all(|phi| satisfies(source, phi) == satisfies(target, phi))
}

// ── Löwenheim–Skolem downward ─────────────────────────────────────────────────

/// Compute an elementary substructure generated by a subset of domain elements.
///
/// Given a structure M and a subset A ⊆ |M|, the Löwenheim–Skolem downward
/// theorem guarantees an elementary substructure N ≼ M with A ⊆ |N|.
///
/// This implementation returns the substructure induced by `subset`: the domain
/// is restricted to `subset`, and interpretations are filtered to only include
/// tuples entirely within `subset`.
pub fn lowenheim_skolem_downward(struc: &FoStructure, subset: &[String]) -> FoStructure {
    // Filter domain to subset (preserving order).
    let new_domain: Vec<String> = struc
        .domain
        .iter()
        .filter(|e| subset.contains(e))
        .cloned()
        .collect();

    let mut sub = FoStructure::new(new_domain.clone());

    for (sym, interp) in &struc.interpretations {
        let new_interp = match interp {
            StructureInterp::Constant(c) => {
                // Keep constant if it's in the new domain.
                if new_domain.contains(c) {
                    StructureInterp::Constant(c.clone())
                } else {
                    // Fall back to first domain element if available.
                    match new_domain.first() {
                        Some(first) => StructureInterp::Constant(first.clone()),
                        None => continue,
                    }
                }
            }
            StructureInterp::Relation(tuples) => {
                // Keep only tuples entirely within new_domain.
                let filtered: Vec<Vec<String>> = tuples
                    .iter()
                    .filter(|tup| tup.iter().all(|e| new_domain.contains(e)))
                    .cloned()
                    .collect();
                StructureInterp::Relation(filtered)
            }
            StructureInterp::Function(table) => {
                // Keep only entries where all args and result are in new_domain.
                let filtered: Vec<(Vec<String>, String)> = table
                    .iter()
                    .filter(|(args, result)| {
                        args.iter().all(|e| new_domain.contains(e)) && new_domain.contains(result)
                    })
                    .cloned()
                    .collect();
                StructureInterp::Function(filtered)
            }
        };
        sub.add_interp(sym, new_interp);
    }

    sub
}

// ── Utility: build a simple FoUltrafilter ────────────────────────────────────

/// Build the trivial ultrafilter on index_set (just the full set).
pub fn trivial_ultrafilter(index_set: usize) -> FoUltrafilter {
    let full: Vec<usize> = (0..index_set).collect();
    FoUltrafilter::new(index_set, vec![full])
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model_theory::eval_types::{FoAssignment, FoStructure, StructureInterp, Term};

    // ── Helpers ───────────────────────────────────────────────────────────────

    /// Build a trivial 1-element group: domain={e}, mul(e,e)=e, inv(e)=e.
    fn trivial_group() -> FoStructure {
        let mut s = FoStructure::new(vec!["e".to_string()]);
        s.add_interp("e", StructureInterp::Constant("e".to_string()));
        s.add_interp(
            "mul",
            StructureInterp::Function(vec![(
                vec!["e".to_string(), "e".to_string()],
                "e".to_string(),
            )]),
        );
        s.add_interp(
            "inv",
            StructureInterp::Function(vec![(vec!["e".to_string()], "e".to_string())]),
        );
        s
    }

    // ── eval_term tests ───────────────────────────────────────────────────────

    #[test]
    fn test_eval_term_var_bound() {
        let s = FoStructure::new(vec!["a".to_string()]);
        let mut assign = FoAssignment::new();
        assign.set("x", "a");
        let t = Term::Var("x".to_string());
        assert_eq!(eval_term(&t, &s, &assign), Some("a".to_string()));
    }

    #[test]
    fn test_eval_term_var_unbound() {
        let s = FoStructure::new(vec!["a".to_string()]);
        let assign = FoAssignment::new();
        let t = Term::Var("x".to_string());
        assert_eq!(eval_term(&t, &s, &assign), None);
    }

    #[test]
    fn test_eval_term_const_in_domain() {
        let s = FoStructure::new(vec!["e".to_string()]);
        let assign = FoAssignment::new();
        let t = Term::Const("e".to_string());
        assert_eq!(eval_term(&t, &s, &assign), Some("e".to_string()));
    }

    #[test]
    fn test_eval_term_const_via_interp() {
        let mut s = FoStructure::new(vec!["zero".to_string()]);
        s.add_interp("0", StructureInterp::Constant("zero".to_string()));
        let assign = FoAssignment::new();
        let t = Term::Const("0".to_string());
        assert_eq!(eval_term(&t, &s, &assign), Some("zero".to_string()));
    }

    #[test]
    fn test_eval_term_app() {
        let mut s = FoStructure::new(vec!["e".to_string()]);
        s.add_interp(
            "f",
            StructureInterp::Function(vec![(vec!["e".to_string()], "e".to_string())]),
        );
        let mut assign = FoAssignment::new();
        assign.set("x", "e");
        let t = Term::App("f".to_string(), vec![Term::Var("x".to_string())]);
        assert_eq!(eval_term(&t, &s, &assign), Some("e".to_string()));
    }

    #[test]
    fn test_eval_term_app_undefined() {
        let s = FoStructure::new(vec!["e".to_string()]);
        let mut assign = FoAssignment::new();
        assign.set("x", "e");
        let t = Term::App("g".to_string(), vec![Term::Var("x".to_string())]);
        assert_eq!(eval_term(&t, &s, &assign), None);
    }

    // ── eval_formula tests ────────────────────────────────────────────────────

    #[test]
    fn test_eval_formula_equality_true() {
        let s = FoStructure::new(vec!["e".to_string()]);
        let mut assign = FoAssignment::new();
        assign.set("x", "e");
        let phi = Formula::Equal(Term::Var("x".to_string()), Term::Const("e".to_string()));
        assert_eq!(eval_formula(&phi, &s, &assign), Some(true));
    }

    #[test]
    fn test_eval_formula_equality_false() {
        let s = FoStructure::new(vec!["a".to_string(), "b".to_string()]);
        let mut assign = FoAssignment::new();
        assign.set("x", "a");
        assign.set("y", "b");
        let phi = Formula::Equal(Term::Var("x".to_string()), Term::Var("y".to_string()));
        assert_eq!(eval_formula(&phi, &s, &assign), Some(false));
    }

    #[test]
    fn test_eval_formula_neg() {
        let s = FoStructure::new(vec!["e".to_string()]);
        let mut assign = FoAssignment::new();
        assign.set("x", "e");
        // ¬(x = x) should be false.
        let phi = Formula::neg(Formula::Equal(
            Term::Var("x".to_string()),
            Term::Var("x".to_string()),
        ));
        assert_eq!(eval_formula(&phi, &s, &assign), Some(false));
    }

    #[test]
    fn test_eval_formula_and() {
        let s = FoStructure::new(vec!["e".to_string()]);
        let mut assign = FoAssignment::new();
        assign.set("x", "e");
        let eq = Formula::Equal(Term::Var("x".to_string()), Term::Const("e".to_string()));
        let phi = Formula::and(eq.clone(), eq.clone());
        assert_eq!(eval_formula(&phi, &s, &assign), Some(true));
    }

    #[test]
    fn test_eval_formula_or_first_true() {
        let s = FoStructure::new(vec!["e".to_string()]);
        let mut assign = FoAssignment::new();
        assign.set("x", "e");
        let t = Formula::Equal(Term::Var("x".to_string()), Term::Const("e".to_string()));
        let f = Formula::neg(t.clone());
        let phi = Formula::or(t, f);
        assert_eq!(eval_formula(&phi, &s, &assign), Some(true));
    }

    #[test]
    fn test_eval_formula_forall_trivial_group() {
        // ∀x. x = x is true in any structure.
        let s = FoStructure::new(vec!["e".to_string()]);
        let assign = FoAssignment::new();
        let phi = Formula::forall(
            "x",
            Formula::Equal(Term::Var("x".to_string()), Term::Var("x".to_string())),
        );
        assert_eq!(eval_formula(&phi, &s, &assign), Some(true));
    }

    #[test]
    fn test_eval_formula_exists() {
        // ∃x. x = e is true in {e}.
        let s = FoStructure::new(vec!["e".to_string()]);
        let assign = FoAssignment::new();
        let phi = Formula::exists(
            "x",
            Formula::Equal(Term::Var("x".to_string()), Term::Const("e".to_string())),
        );
        assert_eq!(eval_formula(&phi, &s, &assign), Some(true));
    }

    // ── satisfies tests ───────────────────────────────────────────────────────

    #[test]
    fn test_satisfies_reflexivity() {
        let s = FoStructure::new(vec!["a".to_string(), "b".to_string()]);
        let phi = Formula::forall(
            "x",
            Formula::Equal(Term::Var("x".to_string()), Term::Var("x".to_string())),
        );
        assert!(satisfies(&s, &phi));
    }

    // ── is_fo_model tests ─────────────────────────────────────────────────────

    #[test]
    fn test_is_fo_model_trivial_group_satisfies_group_theory() {
        let s = trivial_group();
        let t = group_theory();
        assert!(is_fo_model(&s, &t));
    }

    // ── group_theory tests ────────────────────────────────────────────────────

    #[test]
    fn test_group_theory_num_axioms() {
        let t = group_theory();
        assert_eq!(t.num_axioms(), 4);
    }

    #[test]
    fn test_group_theory_signature_has_mul() {
        let t = group_theory();
        assert!(t.signature.function_arity("mul") == Some(2));
    }

    #[test]
    fn test_group_theory_signature_has_inv() {
        let t = group_theory();
        assert!(t.signature.function_arity("inv") == Some(1));
    }

    // ── ring_theory tests ─────────────────────────────────────────────────────

    #[test]
    fn test_ring_theory_num_axioms() {
        let t = ring_theory();
        assert_eq!(t.num_axioms(), 8);
    }

    #[test]
    fn test_ring_theory_signature() {
        let t = ring_theory();
        assert_eq!(t.signature.function_arity("add"), Some(2));
        assert_eq!(t.signature.function_arity("mul"), Some(2));
        assert_eq!(t.signature.function_arity("neg"), Some(1));
    }

    // ── dense_linear_order tests ──────────────────────────────────────────────

    #[test]
    fn test_dlo_num_axioms() {
        let t = dense_linear_order();
        assert_eq!(t.num_axioms(), 6);
    }

    #[test]
    fn test_dlo_signature_has_lt() {
        let t = dense_linear_order();
        assert_eq!(t.signature.relation_arity("lt"), Some(2));
    }

    // ── is_consistent tests ───────────────────────────────────────────────────

    #[test]
    fn test_is_consistent_empty_theory() {
        let sig = FoSignature::new();
        let t = FoTheory::new(vec![], sig);
        assert!(is_consistent(&t));
    }

    #[test]
    fn test_is_consistent_group_theory() {
        let t = group_theory();
        assert!(is_consistent(&t));
    }

    #[test]
    fn test_is_consistent_contradiction() {
        let sig = FoSignature::new();
        let phi = Formula::forall(
            "x",
            Formula::Equal(Term::Var("x".to_string()), Term::Var("x".to_string())),
        );
        let not_phi = Formula::neg(phi.clone());
        let t = FoTheory::new(vec![phi, not_phi], sig);
        assert!(!is_consistent(&t));
    }

    // ── compactness_witness tests ─────────────────────────────────────────────

    #[test]
    fn test_compactness_witness_empty_theory() {
        let sig = FoSignature::new();
        let t = FoTheory::new(vec![], sig);
        let result = compactness_witness(&t, &[]);
        assert!(result.is_some());
    }

    // ── check_elementary_embedding tests ─────────────────────────────────────

    #[test]
    fn test_elementary_embedding_same_structure() {
        let s = trivial_group();
        let t = group_theory();
        let embed = ElementaryEmbedding::new(0, 0);
        assert!(check_elementary_embedding(&embed, &s, &s, &t));
    }

    // ── lowenheim_skolem_downward tests ───────────────────────────────────────

    #[test]
    fn test_lowenheim_skolem_subset() {
        let s = FoStructure::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let sub = lowenheim_skolem_downward(&s, &["a".to_string(), "b".to_string()]);
        assert_eq!(sub.domain_size(), 2);
        assert!(sub.contains("a"));
        assert!(sub.contains("b"));
        assert!(!sub.contains("c"));
    }

    #[test]
    fn test_lowenheim_skolem_full_set() {
        let s = FoStructure::new(vec!["a".to_string(), "b".to_string()]);
        let sub = lowenheim_skolem_downward(&s, &["a".to_string(), "b".to_string()]);
        assert_eq!(sub.domain_size(), 2);
    }

    #[test]
    fn test_lowenheim_skolem_empty_subset() {
        let s = FoStructure::new(vec!["a".to_string(), "b".to_string()]);
        let sub = lowenheim_skolem_downward(&s, &[]);
        assert_eq!(sub.domain_size(), 0);
    }

    // ── trivial_ultrafilter test ──────────────────────────────────────────────

    #[test]
    fn test_trivial_ultrafilter() {
        let uf = trivial_ultrafilter(3);
        assert_eq!(uf.index_set, 3);
        assert_eq!(uf.sets.len(), 1);
        assert_eq!(uf.sets[0], vec![0, 1, 2]);
    }

    // ── FoAssignment tests ────────────────────────────────────────────────────

    #[test]
    fn test_fo_assignment_extend() {
        let a = FoAssignment::new();
        let b = a.extend("x", "elem1");
        assert_eq!(b.get("x"), Some(&"elem1".to_string()));
        // Original unchanged.
        assert_eq!(a.get("x"), None);
    }

    // ── Term free_vars tests ──────────────────────────────────────────────────

    #[test]
    fn test_term_free_vars_var() {
        let t = Term::Var("x".to_string());
        assert_eq!(t.free_vars(), vec!["x".to_string()]);
    }

    #[test]
    fn test_term_free_vars_const() {
        let t = Term::Const("c".to_string());
        assert!(t.free_vars().is_empty());
    }

    #[test]
    fn test_formula_is_sentence() {
        let phi = Formula::forall(
            "x",
            Formula::Equal(Term::Var("x".to_string()), Term::Var("x".to_string())),
        );
        assert!(phi.is_sentence());
    }

    #[test]
    fn test_formula_not_sentence() {
        let phi = Formula::Equal(Term::Var("x".to_string()), Term::Var("x".to_string()));
        assert!(!phi.is_sentence());
    }
}
