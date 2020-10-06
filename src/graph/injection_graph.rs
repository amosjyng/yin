#[cfg(feature = "cypher")]
use super::cypher_graph::CypherGraph;
use super::in_memory_graph::InMemoryGraph;
use super::invalid_graph::InvalidGraph;
use super::{Graph, KBWrapper};
use crate::concepts::attributes::{Attribute, Inherits, Owner, Value};
use crate::concepts::{Archetype, ArchetypeTrait, Tao};
#[cfg(not(debug_assertions))]
use lazy_static::lazy_static;
#[cfg(debug_assertions)]
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(not(debug_assertions))]
use std::sync::{Arc, Mutex};

#[cfg(debug_assertions)]
thread_local! {
    static GRAPH: RefCell<Box<dyn Graph>> = RefCell::new(Box::new(InvalidGraph {}));
}

#[cfg(not(debug_assertions))]
lazy_static! {
    static ref GRAPH: Arc<Mutex<Box<dyn Graph>>> = Arc::new(Mutex::new(Box::new(InvalidGraph {})));
}

/// Add the given Concept type to the KB.
#[macro_export]
macro_rules! initialize_type {
    ($g:expr, ($($t:ty),*)) => {
        $(
            $g.add_node();
            $g.set_node_name(<$t>::TYPE_ID, <$t>::TYPE_NAME.to_string());
        )*
        // set edges later, since edges contain references to node names, and that will be
        // impossible if the nodes themselves don't exist yet
        $($g.add_edge(<$t>::TYPE_ID, Inherits::TYPE_ID, <$t>::PARENT_TYPE_ID);)*
    };
}

/// Bind GRAPH to a new graph that sits entirely in memory.
#[cfg(debug_assertions)]
pub fn bind_in_memory_graph() {
    GRAPH.with(|g| {
        let mut img = InMemoryGraph::new();
        initialize_type!(img, (Tao, Archetype, Attribute, Owner, Value, Inherits));
        *g.borrow_mut() = Box::new(img);
    });
}

#[cfg(not(debug_assertions))]
pub fn bind_in_memory_graph() {
    let mut img = InMemoryGraph::new();
    initialize_type!(img, (Tao, Archetype, Attribute, Owner, Value, Inherits));
    *GRAPH.lock().unwrap() = Box::new(img);
}

/// Bind GRAPH to an external Neo4j database.
///
/// Current limitations:
///
///  * This requries Neo4j version 3, because rusted_cypher currently does not appear to support
///    version 4. See the [deprecation notice](https://neo4j.com/docs/rest-docs/3.5/) on the
///    version 3 API.
///  * Only string values can be attached to nodes.
#[cfg(all(debug_assertions, feature = "cypher"))]
pub fn bind_cypher_graph(uri: &str) {
    GRAPH.with(|g| {
        let mut cg = CypherGraph::new(uri);
        initialize_type!(cg, (Tao, Archetype, Attribute, Owner, Value, Inherits));
        *g.borrow_mut() = Box::new(cg);
    });
}

#[cfg(all(not(debug_assertions), feature = "cypher"))]
pub fn bind_cypher_graph(uri: &str) {
    let mut cg = CypherGraph::new(uri);
    initialize_type!(cg, (Tao, Archetype, Attribute, Owner, Value, Inherits));
    *GRAPH.lock().unwrap() = Box::new(cg);
}

/// Graph usable with dependency injection.
#[derive(Copy, Clone)]
pub struct InjectionGraph {}

impl InjectionGraph {
    /// Creates a new reference to
    pub fn new() -> Self {
        Self {}
    }
}

unsafe impl Send for InjectionGraph {}

#[cfg(debug_assertions)]
impl Graph for InjectionGraph {
    fn size(&self) -> usize {
        GRAPH.with(|g| g.borrow().size())
    }

    fn add_node(&mut self) -> usize {
        GRAPH.with(|g| g.borrow_mut().add_node().clone())
    }

    fn set_node_value(&mut self, id: usize, value: Box<dyn KBWrapper>) {
        GRAPH.with(|g| g.borrow_mut().set_node_value(id, value))
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        GRAPH.with(|g| g.borrow_mut().set_node_name(id, name))
    }

    fn node_name(&self, id: usize) -> Option<Rc<String>> {
        GRAPH.with(|g| g.borrow().node_name(id))
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

    fn into_dot(&self) -> String {
        GRAPH.with(|g| g.borrow().into_dot())
    }
}

#[cfg(not(debug_assertions))]
impl Graph for InjectionGraph {
    fn size(&self) -> usize {
        GRAPH.lock().unwrap().size()
    }

    fn add_node(&mut self) -> usize {
        GRAPH.lock().unwrap().add_node().clone()
    }

    fn set_node_value(&mut self, id: usize, value: Box<dyn KBWrapper>) {
        GRAPH.lock().unwrap().set_node_value(id, value)
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        GRAPH.lock().unwrap().set_node_name(id, name)
    }

    fn node_name(&self, id: usize) -> Option<Rc<String>> {
        GRAPH.lock().unwrap().node_name(id)
    }

    fn node_value(&self, id: usize) -> Option<Rc<Box<dyn KBWrapper>>> {
        GRAPH.lock().unwrap().node_value(id).map(|r| r.clone())
    }

    fn add_edge(&mut self, from: usize, edge_type: usize, to: usize) {
        GRAPH.lock().unwrap().add_edge(from, edge_type, to)
    }

    fn has_edge(&self, from: usize, edge_type: usize, to: usize) -> bool {
        GRAPH.lock().unwrap().has_edge(from, edge_type, to)
    }

    fn outgoing_nodes(&self, from: usize, edge_type: usize) -> Vec<usize> {
        GRAPH.lock().unwrap().outgoing_nodes(from, edge_type)
    }

    fn incoming_nodes(&self, to: usize, edge_type: usize) -> Vec<usize> {
        GRAPH.lock().unwrap().incoming_nodes(to, edge_type)
    }

    fn all_outgoing_nodes(&self, from: usize) -> Vec<usize> {
        GRAPH.lock().unwrap().all_outgoing_nodes(from)
    }

    fn all_incoming_nodes(&self, to: usize) -> Vec<usize> {
        GRAPH.lock().unwrap().all_incoming_nodes(to)
    }

    fn into_dot(&self) -> String {
        GRAPH.lock().unwrap().into_dot()
    }
}

/// Print graph to stdout for debugging purposes.
pub fn print_graph_debug() {
    println!("{}", InjectionGraph::new().into_dot());
}
