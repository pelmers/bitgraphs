use std::io;
use std::io::{BufRead, Read};
use std::collections::BitVec;
use BitGraph;

pub type Graph = Vec<BitVec>;

pub fn new(size: usize) -> Graph {
    //! Construct new Graph with size number of vertices.
    vec![BitVec::from_elem(size, false); size]
}

impl BitGraph for Graph {
    fn verify(&self) -> bool {
        //! Make sure matrix is symmetric.
        let n = self.len();
        n == 0 || (self.iter().all(|v| v.len() == n) &&
                (0..n).all(|i| (0..n).all(|j| self[i][j] == self[j][i])))
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn add_edge(&mut self, fr: usize, to: usize) {
        self[fr].set(to, true);
        self[to].set(fr, true);
    }
    fn remove_edge(&mut self, fr: usize, to: usize) {
        self[fr].set(to, false);
        self[to].set(fr, false);
    }
    fn neighbors(&self, id: usize) -> BitVec {
        self[id].clone()
    }
    fn in_neighbors(&self, id: usize) -> BitVec {
        self.neighbors(id)
    }
    fn out_neighbors(&self, id: usize) -> BitVec {
        self.neighbors(id)
    }
    fn induce(&mut self, vertices: BitVec) {
        for (i, r) in self.iter_mut().enumerate() {
            if vertices[i] {
                r.intersect(&vertices);
            } else {
                r.set_all();
                r.negate();
            }
        }
    }
    fn compressed(&self) -> (Graph, Vec<usize>) {
        // construct mapping of old indices -> new indices
        let map = (0..self.len()).rev().scan(0, |num_empty, idx| {
            if self[idx].none() {
                *num_empty += 1;
            }
            Some(idx-*num_empty)
        }).collect();
        let mut new_graph = self.clone();
        // retain all connected vertices
        new_graph.retain(BitVec::any);
        (new_graph, map)
    }
    fn contract(&mut self, e: (usize, usize)) {
        let (fr, to) = e;
        assert!(self[fr][to]);
        for (i,r) in self.iter_mut().enumerate() {
            if i == to {
                r.set_all();
                r.negate();
            } else if r[to] {
                r.set(fr, true);
                r.set(to, false);
            }
        }
    }
}

pub fn read_csv<R: Read>(reader: &mut io::BufReader<R>) -> Option<Graph> {
    //! Read csv of 0's and 1's into a graph from given buffered reader.
    //! Return Some(graph) if it passes validation, otherwise None.
    let a = reader.lines().map(
        // turn each row into iterator of ints
        |r| r.unwrap_or(String::new()).split(",").map(
            // parse each number into int and collect into vectors
            |s| s.parse().unwrap_or(0)).map(|v| v == 1)
        .collect::<BitVec>()).collect::<Vec<_>>();
    if a.verify() {
        Some(a)
    } else {
        None
    }
}
