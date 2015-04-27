use bitgraphs::BitGraph;
use bitgraphs::graph;
use bitgraphs::digraph;

#[test]
fn test_graph() {
    let mut g = graph::new(5);
    g.add_edge(1, 2);
    g.add_edges(2, &[3,4]);
    assert!(g.has_edge(1,2));
    assert!(g.has_edge(2,3));
    assert!(g.has_edge(4,2));
    assert!(g.in_neighbors(4) == g.in_neighbors(3));
    g.remove_edge(1, 2);
    assert!(!g.has_edge(1,2));
}

#[test]
fn test_digraph() {
    let mut g = digraph::new(5);
    g.add_edge(1, 2);
    g.add_edges(2, &[3,4]);
    assert!(g.has_edge(1,2));
    assert!(g.has_edge(2,3));
    assert!(!g.has_edge(4,2));
    assert!(g.in_neighbors(4) == g.in_neighbors(3));
    g.remove_edge(1, 2);
    assert!(!g.has_edge(1,2));
}
