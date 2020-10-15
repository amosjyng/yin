#[cfg(feature = "cypher")]
use super::cypher_graph::CypherGraph;
use super::in_memory_graph::InMemoryGraph;
use super::invalid_graph::InvalidGraph;
use super::{Graph, KBValue};
use crate::concepts::attributes::{Attribute, HasAttributeType, Inherits, Owner, Value};
use crate::concepts::{Archetype, ArchetypeTrait, Tao};
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static GRAPH: RefCell<Box<dyn Graph>> = RefCell::new(Box::new(InvalidGraph{}));
}

/// Add the given Concept type to the KB.
///
/// # Examples
///
/// Note: do not actually run this on existing types, since they are automatically added when the
/// KB is initialized.
///
/// ```rust
/// # use zamm_yin::graph::bind_in_memory_graph;
/// # bind_in_memory_graph();
/// use zamm_yin::initialize_type;
/// use zamm_yin::concepts::ArchetypeTrait;
/// use zamm_yin::concepts::attributes::Inherits;
/// use zamm_yin::concepts::{Archetype, Tao}; // import your own types instead
/// use zamm_yin::graph::{Graph, InjectionGraph};
///
/// let mut ig = InjectionGraph::new();
/// initialize_type!(ig, (Archetype, Tao));
/// ```
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
pub fn bind_in_memory_graph() {
    GRAPH.with(|g| {
        *g.borrow_mut() = Box::new(InMemoryGraph::new());

        let mut ig = InjectionGraph::default();
        initialize_type!(
            ig,
            (
                Tao,
                Archetype,
                Attribute,
                Owner,
                Value,
                Inherits,
                HasAttributeType
            )
        );
        let mut attributes = Attribute::archetype();
        attributes.add_attribute_type(Owner::archetype());
        attributes.add_attribute_type(Value::archetype());
    });
}

/// Bind GRAPH to an external Neo4j database.
///
/// Current limitations:
///
///  * This requries Neo4j version 3, because rusted_cypher currently does not appear to support
///    version 4. See the [deprecation notice](https://neo4j.com/docs/rest-docs/3.5/) on the
///    version 3 API.
///  * Only string values can be attached to nodes.
#[cfg(feature = "cypher")]
pub fn bind_cypher_graph(uri: &str) {
    GRAPH.with(|g| {
        *g.borrow_mut() = Box::new(CypherGraph::new(uri));

        let mut ig = InjectionGraph::default();
        initialize_type!(
            ig,
            (
                Tao,
                Archetype,
                Attribute,
                Owner,
                Value,
                Inherits,
                HasAttributeType
            )
        );
        let mut attributes = Attribute::archetype();
        attributes.add_attribute_type(Owner::archetype());
        attributes.add_attribute_type(Value::archetype());
    });
}

/// Graph usable with dependency injection.
#[derive(Copy, Clone, Default)]
pub struct InjectionGraph {}

impl InjectionGraph {
    /// Creates a new reference to
    pub fn new() -> Self {
        Self {}
    }
}

impl Graph for InjectionGraph {
    fn size(&self) -> usize {
        GRAPH.with(|g| g.borrow().size())
    }

    fn add_node(&mut self) -> usize {
        GRAPH.with(|g| g.borrow_mut().add_node())
    }

    fn set_node_value(&mut self, id: usize, value: Rc<dyn KBValue>) {
        GRAPH.with(|g| g.borrow_mut().set_node_value(id, value))
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        GRAPH.with(|g| g.borrow_mut().set_node_name(id, name))
    }

    fn node_name(&self, id: usize) -> Option<Rc<String>> {
        GRAPH.with(|g| g.borrow().node_name(id))
    }

    fn node_value(&self, id: usize) -> Option<Rc<dyn KBValue>> {
        GRAPH.with(|g| g.borrow().node_value(id))
    }

    fn lookup(&self, name: &str) -> Vec<usize> {
        GRAPH.with(|g| g.borrow().lookup(name))
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

/// Print graph to stdout for debugging purposes.
pub fn print_graph_debug() {
    println!("{}", InjectionGraph::new().into_dot());
}
