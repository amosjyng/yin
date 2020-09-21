//! Graph implementations

use petgraph::graph::{NodeIndex};

/// A classic directed Graph with nodes and labeled links.
pub trait Graph<'a> {
    /// Adds a new node with the given string name to the graph, and returns the node's ID.
    fn add_node(&mut self, name: &'a str) -> usize;
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_in_memory_graph() {
        new_in_memory_graph();
    }

    #[test]
    fn add_node_to_in_memory_graph() {
        let mut g = new_in_memory_graph();
        g.add_node("A");
    }
}
