//! The edges `run_mapper` adds between overlapping cover elements, against
//! the previous construction (the common points of two elements, each looked
//! up with `position(..).expect(..)`), recomputed from the returned cover.

use super::{first_positions, run_mapper};
use crate::topological_data_analysis::types::{CoverElement, MapperGraph, TomographicProjection};
use std::collections::{HashMap, HashSet};

fn old_edges(cover: &[CoverElement], graph: &MapperGraph) -> Vec<(usize, usize)> {
    let node_map: HashMap<(usize, usize), usize> = graph
        .nodes
        .iter()
        .enumerate()
        .map(|(id, &key)| (key, id))
        .collect();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for ci in 0..cover.len() {
        for cj in (ci + 1)..cover.len() {
            let common: HashSet<usize> = cover[ci]
                .points
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&cover[cj].points.iter().copied().collect::<HashSet<_>>())
                .copied()
                .collect();
            for &p in &common {
                let idx_i = cover[ci]
                    .points
                    .iter()
                    .position(|&q| q == p)
                    .expect("p is in cover[ci]");
                let idx_j = cover[cj]
                    .points
                    .iter()
                    .position(|&q| q == p)
                    .expect("p is in cover[cj]");
                let cli = cover[ci].clusters[idx_i];
                let clj = cover[cj].clusters[idx_j];
                if let (Some(&ni), Some(&nj)) = (node_map.get(&(ci, cli)), node_map.get(&(cj, clj)))
                {
                    if !edges.contains(&(ni, nj)) && !edges.contains(&(nj, ni)) {
                        edges.push((ni, nj));
                    }
                }
            }
        }
    }
    edges
}

#[test]
fn first_positions_keeps_the_first_index() {
    let first = first_positions(&[5, 3, 5, 9, 3]);
    assert_eq!(first.len(), 3);
    assert_eq!(first.get(&5), Some(&0));
    assert_eq!(first.get(&3), Some(&1));
    assert_eq!(first.get(&9), Some(&3));
    assert!(first_positions(&[]).is_empty());
}

#[test]
fn mapper_edges_are_the_ones_the_previous_construction_adds() {
    let mut state = 0x510e_527f_ade6_82d1u64;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut with_edges = 0usize;
    for _ in 0..400 {
        let n = (next() % 14) as usize;
        let values: Vec<f64> = (0..n).map(|_| (next() % 12) as f64 * 0.25).collect();
        let filter = TomographicProjection {
            name: "f".to_string(),
            values,
        };
        let intervals = 1 + (next() % 5) as usize;
        let overlap = (next() % 7) as f64 * 0.1;
        let result = run_mapper(&filter, intervals, overlap);
        let expected = old_edges(&result.cover, &result.graph);
        let got: HashSet<(usize, usize)> = result.graph.edges.iter().copied().collect();
        assert_eq!(got.len(), result.graph.edges.len());
        assert_eq!(got, expected.iter().copied().collect::<HashSet<_>>());
        if !got.is_empty() {
            with_edges += 1;
        }
    }
    assert!(with_edges > 0);
}
