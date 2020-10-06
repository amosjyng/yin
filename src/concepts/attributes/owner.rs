use super::{Attribute, AttributeTrait};
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::wrappers::{debug_wrapper, CommonNodeTrait, FinalWrapper};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// The owner/source/from-node of an attribute.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Owner {
    attr: Attribute,
}

impl Debug for Owner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Owner", Box::new(self), f)
    }
}

impl From<usize> for Owner {
    fn from(id: usize) -> Self {
        Self {
            attr: Attribute::from(id),
        }
    }
}

impl CommonNodeTrait for Owner {
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

impl ArchetypeTrait<Owner> for Owner {
    const TYPE_ID: usize = 3;
    const TYPE_NAME: &'static str = "Owner";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            attr: Attribute::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for Owner {
    fn essence(&self) -> &FinalWrapper {
        self.attr.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalWrapper {
        self.attr.essence_mut()
    }
}

impl AttributeTrait<Owner> for Owner {
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
        assert_eq!(Owner::archetype().id(), Owner::TYPE_ID);
        assert_eq!(
            Owner::archetype().internal_name(),
            Some(Rc::new(Owner::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = Owner::individuate();
        let concept_copy = Owner::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = Owner::individuate();
        let concept2 = Owner::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = Owner::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn get_owner() {
        bind_in_memory_graph();
        let mut owner_instance = Owner::individuate();
        let owner_of_owner = Owner::individuate();
        owner_instance.set_owner(Box::new(&owner_of_owner));
        assert_eq!(owner_instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(owner_instance.value(), None);
    }

    #[test]
    fn get_value() {
        bind_in_memory_graph();
        let mut owner_instance = Owner::individuate();
        let value_of_owner = Owner::individuate();
        owner_instance.set_value(Box::new(&value_of_owner));
        assert_eq!(owner_instance.owner(), None);
        assert_eq!(owner_instance.value(), Some(value_of_owner.ego_death()));
    }
}
