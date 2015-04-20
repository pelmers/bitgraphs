#![feature(collections)]
use std::collections::{BitVec, HashMap};

pub mod graph;
pub mod utils;

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
    fn in_neighbors(&self, id: usize) -> &BitVec;
    /// Vector of out neighborhood of given vertex.
    fn out_neighbors(&self, id: usize) -> &BitVec;
    /// Vector of all neighbors of given vertex.
    fn neighbors(&self, id: usize) -> BitVec {
        let mut i = self.in_neighbors(id).clone();
        i.union(self.out_neighbors(id));
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
    /// Serialize the graph to DOT GraphViz format, where optional attribute maps contain valid
    /// GraphViz properties.
    fn serialize_dot(&self, node_attrs: Option<&HashMap<usize, HashMap<String, String>>>,
                            edge_attrs: Option<&HashMap<(usize, usize), HashMap<String, String>>>)
        -> String;
}

