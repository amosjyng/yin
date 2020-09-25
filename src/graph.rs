//! Graph implementations

mod invalid_graph;
mod kb_wrapper;

use invalid_graph::InvalidGraph;
pub use kb_wrapper::{KBWrapper, WeakWrapper};
use petgraph::graph::NodeIndex;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static GRAPH: RefCell<Box<dyn Graph<'static>>> = RefCell::new(Box::new(InvalidGraph{}));
}

/// Bind GRAPH to a new graph that sits entirely in memory.
pub fn bind_in_memory_graph() {
    GRAPH.with(|g| *g.borrow_mut() = Box::new(InMemoryGraph::new()));
}

/// A classic directed Graph with nodes and labeled links.
pub trait Graph<'a> {
    /// Adds a new node to the graph, and returns the node's ID.
    fn add_node(&mut self) -> usize;

    /// Sets the value for a given node. Values can only be set once.
    fn set_node_value(&mut self, id: usize, value: Box<dyn KBWrapper>);

    /// Sets the name for a given node. Names can only be set once.
    fn set_node_name(&mut self, id: usize, name: String);

    /// Retrieve's a node's name from the graph, or None if the node does not exist or is unnamed.
    fn node_name(&self, id: usize) -> Option<String>;

    /// Retrieve's a node's name from the graph, or None if the node does not exist or does not
    /// have a value.
    fn node_value(&self, id: usize) -> Option<Rc<Box<dyn KBWrapper>>>;
}

/// Graph only for dependency injection.
pub struct InjectionGraph {}

impl Graph<'static> for InjectionGraph {
    fn add_node(&mut self) -> usize {
        GRAPH.with(|g| g.borrow_mut().add_node())
    }

    fn set_node_value(&mut self, id: usize, value: Box<dyn KBWrapper>) {
        GRAPH.with(|g| g.borrow_mut().set_node_value(id, value))
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        GRAPH.with(|g| g.borrow_mut().set_node_name(id, name))
    }

    fn node_name(&self, id: usize) -> Option<String> {
        GRAPH.with(|g| g.borrow().node_name(id).map(|n| n.clone()))
    }

    fn node_value(&self, id: usize) -> Option<Rc<Box<dyn KBWrapper>>> {
        GRAPH.with(|g| g.borrow().node_value(id).map(|r| r.clone()))
    }
}

struct NodeInfo {
    name: Option<String>,
    value: Option<Rc<Box<dyn KBWrapper>>>,
}

struct InMemoryGraph {
    graph: petgraph::graph::Graph<NodeInfo, NodeIndex>,
}

impl InMemoryGraph {
    /// Constructs an empty new in-memory graph
    fn new() -> Self {
        InMemoryGraph {
            graph: petgraph::graph::Graph::<NodeInfo, NodeIndex>::new(),
        }
    }
}

impl<'a> Graph<'a> for InMemoryGraph {
    fn add_node(&mut self) -> usize {
        let new_node_info = NodeInfo {
            name: None,
            value: None,
        };
        self.graph.add_node(new_node_info).index()
    }

    fn set_node_value(&mut self, id: usize, value: Box<dyn KBWrapper>) {
        self.graph
            .node_weight_mut(NodeIndex::new(id))
            .unwrap()
            .value = Some(Rc::new(value));
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        self.graph.node_weight_mut(NodeIndex::new(id)).unwrap().name = Some(name);
    }

    fn node_name(&self, id: usize) -> Option<String> {
        self.graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.name.as_ref().map(|n| n.clone()))
            .flatten()
    }

    fn node_value(&self, id: usize) -> Option<Rc<Box<dyn KBWrapper>>> {
        self.graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.value.as_ref().map(|v| v.clone()))
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory_graph_create() {
        bind_in_memory_graph();
    }

    #[test]
    fn in_memory_graph_add_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let id = g.add_node();
        assert!(g.node_value(id).is_none());
        assert_eq!(g.node_name(id), None);
    }

    #[test]
    fn in_memory_graph_set_node_value() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let v = Rc::new(Box::new(5));
        g.set_node_value(a_id, Box::new(WeakWrapper::new(Rc::downgrade(&v))));
        assert_eq!(
            g.node_value(a_id)
                .map(|v| {
                    v.as_any()
                        .downcast_ref::<WeakWrapper<Box<i32>>>()
                        .map(|w| w.item.upgrade().map(|rc| **rc))
                })
                .flatten()
                .flatten(),
            Some(5)
        );
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    fn in_memory_graph_retrieve_node_string_value() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let v = Rc::new(Box::new("5".to_owned()));
        g.set_node_value(a_id, Box::new(WeakWrapper::new(Rc::downgrade(&v))));
        assert_eq!(
            g.node_value(a_id)
                .map(|v| v
                    .as_any()
                    .downcast_ref::<WeakWrapper<Box<String>>>()
                    .map(|w| w.item.upgrade()))
                .flatten()
                .flatten(),
            Some(v)
        );
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    fn in_memory_graph_retrieve_node_name() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        g.set_node_name(a_id, "A".to_string());
        assert_eq!(g.node_name(a_id), Some("A".to_string()));
    }

    #[test]
    fn in_memory_graph_retrieve_node_name_value() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let v = Rc::new(Box::new(5));
        g.set_node_name(a_id, "A".to_string());
        g.set_node_value(a_id, Box::new(WeakWrapper::new(Rc::downgrade(&v))));
        assert_eq!(g.node_name(a_id), Some("A".to_string()));
        assert_eq!(
            g.node_value(a_id)
                .map(|v| v
                    .as_any()
                    .downcast_ref::<WeakWrapper<Box<i32>>>()
                    .map(|w| w.item.upgrade().map(|rc| **rc)))
                .flatten()
                .flatten(),
            Some(5)
        );
    }
}
