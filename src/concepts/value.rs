use super::owner::{AttributeTrait, Owner};
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::wrappers::{debug_wrapper, BaseNodeTrait, BaseWrapper, CommonNodeTrait};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

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

    fn internal_name(&self) -> Option<Rc<String>> {
        self.base.internal_name()
    }
}

impl ArchetypeTrait<Value> for Value {
    const TYPE_ID: usize = 2;
    const TYPE_NAME: &'static str = "Value";

    fn type_concept() -> Tao {
        Tao::from(Self::TYPE_ID)
    }

    fn individuate() -> Self {
        Value {
            base: BaseWrapper::new(),
        }
    }
}

impl FormTrait for Value {
    fn essence(&self) -> &BaseWrapper {
        &self.base
    }
}

impl AttributeTrait<Value> for Value {
    fn set_owner(&mut self, owner: Box<&dyn FormTrait>) {
        self.base.add_outgoing(Owner::TYPE_ID, owner.essence());
    }

    fn owner(&self) -> Option<Tao> {
        self.base
            .outgoing_nodes(Owner::TYPE_ID)
            .get(0)
            .map(|n| Tao::from(*n))
    }

    fn set_value(&mut self, value: Box<&dyn FormTrait>) {
        self.base.add_outgoing(Value::TYPE_ID, value.essence());
    }

    fn value(&self) -> Option<Tao> {
        self.base
            .outgoing_nodes(Value::TYPE_ID)
            .get(0)
            .map(|n| Tao::from(*n))
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
            Some(Rc::new(Value::TYPE_NAME.to_string()))
        );
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
