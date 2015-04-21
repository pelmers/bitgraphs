use BitGraph;
use std::collections::{BitVec, BitSet, VecDeque};

pub fn sum(vec: &BitVec) -> usize {
    //! Return number of true elements in given vector.
    vec.iter().filter(|v| *v).count()
}

pub fn dot(a: &BitVec, b: &BitVec) -> usize {
    //! Return dot product of a and b.
    let mut comp = a.clone();
    comp.intersect(b);
    sum(&comp)
}

pub fn bfs<G,F>(g: &G, start: usize, visitor: &mut F) -> Vec<i32>
    where G:BitGraph, F:FnMut(usize) {
    //! Perform breadth-first search on graph from given start.
    //! Call optional visitor at each vertex visited in BFS order.
    //! Return mapping of id->depth, -1 for unreached vertices.
    let mut dists = vec![-1; g.len()];
    let mut q = VecDeque::with_capacity(g.len());
    let mut visited = BitSet::with_capacity(g.len());
    q.push_back(start);
    visited.insert(start);
    dists[start] = 0;
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        visitor(v);
        for n in BitSet::from_bit_vec(g.out_neighbors(v).clone())
                    .iter().filter(|&v| visited.insert(v)) {
            dists[n] = dists[v] + 1;
            q.push_back(n);
        }
    }
    dists
}

pub fn dfs<G,F>(g: &G, start: usize, visitor: &mut F) -> Vec<i32>
    where G:BitGraph, F:FnMut(usize) {
    //! Perform depth-first search on graph from given start.
    //! Call optional visitor at each vertex visited in DFS order.
    //! Return mapping of id->visit time, -1 for unreached vertices.
    let mut order = vec![-1; g.len()];
    let mut stack = vec![start];
    let mut iter = 0;
    let mut visited = BitSet::with_capacity(g.len());
    visited.insert(start);
    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        order[v] = iter;
        visitor(v);
        stack.append(&mut BitSet::from_bit_vec(g.out_neighbors(v).clone())
                        .iter().filter(|&v| visited.insert(v)).collect());
        iter += 1;
    }
    order
}

pub fn is_independent<G: BitGraph>(g: &G, s: &BitSet) -> bool {
    //! Decide whether given s is an independent set in g.
    s.iter().all(|u| s.iter().all(|v| !g.out_neighbors(u)[v]))
}

pub fn is_clique<G: BitGraph>(g: &G, v: &BitVec) -> bool {
    //! Decide whether v forms a clique in g.
    let k = sum(v) - 1;
    // filter for elements in v, check that their intersection is k
    (0..g.len()).filter(|i| v[*i]).all(|i| dot(v, g.out_neighbors(i)) == k)
}

fn assign_color(neighbors: &BitVec, coloring: &Vec<BitSet>) -> usize {
    //! Pick a color that does not conflict with any neighbors.
    //! Color can be in range [0..len(colors)] (note the inclusive range).
    coloring.iter().enumerate().find(|&(_,c)| {
        let mut color_vec = c.clone().into_bit_vec();
        color_vec.intersect(neighbors);
        sum(&color_vec) == 0
    }).map(|(i,_)| i).unwrap_or(coloring.len())
}

pub fn greedy_color<G:BitGraph>(g: &G, order: Vec<usize>) -> Vec<BitSet> {
    //! Perform a greedy coloring of g, visiting vertices in the specified order.
    //! Return a vector of the color classes in g.
    //! This will use at most max degree colors.
    let mut coloring = vec![BitSet::with_capacity(g.len())];
    // assign each vertex in the order the first available color
    for &v in order.iter() {
        let c = assign_color(g.out_neighbors(v), &coloring);
        if c < coloring.len() {
            coloring[c].insert(v);
        } else {
            let mut new_color = BitSet::with_capacity(g.len());
            new_color.insert(v);
            coloring.push(new_color);
        }
    }
    // assert that each color class is independent
    assert!(coloring.iter().all(|c| is_independent(g, c)));
    coloring
}
