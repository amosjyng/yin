//! Object-oriented representations of nodes as first-class individuals, as opposed to merely being
//! one of many components of a knowledge-base.
//! 
//! Do not mistake the map for the territory. Concepts are the map that tells you how to interact
//! with the territory of the actual data structures that they point to.

use std::rc::Rc;
use std::cell::RefCell;
use crate::graph::Graph;

/// A generic concept.
pub trait Concept {
    /// The unique integer that's associated with this concept.
    fn id(&self) -> usize;
}

struct ConceptImpl<'a> {
    graph: Rc<RefCell<dyn Graph<'a>>>,
    id: usize,
}

impl<'a> ConceptImpl<'a> {
    /// Create a concept wrapper from an existing node's ID.
    pub fn from(graph: Rc<RefCell<dyn Graph<'a>>>, id: usize) -> Self {
        ConceptImpl {
            graph: graph,
            id: id,
        }
    }

    /// Create a new concept.
    pub fn create(graph: Rc<RefCell<dyn Graph<'a>>>) -> Self {
        let id = graph.borrow_mut().add_node();
        ConceptImpl {
            graph: graph,
            id: id,
        }
    }
}

impl<'a> Concept for ConceptImpl<'a> {
    fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::new_in_memory_graph;

    fn setup_graph<'a>() -> impl Graph<'a> {
        new_in_memory_graph()
    }

    #[test]
    fn create_and_retrieve_node_id() {
        let graph = Rc::new(RefCell::new(setup_graph()));
        let concept1 = ConceptImpl::create(graph.clone());
        let concept2 = ConceptImpl::create(graph.clone());
        assert_eq!(concept1.id() + 1, concept2.id());
    }
}
