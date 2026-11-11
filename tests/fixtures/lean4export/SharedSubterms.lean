/-!
# `SharedSubterms` — small as a shared term, astronomically large as a tree

Source of `SharedSubterms.ndjson`; the export command is in `README.md` next to
this file. Self-contained: it imports nothing beyond the Lean prelude.

`tower` doubles sixty times, and every `let` uses the one before it twice. As a
term with shared subterms it is a few hundred nodes; written out as a tree —
every use of `xₙ` replaced by its value — it is 2^60 additions. Lean's kernel
checks both declarations at once: it substitutes a `let` value as one shared
term and remembers what it has already typed and reduced.

`oxilean-verify` used to charge its per-declaration budget for the TREE: each
substitution that shared an untouched subterm was billed for the subterm's size
as a tree, so the definition ran out of budget around the 25th doubling and was
reported `unsupported`, taking the theorem with it — although next to no work
had been done. A substitution is now charged for the subterms it reaches,
shared subterms stay shared in what it builds, and types and reductions are
remembered by term, so both check in well under a millisecond.

`tower_eq` is proved by kernel evaluation (`decide +kernel`): reducing `tower`
meets each `xₙ` twice and must evaluate it once.
-/
namespace SharedSubterms

/-- `2 ^ 60`, by sixty doublings. Each `let` is used twice by the next. -/
def tower : Nat :=
  let x0 := 1
  let x1 := x0 + x0
  let x2 := x1 + x1
  let x3 := x2 + x2
  let x4 := x3 + x3
  let x5 := x4 + x4
  let x6 := x5 + x5
  let x7 := x6 + x6
  let x8 := x7 + x7
  let x9 := x8 + x8
  let x10 := x9 + x9
  let x11 := x10 + x10
  let x12 := x11 + x11
  let x13 := x12 + x12
  let x14 := x13 + x13
  let x15 := x14 + x14
  let x16 := x15 + x15
  let x17 := x16 + x16
  let x18 := x17 + x17
  let x19 := x18 + x18
  let x20 := x19 + x19
  let x21 := x20 + x20
  let x22 := x21 + x21
  let x23 := x22 + x22
  let x24 := x23 + x23
  let x25 := x24 + x24
  let x26 := x25 + x25
  let x27 := x26 + x26
  let x28 := x27 + x27
  let x29 := x28 + x28
  let x30 := x29 + x29
  let x31 := x30 + x30
  let x32 := x31 + x31
  let x33 := x32 + x32
  let x34 := x33 + x33
  let x35 := x34 + x34
  let x36 := x35 + x35
  let x37 := x36 + x36
  let x38 := x37 + x37
  let x39 := x38 + x38
  let x40 := x39 + x39
  let x41 := x40 + x40
  let x42 := x41 + x41
  let x43 := x42 + x42
  let x44 := x43 + x43
  let x45 := x44 + x44
  let x46 := x45 + x45
  let x47 := x46 + x46
  let x48 := x47 + x47
  let x49 := x48 + x48
  let x50 := x49 + x49
  let x51 := x50 + x50
  let x52 := x51 + x51
  let x53 := x52 + x52
  let x54 := x53 + x53
  let x55 := x54 + x54
  let x56 := x55 + x55
  let x57 := x56 + x56
  let x58 := x57 + x57
  let x59 := x58 + x58
  let x60 := x59 + x59
  x60

/-- The tower evaluates to `2 ^ 60`. -/
theorem tower_eq : tower = 1152921504606846976 := by decide +kernel

end SharedSubterms
