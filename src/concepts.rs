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
//! use yin::concepts::Concept;
//!
//! let mut concept = Concept::create();
//! ```
//!
//! We can set a name for this concept. Note that names don't need to be unique.
//!
//! ```rust
//! # use yin::concepts::Concept;
//! # use yin::graph::bind_in_memory_graph;
//! # bind_in_memory_graph();
//! # let mut concept = Concept::create();
//! use yin::wrappers::CommonNodeTrait;
//!
//! concept.set_internal_name("A".to_string());
//! assert_eq!(concept.internal_name(), Some("A".to_string()));
//! ```

use crate::wrappers::{debug_wrapper, BaseWrapper, CommonNodeTrait};
use std::cmp::{Eq, PartialEq};
use std::fmt::{Debug, Formatter, Result};

/// Interface for all concepts.
pub trait ConceptTrait: CommonNodeTrait {}

/// Implementation for a generic concept.
#[derive(Copy, Clone)]
pub struct Concept {
    base: BaseWrapper,
}

impl Debug for Concept {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Concept", Box::new(self), f)
    }
}

impl Eq for Concept {}

impl PartialEq for Concept {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<'a> Concept {
    /// Create a concept wrapper from an existing node's ID.
    pub fn from(id: usize) -> Self {
        Concept {
            base: BaseWrapper::from(id),
        }
    }

    /// Create a new concept.
    pub fn create() -> Self {
        Concept {
            base: BaseWrapper::create(),
        }
    }
}

impl CommonNodeTrait for Concept {
    fn id(&self) -> usize {
        self.base.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.base.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<String> {
        self.base.internal_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = Concept::create();
        let concept2 = Concept::create();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = Concept::create();
        let concept_copy = Concept::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = Concept::create();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some("A".to_string()));
    }
}
