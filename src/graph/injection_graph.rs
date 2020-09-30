use super::in_memory_graph::InMemoryGraph;
use super::invalid_graph::InvalidGraph;
use super::{Graph, KBWrapper};
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static GRAPH: RefCell<Box<dyn Graph<'static>>> = RefCell::new(Box::new(InvalidGraph{}));
}

/// Bind GRAPH to a new graph that sits entirely in memory.
pub fn bind_in_memory_graph() {
    GRAPH.with(|g| *g.borrow_mut() = Box::new(InMemoryGraph::new()));
}

/// Graph usable with dependency injection.
#[derive(Copy, Clone)]
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

    fn add_edge(&mut self, from: usize, edge_type: usize, to: usize) {
        GRAPH.with(|g| g.borrow_mut().add_edge(from, edge_type, to))
    }

    fn has_edge(&self, from: usize, edge_type: usize, to: usize) -> bool {
        GRAPH.with(|g| g.borrow().has_edge(from, edge_type, to))
    }

    fn outgoing_nodes(&self, from: usize, edge_type: usize) -> Vec<usize> {
        GRAPH.with(|g| g.borrow().outgoing_nodes(from, edge_type))
    }

    fn incoming_nodes(&self, to: usize, edge_type: usize) -> Vec<usize> {
        GRAPH.with(|g| g.borrow().incoming_nodes(to, edge_type))
    }

    fn all_outgoing_nodes(&self, from: usize) -> Vec<usize> {
        GRAPH.with(|g| g.borrow().all_outgoing_nodes(from))
    }

    fn all_incoming_nodes(&self, to: usize) -> Vec<usize> {
        GRAPH.with(|g| g.borrow().all_incoming_nodes(to))
    }
}
