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
//! use yin::concepts::{Concept, ConceptTypeTrait};
//!
//! let mut concept = Concept::new();
//! ```
//!
//! We can set a name for this concept. Note that names don't need to be unique.
//!
//! ```rust
//! # use yin::concepts::{Concept, ConceptTypeTrait};
//! # use yin::graph::bind_in_memory_graph;
//! # bind_in_memory_graph();
//! # let mut concept = Concept::new();
//! use yin::wrappers::CommonNodeTrait;
//! use std::rc::Rc;
//!
//! concept.set_internal_name("A".to_string());
//! assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
//! ```

mod owner;
mod value;

pub use owner::Owner;
pub use value::Value;

use crate::wrappers::{debug_wrapper, BaseWrapper, CommonNodeTrait};
use std::cmp::{Eq, PartialEq};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Interface for all concepts -- separate from ConceptTrait so that ConceptTrait can be a trait
/// object.
pub trait ConceptTypeTrait<T>: From<usize> {
    /// ID for the node that represents this type of node.
    const TYPE_ID: usize;

    /// String name for this type node.
    const TYPE_NAME: &'static str;

    /// The type concept that represents all concepts of this type.
    fn type_concept() -> Concept;

    /// Create a new concept of this type.
    fn new() -> T;
}

/// Interface for all concepts.
pub trait ConceptTrait: CommonNodeTrait {
    /// Get down to the core of the abstraction.
    fn base(&self) -> &BaseWrapper;

    /// Upcast to a Concept.
    fn as_concept(&self) -> Concept {
        Concept {
            base: self.base().clone(),
        }
    }
}

/// Implementation for a generic concept.
#[derive(Copy, Clone)]
pub struct Concept {
    /// Wrapper that this abstraction is based on.
    pub base: BaseWrapper,
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

impl From<usize> for Concept {
    fn from(id: usize) -> Self {
        Concept {
            base: BaseWrapper::from(id),
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

    fn internal_name(&self) -> Option<Rc<String>> {
        self.base.internal_name()
    }
}

impl ConceptTypeTrait<Concept> for Concept {
    const TYPE_ID: usize = 0;
    const TYPE_NAME: &'static str = "Tao";

    fn type_concept() -> Concept {
        Concept {
            base: BaseWrapper::from(Self::TYPE_ID),
        }
    }

    fn new() -> Self {
        Concept {
            base: BaseWrapper::new(),
        }
    }
}

impl ConceptTrait for Concept {
    fn base(&self) -> &BaseWrapper {
        &self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;

    #[test]
    fn check_type_created() {
        bind_in_memory_graph();
        assert_eq!(Concept::type_concept().id(), Concept::TYPE_ID);
        assert_eq!(
            Concept::type_concept().internal_name(),
            Some(Rc::new(Concept::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = Concept::new();
        let concept2 = Concept::new();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = Concept::new();
        let concept_copy = Concept::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = Concept::new();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
