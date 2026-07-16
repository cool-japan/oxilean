# oxilean-std — TODO

> Task list for the standard library crate.
> Last updated: 2026-05-29

## ✅ Completed

**Status**: COMPLETE — ~416,133 SLOC implemented across 1,105 source files

### Core Library Components
- [x] Data structures (List, Array, Vector, HashMap, etc.)
- [x] Mathematical definitions (Nat, Int, Rat, Real)
- [x] Logic and proof utilities
- [x] Type classes and instances
- [x] Functional programming primitives
- [x] Monadic interfaces
- [x] String utilities
- [x] Option and Result types
- [x] Iterator interfaces

### Proof Library
- [x] Basic logic lemmas
- [x] Equality reasoning
- [x] Induction principles
- [x] Decidability instances
- [x] Order theory foundations
- [x] Algebraic structures (Monoid, Group, Ring, Field)

### Standard Tactics
- [x] Automation helpers
- [x] Simplification lemmas
- [x] Rewriting rules

---

## 🐛 Known Issues

None reported. All tests passing.

---

## ✅ Completed: Extended Mathematical Library

- [x] Linear algebra (`linear_algebra.rs`) — vectors, matrices, rank-nullity, Cayley-Hamilton
- [x] Graph theory (`graph.rs`) — 4-color, Euler, Hall, Kuratowski; BFS/DFS/Dijkstra/SCC
- [x] Computational complexity theory (`complexity.rs`) — P, NP, PSPACE, SAT/3-SAT/etc.; 2-SAT/DPLL/Knapsack
- [x] Complex numbers (`complex.rs`) — Euler's formula, roots of unity, Riemann hypothesis
- [x] Number theory (`number_theory.rs`) — primes, CRT, Fermat/Wilson/Dirichlet; Miller-Rabin/Pollard rho

## ✅ Completed: Further Enhancements

- [x] Combinatorics (`combinatorics.rs`) — Fibonacci/Lucas, Catalan, Bell, Stirling, partitions, derangements, Euler totient, Möbius, Ramsey, generating functions, Burnside/Pólya
- [x] Data structures (`data_structures.rs`) — BinaryHeap, SegmentTree, Trie, DisjointSet, AVL tree, SkipList, Deque; kernel axioms for all
- [x] Category theory extended (`category_theory_ext.rs`) — adjunctions, Yoneda, monads/comonads, limits/colimits, monoidal/enriched/2-categories, toposes, Beck monadicity, Kan extensions
- [x] Probability theory (`probability.rs`) — distributions (uniform, binomial, Poisson, Gaussian), Markov chains, Bayes updating, LLN/CLT/Chebyshev axioms
- [x] Formal language theory (`formal_languages.rs`) — DFA, NFA, PDA, CFG, regex, Chomsky hierarchy, pumping lemmas, Myhill-Nerode, Rice's theorem; NFA→DFA subset construction

## ✅ Completed: Extended Mathematical Library (86 modules total)

