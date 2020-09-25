//! Graph implementations

mod in_memory_graph;
mod injection_graph;
mod invalid_graph;
mod kb_wrapper;

pub use injection_graph::{bind_in_memory_graph, InjectionGraph};
pub use kb_wrapper::{KBWrapper, WeakWrapper};

use std::rc::Rc;

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

struct NodeInfo {
    name: Option<String>,
    value: Option<Rc<Box<dyn KBWrapper>>>,
}
