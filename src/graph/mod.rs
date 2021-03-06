extern crate rand;

use std::io;
use std::io::{BufRead, Read};
use std::collections::{BitVec, HashMap, BitSet};
use rand::random;
use BitGraph;

pub type Graph = Vec<BitVec>;

pub fn new(size: usize) -> Graph {
    //! Construct new Graph with size number of vertices.
    vec![BitVec::from_elem(size, false); size]
}

pub fn complete(n: usize) -> Graph {
    //! Construct K_n.
    (0..n).map(|i| BitVec::from_fn(n, |j| i != j)).collect()
}

pub fn erdos_renyi(n: usize, p: f64) -> Graph {
    //! Construct Erdos-Renyi random undirected graph on n vertices, p in range [0,1].
    let mut g = new(n);
    for i in (0..n) {
        for j in (i+1..n) {
            if random::<f64>() < p {
                g.add_edge(i,j);
            }
        }
    }
    assert!(g.verify());
    g
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
    fn in_neighbors(&self, id: usize) -> &BitVec {
        &self[id]
    }
    fn out_neighbors(&self, id: usize) -> &BitVec {
        &self[id]
    }
    fn complement(&self) -> Graph {
        // negate each row and zero the diagonal
        let mut new_graph = self.clone();
        for (i,r) in new_graph.iter_mut().enumerate() {
            r.negate();
            r.set(i, false);
        }
        new_graph
    }
    fn induce(&mut self, vertices: &BitVec) {
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
        // construct mapping of new indices -> old indices
        let map = (0..self.len()).filter(|&idx| self[idx].any()).collect();
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
    fn reordered(&self, order: &[usize]) -> Self {
        (0..self.len()).map(|v| BitVec::from_fn(self.len(), |w|
                                                self[order[v]][order[w]])).collect()
    }
    fn serialize_dot(&self, node_attrs: Option<&HashMap<usize, HashMap<String, String>>>,
                            edge_attrs: Option<&HashMap<(usize, usize), HashMap<String, String>>>)
        -> String
    {
        let mut out_lines = vec![format!("node [fontname=\"{}\",fontsize=\"{}\"]",
                                            "sans-serif", "12")];
        for (i,_) in self.iter().enumerate() {
            let mut n_props = vec![format!("id={}", i)];
            if let Some(attrs) = node_attrs {
                n_props.push_all(&attrs[&i].iter().map(|(k,v)| format!("{}=\"{}\"", k,
                                                                        v)).collect::<Vec<_>>());
            }
            out_lines.push(format!("{} [{}]", i, n_props.connect(",")));
            for j in BitSet::from_bit_vec(self.out_neighbors(i).clone()).iter().filter(|&j| i<=j) {
                let mut e_props = vec![format!("id=\"{},{}\"", i,j)];
                if let Some(attrs) = edge_attrs {
                    e_props.push_all(&attrs[&(i,j)].iter().map(|(k,v)| format!("{}=\"{}\"", k,
                                                                                v)).collect::<Vec<_>>());
                }
                out_lines.push(format!("{} -- {} [{}]", i, j, e_props.connect(",")));
            }
        }
        format!("strict graph {{\n{}\n}}", out_lines.connect("\n"))
    }
}

pub fn read_csv<R: Read>(reader: &mut io::BufReader<R>) -> Option<Graph> {
    //! Read csv of 0's and 1's into a graph from given buffered reader.
    //! Return Some(graph) if it passes validation, otherwise None.
    let a = reader.lines().map(
        // turn each row into iterator of ints
        |r| r.unwrap_or(String::new()).split(",").map(
            // parse each number into int and collect into vectors
            |s| s.trim().parse().unwrap_or(0)).map(|v| v == 1)
        .collect()).collect::<Vec<_>>();
    if a.verify() {
        Some(a)
    } else {
        None
    }
}
