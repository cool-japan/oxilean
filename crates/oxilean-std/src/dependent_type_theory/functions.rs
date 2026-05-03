//! Functions for dependent type theory operations.
//!
//! Provides beta reduction, type checking in Pure Type Systems,
//! Church and Scott encodings of natural numbers, and construction of
//! the Barendregt λ-cube (all 8 PTS instances).

use super::types::{DttContext, DttSort, DttTerm, PureTypeSystem};

// ─── Substitution and shifting ──────────────────────────────────────────────

/// Shift all free variables in `term` by `amount` when they have index ≥ `cutoff`.
///
/// This is needed to correctly substitute under binders.
pub fn shift(term: &DttTerm, amount: isize, cutoff: usize) -> DttTerm {
    match term {
        DttTerm::Var(n) => {
            if *n >= cutoff {
                let shifted = (*n as isize + amount).max(0) as usize;
                DttTerm::Var(shifted)
            } else {
                DttTerm::Var(*n)
            }
        }
        DttTerm::Sort(s) => DttTerm::Sort(s.clone()),
        DttTerm::Nat => DttTerm::Nat,
        DttTerm::Zero => DttTerm::Zero,
        DttTerm::Const(name) => DttTerm::Const(name.clone()),
        DttTerm::Succ(n) => DttTerm::Succ(Box::new(shift(n, amount, cutoff))),
        DttTerm::Fst(t) => DttTerm::Fst(Box::new(shift(t, amount, cutoff))),
        DttTerm::Snd(t) => DttTerm::Snd(Box::new(shift(t, amount, cutoff))),
        DttTerm::Pi { domain, body } => DttTerm::Pi {
            domain: Box::new(shift(domain, amount, cutoff)),
            body: Box::new(shift(body, amount, cutoff + 1)),
        },
        DttTerm::Lambda { ty, body } => DttTerm::Lambda {
            ty: Box::new(shift(ty, amount, cutoff)),
            body: Box::new(shift(body, amount, cutoff + 1)),
        },
        DttTerm::App { func, arg } => DttTerm::App {
            func: Box::new(shift(func, amount, cutoff)),
            arg: Box::new(shift(arg, amount, cutoff)),
        },
        DttTerm::Sigma { first, second } => DttTerm::Sigma {
            first: Box::new(shift(first, amount, cutoff)),
            second: Box::new(shift(second, amount, cutoff + 1)),
        },
        DttTerm::Pair { fst, snd, ty } => DttTerm::Pair {
            fst: Box::new(shift(fst, amount, cutoff)),
            snd: Box::new(shift(snd, amount, cutoff)),
            ty: Box::new(shift(ty, amount, cutoff)),
        },
        DttTerm::Elim {
            motive,
            base,
            step,
            target,
        } => DttTerm::Elim {
            motive: Box::new(shift(motive, amount, cutoff)),
            base: Box::new(shift(base, amount, cutoff)),
            step: Box::new(shift(step, amount, cutoff)),
            target: Box::new(shift(target, amount, cutoff)),
        },
    }
}

/// Substitute `replacement` for `Var(0)` in `term`, correctly shifting
/// the replacement under binders.
///
/// This implements capture-avoiding substitution using de Bruijn indices.
pub fn subst(term: &DttTerm, replacement: &DttTerm) -> DttTerm {
    subst_at(term, replacement, 0)
}

fn subst_at(term: &DttTerm, replacement: &DttTerm, depth: usize) -> DttTerm {
    match term {
        DttTerm::Var(n) => {
            if *n == depth {
                // Shift the replacement to account for the binders we are under.
                shift(replacement, depth as isize, 0)
            } else if *n > depth {
                DttTerm::Var(n - 1)
            } else {
                DttTerm::Var(*n)
            }
        }
        DttTerm::Sort(s) => DttTerm::Sort(s.clone()),
        DttTerm::Nat => DttTerm::Nat,
        DttTerm::Zero => DttTerm::Zero,
        DttTerm::Const(name) => DttTerm::Const(name.clone()),
        DttTerm::Succ(n) => DttTerm::Succ(Box::new(subst_at(n, replacement, depth))),
        DttTerm::Fst(t) => DttTerm::Fst(Box::new(subst_at(t, replacement, depth))),
        DttTerm::Snd(t) => DttTerm::Snd(Box::new(subst_at(t, replacement, depth))),
        DttTerm::Pi { domain, body } => DttTerm::Pi {
            domain: Box::new(subst_at(domain, replacement, depth)),
            body: Box::new(subst_at(body, replacement, depth + 1)),
        },
        DttTerm::Lambda { ty, body } => DttTerm::Lambda {
            ty: Box::new(subst_at(ty, replacement, depth)),
            body: Box::new(subst_at(body, replacement, depth + 1)),
        },
        DttTerm::App { func, arg } => DttTerm::App {
            func: Box::new(subst_at(func, replacement, depth)),
            arg: Box::new(subst_at(arg, replacement, depth)),
        },
        DttTerm::Sigma { first, second } => DttTerm::Sigma {
            first: Box::new(subst_at(first, replacement, depth)),
            second: Box::new(subst_at(second, replacement, depth + 1)),
        },
        DttTerm::Pair { fst, snd, ty } => DttTerm::Pair {
            fst: Box::new(subst_at(fst, replacement, depth)),
            snd: Box::new(subst_at(snd, replacement, depth)),
            ty: Box::new(subst_at(ty, replacement, depth)),
        },
        DttTerm::Elim {
            motive,
            base,
            step,
            target,
        } => DttTerm::Elim {
            motive: Box::new(subst_at(motive, replacement, depth)),
            base: Box::new(subst_at(base, replacement, depth)),
            step: Box::new(subst_at(step, replacement, depth)),
            target: Box::new(subst_at(target, replacement, depth)),
        },
    }
}

