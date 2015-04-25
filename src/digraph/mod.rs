use std::io;
use std::io::{BufRead, Read};
use std::collections::{BitVec, HashMap, BitSet};
use graph::Graph;
use BitGraph;

#[derive(Clone, Debug)]
pub struct DiGraph {
    /// from[i] is the in neighborhood of i.
    from: Vec<BitVec>,
    /// to[i] is the out neighborhood of i.
    to: Vec<BitVec>
}

pub fn new(size: usize) -> DiGraph {
    //! Construct new Graph with size number of vertices.
    DiGraph {
        from: vec![BitVec::from_elem(size, false); size],
        to: vec![BitVec::from_elem(size, false); size]
    }
}

impl DiGraph {
    pub fn transpose(&self) -> DiGraph {
        //! Transpose of self, aka reversed edges.
        DiGraph {
            from: self.to.clone(),
            to: self.from.clone()
        }
    }
}

impl BitGraph for DiGraph {
    fn verify(&self) -> bool {
        self.from.len() == self.to.len()
    }
    fn len(&self) -> usize {
        self.from.len()
    }
    fn add_edge(&mut self, fr: usize, to: usize) {
        self.from[to].set(fr, true);
        self.to[fr].set(to, true);
    }
    fn remove_edge(&mut self, fr: usize, to: usize) {
        self.from[to].set(fr, false);
        self.to[fr].set(to, false);
    }
    fn in_neighbors(&self, id: usize) -> &BitVec {
        &self.from[id]
    }
    fn out_neighbors(&self, id: usize) -> &BitVec {
        &self.to[id]
    }
    fn complement(&self) -> DiGraph {
        DiGraph {
            from: (&self.from as &Graph).complement(),
            to: (&self.to as &Graph).complement()
        }
    }
    fn induce(&mut self, vertices: &BitVec) {
        (&mut self.from as &mut Graph).induce(vertices);
        (&mut self.to as &mut Graph).induce(vertices);
    }
    fn compressed(&self) -> (DiGraph, Vec<usize>) {
        let map = (0..self.len()).rev().scan(0, |num_empty, idx| {
            if self.from[idx].none() && self.to[idx].none() {
                *num_empty += 1;
            }
            Some(idx-*num_empty)
        }).collect();

        let (new_fr, new_to) = self.from.iter().zip(self.to.iter())
            .filter(|&(f,t)| f.any() || t.any())
            .map(|(x,y)| (x.clone(), y.clone()))
            .unzip();
        (DiGraph {
            from: new_fr,
            to: new_to
        }, map)
    }
    fn contract(&mut self, e: (usize, usize)) {
        let (u, v) = e;
        assert!(self.from[u][v]);
        let from_v = self.from[v].clone();
        let to_v = self.to[v].clone();
        self.from[u].union(&from_v);
        self.to[u].union(&to_v);
        for i in BitSet::from_bit_vec(to_v).iter() {
            self.from[i].set(v, false);
            self.from[i].set(u, true);
        }
        for i in BitSet::from_bit_vec(from_v).iter() {
            self.to[i].set(v, false);
            self.to[i].set(u, true);
        }

        // clear the rows of v
        self.to[v].set_all();
        self.to[v].negate();
        self.from[v].set_all();
        self.from[v].negate();
    }
    fn reordered(&self, order: &Vec<usize>) -> Self {
        DiGraph {
            from: (&self.from as &Graph).reordered(order),
            to: (&self.to as &Graph).reordered(order)
        }
    }
    fn serialize_dot(&self, node_attrs: Option<&HashMap<usize, HashMap<String, String>>>,
                            edge_attrs: Option<&HashMap<(usize, usize), HashMap<String, String>>>)
        -> String
    {
        // just like for graph, change -- to -> and graph to digraph
        let mut out_lines = vec![format!("node [fontname=\"{}\",fontsize=\"{}\"]",
                                            "sans-serif", "12")];
        for i in (0..self.len()) {
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
                out_lines.push(format!("{} -> {} [{}]", i, j, e_props.connect(",")));
            }

        }
        format!("digraph {{\n{}\n}}", out_lines.connect("\n"))
    }
}

pub fn read_csv<R: Read>(reader: &mut io::BufReader<R>) -> Option<DiGraph> {
    //! Read csv of 0's and 1's into a graph from given buffered reader.
    //! Return Some(graph) if it passes validation, otherwise None.
    let from = reader.lines().map(
        // turn each row into iterator of ints
        |r| r.unwrap_or(String::new()).split(",").map(
            // parse each number into int and collect into vectors
            |s| s.trim().parse().unwrap_or(0)).map(|v| v == 1)
        .collect::<BitVec>()).collect::<Vec<_>>();
    // to is transpose of from
    let to = (0..from.len()).map(
        |i| (0..from.len()).map(
            |j| from[j][i]).collect()).collect();
    let graph = DiGraph {
        from: from,
        to: to
    };
    if graph.verify() {
        Some(graph)
    } else {
        None
    }
}
