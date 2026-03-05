//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Environment, Expr, FVarId, Level, Name};
use oxilean_parse::Decl;
use std::collections::HashMap;

/// A generated projection declaration.
#[derive(Clone, Debug)]
pub struct ProjectionDecl {
    /// Projection function name.
    pub name: Name,
    /// Name of the structure this projects from.
    pub struct_name: Name,
    /// Index of the field being projected.
    pub field_idx: usize,
    /// Type of the projection function.
    pub ty: Expr,
    /// Value (body) of the projection function.
    pub val: Expr,
}
/// A generated constructor declaration for the structure.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct ConstructorDecl {
    /// Constructor name (e.g., `Foo.mk`).
    pub name: Name,
    /// Name of the structure.
    pub struct_name: Name,
    /// Constructor type.
    pub ty: Expr,
    /// Number of fields.
    pub num_fields: usize,
}
/// Information about an elaborated structure.
#[derive(Clone, Debug)]
pub struct StructureInfo {
    /// Structure name.
    pub name: Name,
    /// Universe parameters.
    pub univ_params: Vec<Name>,
    /// Structure parameters (name, type, binder_info).
    pub params: Vec<(Name, Expr, BinderInfo)>,
    /// Fields in declaration order.
    pub fields: Vec<FieldInfo>,
    /// Parent structures (via `extends`).
    pub parent_structs: Vec<Name>,
    /// Whether this is a class (typeclass).
    pub is_class: bool,
    /// Constructor name (e.g., `Foo.mk`).
    pub ctor_name: Name,
    /// Alternate constructor name used for `mk`.
    pub mk_name: Name,
}
/// Information about a single field in a structure.
#[derive(Clone, Debug)]
pub struct FieldInfo {
    /// Field name.
    pub name: Name,
    /// Field type.
    pub ty: Expr,
    /// Binder info for the field parameter.
    pub binder_info: BinderInfo,
    /// Optional default value.
    pub default_val: Option<Expr>,
    /// Projection function name (e.g., `Foo.bar`).
    pub proj_name: Name,
    /// Field index within the structure (0-based).
    pub idx: usize,
    /// Whether this field was inherited from a parent.
    pub is_inherited: bool,
    /// Which parent this field came from (if inherited).
    pub from_parent: Option<Name>,
}
/// Describes the result of flattening an inheritance chain.
#[derive(Clone, Debug, Default)]
pub struct FlattenedStructure {
    /// All fields in order (including inherited ones).
    pub fields: Vec<FieldInfo>,
    /// Source structure for each field (self or a parent name).
    pub field_sources: HashMap<Name, Name>,
    /// All parent structures in topological order.
    pub ancestors: Vec<Name>,
}
impl FlattenedStructure {
    /// Create an empty flattened structure.
    pub fn new() -> Self {
        Self::default()
    }
    /// Return all field names.
    pub fn field_names(&self) -> Vec<&Name> {
        self.fields.iter().map(|f| &f.name).collect()
    }
    /// Look up a field by name.
    pub fn get_field(&self, name: &Name) -> Option<&FieldInfo> {
        self.fields.iter().find(|f| &f.name == name)
    }
    /// Return `true` if `name` is an inherited field.
    pub fn is_inherited(&self, name: &Name) -> bool {
        self.field_sources
            .get(name)
            .map(|src| src != &Name::str("self"))
            .unwrap_or(false)
    }
    /// Number of own (non-inherited) fields.
    pub fn own_field_count(&self) -> usize {
        self.fields.iter().filter(|f| !f.is_inherited).count()
    }
    /// Number of inherited fields.
    pub fn inherited_field_count(&self) -> usize {
        self.fields.iter().filter(|f| f.is_inherited).count()
    }
}
/// Errors that can occur during structure elaboration.
#[derive(Clone, Debug)]
pub enum StructElabError {
    /// Parent structure not found in environment.
    ParentNotFound(String),
    /// Duplicate field name.
    DuplicateField(String),
    /// Field type does not match parent field type.
    FieldTypeMismatch(String),
    /// Circular inheritance detected.
    CircularInheritance(String),
    /// Invalid class definition.
    InvalidClass(String),
    /// Other error.
    Other(String),
}
/// Builds a structure update expression: `{ s with field := val }`.
#[derive(Clone, Debug)]
pub struct StructUpdateBuilder {
    /// The base expression being updated.
    pub base: Expr,
    /// The structure being updated.
    pub struct_name: Name,
    /// Fields to update: (field_name, new_value).
    pub updates: Vec<(Name, Expr)>,
}
impl StructUpdateBuilder {
    /// Create a new update builder.
    pub fn new(struct_name: Name, base: Expr) -> Self {
        Self {
            base,
            struct_name,
            updates: Vec::new(),
        }
    }
    /// Add a field update.
    pub fn update(mut self, field: Name, val: Expr) -> Self {
        self.updates.push((field, val));
        self
    }
    /// Build the update expression.
    ///
    /// The result is an application of the constructor with projected
    /// values for unchanged fields and the new values for updated fields.
    pub fn build(&self, info: &StructureInfo) -> Result<Expr, StructElabError> {
        validate_update_fields(
            info,
            &self
                .updates
                .iter()
                .map(|(n, _)| n.clone())
                .collect::<Vec<_>>(),
        )?;
        let args: Vec<Expr> = info
            .fields
            .iter()
            .map(|f| {
                if let Some((_, val)) = self.updates.iter().find(|(n, _)| n == &f.name) {
                    val.clone()
                } else {
                    Expr::Proj(f.name.clone(), f.idx as u32, Box::new(self.base.clone()))
                }
            })
            .collect();
        let ctor = Expr::Const(info.ctor_name.clone(), vec![]);
        let result = args
            .into_iter()
            .fold(ctor, |acc, arg| Expr::App(Box::new(acc), Box::new(arg)));
        Ok(result)
    }
    /// Return the number of updates.
    pub fn num_updates(&self) -> usize {
        self.updates.len()
    }
}
/// Main structure elaboration engine.
///
/// Handles elaborating `structure` and `class` declarations, generating
/// projections, constructors, and recursors, and managing the registry
/// of known structures.
pub struct StructureElaborator<'env> {
    /// Reference to the kernel environment.
    env: &'env Environment,
    /// Registry of elaborated structures.
    structures: HashMap<Name, StructureInfo>,
}
impl<'env> StructureElaborator<'env> {
    /// Create a new structure elaborator.
    pub fn new(env: &'env Environment) -> Self {
        Self {
            env,
            structures: HashMap::new(),
        }
    }
    /// Get a reference to the underlying environment.
    #[allow(dead_code)]
    pub fn env(&self) -> &Environment {
        self.env
    }
    /// Elaborate a `structure` declaration.
    pub fn elaborate_structure(&mut self, decl: &Decl) -> Result<StructureInfo, StructElabError> {
        match decl {
            Decl::Structure {
                name,
                univ_params,
                extends,
                fields,
            } => {
                let struct_name = Name::str(name);
                self.check_circular_inheritance(&struct_name, extends)?;
                let mut all_fields = Vec::new();
                let parent_names: Vec<Name> = extends.iter().map(Name::str).collect();
                for parent_name in &parent_names {
                    let parent_fields = self.process_parent(parent_name, &[])?;
                    for pf in parent_fields {
                        if !all_fields.iter().any(|f: &FieldInfo| f.name == pf.name) {
                            all_fields.push(pf);
                        }
                    }
                }
                let own_fields = elaborate_fields(&struct_name, fields)?;
                for field in &own_fields {
                    if all_fields.iter().any(|f| f.name == field.name) {
                        return Err(StructElabError::DuplicateField(format!(
                            "field '{}' already exists (inherited from parent)",
                            field.name
                        )));
                    }
                }
                all_fields.extend(own_fields);
                for (i, f) in all_fields.iter_mut().enumerate() {
                    f.idx = i;
                }
                let ctor_name = Name::str(format!("{}.mk", name));
                let mk_name = ctor_name.clone();
                let info = StructureInfo {
                    name: struct_name,
                    univ_params: univ_params.iter().map(Name::str).collect(),
                    params: Vec::new(),
                    fields: all_fields,
                    parent_structs: parent_names,
                    is_class: false,
                    ctor_name,
                    mk_name,
                };
                Ok(info)
            }
            _ => Err(StructElabError::Other(
                "expected Structure declaration".to_string(),
            )),
        }
    }
    /// Elaborate a `class` declaration.
    pub fn elaborate_class(&mut self, decl: &Decl) -> Result<StructureInfo, StructElabError> {
        match decl {
            Decl::ClassDecl {
                name,
                univ_params,
                extends,
                fields,
            } => {
                let struct_name = Name::str(name);
                self.check_circular_inheritance(&struct_name, extends)?;
                let mut all_fields = Vec::new();
                let parent_names: Vec<Name> = extends.iter().map(Name::str).collect();
                for parent_name in &parent_names {
                    if let Some(parent_info) = self.structures.get(parent_name) {
                        if !parent_info.is_class {
                            return Err(StructElabError::InvalidClass(format!(
                                "parent '{}' is not a class",
                                parent_name
                            )));
                        }
                    }
                    let parent_fields = self.process_parent(parent_name, &[])?;
                    for pf in parent_fields {
                        if !all_fields.iter().any(|f: &FieldInfo| f.name == pf.name) {
                            all_fields.push(pf);
                        }
                    }
                }
                let own_fields = elaborate_fields(&struct_name, fields)?;
                for field in &own_fields {
                    if all_fields.iter().any(|f| f.name == field.name) {
                        return Err(StructElabError::DuplicateField(format!(
                            "field '{}' already exists (inherited from parent)",
                            field.name
                        )));
                    }
                }
                all_fields.extend(own_fields);
                for (i, f) in all_fields.iter_mut().enumerate() {
                    f.idx = i;
                }
                let ctor_name = Name::str(format!("{}.mk", name));
                let mk_name = ctor_name.clone();
                let info = StructureInfo {
                    name: struct_name,
                    univ_params: univ_params.iter().map(Name::str).collect(),
                    params: Vec::new(),
                    fields: all_fields,
                    parent_structs: parent_names,
                    is_class: true,
                    ctor_name,
                    mk_name,
                };
                Ok(info)
            }
            _ => Err(StructElabError::Other(
                "expected ClassDecl declaration".to_string(),
            )),
        }
    }
    /// Process a parent structure and collect its fields.
    pub fn process_parent(
        &self,
        parent_name: &Name,
        _params: &[Expr],
    ) -> Result<Vec<FieldInfo>, StructElabError> {
        if let Some(parent_info) = self.structures.get(parent_name) {
            let mut inherited = Vec::new();
            for field in &parent_info.fields {
                inherited.push(FieldInfo {
                    name: field.name.clone(),
                    ty: field.ty.clone(),
                    binder_info: field.binder_info,
                    default_val: field.default_val.clone(),
                    proj_name: field.proj_name.clone(),
                    idx: field.idx,
                    is_inherited: true,
                    from_parent: Some(parent_name.clone()),
                });
            }
            Ok(inherited)
        } else {
            if self.env.get(parent_name).is_some() {
                Ok(Vec::new())
            } else {
                Err(StructElabError::ParentNotFound(format!(
                    "parent structure '{}' not found",
                    parent_name
                )))
            }
        }
    }
    /// Generate projection functions for all fields of a structure.
    pub fn generate_projections(&self, info: &StructureInfo) -> Vec<ProjectionDecl> {
        let struct_ty = self.mk_structure_type(info);
        let mut projections = Vec::new();
        for field in &info.fields {
            let proj_ty = Expr::Pi(
                BinderInfo::Default,
                Name::str("self"),
                Box::new(struct_ty.clone()),
                Box::new(field.ty.clone()),
            );
            let proj_val = Expr::Lam(
                BinderInfo::Default,
                Name::str("self"),
                Box::new(struct_ty.clone()),
                Box::new(Expr::Proj(
                    info.name.clone(),
                    field.idx as u32,
                    Box::new(Expr::BVar(0)),
                )),
            );
            projections.push(ProjectionDecl {
                name: field.proj_name.clone(),
                struct_name: info.name.clone(),
                field_idx: field.idx,
                ty: proj_ty,
                val: proj_val,
            });
        }
        projections
    }
    /// Generate the constructor declaration for a structure.
    pub fn generate_constructor(&self, info: &StructureInfo) -> ConstructorDecl {
        let ctor_ty = self.mk_constructor_type(info);
        ConstructorDecl {
            name: info.ctor_name.clone(),
            struct_name: info.name.clone(),
            ty: ctor_ty,
            num_fields: info.fields.len(),
        }
    }
    /// Generate the recursor declaration for a structure.
    pub fn generate_recursor(&self, info: &StructureInfo) -> RecursorDecl {
        let struct_ty = self.mk_structure_type(info);
        let motive_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(struct_ty.clone()),
            Box::new(Expr::Sort(Level::Param(Name::str("u")))),
        );
        let rec_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("motive"),
            Box::new(motive_ty),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("t"),
                Box::new(struct_ty),
                Box::new(Expr::Sort(Level::Param(Name::str("u")))),
            )),
        );
        RecursorDecl {
            name: Name::str(format!("{}.rec", info.name)),
            struct_name: info.name.clone(),
            ty: rec_ty,
            num_fields: info.fields.len(),
        }
    }
    /// Build the type of the structure itself.
    ///
    /// For a structure with no parameters, this is just `Sort u`.
    /// For parametrized structures, it is `(params...) -> Sort u`.
    pub fn mk_structure_type(&self, info: &StructureInfo) -> Expr {
        let base = if info.univ_params.is_empty() {
            Expr::Sort(Level::succ(Level::zero()))
        } else {
            Expr::Sort(Level::Param(info.univ_params[0].clone()))
        };
        let mut ty = base;
        for (name, param_ty, bi) in info.params.iter().rev() {
            ty = Expr::Pi(*bi, name.clone(), Box::new(param_ty.clone()), Box::new(ty));
        }
        ty
    }
    /// Build the type of the structure's constructor (`mk`).
    ///
    /// `mk : (field1 : ty1) -> (field2 : ty2) -> ... -> StructTy`
    pub fn mk_constructor_type(&self, info: &StructureInfo) -> Expr {
        let struct_ty = Expr::Const(info.name.clone(), Vec::new());
        let mut ty = struct_ty;
        for field in info.fields.iter().rev() {
            ty = Expr::Pi(
                field.binder_info,
                field.name.clone(),
                Box::new(field.ty.clone()),
                Box::new(ty),
            );
        }
        ty
    }
    /// Register a structure so it can be used as a parent or looked up later.
    pub fn register_structure(&mut self, info: StructureInfo) {
        self.structures.insert(info.name.clone(), info);
    }
    /// Look up a previously registered structure.
    pub fn lookup_structure(&self, name: &Name) -> Option<&StructureInfo> {
        self.structures.get(name)
    }
    /// Check if a name refers to a registered structure.
    pub fn is_structure(&self, name: &Name) -> bool {
        self.structures.contains_key(name)
    }
    /// Check if a name refers to a registered class.
    pub fn is_class(&self, name: &Name) -> bool {
        self.structures.get(name).is_some_and(|info| info.is_class)
    }
    /// Get all fields for a structure.
    pub fn get_fields(&self, name: &Name) -> Vec<&FieldInfo> {
        match self.structures.get(name) {
            Some(info) => info.fields.iter().collect(),
            None => Vec::new(),
        }
    }
    /// Get only the inherited fields for a structure.
    pub fn get_parent_fields(&self, name: &Name) -> Vec<&FieldInfo> {
        match self.structures.get(name) {
            Some(info) => info.fields.iter().filter(|f| f.is_inherited).collect(),
            None => Vec::new(),
        }
    }
    /// Check for circular inheritance by walking the parent chain.
    fn check_circular_inheritance(
        &self,
        struct_name: &Name,
        extends: &[String],
    ) -> Result<(), StructElabError> {
        let mut visited = vec![struct_name.clone()];
        let mut work_list: Vec<Name> = extends.iter().map(Name::str).collect();
        while let Some(parent) = work_list.pop() {
            if visited.contains(&parent) {
                return Err(StructElabError::CircularInheritance(format!(
                    "circular inheritance detected involving '{}'",
                    parent
                )));
            }
            visited.push(parent.clone());
            if let Some(parent_info) = self.structures.get(&parent) {
                for grandparent in &parent_info.parent_structs {
                    work_list.push(grandparent.clone());
                }
            }
        }
        Ok(())
    }
    /// Create the instance type for a class.
    ///
    /// For a class `C` with params `(a : A)`, the instance type is
    /// `(a : A) -> C a`.
    #[allow(dead_code)]
    pub fn mk_instance_type(&self, class_info: &StructureInfo) -> Expr {
        let mut result = Expr::Const(class_info.name.clone(), Vec::new());
        for (i, _) in class_info.params.iter().enumerate() {
            let param_count = class_info.params.len();
            result = Expr::App(
                Box::new(result),
                Box::new(Expr::BVar((param_count - 1 - i) as u32)),
            );
        }
        for (name, ty, bi) in class_info.params.iter().rev() {
            result = Expr::Pi(*bi, name.clone(), Box::new(ty.clone()), Box::new(result));
        }
        result
    }
    /// Elaborate a structure update expression `{ base with field1 := e1, ... }`.
    #[allow(dead_code)]
    pub fn elaborate_struct_update(
        &self,
        struct_name: &Name,
        base: &Expr,
        updates: &[(Name, Expr)],
    ) -> Result<Expr, StructElabError> {
        let info = self.structures.get(struct_name).ok_or_else(|| {
            StructElabError::Other(format!("structure '{}' not found", struct_name))
        })?;
        for (update_name, _) in updates {
            if !info.fields.iter().any(|f| &f.name == update_name) {
                return Err(StructElabError::Other(format!(
                    "unknown field '{}' in structure '{}'",
                    update_name, struct_name
                )));
            }
        }
        let mut ctor_app = Expr::Const(info.ctor_name.clone(), Vec::new());
        for field in &info.fields {
            let field_val =
                if let Some((_, new_val)) = updates.iter().find(|(n, _)| n == &field.name) {
                    new_val.clone()
                } else {
                    Expr::Proj(
                        struct_name.clone(),
                        field.idx as u32,
                        Box::new(base.clone()),
                    )
                };
            ctor_app = Expr::App(Box::new(ctor_app), Box::new(field_val));
        }
        Ok(ctor_app)
    }
    /// Resolve an anonymous constructor `⟨a, b, c⟩` given an expected type.
    ///
    /// When the expected type is a known structure, we desugar the anonymous
    /// constructor into an application of the structure's `mk` constructor.
    #[allow(dead_code)]
    pub fn resolve_anonymous_ctor(
        &self,
        expected_type: &Name,
        args: &[Expr],
    ) -> Result<Expr, StructElabError> {
        let info = self.structures.get(expected_type).ok_or_else(|| {
            StructElabError::Other(format!(
                "cannot resolve anonymous constructor: '{}' is not a structure",
                expected_type
            ))
        })?;
        if args.len() != info.fields.len() {
            return Err(StructElabError::Other(format!(
                "anonymous constructor for '{}' expects {} arguments, got {}",
                expected_type,
                info.fields.len(),
                args.len()
            )));
        }
        let mut result = Expr::Const(info.ctor_name.clone(), Vec::new());
        for arg in args {
            result = Expr::App(Box::new(result), Box::new(arg.clone()));
        }
        Ok(result)
    }
    /// Eta-expand a structure value into `mk (proj₁ e) (proj₂ e) ... (projₙ e)`.
    #[allow(dead_code)]
    pub fn eta_expand_struct(
        &self,
        struct_name: &Name,
        expr: &Expr,
    ) -> Result<Expr, StructElabError> {
        let info = self.structures.get(struct_name).ok_or_else(|| {
            StructElabError::Other(format!(
                "cannot eta-expand: '{}' is not a structure",
                struct_name
            ))
        })?;
        let mut result = Expr::Const(info.ctor_name.clone(), Vec::new());
        for field in &info.fields {
            let proj = Expr::Proj(
                struct_name.clone(),
                field.idx as u32,
                Box::new(expr.clone()),
            );
            result = Expr::App(Box::new(result), Box::new(proj));
        }
        Ok(result)
    }
    /// Eta-reduce a structure value: if `e = mk (proj₁ x) (proj₂ x) ... (projₙ x)` → `x`.
    ///
    /// Returns `Some(x)` if reduction succeeds, `None` otherwise.
    #[allow(dead_code)]
    pub fn eta_reduce_struct(&self, struct_name: &Name, expr: &Expr) -> Option<Expr> {
        let info = self.structures.get(struct_name)?;
        let mut args: Vec<&Expr> = Vec::new();
        let mut current = expr;
        while let Expr::App(fun, arg) = current {
            args.push(arg.as_ref());
            current = fun.as_ref();
        }
        match current {
            Expr::Const(name, _) if name == &info.ctor_name => {}
            _ => return None,
        }
        args.reverse();
        if args.len() != info.fields.len() {
            return None;
        }
        let mut common_base: Option<&Expr> = None;
        for (i, arg) in args.iter().enumerate() {
            match arg {
                Expr::Proj(sn, idx, base) if sn == struct_name && *idx == i as u32 => {
                    match common_base {
                        None => common_base = Some(base.as_ref()),
                        Some(prev) if prev == base.as_ref() => {}
                        _ => return None,
                    }
                }
                _ => return None,
            }
        }
        common_base.cloned()
    }
}
/// Statistics about a structure definition.
#[derive(Clone, Debug, Default)]
pub struct StructureStats {
    /// Number of fields.
    pub num_fields: usize,
    /// Number of inherited fields.
    pub num_inherited: usize,
    /// Number of parent structures.
    pub num_parents: usize,
    /// Whether the structure is a class.
    pub is_class: bool,
    /// Number of fields with default values.
    pub num_defaults: usize,
}
impl StructureStats {
    /// Compute stats from a `StructureInfo`.
    pub fn from_info(info: &StructureInfo) -> Self {
        let num_inherited = info.fields.iter().filter(|f| f.is_inherited).count();
        let num_defaults = info
            .fields
            .iter()
            .filter(|f| f.default_val.is_some())
            .count();
        Self {
            num_fields: info.fields.len(),
            num_inherited,
            num_parents: info.parent_structs.len(),
            is_class: info.is_class,
            num_defaults,
        }
    }
    /// Number of own (non-inherited) fields.
    pub fn num_own(&self) -> usize {
        self.num_fields.saturating_sub(self.num_inherited)
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "fields={} own={} inherited={} parents={} class={} defaults={}",
            self.num_fields,
            self.num_own(),
            self.num_inherited,
            self.num_parents,
            self.is_class,
            self.num_defaults,
        )
    }
}
/// A generated recursor declaration for the structure.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct RecursorDecl {
    /// Recursor name (e.g., `Foo.rec`).
    pub name: Name,
    /// Name of the structure.
    pub struct_name: Name,
    /// Recursor type.
    pub ty: Expr,
    /// Number of fields to eliminate over.
    pub num_fields: usize,
}
