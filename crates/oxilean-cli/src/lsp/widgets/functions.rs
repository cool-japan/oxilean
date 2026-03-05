//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{analyze_document, Document, JsonValue, Position, Range, SymbolKind};
use oxilean_kernel::Environment;

use super::types::{
    GoalDisplay, GoalViewerState, Hypothesis, HypothesisDisplay, PanelPosition, ProofGoal,
    ProofNodeKind, ProofNodeStatus, ProofProgress, ProofTreeBuilder, ProofTreeNode,
    ProofTreeNodeV2, SuggestionCategory, TacticSuggestion, TacticSuggestionItem,
    TacticSuggestionState, TacticSuggestor, TheoremProgress, WidgetColor, WidgetDispatcher,
    WidgetEvent, WidgetEventResponse, WidgetFocusTracker, WidgetHistory, WidgetId, WidgetKind,
    WidgetLayout, WidgetManager, WidgetPanel, WidgetPerfStats, WidgetSearchIndex, WidgetSnapshot,
    WidgetTheme,
};

/// Computes proof progress for a document.
pub fn compute_proof_progress(doc: &Document, env: &Environment) -> ProofProgress {
    let mut builder = ProofTreeBuilder::new(env);
    let trees = builder.build_document_tree(doc);
    let total = trees.len();
    let mut proven = 0;
    let mut sorry = 0;
    let mut in_progress = 0;
    let mut theorem_progress = Vec::new();
    for tree in &trees {
        let sorry_count = tree.sorry_count();
        let status = tree.status;
        match status {
            ProofNodeStatus::Proven => proven += 1,
            ProofNodeStatus::Admitted => sorry += 1,
            _ => in_progress += 1,
        }
        theorem_progress.push(TheoremProgress {
            name: tree.label.clone(),
            status,
            sorry_count,
            range: tree.range.clone(),
        });
    }
    let progress_percent = if total > 0 {
        (proven as f64 / total as f64) * 100.0
    } else {
        100.0
    };
    ProofProgress {
        total_theorems: total,
        proven_theorems: proven,
        sorry_theorems: sorry,
        in_progress_theorems: in_progress,
        progress_percent,
        theorem_progress,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn make_doc(content: &str) -> Document {
        Document::new("file:///test.lean", 1, content)
    }
    #[test]
    fn test_widget_id() {
        let id = WidgetId::from_position("file:///a.lean", 5, 10);
        assert!(id.0.contains("file:///a.lean"));
    }
    #[test]
    fn test_widget_manager_create() {
        let mut mgr = WidgetManager::new();
        let id = mgr.create_widget(
            WidgetKind::GoalViewer,
            "file:///a.lean",
            Position::new(0, 0),
        );
        assert_eq!(mgr.widget_count(), 1);
        assert!(mgr.get_widget(&id).is_some());
    }
    #[test]
    fn test_widget_manager_remove() {
        let mut mgr = WidgetManager::new();
        let id = mgr.create_widget(
            WidgetKind::GoalViewer,
            "file:///a.lean",
            Position::new(0, 0),
        );
        assert!(mgr.remove_widget(&id));
        assert_eq!(mgr.widget_count(), 0);
    }
    #[test]
    fn test_widget_manager_toggle_visibility() {
        let mut mgr = WidgetManager::new();
        let id = mgr.create_widget(
            WidgetKind::GoalViewer,
            "file:///a.lean",
            Position::new(0, 0),
        );
        assert!(
            mgr.get_widget(&id)
                .expect("test operation should succeed")
                .visible
        );
        mgr.toggle_visibility(&id);
        assert!(
            !mgr.get_widget(&id)
                .expect("test operation should succeed")
                .visible
        );
    }
    #[test]
    fn test_widget_manager_widgets_for_uri() {
        let mut mgr = WidgetManager::new();
        mgr.create_widget(
            WidgetKind::GoalViewer,
            "file:///a.lean",
            Position::new(0, 0),
        );
        mgr.create_widget(WidgetKind::ProofTree, "file:///a.lean", Position::new(1, 0));
        mgr.create_widget(
            WidgetKind::GoalViewer,
            "file:///b.lean",
            Position::new(0, 0),
        );
        assert_eq!(mgr.widgets_for_uri("file:///a.lean").len(), 2);
        assert_eq!(mgr.widgets_for_uri("file:///b.lean").len(), 1);
    }
    #[test]
    fn test_proof_goal_to_json() {
        let goal = ProofGoal {
            index: 0,
            hypotheses: vec![Hypothesis {
                name: "h".to_string(),
                ty: "P".to_string(),
                is_new: true,
                is_used: false,
            }],
            target: "Q".to_string(),
            tag: Some("test_thm".to_string()),
            is_focused: true,
        };
        let json = goal.to_json();
        assert!(json.get("target").is_some());
    }
    #[test]
    fn test_proof_tree_node_count() {
        let node = ProofTreeNode {
            id: "1".to_string(),
            label: "root".to_string(),
            kind: ProofNodeKind::Theorem,
            status: ProofNodeStatus::InProgress,
            range: Range::single_line(0, 0, 10),
            children: vec![ProofTreeNode {
                id: "2".to_string(),
                label: "child".to_string(),
                kind: ProofNodeKind::Tactic,
                status: ProofNodeStatus::Proven,
                range: Range::single_line(1, 0, 5),
                children: Vec::new(),
                info: None,
            }],
            info: None,
        };
        assert_eq!(node.node_count(), 2);
    }
    #[test]
    fn test_proof_tree_sorry_count() {
        let node = ProofTreeNode {
            id: "1".to_string(),
            label: "root".to_string(),
            kind: ProofNodeKind::Theorem,
            status: ProofNodeStatus::Admitted,
            range: Range::single_line(0, 0, 10),
            children: vec![ProofTreeNode {
                id: "2".to_string(),
                label: "sorry".to_string(),
                kind: ProofNodeKind::Sorry,
                status: ProofNodeStatus::Admitted,
                range: Range::single_line(1, 2, 7),
                children: Vec::new(),
                info: None,
            }],
            info: None,
        };
        assert_eq!(node.sorry_count(), 1);
    }
    #[test]
    fn test_tactic_suggestor_equality() {
        let env = Environment::new();
        let suggestor = TacticSuggestor::new(&env);
        let goal = ProofGoal {
            index: 0,
            hypotheses: Vec::new(),
            target: "a = a".to_string(),
            tag: None,
            is_focused: true,
        };
        let suggestions = suggestor.suggest(&goal);
        assert!(suggestions.iter().any(|s| s.tactic == "rfl"));
    }
    #[test]
    fn test_tactic_suggestor_conjunction() {
        let env = Environment::new();
        let suggestor = TacticSuggestor::new(&env);
        let goal = ProofGoal {
            index: 0,
            hypotheses: Vec::new(),
            target: "And P Q".to_string(),
            tag: None,
            is_focused: true,
        };
        let suggestions = suggestor.suggest(&goal);
        assert!(suggestions.iter().any(|s| s.tactic == "constructor"));
    }
    #[test]
    fn test_proof_progress() {
        let doc = make_doc("theorem t1 : True := by\n  trivial\ntheorem t2 : True := by\n  sorry");
        let env = Environment::new();
        let progress = compute_proof_progress(&doc, &env);
        assert_eq!(progress.total_theorems, 2);
    }
    #[test]
    fn test_widget_kind_str() {
        assert_eq!(WidgetKind::GoalViewer.as_str(), "goalViewer");
        assert_eq!(WidgetKind::ProofTree.as_str(), "proofTree");
    }
    #[test]
    fn test_suggestion_category_str() {
        assert_eq!(SuggestionCategory::GoalBased.as_str(), "goalBased");
    }
    #[test]
    fn test_widget_manager_clear() {
        let mut mgr = WidgetManager::new();
        mgr.create_widget(
            WidgetKind::GoalViewer,
            "file:///a.lean",
            Position::new(0, 0),
        );
        mgr.create_widget(WidgetKind::ProofTree, "file:///a.lean", Position::new(1, 0));
        assert_eq!(mgr.widget_count(), 2);
        mgr.clear();
        assert_eq!(mgr.widget_count(), 0);
    }
}
/// Handles widget events for goal viewer state.
#[allow(dead_code)]
pub fn handle_goal_viewer_event(
    state: &mut GoalViewerState,
    event: WidgetEvent,
) -> WidgetEventResponse {
    match event {
        WidgetEvent::KeyPress { ref key, .. } => match key.as_str() {
            "ArrowDown" | "j" => {
                state.next_goal();
                WidgetEventResponse::Redraw
            }
            "ArrowUp" | "k" => {
                state.prev_goal();
                WidgetEventResponse::Redraw
            }
            "h" => {
                state.show_hypotheses = !state.show_hypotheses;
                WidgetEventResponse::Redraw
            }
            "t" => {
                state.show_types = !state.show_types;
                WidgetEventResponse::Redraw
            }
            "c" => {
                state.compact_mode = !state.compact_mode;
                WidgetEventResponse::Redraw
            }
            _ => WidgetEventResponse::NoOp,
        },
        WidgetEvent::Refresh => WidgetEventResponse::Redraw,
        _ => WidgetEventResponse::NoOp,
    }
}
/// Serialize a GoalViewerState to JSON value.
#[allow(dead_code)]
pub fn goal_viewer_to_json(state: &GoalViewerState) -> JsonValue {
    let goals: Vec<JsonValue> = state
        .goals
        .iter()
        .map(|g| {
            let hyps: Vec<JsonValue> = g
                .hypotheses
                .iter()
                .map(|h| {
                    JsonValue::Object(vec![
                        ("name".to_string(), JsonValue::String(h.name.clone())),
                        ("type".to_string(), JsonValue::String(h.type_str.clone())),
                    ])
                })
                .collect();
            JsonValue::Object(vec![
                ("index".to_string(), JsonValue::Number(g.index as f64)),
                ("target".to_string(), JsonValue::String(g.target.clone())),
                ("hypotheses".to_string(), JsonValue::Array(hyps)),
                ("is_closed".to_string(), JsonValue::Bool(g.is_closed)),
            ])
        })
        .collect();
    JsonValue::Object(vec![
        ("goals".to_string(), JsonValue::Array(goals)),
        (
            "cursor_pos".to_string(),
            JsonValue::Number(state.cursor_pos as f64),
        ),
    ])
}
/// Return the version for the widgets module.
#[allow(dead_code)]
pub fn widgets_version() -> &'static str {
    "0.1.0"
}
/// Return the set of supported widget kinds.
#[allow(dead_code)]
pub fn supported_widget_kinds() -> &'static [&'static str] {
    &[
        "GoalViewer",
        "ProofTree",
        "TacticSuggestion",
        "TypeInspector",
        "HypothesisExplorer",
        "ExpressionTree",
        "DocumentationPanel",
        "ProofProgress",
    ]
}
#[cfg(test)]
mod widget_extra_tests {
    use super::*;
    #[test]
    fn test_widget_color_css() {
        let color = WidgetColor::new(255, 128, 0);
        assert_eq!(color.to_css_hex(), "#FF8000");
    }
    #[test]
    fn test_widget_theme_dark() {
        let theme = WidgetTheme::dark();
        assert_eq!(theme.background.r, 30);
        assert_eq!(theme.font_family, "monospace");
    }
    #[test]
    fn test_widget_layout_toggle() {
        let mut layout = WidgetLayout::default_layout();
        assert!(!layout.collapsed);
        layout.toggle_collapsed();
        assert!(layout.collapsed);
        layout.toggle_collapsed();
        assert!(!layout.collapsed);
    }
    #[test]
    fn test_goal_viewer_state() {
        let mut state = GoalViewerState::empty();
        assert!(state.current_goal().is_none());
        state.goals.push(GoalDisplay {
            index: 0,
            target: "1 + 1 = 2".to_string(),
            hypotheses: vec![],
            tactic_state: "".to_string(),
            is_closed: false,
        });
        state.goals.push(GoalDisplay {
            index: 1,
            target: "True".to_string(),
            hypotheses: vec![],
            tactic_state: "".to_string(),
            is_closed: false,
        });
        assert_eq!(
            state
                .current_goal()
                .expect("test operation should succeed")
                .index,
            0
        );
        state.next_goal();
        assert_eq!(
            state
                .current_goal()
                .expect("test operation should succeed")
                .index,
            1
        );
        state.next_goal();
        assert_eq!(
            state
                .current_goal()
                .expect("test operation should succeed")
                .index,
            0
        );
    }
    #[test]
    fn test_goal_viewer_render() {
        let mut state = GoalViewerState::empty();
        state.goals.push(GoalDisplay {
            index: 0,
            target: "P -> P".to_string(),
            hypotheses: vec![HypothesisDisplay {
                name: "h".to_string(),
                type_str: "P".to_string(),
                value_str: None,
                is_local_def: false,
            }],
            tactic_state: "".to_string(),
            is_closed: false,
        });
        let rendered = state.render();
        assert!(rendered.contains("P -> P"));
        assert!(rendered.contains("h : P"));
    }
    #[test]
    fn test_proof_tree_node() {
        let mut root = ProofTreeNodeV2::leaf(0, "intro h", "P -> Q");
        root.children.push(ProofTreeNodeV2::leaf(1, "exact h", "Q"));
        assert_eq!(root.node_count(), 2);
        assert!(root.find(1).is_some());
        assert!(root.find(99).is_none());
        let rendered = root.render(0);
        assert!(rendered.contains("intro h"));
        assert!(rendered.contains("exact h"));
    }
    #[test]
    fn test_tactic_suggestion_state() {
        let mut state = TacticSuggestionState::new(vec![
            TacticSuggestionItem::new("intro", "Introduce hypothesis", 0.9),
            TacticSuggestionItem::new("exact", "Close goal", 0.8).closes_goal(),
            TacticSuggestionItem::new("apply", "Apply lemma", 0.7),
        ]);
        assert_eq!(state.suggestions.len(), 3);
        state.sort_by_confidence();
        assert_eq!(state.suggestions[0].tactic, "intro");
        state.filter_text = "ex".to_string();
        let filtered = state.filtered();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].tactic, "exact");
    }
    #[test]
    fn test_handle_goal_viewer_event() {
        let mut state = GoalViewerState::empty();
        state.goals.push(GoalDisplay {
            index: 0,
            target: "P".to_string(),
            hypotheses: vec![],
            tactic_state: "".to_string(),
            is_closed: false,
        });
        let resp = handle_goal_viewer_event(
            &mut state,
            WidgetEvent::KeyPress {
                key: "h".to_string(),
                modifiers: 0,
            },
        );
        assert!(matches!(resp, WidgetEventResponse::Redraw));
        assert!(!state.show_hypotheses);
    }
    #[test]
    fn test_widgets_version() {
        assert!(!widgets_version().is_empty());
    }
    #[test]
    fn test_supported_widget_kinds() {
        let kinds = supported_widget_kinds();
        assert!(kinds.contains(&"GoalViewer"));
        assert!(kinds.contains(&"ProofTree"));
    }
}
#[cfg(test)]
mod widget_ext_tests {
    use super::*;
    #[test]
    fn test_widget_history_push_undo_redo() {
        let mut history = WidgetHistory::new(10);
        let snap1 = WidgetSnapshot {
            widget_id: WidgetId::new("w1"),
            timestamp: std::time::Instant::now(),
            rendered: "state1".to_string(),
        };
        let snap2 = WidgetSnapshot {
            widget_id: WidgetId::new("w1"),
            timestamp: std::time::Instant::now(),
            rendered: "state2".to_string(),
        };
        history.push(snap1.clone());
        assert!(!history.can_undo());
        history.push(snap1.clone());
        history.push(snap2.clone());
        assert!(history.can_undo());
        let prev = history.undo();
        assert!(prev.is_some());
        assert!(history.can_redo());
        let next = history.redo();
        assert!(next.is_some());
    }
    #[test]
    fn test_focus_tracker() {
        let mut tracker = WidgetFocusTracker::new();
        assert!(tracker.current().is_none());
        tracker.focus(WidgetId::new("w1"));
        tracker.focus(WidgetId::new("w2"));
        assert_eq!(tracker.current(), Some(&WidgetId::new("w2")));
        tracker.focus_next();
    }
    #[test]
    fn test_widget_perf_stats() {
        let mut stats = WidgetPerfStats::default();
        stats.record_render(100);
        stats.record_render(200);
        stats.record_render(150);
        assert_eq!(stats.render_count, 3);
        assert_eq!(stats.min_render_us, 100);
        assert_eq!(stats.max_render_us, 200);
        assert!((stats.avg_render_us() - 150.0).abs() < 1.0);
    }
    #[test]
    fn test_widget_panel() {
        let mut panel = WidgetPanel::new("Proof State", PanelPosition::Right);
        panel.add_widget(WidgetId::new("goal_viewer"));
        assert_eq!(panel.widgets.len(), 1);
        assert!(panel.visible);
        panel.toggle_visible();
        assert!(!panel.visible);
    }
}
/// Return the feature list for this widget module.
#[allow(dead_code)]
pub fn widget_features() -> Vec<&'static str> {
    vec![
        "goal-viewer",
        "proof-tree",
        "tactic-suggestion",
        "type-inspector",
        "theming",
        "events",
        "snapshots",
        "undo-redo",
        "focus-tracking",
        "perf-stats",
        "panels",
        "search",
    ]
}
#[cfg(test)]
mod search_tests {
    use super::*;
    #[test]
    fn test_widget_search_index() {
        let mut idx = WidgetSearchIndex::new();
        idx.index(WidgetId::new("w1"), "forall x : Nat, x + 0 = x".to_string());
        idx.index(WidgetId::new("w2"), "P -> Q".to_string());
        let results = idx.search("Nat");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], &WidgetId::new("w1"));
    }
    #[test]
    fn test_widget_features() {
        let features = widget_features();
        assert!(features.contains(&"goal-viewer"));
        assert!(features.contains(&"proof-tree"));
    }
}
#[cfg(test)]
mod dispatcher_tests {
    use super::*;
    #[test]
    fn test_widget_dispatcher() {
        let mut dispatcher = WidgetDispatcher::new();
        dispatcher.record_response(WidgetId::new("w1"), WidgetEventResponse::Redraw);
        dispatcher.record_response(WidgetId::new("w1"), WidgetEventResponse::NoOp);
        let responses = dispatcher.responses_for(&WidgetId::new("w1"));
        assert_eq!(responses.len(), 2);
        dispatcher.clear();
        assert_eq!(dispatcher.responses_for(&WidgetId::new("w1")).len(), 0);
    }
}
