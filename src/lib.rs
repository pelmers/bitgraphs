#![feature(collections)]
use std::collections::{BitVec, BitSet, VecDeque};

pub mod graph;

pub fn sum(vec: &BitVec) -> usize {
    //! Return number of true elements in given vector.
    vec.iter().filter(|v| *v).count()
}

pub trait BitGraph
    where Self: Clone {
    /// Verify that this graph is valid.
    fn verify(&self) -> bool;
    /// Return number of vertices in self.
    fn len(&self) -> usize;
    /// Add edge from fr to to.
    fn add_edge(&mut self, fr: usize, to: usize);
    /// Add edges from fr to each true index in toset, |toset| = |V|.
    fn add_edges(&mut self, fr: usize, toset: BitVec) {
        for (i, _) in toset.iter().enumerate().filter(|&(_,b)| b) {
            self.add_edge(fr, i);
        }
    }
    /// Remove edge from fr to to.
    fn remove_edge(&mut self, fr: usize, to: usize);
    /// Remove edges from fr to each true index in toset, |toset| = |V|.
    fn remove_edges(&mut self, fr: usize, toset: BitVec) {
        for (i, _) in toset.iter().enumerate().filter(|&(_,b)| b) {
            self.remove_edge(fr, i);
        }
    }
    /// Vector of in neighborhood of a given vertex.
    fn in_neighbors(&self, id: usize) -> BitVec;
    /// Vector of out neighborhood of given vertex.
    fn out_neighbors(&self, id: usize) -> BitVec;
    /// Vector of all neighbors of given vertex.
    fn neighbors(&self, id: usize) -> BitVec {
        let mut i = self.in_neighbors(id);
        i.union(&self.out_neighbors(id));
        i
    }
    /// Induce subgraph of given vertices. Does not change size of graph, but disconnects vertices
    /// not set in given set.
    fn induce(&mut self, vertices: BitVec);
    /// Contract the given edge e = (u->v). Size of self does not change, but all edges incident to
    /// v become incident to u instead, and v is disconnected from the graph. e must be an edge in
    /// the graph.
    fn contract(&mut self, e: (usize, usize));
    /// Return copy of self with all disconnected vertices removed and return a vector v where
    /// v[i_old]=i_new, a mapping from old indices to new indices.
    fn compressed(&self) -> (Self, Vec<usize>);
}

pub fn bfs<G,F>(g: &G, start: usize, visitor: Option<F>) -> Vec<i32>
    where G:BitGraph, F:Fn(usize) {
    //! Perform breadth-first search on graph from given start.
    //! Call optional visitor at each vertex visited in BFS order.
    //! Return mapping of id->depth, -1 for unreached vertices.
    let mut dists = vec![-1; g.len()];
    let mut q = VecDeque::with_capacity(g.len());
    q.push_back(start);
    dists[start] = 0;
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        match visitor {
            Some(ref f) => f(v),
            _ => ()
        };
        for n in BitSet::from_bit_vec(g.out_neighbors(v)).iter() {
            dists[n] = dists[v] + 1;
            q.push_back(n);
        }
    }
    dists
}

pub fn dfs<G,F>(g: &G, start: usize, visitor: Option<F>) -> Vec<i32>
    where G:BitGraph, F:Fn(usize) {
    //! Perform depth-first search on graph from given start.
    //! Call optional visitor at each vertex visited in DFS order.
    //! Return mapping of id->visit time, -1 for unreached vertices.
    let mut order = vec![-1; g.len()];
    let mut stack = vec![start];
    let mut iter = 0;
    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        order[v] = iter;
        match visitor {
            Some(ref f) => f(v),
            _ => ()
        };
        stack.append(&mut BitSet::from_bit_vec(
                g.out_neighbors(v)).iter().collect());
        iter += 1;
    }
    order
}
