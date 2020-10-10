use super::{Attribute, AttributeTrait};
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Describes the owner as inheriting all attributes of the value.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inherits {
    attr: Attribute,
}

impl Debug for Inherits {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Inherits", Box::new(self), f)
    }
}

impl From<usize> for Inherits {
    fn from(id: usize) -> Self {
        Self {
            attr: Attribute::from(id),
        }
    }
}

impl CommonNodeTrait for Inherits {
    fn id(&self) -> usize {
        self.attr.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.attr.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.attr.internal_name()
    }
}

impl ArchetypeTrait<Inherits> for Inherits {
    const TYPE_ID: usize = 5;
    const TYPE_NAME: &'static str = "Inherits";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            attr: Attribute::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for Inherits {
    fn essence(&self) -> &FinalNode {
        self.attr.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        self.attr.essence_mut()
    }
}

impl AttributeTrait<Inherits> for Inherits {
    fn set_owner(&mut self, owner: Box<&dyn FormTrait>) {
        self.attr.set_owner(owner);
    }

    fn owner(&self) -> Option<Tao> {
        self.attr.owner()
    }

    fn set_value(&mut self, value: Box<&dyn FormTrait>) {
        self.attr.set_value(value);
    }

    fn value(&self) -> Option<Tao> {
        self.attr.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;

    #[test]
    fn check_type_created() {
        bind_in_memory_graph();
        assert_eq!(Inherits::archetype().id(), Inherits::TYPE_ID);
        assert_eq!(
            Inherits::archetype().internal_name(),
            Some(Rc::new(Inherits::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = Inherits::individuate();
        let concept_copy = Inherits::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = Inherits::individuate();
        let concept2 = Inherits::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = Inherits::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
