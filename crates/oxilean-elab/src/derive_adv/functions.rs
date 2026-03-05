//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, FVarId, Level, Literal, Name};

use super::types::{
    AdvDerivableClass, AdvDeriveError, AdvDeriveRegistry, AdvDeriveResult, CtorInfo, Ordering,
    TypeInfoAdv,
};

/// Trait for class-specific derive handlers.
///
/// Each derivable class provides a handler that can inspect the type
/// information and produce the instance expression.
pub trait DeriveHandler: Send + Sync {
    /// Check whether this class can be derived for the given type.
    fn can_derive(&self, type_info: &TypeInfoAdv) -> bool;
    /// Derive the instance for the given type.
    fn derive(&self, type_info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError>;
    /// The name of the class this handler derives.
    fn class_name(&self) -> Name;
}
/// Generate a `BEq` instance for the given type.
///
/// The generated function takes two arguments of the type and returns
/// `Bool`. For each pair of same constructors, it compares fields pairwise.
/// For different constructors, it returns `false`.
pub(super) fn derive_beq(info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
    if info.is_recursive {
        return Err(AdvDeriveError::RecursiveType {
            class: "BEq".to_string(),
            type_name: format!("{}", info.name),
        });
    }
    if info.constructors.is_empty() {
        return Err(AdvDeriveError::EmptyType {
            type_name: format!("{}", info.name),
        });
    }
    let ty_expr = info.type_expr();
    let mut match_arms: Vec<Expr> = Vec::new();
    for ctor in &info.constructors {
        if ctor.is_nullary() {
            match_arms.push(mk_bool_lit(true));
        } else {
            let comparisons: Vec<Expr> = ctor
                .fields
                .iter()
                .enumerate()
                .map(|(i, (_, field_ty))| mk_beq_call(field_ty, &mk_lhs_var(i), &mk_rhs_var(i)))
                .collect();
            match_arms.push(mk_and_chain(&comparisons));
        }
    }
    if info.constructors.len() > 1 {
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
    let instance_name = Name::str(format!("instBEq{}", info.name));
    let instance_type = mk_class_app("BEq", &ty_expr);
    Ok(AdvDeriveResult::new(
        instance_name,
        instance_type,
        beq_lam,
        vec![],
    ))
}
/// Generate a `DecidableEq` instance.
pub(super) fn derive_decidable_eq(info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
    if info.is_recursive {
        return Err(AdvDeriveError::RecursiveType {
            class: "DecidableEq".to_string(),
            type_name: format!("{}", info.name),
        });
    }
    if info.constructors.is_empty() {
        return Err(AdvDeriveError::EmptyType {
            type_name: format!("{}", info.name),
        });
    }
    let ty_expr = info.type_expr();
    let mut match_arms: Vec<Expr> = Vec::new();
    for ctor in &info.constructors {
        if ctor.is_nullary() {
            match_arms.push(mk_app2(
                Expr::Const(Name::str("Decidable.isTrue"), vec![]),
                Expr::Const(Name::str("rfl"), vec![]),
            ));
        } else {
            let field_checks: Vec<Expr> = ctor
                .fields
                .iter()
                .enumerate()
                .map(|(i, (_, field_ty))| {
                    mk_app2(
                        mk_app2(Expr::Const(Name::str("decEq"), vec![]), field_ty.clone()),
                        mk_app2(mk_lhs_var(i), mk_rhs_var(i)),
                    )
                })
                .collect();
            match_arms.push(mk_decidable_and_chain(&field_checks));
        }
    }
    if info.constructors.len() > 1 {
        match_arms.push(mk_app2(
            Expr::Const(Name::str("Decidable.isFalse"), vec![]),
            Expr::Const(Name::str("noConfusion"), vec![]),
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
    let instance_name = Name::str(format!("instDecidableEq{}", info.name));
    let instance_type = mk_class_app("DecidableEq", &ty_expr);
    Ok(AdvDeriveResult::new(
        instance_name,
        instance_type,
        dec_eq_lam,
        vec![],
    ))
}
/// Generate a `Hashable` instance.
///
/// For each constructor, hash the tag number and all field values, then
/// combine them with `mixHash`.
pub(super) fn derive_hashable(info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
    if info.is_recursive {
        return Err(AdvDeriveError::RecursiveType {
            class: "Hashable".to_string(),
            type_name: format!("{}", info.name),
        });
    }
    if info.constructors.is_empty() {
        return Err(AdvDeriveError::EmptyType {
            type_name: format!("{}", info.name),
        });
    }
    let ty_expr = info.type_expr();
    let mut match_arms: Vec<Expr> = Vec::new();
    for (tag, ctor) in info.constructors.iter().enumerate() {
        let tag_hash = mk_app2(
            Expr::Const(Name::str("hash"), vec![]),
            Expr::Lit(Literal::Nat(tag as u64)),
        );
        let field_hashes: Vec<Expr> = ctor
            .fields
            .iter()
            .enumerate()
            .map(|(i, _)| mk_app2(Expr::Const(Name::str("hash"), vec![]), mk_lhs_var(i)))
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
    let instance_name = Name::str(format!("instHashable{}", info.name));
    let instance_type = mk_class_app("Hashable", &ty_expr);
    Ok(AdvDeriveResult::new(
        instance_name,
        instance_type,
        hash_lam,
        vec![],
    ))
}
/// Generate an `Ord` instance with lexicographic comparison.
///
/// For a type with constructors `C0`, `C1`, ... the ordering is:
/// - `C0 < C1 < C2 < ...` (by constructor index)
/// - Within the same constructor, fields are compared lexicographically.
pub(super) fn derive_ord(info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
    if info.is_recursive {
        return Err(AdvDeriveError::RecursiveType {
            class: "Ord".to_string(),
            type_name: format!("{}", info.name),
        });
    }
    if info.constructors.is_empty() {
        return Err(AdvDeriveError::EmptyType {
            type_name: format!("{}", info.name),
        });
    }
    let ty_expr = info.type_expr();
    let mut match_arms: Vec<Expr> = Vec::new();
    let n = info.constructors.len();
    for (i, ctor_a) in info.constructors.iter().enumerate() {
        let inner_arms: Vec<Expr> = info
            .constructors
            .iter()
            .enumerate()
            .map(|(j, _)| {
                if i < j {
                    mk_ordering_lit(Ordering::Less)
                } else if i > j {
                    mk_ordering_lit(Ordering::Greater)
                } else if ctor_a.is_nullary() {
                    mk_ordering_lit(Ordering::Equal)
                } else {
                    mk_lex_compare(&ctor_a.fields)
                }
            })
            .collect();
        match_arms.push(if n == 1 {
            inner_arms
                .into_iter()
                .next()
                .unwrap_or(mk_ordering_lit(Ordering::Equal))
        } else {
            build_match_body(&inner_arms)
        });
    }
    let body = build_match_body(&match_arms);
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
    let compare_aux_name = Name::str(format!("{}.compare", info.name));
    let ordering_ty = Expr::Const(Name::str("Ordering"), vec![]);
    let compare_type = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Box::new(ty_expr.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Box::new(ty_expr.clone()),
            Box::new(ordering_ty),
        )),
    );
    let instance_name = Name::str(format!("instOrd{}", info.name));
    let instance_type = mk_class_app("Ord", &ty_expr);
    Ok(AdvDeriveResult::new(
        instance_name,
        instance_type,
        compare_lam.clone(),
        vec![(compare_aux_name, compare_type, compare_lam)],
    ))
}
/// Generate a `Repr` instance.
///
/// For each constructor, produce a string literal of the constructor name
/// followed by the repr of each field, separated by spaces.
pub(super) fn derive_repr(info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
    if info.constructors.is_empty() {
        return Err(AdvDeriveError::EmptyType {
            type_name: format!("{}", info.name),
        });
    }
    let ty_expr = info.type_expr();
    let mut match_arms: Vec<Expr> = Vec::new();
    for ctor in &info.constructors {
        let field_reprs: Vec<Expr> = ctor
            .fields
            .iter()
            .enumerate()
            .map(|(i, _)| mk_app2(Expr::Const(Name::str("repr"), vec![]), mk_lhs_var(i)))
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
    let instance_name = Name::str(format!("instRepr{}", info.name));
    let instance_type = mk_class_app("Repr", &ty_expr);
    Ok(AdvDeriveResult::new(
        instance_name,
        instance_type,
        repr_lam,
        vec![],
    ))
}
/// Generate an `Inhabited` instance by picking the first constructor
/// and filling all fields with `default`.
pub(super) fn derive_inhabited(info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
    if info.constructors.is_empty() {
        return Err(AdvDeriveError::EmptyType {
            type_name: format!("{}", info.name),
        });
    }
    let ty_expr = info.type_expr();
    let ctor = &info.constructors[0];
    let mut body: Expr = Expr::Const(ctor.name.clone(), vec![]);
    for (_, field_ty) in &ctor.fields {
        let default_val = mk_app2(Expr::Const(Name::str("default"), vec![]), field_ty.clone());
        body = mk_app2(body, default_val);
    }
    let inhabited_val = mk_app2(Expr::Const(Name::str("Inhabited.mk"), vec![]), body);
    let instance_name = Name::str(format!("instInhabited{}", info.name));
    let instance_type = mk_class_app("Inhabited", &ty_expr);
    Ok(AdvDeriveResult::new(
        instance_name,
        instance_type,
        inhabited_val,
        vec![],
    ))
}
/// Generate a `Nonempty` instance.
pub(super) fn derive_nonempty(info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
    if info.constructors.is_empty() {
        return Err(AdvDeriveError::EmptyType {
            type_name: format!("{}", info.name),
        });
    }
    let ty_expr = info.type_expr();
    let ctor = &info.constructors[0];
    let mut witness: Expr = Expr::Const(ctor.name.clone(), vec![]);
    for (_, field_ty) in &ctor.fields {
        let default_val = mk_app2(Expr::Const(Name::str("default"), vec![]), field_ty.clone());
        witness = mk_app2(witness, default_val);
    }
    let body = mk_app2(Expr::Const(Name::str("Nonempty.intro"), vec![]), witness);
    let instance_name = Name::str(format!("instNonempty{}", info.name));
    let instance_type = mk_class_app("Nonempty", &ty_expr);
    Ok(AdvDeriveResult::new(
        instance_name,
        instance_type,
        body,
        vec![],
    ))
}
/// Generate a `ToString` instance by delegating to `reprStr`.
pub(super) fn derive_to_string(info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
    let ty_expr = info.type_expr();
    let body = Expr::Lam(
        BinderInfo::Default,
        Name::str("a"),
        Box::new(ty_expr.clone()),
        Box::new(mk_app2(
            Expr::Const(Name::str("reprStr"), vec![]),
            Expr::BVar(0),
        )),
    );
    let instance_name = Name::str(format!("instToString{}", info.name));
    let instance_type = mk_class_app("ToString", &ty_expr);
    Ok(AdvDeriveResult::new(
        instance_name,
        instance_type,
        body,
        vec![],
    ))
}
/// Create a `Bool.true` or `Bool.false` literal.
pub fn mk_bool_lit(val: bool) -> Expr {
    if val {
        Expr::Const(Name::str("Bool.true"), vec![])
    } else {
        Expr::Const(Name::str("Bool.false"), vec![])
    }
}
/// Create an `Ordering.lt`, `Ordering.eq`, or `Ordering.gt` constant.
pub(super) fn mk_ordering_lit(ord: Ordering) -> Expr {
    match ord {
        Ordering::Less => Expr::Const(Name::str("Ordering.lt"), vec![]),
        Ordering::Equal => Expr::Const(Name::str("Ordering.eq"), vec![]),
        Ordering::Greater => Expr::Const(Name::str("Ordering.gt"), vec![]),
    }
}
/// Create a free variable for the i-th LHS field.
pub(super) fn mk_lhs_var(i: usize) -> Expr {
    Expr::FVar(FVarId::new(2000 + i as u64))
}
/// Create a free variable for the i-th RHS field.
pub(super) fn mk_rhs_var(i: usize) -> Expr {
    Expr::FVar(FVarId::new(3000 + i as u64))
}
/// Binary application: `f a`.
pub(super) fn mk_app2(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Build `BEq.beq ty lhs rhs`.
pub(super) fn mk_beq_call(field_ty: &Expr, lhs: &Expr, rhs: &Expr) -> Expr {
    mk_app2(
        mk_app2(
            mk_app2(Expr::Const(Name::str("BEq.beq"), vec![]), field_ty.clone()),
            lhs.clone(),
        ),
        rhs.clone(),
    )
}
/// Build `Ord.compare ty lhs rhs`.
pub(super) fn mk_compare_call(field_ty: &Expr, lhs: &Expr, rhs: &Expr) -> Expr {
    mk_app2(
        mk_app2(
            mk_app2(
                Expr::Const(Name::str("Ord.compare"), vec![]),
                field_ty.clone(),
            ),
            lhs.clone(),
        ),
        rhs.clone(),
    )
}
/// Build `Class ty` (single-arg class application).
pub(super) fn mk_class_app(class: &str, ty: &Expr) -> Expr {
    mk_app2(Expr::Const(Name::str(class), vec![]), ty.clone())
}
/// Chain with `and` / `&&`: `a && b && c ...`.
/// Empty list yields `true`.
pub(super) fn mk_and_chain(exprs: &[Expr]) -> Expr {
    if exprs.is_empty() {
        return mk_bool_lit(true);
    }
    let mut result = exprs[0].clone();
    for e in &exprs[1..] {
        result = mk_app2(
            mk_app2(Expr::Const(Name::str("and"), vec![]), result),
            e.clone(),
        );
    }
    result
}
/// Chain decidable AND checks: `Decidable.and d1 d2 ...`.
/// Empty list yields `Decidable.isTrue rfl`.
pub fn mk_decidable_and_chain(exprs: &[Expr]) -> Expr {
    if exprs.is_empty() {
        return mk_app2(
            Expr::Const(Name::str("Decidable.isTrue"), vec![]),
            Expr::Const(Name::str("rfl"), vec![]),
        );
    }
    if exprs.len() == 1 {
        return exprs[0].clone();
    }
    let mut result = exprs[0].clone();
    for e in &exprs[1..] {
        result = mk_app2(
            mk_app2(Expr::Const(Name::str("Decidable.and"), vec![]), result),
            e.clone(),
        );
    }
    result
}
/// Combine hash expressions: `mixHash h1 (mixHash h2 ...)`.
/// Empty list yields `hash 0`.
pub(super) fn mk_hash_combine(exprs: &[Expr]) -> Expr {
    if exprs.is_empty() {
        return mk_app2(
            Expr::Const(Name::str("hash"), vec![]),
            Expr::Lit(Literal::Nat(0)),
        );
    }
    if exprs.len() == 1 {
        return exprs[0].clone();
    }
    let mut result = exprs
        .last()
        .expect("exprs is non-empty (checked above)")
        .clone();
    for e in exprs[..exprs.len() - 1].iter().rev() {
        result = mk_app2(
            mk_app2(Expr::Const(Name::str("mixHash"), vec![]), e.clone()),
            result,
        );
    }
    result
}
/// Build a repr string: `"CtorName" ++ " " ++ repr_field_0 ++ ...`.
pub(super) fn mk_repr_string(ctor_name: &Name, field_reprs: &[Expr]) -> Expr {
    let mut result = Expr::Lit(Literal::Str(format!("{}", ctor_name)));
    for field_repr in field_reprs {
        result = mk_app2(
            mk_app2(Expr::Const(Name::str("String.append"), vec![]), result),
            Expr::Lit(Literal::Str(" ".to_string())),
        );
        result = mk_app2(
            mk_app2(Expr::Const(Name::str("String.append"), vec![]), result),
            field_repr.clone(),
        );
    }
    result
}
/// Build a lexicographic comparison over a list of fields.
///
/// For fields `f0, f1, ..., fn`:
/// ```text
/// match compare f0_lhs f0_rhs with
/// | .eq => match compare f1_lhs f1_rhs with
///          | .eq => ... | .lt => .lt | .gt => .gt
/// | .lt => .lt
/// | .gt => .gt
/// ```
pub(super) fn mk_lex_compare(fields: &[(Name, Expr)]) -> Expr {
    if fields.is_empty() {
        return mk_ordering_lit(Ordering::Equal);
    }
    let mut result = mk_ordering_lit(Ordering::Equal);
    for (i, (_, field_ty)) in fields.iter().enumerate().rev() {
        let cmp = mk_compare_call(field_ty, &mk_lhs_var(i), &mk_rhs_var(i));
        let lt_arm = mk_ordering_lit(Ordering::Less);
        let eq_arm = result;
        let gt_arm = mk_ordering_lit(Ordering::Greater);
        let cases = Expr::Const(Name::str("Ordering.casesOn"), vec![]);
        result = mk_app2(
            mk_app2(mk_app2(mk_app2(cases, cmp), lt_arm), eq_arm),
            gt_arm,
        );
    }
    result
}
/// Build a tag-based ordering comparison placeholder.
///
/// In a complete implementation, this would compare constructor indices.
/// Here we use a simplified expression that compares by tag.
#[allow(dead_code)]
fn mk_ordering_tag_compare() -> Expr {
    mk_app2(
        mk_app2(
            Expr::Const(Name::str("compareTag"), vec![]),
            Expr::FVar(FVarId::new(9000)),
        ),
        Expr::FVar(FVarId::new(9001)),
    )
}
/// Build a simplified match body from a list of arms.
pub fn build_match_body(arms: &[Expr]) -> Expr {
    if arms.is_empty() {
        return Expr::Const(Name::str("absurd"), vec![]);
    }
    if arms.len() == 1 {
        return arms[0].clone();
    }
    let mut result: Expr = Expr::Const(Name::str("casesOn"), vec![]);
    for arm in arms {
        result = mk_app2(result, arm.clone());
    }
    result
}
/// Create a `TypeInfoAdv` from simple parameters (for testing/convenience).
pub fn mk_type_info(name: Name, constructors: Vec<CtorInfo>, is_recursive: bool) -> TypeInfoAdv {
    TypeInfoAdv {
        name,
        univ_params: vec![],
        params: vec![],
        constructors,
        is_inductive: true,
        is_structure: false,
        is_recursive,
        num_indices: 0,
    }
}
/// Create a structure-like `TypeInfoAdv` (single constructor).
pub fn mk_structure_info(name: Name, fields: Vec<(Name, Expr)>) -> TypeInfoAdv {
    let ctor = CtorInfo::new(Name::str(format!("{}.mk", name)), fields);
    TypeInfoAdv {
        name,
        univ_params: vec![],
        params: vec![],
        constructors: vec![ctor],
        is_inductive: true,
        is_structure: true,
        is_recursive: false,
        num_indices: 0,
    }
}
/// Check if a built-in class can be derived for a type.
pub fn can_derive_builtin(class: &AdvDerivableClass, info: &TypeInfoAdv) -> bool {
    match class {
        AdvDerivableClass::BEq | AdvDerivableClass::DecidableEq | AdvDerivableClass::Hashable => {
            !info.is_recursive && !info.constructors.is_empty()
        }
        AdvDerivableClass::Ord => !info.is_recursive && !info.constructors.is_empty(),
        AdvDerivableClass::Repr | AdvDerivableClass::ToString => !info.constructors.is_empty(),
        AdvDerivableClass::Inhabited | AdvDerivableClass::Nonempty => !info.constructors.is_empty(),
        AdvDerivableClass::Custom(_) => false,
    }
}
/// Derive a built-in class directly (without the registry).
pub fn derive_builtin(
    class: &AdvDerivableClass,
    info: &TypeInfoAdv,
) -> Result<AdvDeriveResult, AdvDeriveError> {
    match class {
        AdvDerivableClass::BEq => derive_beq(info),
        AdvDerivableClass::DecidableEq => derive_decidable_eq(info),
        AdvDerivableClass::Hashable => derive_hashable(info),
        AdvDerivableClass::Ord => derive_ord(info),
        AdvDerivableClass::Repr => derive_repr(info),
        AdvDerivableClass::Inhabited => derive_inhabited(info),
        AdvDerivableClass::Nonempty => derive_nonempty(info),
        AdvDerivableClass::ToString => derive_to_string(info),
        AdvDerivableClass::Custom(name) => Err(AdvDeriveError::NoHandler {
            class: format!("{}", name),
        }),
    }
}
/// Derive multiple classes and collect all results (both successes and errors).
pub fn derive_multiple(
    classes: &[AdvDerivableClass],
    info: &TypeInfoAdv,
) -> Vec<Result<AdvDeriveResult, AdvDeriveError>> {
    classes
        .iter()
        .map(|cls| derive_builtin(cls, info))
        .collect()
}
/// Derive multiple classes, returning only the successful results.
pub fn derive_multiple_ok(
    classes: &[AdvDerivableClass],
    info: &TypeInfoAdv,
) -> Vec<AdvDeriveResult> {
    classes
        .iter()
        .filter_map(|cls| derive_builtin(cls, info).ok())
        .collect()
}
/// Check if all fields of all constructors have a given class instance available.
///
/// Returns a list of `(ctor_name, field_name, field_type)` triples for fields
/// whose type does NOT have a known instance of `class`.
///
/// The check is conservative: primitive types (`Nat`, `Int`, `Bool`, `String`,
/// `Char`, `Float`, `Unit`) are assumed to have instances for all built-in
/// derivable classes. Recursive occurrences of the type being derived are
/// assumed to have the instance (as they will, once derived). Any other
/// `Const` type is assumed present. Type-former applications (`App`) are
/// checked by inspecting the head constant. Variables and other expression
/// forms are conservatively accepted. Only `Custom` classes for non-primitive
/// field types are flagged as potentially missing.
pub fn check_field_instances(
    class: &AdvDerivableClass,
    info: &TypeInfoAdv,
) -> Vec<(Name, Name, Expr)> {
    const PRIMITIVE_TYPES: &[&str] = &[
        "Nat", "Int", "Bool", "String", "Char", "Float", "UInt8", "UInt16", "UInt32", "UInt64",
        "Int8", "Int16", "Int32", "Int64", "Unit", "Empty",
    ];
    let is_custom = matches!(class, AdvDerivableClass::Custom(_));
    let mut missing: Vec<(Name, Name, Expr)> = Vec::new();
    for ctor in &info.constructors {
        for (field_name, field_ty) in &ctor.fields {
            if !field_type_has_instance(field_ty, &info.name, PRIMITIVE_TYPES, is_custom) {
                missing.push((ctor.name.clone(), field_name.clone(), field_ty.clone()));
            }
        }
    }
    missing
}
/// Return `true` if `field_ty` is assumed to have an instance of the target
/// class.
///
/// - All `BVar` / `FVar` / `Lit` / `Sort` / `Pi` / `Lam` forms are accepted
///   conservatively (the elaborator would catch real errors).
/// - `Const(name, _)` is checked against the primitive list and the parent
///   type name (recursive occurrence).
/// - `App(f, _)` recurses into the head.
/// - For `Custom` classes, non-primitive `Const` types are flagged as missing.
fn field_type_has_instance(
    ty: &Expr,
    parent_name: &Name,
    primitives: &[&str],
    is_custom_class: bool,
) -> bool {
    match ty {
        Expr::Const(name, _) => {
            if name == parent_name {
                return true;
            }
            let name_str = name.to_string();
            let short = name_str.split('.').next_back().unwrap_or(&name_str);
            if primitives.contains(&short) || primitives.contains(&name_str.as_str()) {
                return true;
            }
            !is_custom_class
        }
        Expr::App(f, _) => field_type_has_instance(f, parent_name, primitives, is_custom_class),
        _ => true,
    }
}
/// Validate that a type can have a class derived, checking field instances.
pub fn validate_derivation(
    class: &AdvDerivableClass,
    info: &TypeInfoAdv,
) -> Result<(), AdvDeriveError> {
    if !can_derive_builtin(class, info) {
        return Err(AdvDeriveError::CannotDerive {
            class: format!("{}", class),
            type_name: format!("{}", info.name),
            reason: "basic derivability check failed".to_string(),
        });
    }
    let missing = check_field_instances(class, info);
    if let Some((ctor, field, field_ty)) = missing.first() {
        return Err(AdvDeriveError::MissingFieldInstance {
            class: format!("{}", class),
            field: format!("{}.{}", ctor, field),
            field_type: format!("{:?}", field_ty),
        });
    }
    Ok(())
}
/// Build an instance type for a parametric type.
///
/// For a type `T α β` with params `(α : Type) (β : Type)`, the instance
/// type for class `C` is:
/// ```text
/// ∀ {α : Type} [BEq α] {β : Type} [BEq β], BEq (T α β)
/// ```
pub fn build_parametric_instance_type(class: &AdvDerivableClass, info: &TypeInfoAdv) -> Expr {
    let base_type = info.applied_type_expr();
    let mut result = mk_class_app(&format!("{}", class), &base_type);
    for (pname, pty, _binder_info) in info.params.iter().rev() {
        let constraint_type = mk_class_app(&format!("{}", class), &Expr::BVar(0));
        result = Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str(format!("inst_{}", pname)),
            Box::new(constraint_type),
            Box::new(result),
        );
        result = Expr::Pi(
            BinderInfo::Implicit,
            pname.clone(),
            Box::new(pty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Build an instance expression for a parametric type.
///
/// Wraps the core derivation body in lambda binders for the type parameters
/// and their class constraints.
pub fn build_parametric_instance_body(
    class: &AdvDerivableClass,
    info: &TypeInfoAdv,
    core_body: Expr,
) -> Expr {
    let mut result = core_body;
    for (pname, pty, _binder_info) in info.params.iter().rev() {
        let constraint_type = mk_class_app(&format!("{}", class), &Expr::BVar(0));
        result = Expr::Lam(
            BinderInfo::InstImplicit,
            Name::str(format!("inst_{}", pname)),
            Box::new(constraint_type),
            Box::new(result),
        );
        result = Expr::Lam(
            BinderInfo::Implicit,
            pname.clone(),
            Box::new(pty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Generate a canonical instance name for a derived class.
///
/// Follows the Lean 4 convention: `inst<ClassName><TypeName>`.
pub fn instance_name(class: &AdvDerivableClass, type_name: &Name) -> Name {
    Name::str(format!("inst{}{}", class, type_name))
}
/// Generate an auxiliary declaration name.
///
/// Follows the convention: `<TypeName>.<suffix>`.
pub fn aux_decl_name(type_name: &Name, suffix: &str) -> Name {
    Name::str(format!("{}.{}", type_name, suffix))
}
/// High-level derivation function that handles the full pipeline:
/// 1. Validate derivability
/// 2. Check field instances
/// 3. Derive the instance
/// 4. Build parametric wrapper if needed
pub fn full_derive(
    class: &AdvDerivableClass,
    info: &TypeInfoAdv,
) -> Result<AdvDeriveResult, AdvDeriveError> {
    validate_derivation(class, info)?;
    let mut result = derive_builtin(class, info)?;
    if !info.params.is_empty() {
        result.instance_type = build_parametric_instance_type(class, info);
        result.instance_expr = build_parametric_instance_body(class, info, result.instance_expr);
    }
    Ok(result)
}
/// Derive all specified classes for a type, collecting results.
pub fn full_derive_many(
    classes: &[AdvDerivableClass],
    info: &TypeInfoAdv,
) -> Vec<Result<AdvDeriveResult, AdvDeriveError>> {
    classes.iter().map(|cls| full_derive(cls, info)).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::derive_adv::*;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn string_ty() -> Expr {
        Expr::Const(Name::str("String"), vec![])
    }
    fn bool_ty() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    /// Simple enum: `inductive Color | red | green | blue`
    fn color_info() -> TypeInfoAdv {
        mk_type_info(
            Name::str("Color"),
            vec![
                CtorInfo::new(Name::str("Color.red"), vec![]),
                CtorInfo::new(Name::str("Color.green"), vec![]),
                CtorInfo::new(Name::str("Color.blue"), vec![]),
            ],
            false,
        )
    }
    /// Struct: `structure Point | mk (x : Nat) (y : Nat)`
    fn point_info() -> TypeInfoAdv {
        mk_structure_info(
            Name::str("Point"),
            vec![(Name::str("x"), nat_ty()), (Name::str("y"), nat_ty())],
        )
    }
    /// Mixed inductive: `inductive Shape | circle (r : Nat) | rect (w h : Nat)`
    fn shape_info() -> TypeInfoAdv {
        mk_type_info(
            Name::str("Shape"),
            vec![
                CtorInfo::new(Name::str("Shape.circle"), vec![(Name::str("r"), nat_ty())]),
                CtorInfo::new(
                    Name::str("Shape.rect"),
                    vec![(Name::str("w"), nat_ty()), (Name::str("h"), nat_ty())],
                ),
            ],
            false,
        )
    }
    /// Recursive: `inductive Tree | leaf | node (l r : Tree)`
    fn tree_info() -> TypeInfoAdv {
        let tree_ty = Expr::Const(Name::str("Tree"), vec![]);
        mk_type_info(
            Name::str("Tree"),
            vec![
                CtorInfo::new(Name::str("Tree.leaf"), vec![]),
                CtorInfo::new(
                    Name::str("Tree.node"),
                    vec![
                        (Name::str("left"), tree_ty.clone()),
                        (Name::str("right"), tree_ty),
                    ],
                ),
            ],
            true,
        )
    }
    /// Empty type (no constructors).
    fn empty_info() -> TypeInfoAdv {
        mk_type_info(Name::str("Empty"), vec![], false)
    }
    /// Person struct with multiple field types.
    fn person_info() -> TypeInfoAdv {
        mk_structure_info(
            Name::str("Person"),
            vec![
                (Name::str("name"), string_ty()),
                (Name::str("age"), nat_ty()),
                (Name::str("active"), bool_ty()),
            ],
        )
    }
    #[test]
    fn test_class_names() {
        assert_eq!(AdvDerivableClass::BEq.class_name(), Name::str("BEq"));
        assert_eq!(AdvDerivableClass::Ord.class_name(), Name::str("Ord"));
        assert_eq!(
            AdvDerivableClass::Custom(Name::str("Foo")).class_name(),
            Name::str("Foo")
        );
    }
    #[test]
    fn test_class_from_name() {
        assert_eq!(AdvDerivableClass::from_name("BEq"), AdvDerivableClass::BEq);
        assert_eq!(AdvDerivableClass::from_name("Ord"), AdvDerivableClass::Ord);
        assert_eq!(
            AdvDerivableClass::from_name("Unknown"),
            AdvDerivableClass::Custom(Name::str("Unknown"))
        );
    }
    #[test]
    fn test_class_is_builtin() {
        assert!(AdvDerivableClass::BEq.is_builtin());
        assert!(!AdvDerivableClass::Custom(Name::str("X")).is_builtin());
    }
    #[test]
    fn test_all_builtins() {
        let builtins = AdvDerivableClass::all_builtins();
        assert_eq!(builtins.len(), 8);
    }
    #[test]
    fn test_class_display() {
        assert_eq!(format!("{}", AdvDerivableClass::BEq), "BEq");
        assert_eq!(
            format!("{}", AdvDerivableClass::Custom(Name::str("Foo"))),
            "Foo"
        );
    }
    #[test]
    fn test_ctor_info_nullary() {
        let ci = CtorInfo::new(Name::str("Unit.unit"), vec![]);
        assert!(ci.is_nullary());
        assert_eq!(ci.num_fields(), 0);
        assert!(ci.field_names().is_empty());
    }
    #[test]
    fn test_ctor_info_with_fields() {
        let ci = CtorInfo::new(
            Name::str("Pair.mk"),
            vec![(Name::str("fst"), nat_ty()), (Name::str("snd"), nat_ty())],
        );
        assert!(!ci.is_nullary());
        assert_eq!(ci.num_fields(), 2);
        assert_eq!(ci.field_names().len(), 2);
        assert!(ci.field_type(&Name::str("fst")).is_some());
        assert!(ci.field_type(&Name::str("nope")).is_none());
    }
    #[test]
    fn test_type_info_enum() {
        let ti = color_info();
        assert!(ti.is_enum());
        assert_eq!(ti.num_constructors(), 3);
        assert_eq!(ti.total_fields(), 0);
        assert!(!ti.is_single_ctor());
    }
    #[test]
    fn test_type_info_struct() {
        let ti = point_info();
        assert!(!ti.is_enum());
        assert_eq!(ti.num_constructors(), 1);
        assert_eq!(ti.total_fields(), 2);
        assert!(ti.is_single_ctor());
        assert!(ti.is_structure);
    }
    #[test]
    fn test_type_info_recursive() {
        let ti = tree_info();
        assert!(ti.is_recursive);
    }
    #[test]
    fn test_type_info_first_ctor() {
        let ti = color_info();
        assert!(ti.first_ctor().is_some());
        assert_eq!(
            ti.first_ctor().expect("test operation should succeed").name,
            Name::str("Color.red")
        );
        let empty = empty_info();
        assert!(empty.first_ctor().is_none());
    }
    #[test]
    fn test_type_info_type_expr() {
        let ti = color_info();
        assert!(matches!(ti.type_expr(), Expr::Const(n, _) if n == Name::str("Color")));
    }
    #[test]
    fn test_derive_result() {
        let dr = AdvDeriveResult::new(
            Name::str("inst"),
            Expr::Sort(Level::zero()),
            Expr::Sort(Level::zero()),
            vec![],
        );
        assert!(!dr.has_aux_decls());
        assert_eq!(dr.num_aux_decls(), 0);
    }
    #[test]
    fn test_derive_result_with_aux() {
        let dr = AdvDeriveResult::new(
            Name::str("inst"),
            Expr::Sort(Level::zero()),
            Expr::Sort(Level::zero()),
            vec![(
                Name::str("aux"),
                Expr::Sort(Level::zero()),
                Expr::Sort(Level::zero()),
            )],
        );
        assert!(dr.has_aux_decls());
        assert_eq!(dr.num_aux_decls(), 1);
    }
    #[test]
    fn test_error_display() {
        let e = AdvDeriveError::CannotDerive {
            class: "BEq".into(),
            type_name: "Foo".into(),
            reason: "reasons".into(),
        };
        assert!(format!("{}", e).contains("BEq"));
        assert!(format!("{}", e).contains("Foo"));
        let e = AdvDeriveError::RecursiveType {
            class: "BEq".into(),
            type_name: "Tree".into(),
        };
        assert!(format!("{}", e).contains("recursive"));
        let e = AdvDeriveError::EmptyType {
            type_name: "Empty".into(),
        };
        assert!(format!("{}", e).contains("no constructors"));
        let e = AdvDeriveError::NoHandler {
            class: "Unknown".into(),
        };
        assert!(format!("{}", e).contains("no handler"));
        let e = AdvDeriveError::MissingFieldInstance {
            class: "BEq".into(),
            field: "x".into(),
            field_type: "Foo".into(),
        };
        assert!(format!("{}", e).contains("missing"));
        let e = AdvDeriveError::Internal("oops".into());
        assert!(format!("{}", e).contains("oops"));
    }
    #[test]
    fn test_derive_beq_enum() {
        let result = derive_beq(&color_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("instBEqColor"));
    }
    #[test]
    fn test_derive_beq_struct() {
        let result = derive_beq(&point_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert!(dr.instance_expr.is_lambda());
    }
    #[test]
    fn test_derive_beq_mixed() {
        let result = derive_beq(&shape_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_beq_recursive_fails() {
        let result = derive_beq(&tree_info());
        assert!(matches!(result, Err(AdvDeriveError::RecursiveType { .. })));
    }
    #[test]
    fn test_derive_beq_empty_fails() {
        let result = derive_beq(&empty_info());
        assert!(matches!(result, Err(AdvDeriveError::EmptyType { .. })));
    }
    #[test]
    fn test_derive_ord_enum() {
        let result = derive_ord(&color_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("instOrdColor"));
        assert!(dr.has_aux_decls());
    }
    #[test]
    fn test_derive_ord_struct() {
        let result = derive_ord(&point_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_ord_recursive_fails() {
        let result = derive_ord(&tree_info());
        assert!(matches!(result, Err(AdvDeriveError::RecursiveType { .. })));
    }
    #[test]
    fn test_derive_repr_enum() {
        let result = derive_repr(&color_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("instReprColor"));
    }
    #[test]
    fn test_derive_repr_struct() {
        let result = derive_repr(&point_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_repr_person() {
        let result = derive_repr(&person_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_decidable_eq_enum() {
        let result = derive_decidable_eq(&color_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_decidable_eq_struct() {
        let result = derive_decidable_eq(&point_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_decidable_eq_recursive_fails() {
        let result = derive_decidable_eq(&tree_info());
        assert!(matches!(result, Err(AdvDeriveError::RecursiveType { .. })));
    }
    #[test]
    fn test_derive_hashable_enum() {
        let result = derive_hashable(&color_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_hashable_struct() {
        let result = derive_hashable(&point_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_hashable_recursive_fails() {
        let result = derive_hashable(&tree_info());
        assert!(matches!(result, Err(AdvDeriveError::RecursiveType { .. })));
    }
    #[test]
    fn test_derive_inhabited_enum() {
        let result = derive_inhabited(&color_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_inhabited_struct() {
        let result = derive_inhabited(&point_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_inhabited_empty_fails() {
        let result = derive_inhabited(&empty_info());
        assert!(matches!(result, Err(AdvDeriveError::EmptyType { .. })));
    }
    #[test]
    fn test_derive_nonempty_enum() {
        let result = derive_nonempty(&color_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_nonempty_empty_fails() {
        let result = derive_nonempty(&empty_info());
        assert!(matches!(result, Err(AdvDeriveError::EmptyType { .. })));
    }
    #[test]
    fn test_derive_to_string() {
        let result = derive_to_string(&color_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert!(dr.instance_expr.is_lambda());
    }
    #[test]
    fn test_registry_builtins() {
        let reg = AdvDeriveRegistry::with_builtins();
        assert_eq!(reg.num_handlers(), 8);
        assert!(reg.has_handler(&Name::str("BEq")));
        assert!(reg.has_handler(&Name::str("Ord")));
        assert!(reg.has_handler(&Name::str("Repr")));
        assert!(!reg.has_handler(&Name::str("Unknown")));
    }
    #[test]
    fn test_registry_can_derive() {
        let reg = AdvDeriveRegistry::with_builtins();
        assert!(reg.can_derive(&Name::str("BEq"), &color_info()));
        assert!(!reg.can_derive(&Name::str("BEq"), &tree_info()));
        assert!(reg.can_derive(&Name::str("Repr"), &color_info()));
    }
    #[test]
    fn test_registry_try_derive() {
        let reg = AdvDeriveRegistry::with_builtins();
        let result = reg.try_derive(&AdvDerivableClass::BEq, &color_info());
        assert!(result.is_ok());
        let result = reg.try_derive(&AdvDerivableClass::BEq, &tree_info());
        assert!(result.is_err());
    }
    #[test]
    fn test_registry_try_derive_no_handler() {
        let reg = AdvDeriveRegistry::with_builtins();
        let result = reg.try_derive(&AdvDerivableClass::Custom(Name::str("Foo")), &color_info());
        assert!(matches!(result, Err(AdvDeriveError::NoHandler { .. })));
    }
    #[test]
    fn test_registry_derive_many() {
        let reg = AdvDeriveRegistry::with_builtins();
        let classes = vec![
            AdvDerivableClass::BEq,
            AdvDerivableClass::Repr,
            AdvDerivableClass::Inhabited,
        ];
        let results = reg.derive_many(&classes, &color_info());
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }
    #[test]
    fn test_registry_derive_all_possible() {
        let reg = AdvDeriveRegistry::with_builtins();
        let results = reg.derive_all_possible(&color_info());
        assert_eq!(results.len(), 8);
    }
    #[test]
    fn test_registry_derive_all_possible_recursive() {
        let reg = AdvDeriveRegistry::with_builtins();
        let results = reg.derive_all_possible(&tree_info());
        assert_eq!(results.len(), 4);
    }
    #[test]
    fn test_registry_custom_handler() {
        let mut reg = AdvDeriveRegistry::new();
        struct CustomHandler;
        impl DeriveHandler for CustomHandler {
            fn class_name(&self) -> Name {
                Name::str("MyClass")
            }
            fn can_derive(&self, _type_info: &TypeInfoAdv) -> bool {
                true
            }
            fn derive(&self, type_info: &TypeInfoAdv) -> Result<AdvDeriveResult, AdvDeriveError> {
                Ok(AdvDeriveResult::new(
                    Name::str(format!("instMyClass{}", type_info.name)),
                    Expr::Sort(Level::zero()),
                    Expr::Sort(Level::zero()),
                    vec![],
                ))
            }
        }
        reg.register_handler(Box::new(CustomHandler));
        assert!(reg.has_handler(&Name::str("MyClass")));
        let result = reg.try_derive(
            &AdvDerivableClass::Custom(Name::str("MyClass")),
            &color_info(),
        );
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_builtin_fn() {
        let result = derive_builtin(&AdvDerivableClass::BEq, &color_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_builtin_custom_fails() {
        let result = derive_builtin(&AdvDerivableClass::Custom(Name::str("X")), &color_info());
        assert!(matches!(result, Err(AdvDeriveError::NoHandler { .. })));
    }
    #[test]
    fn test_derive_multiple_fn() {
        let classes = vec![
            AdvDerivableClass::BEq,
            AdvDerivableClass::Hashable,
            AdvDerivableClass::Repr,
        ];
        let results = derive_multiple(&classes, &color_info());
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }
    #[test]
    fn test_derive_multiple_ok_fn() {
        let classes = vec![AdvDerivableClass::BEq, AdvDerivableClass::Hashable];
        let results = derive_multiple_ok(&classes, &tree_info());
        assert!(results.is_empty());
    }
    #[test]
    fn test_full_derive_beq() {
        let result = full_derive(&AdvDerivableClass::BEq, &color_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_full_derive_ord() {
        let result = full_derive(&AdvDerivableClass::Ord, &point_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_full_derive_repr() {
        let result = full_derive(&AdvDerivableClass::Repr, &person_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_full_derive_recursive_fails() {
        let result = full_derive(&AdvDerivableClass::BEq, &tree_info());
        assert!(result.is_err());
    }
    #[test]
    fn test_full_derive_many() {
        let classes = vec![
            AdvDerivableClass::BEq,
            AdvDerivableClass::Ord,
            AdvDerivableClass::Repr,
        ];
        let results = full_derive_many(&classes, &color_info());
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }
    #[test]
    fn test_instance_name() {
        let n = instance_name(&AdvDerivableClass::BEq, &Name::str("Color"));
        assert_eq!(n, Name::str("instBEqColor"));
    }
    #[test]
    fn test_aux_decl_name() {
        let n = aux_decl_name(&Name::str("Color"), "compare");
        assert_eq!(n, Name::str("Color.compare"));
    }
    #[test]
    fn test_validate_derivation_ok() {
        let result = validate_derivation(&AdvDerivableClass::BEq, &color_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_validate_derivation_recursive() {
        let result = validate_derivation(&AdvDerivableClass::BEq, &tree_info());
        assert!(result.is_err());
    }
    #[test]
    fn test_build_parametric_instance_type() {
        let info = TypeInfoAdv {
            name: Name::str("List"),
            univ_params: vec![Name::str("u")],
            params: vec![(
                Name::str("a"),
                Expr::Sort(Level::Param(Name::str("u"))),
                BinderInfo::Implicit,
            )],
            constructors: vec![],
            is_inductive: true,
            is_structure: false,
            is_recursive: true,
            num_indices: 0,
        };
        let ty = build_parametric_instance_type(&AdvDerivableClass::BEq, &info);
        assert!(ty.is_pi());
    }
    #[test]
    fn test_mk_and_chain_empty() {
        let result = mk_and_chain(&[]);
        assert!(matches!(result, Expr::Const(n, _) if n == Name::str("Bool.true")));
    }
    #[test]
    fn test_mk_and_chain_single() {
        let e = Expr::Const(Name::str("x"), vec![]);
        let result = mk_and_chain(std::slice::from_ref(&e));
        assert_eq!(result, e);
    }
    #[test]
    fn test_mk_hash_combine_empty() {
        let result = mk_hash_combine(&[]);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_hash_combine_single() {
        let e = Expr::Const(Name::str("h"), vec![]);
        let result = mk_hash_combine(std::slice::from_ref(&e));
        assert_eq!(result, e);
    }
    #[test]
    fn test_mk_repr_string_no_fields() {
        let result = mk_repr_string(&Name::str("Unit"), &[]);
        assert!(matches!(result, Expr::Lit(Literal::Str(s)) if s == "Unit"));
    }
    #[test]
    fn test_mk_lex_compare_empty() {
        let result = mk_lex_compare(&[]);
        assert!(matches!(result, Expr::Const(n, _) if n == Name::str("Ordering.eq")));
    }
    #[test]
    fn test_mk_lex_compare_single_field() {
        let fields = vec![(Name::str("x"), nat_ty())];
        let result = mk_lex_compare(&fields);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_structure_info() {
        let info = mk_structure_info(Name::str("Point"), vec![(Name::str("x"), nat_ty())]);
        assert!(info.is_structure);
        assert!(info.is_single_ctor());
        assert_eq!(
            info.first_ctor()
                .expect("test operation should succeed")
                .name,
            Name::str("Point.mk")
        );
    }
    #[test]
    fn test_can_derive_builtin_fn() {
        assert!(can_derive_builtin(&AdvDerivableClass::BEq, &color_info()));
        assert!(!can_derive_builtin(&AdvDerivableClass::BEq, &tree_info()));
        assert!(can_derive_builtin(&AdvDerivableClass::Repr, &tree_info()));
        assert!(!can_derive_builtin(
            &AdvDerivableClass::Custom(Name::str("X")),
            &color_info()
        ));
    }
    #[test]
    fn test_check_field_instances() {
        let missing = check_field_instances(&AdvDerivableClass::BEq, &color_info());
        assert!(missing.is_empty());
    }
    #[test]
    fn test_default_registry() {
        let reg = AdvDeriveRegistry::default();
        assert_eq!(reg.num_handlers(), 8);
    }
    #[test]
    fn test_registry_class_names() {
        let reg = AdvDeriveRegistry::with_builtins();
        assert_eq!(reg.class_names().len(), 8);
    }
    #[test]
    fn test_derive_beq_person() {
        let result = derive_beq(&person_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert_eq!(dr.instance_name, Name::str("instBEqPerson"));
    }
    #[test]
    fn test_derive_ord_person() {
        let result = derive_ord(&person_info());
        assert!(result.is_ok());
        let dr = result.expect("test operation should succeed");
        assert!(dr.has_aux_decls());
    }
    #[test]
    fn test_derive_inhabited_person() {
        let result = derive_inhabited(&person_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_nonempty_person() {
        let result = derive_nonempty(&person_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_derive_hashable_person() {
        let result = derive_hashable(&person_info());
        assert!(result.is_ok());
    }
    #[test]
    fn test_type_info_all_field_types() {
        let ti = person_info();
        assert_eq!(ti.all_field_types().len(), 3);
    }
}
