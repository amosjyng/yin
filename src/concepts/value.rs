use super::owner::{AttributeTrait, Owner};
use crate::concepts::{Concept, ConceptTrait, ConceptTypeTrait};
use crate::wrappers::{debug_wrapper, BaseNodeTrait, BaseWrapper, CommonNodeTrait};
use std::fmt::{Debug, Formatter, Result};

/// The value/target/to-node of an attribute.
#[derive(Copy, Clone)]
pub struct Value {
    /// Wrapper that this abstraction is based on.
    pub base: BaseWrapper,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Value", Box::new(self), f)
    }
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl From<usize> for Value {
    fn from(id: usize) -> Self {
        Value {
            base: BaseWrapper::from(id),
        }
    }
}

impl CommonNodeTrait for Value {
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

impl ConceptTypeTrait<Value> for Value {
    const TYPE_ID: usize = 2;
    const TYPE_NAME: &'static str = "Value";

    fn type_concept() -> Concept {
        Concept {
            base: BaseWrapper::from(Self::TYPE_ID),
        }
    }

    fn new() -> Self {
        Value {
            base: BaseWrapper::new(),
        }
    }
}

impl ConceptTrait for Value {
    fn base(&self) -> &BaseWrapper {
        &self.base
    }
}

impl AttributeTrait<Value> for Value {
    fn set_owner(&mut self, owner: Box<&dyn ConceptTrait>) {
        self.base.add_outgoing(Owner::TYPE_ID, owner.base());
    }

    fn owner(&self) -> Option<Concept> {
        self.base
            .outgoing_nodes(Owner::TYPE_ID)
            .get(0)
            .map(|n| Concept { base: *n })
    }

    fn set_value(&mut self, value: Box<&dyn ConceptTrait>) {
        self.base.add_outgoing(Value::TYPE_ID, value.base());
    }

    fn value(&self) -> Option<Concept> {
        self.base
            .outgoing_nodes(Value::TYPE_ID)
            .get(0)
            .map(|n| Concept { base: *n })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;

    #[test]
    fn check_type_created() {
        bind_in_memory_graph();
        assert_eq!(Value::type_concept().id(), Value::TYPE_ID);
        assert_eq!(
            Value::type_concept().internal_name(),
            Some(Value::TYPE_NAME.to_string())
        );
    }

    #[test]
    fn get_owner() {
        bind_in_memory_graph();
        let mut value_instance = Value::new();
        let owner_of_value = Value::new();
        value_instance.set_owner(Box::new(&owner_of_value));
        assert_eq!(value_instance.owner(), Some(owner_of_value.as_concept()));
        assert_eq!(value_instance.value(), None);
    }

    #[test]
    fn get_value() {
        bind_in_memory_graph();
        let mut value_instance = Value::new();
        let value_of_value = Value::new();
        value_instance.set_value(Box::new(&value_of_value));
        assert_eq!(value_instance.owner(), None);
        assert_eq!(value_instance.value(), Some(value_of_value.as_concept()));
    }
}
