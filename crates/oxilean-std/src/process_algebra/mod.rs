//! # Process Algebra
//!
//! Process algebras provide mathematical frameworks for describing and reasoning about
//! concurrent and communicating systems. The two major calculi are:
//!
//! ## CCS — Calculus of Communicating Systems (Milner, 1980)
//!
//! CCS processes are defined by:
//! ```text
//! P ::= 0              -- nil (deadlock/stop)
//!     | α.P            -- action prefix (α ∈ Act)
//!     | P + Q          -- nondeterministic choice
//!     | P | Q          -- parallel composition
//!     | P\L            -- restriction (hide labels in L)
//!     | P\[f\]           -- relabeling by function f
//!     | X              -- process variable
//!     | μX.P           -- recursive definition
//! ```
//!
//! ## CSP — Communicating Sequential Processes (Hoare, 1978)
//!
//! CSP extends CCS with:
//! - **Synchronous parallel** `P ‖ Q` (must synchronize on shared events)
//! - **Alphabetized parallel** `P ‖_A Q`
//! - **Sequential composition** `P ; Q`
//! - **Interrupt** `P △ Q`
//! - **Failures/divergence** semantics
//!
//! ## Key Equivalences
//!
//! - **Bisimulation**: processes `P ~ Q` if they can simulate each other step-by-step
//! - **Weak bisimulation**: ignores internal τ-transitions
//! - **Trace equivalence**: processes accept the same sequences of visible actions
//! - **Failures equivalence**: traces + sets of refusals
//! - **Testing equivalence**: indistinguishable by all tests
//!
//! ## Hennessy-Milner Logic (HML)
//!
//! Modal logic for describing process properties:
//! ```text
//! φ ::= tt | ff | φ ∧ ψ | φ ∨ ψ | ¬φ | ⟨α⟩φ | \[α\]φ
//! ```
//! - `⟨α⟩φ`: can do α and then satisfy φ (diamond modality)
//! - `[α]φ`: every α-transition leads to a state satisfying φ (box modality)

pub mod functions;
pub mod types;

pub use functions::*;
pub use types::*;
