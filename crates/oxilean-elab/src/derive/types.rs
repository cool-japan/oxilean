//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, Literal, Name};
use std::collections::HashMap;

use super::functions::*;

/// Analysis of a constructor's fields for derivation purposes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FieldAnalysis {
    /// Constructor name.
    pub ctor_name: Name,
    /// Number of fields.
    pub num_fields: usize,
    /// Whether all fields are primitive types (Nat, String, Bool).
    pub all_primitive: bool,
    /// Field names.
    pub field_names: Vec<Name>,
}
#[allow(dead_code)]
impl FieldAnalysis {
    /// Analyze a constructor.
    pub fn analyze(info: &ConstructorInfo) -> Self {
        let field_names: Vec<Name> = info.fields.iter().map(|(n, _)| n.clone()).collect();
        let all_primitive = info.fields.iter().all(|(_, ty)| is_primitive_type(ty));
        Self {
            ctor_name: info.name.clone(),
            num_fields: info.fields.len(),
            all_primitive,
            field_names,
        }
    }
    /// Return true if this constructor is unit-like (no fields).
    pub fn is_unit(&self) -> bool {
        self.num_fields == 0
    }
    /// Return true if this constructor has exactly one field.
    pub fn is_newtype(&self) -> bool {
        self.num_fields == 1
    }
}
/// Automatic instance deriver.
pub struct Deriver {
    /// Enable debug output.
    debug: bool,
}
impl Deriver {
    /// Create a new deriver.
    pub fn new() -> Self {
        Self { debug: false }
    }
    /// Derive an instance for a type (old API, kept for compatibility).
    ///
    /// This API cannot call the rich [`Self::derive_with_info`] pipeline because it
    /// only receives a kernel `Expr` for the type, not the full [`TypeInfo`]
    /// required to inspect constructors and fields.  Callers should migrate to
    /// [`Self::derive_with_info`] instead.
    ///
    /// Returns an error describing the unsupported path.
    pub fn derive(&self, class: DerivableClass, ty: &Expr) -> Result<Expr, String> {
        if !self.can_derive(class, ty) {
            return Err(format!(
                "class {:?} is not in the set of auto-derivable classes",
                class
            ));
        }
        Err(
            format!(
                "derive() old API cannot derive {:?} without TypeInfo; use derive_with_info() with a fully constructed TypeInfo instead",
                class
            ),
        )
    }
    /// Check if a type class is in the set of auto-derivable classes (old API).
    ///
    /// Returns `true` for every class handled by [`Self::derive_with_info`].
    pub fn can_derive(&self, class: DerivableClass, _ty: &Expr) -> bool {
        matches!(
            class,
            DerivableClass::BEq
                | DerivableClass::Eq
                | DerivableClass::Repr
                | DerivableClass::Show
                | DerivableClass::Hashable
                | DerivableClass::Inhabited
                | DerivableClass::Default
                | DerivableClass::DecidableEq
                | DerivableClass::Nonempty
                | DerivableClass::ToString
                | DerivableClass::Ord
        )
    }
    /// Enable debug output.
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    /// Check if debug is enabled.
    pub fn is_debug(&self) -> bool {
        self.debug
    }
    /// Derive a class instance given full type information.
    #[allow(dead_code)]
    pub fn derive_with_info(
        &self,
        class: DerivableClass,
        type_info: &TypeInfo,
    ) -> Result<DeriveResult, DerivationError> {
        match class {
            DerivableClass::BEq => self.derive_beq(type_info),
            DerivableClass::Repr => self.derive_repr(type_info),
            DerivableClass::Hashable => self.derive_hashable(type_info),
            DerivableClass::Inhabited => self.derive_inhabited(type_info),
            DerivableClass::DecidableEq => self.derive_decidable_eq(type_info),
            DerivableClass::Nonempty => self.derive_nonempty(type_info),
            DerivableClass::ToString => self.derive_to_string(type_info),
            DerivableClass::Eq => self.derive_beq(type_info),
            DerivableClass::Ord => self.derive_ord(type_info),
            DerivableClass::Show => self.derive_repr(type_info),
            DerivableClass::Default => self.derive_inhabited(type_info),
        }
    }
    /// Derive a `BEq` instance.
    ///
    /// For each pair of constructors:
    /// - Same constructor: compare fields pairwise with `BEq.beq`.
    /// - Different constructors: return `false`.
    #[allow(dead_code)]
    pub fn derive_beq(&self, type_info: &TypeInfo) -> Result<DeriveResult, DerivationError> {
        if type_info.is_recursive {
            return Err(DerivationError::RecursiveType(format!(
                "Cannot derive BEq for recursive type {}",
                type_info.name
            )));
        }
        let ty_expr = Expr::Const(type_info.name.clone(), vec![]);
        let mut match_arms: Vec<Expr> = Vec::new();
        for ctor in &type_info.constructors {
            if ctor.fields.is_empty() {
                match_arms.push(mk_bool_lit(true));
            } else {
                let comparisons: Vec<Expr> = ctor
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, (_, field_ty))| {
                        mk_field_comparison(field_ty, &mk_lhs_field_var(i), &mk_rhs_field_var(i))
                    })
                    .collect();
                match_arms.push(mk_and_chain(&comparisons));
            }
        }
        if type_info.constructors.len() > 1 {
            match_arms.push(mk_bool_lit(false));
        }
        let body = build_match_body(&match_arms);
        let beq_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(ty_expr.clone()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(ty_expr.clone()),
                Box::new(body),
            )),
        );
        let instance_name = Name::str(format!("inst_BEq_{}", type_info.name));
        let instance_type = Expr::App(
            Box::new(Expr::Const(Name::str("BEq"), vec![])),
            Box::new(ty_expr),
        );
        Ok(DeriveResult {
            instance_name,
            instance_type,
            instance_body: beq_lam,
            aux_defs: vec![],
        })
    }
    /// Derive a `Repr` instance.
    ///
    /// Formats constructor name followed by field representations.
    #[allow(dead_code)]
    pub fn derive_repr(&self, type_info: &TypeInfo) -> Result<DeriveResult, DerivationError> {
        let ty_expr = Expr::Const(type_info.name.clone(), vec![]);
        let mut match_arms: Vec<Expr> = Vec::new();
        for ctor in &type_info.constructors {
            let field_reprs: Vec<Expr> = ctor
                .fields
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    Expr::App(
                        Box::new(Expr::Const(Name::str("repr"), vec![])),
                        Box::new(mk_lhs_field_var(i)),
                    )
                })
                .collect();
            match_arms.push(mk_repr_string(&ctor.name, &field_reprs));
        }
        let body = build_match_body(&match_arms);
        let repr_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(ty_expr.clone()),
            Box::new(body),
        );
        let instance_name = Name::str(format!("inst_Repr_{}", type_info.name));
        let instance_type = Expr::App(
            Box::new(Expr::Const(Name::str("Repr"), vec![])),
            Box::new(ty_expr),
        );
        Ok(DeriveResult {
            instance_name,
            instance_type,
            instance_body: repr_lam,
            aux_defs: vec![],
        })
    }
    /// Derive a `Hashable` instance.
    ///
    /// Hashes the constructor tag followed by each field.
    #[allow(dead_code)]
    pub fn derive_hashable(&self, type_info: &TypeInfo) -> Result<DeriveResult, DerivationError> {
        if type_info.is_recursive {
            return Err(DerivationError::RecursiveType(format!(
                "Cannot derive Hashable for recursive type {}",
                type_info.name
            )));
        }
        let ty_expr = Expr::Const(type_info.name.clone(), vec![]);
        let mut match_arms: Vec<Expr> = Vec::new();
        for (tag, ctor) in type_info.constructors.iter().enumerate() {
            let tag_hash = Expr::App(
                Box::new(Expr::Const(Name::str("hash"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(tag as u64))),
            );
            let field_hashes: Vec<Expr> = ctor
                .fields
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    Expr::App(
                        Box::new(Expr::Const(Name::str("hash"), vec![])),
                        Box::new(mk_lhs_field_var(i)),
                    )
                })
                .collect();
            let mut all_hashes = vec![tag_hash];
            all_hashes.extend(field_hashes);
            match_arms.push(mk_hash_combine(&all_hashes));
        }
        let body = build_match_body(&match_arms);
        let hash_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(ty_expr.clone()),
            Box::new(body),
        );
        let instance_name = Name::str(format!("inst_Hashable_{}", type_info.name));
        let instance_type = Expr::App(
            Box::new(Expr::Const(Name::str("Hashable"), vec![])),
            Box::new(ty_expr),
        );
        Ok(DeriveResult {
            instance_name,
            instance_type,
            instance_body: hash_lam,
            aux_defs: vec![],
        })
    }
    /// Derive an `Inhabited` instance.
    ///
    /// Uses the first constructor whose fields all have `Inhabited` instances.
    #[allow(dead_code)]
    pub fn derive_inhabited(&self, type_info: &TypeInfo) -> Result<DeriveResult, DerivationError> {
        if type_info.constructors.is_empty() {
            return Err(DerivationError::CannotDerive(format!(
                "Type {} has no constructors",
                type_info.name
            )));
        }
        let ty_expr = Expr::Const(type_info.name.clone(), vec![]);
        let ctor = &type_info.constructors[0];
        let mut body: Expr = Expr::Const(ctor.name.clone(), vec![]);
        for (_, field_ty) in &ctor.fields {
            let default_val = Expr::App(
                Box::new(Expr::Const(Name::str("default"), vec![])),
                Box::new(field_ty.clone()),
            );
            body = Expr::App(Box::new(body), Box::new(default_val));
        }
        let instance_name = Name::str(format!("inst_Inhabited_{}", type_info.name));
        let instance_type = Expr::App(
            Box::new(Expr::Const(Name::str("Inhabited"), vec![])),
            Box::new(ty_expr),
        );
        Ok(DeriveResult {
            instance_name,
            instance_type,
            instance_body: body,
            aux_defs: vec![],
        })
    }
    /// Derive a `DecidableEq` instance.
    ///
    /// Similar to BEq but returns `Decidable (a = b)` proof.
    #[allow(dead_code)]
    pub fn derive_decidable_eq(
        &self,
        type_info: &TypeInfo,
    ) -> Result<DeriveResult, DerivationError> {
        if type_info.is_recursive {
            return Err(DerivationError::RecursiveType(format!(
                "Cannot derive DecidableEq for recursive type {}",
                type_info.name
            )));
        }
        let ty_expr = Expr::Const(type_info.name.clone(), vec![]);
        let mut match_arms: Vec<Expr> = Vec::new();
        for ctor in &type_info.constructors {
            if ctor.fields.is_empty() {
                match_arms.push(Expr::App(
                    Box::new(Expr::Const(Name::str("Decidable.isTrue"), vec![])),
                    Box::new(Expr::Const(Name::str("rfl"), vec![])),
                ));
            } else {
                let comparisons: Vec<Expr> = ctor
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, (_, field_ty))| {
                        Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("decEq"), vec![])),
                                Box::new(field_ty.clone()),
                            )),
                            Box::new(Expr::App(
                                Box::new(mk_lhs_field_var(i)),
                                Box::new(mk_rhs_field_var(i)),
                            )),
                        )
                    })
                    .collect();
                match_arms.push(mk_decidable_and_chain(&comparisons));
            }
        }
        if type_info.constructors.len() > 1 {
            match_arms.push(Expr::App(
                Box::new(Expr::Const(Name::str("Decidable.isFalse"), vec![])),
                Box::new(Expr::Const(Name::str("noConfusion"), vec![])),
            ));
        }
        let body = build_match_body(&match_arms);
        let dec_eq_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(ty_expr.clone()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(ty_expr.clone()),
                Box::new(body),
            )),
        );
        let instance_name = Name::str(format!("inst_DecidableEq_{}", type_info.name));
        let instance_type = Expr::App(
            Box::new(Expr::Const(Name::str("DecidableEq"), vec![])),
            Box::new(ty_expr),
        );
        Ok(DeriveResult {
            instance_name,
            instance_type,
            instance_body: dec_eq_lam,
            aux_defs: vec![],
        })
    }
    /// Derive a `Nonempty` instance.
    ///
    /// Simply constructs an element using the first constructor.
    #[allow(dead_code)]
    pub fn derive_nonempty(&self, type_info: &TypeInfo) -> Result<DeriveResult, DerivationError> {
        if type_info.constructors.is_empty() {
            return Err(DerivationError::CannotDerive(format!(
                "Type {} has no constructors",
                type_info.name
            )));
        }
        let ty_expr = Expr::Const(type_info.name.clone(), vec![]);
        let ctor = &type_info.constructors[0];
        let mut witness: Expr = Expr::Const(ctor.name.clone(), vec![]);
        for (_, field_ty) in &ctor.fields {
            let default_val = Expr::App(
                Box::new(Expr::Const(Name::str("default"), vec![])),
                Box::new(field_ty.clone()),
            );
            witness = Expr::App(Box::new(witness), Box::new(default_val));
        }
        let body = Expr::App(
            Box::new(Expr::Const(Name::str("Nonempty.intro"), vec![])),
            Box::new(witness),
        );
        let instance_name = Name::str(format!("inst_Nonempty_{}", type_info.name));
        let instance_type = Expr::App(
            Box::new(Expr::Const(Name::str("Nonempty"), vec![])),
            Box::new(ty_expr),
        );
        Ok(DeriveResult {
            instance_name,
            instance_type,
            instance_body: body,
            aux_defs: vec![],
        })
    }
    /// Derive a `ToString` instance.
    ///
    /// Delegates to `Repr` and wraps the result.
    #[allow(dead_code)]
    pub fn derive_to_string(&self, type_info: &TypeInfo) -> Result<DeriveResult, DerivationError> {
        let ty_expr = Expr::Const(type_info.name.clone(), vec![]);
        let body = Expr::Lam(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(ty_expr.clone()),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("reprStr"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
        );
        let instance_name = Name::str(format!("inst_ToString_{}", type_info.name));
        let instance_type = Expr::App(
            Box::new(Expr::Const(Name::str("ToString"), vec![])),
            Box::new(ty_expr),
        );
        Ok(DeriveResult {
            instance_name,
            instance_type,
            instance_body: body,
            aux_defs: vec![],
        })
    }
    /// Derive an `Ord` instance.
    ///
    /// Generates a `compare` function that orders values by:
    /// 1. Constructor tag (earlier constructors are `Less`).
    /// 2. Fields compared lexicographically left-to-right using `Ord.compare`.
    ///
    /// Does not support recursive types.
    #[allow(dead_code)]
    pub fn derive_ord(&self, type_info: &TypeInfo) -> Result<DeriveResult, DerivationError> {
        if type_info.is_recursive {
            return Err(DerivationError::RecursiveType(format!(
                "Cannot derive Ord for recursive type {}",
                type_info.name
            )));
        }
        let ty_expr = Expr::Const(type_info.name.clone(), vec![]);
        let num_ctors = type_info.constructors.len();
        let mut outer_arms: Vec<Expr> = Vec::new();
        for (i, ctor_a) in type_info.constructors.iter().enumerate() {
            let mut inner_arms: Vec<Expr> = Vec::new();
            for (j, _ctor_b) in type_info.constructors.iter().enumerate() {
                if i == j {
                    let arm = if ctor_a.fields.is_empty() {
                        Expr::Const(Name::str("Ordering.eq"), vec![])
                    } else {
                        mk_lex_field_compare(&ctor_a.fields)
                    };
                    inner_arms.push(arm);
                } else if i < j {
                    inner_arms.push(Expr::Const(Name::str("Ordering.lt"), vec![]));
                } else {
                    inner_arms.push(Expr::Const(Name::str("Ordering.gt"), vec![]));
                }
            }
            let inner_body = if num_ctors == 1 {
                inner_arms
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| Expr::Const(Name::str("Ordering.eq"), vec![]))
            } else {
                build_match_body(&inner_arms)
            };
            outer_arms.push(inner_body);
        }
        let body = if num_ctors == 1 {
            outer_arms
                .into_iter()
                .next()
                .unwrap_or_else(|| Expr::Const(Name::str("Ordering.eq"), vec![]))
        } else {
            build_match_body(&outer_arms)
        };
        let compare_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(ty_expr.clone()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(ty_expr.clone()),
                Box::new(body),
            )),
        );
        let instance_name = Name::str(format!("inst_Ord_{}", type_info.name));
        let instance_type = Expr::App(
            Box::new(Expr::Const(Name::str("Ord"), vec![])),
            Box::new(ty_expr),
        );
        Ok(DeriveResult {
            instance_name,
            instance_type,
            instance_body: compare_lam,
            aux_defs: vec![],
        })
    }
}
/// Derivable type classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DerivableClass {
    /// Equality (structural).
    Eq,
    /// Ordering.
    Ord,
    /// Show/Display.
    Show,
    /// Default values.
    Default,
    /// Boolean equality.
    BEq,
    /// String representation.
    Repr,
    /// Hashable.
    Hashable,
    /// Inhabited (has a default element).
    Inhabited,
    /// Decidable equality (proof-producing).
    DecidableEq,
    /// Nonempty (at least one element exists).
    Nonempty,
    /// ToString.
    ToString,
}
impl DerivableClass {
    /// Return the canonical name of this class.
    #[allow(dead_code)]
    pub fn class_name(&self) -> Name {
        match self {
            DerivableClass::Eq => Name::str("Eq"),
            DerivableClass::Ord => Name::str("Ord"),
            DerivableClass::Show => Name::str("Show"),
            DerivableClass::Default => Name::str("Default"),
            DerivableClass::BEq => Name::str("BEq"),
            DerivableClass::Repr => Name::str("Repr"),
            DerivableClass::Hashable => Name::str("Hashable"),
            DerivableClass::Inhabited => Name::str("Inhabited"),
            DerivableClass::DecidableEq => Name::str("DecidableEq"),
            DerivableClass::Nonempty => Name::str("Nonempty"),
            DerivableClass::ToString => Name::str("ToString"),
        }
    }
    /// Parse a class name string to a `DerivableClass`.
    #[allow(dead_code)]
    pub fn from_name(name: &str) -> Option<DerivableClass> {
        match name {
            "Eq" => Some(DerivableClass::Eq),
            "Ord" => Some(DerivableClass::Ord),
            "Show" => Some(DerivableClass::Show),
            "Default" => Some(DerivableClass::Default),
            "BEq" => Some(DerivableClass::BEq),
            "Repr" => Some(DerivableClass::Repr),
            "Hashable" => Some(DerivableClass::Hashable),
            "Inhabited" => Some(DerivableClass::Inhabited),
            "DecidableEq" => Some(DerivableClass::DecidableEq),
            "Nonempty" => Some(DerivableClass::Nonempty),
            "ToString" => Some(DerivableClass::ToString),
            _ => None,
        }
    }
}
/// Additional derivable type classes beyond the core set.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdditionalDerivableClass {
    /// Functor (map).
    Functor,
    /// Foldable.
    Foldable,
    /// Traversable.
    Traversable,
    /// Monad.
    Monad,
    /// Applicative.
    Applicative,
    /// Semigroup.
    Semigroup,
    /// Monoid.
    Monoid,
    /// Read (parse from string).
    Read,
    /// Enum (enumeration).
    Enum,
    /// Bounded (min/max values).
    Bounded,
}
#[allow(dead_code)]
impl AdditionalDerivableClass {
    /// Return the canonical name for this class.
    pub fn class_name(&self) -> Name {
        match self {
            AdditionalDerivableClass::Functor => Name::str("Functor"),
            AdditionalDerivableClass::Foldable => Name::str("Foldable"),
            AdditionalDerivableClass::Traversable => Name::str("Traversable"),
            AdditionalDerivableClass::Monad => Name::str("Monad"),
            AdditionalDerivableClass::Applicative => Name::str("Applicative"),
            AdditionalDerivableClass::Semigroup => Name::str("Semigroup"),
            AdditionalDerivableClass::Monoid => Name::str("Monoid"),
            AdditionalDerivableClass::Read => Name::str("Read"),
            AdditionalDerivableClass::Enum => Name::str("Enum"),
            AdditionalDerivableClass::Bounded => Name::str("Bounded"),
        }
    }
    /// Return whether this class requires the type to be an enum.
    pub fn requires_enum(&self) -> bool {
        matches!(
            self,
            AdditionalDerivableClass::Enum | AdditionalDerivableClass::Bounded
        )
    }
    /// Return whether this class requires a type parameter.
    pub fn requires_type_param(&self) -> bool {
        matches!(
            self,
            AdditionalDerivableClass::Functor
                | AdditionalDerivableClass::Foldable
                | AdditionalDerivableClass::Traversable
                | AdditionalDerivableClass::Monad
                | AdditionalDerivableClass::Applicative
        )
    }
}
/// High-level analysis of a type for derivation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypeAnalysis {
    /// The type being analysed.
    pub type_name: Name,
    /// Number of constructors.
    pub num_ctors: usize,
    /// Whether the type is an enum (all constructors have no fields).
    pub is_enum: bool,
    /// Whether the type is a newtype (single constructor, single field).
    pub is_newtype: bool,
    /// Whether the type is a unit type (single unit-like constructor).
    pub is_unit: bool,
    /// Whether the type is recursive.
    pub is_recursive: bool,
    /// Per-constructor field analysis.
    pub field_analyses: Vec<FieldAnalysis>,
}
#[allow(dead_code)]
impl TypeAnalysis {
    /// Analyse a `TypeInfo`.
    pub fn analyze(ti: &TypeInfo) -> Self {
        let field_analyses: Vec<FieldAnalysis> =
            ti.constructors.iter().map(FieldAnalysis::analyze).collect();
        let num_ctors = ti.constructors.len();
        let is_enum = ti.is_enum();
        let is_unit =
            num_ctors == 1 && field_analyses.first().map(|f| f.is_unit()).unwrap_or(false);
        let is_newtype = num_ctors == 1
            && field_analyses
                .first()
                .map(|f| f.is_newtype())
                .unwrap_or(false);
        Self {
            type_name: ti.name.clone(),
            num_ctors,
            is_enum,
            is_newtype,
            is_unit,
            is_recursive: ti.is_recursive,
            field_analyses,
        }
    }
    /// Return the classes that are definitely derivable for this type.
    pub fn definitely_derivable_classes(&self) -> Vec<DerivableClass> {
        let mut classes = vec![DerivableClass::Repr, DerivableClass::ToString];
        if !self.is_recursive {
            classes.push(DerivableClass::BEq);
            classes.push(DerivableClass::Hashable);
            classes.push(DerivableClass::DecidableEq);
            classes.push(DerivableClass::Ord);
        }
        if self.num_ctors > 0 {
            classes.push(DerivableClass::Inhabited);
            classes.push(DerivableClass::Nonempty);
        }
        classes
    }
}
/// Statistics about a batch derivation run.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DeriveStats {
    /// Number of derivations attempted.
    pub attempted: usize,
    /// Number of derivations that succeeded.
    pub succeeded: usize,
    /// Number of derivations that failed.
    pub failed: usize,
    /// Number of auxiliary definitions produced.
    pub aux_defs_produced: usize,
}
#[allow(dead_code)]
impl DeriveStats {
    /// Create empty stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Return the success rate.
    pub fn success_rate(&self) -> f64 {
        if self.attempted == 0 {
            1.0
        } else {
            self.succeeded as f64 / self.attempted as f64
        }
    }
    /// Record a success.
    pub fn record_success(&mut self, result: &DeriveResult) {
        self.attempted += 1;
        self.succeeded += 1;
        self.aux_defs_produced += result.aux_defs.len();
    }
    /// Record a failure.
    pub fn record_failure(&mut self) {
        self.attempted += 1;
        self.failed += 1;
    }
}
/// Context for derivation: caches previously derived instances.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DerivationContext {
    /// Cache from (class_name, type_name) to DeriveResult.
    cache: HashMap<(Name, Name), DeriveResult>,
    /// Number of cache hits.
    pub cache_hits: usize,
    /// Number of cache misses.
    pub cache_misses: usize,
}
#[allow(dead_code)]
impl DerivationContext {
    /// Create a new empty context.
    pub fn new() -> Self {
        Self::default()
    }
    /// Look up a cached result.
    pub fn lookup(&mut self, class: &Name, ty: &Name) -> Option<&DeriveResult> {
        let key = (class.clone(), ty.clone());
        if self.cache.contains_key(&key) {
            self.cache_hits += 1;
            self.cache.get(&key)
        } else {
            self.cache_misses += 1;
            None
        }
    }
    /// Store a result in the cache.
    pub fn store(&mut self, class: Name, ty: Name, result: DeriveResult) {
        self.cache.insert((class, ty), result);
    }
    /// Return the cache hit rate (0.0 to 1.0).
    pub fn hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
    }
}
/// Generates instance names for derived instances.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceNamer {
    /// Prefix to prepend before the class name.
    pub prefix: String,
    /// Separator between prefix, class, and type names.
    pub separator: String,
}
#[allow(dead_code)]
impl InstanceNamer {
    /// Create a default namer using `inst_<Class>_<Type>`.
    pub fn default_namer() -> Self {
        Self {
            prefix: "inst".to_string(),
            separator: "_".to_string(),
        }
    }
    /// Create a custom namer.
    pub fn new(prefix: impl Into<String>, separator: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            separator: separator.into(),
        }
    }
    /// Generate the instance name.
    pub fn name_for(&self, class: DerivableClass, type_name: &Name) -> Name {
        let class_n = format!("{}", class.class_name());
        let type_n = format!("{}", type_name);
        Name::str(format!(
            "{}{}{}{}{}",
            self.prefix, self.separator, class_n, self.separator, type_n
        ))
    }
    /// Generate the instance name for a string class name.
    pub fn name_for_str(&self, class_str: &str, type_name: &Name) -> Name {
        Name::str(format!(
            "{}{}{}{}{}",
            self.prefix, self.separator, class_str, self.separator, type_name
        ))
    }
}
/// Specialized deriver that generates structural equality functions.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct StructuralEqDeriver;
#[allow(dead_code)]
impl StructuralEqDeriver {
    /// Create a new structural equality deriver.
    pub fn new() -> Self {
        Self
    }
    /// Generate a structural equality function for the given type.
    ///
    /// Returns an `Expr` representing:
    ///   `fun (a b : T) -> beq a b`
    pub fn derive_structural_eq(&self, type_info: &TypeInfo) -> Result<Expr, DerivationError> {
        if type_info.constructors.is_empty() {
            return Err(DerivationError::CannotDerive(format!(
                "Type {} has no constructors; structural equality is vacuously false",
                type_info.name
            )));
        }
        let ty_expr = Expr::Const(type_info.name.clone(), vec![]);
        let body = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("BEq.beq"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
            Box::new(Expr::BVar(0)),
        );
        Ok(Expr::Lam(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(ty_expr.clone()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(ty_expr),
                Box::new(body),
            )),
        ))
    }
    /// Derive the number of constructors as a constant expression.
    pub fn ctor_count_expr(&self, type_info: &TypeInfo) -> Expr {
        Expr::Lit(Literal::Nat(type_info.constructors.len() as u64))
    }
}
/// Derives multiple classes for multiple types in one pass.
#[allow(dead_code)]
pub struct BatchDeriver {
    /// The underlying deriver.
    deriver: Deriver,
    /// Collected statistics.
    pub stats: DeriveStats,
}
#[allow(dead_code)]
impl BatchDeriver {
    /// Create a new batch deriver.
    pub fn new() -> Self {
        Self {
            deriver: Deriver::new(),
            stats: DeriveStats::new(),
        }
    }
    /// Derive a class for a slice of types, collecting all results.
    pub fn derive_for_all(
        &mut self,
        class: DerivableClass,
        type_infos: &[TypeInfo],
    ) -> Vec<Result<DeriveResult, DerivationError>> {
        type_infos
            .iter()
            .map(|ti| {
                let r = self.deriver.derive_with_info(class, ti);
                match &r {
                    Ok(res) => self.stats.record_success(res),
                    Err(_) => self.stats.record_failure(),
                }
                r
            })
            .collect()
    }
    /// Derive multiple classes for a single type.
    pub fn derive_classes_for(
        &mut self,
        classes: &[DerivableClass],
        type_info: &TypeInfo,
    ) -> Vec<Result<DeriveResult, DerivationError>> {
        classes
            .iter()
            .map(|&class| {
                let r = self.deriver.derive_with_info(class, type_info);
                match &r {
                    Ok(res) => self.stats.record_success(res),
                    Err(_) => self.stats.record_failure(),
                }
                r
            })
            .collect()
    }
    /// Derive all standard derivable classes for a type (best-effort).
    pub fn derive_all_standard(&mut self, type_info: &TypeInfo) -> Vec<DeriveResult> {
        let classes = [
            DerivableClass::BEq,
            DerivableClass::Repr,
            DerivableClass::Inhabited,
            DerivableClass::Nonempty,
            DerivableClass::ToString,
        ];
        classes
            .iter()
            .filter_map(|&c| {
                let r = self.deriver.derive_with_info(c, type_info);
                match r {
                    Ok(res) => {
                        self.stats.record_success(&res);
                        Some(res)
                    }
                    Err(_) => {
                        self.stats.record_failure();
                        None
                    }
                }
            })
            .collect()
    }
}
/// Represents a `#derive` command in OxiLean source code.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeriveCommand {
    /// Type name to derive for.
    pub type_name: Name,
    /// Class names to derive.
    pub class_names: Vec<String>,
}
#[allow(dead_code)]
impl DeriveCommand {
    /// Create a new derive command.
    pub fn new(type_name: Name, class_names: Vec<String>) -> Self {
        Self {
            type_name,
            class_names,
        }
    }
    /// Parse class names into `DerivableClass` values.
    pub fn parse_classes(&self) -> Vec<Option<DerivableClass>> {
        self.class_names
            .iter()
            .map(|n| DerivableClass::from_name(n))
            .collect()
    }
    /// Return class names that are unknown (cannot be parsed).
    pub fn unknown_classes(&self) -> Vec<&str> {
        self.class_names
            .iter()
            .filter(|n| DerivableClass::from_name(n).is_none())
            .map(String::as_str)
            .collect()
    }
    /// Return true if all requested classes are known.
    pub fn all_known(&self) -> bool {
        self.unknown_classes().is_empty()
    }
}
/// Errors that can occur during derivation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivationError {
    /// The class cannot be derived for the given type.
    CannotDerive(String),
    /// A required type class instance is missing.
    MissingInstance(String),
    /// The type is recursive and the class does not support recursion.
    RecursiveType(String),
    /// Other error.
    Other(String),
}
/// Information about an inductive type being derived.
#[derive(Debug, Clone)]
pub struct TypeInfo {
    /// Fully qualified name of the type.
    pub name: Name,
    /// Universe parameters.
    pub univ_params: Vec<Name>,
    /// Type parameters: `(name, type, binder_info)`.
    pub params: Vec<(Name, Expr, BinderInfo)>,
    /// Constructors of the type.
    pub constructors: Vec<ConstructorInfo>,
    /// Whether the type is recursive.
    pub is_recursive: bool,
    /// Number of index arguments.
    pub num_indices: usize,
}
impl TypeInfo {
    /// Create a new type info.
    #[allow(dead_code)]
    pub fn new(
        name: Name,
        univ_params: Vec<Name>,
        params: Vec<(Name, Expr, BinderInfo)>,
        constructors: Vec<ConstructorInfo>,
        is_recursive: bool,
        num_indices: usize,
    ) -> Self {
        Self {
            name,
            univ_params,
            params,
            constructors,
            is_recursive,
            num_indices,
        }
    }
    /// Return the number of constructors.
    #[allow(dead_code)]
    pub fn num_constructors(&self) -> usize {
        self.constructors.len()
    }
    /// Check if the type is an enum (all constructors nullary).
    #[allow(dead_code)]
    pub fn is_enum(&self) -> bool {
        self.constructors.iter().all(|c| c.is_nullary())
    }
    /// Return the total number of fields across all constructors.
    #[allow(dead_code)]
    pub fn total_fields(&self) -> usize {
        self.constructors.iter().map(|c| c.num_fields()).sum()
    }
}
/// A plan for deriving multiple class instances for a type.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DerivationPlan {
    /// Type to derive for.
    pub type_info: TypeInfo,
    /// Ordered list of classes to derive.
    pub classes: Vec<DerivableClass>,
    /// Whether to abort on first error.
    pub fail_fast: bool,
}
#[allow(dead_code)]
impl DerivationPlan {
    /// Create a new plan.
    pub fn new(type_info: TypeInfo, classes: Vec<DerivableClass>) -> Self {
        Self {
            type_info,
            classes,
            fail_fast: true,
        }
    }
    /// Set fail_fast behaviour.
    pub fn with_fail_fast(mut self, v: bool) -> Self {
        self.fail_fast = v;
        self
    }
    /// Execute the plan, returning all results and/or errors.
    pub fn execute(&self) -> (Vec<DeriveResult>, Vec<DerivationError>) {
        let deriver = Deriver::new();
        let mut results = Vec::new();
        let mut errors = Vec::new();
        for &class in &self.classes {
            match deriver.derive_with_info(class, &self.type_info) {
                Ok(r) => results.push(r),
                Err(e) => {
                    errors.push(e);
                    if self.fail_fast {
                        break;
                    }
                }
            }
        }
        (results, errors)
    }
    /// Return the number of classes planned.
    pub fn num_classes(&self) -> usize {
        self.classes.len()
    }
}
/// Information about a single constructor of an inductive type.
#[derive(Debug, Clone)]
pub struct ConstructorInfo {
    /// Constructor name (e.g. `Color.red`).
    pub name: Name,
    /// Ordered fields: `(field_name, field_type)`.
    pub fields: Vec<(Name, Expr)>,
    /// Number of type parameters (shared with the inductive).
    pub num_params: usize,
}
impl ConstructorInfo {
    /// Create a new constructor info.
    #[allow(dead_code)]
    pub fn new(name: Name, fields: Vec<(Name, Expr)>, num_params: usize) -> Self {
        Self {
            name,
            fields,
            num_params,
        }
    }
    /// Return the number of fields (excluding parameters).
    #[allow(dead_code)]
    pub fn num_fields(&self) -> usize {
        self.fields.len()
    }
    /// Check if this is a nullary constructor (no fields).
    #[allow(dead_code)]
    pub fn is_nullary(&self) -> bool {
        self.fields.is_empty()
    }
}
/// The result of a successful derivation.
#[derive(Debug, Clone)]
pub struct DeriveResult {
    /// Name of the generated instance.
    pub instance_name: Name,
    /// Type of the instance.
    pub instance_type: Expr,
    /// Body (value) of the instance.
    pub instance_body: Expr,
    /// Auxiliary definitions generated alongside the instance.
    pub aux_defs: Vec<(Name, Expr, Expr)>,
}
/// Registry of derivable classes.
pub struct DeriveRegistry {
    /// Registered derivable classes.
    classes: Vec<DerivableClass>,
    /// Custom derivers keyed by class name.
    custom_derivers: HashMap<Name, CustomDeriverFn>,
}
impl DeriveRegistry {
    /// Create a new registry with default classes.
    pub fn new() -> Self {
        Self {
            classes: vec![
                DerivableClass::Eq,
                DerivableClass::Ord,
                DerivableClass::Show,
                DerivableClass::Default,
                DerivableClass::BEq,
                DerivableClass::Repr,
                DerivableClass::Hashable,
                DerivableClass::Inhabited,
                DerivableClass::DecidableEq,
                DerivableClass::Nonempty,
                DerivableClass::ToString,
            ],
            custom_derivers: HashMap::new(),
        }
    }
    /// Check if a class is derivable.
    pub fn is_derivable(&self, class: DerivableClass) -> bool {
        self.classes.contains(&class)
    }
    /// Get all derivable classes.
    pub fn all_classes(&self) -> &[DerivableClass] {
        &self.classes
    }
    /// Register a custom deriver for a named class.
    #[allow(dead_code)]
    pub fn register_custom_deriver<F>(&mut self, class_name: Name, deriver_fn: F)
    where
        F: Fn(&TypeInfo) -> Result<DeriveResult, DerivationError> + Send + Sync + 'static,
    {
        self.custom_derivers
            .insert(class_name, Box::new(deriver_fn));
    }
    /// Check if a custom deriver exists for the given class name.
    #[allow(dead_code)]
    pub fn has_deriver(&self, class: &Name) -> bool {
        self.custom_derivers.contains_key(class)
    }
    /// Derive multiple classes for a type.
    #[allow(dead_code)]
    pub fn derive_all(
        &self,
        classes: &[DerivableClass],
        type_info: &TypeInfo,
    ) -> Result<Vec<DeriveResult>, DerivationError> {
        let deriver = Deriver::new();
        let mut results = Vec::new();
        for &class in classes {
            let class_name = class.class_name();
            if let Some(custom) = self.custom_derivers.get(&class_name) {
                results.push(custom(type_info)?);
            } else {
                results.push(deriver.derive_with_info(class, type_info)?);
            }
        }
        Ok(results)
    }
    /// Register a new derivable class.
    #[allow(dead_code)]
    pub fn register_class(&mut self, class: DerivableClass) {
        if !self.classes.contains(&class) {
            self.classes.push(class);
        }
    }
    /// Get the number of registered custom derivers.
    #[allow(dead_code)]
    pub fn num_custom_derivers(&self) -> usize {
        self.custom_derivers.len()
    }
}
