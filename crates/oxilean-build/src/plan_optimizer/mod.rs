//! # Build Plan Optimizer
//!
//! Optimizes the build dependency graph for maximum parallelism.
//!
//! Given a directed acyclic graph (DAG) of build targets described by a
//! [`BuildPlan`], this module provides:
//!
//! - [`critical_path`] — computes the longest weighted path through the DAG,
//!   identifying the true bottleneck regardless of available parallelism.
//! - [`schedule`] — greedily assigns nodes to a fixed pool of workers so as
//!   to minimize the estimated makespan, respecting all dependency ordering
//!   constraints.
//!
//! ## Example
//!
//! ```rust
//! use oxilean_build::plan_optimizer::{BuildNode, BuildPlan, critical_path, schedule};
//!
//! let nodes = vec![
//!     BuildNode::new(0, "parse",   100, vec![]),
//!     BuildNode::new(1, "elab",    200, vec![0]),
//!     BuildNode::new(2, "codegen", 150, vec![1]),
//!     BuildNode::new(3, "link",     50, vec![2]),
//! ];
//! let plan = BuildPlan::from_nodes(nodes);
//!
//! let cp = critical_path(&plan);
//! assert_eq!(cp, vec![0, 1, 2, 3]);
//!
//! let sched = schedule(&plan, 4);
//! println!("Makespan: {} ms", sched.estimated_makespan_ms);
//! ```

pub mod functions;
pub mod types;

#[cfg(test)]
mod tests;

pub use functions::*;
pub use types::*;
