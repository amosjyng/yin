//! Graph implementations

use petgraph::graph::{NodeIndex};

/// A classic directed Graph with nodes and labeled links.
pub trait Graph<'a> {
    /// Adds a new node with the given string name to the graph, and returns the node's ID.
    fn add_node(&mut self, name: &'a str) -> usize;

    /// Retrieve's a node's name from the graph, or None if the node does not exist.
    fn node_name(&self, id: usize) -> Option<&str>;
}

/// Create a new graph that resides entirely in memory.
pub fn new_in_memory_graph<'a>() -> impl Graph<'a> {
    InMemoryGraph::new()
}

struct InMemoryGraph<'a> {
    graph: petgraph::graph::Graph::<&'a str, NodeIndex>
}

impl<'a> InMemoryGraph<'a> {
    fn new() -> Self {
        InMemoryGraph {
            graph: petgraph::graph::Graph::<&'a str, NodeIndex>::new()
        }
    }
}

impl<'a> Graph<'a> for InMemoryGraph<'a> {
    fn add_node(&mut self, name: &'a str) -> usize {
        return self.graph.add_node(name).index();
    }

    fn node_name(&self, id: usize) -> Option<&str> {
        return self.graph.node_weight(NodeIndex::new(id)).map(|s| *s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory_graph_create() {
        new_in_memory_graph();
    }

    #[test]
    fn in_memory_graph_add_node() {
        let mut g = new_in_memory_graph();
        g.add_node("A");
    }

    #[test]
    fn in_memory_graph_retrieve_node_name() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node("A");
        assert_eq!(g.node_name(a_id), Some("A"));
    }
}
