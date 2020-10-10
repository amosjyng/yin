use super::{Attribute, AttributeTrait};
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// The value/target/to-node of an attribute.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    attr: Attribute,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Value", Box::new(self), f)
    }
}

impl From<usize> for Value {
    fn from(id: usize) -> Self {
        Self {
            attr: Attribute::from(id),
        }
    }
}

impl CommonNodeTrait for Value {
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

impl ArchetypeTrait<Value> for Value {
    const TYPE_ID: usize = 4;
    const TYPE_NAME: &'static str = "Value";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            attr: Attribute::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for Value {
    fn essence(&self) -> &FinalNode {
        self.attr.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        self.attr.essence_mut()
    }
}

impl AttributeTrait<Value> for Value {
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
        assert_eq!(Value::archetype().id(), Value::TYPE_ID);
        assert_eq!(
            Value::archetype().internal_name(),
            Some(Rc::new(Value::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = Value::individuate();
        let concept_copy = Value::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = Value::individuate();
        let concept2 = Value::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = Value::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn get_owner() {
        bind_in_memory_graph();
        let mut value_instance = Value::individuate();
        let owner_of_value = Value::individuate();
        value_instance.set_owner(Box::new(&owner_of_value));
        assert_eq!(value_instance.owner(), Some(owner_of_value.ego_death()));
        assert_eq!(value_instance.value(), None);
    }

    #[test]
    fn get_value() {
        bind_in_memory_graph();
        let mut value_instance = Value::individuate();
        let value_of_value = Value::individuate();
        value_instance.set_value(Box::new(&value_of_value));
        assert_eq!(value_instance.owner(), None);
        assert_eq!(value_instance.value(), Some(value_of_value.ego_death()));
    }
}
