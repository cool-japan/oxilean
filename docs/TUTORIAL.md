# OxiLean Tutorial

A progressive tutorial series for learning OxiLean from first principles to advanced topics.

## Table of Contents

1. [Hello Proof — Proving True](#hello-proof--proving-true)
2. [Basic Arithmetic on Nat](#basic-arithmetic-on-nat)
3. [List Operations and Induction](#list-operations-and-induction)
4. [Propositional Logic Proofs](#propositional-logic-proofs)
5. [Quantifiers and Predicates](#quantifiers-and-predicates)
6. [Functions and Function Extensionality](#functions-and-function-extensionality)
7. [Defining Your Own Types](#defining-your-own-types)
8. [Type Classes by Example](#type-classes-by-example)
9. [Building a Small Library](#building-a-small-library)
10. [Advanced Tactics](#advanced-tactics)

---

## Hello Proof — Proving True

**Learning Goals:**
- Understand the structure of a theorem
- Use trivial proof
- Learn basic proof by definition

### Lesson 1.1: The Simplest Proof

The easiest theorem in OxiLean is proving `True`:

```oxilean
theorem true_is_true : True := trivial
```

This says:
- `theorem` — I'm declaring a theorem (a proved statement)
- `true_is_true` — the name of the theorem
- `: True` — the proposition being proved
- `:= trivial` — the proof (trivial is the built-in proof of True)

**Try it:**

```bash
cat > lesson1.oxilean << 'EOF'
theorem true_is_true : True := trivial
EOF

oxilean lesson1.oxilean
# Output: ✓ File checked successfully
```

### Lesson 1.2: Proof by Reflexivity

Equality of identical things:

```oxilean
theorem one_eq_one : 1 = 1 := rfl
```

The proof is `rfl` (reflexivity) — anything is equal to itself.

**Key insight:** `rfl` only works for *definitionally equal* terms:

```oxilean
theorem two_plus_two : 2 + 2 = 4 := rfl
  -- This works because 2+2 *reduces* to 4

-- This DOES NOT work:
-- theorem wrong : 2 + 2 = 3 + 1 := rfl
-- Because while equal, they don't reduce to the same thing
```

### Lesson 1.3: Proof by Definition

Using the definition of a function:

```oxilean
def not (b : Bool) : Bool := match b with
  | true => false
  | false => true

theorem not_true : not true = false := rfl
```

This works because when OxiLean *unfolds* the definition of `not` and applies it to `true`, it gets `false` directly.

### Lesson 1.4: Proof by Tactics

Tactics are commands that transform goals. Use `by` to enter tactic mode:

```oxilean
theorem true_tactic : True := by
  trivial
```

This is equivalent to `trivial` above, but shows tactic syntax.

**Multiple tactics:**

```oxilean
theorem conjunction : True ∧ True := by
  constructor  -- split goal into two: True and True
  · trivial    -- prove first True
  · trivial    -- prove second True
```

### Lesson 1.5: Understanding Errors

Try proving something false:

```oxilean
-- DO NOT COPY — intentional error
-- theorem false_proof : False := trivial
-- Error: trivial cannot prove False
```

OxiLean rejects this because `False` has no proof.

**Key insight:** The type system prevents us from proving falsehoods.

---

## Basic Arithmetic on Nat

**Learning Goals:**
- Understand natural number operations
- Learn `simp` tactic
- Prove basic arithmetic lemmas
- Use `omega` for automation

### Lesson 2.1: Natural Number Basics

```oxilean
-- Nat has two constructors:
-- 0 : Nat
-- n.succ : Nat (successor)

def nat_zero : Nat := 0
def nat_one : Nat := 1
def nat_five : Nat := 5

-- Arithmetic
def two_plus_three : Nat := 2 + 3

theorem add_result : 2 + 3 = 5 := rfl
```

### Lesson 2.2: The `simp` Tactic

`simp` applies simplification lemmas. Natural numbers have many built-in simplifications:

```oxilean
-- Adding zero
theorem add_zero : ∀ n : Nat, n + 0 = n := by
  intro n
  simp

-- Zero plus
theorem zero_add : ∀ n : Nat, 0 + n = n := by
  intro n
  simp
```

**How it works:**
- `simp` looks for lemmas like `n + 0 = n`
- Matches them against the goal
- Applies rewrites until goal is closed

### Lesson 2.3: The `omega` Tactic

`omega` is a solver for linear arithmetic:

```oxilean
theorem arithmetic : 2 + 3 = 5 := by omega

theorem inequality : 5 < 10 := by omega

theorem linear_combo : 2*n + 3*m ≤ 5*(n + m) := by omega
```

**When to use `omega`:**
- Pure arithmetic goals
- Inequalities
- Linear combinations

### Lesson 2.4: Induction on Nat

Prove properties by mathematical induction:

```oxilean
theorem nat_add_zero : ∀ n : Nat, n + 0 = n := by
  intro n
  induction n with
  | zero =>
    -- Base case: 0 + 0 = 0
    simp
  | succ n ih =>
    -- Inductive case: assume n + 0 = n (ih = inductive hypothesis)
    -- Prove: n.succ + 0 = n.succ
    simp [Nat.succ_eq_add_one, ih]
```

**Structure:**
- `intro n` — introduce the variable
- `induction n` — start induction
- `| zero =>` — base case (n = 0)
- `| succ n ih =>` — inductive case (n becomes n.succ, ih is the assumption for n)

### Lesson 2.5: Commutativity of Addition

```oxilean
theorem nat_add_comm : ∀ n m : Nat, n + m = m + n := by
  intro n m
  induction n with
  | zero =>
    -- Goal: 0 + m = m + 0
    simp
  | succ n ih =>
    -- Assumption (ih): n + m = m + n
    -- Goal: n.succ + m = m + n.succ
    simp [Nat.succ_eq_add_one, Nat.add_assoc, ih]
```

### Lesson 2.6: Associativity of Addition

```oxilean
theorem nat_add_assoc : ∀ n m k : Nat, n + (m + k) = (n + m) + k := by
  intro n m k
  induction n with
  | zero => simp
  | succ n ih => simp [Nat.succ_eq_add_one, ih]
```

---

## List Operations and Induction

**Learning Goals:**
- Work with inductive types (Lists)
- Structural induction
- Prove list properties

### Lesson 3.1: List Basics

```oxilean
-- List has two constructors:
-- [] : List α
-- h :: t : List α (cons)

def my_list : List Nat := [1, 2, 3]

def singleton (x : Nat) : List Nat := [x]

def empty : List Nat := []
```

### Lesson 3.2: List Operations

```oxilean
-- Length of list
def length : List α → Nat
  | [] => 0
  | _ :: xs => 1 + length xs

-- Append lists
def append : List α → List α → List α
  | [], ys => ys
  | x :: xs, ys => x :: append xs ys

-- Map over list
def map (f : α → β) : List α → List β
  | [] => []
  | x :: xs => f x :: map f xs
```

### Lesson 3.3: Properties of Length

```oxilean
theorem length_nil : length ([] : List Nat) = 0 := rfl

theorem length_singleton : ∀ x : Nat, length [x] = 1 := by
  intro x
  simp [length]

theorem length_append : ∀ (xs ys : List Nat),
  length (append xs ys) = length xs + length ys := by
  intro xs ys
  induction xs with
  | nil => simp [append, length]
  | cons x xs ih =>
    simp [append, length, ih]
```

### Lesson 3.4: Structural Induction

```oxilean
-- Prove map preserves length
theorem length_map : ∀ (f : Nat → Nat) (xs : List Nat),
  length (map f xs) = length xs := by
  intro f xs
  induction xs with
  | nil =>
    -- Base: length (map f []) = length []
    simp [map, length]
  | cons x xs ih =>
    -- Inductive step:
    -- Assume (ih): length (map f xs) = length xs
    -- Prove: length (map f (x :: xs)) = length (x :: xs)
    simp [map, length, ih]
```

### Lesson 3.5: Append is Associative

```oxilean
theorem append_assoc : ∀ (xs ys zs : List Nat),
  append (append xs ys) zs = append xs (append ys zs) := by
  intro xs ys zs
  induction xs with
  | nil => rfl
  | cons x xs ih =>
    simp [append, ih]
```

### Lesson 3.6: Reverse and Properties

```oxilean
def reverse : List α → List α
  | [] => []
  | x :: xs => append (reverse xs) [x]

theorem reverse_nil : reverse ([] : List Nat) = [] := rfl

theorem reverse_cons : ∀ (x : Nat) (xs : List Nat),
  reverse (x :: xs) = append (reverse xs) [x] := by
  intro x xs
  rfl  -- definitionally equal

theorem reverse_reverse : ∀ (xs : List Nat),
  reverse (reverse xs) = xs := by
  intro xs
  induction xs with
  | nil => rfl
  | cons x xs ih =>
    simp [reverse, append, ih]
```

---

## Propositional Logic Proofs

**Learning Goals:**
- Prove logical theorems
- Understand logical connectives
- Use proof tactics for logic

### Lesson 4.1: Conjunction

Conjunction is "and" (∧):

```oxilean
theorem and_intro : ∀ (P Q : Prop), P → Q → P ∧ Q := by
  intro P Q hp hq
  constructor  -- split goal into P and Q
  · exact hp
  · exact hq

theorem and_left : ∀ (P Q : Prop), P ∧ Q → P := by
  intro P Q h
  exact h.left

theorem and_right : ∀ (P Q : Prop), P ∧ Q → Q := by
  intro P Q h
  exact h.right

theorem and_comm : ∀ (P Q : Prop), P ∧ Q → Q ∧ P := by
  intro P Q h
  constructor
  · exact h.right
  · exact h.left
```

### Lesson 4.2: Disjunction

Disjunction is "or" (∨):

```oxilean
theorem or_intro_left : ∀ (P Q : Prop), P → P ∨ Q := by
  intro P Q hp
  left  -- choose left branch
  exact hp

theorem or_intro_right : ∀ (P Q : Prop), Q → P ∨ Q := by
  intro P Q hq
  right  -- choose right branch
  exact hq

theorem or_comm : ∀ (P Q : Prop), P ∨ Q → Q ∨ P := by
  intro P Q h
  cases h with
  | inl hp => right; exact hp
  | inr hq => left; exact hq
```

### Lesson 4.3: Negation

Negation is "not" (¬):

```oxilean
-- ¬P is defined as P → False
def not (P : Prop) : Prop := P → False

theorem not_intro : ∀ (P : Prop), (P → False) → ¬P := by
  intro P h
  exact h

theorem not_elim : ∀ (P : Prop), ¬P → P → False := by
  intro P h hp
  exact h hp

theorem not_not : ∀ (P : Prop), P → ¬¬P := by
  intro P hp h
  exact h hp

theorem double_neg : ∀ (P : Prop), ¬¬(P ∨ ¬P) := by
  intro P h
  apply h
  by_cases hP : P
  · left; exact hP
  · right; exact hP
```

### Lesson 4.4: Implication

Implication is logical consequence (→):

```oxilean
theorem imp_intro : ∀ (P Q : Prop), Q → P → Q := by
  intro P Q hq _
  exact hq

theorem imp_trans : ∀ (P Q R : Prop),
  (P → Q) → (Q → R) → P → R := by
  intro P Q R hpq hqr hp
  exact hqr (hpq hp)

theorem proof_by_contradiction : ∀ (P : Prop),
  (¬P → False) → P := by
  intro P h
  by_contra hn
  exact h hn
```

### Lesson 4.5: Biconditional

Biconditional is "if and only if" (↔):

```oxilean
-- A ↔ B is defined as (A → B) ∧ (B → A)

theorem iff_intro : ∀ (P Q : Prop),
  (P → Q) → (Q → P) → P ↔ Q := by
  intro P Q hpq hqp
  constructor
  · exact hpq
  · exact hqp

theorem iff_left : ∀ (P Q : Prop),
  (P ↔ Q) → P → Q := by
  intro P Q h hp
  exact h.mp hp

theorem iff_right : ∀ (P Q : Prop),
  (P ↔ Q) → Q → P := by
  intro P Q h hq
  exact h.mpr hq
```

---

## Quantifiers and Predicates

**Learning Goals:**
- Understand universal and existential quantification
- Work with predicates
- Prove statements about all or some

### Lesson 5.1: Universal Quantification

```oxilean
-- ∀ x : Nat, P x means "for all x, P x holds"

theorem all_nat_ge_zero : ∀ n : Nat, n ≥ 0 := by
  intro n
  omega

theorem all_add_zero : ∀ n : Nat, n + 0 = n := by
  intro n
  simp

theorem nested_forall : ∀ n m : Nat, n + m = m + n := by
  intro n m
  omega
```

### Lesson 5.2: Existential Quantification

```oxilean
-- ∃ x : Nat, P x means "there exists x such that P x"

theorem exists_double : ∃ n : Nat, n = 2 * 1 := by
  use 2
  simp

theorem exists_greater : ∀ n : Nat, ∃ m : Nat, m > n := by
  intro n
  use n + 1
  omega

theorem exists_and : ∃ (n m : Nat), n + m = 5 := by
  use 2
  use 3
  simp
```

### Lesson 5.3: Predicates

A predicate is a function returning Prop:

```oxilean
-- Even: a predicate on Nat
def even (n : Nat) : Prop := ∃ k : Nat, n = 2 * k

-- Odd: another predicate
def odd (n : Nat) : Prop := ∃ k : Nat, n = 2 * k + 1

theorem four_is_even : even 4 := by
  use 2
  simp

theorem zero_is_even : even 0 := by
  use 0
  simp

theorem three_is_odd : odd 3 := by
  use 1
  simp
```

### Lesson 5.4: Properties of Predicates

```oxilean
theorem even_add_even : ∀ n m : Nat,
  even n → even m → even (n + m) := by
  intro n m ⟨kn, hn⟩ ⟨km, hm⟩
  use kn + km
  rw [hn, hm]
  ring

theorem odd_add_odd : ∀ n m : Nat,
  odd n → odd m → even (n + m) := by
  intro n m ⟨kn, hn⟩ ⟨km, hm⟩
  use kn + km + 1
  rw [hn, hm]
  ring
```

### Lesson 5.5: Universal Properties

```oxilean
theorem forall_imply : ∀ (P Q : Nat → Prop),
  (∀ n, P n → Q n) → (∀ n, P n) → (∀ n, Q n) := by
  intro P Q h hP n
  exact h n (hP n)

theorem exists_and_distrib : ∃ n : Nat, P n ∧ Q n →
  (∃ n, P n) ∧ (∃ n, Q n) := by
  intro ⟨n, hp, hq⟩
  exact ⟨⟨n, hp⟩, ⟨n, hq⟩⟩
```

---

## Functions and Function Extensionality

**Learning Goals:**
- Understand function definitions
- Learn about higher-order functions
- Prove properties about functions

### Lesson 6.1: Basic Functions

```oxilean
def id (x : Nat) : Nat := x

def const (x : Nat) (y : Nat) : Nat := x

def swap (f : Nat → Nat → Nat) (x y : Nat) : Nat := f y x

theorem id_identity : ∀ n : Nat, id n = n := by
  intro n
  rfl
```

### Lesson 6.2: Higher-Order Functions

```oxilean
-- Function as argument
def apply_twice (f : Nat → Nat) (n : Nat) : Nat := f (f n)

def compose (f : Nat → Nat) (g : Nat → Nat) : Nat → Nat :=
  fun x => f (g x)

theorem compose_assoc : ∀ (f g h : Nat → Nat) (n : Nat),
  compose f (compose g h) n = compose (compose f g) h n := by
  intro f g h n
  rfl
```

### Lesson 6.3: Function Extensionality

Functions that do the same thing are equal:

```oxilean
-- Function extensionality principle
axiom funext : ∀ (f g : Nat → Nat),
  (∀ x, f x = g x) → f = g

theorem double_eq_add_self : (fun n => 2 * n) = (fun n => n + n) := by
  apply funext
  intro n
  ring
```

### Lesson 6.4: Injectivity and Surjectivity

```oxilean
def injective (f : Nat → Nat) : Prop :=
  ∀ x y, f x = f y → x = y

def surjective (f : Nat → Nat) : Prop :=
  ∀ y, ∃ x, f x = y

theorem succ_injective : injective Nat.succ := by
  intro x y h
  omega

theorem succ_not_surjective : ¬(surjective Nat.succ) := by
  intro h
  obtain ⟨x, hx⟩ := h 0
  simp at hx
```

### Lesson 6.5: Functional Composition

```oxilean
def bijective (f : Nat → Nat) : Prop :=
  injective f ∧ surjective f

theorem composition_injective : ∀ (f g : Nat → Nat),
  injective f → injective g → injective (compose f g) := by
  intro f g hf hg x y h
  simp [compose] at h
  exact hg x y (hf (g x) (g y) h)
```

---

## Defining Your Own Types

**Learning Goals:**
- Define inductive types
- Understand constructors
- Prove by cases

### Lesson 7.1: Simple Inductive Types

```oxilean
-- Boolean type
inductive Bool : Type where
  | true : Bool
  | false : Bool

-- Case analysis
def bool_to_nat : Bool → Nat
  | Bool.true => 1
  | Bool.false => 0

theorem bool_cases : ∀ (b : Bool), b = Bool.true ∨ b = Bool.false := by
  intro b
  cases b
  · left; rfl
  · right; rfl
```

### Lesson 7.2: Recursive Inductive Types

```oxilean
-- Natural numbers (encoded)
inductive MyNat : Type where
  | zero : MyNat
  | succ : MyNat → MyNat

def my_add : MyNat → MyNat → MyNat
  | m, MyNat.zero => m
  | m, MyNat.succ n => MyNat.succ (my_add m n)

theorem my_add_zero : ∀ n : MyNat, my_add n MyNat.zero = n := by
  intro n
  cases n
  · simp [my_add]
  · simp [my_add]
```

### Lesson 7.3: Dependent Types

```oxilean
-- Option type: Some or None
inductive Option (α : Type) : Type where
  | none : Option α
  | some : α → Option α

def get_or_default {α : Type} (x : Option α) (default : α) : α :=
  match x with
  | Option.none => default
  | Option.some x => x

-- Proof about Options
theorem option_cases : ∀ {α : Type} (x : Option α),
  x = Option.none ∨ (∃ a, x = Option.some a) := by
  intro α x
  cases x
  · left; rfl
  · right; use a; rfl
```

### Lesson 7.4: Indexed Inductive Types

```oxilean
-- Vector: list with known length
inductive Vector (α : Type) : Nat → Type where
  | nil : Vector α 0
  | cons : ∀ {n}, α → Vector α n → Vector α (n + 1)

def head {α : Type} {n : Nat} : Vector α (n + 1) → α :=
  fun v => match v with
    | Vector.cons x _ => x

def tail {α : Type} {n : Nat} : Vector α (n + 1) → Vector α n :=
  fun v => match v with
    | Vector.cons _ xs => xs

theorem vector_nil_unique : ∀ (v : Vector Nat 0),
  v = Vector.nil := by
  intro v
  cases v
```

### Lesson 7.5: Sum and Product Types

```oxilean
-- Either/Sum type
inductive Sum (α β : Type) : Type where
  | inl : α → Sum α β
  | inr : β → Sum α β

-- Product type
inductive Prod (α β : Type) : Type where
  | mk : α → β → Prod α β

-- Equivalence between (A, B) and (B, A)
def swap_prod {α β : Type} : Prod α β → Prod β α
  | Prod.mk a b => Prod.mk b a

theorem swap_involutive {α β : Type} : ∀ p : Prod α β,
  swap_prod (swap_prod p) = p := by
  intro ⟨a, b⟩
  rfl
```

---

## Type Classes by Example

**Learning Goals:**
- Define type classes
- Create instances
- Use type class resolution

### Lesson 8.1: Simple Type Class

```oxilean
-- Equality type class
class Eq (α : Type) where
  eq : α → α → Bool
  eq_refl : ∀ a, eq a a = true

-- Instance for Bool
instance BoolEq : Eq Bool where
  eq b c := match b, c with
    | true, true => true
    | false, false => true
    | _, _ => false
  eq_refl b := match b with
    | true => rfl
    | false => rfl

-- Using the instance
def check_bool_eq : Bool := Eq.eq true true
```

### Lesson 8.2: Monoid Type Class

```oxilean
class Monoid (α : Type) where
  empty : α
  op : α → α → α
  left_id : ∀ a, op empty a = a
  right_id : ∀ a, op a empty = a
  assoc : ∀ a b c, op (op a b) c = op a (op b c)

-- Instance for Nat with addition
instance NatAddMonoid : Monoid Nat where
  empty := 0
  op := Nat.add
  left_id := Nat.zero_add
  right_id := Nat.add_zero
  assoc := Nat.add_assoc

-- Using it
theorem nat_monoid_works : Monoid.op (0 : Nat) 5 = 5 := by
  simp [NatAddMonoid]
```

### Lesson 8.3: Functor Type Class

```oxilean
class Functor (f : Type → Type) where
  map : ∀ {α β}, (α → β) → f α → f β
  id_law : ∀ {α} (x : f α), map id x = x
  compose_law : ∀ {α β γ} (g : β → γ) (h : α → β) (x : f α),
    map (g ∘ h) x = map g (map h x)

-- Instance for List
instance ListFunctor : Functor List where
  map := fun f xs => xs.map f
  id_law := by intro α x; rfl
  compose_law := by intro α β γ g h x; rfl
```

### Lesson 8.4: Type Class Inheritance

```oxilean
class Semigroup (α : Type) where
  op : α → α → α
  assoc : ∀ a b c, op (op a b) c = op a (op b c)

class Monoid (α : Type) extends Semigroup α where
  empty : α
  left_id : ∀ a, op empty a = a
  right_id : ∀ a, op a empty = a

-- Nat with + forms a monoid
instance NatMonoid : Monoid Nat where
  op := Nat.add
  assoc := Nat.add_assoc
  empty := 0
  left_id := Nat.zero_add
  right_id := Nat.add_zero
```

### Lesson 8.5: Polymorphic Instances

```oxilean
-- Prod (cartesian product) is a functor
instance ProdFunctor (α : Type) : Functor (fun β => α × β) where
  map f ⟨a, b⟩ := ⟨a, f b⟩
  id_law _ := rfl
  compose_law _ _ _ := rfl

-- List operations
def fold_right [Monoid α] : List α → α
  | [] => Monoid.empty
  | x :: xs => Monoid.op x (fold_right xs)

theorem fold_right_monoid : ∀ [Monoid α] (xs : List α),
  fold_right xs = fold_right xs := by
  intro α inst xs
  rfl
```

---

## Building a Small Library

**Learning Goals:**
- Organize theorems in modules
- Build a library of useful lemmas
- Understand dependencies

### Lesson 9.1: Module Organization

```oxilean
module Arithmetic where

def double (n : Nat) : Nat := n + n
def triple (n : Nat) : Nat := n + n + n

theorem double_eq : double n = n + n := rfl

theorem triple_eq : triple n = n + n + n := rfl

end Arithmetic

-- Using the module
example : Arithmetic.double 5 = 10 := by rfl
```

### Lesson 9.2: Building a List Library

```oxilean
module List where

def length : List α → Nat
  | [] => 0
  | _ :: xs => 1 + length xs

def append : List α → List α → List α
  | [], ys => ys
  | x :: xs, ys => x :: append xs ys

def map (f : α → β) : List α → List β
  | [] => []
  | x :: xs => f x :: map f xs

-- Theorems
theorem length_nil : length ([] : List Nat) = 0 := rfl

theorem append_nil : ∀ xs : List Nat, append xs [] = xs := by
  intro xs
  induction xs with
  | nil => rfl
  | cons x xs ih => simp [append, ih]

theorem length_append : ∀ xs ys : List Nat,
  length (append xs ys) = length xs + length ys := by
  intro xs ys
  induction xs with
  | nil => simp [length, append]
  | cons x xs ih => simp [length, append, ih]

end List
```

### Lesson 9.3: Lemmas and Theorems

```oxilean
module Nat where

-- Helper lemmas
lemma succ_injective : ∀ n m : Nat,
  n.succ = m.succ → n = m := by
  intro n m h
  omega

lemma zero_unique : ∀ n : Nat,
  n = 0 ∨ ∃ k, n = k.succ := by
  intro n
  cases n
  · left; rfl
  · right; use n; rfl

-- Theorems built on lemmas
theorem add_comm : ∀ n m : Nat, n + m = m + n := by
  intro n m
  induction n with
  | zero => simp
  | succ n ih => simp [ih]

theorem mul_assoc : ∀ n m k : Nat,
  n * (m * k) = (n * m) * k := by
  intro n m k
  induction n with
  | zero => simp
  | succ n ih => simp [ih]; ring

end Nat
```

### Lesson 9.4: Type Class Library

```oxilean
module Classes where

class Eq (α : Type) where
  eq : α → α → Bool

class Show (α : Type) where
  show : α → String

instance NatEq : Eq Nat where
  eq := Nat.eq

instance NatShow : Show Nat where
  show := Nat.toString

def print [Show α] (x : α) : String := Show.show x

end Classes
```

### Lesson 9.5: Import and Reuse

```oxilean
-- main.oxilean

import Arithmetic
import List
import Nat
import Classes

open Arithmetic Nat List

theorem combined : ∀ n : Nat,
  length (replicate n (double n)) = n := by
  intro n
  induction n with
  | zero => simp
  | succ n ih => simp [ih]
```

---

## Advanced Tactics

**Learning Goals:**
- Master advanced proof techniques
- Understand automation tactics
- Use proof combinators

### Lesson 10.1: Automation Tactics

```oxilean
-- simp with lemmas
theorem simp_example : ∀ n : Nat,
  n + 0 + 0 = n := by
  intro n
  simp [Nat.add_zero]

-- omega for arithmetic
theorem omega_example : ∀ n m : Nat,
  2*n + 3*m ≤ 5*(n + m) := by
  intro n m
  omega

-- ring for polynomials
theorem ring_example : ∀ x y : Nat,
  (x + y)^2 + x^2 = x^2 + 2*x*y + y^2 + x^2 := by
  intro x y
  ring
```

### Lesson 10.2: Proof Combinators

```oxilean
-- Using <|> (try first, then second)
theorem try_tactics : ∀ n : Nat, n + 0 = n := by
  intro n
  omega <|> simp

-- Using ; (sequence)
theorem sequence_tactics : ∀ n m : Nat, n + m = m + n := by
  intro n m
  induction n
  · simp
  · simp [*]; ring

-- Using all_goals
theorem all_goals_example : P ∧ Q ∧ R := by
  constructor
  all_goals intro; assumption
```

### Lesson 10.3: Custom Automation

```oxilean
-- Lemmas for automation
lemma simp_eq : ∀ n : Nat, n = n := by intro n; rfl
lemma simp_zero : ∀ n : Nat, 0 + n = n := Nat.zero_add
lemma simp_succ : ∀ n m : Nat, n.succ + m = (n + m).succ := by
  intro n m; rfl

-- Use with simp
theorem custom_automation : ∀ n : Nat, 0 + n = n := by
  intro n
  simp [simp_zero]
```

### Lesson 10.4: Strong Induction

```oxilean
theorem strong_induction : ∀ n : Nat, P n := by
  intro n
  induction n, n using Nat.recOn with
  | zero => -- base case
  | succ n ih =>
    -- ih: ∀ m ≤ n, P m
    -- goal: P (n + 1)
    sorry
```

### Lesson 10.5: Proof by Contradiction

```oxilean
theorem by_contra_example : ∀ n : Nat,
  n = 0 ∨ ∃ k, n = k + 1 := by
  intro n
  by_contra h
  push_neg at h
  obtain ⟨_, -⟩ := h
  -- Derive contradiction
  cases n
  · simp at h
  · simp at h

theorem classical_excluded_middle : ∀ (p : Prop),
  p ∨ ¬p := by
  intro p
  by_cases hp : p
  · left; exact hp
  · right; exact hp
```

---

## Summary and Next Steps

You've learned:

1. **Basics**: Simple proofs with `trivial` and `rfl`
2. **Arithmetic**: Natural number operations and induction
3. **Lists**: Structural induction and list properties
4. **Logic**: Logical connectives and reasoning
5. **Quantifiers**: Universal and existential statements
6. **Functions**: Higher-order functions and properties
7. **Types**: Defining inductive types
8. **Type Classes**: Polymorphism through type classes
9. **Libraries**: Organizing proofs into modules
10. **Advanced Tactics**: Automation and proof techniques

### Next Steps:

OxiLean is a fully implemented proof assistant with 11 crates and 1.2M+ total SLOC. All development phases are complete, including the kernel, elaborator, parser, standard library, build system, linter, code generation, runtime, and WASM support.

1. **Explore the Standard Library** — Browse modules in `crates/oxilean-std/src/` for built-in lemmas, tactics, and data structures
2. **Prove Your Own Theorems** — Try proving properties of data structures or formalizing mathematical results
3. **Build a Project** — Use `oxilean build` to create and manage formal verification projects
4. **Use the CLI Tools** — Run `oxilean lint`, `oxilean check --verbose`, or `oxilean check --json` for development workflows
5. **Target WASM** — Export proofs and verified code to WebAssembly with `oxilean build --target wasm`
6. **Join the Community** — Contribute to the OxiLean project on GitHub

Good luck with your formal verification journey!
