//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, FVarId, Level, Literal, Name};
use std::collections::HashMap;

use super::functions::*;

/// Extended derivable class enum supporting custom user classes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdvDerivableClass {
    /// Boolean equality — structural field-by-field comparison.
    BEq,
    /// Decidable equality — proof-producing version of BEq.
    DecidableEq,
    /// Hashable — compute a hash from constructor tag and fields.
    Hashable,
    /// Ordering — lexicographic comparison of fields.
    Ord,
    /// String representation — human-readable format.
    Repr,
    /// Inhabited — type has a default element.
    Inhabited,
    /// Nonempty — at least one element exists (propositional).
    Nonempty,
    /// ToString — convert to string via Repr.
    ToString,
    /// Custom user-defined derivable class.
    Custom(Name),
}
impl AdvDerivableClass {
    /// Return the canonical name of this class.
    pub fn class_name(&self) -> Name {
        match self {
            AdvDerivableClass::BEq => Name::str("BEq"),
            AdvDerivableClass::DecidableEq => Name::str("DecidableEq"),
            AdvDerivableClass::Hashable => Name::str("Hashable"),
            AdvDerivableClass::Ord => Name::str("Ord"),
            AdvDerivableClass::Repr => Name::str("Repr"),
            AdvDerivableClass::Inhabited => Name::str("Inhabited"),
            AdvDerivableClass::Nonempty => Name::str("Nonempty"),
            AdvDerivableClass::ToString => Name::str("ToString"),
            AdvDerivableClass::Custom(n) => n.clone(),
        }
    }
    /// Parse a class name string to an `AdvDerivableClass`.
    pub fn from_name(name: &str) -> AdvDerivableClass {
        match name {
            "BEq" => AdvDerivableClass::BEq,
            "DecidableEq" => AdvDerivableClass::DecidableEq,
            "Hashable" => AdvDerivableClass::Hashable,
            "Ord" => AdvDerivableClass::Ord,
            "Repr" => AdvDerivableClass::Repr,
            "Inhabited" => AdvDerivableClass::Inhabited,
            "Nonempty" => AdvDerivableClass::Nonempty,
            "ToString" => AdvDerivableClass::ToString,
            other => AdvDerivableClass::Custom(Name::str(other)),
        }
    }
    /// Check whether this is a built-in (non-custom) class.
    pub fn is_builtin(&self) -> bool {
        !matches!(self, AdvDerivableClass::Custom(_))
    }
    /// List all built-in derivable classes.
    pub fn all_builtins() -> Vec<AdvDerivableClass> {
        vec![
            AdvDerivableClass::BEq,
            AdvDerivableClass::DecidableEq,
            AdvDerivableClass::Hashable,
            AdvDerivableClass::Ord,
            AdvDerivableClass::Repr,
            AdvDerivableClass::Inhabited,
            AdvDerivableClass::Nonempty,
            AdvDerivableClass::ToString,
        ]
    }
}
/// Extended type information for advanced derivation.
///
/// Carries additional metadata compared to the basic `TypeInfo`:
/// - `is_inductive` / `is_structure` flags
/// - Universe parameters
/// - Type parameters with binder info
#[derive(Debug, Clone)]
pub struct TypeInfoAdv {
    /// Fully qualified type name.
    pub name: Name,
    /// Universe parameters.
    pub univ_params: Vec<Name>,
    /// Type parameters: `(name, type, binder_info)`.
    pub params: Vec<(Name, Expr, BinderInfo)>,
    /// Constructors.
    pub constructors: Vec<CtorInfo>,
    /// Whether this is a genuine inductive type (not just a structure).
    pub is_inductive: bool,
    /// Whether this is a structure (single-constructor inductive).
    pub is_structure: bool,
    /// Whether the type is recursive.
    pub is_recursive: bool,
    /// Number of index arguments.
    pub num_indices: usize,
}
impl TypeInfoAdv {
    /// Create a new `TypeInfoAdv`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Name,
        univ_params: Vec<Name>,
        params: Vec<(Name, Expr, BinderInfo)>,
        constructors: Vec<CtorInfo>,
        is_inductive: bool,
        is_structure: bool,
        is_recursive: bool,
        num_indices: usize,
    ) -> Self {
        Self {
            name,
            univ_params,
            params,
            constructors,
            is_inductive,
            is_structure,
            is_recursive,
            num_indices,
        }
    }
    /// Number of constructors.
    pub fn num_constructors(&self) -> usize {
        self.constructors.len()
    }
    /// Check if all constructors are nullary (enum-like).
    pub fn is_enum(&self) -> bool {
        !self.constructors.is_empty() && self.constructors.iter().all(|c| c.is_nullary())
    }
    /// Total number of fields across all constructors.
    pub fn total_fields(&self) -> usize {
        self.constructors.iter().map(|c| c.num_fields()).sum()
    }
    /// Check if the type has exactly one constructor.
    pub fn is_single_ctor(&self) -> bool {
        self.constructors.len() == 1
    }
    /// Get the first constructor, if any.
    pub fn first_ctor(&self) -> Option<&CtorInfo> {
        self.constructors.first()
    }
    /// Build the type expression for this type (no universe instantiation).
    pub fn type_expr(&self) -> Expr {
        Expr::Const(self.name.clone(), vec![])
    }
    /// Build the type expression with universe level parameters.
    pub fn type_expr_with_levels(&self) -> Expr {
        let levels: Vec<Level> = self
            .univ_params
            .iter()
            .map(|p| Level::Param(p.clone()))
            .collect();
        Expr::Const(self.name.clone(), levels)
    }
    /// Build a fully applied type expression (applying parameters).
    pub fn applied_type_expr(&self) -> Expr {
        let mut result = self.type_expr_with_levels();
        for (i, _) in self.params.iter().enumerate() {
            let param_var = Expr::BVar((self.params.len() - 1 - i) as u32);
            result = Expr::App(Box::new(result), Box::new(param_var));
        }
        result
    }
    /// Collect all field types across all constructors.
    pub fn all_field_types(&self) -> Vec<&Expr> {
        self.constructors
            .iter()
            .flat_map(|c| c.field_types())
            .collect()
    }
}
/// Ordering result for comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordering {
    Less,
    Equal,
    Greater,
}
/// Handler for deriving `BEq` instances.
///
/// Strategy: For each constructor, compare fields pairwise using `BEq.beq`
/// and chain with `&&`. Different constructors yield `false`.
pub struct BEqHandler;
impl BEqHandler {
    /// Create a new BEq handler.
    pub fn new() -> Self {
        BEqHandler
    }
}
/// Handler for deriving `Repr` instances.
///
/// Strategy: For each constructor, produce `"CtorName field1 field2 ..."`.
pub struct ReprHandler;
impl ReprHandler {
    /// Create a new Repr handler.
    pub fn new() -> Self {
        ReprHandler
    }
}
/// Handler for deriving `Inhabited` instances.
///
/// Strategy: Pick the first constructor and use `default` for each field.
pub struct InhabitedHandler;
impl InhabitedHandler {
    /// Create a new Inhabited handler.
    pub fn new() -> Self {
        InhabitedHandler
    }
}
/// The result of a successful advanced derivation.
#[derive(Debug, Clone)]
pub struct AdvDeriveResult {
    /// The instance expression (the value of the instance declaration).
    pub instance_expr: Expr,
    /// Auxiliary declarations generated alongside the instance:
    /// `(name, type, value)`.
    pub aux_decls: Vec<(Name, Expr, Expr)>,
    /// Name of the instance.
    pub instance_name: Name,
    /// Type of the instance.
    pub instance_type: Expr,
}
impl AdvDeriveResult {
    /// Create a new derive result.
    pub fn new(
        instance_name: Name,
        instance_type: Expr,
        instance_expr: Expr,
        aux_decls: Vec<(Name, Expr, Expr)>,
    ) -> Self {
        Self {
            instance_expr,
            aux_decls,
            instance_name,
            instance_type,
        }
    }
    /// Check if there are auxiliary declarations.
    pub fn has_aux_decls(&self) -> bool {
        !self.aux_decls.is_empty()
    }
    /// Number of auxiliary declarations.
    pub fn num_aux_decls(&self) -> usize {
        self.aux_decls.len()
    }
}
/// Handler for deriving `Hashable` instances.
///
/// Strategy: Hash the constructor tag, then mix in the hash of each field.
pub struct HashableHandler;
impl HashableHandler {
    /// Create a new Hashable handler.
    pub fn new() -> Self {
        HashableHandler
    }
}
/// Handler for deriving `Nonempty` instances.
///
/// Strategy: Construct a witness using the first constructor.
pub struct NonemptyHandler;
impl NonemptyHandler {
    /// Create a new Nonempty handler.
    pub fn new() -> Self {
        NonemptyHandler
    }
}
/// Errors during advanced derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdvDeriveError {
    /// The class cannot be derived for this type.
    CannotDerive {
        /// Class name.
        class: String,
        /// Type name.
        type_name: String,
        /// Reason.
        reason: String,
    },
    /// A required instance is missing for a field type.
    MissingFieldInstance {
        /// Class name.
        class: String,
        /// Field name.
        field: String,
        /// Field type description.
        field_type: String,
    },
    /// Recursive types are not supported by this class.
    RecursiveType {
        /// Class name.
        class: String,
        /// Type name.
        type_name: String,
    },
    /// The type has no constructors.
    EmptyType {
        /// Type name.
        type_name: String,
    },
    /// No handler registered for the class.
    NoHandler {
        /// Class name.
        class: String,
    },
    /// Internal error.
    Internal(String),
}
/// Registry of derive handlers keyed by class name.
///
/// The registry manages a collection of `DeriveHandler` implementations
/// and provides methods to register new handlers, check derivability,
/// and perform derivation.
pub struct AdvDeriveRegistry {
    /// Handlers indexed by class name.
    handlers: HashMap<Name, Box<dyn DeriveHandler>>,
    /// Order of registration (for deterministic iteration).
    order: Vec<Name>,
}
impl AdvDeriveRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            order: Vec::new(),
        }
    }
    /// Create a registry pre-populated with all built-in handlers.
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register_handler(Box::new(BEqHandler::new()));
        registry.register_handler(Box::new(DecidableEqHandler::new()));
        registry.register_handler(Box::new(HashableHandler::new()));
        registry.register_handler(Box::new(OrdHandler::new()));
        registry.register_handler(Box::new(ReprHandler::new()));
        registry.register_handler(Box::new(InhabitedHandler::new()));
        registry.register_handler(Box::new(NonemptyHandler::new()));
        registry.register_handler(Box::new(ToStringHandler::new()));
        registry
    }
    /// Register a handler. Replaces any existing handler for the same class.
    pub fn register_handler(&mut self, handler: Box<dyn DeriveHandler>) {
        let name = handler.class_name();
        if !self.handlers.contains_key(&name) {
            self.order.push(name.clone());
        }
        self.handlers.insert(name, handler);
    }
    /// Check if a handler is registered for the given class.
    pub fn has_handler(&self, class: &Name) -> bool {
        self.handlers.contains_key(class)
    }
    /// Check if a class can be derived for a type.
    pub fn can_derive(&self, class: &Name, type_info: &TypeInfoAdv) -> bool {
        self.handlers
            .get(class)
            .is_some_and(|h| h.can_derive(type_info))
    }
    /// Try to derive a single class for a type.
    pub fn try_derive(
        &self,
        class: &AdvDerivableClass,
        type_info: &TypeInfoAdv,
    ) -> Result<AdvDeriveResult, AdvDeriveError> {
        let class_name = class.class_name();
        match self.handlers.get(&class_name) {
            Some(handler) => {
                if !handler.can_derive(type_info) {
                    return Err(AdvDeriveError::CannotDerive {
                        class: format!("{}", class_name),
                        type_name: format!("{}", type_info.name),
                        reason: "handler reports type is not derivable".to_string(),
                    });
                }
                handler.derive(type_info)
            }
            None => Err(AdvDeriveError::NoHandler {
                class: format!("{}", class_name),
            }),
        }
    }
    /// Derive multiple classes for a type. Returns results for all that succeed.
    pub fn derive_many(
        &self,
        classes: &[AdvDerivableClass],
        type_info: &TypeInfoAdv,
    ) -> Vec<Result<AdvDeriveResult, AdvDeriveError>> {
        classes
            .iter()
            .map(|cls| self.try_derive(cls, type_info))
            .collect()
    }
    /// Derive all registered classes for a type, returning only successes.
    pub fn derive_all_possible(&self, type_info: &TypeInfoAdv) -> Vec<AdvDeriveResult> {
        self.order
            .iter()
            .filter_map(|name| {
                self.handlers.get(name).and_then(|handler| {
                    if handler.can_derive(type_info) {
                        handler.derive(type_info).ok()
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
    /// Number of registered handlers.
    pub fn num_handlers(&self) -> usize {
        self.handlers.len()
    }
    /// Get the names of all registered classes in registration order.
    pub fn class_names(&self) -> &[Name] {
        &self.order
    }
}
/// Handler for deriving `ToString` instances.
///
/// Strategy: Delegates to `Repr` — `toString a = reprStr a`.
pub struct ToStringHandler;
impl ToStringHandler {
    /// Create a new ToString handler.
    pub fn new() -> Self {
        ToStringHandler
    }
}
/// Handler for deriving `Ord` instances.
///
/// Strategy: Compare constructor tags first. For same constructor,
/// compare fields lexicographically. Return `Ordering`.
pub struct OrdHandler;
impl OrdHandler {
    /// Create a new Ord handler.
    pub fn new() -> Self {
        OrdHandler
    }
}
/// Information about a single constructor for advanced derivation.
#[derive(Debug, Clone)]
pub struct CtorInfo {
    /// Fully qualified constructor name.
    pub name: Name,
    /// Ordered fields: `(field_name, field_type)`.
    pub fields: Vec<(Name, Expr)>,
}
impl CtorInfo {
    /// Create a new constructor info.
    pub fn new(name: Name, fields: Vec<(Name, Expr)>) -> Self {
        Self { name, fields }
    }
    /// Number of fields.
    pub fn num_fields(&self) -> usize {
        self.fields.len()
    }
    /// Whether the constructor is nullary (no fields).
    pub fn is_nullary(&self) -> bool {
        self.fields.is_empty()
    }
    /// Get field names.
    pub fn field_names(&self) -> Vec<&Name> {
        self.fields.iter().map(|(n, _)| n).collect()
    }
    /// Get field types.
    pub fn field_types(&self) -> Vec<&Expr> {
        self.fields.iter().map(|(_, t)| t).collect()
    }
    /// Look up a field type by name.
    pub fn field_type(&self, name: &Name) -> Option<&Expr> {
        self.fields.iter().find(|(n, _)| n == name).map(|(_, t)| t)
    }
}
/// Handler for deriving `DecidableEq` instances.
///
/// Strategy: Similar to BEq, but produces `Decidable (a = b)` proofs.
/// Same constructor: chain pairwise decidable equality checks.
/// Different constructor: `isFalse noConfusion`.
pub struct DecidableEqHandler;
impl DecidableEqHandler {
    /// Create a new DecidableEq handler.
    pub fn new() -> Self {
        DecidableEqHandler
    }
}
