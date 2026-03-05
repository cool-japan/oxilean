//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Name};
use oxilean_parse::AttributeKind;

use super::types::{
    AttrAction, AttrEntry, AttrError, AttrFilter, AttrHandler, AttrInheritance, AttrPipeline,
    AttrPipelineStage, AttrPropagationRule, AttrScope, AttrSnapshot, AttrStats, AttributeManager,
    DeriveHandler, DeriveHandlerRegistry, InstancePriorityQueue, MacroAttr, MacroAttrExpansion,
    ProcessedAttrs, ScopedAttrEntry, SimpEntry, SimpSet,
};

/// Process a list of attributes into structured flags.
///
/// Validates that there are no incompatible combinations
/// (e.g. `@[reducible, irreducible]` or `@[inline, noinline]`).
pub fn process_attributes(attrs: &[AttributeKind]) -> Result<ProcessedAttrs, AttrError> {
    let mut result = ProcessedAttrs::default();
    for attr in attrs {
        match attr {
            AttributeKind::Simp => {
                if result.is_simp {
                    return Err(AttrError::DuplicateAttribute("simp".to_string()));
                }
                result.is_simp = true;
            }
            AttributeKind::Ext => {
                if result.is_ext {
                    return Err(AttrError::DuplicateAttribute("ext".to_string()));
                }
                result.is_ext = true;
            }
            AttributeKind::Instance => {
                if result.is_instance {
                    return Err(AttrError::DuplicateAttribute("instance".to_string()));
                }
                result.is_instance = true;
            }
            AttributeKind::Reducible => {
                if result.is_reducible {
                    return Err(AttrError::DuplicateAttribute("reducible".to_string()));
                }
                if result.is_irreducible {
                    return Err(AttrError::IncompatibleAttributes(
                        "reducible".to_string(),
                        "irreducible".to_string(),
                    ));
                }
                result.is_reducible = true;
            }
            AttributeKind::Irreducible => {
                if result.is_irreducible {
                    return Err(AttrError::DuplicateAttribute("irreducible".to_string()));
                }
                if result.is_reducible {
                    return Err(AttrError::IncompatibleAttributes(
                        "irreducible".to_string(),
                        "reducible".to_string(),
                    ));
                }
                result.is_irreducible = true;
            }
            AttributeKind::Inline => {
                if result.is_inline {
                    return Err(AttrError::DuplicateAttribute("inline".to_string()));
                }
                if result.is_noinline {
                    return Err(AttrError::IncompatibleAttributes(
                        "inline".to_string(),
                        "noinline".to_string(),
                    ));
                }
                result.is_inline = true;
            }
            AttributeKind::NoInline => {
                if result.is_noinline {
                    return Err(AttrError::DuplicateAttribute("noinline".to_string()));
                }
                if result.is_inline {
                    return Err(AttrError::IncompatibleAttributes(
                        "noinline".to_string(),
                        "inline".to_string(),
                    ));
                }
                result.is_noinline = true;
            }
            AttributeKind::SpecializeAttr => {
                if result.is_specialize {
                    return Err(AttrError::DuplicateAttribute("specialize".to_string()));
                }
                result.is_specialize = true;
            }
            AttributeKind::Custom(name) => {
                if result.custom.iter().any(|(n, _)| n == name) {
                    return Err(AttrError::DuplicateAttribute(name.clone()));
                }
                result.custom.push((name.clone(), Vec::new()));
            }
        }
    }
    Ok(result)
}
/// Apply a list of parsed attributes to a declaration.
///
/// This validates the attributes, creates entries, and registers them
/// with the attribute manager.
pub fn apply_attributes(
    manager: &mut AttributeManager,
    decl_name: &Name,
    attrs: &[AttributeKind],
) -> Result<(), AttrError> {
    let _processed = process_attributes(attrs)?;
    for attr in attrs {
        let entry = AttrEntry::new(attr.clone(), decl_name.clone());
        manager.register_attribute(entry)?;
    }
    Ok(())
}
/// Apply a list of parsed attributes with a specific priority.
#[allow(dead_code)]
pub fn apply_attributes_with_priority(
    manager: &mut AttributeManager,
    decl_name: &Name,
    attrs: &[AttributeKind],
    priority: u32,
) -> Result<(), AttrError> {
    let _processed = process_attributes(attrs)?;
    for attr in attrs {
        let entry = AttrEntry::with_priority(attr.clone(), decl_name.clone(), priority);
        manager.register_attribute(entry)?;
    }
    Ok(())
}
/// Check if two attribute kinds are incompatible.
pub fn check_incompatible(a: &AttributeKind, b: &AttributeKind) -> Option<AttrError> {
    match (a, b) {
        (AttributeKind::Reducible, AttributeKind::Irreducible)
        | (AttributeKind::Irreducible, AttributeKind::Reducible) => Some(
            AttrError::IncompatibleAttributes("reducible".to_string(), "irreducible".to_string()),
        ),
        (AttributeKind::Inline, AttributeKind::NoInline)
        | (AttributeKind::NoInline, AttributeKind::Inline) => Some(
            AttrError::IncompatibleAttributes("inline".to_string(), "noinline".to_string()),
        ),
        _ => None,
    }
}
/// Convert a `ProcessedAttrs` back to a list of `AttributeKind`.
#[allow(dead_code)]
pub fn processed_to_kinds(processed: &ProcessedAttrs) -> Vec<AttributeKind> {
    let mut kinds = Vec::new();
    if processed.is_simp {
        kinds.push(AttributeKind::Simp);
    }
    if processed.is_ext {
        kinds.push(AttributeKind::Ext);
    }
    if processed.is_instance {
        kinds.push(AttributeKind::Instance);
    }
    if processed.is_reducible {
        kinds.push(AttributeKind::Reducible);
    }
    if processed.is_irreducible {
        kinds.push(AttributeKind::Irreducible);
    }
    if processed.is_inline {
        kinds.push(AttributeKind::Inline);
    }
    if processed.is_noinline {
        kinds.push(AttributeKind::NoInline);
    }
    if processed.is_specialize {
        kinds.push(AttributeKind::SpecializeAttr);
    }
    for (name, _) in &processed.custom {
        kinds.push(AttributeKind::Custom(name.clone()));
    }
    kinds
}
/// Get the default handler action for a built-in attribute kind.
#[allow(dead_code)]
pub fn default_action(kind: &AttributeKind) -> Option<AttrAction> {
    match kind {
        AttributeKind::Simp => Some(AttrAction::AddToSimpSet),
        AttributeKind::Ext => Some(AttrAction::AddToExtSet),
        AttributeKind::Instance => Some(AttrAction::MarkAsInstance),
        AttributeKind::Reducible => Some(AttrAction::SetReducibility(true)),
        AttributeKind::Irreducible => Some(AttrAction::SetReducibility(false)),
        AttributeKind::Inline => Some(AttrAction::SetInline(true)),
        AttributeKind::NoInline => Some(AttrAction::SetInline(false)),
        AttributeKind::SpecializeAttr => None,
        AttributeKind::Custom(name) => Some(AttrAction::Custom(name.clone())),
    }
}
/// Create a standard set of built-in handlers.
#[allow(dead_code)]
pub fn builtin_handlers() -> Vec<AttrHandler> {
    vec![
        AttrHandler::new(
            "simp",
            "Add lemma to the default simp set",
            AttrAction::AddToSimpSet,
        ),
        AttrHandler::new(
            "ext",
            "Mark lemma as extensionality lemma",
            AttrAction::AddToExtSet,
        ),
        AttrHandler::new(
            "instance",
            "Register as a typeclass instance",
            AttrAction::MarkAsInstance,
        ),
        AttrHandler::new(
            "reducible",
            "Mark definition as reducible",
            AttrAction::SetReducibility(true),
        ),
        AttrHandler::new(
            "irreducible",
            "Mark definition as irreducible",
            AttrAction::SetReducibility(false),
        ),
        AttrHandler::new(
            "inline",
            "Hint to inline this definition",
            AttrAction::SetInline(true),
        ),
        AttrHandler::new(
            "noinline",
            "Prevent inlining of this definition",
            AttrAction::SetInline(false),
        ),
    ]
}
/// Create an attribute manager pre-loaded with built-in handlers.
#[allow(dead_code)]
pub fn create_default_manager() -> AttributeManager {
    let mut mgr = AttributeManager::new();
    for handler in builtin_handlers() {
        mgr.register_custom_handler(handler);
    }
    mgr
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::*;
    use oxilean_kernel::Literal;
    #[test]
    fn test_attr_entry_new() {
        let entry = AttrEntry::new(AttributeKind::Simp, Name::str("my_lemma"));
        assert_eq!(entry.kind, AttributeKind::Simp);
        assert_eq!(entry.decl_name, Name::str("my_lemma"));
        assert!(entry.args.is_empty());
        assert_eq!(entry.priority, 1000);
    }
    #[test]
    fn test_attr_entry_with_priority() {
        let entry = AttrEntry::with_priority(AttributeKind::Instance, Name::str("inst"), 500);
        assert_eq!(entry.priority, 500);
        assert_eq!(entry.kind, AttributeKind::Instance);
    }
    #[test]
    fn test_attr_entry_with_args() {
        let args = vec![Expr::Lit(Literal::Nat(42))];
        let entry = AttrEntry::with_args(
            AttributeKind::Custom("my_attr".into()),
            Name::str("f"),
            args,
        );
        assert_eq!(entry.args.len(), 1);
        assert_eq!(entry.kind_name(), "my_attr");
    }
    #[test]
    fn test_attr_entry_kind_name() {
        assert_eq!(
            AttrEntry::new(AttributeKind::Simp, Name::str("x")).kind_name(),
            "simp"
        );
        assert_eq!(
            AttrEntry::new(AttributeKind::Ext, Name::str("x")).kind_name(),
            "ext"
        );
        assert_eq!(
            AttrEntry::new(AttributeKind::Instance, Name::str("x")).kind_name(),
            "instance"
        );
    }
    #[test]
    fn test_processed_attrs_empty() {
        let p = ProcessedAttrs::empty();
        assert!(!p.has_any());
        assert_eq!(p.count(), 0);
    }
    #[test]
    fn test_processed_attrs_has_any() {
        let mut p = ProcessedAttrs::default();
        assert!(!p.has_any());
        p.is_simp = true;
        assert!(p.has_any());
    }
    #[test]
    fn test_processed_attrs_count() {
        let p = ProcessedAttrs {
            is_simp: true,
            is_ext: true,
            is_inline: true,
            ..ProcessedAttrs::default()
        };
        assert_eq!(p.count(), 3);
    }
    #[test]
    fn test_processed_attrs_count_with_custom() {
        let p = ProcessedAttrs {
            is_simp: true,
            custom: vec![("my_attr".to_string(), vec![])],
            ..ProcessedAttrs::default()
        };
        assert_eq!(p.count(), 2);
    }
    #[test]
    fn test_process_empty() {
        let result = process_attributes(&[]).expect("test operation should succeed");
        assert!(!result.has_any());
    }
    #[test]
    fn test_process_simp() {
        let result =
            process_attributes(&[AttributeKind::Simp]).expect("test operation should succeed");
        assert!(result.is_simp);
        assert!(!result.is_ext);
        assert_eq!(result.count(), 1);
    }
    #[test]
    fn test_process_multiple() {
        let attrs = vec![
            AttributeKind::Simp,
            AttributeKind::Ext,
            AttributeKind::Instance,
        ];
        let result = process_attributes(&attrs).expect("test operation should succeed");
        assert!(result.is_simp);
        assert!(result.is_ext);
        assert!(result.is_instance);
        assert_eq!(result.count(), 3);
    }
    #[test]
    fn test_process_reducible() {
        let result =
            process_attributes(&[AttributeKind::Reducible]).expect("test operation should succeed");
        assert!(result.is_reducible);
        assert!(!result.is_irreducible);
    }
    #[test]
    fn test_process_inline() {
        let result =
            process_attributes(&[AttributeKind::Inline]).expect("test operation should succeed");
        assert!(result.is_inline);
        assert!(!result.is_noinline);
    }
    #[test]
    fn test_process_specialize() {
        let result = process_attributes(&[AttributeKind::SpecializeAttr])
            .expect("test operation should succeed");
        assert!(result.is_specialize);
    }
    #[test]
    fn test_process_custom() {
        let attrs = vec![AttributeKind::Custom("my_attr".to_string())];
        let result = process_attributes(&attrs).expect("test operation should succeed");
        assert_eq!(result.custom.len(), 1);
        assert_eq!(result.custom[0].0, "my_attr");
    }
    #[test]
    fn test_process_duplicate_simp_error() {
        let attrs = vec![AttributeKind::Simp, AttributeKind::Simp];
        let err = process_attributes(&attrs).unwrap_err();
        assert_eq!(err, AttrError::DuplicateAttribute("simp".to_string()));
    }
    #[test]
    fn test_process_duplicate_custom_error() {
        let attrs = vec![
            AttributeKind::Custom("foo".to_string()),
            AttributeKind::Custom("foo".to_string()),
        ];
        let err = process_attributes(&attrs).unwrap_err();
        assert_eq!(err, AttrError::DuplicateAttribute("foo".to_string()));
    }
    #[test]
    fn test_process_incompatible_reducible_irreducible() {
        let attrs = vec![AttributeKind::Reducible, AttributeKind::Irreducible];
        let err = process_attributes(&attrs).unwrap_err();
        assert!(matches!(err, AttrError::IncompatibleAttributes(_, _)));
    }
    #[test]
    fn test_process_incompatible_irreducible_reducible() {
        let attrs = vec![AttributeKind::Irreducible, AttributeKind::Reducible];
        let err = process_attributes(&attrs).unwrap_err();
        assert!(matches!(err, AttrError::IncompatibleAttributes(_, _)));
    }
    #[test]
    fn test_process_incompatible_inline_noinline() {
        let attrs = vec![AttributeKind::Inline, AttributeKind::NoInline];
        let err = process_attributes(&attrs).unwrap_err();
        assert!(matches!(err, AttrError::IncompatibleAttributes(_, _)));
    }
    #[test]
    fn test_process_incompatible_noinline_inline() {
        let attrs = vec![AttributeKind::NoInline, AttributeKind::Inline];
        let err = process_attributes(&attrs).unwrap_err();
        assert!(matches!(err, AttrError::IncompatibleAttributes(_, _)));
    }
    #[test]
    fn test_manager_new() {
        let mgr = AttributeManager::new();
        assert_eq!(mgr.total_entries(), 0);
        assert_eq!(mgr.num_attributed_decls(), 0);
    }
    #[test]
    fn test_manager_register() {
        let mut mgr = AttributeManager::new();
        let entry = AttrEntry::new(AttributeKind::Simp, Name::str("lemma1"));
        mgr.register_attribute(entry)
            .expect("test operation should succeed");
        assert_eq!(mgr.total_entries(), 1);
        assert!(mgr.is_simp(&Name::str("lemma1")));
    }
    #[test]
    fn test_manager_register_multiple_kinds() {
        let mut mgr = AttributeManager::new();
        let name = Name::str("my_fn");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, name.clone()))
            .expect("test operation should succeed");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Ext, name.clone()))
            .expect("test operation should succeed");
        assert_eq!(mgr.total_entries(), 2);
        assert!(mgr.is_simp(&name));
        assert_eq!(mgr.get_attributes(&name).len(), 2);
    }
    #[test]
    fn test_manager_register_duplicate_error() {
        let mut mgr = AttributeManager::new();
        let name = Name::str("f");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, name.clone()))
            .expect("test operation should succeed");
        let err = mgr
            .register_attribute(AttrEntry::new(AttributeKind::Simp, name))
            .unwrap_err();
        assert!(matches!(err, AttrError::DuplicateAttribute(_)));
    }
    #[test]
    fn test_manager_register_incompatible_error() {
        let mut mgr = AttributeManager::new();
        let name = Name::str("f");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Reducible, name.clone()))
            .expect("test operation should succeed");
        let err = mgr
            .register_attribute(AttrEntry::new(AttributeKind::Irreducible, name))
            .unwrap_err();
        assert!(matches!(err, AttrError::IncompatibleAttributes(_, _)));
    }
    #[test]
    fn test_manager_unregister() {
        let mut mgr = AttributeManager::new();
        let name = Name::str("f");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, name.clone()))
            .expect("test operation should succeed");
        assert!(mgr.is_simp(&name));
        let removed = mgr.unregister_attribute(&name, &AttributeKind::Simp);
        assert!(removed);
        assert!(!mgr.is_simp(&name));
        assert_eq!(mgr.total_entries(), 0);
    }
    #[test]
    fn test_manager_unregister_nonexistent() {
        let mut mgr = AttributeManager::new();
        let removed = mgr.unregister_attribute(&Name::str("f"), &AttributeKind::Simp);
        assert!(!removed);
    }
    #[test]
    fn test_manager_get_simp_lemmas() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, Name::str("a")))
            .expect("test operation should succeed");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, Name::str("b")))
            .expect("test operation should succeed");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Ext, Name::str("c")))
            .expect("test operation should succeed");
        let simps = mgr.get_simp_lemmas();
        assert_eq!(simps.len(), 2);
        assert!(simps.contains(&Name::str("a")));
        assert!(simps.contains(&Name::str("b")));
    }
    #[test]
    fn test_manager_get_ext_lemmas() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Ext, Name::str("ext1")))
            .expect("test operation should succeed");
        let exts = mgr.get_ext_lemmas();
        assert_eq!(exts.len(), 1);
    }
    #[test]
    fn test_manager_get_instances() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Instance, Name::str("inst1")))
            .expect("test operation should succeed");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Instance, Name::str("inst2")))
            .expect("test operation should succeed");
        let insts = mgr.get_instances();
        assert_eq!(insts.len(), 2);
    }
    #[test]
    fn test_manager_get_reducible() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Reducible, Name::str("f")))
            .expect("test operation should succeed");
        assert_eq!(mgr.get_reducible().len(), 1);
        assert!(mgr.is_reducible(&Name::str("f")));
        assert!(!mgr.is_irreducible(&Name::str("f")));
    }
    #[test]
    fn test_manager_get_irreducible() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Irreducible, Name::str("g")))
            .expect("test operation should succeed");
        assert_eq!(mgr.get_irreducible().len(), 1);
        assert!(mgr.is_irreducible(&Name::str("g")));
    }
    #[test]
    fn test_manager_get_inline() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Inline, Name::str("h")))
            .expect("test operation should succeed");
        assert_eq!(mgr.get_inline().len(), 1);
        assert!(mgr.is_inline(&Name::str("h")));
    }
    #[test]
    fn test_manager_is_instance() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Instance, Name::str("inst")))
            .expect("test operation should succeed");
        assert!(mgr.is_instance(&Name::str("inst")));
        assert!(!mgr.is_instance(&Name::str("other")));
    }
    #[test]
    fn test_manager_get_by_kind() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(
            AttributeKind::Custom("my_tag".to_string()),
            Name::str("f"),
        ))
        .expect("test operation should succeed");
        mgr.register_attribute(AttrEntry::new(
            AttributeKind::Custom("my_tag".to_string()),
            Name::str("g"),
        ))
        .expect("test operation should succeed");
        let tagged = mgr.get_by_kind("my_tag");
        assert_eq!(tagged.len(), 2);
    }
    #[test]
    fn test_manager_has_attribute() {
        let mut mgr = AttributeManager::new();
        let name = Name::str("f");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, name.clone()))
            .expect("test operation should succeed");
        assert!(mgr.has_attribute(&name, &AttributeKind::Simp));
        assert!(!mgr.has_attribute(&name, &AttributeKind::Ext));
    }
    #[test]
    fn test_manager_validate_ok() {
        let mgr = AttributeManager::new();
        let attrs = vec![AttributeKind::Simp, AttributeKind::Ext];
        assert!(mgr.validate_attributes(&attrs).is_ok());
    }
    #[test]
    fn test_manager_validate_incompatible() {
        let mgr = AttributeManager::new();
        let attrs = vec![AttributeKind::Reducible, AttributeKind::Irreducible];
        assert!(mgr.validate_attributes(&attrs).is_err());
    }
    #[test]
    fn test_manager_validate_duplicate() {
        let mgr = AttributeManager::new();
        let attrs = vec![AttributeKind::Simp, AttributeKind::Simp];
        assert!(mgr.validate_attributes(&attrs).is_err());
    }
    #[test]
    fn test_manager_register_custom_handler() {
        let mut mgr = AttributeManager::new();
        let handler = AttrHandler::new("my_tag", "Custom tag", AttrAction::Custom("my_tag".into()));
        mgr.register_custom_handler(handler);
        assert!(mgr.get_handler("my_tag").is_some());
        assert!(mgr.get_handler("other").is_none());
    }
    #[test]
    fn test_manager_clear() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, Name::str("f")))
            .expect("test operation should succeed");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Ext, Name::str("g")))
            .expect("test operation should succeed");
        assert_eq!(mgr.total_entries(), 2);
        mgr.clear();
        assert_eq!(mgr.total_entries(), 0);
    }
    #[test]
    fn test_manager_merge() {
        let mut mgr1 = AttributeManager::new();
        mgr1.register_attribute(AttrEntry::new(AttributeKind::Simp, Name::str("a")))
            .expect("test operation should succeed");
        let mut mgr2 = AttributeManager::new();
        mgr2.register_attribute(AttrEntry::new(AttributeKind::Ext, Name::str("b")))
            .expect("test operation should succeed");
        mgr1.merge(&mgr2).expect("test operation should succeed");
        assert_eq!(mgr1.total_entries(), 2);
        assert!(mgr1.is_simp(&Name::str("a")));
    }
    #[test]
    fn test_manager_all_attributed_names() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, Name::str("a")))
            .expect("test operation should succeed");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Ext, Name::str("b")))
            .expect("test operation should succeed");
        let names = mgr.all_attributed_names();
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_apply_attributes_basic() {
        let mut mgr = AttributeManager::new();
        let name = Name::str("my_lemma");
        let attrs = vec![AttributeKind::Simp, AttributeKind::Ext];
        apply_attributes(&mut mgr, &name, &attrs).expect("test operation should succeed");
        assert!(mgr.is_simp(&name));
    }
    #[test]
    fn test_apply_attributes_incompatible_error() {
        let mut mgr = AttributeManager::new();
        let name = Name::str("f");
        let attrs = vec![AttributeKind::Reducible, AttributeKind::Irreducible];
        assert!(apply_attributes(&mut mgr, &name, &attrs).is_err());
    }
    #[test]
    fn test_apply_attributes_with_priority() {
        let mut mgr = AttributeManager::new();
        let name = Name::str("inst");
        let attrs = vec![AttributeKind::Instance];
        apply_attributes_with_priority(&mut mgr, &name, &attrs, 500)
            .expect("test operation should succeed");
        let entries = mgr.get_attributes(&name);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].priority, 500);
    }
    #[test]
    fn test_processed_to_kinds() {
        let p = ProcessedAttrs {
            is_simp: true,
            is_reducible: true,
            ..ProcessedAttrs::default()
        };
        let kinds = processed_to_kinds(&p);
        assert_eq!(kinds.len(), 2);
        assert!(kinds.contains(&AttributeKind::Simp));
        assert!(kinds.contains(&AttributeKind::Reducible));
    }
    #[test]
    fn test_default_action() {
        assert_eq!(
            default_action(&AttributeKind::Simp),
            Some(AttrAction::AddToSimpSet)
        );
        assert_eq!(
            default_action(&AttributeKind::Ext),
            Some(AttrAction::AddToExtSet)
        );
        assert_eq!(
            default_action(&AttributeKind::Instance),
            Some(AttrAction::MarkAsInstance)
        );
        assert_eq!(
            default_action(&AttributeKind::Reducible),
            Some(AttrAction::SetReducibility(true))
        );
        assert_eq!(
            default_action(&AttributeKind::Irreducible),
            Some(AttrAction::SetReducibility(false))
        );
        assert_eq!(
            default_action(&AttributeKind::Inline),
            Some(AttrAction::SetInline(true))
        );
        assert_eq!(
            default_action(&AttributeKind::NoInline),
            Some(AttrAction::SetInline(false))
        );
        assert_eq!(default_action(&AttributeKind::SpecializeAttr), None);
    }
    #[test]
    fn test_builtin_handlers() {
        let handlers = builtin_handlers();
        assert_eq!(handlers.len(), 7);
    }
    #[test]
    fn test_create_default_manager() {
        let mgr = create_default_manager();
        assert!(mgr.get_handler("simp").is_some());
        assert!(mgr.get_handler("ext").is_some());
        assert!(mgr.get_handler("instance").is_some());
        assert!(mgr.get_handler("reducible").is_some());
        assert!(mgr.get_handler("irreducible").is_some());
        assert!(mgr.get_handler("inline").is_some());
        assert!(mgr.get_handler("noinline").is_some());
    }
    #[test]
    fn test_attr_error_display() {
        assert_eq!(
            format!("{}", AttrError::UnknownAttribute("foo".into())),
            "unknown attribute: foo"
        );
        assert_eq!(
            format!("{}", AttrError::InvalidArgs("bad".into())),
            "invalid attribute arguments: bad"
        );
        assert_eq!(
            format!("{}", AttrError::DuplicateAttribute("simp".into())),
            "duplicate attribute: simp"
        );
        assert_eq!(
            format!(
                "{}",
                AttrError::IncompatibleAttributes("a".into(), "b".into())
            ),
            "incompatible attributes: a and b"
        );
        assert_eq!(
            format!("{}", AttrError::Other("misc".into())),
            "attribute error: misc"
        );
    }
    #[test]
    fn test_attr_handler_new() {
        let h = AttrHandler::new("test", "Test handler", AttrAction::AddToSimpSet);
        assert_eq!(h.name, "test");
        assert_eq!(h.doc, "Test handler");
        assert_eq!(h.action, AttrAction::AddToSimpSet);
    }
    #[test]
    fn test_attr_action_variants() {
        let actions = [
            AttrAction::AddToSimpSet,
            AttrAction::AddToExtSet,
            AttrAction::MarkAsInstance,
            AttrAction::SetReducibility(true),
            AttrAction::SetReducibility(false),
            AttrAction::SetInline(true),
            AttrAction::SetInline(false),
            AttrAction::Custom("foo".into()),
        ];
        assert_eq!(actions.len(), 8);
    }
}
/// Detects conflicting attribute combinations across all declarations.
#[allow(dead_code)]
pub fn detect_attr_conflicts(manager: &AttributeManager) -> Vec<(Name, AttrError)> {
    let mut conflicts = Vec::new();
    for (name, entries) in &manager.entries {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                if let Some(err) = check_incompatible(&entries[i].kind, &entries[j].kind) {
                    conflicts.push((name.clone(), err));
                }
            }
        }
    }
    conflicts
}
/// Remove all conflicting attributes from the manager.
///
/// For each conflict, the second (lower-priority) attribute is removed.
#[allow(dead_code)]
pub fn resolve_attr_conflicts(manager: &mut AttributeManager) -> usize {
    let conflicts = detect_attr_conflicts(manager);
    let count = conflicts.len();
    for (name, _err) in conflicts {
        manager.unregister_attribute(&name, &AttributeKind::Irreducible);
        manager.unregister_attribute(&name, &AttributeKind::NoInline);
    }
    count
}
/// Check whether a list of attributes contains a `@[simp]` attribute.
#[allow(dead_code)]
pub fn has_simp(attrs: &[AttributeKind]) -> bool {
    attrs.contains(&AttributeKind::Simp)
}
/// Check whether a list of attributes contains a `@[instance]` attribute.
#[allow(dead_code)]
pub fn has_instance(attrs: &[AttributeKind]) -> bool {
    attrs.contains(&AttributeKind::Instance)
}
/// Check whether a list of attributes contains a `@[reducible]` attribute.
#[allow(dead_code)]
pub fn has_reducible(attrs: &[AttributeKind]) -> bool {
    attrs.contains(&AttributeKind::Reducible)
}
/// Check whether a list of attributes contains a `@[inline]` attribute.
#[allow(dead_code)]
pub fn has_inline(attrs: &[AttributeKind]) -> bool {
    attrs.contains(&AttributeKind::Inline)
}
/// Filter a list of attributes to only the simp-relevant ones.
#[allow(dead_code)]
pub fn simp_attrs(attrs: &[AttributeKind]) -> Vec<&AttributeKind> {
    attrs
        .iter()
        .filter(|k| matches!(k, AttributeKind::Simp))
        .collect()
}
/// Count the number of custom attributes in a list.
#[allow(dead_code)]
pub fn count_custom_attrs(attrs: &[AttributeKind]) -> usize {
    attrs
        .iter()
        .filter(|k| matches!(k, AttributeKind::Custom(_)))
        .count()
}
/// Get the name of each custom attribute.
#[allow(dead_code)]
pub fn custom_attr_names(attrs: &[AttributeKind]) -> Vec<&str> {
    attrs
        .iter()
        .filter_map(|k| {
            if let AttributeKind::Custom(name) = k {
                Some(name.as_str())
            } else {
                None
            }
        })
        .collect()
}
/// Convert a string attribute name to the corresponding `AttributeKind`.
///
/// Returns `None` for unrecognised names (they become `Custom`).
#[allow(dead_code)]
pub fn kind_from_str(name: &str) -> AttributeKind {
    match name {
        "simp" => AttributeKind::Simp,
        "ext" => AttributeKind::Ext,
        "instance" => AttributeKind::Instance,
        "reducible" => AttributeKind::Reducible,
        "irreducible" => AttributeKind::Irreducible,
        "inline" => AttributeKind::Inline,
        "noinline" => AttributeKind::NoInline,
        "specialize" => AttributeKind::SpecializeAttr,
        other => AttributeKind::Custom(other.to_string()),
    }
}
/// Produce a compact string representation of a list of attribute kinds.
#[allow(dead_code)]
pub fn format_attrs(attrs: &[AttributeKind]) -> String {
    if attrs.is_empty() {
        return String::new();
    }
    let names: Vec<&str> = attrs.iter().map(|k| k.name()).collect();
    format!("@[{}]", names.join(", "))
}
#[cfg(test)]
mod extra_attr_tests {
    use super::*;
    use crate::attribute::*;
    #[test]
    fn test_simp_set_insert_order() {
        let mut ss = SimpSet::new();
        ss.insert(SimpEntry::new(Name::str("b"), 500));
        ss.insert(SimpEntry::new(Name::str("a"), 1000));
        ss.insert(SimpEntry::new(Name::str("c"), 100));
        let entries = ss.entries();
        assert_eq!(entries[0].name, Name::str("a"));
        assert_eq!(entries[2].name, Name::str("c"));
    }
    #[test]
    fn test_simp_set_remove() {
        let mut ss = SimpSet::new();
        ss.insert(SimpEntry::new(Name::str("f"), 1000));
        assert!(ss.contains(&Name::str("f")));
        let removed = ss.remove(&Name::str("f"));
        assert!(removed);
        assert!(!ss.contains(&Name::str("f")));
        assert!(ss.is_empty());
    }
    #[test]
    fn test_simp_set_by_tag() {
        let mut ss = SimpSet::new();
        ss.insert(SimpEntry::new(Name::str("a"), 1000).with_tag("algebra"));
        ss.insert(SimpEntry::new(Name::str("b"), 500));
        let tagged = ss.by_tag("algebra");
        assert_eq!(tagged.len(), 1);
        assert_eq!(tagged[0].name, Name::str("a"));
    }
    #[test]
    fn test_simp_set_forward_reverse() {
        let mut ss = SimpSet::new();
        ss.insert(SimpEntry::new(Name::str("fwd"), 1000));
        ss.insert(SimpEntry::reverse(Name::str("rev"), 500));
        assert_eq!(ss.forward_entries().len(), 1);
        assert_eq!(ss.reverse_entries().len(), 1);
    }
    #[test]
    fn test_instance_queue_empty() {
        let q = InstancePriorityQueue::new();
        assert!(q.is_empty());
        assert!(q.peek().is_none());
    }
    #[test]
    fn test_instance_queue_insert_pop() {
        let mut q = InstancePriorityQueue::new();
        q.insert(Name::str("inst1"), 100);
        q.insert(Name::str("inst2"), 500);
        assert_eq!(q.len(), 2);
        let top = q.pop().expect("collection should not be empty");
        assert_eq!(top.0, Name::str("inst2"));
        assert_eq!(q.len(), 1);
    }
    #[test]
    fn test_instance_queue_names_in_order() {
        let mut q = InstancePriorityQueue::new();
        q.insert(Name::str("low"), 100);
        q.insert(Name::str("high"), 1000);
        let names = q.names_in_order();
        assert_eq!(*names[0], Name::str("high"));
    }
    #[test]
    fn test_propagation_rule_applies_no_condition() {
        let rule = AttrPropagationRule::new(AttributeKind::Simp, AttributeKind::Ext);
        assert!(rule.applies(&Name::str("anything")));
    }
    #[test]
    fn test_propagation_rule_applies_with_condition() {
        let rule =
            AttrPropagationRule::with_condition(AttributeKind::Simp, AttributeKind::Ext, "Nat.");
        assert!(rule.applies(&Name::str("Nat.add")));
        assert!(!rule.applies(&Name::str("Int.add")));
    }
    #[test]
    fn test_attr_inheritance() {
        let mut inh = AttrInheritance::new();
        inh.register(Name::str("base"), Name::str("derived1"));
        inh.register(Name::str("base"), Name::str("derived2"));
        let ds = inh.derived_from(&Name::str("base"));
        assert_eq!(ds.len(), 2);
        assert!(inh.inherits_from(&Name::str("base"), &Name::str("derived1")));
        assert!(!inh.inherits_from(&Name::str("base"), &Name::str("other")));
    }
    #[test]
    fn test_attr_scope_global() {
        let s = AttrScope::Global;
        assert!(s.is_global());
        assert!(!s.is_local());
        assert!(s.namespace().is_none());
    }
    #[test]
    fn test_attr_scope_namespace() {
        let s = AttrScope::Namespace("Algebra".to_string());
        assert!(!s.is_global());
        assert_eq!(s.namespace(), Some("Algebra"));
    }
    #[test]
    fn test_attr_filter_any() {
        let f = AttrFilter::any();
        let entry = AttrEntry::new(AttributeKind::Simp, Name::str("x"));
        assert!(f.matches(&entry));
    }
    #[test]
    fn test_attr_filter_for_kind() {
        let f = AttrFilter::for_kind(AttributeKind::Simp);
        let simp_entry = AttrEntry::new(AttributeKind::Simp, Name::str("x"));
        let ext_entry = AttrEntry::new(AttributeKind::Ext, Name::str("y"));
        assert!(f.matches(&simp_entry));
        assert!(!f.matches(&ext_entry));
    }
    #[test]
    fn test_attr_filter_priority_range() {
        let f = AttrFilter::priority_range(500, 1500);
        let mut entry = AttrEntry::new(AttributeKind::Simp, Name::str("x"));
        entry.priority = 1000;
        assert!(f.matches(&entry));
        entry.priority = 100;
        assert!(!f.matches(&entry));
        entry.priority = 2000;
        assert!(!f.matches(&entry));
    }
    #[test]
    fn test_attr_stats_collect() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, Name::str("a")))
            .expect("test operation should succeed");
        mgr.register_attribute(AttrEntry::new(AttributeKind::Instance, Name::str("b")))
            .expect("test operation should succeed");
        mgr.register_attribute(AttrEntry::new(
            AttributeKind::Custom("tag".into()),
            Name::str("c"),
        ))
        .expect("test operation should succeed");
        let stats = AttrStats::collect(&mgr);
        assert_eq!(stats.total, 3);
        assert_eq!(stats.simp_count, 1);
        assert_eq!(stats.instance_count, 1);
        assert_eq!(stats.custom_count, 1);
    }
    #[test]
    fn test_detect_attr_conflicts_none() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, Name::str("f")))
            .expect("test operation should succeed");
        let conflicts = detect_attr_conflicts(&mgr);
        assert!(conflicts.is_empty());
    }
    #[test]
    fn test_attr_snapshot_take_restore() {
        let mut mgr = AttributeManager::new();
        mgr.register_attribute(AttrEntry::new(AttributeKind::Simp, Name::str("f")))
            .expect("test operation should succeed");
        let snap = AttrSnapshot::take(&mgr);
        assert_eq!(snap.total_entries(), 1);
        mgr.register_attribute(AttrEntry::new(AttributeKind::Ext, Name::str("g")))
            .expect("test operation should succeed");
        assert_eq!(mgr.total_entries(), 2);
        snap.restore(&mut mgr);
        assert_eq!(mgr.total_entries(), 1);
        assert!(mgr.is_simp(&Name::str("f")));
    }
    #[test]
    fn test_has_simp() {
        assert!(has_simp(&[AttributeKind::Simp, AttributeKind::Ext]));
        assert!(!has_simp(&[AttributeKind::Ext]));
    }
    #[test]
    fn test_has_instance() {
        assert!(has_instance(&[AttributeKind::Instance]));
        assert!(!has_instance(&[AttributeKind::Simp]));
    }
    #[test]
    fn test_has_reducible() {
        assert!(has_reducible(&[AttributeKind::Reducible]));
        assert!(!has_reducible(&[AttributeKind::Irreducible]));
    }
    #[test]
    fn test_has_inline() {
        assert!(has_inline(&[AttributeKind::Inline]));
        assert!(!has_inline(&[AttributeKind::NoInline]));
    }
    #[test]
    fn test_count_custom_attrs() {
        let attrs = vec![
            AttributeKind::Simp,
            AttributeKind::Custom("a".into()),
            AttributeKind::Custom("b".into()),
        ];
        assert_eq!(count_custom_attrs(&attrs), 2);
    }
    #[test]
    fn test_custom_attr_names() {
        let attrs = vec![
            AttributeKind::Simp,
            AttributeKind::Custom("foo".into()),
            AttributeKind::Custom("bar".into()),
        ];
        let names = custom_attr_names(&attrs);
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"bar"));
    }
    #[test]
    fn test_kind_from_str() {
        assert_eq!(kind_from_str("simp"), AttributeKind::Simp);
        assert_eq!(kind_from_str("ext"), AttributeKind::Ext);
        assert_eq!(kind_from_str("instance"), AttributeKind::Instance);
        assert_eq!(kind_from_str("reducible"), AttributeKind::Reducible);
        assert_eq!(kind_from_str("irreducible"), AttributeKind::Irreducible);
        assert_eq!(kind_from_str("inline"), AttributeKind::Inline);
        assert_eq!(kind_from_str("noinline"), AttributeKind::NoInline);
        assert_eq!(kind_from_str("specialize"), AttributeKind::SpecializeAttr);
        assert!(matches!(kind_from_str("unknown"), AttributeKind::Custom(_)));
    }
    #[test]
    fn test_format_attrs_empty() {
        let s = format_attrs(&[]);
        assert_eq!(s, "");
    }
    #[test]
    fn test_format_attrs_one() {
        let s = format_attrs(&[AttributeKind::Simp]);
        assert_eq!(s, "@[simp]");
    }
    #[test]
    fn test_format_attrs_multiple() {
        let s = format_attrs(&[AttributeKind::Simp, AttributeKind::Ext]);
        assert!(s.contains("simp"));
        assert!(s.contains("ext"));
    }
    #[test]
    fn test_simp_attrs() {
        let attrs = vec![AttributeKind::Simp, AttributeKind::Ext, AttributeKind::Simp];
        let simps = simp_attrs(&attrs);
        assert_eq!(simps.len(), 2);
    }
    #[test]
    fn test_scoped_attr_global() {
        let entry = AttrEntry::new(AttributeKind::Simp, Name::str("f"));
        let scoped = ScopedAttrEntry::global(entry);
        assert!(scoped.scope.is_global());
    }
    #[test]
    fn test_scoped_attr_local() {
        let entry = AttrEntry::new(AttributeKind::Ext, Name::str("g"));
        let scoped = ScopedAttrEntry::local(entry);
        assert!(scoped.scope.is_local());
    }
    #[test]
    fn test_scoped_attr_namespace() {
        let entry = AttrEntry::new(AttributeKind::Instance, Name::str("inst"));
        let scoped = ScopedAttrEntry::in_namespace(entry, "MyNs");
        assert_eq!(scoped.scope.namespace(), Some("MyNs"));
    }
}
#[cfg(test)]
mod attribute_ext_tests {
    use super::*;
    use crate::attribute::*;
    #[test]
    fn test_attr_pipeline_stage_display() {
        assert_eq!(format!("{}", AttrPipelineStage::Applied), "Applied");
        assert_eq!(format!("{}", AttrPipelineStage::Done), "Done");
    }
    #[test]
    fn test_attr_pipeline_stage_order() {
        assert!(AttrPipelineStage::Parsed < AttrPipelineStage::Validated);
        assert!(AttrPipelineStage::Applied < AttrPipelineStage::Done);
    }
    #[test]
    fn test_attr_pipeline_advance_to_done() {
        let entry = AttrEntry::new(AttributeKind::Simp, Name::str("f"));
        let mut p = AttrPipeline::new(entry);
        for _ in 0..4 {
            p.advance();
        }
        assert!(p.is_done());
        assert!(p.is_success());
    }
    #[test]
    fn test_attr_pipeline_fail() {
        let entry = AttrEntry::new(AttributeKind::Simp, Name::str("f"));
        let mut p = AttrPipeline::new(entry);
        p.advance();
        p.fail(AttrError::UnknownAttribute("bad".to_string()));
        assert!(p.is_done());
        assert!(!p.is_success());
        assert!(p.error.is_some());
    }
    #[test]
    fn test_attr_pipeline_advance_idempotent_at_done() {
        let entry = AttrEntry::new(AttributeKind::Instance, Name::str("g"));
        let mut p = AttrPipeline::new(entry);
        for _ in 0..10 {
            p.advance();
        }
        assert!(p.is_done());
    }
    #[test]
    fn test_derive_handler_new() {
        let h = DeriveHandler::new(Name::str("Repr"), "derive Repr");
        assert!(h.generates_instance);
        assert!(!h.requires_dec_eq);
    }
    #[test]
    fn test_derive_handler_with_dec_eq() {
        let h = DeriveHandler::new(Name::str("DecidableEq"), "decidable eq").with_dec_eq();
        assert!(h.requires_dec_eq);
    }
    #[test]
    fn test_derive_handler_no_instance() {
        let h = DeriveHandler::new(Name::str("ToJson"), "json gen").no_instance();
        assert!(!h.generates_instance);
    }
    #[test]
    fn test_derive_handler_registry_register_get() {
        let mut reg = DeriveHandlerRegistry::new();
        let h = DeriveHandler::new(Name::str("Repr"), "repr");
        reg.register(h);
        assert!(reg.has(&Name::str("Repr")));
        assert!(reg.get(&Name::str("Repr")).is_some());
        assert!(!reg.has(&Name::str("Missing")));
    }
    #[test]
    fn test_derive_handler_registry_class_names() {
        let mut reg = DeriveHandlerRegistry::new();
        reg.register(DeriveHandler::new(Name::str("A"), "a"));
        reg.register(DeriveHandler::new(Name::str("B"), "b"));
        assert_eq!(reg.len(), 2);
        let names: Vec<_> = reg.class_names().collect();
        assert_eq!(names.len(), 2);
    }
    #[test]
    fn test_derive_handler_registry_empty() {
        let reg = DeriveHandlerRegistry::new();
        assert!(reg.is_empty());
    }
    #[test]
    fn test_macro_attr_no_args() {
        let a = MacroAttr::new(Name::str("simp"), "", (0, 5));
        assert!(a.has_no_args());
    }
    #[test]
    fn test_macro_attr_with_args() {
        let a = MacroAttr::new(Name::str("ext"), "Nat Int", (10, 20));
        assert!(!a.has_no_args());
    }
    #[test]
    fn test_macro_attr_expansion_success() {
        let attr = MacroAttr::new(Name::str("simp"), "", (0, 5));
        let exp = MacroAttrExpansion::success(attr, vec![], vec![]);
        assert!(exp.success);
        assert!(exp.error.is_none());
    }
    #[test]
    fn test_macro_attr_expansion_failure() {
        let attr = MacroAttr::new(Name::str("bad"), "", (0, 3));
        let exp = MacroAttrExpansion::failure(attr, "unsupported macro");
        assert!(!exp.success);
        assert!(exp.error.is_some());
    }
}
