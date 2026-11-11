/-!
# `QuotPartialApp` — a partial application exposed by head reduction

Source of `QuotPartialApp.ndjson`; the export command is in `README.md` next to
this file. Self-contained: it imports nothing beyond the Lean prelude.

Every theorem below is `rfl`. Each needs the kernel to reduce an application
whose head only *becomes* a recursor-like operation (`Quot.lift`, a recursor)
after a head step — a definition unfolding, a structure projection — with the
major premise among the arguments of the *enclosing* application.

Mathlib's quotient API is written in this point-free style (`Quot.map`,
`Quotient.map'`), and its instances on quotients store such partial
applications in structure fields. The declaration that surfaced the kernel bug
this fixture pins is Mathlib's `AddCon.addGroup._proof_1`.

`succQ_mk`, `neg_mk` and `pred_succ` were rejected by `oxilean-verify` before
the fix; `succQ'_mk` and `add_mk` are controls that verified before and after.
-/
namespace QuotPartialApp

/-- Mathlib's `Quot.map`: point-free, so the body is `Quot.lift` one argument
short. -/
protected def Quot.map {α β : Sort _} {ra : α → α → Prop} {rb : β → β → Prop} (f : α → β)
    (h : ∀ ⦃a b : α⦄, ra a b → rb (f a) (f b)) : Quot ra → Quot rb :=
  Quot.lift (fun x => Quot.mk rb (f x)) fun _ _ hra => Quot.sound (h hra)

/-- Mathlib's `Quotient.map'`: point-free over `Quot.map`. -/
protected def Quotient.map' {α β : Sort _} {sa : Setoid α} {sb : Setoid β} (f : α → β)
    (h : ∀ ⦃a b : α⦄, sa.r a b → sb.r (f a) (f b)) : Quotient sa → Quotient sb :=
  QuotPartialApp.Quot.map f h

/-- Mathlib's `Quotient.map₂`: point-free over core's `Quotient.lift₂`. -/
protected def Quotient.map₂ {α β γ : Sort _} {sa : Setoid α} {sb : Setoid β} {sc : Setoid γ}
    (f : α → β → γ)
    (h : ∀ ⦃a₁ a₂ : α⦄, sa.r a₁ a₂ → ∀ ⦃b₁ b₂ : β⦄, sb.r b₁ b₂ → sc.r (f a₁ b₁) (f a₂ b₂)) :
    Quotient sa → Quotient sb → Quotient sc :=
  Quotient.lift₂ (fun x y => Quotient.mk sc (f x y)) fun _ _ _ _ h₁ h₂ => Quot.sound (h h₁ h₂)

/-- The setoid on `Nat` that identifies everything — enough for `rfl` tests. -/
def trivSetoid : Setoid Nat where
  r _ _ := True
  iseqv := ⟨fun _ => trivial, fun _ => trivial, fun _ _ => trivial⟩

/-- `Nat` modulo everything. -/
def Q : Type := Quotient trivSetoid

/-- The class of `a`. -/
def Q.mk (a : Nat) : Q := Quotient.mk trivSetoid a

/-- (1) A constant whose VALUE is a partial application of `Quot.lift`. -/
def succQ : Q → Q :=
  Quot.lift (fun x => Q.mk (Nat.succ x)) fun _ _ _ => Quot.sound trivial

theorem succQ_mk (a : Nat) : succQ (Q.mk a) = Q.mk (Nat.succ a) := rfl

/-- (2) An instance FIELD that is a partial application — the shape of
Mathlib's `AddCon.hasNeg` (`⟨Quotient.map' Neg.neg fun _ _ => c.neg⟩`). -/
instance instNegQ : Neg Q :=
  ⟨QuotPartialApp.Quotient.map' Nat.succ fun _ _ _ => trivial⟩

theorem neg_mk (a : Nat) : -Q.mk a = Q.mk (Nat.succ a) := rfl

/-- (3) The same shape with a recursor in place of `Quot.lift`. -/
noncomputable def pred : Nat → Nat := @Nat.rec (fun _ => Nat) 0 (fun n _ => n)

theorem pred_succ (n : Nat) : pred (Nat.succ n) = n := rfl

/-- (4) Control: `Quot.lift` applied to its major premise in the definition. -/
def succQ' (q : Q) : Q :=
  Quot.lift (fun x => Q.mk (Nat.succ x)) (fun _ _ _ => Quot.sound trivial) q

theorem succQ'_mk (a : Nat) : succQ' (Q.mk a) = Q.mk (Nat.succ a) := rfl

/-- (5) Control: a binary operation, the shape of Mathlib's `AddCon.hasAdd`
(`⟨Quotient.map₂ (· + ·) fun _ _ h1 _ _ h2 => c.add h1 h2⟩`). The partial
application in the field is of `Quotient.lift₂`, a definition, so unfolding
it yields a lambda that takes the outer arguments. -/
instance instAddQ : Add Q :=
  ⟨QuotPartialApp.Quotient.map₂ Nat.add fun _ _ _ _ _ _ => trivial⟩

theorem add_mk (a b : Nat) : Q.mk a + Q.mk b = Q.mk (Nat.add a b) := rfl

end QuotPartialApp
