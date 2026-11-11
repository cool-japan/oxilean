import Lean
open Lean Elab Command

/-!
# `DeepTerm` — a term three hundred levels deep

Source of `DeepTerm.ndjson`; the export command is in `README.md` next to this
file.

`tower` is `Nat.succ` applied three hundred times to `Nat.zero`, written out
as a term — the shape of a long list or array literal, where every element is
one level further down. A kernel walks a term by recursion: to infer the type
of `tower` it goes down all three hundred levels, and to check `tower_eq` —
that the same tower over `z`, a definition that unfolds to zero, equals it —
it compares the two level by level and only finds out at the bottom that it
has to unfold `z`. Every level is a frame of the checker's native stack, and
so is every level of the term when it is released.

How deep a term may be is therefore a property of the stack the checker runs
on, as the nesting of reductions is (`NestedReduction.lean`). `oxilean-verify`
runs on a thread with a stack of its own choosing. A WebAssembly build runs on
its host's, about a megabyte in a browser; there the checker is given a limit
on the depth of its terms (512 in `oxilean-verify-wasm`), a declaration with a
deeper term is not even read into memory, and it is reported as over a
resource limit:
UNSUPPORTED, by name. Three hundred levels are within that limit, so the
sound declarations of this file verify in a browser too; the tests give the
checker a lower limit to see the refusal.

`bad` is a NEGATIVE control, a declaration Lean's own kernel REJECTS (the
command below prints what the kernel says; it is added with
`debug.skipKernelTC` so that it can be exported): the tower over `z` equated
with a tower one level TALLER, which is false, and which takes the same three
hundred levels to find out. A checker that walks the term to the bottom must
REJECT it; one that cannot — because the term is too deep for it, or because
telling the two towers apart takes a reduction nested deeper than it allows,
as in a browser — must say UNSUPPORTED, never verified. A checker that
reports `bad` as verified is unsound.

The terms are built by a small command rather than written out: the
elaborator would refuse to recurse three hundred applications deep long
before the kernel is asked.
-/

namespace DeepTerm

/-- A definition that unfolds to zero. -/
def z : Nat := Nat.zero

end DeepTerm

/-- `Nat.succ` applied `k` times to `base`. -/
def succTower (k : Nat) (base : Expr) : Expr := Id.run do
  let mut e := base
  for _ in [0:k] do
    e := mkApp (mkConst ``Nat.succ) e
  return e

/-- `#deep_terms k` adds, each checked by Lean's kernel,
`DeepTerm.tower : Nat := succ^k zero` and
`DeepTerm.tower_eq : succ^k z = succ^k zero := Eq.refl (succ^k zero)`;
then `DeepTerm.bad : succ^k z = succ^(k+1) zero := Eq.refl (succ^k zero)`
WITHOUT kernel checking, after printing what Lean's own kernel says about it. -/
elab "#deep_terms " k:num : command => do
  let k := k.getNat
  let nat := mkConst ``Nat
  let zero := mkConst ``Nat.zero
  let tower := succTower k zero
  let towerZ := succTower k (mkConst ``DeepTerm.z)
  let eq (a b : Expr) : Expr := mkApp3 (mkConst ``Eq [Level.one]) nat a b
  let refl := mkApp2 (mkConst ``Eq.refl [Level.one]) nat tower
  liftCoreM <| addDecl <| Declaration.defnDecl {
    name := `DeepTerm.tower, levelParams := [], type := nat, value := tower,
    hints := .abbrev, safety := .safe }
  liftCoreM <| addDecl <| Declaration.thmDecl {
    name := `DeepTerm.tower_eq, levelParams := [], type := eq towerZ tower, value := refl }
  let bad := Declaration.thmDecl {
    name := `DeepTerm.bad, levelParams := [],
    type := eq towerZ (succTower (k + 1) zero), value := refl }
  match (← getEnv).addDeclCore 0 bad none with
  | .ok _ => logInfo m!"LEAN KERNEL: ACCEPTED DeepTerm.bad"
  | .error _ => logInfo m!"LEAN KERNEL: REJECTED DeepTerm.bad"
  liftCoreM <| withOptions (fun o => debug.skipKernelTC.set o true) (addDecl bad)

#deep_terms 300
