use tindex::{bitset::TBitSet, tvec, TIndex, TVec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(usize);

impl From<usize> for NodeId {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl TIndex for NodeId {
    fn as_index(&self) -> usize {
        self.0
    }
}

/// A undirected graph made of nodes connected by edges.
#[derive(Debug, Clone)]
pub struct Graph {
    // Invariants
    //
    // - reflexive: `nodes[a].get(b) == nodes[b].get(a)`
    nodes: TVec<NodeId, TBitSet<NodeId>>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph { nodes: TVec::new() }
    }
}

impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if this graph is correct and panics in case the internal invariants are not met.
    ///
    /// This should never happen.
    pub fn check_invariants(&self) {
        for a in self.nodes.index_iter() {
            for b in self.nodes[a].iter() {
                assert!(b.0 < self.nodes.len());
                assert!(self.nodes[b].get(a))
            }
        }
    }

    pub fn add_node(&mut self) -> NodeId {
        self.nodes.push(TBitSet::new())
    }

    /// Adds an edge between `a` and `b`.
    /// 
    /// Edges with `a` == `b` are ignored.
    pub fn add_edge(&mut self, a: NodeId, b: NodeId) {
        if a != b {
            self.nodes[a].add(b);
            self.nodes[b].add(a);
        }
    }

    /// Removes the edge between `a` and `b`.
    pub fn remove_edge(&mut self, a: NodeId, b: NodeId) {
        self.nodes[a].remove(b);
        self.nodes[b].remove(a);
    }

    /// Computes the minimal coloring for this graph.
    ///
    /// As this method uses a heuristic approach, the
    /// actual minimum may be lower.
    pub fn minimal_coloring(&self) -> Coloring {
        self.check_invariants();
        let mut k = 0;
        // all popped nodes
        let mut stack = Vec::new();
        // The modified graph
        // all alive nodes only connect to other living nodes,
        // the node on top of the stack also only connects to living nodes.
        // This object does not hold the invariants of `Graph`.
        let mut graph = self.nodes.clone();
        let mut alive: TBitSet<NodeId> = graph.index_iter().collect();

        // Remove the first node with less than `k` alive neighbors until the graph is empty.
        // In case no such node exists, set `k` to the current minimum.
        'outer: while !alive.is_empty() {
            let mut min_k = usize::max_value();
            for node in alive.iter() {
                let connected = graph[node].element_count();
                if connected < k {
                    alive.remove(node);
                    stack.push(node);
                    for other in alive.iter() {
                        graph[other].remove(node);
                    }
                    continue 'outer;
                } else if connected + 1 < min_k {
                    min_k = connected + 1;
                }
            }
            k = min_k;
        }

        let mut colors = tvec![usize::max_value(); graph.len()];
        while let Some(node) = stack.pop() {
            let mut allowed_colors: TBitSet<usize> = (0..k).collect();
            for other in graph[node].iter() {
                allowed_colors.remove(colors[other]);
            }

            // set the color of node to the first free color
            colors[node] = allowed_colors.iter().next().unwrap();
        }

        Coloring { k, nodes: colors }
    }
}

/// A valid coloring for a given graph.
pub struct Coloring {
    /// The amount of unique colors needed.
    pub k: usize,
    /// The color of each node, inside of the range `0..k`.
    pub nodes: TVec<NodeId, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut g = Graph::new();

        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        let d = g.add_node();

        g.add_edge(a, c);
        g.add_edge(a, d);
        g.add_edge(b, c);
        g.add_edge(b, d);
        g.add_edge(c, d);

        let coloring = g.minimal_coloring();
        assert_eq!(coloring.k, 3);
        assert_eq!(coloring.nodes[a], coloring.nodes[b]);
        assert_ne!(coloring.nodes[a], coloring.nodes[c]);
        assert_ne!(coloring.nodes[a], coloring.nodes[d]);
        assert_ne!(coloring.nodes[c], coloring.nodes[d]);
    }
}
