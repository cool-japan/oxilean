import Lean
open Lean Meta Elab Term Command

/-!
# `KGuardCollision` — a theorem Lean's kernel REJECTS

Source of `KGuardCollision.ndjson`; the export command is in `README.md` next
to this file. This is a NEGATIVE fixture: `KGuardCollision.bad` is not a
theorem. Lean's own kernel refuses it (the command below prints what the
kernel says), so it is added with `debug.skipKernelTC` and only then exported.
A checker that reports it as verified is unsound.

The statement is `Eq.rec m h = m` for a hypothesis
`h : (fun y1 y2 y3 => c) = (fun y1 y2 y3 => y3)` and a constant motive, and the
"proof" is `Eq.refl m`. That is a proof only if `Eq.rec m h` reduces to `m`,
which needs K-like reduction on the VARIABLE `h` — legal only when the two
endpoints of `h` are definitionally equal, and `fun y1 y2 y3 => c` and
`fun y1 y2 y3 => y3` are not.

`oxilean-verify` checks that side condition in a nested checker seeded with the
surrounding local context. Before the fix that nested checker opened the
binders `y1 y2 y3` with free-variable ids that restarted from the base of the
range, so `y3` received the id the surrounding checker had given `c`: both
endpoints opened to the same variable, the side condition passed, K fired, and
this declaration was reported VERIFIED. The binder count is what lines the two
ids up; the shape is otherwise arbitrary.
-/

namespace KGuardCollision
axiom A : Type
end KGuardCollision

/-- Add `n : ty := val` as a theorem WITHOUT kernel checking, after printing
what Lean's own kernel says about it. -/
elab "#add_unchecked " n:ident " : " ty:term " := " val:term : command => do
  liftTermElabM do
    let ty ← elabType ty
    let val ← elabTerm val none
    synthesizeSyntheticMVarsNoPostponing
    let ty ← instantiateMVars ty
    let val ← instantiateMVars val
    let decl := Declaration.thmDecl { name := n.getId, levelParams := [], type := ty, value := val }
    match (← getEnv).addDeclCore 0 decl none with
    | .ok _ => logInfo m!"LEAN KERNEL: ACCEPTED {n.getId}"
    | .error _ => logInfo m!"LEAN KERNEL: REJECTED {n.getId}"
    withOptions (fun o => debug.skipKernelTC.set o true) (addDecl decl)

set_option linter.unusedVariables false

open KGuardCollision in
#add_unchecked KGuardCollision.bad :
  ∀ (c : A) (h : (fun (y1 y2 y3 : A) => c) = (fun (y1 y2 y3 : A) => y3)) (m : Nat),
    @Eq.rec (A → A → A → A) (fun (y1 y2 y3 : A) => c) (fun _ _ => Nat) m
      (fun (y1 y2 y3 : A) => y3) h = m
  := fun (c : A) (h : (fun (y1 y2 y3 : A) => c) = (fun (y1 y2 y3 : A) => y3)) (m : Nat) =>
    @Eq.refl Nat m
