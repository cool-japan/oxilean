import Lean
open Lean Meta Elab Term Command

/-!
# `ProjOutOfProp` — two declarations Lean's kernel REJECTS

Source of `ProjOutOfProp.ndjson`; the export command is in `README.md` next to
this file. This is a NEGATIVE fixture: neither declaration is well typed.
Lean's own kernel refuses both (the commands below print what the kernel
says), so they are added with `debug.skipKernelTC` and only then exported.
A checker that reports either as verified is unsound.

Both rest on one term: `h.1` for `h : ∃ n : Nat, True` — the projection
`Expr.proj Exists 0 h`, which reads the WITNESS out of a PROOF. `Exists` is a
proposition, so any two of its proofs are definitionally equal (proof
irrelevance); a projection that returns data from one is a function that is
constant by definition and still computes on constructors.

* `ProjOutOfProp.wit` is that function, as a definition:
  `fun (h : ∃ n : Nat, True) => h.1 : (∃ n : Nat, True) → Nat`.
* `ProjOutOfProp.false_of_proj` is a closed proof of `False` from it, with no
  axiom and no other declaration of this file: for
  `agree : ∀ h1 h2 : ∃ n : Nat, True, h1.1 = h2.1` — proved by `Eq.refl h1.1`,
  which needs `h1.1` and `h2.1` definitionally equal, and they are if `h1`
  and `h2` are — the instance `agree ⟨0, _⟩ ⟨1, _⟩` is a proof of `0 = 1`
  once the two projections reduce on their constructors, and `Eq.rec` along
  it carries `True.intro` to `False`.

Lean's kernel rejects the projection itself (`infer_proj`): when the
structure's type is a proposition, the projected field's type must be a
proposition, and so must the type of every earlier field a later field's type
depends on. `oxilean-verify` through 0.1.4 had no such condition and reported
both declarations VERIFIED (`7 verified · 0 unsupported · 0 rejected`, exit 0).
-/

/-- Add `n : ty := val` WITHOUT kernel checking, after printing what Lean's
own kernel says about it. -/
def addUnchecked (isThm : Bool) (n : Name) (ty val : Expr) : CoreM Unit := do
  let decl :=
    if isThm then
      Declaration.thmDecl { name := n, levelParams := [], type := ty, value := val }
    else
      Declaration.defnDecl
        { name := n, levelParams := [], type := ty, value := val,
          hints := .abbrev, safety := .safe }
  match (← getEnv).addDeclCore 0 decl none with
  | .ok _ => logInfo m!"LEAN KERNEL: ACCEPTED {n}"
  | .error _ => logInfo m!"LEAN KERNEL: REJECTED {n}"
  withOptions (fun o => debug.skipKernelTC.set o true) (addDecl decl)

/-- Elaborate a closed term. -/
def closed (stx : TermElabM Syntax) : TermElabM Expr := do
  let e ← elabTerm (← stx) none
  synthesizeSyntheticMVarsNoPostponing
  let e ← instantiateMVars e
  if e.hasMVar then throwError "metavariables left in {e}"
  return e

set_option linter.unusedVariables false

-- The elaborator will not write `h.1` for an `Exists` (it is not a
-- structure), so the terms are built with `Expr.proj` directly.
run_cmd liftTermElabM do
  let nat := Lean.mkConst ``Nat
  let ex ← closed `(∃ n : Nat, True)
  let fst (h : Expr) : Expr := mkProj ``Exists 0 h

  -- wit : (∃ n : Nat, True) → Nat := fun h => h.1
  addUnchecked false `ProjOutOfProp.wit
    (mkForall `h .default ex nat)
    (mkLambda `h .default ex (fst (mkBVar 0)))

  -- agreeTy := ∀ h1 h2 : ∃ n : Nat, True, h1.1 = h2.1
  let agreeTy := mkForall `h1 .default ex (mkForall `h2 .default ex
    (mkApp3 (Lean.mkConst ``Eq [Level.one]) nat (fst (mkBVar 1)) (fst (mkBVar 0))))
  -- agree := fun h1 h2 => Eq.refl h1.1
  let agree := mkLambda `h1 .default ex (mkLambda `h2 .default ex
    (mkApp2 (Lean.mkConst ``Eq.refl [Level.one]) nat (fst (mkBVar 1))))
  let w0 ← closed `((⟨Nat.zero, True.intro⟩ : ∃ n : Nat, True))
  let w1 ← closed `((⟨Nat.succ Nat.zero, True.intro⟩ : ∃ n : Nat, True))
  -- motive := fun (n : Nat) (_ : w0.1 = n) => Nat.rec True (fun _ _ => False) n
  let isZero ← closed `(fun (n : Nat) => @Nat.rec (fun _ => Prop) True (fun _ _ => False) n)
  let motive := mkLambda `n .default nat (mkLambda `e .default
    (mkApp3 (Lean.mkConst ``Eq [Level.one]) nat (fst w0) (mkBVar 0))
    (mkApp isZero (mkBVar 1)))
  -- under `agree`: Eq.rec (motive := motive) True.intro (agree w0 w1) : motive w1.1 _
  let body := mkApp6 (Lean.mkConst ``Eq.rec [Level.zero, Level.one])
    nat (fst w0) motive (Lean.mkConst ``True.intro) (fst w1) (mkApp2 (mkBVar 0) w0 w1)
  -- false_of_proj : False := (fun (agree : agreeTy) => body) agree
  addUnchecked true `ProjOutOfProp.false_of_proj
    (Lean.mkConst ``False)
    (mkApp (mkLambda `agree .default agreeTy body) agree)
