//! Object-oriented representations of nodes as first-class individuals, as opposed to merely being
//! one of many components of a knowledge-base.
//!
//! Do not mistake the map for the territory. Concepts are the map that tells you how to interact
//! with the territory of the actual data structures that they point to.
//!
//! # Examples
//!
//! We need to choose which graph implementation to ground our knowledge and reasoning on. All
//! implementations should be logically equivalent. Let's use the in-memory one for simplicity:
//!
//! ```rust
//! use yin::graph::bind_in_memory_graph;
//!
//! bind_in_memory_graph();
//! ```
//!
//! Now, we can create a new concept:
//!
//! ```rust
//! # use yin::graph::bind_in_memory_graph;
//! # bind_in_memory_graph();
//! use yin::concept::ConceptImpl;
//!
//! let mut concept = ConceptImpl::create();
//! ```
//!
//! We can set a name for this concept. Note that names don't need to be unique.
//!
//! ```rust
//! # use yin::concept::ConceptImpl;
//! # use yin::graph::bind_in_memory_graph;
//! # bind_in_memory_graph();
//! # let mut concept = ConceptImpl::create();
//! use yin::concept::Concept;
//!
//! concept.set_internal_name("A".to_string());
//! assert_eq!(concept.internal_name(), Some("A".to_string()));
//! ```
//!
//! We can also set a value for the concept. We use `Rc` here because Yin being the map and not the
//! territory, we generally don't want to have Yin itself own the data being operated on.
//!
//! ```rust
//! # use yin::concept::{Concept, ConceptImpl};
//! # use yin::graph::bind_in_memory_graph;
//! # bind_in_memory_graph();
//! # let mut concept = ConceptImpl::create();
//! use yin::graph::{WeakWrapper, unwrap_weak};
//! use std::rc::Rc;
//!
//! let v = Rc::new(5);
//! concept.set_value(Box::new(WeakWrapper::new(&v)));
//! assert_eq!(unwrap_weak::<i32>(concept.value()), Some(v));
//! ```

use crate::graph::{Graph, InjectionGraph, KBWrapper};
use std::cmp::{Eq, PartialEq};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Interface for all concepts.
pub trait Concept {
    /// The unique integer that's associated with this concept.
    fn id(&self) -> usize;

    /// Associate this concept with an internal name. The name does not need to be unique.
    fn set_internal_name(&mut self, name: String);

    /// The internal name that's associated with this concept, if one exists.
    fn internal_name(&self) -> Option<String>;

    /// Associate this concept with a value.
    fn set_value(&mut self, value: Box<dyn KBWrapper>);

    /// Retrieve the value associated with this concept.
    fn value(&self) -> Option<Rc<Box<dyn KBWrapper>>>;
}

/// Implementation for a generic concept.
#[derive(Copy, Clone)]
pub struct ConceptImpl {
    graph: InjectionGraph,
    id: usize,
}

impl ConceptImpl {
    /// Link this concept to another one via an outgoing edge.
    pub fn add_outgoing(&mut self, edge_type: ConceptImpl, to: ConceptImpl) {
        self.graph.add_edge(self.id(), edge_type.id(), to.id())
    }

    /// All concepts that this one links to via outgoing edges of a certain type.
    pub fn outgoing_nodes(&self, edge_type: ConceptImpl) -> Vec<ConceptImpl> {
        self.graph
            .outgoing_nodes(self.id(), edge_type.id())
            .into_iter()
            .map(|id| ConceptImpl::from(id))
            .collect()
    }
}

impl Debug for ConceptImpl {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.internal_name() {
            Some(name) => f.write_fmt(format_args!("Concept({},{})", self.id, name)),
            None => f.write_fmt(format_args!("Concept({})", self.id)),
        }
    }
}

impl Eq for ConceptImpl {}

impl PartialEq for ConceptImpl {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> ConceptImpl {
    /// Create a concept wrapper from an existing node's ID.
    pub fn from(id: usize) -> Self {
        ConceptImpl {
            graph: InjectionGraph {},
            id: id,
        }
    }

    /// Create a new concept.
    pub fn create() -> Self {
        let mut g = InjectionGraph {};
        let id = g.add_node();
        ConceptImpl { graph: g, id: id }
    }
}

impl<'a> Concept for ConceptImpl {
    fn id(&self) -> usize {
        self.id
    }

    fn set_internal_name(&mut self, name: String) {
        self.graph.set_node_name(self.id, name);
    }

    fn internal_name(&self) -> Option<String> {
        self.graph.node_name(self.id).map(|n| n.clone())
    }

    fn set_value(&mut self, value: Box<dyn KBWrapper>) {
        self.graph.set_node_value(self.id, value)
    }

    fn value(&self) -> Option<Rc<Box<dyn KBWrapper>>> {
        self.graph.node_value(self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{bind_in_memory_graph, unwrap_weak, WeakWrapper};

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = ConceptImpl::create();
        let concept2 = ConceptImpl::create();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = ConceptImpl::create();
        let concept_copy = ConceptImpl::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = ConceptImpl::create();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some("A".to_string()));
    }

    #[test]
    fn retrieve_node_value() {
        bind_in_memory_graph();
        let mut concept = ConceptImpl::create();
        let v = Rc::new(5);
        concept.set_value(Box::new(WeakWrapper::new(&v)));
        assert_eq!(unwrap_weak::<i32>(concept.value()), Some(v));
    }

    #[test]
    fn no_outgoing_nodes() {
        bind_in_memory_graph();
        let a = ConceptImpl::create();
        assert_eq!(a.outgoing_nodes(a), Vec::new());
    }

    #[test]
    fn outgoing_nodes() {
        bind_in_memory_graph();
        let mut a = ConceptImpl::create();
        let b = ConceptImpl::create();
        let c = ConceptImpl::create();
        let d = ConceptImpl::create();
        let mut e = ConceptImpl::create();
        let edge_type1 = ConceptImpl::create();
        let edge_type2 = ConceptImpl::create();
        a.add_outgoing(edge_type1, b);
        a.add_outgoing(edge_type2, c);
        a.add_outgoing(edge_type1, d);
        e.add_outgoing(edge_type1, a);
        assert_eq!(a.outgoing_nodes(edge_type1), vec![b, d]);
    }
}