- [x] Algebraic geometry foundations — `algebraic_geometry.rs` (Schemes, Sheaves, RiemannRoch, SerreDuality, 8 tests)
- [x] Cryptography primitives — `cryptography.rs` (RSA, ECC, AES, SHA, 8 tests)
- [x] Topology (deeper) — `topology_ext.rs` (homology groups, homotopy stubs, 8 tests)
- [x] Algebraic topology — `algebraic_topology.rs` (simplicial complexes, CW complexes, homology, 8 tests)
- [x] Differential geometry — `differential_geometry.rs` (manifolds, Riemannian, curvature, 8 tests)
- [x] Statistical mechanics — `statistical_mechanics.rs` (partition functions, Boltzmann, Bose-Einstein, 8 tests)
- [x] Stochastic processes — `stochastic_processes.rs` (Brownian motion, Markov chains, SDE, 8 tests)
- [x] Machine learning — `machine_learning.rs` (gradient descent, SGD, neural layers, 8 tests)
- [x] Quantum computing — `quantum_computing.rs` (qubits, gates, circuits, 8 tests)
- [x] Proof theory — `proof_theory.rs` (sequent calculus, SAT/DPLL, Gentzen, 8 tests)
- [x] Model theory — `model_theory.rs` (finite structures, Ehrenfeucht-Fraïssé, ultrafilter, 8 tests)
- [x] Linear programming — `linear_programming.rs` (simplex, duality, integer programming, 8 tests)
- [x] Measure theory — `measure_theory.rs` (sigma-algebras, Lebesgue measure, integration, 8 tests)
- [x] Functional analysis — `functional_analysis.rs` (Banach/Hilbert spaces, operators, 8 tests)
- [x] Type theory — `type_theory.rs` (MLTT, CIC, HoTT foundations, 8 tests)
- [x] Information theory — `information_theory.rs` (entropy, mutual information, channel capacity, 8 tests)
- [x] Control theory — `control_theory.rs` (LTI systems, PID controllers, stability, 8 tests)
- [x] Numerical analysis — `numerical_analysis.rs` (bisection, Newton, RK4, Gaussian elimination, 8 tests)
- [x] Game theory — `game_theory.rs` (Nash equilibrium, minimax, evolutionary games, 8 tests)
- [x] Homological algebra — `homological_algebra.rs` (chain complexes, Ext/Tor, spectral sequences, 8 tests)
- [x] Representation theory — `representation_theory.rs` (character theory, Schur's lemma, Maschke's theorem, 8 tests)
- [x] Universal algebra — `universal_algebra.rs` (varieties, Birkhoff's theorem, free algebras, 8 tests)
- [x] Lattice theory — `lattice_theory.rs` (distributive lattices, Boolean algebras, Galois connections, 8 tests)
- [x] Set theory ZFC — `set_theory_zfc.rs` (ZFC axioms, ordinals, cardinals, Zorn, 8 tests)
- [x] Combinatorial game theory — `combinatorial_game_theory.rs` (Nim, Sprague-Grundy, surreal numbers, 8 tests)
- [x] Coding theory — `coding_theory.rs` (Hamming codes, Reed-Solomon, Shannon capacity, 8 tests)
- [x] Point-set topology — `point_set_topology.rs` (metric spaces, separation axioms, compactness, 8 tests)
- [x] Mathematical physics — `mathematical_physics.rs` (Lagrangian/Hamiltonian, Maxwell, GR foundations, 8 tests)
- [x] Convex optimization — `convex_optimization.rs` (gradient descent, ADMM, KKT conditions, 8 tests)
- [x] Operations research — `operations_research.rs` (network flows, queueing, scheduling, DP, 8 tests)

## v0.1.3 Lemma Bundles for Automation Tactics

> Last updated: 2026-05-29

### Ring 0

- [x] Add `OmegaHelper` lemma bundle module to oxilean-std (2026-05-29)
  - **Goal:** Provide axiom-backed `ConstantInfo::Theorem` declarations for integer arithmetic lemmas that the omega proof reconstruction compiler can cite by name.
  - **Design:** New module `src/omega_helper/` with `register_omega_helper(env: &mut Environment) -> Result<(), EnvError>`. Declarations (using `Expr::Const`, `Expr::App`, `Expr::Pi`, `Expr::Lam`, `BinderInfo::Default`, `Level::zero()`): `Int.le_refl : ∀(a:Int), a ≤ a`; `Int.le_trans : ∀(a b c:Int), a ≤ b → b ≤ c → a ≤ c`; `Int.le_antisymm : ∀(a b:Int), a ≤ b → b ≤ a → a = b`; `Int.lt_irrefl : ∀(a:Int), ¬(a<a)`; `Int.lt_iff_add_one_le : ∀(a b:Int), (a<b) ↔ (a+1≤b)`; `Int.le_of_lt : ∀(a b:Int), a<b → a≤b`; `Int.add_le_add : ∀(a b c d:Int), a≤b → c≤d → a+c≤b+d`; `Int.mul_le_mul_of_nonneg_left : ∀(a b c:Int), a≤b → 0≤c → c*a≤c*b`; `Int.le_of_eq`, `Int.le_total`. First check if any already exist via `env.contains(Name::str("Int.le_refl"))`; skip duplicates. Register `register_omega_helper` in the std environment aggregator (`src/lib.rs` or `src/functions.rs`).
  - **Files:** `src/omega_helper/mod.rs`, `src/omega_helper/types.rs` (Expr builder helpers), `src/lib.rs` (aggregator registration)
  - **Prerequisites:** None (uses oxilean-kernel Expr/ConstantInfo/Environment APIs directly)
  - **Tests:** `register_omega_helper(&mut env)` succeeds; `env.find(Name::str("Int.le_refl"))` returns Some; TypeChecker can infer the type of each lemma without error (10+ test assertions).
  - **Risk:** Expr construction for dependent Pi types is verbose but mechanical. The exact Int type name must match what the kernel uses — read oxilean-kernel/src/expr/types.rs to confirm before coding.

### Ring 1

- [x] `linarith_helper` module — Int-ordered arithmetic lemmas for linarith (2026-05-30)
- [x] `polyrith_helper` module — polynomial ring axioms for polyrith (2026-05-30)
- [x] `cc_helper` module — congruence lemmas for cc (2026-05-30)
- [x] omega_helper: strict-inequality transitivity lemmas for nlinarith Farkas (cycle 6) (done 2026-05-29)
  - **Goal:** Add `Int.lt_of_le_of_lt`, `Int.lt_of_lt_of_le`, `Int.lt_trans` to `register_omega_helper` (14 → 17 axioms) so the elab-side Farkas builder can fold transitivity cycles with strict edges and close via `Int.lt_irrefl`. Make `add_omega_lemmas` `pub` and re-export from `lib.rs` so the CLI production env can register the bundle.
  - **Design:** Mirror existing `ty_*` builder fns (correct de Bruijn indices; `Int.lt`/`Int.le` as Const apps). Confirm `Int.lt_irrefl : Not (Int.lt a a)` δ-reduces to `→ False`; if `Not` is opaque in this env, add an explicit `Int.lt_irrefl' : ∀ a, Int.lt a a → False` variant.
  - **Files:** `crates/oxilean-std/src/omega_helper/mod.rs`, `crates/oxilean-std/src/env_builder/functions_2.rs` (pub add_omega_lemmas), `crates/oxilean-std/src/lib.rs` (re-export).
  - **Tests:** each new axiom present + well-formed; count assertion → 17; idempotency; `add_omega_lemmas` reachable as a public symbol.
- [x] LE.le ↔ Int.le bridge axioms + production env wiring (done 2026-05-29)
  - **Goal:** Add `le_of_int_le` and `int_le_of_le` bridge axioms to `omega_helper/mod.rs` (axiom-backed, consistent with existing 10); wire `register_omega_helper` into the production env-builder path so lemmas are available in real (non-test) elaboration; ensure `absurd`/`False.elim` present in the same production env.
  - **Design:** `le_of_int_le : ∀ {inst : LE Int} (a b : Int), Int.le a b → LE.le Int inst a b` and reverse; add to the `register_omega_helper` batch; locate the real env-builder entry point (`build_full_std_env` or upstream caller) and call `register_omega_helper(env)?` there.
  - **Files:** `src/omega_helper/mod.rs`, `src/env_builder/functions_2.rs` (or whichever calls the env build in production)
  - **Prerequisites:** None (omega_helper module already exists)
  - **Tests:** Bridge axioms present + well-formed after registration; production env build returns env containing all 12 omega lemmas + `absurd`; idempotence test.
  - **Risk:** If production env build path differs from `build_full_std_env`, locate via grepping for the CLI/elaborator's env construction call.
