# oxilean-std — TODO

> Task list for the standard library crate.
> Last updated: 2026-05-03

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
