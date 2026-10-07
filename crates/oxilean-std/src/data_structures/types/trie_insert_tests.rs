//! `Trie::insert` against a verbatim copy of the previous code, which read
//! the child it had just created back with `expect`.

use super::{Trie, TrieNode};

fn old_insert(t: &mut Trie, key: &str) {
    let mut current = 0;
    for byte in key.bytes() {
        let idx = byte as usize;
        if t.nodes[current].children[idx].is_none() {
            let new_node = t.nodes.len();
            t.nodes.push(TrieNode::new());
            t.nodes[current].children[idx] = Some(new_node);
        }
        current = t.nodes[current].children[idx].expect("just inserted");
    }
    t.nodes[current].is_terminal = true;
}
fn layout(t: &Trie) -> Vec<(Vec<Option<usize>>, bool)> {
    t.nodes
        .iter()
        .map(|n| (n.children.clone(), n.is_terminal))
        .collect()
}

#[test]
fn tries_are_built_node_for_node_as_before() {
    let keys = [
        "", "a", "ab", "abc", "abd", "b", "ab", "zz", "αβ", "a\u{0}b", "abc",
    ];
    let mut new = Trie::new();
    let mut old = Trie::new();
    for key in keys {
        new.insert(key);
        old_insert(&mut old, key);
        assert_eq!(layout(&new), layout(&old), "after {key:?}");
    }
    for key in keys {
        assert!(new.search(key));
    }
    assert!(!new.search("abe"));
}
