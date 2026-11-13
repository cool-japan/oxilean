import Lean
open Lean Meta Elab Term Command

/-!
# `CtorListLie` — three inductive bundles that lie about their constructors

Source of `CtorListLie.ndjson`. That file is NOT what lean4export writes for
this module: it is the export **with three records edited by hand** (the
commands are in `README.md` next to this file). This is a NEGATIVE fixture.

The three types each have TWO constructors:

* `CtorListLie.Two` — `mk (n : Nat)` and `nil`; not recursive, no index.
* `CtorListLie.L2`  — `cons (head : Nat) (tail : L2)` and `nil`; recursive.
* `CtorListLie.TI : Nat → Type` — `a (b : Bool) : TI 0` and `b : TI 1`; an
  indexed family.

The four declarations below each read a field out of one of them with a raw
projection (`Expr.proj`), which Lean's kernel refuses for an inductive type
that does not have exactly one constructor — the commands print what the
kernel says — so they are added with `debug.skipKernelTC` and then exported:

* `two_proj : Two → Nat := fun t => t.0`
* `two_eta  : ∀ t : Two, t = Two.mk t.0 := fun t => rfl` — and the statement
  is FALSE of `Two`: `Two.nil` is not `Two.mk` of anything.
* `l2_proj  : L2 → Nat := fun l => l.0`
* `ti_proj  : TI 0 → Bool := fun t => t.0`

The hand edit: in each of the three `inductive` records the type's `ctors`
list is cut down to its first constructor (the second constructor's own
record is left in the bundle). A file like that describes no Lean
environment — the kernel builds the constructor list of an inductive type
itself — and a checker must refuse each bundle AS A WHOLE, and with it every
declaration that names any part of it.

`oxilean-verify` through 0.1.4 refused the bundles and left each one half
installed: the type with ONE constructor and that constructor stayed in the
environment. `two_proj` and `two_eta` then verified against the one-constructor
`Two` (structure eta included), and the run reported
`5 verified · 0 unsupported · 5 rejected`. With projections out of recursive
and indexed one-constructor types admitted (0.1.5), `l2_proj` and `ti_proj`
would have verified the same way.
-/

set_option autoImplicit false
-- Keep `TI`'s index an index (Lean would otherwise leave it one here anyway:
-- no constructor argument is the index of every result type).
set_option inductive.autoPromoteIndices false

namespace CtorListLie

inductive Two : Type where
  | mk (n : Nat) : Two
  | nil : Two

inductive L2 : Type where
  | cons (head : Nat) (tail : L2) : L2
  | nil : L2

inductive TI : Nat → Type where
  | a (b : Bool) : TI Nat.zero
  | b : TI (Nat.succ Nat.zero)

end CtorListLie

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

-- The elaborator will not write `t.0` for a type with two constructors, so
-- the terms are built with `Expr.proj` directly.
run_cmd liftCoreM do
  let nat := Lean.mkConst ``Nat
  let bool := Lean.mkConst ``Bool
  let two := Lean.mkConst ``CtorListLie.Two
  let l2 := Lean.mkConst ``CtorListLie.L2
  let ti0 := mkApp (Lean.mkConst ``CtorListLie.TI) (Lean.mkConst ``Nat.zero)

  -- two_proj : Two → Nat := fun t => t.0
  addUnchecked false `CtorListLie.two_proj
    (mkForall `t .default two nat)
    (mkLambda `t .default two (mkProj ``CtorListLie.Two 0 (mkBVar 0)))

  -- two_eta : ∀ t : Two, t = Two.mk t.0 := fun t => Eq.refl t
  addUnchecked true `CtorListLie.two_eta
    (mkForall `t .default two
      (mkApp3 (Lean.mkConst ``Eq [Level.one]) two (mkBVar 0)
        (mkApp (Lean.mkConst ``CtorListLie.Two.mk) (mkProj ``CtorListLie.Two 0 (mkBVar 0)))))
    (mkLambda `t .default two (mkApp2 (Lean.mkConst ``Eq.refl [Level.one]) two (mkBVar 0)))

  -- l2_proj : L2 → Nat := fun l => l.0
  addUnchecked false `CtorListLie.l2_proj
    (mkForall `l .default l2 nat)
    (mkLambda `l .default l2 (mkProj ``CtorListLie.L2 0 (mkBVar 0)))

  -- ti_proj : TI 0 → Bool := fun t => t.0
  addUnchecked false `CtorListLie.ti_proj
    (mkForall `t .default ti0 bool)
    (mkLambda `t .default ti0 (mkProj ``CtorListLie.TI 0 (mkBVar 0)))
