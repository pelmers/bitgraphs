use bitgraphs::BitGraph;
use bitgraphs::utils;
use bitgraphs::graph;
use std::collections::BitVec;

#[test]
fn test_utils() {
    let g = graph::complete(10);
    assert!(utils::sum(&g.neighbors(0)) == 9);
    assert!(utils::dot(&g.neighbors(0), &g.neighbors(1)) == 8);
    assert!(utils::greedy_color(&g, &(0..10).collect()).len() == 10);
    assert!(utils::is_clique(&g, &BitVec::from_elem(10, true)));
    let d = utils::dfs(&g, 0, &mut |_| {});
    assert!(d.len() == g.len());
    let (b,p) = utils::bfs(&g, 0, &mut |_,_,_| {});
    assert!(b.len() == p.len() && b.len() == g.len());
}
