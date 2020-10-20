//! Defines the interface and implementations for working with regular directed graphs with
//! labeled edges.
//!
//! # Examples
//! We need to choose which graph implementation to ground our knowledge and reasoning on. All
//! implementations should be logically equivalent. Let's use the in-memory one for simplicity:
//!
//! ```rust
//! use zamm_yin::graph::bind_in_memory_graph;
//!
//! bind_in_memory_graph();
//! ```
//!
//! No matter which implementation you choose, `InjectionGraph` allows you to use that
//! implementation via dependency injection:
//!
//! ```rust
//! # use zamm_yin::graph::bind_in_memory_graph;
//! # bind_in_memory_graph();
//! use zamm_yin::graph::InjectionGraph;
//!
//! let mut g = InjectionGraph::new();
//! ```
//!
//! Now we can create a new node:
//! ```rust
//! # use zamm_yin::graph::{bind_in_memory_graph, InjectionGraph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph::new();
//! use zamm_yin::graph::Graph;
//!
//! let a_id = g.add_node();
//! ```
//!
//! We can set a name for the node. Note that names don't need to be unique.
//!
//! ```rust
//! # use zamm_yin::graph::{bind_in_memory_graph, InjectionGraph, Graph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph::new();
//! # let a_id = g.add_node();
//! use std::rc::Rc;
//!
//! g.set_node_name(a_id, "A".to_string());
//! assert_eq!(g.node_name(a_id), Some(Rc::new("A".to_string())));
//! ```
//!
//! We can also set a value for the node. We use `Rc` here because Yin being the map and not the
//! territory, we generally don't want to have Yin itself own the data being operated on.
//!
//! ```rust
//! # use zamm_yin::graph::{bind_in_memory_graph, InjectionGraph, Graph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph::new();
//! # let a_id = g.add_node();
//! use zamm_yin::graph::value_wrappers::{unwrap_weak, WeakValue};
//! use std::rc::Rc;
//!
//! let v = Rc::new(5);
//! g.set_node_value(a_id, Rc::new(WeakValue::new(&v)));
//! assert_eq!(unwrap_weak::<i32>(g.node_value(a_id)), Some(v));
//! ```
//!
//! Let's create a few more nodes:
//! ```rust
//! # use zamm_yin::graph::{bind_in_memory_graph, InjectionGraph, Graph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph::new();
//! # let a_id = g.add_node();
//! let b_id = g.add_node();
//! let c_id = g.add_node();
//! let d_id = g.add_node();
//! let edge_type1 = g.add_node();
//! let edge_type2 = g.add_node();
//! ```
//!
//! Let's now connect some of these nodes together. Note that while edge labels can technically be
//! any integer, they are assumed to be node IDs by higher-level Yin abstractions:
//!
//! ```rust
//! # use zamm_yin::graph::{bind_in_memory_graph, InjectionGraph, Graph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph::new();
//! # let a_id = g.add_node();
//! # let b_id = g.add_node();
//! # let c_id = g.add_node();
//! # let d_id = g.add_node();
//! # let edge_type1 = g.add_node();
//! # let edge_type2 = g.add_node();
//! g.add_edge(b_id, edge_type1, a_id);
//! g.add_edge(c_id, edge_type2, a_id);
//! g.add_edge(d_id, edge_type1, a_id);
//!
//! assert_eq!(g.all_incoming_nodes(a_id), vec![b_id, c_id, d_id]);
//! assert_eq!(g.incoming_nodes(a_id, edge_type1), vec![b_id, d_id]);
//! assert_eq!(g.all_outgoing_nodes(b_id), vec![a_id]);
//! assert_eq!(g.outgoing_nodes(c_id, edge_type2), vec![a_id]);
//! ```
//!
//! We can also use the KB to invoke certain functionality. Note that we are passing in a `Form`
//! concept to the callback function because that's the only supported function at this moment.
//!
//! ```rust
//! # use zamm_yin::graph::{bind_in_memory_graph, InjectionGraph, Graph};
//! # use zamm_yin::graph::value_wrappers::{unwrap_weak, WeakValue};
//! # use std::rc::Rc;
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph::new();
//! use zamm_yin::tao::archetype::ArchetypeTrait;
//! use zamm_yin::tao::{Form, FormTrait};
//! use zamm_yin::graph::value_wrappers::{StrongValue, KBClosure, unwrap_closure};
//! use zamm_yin::{define_closure, run_closure};
//! use zamm_yin::node_wrappers::BaseNodeTrait;
//! use std::any::Any;
//! use std::cell::{RefCell, RefMut};
//!
//! let count_id = g.add_node();
//! let count_value: Rc<i64> = Rc::new(5);
//! g.set_node_value(count_id, Rc::new(WeakValue::new(&count_value)));
//!
//! let mut triple_id = g.add_node();
//! g.set_node_value(triple_id, define_closure!(|t: Form| {
//!     Box::new(*unwrap_weak::<i64>(t.essence().value()).unwrap() * 3)
//! }));
//! assert_eq!(
//!     run_closure!(&g.node_value(triple_id), Form::from(count_id), i64),
//!     Some(Box::new(15))
//! );
//! ```
//!
//! In general, it's recommended to only use the KB to decide what to do at a very high level, and
//! not to actually do things via the KB. For example, perhaps we could use the KB to decide to run
//! Dijkstra's. We could even use the KB to design and debug an implementation of Dijkstra's. But
//! the actual computation of Dijkstra's algorithm should involve low-level data structures and
//! logic outside of the KB.