// ─── Beta reduction ──────────────────────────────────────────────────────────

/// Perform full beta reduction to normal form.
///
/// Applies beta and iota (elimination) reduction rules exhaustively.
/// May not terminate on non-normalizing terms; uses a fuel-based step limit
/// (up to 100_000 steps) to prevent infinite loops.
pub fn normalize_beta(term: &DttTerm) -> DttTerm {
    let mut current = term.clone();
    let mut steps = 0usize;
    loop {
        match beta_step_full(&current) {
            None => return current,
            Some(next) => {
                current = next;
                steps += 1;
                if steps >= 100_000 {
                    return current;
                }
            }
        }
    }
}

/// Perform one step of beta/iota reduction (normal-order, leftmost-outermost).
///
/// Returns `None` if the term is already in normal form.
pub fn beta_step_full(term: &DttTerm) -> Option<DttTerm> {
    match term {
        // Beta: (λ ty . body) arg → body[arg/0]
        DttTerm::App { func, arg } => {
            if let DttTerm::Lambda { body, .. } = func.as_ref() {
                let substituted = subst(body, arg);
                return Some(substituted);
            }
            // Reduce function first (normal order)
            if let Some(func2) = beta_step_full(func) {
                return Some(DttTerm::App {
                    func: Box::new(func2),
                    arg: arg.clone(),
                });
            }
            // Then argument
            if let Some(arg2) = beta_step_full(arg) {
                return Some(DttTerm::App {
                    func: func.clone(),
                    arg: Box::new(arg2),
                });
            }
            None
        }
        // Reduce under lambda
        DttTerm::Lambda { ty, body } => {
            if let Some(ty2) = beta_step_full(ty) {
                return Some(DttTerm::Lambda {
                    ty: Box::new(ty2),
                    body: body.clone(),
                });
            }
            if let Some(body2) = beta_step_full(body) {
                return Some(DttTerm::Lambda {
                    ty: ty.clone(),
                    body: Box::new(body2),
                });
            }
            None
        }
        // Reduce under Pi
        DttTerm::Pi { domain, body } => {
            if let Some(d2) = beta_step_full(domain) {
                return Some(DttTerm::Pi {
                    domain: Box::new(d2),
                    body: body.clone(),
                });
            }
            if let Some(b2) = beta_step_full(body) {
                return Some(DttTerm::Pi {
                    domain: domain.clone(),
                    body: Box::new(b2),
                });
            }
            None
        }
        // Iota: Nat.elim P base step Zero → base
        DttTerm::Elim {
            motive,
            base,
            step,
            target,
        } => {
            if let DttTerm::Zero = target.as_ref() {
                return Some(base.as_ref().clone());
            }
            // Iota: Nat.elim P base step (Succ n) → step n (Nat.elim P base step n)
            if let DttTerm::Succ(n) = target.as_ref() {
                let rec = DttTerm::Elim {
                    motive: motive.clone(),
                    base: base.clone(),
                    step: step.clone(),
                    target: n.clone(),
                };
                // step n rec
                let applied_n = DttTerm::App {
                    func: step.clone(),
                    arg: n.clone(),
                };
                return Some(DttTerm::App {
                    func: Box::new(applied_n),
                    arg: Box::new(rec),
                });
            }
            // Reduce target first
            if let Some(t2) = beta_step_full(target) {
                return Some(DttTerm::Elim {
                    motive: motive.clone(),
                    base: base.clone(),
                    step: step.clone(),
                    target: Box::new(t2),
                });
            }
            // Then motive
            if let Some(m2) = beta_step_full(motive) {
                return Some(DttTerm::Elim {
                    motive: Box::new(m2),
                    base: base.clone(),
                    step: step.clone(),
                    target: target.clone(),
                });
            }
            None
        }
        // Fst (fst, snd, ty) → fst
        DttTerm::Fst(t) => {
            if let DttTerm::Pair { fst, .. } = t.as_ref() {
                return Some(fst.as_ref().clone());
            }
            beta_step_full(t).map(|t2| DttTerm::Fst(Box::new(t2)))
        }
        // Snd (fst, snd, ty) → snd
        DttTerm::Snd(t) => {
            if let DttTerm::Pair { snd, .. } = t.as_ref() {
                return Some(snd.as_ref().clone());
            }
            beta_step_full(t).map(|t2| DttTerm::Snd(Box::new(t2)))
        }
        DttTerm::Succ(n) => beta_step_full(n).map(|n2| DttTerm::Succ(Box::new(n2))),
        DttTerm::Sigma { first, second } => {
            if let Some(f2) = beta_step_full(first) {
                return Some(DttTerm::Sigma {
                    first: Box::new(f2),
                    second: second.clone(),
                });
            }
            beta_step_full(second).map(|s2| DttTerm::Sigma {
                first: first.clone(),
                second: Box::new(s2),
            })
        }
        DttTerm::Pair { fst, snd, ty } => {
            if let Some(f2) = beta_step_full(fst) {
                return Some(DttTerm::Pair {
                    fst: Box::new(f2),
                    snd: snd.clone(),
                    ty: ty.clone(),
                });
            }
            if let Some(s2) = beta_step_full(snd) {
                return Some(DttTerm::Pair {
                    fst: fst.clone(),
                    snd: Box::new(s2),
                    ty: ty.clone(),
                });
            }
            None
        }
        // No reduction for atoms
        DttTerm::Var(_) | DttTerm::Sort(_) | DttTerm::Nat | DttTerm::Zero | DttTerm::Const(_) => {
            None
        }
    }
}

