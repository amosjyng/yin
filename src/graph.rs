//! Graph implementations

use petgraph::graph::NodeIndex;
use std::any::Any;

/// A classic directed Graph with nodes and labeled links.
pub trait Graph<'a> {
    /// Adds a new node to the graph, and returns the node's ID.
    fn add_node(&mut self) -> usize;

    /// Sets the value for a given node. Values can only be set once.
    fn set_node_value(&mut self, id: usize, value: &'a dyn Any);

    /// Sets the name for a given node. Names can only be set once.
    fn set_node_name(&mut self, id: usize, name: &'a str);

    /// Retrieve's a node's name from the graph, or None if the node does not exist or is unnamed.
    fn node_name(&self, id: usize) -> Option<&'a str>;

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
}

impl<'a> Graph<'a> for InMemoryGraph<'a> {
    fn add_node(&mut self) -> usize {
        let new_node_info = NodeInfo {
            name: None,
            value: None,
        };
        self.graph.add_node(new_node_info).index()
    }

    fn set_node_value(&mut self, id: usize, value: &'a dyn Any) {
        self.graph.node_weight_mut(NodeIndex::new(id)).expect("").value = Some(value);
    }

    fn set_node_name(&mut self, id: usize, name: &'a str) {
        self.graph.node_weight_mut(NodeIndex::new(id)).expect("").name = Some(name);
    }

    fn node_name(&self, id: usize) -> Option<&'a str> {
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
        let id = g.add_node();
        assert!(g.node_value(id).is_none());
        assert_eq!(g.node_name(id), None);
    }

    #[test]
    fn in_memory_graph_set_node_value() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node();
        g.set_node_value(a_id, &5);
        assert_eq!(
            g.node_value(a_id).expect("entered 5").downcast_ref::<i32>(),
            Some(&5)
        );
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    fn in_memory_graph_retrieve_node_string_value() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node();
        g.set_node_value(a_id, &"5");
        assert_eq!(
            g.node_value(a_id)
                .expect("entered 5")
                .downcast_ref::<&str>(),
            Some(&"5")
        );
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    fn in_memory_graph_retrieve_node_name() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node();
        g.set_node_name(a_id, "A");
        assert_eq!(g.node_name(a_id), Some("A"));
    }

    #[test]
    fn in_memory_graph_retrieve_node_name_value() {
        let mut g = new_in_memory_graph();
        let a_id = g.add_node();
        g.set_node_name(a_id, "A");
        g.set_node_value(a_id, &5);
        assert_eq!(g.node_name(a_id), Some("A"));
        assert_eq!(
            g.node_value(a_id).expect("entered 5").downcast_ref::<i32>(),
            Some(&5)
        );
    }
}
