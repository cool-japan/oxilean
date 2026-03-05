//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::basic::MVarId;
use oxilean_kernel::{Expr, Level, Name};
use std::collections::HashMap;

/// Information about one constructor rule in a recursor.
#[derive(Clone, Debug)]
pub(super) struct ConstructorRuleInfo {
    /// Constructor name.
    pub(super) ctor_name: Name,
    /// Number of fields.
    pub(super) num_fields: u32,
    /// Number of recursive fields.
    pub(super) num_recursive: u32,
}
/// A single minor premise (constructor case) in an induction scheme.
///
/// Each minor premise corresponds to one constructor of the inductive type.
/// It records the constructor, its arity, recursive argument count, and the
/// expected type for the proof obligation.
#[derive(Clone, Debug)]
pub struct MinorPremise {
    /// User-facing name for this case (`zero`, `succ`, `nil`, `cons`, ...).
    pub name: Name,
    /// Fully qualified constructor name (`Nat.zero`, `Nat.succ`, ...).
    pub ctor_name: Name,
    /// Number of non-parameter fields introduced by this constructor.
    pub num_fields: usize,
    /// Number of recursive arguments (fields whose type mentions the inductive).
    pub num_recursive_args: usize,
    /// Expected type of the proof obligation (motive applied to constructor args).
    pub expected_type: Expr,
    /// Names of the fields (used for variable introduction).
    pub field_names: Vec<Name>,
    /// Indices of recursive arguments among the fields.
    pub recursive_arg_indices: Vec<usize>,
    /// Whether this constructor has any non-recursive fields.
    pub has_non_recursive_fields: bool,
}
impl MinorPremise {
    /// Create a minimal minor premise.
    pub fn new(ctor_name: Name, num_fields: usize, num_recursive: usize) -> Self {
        let short_name = extract_short_name(&ctor_name);
        let field_names: Vec<Name> = (0..num_fields)
            .map(|i| Name::str(format!("{}_{}", short_name, i)))
            .collect();
        let recursive_arg_indices: Vec<usize> = if num_recursive > 0 && num_fields > 0 {
            ((num_fields - num_recursive)..num_fields).collect()
        } else {
            Vec::new()
        };
        Self {
            name: Name::str(short_name.clone()),
            ctor_name,
            num_fields,
            num_recursive_args: num_recursive,
            expected_type: Expr::Sort(Level::zero()),
            field_names,
            recursive_arg_indices,
            has_non_recursive_fields: num_fields > num_recursive,
        }
    }
    /// Total number of binders this case introduces (fields + IHs).
    pub fn total_binders(&self) -> usize {
        self.num_fields + self.num_recursive_args
    }
    /// Generate default induction hypothesis names for this case.
    pub fn default_ih_names(&self) -> Vec<Name> {
        (0..self.num_recursive_args)
            .map(|i| {
                if self.num_recursive_args == 1 {
                    Name::str("ih")
                } else {
                    Name::str(format!("ih_{}", i + 1))
                }
            })
            .collect()
    }
}
/// Simplified recursor info fetched from the environment.
#[derive(Clone, Debug)]
pub(super) struct RecursorInfo {
    /// All inductive types in the mutual block.
    pub(super) all_names: Vec<Name>,
    /// Number of parameters.
    pub(super) num_params: u32,
    /// Number of indices.
    pub(super) num_indices: u32,
    /// Universe levels.
    pub(super) universe_levels: Vec<Level>,
    /// Constructor rules.
    pub(super) constructor_rules: Vec<ConstructorRuleInfo>,
    /// Custom major premise index (if different from standard layout).
    pub(super) custom_major_idx: Option<u32>,
}
/// Configuration for an advanced induction tactic invocation.
#[derive(Clone, Debug)]
pub struct InductionConfig {
    /// Names of hypotheses to generalize before induction.
    pub generalizing: Vec<Name>,
    /// Explicit recursor to use (overrides the default `T.rec`).
    pub using_recursor: Option<Name>,
    /// User-supplied names for the variables in each constructor case.
    /// Outer vec: one entry per constructor. Inner vec: names for that case.
    pub with_names: Vec<Vec<Name>>,
    /// Whether to automatically revert hypotheses that depend on the target.
    pub revert_deps: bool,
    /// Whether to clear the induction target from the context after applying.
    pub clear_target: bool,
    /// Additional `simp` lemmas to use when simplifying induction hypotheses.
    pub simp_lemmas: Vec<Name>,
    /// Maximum recursion depth for complex scheme inference.
    pub max_depth: u32,
}
impl InductionConfig {
    /// Create a configuration that only generalizes the given names.
    pub fn generalizing(names: Vec<Name>) -> Self {
        Self {
            generalizing: names,
            ..Default::default()
        }
    }
    /// Create a configuration that uses a specific recursor.
    pub fn using(recursor: Name) -> Self {
        Self {
            using_recursor: Some(recursor),
            ..Default::default()
        }
    }
    /// Set user-supplied names for constructor cases.
    pub fn with_names(mut self, names: Vec<Vec<Name>>) -> Self {
        self.with_names = names;
        self
    }
    /// Whether any generalization is requested.
    pub fn has_generalization(&self) -> bool {
        !self.generalizing.is_empty()
    }
    /// Whether a custom recursor is specified.
    pub fn has_custom_recursor(&self) -> bool {
        self.using_recursor.is_some()
    }
}
/// Configuration for mutual induction across multiple targets.
#[derive(Clone, Debug)]
pub struct MutualInductionConfig {
    /// Per-target configurations.
    pub target_configs: Vec<InductionConfig>,
    /// Names of the targets being inducted on.
    pub target_names: Vec<Name>,
    /// Whether to use a single combined recursor or coordinate separate ones.
    pub use_combined_recursor: bool,
    /// User-supplied names for goals produced by each motive.
    pub motive_names: Vec<Vec<Name>>,
    /// Shared generalization list (applied to all targets).
    pub shared_generalizing: Vec<Name>,
    /// Maximum number of mutual inductive types supported.
    pub max_mutual: usize,
}
impl MutualInductionConfig {
    /// Create a mutual config for the given target names.
    pub fn for_targets(names: Vec<Name>) -> Self {
        let configs = names.iter().map(|_| InductionConfig::default()).collect();
        Self {
            target_configs: configs,
            target_names: names,
            ..Default::default()
        }
    }
    /// Number of targets.
    pub fn num_targets(&self) -> usize {
        self.target_names.len()
    }
    /// Check whether the number of targets exceeds the maximum.
    pub fn exceeds_limit(&self) -> bool {
        self.target_names.len() > self.max_mutual
    }
}
/// Configuration for well-founded induction.
#[derive(Clone, Debug)]
pub struct WellFoundedConfig {
    /// The well-founded relation to use.
    pub relation: Option<Expr>,
    /// Name of the well-founded proof (`h_wf`).
    pub wf_proof_name: Option<Name>,
    /// Decreasing measure function (for measure-based WF induction).
    pub measure: Option<Expr>,
    /// Whether to attempt automatic measure inference.
    pub auto_measure: bool,
    /// User-supplied names for the induction hypotheses.
    pub ih_names: Vec<Name>,
    /// Maximum depth for termination proof search.
    pub max_depth: u32,
    /// Whether to use the `SizeOf` approach for termination.
    pub use_sizeof: bool,
}
impl WellFoundedConfig {
    /// Create a WF config with an explicit relation expression.
    pub fn with_relation(rel: Expr) -> Self {
        Self {
            relation: Some(rel),
            ..Default::default()
        }
    }
    /// Create a WF config with a measure function.
    pub fn with_measure(measure: Expr) -> Self {
        Self {
            measure: Some(measure),
            auto_measure: false,
            ..Default::default()
        }
    }
    /// Whether a specific relation has been provided.
    pub fn has_explicit_relation(&self) -> bool {
        self.relation.is_some()
    }
    /// Whether a measure function has been provided.
    pub fn has_measure(&self) -> bool {
        self.measure.is_some()
    }
}
/// Describes an induction scheme (recursor application layout).
///
/// An induction scheme fully specifies how a recursor is applied to a target:
/// the parameter positions, index positions, motives, and minor premises
/// (one per constructor).
#[derive(Clone, Debug)]
pub struct InductionScheme {
    /// Fully qualified name of the recursor (`Nat.rec`, `List.rec`, etc.).
    pub recursor: Name,
    /// Position of the major premise (the value being eliminated) in the
    /// recursor's argument list.
    pub major_idx: usize,
    /// Number of type parameters (uniform across all constructors).
    pub num_params: usize,
    /// Number of type indices (may vary per constructor).
    pub num_indices: usize,
    /// Motive expressions — one per mutual inductive in the block.
    pub motives: Vec<Expr>,
    /// Minor premises — one per constructor.
    pub minor_premises: Vec<MinorPremise>,
    /// Universe levels for the recursor application.
    pub universe_levels: Vec<Level>,
    /// Whether this scheme was inferred or user-specified.
    pub is_custom: bool,
    /// Name of the inductive type this scheme targets.
    pub inductive_name: Name,
    /// All inductive types in the mutual block.
    pub mutual_names: Vec<Name>,
}
impl InductionScheme {
    /// Create a new induction scheme for a simple (non-mutual) inductive.
    pub fn new_simple(
        recursor: Name,
        inductive_name: Name,
        major_idx: usize,
        num_params: usize,
        num_indices: usize,
    ) -> Self {
        Self {
            recursor,
            major_idx,
            num_params,
            num_indices,
            motives: Vec::new(),
            minor_premises: Vec::new(),
            universe_levels: vec![Level::zero()],
            is_custom: false,
            inductive_name: inductive_name.clone(),
            mutual_names: vec![inductive_name],
        }
    }
    /// Total number of arguments that the recursor expects before the major premise.
    pub fn args_before_major(&self) -> usize {
        self.num_params + self.motives.len() + self.minor_premises.len() + self.num_indices
    }
    /// Total number of goals this scheme will produce.
    pub fn num_goals(&self) -> usize {
        self.minor_premises.len()
    }
    /// Return the constructor names in declaration order.
    pub fn constructor_names(&self) -> Vec<Name> {
        self.minor_premises
            .iter()
            .map(|mp| mp.ctor_name.clone())
            .collect()
    }
    /// Check whether this is a mutual induction scheme.
    pub fn is_mutual(&self) -> bool {
        self.mutual_names.len() > 1
    }
    /// Get the minor premise for a given constructor, if present.
    pub fn find_minor(&self, ctor: &Name) -> Option<&MinorPremise> {
        self.minor_premises.iter().find(|mp| &mp.ctor_name == ctor)
    }
}
/// Result of a generalization pass.
///
/// Records which hypotheses were reverted and any new intermediate goals
/// produced.
#[derive(Clone, Debug)]
pub struct GeneralizationResult {
    /// Names of the hypotheses that were reverted.
    pub reverted: Vec<Name>,
    /// New goal (the generalized one) that replaces the original.
    pub new_goal: MVarId,
    /// The generalized target type (with reverted hyps as Pi-binders).
    pub generalized_type: Expr,
    /// Number of extra binders added by generalization.
    pub num_generalized: usize,
    /// Mapping from original hypothesis names to their positions in the binder telescope.
    pub hyp_positions: HashMap<Name, usize>,
    /// Expressions that were generalized (for re-introduction after induction).
    pub generalized_exprs: Vec<(Expr, Name)>,
}
impl GeneralizationResult {
    /// Whether any hypotheses were actually generalized.
    pub fn is_trivial(&self) -> bool {
        self.reverted.is_empty()
    }
    /// Number of Pi-binders that were added.
    pub fn num_binders(&self) -> usize {
        self.num_generalized
    }
}
