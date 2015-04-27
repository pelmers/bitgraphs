use bitgraphs::BitGraph;
use bitgraphs::graph;

#[test]
fn test_rearrange() {
    let g = graph::complete(4);
    let gp = g.rearranged(&[3,1,2,0]);
    assert!(g.neighbors(3) == gp.neighbors(3));
}
