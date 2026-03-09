# OxiLean User Guide

A comprehensive guide to using OxiLean, the pure Rust Interactive Theorem Prover.

## Table of Contents

1. [Installation and Setup](#installation-and-setup)
2. [First Proof Walkthrough](#first-proof-walkthrough)
3. [Basic Syntax and Concepts](#basic-syntax-and-concepts)
4. [Type Theory Primer](#type-theory-primer)
5. [Defining Functions and Theorems](#defining-functions-and-theorems)
6. [Tactic Reference](#tactic-reference)
7. [Type Classes and Instances](#type-classes-and-instances)
8. [Structures and Inheritance](#structures-and-inheritance)
9. [Module System and Imports](#module-system-and-imports)
10. [CLI Commands Reference](#cli-commands-reference)
11. [REPL Usage and Tips](#repl-usage-and-tips)
12. [Troubleshooting](#troubleshooting)

---

## Installation and Setup

### System Requirements

- **Rust**: 1.70 or later
- **Operating System**: Linux, macOS, or Windows (with WSL recommended)
- **RAM**: 2GB minimum, 8GB+ recommended
- **Disk Space**: 500MB for build artifacts

### Installation from Source

```bash
# Clone the repository
git clone https://github.com/cool-japan/oxilean.git
cd oxilean

# Build the project
cargo build --release

# Verify installation
./target/release/oxilean --version
```

### Installation via Package Manager

OxiLean is not yet available in standard package managers. Installation from source is recommended.

### Build Configuration

For optimal performance, OxiLean uses release-optimized settings:

```toml
[profile.release]
lto = true                  # Link-time optimization
codegen-units = 1          # Single codegen unit for better optimization
opt-level = 3              # Maximum optimization
strip = true               # Strip symbols for smaller binary
```

### Setting Up Your First Project

Create a new directory for your proofs:

```bash
mkdir my_proofs
cd my_proofs

# Create a simple proof file
cat > hello.oxilean << 'EOF'
theorem hello : True := trivial
EOF

# Check the proof
oxilean hello.oxilean
```

### Configuration Files

OxiLean searches for configuration in the following order:

1. `.oxilean.toml` — Project-specific configuration
2. `~/.oxilean/config.toml` — User configuration
3. Built-in defaults

Example `.oxilean.toml`:

```toml
[project]
name = "my_proofs"
version = "0.1.1"

[kernel]
universe_polymorphism = true
proof_irrelevance = true

[tactics]
simp_depth = 100
timeout = 30000  # milliseconds
```

### IDE Integration

OxiLean provides LSP (Language Server Protocol) support:

**VS Code Setup:**

1. Install the OxiLean extension from the marketplace (when available)
2. Configure in `.vscode/settings.json`:

```json
{
  "oxilean.serverPath": "/path/to/oxilean",
  "oxilean.checkOnSave": true,
  "oxilean.trace.server": "verbose"
}
```

**Vim/Neovim Setup:**

Configure with `vim-lsp` or `nvim-lspconfig`:

```lua
require'lspconfig'.oxilean.setup{}
```

---

## First Proof Walkthrough

Let's prove a simple logical statement step-by-step.

### Step 1: The Simplest Proof

Create a file `first_proof.oxilean`:

```oxilean
-- Comment: This is a proof of True
theorem true_is_true : True := trivial
```

Check it:

```bash
oxilean first_proof.oxilean
```

Output:

```
✓ File checked successfully
Theorem true_is_true : True
```

**What happened:**
- `theorem` declares a proof statement
- The name is `true_is_true`
- The type/proposition is `True`
- The proof is `trivial` (built-in proof of True)
- `:=` separates the type from the proof term

### Step 2: Proving with Tactics

For more complex proofs, use tactics. Create `first_tactic_proof.oxilean`:

```oxilean
theorem true_implies_true : True → True := by
  intro h
  exact h
```

**What happened:**
- `by` enters tactic mode
- `intro h` introduces the hypothesis into the context
- `exact h` provides the exact proof (the hypothesis itself)

### Step 3: Proving About Natural Numbers

```oxilean
theorem add_zero : ∀ n : Nat, n + 0 = n := by
  intro n
  simp
```

**What happened:**
- `∀ n : Nat` is universal quantification
- `simp` applies simplification rules, which include `n + 0 = n`

### Step 4: Inductive Proof

```oxilean
theorem add_comm : ∀ n m : Nat, n + m = m + n := by
  intro n m
  induction n with
  | zero => simp
  | succ n ih =>
    simp [Nat.add_succ, ih]
```

**What happened:**
- `induction` applies mathematical induction
- `| zero =>` handles the base case
- `| succ n ih =>` handles the inductive case with `ih` as the inductive hypothesis
- `Nat.add_succ` is a lemma about `Nat.succ`

### Step 5: Understanding Error Messages

Try this incorrect proof:

```oxilean
theorem wrong : 1 = 2 := by
  rfl
```

Error output:

```
Error: reflexivity tactic failed
Expected: 1
Got: 2
These are not definitionally equal
Location: first.oxilean:1:28
```

**Key points:**
- `rfl` (reflexivity) only works for definitionally equal terms
- Error messages include the location in the source file
- Shows what OxiLean expected versus what it found

---

## Basic Syntax and Concepts

### Comments

```oxilean
-- Single-line comment

/- Multi-line comment
   can span multiple lines -/

/- Comments can be /- nested -/ -/
```

### Identifiers and Names

Valid identifiers:

```oxilean
x          -- single letter
my_var     -- with underscores
var123     -- with numbers
Nat        -- capitalized (convention for types)
add_comm   -- mixed case
```

Hierarchical names:

```oxilean
Nat.add         -- Nat namespace
Nat.add.comm    -- nested namespace
List.map.inj    -- deeply nested
```

### Types and Universes

```oxilean
Type           -- Type 0 (type of types)
Type 1         -- Type 1 (type of Type 0)
Type u         -- universe variable
Prop           -- propositions (subtype of Type)
Sort 0         -- same as Prop
Sort 1         -- same as Type
Sort (u + 1)   -- universe expression
```

### Dependent Types

```oxilean
-- Non-dependent function: A → B
f : Nat → Bool

-- Dependent function: for each n : Nat, we get P n
forall n : Nat, P n

-- Pi notation (explicit)
Π (n : Nat), P n

-- Lambda notation
fun n => P n

-- Type annotation
x : Nat := 5
```

### Pattern Matching

```oxilean
-- Match on Nat
match n with
| Nat.zero => ...
| Nat.succ n => ...

-- Match on Bool
match b with
| true => ...
| false => ...

-- Match on Lists
match lst with
| [] => ...
| head :: tail => ...
```

### Operators and Precedence

Binary operators:

```oxilean
+       -- addition (infixl 65)
-       -- subtraction (infixl 65)
*       -- multiplication (infixl 70)
/       -- division (infixl 70)
=       -- equality (infix 50)
<       -- less than (infix 50)
>       -- greater than (infix 50)
∧       -- logical and (infixr 35)
∨       -- logical or (infixr 30)
→       -- implication (infixr 25)
↔       -- iff (infixr 20)
```

### Function Application

```oxilean
-- Explicit application
f x y z

-- Parentheses for clarity
f (g x) y

-- Sections (partial application)
(+ 5)       -- function that adds 5
(5 *)       -- function that multiplies by 5

-- Multiple arguments
map f [1, 2, 3]
```

### Type Ascriptions

```oxilean
-- Explicit type ascription
(5 : Nat)
(fun x => x : Nat → Nat)

-- In function arguments
def double (n : Nat) : Nat := n + n

-- Implicit arguments with {}
def id {α : Type} (x : α) : α := x
```

---

## Type Theory Primer

### Understanding Propositions and Types

In OxiLean, propositions are types:

```oxilean
Prop : Type       -- Prop is a universe
1 = 1 : Prop      -- equality is a proposition
Nat : Type        -- Nat is a type
5 : Nat           -- 5 is a term of type Nat
```

### The Universe Hierarchy

OxiLean has cumulative universes:

```oxilean
Prop : Type 0
Type 0 : Type 1
Type 1 : Type 2
...

-- Equivalently:
Prop : Sort 0
Type : Sort 1
Sort 1 : Sort 2
```

**Cumulativity**: If `A : Type u` then `A : Type (u + 1)`.

### Dependent Types

Functions whose return type depends on arguments:

```oxilean
-- Vector of length n
Vector (α : Type) (n : Nat) : Type

-- Using it
v : Vector Nat 3       -- Vector of 3 natural numbers

-- Dependent function
def length {α : Type} : ∀ n : Nat, Vector α n → Nat :=
  fun n _ => n
```

### Pi Types (Function Types)

```oxilean
-- Simple function type
Nat → Nat

-- Dependent function type
∀ (n : Nat), P n
Π (n : Nat), P n

-- In terms
fun n => n + 1
```

**Key insight**: In CiC, `Nat → Nat` and `∀ (_:Nat), Nat` are the same thing.

### Proof by Propositions as Types

The Curry-Howard isomorphism: propositions are types, proofs are terms.

```oxilean
-- Proposition: P → Q
-- Proof: a function from P to Q
theorem imp_trans : P → Q → R → P → R := by
  intro p q r hp
  exact r

-- Proposition: P ∧ Q
-- Proof: a pair (p, q) where p : P and q : Q
theorem and_intro : P → Q → P ∧ Q := by
  intro p q
  exact ⟨p, q⟩

-- Proposition: P ∨ Q
-- Proof: either inl p or inr q
theorem or_intro_left : P → P ∨ Q := by
  intro p
  exact Or.inl p
```

### Proof Irrelevance

Two proofs of the same proposition are equal:

```oxilean
theorem proof_irrel : ∀ (p q : P), p = q := by
  intro p q
  exact proof_irrelevance p q
```

This is because `P : Prop` is in the Prop universe.

### Universe Polymorphism

Definitions can be generic over universes:

```oxilean
-- Generic identity function works at any universe level
def id {α : Sort u} (x : α) : α := x

-- This type checks:
id (5 : Nat)           -- id : Nat → Nat
id (Nat : Type)        -- id : Type → Type
```

### Implicit Multiplication (IMax)

For impredicative behavior:

```oxilean
-- When both Prop, result is Prop
(Prop → Prop) : Prop

-- When any is Type, result is Type
(Type → Prop) : Type
```

This is captured by `IMax` in the kernel.

---

## Defining Functions and Theorems

### Basic Definitions

```oxilean
-- Definition with type annotation
def double (n : Nat) : Nat := n + n

-- Definition with explicit pattern matching
def is_zero : Nat → Bool
  | 0 => true
  | _ => false

-- Definition using match
def not (b : Bool) : Bool := match b with
  | true => false
  | false => true
```

### Dependent Function Definitions

```oxilean
-- Function returning different types based on input
def at_type : ∀ (t : Type), t → Type := fun t _ => t

-- Function with dependent return type
def replicate {α : Type} : ∀ (n : Nat), α → List α
  | 0, _ => []
  | n + 1, a => a :: replicate n a
```

### Recursive Functions

```oxilean
-- Simple recursion
def factorial : Nat → Nat
  | 0 => 1
  | n + 1 => (n + 1) * factorial n

-- Mutual recursion
mutual
def is_even : Nat → Bool
  | 0 => true
  | n + 1 => is_odd n

def is_odd : Nat → Bool
  | 0 => false
  | n + 1 => is_even n
end
```

### Theorems and Lemmas

```oxilean
-- Theorem: proof term
theorem add_comm : ∀ n m : Nat, n + m = m + n := by
  intro n m
  induction n with
  | zero => simp
  | succ n ih => simp [Nat.add_succ, ih]

-- Lemma: similar, but shorter proofs
lemma add_zero : ∀ n : Nat, n + 0 = n := by simp

-- Corollary: follows from previous theorems
corollary add_left_cancel : a + c = b + c → a = b := by
  intro h
  omega  -- automation tactic
```

### Axioms and Postulates

```oxilean
-- Assume an axiom without proof
axiom classical : ∀ (p : Prop), p ∨ ¬p

-- Postulate (similar to axiom)
postulate univalence : ∀ (A B : Type), (A ↔ B) → A = B
```

### Definition with Arguments

```oxilean
-- Explicit arguments
def add (n : Nat) (m : Nat) : Nat := ...

-- Implicit arguments (inferred)
def length {α : Type} : List α → Nat := ...

-- Instance arguments (resolved by type class system)
def show [Show α] (x : α) : String := Show.show x

-- Universe arguments
def cast {α : Sort u} (a : α) : α := a
```

### Hiding and Abbreviations

```oxilean
-- Abbreviation: inlined at type-checking time
abbrev Predicate (α : Type) := α → Prop

-- Using it
def even : Predicate Nat := fun n => ∃ k, n = 2 * k
```

### Example Definitions

```oxilean
-- List operations
def head {α : Type} : List α → Option α
  | [] => none
  | x :: _ => some x

def tail {α : Type} : List α → List α
  | [] => []
  | _ :: xs => xs

-- Helper for proofs
def congr_fun {α : Type} {β : α → Type} {f g : (a : α) → β a}
  (h : f = g) (a : α) : f a = g a := by
    rw [h]
```

---

## Tactic Reference

Tactics are commands in proof mode (after `by`). They transform goals into simpler subgoals.

### Goal Navigation

#### `intro`

Introduces universally quantified variables or assumptions:

```oxilean
example : ∀ n : Nat, n = n := by
  intro n        -- goal becomes: n = n

example : P → Q := by
  intro hp       -- goal becomes: Q, assuming hp : P
```

#### `intros`

Introduces multiple variables:

```oxilean
example : ∀ n m : Nat, n = m → n = m := by
  intros n m h   -- introduces all three
```

#### `exact`

Closes a goal by providing an exact proof term:

```oxilean
example : 1 + 1 = 2 := by
  exact Nat.add_one_one
```

#### `apply`

Applies a function/theorem, creating new goals from its arguments:

```oxilean
example : a = c := by
  apply eq_trans
  · -- goal: a = b
  · -- goal: b = c
```

#### `rw` / `rewrite`

Rewrites using an equality:

```oxilean
example : n + 0 = n := by
  rw [Nat.add_zero]     -- rewrites using this lemma

example : a + b = c := by
  rw [add_comm, ← some_lemma]  -- multiple rewrites, backwards
```

### Automation Tactics

#### `simp`

Simplification using equality and definitional rules:

```oxilean
example : n + 0 = n := by simp

example : [1, 2] ++ [] = [1, 2] := by simp [List.append_nil]

example : f (g x) = h x := by
  simp only [f_def, g_def]    -- simp with specific lemmas
```

#### `omega`

Arithmetic solver for linear integer arithmetic:

```oxilean
example : 2*n + 3*m ≤ 5*(n+m) := by omega

example : n + m = m + n := by omega
```

#### `ring`

Normalizes polynomial expressions:

```oxilean
example : (x + y)^2 = x^2 + 2*x*y + y^2 := by ring

example : a*(b+c) = a*b + a*c := by ring
```

#### `field_simp`

Simplifies field expressions:

```oxilean
example : a / b * b = a := by
  field_simp
  ring
```

### Induction and Recursion

#### `induction`

Proves by mathematical induction:

```oxilean
example : ∀ n, 0 + n = n := by
  intro n
  induction n with
  | zero => rfl
  | succ n ih => simp [ih]

-- with pattern matching
example : ∀ n, sum n = n * (n+1) / 2 := by
  intro n
  induction n, n with
  | zero m => ...
  | succ n m => ...
```

#### `cases`

Case analysis on constructors:

```oxilean
example : ∀ b : Bool, b = true ∨ b = false := by
  intro b
  cases b
  · exact Or.inl rfl
  · exact Or.inr rfl
```

### Logical Tactics

#### `split`

Splits on a Boolean condition:

```oxilean
example : if n = 0 then true else false := by
  split
  · exact trivial
  · exact trivial
```

#### `by_cases`

Classical case split:

```oxilean
example : ∀ n : Nat, Even n ∨ Odd n := by
  intro n
  by_cases h : Even n
  · exact Or.inl h
  · exact Or.inr (odd_of_not_even h)
```

#### `contradiction`

Closes goal by finding contradiction in hypotheses:

```oxilean
example : False := by
  contradiction
```

### Structural Tactics

#### `sorry`

Admits a goal (for incomplete proofs):

```oxilean
theorem hard : something := by
  intro
  sorry  -- placeholder
```

#### `trivial`

Closes trivial goals:

```oxilean
example : True := by trivial

example : 1 = 1 := by trivial
```

#### `rfl`

Reflexivity - for definitionally equal terms:

```oxilean
example : 1 + 1 = 2 := by rfl

example : (fun x => x) 5 = 5 := by rfl
```

### Goal Management

#### `goal`

Shows the current goal (no effect, just for understanding):

```oxilean
example : P ∧ Q := by
  goal      -- displays current goal
  exact ⟨hp, hq⟩
```

#### `constructor`

Constructs a goal (for And, Or, etc.):

```oxilean
example : P ∧ Q := by
  constructor
  · exact hp
  · exact hq
```

#### `left` / `right`

Chooses left or right side of disjunction:

```oxilean
example : P ∨ Q := by
  left      -- goal becomes P
  exact hp
```

#### `exfalso`

Replaces goal with False (for proof by contradiction):

```oxilean
example : False → anything := by
  intro h
  exfalso
  exact h
```

### Sequence Tactics

#### `all_goals`

Applies tactic to all goals:

```oxilean
example : P ∧ Q := by
  constructor
  all_goals simp
```

#### `any_goals`

Applies tactic only to goals where it succeeds:

```oxilean
example : P ∧ Q ∧ R := by
  constructor
  any_goals constructor
```

#### `;` (semicolon)

Sequence operator:

```oxilean
example : P ∧ Q := by
  constructor; [simp, ring]
```

#### `<|>` (alternation)

Try tactic 1, if fails try tactic 2:

```oxilean
example : goal := by
  simp <|> ring <|> omega
```

### Custom Tactic Combinators

#### `repeat`

Applies tactic repeatedly:

```oxilean
example : deeply_nested := by
  repeat (simp <|> constructor)
```

#### `try`

Attempts tactic, succeeds even if it fails:

```oxilean
example : something := by
  try simp
  exact proof
```

---

## Type Classes and Instances

Type classes enable ad-hoc polymorphism (method overloading).

### Defining Type Classes

```oxilean
-- A type class
class Monoid (α : Type) where
  empty : α
  op : α → α → α
  left_id : ∀ a, op empty a = a
  right_id : ∀ a, op a empty = a
  assoc : ∀ a b c, op (op a b) c = op a (op b c)

-- Usage
class Eq (α : Type) where
  eq : α → α → Prop
  eq_refl : ∀ a, eq a a
  eq_symm : ∀ a b, eq a b → eq b a
  eq_trans : ∀ a b c, eq a b → eq b c → eq a c
```

### Defining Instances

```oxilean
-- Instance for Nat
instance NatMonoid : Monoid Nat where
  empty := 0
  op := Nat.add
  left_id := Nat.zero_add
  right_id := Nat.add_zero
  assoc := Nat.add_assoc

-- Instance for Bool
instance BoolMonoid : Monoid Bool where
  empty := false
  op := fun a b => a && b
  left_id := fun _ => by trivial
  right_id := fun _ => by trivial
  assoc := fun _ _ _ => by trivial
```

### Using Type Classes

```oxilean
-- Function that works with any Monoid
def fold_list [Monoid α] : List α → α
  | [] => Monoid.empty
  | x :: xs => Monoid.op x (fold_list xs)

-- Instance resolution happens automatically
example : fold_list [1, 2, 3] = 6 := by
  rfl  -- instance resolution finds NatMonoid
```

### Type Class Hierarchies

```oxilean
-- Base class
class Show (α : Type) where
  show : α → String

-- Derived class
class Eq (α : Type) where
  eq : α → α → Bool

-- Class with superclass
class Ord (α : Type) extends Eq α where
  lt : α → α → Bool
  le : α → α → Bool
```

### Implicit Arguments in Type Classes

```oxilean
-- These are equivalent:
f [Monoid α] (x : α)
f {inst : Monoid α} (x : α)

-- The former uses instance resolution
-- The latter is explicit
```

### Common Type Classes

```oxilean
-- Equality
class Eq (α : Type) where
  eq : α → α → Prop

-- Ordering
class Ord (α : Type) where
  compare : α → α → Ordering

-- Conversion to string
class Show (α : Type) where
  show : α → String

-- Arithmetic
class Add (α : Type) where
  add : α → α → α

-- Functors
class Functor (f : Type → Type) where
  map : ∀ {α β}, (α → β) → f α → f β

-- Monads
class Monad (m : Type → Type) extends Functor m where
  pure : ∀ {α}, α → m α
  bind : ∀ {α β}, m α → (α → m β) → m β
```

### Instance Search

OxiLean uses depth-first search to resolve instances:

```oxilean
-- Direct instance
instance : Monoid Nat := NatMonoid

-- Derived instance
instance [Monoid α] : Monoid (List α) where
  empty := []
  op := List.append
  -- ...

-- Now works:
def len_list : List (List Nat) → Nat := fold_list (map List.length _)
```

### Avoiding Ambiguity

```oxilean
-- Mark instances as high priority
instance [priority 100] : Add Nat := ⟨Nat.add⟩

-- Or use explicit instance names
instance add_nat : Add Nat := ⟨Nat.add⟩
instance add_string : Add String := ⟨String.append⟩

-- Use explicitly when ambiguous
example : Nat := (5 : Nat) + (3 : Nat)
```

---

## Structures and Inheritance

### Defining Structures

Structures are records with named fields:

```oxilean
-- Simple structure
structure Point where
  x : Nat
  y : Nat

-- Create instance
def origin : Point := { x := 0, y := 0 }

-- Access fields
def distance : Point → Nat := fun p => p.x + p.y
```

### Structures with Inheritance

```oxilean
-- Base structure
structure Point where
  x : Nat
  y : Nat

-- Derived structure
structure Point3D extends Point where
  z : Nat

-- Create instance
def point3d : Point3D := { x := 1, y := 2, z := 3 }

-- Access inherited fields
def get_xy (p : Point3D) : Point := { x := p.x, y := p.y }
```

### Methods and Lemmas

```oxilean
structure Group (α : Type) where
  mul : α → α → α
  inv : α → α
  one : α
  mul_assoc : ∀ a b c, mul (mul a b) c = mul a (mul b c)
  one_mul : ∀ a, mul one a = a
  mul_inv : ∀ a, mul a (inv a) = one

-- Methods are just functions
def pow (G : Group α) : α → Nat → α
  | _, 0 => G.one
  | x, n+1 => G.mul x (pow G x n)
```

### Dependent Structures

```oxilean
-- Dependent record
structure DependentRecord (n : Nat) where
  vec : Vector Nat n
  sum : Nat
  proof : sum_of_vector vec = sum

-- Create with dependent fields
def create_record (n : Nat) (v : Vector Nat n) : DependentRecord n where
  vec := v
  sum := vector_sum v
  proof := rfl
```

### Substructures

```oxilean
-- Substructure: subset with additional properties
structure Sorted (l : List Nat) where
  proof : is_sorted l

-- Coercion to underlying list
instance : Coe (Sorted l) (List Nat) where
  coe s := l
```

---

## Module System and Imports

### Module Basics

```oxilean
-- Define a module
module Nat where
  def double (n : Nat) : Nat := n + n
  def triple (n : Nat) : Nat := n + n + n

-- Use qualified names
def six : Nat := Nat.double 3

-- Open module to use unqualified names
open Nat
def six' : Nat := double 3
```

### Imports

```oxilean
-- Import a file
import Nat

-- Import and open
import open Nat

-- Import with namespace
import Nat as N
def eight : Nat := N.double 4

-- Selective import
import Nat (double, triple)

-- Import except certain items
import Nat hiding triple
```

### File Organization

Standard project structure:

```
my_project/
├── Nat.oxilean       -- module Nat
├── Bool.oxilean      -- module Bool
├── List/
│   ├── Core.oxilean  -- module List.Core
│   └── Theorems.oxilean  -- module List.Theorems
└── main.oxilean      -- main file with imports
```

### Namespaces

```oxilean
-- Define namespace
namespace Nat
  def double (n : Nat) : Nat := n + n
  def triple (n : Nat) : Nat := n + n + n
end Nat

-- Use namespace
def x : Nat := Nat.double 5

-- Open namespace
open Nat
def y : Nat := double 5
```

### Visibility Modifiers

```oxilean
-- Public (default)
def public_def : Nat := 5

-- Hidden (not exported)
private def internal_helper : Nat := 3

-- Protected (can be used, but not opened)
protected def careful : Nat := 7
```

---

## CLI Commands Reference

### Basic Commands

#### `oxilean check`

Checks a file for errors:

```bash
oxilean check myproof.oxilean
oxilean myproof.oxilean          # implicit check
```

Output:

```
✓ File checked successfully
```

#### `oxilean repl`

Starts interactive mode:

```bash
oxilean repl
oxilean                          # default is REPL
```

#### `oxilean version`

Shows version information:

```bash
oxilean version

Output:
OxiLean version 0.1.1
Kernel SLOC: ~113,179
11 crates, 1.2M+ total SLOC
Zero external dependencies in kernel
```

#### `oxilean help`

Shows help information:

```bash
oxilean help
oxilean --help
oxilean -h
```

### Additional Commands

These commands are implemented and available:

```bash
# Build project
oxilean build
oxilean build --target wasm       # WASM export

# Check with output options
oxilean check --verbose myproof.oxilean
oxilean check --json myproof.oxilean

# Performance profiling
oxilean check --profile myproof.oxilean

# Run tests
oxilean test

# Generate documentation
oxilean doc

# Format code
oxilean fmt

# Lint source files
oxilean lint
```

### Environment Variables

```bash
# Set kernel verbosity
export OXILEAN_KERNEL_VERBOSE=1

# Set tactic timeout (milliseconds)
export OXILEAN_TACTIC_TIMEOUT=5000

# Enable kernel logging
export RUST_LOG=oxilean_kernel=debug
```

---

## REPL Usage and Tips

### Starting the REPL

```bash
oxilean repl
```

You'll see:

```
OxiLean interactive theorem prover v0.1.1
Type :help for help, :quit to exit
>
```

### Basic REPL Commands

#### Type Commands

```oxilean
> :type (1 + 2 : Nat)
Nat

> :type [1, 2, 3]
List Nat

> :type Nat.add
Nat → Nat → Nat
```

#### Check Commands

```oxilean
> :check 1 = 1
1 = 1 : Prop

> :check Nat.add_comm
∀ (n m : Nat), n + m = m + n : Prop
```

#### Definition Commands

```oxilean
> def double (n : Nat) : Nat := n + n
Defined: double

> double 5
Result: 10 : Nat

> def lemma : 2 + 3 = 5 := by rfl
Lemma defined: lemma
```

#### Environment Commands

```oxilean
> :env
Definitions in current environment:
- double : Nat → Nat
- lemma : 2 + 3 = 5

> :clear
Environment cleared
```

### Multi-line Input

For multi-line definitions:

```oxilean
> def factorial : Nat → Nat :=
  | 0 => 1
  | n+1 => (n+1) * factorial n
Defined: factorial

> factorial 5
Result: 120 : Nat
```

### History

```oxilean
> :history
1. :type (1 + 2 : Nat)
2. :check Nat.add_comm
3. def double (n : Nat) := n + n

> :history 2
:check Nat.add_comm
```

### Tips and Tricks

**Tip 1: Explore type:**

```oxilean
> :type ?_
> #check @List.map
```

**Tip 2: Experiment with tactics:**

```oxilean
> theorem test : P := by
  intro
  sorry
Theorem defined (partial): test
```

**Tip 3: Quick lemmas:**

```oxilean
> lemma quick : 1 + 1 = 2 := by rfl
Lemma defined: quick
```

**Tip 4: Reload environment:**

```oxilean
> :clear
> import Nat
```

---

## Troubleshooting

### Common Errors

#### "Type mismatch"

```
Error: Expected Nat, got Bool
```

**Solution:**

```oxilean
-- Wrong
def f : Nat := true

-- Correct
def f : Bool := true
```

#### "Unknown identifier"

```
Error: Unknown identifier 'my_var'
```

**Solution:**

Make sure the variable is in scope:

```oxilean
-- Wrong
example : Nat := my_var  -- not defined

-- Correct
example : Nat := by
  intro my_var  -- now in scope
  exact my_var
```

#### "Application error"

```
Error: Cannot apply function
Expected: Nat → Nat
Got: Bool
```

**Solution:**

Check function argument types:

```oxilean
-- Wrong
def double (n : Nat) : Nat := n + n
double true  -- true is Bool

-- Correct
double 5
```

#### "Unification failed"

```
Error: Cannot unify _ and _
```

**Solution:**

Provide type hints:

```oxilean
-- Ambiguous
example := []

-- Clear
example : List Nat := []
```

#### "Tactic failed"

```
Error: simp tactic failed to close goal
Remaining goal: a + b = b + a
```

**Solution:**

Add lemmas to simp or use other tactics:

```oxilean
-- Add lemma
simp [Nat.add_comm]

-- Or use specific tactic
rw [Nat.add_comm]
```

### Performance Issues

**Problem: Proof checking is slow**

Solutions:

1. Use `simp only` instead of `simp` (more specific)
2. Avoid deep recursion in proofs
3. Use `omega` for arithmetic instead of induction
4. Profile with `--profile` flag

```bash
oxilean check --profile myproof.oxilean
```

**Problem: Tactic timeout**

Solution: Increase timeout:

```bash
export OXILEAN_TACTIC_TIMEOUT=30000
oxilean check myproof.oxilean
```

### Kernel Errors

**Problem: "Kernel rejected proof"**

This means the elaborator produced invalid proof terms. Causes:

1. Elaborator bug (file a bug report)
2. Unsafe unsoundness in library
3. Invalid term from custom tactics

**Solution:**

Simplify proof to isolate issue:

```oxilean
-- Complex proof
theorem t : A := by tactic1; tactic2; tactic3

-- Simplify
theorem t : A := by tactic1; sorry
```

### Getting Help

1. **Check error location:**
   - Error messages include file and line numbers
   - Use your IDE to navigate there

2. **Search documentation:**
   - Use `grep` to find similar proofs
   - Check standard library examples

3. **Ask the community:**
   - File issues on GitHub
   - Use discussion forums

4. **Debug with REPL:**
   - Test fragments in REPL
   - Build up incrementally

---

## Advanced Topics

### Implicit Arguments and Unification

Implicit arguments are inferred by unification:

```oxilean
def id {α : Type} (x : α) : α := x

-- Implicit argument is inferred
example : Nat := id 5

-- Can be made explicit
example : Nat := @id Nat 5
```

### Universe Levels

Working with universe polymorphism:

```oxilean
universe u v

def map_between_universes (α : Type u) (f : α → Type v) : Type v := sorry

-- Different universes
example : Type 0 := Bool
example : Type 1 := List
```

### Proof Automation

OxiLean provides several automation tactics:

```oxilean
-- Simplifier with lemmas
simp [lemma1, lemma2]

-- Arithmetic automation
omega

-- Polynomial ring normalization
ring

-- Field simplification
field_simp

-- Congruence closure
cc

-- First-order logic
tauto
```

### Advanced Induction

```oxilean
-- Strong induction
strong_induction n with h

-- Structural induction
induction lst with
| [] => ...
| head :: tail ih => ...

-- Nested induction
induction n, m with
| zero, m => ...
| n, zero => ...
| succ n, succ m (ih_n) (ih_m) => ...
```

---

## Best Practices

1. **Use meaningful names**
   - `n_add_zero` instead of `lemma1`
   - `is_sorted` instead of `check1`

2. **Comment non-obvious proofs**
   - Explain the strategy
   - Note tricky steps

3. **Test incrementally**
   - Build proofs step-by-step
   - Use `sorry` for incomplete parts

4. **Use simp-only with explicit lemmas**
   - More predictable
   - Easier to debug
   - Better performance

5. **Prefer induction for structural properties**
   - Clearer than manual recursion
   - Matches mathematical intuition

6. **Use type classes for polymorphism**
   - More idiomatic
   - Better than manual parameters

7. **Organize proofs in modules**
   - Easier to navigate
   - Clear dependencies

---

## Performance Tips

1. **Use `simp only` with specific lemmas** — faster than `simp`
2. **Use `omega` for arithmetic** — faster than induction
3. **Use `ring` for polynomial normalization** — fast and reliable
4. **Avoid deep proof objects** — they take time to elaborate
5. **Use `partial` for large proofs** — check incrementally
6. **Profile with `--profile`** — identify bottlenecks

---

## Conclusion

This guide covers the essential features of OxiLean. For more information:

- See `TUTORIAL.md` for step-by-step learning
- See `EXAMPLES.md` for complete runnable examples
- See `FAQ.md` for common questions
- Visit GitHub for the latest updates

Happy proving!
