//! Object-oriented representations of nodes as first-class individuals, as opposed to merely being
//! one of many components of a knowledge-base.
//!
//! Do not mistake the map for the territory. Concepts are the map that tells you how to interact
//! with the territory of the actual data structures that they point to.

use crate::graph::{Graph, InjectionGraph};

/// Interface for all concepts.
pub trait Concept {
    /// The unique integer that's associated with this concept.
    fn id(&self) -> usize;

    /// The internal name that's associated with this concept, if one exists.
    fn internal_name(&self) -> Option<String>;
}

/// Implementation for a generic concept.
pub struct ConceptImpl {
    graph: InjectionGraph,
    id: usize,
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

    /// Create a new concept with the given name.
    pub fn create_with_name(name: String) -> Self {
        let mut g = InjectionGraph {};
        let id = g.add_node();
        g.set_node_name(id, name);
        ConceptImpl { graph: g, id: id }
    }
}

impl<'a> Concept for ConceptImpl {
    fn id(&self) -> usize {
        self.id
    }

    fn internal_name(&self) -> Option<String> {
        self.graph.node_name(self.id).map(|n| n.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;

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
        let concept = ConceptImpl::create_with_name("A".to_string());
        assert_eq!(concept.internal_name(), Some("A".to_string()));
    }
}
