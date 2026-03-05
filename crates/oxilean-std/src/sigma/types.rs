//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use std::collections::BTreeMap;

/// A telescope — a sequence of dependent types, each depending on the previous.
/// The type at position `i` is a function from the product of all prior types to `Type`.
/// Represented here as a Vec of named field descriptors.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DependentTelescope {
    fields: Vec<(String, String)>,
}
#[allow(dead_code)]
impl DependentTelescope {
    /// Create an empty telescope.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }
    /// Append a new field to the telescope, extending the dependent context.
    pub fn extend(&self, name: impl Into<String>, ty_desc: impl Into<String>) -> Self {
        let mut fields = self.fields.clone();
        fields.push((name.into(), ty_desc.into()));
        Self { fields }
    }
    /// Return the number of fields (depth of the telescope).
    pub fn depth(&self) -> usize {
        self.fields.len()
    }
    /// Return the field names in order.
    pub fn field_names(&self) -> Vec<&str> {
        self.fields.iter().map(|(n, _)| n.as_str()).collect()
    }
    /// Check if the telescope contains a field with the given name.
    pub fn has_field(&self, name: &str) -> bool {
        self.fields.iter().any(|(n, _)| n == name)
    }
    /// Return the type description for a named field, if present.
    pub fn field_type(&self, name: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, t)| t.as_str())
    }
    /// Project out a sub-telescope by keeping only fields up to (and including) `depth`.
    pub fn truncate(&self, depth: usize) -> Self {
        Self {
            fields: self.fields[..self.fields.len().min(depth)].to_vec(),
        }
    }
}
/// A refinement-type value: a value together with a dynamically-checked proof tag.
/// The proof is stored as a `bool` result from a predicate closure so the type
/// can be constructed and inspected without heavyweight theorem-prover plumbing.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RefinementValue<T: Clone> {
    value: T,
    predicate_name: String,
    predicate_holds: bool,
}
#[allow(dead_code)]
impl<T: Clone> RefinementValue<T> {
    /// Construct a refinement value, evaluating the predicate immediately.
    pub fn new(
        value: T,
        predicate_name: impl Into<String>,
        predicate: impl Fn(&T) -> bool,
    ) -> Self {
        let holds = predicate(&value);
        Self {
            value,
            predicate_name: predicate_name.into(),
            predicate_holds: holds,
        }
    }
    /// Return a reference to the underlying value.
    pub fn val(&self) -> &T {
        &self.value
    }
    /// Return whether the predicate holds for this value.
    pub fn is_valid(&self) -> bool {
        self.predicate_holds
    }
    /// Name of the predicate used to refine this type.
    pub fn predicate_name(&self) -> &str {
        &self.predicate_name
    }
    /// Coerce to a different refinement type given a proof that the new predicate
    /// follows from the old one (encoded as a Rust closure for demo purposes).
    pub fn coerce(
        &self,
        new_name: impl Into<String>,
        implication: impl Fn(&T) -> bool,
    ) -> RefinementValue<T> {
        RefinementValue::new(self.value.clone(), new_name, implication)
    }
}
/// Existential pack: hides the type index, exposing only the interface type.
/// This models `∃ α, value : α` where the type is erased at the boundary.
#[allow(dead_code)]
pub struct ExistentialPack<Interface> {
    value: Box<dyn std::any::Any + Send + Sync>,
    interface: Interface,
    type_name: &'static str,
}
#[allow(dead_code)]
impl<Interface: Clone> ExistentialPack<Interface> {
    /// Pack a value of any type together with its interface projection.
    pub fn pack<T: std::any::Any + Send + Sync>(
        value: T,
        interface: Interface,
        type_name: &'static str,
    ) -> Self {
        Self {
            value: Box::new(value),
            interface,
            type_name,
        }
    }
    /// Access the sealed interface.
    pub fn interface(&self) -> &Interface {
        &self.interface
    }
    /// The Rust type name of the hidden implementation.
    pub fn hidden_type_name(&self) -> &'static str {
        self.type_name
    }
    /// Attempt to unpack if the hidden type is known statically.
    pub fn unpack<T: std::any::Any + Clone>(&self) -> Option<T> {
        self.value.downcast_ref::<T>().cloned()
    }
}
/// W-type node: a well-founded tree where each node has a label of type `A`
/// and a (possibly empty) family of children indexed by `B(label)`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WNode<A: Clone, B: Clone + Ord> {
    label: A,
    children: std::collections::BTreeMap<B, WNode<A, B>>,
}
#[allow(dead_code)]
impl<A: Clone, B: Clone + Ord> WNode<A, B> {
    /// Construct a leaf node (no children).
    pub fn leaf(label: A) -> Self {
        Self {
            label,
            children: std::collections::BTreeMap::new(),
        }
    }
    /// Construct an interior node with the given children.
    pub fn node(label: A, children: std::collections::BTreeMap<B, WNode<A, B>>) -> Self {
        Self { label, children }
    }
    /// Access the label of this node.
    pub fn label(&self) -> &A {
        &self.label
    }
    /// Number of immediate children.
    pub fn arity(&self) -> usize {
        self.children.len()
    }
    /// Compute the height of the W-tree (maximum depth).
    pub fn height(&self) -> usize {
        if self.children.is_empty() {
            0
        } else {
            1 + self
                .children
                .values()
                .map(|c| c.height())
                .max()
                .unwrap_or(0)
        }
    }
    /// Count the total number of nodes in the tree.
    pub fn size(&self) -> usize {
        1 + self.children.values().map(|c| c.size()).sum::<usize>()
    }
    /// Map a function over all node labels.
    pub fn map_labels<C: Clone>(&self, f: &impl Fn(&A) -> C) -> WNode<C, B> {
        WNode {
            label: f(&self.label),
            children: self
                .children
                .iter()
                .map(|(k, v)| (k.clone(), v.map_labels(f)))
                .collect(),
        }
    }
    /// Fold over the tree in a bottom-up manner.
    pub fn fold<R: Clone>(&self, f: &impl Fn(&A, Vec<R>) -> R) -> R {
        let child_results: Vec<R> = self.children.values().map(|c| c.fold(f)).collect();
        f(&self.label, child_results)
    }
}
/// Row type: a dictionary of named fields with types, supporting width subtyping.
/// Implements a lightweight record row that can be checked for subtype relationships.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RowRecord {
    fields: std::collections::BTreeMap<String, String>,
}
#[allow(dead_code)]
impl RowRecord {
    /// Empty row.
    pub fn empty() -> Self {
        Self {
            fields: std::collections::BTreeMap::new(),
        }
    }
    /// Extend the row with a new field.
    pub fn with(mut self, label: impl Into<String>, ty: impl Into<String>) -> Self {
        self.fields.insert(label.into(), ty.into());
        self
    }
    /// Check width subtyping: `self` is a subtype of `other` if `other`'s fields ⊆ `self`'s fields.
    pub fn is_width_subtype_of(&self, other: &RowRecord) -> bool {
        other
            .fields
            .keys()
            .all(|k| self.fields.contains_key(k.as_str()))
    }
    /// Check depth subtyping: every field in `other` appears in `self` with compatible type.
    /// Here "compatible" is modelled as equality of type descriptions (simplified).
    pub fn is_depth_subtype_of(&self, other: &RowRecord) -> bool {
        other.fields.iter().all(|(k, v)| {
            self.fields
                .get(k.as_str())
                .map(|tv| tv == v)
                .unwrap_or(false)
        })
    }
    /// Number of fields in this row.
    pub fn width(&self) -> usize {
        self.fields.len()
    }
    /// Restrict the row to a given set of field names.
    pub fn restrict(&self, labels: &[&str]) -> Self {
        Self {
            fields: labels
                .iter()
                .filter_map(|l| self.fields.get(*l).map(|v| ((*l).to_string(), v.clone())))
                .collect(),
        }
    }
    /// Merge two rows (fields from `other` override `self` on collision).
    pub fn merge(&self, other: &RowRecord) -> Self {
        let mut fields = self.fields.clone();
        for (k, v) in &other.fields {
            fields.insert(k.clone(), v.clone());
        }
        Self { fields }
    }
}
