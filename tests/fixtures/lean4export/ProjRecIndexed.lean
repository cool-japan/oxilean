/-!
# Projections out of recursive structures and out of an indexed family

Lean's kernel types a projection `Expr.proj S i e` for every inductive type
`S` with exactly ONE constructor. `S` may be recursive — an ordinary
recursive `structure` — and it may have indices. Structure eta and unit-like
equality are narrower: they hold only when `S` is also non-recursive and has
no indices. A checker that asks the eta condition of a projection rejects
every declaration below that mentions one.

Everything in this file is ordinary Lean: the elaborator wrote every term and
Lean's kernel checked every declaration (nothing is added with
`debug.skipKernelTC`). The export is a POSITIVE fixture: every declaration
must verify.
-/
set_option autoImplicit false

namespace ProjRecIndexed

/-- A recursive `structure` whose recursion is nested through `List` (a rose
tree). `Tree.label` and `Tree.kids` are the projections Lean generates:
`fun self => self.1` and `fun self => self.2`, each an `Expr.proj`. -/
structure Tree where
  label : Nat
  kids : List Tree

/-- A recursive `structure` whose recursion is direct (under a function: a
lazy stream). -/
structure Stream where
  head : Nat
  tail : Unit → Stream

/-- A one-constructor family with a parameter and an INDEX: `tag` follows
`val` among the constructor's arguments, so Lean cannot turn it into a
parameter (`Tagged : Type → Nat → Type`, one parameter). -/
inductive Tagged (α : Type) : Nat → Type where
  | mk (val : α) (tag : Nat) : Tagged α tag

/-- The `inductive` command generates no projections; the anonymous
projection notation elaborates to the same `Expr.proj Tagged 0 t`. -/
def Tagged.val {α : Type} {n : Nat} (t : Tagged α n) : α := t.1

/-- `Expr.proj Tagged 1 t`: the field, not the index it is equal to. -/
def Tagged.tag {α : Type} {n : Nat} (t : Tagged α n) : Nat := t.2

/-- The label of the first child, or the node's own: projections of a
variable, of the result of a match, and through a nested occurrence. -/
def Tree.firstLabel (t : Tree) : Nat :=
  match t.kids with
  | [] => t.label
  | k :: _ => k.label

/-- The projections are typed on variables (`s.tail`, `t.val`), reduce on
constructor applications, and compute through definitions. -/
theorem proj_mk (l : Nat) (ks : List Tree) (s : Stream) (b : Bool) (n : Nat) (t : Tagged Bool n) :
    (Tree.mk l ks).kids = ks
      ∧ (Tree.mk l []).firstLabel = l
      ∧ (Stream.mk l s.tail).head = l
      ∧ (s.tail ()).head = (s.tail ()).head
      ∧ (Tagged.mk b n).val = b
      ∧ (Tagged.mk b n).tag = n
      ∧ t.val = t.val :=
  ⟨rfl, rfl, rfl, rfl, rfl, rfl, rfl⟩

end ProjRecIndexed