/// Check whether two terms are beta-equivalent.
///
/// Normalizes both terms and compares structurally.
pub fn is_beta_equivalent(a: &DttTerm, b: &DttTerm) -> bool {
    let na = normalize_beta(a);
    let nb = normalize_beta(b);
    na == nb
}

// ─── Type checking in Pure Type Systems ─────────────────────────────────────

/// Type-check a term in a Pure Type System.
///
/// Returns the sort of the term if it is a valid sort-level type,
/// or `None` if the term is not a type (i.e. its type is not a sort).
///
/// This is a simplified sort-level checker: it propagates sort information
/// through the typing derivation without requiring full type inference.
pub fn check_pure_type_system(
    pts: &PureTypeSystem,
    ctx: &DttContext,
    term: &DttTerm,
) -> Option<DttSort> {
    match term {
        DttTerm::Sort(s) => {
            // s : s'  iff (s, s') ∈ Axioms
            pts.axiom_sort(s).cloned()
        }
        DttTerm::Var(n) => {
            // Look up the type of Var(n) in the context; if it's a sort, return it.
            let ty = ctx.lookup(*n)?;
            infer_sort(pts, ctx, ty)
        }
        DttTerm::Pi { domain, body } => {
            // Γ ⊢ A : s₁   Γ,x:A ⊢ B : s₂   (s₁,s₂,s₃) ∈ Rules  ⟹  Γ ⊢ Π(x:A).B : s₃
            let s1 = check_pure_type_system(pts, ctx, domain)?;
            let extended = ctx.extend("_", domain.as_ref().clone());
            let s2 = check_pure_type_system(pts, &extended, body)?;
            pts.rule_sort(&s1, &s2).cloned()
        }
        DttTerm::Lambda { ty, body } => {
            // λ(x:A).t : Π(x:A).B  provided  Γ,x:A ⊢ t : B
            // We return the sort of the Pi type.
            let s1 = check_pure_type_system(pts, ctx, ty)?;
            let extended = ctx.extend("_", ty.as_ref().clone());
            let s2 = check_pure_type_system(pts, &extended, body)?;
            pts.rule_sort(&s1, &s2).cloned()
        }
        DttTerm::App { func, arg: _ } => {
            // f : Π(x:A).B   ⟹  f a : B[a/x]
            // We just propagate the sort of the Pi codomain.
            let func_sort = check_pure_type_system(pts, ctx, func)?;
            // If func is a Pi-typed thing in the right sort, return the codomain sort.
            // For simplicity, we return the function's sort as approximation.
            Some(func_sort)
        }
        DttTerm::Nat => Some(DttSort::Type),
        DttTerm::Zero => Some(DttSort::Type),
        DttTerm::Succ(_) => Some(DttSort::Type),
        DttTerm::Sigma { first, second } => {
            let s1 = check_pure_type_system(pts, ctx, first)?;
            let extended = ctx.extend("_", first.as_ref().clone());
            let s2 = check_pure_type_system(pts, &extended, second)?;
            pts.rule_sort(&s1, &s2).cloned()
        }
        DttTerm::Pair { ty, .. } => infer_sort(pts, ctx, ty),
        DttTerm::Fst(t) | DttTerm::Snd(t) => check_pure_type_system(pts, ctx, t),
        DttTerm::Elim { motive, .. } => check_pure_type_system(pts, ctx, motive),
        DttTerm::Const(_) => {
            // Free constants are assumed to have sort Type.
            Some(DttSort::Type)
        }
    }
}

/// Infer the sort of a type expression `ty` in context `ctx`.
fn infer_sort(pts: &PureTypeSystem, ctx: &DttContext, ty: &DttTerm) -> Option<DttSort> {
    check_pure_type_system(pts, ctx, ty)
}

// ─── Standard PTS instances ──────────────────────────────────────────────────

/// The Calculus of Constructions (CoC / λC).
///
/// Sorts: {Prop, Type, Kind}.
/// Axioms: Prop : Type, Type : Kind.
/// Rules: all combinations that are standard in CoC.
///
/// The three-sort presentation (Prop, Type, Kind) allows self-consistent
/// universe stratification. This is the most expressive system in the
/// λ-cube and forms the foundation of the Coq proof assistant's core theory.
pub fn calculus_of_constructions() -> PureTypeSystem {
    PureTypeSystem {
        axioms: vec![
            (DttSort::Prop, DttSort::Type),
            (DttSort::Type, DttSort::Kind),
        ],
        rules: vec![
            (DttSort::Prop, DttSort::Prop, DttSort::Prop),
            (DttSort::Prop, DttSort::Type, DttSort::Type),
            (DttSort::Type, DttSort::Prop, DttSort::Prop),
            (DttSort::Type, DttSort::Type, DttSort::Type),
            (DttSort::Kind, DttSort::Type, DttSort::Type),
            (DttSort::Kind, DttSort::Kind, DttSort::Kind),
        ],
    }
}

