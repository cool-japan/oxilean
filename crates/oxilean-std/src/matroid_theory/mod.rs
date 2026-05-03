//! # Matroid Theory
//!
//! A matroid is an ordered pair `(E, I)` where `E` is a finite ground set and `I` is a
//! collection of *independent sets* satisfying:
//!
//! 1. **I1** (emptiness): ∅ ∈ I
//! 2. **I2** (hereditary): If A ∈ I and B ⊆ A, then B ∈ I
//! 3. **I3** (augmentation): If A, B ∈ I and |A| < |B|, then ∃ x ∈ B\A such that A ∪ {x} ∈ I
//!
//! ## Key Structures
//!
//! - **Uniform matroids** `U(k, n)`: independent sets are all subsets of size ≤ k
//! - **Graphic matroids** `M(G)`: independent sets are forests of a graph G
//! - **Linear matroids**: independent sets are linearly independent sets of vectors
//! - **Transversal matroids**: independent sets are matchings in a bipartite graph
//! - **Partition matroids**: each element drawn from a disjoint partition with local rank caps
//!
//! ## Key Theorems
//!
//! - **Greedy algorithm**: The greedy algorithm finds a maximum-weight basis
//! - **Matroid intersection**: Two matroids' intersection (common independent sets) can be
//!   optimized in polynomial time via the matroid intersection algorithm
//! - **Matroid union theorem**: The union of k matroids has a well-defined rank function
//! - **Dual matroid**: Every matroid has a dual with bases = complements of bases of the original
//! - **Whitney's theorem**: Graphic matroid `M(G)` determines the graph G (up to 3-connectivity)

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
