use bitgraphs::BitGraph;
use bitgraphs::graph;
use std::collections::BitVec;

#[test]
fn test_induce() {
    let mut g = graph::complete(3);
    let mut v = BitVec::from_elem(3, false);
    v.set(0, true); v.set(2, true);
    g.induce(&v);
    assert!(g.has_edge(0, 2));
    assert!(!g.has_edge(1, 0));
    assert!(!g.has_edge(1, 2));

    let (gp, map) = g.compressed();
    assert!(gp.len() == 2);
    assert!(map == vec![0,2]);
}
