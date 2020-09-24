//! Object-oriented representations of nodes as first-class individuals, as opposed to merely being
//! one of many components of a knowledge-base.
//! 
//! Do not mistake the map for the territory. Concepts are the map that tells you how to interact
//! with the territory of the actual data structures that they point to.

use crate::graph::{Graph, thread_local_graph};

/// Interface for all concepts.
pub trait Concept {
    /// The unique integer that's associated with this concept.
    fn id(&self) -> usize;

    /// The internal name that's associated with this concept, if one exists.
    fn internal_name(&self) -> Option<String>;
}

/// Implementation for a generic concept.
pub struct ConceptImpl {
    id: usize,
}

impl<'a> ConceptImpl {
    /// Create a concept wrapper from an existing node's ID.
    pub fn from(id: usize) -> Self {
        ConceptImpl {
            id: id,
        }
    }

    /// Create a new concept.
    pub fn create() -> Self {
        let id = thread_local_graph().with(|g_cell| g_cell.borrow_mut().add_node());
        ConceptImpl {
            id: id,
        }
    }

    /// Create a new concept with the given name.
    pub fn create_with_name(name: String) -> Self {
        thread_local_graph().with(|gcell| {
            let mut g = gcell.borrow_mut();
            let id = g.add_node();
            g.set_node_name(id, name);
            ConceptImpl {
                id: id,
            }
        })
    }
}

impl<'a> Concept for ConceptImpl {
    fn id(&self) -> usize {
        self.id
    }

    fn internal_name(&self) -> Option<String> {
        thread_local_graph().with(|g_cell| {
            let x = g_cell.borrow();
            let name = x.node_name(self.id);
            name.map(|n| n.clone())
        })
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