#[cfg(feature = "cypher")]
mod cypher_graph;
mod in_memory_graph;
mod injection_graph;
mod invalid_graph;
/// Wrappers around values associated with nodes in the KB. This differs from the other
/// [`wrappers`](../wrappers/index.html) package because this abstraction only wraps the
/// values associated with nodes, while the other one wraps the nodes themselves.
///
/// Due to Rust restrictions around upcasting to Any, it appears necessary to create an extra
/// wrapper.
pub mod value_wrappers;

use crate::graph::value_wrappers::KBValue;
#[cfg(feature = "cypher")]
pub use injection_graph::bind_cypher_graph;
pub use injection_graph::{bind_in_memory_graph, print_graph_debug, InjectionGraph};

use std::rc::Rc;

/// A classic directed Graph with nodes and labeled links.
pub trait Graph {
    /// The number of nodes in the graph.
    fn size(&self) -> usize;

    /// Adds a new node to the graph, and returns the node's ID.
    fn add_node(&mut self) -> usize;

    /// Sets the name for a given node. Names can only be set once.
    fn set_node_name(&mut self, id: usize, name: String);

    /// Sets the value for a given node. Values can only be set once.
    fn set_node_value(&mut self, id: usize, value: Rc<dyn KBValue>);

    /// Retrieve's a node's name from the graph, or None if the node does not exist or is unnamed.
    fn node_name(&self, id: usize) -> Option<Rc<String>>;

    /// Retrieve's a node's name from the graph, or None if the node does not exist or does not
    /// have a value.
    fn node_value(&self, id: usize) -> Option<Rc<dyn KBValue>>;

    /// Look up a node ID based on name. A vec is returned because there are no constraints on name
    /// uniqueness.
    fn lookup(&self, name: &str) -> Vec<usize>;

    /// Add a flag to a node. The flag should be the ID of an existing node.
    fn add_flag(&mut self, id: usize, flag: usize);

    /// Return true if this node has the flag set, false otherwise.
    fn flag(&self, id: usize, flag: usize) -> bool;

    /// Add a labeled edge between two nodes. The label should be the ID of an existing node.
    fn add_edge(&mut self, from: usize, edge_type: usize, to: usize);

    /// Checks for a labeled edge between two nodes. The label should be the ID of an existing node.
    fn has_edge(&self, from: usize, edge_type: usize, to: usize) -> bool;

    /// Retrieve all node IDs that are on the other end of an outgoing edge of the given type.
    fn outgoing_nodes(&self, from: usize, edge_type: usize) -> Vec<usize>;

    /// Retrieve all node IDs that are on the other end of an incoming edge of the given type.
    fn incoming_nodes(&self, to: usize, edge_type: usize) -> Vec<usize>;

    /// Retrieve all node IDs that are on the other end of outgoing edges.
    fn all_outgoing_nodes(&self, from: usize) -> Vec<usize>;

    /// Retrieve all node IDs that are on the other end of incoming edges.
    fn all_incoming_nodes(&self, to: usize) -> Vec<usize>;

    /// Outputs the entire graph in DOT format.
    fn into_dot(&self) -> String;
}
