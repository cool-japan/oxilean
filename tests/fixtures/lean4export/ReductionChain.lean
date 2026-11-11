/-!
# `ReductionChain` — a long head-reduction chain over a small term

Source of `ReductionChain.ndjson`; the export command is in `README.md` next to
this file. Self-contained: it imports nothing beyond the Lean prelude.

`chain n` runs `Nat.rec` with a step that returns its induction hypothesis
unchanged, so reducing `chain n` is ONE head-reduction chain of a few steps per
iteration over a term that never grows. Both theorems are proved by kernel
evaluation (`decide +kernel`), and Lean's kernel checks them at once.

`chain_20000` needs tens of thousands of head steps. `oxilean-verify` used to
stop any chain after 10,000 steps and hand back the unfinished term as if it
were a normal form; the comparison that followed failed, and this valid theorem
was reported REJECTED — although no resource budget had been exceeded. A
declaration that fails that way is now checked again with the cap lifted, under
the per-declaration budget, which this chain is far below. `chain_1000` is the
control: short enough to have verified before the fix too.
-/
namespace ReductionChain

/-- `Nat.rec` with a step that returns its induction hypothesis unchanged. -/
noncomputable def chain (n : Nat) : Bool :=
  @Nat.rec (fun _ => Bool) true (fun _ b => b) n

/-- Control: a chain short enough for the old step cap. -/
theorem chain_1000 : chain 1000 = true := by decide +kernel

/-- A chain several times longer than the old step cap. -/
theorem chain_20000 : chain 20000 = true := by decide +kernel

end ReductionChain
