//! Defines the interface and implementations for working with regular directed graphs with
//! labeled edges.
//!
//! # Examples
//! We need to choose which graph implementation to ground our knowledge and reasoning on. All
//! implementations should be logically equivalent. Let's use the in-memory one for simplicity:
//!
//! ```rust
//! use yin::graph::bind_in_memory_graph;
//!
//! bind_in_memory_graph();
//! ```
//!
//! No matter which implementation you choose, `InjectionGraph` allows you to use that
//! implementation via dependency injection:
//!
//! ```rust
//! # use yin::graph::bind_in_memory_graph;
//! # bind_in_memory_graph();
//! use yin::graph::InjectionGraph;
//!
//! let mut g = InjectionGraph {};
//! ```
//!
//! Now we can create a new node:
//! ```rust
//! # use yin::graph::{bind_in_memory_graph, InjectionGraph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph {};
//! use yin::graph::Graph;
//!
//! let a_id = g.add_node();
//! ```
//!
//! We can set a name for the node. Note that names don't need to be unique.
//!
//! ```rust
//! # use yin::graph::{bind_in_memory_graph, InjectionGraph, Graph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph {};
//! # let a_id = g.add_node();
//! use std::rc::Rc;
//!
//! g.set_node_name(a_id, "A".to_string());
//! assert_eq!(g.node_name(a_id), Some(Rc::new("A".to_string())));
//! ```
//!
//! We can also set a value for the node:
//!
//! ```rust
//! # use yin::graph::{bind_in_memory_graph, InjectionGraph, Graph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph {};
//! # let a_id = g.add_node();
//! use yin::graph::{WeakWrapper, unwrap_weak};
//! use std::rc::Rc;
//!
//! let v = Rc::new(5);
//! g.set_node_value(a_id, Box::new(WeakWrapper::new(&v)));
//! assert_eq!(unwrap_weak::<i32>(g.node_value(a_id)), Some(v));
//! ```
//!
//! Let's create a few more nodes:
//! ```rust
//! # use yin::graph::{bind_in_memory_graph, InjectionGraph, Graph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph {};
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
//! # use yin::graph::{bind_in_memory_graph, InjectionGraph, Graph};
//! # bind_in_memory_graph();
//! # let mut g = InjectionGraph {};
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

mod in_memory_graph;
mod injection_graph;
mod invalid_graph;
mod kb_wrapper;

pub use injection_graph::{bind_in_memory_graph, InjectionGraph};
pub use kb_wrapper::{unwrap_weak, KBWrapper, WeakWrapper};

use std::rc::Rc;

/// A classic directed Graph with nodes and labeled links.
pub trait Graph {
    /// Adds a new node to the graph, and returns the node's ID.
    fn add_node(&mut self) -> usize;

    /// Sets the value for a given node. Values can only be set once.
    fn set_node_value(&mut self, id: usize, value: Box<dyn KBWrapper>);

    /// Sets the name for a given node. Names can only be set once.
    fn set_node_name(&mut self, id: usize, name: String);

    /// Retrieve's a node's name from the graph, or None if the node does not exist or is unnamed.
    fn node_name(&self, id: usize) -> Option<Rc<String>>;

    /// Retrieve's a node's name from the graph, or None if the node does not exist or does not
    /// have a value.
    fn node_value(&self, id: usize) -> Option<Rc<Box<dyn KBWrapper>>>;

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
