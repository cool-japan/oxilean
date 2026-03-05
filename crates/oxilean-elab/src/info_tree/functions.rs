//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Level, Name};

use super::types::{
    BinderKind, CompletionItem, CompletionKind, DefinitionCollector, FindReferencesIndex, GoalInfo,
    GotoDefEntry, GotoDefIndex, HoleCollector, HoverInfo, Info, InfoData, InfoTree,
    InfoTreeBuilder, LocalContextEntry, NameReference, NameUsageCollector, SemanticToken,
    SemanticTokenKind, SemanticTokenList, TacticStateInfo,
};

/// Find all info data at a given cursor position.
///
/// Returns info from the most specific (innermost) node first,
/// then progressively more general (outer) nodes.
pub fn query_at(tree: &InfoTree, pos: usize) -> Vec<InfoData> {
    let mut results = Vec::new();
    query_at_inner(tree, pos, &mut results);
    results.sort_by(|a, b| {
        let a_size = info_data_range_hint(a);
        let b_size = info_data_range_hint(b);
        a_size.cmp(&b_size)
    });
    results
}
fn query_at_inner(tree: &InfoTree, pos: usize, results: &mut Vec<InfoData>) {
    match tree {
        InfoTree::Node { info, children } => {
            if info.contains_pos(pos) {
                results.push(info.data.clone());
            }
            for child in children {
                if child.contains_pos(pos) {
                    query_at_inner(child, pos, results);
                }
            }
        }
        InfoTree::Context { children, .. } => {
            for child in children {
                if child.contains_pos(pos) {
                    query_at_inner(child, pos, results);
                }
            }
        }
        InfoTree::Hole { range, .. } => {
            if let Some((start, end)) = range {
                if pos >= *start && pos < *end {}
            }
        }
    }
}
/// Heuristic for sorting info data by specificity.
fn info_data_range_hint(data: &InfoData) -> usize {
    match data {
        InfoData::TermInfo { .. } => 0,
        InfoData::FieldInfo { .. } => 1,
        InfoData::RefInfo { .. } => 2,
        InfoData::BinderInfo { .. } => 3,
        InfoData::TacticInfo { .. } => 4,
        InfoData::CommandInfo { .. } => 5,
        InfoData::MacroExpansion { .. } => 6,
        InfoData::CompletionInfo { .. } => 7,
        InfoData::LevelInfo { .. } => 8,
    }
}
/// Find the definition of the name under the cursor.
///
/// Returns the fully qualified name if the cursor is on a reference
/// to a definition.
pub fn find_definition(tree: &InfoTree, pos: usize) -> Option<Name> {
    let infos = query_at(tree, pos);
    for info in &infos {
        match info {
            InfoData::RefInfo { name, .. } => return Some(name.clone()),
            InfoData::FieldInfo {
                struct_name,
                field_name,
                ..
            } => {
                return Some(struct_name.clone().append_str(format!("{}", field_name)));
            }
            InfoData::TermInfo { expr, .. } => {
                if let Some(name) = extract_const_name(expr) {
                    return Some(name);
                }
            }
            _ => {}
        }
    }
    None
}
/// Extract a constant name from an expression.
fn extract_const_name(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(func, _) => extract_const_name(func),
        _ => None,
    }
}
/// Find all references to a name in the info tree.
///
/// Returns a list of source ranges `(start, end)` where the name appears.
pub fn find_references(tree: &InfoTree, name: &Name) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    find_references_inner(tree, name, &mut results);
    results.sort_by_key(|&(start, _)| start);
    results.dedup();
    results
}
fn find_references_inner(tree: &InfoTree, name: &Name, results: &mut Vec<(usize, usize)>) {
    match tree {
        InfoTree::Node { info, children } => {
            if references_name(&info.data, name) {
                results.push(info.stx_range);
            }
            for child in children {
                find_references_inner(child, name, results);
            }
        }
        InfoTree::Context { children, .. } => {
            for child in children {
                find_references_inner(child, name, results);
            }
        }
        InfoTree::Hole { .. } => {}
    }
}
/// Check if an info data entry references the given name.
pub(super) fn references_name(data: &InfoData, name: &Name) -> bool {
    match data {
        InfoData::RefInfo { name: ref_name, .. } => ref_name == name,
        InfoData::TermInfo { expr, .. } => expr_references_name(expr, name),
        InfoData::FieldInfo {
            struct_name,
            field_name,
            ..
        } => struct_name == name || field_name == name,
        InfoData::CommandInfo {
            decl_name: Some(decl_name),
            ..
        } => decl_name == name,
        InfoData::BinderInfo {
            name: binder_name, ..
        } => binder_name == name,
        _ => false,
    }
}
/// Check if an expression references a name.
pub(super) fn expr_references_name(expr: &Expr, name: &Name) -> bool {
    match expr {
        Expr::Const(n, _) => n == name,
        Expr::App(f, a) => expr_references_name(f, name) || expr_references_name(a, name),
        Expr::Lam(_, _, ty, body) => {
            expr_references_name(ty, name) || expr_references_name(body, name)
        }
        Expr::Pi(_, _, ty, body) => {
            expr_references_name(ty, name) || expr_references_name(body, name)
        }
        Expr::Let(_, ty, val, body) => {
            expr_references_name(ty, name)
                || expr_references_name(val, name)
                || expr_references_name(body, name)
        }
        Expr::Proj(_, _, base) => expr_references_name(base, name),
        _ => false,
    }
}
/// Get hover information at a cursor position.
///
/// Combines type information, documentation, and local context
/// into a single `HoverInfo` structure.
pub fn hover_info(tree: &InfoTree, pos: usize) -> Option<HoverInfo> {
    let infos = query_at(tree, pos);
    if infos.is_empty() {
        return None;
    }
    let range = find_range_at(tree, pos).unwrap_or((pos, pos + 1));
    let mut hover = HoverInfo::new(range);
    let lctx = collect_local_context(tree, pos);
    hover = hover.with_local_context(lctx);
    for info in &infos {
        match info {
            InfoData::TermInfo { expr, type_ } => {
                if hover.expr.is_none() {
                    hover = hover.with_expr(expr.clone()).with_type(type_.clone());
                }
            }
            InfoData::RefInfo { name, ty, doc } => {
                hover = hover.with_name(name.clone());
                if let Some(ty) = ty {
                    hover = hover.with_type(ty.clone());
                }
                if let Some(doc) = doc {
                    hover = hover.with_doc(doc.clone());
                }
            }
            InfoData::FieldInfo {
                struct_name,
                field_name,
                field_type,
            } => {
                let full_name = struct_name.clone().append_str(format!("{}", field_name));
                hover = hover.with_name(full_name);
                if let Some(ty) = field_type {
                    hover = hover.with_type(ty.clone());
                }
            }
            InfoData::TacticInfo {
                state_before: _,
                state_after,
            } => {
                hover = hover.with_tactic_state(state_after.clone());
            }
            InfoData::BinderInfo { name, ty, .. } => {
                hover = hover.with_name(name.clone()).with_type(ty.clone());
            }
            _ => {}
        }
    }
    if hover.has_content() {
        Some(hover)
    } else {
        None
    }
}
/// Find the source range of the smallest node containing a position.
fn find_range_at(tree: &InfoTree, pos: usize) -> Option<(usize, usize)> {
    match tree {
        InfoTree::Node { info, children } => {
            if !info.contains_pos(pos) {
                return None;
            }
            for child in children {
                if let Some(range) = find_range_at(child, pos) {
                    return Some(range);
                }
            }
            Some(info.stx_range)
        }
        InfoTree::Context {
            children, range, ..
        } => {
            if let Some((start, end)) = range {
                if pos < *start || pos >= *end {
                    return None;
                }
            }
            for child in children {
                if let Some(range) = find_range_at(child, pos) {
                    return Some(range);
                }
            }
            *range
        }
        InfoTree::Hole { range, .. } => {
            if let Some((start, end)) = range {
                if pos >= *start && pos < *end {
                    return Some((*start, *end));
                }
            }
            None
        }
    }
}
/// Collect the local context at a given position.
fn collect_local_context(tree: &InfoTree, pos: usize) -> Vec<LocalContextEntry> {
    let mut ctx = Vec::new();
    collect_local_context_inner(tree, pos, &mut ctx);
    ctx
}
fn collect_local_context_inner(tree: &InfoTree, pos: usize, ctx: &mut Vec<LocalContextEntry>) {
    match tree {
        InfoTree::Context {
            lctx,
            children,
            range,
        } => {
            let in_range = range
                .map(|(start, end)| pos >= start && pos < end)
                .unwrap_or(true);
            if in_range {
                ctx.extend(lctx.iter().cloned());
                for child in children {
                    collect_local_context_inner(child, pos, ctx);
                }
            }
        }
        InfoTree::Node { info, children } => {
            if info.contains_pos(pos) {
                for child in children {
                    collect_local_context_inner(child, pos, ctx);
                }
            }
        }
        InfoTree::Hole { .. } => {}
    }
}
/// Trait for walking an info tree, visiting each node.
///
/// Implement this trait to perform custom traversals of the info tree,
/// such as collecting all definitions, finding specific patterns, etc.
pub trait InfoTreeWalker {
    /// Visit a node in the info tree.
    ///
    /// Return `true` to continue visiting children, `false` to skip.
    fn visit_node(&mut self, info: &Info, depth: usize) -> bool;
    /// Visit a hole in the info tree.
    fn visit_hole(&mut self, expected_type: &Option<Expr>, range: &Option<(usize, usize)>);
    /// Enter a context scope.
    fn enter_context(&mut self, lctx: &[LocalContextEntry]);
    /// Leave a context scope.
    fn leave_context(&mut self);
}
/// Walk an info tree using the given walker.
pub fn walk_info_tree(tree: &InfoTree, walker: &mut dyn InfoTreeWalker) {
    walk_info_tree_inner(tree, walker, 0);
}
fn walk_info_tree_inner(tree: &InfoTree, walker: &mut dyn InfoTreeWalker, depth: usize) {
    match tree {
        InfoTree::Node { info, children } => {
            if walker.visit_node(info, depth) {
                for child in children {
                    walk_info_tree_inner(child, walker, depth + 1);
                }
            }
        }
        InfoTree::Hole {
            expected_type,
            range,
        } => {
            walker.visit_hole(expected_type, range);
        }
        InfoTree::Context { lctx, children, .. } => {
            walker.enter_context(lctx);
            for child in children {
                walk_info_tree_inner(child, walker, depth + 1);
            }
            walker.leave_context();
        }
    }
}
/// Serialize an info tree to a JSON-like string format.
///
/// Produces a human-readable representation of the tree structure,
/// useful for debugging and testing.
pub fn serialize_info_tree(tree: &InfoTree) -> String {
    let mut buf = String::new();
    serialize_inner(tree, &mut buf, 0);
    buf
}
fn serialize_inner(tree: &InfoTree, buf: &mut String, indent: usize) {
    let pad = " ".repeat(indent * 2);
    match tree {
        InfoTree::Node { info, children } => {
            buf.push_str(&format!(
                "{}{{\"type\":\"node\",\"range\":[{},{}],\"data\":\"{}\",\"children\":[",
                pad, info.stx_range.0, info.stx_range.1, info.data
            ));
            if children.is_empty() {
                buf.push_str("]}");
            } else {
                buf.push('\n');
                for (i, child) in children.iter().enumerate() {
                    serialize_inner(child, buf, indent + 1);
                    if i + 1 < children.len() {
                        buf.push(',');
                    }
                    buf.push('\n');
                }
                buf.push_str(&format!("{}]}}", pad));
            }
        }
        InfoTree::Hole {
            expected_type,
            range,
        } => {
            buf.push_str(&format!(
                "{}{{\"type\":\"hole\",\"range\":{:?},\"expected_type\":{:?}}}",
                pad, range, expected_type
            ));
        }
        InfoTree::Context {
            lctx,
            children,
            range,
        } => {
            buf.push_str(&format!(
                "{}{{\"type\":\"context\",\"range\":{:?},\"lctx_count\":{},\"children\":[",
                pad,
                range,
                lctx.len()
            ));
            if children.is_empty() {
                buf.push_str("]}");
            } else {
                buf.push('\n');
                for (i, child) in children.iter().enumerate() {
                    serialize_inner(child, buf, indent + 1);
                    if i + 1 < children.len() {
                        buf.push(',');
                    }
                    buf.push('\n');
                }
                buf.push_str(&format!("{}]}}", pad));
            }
        }
    }
}
/// A hole entry: expected type and optional source range.
pub type HoleEntry = (Option<Expr>, Option<(usize, usize)>);
#[cfg(test)]
mod tests {
    use super::*;
    use crate::info_tree::*;
    use oxilean_kernel::{Expr, Literal, Name};
    fn mk_nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn mk_term_info(start: usize, end: usize) -> Info {
        Info::new(
            start,
            end,
            InfoData::TermInfo {
                expr: Expr::Const(Name::str("x"), vec![]),
                type_: mk_nat(),
            },
        )
    }
    fn mk_ref_info(start: usize, end: usize, name: &str) -> Info {
        Info::new(
            start,
            end,
            InfoData::RefInfo {
                name: Name::str(name),
                ty: Some(mk_nat()),
                doc: None,
            },
        )
    }
    #[test]
    fn test_info_tree_leaf() {
        let info = mk_term_info(0, 5);
        let tree = InfoTree::leaf(info);
        assert_eq!(tree.node_count(), 1);
        assert_eq!(tree.depth(), 1);
        assert!(tree.contains_pos(3));
        assert!(!tree.contains_pos(6));
    }
    #[test]
    fn test_info_tree_node_with_children() {
        let parent = mk_term_info(0, 20);
        let child1 = InfoTree::leaf(mk_term_info(0, 5));
        let child2 = InfoTree::leaf(mk_term_info(6, 10));
        let child3 = InfoTree::leaf(mk_term_info(11, 20));
        let tree = InfoTree::node(parent, vec![child1, child2, child3]);
        assert_eq!(tree.node_count(), 4);
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.children().len(), 3);
    }
    #[test]
    fn test_info_tree_hole() {
        let tree = InfoTree::hole(Some(mk_nat()), Some((5, 6)));
        assert_eq!(tree.node_count(), 1);
        assert!(tree.contains_pos(5));
        assert!(!tree.contains_pos(6));
    }
    #[test]
    fn test_info_tree_context() {
        let entry = LocalContextEntry::new(Name::str("h"), mk_nat());
        let child = InfoTree::leaf(mk_term_info(5, 10));
        let tree = InfoTree::context(vec![entry], vec![child], Some((0, 20)));
        assert_eq!(tree.node_count(), 2);
    }
    #[test]
    fn test_info_contains_pos() {
        let info = Info::new(
            10,
            20,
            InfoData::TermInfo {
                expr: mk_nat(),
                type_: mk_nat(),
            },
        );
        assert!(info.contains_pos(10));
        assert!(info.contains_pos(15));
        assert!(!info.contains_pos(20));
        assert!(!info.contains_pos(9));
        assert_eq!(info.range_len(), 10);
    }
    #[test]
    fn test_info_display() {
        let info = mk_term_info(5, 15);
        let display = format!("{}", info);
        assert!(display.contains("5..15"));
        assert!(display.contains("term"));
    }
    #[test]
    fn test_info_data_display() {
        let data = InfoData::TermInfo {
            expr: mk_nat(),
            type_: mk_nat(),
        };
        assert!(format!("{}", data).contains("term"));
        let data = InfoData::CommandInfo {
            name: "def".to_string(),
            decl_name: Some(Name::str("foo")),
        };
        assert!(format!("{}", data).contains("def"));
        let data = InfoData::FieldInfo {
            struct_name: Name::str("Foo"),
            field_name: Name::str("bar"),
            field_type: None,
        };
        assert!(format!("{}", data).contains("Foo"));
    }
    #[test]
    fn test_local_context_entry() {
        let entry = LocalContextEntry::new(Name::str("h"), mk_nat());
        assert!(entry.is_user_name);
        assert!(entry.val.is_none());
        assert!(format!("{}", entry).contains("h"));
        let let_entry =
            LocalContextEntry::let_bound(Name::str("x"), mk_nat(), Expr::Lit(Literal::Nat(42)));
        assert!(let_entry.val.is_some());
        let auto_entry = LocalContextEntry::new(Name::str("_"), mk_nat()).with_auto();
        assert!(!auto_entry.is_user_name);
    }
    #[test]
    fn test_tactic_state_info() {
        let state = TacticStateInfo::new();
        assert!(state.is_solved());
        assert_eq!(state.num_goals(), 0);
        assert!(state.focused_goal().is_none());
        let goal = GoalInfo::new(mk_nat()).with_hypothesis(Name::str("h"), mk_nat());
        let state = TacticStateInfo::with_single_goal(goal);
        assert!(!state.is_solved());
        assert_eq!(state.num_goals(), 1);
        assert!(state.focused_goal().is_some());
        let display = format!("{}", state);
        assert!(display.contains("1 goal"));
    }
    #[test]
    fn test_goal_info() {
        let goal = GoalInfo::named(Name::str("main"), mk_nat())
            .with_hypothesis(Name::str("h1"), mk_nat())
            .with_hypotheses(vec![(Name::str("h2"), mk_nat())]);
        assert_eq!(goal.name, Some(Name::str("main")));
        assert_eq!(goal.hypotheses.len(), 2);
        assert!(format!("{}", goal).contains("main"));
    }
    #[test]
    fn test_completion_item() {
        let item = CompletionItem::new(
            "Nat.add".to_string(),
            "Nat.add".to_string(),
            CompletionKind::Function,
        )
        .with_doc("Addition on natural numbers".to_string())
        .with_type("Nat -> Nat -> Nat".to_string())
        .with_priority(50);
        assert_eq!(item.kind, CompletionKind::Function);
        assert_eq!(item.sort_priority, 50);
        assert!(item.documentation.is_some());
        assert!(item.type_signature.is_some());
    }
    #[test]
    fn test_completion_kind_display() {
        assert_eq!(format!("{}", CompletionKind::Function), "function");
        assert_eq!(format!("{}", CompletionKind::Tactic), "tactic");
        assert_eq!(format!("{}", CompletionKind::Constructor), "constructor");
    }
    #[test]
    fn test_hover_info() {
        let hover = HoverInfo::new((10, 20))
            .with_expr(mk_nat())
            .with_type(mk_nat())
            .with_name(Name::str("Nat"))
            .with_doc("Natural numbers".to_string());
        assert!(hover.has_content());
        let md = hover.to_markdown();
        assert!(md.contains("Nat"));
        assert!(md.contains("Natural numbers"));
    }
    #[test]
    fn test_hover_info_empty() {
        let hover = HoverInfo::new((0, 0));
        assert!(!hover.has_content());
    }
    #[test]
    fn test_hover_info_with_tactic_state() {
        let goal = GoalInfo::new(mk_nat());
        let state = TacticStateInfo::with_single_goal(goal);
        let hover = HoverInfo::new((0, 10)).with_tactic_state(state);
        assert!(hover.has_content());
        let md = hover.to_markdown();
        assert!(md.contains("Tactic state"));
    }
    #[test]
    fn test_builder_basic() {
        let mut builder = InfoTreeBuilder::new();
        assert!(builder.is_enabled());
        assert_eq!(builder.stack_depth(), 0);
        builder.push_term_info(0, 10, mk_nat(), mk_nat());
        assert_eq!(builder.stack_depth(), 1);
        let tree = builder.pop();
        assert!(tree.is_some());
        assert_eq!(builder.stack_depth(), 0);
        assert!(builder.has_roots());
    }
    #[test]
    fn test_builder_nested() {
        let mut builder = InfoTreeBuilder::new();
        builder.push_term_info(0, 20, mk_nat(), mk_nat());
        builder.push_term_info(0, 5, mk_nat(), mk_nat());
        builder.pop();
        builder.push_term_info(6, 10, mk_nat(), mk_nat());
        builder.pop();
        let tree = builder.pop();
        assert!(tree.is_some());
        let tree = tree.expect("test operation should succeed");
        assert_eq!(tree.children().len(), 2);
    }
    #[test]
    fn test_builder_disabled() {
        let mut builder = InfoTreeBuilder::disabled();
        assert!(!builder.is_enabled());
        builder.push_term_info(0, 10, mk_nat(), mk_nat());
        assert_eq!(builder.stack_depth(), 0);
        assert!(!builder.has_roots());
    }
    #[test]
    fn test_builder_finish() {
        let mut builder = InfoTreeBuilder::new();
        builder.push_term_info(0, 20, mk_nat(), mk_nat());
        builder.push_term_info(0, 5, mk_nat(), mk_nat());
        let roots = builder.finish();
        assert!(!roots.is_empty());
    }
    #[test]
    fn test_builder_add_leaf() {
        let mut builder = InfoTreeBuilder::new();
        builder.push_term_info(0, 20, mk_nat(), mk_nat());
        builder.add_leaf(
            5,
            10,
            InfoData::TermInfo {
                expr: mk_nat(),
                type_: mk_nat(),
            },
        );
        let tree = builder.pop();
        assert!(tree.is_some());
        let tree = tree.expect("test operation should succeed");
        assert_eq!(tree.children().len(), 1);
    }
    #[test]
    fn test_builder_add_hole() {
        let mut builder = InfoTreeBuilder::new();
        builder.push_term_info(0, 20, mk_nat(), mk_nat());
        builder.add_hole(Some(mk_nat()), Some((5, 6)));
        let tree = builder.pop();
        assert!(tree.is_some());
    }
    #[test]
    fn test_builder_context() {
        let mut builder = InfoTreeBuilder::new();
        let entry = LocalContextEntry::new(Name::str("h"), mk_nat());
        builder.push_context(Some((0, 20)), vec![entry]);
        builder.add_leaf(
            5,
            10,
            InfoData::TermInfo {
                expr: mk_nat(),
                type_: mk_nat(),
            },
        );
        let tree = builder.pop();
        assert!(tree.is_some());
        let tree = tree.expect("test operation should succeed");
        assert!(matches!(tree, InfoTree::Context { .. }));
    }
    #[test]
    fn test_builder_doc_strings() {
        let mut builder = InfoTreeBuilder::new();
        builder.register_doc(Name::str("Nat.add"), "Addition".to_string());
        assert_eq!(
            builder.get_doc(&Name::str("Nat.add")),
            Some(&"Addition".to_string())
        );
        assert!(builder.get_doc(&Name::str("unknown")).is_none());
    }
    #[test]
    fn test_query_at_simple() {
        let tree = InfoTree::leaf(mk_term_info(0, 10));
        let results = query_at(&tree, 5);
        assert_eq!(results.len(), 1);
        assert!(matches!(results[0], InfoData::TermInfo { .. }));
    }
    #[test]
    fn test_query_at_nested() {
        let parent = mk_term_info(0, 20);
        let child = InfoTree::leaf(mk_term_info(5, 10));
        let tree = InfoTree::node(parent, vec![child]);
        let results = query_at(&tree, 7);
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_query_at_miss() {
        let tree = InfoTree::leaf(mk_term_info(0, 10));
        let results = query_at(&tree, 15);
        assert!(results.is_empty());
    }
    #[test]
    fn test_find_definition_ref() {
        let tree = InfoTree::leaf(mk_ref_info(0, 10, "Nat.add"));
        let result = find_definition(&tree, 5);
        assert_eq!(result, Some(Name::str("Nat.add")));
    }
    #[test]
    fn test_find_definition_miss() {
        let tree = InfoTree::leaf(mk_term_info(0, 10));
        let result = find_definition(&tree, 15);
        assert!(result.is_none());
    }
    #[test]
    fn test_find_definition_field() {
        let info = Info::new(
            0,
            10,
            InfoData::FieldInfo {
                struct_name: Name::str("Foo"),
                field_name: Name::str("bar"),
                field_type: None,
            },
        );
        let tree = InfoTree::leaf(info);
        let result = find_definition(&tree, 5);
        assert!(result.is_some());
    }
    #[test]
    fn test_find_references_simple() {
        let ref1 = InfoTree::leaf(mk_ref_info(0, 5, "Nat.add"));
        let ref2 = InfoTree::leaf(mk_ref_info(10, 15, "Nat.add"));
        let ref3 = InfoTree::leaf(mk_ref_info(20, 25, "Nat.mul"));
        let parent = mk_term_info(0, 30);
        let tree = InfoTree::node(parent, vec![ref1, ref2, ref3]);
        let refs = find_references(&tree, &Name::str("Nat.add"));
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0], (0, 5));
        assert_eq!(refs[1], (10, 15));
    }
    #[test]
    fn test_find_references_none() {
        let tree = InfoTree::leaf(mk_ref_info(0, 5, "Nat.add"));
        let refs = find_references(&tree, &Name::str("unknown"));
        assert!(refs.is_empty());
    }
    #[test]
    fn test_hover_info_term() {
        let tree = InfoTree::leaf(mk_term_info(0, 10));
        let hover = hover_info(&tree, 5);
        assert!(hover.is_some());
        let hover = hover.expect("hover info should be present");
        assert!(hover.expr.is_some());
        assert!(hover.ty.is_some());
    }
    #[test]
    fn test_hover_info_ref_with_doc() {
        let info = Info::new(
            0,
            10,
            InfoData::RefInfo {
                name: Name::str("Nat.add"),
                ty: Some(mk_nat()),
                doc: Some("Addition of natural numbers".to_string()),
            },
        );
        let tree = InfoTree::leaf(info);
        let hover = hover_info(&tree, 5);
        assert!(hover.is_some());
        let hover = hover.expect("hover info should be present");
        assert_eq!(hover.name, Some(Name::str("Nat.add")));
        assert!(hover.doc.is_some());
    }
    #[test]
    fn test_hover_info_miss() {
        let tree = InfoTree::leaf(mk_term_info(0, 10));
        let hover = hover_info(&tree, 15);
        assert!(hover.is_none());
    }
    #[test]
    fn test_serialize_leaf() {
        let tree = InfoTree::leaf(mk_term_info(0, 10));
        let json = serialize_info_tree(&tree);
        assert!(json.contains("node"));
        assert!(json.contains("0"));
        assert!(json.contains("10"));
    }
    #[test]
    fn test_serialize_nested() {
        let child = InfoTree::leaf(mk_term_info(0, 5));
        let parent = mk_term_info(0, 20);
        let tree = InfoTree::node(parent, vec![child]);
        let json = serialize_info_tree(&tree);
        assert!(json.contains("children"));
    }
    #[test]
    fn test_serialize_hole() {
        let tree = InfoTree::hole(Some(mk_nat()), Some((5, 6)));
        let json = serialize_info_tree(&tree);
        assert!(json.contains("hole"));
    }
    #[test]
    fn test_serialize_context() {
        let entry = LocalContextEntry::new(Name::str("h"), mk_nat());
        let tree = InfoTree::context(vec![entry], vec![], Some((0, 20)));
        let json = serialize_info_tree(&tree);
        assert!(json.contains("context"));
        assert!(json.contains("lctx_count"));
    }
    #[test]
    fn test_definition_collector() {
        let info = Info::new(
            0,
            10,
            InfoData::CommandInfo {
                name: "def".to_string(),
                decl_name: Some(Name::str("myFunc")),
            },
        );
        let tree = InfoTree::leaf(info);
        let defs = DefinitionCollector::collect(&tree);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].0, Name::str("myFunc"));
    }
    #[test]
    fn test_hole_collector() {
        let child1 = InfoTree::hole(Some(mk_nat()), Some((0, 1)));
        let child2 = InfoTree::hole(None, Some((5, 6)));
        let parent = mk_term_info(0, 20);
        let tree = InfoTree::node(parent, vec![child1, child2]);
        let holes = HoleCollector::collect(&tree);
        assert_eq!(holes.len(), 2);
    }
    #[test]
    fn test_name_usage_collector() {
        let ref1 = InfoTree::leaf(mk_ref_info(0, 5, "Nat.add"));
        let ref2 = InfoTree::leaf(mk_ref_info(10, 15, "Nat.add"));
        let parent = mk_term_info(0, 20);
        let tree = InfoTree::node(parent, vec![ref1, ref2]);
        let usages = NameUsageCollector::collect(&tree, &Name::str("Nat.add"));
        assert_eq!(usages.len(), 2);
    }
    #[test]
    fn test_binder_kind_display() {
        assert_eq!(format!("{}", BinderKind::Explicit), "explicit");
        assert_eq!(format!("{}", BinderKind::Implicit), "implicit");
        assert_eq!(format!("{}", BinderKind::Instance), "instance");
        assert_eq!(format!("{}", BinderKind::StrictImplicit), "strict implicit");
    }
    #[test]
    fn test_empty_tree_query() {
        let tree = InfoTree::context(vec![], vec![], None);
        let results = query_at(&tree, 0);
        assert!(results.is_empty());
    }
    #[test]
    fn test_deeply_nested_tree() {
        let mut tree = InfoTree::leaf(mk_term_info(5, 6));
        for i in 0..10 {
            tree = InfoTree::node(mk_term_info(0, 20 + i), vec![tree]);
        }
        assert!(tree.depth() > 10);
        let results = query_at(&tree, 5);
        assert!(results.len() > 1);
    }
    #[test]
    fn test_hover_with_local_context() {
        let entry = LocalContextEntry::new(Name::str("h"), mk_nat());
        let child = InfoTree::leaf(mk_term_info(5, 10));
        let tree = InfoTree::context(vec![entry], vec![child], Some((0, 20)));
        let hover = hover_info(&tree, 7);
        assert!(hover.is_some());
        let hover = hover.expect("hover info should be present");
        assert!(!hover.local_context.is_empty());
    }
    #[test]
    fn test_builder_current_context() {
        let mut builder = InfoTreeBuilder::new();
        assert!(builder.current_context().is_empty());
        let entry = LocalContextEntry::new(Name::str("h"), mk_nat());
        builder.push_context(Some((0, 20)), vec![entry.clone()]);
        assert_eq!(builder.current_context().len(), 1);
        builder.pop();
        assert!(builder.current_context().is_empty());
    }
    #[test]
    fn test_hover_info_markdown() {
        let hover = HoverInfo::new((0, 10))
            .with_name(Name::str("Nat.add"))
            .with_type(mk_nat())
            .with_doc("Adds two naturals".to_string())
            .with_local_context(vec![LocalContextEntry::new(Name::str("n"), mk_nat())]);
        let md = hover.to_markdown();
        assert!(md.contains("Nat.add"));
        assert!(md.contains("Adds two naturals"));
        assert!(md.contains("Context"));
    }
}
#[cfg(test)]
mod info_tree_ext_tests {
    use super::*;
    use crate::info_tree::*;
    fn mk_nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_semantic_token_kind_display() {
        assert_eq!(format!("{}", SemanticTokenKind::Keyword), "keyword");
        assert_eq!(format!("{}", SemanticTokenKind::Function), "function");
    }
    #[test]
    fn test_semantic_token_len() {
        let t = SemanticToken::new((5, 10), SemanticTokenKind::Variable);
        assert_eq!(t.len(), 5);
    }
    #[test]
    fn test_semantic_token_modifier() {
        let t = SemanticToken::new((0, 4), SemanticTokenKind::Keyword).with_modifier(1);
        assert_eq!(t.modifiers, 1);
    }
    #[test]
    fn test_semantic_token_list_push_sort() {
        let mut list = SemanticTokenList::new();
        list.push(SemanticToken::new((10, 15), SemanticTokenKind::Type));
        list.push(SemanticToken::new((2, 5), SemanticTokenKind::Variable));
        list.sort();
        let first = list
            .iter()
            .next()
            .expect("iterator should have next element");
        assert_eq!(first.range.0, 2);
    }
    #[test]
    fn test_semantic_token_list_of_kind() {
        let mut list = SemanticTokenList::new();
        list.push(SemanticToken::new((0, 3), SemanticTokenKind::Keyword));
        list.push(SemanticToken::new((5, 8), SemanticTokenKind::Type));
        list.push(SemanticToken::new((10, 13), SemanticTokenKind::Keyword));
        let keywords = list.of_kind(SemanticTokenKind::Keyword);
        assert_eq!(keywords.len(), 2);
    }
    #[test]
    fn test_semantic_token_list_empty() {
        let list = SemanticTokenList::new();
        assert!(list.is_empty());
    }
    #[test]
    fn test_goto_def_entry_local() {
        let e = GotoDefEntry::in_file((0, 5), (100, 110), Name::str("f"));
        assert!(e.is_local());
    }
    #[test]
    fn test_goto_def_entry_cross_file() {
        let e = GotoDefEntry::cross_file((0, 5), "Nat.lean", (10, 20), Name::str("Nat"));
        assert!(!e.is_local());
        assert_eq!(e.target_file, "Nat.lean");
    }
    #[test]
    fn test_goto_def_index_lookup_pos() {
        let mut idx = GotoDefIndex::new();
        idx.insert(GotoDefEntry::in_file((5, 10), (100, 105), Name::str("f")));
        idx.insert(GotoDefEntry::in_file((20, 25), (200, 205), Name::str("g")));
        let results = idx.lookup_pos(7);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, Name::str("f"));
        let empty = idx.lookup_pos(15);
        assert!(empty.is_empty());
    }
    #[test]
    fn test_goto_def_index_len() {
        let mut idx = GotoDefIndex::new();
        assert!(idx.is_empty());
        idx.insert(GotoDefEntry::in_file((0, 3), (10, 13), Name::str("x")));
        assert_eq!(idx.len(), 1);
    }
    #[test]
    fn test_name_reference_use_site() {
        let r = NameReference::use_site(Name::str("f"), (5, 10));
        assert!(!r.is_def_site);
    }
    #[test]
    fn test_name_reference_def_site() {
        let r = NameReference::def_site(Name::str("g"), (0, 5));
        assert!(r.is_def_site);
    }
    #[test]
    fn test_find_references_index_find() {
        let mut idx = FindReferencesIndex::new();
        idx.insert(NameReference::use_site(Name::str("f"), (5, 8)));
        idx.insert(NameReference::use_site(Name::str("f"), (20, 23)));
        idx.insert(NameReference::use_site(Name::str("g"), (30, 33)));
        let refs = idx.find(&Name::str("f"));
        assert_eq!(refs.len(), 2);
        let refs_g = idx.find(&Name::str("g"));
        assert_eq!(refs_g.len(), 1);
    }
    #[test]
    fn test_find_references_index_at_pos() {
        let mut idx = FindReferencesIndex::new();
        idx.insert(NameReference::use_site(Name::str("f"), (5, 10)));
        let at = idx.at_pos(7);
        assert_eq!(at.len(), 1);
        let miss = idx.at_pos(15);
        assert!(miss.is_empty());
    }
    #[test]
    fn test_find_references_index_stats() {
        let mut idx = FindReferencesIndex::new();
        idx.insert(NameReference::use_site(Name::str("f"), (0, 3)));
        idx.insert(NameReference::def_site(Name::str("f"), (10, 13)));
        idx.insert(NameReference::use_site(Name::str("g"), (20, 23)));
        assert_eq!(idx.num_names(), 2);
        assert_eq!(idx.total_refs(), 3);
    }
    #[test]
    fn test_find_references_index_empty() {
        let idx = FindReferencesIndex::new();
        assert_eq!(idx.num_names(), 0);
        assert_eq!(idx.total_refs(), 0);
        assert!(idx.find(&Name::str("x")).is_empty());
    }
    #[test]
    fn test_mk_nat_is_const() {
        let n = mk_nat();
        assert!(matches!(n, Expr::Const(_, _)));
    }
}
