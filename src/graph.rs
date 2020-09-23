//! Graph implementations

use petgraph::graph::NodeIndex;
use std::any::Any;

/// A classic directed Graph with nodes and labeled links.
pub trait Graph<'a> {
    /// Adds a new node to the graph, and returns the node's ID.
    fn add_node(&mut self) -> usize;

    /// Adds a new node with the given value to the graph, and returns the node's ID.
    fn add_node_with_value(&mut self, value: &'a dyn Any) -> usize;

    /// Adds a new node with the given string name to the graph, and returns the node's ID.
    fn add_node_with_name(&mut self, name: &'a str) -> usize;

    /// Adds a new node with the given string name and value to the graph, and returns the node's
    /// ID.
    fn add_node_with_name_value(&mut self, name: &'a str, value: &'a dyn Any) -> usize;

    /// Retrieve's a node's name from the graph, or None if the node does not exist or is unnamed.
    fn node_name(&self, id: usize) -> Option<&str>;

    /// Retrieve's a node's name from the graph, or None if the node does not exist or does not
    /// have a value.
    fn node_value(&self, id: usize) -> Option<&dyn Any>;
}

/// Create a new graph that resides entirely in memory.
pub fn new_in_memory_graph<'a>() -> impl Graph<'a> {
    InMemoryGraph::new()
}

struct NodeInfo<'a> {
    name: Option<&'a str>,
    value: Option<&'a dyn Any>,
}

struct InMemoryGraph<'a> {
    graph: petgraph::graph::Graph<NodeInfo<'a>, NodeIndex>,
}

impl<'a> InMemoryGraph<'a> {
    /// Constructs an empty new in-memory graph
    fn new() -> Self {
        InMemoryGraph {
            graph: petgraph::graph::Graph::<NodeInfo, NodeIndex>::new(),
        }
    }

    /// Adds a new node with corresponding info to an existing graph
    fn add_node_with_info(&mut self, name: Option<&'a str>, value: Option<&'a dyn Any>) -> usize {
        let new_node_info = NodeInfo {
            name: name,
            value: value,
        };
        return self.graph.add_node(new_node_info).index();
    }
}

impl<'a> Graph<'a> for InMemoryGraph<'a> {
    fn add_node(&mut self) -> usize {
        return self.add_node_with_info(None, None);
    }

    fn add_node_with_value(&mut self, value: &'a dyn Any) -> usize {
        return self.add_node_with_info(None, Some(value));
    }

    fn add_node_with_name(&mut self, name: &'a str) -> usize {
        return self.add_node_with_info(Some(name), None);
    }

    fn add_node_with_name_value(&mut self, name: &'a str, value: &'a dyn Any) -> usize {
        return self.add_node_with_info(Some(name), Some(value));
    }

    fn node_name(&self, id: usize) -> Option<&str> {
        return self
            .graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.name)
            .flatten();
    }

    fn node_value(&self, id: usize) -> Option<&dyn Any> {
        return self
            .graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.value)
            .flatten();
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
        g.add_node();
    }

    #[test]
    fn in_memory_graph_retrieve_node_value() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node_with_value(&5);
        assert_eq!(
            g.node_value(a_id).expect("entered 5").downcast_ref::<i32>(),
            Some(&5)
        );
    }

    #[test]
    fn in_memory_graph_retrieve_node_string_value() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node_with_value(&"5");
        assert_eq!(
            g.node_value(a_id)
                .expect("entered 5")
                .downcast_ref::<&str>(),
            Some(&"5")
        );
    }

    #[test]
    fn in_memory_graph_retrieve_node_without_value() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node();
        assert!(g.node_value(a_id).is_none());
    }

    #[test]
    fn in_memory_graph_retrieve_node_name() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node_with_name("A");
        assert_eq!(g.node_name(a_id), Some("A"));
    }

    #[test]
    fn in_memory_graph_retrieve_node_without_name() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node();
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    fn in_memory_graph_retrieve_node_name_value() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node_with_name_value("A", &5);
        assert_eq!(g.node_name(a_id), Some("A"));
        assert_eq!(
            g.node_value(a_id).expect("entered 5").downcast_ref::<i32>(),
            Some(&5)
        );
    }

    #[test]
    fn in_memory_graph_retrieve_node_name_string_value() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node_with_name_value("A", &"5");
        assert_eq!(g.node_name(a_id), Some("A"));
        assert_eq!(
            g.node_value(a_id)
                .expect("entered 5")
                .downcast_ref::<&str>(),
            Some(&"5")
        );
    }

    #[test]
    fn in_memory_graph_retrieve_node_name_without_value() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node_with_name("A");
        assert_eq!(g.node_name(a_id), Some("A"));
        assert!(g.node_value(a_id).is_none());
    }
}
