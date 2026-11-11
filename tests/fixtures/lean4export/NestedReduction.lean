import Lean
open Lean Meta Elab Term Command

/-!
# `NestedReduction` — a reduction nested hundreds deep

Source of `NestedReduction.ndjson`; the export command is in `README.md` next
to this file.

`count n acc` counts `n` down by structural recursion. Evaluating `count n 0`
in a kernel is a reduction nested about `n` deep: each step of the recursion
is found inside the previous one, and each level holds a frame of the
checker's native stack until the levels inside it are done.

How deep that may go is a property of the stack the checker runs on, not of
the proof. `oxilean-verify` runs on a thread with a stack of its own choosing
(512 MiB by default), where `deep` — four hundred levels — is nothing. A
WebAssembly build runs on its host's stack, about a megabyte in a browser,
which it can neither enlarge nor measure; there the kernel is told to stop
nesting reductions at a fixed depth (256 in `oxilean-verify-wasm`) and `deep`
is reported as over a resource limit: UNSUPPORTED, by name. `shallow` — forty
levels — is verified everywhere.

`bad` is a NEGATIVE control, a declaration Lean's own kernel REJECTS (the
command below prints what the kernel says; it is added with
`debug.skipKernelTC` so that it can be exported): `count 400 0 = 401`, which
is false, and which takes the same four hundred levels to find out. A checker
with the room to evaluate it must REJECT it. A checker without the room
cannot tell, and must say so — unsupported — exactly as for `deep`: stopping
a reduction early must never turn into a verdict, in either direction. A
checker that reports `bad` as verified is unsound.
-/

namespace NestedReduction

/-- Counts `n` down, adding one to `acc` at every step. -/
def count : Nat → Nat → Nat
  | 0, acc => acc
  | n + 1, acc => count n (acc + 1)

/-- Four hundred levels of nested reduction. -/
theorem deep : count 400 0 = 400 := by decide +kernel

/-- Control: forty levels. -/
theorem shallow : count 40 0 = 40 := by decide +kernel

end NestedReduction

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

open NestedReduction in
#add_unchecked NestedReduction.bad : count 400 0 = 401 := @Eq.refl Nat (count 400 0)