/// System F (polymorphic lambda calculus / λ2).
///
/// Sorts: {Prop, Type}.
/// Axioms: Prop : Type.
/// Rules: (Prop,Prop) and (Type,Prop).
///
/// System F adds second-order quantification (∀ X : Type . t) but
/// does not allow type-level functions (no (Type,Type) rule).
pub fn system_f() -> PureTypeSystem {
    PureTypeSystem {
        axioms: vec![(DttSort::Prop, DttSort::Type)],
        rules: vec![
            (DttSort::Prop, DttSort::Prop, DttSort::Prop),
            (DttSort::Type, DttSort::Prop, DttSort::Prop),
        ],
    }
}

/// System Fω (higher-order polymorphic lambda calculus / λω).
///
/// Sorts: {Prop, Type}.
/// Axioms: Prop : Type.
/// Rules: (Prop,Prop), (Type,Prop), (Type,Type).
///
/// System Fω extends System F with type-level functions (type operators),
/// enabling type constructors that take types as arguments.
pub fn system_fomega() -> PureTypeSystem {
    PureTypeSystem {
        axioms: vec![(DttSort::Prop, DttSort::Type)],
        rules: vec![
            (DttSort::Prop, DttSort::Prop, DttSort::Prop),
            (DttSort::Type, DttSort::Prop, DttSort::Prop),
            (DttSort::Type, DttSort::Type, DttSort::Type),
        ],
    }
}

/// System λP (LF / Edinburgh Logical Framework).
///
/// Sorts: {Prop, Type}.
/// Axioms: Prop : Type.
/// Rules: (Prop,Prop) and (Prop,Type).
///
/// λP allows dependent types (Pi types whose domain is a term of type Prop)
/// but not polymorphism (no (Type,Prop) rule).
pub fn system_lp() -> PureTypeSystem {
    PureTypeSystem {
        axioms: vec![(DttSort::Prop, DttSort::Type)],
        rules: vec![
            (DttSort::Prop, DttSort::Prop, DttSort::Prop),
            (DttSort::Prop, DttSort::Type, DttSort::Type),
        ],
    }
}

/// The simple type theory (λ→, STLC at the type level).
///
/// Sorts: {Prop, Type}.
/// Axioms: Prop : Type.
/// Rules: only (Prop,Prop).
///
/// The weakest system in the λ-cube: only simple function types.
pub fn simple_types() -> PureTypeSystem {
    PureTypeSystem {
        axioms: vec![(DttSort::Prop, DttSort::Type)],
        rules: vec![(DttSort::Prop, DttSort::Prop, DttSort::Prop)],
    }
}

/// System λ2 (= System F): second-order polymorphism without type operators.
///
/// Alias for `system_f()` following the Barendregt cube naming.
pub fn lambda2() -> PureTypeSystem {
    system_f()
}

/// System λω (weak) — type operators without polymorphism.
///
/// Sorts: {Prop, Type}.
/// Axioms: Prop : Type.
/// Rules: (Prop,Prop), (Type,Type).
pub fn lambda_omega_weak() -> PureTypeSystem {
    PureTypeSystem {
        axioms: vec![(DttSort::Prop, DttSort::Type)],
        rules: vec![
            (DttSort::Prop, DttSort::Prop, DttSort::Prop),
            (DttSort::Type, DttSort::Type, DttSort::Type),
        ],
    }
}

/// System λPω — dependent types plus type operators (no polymorphism).
///
/// Sorts: {Prop, Type}.
/// Axioms: Prop : Type.
/// Rules: (Prop,Prop), (Prop,Type), (Type,Type).
pub fn lambda_p_omega() -> PureTypeSystem {
    PureTypeSystem {
        axioms: vec![(DttSort::Prop, DttSort::Type)],
        rules: vec![
            (DttSort::Prop, DttSort::Prop, DttSort::Prop),
            (DttSort::Prop, DttSort::Type, DttSort::Type),
            (DttSort::Type, DttSort::Type, DttSort::Type),
        ],
    }
}

/// The Barendregt λ-cube: all 8 PTS instances.
///
/// Returns a vector of `(name, system)` pairs covering all corners of the
/// λ-cube, from the simplest (λ→) to the most expressive (CoC = λC).
///
/// The cube is parameterized by three axes:
/// - λ2: adds second-order polymorphism (Type,Prop rule).
/// - λω: adds type operators (Type,Type rule).
/// - λP: adds dependent types (Prop,Type rule).
pub fn barendregt_cube() -> Vec<(&'static str, PureTypeSystem)> {
    vec![
        ("λ→", simple_types()),
        ("λ2", system_f()),
        ("λω", lambda_omega_weak()),
        ("λP", system_lp()),
        ("λ2ω", system_fomega()),
        ("λPω", lambda_p_omega()),
        ("λP2", lambda_p2()),
        ("λC", calculus_of_constructions()),
    ]
}

