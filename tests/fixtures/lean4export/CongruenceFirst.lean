import Lean
open Lean Meta Elab Term Command

/-!
# `CongruenceFirst` — `f a = f b` is decided by comparing `a` and `b`

Source of `CongruenceFirst.ndjson`; the export command is in `README.md` next
to this file.

Two applications of the SAME definition to definitionally equal arguments are
definitionally equal, whatever the definition unfolds to. Lean's kernel uses
that before it unfolds anything (the `f a =?= f b` step of its lazy delta
reduction), and for some equalities it is the only way through in reasonable
time:

* `slow fuel n` counts `fuel` down and, at every step, asks whether `n` is
  zero. When `n` is a constant the kernel cannot evaluate (`k`, an `opaque`),
  each step is stuck on that question, with the next step inside one of its
  branches. `same_function_equal_args` compares two such terms for a billion
  steps of fuel. Their arguments are equal (`k + 0` reduces to `k`), so the
  answer is immediate — but a checker that unfolds `slow` on both sides first
  and compares what comes out walks the recursion one unfolding at a time.
* `mod_stuck_divisor` is the same situation in Lean core's own terms: `x % y`
  with `y = 2 ^ System.Platform.numBits`, which no kernel can evaluate. It is
  the comparison inside `UInt64.toUSize_ofNatClamp_of_le` and its relatives.

`oxilean-verify` reduced both sides first, and ran out of its per-declaration
budget on both theorems — as it did on the declarations of Lean core `Init`
that compare such terms (conversions between `USize`, `UInt64` and their
signed counterparts among them). It now compares the arguments of two
applications of one definition before reducing either.

`bad` is a NEGATIVE control, a declaration Lean's own kernel REJECTS (the
command below prints what the kernel says; it is added with
`debug.skipKernelTC` so that it can be exported): the same function applied to
arguments that are NOT equal. Comparing arguments first must not make it pass.
A checker that reports it as verified is unsound.
-/

namespace CongruenceFirst

/-- A natural number the kernel cannot evaluate. -/
opaque k : Nat

/-- Counts `fuel` down, asking at every step whether `n` is zero. -/
def slow : Nat → Nat → Nat
  | 0, n => n
  | fuel + 1, n => if n = 0 then 0 else slow fuel n

/-- The same function on equal arguments, for a billion steps of fuel. -/
theorem same_function_equal_args : slow 1000000000 (k + 0) = slow 1000000000 k := rfl

/-- The shape that occurs in Lean core: a remainder by a divisor that is stuck. -/
theorem mod_stuck_divisor :
    18446744073709551615 % USize.size = 18446744073709551615 % 2 ^ System.Platform.numBits :=
  rfl

/-- Control: short enough to check by unfolding as well. -/
theorem short_fuel : slow 3 (k + 0) = slow 3 k := rfl

end CongruenceFirst

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

open CongruenceFirst in
#add_unchecked CongruenceFirst.bad : slow 5 k = slow 6 k := @Eq.refl Nat (slow 5 k)