/// System λP2 — dependent types plus polymorphism (no type operators).
///
/// Sorts: {Prop, Type}.
/// Axioms: Prop : Type.
/// Rules: (Prop,Prop), (Prop,Type), (Type,Prop).
pub fn lambda_p2() -> PureTypeSystem {
    PureTypeSystem {
        axioms: vec![(DttSort::Prop, DttSort::Type)],
        rules: vec![
            (DttSort::Prop, DttSort::Prop, DttSort::Prop),
            (DttSort::Prop, DttSort::Type, DttSort::Type),
            (DttSort::Type, DttSort::Prop, DttSort::Prop),
        ],
    }
}

// ─── Church encodings ────────────────────────────────────────────────────────

/// Church numeral encoding of `n` : `λ s z . s (s (... (s z) ...))` (n times).
///
/// In de Bruijn form:
/// - `Var(0)` refers to `z` (innermost binder).
/// - `Var(1)` refers to `s` (outermost binder).
///
/// Type: `∀ A : Type, (A → A) → A → A`
///
/// Computes `λs.λz. s^n z`.
pub fn church_numeral(n: u64) -> DttTerm {
    // Build the body: s^n z = Var(1) applied n times to Var(0)
    let mut body = DttTerm::Var(0); // z
    for _ in 0..n {
        body = DttTerm::App {
            func: Box::new(DttTerm::Var(1)), // s
            arg: Box::new(body),
        };
    }
    // λs. λz. body
    DttTerm::Lambda {
        ty: Box::new(DttTerm::Sort(DttSort::Type)), // s : A → A (approximated as Type)
        body: Box::new(DttTerm::Lambda {
            ty: Box::new(DttTerm::Sort(DttSort::Type)), // z : A
            body: Box::new(body),
        }),
    }
}

/// Decode a Church numeral to a Rust `u64`.
///
/// Applies the term to a successor function and zero, then counts the
/// number of `Succ` applications on the result. Returns `None` if the
/// term does not normalize to a Church numeral pattern.
///
/// The term is applied as: `t (λx. Succ x) Zero`, then normalized,
/// and the resulting Nat structure is counted.
pub fn church_to_nat(t: &DttTerm) -> Option<u64> {
    // Apply t to successor and zero, normalize, count Succ's.
    let succ_fn = DttTerm::Lambda {
        ty: Box::new(DttTerm::Nat),
        body: Box::new(DttTerm::Succ(Box::new(DttTerm::Var(0)))),
    };
    let applied_s = DttTerm::App {
        func: Box::new(t.clone()),
        arg: Box::new(succ_fn),
    };
    let applied_z = DttTerm::App {
        func: Box::new(applied_s),
        arg: Box::new(DttTerm::Zero),
    };
    let normalized = normalize_beta(&applied_z);
    count_succs(&normalized)
}

/// Count the number of `Succ` wrappers around `Zero`.
fn count_succs(term: &DttTerm) -> Option<u64> {
    match term {
        DttTerm::Zero => Some(0),
        DttTerm::Succ(inner) => count_succs(inner).map(|n| n + 1),
        _ => None,
    }
}

/// Scott encoding of natural numbers.
///
/// The Scott encoding of `ℕ` in a variant encoding is:
/// ```text
/// Nat_Scott = ∀ A : Type, A → (Nat_Scott → A) → A
/// zero_scott = λz s. z
/// succ_scott n = λz s. s n
/// ```
///
/// Returns the Scott-encoded `zero` term:
/// `λz. λs. z`
///
/// The successor `n+1` would be: `λz. λs. s n`
pub fn scott_encoding_nat() -> DttTerm {
    // zero = λz. λs. z
    // In de Bruijn: λ (Type). λ (Type). Var(1)
    DttTerm::Lambda {
        ty: Box::new(DttTerm::Sort(DttSort::Type)), // z : A
        body: Box::new(DttTerm::Lambda {
            ty: Box::new(DttTerm::Sort(DttSort::Type)), // s : Nat → A
            body: Box::new(DttTerm::Var(1)),            // returns z (index 1 = outer lambda)
        }),
    }
}

/// Scott-encode the successor of a natural number `n`.
///
/// `succ_scott(n) = λz. λs. s n`
///
/// In de Bruijn: `λ Type . λ Type . (Var(0) n_shifted)`
/// where `n_shifted` is `n` with all free variables shifted by 2
/// (to account for the two enclosing binders).
pub fn scott_succ(n: &DttTerm) -> DttTerm {
    // Shift n by 2 (two lambdas: z and s)
    let n_shifted = shift(n, 2, 0);
    DttTerm::Lambda {
        ty: Box::new(DttTerm::Sort(DttSort::Type)), // z : A
        body: Box::new(DttTerm::Lambda {
            ty: Box::new(DttTerm::Sort(DttSort::Type)), // s : N → A
            body: Box::new(DttTerm::App {
                func: Box::new(DttTerm::Var(0)), // s
                arg: Box::new(n_shifted),
            }),
        }),
    }
}

/// Check if a term is a closed Scott-encoded natural number by structural analysis.
///
/// Returns the natural number value if the term is a sequence of
/// `scott_succ` applications to `scott_encoding_nat()` (zero),
/// by counting the depth of the Scott encoding structure.
///
/// Scott zero = `λz.λs.Var(1)` (returns z)
/// Scott succ(n) = `λz.λs.(Var(0) n)` (returns s applied to n)
pub fn scott_nat_value(term: &DttTerm) -> Option<u64> {
    scott_nat_value_inner(term, 0)
}

fn scott_nat_value_inner(term: &DttTerm, depth: u64) -> Option<u64> {
    // Normalize first to ensure we see the canonical form.
    let nf = normalize_beta(term);
    match &nf {
        // Scott zero: λ Type . λ Type . Var(1)
        DttTerm::Lambda {
            body: outer_body, ..
        } => {
            match outer_body.as_ref() {
                // λ.λ.Var(1) → this is zero
                DttTerm::Lambda {
                    body: inner_body, ..
                } => {
                    match inner_body.as_ref() {
                        DttTerm::Var(1) => Some(depth),
                        // Scott succ: λ.λ.(Var(0) pred)  → body is App(Var(0), pred)
                        DttTerm::App { func, arg } => {
                            if matches!(func.as_ref(), DttTerm::Var(0)) {
                                // pred needs to be de-shifted by 2 to undo the shift in scott_succ
                                let pred = shift(arg, -2, 0);
                                scott_nat_value_inner(&pred, depth + 1)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Shift / substitution ──────────────────────────────────────────────

    #[test]
    fn test_shift_var_below_cutoff() {
        // Var(0) with cutoff 1 — should not shift.
        let t = DttTerm::Var(0);
        assert_eq!(shift(&t, 1, 1), DttTerm::Var(0));
    }

    #[test]
    fn test_shift_var_at_cutoff() {
        // Var(1) with cutoff 1 — should shift to Var(2).
        let t = DttTerm::Var(1);
        assert_eq!(shift(&t, 1, 1), DttTerm::Var(2));
    }

    #[test]
    fn test_shift_sort_unchanged() {
        let t = DttTerm::Sort(DttSort::Prop);
        assert_eq!(shift(&t, 5, 0), t);
    }

    #[test]
    fn test_subst_simple() {
        // (λ. Var(0))[Zero/0] should beta-reduce to Zero.
        // Actually, subst(Var(0), Zero) = Zero
        let t = DttTerm::Var(0);
        let result = subst(&t, &DttTerm::Zero);
        assert_eq!(result, DttTerm::Zero);
    }

    #[test]
    fn test_subst_var_decrements_free() {
        // Var(1)[Zero/0] = Var(0) — free variable gets decremented
        let t = DttTerm::Var(1);
        let result = subst(&t, &DttTerm::Zero);
        assert_eq!(result, DttTerm::Var(0));
    }

    #[test]
    fn test_subst_under_lambda() {
        // λ. Var(1) substituted with Zero at depth 0:
        // Var(1) under the lambda has depth 1, so subst_at(Var(1), Zero, 1) = Var(0)
        // because n > depth... wait, subst(λ.Var(1), Zero) → λ. subst_at(Var(1), Zero, 1)
        // n=1, depth=1 → substitute → shift(Zero, 1, 0) = Zero. But wait: Var(1) is inside
        // lambda (depth=1), so Var(1) refers to the outer free var → becomes subst target.
        let inner = DttTerm::Var(1);
        let lam = DttTerm::Lambda {
            ty: Box::new(DttTerm::Sort(DttSort::Type)),
            body: Box::new(inner),
        };
        let result = subst(&lam, &DttTerm::Zero);
        // Var(1) under one lambda → depth=1 matches → replaced by shift(Zero, 1, 0) = Zero
        assert_eq!(
            result,
            DttTerm::Lambda {
                ty: Box::new(DttTerm::Sort(DttSort::Type)),
                body: Box::new(DttTerm::Zero),
            }
        );
    }

    // ── Beta reduction ────────────────────────────────────────────────────

    #[test]
    fn test_normalize_identity() {
        // (λ.Var(0)) Zero → Zero
        let id = DttTerm::Lambda {
            ty: Box::new(DttTerm::Sort(DttSort::Type)),
            body: Box::new(DttTerm::Var(0)),
        };
        let app = DttTerm::App {
            func: Box::new(id),
            arg: Box::new(DttTerm::Zero),
        };
        let result = normalize_beta(&app);
        assert_eq!(result, DttTerm::Zero);
    }

    #[test]
    fn test_normalize_const() {
        // Constants are in normal form.
        let t = DttTerm::Const("Foo".to_string());
        assert_eq!(normalize_beta(&t), t);
    }

    #[test]
    fn test_normalize_sort_unchanged() {
        let t = DttTerm::Sort(DttSort::Type);
        assert_eq!(normalize_beta(&t), t);
    }

    #[test]
    fn test_normalize_succ_zero() {
        let t = DttTerm::Succ(Box::new(DttTerm::Zero));
        assert_eq!(normalize_beta(&t), t);
    }

    #[test]
    fn test_normalize_fst_pair() {
        // Fst (Pair a b ty) → a
        let a = DttTerm::Zero;
        let b = DttTerm::Succ(Box::new(DttTerm::Zero));
        let ty = DttTerm::Nat;
        let pair = DttTerm::Pair {
            fst: Box::new(a.clone()),
            snd: Box::new(b),
            ty: Box::new(ty),
        };
        let fst = DttTerm::Fst(Box::new(pair));
        assert_eq!(normalize_beta(&fst), a);
    }

    #[test]
    fn test_normalize_snd_pair() {
        let a = DttTerm::Zero;
        let b = DttTerm::Succ(Box::new(DttTerm::Zero));
        let ty = DttTerm::Nat;
        let pair = DttTerm::Pair {
            fst: Box::new(a),
            snd: Box::new(b.clone()),
            ty: Box::new(ty),
        };
        let snd = DttTerm::Snd(Box::new(pair));
        assert_eq!(normalize_beta(&snd), b);
    }

    #[test]
    fn test_is_beta_equivalent_same() {
        let t = DttTerm::Zero;
        assert!(is_beta_equivalent(&t, &t));
    }

    #[test]
    fn test_is_beta_equivalent_after_reduction() {
        // (λ.Var(0)) Zero ≡ Zero
        let id = DttTerm::Lambda {
            ty: Box::new(DttTerm::Sort(DttSort::Type)),
            body: Box::new(DttTerm::Var(0)),
        };
        let app = DttTerm::App {
            func: Box::new(id),
            arg: Box::new(DttTerm::Zero),
        };
        assert!(is_beta_equivalent(&app, &DttTerm::Zero));
    }

    #[test]
    fn test_is_not_beta_equivalent() {
        let zero = DttTerm::Zero;
        let one = DttTerm::Succ(Box::new(DttTerm::Zero));
        assert!(!is_beta_equivalent(&zero, &one));
    }

    // ── Church encodings ──────────────────────────────────────────────────

    #[test]
    fn test_church_numeral_zero() {
        let c0 = church_numeral(0);
        let n = church_to_nat(&c0);
        assert_eq!(n, Some(0), "church_numeral(0) should decode to 0");
    }

    #[test]
    fn test_church_numeral_one() {
        let c1 = church_numeral(1);
        let n = church_to_nat(&c1);
        assert_eq!(n, Some(1), "church_numeral(1) should decode to 1");
    }

    #[test]
    fn test_church_numeral_two() {
        let c2 = church_numeral(2);
        let n = church_to_nat(&c2);
        assert_eq!(n, Some(2), "church_numeral(2) should decode to 2");
    }

    #[test]
    fn test_church_numeral_five() {
        let c5 = church_numeral(5);
        let n = church_to_nat(&c5);
        assert_eq!(n, Some(5), "church_numeral(5) should decode to 5");
    }

    #[test]
    fn test_church_numeral_ten() {
        let c10 = church_numeral(10);
        let n = church_to_nat(&c10);
        assert_eq!(n, Some(10), "church_numeral(10) should decode to 10");
    }

    // ── Scott encoding ────────────────────────────────────────────────────

    #[test]
    fn test_scott_zero() {
        let scott_zero = scott_encoding_nat();
        // Applying scott_zero to Z and S should give Z
        let z = DttTerm::Zero;
        let s = DttTerm::Lambda {
            ty: Box::new(DttTerm::Nat),
            body: Box::new(DttTerm::Succ(Box::new(DttTerm::Var(0)))),
        };
        let app1 = DttTerm::App {
            func: Box::new(scott_zero),
            arg: Box::new(z.clone()),
        };
        let app2 = DttTerm::App {
            func: Box::new(app1),
            arg: Box::new(s),
        };
        let nf = normalize_beta(&app2);
        assert_eq!(nf, z, "Scott zero applied to Z S should give Z");
    }

    #[test]
    fn test_scott_nat_value_zero() {
        let zero = scott_encoding_nat();
        assert_eq!(scott_nat_value(&zero), Some(0));
    }

    #[test]
    fn test_scott_nat_value_one() {
        let zero = scott_encoding_nat();
        let one = scott_succ(&zero);
        assert_eq!(scott_nat_value(&one), Some(1));
    }

    #[test]
    fn test_scott_nat_value_two() {
        let zero = scott_encoding_nat();
        let one = scott_succ(&zero);
        let two = scott_succ(&one);
        assert_eq!(scott_nat_value(&two), Some(2));
    }

    // ── Pure Type Systems ─────────────────────────────────────────────────

    #[test]
    fn test_coc_axiom_prop_type() {
        let coc = calculus_of_constructions();
        let ctx = DttContext::empty();
        let prop_sort = DttTerm::Sort(DttSort::Prop);
        let result = check_pure_type_system(&coc, &ctx, &prop_sort);
        assert_eq!(result, Some(DttSort::Type), "Prop : Type in CoC");
    }

    #[test]
    fn test_coc_pi_prop_prop() {
        let coc = calculus_of_constructions();
        let ctx = DttContext::empty();
        // Π(x:Prop).Prop
        // Prop has type Type (by axiom Prop:Type), so s1=Type, s2=Type
        // rule(Type,Type)=Type → result is Type
        let pi = DttTerm::Pi {
            domain: Box::new(DttTerm::Sort(DttSort::Prop)),
            body: Box::new(DttTerm::Sort(DttSort::Prop)),
        };
        let result = check_pure_type_system(&coc, &ctx, &pi);
        assert_eq!(result, Some(DttSort::Type));
    }

    #[test]
    fn test_coc_pi_type_type() {
        let coc = calculus_of_constructions();
        let ctx = DttContext::empty();
        // Π(x:Type).Type : Kind  (because Type : Kind by axiom, rule(Kind,Kind)=Kind)
        let pi = DttTerm::Pi {
            domain: Box::new(DttTerm::Sort(DttSort::Type)),
            body: Box::new(DttTerm::Sort(DttSort::Type)),
        };
        let result = check_pure_type_system(&coc, &ctx, &pi);
        assert_eq!(result, Some(DttSort::Kind));
    }

    #[test]
    fn test_system_f_rules() {
        let sf = system_f();
        // System F has (Type,Prop) but NOT (Type,Type)
        assert!(sf.rule_sort(&DttSort::Type, &DttSort::Prop).is_some());
        assert!(sf.rule_sort(&DttSort::Type, &DttSort::Type).is_none());
    }

    #[test]
    fn test_system_fomega_rules() {
        let sfw = system_fomega();
        // System Fω has (Prop,Prop), (Type,Prop), (Type,Type)
        assert!(sfw.rule_sort(&DttSort::Prop, &DttSort::Prop).is_some());
        assert!(sfw.rule_sort(&DttSort::Type, &DttSort::Prop).is_some());
        assert!(sfw.rule_sort(&DttSort::Type, &DttSort::Type).is_some());
        // But NOT (Prop,Type)
        assert!(sfw.rule_sort(&DttSort::Prop, &DttSort::Type).is_none());
    }

    #[test]
    fn test_barendregt_cube_has_eight_systems() {
        let cube = barendregt_cube();
        assert_eq!(cube.len(), 8, "The λ-cube has exactly 8 systems");
    }

    #[test]
    fn test_barendregt_cube_contains_coc() {
        let cube = barendregt_cube();
        let has_coc = cube.iter().any(|(name, _)| *name == "λC");
        assert!(has_coc, "λ-cube should contain λC (CoC)");
    }

    #[test]
    fn test_barendregt_cube_contains_simple_types() {
        let cube = barendregt_cube();
        let has_stlc = cube.iter().any(|(name, _)| *name == "λ→");
        assert!(has_stlc, "λ-cube should contain λ→ (simple types)");
    }

    #[test]
    fn test_dtt_sort_predicates() {
        assert!(DttSort::Prop.is_prop());
        assert!(!DttSort::Prop.is_type());
        assert!(DttSort::Type.is_type());
        assert!(DttSort::Kind.is_kind());
    }

    #[test]
    fn test_dtt_context_lookup() {
        let ctx = DttContext::empty()
            .extend("x", DttTerm::Nat)
            .extend("y", DttTerm::Sort(DttSort::Type));
        // Var(0) = innermost = y
        assert_eq!(ctx.lookup(0), Some(&DttTerm::Sort(DttSort::Type)));
        // Var(1) = x
        assert_eq!(ctx.lookup(1), Some(&DttTerm::Nat));
        // Var(2) = out of range
        assert_eq!(ctx.lookup(2), None);
    }

    #[test]
    fn test_dtt_term_size() {
        assert_eq!(DttTerm::Zero.size(), 1);
        assert_eq!(DttTerm::Succ(Box::new(DttTerm::Zero)).size(), 2);
        let app = DttTerm::App {
            func: Box::new(DttTerm::Zero),
            arg: Box::new(DttTerm::Zero),
        };
        assert_eq!(app.size(), 3);
    }

    #[test]
    fn test_dtt_term_is_normal() {
        assert!(DttTerm::Zero.is_normal());
        assert!(DttTerm::Nat.is_normal());
        // App with Lambda head is NOT normal
        let lam = DttTerm::Lambda {
            ty: Box::new(DttTerm::Sort(DttSort::Type)),
            body: Box::new(DttTerm::Var(0)),
        };
        let app = DttTerm::App {
            func: Box::new(lam),
            arg: Box::new(DttTerm::Zero),
        };
        assert!(!app.is_normal());
    }

    #[test]
    fn test_nat_sort_check() {
        let coc = calculus_of_constructions();
        let ctx = DttContext::empty();
        let nat = check_pure_type_system(&coc, &ctx, &DttTerm::Nat);
        assert_eq!(nat, Some(DttSort::Type));
    }

    #[test]
    fn test_iota_reduction_elim_zero() {
        // Nat.elim P base step Zero → base
        let motive = DttTerm::Lambda {
            ty: Box::new(DttTerm::Nat),
            body: Box::new(DttTerm::Nat),
        };
        let base = DttTerm::Const("base_val".to_string());
        let step = DttTerm::Const("step_fn".to_string());
        let elim = DttTerm::Elim {
            motive: Box::new(motive),
            base: Box::new(base.clone()),
            step: Box::new(step),
            target: Box::new(DttTerm::Zero),
        };
        assert_eq!(normalize_beta(&elim), base);
    }

    #[test]
    fn test_lambda_p2_rules() {
        let lp2 = lambda_p2();
        assert!(lp2.rule_sort(&DttSort::Prop, &DttSort::Prop).is_some());
        assert!(lp2.rule_sort(&DttSort::Prop, &DttSort::Type).is_some());
        assert!(lp2.rule_sort(&DttSort::Type, &DttSort::Prop).is_some());
        assert!(lp2.rule_sort(&DttSort::Type, &DttSort::Type).is_none());
    }
}
